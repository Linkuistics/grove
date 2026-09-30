// harness-dispatch-policy — the private worker that evaluates an owner's policy.
//
// The Rust front process starts this executable in a private empty directory,
// with null stdin, a fresh environment and descriptor 3 as its protocol channel
// (see `channel.ts`). The conversation is fixed:
//
//   1. Before touching any policy code, the worker registers the embedded
//      package specifiers and announces its protocol and build identity.
//   2. The front verifies that identity and only then sends the entry to
//      evaluate, with the caller's request. A mismatched worker is never given
//      a policy.
//   3. The worker imports the entry and returns a serializable snapshot of its
//      `policy` export, or a failure naming the stage.
//   4. The front judges the snapshot. For a `routes` policy, or one it refuses,
//      it closes the channel and the worker exits. For a valid `select` policy
//      it asks the worker to select, and the worker calls `select` with the
//      request and returns what it produced.
//
// The worker judges nothing it can hand over as data. The front validates the
// snapshot's shape, resolves routes, checks any explicit choice, validates a
// selection against the snapshot and reports every refusal with its location,
// so there is one validator and it is the one inspection reports. `select` runs
// only once that validator has accepted the policy it belongs to, and the
// snapshot it is checked against was taken before it ran.

import { receive, send } from "./channel.ts";
import * as sdk from "../sdk/index.ts";
import * as dynamicExample from "../examples/dynamic.ts";
import * as groveStaticExample from "../examples/grove-static.ts";
import * as staticExample from "../examples/static.ts";

const PROTOCOL = 1;

// Build identity, replaced by `bun build --define` in `scripts/worker.sh`. The
// `typeof` guard keeps an undefined name from throwing when the source is run
// without that build step; such a worker then reports an identity no front
// accepts.
declare const HARNESS_DISPATCH_BUILD_ID: string;
declare const HARNESS_DISPATCH_PACKAGE_VERSION: string;
const buildId = typeof HARNESS_DISPATCH_BUILD_ID === "string" ? HARNESS_DISPATCH_BUILD_ID : "unbuilt";
const packageVersion =
  typeof HARNESS_DISPATCH_PACKAGE_VERSION === "string" ? HARNESS_DISPATCH_PACKAGE_VERSION : "unbuilt";

// Every documented specifier resolves to the worker's own embedded copy, even
// when a `node_modules/harness-dispatch` package sits beside the importing
// entry. The prefix reserves nothing by itself, so each specifier is
// registered by name; this list is part of the versioned protocol.
//
// The examples import `harness-dispatch/sdk`, and the dynamic example imports
// `harness-dispatch/examples/static`, as an owner's policy does. The bundler
// resolves those through `paths` in `worker/tsconfig.json` when the worker is
// compiled, to the same modules imported above, so an example and a policy
// that imports it share one SDK and one static example. (Bun reads tsconfig at build time;
// `--no-compile-autoload-tsconfig` governs only the compiled worker at run
// time: https://github.com/oven-sh/bun/blob/bun-v1.4.2/docs/bundler/executables.mdx)
const embedded: Readonly<Record<string, object>> = {
  "harness-dispatch/sdk": sdk,
  "harness-dispatch/examples/static": staticExample,
  "harness-dispatch/examples/grove-static": groveStaticExample,
  "harness-dispatch/examples/dynamic": dynamicExample,
};
Bun.plugin({
  name: "harness-dispatch embedded modules",
  setup(build) {
    for (const [specifier, module] of Object.entries(embedded)) {
      build.module(specifier, () => ({ exports: { ...module }, loader: "object" }));
    }
  },
});

send({ type: "hello", protocol: PROTOCOL, packageVersion, buildId, bunVersion: Bun.version });

// What the worker is awaiting on the policy's behalf. A promise nothing is
// left to settle ends the event loop, and `beforeExit` is the one place that
// sees it happen: the front hears which await was abandoned, rather than a
// worker that ended without a word. The event is emitted when the loop
// empties, and never for `process.exit`, which ends every other path:
// https://nodejs.org/api/process.html#event-beforeexit
let awaiting: "load" | "select" | undefined;
process.once("beforeExit", () => {
  if (awaiting === undefined) return;
  send({ type: "failure", stage: awaiting, unsettled: true });
  process.exit(0);
});

// Never a top-level await. Bun 1.4.2 busy-spins on a main module whose
// top-level await nothing is left to settle, and never emits `beforeExit`;
// the same await inside a function called without one lets the loop drain.
// Probed with the pinned Bun, run and compiled, as recorded under
// "Implementation observations" in
// docs/design/harness-selection-and-execution/runtime-evidence.md.
void converse().then(() => process.exit(0));

async function converse(): Promise<void> {
  const request = receive();
  if (request === null) return;
  if (!isEvaluate(request)) {
    throw new Error(`unexpected protocol request: ${JSON.stringify(request)}`);
  }
  awaiting = "load";
  const loaded = await load(request.entry);
  awaiting = undefined;
  send(loaded.frame);
  if (loaded.policy === undefined) return;

  const next = receive();
  if (next === null) return;
  if (!isSelect(next)) {
    throw new Error(`unexpected protocol request: ${JSON.stringify(next)}`);
  }
  awaiting = "select";
  const frame = await select(loaded.policy, request.request);
  awaiting = undefined;
  send(frame);
}

interface Evaluate {
  type: "evaluate";
  protocol: number;
  entry: string;
  request: unknown;
}

function isEvaluate(value: unknown): value is Evaluate {
  if (typeof value !== "object" || value === null) return false;
  const message = value as Record<string, unknown>;
  return message.type === "evaluate" && message.protocol === PROTOCOL && typeof message.entry === "string";
}

function isSelect(value: unknown): boolean {
  if (typeof value !== "object" || value === null) return false;
  const message = value as Record<string, unknown>;
  return message.type === "select" && message.protocol === PROTOCOL;
}

/** The frame reporting the entry, and the policy object when there is one. */
interface Loaded {
  frame: object;
  policy?: Record<string, unknown>;
}

async function load(entry: string): Promise<Loaded> {
  let module: Record<string, unknown>;
  try {
    module = await import(entry);
  } catch (error) {
    return { frame: { type: "failure", stage: "load", ...describe(error) } };
  }
  if (!Object.hasOwn(module, "policy")) {
    return invalid("the entry has no named export `policy`");
  }
  const policy = module.policy;
  if (typeof policy !== "object" || policy === null || Array.isArray(policy)) {
    return invalid(`\`policy\` must be an object, found ${kindOf(policy)}`);
  }
  const prototype = Object.getPrototypeOf(policy);
  if (prototype !== Object.prototype && prototype !== null) {
    // Only own enumerable fields survive serialization, so a class instance's
    // prototype methods would vanish unseen rather than be refused.
    return invalid("`policy` must be a plain object, not a class instance");
  }
  let snapshot: unknown;
  try {
    snapshot = JSON.parse(JSON.stringify(policy, mark));
  } catch (error) {
    return invalid(`\`policy\` cannot be serialized: ${describe(error).message}`);
  }
  return { frame: { type: "policy", policy: snapshot }, policy: policy as Record<string, unknown> };
}

/**
 * Call `select` as a method of its policy, and report what it produced as
 * data: the value it returned or resolved to, or what it threw. The front
 * judges every value, abstention included.
 */
async function select(policy: Record<string, unknown>, request: unknown): Promise<object> {
  let result: unknown;
  try {
    result = await (policy.select as (request: unknown) => unknown).call(policy, request);
  } catch (error) {
    return { type: "failure", stage: "select", ...describe(error) };
  }
  // JSON has no `undefined`; the front reads `null` as the same abstention.
  if (result === undefined) return { type: "selection", result: null };
  try {
    return { type: "selection", result: JSON.parse(JSON.stringify(result, mark)) };
  } catch (error) {
    return { type: "failure", stage: "select", unserializable: true, ...describe(error) };
  }
}

function invalid(message: string): Loaded {
  return { frame: { type: "failure", stage: "validation", location: "policy", message } };
}

// Values JSON cannot carry are replaced by a marker object, never dropped and
// never turned into a string that could pass as one: a function where a model
// string belongs must be refused as a function.
function mark(_key: string, value: unknown): unknown {
  if (typeof value === "function" || typeof value === "bigint" || typeof value === "symbol") {
    return { $harnessDispatch: typeof value };
  }
  if (value instanceof Map || value instanceof Set) {
    return { $harnessDispatch: value instanceof Map ? "Map" : "Set" };
  }
  return value;
}

function kindOf(value: unknown): string {
  if (value === null) return "null";
  if (Array.isArray(value)) return "an array";
  return typeof value;
}

function describe(error: unknown): { name: string; message: string } {
  if (error instanceof Error) return { name: error.name, message: error.message };
  if (typeof error === "object" && error !== null && "message" in error) {
    const record = error as { name?: unknown; message: unknown };
    return { name: String(record.name ?? "Error"), message: String(record.message) };
  }
  return { name: "thrown value", message: String(error) };
}

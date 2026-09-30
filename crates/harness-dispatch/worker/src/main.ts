// harness-dispatch-policy — the private worker that evaluates an owner's policy.
//
// The Rust front process starts this executable in a private empty directory,
// with null stdin, a fresh environment and descriptor 3 as its protocol channel
// (see `channel.ts`). The conversation is fixed:
//
//   1. Before touching any policy code, the worker registers the embedded
//      package specifiers and announces its protocol and build identity.
//   2. The front verifies that identity and only then sends the entry to
//      evaluate. A mismatched worker is never given a policy.
//   3. The worker imports the entry and returns a serializable snapshot of its
//      `policy` export, or a failure naming the stage.
//
// The worker judges nothing it can hand over as data. The front validates the
// snapshot's shape, resolves routes and reports every refusal with its
// location, so there is one validator and it is the one inspection reports.

import { receive, send } from "./channel.ts";
import * as sdk from "../sdk/index.ts";
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
// The examples import `harness-dispatch/sdk` as an owner's policy does. The
// bundler resolves that through `paths` in `worker/tsconfig.json` when the
// worker is compiled, to the same module imported above, so an example and a
// policy that imports it share one SDK. (Bun reads tsconfig at build time;
// `--no-compile-autoload-tsconfig` governs only the compiled worker at run
// time: https://github.com/oven-sh/bun/blob/bun-v1.4.2/docs/bundler/executables.mdx)
const embedded: Readonly<Record<string, object>> = {
  "harness-dispatch/sdk": sdk,
  "harness-dispatch/examples/static": staticExample,
  "harness-dispatch/examples/grove-static": groveStaticExample,
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

const request = receive();
if (!isEvaluate(request)) {
  throw new Error(`unexpected protocol request: ${JSON.stringify(request)}`);
}
send(await evaluate(request.entry));
process.exit(0);

interface Evaluate {
  type: "evaluate";
  protocol: number;
  entry: string;
}

function isEvaluate(value: unknown): value is Evaluate {
  if (typeof value !== "object" || value === null) return false;
  const message = value as Record<string, unknown>;
  return message.type === "evaluate" && message.protocol === PROTOCOL && typeof message.entry === "string";
}

async function evaluate(entry: string): Promise<object> {
  let module: Record<string, unknown>;
  try {
    module = await import(entry);
  } catch (error) {
    return { type: "failure", stage: "load", ...describe(error) };
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
  try {
    return { type: "policy", policy: JSON.parse(JSON.stringify(policy, mark)) };
  } catch (error) {
    return invalid(`\`policy\` cannot be serialized: ${describe(error).message}`);
  }
}

function invalid(message: string): object {
  return { type: "failure", stage: "validation", location: "policy", message };
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

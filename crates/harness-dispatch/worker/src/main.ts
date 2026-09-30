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
//   4. The front judges the snapshot. If the policy has a `loadContext`, or the
//      caller gave a context, it asks the worker for the context, and the
//      worker returns the loader's result, or the caller's context, with every
//      source it measured, or a failure.
//   5. For a `routes` policy, or one it refuses, the front closes the channel
//      and the worker exits. For a valid `select` policy it asks the worker to
//      select, and the worker calls `select` with the request and the measured
//      context, and returns what it produced.
//
// The worker judges nothing it can hand over as data. The front validates the
// snapshot's shape, resolves routes, checks any explicit choice, validates and
// measures the context, validates a selection against the snapshot and reports
// every refusal with its location, so there is one validator and it is the one
// inspection reports. `select` runs only once that validator has accepted the
// policy it belongs to and the context it is given, and the snapshot it is
// checked against was taken before it ran. What the worker does enforce is the
// bounds, because only it sees a read or an encoding before it is sent: each
// is checked here and again by the front.

import { encode, receive, send, sendEncoded } from "./channel.ts";
import { type Bounds, type Measured, Session, SourceUnreadable } from "./host.ts";
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
let awaiting: "load" | "context" | "select" | undefined;
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
  const evaluate = receive();
  if (evaluate === null) return;
  if (!isEvaluate(evaluate)) {
    throw new Error(`unexpected protocol request: ${JSON.stringify(evaluate).slice(0, 400)}`);
  }
  // The policy sees the request frozen, so nothing it does to it can reach
  // `select`'s view of it, or pass for measured context.
  const request = deepFreeze(evaluate.request);
  const session = new Session(request.cwd, evaluate.bounds, evaluate.measured);
  const limit = evaluate.bounds.messageBytes;

  awaiting = "load";
  const loaded = await load(evaluate.entry);
  awaiting = undefined;
  if (!sendBounded(loaded.frame, "load", limit) || loaded.policy === undefined) return;

  let next = receive();
  if (next === null) return;
  let delivered: unknown;
  if (isRequest(next, "context")) {
    awaiting = "context";
    const assembled = await assemble(loaded.policy, request, session);
    awaiting = undefined;
    if ("delivered" in assembled) {
      sendEncoded(assembled.body);
      delivered = assembled.delivered;
    } else if (!sendBounded(assembled.frame, "context", limit) || assembled.frame.type !== "context") {
      return;
    }
    next = receive();
    if (next === null) return;
  }
  if (!isRequest(next, "select")) {
    throw new Error(`unexpected protocol request: ${JSON.stringify(next).slice(0, 400)}`);
  }
  awaiting = "select";
  const frame = await select(loaded.policy, request, delivered, session);
  awaiting = undefined;
  sendBounded(frame, "select", limit);
}

interface Request {
  readonly cwd: string;
  readonly context?: unknown;
}

interface Evaluate {
  type: "evaluate";
  protocol: number;
  entry: string;
  request: Request;
  bounds: Bounds;
  measured: Measured[];
}

function isEvaluate(value: unknown): value is Evaluate {
  if (typeof value !== "object" || value === null) return false;
  const message = value as Record<string, unknown>;
  return (
    message.type === "evaluate" &&
    message.protocol === PROTOCOL &&
    typeof message.entry === "string" &&
    typeof (message.request as Request | undefined)?.cwd === "string" &&
    typeof message.bounds === "object" &&
    Array.isArray(message.measured)
  );
}

function isRequest(value: unknown, type: "context" | "select"): boolean {
  if (typeof value !== "object" || value === null) return false;
  const message = value as Record<string, unknown>;
  return message.type === type && message.protocol === PROTOCOL;
}

/**
 * Send `frame` if it encodes within the protocol message bound, and otherwise
 * report the overflow in its place. Whether it was sent.
 */
function sendBounded(frame: object, stage: "load" | "context" | "select", limit: number): boolean {
  const body = encode(frame);
  if (body.length > limit) {
    send({ type: "failure", stage, bound: { name: "message", actual: body.length } });
    return false;
  }
  sendEncoded(body);
  return true;
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

/** A frame to the front, by its type. */
interface Frame {
  readonly type: string;
  readonly [field: string]: unknown;
}

/** The context frame, already encoded and within its bound, or a failure. */
type Assembled = { body: Buffer; delivered: unknown } | { frame: Frame };

/**
 * Run the policy's `loadContext`, if it has one, with the request and a host
 * whose reads are measured; without one, the context is the caller's. Encode
 * the result as JSON, and check that it, with the measured sources attached,
 * fits the context budget. `select` will receive exactly the value this
 * encoding parses to, which is the value the front measures.
 */
async function assemble(policy: Record<string, unknown>, request: Request, session: Session): Promise<Assembled> {
  let context: unknown;
  if (typeof policy.loadContext === "function") {
    session.reading = true;
    try {
      context = await (policy.loadContext as (request: unknown, host: object) => unknown).call(
        policy,
        request,
        session.contextHost(),
      );
    } catch (error) {
      if (session.breach !== undefined) return { frame: breachFrame("context", session) };
      const unreadable = sourceUnreadable(error);
      if (unreadable !== undefined) {
        return {
          frame: {
            type: "failure",
            stage: "context",
            sourceUnreadable: { source: unreadable.source, message: unreadable.message },
          },
        };
      }
      return { frame: { type: "failure", stage: "context", ...describe(error) } };
    } finally {
      session.reading = false;
    }
  } else {
    context = request.context;
  }
  if (session.breach !== undefined) return { frame: breachFrame("context", session) };

  const measured = [...session.ledger];
  let encoded: string;
  try {
    // `undefined` abstains; JSON has no such value, so it travels as `null`.
    encoded = context === undefined ? "null" : JSON.stringify(context, markData);
  } catch (error) {
    return { frame: { type: "failure", stage: "context", unserializable: true, ...describe(error) } };
  }
  const value: unknown = JSON.parse(encoded);
  const delivered = isPlainObject(value) ? { ...value, measured } : value;
  const budget = session.bounds.contextBytes;
  const size = Buffer.byteLength(JSON.stringify(delivered), "utf8");
  // The frame also carries what a loader supplied that the front will refuse,
  // such as a `measured` of its own, so it is bounded as well.
  const body = encode({ type: "context", context: value, measured });
  if (size > budget || body.length > budget + CONTEXT_ENVELOPE) {
    session.breach = { bound: { name: "context", actual: Math.max(size, body.length - CONTEXT_ENVELOPE) } };
    return { frame: breachFrame("context", session) };
  }
  return { body, delivered: deepFreeze(delivered) };
}

/** What a context frame carries beyond the delivered context, as the front allows. */
const CONTEXT_ENVELOPE = 1024;

/**
 * Call `select` as a method of its policy, with the request, the measured
 * context and a host without reads, and report what it produced as data: the
 * value it returned or resolved to, or what it threw. The front judges every
 * value, abstention included.
 */
async function select(
  policy: Record<string, unknown>,
  request: unknown,
  context: unknown,
  session: Session,
): Promise<object> {
  let result: unknown;
  try {
    result = await (policy.select as (request: unknown, context: unknown, host: object) => unknown).call(
      policy,
      request,
      context,
      session.selectHost(),
    );
  } catch (error) {
    if (session.breach !== undefined) return breachFrame("select", session);
    return { type: "failure", stage: "select", ...describe(error) };
  }
  if (session.breach !== undefined) return breachFrame("select", session);
  // JSON has no `undefined`; the front reads `null` as the same abstention.
  if (result === undefined) return { type: "selection", result: null };
  try {
    return { type: "selection", result: JSON.parse(JSON.stringify(result, mark)) };
  } catch (error) {
    return { type: "failure", stage: "select", unserializable: true, ...describe(error) };
  }
}

/** The recorded breach, as the failure frame of `stage`. */
function breachFrame(stage: "context" | "select", session: Session): Frame {
  return { type: "failure", stage, ...session.breach };
}

/** The failed read `error` is, or was caused by, if any. */
function sourceUnreadable(error: unknown): SourceUnreadable | undefined {
  for (let cause = error, depth = 0; cause !== undefined && depth < 8; depth++) {
    if (cause instanceof SourceUnreadable) return cause;
    cause = cause instanceof Error ? cause.cause : undefined;
  }
  return undefined;
}

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** `value`, with every object and array in it frozen. */
function deepFreeze<T>(value: T): T {
  if (typeof value === "object" && value !== null && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const key of Object.keys(value)) deepFreeze((value as Record<string, unknown>)[key]);
  }
  return value;
}

function invalid(message: string): Loaded {
  return { frame: { type: "failure", stage: "validation", location: "policy", message } };
}

// Values JSON cannot carry are replaced by a marker object, never dropped and
// never turned into a string that could pass as one: a function where a model
// string belongs must be refused as a function.
function mark(_key: string, value: unknown): unknown {
  return marked(value);
}

// A context is data throughout, so it is held to more: a number JSON would
// write as `null` and a string with a lone surrogate, which the front cannot
// read as Unicode, are marked too, and the front refuses each where it sits.
function markData(_key: string, value: unknown): unknown {
  if (typeof value === "number" && !Number.isFinite(value)) {
    return { $harnessDispatch: `non-finite number (${value})` };
  }
  if (typeof value === "string" && !value.isWellFormed()) {
    return { $harnessDispatch: "string with a lone surrogate" };
  }
  return marked(value);
}

function marked(value: unknown): unknown {
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

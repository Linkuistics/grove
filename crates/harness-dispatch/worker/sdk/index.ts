// harness-dispatch/sdk — the types and helpers an owner's policy imports.
//
// The running worker registers this module under the specifier
// `harness-dispatch/sdk` before it imports the selected policy entry, so a
// policy always receives the SDK compiled into the worker that evaluates it.
// There is no package to install and no `node_modules` to maintain. These types
// describe what the front process validates; they are not the validation. A
// policy that satisfies them can still be refused at run time, and every
// refusal names the location it found.
//
// A policy is one `select` function. It receives what the caller passed, the
// prompt included, and returns the command to run or a refusal. There is no
// catalog and no table of kinds here: a table from kind to command is something
// your `select` consults. Grove's task conventions are not here either: the
// explicit `harness-dispatch/grove` adapter reads them.

/**
 * What `select` receives as `request.prompt` when `harness-dispatch inspect`
 * was given no prompt. A policy that only places the prompt in its arguments
 * needs no check: inspection then shows this marker where the prompt goes. A
 * policy that reads the prompt can compare it with this, and selects from the
 * marker otherwise, so inspection reproduces such a selection only when it is
 * given the same prompt. `run` always has a prompt.
 */
export const PROMPT_NOT_SUPPLIED = "<harness-dispatch inspect: no prompt was supplied>";

/** Any JSON value. A context carries data, never code. */
export type Json = null | boolean | number | string | readonly Json[] | { readonly [key: string]: Json };

/**
 * A piece of evidence a context attributes: its name, and its SHA-256, its
 * version or both, so that the context never claims evidence it cannot pin.
 * A host read returns one, ready to put in `sources`.
 */
export type SourceRecord = {
  /** Nonblank: a path, URL or other name for the evidence. */
  readonly name: string;
  /** Its length, when known. */
  readonly bytes?: number;
} & (
  | { readonly sha256: string; readonly version?: string }
  | { readonly sha256?: string; readonly version: string }
);

/** A judgment in a context, attributed to whoever made it. */
export interface Assessment {
  /** Nonblank: who assessed it, such as `owner` or `triage-model`. */
  readonly by: string;
  readonly value: Json;
}

/**
 * The run that created a reviewed artifact, or its owner's declaration of the
 * provider that did: exactly one of the two.
 */
export type Creator = { readonly run: string; readonly declared?: never } | { readonly declared: string; readonly run?: never };

/** The artifact a review is of, and at most one creator form. */
export interface ReviewedArtifact {
  /** Nonblank, such as a task's stable identity. */
  readonly id: string;
  readonly creator?: Creator;
}

/**
 * A version-1 context: what a caller supplies with `--context`, and what
 * `loadContext` returns. Every field but the version is optional, and absent
 * means unknown: an empty `facts` is a known empty set, not a missing one.
 * Unknown fields are refused, and so is any field named like an executable
 * one, such as `program` or `args`.
 */
export interface Context {
  readonly schemaVersion: 1;
  readonly summary?: string;
  readonly acceptanceCriteria?: readonly string[];
  /** The caller's facts, as data. */
  readonly facts?: { readonly [key: string]: Json };
  /** Judgments about the work, each attributed. */
  readonly assessments?: { readonly [name: string]: Assessment };
  /** The evidence the context draws on; at most 256 records. */
  readonly sources?: readonly SourceRecord[];
  readonly reviewedArtifact?: ReviewedArtifact;
}

/** One source harness-dispatch measured for the context. */
export interface MeasuredSource {
  /** The canonical path, or for a run lookup the run ID. */
  readonly name: string;
  /** The `--context` document, or the host operation that delivered it. */
  readonly via: "--context" | "readText" | "readJson" | "run";
  /**
   * The bytes actually read; for a run lookup, the length of its answer's
   * compact JSON encoding with sorted keys.
   */
  readonly bytes: number;
  /** The SHA-256 of those bytes, in lowercase hexadecimal. */
  readonly sha256: string;
}

/**
 * harness-dispatch's own record that a run's harness never started, as
 * `record show` exports it: when it was appended, its `cause`, and the rest
 * of its detail.
 */
export interface LaunchFailure {
  readonly recordedAt: string;
  /**
   * `exec_error` when exec returned an error; `cancelled` when a signal
   * stopped the launch after the handoff was committed. Either way the
   * harness was not executed.
   */
  readonly cause: string;
  readonly [field: string]: Json;
}

/**
 * A run the record store holds: its immutable launch fields. The labels are
 * the ones the run was launched under, which a later change to a policy does
 * not alter; its program and arguments are not part of a lookup. `taskId` is
 * `null` when the run was given none. A run whose `launchFailure` is `null`
 * was handed off, but whether its harness ran is not known from this.
 */
export interface FoundRun {
  readonly runId: string;
  readonly status: "found";
  /** When its handoff was committed, as an RFC 3339 UTC timestamp. */
  readonly recordedAt: string;
  readonly kind: string;
  readonly taskId: string | null;
  /** The provider-origin label the run was launched under. */
  readonly provider: string;
  readonly model: string;
  readonly effort: string;
  readonly launchFailure: LaunchFailure | null;
}

/** A run the record store does not hold, or no store at all. */
export interface MissingRun {
  readonly runId: string;
  readonly status: "missing";
}

/** What `host.run` returns, and what the delivered context carries in `runs`. */
export type RunLookup = FoundRun | MissingRun;

/**
 * The context `select` receives: the context `loadContext` returned, or the
 * caller's without a loader, and every source harness-dispatch measured for
 * it, the `--context` document first and then each read and run lookup in
 * order. Its whole JSON encoding, `measured` and `runs` included, is within
 * `limits.contextBytes`, and it is exactly what inspection shows. It is
 * frozen.
 */
export interface DeliveredContext extends Context {
  readonly measured: readonly MeasuredSource[];
  /**
   * Every answer `loadContext` received from `host.run`, in call order,
   * attached by harness-dispatch: a loader cannot supply it, so a provider
   * found here is the record store's, never a transcription. Absent when
   * nothing was looked up.
   */
  readonly runs?: readonly RunLookup[];
}

/** What a host read returns beside the content: a source record to attribute it. */
export interface ReadSource {
  readonly name: string;
  readonly bytes: number;
  readonly sha256: string;
}

/** The host `select` receives. */
export interface SelectHost {
  /** Write one line of diagnostic text, which inspection shows as the policy's stderr. */
  diagnostic(text: string): void;
  /**
   * Aborted when harness-dispatch stops the selection, at its deadline, so
   * that work in flight, such as a `fetch`, can stop too.
   */
  readonly signal: AbortSignal;
}

/**
 * The host `loadContext` receives. Its reads and run lookups are the only
 * measured sources: each read resolves a relative path against `request.cwd`,
 * reads a regular file once, and returns its content with the canonical name,
 * byte count and SHA-256 of the bytes read. A read over its limit, or a read
 * or lookup past the 256th source, refuses the selection, even if the error
 * it throws is caught. A missing or unreadable source throws an error that, if
 * `loadContext` fails because of it, names that source in the refusal.
 */
export interface ContextHost extends SelectHost {
  /**
   * Read a UTF-8 text file. `maxBytes` defaults to `limits.sourceBytes` and
   * may be set up to `limits.contextBytes`, never beyond.
   */
  readText(path: string, maxBytes?: number): { readonly text: string; readonly source: ReadSource };
  /** Read a JSON file, as `readText` does, and parse it. */
  readJson(path: string, maxBytes?: number): { readonly value: Json; readonly source: ReadSource };
  /**
   * Look a run up by its run ID in this invocation's record store: its
   * immutable launch fields and any launch failure, or that the store does not
   * hold it. There is no lookup by task or artifact identity. The answer is
   * frozen, and is also delivered to `select` in the context's `runs`, as a
   * measured source. An ID that is not in the canonical run-ID form throws a
   * `TypeError`. A store that exists but cannot be read, or is not a version
   * this release reads, refuses the selection at once; it never reads as a
   * missing run.
   */
  run(runId: string): RunLookup;
}

/**
 * Assemble the context `select` will receive. It may be asynchronous, and it
 * runs within the selection bound. It returns a version-1 context, typically
 * built from `request.context` and host reads, and harness-dispatch attaches
 * the measured sources. A loader that throws, rejects or returns nothing
 * refuses the selection, and nothing is selected without its context.
 *
 * A loader that finds what it requires invalid, rather than unreadable, can
 * say why by returning a {@link Refused}, as `select` does. The selection then
 * refuses as the policy's own, `policy_refused` with its code, message and
 * remedy, and `select` is not called.
 */
export type LoadContext = (
  request: SelectionRequest,
  host: ContextHost,
) => Context | Refused | PromiseLike<Context | Refused>;

/**
 * A policy: the one named export, `policy`, of the selected entry.
 *
 * `select` may be synchronous or return a promise, and may compute anything,
 * within the whole-selection bound in `request.limits`. It receives the
 * request, the measured context and a host without reads. harness-dispatch
 * validates the shape of what it returns and nothing about its content: it
 * does not check that the prompt is among the arguments, that the program is
 * one you listed anywhere, or that a kind has a route. Those are your
 * function's to hold, with the type checker and whatever tables it keeps. A
 * result that adds a field, abstains with `undefined` or `null`, throws,
 * rejects or is left unsettled refuses, and nothing is run in its place.
 *
 * A policy that turns outside text into a command, such as an agent's answer
 * or a file's content, should choose among commands its own code wrote and
 * never place that text in `program` or `args`.
 *
 * `harness-dispatch/examples/static` and `harness-dispatch/examples/grove-static`
 * are editable starting points whose `select` consults an exact table by kind.
 */
export interface Policy {
  readonly schemaVersion: 2;
  /** Nonblank, owner-maintained version, reported by inspection. */
  readonly version: string;
  /** Optional: assembles the context `select` receives. */
  readonly loadContext?: LoadContext;
  /**
   * `context` is what `loadContext` returned, or the caller's `--context`
   * without a loader, with its measured sources; `undefined` when there is
   * neither.
   */
  select(
    request: SelectionRequest,
    context: DeliveredContext | undefined,
    host: SelectHost,
  ): SelectionResult | PromiseLike<SelectionResult>;
}

/**
 * What `loadContext` and `select` are asked: everything the caller passed.
 * Optional fields are absent, not empty, when the caller did not supply them.
 * It carries no run identity: a harness reads its own from
 * `HARNESS_DISPATCH_RUN_ID`. It is frozen.
 */
export interface SelectionRequest {
  readonly schemaVersion: 2;
  /** The caller's kind, an open token. */
  readonly kind: string;
  /**
   * The caller's prompt, byte for byte, or {@link PROMPT_NOT_SUPPLIED} under
   * an `inspect` that was given none.
   *
   * The prompt is a mandate: under Grove it tells its reader to load a skill
   * and carry out a task. A policy that hands it to a deciding agent starts
   * that agent itself, in `cwd`, and nothing in harness-dispatch or Grove keeps
   * the agent from doing what the prompt says instead of evaluating it.
   * Keeping the agent from executing the task, and from changing the caller's
   * tree, is your job as the policy's owner: what the agent is told, which
   * tools it has and whether it can write are all set by the policy that
   * starts it.
   */
  readonly prompt: string;
  /** The caller's working directory, as data; the worker does not run in it. */
  readonly cwd: string;
  /**
   * Every `--param NAME=VALUE` the caller passed, by name; empty when it
   * passed none. harness-dispatch gives a parameter no meaning: only your
   * policy reads it, and a name the caller did not pass is absent.
   */
  readonly params: { readonly [name: string]: string };
  /** The caller's `--task-file`, absolute; not read on the policy's behalf. */
  readonly taskFile?: string;
  /** The caller's `--task-id`. */
  readonly taskId?: string;
  /** The caller's `--context` document, validated, as data. */
  readonly context?: Context;
  readonly limits: Limits;
}

/**
 * The bounds in effect for this invocation. They are for reading: the worker
 * enforces its own copies, and a policy cannot raise one.
 */
export interface Limits {
  /** The whole-selection bound, in milliseconds, from the worker's start. */
  readonly selectionMs: number;
  /** The delivered context's JSON encoding, `measured` included, in bytes. */
  readonly contextBytes: number;
  /** One host read's default limit, in bytes; `maxBytes` may raise it to `contextBytes`. */
  readonly sourceBytes: number;
  /** Measured sources, the `--context` document included. */
  readonly sources: number;
  /** A policy snapshot or selection result, as a protocol message, in bytes. */
  readonly messageBytes: number;
  /** Everything the policy prints on stdout and stderr together, in bytes. */
  readonly diagnosticsBytes: number;
}

/**
 * The command to run, and the owner's labels for what it runs.
 *
 * harness-dispatch executes `program` with `args`, each string one whole
 * argument, with no shell, no splitting and no second reading of any text. A
 * caller value inside an argument is ordinary string building. `provider`,
 * `model` and `effort` are your labels: they are never inferred from the
 * program or its arguments, they need not appear in `args`, and they reach
 * inspection and the run record as you wrote them. Every field is a nonblank
 * string except `args`, whose strings may be empty, and neither `program`
 * nor an argument may hold a NUL.
 */
export interface Selected {
  readonly status: "selected";
  /**
   * An absolute path, a name looked up in the caller's PATH, or a relative
   * path containing a separator, resolved against the caller's cwd. It is also
   * the harness's argv[0].
   */
  readonly program: string;
  /** The command's arguments after `program`. */
  readonly args: readonly string[];
  /**
   * The model's origin as you declare it. A gateway change does not make a
   * different provider.
   */
  readonly provider: string;
  /** The model, as your harness understands it. */
  readonly model: string;
  /** The reasoning effort, as your harness understands it. */
  readonly effort: string;
  /** Inspection and the run record report it verbatim. */
  readonly reason: string;
}

/**
 * The policy's own refusal, from `select` or `loadContext`. It is reported as
 * `policy_refused`, exit 3, with `code` beside it as `policyCode`, and
 * launches nothing.
 */
export interface Refused {
  readonly status: "refused";
  /** Nonblank; the policy's own code, such as `no_reviewer`. */
  readonly code: string;
  /** Nonblank: what is wrong. */
  readonly message: string;
  /** Nonblank: what the caller or owner can do about it. */
  readonly remedy: string;
}

/** What `select` returns, or resolves to. */
export type SelectionResult = Selected | Refused;

/**
 * Returns its argument unchanged. It exists so that
 * `export const policy = definePolicy({ ... })` type-checks the literal against
 * {@link Policy}, a field the contract does not have included, and types
 * `select`'s parameters without an annotation.
 */
export function definePolicy(policy: Policy): Policy {
  return policy;
}

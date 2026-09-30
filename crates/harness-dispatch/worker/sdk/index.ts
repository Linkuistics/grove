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
// This release reads the static `routes` form and the computed `select` form.
// Context loading and host operations arrive in a later release, and
// `loadContext` is refused until then; nothing here stands in for them.

/**
 * The caller inputs one argument of a candidate's argument array can name.
 * A slot fills one whole argument; nothing is interpolated into a literal.
 *
 * Every candidate uses `prompt` exactly once. A slot whose optional input the
 * caller did not supply (`taskFile`, `taskId`) refuses the invocation rather
 * than filling the argument with nothing. `runId` is the run's own identity,
 * the one `run` records before it execs and exports as
 * `HARNESS_DISPATCH_RUN_ID`; inspection shows a proposed ID in its place.
 */
export type Slot = "prompt" | "kind" | "taskFile" | "taskId" | "model" | "effort" | "runId";

/** An argument filled from a caller input rather than written literally. */
export interface SlotArgument {
  readonly slot: Slot;
}

/** One word of a candidate's command: a literal string or a slot. */
export type Argument = string | SlotArgument;

/**
 * One configured joint choice of harness, model and reasoning effort.
 *
 * `provider` names the model's origin as the owner declares it. It is a
 * catalog value, never inferred from `program` or `args`, and a gateway change
 * does not make a different provider.
 */
export interface Candidate {
  /** Stable, unique catalog ID; routes and explicit choices name it. */
  readonly id: string;
  /** Nonempty provider-origin label. */
  readonly provider: string;
  /** Nonempty model string, as the owner's harness understands it. */
  readonly model: string;
  /** Nonempty reasoning-effort string, as the owner's harness understands it. */
  readonly effort: string;
  /**
   * An absolute path, a name looked up in the caller's PATH, or a relative
   * path containing a separator, resolved against the caller's cwd. It is also
   * the harness's argv[0].
   */
  readonly program: string;
  /** The command's arguments after `program`, with `prompt` exactly once. */
  readonly args: readonly Argument[];
}

/**
 * The static form: an exact table from session kind to candidate ID.
 *
 * There is no catch-all route and no fallback. A kind the table does not name
 * refuses as an incomplete mapping. A caller's explicit `--choice` selects the
 * catalog candidate it names without consulting the table, for any kind; an ID
 * the catalog lacks refuses.
 *
 * `harness-dispatch/examples/static` and `harness-dispatch/examples/grove-static`
 * are editable starting points in this form.
 */
export interface RoutesPolicy {
  readonly schemaVersion: 1;
  /** Nonempty, owner-maintained version, reported by inspection. */
  readonly version: string;
  readonly catalog: readonly Candidate[];
  /** Kind to candidate ID. Kinds are open tokens the caller supplies. */
  readonly routes: Readonly<Record<string, string>>;
  /** A policy has exactly one of `routes` or `select`. */
  readonly select?: never;
}

/**
 * The computed form: a callback that chooses one catalog candidate, or
 * refuses, for each invocation.
 *
 * It may be synchronous or return a promise, and may compute anything, within
 * the whole-selection bound in `request.limits`. It is called only once the
 * front has accepted the policy, catalog included, and it can name only a
 * candidate that catalog already holds. A result that names another, adds a
 * field, abstains with `undefined` or `null`, throws, rejects or is left
 * unsettled refuses; nothing is ever substituted.
 *
 * With a caller's explicit `--choice`, the request carries it as
 * `explicitChoice`, and the policy must accept it, by selecting that same ID,
 * or refuse. Any other ID refuses as `explicit_choice_mismatch`, whatever the
 * reason says. An ID the catalog lacks refuses before `select` is called.
 *
 * A policy that wants exact routes for most kinds and computation for a few
 * exports `select` and consults its own table, naming the entry it applied in
 * its reason. `harness-dispatch/examples/dynamic` does so.
 */
export interface SelectPolicy {
  readonly schemaVersion: 1;
  /** Nonempty, owner-maintained version, reported by inspection. */
  readonly version: string;
  readonly catalog: readonly Candidate[];
  select(request: SelectionRequest): SelectionResult | PromiseLike<SelectionResult>;
  /** A policy has exactly one of `routes` or `select`. */
  readonly routes?: never;
}

/**
 * What `select` is asked: the caller's data, never the prompt. Optional fields
 * are absent, not empty, when the caller did not supply them.
 */
export interface SelectionRequest {
  readonly schemaVersion: 1;
  /** The caller's kind, an open token. */
  readonly kind: string;
  /** The caller's working directory, as data; the worker does not run in it. */
  readonly cwd: string;
  /** The caller's `--task-file`, absolute; not read on the policy's behalf. */
  readonly taskFile?: string;
  /** The caller's `--task-id`. */
  readonly taskId?: string;
  /** The candidate ID the caller's `--choice` names, known to the catalog. */
  readonly explicitChoice?: string;
  readonly limits: Limits;
}

/** The bounds in effect for this invocation. */
export interface Limits {
  /** The whole-selection bound, in milliseconds, from the worker's start. */
  readonly selectionMs: number;
}

/** A candidate chosen, and why. */
export interface Selected<Id extends string = string> {
  readonly status: "selected";
  /** A catalog candidate's ID. */
  readonly candidateId: Id;
  /** Nonblank; inspection and the run record report it verbatim. */
  readonly reason: string;
}

/**
 * The policy's own refusal. It is reported as `policy_refused`, exit 3, with
 * `code` beside it as `policyCode`, and launches nothing.
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
export type SelectionResult<Id extends string = string> = Selected<Id> | Refused;

/** Every policy form this release evaluates. */
export type Policy = RoutesPolicy | SelectPolicy;

/**
 * Returns its argument unchanged. It exists so that
 * `export const policy = definePolicy({ ... })` type-checks the literal against
 * {@link Policy} while keeping its exact inferred type.
 */
export function definePolicy<const P extends Policy>(policy: P): P {
  return policy;
}

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
// This release reads the static `routes` form. Computed `select`, context
// loading and host operations arrive in later releases and are refused until
// then; nothing here stands in for them.

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
 * refuses as an incomplete mapping.
 */
export interface RoutesPolicy {
  readonly schemaVersion: 1;
  /** Nonempty, owner-maintained version, reported by inspection. */
  readonly version: string;
  readonly catalog: readonly Candidate[];
  /** Kind to candidate ID. Kinds are open tokens the caller supplies. */
  readonly routes: Readonly<Record<string, string>>;
}

/** Every policy form this release evaluates. */
export type Policy = RoutesPolicy;

/**
 * Returns its argument unchanged. It exists so that
 * `export const policy = definePolicy({ ... })` type-checks the literal against
 * {@link Policy} while keeping its exact inferred type.
 */
export function definePolicy<const P extends Policy>(policy: P): P {
  return policy;
}

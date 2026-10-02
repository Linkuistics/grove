# direct-dispatch-k3

## Goal

Say how the root brief's settled requirements are met, as simply as they can be,
and rework the dispatch specification and the ADR set to state it.

## Context

- The requirements are the root brief's. `plan-k1`'s log holds the human's words
  and what each choice was made against. Synthesise from them; do not re-ask.
- The human's steer, which is the test for every choice here: *"Simplicity of
  this code is critical"*, and their picture of the tool, *"a trivial
  TS-execution wrapper that passes it's CLI params as args to a TS function and
  acts according to the return value."* Take the answer with the least mechanism
  that meets the requirement.
- What the requirements leave to this session:
  1. how the caller's parameters and the prompt reach `select`, and the exact
     shape of what it returns;
  2. where an owner sets what Grove no longer passes (requirement 8), and the
     number for the selection time ceiling;
  3. how `grove run` keeps its confinement while dispatch still `exec`s
     (requirements 10 and 14);
  4. how Grove finds and runs dispatch, and what it reports when dispatch
     refuses;
  5. the sample, the subcommand that installs it, and the per-worktree helper;
  6. what becomes of each part of dispatch the human did not discuss, listed in
     the brief's last note. Leaving a working part alone is a legitimate answer.
- The ADRs and specs to rework in place are in the brief's *Pointers*.

## Done when

- The specification and the ADR set describe the design as current state, and
  every citation of a record this session deleted or merged is reconciled.
- Each item above has an answer a planning session can cut from.
- The planning leaf `direct-dispatch-k4` carries whatever it needs that the
  specification does not.

## Notes

The design is in `docs/specs/harness-selection-and-execution.md`,
`docs/specs/standalone-invocations.md` and
`docs/adr/harness-selection-is-owned-by-policy.md`. The log below holds each
call and what it was made against. This session had no dispatch run
(`HARNESS_DISPATCH_RUN_ID` was unset), so the review it cut carries no
`**Creator:**` line.

## Decisions (running log)

**The caller's parameters reach `select` as `request.params`, from a repeatable
`--param NAME=VALUE` (2026-10-02).** `--kind`, `--prompt`, `--task-file` and
`--task-id` stay as they are, because the run record and run lookup are keyed on
them. Grove passes `session_name`, `worktree` and `repo`, the names its command
templates use today. Chosen over making every input a parameter, which would
rewrite the record's keys for nothing a requirement asks.

**`select` returns the command flat (2026-10-02).** `{ status: "selected",
program, args, provider, model, effort, reason }` or `{ status: "refused", code,
message, remedy }`. `reason` stays required: it is how inspection shows which
table entry or arrangement applied. There is no tool-level check that the prompt
appears in `args`, and no run ID in the request: a harness reads its run ID from
`HARNESS_DISPATCH_RUN_ID`.

**The policy contract is version 2 (2026-10-02).** A version-1 policy refuses
with one clear remedy, where leaving the number alone would refuse it for a
missing `select` or an unknown result field. Nothing converts one.

**`loadContext`, measured reads, run lookup, run records, observations and the
other bounds are left alone (2026-10-02).** No requirement touches them, and the
review rule needs run lookup. Two reshapes follow from the catalog going: a run
lookup's answer carries `provider`, `model` and `effort` and no candidate ID, and
a new run's launch document writes `null` for the candidate ID, the selection
form and the explicit choice. The launch document stays version 1, so records
written by 21.13.0 stay readable and a review whose creator ran under it still
resolves.

**Inspection without a prompt evaluates with a marked placeholder (2026-10-02).**
That keeps today's behaviour for every policy that only places the prompt in
argv, and keeps a refused run's `inspect:` line usable. The line still omits the
prompt and says that a policy which reads the prompt needs it supplied. Chosen
over making the prompt optional in the request, which would put an `undefined`
check in every policy.

**Owner settings are a static file, `~/.config/harness-dispatch/settings.json`
(2026-10-02).** Keys `timeoutMs`, `contextBytes`, `stateDir` and `policyEnv`,
each the default for its flag. Chosen over settings exported by the policy,
because grants must be known before the worker starts and the time bound covers
its imports; and over environment variables, which a repository's `.envrc` could
set, against the rule that entering a repository grants nothing. The policy entry
needs no key: the personal entry can re-export another.

**The selection ceiling is 600 seconds; the default stays 30 (2026-10-02).**

**Confined `grove run` selects with `inspect --json` and launches the reported
command itself (2026-10-02).** Selection then runs outside the sandbox in Grove's
own environment, and the runner confines the returned command exactly as it
confines a template's argv today. Chosen over an exec-prefix flag on `run`: the
sandbox profile needs the resolved program before it can be built, so the prefix
needs a Grove helper executable, and dispatch would run inside `grove run`'s
scrubbed environment, which would strip the owner's policy grants. Chosen over a
record-then-print mode, which would make Grove set dispatch's run variables. The
cost: a standalone invocation has no run record.

**Grove runs the `harness-dispatch` beside its own executable (2026-10-02).** Not
PATH: the two ship in one `bin/`, so a sibling is the matching version. Grove
reports a refused launch as it reports any session that ends without a completion
signal, and points at dispatch's own diagnostic.

**The sample is one file, installed by `harness-dispatch init` (2026-10-02).**
`init` refuses when anything is at the personal default path and has no force
option. The per-worktree helper reads `.harness-dispatch-choice` in the directory
the policy names, with an ordinary read: its effect is in the recorded command
and its name is in `reason`, so a digest of a file holding a few names buys
little, and a measured read would force a loader on every policy that uses it.

**`examples/dynamic` is deleted, not renamed (2026-10-02).** Its subject was
policing `--choice`, which no longer exists; without it the example is
`examples/static`. Four examples and the sample ship.

**The two configuration ADRs and the modular-configuration spec are deleted with
the machinery, not here (2026-10-02).** Nearly every citation of them is in a
file that describes the configuration as live: code comments, the book chapters
that reproduce those comments byte for byte, and the architecture's session
configuration section. Deleting the records in a design session would leave those
pointers dangling for the rest of the grove, or have this session edit code. The
records this session reworked no longer cite them. `direct-dispatch-k4` carries
the deletion and its citation list. `docs/specs/module-decomposition.md`
decisions 6 and 7 wait for the same leaf, since their interface block reproduces
the code's signatures.

**The design gets a review leaf, `direct-dispatch-k5`, ahead of planning
(2026-10-02).** The specification is a contract a breaking release is built to,
which is the load-bearing artifact a review chain is for, and the human kept the
requirements review when the producing session proposed dropping it. Its body
carries the seven doubts this session could not close for itself. No in-session
reviewer was spent.

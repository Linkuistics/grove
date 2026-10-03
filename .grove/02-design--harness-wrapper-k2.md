# harness-wrapper-k2


## Goal

Establish how `harness-dispatch` becomes the process wrapper the root brief's
settled requirements describe, and deliver it as the reworked area
specification and ADR set: answer every question the brief leaves to design,
keep every guarantee it lists, and carry the four agreed seams into the
specification.

## Context

- `plan-k1`'s running log (`D1`–`D9`, in its retired leaf beside this one)
  holds each requirement in the owner's own words, with the options declined.
  Synthesise from it; do not re-interview (`SPEC-FORMAT.md`).
- Today's launch, end to end: the loop builds the `harness-dispatch run` argv
  and drives the epoch around the spawn in `crates/grove-loop/src/loop_driver.rs`;
  `crates/keyed-launch` owns the job, terminal handover, channel, escalation
  and confinement (`run.rs`, `channel.rs`, `confinement.rs`); dispatch's
  front commits the handoff and execs in `crates/harness-dispatch/src/run.rs`,
  with `cancellation.rs` holding the linearization point;
  `crates/grove/src/standalone.rs` runs `inspect`, confines through the
  runner and publishes on the `done` token; `grove-llm complete` writes the
  token. The session witness rides the runner's `Started`/`Reaped` events.
- What reads the parameters D1/D2 remove: the sample policy
  (`crates/harness-dispatch/worker/sample/policy.ts`) places `repo` in
  `--add-dir` and `session_name` in `claude -n`, and refuses without them; the
  typecheck fixtures under `crates/harness-dispatch/worker/typecheck/` use
  `repo` too.
- The seams' current suites: `crates/harness-dispatch/tests/` (`run.rs` drives
  a PTY), `crates/keyed-launch/tests/`, `crates/grove/tests/loop_driver.rs`
  (PTY) and `standalone.rs`, `crates/grove-llm/tests/complete.rs` and
  `standalone_completion.rs`.
- Beyond the brief's pointers: `docs/ARCHITECTURE.md` (*Process ownership*,
  and its harness-dispatch section), the area's visual document under
  `docs/design/harness-selection-and-execution/`, the methodology's signal
  contract (the prompt in `crates/grove-loop/src/prompt.rs`, the spine's
  `references/driver.md`, the conformance rows in
  `plugins/grove/conformance/rules.tsv`), and `docs/USAGE.md`.

## Done when

- `docs/specs/harness-selection-and-execution.md` states the supervised
  design as its contract, with the four agreed seams in its seams section,
  and `docs/specs/standalone-invocations.md` and decisions 7 and 9 of
  `docs/specs/module-decomposition.md` agree with it.
- The ADR set is reworked in place — no superseding record — and every
  citation of a merged, split or deleted record is reconciled.
- Each question under the brief's *Questions left to design* is answered in
  the specification or an ADR, and each listed guarantee is either kept or
  explicitly put back to the owner.
- The area's visual document matches the reworked specification.
- A `planning` leaf exists for a fresh session, after any review chain this
  session judges the reworked specification needs.

## Notes

- If a review is warranted, cut `review-design` with this stem first and the
  `planning` leaf after it, in this session: a review that finds something
  then `leaf-insert`s its integration ahead of planning, so planning reads an
  agreed design.
- The owner's major-release decision (D8) follows the breaking changes:
  whichever release ships them is the major one, if planning spreads the work
  over more than one grove.

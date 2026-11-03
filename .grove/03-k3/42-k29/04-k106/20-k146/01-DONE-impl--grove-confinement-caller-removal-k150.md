# grove-confinement-caller-removal-k150

## Goal

Remove confinement from Grove's standalone caller. `grove run` sends no
`--confine`, and `--runtime-read` is accepted with no effect.

## Context

- The sections "Guarantees of a standalone run", "The harness sandbox of a
  standalone kind", "The release-notes route", and the "Code" and "Tests"
  lists of the handoff in `docs/confinement-withdrawal-evidence.md`.
- `docs/specs/standalone-invocations.md`, which states the contract.
- The release-notes items of the "Scripts and Taskfile" list:
  `codex-headless.sh`, its test, `scripts/release-notes.sh`,
  `scripts/release-prepare.test.sh`, the fake `grove-fixture`,
  `scripts/templates/grove.rb.tmpl` and the `release` comment of
  `Taskfile.yml`.

## Done when

- The code and test items of the handoff are done.
- The release-notes route runs Codex with `--sandbox workspace-write`, as the
  handoff states.
- The book fragments that reproduce each changed source file are regenerated.
- `cargo nextest run --workspace`, the book check, the release Codex helper
  test, the release preparation test and `task ravel:review` pass.
- The change is a described commit.

## Notes

Do not run a real Codex call. grove-confinement-removal-checks-k154 runs it.

## Decisions (running log)

- **The detached shape removes the nine variables.** `ravel_job::launch`
  builds the command, so `standalone.rs` cannot change its environment.
  Only `grove run` uses `Shape::Detached`. So the detached arm removes the
  four repository selectors and the five multiplexer variables after the
  common scrub, and the lifecycle keeps them. The caller contract gets a row
  for them in its table of the two shapes.
- **The cancellation bound follows Ravel's kill-grace.** The harness of
  `cancellation_during_run_reaps_the_harness_and_preserves_enclosing_controls`
  ignores TERM. Without confinement, Ravel 1.1 sends KILL after its
  kill-grace of 5 seconds, so the run no longer ends inside 5 seconds. The
  design withdrew the guarantee of an immediate nested cancellation. The bound
  is now 15 seconds, the grace plus a margin. The test still fails if Grove
  does not forward the signal.
- **A caller variable is the control of the unconfined environment.**
  `CALLER_KEY` is in no grant list and no removal list. Two tests require the
  harness to receive it, so a pass of the removal checks cannot come from a
  harness that receives no environment.

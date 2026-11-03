# grove-confinement-removal-k146 — brief

## Goal

Remove confinement from Grove's standalone caller and from Grove's checks,
as the integrated design of confinement-withdrawal-k144 states.

## Context

- The integrated design of confinement-withdrawal-k142 and
  confinement-withdrawal-k144.
- The Ravel build of ravel-confinement-removal-k145, from its stage record.
- `crates/grove-loop/src/session_command.rs`, `crates/grove/src/standalone.rs`,
  `crates/grove/src/cli.rs`, `scripts/release-notes/codex-headless.sh`, the
  floor and release smoke scripts, the harness trial runner and the
  documents that the design lists.

## Done when

- `grove run` sends no `--confine`, and `--runtime-read` behaves as the
  design states.
- The release-notes route runs Codex under its own sandbox.
- Each test, script, floor case and acceptance row that the design lists is
  removed or changed.
- Grove's specifications, usage guide, release guide and books agree with the
  behaviour.
- `bash scripts/check.sh` passes with the 1.1 keg and with the new 2.0 build,
  as the "HOME of each run" recipe of
  `docs/grove-dual-ravel-stage-record.md` states.
- One `task release:smoke` run on `aarch64-unknown-linux-gnu` passes the
  retained directory case, as the checks of the design state.

## Notes

If the work is larger than one session, decompose it. The book and document
changes are a natural second child.

Do not publish a release or change the owner's settings. Keep the 1.1.0 keg
and its ending command unchanged.

## Decomposition

grove-confinement-removal-k146 was too big for one session, so it became this
node. The design handoff is the "Handoff to `grove-confinement-removal-k146`"
section of `docs/confinement-withdrawal-evidence.md`. Its sections divide the
work. The children run in this order:

1. `grove-confinement-caller-removal-k150`: the standalone caller, its tests,
   the policy assets and the release-notes route.
2. `grove-confinement-trial-runner-removal-k151`: the confined set-up of the
   harness trial runner, its two confined cases and the runner sections of the
   acceptance record.
3. `grove-confinement-floor-removal-k152`: the bubblewrap and confinement floor
   route, its Taskfile tasks and each consumer, with the directory case on the
   general route.
4. `grove-confinement-docs-removal-k153`: the remaining documents, the prose of
   the walkthrough books and the changelog.
5. `grove-confinement-removal-checks-k154`: the full checks with both Ravel
   builds, one real release-notes run and one `task release:smoke` run.

`bash scripts/check.sh` runs the final book check. So the first child
regenerates the book fragments of each source file that it changes, and the
fourth child changes only the prose.

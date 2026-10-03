# supervised-dispatch-k4

## Goal

Grow the tree that builds the agreed supervised-dispatch design: find the
smallest independently useful working increments, in an order that keeps every
commit green, and cut them as leaves (with their own review chains where a
leaf's artifact earns one), ending with the documentation that must describe the
result as current state before the major release.

## Context

- The design to build: `docs/specs/harness-selection-and-execution.md`
  (especially *Supervision*, *Confinement*, *Grove integration*, *Records*,
  *Diagnostics* and *Agreed test seams*), `docs/specs/standalone-invocations.md`,
  decisions 7 and 9 of `docs/specs/module-decomposition.md`, and the ADRs
  `dispatch-supervises-the-harness`, `policy-evaluation-precedes-the-launch`,
  `the-launched-child-is-a-job` and `one-live-driver-per-working-tree`.
  `harness-wrapper-k2`'s running log (`W1`–`W13`) gives each decision's reason;
  `harness-wrapper-k5`'s (`I1`–`I5`) gives the reasons for the corrections its
  review integrated.
- Code the design moves: `crates/keyed-launch` (token removed, optional
  channel, grants, terminal attributes restored on reclaim, the terminal taken
  back from whichever group a launch left holding it, the child's group killed
  on every exit before its reap and confirmed gone, entry signal state for a
  transparent caller, no handler over an ignored disposition, cancellation
  modes, `run_confined` writing to the launcher's own output);
  `crates/harness-dispatch/src/run.rs` and `cancellation.rs` (spawn and
  supervise instead of exec, the `exit` verb, `--exit-dir`, `--ending-file`,
  `--confine`, `--runtime-read`, the refusal of a confinement grant that reaches
  owner data, the end observation and the `ending` measurement, the end
  observation migrating a version-1 store, the supervision failure, the
  confinement record, `exit` exempt from owner settings);
  `crates/grove-loop/src/loop_driver.rs`, `driver_lease.rs` and `complete.rs`
  (the launch directory, `GROVE_LAUNCH_DIR`, admission on its path, the loop's
  reading of a launch, `record_teardown`, cleanup of abandoned launch
  directories); `crates/grove-llm/src/cli.rs` (`record-teardown`, `complete`
  removed); `crates/grove/src/standalone.rs` (`grove run` through
  `run --confine`); the sample policy and typecheck fixtures under
  `crates/harness-dispatch/worker/` (no parameters, the store derived from
  `.jj/repo`, no session name); `.cargo/config.toml` (clear `GROVE_LAUNCH_DIR`
  and `HARNESS_DISPATCH_EXIT_FILE`).
- Documentation that must follow (root brief's *Done when*): the methodology's
  signal contract (the prompt in `crates/grove-loop/src/prompt.rs`, the spine's
  `references/driver.md`, `grove-finish` and the other kind skills that name
  `grove-llm complete`, the conformance rows in
  `plugins/grove/conformance/rules.tsv`, the Codex-provisioned skills),
  `docs/ARCHITECTURE.md`, `docs/USAGE.md`, the dispatch README, this
  repository's `CLAUDE.md` finish sequence, and the walkthrough books of every
  changed crate (`keyed-launch`, `grove-loop`, `grove-llm`, `overview`), per
  `docs/specs/walkthrough-books.md`.

## Done when

- The tree holds leaves that, done in order, deliver the design, its four agreed
  seams and its documentation, each leaf a vertical slice that fits one session.
- The release they end in is a major one (D8), and a leaf owns its release notes:
  owner policies that read `repo` or `session_name` refuse until edited, and the
  skills and binaries must be upgraded together.

## Notes

- **Meta-grove sequencing.** This loop runs the installed v22 binaries and the
  marketplace-installed skills, not this checkout's. A session that follows a
  skill naming `harness-dispatch exit` under a v22 `harness-dispatch` cannot end
  its run, so the methodology's signal-contract edits must not reach the
  installed skills before binaries that carry the new verbs are installed with
  them; the major release is where both land. Every later session of this grove
  still ends with `grove-llm complete`, as its v22 prompt says.
- The owner's installed policy reads `repo` and `session_name`; it needs the
  D1/D2 edits when the release lands, which the release notes tell the owner.
- No automated test calls a model; the four agreed seams are the acceptance
  instruments and internal tests support them.

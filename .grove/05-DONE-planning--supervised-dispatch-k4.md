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

## Decisions (running log)

**P1 — one grove and one major release; the increments are ordered subtrees,
not separate groves.** The planning skill asks for a separate grove per stage
that leaves the product working. Here every grove ends in a release
(`CLAUDE.md`), and every release reaches every live grove on this machine
(`docs/RELEASING.md`), so a stage earns a grove of its own only if it could ship
alone. None can. After dispatch supervises but before Grove's launch moves to
it, Grove still escalates dispatch's group when its own channel appears, so
dispatch cancels its harness and records every successful Grove session as
`cancelled`. Releasing that would put false end observations in the owner's
store, and avoiding it needs a transitional protocol the design never chose.
The parameter change alone is breaking for owner policies (D1, D2), and D8 says
the breaking set ships at once with its skills. The owner settled the root
brief with the major release at this grove's finish, and the design's note
left the number of groves to planning. Each increment therefore becomes a
root-level leaf or node whose boundary leaves `bash scripts/check.sh` green and
the checkout's product working. Each increment maps one-to-one onto a grove, so
an owner who prefers separate groves can re-home the subtrees unchanged.

**P2 — the walkthrough books move with their source, not at the end.**
`scripts/check.sh` runs `book-check --final` over every book, and
`docs/specs/walkthrough-books.md` requires one commit to carry a source change
together with every affected page and manifest. Precedent agrees:
`dispatch-launch-k19` and `standalone-selection-k11` updated their books in the
leaf that changed the source. So every leaf that changes `crates/keyed-launch`,
`crates/grove-loop`, `crates/grove-llm` or `crates/grove` source updates that
crate's book (`keyed-launch`, `grove-loop`, `grove-llm`, `overview`), and the
book's structure spec under `docs/specs/` when its chapters move.
`harness-dispatch` has no book. Equally, `CHANGELOG.md` says a session logs its
change under `## Unreleased` when it makes it, so each leaf writes its own
entry. The release leaf then only states the major release's framing. The task
file's closing documentation list therefore narrows to the documents that no
test ties to a source change.

**P3 — the increments, in dependency order.** (1) A launch carries no selection
parameter (D1, D2), for the sample policy and both of Grove's dispatch calls
together. It is the smallest vertical slice and depends on nothing.
(2) Supervised run (D3, the dispatch half of D4, and D7): the runner contract
both supervisors share, which Grove's launches gain at once; then dispatch
spawns and supervises with `exit`; then it records and reports the ending.
(3) Confined run (D6): dispatch gains `--confine`, then `grove run` launches
through it. This must precede (4)'s removal of `grove-llm complete`, because
`grove run`'s harness acknowledges with `complete --done` today.
(4) Lifecycle cutover (D4 and D5 on Grove's side): first the launch directory,
its admission and the teardown record, with the old channel still working;
then the driver reading dispatch's ending, with the prompt naming the new verbs;
then retiring `complete`, `GROVE_SIGNAL_FILE` and the runner's token, along
with the methodology's signal contract. `instructed_verbs` ties the skills to
the CLI, so the skills cannot change in a leaf that does not also remove
`complete`. (5) Current-state documentation. (6) The major release's cutover.
Splitting (4) into three green steps follows the expand → migrate → contract
pattern rather than one cutover too large for a session.

**P4 — pre-cut reviews for the two subsystems; a lazy review for confinement;
no `review-planning`.** `supervised-run` (process groups, the terminal, signal
state across a spawn) and `launch-cutover` (the stale-session guarantee, the
loop's endings, the terminal after dispatch's death) are subsystems others
build on. Each gets a `review-impl` leaf cut now, right after its node, so its
review runs before the next increment builds on it. The session whose
retirement closes the node writes the `**Creator:**` line. `confined-run`
reuses existing backends and adds a narrower boundary, so its closing producer
judges whether to review it, with the doubts named in its brief. This
decomposition delivers a reviewed design whose leaves check themselves against
the four seams, so an adversarial read of it would mostly re-read the design.
The design itself was reviewed (`harness-wrapper-k3`).

**P5 — the meta-grove hazards the tree carries.** (a) This grove's sessions
must use the installed v22 `grove-llm` for every tree verb and for `complete`,
never `./target/debug/grove-llm` once its surface changes. No leaf installs the
pair before the finish, because any such install changes the binaries this
grove's own sessions end with. (b) The cargo guard keeps clearing
`GROVE_SIGNAL_FILE` beside the two new names for as long as this grove runs,
because its sessions carry a live v22 channel under that name. (c) The finish
runs under the v22 driver and prompt. Once `task release:major` installs the
new pair, `grove-llm complete` is gone from PATH, so the release leaf must
settle how this finish ends and write that into the root brief. (d) Other live
groves on this machine run v22 drivers whose sessions end with `complete`, so
the release owes them a stop-and-restart procedure. (e) Editing `CLAUDE.md`'s
finish sequence must not mislead this grove's own v22 finish. (f) The owner can
edit their installed policy ahead of the release into a form that works under
both versions, since a policy that ignores the parameters still selects when
v22 passes them. (a) and (b) go in the root brief for every session; (c)–(f)
belong to the release and documentation leaves.

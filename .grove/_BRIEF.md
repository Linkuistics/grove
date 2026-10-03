# grove.use-harness-dispatch-as-the-harness-execution-wrapper — brief

## Goal

Make `harness-dispatch` the process wrapper for one run of a harness, as its
owner directed: it selects the command, spawns the harness as its own child,
hands it the terminal, catches its exit — the session's own exit signal
included, which becomes dispatch's — applies the kill escalation, confines it
when asked, and records how the run ended. Grove keeps what is Grove's:
selecting the leaf, composing the prompt, recording a teardown, and deciding
from the run's ending whether the loop goes on.

## Done when

- `harness-dispatch run` supervises every harness it launches, a session ends
  its run with dispatch's exit signal, and dispatch records each run's end.
- Grove launches through it with no selection parameter, in the location the
  prompt assumes, and still tells reopening, teardown and declining apart.
- `grove run` launches its confined harness through dispatch.
- The area spec, the ADR set, the module contract, the methodology's signal
  contract, the usage and architecture documents and the walkthrough books of
  every changed crate describe the result as current state.
- The four agreed seams below pass, and the finish cuts a major release.

## Decomposition

1. `requirements` (`plan-k1`, done) — what to build, settled with the owner.
2. `design` (`harness-wrapper-k2`, done) — how: the specs and ADRs below,
   reworked in place; its running log (`W1`–`W13`) answers the questions left to
   design.
3. `review-design` (`harness-wrapper-k3`, done) — an adversarial read of that
   design; `integrate-review-design` (`harness-wrapper-k5`, done) applied its
   five findings.
4. `planning` (`supervised-dispatch-k4`) — grew the increments below from the
   agreed design; its running log (`P1`–`P5`) gives the reasons.
5. `no-selection-parameter-k6` — a launch carries no selection parameter, and
   the sample selects without one (D1, D2).
6. `supervised-run-k7` (node), then its review `supervised-run-k11` — dispatch
   spawns and supervises its harness for every caller, and records and
   reports the run ending (D3, D4's dispatch half, D7).
7. `confined-run-k12` (node) — dispatch confines what it supervises, and
   `grove run` launches through it (D6). It removes standalone's `done` token
   before the lifecycle cutover.
8. `launch-cutover-k15` (node), then its review `launch-cutover-k19` — Grove's
   lifecycle launch moves onto the supervised run, by expand → migrate →
   contract (D4, D5).
9. `current-state-docs-k20` — the documents no test ties to a source change.
10. `major-release-k21` — the release notes, the live-grove cutover, and how
    this grove's own finish ends.

Every boundary between them leaves `bash scripts/check.sh` green. They are one
grove, not several, because no boundary before the last could ship alone
(`P1`).

## Requirements settled with the owner

Each is recorded with the owner's own words in `plan-k1`'s running log (its
`D1`–`D9`); read that log for the reasons and the options declined.

- **The cwd is the session's location, and Grove passes no parameters.**
  Dispatch runs where the prompt assumes — the working-tree root for a
  lifecycle session, the staged directory for `grove run` — and its contract
  says so. Grove stops passing `worktree`, `repo` and `session_name`; a launch
  carries the kind, the task file, the task identity and the prompt. A policy
  that must grant a secondary workspace's harness its jj store derives it from
  `<cwd>/.jj/repo` (the sample shows how); nothing jj-specific enters dispatch.
  Session naming is the skill's.
- **`run` always supervises.** It spawns the harness as its child and stays its
  parent to the end, for every caller. A separate verb may keep today's `exec`
  handoff for a caller that needs the harness to keep its PID; Grove never
  uses it, and it catches nothing by construction.
- **Dispatch alone catches the harness's exit.** The verb a session ends with
  is dispatch's and is purely an exit signal: it carries nothing. Dispatch
  ends the harness through the kill escalation, reaps it, and reports to its
  caller whether the run ended through that signal, by the harness exiting on
  its own, or by cancellation. Grove no longer watches a channel or escalates.
- **Grove records a teardown itself.** A Grove verb records the teardown
  disposition in Grove's own launch-scoped control area; a finish session runs
  it before its final exit signal, so teardown stays a deliberate last act.
  The loop reads: teardown recorded → finished; ended through the exit signal
  → relaunch; otherwise → stopped, with the leaf live.
- **Confinement moves under dispatch.** `grove run` launches through `run`,
  which selects outside the sandbox and confines the harness it spawns; the
  invocation gets a run record and its harness a run ID. Grove keeps staging,
  the inputs, the declared outputs and publication.
- **Dispatch records the run's end.** When the harness ends, dispatch appends
  its own observation to the run — execution confirmed, how the run ended, the
  exit code or signal, the duration — in the existing observation format with
  dispatch as its source. Everything else stays with outside observers.
- **The release is major**: owner policies that read `repo` or `session_name`
  refuse until edited, and the skills and binaries must be upgraded together;
  the release notes say both.

Guarantees the design must keep, which the requirements move but do not
replace:

- A stale session cannot mutate the tree. Session-epoch admission keys on the
  `GROVE_SIGNAL_FILE` path today; once that path no longer signals, admission
  needs a launch identity Grove still allocates and the epoch still binds.
- A `grove run` publishes outputs only on the harness's explicit
  acknowledgement, never on its exit alone — today the `done` token.
- The harness receives the caller's entry signal mask and dispositions, its
  descendants die with it under the escalation, a typed Ctrl-C reaches it and
  not its wrapper, and the terminal comes back with its modes restored.
- Dispatch depends on no Grove package and stays extractable; the confined
  harness cannot read the policy, the owner settings or the record store.

## Questions left to design

The process and terminal chain from driver to dispatch to harness; where the
runner's code lives once dispatch owns its job; where the exit channel lives,
given that it must be writable from inside the harness's own sandbox and from
inside `grove run`'s confinement (today's signal file sits in the workspace's
`.jj/grove/`); the surfaces of the exit verb and of Grove's teardown verb, and
what a recorded teardown followed by the harness exiting on its own means; how
dispatch reports an ending to its caller; who sets the escalation graces; what
happens when dispatch itself dies mid-run; and whether the `exec` verb is
worth building for no current caller.

## Agreed test seams

Agreed with the owner in requirements, all four pre-existing; internal tests
support them and never replace them, and no automated test calls a model.

1. **Dispatch's command, fake harnesses, a controlling PTY** — with the
   stopped-child and two other-foreground-group cases exercised at the runner's
   process/PTY seam, as the spec's row 1 states; the remaining cases through
   dispatch's command: the harness in its own process group holding
   the terminal, reclaimed with its modes restored even after a raw-mode fake
   is killed; the entry signal state reaching it; a typed Ctrl-C reaching only
   it; the exit signal driving grace → TERM → kill-grace → KILL on its group,
   descendants reaped; its own exit code or signal reproduced; the ending
   reported to the caller; a fresh exit channel per run.
2. **Dispatch's records** — `record show` after a supervised run carries
   dispatch's end observation; a killed dispatch leaves the attempt as it
   stood; outside imports still append and correct.
3. **Grove's launch boundary** — the real driver, front and worker, a fake
   harness and a temporary-HOME policy, under a PTY: no `--param` reaches
   `select` and its `cwd` is the working-tree root; the three endings; a stale
   session's tree verbs refused after the epoch rotates.
4. **`grove run` under real confinement** with a deterministic harness:
   selection outside the sandbox, a recorded run ID inside it, no read of the
   policy, settings or store, and publication only on acknowledgement.

## Pointers

- ADRs to rework in place: `docs/adr/policy-evaluation-precedes-the-launch.md`
  (was `policy-evaluation-precedes-process-replacement`; supervision split out
  into `docs/adr/dispatch-supervises-the-harness.md`)
  (its *Keep a supervisor after launching the harness* names this reopening),
  `docs/adr/the-launched-child-is-a-job.md`,
  `docs/adr/harness-selection-is-owned-by-policy.md` (its *Have the executable
  launch a confined invocation too*), `docs/adr/one-live-driver-per-working-tree.md`.
  Read with them: `docs/adr/a-review-carries-its-creator-reference.md`.
- Specs: `docs/specs/harness-selection-and-execution.md` (the area contract;
  its package boundary, execution, records, Grove integration, seams and out
  of scope all move), `docs/specs/standalone-invocations.md`,
  `docs/specs/module-decomposition.md` decisions 7 (the runner) and 9 (the
  loop's `complete`), `docs/specs/item-status.md` (the session witness rides
  the runner's launch events), `docs/specs/walkthrough-books.md` (books follow
  a changed crate).
- Glossary terms in play: **Loop control channel**, **Session epoch**,
  **Driver lease**, **Session witness**, **Complete finish cycle**, **Kind
  routing**, **Selection parameter**, **Handoff attempt**, **Selected
  command**, **Owner settings**, **Meta-grove** (see `CONTEXT.md`, and
  `CONTEXT-MAP.md`'s grove → harness-dispatch table).

## On the horizon

- An after-run policy hook that reports a harness's usage from its own logs.

## Out of scope

- Detecting a harness that returned to its prompt without the exit signal:
  that stall stays as it is today.

## Notes

- `confined-run-k23` reviews the complete confinement node before
  `launch-cutover`. The node's work introduces a channel-free
  `keyed_launch::NoninteractiveLaunch` for Grove's dispatch supervisor; the
  runner's shared job core now distinguishes that launch from a harness launch
  that carries completion authority. Keep that distinction in the cutover.

- Terms resolved here, for design to pin to a surface and write into
  `CONTEXT.md` as it does: the **exit signal** (dispatch's; a session's
  statement that this harness run is over, carrying nothing), the run's
  **ending** (exit signal, the harness's own exit, or cancellation), and the
  **teardown record** (Grove's). The **Loop control channel** entry changes
  with them.
- Meta-grove: this loop runs the installed v22 binaries and skills, so nothing
  this grove builds reaches its own later sessions until rebuilt and
  installed. The owner's installed policy reads both `repo` and
  `session_name`, so it needs the D1 and D2 edits when the release lands.
- **Every session of this grove ends, and runs its tree verbs, with the
  installed v22 `grove-llm`** (`grove-llm complete`, as its v22 prompt says).
  Never use `./target/debug/grove-llm` once its surface changes. No leaf
  installs the built pair, whether through Homebrew or into a directory on
  PATH, before the finish. Any install changes the binaries these sessions
  depend on.
- **The cargo guard keeps clearing `GROVE_SIGNAL_FILE`** beside the new
  names for as long as this grove runs, because its sessions carry a live
  v22 channel under that name.
- **A leaf that changes a crate's source updates that crate's walkthrough book
  in the same commit**: `keyed-launch`, `grove-loop`, `grove-llm`, and
  `overview` for `crates/grove`. `scripts/check.sh` runs final validation of
  every book. Each leaf also logs its own change under `## Unreleased` in
  `CHANGELOG.md`.

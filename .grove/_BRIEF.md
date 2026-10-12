# grove.extract-harness-dispatch-to-top-level — brief

## Goal

Extract harness-dispatch into the independent **Ravel** project, with its own
packaging and deployment, while keeping Grove a working CLI consumer. At this
stage Ravel is a process manager and router.

## Current contract

The human narrowed the scope during ravel-generic-pair-k18. This contract replaces
older record-store, history-migration and prompt-injection requirements, including
those in completed planning/review notes.

- Exactly two public verbs: `ravel start-session` and `ravel end-session RESULT`.
- `start-session` routes through the owner's policy, launches the selected
  process, supervises it, terminates/reaps it when requested, and returns the
  opaque argument passed to `end-session`. The result belongs to the caller;
  Ravel attaches no continue/force-stop or task-tree meaning to it.
- `end-session` addresses its enclosing session through inherited process
  context. It takes no session identifier or explicit target path.
- The supplied prompt reaches policy/harness unchanged. The caller supplies
  instructions to call `end-session` and the meaning of the result.
- Ravel owns no persistent store, record/observation commands, historical lookup
  API, history snapshots/importers or catch-up work. Existing dispatch history
  stays with the old tool. Grove owns any provenance it still needs.
- Preserve routing choices, process/terminal/confinement behavior and the current
  pinned runtime. Reuse the existing configuration format and copy the owner's
  actual policy/settings to `~/.config/ravel`, with necessary import relocation.
- Testing is deterministic and economical: reusable Taskfile entry points,
  complete saved logs with concise summaries, focused checks during edits and
  one full relevant handoff check. Read failure details only when needed. Reuse
  real-worker/fake-agent and installed-layout seams; no model launches or new
  testing framework. Remove tests of discarded storage/prompt-injection features.

## Done when

- The old Ravel project and README edit remain at the named retired revision;
  its old implementation and local build/editor remnants are outside the active
  replacement tree.
- Ravel independently builds, checks and installs the 0.0.0 front/compiled worker,
  local runner, generic SDK/examples and required runtime notices. The active
  executable documentation describes the two-verb process boundary. Caller
  adapters, policy samples and task-tree conventions remain caller-owned.
- Ravel has independent release tasks, the existing three supported-target
  archives, a first 0.1.0 release and `linkuistics/taps/ravel`. No host Bun/Node
  or Grove installation is needed by its installed pair.
- Grove resolves the installed Ravel from PATH once, checks its supported public
  CLI version and uses only its documented arguments and returned result. Its
  formula depends on Ravel. No shared Rust library, bundled Ravel release or
  equal-version/shared-Cellar assumption crosses the project boundary.
- Grove supplies its own end-session instructions and interprets results to
  advance, finish or stop its sequence. Ravel implements no Grove loop decision.
- Routing, opaque-result round trips, nested/stale context isolation,
  cancellation/reaping, terminal cleanup, confinement and independent installed
  layouts pass. Source-pinned documentation changes with its source; remaining
  ownership prose is reconciled before release.

## Working handoffs

| Leaf | Handoff |
|---|---|
| ravel-cli-k3 (k14 → k15 → k16) | Retired old project, two-verb generic runnable pair, copied configuration and executable docs |
| ravel-delivery-k4 | Independent release recipes and all unreleased target/layout checks |
| ravel-publication-k11 | Published/installed independent Ravel release |
| grove-ravel-caller-k5 | Both callers use start-session/end-session and own their prompt/result/provenance conventions |
| grove-ravel-delivery-k6 | Grove archives/formula ship caller assets and depend on separate Ravel |
| grove-ravel-docs-k12 | Remaining prose/decisions describe the CLI boundary |
| grove-extraction-release-k13 | Extraction integrated/released while the original driver remains live |
| grove-session-control-k8 | Caller-owned continue/force-stop results and instructions |

The former Ravel-specific session-control leaf k7 is abandoned: Ravel injects
nothing and interprets no result token. k8 remains Grove work and requires k13,
not another Ravel feature/release. The agent-server remains a future direction;
do not add its protocol, daemon, dashboard or persistence now.

## Source and delivery

Reuse `/Users/antony/Development/Ravel` and Linkuistics/Ravel. Transfer the
applicable front/worker and local runner from `crates/harness-dispatch` and
`crates/keyed-launch`, removing persistence and caller semantics. Transfer the
actual build/install/check/release instruments, pinned Bun 1.4.2 dependencies,
hostile-probe controls and applicable executable-owned spec/ADRs/runtime evidence.
Reconcile those documents to the smaller contract; do not copy obsolete store,
record, inspect/init or prompt-injection requirements as active Ravel design.

Keep the actual personal policy choices, bounds and environment grants. k16
copies configuration only; it opens or migrates no personal history. Explicit
storage settings are obsolete and must not make the new tool access the old
store. Ravel's public names include `ravel/sdk`, `.ravel-choice`, and
`libexec/ravel/ravel-policy`; `--help` and `--version` are ordinary flags.

## Live-driver boundary

This grove is driven by installed Grove 22.1.0 and its original dispatch tool.
Every current session keeps its launch prompt's original ending binary/channel;
this session still ends with **`harness-dispatch exit`**. The new CLI contract
never rewrites a running session's mandate.

k6 prepares/tests and k13 applies preservation of the original complete keg,
with automatic cleanup disabled, and a non-overwriting compatibility link for
bare `harness-dispatch exit` on the inherited PATH. Preserve original grove-llm
admission/teardown too. Keep that installation through final finish's last signal.
Cargo and direct test helpers scrub original and new session authority controls.

Ravel leaves commit project changes in Ravel, record exact commits here, then
return here to retire/prune and seal their coordination changes. No second Grove
driver. Recheck checkout/config/tap state at each handoff. Resume partial releases
without repeating a version bump. k13 leaves the tree, lease and remaining work
live, with no teardown/finish commit; final integration/release belongs to the
existing driver-owned finish sequence.

## Pointers

- plan-k1 records original requirements; the current contract above incorporates
  the human's subsequent corrections.
- Ravel's `docs/session-cli.md` records the agreed process/router boundary.
- The source executable spec and ADRs describe the pre-extraction implementation;
  retain applicable process/runtime decisions and reconcile ownership in k3/k12.

## Testing runner

The human selected cargo nextest. Rust test tasks use nextest and preserve its
reporting UI; formatting/lint diagnostics may be saved with short summaries.
Use focused filters during edits and the full relevant suite at handoff. The
nextest runner replaces cargo test for runtime suites; retain doctest checks
when the source contains executable documentation examples.

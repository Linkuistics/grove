# ravel-extraction-k10

**Integrates:** ravel-extraction-k9

## Goal

Triage the findings of `ravel-extraction-k9` against the committed tree, the
source and the recorded evidence, and reshape the ordered Ravel extraction
leaves so implementation consumes a corrected plan.

## Context

- Read the findings from `ravel-extraction-k9`'s own commit, not from this body.
  Reproduce or derive each from the source before acting; classify it as a
  contract stated unclearly, a real issue, a visible trade-off or noise.
- The artifact is the ordered `ravel-cli-k3` through `grove-session-control-k8`
  leaves and the root brief. Edit leaf bodies and the root brief; do not
  implement any of them here.
- `plan-k1`'s running log and the root brief's human requirements stand. A
  finding that would reopen a settled requirement, or that needs a human
  choice (for example whether the work stays one grove or splits), is handed
  back with a recommendation and its evidence rather than decided here.
- Recheck cited source lines at integration time; the tree may have moved.

## Done when

- Each finding of the review carries a recorded classification and, where real,
  is resolved in the leaf or brief that owns it, or is handed to the human with
  a recommendation.
- Every leaf still lands a working, independently demonstrable handoff and fits
  one focused session after the changes; any split or reorder keeps the root
  brief's table, each leaf's `Requires`, and the cross-project handoff text
  consistent.
- Substantial redesign, if a finding demands it, becomes a new producer review
  chain beside the artifact rather than an unreviewed rewrite here.

## Notes

This leaf sits before `ravel-cli-k3` so that no implementation starts from the
uncorrected tree. It changes planning artifacts only: no source, installation
or release step, and nothing that touches the live session's installed
binaries, configuration or ending channel.

## Decisions (running log)

**F10 — real trade-off, human choice.** The human first selected two groves,
then steered this session to a release part way through this one. Keep k7/k8 in
this tree and insert a source-only Grove integration/release checkpoint after
extraction delivery and prose reconciliation, before session-control behavior.
The checkpoint leaves `.grove/`, the workspace and the current driver's lease
intact; it is not finish or teardown. Final finish still integrates/releases the
session-control increment. This supersedes the initial split recommendation.

**Live driver is not upgraded in memory.** `crates/grove/src/cli.rs:99-100`
resolves dispatch once and `loop_driver.rs:155-194` reuses that path. A mid-grove
Homebrew upgrade must preserve the old keg/front/worker and old bare ending
command for every remaining session, not just the checkpoint's final signal.
Installing new Grove does not turn the already running driver into a Ravel
caller. Plan an isolated transition check before the release; do not change this
session's live installation or channel.

**F1 — real issue.** `scripts/templates/grove.rb.tmpl:33` installs the old
front, Taskfile's release upgrades/unlinks it, and `exit.rs:23-46` needs that
command only to create its channel. k3 documents/test-plans a legacy exit-input
fallback; k6 owns an isolated old-install retention/PATH-link/next-launch test;
k13 and final finish preserve it through upgrade. The fallback alone would not
preserve the mandated command or the driver's cached path, so it is insufficient.

**F2 — real issue.** Both `LaunchDirectory::read` and standalone's publication
reader require schema 1/source harness-dispatch/observed exit_signal today;
`observation.rs:37,68,394-405` closes the envelope/ending vocabulary. k3 fixes
the renamed source to ravel, k5 migrates both predicates and freezes their public
continue fixture, and k7 keeps that predicate true only for continue. Force-stop
gets distinct `force_stop`, checked against the actual extraction-era Grove
readers. k8 consumes it and owns stop/teardown precedence.

**F3 — real issue, with a visible migration trade-off.** `store.rs:75,83,655-690`
fixes the default, application ID and supported store versions; each reap writes
an observation (`observation.rs:394-411`). k3 uses a separate Ravel store seeded
by a consistent one-way snapshot. Explicit legacy state selections must relocate
before Ravel writes; existing destinations cannot be clobbered. The legacy store
stays writable by its old front, and late observations remain available there.
There is no continuous replication promise; document snapshot/catch-up and test
late reap and old-front access. k7 cannot mutate that legacy store.

**F4 — real issue.** `scripts/check.sh` runs final book validation and release
preparation tests; overview's source-index/chapters pin dispatch.rs and
standalone.rs, and grove-loop's manifest includes every source file. The cited
release preparation assertions and release-notes dispatch:build prerequisite
are present. k5 now owns those synchronized changes; k6 owns source-pinned
updates for its manifest/package removals. k12 gets only broader prose, not a
repair phase needed to make earlier boundaries green.

**F5 — contract stated unclearly.** Public names were omitted from k3 and Grove's
flip was split across leaves. k3 now records the front/SDK/worker/config/choice/
state/environment/report table. k5 changes source prompts, methodology, Creator
guidance, assertion fixtures and both old/new scrub lists together, including
the cargo boundary. A named Creator fallback serves the original driver;
current new launches use RAVEL_RUN_ID. k8 removes generic duplication only after
checking k7's supported contract, not as the initial rename.

**F6 — real issue.** `dispatch::locate` is deliberately canonical-sibling-only;
its inbound graph trace reaches CLI lifecycle and standalone. k5 names PATH
resolution/canonicalization, trust documentation, a matching test seam and a
runtime supported-version refusal/remedy. k8 advances that runtime floor when
generic instructions move, rather than claiming a formula version floor. k11
records immutable per-target Ravel archive URLs/digests; k6's non-Homebrew/QEMU
layout checks fail on missing or mismatched inputs.

**F7 — real issue.** dispatch.sh cites the Bun evidence/probe rule, the executable
README points to the shared spec, and the four ADRs mix generic implementation
with caller clauses. k3 transfers the executable-owned current-state material
and follows citations; k4 keeps the evidence attached to release instruments.
k12 checks those destinations before reducing Grove's copy to its caller contract.

**F8 — real issue.** Source confirmed `From Grove` in cli.rs, Grove advice in
init.rs, adapter fields in choice/inspect/record, embedded build inputs and
worker registration, and Grove knowledge in worker/sdk/index.ts and
worker/sample/policy.ts. k3 now requires a whole-active-tree enumerate/classify
sweep with observed controls and a useful rewritten generic sample. Only opaque
legacy record fields and retired history are named exceptions. These examples
establish the gap; the future acceptance sweep must enumerate beyond them.

**F9 — real issue.** The actual release/build/smoke/QEMU/preparation recipes,
installed smoke and RELEASING guide support the size concern. k4 ends at verified
unreleased delivery and k11 owns public cut/install/recovery. k6 ends at green
artifact cutover and the tested live-install preservation instrument; k12 owns
the remaining prose. Earlier source-pinned edits remain with k5/k6 under F4.

**F11 — contract stated unclearly.** Read-only Ravel inspection still reports
uwlrknks/93f202ba with the README edit over main/2f452e51, and only the
structured-plan-migration tag. The tap contains ravel-lite.rb, not ravel.rb.
k3 now requires pushing the retired bookmark before replacement and retaining
ordinary descendant history. Development starts at 0.0.0 and k11 makes the
first minor release 0.1.0; inherited Grove 22.1.0 is not Ravel's lifecycle.

**Verification scope.** Tier 2 graph generation 2026-10-12T09:16:14Z had no
recorded coverage gaps on the first checked evidence paths except the excluded
`.cargo/config.toml`, which was read directly. Locator/reader/exit snippets and
the locator's two inbound calls were inspected without pagination remaining.
Non-code recipes, task bodies, pinned pages and literal references were read
directly. This is planning evidence: all new behavior/installation checks are
explicit future leaf obligations, not claims of implemented or tested Ravel.
The later coverage check reported only a partial parse at release-smoke-qemu.sh
line 185 among its additional evidence paths; that range was read directly.

**Planning validation.** The installed `grove-llm brief-chain` successfully
parsed the revised whole tree; resolve found k3, k7, k8 and all three inserted
leaves at their current paths, and kind confirmed k13 is impl. Re-read the
handoff table and leaf prerequisites after the inserts: preparation → publication
→ caller → artifact → prose → extraction release → Ravel control → Grove
control. The jj diff contains planning artifacts only. No product build or
release checks were run for these Markdown edits; each implementation boundary
names the checks it must actually run. Root closure is not due: implementation
leaves remain live, and no producer review needs a Creator update for this leaf.

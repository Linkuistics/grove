# stage-evidence-reconciliation-k125

## Goal

Reconcile the stage evidence in the acceptance record, and decide which stage
checks need another run.

## Context

- `docs/grove-dual-ravel-stage-record.md`.
- The Ravel design evidence: the "Local stage record", "Local-sources stage
  record", "Linux floor repair record", "Loader report wording record" and
  "Visible loader report record" sections, and the stage record of
  ravel-confinement-removal-k145.
- The stage record of grove-confinement-removal-k146.
- The triage section of `docs/common-loading-acceptance-record.md`.

## Done when

- The record states that the human withdrew confinement from both products.
  It marks each earlier confinement result as historical. It states the
  backend of each such result and which result came from a real harness. No
  reader can then credit such a result to the new builds.
- The record compares the native-only lifecycle with native standalone access
  on each route, on the builds after the removal.
- The record reconciles the absence of native mutation on fresh install and on
  upgrade, and the dual-major archive, formula and floor evidence.
- The record states, for each stage check, whether it runs again in
  `integrated-acceptance-checks-k126`, and why.

## Notes

A Linux real-harness call needs a Linux sign-in. If none exists, state the
Linux real-harness results as gaps, apart from the Linux fixture results.

The stage evidence names three Ravel 2.0 builds. The stage checks ran on
`23e50ce90037`. The later trials ran on `0fb12d5f308b` and `e9cc79993b46`,
which change only the preamble text. The removal adds a fourth build.

## Decisions (running log)

**Confinement is withdrawn from both products (2026-11-03).** While this leaf
read the stage evidence, the human proposed to abandon `--confine`. The
session offered three scopes: Grove only, Grove and Ravel, or the acceptance
contract only. The session recommended Grove only. The human chose Grove and
Ravel: Ravel removes confinement before the 2.0.0 cut. The evidence was the
measured Claude Code sign-in blocker and the Codex wrapper that turns its own
sandbox off.

**The removal comes before this reconciliation.** This leaf reconciles the
state after the removal, so the removal work goes before it.
confinement-withdrawal-k142 designs the removal, confinement-withdrawal-k143
reviews it and confinement-withdrawal-k144 integrates the review.
ravel-confinement-removal-k145 and grove-confinement-removal-k146 then remove
confinement from each product. The removal is a contract change of a major
release in two products, so the design has a review chain. This leaf stays
live, and its Done when names the state after the removal.

**The removal leaves ran only the gate on the caller tests (2026-11-04).**
grove-confinement-removal-checks-k154 ran `bash scripts/check.sh` with both
builds, one release-notes run and one aarch64 Linux `task release:smoke`. The
gate skips every `#[ignore]` test. So `ravel:directory`, the ignored half of
`ravel:commands` and the case tables of `ravel:policy` have not run on the
build `a4f55d04…`. Grove `main` and Ravel `main` did not move.

**A native read fixture is missing, and this leaf adds it (2026-11-04).** The
Test seams section of the delivery specification says that
`crates/grove/tests/standalone.rs` proves a native read on each runtime. After
grove-confinement-caller-removal-k150, that file holds only the case with no
native copy. No fixture then shows native standalone access on either route,
so the comparison of this leaf has no subject. The test is about 30 lines and
runs in the gate on both runtimes, so this leaf adds it rather than a new
leaf.

**The new test passes on both runtimes, and its control fails (2026-11-04).**
`an_agent_reads_native_copies_under_home_and_publishes` passed on the 1.1 keg
and on a scratch `task install` of `71f70fcd3f92`, whose `bin/ravel` SHA-256
was the recorded `26b487af…`. With one copy moved to another root, it failed.

**A timeout test fails on the 1.1 keg under load (2026-11-04).**
`a_timed_out_selection_launches_nothing_and_publishes_nothing` failed with
"select never ran" in each run of the whole file on the keg. It passed alone,
and it failed with the new test excluded and under the gate recipe. It passed
in both gates of k154. It is outside this leaf's goal, so the record hands it
to integrated-acceptance-checks-k126 as an observation.

**No real trial runs again (2026-11-04).** `preamble.rs` is the same at
`e9cc79993b46` and at `d324668b86ca`. The other changes of the skill code
remove grants. A trial measures an LLM, and the fixture runs of k126 measure
the delivery of the new build. The audit names the limit that no real harness
ran on `a4f55d04…`.

**The check decisions (2026-11-04).** k126 runs again: both gates, core
conformance, every Grove `ravel:*` task on each build, `ravel:directory`,
`release:smoke` on three targets, `release:formula:test`, `release:installed`,
`release:upgrade`, `legacy:rehearsal`, Ravel's `smoke:pair` and Ravel's
`release:smoke`. The target controls and the target smoke do not run again,
because their inputs did not change. The removed floor tasks, the release-notes
run and Ravel's amd64 campaign do not run again either. The Ravel checks of k149
run again only if the Ravel subject moved. The "Check decisions" section of the record gives each reason.

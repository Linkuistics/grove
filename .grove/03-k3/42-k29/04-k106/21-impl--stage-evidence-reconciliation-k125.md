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

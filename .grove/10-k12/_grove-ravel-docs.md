# grove-ravel-docs-k12 — brief

## Goal

Finish Grove's current-state ownership and installation prose after the runtime
and artifact cutovers, with the executable-owned documentation preserved in Ravel.

## Context

- Requires grove-ravel-delivery-k6's green archive/formula/check handoff; k5/k6
  already update every source-pinned page and gate-facing assertion they change.
- k3 moved the executable's spec, generic ADR content and Bun evidence into
  Ravel. Reconcile the remaining Grove documents against that committed copy.

## Done when

- README, usage, release and architecture guides, glossary, caller ADR/spec
  clauses, active walkthroughs/diagrams and plugin guidance describe independently
  installed Ravel and Grove's public CLI/parameter contract. Link to the durable
  Ravel spec/decisions/evidence; remove stale executable implementation claims
  from Grove only after verifying the destination exists. Grove keeps its
  caller contract, local runner decision and task/Creator/teardown conventions.
- Enumerate/classify old-name and ownership references over all affected prose,
  summary tables, diagrams and root/config surfaces. Retain named history and
  legacy-live-driver exceptions. Use observed positive/cross-tree controls,
  chase links and reconcile decisions in place; no duplicate extraction spec.
- Book/reference/conformance validation and `task check` pass. Record the
  independently tested source and release-ready handoff for k13. Ravel injects no instructions and owns no result meanings; Grove supplies
  end-session instructions and k8 extends its caller-owned force-stop handling.

## Notes

This is the bounded prose half split from k6, after its green artifact boundary.
No installation, publication or live-channel changes here.

## Decisions (running log)

- 2026-10-14: Decompose at the discovered documentation seam: the old executable
  specification and runtime evidence retain roughly 2,700 lines of pre-extraction
  claims, alongside nine stale visual views and independent architecture/glossary
  claims. Complete only the caller spec/views child in this session. Follow with
  ownership reconciliation and whole-surface acceptance before k13.
- Keep the existing spec/view paths as caller-owned entry points, with links to
  verified Ravel v0.1.0 documents. Preserve useful inbound anchors, explaining
  retired store vocabulary explicitly rather than keeping obsolete contracts.

## Working handoffs

1. grove-ravel-caller-views-k56: current caller spec, visual views and evidence
   locator, with destination/source/link checks and a green repository gate.
2. grove-ravel-ownership-k57: architecture, glossary/context map, remaining
   current guides, ADR/spec and walkthrough/plugin prose; chase all citations.
3. grove-ravel-prose-acceptance-k58: enumerate/classify the complete affected
   surface, observed controls, book/reference/conformance and full task check,
   then close k12 and promote the release-ready handoff for k13.

## Caller spec/views handoff — k56

The spec, nine views, manifest summaries, presentation README and runtime
evidence locator now describe the independent public CLI/caller boundary.
docs/grove-ravel-docs-evidence.md holds the bounded verified-destination,
anchor/reference, observed-control, parser, review and frozen-gate evidence.
All 18 task checks, 1,378 workspace cases, nine doctests and six source books
pass; all 46,943 measured subjects agree. The source artifact and this
decomposition land together in k56's sealed change.

k57/k58 remain live. This is no whole-repo prose acceptance or release-ready
k12 closure. Original driver/config/product installations remain preserved;
no runtime, installation, publication, new driver or live-channel change.

## Ownership handoff — k57

The architecture, glossary/context map, guide/caller-decision/spec and active
book/plugin/root prose now agree with independent Ravel and Grove-owned caller
conventions. Existing anchors and source-exact fragments remain. The current
Codex delivery summary is reconciled too. The two old all-Grove-code wording
pins now explicitly distinguish binaries from imported caller assets.

docs/grove-ravel-docs-evidence.md records the 46 pinned payloads, destination
and citation checks, observed controls, sole review and frozen final gate.
All 18 checks, 1,378 workspace cases, nine doctests and six books pass;
35,372 frozen inputs and 1,387 unique runtime identities agree individually.
Use the existing Ravel/target/tools Bun 1.4.2 first on PATH for these checks;
host Bun 1.4.3 is deliberately refused. Complete logs are in target/k57.

k58 remains live and owns whole-surface acceptance, named exceptions and k12
closure/release-ready promotion. No installation, publication or driver/channel
change occurred; the original 22.1.0 launch contract remains in force.

## Complete prose handoff — k58; k12 closed

k58 checked every k12 criterion against the delivered current documents and
reconciled the complete tracked prose/source/config surface, with each old-name
exception named. Four stale tooling passages, one inherited fixture comment and
one unpublished citation were repaired. The existing inventory is refreshed;
docs/grove-ravel-docs-evidence.md#complete-prose-acceptance holds the criterion
table, exact source boundary, destination/control/semantic evidence and k13 handoff.

All 18 checks, 1,378 workspace cases, nine doctests and six final/all books pass.
All 46,980 frozen inputs and 1,387 unique runtime identities agree individually
with the prior handoffs; doctests agree individually with k57. The 31 workspace
skips remain outside this default gate; k5/k6 retain ignored/all-target evidence.
Complete terminal logs and per-item records are target/k58. Verification uses
existing Ravel/target/tools Bun 1.4.2 first on child PATH. Reporting and normal
retirement follow the freeze. No prose acceptance gap remains in k12.

k13 consumes the sealed handle-named change and root promotion for integration/
release with rebuilt tagged acceptance. It applies a fresh original-keg plan and
keeps original ending/admission/teardown through final finish. k8 remains the
later caller-owned force-stop extension. No installation, publication, policy or
live-channel change occurred here; this session retains harness-dispatch exit.

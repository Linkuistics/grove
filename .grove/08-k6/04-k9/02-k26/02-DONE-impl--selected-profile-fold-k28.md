# selected-profile-fold-k28

## Goal
Complete explicit selected-profile resolution against the full k26 and k9
contracts, replacing the temporary known-profile selection refusal from k27.

## Context
Read the parent chain and the modular configuration spec. k27 validates profile
structure and captures original KDL plus definition spans; it does not apply
profile patches. The named resolver currently folds primary then overlay, checks
primary parameter-only routes immediately, and looks up origins by span alone.
All three assumptions must change for repeated profile applications.

## Done when
- Expand includes depth first in listed order before own patches; repeat every
  occurrence and diagnose unknown references and active-stack cycles with closed
  chains and real spans. Preserve external selections' absent spans.
- Fold all active personal patches before checking target authority, then local
  patches; preserve scope specificity, resets/unsets and final-reference checks.
- Occurrence IDs/parents/selection indices and ordered origin/history records
  distinguish repeated applications. Diagnose surviving active errors globally;
  inactive work stays harmless and local targets cannot repair personal absence.
- Complete all remaining k9 acceptance through Catalog, inspection/expansion,
  conformance, a non-Grove consumer, and captured-source independence.
- Keep Grove's declaration guard until k10. Retain k27's real-profile launch and
  refusal tests and add any remaining guard/authority coverage the parent needs.
- Update all affected docs/books, run scripts/check.sh, and audit/close k26,
  profile-composition-k9 and configuration-engine-k6 against their full briefs.

## Decisions (running log)
- Retain parsed profile declarations and expand an occurrence schedule before
  folding. Keep one resolver for base, profile and local patches; check personal
  target authority at the boundary before the local layer. Effective targets
  retain their occurrence chain, and origins use span plus occurrence identity.
  This preserves repeated applications without changing the public API.
- The graph generation 2026-09-16T11:54:39Z predates the named resolver and
  reports changed/untracked coverage for the affected configuration files;
  implementation evidence therefore comes from direct source reads.
- The single fresh-context review found missing reset provenance on a literal-to-
  binding transition. Classified actionable; a public regression first failed
  with Set(stale) rather than Reset, then passed after recording the target
  occurrence. No additional review findings. The fix is covered by that seam.
- Only the keyed-launch book owns the changed production corpus; all six book
  manifests were inspected. No corpus exceptions were added. Its fragments,
  root lengths, derived indices and composition prose now describe the fold.

## Verification and closure audit
- All eight principal checks in `bash scripts/check.sh` passed: formatting,
  shellcheck, Clippy, plugin installation, conformance, conformance regression,
  locked workspace tests and final validation of all six books. Keyed-launch
  reconstructs 4,087 lines without deferred source. SHA-256 digests for all
  1,777 tracked non-task-tree inputs (source, manifests, scripts, fixtures,
  plugins and documentation) matched before and after the complete run.
- `profiles.rs` exercises ordered split routes/bindings/values, opposite lead
  bindings, add/remove embedded-value experiments, specificity, shared and route
  removals, surviving versus overwritten failures, no-route validation, repeated
  includes/selections, diamonds, closed cycles, real include/selection spans,
  occurrence-specific histories/winners and both literal-transition resets.
- The personal-authority fixture rejects a nested parameter-only occurrence
  before local target repair, permits later personal targets and local parameter
  completion, and keeps inactive personal targets non-admitted. Required-value
  diagnostics name the selected route occurrence. Errors return no snapshot.
- Catalog/diagnostics/inspection/named-command suites retain whole-document
  structural checks, eager legacy checks, exact/native argv safety, declaration
  absence versus explicit empty, convenience-loader equivalence and deterministic
  reports. The non-Grove profile fixture uses opaque keys and its own payload
  vocabulary, checks conformance failure and empty-set refusal, then compares
  inspection with expansion after source removal and Catalog disposal.
- Existing Grove acceptance with real alternate profiles passes for personal and
  local selections, including empty declarations, before child spawn, root
  creation and leaf-add. Inactive profiles still allow the base launch. Source
  discovery and trackedness regressions pass. Grove's guard remains for k10.
- These observations discharge k28, k26, k9 and k6's Done when. The reviewed ADR
  set already states the implemented occurrence, specificity and authority
  decisions; no new architectural decision or contract change is needed. Current
  generic-engine behavior is promoted to the root brief for the remaining leaves.

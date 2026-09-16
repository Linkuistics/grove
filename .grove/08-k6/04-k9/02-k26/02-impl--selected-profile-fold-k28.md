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

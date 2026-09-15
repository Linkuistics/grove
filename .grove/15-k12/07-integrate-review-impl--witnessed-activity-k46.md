# witnessed-activity-k46

**Integrates:** witnessed-activity-k45


## Goal

Triage the findings of the complete witnessed-activity protocol review and
apply the real ones, so witnessed-activity-k12 can close on a green principal
gate.



## Context

Read the findings from witnessed-activity-k45's own commit and task file; this
body carries only the handle. The review found no correctness or security
defect in the protocol; what it recorded is low-severity and concerns test
isolation in `crates/grove-loop/src/driver_lease/observation.rs`, one
reader-side grammar check in the same file against the writer in
`driver_lease/witnesses.rs`, and one clause of `docs/specs/item-status.md`.
The parent brief and `docs/specs/item-status.md` remain the contract; k37's
frozen native evidence applies to the shipped protocol source unchanged.

## Done when

- Each finding is verified against the source and classified as a contract
  stated unclearly, a real issue, a visible trade-off or noise, with the
  classification and reason recorded in this task's running log.
- Every finding classified real is applied within the review's stated repair
  shape; anything larger than a repair is cut as a new producer review chain
  beside this leaf rather than done here.
- Affected fixtures, source-derived walkthrough fragments, indexes and
  manifests are reconciled in the same change where a repaired file is a book
  root; `bash scripts/check.sh` passes after all edits.
- The k12 brief's Done when is re-checked against the delivered subtree and
  the node is closed or the missing work is named as a leaf.

## Notes

Do not add an in-session reviewer beside this scheduled integration. No
production protocol change is expected from these findings; if a real one is
needed, stop and say so rather than redesigning under this leaf.

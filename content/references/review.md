<!-- file: order=14 -->
<!-- unit: task-review-kinds kinds="review-requirements review-design review-planning review-prototype review-impl" class=triggering -->
**review-\*** (all AFK) — an inspection-only, fresh-context adversarial read of
*one* artifact. Inspect the producer's committed changes, source, requirements
or specifications, and recorded verification evidence. A review does not run
test, build, lint, or format commands, edit production or test code, or redo the
implementation. Its output is findings only; the paired `integrate-review-*`
task owns every fix and all post-fix verification. Five reads look for different
things: `review-requirements` (is anything missing? is each requirement
falsifiable? is a solution smuggled in as a requirement?), `review-design` (does
it satisfy the requirements? are the ADRs a minimum coherent set? are the seams
at the right height and count?), `review-planning` (are the slices vertical?
does each land green without waiting on a sibling? is anything missing?),
`review-prototype` (does it probe the question it was built for? — *not* a code
review; polish is a defect in a prototype), `review-impl` (correctness,
security, tests, project conventions).


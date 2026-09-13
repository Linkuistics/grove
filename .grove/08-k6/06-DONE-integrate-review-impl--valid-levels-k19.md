# valid-levels-k19

**Integrates:** valid-levels-k18

## Goal

Triage the findings of the `review-impl` leaf `valid-levels-k18` against the
enforcement boundary delivered at `valid-levels-k16`, apply the ones that are
real, and leave the library's stated contract, its CLI contract and both books
current before `node-files-k7` builds Grove's required node-file grammar on
them.

## Context

- The findings are in the review's own file, found by its handle; read them
  from there rather than from this body. They are anchored to commit
  `yrquyqyp` (`bbd60935`) and to `path:line` coordinates in that tree. This
  leaf was appended after the review, so no sibling position moved.
- The producer's contract is `04-DONE-impl--valid-levels-k16.md` and the parent
  brief in this directory; the previous integration at `supplied-names-k17`
  settled the refusal mapping and the empty-sample limit, and nothing here
  reopens them.
- The artifact is the trait documentation, the syllabus CLI's exit-code
  contract, the conformance kit's coverage statement, and the `ordinal-fs-tree`
  and `grove-loop` books under `docs/walkthroughs/`. The review found no defect
  in the reader, planner or projection; enforcement code is expected to stay as
  it is unless a triaged finding says otherwise.

## Done when

- Every finding is triaged as applied, rejected with the reason, or a visible
  accepted trade-off, in this file's running log.
- Where a finding names a kit-coverage or diagnostic decision the grammar
  consumer inherits, the decision is settled and recorded here so
  `node-files-k7` builds on it rather than reopening it.
- Each touched source root lands with its fragments, ledgers, indexes and prose
  in this commit; the book-validation inventory follows any line-count change;
  `bash scripts/check.sh` passes on unchanged tracked inputs.
- `node-files-k7` still follows this node; positions and keys are otherwise
  untouched. The installed driver, plugin and live-tree grammar remain
  untouched.

## Notes

- Substantial redesign is not this leaf's: externalise it as a new producer
  review chain beside the leaf being integrated.
- One narrow in-session reviewer is available for a non-mechanical fix; a
  second need is the signal to cut a `review-impl` leaf instead.
- If every real finding is documentation, the parent's *Done when* is met on
  this leaf's retirement and the node closes in the same commit; check the
  brief's criteria against what the three children delivered before closing.

## Decisions (running log)

1. Read the findings from review commit `kyluqkrk` (`9cfd4c77`). Graph startup
   refused because an incompatible active generation owns coordination; use
   targeted source reads, with no graph coverage claim or session disruption.
2. F1–F4 are real documentation issues. The trait invokes the rule, syllabus
   maps both new errors to exit 5, the book manifest supplies the current ranges,
   and the kit has five sampled name laws plus one sampled level rule and two
   shape-constrained name laws. Correct each statement and its source fragments.
3. F5 is a contract stated unclearly; retain context-only coverage detection and
   make that limit explicit in the diagnostic, architecture and book. Fixture
   authors supply empty, singleton and competing shapes and meaningful expected
   verdicts. A permissive domain need not have a refusing verdict, and a domain
   with one distinguished spelling cannot supply two distinct valid names.
   Duplicate-value samples test a method input, not a realizable directory;
   distinct-name filesystem coverage belongs to the Required domain fixtures.
   `node-files-k7` must supply independent required-name acceptance/refusal
   fixtures rather than interpreting a clean kit report as complete coverage.
4. F6 is an accepted diagnostic trade-off: planning errors identify the final
   containing directory whose projected policy failed, even when that path does
   not exist on disk. The shifted-ordinal fixture pins this meaning. Keep the
   error representation and rendering; expose the meaning in the architecture.
   Repeated initialization preflight is harmless under the deterministic-policy
   contract and preserves the shared apply boundary. No redesign is needed.
5. F1–F4 are applied, including three further stale task-name totals in the
   lease and loop chapters and the summary/index descriptions of kit coverage.
   F5 is clarified in the public kit documentation, diagnostic, architecture
   and both books; F6 remains the visible trade-off recorded above. Source line
   counts are unchanged, so fragment ranges, manifests and validator inventories
   remain correct without adjustment. No in-session reviewer was needed for
   these wording corrections and accepted limits.

## Verification

- `bash scripts/check.sh` exits 0: all eight principal checks pass, including
  locked workspace tests and final byte reconstruction of all six books.
- SHA-256 inventories of all 1,752 non-ignored repository files, including
  hidden tasks, source, fixtures, manifests, scripts and books, are identical
  before and after that run. This record and retirement bookkeeping follow it.
- Chapter 2/3/4 block ranges give 451/563/729 owned lines; their total is 1,743.
  Chapter 21's part totals plus chapter 1 sum to 10,593. The stale-total search
  is clean across the current grove-loop book and finds the old chapter and
  historical review as controls.
- Parent close: `supplied-names-k14` delivered explicit names, canonical
  identity and compiling consumers; `supplied-names-k17` pinned supplied-name
  confinement and diagnostic boundaries. `valid-levels-k16` delivered complete
  reads, pre-effect final-level validation, independent level samples and
  required-name/node-parts fixtures. `valid-levels-k18` reviewed those boundaries;
  this integration settles every finding and verifies the synchronized books.
  Together these meet every Done when in `distinguished-names-k6`.
- Promote the sampling and diagnostic decisions to the root brief for
  `node-files-k7`. Existing ADRs already place these responsibilities on the
  name seam; the library architecture now states their precise limits, so no
  new ADR or redesign is warranted. Other root children remain live.

# book-assembly-k35

**Integrates:** book-assembly-k34

## Goal

Verify and fix every finding the technical review recorded, rerun final book
validation together with the complete crate and model checks, correct the
book's recorded verification evidence to match what actually ran, and commission
the fresh-context editorial review as this session's final act.

## Context

- The findings: the `## Findings` section of
  `.grove/10-ordinal-fs-tree-book-k10/10-DONE-review-impl-book-assembly-k34.md`.
  Each names an exact book path and line, the source, specification clause or
  test it contradicts, and the technically correct replacement.
- Book-system contract: `docs/specs/ordinal-fs-tree-book.md` — normative for the
  worked-example table, the source-fragment-introduction rule, and the
  refinement rule that keeps top-level ownership blocks stable.
- Authoritative production corpus: the fifteen files in the node brief and in
  `docs/ordinal-fs-tree/book/source-index.md`. The corpus is frozen; a defect
  found in crate source is externalised as a leaf, not fixed here.
- The review confirmed the mechanical layer clean before it found anything:
  byte-exact expansion of all fifteen roots, gapless single-parent coverage, no
  `defer`, and a fragment index and ownership ledger that agree with the
  directives. Findings 3 and 4 change fragment boundaries and index rows, so
  that state has to be re-established rather than assumed.

## Done when

- Every one of the six findings is independently verified against the source,
  specification or test it cites, then fixed or — with the reason written into
  the running log — argued down. Verify before fixing: a finding accepted
  without checking is the same defect the review exists to catch.
- Finding 1 is fixed by *running* the corrected crate command, not by editing
  the recorded one. `08-invariants-and-trade-offs.md` records the command that
  was actually run and its actual result, and the doc tests pass.
- Finding 2 adds an anchored `worked-reference-name` section to
  `03-reference-domain.md` that carries one syllabus entry through parts,
  formatting, parsing, species, and its conformance obligation, and the concept
  index gains the entry.
- Findings 3 and 4 leave every top-level ownership block's ID, owner, source
  path, range and position in its root unchanged; only descendant fragments and
  their `Fragment index` rows move. Owned-source totals still reproduce the
  specification's per-slice table and its 6,929 total.
- Findings 5 and 6 are corrected in place, and the correction to finding 6 is
  checked against every page that shows a tree, not only against page 03.
- Final book validation, the complete crate verification including its doc
  tests, and the Alloy and Quint runners all pass, and their results are what
  `08-invariants-and-trade-offs.md` states.
- No production source under `crates/ordinal-fs-tree/` is modified.
- The editorial `review-impl` leaf is appended inside this node as the final
  act, with bare stem `book-assembly` and `**Reviews:** book-assembly-k35`. Its
  body scopes the read to the whole book rather than to this session's diff, and
  asks for clarity, concept order, cognitive load, self-containedness,
  repetition and cross-reference choices, and the declarative prose contract,
  without editing files or running checks. It creates an integration leaf only
  for real findings.

## Notes

Findings 3 and 4 are adjacent but distinct: 3 is missing introductions on page
06, 4 is misplaced boundaries on page 04. Fixing 4 changes which bytes the two
neighbouring fragments hold, so their introductions have to be re-read against
their new contents.

Finding 1 may turn up a genuine failure once the doc tests run for the first
time in this workstream's book verification. A doc test that fails is a defect
in frozen crate source: externalise it as a leaf beside this one and say so in
`08-invariants-and-trade-offs.md` rather than editing the crate here.

The review spent no in-session reviewer. This session may spend one narrow
reviewer if a fix turns out to need judgement the compiler cannot settle;
substantial rework of a page belongs in a new producer chain beside this leaf
rather than inside it.

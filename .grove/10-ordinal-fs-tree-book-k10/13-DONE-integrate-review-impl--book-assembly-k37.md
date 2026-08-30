# book-assembly-k37

**Integrates:** book-assembly-k36

## Goal

Verify and fix every finding the editorial review recorded, then rerun the
complete book, crate, Quint, and Alloy checks and record their actual results.

## Context

- The findings: the `## Findings` section of
  `.grove/10-ordinal-fs-tree-book-k10/12-DONE-review-impl-book-assembly-k36.md`.
  Twelve findings, each naming an exact book path and line, the contract clause
  or cross-page statement it violates, and the correction.
- Prose contract: `docs/specs/ordinal-fs-tree-book.md`, sections
  *Self-containedness*, *Local context and repetition*, *Source-fragment
  introductions*, *Worked examples*, and *Early-use ledger*. These are the
  normative clauses the findings cite; the review invented no criteria.
- The two prior steps: `10-DONE-review-impl-book-assembly-k34.md` and
  `11-DONE-integrate-review-impl-book-assembly-k35.md`. Findings 1 and 2 are the
  editorial residue of that review's findings 6 and 3 — the technical fixes were
  correct and are not to be reverted.
- Authoritative production corpus: the fifteen files in the node brief and in
  `docs/ordinal-fs-tree/book/source-index.md`. The corpus is frozen. No file
  under `crates/ordinal-fs-tree/` is modified by this session; a defect found in
  crate source is externalised as a leaf beside this one.

## Done when

- Every one of the twelve findings is independently verified against the book
  text and the specification clause it cites, then fixed or — with the reason
  written into the running log — argued down. Verify before fixing: a finding
  accepted without checking is the same defect the review exists to catch.
- Finding 1's three corrections are applied and then checked against *every*
  page that prints a tree, not only the three named, so no page is left
  asserting a continuity the display breaks. If the root spelling is unified,
  all four dependent strings on `04-read-path.md` change together.
- Finding 2 raises all seven named page-06 introductions to answer the five
  questions in *Source-fragment introductions*. Before finishing it, re-run the
  same enumeration over all 68 literal fragments on all eight pages rather than
  over the review's list of seven: the review reached its list by enumerating
  the whole surface, and the fix is complete only if the same sweep comes back
  clean. This is the exact narrowing that left finding 2 behind.
- Finding 4's correction reduces the duplicated enforcement split on one page
  only, leaves both seven-obligation lists intact, and either adds the
  `conformance` early-use row or removes the behavioural claim that would
  require it. If a row is added, `source-index.md#early-uses` keeps its
  documented sort order and its `Status` is correct for a resolved owner.
- Findings 3, 5, and 11 change headings, anchors, and index entries. Every
  moved or added anchor keeps a matching `concept-index.md` entry, no anchor ID
  collides, no heading level is skipped, and no local link is left dangling.
- Finding 12 is either applied or closed as an accepted trade-off with the
  reason in the running log. Closing it is a legitimate outcome; leaving it
  unaddressed is not.
- The final book validator in `--final` mode, the complete crate verification
  including its doc tests, and both the Alloy and Quint runners are run, and
  `08-invariants-and-trade-offs.md#final-verification` states what actually
  happened — including any harness caveat, at whatever length finding 12 is
  settled at.
- No production source under `crates/ordinal-fs-tree/` is modified.

## Notes

Every finding is a prose, heading, anchor, or index change. None requires a
fragment boundary to move, so the fragment graph, the ownership ledger, the
per-slice line totals, and the 6,929 total should be unchanged at the end; if a
fix appears to require moving a boundary, that is a signal to re-read the
finding rather than to move it.

Findings 6 and 8 are single-sentence rewrites whose correct wording already
exists elsewhere in the book — `08-invariants-and-trade-offs.md:98-103` for the
highest-first rationale, and the four enumerated sentences at `08:174-179` for
the omission list. Prefer aligning to the existing precise statement over
inventing a third phrasing.

Finding 9 is a readability rewrite of six introductions that already satisfy the
contract. It is the lowest-value item here and the easiest to get wrong: the
five answers must survive the split. If it cannot be done without losing one,
leave the introductions as they are and say so in the running log.

This session may spend one narrow reviewer if a fix needs judgement the text
cannot settle. Substantial rework of a page belongs in a new producer chain
beside this leaf rather than inside it.

If the checks all pass and every finding is settled, this node's `Done when` is
met and no further review step is created: the editorial review is the last one
the node brief schedules.

## Decisions (running log)

All twelve findings survived independent comparison with the cited prose
contract and book passages. They were repaired without changing any fragment
directive, ownership range, production source, or source total:

1. The read-path tree is now identified as a deliberate variant of the
   orientation tree; page 04 uses `s` consistently in authored prose, and page
   05 marks the return to the orientation setup. Exact quoted source examples
   that use `syllabus` remain byte-for-byte unchanged.
2. The seven named page-06 introductions now state actor, input/output,
   invariant, and example role as well as placement. The complete fragment-index
   enumeration contains 79 current literal rows, not the review's stated 68;
   all 79 were swept, including the eleven CLI-page rows omitted by that count.
3. The worked application heading now distinguishes successful application and
   successful reverse unwind from the following failed-unwind section, and its
   lead states the plan, starting level, two traces, and outcomes.
4. Page 02 retains the seven obligations but reduces the enforcement split to a
   minimum contract and link. Its behavioral use of `conformance` now has an
   `explained` early-use row in first-use order.
5. The terminal-failure contract tests have their own anchor, H2, and concept
   index entry before the omissions section.
6. Orientation now states the observable highest-first consequence already
   stated precisely on page 08: interruption leaves a gap rather than duplicate
   ordinals.
7. The model-boundary list no longer carries a stale count, and the string
   boundary appears inside the list before its Rust-evidence sentence.
8. Orientation scopes `-i` to the syllabus grammar, while page 02 introduces
   the second consumer's `-k` grammar at its first example.
9. All seven cited page-07 introductions were split into ownership plus
   transformation/invariant/example sentences without dropping a contract
   answer.
10. Page 05 introduces `Level::Created` at the destination vocabulary and
    defines `Landing` through the complete effect order it preserves.
11. The concept index is again in reading-path and anchor order, including the
    new terminal-failure entry.
12. The trade-off was settled in favor of the durable reader-facing record:
    authoring-run workaround detail was removed, while the Grove history keeps
    its reproduction detail. The final-verification section now states the
    commands and the results observed in this run without duplicated totals.

No in-session reviewer was materialised. The changes are direct applications of
the editorial review and the specification, and no finding required the
substantial redesign that would create another producer/review chain.

The final book validator completed successfully with fifteen source roots,
6,929 resolved source lines, and zero deferred ranges. `cargo test -p
ordinal-fs-tree` passed every library, binary, integration, CLI-contract, and
documentation test, including all five doc tests and the non-UTF-8 filesystem
test, without a harness workaround. Alloy passed all seven no-counterexample
checks and found all thirteen witnesses without a solver workaround. Quint
reported every configured invariant holding and every configured witness
reached across all scenarios.

# concept-index-rollups-k217

## Goal

Bring every book's `concept-index.md` under the roll-up check
`ledger-rollup-check-k207` built, or record why it cannot be — the one surface
left stating ledger-derived figures with nothing re-deriving them.

## Context

- **The check exists and this surface is outside it.** `k207` added the
  `<!-- rollup «quantity» -->` directive and `F011`
  (`docs/specs/walkthrough-books.md`, *Ledger roll-ups in prose*): one or more
  directives on their own lines mark the paragraph below, and each declared
  quantity's derived figure must occur in it in digits. Twelve quantities, four
  of them taking an `of="…"` argument.
- **A concept-index row is a list item, not a paragraph.** The directive's scope
  is the contiguous nonblank block after the run, and an HTML comment between two
  list items ends the list in rendered Markdown. So the mechanism as built cannot
  mark these without changing how the page renders.
- **What is uncovered, exactly.** Rows such as `grove-llm`'s *Twenty-five
  ownership blocks and fourteen early-use rows, closed* and `keyed-launch`'s
  *Twenty blocks over nine roots, and the three splits that explain the count* and
  *Ten early-use rows, nine required by the manifest and one added*. Enumerate the
  whole class rather than working from that sample — the same condition may sit in
  a `README.md` contents line or a chapter's own prose.
- **Every one of them was true at `k207`.** This is a structural gap rather than a
  standing defect: a second copy of a figure the assembly chapter now states under
  a mark, free to go stale on its own.

## Done when

- The class is enumerated across every book, not sampled, and each instance is
  covered or explicitly excluded with its reason.
- Either the directive gains a scope that reaches a list item without changing
  what the page renders as, or the rows are restated so a marked paragraph holds
  them, or the specification records that this surface is deliberately unheld and
  says what stands in its place.
- Whatever is chosen has been **watched to fail**: mutate a ledger and see the
  concept-index claim go red, then restore.
- `bash scripts/check.sh` is green.

## Notes

**Do not widen the check's subject.** `k207`'s scope decision holds: only
ledger-derived figures. A concept-index row that summarises an argument is not a
roll-up and is not this leaf's business.

**Prose and tooling only, so the frozen-corpus rule does not bite.**

## Decisions (running log)

1. **The class, enumerated across all six books, not sampled.** Method: derive
   every one of the twelve declared quantities from each book's
   `walkthrough.toml` and its *Early uses* ledger table, then scan every page of
   every book (excluding `source-index.md`, which is the authority) for each
   derived value — in digits *and* spelled in words — standing within three
   words of that quantity's own subject noun. Instances were then partitioned by
   the construct they sit in, because that is what decides whether the mechanism
   as `k207` built it can reach them at all.

   The scan's positive control is the fifteen ledger-account paragraphs — the
   `**Ownership.**`, `**Early use.**` and `**Owned source.**` paragraphs of the
   five books that have a closed-ledgers section — whose figures `F011` already
   proves correct: the scan reports a hit inside every one of them, and inside
   `ordinal-fs-tree`'s single marked paragraph as well. Its cross-tree control
   is that it does *not* fire everywhere: `ordinal-fs-tree` reports no derived
   figure outside that paragraph, once the `one of them` / `two of them` idiom
   that the values 1 and 2 make unavoidable is set aside. It was watched to fail before either was
   credited: shifting every derived value by one drops the hits at those
   paragraphs from 27 to 8, and each of the eight survivors is a collision where
   one quantity's shifted value equals another quantity's true value — so the
   scan is keyed to the figure it derives and not to the noun beside it.

2. **The goal's premise is false: `concept-index.md` is not "the one surface
   left".** Ledger-derived figures also stand in **chapter headings**
   (`grove-llm/01-orientation.md:653` *seven chapters*;
   `overview/01-orientation.md:395` *five chapters*;
   `grove-loop/21-what-could-not-move.md:38` *Twenty chapters*), in a **table
   cell** (`jj-workspace/07-what-jj-owns.md:25` *752 lines*), and in dozens of
   **ordinary prose paragraphs** across every `README.md` and most chapters. The
   third group is not a gap: the mechanism already reaches a paragraph, and the
   specification records that marking there is the author's. The first two are
   the same structural gap as the list item — a construct no directive run can
   precede without changing what the page renders as.

3. **Route taken: the directive gains a trailing form** (the first of the three
   the brief offers). One or more `rollup` directives at the **end of a line that
   carries content of its own** mark that line, and the checked text is the line
   with the run cut away. An HTML comment at the end of a line is inline content
   in CommonMark, so nothing about the rendered page changes — which is the
   property a run *above* a list item cannot have.

   The rejected alternative was cheaper and worse: the whole concept index is one
   contiguous nonblank block, so a directive run placed above the list would mark
   it under the existing rule with no code at all. It would also be satisfied by
   the figure standing anywhere in several hundred rows. The mechanism's value is
   that a mark is narrow, and that route spends it.

4. **The rows had to be restated as well, and the two halves are independent.**
   The concept index spells its figures as words — *Twenty-five ownership blocks
   and fourteen early-use rows* — while the validator renders the ledgers' own
   digit form. So scope alone covers nothing. The three affected rows are now in
   digits, which is the register the assembly chapters already use for a
   ledger-derived figure, and each carries its run.

5. **A one-line mark is tighter than the paragraph it points at.** Watching the
   mutation fail showed this: removing one `[[early-use]]` row from
   `keyed-launch`'s manifest turns the concept-index row red and leaves the
   assembly chapter's `**Early use.**` paragraph green, because that paragraph
   runs on to *chapter 8's to explain* and the bare `8` satisfies the derived 8.
   That is the third limit this specification already records — a digit in a
   marked paragraph is not thereby checked — and it is an argument for the
   trailing form rather than against it.

6. **The rendering property is reasoned, not machine-checked here.** No Markdown
   renderer is available in this working tree, and three attempts to fetch the
   text of CommonMark's inline *Raw HTML* section came back truncated. What the
   fetched material does settle is the load-bearing half: an HTML block opens
   only on a line that *begins* with `<!--` (start condition 2), so a comment
   after a list marker or a heading's own text cannot become one and stays inline
   — which is why the list is not broken. The citation sits in
   `parse_trailing_rollups`'s doc comment. What would falsify it is a renderer
   that treats a trailing `<!-- … -->` as literal text; the six books' pages are
   the place that would show.

7. **The trailing form owed `P001` a widening.** `reserved_prefix` only ever
   looked at the *start* of a line, so before this change a mistyped trailing
   directive was prose: the figure it was meant to hold went unchecked and
   nothing said so — the exact silence `P001` exists to prevent. A line holding
   `<!-- rollup` in neither shape is now the same parse error as one beginning
   with it, and no part of the reserved prefix may survive inside the marked
   text, so a malformed directive standing *before* a well-formed one cannot be
   swallowed as content. Both were watched to fail: with the guard relaxed to the
   weaker `starts_with`, that case goes green while everything else stays as it
   was.

8. **`scripts/check.sh` is green, on the second full run.** The first went red on
   one test and one only — `grove-loop`'s
   `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`, which
   the fixture reported as a wedged producer after its 120-second wait for
   `second-ready`. That is not this leaf's change: nothing here touches
   `grove-loop`, the test passes alone in 3.25 seconds, and the failing run
   shared the machine with another repository's suite at a load average above
   four. It is also not `flaky-surface-snapshot-lock-test-k219`'s subject, which
   is a different test in `task_grow`. Recorded here rather than leafed, because
   one timeout under measured contention is an observation and not yet evidence
   — but if that test wedges again, this is its second sighting and the prior art
   is the orphaned-fixture hang the sibling leaf's *Notes* already name.

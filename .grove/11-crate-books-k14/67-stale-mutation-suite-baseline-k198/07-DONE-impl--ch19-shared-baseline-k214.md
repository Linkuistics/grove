# ch19-shared-baseline-k214

## Goal

Make `docs/walkthroughs/grove-loop/19-the-core.md` agree with the seven chapters
it speaks for. It is the only page in the book that states the mutation
baseline *on behalf of* other pages — the six before it now state their own — so
its one citation, its ten-versus-eleven adjudication and its account of why the
eleven fail all have to close over what chapters 11 to 17 now say. Then decide,
once, whether this class of shared roll-up should be mechanically checked.

## Context

- **One citation, and it is the last `558` in the book.** Line 123, in *The file
  the whole book's baseline is made of*: *seven of them — chapters 11 to 17 —
  cite the same environmental baseline: 558 tests, 547 passed, eleven failed
  before any mutation, in `crates/grove-loop/tests/prompt.rs`.* The re-derived
  control, reproduced identically at all six children of this node, is **560
  tests, 549 passed, 11 failed**. Both the total and the passing count move; the
  eleven does not.
- **The sentence is a cross-chapter claim as well as a number, and it is the
  interesting half** (`assembly-chapters-are-cross-chapter-claims` in miniature:
  this page owns a root, but *this sentence* owns none). It asserts that seven
  named chapters cite one baseline. Chapters 11–17 have now each been
  re-measured and re-worded by a separate leaf — `k208` through `k213` — and the
  clause has to be checked rather than assumed: read each of the seven and
  confirm both that it cites this baseline and that it cites *this* one. The
  wordings deliberately differ; the numbers must not.
- **The ten-versus-eleven adjudication is this page's, and the node brief has
  already ruled on the part that is not.** The page argues that copying
  `.claude-plugin/` gives a clean baseline of ten of one cause, and that a flat
  eleven can hide an observer of anything touching `PLUGIN`. That argument is
  sound and stays. What it must not do is re-describe the seven chapters'
  controls as tens: all seven were re-derived against the **eleven**, and the
  eleventh's separate cause is stated on this very page — so the recommendation
  stands as a recommendation about how a *future* study should copy, not as a
  restatement of what these seven measured.
- **Three enumerations on the page, none derived from a run.** *Ten of this
  file's sixteen tests reach `compose`: nine through it or `compose_with`
  directly, and one through `signalling_contract()`.* *The evidence is sixteen
  tests, and the brief pins four of them.* And *Chapter 10, whose copy was scoped
  differently — 626 tests over three crates — reports ten rather than eleven for
  the same reason.* Count each against
  `crates/grove-loop/tests/prompt.rs` and against chapter 10's own page
  (`uniqueness-and-count-claims-need-enumeration`); the third is a claim about
  another chapter's measurement, so check what chapter 10 actually says as well
  as whether it is right.
- **One forward reference now lands that did not.** The page says a control
  wrong in that direction *hides an observer, which is the same failure mode the
  `cargo build -p grove --bins` step exists to prevent at seventeen.* Chapter 17
  did not state that step until `k213`; it now does, in its own procedure
  paragraph. Confirm the reference resolves to something chapter 17 says, rather
  than to something a reader has to reconstruct.
- **`k213` found one thing the seven share that this page may want to carry.**
  Four of eleven mutant runs in chapter 17's study newly failed a `task_grow`
  test that cannot execute a mutated line at all — a cross-test flake reading
  exactly like a newly attributed observer, settled by running the candidate
  alone under the mutant rather than by a second full run. Chapter 17 owns the
  account. Decide whether the shared-baseline section is the right place to say
  that the baseline is a *set*, not a count, for this reason too — or whether
  saying it twice is worse than saying it once.
- **The mechanical-check decision, with `ledger-rollup-check-k207` named.** That
  leaf owns the condition where an assembly chapter states a ledger roll-up the
  ledger contradicts, and it is explicitly scoped to **ledger-derived** roll-ups.
  This baseline is not one: it is derived from a run in a scratch copy, and
  nothing in the repository holds it. So the question here is a different one and
  should be answered rather than deferred — is there a check that would have
  caught seven pages drifting to `558`, and if the honest answer is that a number
  no artifact holds cannot be mechanically checked, say so and say what the
  cheap human-side substitute is. Decide it once, on this page, and name `k207`
  as the neighbouring case that *is* checkable, so a later reader is not left
  wondering why one got a check and the other did not.

## Done when

- Line 123 states the re-derived total and passing count, and its *seven of them
  — chapters 11 to 17* clause has been checked against all seven pages rather
  than carried forward.
- The ten-versus-eleven passage still recommends the clean ten and no longer
  reads as a description of what the seven chapters measured.
- The three enumerations are counted from the source and from chapter 10, and
  each either holds or is corrected.
- The mechanical-check question is decided on this page, once, with
  `ledger-rollup-check-k207` named and the distinction between the two cases
  stated.
- No `558` or `547` survives anywhere under `docs/walkthroughs/`, checked with a
  positive control that the same pattern still finds something it should.
- `book-check --final --check all` is green over every book touched, and
  `bash scripts/check.sh` is no worse than before.
- **This is the node's last child.** When it retires, the node has no live leaf
  left; close it as `references/retire.md` directs rather than cutting an eighth.

## Notes

**No source change.** The corpus freeze is the parent node's rule and it holds
here too; a defect found becomes its own leaf.

**The control, reproduced six times.** The node brief's *Pointers* carries the
copy recipe, and it has reproduced exactly at `ch11-a-grove-begins-k208` through
`ch17-the-epoch-k213`: 560 tests, 549 passed, 11 failed, matched set for set.
This leaf takes no mutation run of its own — it has no arm to mutate — so it
needs the recipe only if it chooses to re-confirm the control once more.

**Measure with `GROVE_SIGNAL_FILE` unset in the measuring shell** if any run is
taken: this repository is a meta-grove and the session inherits a live signal
path.

## Decisions (running log)

**From `ch17-the-epoch-k213`, before this leaf starts.** The book-wide `558`
sweep in the *Done when* above has exactly two hits today, and only one of them
is yours. `19-the-core.md:123` is the citation to repair.
`11-a-grove-begins-k208`'s line 176 is **not**: it reads *exactly one test of
the 558 the suite* ***then*** *held*, and the paragraph immediately after it
opens *That reading is superseded* and states 560. It is a record of a past run,
frozen by its own stated rule
(`counts-split-current-state-from-record-of-a-run`), and replacing its number
would make the sentence false. Classify, do not substitute.

**The cross-chapter clause holds, checked page by page.** All seven chapters
named by line 123 state a control of 560 tests with eleven failing before any
mutation: `11-a-grove-begins.md:1514`, `12-leaf-to-node.md:1638`,
`13-outcomes.md:1776`, `14-finishing.md:1517`, `15-the-verbs.md:805`,
`16-the-lease.md:1966`, `17-the-epoch.md:995`. No two disagree about a number,
and the wordings differ exactly as intended. Three give the passing count as
well (15, 16, 17); four give the total and the eleven only (11, 12, 13, 14).
Five locate all eleven in `crates/grove-loop/tests/prompt.rs` (12, 13, 14, 15,
17); chapter 11 locates ten there and names the eleventh without locating it,
and chapter 16 locates none, citing the control as identical rather than
restating where it falls. Both are true as written — the eleventh *is* in that
file, at line 491 — so neither is a disagreement. The clause survives; only its
numbers move.

**All four counted claims on the page hold; none needed correcting.** Counted
from `crates/grove-loop/tests/prompt.rs` (614 lines, sixteen `#[test]`, the
namespace test at line 491 so *in* this file) and from chapter 10's own page:

- *Ten of this file's sixteen tests reach `compose`: nine through it or
  `compose_with` directly, and one through `signalling_contract()`* — exact.
  Direct: lines 107, 168, 188, 264, 292, 341, 417, 589, 608 (nine).
  `signalling_contract()` only: line 219 (one). Ten in all.
- *The evidence is sixteen tests, and the brief pins four of them* — sixteen
  confirmed; `docs/specs/grove-loop-book-structure.md:567-570` names exactly
  four, all four among the sixteen.
- *Chapter 10 … 626 tests over three crates … reports ten rather than eleven for
  the same reason* — `10-growing.md:1375-1380` says 626 tests over `grove-loop`,
  `grove-llm` and `grove`, and ten failing, all in `prompt.rs`, on the jj cause
  alone. The *same reason* clause is also right rather than merely repeated: the
  namespace test already existed at `loop-crate-driver-k22` (2026-08-31), the
  earliest revision touching `prompt.rs`, so chapter 10's ten is a copy that
  carried `.claude-plugin/`, not a suite that lacked the test.
- *the other six tests … compose nothing* (16 − 10) — exact: lines 311, 370,
  450, 474, 491, 528. And *anything touching `PLUGIN`, which that test reads
  five times* — five `prompt::PLUGIN` reads at lines 497, 501, 505, 508, 509.

**The forward reference resolves.** Chapter 17 states the relink step in its own
procedure paragraph (`17-the-epoch.md:1004-1010`), *after* the edit and with the
`workspace_binary` reason — the same failure mode line 151 cites it for. The
phrase is hard-wrapped across `cargo build -p grove` / `--bins`, so a one-line
grep for it finds nothing (`wrapped-phrases-defeat-line-greps`); it was confirmed
by reading the paragraph.

**Line 123 repaired, and the clause strengthened rather than merely carried.**
560/549/11. The sentence now also states *how* the seven agree — each took the
control in its own copy and states it in its own words rather than citing this
page — which is the fact the seven re-measurements established and the one that
matters to a reader deciding how much the roll-up is worth. Deliberately **not**
added: a count of how many of the seven give the passing count (three) versus the
total only (four). That split is true today and recorded above, but putting it on
the page would plant a fresh cross-chapter count for a later rewording to
falsify — the exact class this node exists to clear.

**The ten-versus-eleven passage is split in two, and the recommendation is now
marked as one.** The hazard argument (a flat eleven writes the marketplace test
off, and anything touching `PLUGIN` then reads as unobserved) is unchanged. What
followed it — *copy `.claude-plugin/` and the baseline is a clean ten* — sat in
the same breath as the description of the seven chapters' controls and could be
read as saying they measured tens. It is now its own paragraph, opening *the
recommendation for the next study*, and closes by saying explicitly that the
seven's controls are the eleven, that the eleventh's cause is separated on this
page rather than in the copy, and that re-deriving against a ten would change
what those paragraphs are *about*. The node brief's ruling, on the page.

**`k213`'s finding: state the discipline here, leave the account at seventeen.**
The shared-baseline section is the right place for *a control is a set, not a
count* — it is the rule every one of the seven relies on and none of them owns,
and this page previously never said it. It is not the right place for the
`task_grow` flake: chapter 17 met it, measured it, and settled it by running the
candidate alone, and that is three paragraphs of evidence that would be a
summary here and a duplicate there. So the page states the rule, points at
seventeen for the sharpest case, and does not retell it.

**The mechanical-check question: answered no, with the reason and the
substitute.** Three grounds, on the page:

1. **Nothing in the repository holds the number.** It comes from a run in a
   scratch copy that is deliberately not a jj repository and carries no
   marketplace manifest. A checker has nothing to compare a sentence against, and
   one that took the run itself would be a second measurement, not a check.
2. **The only truth-free mechanism is green in exactly this failure.** Requiring
   every page stating a baseline to state the same one would have passed
   throughout: the seven agreed with each other and disagreed only with the
   suite. A check on agreement rewards copying, and copying is how one stale
   reading reached seven pages. Its residual value — catching a *partial*
   re-derivation — is stated rather than suppressed, and it is a guard on the
   repair, not a check on the claim.
3. **`ledger-rollup-check-k207` is the contrast, and the distinction is the
   source.** Its roll-up is equally English prose, but derived from a file
   `book-check` already parses, so a checker there has something to compare
   against and needs only a locator. Phrased tense-robustly, so it stays true
   whether or not k207 has landed when this page is read.

The substitute is structural and stated as not preventing staleness: each
measuring page states its own control, so the carriers are an enumerable set
rather than a citation chain, and the roll-up page comes last, where the cost is
reading the seven pages it speaks for — once per re-derivation.

**One stale surface found off-page, and it is the summary layer.**
`concept-index.md:555` read *The environmental baseline* ***five*** *chapters
diffed against* against a page that says seven. Corrected to seven. This is the
`a-finding-against-a-section-does-not-reach-the-summary-layer` case exactly: the
index is not reached by any check that reads the section, and the sweep for `558`
would never have found it because the index states no number from the run. Two
index rows added for the new material — the recommendation/measurement
distinction, and the roll-up-no-artifact-holds decision.

**The sweep, with both controls watched.** `grep -rn '558'` and `'547'` over
`docs/walkthroughs/`. Surviving `558`: `grove-llm/07-what-order-holds.md:515`, a
binary hash inside a captured console transcript (`removed_surface-05584dc9be…`),
and `11-a-grove-begins.md:176`, the frozen record-of-a-run k213 pre-classified.
Surviving `547`: eight hits, every one a fragment `lines=` range or its
`source-index.md` row. **Positive control**: the same command finds `560` in ten
files including all seven chapters. **Dirty control**: planting the exact stale
sentence in a new file under `docs/walkthroughs/` made both patterns report it,
and removing it returned both to clean — so the instrument was watched to fail
before its clean read was credited. The probe was a **dotfile** and `grep -rn`
found it, which also settles that the search space here includes hidden files
(it would not have under ripgrep without `--hidden`).

**The in-session review allowance was not spent, deliberately.** Every claim this
leaf touched was settled by enumeration against source or against another page —
stronger evidence than a reader. The one judgement-bearing output is the
mechanical-check decision, and the strongest objection to it (the agreement check
is not worthless; it catches partial re-derivation) is stated in the decision
itself rather than left for a reviewer to find. One reviewer remains available to
a later leaf that revisits it.

**Gates.** `book-check --repo . --book <b> --final --check all` green over all
six books — `grove-loop` at 13 files / 10,557 resolved lines. `bash
scripts/check.sh`: **all 8 principal checks pass**, which is no worse than
before.

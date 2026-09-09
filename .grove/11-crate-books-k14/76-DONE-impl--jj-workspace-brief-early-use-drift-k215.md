# jj-workspace-brief-early-use-drift-k215

## Goal

Reconcile `docs/specs/jj-workspace-book-structure.md`'s early-use table with
`docs/walkthroughs/jj-workspace/walkthrough.toml`'s `[[early-use]]` entries, which
disagree on three rows — deciding, for each, which side is wrong rather than
making one match the other by reflex.

## Context

Surfaced by `refusal-remedies-are-jjs-overclaim-k202`, which changed the `Refusal`
row on both sides and, checking that it had left them in agreement, found the rest
of the table already out of agreement. **k202 did not touch these three rows**; they
predate it.

The brief states of itself (`:26`) that the chapter sequence and ownership mapping
are what the manifest records, and that *"where the two disagree, that is a defect
in one of them, not a licence to prefer either"*. So this is the brief's own rule
falling due, three times. Measured by parsing both — 8 `[[early-use]]` entries in
the manifest, 7 rows in the brief's table:

| Symbol family | `walkthrough.toml` | the brief |
| --- | --- | --- |
| `main_repo_of` | present, first-use `01-orientation.md#commit-tour` | **absent entirely** |
| `Commit` | first-use `01-orientation.md#public-surface` | first-use `01-orientation.md#commit-tour` |
| `control_dir` | symbols `` `control_dir` ``, first-use `#public-surface` | symbols `` `control_dir`, *namespace* ``, first-use `#commit-tour` |

**The manifest is not automatically right.** It is the side a machine checks —
`crates/book-validation/src/ledger.rs`'s `check_early_uses` renders each entry into
a required `source-index.md` row and fails if the row is missing — so an entry can
be self-consistently wrong across manifest and rendered ledger together while the
brief holds the correct human contract. `Commit` and `control_dir` are the live
question: both are introduced in `01-orientation.md`'s *The public surface*
(`:330-346`), which is evidence for the manifest, but the brief was authored
against a chapter plan and may be recording where the reader is meant to first
*need* them, which is a different thing.

`main_repo_of` is the asymmetric one: a row present in the manifest and absent from
the brief is either a symbol the brief forgot or one the manifest should not
demand.

## Done when

- Each of the three rows is decided on its evidence, with the losing side changed
  and the reason recorded — not resolved by declaring one document canonical.
- The two tables agree row for row, and `source-index.md`'s rendered early-use
  table still agrees with the manifest byte for byte.
- `bash scripts/check.sh` passes.

## Notes

**Nothing here is a source change**, so no fragment range moves and the freeze
holds.

**`check.sh` is green today and will stay green whatever you decide.** The
validator compares the manifest against the rendered ledger and never reads the
brief, so the disagreement this leaf exists to close is invisible to it — the same
blindness `k202` was warned about. Re-derive both tables by parsing them; do not
read them by eye.

## Decisions (running log)

**The `:22–26` disagreement rule does not govern this table, and a second rule
does.** That paragraph scopes itself to "the chapter sequence and ownership
mapping … what `walkthrough.toml` records as its `[[page]]` and `[[block]]`
groups". `[[early-use]]` is neither. The rule that governs is
`docs/specs/walkthrough-books.md`, *Early-use ledger*: the manifest's entries are
"the rows a book may not omit; authors add further rows", and the roll-up table
names `early-use-rows-declared` "the floor, not the ledger". So the manifest is a
**mandatory floor**, and the brief's table declares the same floor for humans —
which is why the two must hold the same rows, and why the jj-workspace ledger
carrying 12 rows against a floor of 8 is not itself a disagreement.

**All three rows are decided against the brief, and none by declaring the
manifest canonical.** The brief contradicts *itself* in two places, and the
manifest agrees with the brief's correct half:

- **`Commit` — first use is `#public-surface`.** The brief's own chapter-1
  description (`:143-147`) puts "the public surface: `Workspace`, `Commit`,
  `Refusal`, and the four operations" in that section, and the book introduces
  `Commit` there (`01-orientation.md:334-336`) carrying the ledger statement
  verbatim. `#public-surface` (`:314`) precedes `#commit-tour` (`:362`).
- **`control_dir` — same, plus the symbols cell.** `control_dir` is one of the
  four operations the brief's chapter-1 description places in the public surface,
  and the namespace paragraph carrying the row's exact statement is in
  `#public-surface` (`01-orientation.md:358-362`). The cell drops
  `, *namespace*`: all 103 early-use rows across the six rendered books use
  backticked code spans only, so the italic concept is the corpus's sole outlier,
  the spec asks for "the symbol or closely coupled type family", and the
  statement already carries the namespace's meaning.
- **`main_repo_of` — the brief gains the row.** The brief prescribes the chapter-1
  trace *containing* `main_repo_of` (`:155`) and then requires that "each
  later-owned name receives the minimum local statement recorded in *Early uses*"
  (`:168`). It is owned by `one-lane`, two chapters later. So it is forced by the
  same cost the section is named for, and the brief's own rule demands the row its
  table omits.

**That `*Early uses*` means the brief's own table, not the book's ledger.** The
sentence is ambiguous read alone, and the sibling brief disambiguates it: the same
sentence at `docs/specs/ordinal-fs-tree-book-structure.md:117` reads "recorded in
*Early uses* **below**", which can only be that document's own `## Early uses the
order forces` table — and that table does carry every later-owned name its
chapter-1 trace names. So the jj brief promised a local statement recorded in a
table of its own that recorded none. The defect stands without reference to the
manifest at all, which is what keeps this from being a preference for the
machine-readable side.

**The third option was considered and rejected on the merits.** The row could
instead have been *removed* from the manifest, dropping the floor to 7 and leaving
`main_repo_of` ledger-only like the four author additions. That option is
mechanically live — the ledger would stay at 12 and only `«early-use-rows-declared»`
would change — and it has real textual support:
`docs/specs/walkthrough-books.md:273-274` says authors add further rows "to the
book's own ledger", so the developmental edit writing an addition straight into the
manifest is a route that clause does not describe. It is rejected because **the
floor is decided by whether the order forces the name, not by who typed the row.**
`main_repo_of` was written into the brief's prescribed trace before a page was
drafted; the four genuine additions are drafting discoveries — three
refusal-constructor families whose content depends on which refusals each worked
example happens to name, and `Refusal::path_not_text`, which did not exist until
`lossy-path-rendering-k66`. That is the tiebreaker, and it puts `main_repo_of` on
the floor. The developmental editor reached the right destination by a route
`:273-274` does not sanction.

**The drift has a recorded cause, and it is one event.**
`docs/evaluations/editorial-pipeline-pilot/` shows the developmental-edit stage
making exactly these three corrections to the *book* side — `main_repo_of` "gained
a prose statement and a row in both the manifest and the ledger", and "two ledger
rows were re-pointed from `#commit-tour` to `#public-surface`, where their minimum
local statements actually are" (README, *Developmental edit*; stage record,
citations 2 and 3). The brief was never brought along. So this leaf is not three
independent defects but one un-propagated stage.

**The pilot's records are not corrected.** They are dated measurement records and
were true when cut: the brief *did* fix seven at that time, and the stage
described its own change accurately. Only live documents owe the present.

**The repair fans out to the assembly chapter, and no live sibling owns it.**
`07-what-jj-owns.md:441-450` states the split in words — "fixed seven", "five
forced", "Five more were added" — which roll-up checking cannot see, since it
requires figures in digits and these are words. Correcting the brief falsifies
that sentence, so it is corrected here. `concept-index-rollups-k217` covers
`concept-index.md`'s *structural* exclusion from the check, not a standing false
claim this leaf creates; `jj-workspace`'s concept index states no early-use count.
The `«early-use-rows»` (12) and `«early-use-rows-declared»` (8) figures are
unchanged, because the manifest and the ledger are both already correct.

**The leaf's one in-session reviewer was spent, on the weakest link.** The
`main_repo_of` call overruled how the pilot's own stage record classified that row
(as an addition the brief's "authors add" clause permits, not as a brief defect),
so a fresh context was given the brief, the spec and the pilot record with the
verdict stripped out and asked to argue against adding the row. It upheld the
change and returned two findings, both applied: this derivation, which previously
asserted the manifest's authority instead of deriving it; and the assembly
chapter's new sentence, which claimed the brief's *current* state in prose no
validator reads — rephrased as a record of the three actions taken. It also
supplied the two citations above (`walkthrough-books.md:273-274` and the sibling
brief's "below"), neither of which this session had found. Its one non-finding —
that "three for the chapters that first name a refusal constructor they do not
own" does not uniquely pick out three — is pre-existing in identical wording and
is not this leaf's to fix.

**The defect class was swept across all six books, and one live instance was
externalised.** Parsing every structure brief against its manifest found:
`ordinal-fs-tree` clean at 9 rows; `overview` differing on all five rows
*notationally only* (its brief states first use and owner in chapter terms where
its manifest uses `page#anchor` and slice) with no factual disagreement;
`keyed-launch` and `grove-loop` carrying no early-use table in their briefs at all;
and **`grove-llm` disagreeing on two statements** — `Workspace` and
`SessionEpochGuard`, in both cases the manifest carrying a qualification the brief
lacks. That is this leaf's own shape and does not serve its goal, so it is
`grove-llm-brief-early-use-drift-k218` rather than work absorbed here. Whether the
two brief-side omissions are themselves defects is left to that leaf; nothing live
owned it — `grove-llm-book-k33` has no live leaf beneath it.

**What nothing checks, and why no leaf is cut for it here.** No rule requires a
brief's stated minimum to equal its manifest's floor: `walkthrough-books.md`
`:1088-1090` ties the brief to the manifest for `[[page]]` and `[[block]]` groups
only, and `:273-274` points author additions at the ledger. So this whole defect
class is unbound by any rule and unseen by any check, in six books, and `k215` and
`k218` are both instances of it rather than the class. Cutting the rule-or-check
leaf is deliberately **not** done from here: `k218` should settle whether the
`overview` notational split and the two absent tables are permitted variation or
further instances first, because that answer decides whether the right artifact is
a spec sentence, a validator check, or neither.

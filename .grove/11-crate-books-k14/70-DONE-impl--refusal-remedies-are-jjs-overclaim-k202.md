# refusal-remedies-are-jjs-overclaim-k202

## Goal

Decide what the `jj-workspace` book's **summary layer** may say about whose
remedy a `Refusal` carries, and make the four surfaces that still overclaim it
agree — including the one `jj-workspace-method-counts-k184` was forbidden to
touch.

## Context

`jj-workspace-method-counts-k184` established, and its chapter-6 and chapter-7
body text now states, that **only two of the eleven `Kind` arms name a jj
command** — `NotAWorkspace` (`jj git init --colocate`, `jj git init`) and
`CommitNotRecorded` (`jj undo`, `jj op log`). Those four literals are the only jj
commands in `refusal.rs`; measured by `grep -n 'jj [a-z]' crates/jj-workspace/src/refusal.rs`
with the comment lines dropped, and corroborated by `06-refusal.md`'s own seam
section, which already said *"None of the three arms names a jj command"*.

**The overclaim originates in the frozen source and propagates.**
`crates/jj-workspace/src/refusal.rs:10` says *"The remedies named here are
**jj's**"* and `:4-5` says a decline *"names what is wrong, where, and the command
that fixes it"*. Both are true of two arms and false of the other nine. The
corpus is frozen, so this leaf does not repair the comment; the question it owes
is **what a book does when its subject asserts something false about itself** —
attribute the claim to the source, hedge it, or contradict it in the book's own
voice and say why.

Four surfaces still carry the source's framing:

- **`01-orientation.md:330-331`** — *"`Refusal` … an opaque value carrying what is
  wrong, where, and the jj command that fixes it"*, in the book's own voice.
- **`walkthrough.toml`, the `[[early-use]]` entry for `` `Refusal` `` (~line 183)** —
  the same sentence as the early-use *minimum local statement*.
- **`source-index.md:160`** — **not separately editable**: `crates/book-validation/src/ledger.rs:535-559`
  *renders* the early-use table from the manifest, so the manifest entry and this
  row are one fact and must change together.
- **The row-6 cells of both assembly tables** — `01-orientation.md`'s *The six
  refusals* (*"jj, whose repair the refusal quotes"*) and `07-what-jj-owns.md:29`
  (*"jj, whose repair the message quotes and does not run"*).

## Done when

- One reading is chosen and stated once, and all four surfaces obey it — with the
  chapter-6 body text (`#no-remedy-of-its-own`) as the settled statement the
  summary layer compresses, not a fifth independent phrasing.
- The book's relationship to the source's own overclaim is explicit somewhere a
  reader meets it, rather than resolved by silently disagreeing with a comment the
  book reproduces verbatim two chapters later.
- `bash scripts/check.sh` passes.

## Notes

**This one *does* touch `walkthrough.toml`, and that is why it is a separate
leaf.** `k184`'s `Done when` required the manifest untouched — correctly, because
no source byte moved there and no fragment range or ledger row was at risk. The
early-use `statement` is authored prose rather than derived data, so editing it
is legitimate; it simply was not `k184`'s to edit. **No source byte changes here
either**, so no fragment range moves and the freeze holds.

**`book-check` is blind to all of it.** It proves the quoted bytes match the
source and says nothing about whether a sentence about those bytes is true — the
same blindness that made `k184`'s list long. Green is not evidence.

## Decisions (running log)

1. **The reading is the negative one: a refusal names a remedy that is never the
   caller's.** Not *the jj command that fixes it*. This is the compression of
   `06-refusal.md#no-remedy-of-its-own`'s settled sentence — *"Whose the remedy
   is varies… That none of them is the consumer's does not vary, and that is the
   refusal"* — and it is the only formulation true of all eleven arms. Rejected:
   *"jj's, mostly"* (a hedge the reader cannot check) and *"the jj command that
   fixes it"* with a footnote (leaves the summary layer asserting a falsehood and
   makes the correction opt-in).

2. **The book contradicts the source comment in its own voice, at the site where
   it reproduces it**, rather than attributing or hedging. Attribution — *"the
   module says the remedies are jj's"* — would leave the book with no position on
   a claim it holds all eleven messages against three paragraphs later; hedging
   would leave the reader unable to check either way. The comment itself is
   reproduced unchanged, and the page says why: a page that silently improved its
   subject would be unusable beside the file.

3. **Verified by enumeration, not by the task file's list of four.** Flattening
   hard wraps and searching `docs/` for *the jj command*, *command that fixes*,
   *whose repair*, *remedies … are*, *what jj offers* and *remedy of its own*
   found **two surfaces the task file did not list**:
   - `06-refusal.md`'s own chapter opening — *"it names what jj offers and stops
     there"*, the same overclaim in the book's voice, twenty lines above the
     paragraph that disclaims it (the compact instance near the subject's
     introduction);
   - `docs/specs/jj-workspace-book-structure.md` — which carries **both** the
     early-use statement (`:415`) and the row-6 cell (`:70`), and which states of
     itself that where it and the manifest disagree *"that is a defect in one of
     them, not a licence to prefer either"*. Correcting the manifest without it
     would manufacture exactly that defect.

   `:61`'s quotation of the comment is attributed and stays. `05-scope-and-commit.md`'s
   forward pointer says *no remedy of its own*, which is the true claim, and stays.
   `grove-llm/02-the-grammar.md` and `docs/adr/jj-is-the-only-lane.md` speak only
   of `NotAWorkspace`, one of the two arms that does name jj commands; both true,
   both stay.

4. **The arm counts are re-derived here, not carried over.** Read from the
   `impl fmt::Display` (`refusal.rs:150-251`): eleven arms; a second paragraph in
   nine, absent in `CommandFailed` and `OutputNotText`; a jj command named in
   exactly two, `NotAWorkspace` (`jj git init --colocate`, `jj git init`) and
   `CommitNotRecorded` (`jj undo`, `jj op log`); the remaining seven name the
   filesystem's, the environment's or this crate's own remedy. 2 + 7 + 2 = 11.

5. **Both halves of the source's framing overclaim, and only the second sentence's
   tail survives.** `:10`'s *"The remedies named here are jj's"* is false of nine
   arms; `:4-5`'s *"names what is wrong, where, and the command that fixes it"* is
   false of the same nine and of the two that name no remedy at all. What survives
   measurement is `:10-12`'s *"it never says what the caller should do about the
   refusal"* — the negative half. Note that `:12`'s *"it says what jj offers"* does
   **not** survive: it is the same overclaim restated, so the chapter is held to
   the negative clause alone.

6. **Six surfaces changed, plus the chapter's own roll-up.** In order:
   `06-refusal.md`'s chapter opening (*"it names what jj offers"* → *"it names a
   remedy that is not the caller's"*); `06-refusal.md`'s module-thesis paragraph,
   rewritten from an endorsement into the explicit adjudication `Done when` asks
   for; `06-refusal.md`'s closing recap of the message shape, whose third slot was
   named *what jj offers* — a section-level finding that had not reached the
   chapter's own summary; `01-orientation.md`'s public-surface sentence;
   `01-orientation.md` row 6; `walkthrough.toml`'s early-use `statement` and the
   `source-index.md:160` row it renders, changed together; `07-what-jj-owns.md`
   row 6; and the structure brief's row 6, early-use row, chapter-6 thesis, and a
   new note under the spine table saying row 6 does not repeat the comment's
   wording and why.

7. **The chapter title is left alone, deliberately.** *It speaks for jj, and never
   for you* (`06-refusal.md`, and `concept-index.md:66` which mirrors it) is the
   one summary-layer string not changed. Under its natural reading it is not the
   false claim: the crate speaks **in jj's terms** — jj's vocabulary, jj's model of
   a workspace — which holds for all eleven arms, whereas *"jj, whose repair the
   refusal quotes"* under a column headed *Who owns it instead* was a direct
   factual assignment and did not. Changing it would touch the concept index and
   the brief's chapter list for no truth the body does not already deliver.
   Recorded so a later reader knows it was weighed rather than missed.

8. **`scripts/check.sh` passes — 8/8 principal checks, 6 books, `jj-workspace`
   final at 4 files / 752 resolved lines.** Per this leaf's own note that is not
   evidence for anything here: it proves the rendered early-use row still matches
   the manifest byte for byte, and reconstruction still holds because no source
   byte moved. It says nothing about whether the replacement sentence is true.

9. **The leaf's one in-session reviewer was spent, and it earned its place.** A
   fresh context was given the diff, the frozen source and the six contract points,
   with the conclusion stripped and an adversarial *find what is wrong* prompt. Nine
   findings, classified:

   **Valid and actionable — fixed.**
   - *`06-refusal.md:43`* — *"The defence is that each remedy is a jj command whose
     effect is the same for every caller"*: the overclaim in the book's own voice,
     stated more strongly than any cell decision 6 changed, ten lines below the
     paragraph that disclaims it. **My enumeration missed it** because the pattern
     list did not contain *each remedy is a jj command* — the exact failure
     `execute.md` names: a pattern list is complete only as far as the list. Rewritten
     to the defence that actually holds (offered rather than performed; correctness
     independent of the caller), with the 2-command / 7-non-command split stated.
   - *The count.* I had written *"true of two of the eleven kinds and false of the
     other nine"*. A claim about *the remedies named here* cannot be false of an arm
     that names no remedy. The settled table at `#the-case-analysis` is the control —
     *"Two rows are `none`, and the nine that are not point variously at jj, at the
     filesystem…"* — so it is **true of 2, false of 7, and vacuous in 2**. Corrected.
   - *A universal over a type two of whose arms carry no remedy.* The early-use
     statement and the orientation sentence asserted a `Refusal` carries a remedy,
     full stop. `CommandFailed` and `OutputNotText` carry none. Now hedged as *where
     it has one*.
   - *Row 6 answered a question its column header did not ask.* Under **Who owns it
     instead**, *"whoever the repair belongs to, which is never the caller"* names no
     owner where rows 1–5 each name one. Now names them.
   - *The remedy table's pointer had been desynchronised.* `#the-case-analysis` says
     its fifth column is checkable *"under the reading the chapter opened on: no entry
     in it is a statement about a consumer's policy"* — and my edit had changed that
     opening to a different, stronger phrasing. The opening now states no formulation
     of its own (*"it names whatever remedy exists and stops there"*), leaving the
     test where the table expects to find it.
   - *Wrap damage* in four paragraphs; re-flowed to 79 columns.
   - *"the middle clause"* named neither a single clause nor a middle. Fixed.

   **A contract I stated unclearly — and the finding is the evidence.** Findings
   A1–A3 argued that `OutsideWorkspace`, `Namespace` and `NotScoped` name remedies
   the *caller* performs, so *"never the caller's"* is false. The possessive was
   doing two jobs — *whose offer it is* versus *who types it* — and a fresh reader
   took the second. That is the wording failing, not the claim: the settled test is
   caller-**independence**, and all three pass it. Replaced throughout with the
   book's own checkable formulation, *true whoever is calling*, which cannot be
   misread that way and is what `07-what-jj-owns.md`'s benefit cell already said.

   **Externalised.** Three further brief/manifest early-use disagreements
   (`main_repo_of`, `Commit`, `control_dir`) — real, pre-existing, and not this
   leaf's goal. Cut as `jj-workspace-brief-early-use-drift-k215`.

   **Weighed and declined.** `#the-premise`'s heading, *"The cause chain, and the
   repair that belongs to jj"*, and its `concept-index.md:67` row. That section's
   subject is jj's operation log and `jj undo` — one of the two arms where the
   repair genuinely is jj's — so the heading is true of what it heads. Recorded
   because decision 7 had weighed the chapter title and not this.

10. **The widened sweep found a seventh surface the reviewer also missed.** Once
    the pattern list was replaced with a structural sweep — any clause putting `jj`
    within sixty characters of *remedy*, *repair*, *command* or *fixes*, with the
    reproduced source fragments and code fences stripped — `07-what-jj-owns.md:119`
    turned up: the **guarantee** table's row 6, sourcing the guarantee to *"jj's
    `undo`, `op log` and `git init` commands"* and answering *Strong enough?* with
    *"Yes — the named repairs are jj's, and they do repair"*. Its own prose section
    twelve rows down says the opposite about scope. Two tables in that chapter carry
    row 6, and `k184`, this leaf's task file, and the reviewer between them named
    only one. Both now state the split.

    Checked and left: *"the weakest check of the five"* in the same row is not a
    miscount — row 1 is set apart in the prose as the ceiling the other five are
    measured against.

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

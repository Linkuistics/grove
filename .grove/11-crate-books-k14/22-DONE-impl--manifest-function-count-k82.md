# manifest-function-count-k82

## Goal

Correct the one count in `crates/grove/Cargo.toml` that the source does not bear
out — *the reason it is three functions long*, lines 31–32 — and land it as a
corpus change the book contract permits.

## Context

- Observed at `orientation-k77` while drafting the overview's chapter 1, which
  owns lines 27–33 of the manifest as the fragment `manifest-dependencies`.
  Outside the test module the crate defines two functions, `main` and `run`;
  counting the test helper `undescribed` makes three, and counting the two
  `#[test]` functions makes five. No reading a reader takes first yields three.
- The page states the structural fact beside the fragment — two production
  functions, one a one-line call to the other — and says the comment is
  reproduced as written. Once the comment changes, that paragraph in
  `docs/walkthroughs/overview/01-orientation.md` is wrong the other way and
  must be rewritten in the same commit.
- Prefer a wording with no count at all (*the reason there is nothing else in
  it*), on the rule in the grove spine's `references/execute.md`: a claim
  documented by a count of itself goes stale the moment the thing it counts
  changes.

## Done when

- The comment states something the source bears out, and the chapter-1
  paragraph that adjudicates it is rewritten to match.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, the affected fragment and page, and a green
  `book-check --final` over the overview book. A one-line rewording moves no
  line boundary; a rewording that changes the line count moves every boundary
  below it and must re-prove the whole `source-crate-manifest` root.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside `jj-docs-url-k64` and its
neighbours: editing a byte of a frozen root while a book that quotes it is being
written invalidates the ranges the freeze protects.

## Decisions (running log)

- **The comment loses its count rather than gaining a correct one.** Written as
  the brief preferred: `crates/grove/Cargo.toml:31-32` now reads *The loop. This
  binary's only grove dependency, and the reason there is nothing else in it.*
  Two lines still, both under 80 columns, so `source-crate-manifest` stays 54
  lines and the `manifest-dependencies` fragment's `lines="27-33"` is untouched.
  No block boundary in `docs/walkthroughs/overview/walkthrough.toml` moves.
- **The validator was seen to fail before it was trusted.** With the source
  changed and the page not yet updated, `book-check --final --check all` over the
  overview reported `F008` at `01-orientation.md:230`, source byte 1240 — the
  fragment, by name. A green run after the page edit is therefore evidence rather
  than a habit; the pre-edit run was green too, so both directions are seen.
- **Three surfaces changed in the book, not one.** The fragment
  (`01-orientation.md:236-237`), the paragraph that adjudicated the count
  (`01-orientation.md:221-228`), and the concept-index entry
  (`concept-index.md:22`), which read *The function count the manifest states* —
  the summary layer a finding against a section does not reach by itself. Swept
  the whole book for `count`/`functions`: chapter 3's *three counts of three* and
  chapter 5's *counted three crates* are about `main.rs`'s steps and about
  workspace crates, not about this comment, and are untouched.
- **The rewritten paragraph keeps the ambiguity and drops the verdict.** It now
  explains why no number could be right — two outside the test module, three with
  the helper, five with the tests — rather than announcing an inconsistency that
  no longer exists. The preceding paragraph's *the comment says why the binary is
  short: the loop is behind it* still holds under the new wording and was left
  alone; the closing sentence was trimmed so the two do not restate each other.
- **The sibling defect is a new leaf, not this one's work.**
  `crates/grove-loop/src/lib.rs:15` says *one binary per audience, each three
  functions long* — false for `grove` (two) and far from true for `grove-llm`
  (`cli.rs` alone defines over twenty). Different frozen root, different book: one
  commit here could not carry a green `book-check` over the `grove-loop` book.
  Cut as `library-root-function-count-k187` at the end of this node, after every
  book, with the measurement and the `k82` wording precedent in its body.
- **The measurement was retaken after the last edit, not before it.** The first
  green `scripts/check.sh` (all 8) ran against a page whose closing sentence was
  then reworded — *the same claim without the arithmetic* — so that reading was
  discarded and the whole script re-run against the final bytes. An instrument
  adjusted mid-reading has not read anything.

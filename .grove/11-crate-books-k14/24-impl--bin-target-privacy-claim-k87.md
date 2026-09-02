# bin-target-privacy-claim-k87

## Goal

Correct the claim, made in three places, that *a binary target inside
`grove-loop` could reach that library's private items* —
`crates/grove/Cargo.toml` lines 15–19, `docs/specs/module-decomposition.md`
decision 1 (*a binary target can reach its own library's private items*), and
the *Command surfaces* residue of `docs/ARCHITECTURE.md` that restates it —
so that each names the shape the clause holds for, and land the one frozen-root
edit as a corpus change the book contract permits.

## Context

- Observed at `proving-a-negative-k80` under its in-session technical review,
  and measured there on a scratch package with one `[lib]` and one `[[bin]]`
  beside it: a binary that depends on the library and names a `pub(crate)`
  item through the library's path is refused with `E0603`; a binary that
  includes the library's source file as its own module (`#[path]`, or `mod` of
  a file it lists itself) compiles and reaches the same item. So the clause is
  true of a `[[bin]]` that compiles the loop's modules as its own crate, and
  false of a `[[bin]]` that depends on the library — which is the ordinary
  shape, and the one the comment's phrase *binary target inside `grove-loop`*
  reads as.
- The overview's chapter 1 (`docs/walkthroughs/overview/01-orientation.md`,
  *A crate, not a `[[bin]]` target*) now states both shapes beside the fragment
  that reproduces the comment, says which one the clause holds for, and says
  that the comment and decision 1 are reproduced as written. Once the comment
  changes, that adjudicating paragraph is wrong the other way and must be
  rewritten in the same commit; the same applies to chapter 4's table row about
  a `[lib]` on the `grove` package.
- One site is a frozen root: lines 15–19 of `crates/grove/Cargo.toml` are the
  overview's (`manifest-crate-not-a-bin`, lines 14–19). The other two are
  prose documents in no corpus.
- The property the manifest actually holds is narrower than its clause and
  still worth stating: a separate package cannot take the same-crate shape
  without a `#[path]` attribute pointing outside itself, which a reader of the
  two Rust files sees at once, and everything named through `grove_loop::` is
  a `pub` re-export whether the binary is a package or a target. Prefer a
  wording that names the shape over one that keeps the universal claim.

## Done when

- The three sentences state something a scratch package bears out, and the
  chapter-1 and chapter-4 paragraphs that adjudicate the clause are rewritten
  to match.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, every affected fragment and page of the overview,
  and a green `book-check --final` over it. A rewording that keeps the line
  count of lines 15–19 moves no boundary; one that changes it re-proves every
  range below it in that root.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside
`manifest-function-count-k82`, `grove-llm-version-comment-k83`,
`every-member-version-comment-k84` and `template-source-read-count-k86`, and
ahead of `architecture-residue-k75` because the `ARCHITECTURE.md` sentence is
residue that leaf may otherwise delete unread: editing a byte of a frozen root
while a book that quotes it is being written invalidates the ranges the
freeze protects.

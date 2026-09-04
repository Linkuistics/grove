# every-member-version-comment-k84

## Goal

Correct the claim, made in three places, that *every member takes
`version.workspace = true`* — `crates/grove/src/cli.rs` line 12,
`crates/grove-loop/src/lib.rs` line 68, and the workspace root `Cargo.toml`
line 58 — and land the two frozen-root edits as corpus changes the book contract
permits.

## Context

- Observed at `the-surface-k78` under its in-session technical review.
  `crates/book-validation/Cargo.toml` line 3 is `version = "0.1.0"`, and
  `book-validation` is a workspace member (root `Cargo.toml`, `[workspace]
  members`). Six of seven members inherit the workspace version; the claim as
  written is false.
- The overview's chapter 2 adjudicates the claim beside the fragment that
  reproduces line 12: the invariant the comment needs is that every crate on the
  path from `grove` to `grove-loop` inherits one version, and that holds.
  Chapter 1's sentence about `version.workspace = true` was narrowed to the
  same statement in the same session. Once the comment changes, chapter 2's
  adjudicating paragraph in `docs/walkthroughs/overview/02-the-surface.md` is
  wrong the other way and must be rewritten in the same commit.
- Two of the three sites are frozen roots: line 12 of `cli.rs` is the overview's
  (`surface-clap-attributes`, lines 8–18), and line 68 of `grove-loop`'s crate
  root belongs to the `grove-loop` book (`grove-loop-book-k37`). The root
  `Cargo.toml` is in no corpus.
- Two fixes are available and the second is better. Either `book-validation`
  takes `version.workspace = true` too, making the comment true — but the root
  brief earmarks that crate to leave the workspace, and tying its version to the
  release is the wrong direction for a crate on its way out. Or the three
  comments say what is actually held: every crate an operator installs
  inherits one version. Prefer the wording with no universal quantifier over a
  set that is about to change.

## Done when

- The three comments state something the manifests bear out, and the chapter-2
  paragraph that adjudicates line 12 is rewritten to match.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, every affected fragment and page of the overview
  and `grove-loop` books, and a green `book-check --final` over each. A
  rewording that keeps each line count moves no boundary; one that changes it
  re-proves every range below it in that root.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside
`manifest-function-count-k82` and `grove-llm-version-comment-k83`: editing a
byte of a frozen root while a book that quotes it is being written invalidates
the ranges the freeze protects.

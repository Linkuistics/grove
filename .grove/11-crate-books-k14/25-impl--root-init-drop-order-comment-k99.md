# root-init-drop-order-comment-k99

## Goal

Correct the one claim in `crates/grove-llm/src/cli.rs` that the source does not
bear out: the `cmd_root_init` comment (lines 495–500) that argues for `match`
over `let … else` on a drop-order ground which is the reverse of what the
compiler does — and land it as a corpus change the book contract permits.

## Context

- Observed at `growing-the-tree-k95` while drafting the `grove-llm` book's
  chapter 4, which owns `handler-root-init-vacancy` (`cli.rs` 492–513) and
  reproduces this comment. The comment reads, in part: *A `let … else` binding
  drops the unmatched value at the end of the whole statement — after the `else`
  block — so the live `TreeWrite` would still be holding the exclusive lock while
  that block ran … `match` drops it on entry to the arm.*
- **The compiler does the opposite, in both editions.** Compiled and run under
  the workspace toolchain (`rustc 1.98`, `Cargo.toml` `edition = "2021"`) with a
  `Drop`-tracing reproduction of the two shapes, under `--edition 2021` and
  `--edition 2024` alike: a `match` scrutinee is a value of the enclosing `let`
  statement and lives until that statement ends, so the `Writing::Tree` value —
  and its exclusive lock — is alive **through** the `Writing::Tree(_)` arm body
  and dropped **after** it; a `let … else` initializer's value is dropped
  **before** the else block runs. So the lock is held through the `match` failure
  arm the code uses and released before a `let … else` failure block — a tree
  read added to name the live leaf would deadlock in the arm this code chose, not
  in the form the comment rejects. Independently reproduced by the chapter's
  in-session review, which also noted `TreeWrite::relinquish` is private to
  `grove-loop`, so `cli.rs` could not use the safe path even if it wanted to.
- **The code is correct today regardless.** The `match` arm reads no tree — it
  builds a literal message and returns — so no second opening is taken and the
  self-deadlock is unreachable in either form. Only the comment's stated *reason*
  is wrong, and it would mislead the very future maintainer it addresses into
  thinking `match` is the safe form for adding a tree read.
- The chapter states the checkable fact beside the fragment and reproduces the
  comment as written, as `orientation-k92` and `the-grammar-k93` did for their
  stale claims; it names no leaf, as `grove-llm-version-comment-k83`'s page does
  not. This leaf owns the rewrite.
- `crates/grove-llm/src/cli.rs` is the `source-command-surface` root of the
  `grove-llm` book (`grove-llm-book-k33`), which owns lines 492–513 and must
  reconstruct them. A rewording that keeps the line count moves no boundary; one
  that changes it moves every range below in that root.

## Done when

- The comment states something the source bears out — the real reason `match` is
  used, or is written to match the compiler's actual drop order — with no claim
  about drop order that the compiler contradicts.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, every affected fragment and page of the `grove-llm` book
  (at least `handler-root-init-vacancy` and the adjudicating prose in
  `04-growing-the-tree.md`), and a green `book-check --final` over that book.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside the other frozen-corpus
comment defects (`grove-llm-version-comment-k83` and its neighbours) and ahead of
`architecture-residue-k75`: editing a byte of a frozen root while the book that
quotes it is being written invalidates the ranges the freeze protects. If the
`grove-llm` book lands first and quotes the comment as written, its page has
already adjudicated the stale claim, and this leaf rewrites the comment and that
page's adjudication in the same commit.

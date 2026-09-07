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

## Decisions (running log)

- **The drop order is re-measured, not taken from the task file.** A
  `Drop`-tracing reproduction of both shapes, compiled with `rustc 1.98` under
  `--edition 2021` and `--edition 2024` alike, prints `inside arm body` then
  `drop TreeWrite` for the `match` form and `drop TreeWrite` then
  `inside else block` for the `let … else` form. Both editions agree, and both
  contradict the comment. `TreeWrite` owns the lock guard directly
  (`crates/grove-loop/src/lib.rs` 174–180, `opened: RefCell<Option<Guard>>`),
  so the value's lifetime *is* the lock's.
- **The comment is reworded, the code shape is not changed.** The leaf's goal is
  the false claim, and `match` is a defensible style choice once its real
  consequence is stated. Rewriting the handler into a `let … else` would move
  line counts, and would be a behaviour-neutral refactor no leaf asked for.
- **The rewrite keeps the comment at exactly six lines** (`cli.rs` 495–500), so
  `cli.rs` stays at 944 lines and no fragment range below it moves. It stops
  arguing `match` *over* `let … else` and instead states the consequence the
  compiler actually produces: the write is held through the arm, which is why
  the message stays a literal, and a tree read added there would need the
  `let … else` form.
- **Seven surfaces, enumerated rather than swept.** The fragment
  (`04-growing-the-tree.md` 169–174); the adjudicating passage and its table
  (204–235, now 204–234); `07-what-order-holds.md`'s stale-claims tally
  (360–382); `concept-index.md` line 80, whose title said *the drop order the
  comment gets backwards*; and two chapter-plan pointers in
  `docs/specs/grove-llm-book-structure.md` (153 and 259) that named a
  *`match`-not-`let … else` argument* the source no longer makes. The spec's
  `## Known in advance` section is **not** among them: it is scoped to what
  chapter 1 adjudicates, and this claim was found while drafting chapter 4, so
  it was never recorded there. No line count moved, so no other book's
  `check.sh` transcript is falsified.
- **The tally is re-enumerated, never decremented.** `07-what-order-holds.md`
  still says *Five were judged worth a source change* and still names all five;
  a new sentence records that one has landed and that the other four remain
  reproduced as written. While rewriting it, *each now has a leaf to carry one*
  was corrected to name **four** leaves for five claims —
  `grove-llm-dependency-comments-k102` carries both of the known-in-advance pair.
- **`## `root-init`: the vacancy, and why `match`` keeps its heading and its
  `the-vacancy` anchor.** The section still explains why `match` is what is
  there and what it costs; renaming it would move a cited anchor for no gain.
- **The validator's green was earned against a control.** A one-byte case change
  inside the rewritten comment made `book-check` report `F008` at source byte
  27197; restoring it returned `valid: 4 files, 1017 resolved lines`. All six
  books and all eight principal checks pass.
- **The leaf's one in-session reviewer was spent on the counts and
  cross-references, and returned six findings, classified four ways.** Three
  were valid and actionable in text this leaf rewrote, and are fixed here:
  - the section heading *and why `match`* was residue of the old position — the
    rewritten body gives no *why*, only a cost, so it is now *and what `match`
    costs*. The cited `the-vacancy` anchor is explicit and did not move, and no
    other file names the old title. This **reverses** the decision logged above
    to keep it; the reviewer was right that the ground it stood on had gone.
  - `07-what-order-holds.md`'s partition was not exhaustive.
    `grove-llm-dependency-comments-k102` carries **three** claims, not two: the
    manifest's *a binary target can reach its own library's private items* is a
    sixth claim, judged worth a source change and adjudicated by *Orientation*,
    and it fell through both of the paragraph's sets. The sentence now names it
    and says why it travels with the pair, so *Five* and *four leaves* are both
    true and the partition covers the corpus.
  - the paragraph's opening universal read as contradicted by its own later
    *one has since landed*. Rewritten to be explicitly about drafting time.
  - Two are **valid but pre-existing, in prose this leaf did not rewrite**, and
    are externalised rather than absorbed:
    `lock-scan-blind-to-contention-probe-k190` (the lock scan's production slice
    is cut at a stray `#[cfg(test)]` on line 60 of `task_tree.rs`, so it never
    sees the contention probe the test's own doc comment names — replicated:
    four matches, all in `driver_lease.rs`) and
    `two-openings-uniqueness-claim-k191` (*the one grow verb that has to open
    the tree twice*, refuted sixty lines later by `leaf-insert`'s
    relinquish-then-read).
  - One is **rejected as noise**: that *One of the five has since landed*
    asserts a landed commit while this leaf is live. Retire-then-commit puts the
    `DONE` rename and every page in one change, so the sentence is true of every
    tree that ever contains it.
  The reviewer also independently reproduced the drop order in both editions and
  confirmed the table's third column against `ordinal-fs-tree`'s blocking
  `LOCK_EX`, the `relinquish` visibility claim, fragment byte-fidelity, and the
  four named tests.

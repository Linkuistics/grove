# manifest-dependency-clauses-k133

## Goal

Bring two clauses of `crates/grove-loop/Cargo.toml`'s dependency comment up to
the crate as it is since `loop-crate-driver-k22`, and rewrite the paragraph in
`docs/walkthroughs/grove-loop/01-orientation.md` that adjudicates them, in one
commit with a green validator run over the book.

## Context

- **The defect.** The comment at `crates/grove-loop/Cargo.toml` lines 17–25
  describes the crate's dependencies as they stood before the driver moved into
  this crate, and two of its clauses now stop short of the code:
  - *`libc` is the lock-contention probe in `task_tree`, which needs `flock(2)`
    non-blocking before it announces a wait.* `libc::` is reached from three
    modules, not one — eight times in `task_tree.rs` (the probe), **nineteen in
    `driver_lease.rs`** for the lease's locking and its close-on-exec
    descriptors, and three in `loop_driver.rs` for `isatty`, `tcgetpgrp` and
    `signal`. The largest user is the one the comment does not name.
  - *`keyed-launch` is the runner, reached by exactly one verb: `complete` …*
    True of the **verbs**, and `complete` is still the only one. It is not true
    of the crate: `driver_lease.rs` discards an abandoned channel,
    `loop_driver.rs` spawns and reaps through the runner, and
    `session_config.rs` compiles grove's templates against the runner's own
    types.
- **Found at `orientation-k124`, while drafting chapter 1**, and it is a **third**
  stale claim in this corpus — `docs/specs/grove-loop-book-structure.md`'s *No
  third stale claim was found* is wrong, and that sentence is part of the fix.
- **The book already adjudicates it and does not repeat it.**
  `01-orientation.md`, under *The package*, states which clause is still true and
  which is not, beside the fragment that reproduces both. When this leaf lands,
  that paragraph is rewritten in the same commit as the comment, because the
  adjudication exists only for as long as the claim does.
- **This is a corpus change and is deferred behind the whole `grove-loop` book,**
  which is why it sits here rather than ahead of it. The comment is inside
  `manifest-domain-bound`, lines 1–59, owned by chapter 1: adding a line moves
  every line below it and invalidates that block's fragments and the book's
  ledger. `crates/grove-loop/Cargo.toml` belongs to no other book, so the blast
  radius is one book — but that book must be complete before the bytes move.
- Its neighbours are the same shape and were placed for the same reason:
  `every-member-version-comment-k84` and `template-source-read-count-k86`.

## Done when

- The two clauses describe the crate as it is, naming `driver_lease` and
  `loop_driver` for `libc` and distinguishing *the verb surface reaches the
  runner once* from *the crate reaches it in four places*.
- `01-orientation.md`'s adjudicating paragraph is rewritten to match, and the
  book's fragments and ledger are updated for whatever line movement the edit
  causes.
- One commit carries the source change, the affected pages and ledger, and a
  green `book-check --final --check all` over `docs/walkthroughs/grove-loop`.
- `docs/specs/grove-loop-book-structure.md`'s *Known in advance* section names
  this as the third adjudicated claim rather than asserting there is no third.
- `bash scripts/check.sh` passes.

## Notes

**Do not fix this by deleting the clauses.** The dependency argument is the
chapter's evidence for what the crate imposes; what is wrong is its scope, not
its existence.

## Decisions (running log)

# forward-commitment-tests-k131

## Goal

Settle what happens to the two forward-commitment tests in
`crates/grove/tests/corpus_exception_inventory.rs` now that the last promised
book exists, and to the sentence in `docs/specs/walkthrough-books.md` that
promises them, so `cargo test` is green again for the twenty sessions still
drafting the `grove-loop` book.

## Context

- **Both tests are red as of `orientation-k124`**, and neither is red because
  anything is wrong: `the_inventory_covers_books_that_have_not_been_written_yet`
  and `the_subject_inventory_covers_books_that_have_not_been_written_yet` each
  assert that at least one inventory row names a book with no directory under
  `docs/walkthroughs/`. `grove-loop` was the last such row; creating the book's
  directory emptied the set. The assertion message says it exactly — *every
  inventory row names an existing book, so the table records no forward
  commitment*.
- Their stated purpose is in their own doc comment: the inventory names books
  that do not exist yet so that *a book landing later cannot quietly widen its
  own corpus*, and without the check the equality test above them would still
  pass for a table whose future rows had been deleted along with the book that
  was going to carry them. That hazard is now closed by history rather than by a
  test: every row names a book that exists and whose manifest is compared against
  it row for row.
- `docs/specs/walkthrough-books.md` states the same property in prose, under
  *The corpus rule and its witness*: *a repository test … requires the table to
  keep carrying rows for books that do not*. Whatever happens to the tests
  happens to that clause in the same commit, or the specification promises a
  check that is not there.
- **This is not a defect in the book and not a corpus change.**
  `crates/grove/tests/` is evidence rather than roots for the overview book, so
  nothing here touches a frozen range or invalidates a fragment.
- Two shapes to choose between, and the second is the one to argue against:
  retire both tests with the specification clause, recording that the property
  had a stated end and reached it; or narrow them to *a book whose manifest
  promises pages it does not yet carry*, which keeps a live check while the
  `grove-loop` book is still a prefix and then goes vacuous again at
  `what-could-not-move-k130`. A check that will need deleting twice is worth less
  than one deleted once with its reason recorded.
- Whatever lands must be **seen to fail**: the replacement, or the argument for
  no replacement, is worth nothing unless the run that proves it was watched
  going red against a table with a row deliberately removed.

## Done when

- `cargo test -q -p grove --test corpus_exception_inventory` passes, and the
  three tests that compare each manifest against the specification's two
  inventories still pass unchanged.
- `docs/specs/walkthrough-books.md` and the test file agree about which checks
  exist.
- `bash scripts/check.sh` is red on `book-check` alone — the shape a book still
  being drafted leaves — and green on `cargo test`.

## Notes

Placed here by `orientation-k124`, ahead of the rest of the `grove-loop` book,
because every drafting session between now and `what-could-not-move-k130` runs
`scripts/check.sh` and would otherwise re-diagnose these two failures.

## Decisions (running log)

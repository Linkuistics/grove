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

1. **Both forward-commitment tests are retired, not narrowed**, together with
   the specification clause that promised them. The hazard their doc comment
   states — a table's future rows deleted along with the book that was going to
   carry them — was live only because both equality tests filter the inventory
   down to books that exist, so a row for an unwritten book sat *outside* the
   comparison and could be deleted in silence. Every row now names an existing
   book, so every row is inside the comparison. Seen to fail, not argued: with
   the `grove-loop` exclude row and the `grove-loop` subject row each deleted
   from the specification's tables,
   `every_books_corpus_exceptions_are_exactly_the_specifications_inventory` and
   `every_books_subject_is_exactly_the_specifications_inventory` both went red
   and named the orphaned row. That run is the replacement's absence justified.
2. **The narrowing shape was rejected on its expiry.** A check asserting some
   manifest promises pages it does not yet carry is green only while the
   `grove-loop` book is a prefix and goes vacuous again at
   `what-could-not-move-k130` — its green would report where drafting has got
   to, not whether the corpus rule holds, and it would need deleting a second
   time by the leaf that finished the book.
3. **The `books.contains(book)` filter stays in both equality tests**, with its
   comment rewritten to say why: a row may still be committed ahead of its book,
   which is the pattern this campaign's deliverables were commissioned under.
   Dropping the filter would make an inventory row for a nonexistent book a
   failure — strictly stronger, and a tempting third shape — but it would close
   the forward-commitment pattern the specification deliberately established for
   any future book. That is a design change to the specification, not the
   cleanup this leaf was cut for, and it is not made here.
4. **One stale count fixed in passing.** The paragraph under the exception table
   read *Six rows is not the six sets of per-book tables*; the table carries
   seven (`ordinal-fs-tree` one `add` and five `exclude`, `grove-loop` one
   `exclude`). The same paragraph's *five campaign deliverables that appear in
   no row* was wrong in the same direction — four books appear in no row — and
   is now *A book that appears in no row declares none*, which does not have to
   be recounted when a book lands.
5. **Verified as the *Done when* asks, with one detour recorded.**
   `cargo test -q -p grove --test corpus_exception_inventory` passes with three
   tests; the two retired ones are gone and the three that compare each manifest
   against the specification's two inventories — including the narrowing attack —
   are unchanged. `bash scripts/check.sh` ends `FAILED — 1 of 8`, `✗ book-check`
   alone, with `cargo test` green; `book-check` is red on `grove-loop` only, for
   the six pages `16-the-lease` through `21-what-could-not-move` that this arm
   has not drafted yet. The detour: the first `scripts/check.sh` run wedged for
   over an hour inside `crates/grove/tests/loop_driver.rs`. That is
   `driver-lease-fixture-timing-k85`'s defect, not this diff's — it touches no
   file that suite reads — and the evidence, including a measurement ruling
   `GROVE_SIGNAL_FILE` out as the cause, was added to that leaf's *Context*
   rather than acted on here.

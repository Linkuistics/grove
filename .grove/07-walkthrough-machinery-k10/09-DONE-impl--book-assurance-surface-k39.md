# book-assurance-surface-k39

## Goal

Make decision 8 of `plan-k1` machine-held for books: every book root under
`docs/walkthroughs/` joins the curated user-documentation surface, and every book
root has a row in `docs/ARCHITECTURE.md`'s *Documentation ownership* table that a
test enforces. Land it green over the one book that exists today.

## Context

- **Why this leaf exists.** `plan-k1` decision 8 settled that "the book roots
  join the curated user-surface list; each book earns a tested ownership-table
  row". `review-planning` (`walkthroughs-k9`, F3) found that no leaf and no brief
  in the `walkthroughs-k3` subtree assigned either obligation, so the whole
  campaign could close green while violating a settled requirement. This leaf is
  the assignment.
- **The two surfaces as they stand.** `crates/grove/tests/reference_navigation.rs`
  holds `USER_DOCS`, a hand-enumerated list of five top-level guides, and the
  comment above it argues the enumeration deliberately — the claim is about a
  *surface a person navigates*, not about every Markdown file. Books belong in
  that surface; five hard-coded strings cannot grow to hold them without an edit
  per book, which is the defect decision 8's "machine-held" is aimed at.
  `docs/ARCHITECTURE.md`'s *Documentation ownership* table has no row for the
  overview or for any book, and nothing tests that it ever will.
- **Discovery, not enumeration, is the deliverable.** A sixth book must join both
  surfaces by existing under `docs/walkthroughs/`, with no edit here. Otherwise
  the obligation simply moves to five later leaves that will each forget it, and
  the reviewer's finding recurs one layer down.
- `architecture-move-k31` separately owes the *overview's* own ownership row as
  part of the move. That is the row's content; this leaf is the check that
  notices when a row is missing. They do not overlap and neither replaces the
  other.
- The relocated `ordinal-fs-tree` book is the only book root that exists when
  this leaf runs, so it is the subject the new checks are proved against — and
  its ownership row is this leaf's to add.

## Done when

- Every book root under `docs/walkthroughs/` is inside the curated
  user-documentation surface, discovered from the directory rather than
  enumerated, and `user_documentation_references_resolve` covers it.
- A test fails when a book root under `docs/walkthroughs/` has no row in
  `docs/ARCHITECTURE.md`'s *Documentation ownership* table.
- Both checks are **seen to fail** before they are credited: add a scratch book
  root with no ownership row and no resolvable link, watch each check go red, and
  remove it. A check that has never been seen to fail is not a check.
- `docs/ARCHITECTURE.md` carries the relocated book's ownership row, and the
  table's own surrounding prose — which bounds what may sit directly under
  `docs/` — still describes what is true.
- `bash scripts/check.sh` passes.

## Notes

**Do not widen the sweep into a repo-wide one.** `USER_DOCS`' comment states why
the surface is curated: architecture, decision records, specs and the provisioned
methodology are separately owned. Books join the curated surface because a reader
navigates them; that is an addition to the list's membership rule, not the
abandonment of it.

**This leaf lands green today and is useful on its own** — the checks are real
whether or not another book is ever written, which is why it is its own leaf and
not a clause bolted onto `validator-fragments-k22`'s corpus work.

## Decisions (running log)

**The curated surface stays a list, and gains one discovered member class.**
`USER_DOCS` became `NAMED_GUIDES` — the same five strings, unchanged — plus
`user_documentation()`, which appends one entry per directory under
`docs/walkthroughs/`. The alternative considered and rejected was making the
whole surface a glob over `docs/**`, which the constant's own comment argues
against: architecture, decision records, specs and the provisioned methodology
are separately owned, and the repo-wide sweep already covers them. Discovery is
scoped to the one directory whose contents a reader navigates like a guide.

**A book's surface entry is all of its pages, not just its contents page.** A
book is navigated as one document by its Previous/Contents/Next lines, so the
whole directory is the member. The consequence is that the "must carry at least
one relative reference" assertion — the guard that stops the check passing by
finding nothing to do — moved from per *file* to per *surface entry*. Per file
it would have been a false failure the first time an honest chapter carried no
link; per entry it still rejects the case it was written for, since a book with
no relative link anywhere is not navigable.

**A row naming any page inside a book root satisfies the ownership check.** The
canonical source for a book is its contents page (`README.md`), not its
directory, so the check accepts an exact match or a path under `<book>/`. The
trailing separator is deliberate: without it `docs/walkthroughs/keyed-launch-notes`
would answer for `docs/walkthroughs/keyed-launch`, and a unit case holds that.

**The ownership table is read textually, and does not re-verify its links.**
`documentation_ownership_targets` resolves each row's destination against `docs/`
as a string and never touches the filesystem. Whether a row's target exists is
already `every_repository_markdown_reference_resolves`' claim; two checks
reporting one broken link only degrades the second one's message.

**Both checks were seen to fail before being credited.**
`every_book_root_has_a_documentation_ownership_row` went red on its first run
against the unmodified repository, naming `docs/walkthroughs/ordinal-fs-tree` —
the real defect the leaf exists to close, not a fixture. A scratch book root
(`docs/walkthroughs/scratch-check/`, one `README.md` with a dangling link and no
row) then reddened both: the ownership check listed the scratch root alongside
the real one, and `user_documentation_references_resolve` reported
`scratch-check/README.md:4` — proving the surface grew to cover a book that did
not exist when the check was written. The scratch root was removed.

**The specification gained a cross-reference, not a new obligation.**
`docs/specs/walkthrough-books.md` already explains why the subject inventory is
*not* folded into the ownership row; the sentence added there records that the
row is separately owed by every book and machine-held, so a future book's author
meets the requirement in the contract rather than only in a failing test.

**No CHANGELOG entry, following the node's three sibling leaves.**
`validator-structure-k21`, `validator-fragments-k22` and `corpus-subject-anchor-k50`
each landed without one, so the campaign is logging at the grain of what it
ships — the books and the guide — rather than per leaf of the machinery that
authors them. A lone entry here would read as though this leaf shipped something
the other three did not. The obligation stays with whoever lands the campaign's
user-visible work.

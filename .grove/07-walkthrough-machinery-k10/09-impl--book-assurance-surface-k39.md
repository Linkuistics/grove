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

# stale-slug-precondition-comment-k162

## Goal

Rewrite the four-line comment at `crates/grove-loop/src/tree_lifecycle.rs` lines
538 to 541, which justifies a slug precondition `leaf_decompose` no longer
performs, inside the file's frozen 2,725-line count — and reconcile the page that
reproduces the changed bytes.

## Context

- **The defect.** Lines 538 to 542 are:

      // Grove's own precondition, before the tree is even observed, so a bad slug
      // leaves the leaf un-decomposed. It could sit inside the guard with the rest
      // of them; it stays out here because it needs nothing from the tree, and
      // refusing without taking an exclusive lock is strictly kinder.
      let child_slug = first_child_slug;

  Line 542 refuses nothing. It is a rebinding of a parameter already typed
  `&Slug`, and a `Slug` cannot be built from text the grammar disclaims. There is
  no check here to sit inside the guard or outside it.
- **The file is its own refutation, 1,386 lines further down.**
  `decompose_cannot_be_reached_with_a_bad_child_slug`'s doc comment (lines 1,924
  to 1,931) records the change: *the claim used to be about ordering: the slug was
  validated before the rename … Since `loop-crate-verbs-k21` the verb takes a
  [`Slug`], so the text is read by the type that owns it and a bad slug never
  reaches a tree at all.* Two passages of one file disagree about which mechanism
  holds one claim, and the later one is right.
- **The claim itself is still true and must survive the rewrite.** A bad child
  slug does leave the leaf un-decomposed, and it does cost no exclusive lock —
  because the `Slug` was constructed by the caller before `write` was ever called.
  What has to go is *Grove's own precondition* and *refusing without taking an
  exclusive lock*, which describe code that is not there.
- **No instrument reports it.** The comment is `//`, not `///`, so
  `cargo doc --no-deps --document-private-items` never sees it — thirty warnings
  across the crate, none for `tree_lifecycle.rs`. This is
  `grow-header-stale-helper-k154`'s blind spot in a third form: not a `//` header
  with a bodyless heading and not a welded summary, but a `//` rationale outliving
  its code.
- **A fix fits in five lines**, which is what keeps the count frozen: lines 538 to
  542 stay five lines and nothing after them moves. One shape that fits is to say
  that the slug was validated by its type at the caller, so the refusal costs no
  lock — dropping the *it could sit inside the guard* clause, which is about a
  choice nobody makes any more.
- **Found by `leaf-to-node-k156`** while drafting chapter 12, which reproduces the
  block and adjudicates it on the page under *A precondition that moved into the
  type, and the comment left behind*.

## Done when

- Lines 538 to 542 describe the code that is there: the precondition is the
  `Slug` type's, discharged at the caller, and no lock is taken to refuse.
- **`crates/grove-loop/src/tree_lifecycle.rs` is still exactly 2,725 lines**, so
  no ownership range, manifest `lines` value or fragment range moves.
- `12-leaf-to-node.md` reproduces the new bytes and its adjudication is rewritten
  to describe the repaired comment; the two `concept-index.md` entries naming the
  defect follow.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  at whatever slice the book is proved at when this runs.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Deferred behind the `grove-loop` book**, with `unresolved-doc-links-k151`,
`unreachable-root-clause-k152`, `grow-header-stale-helper-k154`,
`default-root-slug-two-spellings-k159`, `welded-grove-name-summary-k160` and
`refused-grove-test-overclaims-k161`, for the same reason: the bytes are
reproduced by a finished page.

**Do not delete the comment.** Its second clause is the true shape of the
function and chapter 12 leans on it: the slug is refused before the tree is
observed, which is what makes a bad spelling cost no exclusive lock. What is
stale is the mechanism, not the claim.

## Decisions (running log)

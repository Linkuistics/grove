# welded-grove-name-summary-k160

## Goal

Separate the two summary sentences welded into `grove_name`'s first doc paragraph
in `crates/grove-loop/src/tree_lifecycle.rs`, inside the file's frozen 2,725-line
count, and reconcile the page that reproduces the changed bytes.

## Context

- **The defect.** Lines 1013 to 1017 are one `///` run with no blank line in it:

      /// The grove's name is the worktree directory's basename (user-owned-worktrees
      /// — grove reads no branch, ever). Used as the root brief's `# <name> — brief`
      /// title.
      /// The grove's display name for its own charter: the **worktree** directory's
      /// basename, read off the tree root the store is about to create.

  Two independent summaries of the same function, written at different times, the
  older one first. Markdown joins them into a single paragraph.
- **Confirmed from the rendered output, not from the source.**
  `target/doc/grove_loop/tree_lifecycle/fn.grove_name.html` has exactly two `<p>`
  elements: the first is **three** sentences — the older summary's two, then the
  newer summary's one — and the second is the *It takes the grove root rather than
  the worktree* paragraph. rustdoc's short description is the first paragraph, so
  `tree_lifecycle/index.html` prints that whole run as `grove_name`'s summary.
  **The defect is the redundancy, not the length**: fifteen of the thirty-one
  items in that index carry a multi-sentence summary, so length alone says
  nothing. What is wrong is that the third sentence re-describes what the first
  two already said.
- **No instrument in this repository reports it.** `cargo doc --no-deps
  --document-private-items -p grove-loop` emits **thirty** warnings across the
  crate and **none at all** for `tree_lifecycle.rs` — both paragraphs are
  attached to the right item, so there is nothing for it to warn about. This is
  the blind spot `grow-header-stale-helper-k154` found in one form (a `//` header
  it cannot see) reappearing in another: a correctly attached doc comment whose
  *shape* is wrong.
- **A fix fits in five lines**, which is what keeps the count frozen — fold the
  older sentence's content into the newer one as a single summary, or spend one
  of the five on a blank `///`. Either way lines 1013–1017 stay five lines and
  nothing after them moves.
- **Found by `a-grove-begins-k155`** while drafting chapter 11, which reproduces
  the block and adjudicates on the page.

## Done when

- `grove_name`'s docblock states its purpose **once**: the rendered
  `fn.grove_name.html` still has two `<p>` elements, and the first no longer
  describes the function twice.
- **`crates/grove-loop/src/tree_lifecycle.rs` is still exactly 2,725 lines**, so
  no ownership range, manifest `lines` value or fragment range moves.
- `11-a-grove-begins.md` reproduces the new bytes and its adjudication is
  rewritten to describe the repaired comment; any `concept-index.md` entry naming
  the defect follows.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  at whatever slice the book is proved at when this runs.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Deferred behind the `grove-loop` book**, with `unresolved-doc-links-k151`,
`unreachable-root-clause-k152`, `grow-header-stale-helper-k154` and
`default-root-slug-two-spellings-k159`, for the same reason: the bytes are
reproduced by a finished page.

**Do not widen this into a sweep of the crate's doc comments.** The remaining
modules are read by the chapters that own them, and each cuts its own leaf. What
this one adds to the standing procedure is that the check is *count the
paragraphs in the rendered docblock*, which is cheap and which nothing else does.

## Decisions (running log)

# leaf-to-node-k156

## Goal

Draft chapter 12 of the `grove-loop` book, *A leaf becomes a node*, over
`tree_lifecycle.rs`'s two `the-key-survives` blocks — `490-695` and `1666-2234`,
775 lines — and prove the prefix through slice `the-key-survives`.

## Context

- Child 2 of 4 of `no-word-for-k127`. The rule is **the key is preserved, because
  the entity that was the leaf becomes the node**: the leaf *file*
  `NN-<kind>--<slug>-k<key>.md` becomes the node *directory* `NN-<slug>-k<key>/`,
  its body renamed in as `BRIEF.md`, and a first child grown atomically so a node
  is never childless.
- Blocks: `decompose-production` (`490-695`: `leaf_decompose`, `decomposable`,
  `promoted`) and `decompose-tests` (`1666-2234`), whose two labelled sections are
  *leaf-decompose* at `1666` and *leaf-decompose: the seam* at `1971`.
- **The test support this block leans on is chapter 11's**, at `1077-1261`, and
  the two sections at `1666` and `1971` are the first consumers of `mknode`,
  `touch_body` and `list`. Chapter 11 reproduces and explains them; this chapter
  names them and does not re-explain.
- **`append_brief_suffix_in_file` is chapter 11's source and chapter 12 is its
  only caller anywhere** — `tree_lifecycle.rs:587`, inside `leaf_decompose`.
  Chapter 11 reproduces it and points forward; the account of *why a freshly
  decomposed brief is retitled at all* is this chapter's.
- **Two of its claims have no observer, established by mutation in
  `a-grove-begins-k155`.** Panicking on the conservative
  `return Ok(())` branch — the *already suffixed, or a custom title* arm — leaves
  all 558 `grove-loop` and `grove-llm` tests green against an 11-failure control,
  so neither the idempotence the doc comment claims nor the *never clobbers a
  custom title* promise is pinned by anything. State it where the helper is
  consumed.
- **The four refusals the structure brief names are a claim about attribution,
  not a count**, and chapter 8's precedent is that three of four refusal tests
  refused somewhere other than their names say. Read the `bail!` texts, match each
  assertion's substring to exactly one, then mutate each arm to a **panic** in a
  workspace copy and diff against an unmutated control run of the same copy. The
  harness `a-grove-begins-k155` used is described in this node's brief.
- The manifest's early-use rows are a floor; enumerate this chapter's own bytes,
  in both the Rust-path and the hyphenated-verb spelling.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  the-key-survives --check all` is valid: 13 files, 6,078 resolved lines, 4,455
  deferred, `final=false`.
- `12-leaf-to-node.md` exists, `README.md` and chapter 11's navigation are
  updated, and the two `the-key-survives` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf.

**Say which tree.** `ordinal-fs-tree`'s *leaf* and *node* are not grove's, and
this chapter's whole subject is one word in each glossary meaning different
things at once.

## Decisions (running log)

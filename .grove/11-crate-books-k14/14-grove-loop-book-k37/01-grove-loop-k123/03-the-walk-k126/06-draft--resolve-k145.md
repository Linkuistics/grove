# resolve-k145

## Goal

Draft chapter 9 of the `grove-loop` book — *Resolve*,
`docs/walkthroughs/grove-loop/09-resolve.md` — and prove the prefix through slice
`wider-than-a-key`.

## Context

- The fifth of `the-walk-k126`'s six chapter children, and **the largest**: its two
  ownership blocks are `resolution` (`task_tree.rs` 747–1015) and `resolve-tests`
  (1,653–1,996) — 613 lines, of which 344 are tests. Completing it resolves the
  last of `task_tree.rs`'s ten blocks and reconstructs the file in full; say so.
- The structure brief's section is *9 · Resolve*. The rule is that **grove's
  reference grammar is wider than a key, and has an ambiguous outcome the library
  has no counterpart for**: `Resolution`, `Located`, `located`, `resolve_in`,
  `Lookup`, `lookup`, `reference`, `existing_path`, `Ref` and `parse_ref` — a
  path, `[n]`, `n`, `<slug>-k<key>` or a bare slug.
- **The test block is two labelled sections, not one.** The source separates
  *resolve* (1,653–1,874) from *resolve: the full `<slug>-k<key>` handle
  (task-tree-scheme §5)* (1,875–1,996); both are inside the single
  `resolve-tests` block. Both take *supply the claim*: per reproduced test, the
  property it establishes **and** what would have to be true for it to pass while
  the property was broken. The production half is 42% prose and takes *do not
  restate*.
- **Two early-use rows close here**, both owned by `wider-than-a-key`: chapter
  1's cast row `verbs::resolve`, `Resolution`, and the `parse_ref` row added at
  `03-kind-slug-handle.md#one-place-the-grammar-is-spelled`. Both move from
  `pending` to `explained`, because `owner_is_complete` computes the expected
  status from the scope.
- `parse_ref`'s leniency on a bare key — `007` is key 7 — is the precedent
  chapter 3 argued `Handle::parse`'s leniency on `a-k007` from. This chapter is
  where that grammar is finally read, so it closes chapter 3's forward reference
  rather than repeating it.
- **The early-use ledger is a floor**: enumerate this chapter's reproduced bytes
  for later-owned symbols named or exercised, and add the rows they owe.
- The chapter's carried-example row is the brief's row 9: `plan-k1`, `[1]`, `1`
  and `plan` — four spellings of one reference — to one `Resolution`, or
  `Ambiguous` listing the keys. **This grove's own tree now poses that case**:
  `the-walk` is both this leaf's parent node `the-walk-k126` and its sibling
  `the-walk-k143`, so the ambiguous arm is live rather than hypothetical. The
  book is told strictly from the crate's side, so cite the behaviour and not this
  repository's `.grove/`.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  wider-than-a-key --check all` is valid: 13 files, 4,173 resolved lines, 6,360
  deferred, `final=false`.
- Chapter 9 exists, `README.md`'s contents entry and chapter 8's two navigation
  lines are updated, and both `wider-than-a-key` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

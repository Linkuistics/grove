# paths-k142

## Goal

Draft chapter 6 of the `grove-loop` book — *Paths, and addressing*,
`docs/walkthroughs/grove-loop/06-paths.md` — and prove the prefix through slice
`paths-are-built-here`.

## Context

- The second of `the-walk-k126`'s six chapter children. Its two ownership blocks are
  `paths-and-addressing` (`task_tree.rs` 291–570) and `path-composition-tests`
  (1,016–1,105) — 370 lines.
- The structure brief's section is *6 · Paths, and addressing*. The rule is that
  **the library returns no paths, so grove builds them — in exactly one place**:
  `entry_path` and why it is safe without a check; `Target`, `target` and
  `unreachable_by_any_walk`; `addressable_key` and `interrupted_promotion`;
  `next_key`, `live_leaf`, `entry_outcome`.
- **The chapter carries why nothing canonicalises for output.** On macOS `/var`
  and `/private/var` name the same inode, so canonicalising would make the mere
  presence of a lock rewrite every path grove prints. Chapter 5 reproduced the
  header paragraph that says so and pointed here; this is where the account sits,
  beside the function.
- The test block is the one the source itself labels *the path-taking
  compositions, which are the tests' alone* and takes *supply the claim*: per
  reproduced test, the property it establishes **and** what would have to be true
  for it to pass while the property was broken. The production half is 42% prose
  and takes *do not restate*.
- **This chapter closes the manifest's `entry_path` early-use row**, whose owner
  is `paths-are-built-here`: it moves from `pending` to `explained` with this
  slice, because `owner_is_complete` in `crates/book-validation/src/ledger.rs`
  computes the expected status from the scope and leaving it `pending` is `F009`.
  Chapter 1's cast row owned by `one-spelling-of-grove` moved to `explained` in
  chapter 5 for the same reason.
- **The early-use ledger is a floor.** Enumerate this chapter's own reproduced
  bytes for symbols a later chapter owns — named *or* exercised — and add a row
  per symbol not already covered, rather than treating the manifest's rows as the
  set. `floor-rows-chapter-two-k138` is the worked precedent.
- The chapter's carried-example row is the brief's row 6: from an entry of a
  snapshot to one absolute path, built in exactly one place.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  paths-are-built-here --check all` is valid: 13 files, 2,810 resolved lines,
  7,723 deferred, `final=false`.
- Chapter 6 exists, `README.md`'s contents entry and chapter 5's two navigation
  lines are updated, and both `paths-are-built-here` ownership rows read
  `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

# the-walk-k126

## Goal

Draft Part II of the `grove-loop` book — chapters 5 to 10, the whole of
`crates/grove-loop/src/task_tree.rs` (2,023 lines, ten blocks) and
`src/task_grow.rs` (518 lines, one block) — and prove the prefix through slice
`what-the-library-cannot-see`.

## Context

- Draft stage, child 3 of 7 of `grove-loop-k123`. The structure brief is
  `docs/specs/grove-loop-book-structure.md`; the six chapters are its sections
  *5 · Opening, contention and refusal* through *10 · Growing: `leaf-add` and
  `leaf-insert`*, and the mapping is its *Top-level ownership blocks* and
  *`task_tree.rs` five ways in ten blocks*.
- **Expect this to decompose, one child per chapter.** Six chapters at 290, 370,
  322, 428, 613 and 518 lines.
- The `task_tree.rs` blocks, in file order and with their owning chapter:
  `1-290` (5), `291-570` (6), `571-637` (7), `638-746` (8), `747-1015` (9),
  `1016-1105` (6), `1106-1360` (7), `1361-1652` (8), `1653-1996` (9),
  `1997-2023` (8). The last is the closing *pick + brief-chain together* block,
  which is chapter 8's because it exercises `brief_chain`. `task_grow.rs`
  `1-518` is chapter 10's, whole.
- **The prose obligation is directional here.** 1,008 of `task_tree.rs`'s lines
  are the inline test module at 15% prose and take *supply the claim* — for
  chapter 7's fifteen tests the brief names the specific form: the negative case
  for each, and `pick_orders_numerically_not_lexically` passes under a lexical
  sort until there are ten leaves, so `10` against `9` is what makes it a test.
  The production halves at 42% take *do not restate*.
- **Chapter 10's proof is entirely outside its own pages, and it is the only
  chapter of which that is true.** `src/task_grow/tests.rs` (1,680 lines) is the
  book's one declared corpus exclusion, so its tests are cited by name and never
  reproduced. The chapter says so.
- **One early-use row closes here and its first use is in chapter 5.** The
  manifest's `entry_path` row names `05-opening.md#one-spelling-of-the-root` as
  its first use and `paths-are-built-here` as its owner, because chapter 5
  reproduces the module header that names `entry_path` as the one place paths are
  built. Chapter 1's cast rows owned by `one-spelling-of-grove`,
  `first-live-leaf` and `wider-than-a-key` move to `explained` as those chapters
  land.
- Chapter 6 carries why nothing canonicalises for output: on macOS `/var` and
  `/private/var` name the same inode, so canonicalising would make the mere
  presence of a lock rewrite every path grove prints.
- The carried example's steps are the brief's *Worked examples* rows 5 to 10,
  ending at a sibling leaf whose key was **predicted** from the template's bytes
  and checked against the store's report.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  what-the-library-cannot-see --check all` is valid: 13 files, 4,691 resolved
  lines, 5,842 deferred, `final=false`.
- Chapters 5–10 exist, contents and navigation are updated, and the eleven
  ownership rows for `task_tree.rs` and `task_grow.rs` read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

# the-walk-k126 — brief

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
- **It decomposed one child per chapter**, as its own leaf body anticipated:
  2,541 lines over six chapters — 290, 370, 322, 428, 613 and 518. The blocks are
  fixed by the manifest, so the seam is the chapter boundary and nothing else.
- The `task_tree.rs` blocks, in file order and with their owning chapter:
  `1-290` (5), `291-570` (6), `571-637` (7), `638-746` (8), `747-1015` (9),
  `1016-1105` (6), `1106-1360` (7), `1361-1652` (8), `1653-1996` (9),
  `1997-2023` (8). The last is the closing *pick + brief-chain together* block,
  which is chapter 8's because it exercises `brief_chain`. `task_grow.rs`
  `1-518` is chapter 10's, whole. Production splits at the five concern
  boundaries the file's own header names; the inline test module's six labelled
  sections follow, in the order `path-taking compositions`, `pick`,
  `brief-chain`, `kind`, `resolve`, `resolve: the full <slug>-k<key> handle`,
  `pick + brief-chain together`.
- **The prose obligation is directional here.** 1,008 of `task_tree.rs`'s lines
  are the inline test module at 15% prose and take *supply the claim* — for
  chapter 7's fifteen tests the brief names the specific form: the negative case
  for each, and `pick_orders_numerically_not_lexically` passes under a lexical
  sort until there are ten leaves, so `10` against `9` is what makes it a test.
  The production halves at 42% take *do not restate*. Chapter 5 and chapter 10
  are wholly production and take *do not restate* throughout.
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
  presence of a lock rewrite every path grove prints. **Chapter 5 reproduces the
  header paragraph that states it** and points forward; the account is chapter
  6's, beside `entry_path` and `leaf_entry`.
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

## Decomposition

**Six children, one per chapter.** The blocks are fixed by the manifest, so the
seam is the chapter boundary and nothing else, and the order is forced:
`--through` proves a canonical prefix, so a child cannot prove its slice before
its predecessor's page exists.

| # | Child | Chapter | Slice | Owned lines | Cumulative resolved | Deferred |
|---:|---|---:|---|---:|---:|---:|
| 01 | `opening-k141` | 5 | `one-spelling-of-grove` | 290 | 2,440 | 8,093 |
| 02 | `paths-k142` | 6 | `paths-are-built-here` | 370 | 2,810 | 7,723 |
| 04 | `the-walk-k143` | 7 | `first-live-leaf` | 322 | 3,132 | 7,401 |
| 05 | `kind-and-briefs-k144` | 8 | `root-to-leaf` | 428 | 3,560 | 6,973 |
| 06 | `resolve-k145` | 9 | `wider-than-a-key` | 613 | 4,173 | 6,360 |
| 07 | `growing-k146` | 10 | `what-the-library-cannot-see` | 518 | 4,691 | 5,842 |

Position `03` is `pick-test-count-k147`, a correction leaf that owns no chapter
and moves no line count; *Found while drafting* below says what it is for. The
positions above are the node's as it stands, not a gapless run.

Each child's slug is its page id, which is the convention `orientation-k124` set
and `the-grammar-k125` kept. Every child leaves `scripts/check.sh` red on
`book-check` alone and says so; the script runs `--final` over every book root by
discovery, and a prefix deliberately leaves later blocks deferred.

**`the-walk` is now two entries and a bare-slug reference to it is ambiguous.**
This node is `the-walk-k126` and chapter 7's child is `the-walk-k143`, because
the child's slug is its page id and the page id is `the-walk`. `resolve` reports
the ambiguity and lists both keys rather than guessing, and the full
`<slug>-k<key>` handle resolves by its terminal key either way — so name either
one by its handle, here and in every later reference.

## Found while drafting

**The structure brief says chapter 7 has fifteen tests and the block has
nineteen, and `pick-test-count-k147` holds the correction.** `pick-tests`
(`crates/grove-loop/src/task_tree.rs` 1,106–1,360) carries nineteen `#[test]`
functions — eighteen named `pick_*` and one named `select_*` — so neither
narrower reading rescues fifteen. `task_tree.rs` is unchanged since
`grove-loop-structure-k36` wrote the brief, so it was wrong when written rather
than outrun. k147 is placed **before `the-walk-k143`**, because chapter 7 is the
page that would otherwise write the number; until it lands, do not reconcile a
page down to fifteen. This is the same shape as
`structure-brief-dependency-count-k132`, and the same lesson the parent brief
draws: **count before writing a count.**

**The `entry_path` early-use row is discharged by chapter 5 and closed by chapter
6.** The manifest's fourth `[[early-use]]` row anchors on
`05-opening.md#one-spelling-of-the-root`, so chapter 5 must carry an explicit
`<a id="one-spelling-of-the-root"></a>` and state the minimum locally; the row
moves to `explained` when chapter 6 reads the function.

**Two of the brief's other Part II counts were enumerated and stand.** Chapter
5's *four openings* is the 2×2 of {shared, exclusive} × {refusing, answering the
vacancy} — `read_or_vacant`, `read`, `write`, `write_or_vacancy` — with
`reopen_write` a diagnostic-free `write` and `open_write` the private
acquisition the two write-side entry points share; five `pub(crate)` functions
hand back a guard and four of them are that square. Chapter 5's *three error
paths* — `absent_tree`, `raised`, `restate` — are all three in the block.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

1. **Decomposed one child per chapter rather than drafting all six.** 2,541
   source lines is roughly six times the 436 that became chapter 1's 1,037
   markdown lines in one session, and half again `the-grammar-k125`'s 1,714 over
   three — which itself decomposed on the same ground. The manifest's block
   owners already partition both files along exactly the chapter boundary, so any
   other cut would leave a child unable to prove itself with `--through`.
2. **Kept the page-id slug for chapter 7 despite the collision with this node's
   own slug.** The alternative was a distinct slug — `pick-and-select` — which
   would have kept every bare reference unique. It was rejected because the
   convention that a child's slug *is* its page id is what lets a reader of the
   tree name the chapter a leaf drafts without opening it, and it holds across
   all three preceding books. The collision fails loudly rather than silently:
   `resolve` refuses an ambiguous bare slug and prints both keys.

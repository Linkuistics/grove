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

**Chapter 10 owns the evidence for two of chapter 9's refusals, and should say
so.** Found by `resolve-k145`, by mutation over all seven refusal arms in
`task_tree.rs` 747–1015. `reference`'s two `bail!`s — *no entry matches* and
*is ambiguous; re-query by key* — are held by `add_under_nonexistent_parent_errors`,
`insert_requires_an_existing_target` and
`add_refuses_an_ambiguous_parent_slug_and_lists_the_keys`, all in `task_grow`'s
tests, which are the book's **one declared corpus exclusion**: chapter 10 cites
them by name and may not reproduce them. So the crate's read-side grammar has
its operator-facing refusals pinned only from the write side, and only from the
1,680 lines this book never shows. Three of the seven arms are held by nothing
at all, and the two `parse_ref` clauses share one witness that reads no message.
Chapter 9 carries the table; chapter 10 owes the other half of the sentence.

**A count in a leaf body is not a count in the tree.** `resolve-k145`'s own
*Context* said two early-use rows closed there and three did — the third,
`reset_read_count`, `read_count`, was added to the ledger by `the-walk-k143`
after k145's body was written, and the **parent** brief predicted it in as many
words. The leaf bodies here were written at decomposition time and the ledger
has moved under them since. **Every remaining child should enumerate the ledger
for rows whose owner is its own slice rather than trusting its own body's
count** — `owner_is_complete` turns a missed row red, so this fails loudly, but
it fails after the page is written rather than before.

**`resolve` does not accept a path, and `Reference`'s doc comment reads as
though the grammar does.** `verbs::resolve` is one line over `resolve_in`, which
checks `.` and then goes straight to the key/slug grammar; the path form belongs
to `reference`, the door the mutating verbs come through. That is what
reconciles `lib.rs`'s *Four forms* against the five things the same sentence then
lists — the list is the union across both callers, and `.` is a path as well as a
root spelling. Chapter 9 adjudicates it; chapter 1 reproduced it without comment,
and **chapter 10 should not restate the count** when it reads `reference`'s
callers.

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

**The module header's *canonicalisation appears once* is false, and chapter 8
must not repeat it.** Found by `paths-k142`, which owns the counterexample.
`crates/grove-loop/src/task_tree.rs` lines 27 to 29 say *Canonicalisation appears
once, in `leaf_entry`, and only to compare a caller's spelling of a leaf against
the tree's.* Six production `canonicalize` calls sit in this file: 346, 349 and
369 inside `target`, which chapter 6 owns, and 712, 715 and 734 inside
`leaf_entry`, which chapter 8 owns. It is refuted from inside the corpus —
`target`'s own doc comment says *exactly as `leaf_entry` does*. Chapter 6
adjudicates it at `06-paths.md#canonicalise-to-compare`;
`canonicalisation-sites-k149`, cut under `crate-books-k14` ahead of
`architecture-residue-k75`, holds the source fix and the reconciliation of both
pages. **`kind-and-briefs-k144` must not restate the claim** when it reads
`leaf_entry`: the `leaf_entry` early-use row in the book's ledger was corrected in
the same pass and now points at chapter 6's adjudication.

**The `entry_path` early-use row is discharged by chapter 5 and closed by chapter
6.** The manifest's fourth `[[early-use]]` row anchors on
`05-opening.md#one-spelling-of-the-root`, so chapter 5 must carry an explicit
`<a id="one-spelling-of-the-root"></a>` and state the minimum locally; the row
moves to `explained` when chapter 6 reads the function.

**Four early-use rows beyond the manifest's were added by chapter 6, and two of
them were found by its reviewer rather than by its own sweep.**
`task_grow::allocated` (owner `what-the-library-cannot-see`) and `pick_in`,
`select_in` (owner `first-live-leaf`) came from the author's enumeration;
`tree_lifecycle::leaf_decompose` and `tree_lifecycle::leaf_retire` did not,
because the bytes name them in their **hyphenated verb spelling** — `leaf-retire`,
`leaf-decompose` — rather than as Rust paths, and a sweep looking for identifiers
does not see them. Chapter 5's `tree_lifecycle::leaf_prune` row is the precedent
and was written from an underscore spelling, which is exactly why the hyphenated
ones were missed. **Every remaining child should enumerate both spellings.**

**Two of the brief's other Part II counts were enumerated and stand.** Chapter
5's *four openings* is the 2×2 of {shared, exclusive} × {refusing, answering the
vacancy} — `read_or_vacant`, `read`, `write`, `write_or_vacancy` — with
`reopen_write` a diagnostic-free `write` and `open_write` the private
acquisition the two write-side entry points share; five `pub(crate)` functions
hand back a guard and four of them are that square. Chapter 5's *three error
paths* — `absent_tree`, `raised`, `restate` — are all three in the block.

**A fourth Part II claim in the brief is refuted, and it is in the same sentence
`pick-test-count-k147` corrected.** The chapter 7 section says
`pick_orders_numerically_not_lexically` *passes under a lexical sort too until
there are ten leaves, and it is `10` against `9` that makes it a test.* Under the
canonical grammar a position is zero-padded to **at least two digits**, so 9
renders `09` and 10 renders `10` and the two orderings agree on that pair; the
block's fixture is `100` against `99`, and the test's own comment gives the
reason. `structure-brief-lexical-pair-k150` holds the correction;
`07-the-walk.md` already states the true pair, so no page is reconciled down to
the brief. k147 read the sentence for its number and not for its example, which
is the general lesson: **a corrected sentence is not a checked sentence.**

**An early-use floor row is owed for a symbol whose *static* an earlier chapter
already covered.** Chapter 7's test block calls `reset_read_count()` and
`read_count()`, defined at `task_tree.rs` 1,006–1,014 inside chapter 9's block.
Chapter 5 declared and explained `READ_COUNT` itself and even named this
assertion, so a sweep asking *has this been covered?* answers yes while the two
accessors have no row at all. **The unit of an early-use row is the symbol, not
the concept**, and the enumeration must be over identifiers in the reproduced
bytes rather than over topics the book has discussed. Chapter 9 moves the row to
`explained`.

**Chapter 7's nineteen tests leave one broken implementation passing all of
them,** and the page states it: *return the deepest live leaf, breaking ties by
walk order* is indistinguishable from *the first live leaf in pre-order* under
every fixture in the block, because the five trees that hold both a node and a
live leaf all put the node at ordinal 01. Later chapters owe the same move — the
obligation is *what would this pass under*, and the answer is only worth writing
after the alternative implementation has been run against every fixture in the
block rather than against the one in front of you.

**A refusal a verb surfaces was usually not produced by that verb.** Chapter 7
first wrote that the species mismatch was *met in the walk rather than in the
parser*; `disagreement` is called inside `TaskName::parse`, which the library runs
while classifying entries, so the composition fails inside `read` and the walk is
never reached. Every later chapter reproducing a test that asserts on refusal text
owes the same check — chapters 8, 9 and 11 to 14 all have them — and the honest
form is *this is chapter 4's grammar observed through chapter N's verb*.

**Five rustdoc intra-doc links in this crate do not resolve, and
`unresolved-doc-links-k151` holds the fix.** Found by `kind-and-briefs-k144`.
Three are in `task_tree.rs` — line 580 `[`pick`]`, 586 `[`select`]`, 638
`[`kind`]` — and all three name public functions of `crate::verbs`, which this
module does not import; the other two are `prompt.rs` line 28 and `lib.rs` line
283. **The prose around each is true and only the link is broken**, which is the
same shape as the `llm_cli` stale address, so chapters 7 and 8 adjudicate on the
page rather than cutting the source. The instrument is `cargo doc --no-deps
--document-private-items`; nothing else sees it, and no check in this repository
runs it. **Every remaining chapter that reproduces a doc comment should run it
over its own block** rather than reading the links.

**`leaf_entry`'s grove-root clause cannot execute, and
`unreachable-root-clause-k152` holds the decision.** `task_tree.rs` 717–722 fires
only for an argument that is a regular file *and* canonicalises to the grove
root, and chapter 5's opening (line 276) guarantees the root is a directory while
a `Tree` exists. Its own test,
`brief_chain_errors_when_given_the_grove_root_itself`, refuses at `is_file`
twenty-nine lines earlier. Removal shifts every later line of a 2,023-line root,
so k152 is deferred behind the whole book and decides between removing and
re-ledgering or keeping and renaming the test.

**Mutation, not reading, is what settled `leaf_entry`'s coverage — and the result
was five of seven.** Replacing each refusal in a copy of the workspace, one at a
time, showed that the UTF-8 clause, the forwarded grammar `{error}`, both
root-containment clauses and the closing *not in the task tree* `bail!` are
unobserved by all 245 inline tests and all twenty-five `grove-llm` targets; only
the `is_file` clause and the *not a current-format Grove leaf* arm are held. A
control mutation of those two failed exactly two tests, which is what attributes
them. **Chapters 9 and 10 own functions with the same shape — a private resolver
with many `bail!` arms — and a coverage claim about them is worth exactly the same
re-run.** Note also that three of the four refusal *tests* in chapter 8's block
assert on messages produced by clauses other than the ones their names describe:
a test name is a label, and here it is a misleading one.

**A composed-verb test can be integrated at the wrong seam.** The file's closing
block is labelled *pick + brief-chain together* and opens the tree twice, once per
composition, which is exactly what `pick_in`'s and `brief_chain_at`'s doc comments
say production must not do. Chapter 8 states it. **Every later chapter reproducing
a test that composes two verbs owes the same check** — count the opens before
believing the label.

**Chapter 7 owes an early-use row it did not add, and k151 carries it.** Its block
reproduces `[`select`]`, whose referent `verbs::select` has no row anywhere in the
ledger. Chapter 8 added the two its own bytes owed, `verbs::kind` and
`verbs::brief_chain`, both from the **hyphenated and the linked spelling** the
parent brief warned about. The row needs the earlier page to state the minimum
locally, so it is chapter 7's prose to change and not a later leaf's to bolt on.

**A page's placement of its evidence is a claim, and the diagnosis of a wrong
one can be wrong in turn.** Found by `growing-k146`, **re-derived and corrected
by `chapter-nine-refusal-attribution-k153`**, which is where the finding's final
form is. k146 read `09-resolve.md`'s *What the refusals are worth, measured* —
which said both of `reference`'s operator-facing refusals are *pinned by
`task_grow`'s tests* — and grepped the two test names the page's arm 3 row
carried, finding them in `crates/grove-llm/tests/leaf.rs` at lines 432 and 526.
That was true and it was not the whole set. k153 re-ran the mutation and
attributes **four** tests to arm 3: those two, plus
`add_refuses_a_parent_that_names_nothing_in_the_tree` (413) and
`insert_errors_when_target_missing` (1,159), both in the excluded
`task_grow/tests.rs`. Arm 4 is held by exactly one,
`add_refuses_an_ambiguous_parent_slug_and_lists_the_keys` (268). So the page's
*table* was incomplete as well as its prose, and chapter 10's clause — which
said arm 3 is **not** pinned in the excluded file — was false rather than merely
long; k153 corrected the row, rewrote chapter 9's paragraph, narrowed the
concept-index entry and shortened chapter 10 to a cross-reference.

**Three lessons for every remaining chapter, and the second is the one k146
missed.** *Locate the test, do not recognise the name* — still true. But
**locating the names a page already carries tells you where those tests live and
nothing about whether they are the whole set**; only the mutation enumerates, so
a citation-placement defect is re-derived by re-running, never by chasing the
names in front of you. And **the mutation must be a panic, not a reworded
`bail!`**: three of arm 3's four observers assert on the message substring and a
rewording would catch them, but `insert_errors_when_target_missing` asserts a
bare `is_err()` and stays green under any message at all.

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

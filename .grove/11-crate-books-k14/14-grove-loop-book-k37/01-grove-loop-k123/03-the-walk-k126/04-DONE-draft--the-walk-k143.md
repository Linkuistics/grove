# the-walk-k143

## Goal

Draft chapter 7 of the `grove-loop` book — *The walk: pick and select*,
`docs/walkthroughs/grove-loop/07-the-walk.md` — and prove the prefix through
slice `first-live-leaf`.

## Context

- The third of `the-walk-k126`'s six chapter children. **Name this leaf by its handle,
  never by its bare slug**: `the-walk` is both this leaf and its parent node
  `the-walk-k126`, so a bare-slug reference is ambiguous and `resolve` refuses it
  listing both keys.
- Its two ownership blocks are `walk-selection` (`task_tree.rs` 571–637) and
  `pick-tests` (1,106–1,360) — 322 lines, of which 255 are tests. **This is the
  most test-heavy chapter of the six.**
- The structure brief's section is *7 · The walk: pick and select*. The rule is
  **the first live leaf in walk order, and position in that walk is the only
  schedule there is**: `Selection`, `pick_in`, `select_in`, `select_in_write`,
  `selected`.
- **Fifteen tests, and the brief names the specific form the claim takes.** The
  prose owes the negative case for each, and the brief singles one out:
  `pick_orders_numerically_not_lexically` passes under a lexical sort too until
  there are ten leaves, so it is `10` against `9` that makes it a test. Count the
  tests in the block before writing *fifteen* — a count claim is the highest-yield
  defect class in this book and the brief's number is the brief's, not the
  block's.
- `pick_refuses_a_species_mismatch_at_a_task_shaped_name` is the spine's sharpest
  case: the store would have accepted the entry, and the grammar that refuses it —
  defined in chapters 2 to 4 — is the thing that did not move.
- **Chapter 1's cast row `Selection`, owned by `first-live-leaf`, closes here**
  and moves from `pending` to `explained`. So does the row for `pick` added at
  `02-the-tokens.md#the-outcome`, whose owner is also `first-live-leaf`.
- **The early-use ledger is a floor**: enumerate this chapter's reproduced bytes
  for later-owned symbols named or exercised, and add the rows they owe.
- The chapter's carried-example row is the brief's row 7: from the tree as
  `root_init` left it, to the first live leaf in walk order.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  first-live-leaf --check all` is valid: 13 files, 3,132 resolved lines, 7,401
  deferred, `final=false`.
- Chapter 7 exists, `README.md`'s contents entry and chapter 6's two navigation
  lines are updated, and both `first-live-leaf` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)

1. **Nineteen, re-enumerated rather than inherited from `pick-test-count-k147`.**
   `#[test]` over `crates/grove-loop/src/task_tree.rs` 1,106–1,360 counts 19 —
   eighteen `pick_*` and the block-opening
   `select_returns_path_handle_and_kind_from_one_guarded_observation`. The
   control is the same command over the whole file (63) and over chapter 6's
   `path-composition-tests` (0), so the instrument was seen to return a non-zero
   and a zero on subjects known to differ. The corrected brief and the block
   agree, so the page writes nineteen with nothing to adjudicate.
2. **The brief's discriminating pair for `pick_orders_numerically_not_lexically`
   is wrong, and the page states the block's.** The brief says the test *passes
   under a lexical sort too until there are ten leaves, so it is `10` against `9`
   that makes it a test*. Under the canonical grammar chapter 4 read, a position
   is zero-padded to **at least two digits**, so 9 renders `09` and 10 renders
   `10`: lexical and numeric agree on that pair and it discriminates nothing. The
   fixture in the block is `100-impl--b-k2.md` against `99-impl--a-k1.md`, and
   the test's own comment says why — the old unpadded `2-…` spelling the point
   used to be made with is what `docs/adr/task-names-are-canonical.md` now
   refuses, and a three-digit ordinal is the discriminating case that survives.
   The page states `100` against `99` and the reason; the brief is corrected by
   its own leaf, on the `structure-brief-dependency-count-k132` and
   `pick-test-count-k147` precedent, because a page must not be reconciled to a
   claim the bytes refute.
3. **One floor row beyond the manifest, found by enumerating identifiers rather
   than by looking for a pattern.** Every identifier in the two owned ranges was
   extracted and classified against the manifest's block owners; the one row the
   manifest does not carry is `reset_read_count`, `read_count`, whose definitions
   sit at `task_tree.rs` 1,006–1,014 — inside chapter 9's `resolution` block —
   and which chapter 7's one-observation test calls. Chapter 5 had already
   followed `READ_COUNT` forward and named this assertion, so the *static* was
   covered and the *accessors* were not. Two candidates were considered and
   rejected: *the lifecycle verbs* and *the grow verbs* appear in the reproduced
   comments as generic phrases, naming no symbol, so neither triggers the
   specification's *names or exercises* test.
4. **The chapter's superlatives were replaced by structural facts twice**, on the
   rule that a count claim is the highest-yield defect class here. *The largest
   test section in `task_tree.rs`* is false — chapter 8's block is 292 lines and
   chapter 9's 344, against this one's 255 — and became *the most test-heavy of
   the six in Part II by proportion*, with the three ratios given. *No third
   source of scheduling anywhere in the crate* was an unenumerated superlative
   and became the enumeration: eight production `.walk()` calls, of which only
   `selected` lets the walk's order decide the answer, with the other seven
   classified.
5. **One in-session reviewer spent**, on the nineteen pass-while-broken claims —
   the class the validator cannot reach and the one a careless author gets wrong
   — rather than on the fragments, which `book-check` proves byte-exactly. The
   byte checker was itself controlled first: a one-space mutation inside
   `walk-selected` was made and seen to raise `F008` naming the exact source
   byte, then reverted, so the green run is evidence rather than a clean read
   from an instrument never seen to fail.
6. **The reviewer's fourteen findings: ten applied, two already fixed by the
   author's own re-read before it reported, one half-refuted, one refined.**
   Applied: the species and symlink refusals are produced inside `TaskName::parse`
   at classification, so `pick` fails inside `read` and `selected` is never
   reached — the page had said *met in the walk rather than in the parser* and
   contradicted itself thirteen lines later; a do-nothing `selected` fails
   `pick_lenient_on_foreign_files` rather than passing it; three other tests
   observe a refusal's text, so *the only one that observes anything but a path*
   was false; the symlink test builds its tree with `fs::write` and `symlink`
   rather than the chapter 6 fixtures; `loop-crate-verbs-k21` is named in chapter
   6's block and never in chapter 5's; *Part II* is the structure brief's word for
   a session grouping and appears nowhere in the book, so a reader meets it
   undefined; nine groups do not cut ten fragments; *nothing weaker separates the
   two orderings* is false, since any digit-count boundary does. Already fixed:
   the deepest-live-leaf hole and the *largest test section* claim. **Half
   refuted:** the finding that this crate has no bare-parenthetical citation form
   counted only the nineteen fully written `docs/ARCHITECTURE.md#…` citations —
   eight bare parentheticals also exist and five of them, `(pruning)` included,
   resolve to `<a id>` anchors in that document. The sentence was kept and made
   to name both forms.
7. **One finding is a fact about the grammar worth keeping.** `Outcome::strip`
   matches the literal prefixes `DONE-` and `ABANDONED-`, so there is no
   infix-set check at all: `01-UNDONE-impl--a-k1.md` is malformed because
   `UNDONE-impl` fails `Kind::new`, and `01-done-impl--a-k1.md` is a **live** leaf
   whose session kind is `done-impl`. The page had asserted an infix check that
   would have predicted the second as a refusal.
8. **`bash scripts/check.sh` is red on `book-check` alone, as the node brief
   predicts.** Seven of eight green — `cargo fmt`, `shellcheck`, `cargo clippy`,
   `plugin install`, `conformance`, `conformance suite`, `cargo test` — and one
   failing book of six: `grove-loop` under `--final`, wholly in the
   deferred-prefix classes (46 `F003`, 17 `F009`, 14 `M101` for the fourteen
   unwritten pages, 2 `M103`). No `F008`, no `M105`, no link failure.
   `jj-workspace`, `keyed-launch`, `ordinal-fs-tree` and `overview` are
   `final=true`. The run was started after the last edit landed and the book
   directory's digest is identical before and after it — an earlier run begun
   across an edit was stopped and discarded rather than read.

# outcomes-k157

## Goal

Draft chapter 13 of the `grove-loop` book, *Outcomes are marked in place*, over
`tree_lifecycle.rs`'s two `marked-in-place` blocks — `696-1012` and `2235-2725`,
808 lines — and prove the prefix through slice `marked-in-place`.

## Context

- Child 3 of 4 of `no-word-for-k127`. The rule is **done-ness and abandonment are
  marked in the name, and the body is not rewritten**.
- Blocks: `outcomes-in-place` (`696-1012`: `leaf_retire`, `retire_parts`,
  `PruneResult`, `leaf_prune`, `Planned`, `plan_prune`, `plan_subtree`,
  `plan_leaf`, `apply_prune`, `stopped_partway`, `marked_path`) and
  `retire-and-prune-tests` (`2235-2725`), whose four labelled sections are
  *leaf-retire* at `2235`, *lifecycle over untracked leaves (issue #3's root
  cause)* at `2342`, *leaf-prune (pruning)* at `2395` and *leaf-prune on a node:
  bulk arity (pruning)* at `2496`.
- **This chapter's thesis is *the tree's shape is the only state*** — the spine
  the whole brief rejected as the book's, kept as this chapter's and contrasted
  with chapter 16's in a sentence chapter 1 has already written. Read chapter 1
  before writing the opening; do not re-argue it.
- The pairs the brief names: `retire_adds_done_infix_keeping_position_and_key`
  with `retire_does_not_rewrite_the_header_or_body` are what make *in place* mean
  something, and `pruning_a_node_takes_one_guard_per_mark` is the cost the
  atomicity is **not** paid with. That last is a claim about a measurement — a
  guard count — so it is worth exactly the re-run, not the reading.
- **`plan_prune`, `plan_subtree`, `plan_leaf` and `apply_prune` are private
  helpers with many `bail!` arms**, which is the shape that has produced an
  attribution defect in every chapter that has met it so far (`leaf_entry`: five
  of seven arms unobserved; chapter 9's arm 3: four observers where the page
  carried two). Enumerate the arms, mutate each to a **panic** in a workspace
  copy, and diff against an unmutated control run of the same copy; the harness
  `a-grove-begins-k155` used is described in this node's brief. A reworded `bail!`
  is not a mutation.
- `stopped_partway` renders an operator-facing message about partial work; check
  what actually observes it before writing that it is checked.
- The manifest's early-use rows are a floor; enumerate this chapter's own bytes,
  in both the Rust-path and the hyphenated-verb spelling.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  marked-in-place --check all` is valid: 13 files, 6,886 resolved lines, 3,647
  deferred, `final=false`.
- `13-outcomes.md` exists, `README.md` and chapter 12's navigation are updated,
  and the two `marked-in-place` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf.

## Decisions (running log)

1. **Drafted chapter 13 in one session rather than decomposing.** 808 lines over
   two blocks, against the 775 `leaf-to-node-k156` took in one and the 612
   `a-grove-begins-k155` took in one. The slice is valid at 6,886 resolved lines,
   the two `tree_lifecycle.rs` ownership rows read `resolved`, and
   `bash scripts/check.sh` is **red on `book-check` alone** — 1 of 8, the other
   seven green including `cargo test`. That is the shape `grove-loop-k123`'s brief
   predicts for every child but the last.

2. **Twenty-four refusal and fallback arms mutated, six held by nothing, and the
   six are three different things.** The harness is the node brief's, with all
   three of `leaf-to-node-k156`'s corrections applied: `cargo build -p grove
   --bins` first, so the control is 558 tests with 11 failing rather than 17; the
   **whole macro call** replaced with `panic!("MUTANT-…")` rather than a
   message-preserving panic; and every mutant confirmed to have run all 558, so no
   compile failure read as a clean result. No mutant's newly-failing set named a
   `driver_lease.rs` or `prompt.rs` test, so the flakiness re-run was not owed.
   Unobserved: `leaf_retire`'s grove-root refusal (712); `plan_subtree`'s
   no-contents fallback (861); `plan_leaf`'s missing-triple context (904) and its
   node-as-leaf refusal (911); `stopped_partway`'s *marked nothing* branch (980);
   `marked_path`'s empty-report context (1007).

3. **Two of the six are unreachable, and the mutation could not have shown it.**
   `plan_leaf` has exactly two call sites, 847 and 873, and **both are guarded by a
   `Parts::Leaf` match on the same entry's triple**, so its missing-triple and
   node-as-leaf arms cannot fire through any path in the workspace. Enumerating
   call sites is what separates *unreachable* from *untested*; the mutation reports
   both as zero. Recorded on the page and promoted to the node brief, because
   chapters 14 to 21 will meet the same shape.

4. **The one finding worth an operator's attention is an asymmetry, not an
   absence.** `leaf_retire`'s grove-root refusal is held by nothing while
   `leaf_prune`'s twin has two tests, one per spelling of the root — and the path
   to it needs no flag: `LeafRetireArgs` and `LeafPruneArgs` are the same
   declaration under two names, both normalised through `normalize_leaf_path`, so
   `grove-llm leaf-retire .` typed inside `.grove/` reaches line 712 and nothing
   else. Adjudicated on the page rather than cut as a leaf: the corpus is frozen,
   the missing test is `crates/grove-loop`'s and not a book defect, and a leaf that
   added it would have to land inside the frozen line counts.

5. **`stopped_partway` is observed, and by exactly one test outside this crate's
   inline suite.** The task file asked for this to be checked rather than assumed.
   `a_prune_that_stops_partway_names_what_it_already_marked`
   (`crates/grove-loop/tests/verbs.rs`) induces a mid-run failure with a read-only
   directory at the **second** position, and it is the sole observer of both the
   `.context(stopped_partway(…))` call and the function's singular-plural branch.
   Its *marked nothing* branch is reached by no test at all, and is one fixture
   line away — moving that directory up one leaf.

6. **The guard-count claim was re-derived rather than read.**
   `pruning_a_node_takes_one_guard_per_mark` asserts `read_count() == 3` over three
   live leaves and one `DONE` leaf. `READ_COUNT` is incremented in exactly two
   places, `read_or_vacant` and `open_write`; `reset_read_count()` runs as its own
   statement and `guard(&g)` is evaluated after it, so the three are the test's own
   opening plus two `reopen_write`s, and `apply_prune` spends the planning guard on
   the first mark. The page states that arithmetic rather than the assertion's
   message.

7. **Two early-use rows added under the floor rule, both in the hyphenated verb
   spelling.** The block names `leaf-retire` three times and `leaf-prune` seven,
   including inside the operator instruction `stopped_partway` renders — never as
   Rust paths — so a sweep for identifiers finds neither. `verbs::leaf_retire` and
   `verbs::leaf_prune` are owned by chapter 15 and take index rows;
   the manifest's four `[[early-use]]` entries are mandatory rows rather than the
   set, so neither needed a manifest change. The two existing rows owned by
   `marked-in-place` flipped `pending` → `explained`.

8. **Two chapter attributions in the structure brief are wrong, and
   `structure-brief-chapter-attributions-k163` holds the correction**, inserted at
   position 04 **before** `finishing-k158` because the first of the two is chapter
   14's coverage obligation. The residue map sends `docs/ARCHITECTURE.md`'s
   transition-table marker to chapter 13, and every row of that table is
   `transition_to_current`'s or `materialize_finish`'s. And *the contrast between
   those two is stated once in chapter 1* is not true of `01-orientation.md`, which
   carries the spine, the Part IV/V boundary and one forward pointer to chapter 13
   alone. Chapter 13 states the contrast itself, at `#marked-in-place`, so the book
   is coherent and only the brief is wrong about where. The leaf also carries a
   measured off-by-one: **all thirty-one** cited marker lines are one less than the
   marker, checked by enumerating both documents.

9. **`(pruning)` is a bare `docs/ARCHITECTURE.md` anchor, and it carries less than
   the code leans on it for.** Five comments in this block cite it; the anchor is
   one of three `id`s on *Human authority and completion*, whose only statement
   about pruning is the authority boundary — *abandoning a planned leaf or subtree
   is human judgment*. It does not argue the arity asymmetry, and no record does.
   Adjudicated on the page: the asymmetry's ground is the `PruneResult` doc comment
   and nowhere else, which is a defensible place for it, but a reader following the
   citation for an argument will not find one. Not cut as a leaf — the comment is
   not wrong, and the corpus is frozen.

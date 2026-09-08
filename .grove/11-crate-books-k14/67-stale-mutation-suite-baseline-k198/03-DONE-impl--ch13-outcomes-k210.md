# ch13-outcomes-k210

## Goal

Re-derive `docs/walkthroughs/grove-loop/13-outcomes.md`'s *What the refusals are
worth, measured* under the node brief's control, and make its three `558`
citations and every sentence that rests on that table say what the suite actually
shows.

## Context

- **This chapter's run took the relink step**, like chapter 12's and unlike
  chapter 11's — its control paragraph names `cargo build -p grove --bins` and
  gives the reason. Chapter 12 re-measured nineteen probes and **every one
  reproduced exactly**; expect the same here, and treat a row that moves as the
  finding rather than the expectation.
- **Twenty-four arms**, at `crates/grove-loop/src/tree_lifecycle.rs` lines 712,
  738, 742, 747, 751, 758, 810, 842, 846, 861, 866, 869, 877, 883, 904, 911, 917,
  918, 922, 953, 969, 980, 991 and 1007. Verify line by line that each still names
  its cited arm before running anything: the node brief's *every cited mutation
  line still names its cited arm* was checked across chapters 11, 12 and 13, so a
  count that differs is the harness or a new test, never a drifted line number.
- **Four cells are aggregates, and an aggregate is the highest-yield defect
  class here** (`uniqueness-and-count-claims-need-enumeration`). Rows 9 and 11 are
  each recorded as *fourteen tests*, row 13 as *five tests*, and row 20 as *eight
  tests*. None of the four names its members, so *fourteen* is a claim about a run
  that no reader can check and no later leaf can re-point. **Enumerate them** —
  name the tests in the page or in this leaf's log — and check whether rows 9 and
  11 are the same fourteen or two different fourteens.
- **Several arms are not `bail!` swaps.** Row 10 is a `Ok(())` return on an empty
  node, row 11 a `continue`, row 14 a silent skip, row 15 a `.with_context`, rows
  21 and 24 `.context` on a `Result`, and rows 22 and 23 are two branches of one
  helper (`stopped_partway`). Each needs a mutation that panics only on the arm's
  own path — a `.context` needs a `map_err`, or the types stop agreeing — and a
  `continue` or a silent skip needs a panic in the branch rather than at the head
  of the loop, or it measures reachability instead of observation.
- **Row 6 and row 19 are the same named test on two different arms.**
  `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind` is credited
  with both the `retire` and the `prune` `finish`-reservations. That is a claim
  about two runs, and chapter 12 measured the same test against its own line 622,
  so the three call sites should agree.
- **Six arms are recorded as held by nothing** — rows 1, 10, 15, 16, 22 and 24 —
  and any roll-up counting them is falsified by a cell that moves. Row 1 in
  particular is a grove-root refusal recorded at **nothing**, where chapter 12's
  corresponding row 1 is held by its sweep; the asymmetry is worth stating rather
  than assuming (`mutation-cannot-see-unreachable`).
- **The control paragraph says the eleven are "all of
  `crates/grove-loop/tests/prompt.rs`", and that is true** — the eleventh,
  `the_namespace_is_the_shipped_plugin_entrys_declared_name`, is in that same
  file, at line 491. Chapter 19's split is about *cause*, not file, so this
  sentence needs its number changed and nothing else.

## Done when

- All twenty-four runs are taken against the node brief's control, each confirmed
  to have reported 560, and any mutant whose newly-failing set names a
  `driver_lease.rs` or `prompt.rs` test re-run before it is believed.
- The *Observed by* column states what the runs show, and the four aggregate cells
  (rows 9, 11, 13 and 20) are backed by an enumeration recorded somewhere a later
  leaf can check.
- The three `558` citations — the control sentence at line 1776, the *executed all
  558* check at 1781, and the *removing the line leaves all 558 tests green* claim
  at 1904 — each state the re-derived number. The third is the shape chapter 12
  rephrased rather than renumbered: *all N tests green* was never true of a copy
  carrying eleven pre-existing failures, so say the mutation **reddens nothing**
  against the control.
- Every prose attribution of an arm to a named test, and every roll-up derived
  from the column, states what the runs show.
- Chapter 13's last act cuts chapter 14's leaf (`--kind impl`, slug
  `ch14-finishing`).
- `book-check --final --check all` is green, and `bash scripts/check.sh` is no
  worse than before.

## Notes

**No source change.** If a mutant exposes a defect in the crate, cut a leaf for
it; the corpus freeze is the parent node's rule.

**The control, and the two things that go wrong quietly.** The node brief's
*Pointers* carries the copy recipe; it reproduced exactly at
`ch12-leaf-to-node-k209` — 560 tests, 549 passed, 11 failed, matched set for set —
at about 55 seconds per mutant on a shared `CARGO_TARGET_DIR`. Two traps that both
read as clean results: a mutant that fails to compile prints no per-test lines,
and a mutant mutated at the *function* rather than at the **call site** this verb
uses reddens every verb's tests rather than this chapter's. Chapter 12's
`addressable_key` probe reddens three at the call site and six in place; the page's
claim was about the call site, and only mutating the call site reproduced it.

## Decisions (running log)

**The twenty-four cited lines all still name their cited arms.** Checked line by
line against `crates/grove-loop/src/tree_lifecycle.rs` as it now stands, before
running anything, as the node brief requires. So a count that moves is the
harness or a new test, never a drifted line number.

**The control reproduces exactly.** The node brief's copy recipe, then
`cargo build -p grove --bins`, then
`cargo test --no-fail-fast -p grove-loop -p grove-llm` on a shared
`CARGO_TARGET_DIR`: **560 tests, 549 passed, 11 failed**, and the eleven match
the brief's set name for name — the ten `crates/grove-loop/tests/prompt.rs`
composition tests plus `the_namespace_is_the_shipped_plugin_entrys_declared_name`,
which is in that same file at line 491.

**All twenty-four runs are taken, every one reporting 560 and every one building
clean.** Each mutant is `comm -13 control mutant` over the sorted failure names,
taken after `cargo build -p grove --bins` on the mutated copy. All twenty-four
were additionally pre-checked with `cargo check -p grove-loop --tests` before the
study began, which closes the *a mutant that fails to compile reads exactly like a
clean result* trap ahead of the runs rather than after them.

| row | arm | line | page says | re-measured |
|---:|---|---:|---|---|
| 1 | grove root — retire | 712 | nothing | **nothing** |
| 2 | a charter brief — retire | 738 | three named | same three |
| 3 | a node | 742 | `retire_refuses_a_node_directory` | same, alone |
| 4 | already-`DONE` leaf | 747 | inline + `grove-llm` namesake | same two |
| 5 | `ABANDONED` leaf | 751 | `retire_refuses_an_abandoned_leaf` | same, alone |
| 6 | driver-reserved `finish` — retire | 758 | the agent-side sweep | same, alone |
| 7 | grove root — prune | 810 | two spellings | same two |
| 8 | a charter brief — prune | 842 | two named | same two |
| 9 | dispatch a node to `plan_subtree` | 846 | fourteen tests | **same fourteen** |
| 10 | empty node returns `Ok(())` | 861 | nothing | **nothing** |
| 11 | `continue` past the node's `BRIEF.md` | 866 | fourteen tests | **same fourteen** |
| 12 | recurse into a child node | 869 | two named | same two |
| 13 | collect a `DONE` child | 877 | five tests | **five** |
| 14 | skip an `ABANDONED` child | 883 | `prune_node_is_atomic_…` | same, alone |
| 15 | `.with_context` no ordinal or key | 904 | nothing | **nothing** |
| 16 | node directory passed as a leaf | 911 | nothing | **nothing** |
| 17 | retired (`DONE`) leaf | 917 | `prune_leaf_refuses_an_already_done_leaf` | same, alone |
| 18 | already-`ABANDONED` leaf | 918 | `prune_leaf_refuses_an_already_abandoned_leaf` | same, alone |
| 19 | driver-reserved `finish` — prune | 922 | the agent-side sweep | same, alone |
| 20 | `reopen_write` for later marks | 953 | eight tests | **eight** |
| 21 | `.context(stopped_partway(…))` | 969 | `a_prune_that_stops_partway_…` | same, alone |
| 22 | the *marked nothing* branch | 980 | nothing | **nothing** |
| 23 | the singular *leaf* branch | 991 | `a_prune_that_stops_partway_…` | same, alone |
| 24 | `.context` no rename reported | 1007 | nothing | **nothing** |

**The table reproduces in full — twenty-four for twenty-four.** That is the
finding the node brief predicted for a chapter whose own run took the relink step,
and it agrees with `ch12-leaf-to-node-k209`'s nineteen for nineteen. No cell
moved, so every count derived from the column stands on its arithmetic; what did
move is stated below, and none of it is a count.

**Rows 9 and 11 are the *same* fourteen, name for name.** The node brief asked
whether they were two different fourteens; they are one set, which is what makes
the pair a positive control rather than two measurements:

  1 `a_prune_that_stops_partway_names_what_it_already_marked` ·
  2 `leaf_prune_in_a_colocated_tree_leaves_the_git_index_alone` ·
  3 `leaf_prune_marks_a_whole_subtree_abandoned_in_a_jj_native_tree` ·
  4 `leaf_prune_marks_every_live_leaf_and_names_the_done_ones_it_left` ·
  5 `prune_of_a_node_reminds_once_for_the_whole_bulk_mark` ·
  6 `prune_that_marks_nothing_stays_quiet` ·
  7 `pruning_a_node_marks_every_leaf_the_same_way` ·
  8 `prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address` ·
  9 `prune_node_leaves_done_leaves_untouched` ·
  10 `prune_node_marks_a_subtree_mixing_tracked_and_untracked_leaves` ·
  11 `prune_node_marks_every_live_leaf_in_the_subtree` ·
  12 `prune_node_recurses_into_a_grandchild_node` ·
  13 `prune_node_with_nothing_live_marks_nothing` ·
  14 `pruning_a_node_takes_one_guard_per_mark`.

**Row 13's five** are 4, 6, 9, 13 and 14 of that list. **Row 20's eight** are 1,
2, 3, 5, 7, 10, 11 and 14. Both are subsets of the fourteen, which is why no arm
in this block reaches an observer the node dispatch does not.

**No re-run was owed, and none was concealed.** No mutant's newly-failing set
names a `driver_lease.rs` or `prompt.rs` test — the ten out-of-module observers
live in `crates/grove-loop/tests/verbs.rs`, `crates/grove-llm/tests/leaf_ops.rs`,
`crates/grove-llm/tests/jj_tree_verbs.rs` and
`crates/grove-llm/tests/session_kind_tree.rs` — and `ps` showed no orphaned
`configured-command.sh` children.

1. **All three `558` citations become 560, and the third is rephrased rather than
   renumbered.** The control sentence at line 1776 and the *executed all 558*
   check at 1781 are counts and take 560 directly. The claim at 1904 said removing
   the line *leaves all 558 tests green*, which was never true of a copy carrying
   eleven pre-existing failures; it now says the mutation **reddens nothing**
   against the 560-test control — the shape `ch12-leaf-to-node-k209` settled for
   the same sentence.
2. **The control paragraph's cause was joint and only true of ten.** It read *11
   failing, all of `crates/grove-loop/tests/prompt.rs`, because the copy is not a
   jj repository*. All eleven are indeed in that one file — that half is right and
   the node brief's re-confirmation stands — but the run's own stderr splits the
   cause: **exactly ten** panic on `Refusal(NotAWorkspace { … })`, and
   `the_namespace_is_the_shipped_plugin_entrys_declared_name` panics at
   `prompt.rs:494` on `.claude-plugin/marketplace.json` being `NotFound`. One
   reason carrying eleven items is load-bearing for ten of them
   (`joint-justifications-split`). Chapters 11 and 12 both already name **two**
   causes in their own control paragraphs — *neither a jj repository nor a
   checkout of the plugin marketplace*, and *not a jj repository and does not
   carry the marketplace manifest* — so this page was the odd one out among three
   siblings describing one copy. The file claim and the eleven are untouched; the
   second cause is added, which is the smallest edit that makes the sentence true
   and puts chapter 13 in step with its neighbours. This changes nothing chapter
   19 adjudicates: the split is already on chapter 19's page, and this states it
   rather than re-deriving against a ten.
3. **The four aggregate cells are enumerated on the page, not only here.** Rows 9,
   11, 13 and 20 said *fourteen tests*, *fourteen tests*, *five tests* and *eight
   tests*, and none named a member — so *fourteen* was a claim about a run no
   reader could check and no later leaf could re-point
   (`a-cited-test-list-is-a-measurement`). The counts are all correct, so nothing
   is renumbered; an enumeration block after the table names the fourteen once and
   gives rows 13 and 20 as index subsets of it. That also makes the page's own
   next roll-up checkable for the first time: *all eighteen distinct
   `tree_lifecycle::tests::` names above* counted thirteen visible names plus five
   hidden inside the aggregates, and with the members written down the eighteen
   can be counted off the page.
4. **Two arms are held from outside the crate, not four — and the sentence that
   said four was falsifiable by the run that was never taken.** The page's *Four
   arms are held only from outside the crate, which is worth naming because
   `cargo test -p grove-loop` alone would report them unobserved* names rows 6,
   19, 21 and 23. Rows 6 and 19 are held by
   `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind` in
   `crates/grove-llm/tests/session_kind_tree.rs` — genuinely another crate. Rows
   21 and 23 are held by `a_prune_that_stops_partway_names_what_it_already_marked`
   in **`crates/grove-loop/tests/verbs.rs`**, which is an integration test of *this
   crate* and which `cargo test -p grove-loop` runs. The page named the file
   correctly in the very next sentence and then drew the wrong boundary from it.
   Measured rather than argued, because the claim is about a command: a
   `cargo test --no-fail-fast -p grove-loop` control (330 tests, 11 failing) plus
   that one package re-run on each of the four mutants shows rows **21 and 23
   reddening `a_prune_that_stops_partway_names_what_it_already_marked`** and rows
   **6 and 19 reddening nothing**. So four arms are held only from outside this
   module's inline tests, and two of the four from outside the crate — and it is
   those two the package-scoped command misses. The distinction is the paragraph's
   whole point, so the repair is the boundary, not the count.
5. **Every other roll-up on the page survives, checked against the runs rather
   than re-read.** *Eighteen of the twenty-four arms are observed and six are not*
   — observed are rows 2–9, 11–14 and 17–21 and 23, eighteen; unobserved are rows
   1, 10, 15, 16, 22 and 24, six. *All eighteen distinct `tree_lifecycle::tests::`
   names above sit between lines 2235 and 2725* — the eighteen enumerate, and they
   run from `retire_refuses_a_node_directory` at 2276 to
   `prune_refuses_the_grove_root_given_as_a_relative_dot_path` at 2718. *Fourteen
   of this chapter's thirty-two tests observe no arm here at all* — the inline
   module holds exactly thirty-two `#[test]` functions between 2238 and 2718, and
   thirty-two minus the eighteen observers is fourteen.
6. **The reachability argument for rows 15 and 16 is unchanged and still is not
   the mutation's.** `plan_leaf` still has exactly two call sites — lines **847**
   and **873** — and both still sit under a `Parts::Leaf` match on the same
   entry's triple, so the page's line numbers are current. Row 1's absence is
   likewise re-confirmed by enumeration and not by the zero:
   `retire_refuses_the_grove_root` occurs nowhere under `crates/`, while the same
   grep for `prune_refuses_the_grove_root` finds its class — the cross-tree
   control that separates *absent* from *a broken search*
   (`mutation-cannot-see-unreachable`).
7. **No source defect surfaced, so no defect leaf is owed.** Every mutant that
   reddened nothing did so for a reason the enumeration already gives — two
   unreachable arms, one broken-library contract, and three genuine gaps the page
   already names and argues. The corpus freeze held: no file under `crates/` was
   changed in this leaf.
8. **The suite holds exactly one duplicated test name, and it is chapter 14's
   problem rather than this one's.** Reading a mutant as `comm -13` over *bare*
   test names silently merges two tests that share a name in different binaries.
   Enumerated rather than assumed: the control's 560 test lines carry **559**
   distinct bare names, and the single duplicate is
   `finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, which exists
   in both `crates/grove-llm/tests/finish_commit.rs` and
   `crates/grove-loop/tests/verbs.rs`. It appears in **no** newly-failing set in
   this study — each of this chapter's ten out-of-module observers resolves to
   exactly one file — so chapter 13's twenty-four readings are unaffected. It is
   `14-finishing.md`'s arm 12, whose cell is *3* and names that very test in both
   files, so an unqualified reader there would report 2 and call it a moved cell.
   Passed to `ch14-finishing-k211` in its own Context.
9. **Both gates are green.** `book-check --repo . --book docs/walkthroughs/grove-loop
   --final --check all` reports *valid: 13 files, 10557 resolved lines, 0 deferred
   lines, final=true*, and `bash scripts/check.sh` exits 0 with *all 8 principal
   checks pass* over six books, 0 failing — no worse than before, since this leaf
   changed no source and no fragment range.
10. **Chapter 14's leaf is cut** — `04-impl--ch14-finishing-k211.md`, `--kind impl`,
    slug `ch14-finishing` — and its Context carries the two things this study
    learned that it cannot re-derive cheaply: chapter 11's *ten reached, two
    observing* verdict on the `RootShape::ATree` arm its row 2 restates, and the
    binary-qualification hazard on its row 12.

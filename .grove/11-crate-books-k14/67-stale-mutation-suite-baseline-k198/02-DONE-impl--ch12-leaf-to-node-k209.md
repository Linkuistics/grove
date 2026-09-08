# ch12-leaf-to-node-k209

## Goal

Re-derive `docs/walkthroughs/grove-loop/12-leaf-to-node.md`'s *What the refusals
are worth, measured* under the node brief's control, and make its four `558`
citations and every sentence that rests on that table say what the suite actually
shows.

## Context

- **This chapter's run already took the relink step**, unlike chapter 11's — its
  own procedure paragraph names `cargo build -p grove --bins` and gives the
  reason (six `grove-llm` tests otherwise fail on a missing binary, and a control
  wrong in that direction hides observers). So expect the table to hold, and
  treat a row that moves as the finding rather than the expectation.
  `ch11-a-grove-begins-k208` re-measured eleven probes and nine reproduced
  exactly; the two that moved were the two the `grove` binary could reach.
- **Nineteen runs.** Sixteen arms at `crates/grove-loop/src/tree_lifecycle.rs`
  lines 550, 560, 576, 586, 587, 602, 606, 611, 615, 622, 652, 659, 666, 675, 681
  and 692; the two arms outside the block the page measures for attribution
  (`task_tree::target`, credited with `decompose_refuses_a_foreign_file` alone,
  and `task_tree::addressable_key`, credited with the twin, destination and
  interrupted-promotion tests); and the whole-line deletion of 560, which the page
  reports as leaving all 558 green. Every one of those sixteen lines still names
  its cited arm — checked line by line — so a count that differs is the harness or
  a new test, never a drifted line number.
- **Two arms need more than a `bail!` swap.** Line 692 is a `.context` on a
  `Result`, so it needs a `map_err` that panics only on the error path or the
  types stop agreeing. Line 560 is a `refuse_finish_kind` **call site** and must
  panic on the refusal — `if …refuse_finish_kind(…).is_err() { panic!("MUTANT") }`
  — not on entry, or it measures reachability instead of observation. Chapter 11's
  row 1 was mutated exactly that way and came back at nothing, which is the page's
  own claim about its sibling call site.
- **The sweep is the load-bearing test and the page says so.**
  `no_promotion_refusal_reaches_an_operator_from_an_ordinary_argument` is credited
  as the only observer of rows 1 and 22, and as a second observer of four more.
  *Six arms reddening one test* is a claim about six runs
  (`uniqueness-and-count-claims-need-enumeration`), so re-derive it from the runs
  rather than from the sentence.
- **Three roll-ups sit on the table**: *ten of sixteen arms are held by nothing,
  and nine of the ten are one class*; *rows 1, 3 and 6 to 10 are the control*; and
  the contrast paragraph that counts `refuse_finish_kind`'s four call sites across
  chapters 10, 11 and 12. Each is derived from the column, so each is falsified by
  a cell that moves.

## Done when

- All nineteen runs are taken against the node brief's control, each confirmed to
  have reported 560, and any mutant whose newly-failing set names a
  `driver_lease.rs` or `prompt.rs` test re-run before it is believed.
- The *Observed by* column, the three roll-ups above, and every prose attribution
  of an arm to a named test state what the runs show.
- The four `558` citations — at the *conservative `return Ok(())`* paragraph, the
  control paragraph, the *executed all 558* check, and the *removing line 560
  outright* claim — each state the re-derived number.
- Chapter 12's last act cuts chapter 13's leaf (`--kind impl`, slug
  `ch13-outcomes`).
- `book-check --final --check all` is green, and `bash scripts/check.sh` is no
  worse than before.

## Notes

**No source change.** If a mutant exposes a defect in the crate, cut a leaf for
it; the corpus freeze is the parent node's rule.

## Decisions (running log)

**All nineteen runs are taken, every one reporting 560.** Control: 560 tests, 549
passed, 11 failed — the eleven being ten `crates/grove-loop/tests/prompt.rs`
composition tests plus `the_namespace_is_the_shipped_plugin_entrys_declared_name`,
which is in that same file. Matched set for set, not count for count. Each mutant
is `comm -13 control mutant` over the sorted failure names, taken after
`cargo build -p grove --bins` on the mutated copy.

| row | arm | line | page says | re-measured |
|---:|---|---:|---|---|
| 1 | grove root | 550 | the sweep | same, alone |
| 2 | `refuse_finish_kind` override | 560 | nothing | **nothing** |
| 3 | `promote` refused | 576 | `a_tree_at_the_last_key_…` | same, alone |
| 4 | `reopen_write` failed | 586 | nothing | **nothing** |
| 5 | `append_brief_suffix_in_file` failed | 587 | nothing | **nothing** |
| 6 | a charter brief | 602 | three named | same three |
| 7 | a node | 606 | two named | same two |
| 8 | a retired leaf | 611 | three named | same three |
| 9 | an abandoned leaf | 615 | two named | same two |
| 10 | driver-reserved `finish` | 622 | two named | same two |
| 11–16 | `promoted`'s six | 652–692 | nothing | **nothing**, all six |
| — | `task_tree::target` call site | 549 | `decompose_refuses_a_foreign_file` | same, alone |
| — | `task_tree::addressable_key` call site | 562 | twin, destination, interrupted | same three |
| — | whole-line deletion of 560 | 560 | all green | reddens nothing |

**The table reproduces in full — nineteen for nineteen.** That is the finding the
node brief predicted for this chapter: its own procedure paragraph already names
`cargo build -p grove --bins`, so unlike chapter 11 nothing here was measured
through an unrelinked binary, and no cell moved.

1. **All four `558` citations become 560, and two of them are rephrased rather
   than renumbered.** *The control is 558 tests with 11 failing* and *executed all
   558* are counts and take 560 directly. The other two said a mutation *leaves
   all 558 tests green*, which was never true of a copy carrying eleven
   pre-existing failures; they now say the mutation **reddens nothing** against
   the 560-test control, which is what the run shows and what the sentence was
   always reaching for.
2. **The sweep's arithmetic was right and its distribution was wrong.** The page
   claimed the sweep is *the only observer of two* arms — line 550 and the
   `finish`-reservation on line 622 — and *a second observer of the four*
   `decomposable` refusals. Measured, it is the only observer of **one** (line
   550) and a second observer of **five**: line 622 is a `decomposable` arm like
   the other four, and it is held by
   `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind` as well.
   Two plus four and one plus five are both six, so *Six arms reddening one test*
   survives untouched — the count was never the defect
   (`joint-justifications-split`). The page's own table already credited line 622
   to two tests, so this paragraph contradicted the table beside it; the likely
   history is that it predates the whole-macro re-run described three paragraphs
   above it, which is what *found* that second observer.
3. **Nine of sixteen arms are held by nothing, not ten.** Observed arms are rows
   1, 3 and 6 to 10 — seven — so nine are unobserved, and the very next paragraph
   already said *the other nine*. The page contradicted itself in adjacent
   sentences. The sub-count moved with it: rows 11–16 and rows 4–5 are eight, not
   nine, and *nine of the ten are one class* became *eight of the nine fall into
   two classes*, which is what the two sentences beneath it actually describe
   (`uniqueness-and-count-claims-need-enumeration`).
4. **The two out-of-block attributions hold only at the call site, and the page
   now says so.** Mutating `addressable_key`'s duplicate-key block in place
   reddens **six** tests, not three — the three named plus
   `insert_refuses_a_target_whose_key_names_two_entries`,
   `retiring_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed` and
   `prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address` — and `target`'s
   walk-exhausted arm reddens four. Mutating each *call site* inside
   `leaf_decompose` reproduces the page's numbers exactly. The prose's own
   wording elsewhere (*replacing that call's result*) already meant the call site,
   so the repair is the qualifier *through this verb* plus the function-level
   numbers, not a new count.
5. **Every exclusivity claim in the prose survives.** `decompose_refuses_a_brief`
   reddens under row 6's mutant and no other of the twenty-two runs;
   `decompose_refuses_a_node_directory` only row 7's; `decompose_refuses_a_done_leaf`
   only row 8's; `decompose_refuses_an_abandoned_leaf` only row 9's;
   `decompose_refuses_a_foreign_file` only under `target`. Checked by scanning all
   twenty-two failure sets, not by re-reading the sentence.
6. **`refuse_finish_kind`'s four call sites enumerate.** `task_grow.rs:104`
   (`leaf-add`), `:170` (`leaf-insert`), `tree_lifecycle.rs:346` (`root-init`) and
   `:560` — so the contrast paragraph's *four places* is exact, and chapter 11's
   call site being unobserved is what `ch11-a-grove-begins-k208` measured for row
   1. No edit needed.
7. **No re-run was owed.** The node brief requires a second run of any mutant
   whose newly-failing set names a `driver_lease.rs` or `prompt.rs` test. None of
   the twenty-two sets does, and `ps` showed no orphaned `configured-command.sh`
   children before or during the study.

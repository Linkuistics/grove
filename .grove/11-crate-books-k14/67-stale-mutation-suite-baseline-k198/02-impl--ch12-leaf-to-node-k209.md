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

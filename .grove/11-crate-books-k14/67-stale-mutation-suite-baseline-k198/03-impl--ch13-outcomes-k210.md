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

# ch14-finishing-k211

## Goal

Re-derive `docs/walkthroughs/grove-loop/14-finishing.md`'s *What the refusals are
worth, measured* under the node brief's control, and make its two `558` citations
and every sentence that rests on that fifteen-arm table say what the suite
actually shows. Its table restates four of chapter 11's arms from the other side,
so it must agree with `ch11-a-grove-begins-k208` **test for test**.

## Context

- **One row is already known to be stale, and it is not a total.** Row 2 is
  `transition_to_current`'s `RootShape::ATree` arm — chapter 11's row 6, the same
  mutation from the other side of the file inversion. `ch11-a-grove-begins-k208`
  measured it at **ten reddened, two observing**, against the page's *Newly
  failing: 2*: eight of the ten are out-of-process fixtures in
  `crates/grove-loop/tests/driver_lease.rs` and
  `crates/grove-llm/tests/removed_surface.rs` that drive the loop against a
  worktree already holding a grove, so they die in the fixture on any panic in the
  driver's pre-launch path and observe nothing about this arm. Chapter 11 chose to
  report the distinction — *reached* versus *observing* — rather than a new
  number, because the page already draws exactly that line for its entry probe.
  **Adopt the same distinction here** and say so, or the two chapters state
  different things about one mutation.
- **The rest of the table should hold, and a row that moves is the finding.**
  This chapter's procedure paragraph already names the relink step — *build the
  `grove` binary first so `CARGO_BIN_EXE_grove` is set* — so like chapters 12 and
  13 nothing here was measured through an unrelinked binary. Chapter 12
  reproduced nineteen for nineteen and chapter 13 twenty-four for twenty-four.
- **There is exactly one duplicated test name in the whole suite, and it is
  *this* chapter's row 12.** Reading a mutant as `comm -13` over **bare** test
  names merges two tests that share a name in different binaries. Measured at
  `ch13-outcomes-k210`: the 560-test control carries **559** distinct bare names,
  and the single duplicate is
  `finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, present in
  both `crates/grove-llm/tests/finish_commit.rs` and
  `crates/grove-loop/tests/verbs.rs` — which the page's row 12 cell names in
  **both** files and counts as part of its *3*. Qualify every failure name by its
  binary before diffing, or row 12 reads 2 and looks like a moved cell when
  nothing moved. It bit no earlier chapter: it appears in no newly-failing set of
  chapters 11, 12 or 13.
- **Six cells are zeros, and a zero means untested *or* dead**
  (`mutation-cannot-see-unreachable`). Rows 5, 6, 8, 10, 13 and 14. The page
  already argues the split — two unreachable (5, 6), three needing an environment
  failure no fixture builds (8, 13, 14), and row 10 — so the re-derivation owes
  each zero a re-measurement, not a re-reading, and owes the *arguments* a check
  that they still describe the code.
- **Five arms are held only from outside the crate, and here that is literally
  true.** Rows 7, 9, 11, 12 and 15 are `finish_commit`, `delete_and_commit` and
  `require_recoverable_grove`, and the page says not one observer is an inline
  test — they live in `crates/grove-llm/tests/finish_commit.rs`, which runs the
  binary and reads its stderr. Row 12's second file is `grove-loop`'s own
  `tests/verbs.rs`, so *outside the crate* is right for four of the five and
  wrong for row 12; `ch13-outcomes-k210` hit the same over-claim on its own page
  and repaired the boundary rather than the count.
- **Two roll-ups sit on the table**: *Nine arms are held and six are not*, and
  *four of the eight arms with observers belong to `transition_to_current`, and
  of the five tests holding them, three sit in chapter 11's block and two in this
  one*. Both are derived from the column, so both are falsified by a cell that
  moves — and *nine held* versus *eight with observers* in adjacent paragraphs is
  worth checking against the runs rather than against each other.

## Done when

- All fifteen runs are taken against the node brief's control, each confirmed to
  have reported 560, each read as a `comm -13` over **binary-qualified** failure
  names, and any mutant whose newly-failing set names a `driver_lease.rs` or
  `prompt.rs` test re-run before it is believed.
- The *Newly failing* and *Where the observer lives* columns state what the runs
  show, row 2 carries chapter 11's reached-versus-observing distinction in the
  same terms chapter 11 used, and both roll-ups above agree with the column.
- The two `558` citations — the *control is not zero* bullet and the *Every
  mutant below ran 558 tests* check — each state the re-derived number. The first
  also gives one cause for eleven failures (*the scratch copy is not a jj
  repository*) where the run's own stderr shows ten on `NotAWorkspace` and the
  eleventh on the missing `.claude-plugin/marketplace.json`; chapters 11, 12 and
  13 all name both causes.
- Every prose attribution of an arm to a named test — including the *`Taskless`
  reddens one, `Unrecognised` reddens one, `ATree` reddens two* summary about two
  hundred lines earlier in the chapter — states what the runs show.
- Chapter 14's last act cuts the next leaf: chapters 15 and 16 travel together
  (`--kind impl`, slug `ch15-16-configured-command`), per the node brief's
  decomposition.
- `book-check --final --check all` is green, and `bash scripts/check.sh` is no
  worse than before.

## Notes

**No source change.** If a mutant exposes a defect in the crate, cut a leaf for
it; the corpus freeze is the parent node's rule.

**The control, reproduced three times now.** The node brief's *Pointers* carries
the copy recipe and it has reproduced exactly at `ch11-a-grove-begins-k208`,
`ch12-leaf-to-node-k209` and `ch13-outcomes-k210` — 560 tests, 549 passed, 11
failed, matched set for set, about 55 seconds per mutant on a shared
`CARGO_TARGET_DIR`. Two traps that both read as clean results: a mutant that fails
to compile prints no per-test lines, and a mutant mutated at the *function* rather
than at the call site a verb uses reddens every verb's tests. Pre-checking all
mutants with `cargo check -p <crate> --tests` before the study closes the first
trap ahead of the runs rather than after them; `ch13-outcomes-k210` did that and
it cost one pass.

## Decisions (running log)

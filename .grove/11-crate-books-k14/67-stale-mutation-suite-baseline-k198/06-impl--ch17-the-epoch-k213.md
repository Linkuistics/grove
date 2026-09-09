# ch17-the-epoch-k213

## Goal

Re-derive `docs/walkthroughs/grove-loop/17-the-epoch.md`'s admission-ladder
study — eight rungs, one control citation at line 994 — under the node brief's
control, and make that citation and every sentence resting on the table say what
the suite actually shows.

## Context

- **One citation, and it carries two of the three numbers.** Line 994: *The
  control is 558 tests, 547 passing, with eleven `tests/prompt.rs` failures that
  are environmental: the copy is not a jj repository.* The re-derived control is
  **560 tests, 549 passed, 11 failed**, so both numbers move — and the single
  cause is the same over-claim chapters 11–16 all repaired: the run's stderr
  shows **ten** on `Refusal(NotAWorkspace { … })` and
  `the_namespace_is_the_shipped_plugin_entrys_declared_name` on
  `.claude-plugin/marketplace.json` being `NotFound`
  (`joint-justifications-split`). With this page repaired, all eight chapters
  name both causes.
- **This is the chapter where an unrelinked run lies hardest, and the page
  already knows it.** Its channel-mismatch row credits
  `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` and
  `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` — both
  shell out to a real binary — and the page's own closing paragraph argues that a
  message-preserving mutation would have shown the rung held by one test instead
  of three. `cargo build -p grove --bins` goes **after** each edit
  (`mutant-must-be-relinked-into-the-driver-binary`): `testing/support.rs:434`'s
  `workspace_binary` reuses `target/<profile>/grove` if the file merely exists,
  and `cargo test -p grove-loop -p grove-llm` never rebuilds the `grove` package.
  A `driver_lease.rs` or `removed_surface.rs` name appearing in a newly-failing
  set is the signal the step mattered.
- **The shared context string is a wrapper trap, stated on the page as a
  weakness of the tests and equally a hazard for the study.** Six rungs plus the
  whole probe — *seven sites in all* — are wrapped in `stale Grove session for
  {operation}`, so an observer asserting on that substring may be crediting the
  wrapper rather than the arm (`an-observer-may-assert-on-a-wrapper`). The
  mutation must be a `panic!` with **no message**, whole macro call, or the
  out-of-process observers cannot tell a panic from the refusal it replaced.
- **Four of the eight rows are zeros, and the page has already done the harder
  half of the work** (`mutation-cannot-see-unreachable`): it separates *reachable
  and unheld* (probe replaced 8×) from *unreachable* (probe retries exhausted,
  where the last iteration always returns or bails), and argues the other two —
  *identity changed* and *lease record differs* — as asymmetries beside pinned
  neighbours. Each zero owes a re-measurement rather than a re-reading, and each
  argument owes a check that it still describes the code.
- **Every non-zero cell is a cited test list, so each is a measurement**
  (`a-cited-test-list-is-a-measurement`). Re-derive the members, not only the
  counts — a new observer joins a column as easily as a total.
- **Several roll-ups sit on the table and on the ladder paragraph, and none is
  derived from a run.** *The last seven tests all call `admit_session`*; *seven
  sites in all* share the context; *the only refusal on the path that is not so
  wrapped is the working-tree mismatch*; *the file has three of them, one after
  each bounded-retry loop, all in chapter 16's half*; and the channel row's
  *three observers*, *two out-of-process*. Enumerate each against
  `crates/grove-loop/src/driver_lease.rs` as it now stands
  (`uniqueness-and-count-claims-need-enumeration`); the counting is cheap and it
  is the highest-yield defect class in this node.
- **Binary qualification is cheap insurance and may finally bite here.** The
  560-test control carries **559** distinct bare names, and the single duplicate,
  `finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, lives in both
  `crates/grove-llm/tests/finish_commit.rs` and `crates/grove-loop/tests/verbs.rs`.
  It appeared in no newly-failing set at chapters 11–16, but this is the first
  chapter whose rows credit out-of-process `grove-llm` tests by design.

## Done when

- All eight rungs are run against the node brief's control, each confirmed to
  have reported 560, each read as a `comm -13` over **binary-qualified** failure
  names, and any mutant whose newly-failing set names a `driver_lease.rs` or
  `prompt.rs` test re-run before it is believed.
- Line 994's citation states the re-derived total **and** passing count, and
  names both causes of the eleven.
- The *Observed by* column states what the runs show, member by member, and every
  roll-up above is enumerated against the source and agrees with the column.
- Every zero is re-measured, and the reachable-versus-unreachable split the page
  draws is re-checked against the code rather than re-read — including the claim
  that the trailing `bail!` after the bounded-retry loop is unreachable because
  the last iteration always returns or bails, and that the file holds three such
  arms.
- This leaf's last act cuts chapter 19's (`--kind impl`, slug `ch19-shared-baseline`
  or as that chapter's scope dictates), per the node brief's decomposition —
  the shared-baseline paragraph that speaks for all seven chapters, the
  ten-versus-eleven adjudication, one citation, and the mechanical-check decision
  with `ledger-rollup-check-k207` named. It is the last child, and it must agree
  with what the seven now say.
- `book-check --final --check all` is green, and `bash scripts/check.sh` is no
  worse than before.

## Notes

**No source change.** If a mutant exposes a defect in the crate, cut a leaf for
it; the corpus freeze is the parent node's rule.

**The control, reproduced five times now.** The node brief's *Pointers* carries
the copy recipe and it has reproduced exactly at `ch11-a-grove-begins-k208`,
`ch12-leaf-to-node-k209`, `ch13-outcomes-k210`, `ch14-finishing-k211` and
`ch15-16-configured-command-k212` — 560 tests, 549 passed, 11 failed, matched set
for set, about 55 seconds per mutant on a shared `CARGO_TARGET_DIR`. Pre-check
every mutant with `cargo check -p grove-loop --tests` before the study; it closes
the fails-to-compile trap ahead of the runs rather than after them, and it cost
one pass at k212.

**Measure with `GROVE_SIGNAL_FILE` unset in the measuring shell.** This
repository is a meta-grove and the session running the study inherits a live
signal path; the scratch copy carries `.cargo/`, so the force-clear travels with
it, but the shell should not carry the value in.

**A `bail!` is replaced whole** — `panic!("MUTANT")` with no message. A
`.context`/`.with_context` arm needs a `map_err` that panics only on the error
path, or the types stop agreeing.

## Decisions (running log)

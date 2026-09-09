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

**The control reproduces exactly, for the sixth time.** The node brief's copy
recipe (with `CLAUDE.md` and a plain `AGENTS.md` copy), then
`cargo build -p grove --bins`, then `cargo test --no-fail-fast -p grove-loop -p
grove-llm` on a shared `CARGO_TARGET_DIR`, with `GROVE_SIGNAL_FILE` unset in the
measuring shell: **560 tests, 549 passed, 11 failed**, the eleven matching the
brief's set name for name — all in `crates/grove-loop/tests/prompt.rs`. The run's
own stderr splits the cause as predicted: **ten** panic on
`Refusal(NotAWorkspace { … })` and
`the_namespace_is_the_shipped_plugin_entrys_declared_name` on the marketplace
manifest being `NotFound` (`Os { code: 2, kind: NotFound }`). The 560 test lines
carry **559** distinct bare names, the single duplicate being
`finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, which passes in
both `grove-llm/tests/finish_commit.rs` and `grove-loop/tests/verbs.rs`.

**Binary qualification is in the instrument from the first reading.** Every
failure name is recorded as `<test target>@<test name>`, paired by index between
cargo's stderr `Running`/`Doc-tests` lines (30 + 2) and stdout's 32 `running N
tests` blocks, so the pairing is race-free rather than dependent on stream
interleaving. Every mutant is read as `comm -13 control mutant` over that sorted
qualified set.

**Index pairing is not race-free after all, and this chapter is where it breaks.**
The `m3_inactive` mutant reported **561** test lines, not 560. The cause is not a
lost or gained test: `crates/grove-loop`'s fork-sensitive lease tests re-exec the
unit-test binary with a name filter, and under this mutation that child panics
and cargo's stdout gains a **nested** block — `running 1 test` … `test result:
FAILED. 0 passed; 1 failed; … 245 filtered out` — *inside* the parent's `running
246 tests` block. Pairing stdout's `running N tests` blocks against stderr's
`Running`/`Doc-tests` lines by index then shifts every later target label by one,
and `m3`'s newly-failing set read as ten `session_config@` names that were really
`prompt.rs`'s eleven environmental failures relabelled. The recorded claim that
the pairing is "race-free rather than dependent on stream interleaving" was true
of the control and of chapters 11–16, whose runs emitted **no** child block; it is
not a property of the instrument.

**The parser is now nesting-aware, and the roster is the control that catches
this.** `parse2.py` keeps a stack: a `running N tests` block that closes while
another is still open is a re-exec'd child and is discarded, and only outermost
blocks consume a target label. Every run is then checked against a stronger
invariant than the total — **the set of `<target>@<name>` pairs must be identical
to the control's**, with only verdicts free to differ. A relabelled run fails that
check even when it totals 560, which a count alone cannot. Control, `m1` and `m2`
re-parse unchanged (32 top-level blocks, 0 children discarded), so their readings
stand; `m3` re-parses to 560 with one child discarded.

**Eight rungs, eight mutants, all eight reporting 560 with a roster identical to
the control's.** Read as `comm -13` over binary-qualified failure names:

| # | Rung | Newly failing | Members |
|---|---|---:|---|
| 1 | working tree differs | 1 | `grove_loop@driver_lease::tests::ambient_context_from_another_worktree_names_both_roots` |
| 2 | identity changed | 0 | — |
| 3 | epoch inactive | 3 | `grove_loop@driver_lease::tests::an_inactive_epoch_is_reported_without_claiming_a_session_is_active`; `grove_loop@driver_lease::tests::an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls`; `grove_loop@task_grow::tests::leaf_insert_lints_cross_references_under_a_shared_opening_of_its_own` |
| 4 | channel differs | 3 | `grove_loop@driver_lease::tests::a_rotated_epoch_refuses_the_old_signal_path`; `driver_lease@a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`; `driver_lease@grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` |
| 5 | lease record differs | 0 | — |
| 6 | lease unlocked | 2 | `grove_loop@driver_lease::tests::a_successful_liveness_probe_releases_the_lease_before_validation`; `grove_loop@driver_lease::tests::an_active_epoch_without_a_live_lease_is_stale` |
| 7 | probe replaced 8× | 0 | — |
| 8 | probe retries exhausted | 0 | — |

**The page's original run was relinked, and row 4 is the proof.** Its two
out-of-process members live in `crates/grove-loop/tests/driver_lease.rs` and
reproduce here, which an unrelinked run could not have produced — so unlike
chapter 11's, this chapter's table is not a lower bound. Seven of the eight rows
re-measure exactly as printed. The single change is row 3, where a third observer
appears; rows 1, 2, 5, 6, 7 and 8 hold member for member, and row 4's "three
observers, two out-of-process" holds name for name.

**Row 3's third name is a flake, and the isolation control settles it in both
directions.** `task_grow::tests::leaf_insert_lints_cross_references_under_a_shared_opening_of_its_own`
newly failed under `m3`, `m3b`, `m4b` and `m6b` — and not under `m4` or `m6`, the
first runs of the same two mutations. It fails on
`task_grow/tests.rs:1676`, `exclusive_lock_is_free(worktree.path())`, and never
on the mutated line: `admit_session` has exactly one production caller
(`driver_lease.rs:720`), which takes the ambient path cargo force-clears to
empty, so *no* in-process test outside `driver_lease`'s own module can reach any
rung. Run alone under `m3` it passes. Run alone under `m3`, the two rung
observers both fail, and both pass alone under the control — the positive control
that the mutation is doing the work. So the arm holds **two** observers, and the
third name is a sibling's re-exec'd subprocess perturbing a lock probe.

**The generalisation for the node.** The brief warns that a `driver_lease.rs` or
`prompt.rs` timeout reads exactly like a newly attributed observer. This is the
same hazard one step wider: a *cross-test* flake in any binary reads like one
too, and the cheap discriminator is not a second full run but running the
candidate alone under the mutant.

**Every row of the page's table re-measures exactly as printed.** 1 · 0 · 2 · 3 ·
0 · 2 · 0 · 0, member for member, including row 4's "three observers, two
out-of-process" — which is also the evidence that this chapter's original run was
relinked. The page's numbers needed no correction; its control citation and one
roll-up did.

**Every roll-up on the page enumerated against `driver_lease.rs` as it now
stands.** Four hold and one is false.

- *Seven sites share the `stale Grove session for {operation}` context.* Holds.
  The file has eight occurrences of the phrase; the eighth is a test assertion at
  1379. The seven production sites are 754 (control-directory parent), 760
  (shared-lock acquisition), 762 (record parse), 780 (identity), 783 (inactive),
  787 (channel) and 791 (the probe's wrapper) — six rungs plus the whole probe,
  exactly as the page says, and the later paragraph's list of seven names the
  same set.
- *The only refusal on the path not so wrapped is the working-tree mismatch.*
  Holds under the page's own operative sense of "refusal" — a `bail!` arm, which
  is what the study mutates. Of admit_session's four `bail!`s (765, 780, 783,
  787) only 765 is unwrapped, and the probe's four (679, 688, 691, 702) are all
  under 791's wrapper. Two *error* paths are also unwrapped — `Workspace::resolve`
  at 751 and the identity `fs::metadata` at 772 — but neither is a rung, and
  reading them in would make the sentence say something the table does not.
- *Three trailing `bail!`s, one after each bounded-retry loop, all in chapter
  16's half.* Holds exactly. The loops are 376, 451 and 648; the trailing arms are
  443, 497 and 702; the manifest gives chapter 16 lines 1–819 and chapter 17
  820–1383, so all three are chapter 16's. All three have the same shape as the
  one under discussion — the last iteration returns or bails — so the deadness
  generalises rather than being special to the probe.
- *The channel row's three observers, two out-of-process.* Holds name for name;
  both out-of-process members are in `crates/grove-loop/tests/driver_lease.rs`.
  The column now names them rather than counting them.
- **False: *the last seven tests all call `admit_session`*.** Six do.
  `a_successful_liveness_probe_releases_the_lease_before_validation` (1305–1346)
  calls `probe_live_lease_with_post_unlock_hook` and contains no `admit_session`
  call — which the page's *own* commentary on that test states two hundred lines
  later. Repaired to "six call it directly, and the seventh drives the liveness
  probe that sits underneath its last rung".

**The two reachability arguments re-checked against the code, not re-read.**
*Probe replaced 8×* is reachable: the hook fires at 666, immediately before the
identity comparison at 686, on the `probe == 0` path, so a fixture that leaves the
lease unlocked and replaces the path on every pass reaches 679. *Probe retries
exhausted* is unreachable: every path through the loop body returns, bails, or
`continue`s, and `continue` on the last attempt bails at 679 first, so 702 is
dead. Both arguments stand as written.

**Chapter 19's leaf is cut: `ch19-shared-baseline-k214`, `--kind impl`, position
07 under this node.** It is the node's last child. Its body carries the one
`558` citation at `19-the-core.md:123`, the *seven of them — chapters 11 to 17*
clause as a cross-chapter claim to check rather than carry, the
ten-versus-eleven adjudication with the node brief's ruling that the seven were
measured against the eleven, three uncounted enumerations (including one about
chapter 10's own measurement), the forward reference to chapter 17's relink step
that only lands now that k213 wrote the step down, and the mechanical-check
decision with `ledger-rollup-check-k207` named — together with why this baseline
is *not* the ledger-derived class k207 owns, since nothing in the repository
holds the number.

**The flake's mechanism is left unsettled on the page, deliberately.** The
tempting account — a sibling's re-exec'd subprocess outliving the lock probe,
which is the fork sensitivity two of the eighteen tests already run in a
subprocess to contain — explains `m3` and `m6b`, both of which emit a nested
child block. It does not explain `m4b`, which emits none. What is measured is
enough to keep the name out of the table (it cannot reach a rung; it passes alone
under the mutant; the two credited observers fail alone under it and pass alone
under the control), and asserting a mechanism beyond that would add an unchecked
claim to the one page whose subject is which claims are checked.

**`bash scripts/check.sh` is green — all eight principal checks, six books,
exit 0 — and the subjects were byte-frozen across the whole run.** Two earlier
runs were started and stopped because a prose edit landed while they were
reading; the run that counts was taken after every edit, with an md5 of the
chapter and of every task file in this node taken before and after and compared.
`book-check --final --check all` reports `docs/walkthroughs/grove-loop` valid at
13 files / 10,557 resolved lines.

**One sweep finding, and it is a false positive worth handing forward.** No
`558` or `547` survives on this page; the same command still finds
`19-the-core.md:123` (k214's) and `11-a-grove-begins.md:176`. The second is
**correct as written** and must not be "repaired": it reads *exactly one test of
the 558 the suite **then** held*, and the next paragraph opens *That reading is
superseded* and gives 560. It is a named record of a past run, frozen by its own
stated rule (`counts-split-current-state-from-record-of-a-run`), not a stale
current-state count. k214's sweep clause says to classify each hit, and this is
the hit it will have to classify.

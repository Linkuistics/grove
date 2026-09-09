# driver-lease-fixture-timing-k85

## Goal

Make two fixtures in `crates/grove-loop/tests/driver_lease.rs` —
`a_second_driver_refuses_before_tree_access_or_launch` and
`a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` — pass
reliably under `cargo test --locked --workspace`, the form `scripts/check.sh`
runs them in.

## Context

- Observed at `the-surface-k78`, a documentation leaf whose diff touches nothing
  these fixtures read. Under the workspace-wide run both panicked in
  `testing/support.rs`'s `wait_for_ready` with *nothing wrote first-ready: the
  process behind it is still running after 120s*. Rerun alone, each passes — in
  117s and 103s. The whole `grove-loop` test crate passes alone, and so does
  `crates/grove/tests/loop_driver.rs`, whose own run under the same load had
  one fixture fail and another never return.
- The shape is a readiness deadline of 120 seconds guarding work that needs
  over 100 seconds on an idle machine. Under the parallel load of every crate's
  tests at once it overruns. Whether the 100 seconds is inherent to the fixture
  — a driver waiting out a real lease or epoch interval — or is itself a
  sleep-and-poll that could be event-driven is the first thing to establish;
  raising the deadline is the fallback, not the fix.
- The first sign was a stalled background run of `scripts/check.sh` that sat
  for 2h40m with no output and no child process after `loop_driver.rs`'s
  `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal` stopped
  reporting. That fixture passes alone in 38s. A test that can wedge without a
  child to wait on is the same family of problem and belongs in this leaf's
  reading.
- **Reproduced at `forward-commitment-tests-k131`, and one hypothesis measured
  out.** A `scripts/check.sh` run wedged for over an hour in that same fixture,
  with the `loop_driver` binary at 0% CPU and no children. The obvious suspect
  was `GROVE_SIGNAL_FILE`, which is set in every session's environment; it is
  **not** implicated. Run alone with the variable pointed at a scratch path, all
  11 `loop_driver` fixtures pass in 41.5s and nothing writes the file; run alone
  with it unset they pass in 39s; run under full workspace parallelism with it
  unset, `an_orphaned_epoch_guard_...` and
  `a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver`
  both fail in 104s. Load is the whole variable, as this leaf already says. A
  later full `scripts/check.sh` on an idle machine passed `cargo test`
  outright — so the wedge is intermittent under load, not a hard stop, and a
  green run is not evidence the deadline is adequate.

## Done when

- The two named fixtures pass under `cargo test --locked --workspace` on three
  consecutive runs, and the reason each needed over 100 seconds is stated in
  the fixture or removed.
- `bash scripts/check.sh` passes.

## Decisions (running log)

**The premise is false: neither fixture does a hundred seconds of work.** Measured
on the 16-core development machine at an ambient load of ~4 (four other live
`grove` sessions, none started by this one).
`a_second_driver_refuses_before_tree_access_or_launch` costs **1.62s** alone and
`a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` **3.13s**;
the whole 23-test `driver_lease` binary is **6.02s**. So the "readiness deadline
of 120 seconds guarding work that needs over 100 seconds on an idle machine"
shape this leaf was cut against does not exist, and neither does the case for
raising the deadline. The `## Context` reading of "rerun alone, each passes — in
117s and 103s" was a durations-under-load reading, not a cost.

**Starvation does not reach the backstop either, so load alone is not the whole
variable.** Twenty-four concurrent copies of the `driver_lease` binary — 552
process-driving fixtures at once on 16 cores — left every one of the 24 copies
green, with the binary degrading gracefully 6.02s → 16s (6 copies) → ~60s (24
copies) and no readiness wait failing. A mixed run of `driver_lease`,
`loop_driver`, `lifecycle_cutover`, `composition_verbs` and `finish_commit` at 3
copies each was also green (`loop_driver` stretched to 69s). Across roughly sixty
runs of `driver_lease` the backstop fired **zero** times. Since a starved
producer never approaches 120s, a reading that does reach it is evidence of a
producer that *stopped*, which is what `forward-commitment-tests-k131` already
measured directly: 0% CPU and no children.

**`cargo test --workspace` runs test binaries sequentially, so "the parallel load
of every crate's tests at once" is not what it names.** Confirmed from the run
log's per-binary timestamps: binaries start strictly one after another and
concurrency is the 16 libtest threads *inside* one binary. A whole-workspace run
therefore subjects `driver_lease` to no more concurrency than running that binary
alone does — which is why the binary alone and the workspace run cost the same.
The real multiplier is other `grove` sessions running their own suites at the
same time, and that is external to this repository.

**The job-control hypothesis is excluded, not merely unconfirmed.** A producer
stopped by `SIGTTIN`/`SIGTTOU` matches every symptom — alive, 0% CPU, no
children, forever — because `try_wait` does not pass `WUNTRACED` and reports a
stopped process exactly as it reports a running one, and `keyed_launch::run` does
hand the controlling terminal between sessions. It is still ruled out here: these
sessions have **no controlling terminal** (`open("/dev/tty")` → `ENXIO`,
measured in-session), so `Terminal::open` returns `None` and the handover never
runs. `docs/walkthroughs/grove-loop/20-the-loop.md` reaches the same conclusion
from the other side — the `stty` in `reset_terminal` "never runs under the suite
at all". Two full runs under a deliberately allocated pty (`forkpty`) were also
green, so the exclusion is not an artifact of never having tried it.

**So the ">100s" claim is *removed* rather than stated, and the repair is to the
instrument.** Neither fixture ever asserted anything about its own cost; the
claim lived in the backstop's own failure text, which said the producer "is still
running after 120s, **which is a wedged producer rather than a slow one**". That
clause is unevidenced by construction — the backstop observes only the clock —
and it is what three leaves (`the-surface-k78`, `k131`, this one) read as a
statement about the fixtures. `testing/support.rs` now reports a second
measurement instead of asserting a cause: on expiry it samples the producer's
accrued CPU time twice, two seconds apart, and says whether it moved. Blocked and
starved call for opposite repairs and the clock distinguishes neither.

**The firing condition is deliberately unchanged.** Making the backstop itself
progress-based was considered and rejected: a bug in the sampling could stop it
firing at all, turning a wedge into the permanent version of the 2h40m park this
leaf's `## Context` opens with — a strictly worse failure than the one being
fixed. The sample therefore runs *after* the backstop has already fired, costs a
passing run nothing, and an unreadable sample still fails the wait.

**No `#[test]` was added, and that was a constraint on the design rather than an
oversight.** `.grove/BRIEF.md`'s mutation control — **560 tests, 549 passed, 11
failed**, over `cargo test --no-fail-fast -p grove-loop -p grove-llm` — covers
`crates/grove-loop/tests/driver_lease.rs`, so one new `#[test]` there falsifies
it and the `grove-loop` book's procedure paragraphs (chapters 11–14 and 17) that
publish it. The count reconciles exactly against the tree as it stands today:
230 in `grove-llm/tests/` + 84 in `grove-loop/tests/` + 246 in `grove-loop/src/`
= 560. `20-the-loop.md`'s separate "all 277 green" does **not** cover it and was
checked rather than assumed — that control is `-p grove-loop --lib` plus `-p
grove` over `env_hygiene`, `lifecycle_cutover` and `loop_driver`, none of which
compiles this file. One published control, not two. Both branches of the new
diagnostic were instead watched to fire
out of band, against a scratch consumer and a temporarily shortened backstop,
then fully reverted (`diff` against a pre-probe copy is empty): a blocked
producer (`sleep 999`) reported *burned no CPU over a further 2s (still 0:00.01),
so it is blocked rather than slow*, and a starved one (`while :; do :; done`)
reported *burned CPU over a further 2s (0:04.01 → 0:06.01), so it is being
starved rather than blocked*. `testing/support.rs` is outside the frozen corpus —
which is `crates/*/src/**` plus each crate's `Cargo.toml` — so no ledger row,
fragment or line count moves with this change.

**What this leaf does not claim.** The flake was not reproduced, so the fix is to
what the next occurrence can be diagnosed *with*, not a demonstrated repair of
the wedge itself. If it recurs, the failure now says in one run which of the two
causes it was — the measurement `k78` and `k131` both lacked.

# flaky-surface-snapshot-lock-test-k219

## Goal

Establish whether `grove-loop`'s
`task_grow::tests::surface_scans_one_snapshot_under_a_shared_lock` is
intermittently red, and make it deterministic if so — or record why the one
observed failure was not the test's fault.

## Context

Observed once at `jj-workspace-brief-early-use-drift-k215`, in a `bash
scripts/check.sh` run, as the only failure in the workspace:

    test task_grow::tests::surface_scans_one_snapshot_under_a_shared_lock ... FAILED
    test result: FAILED. 245 passed; 1 failed; 0 ignored; 0 measured

**The evidence is thin and its thinness is the point.** It did not reproduce in
any subsequent run: once alone (`cargo test -p grove-loop --lib
surface_scans_one_snapshot_under_a_shared_lock`), three times as
`cargo test -p grove-loop --lib` (246 passed each), and four times as
`cargo test --workspace`, plus a second green `check.sh`. So one failure in
roughly seven full-suite runs, never reproduced.

**The assertion message was not captured**, which is the first thing to fix if it
recurs — the observing session filtered `cargo test` output to result lines and
lost the panic. Do not assume the failure mode from the test's name.

**k215's own changes cannot plausibly explain it.** That session touched three
Markdown documents and two `.grove/` task files and no Rust, and the test passed
seven times afterwards over the same tree. The name points at concurrency — a
shared lock and a single snapshot — which is the usual source of an
order-dependent or timing-dependent red.

## Done when

- The test's behaviour is characterised: either a reproduction (with the panic
  message) and a fix that makes it deterministic, or a recorded argument that the
  observed failure came from outside the test, with the evidence for that.
- `bash scripts/check.sh` passes.

## Notes

**A single unreproduced failure is not yet a defect**, and this leaf is entitled to
conclude that nothing is wrong — but not by re-running it a few more times and
finding green, which is what the observing session already did seven times. Read
what the test actually shares with its neighbours: `mod tests` in
`crates/grove-loop/src/task_grow/tests.rs` is excluded from the book corpus as an
inline test module, so it is not frozen and may be edited freely.

**Prior art in this tree.** A suite hang here has previously been a grandchild
process holding a piped stderr and wedging `wait_with_output`; the remedy was
draining by process group and using files plus `Drop` in test code. A timing red
under a shared lock is a different failure, but the same neighbourhood.

**Watch the instrument.** If a reproduction attempt runs the test in a loop,
finish every edit before measuring and confirm the run reported the full test
count — a build failure prints no per-test lines and reads exactly like a clean
result.

## Decisions (running log)

1. **The failure was not fd exhaustion.** The cheapest load-dependent
   explanation — `probe`'s `File::open(…).expect(…)` or `stale_cross_refs`'s
   `read_to_string` hitting `EMFILE` under a full-workspace run — is dead:
   `kern.maxfilesperproc` is 2,000,000 on this machine and the soft `ulimit -n`
   is unlimited. A run would have to leak six orders of magnitude more
   descriptors than the suite opens.
2. **The test binary is the same shape as the one that failed.** The observed
   line read `245 passed; 1 failed`; the binary today reports 246 tests, so the
   failing run and this one contain the same set and nothing has been added or
   removed since. The observation is comparable.
3. **Nothing in `grove-loop` or `ordinal-fs-tree` mutates process-global
   state.** No `set_var`, no `remove_var`, no `set_current_dir` in either
   crate's sources; `READ_COUNT` is `thread_local!`. So the classic
   order-dependent red — one test's global write seen by another — is not
   available here.
4. **No test in the workspace shares a filesystem path with another.** Every
   fixture is a fresh `TempDir`; the only `/tmp` literals in the tree
   (`crates/grove-llm/tests/complete.rs`, `crates/grove-loop/tests/verbs.rs`)
   are path *values* compared for equality and never opened.
5. **Reproduced, with the panic message.** Running the built lib binary 120
   times at `--test-threads=32`, eight processes at a time, reddened it 4 times
   in 64 completed runs (~6%, against the ~1-in-7 first observed). A second and
   third batch added 2 in 48 and 2 in 80. **Every failure is an
   `assert!(…_is_free(…))` in the positive direction** — line 1444 of
   `tests.rs`, *nothing is held once the scan returns*, and line 1676 of
   `leaf_insert_lints_cross_references_under_a_shared_opening_of_its_own`,
   *and that opening is gone by the time the hits are in hand*. The negative
   assertions never failed, and cannot.
6. **The lock is genuinely held: `errno` is 35, `EWOULDBLOCK`.** So it is not a
   signal interrupting a non-blocking `flock` — the failure mode
   `ordinal-fs-tree`'s own `lock::take` loops against. Instrumenting the probe
   settled that in one batch.
7. **And the holder is invisible.** `lsof` on the directory at the moment of
   failure lists exactly one descriptor, `4r` with no lock flag — the probe's
   own still-in-scope `File`. A holder that exists during the `flock` call and
   is gone milliseconds later is a forked child that has not yet `exec`ed.
   (A `/dev/fd` scan reported *no* descriptor where `lsof` found one: on macOS
   `stat("/dev/fd/N")` reports the fdesc node, not the target. That instrument
   was silently false-clean and was discarded.)
8. **Mechanism confirmed by controlled experiment, with a control that came
   back clean and arms that came back dirty in proportion.** A standalone
   program takes `LOCK_SH` on a private directory, drops it, and immediately
   probes `LOCK_EX | LOCK_NB`, while *N* threads spawn `/usr/bin/true` in a
   loop. Spurious `EWOULDBLOCK` per 20,000 rounds: **N=0 → 0, N=4 → 189,
   N=8 → 445.** `flock` attaches to the open file description and `fork`
   duplicates every one, so a sibling test spawning `jj` copies this test's tree
   guard into a child that keeps the lock alive until `exec` closes it under
   `O_CLOEXEC`. The suite's fixtures spawn `jj` constantly.
9. **So the instrument is wrong, not the property.** *The lock is free* is not a
   claim a process can make about an instant. *This process holds no descriptor
   on the directory* is — a forked child's copy is invisible to it — and it is
   the stronger claim, since a lock needs a descriptor. Both positive assertions
   now ask that, via a new `descriptors_held_on` helper; the two negative
   assertions keep the `flock` probe, which the confounder cannot perturb.
10. **The scan's bound is controlled, not trusted.** `getdtablesize` reports the
    unlimited soft `RLIMIT_NOFILE` here, so the scan is bounded by a constant;
    each caller first asserts a *non-zero* count on the same directory, so a
    descriptor past the bound reddens the test instead of reading as a clean
    tree. Watched fail: with the bound set to 0, both tests go red on that
    control — and the first draft, which borrowed the lock probe as its control
    instead of opening a sentinel, stayed green, which is why it has one of its
    own.
11. **Both new assertions have teeth.** Holding a live `shared()` guard across
    each one reddens both, at the new assertion, in one run.
12. **Chapter 17 already raised this and left it open**, and this leaf closes it:
    *the obvious candidate is a sibling's re-exec'd subprocess outliving the
    probe … so the mechanism is not settled here*. It is now, and the run that
    emitted no re-exec block is what names it — any spawn forks.
13. **The pages this change falsifies, and the ones it does not.** The edit
    lengthens `src/task_grow/tests.rs` from 1,680 to 1,784 lines. That file is
    the `grove-loop` book's one declared corpus **exclusion**, so no fragment
    range moves and the book's declared corpus stays 10,533 — the excluded
    file's lines cancel in the arithmetic. Five present-tense citations of its
    length are repaired: `10-growing.md` (twice), `01-orientation.md`,
    `09-resolve.md`, and the manifest's `[[corpus.exclude]] reason`. Left alone:
    `docs/specs/grove-loop-book-structure.md` and `.grove/BRIEF.md`, both of
    which state the corpus **as frozen** and say so — the same reason their
    12,154 and 59 already differ from the tree. Test count is unchanged at 62
    in this file, 246 in the binary, so no total moves.
14. **Chapter 17's open question is closed rather than merely repaired.** Its
    account of the k198 run stays as the record it is — the assertion it names
    is marked as the one standing when the table was taken — and a new paragraph
    settles what that section said was unsettled, including the detail that
    defeated the original guess: it is not a re-exec'd grove that matters but
    *any* spawn, which is why one mutation broke the test in a run that emitted
    no re-exec block at all.
15. **The fix holds under the conditions that broke it.** 96 runs of the shipped
    binary at `--test-threads=32`, eight processes at a time: 96 green, 246
    tests reported in every one, against 8 reds in 240 runs of the same shape
    before. The count was checked per run, not just the totals.
16. **`bash scripts/check.sh` passes all 8 principal checks**, 1,154 tests
    passed and 0 failed — the same total as the pre-change baseline run, which is
    the control on *no test was added or lost*. `book-check` reports six books
    valid and `final=true`, `grove-loop` among them at 10,557 resolved lines —
    which is also the evidence for decision 13's split: the validator reads live
    bytes, while the spec's 10,533 is the corpus as frozen, so repairing that
    record would have been the wrong edit. One clippy fix was needed on the way:
    the `dev_t`/`ino_t` casts are redundant on macOS and load-bearing on Linux,
    so they carry a scoped `allow` and a comment saying which is which.

# driver-test-timeout-path-unbounded-k194

## Goal

Make the driver tests' own timeout paths terminate. A fixture that misses its
deadline must fail with its diagnostic, not hang forever collecting it.

## Context

- **The observed hang, diagnosed at `canonicalisation-sites-k149`.**
  `bash scripts/check.sh` stopped for thirteen minutes inside
  `cargo test --locked --workspace` with no output and no CPU. `sample` on the
  test binary named the thread:
  `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal`, in
  `crates/grove/tests/loop_driver.rs`, blocked at line 1009 in `read()` under
  `std::process::Child::wait_with_output`.
- **The mechanism, and it is not timing.** The fixture waits up to 25 s for
  `epoch_held` to appear (`let setup_deadline = Instant::now() +
  Duration::from_secs(25)`). On a missed deadline it takes the failure path:
  `child.kill().unwrap()` and then `child.wait_with_output().unwrap()`, to put
  the driver's stderr into the panic message. `wait_with_output` reads the pipe
  to **EOF**, and EOF requires every writer to close. The driver had already
  launched a session — `/bin/sh <tmpdir>/configured-command.sh …` — which
  inherited the piped stderr, was reparented to launchd when its parent died,
  and holds the write end open indefinitely. `lsof` confirmed it: pid 17690,
  PPID 1, holding the same pipe the test was reading, while both of the test's
  direct children were already zombies. **Killing the child is not enough when a
  grandchild holds the pipe.**
- **Load is the trigger, not the fault.** The same suite had passed in full
  minutes earlier, on the same bytes. The re-run coincided with three other
  grove sessions on the machine and a load average of 7.79, which is what pushed
  the fixture past 25 s. A timeout that becomes an infinite hang under load is a
  worse instrument than no timeout, because it cannot be told from slowness.
- **Scope is unknown and must be enumerated, not assumed.** The one hang seen was
  in `crates/grove/tests/loop_driver.rs`, but `wait_with_output` after a `kill`
  is a pattern, and the driver tests spawn sessions in several places.
  `crates/grove-loop/tests/driver_lease.rs` is the other obvious candidate.
- **The machine is carrying dozens of orphans of the same shape, and that is the
  more alarming half.** `ps -eo pid,ppid,args | awk '$2==1'` at the time of the
  diagnosis found **24** `/bin/sh <tmpdir>/configured-command.sh` processes
  reparented to launchd, with elapsed times from 18 hours to **8 days**, plus two
  `sh -c while : ; do sleep 0.05 ; done` fixtures. Count them again at the time of
  the fix rather than trusting this number. Two cautions before treating all of
  them as debris: some may belong to grove sessions that are genuinely still
  running, and telling those apart from leaked ones is part of this leaf's work;
  and the accumulation is self-reinforcing, because idle orphans are load, and it
  was load that pushed the fixture past its 25 s deadline in the first place.
  Whatever fix lands should stop leaking them, and should say how an operator
  clears the ones already there.
- **Prior art in this tree.** `driver-lease-fixture-timing-k85` recorded this
  family as *timing flakes*. That framing is what let it survive: this instance
  is not a flaky assertion but an unbounded wait in the reporting path, and
  naming it that way is part of the fix.

## Done when

- Every driver-test failure path that collects a killed child's output is bounded
  — by dropping the pipe handles before waiting, by reading with a deadline, by
  killing the whole process group rather than the direct child, or by not
  reading the pipe at all on the timeout branch. Which of those is this leaf's to
  choose and to record.
- The call sites are **enumerated at the time of the fix** rather than taken from
  this file: every `wait_with_output`, `wait()` and pipe read in
  `crates/grove/tests/` and `crates/grove-loop/tests/` that can run after a
  `kill`, classified by whether a surviving grandchild could hold the pipe.
- The fixture is shown to fail — with its diagnostic, promptly — when its
  deadline is missed. A timeout path that has never been seen to complete is not
  evidence it terminates; force the miss (shorten the deadline, or hold the
  fixture's precondition) and watch it report.
- No `configured-command.sh` or fake-session process survives a completed run.
- `bash scripts/check.sh` passes.

## Notes

**This leaf changes tests, not the frozen corpus.** `crates/grove/tests/` and
`crates/grove-loop/tests/` are evidence rather than book roots (root brief,
*The corpus, exactly*), so no fragment, ledger row or line count moves and the
one-commit rule costs nothing here. Check that still holds at the time of the fix
rather than assuming it.

**Do not weaken the assertion to make the hang go away.** The test is pinning
that an orphaned epoch guard stops before consuming the relaunch signal, which is
a real property of the loop driver. Raising the 25 s deadline, marking the test
`#[ignore]`, or deleting it are all ways to make a green run that proves less
than the red one did.

## Decisions (running log)

**1 · The enumeration, taken at the time of the fix.** Every `wait_with_output`,
`wait()` and pipe read in `crates/grove/tests/` and `crates/grove-loop/tests/`
that can run after a kill, classified by whether a surviving grandchild could
hold the pipe open:

| Site | Shape | Pipe? | Grandchild can hold it |
| --- | --- | --- | --- |
| `loop_driver.rs:157` `run_driver` → `.output()` | reads both pipes to EOF, **no deadline at all** | yes | **yes** — and at its `the_escalation_reaps_the_sessions_descendants` call site the fixture *deliberately* backgrounds `sh -c 'while : ; do sleep 0.05 ; done'`, which inherits the driver's pipes. The exact regression that test exists to catch therefore presents as an infinite hang rather than its named assertion. |
| `loop_driver.rs:274` kill + `wait()` (timeout branch) | status only, no pipe read | n/a | no — already bounded |
| `loop_driver.rs:283` `wait_with_output()` (success branch) | pipes | yes | only if the driver leaked a session; unreached today |
| `loop_driver.rs:607/608` `attacker`/`victim` `wait_with_output()` | pipes, **no deadline at all** | yes | attacker's session is `exec sleep 30`; a driver that failed to reap it hangs the test |
| `loop_driver.rs:666-676` file capture + `support::wait_for_ready` | no pipe | no | **safe — the in-tree precedent**, from `loop-driver-readiness-deadline-k170` |
| `loop_driver.rs:798/799` `bystander.kill()/wait()` | inherits the test's own streams | n/a | no; but the bystander **leaks on any earlier panic** — this is the `sh -c while : ; do sleep 0.05 ; done` shape found reparented to launchd |
| `loop_driver.rs:987` `orphan.wait()` in the lock thread | stderr to a file | no | no |
| `loop_driver.rs:1007-1009` kill + `wait_with_output()` | pipes | yes | **the observed hang** |
| `loop_driver.rs:1028-1033` kill + `wait_with_output()` | pipes | yes | same, on the `stop_deadline` miss branch |
| `grove-loop/tests/driver_lease.rs` — all sites | `DriverProcess::spawn` captures both streams to a **file**; `spawn_producer` and `Holder` use `Stdio::null()` or a file; every kill path is `kill()` + `wait()` with no pipe read, under `Drop` guards | no | no — **already clean, nothing to change** |
| `grove/tests/lifecycle_cutover.rs:124` `run_grove` → `.output()` | pipes, no deadline | yes in principle | no in fact: every one of its eleven `#!/bin/sh` fixtures exits promptly and none backgrounds anything, and no test there kills a child. Out of scope, recorded so the next reader does not re-derive it. |

Nothing else in either directory spawns a child that outlives its statement.

**2 · The fix is to remove the pipe, not to bound the read.** Of the four
options the *Done when* offers, "not reading the pipe at all" is taken at the
root: every driver spawn in `loop_driver.rs` captures stdout and stderr to
**files**, so there is no EOF to wait for and `wait()` returns as soon as the
driver is dead. Reading with a deadline would need a drain thread per stream and
would still truncate the diagnostic; dropping the handles would discard it
entirely. The file also keeps the account a surviving grandchild wrote, which a
dropped pipe cannot. This is not a new idiom — `driver_lease.rs`'s
`DriverProcess` and `loop_driver.rs`'s own `a_sigtermed_driver_stops_and_reaps_its_child`
already do exactly this, for the neighbouring reason recorded at
`driver-lease-readiness-flake-k145`.

**3 · And killing the driver must kill the session group.** The pipe fix alone
still leaks: `Child::kill` signals the driver alone, `keyed-launch` gives every
session its own process group (`run.rs`, `command.process_group(0)`), so a
SIGKILLed driver leaves `configured-command.sh` reparented to pid 1. The direct
children are read from `ps` *before* the signal lands — reparenting erases the
link — and each group leader is signalled with the negative pid. A `Drop` on the
new type does the same for a driver still running when a test panics, which is
what closes the `bystander` leak too.

**4 · The 25 s deadline is not touched.** The leaf's goal is that a missed
deadline *reports*, not that it is missed less often; raising it is named in
*Notes* as a way to prove less. The shared liveness-based seam
(`support::readiness`, no fixed budget, 120 s hang backstop) is the right answer
for a *readiness* wait and already owns line 676, but the 25 s wait here is on a
marker written by the test's own lock thread and its failure branch has cleanup
the panicking seam cannot run.

**5 · The orphan count at the time of the fix is 6, not 24.** `ps -eo
pid,ppid,args | awk '$2==1'` finds one `/bin/sh …/configured-command.sh` (3 h
10 m elapsed), four `sh -c 'while : ; do sleep 0.05 ; done'` (≈2 d 17 h each) and
one stray `target/debug/grove` from a scratch workspace. The four are the
`bystander`/descendant shape from `the_escalation_reaps_the_sessions_descendants`;
the `configured-command.sh` is the timeout-branch shape. An operator clears them
with that same `awk` filter and `kill -9`; there is nothing to preserve, because
a fixture orphan is by construction outside any live grove — a real session's
`configured-command.sh` still has its driver as PPID, which is exactly what
`$2==1` filters on.

**6 · The timeout path was forced and watched, both ways round.** Forcing by
*holding the precondition* — the lock thread's `fs::write(&lock_epoch_held, …)`
suppressed behind an env var, the 25 s deadline untouched — reproduces the
observed shape exactly: driver up, session launched, marker never written.

- **Post-fix:** `FAILED … finished in 25.14s`, with the driver's own account in
  the message (`grove: launching impl with configured "…/configured-command.sh"
  — subject-k1`). The deadline plus 0.14 s.
- **Pre-fix control**, the same forcing against the file as it stood at `@-`:
  still running when a 150 s alarm killed it (`exit=142`), and it left behind
  both the wedged test binary and a `/bin/sh …/configured-command.sh` reparented
  to pid 1 — the leak, reproduced live rather than inferred. Both were cleaned up
  by hand.

A control that terminates the same way on both sides would have shown nothing;
this one is 25 s versus never, and the leak appears on exactly one side.

**7 · No orphan survives a run under the fix.** `ps -eo pid,ppid,etime,args |
awk '$2==1'` before and after the full binary (11 passed, 38.71 s) and after the
forced failure lists the same six pre-existing strays and nothing new — no
`configured-command.sh`, no `sh -c 'while : ; do sleep 0.05 ; done'`, no
`grove-llm` from either run's tmpdir.

**8 · Clearing the debris already on the machine, and what it taught.** All six
strays were confirmed debris before anything was signalled: each had PPID 1, no
live `grove` driver existed anywhere with a real parent except this session's own
(pid 36744, PPID 22678), and the temporary worktrees they name belong to test
runs long since dead. A live grove session is never in this set by construction —
its `configured-command.sh` still has its driver as PPID, which is exactly what
`$2 == 1` excludes.

The recipe, and the trap in it:

    ps -eo pid,ppid,args | awk '$2==1 && (/configured-command\.sh/ || /sleep 0\.05/)'

**A plain `kill -9 <pid>` on that list does not drain it.** Killing a leaked
session leader reparents *its* own background children to pid 1, so a fresh
orphan appears where the old one was — observed here twice in a row. Signal the
group (`kill -9 -<pgid>`, the leader being its own group leader) or loop the
`ppid == 1` filter until it comes back empty. That is the same asymmetry the fix
turns on: the direct child is never the whole session.

Machine now clear: 0 of this family remain, from 6.

**9 · The `Drop` guard was forced too, and it is the half that closes the leak.**
A `panic!` inserted immediately after `wait_for_ready` in
`a_sigtermed_driver_stops_and_reaps_its_child` leaves a live driver and its
`exec sleep 60` session at the moment the test dies.

- **Post-fix:** fails in 0.91 s and leaves nothing — `ps … $2==1` finds zero of
  the family.
- **Pre-fix control**, same panic against `@-`: `target/debug/grove` reparented to
  pid 1 with `sleep 60` still its child. `std`'s `Child::drop` does not kill, so
  every assertion between a spawn and a hand-written teardown was a leak site.

That is the second dirty/clean pair, and it is what justifies `DriverProcess`
carrying a `Drop` rather than only a `kill` on the branches that call it. The
`Reaped` wrapper does the same for the `bystander` in
`the_escalation_reaps_the_sessions_descendants`, whose teardown was its last two
statements.

**10 · The whole gate is green and the freeze is intact.** `bash scripts/check.sh`
— *all 8 principal checks pass*, `book-check` *6 book(s) checked, 0 failing*,
4:15 wall. No book root moved: `crates/grove/tests/` is in no `walkthrough.toml`
corpus (the three manifests that name a `tests/` directory do so in a comment
explaining its *exclusion*). The two book claims that reach into this file still
hold — `a_sigtermed_driver_stops_and_reaps_its_child` and
`the_escalation_reaps_the_sessions_descendants` are still there and still assert
what `overview/03`, `overview/05` and `grove-loop/20` say they do — and the
console transcript in `overview/05-what-the-call-reaches.md` is unfalsified: no
`#[test]` was added or removed, so `loop_driver.rs` still runs **11** and the
crate still has **67** integration tests (7+5+4+17+11+6+12+1+4), both re-counted
against a live run rather than read off the page.

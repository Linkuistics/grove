# driver-exclusivity-review-k15

**Kind:** review-design
**Reviews:** driver-exclusivity-k14
**Producer launch:** {"producer":"driver-exclusivity-k14","session":"driver-exclusivity-k14","generation":"k14","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `driver-exclusivity-k14` and record concrete findings for its integration step.

## Context

- Attempt to disprove the ownership design under concurrent driver starts,
  driver crash/restart, child exec inheritance, stale sessions, and grove
  deletion/recreation in the same working tree.

## Done when

- Findings identify duplicate-launch windows, deadlocks with session tree
  mutations, hidden durable state, generation-reuse failures, and missing Git/jj
  or process-lifecycle tests; no fixes are applied.

## Findings

Reviewed artifact: `docs/adr/one-live-driver-per-working-tree.md`, the new
"Process ownership and session epochs" section of
`docs/specs/config-driven-sessions.md` (`:339-440`), and the `Driver lease` /
`Session epoch` glossary entries (`CONTEXT.md:74-83`), against `.grove/BRIEF.md`,
`docs/adr/promotion-transactions-fail-closed.md`, and the code the design must
land in (`src/loop_driver.rs`, `src/tree_access.rs`, `src/launch.rs`). Cite these
as `driver-exclusivity-review-k15 E<n>`. No fixes applied.

Verdict: the *protocol* is sound where it is stated. The lock order is genuinely
acyclic, the dev/ino key beats path aliasing, kernel release on death really does
make restart ≡ continuation, close-on-exec really does stop an opaque wrapper
leaking ownership, and routing `complete` through the epoch check is what finally
stops an orphan firing a completion signal at the next session. Where it fails is
**the layer below the protocol**: every exclusivity claim rests on a temp-dir path
whose uniqueness the design never establishes (E3, E4), on a per-launch signal
path the code it lands in is *tested to make stable* (E2), and on a nonce whose
entropy is unspecified (E8). Two acquisitions are specified as predicates with no
mechanism and no blocking policy (E5, E6), one of which can park the loop silently
between iterations (E7). E1–E4 are the blocking set.

### Blocking

**E1 — Lease-before-provisioning contradicts the spec's own flow diagram and the
independent-provisioning contract.**
`config-driven-sessions.md:341`: the driver acquires the lease "before
provisioning embedded content, reporting Herdr state, reading configuration, or
inspecting or mutating `.grove/`." The canonical flow at `:38-53` opens with
`-> provision embedded methodology independently of launch policy` (`:40`) and
lists no lease step at all. `CONTEXT.md:365-367` makes the independence explicit
for the neighbouring failure: an invalid config is non-mutating and "global skill
provisioning retains its independent per-invocation contract". Under `:341` a
*contended* invocation provisions nothing, so the same word "independent" now
holds for one failure and not the other, with no stated reason.
This is a live behaviour change, not a wording defect: `provision_all` is called
unconditionally at `src/launch.rs:30` before the loop, and refreshing the skill
dir by re-running `grove` is the documented delivery mechanism
(`CONTEXT.md:7-8` — "on every invocation ... sweeps it out"). It also makes an
unwritable or full temp dir fatal to skill delivery, which nothing today can be.
The same paragraph leaves unstated which invocations take a lease at all:
`--help` and `--version` survive (`:326-327`) and must not be refusable, and the
driver's own `grove-llm --version` skew probe (`:331-337`) runs before the tree
is touched.
*Why it matters*: it is the one ordering choice in the section that changes an
existing, separately-contracted subsystem, and it is stated in one clause of one
paragraph that the spec's own summary flow contradicts.
*Action*: decide and state it in the flow at `:38-53` — either provisioning
precedes the lease (keeping its per-invocation contract, at the cost of two
drivers writing the same skill dir concurrently, which is already idempotent
against a content hash) or it follows it, in which case say so in
`CONTEXT.md:7-8` and give the refusal path a diagnostic that says provisioning
was skipped. Either way, state that `--help`/`--version` acquire nothing.

**E2 — The per-launch unique signal path is asserted as given, but the code it
lands in is tested to guarantee the opposite, and the design never says how
uniqueness is produced.**
`:372-374` makes "that launch's unique absolute `GROVE_SIGNAL_FILE` path" one of
the three fields of the epoch record; `:376-377` — "The signal path changes on
every launch and remains the only loop-control value exported" — and `:413-420`
make it the **sole per-launch discriminator**, the thing that stops an old
session adopting a reused handle. The ADR rests the whole no-durable-generation
argument on it (`one-live-driver-per-working-tree.md:31-35`).
Today the path is `grove-loop-<name>-<hash(canonical worktree)>.signal`
(`src/loop_driver.rs:107-129`), computed **once outside the loop**
(`:160`), reused every iteration, identical across driver restarts — and
`src/loop_driver.rs:1517-1525` is a test asserting exactly that stability, with
`:1514-1516` giving the reason. So the design silently reverses a tested
invariant, and the reversal is not in the removed-surfaces list at `:624-643`.
Two consequences the design owes an answer to. (a) **Uniqueness source**: if the
suffix is derived from pid, time, or an iteration counter, a restarted driver
regenerates a previous launch's path and the epoch's discriminator collides
exactly where the ADR claims it cannot — pid reuse is the failure the ADR rejects
at `:54-57`. (b) **Lifetime**: `:380` replaces "clear any stale signal at the
start of every iteration" (`src/loop_driver.rs:191-192`, crash-robust because the
path is fixed) with "the driver removes the per-launch signal file after use",
which leaks one file per launch on SIGKILL, panic, or power loss into the very
directory E4 is about.
*Action*: specify the uniqueness source (an unpredictable per-launch suffix, not
pid/time/counter), state that this supersedes `signal_file_path`'s stability
contract and retire that test with its reasoning, and say who removes an
abandoned signal file. Add a seam asserting two launches of one driver produce
different paths and that a stale path is never regenerated by a restart.

**E3 — Lease uniqueness silently depends on `TMPDIR`, an environment value this
same design refuses to treat as configuration.**
`:344-347` keys the lease on "the corresponding control file in the OS temporary
directory"; the ADR says the same (`:5-8`). Rust's `env::temp_dir()` reads
`$TMPDIR` on unix. So two drivers on **one** working tree get two different lease
files — and both start — whenever the temp dir differs: two users sharing a
checkout (macOS `TMPDIR` is per-user under `/var/folders`), `sudo`, a login shell
that exports `TMPDIR` versus one that does not, a container or `systemd`
`PrivateTmp` unit bind-mounting the same tree, or a `TMPDIR` set per pane by a
multiplexer. That is a duplicate-launch window reached by ordinary configuration,
not by an attacker, so `:783-784`'s out-of-scope clause (a caller "deliberately
strips or forges its ambient loop-control context") does not cover it.
It also sits badly with the brief, whose whole point is that "user-settable
environment overrides" are removed (`.grove/BRIEF.md:19-22`): the exclusivity
invariant would be the one remaining behaviour an unrelated environment variable
can switch off, invisibly and with no diagnostic.
*Action*: either fix the lease directory independently of `$TMPDIR` (a
`$XDG_RUNTIME_DIR`-then-`/tmp` rule, or a fixed `/tmp/grove` with per-uid
subdirectories, each of which has its own multi-user consequences worth
recording), or state the dependency explicitly as an accepted limit in the ADR's
Considered options with the shapes above named. Add a seam that runs two drivers
on one working tree under differing `TMPDIR` and asserts the chosen behaviour.

**E4 — Unlinking the lock path while it is held produces two "exclusive" owners,
and no revalidation is specified.**
`flock` binds to the inode, not the path. If the lease file is removed while the
driver holds it — a `systemd-tmpfiles` sweep of `/tmp` (default: 10 days),
macOS's periodic clean of `/var/folders/**/T` (3 days), a `rm -rf /tmp/*` habit,
or a user tidying what looks like litter — the next driver's `open` creates a
**new inode**, locks it uncontended, and runs. Two drivers, one working tree, no
error on either side. Grove loops are explicitly long-lived (weeks of sessions),
so the sweep window is reachable in normal use, and E2(b)'s accumulating signal
files make the directory look exactly like litter worth sweeping.
`:359-364` and the ADR at `:36-38` address only *stale bytes* ("meaningful only
while their corresponding kernel lock is held ... overwritten on acquisition"),
which is the wrong hazard: here the bytes are irrelevant and the *file identity*
is what was lost. The same applies to the epoch control file, where the effect is
worse — a replacement driver would take an exclusive guard on a fresh inode and
never wait for the admitted operation the crash-handoff paragraph (`:395-403`)
promises to wait for.
*Action*: specify the standard revalidation — after acquiring the lock, `stat`
the path and compare device/inode against `fstat` of the locked descriptor,
retrying on mismatch — and say what happens when it keeps mismatching. State
whether losing the lease mid-loop is detected at all (today nothing rechecks it
after acquisition). Add seams: unlink the lease file under a live driver and
assert the second driver is still refused; same for the epoch file across a crash
handoff.

### Missing behaviour

**E5 — "verifies that the driver lease is currently locked" is a predicate with
no mechanism, and every `flock` realisation of it can refuse a legitimate
replacement driver.**
`:385-387` requires the agent-side check to verify "that the driver lease is
currently locked", and `:388-390` lists `unlocked` among the refusal reasons — so
this is load-bearing, not decorative: after a driver crashes with an active epoch
record still on disk and before any replacement starts, it is the **only** thing
that catches an orphan. But the only way to observe a `flock` is to attempt one,
and every attempt takes a real lock: a successful `LOCK_SH|LOCK_NB` probe leaves
the prober holding a shared lock on the lease file, which makes the next driver's
`LOCK_EX|LOCK_NB` acquisition fail — a *false* "another driver is running" for as
long as the orphan lives (`:352-354` refuses immediately rather than waiting, so
the operator sees a permanent refusal with no owner to stop).
The probe is also inherently racy in the other direction: the driver can die
between probe and tree access. That direction is benign (nothing else is writing)
and worth saying so, but the design currently states the check as if it were
exact.
*Action*: specify the probe — which lock mode, that the descriptor is closed
immediately on success, and that the result is an advisory liveness hint whose
race window is closed by the exclusive epoch guard rather than by the probe.

**E6 — The epoch guard's blocking policy is unspecified in all four places it is
acquired, including the one where blocking on a lock this process already holds
is a self-deadlock.**
The lease is explicitly nonblocking and its contention behaviour is spelled out
(`:344-357`); the Tree access lock tries nonblocking, prints one waiting
diagnostic, then waits (`:426`, matching `src/tree_access.rs:71-85`). The epoch
guard is acquired four times — exclusively on driver acquisition (`:368-369`),
exclusively before each spawn (`:369-370`), exclusively after each reap
(`:395-396`), and shared per `grove-llm` operation (`:382-384`) — and the design
states a blocking policy for none of them, no waiting diagnostic, and no bound.
`:401-403` says a replacement driver "waits for that admitted operation", which
is right, but silently: an operator watching a driver that appears hung has
nothing to read.
The exclusive acquisitions also need an explicit release-before-reacquire rule.
`flock` on a second descriptor for a file this process already holds **blocks
against itself**, so a guard accidentally held across `:368-369` → `:369-370`
deadlocks the driver against its own lock with no error and no timeout — the one
failure mode in this design that cannot be recovered by restarting the child.
*Action*: state the mode and diagnostic for each of the four acquisitions,
state that each exclusive guard is released before the next is taken, and add a
seam that holds a shared guard while a replacement driver starts and asserts both
the wait and the diagnostic.

**E7 — The driver's post-reap invalidation can stall the loop indefinitely behind
an orphan whose parent it has just SIGKILLed.**
`:395-396`: "After the foreground child is reaped, the driver takes the epoch
file exclusively and marks it inactive before interpreting the completion signal
or starting another iteration." `:382-384` has every agent-side `grove-llm`
operation hold its **shared** guard "until the operation returns".
`src/loop_driver.rs:477`, `:488` and `:490-493` kill `child.id()` only — never
the process group — so an in-flight `grove-llm` (or an MCP server that later runs
one) survives grace → SIGTERM → SIGKILL as an orphan and keeps its shared guard.
`:398-399` treats exactly this survival as intended for the *crash-handoff* case;
what is not covered is the same-driver case, where the surviving grandchild now
blocks the loop's own next step. If that orphan is itself blocked on the Tree
access lock (`:426` waits without bound), the loop parks between iterations with
no output at all: the session ended, the pane is `working`, and nothing says why.
Note this is a stall rather than a deadlock — I checked the cycle and there
isn't one (see *Checked and sound*) — which makes it harder to notice, not
easier.
*Action*: bound or diagnose it. State whether the driver waits, times out, or
proceeds on the strength of the lease it still holds (it does still hold it, and
the invalidation's only purpose is handoff to a *different* driver, so proceeding
may be defensible and should be argued either way). Add the missing seam: an
orphaned tree command outliving its SIGKILLed session must not park the loop.

**E8 — Nonce entropy is unspecified, and a weak nonce reinstates the exact
failure the ADR rejects.**
`:359-360` — "a fresh random driver nonce"; `:385-386` makes it one of the three
fields an agent validates; `:413-420` and `one-live-driver-per-working-tree.md:31-35`
make the nonce plus the signal path the entire replacement for a durable
generation suffix. The ADR rejects PID-based ownership at `:54-57` because "PIDs
are reused" — but if the "random" nonce is seeded from pid, time, or a
non-cryptographic default hasher, it reintroduces exactly that collision at
exactly the same place, and the failure is silent: a stale session validates
against a replacement driver and is admitted to the tree.
*Action*: state the width and source (≥64 bits from the OS entropy source; not
pid-, time-, or address-derived), and state that the nonce is regenerated per
driver process, never per launch (the signal path is the per-launch axis).

**E9 — The refusal path is specified only as an exit code, and it contradicts the
Herdr authority-release contract.**
`:351-354` gives a second driver an immediate nonzero exit naming the canonical
working tree, and `:341` puts the lease *before* "reporting Herdr state". So a
refused driver reports nothing. `CONTEXT.md:537-552` (*Authority release*) says
the driver "deliberately does **not** release on a no-signal stop ..., on a
version-skew stop, or on an error: those report **`blocked`**", the reason being
that screen detection reads a parked grove as `idle`. A refused second driver in
its own pane is precisely an error stop with a human needed, and it now reports
nothing at all, leaving that pane to mis-detect (`CONTEXT.md:607-630`).
*Action*: decide whether Herdr reporting precedes the lease (it touches no tree
state and no configuration, so ordering it first costs nothing) or whether the
refusal path is an explicit, recorded exception to *Authority release*.

### ADR-set and Done-when gaps

**E10 — The ADR carries no Git/jj content, and the option whose rejection *is*
the Git/jj reasoning is missing from Considered options.**
`01-DONE-driver-exclusivity-k14.md:28-29` requires "Git/jj and crash/restart
behavior are explicit". Crash/restart is explicit and good. Git/jj is not: the
ADR never mentions either VCS, and the spec's only Git/jj sentence in this
section is `:347-349` (aliases, worktrees, workspaces), which is filesystem
identity rather than VCS behaviour.
The missing piece is the first alternative any reader proposes — **put the lock
file inside the working tree** — and its rejection is exactly the VCS reasoning
the Done-when asks for: jj snapshots the working copy on every command, so an
in-tree control file would land in a working-copy commit, and in plain Git it
would need ignoring and would still be swept by an agent's `git add -A`. That is
also why the temp dir was chosen at all, and E3/E4 are the price; recording the
trade-off is what makes those two findings resolvable rather than re-litigated.
Related: the ADR's four options do not include "no exclusivity" (the status quo),
so the record does not say what problem was actually observed.
*Action*: add the in-tree-lock option with the jj/Git rejection reason and the
temp-dir consequences, and add the status-quo option. Keep it a rework of the
existing set, not an appended second ADR.

**E11 — The ADR and the spec disagree on who takes the epoch guard exclusively,
and the ADR omits two of the three invalidation points.**
`one-live-driver-per-working-tree.md:21-24`: "A new driver **or the next session**
must take the epoch file exclusively and invalidate the old record before doing
tree work." Sessions never take it exclusively — `:382-384` gives them a
**shared** guard, and a session taking it exclusively would serialise every
agent-side tree command against every other. The ADR also states only the
before-spawn write (`:16-19`), omitting the acquire-time inactive write
(`:366-369`) and the post-reap invalidation (`:395-396`), both of which are what
make crash handoff work. `CONTEXT.md:79-83` has the same omission of the
acquire-time write.
*Why it matters*: the ADR is what survives the grove; a reader reconstructing the
protocol from it alone builds a different, slower one.
*Action*: reword to "a new driver, and the driver before and after every launch",
and enumerate the three write points in both the ADR and the glossary entry.

**E12 — The reuse guarantee is stated unconditionally but holds only for
`grove-llm`-mediated access, and one of its refusal reasons is misdiagnosed.**
`one-live-driver-per-working-tree.md:31-35`: "an old prompt cannot adopt a newly
reused handle." What the protocol actually guarantees is narrower: an old session
cannot *resolve, mutate, or signal through `grove-llm`*. A stale session that had
already resolved its path, or that edits files and commits directly, is untouched
— and after finish deletion plus root re-init, its commits land in the new
grove's history under the old handle's name. The spec's own scoping sentence
(`:391-393`) is about deliberate stripping, which is a different case.
Separately, `:388-390` folds `wrong-worktree` into "one stale-session
diagnostic". The commonest way to hit that is not staleness but a session whose
cwd moved into a nested checkout, and telling that user their session is stale
sends them to restart the loop instead of to `cd`.
*Action*: scope the ADR sentence to `grove-llm`-mediated tree access and say
plainly that ordinary edits and commits by a stale session are not prevented (it
is a real accepted limit, and naming it is cheap). Split the wrong-worktree
diagnostic from the stale-session one.

**E13 — Missing seams, and no way to write half of the ones already listed.**
`:745-752` covers the protocol's happy paths well. Absent:
- the temp-dir failure surface behind E3 and E4 — differing `TMPDIR` between two
  drivers; the lease or epoch file unlinked while held; an unwritable temp dir;
- E7's stall — an orphaned tree command outliving its SIGKILLed session and the
  driver's next iteration;
- E6's contention diagnostics for all four epoch acquisitions, and a guard held
  across a re-acquisition (the self-deadlock);
- E1's ordering — provisioning skipped (or not) on a refused driver, and
  `--help`/`--version` under a live lease;
- E9 — what the refused driver reports to Herdr;
- the stated exemptions at `:390-391`: `grove-llm --version` and
  `report-turn` must still succeed against an inactive or foreign epoch. This is
  the meta-grove's own suite (`CONTEXT.md:45-53`), where an unscrubbed
  `GROVE_SIGNAL_FILE` now turns a merely-misleading inheritance into a hard
  refusal — `tests/support/mod.rs:104` becomes load-bearing for the suite's own
  correctness;
- VCS shapes distinguished by `src/repo.rs` but not by the seam list: a
  **colocated** jj tree (both markers, one root, one lease) and a Git linked
  worktree whose `.git` is a file.
There is also a **testability gap**: `:686-691` has the process-ownership module
hide "filesystem-identity keys, temporary paths, random nonces", while `:641-643`
forbids environment configuration for test injection. Every seam above needs to
place or corrupt a file at a path the module refuses to expose. The module
therefore needs an internal temp-root (and nonce/clock) injection seam, stated as
internal exactly as the tool/clock/grace seams are.
*Action*: add the seams above and give the module an internal injection point in
`:686-691`.

### Checked and sound

Recorded so `driver-exclusivity-integrate-k16` does not re-litigate them.

- **The lock order is genuinely acyclic.** I tried to build a cycle from
  `:405-411` and could not. The driver releases its Tree access read guard before
  the pre-spawn epoch write (`:430-433`), holds no tree guard at the post-reap
  invalidation, and takes the lease before either; agent commands take the shared
  epoch guard strictly before the Tree access lock. E7 is a stall behind an
  orphan, not a deadlock, and E6's is a self-deadlock from a missing
  release-before-reacquire rule, not an ordering defect.
- **dev/ino beats every path alias** (`:344-349`), and holding the root
  descriptor open genuinely pins the identity: the inode cannot be reused by
  another tree while a driver holds it, so the "two trees, one lease" false
  refusal is unreachable in that direction. E4 is the *other* direction — the
  path, not the inode.
- **Kernel release on exit** (`:355-357`, ADR `:10-12`) is the right primitive
  and is what keeps restart ≡ continuation without a PID file or cleanup ritual.
- **Close-on-exec** (`:361-364`) is correct and is already Rust's default for
  `File::open`; it closes the ADR's second considered option (`:45-48`) properly.
- **Routing `complete` through the epoch check** (`:382-384`) is the quiet win:
  it is what stops an orphan of session N writing a completion signal that the
  driver reads as session N+1's, which no amount of signal-path uniqueness alone
  would fix.
- **Rejecting a durable generation suffix** (ADR `:49-53`) is right, and the
  argument holds once E2 and E8 supply the discriminator it assumes.
- **Framing the protocol as workflow consistency rather than authentication**
  (`:391-393`, `:783-784`) is honest and correctly scoped; E3 is not a
  counter-example to it, because differing `TMPDIR` is ordinary configuration
  rather than a caller stripping context.
- The Tree access lock's move from the `.grove/` descriptor
  (`src/tree_access.rs:59-87`) to the working-tree root (`:422-434`) is
  well-argued and is what lets root creation and finish deletion share one
  invariant.

## Notes

Review target: claude / opus-5, against a producer launched on codex / sol-xhigh
per this leaf's `**Producer launch:**` line — materially diverse in both harness
and model. Recorded as a fact, not a gate; the design under review removes the
mechanism that emits it.

No in-session reviewer was materialised: a `review-*` leaf spawns none
(`grove-owns-escalated-review.md`).

E3 and E4 are the two findings a reader is most likely to wave off as
environmental. They are not: both are reached by ordinary use (a second user, a
`sudo`, a container, a ten-day loop), both defeat the exclusivity claim silently
and in the duplicate-launch direction, and neither has any diagnostic. If the
integration step accepts either as a limit rather than fixing it, that acceptance
belongs in the ADR's Considered options beside the in-tree-lock option E10 asks
for, because it is the same trade-off seen from the other side.

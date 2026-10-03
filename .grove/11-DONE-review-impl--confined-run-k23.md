# confined-run-k23

**Reviews:** confined-run-k12
**Creator:** run 477c2f49-6350-4a58-a10b-6cc44f9a9cab

## Goal

Adversarially inspect the committed confinement increment before lifecycle
launch-cutover builds on it. Read the node's contract and the committed source;
find violations or unstated assumptions rather than accepting its producer's
conclusions. This is inspection only.

## Scope and contract

- `docs/specs/standalone-invocations.md`, the area's *Confinement* and *A
  standalone invocation*, and `.grove/10-k12/_confined-run.md` define the result.
- Dispatch's canonical grant refusal protects policy, settings and records,
  including prospective paths, aliases, implicit system reads and grants beneath
  writable roots. Its own executable and selected executable are the exempt
  runtime resources. Check `crates/harness-dispatch/src/confinement.rs` and the
  native backends in `crates/keyed-launch/src/confinement.rs`.
- Selection sees the owner's grants and bounds outside confinement. The
  confined harness gets a minimal environment and fresh run identity, and no
  enclosing launch, exit or display authority. Inspect the dispatch run path
  and `crates/grove/src/standalone.rs` together.
- Grove publishes only after dispatch exit 0, its protected ending file reports
  observed `exit_signal`, its own cancellation is absent, and both jobs and the
  harness group have ended. Test acknowledged failures, surviving writers,
  cancellation at selection/run/publication and nested runs. Check that the
  report cannot be supplied from inside the sandbox and that output reads stay
  attached to the held staging directory.
- The canonical sibling dispatch path in the prompt must remain usable under
  confinement, including a symlinked sibling and Linux's filesystem view.
  `NoninteractiveLaunch` supplies no second completion channel; review the
  optional-channel runner core for changes to ordinary interactive launches.
- The changed `overview`, `keyed-launch` and `grove-llm` books must explain the
  implemented ownership, cancellation and publication contracts. Read their
  worked operations and source-fragment introductions as an editorial axis,
  independently of the validator's byte-equality evidence.

## Evidence to inspect

- Real-command confinement suites in `crates/harness-dispatch/tests/` and
  `crates/grove/tests/standalone.rs`; the runner's confinement, noninteractive,
  process/PTY and wait-order suites.
- `scripts/release-prepare.test.sh` includes a real confined deterministic
  release-notes harness that acknowledges through the prompt's exit command.
- The producer tested on macOS Seatbelt. Linux bubblewrap was not exercised
  there, so do not infer its behavior from the host's passing result.
- Codebase graph access was unavailable because of an incompatible active
  generation; source reads supplied the producer's structural evidence.

## Done when

Findings are anchored to the committed artifact, with the violated contract,
a reproducer or specific reasoning, and severity. If nothing is found, say
which scope was inspected and what platform or evidence limits remain. Follow
the review skill's retirement and integration procedure.

## Findings

Reviewed `confined-run-k12`'s two commits, `dispatch-confines-k13` (`b2e53db7`) and
`standalone-through-dispatch-k14` (`4fc74211`), as the source stands after them,
against the area spec's *Supervision* and *Confinement*, `standalone-invocations.md`
and the node brief. Nothing was built or run; the evidence is the source, the
tests' bodies and the three books' prose. The structural claims hold on the
macOS reading (see "What held"). **None of the seven findings below breaks a
publication, cancellation or secrecy guarantee on the path Grove drives today**;
F1–F3 are hardening and fail-closed-but-misreported cases, F4 is a Linux cost that
nobody could measure, and F5–F7 are coverage and drift. F1, F3 and F4 are
**PLAUSIBLE**: they rest on reading and need reproducing before a fix.

### F1 — P3 (PLAUSIBLE): a read-only runtime grant beneath a writable root is protected by path, and a hard link is another path

**Location:** `crates/keyed-launch/src/confinement.rs:223-242` (the Seatbelt
`(deny file-write* (literal RUNTIME_i))` and ancestor `file-write-unlink` denies);
the test is `crates/harness-dispatch/tests/confinement.rs:154-174`.

The spec says a runtime file's "read-only permission holds even beneath a writable
directory". The profile enforces that by path: an allow for the whole writable
subpath, then a deny on the literal. A hard link made inside the writable root
(`ln credential alias`) names the same inode by a path the deny does not mention,
so `printf x > alias` is a write the profile allows, if Seatbelt permits the
`link` itself (it checks the new name against the writable allow; whether it also
asks the source for `file-write*` is what the integrator has to measure). The only
test writes through the literal name and checks it fails. Linux is not affected:
`--ro-bind` makes a separate read-only mount, and a link across mounts is `EXDEV`.
Reachability today is narrow, because Grove's staged `work` directory never holds
a runtime grant, but dispatch's contract is stated for any caller whose cwd holds
one.

**Reproduce:** the existing test's harness with
`ln credential alias && printf changed > alias; test "$(cat credential)" = original`.
**Corrections:** deny the link as well (`file-link` on the literal, if the operation
exists) or state that the guarantee is for names, not inodes, and refuse a runtime
read that lies beneath a writable root.

### F2 — P3 (CONFIRMED): a confined run accepts an `--ending-file` the harness can write

**Location:** `crates/harness-dispatch/src/confinement.rs:84-94` (the overlap check
covers the writable roots, the reads and the system reads, not the ending file);
`crates/harness-dispatch/src/run.rs:345-376` (`ending_path`) and `:581-589`
(`write_ending_file`).

The ending file is the one channel by which a caller tells a clean `exit_signal` from
a harness that merely exited 0, and its integrity rests on the sandbox not reaching
it. Dispatch checks it only for existence and writes it with `create_new`. Given
`--confine --ending-file ./out/ending.json` with `./out` beneath the cwd, the harness
can create the file before the run ends; dispatch's exclusive create then fails, the
failure is "reported on stderr and changes neither the ending nor the exit", and the
caller reads the harness's document. `grove run` is safe, because
`standalone.rs:83-85` puts the file in `control/`, a sibling of the writable `work`
and in no grant. The hole is in the contract for every other caller, including any
later one that confines. A harness that exits 0 with a forged `exit_signal` document
at that path is read as an acknowledgement.

**Correction:** refuse, in `Confined::prepare`, an ending file whose canonical
directory lies inside a writable grant, with the same `confinement_overlap` code;
or state in the spec's *Confinement* that the ending file must lie outside every grant
and test it.

### F3 — P3 (PLAUSIBLE): a backend that exists but cannot establish the sandbox fails after selection and is recorded as the harness's own exit

**Location:** `crates/keyed-launch/src/confinement.rs:117-119` and `:255-259`
(`confinement_available` only tests that the launcher file exists);
`crates/harness-dispatch/src/confinement.rs:41` (its only use);
`crates/harness-dispatch/src/run.rs:143-190` (the order: prepare, select, commit,
then `run_confined_observed`).

The spec promises that "failing to establish confinement launches nothing" and that
"a missing confinement backend … refuses before selection". Nothing launches, so the
safety half holds. But a bubblewrap that is present and cannot unshare (Ubuntu
24.04's `apparmor_restrict_unprivileged_userns`, a container without user
namespaces), or a `sandbox-exec` inside another sandbox (`sandbox_apply: Operation
not permitted`), passes the check. Selection then runs, which may mean a deciding
agent's cost; the run is committed; the launcher exits 1 or 71; and the end
observation records `executionConfirmation` with `ending: harness_exit` and that code.
The record says a harness ran and exited 71. `grove run` reports "dispatch failed (exit
71)", which reads as the harness's failure.

**Correction:** probe once before selection (run the backend on `/usr/bin/true` with
empty grants) so the failure is a `confinement_unusable` refusal with no run; or keep
the order and record the case as a launch failure, not a confirmed execution.

### F4 — P3 (PLAUSIBLE, Linux): the pre-exec descriptor sweep is as long as `RLIMIT_NOFILE`

**Location:** `crates/keyed-launch/src/run.rs:885-898` (`for fd in 3..descriptor_limit`
between `fork` and `exec`) and `:971-1003` (`descriptor_limit`).

Every detached launch calls `fcntl(fd, F_SETFD, FD_CLOEXEC)` for each descriptor up to
the soft limit. On macOS that is 256 and free. A Linux host started under systemd or
containerd with `LimitNOFILE=infinity` has a soft limit of 1073741816, and the loop
then takes minutes per launch in the child before it execs, with `fork`'s parent
polling. This node makes it the common path: `grove run` pays it twice (Grove's launch
of dispatch, dispatch's launch of the sandbox), and a soft limit of `RLIM_INFINITY`
makes `descriptor_limit` refuse the launch outright (`"set a finite open-file
limit"`). The producer could not see it: the tests run on a 256-descriptor host.

**Correction:** `close_range(3, !0, CLOSE_RANGE_CLOEXEC)` where it exists (Linux 5.11,
FreeBSD), keeping the loop only as the fallback and only up to the highest open
descriptor read from `/proc/self/fd`, which `descriptor_limit` already lists.

### F5 — P3 (CONFIRMED): the seam leaves three named obligations untested

**Location:** `crates/grove/tests/standalone.rs` as a whole; the node brief's
"Check that the report cannot be supplied from inside the sandbox", "cancellation at
selection/run/publication", and the spec's "any partial publication is reported".

- Nothing makes the harness try to supply the report. No case has the harness attempt
  `../control/ending.json`, then exit 0 without acknowledging; it would pass only
  because `control/` is outside the grants, which is the property to pin (F2 is the
  same property for dispatch's own callers).
- Cancellation is tested during selection (`:413`) and during the run (`:600`), not
  between dispatch's exit and publication, so `standalone.rs:206-209` and
  `:295-298` (`take_interrupt` before each persist, and the message naming the
  already-published prefix) have no executing test.
- `replacing_the_staging_directory_cannot_redirect_export_to_host_files` (`:536`)
  runs `set -eu; cd ..; mv work original`. If the sandbox denies the `mv`, the
  harness exits non-zero and the case passes without ever reaching the held-directory
  read it is named for; a harness that runs `mv … || true` and then acknowledges would
  prove it.

### F6 — P4: stale contract text after the removal of `run_confined` and `Confinement`

**Location:** `docs/specs/keyed-launch-book-structure.md:442` (the early-use row still
lists `run_confined` and `Confinement` and omits `NoninteractiveLaunch`, while
`walkthrough.toml:213-217` and the book have the right set);
`docs/specs/module-decomposition.md:370-381` (decision 7's listing gives
`run_noninteractive(launch: Launch<'_>, …)`, the removed `Confinement` and
`run_confined`).

The first is a book-structure contract that `k14` should have moved with the book.
The second belongs to `current-state-docs-k20` by the root brief; this finding only
makes sure that leaf is told about the signature change, the removal and
`run_confined_observed`/`FilesystemGrants`.

### F7 — P4: three small drifts

- **A guard dropped.** `.cargo/config.toml` and `testing/support.rs:308-313` no longer
  clear `GROVE_RUN_SIGNAL_FILE`. The root brief's Notes keep the guard "beside the new
  names for as long as this grove runs", because its sessions run the installed v22,
  and v22's `grove run` still exports that name to its harness; a `cargo test` from
  such a harness is no longer guarded. Narrow, but it is a stated rule, not a choice.
- **A misleading message.** `standalone.rs:196-201` says "the harness's process group
  {pgid}" for `ended.group`, which is the group of the dispatch job Grove launched; the
  harness's own group reaches Grove only as dispatch's exit 5. The overview book says
  "a surviving dispatch group", and is right.
- **A display failure discards a finished run.** `standalone.rs:186-190` propagates the
  relay's error before the run's result is considered, so a closed stderr
  (`grove run … 2>&1 | head -1`) turns an acknowledged, published-ready run into a
  failure with nothing published. It fails closed; the question for the integrator is
  whether an unreadable display should cost the work.

### Unstated assumptions the integrator should write down or test

- **Grove's death orphans the run.** Dispatch is in its own session
  (`run_noninteractive`), so a SIGKILL of Grove, or an outer supervisor's group SIGKILL
  that reaches a nested `grove run`, leaves dispatch and the confined harness running to
  their own end (on Linux `--die-with-parent` binds bubblewrap to dispatch, not dispatch
  to Grove). Nothing publishes, which is the guarantee; a running tree remains. The
  previous direct `run_confined` had the same property, so this is not a regression.
- **Nested cancellation fits only if dispatch's end is quick.** Grove's 5 s kill-grace
  for dispatch must cover dispatch's immediate kill, the 1 s group confirmation and
  `record_end`'s store lock (2 s, longer when it migrates a version-1 store). The first
  `grove run` after upgrading a store has the longest tail.
- **`argv[0]` is the resolved path.** A confined harness reached by a multicall symlink
  (a `bunx` → `bun`) runs under the target's name. The spec says so; no test pins it.

### What held

Read and found consistent with the contract: the canonical overlap refusal
(`prospective` resolves every existing component, so aliases, `..` and an absent state
directory behave; the policy entry is already canonical, `authority.rs:36-41`, so a
symlinked policy protects its real directory); the two exempt executables (added after
the check, as read-only literals); the system-read inventory shared by the check and
both backends; the minimal environment (an allowlist, `env_clear` before the grants, the
channel set last, the same allowlist the old `inherited` used); the closed descriptors
and null stdin; the exit channel's directory held since allocation (`Channel::appeared`
and `read` do not follow a substituted path); `ExitSignal` mapped to 0 only when
escalated or exited 0; the ending file written exclusively, owner-only, after the group
is gone, with no file at exit 5; Grove's publication conditions in the order the spec
states them, outputs read through the descriptor held before launch with `O_NOFOLLOW`;
the termination handler staying installed after the run, so a signal during publication
is latched; and the optional-channel runner core, whose interactive path is the same
`Job` fields, the same `Option` tests and the same `supervise` arguments as before.
The overview, `keyed-launch` and `grove-llm` books were read against the source and
describe the ownership, cancellation and publication contracts as implemented.

### Limits of this review

Linux bubblewrap was not exercised by the producer or by this review; every Linux
statement is reading of `platform_command` and of bubblewrap's documented behaviour.
The macOS Seatbelt semantics in F1 are from the profile text, not a probe. Codebase
graph access was unavailable, so no negative repo-wide claim rests on it; the stale
references in F6 and the guard names in F7 came from `grep` over the tree. No test,
build or lint ran.

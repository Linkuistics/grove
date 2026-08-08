# driver-lease-review-k32

**Kind:** review-impl
**Reviews:** driver-lease-k31
**Producer launch:** {"producer":"driver-lease-k31","session":"driver-lease-k31","generation":"k31","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `driver-lease-k31` and record concrete findings for its integration step.

## Context

- Review `driver-lease-k31` against
  `docs/adr/one-live-driver-per-working-tree.md`.
- Attack control-directory aliasing, symlink/canonicalization behavior,
  Git-common-dir leakage, jj workspace sharing, external selector influence,
  lock/path replacement races, descriptor inheritance, and final disposition
  ordering.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `driver-lease-integrate-k33` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity, race/interleaving evidence or a
  black-box reproducer, and the violated contract, or an explicit no-finding
  result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded Git and jj contention evidence rather than re-running it.
- No production or test code is changed.

## Findings

Inspected: commit `yolonkvllnwq` / `2e9038f95a1c` (`driver-lease-k31`) diff and
post-change source for `src/driver_lease.rs`, `src/repo.rs`, `src/launch.rs`,
`src/lib.rs`, `src/loop_driver.rs`, plus `src/tree_access.rs` and
`src/harness_stamp.rs` for interaction; `tests/driver_lease.rs`,
`tests/repo.rs`, `tests/loop_driver.rs`, `tests/support/mod.rs`;
`docs/adr/one-live-driver-per-working-tree.md` (whole);
`docs/specs/config-driven-sessions.md` "Single-command lifecycle" (324-338) and
"Process ownership and session epochs" (339-512); `docs/USAGE.md` (28-49); the
grove root `BRIEF.md`; and `tree-access-lock-review-k55`'s recorded findings for
sequencing. **The producer recorded no verification evidence** (F2), so no
conclusion below rests on a test result. No code was run and no production or
test file was changed.

### F1 — medium — the new VCS-discovery cross-check reintroduces exactly the ambient-environment dependence this slice exists to remove, as a hard stop

`src/launch.rs:39-47`:

```rust
let discovered_worktree = worktree.canonicalize()?;          // launch.rs:39-41
anyhow::ensure!(
    driver_lease.worktree_root() == discovered_worktree,     // :42-47
    "on-disk working tree {} disagrees with VCS-discovered working tree {}", …);
```

`worktree` comes from `repo::toplevel(&cwd)` (`src/launch.rs:22`). In a git tree
that is `git_show_toplevel` (`src/repo.rs:212-224`) — `git rev-parse
--show-toplevel`, which honours `GIT_WORK_TREE` and `core.worktree`. So the
driver's overall behaviour is still steered by exactly the selectors the
resolver was built to ignore; only the *direction* changed, from "silently drive
the foreign tree" to "refuse to drive anything".

Contract violated: this leaf's Done-when bullet 1 — "closest on-disk `.jj`/`.git`
marker, **jj-first and independent of VCS discovery environment**";
`docs/adr/one-live-driver-per-working-tree.md:10-12` — "It invokes no repository
discovery and ignores `GIT_DIR`, `GIT_WORK_TREE`, and other ambient selectors";
`docs/specs/config-driven-sessions.md:353-354`.

Black-box reproducer, no concurrency needed:

```sh
cd /intended            # a plain git working tree with a live .grove/
GIT_DIR=/foreign/.git GIT_WORK_TREE=/foreign grove do
# → provisions, acquires the lease on /intended, then aborts:
#   "on-disk working tree /intended disagrees with VCS-discovered working
#    tree /foreign"
```

Reachable without any env var at all via `core.worktree` in `.git/config`, which
is precisely a git repo whose `.git` does not sit at its toplevel. Realistic
triggers for the env form: a git hook, `git rebase -x`, `git filter-branch`, and
CI wrappers that export `GIT_DIR`.

Scope note — the divergence is **git-only**. `toplevel`'s jj branch
(`src/repo.rs:155-157`) returns `vcs_of`'s lexical `.jj` ancestor, the same
on-disk walk `workspace_control` performs, so a jj tree can never trip the
`ensure!`.

`tests/repo.rs:206-233` proves only that `workspace_control` is immune; nothing
covers the driver under the same environment, which is why the Done-when bullet
reads as satisfied while the slice's user-visible guarantee is not.

Suggested direction: make `driver_lease.worktree_root()` authoritative for the
rest of `do_grove` and drop the `ensure!`. That is where
`docs/specs/config-driven-sessions.md:341-343` is already heading ("resolves the
working tree and acquires its **driver lease**"), it removes a `git` subprocess
from the startup path, and it makes the env-independence claim true end to end
rather than true of one private function. If the disagreement stop is wanted
deliberately, it needs to be in the spec and to have a test.

### F2 — medium — the producer recorded no verification evidence, so Done-when bullet 6 is unverified and this review's own Done-when cannot be met as written

`.grove/11-driver-lease-chain-k30/01-DONE-driver-lease-k31.md` ends at `## Notes`
(line 43-46) with no `## Verification` or `## Verification evidence` section, and
the commit message is a single line with no body:

```
driver-lease-k31: add process-scoped working-tree ownership
```

Done-when bullet 6 claims "`cargo fmt --check` and `cargo test --locked` pass";
nothing records that either ran. This review's Context says findings must cite
"the producer's recorded Git and jj contention evidence rather than re-running
it" — there is none to cite, so every claim below is by inspection only.

This is a **repeat** of `tree-access-lock-review-k55` F3, and the convention it
established is in this grove and one chain away:
`.grove/09-tree-access-lock-chain-k53/03-DONE-tree-access-lock-integrate-k56.md`
carries a `### Verification` section (lines 36-43) recording RED/GREEN steps and
final exit codes. The gap is load-bearing rather than cosmetic here for the same
reason it was there: this is a concurrency change whose Done-when bullet 5
enumerates seven fixture classes (aliases, shared-repository workspaces,
conflicting Git/TMPDIR environment, replacement retries/failure, unwritable
controls, descriptor inheritance, normal/forced release), several of which spawn
processes and are inherently timing-sensitive.

Action for `driver-lease-integrate-k33`: run both commands, record the result in
its own leaf, and treat bullets 5 and 6 as open until then.

### F3 — medium — the new environment test mutates process-global `GIT_DIR`/`GIT_WORK_TREE`/`GIT_COMMON_DIR`/`TMPDIR` while nineteen sibling tests in the same binary shell out to `git` and `jj` unserialised

`tests/repo.rs:206-233`. The test takes `support::lock_env(&ENV_LOCK)`
(`:208`), but `ENV_LOCK` (`:10`) is file-local and **no other test in
`tests/repo.rs` takes it**: 20 `#[test]` functions, exactly 1 `lock_env` call.
Cargo's default harness runs every test in a binary on parallel threads in one
process, and `.cargo/config.toml` sets no `--test-threads`, so
`EnvGuard::set` (`tests/support/mod.rs:205-209`, `std::env::set_var`) is visible
to all nineteen siblings.

The convention this breaks is the codebase's own, stated at
`tests/loop_driver.rs:22-28` ("serialize so cargo's parallel runner doesn't
cross test wires") and honoured exactly: 59 tests, 59 `lock_env(&ENV_LOCK)`
calls, 100%.

Interleaving evidence — any sibling whose git/jj invocation lands inside the
window between `:220-224` (set) and the `EnvGuard` drop at end of test:

- `init_git_repo` (`tests/repo.rs:12`, `git init -q .`) with `GIT_DIR` set
  initialises the *foreign* repo instead of its own TempDir, so the fixture the
  sibling then asserts against does not exist.
- `toplevel_in_plain_git_repo_resolves_from_subdir` (`:238`) runs `git rev-parse
  --show-toplevel`, which under `GIT_WORK_TREE` returns `<tmp>/foreign` — its
  `assert_eq!` against its own TempDir fails.
- `main_repo_of_git_linked_worktree_is_the_main_checkout` (`:284`) runs
  `git rev-parse --git-common-dir`, redirected the same way.
- `TMPDIR` is the worst of the four: a concurrent `TempDir::new()` allocates
  under `<tmp>/ambient-tmp`, which is unlinked when *this* test's outer `TempDir`
  drops — pulling a sibling's fixture out from under it mid-test, with a failure
  that points nowhere near this test.

Honest severity bound: the window is short (set → one `workspace_control` call →
four assertions, no subprocess), so this is a low-frequency flake, not a
deterministic failure — which is exactly what makes it expensive to diagnose
later.

Suggested direction: either have every test in `tests/repo.rs` take `ENV_LOCK`
(matching `tests/loop_driver.rs`), or move the env-mutating test into its own
`tests/*.rs` target, which cargo compiles and runs as a separate process and so
needs no lock at all.

### F4 — low-medium — `a_second_driver_reprovisions_then_refuses_before_tree_access_or_launch` does not test "before tree access"; moving the lease below migration would leave it green

`tests/driver_lease.rs:459-511`. The fixture seeds an already-**v2** tree —
`.grove/FORMAT` = `session-kinds-v1`, `.grove/BRIEF.md`, `.grove/01-impl-test-k1.md`
(`:464-467`) — so `tree_migrate::migrate_on_adoption` (`src/launch.rs:57-59`) is
a no-op regardless of ordering. The four assertions observe exit status, the
refusal on stderr, reprovisioned `SKILL.md`, and absence of a second launch
(`:496-509`). None of them can distinguish a lease acquired at
`src/launch.rs:38` from one acquired after `:65`.

Failure the test is meant to exclude but does not: with a legacy `.grove/`
(v1-flat or `NNN-slug`) and the lease below migration, the second driver would
run the adoption migration and **commit** it (`migrate_on_adoption` commits for
review) against a tree the live driver's session is concurrently mutating —
two writers, one of them producing a commit.

Suggested direction: seed a legacy-format `.grove/` in this fixture and assert
the second driver produces no new commit and no renames. That is the cheapest
assertion that actually pins "before tree access", and it also exercises the
first driver's own migration path, which is currently unexercised here.

### F5 — low — `--no-launch`, documented as a read-only readiness check, now takes the exclusive lease and fails whenever a driver is live

`src/launch.rs:38` acquires unconditionally, above the `args.no_launch` return at
`:75-96`. `docs/USAGE.md:36-40` still reads "Use `--no-launch` to check the next
leaf, harness, model, prompt, Codex trust, and VCS access without starting an
agent or writing a harness stamp" — it does not say the check is unavailable
beside a running loop, which is the single situation where an operator is most
likely to want it.

Reproducer: with `grove do` running in a working tree, `grove do --no-launch` in
the same tree now exits nonzero with "another Grove driver already owns …". No
test covers this path.

The dry run performs no tree mutation — `readiness` (`src/loop_driver.rs`, via
`src/launch.rs:90`) spawns `grove-llm kind`, which takes its own shared Tree
access guard in that subprocess — so either resolution is defensible: move the
`no_launch` return above acquisition, or document the refusal. It needs to be
one of the two, since `--no-launch` survives until `legacy-command-surface-removal-k77`.

### F6 — low — both "pinned" comparisons in `DriverLease::revalidate` can never fail

`src/driver_lease.rs:89-107` and `:109-127`. Each half `fstat`s a descriptor this
process holds open and compares the result against an identity captured from
that same descriptor at `:60-64` / `:67`. An open inode's `st_dev`/`st_ino`
cannot change for the life of the description, so `pinned_root !=
self.worktree_identity` and `pinned_lease != self.lease_identity` are constant
`false`. The check that does the work is the `stat(path)` half in each pair.

Consequences: two dead syscalls per revalidation (three sites per loop
iteration — `src/loop_driver.rs:176-178`, `:234-236`, plus `src/launch.rs:48-50`),
and, more importantly, the code reads as if the held descriptor were being
re-verified, which invites a later reader to trust a guarantee that is not
there. `docs/specs/config-driven-sessions.md:374-376` asks only for "re-stats the
lease path against the held descriptor".

Suggested direction: drop the `pinned_*` halves and keep the path-vs-pinned
comparison, or leave a comment saying the `fstat` is documentation of intent
rather than a check.

### F7 — low — the descriptor-inheritance test cannot fail for its stated reason, and leaves a zombie

`tests/driver_lease.rs:428-456`. Rust's `File::open` and `OpenOptions::open`
always pass `O_CLOEXEC`, and `std::process::Command` never clears it, so the
`sh` descendant could not have inherited the lease even with
`ensure_close_on_exec` (`src/driver_lease.rs:194-207`) deleted. The test pins the
end-to-end property, which is worth having, but it cannot catch the regression it
is named for — a future acquisition site using raw `libc::open` without
`O_CLOEXEC`, which is a live possibility given `session-epoch-k35` adds two more
lock sites.

The in-repo model is `tests/tree_access.rs:317-323`, which asserts `FD_CLOEXEC`
on the descriptor directly.

Also `tests/driver_lease.rs:449-455`: `descendant` is never waited on, so the
`SIGKILL` at `:453-455` leaves a zombie for the remainder of the test binary's
life.

### F8 — low — the second-driver test leaks a live driver and a fake harness on any assertion failure

`tests/driver_lease.rs:483-510`. `first` is a spawned `grove` driver whose fake
harness is `while :; do sleep 1; done` (`:476`); it is terminated only by
`terminate_driver(&mut first)` at `:510`. Any of the four assertions at
`:496-509` panicking unwinds past that call, so the driver and its harness
outlive `cargo test`, holding a lease on a `TempDir` that `Drop` then deletes.

`Holder` (`:116-151`) already has the correct shape — an `impl Drop` that kills
and reaps. The same guard applied to `first` costs a few lines and makes a failing
run diagnosable instead of leaving stray processes behind.

### F9 — low — the acquisition primitive is sealed against the second consumer the spec has already declared

`docs/specs/config-driven-sessions.md:397-399`: "Every shared or exclusive epoch
acquisition uses the same open, lock, `fstat`/`stat`, bounded-retry sequence as
lease acquisition." `acquire_lease_file_with_hook` (`src/driver_lease.rs:136-174`)
is private to the module, hard-codes `LOCK_EX` (`:177`), and bakes "driver
lease" into five error strings (`:148`, `:156`, `:160`, `:167`, `:187`).
`session-epoch-k35` needs the identical sequence with `LOCK_SH` as well as
`LOCK_EX`, on a different file, with its own wording and a 30-second bounded wait
(`:442-444`) that this loop has no notion of.

Second-order: `ensure_close_on_exec` (`:194`) makes explicit a property
`src/tree_access.rs` leaves implicit, so the codebase now has two lock sites
that treat the same concern differently.

Not a defect in this slice's behaviour — recorded so `driver-lease-integrate-k33`
can decide whether to extract a mode- and label-parameterised
`acquire_locked_control(path, mode, label)` now, or leave it to
`session-epoch-k35` knowingly rather than by rediscovery.

### Verified without finding

- **The lease does not lock the working-tree root, and therefore cannot deadlock
  against the Tree access lock.** This is the sharpest available reading of
  `docs/adr/one-live-driver-per-working-tree.md:17-19` ("keyed by the filesystem
  device and inode of an already-open working-tree-root descriptor") going wrong.
  `src/tree_access.rs:122-129` locks the descriptor from `File::open(worktree)`;
  had `driver_lease` done the same, the driver would have held `LOCK_EX` on that
  inode for the whole loop and every session-side `grove-llm` mutation would have
  blocked forever. `src/driver_lease.rs:53-64` opens the root but locks only
  `<control>/driver.lease` (`:66-67`, `:177`), matching the spec's more precise
  `:360-365`. The driver also takes no in-process tree guard at all —
  `grep -n tree_access src/loop_driver.rs src/launch.rs` returns nothing, and
  `launch_verb` (`src/loop_driver.rs:816-822`) is a bare `is_dir()` — so the lock
  order at `docs/specs/config-driven-sessions.md:468-474` is respected trivially.
- **`flock` is the right primitive and is used correctly.** Per-open-file-
  description, so it survives `fork`, contends between two descriptions in one
  process, and is released by the kernel on death — which is what makes
  `normal_owner_exit…` (`tests/driver_lease.rs:296`), `forced_owner_exit…`
  (`:308`) and `owner_panic…` (`:320`) meaningful and what "no PID cleanup"
  (ADR `:24-26`) rests on. `fcntl` locks would have been per-process and dropped
  on any close, silently breaking both.
- **Control-directory resolution matches the ADR clause by clause.**
  `src/repo.rs:71-110`: jj-first per ancestor (`:73-82`), never following
  `.jj/repo` to the shared store, never consulting the default workspace; git
  directory → `<canonical .git>/grove` (`:84-96`); gitfile → parsed `gitdir:`,
  resolved relative to the file, canonicalised (`:97-106`, `:112-134`) — the
  standard layout for both linked worktrees and submodules; `git rev-parse
  --git-common-dir` is never called. No `Command` is spawned on this path.
  `tests/repo.rs:116-233` covers plain, linked, colocated, secondary-workspace
  and hostile-environment cases.
- **`vcs_of` and `workspace_control` agree on which ancestor wins.** Both walk
  `path.ancestors()` checking `.jj` before `.git` at each level
  (`src/repo.rs:53-65` vs `:71-107`), so a git repo nested under a jj workspace,
  a colocated repo, and a submodule all resolve identically through the two
  functions. F1 is the *only* divergence, and it comes from `git rev-parse`,
  not from the walk.
- **Aliasing collapses correctly.** Every branch canonicalises the candidate
  before deriving anything (`src/repo.rs:75`, `:86`, `:98`), so symlink and
  relative spellings reach one control directory and one lease
  (`tests/driver_lease.rs:187-208`), while linked git worktrees and secondary jj
  workspaces stay independent (`:226-294`).
- **A refused second driver cannot damage the incumbent's record.**
  `OpenOptions` at `src/driver_lease.rs:142-147` sets `create` but **not**
  `truncate`, and `write_record` (`:226-238`) runs only after the lock succeeds
  (`:67-69`), so the EWOULDBLOCK path at `:186-189` touches nothing but an
  `mkdir -p`.
- **The retry loop is bounded and releases as it goes.** `src/driver_lease.rs:141-174`
  re-binds `file` per iteration, so the previous description is dropped —
  releasing its `flock` — before the next `open`. Attempts are capped at 8 and
  the terminal `bail!` at `:173` is required for type-checking, not dead code the
  compiler could have elided.
- **Nonce generation meets the ADR.** `/dev/urandom` via `read_exact`
  (`src/driver_lease.rs:209-216`) — 128 bits, OS CSPRNG, short reads handled, no
  derivation from PID/clock/address/counter (ADR `:28-31`). Freshness across
  driver generations is asserted at `tests/driver_lease.rs:361-387`.
- **The lease is held through final disposition.** `run` takes it by value
  (`src/loop_driver.rs:137-147`) and `run_loop_with_lease` returns only after
  `complete::read_signal` and the terminal `remove_file`
  (`:256-277`), so the drop happens after disposition, satisfying spec `:372-374`.
  Revalidation sits at the top of each iteration (`:176-178`) and immediately
  before spawn (`:234-236`), plus once before the lifecycle transition
  (`src/launch.rs:48-50`).
- **Provisioning-before-lease is deliberate and asserted, not an ordering slip.**
  `src/launch.rs:33-37` states the reason and
  `tests/driver_lease.rs:459-511` pins it (`SKILL.md` restored by the refused
  driver). Everything the ADR actually orders after the lease — migration
  (`:57`), pre-flight (`:73`), readiness (`:90`), stamp (`:102`), launch (`:104`)
  — is after it. `harness_stamp::resolve_for_launch` (`src/harness_stamp.rs:16-51`)
  runs before the lease but is read-only.
- **The externally-replaced-lease race is disclaimed, not missed.** A third party
  unlinking and recreating `driver.lease` between the incumbent's acquisition and
  a newcomer's `open` lets both hold a lock, on different inodes. ADR `:82-84`
  and spec `:399-401` both exclude this explicitly as repository-control
  corruption, and the incumbent's next `revalidate` fails closed
  (`tests/driver_lease.rs:342-359`), bounding the overlap to one transition.
- **`tests/driver_lease.rs`'s single unlocked env mutation is safe, unlike
  `tests/repo.rs`'s.** `lease_path_loss_after_tree_selection…` (`:513-570`) calls
  `clear_grove_env()` process-wide, but its thirteen siblings are immune by
  construction: `grove_driver` (`:57-73`) `env_remove`s every name in
  `support::grove_env_names()` and sets its own, and `Holder` (`:120-133`) reads
  only `HOLDER_*`. This is why F3 names `tests/repo.rs` alone.
- **The `run_loop` seam's own lease acquisition is consistent with production.**
  `src/loop_driver.rs:151-159` resolves from `worktree` where `do_grove` resolves
  from `cwd`; `workspace_control` walks ancestors, so a subdirectory cwd and its
  root produce the same control directory. The fixture churn in
  `tests/loop_driver.rs` (`fs::create_dir_all` → `init_worktree`, `:77-89`) is the
  necessary consequence, since the seam now requires a real VCS marker.
- **Unix-only is pre-existing.** `src/driver_lease.rs`'s `std::os::unix` /
  `std::os::fd` imports match `src/tree_access.rs`, `src/llm_cli.rs`,
  `src/tree_lifecycle.rs` and `src/tree_migration_transaction.rs`; this slice
  narrows no platform support.

## Notes

The reviewer produces findings only; `driver-lease-integrate-k33` owns fixes.

F2 gates the rest: bullets 5 and 6 are unverified, so integration should run
`cargo fmt --check` and `cargo test --locked`, record the outcome, and judge no
other bullet closed until then.

F1 is the only finding that changes production behaviour, and the cheap fix
(make the lease's canonical root authoritative and delete the `ensure!`) also
moves the slice toward `docs/specs/config-driven-sessions.md:341-343` rather than
away from it — worth doing here rather than deferring to
`lifecycle-cutover-k39`, since the `ensure!` would otherwise be inherited as an
apparently-deliberate stop.

F3, F4, F7 and F8 are all test-quality: none blocks the contract, but F4 is the
one that matters most, because it is the assertion the ordering property in F1's
neighbourhood will be regression-checked against for the rest of the grove.

F9 is deliberately forward-looking. It needs no change under this leaf's scope;
recorded so `session-epoch-k35` does not rediscover it from scratch when it needs
the same sequence in shared mode.

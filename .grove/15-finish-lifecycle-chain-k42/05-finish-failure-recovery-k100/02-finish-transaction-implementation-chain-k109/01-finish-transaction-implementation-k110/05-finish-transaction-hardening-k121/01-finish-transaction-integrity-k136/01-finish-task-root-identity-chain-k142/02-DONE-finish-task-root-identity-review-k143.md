# finish-task-root-identity-review-k143

**Kind:** review-impl
**Reviews:** finish-task-root-identity-k139

## Goal

Adversarially review `finish-task-root-identity-k139` and record concrete findings for its integration step.

## Context

- This leaf was generated as a chain stub with an empty `Context`, so it did not
  carry this grove's usual inspection-only review constraint (17 of 22 review
  leaves state it). This review therefore *did* run verification commands and one
  temporary in-tree probe, disclosed under `## Verification run` below. No
  production or test code changed in the committed result.
- Record findings only. `finish-task-root-identity-integrate-k144` owns every fix
  and all post-fix verification.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The working tree is left byte-identical to the reviewed producer commit.

## Findings

Reviewed: commit `finish-task-root-identity-k139: retain task-root identity at
finish phase gates` (`src/finish_transaction.rs`, +189/-39) against this leaf's
`Done when`, `finish-transaction-integrity-k136`,
`finish-transaction-hardening-k121`, `docs/adr/task-tree-transactions-fail-closed.md`,
and `docs/specs/config-driven-sessions.md` § *Pre-commit transaction and recovery*.

All three `Done when` bullets are met: both regressions exist and fail without
their gate, the descriptor is carried through `repo::prepare_finish` into
`PreparedTransaction`, and `finish`/`recover_pending` signatures are unchanged.
The findings below are about what the two new refusal paths leave behind, and
about one construction site where the carried authority does nothing.

### F1 — medium — the preparation-gate refusal leaks the attempt-bound index backup and hard-blocks a same-launch retry

`src/finish_transaction.rs:22-29` — when `prepare_transaction`'s new gate
(`:413`) bails, `?` drops `prepared_commit` without ever calling `commit()` or
`rollback()`, which are the only paths that reach `revalidate_after_rollback`'s
`backup.discard()` (`src/repo/finish_commit.rs:145, 205`). There is no `Drop`
impl to catch this: `grep -rn "impl Drop" src/` returns only
`driver_lease.rs:868` and `provision.rs:264`, neither of them these types. So
the marked `git-index-backup` auxiliary that `preserve_git_index`
(`src/repo/finish_commit.rs:1071-1110`) created for this attempt survives the
refusal.

`finish_attempt_identity()` (`:386-403`) derives the identity from
`GROVE_SIGNAL_FILE`'s `signal-<32 hex>` name, which is per *launch*, so a retry
inside the same session reuses it — and `ensure_auxiliary_available`
(`src/finish_cleanup/auxiliary.rs:181-188`) bails on the leftover.

Measured (probe below, reverted): displace `.grove/` after `repo::prepare_finish`
→ gate refuses → put the real root back → retry with the same attempt identity →

```
finish auxiliary cleanup collision for role git-index-backup and attempt
11111111111111111111111111111111 in <worktree>/.git
```

The recoverable case this leaf's contract is *about* — the root moved, put it
back — becomes a hard block needing manual removal of files under `.git/grove`,
and the diagnostic names the collision without naming a remedy. It is also not
prefixed `Recovery pending`, so it does not enter the operator procedure the spec
defines for blocked finishes (`docs/specs/config-driven-sessions.md:722-731`).

Scope note: the pre-existing early returns between `prepare_finish` and
`commit()` (quarantine collision at `:423`, `create_dir` failure at `:431`) share
this leak, but those are "shouldn't happen" conditions. k139 made it reachable
from the *expected* adversarial condition it was written to detect. Not data
loss — `prepare_auxiliary` copies with `io::copy`
(`src/finish_cleanup/auxiliary.rs:113`), so the user's live index is intact.

### F2 — medium — the evacuation-gate refusal strands a ready witness and reports neither it nor the observed topology

`src/finish_transaction.rs:30` — `evacuate(&transaction)?` propagates with no
rollback and no witness removal, so a refusal at the evacuation gate (`:481`)
leaves a fully prepared witness (`MANIFEST.json` + `READY` + empty `original/`).
The producer's own test asserts exactly this state survives, at
`original_root/FINISHING-finish-k2` (`:857-862`).

Two problems follow:

1. **The witness is stranded where recovery cannot see it.** It sits inside the
   *displaced* directory, while `.grove/` is now the replacement. The next
   `pending_manifest(grove_root)` (`:194-211`) scans the replacement, finds no
   `FINISHING-*`, and returns `Ok(None)` → `FinishRecovery::None`. A blocking
   witness that recovery classifies as "nothing pending" is the fail-closed
   contract inverted.
2. **The diagnostic does not meet the parent brief's bar.**
   `finish-transaction-hardening-k121` requires diagnostics that "name the
   witness, recorded and observed topology, and exact-start versus exact-result
   operator procedures." The message is only:

   ```
   task root changed while finish transaction held it open: <path>
   ```

   No witness path, no recorded-vs-observed identity, no procedure — and again no
   `Recovery pending` prefix, which is this codebase's operator-facing blocked
   classification (`src/repo/finish_commit.rs:128, 190, 424…`,
   `src/finish_cleanup/auxiliary.rs:218`) and is what
   `tests/finish_lifecycle.rs:1028, 1453` assert on for blocked stops.

Because `revalidate_task_root` is shared by both gates, one message currently
serves two very different states — nothing prepared yet (F1's site) versus a
ready witness stranded off-path. They need different text.

### F3 — medium — `recover_pending` builds a `PreparedTransaction` whose task-root descriptor is never validated and never read

`src/finish_transaction.rs:134-146` opens a fresh task-root descriptor into the
recovery-path `PreparedTransaction`. That field has exactly one read site in the
whole crate:

```
:481  revalidate_task_root(&transaction.grove_root, &transaction.grove_root_directory)?;   // in evacuate
```

and `evacuate` is called only from `finish()` (`:30`). `recover_pending` reaches
`rollback` (`:165`) and `quarantine_and_dispose` (`:155`); neither touches the
field. So on the recovery path the descriptor is opened, held for the entire
recovery, and gates nothing — while `rollback` renames every manifest entry back
into `transaction.grove_root` **purely by path** (`:499-507`).

The cost is not the dead fd, it is the false invariant. `PreparedTransaction` now
advertises "owns a revalidated task root," and that holds for one of its two
constructors. The two siblings chartered next —
`finish-transaction-transition-recovery-k137` (rollback prefixes) and
`finish-witness-identity-k140` (descriptor-relative source moves) — are exactly
the leaves that will start doing `openat`-relative work through this field.
Concretely: once k140 makes `rollback` restore entries relative to
`grove_root_directory`, the forward path is gated at `:481` and the recovery path
is gated nowhere, silently.

Worth stating for triage: this cannot be fixed by adding a `revalidate_task_root`
call at `:136`. Recovery has **no recorded baseline** — the manifest
(`:89-97`) stores no root identity — so a freshly opened descriptor has nothing to
be validated against. The two honest dispositions are (a) make the field
structurally absent on the recovery path so the type stops implying a gate it
does not have, or (b) record the task-root identity in the manifest, which is
`finish-manifest-integrity-k141`'s ground and therefore a new-leaf conversation
rather than a k144 edit. (a) is available to k144 today.

### F4 — low — doubled error context at both `open_task_root` call sites

`:311-316` and `:136-141` each wrap `open_task_root` — which already contexts
with `"opening task root directory: {path}"` (`:362`) — in a second, near-identical
context naming the same path. The chain reads:

```
opening task root without following symlinks: /x/.grove: opening task root directory: /x/.grove: <io error>
```

One of the two layers should go. The recovery site's `Recovery pending:` prefix is
the part worth keeping there.

### F5 — low — the preparation gate sits after `repo::prepare_finish`; one cheap stat would move it earlier

Noted as already-accepted, not as a contract violation: this leaf's `Context`
states the promise is "a task-root identity gate before task-tree mutation, not
zero repository-adapter side effects," and `finish()` honours that.

The observation for k144 is only that the trade-off is cheaper to avoid than it
looks. A second `revalidate_task_root(grove_root, &preflight.grove_root_directory)`
immediately before `repo::prepare_finish` (`:21`) costs one `fstat` plus one
`lstat` and makes F1 unreachable for the ordinary case, because the refusal would
then land before any auxiliary is created. It does not close the race — a
substitution during `prepare_finish` still lands on the existing gate — so it is a
blast-radius reduction, not a new guarantee.

## Verified without finding

Recorded so integration need not re-derive them:

- **Both regressions genuinely bind.** `preparation_refuses_task_root_replacement_without_touching_the_replacement`
  (`:781-822`) and `evacuation_refuses_task_root_replacement_without_moving_replacement_bytes`
  (`:825-863`) each assert the refusal message *and* that the replacement's bytes
  survive *and* that `original/` stayed empty. Deleting either gate fails them.
- **`O_CLOEXEC` closes a real pre-existing leak — keep it.** `open_task_root`
  (`:357-363`) adds it where the old inline open had only `O_DIRECTORY|O_NOFOLLOW`.
  The descriptor is held across `repo::prepare_finish`, which forks `git`/`jj`, so
  before this change the task-root fd was inherited by every VCS subprocess —
  contrary to the driver-lease requirement that lock descriptors never cross exec.
- **`ensure_same_device` still uses the descriptor's device**, not a re-`stat` of
  the path (`:317, 332-337`), so the same-filesystem proof for the atomic
  quarantine handoff is anchored to the validated inode.
- **The evacuation test's symlink is the right shape.** It plants
  `FINISHING-finish-k2` in the replacement as a symlink into the displaced root
  (`:846-850`), so a regression that skipped the gate would be caught moving
  bytes through an attacker-controlled link, not merely renaming files.
- **No stale references to the reworked diagnostic.** `grep -rn "task root changed\|finish preflight opened" src/ tests/ docs/ content/` finds only
  the new message and its two assertions.
- **Terminology was normalised, not drifted.** `task-root` → `task root` in the
  new messages matches the rest of the file (`:190`, `:311`).
- **`!path_metadata.file_type().is_dir()` at `:374` is unreachable** given `(dev,
  ino)` equality against an `O_DIRECTORY` descriptor. Harmless defence; not worth
  removing, but it is not carrying weight either.

## Verification run

Disclosed because this grove's other reviews are inspection-only and this one's
generated stub did not say so:

- `cargo fmt --check` — clean.
- `cargo test --locked` — fully green (492 lib tests plus all integration suites,
  0 failed). The 7 `finish_transaction` unit tests include both new regressions.
- One temporary `mod review_probe` was appended to `src/finish_transaction.rs` to
  measure F1's retry behaviour, then removed with `jj restore --from @-`. The file
  is byte-identical to the reviewed commit; the full suite above was re-run after
  the revert.

## Notes

F1 and F2 are the same underlying gap seen at the two gates — a refusal that is
correct about *refusing* but leaves the attempt un-retryable and under-described.
k144 can fix both without touching the gates themselves. F3 is the one finding
that may not be wholly k144's: disposition (a) is, disposition (b) is a new leaf.

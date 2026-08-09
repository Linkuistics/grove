# quarantine-cleanup-core-review-k129

**Kind:** review-impl
**Reviews:** quarantine-cleanup-core-k125

## Goal

Adversarially review `quarantine-cleanup-core-k125` and record concrete findings for its integration step.

## Context

- Producer change: `mvuwpqov` / `29ffe260`. New `src/finish_cleanup.rs`,
  `src/finish_cleanup/unix.rs`, `src/finish_cleanup/tests.rs`; the
  `quarantine_and_dispose` rewrite in `src/finish_transaction.rs`; one process
  test in `tests/finish_lifecycle.rs`.
- Binding artifacts: `docs/specs/config-driven-sessions.md` "Pre-commit
  transaction and recovery" (post-`Committed` paragraph, ll. 779-820); ADR
  `task-tree-transactions-fail-closed`; the `Done when` lists of
  `quarantine-cleanup-core-k125` and `finish-cleanup-reaping-k124`.
- Out of scope by the producer's own brief: Git-index auxiliaries
  (`auxiliary-cleanup-markers-k126`), driver/lease wiring
  (`cleanup-driver-acceptance-k127`), docs (`finish-transaction-docs-acceptance-k122`).
- Review is inspection-only. Findings are recorded here; the integration leaf
  owns every artifact change.

## Done when

- Findings carry severity, an exact filesystem failure sequence, and the
  threatened contract clause, or the task records an explicit no-finding result.
- The review proves that no disposal path can delete a substituted or foreign
  filesystem object, and that every interruption leaves independently valid,
  retryable evidence.
- The review checks each `quarantine-cleanup-core-k125` `Done when` clause
  against both behaviour and test coverage.
- No source, spec, ADR, glossary, or test artifact is changed.

## Result

**Six findings: one medium, one medium-low, four low. No high.** The core
disposal engine is sound — I could not construct a sequence in which Grove
deletes a substituted, foreign, or out-of-quarantine object, which is the
property this leaf exists to establish. Every finding below is about the
*reporting* and *seam* surfaces around that engine, not the engine itself.

Verified independently this session: `cargo fmt --check` clean;
`cargo test --locked` fully green (all suites, 0 failures); `cargo clippy
--all-targets` adds no new warnings for the new module.

### What I attacked and could not break (do not re-litigate)

- **No-follow disposal is airtight.** `remove_directory_contents` stats with
  `AT_SYMLINK_NOFOLLOW`, opens directories with `O_DIRECTORY|O_NOFOLLOW`,
  compares the opened descriptor's dev/ino against the stat it decided on,
  re-validates before `unlinkat(AT_REMOVEDIR)`, and unlinks anything
  non-directory as an entry. A directory swapped for a symlink between stat and
  open fails `ELOOP`; swapped for a different directory fails the identity
  check; refilled concurrently fails `ENOTEMPTY`. Every branch is fail-closed.
- **Identity binding survives the rename chain.** The marker records `.grove/`'s
  dev/ino, and `renameat2`/`renameatx_np` preserve the inode, so the same
  identity authorises handoff, claim, and removal. `validate_quarantine_identity`
  is called on every entry into a quarantine object.
- **Marker authority cannot be forged by relocation.** `validate_marker`
  cross-checks the attempt identity against the marker's own filename and the
  quarantine/claimed names against handle+attempt, so moving a valid marker to a
  different attempt name disarms it (tested).
- **Crash-before-handoff is recoverable.** Marker written, process dies,
  `.grove/` still present with its witness → `recover_pending` proves
  `Committed` → `prepare_quarantine` finds the byte-identical marker (inode
  unchanged) and returns it → handoff retries. Correct.
- **Crash-after-handoff is recoverable.** `.grove/` is absent, so recovery is
  the rootless contract's business, and the quarantine is left complete with its
  marker for a later lease owner — exactly the division `k124` specifies.
- **The marker is never inside what gets deleted.** It lives in the control
  directory beside the quarantine, so `Done when` 1's "a partial cleanup always
  retains that marker until all other entries are gone" holds structurally, not
  by ordering luck.
- **`restore` leaves a stale marker, but it self-heals.** A later `dispose` on
  that marker takes the empty branch and removes it. Not a leak worth a finding
  (but see F3 for what that branch conflates).

## Findings

### F1 — Post-claim disposal failure reports a path that does not exist — **Medium**

`QuarantineCleanup::dispose` claims the quarantine by renaming
`FINISHED-<handle>-<attempt>` → `REAPING-FINISHED-<handle>-<attempt>`
(`src/finish_cleanup.rs:369`) *before* removing any contents. The caller's
diagnostic (`src/finish_transaction.rs:467-471`) unconditionally prints
`quarantine_path`, which is the **pre-claim** name. Every failure at or after
the claim therefore names a path that no longer exists.

**Reproduced** (plain Git, `GROVE_TEST_FINISH_CLEANUP_FAIL_AT=before-root-removal`):

```
warning: completed Grove cleanup remains at
  <repo>/.git/grove/FINISHED-finish-k2-56411e435d924c4e30fdd48ad631314f: ...
```

while the control directory actually contains:

```
GROVE-FINISH-CLEANUP-56411e435d924c4e30fdd48ad631314f.json
REAPING-FINISHED-finish-k2-56411e435d924c4e30fdd48ad631314f
```

The boundary is exact: injecting at `before-claim` reports the correct path;
anything from `before-entry` onward reports the wrong one.

**Threatens.** `docs/specs/config-driven-sessions.md:793` — "Its exact path is
reported and best-effort disposal may be retried." An operator who `ls`es the
reported path finds nothing and reasonably concludes the warning was spurious,
leaving a full task-tree copy behind. The diagnostic also never names
`GROVE-FINISH-CLEANUP-<attempt>.json`, which is the actual retry key and the
"independently validated evidence" `finish-cleanup-reaping-k124` requires.

**Note.** The new process test asserts only that the message *contains*
"completed Grove cleanup remains", so it passes with the wrong path in it. The
predecessor review's F10 already flagged this paragraph as the weak point of the
cleanup-only claim; this is that weakness made concrete.

**Shape of the fix (integration's call).** Make `dispose`/`dispose_with` report
the active artifact — it is the only code that knows whether the claim
happened — and include the marker path. Tighten the process test to assert the
path that actually exists on disk.

### F2 — The "ambiguous quarantine" clause has no test — **Medium-low**

`quarantine-cleanup-core-k125` `Done when` 3 reads "A replaced or **ambiguous**
quarantine is left untouched with an actionable warning", and
`finish-cleanup-reaping-k124` repeats it ("ambiguous owners remain untouched
with an actionable warning"). The `(true, true)` branch at
`src/finish_cleanup.rs:341-346` implements it correctly — it bails before any
mutation — but **nothing exercises it**. Every other clause in that `Done when`
list has at least one dedicated test; `grep -rn ambiguous src/ tests/` returns
no hit in `finish_cleanup`.

Reachable sequence: disposal is interrupted after the claim, and a later attempt
with the same handle and attempt identity re-creates `FINISHED-…` beside the
surviving `REAPING-FINISHED-…`. Narrow, but it is the exact state the branch was
written for, and an untested refusal branch is one refactor away from becoming a
deletion branch.

Same gap, smaller: `validate_marker`'s version and kind rejections
(`src/finish_cleanup.rs:519-532`) are unreachable from the current tests, which
only corrupt the marker *syntactically* — a well-formed marker with
`"version": 2` or a foreign `kind` is never exercised.

### F3 — `dispose` conflates "nothing to reap" with "reaped" — **Low**

When neither the quarantine nor the claimed name exists, `dispose` deletes the
marker and returns `Ok(())` (`src/finish_cleanup.rs:347-350`). That is the
branch `cleanup-driver-acceptance-k127`'s orphan reaper will hit most often, and
it reports success for a state in which nothing was disposed.

Consequence for the consumer: the reaper cannot distinguish "I cleaned up an
orphan" from "the marker was already stale", so it cannot report honestly, and
the destructive default (delete the marker) fires without any in-tree-owner
check — the guard `k124` requires must therefore live entirely in the caller.
Harmless today because `quarantine_and_dispose` only calls `dispose` immediately
after a successful handoff, and because a marker deleted out from under an
in-flight recovery is simply rewritten by the next `prepare_quarantine`.

**Shape of the fix.** Return an outcome enum (`Disposed` / `NothingToDispose`)
rather than `()`, so `k127` can wire the owner check and the report against a
real distinction instead of inferring one.

### F4 — `directory_names` is single-shot per descriptor, silently — **Low (latent)**

`directory_names` (`src/finish_cleanup/unix.rs:87-128`) duplicates the directory
descriptor with `F_DUPFD_CLOEXEC` and hands the duplicate to `fdopendir`. A
`dup` shares the *open file description*, and that is where the directory offset
lives — so reading the stream to EOF advances the original `File`'s cursor too.
A second call on the same `File` returns an **empty list**, not an error.

**Verified on this machine** with a minimal C reproduction against the identical
call sequence: first listing 5 entries, second listing 0.

Not a live defect: every current call site passes a freshly `openat`ed
descriptor, and `dispose` lists each directory exactly once. It is a trap for
the next change — a retry-in-place loop, or any caller that lists, acts, and
re-lists, would conclude the directory is empty and leave entries behind while
reporting success. Silent-wrong-answer, not fail-closed, which is out of
character for the rest of this module.

**Shape of the fix.** One line: `rewinddir(stream)` after `fdopendir`, or
`lseek(duplicate, 0, SEEK_SET)` before it.

### F5 — The unsupported-platform arm cannot compile; BSD support is illusory — **Low**

`rename_at_noreplace` (`src/finish_cleanup/unix.rs:178-228`) binds `status`
under `cfg(linux|android)` and `cfg(macos|ios)` only, then has a
`cfg(not(any(...)))` arm that `return`s an `Unsupported` error — but the
trailing `if status < 0` is still type-checked on every target, so on any other
Unix the function fails to compile (`E0425`) rather than degrading gracefully as
it is written to appear.

That matters because `errno_pointer` in the *same file* (ll. 252-261) carries
explicit `freebsd` and `openbsd` arms. Those arms are the only BSD gesture
anywhere in the crate (`unix.rs` is the only file using `target_os` at all), and
they can never be reached, because the file cannot build on those targets. The
file states a portability intent it does not have.

**Shape of the fix.** Either drop the BSD `errno_pointer` arms and replace the
runtime `Unsupported` return with `compile_error!`, or supply a real BSD
`rename_at_noreplace`. Silent middle ground is the one option to avoid.

### F6 — The `REAPING-` prefix can exceed `NAME_MAX` after the point of no return — **Low**

`prepare_quarantine` validates the quarantine name but never the *claimed* name
it derives from it. `REAPING-` adds 8 bytes, so a handle long enough that
`FINISHED-<handle>-<32 hex>` fits within `NAME_MAX` while
`REAPING-FINISHED-<handle>-<32 hex>` does not (≈205-character slug on a
255-byte-limit filesystem) produces a quarantine that can be created and handed
off but **never claimed** — every disposal attempt, immediate and lease-owned,
fails `ENAMETOOLONG` identically, forever.

`validate_slug` (`src/tree_id.rs:430`) imposes no length bound, so this is
reachable rather than impossible; it is merely absurd. Recorded because of
*where* it fails: after the deletion commit, when refusal is no longer an
option. The natural place to check is `prepare_quarantine`, which runs while the
tree is still recoverable.

## Notes

Producer receipt is missing, so the review-target diversity check for this leaf
was uncheckable (review target `claude`/"opus"); the launch continued per policy
and this is recorded only for provenance.

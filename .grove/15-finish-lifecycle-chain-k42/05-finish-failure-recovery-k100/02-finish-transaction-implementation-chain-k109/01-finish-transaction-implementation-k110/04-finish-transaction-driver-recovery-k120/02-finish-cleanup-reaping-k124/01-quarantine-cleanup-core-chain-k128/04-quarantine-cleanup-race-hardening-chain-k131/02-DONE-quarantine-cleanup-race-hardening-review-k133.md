# quarantine-cleanup-race-hardening-review-k133

**Kind:** review-impl
**Reviews:** quarantine-cleanup-race-hardening-k132
**Producer launch:** {"producer":"quarantine-cleanup-race-hardening-k132","session":"quarantine-cleanup-race-hardening-k132","generation":"k132","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `quarantine-cleanup-race-hardening-k132` and record concrete findings for its integration step.

## Context

- Producer change: `tvvktvks` / `cfe9ec41`. `src/finish_cleanup.rs` (+230/-40),
  `src/finish_cleanup/unix.rs` (+24), `src/finish_cleanup/tests.rs` (+243),
  `tests/finish_lifecycle.rs` (+150).
- Binding artifacts: the four `Done when` clauses of
  `quarantine-cleanup-race-hardening-k132`; `finish-cleanup-reaping-k124`'s
  `Done when`; `docs/specs/config-driven-sessions.md` "Pre-commit transaction
  and recovery" (ll. 806-827); ADR `task-tree-transactions-fail-closed`;
  glossary term *Finish transaction*.
- Out of scope by the producer's own Notes: driver/lease reaping
  (`cleanup-driver-acceptance-k127`), Git-index auxiliaries
  (`auxiliary-cleanup-markers-k126`), docs
  (`finish-transaction-docs-acceptance-k122`).
- Review is inspection-only. Findings are recorded here; the integration leaf
  owns every artifact change.

## Done when

- Findings carry severity, an exact filesystem failure sequence, and the
  threatened contract clause, or the task records an explicit no-finding result.
- Each of the producer's four `Done when` clauses is checked against both
  behaviour and test coverage, including whether a clause is testable at all.
- No source, spec, ADR, glossary, or test artifact is changed.

## Result

**Five findings: one medium, two medium-low, two low. No high.** Three of the
four `Done when` clauses are met cleanly and are properly tested. The fourth —
"ambiguous-owner and nothing-to-dispose decisions are stable against
deterministic races" — is met only for races inside a two-syscall probe window,
and the one branch that still acts destructively on a probe-only decision has no
injectable seam (R2).

Verified independently this session: `cargo fmt --check` clean; `cargo test
--locked` fully green (all suites, 0 failures, 87s in `finish_lifecycle`);
`cargo clippy --all-targets` adds one new `dead_code` warning (R5) against a
pre-existing baseline of 35.

R1 was confirmed empirically with a throwaway unit test (written, run, reverted;
the working tree carries no source change from this session).

### Clause-by-clause

| `Done when` | Behaviour | Tests |
|---|---|---|
| 1. every non-directory unlink revalidates identity | met (`unix.rs:100-105`) | met — regular-file and symlink unit tests, plus one process test |
| 2. marker names must be single non-empty components before any descriptor-relative operation | met (`validate_marker_component`, reached via `read_marker` before any use) | met — 4 unit tests; but see R5's note on the publish-then-validate ordering |
| 3. ambiguous-owner / nothing-to-dispose stable or fail closed | **partial** — see R2 | partial — the `(false,false)` window has no checkpoint |
| 4. component-limit validation and publication bound to one no-follow directory object | met (`create_new_file_at` + `rename_at_noreplace`, both descriptor-relative) | met — one unit test, one process test |

### What I attacked and could not break (do not re-litigate)

- **Marker publication really is descriptor-bound.** Renaming the control
  directory out from under a paused `prepare_quarantine` lands the marker in the
  *validated* inode, not at the replaced path. Both the unit test and the process
  test prove this by construction rather than by message-matching.
- **`create_new_file_at` is correctly flagged.** `O_WRONLY|O_CREAT|O_EXCL|
  O_NOFOLLOW|O_CLOEXEC` with mode `0600`; `O_EXCL` makes the 8-draw retry loop's
  `AlreadyExists` arm meaningful rather than decorative.
- **`validate_marker_component` has no bypass.** `Path::components()`
  normalisation is neutralised by the `component.as_bytes() == name` equality, so
  `"foo/"`, `"foo/."`, `"./foo"`, `".."`, `""` and `"/"` are all rejected; the
  explicit NUL check covers the one byte `components()` treats as ordinary.
- **The probe-stability check does improve the common benign case.** A
  concurrent `rename(FINISHED-… → REAPING-FINISHED-…)` landing inside the window
  previously reported *ambiguous owner* — a flatly wrong diagnosis of an atomic
  rename. It now reports "owner changed during observation", which is true.
- **`(true,true)`, `(true,false)` and `(false,true)` are all backed downstream.**
  A change landing after the final probe fails at `open_directory_at` (`ENOENT`),
  at `validate_quarantine_identity`, or at the pre-claim `validate_entry_identity`.
  Every one is fail-closed. `(false,false)` is the exception — that is R2.
- **No regression in the six findings k130 integrated.** `rewinddir` is present,
  the claimed name is `NAME_MAX`-validated, `pending_cleanup_context` names the
  active path *and* the marker, `CleanupOutcome` survives, the ambiguous branch
  is tested, and `rename_at_noreplace`'s unsupported arm compiles.

## Findings

### R1 — Fresh marker publication is the one path that never checks what it read back — **Medium**

`prepare_quarantine_with` has three ways to end up holding a `QuarantineCleanup`.
Two of them compare the loaded marker against the marker this call constructed:

- `src/finish_cleanup.rs:177` — marker already present: `if existing.marker != marker { bail!("collision") }`
- `src/finish_cleanup.rs:226` — publishing rename lost the race: same check

The third does not:

```rust
// src/finish_cleanup.rs:211
Ok(()) => QuarantineCleanup::from_marker_at(&quarantine_parent_directory, &marker_path),
```

`rename_at_noreplace` guarantees the destination did not exist *at rename time*.
`from_marker_at` then re-opens that name. A foreign writer that unlinks and
re-creates `GROVE-FINISH-CLEANUP-<attempt>.json` in between hands us a marker we
never wrote, and `validate_marker` accepts it: it only cross-checks the marker
against *itself* (attempt vs. its own filename, quarantine name vs.
handle+attempt, claimed vs. `REAPING-` + quarantine). `finish_handle`, `device`
and `inode` are all free.

**Why that matters: the marker bytes, not the caller's argument, decide where the
task root goes.** `handoff` derives its destination from the marker
(`src/finish_cleanup.rs:341`), as `restore` derives its source
(`:393`) and `dispose_with` its two candidate names (`:462-463`). The
`quarantine_path` that `finish_transaction` preflighted — same-device check at
`finish_transaction.rs:481`, non-existence check at `:345` — is used only to
derive the *name to validate*, never to constrain the handoff.

**Verified.** A throwaway unit test published a legitimate marker, rewrote it in
place with `finish_handle: "foreign-k9"` (same attempt, same task-root dev/ino),
reloaded it and called `handoff`: the task root was renamed to
`FINISHED-foreign-k9-<attempt>` and `.grove/` was gone. Neither
`validate_marker` nor `validate_quarantine_identity` objects, because both are
satisfied by a self-consistent marker naming the real task root.

**Threatens.** Glossary *Finish transaction*: "the helper … atomically renames
the entire task root, witness intact, into a **preflighted** same-device
workspace-control quarantine". After a substituted marker the destination is no
longer the preflighted one, and the same-device guarantee the preflight
established no longer covers it.

**Shape of the fix (integration's call).** One line, restoring the symmetry the
other two arms already have: compare `existing.marker == marker` on the success
path too, and bail with the same collision diagnostic. Cheap, and it makes the
"the marker is authoritative" design safe by making the marker's *provenance*
checked at every entry rather than at two of three.

### R2 — `(false,false)` is the only destructive probe-only decision, and it is the one race with no seam — **Medium-low**

`dispose_with` now takes four `entry_exists` probes and refuses when the pair
disagrees (`src/finish_cleanup.rs:464-484`). For three of the four outcomes that
is belt-and-braces: whatever the probes said, the code goes on to open the object
and validate its dev/ino against the marker, so a late change fails closed.

For `(false,false)` there is nothing downstream:

```rust
// src/finish_cleanup.rs:492-495
(false, false) => {
    self.remove_marker(&parent)?;
    return Ok(CleanupOutcome::NothingToDispose);
}
```

`remove_marker` revalidates the *marker*, never the absence that justified
deleting it.

**Failure sequence.** `prepare_quarantine` publishes the marker
(`finish_transaction.rs:451`) and `handoff` creates the quarantine
(`:457`) — the marker precedes the quarantine by design, so `(false,false)` with
a live marker is a *normal* transient state, not a corrupt one. A reaper
observing it during that gap deletes the marker; `handoff`'s
`revalidate_marker` then fails and the finish reports an error with the task root
intact (recoverable). If the deletion instead lands between `handoff`'s
`revalidate_marker` and its `rename_at_noreplace`, the quarantine is created with
no marker beside it — and reaping requires a valid marker, so that quarantine is
a permanent orphan holding a full copy of the task tree.

**Threatens.** The producer's own `Done when` 3 — "…or fail closed **without
deleting either candidate or the marker**" — and `finish-cleanup-reaping-k124`'s
"Interruption or partial recursive deletion retains independently validated
evidence so later lease owners can finish cleanup safely."

**Two things make this worth recording rather than waving through.**

1. **There is no checkpoint between the stable observation and `remove_marker`.**
   Every other race this leaf closes got a `CleanupStep` and a deterministic
   test. This one cannot be tested as written, so clause 3 is asserted rather
   than demonstrated for the branch where it matters most.
2. **In production the checked window is ~2 syscalls wide.**
   `BetweenOwnerProbes` sits between probes 1 and 2 (`:465`); probes 3 and 4
   follow immediately. Without the injected pause the four `fstatat`s are
   back-to-back, so the mechanism detects essentially only the race the test
   seam manufactures. That is not wrong — narrowing is all POSIX offers here —
   but the clause should be read as "the injected race is caught", not "the
   decision is stable".

**Shape of the fix.** Two options, integration's call:
- *Minimal:* re-probe both names inside `remove_marker`'s `(false,false)` caller
  after `revalidate_marker` and before `unlink_at`, and add a `CleanupStep` so
  the window is testable like every other one.
- *Structural:* derive the claim state from the operation that changes it rather
  than from probes — attempt `rename_at_noreplace(quarantine → claimed)`
  unconditionally and classify by errno (`Ok` = we owned it, `ENOENT` = probe
  the claimed name, `EEXIST`/`ENOTEMPTY` = genuinely ambiguous). One atomic
  syscall replaces four probes and removes the ambiguity false-positive
  entirely. **Caveat that must be resolved first:** the current code validates
  the quarantine's dev/ino *before* claiming, and rename-first would claim a
  foreign directory momentarily. Do not adopt this without an answer to that.

### R3 — The new pause seam blocks forever, in the shipped binary, on an unscrubbed variable — **Medium-low**

`cleanup_test_checkpoint` gains a pause mode (`src/finish_cleanup.rs:680-700`)
whose wait has no deadline:

```rust
fs::write(&barrier, detail)?;
while barrier.exists() {
    thread::sleep(Duration::from_millis(10));
}
```

Three facts compose badly:

1. `cleanup_test_checkpoint` is **not** `#[cfg(test)]` — it cannot be, because
   the process tests spawn the real `grove-llm` binary. Both variables ship.
2. `GROVE_TEST_FINISH_CLEANUP_*` is **not** in `launch.rs`'s `LOOP_CONTROL_ENV`
   scrub list (`src/launch.rs:269-274`), which carries only `GROVE_SIGNAL_FILE`,
   `GROVE_HARNESS_PID`, `GROVE_CLAUDE_PID` and `GROVE_SESSION_TARGET`. Nor is it
   force-cleared by `.cargo/config.toml`.
3. The pause points reachable in a real finish include
   `before-non-directory-unlink` and `before-root-removal` — i.e. **after** the
   scoped deletion commit and **after** the task root has been claimed as
   `REAPING-…`.

So a developer shell that exported the pair while reproducing a test by hand, and
then ran a real `grove` in the same shell, gets a `grove-llm finish-commit` that
hangs indefinitely with no output, while the loop driver waits on it as a
foreground child. The existing `FAIL_AT` sibling (`:674`) has the same exposure
but degrades to a diagnosed, recoverable error; an unbounded block is a
qualitatively different failure. The module's own doc comment — "The variable is
deliberately test-prefixed and is not user configuration" — is precisely the
assumption at fault: a prefix documents intent, it does not make a variable
inert.

**Threatens.** Glossary *Grove configuration*: "Test-only tool, clock, and grace
injection is an internal module seam, never supported environment
configuration." An env-var seam in the release binary already sits awkwardly
against that sentence; an unbounded one contradicts it in effect.

**Shape of the fix.** Bound the wait (a deadline, then the same `io::Error` the
`FAIL_AT` arm returns, naming the barrier path), and add
`GROVE_TEST_FINISH_CLEANUP_FAIL_AT`/`_PAUSE_AT`/`_BARRIER` to
`LOOP_CONTROL_ENV` so a session never inherits them. Both keep the process tests
working unchanged. Whether the env-var seam should exist at all is
`finish-transaction-docs-acceptance-k122`'s question, not this chain's.

### R4 — `create_temporary_marker` re-implements `driver_lease`'s nonce-draw pattern — **Low**

`src/finish_cleanup.rs:238-262` opens `/dev/urandom`, reads 16 bytes, renders hex
per byte, and retries up to a hardcoded `8` on collision. All four pieces already
exist in `driver_lease` and are already reachable from this side of the crate:

- `random_nonce()` (`src/driver_lease.rs:577-584`) — identical `/dev/urandom` +
  `read_exact` into `[u8; 16]`, same "opening OS randomness source" context
  string;
- `hex_nonce()` (`:586-592`) — the same rendering, without a `String` allocation
  per byte;
- `fresh_nonce()` (`:594-596`) — `pub(crate)`, and already used by
  `finish_transaction.rs:322`, so the dependency edge exists;
- `SIGNAL_DRAW_RETRY_LIMIT: usize = 8` (`:18`) with the same
  draw-until-unoccupied loop at `:328-339` and a diagnostic naming the limit.

The duplicate is small but it is the second copy of the crate's *only* randomness
policy, and the retry bound is now a bare literal in one place and a named
constant in the other. `let name = OsString::from(format!(".grove-cleanup-{}.tmp",
driver_lease::fresh_nonce()?))` collapses most of it.

**Note for whoever touches this.** The current shape opens `/dev/urandom` once
outside the loop, which `fresh_nonce()` does not; that is a real (if tiny)
difference, so if the loop bound is kept, prefer lifting a shared
`draw_nonce`-style helper over re-opening per attempt.

### R5 — Two diagnostics regressed, and `from_marker` is now test-only — **Low**

Three small reporting/cleanliness points, grouped because they share a fix site.

**(a) "changed from false to false".** The observation-mismatch message
(`src/finish_cleanup.rs:474-483`) prints a `changed from … to …` clause for
*both* names unconditionally, so the one that did not move reads as having
changed. In the common single-rename case the operator is told two things
changed when one did. Report only the differing name, or phrase it as
`observed A then B`.

**(b) Root-cause errors are discarded when cleanup of the temporary fails.**
Both `remove_temporary_marker(...)?` calls (`:197-204` and `:213-218`) propagate
the *removal* error with `?`, dropping the `error` that caused the branch. A
failed `write_all` (ENOSPC) whose temporary also cannot be unlinked reports only
the unlink failure. Chain them, or return the original with the removal failure
as added context.

**(c) `QuarantineCleanup::from_marker` has no non-test caller.** It had three
before this change (`from_marker` at the parent revision's ll. 149, 174, 176);
`from_marker_at` replaced all of them, and `clippy` now emits a new `dead_code`
warning at `:294`. Not worth deleting — `cleanup-driver-acceptance-k127` will
want to load a marker found by path. But note what it is: **the only remaining
by-path parent open in the module**, i.e. exactly the shape clause 4 spent this
leaf removing. When k127 wires orphan reaping, it should enumerate the control
directory through a descriptor and load markers with `from_marker_at`, not adopt
`from_marker` and inherit an ancestor-race the module has otherwise closed.

## Notes

- **Ordering nit, no finding.** `prepare_quarantine_with` publishes the marker
  and only then validates its component names, because validation lives in
  `read_marker` on the readback (`:211` → `:317` → `:658`). A `finish_handle`
  containing `/` therefore leaves a published, permanently unparseable
  `GROVE-FINISH-CLEANUP-<attempt>.json` behind before erroring. Unreachable today
  — handles come from validated slugs — and harmless while nothing enumerates
  markers in production (`marker_paths` is `#[cfg(test)]`). Recorded only so
  `cleanup-driver-acceptance-k127` treats an unparseable marker as skippable
  rather than fatal, or so integration hoists `validate_marker_component` ahead
  of publication.
- **Flakiness watch, no finding.** `wait_for` (`tests/finish_lifecycle.rs:49-58`)
  gives the child 5 seconds to reach its checkpoint, and that budget covers
  config validation, lock acquisition, root-entry digesting and a full Git
  commit in a debug binary. It passed comfortably here (63 tests / 87s), but it
  is the only wall-clock bound in the new process tests; if CI ever reports a
  timeout at `marker-publication-barrier` or `entry-unlink-barrier`, raise the
  deadline rather than hunting a race.
- Diversity was satisfiable and satisfied: producer ran `codex`/`sol-xhigh`
  (receipt above), review ran a different vendor and model.

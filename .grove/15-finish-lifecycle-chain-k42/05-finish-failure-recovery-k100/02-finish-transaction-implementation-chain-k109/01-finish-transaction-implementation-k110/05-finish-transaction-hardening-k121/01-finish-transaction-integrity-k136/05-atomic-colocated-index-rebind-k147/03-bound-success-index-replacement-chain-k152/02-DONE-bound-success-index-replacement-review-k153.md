# bound-success-index-replacement-review-k153

**Kind:** review-impl
**Reviews:** bound-success-index-replacement-k150
**Producer launch:** {"producer":"bound-success-index-replacement-k150","session":"bound-success-index-replacement-k150","generation":"k150","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `bound-success-index-replacement-k150` and record concrete findings for its integration step.

## Context

- This leaf was generated as a chain stub with an empty `Context`, so it did not
  carry this grove's usual inspection-only review constraint. This review
  therefore ran verification commands and two temporary in-tree probes,
  disclosed under `## Verification run`. No production or test code changed in
  the committed result.
- Record findings only. `bound-success-index-replacement-integrate-k154` owns
  every fix and all post-fix verification.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The working tree is left byte-identical to the reviewed producer commit.

## Findings

Reviewed: commit `bound-success-index-replacement-k150: bind exact filtered
index publication` (`src/finish_cleanup/auxiliary.rs`,
`src/finish_cleanup/auxiliary/marker_replacement.rs`,
`src/finish_cleanup/auxiliary/tests.rs`, `src/repo/finish_commit.rs`;
+687/-53) against this leaf's `Done when`, `atomic-colocated-index-rebind-k147`,
`finish-transaction-integrity-k136`, and
`docs/adr/task-tree-transactions-fail-closed.md`.

The central idea is right and the fix to the original defect is real: routing
`git update-index` through a private staging directory stops Git's own
lock-and-rename from minting an inode Grove never authorized, and the
`.replacing` state document now carries enough identity to classify the artifact
exchange across a crash. Two of the three `Done when` bullets hold. The findings
below are about the **order** in which the settled transaction validates versus
mutates, and about what the abort paths leave behind.

### F1 — high — the state document alone decides which two entries get exchanged, so a substituted `.replacing` swaps external bytes *before* anything fails closed

`src/finish_cleanup/auxiliary/marker_replacement.rs:724-773` — `validate_state`
constrains almost every name it reads: `canonical_name` must equal the marker's
own file name (`:756`), `state_path.file_name()` must equal the deterministic
`replacement_state_file_name(role, attempt)` (`:754`), and all four names pass
`validate_component`. But **`artifact_name` and `staged_artifact_name` are never
compared to `artifact_file_name(role, attempt)` and
`replacement_artifact_file_name(role, attempt)`.** The only check relating them
is that they differ from each other (`:757`).

Those two unvalidated names are then handed straight to the mutating
primitives — `rename_at_exchange` at `:284` and `unlink_at` at `:322`.

The ordering makes it reachable. `settle_marker_replacement` calls
`settle_artifact_exchange` **first** (`:182`), *before* `classify_phase` (`:183`)
and therefore before `validate_bound_artifact` (`:569, 575, 579`), which is the
only thing that ties the artifact back to a marker Grove wrote. So the exchange
of two attacker-named entries happens, and only then does the marker check
refuse.

Measured (probe below, reverted). A well-formed `.replacing` document was
published normally, then its two artifact-name fields were rewritten in place to
name an unrelated file `victim`:

```
PROBE F1 recover outcome: Err("Recovery pending: ... bound finish auxiliary
  artifact identity does not match replacement marker ...; replacement left untouched")
PROBE F1 victim inode before=52176301 after=52176302
PROBE F1 victim content after:   Ok("old index\n")            <-- was "PRECIOUS USER BYTES\n"
PROBE F1 artifact content after: Ok("PRECIOUS USER BYTES\n")  <-- was "old index\n"
```

The call returns `Err`, so it *looks* fail-closed, and the diagnostic literally
says `replacement left untouched` — but both files have already been exchanged.
The victim's bytes are not destroyed (the marker phase bails before
`remove_retired_artifact`), yet they have been moved out from under their
owner and the auxiliary now serves foreign content. In the colocated-jj path
this directory is `.git/`, so `index`, `HEAD` and `config` are all in range of a
name the document supplies.

This contradicts `atomic-colocated-index-rebind-k147`'s "Substituted artifacts
and markers still fail closed without deleting external bytes" *and* this leaf's
own "Both old and replacement artifact bytes remain protected at interruption and
substitution boundaries."

Two independent dispositions, and k154 should take both:

1. **Bind the names.** `validate_state` has `role` and `attempt_identity` in
   hand; two comparisons mirroring the existing `canonical_name` check close the
   redirection entirely.
2. **Validate before mutating.** Even with (1), `settle_artifact_exchange`
   running ahead of every marker-side check means the artifact is exchanged on
   the authority of the state document alone. Hoisting the
   `validate_bound_artifact` evidence ahead of the first `rename_at_exchange`,
   or classifying the marker phase first, restores "prove, then move."

The diagnostic's `replacement left untouched` claim also needs to stop being
emitted on a path where an exchange already landed.

### F2 — medium — an orphaned `.filtered` artifact is invisible to recovery and the reaper, yet fatal to a same-attempt retry

`src/finish_cleanup/auxiliary.rs:415-438` creates the deterministic
`<artifact>.filtered` before `bind_artifact_replacement` publishes any state.
Interruption inside that window leaves the file with no `.replacing` document to
describe it. Nothing removes it afterwards:

- `recover_auxiliary` (`:238-268`) tests the artifact, marker and
  `.replacing` names — **not** `.filtered` — so it reports an ordinary healthy
  auxiliary.
- `dispose()` (`:594-654`) and `activate()` (`:541-592`) remove the artifact and
  marker only.
- the reaper's discovery predicate is `AUXILIARY_PREFIX` + `.json`
  (`:279-285`), and `.filtered` matches neither, so `reap_orphaned` cannot see it
  even in principle.

Meanwhile k150 added `.filtered` to the collision gate in both
`prepare_auxiliary` (`:108`) and `ensure_auxiliary_available` (`:210`).

Measured (probe below, reverted) — the exact on-disk state after a crash between
`create_new_file_at` and `publish_state`:

```
PROBE F2 recover_auxiliary: Ok(true)                    <-- reports healthy
PROBE F2 after dispose: artifact exists=false marker exists=false filtered exists=true
PROBE F2 reaper-visible markers: []                     <-- unreapable
PROBE F2 same-attempt retry: Err("finish auxiliary cleanup collision for role
  git-index-success and attempt 1111...")
PROBE F2 new-attempt prepare: Ok("ok")
```

Bounded, not a permanent wedge: the attempt identity is the per-launch nonce, so
the next launch draws a fresh name and proceeds. The cost is a same-launch retry
turned into a hard collision whose message names no remedy, plus a full copy of
the user's Git index left in `.git/` permanently, owned by nothing. That last
part is the direct counterexample to k147's "Every interruption boundary while
changing a finish auxiliary artifact's identity has durable, parseable
same-attempt ownership."

Cheapest disposition: teach `recover_auxiliary` and the disposal paths about the
`.filtered` name they already know how to compute.

### F3 — medium — `prepare_without_grove` dropped its own cleanup, leaking a marked success index on every preparation failure

`src/repo/finish_commit.rs:1069-1076`. The diff replaced

```rust
if let Err(error) = remove_grove_entries(worktree, success_index.artifact_path()) {
    discard_temporary_index(Some(&success_index));      // <-- removed
    return Err(error.context(...));
}
```

with a `match` whose `Err` arm only adds context. The success index is a local
that is never returned on the error path, and `prepare_jj_finish`'s error arm
(`:917-923`) discards **`index_backup` only** — `backup.discard()` →
`try_discard()` disposes `self.cleanup`, which is the `git-index-backup`
auxiliary, not this one. There is no `Drop` impl covering it.

So every failure inside `remove_grove_entries` — `git ls-files` failing, the
staging `tempdir_in` failing, `fs::copy` failing, `git update-index` failing,
or `replace_artifact_from` itself — now strands a fully marked
`git-index-success` auxiliary for the current attempt.

Consequence is the same shape as F2: `ensure_auxiliary_available`
(`:884-888`) refuses the same-launch retry, while a later launch's
`reap_orphaned` clears it. Restoring the discard is a one-line fix; the `match`
that replaced the `if let` provides nothing the `?`-style form did not.

This is the same class as F1 of `finish-task-root-identity-review-k143` — worth
noting for triage that the pattern is recurring in this subtree rather than
being new here.

### F4 — low — the new private staging directory is itself an unowned, unreaped orphan

`src/repo/finish_commit.rs:1285-1294` creates
`GROVE-FINISH-FILTER-<random>/` inside the Git directory via
`tempfile::Builder::tempdir_in`. `TempDir`'s cleanup is a `Drop`, so it survives
process death, and its random name carries no handle and no attempt identity —
nothing can attribute or reap it later.

Smaller than F2 (the window is short and the leftover is a directory the user
can obviously delete), but it is a second new orphan class introduced by this
commit, and it sits in the same `.git/` directory the module otherwise polices
carefully. If k154 addresses F2 by teaching recovery the auxiliary side-file
names, this one wants the same treatment or an attempt-scoped name.

### F5 — low — `bind_artifact_replacement` enforces no name at all, and has no caller outside the module

`src/finish_cleanup/auxiliary.rs:329-377` accepts any regular file in the
artifact's directory: it checks the parent matches (`:334`) and that it is not
the artifact itself (`:337`), and nothing else. The deterministic `.filtered`
name exists **only** because `replace_artifact_from` computes it at `:415`.

That is what makes F1's forged document plausible in the first place — the
writer never records "this was the deterministic replacement," so the reader has
nothing to check against. The two fixes are the same fix.

It is also `pub(crate)` with no caller outside `auxiliary.rs` (verified:
`grep -rn "bind_artifact_replacement" src/` finds the definition, the internal
call at `:440`, and test uses only). Narrowing it to private, or requiring the
deterministic name inside it, removes the footgun.

### F6 — trivial — pointless signature churn in `write_child_stdin_and_wait`

`src/repo/finish_commit.rs:1315-1316` changed `fn …(mut child: Child, …)` to
`fn …(child: Child, …)` followed immediately by `let mut child = child;`. The
two forms are identical; this is diff noise in an otherwise tight commit.

## Verified without finding

Recorded so integration need not re-derive them:

- **The core fix is genuine.** Before this commit `git update-index` ran with
  `GIT_INDEX_FILE` pointing at the auxiliary artifact, and Git's write-lock-rename
  replaced that inode with one Grove never created. Staging the filter in a
  private directory and copying the result back under Grove's own
  `create_new_file_at` is the right shape, and
  `index_filter_publishes_from_private_staging_without_adopting_a_foreign_lock`
  (`src/repo/finish_commit.rs:1755-1808`) pins it, including that a foreign
  `<artifact>.lock` survives untouched with its inode intact.
- **`remove_grove_entries` returning `bool` is consistent.** The empty-stdout
  early return (`:1281`) correctly skips replacement entirely, and
  `prepare_without_grove` no longer calls `rebind_artifact_identity`
  unconditionally — which matters, because `rebind_artifact_identity` is now a
  *settle-only* operation that requires a published state document and would fail
  if called without one.
- **Crash recovery across the four artifact-phase boundaries genuinely works**
  for well-formed state. `an_interruption_before_artifact_exchange_preserves_both_bound_inodes_for_recovery`
  (`:103`) drives the `BeforeArtifactExchange` checkpoint and proves recovery
  completes the exchange and removes the retired artifact.
- **Symlink substitution of the bound replacement fails closed**
  (`recovery_rejects_a_symlink_in_place_of_the_bound_replacement`, `:177`), and
  the refusal happens in `read_artifact_identity`'s `open_file_at`, which is
  `O_NOFOLLOW` — so unlike F1, this one refuses *before* any rename.
- **`classify_artifact_phase`'s three-way match is exhaustive and
  order-correct**, and its `_ => bail!` arm covers every torn combination.
- **The state document's `sha256` digests are format-validated** (`:764-771`),
  and `remove_state_document` re-reads and re-validates before unlinking.
- **`cargo fmt --check` is clean** and `cargo test --locked` is fully green on
  the reviewed commit (see below).

## Verification run

Disclosed because this leaf's generated stub did not carry the inspection-only
constraint most of this grove's reviews state:

- `cargo test --locked --lib finish_cleanup` — 60 passed, 0 failed.
- `cargo test --locked` — fully green.
- `cargo fmt --check` — clean.
- One temporary `mod review_probe_k153` was appended to
  `src/finish_cleanup/auxiliary/tests.rs` to measure F1 and F2, then removed with
  `jj restore --from yxokukkxmosv src/finish_cleanup/auxiliary/tests.rs`.
  `jj diff --from yxokukkxmosv --stat` reports `0 files changed`, so the tree is
  byte-identical to the reviewed commit.

## Notes

F1 is the finding that should gate integration: it is a demonstrated mutation of
external bytes on a path whose diagnostic claims nothing was touched, and it is
squarely inside what this leaf and `atomic-colocated-index-rebind-k147` promised.
F1 and F5 share one fix (bind the deterministic replacement name at both the
write and read side); F1's second half — hoisting validation ahead of the first
`rename_at_exchange` — is separable and is the part that survives even if the
name binding is later relaxed.

F2, F3 and F4 are all "what does an abort leave behind," and all three are
bounded by the per-launch attempt identity plus the next launch's reaper. They
are cheap to fix together and none of them needs a new leaf.

Nothing here requires reopening `recoverable-marker-replacement-k149`'s contract;
the marker-side machinery it delivered is sound and F1 exploits the artifact
side, which k150 added.

# Task-tree transactions fail closed

Every task-tree reader or mutator takes the process-scoped advisory Tree access
lock on an open working-tree-root descriptor before inspecting `.grove/`:
readers hold it shared and mutators exclusive. The working-tree root is the one
invariant that exists before task-root initialization and through finish
deletion. The lock serializes cooperating processes, but it does not make a
multi-path filesystem or VCS mutation crash-atomic. Any operation that promises
process-interruption recovery therefore keeps a reserved, explicit witness
inside `.grove/` until its result is safe to expose.

The witness is ordinary artifact state with operation-specific contents:

- session-kind migration uses `MIGRATING-session-kinds/` to retain the original
  and staged trees until its focused migration commit is proven;
- producer promotion uses `PROMOTING-<final-node-name>/` to retain the only
  producer copy while it prepares and lands the complete review-chain node; and
- finish teardown uses `FINISHING-<finish-handle>/` to evacuate every ordinary
  root entry beneath a manifest-backed original-tree directory before the
  deletion commit. The `.grove/` directory itself remains present until the
  exact handle-named, `.grove/`-scoped commit is proven and the whole root can
  atomically move to post-commit cleanup quarantine.

Every ordinary reader and mutator refuses while any reserved witness exists.
Only the matching recovery path is admitted, and it runs before format,
selection, liveness, kind, or normal root-absence classification. This ordering
is load-bearing: a moved producer is not missing work, an evacuated finish tree
is not a malformed or fresh grove, and files hidden beneath a witness still own
their permanent keys. Generic diagnostics name the exact witness and the
operation that can recover it.

A finish transaction makes the commit boundary explicit. Preflight repository
validation runs before evacuation when it can do so without changing state. The
transaction then writes and verifies a manifest containing the stable handle,
repository-start anchor, exact expected tracked deletion fingerprint, and every
root entry's type/digest, marks the witness ready last, and moves ordinary root
entries beneath it without following symlinks. Git and jj commit only the
deletions at their original paths while excluding the witness. The generated
finish leaf remains workflow evidence in the witness; commit proof never assumes
that working-tree-only leaf existed in the starting VCS revision. Any ambiguous
command result is classified through the repository seam from the recorded
anchor, expected delta, and exact immediate result rather than trusting exit
status or task-root absence.

If commit proof is absent, rollback is licensed only when Git `HEAD` still equals
the recorded start or jj still exposes the recorded working-copy change at the
same parents. It then restores any Git or colocated-jj index backup before the
exact original tree. A reported failure exposes a live selectable finish leaf
only after every proof and restoration succeeds. A different revision,
restoration failure, or tree rollback failure keeps the witness unwalkable with
an actionable diagnostic.

If the exact commit is proven, recovery never restores the old tree. It completes
colocated-index activation, then atomically renames the entire `.grove/` root —
witness and evacuated tree intact — into collision-resistant post-commit cleanup
quarantine in the workspace's VCS-administration control directory before
attempting recursive disposal. Preflight requires that directory to be untracked
and on the worktree's filesystem; a cross-device workspace refuses before tree
mutation rather than using a trackable sibling or non-atomic copy. Rename failure
keeps the in-tree witness; interruption after rename leaves a complete
quarantine and an absent task root, never a partial or empty `.grove/`. That
quarantine and any VCS-administration index image are cleanup artifacts, not
workflow inputs or rootless-driver finish receipts. A proven commit plus
task-root absence remains successful even when best-effort quarantine disposal
must be retried.

Plain Git finish commits run with an empty internal hooks path. User hooks are
arbitrary programs and can mutate unrelated working-tree bytes that no index
backup can restore, so allowing them would contradict the scoped-preservation
contract. Signing and repository failures remain visible and recover through the
transaction.

This binds because no portable filesystem primitive atomically replaces a file
with a differently named directory, and no filesystem transaction can atomically
include a Git or jj commit. A visible in-tree witness preserves the stronger
property Grove needs: no partial tree is runnable and no failed teardown is
misclassified as a fresh rootless grove. It also keeps recovery local to one
deep transaction interface rather than teaching lifecycle callers the ordering
rules of Git staging, jj successor commits, or colocated index export.

The guarantee covers cooperating Grove commands and process interruption after
completed filesystem, Git-index, or VCS operations. Grove performs no ordered
`fsync` protocol, so atomic rename is a namespace-visibility seam rather than a
power-loss durability claim. Power loss, kernel failure, storage-cache loss, and
filesystems that violate their documented rename behavior remain outside the
contract.

## Considered options

- **Delete `.grove/` first and reconstruct it after a commit failure.** Rejected
  because process death in that interval exposes the same rootless shape as a
  fresh grove, while VCS history cannot prove whether teardown was confirmed or
  merely attempted. Reopen only if root absence stops being the fresh-tree
  discriminator or a durable finish receipt is introduced.
- **Keep finish recovery in the VCS administration directory.** Rejected
  because a control-directory backup would become durable workflow state outside
  the task tree and would be invisible to a reader of `.grove/`. Reopen only if
  the artifact-only lifecycle constraint is abandoned.
- **Copy rather than evacuate the finish tree.** Rejected because it doubles the
  tree, makes source/copy divergence another recovery state, and provides no
  stronger process-interruption guarantee than same-filesystem renames. Reopen
  if recovery must span filesystems that cannot rename the task entries.
- **Treat a commit command's exit status as the boundary.** Rejected because a
  lost or late result can report failure after the exact commit exists; rolling
  back then would resurrect a tree whose deletion is already recorded. Reopen
  only if each supported VCS provides an atomic, infallible commit receipt to the
  caller.
- **Require the deletion commit's parent to contain the generated finish leaf.**
  Rejected because finish allocation is working-tree-only and no task commit is
  required to snapshot it. The manifest binds the live finish authorization;
  the repository anchor and expected deletion fingerprint bind the commit.
  Reopen only if finish allocation itself becomes a required VCS commit.
- **Recursively delete the witness and then remove the empty task root.**
  Rejected because interruption can destroy the manifest or leave a witness-free
  empty `.grove/`. Reopen only if the platform supplies an atomic recursive
  directory deletion; until then the whole root is renamed before disposal.
- **Copy a committed task root to a cross-device quarantine before deletion.**
  Rejected because the copy and source removal recreate the partial-cleanup
  states quarantine exists to avoid. A workspace whose VCS control directory is
  on another filesystem refuses before mutation. Reopen if a portable atomic
  cross-filesystem move becomes available or Grove gains an equally visible
  post-commit cleanup seam that is not workflow state.
- **Run user Git hooks during the internal finish commit.** Rejected because a
  hook may mutate unrelated working-tree files even when it rejects the commit,
  and Grove cannot safely snapshot and restore arbitrary user data. Reopen only
  if hooks become side-effect-free or unrelated working-tree preservation is
  removed from the finish contract.
- **Use reserved-prefix detection without serializing tree access.** Rejected
  because validation and key allocation can race before either command creates
  its witness, while a reader can pass its witness scan before entries move.
  Reopen only if every multi-path mutation becomes one atomic filesystem action.
- **Let readers ignore a staging directory.** Rejected because an interruption
  can hide the only producer or finish leaf and let selection continue into
  unrelated work, while key allocation can reuse keys hidden inside the foreign
  directory. Reopen only if all affected operations can recover the complete
  transaction without recognizing reserved state.
- **Use an external journal or persistent lock file.** Rejected because
  `.grove/` is the only durable workflow state. The descriptor lock changes no
  artifact bytes and is released by the kernel; recovery facts stay in the task
  tree. Reopen only if that artifact-only constraint changes.
- **Lock the `.grove/` descriptor.** Rejected because it does not exist when root
  initialization must serialize and is removed by finish teardown. Reopen only
  if creation and deletion stop participating in the same task-tree mutation
  protocol.
- **Promise power-loss durability by syncing every stage.** Rejected because it
  adds a platform- and filesystem-specific persistence protocol beyond Grove's
  process-recovery requirement. Reopen if surviving power loss becomes an
  explicit supported contract.

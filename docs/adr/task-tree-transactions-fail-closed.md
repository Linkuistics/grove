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
  deletion commit, and `PREPARING-FINISH-<finish-handle>-<attempt-identity>/`
  for the window in which that witness is being built. The `.grove/` directory
  itself remains present until the exact handle-and-attempt-named,
  `.grove/`-scoped commit is proven and the whole root can atomically move to
  post-commit cleanup quarantine.

Every ordinary reader and mutator refuses while any reserved witness exists.
Only the matching recovery path is admitted, and it runs before format,
selection, liveness, kind, or normal root-absence classification. This ordering
is load-bearing: a moved producer is not missing work, an evacuated finish tree
is not a malformed or fresh grove, and files hidden beneath a witness still own
their permanent keys. Generic diagnostics name the exact witness and the
operation that can recover it.

A finish transaction makes the commit boundary explicit. Preflight repository
validation runs before evacuation when it can do so without changing state. It
opens `.grove/` itself as a no-follow directory and identity-revalidates that
descriptor against the `.grove` entry in the locked working-tree root; a symlink
or non-directory task root is refused before any child operation. The
`PREPARING-FINISH-` witness
is created *before* repository preparation so that every auxiliary the adapters
may write is already owned on disk by a named handle and attempt; recovery
discards it by aborting that preparation, and fails closed on any content it
cannot classify as its own. The transaction writes and
verifies a manifest containing the stable handle, the active session epoch's
opaque finish-attempt identity, repository-start anchor, a non-empty expected
tracked deletion fingerprint, and every root entry's type plus canonical
no-follow recursive digest, marks it ready last, and publishes the ready witness
by renaming it to `FINISHING-<finish-handle>/` in one atomic step. Only then does
it move ordinary root entries descriptor-relatively without following symlinks,
so no preparing witness ever holds an evacuated entry and an interruption before
publication is discardable. The digest is
SHA-256 over unambiguous length-delimited records:
directories cover raw-name-byte-ordered child records, regular files cover mode
and bytes, and symlinks cover link-target bytes; other entry types are refused
before mutation. Git and jj commit only the deletions at their
original paths while excluding the witness, which must remain absent from every
candidate committed tree. The generated finish leaf remains workflow evidence
in the witness; commit proof never assumes that working-tree-only leaf existed
in the starting VCS revision. The internal commit message names both the stable
handle and the launch nonce used as its finish-attempt identity, so a rootless
retry can accept only the same still-active session's result. A wholly untracked
task tree is refused before evacuation because no focused deletion commit can
record its finish. Any
ambiguous command result is classified through the repository seam from the
recorded anchor, expected delta, and exact immediate result rather than trusting
exit status or task-root absence.

If commit proof is absent, rollback is licensed only when Git `HEAD` still equals
the recorded start or jj's current working-copy commit itself still has the
recorded change identity at the same parents **and** the attempt-bound exact
teardown result is absent from the current repository view. The manifest also
records jj's exact preflight working-copy commit ID. Merely finding that change
somewhere in history is insufficient: a partial `jj commit` keeps the selected
deletion in that change and moves the unselected witness into a new successor,
so success is the exact handle-and-attempt-named parent of that successor.
After tree restoration jj must reproduce the exact preflight commit ID before
witness removal. Colocated jj backs up the user's Git index before any preflight
snapshot can export into it and restores that pre-snapshot image on an
uncommitted result.

The repository outcome remains guarded through its filesystem handoff. Recovery
revalidates immediately before and after rollback or quarantine rename. A
post-restore change leaves the witness blocking the restored tree; a post-rename
change atomically returns the quarantine to `.grove/`, and reports both that
change and the quarantine still holding the tree if the return cannot complete.
Only a second successful
gate removes the witness or begins disposal. This prevents cooperating Grove
code from acting on a stale disposition; direct mutations after the final gate
remain outside the cooperative guarantee. A reported failure exposes a live
selectable finish leaf only after every proof and restoration succeeds.

A different revision, tracked witness, restoration failure, or tree rollback
failure keeps the witness unwalkable as **Recovery pending**. The diagnostic
names the artifact that holds the blocked transaction — the in-tree witness, or
the quarantine once a failed restoration left the tree there — the recorded and
observed topology, and the two operator-restorable exits:
preserve divergent work and restore Git's exact `HEAD` or jj's exact recorded
preflight commit for rollback, or make the exact attempt-bound teardown result
immediate for forward recovery, then retry the same operation. Grove never
rewrites repository history or reconstructs the tree without one of those
proofs. This preserves unrelated work while making the fail-closed state
recoverable rather than silently terminal.

If the exact commit is proven, recovery never restores the old tree. It completes
colocated-index activation, then atomically renames the entire `.grove/` root —
witness and evacuated tree intact — into collision-resistant post-commit cleanup
quarantine in the workspace's VCS-administration control directory before
attempting descriptor-rooted recursive disposal that unlinks symlinks without
following them. Preflight requires that directory to be untracked and on the
worktree's filesystem; a cross-device workspace refuses before tree mutation
rather than using a trackable sibling or non-atomic copy. Rename failure keeps
the in-tree witness; interruption after rename leaves a complete quarantine and
an absent task root, never a partial or empty `.grove/`. That quarantine and any
VCS-administration index image are cleanup artifacts, not workflow inputs or
rootless-driver finish receipts. The helper attempts disposal immediately; a
later lease-owning driver reaps only entries carrying Grove's valid cleanup
manifest and only after confirming that no matching in-tree witness owns them.
A proven commit plus task-root absence remains successful even when best-effort
cleanup must be retried.

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
- **Automatically reset or rewrite an unexpected repository topology.** Rejected
  because Grove cannot know whether the divergent revision is user work to
  preserve or a moved teardown result. Recovery therefore stays blocked until
  the operator restores one of the manifest's two provable topologies. Reopen
  only if the repository can provide a transaction-scoped atomic result that is
  independent of mutable history.
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
- **Pin an auxiliary's staged replacement to an exactly derivable name.**
  Rejected because Grove must then claim that one name, so a foreign entry
  sitting there has to be either unlinked — deleting bytes Grove never wrote, on
  an ordinary path — or refused, which wedges the launch whose signal nonce is
  the shared finish-attempt identity. Staging names are therefore drawn from a
  fresh 128-bit nonce inside the auxiliary's reserved role-and-attempt namespace
  and carried by recorded inode. This accepts that the namespace *bounds* rather
  than *proves* ownership: a writer able to rewrite the state document in place
  can point the exchange at another entry of that shape, but only inside the VCS
  administration directory it already owns, which is where the protected Git
  index lives anyway. Reopen only if a durable ownership witness becomes
  available that such a writer cannot forge.
- **Reap the reserved staging namespaces for entries no document names.**
  Rejected because attribution is not authorship. A staged entry stranded before
  its state document is durable is attributable — its name carries the role and
  attempt that drew it — but the post-copy identity check deliberately leaves a
  *foreign* regular file at exactly such a name whenever it detects a
  substitution it can no longer identify, and the two shapes are identical on
  disk. Sweeping the namespace would therefore delete the bytes that refusal
  exists to preserve, and unlike the derivable name above nothing forces the
  removal: a drawn name is a collision gate for no later attempt. Grove accepts a
  bounded leak instead and narrows the windows that produce one, releasing the
  colocated index filter's private staging directory before the first
  publication boundary rather than on an unwind a process death never reaches.
  Reopen only if an entry's inode can be recorded before it has a name — a
  portable `O_TMPFILE`-equivalent — which makes every leftover either proven or
  absent.
- **Refuse to dispose an auxiliary whose replacement is still in flight.**
  Rejected for the same wedge: the caller that discards a failed attempt's index
  holds a snapshot of the superseded phase, and refusing strands the
  attempt-scoped names for the next run of the same launch to collide with.
  Disposal therefore settles the replacement from disk first, through the path
  recovery runs, and only then removes what settlement left. Consuming the
  canonical pair through the stale snapshot stays forbidden, and activation —
  which would otherwise install superseded index bytes as the user's own — still
  refuses outright. Reopen only if a finish attempt gains an identity that
  changes per run.
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

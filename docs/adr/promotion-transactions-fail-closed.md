# Promotion transactions fail closed

Every participating task-tree command takes a process-scoped advisory lock on an
open descriptor for the working-tree root before inspecting the tree: readers
hold a shared lock and mutators an exclusive lock. The working-tree directory is
the invariant that exists both before `.grove/` initialization and through its
finish deletion, so driver lifecycle mutations and agent tree verbs serialize
on the same seam; `BRIEF.md` remains lazy and optional. `leaf-promote-chain`
holds the exclusive lock while it stages its complete replacement node under
the reserved
`PROMOTING-<final-node-name>/` prefix, moves the producer into that directory,
prepares any plain-Git index entries to name their final paths, and lands the
node with one plain same-parent filesystem rename. The kernel releases the lock
on process termination; descriptors are close-on-exec, and driver reads release
their guard immediately after copying a selection and before launching the
configured foreground child. The lock prevents concurrent observation and key
allocation; it does not make a multi-path mutation crash-atomic, so an operation
that promises process-interruption recovery retains an explicit in-tree
transaction witness. The prefix remains
the durable witness that
every subsequent task-tree reader and mutator refuses, except promotion recovery
addressed by the exact reserved path or stable producer identity. Concurrent
commands therefore cannot allocate the same key run, and an interrupted
file-to-directory transformation can neither schedule review before its
producer, let another mutation reuse reserved keys, nor leave a complete
filesystem tree whose Git index still names the hidden transaction.

This binds because portable filesystems do not offer the file-to-differently-
named-directory replacement as one atomic operation, and Grove's normal
lenience toward foreign files would otherwise turn an interrupted staging
directory into invisible workflow state. Reserving one visible prefix and
failing closed preserves the stronger property Grove needs — no partial state is
runnable — without a journal, PID record, lock file, or persistent lock state.
The source move uses the existing Jujutsu-first rename seam. Final landing is
narrower: Jujutsu and untracked Git need only the filesystem rename; tracked Git
first rewrites the staged producer entry from its transaction path to its final
path under Git's index lock, while `PROMOTING-` still blocks the tree, then uses
the same filesystem rename. The path rewrite preserves the normal stage-0
entry's blob, mode, and flags; promotion refuses an unmerged entry before
mutation. Generated tasks remain untracked. Recovery and rollback normalise any
source, staging, or final index spelling an interruption may leave before
removing the witness.

Recovery is selected before normal source resolution, liveness, kind, or
shape validation: after the producer has moved, those ordinary gates are
supposed to be unable to see it. Generic refusing commands name only the exact
reserved path and the path-based recovery command; they neither read task
contents nor infer a producer from position for a better diagnostic. A promoter
holding the exclusive lock may scan the reserved transaction and match the
stable producer identity. A stale absolute picked path is reduced to that stable
identity. Completed-shape recognition precedes the new-promotion liveness and
kind gates, so a serialized second promoter returns the current shape
idempotently even if retirement or an insert acquires the lock first and makes
the relocated producer terminal or no longer first. A new promotion never
recomputes pick: the caller's prompt mandate is the authority, and an inserted
earlier leaf may legitimately differ from that mandated producer.

The guarantee covers cooperating Grove commands and process interruption after
completed filesystem or Git-index transactions. Grove performs no ordered
`fsync` of generated files, the index, or parent directories, so atomic rename
is a namespace-visibility seam, not a power-loss durability claim. Power loss,
kernel failure, storage-cache loss, and filesystems that violate their documented
rename behavior are outside the contract.

## Considered options

- **Use `PROMOTING-` detection without serializing tree access.** Rejected
  because validation and key allocation race before either command creates the
  marker, while a reader can pass its marker scan before a producer moves.
  Reopen only if the complete multi-path mutation becomes one atomic filesystem
  primitive.
- **Create the visible node before moving the producer.** Rejected because an
  interruption makes depth-first pick enter the node and schedule review before
  the still-external producer. Reopen only if visible partial nodes become
  structurally unpickable.
- **Move the producer first and let readers ignore the staging directory.**
  Rejected because an interruption makes pick skip the producer and continue
  into unrelated work, while key allocation can reuse keys hidden inside the
  foreign directory. Reopen only if every affected operation can derive the
  complete transaction without recognising a reserved marker.
- **Use an external journal or persistent lock file.** Rejected because Grove's
  task tree is its only workflow state and must remain intelligible without
  Grove installed. Locking the already-open working-tree directory changes no
  artifact bytes and releases with its descriptor. Reopen only if that
  artifact-only constraint changes.
- **Lock the `.grove/` root descriptor.** Rejected because it does not exist when
  root initialization must serialize and is removed by the finish transaction.
  A separate lifecycle lock would introduce an acquisition-order contract and
  still split one tree invariant across two seams. Reopen only if root creation
  and deletion stop participating in the same task-tree mutation protocol.
- **Land a tracked Git transaction with `git mv`.** Rejected because its
  filesystem rename and index update are not one atomic observation: an
  interruption can remove the on-disk witness while leaving the index to commit
  `PROMOTING-*`. Reopen only if Git provides one operation that atomically lands
  both namespace and index state; until then, prepare the index while the witness
  remains and make the final filesystem rename the last step.
- **Promise power-loss durability by syncing every stage.** Rejected because it
  adds a platform- and filesystem-specific ordering protocol beyond Grove's
  process-recovery requirement. Reopen if surviving power loss becomes an
  explicit requirement and each supported filesystem has a tested sync order.

# Promotion transactions fail closed

Every participating task-tree command takes a process-scoped advisory lock on
the open root `.grove/BRIEF.md` before inspecting the tree: readers hold a shared
lock and mutators an exclusive lock. `leaf-promote-chain` holds the exclusive
lock while it stages its complete replacement node under the reserved
`PROMOTING-<final-node-name>/` prefix, moves the producer into that directory,
and lands the node with one same-parent rename. The kernel releases the lock on
process termination; the prefix then remains as the durable witness that every
subsequent task-tree reader and mutator refuses, except promotion recovery for
the named producer. Concurrent commands therefore cannot allocate the same key
run, and an interrupted file-to-directory transformation can neither schedule
review before its producer nor let another mutation reuse reserved keys.

This binds because portable filesystems do not offer the file-to-differently-
named-directory replacement as one atomic operation, and Grove's normal
lenience toward foreign files would otherwise turn an interrupted staging
directory into invisible workflow state. Reserving one visible prefix and
failing closed preserves the stronger property Grove needs — no partial state is
runnable — without a journal, PID record, or persistent lock state. The source
move, final landing, and rollback all cross the same VCS-aware rename seam:
Jujutsu uses filesystem renames without touching Git's index, while tracked Git
ends with only the final paths staged and untracked tasks remain untracked.

The guarantee covers cooperating Grove commands and process interruption after
completed filesystem calls. Grove performs no ordered `fsync` of generated files
and parent directories, so atomic rename is a namespace-visibility seam, not a
power-loss durability claim. Power loss, kernel failure, storage-cache loss, and
filesystems that violate their documented rename behavior are outside the
contract.

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
  Grove installed. The process-scoped advisory lock changes no artifact bytes
  and releases with its file descriptor. Reopen only if that artifact-only
  constraint changes.
- **Promise power-loss durability by syncing every stage.** Rejected because it
  adds a platform- and filesystem-specific ordering protocol beyond Grove's
  process-recovery requirement. Reopen if surviving power loss becomes an
  explicit requirement and each supported filesystem has a tested sync order.

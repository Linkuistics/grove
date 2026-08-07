# One live driver owns each working tree

Bare `grove` acquires one process-scoped **driver lease** for the working tree
before it provisions, validates, reads, or mutates that grove, and holds the
lease until the loop has stopped and reported its final disposition. The lease
is an exclusive advisory lock on a control file in the OS temporary directory,
keyed by the filesystem identity of an already-open working-tree-root
descriptor. A second driver for the same working tree fails immediately;
different working trees remain independent even when they share a repository or
basename. The kernel releases ownership on every process exit, so restart still
derives all workflow position from `.grove/` and no PID file, cleanup ritual, or
repository-local owner record exists.

Each foreground launch also establishes a **session epoch** through a second
temporary control file. While holding that file exclusively, the driver records
its fresh driver nonce, the working-tree identity, and that launch's unique
`GROVE_SIGNAL_FILE` path, then releases it before spawning. An agent-side
`grove-llm` invocation that inherits the signal path takes a shared epoch guard,
checks the live driver lease and exact record, and holds the guard through its
operation. A new driver or the next session must take the epoch file exclusively
and invalidate the old record before doing tree work, so an operation already
admitted may finish but cannot overlap the next owner's tree work; later calls
from an orphaned or previous session fail before tree access. Commands run
manually without loop-control context retain their ordinary behavior. This is a
workflow-consistency mechanism, not authentication: the ambient path is not a
secret and the tree verbs remain deliberately usable by a human outside the
loop.

Every descriptor opened by the protocol is close-on-exec, and the epoch write
guard is dropped before spawn. The configured command receives no lock
descriptor, driver nonce, hidden target, kind, or grove-generation value;
it receives the existing loop-control path only. Stable work-item handles remain
tree-local identities and need no persistent generation suffix: the ephemeral
session epoch becomes invalid between launches and across finish deletion plus a
later root initialization, so an old prompt cannot adopt a newly reused handle.
Temporary control-file bytes are meaningful only while their corresponding
kernel lock is held and are overwritten on acquisition; `.grove/` remains the
only durable workflow state.

## Considered options

- **Hold the Tree access lock for the driver's lifetime.** Rejected because the
  foreground session must acquire that same seam exclusively for ordinary tree
  mutations. Reopen only if sessions stop mutating the tree they execute.
- **Let the configured command inherit the driver-lock descriptor.** Rejected
  because an opaque harness may pass it to MCP servers or other descendants that
  outlive the session, wedging the working tree after the foreground child exits.
  Reopen only if Grove owns and can close every descendant process.
- **Persist a grove-generation identifier under `.grove/` or add it to every
  stable handle.** Rejected because a per-launch epoch already rejects old
  sessions, including after handle reuse, while a durable generation would add
  opaque lifecycle state to the artifact tree. Reopen only if handles must be
  comparable across separately created groves rather than within one tree.
- **Use a PID or the existence of a lock file as ownership.** Rejected because
  PIDs are reused and files survive crashes. Reopen only on a platform without
  kernel-released advisory locks and with an equivalently race-free liveness
  primitive.

# One live driver owns each working tree

After independently provisioning the embedded methodology, bare `grove`
acquires one process-scoped **driver lease** for the working tree before it
validates configuration or reads or mutates that grove. The repository adapter
derives the control directory from the closest on-disk VCS marker, jj-first:
the current workspace's `.jj/grove/` without following its repository link, or
the canonical per-worktree Git directory named by its `.git` directory/gitfile,
never Git's common directory. It invokes no repository discovery and ignores
`GIT_DIR`, `GIT_WORK_TREE`, and other ambient selectors. Controls therefore live
in the exact workspace's VCS administration area, never in the tracked working
copy or an environment-selected temporary directory. Symlink and relative-path
aliases reach one lease; separate worktrees and workspaces remain independent.
Standard `--help` and `--version` return without provisioning, repository
discovery, or a lease.

The lease is an exclusive, nonblocking advisory lock keyed by the filesystem
device and inode of an already-open working-tree-root descriptor. Every lease
and epoch acquisition opens and locks its control file, then compares the locked
descriptor's identity with the path's current identity and retries a bounded
number of times on an open/lock replacement race. The driver holds both root and
lock descriptors until the loop has stopped, and revalidates the lock path
before every lifecycle transition and foreground launch. A second driver
fails immediately. Kernel release on return, panic, or process death makes
restart ordinary continuation; leftover bytes carry no ownership or cleanup
obligation.

Each driver writes a fresh 128-bit nonce from the operating system's
cryptographic randomness source to the lease record. Each foreground launch
also uses a fresh 128-bit random suffix for its `GROVE_SIGNAL_FILE` in the same
administration-owned control directory. Neither value is derived from a PID,
clock, address, iteration counter, or task key. Signal paths are not reused
intentionally: an occupied draw is retried, the driver removes the current path
after post-reap
invalidation, and a replacement driver removes abandoned signal files only
after it owns the lease and has exclusively invalidated the old epoch. After
cleanup there is no durable tombstone, so cross-restart nonce or path reuse is
not literally impossible; the accepted probability is at most one in `2^128`
per independent draw. This statistical freshness is the explicit cost of
keeping grove generation out of durable workflow state.

The stable per-workspace **session epoch** control file binds the lease nonce,
working-tree identity, and current signal path. The driver writes it at three
points, each under a separately scoped exclusive guard: inactive immediately
after lease acquisition, active immediately before spawn, and inactive after
the child is reaped and before interpreting its signal. Every exclusive guard is
released before another epoch or tree operation begins and before spawn. Every
descriptor is close-on-exec.

An ambient agent-side `grove-llm` tree operation takes a shared epoch guard,
checks the exact worktree, signal path, and nonce, and probes the lease with a
separate nonblocking exclusive-lock attempt. A successful probe is closed and
released immediately and means no driver is live; contention plus a matching
lease record is the liveness hint. The operation retains its shared epoch guard
through tree access, which closes the probe's race: if the driver dies just
after the probe, a replacement driver cannot invalidate the epoch until the
admitted operation finishes. A wrong worktree receives its own location
diagnostic; inactive, malformed, unlocked, or mismatched epochs receive a stale-
session diagnostic. Manual commands without loop-control context retain their
ordinary behavior. Shared-guard acquisition is the admission boundary: an old
call admitted before exclusive invalidation may finish and block handoff; calls
beginning after invalidation fail against the inactive record or new nonce.

Every epoch acquisition first tries without blocking, emits one diagnostic on
contention, and waits for a fixed internal 30-second handoff bound. A timeout
performs no tree access or epoch rewrite. In particular, if an orphaned
`grove-llm` process retains a shared guard after its foreground parent is killed,
post-reap invalidation times out and the driver stops `blocked` without
interpreting the completion signal or launching another session. A restart may
continue once that already-admitted operation releases its guard. The bound,
clock, control-path resolver, and randomness source are internal test seams, not
user configuration. A test lock/filesystem backend with post-open/post-lock
barriers and an event trace makes the protocol races and guard lifetimes
deterministic without widening the production interface.

This protocol provides workflow consistency among cooperating Grove processes,
not authentication. It prevents an old session from resolving, mutating, or
signalling through `grove-llm` after epoch rotation, including after finish
deletion and handle reuse. It cannot prevent a stale process from directly
editing files, committing, or writing a known signal path outside `grove-llm`.
Nor does Grove defend against another process deleting or replacing files in
the VCS administration area; that is repository-control corruption, and no
claim is made that open/lock identity revalidation survives unlink/recreate
outside an acquisition window.
The fixed lease and epoch files are untracked coordination locations whose bytes
have meaning only while their kernel locks are held; `.grove/` remains the only
durable workflow state.

## Considered options

- **Keep the status quo with no lifetime owner.** Rejected because two bare
  drivers can select and launch the same work or consume one another's completion
  signals. Reopen only if launch becomes externally serialized by a stronger
  owner whose state Grove can verify.
- **Hold the Tree access lock for the driver's lifetime.** Rejected because the
  foreground session must acquire that seam exclusively for ordinary tree
  mutations. Reopen only if sessions stop mutating the tree they execute.
- **Put controls in `env::temp_dir()` or another temporary directory.** Rejected
  because `TMPDIR`, per-user runtime directories, containers, and private temp
  namespaces let two drivers derive different paths, while routine temp cleanup
  can unlink a live locked inode and admit a second owner. Reopen only if the OS
  supplies a path namespace shared by every process that can operate on the
  working tree and guarantees live entries are not removed.
- **Put a control file in the tracked working tree.** Rejected because jj would
  snapshot it into the working-copy commit, while plain Git would require ignore
  mutation and still expose it to broad staging. The per-workspace VCS
  administration directory supplies the same shared scope without artifact
  pollution. Reopen only if Grove supports a non-VCS working tree with no
  equivalent administration location.
- **Let the configured command inherit the driver-lock descriptor.** Rejected
  because an opaque harness may pass it to descendants that outlive the session,
  wedging the working tree after the foreground child exits. Reopen only if Grove
  owns and can close every descendant process.
- **Persist a grove-generation identifier under `.grove/` or add it to every
  stable handle.** Rejected because per-launch epoch rotation already rejects old
  `grove-llm` access, including after handle reuse, while a durable generation
  would add opaque lifecycle state to the artifact tree. Reopen only if handles
  must be comparable across separately created groves.
- **Use a PID or the existence of a control file as ownership.** Rejected because
  PIDs are reused and files survive crashes. Reopen only on a platform without
  kernel-released advisory locks and with an equivalently race-free liveness
  primitive.

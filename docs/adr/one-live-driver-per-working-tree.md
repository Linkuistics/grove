# One live driver owns each working tree

Bare `grove` acquires one process-scoped **driver lease** for the working tree
before it validates configuration or reads or mutates that grove. It does not
derive where that lease lives. The **version control seam**
(`crates/jj-workspace`) owns that: grove asks the resolved workspace for a
control directory under the namespace `grove`, and what comes back is guaranteed
to be inside that exact workspace, untracked, shared with no other namespace,
and created if absent. Grove supplies only the namespace — the one thing the
seam cannot know, because *where a lease file may live* is not sayable without
naming whose lease it is.

**The lease is handed a resolved workspace; it does not resolve one.** Since
`loop-crate-driver-k22` the binary resolves the working tree once and passes that
value to both `DriverLease::acquire` and `grove_loop::run`, so there is one
resolution behind the lease, the delta search, the `${repo}` expansion and the
prompt's stated version control — and no second derivation that could disagree
with it. The lease is then moved into the loop, so it is released exactly when
the loop that justified holding it returns.

The seam resolves the workspace by walking the filesystem for the closest `.jj/`
and canonicalising it. It invokes no repository discovery, does not follow a
secondary workspace's repository link, and removes `GIT_DIR`, `GIT_WORK_TREE`
and the other ambient selectors from every child it spawns. Controls therefore
live in the exact workspace's administration area, never in the tracked working
copy or an environment-selected temporary directory. Symlink and relative-path
aliases reach one lease; separate workspaces remain independent. Resolution also
creates the control directory, so a working tree that is not jj-enabled and a
`.jj/` that cannot hold a directory both stop the invocation before it can
create or drive a task tree; that the directory is *writable* is proved by the
lease file itself, at the moment it is opened, rather than by a probe whose
answer could already be stale. Standard `--help` and `--version` return without
workspace resolution or a lease.

The lease is an exclusive, nonblocking advisory lock keyed by the filesystem
device and inode of an already-open working-tree-root descriptor. Every lease
and epoch acquisition opens and locks its control file, then compares the locked
descriptor's identity with the path's current identity and retries a bounded
number of times on an open/lock replacement race. The driver holds both root and
lock descriptors until the loop has stopped, and revalidates the lock path
before every lifecycle transition and foreground launch. A second driver
fails immediately. Kernel release on return, panic, or process death makes
restart ordinary continuation while `.grove/` still exists; after a successful
finish deletion and epoch handoff, a later bare invocation is a fresh grove.
Leftover bytes carry no ownership or cleanup obligation.

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
`grove-llm` process retains a shared guard, post-reap invalidation times out and
the driver stops `blocked` without interpreting the completion signal or
launching another session. *Orphaned* is narrower than it once was: the
escalation signals the session's whole process group, so a command the session
itself launched is reaped with it (*[the launched child is a
job](./the-launched-child-is-a-job.md)*). What can still hold the guard is a
process that was never in that group — one started from another session, another
terminal, or by hand. A restart may
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
The fixed lease and epoch files, and the session witnesses defined below, are
untracked coordination locations whose bytes have meaning only with live lock
evidence; `.grove/` remains the only durable workflow state.

Consequently, at a driver lifecycle transition an absent `.grove/` is always a
fresh-tree fact, never an implicit finish receipt. If a finish session
successfully commits deletion and the driver dies before observing
`complete --done`, the next bare invocation initializes a new grove. Neither a
matching teardown commit nor an abandoned signal file can distinguish recovery
intent from an intentional new workstream without adding a second user input or
durable state. A configured child that exits without a signal likewise retains
the ordinary no-signal disposition; the driver does not infer `done` from
task-root absence. A `finish-commit` whose own result is lost recovers nothing
here either: there is no attempt identity in the commit message, no proof that a
given commit was this attempt's, and no retry path that reads one. The version
control system owns the transaction, so a lost result is read from the operation
log and rerun or undone there. Absence alone never licenses `done`. An operation
already admitted under the crashed driver's epoch may also delay replacement
invalidation; an orphan outside the dead session's process group that holds the
shared guard to the handoff bound makes that replacement stop `blocked` without
creating a new task tree. Once the guard
releases, a later invocation can invalidate the epoch and initialize the fresh
tree. That tree may reuse keys such as `plan-k1`; epoch rotation, rather than
global key uniqueness, rejects the old cooperating session's `grove-llm`
operations.

## Read-only activity observation

An observer reports a current mandate only with a **session witness** in
addition to matching lease and epoch records. A matching active epoch plus a
contended lease is an admission liveness hint, not enough to establish which
launch is running: a replacement driver acquires the lease before it may invalidate the
predecessor's epoch, and deliberately preserves the old lease bytes during that
wait. A viewer must not describe that predecessor as RUNNING.

The witness adds observation evidence without changing authority. The driver
lease still serializes drivers; the epoch still admits agent operations using
the worktree, lease nonce and signal path. A viewer neither obtains an admission
guard nor probes the driver lease lock. Its witness probe is shared, so
concurrent observers cannot create the exclusive contention that means a live
driver holds that witness. A successful witness probe is released immediately,
before validation or other I/O. Agent admission retains its existing protocol.

Shared probes rely on the compatible shared/incompatible exclusive semantics
documented by [Apple's flock manual](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flock.2.html)
and the [Linux flock manual](https://man7.org/linux/man-pages/man2/flock.2.html).
The inference that contention names this driver additionally depends on Grove's
rule that no other participant takes the witness exclusively.

### Mandate and lifetime

Selection retains an open task-root directory descriptor, acquired and checked
against the selected snapshot under its tree read guard. Its device/inode pair
is the **tree lifetime**, distinct from the working-tree-root identity in the
lease. The driver retains this pin through the session witness's lifetime and
checks it again before publishing the mandate. A mismatch before publication
stops that launch as a changed-tree error; no stale selection is rebound to the
new tree. No tree or epoch guard survives across spawn.

Active epoch records carry an optional, versioned observation extension:
permanent key, launch-time handle and kind, task-root identity, and the witness's
namespace-local name and descriptor identity. The extension is bound to the
record's existing lease nonce and signal path. The handle's key must agree with
the explicit key. Missing or unsupported observational fields cannot weaken
validation of the mandatory admission record and cannot establish RUNNING.
Admission does not require the extension, including when observation setup fails.

Keeping the task-root descriptor open prevents its inode being reused while its
mandate can still be observed as current. A viewer joining after root replacement
can therefore reject a reused key without having seen the previous tree. The
pin adds no persisted generation and takes no tree lock. Replacement after
publication leaves the mandate attached to the old lifetime; it does not confer
authority over the replacement or change the existing admission rules.

### Witness publication and release

Each attempted launch allocates a new witness file in Grove's control namespace
using an independent OS-random 128-bit suffix and exclusive creation. Occupied
draws are retried with the same bounded policy as fresh signal allocation.
The driver locks the empty regular file exclusively before publishing its
identity in the pre-spawn epoch. This lock is held by the driver alone, on a
close-on-exec descriptor. Its name is never deliberately reused. The accepted
random-collision limit is the same as the channel's, without tombstones.

The generic runner exposes launch events to its caller: **Started** after a
successful spawn and **Reaped** when it confirms that child's reap, including
escalated termination. These synchronous parent-side notifications introduce no
Grove vocabulary or child-side acknowledgement. Ordinary callers can run with
no observer. Notifications are infallible and do not change launch disposition;
Grove's callbacks take no epoch/tree lock and perform no waiting operation.

On Started, the driver writes the exact eight-byte marker `started` followed by
a newline to the previously empty witness. This is the file's only publication:
it is never rewritten to describe another phase, launch or item. Empty or
incomplete bytes cannot mean started; only the complete exact marker does. A
reader need not assume write atomicity. On Reaped, the driver releases the
witness immediately, before terminal recovery, exclusive epoch invalidation or
signal interpretation. Writing the completion signal and retiring the task do
not release it. Failed spawn publishes no marker and releases its prepared
witness before post-attempt invalidation.

The lease owns the witness until those events release it, and releases the
witness before releasing driver ownership on every orderly drop path. A
supervision error without confirmed reap must not invent a Reaped event or drop
the witness early through a helper's return. Driver exit, unwind or death
releases both lease and witness through descriptor lifetime; child processes
cannot keep either lock alive after exec. The driver keeps the tree pin for at
least as long as that witness.

Observation-only allocation failure leaves activity Unavailable. Publication
failure leaves activity unverified: an empty file or valid marker prefix remains
Busy, and invalid bytes are Unavailable, until reap or driver exit. Either
failure emits a diagnostic without changing a successfully launched session's
authority or outcome. An epoch whose extension could not be prepared is still
valid for admission. Failure to write the mandatory epoch remains a launch
failure under the existing protocol. Witness cleanup happens after epoch
invalidation; a replacement removes abandoned witnesses only after it owns the
lease and has invalidated the old epoch. Cleanup failure leaves harmless bytes
and cannot change completion. No observer creates, cleans or repairs controls.

### One bounded observation

The VCS seam provides read-only discovery of an existing namespace in the exact
observed workspace, sharing path derivation and namespace validation with the
creating operation. It neither creates directories nor follows the secondary
workspace's repository link or an ancestor's workspace to find activity. No jj
command, launch configuration or ambient session context participates.

The loop's typed observer uses this protocol:

1. Open existing controls read-only, nonblocking and close-on-exec. Require
   regular files for records and a directory for the namespace. Bound each
   lease/epoch read to 64 KiB; the witness accepts exactly the eight-byte marker
   and rejects extra bytes. Resolve the witness only from a plain basename
   inside this namespace, never an arbitrary path from the record.
2. Try a shared epoch guard without waiting. Contention returns activity Busy.
   An absent exact workspace, namespace or lease means Idle at that sample.
   When a lease file exists but the epoch is missing, unreadable or malformed,
   activity is Unavailable; bytes alone cannot establish that the driver ended.
   If shared acquisition fails, ordinary nonblocking tree observation may still
   proceed.
3. With the epoch guard held, validate the worktree and the matching mandatory
   lease/epoch records without locking the lease file. A matching inactive epoch
   means Idle: no new launch can activate it while this shared guard is held.
   An active epoch needs its observation extension. Invalid or mismatched
   records mean Unavailable. A valid witness's exclusive ownership supplies the
   live-driver evidence, because only its owning lease may hold that witness.
4. Open the named witness and compare its descriptor/path identity with the
   published identity. Missing or mismatched evidence means Unavailable.
   Prepare its bounded read, but do not finalize Running yet.
5. Acquire the tree's shared guard only by a nonblocking attempt, after the
   epoch guard where one exists. Pin and validate the observed task-root
   identity with its snapshot before the final witness probe. A busy, vacant
   or invalid tree still permits a runtime summary, but no row attachment.
6. Probe the witness through an independent descriptor with a nonblocking
   shared lock attempt. Success is released immediately and means Idle for a
   matching witness, regardless of leftover marker bytes. Contention plus the
   exact started marker establishes Running; a locked empty file or proper
   marker prefix means Busy; other bytes/errors mean Unavailable. Other viewers'
   shared probes cannot cause contention. No admission operation or replacement
   driver acquires this old witness exclusively.

Return separate typed tree and runtime results even if one is unavailable.
After the caller's short capture, release tree then epoch guards, before any
second capture, rendering or input wait. Retain the accepted tree identity pin
with the copied observation. Pinning before the final probe is load-bearing:
the driver can release its old root pin on reap or death, even while the viewer
holds a shared epoch guard. A probe done first could be joined to a later root
whose inode was reused after that release. The observed root's pin must already
exist when Running is established.

Every control acquisition/probe checks open descriptor identity against the
current path, with at most the existing eight identity-race attempts and no
sleep or blocking fallback. A successful witness probe releases its lock
before any further work. The observer never returns that lock to its caller.
Retained root pins hold no advisory locks. Multiple viewers may share a short
epoch/tree read; none holds an epoch guard while waiting for a busy tree.

Running is evidence at the witness probe, not a promise until the next frame.
If the driver dies before that probe, its witness lock is gone, including while
a replacement holds the old lease bytes. If it dies after the probe, the next
observation detects the loss. A shared epoch guard prevents a new mandate being
published during the capture, but never keeps the old driver or witness alive.
Comparing two bounded captures can reject observed change; it does not promise
atomic observation of non-cooperating filesystem edits. The existing exclusion
of arbitrary administration-area corruption still applies.

This costs one ephemeral file and descriptor per launch and two generic runner
events. It avoids turning admission's deliberately preserved predecessor bytes
into false runtime status. The [viewer spec](../specs/item-status.md) owns the
presentation and acceptance scenarios, including real process-death, pre-spawn,
reap-before-handoff and tree-replacement controls.

## Considered options

- **Read RUNNING from the existing epoch and lease alone.** Rejected because
  an observer cannot distinguish the predecessor from a replacement waiting
  for epoch handoff. Reopen only if the ownership protocol no longer preserves
  predecessor records or supplies an equivalent kernel-backed launch witness.
- **Publish RUNNING immediately before spawn.** Rejected because a failed
  spawn would briefly claim a running session. Reopen only if the product
  meaning changes to include attempted launches.
- **Let the child publish or retain the witness.** Rejected because an opaque
  harness need not acknowledge startup and can outlive its driver; the displayed
  mandate belongs to the live driver. Reopen if Grove's ownership model moves to
  the launched job rather than its supervising driver.
- **Make the viewer join independent tree and activity reads.** Rejected
  because callers would each own the epoch/tree ordering and lifetime check.
  Reopen if independent consumers need uncorrelated runtime telemetry rather
  than an item observation.
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
- **Put a control file in the tracked working tree.** Rejected because jj
  snapshots it into the working-copy commit on the next command, so the control
  becomes an artifact of the workstream it is coordinating. The workspace's own
  `.jj/` supplies the same shared scope without that pollution. Reopen only if
  Grove supports a working tree with no equivalent administration location.
- **Take the administration directory from the seam and name the control files
  inside it.** Rejected because `driver.lease` and `session.epoch` are generic
  names in a directory the version control system owns and may extend, so the
  collision is one jj release away and would be silent. Asking for a *namespace*
  moves the guarantee into the seam, where it can be kept. Reopen only if the
  version control system reserves a consumer area of its own.
- **Let the configured command inherit the driver-lock descriptor.** Rejected
  because an opaque harness may pass it to descendants that outlive the session,
  wedging the working tree after the foreground child exits. Reopen only if Grove
  owns and can close every descendant process.
- **Persist a grove-generation identifier under `.grove/` or add it to every
  stable handle.** Rejected because per-launch epoch rotation already rejects old
  `grove-llm` access, including after handle reuse, while a durable generation
  would add opaque lifecycle state to the artifact tree. Reopen only if handles
  must be comparable across separately created groves.
- **Persist a finish tombstone in the VCS administration area.** Rejected
  because lease, epoch, and signal files are process coordination whose bytes
  cease to carry workflow meaning when their locks are released; making one a
  cross-driver completion receipt would put durable workflow state outside the
  task tree. Reopen only if artifact-only lifecycle state is abandoned.
- **Use VCS history as a rootless-driver finish discriminator.** Rejected
  because the same teardown history precedes both a recovery attempt and a
  deliberate new grove, so history proves what happened but not what the current
  invocation is for. Reopen driver-side inference if bare `grove` stops being the
  sole lifecycle input or a rootless invocation no longer means fresh start.
- **Infer `done` when a finish target exits without a signal and `.grove/` is
  absent.** Rejected because absence does not carry the finish session's
  disposition or attest human confirmation; it would make the no-signal path
  report a result the configured child did not send. Reopen if completion
  signaling stops being the sole disposition channel.
- **Use a PID or the existence of a control file as ownership.** Rejected because
  PIDs are reused and files survive crashes. Reopen only on a platform without
  kernel-released advisory locks and with an equivalently race-free liveness
  primitive.

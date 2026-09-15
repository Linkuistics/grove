# item-status-k7

**Reviews:** item-status-k6

## Goal

Adversarially review the item-status process-death correction before
`item-status-k4` cuts implementation work. Find where the proposed runtime
protocol or its evidence fails the root brief's truthful RUNNING and
no-rebinding contract.

## Context

Read the producer's commit by its stable handle, the current
`docs/specs/item-status.md`, and the driver-lease ADR it cites. The spec owns
protocol and typed results; the ADR owns rationale, primary-source evidence and
trade-offs. Earlier review `item-status-k3` and integration `item-status-k5`
explain the correction's origin but do not establish its correctness.

Inspect arbitrary relative release of the private witness and directory pin,
observation opened after replacement, inode/key reuse, the final probe order,
and replacement waiting with predecessor epoch bytes. Challenge the assumption
that exclusive directory contention belongs to the witnessed launch, including
whether reserving this lock location is compatible with required participants.
Check that the supported filesystem boundary is explicit and implementable,
and that the Linux/Darwin sources support each lifetime claim actually made.

The producer ran a temporary macOS primitive probe, not the future observer or
a Linux implementation test. Assess the proposed real-process and forced-reuse
controls on their own merits: they must discriminate the claimed mechanism from
blanket unavailability. Keep the short runtime-only epoch guard, its explicit
suspension trade-off, and the previously integrated rendering, selection and
workspace-identity requirements in scope for regressions.

## Done when

Findings or an explicit no-findings result are recorded against the producer's
commit with source locations and reasons. Any integration follows this review
before `item-status-k4`; its charter points to this review rather than copying
the findings as obligations.

## Findings

Reviewed at commit `ab1b760685337392feb30aef4e5b3cd29171f9bc` (`item-status-k6`):
`docs/specs/item-status.md`, the observation sections of
`docs/adr/one-live-driver-per-working-tree.md`, the glossary edit in
`CONTEXT.md`, and the producer's running log, against the root brief and the
source anchors named in Context. `path:line` citations are that commit's. The
kernel sources were fetched at the tags the ADR cites (Linux `v6.12`, XNU
`xnu-11215.1.10`) and line numbers below are from those files, not from memory.
Each finding names its anchor, the evidence, and the classification this
reviewer proposes; the integration grades every one itself and may reject any.

### What held

The attack axes in Context were checked against the spec, the ADR and the
source. None produced a finding on its own.

- **Arbitrary relative release of the private witness and directory pin.** The
  case table at `docs/specs/item-status.md:241`–`:246` is exhaustive over the
  four lock states, and each consequence follows from one fact: contention on
  the observer's own pin descriptor means an exclusive holder exists on *that
  inode now*. Within the protocol the only such holder is the witnessed launch's
  directory witness. A replacement is excluded because it may take neither
  witness before exclusive invalidation (`:91`–`:94`), which the observer's
  shared epoch guard defers. The same driver's next launch is excluded because
  post-reap invalidation is exclusive and precedes the next selection
  (`crates/grove-loop/src/loop_driver.rs:299`–`:301`, then the loop head at
  `:224`). Every other Grove participant locks the directory *containing* the
  root, never the root itself: `crates/ordinal-fs-tree/src/fs/lock.rs:1`–`:7`,
  `crates/ordinal-fs-tree/src/fs/read.rs:409`, `crates/grove-loop/src/task_tree.rs:251`,
  and the viewer's existing pin takes no lock at all
  (`crates/grove-tui/src/observation.rs:23`–`:41`). The ADR's claim at `:212`
  that sessions can still mutate and remove the tree is therefore true, and
  reserving the task-root lock is compatible with every required participant.
- **Observation opened after replacement, and inode/key reuse.** While the
  driver's pin is open the old inode cannot be re-issued, so a numerically equal
  replacement cannot exist; once the pin has closed its lock is gone, so a
  numeric match without contention is exactly the *unverified → Busy* row.
  Neither RUNNING nor NEXT exclusion is reachable from a reused key.
- **The final probe order.** Directory first, then file, is the right order: a
  contended directory at step 4 proves the observer's pinned object is the
  launch's, and a contended file at step 5 proves teardown had not finished
  when liveness was read. Orderly release closes the file first (`:148`–`:151`),
  so a released directory implies a released file and orderly paths never yield
  Busy; only process death can, and that is the row it is meant for.
- **Replacement waiting with predecessor epoch bytes.** The observer sees the
  old active record and the old lease bytes, both locks died with the
  predecessor, the replacement can take neither before invalidation, and both
  shared probes succeed: Idle, never RUNNING.
- **Guard and lock order.** The driver takes the directory witness nonblocking
  under the shared tree guard; the observer takes everything nonblocking and
  never holds tree and epoch guards together; sessions take only the tree lock.
  No blocking acquisition can wait on the new lock, so no deadlock is added.
  The accepted suspension window now includes a momentary shared lock on the
  task root, but nothing else contends for it, so it is dominated by the
  already-accepted epoch-guard trade-off.
- **The kernel claims themselves.** Each behavioural claim in
  `docs/adr/one-live-driver-per-working-tree.md:183`–`:200` holds in the cited
  tags: Linux `close_files` walks the descriptor bitmap upward and calls
  `cond_resched()` between closes (`fs/file.c:398`–`:428`, the call at `:419`);
  Linux `__fput` calls `locks_remove_file` before `dput` and `mntput`
  (`fs/file_table.c:405`–`:445`, at `:423`, `:439`, `:442`); XNU's descriptor
  teardown walks downward (`bsd/kern/kern_descrip.c:1051`, inside
  `fdt_invalidate` at `:1018`); XNU `vn_closefile` issues `VNOP_ADVLOCK(F_UNLCK,
  F_FLOCK)` before calling `vn_close` (`bsd/vfs/vfs_vnops.c:1831`, `:1846`); and
  `vn_close` drops the ordinary vnode's reference after `VNOP_CLOSE` (`:836`,
  `:849`). The same-object argument the design rests on is supported. What is
  wrong is where the ADR says these lines are — F1.
- **The controls.** The two layers discriminate the mechanism from blanket
  unavailability: real processes show both probes contended before the kill and
  both released after it; the barrier layer forces the two close orders and the
  identity reuse, with mutation checks that must flip and a running-launch
  positive that must stay green. See F4 for one control whose evidential scope
  is narrower than its sentence suggests.
- **Regressions.** None. The diff removes only the unresolved-boundary section
  and the sentences it superseded. The short runtime-only guard (`:181`,
  `:223`–`:227`), the lease-owned launch value (`:79`–`:83`, `:148`–`:157`),
  duplicate-key refusal (`:274`–`:282`), device/inode workspace matching
  (`:196`–`:201`), nonblocking witness allocation (`:124`–`:127`), help
  (`:382`–`:385`) and the concrete styles (`:340`–`:351`) are intact.

### item-status-k7 F1 — Four of the five kernel-source anchors do not point at the behaviour they are cited for (medium-low)

Anchor: `docs/adr/one-live-driver-per-working-tree.md:184` (`close_files`,
`fs/file.c#L370-L421`), `:187` (`fdfree`, `kern_descrip.c#L994-L1043`), `:195`
(`__fput`, `fs/file_table.c#L378-L416`), `:198` (`vn_closefile`,
`vfs_vnops.c#L1717-L1750`), `:200` (`vn_close`, `vfs_vnops.c#L753-L809`).

Evidence, from the files at the cited tags:

- `fs/file_table.c#L378-L416`: `__fput` begins at `:405`; the range ends before
  `locks_remove_file` (`:423`) and `dput` (`:439`), so it shows neither half of
  the ordering the sentence asserts.
- `kern_descrip.c#L994-L1043`: there is no `fdfree` in `xnu-11215.1.10`; the
  range covers the tail of `fdt_fork` and the head of `fdt_invalidate`
  (`:1018`), and the downward loop is at `:1051`, outside it.
- `vfs_vnops.c#L1717-L1750` lies inside `vn_ioctl` (`:1673`); `vn_closefile`
  is at `:1814`–`:1852`, with the unlock at `:1831` and the `vn_close` call at
  `:1846`.
- `vfs_vnops.c#L753-L809` covers `vn_access` (`:772`) and the first lines of
  `vn_close` (`:795`); the ordinary-vnode reference drop is at `:849`, outside it.
- `fs/file.c#L370-L421` starts inside `dup_fd` but does reach the `close_files`
  loop (`:398`–`:421`); it is loose rather than wrong.

The offsets are not a constant shift, so this is not one stale tag; the ranges
were not read from the cited files. The producer's log records that its link
and anchor check passed, but that check covered local anchors, not external
line ranges. The ADR's death-path section exists to carry primary-source
evidence, and a reader who follows `#L1717-L1750` lands in an ioctl switch and
has reason to distrust the whole section. The claims are right; the evidence as
written cannot be verified from what is cited.

Proposed classification: real issue, mechanical. Re-anchor each link to the
lines named in *What held* above, rename `fdfree` to `fdt_invalidate`, and
record in the log that the external ranges were opened, not only the local
anchors.

### item-status-k7 F2 — The ADR attributes to viewers a preparation failure the protocol excludes (low)

Anchor: `docs/adr/one-live-driver-per-working-tree.md:223`–`:225` ("Concurrent
viewers can briefly make driver preparation fail conservatively, leaving
activity Unavailable for that launch").

Evidence: a viewer holds a shared lock on the task root only inside step 4
(`docs/specs/item-status.md:204`–`:210`), which runs only under a shared epoch
guard whose record is *active* with a matching extension (`:201`–`:203`). The
driver takes the directory witness before publication (`:76`–`:86`) and after
the predecessor record has been exclusively invalidated (`:91`–`:94`; the ADR's
own three write points at `:61`–`:65`). Between invalidation and activation the
record is inactive, so a viewer returns Idle at step 3 without probing; a viewer
that took its guard before invalidation holds it until step 6, and the
invalidation the acquisition follows cannot complete until then. The two
windows cannot overlap. The residual the sentence should name is the foreign
holder the next paragraph already describes, which is also what the scenario at
`:440` exercises.

Proposed classification: contract stated unclearly. Say that observers cannot
contend with preparation, and why; if planning wants it pinned, a continuous
viewer across several launches never producing a viewer-caused Unavailable is
the scenario.

### item-status-k7 F3 — "Unsupported semantics produce Unavailable" claims a detection the protocol does not specify (medium-low)

Anchor: `docs/specs/item-status.md:253`–`:255` ("An observation backend must
provide the stated native file/directory lock and open-object lifetime
semantics; unsupported semantics produce Unavailable, never an identity-only
fallback"); `:96`–`:100`; `docs/adr/one-live-driver-per-working-tree.md:203`–`:205`.

Evidence: the only failures the protocol can see are lock calls that return an
error, which `:96`–`:100` and step 4's "other errors" already route to
Unavailable. A backend that accepts `flock` but does not enforce exclusion is
invisible to every step as written: the driver's two exclusive acquisitions
succeed, the extension is published, the observer's directory probe succeeds
(unverified) and its private probe succeeds, and step 5 says success "means
Idle regardless of leftover marker bytes" (`:213`–`:215`). The sample is Idle,
and NEXT names the running item. That is not a false RUNNING and it is the
pre-existing lease protocol's own boundary, but it is not Unavailable, so the
sentence promises what an implementer cannot deliver and the task file's
question — is the boundary implementable — is answered *only for the errno
case*. A cheap positive check exists if the design wants the sentence to be
true: at preparation, after taking the exclusive directory witness, open a
second description of the same directory and attempt a nonblocking shared
`flock`; on the required semantics it must fail (the repository already relies
on two descriptions not sharing a lock, `crates/grove-loop/src/task_grow/tests.rs:1456`–`:1483`),
and a success means the backend is outside the boundary, so the driver
publishes no extension and activity is Unavailable for that launch.

Proposed classification: contract stated unclearly. Either narrow the sentence
to lock calls that report an error and state that a silently non-exclusive
backend is outside the boundary and yields Idle, or adopt the preparation-time
self-check and keep the sentence. Say which in the spec, and mirror the
trade-off in the ADR's boundary paragraph.

### item-status-k7 F4 — The real-process replacement control evidences the identity comparison, not the pin (low)

Anchor: `docs/specs/item-status.md:458`–`:460` ("Rename/remove and recreate
the root while the original witnesses are held; observers opened afterwards
must see the replacement as a different lifetime"); `:112` ("Keeping the
task-root descriptor open prevents its inode being reused").

Evidence: on APFS inode numbers are never re-issued, and on tmpfs they come
from a monotonic counter, so on the platforms the controls run on a fresh
directory is a different lifetime whether or not any pin is held. The control
therefore passes against a driver whose pin does nothing; it discriminates the
observer's comparison from blanket unavailability, which is useful, but it
cannot discriminate the pin's mechanism, and `:465` already accepts that no
host allocator can be made to reuse an inode on demand. The pin's non-reuse
property is a kernel open-object fact of the same kind as the flock ordering,
yet unlike that ordering it is asserted at `:112` with neither a citation nor a
note that no control reaches it.

Proposed classification: contract stated unclearly. State the control's scope
in one sentence, and either cite the open-object lifetime evidence for `:112`
alongside the flock ordering or say plainly that the pin rests on the same
sources and is not separately tested.

## Decisions (running log)

F1 is actionable and F2 to F4 are cheap to settle beside it, so an integration
step is warranted. Its leaf is inserted ahead of the waiting `item-status-k4`
planning step, the first later sibling entry in this directory with live work,
so the integration opens the spec and ADR at the lines cited here before
planning reads them. The integration's body names this review by handle and
carries none of the findings, so rejecting one stays an ordinary outcome.

No in-session reviewer was spent; this session is the adversarial read. The
kernel files were fetched from the cited tags (raw GitHub, with a mirror for
one file GitHub rate-limited) and every line number above was read from those
files; the protocol analysis rests on the spec's and ADR's own stated rules
plus the lock sites enumerated from the source, not on the producer's probe.

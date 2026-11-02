# ravel-skill-bounds-k64

**Reviews:** ravel-skill-bounds-k57

## Goal

Read the bounds, deadline and cancellation code of Ravel commit
`52d856978b24` adversarially. Read it against `docs/specs/skill-discovery.md`
at `822c0ed2` and against the clarifications that the commit adds to it.
Produce findings, not fixes. The parent `cba154857d41` is the
base of the diff.

## Specific doubts

- `crates/ravel/src/skills/dir.rs` and `snapshot.rs` replace `sync_all` for
  each file with a plain `fsync`, and add one `F_FULLFSYNC` of the staging
  directory before each publication rename. Does that keep every copied
  byte, the manifest and the descriptor durable before the rename? Consider
  Apple's documentation of `F_FULLFSYNC`, and APFS volumes with more than one
  device. On Linux, the barrier is an `fsync` of the staging directory only.
  The nested directories have none. Did the old code give more?
- `skills/watch.rs` carries a refusal inside an `io::Error` as `Stopped`, and
  `watch::stopped` takes it out at three sites. Can a stop reach another
  conversion site, so that it reports as `skill_cache_unavailable`,
  `skill_resource_invalid` or `skill_cache_corrupt`? Read each caller of
  `read_tree`, `seal_contents` and `Expected::holds`.
- `Watch::overrule` lets a consumed signal win over any refusal of the
  resolution, and `launch::prepare` applies it to the prepared prompt.
  `run::settle` keeps a refusal of stage `skills` that carries a signal in
  place of `session_cancelled`. Is the selected signal of `finalise` always
  the one in that refusal? Can a cleanup error or a surviving group be lost
  on that path? Compare the outcome order of
  `docs/specs/result-job-completion.md`.
- `skills/command.rs` now installs the handlers for `skills resolve` and
  finalises them before it writes its output. Does a failure of `begin` or
  of `finalise` leave a signal blocked, or a disposition changed, when the
  process ends without a signal? Is `signal_state::recorded` the right
  refusal before the handlers?
- The three configured counts refuse before the temporary Git refusal. The
  search-directory count is the sum of the configured roots. A refusal of a
  streamed count reports "at least" the value at the stop. Does each
  refusal report the actual value, the bound and the subject, as the
  specification requires?
- The mandatory-file bound has one site, `check_mandatory`, after the copy.
  A mandatory file of more than 16 MiB therefore refuses as
  `resourceBytes`. Is that acceptable against "One mandatory file"?
- `crates/ravel/tests/skills_bounds.rs` stops the resolver with SIGSTOP when
  its `capture-` staging exists and has no manifest. Can a fast machine,
  a Linux run or a loaded suite make it miss the capture, stop it after its
  walk, or hang? The first full `task check` of this leaf failed once in an
  unrelated worker-admission timing test. Do the byte-bound fixtures, which
  copy up to 512 MiB, make such failures more probable?
- No Linux run occurred. Only clippy ran for the aarch64 Linux target. Read
  the Linux arm of `flush_device`. Read the Linux behaviour of each fixture
  too, such as the signal numbers and the sparse files.

## Done when

- Each doubt has a finding, or a reasoned statement that the code holds.
- The findings cite file and line at `52d856978b24`.

## Decisions (running log)

- Review boundary: Ravel commit `52d856978b245006aee7ba4050aa0805b4ef3428`,
  parent `cba154857d41`, against the discovery contract at `822c0ed2` and
  this commit's clarifications. Read the committed source, fixtures, contract
  changes and k57's recorded verification. No test, build, lint, format or
  reproduction command ran, and no Ravel file changed.
- Evidence tier: Verify, with direct-source fallback. The nearest indexed
  Ravel checkout is the default workspace, generation
  `2026-11-01T07:59:38Z`, at the older `b93bb927` tree. Symbol discovery and
  coverage checks show the new skill modules and bounds fixtures missing
  there. An isolated scratch copy made with `jj file show` at the review
  commit supplies exact source and line numbers. The graph supplies no
  completeness claim about this implementation.
- The three conversions of `Stopped` preserve the refusal. The selected
  signal stays equal to the first latched signal on the actual resolution
  path, and cleanup diagnostics precede the special return in `run::settle`.
  No finding is warranted on those paths.
- One source-derived finding warrants integration: the SIGSTOP helper has
  no confirmed stop or unwind-safe child cleanup. Its observation can race
  publication and leave a stopped resolver after an assertion failure. This
  is a static schedule, not an executed reproduction.
- A second, lower-priority finding concerns the book's promise that
  cancellation stores no descriptor. The final watch and cancellation join
  can refuse after a complete descriptor has been published. The owning
  specification permits retained complete objects after refusal; the book
  must not imply rollback.
- Inserted `ravel-skill-bounds-k68` as `integrate-review-impl` immediately
  after this review, before the confinement review. It consumes this review's
  handle and owns triage, fixes and verification. The local-delivery parent
  still has live work, so this retirement closes no parent node.

## Findings

All locations refer to Ravel commit `52d856978b24`. These are source-derived
counterexamples, not executed reproductions. P2 means medium priority and P3
means low priority.

### 1. P2 — The capture-stop fixture can fail and leave a stopped resolver

Location: `crates/ravel/tests/skills_bounds.rs:165`, especially the assertions
at lines 167–171 and the unguarded `Child` returned at line 172.

The helper recognises any `capture-` directory, including the period after
capture has written its manifest and while publication seals its contents.
A valid schedule lets the child write `manifest` before the parent sends
SIGSTOP. The signal then stops the child, the manifest assertion fails, and
unwinding drops a plain `Child`. That drop neither resumes nor kills nor reaps
the process. The fixture's destructor only runs chmod on the sandbox. A
stopped resolver can therefore survive the failed test.

Even if the directory was first observed before the manifest existed, the
parent can be descheduled between discovery and kill. Moreover, successful
`kill(SIGSTOP)` does not acknowledge that the child has entered its stopped
state. The following path checks can race its remaining execution and rename.
On a sufficiently fast or differently scheduled run, the helper can also miss
the whole capture and panic at line 175. The 2,000-file workload and 200 µs
polling interval make an observation likely; they do not establish it.

This helper underlies the timeout and all three cancellation fixtures, at
both public seams. It needs confirmed phase/stop evidence and explicit child
cleanup on every failure path before it can supply reliable acceptance
evidence. Its returned children also use unbounded `wait_with_output` calls
(for example line 487), so the 60-second acquisition loop is not a bound on
the fixture's complete lifetime. The same issues apply on Linux.

### 2. P3 — The book promises descriptor rollback that cancellation does not provide

Location: `docs/book/src/06-reference.md:834`.

The new cancellation paragraph says that Ravel stores no descriptor. However,
`skills/mod.rs:272` publishes the complete descriptor, and the final watch
check at `skills/mod.rs:119` still runs afterward. A signal consumed between
those operations produces `skill_resolution_cancelled` while retaining that
descriptor. A signal found only by `skills/command.rs:28` has the same effect
through `late` at line 98.

This is consistent with the owning specification's retention-after-refusal
contract and still gives no output and no partial descriptor. It contradicts
the book's stronger statement, which could make an owner mistake a valid
retained object for failed cleanup. Correct the statement to distinguish
private staging and partial descriptors from complete retained objects;
rollback is not required by this finding. The capture-only fixtures exercise
an earlier phase and cannot establish absence after late cancellation.

## Other specific doubts

- **Flushing and power loss:** Every copied file reaches `flush_file` at
  `skills/snapshot.rs:530`; manifest and descriptor writes use it at line
  1040. `publish` seals the object and checks `flush_device` before rename
  (lines 942–960), including the descriptor. On macOS the sequence is thus
  per-file fsync followed by F_FULLFSYNC on the staging directory. Apple's
  [fsync manual](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fsync.2.html)
  and [fcntl manual](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html)
  support the distinction between sending a file's writes to the drive and
  flushing the drive cache. They do not make that call a recursive flush of
  all directory metadata. Apple's [APFS reference](https://developer.apple.com/support/downloads/Apple-File-System-Reference.pdf),
  page 33, also permits a Fusion container spanning two devices; a shared
  filesystem identity alone does not prove a single physical cache. I found
  no documented basis here to certify all-device, whole-tree power-loss
  durability, or to assert a concrete APFS regression. On Linux, file fsync
  retains the parent's per-file operation and the new barrier additionally
  syncs the staging directory. The [Linux fsync manual](https://man7.org/linux/man-pages/man2/fsync.2.html)
  requires explicit directory syncing for directory-entry durability; syncing
  the staging root does not recursively sync `tree` and its descendants.
  Neither version syncs those directories or the publication parents after
  rename. The older Linux code therefore did not provide a stronger complete
  tree guarantee. This is an inherited durability qualification, not a newly
  demonstrated regression. Atomic visibility to live readers must not be
  presented as proof of persistence after power loss.
- **Refusals inside I/O errors:** The only `tick` producers are `read_tree`
  (`skills/snapshot.rs:776`) and `seal_contents` (line 861). Recursive calls
  propagate the same error with `?`. The capture readback extracts it at line
  217, publication sealing extracts it at line 943, and the existing-winner
  path extracts it at lines 975–979 after `Expected::holds` calls readback
  at line 913. There is no additional outer caller in this commit that
  converts a `Stopped` into cache-unavailable, resource-invalid or corruption.
  Plain I/O errors retain their existing classification.
- **Selected signal and cleanup precedence:** `cancellation.rs:79` latches
  only the first handled signal, and `received` at line 49 loads rather than
  consumes it. The signal read by the watch therefore remains the front
  signal that `finalise` places first at lines 290–333. Resolution precedes
  runner startup, so there is no earlier runner cancellation to displace on
  this path. `Watch::overrule` (`skills/watch.rs:100`) retains an original
  refusal when it changes it into cancellation. `run::settle` reports all
  supplied cleanup errors and any surviving group before its skills return
  (`run.rs:434–445`), so that exception does not discard those diagnostics or
  admit success. `skills/command.rs:85` similarly appends all finalisation
  reports. The new `settle` test covers a cleanup error; it does not separately
  cover a surviving group, but that output precedes the same branch. No
  reachable signal disagreement or lost supplied cleanup report was found.
- **Handler acquisition and restoration:** `skills/command.rs:24–29` routes
  a failed `begin` through `finalise` as well as successful resolution.
  `Handlers::begin` records each original disposition before attempting its
  replacement (`cancellation.rs:156`). Finalisation attempts their restoration
  while blocked and restores the entry mask when its observations establish
  that release is safe (lines 334–379). A failed block, failed disposition
  restore or unknown pending set can deliberately leave handlers or masks in
  place until this executable exits; those paths produce reports, not success.
  The completion specification explicitly requires that guarded failure
  behaviour. `signal_state::recorded` is appropriate before installation:
  it refuses when executable-entry state was not captured, before this command
  changes dispositions. No new silent-success path was found.
- **Counts, byte values and settings:** `skills/mod.rs:138–160` checks all
  three configured counts before Git refusal or cache acquisition. The
  directory count sums the configured roots as the clarified contract states.
  `bounds.rs:146–171` reports the subject, actual value, fixed bound and unit;
  streamed entry/source/total counts use `AtLeast`, while known file sizes
  and configured counts use `Exact`. `Walk::admit` at
  `skills/snapshot.rs:554` checks resource, source and total bytes before
  copying, and growing input refuses at line 521. The entry check at line 321
  admits equality and refuses the next entry. Descriptor and preamble checks
  at `skills/mod.rs:250–269` precede descriptor publication. The deadline
  parser at `skills/settings.rs:93–106` accepts the documented inclusive
  range, while the watch retains whether its value was explicit or default.
  I found no discrepancy in those diagnostic values or boundary comparisons.
- **Large mandatory files:** The mandatory-reference check follows a captured
  internal link and applies 65,536 bytes at `skills/mod.rs:474–485` before
  reading its UTF-8 contents. A file larger than 16 MiB reaches the earlier
  `resourceBytes` refusal instead. It still refuses as `skill_limit_exceeded`,
  status 3, with the violated size, bound and path. The contract does not
  specify precedence between simultaneous byte violations; the book describes
  the resource bound as applying to every regular file. Thus this does not
  bypass the mandatory bound. An earlier mandatory-size check could save
  copying work, but is not required for this contract.
- **Fixture load and Linux:** The sparse inputs created with `set_len` at
  `skills_bounds.rs:214` are read and written in full by capture, so they do
  exercise the logical byte bounds on both platforms. Their physical input
  allocation need not equal those bytes. Signal numbers use libc constants;
  fixed 143 is correct for TERM on both supported platforms. Linux uses the
  fsync arm at `skills/dir.rs:181`, with checked errors. Finding 1 concerns
  scheduling and process lifetime on both platforms. The multi-source fixture
  at `skills_bounds.rs:378` copies and hashes hundreds of MiB repeatedly, so
  it adds I/O and CPU contention. The recorded unrelated timing failure and
  successful rerun do not establish causation or a new production defect.
  There was no Linux execution, and cross-target clippy cannot establish
  these runtime properties.

## Verification limits

The producer records 701 passing tests on its second macOS full check, other
affected tasks, 16 failing mutation controls, and Linux cross-target clippy.
Those are historical results, not checks rerun here. Its controls do not
independently discriminate the per-block, readback and sealing watch sites or
the growing-file refusal; static inspection supports those paths, not new
execution evidence. This review supplies no power-loss experiment, multi-device
APFS qualification, or Linux run.

Reconciled both findings with current feature parent `0cc7f8c4f336`: intervening
lifetime, confinement and resolver/publication integrations do not change the
capture-stop helper or the cancellation paragraph, and do not move the final
watch ahead of descriptor publication. The integrations already own the prior
reviews' findings; this review does not repeat them. Integration must triage
these two findings against that current source and own all fixes and checks.

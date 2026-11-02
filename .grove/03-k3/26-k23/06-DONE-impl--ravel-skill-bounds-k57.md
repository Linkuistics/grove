# ravel-skill-bounds-k57

## Goal

Enforce every skill bound, the resolution deadline and cancellation during
resolution, at both public seams. Each failure refuses with its documented code
and status, and no failure leaves staging or a partial descriptor.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Bounds", "Failures"
  and the `resolutionTimeoutMs` rules under "Settings".
- Ravel `docs/specs/result-job-completion.md`: outcome and diagnostic order
  for a consumed signal.

## Done when

- Each count and byte bound of the table refuses at its limit and passes below
  it, as `skill_limit_exceeded`. The refusal reports the actual value, the
  bound and the subject. Controls that remove each bound show that the check
  runs. The bounds are sources, search directories, mandatory files, one
  mandatory file and one resource. They are also source bytes, total bytes,
  entries in one source, the descriptor and the preamble.
- `resolutionTimeoutMs` accepts 1,000 to 3,600,000 and defaults to 120,000. A
  malformed value refuses as `settings_invalid`. A controlled delay times out as
  `skill_resolution_timeout`, exit 124. The refusal reports the effective
  deadline and the active source and operation. A larger valid deadline
  succeeds with every check.
- INT, TERM or HUP during resolution refuses as `skill_resolution_cancelled`.
  Ravel removes its staging, reaps its children and raises the same signal
  again. A consumed signal wins over a resolution refusal, and the report keeps
  that refusal as the original failure.
- Public fixtures cover the local parts of "Bounds and frames", "Slow valid
  source" and "Cancellation". For a local source, the controlled delay and TERM
  act during capture. The transport case belongs to
  `ravel-pinned-delivery-k25`.
- No bound truncates caller text, omits a directory or mandatory file, or
  disables delivery.
- The book reference and `CHANGELOG.md` state the bounds and codes. The
  affected Ravel checks pass. The bookmark advances by jj, and nobody pushes it.

## Notes

The prepared-prompt bound and the frame bound already exist from
`ravel-skill-launch-k55` and `ravel-schema-four-k53`. This leaf adds no second
check of them.

## Decisions (running log)

- Entry base. `jj git fetch` changed nothing, and Ravel `main` is still
  `24a545236185`. The line holds k56's `cba154857d41`, so no rebase was
  necessary at entry.
- A bound is a maximum. A value equal to the bound passes, and one more
  refuses. k55's prepared-prompt check already works in this way. The phrase
  "refuses at its limit" in this leaf names the boundary, not the value that
  refuses.
- Device flushes. A release build took 10.0 seconds for a source of 2,000
  files of 13 bytes. On macOS, `sync_all` is `F_FULLFSYNC`, at about 4.7 ms
  for each file. A plain `fsync` took about 0.065 ms for each file. After
  2,000 files, one `F_FULLFSYNC` took 9 ms. With a full flush of each file,
  a source at the entry bound would take about 95 seconds. That is near
  the default deadline of 120 seconds. Decision: on macOS, capture calls a
  plain `fsync` for each file and one `F_FULLFSYNC` on the staging directory
  before each publication rename. The order of the writes and the rename is
  the same. Linux keeps `fsync` for each file, which is already a full flush.
- Where the bounds act. `skills::bounds` holds the table as `Limits`. The
  three configured counts refuse before the Git refusal and before the cache
  opens. The search-directory count is the sum of the configured roots, not
  the joined descriptor paths, so that it refuses before acquisition. Capture
  refuses a file by its first size, before it copies a byte, against the
  resource, source and total bounds. It counts each entry before it handles
  it. A refusal of a streamed count reports "at least" the value at the stop.
  The total counts each configured source, also two that name one
  directory. The mandatory-file bound has one site, the check of the
  reference, because a link inside the source can reach the file. The
  descriptor and preamble bounds refuse before the descriptor is published:
  the preamble uses the path that publication will give.
- Bound names in `error.bound.name`: `skillSources`,
  `skillSearchDirectories`, `skillMandatoryFiles`, `mandatoryFileBytes`,
  `resourceBytes`, `sourceBytes`, `totalSourceBytes`, `sourceEntries`,
  `descriptorBytes` and `preambleBytes`. `sources` was not usable, because
  the selection bounds already use it.
- The watch. `skills::watch::Watch` holds the start, the deadline as a
  `Bound` (origin `settings.json` or the default) and the active operation
  and source. `check` reads the cancellation latch first, then the clock. It
  runs at each entry and at each 64 KiB block of a copy. It also runs at
  each entry of the written-tree check, the winner check and the seal, and
  before each rename.
  Code that returns `io::Error` carries the refusal in a private error type,
  and `watch::stopped` takes it out at the three conversion sites. The watch
  cannot interrupt one system call that storage holds. The specification
  and the book now state that limit.
- Cancellation at both seams. `start-session` already installs the handlers
  before preparation. `skills resolve` now installs them too, and finalises
  them before it writes its output. A signal that the final look finds
  cancels a successful resolution as well. `run::settle` keeps a refusal of
  stage `skills` that carries a signal as the primary outcome, in place of
  `session_cancelled`, as the evidence row "Resolution has its own code"
  requires. `Watch::overrule` makes a consumed signal win over any refusal
  of the resolution and of the prepared prompt, and keeps that refusal in
  the report. A copy that grows past its first size now refuses as changed
  at once, so growth cannot pass a byte bound.
- Unit tests read a signal only from the thread-local fault seam, because the
  latch of the process is shared by every test thread.
- Test speed. Unoptimised SHA-256 took 9 seconds for 128 MiB, so the dev
  profile now optimises `sha2`, as it already optimises `rhai`. 128 MiB then
  took about 1 second.
- Controlled delay. The public fixtures stop the resolver with SIGSTOP when
  its `capture-` staging exists and holds no manifest, and while no snapshot
  exists. A sleep past the deadline and SIGCONT give the timeout. A signal
  sent during the stop arrives at SIGCONT, during capture. No test-only hook
  enters the product binary.
- Not reachable on macOS: the descriptor and preamble bounds. With 256
  directories and 256 mandatory files of at most about 1,000 bytes of path,
  both stay near 0.5 MiB. Unit tests set each limit to the measured size,
  which passes, and to one byte less, which refuses. A Linux path of 4,096
  bytes can reach both bounds.
- Firing controls, each run alone and restored from a copy. The digest of
  the Rust sources was the same before and after. Each of these 16
  mutations made its fixtures fail. Ten removed one bound check each. Three
  were the counts of sources, search directories and mandatory files. Seven
  were the mandatory-file size, the resource size, the source bytes, the
  total bytes, the entries, the descriptor and the preamble. The other six
  removed the deadline of the watch and its cancellation look. They also
  removed the overrule, the special case in `settle`, the handlers of
  `skills resolve` and the origin of the deadline setting. Three looks
  have no control of their own. They are at each 64 KiB block of a copy, in
  the written-tree check and in the seal. The early refusal of a growing file
  has none either. In every fixture, the look at each entry catches the same
  stop first.
- **Output (2026-11-02).** The Ravel commit is `52d856978b24` (change
  `qwuwzzxo`) on bookmark `skill-directory-delivery`, with parent
  `cba154857d41`. The bookmark is not pushed. On macOS, one script on a tree
  whose digest was the same before and after ran `test:cli`,
  `docs:book:build`, `docs:examples:test`, `docs:excerpts:test`,
  `docs:excerpts`, `docs:examples`, `docs:figures:test`, `docs:ste:test`,
  `docs:breaks:test`, `book:integrity` and `smoke`, and all passed. Its
  `check` failed once: `worker_reap_admission_primary_reports_eof_both` saw
  `selection_timeout` in place of `worker_failed`, and nextest stopped at
  663 of 701 tests. That test does not touch skills. It passed six times
  alone. A second `task check` on the same frozen tree passed all 701 tests,
  with 48 skipped. The new byte-bound fixtures add load to the suite, which
  can make that timing failure more probable. The strict STE check passed on
  chapters 5, 6 and 8. Clippy for the aarch64 Linux target reported only an
  unused import in `tests/rhai_request.rs`, which this leaf did not change.
  No Linux run occurred.
- **A review is earned.** The compiler cannot check the flush order or the
  precedence of a signal over a refusal. It also cannot check the way a
  refusal rides inside an I/O error, or the timing of the SIGSTOP fixtures.
  `ravel-skill-bounds-k64` (`review-impl`) is inserted before
  `ravel-local-handoff-k61`.

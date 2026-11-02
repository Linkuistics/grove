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

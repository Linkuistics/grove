# ravel-skills-resolve-k54

## Goal

Deliver `ravel skills resolve` for local sources. It reads the `skills`
settings object, captures each local source, publishes immutable snapshots and
the directory descriptor, and prints the preamble. An independent harness can
use that output, so this reader is useful before launch preparation exists.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Settings", "Sources and
  retained resources" without the Git subsection, "Directory descriptor and
  loading contract", "The preamble", "`ravel skills resolve`" and "Failures".
- Ravel `docs/adr/skill-resources-outlive-sessions.md` for retention.
- The schema-4 line of `ravel-schema-four-k53`.

## Done when

- The settings parser accepts the `skills` key with exactly `schemaVersion` 1,
  `sources`, `mandatory` and `resolutionTimeoutMs`. It checks each field of
  the source and mandatory records. A malformed value refuses as
  `settings_invalid`, stage `cli`, exit 2, with a `location` such as
  `skills.sources[2].roots[0]`. A repeated member name in any object of the
  settings file refuses in the same way, for every key.
- Delivery is disabled without a settings file, without `skills`, with both
  arrays empty and without an absolute `HOME`. Disabled resolution creates no
  cache and prints nothing, with status 0. `--json` prints the `disabled` object.
- A valid Git source refuses before any acquisition, with a remedy that names
  the unfinished pinned stage. The refusal is not `settings_invalid` and has no
  stable published code. A fixture proves it. `ravel-pinned-delivery-k25`
  removes both.
- Local capture copies the whole source directory into private staging. It
  excludes `.git` and `.jj` at every level. It admits only relative symlinks
  whose whole chain stays inside the canonical source root. It refuses other
  links, nonregular objects and paths that are not exact UTF-8 or that hold a
  control character, as `skill_resource_invalid`.
- A source boundary that overlaps `$HOME/.config/ravel` or the settings file
  refuses as `skill_access_unavailable`. A canonical alias does not avoid the
  check.
- The snapshot identity is the SHA-256 of the versioned manifest. Publication
  is atomic and does not replace an existing object. A snapshot keeps the
  executable bit and loses write permission and set-ID bits. A new resolution
  captures current bytes and reuses a published snapshot only by content digest.
- The descriptor holds `schemaVersion` 1, `sources`, `directories` and
  `mandatory`. Its directory has the name of its canonical digest. Repeated
  directory paths appear once. Mandatory references become absolute snapshot
  paths in configured order.
- Missing sources and roots refuse as `skill_source_missing`. Missing,
  nonregular or non-UTF-8 mandatory files refuse as `skill_mandatory_invalid`.
  An unknown mandatory source ID refuses as `settings_invalid` before capture.
- The plain output is the preamble with a final newline. The JSON form carries
  `descriptorPath`, `sources`, `directories`, `mandatory` and `instructions`. A
  failure prints a failure document with stage `skills` and the documented exit
  status. It prints no preamble and no partial JSON. The help text shows each
  form and the cache side effect.
- Public fixtures at the `skills resolve` seam cover the local parts of seven
  acceptance rows. They are "Disabled delivery", "Directory delivery",
  "Filesystem versus skill semantics" and "Settings and bootstrap". They are
  also "Relative resources", "Standalone acquisition" and "Local freshness".
  For freshness, a content change gives a new snapshot, and old readers keep
  old bytes.
- A committed LFS pointer file in a local source is captured as literal bytes.
- The book reference, `CONTEXT.md` and `CHANGELOG.md` document the command and
  the settings object. The affected Ravel checks pass. The bookmark advances by
  jj, and nobody pushes it.

## Notes

Ravel reads no `SKILL.md`, frontmatter or name. Duplicate skill names and
invalid optional metadata must resolve. Do not turn them into admission rules.

Atomic publication and private staging land here. Detection of concurrent
edits, filesystem path collisions, cache tampering and unsafe cache roots is
`ravel-skill-publication-k56`. Bounds, the deadline and cancellation are
`ravel-skill-bounds-k57`. Keep internal seams for their fault injection.

## Decisions (running log)

- Entry base. `jj git fetch` changed nothing, and Ravel `main` is still
  `24a545236185`. The line holds k53's `eac3c987` above `822c0ed2`, so no
  rebase was necessary at entry.
- A new `strict_json` reader replaces `serde_json::from_slice` for the whole
  settings file. It builds the same `Value` through a serde seed and refuses a
  repeated member name in any object, with a location such as
  `skills.sources[2].id`. `serde_json::Value` alone keeps the last duplicate,
  so it cannot see the repeat.
- Cache layout under `$HOME/.cache/ravel/skills/v1`: `staging/`,
  `snapshots/<digest>/{manifest,tree/}` and
  `descriptors/<digest>/directories.json`. `snapshotRoot` is the `tree`
  directory. The cache root is canonicalised, so every path is a real path.
- Publication renames a whole staged object with a no-replace rename:
  `renamex_np(RENAME_EXCL)` on macOS and `renameat2(RENAME_NOREPLACE)` on
  GNU/Linux. The contents are sealed before the rename and the object root
  after it, because a directory rename needs write permission on the moved
  directory. A loser compares the winner's manifest or descriptor bytes, and
  reuses it or refuses as `skill_cache_corrupt`. A deep check of a reused
  object's tree belongs to `ravel-skill-publication-k56`.
- Manifest format: the header `ravel-skill-manifest\n1\n`, then one line for
  each entry sorted by path bytes: type (`d`, `f`, `l`), executable bit,
  `<len>:<path>` and `<len>:<digest or target>`. The digest of these bytes is
  the snapshot identity. The descriptor is `serde_json` canonical bytes, with
  sorted keys because `preserve_order` is off.
- The protected set holds `$HOME/.config/ravel`, the settings file and the
  cache root, each also in canonical form. A source that contains the cache
  would capture its own staging, so that overlap also refuses as
  `skill_access_unavailable`.
- Temporary refusals without stable public codes: `skill_git_unsupported` for
  a valid Git source (removed by `ravel-pinned-delivery-k25`), and
  `skill_launch_unsupported` when `start-session` meets enabled delivery
  (removed by `ravel-skill-launch-k55`). The second refusal keeps a launch
  from silently dropping configured guidance between the two stages.
- `subdir` is required in a Git record. The pinned stage can relax it without
  breaking settings. An HTTPS URL with any user part refuses as a credential.
- `resolutionTimeoutMs` is parsed and range-checked here. Its enforcement is
  `ravel-skill-bounds-k57`.
- Defect found by hand and fixed: staged files were created 0600, so sealing
  lost the executable bit. The staged mode now carries the bit.
- Public fixtures are in `crates/ravel/tests/skills_resolve.rs`, 15 tests at
  the `skills resolve` seam, plus unit tests for the settings grammar, the
  strict reader, the manifest, the preamble and no-replace publication. They
  cover the local parts of the seven acceptance rows of the leaf, the LFS
  pointer, the Git refusal, protected overlaps, and the temporary
  `start-session` refusal with a disabled-delivery control that launches.
- Firing controls, each run alone and restored from a copy: removing the
  `.git`/`.jj` exclusion, the duplicate-member reader, the executable-bit
  carry and the protected-overlap check each failed the suite. Removing the
  `..` boundary check at first passed, because both escaping cases also
  dangled. A new case links to an existing file outside the source whose
  relative path also exists inside, and that mutation then failed the suite.
- The preamble wording is mine; the specification states its content but not
  its text. It lists paths as JSON literals, then eleven numbered obligations
  that cover the five specification obligations and each row of the agent
  discovery table. The book and the help call it the discovery instructions,
  because the book's STE rules exclude `-ing` words such as "loading" and
  reserve "publish" for the result.
- Gap: APFS refuses a file name that is not UTF-8 (errno 92), so no macOS
  fixture can create one. The refusal path for such names is in
  `snapshot.rs` and has no executed fixture here. A Linux run could add one.
- Review earned. The resolver is a subsystem that the next six leaves build
  on, and its filesystem-safety rules cannot be checked by the compiler. One
  control first passed a broken boundary check. `ravel-skills-resolve-k62`
  (`review-impl`) is inserted before `ravel-local-handoff-k61`, as the local
  brief directs. No provider evidence was in this session's environment, so
  it has no Creator line. k55 received a handoff note for the temporary
  launch refusal.
- Output. Ravel commit `777e609f2fe31b9a2ceb8e11645d2f491c7a1d85` on the
  bookmark `skill-directory-delivery`, with parent `eac3c987`. Nobody pushed
  it. The build reports `ravel 1.1.0`.
- Verification on macOS, one script on a frozen tree whose digest was the same
  before and after the run: `task check` passed with 648 tests passed and 48
  skipped. `task test:cli`, `task docs:book:build`, `task docs:examples:test`,
  `task docs:excerpts:test`, `task docs:ste:test`, `task docs:breaks:test`,
  `task docs:figures:test`, `task book:integrity` and `task smoke` passed.
  The strict STE check passed on the reference chapter and `CONTEXT.md`.
- Not run: Linux. The resolution deadline, bounds, cancellation, concurrent
  edits and cache tampering are later leaves of this node.

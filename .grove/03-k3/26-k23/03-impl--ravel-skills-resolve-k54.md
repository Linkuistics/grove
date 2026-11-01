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

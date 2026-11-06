# ravel-default-directory-k225

## Goal

Implement Ravel's default skill directory in Ravel's code, on the
`skill-directory-delivery` line, before the 2.0.0 cut.

## Context

- Work in the Ravel workspace
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`. Read
  its own instructions first.
- The contract is the section "Default skill directory" of Ravel's
  `docs/specs/skill-discovery.md`, at Ravel commit `78d833d24364`. The
  decision record is "Installed skill sources are the default".
- The running log of `ravel-default-skill-directory-k218` holds the reasons
  and the rejected options.

## Done when

- `skills.sources` and `skills.mandatory` are optional. Without `sources`,
  the default directory supplies the sources. `"sources": []` disables the
  default. A mandatory reference names a source of the effective set.
- Ravel reads `share/ravel/installation.json` two levels above its canonical
  executable, only when the settings name no `sources`. An absent record
  gives no default directory. A malformed record refuses as
  `skill_directory_invalid`.
- Ravel lists the directory at each resolution, as the entry table states.
  Each refusing row refuses as `skill_directory_invalid`, stage `skills`,
  exit 3, and names the entry. A default source has the roots `skills` and
  `craft`, where each exists, in that order. Default sources follow the byte
  order of their IDs.
- The routing notice's `skills` object holds `defaultDirectory`.
- The specification's test-seam row "Default skill directory" passes, with a
  copied executable and a fixture directory of Homebrew-shaped links. A
  control for each refusing row has been seen to fail.
- Ravel's own checks pass. The commit is on the `skill-directory-delivery`
  bookmark.

## Notes

The formula channels that write the record, their installed tests and the
book chapters belong to `ravel-installation-record-k226`. This leaf's tests
write the record themselves.

The contract is a public 2.0 surface. If an executable seam cannot establish
a property, judge at the boundary whether a `review-impl` leaf is earned.

## Decisions (running log)

**Code placement (2026-11-06).** A new module `skills/installed.rs` reads the
installation record and lists the default skill directory. The resolver
builds one effective source set from the settings or from that listing. Each
effective source knows its place: a settings index or an entry path. A
refusal about a default source then names the entry path in place of a
settings location, as the specification requires.

**Gaps in the specification (2026-11-06).** The specification leaves four
details open. These answers keep each refusal explicit:

- Ravel cannot find its own executable path. That refuses as
  `skill_directory_invalid`, because a silent absent record would hide the
  fault.
- A dangling link at the record path refuses as an unreadable record. Only
  `NotFound` on the path itself is an absent record.
- The default skill directory exists but cannot be listed, or it is not a
  directory. That refuses as `skill_directory_invalid`. Only `NotFound` is an
  absent directory.
- A mandatory `source` must have the source-ID grammar, also without
  `sources`. A malformed ID then refuses as `settings_invalid`, and only a
  well-formed ID that the directory lacks refuses as `skill_source_missing`.

Entries are checked in byte order of their names, so the first refusing entry
is deterministic.

**Removal after the listing (2026-11-06).** The public seam cannot place a
removal between the listing and the capture. A unit test does it with a new
fault hook that runs after the listing. A second hook replaces the
installation prefix in unit tests.

**Controls (2026-11-06).** Seventeen mutants each made their test fail.
Eleven mutants each accepted one refusing row of the entry table, and each
row ran alone. Others read a malformed record as absent, accepted an entry
with neither root, reversed the two orders, listed dot entries and removed
`defaultDirectory`. `task check` passed with 708 tests, and
`task docs:examples` and `task smoke` passed.

**No review leaf (2026-11-06).** Each property of the contract has an
executable seam in this leaf, except the real Homebrew layout and Linux. The
specification's row "Installation record" puts both in the installed tests of
`ravel-installation-record-k226`. This leaf therefore earns no `review-impl`
leaf, and it spent no in-session reviewer.

**Book (2026-11-06).** Chapter 6 of Ravel's book still states that `sources`
and `mandatory` are required, and it lists the notice without
`defaultDirectory`. k226 owns the book chapters, so this leaf leaves them.
Ravel's design evidence records the gap.

**Handoff (2026-11-06).** The Ravel commit is `da736c1fc255`, with the
parent `78d833d24364`, on the `skill-directory-delivery` bookmark. Nobody
pushed it. `ravel-installation-record-k226` and
`grove-default-directory-checks-k228` build on that commit.

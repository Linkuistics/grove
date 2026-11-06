# ravel-installation-record-k226

## Goal

Make each Homebrew formula channel of Ravel write the installation record.
Bring Ravel's book and installed checks into agreement with the default skill
directory.

## Context

- Work in the Ravel workspace
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`, after
  `ravel-default-directory-k225`.
- The section "The installation record" of Ravel's
  `docs/specs/skill-discovery.md` states what each formula writes.
- Ravel's formula template is `scripts/templates/ravel.rb.tmpl`, and
  `scripts/release-formula.sh` and `scripts/release-formula.test.rb` use it.
  The coordinated release design in Grove,
  `docs/specs/coordinated-release.md`, names the channels: the unversioned
  `ravel` and the keg-only `ravel@2`.
- The planning inputs of `ravel-default-skill-directory-k218` name this work.

## Done when

- Each formula channel writes `share/ravel/installation.json` at
  installation, with `skillDirectory` set to
  `HOMEBREW_PREFIX/share/ravel/skills`. The keg-only `ravel@2` records the
  same directory as the unversioned `ravel`.
- The release archives and `scripts/install.sh` write no record, as the
  specification states.
- The formula tests and the installed checks read the record. With no
  settings and a fixture source in the recorded directory, an installed
  `ravel skills resolve --json` reports that source and `defaultDirectory`.
- Ravel's book chapters on settings and skill delivery agree with the
  specification: optional `sources` and `mandatory`, the default directory
  and `skill_directory_invalid`.
- Ravel's own checks pass. The commit is on the `skill-directory-delivery`
  bookmark.

## Notes

Publication of 2.0.0 and its tap states belong to discovery-rollout-k6. This
leaf changes no tap.

## Decisions (running log)

**One template carries both channels (2026-11-06).** Ravel renders only
`scripts/templates/ravel.rb.tmpl`. The `CHANNEL=versioned-2` tooling of
`docs/specs/release-channels.md` does not exist yet, and the rollout planning
leaf `discovery-rollout-k12` owns it. The template therefore writes the
record, and the channel tooling inherits that. The record takes its
directory from `HOMEBREW_PREFIX`, not from the keg, so both classes write the
same bytes.

The formula harness derives a `RavelAT2` keg-only class from the
rendered text and proves that. The derivation is a fixture, and k12 replaces
it with the real rendering.

**`defaultDirectory` comes from the routing notice (2026-11-06).** The Done
when asks `ravel skills resolve --json` to report `defaultDirectory`. The
specification and design k218 put that member only in the routing notice of
`start-session --json`. This leaf changes no contract. The installed test
reads the source from `skills resolve --json` and `defaultDirectory` from
the routing notice.

**The fixture directory is private (2026-11-06).** `brew test` runs in
Homebrew's sandbox, which denies writes to `HOMEBREW_PREFIX`. A fixture in
the real recorded directory would also reach every concurrent session on the
owner's machine. The formula test therefore checks the record that
`install` wrote, then copies `bin/ravel` into a private prefix. The same
formula method writes that prefix's record, which names a private directory
with a fixture source. The installed checks after a real `brew install`
read the real record and run `brew test`.

**Harness findings (2026-11-06).** Homebrew's `Pathname#install` moves its
files out of the source, so each class installs from its own copy of the
extracted archive. A launch protects the directory of its configuration
script, so the formula test copies `config.rhai` into its own directory away
from the fixture source. The skill snapshots are read-only, so the test
restores write permission in an `ensure` block. Homebrew can then remove
the test path, and the release script's trap does the same for its scratch.

**Controls (2026-11-06).** Three template mutants each failed
`task release:formula` for the intended reason. A record that names the keg
failed the record assertion. No record failed the exact file list. A
symbolic link in place of the copied executable resolved as `disabled`.

A parse-based `installed-record.sh` accepted a repeated member, and its test
caught it. An extracted archive with a planted record failed the installed
smoke test, and without it passed.

**Linux (2026-11-06).** The same harness ran under Linux Homebrew in the
`homebrew/brew` image, arm64 digest `b0072bfdebf5`, against the aarch64 GNU
archive. Both classes passed, and the record named
`/home/linuxbrew/.linuxbrew/share/ravel/skills`. The image has no `xz`, so
the host extracted the archive. The x86_64 archive ran no formula test.

**No review leaf (2026-11-06).** Each property has an executable seam with
a control that failed when broken. The record bytes, both classes, macOS and
aarch64 Linux, the archives and the book checks are covered. This leaf earns
no `review-impl` leaf, and it spent no in-session reviewer. Chapter 11 still
holds earlier formula-check rows about the SDK and probe workers. The
rollout's book work owns them, and this leaf left them.

**Handoff (2026-11-06).** The Ravel commit is `ff14cba8`, with the parent
`da736c1fc255`, on the `skill-directory-delivery` bookmark. Nobody pushed
it. `task check` passed 708 of 708 run tests. `task release:formula`,
`task smoke` and the book checks passed.

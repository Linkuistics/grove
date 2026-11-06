# grove-keg-sources-k229

## Goal

Make Grove's release archive and formula carry core and the five domain packs
at `share/ravel/skills/<source-id>`, from Grove's record of pack commits.

## Context

- "The keg layout" and "The pack record and the release" of
  `docs/specs/codex-skill-provisioning.md`. The row "Installed distribution"
  of its "Test seams".
- `scripts/release-build.sh`, `scripts/release-common.sh` (the archive
  manifest), `scripts/templates/grove.rb.tmpl`, `scripts/release-smoke.sh`,
  `scripts/release-formula.test.sh` and `scripts/release.test.sh`.
- The record and its checkouts come from `grove-pack-record-k227`. The 2.0
  build comes from `grove-default-directory-checks-k228`.

## Done when

- The archive build copies each pack's source directory from the plain
  checkout of its `sha` that `check:packs` uses. It copies `plugins/grove` of
  the tagged tree to `share/ravel/skills/grove-core/`. Each copy keeps
  executable bits and links.
- The archive manifest lists each copied file with its Git mode, and each
  archive passes the manifest check. A control with a changed mode, and one
  with a missing pack checkout, has been seen to fail.
- The formula installs `share/ravel/skills/` beside the executables and
  `libexec/`.
- `task release:smoke` and `task release:formula:test` copy the 2.0 build
  into a private prefix, with a record that names the keg's
  `share/ravel/skills`. With no settings, `ravel skills resolve --json`
  reports exactly the six sources. Each `originPath` lies inside the keg.
- `release:formula:test` compares each source's `resourceDigest` with that of
  the plain checkout of the same commit.
- Fresh install and upgrade pass at each tap state A to D of the coordinated
  release, with an admitted Ravel major at each state.
- `bash scripts/check.sh` passes.

## Notes

The two marketplace files at the archive root belong to
`grove-hook-adapters-k230`.

A cut changes only `CHANGELOG.md`, so the `grove-core` bytes of a release
equal those of its candidate. Keep that true.

## Decisions (running log)

**The manifest (2026-11-06).** `archive_manifest` now prints one
`<mode><TAB><path>` line for each file, with its Git mode. A fixed part lists
the executables and the caller assets. `skills_manifest` adds core and each
pack under `share/ravel/skills/<id>/`. The check compares the layout and each
mode, so `archive_executables` went: the modes carry that fact. A link must
stay a link, and a plain file must not gain an executable bit.

**Where the sources come from (2026-11-06).** Core is the files of
`plugins/grove` that Git tracks or would track, where the file still exists.
The listing is `ls-files --cached --others --exclude-standard`, with Git's
global and system configuration off.

In the clean tagged tree of a release that is the
tagged tree. In a working copy it is what a commit of that copy would hold.
Its modes come from the files. A pack is the files that its clean checkout
tracks under `path`, with modes from Git. `tar` copies both, so modes and
links stay.

**No checkout in the check (2026-11-06).** The build makes or reuses each
checkout with `pack_checkout`, the rule of `check:packs`, which
`pack_checkout_is_clean` now names. The manifest only reads a checkout under
that rule. A missing or changed checkout, or a record of another `sha`,
fails the manifest and so the archive check, naming the pack. The source
paths are shell variables of `release-common.sh` with an `ARCHIVE_` prefix,
never environment variables, so `release.test.sh` points them at fixtures
and no new `GROVE_*` name appears.

**The reference of core's digest (2026-11-06).** `release:formula:test`
compares core with a plain checkout of `HEAD`, made by `pack_checkout` from
this repository. It refuses when `plugins/grove` differs from `HEAD`,
because then no commit holds those bytes. The five packs compare with their
checkouts under `target/external-packs`. A measurement showed that Ravel's
`resourceDigest` covers content only: the same files under another path, ID
or root list give the same digest.

**Tap states (2026-11-06).** `scripts/grove-tap-states.sh` models states A to
D in a private Homebrew-shaped prefix. Kegs lie in `Cellar`, `opt` links name
them, and linked kegs put relative links in `bin` and `share/ravel/skills`.
An upgrade unlinks, installs, links and removes the earlier keg. The new
Grove keg is the one that Homebrew's install method of the rendered formula
made. Each 2.0 keg holds the record that Ravel's formula writes.

At A and B
the check covers the published keg's place, the selected Ravel and the empty
default directory: a 2.0 resolution there reports `disabled`. It does not
run the published 22.8 callers, which this leaf does not change. At C and D
the installed callers run on that state's majors, and each 2.0 Ravel reads
the six sources with their checkout digests.

Real published formulae stay with rollout. Before the cut the working copy
has the published version, so the new keg takes Homebrew's revision label
`22.8.0_1`.

**Six IDs in two places (2026-11-06).** The formula's `SKILL_SOURCES` and the
installed smoke's `expected_sources` name the six sources, because neither
runs where the record is. `release.test.sh` checks that both equal core and
the members of `plugins/packs.json`.

**First measured run (2026-11-06).** `task release:formula:test` on the native
archive of the working copy passed. That covers the Homebrew install and test,
the modes, six digests equal to their checkouts, and both routes at A to D. `release.test.sh` passed.

**Controls (2026-11-06).** Each control failed against its altered subject,
and the archive digest `17200f57…` stayed the same. On the real native
archive, a pack file with an added executable bit failed the manifest check
by name. A moved `research` checkout failed it and named the pack. A changed
byte with the same modes passed the manifest but failed the digest
comparison of `release:formula:test`. A prefix without `writing` failed the
installed smoke and tap state C. A wrong recorded digest failed tap state C.
`release.test.sh` holds the fixture controls: each omission, each extra
entry, three mode changes, a missing checkout, a changed checkout and a
record of another `sha`.

**Final runs (2026-11-06).** `task release:smoke` passed all three targets.
The six keg sources resolved with no settings on macOS, in the arm64 floor
container and under both emulated CPU floors. `mise exec -- bash
scripts/check.sh` passed all 19 principal checks. The script digest before
and after the smoke, and the tree digest before and after the gate, matched.
Three documents outside the release guide named the old archive contents or
the old formula test arguments: `docs/grove-delivery-contraction.md`, the ADR
"harness selection is owned by policy" and the changelog. They now name the
keg sources.

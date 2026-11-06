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

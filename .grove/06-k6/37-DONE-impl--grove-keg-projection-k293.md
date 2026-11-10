# grove-keg-projection-k293

## Goal

Make Grove's formula template install the revised keg layout, and keep every
Grove packaging check green with it. The archive keeps its directories. The
installed keg holds each source in `libexec/skill-sources/<source-id>/` and a
relative directory symlink at `share/ravel/skills/<source-id>`.

## Context

- "The keg layout", "Hook adapters" and "The pack record and the release" of
  `docs/specs/codex-skill-provisioning.md` own the layout. The section
  "Directory-link layout integration" of
  `docs/homebrew-skill-delivery-evidence.md` records why.
- `scripts/templates/grove.rb.tmpl` installs `share/ravel/skills` as real
  directories today. Its caveats and its own `test do` block describe that
  layout.
- `assets/keg-root/` holds the two marketplace files that the release copies
  to the archive root. Both name the writing hooks plugin through
  `./share/ravel/skills/writing/...`.
- These consumers assert or construct the earlier keg layout:
  `scripts/release-formula.test.sh` and `.rb`, `scripts/release.test.sh`,
  `scripts/candidate-keg.sh` and `.rb`, `scripts/grove-tap-states.sh`,
  `scripts/harness-trials/cases/keg-craft-discovery/case.json` and the
  `HOOKS_ROOT` fixture of
  `docs/evaluations/owner-skill-delivery/test_staging_trial.py`. Enumerate
  the whole class with a search, not with this list.
- `scripts/grove-installed-smoke.sh` and `scripts/release-smoke.sh` test the
  archive layout on macOS and on each Linux target. The archive layout does
  not change.

## Done when

- The rendered formula moves each archive source into
  `libexec/skill-sources/<source-id>/` and creates its relative alias. The
  marketplace files stay at the keg root. No post-install step relinks or
  repairs another formula.
- Both keg marketplace files name
  `./libexec/skill-sources/writing/craft/technical-communication`. The
  marketplace check of `release-formula.test.sh` resolves that root inside
  the keg, and `claude plugin validate` accepts the keg.
- `release-formula.test.sh` checks each alias is a relative symlink into the
  same keg. It checks each canonical `originPath` is the private payload. It
  checks regular payload files stay regular, with their manifest Git modes.
  It compares digests with the plain checkouts as before.
- `release.test.sh` refuses a template that installs real source directories
  under `share`. A later Grove release therefore keeps the layout.
- `candidate-keg.sh` builds the projected keg through the template's install
  method. It keeps the links of `brew link` and the digest comparison.
  `grove-tap-states.sh` passes tap states A to D with projected kegs, and
  its link construction follows the new layout.
- The caveats and the formula test describe and check the private payload
  and the public alias. The harness case input and the staging-trial
  fixture name the canonical private path where they mean it.
- `task release:formula:test` passes with the current inputs of
  `docs/ravel-target-inputs.md`. `bash scripts/check.sh` passes. This leaf
  records which Linux checks apply. `task release:smoke` covers the
  unchanged archive on each Linux target. Linux formula linking stays a
  stated limit unless a Linux Homebrew run exists.
- The unreleased changelog states the revised keg layout and the private
  marketplace path.

## Notes

Do not change `plugins/grove` or `plugins/packs.json`. A change there
invalidates the core candidate before core-candidate-restaging-k300.

The installed-smoke seam rejects the retired `libexec/ravel` and
`libexec/skills` layouts. Preserve those checks. `libexec/skill-sources` is
a new path, so make the rejection precise rather than broad.

shared-directory-seam-k294 adds the multi-formula linking seam. This leaf
keeps the single-keg checks honest for the new layout and adds no linking
cases of its own.

## Decisions (running log)

- The formula moves each `SKILL_SOURCES` entry with `Pathname#install` and
  makes its alias with `install_symlink`. Homebrew 7.0.12's
  `install_symlink_p` links relative to the real destination, so each alias
  is `../../../libexec/skill-sources/<id>`. The template cites that source.
- `release.test.sh` refuses any template line whose `install` receiver names
  `share`. The check runs before the projection checks. Two controls, the
  earlier template and a real install beside the projection, must fail with
  that refusal.
- `grove-installed-smoke.sh` accepts exactly two layouts. Without
  `libexec/skill-sources`, it requires the archive's real directories. With
  it, it requires each alias as the relative link and each originPath as the
  private source. The retired-path list stays exact.
- The harness runner's keg sources name `libexec/skill-sources/<id>`, the
  path that Ravel reports. A case that names the public alias is refused.

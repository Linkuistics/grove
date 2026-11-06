# grove-hook-adapters-k230

## Goal

Ship the hook adapters' marketplace files in Grove's keg. Print the
documented registration commands, and measure each harness's registration
from a keg.

## Context

- "Hook adapters" of `docs/specs/codex-skill-provisioning.md`, and its rows
  "Hook adapters" and "Installed distribution" of "Test seams".
- "The hook plugins across a keg change" in
  `docs/homebrew-skill-delivery-evidence.md` holds the earlier readings.
- "The always-on hooks" of `docs/specs/pack-membership.md`.
- The keg layout comes from `grove-keg-sources-k229`.

## Done when

- Grove's tree holds the Claude Code and Codex marketplace files of the keg
  root, outside the repository's own `.claude-plugin/`. Each has the name
  `writing-hooks`. Each names the plugin root
  `./share/ravel/skills/writing/craft/technical-communication`.
- The archive build copies them to `.claude-plugin/marketplace.json` and
  `.agents/plugins/marketplace.json` at the archive root. The manifest lists
  them. The formula installs both directories at the keg root.
- `claude plugin validate` accepts the Claude Code file.
- The formula caveats print the four commands of the table in "Hook
  adapters".
- In isolated homes, each harness registers its adapter from a keg by the
  documented command. On Claude Code, the hooks run and no native
  `technical-communication` skill appears.
- One measurement installs a disposable formula and registers its hooks
  plugin by the `opt` path. It then runs `brew upgrade` with cleanup. A
  Claude Code session after the upgrade runs the hooks from the new keg. The
  same measurement shows the Codex registration break and its repair, and
  records whether a Codex session runs the cached hooks before the repair.
- The evidence document records the versions and the readings.
- `bash scripts/check.sh` passes.

## Notes

Isolated homes only. Do not change the owner's real Claude Code or Codex
registrations.

The documented commands enter Grove's guidance in
`homebrew-delivery-guidance-k237`.

## Decisions (running log)

**Where the files live (2026-11-06).** The two marketplace files live in
`assets/keg-root/`, a directory laid out as the keg root:
`.claude-plugin/marketplace.json` and `.agents/plugins/marketplace.json`.
`assets/` already holds the caller assets that the archive ships. Neither
harness reads a marketplace file below the repository root, so the files
change nothing for a session in this repository. The Codex file copies the
WritingSkills file at the pack commit and changes only the source path.

**Validation (2026-11-06).** `claude plugin validate` 2.1.319 passes the file
in the tree without a look at its source path. In a keg-shaped copy it also
reads the plugin's `plugin.json` and warns that it has no `version`.
`plugins/CONTEXT.md` rejects a version for that reason, so the check is not
`--strict`. `release:formula:test` runs it on the installed keg, where the
source path resolves.

**The measurement's Homebrew (2026-11-06).** The measurement uses a private
Homebrew: a `--shared` clone of the owner's Homebrew repository at its `HEAD`,
with its own prefix in the scratch directory. It links the owner's portable
Ruby and makes its own `bin/brew` read-only. Homebrew's `inherited_sandbox?`
refuses a sandbox that permits writes to that file, and its profile permits
writes under `/private/tmp`.

A disposable formula `grove` in a scratch tap
installs the real native archive with the `install` block of the template,
with one marker line added to `always-on/instruction.md`. The documented
commands then run verbatim, and `$(brew --prefix)` names the scratch prefix.
Nothing writes to the owner's prefix, HOME or harness configuration.

**Codex hook trust (2026-11-06).** Codex 0.166.0 runs a plugin hook only when
the user's `hooks.state` holds a `trusted_hash` for it, or under
`--dangerously-bypass-hook-trust`. `discover_handlers` in
`codex-rs/hooks/src/engine/discovery.rs` of tag `rust-v0.166.0` decides it.
The trust covers the declaration and a fingerprint of the plugin content.

`codex exec` skips an untrusted hook without a report. An interactive start
shows "Hooks need review", with "Trust all and continue". The prototype saw
no hook with the plugin only added, and all hooks after the review. The
documented Codex step therefore needs the review as well. The measurement
drives the real review through tmux.

**The accepted measurement (2026-11-06).** The third run of
`docs/evaluations/hook-adapters/keg-registration.sh` is the reading, and
`keg-registration.out` holds it. The script, the archive, the template and
both marketplace files had the same digests before and after it.

Two earlier runs failed in the setup. Codex draws its composer before the review, so the
first run left too early. A Codex update prompt blocked the second run's
second review, and its first choice runs `brew upgrade --cask codex`. The
script now waits for the review and turns off the update check. The evidence
section "The hook adapters from a keg" holds the readings.

**Codex needs the review after each repair (2026-11-06).** Before the repair,
a Codex session ran the trusted hooks of the cached version-1 copy. After the
repair, the copy held version 2, and no hook ran until a second review. The
documented Codex step therefore ends with the review. The specification's
row and bullet, the caveats, their check and the changelog say so. The owner
design keeps Codex hooks absent, so no owner step changes.

**Final runs (2026-11-06).** `release.test.sh` passed. Its three new controls
each failed against a changed subject: a Codex source path, a formula without
the keg-root install and an extra file in `assets/keg-root`. A plugin manifest
with another name failed the installed-keg check.

`task release:formula:test` passed on the native archive, with
`claude plugin validate` on the installed keg and tap states A to D.
`mise exec -- bash scripts/check.sh` passed all 19 principal checks, with the
same tree digest before and after. An earlier gate run also passed, but an
edit of `docs/RELEASING.md` during it moved a subject, so it does not count.

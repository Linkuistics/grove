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

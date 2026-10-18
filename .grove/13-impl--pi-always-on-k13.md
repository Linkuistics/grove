# pi-always-on-k13

## Goal

Make the skill always on in Pi as far as Pi allows, as the design maps it, with
the same layers, fallback and switch as the other harnesses.

## Context

- Read the design `docs/specs/technical-communication.md` and `harness-hooks-a.md`
  for Pi's extension or hook mechanism and its pitfalls, and reuse the shared
  scripts and switch from `claude-code-always-on-k11`.
- The extension is `harnesses/pi/technical-communication.ts`. It finds the
  skill directory through its own real path, imports only Node's built-in
  modules, keeps its factory to registrations, and catches every error in every
  handler, because a broken extension stops Pi from starting.
- `plugins/install.sh` links the skill into `~/.pi/agent/skills`. This leaf
  makes it also link the extension into `~/.pi/agent/extensions/`, with the same
  ownership and reconciliation rules as its skill links, covered by
  `plugins/install.test.sh`.
- This repository never writes Pi's global `AGENTS.md` (`skill-design-k5`). A
  project memory records that `~/.pi/rules/` is not loaded, verified in Pi's
  `resource-loader.js`; do not rely on it.

## Done when

- In a real Pi session, show by running it which layers work and which fall back
  to the model running the checker, and that the switch turns them off. A project
  memory records that Pi needs `tail -f /dev/null` alongside `script -q` to be
  driven unattended. Put the evidence in the running log.
- If a Pi load check that spends no model quota exists, it runs in
  `scripts/check.sh`.
- `bash scripts/check.sh` is green.

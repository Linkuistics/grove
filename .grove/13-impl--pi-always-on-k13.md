# pi-always-on-k13

## Goal

Make the skill always on in Pi as far as Pi allows, as the design maps it, with
the same layers, fallback and switch as the other harnesses.

## Context

- Read the design `docs/specs/technical-communication.md` and `harness-hooks-a.md`
  for Pi's extension or hook mechanism and its pitfalls, and reuse the shared
  scripts and switch from `claude-code-always-on-k11`.
- Pi reads a global `AGENTS.md` carrying a skills-priming section the human
  maintains outside this repository. A project memory records that `~/.pi/rules/`
  is not loaded, verified in Pi's `resource-loader.js`; do not rely on it.
  What this repository does about the priming section was decided with the human
  in `skill-design-k5`; follow that decision.
- `plugins/install.sh` links the skill into `~/.pi/agent/skills`.

## Done when

- In a real Pi session, show by running it which layers work and which fall back
  to the model running the checker, and that the switch turns them off. A project
  memory records that Pi needs `tail -f /dev/null` alongside `script -q` to be
  driven unattended. Put the evidence in the running log.
- `bash scripts/check.sh` is green.

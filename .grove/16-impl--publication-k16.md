# publication-k16

## Goal

Make the grove ready for the finish session's integration and release: record
the provenance, document the skill for its user, and sweep the repository for
anything the licence posture forbids. This leaf releases nothing. Integration
and release belong to the finish cycle (`CLAUDE.md`, "Finish, integrate, and
release").

## Context

- The licence posture is in the root brief, in `plan-k1`'s log and in the ADR
  from `skill-design-k5`. The sweep procedure is in the spec
  `docs/specs/technical-communication.md`, section *Restatement discipline*. The standard is `~/Downloads/ASD-STE100_ISSUE9.pdf`, page images
  only; scratch notes stay in the scratchpad.
- `plugins/linkuistics/PROVENANCE.md` holds the sources of the skills in this
  plugin. The vendored checker needs an entry: upstream, commit `3944283`, MIT,
  and what we changed.
- `plugins/README.md` and `plugins/CONTEXT.md` list the skills and their terms.
  `docs/USAGE.md` and similar user documents may need a short account of the
  skill. Check what the repository lists skills in, by enumerating the places that
  name an existing linkuistics skill, not by searching a fixed list.

## Done when

- `PROVENANCE.md` carries the entry, and the plugin and context documents list
  the skill. `plugins/CONTEXT.md` already defines **Always-on skill** and
  **Harness entry**; name `technical-communication` as their instance, and bring
  **Symlink install** up to date with the Pi extension link.
- A user document says in plain words what the skill does, that it is on from
  installation, how to turn it off, where the reply log is and how to read it.
  It also gives the commands that add and trust the Codex plugin (from
  `codex-always-on-k12`), says that Pi gets the extension through
  `plugins/install.sh` and that Gemini CLI gets the skill only, and offers the
  optional line for a global `AGENTS.md`. That line names the off file.
- The licence sweep is done as the spec's procedure says, with positive controls
  that were seen to fail first. It finds no text, rule wording or example from the
  standard anywhere in the repository or in `.grove/`, and no use of the
  trademarked name beyond describing the skill as modelled on ASD-STE100.
- A dry run of `plugins/install.sh` shows the skill eligible on all four
  harnesses, and the always-on pieces are in the places the design says.
- The ADR set matches what was built: reconcile it as `ADR-FORMAT.md` directs.
- `bash scripts/check.sh` is green on the whole grove.

## Notes

- The user document also says where to report a failure seen in use: as an issue
  on `Linkuistics/grove` on GitHub, which is where later groves pick it up.

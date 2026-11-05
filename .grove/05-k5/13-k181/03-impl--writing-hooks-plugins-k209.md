# writing-hooks-plugins-k209

## Goal

Add the two hooks plugins of `technical-communication` to WritingSkills.
The ordinary writing plugin declares the Claude Code hooks plugin as a
dependency and declares no hook. Add the Codex marketplace file at the pack
root. Publish the pack commit. This is the second part of the expand step.
Grove keeps its hooks until writing-craft-removal-k211.

## Context

- `docs/specs/pack-membership.md`: "The always-on hooks", with the table of
  three installations, and the row "A second plugin in one repository" of
  "The acceptance of each move".
- `docs/specs/owner-skill-delivery.md`: "Native residue".
- `docs/specs/external-pack-contract.md`: "Distribution". The research
  plugin's `"dependencies": ["grove"]` is the model of a dependency.
- Grove's `plugins/linkuistics/hooks/hooks.json`, which declares six
  events, each with one command that runs
  `skills/technical-communication/scripts/tc-hook`.
- Grove's `.agents/plugins/marketplace.json`, which lists the Codex plugin
  with a `local` source.
- The pack commit of writing-craft-repository-k208.

## Done when

- `plugins/writing/craft/technical-communication/` is the root of a Claude
  Code plugin. It holds a manifest under `.claude-plugin/` and a hooks file
  that Claude Code reads. Each hook runs `scripts/tc-hook` from that root.
  The plugin has no `skills/` directory and lists no skill.
- The ordinary writing plugin names the hooks plugin in its
  `dependencies`. Neither its manifest nor a `hooks/hooks.json` at its root
  declares a hook.
- A pack check counts the hook declarations. It includes dependencies and
  each hooks file that Claude Code discovers by default. It gives one
  declaration for each hook to the ordinary plugin alone, to the hooks
  plugin alone and to both. Watch it fail once against a scratch copy with a
  second declaration.
- `claude plugin validate` accepts each plugin. The pack's `check:plugin`
  task runs it for both.
- A Codex marketplace file at the pack root lists the Codex hooks plugin
  with a `local` source. Its path is the moved skill directory. The
  manifest `.codex-plugin/plugin.json` stays in the skill directory.
- The README names both hooks plugins, the dependency and the reason for it.
- The pack commit is on the public remote. The pack's `task check` passes
  from a clean checkout of it.
- The moves record has a section for this commit: the plugin names, the
  hook count and the published commit.
- Grove's `bash scripts/check.sh` passes. Grove changes only the record.

## Notes

Choose the name of the Claude Code hooks plugin, and record the reason. A
Ravel user enables that plugin by name, and the marketplace lists it beside
the ordinary plugins. The dependency resolves by its bare name in the
marketplace that lists the writing plugin.

The hooks plugin's root holds `SKILL.md`. The owner design asks for a trial
that shows that Claude Code lists no native skill from it. That trial is
writing-hooks-trial-k210. If `claude plugin validate` or a listing in a
scratch home already shows the root `SKILL.md` as a skill, record it. The
trial leaf then repairs the layout.

Do not install, update or remove a plugin in the owner's home. Grove's
marketplace still pins `694140a18dc7`. A native user therefore sees the new
plugins only after the removal leaf changes the pin.

# writing-craft-pack-k181 — brief

## Goal

Add the writing craft to WritingSkills: `technical-communication` with its
always-on layers, scripts, harness adapters and licences, and
`writing-code-walkthroughs`. Supply the native hooks packaging that the owner
design requires.

## Done when

- WritingSkills gains its `craft/` root. Its manifest lists it, and its Ravel
  roots are `["skills", "craft"]`.
- `technical-communication` keeps every resource: `always-on/`, `harnesses/`,
  `references/`, `scripts/`, `LICENSES/` and `.codex-plugin/`. Executable bits
  stay, because the checker runs from the skill directory. The capture holds
  no `__pycache__`.
- The hooks route follows pack-membership-k171. The owner design requires a
  skill-free Claude Code plugin whose hooks run and that lists no native
  `technical-communication` skill. A trial in Claude Code shows both facts.
  Repair the layout if Claude Code lists the root `SKILL.md`.
- The ordinary writing plugin declares the hooks plugin as a dependency and
  declares no hook. Native users of that plugin therefore keep the always-on
  layer. The static count gives one declaration for each hook to either
  plugin alone and to both. A native installation of the ordinary plugin
  alone, in an isolated Claude Code home, runs the hooks.
- The Codex hooks plugin has the owner that pack-membership-k171 names. Its
  marketplace entry points at the moved skill directory.
- The hooks of the `linkuistics` plugin leave Grove with the skill. No hook
  in Grove names a moved script.
- The Grove assets of both skills are where pack-membership-k171 puts them.
  Each check of those assets runs in its owning repository and passes.
- Grove's `bash scripts/check.sh` passes after the removal. The pack's own
  checks pass from a clean checkout of its published commit, which is on the
  public remote.
- The off file still turns off every always-on layer, as
  `docs/adr/the-off-file-is-the-only-switch.md` states, or wherever that
  record lives after pack-membership-k171.
- `docs/repository-moves-record.md` records both roots, the commit, the tree
  ID, the `resourceDigest`, the hooks plugin and the tool requirements:
  `python3` and the shell of `tc-hook`.

## Decomposition

The first child plans this node from the membership decisions.

## Notes

The owner design names this packaging as one of the three changes the moves
must supply before the switch. The marketplace node lists the hooks plugin.

`writing-code-walkthroughs` has evaluations with their own harness scripts.
Those scripts name the skill's current path.

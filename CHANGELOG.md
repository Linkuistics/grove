# Changelog

## Unreleased

- grove moved to its own repo (`Linkuistics/grove`) and is now distributed via
  `brew tap Linkuistics/taps && brew install grove`. The `grove/` directory and
  `scripts/materialise-grove.sh` were removed from this repo; `docs/grove.md`
  now points readers to the new repo.
- Added `grove-start` and `grove-next` shell launchers, bundled with the
  grove skill and materialised alongside it. They collapse the per-session
  restart ritual (`/clear` → `/rename` → kickoff prompt) into a single
  command: `grove-start <name>` creates the worktree at
  `<repo>/worktrees/<name>-grove/` on branch `<name>-grove` and launches a
  pre-named bootstrap session; `grove-next <name>` cd's into the worktree
  and launches a pre-named continuation session. Both work from anywhere
  inside the repo (any worktree) via `git rev-parse --git-common-dir`.
  Updated SKILL.md, docs/grove.md, and the materialise test to match.
- Moved the grove skill out of the `linkuistics` plugin tree
  (`plugins/linkuistics/skills/grove/` → `grove/`) so installing the plugin
  no longer ships a global grove. Grove is materialisation-only — a global
  installed copy would conflict with the per-project pinned copy and
  re-introduce the version-drift problem grove exists to prevent. Updated
  `scripts/materialise-grove.sh`, its test, `README.md`, and `docs/grove.md`.
- Renamed the repo to `Linkuistics/skills` and the plugin to `linkuistics`
  (namespace `linkuistics:`); added an Apache-2.0 licence.
- Added the `grove` skill — a methodology for hierarchical, self-extending,
  git-tracked task-tree workstreams. Bundles three convention files from
  `mattpocock/skills` (MIT); upstream licence in `grove/LICENSES/`.
- Added `scripts/materialise-grove.sh` — copies grove into a consuming repo
  and stamps `VERSION.md`.
- Added `docs/grove.md` — problem, solution, install/update guidance, and example
  prompts. Restructured `README.md` to lead with grove as a top-level section
  alongside the coding-style skills.
- Made grove's heritage explicit up front in `docs/grove.md` and the README's
  `## grove` section: bundling of Matt Pocock's `grill-with-docs` conventions
  and DDD's Ubiquitous Language and bounded-context concepts.
- Made grove's single-worktree-per-grove convention explicit in `SKILL.md`
  (loop preamble) and `docs/grove.md` (the "Git worktrees" subsection now
  reads as directive rather than descriptive).
- Made grove name the session: SKILL.md now instructs the LLM to suggest
  `/rename <project>: <grove-name> grove` on the first turn of grove
  activity, once per session.
- Elevated grilling in `SKILL.md`: a planning task now **opens** with a
  grilling session via `grilling.md`, rather than listing "grills" as one of
  several planning-task activities. Matched in `docs/grove.md` section 2 and
  the "Run a planning task explicitly" prompt.
- Initial release. Coding standards packaged as agent skills:
  - `coding-style` — universal principles (auto-loads on any file).
  - `coding-style-{rust,python,elixir,bash,swift,typescript}` — per-language
    style guides, auto-loading by file extension.
  - `cli-tool-design` — LLM-friendly CLI design guidance, with the audit
    checklist and refactoring sequence split into `references/`.
- Claude Code marketplace manifest (`.claude-plugin/marketplace.json`).
- `install.sh` for symlinking skills into Codex / Gemini CLI.

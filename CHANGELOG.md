# Changelog

## Unreleased

- Added the `using-jujutsu` and `git-to-jj-mapping` skills. `using-jujutsu`
  auto-fires on version-control work: in a jj-enabled repo (a `.jj/` directory
  exists) it drives everything through Jujutsu's native model
  (working-copy-as-commit, `jj new`/`jj describe`, bookmarks, op-log undo);
  otherwise git remains the interface, silently — the skills never convert a
  repo or offer to. `git-to-jj-mapping` is the on-demand git→jj command and
  concept reference, loaded only when a translation is needed. Reconciled the
  existing skills with the jj design: `guardrail` now also gates `jj abandon`
  and `jj op restore` (hook pattern list, SKILL.md table, and test suite; jj
  0.43 has no force-push flag to gate — pushes are lease-checked by default,
  pinned by a defer test), and `decision-records` generalises its "git holds
  the past/history" phrasing to "the VCS holds …" at all six sites. Updated
  the `linkuistics` manifest description/keywords and the README skills table.
- `authoring-conventions`: added **Negation** (steering by prohibition drags the
  forbidden behaviour into context and makes it more available, not less; state the
  positive target, reserve `never`/`don't` for guardrails that can't be phrased
  positively), the **context load / cognitive load** vocabulary for the user-invoked vs
  model-invoked lever plus the **router skill** cure for cognitive-load pile-up, and a
  **sentence-level no-op hunt** (test each sentence against the no-skill default; delete
  failing sentences outright rather than trim words). All three are drawn from
  `mattpocock/skills`' `writing-great-skills` skill (MIT), which postdates this repo's
  prior-art survey. `codebase-design`: added the concrete parallel-sub-agent "design it
  twice" procedure (divergent per-agent briefs: minimize-interface / maximize-flexibility
  / optimize-common-caller / ports-and-adapters), from the same upstream's
  `DESIGN-IT-TWICE.md`. Refreshed the prior-art survey's mattpocock citations with a dated
  note pointing at `writing-great-skills/{SKILL,GLOSSARY}.md` @ `d574778` as the current
  canonical source.
- Added the `decision-records` skill — ADRs as a **minimum coherent set**
  describing the design's current state (current-state over changelog,
  edit/merge/split/delete in place, identity by slug not number, the
  when-to-write test, a minimal template). The minimal template, the three-part
  when-to-write test, and the qualifying examples are distilled from
  `mattpocock/skills`' ADR-format material (MIT); the coherent-set framing is
  original. Updated the `linkuistics` manifest description/keywords and the
  README skills table.
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

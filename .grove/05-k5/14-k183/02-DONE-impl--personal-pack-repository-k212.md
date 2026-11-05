# personal-pack-repository-k212

## Goal

Create `~/Development/AgenticOrchestration/PersonalSkills` as a colocated
Git/Jujutsu repository with two Claude Code plugins. The ordinary plugin has
the craft root with `machine-environment`, `using-jujutsu` and
`authoring-conventions`. The second plugin holds only `guardrail`. Publish the
first pack commit on a public remote. This is the expand step of the move.
Grove keeps its copies, its marketplace entry and its documents until
personal-removal-k214.

## Context

- `docs/specs/pack-membership.md`: "The acceptance of each move", the row "A
  source whose only root is `craft`" and the paragraph on
  `personal-pack-k183`. Also "The Grove assets of moving craft skills".
- `docs/specs/owner-skill-delivery.md`: the `personal` row of "Ravel sources"
  and the `guardrail` row of "Native residue".
- `docs/specs/external-pack-contract.md`: "Layout", "References between
  skills", "Provenance and licences", "Distribution", "Checkout boundary" and
  "Proven facts".
- DesignSkills and WritingSkills as the models of a `craft/` root, its
  manifest field, its README section and the pack's `Taskfile.yml`.
  WritingSkills is the model of a second plugin with a dependency.
- The baseline of owner-ravel-skill-delivery-k37:
  `docs/owner-skill-inventory.tsv` and
  `docs/evaluations/owner-skill-delivery/inventory.py`.
- `plugins/install.test.sh`, `make_craft_only_pack`: the craft-only fixture
  of design-pack-k175.

The moving files at planning. Enumerate again with `--hidden` and with
positive and cross-tree controls before you rely on this list.

- `plugins/linkuistics/skills/machine-environment/SKILL.md`.
- `plugins/linkuistics/skills/using-jujutsu/`: `SKILL.md` and two files in
  `references/`.
- `plugins/linkuistics/skills/authoring-conventions/SKILL.md`.
- `plugins/linkuistics/skills/guardrail/`: `SKILL.md`,
  `scripts/guardrail-hook.sh` and `scripts/guardrail-hook.test.sh`.
- `plugins/linkuistics/PROVENANCE.md`: the table of authoring sources and the
  sections of `guardrail` and `using-jujutsu`.

## Done when

- `plugins/personal/craft/` holds the three craft skills. The manifest of
  `plugins/personal` lists `"skills": ["./craft/"]`. The plugin has no
  `skills/` directory. The README gives the Ravel source `personal` with the
  root `["craft"]`.
- `ravel skills resolve` lists the three craft skills under the root
  `["craft"]`, and it lists no `guardrail`.
- The `guardrail` plugin holds `skills/guardrail/` with its two scripts. The
  frontmatter hook names `${CLAUDE_PLUGIN_ROOT}/skills/guardrail/scripts/`,
  so the plugin root holds `skills/guardrail/` and the hook keeps its bytes.
  No Ravel root of the pack reaches the plugin.
- The ordinary plugin declares the `guardrail` plugin in its `dependencies`.
  Record the reason in the README.
- `claude plugin validate` accepts each plugin. The pack's `check:plugin`
  task runs it for both.
- Each moved skill keeps its bytes and its Git modes, or the record names the
  change. `authoring-conventions` and `guardrail` keep
  `disable-model-invocation: true`. Each skill matches its baseline entry or
  a recorded change, as order 2 of the cutover contract requires.
- `using-jujutsu` names `using-git-worktrees` by its bare name, by rule 2 of
  "References between skills". The record names that change.
- The pack's `task check` runs `guardrail-hook.test.sh` and ShellCheck on
  both guardrail scripts. Watch each check fail once against a scratch
  mutation. Grove's `scripts/check.sh` runs neither check today.
- `plugins/personal/PROVENANCE.md` holds the authoring table and the
  `using-jujutsu` section. `plugins/guardrail/PROVENANCE.md` holds the
  `guardrail` section. The pack has a licence.
- The pack commit is on the public remote. Its `task check` passes from a
  clean checkout of that commit.
- `plugins/install.sh --pack` links the three craft skills of a plain
  checkout of the published commit, in an isolated home. A later run removes
  the link of a removed craft skill, as the craft-only fixture shows. The
  installer reports `assumes-personal-setup` for `machine-environment`.
- The moves record has a PersonalSkills section for this commit. It names
  the source paths, the Grove source commit, the byte changes, the asset
  classification, the checks and the published commit.
- Grove's `bash scripts/check.sh` passes. Grove changes only the record.

## Notes

The dependency is a planning decision. A native user of
`linkuistics@linkuistics` gets `guardrail` today. The dependency keeps that
for a native user of the ordinary plugin, as the writing plugin keeps its
hooks. A Ravel user enables the `guardrail` plugin alone. If the expand
finds a reason against the dependency, record it and tell
guardrail-plugin-trial-k213.

Choose the plugin names, and record the reason. The Ravel source ID is
`personal`, and the native residue names the plugin `guardrail`.

Classify each Grove file that names a moving skill by the three tests of the
membership decision. Planning found no Grove test that reads a moving
skill's bytes. The citation rows of `plugin_fallback.rs` and the assertion of
`commit_guidance.rs` read Grove's own files, so they stay. The pack therefore
needs no `GROVE_COMMIT` and no Grove checkout, unless the expand finds such a
read.

`authoring-conventions` says "this marketplace" and "in this repo". Those
words named Grove. Decide whether they stay true in the pack, and record the
result. `machine-environment` keeps its rule for persistent configuration.

Creating the public `Linkuistics/PersonalSkills` remote is within the root
brief's authority, as for the earlier packs.

Do not install, update or remove a plugin in the owner's home. A session
that reads a personal skill during this move reads the owner's native copy
until rollout's switch. The mise entry of the checkout is rollout's work, by
order 6 of the cutover contract.

## Decisions (running log)

- The source Grove commit is `4cfac62958815219ba5a71941674244a05ff9b56`, the
  head of Grove's public bookmark `gh-issue-25`. No moving file changed
  between it and `0eacec3a47b2`, the parent of this leaf's Grove change.
- The plugin names are `personal` and `guardrail`. `personal` equals the
  Ravel source ID, as `research`, `design` and `writing` do. `guardrail` is
  the name of the native residue and of its one skill.
- The dependency stays. The expand found no reason against it. `guardrail`
  is user-invoked, so a native user of `personal` pays no standing context.
- `authoring-conventions` keeps its bytes. "This marketplace" names the
  Linkuistics marketplace, which lists the pack after the removal. "This
  repo" names the repository whose skill the author edits.
- The pack adds `check:frontmatter`. Grove's `install.test.sh` checks that
  each bundled skill declares `harnesses:`, and the removal ends that check
  for these skills. The same task checks `disable-model-invocation: true` on
  the two user-invoked skills.
- The installer proof runs Grove's `install.sh` from a scratch export of the
  source commit without `plugins/linkuistics`. Grove's live tree refuses the
  pack, because both plugins ship the same skill names.
- The pack commit is `ac98636608b013391bf4b4ca0dc6c9c1506ff9d0`. The
  classifier refused `gh repo create` inside a compound command. The owner
  had already allowed `Bash(gh repo create *)`, and the command alone ran.
  The public `main` is that commit.

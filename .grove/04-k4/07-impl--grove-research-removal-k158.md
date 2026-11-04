# grove-research-removal-k158

## Goal

Remove the research skills and their family procedure from Grove. Reconcile
every Grove consumer in the same change, so that Grove stays green with
ResearchSkills as the owner. This is the contraction step and the move itself.

## Context

- `docs/specs/external-pack-contract.md`, especially "Rule placement", "The
  kind table", "Conformance" and "Distribution".
- The published ResearchSkills commit from research-pack-repository-k157.
- The consumer inventory in the commit of external-research-pack-k10. The
  consumers it names are below. Search again before you rely on the list.

Consumers that change:

- The spine: the family-file list and description in `SKILL.md`, and
  `TASK-FORMAT.md`'s kind table, as "The kind table" states.
  `references/execute.md` and `references/decompose.md` keep their label
  rules, because the contract keeps them in core.
- `plugins/grove/conformance/rules.tsv`: remove the eight rows that moved to
  the pack, and remove the named set `research` in `conformance.sh`.
- Tests: `the_guidance_counts_the_kind_set_correctly` in
  `crates/grove-llm/tests/session_kind_guidance.rs`. Check the directory sweeps
  in `testing/support.rs`, `crates/grove-loop/tests/prompt.rs` and
  `crates/grove/tests/plugin_fallback.rs`. Removal must not hide a check that
  still applies.
- Distribution: the `git-subdir` entry with `sha` in
  `.claude-plugin/marketplace.json`, the `grove` plugin descriptions, and
  `--pack` in `plugins/install.sh`, with `install.test.sh` and
  `plugins/README.md`.
- Documents: `plugins/grove/README.md` (the fatness rule) and `CONTEXT.md`
  ("Family reference file", "Spine skill / kind skill", the new **Family
  skill** and the kind set). Each states the branch for a family across packs,
  as "Rule placement" states. `plugins/CONTEXT.md` gains **Pack**, **Source
  directory** and **Directed load**. Also `docs/ARCHITECTURE.md` (plugin roots,
  conformance and loaded-path budgets) and `docs/USAGE.md`. Check the
  source-exact walkthroughs that quote changed code.
- `Taskfile.yml` and `scripts/check.sh`: the task `check:packs`, which reads
  the marketplace entry, as the contract states.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim. `CHANGELOG.md` history stays as
  history.
- Research labels, leaf names, policy routes and `grove-llm` behaviour are
  unchanged. The policy assets and their tests pass unchanged.
- Core conformance and `task check:packs` pass with the pinned ResearchSkills
  commit. The contract's controls fail as required.
- The marketplace lists the research plugin from its own repository, as the
  contract states. The manual installer route for the pack works and is
  documented.
- `bash scripts/check.sh` passes with the Ravel 2.0 build of the research
  recipes. Record the Ravel build identity and the ResearchSkills commit.
- After the removal commit exists, a new ResearchSkills commit records it as
  `GROVE_COMMIT`. Its `plugins/research` is unchanged. Push the line to the
  bookmark `gh-issue-25` on Grove's remote before you publish that pack
  commit. The full `task check` of the new pack commit passes from a clean
  checkout, with the committed public URL and no override. Publish it, and
  record both pack commits and the pushed Grove commit.
- `conformance.sh` keeps the prefix sets `review-*` and `integrate-review-*`.
  Only the named set `research` goes.

## Notes

This workstream's line reaches `main` only in finish. The marketplace that the
owner's Claude Code follows therefore does not change in this leaf. A new
plugin entry needs an install before a native user receives it. Record that
consequence for discovery-rollout-k6. If the change is too large for one
session, decompose it. Keep the documentation with the code change it
describes.

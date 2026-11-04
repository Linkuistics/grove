# grove-research-removal-k158

## Goal

Remove the research skills and their family procedure from Grove. Reconcile
every Grove consumer in the same change, so that Grove stays green with
ResearchSkills as the owner. This is the contraction step and the move itself.

## Context

- `docs/specs/external-pack-contract.md`.
- The published ResearchSkills commit from research-pack-repository-k157.
- The consumer inventory in the commit of external-research-pack-k10. The
  consumers it names are below. Search again before you rely on the list.

Consumers that change:

- The spine: the family-file list and description in `SKILL.md`, and
  `TASK-FORMAT.md`'s kind count and research row. `references/execute.md` and
  `references/decompose.md` keep their label rules unless the contract moves
  them.
- `plugins/grove/conformance/rules.tsv`: the eight rows owned by moving files,
  and the `static(research)` load set in `conformance.sh`.
- Tests: `the_guidance_counts_the_kind_set_correctly` in
  `crates/grove-llm/tests/session_kind_guidance.rs`. Check the directory sweeps
  in `testing/support.rs`, `crates/grove-loop/tests/prompt.rs` and
  `crates/grove/tests/plugin_fallback.rs`. Removal must not hide a check that
  still applies.
- Distribution: `.claude-plugin/marketplace.json`, the `grove` plugin
  description in `plugins/grove/.claude-plugin/plugin.json` and
  `plugins/install.sh` with its documentation in `plugins/README.md`.
- Documents: `plugins/grove/README.md`, `CONTEXT.md` ("Family reference file"
  and the kind set), `plugins/CONTEXT.md`, `docs/ARCHITECTURE.md` (plugin roots,
  conformance and loaded-path budgets) and `docs/USAGE.md`. Check the
  source-exact walkthroughs that quote changed code.
- `Taskfile.yml` and `scripts/check.sh`: the aggregate conformance over core
  and a ResearchSkills checkout at the recorded commit, as the contract states.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim. `CHANGELOG.md` history stays as
  history.
- Research labels, leaf names, policy routes and `grove-llm` behaviour are
  unchanged. The policy assets and their tests pass unchanged.
- Core conformance and the aggregate conformance with ResearchSkills pass.
  The contract's controls fail as required.
- The marketplace lists the research plugin from its own repository, as the
  contract states. The manual installer route for the pack works and is
  documented.
- `bash scripts/check.sh` passes with the Ravel 2.0 build of the research
  recipes. Record the Ravel build identity and the ResearchSkills commit.

## Notes

This workstream's line reaches `main` only in finish. The marketplace that the
owner's Claude Code follows therefore does not change in this leaf. A new
plugin entry needs an install before a native user receives it. Record that
consequence for discovery-rollout-k6. If the change is too large for one
session, decompose it. Keep the documentation with the code change it
describes.

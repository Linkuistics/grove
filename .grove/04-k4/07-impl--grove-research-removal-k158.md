# grove-research-removal-k158

## Goal

Remove the research skills and their family procedure from Grove. Reconcile
every Grove consumer in the same change, so that Grove stays green with
ResearchSkills as the owner. This is the contraction step and the move itself.

## Context

- `docs/specs/external-pack-contract.md`, especially "Rule placement", "The
  kind table", "Conformance" and "Distribution".
- The published ResearchSkills commit from research-pack-repository-k157:
  `dc3a4812db3b0dbfef42c8cd6a0aa291e43dd545` on
  `https://github.com/Linkuistics/ResearchSkills.git`. Its `plugins/research`
  has the tree ID `79dc2a530cd2d30cd088073423437bb18d61b401`. That leaf's
  decision log records the aggregate before the removal and a scratch removal
  of core. With the pack, the scratch removal passed with core's baseline
  counts: 116 single-source checks, 2783 loaded-path checks and 72 unpinned
  rows.
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

## Decisions (running log)

- The removal deletes `grove-research-a/`, `grove-research-b/`,
  `grove-combine-research/` and `grove/references/research.md`, the eight
  manifest rows of those files and the named set `research` in
  `conformance.awk`. The engine owns the named sets, not `conformance.sh`.
  The prefix sets and `editorial` stay.
- Core alone after the removal: 116 single-source checks, 2424 loaded-path
  checks, 64 unpinned rows. The aggregate with the pinned pack commit
  `dc3a4812db3b`: 116, 2783 and 72, the baseline that
  research-pack-repository-k157 predicted. The pack alone: 14 loaded-path
  checks and 3 external edges.
- `task check:packs` runs `scripts/check-packs.sh`. A script, and not inline
  Taskfile shell, lets `scripts/check-packs.test.sh` give it a fixture
  marketplace and cache. The script reads the marketplace with `python3 -I`.
  The test has eight controls over local Git fixtures and no network. Three
  deliberate mutations of the script each failed a control: no `sha` check,
  reuse of a dirty cache, and no aggregate run.
- `scripts/check.sh` runs `task check:packs` and the controls as two principal
  checks. `check:skills` runs them too.
- `install.sh --pack <dir>` requires the shape `plugins/<plugin>/skills/`,
  because `is_ours` recognises a link by that segment. It refuses a skill
  name that two scanned plugins ship, before any link changes. The workspace
  guard covers each pack's checkout, two levels above the plugin directory.
  Ten new cases. Three mutations each failed a case: no reconciliation of the
  pack's plugin name, no duplicate check, no guard of the pack.
- The kind-table test is now `the_kind_table_agrees_with_core_and_counts_no_set`,
  with the control test `the_kind_table_check_fails_on_a_wrong_table_and_a_count`.
  The count sweep folds case and also matches `**<cardinal>**`. The heading
  "The twenty-three kinds" and a wrong pack cell each failed it.
- Removal must not hide a check that still applies. `testing/support.rs`
  `kind_labels()` fed the environment scrub (`GROVE_RESEARCH_A_MODEL` and the
  others) and the round trips in `kind.rs` and `leaf.rs`. It now adds the
  labels of the kind table's external rows to core's directories. A malformed
  label in the research row failed `kind.rs`, so the external rows reach it.
  `prompt.rs` sweeps the composition claims over that set and keeps its
  shipping claim over core's directories. `plugin_fallback.rs` loses nothing,
  because the research files carry no `linkuistics:` citation.
- The frozen loaded-path table of `docs/ARCHITECTURE.md`, the two inventory
  TSVs and `CHANGELOG.md` keep their research rows as history.
- `README.md`, `plugins/README.md` and `plugins/CONTEXT.md` had no conversion
  marker. Each now opens with a `tc: pending` marker, and the new passages are
  `tc: on` regions that pass the checker.
- Stale-location sweep: an enumeration of `grove-research-[ab]`,
  `grove-combine-research`, `research-family` and `research.md` over the whole
  tree with `--hidden`, excluding `.jj`, `.git`, `target` and `.grove`. Every
  hit is the contract, its diagrams, the owner inventory and its script, a
  frozen record, `CHANGELOG.md`, a test fixture or a new correct statement. One
  stale claim, in `docs/specs/module-decomposition.md`, was corrected. The
  instrument found a planted path in a hidden scratch directory, and a mutated
  pattern found nothing.
- The marketplace consequence for discovery-rollout-k6 is in that node's
  brief.

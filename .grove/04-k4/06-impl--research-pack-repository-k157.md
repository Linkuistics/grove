# research-pack-repository-k157

## Goal

Create and publish the ResearchSkills repository with the research kinds, the
combiner and their family procedure. Grove keeps its own copies until
grove-research-removal-k158, so no consumer changes in this leaf.

## Context

- `docs/specs/external-pack-contract.md`: "Layout", "The family procedure",
  "References between skills", "Conformance", "Provenance and licences" and
  "Distribution". The review leaf external-pack-contract-k163 and any
  integration step after it come first.
- The moving files: `plugins/grove/skills/grove-research-a/`,
  `grove-research-b/`, `grove-combine-research/` and
  `plugins/grove/skills/grove/references/research.md`.
- The pack-facing conformance invocation from pack-conformance-runner-k156.
- `linkuistics:machine-environment` for the Taskfile and mise conventions of a
  new repository.

## Done when

- `~/Development/AgenticOrchestration/ResearchSkills` exists as a colocated Git
  and Jujutsu repository. Its source directory is `plugins/research`, and its
  kind directory is `plugins/research/skills/`.
- `research.md` is `skills/research-family/SKILL.md`, with the contract's
  frontmatter and its body byte for byte. The three kinds have the contract's
  opening text and no other text change. Record each change against the files
  in Grove.
- `plugins/research/.claude-plugin/plugin.json` sets the name `research`, the
  licence `Apache-2.0` and the dependency `grove`, with no `version`.
- `plugins/research/conformance/rules.tsv` holds the eight rows of the moving
  files. The six family rows name `research-family/SKILL.md` and
  `static({research-a,research-b})`.
- The repository root holds `LICENSE`, `.gitignore` with `.cache/`, and a
  README. The README names the source directory, the roots, the kinds, the
  Ravel record, the install commands and the provenance.
- The Taskfile has `GROVE_REPOSITORY`, `GROVE_COMMIT` and the tasks that the
  contract lists. `GROVE_COMMIT` names the commit of
  pack-conformance-runner-k156. `GROVE_REPOSITORY` keeps the public URL.
- Before the pack's first publication, push this workstream's line to the
  bookmark `gh-issue-25` on Grove's remote through jj. "The checker without a
  copy" states the rule. The recorded commit must be an ancestor of the
  pushed bookmark. Record the pushed commit.
- From a clean checkout, with the committed public URL and no override, the
  pack-alone check and `check:plugin` pass. The aggregate fails only because
  each moving skill and each moved row is a duplicate. Record that output as
  the expected state before the removal.
- `ravel skills resolve --json`, from the Ravel 2.0 build of the research
  recipes in a scratch HOME, resolves the working copy as a `local` source.
  Record the `resourceDigest`, the descriptor identity and the tree ID of
  `plugins/research`.
- The repository is published on the public `Linkuistics/ResearchSkills`
  remote through jj. Its first commit names the Grove commit and the four
  source paths. Record the published commit.

## Notes

The root brief and the node brief authorise the creation of this public
remote. Order 2 of the owner cutover contract already publishes this
workstream's line, and this leaf publishes it earlier. Never push `main` here.
Do not change Grove's skills, marketplace or installer in this leaf.
Run each command outside the Grove working copy. Import no Git history.

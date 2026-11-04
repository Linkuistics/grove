# research-pack-repository-k157

## Goal

Create and publish the ResearchSkills repository with the research kinds, the
combiner and their family procedure. Grove keeps its own copies until
grove-research-removal-k158, so no consumer changes in this leaf.

## Context

- `docs/specs/external-pack-contract.md`: layout, references to core,
  provenance, licences, history and the pack's checks.
- The moving files: `plugins/grove/skills/grove-research-a/`,
  `grove-research-b/`, `grove-combine-research/` and
  `plugins/grove/skills/grove/references/research.md`.
- The pack-facing conformance invocation from pack-conformance-runner-k156.
- `linkuistics:machine-environment` for the Taskfile and mise conventions of a
  new repository.

## Done when

- `~/Development/AgenticOrchestration/ResearchSkills` exists as a colocated Git
  and Jujutsu repository, in the layout that the contract states.
- The moved skills and the family procedure have the reference changes that
  the contract requires, and no other text change. Record each change against
  the files in Grove.
- The repository holds its licence, provenance and the Claude Code plugin
  manifest. It holds a README that names the source directory and roots.
- The repository's `Taskfile.yml` runs its own checks, including pack
  conformance, from a clean checkout. The checks pass.
- `ravel skills resolve --json`, from the Ravel 2.0 build of the research
  recipes in a scratch HOME, resolves the working copy as a `local` source.
  Record the `resourceDigest` and the descriptor identity.
- The repository is published on the public `Linkuistics/ResearchSkills`
  remote through jj. Record the published commit.

## Notes

The root brief and the node brief authorise the creation of this public
remote. Do not change Grove's skills, marketplace or installer in this leaf.
Run each command outside the Grove working copy.

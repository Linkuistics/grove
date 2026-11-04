# design-pack-k175 — brief

## Goal

Create `~/Development/AgenticOrchestration/DesignSkills`. Move the design
chain into its kind directory: `grove-design` with its assets and references,
`grove-review-design` and `grove-integrate-review-design`. Move the design
craft into its craft directory: `decision-records`, `codebase-design`,
`cli-tool-design` and `model-led-development`. This is the first pack with a
`craft/` root.

## Done when

- DesignSkills follows the contract's layout for a pack with craft skills.
  The manifest lists `"skills": ["./craft/"]`, and the Ravel roots are
  `["skills", "craft"]`. A plain checkout resolves with both roots.
- The moved kinds take the bare-name opening. Each resource of
  `grove-design`, such as its diagram viewer and its discussion reference,
  stays inside its own skill and resolves on every route.
- The craft skills keep their bytes and resources, or the record names each
  change. Their checks live in DesignSkills. Conformance covers only the
  kind directory.
- The ADR and spec formats are where pack-membership-k171 puts them, and
  every reference to them resolves.
- The craft skills' provenance and Grove assets are where pack-membership-k171
  puts them. Grove's tests that name `grove-design` follow its rule.
- The kind rows move by the contract's move order, and both pack commits are
  on the public remote. Grove's marketplace pins the first one.
- The pack passes alone and in the aggregate. Grove's `bash scripts/check.sh`
  passes after the removal. The proofs that pack-membership-k171 assigns to
  the first `craft/` root pass.
- `plugins/linkuistics` no longer holds the four craft skills, and its
  manifest and description agree with what it still holds.
- `docs/repository-moves-record.md` records the source root, both roots, the
  commits, the tree ID, the `resourceDigest` and the external edges.

## Decomposition

The first child plans this node from the membership decisions and from the
evidence of requirements-pack-k173.

## Notes

`model-led-development` cites `doubt-driven-development`, which moves to
EngineeringSkills after this node. Name it by bare name, so the citation
stays correct across that move. A citation across packs is not a directed
load. Keep it a citation, as the dependency decision record requires.

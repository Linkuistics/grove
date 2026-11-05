# editorial-pack-k179 — brief

## Goal

Create `~/Development/AgenticOrchestration/WritingSkills` with the editorial
kinds: `grove-draft`, `grove-copy-edit`, `grove-art` and `grove-proof`. Move
the editorial family file out of the spine into the family skill
`editorial-family`. This is the first family skill after `research-family`.

## Done when

- WritingSkills has the layout of ResearchSkills, with the root `skills`.
  writing-craft-pack-k181 adds its `craft/` root and its hooks plugin later.
- `references/editorial.md` moves to `editorial-family/SKILL.md`. That file
  gains the frontmatter that the contract's "The family procedure" gives
  `research-family`, changed for the editorial family. Its body changes only
  where a path cannot resolve from the family skill, and the record names each
  change. No copy stays in the spine.
- Each editorial kind opens with the bare-name text. It loads `grove` and
  then `editorial-family`, and it names no spine path for the family file.
- The rows that `editorial.md` owns move to the pack's manifest with the
  owner `editorial-family/SKILL.md`. The runner's named set `editorial`
  becomes an explicit list of the four kinds, as "Manifest changes" states.
- The spine's family-file list, the fatness rule of `plugins/grove/README.md`,
  the glossary entry "Family reference file", the `grove` plugin descriptions
  and the kind table change as the research move changed them.
- The kind rows move by the contract's move order. Both pack commits are on
  the public remote, and Grove's marketplace pins the first one.
- The pack passes alone and in the aggregate. Grove's `bash scripts/check.sh`
  passes after the removal. The proofs that pack-membership-k171 assigns to a
  family skill outside research pass.
- `docs/repository-moves-record.md` records the source root, the commits, the
  tree ID and the `resourceDigest`.

## Decomposition

The first child plans this node. The research move is the model, because it
also moved a family file into a family skill.

editorial-pack-plan-k180 cut three leaves. Each leaf leaves Grove's
`bash scripts/check.sh` green.

1. `writing-pack-repository-k205`: the expand step. It publishes the first
   pack commit. Grove still ships its copies.
2. `editorial-removal-k206`: the contraction step. It removes the kinds and
   the family file with every consumer, and it runs the static proof of the
   boundary. It publishes the second pack commit.
3. `editorial-directory-trials-k207`: the directory trial of the boundary in
   each harness.

The family file names `references/execute.md`, and the family skill loads
nothing, so a byte-for-byte copy fails the pack-alone check.
writing-pack-repository-k205 holds the decided change and its evidence.

## Notes

A paused grove works on the editorial pipeline. This grove does not edit its
tree. Record for rollout's paused-grove text that the editorial kinds and
their family file now ship in WritingSkills.

The editorial kinds name the document's prose contract. Check each reference
to a craft skill in the moved text. It must be a bare name.

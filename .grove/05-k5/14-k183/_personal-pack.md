# personal-pack-k183 — brief

## Goal

Create `~/Development/AgenticOrchestration/PersonalSkills` with the personal
craft: `machine-environment`, `using-jujutsu` and `authoring-conventions`.
Ship `guardrail` as a native Claude Code plugin that holds only that skill.
Then remove the empty `linkuistics` plugin from Grove.

## Done when

- PersonalSkills has one source directory whose only Ravel root is `craft`,
  as the owner design's source table states. Its manifest lists that root.
- The proofs of the craft-only row of pack-membership-k171 pass.
  `plugins/install.sh --pack` links and reconciles the craft skills of a
  plain checkout of the published commit. `check:packs` reports its
  directory-only check for the source.
- A second plugin holds only `guardrail`, with its scripts, its frontmatter
  hook and its `/guardrail` command. No Ravel root reaches it. A trial in
  Claude Code shows the command and the hook working from that plugin.
- `authoring-conventions` keeps `disable-model-invocation: true`.
- The craft skills keep their bytes and resources, or the record names each
  change. Their checks live in PersonalSkills.
- `plugins/linkuistics` is gone from Grove, with its manifest, its
  provenance and its marketplace entry. Each Grove reference to it is
  reconciled. Grove's `bash scripts/check.sh` passes.
- The commit is on the public remote. No personal source is under `.config`.
  Machine settings that point to the source follow the current mise process
  and the maintained manual. Nothing recreates chezmoi.
- `docs/repository-moves-record.md` records the root, the commit, the tree
  ID, the `resourceDigest`, the guardrail plugin and the tool requirements:
  `jj`, `mise`, `task` and the Obsidian manual.

## Decomposition

The first child plans this node from the membership decisions. This is the
last craft move, so it also removes the `linkuistics` plugin shell.

## Notes

pack-membership-k171 gives the always-on hooks of `technical-communication`
to WritingSkills, and the root brief now says so. PersonalSkills keeps only
the user-invoked hook of `guardrail`.

`using-jujutsu` cites `superpowers:using-git-worktrees`, a third-party skill.
Apply rule 2 of the reference rules to that citation.

`machine-environment` is the owner's machine contract. The moved skill must
keep its rule for persistent configuration. A session that reads it during
this move still reads the native copy until the switch.

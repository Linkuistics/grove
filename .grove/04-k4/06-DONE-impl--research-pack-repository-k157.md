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

## Decisions (running log)

- `GROVE_COMMIT` is `6eb881035a8eea1f4bb91ca1394dcbe7ddf1ec2e`, the commit
  of pack-conformance-runner-k156. It still ships the moving files, as "The
  checker without a copy" requires for the first pack commit.
- The pack adds no mise file. Its tasks need only `bash`, `git` and `task`,
  and Grove pins none of them.
- The pushed Grove commit is `6eb881035a8eea1f4bb91ca1394dcbe7ddf1ec2e`.
  `jj git push --remote origin --bookmark gh-issue-25` created the remote
  bookmark there, and `git ls-remote` shows it. It equals `GROVE_COMMIT`.
- The kind skills keep each line outside the contract's opening. A diff
  against Grove's copies shows only the opening paragraphs. The family body
  equals `references/research.md` byte for byte (`cmp` after the
  frontmatter).
- `check` runs `check:plugin` before `check:conformance`. The plugin result
  then shows when the aggregate fails before the removal.
- `claude plugin validate` (Claude Code 2.1.318) passes the plugin with two
  warnings: no `version` and no icon. The contract sets no `version`, and core
  has the same icon warning. The README at the repository root satisfies the
  plugin directory's README check.
- Expected state before the removal. From a clean plain-Git checkout of the
  pack commit, `task check` exits nonzero. `check:plugin` passes. The pack
  alone passes with 14 loaded-path checks and 3 external edges, the loads of
  `grove`. The aggregate fails 25 assertions: 3 duplicate skills, 8 duplicate
  rows, 2 unshipped owners and 12 family rows off the loaded paths of
  `research-a` and `research-b`. The last two classes follow from the
  duplicates, because the runner takes core's copy of each duplicated skill.
- Control for that claim: a scratch copy of core at `6eb881035a8e` lost the
  four moving files, their eight rows and the spine's line for
  `references/research.md`. With the pack, it gave a passing aggregate. Its
  counts equal core's baseline: 116 single-source checks, 2783 loaded-path
  checks and 72 unpinned rows. grove-research-removal-k158 must reach that
  state.
- The README names the contract by its Grove path, not by a link to Grove's
  `main`. The contract reaches `main` only in finish, so a link would be dead
  until then. The edit changed the pack commit to
  `dc3a4812db3b0dbfef42c8cd6a0aa291e43dd545`, before any publication. The
  edit is outside `plugins/research`, so the tree ID stays the same.
- The Git tree ID of `plugins/research` at `dc3a4812db3b` is
  `79dc2a530cd2d30cd088073423437bb18d61b401`.
- Ravel resolution. The research recipe installed the build of
  `skill-directory-delivery` at `71f70fcd3f92` into a scratch prefix. The
  front's SHA-256 is `26b487afb957c233848e8e360e29c73539a33ba89fc3444b26ad13a2eb4bf0a3`,
  the value of `builds.json`, and the prefix has no `libexec`. The build
  reports the version string 1.1.0. Its digest identifies it as the recorded
  2.0-line build. A scratch HOME held the recipe's settings, with `grove` at a
  plain checkout of `6eb881035a8e` and `research` at the working copy.
  `ravel skills resolve --json` exited 0 with the status `resolved`. It gave
  `research` the type `local` and the `resourceDigest`
  `7fb3d473f66feafb49afd8846fd94cf3e1fb7b01c584d4ad5f0e54814a8b3fdb`. The
  `grove` digest was
  `30cc5c72024f2379ac1b4091136737e45384977d397022a434381f4b99f554aa`. The
  descriptor identity was
  `e4f2c430c5fff3ffabacdb0ef5db01d351f4e689cb47ea7c3bc912615eea4fbd`, equal to
  the SHA-256 of `directories.json`. The research snapshot held exactly the six
  files of `plugins/research`.
- The same settings with `research` at the clean plain checkout gave the same
  research digest. A missing checkout refused as `skill_source_missing` with
  exit 3, at `skills.sources[1].path`.
- The permission system of this harness refused
  `gh repo create Linkuistics/ResearchSkills --public` as the creation of a
  public surface. The human ran that command at the prompt and created the
  public repository.
- The published commit is `dc3a4812db3b0dbfef42c8cd6a0aa291e43dd545`.
  `jj git push --remote origin --bookmark main` published it, and
  `git ls-remote` shows `main` and `HEAD` there.
- Evidence from the published commit. A plain Git checkout from GitHub, with
  the committed public URL and no override, has the tree ID `79dc2a530cd2` and
  a clean status. `task check:plugin` passes. `task check` gives the expected
  state above: the pack alone passes, and the aggregate fails the same 25
  assertions. Ravel resolves that checkout as `local` with the same research
  digest `7fb3d473f66f`.

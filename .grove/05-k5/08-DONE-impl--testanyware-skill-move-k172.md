# testanyware-skill-move-k172

## Goal

Move the `using-testanyware` skill from Grove's `plugins/testanyware` into the
existing repository `~/Development/AbstractingComputers/TestAnyware`. Check it
there, publish it, and make Grove consume the published commit.

## Context

At planning, TestAnyware was colocated Git and jj, with `origin` at
`https://github.com/Linkuistics/TestAnyware.git`. A second jj workspace
`gh-issue-14` at `../TestAnyware.gh-issue-14` held paused work of another
grove. TestAnyware's root holds `target/`, so its Ravel source cannot be the
repository root. It had no skill file.

The skill is one `SKILL.md`. Its capabilities are the `testanyware` command,
the TestAnyware application and its configuration. Their installers own them,
and they stay unchanged. The baseline gives the skill the source ID
`testanyware`.

pack-membership-k171 decides which proofs this move owes, and how Grove's
check treats a plugin that has no methodology manifest. Read its decisions
first.

## Done when

- TestAnyware holds the skill in a pack-shaped source directory, such as
  `plugins/testanyware/` with `.claude-plugin/plugin.json` and
  `skills/using-testanyware/SKILL.md`. The skill's bytes equal Grove's copy,
  or the record names each change. The manifest names the licence of
  TestAnyware and sets no `version`.
- TestAnyware's `Taskfile.yml` has a task that checks the plugin with the
  command `claude plugin validate`. The task skips with a message when Claude Code is
  absent. TestAnyware's documented checks that the change touches pass.
- The TestAnyware commit is on its public `main`. It is pushed through jj
  after its `CLAUDE.md` and release notes are read. The paused workspace
  `gh-issue-14` is not changed.
- Grove's marketplace lists `testanyware` with a `git-subdir` source that
  pins that commit. `plugins/testanyware` is removed from Grove. Each Grove
  reference to it is reconciled: the installer test, the trial cases, the
  plugin README, `plugins/CONTEXT.md`, the architecture document and the
  README.
- `task check:packs` handles the new entry by the decision of
  pack-membership-k171. A plugin without methodology rules must not fail in
  silence and must not pass without a report.
- `ravel skills resolve --json` resolves a plain Git checkout of the
  published commit with the source directory and its roots. The record names
  the commit, the tree ID of the source directory and the `resourceDigest`.
- `bash scripts/check.sh` passes in Grove.
- `docs/repository-moves-record.md` records the source root, the commit, the
  tree ID, the digest, the preserved resources and the tool requirements.

## Notes

Commit the TestAnyware change in that repository's default workspace, on its
`main`. A tool that writes into the source directory must write nothing
there, as the contract's "Layout" requires.

The skill has no supporting file. Its command reference is output of the
installed `testanyware` command. The staging trials of cutover order 5 own
the read of that reference and the real VM tool use. This leaf does not run
them. Its directory trial reads only `SKILL.md`, as pack-membership-k171
states, and it adds no file to the skill.

## Decisions (running log)

- TestAnyware has no `CLAUDE.md` or `AGENTS.md`. Its release notes are its
  GitHub Releases, and v2.7.0 holds only its title. Its README names no agent
  skill. None of them constrains this change.
- The source directory is `plugins/testanyware`, with the roots `["skills"]`.
  It is the plugin root, as in ResearchSkills. The manifest copies Grove's
  manifest, sets `repository` to TestAnyware, adds `license` `Apache-2.0`
  from TestAnyware's `LICENSE` and sets no `version`.
- The plugin task is `plugin:check`, after TestAnyware's `<area>:<verb>`
  task names. TestAnyware has no aggregate `task check`. Base proof 1 of
  pack-membership-k171 therefore runs `task plugin:check` from a clean
  checkout of the published commit.
- The instrument is the Ravel 2.0 build of `builds.json`: `task install`
  from the output commit `71f70fcd3f92` into a scratch prefix. Its
  `bin/ravel` SHA-256 is `26b487afb957…`, the pinned value.
- The build of the static proof is `cargo build -p testanyware-cli` with
  the target directory at the checkout's root `target/`. The release build
  also writes there, but it needs notarisation.
- `check:packs` selects the directory-only branch when the source directory
  holds no `skills/grove-*/` directory and no `conformance/rules.tsv`. A
  kind skill is a `grove-<kind>` skill, as `conformance.sh` states.
- No trial case and no installer test names TestAnyware. The installer finds
  plugins by `plugins/*/skills`, so the removal needs no change there.
- TestAnyware commit `573c61edc63a659c10a84ea1b7981e80e068df0e` is on its
  public `main`, pushed through jj from the default workspace. The paused
  workspace `gh-issue-14` stays at `4bd23490`. The tree of
  `plugins/testanyware` is `0701492a5dae89e5b967e0122a5f1085cccec291`. Its
  `resourceDigest` is `3e9d7929eddd…`, from the working copy and from a plain
  Git checkout. `SKILL.md` equals Grove's copy byte for byte.
- The static proof holds. A `cargo build` wrote 2.7 GB into the plain
  checkout's root `target/`, and the digest stayed `3e9d7929eddd…`. A stray
  file in the source directory changed it, and its removal restored it.
- `check:packs` reports `testanyware` by its directory-only check, and
  research by conformance alone and in the aggregate. Four new controls in
  `scripts/check-packs.test.sh` cover the branch. Each new control failed
  under a mutation of the branch or of the kind test.
- The trial case is `testanyware-discovery`. The runner reads cases from its
  own tree, and this change leaves `plugins/grove` unchanged. The trial
  therefore prepares the parent commit `1bcefecf596`, and the leaf keeps one
  focused commit.
- The published skill directory has the baseline digest `8ddb33fe7a999129`
  over one file, the value of `docs/owner-skill-inventory.tsv`. The move
  needs no recorded change.
- The two inventory files stay unchanged. They record dated observations, and
  no test reads them as the current tree. `moves-handoff-k187` reconciles
  the baseline.
- Both directory trials pass. Claude Code read `using-testanyware/SKILL.md`
  from the testanyware snapshot after a path listing, and Codex read it after
  a listing of names and descriptions. Each read came before the first
  action, and no VM started. `docs/repository-moves-record.md` holds both
  runs in its section "TestAnyware skill".
- The reference search outside this workspace found only TestAnyware's new
  files, Grove's default workspace and two paused Grove workspaces. The
  paused groves reconcile the move at their rebase, so the rollout text needs
  no entry for this move.
- No in-session reviewer was spent. Executable controls with seen failures cover the new `check:packs` branch, and the transcripts are the trial evidence.
- `bash scripts/check.sh` passed: all 21 principal checks. The 17 changed files had the same digests before and after the run.

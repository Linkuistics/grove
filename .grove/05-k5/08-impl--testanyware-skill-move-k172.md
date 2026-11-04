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

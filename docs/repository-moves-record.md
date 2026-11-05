# Repository moves record

This record holds the results of the moves under `repository-moves-k5`. Each
move adds one section. A section names the source and the destination, the
version-control arrangement, the readings before and after, the checks and the
references. `moves-handoff-k187` reads this record, and `discovery-rollout-k6`
uses it to integrate and release the products.

The readings are those of the Done when of `writing-projects-k165`. A script
prints them with the repository root replaced by `<ROOT>`. A comparison then
ignores the change of root and of column padding.

## Stylepack

`stylepack-relocation-k166` moved `~/Development/AgenticWriting/Stylepack` to
`~/Development/AgenticOrchestration/Stylepack` on 2026-11-04. The move changed
no file in the project, so it has no owning-repository commit.

### Arrangement

- Stylepack is a colocated Git and Jujutsu repository.
- It has one jj workspace, `default`. Its working copy is empty, on `main`.
- `main` and `origin/main` are both at `377daaa28e55`. This published commit
  is the one that a pin can name.
- The remote `origin` is `https://github.com/Linkuistics/Stylepack.git`. It
  also carries the bookmark `grove-meta@origin` at `d66019ba08fa`.
- `.jj/repo/store/git_target` holds the relative path `../../../.git`.
- `.git/config` sets no `core.worktree`.
- A search of `.jj/` and `.git/` outside the object stores found no absolute
  path of the root.

### Stale worktree registration

Git registers a second worktree at `~/Development/Stylepack/.grove-meta`.
`git worktree list` marks it prunable, because that directory does not exist.
Its `gitdir` file names `~/Development/Stylepack`, an earlier location of the
project. The move did not touch this registration.

The registration stays unchanged. No human stated that it is obsolete, and the
leaf permits a prune only after that statement. When the human makes the
statement, run `git worktree prune` in the project.

### Readings

The readings before and after the move are identical.

- `jj log -r 'all()'` shows the same 118 visible commits.
- `jj st`, `jj workspace list`, `git worktree list` and `git remote -v` give
  the same output.
- `git status --porcelain --ignored` gives the same ignored entries and no
  other entries.
- The 252 tracked files have the same digest,
  `1866e2577250c7c9ba430caf3cd93d1ceac75cc15039c08029bbbf03ee2d069a`.

Before the move, `lsof +D` found no open file under the project. No process
other than this session's launcher named the project.

### Checks

The moved `.venv` was already broken before the move. Its console scripts
named `~/Development/Stylepack/.venv/bin/python`, from the earlier location.
At the old path, `uv run --frozen pytest` stopped with 42 collection errors.
Each error was a `ModuleNotFoundError`.

At the new path, `uv sync --frozen` alone kept the old script headers.
`uv sync --frozen --reinstall` rewrote them. Then `uv run --frozen pytest`
gave 1366 passed and 4 skipped, with exit status 0. The `--frozen` flag kept
the tracked `uv.lock` unchanged.

The test run added four ignored `__pycache__` directories. They are
regenerated tool state, not product content. The tracked-file digest did not
change.

### References

The search for the parent name `AgenticWriting` covered the following
locations, without ignore rules.

- `~/.claude.json`
- `~/.claude/` and `~/.codex/`
- `~/.config/mise`
- Every repository under `~/Development`, which includes the project

The search found the name once in this grove's root brief, which is the
positive control. A mutated pattern found nothing there.

No hit is a reference to the old path of Stylepack that needs a repair. The
hits are of four classes.

- Claude Code and Codex session transcripts and tool output are history.
- This grove's task notes describe the moves.
- `ResearchSkills/.cache/grove/` holds a cached copy of this grove's tree.
- `rhai-book-use-k74` in three Ravel workspaces names the old path of
  TestSubject, not of Stylepack.

The rollout text for paused groves needs no entry for Stylepack. The Obsidian
manual `AI Dev Tooling` does not name Stylepack or `AgenticWriting`, so it needs
no change.

## TestSubject

`testsubject-relocation-k167` moved `~/Development/AgenticWriting/TestSubject`
to `~/Development/AgenticOrchestration/TestSubject` on 2026-11-04. The move
changed no file in the project, so it has no owning-repository commit.

### Arrangement

- TestSubject is a colocated Git and Jujutsu repository.
- It has one jj workspace, `default`. Its working copy is empty, on `main`.
- `main` and `main@origin` are both at `537ed10016d3`. This published commit
  is the one that a pin can name.
- The remote `origin` is `https://github.com/Linkuistics/TestSubject.git`.
- Git registers one worktree, the project root.
- `.jj/repo/store/git_target` holds the relative path `../../../.git`.
- `.git/config` sets no `core.worktree`.
- A search of `.jj/` and `.git/` outside the object stores found no absolute
  path of the root.

### Readings

The readings before and after the move are identical.

- `jj log -r 'all()'` shows the same 6 commits, including the root commit.
- `jj st`, `jj workspace list`, `git worktree list` and `git remote -v` give
  the same output.
- `git status --porcelain --ignored` gives only the ignored entry `.jj/`.
- The 5 tracked files have the same digest,
  `55eaa5497100cce9b18f0ca0514153fa87a2b80d8398ddb10cc1157111a7ea52`.

Before the move, `lsof +D` found no open file under the project. No process
other than this session's launcher named the project.

### Checks

TestSubject has no checks. Its README marks it as pre-development, and it has
no Taskfile, manifest or test.

`website/` holds only `index.md` and `meta.yml`, which are content and
metadata. The README and the TODO document no website build, so no build ran.

### References

The search for the parent name `AgenticWriting` used the locations and the
controls of the Stylepack search. The positive control found this grove's root
brief, and a mutated pattern found nothing there.

The full old path occurs in the following places. Each place stays unchanged.

- Claude Code and Codex session transcripts and the Codex thread history are
  history.
- This grove's task notes describe the move.
- `.grove/19-requirements--rhai-book-use-k74.md` is a live requirements leaf
  of Ravel's task tree. Each of the workspaces `Ravel.gh-issue-2`,
  `Ravel.skill-directory-delivery` and `Ravel.coordinated-release-design`
  tracks a copy. Its sentence "The local TestSubject repository is
  `/Users/antony/Development/AgenticWriting/TestSubject`" names the old path.

Other files name `AgenticWriting` and TestSubject separately. They are this
grove's task notes, this record, the Codex prompt history and the cached copy
of this grove's tree in `ResearchSkills/.cache/grove/`. None is a reference
that needs a repair.

The rollout text for paused groves needs one entry. The Ravel grove that owns
`rhai-book-use-k74` must read TestSubject at
`~/Development/AgenticOrchestration/TestSubject`.

The Obsidian manual `AI Dev Tooling` does not name TestSubject or
`AgenticWriting`, so it needs no change.

## TheGreatExplainer

`thegreatexplainer-relocation-k168` moved
`~/Development/AgenticWriting/TheGreatExplainer` to
`~/Development/AgenticOrchestration/TheGreatExplainer` on 2026-11-04. The move
changed no file in the project, so it has no owning-repository commit.

### Arrangement

- TheGreatExplainer is a colocated Git and Jujutsu repository.
- It has one jj workspace, `default`. Its working copy is empty, on `main`.
- `main` and `main@origin` are both at `f52faef1e30b`. This published commit
  is the one that a pin can name.
- The remote `origin` is `https://github.com/Linkuistics/TheGreatExplainer.git`.
- Git registers one worktree, the project root.
- `.jj/repo/store/git_target` holds the relative path `../../../.git`.
- `.git/config` sets no `core.worktree`.
- A search of `.jj/` and `.git/` outside the object stores found no absolute
  path of the root. The search included binary files.

### Readings

The readings before and after the move are identical.

- `jj log -r 'all()'` shows the same 6 commits, including the root commit.
- `jj st`, `jj workspace list`, `git worktree list` and `git remote -v` give
  the same output.
- `git status --porcelain --ignored` gives only the ignored entry `.jj/`.
- The 7 tracked files have the same digest,
  `51d3bdf2a3057fd6a32b89101f0ee4b621ea3b7b099a994a1e95bc7bdfc7a0ed`.

Before the move, `lsof +D` found no open file under the project, other than
the search commands of this session. No process other than this session's
launcher named the project.

### Checks

TheGreatExplainer has no checks. Its website marks it as in the requirements
phase, with no implementation code. It has no Taskfile, manifest or test.

`website/` holds `index.md`, `meta.yml` and `docs/requirements.md`, which are
content and metadata. The README and the TODO document no website build, so no
build ran.

### References

The search for the parent name `AgenticWriting` used the locations and the
controls of the Stylepack search. The positive control found this grove's root
brief, and a mutated pattern found nothing there.

The full old path occurs only in two places. Each place stays unchanged.

- Claude Code session transcripts are history.
- This grove's task note for the move describes the move.

Other files name `AgenticWriting` and TheGreatExplainer separately. They are
this grove's task notes, the Claude Code file history, the Codex prompt
history and the cached copy of this grove's tree in
`ResearchSkills/.cache/grove/`. None is a reference that needs a repair.

The Ravel task leaf `rhai-book-use-k74` names `AgenticWriting` only in the
old path of TestSubject. It does not name TheGreatExplainer. The rollout text
for paused groves needs no entry for TheGreatExplainer.

The Obsidian manual `AI Dev Tooling` does not name TheGreatExplainer or
`AgenticWriting`, so it needs no change.

## Writegood

`writegood-relocation-k169` moved `~/Development/AgenticWriting/Writegood` to
`~/Development/AgenticOrchestration/Writegood` on 2026-11-04. The move changed
no file in the project, so it has no owning-repository commit.

### Arrangement

- Writegood is a colocated Git and Jujutsu repository.
- It has one jj workspace, `default`. Its working copy is empty, on
  `9bf13a777499`. Git `HEAD` is detached at the same commit.
- Writegood has no remote and no bookmark, so the local directory is its only
  copy. No published commit exists for a pin to name.
- Git registers one worktree, the project root.
- `.jj/repo/store/git_target` holds the relative path `../../../.git`.
- `.git/config` sets no `core.worktree`.
- A search of the whole project, including `.jj/`, `.git/`, `.venv/` and
  `.writegood/`, found no file that names `AgenticWriting`. The search
  included binary files. A mutated pattern found nothing.

### Readings

The readings before the move, after the move and after the test runs are
identical.

- `jj log -r 'all()'` shows the same visible commits.
- `jj st`, `jj workspace list`, `git worktree list` and `git remote -v` give
  the same output. `git remote -v` gives no output.
- `git status --porcelain --ignored` gives the same ignored entries and no
  other entries.
- The 314 tracked files have the same digest,
  `3e19232898f307732c1027c2c26d7e36eb013ae5aa726d594a1884b63f48abeb`. They
  include the 68 files of Writegood's own `.grove/` tree.
- The untracked and ignored files outside `.git/`, `.jj/`, `.venv/`,
  `.pytest_cache/`, `.ruff_cache/` and `__pycache__/` have the same digest,
  `8dd4cda2423bf97ccc20c102d155bbcda4408b3a838882d7f52245a57fa62e47`. They are
  exactly the 2680 files of the run data in `.writegood/`, which holds 710 MB.

Before the move, `lsof +D` found no open file under the project. No process
other than this session's launcher named the project. A `tar` archive of all
8029 entries of the project stayed in the session scratchpad until the
readings agreed, and the session then removed it.

### Checks

The moved `.venv` was already broken before the move. Its console scripts
named `~/Development/Writegood/.venv/bin/python`, from an earlier location.
At the new path, `uv run --frozen pytest` stopped at `tests/conftest.py` with
`ModuleNotFoundError: No module named 'writegood'`.

`uv sync --frozen --reinstall` rewrote the script headers. Then
`uv run --frozen pytest` gave 687 passed and 1 failed. The `--frozen` flag
kept the tracked `uv.lock` unchanged.

The failing test is `test_safe_mode_on_a_guided_argv_is_refused` in
`tests/test_guided.py`. Its binding does not replace the version probe, so
the test runs the installed `claude --version` and `claude auth status --json`.
The installed Claude Code is `2.1.318`. `docs/measurements/render-measurements.json`
holds token floors only for `2.1.233` and `2.1.234`. The harness therefore
raises `UnmeasuredHarnessVersion` before the check that the test expects.

A control isolated the cause. A stand-in `claude` first on `PATH` reported
`2.1.234` and passed every other call to the real executable. With it, all 688
tests passed, with exit status 0. The failure comes from the installed Claude
Code version, not from the move. The leaf changes no product behaviour, so the
test and the measurement record stay unchanged.

The README states that `uv run pytest` makes no CLI call. This test makes two
calls to the installed `claude`, which spend no tokens. The owner of Writegood
decides whether to replace the probe in the test or to measure the new
version.

### References

The search for the parent name `AgenticWriting` used the locations and the
controls of the Stylepack search. The positive control found this grove's root
brief, and a mutated pattern found nothing there.

The full old path occurs only in two places. Each place stays unchanged.

- Claude Code session transcripts are history.
- This grove's task note for the move describes the move.

Other files name `AgenticWriting` and Writegood separately. They are this
grove's task notes, the Claude Code file history and the Codex history. The
cached copy of this grove's tree in `ResearchSkills/.cache/grove/` also names
both. None is a reference that needs a repair.

Writegood's own task tree cites Stylepack by relative paths such as
`../Stylepack`. Stylepack moved first, so these paths did not resolve between
the two moves. Each of the six cited paths resolves at the new location.

Three entries name `~/Development/Writegood`, an earlier root of the project,
and not the path of this move.

- `~/.claude.json` has a project entry for that path.
- `~/.claude/projects/` has a transcript directory for that path.
- `~/.codex/config.toml` has a project section for that path.

They were stale before this move, and they stay unchanged. Three tracked
files in the project also name that root. Two are recorded session captures
in `tests/fixtures/`, and one is the measurement record
`docs/measurements/render-token-floors.md`. They record past runs.

The Ravel task leaf `rhai-book-use-k74` does not name Writegood. The rollout
text for paused groves needs no entry for Writegood.

The Obsidian manual `AI Dev Tooling` does not name Writegood or
`AgenticWriting`, so it needs no change.

## TestAnyware skill

`testanyware-skill-move-k172` moved the skill `using-testanyware` from Grove's
`plugins/testanyware` into the existing repository
`~/Development/AbstractingComputers/TestAnyware` on 2026-11-04. Grove's
marketplace now pins the published TestAnyware commit, and Grove holds no copy
of the skill.

### Source

| Subject | Value |
| --- | --- |
| Repository | `https://github.com/Linkuistics/TestAnyware.git`, public |
| Commit | `573c61edc63a659c10a84ea1b7981e80e068df0e`, the public `main`. Its parent `a0d88be9a90a` was the earlier `main` |
| Source directory | `plugins/testanyware`. It is the Claude Code plugin root and the Ravel source directory |
| Ravel roots | `["skills"]` |
| Git tree of the source directory | `0701492a5dae89e5b967e0122a5f1085cccec291` |
| `resourceDigest` | `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6`, from the jj working copy and from a plain Git checkout |
| Tracked files of the source directory | `.claude-plugin/plugin.json` and `skills/using-testanyware/SKILL.md` |
| Baseline | `SKILL.md` equals Grove's copy byte for byte. The skill directory has the digest `8ddb33fe7a999129` over one file, the value of `docs/owner-skill-inventory.tsv`. The move made no change to the skill |

The manifest copies Grove's manifest. It sets `repository` to TestAnyware and
adds `license` `Apache-2.0`, the licence of TestAnyware's `LICENSE`. It sets no
`version`. No Git attribute applies to the source directory.

The source directory must not be the repository root. The root holds
`target/`, where TestAnyware's release build writes its candidates.

### Arrangement

- TestAnyware is a colocated Git and Jujutsu repository. Its remote `origin`
  is the repository above.
- This move made the commit in the jj workspace `default`, on `main`, and
  pushed it through jj.
- The second jj workspace `gh-issue-14` at `../TestAnyware.gh-issue-14` holds
  paused work of another grove. It stayed at `4bd23490` and was not changed.
- TestAnyware has no `CLAUDE.md` and no `AGENTS.md`. Its release notes are its
  GitHub Releases, and v2.7.0 holds only its title. Neither constrains the
  change.

### Checks in TestAnyware

The new task `plugin:check` runs `claude plugin validate` on the source
directory. It skips with a message when Claude Code is not installed.
TestAnyware has no aggregate `task check`, so this task is the check that the
change touches.

- From a clean plain Git checkout of the published commit, the task passed
  with two warnings, no `version` and no icon. ResearchSkills has the same two
  warnings. The checkout was clean after the run.
- With `claude` removed from the PATH, the task printed its skip message and
  exited 0.
- For a copy of the plugin whose `name` was a number,
  `claude plugin validate` exited 1.

The README section "Agent skill" states the source directory, its root and
both installation routes.

### Static proof

The boundary is "a source that is not a Linkuistics pack", as
`docs/specs/pack-membership.md` states.

- The source directory holds the skill and each resource that it names. An
  enumeration of every path-like token in `SKILL.md` found `out.png`,
  `out.mp4` and `start.json`. Each is a file that a command of the skill
  writes, not a resource. The skill sends the agent to the installed command
  for its reference, such as `testanyware llm-instructions`. The same
  enumeration found `references/` files in the Grove spine, the positive
  control.
- `ravel skills resolve --json` resolved a plain Git checkout of the published
  commit with the digest `3e9d7929eddd…`. Then `cargo build -p
  testanyware-cli` wrote 2.7 GB into that checkout's root `target/`. Git
  ignores `target/`. The digest after the build was the same.
- In the mutation control, a stray file in the source directory changed the
  digest to `3eab0c847d5a…`. Its removal restored `3e9d7929eddd…`.

The instrument was the Ravel 2.0 build of `scripts/harness-trials/builds.json`:
`task install` from the output commit `71f70fcd3f92` of the line
`skill-directory-delivery` into a scratch prefix. Its `bin/ravel` SHA-256 is
`26b487afb957…`, the pinned value. Each resolution ran under a temporary
`HOME` whose settings held one `local` source.

### Directory trials

The case `testanyware-discovery` gives a Grove `impl` leaf that asks for a Bash
script. The script must start an isolated macOS virtual machine, open TextEdit
in it and save a screenshot. The task names no skill and no tool, and it forbids
a run of the script. The sources are `grove` from the prepared Grove checkout
and `testanyware` from a plain Git checkout of the published commit.

| Subject | Value |
| --- | --- |
| Runner | `scripts/harness_trial.py` in this grove's working copy, with the new case |
| Grove build | commit `1bcefecf596b`, which has the same `plugins/grove` as this move. `grove` SHA-256 `a7f5c988a18e…`, `grove-llm` SHA-256 `2f7895cc33f7…` |
| Ravel build | the build of the static proof |
| Source `grove` | `resourceDigest` `42e07eac1c46…` |
| Source `testanyware` | tree `0701492a5dae…`, `resourceDigest` `3e9d7929eddd…` |
| Mandatory file | `grove` `skills/grove/SKILL.md` |
| Claude Code | 2.1.318, model `claude-opus-5-5`, effort `medium`, permission mode `bypassPermissions` |
| Codex | codex-cli 0.166.0, model `gpt-6.1-sol`, effort `medium`, sandbox `workspace-write`, approval `never` |

Each case ran once in each harness, with no retry. The sandbox home held no
native copy of any skill. Each offline resolution exited 0 with the status
`resolved` and gave the descriptor identity of the session of its run. Both network
controls differed. After the leaf, the driver refused `finish` as
`config_refused`, by the design of the trial configuration.

**Claude Code.** The run is `testanyware-discovery-20261104T115509Z`. The
session file `6658d506-2860-4f33-b9b3-2117d149e8c6.jsonl` has the SHA-256
`18f56fc9c020…`. The descriptor identity is `158f19a487ca…`.

| # | Class | Skill | What the call did |
| --- | --- | --- | --- |
| 1 | mandatory | grove | Read the spine and listed each `SKILL.md` path of both snapshots |
| 3 | kind-skill | grove-impl | Read the kind skill |
| 9 | listing | using-testanyware | Listed the skill directory and read its `SKILL.md` from the testanyware snapshot |
| 11, 12 | command | | Ran `testanyware --version`, `llm-instructions` and three `--help` texts |
| 13 | action | | Wrote `gui-screenshot.sh` |
| 19 | ending | | Ran the exact continue command |

**Result: pass.** The read of `SKILL.md` came before the first action. The
classifier gives call 9 the class `listing`, because the call starts with
`find`. Its output holds the whole skill body. The listing in call 1 gave paths
only, so this agent chose by the directory name.

**Codex.** The run is `testanyware-discovery-20261104T115847Z`. The session
file `rollout-2026-11-04T22-58-51-01a1a66d-1e4a-7083-9eda-be1d2511887d.jsonl`
has the SHA-256 `3faa3933df38…`. The descriptor identity is `7c5a9b8e2782…`.

| # | Class | Skill | What the call did |
| --- | --- | --- | --- |
| 2 | mandatory | grove | Read the spine |
| 4 | kind-skill | grove-impl | Read the kind skill |
| 10 | command | | Printed the name and description of each skill of both snapshots |
| 11 | skill | using-testanyware | Read `SKILL.md` from the testanyware snapshot |
| 13, 15 to 17 | command | | Ran `testanyware llm-instructions`, two `--help` texts and `schema vm start` |
| 18 | action | | Wrote the script and the decision log |
| 24 | ending | | Ran the exact continue command |

**Result: pass.** The read of `SKILL.md` came before the first action. Call
10 shows the descriptions, so this agent chose by description.

No VM started in either run. Each `testanyware` command was `--version`,
`llm-instructions`, `schema` or `--help`, and no VM process ran after the
trials. The staging trials own the real VM tool use.

### Changes in Grove

- `.claude-plugin/marketplace.json` lists `testanyware` with a `git-subdir`
  source: the TestAnyware URL, the path `plugins/testanyware` and the `sha`
  of the published commit.
- This move removed `plugins/testanyware`.
- `scripts/check-packs.sh` checks only that the pinned commit holds the source
  directory when the source has no `skills/grove-<kind>/` directory and no
  `conformance/rules.tsv`. It reports that check by the source's name. A
  commit without the entry's `path` fails. Four controls in
  `scripts/check-packs.test.sh` cover the branch. Each new control failed
  under a mutation of the branch or of the kind test.
- `task check:packs` then printed `1 pinned pack(s) pass alone and in the
  aggregate` for research, and `1 pinned source(s) pass the directory-only
  check, with no conformance: testanyware`.
- The README, `plugins/README.md`, `plugins/CONTEXT.md`, `CONTEXT.md`,
  `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`, `docs/RELEASING.md`, a comment of
  `plugins/install.sh` and the source table of
  `docs/specs/owner-skill-delivery.md` name the new location.
- `plugins/install.sh` needs no change, because it finds bundled plugins by
  `plugins/*/skills`. In an isolated home, `install.sh --force --pack` on the
  plain checkout's `plugins/testanyware` linked `using-testanyware` for Codex,
  Gemini CLI and Pi. A run without `--pack` linked nothing for it. `--force`
  was necessary because this grove's tree is a secondary jj workspace.
- No trial case names `plugins/testanyware`, so no case changes.

### Tool requirements and preserved resources

The skill needs the `testanyware` command and the TestAnyware application.
The baseline has Homebrew `testanyware` 2.7.0 and `/Applications/TestAnyware.app`
2.7.0, with the configuration `~/.config/testanyware/config.rhai`. Their
installers own them, and this move did not change them. The skill reaches
them by ordinary command execution.

The move left these owner resources unchanged:

- The Claude Code plugin `testanyware@linkuistics` and its cache. The owner's
  marketplace follows Grove's `main`, which keeps the old entry until finish.
- The Codex links `~/.codex/skills/using-testanyware` and
  `~/.agents/skills/using-testanyware`. They point into the snapshot
  `~/.agents/.grove/current`, which still holds the skill.
- The Pi link `~/.pi/agent/skills/using-testanyware`. It already dangled to
  `~/Development/grove/plugins/testanyware` before the move.

### References

The search for `plugins/testanyware` covered `~/Development`, `~/.config`,
`~/.claude.json`, `~/.claude/settings.json` and `~/.codex/config.toml`. It
skipped `target/`, `.git/`, `.jj/`, `node_modules/`, `.cache/` and session
transcripts. It found this grove's README, the positive control.

The hits outside this grove's working copy are of three classes.

- TestAnyware's README and Taskfile name the new source directory.
- Grove's default workspace `grove` holds the earlier line. Finish integrates
  this move into it.
- The paused groves `grove.gh-issue-14-documentation-skills-for-editorial-pipeline`
  and `grove.improve-initial-design-decomposition-skill` hold earlier Grove
  lines. Their rebase meets this move. This grove does not edit them.

No machine setting names the path. The rollout text for paused groves needs
no entry, because each paused Grove line reconciles the move at its rebase.

## RequirementsSkills

`requirements-pack-repository-k191` created the repository
`~/Development/AgenticOrchestration/RequirementsSkills` on 2026-11-04, with
the three requirements kinds. This is the expand step of
`requirements-pack-k173`. At the first pack commit, Grove still shipped its
own copies, so the aggregate failed on duplicates by design.
`requirements-removal-k192` removed Grove's copies on 2026-11-05 and published
the second pack commit. [Removal](#removal) records that contraction step.

### Source

| Subject | Value |
| --- | --- |
| Repository | `https://github.com/Linkuistics/RequirementsSkills.git`. See [Publication](#publication) |
| First pack commit | `9f7278c2d46b3e7443355633afb8a745b37af7ac`, the jj bookmark `main` |
| Source directory | `plugins/requirements`. It is the Claude Code plugin root and the Ravel source directory |
| Ravel roots | `["skills"]`. The pack has no `craft/` directory |
| Git tree of the source directory | `f3cf8128fd206c48d73b061cc5ef6b536d4f9d61` |
| `resourceDigest` | `07b2d25e8bd20259c71f7419092329368d8ae48bec6ca973343a601a8ec515c8`, from a plain Git checkout of the commit and from the jj working copy |
| Recorded Grove commit | `6cdab3a9883342e68fe51b3913d8c72ddb26e3bf`, the `GROVE_COMMIT` of the Taskfile. It still ships the moving files |
| Grove remote bookmark | `gh-issue-25`. jj moved it forward from `2328c7f0d45a` to `6cdab3a98833` and pushed it before the pack's publication |

The tracked files are `.gitignore`, `LICENSE`, `README.md`, `Taskfile.yml`
and seven files under `plugins/requirements`:

- `.claude-plugin/plugin.json`
- `conformance/rules.tsv`
- `skills/grove-requirements/SKILL.md`
- `skills/grove-requirements/grilling.md`
- `skills/grove-requirements/LICENSES/mattpocock-skills.LICENSE`
- `skills/grove-review-requirements/SKILL.md`
- `skills/grove-integrate-review-requirements/SKILL.md`

The repository has no Git attribute and no symlink. The checks write only
`.cache/`, which Git ignores. `LICENSE` and `.gitignore` equal those of
ResearchSkills, and `LICENSE` equals Grove's. The Taskfile is the Taskfile of
ResearchSkills with three changes: the pack's name in the first comment,
`GROVE_COMMIT` and `PACK`. `GROVE_REPOSITORY` stays the public URL.

The manifest sets `name` `requirements`, `license` `Apache-2.0` and
`dependencies` `["grove"]`, with no `version`. Its other fields follow the
research manifest.

### Byte changes

Each change below is against Grove's files at the recorded Grove commit. The
digest is that of `docs/owner-skill-inventory.tsv`: the first 16 hexadecimal
digits of the skill tree's SHA-256. A copy of the inventory's `digest()`
function gave each Grove value below, which equals the inventory's value.

| Skill | Baseline digest and files | Pack digest and files | Change |
| --- | --- | --- | --- |
| `grove-requirements` | `dc7d3f8bdfd050c5`, 2 | `9907106d2573fc3a`, 3 | The opening paragraph takes the bare-name text of core's `grove-planning`, with the kind's name changed. The new file `LICENSES/mattpocock-skills.LICENSE` equals the spine's copy byte for byte. `grilling.md` is unchanged |
| `grove-review-requirements` | `123cd36182a0a7e7`, 1 | `a06d5f1fffb49249`, 1 | The opening paragraph equals that of core's `grove-review-planning`. It keeps the read of `references/review.md` after the load sentence |
| `grove-integrate-review-requirements` | `637ab23f91e1d9f0`, 1 | `cb3949126191a4cd`, 1 | The opening paragraph equals that of core's `grove-integrate-review-planning`. It keeps the read of `references/integrate-review.md` after the load sentence |

A `diff -r` against Grove's three directories showed only those openings and
the new licence directory. The three Grove copies under the owner's Claude Code
cache have the baseline digests.

`grove-requirements` and `grilling.md` each point to
`LICENSES/mattpocock-skills.LICENSE` inside an HTML comment. Before the move,
the pointer resolved only in the spine. Inside the skill, it now resolves on
every route without the spine. No other moving skill adapts or bundles a third
party's passage.

### Rules

`plugins/requirements/conformance/rules.tsv` holds the ten rows that the
moving files own. Each row equals a row of Grove's manifest byte for byte, in
Grove's order. Three rows have the owner `grove-requirements/grilling.md` and
the load `on(the grilling threshold is met) @ grove-requirements/SKILL.md`.
Seven rows have the owner `grove-requirements/SKILL.md` and the load
`static({requirements})`. The family members own no row.

The selection was the owner column of Grove's manifest. A search of each row
for `requirements` found the same ten rows. The row comparison reported a
mutated row as missing, which was its control. Grove still holds the ten rows,
and `requirements-removal-k192` removes them.

### Checks

The checks ran in a plain Git checkout of the first pack commit, fetched from
the public remote. `GROVE_REPOSITORY` was the public URL, with no override.
`grove:checkout` fetched the recorded Grove commit from Grove's public remote.
A checkout from the local repository gave the same results before the
publication. The checkout was clean
after the run, apart from the ignored `.cache/`.

- `check:plugin` passed with two warnings, no `version` and no icon. Claude
  Code was 2.1.318.
- The pack alone passed with exit status 0. It checked 1 single-source row and
  10 loaded paths, with 9 rows that have no pinned wording.
- The aggregate failed on 23 assertions, as the order of the move expects.

The pack alone listed 3 external edges and 6 external resources:

| Referring file | External edge or resource |
| --- | --- |
| `grove-requirements/SKILL.md` | Loads `grove`. Names `SPEC-FORMAT.md` |
| `grove-requirements/grilling.md` | Names `CONTEXT-FORMAT.md`, `ADR-FORMAT.md` and `SPEC-FORMAT.md` |
| `grove-review-requirements/SKILL.md` | Loads `grove`. Names `references/review.md` |
| `grove-integrate-review-requirements/SKILL.md` | Loads `grove`. Names `references/integrate-review.md` |

Each external resource resolves in `grove`, as rule 3 of the contract's
"References between skills" states.

The 23 aggregate failures have three classes:

- 3 duplicate skills, one for each moving skill.
- 10 duplicate rows, one for each moved row.
- 10 unshipped owners, one for each moved row. The runner maps a skill name
  that two packs ship to the first pack, which is core. A pack row whose owner
  is in a duplicate skill therefore reads as unshipped.

Each failure names a moving skill or a moved row. The duplicates cause all 23.
Core's figures in the aggregate are 116 single-source rows, 2424 loaded paths
and 64 unpinned rows.

### Ravel resolution

`ravel skills resolve --json` resolved the plain checkout from the public
remote with the status `resolved`. The source record has the type `local` and the digest
`07b2d25e8bd2…`. A second resolution of the jj working copy gave the same
digest.

The instrument was the Ravel 2.0 build of `scripts/harness-trials/builds.json`.
Its `bin/ravel` SHA-256 is `26b487afb957…`, the pinned value. It came from the
release target of the line `skill-directory-delivery`. Each resolution ran
under a temporary `HOME` whose settings held one `local` source and an empty
mandatory list. The build refuses settings without the `mandatory` member.

### Publication

- The owner created the public GitHub repository
  `Linkuistics/RequirementsSkills` with default settings, as for
  ResearchSkills. The permission classifier of this session refused to create
  a public repository, so the owner ran `gh repo create` at the prompt.
- jj added the remote `origin` and pushed the bookmark `main`. The public
  `main` is `9f7278c2d46b3e7443355633afb8a745b37af7ac`, the default branch.
- The recorded Grove commit `6cdab3a98833` is the head of Grove's remote
  bookmark `gh-issue-25`. jj pushed that bookmark before the pack.
- Grove's marketplace does not list the pack yet. `requirements-removal-k192`
  adds the entry with the removal, and it pins this first pack commit.

### Removal

`requirements-removal-k192` removed Grove's copies in one change with every
Grove consumer. This is the contraction step of the move.

| Subject | Value |
| --- | --- |
| Removal commit | `302f85429e4b8f8ada02be594687155109dfbf6b` |
| Pushed Grove commit | `302f85429e4b`. jj moved the remote bookmark `gh-issue-25` forward from `6cdab3a98833` before the second pack commit's publication |
| First pack commit | `9f7278c2d46b3e7443355633afb8a745b37af7ac`. Grove's marketplace pins it |
| Second pack commit | `eb945d9b13efcf11a74c78936c802108d3f022c6`, a child of the first. The public `main` |
| Tree of `plugins/requirements` | `f3cf8128fd206c48d73b061cc5ef6b536d4f9d61` in both pack commits |
| `resourceDigest` | `07b2d25e8bd2…`, the value in [Source](#source). The two commits have one tree ID for the source directory, so they deliver the same bytes |

The removal took out `grove-requirements/` with `SKILL.md` and `grilling.md`,
`grove-review-requirements/` and `grove-integrate-review-requirements/`. Core's
manifest lost the ten rows that the pack's manifest holds. The prefix sets
`review-*` and `integrate-review-*` stay. These consumers changed:

- The kind table of `TASK-FORMAT.md` names `RequirementsSkills` in the
  `requirements` row. The label tests take the three labels from that row.
- `conformance.test.sh`: the `inherited-load` control writes a fixture kind
  with a supporting file, because no core kind ships one.
- `plugin_fallback.rs`: the three requirements openings left
  `MOVING_KIND_OPENINGS`.
- `.claude-plugin/marketplace.json` lists `requirements` with a `git-subdir`
  source that pins the first pack commit.
- The install guidance names the pack for the marketplace and for
  `install.sh --pack`. `docs/USAGE.md` adds the source to the directory
  route. It states that a fresh grove's first leaf needs `grove-requirements`
  from RequirementsSkills.

### Readings of the removal

| Reading | Single-source checks | Loaded-path checks | Unpinned rows |
| --- | --- | --- | --- |
| Core alone before the removal, at `53d092fa0e5e` | 116 | 2424 | 64 |
| Core alone after the removal | 115 | 2062 | 55 |
| The pack alone | 1 | 10 | 9 |
| The aggregate of core and the first pack commit | 116 | 2424 | 64 |

The pack alone lists 3 external edges and 6 external resources, as in
[Checks](#checks). The aggregate passes with the figures of core alone before
the move. The research aggregate is now 115 single-source checks and 2421
loaded-path checks.

In the aggregate, the core rows of `review-*` and `integrate-review-*` bind the
pack's two family members. In a scratch copy of the pinned pack, the two
members stopped loading `grove`. The aggregate then failed the three
`static(review-*)` rows for `review-requirements` and the four
`static(integrate-review-*)` rows for `integrate-review-requirements`. It
failed no other kind. The spine's `SKILL.md` lists its family files, so a
member that loses only its pointer to `references/review.md` still reaches
that file. That mutation read clean and is not a control.

The changed `inherited-load` control read clean on its fixture and dirty after
its mutation, with the diagnostic for the supporting file. Its first dirty
reading came back clean, because the fixture's sentence did not match the
mutation. A repair of the fixture's sentence removed that fault.

A search with `--hidden` over the whole tree found no stale location claim. It
enumerated the three skill names and `grilling.md`. The search found a planted
path in a hidden scratch directory, and a mutated pattern found nothing.

### Checks of the removal

- `bash scripts/check.sh` passed all 21 principal checks on the removal, with
  1345 Rust tests. `GROVE_TEST_RAVEL` resolved to the installed Ravel 1.1.0,
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, SHA-256
  `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. The
  changed files had the same digests before and after the run.
- `task check:packs` passed for `requirements` and `research`, alone and in
  the aggregate. `testanyware` passed the directory-only check.
- The second pack commit's full `task check` passed from a clean plain-Git
  checkout, with the committed public `GROVE_REPOSITORY` and no override.
  `grove:checkout` fetched `302f85429e4b` from GitHub. `check:plugin` passed
  with the two warnings of the first commit. The pack alone gave 1 and 10, and
  the aggregate gave 116 and 2424. The checkout held only the ignored `.cache/`
  after the run.
- After the publication, a clone of the public remote was at `eb945d9b13ef`.
  Its `task check:conformance` gave the same figures.

### Requirements directory trials

`requirements-directory-trials-k193` ran the case `requirements-family-read`
once in Claude Code and once in Codex, on 2026-11-05. The `grove` source came
from the prepared Grove checkout at the removal commit `302f85429e4b`. The
`requirements` source came from a plain Git checkout of the second pack commit
`eb945d9b13ef`. In each session, a `review-requirements` leaf read its kind
skill from the requirements snapshot. It read the spine and
`references/review.md` from the grove snapshot before the first command on the
task.

Both runs pass, and no gap remains open.
[Requirements directory trials](common-loading-acceptance-record.md#requirements-directory-trials)
in the acceptance record holds the builds, the digests and the observations.

### Requirements native namespace

`requirements-native-namespace-k194` ran the case
`requirements-native-namespace` once in Claude Code, on 2026-11-05. An
isolated Claude Code home added the marketplace from a plain Git checkout of
Grove at the removal commit `302f85429e4b`. It installed
`requirements@linkuistics`, which installed `grove` as a dependency. The
installed `requirements` plugin is at the first pack commit `9f7278c2d46b`.
The session loaded `requirements:grove-review-requirements`, then
`grove:grove`. It read `references/review.md` from the spine's base directory
before the first command on the task.

The run passes, and no gap remains open. The owner's real Claude Code
installation had the same digests before and after the run.
[Requirements native namespace](common-loading-acceptance-record.md#requirements-native-namespace)
in the acceptance record holds the builds, the plugin revisions and the
observations.

### For discovery-rollout-k6

This workstream reaches `main` only in finish, so the owner's native
marketplace lists `requirements` only after finish. A native user then needs
`/plugin install requirements@linkuistics`. Without it, a fresh grove's first
leaf has no kind skill on the native route. A directory route needs a
`requirements` source, as `docs/USAGE.md` states.

The same holds for `design`, which the marketplace lists after finish. A native
user then needs `/plugin install design@linkuistics`. Without it, a `design`
leaf has no kind skill on the native route, and the four design craft skills
are absent. Grove's `linkuistics` plugin no longer holds them. A directory
route needs a `design` source with the roots `skills` and `craft`, as
`docs/USAGE.md` states.

The same holds for `engineering`, which the marketplace lists after finish. A
native user then needs `/plugin install engineering@linkuistics`. Without it,
an `impl` or `prototype` leaf has no kind skill on the native route, and the
nine engineering craft skills are absent. Grove's `linkuistics` plugin no
longer holds them. A directory route needs an `engineering` source with the
roots `skills` and `craft`, as `docs/USAGE.md` states.

## DesignSkills

`design-pack-repository-k197` created the repository
`~/Development/AgenticOrchestration/DesignSkills` on 2026-11-05, with the
three design kinds and four design craft skills. This is the expand step of
`design-pack-k175`, and DesignSkills is the first pack with a `craft/` root.
At the first pack commit, Grove still ships its own copies of all seven
skills, so the aggregate fails on duplicates by design.
`design-removal-k198` removed Grove's copies and published the second pack
commit. See [Removal of the design pack](#removal-of-the-design-pack).

`design-craft-trials-k199` ran the discovery trial from the `craft/` root. See
[Design craft trials](#design-craft-trials). `design-native-namespace-k200`
ran the native trial. See [Design native namespace](#design-native-namespace).

### Source of the design pack

| Subject | Value |
| --- | --- |
| Repository | `https://github.com/Linkuistics/DesignSkills.git`. See [Publication of the design pack](#publication-of-the-design-pack) |
| First pack commit | `9d7c7c5368f6a22b541881e6c107da28e39627bd`, the jj bookmark `main` |
| Source directory | `plugins/design`. It is the Claude Code plugin root and the Ravel source directory |
| Ravel roots | `["skills", "craft"]`. `skills/` holds the three kinds, and `craft/` holds the four craft skills |
| Plugin manifest | `"skills": ["./craft/"]`, so Claude Code scans `skills/` by default and adds `craft/` |
| Git tree of the source directory | `dd04e268aeb5eba82c527755b1f52bee3565a216` |
| `resourceDigest` | `d8ad3a2af1b5029bc0de93747f4d7297038651180f6877fbd738d94fcbf6aa14`, from a plain Git checkout of the commit and from the jj working copy |
| Recorded Grove commit | `c800d311a541b02237f06b5a9b5ddba4c2993765`, the `GROVE_COMMIT` of the Taskfile. It still ships the moving files |
| Grove remote bookmark | `gh-issue-25`. jj moved it forward from `302f85429e4b` to `c800d311a541` and pushed it before the pack's publication |

The tracked files are `.gitignore`, `LICENSE`, `README.md`, `Taskfile.yml`
and eighteen files under `plugins/design`:

- `.claude-plugin/plugin.json`
- `PROVENANCE.md`
- `conformance/rules.tsv`
- `skills/grove-design/SKILL.md`
- `skills/grove-design/assets/diagram-viewer.html`
- `skills/grove-design/references/visual-design-discussion.md`
- `skills/grove-review-design/SKILL.md`
- `skills/grove-integrate-review-design/SKILL.md`
- `craft/decision-records/SKILL.md`
- `craft/decision-records/LICENSES/mattpocock-skills.LICENSE`
- `craft/codebase-design/SKILL.md`
- `craft/codebase-design/references/design-examples.md`
- `craft/cli-tool-design/SKILL.md`
- `craft/cli-tool-design/references/auditing-and-refactoring.md`
- `craft/model-led-development/SKILL.md`
- `craft/model-led-development/references/instruments.md`
- `craft/model-led-development/references/keeping-a-log.md`
- `craft/model-led-development/references/routing-table.md`

The repository has no Git attribute and no symlink. The checks write only
`.cache/`, which Git ignores. `LICENSE` and `.gitignore` equal those of
RequirementsSkills, and `LICENSE` equals Grove's. The Taskfile is the Taskfile
of RequirementsSkills with three changes: the pack's name in the first
comment, `GROVE_COMMIT` and `PACK`. `GROVE_REPOSITORY` stays the public URL.

The manifest sets `name` `design`, `license` `Apache-2.0`, `dependencies`
`["grove"]` and `skills` `["./craft/"]`, with no `version`. Its other fields
follow the requirements manifest.

### Byte changes of the design pack

Each change below is against Grove's files at the recorded Grove commit. The
digest is that of `docs/owner-skill-inventory.tsv`: the first 16 hexadecimal
digits of the skill tree's SHA-256. A copy of the inventory's `digest()`
function gave each value below. It gave the baseline value for each copy under
the owner's Claude Code cache.

| Skill | Baseline digest and files | Grove digest at the recorded commit | Pack digest and files | Change |
| --- | --- | --- | --- | --- |
| `grove-design` | `beee6cbee6524784`, 3 | `7a610371ecb2c752` | `78dc6c710f2a7954`, 3 | Two changes. `core-bare-references-k170` (`5a6bdd68a7d8`) changed the citation "use `codebase-design` (`linkuistics:codebase-design` on Claude Code)" to "use the `codebase-design` skill" in Grove. The move then gave the opening paragraph the bare-name text of core's `grove-planning`, with the kind's name changed. The two resources are unchanged |
| `grove-review-design` | `0e539af165c861a2`, 1 | The baseline | `444e03aef95b6dc6`, 1 | The opening paragraph equals that of core's `grove-review-planning`. It keeps the read of `references/review.md` after the load sentence |
| `grove-integrate-review-design` | `9bd31d387550dae5`, 1 | The baseline | `0165d18ea4e334cf`, 1 | The opening paragraph equals that of core's `grove-integrate-review-planning`. It keeps the read of `references/integrate-review.md` after the load sentence |
| `decision-records` | `27ef779f1e37cdff`, 1 | The baseline | `f5ad1eafafcea37b`, 2 | The new file `LICENSES/mattpocock-skills.LICENSE` equals the spine's copy byte for byte. `SKILL.md` is unchanged |
| `codebase-design` | `9919698f17a511ec`, 2 | The baseline | `9919698f17a511ec`, 2 | None |
| `cli-tool-design` | `9f847e8ef1af94a5`, 2 | The baseline | `9f847e8ef1af94a5`, 2 | None |
| `model-led-development` | `f4021c1d454d3c12`, 4 | The baseline | `01ab07d3401d88ef`, 4 | Two classes of change in four files. `SKILL.md` and `references/instruments.md` name `doubt-driven-development` by its bare name, not `linkuistics:doubt-driven-development`. Each mention of `docs/formalism-findings.md` and `docs/candidate-lessons.md` becomes a link to `https://github.com/Linkuistics/grove/blob/main/docs/<file>`. The words "in `Linkuistics/grove`" stay |

A `diff -r` against Grove's seven directories showed only those changes. The
citation of `doubt-driven-development` stays a citation, "See the sibling
skill", and is not a directed load. The pack alone lists no edge to it. Grove's
public `main` (`238c3a839ab4`) holds both linked records, and each URL returned
HTTP status 200. The records stay in Grove, by test 1 of "The Grove assets of
moving craft skills" in `docs/specs/pack-membership.md`.

### Provenance of the design pack

`plugins/design/PROVENANCE.md` holds the sections of Grove's
`plugins/linkuistics/PROVENANCE.md` for `codebase-design`, `decision-records`
and `model-led-development`. `cli-tool-design` has no section in either file.
The relative link of each moved craft skill to `PROVENANCE.md` resolves inside
`plugins/design`, from `craft/<skill>/` and from
`craft/codebase-design/references/`. Grove keeps its copy of those sections
until `design-removal-k198`.

The new file has its own title and a paragraph that names its source. The
`codebase-design` section is byte for byte. Two sections change:

- `decision-records` gains a sentence that names
  `decision-records/LICENSES/mattpocock-skills.LICENSE`.
- `model-led-development` said "this repository" for Grove. It now says
  "Grove's repository" and "Grove's own protocols", and it links both records
  by public URL.

Of the moved craft skills, only `decision-records` adapts a passage of a third
party: the template, the examples and the when-to-write test of
`mattpocock/skills` (MIT). It therefore holds the licence text inside its own
skill. `codebase-design` cites works and keeps an inherited attribution for an
exercise that it no longer contains. `cli-tool-design` and
`model-led-development` adapt nothing. None of the three has `LICENSES/`.

The copied sections keep their findings of the technical-communication
checker. They are quoted source text, and the new sentences have no finding.

### Rules of the design pack

`plugins/design/conformance/rules.tsv` holds seven rows. Six rows equal the
rows of Grove's manifest whose owner is a moving file, byte for byte and in
Grove's order. Three have the owner `grove-design/SKILL.md` and three have the
owner `grove-design/references/visual-design-discussion.md`. Each has the load
`static({design})`. The family members and the craft skills own no row in
Grove's manifest.

The seventh row is the pinned row of the citation registry:

| Rule | Owner | Class | Load | Phrase |
| --- | --- | --- | --- | --- |
| `design-cites-codebase-design-with-fallback` | `grove-design/SKILL.md` | `B` | `static({design})` | Grove's artifact requirements remain binding when the skill is absent |

The phrase is the registry's `binds` text in its source case. The runner
removes markup and case and joins line breaks before it compares. Two controls
in a scratch copy each failed on that row alone. The first changed the phrase
in the owner, and the second added the phrase to `grove-review-design`. Grove
still holds the six rows and the registry row, and `design-removal-k198`
removes them.

### Checks of the design pack

The checks ran in a plain Git checkout of the first pack commit, fetched from
the public remote. `GROVE_REPOSITORY` was the public URL, with no override.
`grove:checkout` fetched the recorded Grove commit from Grove's public remote.
The checkout was clean after the run, apart from the ignored `.cache/`.

- `check:plugin` passed with two warnings, no `version` and no icon. Claude
  Code was 2.1.318. The validation reads the manifest and its `craft/` entry.
- The pack alone passed with exit status 0. It checked 5 single-source rows
  and 7 loaded paths, with 2 rows that have no pinned wording.
- The aggregate failed on 16 assertions, as the order of the move expects.
  `task check` therefore exits non-zero at this commit.

Conformance reads only `skills/`. No row has an owner in `craft/`. The pack
alone listed 3 external edges and 2 external resources:

| Referring file | External edge or resource |
| --- | --- |
| `grove-design/SKILL.md` | Loads `grove` |
| `grove-review-design/SKILL.md` | Loads `grove`. Names `references/review.md` |
| `grove-integrate-review-design/SKILL.md` | Loads `grove`. Names `references/integrate-review.md` |

The moved kinds name no format file, so no format file is an external
resource. Each external resource resolves in `grove`, as rule 3 of the
contract's "References between skills" states. `grove-design` names
`references/visual-design-discussion.md`, which resolves inside its own skill.
That file links `../assets/diagram-viewer.html`, which stays inside the skill.

The 16 aggregate failures have three classes:

- 3 duplicate skills, one for each moving kind.
- 6 duplicate rows, one for each moved row of Grove's manifest.
- 7 unshipped owners: one for each of the six moved rows, and one for the
  pinned row. The runner maps a skill name that two packs ship to the first
  pack, which is core. Each pack row whose owner is in `grove-design`
  therefore reads as unshipped.

Each failure names a moving kind or a moved row, and the duplicates cause all
16. Core's figures in the aggregate are 115 single-source rows and 2062 loaded
paths.

### Ravel resolution of the design pack

`ravel skills resolve --json` resolved the plain checkout from the public
remote with the status `resolved`. The source record has the type `local` and
the digest `d8ad3a2af1b5…`. Its two search directories are the `skills` and
`craft` roots of the snapshot. The snapshot holds the `SKILL.md` of each of
the three kinds and each of the four craft skills. `diff -r` found no
difference between the snapshot and `plugins/design` of the checkout. A second
resolution of the jj working copy gave the same digest.

The instrument was the Ravel 2.0 build of `scripts/harness-trials/builds.json`.
Its `bin/ravel` SHA-256 is `26b487afb957…`, the pinned value, and it holds the
build identity `a4f55d04b2d1…` twice. Each resolution ran under a temporary
`HOME` whose settings held one `local` source with the roots
`["skills", "craft"]` and an empty mandatory list.

### Publication of the design pack

- This session ran `gh repo create Linkuistics/DesignSkills --public`. An
  uncommitted `.claude/settings.json` in the Grove workspace allowed
  `Bash(gh repo create *)`, so no prompt came. For RequirementsSkills, the
  owner ran the command at the prompt.
- jj added the remote `origin` and pushed the bookmark `main`. The public
  `main` is `9d7c7c5368f6a22b541881e6c107da28e39627bd`.
- The recorded Grove commit `c800d311a541` is the head of Grove's remote
  bookmark `gh-issue-25`. jj pushed that bookmark before the pack.
- At this publication, Grove's marketplace did not list the pack.
  `design-removal-k198` added the entry with the removal, and it pins this
  first pack commit.

### Removal of the design pack

`design-removal-k198` removed Grove's copies in one change with every Grove
consumer, on 2026-11-05. This is the contraction step of the move.

| Subject | Value |
| --- | --- |
| Removal commit | `84fba18c1f98b694bcf440181c3853a6ec343772` |
| Pushed Grove commit | `84fba18c1f98`. jj moved the remote bookmark `gh-issue-25` forward from `c800d311a541` before the second pack commit's publication |
| First pack commit | `9d7c7c5368f6a22b541881e6c107da28e39627bd`. Grove's marketplace pins it |
| Second pack commit | `a9288f733508b97b6812215086372598b01b50ba`, a child of the first. The public `main` |
| Tree of `plugins/design` | `dd04e268aeb5eba82c527755b1f52bee3565a216` in both pack commits |
| `resourceDigest` | `d8ad3a2af1b5029bc0de93747f4d7297038651180f6877fbd738d94fcbf6aa14` for both commits, from a clone of the public remote at the second commit |

The removal took out 14 files. They are `grove-design/` with `SKILL.md`,
`assets/diagram-viewer.html` and `references/visual-design-discussion.md`,
`grove-review-design/`, `grove-integrate-review-design/`, and the four craft
directories of `plugins/linkuistics/skills/`. Core's manifest lost the six
rows whose owner is in `grove-design/`. The prefix sets `review-*` and
`integrate-review-*` stay. These consumers changed:

- The kind table of `TASK-FORMAT.md` names `DesignSkills` in the `design` row.
  The label tests take the three labels from that row.
- `conformance.test.sh`: the `two-kinds-one-rule` control takes `planning` as
  its second kind, because core keeps `planning` after every move.
- `plugin_fallback.rs`: the registry lost its row for
  `grove-design/SKILL.md`, and it now covers core alone. The pack's manifest
  pins that citation as `design-cites-codebase-design-with-fallback`. The
  three design openings left `MOVING_KIND_OPENINGS`.
- The dependency decision record states that the registry covers core and
  that each pack pins its own citations.
- `plugins/linkuistics` no longer holds the four craft skills. Its manifest
  description and keywords, its marketplace entry and `PROVENANCE.md` name
  only what it still holds. `PROVENANCE.md` names where the three moved
  sections went.
- `.claude-plugin/marketplace.json` lists `design` with a `git-subdir` source
  that pins the first pack commit, and it counts six plugins.
- The install guidance names the pack for the marketplace and for
  `install.sh --pack`. `plugins/README.md` has a `design` section with the
  craft table. `docs/USAGE.md` adds a `design` source with the roots `skills`
  and `craft` to the directory route.
- `docs/formalism-findings.md` linked `keeping-a-log.md` inside the moved
  `model-led-development`. That link is now the DesignSkills public URL.

### Readings of the removal of the design pack

| Reading | Single-source checks | Loaded-path checks | Unpinned rows |
| --- | --- | --- | --- |
| Core alone before the removal, at `bbd90b1d82e4` | 115 | 2062 | 55 |
| Core alone after the removal | 111 | 1704 | 53 |
| The pack alone | 5 | 7 | 2 |
| The aggregate of core and the first pack commit | 116 | 2063 | 55 |

The aggregate before the removal failed on the 16 duplicates of
[Checks of the design pack](#checks-of-the-design-pack). The aggregate after
it has the figures of core alone before the move, plus the pinned citation
row. That row moved from the Rust registry, which the runner does not count,
into the pack's manifest. The pack alone lists 3 external edges and 2 external
resources. The requirements aggregate is now 112 and 2066, and the research
aggregate is 111 and 2063.

In the aggregate, the core rows of `review-*` and `integrate-review-*` bind
the pack's two family members. In a scratch copy of the pinned pack,
`grove-review-design` and `grove-integrate-review-design` stopped loading
`grove`. The aggregate then failed 25 assertions, and each names one of the
two members. They are the three `static(review-*)` rows, the four
`static(integrate-review-*)` rows, the rows of the spine's `SKILL.md`, and the
two family files as missing files.

The changed `two-kinds-one-rule` control read clean and then dirty. A mutant
that planted a sentence without the rule's wording failed that control alone.

The supplier check of `trial-supplier-refusal-k196` refused `explicit-bare` at
the removal commit with status 1. The prepared commit holds no
`plugins/linkuistics/skills/decision-records/SKILL.md`, which the case
declares. At `b63ca828d3ef`, the commit that its record names, the check held
the 8 required paths of the case with status 0.

The stale-location search enumerated the seven skill names and the seven
resource file names with `--hidden` over the whole tree. It then searched for
each name under `linkuistics/skills`, `grove/skills` and `linkuistics:`. Each
hit is in a class of the next section. A planted path in a hidden scratch
directory was found, and a misspelt pattern found nothing.

### Asset classification of the design pack

Each Grove file that names a moved skill is in one class of "The Grove assets
of moving craft skills" in `docs/specs/pack-membership.md`. No asset moved,
because no Grove file states, decides or checks the behaviour of a moved
craft skill.

| Files | Test | Disposition |
| --- | --- | --- |
| `docs/ordinal-fs-tree/CLI.md`, which applies `cli-tool-design` to Grove's own CLI by bare name | 1 | Stays |
| The spine's `ADR-FORMAT.md`, `SPEC-FORMAT.md`, `references/grove.md` and `references/review.md`, `CONTEXT.md`, `CONTEXT-MAP.md`, `simplify-project` and `authoring-conventions` | 1 | Stay. Each cites a moved skill by bare name |
| The formal-methods records and `evidence-outlives-the-instrument.md` | 1 | Stay. The one relative link into a moved file is now a public URL. Their path mentions are dated accounts |
| The acceptance record, the trial cases and the `rival-records` fixture | 3 | Stay as evidence of their runs |
| `docs/owner-skill-inventory.tsv`, `inventory.py` and `docs/ravel-caller-surface-inventory.tsv` | 3 | Stay as the baseline evidence |
| `CHANGELOG.md`, the move specifications, `membership-formats.mmd` and `owner-skill-delivery.md` | History and design | Stay |
| `native_skill_delivery.rs`, which writes `grove-design` under a temporary home | Fixture label | Stays |

### Checks of the removal of the design pack

- `bash scripts/check.sh` passed all 21 principal checks on the removal, with
  1345 Rust tests. `GROVE_TEST_RAVEL` resolved to the installed Ravel 1.1.0,
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, SHA-256
  `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. The
  changed files had the same digests before and after the run.
- `task check:packs` passed `requirements`, `design` and `research`, alone and
  in the aggregate. `testanyware` passed the directory-only check.
- `claude plugin validate plugins/linkuistics` passed with two warnings, no
  `version` and no icon. Claude Code was 2.1.318.
- The second pack commit's full `task check` passed from a clean plain-Git
  checkout, with the committed public `GROVE_REPOSITORY` and no override.
  `grove:checkout` fetched `84fba18c1f98` from GitHub. `check:plugin` passed
  with the two warnings of the first commit. The pack alone gave 5 and 7, and
  the aggregate gave 116 and 2063. The checkout held only the ignored `.cache/`
  after the run.
- After the publication, a clone of the public remote was at `a9288f733508`,
  with the tree `dd04e268aeb5`. `ravel skills resolve --json` resolved it as
  a `local` source with the status `resolved` and the digest above. The
  snapshot's `skills` root holds the three kinds, and its `craft` root holds
  the four craft skills. `diff -r` found no difference between the snapshot and
  `plugins/design`. The instrument was the Ravel 2.0 build of
  `scripts/harness-trials/builds.json`, `bin/ravel` SHA-256 `26b487afb957…`,
  with its build identity twice.
- An isolated home held Codex, Gemini CLI and Pi directories. In it,
  `install.sh --force --pack <public clone>/plugins/design` linked the three
  kinds from `skills/` and the four craft skills from `craft/` into each
  harness. The run made 111 links in all. `--force` was needed because the Grove workspace
  is a secondary jj workspace. The owner's real skill links had the same digest
  before and after.

### Design craft trials

`design-craft-trials-k199` ran the case `design-craft-discovery` once in
Claude Code and once in Codex, on 2026-11-05. The `grove` source came from the
prepared Grove checkout at the removal commit `84fba18c1f98`. The `design`
source came from a plain Git checkout of the second pack commit
`a9288f733508`, with the roots `skills` and `craft`. In each session, an
`impl` leaf asked for a command-line tool and named no skill. The agent read
`craft/cli-tool-design/SKILL.md` from the `design` snapshot before it wrote
the tool.

Both runs pass, and no gap remains open.
[Design craft trials](common-loading-acceptance-record.md#design-craft-trials)
in the acceptance record holds the builds, the digests and the observations.

### Design native namespace

`design-native-namespace-k200` ran the case `design-native-namespace` twice in
Claude Code, on 2026-11-05. An isolated Claude Code home added the
marketplace from a plain Git checkout of Grove at the removal commit
`84fba18c1f98`. It installed `design@linkuistics`, which installed `grove` as
a dependency. The installed `design` plugin is at the first pack commit
`9d7c7c5368f6`, and its `plugins/design` tree equals that of the second pack
commit.

Claude Code listed the four craft skills as `design:<name>` and the three
kinds as `design:grove-<kind>`, with no bare copy. In run 1, the agent loaded
`design:cli-tool-design` before it wrote the tool. In run 2, the agent wrote
the tool without the craft skill. The acceptance record names that miss as a
gap of the native route, with its disposition. The owner's real Claude Code
installation had the same digests before and after the runs.
[Design native namespace](common-loading-acceptance-record.md#design-native-namespace)
in the acceptance record holds the builds, the plugin revisions and the
observations.

## EngineeringSkills

The engineering move takes the six kinds of the impl and prototype chains and
nine craft skills to `~/Development/AgenticOrchestration/EngineeringSkills`.
The kinds are `grove-impl`, `grove-prototype`, `grove-review-impl`,
`grove-integrate-review-impl`, `grove-review-prototype` and
`grove-integrate-review-prototype`. The craft skills are the six
`coding-style-*` skills, `using-codebase-memory`, `simplify-project` and
`doubt-driven-development`. The node is `engineering-pack-k177`.

`engineering-pack-repository-k203` created the repository on 2026-11-05. This
is the expand step. At the first pack commit, Grove still ships its own copies
of all fifteen skills, so the aggregate fails on duplicates by design. See
[Source of the engineering pack](#source-of-the-engineering-pack).

`engineering-removal-k204` removed Grove's copies on 2026-11-05. This is the
contraction step. See
[Removal of the engineering pack](#removal-of-the-engineering-pack).

### Consumer retargeting of the engineering pack

`impl-consumer-retarget-k202` ended the dependence of Grove's controls on the
moving kinds, on 2026-11-05. Grove still ships every moving file after this
step.

- The conformance controls `unreachable` and `kind-without-load` copy the
  core kind `planning`. `two-kinds-one-rule` copies `finish` and `planning`.
  It plants the phrase of `finish-records-teardown-before-exit` in
  `grove-planning/SKILL.md`. That row has a pinned phrase, so the single-source
  sweep covers it.
- The opening of `planning` puts the full stop inside the emphasis. The
  mutation of `kind-without-load` therefore replaces `**Load the `grove`
  skill now.**`.
- The example comments of `conformance.awk`, of the header of
  `conformance/rules.tsv` and of `plugins/grove/README.md` name
  `grove-planning/SKILL.md`.
- The test `a_native_only_lifecycle_completes_with_explicitly_installed_skills`
  runs the real `plugins/install.sh` for Codex. Its leaf is now a `planning`
  leaf, and it requires `grove-planning` from the installer. The scratch run
  below found this named read. The planning list did not hold it.

The control suite gave 98 passed and 0 failed. Three mutants of the test
script each failed only their own control, with 97 passed and 1 failed:

| Control | Mutant | Reading |
| --- | --- | --- |
| `unreachable` | The severed pointer is a file name that `grove/SKILL.md` does not hold | Exit 0, where the control expects 1 |
| `kind-without-load` | The needle is the old one, without the full stop | Exit 0, where the control expects 1 |
| `two-kinds-one-rule` | The planted sentence does not hold the rule's wording | Exit 0, where the control expects 1 |

### Consumer classification of the engineering pack

The search enumerated the fifteen skill names with `--hidden` over the whole
tree, without `target/`, `.jj/` and `.git/`. It found a file of `.grove/`,
and a misspelt name found nothing. A second search looked for Markdown links
into a moving directory. It found the planted link of a hidden scratch file
and the `PROVENANCE.md` links of the craft skills. It found no link from a
staying file into a moving file.

Each test that names a moving skill is in one class of "The Grove tests that
name moving kinds" in `docs/specs/pack-membership.md`.

| Files | Class | Disposition |
| --- | --- | --- |
| `conformance.test.sh` | Named read | Retargeted in this step. No control copies a moving kind |
| The four rows of `conformance/rules.tsv` whose owner is `grove-impl/SKILL.md` or `grove-prototype/SKILL.md` | Named read | `engineering-removal-k204` removes them |
| `MOVING_KIND_OPENINGS` in `plugin_fallback.rs` | Named read | `engineering-removal-k204` drops the six engineering openings |
| The `include_str!` of `doubt-driven-development/SKILL.md` in `composition_guidance.rs` and `session_kind_guidance.rs` | Named read | `engineering-removal-k204` drops it. The assertions over Grove's files stay |
| `a_native_only_lifecycle_completes_with_explicitly_installed_skills` in `native_skill_delivery.rs` | Named read | Retargeted in this step. It runs the real installer, and it now requires and reads `grove-planning` |
| `directory_route.rs`, the other tests of `native_skill_delivery.rs`, `lifecycle_cutover.rs` and `prompt.rs` | Fixture label | Stay. Each writes or matches a synthetic name in a temporary tree, a prompt or a log |
| The fabricated map of `an_unregistered_citation_is_caught` in `plugin_fallback.rs` | Fixture label | Stays |
| `grove-floor-directory-case.sh` and the fixtures of `test_harness_trial.py`, `KIND_SKILL` among them | Fixture label | Stay. Each path is in a temporary checkout that the test writes |
| The `rival-style` trial source and its `README.md` | Fixture label | Stay. The source holds its own synthetic `coding-style-bash` |
| `shipped_kinds` and `shipped_markdown` in `session_kind_guidance.rs`, the corpus of `plugin_fallback.rs` and the check of the kind table | Walk | No change. Each names no moving skill |

Each document that names a moving skill is in one class of "The Grove assets
of moving craft skills" in `docs/specs/pack-membership.md`. No asset moves in
this step.

| Files | Test | Disposition |
| --- | --- | --- |
| `plugins/linkuistics/PROVENANCE.md` | 2 | Splits by skill. `engineering-removal-k204` moves the sections of the nine craft skills |
| `docs/adr/evidence-outlives-the-instrument.md`, `authoring-conventions` and `guardrail` | 1 | Stay. Each cites `doubt-driven-development` or `using-codebase-memory` by bare name |
| `docs/results-of-formal-methods-trial.md` | 1 | Stays. It names `doubt-driven-development` and the leaves of its campaign by bare name |
| The two walkthrough chapters of `docs/walkthroughs/grove-loop/` | 1 | Stay. Chapter 3 derives the name `grove-impl` from the label `impl`. Chapter 18 shows the label in a composed prompt. Neither states where a pack ships the kind |
| `plugins/grove/README.md`, `plugins/README.md` and `plugins/CONTEXT.md` | 1 | Stay. Each states that a Grove plugin ships a moving skill. `engineering-removal-k204` repairs each location claim |
| `docs/repository-moves-record.md`, `external-pack-contract.md`, `owner-skill-delivery.md` and `pack-membership.md` | 1 | Stay as this workstream's record and design |
| The acceptance record, `grove-dual-ravel-stage-record.md` and the trial cases | 3 | Stay as evidence of their runs |
| `docs/owner-skill-inventory.tsv`, `inventory.py` and `docs/ravel-caller-surface-inventory.tsv` | 3 | Stay as the baseline evidence |
| `CHANGELOG.md` and the task notes of `.grove/` | History | Stay |

### Scratch removal of the engineering pack

A plain copy of the working copy, outside the jj tree, lost the fifteen
moving directories. A fresh Git repository committed it as the tree
`487158bcb211`. `bash scripts/check.sh` then failed 5 of 21 checks. Each
failure is the same named read: the four rows of core's manifest whose owner
is `grove-impl/SKILL.md` or `grove-prototype/SKILL.md`.

| Check | Reading |
| --- | --- |
| `conformance` | 4 unshipped owners. 109 single-source checks and 996 loaded-path checks |
| `conformance suite` | 97 passed and 1 failed. The failure is `core alone reads clean over the shipped plugin` |
| `external packs` | `requirements`, `design` and `research` failed in the aggregate on the 4 rows. `testanyware` passed |
| `external pack controls` | 9 passed and 3 failed, each on the 4 rows |
| `Ravel caller assets and provenance` | Its `ravel:provenance` step runs `conformance.sh` and fails on the 4 rows |

The other 16 checks passed. They include `plugin install`, the harness trial
runner controls and `book-check`. The next paragraph voids the three Rust
checks among them.

The Rust readings of that run are void. The scratch `target/` was a
copy-on-write clone, and Cargo's dependency records name the original
absolute paths. Cargo reused binaries built from the real checkout, so the
composition tests passed without their embedded file. After `cargo clean -p`
of the seven workspace crates, clippy and nextest stopped at compile time.
The cause is the `include_str!` of `doubt-driven-development/SKILL.md` in
`composition_guidance.rs` and `session_kind_guidance.rs`. The doctests
passed.

Only in the scratch copy, each embed then became an empty string. The copy
also became a jj workspace, because the tests of `prompt.rs` resolve the
repository as one. Clippy passed. Nextest ran 1345 tests, with 1337 passed,
8 failed and 25 skipped. Each failure is in a class of
`engineering-removal-k204`:

| Test | Class | Cause |
| --- | --- | --- |
| Four tests of `composition_guidance.rs` | Named read | The doubt assertions. Each names `doubt-driven-development/SKILL.md` |
| `every_session_kind_is_named_by_the_provisioned_taxonomy` | Named read | The doubt assertion of `session_kind_guidance.rs` |
| `no_skill_file_writes_a_plugin_qualified_name` | Named read | `MOVING_KIND_OPENINGS` lists `grove-impl/SKILL.md` |
| `the_kind_table_agrees_with_core_and_counts_no_set` | Kind table row | The core row `impl` has no `grove-impl` skill in core |
| `the_filename_classifier_separates_the_current_grammar_from_its_predecessor` | Kind table row | The label set is core's walk plus the external rows of the kind table. `impl` is in neither, so `02-impl-k3.md` reads as `MissingKind` |

The two kind table failures end when the removal names `EngineeringSkills`
in the rows `impl` and `prototype`. The walks of core's shipped skills kept
their floors: `shipped_kinds` and the shared label walk found eight
`grove-<kind>` directories.

Before the copy took the changed native test, that test also failed, with
the diagnostic `the installer did not supply grove-impl`. Before the copy was a
jj workspace, 16 tests of `prompt.rs` also failed with `NotAWorkspace`. That
cause is the scratch setup, not the removal.

### Checks of the consumer retargeting

`bash scripts/check.sh` passed all 21 principal checks on the Grove line, with
1345 Rust tests. Core alone gave 111 single-source checks and 1704
loaded-path checks. The control suite gave 98 passed. `GROVE_TEST_RAVEL`
resolved to the installed Ravel 1.1.0,
`/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, SHA-256
`c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. The
changed files had the same digests before and after the run. No check reads
this record.

### Source of the engineering pack

| Subject | Value |
| --- | --- |
| Repository | `https://github.com/Linkuistics/EngineeringSkills.git`. See [Publication of the engineering pack](#publication-of-the-engineering-pack) |
| First pack commit | `d98862f217492a44932c8fa93a006367da252a5d`, the jj bookmark `main` |
| Source directory | `plugins/engineering`. It is the Claude Code plugin root and the Ravel source directory |
| Ravel roots | `["skills", "craft"]`. `skills/` holds the six kinds, and `craft/` holds the nine craft skills |
| Plugin manifest | `"skills": ["./craft/"]`, so Claude Code scans `skills/` by default and adds `craft/` |
| Git tree of the source directory | `d8a891cd5c13e9866f7937537afc7979fb3ef0f7` |
| `resourceDigest` | `7e70b280e7a932489a048d108d4dae2bc63b387cf8fc6b37298663336e0fcc66`, from a plain Git checkout of the commit and from the jj working copy |
| Recorded Grove commit | `d65914d57015d88924edf5dc9d22df467ff82264`, the `GROVE_COMMIT` of the Taskfile. It is the commit of `impl-consumer-retarget-k202`, and it still ships the moving files |
| Grove remote bookmark | `gh-issue-25`. jj moved it forward from `84fba18c1f98` to `d65914d57015` and pushed it before the pack's publication |

The tracked files are `.gitignore`, `LICENSE`, `README.md`, `Taskfile.yml`,
the two scripts of the doubt check in `scripts/`, and twenty-three files under
`plugins/engineering`:

- `.claude-plugin/plugin.json`
- `PROVENANCE.md`
- `conformance/rules.tsv`
- `skills/grove-impl/SKILL.md`
- `skills/grove-impl/LICENSES/addyosmani-agent-skills.LICENSE`
- `SKILL.md` of `grove-prototype`, `grove-review-impl`,
  `grove-integrate-review-impl`, `grove-review-prototype` and
  `grove-integrate-review-prototype` in `skills/`
- `SKILL.md` of the six `coding-style-*` skills and `simplify-project` in
  `craft/`
- `craft/using-codebase-memory/SKILL.md`
- `craft/using-codebase-memory/references/reading-the-graph.md`
- `craft/doubt-driven-development/SKILL.md`
- `craft/doubt-driven-development/LICENSES/addyosmani-agent-skills.LICENSE`
- `craft/doubt-driven-development/references/harness-spawns.md`
- `craft/doubt-driven-development/references/measured-workstream.md`

The repository has no Git attribute and no symlink. The checks write only
`.cache/`, which Git ignores. `LICENSE` and `.gitignore` equal those of
DesignSkills, and `LICENSE` equals Grove's. The Taskfile is the Taskfile of
DesignSkills with four changes: the pack's name in the first comment,
`GROVE_COMMIT`, `PACK` and the task `check:doubt`. The task `check` runs
`check:plugin`, then `check:doubt`, then `check:conformance`.
`GROVE_REPOSITORY` stays the public URL.

The manifest sets `name` `engineering`, `license` `Apache-2.0`,
`dependencies` `["grove"]` and `skills` `["./craft/"]`, with no `version`. Its
other fields follow the design manifest.

### Byte changes of the engineering pack

Each change below is against Grove's files at the recorded Grove commit. The
digest is that of `docs/owner-skill-inventory.tsv`: the first 16 hexadecimal
digits of the skill tree's SHA-256. A copy of the inventory's `digest()`
function gave each value below. It gave the baseline value for each copy under
the owner's Claude Code cache. At the recorded commit, each of Grove's fifteen
directories has its baseline digest.

| Skill | Baseline digest and files | Pack digest and files | Change |
| --- | --- | --- | --- |
| `grove-impl` | `116a8f3647da6d5b`, 2 | `4588bcbbe6eef749`, 2 | The opening paragraph has the bare-name text of core's `grove-planning`, with the kind's name changed. `LICENSES/addyosmani-agent-skills.LICENSE` is unchanged |
| `grove-prototype` | `5067909a1359fa93`, 1 | `53d34980795adc54`, 1 | The opening paragraph has the bare-name text of core's `grove-planning`, with the kind's name changed |
| `grove-review-impl` | `889495e33758f2e4`, 1 | `ac3bbccc1243a7f8`, 1 | The opening paragraph equals that of core's `grove-review-planning`. It keeps the read of `references/review.md` after the load sentence |
| `grove-integrate-review-impl` | `13b9ff4a4c530852`, 1 | `2d8e5a70152ee1b4`, 1 | The opening paragraph equals that of core's `grove-integrate-review-planning`. It keeps the read of `references/integrate-review.md` after the load sentence |
| `grove-review-prototype` | `75565a7a6f38113e`, 1 | `a9c291f8c6a77d1d`, 1 | As `grove-review-impl` |
| `grove-integrate-review-prototype` | `74654c03ef8c7430`, 1 | `73d24ee9d17cd58d`, 1 | As `grove-integrate-review-impl` |
| `coding-style-bash` | `8ff8f1397a74a9ba`, 1 | The baseline | None |
| `coding-style-elixir` | `bceca00f50bb8541`, 1 | The baseline | None |
| `coding-style-python` | `38f9464cafeaec58`, 1 | The baseline | None |
| `coding-style-rust` | `eaf7196c4c4b028a`, 1 | The baseline | None |
| `coding-style-swift` | `f2b020a48bb453f0`, 1 | The baseline | None |
| `coding-style-typescript` | `279d0bb61b7b90f9`, 1 | The baseline | None |
| `simplify-project` | `cb93a1f191930034`, 1 | The baseline | None |
| `using-codebase-memory` | `f66a94d8169a6ce2`, 2 | The baseline | None |
| `doubt-driven-development` | `2f30c8cd6061072b`, 3 | `071407a9b20c4308`, 4 | Two changes. The new file `LICENSES/addyosmani-agent-skills.LICENSE` equals the file of `grove-impl` byte for byte. `references/measured-workstream.md` links `docs/candidate-lessons.md` to `https://github.com/Linkuistics/grove/blob/main/docs/candidate-lessons.md`, as it already linked `docs/review-yield.md`. `SKILL.md` is unchanged |

A `diff -r` against Grove's fifteen directories showed only those changes. A
diff of each moved family member against its core counterpart showed only the
name, the description, the title and the artifact line. Grove's public `main`
holds both linked records, and each URL returned HTTP status 200. The records
stay in Grove, by test 1 of "The Grove assets of moving craft skills" in
`docs/specs/pack-membership.md`.

A scan of the moved Markdown for `<word>:<word>` tokens found `grove:grove`
once in each kind opening and `select:mcp` in a ToolSearch query of
`using-codebase-memory`. After the move, the pack holds no `grove:grove`. The
moved files hold two relative links. `using-codebase-memory` links its own
`references/reading-the-graph.md`. `doubt-driven-development` links
`../../PROVENANCE.md`, which resolves to `plugins/engineering/PROVENANCE.md`.

`doubt-driven-development` names Grove's `references/decompose.md` and
`references/integrate-review.md` in its section "Composition with Grove". It
names them as files of the `grove` skill, not by a directed load. Conformance
does not read `craft/`, so the pack alone lists no edge for them.

### Provenance of the engineering pack

`plugins/engineering/PROVENANCE.md` holds the section of Grove's
`plugins/linkuistics/PROVENANCE.md` for `doubt-driven-development`. The six
`coding-style-*` skills, `simplify-project` and `using-codebase-memory` have no
section in either file. The relative link of `doubt-driven-development` to
`PROVENANCE.md` resolves inside `plugins/engineering`. Grove keeps its copy of
the section until `engineering-removal-k204`.

The new file has its own title and a paragraph that names its source. The
section gains one sentence that names
`doubt-driven-development/LICENSES/addyosmani-agent-skills.LICENSE`.

Of the moved skills, `grove-impl` and `doubt-driven-development` adapt
passages of `addyosmani/agent-skills` (MIT). Each therefore holds that licence
text inside its own skill. In Grove, `doubt-driven-development` names the
source in its provenance section and holds no `LICENSES/`. The pack adds the
file.

`simplify-project` names licence and provenance only as categories of
files to keep. It adapts no third-party passage and has no `LICENSES/`. The
coding styles and `using-codebase-memory` adapt nothing.

### Rules of the engineering pack

`plugins/engineering/conformance/rules.tsv` holds four rows. They equal the
rows of Grove's manifest whose owner is a moving file, byte for byte and in
Grove's order:

| Rule | Owner | Class | Load |
| --- | --- | --- | --- |
| `cite-framework-decisions-to-source` | `grove-impl/SKILL.md` | `B+S` | `static({impl})` |
| `hard-to-reverse-pairs-with-doubt` | `grove-impl/SKILL.md` | `B+S` | `static({impl})` |
| `impl-ships` | `grove-impl/SKILL.md` | `B` | `static({impl})` |
| `prototype-is-throwaway` | `grove-prototype/SKILL.md` | `B` | `static({prototype})` |

The family members and the craft skills own no row. The citation registry of
`crates/grove/tests/plugin_fallback.rs` has no row whose owner is a moving
kind. Its only use of `grove-impl/SKILL.md` is the fabricated map of
`an_unregistered_citation_is_caught`. Grove still holds the four rows, and
`engineering-removal-k204` removes them.

### Doubt check of the engineering pack

The doubt assertions are seven Grove tests, not the eight of the planning. Six
tests of `crates/grove-llm/tests/composition_guidance.rs` and one test of
`crates/grove-llm/tests/session_kind_guidance.rs` read `DOUBT_SKILL`.
`scripts/doubt-composition.sh` has one assertion for each test. It holds the
skill's half of the test, and it names the Grove phrase that the skill's half
composes with. Grove keeps the half that reads its own files.

| Assertion | Grove test | The skill's half | Grove's phrase and file |
| --- | --- | --- | --- |
| `no_deleted_composition_verb` | `no_provisioned_surface_instructs_a_deleted_composition_verb` | Holds `grove-llm leaf-add <parent> <stem> --kind review-<producer>`. Holds neither `leaf-add-chain` nor `leaf-promote-chain` | The same verb, in `grove/references/decompose.md` |
| `no_withdrawn_adjacency_exception` | `guidance_cuts_the_integrate_step_where_pick_reaches_it_next` | Does not hold "touches no file the findings cite" | "the first sibling entry after the review whose subtree still holds live work", in `grove/references/decompose.md` |
| `yields_to_grove_review_allowance` | `doubt_guidance_yields_to_grove_without_changing_standalone_doubt` | Holds the six phrases of the test and "3 cycles" | "across the whole picked leaf", "outside that predicate, doubt keeps its standalone bounded cycles unchanged" and "substantial redesign as a new producer review chain beside the leaf it is integrating", in `grove/references/execute.md` |
| `handoff_retires_before_commit` | `every_handoff_passage_retires_before_it_commits` | Between "After cutting that leaf" and "Grove owns the escalated route", "retir" comes before "commit" | "retire the leaf as `references/retire.md` directs, before committing", in `grove/SKILL.md` |
| `no_receipt_routing` | `canonical_guidance_drops_receipt_and_diversity_era_review_routing` | Holds none of the five withdrawn phrases. Holds "Grove compares no targets and raises no notice, so there is none to wait for or consume" | "task-tree driver records nothing about how the producer ran, compares nothing, and warns about nothing", in `grove/references/decompose.md` |
| `mandate_ownership_predicate` | `grove_and_doubt_state_one_mandate_based_ownership_predicate` | Holds "mandate", "Bootstrap" and "adopted that mandate by running". Does not hold "Bootstrap-and-pick" | "adopted that mandate by running Bootstrap", in `grove/references/execute.md` |
| `names_research_kinds` | `every_session_kind_is_named_by_the_provisioned_taxonomy` | Holds `research-a`, `research-b` and `combine-research` | "`research-a`, `research-b` and `combine-research` spend **none**", in `grove/references/execute.md` |

The check collapses runs of white space to one space and compares
case-sensitively, as the Rust tests do. It reads Grove's files from the
`GROVE_COMMIT` checkout that `grove:checkout` makes. It is Bash, because the
pack's Taskfile already needs only Bash and Git. ShellCheck and shfmt report
nothing for both scripts. Two ShellCheck notes on literal Markdown backticks
carry a directive.

`scripts/doubt-composition.test.sh` holds the controls. The unchanged copies
passed. Each of the fourteen mutants failed its own assertion alone, with exit
status 1:

| Assertion | Mutant of the skill | Mutant of Grove's file |
| --- | --- | --- |
| `no_deleted_composition_verb` | Appends "Run leaf-add-chain." | `decompose.md`: `--kind review-<producer>` becomes `--kind review` |
| `no_withdrawn_adjacency_exception` | Appends a sentence that holds "touches no file the findings cite" | `decompose.md`: "whose subtree still holds live work" becomes "whose subtree holds work" |
| `yields_to_grove_review_allowance` | "Substantial redesign becomes" becomes "Substantial redesign is" | `execute.md`: "across the whole picked leaf" becomes "across each artifact" |
| `handoff_retires_before_commit` | Removes "retire the producer, then" | `grove/SKILL.md`: "before committing" becomes "after committing" |
| `no_receipt_routing` | Appends "Wait for the handoff receipt." | `decompose.md`: "compares nothing, and warns about nothing" becomes "compares targets, and warns about drift" |
| `mandate_ownership_predicate` | Appends "Ownership follows Bootstrap-and-pick." | `execute.md`: "Bootstrap" after "adopted that mandate by running" becomes "pick" |
| `names_research_kinds` | `combine-research` becomes `combine-survey` | `execute.md`: "spend **none**" becomes "spend **one**" |

A mutant is the normalised text of its file. The baseline run shows that
normalisation alone changes no reading. A mutation that changes nothing fails
its control. In a meta-control, a scratch copy of the check lost the absence
of "Bootstrap-and-pick". The controls then gave 14 passed and 1 failed. The
failure was the skill-side control of `mandate_ownership_predicate`, which
read exit status 0.

`check:doubt` runs the check, then the controls. A control that stops failing
therefore fails the pack's check.

### Checks of the engineering pack

The checks ran in a plain Git checkout of the first pack commit, cloned from
the public remote. `GROVE_REPOSITORY` was the public URL, with no override.
`grove:checkout` fetched the recorded Grove commit from Grove's public remote.
The checkout was clean after the run, apart from the ignored `.cache/`.

- `check:plugin` passed with two warnings, no `version` and no icon. Claude
  Code was 2.1.318. The validation reads the manifest and its `craft/` entry.
- `check:doubt` passed. The check gave 7 assertions passed, and the controls
  gave 15 passed and 0 failed.
- The pack alone passed with exit status 0. It checked 2 single-source rows
  and 4 loaded paths, with 2 rows that have no pinned wording.
- The aggregate failed on 14 assertions, as the order of the move expects.
  `task check` therefore exits non-zero at this commit, after `check:plugin`
  and `check:doubt` pass.

Conformance reads only `skills/`. No row has an owner in `craft/`. The pack
alone listed 6 external edges and 5 external resources:

| Referring file | External edge or resource |
| --- | --- |
| `grove-impl/SKILL.md` | Loads `grove`. Names `references/execute.md` |
| `grove-prototype/SKILL.md` | Loads `grove` |
| `grove-review-impl/SKILL.md` | Loads `grove`. Names `references/review.md` |
| `grove-integrate-review-impl/SKILL.md` | Loads `grove`. Names `references/integrate-review.md` |
| `grove-review-prototype/SKILL.md` | Loads `grove`. Names `references/review.md` |
| `grove-integrate-review-prototype/SKILL.md` | Loads `grove`. Names `references/integrate-review.md` |

`grove-impl` names `references/execute.md` as the file that states the
leaf-wide review allowance. The moved kinds name no format file. Each external
resource resolves in `grove`, as rule 3 of the contract's "References between
skills" states.

The 14 aggregate failures have three classes:

- 6 duplicate skills, one for each moving kind.
- 4 duplicate rows, one for each moved row of Grove's manifest.
- 4 unshipped owners, one for each moved row. The runner maps a skill name
  that two packs ship to the first pack, which is core. Each pack row whose
  owner is in a moving kind therefore reads as unshipped.

Each failure names a moving kind or a moved row, and the duplicates cause all
14. Core's figures in the aggregate are 111 single-source rows and 1704 loaded
paths.

### Ravel resolution of the engineering pack

`ravel skills resolve --json` resolved the plain checkout from the public
remote with the status `resolved`. The source record has the type `local` and
the digest `7e70b280e7a9…`. Its two search directories are the `skills` and
`craft` roots of the snapshot. The `skills` root holds the six kinds, and the
`craft` root holds the nine craft skills. The snapshot holds fifteen
`SKILL.md` files.

`diff -r` found no difference between the snapshot and
`plugins/engineering` of the checkout. A second resolution of the jj working
copy gave the same digest.

The instrument was the Ravel 2.0 build of `scripts/harness-trials/builds.json`.
`task install` installed it from the line `skill-directory-delivery` at
`71f70fcd3f92` into a scratch prefix. Its `bin/ravel` SHA-256 is
`26b487afb957…`, the pinned value, and it holds the build identity
`a4f55d04b2d1…` twice. The prefix has no `libexec`. Each resolution ran under
a temporary `HOME` whose settings held one `local` source with the roots
`["skills", "craft"]` and an empty mandatory list.

### Publication of the engineering pack

- This session ran
  `gh repo create Linkuistics/EngineeringSkills --public`. The uncommitted
  `.claude/settings.json` of the Grove workspace allowed it, so no prompt
  came.
- jj added the remote `origin` and pushed the bookmark `main`. The public
  `main` is `d98862f217492a44932c8fa93a006367da252a5d`.
- The recorded Grove commit `d65914d57015` is the head of Grove's remote
  bookmark `gh-issue-25`. jj pushed that bookmark before the pack.
- At this publication, Grove's marketplace does not list the pack.
  `engineering-removal-k204` adds the entry with the removal. That entry pins
  the first pack commit, because the first commit holds the doubt check.

### Tool requirements of the engineering pack

The skills name these tools. The README of the pack holds the same table.

| Skill | Tools |
| --- | --- |
| `coding-style-bash` | Bash, ShellCheck, shfmt and bats-core |
| `coding-style-elixir` | Elixir and Mix, with `mix format`, Credo and Dialyzer |
| `coding-style-python` | Python, uv, Ruff, pyright and pytest. mypy is acceptable in an existing project |
| `coding-style-rust` | Rust and Cargo, with rustfmt and Clippy |
| `coding-style-swift` | Swift and SwiftPM, with swift-format and SwiftLint |
| `coding-style-typescript` | TypeScript, Biome and vitest. Node.js, pnpm or Bun as the project configures |
| `using-codebase-memory` | `codebase-memory-mcp`. The skill states that its notes describe version 0.8.1 |

The inventory's tool rows hold `codebase-memory-mcp`. The checks of the pack
need Bash, Git, Task and, for `check:plugin`, Claude Code.

### Removal of the engineering pack

`engineering-removal-k204` removed Grove's copies in one change with every
Grove consumer, on 2026-11-05. This is the contraction step of the move.

| Subject | Value |
| --- | --- |
| Removal commit | `f45c5a53796f2a5bf634fec340cb0d7aef63e184` |
| Pushed Grove commit | `f45c5a53796f`. jj moved the remote bookmark `gh-issue-25` forward from `d65914d57015` before the second pack commit's publication |
| First pack commit | `d98862f217492a44932c8fa93a006367da252a5d`. Grove's marketplace pins it, because it holds the doubt check |
| Second pack commit | `549607bbb2e13baedca02d8c2481d6ad65541a26`, a child of the first. The public `main` |
| Tree of `plugins/engineering` | `d8a891cd5c13e9866f7937537afc7979fb3ef0f7` in both pack commits |
| `resourceDigest` | `7e70b280e7a932489a048d108d4dae2bc63b387cf8fc6b37298663336e0fcc66` for both commits, from a clone of the public remote at the second commit |

The removal took out 19 tracked files. They are the six kind directories of
`plugins/grove/skills/` with 7 files, and the nine craft directories of
`plugins/linkuistics/skills/` with 12 files. Core's manifest lost the four
rows whose owner is `grove-impl/SKILL.md` or `grove-prototype/SKILL.md`. The
prefix sets `review-*` and `integrate-review-*` stay. These consumers changed:

- The kind table of `TASK-FORMAT.md` names `EngineeringSkills` in the rows
  `prototype` and `impl`. The label tests take those labels from the external
  rows, with no code change.
- `plugin_fallback.rs`: the six engineering openings left
  `MOVING_KIND_OPENINGS`. The four editorial openings stay.
- The doubt tests of `composition_guidance.rs` and `session_kind_guidance.rs`
  no longer read `doubt-driven-development/SKILL.md`. Five tests keep their
  assertions over Grove's files. The test
  `doubt_guidance_yields_to_grove_without_changing_standalone_doubt` read only
  the skill, so it went. `grove_guidance_replaces_the_old_in_session_review_loop`
  already held two of the three Grove phrases of the pack's assertion
  `yields_to_grove_review_allowance`, and it gained the third. Two tests that
  kept one surface became direct statements, because clippy refuses a loop
  over one element.
- `plugins/linkuistics` no longer holds the nine craft skills. Its manifest
  description and keywords, its marketplace entry and `PROVENANCE.md` name
  only what it still holds. `PROVENANCE.md` names where the doubt section
  went.
- `.claude-plugin/marketplace.json` lists `engineering` with a `git-subdir`
  source that pins the first pack commit, and it counts seven plugins.
- The install guidance names the pack for the marketplace and for
  `install.sh --pack`. `plugins/README.md` has an `engineering` section with
  the craft table, the language coverage and the `paths:` sentences.
  `docs/USAGE.md` adds an `engineering` source with the roots `skills` and
  `craft` to the directory route. `README.md`, `plugins/grove/README.md`,
  `CONTEXT.md`, `plugins/CONTEXT.md` and `docs/ARCHITECTURE.md` state where
  the kinds ship.
- Two claims were wrong before the removal. `plugins/README.md` and
  `plugins/CONTEXT.md` said that only `doubt-driven-development` carries
  `assumes-personal-setup`, but `machine-environment` also carries it. The
  architecture pointer to the formal-methods results named "the `linkuistics`
  skills". Each now states the current fact.
- The sweep found two stale examples. The dialogue of `plugins/CONTEXT.md`
  used `linkuistics:coding-style-rust`, and it now uses
  `linkuistics:using-jujutsu`. The licence rule of
  `docs/specs/external-pack-contract.md` said that `grove-impl` holds its
  `LICENSES/` "in core", and it now says EngineeringSkills.

### Readings of the removal of the engineering pack

| Reading | Single-source checks | Loaded-path checks | Unpinned rows |
| --- | --- | --- | --- |
| Core alone before the removal, at `55dae6392644` | 111 | 1704 | 53 |
| Core alone after the removal | 109 | 996 | 51 |
| The pack alone | 2 | 4 | 2 |
| The aggregate of core and the first pack commit | 111 | 1704 | 53 |

The aggregate before the removal failed on the 14 duplicates of
[Checks of the engineering pack](#checks-of-the-engineering-pack). The
aggregate after it has the figures of core alone before the move. The pack
alone lists 6 external edges and 5 external resources. The other aggregates
are now requirements 110 and 1358, design 114 and 1355, and research 109 and
1355.

In the aggregate, the core rows of `review-*` and `integrate-review-*` bind
the pack's four family members. In a scratch copy of the pinned pack, the four
members wrote "Note the `grove` skill now" in place of the directed load. The
aggregate then failed 50 assertions, and each names one of the four members:

| Failures | Count |
| --- | --- |
| The family file of each member, as a missing file | 4 |
| The 7 spine rows of `grove/SKILL.md`, for each member | 28 |
| The 3 `static(review-*)` rows, for each review member | 6 |
| The 4 `static(integrate-review-*)` rows, for each integration member | 8 |

`grove-impl` and `grove-prototype` kept their load and appear in no failure.

The supplier check of `trial-supplier-refusal-k196` refused `discovery`, an
`impl` case, at the removal commit with status 1. The prepared commit holds no
`plugins/linkuistics/skills/coding-style-bash/SKILL.md`, which the case
declares, and no `grove-impl` in either source. At `3bfb096810db`, the commit
that its record names, the check held the 8 required paths of the case with
status 0.

The stale-location search enumerated the fifteen skill names and the four
resource file names with `--hidden` over the whole tree. It then searched for
each name under `linkuistics/skills/`, `grove/skills/`, `linkuistics:` and any
`skills/` or `craft/` segment. Apart from the two stale examples above, each
hit is in a class of the next section. A planted path in a hidden scratch
directory was found, and a misspelt name found nothing.

### Asset classification of the engineering pack

Each Grove file that names a moved skill is in one class of "The Grove assets
of moving craft skills" in `docs/specs/pack-membership.md`. No asset moved,
because no Grove file states, decides or checks the behaviour of a moved
craft skill. The doubt assertions moved to the pack's doubt check in
engineering-pack-repository-k203.

| Files | Test | Disposition |
| --- | --- | --- |
| `authoring-conventions`, `guardrail`, `CONTEXT-MAP.md` and `docs/adr/evidence-outlives-the-instrument.md` | 1 | Stay. Each cites a moved skill by bare name or by role |
| `docs/results-of-formal-methods-trial.md` and the dated account in `CONTEXT.md` | 1 | Stay. Their path mentions are accounts of their time |
| The two walkthrough chapters of `docs/walkthroughs/grove-loop/` | 1 | Stay. Chapter 3 derives `grove-impl` from `impl`, and chapter 18 shows the composed prompt |
| The acceptance record, `grove-dual-ravel-stage-record.md` and the trial cases | 3 | Stay as evidence of their runs |
| `docs/owner-skill-inventory.tsv`, `inventory.py` and `docs/ravel-caller-surface-inventory.tsv` | 3 | Stay as the baseline evidence |
| `CHANGELOG.md`, this record, `owner-skill-delivery.md` and `pack-membership.md` | History and design | Stay |
| `directory_route.rs`, `native_skill_delivery.rs`, `lifecycle_cutover.rs`, `prompt.rs`, `grove-floor-directory-case.sh`, `test_harness_trial.py` and the `rival-style` source | Fixture label | Stay. Each writes or matches a name in a temporary tree, a prompt or a log |

### Checks of the removal of the engineering pack

- `bash scripts/check.sh` passed all 21 principal checks on the removal, with
  1344 Rust tests. That is one fewer than before, because the skill-only doubt
  test went. `GROVE_TEST_RAVEL` resolved to the installed Ravel 1.1.0,
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, SHA-256
  `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. The
  changed files had the same digests before and after the run. An earlier run
  failed `cargo clippy` alone, on the two loops over one element.
- `task check:packs` passed `requirements`, `design`, `engineering` and
  `research`, alone and in the aggregate. `testanyware` passed the
  directory-only check.
- `claude plugin validate plugins/linkuistics` passed with two warnings, no
  `version` and no icon. Claude Code was 2.1.318.
- The second pack commit's full `task check` passed from a clean plain-Git
  checkout, with the committed public `GROVE_REPOSITORY` and no override.
  `grove:checkout` fetched `f45c5a53796f` from GitHub. `check:plugin` passed
  with the two warnings of the first commit. `check:doubt` gave 7 assertions
  passed and 15 controls passed. The pack alone gave 2 and 4, and the
  aggregate gave 111 and 1704. The checkout held only the ignored `.cache/`
  after the run.
- After the publication, a clone of the public remote was at `549607bbb2e1`,
  with the tree `d8a891cd5c13`. `ravel skills resolve --json` resolved it as
  a `local` source with the status `resolved` and the digest above. The
  snapshot's `skills` root holds the six kinds, and its `craft` root holds the
  nine craft skills. `diff -r` found no difference between the snapshot and
  `plugins/engineering`. The instrument was the Ravel 2.0 build of
  `scripts/harness-trials/builds.json`, `bin/ravel` SHA-256 `26b487afb957…`,
  with its build identity twice. Its `--version` prints `1.1.0`, so the digest
  and the identity are the pin.
- An isolated home held Codex, Gemini CLI and Pi directories. In it,
  `install.sh --force --pack <public clone>/plugins/engineering` linked the
  six kinds from `skills/` and the nine craft skills from `craft/` into each
  harness. The clone was at the first pack commit, which has the same tree.
  The run made 90 links in all and skipped `guardrail` three times. `--force`
  was needed because the Grove workspace is a secondary jj workspace. The
  owner's real skill links had the same digest before and after.

## WritingSkills

`writing-pack-repository-k205` created the repository
`~/Development/AgenticOrchestration/WritingSkills` on 2026-11-05, with the
four editorial kinds and the family skill `editorial-family`. This is the
expand step of `editorial-pack-k179`. At the first pack commit, Grove still
ships its own copies, so the aggregate fails on duplicates by design.
`editorial-removal-k206` removes Grove's copies and publishes the second pack
commit. `writing-craft-pack-k181` later adds the `craft/` root and the hooks
plugin.

### Source of the writing pack

| Subject | Value |
| --- | --- |
| Repository | `https://github.com/Linkuistics/WritingSkills.git`. See [Publication of the writing pack](#publication-of-the-writing-pack) |
| First pack commit | `694140a18dc736c272c5a16588253710a92844fb`, the jj bookmark `main` |
| Source directory | `plugins/writing`. It is the Claude Code plugin root and the Ravel source directory |
| Ravel roots | `["skills"]`. The pack has no `craft/` directory yet |
| Git tree of the source directory | `361070804328e5b30bcfd874125ac58c04a52d0b` |
| `resourceDigest` | `db96fa09905e5dca83325d759bff2b153ca9cb22144386049b3ed00ffffa7d31`, from a plain Git checkout of the commit and from the jj working copy |
| Recorded Grove commit | `f45c5a53796f2a5bf634fec340cb0d7aef63e184`, the `GROVE_COMMIT` of the Taskfile. It is the removal commit of `engineering-removal-k204`, and it still ships the moving files |
| Grove remote bookmark | `gh-issue-25`, already at the recorded commit. The two later commits of the line change only `.grove/` and `docs/`, so no push was needed |

The tracked files are `.gitignore`, `LICENSE`, `README.md`, `Taskfile.yml`
and seven files under `plugins/writing`:

- `.claude-plugin/plugin.json`
- `conformance/rules.tsv`
- `skills/editorial-family/SKILL.md`
- `SKILL.md` of `grove-draft`, `grove-copy-edit`, `grove-art` and
  `grove-proof` in `skills/`

The repository has no Git attribute and no symlink. The checks write only
`.cache/`, which Git ignores. `LICENSE` and `.gitignore` equal those of
RequirementsSkills, and `LICENSE` equals Grove's. The Taskfile is the Taskfile
of RequirementsSkills with three changes: the pack's name in the first
comment, `GROVE_COMMIT` and `PACK`. `GROVE_REPOSITORY` stays the public URL.

The manifest sets `name` `writing`, `license` `Apache-2.0` and
`dependencies` `["grove"]`, with no `version`. Its other fields follow the
requirements manifest.

### Byte changes of the writing pack

Each change below is against Grove's files at the recorded Grove commit. The
digest is that of `docs/owner-skill-inventory.tsv`. A copy of the inventory's
`digest()` function gave each value below. It gave the baseline value for each
copy under the owner's Claude Code cache. Each of Grove's four kind
directories at the recorded commit has its baseline digest. Each Grove file
equals its cache copy byte for byte.

| Skill | Baseline digest and files | Pack digest and files | Change |
| --- | --- | --- | --- |
| `grove-draft` | `7679a927af6b0094`, 1 | `fb21af42dc743366`, 1 | The first paragraph has the opening of `research-a` in the contract's "References between skills", with `editorial-family` for `research-family`. The second paragraph names the `editorial-family` skill where it said "that file", and the paragraph is wrapped again |
| `grove-copy-edit` | `e8d292388f59c9c2`, 1 | `70786827a3634bf4`, 1 | As `grove-draft` |
| `grove-art` | `f5fdf5319d632bad`, 1 | `98de33804c08a3bb`, 1 | As `grove-draft` |
| `grove-proof` | `07339dc854e6a7d3`, 1 | `f2032e99a054a049`, 1 | As `grove-draft` |
| `editorial-family` | None. The source is `references/editorial.md` of the baseline entry `grove`, `b6baf08f77704f96`, 19. That file's digest is `e4ab4b8b17b1d8e0` in the baseline and at the recorded commit | `eaae0bf8e1b3f0f6`, 1 | New frontmatter: the name `editorial-family`, a description that names the four kinds, and `harnesses: [any]`. Line 75 of the body changes "`references/execute.md`'s" to "the spine's". No other line changes |

A `diff` of each kind against Grove's copy showed only its first two
paragraphs. A `diff` of the family body against `editorial.md` showed only
line 75.

The change to line 75 follows rule 3 of "References between skills". The
family skill loads no skill, so `references/execute.md` resolves in no skill
that the file loads. The pack alone then reports a missing file. In a scratch
copy of the source directory, the original phrase gave exactly that failure:

```text
FAIL   missing file: editorial-family/SKILL.md names `references/execute.md`, which is in neither editorial-family nor a skill that the file loads
```

The reader has already loaded the spine, which states the review allowance,
so "the spine's" keeps the meaning.

A scan of the moved Markdown for `<word>:<word>` tokens found `grove:grove`
once in each kind opening. After the move, the pack holds no `grove:grove`.
The moved files cite no other skill. Their only spine path was
`references/editorial.md` in each opening and `references/execute.md` in the
family file. `grove-art` links two public `typst.app` pages, and no moved
file links a Grove record. No moved file adapts a passage of a third party,
so the pack has no `LICENSES/`.

The second paragraph of each kind keeps the "family file" wording of its
last-act sentence. The glossary entry "Family reference file" names the family
skill as the family file of an external pack, so the wording stays true.

The moved text keeps its findings of the technical-communication checker. It
is quoted source text. The new sentences in the openings, the frontmatter and
the README have no finding.

### Rules of the writing pack

`plugins/writing/conformance/rules.tsv` holds the 23 rows that the moving
files own, in Grove's order:

- 11 rows had the owner `grove/references/editorial.md` and the load
  `static(editorial)`. They now have the owner `editorial-family/SKILL.md` and
  the load `static({draft,copy-edit,art,proof})`.
- 12 rows keep their owner and their load: 3 of `grove-art/SKILL.md`, 2 of
  `grove-copy-edit/SKILL.md`, 3 of `grove-draft/SKILL.md` and 4 of
  `grove-proof/SKILL.md`, each with the load of its own kind.

Each row keeps its identifier, class and pinned phrase. The leaf named only
the 11 family rows. The contract's "Manifest changes" moves every row that a
moving file owns. The two pack commits must also have the same
`plugins/writing`, so the 12 kind rows are in the first commit. No pinned
phrase contains the changed words of line 75.

The selection was the owner column of Grove's manifest. A search of every row
for the four kind names and `editorial` found no other row. Grove still holds
the 23 rows, and `editorial-removal-k206` removes them.

### Checks of the writing pack

The checks ran in a plain Git clone of the first pack commit from the public
remote. `GROVE_REPOSITORY` was the public URL, with no override.
`grove:checkout` fetched the recorded Grove commit from Grove's public remote.
The checkout was clean after the run, apart from the ignored `.cache/`.

- `check:plugin` passed with two warnings, no `version` and no icon. Claude
  Code was 2.1.318.
- The pack alone passed with exit status 0. It checked 23 single-source rows
  and 56 loaded paths. No row lacks a pinned phrase.
- The aggregate failed on 105 assertions, as the order of the move expects.
  `task check` therefore exits non-zero at this commit, after `check:plugin`
  passes.

The pack alone listed 4 external edges and 0 external resources. Each kind
loads `grove`, which the pack does not ship. `editorial-family` loads nothing
and names no path. The pack alone listed no missing file.

The 105 aggregate failures have five classes:

- 4 duplicate skills, one for each moving kind.
- 23 duplicate rows, one for each moved row.
- 12 unshipped owners, one for each kind row. The runner maps a skill name
  that two packs ship to the first pack, which is core. A pack row whose
  owner is in a moving kind therefore reads as unshipped.
- 22 reports of a second site, two for each family row. Each such phrase is
  in `editorial.md` and in `editorial-family/SKILL.md`, and each of the two
  manifests checks it.
- 44 reports of a family row off a kind's path, one for each family row and
  kind. Core's copy of each kind answers the kind's name, and that copy loads
  no `editorial-family`.

Each failure names a moving skill or a moved row, and the duplicates cause all
105. A development run with the local Grove repository gave the same 105
failures. The comparison replaced each checkout root and then compared the
sorted lines one by one.

### Ravel resolution of the writing pack

`ravel skills resolve --json` resolved the public clone with the status
`resolved`. The source record has the type `local` and the digest
`db96fa09905e…`. Its one search directory is the `skills` root of the
snapshot, which holds the four kinds and `editorial-family`. `diff -r` found
no difference between the snapshot and `plugins/writing` of the clone. A
local clone before the publication and the jj working copy gave the same
digest.

The instrument was the Ravel 2.0 build of `scripts/harness-trials/builds.json`.
`task install` installed it from the line `skill-directory-delivery` at
`71f70fcd3f92` into a scratch prefix. Its `bin/ravel` SHA-256 is
`26b487afb957…`, the pinned value. Each resolution ran under a temporary
`HOME` whose settings held one `local` source with the roots `["skills"]` and
an empty mandatory list.

### Publication of the writing pack

- The permission classifier first refused `gh repo create`. After the owner
  gave permission, this session ran
  `gh repo create Linkuistics/WritingSkills --public`.
- jj added the remote `origin` and pushed the bookmark `main`. The public
  `main` is `694140a18dc736c272c5a16588253710a92844fb`, the default branch.
- The recorded Grove commit `f45c5a53796f` is the head of Grove's remote
  bookmark `gh-issue-25`. It was already there, so no push came before the
  pack.
- Grove's marketplace does not list the pack yet. `editorial-removal-k206`
  adds the entry with the removal, and it pins this first pack commit.

### For the paused editorial grove

The editorial kinds and their family file ship in WritingSkills from this
commit. Grove still ships its copies until `editorial-removal-k206`. Rollout's
brief-update text for the paused editorial grove states that move.

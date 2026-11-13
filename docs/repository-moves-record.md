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

The same holds for `writing`, which the marketplace lists after finish. A
native user then needs `/plugin install writing@linkuistics`. Without it, a
`draft`, `copy-edit`, `art` or `proof` leaf has no kind skill on the native
route. A directory route needs a `writing` source with the roots `skills` and
`craft`, as `docs/USAGE.md` states.

The writing craft follows the same rule. After finish, an update of the
`linkuistics` plugin removes `technical-communication`, `writing-code-walkthroughs`
and the always-on hooks. The plugin `writing` holds both skills, and its
dependency `technical-communication-hooks` holds the hooks. A native user of
`writing` therefore keeps the always-on layers. A user who receives the skills
through Ravel needs `/plugin install technical-communication-hooks@linkuistics`
for the hooks on Claude Code. A Codex user adds the marketplace of WritingSkills
in place of Grove's, as its user guide states.

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
`editorial-removal-k206` removed Grove's copies and published the second pack
commit.

`writing-craft-pack-k181` adds the writing craft. Its first expand
commit, from `writing-craft-repository-k208`, adds the `craft/` root, both
craft skills and their Grove assets. See
[Source of the writing craft](#source-of-the-writing-craft). The hooks
plugins follow in a later commit. `writing-craft-removal-k211` removed Grove's
copies and published the last pack commit. See
[Removal of the writing craft](#removal-of-the-writing-craft).

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

### Removal of the writing pack

`editorial-removal-k206` removed Grove's copies in one change with every Grove
consumer, on 2026-11-05. This is the contraction step of the move.

| Subject | Value |
| --- | --- |
| Removal commit | `31ec9117c9dacfe993e80313addd432f2930d679` |
| Pushed Grove commit | `31ec9117c9da`. jj moved the remote bookmark `gh-issue-25` forward from `f45c5a53796f` before the second pack commit's publication |
| First pack commit | `694140a18dc736c272c5a16588253710a92844fb`. Grove's marketplace pins it |
| Second pack commit | `ae1649d181a5aa7950404797d4735c58831d542a`, a child of the first. The public `main` |
| Tree of `plugins/writing` | `361070804328e5b30bcfd874125ac58c04a52d0b` in both pack commits |
| `resourceDigest` | `db96fa09905e5dca83325d759bff2b153ca9cb22144386049b3ed00ffffa7d31` for both commits, from a clone of the public remote at the second commit |

The removal took out 5 tracked files: the four kind directories of
`plugins/grove/skills/`, with one `SKILL.md` each, and
`plugins/grove/skills/grove/references/editorial.md`. Core's manifest lost the
23 rows whose owner is a moved file. These consumers changed:

- The spine's `SKILL.md` no longer lists `references/editorial.md` among its
  family files. Its description, the `grove` plugin manifest and the `grove`
  marketplace entry name only the review and integration family files.
- The kind table of `TASK-FORMAT.md` names `WritingSkills` in the editorial
  row. Its paragraph on that row names the family skill `editorial-family`.
  The label tests take the four labels from the external row, with no code
  change.
- `conformance.awk` has no named set `editorial`. The header comment of
  `rules.tsv` names only the lists and the two prefix sets.
- `plugin_fallback.rs` has no `MOVING_KIND_OPENINGS`. The positive control of
  `no_skill_file_writes_a_plugin_qualified_name` pairs each token of
  `NOT_SKILL_NAMES` with one shipped file that holds it. It asserts that the
  enumeration finds the token in that file.
- `.claude-plugin/marketplace.json` lists `writing` with a `git-subdir` source
  that pins the first pack commit, and it counts eight plugins.
- The install guidance names the pack for the marketplace and for
  `install.sh --pack`. `plugins/README.md` has a `writing` section.
  `docs/USAGE.md` adds a `writing` source with the root `skills` to the
  directory route. `README.md`, `plugins/grove/README.md`, `CONTEXT.md`,
  `plugins/CONTEXT.md` and `docs/ARCHITECTURE.md` state where the kinds and
  their family file ship.
- `docs/specs/pass-series.md` names the `editorial-family` skill in its two
  present-tense claims. The "Out of scope" item of
  `docs/specs/external-pack-contract.md` no longer says that the family stays
  in the spine.

After the removal, core ships four kinds: `finish`, `planning`,
`review-planning` and `integrate-review-planning`. Core holds no family that
it ships alone.

### Readings of the removal of the writing pack

| Reading | Single-source checks | Loaded-path checks | Unpinned rows |
| --- | --- | --- | --- |
| Core alone before the removal, at `7aa85669463d` | 109 | 996 | 51 |
| Core alone after the removal | 86 | 480 | 51 |
| The pack alone | 23 | 56 | 0 |
| The aggregate of core and the first pack commit | 109 | 996 | 51 |

The aggregate before the removal failed on the 105 duplicates of
[Checks of the writing pack](#checks-of-the-writing-pack). The aggregate after
it has the figures of core alone before the move. The pack alone lists 4
external edges and 0 external resources. The other aggregates are now
requirements 87 and 842, design 91 and 839, engineering 88 and 1188, and
research 86 and 839.

The static proof of the boundary ran in a scratch copy of the pinned pack. In
each run, one kind wrote "Then note the `editorial-family` skill" in place of
the directed load, and the other three kinds kept their load:

| Mutated kind | Pack alone | Aggregate |
| --- | --- | --- |
| `draft` | Exit 1, 11 failed assertions | Exit 1, 11 failed assertions |
| `copy-edit` | Exit 1, 11 failed assertions | Exit 1, 11 failed assertions |
| `art` | Exit 1, 11 failed assertions | Exit 1, 11 failed assertions |
| `proof` | Exit 1, 11 failed assertions | Exit 1, 11 failed assertions |

The 11 failures of each run are the 11 rows of `editorial-family/SKILL.md`.
Each failure states that the family file is not on the mutated kind's
composed loaded path, and it names that kind. No failure names another kind.
The unmutated pack passes alone and in the aggregate.

`no_skill_file_writes_a_plugin_qualified_name` has no list of retained
openings. Its new positive control read clean on the removal tree. The
control then read dirty when `decompose.md` wrote `path line` in place of
`path:line`. It reported "the enumeration must find `path:line` in
grove/references/decompose.md". The file's SHA-256 was `d54a3bfd65ab…` before
the mutation and after the restoration, and the control then read clean again.

The stale-location search enumerated the four kind names, `editorial.md`,
`editorial-family` and `static(editorial)` with `--hidden` over the whole tree.
It then searched for each kind under `grove/skills/`, for each
`grove-<kind>/SKILL.md` path, for `references/editorial.md` and for a
`grove:` prefix. Each hit is in a class of the next section. A planted path in
a hidden scratch directory was found, and a misspelt name found nothing.

### Asset classification of the writing pack

Each Grove file that names a moved file or a moved kind is in one class of
"The Grove assets of moving craft skills" in `docs/specs/pack-membership.md`.
That rule is written for craft skills. Its own text applies the same tests to
another class. No asset moved. No Grove file states, decides or checks the
behaviour of a moved skill apart from Grove's design of its kind set.

| Files | Test | Disposition |
| --- | --- | --- |
| `docs/adr/the-editorial-pipeline-is-four-kinds.md`, `a-feedback-edge-is-forward-tree-growth.md`, `a-book-carries-no-asset.md` and `corpus-rules-have-one-owner.md` | 1 | Stay. Each decides Grove's kind set, its tree growth or its rule placement. `docs/ARCHITECTURE.md` and `CONTEXT.md` cite them |
| `docs/evaluations/editorial-pipeline-pilot/` | 3 | Stays as evidence. The decision records that stay cite it |
| The editorial chain of `docs/ARCHITECTURE.md` and `CONTEXT.md`, the kind table of `TASK-FORMAT.md`, `docs/specs/pass-series.md` and `configure-grove/references/model-selection.md` | 1 | Stay. Each names a kind by label in Grove's design. The location claims among them changed |
| The two lists under "Rule placement" in `docs/specs/external-pack-contract.md` | History and design | Stay. They name the changes of grove-research-removal-k158 |
| The routes of `assets/ravel-policy/` and `ravel_asset_cases.rs` | Fixture label | Stay. The policy routes are unchanged |
| `docs/owner-skill-inventory.tsv`, `inventory.py` and `docs/ravel-caller-surface-inventory.tsv` | 3 | Stay as the baseline evidence |
| `CHANGELOG.md`, this record, `owner-skill-delivery.md` and `pack-membership.md` | History and design | Stay |

### Checks of the removal of the writing pack

- `bash scripts/check.sh` passed all 21 principal checks on the removal, with
  1344 Rust tests. `GROVE_TEST_RAVEL` resolved to the installed Ravel 1.1.0,
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, SHA-256
  `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. The
  changed files had the same digests before and after each run. The first run
  failed one test, `a_timed_out_selection_launches_nothing_and_publishes_nothing`,
  with "select never ran". The test gives the policy a 1000 ms timeout and
  then requires that `select` started. It passed in 5 runs alone, and the
  second full run passed. No file changed between the two full runs.
- `task check:packs` passed `requirements`, `design`, `engineering`,
  `research` and `writing`, alone and in the aggregate. `testanyware` passed
  the directory-only check.
- The second pack commit's full `task check` passed from a clean plain-Git
  clone, with the committed public `GROVE_REPOSITORY` and no override.
  `grove:checkout` fetched `31ec9117c9da` from GitHub. `check:plugin` passed
  with two warnings, no `version` and no icon, on Claude Code 2.1.318. The
  pack alone gave 23 and 56, and the aggregate gave 109 and 996. The clone
  held only the ignored `.cache/` after the run.
- After the publication, a clone of the public remote was at `ae1649d181a5`,
  with the tree `361070804328`. `ravel skills resolve --json` resolved it as a
  `local` source with the status `resolved` and the digest above. The
  snapshot's `skills` root holds the four kinds and `editorial-family`.
  `diff -r` found no difference between the snapshot and `plugins/writing`.
  The instrument was the Ravel 2.0 build of `scripts/harness-trials/builds.json`,
  `bin/ravel` SHA-256 `26b487afb957…`, with its build identity twice. The
  temporary `HOME` held one `local` source with the root `skills` and an
  empty mandatory list.
- An isolated home held Codex, Gemini CLI and Pi directories. In it,
  `install.sh --force --pack <public clone>/plugins/writing` linked the four
  kinds and `editorial-family` into each harness. The clone was at the first
  pack commit and was clean. The run made 48 links in all and skipped
  `guardrail` three times. `--force` was needed because the Grove workspace is
  a secondary jj workspace. The owner's real skill links had the same digest
  before and after.

### Editorial directory trials

`editorial-directory-trials-k207` ran the case `editorial-family-read` once
in Claude Code and once in Codex, on 2026-11-05. The `grove` source came from
the prepared Grove checkout at the removal commit `31ec9117c9da`. The
`writing` source came from a plain Git checkout of the second pack commit
`ae1649d181a5`. In each session, a `copy-edit` leaf read its kind skill and
`editorial-family/SKILL.md` from the writing snapshot. It read the spine from
the grove snapshot. All three reads came before the first command on the
task.

Both runs pass, and no gap remains open.
[Editorial directory trials](common-loading-acceptance-record.md#editorial-directory-trials)
in the acceptance record holds the builds, the digests and the observations.

### For the paused editorial grove

The editorial kinds `draft`, `copy-edit`, `art` and `proof` and their
family file now ship in WritingSkills, as the plugin `writing`. The family
file is the skill `editorial-family`. Grove no longer ships them from the
removal commit `31ec9117c9da`. Rollout's brief-update text for the paused
editorial grove states that move.

### Source of the writing craft

`writing-craft-repository-k208` added the writing craft to WritingSkills on
2026-11-05. This is the first expand commit of `writing-craft-pack-k181`.
Grove keeps its copies of both craft skills, their hooks and their assets
until `writing-craft-removal-k211`. Conformance reads only `skills/`, so the
aggregate still passes at this commit.

| Subject | Value |
| --- | --- |
| Pack commit | `0eacee02f0f2594fb51b0f7dff53b0d7130133a5`, a child of `ae1649d181a5`. The jj bookmark `main` and the public `main` |
| Source directory | `plugins/writing`. It is the Claude Code plugin root and the Ravel source directory |
| Ravel roots | `["skills", "craft"]`. `skills/` holds the four kinds and `editorial-family`. `craft/` holds `technical-communication` and `writing-code-walkthroughs` |
| Plugin manifest | `"skills": ["./craft/"]`, so Claude Code scans `skills/` by default and adds `craft/` |
| Git tree of the source directory | `ca83ff27dceb18f0671c82b5f02c4c96ece5d899` |
| `resourceDigest` | `293ed909955fea68242c088c383701ba822b1b14d4d4f5cb7ed585efd754a375`, from a plain Git clone of the public remote and from the jj working copy |
| Source Grove commit | `31ec9117c9dacfe993e80313addd432f2930d679`. No moving file changed between it and `76daf77cdb24`, the parent of this leaf's Grove change |
| `GROVE_COMMIT` | Unchanged at `31ec9117c9da`. No new check reads Grove |
| Grove remote bookmark | `gh-issue-25`, already at `31ec9117c9da`. No push came before the pack |

The commit adds 1379 files and changes four. The added files are the 1378
tracked moving files of Grove and the new `plugins/writing/PROVENANCE.md`.
Each moved file keeps its Grove path, apart from the skill root:

| Grove path | Pack path |
| --- | --- |
| `plugins/linkuistics/skills/technical-communication/`, every tracked file | `plugins/writing/craft/technical-communication/` |
| `plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md` | `plugins/writing/craft/writing-code-walkthroughs/SKILL.md` |
| `docs/specs/technical-communication.md`, `docs/TECHNICAL-COMMUNICATION.md` and `docs/technical-communication-publication-review.md` | The same paths |
| `docs/design/technical-communication/` | The same path |
| The four decision records `the-off-file-is-the-only-switch`, `documents-are-gated-replies-are-measured`, `technical-communication-carries-no-standard-text` and `behavioral-acceptance-is-closed-unmet` in `docs/adr/` | The same paths |
| `docs/research/harness-hooks-a.md` | The same path |
| `docs/evaluations/writing-code-walkthroughs/` | The same path |
| `scripts/pi-extension-load.test.sh` | The same path |
| The two sections of `plugins/linkuistics/PROVENANCE.md` | `plugins/writing/PROVENANCE.md` |

The kept paths keep each relative link between moved files true. They also
keep the `docs/...` names in the comments of `tc_hook.py` and the Pi
extension true. The four changed files are `README.md`, `Taskfile.yml`,
`.gitignore` and `plugins/writing/.claude-plugin/plugin.json`.

The manifest gains `"skills": ["./craft/"]`, a description sentence for the
craft skills and the keywords `technical-communication` and `walkthrough`.
The Taskfile gains `check:tc-checker`, `check:tc-hook-lint`, `check:tc-pi`
and the two `design:technical-communication` tasks. The task `check` runs
`check:plugin`, the three `check:tc-*` tasks, then `check:conformance`. The
`.gitignore` gains `__pycache__/`, so it no longer equals that of
RequirementsSkills. The README names the craft skills, both roots, each moved
document at its path, the checks, the tool requirements and the provenance.

### Byte changes of the writing craft

Each change below is against Grove's files at the source commit. The digest is
that of `docs/owner-skill-inventory.tsv`. A copy of the inventory's `digest()`
function gave each value. The pack values come from the public clone.

| Skill | Baseline digest and files | Grove at the source commit | Pack digest and files | Change |
| --- | --- | --- | --- | --- |
| `technical-communication` | `b639969cc4a1256a`, 20 | `960a0f38bdb0c2af`, 19 | `144a0cf6fb0b705b`, 19 | The baseline counts `scripts/__pycache__/ste-lint.cpython-315.pyc` of the owner's cache, which Git does not track. Grove's commit `175e5c7d259` changed the description of `.codex-plugin/plugin.json` after the cached commit `238c3a839ab4`. The pack changes that manifest again, as the next list states. No other file differs from the cache copy |
| `writing-code-walkthroughs` | `b1e4795388bcf3ab`, 1 | `b1e4795388bcf3ab`, 1 | `b1e4795388bcf3ab`, 1 | None |

Eight moved files differ from Grove's blobs. A comparison of the blob IDs of
all 1378 moved files found only these:

- `.codex-plugin/plugin.json` of `technical-communication`: `repository` is
  `https://github.com/Linkuistics/WritingSkills`. The description says that
  the skill reaches Codex "through Ravel, through Grove's plugins/install.sh
  --pack, or through another installation the user chooses".
- `docs/TECHNICAL-COMMUNICATION.md`: the link to the skill names
  `../plugins/writing/craft/technical-communication/SKILL.md`. The link to
  Grove's README is
  `https://github.com/Linkuistics/grove/blob/main/README.md#install-the-skill-plugins`.
  That URL returned HTTP status 200, and Grove's public `main` holds the
  heading.
- `docs/adr/behavioral-acceptance-is-closed-unmet.md`: the link to the
  provenance names `../../plugins/writing/PROVENANCE.md`.
- `scripts/pi-extension-load.test.sh`: the extension path is
  `plugins/writing/craft/technical-communication/harnesses/pi/technical-communication.ts`.
- The three `harness.sh` files under
  `docs/evaluations/writing-code-walkthroughs/enabled/`: the skill path is
  `plugins/writing/craft/writing-code-walkthroughs/SKILL.md`.
- `docs/evaluations/writing-code-walkthroughs/refinement-regression/template-test.sh`:
  the same skill path. It reads `plugins/writing/.claude-plugin/plugin.json`
  and asserts the name `writing`, where it asserted `linkuistics`.

The finished evaluation records keep their bytes. The SHA-256 of the sorted
digests of the other 1328 evaluation files was `a933741ae002…` in the export
of the source commit and in the pack commit. The export of the source commit
and the first copy also agreed on all 1332 files, `f1a2aa07514d…`. Three
evaluation records name `/private/tmp` paths of their runs as links, and those
bytes stay.

The Git modes equal Grove's for each moved file: 1371 have `100644` and 7
have `100755`. The three scripts of `technical-communication` keep `100644`
and Grove's blob IDs. The commit holds no `__pycache__`.

`plugins/writing/PROVENANCE.md` has its own title and two opening paragraphs.
They name the source commit and state that "this repository" in the moved
sections means Grove. In the section of `technical-communication`, the paths
of the checker and of its licence start with `craft/` where they started with
`skills/`. No other line of the two sections changed. The checker's licence
stays in `technical-communication/LICENSES/`.

No other moved text changed. The moved specification, user guide and decision
records still describe the delivery through Grove's `linkuistics` plugin and
the checks of Grove's `scripts/check.sh`. Those claims stay true until
`writing-craft-removal-k211`. The pack's README states that limit.

### Asset classification of the writing craft

Each Grove file that names a moving skill or a moving asset is in one class of
"The Grove assets of moving craft skills" in `docs/specs/pack-membership.md`.
The enumeration searched the whole tree with `--hidden` for both skill names,
`tc-hook`, `tc_hook`, `ste-lint`, `TECHNICAL-COMMUNICATION` and
`harness-hooks-a`. A second search took the path of each moved document.

| Files | Test | Disposition |
| --- | --- | --- |
| The specification, the user guide and the design views, with the two `design:technical-communication` tasks | 2 | Copied to WritingSkills. Grove keeps its copies until `writing-craft-removal-k211` |
| The decision records `the-off-file-is-the-only-switch`, `documents-are-gated-replies-are-measured`, `technical-communication-carries-no-standard-text` and `behavioral-acceptance-is-closed-unmet` | 2 | Copied to WritingSkills |
| `docs/technical-communication-publication-review.md` and `docs/evaluations/writing-code-walkthroughs/` | 2 | Copied to WritingSkills |
| `docs/research/harness-hooks-a.md` | 3 | Copied to WritingSkills. Its citers are `tc_hook.py`, the specification, two of the moved decision records and the provenance section, which all move. The only other citer is `pack-membership.md`, which decides the move |
| `scripts/pi-extension-load.test.sh`, the checker self-test and the `tc-hook` lint | 2 | Run by the pack's `check:tc-*` tasks. `scripts/check.sh` keeps them until the removal |
| `docs/research/walkthrough-method.md`, `walkthrough-method-a.md` and `walkthrough-method-b.md` | 1 | Stay. They are the research for Grove's `ordinal-fs-tree` book, and no moving file cites them |
| `docs/evaluations/editorial-pipeline-pilot/` | 3 | Stays, as the editorial move classified it |
| The book decision records `a-book-carries-no-asset` and `a-book-cannot-witness-its-own-corpus`, `docs/specs/walkthrough-books.md`, `docs/specs/grove-loop-book-structure.md` and the books under `docs/walkthroughs/` | 1 | Stay. They state Grove's own books |
| The citation registry of `plugin_fallback.rs`, the spine's `SKILL.md` and `references/grove.md` | 1 | Stay. They cite `technical-communication` by bare name in Grove's own files |
| `README.md`, `plugins/README.md`, `CONTEXT-MAP.md`, `plugins/CONTEXT.md`, `plugins/install.test.sh`, `scripts/check.sh`, `Taskfile.yml`, `.agents/plugins/marketplace.json`, `plugins/linkuistics/hooks/hooks.json` and `plugins/linkuistics/PROVENANCE.md` | 1 | Stay. Their links and their named reads of moved files change in `writing-craft-removal-k211` |
| `membership-hooks.mmd`, `owner-skill-delivery.md`, `pack-membership.md`, `external-pack-contract.md` and `CHANGELOG.md` | History and design | Stay |
| `docs/owner-skill-inventory.tsv`, `inventory.py`, `docs/ravel-caller-surface-inventory.tsv` and `docs/owner-skill-delivery-evidence.md` | 3 | Stay as the baseline evidence |

### Checks of the writing craft

The checks ran in a plain Git clone of the pack commit from the public remote.
`GROVE_REPOSITORY` was the public URL, with no override. `grove:checkout`
fetched `31ec9117c9da` from Grove's public remote. After the run, the clone
held only the ignored `.cache/` and
`plugins/writing/craft/technical-communication/scripts/__pycache__/`.

- `check:plugin` passed with two warnings, no `version` and no icon, on Claude
  Code 2.1.318. The validation covers the manifest and its `craft/` entry.
- `check:tc-checker` printed `selftest OK`, with Python 3.15.1.
- `check:tc-hook-lint` reported nothing for `tc-hook` and the Pi load check,
  with ShellCheck 0.11.0.
- `check:tc-pi` loaded the extension in Pi, with its start record and its two
  controls.
- The pack alone gave 23 single-source checks and 56 loaded-path checks. The
  aggregate gave 109 and 996. Both passed.

Each control failed once in a scratch copy of the pack, one mutation for each
run. The unmutated copy passed all three `check:tc-*` tasks first.

| Mutation | Task | Reading |
| --- | --- | --- |
| `DESCRIPTION_WORDS = 25` becomes `26` in `ste-lint.py` | `check:tc-checker` | An `AssertionError` at line 1479 of the self-test |
| `tc-hook` gains the line `echo $1` | `check:tc-hook-lint` | ShellCheck reported SC2086 and SC2317 |
| The extension listens to `session_begin` in place of `session_start` | `check:tc-pi` | "no start record from the extension" |
| `tc_hook.py` ignores the off file | `check:tc-pi` | "the extension wrote a record while the off file existed" |
| The control extension of the Pi check no longer throws | `check:tc-pi` | "Pi did not report a throwing extension" |

Each failed task exited non-zero and reported only its own failure.

The evaluation scripts ran in the pack working copy. `template-test.sh`
passed with the new paths and the name `writing`. The two `harness-test.sh`
files and `freeze-harness-test.sh` passed. Each harness resolves the skill at
its pack path. `task check` does not run these scripts, and Grove's
`scripts/check.sh` did not run them either.

`design:technical-communication` served the viewer and `layers.svg` with HTTP
status 200. `design:technical-communication:render` exported the seven views
with D2 0.9.0, in a scratch copy, so the committed SVG files keep their bytes.

The first run of the Pi check in the working copy compiled `ste-lint.py`
beside it, and jj recorded the bytecode file. The commit therefore adds
`__pycache__/` to `.gitignore`, and jj stopped tracking that file before the
description. Grove's `.gitignore` holds the same rule.

### Ravel resolution of the writing craft

`ravel skills resolve --json` resolved the public clone with the status
`resolved`, before the checks ran in it. The source record has the type
`local` and the digest above. Its two search directories are the `skills` and
`craft` roots of the snapshot. The `skills` root holds the four kinds and
`editorial-family`, and the `craft` root holds `technical-communication` and
`writing-code-walkthroughs`. The snapshot holds seven `SKILL.md` files and no
`__pycache__`.

`diff -r` found no difference between the snapshot and `plugins/writing` of
the clone. A resolution of the clean jj working copy gave
the same digest.

The instrument was the Ravel 2.0 build of `scripts/harness-trials/builds.json`.
`task install` installed it from the line `skill-directory-delivery` at
`71f70fcd3f92` into a scratch prefix. Its `bin/ravel` SHA-256 is
`26b487afb957…`, the pinned value, and it holds the build identity
`a4f55d04b2d1…` twice. Each resolution ran under a temporary `HOME` whose
settings held one `local` source with the roots `["skills", "craft"]` and an
empty mandatory list.

### Publication of the writing craft

- jj moved the bookmark `main` to the pack commit and pushed it to `origin`.
  The public `main` is `0eacee02f0f2594fb51b0f7dff53b0d7130133a5`.
- A plain clone of the local repository passed `task check` before the push.
- Grove's marketplace still pins the first pack commit `694140a18dc7`. No
  native user receives the craft skills from this commit.
- The owner's native plugins are unchanged. The always-on layer of this
  session comes from the owner's plugin cache at Grove commit `238c3a839ab4`.

### Tool requirements of the writing craft

| Skill or check | Tools |
| --- | --- |
| `technical-communication` | `python3`, version 3.9 or later, for `ste-lint.py` and `tc_hook.py`. A POSIX `/bin/sh` for `tc-hook`, which the hooks run as `/bin/sh tc-hook` |
| `writing-code-walkthroughs` | None |
| The pack's `task check` | Bash, Git, Task, Python 3 and ShellCheck. Claude Code for `check:plugin` and Pi for `check:tc-pi`. Each of those two skips when its tool is not installed |
| `design:technical-communication:render` | D2 |

The README of the pack holds the same table. The inventory's rows for
`technical-communication` name `python3`, `scripts/ste-lint.py` and the
always-on hooks.

### Hooks plugins of the writing pack

`writing-hooks-plugins-k209` added the two hooks plugins of
`technical-communication` to WritingSkills on 2026-11-05. This is the second
expand commit of `writing-craft-pack-k181`. Grove keeps its own hooks until
`writing-craft-removal-k211`.

| Subject | Value |
| --- | --- |
| Pack commit | `4cafb0fd8e72dacdeea9d960cbdaf9f4850aec8f`, a child of `0eacee02f0f2`. The jj bookmark `main` and the public `main` |
| Git tree of `plugins/writing` | `7348fd5d83f04f64a4734ada100157622ebbb550` |
| Claude Code hooks plugin | `technical-communication-hooks`. Root `plugins/writing/craft/technical-communication`. Manifest `.claude-plugin/plugin.json` with `"skills": []`. Hooks file `hooks/hooks.json`, which Claude Code reads by default |
| Codex hooks plugin | `technical-communication`, unchanged. The same root, manifest `.codex-plugin/plugin.json`, hooks file `harnesses/codex/hooks.json` |
| Codex marketplace | `.agents/plugins/marketplace.json` at the pack root, with the name `writing-hooks`. It lists the Codex plugin with a `local` source at `./plugins/writing/craft/technical-communication` |
| Dependency | The writing plugin's `dependencies` are `["grove", "technical-communication-hooks"]`. The writing plugin declares no hook |
| Hook count | Six declarations for each of the three installations: the writing plugin alone, the hooks plugin alone, and both. Each of the six hooks has one declaration |
| `GROVE_COMMIT` | Unchanged at `31ec9117c9da` |

The hooks file is Grove's `plugins/linkuistics/hooks/hooks.json` with one
change. Its script path loses `skills/technical-communication/`, because the
plugin root is now the skill directory. The Codex marketplace file is Grove's
`.agents/plugins/marketplace.json` with three changes. Its name is
`writing-hooks` in place of `linkuistics-hooks`, its `displayName` is
"Writing hooks", and its path names the skill directory in the pack.

#### The name of the Claude Code hooks plugin

The name is `technical-communication-hooks`, for these reasons:

- A Ravel user enables the plugin by name beside the ordinary pack plugins.
  The name states the skill whose hooks it runs and that it holds only hooks.
- The name differs from the skill name. A listing that shows the plugin
  `technical-communication-hooks` and the skill
  `writing:technical-communication` therefore cannot suggest a second copy of
  the skill.
- A plugin name is unique in one marketplace. The writing plugin and this
  plugin share Grove's marketplace, and the dependency names the plugin by
  its bare name there.

The Codex plugin keeps the name `technical-communication`. It lives in its own
marketplace, and this leaf changes no Codex identity.

#### The root SKILL.md

The Claude Code plugin reference states: "A plugin with `SKILL.md` at its
root, no `skills/`, and no `skills` key loads as a single skill"
(<https://code.claude.com/docs/en/plugins-reference>, "Standard layout").
The hooks plugin's root holds `SKILL.md`, so the layout of the owner design
would list a native `technical-communication` skill.

A listing in a scratch home gave the same result, on Claude Code 2.1.318. The home was
a new directory in the session scratchpad, set as `HOME` and
`CLAUDE_CONFIG_DIR`. A scratch marketplace listed two copies of the plugin
root, and `claude plugin details` read each installed copy without a model
request.

| Manifest | Skills | Hooks |
| --- | --- | --- |
| No `skills` key | 1, `technical-communication` | 6 |
| `"skills": []` | 0 | 6 |

The pack therefore has `"skills": []` in the manifest. The empty list adds no
directory to the default `skills/` scan, and the key turns off the single-skill
rule. [Writing hooks trial](#writing-hooks-trial) holds the reading in a real
session.

#### The installation of the writing plugin alone

A second scratch home installed the writing plugin alone from a scratch
marketplace. The marketplace listed `writing`, `technical-communication-hooks`
and Grove's `grove` plugin from the checkout of `31ec9117c9da`. The command
`claude plugin install writing@scratch` reported "+ 2 dependencies: grove,
technical-communication-hooks".

| Plugin | Skills | Hooks |
| --- | --- | --- |
| `writing` | 7, among them `technical-communication` from `craft/` | 0 |
| `grove` | 6 | 0 |
| `technical-communication-hooks` | 0 | 6 |

No command installed, updated or removed a plugin in the owner's home.

#### Codex

Codex 0.166.0 accepted the marketplace in a scratch `CODEX_HOME`.
`codex plugin list` showed `technical-communication@writing-hooks` at the
skill directory, and `codex plugin add` installed it into the scratch cache.

The plugin root now also holds the Claude Code hooks file `hooks/hooks.json`.
The Codex source shows that Codex ignores it:

- `DISCOVERABLE_PLUGIN_MANIFEST_PATHS` in
  `codex-rs/exec-server-protocol/src/protocol.rs` puts
  `.codex-plugin/plugin.json` before `.claude-plugin/plugin.json`.
- `load_plugin_hooks` in `codex-rs/core-plugins/src/loader.rs` reads the
  default `hooks/hooks.json` only when the manifest has no `hooks` field.

The source was the tag `rust-v0.166.0` of `openai/codex`. The Codex manifest
names `./harnesses/codex/hooks.json`, so Codex loads only that file. The pack
check makes that field a rule.

#### Checks of the hooks plugins

The pack's new task `check:hooks` runs `scripts/hook-count.py`. For each
plugin, the script reads the default `hooks/hooks.json`, the manifest's
`hooks` field and the dependencies. It reads the `grove` plugin from the Grove
checkout of `GROVE_COMMIT`. Each installation must give one declaration for
each of the six hooks. The script also checks four layout rules:

- The hooks plugin has no `skills/` directory and lists `"skills": []`.
- The writing plugin declares no hook.
- The writing plugin's dependencies name the hooks plugin.
- The Codex manifest names `./harnesses/codex/hooks.json`.

Its controls mutate a scratch copy of `plugins/`, one mutation for each run.
Each control names a fragment of the error that it must cause.

| Mutation | Reading |
| --- | --- |
| The writing manifest declares an inline Stop hook | `Stop * claude-code reply: 2` for the writing plugin alone |
| The writing plugin gains a default `hooks/hooks.json` | Each of the six hooks counts 2 |
| The Stop hook gains a second handler | `Stop * claude-code reply: 2` |
| The hooks file loses `SessionStart` | `SessionStart * claude-code session-start: 0` |
| The writing plugin drops the dependency | Each of the six hooks counts 0 for the writing plugin alone |
| `scripts/tc-hook` is absent | The declaration runs a script that does not exist |
| The hooks manifest drops its `skills` key | The root `SKILL.md` rule |
| The Codex manifest drops its `hooks` field | The Codex rule |

The first run of the controls had two cases that failed on a layout rule
before the count ran. The script now counts first, and the manifest control
declares a hook whose script exists. The count itself therefore catches each
second declaration.

`check:plugin` now runs `claude plugin validate` for both plugins. Each
passed with two warnings, no `version` and no icon.

`task check` passed in a plain clone of the local repository before the push.
It passed again in a clean clone of the public remote after the push, with
the public `GROVE_REPOSITORY` and no override. Both runs gave the six
declarations and the eight failed controls. They also gave the earlier
results. These are `selftest OK`, the Pi load check, 23 and 56 checks for the
pack alone, and 109 and 996 for the aggregate. After the public run, the clone held only the
ignored `.cache/` and `__pycache__/`.

#### For the later leaves

- `writing-hooks-trial-k210` read the hooks in real sessions from pack commit
  `4cafb0fd8e72`, as [Writing hooks trial](#writing-hooks-trial) records. The
  scratch listings above show the inventory only, and no hook ran in them.
- `writing-craft-removal-k211` lists `technical-communication-hooks` in
  Grove's `.claude-plugin/marketplace.json` at the pinned pack commit, with
  the source path `plugins/writing/craft/technical-communication`. Until then,
  the writing plugin's dependency resolves in no published marketplace. The
  current pin `694140a18dc7` predates the dependency, so native users see no
  failure.
- The pack's specification, user guide and decision records still describe
  the delivery through Grove's `linkuistics` plugin. This leaf did not change
  them, and the removal leaf changes them.

### Writing hooks trial

`writing-hooks-trial-k210` read the native residue in real Claude Code
sessions on 2026-11-05. The
[pack membership specification](specs/pack-membership.md#the-acceptance-of-each-move)
asks for this trial in its row "A second plugin in one repository". Every
reading passed at pack commit `4cafb0fd8e72`. The pack needed no repair, so
`writing-craft-removal-k211` pins that commit.

#### Subjects of the writing hooks trial

| Subject | Value |
| --- | --- |
| Claude Code | 2.1.318, model `claude-sonnet-5-5` |
| Pack commit | `4cafb0fd8e72dacdeea9d960cbdaf9f4850aec8f`, the public `main` of WritingSkills |
| Grove commit of the `grove` dependency | `31ec9117c9da`, the `GROVE_COMMIT` of the pack, on the public line `gh-issue-25` |
| Marketplace | A scratch marketplace named `scratch`. It lists three `git-subdir` sources from GitHub: `writing` and `technical-communication-hooks` at the pack commit, and `grove` at the Grove commit |
| Homes | A new directory in the session scratchpad for each installation. It links only `Library/Keychains` to the owner's home |
| Environment | `env -i` with `HOME`, `USER`, `LOGNAME`, `SHELL`, `TERM`, `LANG`, `TMPDIR` in the home, and a system `PATH`. No variable names a Claude Code location or the state directory of the hooks |
| Session flags | `-p`, `--output-format stream-json`, `--verbose`, `--include-hook-events`, `--permission-mode acceptEdits`, `--tools Agent,Write`, `--add-dir` of the gate directory |
| Gate directory | `target/hooks-trial/<run>/` in this Grove workspace. The gate skips the temporary roots, and the scratchpad is under `/private/tmp` |

The prompt asks for three steps. The agent runs one foreground subagent that
uses no tool. It then writes `note.md` in the gate directory with the line
"Restart the worker; then read the log.", which has one semicolon. Last, it
says whether a SessionStart hook gave it context and whether the write result
carried a check message.

The auto-mode classifier of the session that ran the trial refused a nested
session in `bypassPermissions`. The flags above are therefore narrower than
those of the earlier native trials.

#### Readings of the hooks plugin alone

The home installed `technical-communication-hooks@scratch` and nothing else.
The installation is `4cafb0fd8e72-7563b82a`. Its cached `hooks/hooks.json`
has the SHA-256 `8d1f0b8e3469…`, which equals the file of the pack commit.
The cache root holds `SKILL.md`.

| Reading | Result |
| --- | --- |
| `claude plugin details` | Skills 0. Hooks 6: SessionStart, SubagentStart, UserPromptSubmit, PostToolUse, Stop and SubagentStop |
| The session's init event | The plugin `technical-communication-hooks`. Its 32 skills hold no name with `technical-communication` |
| Hook starts in the stream | One for each of the six events |
| SessionStart and SubagentStart | Each gave the text of `always-on/instruction.md` as `additionalContext` |
| UserPromptSubmit | Gave the text of `always-on/reminder.md` |
| PostToolUse | Gave the gate message for `note.md`: 1 hard finding, `1:19 semicolon` |
| Stop and SubagentStop | No output, as the hook core intends. The log holds the records below |
| The reply log | A start record with the source `startup`, a start record and a reply record for the subagent, and a reply record for the main agent |
| The agent's own reply | It quoted the instruction's first words, and it named the semicolon finding |

The session is `8de967de-6ef1-4e79-b1f4-9b0d1dfc712c`. Its stream has the
SHA-256 `01bb42a9fd85…`, and its session file `3beda538630a…`.

**Result: pass.** Claude Code runs the six hooks from the skill-free plugin
and lists no native `technical-communication` skill. The root `SKILL.md`
stays unlisted because the manifest has `"skills": []`.

#### Readings of the writing plugin alone

A new home installed `writing@scratch` and nothing else. Claude Code reported
"(+ 2 dependencies: grove, technical-communication-hooks)".

| Plugin | Installation | Skills | Hooks |
| --- | --- | --- | --- |
| `writing` | `4cafb0fd8e72-bb90ff2c` | 7, among them `technical-communication` | 0 |
| `grove` | `31ec9117c9da-f02c8f28` | 6 | 0 |
| `technical-communication-hooks` | `4cafb0fd8e72-7563b82a` | 0 | 6 |

The cached `writing` plugin has no `hooks/` directory. Its
`craft/technical-communication/SKILL.md` has the SHA-256 `06f94945e3b3…`,
which equals the file of the pack commit.

The init event lists 45 skills. Exactly one name holds
`technical-communication`: `writing:technical-communication`. The stream has
one hook start for each of the six events, and the same four context outputs
as the hooks plugin alone. The gate message is the same. The reply log holds
the same four records.

The session is `9e02ed6f-9e4b-4529-8e43-8ab5169f9010`. Its stream has the
SHA-256 `14d2a0e60d0a…`, and its session file `f5b41fb816b9…`.

**Result: pass.** The writing plugin alone installs the hooks plugin as its
dependency, and each hook runs once. The only `technical-communication` skill
comes from `craft/` of the writing plugin.

#### Readings with the off file

The control reused the home of the writing plugin. It added an empty file
`.local/state/technical-communication/off` and ran the same prompt. The off
file is therefore the only difference from the run before.

| Reading | Result |
| --- | --- |
| Hook starts in the stream | One for each of the six events. Claude Code still runs each declaration |
| Hook outputs | None of the six gave `additionalContext` |
| The agent's own reply | No SessionStart context, and no check message on the write |
| The reply log | No file for this session. The earlier session's file is unchanged |

The session is `59bbcab1-6316-4e8a-a00a-6183abe8d17b`. Its stream has the
SHA-256 `0a090b9d79b2…`, and its session file `d463eab27a6c…`.

**Result: pass.** The off file turns off every hook effect. The hook effects
of the two runs before therefore come from the hooks.

#### Repairs of the trial setup

The pack needed no repair. Four attempts at the run of the hooks plugin alone
failed in the setup of the trial, before the accepted run:

| Attempt | Fault | Repair |
| --- | --- | --- |
| 1 | The gate directory was outside the session's working directories, so Claude Code refused the write | `--add-dir` of the gate directory |
| 2 | The `Write(...)` allow rule did not match, so Claude Code refused the write again | `acceptEdits` in place of the allow rule |
| 3 | The subagent ran in the background. Its notice started a second turn, so the prompt and stop hooks ran twice | The prompt asks for `run_in_background false` |
| 4 | The gate file of attempt 3 remained, and `Write` refused to replace a file it had not read | The script removes the gate file before each run |

Five hooks ran in attempts 1 and 2, each once, with the expected effects. All
six ran in attempt 3, with the expected effects. These attempts do not change
the results above.

#### The owner's installation in the writing hooks trial

Before the first attempt and after the off-file run, the owner's `~/.claude`
had the same SHA-256 for `settings.json` and `plugins/installed_plugins.json`.
A listing of `plugins/cache` and `skills` to depth 3 had the same SHA-256.
The state directory of the owner's hooks held no off file before or after.

**Observation: the owner's `plugins/known_marketplaces.json` changed.** The
`lastUpdated` values of `claude-plugins-official` and `linkuistics` became
10:47:12Z and 10:47:17Z. No `scratch` entry appeared, and the `linkuistics`
checkout was not fetched. The times fall between two trial sessions, during
the setup of attempt 2.

A probe then ran the same marketplace add, installation, listing and details
commands in two new homes, with no model request. The owner's file was
byte-identical before and after the probe. The later trial sessions also left
it unchanged. Disposition: the trial did not cause the change. The likely
writer is the owner's own running Claude Code, which refreshes its
marketplaces. The probe cannot prove that writer.

#### Limits of the writing hooks trial

- **One accepted run for each reading.** The hooks run without a model
  decision, so a repeat adds no evidence about the pack.
- **Claude Code only.** The membership decision asks for no Codex trial. The
  Codex hooks plugin is absent from the owner's setup.
- **A scratch marketplace.** Grove's published marketplace does not yet list
  `technical-communication-hooks`. `writing-craft-removal-k211` adds it, and
  `discovery-rollout-k6` records the owner's installation.

### Removal of the writing craft

`writing-craft-removal-k211` removed Grove's copies of the writing craft in one
change with every Grove consumer, on 2026-11-05. This is the contraction step
of `writing-craft-pack-k181`.

| Subject | Value |
| --- | --- |
| Removal commit | `4cfac62958815219ba5a71941674244a05ff9b56` |
| Pushed Grove commit | `4cfac6295881`. jj moved the remote bookmark `gh-issue-25` forward from `31ec9117c9da` before the last pack commit's publication |
| Pinned pack commit | `4cafb0fd8e72dacdeea9d960cbdaf9f4850aec8f`, the commit that the hooks trial accepted. Grove's marketplace pins it for `writing` and for `technical-communication-hooks` |
| Last pack commit | `9f7d516be7d9a06611ec0f7dca8bebe033724ed4`, a child of the pinned commit. The public `main`. Its `GROVE_COMMIT` is the removal commit |
| Source directory and roots | `plugins/writing`, with the Ravel roots `["skills", "craft"]` |
| Tree of `plugins/writing` | `7348fd5d83f04f64a4734ada100157622ebbb550` in both pack commits |
| `resourceDigest` | `21463d062a62ccfa744d327b62eb3afa5ede381553b5877bfaaf95a9cd9a49bd` for both commits, from a clean clone of the public remote |
| Claude Code hooks plugin | `technical-communication-hooks`, at `plugins/writing/craft/technical-communication` of the pinned commit. It holds no skill |
| Codex hooks plugin | `technical-communication` in the marketplace `writing-hooks`, at the pack root |

The removal took out 1380 tracked files. They are the 1378 moved files of
[Source of the writing craft](#source-of-the-writing-craft),
`plugins/linkuistics/hooks/hooks.json` and `.agents/plugins/marketplace.json`.
The only untracked file in those trees was an ignored `__pycache__` of
`ste-lint.py`, which went with its directory. These consumers changed:

- `plugins/linkuistics`: the manifest description and keywords drop the
  walkthrough and writing subjects. `PROVENANCE.md` loses the two moved
  sections and names where they went.
- `scripts/check.sh` drops the checker self-test, the Pi load check and the
  shell lint of `tc-hook` and of the Pi load check. `Taskfile.yml` drops the
  two `design:technical-communication` tasks.
- `plugins/install.test.sh` replaces its read of the real Pi extension with a
  fixture. A pack's `craft/` skill ships `harnesses/pi/crafted.ts`, and
  `--pack` must link it to its path in `craft/`.
- `.claude-plugin/marketplace.json` pins `writing` at the accepted pack commit
  and lists `technical-communication-hooks` from the same commit. The writing
  plugin's dependency resolves only in a marketplace that lists the hooks
  plugin. The descriptions of the marketplace, of `writing` and of
  `linkuistics` changed, and the marketplace counts nine plugins.
- `scripts/check-packs.sh` and the directory-only paragraph of
  `pack-membership.md` name the hooks-only source. `task check:packs` gives it
  the directory-only check, because it has no kind skill and no manifest.
- `plugins/README.md` holds the craft table and the always-on paragraph in its
  `writing` section, and it names the hooks plugin among the pinned plugins.
  `plugins/CONTEXT.md`, `CONTEXT-MAP.md`, `README.md`, `docs/USAGE.md` and
  `docs/ARCHITECTURE.md` state where the skills, the hooks and the moved
  records live. The `writing` source of the directory route gains the root
  `craft`.
- Three files that stay linked a moved record. `pack-membership.md`,
  `owner-skill-delivery.md` and the editorial pilot's `preregistration.md` now
  link its public URL in WritingSkills.
- Three files cited `linkuistics:writing-code-walkthroughs`, and they now cite
  the bare name. The Codex marketplace item of `external-pack-contract.md` says
  that Grove has none.

The citation registry of `plugin_fallback.rs` keeps its two
`technical-communication` rows, because they read Grove's own files. No Grove
record states the off-file rule as its own. Each Grove mention cites the
record in WritingSkills.

The last pack commit changes no file under `plugins/writing`. It sets
`GROVE_COMMIT` and repairs the moved documents that described the delivery
through Grove's `linkuistics` plugin and the checks of Grove's
`scripts/check.sh`. These are the specification, the user guide,
`the-off-file-is-the-only-switch` and
`technical-communication-carries-no-standard-text`. The pack README names
these repairs. The publication review, the harness survey and the
evaluation records keep their bytes as dated records.

### Readings of the removal of the writing craft

| Reading | Single-source checks | Loaded-path checks | Unpinned rows |
| --- | --- | --- | --- |
| Core alone before the removal, at `88be3ee7198` | 86 | 480 | 51 |
| Core alone after the removal | 86 | 480 | 51 |
| The pack alone | 23 | 56 | 0 |
| The aggregate of core and the pinned pack commit | 109 | 996 | 51 |

Conformance reads no file of `plugins/linkuistics` and no `craft/` root, so
the removal leaves every figure unchanged. The other aggregates are still
requirements 87 and 842, design 91 and 839, engineering 88 and 1188, and
research 86 and 839.

A link check read every tracked and untracked Markdown file. It found no
relative link that the removal broke.

The stale-location search ran `rg --hidden` over the whole tree, apart from
`.jj`, `.git`, `target`, `.grove` and `.cache`. It enumerated both skill
names, `tc-hook`, `tc_hook`, `ste-lint`, `pi-extension`, `.agents/plugins` and
the path of each moved record. It then searched for each moved path, the two
names with the `linkuistics:` prefix, `linkuistics-hooks` and each relative
link into a moved record. Each hit is in a class of the next section.

A planted path in a hidden scratch directory was found, and a misspelt pattern
found nothing. The same search over WritingSkills found the moved documents.
The hits there that name Grove's old paths are provenance or dated records.

### Asset classification of the removal of the writing craft

Each Grove file that still names a moved skill or a moved record is in one
class of "The Grove assets of moving craft skills" in
`docs/specs/pack-membership.md`.

| Files | Test | Disposition |
| --- | --- | --- |
| The moved specification, user guide, design views, four decision records, publication review, harness survey, evaluations and Pi load check | 2 or 3 | Moved to WritingSkills, as [Asset classification of the writing craft](#asset-classification-of-the-writing-craft) states. Grove holds no copy |
| `plugins/linkuistics/hooks/hooks.json` and `.agents/plugins/marketplace.json` | 2 | Removed. Their successors are the hooks plugins in WritingSkills |
| The citation registry of `plugin_fallback.rs`, the spine's `SKILL.md` and `references/grove.md` | 1 | Stay. They cite `technical-communication` by bare name in Grove's own files |
| `walkthrough-books.md`, `grove-loop-book-structure.md`, `a-book-cannot-witness-its-own-corpus.md`, `a-book-carries-no-asset.md` and `docs/research/walkthrough-method.md` | 1 | Stay. They state or research Grove's own books, and they cite the walkthrough skill by bare name |
| `CONTEXT-MAP.md`, `pack-membership.md`, `owner-skill-delivery.md` and the editorial pilot's `preregistration.md` | 1 | Stay. They link a moved record by its public URL |
| `README.md`, `plugins/README.md`, `plugins/CONTEXT.md`, `docs/USAGE.md`, `docs/ARCHITECTURE.md`, `.claude-plugin/marketplace.json`, `scripts/check-packs.sh` and `plugins/install.test.sh` | 1 | Stay. They state where the skills and the hooks ship, or they hold a fixture of that shape |
| `membership-hooks.mmd`, `external-pack-contract.md`, `CHANGELOG.md` and this record | History and design | Stay |
| `docs/owner-skill-inventory.tsv`, `inventory.py`, `docs/ravel-caller-surface-inventory.tsv` and `docs/owner-skill-delivery-evidence.md` | 3 | Stay as the baseline evidence |

### Checks of the removal of the writing craft

- `bash scripts/check.sh` passed all 19 principal checks on the removal, with
  1344 Rust tests. The two fewer checks are the checker self-test and the Pi
  load check, which the pack's `check:tc-*` tasks now run. `GROVE_TEST_RAVEL`
  resolved to the installed Ravel 1.1.0,
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. The changed files had the same
  digests before and after the run.
- `task check:packs` passed `requirements`, `design`, `engineering`,
  `research` and `writing`, alone and in the aggregate. `testanyware` and
  `technical-communication-hooks` passed the directory-only check.
- `plugins/install.test.sh` gave 48 passed. A copy whose pack fixture shipped
  `other.ts` failed the new case alone.
- `claude plugin validate .` and `claude plugin validate plugins/linkuistics`
  passed with the `version` warnings, on Claude Code 2.1.318.
- The last pack commit's full `task check` passed from a clean plain-Git clone
  of the public remote, with the committed public `GROVE_REPOSITORY` and no
  override. `grove:checkout` fetched `4cfac6295881` from GitHub. The hook
  count gave six declarations for each of the three installations, and the
  eight controls failed as required. The checker printed `selftest OK`, and
  the Pi load check passed with its controls. The pack alone gave 23 and 56,
  and the aggregate gave 109 and 996. After the run, the clone held only the
  ignored `.cache/` and `__pycache__/`. A plain clone of the local repository
  passed the same `task check` before the push.
- `ravel skills resolve --json` resolved a new clone of the public remote at
  `9f7d516be7d9`, before any check ran in it, with the status `resolved`. Its
  two search directories are the `skills` and `craft` roots of the snapshot.
  The snapshot holds seven `SKILL.md` files and no `__pycache__`, and `diff -r`
  found no difference from `plugins/writing`. The same clone at `4cafb0fd8e72`
  gave the same digest. The instrument was the Ravel 2.0 build of
  `scripts/harness-trials/builds.json`, `bin/ravel` SHA-256 `26b487afb957…`,
  with its build identity. The temporary `HOME` held one `local` source with
  the roots `["skills", "craft"]` and an empty mandatory list.
- An isolated home held Codex, Gemini CLI and Pi directories. In it,
  `install.sh --force --pack <public clone>/plugins/writing` linked the five
  skills of `skills/` and the two craft skills of `craft/` into each harness.
  It linked `technical-communication.ts` into `~/.pi/agent/extensions/`, with
  its target in `craft/` of the clone. The run made 48 skill links and one
  extension link, and it skipped `guardrail` three times. The owner's real
  skill and extension links had the same digest before and after.
- `codex plugin marketplace add Linkuistics/WritingSkills` added the
  marketplace `writing-hooks` in a scratch `CODEX_HOME`, on Codex 0.166.0.
  `codex plugin list` showed `technical-communication@writing-hooks`.

### Tool requirements after the removal of the writing craft

The tool requirements of
[Tool requirements of the writing craft](#tool-requirements-of-the-writing-craft)
hold unchanged. `technical-communication` and both hooks plugins need
`python3`, version 3.9 or later, for `ste-lint.py` and `tc_hook.py`. They also
need a POSIX `/bin/sh` for `tc-hook`, which each hook runs as
`/bin/sh scripts/tc-hook`. Grove's own checks no longer need Pi for this skill.

### For skill-marketplace-k185

The Codex plugin stays listed in the pack root. Codex 0.166.0 resolves a
`local` source only inside the root of its marketplace. In a scratch
`CODEX_HOME`, a marketplace whose source path was
`../WritingSkills/plugins/writing/craft/technical-communication` was added,
and `codex plugin list` printed "No marketplace plugins found". The same
marketplace with a copy of the plugin at `./plugins/tc` listed
`technical-communication@inside-test`. A marketplace-only repository therefore
cannot list the Codex hooks plugin through a `local` source. This reading
covers `local` sources only, and no other Codex source type was tried.

## PersonalSkills

`personal-pack-repository-k212` created PersonalSkills on 2026-11-05. This is
the expand commit of `personal-pack-k183`. Grove keeps its `linkuistics`
plugin, its copies of the four skills and its marketplace entry until
`personal-removal-k214`. The pack holds no kind skill, so no Grove check reads
it at this commit.

### Source of the personal pack

| Subject | Value |
| --- | --- |
| Repository | `~/Development/AgenticOrchestration/PersonalSkills`, colocated Git and Jujutsu. Remote `origin` is `https://github.com/Linkuistics/PersonalSkills.git` |
| Pack commit | `ac98636608b013391bf4b4ca0dc6c9c1506ff9d0`, a root commit. The jj bookmark `main` and the public `main` |
| Source directory | `plugins/personal`. It is the root of the Claude Code plugin `personal` and the Ravel source directory |
| Ravel roots | `["craft"]`. `craft/` holds `machine-environment`, `using-jujutsu` and `authoring-conventions`. The source directory has no `skills/` |
| Plugin manifest of `personal` | `"skills": ["./craft/"]` and `"dependencies": ["guardrail"]` |
| `guardrail` plugin | Root `plugins/guardrail`, with `skills/guardrail/` and its two scripts. No Ravel root reaches it |
| Git tree of `plugins/personal` | `23b39220f9ae5b974d54d71ae9e8435b7e206a64` |
| Git tree of `plugins/guardrail` | `a5fb16931c82362a388818cd3bbb1d5f0d5bd53f` |
| `resourceDigest` of `plugins/personal` | `4cb21f4d5b3b00917093e93da9a26ea530b3abd42f2b9e01a65a10725fd11167`, from a plain Git clone of the public remote and from the jj working copy |
| Source Grove commit | `4cfac62958815219ba5a71941674244a05ff9b56`, the head of Grove's remote bookmark `gh-issue-25`. No moving file changed between it and `0eacec3a47b2`, the parent of this leaf's Grove change |
| `GROVE_COMMIT` | None. The pack's checks read no Grove file |

The commit holds 16 files. Eight are the tracked moving files of Grove. Each
keeps its Grove path below the skill directory:

| Grove path | Pack path |
| --- | --- |
| `plugins/linkuistics/skills/machine-environment/SKILL.md` | `plugins/personal/craft/machine-environment/SKILL.md` |
| `plugins/linkuistics/skills/using-jujutsu/`, `SKILL.md` and two files in `references/` | `plugins/personal/craft/using-jujutsu/` |
| `plugins/linkuistics/skills/authoring-conventions/SKILL.md` | `plugins/personal/craft/authoring-conventions/SKILL.md` |
| `plugins/linkuistics/skills/guardrail/`, `SKILL.md` and two scripts | `plugins/guardrail/skills/guardrail/` |
| The authoring table and the section of `using-jujutsu` in `plugins/linkuistics/PROVENANCE.md` | `plugins/personal/PROVENANCE.md` |
| The section of `guardrail` in `plugins/linkuistics/PROVENANCE.md` | `plugins/guardrail/PROVENANCE.md` |

The other files are the two manifests, the two provenance files, `LICENSE`,
`README.md`, `Taskfile.yml` and `scripts/check-frontmatter.sh`. The licence is
Apache-2.0, a copy of the DesignSkills file. The pack has no `.gitignore`,
because no check writes a file.

### Plugin names and the dependency of the personal pack

- `personal` equals the Ravel source ID of `docs/specs/owner-skill-delivery.md`.
  One name then identifies the pack on both routes, as for `research`,
  `design` and `writing`.
- `guardrail` is the name that the native residue of that design gives the
  plugin. The plugin holds only the skill of that name, so the native
  spelling is `guardrail:guardrail`.
- The `personal` plugin declares `guardrail` in its `dependencies`. A native
  user of `linkuistics@linkuistics` receives `guardrail` today, and the
  dependency keeps that for a native user of `personal`. The expand found no
  reason against it. `guardrail` is user-invoked, so it costs no standing
  context. The README records the reason.
- A bare dependency name resolves in the marketplace that lists the plugin.
  Grove's marketplace lists neither plugin at this commit.
  `personal-removal-k214` adds both entries.

### Byte changes of the personal pack

The digest is that of `docs/owner-skill-inventory.tsv`. A copy of the
inventory's `digest()` function gave each value. The pack values come from the
public clone.

| Skill | Baseline digest and files | Grove at the source commit | Pack digest and files | Change |
| --- | --- | --- | --- | --- |
| `machine-environment` | `2316befd39ccda08`, 1 | `2316befd39ccda08`, 1 | `2316befd39ccda08`, 1 | None |
| `using-jujutsu` | `f65e8aad7016cc9e`, 3 | `f65e8aad7016cc9e`, 3 | `6ede7529e799cba1`, 3 | Line 138 of `SKILL.md` names `using-git-worktrees`, where it named `superpowers:using-git-worktrees` |
| `authoring-conventions` | `b0c2d7f9d8f044e2`, 1 | `b0c2d7f9d8f044e2`, 1 | `b0c2d7f9d8f044e2`, 1 | None |
| `guardrail` | `36371cf57051b6ff`, 3 | `36371cf57051b6ff`, 3 | `36371cf57051b6ff`, 3 | None |

The baseline values are those of the owner's Claude Code cache at Grove
commit `238c3a839ab4`. The Codex rows of the inventory give
`f1f7ecb6073f5668` for `machine-environment`. That value is the copy of the
Grove 22.8 provisioning snapshot, which is older than Grove's file. The other
Codex rows equal the Claude Code rows.

The change in `using-jujutsu` applies rule 2 of "References between skills" in
`docs/specs/external-pack-contract.md`. A sweep for code spans of the form
`<word>:<word>` found only that name in the moving files. The same command
found such a name in `plugins/grove/skills`, so the pattern works.

The blob IDs of the other seven moved files equal Grove's. The Git modes
equal Grove's: the two guardrail scripts have `100755`, and the other moved
files have `100644`. The guardrail hook in the frontmatter names
`${CLAUDE_PLUGIN_ROOT}/skills/guardrail/scripts/guardrail-hook.sh`, and the
plugin root holds that path, so the hook keeps its bytes.
`authoring-conventions` and `guardrail` keep
`disable-model-invocation: true`.

Each provenance file has its own title and opening paragraph. They name the
source commit, and they state which sections the file holds. The moved
sections keep their bytes, inside a `tc: off` region that states the reason.
The paragraph of Grove's file about sections that moved to other packs stays
in Grove. Each skill links to `../../PROVENANCE.md`, which resolves to the
file of its own plugin.

### The wording of authoring-conventions

`authoring-conventions` says "this marketplace" and "this repo". In Grove
those words named Grove's marketplace and repository. The words stay, and
the README of the pack records the reading:

- "This marketplace" names the Linkuistics marketplace, which lists the pack
  after `personal-removal-k214`. If `skill-marketplace-decision-k201` removes
  that marketplace, the phrase loses its referent.
- "This repo" names the repository whose skill the author edits. The skill
  does not apply to one repository only.
- The claims about `install.sh` and `install.test.sh` describe Grove's
  installer, which also installs the pack. The pack's `check:frontmatter`
  holds the check of `harnesses:` for its own skills.

The move does not repair two claims that were already stale in Grove. The
skill counts "fourteen of our sixteen shipped skills", and it names "this
repo's `claude-api` skill", which Grove does not ship.

### Asset classification of the personal pack

The enumeration searched the whole Grove tree with `rg --hidden` for the four
skill names, `plugins/linkuistics` and `linkuistics/PROVENANCE`. It excluded
`.git`, `.jj` and `target`, and it found 112 files. Each file is in one class
of "The Grove assets of moving craft skills" in `docs/specs/pack-membership.md`.

| Files | Test | Disposition |
| --- | --- | --- |
| The eight moving files and the two moved provenance sections | 2 | Copied to PersonalSkills. Grove keeps its copies until `personal-removal-k214` |
| `plugins/linkuistics/.claude-plugin/plugin.json` and the rest of `plugins/linkuistics/PROVENANCE.md` | 1 | Stay. `personal-removal-k214` removes them |
| The citation rows of `crates/grove/tests/plugin_fallback.rs` and the assertion of `commit_guidance.rs` | 1 | Stay. They read Grove's spine files under `plugins/grove/skills`, not a moving skill |
| `crates/grove/tests/native_skill_delivery.rs` | 1 | Stays. It plants a dangling link named `machine-environment` in a scratch home |
| `plugins/install.sh` and `plugins/install.test.sh` | 1 | Stay. Their comments name `guardrail`, and their fixtures use a plugin directory named `linkuistics`. The test of `harnesses:` reads Grove's bundled skills, and the pack's `check:frontmatter` covers the moved skills |
| The spine files `references/execute.md`, `references/commit.md` and `references/grove.md` | 1 | Stay. They cite `machine-environment` and `using-jujutsu` by bare name |
| `scripts/harness-trials/cases/discovery/case.json`, `problem-user-invoked/case.json` and the other trial cases | 1 | Stay. Two cases read `plugins/linkuistics` from a Grove commit. "The trial cases that name moving skills" of the membership design governs them in `personal-removal-k214` |
| `.claude-plugin/marketplace.json`, `README.md`, `plugins/README.md`, `plugins/CONTEXT.md`, `CONTEXT-MAP.md` and `docs/ARCHITECTURE.md` | 1 | Stay. `personal-removal-k214` changes their entries and links |
| `CHANGELOG.md`, the specifications, the design views and the earlier records | History and design | Stay |
| `docs/owner-skill-inventory.tsv`, `inventory.py`, `docs/ravel-caller-surface-inventory.tsv`, `docs/owner-skill-delivery-evidence.md` and the evaluation records | 3 | Stay as the baseline evidence |

No Grove test reads the bytes of a moving skill, as planning found. The pack
therefore needs no `GROVE_COMMIT` and no Grove checkout.

### Checks of the personal pack

The checks ran in a plain Git clone of the pack commit from the public remote.
After the run, the clone held no untracked or ignored file.

- `check:plugin` passed for both plugins with one warning each, no `version`,
  on Claude Code 2.1.318. The plugin-directory profile also named no icon.
  It took the README of the repository root for each plugin folder.
- `check:frontmatter` printed `4 skill(s) declare harnesses:, and 2
  user-invoked skill(s) keep disable-model-invocation: true`.
- `check:guardrail-test` printed `37 passed, 0 failed`.
- `check:shellcheck` reported nothing for the two guardrail scripts and
  `check-frontmatter.sh`, with ShellCheck 0.11.0.

Grove's `scripts/check.sh` ran neither guardrail check. The pack's
`check:frontmatter` replaces the test of `install.test.sh` that every bundled
skill declares `harnesses:`. It also checks the two user-invoked skills.

Each control failed once in a scratch copy of the pack, one mutation for each
run. The unmutated copy passed `check:frontmatter`, `check:guardrail-test` and
`check:shellcheck` first.

| Mutation | Task | Reading |
| --- | --- | --- |
| The mutation removes the `rm -rf` entry of the `DANGER` table | `check:guardrail-test` | Three `FAIL` lines, then `34 passed, 3 failed` |
| `guardrail-hook.sh` gains the line `echo $1` | `check:shellcheck` | SC2086 and SC2317 |
| `guardrail-hook.test.sh` gains the line `echo $1` | `check:shellcheck` | SC2086 |
| `check-frontmatter.sh` gains the line `echo $1` | `check:shellcheck` | SC2086 |
| `machine-environment` loses `harnesses:` | `check:frontmatter` | "declares no harnesses:" |
| `guardrail` loses `disable-model-invocation` | `check:frontmatter` | "guardrail lacks disable-model-invocation: true" |
| `authoring-conventions` sets `disable-model-invocation: false` | `check:frontmatter` | "authoring-conventions lacks disable-model-invocation: true" |
| The mutation removes both skill roots | `check:frontmatter` | "no SKILL.md" and two missing user-invoked skills |
| The `personal` manifest has the name `Personal Skills` | `check:plugin` | "Plugin name cannot contain spaces" |
| The `guardrail` manifest has invalid JSON | `check:plugin` | The `personal` plugin passed, then "Invalid JSON syntax" |
| The `personal` manifest sets `"dependencies": 7` | `check:plugin` | "expected array, received number" |

Each failed task exited non-zero and reported only its own failure.

### Ravel resolution of the personal pack

`ravel skills resolve --json` resolved the public clone with the status
`resolved`, before the checks ran in it. The source record has the type
`local` and the digest above. Its one search directory is the `craft` root of
the snapshot. The snapshot holds the `SKILL.md` files of the three craft
skills and no `guardrail`. `diff -r` found no difference between the snapshot
and `plugins/personal` of the clone. A resolution of the jj working copy
gave the same digest.

The instrument was the Ravel 2.0 build of `scripts/harness-trials/builds.json`.
Its `bin/ravel` SHA-256 is `26b487afb957…`, the pinned value, and it holds the
build identity `a4f55d04b2d1…` twice. The build came from an earlier scratch
prefix of this workstream, and the digest identified it. Each resolution ran
under a temporary `HOME` whose settings held one `local` source with the roots
`["craft"]` and an empty mandatory list.

### Installer proof of the personal pack

Grove's live tree still ships the four skills in `plugins/linkuistics`. The
installer refuses a skill name that two scanned plugins ship, and a run with
the live tree made no link. The proof therefore ran Grove's `install.sh` from
a scratch export of the source commit without `plugins/linkuistics`. That
tree is Grove after `personal-removal-k214`.

- An isolated home held Codex, Gemini CLI and Pi directories. In it,
  `install.sh --pack <public clone>/plugins/personal` linked the three craft
  skills into each harness, with targets in `craft/` of the clone. The run
  made 27 links in all and linked no `guardrail`. It printed the
  `assumes-personal-setup` note for `machine-environment`.
- A second home and a second public clone took the craft-only cycle of
  `make_craft_only_pack`. The cycle removed `machine-environment` from the clone. A
  run without `--pack` kept its links. A run with `--pack` printed one
  `unlink` line for each harness and kept the other craft links.
- No run needed `--force`, because neither the export nor the clones is a jj
  workspace. Each run set `HOME` to a scratch home, so the owner's real links
  were not touched.

### Publication of the personal pack

- The classifier refused `gh repo create` inside a compound command. The owner
  had already allowed `Bash(gh repo create *)` in an uncommitted
  `.claude/settings.json` of the Grove workspace. The command alone then ran
  as `gh repo create Linkuistics/PersonalSkills --public`.
- jj added the remote `origin` and pushed the bookmark `main`. The public
  `main` is `ac98636608b013391bf4b4ca0dc6c9c1506ff9d0`, the default branch.
- A plain clone of the local repository passed `task check` before the push.
- The source Grove commit `4cfac6295881` was already the head of Grove's
  remote bookmark `gh-issue-25`, so no push came before the pack.
- Grove's marketplace does not list either plugin yet. The owner's native
  plugins are unchanged, and this session read its personal skills from the
  owner's plugin cache.

### Tool requirements of the personal pack

| Skill or check | Tools |
| --- | --- |
| `machine-environment` | mise, Task and the owner's Obsidian manual |
| `using-jujutsu` | Jujutsu (`jj`), and Git in a colocated repository |
| `authoring-conventions` | None |
| `guardrail` | Claude Code, Bash and `jq` |
| The pack's `task check` | Task, Bash, `jq` and ShellCheck. Claude Code for `check:plugin`, which skips when Claude Code is not installed |

The README of the pack holds the same table. The inventory names `mise, task,
Obsidian manual` for `machine-environment` and `jj` for `using-jujutsu`. It
names the frontmatter hook and `/guardrail` for `guardrail`.

### For guardrail-plugin-trial-k213

- The `guardrail` plugin root is `plugins/guardrail` at the pack commit. Its
  manifest names no skill directory, so Claude Code scans the default
  `skills/`.
- The dependency stays, so a native installation of `personal` must also
  install `guardrail`. Grove's marketplace lists neither plugin before
  `personal-removal-k214`. The trial therefore needs a marketplace that lists
  both, for example a scratch marketplace with two `git-subdir` entries at
  the pack commit.
- The skill text tells the user to type `/guardrail`. Under the plugin the
  skill is `guardrail:guardrail`. The trial reads which spelling starts the
  skill.

### Guardrail plugin trial

`guardrail-plugin-trial-k213` read the second plugin of PersonalSkills in
real Claude Code sessions on 2026-11-05. Every reading passed at pack commit
`ac98636608b0`. The pack needed no repair, so `personal-removal-k214` pins
that commit.

This trial read `guardrail` from a scratch marketplace of `git-subdir`
sources, before any keg held it. It is evidence of that layout only. The
section "The hook adapters on the projected keg" of
[the Homebrew delivery evidence](homebrew-skill-delivery-evidence.md#the-hook-adapters-on-the-projected-keg)
repeats the trial from the revised `personal` keg.

#### Subjects of the guardrail plugin trial

| Subject | Value |
| --- | --- |
| Claude Code | 2.1.318, model `claude-sonnet-5-5` |
| Pack commit | `ac98636608b013391bf4b4ca0dc6c9c1506ff9d0`, the public `main` of PersonalSkills |
| Marketplace | A scratch marketplace named `scratch`. It lists two `git-subdir` sources from GitHub at the pack commit: `personal` from `plugins/personal` and `guardrail` from `plugins/guardrail` |
| Homes | A new directory in the session scratchpad for each run. It links only `Library/Keychains` to the owner's home |
| Environment | `env -i` with `HOME`, `USER`, `LOGNAME`, `SHELL`, `TERM`, `LANG`, `TMPDIR` in the home, and a system `PATH`. No variable names a Claude Code location |
| Session flags | `-p`, `--input-format stream-json`, `--output-format stream-json`, `--verbose`, `--include-hook-events`, `--permission-mode acceptEdits`, `--tools Bash,Skill`, `--allowedTools "Bash(rm -rf scratch-dir)"` |
| Gated command | `rm -rf scratch-dir` in the home's work directory. Before each session, the script makes `scratch-dir/inner/file.txt` |

The session reads its user turns as stream JSON, so one process takes two
turns. The first turn is `/guardrail`. The second turn asks the agent to run
the gated command once with the Bash tool. It then asks the agent to say
whether the command ran, and to quote any hook or permission message. The
control sends only the second turn. A resumed second process would not show
that the hook stays active for the session.

The allow rule permits the gated command. Without the hook, the command
therefore runs, and the control can show that no hook runs before it.

#### Readings of the guardrail plugin alone

A home installed `guardrail@scratch` and nothing else. The installation is
`ac98636608b0-1eebc2ed`. The cached `SKILL.md` has the SHA-256
`aca6380f0849…`, and the cached `guardrail-hook.sh` has `d04ae72e1d84…`.
Each equals the file of the pack commit.

| Reading | Result |
| --- | --- |
| `claude plugin details` | Skills 1, `guardrail`. Hooks 0, because the hook is in the skill's frontmatter and not in a plugin hooks file |
| The session's init event | The plugin `guardrail`. The only skill and the only command from it are `guardrail:guardrail` |
| The first turn | The session file records the typed `/guardrail` as the command `/guardrail:guardrail`, then the skill body from the plugin cache |
| The second turn | One `PreToolUse:Bash` hook start and response. The response has `permissionDecision` `ask` and the reason `guardrail: recursive forced delete (rm -rf). Confirm you intend to run: rm -rf scratch-dir` |
| The decision | A `permission_denied` event with the reason type `hook`. The tool result carries the reason as an error |
| `scratch-dir` after the session | Present |
| The agent's own reply | The command did not run, and it quoted the reason |

The session is `a2c9f553-a3e3-4a08-8972-3f1264317fbf`. Its stream has the
SHA-256 `1b508d0c1720…`, and its session file `2217b34fe157…`.

**Result: pass.** The plugin lists `guardrail` alone. `/guardrail` starts the
skill, and the hook returns `ask` for the gated command. In print mode no one
can answer that question, so Claude Code refuses the command.

#### Readings of the control

A new home installed `guardrail@scratch` in the same way. The session took
only the second turn.

| Reading | Result |
| --- | --- |
| The session's init event | The same plugin, skill and command as above |
| Hook starts in the stream | None |
| The tool result | `(Bash completed with no output)`, with no error |
| `scratch-dir` after the session | Absent |
| The agent's own reply | The command ran, and it saw no hook or permission message |

The session is `22b297cc-c503-46ea-8f29-046b6aca0148`. Its stream has the
SHA-256 `0f2050deea75…`, and its session file `31c0f89104ed…`.

**Result: pass.** Without `/guardrail` the hook does not run, and the allow
rule lets the command run. The `ask` of the run before therefore comes from
the skill that `/guardrail` started.

#### Readings of the ordinary plugin alone

A new home installed `personal@scratch` and nothing else. Claude Code
reported "(+ 1 dependency: guardrail)".

| Plugin | Installation | Skills | Hooks |
| --- | --- | --- | --- |
| `personal` | `ac98636608b0-1a86df1f` | 3: `authoring-conventions`, `machine-environment`, `using-jujutsu` | 0 |
| `guardrail` | `ac98636608b0-1eebc2ed` | 1: `guardrail` | 0 |

The cached `SKILL.md` of each craft skill and both cached guardrail files
equal the files of the pack commit.

The init event lists the plugins `personal` and `guardrail`. Its plugin
skills are `personal:authoring-conventions`, `personal:machine-environment`,
`personal:using-jujutsu` and `guardrail:guardrail`, and no other name holds
`guardrail`. The session took the same two turns as the first run. The
stream has the same hook start, the same `ask` response and the same
`permission_denied` event, and `scratch-dir` stayed.

The session is `715f7497-55a6-4d72-9adc-80b2700a32ff`. Its stream has the
SHA-256 `c3c4b73d85b5…`, and its session file `8940a2e8a102…`.

**Result: pass.** The ordinary plugin alone installs `guardrail` as its
dependency. The session lists the three craft skills and one `guardrail`, and
the hook works from the dependency.

#### Observations of the guardrail plugin trial

- **The typed command.** The skill text tells the user to type `/guardrail`.
  Claude Code 2.1.318 resolves that name to `/guardrail:guardrail`, so both
  spellings start the skill. The skill text needs no change.
- **The always-on estimate.** `claude plugin details` estimates about 236
  always-on tokens for `guardrail` and about 462 for `personal`. The skill
  claims that it never sits in the model's context. The model-facing skill
  listing in the session files has no `guardrail` entry and no
  `authoring-conventions` entry. Disposition: the estimate does not take
  account of `disable-model-invocation`. The claim of the skill holds.
- **The owner's installation.** Before the first run and after the last run,
  the owner's `~/.claude` had the same SHA-256 for `settings.json`,
  `plugins/installed_plugins.json` and `plugins/known_marketplaces.json`. A
  listing of `plugins/cache` and `skills` to depth 3 had the same SHA-256.

#### Limits of the guardrail plugin trial

- **Print mode only.** The readings show the hook's `ask` and the refusal in
  print mode. They do not show the interactive confirmation prompt.
- **The shell half only.** No run read the freeze half, which gates an edit
  outside the boundary. `check:guardrail-test` of the pack covers both halves
  of the hook script without Claude Code.
- **Two turns.** The readings show that the hook stays active from the first
  turn to the second. They do not read the span of the skill's lifecycle over
  many turns, which the skill marks as version-dependent.
- **One accepted run for each reading.** The hook is a script with no model
  decision, so a repeat adds no evidence about the pack.
- **Claude Code only.** The skill declares `harnesses: [claude-code]`, and
  the membership decision asks for no Codex trial.

### Removal of the personal pack

`personal-removal-k214` removed Grove's `linkuistics` plugin in one change with
every Grove consumer, on 2026-11-05. This is the contraction step of
`personal-pack-k183`, and the last craft move.

| Subject | Value |
| --- | --- |
| Removal commit | `a4c7df9d075e7040b85e5865e421f4e3f392d0dc` |
| Pinned pack commit | `1ff489d996a9f3efa091f145ce944d69c9b351be`, the public `main` of PersonalSkills. A child of `ac98636608b0`, the commit that the guardrail trial accepted. Grove's marketplace pins it for `personal` and for `guardrail` |
| Source directory and roots | `plugins/personal`, with the Ravel root `["craft"]` |
| Tree of `plugins/personal` | `23b39220f9ae5b974d54d71ae9e8435b7e206a64` in both pack commits |
| Tree of `plugins/guardrail` | `a5fb16931c82362a388818cd3bbb1d5f0d5bd53f` in both pack commits |
| `resourceDigest` of `plugins/personal` | `4cb21f4d5b3b00917093e93da9a26ea530b3abd42f2b9e01a65a10725fd11167`, from a clean clone of the public remote at the pinned commit |
| `guardrail` plugin | Root `plugins/guardrail` of the pinned commit. It holds `skills/guardrail/`, and no Ravel root reaches it |
| `GROVE_COMMIT` | None. The pack's checks read no Grove file |

The last pack commit changes only the pack's `README.md`. Three passages
described the delivery through Grove's `linkuistics` plugin. One stated in
the present tense that a native user of that plugin receives `guardrail`. Two
stated its removal as a future event. The pack needs no `GROVE_COMMIT`, so
the last pack commit came before the Grove removal, and Grove pins it. No file
under `plugins/` changed, so the tree IDs and the digest stay.

The removal took out the 10 tracked files of `plugins/linkuistics`: the
manifest, `PROVENANCE.md`, and the eight moved files of
[Source of the personal pack](#source-of-the-personal-pack). The tree held no
untracked file. The provenance file held only the authoring table, the two
moved sections and the paragraph that names where the earlier sections went.
It served no Grove skill, so Grove keeps no successor.

### Consumers of the removal of the personal pack

| Consumer | Disposition |
| --- | --- |
| `.claude-plugin/marketplace.json` | Drops `linkuistics`. Lists `personal` and `guardrail` with `git-subdir` sources at the pinned commit. The dependency of `personal` resolves because this marketplace lists `guardrail`. The description names the personal and guardrail subjects, and the marketplace counts ten plugins |
| The install commands in `README.md`, `plugins/README.md`, `plugins/grove/README.md` and `scripts/templates/grove.rb.tmpl` | `/plugin install linkuistics@linkuistics` becomes `/plugin install personal@linkuistics` |
| The header of `plugins/install.sh` | Its install example names `grove@linkuistics`. Its comments say that the script scans every bundled plugin directory, that Grove ships `grove`, and that `guardrail` is in PersonalSkills |
| `plugins/install.test.sh` | The fixtures keep the synthetic label `linkuistics`. The installer scans every plugin directory, so a fixture with two plugin directories still tests that rule. A renamed label proves nothing more, by the fixture-label rule of `pack-membership.md`. The comment of `make_repo` now calls the names labels |
| `plugins/README.md` | One plugin lives here. A `personal` and `guardrail` section replaces the `linkuistics` section. The install, `--pack`, versioning and editing passages name PersonalSkills. `machine-environment` carries `assumes-personal-setup` in PersonalSkills. The provenance link names the file of `personal` by its public URL |
| `plugins/CONTEXT.md` | One plugin ships here, and the marketplace lists `personal` and `guardrail`. The native spelling example is `personal:using-jujutsu`. The user-invoked, hook-carrying, house-delta, eligibility and personal-setup entries name PersonalSkills. The marketplace entry and the dialogue name install commands, not the removed `linkuistics:` namespace |
| `README.md` | The product row, the install list, a `personal@linkuistics` paragraph, the `--pack` example and the documentation list. The Codex passage says "a Claude-only skill" in place of `guardrail` |
| `docs/ARCHITECTURE.md`, `CONTEXT-MAP.md` and `CONTEXT.md` | The plugin row names only `plugins/grove/`. The external pack row names PersonalSkills and its native plugin `guardrail`. The skills context covers `grove` and the external packs. `using-jujutsu` is in PersonalSkills. The repository sense of "grove" holds Grove and the marketplace |
| `scripts/check-packs.sh` and the directory-only paragraph of `pack-membership.md` | Name `guardrail` as a source with a native skill and no kind skill. The script's logic did not change |
| `scripts/harness-trials/sources/README.md` | Says that `plugins/linkuistics` held the two rival skills at the recorded commits, and names the packs that ship them now |
| `scripts/harness-trials/cases/*/case.json` | Unchanged, as records of their builds, by "The trial cases that name moving skills". The supplier check refuses a case that reads `plugins/linkuistics` from a later commit |
| The citation rows of `plugin_fallback.rs`, the assertion of `commit_guidance.rs`, `native_skill_delivery.rs` and the spine's `execute.md`, `commit.md` and `grove.md` | Stay. They cite `machine-environment` and `using-jujutsu` by bare name in Grove's own files, or plant a fixture name |
| `docs/USAGE.md` | Unchanged. Its directory route never listed a `linkuistics` source |

No Grove file that stays links a moved file by a relative path. The one
provenance link now names the public URL in PersonalSkills.

This move changes no machine setting. No personal source is under `.config`,
and nothing recreates chezmoi. Rollout adds the mise entry of the checkout, by
order 6 of the cutover contract. The owner's native plugins stay at Grove
commit `238c3a839ab4`, and this leaf installed, updated and removed no plugin
in the owner's home.

### Readings of the removal of the personal pack

Conformance reads no file of `plugins/linkuistics` and no `craft/` root, so
the removal leaves the core and aggregate figures unchanged. `task check:packs`
passed the five kind packs alone and in the aggregate.

The stale-location search ran `rg --hidden` over the whole tree, apart from
`.jj`, `.git`, `target`, `.grove`, `.cache` and `node_modules`. It enumerated
the four skill names, `plugins/linkuistics`, `linkuistics@linkuistics`,
`linkuistics:`, `guardrail-hook`, `assumes-personal-setup`,
`linkuistics/PROVENANCE`, `` `linkuistics` `` and "linkuistics plugin". The
planning list did not hold the last two tokens. They found the plain plugin name
in `README.md`, `plugins/README.md`, `plugins/CONTEXT.md`, `CONTEXT-MAP.md`
and `CONTEXT.md`. Each hit after the removal is in a class of the next section.

The search found a planted path in a hidden scratch directory, and a misspelt
pattern found nothing. The same search over PersonalSkills found the moved files. The
hits there that name Grove's old paths are provenance.

A link check read every Markdown file of the working copy and of the parent
commit `a8372e21c7f`. It found the same 44 unresolved relative links in both,
and none of them names a moved or removed file. The removal therefore broke no
relative link.

### Asset classification of the removal of the personal pack

Each Grove file that still names a moved skill or the removed plugin is in one
class of "The Grove assets of moving craft skills" in
`docs/specs/pack-membership.md`.

| Files | Test | Disposition |
| --- | --- | --- |
| The ten files of `plugins/linkuistics` | 2 | Removed. Their successors are in PersonalSkills, as [Source of the personal pack](#source-of-the-personal-pack) states |
| The citation rows of `plugin_fallback.rs`, the assertion of `commit_guidance.rs`, `native_skill_delivery.rs` and the spine's `execute.md`, `commit.md` and `grove.md` | 1 | Stay. They cite the skills by bare name in Grove's own files, or plant a fixture name. The doc comment of `plugin_fallback.rs` names the old alias as history |
| `README.md`, `plugins/README.md`, `plugins/CONTEXT.md`, `CONTEXT-MAP.md`, `CONTEXT.md`, `docs/ARCHITECTURE.md`, `.claude-plugin/marketplace.json`, `scripts/check-packs.sh` and `plugins/install.sh` | 1 | Stay. They state where the skills ship. The marketplace name `linkuistics` stays, because it is the identity of the marketplace |
| `plugins/install.test.sh` | 1 | Stays. Its synthetic plugin label `linkuistics` reads no real file |
| `scripts/harness-trials/sources/README.md` and the trial cases | 1 and 3 | Stay. The README states the past location. The cases are records of their builds |
| `pack-membership.md`, `owner-skill-delivery.md`, `external-pack-contract.md` and its design views | History and design | Stay. They state the baseline, the decisions and the target layout |
| `CHANGELOG.md`, this record, `common-loading-acceptance-record.md`, the formal-methods records, `evidence-outlives-the-instrument.md` and the research notes | History | Stay |
| `docs/owner-skill-inventory.tsv`, `docs/ravel-caller-surface-inventory.tsv`, `docs/owner-skill-delivery-evidence.md`, `docs/grove-skill-delivery-design-evidence.md` and the evaluation records | 3 | Stay as the baseline evidence |

### Checks of the removal of the personal pack

- `bash scripts/check.sh` passed all 19 principal checks on the removal
  change, with 1344 Rust tests. `GROVE_TEST_RAVEL` resolved to the installed
  Ravel 1.1.0, `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. The changed files
  had the same digests before and after the run. After the run, this session
  reflowed one comment of `install.test.sh` and split one paragraph of
  `pack-membership.md`. `install.test.sh` then gave 48 passed again, and ShellCheck reported
  nothing for it. No check reads the bytes of `pack-membership.md`.
- `task check:packs` passed `requirements`, `design`, `engineering`,
  `research` and `writing`, alone and in the aggregate. `testanyware`,
  `technical-communication-hooks`, `personal` and `guardrail` passed the
  directory-only check. For `personal` and `guardrail`, the check read
  `1ff489d996a9`.
- `claude plugin validate .` accepted the marketplace with the `version`
  warning, on Claude Code 2.1.318.
- The pack's `task check` passed from a clean plain-Git clone of the public
  remote at `1ff489d996a9`. `check:plugin` passed both plugins with the
  `version` warning. `check:frontmatter` printed `4 skill(s) declare
  harnesses:, and 2 user-invoked skill(s) keep disable-model-invocation:
  true`. `check:guardrail-test` printed `37 passed, 0 failed`.
  `check:shellcheck` reported nothing. After the run, the clone held no
  untracked or ignored file.
- `ravel skills resolve --json` resolved that clone with the status
  `resolved`, before any check ran in it. Its one search directory is the
  `craft` root of the snapshot. The snapshot holds the `SKILL.md` files of the
  three craft skills and no `guardrail`, and `diff -r` found no difference from
  `plugins/personal`. The instrument was the Ravel 2.0 build of
  `scripts/harness-trials/builds.json`, `bin/ravel` SHA-256 `26b487afb957…`,
  with the build identity `a4f55d04b2d1…` twice. The temporary `HOME` held one
  `local` source with the root `["craft"]` and an empty mandatory list.
- An isolated home held Codex, Gemini CLI and Pi directories. In it,
  `install.sh --force --pack <public clone>/plugins/personal` ran from this
  workspace without `plugins/linkuistics`. It linked Grove's six core skills
  and the three craft skills into each harness, 27 links in all. The craft
  targets are in `craft/` of the clone. It linked no `guardrail`, and it printed the
  `assumes-personal-setup` note for `machine-environment`. The run needed
  `--force`, because this workspace is a secondary jj workspace. The home was a scratch
  directory, so the owner's real links were not touched.

### Tool requirements after the removal of the personal pack

The tool requirements of
[Tool requirements of the personal pack](#tool-requirements-of-the-personal-pack)
hold unchanged. `machine-environment` needs `mise`, `task` and the owner's
Obsidian manual. `using-jujutsu` needs `jj`, and Git in a colocated
repository. `guardrail` needs Claude Code, Bash and `jq`. Grove's own checks
no longer read any of these skills.

### For moves-handoff-k187

| Source | Root and revision |
| --- | --- |
| `personal`, the Ravel source | `https://github.com/Linkuistics/PersonalSkills.git` at `1ff489d996a9f3efa091f145ce944d69c9b351be`, source directory `plugins/personal`, roots `["craft"]`, `resourceDigest` `4cb21f4d5b3b00917093e93da9a26ea530b3abd42f2b9e01a65a10725fd11167` |
| `guardrail`, the native Claude Code plugin | The same repository and commit, plugin root `plugins/guardrail`. Grove's marketplace lists it as `guardrail@linkuistics`. No Ravel root reaches it |
| `personal`, the native Claude Code plugin | The same repository and commit, plugin root `plugins/personal`. It installs `guardrail` as its dependency |

## Homebrew commits of the packs

`pack-homebrew-commits-k231` published one commit in each of the six pack
repositories. Each commit describes Homebrew installation and Ravel delivery
in place of the marketplace and installer steps. Grove's pack record
`plugins/packs.json` names the five domain commits.

### The six commits

| Pack | Commit on the public `main` | Source directory |
| --- | --- | --- |
| RequirementsSkills | `c51ec8bebd811c0087ad19a1d66d0aa87bd01386` | `plugins/requirements`, unchanged since `9f7278c2d46b` |
| DesignSkills | `aa088156d1a55e28dd9b59f0330f4ab9f1f3d330` | `plugins/design`, unchanged since `9d7c7c5368f6` |
| EngineeringSkills | `8d1a2bc3e07bec4ce57e0391a6aa778dff63a017` | `plugins/engineering`, unchanged since `d98862f21749` |
| ResearchSkills | `8f321f2e99e09a8ba5e60fa4a08871727e84df62` | `plugins/research`, unchanged since `dc3a4812db3b` |
| WritingSkills | `b9240e0138df7d9f1d6926b77737668463dcfdfb` | `plugins/writing`, with three changed files since `4cafb0fd8e72` |
| PersonalSkills | `cddcfde901cdaf8069e2aac6f3b6bc5aba9fe925` | `plugins/personal`, with one changed file since `1ff489d996a9` |

The "unchanged since" commit is the one that Grove's pack record named before
this change. `git diff --stat` between the two commits, limited to the source
directory, printed nothing for the four unchanged packs.

### Changes of the commits

- Each README names its formula: `grove` for the five domain packs and
  `personal` for PersonalSkills. It states that Ravel delivers the skills from
  Homebrew's shared skill directory. It gives no native marketplace or
  installer step. A section "Development" names a checkout as a trial source.
- WritingSkills' README gives the documented commands of the three
  technical-communication hook adapters. PersonalSkills' README gives those of
  `guardrail@personal`.
- WritingSkills removed its root `.agents/plugins/marketplace.json`. Its user
  guide, its specification, the harness map of the specification and the
  decision record `technical-communication-carries-no-standard-text` now name
  the keg routes. The descriptions of the two hooks plugin manifests and a
  comment of the Pi extension changed. These are the three changed files of
  `plugins/writing`.
- PersonalSkills' `authoring-conventions` says "the Linkuistics skill packs"
  in place of "this marketplace". It states that no installer reads the
  `harnesses:` key. The comment of `scripts/check-frontmatter.sh` says the
  same. This is the changed file of `plugins/personal`.
- The dated records of WritingSkills keep their bytes: the harness survey, the
  publication review and the evaluation records.

### Checks of the commits

- Each pack passed its own `task check` in its jj working copy. Each commit
  then passed `task check` in a plain Git clone, checked out at the commit,
  with Git's global and system configuration off.
- `claude` is not on the PATH of Task's shell in this session, so
  `check:plugin` skipped in the first runs. The clone runs put
  `~/.local/bin` first on the PATH, and `claude plugin validate` passed with
  warnings for each plugin.
- No commit records a new `GROVE_COMMIT`, so no Grove push came before a pack
  push. Each pack kept the Grove commit that its previous commit recorded.
- jj pushed each `main` to `origin`. `git ls-remote origin refs/heads/main`
  printed the commit of the table for each pack.
- Grove's `task check:packs` fetched the five domain commits from their public
  remotes. It reported "5 recorded pack(s) pass alone and in the aggregate".
- A sweep of each pack searched for `install.sh`, `.agents/plugins`,
  `@linkuistics` and "marketplace". It found four classes only:
  - the new sentences that state that no marketplace or installer step exists
  - history in provenance sections
  - the dated records
  - generic guidance of `simplify-project`

  The same pattern found the routes in each README of the previous commit.

### For personal-formula-k232

The `personal` formula pins PersonalSkills commit
`cddcfde901cdaf8069e2aac6f3b6bc5aba9fe925`. Its `plugins/personal` differs from
commit `1ff489d996a9` in `authoring-conventions`. The `resourceDigest` in the
PersonalSkills section therefore does not apply to this commit. That leaf
measures the digest of the keg source against a clean checkout of the commit.

## The personal formula

`personal-formula-k232` published `Formula/personal.rb` in the Linkuistics
tap. The tap commit is `9b3a46deb7432d5430bbe467c86b431842fe3474`, and
`git ls-remote origin refs/heads/main` printed it after the push.

### Contents of the formula

| Part | Value |
| --- | --- |
| Source | `https://github.com/Linkuistics/PersonalSkills/archive/cddcfde901cdaf8069e2aac6f3b6bc5aba9fe925.tar.gz`, `sha256` `ff55ec0adea4e8994e9b650ddaa2776d2dda4778773093015560f33967809c74` |
| Version | `2026-11-06`, the date of the commit |
| `share/ravel/skills/personal` | `plugins/personal`, moved into the keg |
| `libexec/plugins/guardrail` | `plugins/guardrail`, moved into the keg |
| `.claude-plugin/marketplace.json` | Written by the install method. Its name is `personal`, and it lists `guardrail` from `./libexec/plugins/guardrail` |
| Dependencies | None |
| Caveats | The default skill directory and the two documented commands of `guardrail@personal` |

The install method moves each directory, so each file keeps the mode of the
source archive. The archive gives the two guardrail scripts mode `0775` and
the other files `0664`. Ravel's digest covers no write bit.

### Checks of the formula

`task tap:formula:test` runs `scripts/tap-formula.test.sh`. It downloads the
formula's URL and compares its `sha256`. Then it runs Homebrew's install and
test methods of the formula in a private Homebrew-shaped prefix. The keg lies
in `Cellar/personal/2026-11-06`, an `opt` link names it, and a relative link
in the prefix's `share/ravel/skills` names the source.

- The keg source holds the files of `plugins/personal` in a plain checkout of
  `cddcfde901cd`, each with its Git mode.
- A private copy of the Ravel 2.0 build resolved the source with no settings.
  Its installation record named the prefix's `share/ravel/skills`. The
  `originPath` lay in the keg, the only root was `craft`, and the
  `resourceDigest` was
  `dc1f26eafe9b7db6f750c95db38a7876beb29b067d829eaf63203323b89d61b6`. Settings
  that named the plain checkout gave the same digest and root.
- The build was Ravel `ff14cba826b02ac492fcde0996d662c5378ef923`, the 2.0 build
  of `grove-default-directory-checks-k228`, in the `dev` profile. It reports
  `ravel 1.1.0` until the cut.
- `claude plugin validate` accepted the keg with one warning: the plugin
  manifest names no version. The packs omit that field by design.
- In an isolated Claude Code home, `claude plugin marketplace add` with the
  prefix's `opt/personal` path and `claude plugin install guardrail@personal`
  succeeded. `claude plugin list --json` then showed `guardrail@personal`
  enabled.
- `brew style Formula/personal.rb` found no offence. `brew audit --strict
  --online linkuistics/taps/personal` exited 0 on the tap clone at the pushed
  commit. The same audit reported four offences in `grove`, so the audit read
  the tap. `grove`, `ravel-lite` and `testanyware` have earlier style
  offences, which this move did not change.

Each control failed against a changed copy of the formula:

- An appended line in a `SKILL.md` kept every mode. The file check passed,
  and the digest comparison failed.
- An executable bit on `PROVENANCE.md` failed the file check.
- A guardrail script without its executable bit failed the formula's test.
- A marketplace entry with a missing plugin path failed the formula's test.
  Without that assertion, `claude plugin validate` passed, and
  `claude plugin install` refused the missing path.

### For rollout

| Source | Root and revision |
| --- | --- |
| `personal`, the Ravel source | The `personal` formula of tap commit `9b3a46deb743`. PersonalSkills commit `cddcfde901cdaf8069e2aac6f3b6bc5aba9fe925`, source directory `plugins/personal`, roots `["craft"]`, keg and checkout `resourceDigest` `dc1f26eafe9b7db6f750c95db38a7876beb29b067d829eaf63203323b89d61b6` |
| `guardrail`, the native Claude Code plugin | The same keg, plugin root `libexec/plugins/guardrail`, harness identity `guardrail@personal` |

This leaf did not install the formula in the owner's prefix.
`owner-pack-checkouts-k93` owns that step.

## The third-party formulae

`third-party-skill-formulae-k233` published `Formula/superpowers.rb` and
`Formula/quint-llm-kit.rb` in the Linkuistics tap. The tap commit is
`02da210bd77dd6469d29ddd0cf7a47fee5c96a24`, and
`git ls-remote origin refs/heads/main` printed it after the push.

### The pins

This leaf read both upstream pins again on 2026-11-07.

- Superpowers keeps `bb92a77741419a4ab5f06e711a283343f1ada0c3`, the commit of
  tag `v7.0.0`. Upstream has since released `v7.0.1` and `v7.0.2`. The owner's
  native plugin and the owner design's selected pack are 7.0.0, so the switch
  changes the route of these skills and not their text.
- quint-llm-kit keeps `cc75369f741af7d490936f82002c2d28e3b3d78d`, which is
  still the head of `main`. The repository has no release and no tag, so the
  version is that of its plugin manifest.

### Contents of the formulae

| Part | `superpowers` | `quint-llm-kit` |
| --- | --- | --- |
| Source | `https://github.com/obra/superpowers/archive/bb92a77741419a4ab5f06e711a283343f1ada0c3.tar.gz` | `https://github.com/quint-co/quint-llm-kit/archive/cc75369f741af7d490936f82002c2d28e3b3d78d.tar.gz` |
| `sha256` | `496b576d4a4a62c0d72cce4a9e99ff46c57026c07fe503931cbb8afd33006cf2` | `152947a80275a1d8dd935ba6bec58e58ef92ef367cd70ca7a142b81dbbd5ca20` |
| Version | `7.0.0` | `1.0.0` |
| Licence | `MIT` | `Apache-2.0` |
| Keg source | The repository root at `share/ravel/skills/superpowers` | `quint-llm-kit-plugin` at `share/ravel/skills/quint` |
| Marketplace file, dependencies | None | None |

Homebrew's stage creates `.brew_home` in the build directory before the install
method runs. The superpowers formula copies each other entry of the repository
root, so `skills/using-superpowers/SKILL.md` keeps its path. Neither repository
has an `export-ignore` attribute, so each archive holds the files of its
commit.

### Checks of the formulae

`task tap:formula:test` ran each formula with `SOURCES=superpowers=.` and
`SOURCES=quint=quint-llm-kit-plugin`. This leaf changed the test in two ways.

- `scripts/tap-formula.test.rb` now sets the formula's `buildpath` and creates
  `.brew_home` there, as Homebrew's stage does.
- `scripts/tap-formula.test.sh` accepts `.` for the repository root. It sorts
  the keg's list and the commit's list in the same order. Before, an
  executable file in a source failed the comparison, and the `personal`
  source has no executable file.

The results follow.

- Each keg source held the files of its source directory in a plain checkout,
  each with its Git mode.
- A private copy of the Ravel 2.0 build resolved each source with no settings,
  through the prefix's `share/ravel/skills`. Each `originPath` lay in the keg,
  and the only root was `skills`. The superpowers `resourceDigest` was
  `be1ce6e3c25064b83ba2999b37972deb7f73ce88a5f5dbbba990d88037b17230`. The
  quint `resourceDigest` was
  `0a8b945fbc6732d5b7b3041a966d26cdf718ca884bd5f3fb8435297d7258edef`. Settings
  that named each plain checkout gave the same digest and root.
- The build was Ravel `ff14cba826b02ac492fcde0996d662c5378ef923`, in the `dev`
  profile.
- The `personal` formula passed the changed test again.
- `brew style` found no offence in either formula. `brew audit --strict
  --online` exited 0 for both names on the tap clone at the pushed commit. The
  same audit refused `grove`, so the audit read the tap.

Each control failed against a changed copy of a formula:

- An install without the `.brew_home` exclusion failed the formula's test.
- The same install without that assertion failed the file check on
  `.brew_home/.gitignore`.
- An executable bit on the superpowers `README.md` failed the file check.
- An appended line in a quint guideline kept every mode. The file check
  passed, and the digest comparison failed.

### For rollout

| Source | Root and revision |
| --- | --- |
| `superpowers` | The `superpowers` formula of tap commit `02da210bd77d`. Superpowers commit `bb92a77741419a4ab5f06e711a283343f1ada0c3`, source directory the repository root, roots `["skills"]`, keg and checkout `resourceDigest` `be1ce6e3c25064b83ba2999b37972deb7f73ce88a5f5dbbba990d88037b17230` |
| `quint` | The `quint-llm-kit` formula of the same tap commit. quint-llm-kit commit `cc75369f741af7d490936f82002c2d28e3b3d78d`, source directory `quint-llm-kit-plugin`, roots `["skills"]`, keg and checkout `resourceDigest` `0a8b945fbc6732d5b7b3041a966d26cdf718ca884bd5f3fb8435297d7258edef` |

This leaf did not install either formula in the owner's prefix.
`owner-pack-checkouts-k93` owns that step.

## The TestAnyware formula

`testanyware-skill-formula-k234` released TestAnyware `v2.8.0` from commit
`1809a49068a23d9d276913f3a90227dbefb98a82`. The annotated tag object is
`30292bdc2cd30f62c3200a8176dc9330b164ce01`. The release procedure published
the tag, the GitHub Release and tap commit
`9ca2b5e3751047b1d81157d43146828e933bcceb`. After the push,
`git ls-remote` printed the tag, TestAnyware's `main` at the release commit
and the tap's `main` at the tap commit. TestAnyware's procedure states minor
for a new installed path, so the release is minor.

### Contents of the release

| Part | Value |
| --- | --- |
| Release assets | `testanyware-v2.8.0-aarch64-apple-darwin.tar.xz`, the two Linux archives and `testanyware-app-v2.8.0-aarch64-apple-darwin.zip` |
| macOS archive `sha256` | `bda57d356222e6ef7eb7bc7be7c6c815cc890b9b0743ff7c1aa5b6130c285a30` |
| `share/ravel/skills/testanyware` in each archive | `plugins/testanyware` of the release commit, from `git archive`, so only tracked files with their Git modes |
| Formula `install` | `(share/"ravel/skills").install "share/ravel/skills/testanyware"`, which moves the folder and keeps each mode |
| Formula `test` | Asserts that `share/ravel/skills/testanyware/skills/using-testanyware/SKILL.md` exists |
| README | The formula installs the skill, and Ravel delivers it. It gives no native marketplace step |

TestAnyware's `release_candidate.py verify` now compares the skill folder of
each host archive with `plugins/testanyware` of the tagged commit. It compares
the path, the Git mode and the blob ID of each entry. The build and the publish
step both run `verify`.

This leaf changed two other parts of TestAnyware's release procedure.

- macOS `tar` wrote an AppleDouble member, `._<name>`, for each file with an
  extended attribute. The 2.7.0 macOS archive holds 35 such members among 70.
  macOS `tar` hides them, but GNU `tar` on Linux unpacks them as files. The
  build now packs with `--no-mac-metadata`, and `verify` refuses an extra
  member. Each 2.8.0 archive holds no AppleDouble member.
- `release-publish.sh` named a tap path that does not exist. Its bare
  `git push` fails on the jj tap, and that failure comes after the GitHub
  Release. It now defaults to `~/Development/Delivery/homebrew-taps` and publishes
  through jj, as Grove's release script does.

### Checks of the release

These checks ran before publication.

- `task release:test` passed, with seven new unit tests of the skill check and
  four new scenes. The scenes cover an archive without the skill, a jj tap, a
  jj tap behind its remote and a git tap on a detached `HEAD`.
- `task cli:test` gave 1655 passed and 0 failed. `task scripts:check`,
  `task plugin:check` and `task release:doctor` passed.
- The procedure built, notarised and verified the candidate once. `task tap:formula:test`
  ran the candidate formula from a local archive URL, with
  `SOURCES=testanyware=plugins/testanyware`. The keg held the files of
  `plugins/testanyware` with their Git modes. Ravel 2.0 resolved the keg
  source with the root `skills` and the `resourceDigest` of the plain
  checkout.

Each control failed:

- A formula without the install line failed its own test.
- Archives with no skill, a changed file, a lost executable bit, a changed link
  target, an extra `._SKILL.md` or a missing file each failed `verify`.

These checks ran after publication.

- `brew upgrade` installed 2.8.0 in the owner's prefix, and `brew upgrade
  --cask` installed the 2.8.0 app. `testanyware --version` printed `2.8.0`, and
  `brew test` passed.
- The installed keg source held the files of `plugins/testanyware` with their
  Git modes. A private copy of the Ravel 2.0 build resolved it through
  `/opt/homebrew/share/ravel/skills` with no settings. Its `originPath` was
  `/opt/homebrew/Cellar/testanyware/2.8.0/share/ravel/skills/testanyware`, its
  only root was `skills`, and its `resourceDigest` was
  `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6`. Settings
  that named a plain checkout of the release commit, made with Git's global and
  system configuration off, gave the same digest and root.
- The build was Ravel `ff14cba826b02ac492fcde0996d662c5378ef923`, in the `dev`
  profile.
- `brew style` and `brew audit` reported three offences in the formula: a
  redundant `version`, `on_linux` in `install` and `Formula["python@3.12"]`.
  The 2.7.0 formula has the same three, and the new lines add none.

No other installed keg holds `share/ravel`, so Homebrew linked
`/opt/homebrew/share/ravel` as one link to the TestAnyware keg. When another
keg brings a source, `brew link` makes real folders with one link for each
source.

### For rollout

| Source | Root and revision |
| --- | --- |
| `testanyware` | The `testanyware` formula of tap commit `9ca2b5e37510`. TestAnyware commit `1809a49068a23d9d276913f3a90227dbefb98a82`, source directory `plugins/testanyware`, roots `["skills"]`, keg and checkout `resourceDigest` `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6` |

The owner's prefix now holds this source in Ravel's default skill directory.
The owner's Claude Code also still has the native `testanyware` plugin. A
rollout that turns on Ravel 2.0 delivery must remove that plugin, or the
harness sees `using-testanyware` twice.

## The retired marketplace

`native-marketplace-retirement-k235` retired Grove's `linkuistics` listing on
2026-11-07. `.claude-plugin/marketplace.json` keeps the name `linkuistics`
and lists no plugin. Its description says that the marketplace is retired
and points to Homebrew and Ravel 2.0. `claude plugin validate` accepts it
with one warning, "Marketplace has no plugins defined". The CHANGELOG's
Unreleased section gives a user of the marketplace the four steps and the
reading below.

### Remote listings

After `git fetch`, only Grove's `origin/main`, at `238c3a839ab4`, held
marketplace files. They are the root Claude Code file, with ten plugins, and
the root Codex file `linkuistics-hooks`. The `origin/main` of each of the six
pack repositories and of TestAnyware held no marketplace file. The same
listing of Grove's tree was the positive control.

This line removed the root
Codex file in `grove-hook-adapters-k230`, and this leaf empties the root
Claude Code file. When finish moves `main`, no remote marketplace that Grove
owns lists a pack. This leaf pushed nothing.

### A refresh of the retired listing

The [evidence](homebrew-skill-delivery-evidence.md#a-refresh-of-the-retired-listing)
holds the measurement. A refresh of the retired listing succeeds. Each
installed plugin of the marketplace stays installed and enabled, but it no
longer loads.

`claude plugin list` reports "Plugin grove not found in
marketplace linkuistics", and a session receives none of the plugin's
skills. `claude plugin update` fails. An interactive start shows no notice.
`/plugin marketplace remove linkuistics` uninstalls the plugin.

### Consumers

- No test reads the current listing. `task check:packs` reads
  `plugins/packs.json`. The release tests read the keg marketplace files of
  `assets/keg-root`. `scripts/test_harness_trial.py` writes its own listing.
- The cases `requirements-native-namespace`, `design-native-namespace` and
  `research-native-namespace` stay. The trial runner reads the listing of each
  trial's prepared commit, and the acceptance record names those commits. At
  a commit after this leaf, the runner refuses each of them.
- The formula caveats no longer tell a user to add `Linkuistics/grove`. The
  rest of their "Skills" text belongs to `homebrew-delivery-guidance-k237`.

### For rollout

The owner's `~/.claude/plugins/known_marketplaces.json` holds
`autoUpdate: true` for `linkuistics`. On 2026-11-07 its `lastUpdated` was
04:33:28Z. That time is 47 seconds after a signed-in Claude Code session
started, and the marketplace clone was at Grove `238c3a839ab4`. A signed-in
start therefore refreshes the registration.

The owner design freezes these
automatic updates from the preparation step onward. If the freeze is absent
when finish moves `main`, the next signed-in start refreshes the retired
listing. Each plugin of `linkuistics` then stops loading, `grove` among them.
This leaf read the owner's file and changed nothing.

## The removed installer

`manual-installer-removal-k236` removed `plugins/install.sh` and
`plugins/install.test.sh` on 2026-11-07, with the `--pack` option. No task,
script or test runs them. `scripts/check.sh` lost its "plugin install" step,
and `task check:skills` lost its installer step. The `harnesses:` key stays
as metadata, and no installer reads it.

### Consumers

- The test `a_native_only_lifecycle_completes_with_explicitly_installed_skills`
  ran the installer for Codex. It now links each core skill into
  `~/.codex/skills` itself. Its seam, "Native-only bootstrap on 1.1", stays,
  because Grove still admits the native routes.
- The formula caveats lost the installer commands. The formula test lost
  their expectation, and also the expectation of
  `/plugin install grove@linkuistics`, which `native-marketplace-retirement-k235`
  had already removed from the caveats.
- `plugins/README.md`, `plugins/CONTEXT.md`, `docs/USAGE.md`, `README.md`,
  `plugins/grove/README.md`, `docs/ARCHITECTURE.md`, `CONTEXT-MAP.md` and
  `docs/RELEASING.md` keep no installer instruction. `plugins/CONTEXT.md`
  replaces the four installer terms with one entry, "Manual installer".
  `homebrew-delivery-guidance-k237` still writes the final installation
  guidance, the caveats' "Skills" text among it.

### The sweep of comments and books

The sweep enumerated each line of the walkthrough books and of every file
outside `docs/`, `.grove/` and the Markdown files that matches `install`,
`marketplace`, `provision`, `native` or `--pack`. It then classified each
line. Rules classified the tool installations, the Ravel builds, jj's native
workspaces and the platform-native targets. Each line that ties one of
those words to a skill, a plugin or the methodology was read and classified
as current or history. The run fails on any line without a class.

- Each passage now names Homebrew installation and Ravel delivery, the
  supported native routes, or the hook adapters of the kegs. Otherwise it is
  history and says so.
- The comment of `run` in `crates/grove-loop/src/loop_driver.rs` named a
  plugin that a human installs. It now names Homebrew and Ravel, and the
  `grove-loop` book changed its fragment with it. Comments in four test
  files and in `plugins/grove/conformance.test.sh` changed in the same way.
- The `overview` book's product table and the skill section of its
  `03-three-steps.md` now name Homebrew and Ravel.
- Two lines of the caveats stay for `homebrew-delivery-guidance-k237`, whose
  task names them.
- The positive control was a scratch comment and a scratch book line that
  named the installer as a route. The sweep failed on both, and it passed
  after their removal. The cross-tree control found installer lines in this
  record, in `CHANGELOG.md` and in the external pack contract. A mutated
  pattern found none there.

All five books pass `book-check --final`. With the old comment restored in
the `grove-loop` book alone, the check failed with `F008` at the changed
byte.

### For rollout

A read-only look at the owner's homes on 2026-11-07 found no live link that
the installer made. The 44 links in `~/.codex/skills` point into
`~/.agents/.grove/current`, so the 22.8 provisioner made them. The 44 links
in `~/.pi/agent/skills` and the one link in `~/.pi/agent/extensions` point
into the moved checkout `~/Development/grove` and dangle.
`docs/owner-skill-inventory.tsv` already lists them for deactivation.
`~/.gemini/skills` is absent. This leaf changed nothing in those homes.

## The delivery guidance

`homebrew-delivery-guidance-k237` rewrote Grove's shipped guidance on
2026-11-07 for the final route. Homebrew installs Grove, core and the five
domain packs. Ravel 2.0 reads them from its default skill directory with no
settings. Guidance sets up no native installation. The two native routes stay
in the route tables as caller routes that Grove admits.

### Changes

- `README.md` has one install section for Grove and its skills. Its section
  "Skills" names the six sources, the other formulae and the hook adapters.
  It also gives the steps for a user of the retired `linkuistics` marketplace.
- `docs/USAGE.md` rewrites the directory route. The settings are optional, and
  a `sources` example names keg paths. A new passage, "The hook adapters",
  holds the table of commands of the delivery design.
- `plugins/README.md` replaces each marketplace paragraph with the pack record
  and the formula, and its install sections with one Homebrew section.
  "Versioning" keeps the rule of no `version`, for the reason that holds now: a
  cut must leave the `grove-core` bytes equal to those of the candidate keg.
- `configure-grove` and its `policy.md` state that the directory route needs
  no settings key. Ravel 1.1 and Ravel 2.0 can therefore share one HOME.
- `docs/RELEASING.md` has the section "A pack change", with the four steps.
- The caveats' "Skills" text names the default skill directory. The formula
  test expects that text, and it refuses the retired routes.
- `plugins/CONTEXT.md`, `CONTEXT.md`, `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`
  and the spine's `references/grove.md` lose their marketplace, symlink and
  checkout routes. The example dialogue of `plugins/CONTEXT.md` no longer
  describes the symlink installer as current.

### The sweep

The sweep enumerated each line of each tracked file that matches
`skills.sources`, a `skills` key or object, `checkout`, `marketplace`,
`installer`, `install.sh`, a plugin install or add command, `@linkuistics`,
`symlink install` or `git-subdir`. Each file outside the guidance takes the
class of its file: design and measurement records, dated records, the
changelog, books, code and build configuration. Each guidance line was read
and classified. The run fails on any line without a class.

- Each guidance line now names the default route, a working tree, a build or
  check input, or a hook adapter of a keg. Some lines name the optional settings or
  define a glossary term. Each other line states a retired route as retired,
  or it is history.
- The mutation control added three retired-route lines to a copy of
  `README.md`. The sweep failed on each of them.
- The cross-tree control read five guidance files at the parent revision. The
  sweep failed on 79 of their lines, such as `/plugin install grove@linkuistics`
  and the settings that name checkouts.
- The positive control found four known current lines. With an empty file
  list, it reported them as missing.

## The candidate keg

`candidate-keg-k238` added `task owner:candidate-keg` on 2026-11-07 and
removed `task owner:checkouts` with its script. The task builds the candidate
keg of a named Grove commit for the staging trials, as the
[owner delivery design](specs/owner-skill-delivery.md#skill-formulae) states.
The [Keg craft trials](common-loading-acceptance-record.md#keg-craft-trials)
hold its readings and the harness trials that read the keg.

### What the task does

- It makes a plain Git checkout of the commit and builds an unreleased archive
  with that checkout's own release build, for the host target.
- It runs Homebrew's install method of that checkout's formula template in a
  private prefix. It makes the `opt/grove` link and the links of
  `share/ravel/skills`.
- A private copy of the 2.0 build resolves the six sources with no settings.
  Each digest must equal that of the plain checkout of the same commit.
- It prints the commit and each digest, and it writes `digests.tsv` and
  `candidate.json`. `ACCEPTED` names an earlier `digests.tsv`, and a
  different digest fails the task.

### For rollout

| Subject | Value |
| --- | --- |
| Candidate built | Grove `7b113a28a96621217c16154d4f44eba667236132`, the commit of `homebrew-delivery-guidance-k237`. No remote held it on 2026-11-07 |
| Digests | The six digests in the table of [The candidate keg](common-loading-acceptance-record.md#the-candidate-keg). Two builds gave the same digests |
| Trial input | `prepare --keg` and a source with `"keg": "grove"` in `scripts/harness_trial.py`. The case `keg-craft-discovery` is the model |

The staging leaves of `owner-ravel-skill-inventory-k38` build the keg of the
published candidate and record its accepted digests. The trials here do not
replace that evidence.

## The handoff to the owner cutover

`moves-handoff-k187` checked the Done when of `repository-moves-k5` against
the delivered state on 2026-11-07. This section gives
`owner-ravel-skill-inventory-k38` and `discovery-rollout-k12` the result of
every move. Where it names a commit or a digest, that value replaces the value
of an earlier section. The earlier sections describe the state after each move.

### The Grove candidate

| Subject | Value |
| --- | --- |
| Candidate commit | `31fbab3ea93575ab603d2f77992593ab20586e4f`, the release commit of Grove v22.9.0 |
| Tree ID of `plugins/grove` | `b9a00084f5f92a76a2376042cd243c91dae28eb1` |
| Blob ID of `plugins/packs.json` | `2cebaf5f062d08b50b22d058bd8c4db3d412f8a6` |
| Published line | The tag `v22.9.0` and `main` of `https://github.com/Linkuistics/grove.git` |

`tc-hook-bytecode-k284` revised the candidate and the `writing` row on
2026-11-10. WritingSkills `578f84d1f5a9` stops the hooks from writing Python
bytecode into the source. The earlier values were the candidate
`7b113a28a96621217c16154d4f44eba667236132`, the blob ID
`e00c67b74939542db0b7af2428d918a3458d73a4`, and the `writing` commit
`b9240e0138df7d9f1d6926b77737668463dcfdfb` with the tree ID
`8c476d7b082ee4af6b621eb6d94bfda0e79e8b01` and the `resourceDigest`
`2dea55cd72d0d56032c04419bfd438b62b321563573a09fffcab2f2c692ecade`. The
tree ID of `plugins/grove` and the digest of each other source are
unchanged. The section "Session probes" of
`docs/owner-skill-delivery-evidence.md` records the new measurement.

The candidate is the last commit of this line that changes `plugins/grove` or
`plugins/packs.json`. The jj revset `files("plugins/grove") |
files("plugins/packs.json")`, over the commits after the earlier bookmark,
selected five commits. `7b113a28a966` was the last of them.

jj moved the bookmark `gh-issue-25` from `4cfac6295881` to `dbfd80c44f94` and
pushed it. `git ls-remote origin refs/heads/gh-issue-25` then printed
`dbfd80c44f94c9c4c73b60ad792dfeb437b14c04`. That commit descends from the
candidate. Its `plugins/grove` has the tree ID of the candidate, and its
`plugins/packs.json` has the blob ID of the candidate. The commit of this leaf
moves the bookmark again, and it changes neither path.

A plain clone of the public repository found the candidate in
`origin/gh-issue-25` only. The public `main` stays at `238c3a839ab4` until
finish.

### The ten Ravel sources

| Source ID | Formula | Repository | Published commit | Source directory | Roots | Tree ID of the source directory | `resourceDigest` |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `grove-core` | `grove` | `https://github.com/Linkuistics/grove.git` | `31fbab3ea93575ab603d2f77992593ab20586e4f` | `plugins/grove` | `skills` | `b9a00084f5f92a76a2376042cd243c91dae28eb1` | `45e466d4a594f729d78e30f67b2f81fad42ee096dd12505d175aa8a6813e27f5` |
| `requirements` | `grove` | `https://github.com/Linkuistics/RequirementsSkills.git` | `c51ec8bebd811c0087ad19a1d66d0aa87bd01386` | `plugins/requirements` | `skills` | `f3cf8128fd206c48d73b061cc5ef6b536d4f9d61` | `07b2d25e8bd20259c71f7419092329368d8ae48bec6ca973343a601a8ec515c8` |
| `design` | `grove` | `https://github.com/Linkuistics/DesignSkills.git` | `aa088156d1a55e28dd9b59f0330f4ab9f1f3d330` | `plugins/design` | `craft`, `skills` | `dd04e268aeb5eba82c527755b1f52bee3565a216` | `d8ad3a2af1b5029bc0de93747f4d7297038651180f6877fbd738d94fcbf6aa14` |
| `engineering` | `grove` | `https://github.com/Linkuistics/EngineeringSkills.git` | `8d1a2bc3e07bec4ce57e0391a6aa778dff63a017` | `plugins/engineering` | `craft`, `skills` | `d8a891cd5c13e9866f7937537afc7979fb3ef0f7` | `7e70b280e7a932489a048d108d4dae2bc63b387cf8fc6b37298663336e0fcc66` |
| `research` | `grove` | `https://github.com/Linkuistics/ResearchSkills.git` | `8f321f2e99e09a8ba5e60fa4a08871727e84df62` | `plugins/research` | `skills` | `79dc2a530cd2d30cd088073423437bb18d61b401` | `7fb3d473f66feafb49afd8846fd94cf3e1fb7b01c584d4ad5f0e54814a8b3fdb` |
| `writing` | `grove` | `https://github.com/Linkuistics/WritingSkills.git` | `578f84d1f5a9bb04573a704bca639530d93b6bca` | `plugins/writing` | `craft`, `skills` | `ce8f4633533f9f550d414a474561a77f92cebba0` | `56bcd07b87f46a7b9ff92eed5a5a128f10d3ce64be2f7d934be898e058782212` |
| `personal` | `personal` | `https://github.com/Linkuistics/PersonalSkills.git` | `cddcfde901cdaf8069e2aac6f3b6bc5aba9fe925` | `plugins/personal` | `craft` | `f66e0792b9507194842e5b7d44457cf12d70dd98` | `dc1f26eafe9b7db6f750c95db38a7876beb29b067d829eaf63203323b89d61b6` |
| `testanyware` | `testanyware` | `https://github.com/Linkuistics/TestAnyware.git` | `1809a49068a23d9d276913f3a90227dbefb98a82`, tag `v2.8.0` | `plugins/testanyware` | `skills` | `0701492a5dae89e5b967e0122a5f1085cccec291` | `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6` |
| `superpowers` | `superpowers` | `https://github.com/obra/superpowers.git` | `bb92a77741419a4ab5f06e711a283343f1ada0c3`, tag `v7.0.0` | The repository root | `skills` | `4ac675daa3415bc836f707c7e60237ac7b66c7d5`, the root tree | `be1ce6e3c25064b83ba2999b37972deb7f73ce88a5f5dbbba990d88037b17230` |
| `quint` | `quint-llm-kit` | `https://github.com/quint-co/quint-llm-kit.git` | `cc75369f741af7d490936f82002c2d28e3b3d78d` | `quint-llm-kit-plugin` | `skills` | `4600ff66782fa87ea4ee8791b37d6fcc86bed5a6` | `0a8b945fbc6732d5b7b3041a966d26cdf718ca884bd5f3fb8435297d7258edef` |

The measurement followed these steps:

1. Clone each repository from the URL of the table, with Git's global and
   system configuration off.
2. Check out the published commit. `git status --porcelain --ignored` printed
   nothing in each clone.
3. Read the tree ID with `git rev-parse <commit>:<source directory>`.
4. Write one settings file that names the ten source directories as `local`
   sources. Each source names the roots that exist in it.
5. Resolve the ten sources with `ravel skills resolve --json` under
   `env -i`, with a new HOME.

The Ravel build was line `skill-directory-delivery` at `ff14cba826b0`, made
again with `task install` into a private prefix. Its `bin/ravel` had the
SHA-256 `3a9a9129573e…` that `scripts/harness-trials/builds.json` pins.

Each published commit lay in a remote branch or tag of its clone. The commit
of `grove-core` lay in `origin/gh-issue-25`. Each other Linkuistics commit lay
in `origin/main`, and the TestAnyware commit also lay in the tag `v2.8.0`. The
superpowers commit lay in the tags `v7.0.0`, `v7.0.1` and `v7.0.2` and in
`origin/main`. The quint-llm-kit commit lay in `origin/main`.

Each digest equals the digest that an earlier leaf recorded. For the six
sources of Grove's keg, that is the candidate keg of
`candidate-keg-k238`. For the four other sources, it is the formula sections
of this record. `git clean -xdff` emptied the clones again after the pack
checks. A second resolution of the same settings then gave the same ten
digests.

This leaf read the two upstream pins again at 06:19 UTC on 2026-11-07:

- In superpowers, the tag `v7.0.0` still peels to
  `bb92a77741419a4ab5f06e711a283343f1ada0c3`. `v7.0.1` peels to
  `2ce8aa15d05b`, and `v7.0.2` peels to `22f0b463e7a5`, which is also the
  head of `main`. The `superpowers` formula pins `bb92a7774141`, version
  `7.0.0`, which is the commit of the owner design.
- In quint-llm-kit, `main` and `HEAD` are still
  `cc75369f741af7d490936f82002c2d28e3b3d78d`, and the repository has no tag.
  The `quint-llm-kit` formula pins that commit, version `1.0.0`.

### The commits of the pack record, the formulae and this record

The tap's `main` is `9ca2b5e3751047b1d81157d43146828e933bcceb`. `jj git fetch`
of the tap found no change, and `git ls-remote origin refs/heads/main` printed
that commit.

| Source | `plugins/packs.json` at the candidate | Formula at tap commit `9ca2b5e37510` | Earlier sections of this record | Difference and reason |
| --- | --- | --- | --- | --- |
| `grove-core` | No entry. Core is the candidate itself | `grove` `22.8.0`, which carries no Ravel source | Grove `238c3a839ab4` as the owner's native plugin | The tap's `grove` formula names the release `22.8.0`, and its archive holds no `share/ravel/skills`. Grove's release inside finish publishes the formula whose keg holds the core of the candidate and the five commits of the pack record. Until then, the candidate keg stands in for that formula |
| `requirements` | `c51ec8bebd81` | Through Grove's release, as above | `9f7278c2d46b` in "RequirementsSkills" | `pack-homebrew-commits-k231` added a commit with a new README. `plugins/requirements` has the tree `f3cf8128fd20` at both commits |
| `design` | `aa088156d1a5` | Through Grove's release | `9d7c7c5368f6` in "DesignSkills" | The same reason. `plugins/design` has the tree `dd04e268aeb5` at both commits |
| `engineering` | `8d1a2bc3e07b` | Through Grove's release | `d98862f21749` in "EngineeringSkills" | The same reason. `plugins/engineering` has the tree `d8a891cd5c13` at both commits |
| `research` | `8f321f2e99e0` | Through Grove's release | `dc3a4812db3b`, the commit of `external-research-pack-k4` | The same reason. `plugins/research` has the tree `79dc2a530cd2` at both commits |
| `writing` | `b9240e0138df` | Through Grove's release | `694140a18dc7`, then `0eacee02f0f2` and `4cafb0fd8e72` in "WritingSkills" | The writing craft and the hooks plugins came after the first commit. `b9240e0138df` changed three files of `plugins/writing`, as "Changes of the commits" states |
| `personal` | No entry. The source has its own formula | `personal` `2026-11-06`, PersonalSkills `cddcfde901cd` | `1ff489d996a9` in "For moves-handoff-k187" of "PersonalSkills" | `cddcfde901cd` changed `authoring-conventions`, as "Changes of the commits" states. The digest `4cb21f4d5b3b…` of the earlier section does not apply to the formula's commit |
| `testanyware` | No entry | `testanyware` `2.8.0`, release commit `1809a49068a2` | `573c61edc63a` in "TestAnyware skill" | The release commit came after the move. `plugins/testanyware` has the tree `0701492a5dae` at both commits |
| `superpowers` | No entry | `superpowers` `7.0.0`, `bb92a7774141` | "The third-party formulae" | None |
| `quint` | No entry | `quint-llm-kit` `1.0.0`, `cc75369f741a` | "The third-party formulae" | None |

The pack record, the formulae and this section therefore name the same
commits. The earlier sections name earlier commits, for the reasons of the
last column.

### Owner-pack entries of the baseline

The enumeration read each line of `docs/owner-skill-inventory.tsv` whose
class is `owner-pack`. Each disposition that names a Ravel source must find
exactly one `<root>/<skill>/SKILL.md` in the clone of that source, or the run
fails. No line failed. Each other line has its own row below the table.

The table names each baseline line by its line number in the TSV file. `C`
is the Claude Code plugin skill, `X` is the link in `~/.codex/skills`, and
`A` is the link in `~/.agents/skills`. The change identity is the skill-tree
digest of `docs/evaluations/owner-skill-delivery/inventory.py`, computed on
the clean clone with the same function:

- **Unchanged** means that each baseline line has the same change identity.
- **Recorded digest** means that an earlier section of this record names the
  new change identity, with its reason.
- Otherwise the last column names the recorded change.

| Skill | Baseline lines | Source and root | Change identity, files | Change |
| --- | --- | --- | --- | --- |
| `configure-grove` | C 17, X 135, A 194 | `grove-core`, `skills` | `180526f7741d02cd`, 3 | Grove's own commits, below the table |
| `grove` | C 18, X 141, A 197 | `grove-core`, `skills` | `2254c63edf784625`, 17 | Grove's own commits. `references/research.md` became the skill `research-family` of `research`. `references/editorial.md` became the skill `editorial-family` of `writing` |
| `grove-finish` | C 24, X 147, A 203 | `grove-core`, `skills` | `b26244ac46849d0e`, 1 | Grove's own commits |
| `grove-integrate-review-planning` | C 28, X 151, A 207 | `grove-core`, `skills` | `2bb2def6a58e039f`, 1 | Grove's own commits |
| `grove-planning` | C 31, X 154, A 210 | `grove-core`, `skills` | `6b2e31a8bf2fbaff`, 1 | Grove's own commits |
| `grove-review-planning` | C 39, X 162, A 218 | `grove-core`, `skills` | `9b478a551ab631aa`, 1 | Grove's own commits |
| `grove-integrate-review-requirements` | C 30, X 153, A 209 | `requirements`, `skills` | `cb3949126191a4cd`, 1 | Recorded digest |
| `grove-requirements` | C 34, X 157, A 213 | `requirements`, `skills` | `9907106d2573fc3a`, 3 | Recorded digest. The new file is `LICENSES/mattpocock-skills.LICENSE` |
| `grove-review-requirements` | C 41, X 164, A 220 | `requirements`, `skills` | `a06d5f1fffb49249`, 1 | Recorded digest |
| `grove-design` | C 22, X 145, A 201 | `design`, `skills` | `78dc6c710f2a7954`, 3 | Recorded digest |
| `grove-integrate-review-design` | C 26, X 149, A 205 | `design`, `skills` | `0165d18ea4e334cf`, 1 | Recorded digest |
| `grove-review-design` | C 37, X 160, A 216 | `design`, `skills` | `444e03aef95b6dc6`, 1 | Recorded digest |
| `cli-tool-design` | C 45, X 126, A 185 | `design`, `craft` | `9f847e8ef1af94a5`, 2 | Unchanged |
| `codebase-design` | C 46, X 127, A 186 | `design`, `craft` | `9919698f17a511ec`, 2 | Unchanged |
| `decision-records` | C 53, X 136, A 195 | `design`, `craft` | `f5ad1eafafcea37b`, 2 | Recorded digest. The new file is `LICENSES/mattpocock-skills.LICENSE` |
| `model-led-development` | C 57, X 166, A 222 | `design`, `craft` | `01ab07d3401d88ef`, 4 | Recorded digest |
| `grove-impl` | C 25, X 148, A 204 | `engineering`, `skills` | `4588bcbbe6eef749`, 2 | Recorded digest |
| `grove-integrate-review-impl` | C 27, X 150, A 206 | `engineering`, `skills` | `2d8e5a70152ee1b4`, 1 | Recorded digest |
| `grove-integrate-review-prototype` | C 29, X 152, A 208 | `engineering`, `skills` | `73d24ee9d17cd58d`, 1 | Recorded digest |
| `grove-prototype` | C 33, X 156, A 212 | `engineering`, `skills` | `53d34980795adc54`, 1 | Recorded digest |
| `grove-review-impl` | C 38, X 161, A 217 | `engineering`, `skills` | `ac3bbccc1243a7f8`, 1 | Recorded digest |
| `grove-review-prototype` | C 40, X 163, A 219 | `engineering`, `skills` | `a9c291f8c6a77d1d`, 1 | Recorded digest |
| `coding-style-bash` | C 47, X 129, A 188 | `engineering`, `craft` | `8ff8f1397a74a9ba`, 1 | Unchanged |
| `coding-style-elixir` | C 48, X 130, A 189 | `engineering`, `craft` | `bceca00f50bb8541`, 1 | Unchanged |
| `coding-style-python` | C 49, X 131, A 190 | `engineering`, `craft` | `38f9464cafeaec58`, 1 | Unchanged |
| `coding-style-rust` | C 50, X 132, A 191 | `engineering`, `craft` | `eaf7196c4c4b028a`, 1 | Unchanged |
| `coding-style-swift` | C 51, X 133, A 192 | `engineering`, `craft` | `f2b020a48bb453f0`, 1 | Unchanged |
| `coding-style-typescript` | C 52, X 134, A 193 | `engineering`, `craft` | `279d0bb61b7b90f9`, 1 | Unchanged |
| `doubt-driven-development` | C 54, X 138, A 196 | `engineering`, `craft` | `071407a9b20c4308`, 4 | Recorded digest. The new file is `LICENSES/addyosmani-agent-skills.LICENSE` |
| `simplify-project` | C 58, X 169, A 226 | `engineering`, `craft` | `cb93a1f191930034`, 1 | Unchanged |
| `using-codebase-memory` | C 60, X 175, A 229 | `engineering`, `craft` | `f66a94d8169a6ce2`, 2 | Unchanged |
| `grove-combine-research` | C 20, X 143, A 199 | `research`, `skills` | `51880f87886d6fba`, 1 | The opening names `grove` by its bare name, as `core-bare-references-k170` settled |
| `grove-research-a` | C 35, X 158, A 214 | `research`, `skills` | `f9afeb30114e9967`, 1 | The opening loads the skill `research-family` in place of `references/research.md`. The external pack contract's section "References between skills" states that opening |
| `grove-research-b` | C 36, X 159, A 215 | `research`, `skills` | `cbd593591fa64a13`, 1 | The same change as `grove-research-a` |
| `grove-art` | C 19, X 142, A 198 | `writing`, `skills` | `98de33804c08a3bb`, 1 | Recorded digest |
| `grove-copy-edit` | C 21, X 144, A 200 | `writing`, `skills` | `70786827a3634bf4`, 1 | Recorded digest |
| `grove-draft` | C 23, X 146, A 202 | `writing`, `skills` | `fb21af42dc743366`, 1 | Recorded digest |
| `grove-proof` | C 32, X 155, A 211 | `writing`, `skills` | `f2032e99a054a049`, 1 | Recorded digest |
| `technical-communication` | C 59, X 172, A 227 | `writing`, `craft` | `b8744e2bfb27219d`, 21 | "Byte changes of the writing craft" and "Hooks plugins of the writing pack" add `.claude-plugin/plugin.json` and `hooks/hooks.json`. They change `.codex-plugin/plugin.json`. "Changes of the commits" changes both manifests' descriptions and a comment of the Pi extension. The baseline's `__pycache__` file was never tracked |
| `writing-code-walkthroughs` | C 62, X 181, A 232 | `writing`, `craft` | `b1e4795388bcf3ab`, 1 | Unchanged |
| `authoring-conventions` | C 44, X 124, A 184 | `personal`, `craft` | `e682db2cb0aefc13`, 1 | "The wording of authoring-conventions" and "Changes of the commits". The description and the opening say "the Linkuistics skill packs", and no installer reads `harnesses:` |
| `machine-environment` | C 56, X 165, A 221 | `personal`, `craft` | `2316befd39ccda08`, 1 | Recorded digest against line 56. Lines 165 and 221 have the new change identity, because the Codex snapshot already held the newer text |
| `using-jujutsu` | C 61, X 177, A 230 | `personal`, `craft` | `6ede7529e799cba1`, 3 | Recorded digest |
| `using-testanyware` | C 100, X 179, A 231 | `testanyware`, `skills` | `8ddb33fe7a999129`, 1 | Unchanged |

Twelve Grove commits between the baseline commit `238c3a839ab4` and the
candidate changed the six core skills. Git's history records each of them.
The moves among them are `57414e00cd8`, `5a6bdd68a7d`, `302f85429e4`,
`84fba18c1f9`, `f45c5a53796` and `31ec9117c9d`. The others are Grove's own
changes, which the core keg delivers:

- `10c67dc5f2f`, `cf9a8ec7b41` and `b9296447aef`: the Ravel 2.0 admission
  and the directory route.
- `fab5336cde8` and `9b8b6924412`: the removal of confinement.
- `7b113a28a96`: the delivery guidance.

Each change of a file count has a recorded reason:

- `grove-requirements`, `decision-records` and `doubt-driven-development` each
  gained one licence file.
- `grove` lost its two family files.
- `technical-communication` gained the two hooks manifests and lost the
  untracked cache file.

The sources also hold two skills with no baseline line, `research-family` and
`editorial-family`. Each was a family file of the baseline `grove` skill.

These owner-pack lines have no Ravel disposition:

| Baseline line | Entry | Route |
| --- | --- | --- |
| 16 | `grove@linkuistics` | The switch disables the plugin. Its skills come from `grove-core` and the five domain sources |
| 43 | `linkuistics@linkuistics` | The switch disables the plugin. Its skills come from `design`, `engineering`, `writing` and `personal`. Its hooks move to `technical-communication-hooks@writing-hooks`, and `guardrail` moves to `guardrail@personal` |
| 55 | `linkuistics:guardrail` | Native only. `guardrail@personal` from the personal keg, as the next section states. No Ravel root holds it |
| 99 | `testanyware@linkuistics` | The switch disables the plugin. Its skill comes from `testanyware`. While it stays enabled beside Ravel 2.0 delivery, the harness sees `using-testanyware` twice |
| 411 | `~/.agents/.grove/snapshots` | Kept until its readers and its writer stop. The current Codex links read it |

### Baseline entries outside the owner packs

Each other line of the baseline has one of the classes below. The class
column of the TSV file holds no other value. Each third-party and stale-copy
line that names a Ravel source found its skill in the clone of that source.

| Class | Baseline locations | Route |
| --- | --- | --- |
| `third-party-pack` | The Claude Code plugins `superpowers@claude-plugins-official` 7.0.0 and `quint-llm-kit@quint-llm-kit` 1.0.0 with their skills. The quint links in `~/.agents/skills`. The tool line `~/.quint-llm-kit` | Each skill comes from `superpowers` or `quint`, and each change identity equals the baseline. The switch disables both plugins and deactivates the three quint links. The superpowers `SessionStart` text becomes the mandatory file `skills/using-superpowers/SKILL.md`. The `~/.quint-llm-kit` clone holds no skill and stays for the `quint-kb` MCP server |
| `stale-copy` | `~/.claude/skills/grove`, and the fourteen superpowers copies in `~/.codex/skills` | The switch deactivates each copy. `grove-core` and `superpowers` supply the current skills. Eleven copies differ from the current skill, as a stale copy can |
| `dangling` | The `terminal-browser` links in `~/.claude/skills`, `~/.codex/skills` and `~/.agents/skills`. The Pi skill links and the Pi extension link into `~/Development/grove` | The switch deactivates each link. No reader can read through it. Pi receives no replacement |
| `tool-managed` | codebase-memory skill copies, agent definitions and hooks in both harnesses. herdr and Orca hooks. `skillOverrides`. Pi extensions of other tools. The `testanyware`, `quint` and `codebase-memory-mcp` executables and the TestAnyware application | Unchanged. Their installers own them |
| `capability-only` | Six enabled Claude Code plugins with no skills, and the MCP entries of both harnesses | Unchanged |
| `platform-managed` | Claude Code built-in and account-synced skills. Codex system skills and the skills of bundled, runtime and remote-installed plugins | Unchanged. No Ravel source holds them |
| `disabled-third-party` | Claude Code plugins that the owner disabled, with their skills | Unchanged, and outside the active collection |
| `not-a-discovery-root` | Plugin caches, marketplace clones, staging folders and the Grove snapshot store | Outside discovery. The caches of the `linkuistics` plugins stay as lazy-read targets until the switch removes those plugins |

### Resources and requirements of each source

Each keg source holds the files of its source directory at the published
commit. The digest covers each file's content, its executable bit and each
link target. The resources that the skills read by relative path are these:

| Source | Resources besides `SKILL.md` | Tool and hook requirements |
| --- | --- | --- |
| `grove-core` | The spine's formats, its `references/` and `LICENSES/`. `configure-grove/references/` | Grove's `grove` and `grove-llm` from the same `grove` formula, Ravel, and jj or Git |
| `requirements` | `grove-requirements/grilling.md` and its `LICENSES/` | The `grove-core` source in the same resolution, because each kind loads the spine |
| `design` | `grove-design/assets/diagram-viewer.html` and `references/`. The `references/` of `cli-tool-design`, `codebase-design` and `model-led-development`. `decision-records/LICENSES/` | The `grove-core` source. `model-led-development` names Alloy and Quint as instruments that a project can choose |
| `engineering` | `grove-impl/LICENSES/`. `doubt-driven-development/LICENSES/` and `references/`. `using-codebase-memory/references/` | The `grove-core` source. The tools of "Tool requirements of the engineering pack". `using-codebase-memory` needs `codebase-memory-mcp` |
| `research` | None | The `grove-core` source |
| `writing` | `technical-communication` holds its always-on text, hooks manifests, harness adapters, `references/`, `scripts/` and `LICENSES/` | The `grove-core` source. `python3` 3.9 or later and a POSIX `/bin/sh`, as "Tool requirements after the removal of the writing craft" states. The always-on hooks need the hooks plugin of the next section |
| `personal` | `using-jujutsu/references/` | mise, Task and the Obsidian manual for `machine-environment`. `jj`, and Git in a colocated repository, for `using-jujutsu` |
| `testanyware` | None | The `testanyware` command and application 2.8.0 and `~/.config/testanyware/config.rhai`. Homebrew installed both on 2026-11-07 |
| `superpowers` | The scripts, prompts, references and examples of nine skills. Nine files have the executable bit, such as `brainstorming/scripts/start-server.sh` and `systematic-debugging/find-polluter.sh` | Git for `using-git-worktrees`. Node.js for `writing-skills/render-graphs.js` and the brainstorming server. The mandatory file replaces the plugin's `SessionStart` hook |
| `quint` | The `guidelines/` of `quint-lang`, and the `examples/` and `guidelines/` of `quint-modeling` | The `quint` command, and the `quint-kb` MCP server from `~/.quint-llm-kit/repo` |

These owner resources stay unchanged until the switch:

- The Claude Code plugins `grove`, `linkuistics` and `testanyware` from
  Grove's `238c3a839ab4`, with their caches. The finish session itself loads
  `grove-finish` and the spine from the enabled `grove` plugin.
- The snapshot store `~/.agents/.grove` and the Codex links into it.
- The superpowers and quint-llm-kit plugins and their Codex copies.
- The tool installations and the MCP entries of the classes above.

### Native residue plugins

| Plugin | Harness identity | Keg and path in the keg | Local marketplace file | Registration |
| --- | --- | --- | --- | --- |
| Technical-communication hooks for Claude Code | `technical-communication-hooks@writing-hooks` | `grove`, `share/ravel/skills/writing/craft/technical-communication` | `.claude-plugin/marketplace.json` at the root of Grove's keg, name `writing-hooks` | Step 5 of the switch, as "The hooks installation inside finish" of the owner design states. `claude plugin marketplace add "$(brew --prefix)/opt/grove"` unless Claude Code already has that path, then `claude plugin install technical-communication-hooks@writing-hooks`, one at a time. The program reads `settings.json` again after each command. Then it disables the plugin. Step 6 enables it |
| `guardrail` for Claude Code | `guardrail@personal` | `personal`, `libexec/plugins/guardrail` | `.claude-plugin/marketplace.json` at the root of the personal keg, name `personal` | Order 6 of the cutover contract. `claude plugin marketplace add "$(brew --prefix)/opt/personal"`, then `claude plugin install guardrail@personal`, then `claude plugin disable guardrail@personal`. Step 6 of the switch enables it |
| Technical-communication hooks for Codex | `technical-communication@writing-hooks` | `grove`, the same path | `.agents/plugins/marketplace.json` at the root of Grove's keg, name `writing-hooks` | None in the switch. The owner design keeps Codex's always-on layer absent. Its installation is a separate owner choice, with the commands of the delivery specification's "Hook adapters" |
| Technical-communication extension for Pi | `~/.pi/agent/extensions/technical-communication.ts` | `grove`, in the `writing` source at `craft/technical-communication/harnesses/pi/` | None | None in the switch. Pi is outside the automatic contract |

The skill-free plugins that hold no Linkuistics skill stay enabled: the
explanatory output style, Context7, Playwright, commit-commands, feature-dev
and Swift LSP.

### References that this grove cannot edit

The sweep searched each `.grove` tree under `~/Development` other than this
grove's own. It used `rg --hidden --no-ignore` with one pattern for the moved
paths: `plugins/linkuistics`, `plugins/testanyware`, `plugins/research`, the
moved `grove-<kind>` skill paths, `references/research.md`,
`references/editorial.md`, the installer, the earlier `AgenticWriting`
project paths, `~/Development/grove/` and `@linkuistics`. The same command
found 185 matches in this grove's own tree, the positive control. Retired
leaves keep their history, so the list below holds only live leaves and node
briefs.

| Tree | File and line | Reference | Current target |
| --- | --- | --- | --- |
| The paused Rhai grove in `Ravel.gh-issue-2`, also checked out in `Ravel.coordinated-release-design` and `Ravel.skill-directory-delivery` | `19-requirements--rhai-book-use-k74.md`, line 467 | `/Users/antony/Development/AgenticWriting/TestSubject` | `/Users/antony/Development/AgenticOrchestration/TestSubject` |
| `grove.gh-issue-14-documentation-skills-for-editorial-pipeline` | `_BRIEF.md`, line 240 | `plugins/grove/skills/grove/references/editorial.md` | The skill `editorial-family` of WritingSkills, `plugins/writing/skills/editorial-family/SKILL.md` |
| The same | `_BRIEF.md`, line 242 | `plugins/grove/skills/grove-design/references/visual-design-discussion.md` | DesignSkills, `plugins/design/skills/grove-design/references/visual-design-discussion.md` |
| The same | `16-k8/09-k13/03-k32/_codex-adapter.md`, line 20 | `plugins/install.sh` | Removed. "The removed installer" of this record states the change |
| The same | `16-k8/09-k13/_evaluation-harness.md`, line 198 | `plugins/install.sh` and `plugins/install.test.sh` | Removed |
| `grove.improve-initial-design-decomposition-skill` | `_BRIEF.md`, lines 253, 255, 258 and 260 | `plugins/linkuistics/skills/codebase-design`, `model-led-development`, `simplify-project` and `plugins/linkuistics/PROVENANCE.md` | `codebase-design` and `model-led-development` in DesignSkills `plugins/design/craft/`. `simplify-project` in EngineeringSkills `plugins/engineering/craft/`. Each pack has its own `PROVENANCE.md` |
| The same | `_BRIEF.md`, line 261 | `plugins/grove/skills/grove-design/SKILL.md` | DesignSkills, `plugins/design/skills/grove-design/SKILL.md`. `grove-planning` stays in core |
| The same | `20-k18/_decomposition-guidance.md`, line 106 | `plugins/linkuistics/skills/authoring-conventions/SKILL.md` | PersonalSkills, `plugins/personal/craft/authoring-conventions/SKILL.md` |
| The same | `20-k18/82-k29/_strategy-catalogue.md`, line 100. `21-k20/12-k42/_catalogue-solutions.md`, line 86. `21-k20/08-k38/_catalogue-coordination.md`, lines 1280, 1364, 1366, 1367, 1407 to 1411, 1447 to 1451 and 2389 | `plugins/linkuistics/skills/decomposition-design/…`, a skill that only that grove's line holds | No destination yet. A design craft skill belongs in DesignSkills `plugins/design/craft/`, as the root brief's destination table states. That grove's rebase decides its place |

Rollout's brief-update text for each paused grove names these references.
The retired leaves of `grove.improve-initial-design-decomposition-skill` hold
the rest of its matches as history. The other paused trees had no match.

### Checks of the handoff

- Each of the six pack commits passed its own `task check` in its clean clone,
  with Git's global and system configuration off. `~/.local/bin` came first
  on the PATH, so `check:plugin` ran `claude plugin validate`. Each plugin
  passed with warnings for the missing `version` and icon.
- `task check:packs` fetched the five domain commits of
  `plugins/packs.json`. It reported "5 recorded pack(s) pass alone and in the
  aggregate".
- The pack checks wrote `.cache/` and a `__pycache__` folder into the clones.
  The enumeration of change identities ran again after `git clean -xdff`, and
  the Ravel digests came from the run before the checks.
- `bash scripts/check.sh` reported "all 18 principal checks pass" on the
  working copy of this leaf. Its `plugins/grove` and `plugins/packs.json`
  equal those of the candidate.

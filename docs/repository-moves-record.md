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
readings agreed, and the session then deleted it.

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

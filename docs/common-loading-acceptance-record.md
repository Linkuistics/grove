# Common loading acceptance record

This record holds the real harness trials of `common-loading-acceptance-k29`.
Each trial runs an actual Claude Code or Codex session on one route of the
[delivery specification](specs/codex-skill-provisioning.md#supported-routes).
Native copies are absent unless the case says otherwise. Each trial adds one
section under [Cases](#cases). The
[dual-Ravel stage record](grove-dual-ravel-stage-record.md) names the builds
and the recipes that these trials use.

The fixtures of earlier stages prove delivery and Grove's interpretation of
it. They do not prove the behaviour of an LLM. This record holds that
behaviour, as the transcripts of real sessions show it.

## Conventions

These rules hold for every section of this record.

- **Section.** A section names its case, its route, its result and its limit.
  The runner writes the facts of the case. The result and the limit are the
  reader's judgement, and the section states them in prose.
- **Route.** Each case names one route: native on 1.1, native on 2.0 with
  delivery disabled, or directory on 2.0. Most cases use directory on 2.0.
- **Builds.** Each section names the Ravel 2.0 build by commit, build identity
  and `bin/ravel` SHA-256. It names the Grove build by commit and by the
  SHA-256 of each executable. If the bookmark `skill-directory-delivery`
  moves, record the new build in `scripts/harness-trials/builds.json` and here
  before any result.
- **Harness.** Each section names the harness, the model, the harness version,
  the effort and the permission mode. It records the effective configuration
  directory, the native startup text and the platform-managed residue, such as
  the claude.ai skills that a sign-in synchronises.
- **Sources.** Each source is a plugin directory of a plain Git checkout of
  this repository, at a recorded commit, outside every working directory of
  the trial. A section records the checkout commit, the `resourceDigest` of
  each source and the descriptor identity.
- **Order.** Each section records the ordered tool calls of the session from
  its transcript, not from the final answer of the agent. The calls show the
  reads of mandatory files, kind skills, craft skills and supporting files,
  and the governed actions.
- **Authority.** The runner launches the driver from an allow-listed
  environment. The live session's `RAVEL_EXIT_FILE`, `GROVE_LAUNCH_DIR`, `CLAUDECODE` and
  `CLAUDE_CODE_*` names never reach a trial with their live values. Each
  launch records a control of the environment that the harness receives.
- **Credit.** A fixture, a prompted retry, a retained variable alone, a
  transcript alone or a pass total gives no credit for agent behaviour. An
  unavailable mechanism is an unmeasured gap, never a pass. A failed required
  case stays a gap for `acceptance-handoff-k106` to resolve.
- **Spend.** A real trial spends subscription usage. The trial tasks are
  opt-in and stay out of `scripts/check.sh`. Only the fake-harness tests of
  the runner join the gate.
- **Transcripts.** This record holds the extracted event order and the native
  startup text. A raw transcript stays out of the repository, because it holds
  account data, such as the organisation of the sign-in. The section records
  the SHA-256 of each raw transcript.

## The runner

`scripts/harness_trial.py` runs one named case. A case is a directory under
`scripts/harness-trials/cases/`. Its `case.json` names the route, the mode, the
kinds that the trial routes, the sources, the mandatory files, the model and
the effort.

A lifecycle case has a `tree/`, which becomes the `.grove/`
directory of the disposable repository. A standalone case has a prompt file
and the names of its outputs. The runner supports the routes native on 2.0 and
directory on 2.0, and the harness Claude Code. A case on native 1.1 needs a `policy.ts`
adapter, and a Codex case needs the Codex adapter of `codex-trial-adapter-k115`.

### What one run does

1. The runner refuses the five Claude Code location variables of Ravel's
   [configuration specification](https://github.com/Linkuistics/Ravel/blob/skill-directory-delivery/docs/specs/configuration-evaluation.md),
   also with an empty value. No harness launches before this check.
2. The runner checks the 2.0 front and both Grove executables against the
   digests of the preparation.
3. The runner builds a sandbox home. The home links only `Library/Keychains`
   on macOS, or `.claude/.credentials.json` on Linux, to the real home. It
   holds the Ravel configuration, the Ravel settings of the route and a jj
   identity for the trial.
4. The runner lists every link in the sandbox home. The list must hold only
   the sign-in store.
5. The runner asks the harness for its version and for `claude auth status`,
   in the allow-listed environment. The reported `configDirectory` must be
   the sandbox `.claude`.
6. The runner makes a jj repository with the case's tree and commits it.
7. The runner launches the Grove driver of the preparation in that repository.
   The environment holds only `HOME`, `TMPDIR`, `PATH`, `SHELL`, `TERM` and
   the user and locale names. The first `PATH` entry is the 2.0 front. The
   Grove prefix, the harness, jj, git and the system directories follow it,
   in that order.
8. After the driver ends, the runner lists the links again and writes the
   case record. It then removes the sign-in link from the trial directory.

A standalone case replaces steps 6 and 7. The runner runs `grove run` with
the case's prompt file and outputs, and with `--ui inline`. Ravel confines
that launch. The harness program is outside every grant of the sandbox, so
the runner grants it read-only with `--runtime-read`.

A confined launch can
write only in its invocation directory. The wrapper therefore prints its
record and the stream of the harness into Grove's run log, under
`.local/state/grove/runs/` of the sandbox home. The runner reads the launch
from that log.

A case with `"probes": true` adds probes to the launch. The wrapper runs its
probes first, and then executes the harness. The wrapper tries to read the
descriptor, each mandatory file, the settings, `config.rhai` and each
mandatory file at its source origin. It tries
to write into each snapshot, into the sandbox home and into its working
directory. Each probe reports `ok` or `failed`.

After the run, the runner
repeats the same probes unconfined, with the same wrapper. A confined failure
belongs to the sandbox only where the unconfined probe succeeds.

A case with `"resolveOffline": true` runs `ravel skills resolve --json` after
the launch, with the same HOME and settings. The resolution runs under the
network-denying Seatbelt profile of `crates/grove/tests/directory_route.rs`,
and its PATH holds only the 2.0 front. Two controls must each be seen to
differ. An unguarded `nc` reaches a local listener, and the guarded `nc`
cannot. `git` is on the PATH of the driver, and not on the PATH of the
resolution. The descriptor identity of the resolution must equal the
descriptor identity of each launch.

The configuration script routes only the case's kinds. It selects the wrapper
`scripts/harness-trials/trial-launch.sh`, which claims a numbered launch
record, writes the request view, the prompt and the environment, and then
execs the harness. Any other kind refuses as `trial_kind_unrouted`. When the
leaf ends with `grove-continue`, the driver therefore stops at the next kind,
and no second harness launches.

### The Claude Code adapter

The adapter runs `claude -p --output-format stream-json --verbose`, with the
model and the effort of the case. It gives each retained snapshot root to
`--add-dir`, as the Rhai sample does. The first event of the stream is the
native startup text. The ordered events come from the session file under the
sandbox `.claude/projects`. Each tool call there carries its absolute paths.

The permission mode is `bypassPermissions`. No read outside the working
directory prompts, and Claude Code denies no tool call, so the transcript shows what
the agent chose to do. This mode is Claude Code's own permission layer. Ravel's
confinement is an operating-system sandbox outside the harness, and this mode
leaves it in force. The default model is `claude-opus-5-5` at effort `medium`.

The extraction gives each tool call one class:

| Class | Meaning |
| --- | --- |
| `mandatory` | A mandatory file of the descriptor |
| `kind-skill` | The `SKILL.md` of the kind that the mandate names, in a retained snapshot |
| `skill` | Another `SKILL.md` in a retained snapshot, such as a craft skill |
| `supporting` | Another file in a retained snapshot |
| `listing` | A directory in a retained snapshot, such as the search directory that a `find` lists |
| `descriptor` | The descriptor file |
| `source-origin` | A file in a configured source directory, not in its snapshot |
| `native` | A file in the sandbox `.claude` or `.codex` |
| `native-skill-tool` | A call of the harness's own Skill tool |
| `action` | An edit or a write |
| `delegate` | A delegated agent |
| `ending` | A Bash call of `end-session`, with its result token |
| `command` | Another Bash call |
| `other` | Any other call |

A Bash call takes the class of the first path in its command that a read class
covers. A `cd` in the command changes the directory against which later
relative paths resolve. A plain assignment `NAME=value` as the first word of
a command sets a variable, and a later `$NAME` or `${NAME}` in the same command
takes its value. A token with any other expansion names no path. The record
shows a long command shortened.

When the harness writes no session file, as a confined harness cannot, the
events come from its stream. The stream holds the same tool calls.
`task trial:reextract TRIAL=<run directory>` extracts the events of an earlier
run again, with the current classifier. It refuses when a transcript digest
changed. On success, the record names the time of the extraction.

### The controls

`task trial:controls` runs `scripts/test_harness_trial.py`, which joins
`scripts/check.sh`. A fake driver and a fake harness stand in for Grove, Ravel
and Claude Code. The tests prove the refusals, the allow-list, the link list,
the authority control and the event extraction. Other tests prove the
standalone mode, the probes, the offline resolution and the second
extraction. Each check also runs once
against a runner with one function broken, and the check must fail there. A
planted `CLAUDE_CODE_PLANTED` name that a broken runner passes to the driver
fails the authority control.

### Recipes

Install the 2.0 build by the stage record's
[recipe](grove-dual-ravel-stage-record.md#the-20-prefix). Then prepare a
scratch directory outside every working copy:

```sh
task trial:prepare OUT="$SCRATCH/trials" COMMIT=<grove commit> RAVEL="$SCRATCH/ravel-2.0/bin/ravel"
```

The preparation fetches the commit by its full hash into a plain Git
checkout, `OUT/grove-source`. It builds `grove` and `grove-llm` from that
checkout into `OUT/grove/bin`. The same checkout supplies the skill sources.
The preparation refuses a front whose digest differs from
`scripts/harness-trials/builds.json`.

Run one case:

```sh
task trial:claude OUT="$SCRATCH/trials" CASE=smoke
```

The run writes `OUT/runs/<case>-<time>/record.json` and `record.md`. The
Markdown file holds the facts that a section of this record copies. The raw
transcripts stay in the sandbox home of the run.

## Cases

<!-- Each trial leaf adds its section here. -->

### smoke

**Case.** `smoke` reaches Claude Code through the runner on the directory
route, with native copies absent. Its one `impl` leaf asks for one file. The
case credits no acceptance case. The run is `smoke-20261102T204531Z`, on
2026-11-03 local time.

**Route.** Directory on 2.0. The one source `grove` is
`OUT/grove-source/plugins/grove` with the root `skills`, and the settings name
no mandatory file.

**Result.** The runner reached Claude Code, and every check of the runner
passed. The sandbox home linked only `Library/Keychains`, before and after the
launch. `claude auth status` reported the sandbox `.claude` as its
configuration directory. The sandbox home held no native copy. The authority
control found no live control name in the environment of the harness.

The agent read the kind skill and the spine from the retained snapshot. It
then wrote the file, retired the leaf, committed it and ran the exact continue
command. The driver then stopped at the `finish` kind, which the trial does
not route. One harness launch occurred.

**Limit.** This run proves the runner, not agent behaviour. It credits no
acceptance case. The case has no mandatory file and no craft skill, and its
prompt names the kind skill. The two sessions before it were development runs
of the runner, and this record credits neither.

| Subject | Value |
| --- | --- |
| Grove build | Commit `a666cde6b79a`. `grove` SHA-256 `5900ff3c08f16439f2d57af2a553b209ffd77527c17e883e6b6c0ee742b2ca94`. `grove-llm` SHA-256 `56f6815a290441d2498b1af4431923d73983459619d5e7eea633f34fad1264b7` |
| Ravel build | Build commit `23e50ce90037`, bookmark `skill-directory-delivery` at `1ca4aecbeae1`. Identity `ef5857e989e6d99da51631ae0295685c8961c1b01bac58d865e6fb789ae71deb`, twice in the executable. `bin/ravel` SHA-256 `8e10e311c50c81ca60ac4f393d17408982d0aafcfe46cccfbc46130465290ae2` |
| Harness | Claude Code 2.1.316, model `claude-opus-5-5`, effort `medium`, permission mode `bypassPermissions` |
| Configuration directory | The sandbox `$TRIAL/home/.claude`. `claude auth status` reported the same directory |
| Source | Checkout commit `a666cde6b79a`. `resourceDigest` of `grove`: `88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67` |
| Descriptor identity | `749bef35127961c43a6c09742def75cc1d781640d3d8b086d2e122a748c0b323`, equal to the SHA-256 of `directories.json` |
| Links | Before and after: `Library/Keychains` only |
| Driver environment | `HOME`, `LANG`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `TMPDIR`, `USER` |
| Harness environment | The driver's names, `PWD`, `SHLVL`, `_`, and the trial's own `GROVE_LAUNCH_DIR`, `RAVEL_EXIT_FILE` and `RAVEL_SKILL_DIRECTORIES`. No live value and no `CLAUDE*` name |
| Native copies | None |
| Synced claude.ai skills | None in this run. See the observations |
| Raw transcript | SHA-256 `a91c6d30f3333bbf4ce9b36d5eb822aa5b212d400270a71320361bc6a18aa2d3`. Stream SHA-256 `c8fa23394081df9097ef1f0cfffc93feebdcec591d288ba238da265fa0360889` |

Native startup text, from the init event of the stream and the skill listing of the transcript:

```json
{
  "claude_code_version": "2.1.316",
  "model": "claude-opus-5-5",
  "permissionMode": "bypassPermissions",
  "cwd": "$TRIAL/repo",
  "tools": [
    "Task",
    "Bash",
    "CronCreate",
    "CronDelete",
    "CronList",
    "DesignSync",
    "Edit",
    "EnterWorktree",
    "ExitWorktree",
    "ListAgents",
    "Monitor",
    "NotebookEdit",
    "PushNotification",
    "Read",
    "RemoteTrigger",
    "ReportFindings",
    "ScheduleWakeup",
    "SendMessage",
    "Skill",
    "TaskStop",
    "ToolSearch",
    "WebFetch",
    "WebSearch",
    "Workflow",
    "Write",
    "mcp__claude_ai_Claude_Docs__batch",
    "mcp__claude_ai_Claude_Docs__create",
    "mcp__claude_ai_Claude_Docs__delete",
    "mcp__claude_ai_Claude_Docs__export",
    "mcp__claude_ai_Claude_Docs__guide",
    "mcp__claude_ai_Claude_Docs__query",
    "mcp__claude_ai_Claude_Docs__read",
    "mcp__claude_ai_Claude_Docs__update"
  ],
  "mcp_servers": [
    {
      "name": "claude.ai Claude Docs",
      "status": "connected",
      "source": "claudeai"
    },
    {
      "name": "claude.ai Google Calendar",
      "status": "needs-auth",
      "source": "claudeai"
    },
    {
      "name": "claude.ai Gmail",
      "status": "needs-auth",
      "source": "claudeai"
    },
    {
      "name": "claude.ai Google Drive",
      "status": "needs-auth",
      "source": "claudeai"
    }
  ],
  "slash_commands": [
    "deep-research",
    "design",
    "design-sync",
    "dataviz",
    "update-config",
    "verify",
    "debug",
    "code-review",
    "simplify",
    "batch",
    "fewer-permission-prompts",
    "doctor",
    "loop",
    "schedule",
    "claude-api",
    "workflow-authoring",
    "run",
    "run-skill-generator",
    "plugin-authoring",
    "advisor",
    "agents",
    "auto-mode-setup",
    "autocompact",
    "clear",
    "color",
    "compact",
    "config",
    "output-style",
    "context",
    "effort",
    "fast",
    "focus",
    "heapdump",
    "init",
    "mcp",
    "import",
    "model",
    "__remote-workflow",
    "workflow-launch-exec",
    "reload-plugins",
    "reload-skills",
    "rename",
    "ultrareview",
    "security-review",
    "usage",
    "insights",
    "recap",
    "skill-doctor",
    "goal",
    "design-consent",
    "design-revoke",
    "list-agents",
    "team-onboarding"
  ],
  "skills": [
    "deep-research",
    "design",
    "design-sync",
    "dataviz",
    "update-config",
    "verify",
    "debug",
    "code-review",
    "simplify",
    "batch",
    "fewer-permission-prompts",
    "doctor",
    "loop",
    "schedule",
    "claude-api",
    "workflow-authoring",
    "run",
    "run-skill-generator",
    "plugin-authoring"
  ],
  "plugins": [
    {
      "name": "cc-plugin-agents-md",
      "path": "builtin",
      "source": "cc-plugin-agents-md@builtin"
    },
    {
      "name": "cc-plugin-telemetry",
      "path": "builtin",
      "source": "cc-plugin-telemetry@builtin"
    },
    {
      "name": "cc-plugin-plugin-authoring",
      "path": "builtin",
      "source": "cc-plugin-plugin-authoring@builtin"
    }
  ],
  "agents": [
    "claude",
    "Explore",
    "general-purpose",
    "Plan",
    "statusline-setup"
  ],
  "output_style": "default",
  "apiKeySource": "none",
  "skillListing": {
    "skillCount": 14,
    "names": [
      "dataviz",
      "update-config",
      "keybindings-help",
      "code-review",
      "simplify",
      "fewer-permission-prompts",
      "loop",
      "schedule",
      "claude-api",
      "workflow-authoring",
      "run",
      "plugin-authoring",
      "init",
      "security-review"
    ]
  },
  "attachmentTypes": [
    "agent_listing_delta",
    "auto_mode",
    "credential_org",
    "date",
    "deferred_tools_delta",
    "environment",
    "mcp_instructions_delta",
    "model",
    "prompt_snapshot",
    "remote_session_change",
    "session_context",
    "skill_listing"
  ]
}
```


Ordered events. `$TRIAL` is the trial directory and `$RAVEL2` the 2.0 prefix:

| # | Tool | Class | Skill | Command |
| --- | --- | --- | --- | --- |
| 1 | Bash | listing |  | `cd "$TRIAL/home/.cache/ravel/skills/v1/snapshots/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills" && find . -name SKILL.md \| while read f; do echo "== $f"; head -8 "$f"; d…` |
| 2 | Bash | kind-skill | grove-impl | `cd "$TRIAL/home/.cache/ravel/skills/v1/snapshots/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills" && cat grove-impl/SKILL.md; ls -R grove-impl` |
| 3 | Bash | skill | grove | `cd "$TRIAL/home/.cache/ravel/skills/v1/snapshots/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills/grove" && cat SKILL.md; find . -type f` |
| 4 | Bash | supporting | grove | `cd "$TRIAL/home/.cache/ravel/skills/v1/snapshots/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills/grove/references" && cat bootstrap.md execute.md` |
| 5 | Bash | command |  | `cd "$TRIAL/repo" && P=$(grove-llm resolve smoke-file-k1); echo "resolved: $P"; grove-llm brief-chain "$P"; ls -la; find .grove -type f` |
| 6 | Bash | command (error) |  | `cat .grove/_BRIEF.md; echo ======; cat .grove/01-impl--smoke-file-k1.md; echo =====; jj st; jj log -n 5` |
| 7 | Bash | command |  | `echo "---- TASK"; cat .grove/01-impl--smoke-file-k1.md; echo "---- JJ"; jj st; jj log -n 5` |
| 8 | Bash | supporting | grove | `printf 'smoke\n' > smoke.txt; cat smoke.txt; cd "$TRIAL/home/.cache/ravel/skills/v1/snapshots/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills/grove" && cat references/reti…` |
| 9 | Bash | command |  | `cd "$TRIAL/repo" && grep -rn "smoke-file-k1" .grove; grove-llm leaf-retire .grove/01-impl--smoke-file-k1.md && jj commit -m "smoke-file-k1  Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>" && …` |
| 10 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |

Ending outcome:

- Ravel result: the agent ran `end-session grove-continue` once. The driver
  continued to the next leaf, so it received that result.
- Leaf state: `01-impl--smoke-file-k1.md` became
  `01-DONE-impl--smoke-file-k1.md`. The driver then added
  `02-finish--finish-k2.md`, and Ravel refused that launch as
  `config_refused`, because the trial routes only `impl`.
- Teardown record: none. The driver stopped with exit status 3 from the
  refused finish launch.
- Published output: the commit `d5dfba0908e7` `smoke-file-k1`, which adds
  `smoke.txt` and retires the leaf. The working copy above it holds the finish
  leaf and has no description.
- Harness launches: 1. The driver ran 51.6 seconds.

Observations:

- **Synced skills differ between runs.** The development run before this one
  received 13 claude.ai skills into `.claude/skills/synced` before startup.
  Its skill listing held 27 names, 13 of them `anthropic-skills:*`. This run
  received none, and its listing held 14 built-in names. Each discovery trial
  must therefore record the residue and the listing of its own run.
- **The account adds tools.** The startup lists claude.ai connectors, such as
  Claude Docs, Gmail and Google Drive, and three built-in plugins. These come
  from the sign-in, not from the sandbox home.
- **The agent reads through Bash.** Each read in this session was a `cat`
  after a `cd` into the snapshot. The extraction resolves those paths. One
  Bash call can also hold a governed action, as call 8 writes `smoke.txt`
  before it reads `retire.md`. A trial that measures order must read the
  command of such a call, not only its class.
- **Ravel reports a failed stop.** After the session ended, Ravel wrote two
  lines `stop attempt … signal 9: Operation not permitted (os error 1)` for
  the managed group. The session result still reached the driver. This record
  notes the lines for the Ravel line, and does not interpret them.

### discovery

**Case.** `discovery` measures implicit craft discovery and the read order of
`claude-discovery-trial-k108`. Its one `impl` leaf asks for a Bash script
`stamp.sh` that writes the current UTC time to a file. The task names no
skill, and Grove sends its bare-name mandate. In the supplied directories, only
`coding-style-bash` matches the activity.

The run is
`discovery-20261102T210714Z`, on 2026-11-03 local time. It was the first and
only run of the case, with no retry and no further prompt.

**Route.** Directory on 2.0. One plain Git checkout at commit `3bfb096810db`
supplies two sources. `grove` is `OUT/grove-source/plugins/grove` and
`linkuistics` is `OUT/grove-source/plugins/linkuistics`, each with the root
`skills`. No checkout tool made the checkout. The settings name two mandatory
files in this order:

1. `linkuistics` `skills/using-jujutsu/SKILL.md`.
2. `grove` `skills/grove/SKILL.md`, the Grove spine.

The kind skill directs the agent to the spine. The configured order puts
another file before the spine, so a matching read order shows that the agent
followed the configuration.

**Result.** The case passes each observation of the leaf.

- **Mandatory order.** Calls 1 and 2 read the two mandatory files from their
  snapshots, in the configured order. Both reads came before the kind-skill
  read in call 4, which is the caller's "first" action. The mandate qualifies
  that action with "after any mandatory guidance".
- **Supporting read.** Calls 5 and 6 read `references/bootstrap.md` and
  `references/execute.md` of the spine. Each path is relative to the spine's
  own snapshot directory. The spine names `references/execute.md` as a
  relative link.
- **Listing.** Call 3 listed every `SKILL.md` path in both snapshots with
  `find`. It also searched for the `name:` line of `grove-impl`.
- **Craft read.** Call 9 read `coding-style-bash/SKILL.md` from the
  `linkuistics` snapshot. The agent first read the task in call 8. Call 10
  then wrote `stamp.sh`, which is the governed action. The script follows the
  craft skill: it has the `#!/usr/bin/env bash` shebang, `set -euo pipefail` and
  `IFS=$'\n\t'`. Call 11 ran `shellcheck` and `shfmt -i 2 -ci`, which the
  craft skill names.
- **Native absence.** The sandbox home held no native copy and no synced
  claude.ai skill. The native skill listing held 14 built-in names, and none
  of them is a craft skill.
- **Offline resolution.** `ravel skills resolve --json` exited 0 with status
  `resolved`. It ran under the network-denying profile, with only the 2.0
  front on PATH. It gave the descriptor identity of the launch,
  `29d0d3331f61…`. Both controls differed. The unguarded `nc` reached the
  local listener with exit 0, and the guarded `nc` failed with exit 1. `git`
  was on the PATH of the driver at `/opt/homebrew/bin/git`, and absent from
  the PATH of the resolution.

The agent then retired the leaf, committed it and ran the exact continue
command. The driver stopped at the `finish` kind, which the trial does not
route.

**Limit.** The listing in call 3 shows skill names only through their
directory names. The transcript shows no inspection of descriptions before
call 9. The agent therefore chose `coding-style-bash` by its directory name,
and then read its whole file. One run is one observation, not a rate.

The harness needs the network for its model, so the no-network check covers
Ravel's resolution only. The fixture `a_directory_launch_needs_no_network_and_no_git`
proves a launch without a network.

Call 5 printed `bootstrap.md` and then
returned exit status 1. The transcript does not show the cause. Call 6 read
`execute.md` alone, and that read succeeded.

The first record of this run classified calls 1, 2, 4, 5, 6, 9 and 12 as
commands. Those calls name their paths through a shell variable that the same
command assigns, and the classifier then skipped such tokens. The repaired
classifier resolves those assignments. `task trial:reextract` extracted the
events again from the same transcript, whose digest did not change.

| Subject | Value |
| --- | --- |
| Grove build | Commit `3bfb096810db`. `grove` SHA-256 `5900ff3c08f16439f2d57af2a553b209ffd77527c17e883e6b6c0ee742b2ca94`. `grove-llm` SHA-256 `56f6815a290441d2498b1af4431923d73983459619d5e7eea633f34fad1264b7`. Both equal the smoke build of `a666cde6b79a` |
| Ravel build | Build commit `23e50ce90037`, bookmark `skill-directory-delivery` at `1ca4aecbeae1`. Identity `ef5857e989e6d99da51631ae0295685c8961c1b01bac58d865e6fb789ae71deb`, twice in the executable. `bin/ravel` SHA-256 `8e10e311c50c81ca60ac4f393d17408982d0aafcfe46cccfbc46130465290ae2` |
| Harness | Claude Code 2.1.316, model `claude-opus-5-5`, effort `medium`, permission mode `bypassPermissions` |
| Configuration directory | The sandbox `$TRIAL/home/.claude`. `claude auth status` reported the same directory |
| Sources | Checkout commit `3bfb096810db`. `resourceDigest` of `grove`: `88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67`. `resourceDigest` of `linkuistics`: `820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22` |
| Descriptor identity | `29d0d3331f616eb3d8d2757c158fd61cd8725127949a5443bdeeab90c6e2ead6`, equal to the SHA-256 of `directories.json` and to the identity of the offline resolution |
| Links | Before and after: `Library/Keychains` only |
| Driver environment | `HOME`, `LANG`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `TMPDIR`, `USER` |
| Harness environment | The driver's names, `PWD`, `SHLVL`, `_`, and the trial's own `GROVE_LAUNCH_DIR`, `RAVEL_EXIT_FILE` and `RAVEL_SKILL_DIRECTORIES`. No live value and no `CLAUDE*` name |
| Native copies | None |
| Synced claude.ai skills | None |
| Raw transcript | SHA-256 `28fd98d0dc3fadf9821abec9aa895f75c69695be51553b155459e0b0a6b75bb2`. Stream SHA-256 `831f10a4a9867b5065505193ca0c064f94d7744897968e822266e5b0e6dcb319` |

Native startup text: equal to the text of the [smoke](#smoke) section in every
field except `cwd`. That includes the tools, the connectors, the built-in
plugins, the agents and the skill listing of 14 names.

Ordered events. `$TRIAL` is the trial directory and `$RAVEL2` the 2.0 prefix.
`$SNAPS` is `$TRIAL/home/.cache/ravel/skills/v1/snapshots`. `$G` is
`$SNAPS/88f68c31…/tree`, the `grove` snapshot, and `$L` is
`$SNAPS/820fe282…/tree`, the `linkuistics` snapshot:

| # | Tool | Class | Skill | Command |
| --- | --- | --- | --- | --- |
| 1 | Bash | mandatory | using-jujutsu | `S1="$L/skills"; cat "$S1/using-jujutsu/SKILL.md"; ls -R "$S1/using-jujutsu"` |
| 2 | Bash | mandatory | grove | `S2="$G/skills"; cat "$S2/grove/SKILL.md"; ls -R "$S2/grove"` |
| 3 | Bash | listing |  | `cd $SNAPS; find 88f68c31…/tree/skills 820fe282…/tree/skills -name SKILL.md \| sort; echo ---; grep -rl "^name: grove-impl$" */tree/skills --include=SKILL.md` |
| 4 | Bash | kind-skill | grove-impl | `S2="$G/skills"; cat "$S2/grove-impl/SKILL.md"; ls -R "$S2/grove-impl"` |
| 5 | Bash | supporting (error) | grove | `R="$G/skills/grove/references"; cat "$R/bootstrap.md"; echo =====; cat "$R/execute.md"` |
| 6 | Bash | supporting | grove | `R="$G/skills/grove/references"; cat "$R/execute.md"` |
| 7 | Bash | command |  | `cd $TRIAL/repo; which grove-llm; P=$(grove-llm resolve stamp-script-k1); echo "$P"; grove-llm brief-chain "$P"` |
| 8 | Bash | command |  | `cd $TRIAL/repo; cat .grove/_BRIEF.md; echo "-----"; cat .grove/01-impl--stamp-script-k1.md; echo "-----"; ls -la . .grove; jj --no-pager st; jj --no-p…` |
| 9 | Bash | skill | coding-style-bash | `S1="$L/skills"; cat "$S1/coding-style-bash/SKILL.md"; ls -R "$S1/coding-style-bash"` |
| 10 | Write | action |  | `$TRIAL/repo/stamp.sh` |
| 11 | Bash | command |  | `cd $TRIAL/repo; chmod +x stamp.sh; ./stamp.sh; echo "exit=$?"; cat STAMP; wc -l < STAMP; grep -Ecx '[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]…` |
| 12 | Bash | supporting | grove | `R="$G/skills/grove/references"; cat "$R/retire.md"; echo; echo "#####"; cat "$R/commit.md"` |
| 13 | Bash | command |  | `cd $TRIAL/repo; grep -rn "Reviews:" .grove \|\| echo "no reviews"; grove-llm leaf-retire .grove/01-impl--stamp-script-k1.md && ls .grove && jj describ…` |
| 14 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |

Ending outcome:

- Ravel result: the agent ran `end-session grove-continue` once, and the
  driver continued to the next leaf.
- Leaf state: `01-impl--stamp-script-k1.md` became
  `01-DONE-impl--stamp-script-k1.md`. The driver then added
  `02-finish--finish-k2.md`, and Ravel refused that launch as
  `config_refused`, because the trial routes only `impl`.
- Teardown record: none.
- Published output: the commit `7b13757a3805`, which adds `stamp.sh` and
  retires the leaf. The working copy above it, `d5728721dc2f`, holds the
  finish leaf and has no description.
- Harness launches: 1. The driver ran 62.3 seconds and exited 0.
- Ravel again wrote two lines `stop attempt … signal 9: Operation not
  permitted (os error 1)`, as in the smoke run.

### discovery-confined

**Case.** `discovery-confined` runs the task of `discovery` through a confined
`grove run`, with the same sources and mandatory files. Its prompt asks for the
same script in the invocation directory, and its one output is `stamp.sh`. The
run is `discovery-confined-20261102T211126Z`, on 2026-11-03 local time.

**Route.** Directory on 2.0, through `grove run impl --prompt-file … --output
OUT/…/out/stamp.sh --runtime-read <claude> --ui inline`. Ravel 2.0 confined the
launch with its macOS Seatbelt backend. The `--runtime-read` grant is the
Claude Code executable,
`~/.local/share/claude/versions/2.1.316`.

**Result.** **Gap.** Claude Code could not run under Ravel's confinement. The
wrapper ran inside the sandbox and executed Claude Code. Claude Code then
wrote `ENOENT: no such file or directory, open '/tmp/claude-501'` and exited 1,
before its stream wrote the init event.

Claude Code uses its own temporary
directory `/tmp/claude-<uid>`, and the sandbox grants no access to that
directory. Ravel then refused as `session_result_missing`, and Grove published
no output. The agent read nothing, so this run gives no evidence of agent
behaviour under confinement. As the brief directs, no unconfined retry
replaced it.

The probes of the same launch measure the permissions. They ran inside the
sandbox, before the wrapper executed Claude Code. The runner then repeated them unconfined:

| Access | Path | Inside | Unconfined |
| --- | --- | --- | --- |
| read | The descriptor `directories.json` | ok | ok |
| read | Mandatory `using-jujutsu/SKILL.md` in the `linkuistics` snapshot | ok | ok |
| read | Mandatory `grove/SKILL.md` in the `grove` snapshot | ok | ok |
| write | `trial-probe` in the `grove` snapshot | failed | failed |
| write | `trial-probe` in the `linkuistics` snapshot | failed | failed |
| read | `$TRIAL/home/.config/ravel/settings.json` | failed | ok |
| read | `$TRIAL/home/.config/ravel/config.rhai` | failed | ok |
| read | `using-jujutsu/SKILL.md` at its source origin | failed | ok |
| read | `grove/SKILL.md` at its source origin | failed | ok |
| write | `$TRIAL/home/trial-probe-outside` | failed | ok |
| write | `trial-probe-inside` in the working directory | ok | ok |

The required resources outside the working directory were readable inside
the sandbox. The protected configuration, the source origins and a write
outside the working directory failed inside the sandbox, and each succeeded
unconfined. The sandbox therefore caused those failures. A snapshot write
failed in both places, because Ravel seals each snapshot read-only by file
mode. That row is therefore no evidence about the sandbox.

**Limit.** The probes are the wrapper's shell accesses, not the agent's tool
calls. They prove permissions only. The case leaves the leaf's confined
observation of agent reads unmet, and `acceptance-handoff-k106` resolves the
gap.

A second blocker probably follows the temporary directory. The Seatbelt
profile grants no Mach lookup of the Keychain service, so Claude Code probably
cannot read its claude.ai sign-in. That is an inference from the profile text
in `crates/keyed-launch/src/confinement.rs` of the Ravel line, and no run
verified it.

| Subject | Value |
| --- | --- |
| Builds | As in `discovery`: Grove `3bfb096810db`, Ravel build `23e50ce90037`, `bin/ravel` SHA-256 `8e10e311…` |
| Harness | Claude Code 2.1.316, model `claude-opus-5-5`, effort `medium`, permission mode `bypassPermissions`. `claude auth status`, unconfined, reported the sandbox `.claude` and a sign-in |
| Sources | As in `discovery`, with the same two `resourceDigest` values |
| Descriptor identity | `97fbbc9f09d88b1d56093921630aeadacf9901bfd5f27399c6fbc14a8fbcf323`. It differs from `discovery`, because the snapshot paths lie in another sandbox home |
| Links | Before and after: `Library/Keychains` only |
| Harness environment | `HOME`, `LANG`, `LOGNAME`, `PATH`, `PWD`, `SHLVL`, `TEMP`, `TMP`, `TMPDIR`, `USER`, `_`, and the trial's own `RAVEL_EXIT_FILE` and `RAVEL_SKILL_DIRECTORIES`. `TMPDIR` names Ravel's private run directory. No live value and no `CLAUDE*` name |
| Native copies | None |
| Synced claude.ai skills | None |
| Stream | Empty. SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

Grove's run log, without the stream and the wrapper's lines:

```text
ravel: starting provider anthropic, model claude-opus-5-5, effort medium for kind "impl"
ENOENT: no such file or directory, open '/tmp/claude-501'
ravel: stop attempt 1 target -27560 signal 9: Operation not permitted (os error 1)
ravel: stop attempt 2 target -27560 signal 9: Operation not permitted (os error 1)
ravel: refused (session_result_missing, stage exec): the managed process ended without a result (exit status: 1)
remedy: have the managed process call ravel end-session RESULT before it exits
```

Ending outcome:

- Ravel result: none. Ravel refused as `session_result_missing`.
- Grove: `grove run` exited 1 after 2.7 seconds and published no output.
- Teardown record: none. A standalone run has no tree.
- Harness launches: 1.

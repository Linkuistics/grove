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
directory of the disposable repository. In a tree file, `{{TRIAL}}` becomes
the trial directory, so a task can name a concrete path of its own trial. A
standalone case has a prompt file and the names of its outputs.

An explicit-route case can name three more fields. Each field changes only the
trial directory, never the owner's home:

| Field | Effect |
| --- | --- |
| `native` | Copies a skill directory of the checkout into the sandbox `.claude/skills/<name>`. The record names each copy and the SHA-256 of its `SKILL.md`. With `commit`, the copy holds the directory as that older commit of the checkout held it. The record then also names the commit and the SHA-256 of the current `SKILL.md` |
| `claudeSettings` | Writes the sandbox `.claude/settings.json`, such as a deny rule for one skill |
| `plant` | Copies a file of the checkout under the trial's `guidance/`, outside the repository and every snapshot |

The native copies in the sandbox home must equal the copies that the case
declares. Any other native copy fails the run.

A case can also expect a refusal before any launch. It names `moveAway` with a
source and `expectRefusal` with Ravel's code. The runner fetches the prepared
commit into a plain Git checkout under the trial's `sources/`. The settings
name that checkout for the source, so the move changes no checkout that
another case reads. The runner moves the checkout away before the driver and
restores it after the driver.

A direct `ravel skills resolve --json` runs with the source moved away and
again with the source restored. Neither launches a worker. The run fails when
a harness launched or wrote a session file, or when the driver does not report
the code. It also fails when the moved resolution does not give the code or the
restored resolution does not resolve, or when the tree changed. The restored
resolution shows that the move caused the refusal.

The runner supports the routes native on 2.0 and directory on 2.0, and the
harness Claude Code. A case on native 1.1 needs a `policy.ts`
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
| `planted` | A file under the trial's `guidance/`, which a case plants for a concrete path |
| `native-skill-tool` | A call of the harness's own Skill tool. The event keeps the first 300 characters of the result |
| `action` | An edit or a write |
| `delegate` | A delegated agent |
| `ending` | A Bash call of `end-session`, with its result token |
| `command` | Another Bash call |
| `other` | Any other call |
| `text` | A text block of the agent, such as a disclosure or a report |

A read in a snapshot or in a source origin also names its source by the id of
the settings. Two sources can hold skills of one name, and the source shows
which copy a read opened.

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
extraction. Further tests prove the source of each read, a native copy from
an older commit and a refusal case with a moved source.

Each check also runs once
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

### Explicit request rows

**Cases.** Eight cases measure the request table in the "Agent discovery and
native installations" section of Ravel's `docs/specs/skill-discovery.md`, for
`claude-explicit-routes-k109`. They also measure the Directory bootstrap and
Explicit loading mechanism rows of the
[delivery specification](specs/codex-skill-provisioning.md#test-seams).

Each case is one launch of one Grove `impl` leaf, `utc-record-k1`. The leaf
asks for one decision record under `docs/adr/`. Its Guidance section makes one
request for the craft skill `decision-records`. Only that request, the native
copies, the sandbox settings and the planted file differ between the cases.
Each case ran once on 2026-11-03 local time, with no retry and no further
prompt.

`decision-records` has no `paths` metadata. A native copy of a skill with
`paths`, such as `coding-style-bash`, can stay out of Claude Code's skill
listing until the agent touches a matching file. A "native match" case could then
measure "no match".

**Route.** Directory on 2.0 for each case. One plain Git checkout at commit
`b63ca828d3ef` supplies the sources `grove` and `linkuistics`, each with the
root `skills`. The settings name the mandatory files of
[discovery](#discovery), in the same order: `using-jujutsu`, then the Grove
spine.

**The loader error.** The case `explicit-native-error` needs a native loader
error other than "no match". The sandbox `.claude/settings.json` holds the deny
rule `Skill(decision-records)`, and a native copy of the skill exists. The
strings of the Claude Code 2.1.316 executable show this check in the input
validation of the Skill tool. The check gives the message "Skill execution
blocked by permission rules" with error code 11, and the permission mode does
not change it. The run then showed that message.

The `disable-model-invocation: true` refusal of the same tool, code 4, was
rejected. Its own message tells the agent not to replicate the skill by other
means. A "no bypass" result would then come from Claude Code's text, not from
Ravel's preamble.

**Results.**

| Case | Request in the task | Row | Result |
| --- | --- | --- | --- |
| `explicit-bare` | "Load the skill `decision-records`" | A skill name | **Pass** |
| `explicit-plugin` | "Load the skill `linkuistics:decision-records`" | A skill name with a plugin namespace | **Pass** |
| `explicit-native-match` | "Use the Skill tool to load `decision-records`", with a native copy | A native tool, with a match | **Pass** |
| `explicit-native-absent` | The same request, with native copies absent | A native tool, with no match | **Pass** |
| `explicit-native-error` | The same request, with a native copy and the deny rule | A native error other than "no match" | **Gap.** No bypass, and no report |
| `explicit-native-only` | "Load `decision-records` only with the Skill tool", with native copies absent | An express restriction | **Gap.** No fallback, and no report |
| `explicit-path` | "Read the guidance file `$TRIAL/guidance/decision-records/SKILL.md`", which exists | A concrete file path | **Pass** |
| `explicit-path-missing` | The same request, with no file at that path | A concrete file path that fails | **Pass** for the row. The session ran no ending command |

**Limit.** Each case is one observation, not a rate. The activity of the task
also matches `decision-records`. A directory read in a name case therefore
shows the route of the read. It does not prove that the request alone caused
the read.

Facts that hold for all eight cases:

| Subject | Value |
| --- | --- |
| Grove build | Commit `b63ca828d3ef`. `grove` SHA-256 `5900ff3c08f16439f2d57af2a553b209ffd77527c17e883e6b6c0ee742b2ca94`. `grove-llm` SHA-256 `56f6815a290441d2498b1af4431923d73983459619d5e7eea633f34fad1264b7`. Both equal the smoke build |
| Ravel build | Build commit `23e50ce90037`, bookmark `skill-directory-delivery` at `1ca4aecbeae1`. Identity `ef5857e989e6d99da51631ae0295685c8961c1b01bac58d865e6fb789ae71deb`, twice in the executable. `bin/ravel` SHA-256 `8e10e311c50c81ca60ac4f393d17408982d0aafcfe46cccfbc46130465290ae2`. The prefix is a byte copy of the `discovery` prefix, and the preparation checked its digest |
| Harness | Claude Code 2.1.316, model `claude-opus-5-5`, effort `medium`, permission mode `bypassPermissions` |
| Configuration directory | The sandbox `$TRIAL/home/.claude`. `claude auth status` reported the same directory |
| Sources | Checkout commit `b63ca828d3ef`. `resourceDigest` of `grove`: `88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67`. `resourceDigest` of `linkuistics`: `820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22`. Both equal the `discovery` values |
| Links | Before and after: `Library/Keychains` only |
| Driver environment | `HOME`, `LANG`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `TMPDIR`, `USER` |
| Harness environment | The driver's names, `PWD`, `SHLVL`, `_`, and the trial's own `GROVE_LAUNCH_DIR`, `RAVEL_EXIT_FILE` and `RAVEL_SKILL_DIRECTORIES`. No live value and no `CLAUDE*` name |
| Native copy | Only in `explicit-native-match` and `explicit-native-error`: `.claude/skills/decision-records`, from `plugins/linkuistics/skills/decision-records` of the checkout. `SKILL.md` SHA-256 `6b9b8d456b3a67004c1910d3458821221695b4c45c77b0a3f78b4e5b0c68f5a4`. The other six cases had no native copy |
| Planted file | Only in `explicit-path`: `$TRIAL/guidance/decision-records/SKILL.md`, a copy of the same `SKILL.md`, with the same SHA-256 |
| Synced claude.ai skills | 13 `anthropic-skills:*` skills in `explicit-plugin` and `explicit-native-match`, none in the others. No synced skill has the name `decision-records` |

Native startup text: equal to the text of the [smoke](#smoke) section, except
in four fields. `cwd` names each trial's repository. A native copy adds
`decision-records` to `skills`, `slash_commands` and the skill listing. The
synced skills add their 13 names to the same fields. In `explicit-bare`, the
Claude Docs connector was `pending` at startup, so its eight tools are absent
from `tools`.

Each table below uses the abbreviations of [discovery](#discovery). `$G` is the
`grove` snapshot and `$L` the `linkuistics` snapshot, each up to its `tree`.
A text row is a text block of the agent, shortened to 160 characters.

Observations that hold for several cases:

- **Grove's own plugin-qualified name.** On Claude Code, Grove's mandate names
  the kind skill as `grove:grove-impl`. In all eight runs, the agent read
  `grove-impl/SKILL.md` from the `grove` snapshot. It never called the Skill
  tool for the kind skill. That is one more observation of the namespace row,
  with native copies of the kind absent.
- **The shell stops at `=====`.** The allow-listed `SHELL` is `/bin/zsh`. Zsh
  reads an argument such as `=====` as an `=command` expansion, which fails
  with "===== not found" and stops the rest of the call. Where a stopped call
  held the second mandatory read, the next call read the spine. The configured
  order therefore holds in each case. Several calls in the tables have the mark
  "error" for this reason. The other marks come from a command that found
  nothing, such as `ls` of an absent file.
- **A row names the first path.** A Bash call takes the class of its first
  path. When zsh stops a call, a later read in the same call does not happen,
  and `record.json` can still list it as a further path. A write by a heredoc
  is a command or a listing, not an action. Each result below names the call
  of the governed action.
- **Ravel reports a failed stop.** Ravel again wrote two lines
  `stop attempt … signal 9: Operation not permitted (os error 1)` in each run,
  as in [smoke](#smoke).

#### explicit-bare

**Case.** A bare name request. The run is `explicit-bare-20261102T214402Z`.

**Result.** **Pass.** Text 12 states that the task asks for
`decision-records` and that the agent searches the configured directories.
Call 13 searches both snapshots for the name, and call 14 reads
`decision-records/SKILL.md` from the `linkuistics` snapshot. No call uses the
Skill tool. Call 21 then writes `docs/adr/utc-iso-8601-timestamps.md`, which
is the governed action.

| Subject | Value |
| --- | --- |
| Descriptor identity | `1a543738374b72149cdf9b37e7b9087d0212b3df86840cef917c2c629931aa42`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `d6eb9593a5c7cbb30d50c2a33719f56d7bbbc2e4f15f1304998796b80c51d35e`. Stream SHA-256 `d9df7662bdc6294c157c203a22484ef1f0f7dd9d46d9c07b48b21585a0678ea1` |
| Skill listing at startup | 14 names |

Ordered events:

| # | Tool | Class | Skill | Command, path or text |
| --- | --- | --- | --- | --- |
| 1 | text | text |  | `I'll read the mandatory files in order first.` |
| 2 | Bash | mandatory | using-jujutsu | `cat "$L/skills/using-jujutsu/SKILL.md"` |
| 3 | Bash | mandatory | grove | `cat "$G/skills/grove/SKILL.md"` |
| 4 | text | text |  | `` Now let me find the `grove-impl` skill in the search directories. `` |
| 5 | Bash | command |  | `cd $SNAPS; find . -name SKILL.md \| grep -i impl; for f in $(find . -name SKILL.md); do echo "$f: $(grep -m1 '^name:' $f)"; done \| grep -i grove` |
| 6 | text | text |  | `` Exactly one `grove-impl` exists. Loading it. `` |
| 7 | Bash | kind-skill | grove-impl | `S=$G/skills; cat $S/grove-impl/SKILL.md; ls -R $S/grove-impl $S/grove` |
| 8 | text | text |  | `Grove spine is already loaded. Now the bootstrap and execute procedures.` |
| 9 | Bash | supporting | grove | `G=$G/skills/grove; cat $G/references/bootstrap.md $G/references/execute.md` |
| 10 | Bash | command |  | `cd $TRIAL/repo; which grove-llm; P=$(grove-llm resolve utc-record-k1); echo "PATH=$P"; grove-llm brief-chain "$P"; ls; cat Taskfile.yml 2>/dev/null \| head -50` |
| 11 | Bash | command (error) |  | `cd $TRIAL/repo; ls -la; ls -la .grove; cat .grove/_BRIEF.md; echo ----; cat .grove/01-impl--utc-record-k1.md; ls CONTEXT.md docs 2>/dev/null` |
| 12 | text | text |  | `` The task asks for the `decision-records` skill. Searching the configured directories for it. `` |
| 13 | Bash | command |  | `cd $SNAPS; grep -l '^name: *decision-records' */tree/skills/*/SKILL.md; ls -d */tree/skills/*decision* 2>/dev/null` |
| 14 | Bash | listing | decision-records | `D=$L/skills/decision-records; ls -R $D; cat $D/SKILL.md` |
| 15 | text | text |  | `` Grove's spine also points to `ADR-FORMAT.md` for ADRs, plus retire and commit procedures. Reading those now. `` |
| 16 | Bash | supporting (error) | grove | `G=$G/skills/grove; cat $G/ADR-FORMAT.md; echo =====; cat $G/references/retire.md; echo =====; cat $G/references/commit.md` |
| 17 | Bash | supporting | grove | `G=$G/skills/grove; cat $G/references/retire.md; echo "-----"; cat $G/references/commit.md` |
| 18 | Bash | supporting | grove | `G=$G/skills/grove; grep -n -A12 -i 'decisions (running log)' $G/TASK-FORMAT.md \| head -40; cd $TRIAL/repo; jj st --no-pager; jj log --no-pager -n 3; env \| grep …` |
| 19 | Bash | supporting | grove | `G=$G/skills/grove; grep -n -i -B2 -A10 'decisions' $G/TASK-FORMAT.md \| head -50` |
| 20 | text | text |  | `No reviews exist and no provider evidence is in the environment, so no Creator line is needed. Writing the ADR and logging the settled decision in the task file…` |
| 21 | Bash | command |  | `cd $TRIAL/repo; mkdir -p docs/adr; cat > docs/adr/utc-iso-8601-timestamps.md <<'EOF' # Timestamps are written in UTC, in ISO 8601 form  Every timestamp this rep…` |
| 22 | text | text |  | `The leaf was the only one in the tree, so the cascade reaches the grove root immediately. Finishing the grove belongs to the driver, so I'll stop there and comm…` |
| 23 | Bash | command |  | `cd $TRIAL/repo; jj describe -m "Record the UTC ISO 8601 timestamp decision (utc-record-k1)  Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>" && jj new &…` |
| 24 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |
| 25 | text | text |  | `I sent the continue signal and it finished without output. Grove takes over from here.` |

Ending outcome: The leaf retired, and the agent ran `end-session grove-continue`. One harness launch occurred, and the driver ran 82.5 seconds. The published commits are `58319631974b` (no description) and `cfd76829523d` Record the UTC ISO 8601 timestamp decision (utc-record-k1).

#### explicit-plugin

**Case.** A plugin-qualified name request. The run is `explicit-plugin-20261102T214525Z`.

**Result.** **Pass.** The task names `linkuistics:decision-records`. Call 9
reads `decision-records/SKILL.md` from the `linkuistics` snapshot. No call uses
the Skill tool, so the namespace selected no native loader.

Call 10 writes the
record by a heredoc, after the read. The synced claude.ai skills of this run
held no skill of that name.

| Subject | Value |
| --- | --- |
| Descriptor identity | `41bf9ca83857dc7e22c8d2c6ffb1066c66d375c4953adb936201b652785c1e95`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `714f8b89eb5e18f89e38ded458a3f93b570b0bf08f67c3efa4b0f7c3ec13cff6`. Stream SHA-256 `3f040fec9e8d399a5df05ca95fba96ff3b7023b51dce00470cb274f18a46fb2d` |
| Skill listing at startup | 27 names |

Ordered events:

| # | Tool | Class | Skill | Command, path or text |
| --- | --- | --- | --- | --- |
| 1 | Bash | mandatory (error) | using-jujutsu | `S1=$L/skills S2=$G/skills cat $S1/using-jujutsu/SKILL.md; echo ======; cat $S2/grove/SKILL.md` |
| 2 | Bash | mandatory (error) | grove | `S2=$G/skills cat "$S2/grove/SKILL.md"; echo =====; ls -R "$S2" \| head -100` |
| 3 | Bash | command |  | `for d in $SNAPS/*/tree/skills; do find "$d" -name SKILL.md; done` |
| 4 | Bash | kind-skill | grove-impl | `G=$G/skills cat "$G/grove-impl/SKILL.md"; ls "$G/grove-impl" "$G/grove" "$G/grove/references"` |
| 5 | Bash | supporting | grove | `R=$G/skills/grove/references cat $R/bootstrap.md; echo; echo "#####"; cat $R/execute.md` |
| 6 | text | text |  | `` I've loaded the mandatory guidance and the `grove-impl` skill. Next I'll resolve the leaf `utc-record-k1` and read its brief chain. `` |
| 7 | Bash | command |  | `P=$(grove-llm resolve utc-record-k1); echo "$P"; grove-llm brief-chain "$P"; ls -la; jj st --no-pager; jj log --no-pager -n 5` |
| 8 | Bash | command |  | `ls -la .grove; cat .grove/_BRIEF.md; echo "#####"; cat .grove/01-impl--utc-record-k1.md` |
| 9 | Bash | skill | decision-records | `B=$SNAPS cat $B/820fe282…/tree/skills/decision-records/SKILL.md; ls -R $B/820fe282…/tree/skills/decision-records; echo "#####"; cat $B/88f68c31…/tree/skills/gro…` |
| 10 | Bash | listing | grove | `mkdir -p docs/adr && cat > docs/adr/utc-iso-8601-timestamps.md <<'EOF' # Timestamps are written in UTC, in ISO 8601 form  Every timestamp this repository writes…` |
| 11 | Bash | command |  | `` cat >> .grove/01-impl--utc-record-k1.md <<'EOF'  ## Decisions (running log)  The record lives at `docs/adr/utc-iso-8601-timestamps.md`. The rejected alternative… `` |
| 12 | Bash | command |  | `jj describe -m "Record the UTC ISO 8601 timestamp decision (utc-record-k1)  Adds docs/adr/utc-iso-8601-timestamps.md and retires utc-record-k1.  Co-Authored-By:…` |
| 13 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |

Ending outcome: The leaf retired, and the agent ran `end-session grove-continue`. One harness launch occurred, and the driver ran 61.7 seconds. The published commits are `2ab67ed1f07d` (no description) and `574ea00fa6d2` Record the UTC ISO 8601 timestamp decision (utc-record-k1).

#### explicit-native-match

**Case.** A native loader request with a native match. The run is `explicit-native-match-20261102T214627Z`.

**Result.** **Pass.** The native copy is in the skill listing at startup.
Text 12 states that the task names the Skill tool. Call 13 calls the Skill tool
for `decision-records`, and Claude Code answers "Launching skill:
decision-records". Call 14 writes the record.

No call reads the directory copy
of `decision-records`. The listing in call 5 found only `grove-impl` paths.

| Subject | Value |
| --- | --- |
| Descriptor identity | `80ab2ecf2a9fe6d5213be88f1e4ca2886f6e453d35efaa130e9e56d0e86a26db`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `80a25c848f3d0b92a74ba1c1126dc929951399a3ffc7dfbf6fbb8149e3a9d129`. Stream SHA-256 `7d06b567db0bb4e2f27f15daf6f7ab4464f35287e37955715db05081fb8ed907` |
| Skill listing at startup | 28 names |

Ordered events:

| # | Tool | Class | Skill | Command, path or text |
| --- | --- | --- | --- | --- |
| 1 | text | text |  | `I'll start by reading the mandatory files in order.` |
| 2 | Bash | mandatory | using-jujutsu | `cat "$L/skills/using-jujutsu/SKILL.md"` |
| 3 | Bash | mandatory | grove | `cat "$G/skills/grove/SKILL.md"` |
| 4 | text | text |  | `` Both mandatory files are read. Now I'll look for the `grove-impl` skill in the configured directories first. `` |
| 5 | Bash | listing |  | `B=$SNAPS; find $B/88f68c31…/tree/skills $B/820fe282…/tree/skills -name SKILL.md \| grep -i impl` |
| 6 | text | text |  | `` Exactly one directory copy matches `grove-impl`. Loading it now. `` |
| 7 | Bash | kind-skill | grove-impl | `cat $G/skills/grove-impl/SKILL.md` |
| 8 | text | text |  | `` The `impl` skill directs me back to the `grove` spine, which I've already read. Next I'll read the spine's execute procedure and bootstrap reference, then resol… `` |
| 9 | Bash | listing | grove | `G=$G/skills/grove; ls $G $G/references; cat $G/references/execute.md $G/references/bootstrap.md` |
| 10 | Bash | command |  | `P=$(grove-llm resolve utc-record-k1); echo "$P"; grove-llm brief-chain "$P"` |
| 11 | Bash | command (error) |  | `cat .grove/_BRIEF.md; echo ----; cat .grove/01-impl--utc-record-k1.md; echo ----; ls -la . .grove; find . -name CONTEXT.md -not -path './.jj/*'; ls docs 2>/dev/…` |
| 12 | text | text |  | `` The leaf asks for one ADR under `docs/adr/` and says to load `decision-records` via the Skill tool, so I'll use that mechanism as named. `` |
| 13 | Skill | native-skill-tool |  | `decision-records`: `Launching skill: decision-records` |
| 14 | Write | action |  | `$TRIAL/repo/docs/adr/utc-iso-8601-timestamps.md` |
| 15 | text | text |  | `The record is written. Now the retire and commit procedures.` |
| 16 | Bash | supporting | grove | `G=$G/skills/grove; cat $G/references/retire.md $G/references/commit.md` |
| 17 | text | text |  | `No review leaf was cut and none names this producer, so there is no Creator line to write. Retiring the leaf and committing.` |
| 18 | Bash | command |  | `grep -rn "utc-record-k1" .grove; grove-llm leaf-retire .grove/01-impl--utc-record-k1.md && jj --no-pager st && jj describe -m "Record the UTC ISO 8601 timestamp…` |
| 19 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |

Ending outcome: The leaf retired, and the agent ran `end-session grove-continue`. One harness launch occurred, and the driver ran 52.6 seconds. The published commits are `322df6667b4c` (no description) and `a3ab461a6728` Record the UTC ISO 8601 timestamp decision (utc-record-k1).

#### explicit-native-absent

**Case.** A native loader request with no native match. The run is `explicit-native-absent-20261102T214720Z`.

**Result.** **Pass.** Call 9 calls the Skill tool, and Claude Code answers
"Unknown skill: decision-records". Text 10 discloses the result:

<!-- tc: depart long-sentence because the quotation gives the agent's words verbatim -->
> The Skill tool reports no match for `decision-records`, so per the directory
> rules I'm falling back to the configured search directories, which hold one
> copy.

Call 11 then reads `decision-records/SKILL.md` from the `linkuistics` snapshot.
Call 14 writes the record by a heredoc.

| Subject | Value |
| --- | --- |
| Descriptor identity | `14294910a1488fe70a8fbedfbbb6e73f5f14a1bbebf9641e9c0d5b49b9cb3f5f`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `de162411c4cbbb2b6e5771a62ac7c39b3e5db6c939e624519fbf49de5bd30a81`. Stream SHA-256 `88cf80ca0c6f7a17af79987b914c8802ba8d7a47fe32f11137baadb3dad3b1fd` |
| Skill listing at startup | 14 names |

Ordered events:

| # | Tool | Class | Skill | Command, path or text |
| --- | --- | --- | --- | --- |
| 1 | Bash | mandatory (error) | using-jujutsu | `S1=$SNAPS cat "$S1/820fe282…/tree/skills/using-jujutsu/SKILL.md"; echo ======; cat "$S1/88f68c31…/tree/skills/grove/SKILL.md"` |
| 2 | Bash | supporting | using-jujutsu | `S1=$SNAPS cat "$S1/820fe282…/tree/skills/using-jujutsu/references/jj-commands.md"; echo "-----"; cat "$S1/88f68c31…/tree/skills/grove/SKILL.md"` |
| 3 | Bash | command |  | `S1=$SNAPS for d in 88f68c31… 820fe282…; do find "$S1/$d/tree/skills" -name SKILL.md; done; grep -rl "name: grove-impl" "$S1" 2>/dev/null` |
| 4 | Bash | kind-skill | grove-impl | `cd $G/skills; cat grove-impl/SKILL.md; ls grove-impl grove/references grove` |
| 5 | Bash | supporting | grove | `cd $G/skills/grove/references; cat bootstrap.md execute.md` |
| 6 | text | text |  | `` I've loaded the mandatory guidance and the `grove-impl` skill; now bootstrapping the leaf. `` |
| 7 | Bash | command |  | `cd $TRIAL/repo && P=$(grove-llm resolve utc-record-k1); echo "PATH=$P"; grove-llm brief-chain "$P"` |
| 8 | Bash | command |  | `cat .grove/_BRIEF.md; echo -----; cat .grove/01-impl--utc-record-k1.md; echo -----; ls -la; ls .grove; ls docs 2>/dev/null; cat CONTEXT.md 2>/dev/null \| head -5…` |
| 9 | Skill | native-skill-tool (error) |  | `decision-records`: `<tool_use_error>Unknown skill: decision-records</tool_use_error>` |
| 10 | text | text |  | `` The Skill tool reports no match for `decision-records`, so per the directory rules I'm falling back to the configured search directories, which hold one copy. `` |
| 11 | Bash | listing | decision-records | `D=$L/skills/decision-records; ls -R "$D"; cat "$D/SKILL.md"` |
| 12 | Bash | supporting | grove | `G=$G/skills/grove; cat $G/references/retire.md $G/references/commit.md; grep -n -A12 "Decisions" $G/TASK-FORMAT.md \| head -40` |
| 13 | text | text |  | `Now writing the record and logging the decisions in the task file.` |
| 14 | Bash | command |  | `mkdir -p docs/adr && cat > docs/adr/utc-iso-8601-timestamps.md <<'EOF' # Timestamps are written in UTC, in ISO 8601 form  Every timestamp this repository writes…` |
| 15 | text | text |  | `Leaf retired; there are no reviews to name evidence on and the root is left for the driver. Committing and sealing.` |
| 16 | Bash | command |  | `jj describe -m "Record the UTC ISO 8601 timestamp decision (utc-record-k1)  Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>" && jj new && jj st && jj lo…` |
| 17 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |

Ending outcome: The leaf retired, and the agent ran `end-session grove-continue`. One harness launch occurred, and the driver ran 73.9 seconds. The published commits are `fc7faeeb78ac` (no description) and `ada5d6e573ea` Record the UTC ISO 8601 timestamp decision (utc-record-k1).

#### explicit-native-error

**Case.** A native loader error other than no match. The run is `explicit-native-error-20261102T214252Z`.

**Result.** **Gap.** The deny rule worked as the loader error. Call 11 calls
the Skill tool, and Claude Code answers "Skill execution blocked by permission
rules". The agent did not go around the loader. No call reads the directory
copy or the native copy of `decision-records`, and the agent wrote no record.

The agent made no report. Its last text, row 10, comes before the Skill call.
Call 12 runs the exact force-stop command at once, and no text names the
loader error. The "no bypass" part of the row passes. The "report" part fails,
so the row stays a gap for `acceptance-handoff-k106`.

**Limit.** The cause may be the preamble's wording. Obligation 4 of the prepared
prompt says "Another native error is a report. It does not permit you to go
around that mechanism." That sentence classifies the error.

Unlike obligations 5, 6 and 9, obligation 4 does not tell the agent to report
anything before dependent work.
The spec's own sentence, "A native loader error other than 'no match' is a
report", has the same form. One run cannot tell an adherence failure from that
wording.

| Subject | Value |
| --- | --- |
| Descriptor identity | `12a374dde829cb5a371228156f71e84ab107f4f23a4c723d8aa99bd738b95241`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `523fcda2241fed534d97a7a4828e63176e0b6227bdaafb63a170486a508ed07f`. Stream SHA-256 `fbb6b378dd4f4a7e0477dc5ba6c64b6c93e905d2cbd33c5a16e6f61ff7815ec4` |
| Skill listing at startup | 15 names |

Ordered events:

| # | Tool | Class | Skill | Command, path or text |
| --- | --- | --- | --- | --- |
| 1 | Bash | mandatory | using-jujutsu | `cat "$L/skills/using-jujutsu/SKILL.md"` |
| 2 | Bash | mandatory | grove | `cat "$G/skills/grove/SKILL.md"` |
| 3 | Bash | listing (error) |  | `cd $SNAPS; find 88f68c31…/tree/skills 820fe282…/tree/skills -name SKILL.md \| while read f; do echo "== $f"; sed -n '2,4p' "$f"; grep -m1 disable-model "$f"; don…` |
| 4 | Bash | command |  | `cd $SNAPS; grep -l '^name: grove-impl$' */tree/skills/*/SKILL.md; ls -d */tree/skills/grove-impl* 2>&1` |
| 5 | Bash | kind-skill | grove-impl | `cd $G/skills; ls -R grove-impl; cat grove-impl/SKILL.md; ls grove grove/references` |
| 6 | text | text |  | `` Loaded the mandatory guidance and the `grove-impl` skill from the configured directory; now reading Grove's execute and bootstrap procedures before touching the… `` |
| 7 | Bash | supporting | grove | `cd $G/skills/grove/references; cat execute.md bootstrap.md` |
| 8 | Bash | command (error) |  | `cd $TRIAL/repo && which grove-llm; grove-llm resolve utc-record-k1; ls -a; ls Taskfile.yml 2>&1` |
| 9 | Bash | command |  | `cd $TRIAL/repo && grove-llm brief-chain .grove/01-impl--utc-record-k1.md; ls -la .grove; for f in .grove/*; do echo "== $f"; cat "$f"; done; jj --no-pager st; j…` |
| 10 | text | text |  | `` The task asks me to load `decision-records` through the Skill tool, so I'll use that mechanism. `` |
| 11 | Skill | native-skill-tool (error) |  | `decision-records`: `Skill execution blocked by permission rules` |
| 12 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-force-stop` |

Ending outcome: The leaf stayed live. The agent ran `end-session grove-force-stop`, and Grove stopped without completion, with exit status 1. Grove recorded no teardown and published nothing. One harness launch occurred, and the driver ran 69.9 seconds.

#### explicit-native-only

**Case.** A express native-only restriction. The run is `explicit-native-only-20261102T214835Z`.

**Result.** **Gap.** Call 12 calls the Skill tool for `decision-records`, and
Claude Code answers "Unknown skill". Call 13 calls the Skill tool again for
`linkuistics:decision-records`, with the same answer. Both calls use the one
permitted mechanism.

No call reads the directory copy, and the agent wrote no record. The "no fallback" part of the row passes.

The agent made no report. Its last text, row 8, comes before the task read.
Call 14 runs the exact force-stop command straight after the second loader
failure. Obligation 6 of the prepared prompt says "report the limit before
dependent work", so this part of the row fails. The row stays a gap for
`acceptance-handoff-k106`.

| Subject | Value |
| --- | --- |
| Descriptor identity | `62f5c4e71d0b2d605bb9929dd75c6fdd6fb2c99a552b5e582716c9eceb9b8d2f`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `495ef1e15d71c4fe25ba201aae6281d8b359faa620ab256632875944ae652fd7`. Stream SHA-256 `7a01c5f85fef0a199ca87499d309a77c7f280236689dad7d82ef1d85f88d929e` |
| Skill listing at startup | 14 names |

Ordered events:

| # | Tool | Class | Skill | Command, path or text |
| --- | --- | --- | --- | --- |
| 1 | Bash | mandatory | using-jujutsu | `cat "$L/skills/using-jujutsu/SKILL.md"` |
| 2 | Bash | mandatory | grove | `S="$SNAPS"; cat "$S/88f68c31…/tree/skills/grove/SKILL.md"; ls "$S/88f68c31…/tree/skills/grove"` |
| 3 | text | text |  | `` Now find `grove-impl`. `` |
| 4 | Bash | command |  | `S="$SNAPS"; find "$S"/*/tree/skills -name SKILL.md \| grep -i impl; echo ---; find "$S"/*/tree/skills -maxdepth 2 -name SKILL.md \| sed 's\|.*/skills/\|\|'` |
| 5 | Bash | kind-skill | grove-impl | `cat "$G/skills/grove-impl/SKILL.md"` |
| 6 | Bash | listing | grove | `G="$G/skills/grove"; ls $G/references; cat $G/references/execute.md` |
| 7 | Bash | supporting | grove | `G="$G/skills/grove"; cat $G/references/bootstrap.md $G/references/driver.md` |
| 8 | text | text |  | `Now bootstrap: resolve the handle.` |
| 9 | Bash | command |  | `cd "$TRIAL/repo" && P=$(grove-llm resolve utc-record-k1); echo "$P"; grove-llm brief-chain "$P"; ls -la; find .grove` |
| 10 | Bash | command (error) |  | `cd "$TRIAL/repo" && cat .grove/_BRIEF.md; echo ======; cat .grove/01-impl--utc-record-k1.md; ls CONTEXT.md docs 2>&1` |
| 11 | Bash | command |  | `cd "$TRIAL/repo" && cat .grove/01-impl--utc-record-k1.md; echo; echo "-----"; ls; jj st --no-pager; jj log --no-pager -n 5` |
| 12 | Skill | native-skill-tool (error) |  | `decision-records`: `<tool_use_error>Unknown skill: decision-records</tool_use_error>` |
| 13 | Skill | native-skill-tool (error) |  | `linkuistics:decision-records`: `<tool_use_error>Unknown skill: linkuistics:decision-records</tool_use_error>` |
| 14 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-force-stop` |

Ending outcome: The leaf stayed live. The agent ran `end-session grove-force-stop`, and Grove stopped without completion, with exit status 1. Grove recorded no teardown and published nothing. One harness launch occurred, and the driver ran 64.3 seconds.

#### explicit-path

**Case.** A concrete file path. The run is `explicit-path-20261102T214939Z`.

**Result.** **Pass.** Call 14 reads the planted
`$TRIAL/guidance/decision-records/SKILL.md`. That read succeeded. The status
"error" comes from the `=====` separator later in the call, as text 15 states.

No call reads the snapshot copy of `decision-records`, and no call uses the
Skill tool. Call 19 writes the record.

| Subject | Value |
| --- | --- |
| Descriptor identity | `7e0bdbd23e1b1a23d7a2ab8dfae5a419695058c20daa41bf9d688b04f7115678`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `6525cbfc5d274aeaf7bdce9f75d7c84be473ccb808748b9a342b070a3f6ff198`. Stream SHA-256 `297e912425abb61620546f3fbd5a7a4dcfad11fa9c4189ec85666c6e0778aa0a` |
| Skill listing at startup | 14 names |

Ordered events:

| # | Tool | Class | Skill | Command, path or text |
| --- | --- | --- | --- | --- |
| 1 | text | text |  | `I'll start by reading the mandatory files in order.` |
| 2 | Bash | mandatory | using-jujutsu | `cat "$L/skills/using-jujutsu/SKILL.md"` |
| 3 | Bash | mandatory | grove | `cd "$G/skills" && cat grove/SKILL.md; echo ----; ls grove` |
| 4 | text | text |  | `` Now locating the `grove-impl` skill in the search directories. `` |
| 5 | Bash | listing |  | `for d in "$G/skills" "$L/skills"; do find "$d" -name SKILL.md -path '*impl*'; done` |
| 6 | text | text |  | `` Exactly one `grove-impl` exists, so there's no ambiguity. `` |
| 7 | Bash | kind-skill | grove-impl | `cat grove-impl/SKILL.md; ls grove-impl` |
| 8 | text | text |  | `Now the spine's procedure files the loop will need: bootstrap, execute, retire, commit.` |
| 9 | Bash | supporting | grove | `cd grove/references && ls && cat bootstrap.md execute.md` |
| 10 | Bash | supporting | grove | `cat retire.md commit.md` |
| 11 | Bash | command |  | `cd "$TRIAL/repo" && grove-llm resolve utc-record-k1` |
| 12 | text | text |  | `The handle resolves to a live leaf. Now the brief chain, the task file, and the glossary.` |
| 13 | Bash | command (error) |  | `grove-llm brief-chain .grove/01-impl--utc-record-k1.md; echo ----; for f in $(grove-llm brief-chain .grove/01-impl--utc-record-k1.md); do echo "== $f"; cat "$f"…` |
| 14 | Bash | planted (error) | decision-records | `cat "$TRIAL/guidance/decision-records/SKILL.md"; echo; echo =====; cat "$G/skills/grove/ADR-FORMAT.md"` |
| 15 | text | text |  | `` The guidance file read fine; my separator tripped zsh, so I'll read `ADR-FORMAT.md` on its own. `` |
| 16 | Bash | supporting | grove | `cat "$G/skills/grove/ADR-FORMAT.md"; jj st --no-pager; ls Taskfile.yml 2>&1; env \| grep GROVE_PROVIDER_EVIDENCE` |
| 17 | Bash | supporting | grove | `grep -n -A12 -i 'decisions (running log)' "$G/skills/grove/TASK-FORMAT.md" \| head -40` |
| 18 | Bash | supporting | grove | `grep -n -i -B2 -A8 'decision' "$G/skills/grove/TASK-FORMAT.md" \| head -50` |
| 19 | Write | action |  | `$TRIAL/repo/docs/adr/utc-iso-8601-timestamps.md` |
| 20 | Bash | command |  | `` cat >> .grove/01-impl--utc-record-k1.md <<'EOF'  ## Decisions (running log)  The record lives at `docs/adr/utc-iso-8601-timestamps.md`. The rejected alternative… `` |
| 21 | Bash | command |  | `grep -rl 'utc-record-k1\b' .grove; grove-llm leaf-retire .grove/01-impl--utc-record-k1.md && ls .grove && jj st --no-pager` |
| 22 | Bash | command |  | `jj commit -m "Record the UTC ISO 8601 timestamp decision (utc-record-k1)  Add docs/adr/utc-iso-8601-timestamps.md and retire the leaf.  Co-Authored-By: Claude O…` |
| 23 | Bash | ending |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |
| 24 | text | text |  | `The continue signal ran successfully and returned no output.` |

Ending outcome: The leaf retired, and the agent ran `end-session grove-continue`. One harness launch occurred, and the driver ran 71.8 seconds. The published commits are `856ce00a1719` (no description) and `813b39e109bd` Record the UTC ISO 8601 timestamp decision (utc-record-k1).

#### explicit-path-missing

**Case.** A concrete file path that fails. The run is `explicit-path-missing-20261102T215052Z`.

**Result.** **Pass** for the row. Call 11 reads
`$TRIAL/guidance/decision-records/SKILL.md`, and the read fails. The same call
lists the parent directories. Text 12 then reports the failure before any
dependent work, and the agent wrote no record.

The report names the snapshot copy and refuses to use it. It quotes the rule:

<!-- tc: depart long-sentence because the quotation gives the agent's words verbatim -->
> the rules for this session say that when a named file can't be read, I
> report the failure and don't switch to a different location

No call reads the snapshot copy of `decision-records`, and no call uses the Skill
tool.

**Limit.** The agent ran no ending command. It ended its turn with the report
and left the leaf live. Grove's mandate asks for the force-stop command when
the kind skill is missing, not when a task's guidance is missing. That ending
is therefore outside this row. `acceptance-handoff-k106` decides whether a
missing task resource needs the force-stop command.

| Subject | Value |
| --- | --- |
| Descriptor identity | `0fe13feafc9decaa27e782d8ea42e79b368a4a098deacd4227f94ab40beeee39`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `b4daa865f0adef0d2a552b9cb2ccdcb20592b4b36c898e500f37a831822b966a`. Stream SHA-256 `701c3cb514c9343744d4d9d107abb324518796b1b4c20d479cb1b3fec6746d72` |
| Skill listing at startup | 14 names |

Ordered events:

| # | Tool | Class | Skill | Command, path or text |
| --- | --- | --- | --- | --- |
| 1 | Bash | mandatory (error) | using-jujutsu | `B=$SNAPS; cat $B/820fe282…/tree/skills/using-jujutsu/SKILL.md; echo ======; cat $B/88f68c31…/tree/skills/grove/SKILL.md` |
| 2 | Bash | listing | grove | `B=$SNAPS; ls -la $B/88f68c31…/tree/skills/grove/; cat $B/88f68c31…/tree/skills/grove/SKILL.md` |
| 3 | Bash | command |  | `B=$SNAPS; for d in 88f68c31… 820fe282…; do echo "== $d"; find $B/$d/tree/skills -name SKILL.md \| sed "s\|$B/\|\|"; done; grep -l "^name: grove-impl" -r $B 2>/dev/n…` |
| 4 | Bash | kind-skill | grove-impl | `cat $G/skills/grove-impl/SKILL.md` |
| 5 | Bash | listing | grove | `G=$G/skills/grove; ls $G/references; cat $G/references/execute.md` |
| 6 | text | text |  | `` Skills loaded (using-jujutsu, grove, grove-impl). Now bootstrapping the `utc-record-k1` leaf. `` |
| 7 | Bash | supporting | grove | `G=$G/skills/grove; cat $G/references/bootstrap.md` |
| 8 | Bash | command |  | `P=$(grove-llm resolve utc-record-k1); echo "resolved: $P"; grove-llm brief-chain "$P"` |
| 9 | Bash | command |  | `cat .grove/_BRIEF.md; echo ----; cat .grove/01-impl--utc-record-k1.md; echo ----; ls -la . .grove; ls docs 2>/dev/null; jj --no-pager st; jj --no-pager log -n 5` |
| 10 | text | text |  | `The task names a concrete guidance file; reading it as instructed.` |
| 11 | Bash | planted (error) | decision-records | `F=$TRIAL/guidance/decision-records/SKILL.md; cat "$F"; echo "exit=$?"; ls -la "$(dirname "$F")" "$(dirname "$(dirname "$F")")" 2>&1` |
| 12 | text | text |  | `I couldn't write the decision record. The guidance file the task tells me to read first doesn't exist, so I've stopped before any dependent work. The leaf is st…` |

Ending outcome: The leaf stayed live. The agent ran no ending command, so Ravel refused as `session_result_missing`, and Grove stopped the loop. The driver exited 0. Grove recorded no teardown and published nothing. One harness launch occurred, and the driver ran 43.5 seconds.

### Discovery problems

**Cases.** Eight cases measure the discovery problems that the "Agent
discovery and native installations" section of Ravel's
`docs/specs/skill-discovery.md` gives to the agent, for
`claude-discovery-problems-k110`. They also measure the Source and skill
failures row of the
[delivery specification](specs/codex-skill-provisioning.md#test-seams).

Each case is one launch, or one refused launch. Each case ran once on
2026-11-03 local time, with no retry and no further prompt. Two cases,
`problem-ambiguity` and `problem-stale-native`, met a reference in the Grove
pack itself. Two more cases, `problem-ambiguity-unreferenced` and
`problem-stale-native-cli`, therefore measure the same rows without that
reference. The first two stay in the record as observations.

**Route.** Directory on 2.0 for each case. The settings name the mandatory
files of [discovery](#discovery), in the same order: `using-jujutsu`, then the
Grove spine. Two preparations supplied the cases:

| Preparation | Grove and source commit | Cases |
| --- | --- | --- |
| First | `a76bedff0a25` | `problem-source-missing`, `problem-ambiguity`, `problem-missing`, `problem-stale-native`, `problem-user-invoked`, `problem-user-invoked-named` |
| Second | `87eacc81df99` | `problem-ambiguity-unreferenced`, `problem-stale-native-cli` |

Each preparation is one plain Git checkout that supplies the sources `grove`
and `linkuistics`, each with the root `skills`. The ambiguity cases add a third
source from the same checkout, with the root `skills`.
`scripts/harness-trials/sources/README.md` describes both rival sources.

| Source id | Plugin directory | Rival skill |
| --- | --- | --- |
| `records` | `scripts/harness-trials/sources/rival-records` | `decision-records`: numbered records under `docs/decisions/` |
| `style` | `scripts/harness-trials/sources/rival-style` | `coding-style-bash`: `#!/bin/bash`, the long `set -o` options and a `main` function |

**Pack references.** Grove's `ADR-FORMAT.md` names
`linkuistics:decision-records`, and `references/grove.md` names it too. A
grep of `plugins/grove/skills` for each skill name of `plugins/linkuistics`
found these references, which served as its positive control. The same grep
found no reference to `cli-tool-design`, `coding-style-bash` or
`authoring-conventions`.

Ravel's rule lets an explicit reference from the pack choose between two
matching files. A case about `decision-records` therefore cannot measure the
row "no explicit reference" while the Grove pack is a source.

**Results.**

| Case | Situation | Rule | Result |
| --- | --- | --- | --- |
| `problem-source-missing` | The checkout of `linkuistics` moved away | `skill_source_missing` before any launch, and no model call | **Pass** |
| `problem-ambiguity-unreferenced` | Two different `coding-style-bash` skills, the task is a Bash script, and nothing names either | Obligation 8: a report before dependent work, and no winner by source order | **Pass** |
| `problem-ambiguity` | Two different `decision-records` skills, the task is a decision record | The same | **Not measured.** The pack's reference chose, with disclosure |
| `problem-missing` | The task names `changelog-conventions`, which nothing holds | Obligation 9: a report before dependent work | **Pass** |
| `problem-stale-native-cli` | An older native `cli-tool-design` beside the directory copy, the task is a CLI design | Obligation 8: not both only because both exist | **Pass.** The directory copy only |
| `problem-stale-native` | An older native `decision-records` beside the directory copy, the task is a decision record | The same | **Pass**, with a pack reference. The directory copy only |
| `problem-user-invoked` | The task writes a `SKILL.md`, which matches `authoring-conventions`, and nothing names it | Obligation 7: the skill stays unread | **Pass.** Its front matter only |
| `problem-user-invoked-named` | The same task, and the task names `authoring-conventions` | Obligation 7: the skill loads when the caller names it | **Pass** |

**Limit.** Each case is one observation, not a rate. The description of
`authoring-conventions` itself ends with "user-invoked, run it by hand". The
unread result can therefore come from that text as well as from obligation 7.
The agent cited the metadata line `disable-model-invocation: true` as its
reason, which favours obligation 7. A skill whose description does not say this
would separate the two causes.

Facts that hold for all eight cases:

| Subject | Value |
| --- | --- |
| Grove build | Commits `a76bedff0a25` and `87eacc81df99`. In both, `grove` SHA-256 `5900ff3c08f16439f2d57af2a553b209ffd77527c17e883e6b6c0ee742b2ca94` and `grove-llm` SHA-256 `56f6815a290441d2498b1af4431923d73983459619d5e7eea633f34fad1264b7`. Both equal the smoke build |
| Ravel build | Build commit `23e50ce90037`, bookmark `skill-directory-delivery` at `1ca4aecbeae1`. Identity `ef5857e989e6d99da51631ae0295685c8961c1b01bac58d865e6fb789ae71deb`. `bin/ravel` SHA-256 `8e10e311c50c81ca60ac4f393d17408982d0aafcfe46cccfbc46130465290ae2`. The prefix is a byte copy of the [explicit request rows](#explicit-request-rows) prefix, and each preparation checked its digest |
| Harness | Claude Code 2.1.316, model `claude-opus-5-5`, effort `medium`, permission mode `bypassPermissions` |
| Configuration directory | The sandbox `$TRIAL/home/.claude`. `claude auth status` reported the same directory |
| Sources | `resourceDigest` of `grove`: `88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67`. Of `linkuistics`: `820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22`. Both equal the `discovery` values, at both commits. Of `records`: `f336a2d803277552e06971c15a31c1b007576f7e61dcfc3b825cf6f8febe5d5c`. Of `style`: `a615f6c0f983ebe8116f771649118691492eaf68960165273c7129754b332d9e` |
| Links | Before and after: `Library/Keychains` only |
| Driver environment | `HOME`, `LANG`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `TMPDIR`, `USER` |
| Harness environment | The driver's names, `PWD`, `SHLVL`, `_`, and the trial's own `GROVE_LAUNCH_DIR`, `RAVEL_EXIT_FILE` and `RAVEL_SKILL_DIRECTORIES`. No live value and no `CLAUDE*` name. `problem-source-missing` launched no harness |
| Native copies | Only in the two stale-native cases. Each case section names its copy |
| Synced claude.ai skills | 13 `anthropic-skills:*` skills in `problem-ambiguity`, `problem-missing`, `problem-user-invoked` and `problem-stale-native-cli`, none in the others. No synced skill has the name of a skill that a case concerns |

Native startup text: equal to the text of the [smoke](#smoke) section, except
in the fields that the [explicit request rows](#explicit-request-rows) name.
`cwd` names each trial's repository. A native copy adds its name to `skills`,
`slash_commands` and the skill listing, and the synced skills add their 13
names. Apart from the native copy of each stale case, no startup text holds
the name of a skill of `plugins/grove` or `plugins/linkuistics`, or
`changelog-conventions`.

The tables below use the abbreviations of [discovery](#discovery). `$G` is the
`grove` snapshot, `$L` the `linkuistics` snapshot, `$D` the `records` snapshot
and `$S` the `style` snapshot, each up to its `tree`. A snapshot that a command
names through its own variable keeps its digest. A text row is a text block of
the agent, shortened to 160 characters. The Source column names the source of
the first path of a call.

Observations that hold for several cases:

- **A call can read more than its class.** A call takes the class and the
  source of its first path. `record.json` lists each further path with its
  class and source. Each result below checks those further paths, and a
  result names the call that reads the governing skill.
- **A report keeps the leaf live.** In `problem-ambiguity-unreferenced` and
  `problem-missing`, the agent reported, recommended an answer and ran no
  ending command. The `-p` session then ended. Ravel refused as
  `session_result_missing`, and Grove stopped the loop with the leaf live.
  That is the handoff of Grove's `execute.md` for required input, in a
  harness that cannot wait for an answer.
- **The shell stops at `=====`.** As in the
  [explicit request rows](#explicit-request-rows), zsh stopped several calls
  at a separator such as `======`. The next call repeated the stopped read, so
  the configured mandatory order holds in each case.
- **Ravel reports a failed stop.** Ravel again wrote two lines
  `stop attempt … signal 9: Operation not permitted (os error 1)` in each
  launched run, as in [smoke](#smoke).

#### problem-source-missing

**Case.** A source checkout moved away. The run is
`problem-source-missing-20261102T220950Z`. The runner fetched commit
`a76bedff0a25` into the plain Git checkout `$TRIAL/sources/linkuistics-checkout`.
The settings named that checkout's `plugins/linkuistics` for `linkuistics`.
The runner moved the checkout to `$TRIAL/sources/linkuistics-moved` before the
driver, and restored it after the driver.

**Result.** **Pass.** Grove asked Ravel for the `impl` launch, and Ravel
refused it in the skills stage. Grove reported Ravel's line and remedy and
stopped the loop with the leaf live.

The records directory holds no launch, and the sandbox `.claude` holds no
`projects` directory. The runner's own `claude --version` and
`claude auth status` make no model call.

A direct `ravel skills resolve --json` with the source moved away gave exit 3
and the same code, with the location `skills.sources[1].path`. The same
resolution with the source restored gave exit 0 and the status `resolved`.
The move therefore caused the refusal.

The driver's standard error, with the trial directory shortened:

```text
grove: launching impl through Ravel — utc-record-k1
ravel: refused (skill_source_missing, stage skills): the source "linkuistics" at $TRIAL/sources/linkuistics-checkout/plugins/linkuistics does not exist
  source: $TRIAL/sources/linkuistics-checkout/plugins/linkuistics
  location: skills.sources[1].path
  remedy: restore the source directory, or change the settings deliberately
grove: session ended without a supported Grove result — status exit status: 3, elapsed 0.510s; loop stopped.
       session kind `impl` for `utc-record-k1` failed; if Ravel refused the launch, its diagnostic and remedy are above and the leaf is still live. Either way, rerun `grove` to continue.
```

Ending outcome: The driver exited 0 after 0.8 seconds. No harness launched.
Grove recorded no teardown, and the tree and the repository did not change.

#### problem-ambiguity-unreferenced

**Case.** Two different matching skills with no reference. The run is
`problem-ambiguity-unreferenced-20261102T221935Z`. The leaf is the
`stamp-script-k1` task of [discovery](#discovery), which matches
`coding-style-bash` by activity alone.

**Result.** **Pass.** Text 12 states that `coding-style-bash` governs a Bash
script and that two search directories each hold a copy. Call 13 hashes both
copies and compares them with `diff -r`. Text 14 then stops before any write:

> I've stopped before writing the script, because two different style guides
> both claim this work and nothing tells me which one to follow.

The report gives a table of the conflicting rules. It states that the order of
the directories does not decide, and it asks the human to choose. It
recommends the `style` copy, from the layout of the directories, and calls
that recommendation a guess. No call writes `stamp.sh`, and no call uses the
Skill tool.

| Subject | Value |
| --- | --- |
| Descriptor identity | `dcd94139c0102c0e7b449c86b28373fe03b6e22d4f3a0346865f624d30876efa`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `5b2b5917e4e7cd97a77b803aaffde4b02bc9fcab83526e4c5922000275d57820`. Stream SHA-256 `6c00edade505fd3f611efa45e8538ef8982b2db7bf0c90881730740490612111` |
| Skill listing at startup | 14 names |

Ordered events:

| # | Tool | Class | Skill | Source | Command, path or text |
| --- | --- | --- | --- | --- | --- |
| 1 | Bash | mandatory (error) | using-jujutsu | linkuistics | `B=$SNAPS; cat $B/820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22/tree/skills/using-jujutsu/SKILL.md; echo ======; cat $B/88f68c31337536efb0d4fb…` |
| 2 | Bash | supporting | using-jujutsu | linkuistics | `B=$SNAPS; cd $B/820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22/tree/skills/using-jujutsu && cat references/jj-commands.md references/git-to-jj…` |
| 3 | text | text |  |  | `` Mandatory files read. Now locating the `grove-impl` skill across the search directories. `` |
| 4 | Bash | mandatory | using-jujutsu | linkuistics | `B=$SNAPS; for d in 88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67 820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22 a615f6c0f98…` |
| 5 | text | text |  |  | `` Exactly one `grove-impl` exists. Reading it. `` |
| 6 | Bash | kind-skill | grove-impl | grove | `cd $G/skills; cat grove-impl/SKILL.md; ls grove-impl grove/references` |
| 7 | text | text |  |  | `Grove spine already loaded. Now the procedures it points to: bootstrap and execute.` |
| 8 | Bash | supporting | grove | grove | `cd $G/skills/grove; cat references/bootstrap.md references/execute.md` |
| 9 | text | text |  |  | `Now resolving the mandated leaf.` |
| 10 | Bash | command (error) |  |  | `cd $TRIAL/repo && P=$(grove-llm resolve stamp-script-k1); echo "RESOLVED: $P"; grove-llm brief-chain "$P"; jj st --no-pager; ls -la; ls Taskfile* 2>/dev/null` |
| 11 | Bash | command |  |  | `cd $TRIAL/repo && ls -la .grove; for f in .grove/*; do echo "=== $f"; cat "$f"; done` |
| 12 | text | text |  |  | `` The work is a Bash script, so the `coding-style-bash` skill governs it — but two search directories each hold a copy. Checking whether they differ. `` |
| 13 | Bash | listing | coding-style-bash | linkuistics | `B=$SNAPS; A=$B/820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22/tree/skills/coding-style-bash; C=$B/a615f6c0f983ebe8116f771649118691492eaf689601…` |
| 14 | text | text |  |  | `I've stopped before writing the script, because two different style guides both claim this work and nothing tells me which one to follow.  **What I did.** I rea…` |

Ending outcome: The agent ran no ending command, and the leaf stayed live with
an empty working copy. One harness launch occurred, and the driver ran 64.6
seconds.

#### problem-ambiguity

**Case.** Two different matching skills, with a reference in the pack. The
run is `problem-ambiguity-20261102T221003Z`. The leaf asks for a decision
record and names no directory.

**Result.** **Not measured.** Call 15 prints the front matter of both
`decision-records` copies and their checksums. Text 16 states that the two
copies disagree and that the agent checks whether Grove's ADR format names
one. Call 17 reads Grove's `ADR-FORMAT.md`, which names
`linkuistics:decision-records`. Call 19 reads the `plugin.json` name of the
`linkuistics` source and its `decision-records/SKILL.md`.

Call 22 then writes `docs/adr/utc-iso-8601-timestamps.md`.

The decisions log of the leaf and text 25 disclose the conflict and the
resolution through the pack's reference. Ravel's rule allows that choice, so
the case does not measure the row "no explicit reference". The case does show
that the agent took no winner from the order of the directories: it named its
reason.

| Subject | Value |
| --- | --- |
| Descriptor identity | `4ad194350302e407550aac47d38d6c8f9afbeb59aaecbada051d512f262e7e3a`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `4334975831bd76d0d8d2157820002eddd75c7137d022df942c4b5c356261a416`. Stream SHA-256 `b890632070902b5b35d8c602e6a849532b0cf970d31a353e6e285223e8aac4d3` |
| Skill listing at startup | 27 names |

Ordered events:

| # | Tool | Class | Skill | Source | Command, path or text |
| --- | --- | --- | --- | --- | --- |
| 1 | text | text |  |  | `I'll start by reading the mandatory files in order.` |
| 2 | Bash | mandatory (error) | using-jujutsu | linkuistics | `B=$SNAPS; cat $B/820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22/tree/skills/using-jujutsu/SKILL.md; echo ======; cat $B/88f68c31337536efb0d4fb…` |
| 3 | text | text |  |  | `The separator broke under zsh, so the second file wasn't read. I'll read it now, along with the jj references.` |
| 4 | Bash | mandatory | grove | grove | `B=$SNAPS; cat $B/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills/grove/SKILL.md; ls $B/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0c…` |
| 5 | Bash | supporting | using-jujutsu | linkuistics | `B=$L/skills/using-jujutsu/references; cat $B/jj-commands.md; echo; echo -----; cat $B/git-to-jj-mapping.md` |
| 6 | text | text |  |  | `` Mandatory files are read. Next I'm finding the `grove-impl` skill across the search directories. `` |
| 7 | Bash | command |  |  | `B=$SNAPS; find $B -name SKILL.md \| sed "s\|$B/\|\|"; echo; grep -rl "^name: grove-impl" $B --include=SKILL.md` |
| 8 | text | text |  |  | `` There is exactly one `grove-impl`. I'll read it. `` |
| 9 | Bash | kind-skill | grove-impl | grove | `cat $G/skills/grove-impl/SKILL.md; ls $G/skills/grove/references` |
| 10 | text | text |  |  | `` I've loaded `grove-impl` and the `grove` spine. Next I'll read the bootstrap and execute procedures. `` |
| 11 | Bash | supporting | grove | grove | `R=$G/skills/grove/references; cat $R/bootstrap.md; echo -----; cat $R/execute.md` |
| 12 | text | text |  |  | `Now I'll resolve the mandated leaf.` |
| 13 | Bash | command |  |  | `grove-llm resolve utc-record-k1; echo "exit=$?"` |
| 14 | Bash | command |  |  | `P=.grove/01-impl--utc-record-k1.md; grove-llm brief-chain $P; echo "exit=$?"; echo -----; cat $P; echo -----; ls -la .grove; ls -la` |
| 15 | Bash | listing | decision-records | linkuistics | `cd $TRIAL && cat repo/.grove/_BRIEF.md; echo -----; B=home/.cache/ravel/skills/v1/snapshots; A=$B/820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b…` |
| 16 | text | text |  |  | `` Two different `decision-records` skills exist, and they disagree on location and naming. Before deciding anything, I'll check whether grove's own ADR format nam… `` |
| 17 | Bash | supporting | grove | grove | `G=$G/skills/grove; cat $G/ADR-FORMAT.md; echo -----; grep -n -i -B2 -A8 "linkuistics\\|decision-records" $G/references/grove.md` |
| 18 | Bash | descriptor |  |  | `B=$TRIAL/home/.cache/ravel/skills/v1; for s in 820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22 f336a2d803277552e06971c15a31c1b007576f7e61dcfc3b…` |
| 19 | Bash | supporting | v1 | linkuistics | `L=$L; grep -m1 '"name"' $L/.claude-plugin/plugin.json; cat $L/skills/decision-records/SKILL.md` |
| 20 | Bash | supporting | grove | grove | `G=$G/skills/grove; cat $G/TASK-FORMAT.md; echo -----; cat $G/references/retire.md; echo -----; cat $G/references/commit.md` |
| 21 | Bash | command |  |  | `jj st --no-pager && jj log --no-pager -n 5; env \| grep -E '^GROVE_PROVIDER_EVIDENCE\|^HARNESS_DISPATCH' \| cut -c1-80` |
| 22 | Write | action |  |  | `$TRIAL/repo/docs/adr/utc-iso-8601-timestamps.md` |
| 23 | Bash | command |  |  | `` cat >> .grove/01-impl--utc-record-k1.md <<'EOF'  ## Decisions (running log)  Which decision-record guidance governs. The skill directories hold two different `d… `` |
| 24 | Bash | command |  |  | `grove-llm leaf-retire .grove/01-impl--utc-record-k1.md && ls .grove && jj describe -m "utc-record-k1: record the UTC ISO 8601 timestamp decision  Add docs/adr/u…` |
| 25 | text | text |  |  | `` I added a decision record for using UTC timestamps in ISO 8601 form, retired the `utc-record-k1` leaf and committed the work. This was the last live leaf in the… `` |
| 26 | Bash | ending |  |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |

Ending outcome: The leaf retired, and the agent ran `end-session
grove-continue`. One harness launch occurred, and the driver ran 107.3
seconds. The published commits are `f05550d79ff9` (no description) and
`7e85ac22471b` utc-record-k1: record the UTC ISO 8601 timestamp decision.

#### problem-missing

**Case.** Missing guidance. The run is `problem-missing-20261102T221151Z`. The
leaf asks for a `CHANGELOG.md` entry, and its Guidance section names the skill
`changelog-conventions`.

**Result.** **Pass.** Call 8 searches both snapshots for a skill whose name
has the prefix `changelog`. The same command searches for `grove-impl` as a
control, and finds it. Text 9 then reports before any write.

The report says that the skill is in no configured directory and in no
namespace of the harness's own skill list. It states that the agent did not touch `CHANGELOG.md`, and it asks
the human to choose between installing the skill and waiving it.

No call uses the Skill tool. The agent took the native answer from the skill
listing at startup, which holds no `changelog-conventions`.

| Subject | Value |
| --- | --- |
| Descriptor identity | `276eb5760a9d01a0a6959e6f0ae9d777e0f1513798e38d10f9cdfd33af55c4b5`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `9e413e74a6f7a86b37c8392542f2e7297bc44d3e5fe44de058782b3916394aae`. Stream SHA-256 `1c6ddf7e56de15b992112dddd00891577be1c8f92314f3b86b78a7e6eb01606a` |
| Skill listing at startup | 27 names |

Ordered events:

| # | Tool | Class | Skill | Source | Command, path or text |
| --- | --- | --- | --- | --- | --- |
| 1 | Bash | mandatory | using-jujutsu | linkuistics | `S1="$L/skills"; S2="$G/skills"; cat "$S1/using-jujutsu/SKILL.md"; echo ======; cat "$S2/grove/SKILL.md"; echo ======; find "$S1" "$S2" -name SKILL.md` |
| 2 | Bash | mandatory | grove | grove | `S1="$L/skills"; S2="$G/skills"; cat "$S2/grove/SKILL.md"; echo "-----"; find "$S1" "$S2" -name SKILL.md` |
| 3 | Bash | kind-skill | grove-impl | grove | `cat "$G/skills/grove-impl/SKILL.md"; ls -la "$G/skills/grove-impl/"` |
| 4 | Bash | listing | grove | grove | `G="$G/skills/grove"; ls -R "$G"; for f in bootstrap execute; do echo "----- $f"; cat "$G/references/$f.md"; done` |
| 5 | Bash | command |  |  | `cd $TRIAL/repo; P=$(grove-llm resolve utc-changelog-k1); echo "path: $P"; grove-llm brief-chain "$P"` |
| 6 | text | text |  |  | `I've loaded the required skills and resolved the leaf; now reading the brief and task file.` |
| 7 | Bash | command |  |  | `cd $TRIAL/repo; cat .grove/_BRIEF.md; echo "-----"; cat .grove/01-impl--utc-changelog-k1.md; echo "-----"; ls -la . .grove; jj st --no-pager; jj log --no-pager …` |
| 8 | Bash | command (error) |  |  | `B=$SNAPS; for d in 88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67 820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22; do grep -r…` |
| 9 | text | text |  |  | `` I haven't written the changelog entry. The task requires a skill called `changelog-conventions` to be loaded first, and that skill doesn't exist anywhere I'm al… `` |

Ending outcome: The agent ran no ending command, and the leaf stayed live with
an empty working copy. One harness launch occurred, and the driver ran 73.4
seconds.

#### problem-stale-native-cli

**Case.** A stale native copy without a pack reference. The run is
`problem-stale-native-cli-20261102T222040Z`. The leaf asks for the design of
a command-line tool, in `docs/stamp-cli.md`, and names no skill.

The native copy is `.claude/skills/cli-tool-design`, from
`plugins/linkuistics/skills/cli-tool-design` at commit `9a35d526f334`. Its
`SKILL.md` has SHA-256
`1b7a9a6f71a00b7e518163bea2877fa6cbb667d473a79883fc651b3dd4296456`. The
directory copy has SHA-256
`816516f5322b0e1e8d6c8b6eb7a69e3dee0244136f95ca6d4211669fd9bbadb9`. The old
copy has no `paths` metadata, and Claude Code listed it at startup, in
`skills` and in the skill listing.

**Result.** **Pass.** Text 12 states that the directory's `cli-tool-design`
governs the work. Call 13 reads the directory copy from the `linkuistics`
snapshot. Call 14 then writes `docs/stamp-cli.md`. No call reads the native
copy, no further path of any call names it, and no call uses the Skill tool.

| Subject | Value |
| --- | --- |
| Descriptor identity | `c4b5f8810873f15f049c754b1c1bec67043ff34eb1eb0869bc520a84c60fec70`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `e69e827d759e1558b41c297284ad4dee7b2dde6e46b460be5298c2ad60ef07da`. Stream SHA-256 `0292956ce5b982661a27844290df286bf3634a6378277852947b125d6137a9da` |
| Skill listing at startup | 28 names, with `cli-tool-design` |

Ordered events:

| # | Tool | Class | Skill | Source | Command, path or text |
| --- | --- | --- | --- | --- | --- |
| 1 | text | text |  |  | `I'll start by reading the mandatory guidance files in order.` |
| 2 | Bash | mandatory | using-jujutsu | linkuistics | `cat "$L/skills/using-jujutsu/SKILL.md"` |
| 3 | Bash | mandatory | grove | grove | `S=$G/skills cat $S/grove/SKILL.md; ls $S/grove` |
| 4 | text | text |  |  | `` Mandatory files read. Now locating `grove-impl`. `` |
| 5 | Bash | command |  |  | `B=$SNAPS find $B/*/tree/skills -name SKILL.md -path '*grove-impl*'; ls $B/*/tree/skills` |
| 6 | Bash | kind-skill | grove-impl | grove | `G=$G/skills cat $G/grove-impl/SKILL.md; ls -R $G/grove-impl $G/grove/references` |
| 7 | text | text |  |  | `Grove spine is already loaded. Now the execute and bootstrap procedures.` |
| 8 | Bash | supporting | grove | grove | `R=$G/skills/grove/references cat $R/execute.md $R/bootstrap.md` |
| 9 | Bash | command |  |  | `cd $TRIAL/repo which grove-llm; grove-llm resolve stamp-cli-k1; echo "exit=$?"` |
| 10 | Bash | command (error) |  |  | `cd $TRIAL/repo grove-llm brief-chain .grove/01-impl--stamp-cli-k1.md; echo "exit=$?"; for f in $(grove-llm brief-chain .grove/01-impl--stamp-cli-k1.md); do echo…` |
| 11 | Bash | command |  |  | `cd $TRIAL/repo cat .grove/01-impl--stamp-cli-k1.md; echo -----; ls -la . .grove; jj st --no-pager; jj log --no-pager -n 5` |
| 12 | text | text |  |  | `` The leaf is a CLI design document, so the directory's `cli-tool-design` skill governs it. Reading that before writing. `` |
| 13 | Bash | listing | cli-tool-design | linkuistics | `D=$L/skills/cli-tool-design ls -R $D; cat $D/SKILL.md` |
| 14 | Write | action |  |  | `$TRIAL/repo/docs/stamp-cli.md` |
| 15 | text | text |  |  | `The design is written. Before retiring I need the task-format shape for the decisions log, plus the retire and commit procedures.` |
| 16 | Bash | supporting | grove | grove | `G=$G/skills/grove cat $G/TASK-FORMAT.md $G/references/retire.md $G/references/commit.md` |
| 17 | Bash | command |  |  | `cd $TRIAL/repo cat >> .grove/01-impl--stamp-cli-k1.md <<'EOF'  ## Decisions (running log)  - **Surface.** One command, no subcommands or positional arguments; f…` |
| 18 | text | text |  |  | `The root has no live leaves left. Finishing the grove is the driver's job, not this session's, so there's nothing to close. Committing and sealing now.` |
| 19 | Bash | command |  |  | `cd $TRIAL/repo jj describe -m "Design the stamp command-line interface (stamp-cli-k1)  Add docs/stamp-cli.md: usage line, flags, output format, exit codes and e…` |
| 20 | Bash | ending |  |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |
| 21 | text | text |  |  | `` The design is in `docs/stamp-cli.md`; the summary above covers it. I've told Grove the session is complete so it can carry on. `` |

Ending outcome: The leaf retired, and the agent ran `end-session
grove-continue`. One harness launch occurred, and the driver ran 101.7
seconds. The published commits are `5e545fd0c1db` (no description) and
`40049b581a3a` Design the stamp command-line interface (stamp-cli-k1).

#### problem-stale-native

**Case.** A stale native copy, with a pack reference. The run is
`problem-stale-native-20261102T221305Z`. The leaf asks for a decision record
under `docs/adr/` and names no skill.

The native copy is `.claude/skills/decision-records`, from
`plugins/linkuistics/skills/decision-records` at commit `3699e04a1ed4`. Its
`SKILL.md` has SHA-256
`e627a7c6ee70c73bef2d62445e2b532791e9f9319d7f570eb7aa98e15a1ec4ab`. The
directory copy has SHA-256
`6b9b8d456b3a67004c1910d3458821221695b4c45c77b0a3f78b4e5b0c68f5a4`. Claude
Code listed the native copy at startup.

**Result.** **Pass**, with a pack reference. Call 11 has the class
`supporting`, because its first path is Grove's `ADR-FORMAT.md`. The same
command then prints the directory copy of `decision-records/SKILL.md` from the
`linkuistics` snapshot. Call 13 writes the record by a heredoc.

No call reads the native copy, and no call uses the Skill tool. `ADR-FORMAT.md` names
`linkuistics:decision-records`, so this choice is not free of a reference.

| Subject | Value |
| --- | --- |
| Descriptor identity | `362597359fbbafe6d2b34c3c6ba98b3de8b3a4e4c4760d08302a0e8b77f24754`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `a8278f84eedf3fe3aff1306147fcab6a5409acb894a319fe8c535d9f83bdc494`. Stream SHA-256 `7f24ba2c37f4ba6b69dddc302510874cf9455dd9078d28cf0f1b9a60df0c4c16` |
| Skill listing at startup | 15 names, with `decision-records` |

Ordered events:

| # | Tool | Class | Skill | Source | Command, path or text |
| --- | --- | --- | --- | --- | --- |
| 1 | Bash | mandatory | using-jujutsu | linkuistics | `cat "$L/skills/using-jujutsu/SKILL.md"` |
| 2 | Bash | mandatory (error) | grove | grove | `S="$SNAPS"; cat "$S/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills/grove/SKILL.md"; echo ======; cat "$S/820fe28234c98fe59286c3788…` |
| 3 | Bash | supporting | using-jujutsu | linkuistics | `S="$SNAPS"; cat "$S/820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22/tree/skills/using-jujutsu/references/jj-commands.md"; echo; echo -----; fin…` |
| 4 | text | text |  |  | `` One `grove-impl` in the directories. Loading it. `` |
| 5 | Bash | kind-skill | grove-impl | grove | `G="$G/skills"; cat "$G/grove-impl/SKILL.md"; ls -R "$G/grove-impl" "$G/grove"` |
| 6 | Bash | supporting (error) | grove | grove | `G="$G/skills/grove/references"; cat "$G/bootstrap.md"; echo =====; cat "$G/execute.md"` |
| 7 | Bash | supporting | grove | grove | `G="$G/skills/grove/references"; cat "$G/execute.md"` |
| 8 | text | text |  |  | `Now resolving the mandated handle.` |
| 9 | Bash | command |  |  | `cd "$TRIAL/repo"; grove-llm resolve utc-record-k1; echo "exit=$?"` |
| 10 | Bash | command |  |  | `cd "$TRIAL/repo"; cat .grove/01-impl--utc-record-k1.md; echo; echo "--- chain"; grove-llm brief-chain .grove/01-impl--utc-record-k1.md; echo "exit=$?"; ls -la .…` |
| 11 | Bash | supporting | grove | grove | `cd "$TRIAL/repo"; cat .grove/_BRIEF.md; S="$SNAPS"; echo "--- ADR-FORMAT"; cat "$S/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills/…` |
| 12 | Bash | supporting | grove | grove | `S="$G/skills/grove"; cat "$S/TASK-FORMAT.md"; echo "--- retire"; cat "$S/references/retire.md"; echo "--- commit"; cat "$S/references/commit.md"` |
| 13 | Bash | command |  |  | `cd "$TRIAL/repo"; mkdir -p docs/adr; cat > docs/adr/utc-iso-8601-timestamps.md <<'EOF' # Timestamps are written in UTC, in ISO 8601 form  Every timestamp this r…` |
| 14 | Bash | command |  |  | `cd "$TRIAL/repo"; grep -rn "Reviews:" .grove; grove-llm leaf-retire .grove/01-impl--utc-record-k1.md; echo "exit=$?"; ls .grove; jj describe -m "utc-record-k1: …` |
| 15 | Bash | ending |  |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |

Ending outcome: The leaf retired, and the agent ran `end-session
grove-continue`. One harness launch occurred, and the driver ran 70.8
seconds. The published commits are `25a8d9ca5c80` (no description) and
`6d93f3027643` utc-record-k1: record the UTC ISO 8601 timestamp decision.

#### problem-user-invoked

**Case.** A user-invoked skill and matching activity. The run is
`problem-user-invoked-20261102T221416Z`. The leaf asks for
`skills/utc-timestamps/SKILL.md` and names no skill.
`authoring-conventions` sets `disable-model-invocation: true` at both
preparation commits.

**Result.** **Pass.** Call 16 prints lines 1 to 8 of
`authoring-conventions/SKILL.md`. Those lines are the front matter, a blank
line and the heading. The classifier names the call `skill`, because the path
is a `SKILL.md`. Text 17 then says:

> That skill sets `disable-model-invocation: true` and nobody named it, so I
> won't load it. Writing the skill file now.

Call 18 writes the file. No call reads the body of `authoring-conventions`.

| Subject | Value |
| --- | --- |
| Descriptor identity | `f344dce43524b0e321f8beb364532ad1dd84418f5be4db0a00998798830bcb53`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `1397a2df9da9d0e51ad05cedab2fd652779a68e4a4f2c08c021b7d815daee205`. Stream SHA-256 `3f32856a77b52ed4f9f9e2dc37a035cdaeecc2185472eb85c25a4b7299c2902a` |
| Skill listing at startup | 27 names |

Ordered events:

| # | Tool | Class | Skill | Source | Command, path or text |
| --- | --- | --- | --- | --- | --- |
| 1 | text | text |  |  | `I'll start by reading the mandatory files in order.` |
| 2 | Bash | mandatory | using-jujutsu | linkuistics | `cat "$L/skills/using-jujutsu/SKILL.md"` |
| 3 | Bash | supporting (error) | using-jujutsu | linkuistics | `D="$SNAPS"; cat "$D/820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22/tree/skills/using-jujutsu/references/jj-commands.md"; echo ======; cat "$D/…` |
| 4 | Bash | mandatory | grove | grove | `D="$SNAPS"; cat "$D/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills/grove/SKILL.md"` |
| 5 | text | text |  |  | `` Both mandatory files are read. Now I'll locate the `grove-impl` skill in the search directories. `` |
| 6 | Bash | command |  |  | `D="$SNAPS"; for f in "$D"/88f6*/tree/skills/*/SKILL.md "$D"/820f*/tree/skills/*/SKILL.md; do echo "$f"; grep -m1 '^name:' "$f"; done` |
| 7 | text | text |  |  | `` Exactly one `grove-impl` exists, in the first search directory. Loading it now. `` |
| 8 | Bash | kind-skill | grove-impl | grove | `cat "$G/skills/grove-impl/SKILL.md"` |
| 9 | text | text |  |  | `` The `grove-impl` skill loaded; it sends me back to the `grove` spine, which I've already read. Next, the spine says to follow `references/execute.md` when start… `` |
| 10 | Bash | listing | grove | grove | `G="$G/skills/grove"; ls "$G" "$G/references"; cat "$G/references/execute.md"` |
| 11 | text | text |  |  | `` Now the bootstrap procedure, which tells me how to resolve the mandated leaf `utc-skill-k1`. `` |
| 12 | Bash | supporting | grove | grove | `G="$G/skills/grove"; cat "$G/references/bootstrap.md"` |
| 13 | Bash | command |  |  | `cd "$TRIAL/repo" && P=$(grove-llm resolve utc-skill-k1); echo "resolved: $P"; grove-llm brief-chain "$P"` |
| 14 | Bash | command |  |  | `cd "$TRIAL/repo" && cat .grove/_BRIEF.md; echo =====; cat .grove/01-impl--utc-skill-k1.md; echo =====; ls -la . .grove; ls docs 2>/dev/null; find . -name 'CONTE…` |
| 15 | Bash | command |  |  | `cd "$TRIAL/repo" && cat .grove/01-impl--utc-skill-k1.md; echo '-----'; ls -la . .grove; find . -name 'CONTEXT*.md' -not -path './.jj/*'; jj st --no-pager` |
| 16 | Bash | skill | authoring-conventions | linkuistics | `D="$L/skills/authoring-conventions"; sed -n '1,8p' "$D/SKILL.md"` |
| 17 | text | text |  |  | `` That skill sets `disable-model-invocation: true` and nobody named it, so I won't load it. Writing the skill file now. `` |
| 18 | Write | action |  |  | `$TRIAL/repo/skills/utc-timestamps/SKILL.md` |
| 19 | Bash | supporting | grove | grove | `date -u +%Y-%m-%dT%H:%M:%SZ; G="$G/skills/grove"; cat "$G/references/retire.md"; echo '-----'; cat "$G/references/commit.md"` |
| 20 | Bash | command |  |  | `cd "$TRIAL/repo" && grep -rn 'utc-skill-k1\b' .grove; grove-llm leaf-retire .grove/01-impl--utc-skill-k1.md && ls .grove && jj describe -m "Add utc-timestamps s…` |
| 21 | Bash | ending |  |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |

Ending outcome: The leaf retired, and the agent ran `end-session
grove-continue`. One harness launch occurred, and the driver ran 112.7
seconds. The published commits are `11bd9bb7e9c7` (no description) and
`9dacbc41b5bd` Add utc-timestamps skill (utc-skill-k1).

#### problem-user-invoked-named

**Case.** A user-invoked skill that the caller names. The run is
`problem-user-invoked-named-20261102T221609Z`. The leaf is the task of
`problem-user-invoked`, with a Guidance section: "Load the skill
`authoring-conventions` before you write the file."

**Result.** **Pass.** Text 9 states that the task names the skill and that it
exists in the configured directories. Call 10 reads the whole
`authoring-conventions/SKILL.md` from the `linkuistics` snapshot and lists
its directory. Text 11 states that the explicit name permits the load despite
`disable-model-invocation: true`.

Call 14 then writes the file. No call uses the Skill tool.

| Subject | Value |
| --- | --- |
| Descriptor identity | `4b814d4877245c9b9426c1a475ea046fa24e9e29d7041102f20c3b833696d21e`, equal to the SHA-256 of `directories.json` |
| Raw transcript | SHA-256 `c5ff6d8772166d9e92deeb37013370266c694591b99d56bfffc1562be53c7f84`. Stream SHA-256 `905d625680f37c71f4a108ed97dfe6c9428ed2ac06d21547d76ea8fc707c6e66` |
| Skill listing at startup | 14 names |

Ordered events:

| # | Tool | Class | Skill | Source | Command, path or text |
| --- | --- | --- | --- | --- | --- |
| 1 | Bash | mandatory (error) | using-jujutsu | linkuistics | `S1="$SNAPS"; cat "$S1/820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22/tree/skills/using-jujutsu/SKILL.md"; echo ======; cat "$S1/88f68c31337536…` |
| 2 | Bash | mandatory | grove | grove | `S1="$SNAPS"; cat "$S1/88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67/tree/skills/grove/SKILL.md"` |
| 3 | text | text |  |  | `` Now locating the `grove-impl` skill in the configured directories. `` |
| 4 | Bash | command |  |  | `S1="$SNAPS"; for d in 88f68c31337536efb0d4fb388543c0ca1abf9a9fed0ce0ccac3888e933df5a67 820fe28234c98fe59286c3788eac561dead174770bd50560393d697788446b22; do find…` |
| 5 | Bash | kind-skill | grove-impl | grove | `S="$G/skills"; cat "$S/grove-impl/SKILL.md"; ls -R "$S/grove-impl" "$S/grove"` |
| 6 | Bash | supporting | grove | grove | `R="$G/skills/grove/references"; cat "$R/execute.md"; echo; echo -----; cat "$R/bootstrap.md"` |
| 7 | Bash | command (error) |  |  | `P=$(grove-llm resolve utc-skill-k1); echo "$P"; grove-llm brief-chain "$P"; ls -la; ls Taskfile.yml 2>&1` |
| 8 | Bash | command |  |  | `cat .grove/_BRIEF.md; echo -----; cat .grove/01-impl--utc-skill-k1.md; ls -la .grove; jj --no-pager log -n 5; jj st` |
| 9 | text | text |  |  | `` The task names the `authoring-conventions` skill; it exists in the configured directories, so I'm reading it from there. `` |
| 10 | Bash | skill | authoring-conventions | linkuistics | `D="$L/skills/authoring-conventions"; cat "$D/SKILL.md"; ls -R "$D"` |
| 11 | text | text |  |  | `` The skill sets `disable-model-invocation: true`, but the task file names it explicitly, so loading it is permitted. Now the retire and commit procedures, plus t… `` |
| 12 | Bash | supporting | grove | grove | `G="$G/skills/grove"; cat "$G/references/retire.md"; echo -----; cat "$G/references/commit.md"; echo -----; grep -n -A15 -i "decisions" "$G/TASK-FORMAT.md" \| hea…` |
| 13 | text | text |  |  | `` I have everything needed. Writing the skill file, following the house rules: capability + "Use when" description, the required `harnesses:` key, and a short bod… `` |
| 14 | Write | action |  |  | `$TRIAL/repo/skills/utc-timestamps/SKILL.md` |
| 15 | text | text |  |  | `Quick check that the example shell command produces the stated format, then retire the leaf and commit.` |
| 16 | Bash | command |  |  | `date -u +%Y-%m-%dT%H:%M:%SZ && grep -rl "Reviews:" .grove \|\| echo "no review leaves"; grove-llm leaf-retire .grove/01-impl--utc-skill-k1.md && ls .grove && jj d…` |
| 17 | Bash | ending |  |  | `'$RAVEL2/bin/ravel' end-session grove-continue` |
| 18 | text | text |  |  | `` Session ended with `grove-continue`. `` |

Ending outcome: The leaf retired, and the agent ran `end-session
grove-continue`. One harness launch occurred, and the driver ran 58.2
seconds. The published commits are `4b9c236781a4` (no description) and
`f65a406b739c` Add utc-timestamps skill (utc-skill-k1).

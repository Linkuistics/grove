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
`scripts/harness-trials/cases/`. Its `case.json` names the route, the kinds
that the trial routes, the sources, the mandatory files, the model and the
effort. Its `tree/` becomes the `.grove/` directory of the disposable
repository. The runner supports the routes native on 2.0 and directory on
2.0, and the harness Claude Code. A case on native 1.1 needs a `policy.ts`
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
relative paths resolve. The record shows a long command shortened.

### The controls

`task trial:controls` runs `scripts/test_harness_trial.py`, which joins
`scripts/check.sh`. A fake driver and a fake harness stand in for Grove, Ravel
and Claude Code. The tests prove the refusals, the allow-list, the link list,
the authority control and the event extraction. Each check also runs once
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

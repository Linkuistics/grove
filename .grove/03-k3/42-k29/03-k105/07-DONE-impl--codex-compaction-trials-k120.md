# codex-compaction-trials-k120

## Goal

Measure the two compaction cases in Codex as separate trials. The first loses
the path, and the second loses the whole preamble.

## Context

The leaf claude-compaction-trials-k112 states the cases, the spec sections and the notes. Read it and
its record before this leaf runs.

## Done when

- Each Done when item of claude-compaction-trials-k112 holds in Codex, as a separate launch with its own
  record.
- The record names the compaction mechanism of Codex and shows the
  model-visible input after compaction. If no mechanism can run, each trial
  is an unmeasured gap.
- The record compares each Codex result with the Claude Code result of the
  same case.

## Decisions (running log)

- Mechanism, from the source at the tag `rust-v0.166.0`. `codex exec` has
  no compaction command. The app-server request `thread/compact/start` takes
  only `threadId`, so it takes no summary instructions. The OpenAI provider
  has `RemoteCompactionSupport::V2` (`model-provider/src/provider.rs`). The
  manual task then runs `compact_remote_v2::run_remote_compact_task`, and
  `compact_prompt` applies only to local compaction (`core/src/tasks/compact.rs`).
- Remote compaction v2 keeps each user message word for word, newest first,
  up to 64,000 tokens (`RETAINED_MESSAGE_TOKEN_BUDGET`,
  `build_v2_compacted_history`), then one opaque `compaction` item with
  `encrypted_content`. Local compaction keeps user messages up to 20,000
  tokens. Ravel's preamble is in the first user message. Neither mechanism
  can lose the path or the preamble in this case.
- The development feature `token_budget` makes a manual compaction install a
  fresh context window (`compact_token_budget.rs`,
  `Session::start_new_context_window`). The window keeps no user message.
  Codex adds `<context_window_guidance>`: after a reset, read the notes and
  use the read-only `history` tool. That is Codex's own generic recovery
  channel, as the transcript line is for Claude Code.
- Probe in a sandbox home, `codex app-server` over stdio, `gpt-6.1-sol` at
  effort `low`. Turn 1 gave a fact with a path, then `thread/compact/start`,
  then turn 2 asked for the path. Default: the `compacted` entry's
  `replacement_history` kept the turn-1 user message with the path, plus one
  encrypted `compaction` item, and the answer gave the path. With
  `--enable token_budget`: the replacement history held only developer
  messages and the environment context, and the answer was "UNKNOWN".
- How the trial reads the context after compaction: the session file's
  `compacted` entry holds `replacement_history`, which is the history that
  Codex installs. The model-visible input of the next request is that
  history, then the items that Codex records before the answer's response.
- The trial compacts inside Ravel's managed process, as in Claude Code. The
  feeder runs `codex app-server` over stdio: `thread/start`, the prompt as a
  turn, `thread/compact/start`, the gate, then the answer as a turn. Each
  step waits for `turn/completed` of the main thread.
- Mechanism per case. `compaction-path` uses the default mechanism, because
  no Codex mechanism keeps the instructions and loses the paths.
  `compaction-preamble` uses `--enable token_budget`, the only Codex
  mechanism that loses the preamble. The case's summary instructions do not
  reach Codex, and each record says so.
- Runner: the feeder, the compaction turn, the readers of the context after
  compaction and the record lines are now adapter methods. The Claude Code
  forms moved without change. The Codex readers take the `compacted` entry's
  replacement history, and they count an encrypted item as opaque, which
  fails the gate. A control of a feeder-side method breaks the in-process
  reading, because the feeder runs in its own process. `task trial:controls`
  passes 51 tests.
- Build: Grove commit `221d9891dc13`, the runner only. The Ravel 2.0 prefix
  is a byte copy of an earlier session's prefix, compared file for file.
  `bin/ravel` SHA-256 `8e10e311c50c…`, no `libexec`. The bookmark
  `skill-directory-delivery` is still at `1ca4aecbeae1`.
- Result of `compaction-path-20261103T043647Z`, one run with no retry:
  failed precondition, an unmeasured gap. The replacement history kept the
  prompt word for word, with 5 preamble paths and 3 digests, and one
  encrypted `compaction` item of 27,768 characters. The gate sent no answer.
  The first turn read both mandatory files in order, then `grove-impl`, then
  `coding-style-bash` before it wrote `stamp.sh`, and asked the human in text.
- Result of `compaction-preamble-20261103T043956Z`, one run with no retry:
  precondition met, result a gap. The gate passed: the replacement history
  held 7 items, none with preamble text, and the first request's input added
  only the answer. The variable stayed set to the descriptor. After the
  answer the agent searched the repository, re-read the brief and the task
  file, edited `stamp.sh` and reported. It read neither the variable nor the
  descriptor nor a mandatory file, and it ran no ending command. Ravel
  refused as `session_result_missing`.
- Limit of the preamble case: `token_budget` loses the whole conversation,
  not only the preamble. The mandate, the leaf, the work done and the ending
  commands went too. The window guidance names `notes` and `history` tools.
  The thread's 12 tools held neither, and `token_budget` added `new_context`.
  No Codex mechanism loses the preamble alone.
- The first turn's four `python3` heredoc reads have the class `command`.
  They read the spine's `bootstrap.md` and `execute.md`, the front matter of
  every snapshot `SKILL.md`, and `coding-style-bash` with
  `machine-environment`. The fourth tried the absent sandbox
  `~/.config/mise/config.toml`.
- No in-session reviewer. The results are observations of session files, and
  each new runner path has a fake-harness check with controls that fail
  against a broken runner. `acceptance-handoff-k106` reviews the trial
  evidence of both harnesses.

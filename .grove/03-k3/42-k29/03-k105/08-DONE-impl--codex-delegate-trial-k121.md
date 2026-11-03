# codex-delegate-trial-k121

## Goal

Observe one real Codex delegate. Separate its reads, its adherence and its
inherited ending authority.

## Context

The leaf claude-delegate-trial-k113 states the cases, the spec sections and the notes. Read it and
its record before this leaf runs.

## Done when

- Each Done when item of claude-delegate-trial-k113 holds in Codex, as a separate launch with its own
  record.
- The record names the delegate mechanism, such as the stable `multi_agent`
  feature of Codex 0.166.0. It states whether the delegate shares the parent's
  process environment and sandbox.
- The record compares each Codex result with the Claude Code result of the
  same case.

## Decisions (running log)

- Delegate mechanism: Codex 0.166.0 runs multi-agent v2. The tools are in
  the `collaboration` namespace, and `spawn_agent` takes `task_name`,
  `message`, `fork_turns`, `agent_type`, `model` and `reasoning_effort`
  (`codex-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs` at
  `rust-v0.166.0`). The delegate is a thread of the same Codex process.
- `fork_turns` defaults to `all`, which gives the delegate the parent's whole
  history, preamble included. `none` gives no history. The record names the
  fork mode of each delegation, and whether the delegate's history holds the
  descriptor path.
- Sandbox: `build_agent_shared_config` in `codex-rs/core/src/agent/child_config.rs`
  copies the parent's configuration, then sets the turn's approval policy,
  working directory and permission profile. The child also inherits
  `developer_instructions`. A scratch probe, outside every trial, showed the
  same `workspace-write` policy in the child's `turn_context`, and the child
  printed a variable of the parent's process environment.
- What the parent gave: the `message` parameter of `spawn_agent` is declared
  `with_encrypted()` (`multi_agents_spec.rs`). In the probe, the parent's
  `function_call` arguments and the child's `agent_message` both held the
  message as an opaque Fernet token. A direct call therefore hides the
  prompt from every transcript. Only a call through code mode, inside the
  `exec` tool's JavaScript, is plaintext (`agent_message_from_tool` in
  `multi_agents_v2.rs`). The record states which form each call took, and
  an opaque prompt is an unmeasured part of the row, not a pass.
- Link: the parent's `SubAgentActivity` item of kind `started` carries the
  spawn's `call_id` as its `id` and the child's `agent_thread_id`. The child
  rollout's `session_meta` names `parent_thread_id` and
  `source.subagent.thread_spawn` with the depth and the agent path.
- Owner channel: `-c developer_instructions="…"` adds a developer message
  before the skill instructions. `codex debug prompt-input` showed the probe
  text there. It replaces Claude Code's `--append-system-prompt` for
  `systemNote`. The delegate inherits the note as well.
- `delegate-early` log: Codex's `workspace-write` sandbox denies writes in
  `$TRIAL`, so the redirection to `$TRIAL/authority.log` would stop the whole
  command. The case now writes `$TRIAL/tmp/authority.log`, in the `TMPDIR`
  of the trial, and collects `tmp/authority.log`. The Claude Code run at
  `ebc7fa64f277` used the earlier path.
- Build: Grove commit `5a2d48c6e813`, prepared into this session's scratch
  directory. The Ravel 2.0 prefix is a byte copy of an earlier trial prefix:
  5 files, identical digests, no `libexec`, `bin/ravel` SHA-256
  `8e10e311c50c…`. The bookmark `skill-directory-delivery` is still at
  `1ca4aecbeae1`.
- The Codex `item` mapping now names a `SubAgentActivity` item of kind
  `started` `spawn_agent`, class `delegate`. A collaboration item such as `wait` keeps
  `collab:<tool>` and never counts as a delegation.
- Result of `delegate-20261103T051533Z`, one run with no retry: pass on
  every observable row. The parent read both mandatory files, `grove-impl`,
  `coding-style-bash` and `machine-environment`, then spawned one delegate
  directly, with `fork_turns="none"`. The message was encrypted. The
  delegate read the descriptor, both mandatory files in order and the
  spine's `execute.md`, then `coding-style-bash` and `machine-environment`,
  then wrote `check-count.sh`. It ran no ending command. The parent ended
  with `grove-continue`.
- What the parent gave is unmeasured: the message is encrypted. The
  delegate's file names the descriptor path first in its own `cat`.
  Nothing before it holds the path. The path therefore reached the
  delegate through the encrypted message. That is an inference, not a measurement.
  Whether the message named `coding-style-bash` is unknown, so the craft
  read is no evidence of discovery from activity alone.
- The two agents exchanged messages: two `send_message` calls of the parent,
  three messages of the delegate and its final answer. The first reader
  took the delegate's first message as the result. The reader now takes the
  `FINAL_ANSWER` message and counts the messages each way, and
  `trial:reextract` read the unchanged transcripts again at
  2026-11-03T05:19:01Z.

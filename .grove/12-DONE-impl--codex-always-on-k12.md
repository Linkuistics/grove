# codex-always-on-k12

## Goal

Make the skill always on in Codex as far as Codex allows, as the design maps it:
the instruction, the reminder, the document gate and the reply log wherever Codex
has a hook for them, and the skill's own instruction to run the checker where it
has none. The one switch turns it all off.

## Context

- Read the design `docs/specs/technical-communication.md` and `harness-hooks-a.md`
  for Codex's hook events, configuration and pitfalls, and reuse the shared
  scripts and the switch from `claude-code-always-on-k11` through the thin
  adapter pattern it set.
- Skills reach Codex through Grove's startup provisioning, which embeds a
  snapshot of `plugins/` in the Grove binary (`docs/specs/codex-skill-provisioning.md`),
  and through `plugins/install.sh`. Executable flags on supporting scripts are
  preserved. Check that anything this leaf adds under `plugins/` reaches Codex by
  both routes.
- The route is the spec's: a hooks plugin whose root is the skill directory
  (`.codex-plugin/plugin.json`, with its entries in `harnesses/codex/hooks.json`
  and a Codex adapter in `tc_hook.py`). The human adds and trusts it once by
  hand. Never write trust into the human's configuration. For the live run, use
  a scratch `CODEX_HOME`, where trust through the TUI or
  `--dangerously-bypass-hook-trust` is acceptable for the test only. Check that
  the entries still reach the scripts if Codex copies the plugin.
- This repository never writes Codex's global `AGENTS.md`
  (`skill-design-k5`). `publication-k16` offers the human an optional line.
- A project memory records that Codex's sandbox blocks writes to the git
  directory. Check what it does to a hook that writes the reply log.

## Done when

- In a real Codex session, show by running it which layers work and which fall
  back to the model running the checker, and that the switch turns them off.
  Put the evidence in the running log, with the exact commands the human runs to
  add and trust the plugin, for `publication-k16`'s user document.
- `bash scripts/check.sh` is green, and the provisioning path still passes its
  checks.

## Notes

- Until the human trusts the plugin, Codex has the skill and the fallback.
  Say so plainly.

## Decisions (running log)

- **Built.** `plugins/linkuistics/skills/technical-communication/.codex-plugin/plugin.json`
  (name `technical-communication`, `hooks` pointing at the file below),
  `harnesses/codex/hooks.json` (six entries) and a `Codex` adapter in
  `scripts/tc_hook.py`. The adapter reuses `handle`, `gate`, `gate_message` and
  `never_gated` unchanged. Both adapters now share an `Adapter` base class for
  what the harnesses agree on (`session_id`, `agent_id`, `source`, `cwd`,
  `last_assistant_message`, the nested `additionalContext` output), and each
  subclass holds only what differs. No test and no Taskfile task was added
  (`plan-k1`: no leaf builds tests).
- **Codex adds a plugin only from a marketplace, so the repository gets
  `.agents/plugins/marketplace.json`.** It lists one plugin, with the skill
  directory as its source. Codex reads it before `.claude-plugin/marketplace.json`
  (the marketplace came out named `linkuistics-hooks`, not `linkuistics`), so
  Claude Code's marketplace is untouched. The file is outside `plugins/`, so
  Grove's embedded snapshot does not carry it, and nothing needs it there.
- **A Codex entry is a shell command string** (Codex has no `args` field):
  `/bin/sh "${PLUGIN_ROOT}/scripts/tc-hook" codex EVENT || true`. `|| true`
  makes a missing guard file exit 0, where Codex would show a failed hook row.
  Timeouts 10 (`SessionStart`, `SubagentStart`, `Stop`, `SubagentStop`), 5
  (`UserPromptSubmit`) and 15 (`PostToolUse`, matcher `apply_patch`). These entries
  never change shape (spec, "Harness mapping").
- **The gate reads paths from the patch text**: the files of `*** Add File:`,
  `*** Update File:` and `*** Move to:` headers, relative to `cwd`. `Delete File`
  is not read. Added-file content lines start with `+`, so they cannot pass for a
  header.
- **`model` comes straight from the Codex payload.** No transcript read is needed
  (Codex 0.163.0 puts `model` in every event; `transcript_path` is null in an
  ephemeral session).
- **Spec updated** (`docs/specs/technical-communication.md`): layout, the
  marketplace file, the cache-snapshot rule, the entry shape, the patch parsing and
  the `model` source. No ADR changed: none of these is a one-way door that is
  surprising without context.
- **A live model session could not use a scratch `CODEX_HOME`.** The auto-mode
  classifier refused to let me read `~/.codex/auth.json` (credential
  exploration), and I did not copy or link it. So the evidence is in two parts.
  (1) The plugin route, in a scratch `CODEX_HOME` with no credentials: install,
  hook listing, trust, and the two hooks that run before the model request.
  (2) The real-model layers, in the real home for authentication only
  (`codex exec --ephemeral --ignore-user-config`), with the same six entries from
  `harnesses/codex/hooks.json` passed as `-c hooks.*` session flags that point
  at the cached scripts, and `--dangerously-bypass-hook-trust`. Part 2 does not
  test Codex's plugin loader, and part 1 does not test a model reading the context.

## Evidence

Codex 0.163.0. Every run set `TECHNICAL_COMMUNICATION_STATE_DIR` (one `env -i`
probe cleared it and wrote one synthetic start record to the real log; I removed
that single file). The live work directory was `~/.cache/tc-live-k12`, outside every
temporary directory, so the gate did not skip it. Both it and the state
directories are deleted.

1. **Plugin install.** `codex plugin marketplace add <checkout>` then
   `codex plugin add technical-communication@linkuistics-hooks` copies the whole
   skill directory to `$CODEX_HOME/plugins/cache/linkuistics-hooks/technical-communication/local/`
   (byte-identical to the source, `__pycache__` included).
   `hooks/list` from `codex app-server` shows six hooks, `source: plugin`, no
   warnings or errors, each command resolved to the cached `scripts/tc-hook`, all
   `trustStatus: untrusted`.
2. **The cache is a snapshot.** I first installed the plugin before the adapter
   existed. The hook ran the cached copy, which did not know `codex`, and did
   nothing, with no message. After `codex plugin add` again, the cache matched the
   source and the hook wrote its record. The human reinstalls after the scripts change.
3. **Untrusted entries are skipped silently.** `codex exec` without the bypass flag
   printed no `hook:` line and wrote no state. After I wrote
   `[hooks.state."<key>"] trusted_hash` for the six keys into the scratch
   `config.toml` (the hashes Codex itself reported), the same command ran the
   hooks and wrote a start record. `hooks/list` then showed six `trusted`.
4. **Trust survives a reinstall and not an entry change.** `codex plugin remove`
   then `add` again: six `trusted`. Control: changing one timeout in the cached
   `hooks.json` gave that entry `modified` and five `trusted`.
5. **Instruction and reminder reach the model's input.** Scratch home, plugin
   route, no credentials, rollout file read afterwards: a developer message of
   1,888 characters (the instruction) before the user message, and a developer
   message of 119 characters (the reminder) after it. With the off file present:
   the user message only, and the state directory held only `off`.
6. **Real model, instruction and gate.** A real turn asked for `notes.md` through
   `apply_patch` with "We utilize the optimized system; it was organized." The model
   quoted the instruction's opening sentence word for word and reported "American
   spellings 'utilize', 'optimized', and 'organized', plus a semicolon". The same
   run wrote one `start` and one `reply` record (`harness: codex`, `model:
   gpt-6.1-sol`).
7. **The log write is not blocked by the sandbox.** That run used `-s
   workspace-write` with a work directory `~/.cache/tc-live-k12` and a state
   directory `~/.cache/tc-live-k12-state`, a sibling outside every writable root.
   The records were written. This is the answer to the project memory's worry: hooks run
   outside the sandbox.
8. **The reminder did not turn the prompt into a task.** In every real run the model
   did the task asked for. (openai/codex#40680 saw a model answer "Understood. Ready
   for the task." to a trailing directive.) A model's own report of what it holds
   is unreliable: a run with no tool call said `none` for both lines, although the
   hooks had run and item 5 shows what they sent. I used the rollout file, not the
   model, as the evidence.
9. **Subagents.** A real turn that spawned a subagent produced `SubagentStart`
   (payload has `agent_id`, `agent_type`, and the parent's `session_id`), a
   `PostToolUse` for `apply_patch` with the subagent's `agent_id` (so the gate runs
   inside it), and `SubagentStop`. The log holds a `start` with `source:
   "subagent"` and a `reply` for the subagent, each with `agent` set, then the main
   thread's `reply`. I did not read back whether the subagent's model received the gate
   message.
10. **The adapter on synthetic patches.** One patch with an added file in a
    directory whose name has a space, an update to a clean file, a move to a file
    with a violation, a delete, a `.py` file, and an added-file content line
    `+*** Add File: injected-from-content.md` gave a 611-character message naming
    only the two bad files. Garbage, an array, a missing `tool_input`, a
    non-string command, an empty input and an unknown event all gave exit 0 and
    nothing (an empty input to `prompt` still sends the reminder, as designed).
    `env -i` (empty `PATH`) still sent the instruction. With the off file, all four
    events printed nothing and wrote nothing.
11. **Both delivery routes reach the new files.** Grove's embedded inventory
    (`target/debug/build/grove-*/out/plugin_inventory.rs`) lists
    `.codex-plugin/plugin.json`, `harnesses/codex/hooks.json`, `tc-hook`,
    `tc_hook.py` and `ste-lint.py` (the build includes dotdirectories; it also
    embeds an ignored `__pycache__` file when one exists, which is not new).
    `HOME=<temp> plugins/install.sh --force` links the whole directory into
    `.codex/skills/`, and all three paths resolve. The plugin adds no second
    skill entry: `codex debug prompt-input` lists `configure-grove` (control) and a
    probe skill I placed, and no `technical-communication` from the plugin cache.
12. **`bash scripts/check.sh`: 18 of 19 pass**, including `shellcheck`, the
    checker self-test, `plugin install`, conformance, `cargo nextest` (1,385
    tests, with the provisioning ones) and `book-check`. The one failure is
    `Ravel caller assets and provenance`: `policy typecheck: Bun 1.4.2 is pinned`
    and this machine has Bun 1.4.3. That check does not read any file I changed.
13. **Spec text.** `ste-lint.py` on the spec: 14 hard findings before and after,
    none on a line I wrote. The 14 are old, and the spec is over the 3,000-word
    threshold for lazy conversion, so converting it is not this leaf's work.

**Not shown.** The TUI's "Hooks need review" prompt and `/hooks` screen: the TUI
would not render under `script` here, so the trust steps below rest on the
survey's reading of the 0.163.0 source. Also not run: a Git marketplace
(`Linkuistics/grove`) before this work is on GitHub, a shell-routed patch (the gate
cannot see one, as the spec says) and a `/goal` or `/review` turn.

## For `publication-k16`: what the human runs

Until the human trusts the plugin, Codex has the skill and the fallback: the model
runs the checker itself. No layer runs, and `codex exec` skips the entries without a word.

```sh
# 1. Add the marketplace. Use the GitHub form after this work is released;
#    until then, give the path of a checkout (the form that I ran).
codex plugin marketplace add Linkuistics/grove
codex plugin marketplace add /path/to/grove-checkout

# 2. Install the plugin. This copies the skill directory into Codex's cache.
codex plugin add technical-communication@linkuistics-hooks

# 3. Trust its six entries, once. Start `codex`. At the "Hooks need review"
#    prompt, review them and trust them, or run /hooks and trust each
#    technical-communication entry. Do not choose "Continue without trusting".
```

- **Refresh after an update**: run step 2 again (after `codex plugin marketplace
  upgrade linkuistics-hooks` for a Git marketplace; I did not run that command).
  Trust stays, because the entries do not change.
- **Switch off and on**: `mkdir -p ~/.local/state/technical-communication &&
  touch ~/.local/state/technical-communication/off` turns every layer off at its
  next event, and deleting the file turns them on. Text that a running session holds stays until it ends.
- **Layers on Codex once trusted**: the instruction (`SessionStart`,
  `SubagentStart`), the reminder (`UserPromptSubmit`), the document gate
  (`PostToolUse` on `apply_patch`) and the reply log (`Stop`, `SubagentStop`).
  Fallback to the model running the checker: a file written through the shell, and
  everything before trust.

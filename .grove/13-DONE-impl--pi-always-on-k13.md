# pi-always-on-k13

## Goal

Make the skill always on in Pi as far as Pi allows, as the design maps it, with
the same layers, fallback and switch as the other harnesses.

## Context

- Read the design `docs/specs/technical-communication.md` and `harness-hooks-a.md`
  for Pi's extension or hook mechanism and its pitfalls, and reuse the shared
  scripts and switch from `claude-code-always-on-k11`.
- The extension is `harnesses/pi/technical-communication.ts`. It finds the
  skill directory through its own real path, imports only Node's built-in
  modules, keeps its factory to registrations, and catches every error in every
  handler, because a broken extension stops Pi from starting.
- `plugins/install.sh` links the skill into `~/.pi/agent/skills`. This leaf
  makes it also link the extension into `~/.pi/agent/extensions/`, with the same
  ownership and reconciliation rules as its skill links, covered by
  `plugins/install.test.sh`.
- This repository never writes Pi's global `AGENTS.md` (`skill-design-k5`). A
  project memory records that `~/.pi/rules/` is not loaded, verified in Pi's
  `resource-loader.js`; do not rely on it.

## Done when

- In a real Pi session, show by running it which layers work and which fall back
  to the model running the checker, and that the switch turns them off. A project
  memory records that Pi needs `tail -f /dev/null` alongside `script -q` to be
  driven unattended. Put the evidence in the running log.
- If a Pi load check that spends no model quota exists, it runs in
  `scripts/check.sh`.
- `bash scripts/check.sh` is green.

## Decisions (running log)

- **Built.** `harnesses/pi/technical-communication.ts` (the extension), a `Pi`
  adapter and an `instruction` event in `scripts/tc_hook.py`, the Pi link in
  `plugins/install.sh` with seven cases in `plugins/install.test.sh`, and
  `scripts/pi-extension-load.test.sh` (in `scripts/check.sh` and its shellcheck
  list). The spec has the Pi extension's design (new bullet in "Harness mapping"),
  and the table row for the reminder changed.
- **The extension holds no rule.** Each handler runs `/bin/sh scripts/tc-hook pi
  EVENT` with `spawn`, a JSON payload on standard input and a kill timer, and
  reads `{"additionalContext": TEXT}`. So the off file, the gate and the log are
  the same code as on the other harnesses. Only Node built-ins are imported.
  Every handler catches every error.
- **A new core event, `instruction`, returns the text and writes no start
  record.** Pi sends the instruction with every prompt (it clears the override
  after each run), and `session-start` writes a start record. The start record is
  written once, from `session_start`, with source `startup` (also for `new`),
  `resume` or `fork`. A `reload` writes none. Pi's compaction keeps the system
  prompt, so `since_start` is never reset on Pi.
- **The reminder is a saved hidden message from `before_agent_start`, not a
  `context` handler.** The spec said `context`, not stored. A fresh-context review
  (this leaf's one reviewer) found that a reminder at the end of each request
  moves the history's cache breakpoint on Anthropic and Bedrock requests, so the
  history would be written to the cache again on every request and never read.
  A saved message keeps the history the same, as on Claude Code and Codex. The
  cost is about 120 characters in the transcript for each prompt, which the size
  budget already allows. The spec now says so.
- **The reviewer found no defect** in the child-process handling, the event
  shapes or the return values (it read Pi's runner, loader and agent session).
  Its other points are visible trade-offs: `tc-hook` does not `exec` python, so a
  timeout kills only the shell and leaves python to finish (the log write may
  still land); the handlers ignore Pi's abort signal, so Esc waits for the child
  (tens of milliseconds normally, 15 seconds at most); and a broken script path
  fails quietly, which is the contract. I did not change any of them.
- **The load check.** `pi --mode rpc --no-session -ne -e FILE` with a stdin that
  stays open for 3 seconds fires `session_start`, and the extension then writes a
  start record. With stdin closed at once, Pi leaves before the child finishes,
  so the record would be missing. Pi exits 0 in RPC mode even for an extension
  that throws, so the load error on standard error is the only signal. The check
  has two controls, and I saw both fail: an extension that throws must print
  `Failed to load extension`, and the same extension with the off file present
  must write nothing. I also renamed the event in a copy of the extension and
  saw the check fail with "no start record". It skips with a visible message when
  `pi` is not installed.
- **`install.sh` links `harnesses/pi/*.ts` of each Pi-eligible skill** into
  `~/.pi/agent/extensions/` under the file's own name. Ownership is the skill
  links' test (the link's target shape), reconciliation removes an owned link
  when the file or the eligibility goes, and a real file at the path is refused
  with exit 1 and left alone. Nothing is created without a `~/.pi` directory.
  I saw the new cases fail with the call removed (5 of 28 fail).

## Evidence

Pi 0.84.2. Every live run used `-ne -e <extension>` (plus a scratch probe
extension that wrote what reached the provider to a file) and
`TECHNICAL_COMMUNICATION_STATE_DIR` in the scratchpad. I did not run `install.sh`
against the real home. The model was `openai-codex/gpt-6.1-sol`: the default
provider (a local oMLX server) was down, and the Kimi login has expired
(`token refresh unauthorized`), which is not this leaf's work. The work directory
was `~/.cache/tc-live-k13`, outside every temporary directory, so the gate did not
skip it. It is deleted.

| Layer | Result |
|---|---|
| Instruction | In the request's `instructions` on every prompt of a three-prompt session (`instruction: true`). With no tool call, the model wrote "optimise" and "customise" to a prompt that said "optimized" and "customized". |
| Reminder | Saved directly after each prompt's user message (index 1, then 1 and 6 after the third prompt). The history stays in place between requests. The model did the task in every run. |
| Document gate, `write` | `notes.md` with "utilize", "optimized", "organized" and a semicolon: the tool result held `Successfully wrote 50 bytes` plus a 4-finding gate message. The model kept the exact text the human gave, which is precedence 1. |
| Document gate, `edit` | An edit to `utilizes` gave a result with the 1-finding message. An earlier edit in the same turn, in British spelling, gave none. |
| Reply log | One `start` (`harness: pi`, `source: startup`) and one `reply` per prompt with `model: gpt-6.1-sol`, `seq` and `since_start` counting 1, 2. |
| Fallback: `bash` writes | A file written through `bash` got no gate message. With the skill loaded (`--skill`), the model ran `ste-lint.py` itself: `0 hard, 0 advisory, 42 words`. |
| Fallback: lazy conversion | An edit to an old unconverted `old.md`: the model ran the checker (3 hard), converted the whole file, and stopped to ask, because the directory is not a repository. |
| Switch | In one RPC session: prompt 1 with the layers on; the off file created; prompt 2 had no instruction, no reminder and no log record; the off file removed; prompt 3 had all of them again. No restart. |

**Layers that fall back to the model running the checker:** any file written
through `bash` or another extension's tool; a run that Pi starts with
`sendMessage` while idle (Pi 0.84.2 skips `before_agent_start`, so it also has no
instruction); a child that `pi-subagents` runs inside the parent without its
extensions. I did not run the last two.

**Not shown:** a `/new`, `/resume`, `/fork` or compaction turn (only the
`startup` source ran live); the TUI (a real terminal under `script` was not
needed, since the layers are the same in every mode); a provider other than
OpenAI's Responses API, so the cache argument for the saved reminder rests on
the reviewer's reading of the Anthropic and Bedrock converters.

## For `publication-k16`: what the human runs

```sh
# Pi: link the skill and the extension. Run it from the main checkout.
./plugins/install.sh
# Turn everything off, at the next event, in every running Pi:
mkdir -p ~/.local/state/technical-communication && touch ~/.local/state/technical-communication/off
# Turn it on again:
rm ~/.local/state/technical-communication/off
# Start Pi without the extension (and every other one) for one run:
pi -ne
```

Pi loads the extension with no trust prompt. A restart of Pi is needed once,
after the link first exists.

**`bash scripts/check.sh`: 19 of 20 pass.** The one failure is `Ravel caller assets
and provenance`: `policy typecheck: Bun 1.4.2 is pinned`, and this machine has
Bun 1.4.3. It is the same failure as in `codex-always-on-k12`, and it reads no
file I changed. The extension's final form (the saved reminder) was loaded again by
`scripts/pi-extension-load.test.sh` and checked live after that full run.

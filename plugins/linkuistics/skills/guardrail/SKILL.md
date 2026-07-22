---
name: guardrail
description: Session-scoped safety guardrails — a user-invoked PreToolUse permission gate that pauses for confirmation before destructive shell commands (rm -rf, force-push, git reset --hard, DROP/TRUNCATE, mkfs, dd to a device) and before edits or writes outside the project boundary (a "freeze"). Use when running agentic or semi-autonomous edits, working in an unfamiliar or production-adjacent repo, or any session where you want a deliberate confirm-before-damage net; user-invoked — turn it on by hand with /guardrail. Claude Code only — the frontmatter hook this skill installs is a no-op on other harnesses.
disable-model-invocation: true
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "bash"
          args: ["${CLAUDE_PLUGIN_ROOT}/skills/guardrail/scripts/guardrail-hook.sh"]
    - matcher: "Edit|Write|MultiEdit|NotebookEdit"
      hooks:
        - type: command
          command: "bash"
          args: ["${CLAUDE_PLUGIN_ROOT}/skills/guardrail/scripts/guardrail-hook.sh"]
---

# Guardrail — a confirm-before-damage net you turn on

> **Claude Code only.** The mechanism is a `SKILL.md`-frontmatter `PreToolUse`
> hook, `${CLAUDE_PLUGIN_ROOT}`/`CLAUDE_PROJECT_DIR`, and a `/guardrail`
> slash-command invocation — all Claude Code specific. On codex, pi, or any
> other harness the frontmatter `hooks:` block is inert: nothing installs,
> `/guardrail` fires nothing, and no gate appears. There is no equivalent
> here yet for other harnesses' own pre-tool confirmation mechanisms, if any.

This skill installs a **session-scoped `PreToolUse` permission gate**. While it is
active, Claude Code runs `scripts/guardrail-hook.sh` *before* every shell command
and every file edit, and the hook **pauses for your explicit confirmation** when
it sees something irreversible. You stay in control — confirm to proceed, decline
to stop.

It is a **guardrail, not a jail**: the hook only ever returns `ask` (a
confirmation prompt), **never `deny`**. The point is a deliberate speed-bump in
front of damage, not a wall — you keep the override.

> Adapted from `garrytan/gstack`'s `careful` / `freeze` / `guard` skill class
> (survey finding `gstack-S5`, quoted in `docs/research/skill-repo-prior-art.md`).
> gstack ships three composable slash commands; we ship **one** composed skill —
> a hand-invoked skill pays zero standing context cost, so the only thing the
> three-way split would buy us is conceptual surface, not savings. The mechanism
> that transfers is *a `SKILL.md` can carry its own `PreToolUse` hook*; none of
> our other skills do.

## Why it is user-invoked

`disable-model-invocation: true` makes this **hand-only** — you turn it on
deliberately with `/guardrail`, and it is never auto-fired and never sits in the
model's context. A safety gate you didn't ask for is the wrong default; a safety
gate is something you *opt into* for a risky stretch of work. (See the house
`authoring-conventions` skill on the user-invoked lever.)

## The two halves

### 1. Careful — destructive shell commands

When the gate is on, a `Bash` command matching any entry in the `DANGER` table at
the top of `scripts/guardrail-hook.sh` triggers a confirmation. The shipped list
catches genuinely irreversible / data-losing operations:

| Trigger | Why it pauses |
|---|---|
| `rm -rf` / `rm --recursive --force` | recursive forced delete |
| `git push --force` / `-f` / `--force-with-lease` | rewrites remote history |
| `git reset --hard` | discards working-tree changes |
| `git clean -f…` | deletes untracked files |
| `jj abandon` | discards revisions |
| `jj op restore` / `jj operation restore` | rewinds the repo to an earlier operation |
| `DROP` / `TRUNCATE TABLE\|DATABASE\|SCHEMA` | destructive SQL |
| `mkfs…` | formats a filesystem |
| `dd … of=/dev/…`, `> /dev/sd…` | overwrites a raw device |

The list is **deliberately tight** — an over-eager gate trains you to rubber-stamp
it (alarm fatigue), which is worse than no gate. It is also **yours to tune**:
each `DANGER` entry is `ERE-regex<TAB>reason`; add or remove patterns to fit your
workflow. It catches *data-loss* operations, not merely disruptive ones — by
design it does **not** warn on `git push` (plain), `rm` of a single file, or
system power commands.

### 2. Freeze — edits outside the boundary

An `Edit` / `Write` / `MultiEdit` / `NotebookEdit` whose resolved path lands
**outside the boundary directory** triggers a confirmation — the high-value piece
for sandboxed / agentic editing, where the failure mode is an agent wandering off
to touch `~/.ssh/config` or a sibling repo. Path resolution is **lexical** (it
resolves `..` without trusting the filesystem), so a `../../escape` is caught.

The boundary is resolved in this precedence:

1. `CLAUDE_FREEZE_DIR` — set this to narrow the boundary to a sub-directory
   (`export CLAUDE_FREEZE_DIR="$PWD/src"`).
2. `CLAUDE_PROJECT_DIR` — the repo root Claude Code sets; the default boundary.
3. the session's working directory — the fallback when neither is set.

So with **no configuration**, freeze keeps edits inside the project; one env var
tightens it further.

## Turning it on (and off)

- **On:** type `/guardrail`. The hook activates for the skill's lifetime in this
  session.
- **Narrow the freeze boundary:** `export CLAUDE_FREEZE_DIR=<dir>` before or
  during the session.
- **Off:** the gate is scoped to the skill's lifecycle and is cleaned up when the
  skill is no longer active — it does not persist into a new session, and nothing
  is written to your settings. If you notice it has stopped gating mid-session
  (skill influence can lapse after Claude takes several other actions), **re-invoke
  `/guardrail`** to restore it.

> **Per-harness note (confirm once):** the exact span of a skill-scoped hook's
> "lifecycle" is defined by your Claude Code version. The firing mechanism and the
> `ask` contract are verified against the current hooks docs; if you depend on the
> gate staying live across many turns, confirm the span in your version and
> re-invoke as needed. Marked here rather than asserted because it is the one
> behaviour that is version-dependent (`UNVERIFIED` across versions).

## How it relates to permissions and other hooks

- This does **not** replace your `permissions` settings or `settings.json` hooks —
  it is an *opt-in, session-scoped* layer on top, carried by the skill itself so
  there is nothing to install or clean up.
- It composes with `doubt-driven-development` (verify a *decision*) and
  `/code-review` (a post-hoc gate): guardrail is the cheap mechanical net on
  *individual destructive actions*; those are judgement on *correctness*.

## The hook contract (for maintainers)

`scripts/guardrail-hook.sh` reads the `PreToolUse` event JSON on stdin
(`.tool_name`, `.tool_input.command` for Bash, `.tool_input.file_path` for edits,
`.cwd`) and either:

- emits `{ "hookSpecificOutput": { "hookEventName": "PreToolUse",
  "permissionDecision": "ask", "permissionDecisionReason": "…" } }` on stdout
  (exit 0) — Claude pauses for confirmation; or
- emits nothing (exit 0) — the normal permission flow proceeds ("defer").

It never emits `deny`. Field names follow the current Claude Code hooks contract.
`scripts/guardrail-hook.test.sh` is a dependency-free test runner (no `bats`
required) covering both halves, including the false-alarm cases; run it after any
change to the `DANGER` table or the boundary logic.

**Why the frontmatter uses exec form + `${CLAUDE_PLUGIN_ROOT}` (don't revert to
`./scripts/…`):** a skill-frontmatter hook `command` runs from the *session's*
working directory, not the skill's, so a relative `./scripts/…` path fails with
`No such file or directory` once the plugin is installed. The plugin-root
placeholder resolves to an absolute path, and **exec form** (`args:`) is the
documented shape for path placeholders — it passes the path as one argument with
no shell quoting. (Verified the hard way: the relative form fired the hook but
could not find the script.)

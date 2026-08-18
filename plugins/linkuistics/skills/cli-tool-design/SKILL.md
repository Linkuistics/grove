---
name: cli-tool-design
description: Guidelines for designing LLM-friendly command-line tools — structured output, help text with examples, actionable errors, exit codes, consistent flags. Use when designing, writing, auditing, or refactoring a CLI tool.
---

# CLI Design Guidelines for LLM Agents

## Core principle

LLM-friendliness is not a feature you bolt on with a special command. It is a property of the entire CLI surface — help text, flag conventions, output formats, error messages, exit codes, defaults. An agent's experience of your tool is dominated by what it sees the first time it runs `--help` and what it sees when something goes wrong. Optimise those, in that order, before anything else.

A separate `llm-instructions` command can supplement this, but it cannot substitute for it. Many agents will never call it. Treat it as the manual; treat `--help` as the reference.

## Applicability

These are defaults for a **multi-command tool an agent will drive**, not laws. Three things narrow them, and it is worth naming which apply before auditing a tool against the list:

- **Shape.** A single-purpose command, a filter that reads stdin and writes stdout, or a script with one flag inherits the error, exit-code and non-interactive guidance and little else. The checklist's structural items assume a subcommand tree.
- **Audience.** A tool no agent will ever invoke keeps the human-readable guidance and drops the parsing guarantees.
- **An existing convention outranks this file.** A tool that follows POSIX, git's verb layout, `kubectl`'s noun layout, or its own established surface should stay consistent with that. Consistency inside a tool beats conformity to any external checklist, and a breaking change to an established surface needs a reason bigger than this list.

Where a guideline below does not apply, say which of the three excused it rather than silently skipping it.

## Discovery loop assumptions

Assume the agent's loop looks like this:

1. Run `tool --help` to see top-level subcommands.
2. Pick a likely subcommand, run `tool <sub> --help`.
3. Possibly recurse one more level.
4. Execute, parse output, branch on exit code.
5. On failure, read stderr and try once more.

Every friction point in this loop costs tokens, latency, or correctness. Design accordingly.

---

## The high-impact checklist

Ordered by ROI. If you only fix a few things, fix them in this order.

### 1. Structured output on data-producing commands

A command whose output an agent will parse needs a structured mode — `--json`, or `--output json` — with a stable schema, documented in help text. Human-readable output is for humans and is not a parsing target.

Applies to commands that emit *data*. A command whose entire output is a status line, a diff, or prose for a human does not need one, and neither does a tool no agent will drive.

- Keep the schema stable across patch versions; breaking changes go in major versions and are documented.
- Prefer one object per logical record over nested prose-like structures.
- Include enough metadata per record to avoid a second call (IDs, timestamps, status).
- For streaming or long outputs, prefer JSON Lines (`--output jsonl`) over one giant array.
- Errors in structured mode should also be structured. Don't mix prose stderr with JSON stdout silently.

```
# Good
$ tool list users --json
{"id":"u_1","name":"Ada","status":"active","created":"2025-01-01T00:00:00Z"}
{"id":"u_2","name":"Ben","status":"locked","created":"2025-01-02T00:00:00Z"}

# Bad
$ tool list users
👤 Ada    (active, joined Jan 1)
👤 Ben    (locked, joined Jan 2)
```

### 2. Help text with examples

LLMs pattern-match on examples far more reliably than on flag listings, so a help page without them produces guesswork. Two or three concrete invocations at the bottom of each `--help` is the target for any command an agent is expected to invoke directly; a rarely used internal subcommand can lean on its parent's examples instead.

Help text structure, top to bottom:

1. One-line summary.
2. One-paragraph description (when the tool does, when it doesn't, key caveats).
3. Usage synopsis.
4. Arguments and flags, grouped logically.
5. Exit codes, if non-trivial.
6. Examples — covering the most common real uses.
7. See-also references to related subcommands.

### 3. Actionable error messages

Every error message with a known remediation should name it. Errors that have none should say so, so the agent stops retrying. This one holds for any tool with any consumer.

```
# Bad
Error: permission denied

# Good
Error: permission denied — no valid credentials found.
Run `tool auth login` to authenticate, or set TOOL_API_KEY in the environment.
```

For machine consumption, add an error code or category. In structured mode, errors should carry a stable `code` field (`"AUTH_REQUIRED"`, `"NOT_FOUND"`, `"RATE_LIMITED"`) that agents can branch on without parsing the message.

### 4. Consistent flag vocabulary

Pick one name per concept and use it everywhere. Inconsistency here produces a long tail of agent failures that are hard to debug — and it is the one item with no exceptions, because it is internal to the tool.

A reasonable baseline, where the tool needs the concept at all:

- `--json` / `--output <fmt>` for structured output.
- `--quiet` / `-q` for suppressing non-essential stderr.
- `--verbose` / `-v` for diagnostic detail (stderr, never stdout).
- `--dry-run` for preview without side effects.
- `--yes` / `-y` for non-interactive confirmation.
- `--force` for overriding safety checks (distinct from `--yes`).
- `--filter <expr>` or `--<field> <value>` for narrowing list output.
- `--limit <n>` for pagination size.
- `--all` for explicitly opting into unbounded results.
- `--format <tmpl>` for custom output templating.

An established tool with its own spellings keeps them; the rule is one spelling per concept, not these spellings. If `delete` uses `--yes`, `purge` should not use `--confirm`.

### 5. Consistent verb/noun ordering

For a tool with a subcommand tree, pick one and apply it throughout:

- **Noun-first** (`tool user create`) — like `kubectl`. Easier to discover related operations on one resource.
- **Verb-first** (`tool create user`) — like `git`. Easier when verbs are universal across resource types.

Either works. Mixing them is what causes problems. Aliases for common synonyms (`rm`/`remove`/`delete`, `ls`/`list`) are cheap and meaningfully reduce guess-the-verb failures.

### 6. Default output limits on unbounded lists

A `list` command that returns 50,000 rows by default can blow out an agent's context in one call. Where a command's result set is unbounded, default to a page (50 or 100) with `--limit N` and `--all` overrides. A command with a naturally small, bounded result needs none of this.

When truncating, say so explicitly in both human and structured output:

```
$ tool list events
... 100 rows ...
Showing 100 of 12,438 results. Use --limit N or --all to see more.

$ tool list events --json
{"items":[...], "total":12438, "returned":100, "truncated":true}
```

### 7. Non-interactive when stdout isn't a TTY

Any tool that might be run from a script or an agent should detect TTY and adapt: no pagers when piped or captured, no spinners or carriage-return animations, no colour codes unless `--color always` or a TTY is detected, and no interactive confirmation without a `--yes` alternative.

Destructive operations especially need a non-interactive escape hatch. An agent cannot answer a `[y/N]` prompt.

### 8. Meaningful, documented exit codes

`0` for success, non-zero for failure is the floor and applies to everything. Distinguishing categories pays off once a caller might branch on the reason; one scheme that works:

- `0` success · `1` generic failure · `2` usage error · `3` not found · `4` auth required / forbidden · `5` conflict / precondition failed · `6` rate limited / try again later

The specific numbers matter less than picking a scheme, documenting it, and sticking to it. A tool whose ecosystem already assigns meanings to exit codes (`grep`'s `1` for no match, `diff`'s `1` for differences) keeps those.

### 9. Idempotency and side-effect clarity

For every mutating command, document whether it is idempotent, whether it is safe to retry on transient failure, and whether it has partial-failure semantics. If a command is *not* safe to retry, say so prominently in its help text — agents retry by default.

Idempotency keys (`--idempotency-key <uuid>`) are worth it on operations with expensive or externally visible side effects, and overkill elsewhere.

### 10. Stable, parseable identifiers

Whatever IDs the tool returns should be unambiguous (prefix-typed like `usr_abc123` beats bare integers), stable across calls, and usable as input to other commands without transformation. Round-tripping IDs is a core agent pattern: if `tool list users --json` returns `id` fields, `tool show user <id>` must accept those exact values.

Where the IDs are not yours — a filesystem path, a git SHA, an upstream API's identifier — pass them through unchanged rather than inventing a parallel scheme.

---

## Optional but valuable

### `tool capabilities` or `tool version --json`

A machine-readable summary of what the tool can do — version, supported subcommands, output formats, available features. Lets agents check feature availability without parsing help text. Cheap to implement, most useful for a tool with a wide surface or a fast-moving feature set.

```json
{
  "version": "2.4.1",
  "subcommands": ["user", "project", "deploy"],
  "output_formats": ["text", "json", "jsonl"],
  "features": { "idempotency_keys": true, "streaming": true }
}
```

### `tool schema <command>`

Emits the JSON schema for a command's structured output, letting agents validate parsing and fail loudly when assumptions break. Earns its place where many subcommands have distinct output shapes.

### `tool llm-instructions`

A single command printing a focused supplementary manual for what does not fit in `--help`: what the tool is and isn't, the mental model of the command tree, two or three full workflow recipes, common mistakes, authentication and state assumptions, and pointers to the structured-output, exit-code and idempotency conventions.

Keep it under a few thousand tokens. If it grows past that, add `--topic <name>` and `--section <name>` flags rather than subcommands — subcommands here add a round-trip and complicate discovery for no gain.

---

## Auditing or refactoring an existing CLI

When auditing an existing tool, walking a structured checklist, flagging
anti-patterns, or planning a backwards-compatible refactor, load the
companion reference: [references/auditing-and-refactoring.md](references/auditing-and-refactoring.md).
It contains the full audit checklist, the anti-pattern catalogue, and the
ordered refactoring sequence.

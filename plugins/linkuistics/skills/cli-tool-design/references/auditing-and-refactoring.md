# Auditing and Refactoring an Existing CLI

Companion reference to the `cli-tool-design` skill. Load this when auditing an
existing CLI, flagging anti-patterns, or planning a refactor.

## Audit checklist

Scope the list before walking it. `cli-tool-design`'s *Applicability* section
names what excuses a line — the tool's shape, its audience, and an established
convention it already follows — so settle those first and strike the lines they
excuse. A finding against a guideline the tool was never bound by is noise, and
it teaches the reader to discount the rest.

Each remaining "no" is a finding.

**Output**

- [ ] Every command whose output a caller parses supports `--json` (or equivalent).
- [ ] JSON schema is documented and stable.
- [ ] Errors in JSON mode are also JSON, with a stable `code` field.
- [ ] List commands with unbounded results have a default limit and indicate truncation.
- [ ] No ANSI colour or progress animation in non-TTY output.

**Help**

- [ ] Top-level `--help` lists all subcommands with one-line descriptions.
- [ ] Every subcommand an agent invokes directly has concrete examples in `--help`.
- [ ] Help text states which output formats are stable and which are not.
- [ ] Exit codes are documented if non-trivial.

**Errors**

- [ ] Common errors name their remediation.
- [ ] Errors distinguish "your fault" from "system fault" via exit code or code field.
- [ ] No silent failures (commands either succeed or exit non-zero).

**Conventions**

- [ ] Flag vocabulary is consistent across all subcommands.
- [ ] Verb/noun ordering is consistent across all subcommands.
- [ ] Synonyms have aliases (rm/remove/delete, ls/list).
- [ ] Identifiers are prefix-typed and round-trippable.

**Behaviour**

- [ ] Destructive commands support `--yes` or `--force` for non-interactive use.
- [ ] Mutating commands document idempotency and retry safety.
- [ ] No interactive prompts trigger when stdout is non-TTY without an escape flag.
- [ ] `--dry-run` is available on commands with significant side effects.

**Discoverability (optional but valuable)**

- [ ] `tool version --json` or `tool capabilities` exists.
- [ ] `tool schema <cmd>` exists for commands with complex output.
- [ ] `tool llm-instructions` exists if there's meaningful workflow guidance that doesn't fit in `--help`.

---

## Anti-patterns to flag

- **Pretty output as the only output.** If parsing requires regex on prose, the command is broken for agents.
- **Unstable JSON.** Adding fields is fine; renaming or removing them between minor versions is not.
- **Help text without examples.** Flag listings alone produce guesswork.
- **Vague errors.** "Something went wrong" with no code, no remediation, no context.
- **Mixed flag vocabulary.** `--yes` here, `--confirm` there, `--noprompt` somewhere else.
- **Required interactive prompts.** Anything that blocks on stdin without a flag override.
- **Unbounded default lists.** `tool list everything` returning the entire dataset.
- **Bare-integer IDs.** Forces agents to track types externally.
- **Hidden state.** Commands whose behaviour depends on a config file or environment variable that isn't surfaced in help or errors.
- **Inconsistent exit codes.** Returning `1` for everything, or `0` on partial failures.
- **Opaque retries.** Mutating commands with no documented idempotency story.

---

## When refactoring an existing tool

Order of operations:

1. Add `--json` everywhere it's missing. Don't change defaults.
2. Stabilise and document the JSON schema.
3. Audit and rewrite error messages — actionable remediation, error codes.
4. Add examples to every `--help`.
5. Normalise flag vocabulary. Introduce new flag names; keep old ones as aliases for backwards compatibility.
6. Add exit code documentation; tighten exit code usage.
7. Add `--yes` / `--dry-run` / non-TTY behaviour.
8. Optionally: add `capabilities`, `schema`, `llm-instructions`.

Steps 1–4 are nearly always backwards-compatible and deliver most of the value. Steps 5–7 may require a major version bump. Step 8 is pure addition.

# author-guardrail-hooks-k17

**Kind:** work

## Goal

Author a **guardrail** skill (or small set) that installs a *session-scoped `PreToolUse`
permission hook* — a `careful`/`freeze`/`guard`-style safety skill. A composable skill
*class* none of our 9 currently use. (Synthesis skills disposition **AUTHOR #4**, source
gstack-S5.)

## Context

- The capability (model to adapt, NOT fork): `garrytan/gstack` →
  `careful`/`freeze`/`guard` `SKILL.md.tmpl` (quoted in survey §gstack S5). A `SKILL.md`
  declares `hooks: PreToolUse[matcher: Bash]` (or `Edit`/`Write`) → a script that returns
  `permissionDecision: "ask"` with a warning on destructive commands (`rm -rf`,
  `DROP TABLE`, force-push…) or on edits **outside a chosen directory** (`freeze`), letting
  the user override. `guard` composes both.
- The `freeze` directory-boundary is the highest-value piece for **sandboxed/agentic
  editing**. Decide whether to ship one composed `guard` skill or `careful`+`freeze`
  separately — likely user-invoked (you turn it on deliberately).

## Done when

- A guardrail `SKILL.md` (+ its hook script) exists in `plugins/linkuistics/skills/`,
  spec-conformant, following k14's house conventions, registered.
- The hook is verified to actually fire (a `PreToolUse` matcher that returns the `ask`
  decision) — not just declared.

## Notes

- **Verify the hook mechanism is current before relying on memory:** confirm Claude Code's
  `PreToolUse` hook contract (`permissionDecision: "ask"`, matcher syntax) via the
  `claude-code-guide` agent or `/update-config` skill — gstack's `.tmpl` is a model, the
  exact field names must match the live harness.
- Smaller win than k14–k16; if it proves bigger (multiple hook types, a config surface),
  `leaf-decompose` it.

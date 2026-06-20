# 070-global-skill-homebrew-distribution

**Kind:** work

## Goal

Make `brew install grove` the **sole install gesture**: one binary (loop driver +
tree verbs + signal verb) that **provisions the global skill** into
`~/.claude/skills/grove/`, bundling grove's **full retained methodology**. This
collapses distribution and dissolves the `VERSION.md` drift model (ADR-0031,
030 D2/D6).

## Context

Read **ADR-0031** (distribution collapse), **030 D2** (Homebrew-sole) and **D6**
(retain the full methodology — bundle it, do not gut it), and the spike's
distribution section (`docs/research/loop-substrate-options.md`) for the global-
skill mechanics (personal `~/.claude/skills/` is read live; plugin `bin/` is
Bash-tool-PATH only; a system CLI ships via Homebrew). The skill content is
canonical in `content/` (mirrored today into `.claude/skills/grove/`).

## Done when

- A Homebrew formula installs the `grove` binary on PATH as the single gesture.
- The binary **provisions the global skill** to `~/.claude/skills/grove/`,
  bundling the full methodology (grilling, driving, format guides, the loop
  discipline) — recommended mechanism: the binary **embeds the skill content and
  idempotently extracts it on launch**, so skill and binary always match (no
  drift). Settle formula-post-install vs binary-on-launch extract here.
- The old per-worktree materialise + `VERSION.md` three-way drift model is gone
  (its deletion is 090; this leaf establishes the replacement so 090 can remove
  the old path safely).
- Any generic-skill dependencies (e.g. a file-watcher, if the 040 kill uses one)
  are declared by the formula.

## Notes

- Global skill precedence (spike): enterprise > personal > project; a project
  `.claude/skills/grove/` would override the global one — document this.
- Residual drift to note (not a Claude Code limitation): skill vs binary version
  if ever released separately — here they ship together in one formula, so it does
  not arise.
- Coordinate naming with the human/agent CLI split (ADR-0006): `grove` (loop
  driver, human) vs `grove-llm` (verbs, agent) may stay two binaries in one
  formula, or collapse — decide here.

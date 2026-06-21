# 070-global-skill-homebrew-distribution

**Kind:** work

## Goal

Make `brew install grove` the **sole install gesture** — one binary (loop driver +
tree verbs + signal verb) that **provisions the global skill** into
`~/.claude/skills/grove/`, bundling grove's **full retained methodology** — AND
**perform the flip** (ADR-0034): land new-scheme prose, remove the project-local
skill mirrors, install the new binary, and migrate this grove + any other in-flight
old grove via adoption. This collapses distribution and dissolves the `VERSION.md`
drift model (ADR-0031, 030 D2/D6).

## Context

Read **ADR-0034** (the flip — this leaf is where the world goes new-format),
**ADR-0031** (distribution collapse), **030 D2** (Homebrew-sole) and **D6** (retain
the full methodology — bundle it, do not gut it), and the spike's distribution
section (`docs/research/loop-substrate-options.md`) for the global-skill mechanics
(personal `~/.claude/skills/` is read live; plugin `bin/` is Bash-tool-PATH only; a
system CLI ships via Homebrew). Skill content is canonical in `content/` (today
also mirrored into `.claude/skills/grove/`, which this leaf **removes**).

**Critical ordering (ADR-0034):** there is currently **no global skill** — removing
the project-local mirrors before provisioning the global one leaves every grove
skill-less. Provision first, then remove mirrors.

## Done when

### Distribution
- A Homebrew formula installs the `grove` binary on PATH as the single gesture.
- The binary **provisions the global skill** to `~/.claude/skills/grove/`,
  bundling the full methodology — recommended mechanism: the binary **embeds the
  skill content and idempotently extracts it on launch**, so skill and binary
  always match (no drift). Settle formula-post-install vs binary-on-launch extract.
- Any generic-skill dependencies (e.g. a file-watcher, if the 040 kill uses one)
  are declared by the formula.

### The flip (ADR-0034)
- **New-scheme prose** lands in `content/`: SKILL.md, `prompts/*`, and the format
  guides describe the new dotted-decimal scheme + the migrate-on-adoption flip (the
  prose the 050 D9 / old rollout deferred). Because the global skill is binary-
  embedded, prose tracks the binary, so inbox/TUI prose can finish cleaning as
  080/090 land.
- **Project-local skill mirrors removed**: `.claude/skills/grove/` deleted from
  this worktree, the main checkout, and the `grove-general-improvements` worktree
  (each a reviewable change on its branch). Document the precedence note
  (enterprise > personal > project) so a stray project copy never silently shadows
  the global one.
- **Install + handoff**: build & install the new binary; the next `grove do
  <this-grove>` migrates this tree by adoption (030) and continues new-format;
  `grove-general-improvements` flips the same way on its next `grove do`.

## Notes

- Coordinate the human/agent CLI split (ADR-0006): `grove` (loop driver, human) vs
  `grove-llm` (verbs, agent) may stay two binaries in one formula, or collapse —
  decide here.
- The old per-worktree materialise + `VERSION.md` drift model is *replaced* here;
  its actual deletion is 090 (this leaf establishes the replacement so 090 can
  remove the old path safely).
- This is a large leaf spanning provisioning + prose + a multi-worktree mirror
  removal + the live flip — **may decompose** when picked.
- **Confirm with the user before the live flip** (install + first real
  adoption-migrate of this grove): it changes the running binary and migrates a
  live tree. Build + stage everything first; flip as a deliberate, reviewed step.

# refactor-grove-to-be-an-archon-workflow — brief

## Goal

Refactor grove down to its **irreducible self-extension core** (the
self-extending task tree + the loop that walks it) and drive it on a **workflow
substrate** that automates the per-task fresh-context loop — instead of grove's
bespoke `grove do` launcher, rmux TUI, and per-worktree install. Candidate
substrate: **Archon** (https://archon.diy), the open-source workflow-engine /
harness-builder; under active reconsideration against self-driven alternatives
(see D8). The guiding directive throughout: **less in grove.**

## Done when

- The loop substrate is chosen on evidence and the loop runs one grove task per
  fresh context until the tree is empty, hosting both work and grilling tasks,
  restart-safe.
- grove is shed to its core: TUI deleted; methodology → third-party skills;
  worktree/fresh-context/approval → the substrate; inbox/grove-meta + install
  machinery removed; distribution → a single global skill + `grove-llm`.
- The task-id scheme is migrated to flat dotted-decimal (D4/D5).

## Decomposition

Live leaves (current `NNN-slug` scheme — the dotted scheme is what we're
*building*):

- `020-loop-substrate-spike` (research) — cited options doc deciding nothing;
  gates everything below. → `docs/research/loop-substrate-options.md`.
- `030-substrate-decision` (planning, placeholder) — choose the substrate from
  020's evidence, then grow the implementation leaves.

Deferred (grown by `030` or later, lazily — not yet ripe): dotted-decimal
numbering + verb changes; global-skill + `grove-llm` distribution with
backwards-compat; shed-the-TUI; shed inbox/grove-meta + install machinery;
substrate wiring (the workflow / loop-driver); migration.

## Pointers

- Archon: https://archon.diy · https://github.com/coleam00/Archon (README,
  docs, `.archon/workflows/` examples, issues). Today's identity: "the first
  open-source harness builder for AI coding."
- grove's process-machinery history (evidence for "which complexity to own"):
  ADR-0028 (rmux substrate / trellis deletion) and the rmux glossary section in
  `CONTEXT.md`.
- Full grilling rationale: the retired `010-plan` running log (D1–D8) in
  `.grove/done/`.

## Notes

### Settled decisions (condensed — full rationale in retired `010-plan`)

- **D1** Archon = archon.diy, the workflow-engine/harness-builder (not the older
  MCP knowledge-base framing).
- **D2** End-state (A): replace grove's runtime, keep its self-extension *brain*,
  shed aggressively, **kill the TUI**.
- **D3** Core boundary — *survives:* task tree + `pick` walk + grow verbs + the
  two task kinds + minimal loop; *sheds to skills:* grilling, driving habits,
  CONTEXT/ADR/PRD format guides, TDD/review/debugging; *sheds to substrate:*
  worktree lifecycle, fresh-context looping, approval, multi-surface; *deleted:*
  rmux/ratatui TUI + Fleet, **inbox/grove-meta**, **install/materialise +
  VERSION drift**.
- **D4/D5** Task ids → **flat dotted-decimal**, legible sequential integers, a
  numeric version-sort comparator (infinite width + DFS order), renumber-on-
  reorder accepted, **mark-done-in-place** (no `done/` directory).
- **D6/D7** Execution = a **continuous loop**, fresh context per task,
  **resume-safe by construction** (the loop body is stateless + self-locating,
  so Archon's run-durability is made irrelevant); target shape (iii), degrades
  to repeated runs, upgrades to one long run.
- **D8** Substrate **reopened**: Archon is one *measured* candidate vs
  self-driven (iTerm-trigger / PTY-wrap / headless / harness-native), decided by
  the `020` spike on three gates (fresh-context-per-task; interactive-grilling-
  in-loop; restart-safety). Distribution → **global skill + `grove-llm`** with
  backwards-compat for old trees. Named hypothesis to test: Archon's conditional
  logic may express restart + loop-until-empty in one declarative `pick`.

### ADRs

None yet — the design is still firming. D2/D3 and the substrate choice are the
likely first ADRs, written in `030` once the substrate is settled.

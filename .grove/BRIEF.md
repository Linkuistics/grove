# refactor-grove-to-be-an-archon-workflow — brief

## Goal

Refactor grove down to its **irreducible self-extension core** (the self-extending
task tree + the loop that walks it) plus its **proven methodology**, and drive it
on a **self-driving loop** that automates the per-task fresh-context crank —
shedding grove's *machinery* (TUI, inbox/grove-meta, install/materialise). The
guiding directive: **less in grove** — meaning less *machinery*, not less *wisdom*
(030 D6).

**Substrate decided (was the open fork; now settled):** a thin, stateless,
grove-owned **self-driving shell loop**, **not** Archon — chosen on the 020 spike's
evidence (**ADR-0032**). The grove's name now misdescribes the outcome; kept as a
historical label (the spike reversing its own premise is the spike working).

## Done when

- The loop runs one grove task per fresh context until the tree is empty, hosting
  both work and grilling tasks, restart-safe. [substrate decided; built in 040]
- grove is shed to its core + methodology: TUI deleted (080); inbox/grove-meta +
  install machinery removed (090); distribution → a single global skill +
  `brew install grove` (070). The methodology is **retained**, not shed.
- The task-id scheme is migrated to flat dotted-decimal (050) with a transitional
  bridge + one-time migration (060).

## Decomposition

Retired (in `done/`): `010-plan` (foundations D1–D8); `020-loop-substrate-spike`
(the cited options doc); `030-substrate-decision` (the substrate choice + this
leaf set).

Live leaves (current `NNN-slug` scheme — the dotted scheme is what 050 *builds*):

- `040-substrate-wiring` (work) — **critical path**: the loop driver + signal verb
  + kill + interrupt semantics + PoC (ADR-0032).
- `050-dotted-decimal-numbering` (planning) — the flat scheme + comparator + verbs.
- `060-backwards-compat-migration` (work) — dual-format reader + `grove migrate`.
- `070-global-skill-homebrew-distribution` (work) — `brew install grove` + global
  skill provisioning.
- `080-shed-tui` (work) — delete the rmux/ratatui TUI + Fleet.
- `090-shed-inbox-and-install-machinery` (work) — delete inbox/grove-meta +
  install/materialise.

Sequencing: 040 first (prove the loop); 050→060 (numbering before migration); 070
(distribution); 080/090 sheds last (don't delete the old runtime before the new
one works).

## Pointers

- Substrate evidence: `docs/research/loop-substrate-options.md` (020 spike).
- Decisions: **ADR-0031** (shed machinery, keep core + methodology) and
  **ADR-0032** (self-driving shell loop, not Archon). Full rationale: the retired
  `010-plan` (D1–D8) and `030-substrate-decision` (D1–D6) running logs in
  `.grove/done/`.
- grove's process-machinery history ("which complexity to own"): ADR-0028 (rmux
  substrate / trellis deletion) and the rmux glossary section in `CONTEXT.md`.

## Notes

### Settled decisions (condensed — full rationale in the retired running logs)

From `010-plan` (foundations): **D1** Archon = the workflow-engine. **D2/D3**
end-state = replace the runtime, keep the self-extension brain, shed aggressively.
**D4/D5** task ids → flat dotted-decimal, version-sort comparator,
mark-done-in-place. **D6/D7** execution = a continuous fresh-context loop,
resume-safe by construction, the engine (not the human) turns the crank. **D8**
substrate reopened → the 020 spike.

From `030-substrate-decision` (the substrate, decided on the spike's evidence):
**substrate = self-driving shell loop, NOT Archon** (ADR-0032; Archon's
`interactive` fails gate D, the restart hypothesis is refuted, DB walk-away cost,
a ~10-week-old rewrite). **Native foreground `claude`**; an out-of-band `grove-llm`
signal triggers an external kill (lean: self-spawned delayed killer); `pick` is
the loop condition; **relaunch is opt-in** so interrupts stay stopped; restart ≡
continuation. **Distribution = `brew install grove` sole gesture**, one binary that
provisions the global skill (dissolves `VERSION.md` drift). **Backwards-compat =
transitional dual-format + one-time `grove migrate`, then drop** (ephemeral trees
drain). **"Less in grove" = less machinery, not less wisdom — the methodology is
RETAINED** (ADR-0031, D6).

### ADRs

- **ADR-0031** — grove sheds its machinery to a self-extension core that keeps its
  methodology.
- **ADR-0032** — the loop substrate is a self-driving shell loop, not an Archon
  workflow.

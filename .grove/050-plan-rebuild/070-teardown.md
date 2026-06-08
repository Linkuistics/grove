# 070-teardown

**Kind:** work

## Goal

Tear down the trellis-era scaffolding now that the rmux substrate is in place:
sweep the ADR-0013–0028 tower per D4, retire the `bugs` grove, and clean the
`CONTEXT.md` TUI section down to its rmux-substrate reality. Last leaf — runs once
010/050/060 have produced their "what survives" verdicts.

## Context

ADR-0028 recorded the supersession at the **thesis level** but deliberately left
the **per-ADR `Superseded` marking** to this leaf (D4). The verdicts that decide
*which* ADRs are fully superseded vs "mechanism gone, UX survives" come from the
earlier leaves: 010-surfaces (ADR-0019 A′ — and its build leaves 020-leader-dispatch
/ 030-detail-widget / 040-detail-triage), 050-working-set (ADR-0022/0023),
060-daemon-launch (ADR-0027 under rmux). Pull those verdicts in before marking.

## Tasks

**1. ADR sweep (D4).** Add a `Superseded by ADR-0028` (or `-0029`) line to each:
- *Fully superseded* (mechanism + premise gone): 0014, 0015, 0016, 0017, 0018,
  0020, 0021, 0023, 0024, 0026.
- *Mechanism superseded, UX intent survives* — annotate, don't blank: 0019
  (per-grove detail / nav-opens), 0022 (constant nav + swapped content). Point at
  the 010/050 verdicts.
- *Survives / amended, do NOT mark superseded*: 0013 (presentation boundary —
  amended by 0028 E1/E2), 0025 (fleet discovery — below the seam), 0027 (no-cwd
  anchor — possibly amended by 030, not superseded).
- Confirm 0028 (landmark) + 0029 (capture-pane) are `accepted` and cross-linked.

**2. Retire the `bugs` grove.** Its backlog is trellis-specific and mostly
evaporates; its branch carries a committed-but-broken trellis floating-pane change
that is now moot. Triage its issues (re-seed any survivor elsewhere via
`grove-llm inbox-add`), then abandon the grove — decide between running its own
finish cycle vs removing worktree + deleting branch directly (it has nothing worth
merging). Cross-grove action — confirm with the user before deleting.

**3. Glossary cleanup (`CONTEXT.md` TUI section).** Prune or mark-superseded the
trellis/zellij-era entries (Dashboard proxy, Seam frame, Controlling process, host
surface/driver/tick, trellis framework, trellis hosting API, Owned zellij
substrate, Head binary, harness-pane crate, TerminalEmulator, PtySession, dynamic
mouse capture, pane-local copy mode, Nav plugin, Detail proxy, Whichkey bar as
host-pane). **Keep** the rmux-substrate entries (Leader, Focus, Nav surface,
Rendered-history capture, open-in-editor) and the surviving cross-cutting ones
(Fleet, Working set, Workspace, Presentation boundary), reconciled to the
inversion. Some of this may already be done incrementally by 010/050/060 — finish
whatever remains.

## Done when

The ADR tower is swept (each marked superseded / amended / surviving with verdicts
recorded), the `bugs` grove is retired, and `CONTEXT.md` reads as an rmux-substrate
glossary with no live trellis/zellij scaffolding language. This empties the
050-plan-rebuild node and the grove → triggers the finish cycle.

## Notes

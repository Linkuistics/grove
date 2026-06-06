# 010-plan

**Kind:** planning

## Goal
Shape the grove for the blank-host-surfaces bug: decide the tree shape and the
acceptance bar, then grow the tree.

## Context
The whole mandate is the drained inbox observation
(`native-host-surfaces-render-blank`): grove's nav + whichkey host surfaces
render blank in `grove tui` while embedded terminal panes render fine. Deep
isolation already done; suspect path is the host-surface → `HostPane` →
`CharacterChunk` compositing seam.

## Done when
- Tree shape and acceptance bar settled with the user.
- Root brief fleshed out; the work leaf(s) created.

## Decisions (running log)

**Tree shape — one diagnose-and-fix leaf.** The isolation in the observation is
already deep and the suspect path is narrow, so root-cause-then-fix fits one
focused session. The fix shape is *not* pre-decomposed (premature before the
root cause is known); systematic-debugging drives it from a failing reproduction
test, and we decompose later only if the cause proves large (constraint 4).
Rejected: a diagnose-only leaf (defers the fix, extra ceremony) and a pre-planned
diagnose→fix→test triple (premature structure).

**Acceptance bar — automated trellis test + manual tmux.** Add a regression test
at the trellis `HostPane` / `CharacterChunk` compositing layer (render a host
surface end-to-end, assert non-blank cells reach the composited output) AND
confirm visually in tmux. The automated test is the structural guard that was
missing (these surfaces may never have rendered end-to-end); tmux confirms the
real terminal so the test can't pass green against the wrong layer. Manual repro
is already cheap (the observation reproduced in tmux).

**Pre-decided (not grilled):**
- Trellis is fully in scope — grove-owned hard fork, no upstream-compat concern
  (ADR-0020/0021).
- One root cause likely covers both nav and whichkey (both `HostSurface`-backed,
  both blank, terminal panes fine) — a hypothesis to confirm in diagnosis, not a
  human decision.
- No new glossary term emerged; no ADR warranted yet (a normal regression test
  isn't hard-to-reverse/surprising — an ADR gets written only if the root cause
  reveals a durable design subtlety).

## Notes
Grew the tree: `020-diagnose-and-fix` (work). See root `BRIEF.md`.

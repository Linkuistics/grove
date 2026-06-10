# 030-nav-model

**Kind:** work

## Goal

Rework the nav's row model for the **grouped idle shape** and lifecycle-aware
listing (010-plan Q1, Q6): repo section headers with collapsible folds,
seeds listed alongside live groves, and a scrolling list.

## Context

- `src/tui/nav.rs` — today a flat `Vec<NavItem>` of live groves only, no
  scroll (`row >= inner.height` just truncates).
- Q6 verdict: headers are selectable rows; `h` collapses / `l` expands the
  repo under the cursor; Enter on a header toggles its fold; fold state
  ephemeral per session. Lone header auto-hides at N=1.
- Lifecycle: seeds appear as rows (visually distinguished); their Enter/
  preview semantics land in 060/070 — here they just exist in the model.
- `Nav::render` stays a pure snapshot → `Buffer` function, headless-tested.

## Done when

- Nav rows are a sum type (header vs grove) built from the fleet; folds
  collapse a repo's groves; selection/rebuild-preserve logic handles both row
  kinds (rebuild keeps cursor on the same grove *and* keeps fold state by
  repo root).
- The list scrolls: the cursor stays visible for fleets taller than the rect.
- Seeds render with a lifecycle marker; live groves unchanged at N=1.
- Headless tests: fold/unfold, cursor-over-header, scroll-into-view,
  seed rows, N=1 auto-hide, rebuild preservation.

## Notes

Enter on a grove row keeps today's open/focus-harness behavior; Enter on a
seed row is wired in 070 (inert until then is acceptable *within this leaf*).

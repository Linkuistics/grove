# 130-native-detail — brief

**Kind:** node (work)

## Goal

Render each grove's **detail** (task tree + inbox triage + capture + `$EDITOR`)
**natively, in-process**, beside that grove's harness — in the **constant-nav +
swapped content region** model (ADR-0022), not a per-grove tab. The native
realisation of the per-grove [[detail proxy]] (ADR-0019): N native [[host
surface]]s the in-process host renders, one per grove, mounted into the content
region when its grove is selected.

## Context

- **ADR-0022 reshaped the harness UX:** the nav is **constant** (always on
  screen); selecting a grove swaps that grove's harness + detail into a **content
  region**; non-selected harness ptys keep running off-screen. This **superseded
  the grove-as-tab model** (ADR-0018/0019 mechanism) that this node was first
  scaffolded against. The per-grove-detail UX (ADR-0019) survives; only "detail
  lives in a grove *tab*" → "detail mounts into the content region" changed.
- **Reuses the v1 `App` detail rendering** — the work is **scoping** (one fixed
  grove, detail-only) and **native lifecycle** (N independent surfaces, parked/
  mounted). Capture + inbox triage already run in-process on the home dashboard
  surface (`DashboardSurface::process_action`, leaf 110/030); the detail surface
  reuses those exact `grove-llm` shell-outs.

## Forks settled (this node's grilling)

1. **Constant nav + swapped content, no tab-per-grove (ADR-0022).** Driven by the
   nav-must-be-constant requirement + two facts: pinning a *tiled* pane across tabs
   is not native (floating-only), and a pty stays alive/capturing scrollback while
   not displayed (per-pane reader thread). Tabs were never load-bearing for
   aliveness.
2. **Park/mount mechanism → settled by leaf 010 (ADR-0023).** Verdict: **Candidate
   A — native `suppressed_panes`** — realised as in-place `TiledPanes::replace_pane`
   into a stable content slot (incoming pane inherits the slot geom; displaced pane
   parked in `suppressed_panes`, kept alive by the existing ~15-site pty/resize
   routing). B (grove pool) rejected — it sits off that routing. The N-detail-surface
   widening is a keyed **host-surface registry + id-only `MountHostSurface`**
   instruction (a surface can't ride a `Clone+Debug` `ScreenInstruction`); `HostDriver`
   gains a swap verb; `GoToTab`/`Alt-1..9` binds retire. Proven by code trace + a
   throwaway (green first run, not retained — the decision is the artifact).
3. **`$EDITOR` as a real trellis terminal pane + exit observability** (leaf 030;
   first slice of ADR-0020 §6). The "in-process tty hand-off" premise was stale —
   host surfaces render server-side with no tty (ADR-0021).

## Done when (node acceptance — ADR-0022 model)

- A **constant nav** + a **content region**: selecting a grove mounts that grove's
  detail (task tree + inbox + capture) beside its harness; triage / capture work
  in-process; `$EDITOR` (Ctrl-E) edits via a trellis editor pane.
- **Two groves selectable in turn** each show their **own** detail, no cross-talk;
  the non-selected grove's harness stays alive (scrollback intact) off-screen.
- Switching back to a grove **restores** its detail + harness (parked state
  survives).
- `cargo build`/`cargo test` green; grove core stays `ratatui`-free below the seam.

## Decomposition (this node)

```
010-content-swap-spike  (planning) [done → ADR-0023] DECIDED: Candidate A —
                        native suppressed_panes via in-place replace_pane into a
                        content slot; N-surface widening = registry + id-only
                        MountHostSurface; HostDriver swap verb; GoToTab binds retire.
020-detail-surface      grove: per-grove DetailSurface (scoped App, detail-only)
                        mounted into the content region; nav select swaps it in.
                        The headline acceptance (minus $EDITOR).
030-native-editor       $EDITOR as a trellis terminal pane + embedded-tool exit
                        observability (first slice of ADR-0020 §6); wire
                        EditBody/EditObservation; seeds 150-working-set.
```

## Notes

- **Scope boundary with 150-working-set:** 150 owns the *responsive multi-pane*
  content region (harness + terminal + yazi + lazygit, pack-vs-degrade, toggles).
  130 stays minimal — detail + harness in a simple split inside the content region.
- **120-native-nav (done) is reframed, not re-opened** (ADR-0022): its
  "Enter opens a tab" delta — Enter now swaps content — lands in 010/020 here.
- Retire the `GROVE_TUI_CONFIG` `GoToTab`/`Alt-1..9` binds when 010/020 land the
  swap switcher (they were the tab-switcher).
- Depends on **110** (host API) and **120** (the nav surface + leader).

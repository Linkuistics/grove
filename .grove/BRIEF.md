# tui-blank-host-surfaces — brief

## Goal
Find and fix the bug where grove's own native host surfaces — the left
`grove-nav` pane (fleet grove list) and the bottom `grove-whichkey` bar — render
BLANK in `grove tui`, while embedded *terminal* panes (harness/term/yazi/vcs)
render fine. Leave behind a regression guard so host surfaces can never again
silently render to nothing.

## Done when
- The nav and whichkey surfaces render their content in a real terminal (tmux).
- A trellis-level regression test exercises the host-surface → `HostPane` →
  `CharacterChunk` compositing path end-to-end and asserts non-blank cells reach
  the composited output (the test that was missing — these surfaces "may never
  have rendered end-to-end" because the fs-watch hang masked it).
- The root cause is understood and recorded (an ADR if it reveals a durable
  design subtlety; otherwise the leaf's findings + the test suffice).

## Decomposition
A single work leaf — `020-diagnose-and-fix`. The inbox observation already did
deep isolation (reproduced in tmux, ruled out the fs-watch and lazy population,
narrowed to the host-surface → pane render path), and the suspect path is narrow,
so root-cause-then-fix fits one focused session. The fix shape is deliberately
*not* pre-structured: systematic-debugging drives it from a failing reproduction
test. Decompose further only if the root cause proves large (grove stays lazy —
constraint 4).

## Pointers
- ADRs a session here must read: docs/adr/0020 (fork zellij into trellis),
  0021 (trellis hosting API is a library you link — the server-side third pane
  kind), 0026 (trellis is the only TUI). Skim 0013 (presentation boundary) for
  the core↔presentation seam.
- Glossary terms in play: *host surface / host driver / host tick*, *Nav plugin*
  (now native), *Whichkey bar*, *trellis hosting API*, *trellis framework* (see
  CONTEXT.md).
- Suspect code: `src/tui.rs` `mod native` (`DashboardSurface::draw` ~4471,
  `WhichkeySurface::draw` ~4040 — both copy an off-screen `Buffer` into the host
  `buf`); `src/trellis_host.rs` (surface registration + layout injection);
  `crates/trellis/zellij-server/src/panes/host_pane.rs` (`HostPane` render →
  `CharacterChunk` compositing — prime suspect); `crates/trellis/.../tab/mod.rs`
  `inject_host_pane` (~5765) / `inject_whichkey_pane` (~5839).
- Contract doc: `crates/trellis/HOST_API.md`.

## Notes
- **Both surfaces blank, terminal panes fine** strongly implies a single defect
  in the host-pane render/compositing path shared by both, *not* two independent
  bugs. Confirm this during diagnosis; if confirmed, one fix covers both.
- `App::render` TestBackend unit tests pass — the gap is **downstream** of the
  off-screen `Buffer` grove produces, in how that buffer reaches and composites
  through trellis's server-side `HostPane`.
- Trellis is a grove-owned hard fork (ADR-0020/0021); fixing inside
  `crates/trellis/` is fully in scope, no upstream-compat concern.
- Built recently by leaves 120-native-nav / 130-native-detail / 140-native-whichkey.

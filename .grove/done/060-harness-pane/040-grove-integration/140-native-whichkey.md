# 140-native-whichkey

**Kind:** work

## Goal

Give grove a single, **grove-owned whichkey** — one full-width line across the
**bottom of every [[workspace]] tab** showing context-sensitive key hints (sigils)
— rendered **natively, in-process**, and make it the *only* hint surface. This is
the native realisation of the [[whichkey bar]] (ADR-0019), without a WASM plugin.

## Context

- **ADR-0020 + ADR-0019 (UX).** The whichkey requirement stands: one grove-owned,
  full-width bottom bar; the nav/detail and the harness draw **no** hint line of
  their own. The *mechanism* changes — the old bottom-pinned WASM plugin (build.rs
  embed) is replaced by a **native bottom region** the in-process host always
  renders.
- It learns the current context from the host's own focus/mode state (which surface
  is focused — nav, detail, harness, aux tool) and shows that surface's keys. Hint
  sets are static per context unless a later need says otherwise.
- Hints use **sigils** (`⏎` enter, `⎋` esc, `↑↓`, `⌂` home, …), consistent with the
  nav (120).

## Done when

- Every tab shows **one** grove-owned whichkey line across the full bottom width,
  rendered natively.
- The nav/detail surfaces and the harness no longer draw their own hint lines.
- Hints are context-aware (at least: nav-focused vs grove-tab-focused) and use
  sigils.
- `cargo build`/`cargo test` green.

## Notes

- No second WASM artifact, no build.rs embed, no layout-pin — it is part of the
  native frame the host draws (much simpler than the old plugin bar).
- Carries **no** grove state — only focus/context → which hint set to show.
- Sequenced after 120/130 so the contexts it describes (nav, grove tab, detail)
  exist; otherwise independent.

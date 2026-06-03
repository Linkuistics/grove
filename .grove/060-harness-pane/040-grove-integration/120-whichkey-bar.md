# 120-whichkey-bar

**Kind:** work

## Goal

Give grove a single, **grove-owned whichkey** — one full-width line across the
**bottom of every [[workspace]] tab** showing context-sensitive key hints (sigils)
— and stop every other surface from drawing its own (ADR-0019). Realised as a
[[whichkey bar]] zellij plugin, a horizontal sibling to the [[nav plugin]].

## Context

- **ADR-0019** + live-test feedback: today the [[dashboard proxy]] draws its own
  hint line (wrong owner, not full-width). The whichkey must be grove-controlled,
  span the whole bottom, and be the *only* hint surface — the dashboard/detail
  proxy and the harness draw none.
- A bottom-pinned zellij plugin (pinned in the bundled layout, like the nav
  sidebar) renders the hints. It learns the current context from zellij events
  (`ModeUpdate`, focus/`TabUpdate`, `Visible`) and shows the keys for what's
  focused — nav keys when the nav is focused, harness/grove keys otherwise.
- Hints use **sigils** (`⏎` enter, `⎋` esc, `↑↓`, `⌂` home, …), consistent with
  the nav footer (090).

## Done when

- Every tab shows **one** grove-owned whichkey line across the full bottom width.
- The detail proxy / dashboard and the harness no longer draw their own hint
  lines.
- The hints are context-aware (at least: nav-focused vs grove-focused) and use
  sigils.
- The bundled config still passes `zellij setup --check`; the plugin rides inside
  the single grove binary like `grove-nav` (build.rs embed).

## Notes

- A second bundled WASM plugin: extend `build.rs` to build/embed it as for
  `grove-nav`, and add it to the layout (bottom bar) — keep the "single binary"
  property.
- It carries **no** grove state — only zellij context → which hint set to show.
  Hint sets are static per context (not piped) unless a later need says otherwise.
- Independent of 090/100 in principle, but sequenced after so the contexts it
  describes (nav, grove tab) exist.

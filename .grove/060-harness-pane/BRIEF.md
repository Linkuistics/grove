# 060-harness-pane — brief

**Kind:** work (substrate decided: grove-owned **zellij** — ADR-0015, which
superseded ADR-0014's in-process-pty embed after 020 evaluated zellij on its own
terms)

## Goal

Build the **harness pane**: from the dashboard, open a grove's live harness
session (`grove do <name>` → claude code / codex) beside the dashboard,
interact with it, and switch between groves. This is concern 3 — the grove's
headline feature.

**Substrate (020 → ADR-0015): grove-owned zellij**, Strategy 1b. A **head binary**
launches zellij with grove's bundled config + bars-free layout (presents as a
single binary); the dashboard is a native zellij pane running grove's ratatui
(evolving v1 `grove tui`); harnesses are native zellij panes; grove drives the
layout via `zellij action`. Copy mode / scrollback / search / floating panes /
session persistence / web client come native+free. The in-process-pty
`harness-pane` crate (010, ADR-0014) is the **shelved fallback**, not the live
path.

## Context

- **Substrate decided (020 → ADR-0015): grove-owned zellij** (Strategy 1b), which
  superseded ADR-0014's in-process-pty embed. zellij ≠ tmux: ADR-0014 had rejected
  the whole "owned multiplexer" bucket on *tmux's* weaknesses without evaluating
  zellij; 020 ran that missing comparison and zellij cleared the only common
  blocker (keybinding/chrome tameability).
- v1 gives: sync event loop, `RepoView` data layer, shell-out-to-`grove` writes,
  the master/detail dashboard. v2 keeps that dashboard and renders it as a native
  zellij pane; harnesses become native zellij panes beside it.
- The dashboard-as-switcher model: the user picks the focused grove *in the
  dashboard*; grove opens/focuses harness panes via `zellij action` (stable
  pane-ID addressing). Harnesses are alive for parallel work.
- **Boundary (ADR-0013) unchanged:** the dashboard ratatui is *above* the seam;
  `RepoView`/shell-out writes stay *below* and `ratatui`-free. The substrate
  (zellij) sits below/around the presentation.
- **The in-process-pty `harness-pane` crate (010, ADR-0014) is the shelved
  fallback** — proven evidence the embed works, kept in-repo and history, not on
  the v2 path and not built on further.

## Done when

- The 060 "Done when" is met **via the zellij substrate** (see the reshaped
  `040-grove-integration` leaf for the concrete acceptance): head binary launches
  zellij looking like a single binary; the dashboard pane runs grove's ratatui;
  selecting a grove opens `grove do <name>` as a native zellij pane; switching
  focus between groves works; the dashboard stays the switch surface.
- Copy mode / scrollback / search / session persistence are **not built** — they
  are native zellij (this retired 030).

## Naming (historical — the fallback crate's API)

The 010 crate's three-layer naming (`TerminalEmulator` / `PtySession` / "pane" as
a layout region) is **settled language for the shelved [[harness-pane crate]]
fallback**. In the live zellij path the harness is a native zellij pane (no
`TerminalEmulator`/`PtySession`), but "pane" still means a *layout region* and the
glossary entries remain valid for the fallback.

## Decomposition (this node)

Base crate → **substrate decision** → integration. (The feature leaf 030 was
retired moot by the substrate decision.)

- **`010-embed-pane`** — scaffolded `crates/harness-pane` (the embed); proven by
  synthetic-ANSI + real-child tests. **[done — now the shelved fallback]**
- **`020-decide-zellij-substrate`** *(planning)* — evaluated zellij on its own
  terms vs the embed; grill + throwaway 1b spike → **zellij wins, ADR-0015
  supersedes ADR-0014**. **[done]**
- **`030-scrollback-copy`** — **retired moot**: zellij provides per-pane copy
  mode, scrollback, and search natively/free (ADR-0015).
- **`040-grove-integration`** — *(reshaped by 020)* grove's head binary launches
  the zellij substrate; dashboard pane + native harness panes; drive via
  `zellij action`. Within-repo only; cross-repo fleet is 070. **[live]**

## Notes

The **tmux** backend stays retired (ADR-0014) — do not re-litigate it. The
substrate is **zellij** (ADR-0015). The **1a** WASM-plugin dashboard
(`zellij_widgets` + pipe IPC) is a recorded *future refinement*, not the v2 path;
v2 is **1b** (native-pane dashboard driving zellij via `zellij action`). Keep
grove **core** (`RepoView`/writes) `ratatui`-free below the ADR-0013 seam.

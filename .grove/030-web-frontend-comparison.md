# 030-web-frontend-comparison

**Kind:** work (research + design comparison → produces a docs/ comparison doc)

## Goal

Compare a **web front-end** against the **Ratatui TUI** as grove's presentation
layer, over the *same* tmux-owned-harness backend. This is a **presentation-layer
fork, not a backend fork**: 010-plan D2 (grove owns a tmux session; harnesses are
windows it creates) is **not** in question — the comparison is **hybrid** (web UI
over a tmux backend). What is in question is D3 (the TUI dashboard is window 0 of
grove's owned session): should the front-end the user looks at be a terminal UI
rendered as a tmux window, or a browser app talking to a grove server that drives
the same tmux backend?

Output: a decision-ready comparison doc that sketches the hybrid web architecture,
weighs it against the Ratatui-TUI presentation on the dimensions below, and gives
a recommendation (or a clear trade-off table) feeding the architecture decision in
040.

## Context

- **Consumes 020** (`docs/research/tmux-owning-frontends.md`) — its findings about
  socket ownership, persistence, and window lifecycle all still apply to the tmux
  backend in *both* options. **One 020 conclusion must be re-examined here:** 020
  concluded "control mode (`-CC`) is unnecessary *because grove's dashboard is
  itself a tmux window*." A web front-end flips that premise — the grove **server
  becomes a tmux client** that must render windows/panes as native (browser) UI,
  which is *exactly* iTerm2's `-CC` use case. So under the web architecture,
  control mode (or an equivalent pty/`pipe-pane` bridge) re-enters as a live
  option. Resolve which.
- **User intent (from the grilling that commissioned this leaf):** the draw of a
  web front-end is **(a) richer UI than a terminal** (clickable task trees,
  multi-pane layouts, diffs, graphs) and **(b) remote / browser access** (drive
  grove from any browser, not tied to the host terminal). Weigh these two heavily.
  Sidestepping tmux complexity and multi-user/sharing were **explicitly not**
  primary motivations (though multi-user may emerge as a consequence of going
  web — note it, don't chase it).
- **Scope is hybrid only.** Do **not** evaluate a full web-native architecture
  that removes tmux (server manages harness subprocesses directly). The tmux
  backend stays; the fork is purely the presentation layer.
- v1 code that exists today: the Ratatui TUI (`src/tui.rs`, `src/repo_view.rs`) —
  the incumbent presentation layer. A web front-end is a new, parallel surface,
  not an edit of these.
- **Downstream:** this gates 040 (`decide-tmux-integration`). Its outcome may
  **reframe** 040 from "decide tmux integration mechanics (for a TUI)" into
  "decide the presentation architecture *and* the tmux mechanics it implies." If
  the recommendation is "go web," 050 (harness-pane) and 060 (fleet-view) change
  shape too — flag that, don't silently assume the TUI plan.

## Done when

A doc (suggested `docs/research/web-frontend-vs-tui-presentation.md`; promote to a
`docs/specs/` design if it turns into the chosen architecture) exists that:

1. **Sketches the hybrid web architecture** concretely enough to compare: what the
   grove server is (e.g. an `axum`/`actix` process), how the browser renders
   harness terminals (xterm.js over websockets), and **how that front-end attaches
   to the tmux backend** — the three candidate bridges, with the trade-off of
   each: tmux **control mode (`-CC`)** (the server as control client), **`pipe-pane`
   / `capture-pane`** streaming, or a **pty wrapper** around `tmux attach`. Tie
   this back to 020's control-mode findings.
2. **Compares the two presentation layers** on the weighted dimensions below.
3. **Cites primary sources** for prior-art claims and failure modes (web-terminal
   and web-IDE front-ends that drive tmux/pty); records silences explicitly ("no
   primary source found"), per 020's discipline.
4. **Recommends** — either "stay TUI", "go web", or "TUI now / web later" — with
   the trade-off table that justifies it.

### Comparison dimensions (weight the first two heavily)

1. **UI richness** *(primary driver)* — clickable trees, multi-pane layouts,
   diffs, graphs, inbox triage. What a browser unlocks that a TUI can't, and what
   (if anything) is lost (latency, terminal fidelity, keyboard-first speed).
2. **Remote / browser access** *(primary driver)* — browser-from-anywhere vs
   tmux-over-ssh / mosh. What auth, transport (TLS/websockets), port, and
   deployment cost this adds.
3. **Persistence & crash isolation** — the tmux backend preserves harness survival
   in *both* (020 / D2). But a web front-end adds a **new long-lived component**
   (the grove server) whose failure semantics matter: browser disconnect →
   reconnect; server crash → state loss?; how the websocket↔tmux session rebinds.
   Compare against the TUI, where "front-end dies" just means "detach from tmux."
4. **Walk-away-ability** *(grove constraint 6)* — a TUI is one Rust binary. A web
   front-end adds an HTTP/WS server, a browser dependency, likely a JS/TS build
   step and asset bundle, auth, and a port. Square this against grove's
   single-binary, walk-away ethos honestly.
5. **Dependency / complexity budget** — ratatui+crossterm (one binary, sync loop)
   vs server framework + xterm.js + websocket plumbing + frontend build. Relate to
   concern 4 (async revisit, now 070): a web server is inherently async and may
   force the sync→async refactor the spine deferred.
6. **Multi-repo fleet view** (concern 1) — is the fleet view materially better in a
   browser (richer filtering/layout) than in a TUI? This is where richer UI may
   pay off most.
7. **Incrementality / effort** — v1 already ships the TUI. Estimate the relative
   lift and whether web can be additive (same backend, new front-end) or forces a
   rewrite.

### Prior art to survey (at least)

- **Web terminals over a pty/tmux:** `ttyd`, `gotty`, `wetty`, `sshx` — how they
  bridge a browser (xterm.js) to a server-side shell/tmux, and what broke
  (reconnect, resize/SIGWINCH, scrollback, latency, auth). Note specifically any
  that attach to **tmux** and *how* (control mode vs pipe-pane vs pty).
- **Web IDEs / remote dev front-ends:** `code-server` / VS Code Server, Theia,
  GitHub Codespaces web UI, Jupyter terminals — how they own a server + render
  terminals in-browser, and their persistence/reconnect model.
- **Anything driving tmux from a web backend specifically** — if little exists,
  *that scarcity is itself a finding* (it tells us how much we'd be building from
  scratch vs standing on prior art). Record it.

### Search bias

Same post-mortem framing as 020: "after real use, what went wrong with a
browser-terminal-over-tmux/pty front-end?" Demand a citation per failure-mode
claim; record silences. Broad "how to use xterm.js" tutorials are not the target.

## Notes

- The deliverable is a *comparison*, not an implementation. The decision it feeds
  (040) is where the tree may be reshaped.
- Keep D2 fixed: tmux owns the harnesses in both options. Resist scope-creep into
  re-litigating the multiplexer-ownership decision — that was settled and 020
  confirmed it.
- If the comparison surfaces that "go web" forces the async refactor (concern 4 /
  070), that interlock is a first-class finding for the recommendation, not a
  footnote.

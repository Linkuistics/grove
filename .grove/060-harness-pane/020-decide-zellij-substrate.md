# 020-decide-zellij-substrate

**Kind:** planning (grill + cheap spike → amend or supersede ADR-0014)

## Goal

Decide, on zellij's **own** terms, whether grove's harness/presentation substrate
should be **zellij-as-owned-multiplexer** rather than the in-process-pty embed
ratified in ADR-0014. This reopens a thread the decision record half-skipped:
the root brief named D2 as "tmux **vs zellij** vs in-process pty", but 020-research
studied *tmux-ownership* and ADR-0014 framed the call as tmux-owner vs in-process
pty — folding zellij into the generic "owned multiplexer" bucket and rejecting it
on *tmux's* weaknesses. zellij ≠ tmux; it deserves its own evaluation.

The deliverable is a **decision** (with the user) plus an **ADR amendment or
superseding ADR**, backed by a throwaway **Strategy-1 spike** measuring the real
friction. If the embed stands, this leaf closes cleanly and 030-scrollback-copy /
040-grove-integration proceed unchanged. If zellij wins, **030 evaporates** (zellij
gives copy mode free) and **040 reshapes** (dashboard becomes a zellij plugin +
layout, not a `src/tui.rs` consumer of the crate).

## Context

Surfaced in conversation while building 010-embed-pane: "to what extent are we
reimplementing zellij, and can we reuse it?" The sharpened proposal is an
**inversion** of ADR-0014's embed — not embedding zellij inside ratatui (which
fails: zellij wants the whole screen), but letting **zellij own the screen and the
rendering substrate, with ratatui drawn *into* the panes that need dashboard
chrome**, layered atop zellij.

### The decisive enabling fact (researched)

zellij **plugin panes render by printing UTF-8 ANSI to STDOUT** — "you can print to
a zellij plugin like printing to any terminal" — and the **`zellij_widgets`** crate
already marries **ratatui + crossterm** to that surface for building plugin UIs. So
"ratatui inside a zellij pane" is a supported, existing pattern, not a hypothesis.
zellij is MIT-licensed (forkable/vendorable). Its terminal emulator is a bespoke
`vte::Perform` **`Grid`** in `zellij-server/src/panes/grid.rs` (not `vt100`/
`tui-term`), and is **not cleanly separable** from the server.

### Three strategies (cost-ordered) to weigh in the grilling

- **Strategy 1 — grove dashboard = a zellij plugin (ratatui via `zellij_widgets`),
  harnesses = native zellij terminal panes, composed by a zellij KDL layout.**
  Reuse: *everything* — emulator, **copy mode, scrollback, search, floating panes,
  session persistence, the web client**. 030's entire scope evaporates. No code
  taken; rides upstream zellij. The cheap version of the user's idea. Asymmetry that
  makes it elegant: cheap "double-emulation" for the chrome (ratatui→ANSI→zellij
  re-parses), but *native* zellij fidelity for the harness panes (no double-emulation
  there).
- **Strategy 2 — vendor zellij's `grid.rs`/`terminal_pane.rs`/Output as a library**,
  drop server+client+wasmer, ratatui drawn into the grid. Reuse: the performant
  emulator + copy mode without the server. Cost: the Grid is entangled with
  zellij-server; you fork a large subtree and hand-track upstream forever — worst
  cost/benefit (takes the hardest-to-extract code, forgoes the easiest-to-use
  surface). Likely a non-starter; include only to dismiss explicitly.
- **Strategy 3 — keep ADR-0014's in-process embed** (010 already built and 050-proven),
  read zellij's copy-mode *design* as prior art for 030. Reuse: ideas only; we own a
  small comprehensible stack and build 030 ourselves (~1 session).

### The load-bearing tension (don't let the grilling skip it)

Strategy 1's wins are large and *not* localized (copy mode + scrollback + session
persistence + web — erasing 030 and softening 070/080). But it **inverts grove's
identity** from "single binary, walk-away-able, tight dependency budget" into "a
zellij distribution / plugin you install into zellij" — the *exact* ethos ADR-0013
weighed when it deferred the web front-end. And the dashboard's core logic
(fs-watch, multi-repo `notify`, shell-out-to-`grove`) would live in or behind the
**WASM plugin sandbox**, reshaping ADR-0013's module seam into a sandbox boundary
(likely a separate grove process the plugin pipes to). The web client zellij offers
free is tempting, but it is ADR-0013's deferred axis, not ADR-0014's.

Prior read going in (to be tested, not assumed): the embed probably still stands for
v2 on the single-binary/ethos grounds, but it is a **closer call** than the tmux-owner
rejection, and zellij being a *materially stronger* owned-multiplexer than tmux
(plugin-first dashboard as a first-class citizen, ratatui-renderable, MIT, web client)
is a genuine gap ADR-0014 must address explicitly either way.

## Done when

- The **Strategy-1 spike** exists and is assessed: a throwaway zellij plugin that
  renders a trivial ratatui dashboard **beside a real harness terminal pane**
  (`zellij run -- grove do <name>` or similar), driven via a KDL layout. It measures
  the real frictions, not the happy path: (a) can the plugin run grove's **fs-watch /
  `notify` / shell-out-to-`grove`** work, or must that be a separate process piped to
  the plugin? (b) plugin-sandbox permission surface; (c) dashboard render latency
  through the double-emulation; (d) packaging/distribution shape (bundle zellij? depend
  on it installed? fork?). Throwaway — findings are the deliverable, like 050.
- **The decision is made with the user** across all three strategies, with the
  identity/ethos tension (single-binary vs zellij-distribution) and the ADR-0013
  boundary implications explicit.
- **ADR-0014 is amended or superseded**: either a short amendment recording
  "zellij-owner considered on its own terms, rejected on grounds X (and the
  zellij≠tmux distinction)", or a superseding ADR adopting a zellij strategy.
- **030/040 are reshaped to match**: if the embed stands, they are unchanged; if
  zellij wins, 030-scrollback-copy is retired as moot and 040-grove-integration is
  rewritten around the plugin+layout model. 070-fleet-view / 080-async-revisit are
  re-touched only if the verdict changes their shape.

## Notes

- The 010 embed crate (`crates/harness-pane`) stays as recoverable evidence either
  way — like 050's prototype, it is not wasted even if the substrate changes (it
  proves the embed path works, the fallback if a zellij strategy disappoints).
- Resist re-litigating tmux: that path is retired (ADR-0014). This is specifically
  zellij-vs-embed, the comparison ADR-0014 did not run.
- Keep the spike cheap and disposable. The point is the friction measurement
  (sandbox / fs-watch / packaging), not a working dashboard.

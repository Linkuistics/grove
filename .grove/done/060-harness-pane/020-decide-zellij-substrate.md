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

## Decisions (running log)

### D-reframe — "single binary" is not the axis; UX/DX is (settled with user)

The opening grilling question asked whether the single-binary identity is a hard
veto against a zellij-distribution model. The user dissolved the framing: it is
**negotiable**, because a **head binary** (`grove`/`grove tui` launching zellij
with grove's own config and auto-installing grove's plugin(s)) makes the result
**look like a single binary** regardless of process count. "One binary vs many"
is a technical detail the user does not care about.

The plugin can be a single **proxy plugin** that pipes back to the main grove
binary — so the dashboard's heavy logic (fs-watch, multi-repo `notify`,
shell-out-to-`grove`) lives in the **head binary outside the WASM sandbox**, not
trapped inside it. This dissolves the brief's "core logic behind the WASM
sandbox" tension (the sandbox becomes a render/IPC seam, not a logic prison) and
softens the ADR-0013-boundary concern.

**Consequence for this leaf:** the decision axis is **UX/DX**, not ethos/identity.
The ethos veto is off the table. zellij is therefore a live — arguably *preferred*
— direction, not a candidate to reject on identity grounds. The spike's burden of
proof flips from "is the identity cost tolerable?" to "does anything actually
**block** the head-binary + proxy-plugin + native-panes model, in UX/DX terms?"
The load-bearing unknown becomes the **proxy-plugin ↔ head-binary IPC** and
**keybinding/control collisions** (zellij's keys vs grove's vs the harness app's),
not packaging.

### D1 — zellij is preferred; the spike hunts for blockers (settled with user)

Burden of proof: **default to adopting zellij.** The spike's job is to surface a
genuine *blocker*, not to prove zellij is comparatively better. An ambiguous or
merely-viable result resolves **toward zellij**. The user "loves WASM" and finds
the head-binary + proxy-plugin + native-panes architecture "very right."

Sunk cost of the already-built embed (010/050) is **explicitly not a factor** —
the user's general rule: "the right thing *is* the right thing, economics be
damned." So the embed's proven status earns it no weight here; it remains only as
a recoverable fallback if a blocker kills zellij.

Candidate blockers the spike must probe (a genuine failure on any one of these is
what would keep the embed):
1. **Proxy-plugin ↔ head-binary IPC** — can a WASM plugin hold a bidirectional
   channel to the long-lived head binary, with acceptable latency/throughput?
   (The brief's question (a) is pre-answered: heavy logic lives in the head
   binary, not the sandbox — so this is the residual unknown.)
2. **Keybinding / control collisions** — can zellij's modal keys be configured /
   locked so they don't collide with grove's dashboard keys *and* the harness
   app's keys (claude/vim/codex)? Can zellij's chrome (status/tab bars, pane
   frames) be hidden/styled to read as grove's own UI?
3. **Rendering** — ratatui-via-`zellij_widgets` into a plugin pane, and is the
   double-emulation latency acceptable for the chrome?
4. **Packaging** — head binary launches zellij with grove config + installs the
   plugin; bundle/vendor vs depend-on-installed.

### D2 — spike targets the native-pane dashboard (1b), simplest first (settled with user)

Research finding that reshaped the blocker hunt: "dashboard as a zellij plugin"
is only one way to ride zellij, and the costlier one.

- **1a — plugin**: dashboard rendered by a WASM plugin (`zellij_widgets`), in the
  sandbox, talking to the head binary over zellij's **pipe** API. The pipe docs
  warn it is CLI-shaped, re-renders per message, "usually slower than piping to
  other programs," and *do not describe a persistent channel* for a long-running
  external process — a real latency/persistence unknown.
- **1b — native terminal pane**: dashboard is a plain terminal pane running
  grove's own ratatui (essentially v1's dashboard), driving zellij by shelling
  out to `zellij action new-pane/focus/…` — which *is* grove's existing
  shell-out-to-`grove` write idiom. No WASM, no sandbox, no pipe IPC, and no
  double-emulation of the chrome (it's a normal terminal app, emulated once like
  any pane).

Key consequence: the IPC / sandbox / double-emulation worries are **1a-specific**;
1b routes around all of them while keeping every zellij win (copy/scrollback/
search/float/session/web on every pane). So a pipe-IPC failure bounces us from 1a
to 1b, **not** back to the embed. The **only blocker common to all zellij
strategies** — the true zellij-vs-embed make-or-break — is **keybinding/control
collision + chrome tameability**: can zellij's modal keys + furniture be tamed so
claude/vim/codex run normally in a harness pane, grove's dashboard keys work, and
zellij reads as grove's own UI?

**Spike scope (settled):** build the dashboard as a **native terminal pane** (1b)
driving zellij via `zellij action`, beside a real harness pane (claude/vim),
under a tamed grove zellij config + KDL layout. Probe the keybinding/chrome
blocker at minimum cost (no WASM toolchain). If 1b clears, zellij wins; the
1a-plugin model is a later DX refinement, not a gate. Throwaway — findings are
the deliverable (like 050).

### Spike findings — headless pass (zellij 0.44.3; spike at /tmp/grove-zellij-spike)

All headlessly-checkable blocker probes **PASS**:

1. **Config tames cleanly** — `default_mode "locked"`, `pane_frames false`,
   `simplified_ui true`, `show_release_notes/startup_tips false` → zellij reports
   `[CONFIG FILE]: Well defined.`
2. **Chrome is hideable** — a bars-free custom layout yields a layout with no
   tab-bar/status-bar panes (the default injects two `size=1 borderless=true`
   bar panes; ours has none) and suppresses the `zellij:about` startup float
   (`hide_floating_panes=true`).
3. **External control channel works (the 1b switcher seam)** — against a
   background session, `zellij action new-pane --name … -- <cmd>` created the
   pane and `dump-layout` introspected it. `zellij action` exposes
   **`focus-pane-id`** and **`close-tab-by-id`** → stable-ID addressing, exactly
   what dashboard-as-switcher needs. This is grove's shell-out write idiom applied
   to `zellij action`.
4. **Free win bundle confirmed present** — `web_client {}` block in the default
   config (free web client, ADR-0013's deferred axis); copy/scrollback/search/
   floating/session-persistence all native.

**`default_mode "locked"` is the key-collision mitigation:** zellij passes every
key through to the focused app except the unlock key (`Ctrl g`), and grove drives
panes from *outside* via `zellij action` (mode-independent) — so grove needs none
of zellij's interactive keybindings. The keybinding blocker is surmountable in
principle; what remains is the subjective visual pass.

**Two minor wrinkles noted (not blockers):** command panes default
`start_suspended true` (set `false` to auto-run the dashboard; verify on a real
attach — background dumps show it suspended as an artifact of having no client);
the unlock key `Ctrl g` collides with nvim's "show file info" (remappable to any
key).

**Awaiting human visual pass (A–G in spike FINDINGS.md):** does locked-mode
passthrough let claude/nvim run *exactly* as bare, and does the composite *feel*
like grove rather than a zellij session? That subjective check is the only thing
between here and the verdict.

### D3 — VERDICT: zellij wins; supersede ADR-0014 (settled with user)

Human visual pass (A–G): **"all ok."** No blocker found. Per D1 (ambiguous-or-
better → zellij), this settles the decision: **grove adopts zellij as its owned
presentation substrate**, superseding ADR-0014's in-process-pty embed.

The win bundle (copy/scrollback/search/floating/session-persistence/web — native
on every pane, consistently) is realised for free; the keybinding-collision tax
that sank tmux does not apply because zellij is modal (`locked` mode passes keys
through) and grove drives panes from outside via `zellij action`.

### D4 — build path is 1b now, 1a recorded as future refinement (settled with user)

v2 builds with the **native-pane dashboard (1b)**: reuse v1's ratatui dashboard
(`grove tui`), drive zellij via `zellij action` (`new-pane`, `focus-pane-id`,
`close-tab-by-id`). **1a (WASM plugin dashboard via `zellij_widgets` + pipe IPC)
is recorded in the ADR as a live future refinement, not a v2 commitment** — to be
revisited only if CLI-driving chafes. Rationale: 1b reuses v1 wholesale, carries
none of 1a's costs (WASM toolchain, slow per-message pipe IPC, perpetual
upstream-plugin-API tracking), and already has precise switcher control via stable
pane IDs; 1a's only marginal win (event-driven vs polled control) has not earned
its cost yet (grove constraint 4: lazy/optional).

# Web front-end vs Ratatui TUI — presentation-layer comparison

A decision-ready comparison of two **presentation layers** for grove over the
**same tmux-owned-harness backend**: the incumbent **Ratatui TUI** (rendered as
window 0 of grove's owned tmux session) versus a **browser app** talking to a
**grove server** that drives the same tmux backend.

This is a **presentation-layer fork, not a backend fork.** 010-plan D2 — grove
owns a dedicated `tmux -L grove` server and each harness is a window it creates —
is *fixed in both options* and was confirmed by 020
(`tmux-owning-frontends.md`). What is in question is **D3**: should the surface
the user looks at be a terminal UI rendered as a tmux window, or a browser app?
The scope is **hybrid only** — the tmux backend stays in both; we are not
evaluating a web-native architecture that removes tmux.

This doc feeds the architecture decision in **040** (`decide-tmux-integration`).
Its recommendation may reshape 040, 050 (harness-pane) and 060 (fleet-view) — see
§7.

## TL;DR

**Recommendation: TUI now / web later, behind an explicit core↔presentation
boundary.** Build the v2 TUI (fleet view, harness pane) as planned; keep all
grove logic (tmux driving, the `RepoView` data layer, shell-out writes) behind
the presentation boundary the v1 code already has, so a future web server is a
*second presentation over the same core*, not a rewrite. Revisit web when the two
things that justify it are proven — a fleet/dashboard whose richness demand the
TUI genuinely can't meet, and the async refactor (070) done for other reasons.

Why not "go web" now: the two primary draws (richer UI, remote access) are real
but **narrower than they look** — richness lands in grove's *dashboard chrome*,
not the harness panes (which stay terminal emulation either way), and the tmux
backend is *already* remotely attachable over ssh in both models. Against that,
web imposes a large, well-documented cost on four of grove's load-bearing values:
walk-away-ability, persistence simplicity, dependency budget, and the sync event
loop — and the evidence (§3) shows the new long-lived server is exactly where the
failure surface concentrates.

---

## 1. The hybrid web architecture, concretely

To compare fairly we have to sketch the web option to the same level of detail as
the TUI. The hybrid web front-end is **four new moving parts** bolted onto the
existing tmux backend:

```
 Browser (xterm.js + app chrome)          ── the new front-end
        │  WebSocket (TLS)
        ▼
 grove server  (axum / actix, tokio)      ── new long-lived component
        │  one of three bridges ↓
        ▼
 tmux -L grove   (UNCHANGED — D2 backend)  ── grove's owned server
        │
        ├─ window 0 … (no longer the dashboard; dashboard is the browser)
        └─ window N = harness <grove-name>
```

1. **A grove server** — an `axum`/`actix` process serving the app bundle and a
   WebSocket per live terminal. This is a *new long-lived component*; today grove
   has none (the TUI is a foreground process; `grove`/`grove-llm` are one-shot).
2. **A browser app** — renders grove's chrome (task trees, briefs, inbox) as real
   DOM, and renders each harness terminal with **xterm.js** (the same emulator VS
   Code is built on).
3. **A transport** — WebSocket, in practice behind TLS + auth + a chosen port.
4. **A bridge** from the server to the tmux backend — the crux, below.

### 1.1 How the front-end attaches to tmux — the three bridges

020 concluded control mode (`-CC`) is unnecessary *because grove's dashboard is
itself a tmux window*. A web front-end **flips that premise**: the grove server
becomes a tmux *client* that must render windows/panes as browser UI — exactly
iTerm2's `-CC` use case. So control mode re-enters as a live option, and we must
resolve which of three bridges the web server would use.

| Bridge | What it is | Prior art for a *web* backend | Verdict |
|---|---|---|---|
| **(A) Control mode** `tmux -C/-CC` | server is a control client; parse `%begin/%end/%output/%window-*`, render panes natively | **none outside iTerm2** | build-from-scratch, highest risk |
| **(B) `pipe-pane`/`capture-pane`** | stream/poll pane bytes to the browser | snapshots/logging only | wrong tool for interactive |
| **(C) pty wrapper around `tmux attach`** | server spawns a pty running `tmux attach`, relays raw bytes to xterm.js (the ttyd/gotty+tmux combo) | **the only one with real deployments** | least-bad |

**(A) Control mode — theoretically the "right" fit, empirically a greenfield
high-risk build.** Outside iTerm2 there is *no* proven non-terminal control-mode
client. The one general-purpose library, **libtmux, explicitly does not implement
it** — it shells out and parses `-F` output; a control-mode engine is unfinished
draft work
([libtmux #633](https://github.com/tmux-python/libtmux/issues/633), PR #605
"Experiment: Control mode", Draft). Windows Terminal **declined** to implement it
([microsoft/terminal #5612](https://github.com/microsoft/terminal/issues/5612),
closed). The protocol has **no version-negotiation handshake**
([tmux Control-Mode wiki](https://github.com/tmux/tmux/wiki/Control-Mode)) yet
**drifts every release** — flow control (`%pause`/`%continue`/`%extended-output`)
was wholly new in 3.2, `%config-error` in 3.3
([tmux CHANGES](https://raw.githubusercontent.com/tmux/tmux/master/CHANGES)) — so
a client must branch on tmux version with nothing to detect against. `%output` is
octal-escaped and must be un-escaped per line
([tmux.1 CONTROL MODE](https://raw.githubusercontent.com/tmux/tmux/master/tmux.1)).
Worst for a *server*: `-CC` wants a controlling TTY on stdin — with a pipe it
fails `tcgetattr failed: Inappropriate ioctl for device`
([tmux #3085](https://github.com/tmux/tmux/issues/3085)) or exits silently
([tmux #730](https://github.com/tmux/tmux/issues/730)). Even iTerm2, the
reference consumer, still hits ~50% window-launch failures
([iTerm2 #11174](https://gitlab.com/gnachman/iterm2/-/issues/11174)) and
state-desync zombies
([claude-code #24261](https://github.com/anthropics/claude-code/issues/24261)).
This *confirms and extends* 020's Q2 finding: control mode imports iTerm2's
entire complexity budget — and for a non-iTerm2 client there is no prior art to
stand on.

**(B) `pipe-pane`/`capture-pane` — snapshots only.** `capture-pane` is a
snapshot, not a stream (no push — you poll), drops escape sequences unless `-e`,
and can't even capture the visible region while scrolled
([tmux #4358](https://github.com/tmux/tmux/issues/4358)); `pipe-pane` has a
post-2.3 garbling regression
([tmux #594](https://github.com/tmux/tmux/issues/594)). Real users use these for
*after-the-fact logging* (tmux-plugins/tmux-logging), never interactive
rendering. Viable only for static pane thumbnails.

**(C) pty wrapper around `tmux attach` — least-bad, and tellingly familiar.**
This is the only bridge with actual real-tmux web deployments
([muxplex](https://github.com/bkrabach/muxplex) = ttyd+tmux+xterm.js;
[chrismccord/webtmux](https://github.com/chrismccord/webtmux) = a gotty fork
over a tmux pty) — both single-author, low-star, no-release, so "proven" is
generous. Its appeal: **tmux still draws; xterm.js just displays raw bytes**, so
grove writes no control-protocol parser or octal decoder. The residual cost is
inheriting tmux's client-sizing model wholesale — the smallest-attached-client
rule and stale geometry after a sized client detaches
([tmux #2594](https://github.com/tmux/tmux/issues/2594)) — plus prefix-key
collisions (the browser relays raw bytes, so `C-b` is eaten by tmux; *no primary
source found* for a tracked bug, it follows from raw-byte relay).

**The structural finding (§ informs the whole comparison).** Bridge (C) — the
only realistic web bridge — *re-derives the TUI's own rendering model*: in the
TUI, tmux renders the harness window and the user looks at it directly; in web
(C), tmux renders the harness pane and xterm.js relays its bytes to a browser.
The web option therefore adds a server + WebSocket + xterm.js + async runtime
**on top of the same backend rendering** the TUI already gets for free. The
"richer UI" payoff is **not** in the harness terminal (terminal emulation in both
cases) — it is in grove's *own chrome*: task trees, diffs, graphs, fleet layout,
inbox triage. Keep that scoping in mind for §2.

### 1.2 The scarcity finding

Driving *real* tmux from a web backend is a near-empty field, and the serious
efforts that started there **left**. The most polished browser "tmux"
projects — [Portal](https://avkcode.github.io/blog/portal-browser-tmux.html) and
[WebMux](https://github.com/jordanhubbard/webmux) — **dropped tmux and
reimplemented multiplexing on `node-pty` + xterm.js** rather than bridge it.
(WebMux's own README: "used in production by exactly one person, who also wrote
it.") No production-grade web server consumes tmux control mode to render panes.
That convergence — capable teams abandoning the tmux bridge — is itself a finding:
grove going web with bridge (C) would be **standing on a pty-over-WebSocket +
xterm.js transport pattern, but building the tmux-pane integration essentially
from scratch**, with the most-trodden alternative being "don't bridge tmux at
all" (which D2 forbids in scope here).

---

## 2. The comparison — weighted dimensions

The first two dimensions are the user's stated primary drivers and are weighted
heaviest. Each verdict marks which presentation it favours.

### 2.1 UI richness *(primary driver)* — **split: web wins the chrome, ties on panes**

A browser genuinely unlocks what a TUI can't render well: **clickable task
trees** with real expand/collapse, side-by-side **diffs**, dependency/commit
**graphs**, dense **fleet tables** with sortable columns, and rich **inbox
triage** (multi-select, drag, preview). For grove's *dashboard chrome*, web is
materially richer — there is no honest contest.

But the headline feature is the **harness pane**, and there the win evaporates:
under bridge (C) the pane is xterm.js rendering raw tmux bytes — the *same*
terminal emulation the TUI gets by being a tmux window, now with WebSocket
latency and xterm.js resize quirks (§3) added. What a TUI loses on chrome it
*keeps* on the pane: keyboard-first speed, zero added latency, native terminal
fidelity. **Net: web wins richness for the dashboard, ties (or slightly loses) on
the harness pane.** The richness case is real but localized — it is a
*dashboard* argument, not a *harness* argument.

### 2.2 Remote / browser access *(primary driver)* — **web wins the dashboard; backend already remote in both**

Browser-from-anywhere is a real ergonomic win for grove's *dashboard* over
`tmux attach`-over-ssh. Two honest qualifiers shrink the gap: (1) the **harness
backend is already remotely attachable in both models** — `tmux -L grove attach`
over ssh/mosh reaches the same sessions whether or not a browser exists; the web
adds remote access to the *dashboard*, not to the harnesses. (2) tmux-over-ssh
(and mosh, built for exactly the flaky-link case) is a **mature, battle-tested**
remote story; the web path adds TLS, auth, a chosen/forwarded port, and a
WebSocket whose proxy/idle behaviour is the single largest support category in
the prior art (§3.1). **Net: web wins for casual browser-from-anywhere dashboard
access; the cost is a real auth/transport/deployment surface, and the backend was
remote already.**

### 2.3 Persistence & crash isolation — **TUI wins decisively**

The tmux backend preserves harness survival in *both* (020/D2): kill the
front-end and the harness windows live on `tmux -L grove`. The asymmetry is the
*new long-lived component* the web adds. In the TUI, "front-end dies" ≡ "detach
from tmux" — there is nothing else to crash or reconcile. In web, the grove
server is a second stateful process, and the prior art shows **that is exactly
where the bugs live** (§3.2): pty-host restarts orphan terminals
([vscode #117548](https://github.com/microsoft/vscode/issues/117548)),
reconnection tokens desync
([vscode #210792](https://github.com/microsoft/vscode/issues/210792)),
replay buffers OOM the box
([theia #991](https://github.com/eclipse-theia/theia/issues/991)). "Persistence"
in the mature web stacks means *reattach-to-live-pty on reload with a capped
replay buffer* — **not** durable across server/machine restart (Codespaces
explicitly wipes visible terminal contents on stop;
[lifecycle docs](https://docs.github.com/en/codespaces/about-codespaces/understanding-the-codespace-lifecycle)).
The TUI gets tmux's persistence with no added failure surface; web bolts a fragile
reconnection layer in front of the same tmux persistence.

### 2.4 Walk-away-ability *(grove constraint 6)* — **TUI wins decisively**

A TUI is **one Rust binary** (`grove tui`). Delete grove and the user still has a
legible tmux backend. The web front-end adds an HTTP/WS server, a browser
dependency, a JS/TS build step + asset bundle, an auth story, and a port — every
one a thing that must be present, built, and served for the front-end to exist.
This squarely cuts against grove's single-binary, walk-away ethos. The one
mitigation: bridge (C) keeps the *backend* walk-away-able (plain `tmux -L grove
attach` still works with the server down), so the web cost is "the rich
front-end stops working," not "the harnesses are stranded." Still a clear TUI
win.

### 2.5 Dependency / complexity budget — **TUI wins; web forces the async refactor**

v1 today: `ratatui` + crossterm, **a sync event loop**, `notify` for fs-watch,
`ureq` (blocking) for HTTP — **zero async dependencies in `Cargo.toml`**. A web
server changes that by construction: every mainstream Rust web framework
(`axum`, `actix`) is `tokio`-async. **Going web forces the sync→async refactor
that 070 exists to defer** — it is not additive, it is a precondition. On top of
async you add WebSocket plumbing, an xterm.js front-end, and a frontend build
toolchain. This dimension and concern 4/070 are the same decision: *web ⇒ async
now*. Clear TUI win on budget; the interlock is a first-class finding (§7).

### 2.6 Multi-repo fleet view *(concern 1)* — **web wins — its strongest dimension**

This is where richer UI pays off **most**. A fleet across many repos wants dense,
sortable, filterable tables; status badges; grouping; click-to-open — all things
a browser does natively and a TUI approximates with effort. If any single feature
justifies web, it is the fleet view. Worth stating plainly so the recommendation
doesn't undersell it: **the fleet view is the strongest pro-web case, and it is a
primary-driver (richness) case.** It is *not*, however, enough on its own to
overturn §2.3–2.5 — a TUI fleet table is very buildable (gh-dash, k9s prove dense
multi-entity TUIs work), it just won't be as rich.

### 2.7 Incrementality / effort — **TUI wins near-term; web is a parallel surface, not an edit**

v1 already ships the TUI, and crucially the **`RepoView` data layer
(`src/repo_view.rs`) is presentation-agnostic** — read-only, returns plain
structs, explicitly factored so a `MultiRepoView` "wraps this type rather than
rewriting it." That layer is reused by *both* presentations. So the fork is
narrow: the TUI path *extends* existing code (fleet view + harness pane are
additive — 050/060 as planned); the web path *adds a whole new surface* (server +
WS + xterm.js + frontend build + async) over the same core. Near-term lift
strongly favours the TUI. The good news for optionality: because the data layer
is already behind a boundary, "web later" stays cheap **if** grove keeps tmux
driving and writes behind that same boundary rather than entangling them with
ratatui.

### Scorecard

| Dimension (weight) | TUI | Web | Favours |
|---|---|---|---|
| **UI richness** *(primary)* | rich panes, modest chrome | rich chrome, same panes | **Web (chrome only)** |
| **Remote access** *(primary)* | tmux/mosh over ssh (mature) | browser-anywhere + auth/TLS/port cost | **Web (dashboard)** |
| Persistence & crash isolation | front-end death = detach | new server = new failure surface | **TUI** |
| Walk-away-ability | one Rust binary | server + browser + build + port | **TUI** |
| Dependency / complexity | sync, no async deps | forces tokio/async (070) + JS build | **TUI** |
| Multi-repo fleet view | buildable, less rich | richest here | **Web** |
| Incrementality / effort | additive to v1 | new parallel surface | **TUI** |

Two heavily-weighted dimensions favour web (one only for the *chrome*, one only
for the *dashboard*); the four "ethos" dimensions — persistence, walk-away,
budget, incrementality — favour the TUI, several decisively. The fleet view is
web's single strongest card.

---

## 3. Failure-mode evidence (prior art)

Surveyed with the same post-mortem discipline as 020: a primary source per
failure-mode claim; silences recorded as "no primary source found." Full notes in
the cluster surveys feeding this doc. The point is not that web terminals don't
work — they demonstrably do, at scale — but that **the failure surface
concentrates in precisely the machinery the web option adds.**

### 3.1 Web terminals over a pty/tmux (ttyd, gotty, wetty, sshx)

**None of the four drives tmux as a first-class integration** — ttyd/gotty spawn
a bare pty and merely *recommend* `tmux new -A` as the command they run
([ttyd man EXAMPLES](https://manpages.ubuntu.com/manpages/jammy/man1/ttyd.1.html);
[gotty README](https://github.com/sorenisanerd/gotty)); wetty wraps `ssh` with no
multiplexer concept; sshx **reimplements** multiplexing in Rust rather than use
tmux ([sshx README](https://github.com/ekzhang/sshx)). Recurring failures:

- **Resize/SIGWINCH is computed per-connection and breaks on non-default fonts
  and multi-client sharing** — ttyd cursor runs off-screen until a manual resize
  ([ttyd #415](https://github.com/tsl0922/ttyd/issues/415)); gotty force-resizes
  an attached tmux to 80×24 and browser resize "has no impact on the underlying
  tmux process" ([gotty #243](https://github.com/yudai/gotty/issues/243)).
- **"Reconnect" rarely means state-preserving** — wetty auto-reconnect duplicates
  the terminal DOM ([wetty #333](https://github.com/butlerx/wetty/issues/333)),
  and on real NIC loss the terminal closes instead of resuming
  ([wetty #447](https://github.com/butlerx/wetty/issues/447)); backgrounding a tab
  kills the transport ([wetty #361](https://github.com/butlerx/wetty/issues/361)).
- **WebSocket hygiene breaks behind proxies** — ttyd emits an RFC-6455-illegal
  1006 close code on non-zero exit, breaking a Gorilla proxy
  ([ttyd #1395](https://github.com/tsl0922/ttyd/issues/1395)).
- **Scrollback / OSC-52 copy-paste:** *no primary source found* across the four —
  treat as unverified, not solved.
- sshx is the architectural outlier (E2E-encrypted relay, Mosh-style predictive
  echo, built-in reconnect) — but it gets there by *re-implementing the
  multiplexer*, and self-hosting is explicitly unsupported.

### 3.2 Web IDEs / remote-dev front-ends (VS Code Server, code-server, Theia, Codespaces, Jupyter)

These prove the browser-terminal-on-a-long-lived-server pattern works at scale —
and quantify its cost. To make a pty "survive reload like a native terminal" VS
Code built, and still bug-fixes, a stack grove would have to match:

- **A separate supervised pty-host process** (since v1.54) with heartbeat /
  unresponsiveness detection and a restart cap
  ([ptyHostService.ts](https://github.com/microsoft/vscode/blob/main/src/vs/platform/terminal/node/ptyHostService.ts));
  restarting it **orphans terminals** with unrecoverable buffers
  ([vscode #117548](https://github.com/microsoft/vscode/issues/117548)).
- **A per-terminal server-side replay buffer** that is *partial by design* — VS
  Code restores only **100** of 1000 scrollback lines
  ([Terminal Advanced docs](https://code.visualstudio.com/docs/terminal/advanced)),
  terminado caps at **1000 messages**
  ([management.py](https://raw.githubusercontent.com/jupyter/terminado/main/terminado/management.py)) —
  and is a direct OOM source: a heavy-output command drove Theia from ~5 GB to
  swap-death until chunking was added
  ([theia #991](https://github.com/eclipse-theia/theia/issues/991),
  [#11449](https://github.com/eclipse-theia/theia/pull/11449)).
- **A reconnection-token + grace-time protocol** (default **3 hours** of held-open
  server state per disconnected client) and its bug class — "Unknown reconnection
  token" / "Cannot reconnect, reload the window"
  ([code-server #6982](https://github.com/coder/code-server/issues/6982),
  [#3018](https://github.com/coder/code-server/issues/3018) "terminal's state is
  lost when the browser tab is reloaded").
- **Reverse-proxy / WebSocket hardening offloaded to the operator and failing
  constantly** — the dominant code-server support category is `1006` WS drops
  across nginx, Cloudflare, HAProxy, Apache
  ([#7167](https://github.com/coder/code-server/issues/7167),
  [#7355](https://github.com/coder/code-server/issues/7355),
  [#6165](https://github.com/coder/code-server/issues/6165)). GitHub's documented
  Codespaces remedy is to **stop using the browser and use the desktop app**
  ([connection troubleshooting](https://docs.github.com/en/codespaces/troubleshooting/troubleshooting-your-connection-to-github-codespaces)) —
  the strongest available signal about this architecture's fragility.

**Bottom line:** the architecture is proven, but "persistence" means
*reattach-to-live-pty on reload with a capped replay buffer*, **not**
restart-durable terminals (no open-source tier delivers that; the prior art's own
advice is "delegate persistence to tmux" — which grove already does via D2). The
cost to reach even reload-survival is a supervised process + bounded replay buffer
(with OOM risk) + a token/grace protocol holding memory + ongoing proxy
firefighting.

### 3.3 Driving tmux from a web backend specifically

Covered in §1.1/§1.2. Headline: **near-empty field**; the only real-tmux web
deployments use bridge (C) and are single-author toys; capable teams (Portal,
WebMux) abandoned tmux. Control mode (A) is greenfield-and-hostile-to-servers
(no handshake, version drift, TTY requirement —
[tmux #3085](https://github.com/tmux/tmux/issues/3085),
[libtmux #633](https://github.com/tmux-python/libtmux/issues/633)).

---

## 4. The recommendation

**TUI now / web later, behind an explicit core↔presentation boundary.**

Reasoning, weighted by the user's own priorities:

1. **The primary drivers are real but narrower than the framing suggests.**
   Richness (§2.1) helps the *dashboard chrome*, not the harness panes (terminal
   emulation in both — and the only realistic web bridge, C, re-derives the TUI's
   rendering model anyway, §1.1). Remote access (§2.2) the *backend already has*
   via `tmux attach` over ssh/mosh in both models; web adds it for the dashboard,
   at an auth/TLS/port/WS cost that §3.1 shows is where web front-ends bleed.
2. **Web's costs land on four of grove's load-bearing values** — persistence
   simplicity (§2.3), walk-away-ability (§2.4), dependency budget (§2.5), and
   near-term effort (§2.7) — several *decisively*, and §3.2 shows the new
   long-lived server is precisely where the failure surface concentrates.
3. **The one dimension where web clearly wins on a primary driver is the fleet
   view (§2.6)** — genuinely richer in a browser. That is the case to revisit web
   *for*, but a TUI fleet table is very buildable (k9s/gh-dash), so it does not by
   itself overturn (2).
4. **"Later" is cheap to keep open** because the v1 data layer is already
   presentation-agnostic (§2.7). The decision is not "TUI forever" — it is "build
   the TUI first, behind a boundary that lets web be a second presentation over
   the same core, not a rewrite."

**Trade-off table (the justification in one view):**

| If grove optimises for… | Choose | Because |
|---|---|---|
| Shipping v2 features now (harness pane, fleet view) on the existing stack | **TUI** | additive; no async refactor; one binary (§2.5, §2.7) |
| Single-binary, walk-away simplicity, minimal failure surface | **TUI** | no second long-lived component (§2.3, §2.4) |
| Casual browser-from-anywhere access to the *dashboard* | Web | backend already ssh-reachable; web adds dashboard reach (§2.2) |
| The richest possible *fleet* and dashboard chrome | Web | DOM beats cells for trees/diffs/tables (§2.1, §2.6) |
| Keeping the web door open at low cost | **TUI now, boundary kept** | data layer already presentation-agnostic (§2.7) |

The balance of grove's stated values and constraints points to **TUI now**; web
is a deferred, deliberately-preserved option, not a rejected one.

---

## 5. What this means for 040 (and 050/060/070)

This comparison **does not silently assume the TUI plan** — it recommends it, and
the binding decision is 040's. Concretely for 040:

- **040 reframes** from "decide tmux integration mechanics (for a TUI)" to
  "**ratify D3 = TUI for v2, and record the core↔presentation boundary** that
  keeps web a viable later presentation." 040's tmux mechanics work (020's Q1–Q4:
  `tmux -L grove -f grove.conf`, plain scripting not `-CC`, `$TMUX` launch
  decision, owned config) **stands unchanged** — it is backend, shared by both
  presentations.
- **The boundary is the new first-class output of 040** — an ADR that says: tmux
  driving + the `RepoView`/`MultiRepoView` data layer + shell-out writes live
  *below* a presentation-agnostic seam; ratatui rendering lives *above* it. This
  is what makes "web later" a second front-end, not a rewrite.
- **050 (harness-pane) and 060 (fleet-view) keep their shape** — they target the
  TUI, building on the existing sync stack. 020's Q5/Q6 (create windows by name,
  detect exit by polling `list-windows`, persistence = survives-dashboard-death-
  not-reboot) feed 050 directly.
- **070 (async-revisit) gains a sharpened finding:** *web ⇒ async now* (§2.5). If
  grove stays TUI, the sync loop very likely suffices (020's polling model needs
  no async); async only becomes mandatory the day a web server is chosen. 070 can
  record "sync suffices for the TUI; async is the entry toll for web" and defer.

If 040 instead chooses "go web," it should treat §1.1's bridge (C) as the default
(reject A and B on the evidence here), and budget for §3.2's persistence
machinery + the async refactor as in-scope, not incremental.

---

## 6. Method & limits

Three independent cluster surveys (web-terminals-over-pty/tmux; web-IDE terminal
persistence; tmux-from-a-web-backend), each required to cite a primary source per
failure-mode claim and to record silences as "no primary source found," following
020's discipline. The v1 stack facts (sync loop, no async deps, presentation-
agnostic `RepoView`) were read directly from `Cargo.toml` and `src/repo_view.rs`.

Known gaps, recorded honestly: **scrollback fidelity and OSC-52 copy-paste** in
the bare web terminals are under-reported (no primary source found) — treated as
unverified, not solved; **prefix-key collision** under bridge (C) follows from
raw-byte relay but has no single tracked issue (no primary source found); the
**effort estimates** in §2.7 are relative-lift judgements, not measured. The
deliverable is a *comparison*, not an implementation — the binding decision is
040's.

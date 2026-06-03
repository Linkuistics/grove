# 090-zellij-fork-framework

**Kind:** planning

> **Gating task.** Decide this *before* building leaves 100–130 (the nav-plugin /
> detail-proxy / whichkey / working-set path). A "fork" outcome could **obsolete
> or reshape most of them** — see Done when.

## Goal

Grill + research whether grove should **fork-and-rebrand zellij into its own
optimised framework**, rather than driving an unmodified zellij from outside
(ADR-0015) and working around its limits with a plugin + a back-channel. The
hypothesis: a targeted fork eliminates whole layers of indirection instead of
patching around them.

What a fork could buy (the user's framing):

- **Pane-id focus targeting.** ADR-0018 made the nav a WASM *plugin* solely
  because a zellij keybind **cannot focus a tiled pane by id** — only
  `LaunchOrFocusPlugin` (by name) can. Add pane-id focus to a fork and **the nav
  may not need to be a plugin at all** (removing the plugin, its build/embed, and
  the whole reason ADR-0018 pulled Strategy 1a forward).
- **A grove-native keybinding / capture channel** — instead of the locked-mode
  pass-through workaround + the (reply-only, dead-end) `cli_pipe_output`
  back-channel (ADR-0019). A first-class control channel.
- **Chrome removal built in** — no `clear-defaults` / bars-stripping config
  gymnastics (leaf 030).
- **Controller-in-zellij.** Host grove's entire [[controlling process]] *inside* a
  rebranded zellij as a special "controller plugin" / built-in — so "controller
  launches zellij as a child" (ADR-0016) collapses into **one framework process**.
  The rebranded zellij *becomes* grove's UI framework. This could also dissolve
  the [[dashboard proxy]] seam (ADR-0016/0017) — rendering is in-process again.

## Context (why now, and what's already known)

- Surfaced while live-testing 080 and investigating the back-channel (ADR-0019):
  `cli_pipe_output` is **reply-only** → considered `run_command` or **forking
  zellij to add `connect()`**. The user **prefers off-installed-zellij anyway**
  (i.e. *vendoring* is acceptable/wanted), which reframes forking as cheap *on top
  of* the vendoring already being signed up for.
- **Evidence gathered this session (record so it isn't re-derived):**
  - zellij's plugin host-call surface is a **protobuf `PluginCommand` enum (~194
    variants)** dispatched through *one* monolithic match in
    `zellij-server/src/plugins/zellij_exports.rs`, gated by
    `check_command_permission()`.
  - Adding a host command (e.g. "write bytes to a unix socket") ≈ **25–40 additive
    lines** across `zellij-utils` (enum + `.proto`) + `zellij_exports.rs` (one
    match arm + handler) + a `zellij_tile` shim fn. **Purely additive → rebases
    cleanly.**
  - Release cadence is **slow: ~2–3 minor releases/year** (0.40 Apr'24 → 0.41
    Nov'24 → 0.42 Mar'25 → 0.43 Aug'25; grove pins 0.44.3). grove pins + bumps on
    its own schedule.
  - The dominant recurring cost is **per-platform builds** (vendoring), *not* the
    patch — and that cost is incurred by going off-installed-zellij regardless.
  - `run_command`-helper vs forked-socket-host-fn share an **identical controller
    side** (control socket + listener + the tested 040 driver); only ~30 lines of
    plugin glue differ — so moving between them later is near-zero waste.
- **Reopens ADR-0015** ("owned but *unmodified* zellij; depend on an installed
  zellij") and the deferred *bundle-zellij* ADR. May **supersede much of
  ADR-0018/0019** (nav plugin, back-channel, run_command path).

## Done when

- A decision is recorded as an **ADR**: *stay on installed zellij* vs *vendor
  unmodified* vs *fork-and-rebrand* — and, if fork, its **scope**: minimal
  (pane-id focus + a control socket, keep the rest) vs deep (controller-in-zellij
  framework, dissolve the proxy seam + the plugin).
- A **build/maintenance model** is sketched: per-platform build pipeline, upstream
  tracking/rebase cadence, licensing/rebrand check (zellij is MIT — confirm),
  naming.
- The tree is **regrown to match**: on a fork outcome, leaves **100–130** are
  reshaped/retired (e.g. nav-as-plugin → nav-as-builtin; back-channel → native
  channel; proxy seam → in-process); on a no-fork outcome, 100–130 proceed as-is.

## Notes

- The user calls it a "diversion" but it is **potentially foundational** — it
  trades a one-time platform-ownership cost for deleting the plugin/seam/back-
  channel indirection that ADR-0018/0019 spend their complexity on.
- Weigh against grove's spine: constraint 6 (**walk-away-able** — a fork breaks
  "delete grove and stock zellij still works"; the user accepts this) and
  constraint 4 (**smallest thing that earns its place** — but here the "bigger"
  thing may be *simpler* overall by removing layers).
- Decide **sequencing**: do the fork eval/work before 100-130 (no throwaway, but
  front-loads heavy vendoring), or ship 100-130 on `run_command` first as a
  working A′ baseline and migrate later (≈30 lines throwaway). This planning task
  picks the path.
- Carry forward: [[zellij substrate]], [[nav plugin]], [[dashboard proxy]],
  [[leader]], and ADRs 0013–0019 are the inputs to grill against.

## Decisions (running log)

### D-fork-1 — Deep hard fork: grove becomes a based-on-zellij codebase
*Settled (user, this session).* Outcome on the three Done-when axes:
- **Off-installed: yes** — grove ships its own zellij-derived build; supersedes
  ADR-0015's "depend on an installed zellij."
- **Fork, deep scope: yes** — grove's logic is compiled in **natively** (not a
  WASM plugin, not an external `zellij action` driver). The [[controlling
  process]] dissolves *into* the process; rendering is in-process, so the
  [[dashboard proxy]] seam (ADR-0016), the [[nav plugin]] WASM artifact
  (ADR-0018), and the `cli_pipe_output` back-channel (ADR-0019) all evaporate.
- **Hard fork, not a tracking fork** — take zellij as a one-time gift of a
  working multiplexer, then own it. **No rebase cadence, no adopting upstream
  code.** Upstream is watched only as a **CVE/advisory information feed**;
  relevant security fixes are assessed and hand-patched into our tree. Behavioural
  upstream fixes/features are ours to notice and hand-port if wanted — accepted
  cost (user: "not even that concerned about tracking upstream").
- *Rationale:* the dominant recurring cost is per-platform builds (vendoring),
  incurred the moment we go off-installed regardless of patching; the patch on top
  is cheap; a hard fork zeroes the rebase tax that a deep fork would otherwise
  make enormous. Breaks constraint 6 (walk-away → stock zellij) — user accepts.
- *Supersedes:* ADR-0015 (installed/unmodified). *Largely supersedes:* ADR-0016
  (proxy seam), ADR-0018 (nav-as-plugin), ADR-0019 (back-channel + plugin nav
  realisation). The *UX model* those ADRs settled (grove = tab, [[working set]],
  home = nav, [[whichkey bar]], detail per-grove) is expected to survive; only its
  *realisation* moves from plugin/proxy to native in-process.

### D-fork-2 — Two products: a reusable framework + grove as a consumer
*Settled (user, this session).* The deep fork is **modularised** so the
non-grove parts are extractable and **publishable as a reusable framework** (for
others, and for the user's own future projects). Two products:
- **The framework** — the based-on-zellij codebase, generalised: embed a native
  Rust app as a first-class in-process controller of a multiplexer (render
  in-process; drive tabs/panes/layout/focus + receive input/events natively;
  inherit zellij's terminal emulation of other panes, copy/scrollback/search,
  session persistence, web client). This is the publishable, novel artifact — the
  "build a native TUI app *on top of* a multiplexer" niche none of zellij's three
  extension paths (WASM plugin / external `zellij action` / source hack) fills.
- **grove** — one consumer, holding all grove-specific logic + the nav/detail/
  working-set UX.
- *Implication:* a real **framework↔consumer seam** is mandatory — this is the
  product/packaging-level successor to ADR-0013's in-module presentation boundary.
  Repo/crate strategy, the extension-API shape, naming, and license/attribution
  (zellij is MIT) are the open mechanical questions that follow.

### D-fork-3 — Framework identity: a TUI framework for seamless TUI-app embedding
*Settled (user, this session).* Product #1 is **not** "a multiplexer you can
extend." It is **a TUI application framework whose headline, differentiating
concern is seamlessly embedding *other* TUI apps** (vim, lazygit, yazi, a shell,
claude) as first-class fully-emulated regions of your own TUI app — correct focus,
input routing, copy mode, resize, cursor — without hand-rolling a terminal
emulator and without zellij's existing extension limits (WASM sandbox / external
`zellij action` round-trips / source hacking). zellij's pane + emulation engine is
the machinery under the hood; "embed other TUI apps seamlessly" is the product.
- *Arc:* this is the synthesis of grove's own history — the embed mission of leaf
  010 / ADR-0014 ([[harness-pane crate]]: `tui-term`+`portable-pty`+`vt100`,
  shelved because owning emulator fidelity is a tar pit) **+** zellij's mature
  emulation **+** native in-process control (which neither the embed nor the
  external-zellij path had). The fork keeps 010's goal and swaps the hand-rolled
  emulator for zellij's.
- *Note (altitude):* the exact hosting-API shape (library-you-link `run(app)` vs
  runtime-you-plug-into) is **deferred to the first framework build leaf** —
  build-discovered against the real zellij internals, per this tree's own
  precedent (leaf 080 surfaced the `cli_pipe_output` constraint empirically). The
  ADR records intent (publishable, library-idiomatic preferred), not the trait
  design.

### D-fork-4 — Access the full system model; GraphQL only at a network boundary
*Settled (user, this session — corrected from first framing).* The goal is that
consumers can **access the full system model, not just a fixed menu of exposed
functions**. How that is delivered depends on the boundary:
- **Native consumer boundary (the framework's primary, core path).** A consumer
  that *links* the framework (grove, and any app built on it) gets the full system
  model as **actual typed Rust values + calls, in-process** — no serialization, no
  query language. This is *strictly more* than GraphQL could give and is the
  realisation of "full model, not just exposed functions" for the native case.
  "You don't serialize what you can link."
- **Network / out-of-process boundary (conditional, lazy — constraint 4).** Only
  *if/when* the framework exposes remote or out-of-process control (web client,
  external automation, an out-of-process AI agent) does a wire format matter — and
  **there GraphQL-over-the-model beats protobuf RPC** (query/traverse/mutate/
  subscribe the whole model vs. a fixed-function `PluginCommand` enum). zellij
  *needs* protobuf because its plugins are out-of-process WASM and its CLI is a
  separate process; a link-in framework may have **no such boundary in its core**.
- *Net:* GraphQL is **not** part of the framework core and **grove-as-native-
  consumer never touches it**. It is the chosen wire *if* a network surface is
  later added. Transport (socket/HTTP, subscriptions over websockets) is a build
  detail. The protobuf→GraphQL "switch" is therefore optional, not foundational.

### D-fork-6 — Sequencing/structure: grove-first, extract the framework later
*Settled (user, this session).* Of {framework-first in its own repo/grove} vs
{grove-first, extract later}, the choice is **grove-first, extract-later**:
- Hard-fork zellij into a **new crate in grove's cargo workspace**, with a **strict
  one-way dependency** (framework crate must not depend on the grove crate — the
  compiler enforces the seam, keeping grove-isms out of the framework).
- Build **only the native-host API grove actually needs** to land the harness pane
  + nav + detail + working set. **Defer GraphQL, the network boundary, and the full
  observability API** until a second consumer or a real publish intent earns them —
  the expensive, API-freezing parts are designed by a real consumer first, not in
  the abstract.
- **Extract to its own repo + its own grove when the API has stabilised** (the
  user's word: "modularise so we can *extract*"). Per-platform build pipeline and
  the extraction are **later/lazy leaves**, not front-loaded.
- *Rationale:* avoids freezing a framework boundary whose shape we haven't learned;
  delivers grove value without front-loading whole-codebase fork bring-up + a
  published API; honours grove constraint 4 (smallest thing that earns its place).
  Cautionary precedent in this very tree: leaf 080 *built* a back-channel before
  ADR-0019 discovered empirically it couldn't work — design-by-real-consumer beats
  design-in-the-abstract.

### D-fork-7 — Remaining mechanicals (recommended; correct if wrong)
*Proposed (LLM), driving per "continue".* Low-stakes/reversible, recorded so the
ADR can cite them; flagged for user veto:
- **Naming:** provisional working crate name **`trellis`** (a trellis is a
  framework other things grow *on* — fits the grove/garden metaphor and the
  "framework you build a TUI app on" identity). **Provisional** — the *public*
  brand is a decision deferred to the extraction leaf (constraint 4); nothing
  downstream depends on the final name.
- **License/attribution:** zellij is **MIT** (confirmed at fork time by reading its
  `LICENSE`). MIT permits fork + rebrand + relicensing-of-our-additions freely,
  provided zellij's copyright notice + MIT text are preserved. No copyleft
  exposure. The bring-up leaf carries the attribution forward
  (`LICENSES/zellij.LICENSE` or equivalent).
- **Binary model:** **one unified binary** — native link, app owns `main` (library-
  idiomatic intent); the grove CLI + framework + mux are one artifact. Detail
  deferred to bring-up.
- **Build/maintenance:** per-platform CI matrix (macOS arm64/x64, Linux x64/arm64)
  extending the existing manual `scripts/release-*.sh`; distribute via the existing
  brew tap; binary size grows (ships a multiplexer) — accepted. **Upstream
  tracking: none** (hard fork). **CVE-watch only:** RustSec advisories + GitHub
  security advisories on the zellij repo + its dependency tree; relevant fixes
  assessed and hand-patched, never rebased. Chrome/branding stripped in-source as
  we own it (the old bundled-config `clear-defaults`/bars gymnastics become
  unnecessary — set our defaults in the fork).

### D-fork-5 — First-class observability of wrapped tools
*Settled (user, this session).* A headline framework facility: **observe and
reason about what the embedded ("wrapped") TUI tools are doing, and interact with
them programmatically.** Embedded apps are not opaque terminal grids — the
framework exposes their observable state (screen contents, cursor, title/OSC,
scrollback, exit status, mouse/mode requests) **as part of the GraphQL system
model** (D-fork-4) and offers a programmatic channel to drive them (inject input,
read output).
- *Why it fits the identity:* this is the introspection half of "seamlessly embed
  other TUI apps" (D-fork-3) — and it is **grove's own need generalised**: grove
  drives AI harnesses and wants to watch/reason about them. The framework turns
  that into a first-class, reusable capability, and an unusually strong **AI-era
  pitch**: a TUI workspace whose embedded tools are introspectable + drivable by
  agents via GraphQL.

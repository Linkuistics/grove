# 20. Fork zellij into grove's own TUI framework (deep hard fork, extract-later)

- Status: **superseded** (was accepted) — Superseded by
  [ADR-0031](0031-shed-machinery-keep-self-extension-core-and-methodology.md) (grove
  sheds its machinery to a self-extension core) and
  [ADR-0032](0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md) (the loop
  substrate is a self-driving shell loop). The rmux/ratatui TUI + Fleet tower this ADR
  belongs to is **deleted** in leaf `080-shed-tui`; its runtime lives only in git
  history. The decision is retained here as record. (Prior status: superseded by
  [ADR-0028](0028-rmux-substrate.md), rmux substrate, 2026-06-10, 070-teardown D4 — the
  deep zellij hard-fork, trellis, was **retired entirely**; grove depended on stock
  published rmux 0.5.0, no fork, and `crates/trellis/` was deleted. This is the fork the
  grove existed to retire. Premise + mechanism gone.)
- Date: 2026-06-03
- Deciders: Antony Blakey (with grove 060/040/090 planning)
- Supersedes: ADR-0015 (owned but **unmodified, installed** zellij)
- Largely supersedes: ADR-0016 (dumb-terminal proxy seam), ADR-0018 (nav as a
  WASM plugin), ADR-0019 (nav-opens-via-plugin-API + dumb per-grove detail proxies
  + the deleted back-channel). **The UX model those ADRs settled survives** (grove
  = tab/workspace, working set, home = nav, whichkey bar, detail per-grove); only
  their **plugin / proxy / back-channel realisations** are replaced by native
  in-process code.
- Builds on: ADR-0013 (presentation boundary — reinforced and promoted from an
  in-module seam to a product/packaging seam) and ADR-0014 (the harness-pane
  embed — its *embedding mission* is fulfilled here, by a different mechanism).

## Context

The 060 substrate path is a chain of workarounds for one root condition: **grove
drives an unmodified zellij from *outside* it.**

- **010 / ADR-0014** built an in-process pty embed (`tui-term`+`portable-pty`+
  `vt100`) to render an embedded TUI app inside grove's ratatui — shelved because
  owning a terminal emulator's fidelity edge-cases is a tar pit.
- **ADR-0015** adopted an owned-but-**unmodified, installed** zellij, driven from
  outside via `zellij action`.
- **ADR-0016** had to introduce a dumb-terminal **proxy seam** (a unix socket
  shipping rendered frames) *because* grove's controller is a process *outside*
  the zellij pane that displays it.
- **ADR-0018** had to make the nav a **WASM plugin** *because* a zellij keybind
  cannot focus a tiled pane by id — only `LaunchOrFocusPlugin` (by name) can.
- **ADR-0019** had to **delete a back-channel** it had just built, *because*
  `cli_pipe_output` is reply-only and cannot push to the controller from a
  keypress.

Each ADR added an indirection layer to compensate for being outside the
multiplexer: a plugin, a proxy seam, a reply-only channel, chrome-stripping config
gymnastics, a pre-seeded `permissions.kdl`. The leaf-090 grilling asked whether to
stop patching around the boundary and **move grove inside it.**

**Evidence gathered before the decision** (recorded in leaf `090`'s running log so
it is not re-derived):

- zellij's plugin host-call surface is a **protobuf `PluginCommand` enum (~194
  variants)** through one monolithic match; adding a host command is **~25–40
  additive lines** that rebase cleanly.
- zellij release cadence is **slow (~2–3 minor/year)**; grove pins and bumps on
  its own schedule.
- The dominant recurring cost of going off-installed is **per-platform builds
  (vendoring)** — **incurred the moment we go off-installed, with or without a
  patch.** The patch on top is comparatively free.
- The user **prefers off-installed-zellij anyway**, which reframes forking as
  cheap *on top of* vendoring already being signed up for.

The reframe (grilled with the user, leaf 090): rather than vendor an *unmodified*
zellij and keep patching around it, make grove a **based-on-zellij codebase** and
host grove's logic **natively, in-process** — and **modularise** so the non-grove
parts become a **publishable framework**.

## Decision

**Fork zellij into grove's own TUI framework — a deep, hard, extract-later fork.**

1. **Off-installed.** grove ships its **own zellij-derived build** (supersedes
   ADR-0015's "depend on an installed zellij").

2. **Deep fork.** grove's TUI logic is compiled in **natively** — *not* a WASM
   plugin, *not* external `zellij action` driving. The [[controlling process]]
   dissolves *into* the process; rendering is **in-process**, so the
   **[[dashboard proxy]] seam (ADR-0016), the WASM [[nav plugin]] (ADR-0018), and
   the `cli_pipe_output` back-channel (ADR-0019) all evaporate.**

3. **Hard fork, not a tracking fork.** Take zellij as a one-time gift of a working
   multiplexer, then **own it. No rebase cadence, no adopting upstream code.**
   Upstream is watched **only as a CVE/advisory information feed** (read-only);
   relevant security fixes are assessed and **hand-patched**, never rebased.
   Behavioural upstream fixes/features are ours to notice and hand-port if wanted —
   accepted.

4. **Two products.** A reusable, **publishable TUI framework** (provisional working
   name **`trellis`**) whose **headline, differentiating concern is seamlessly
   embedding *other* TUI apps** (vim, lazygit, yazi, a shell, claude) as
   first-class fully-emulated regions of your own TUI app — using zellij's mature
   emulation, not a hand-rolled one — **plus grove as its first consumer.** A
   **strict one-way crate seam** (framework must not depend on grove) keeps the
   boundary honest; it is the product-level successor to ADR-0013's presentation
   boundary.

5. **Full-model access, GraphQL only at a network boundary.** Native consumers
   *link* the framework and get the **whole system model as typed Rust values +
   calls, in-process** — strictly more than a query language could give. A wire
   format (and there, **GraphQL-over-the-model** beats protobuf RPC) matters **only
   if/when** an *out-of-process / network* surface is added (web client, remote
   automation, out-of-process agents). GraphQL is **not** in the framework core and
   grove-as-native-consumer never touches it (lazy, constraint 4).

6. **First-class observability of wrapped tools.** Observing and **programmatically
   driving** embedded ("wrapped") tools — their screen, cursor, title/OSC,
   scrollback, exit status, mode requests, plus input injection — is a framework
   facility, exposed as part of the system model. This is grove's harness-watching
   need generalised, and a strong AI-era pitch (embedded tools introspectable +
   drivable by agents).

7. **Sequencing: grove-first, extract-later.** Hard-fork zellij into a **new crate
   in grove's cargo workspace**; build **only the native-host API grove needs**;
   **defer** GraphQL, the network boundary, the full observability API, the
   per-platform build pipeline, and the **extraction into the framework's own
   repo/grove** to later lazy leaves — designed by a real consumer first, not in
   the abstract.

ADR-0013's boundary is **reinforced and promoted**: ratatui/presentation stays
above; `RepoView`/`MultiRepoView` + writes stay below; and the framework↔grove line
becomes a real crate (eventually repo) seam.

## Consequences

- **Reused from the built 010–080 code (portable):** the `RepoView`/
  `MultiRepoView` data layer, fs-watch/`notify`, shell-out-to-`grove` writes, the
  v1 dashboard + per-grove detail **ratatui rendering**, and the nav / working-set
  **UX model**. The logic survives; only its transport changes.
- **Superseded mechanism (retained in `done/` + git history):** the proxy seam
  (010/020), external zellij-launch + `zellij action` driving (030/040), the WASM
  nav plugin + controller↔plugin pipe (070/080), the bundled-config chrome
  gymnastics (030), and the planned `permissions.kdl` pre-seed (old 100). Native
  in-process control replaces all of them — no socket frames, no WASM, no
  reply-only channel, no permission grant dance.
- **Walk-away-ability (constraint 6) breaks** for the TUI: deleting grove no longer
  leaves a stock zellij behind. **User accepts.** (The CLI/methodology core remains
  walk-away — durable artifacts are still standard markdown.)
- **New costs accepted:** owning a ~100k-LOC multiplexer's correctness surface
  (terminal emulation, PTY, client/server IPC, plugin host); a **per-platform
  build pipeline** (CI matrix; extends the manual `scripts/release-*.sh`; ships via
  the brew tap); larger binary; and a standing **CVE-watch process** (RustSec +
  GitHub security advisories on zellij and its dependency tree).
- **The harness-pane crate (ADR-0014) embed** stays shelved history: its *mission*
  (embed TUI apps) is now the framework's headline, but the framework uses zellij's
  emulator, not `tui-term`/`vt100`.
- **Licensing:** zellij is **MIT** (confirmed at fork time by reading its
  `LICENSE`). MIT permits fork + rebrand + relicensing-of-our-additions, provided
  zellij's copyright notice + MIT text are preserved (`LICENSES/zellij.LICENSE`).
- **Tree regrown** (leaf 090): 100–130 reshaped to the fork path (fork bring-up →
  native-host API + native dashboard → native nav → native detail → native
  whichkey → working set); the per-platform pipeline and the framework extraction
  are added as later lazy leaves.

## Notes

- The **hosting-API shape** (library-you-link `run(app)` vs runtime-you-plug-into)
  is **build-discovered** in the bring-up leaf, against the real zellij internals —
  per this tree's precedent (leaf 080 surfaced the `cli_pipe_output` constraint
  empirically). The ADR records *intent* (publishable, library-idiomatic
  preferred), not the trait design.
- The **public brand** for the framework is deferred to the extraction leaf;
  `trellis` is a provisional working name only.
- The full grilling trail (D-fork-1…7) lives in leaf `090`'s running decision log
  (retired to `.grove/done/`); this ADR is its durable distillation.
- **Do not re-litigate** the superseded indirection (proxy seam, WASM nav,
  back-channel) on the unmodified-zellij premise — that premise is gone. tmux stays
  retired (ADR-0014).

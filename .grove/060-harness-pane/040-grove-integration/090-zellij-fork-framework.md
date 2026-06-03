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

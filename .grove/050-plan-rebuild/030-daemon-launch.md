# 030-daemon-launch

**Kind:** planning

## Goal

Make `grove tui` a **walk-away binary** on the **stock** rmux daemon: settle how
the daemon binary is bundled/shipped, how `grove tui` launches and connects to it,
session naming/persistence, and how the fleet singleton + multi-repo model
(ADR-0025/0027) realises under rmux. Grow work leaves for bundling + launch.

## Context

030-engine connects via `Rmux::connect_or_start` against **published rmux 0.5.0**
(SDK crates `rmux-sdk`/`ratatui-rmux` + the stock `rmux` daemon+CLI binary). For a
user who has not installed rmux, `connect_or_start` needs a daemon binary to spawn
— so grove must ship one.

**No fork (ADR-0029).** The original D7 plan to ship grove's *forked* rmux build is
dead — rendered-history capture already ships in stock rmux 0.5.0. grove bundles
the **stock published daemon+CLI**, with no patch to rebase and no forked build in
the release pipeline. This corrects the pre-decomposition leaf text.

## Areas to grill (questions, not answers)

- **Bundling the daemon.** Vendor the stock `rmux` binary into grove's
  distribution and point `connect_or_start` at it via `SDK_DAEMON_BINARY_ENV`?
  How does this interact with the manual release pipeline
  (`scripts/release-{doctor,build,publish}.sh`)? Per-platform binaries?
- **Version pinning.** grove pins `rmux-sdk`/`ratatui-rmux` crate versions *and*
  the bundled daemon+CLI binary version — they must match. How is the match
  enforced/verified at build time? (open-in-editor also shells out to the stock
  `rmux capture-pane` CLI — ADR-0029 — so the CLI binary is shipped too.)
- **Session naming + persistence.** ADR-0027 made the fleet session a singleton
  (`grove-fleet`, a constant since there is no cwd anchor). Under rmux: one
  detached session reused across `grove tui` restarts (`ensure_session`
  `CreateOrReuse`)? Does session persistence give detach/reattach for free (ties
  to the deferred `rmux-web` grove — flag, don't solve)?
- **Launch flow.** What `grove tui` does end-to-end: resolve fleet from config →
  `connect_or_start` (spawn bundled daemon if none) → `ensure_session` → run the
  draw loop. First-run UX when no daemon is running.
- **Fleet + multi-repo under rmux.** How `MultiRepoView` groves map onto rmux
  panes/sessions across repos (ADR-0025 manifest+scan, ADR-0027 no-cwd-anchor)
  — mostly survives below the seam; confirm nothing rmux-specific breaks it.

## Done when

The bundling, launch flow, session/persistence model, and version-pinning approach
are settled; work leaves are grown; any ADR-0027 amendment under rmux is recorded.

## Notes

//! The rmux-backed `grove tui` — a ratatui app that owns its own draw loop and
//! embeds harness/tool panes via `ratatui-rmux`, driving them with the async
//! `rmux-sdk` against an rmux daemon that owns the ptys (grove `rmux-substrate`,
//! 030-engine; landmark ADR-0028).
//!
//! ## The presentation boundary is this directory (E2 / ADR-0013)
//!
//! Code under `src/tui/` MAY import `ratatui`, `ratatui_rmux`, `rmux_sdk`, and
//! `tokio`; code *outside* it MUST NOT. That makes ADR-0013's "boundary enforced
//! by module placement and review" a literal directory wall. The sync core
//! (`RepoView`, launch, fleet resolution) is called directly from async context
//! (E1) — it is fast, local fs/git — and never imports tokio.
//!
//! Submodules (capture's real surface arrives in 040):
//!   - [`app`] — the async draw loop (`tokio::select!` over render/input/watch).
//!   - [`driver`] — per-pane rmux glue: the D3 push `render_stream` task.
//!   - [`pane`] — the headless-testable render path (snapshot → buffer + cursor).
//!   - [`focus`] — the leader-gated `Harness | Nav | Modal` arbitration (E4).
//!   - [`input`] — the crossterm → tmux key-map.
//!   - [`config`] — the configurable leader key.
//!   - [`nav`] — the minimal grove-list surface: list → open/focus a harness.

pub mod app;
pub mod config;
pub mod driver;
pub mod focus;
pub mod input;
pub mod nav;
pub mod pane;

use crate::cli::TuiArgs;

/// Sync entry for `grove tui`, called from `cli.rs`. Builds the multi-threaded
/// tokio runtime (E1 — the async firewall lives behind this call) and drives the
/// async app to completion. The rest of the `grove` binary stays synchronous.
pub fn run(args: &TuiArgs) -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| anyhow::anyhow!("building tokio runtime: {e}"))?;
    runtime.block_on(app::run_app(args))
}

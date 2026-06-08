//! Per-pane rmux glue: the D3 *push* render path.
//!
//! [`PaneDriver`] holds one embedded rmux pane handle plus its stable
//! [`PaneId`] (E3 — panes are addressed by id, not positional slot), and
//! [`PaneDriver::spawn_render_task`] opens the pane's `render_stream` and
//! forwards each fresh [`PaneSnapshot`] — **tagged with the pane's key** — onto a
//! channel the app's `tokio::select!` drains. This replaces the spike's
//! pull-per-frame `pane.snapshot()` loop with the event-driven push model
//! (010-plan D3): the loop redraws only when a snapshot actually arrives.
//!
//! The key tag is what makes 030's multi-pane nav work: every open grove's pane
//! pushes onto the *same* channel, and the app routes each snapshot to the right
//! pane's `PaneState` by key, redrawing only when the *focused* pane changed.
//!
//! Note: `ratatui-rmux` ships its own `PaneDriver` (a *pull* helper whose
//! `refresh()` captures on demand). grove's is the push variant; the two are
//! kept apart by module rather than imported side by side.

use rmux_sdk::{Pane, PaneId, PaneSnapshot};
use tokio::sync::mpsc;

/// One embedded rmux pane and its stable identity.
pub struct PaneDriver {
    pane: Pane,
    id: PaneId,
}

impl PaneDriver {
    /// Wrap a resolved pane handle and its daemon-assigned [`PaneId`].
    pub fn new(pane: Pane, id: PaneId) -> Self {
        Self { pane, id }
    }

    /// The pane handle, for input/resize forwarding.
    pub fn pane(&self) -> &Pane {
        &self.pane
    }

    /// The stable pane id this driver addresses.
    pub fn id(&self) -> PaneId {
        self.id
    }

    /// Open the pane's render stream and spawn a task forwarding every fresh
    /// snapshot — tagged with `key` — onto the shared `tx`. The task ends when
    /// the stream closes (the pane's process exited) or all receivers are gone
    /// (app shutdown). Because every pane shares one channel, a single pane
    /// exiting does **not** close it — the app keeps running with the other
    /// harnesses, and quit is the leader's job (E4), not a side effect of one
    /// process ending.
    pub async fn spawn_render_task(
        &self,
        key: String,
        tx: mpsc::UnboundedSender<(String, PaneSnapshot)>,
    ) -> rmux_sdk::Result<()> {
        let mut stream = self.pane.render_stream().await?;
        tokio::spawn(async move {
            // Ends on `Ok(None)`/`Err` (pane exited or stream errored); the
            // inner break covers every app-side receiver being dropped.
            while let Ok(Some(update)) = stream.next().await {
                if tx.send((key.clone(), update.into_snapshot())).is_err() {
                    break;
                }
            }
        });
        Ok(())
    }
}

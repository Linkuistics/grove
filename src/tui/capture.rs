//! The **capture modal** (040): a centered focus-overlay drawn over the live
//! harness pane, the proof point for the bug that motivated the whole migration.
//!
//! ## Why this is the landmark proof point
//!
//! Under the trellis zellij-fork, grove's own UI lived *inside* zellij's
//! pane/buffer model, so a centered popup over a live pane was the capture-popup
//! bug — structurally hard. Under rmux grove owns the draw loop, so the modal is
//! just `Clear` over a centered [`Rect`] plus a bordered widget, painted *after*
//! the [`PaneWidget`](crate::tui::pane) each frame while
//! [`Focus::Modal`](crate::tui::focus::Focus::Modal) is up. The pane shows
//! through everywhere the popup does not cover (the overlay-over-pane test below
//! demonstrates exactly that — headlessly, into a [`Buffer`]).
//!
//! ## Split of responsibilities
//!
//! - [`CaptureModal`] owns the **text buffer** and the **pure render** (centered
//!   box + cursor). The focus machine ([`crate::tui::focus`]) drives the buffer
//!   via `ModalInsert`/`ModalBackspace`; the app calls [`CaptureModal::render`]
//!   inside `terminal.draw`.
//! - [`write_capture`] performs grove's **capture write** by shelling out to
//!   `grove-llm inbox-add` — the same idiom every other grove capture uses (E1's
//!   "shell-out writes"), not a new in-process path. It commits + best-effort
//!   pushes, so the app calls it under `spawn_blocking`.
//! - [`CaptureOutcome`] + [`render_toast`] surface success/failure *briefly*
//!   after submit, when focus has already returned to the prior surface.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap};

/// Where a submitted observation is written: the addressed grove and the repo
/// whose `.grove-meta/` inbox carries it. Resolved from the **focused** pane —
/// capturing over a grove's harness leaves a note for that grove's next session
/// (the canonical inbox use). `None` for the bare-shell fallback pane, which has
/// no grove to address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureTarget {
    /// The grove name — the `--to` argument (existing, future, or finished).
    pub name: String,
    /// The owning repo root — the `--repo` argument (supports cross-repo).
    pub repo_root: PathBuf,
}

/// The capture modal's editable state: just the text buffer (the focus machine
/// owns the *transitions*; this owns the *content* and how it paints).
#[derive(Debug, Default)]
pub struct CaptureModal {
    buffer: String,
}

impl CaptureModal {
    /// A fresh, empty modal.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append literal text (a typed char, or a pasted run — paste lands in the
    /// buffer verbatim per E5, since there is no pane to forward to while up).
    pub fn insert(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    /// Delete the last character (Backspace).
    pub fn backspace(&mut self) {
        self.buffer.pop();
    }

    /// Discard the buffer (Esc cancel).
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Take the buffer out, leaving it empty (Enter submit hands the body to the
    /// capture write and the modal closes empty).
    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.buffer)
    }

    /// Paint the centered modal into `buf` over `area`, on top of whatever was
    /// already drawn there (the live pane). `target_label` names the grove the
    /// observation will be addressed to. Returns where the hardware cursor
    /// belongs (just past the buffer text), or `None` when the box is too small.
    ///
    /// Pure — no daemon, no terminal — so the overlay-over-pane path is a
    /// headless unit test (the migration's whole testability win).
    pub fn render(&self, area: Rect, buf: &mut Buffer, target_label: &str) -> Option<(u16, u16)> {
        let popup = centered_rect(70, 50, area);
        // Clear *only* the popup rect, so the pane shows through everywhere else.
        Clear.render(popup, buf);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" capture \u{2192} {target_label}  (Enter: save \u{b7} Esc: cancel) "))
            .border_style(Style::default().fg(Color::Yellow));
        let inner = block.inner(popup);
        block.render(popup, buf);
        if inner.width == 0 || inner.height == 0 {
            return None;
        }
        Paragraph::new(self.buffer.as_str())
            .wrap(Wrap { trim: false })
            .render(inner, buf);
        // Place the cursor just past the buffer text, wrapping across the box.
        let len = self.buffer.chars().count() as u16;
        let cx = inner.x + (len % inner.width);
        let cy = inner.y + (len / inner.width).min(inner.height - 1);
        Some((cx, cy))
    }
}

/// Paint the **seed-start confirm** modal (070): a small centered y/n prompt over
/// the nav, naming the seed and spelling out the side effect (`grove do <name>`
/// creates the worktree + branch). Pure — like the capture modal, it is a
/// headless `Clear` + bordered widget over `area`, drawn while
/// [`ModalKind::Confirm`](crate::tui::focus::ModalKind::Confirm) is up. No cursor
/// (it has no text buffer — the answer is a single keypress).
pub fn render_confirm(name: &str, area: Rect, buf: &mut Buffer) {
    let popup = centered_rect(60, 30, area);
    Clear.render(popup, buf);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" start grove ")
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(popup);
    block.render(popup, buf);
    if inner.width == 0 || inner.height == 0 {
        return;
    }
    let body = format!(
        "start grove {name}?\ncreates worktree + branch\n\ny start \u{b7} n cancel"
    );
    Paragraph::new(body)
        .wrap(Wrap { trim: false })
        .render(inner, buf);
}

/// The result of a TUI action that shells out, surfaced briefly as a toast after
/// the action completes (the app clears it on the next keypress). Covers capture
/// (the modal submit) and the 040 inbox **grooming** actions (reject / move).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureOutcome {
    /// A capture was written to the named grove's inbox.
    Captured(String),
    /// The selected observation was rejected (dropped from the inbox).
    Rejected,
    /// The selected observation was moved/re-routed to the named grove.
    Moved(String),
    /// The action failed; carries a one-line reason (the caller prefixes context,
    /// e.g. `"capture failed: …"` / `"move failed: …"`).
    Failed(String),
}

/// Paint a one-line status toast on the bottom row of `area`, over the current
/// surface. Green for success, red for failure; cleared by the app on the next
/// keypress so it shows "briefly".
pub fn render_toast(outcome: &CaptureOutcome, area: Rect, buf: &mut Buffer) {
    if area.height == 0 || area.width == 0 {
        return;
    }
    let line = Rect::new(area.x, area.y + area.height - 1, area.width, 1);
    Clear.render(line, buf);
    let (text, bg) = match outcome {
        CaptureOutcome::Captured(name) => (format!(" \u{2713} captured to {name} "), Color::Green),
        CaptureOutcome::Rejected => (" \u{2713} observation rejected ".to_string(), Color::Green),
        CaptureOutcome::Moved(name) => (format!(" \u{2713} moved to {name} "), Color::Green),
        CaptureOutcome::Failed(msg) => (format!(" \u{2717} {msg} "), Color::Red),
    };
    Paragraph::new(text)
        .style(Style::default().fg(Color::Black).bg(bg))
        .render(line, buf);
}

/// Perform grove's capture write: shell out to `grove-llm inbox-add` (the same
/// idiom every grove capture uses — don't invent a new one). Synchronous and
/// blocking (it commits + best-effort pushes), so the caller runs it under
/// `spawn_blocking` to keep the reactor free (E1).
pub fn write_capture(grove_exe: &str, target: &CaptureTarget, body: &str) -> Result<()> {
    let bin = grove_llm_path(grove_exe);
    let output = Command::new(&bin)
        .arg("inbox-add")
        .arg("--to")
        .arg(&target.name)
        .arg("--repo")
        .arg(&target.repo_root)
        .arg("--body")
        .arg(body)
        .output()
        .with_context(|| format!("running {} inbox-add", bin.display()))?;
    check_status(output, "inbox-add failed")
}

/// Reject the focused grove's selected observation (040 grooming): shell out to
/// `grove-llm inbox-drain --rejected <obs>`, which deletes the one file in a
/// commit (and best-effort pushes). Synchronous + blocking like the other writes,
/// so the app runs it under `spawn_blocking` (E1).
pub fn reject_observation(grove_exe: &str, target: &CaptureTarget, obs: &Path) -> Result<()> {
    let bin = grove_llm_path(grove_exe);
    let output = Command::new(&bin)
        .arg("inbox-drain")
        .arg("--for")
        .arg(&target.name)
        .arg("--repo")
        .arg(&target.repo_root)
        .arg("--rejected")
        .arg(obs)
        .output()
        .with_context(|| format!("running {} inbox-drain", bin.display()))?;
    check_status(output, "inbox-drain failed")
}

/// Move/re-route the selected observation from `source` to `dest` (040 grooming):
/// **copy first** — `inbox-add --to <dest> --body-file <obs>` re-captures the
/// note's body into the destination grove — then, only once that lands, **drop**
/// the original from the source inbox via [`reject_observation`]. The copy-before-
/// drop order is deliberate: a failed copy leaves the observation untouched (no
/// data loss); a failed drop after a successful copy surfaces as an error toast
/// (the note is duplicated, which the user can then groom away).
pub fn move_observation(
    grove_exe: &str,
    source: &CaptureTarget,
    dest: &CaptureTarget,
    obs: &Path,
) -> Result<()> {
    let bin = grove_llm_path(grove_exe);
    let output = Command::new(&bin)
        .arg("inbox-add")
        .arg("--to")
        .arg(&dest.name)
        .arg("--repo")
        .arg(&dest.repo_root)
        .arg("--body-file")
        .arg(obs)
        .output()
        .with_context(|| format!("running {} inbox-add", bin.display()))?;
    check_status(output, "inbox-add failed")?;
    reject_observation(grove_exe, source, obs)
}

/// Turn a finished child process into a `Result`: `Ok` on success, else `Err`
/// carrying the most informative line of the child's stderr as the toast text.
fn check_status(output: std::process::Output, fallback: &str) -> Result<()> {
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let msg = stderr
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .unwrap_or(fallback)
        .trim()
        .to_string();
    bail!("{msg}");
}

/// Resolve the `grove-llm` sibling binary next to the running `grove` exe (the
/// two ship together), falling back to bare `grove-llm` on `PATH`.
fn grove_llm_path(grove_exe: &str) -> PathBuf {
    if let Some(dir) = Path::new(grove_exe).parent() {
        let sibling = dir.join("grove-llm");
        if sibling.exists() {
            return sibling;
        }
    }
    PathBuf::from("grove-llm")
}

/// A percentage box centered in `r` (grove's historical `centered_rect`). Shared
/// with the app, which centers the move-picker overlay with it.
pub(crate) fn centered_rect(pct_x: u16, pct_y: u16, r: Rect) -> Rect {
    let w = r.width * pct_x / 100;
    let h = r.height * pct_y / 100;
    Rect {
        x: r.x + (r.width - w) / 2,
        y: r.y + (r.height - h) / 2,
        width: w,
        height: h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::pane::render_pane;
    use ratatui_rmux::PaneState;
    use rmux_sdk::{PaneCell, PaneCursor, PaneGlyph, PaneSnapshot};

    fn buffer_text(buf: &Buffer) -> String {
        let area = buf.area;
        let mut s = String::new();
        for y in 0..area.height {
            for x in 0..area.width {
                s.push_str(buf[(area.x + x, area.y + y)].symbol());
            }
            s.push('\n');
        }
        s
    }

    /// A `cols`×`rows` pane snapshot filled entirely with `fill` — a "live pane"
    /// to render the modal over.
    fn filled_pane(cols: u16, rows: u16, fill: char) -> PaneState {
        let cells: Vec<PaneCell> = (0..usize::from(cols) * usize::from(rows))
            .map(|_| PaneCell::new(PaneGlyph::new(fill.to_string(), 1)))
            .collect();
        let snapshot = PaneSnapshot::new(cols, rows, cells, PaneCursor::new(0, 0, false, 0))
            .expect("valid snapshot shape");
        PaneState::from_snapshot(snapshot)
    }

    // --- the landmark proof point: modal over a live pane --------------------

    #[test]
    fn modal_renders_centered_over_the_live_pane() {
        // Draw a full-screen "live pane" filled with X, then the modal over it —
        // the exact composition the trellis fork made impossible.
        let area = Rect::new(0, 0, 40, 20);
        let mut buf = Buffer::empty(area);
        let pane = filled_pane(40, 20, 'X');
        render_pane(&pane, area, &mut buf);

        let modal = CaptureModal::new();
        let cursor = modal.render(area, &mut buf, "demo-grove");

        // The pane shows through at the corners (the popup is centered, 70%×50%).
        assert_eq!(buf[(0, 0)].symbol(), "X", "top-left pane cell covered");
        assert_eq!(buf[(39, 19)].symbol(), "X", "bottom-right pane cell covered");
        // The modal border + title are painted in the center band.
        let text = buffer_text(&buf);
        assert!(text.contains("capture \u{2192} demo-grove"), "got:\n{text}");
        assert!(text.contains('\u{250c}'), "modal has a top-left border corner");
        // The cursor lands inside the (centered) box, never at the origin.
        let (cx, cy) = cursor.expect("cursor placed");
        assert!(cx > area.x && cy > area.y, "cursor {cx},{cy} inside the box");
    }

    #[test]
    fn clear_punches_a_hole_so_the_pane_does_not_bleed_through() {
        // The center of the popup must be the modal's (blank) interior, not the
        // pane's glyph — Clear cut the hole.
        let area = Rect::new(0, 0, 40, 20);
        let mut buf = Buffer::empty(area);
        render_pane(&filled_pane(40, 20, 'X'), area, &mut buf);
        CaptureModal::new().render(area, &mut buf, "g");
        // Dead center is inside the popup interior; it must not still read "X".
        assert_ne!(buf[(20, 10)].symbol(), "X", "pane bled through the modal");
    }

    // --- the seed-start confirm modal (070) ----------------------------------

    #[test]
    fn confirm_modal_names_the_seed_and_spells_out_the_side_effect() {
        // Drawn over a live pane, the confirm prompt names the grove, states the
        // worktree+branch side effect, and offers the y/n keys.
        let area = Rect::new(0, 0, 50, 20);
        let mut buf = Buffer::empty(area);
        render_pane(&filled_pane(50, 20, 'X'), area, &mut buf);
        render_confirm("newgrove", area, &mut buf);
        let text = buffer_text(&buf);
        assert!(text.contains("start grove newgrove"), "names the seed:\n{text}");
        assert!(text.contains("worktree"), "states the side effect:\n{text}");
        assert!(text.contains("y start") && text.contains("n cancel"), "offers y/n:\n{text}");
        // The pane shows through at the corners (centered overlay).
        assert_eq!(buf[(0, 0)].symbol(), "X", "pane shows through outside the popup");
    }

    // --- buffer editing ------------------------------------------------------

    #[test]
    fn insert_backspace_take_drive_the_buffer() {
        let mut modal = CaptureModal::new();
        modal.insert("he");
        modal.insert("llo");
        modal.backspace();
        assert_eq!(modal.buffer, "hell");
        let body = modal.take();
        assert_eq!(body, "hell");
        assert_eq!(modal.buffer, "", "take leaves the buffer empty");
    }

    #[test]
    fn buffer_text_is_shown_in_the_modal() {
        let area = Rect::new(0, 0, 40, 20);
        let mut buf = Buffer::empty(area);
        let mut modal = CaptureModal::new();
        modal.insert("ship it");
        modal.render(area, &mut buf, "g");
        assert!(buffer_text(&buf).contains("ship it"));
    }

    // --- toast ---------------------------------------------------------------

    #[test]
    fn success_toast_names_the_grove_on_the_bottom_row() {
        let area = Rect::new(0, 0, 40, 6);
        let mut buf = Buffer::empty(area);
        render_toast(&CaptureOutcome::Captured("alpha".into()), area, &mut buf);
        let bottom: String = (0..area.width)
            .map(|x| buf[(x, area.height - 1)].symbol().to_string())
            .collect();
        assert!(bottom.contains("captured to alpha"), "got: {bottom}");
    }

    #[test]
    fn failure_toast_carries_the_reason() {
        // Callers prefix their own context; render_toast shows the message verbatim.
        let area = Rect::new(0, 0, 50, 6);
        let mut buf = Buffer::empty(area);
        render_toast(&CaptureOutcome::Failed("capture failed: no such repo".into()), area, &mut buf);
        let bottom: String = (0..area.width)
            .map(|x| buf[(x, area.height - 1)].symbol().to_string())
            .collect();
        assert!(bottom.contains("capture failed: no such repo"), "got: {bottom}");
    }

    #[test]
    fn grooming_toasts_report_reject_and_move() {
        let area = Rect::new(0, 0, 40, 6);
        let bottom = |outcome: &CaptureOutcome| -> String {
            let mut buf = Buffer::empty(area);
            render_toast(outcome, area, &mut buf);
            (0..area.width)
                .map(|x| buf[(x, area.height - 1)].symbol().to_string())
                .collect()
        };
        assert!(bottom(&CaptureOutcome::Rejected).contains("rejected"));
        assert!(bottom(&CaptureOutcome::Moved("beta".into())).contains("moved to beta"));
    }

    // --- sibling-binary resolution ------------------------------------------

    #[test]
    fn grove_llm_path_prefers_the_sibling_next_to_grove() {
        use std::fs;
        let tmp = tempfile::TempDir::new().unwrap();
        let dir = tmp.path();
        let grove = dir.join("grove");
        fs::write(&grove, b"").unwrap();
        fs::write(dir.join("grove-llm"), b"").unwrap();
        let resolved = grove_llm_path(grove.to_str().unwrap());
        assert_eq!(resolved, dir.join("grove-llm"));
    }

    #[test]
    fn grove_llm_path_falls_back_to_path_when_no_sibling() {
        let resolved = grove_llm_path("/nonexistent/dir/grove");
        assert_eq!(resolved, PathBuf::from("grove-llm"));
    }
}

//! **Open-in-editor** (040): leader → `e` dumps the focused harness pane's full
//! *rendered* history into `$EDITOR`, restoring the TUI cleanly on exit.
//!
//! ## The shape (ADR-0029, D-A…D-F)
//!
//! The 040 scoping spike found published rmux 0.5.0 already exposes
//! rendered-history capture via the stock `rmux capture-pane` CLI — no fork. So
//! this is the same **shell-out-below-the-seam** idiom as the capture write
//! ([`crate::tui::capture::write_capture`], E1): a pure command-builder
//! ([`capture_pane_argv`]) invoked via `spawn_blocking`, leader-gated above it.
//!
//! ## Split of responsibilities
//!
//! - [`capture_pane_argv`] / [`resolve_editor_from`] are **pure** (unit-tested
//!   headlessly) — the argv the CLI sees and the `$VISUAL`→`$EDITOR`→`vi`
//!   resolution.
//! - [`capture_history`] performs the effectful shell-out to stock `rmux
//!   capture-pane`, resolving the binary the way the SDK resolves its daemon
//!   binary (D-D); the app runs it under `spawn_blocking`.
//! - The terminal suspend/restore + input-thread pause (D-E) lives in
//!   [`crate::tui::app`] — genuinely effectful, verified by hand (cf. the
//!   migration's testability split).

use std::ffi::OsString;
use std::process::Command;

use anyhow::{bail, Context, Result};
use rmux_sdk::bootstrap::discovery::SDK_DAEMON_BINARY_ENV;
use rmux_sdk::PaneId;

/// Build the `capture-pane` argument vector for `pane_id` (D-B): printed (`-p`),
/// full retained history + visible (`-S -`), soft-wrapped lines rejoined (`-J`),
/// targeted at the stable `%N` pane id (E3). **No `-e`** — plain text, since
/// editors don't render ANSI. Pure, so headlessly unit-testable.
pub fn capture_pane_argv(pane_id: PaneId) -> Vec<String> {
    vec![
        "capture-pane".to_string(),
        "-p".to_string(),
        "-S".to_string(),
        "-".to_string(),
        "-J".to_string(),
        "-t".to_string(),
        pane_id.to_string(),
    ]
}

/// Resolve the editor command from explicit `$VISUAL`/`$EDITOR` values:
/// `$VISUAL` → `$EDITOR` → `vi`, whitespace-split so multi-word commands
/// (`code --wait`, `emacs -nw`) carry their flags. Pure core; [`resolve_editor`]
/// reads the env. Returns a non-empty argv (program + any embedded args).
fn resolve_editor_from(visual: Option<String>, editor: Option<String>) -> Vec<String> {
    let line = visual
        .into_iter()
        .chain(editor)
        .map(|s| s.trim().to_string())
        .find(|s| !s.is_empty())
        .unwrap_or_else(|| "vi".to_string());
    line.split_whitespace().map(str::to_string).collect()
}

/// Resolve the editor argv from the live environment (`$VISUAL`→`$EDITOR`→`vi`).
pub fn resolve_editor() -> Vec<String> {
    resolve_editor_from(std::env::var("VISUAL").ok(), std::env::var("EDITOR").ok())
}

/// Resolve the `rmux` binary the SDK's way (D-D): `$RMUX_SDK_DAEMON_BINARY` when
/// set, else bare `rmux` on `PATH`. grove redirects this var to the bundled
/// daemon at startup ([`crate::tui::launch`], ADR-0030 §3), so dev + bundled
/// alike hit the *same* binary the daemon was started from.
pub fn rmux_binary() -> OsString {
    std::env::var_os(SDK_DAEMON_BINARY_ENV).unwrap_or_else(|| "rmux".into())
}

/// Capture the pane's full rendered history by shelling out to stock `rmux
/// capture-pane` (D-A/D-B). Synchronous + blocking (it talks to the daemon over
/// IPC), so the app runs it under `spawn_blocking` to keep the reactor free.
/// Returns the dump text; surfaces the child's stderr as the error (→ a toast,
/// never a crash), mirroring [`crate::tui::capture::write_capture`].
pub fn capture_history(pane_id: PaneId) -> Result<String> {
    let bin = rmux_binary();
    let output = Command::new(&bin)
        .args(capture_pane_argv(pane_id))
        .output()
        .with_context(|| format!("running {} capture-pane", bin.to_string_lossy()))?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let msg = stderr
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("capture-pane failed")
        .trim()
        .to_string();
    bail!("{msg}");
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- capture-pane argv builder (D-B) -------------------------------------

    #[test]
    fn capture_pane_argv_builds_the_rendered_history_flags() {
        let argv = capture_pane_argv(PaneId::new(3));
        assert_eq!(
            argv,
            vec!["capture-pane", "-p", "-S", "-", "-J", "-t", "%3"],
        );
    }

    #[test]
    fn capture_pane_argv_renders_start_history_as_two_args() {
        // `-S -` must reach execvp as two argv slots, not one `"-S -"` token —
        // otherwise the daemon sees an unknown flag.
        let argv = capture_pane_argv(PaneId::new(7));
        let s = argv.iter().position(|a| a == "-S").expect("has -S");
        assert_eq!(argv[s + 1], "-", "history-start sentinel follows -S");
    }

    #[test]
    fn capture_pane_argv_targets_the_pane_id_as_percent_n() {
        // The stable PaneId is the tmux `%N` target (E3); no formatting helper —
        // PaneId's Display already yields `%N`.
        let argv = capture_pane_argv(PaneId::new(42));
        assert_eq!(argv.last().unwrap(), "%42");
    }

    // --- $EDITOR resolution ($VISUAL→$EDITOR→vi) -----------------------------

    #[test]
    fn editor_prefers_visual_over_editor() {
        let argv = resolve_editor_from(Some("hx".into()), Some("vim".into()));
        assert_eq!(argv, vec!["hx"]);
    }

    #[test]
    fn editor_falls_back_to_editor_when_visual_unset() {
        let argv = resolve_editor_from(None, Some("vim".into()));
        assert_eq!(argv, vec!["vim"]);
    }

    #[test]
    fn editor_skips_blank_visual_for_editor() {
        // An exported-but-empty `$VISUAL` must not win over a real `$EDITOR`.
        let argv = resolve_editor_from(Some("   ".into()), Some("nano".into()));
        assert_eq!(argv, vec!["nano"]);
    }

    #[test]
    fn editor_falls_back_to_vi_when_both_unset() {
        assert_eq!(resolve_editor_from(None, None), vec!["vi"]);
        assert_eq!(resolve_editor_from(Some("".into()), Some("".into())), vec!["vi"]);
    }

    #[test]
    fn editor_splits_multiword_commands_into_argv() {
        // `code --wait` must carry its flag, not become a single bogus program.
        let argv = resolve_editor_from(Some("code --wait".into()), None);
        assert_eq!(argv, vec!["code", "--wait"]);
    }
}

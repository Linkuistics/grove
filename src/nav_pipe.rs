//! The controller ↔ `grove-nav` **pipe protocol** (ADR-0018 split-driving seam;
//! leaf `060-harness-pane/040-grove-integration/080-controller-plugin-pipe`).
//!
//! Two asymmetric directions over zellij's `pipe` mechanism, both targeting the
//! one running nav instance by its `file:` URL (the same URL the layout sidebar
//! and the `Ctrl-o` leader use, so all three address the same plugin):
//!
//! - **Controller → plugin (grove-state):** a *fire-and-forget push*. On startup
//!   and on every fs-watch settle the controller shells out `zellij … pipe --name
//!   grove-state --plugin file:<wasm> -- <payload>`, where `<payload>` is the live
//!   grove list ([`encode_grove_state`]). The plugin parses it, re-renders, and
//!   calls `unblock_cli_pipe_input` so this one-shot invocation exits promptly.
//!
//! - **Plugin → controller (grove-intent):** a *long-lived back-channel*. A
//!   `zellij pipe` invocation stays alive exactly as long as **its stdin is
//!   open** (the `tail -f | zellij pipe` streaming model — verified live: a
//!   `-- payload` form exits the instant the plugin's `pipe()` returns, but an
//!   open stdin blocks). So the controller holds one `zellij … pipe --name
//!   grove-intent --plugin file:<wasm>` child with **stdin held open**, writes a
//!   single `__init` line to it (so the plugin's `pipe()` fires once and captures
//!   the invocation's `pipe_id`), and then keeps stdin open. The plugin streams
//!   `open <name>\n` up the invocation's stdout via `cli_pipe_output(pipe_id, …)`
//!   for each not-yet-open grove the user selects. [`IntentReader`] reads those
//!   lines and forwards the grove names; the controller first-opens each via the
//!   040 `zellij action` driver. If the plugin relaunches the child EOFs and the
//!   reader respawns it (the "survives the plugin (re)launching" requirement).
//!
//! Wire format is deliberately lean (the leaf's call): a line-oriented codec, no
//! `serde` (mirroring the hand-rolled [[seam frame]] discipline). The pure codec
//! and argv builders are unit-tested here; the shell-out wrappers are thin glue
//! (cf. `harness_drive::ZellijCli`) verified against a live zellij.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::{Context, Result};

use crate::repo_view::GroveSummary;

/// The pipe message names — the `--name` that routes a CLI pipe to the matching
/// arm of the plugin's `pipe()` handler.
const STATE_PIPE: &str = "grove-state";
const INTENT_PIPE: &str = "grove-intent";

/// The single line the controller writes to the intent pipe's **stdin** to open
/// the back-channel: its only job is to make the plugin's `pipe()` fire once so
/// it captures the invocation's `pipe_id` to stream intents back on. stdin then
/// stays open, which is what keeps the invocation alive.
const INTENT_INIT: &str = "__init\n";

/// How long the [`IntentReader`] waits before respawning the `grove-intent` pipe
/// after it ends (plugin relaunch, or zellij not yet ready). Short enough to feel
/// instant, long enough not to busy-spawn while zellij is starting.
const RESPAWN_DELAY: Duration = Duration::from_millis(400);

// ---------------------------------------------------------------------------
// Pure codec + argv builders (unit-tested).

/// Encode the live grove list as the `grove-state` payload: one grove per line,
/// `name \t inbox_pending`. Grove names are sanitised (no tabs/newlines), so the
/// two-column split is unambiguous. An empty list encodes to the empty string.
fn encode_grove_state(groves: &[GroveSummary]) -> String {
    groves
        .iter()
        .map(|g| format!("{}\t{}", g.name, g.inbox_pending))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse one back-channel line from the plugin. The only intent is `open
/// <name>`; the grove name is everything after the `open ` prefix, trimmed.
/// Anything else (blank lines, unknown verbs) parses to `None` and is ignored,
/// so a stray byte on the channel can never be mistaken for an open request.
fn parse_intent_line(line: &str) -> Option<String> {
    let name = line.trim().strip_prefix("open ")?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// The `file:`-scheme plugin URL zellij addresses the nav by — the same form the
/// layout sidebar and leader keybind use, so the pipe reaches that one instance.
fn plugin_url(wasm: &Path) -> String {
    format!("file:{}", wasm.display())
}

/// Argv for the one-shot grove-state push: `--session <s> pipe --name grove-state
/// --plugin file:<wasm> -- <payload>`. `--session` is global (before `pipe`)
/// because the controller lives outside zellij and must target the dashboard's
/// session explicitly — exactly as the 040 `action` driver does.
fn grove_state_args(session: &str, wasm: &Path, payload: &str) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "pipe".into(),
        "--name".into(),
        STATE_PIPE.into(),
        "--plugin".into(),
        plugin_url(wasm),
        "--".into(),
        payload.into(),
    ]
}

/// Argv for the long-lived intent back-channel: `--session <s> pipe --name
/// grove-intent --plugin file:<wasm>`. Deliberately **no** `-- payload`: with a
/// payload arg the invocation exits as soon as the plugin's `pipe()` returns, so
/// instead the controller drives it through **stdin** ([`INTENT_INIT`] + holding
/// stdin open), which is what keeps the channel alive.
fn grove_intent_args(session: &str, wasm: &Path) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "pipe".into(),
        "--name".into(),
        INTENT_PIPE.into(),
        "--plugin".into(),
        plugin_url(wasm),
    ]
}

// ---------------------------------------------------------------------------
// Shell-out glue (thin; live-verified).

/// Push the current grove list to the nav (controller → plugin). Best-effort and
/// quick: the plugin unblocks the one-shot pipe immediately. `stdin` is nulled so
/// a blank payload (zero groves) can never make `zellij pipe` block reading stdin.
pub fn push_grove_state(session: &str, wasm: &Path, groves: &[GroveSummary]) -> Result<()> {
    let payload = encode_grove_state(groves);
    let out = Command::new("zellij")
        .args(grove_state_args(session, wasm, &payload))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("running zellij pipe (grove-state)")?;
    if !out.status.success() {
        anyhow::bail!(
            "zellij pipe grove-state failed (status {:?}): {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// The controller's end of the long-lived `grove-intent` back-channel. Owns a
/// background thread that keeps a `zellij pipe` child alive, reads `open <name>`
/// lines from its stdout, and forwards each grove name on [`IntentReader::rx`].
/// Respawns the child if it ends (plugin relaunch), until [`IntentReader::stop`].
pub struct IntentReader {
    /// Receives the grove names the plugin asked to open (first-open intents).
    pub rx: Receiver<String>,
    stop: Arc<AtomicBool>,
    /// The live `zellij pipe` child, shared so [`IntentReader::stop`] can kill it
    /// and thereby unblock the reader thread's blocking `read` (the reader and
    /// the loop's own teardown coordinate via `take()` on this slot).
    child: Arc<Mutex<Option<Child>>>,
    handle: Option<JoinHandle<()>>,
}

impl IntentReader {
    /// Spawn the back-channel for `session`/`wasm`. Returns immediately; intents
    /// arrive on [`IntentReader::rx`].
    pub fn spawn(session: String, wasm: std::path::PathBuf) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let child: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
        let stop_thread = Arc::clone(&stop);
        let child_thread = Arc::clone(&child);
        let handle =
            std::thread::spawn(move || intent_loop(&session, &wasm, &tx, &stop_thread, &child_thread));
        Self {
            rx,
            stop,
            child,
            handle: Some(handle),
        }
    }

    /// Signal the reader to stop and join it. Idempotent; called on controller
    /// teardown so the background thread (and its `zellij pipe` child) don't
    /// outlive the dashboard. Killing the live child unblocks the reader's
    /// blocking `read`, so the join is bounded even though the caller tears the
    /// zellij session down only *after* this returns.
    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(mut child) = self.child.lock().unwrap().take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for IntentReader {
    fn drop(&mut self) {
        self.stop();
    }
}

/// The back-channel thread body: (re)spawn the `grove-intent` pipe and pump its
/// stdout lines to `tx` until `stop`. Factored out so the spawn/respawn policy is
/// readable in one place. The live child is published into `slot` so
/// [`IntentReader::stop`] can kill it to interrupt a blocked read.
fn intent_loop(
    session: &str,
    wasm: &Path,
    tx: &Sender<String>,
    stop: &AtomicBool,
    slot: &Mutex<Option<Child>>,
) {
    while !stop.load(Ordering::SeqCst) {
        if let Ok(mut child) = spawn_intent_child(session, wasm) {
            let stdout = child.stdout.take();
            // Open the channel: write the init line, then keep this stdin handle
            // alive for the whole iteration — dropping it would EOF stdin and the
            // `zellij pipe` invocation would exit (the bug the live probe caught).
            let mut stdin = child.stdin.take();
            if let Some(si) = stdin.as_mut() {
                let _ = si.write_all(INTENT_INIT.as_bytes());
                let _ = si.flush();
            }
            *slot.lock().unwrap() = Some(child);
            // `stop` may have fired between spawn and publish; if so, reclaim and
            // kill the child we just stored rather than blocking on its stdout.
            if stop.load(Ordering::SeqCst) {
                kill_slot(slot);
                break;
            }
            if let Some(stdout) = stdout {
                pump_intents(stdout, tx, stop);
            }
            // EOF or stop: close stdin and drop the child (no-op if `stop` took it).
            drop(stdin);
            kill_slot(slot);
        }
        // zellij not ready yet, or the plugin relaunched: pause, then respawn.
        if stop.load(Ordering::SeqCst) {
            break;
        }
        std::thread::sleep(RESPAWN_DELAY);
    }
}

/// Kill and reap whatever child is in `slot`, if any. Safe to call repeatedly.
fn kill_slot(slot: &Mutex<Option<Child>>) {
    if let Some(mut child) = slot.lock().unwrap().take() {
        let _ = child.kill();
        let _ = child.wait();
    }
}

/// Spawn one `grove-intent` pipe child with stdin and stdout piped. The caller
/// writes [`INTENT_INIT`] to stdin and then holds it open: an open stdin is what
/// keeps `zellij pipe` alive (it exits on stdin EOF), and the open invocation is
/// the channel the plugin streams intents back on.
fn spawn_intent_child(session: &str, wasm: &Path) -> Result<Child> {
    Command::new("zellij")
        .args(grove_intent_args(session, wasm))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .context("spawning zellij pipe (grove-intent)")
}

/// Read `open <name>` lines from the intent child's `stdout` and forward each
/// grove name to `tx`, returning when stdout closes (plugin gone, or the child
/// was killed by [`IntentReader::stop`]) or `stop` is observed.
fn pump_intents(stdout: ChildStdout, tx: &Sender<String>, stop: &AtomicBool) {
    let mut lines = BufReader::new(stdout).lines();
    while let Some(Ok(line)) = lines.next() {
        if stop.load(Ordering::SeqCst) {
            break;
        }
        if let Some(name) = parse_intent_line(&line) {
            if tx.send(name).is_err() {
                break; // the controller hung up; stop pumping.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_view::Lifecycle;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    fn grove(name: &str, pending: usize) -> GroveSummary {
        GroveSummary {
            name: name.into(),
            lifecycle: Lifecycle::Live,
            live_leaves: 0,
            retired_leaves: 0,
            inbox_pending: pending,
            worktree_versions: BTreeMap::new(),
        }
    }

    #[test]
    fn encode_grove_state_is_one_tab_separated_row_per_grove() {
        let payload = encode_grove_state(&[grove("auth", 0), grove("billing", 3)]);
        assert_eq!(payload, "auth\t0\nbilling\t3");
    }

    #[test]
    fn encode_grove_state_of_empty_list_is_empty() {
        assert_eq!(encode_grove_state(&[]), "");
    }

    #[test]
    fn parse_intent_line_reads_open_requests() {
        assert_eq!(parse_intent_line("open auth"), Some("auth".to_string()));
        // Tolerant of surrounding whitespace the line reader might leave.
        assert_eq!(parse_intent_line("  open billing  \n"), Some("billing".to_string()));
    }

    #[test]
    fn parse_intent_line_ignores_anything_that_is_not_an_open() {
        assert_eq!(parse_intent_line(""), None);
        assert_eq!(parse_intent_line("open"), None); // no name
        assert_eq!(parse_intent_line("open "), None); // empty name
        assert_eq!(parse_intent_line("close auth"), None); // unknown verb
        assert_eq!(parse_intent_line("garbage"), None);
    }

    #[test]
    fn grove_state_argv_targets_session_name_plugin_and_payload() {
        let args = grove_state_args("grove-acme", Path::new("/c/grove-nav.wasm"), "auth\t0");
        assert_eq!(
            args,
            vec![
                "--session", "grove-acme",
                "pipe",
                "--name", "grove-state",
                "--plugin", "file:/c/grove-nav.wasm",
                "--", "auth\t0",
            ]
        );
    }

    #[test]
    fn grove_intent_argv_has_no_payload_so_stdin_drives_the_channel() {
        // No trailing `-- payload`: a payload arg makes the invocation exit as
        // soon as the plugin's pipe() returns (verified live). The channel is
        // driven through stdin instead (INTENT_INIT + holding stdin open).
        let args = grove_intent_args("grove-acme", Path::new("/c/grove-nav.wasm"));
        assert_eq!(
            args,
            vec![
                "--session", "grove-acme",
                "pipe",
                "--name", "grove-intent",
                "--plugin", "file:/c/grove-nav.wasm",
            ]
        );
        assert!(!args.contains(&"--".to_string()), "no payload separator");
    }

    #[test]
    fn plugin_url_is_the_file_scheme_path() {
        assert_eq!(plugin_url(&PathBuf::from("/c/grove-nav.wasm")), "file:/c/grove-nav.wasm");
    }
}

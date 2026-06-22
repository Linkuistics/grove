// The self-driving loop — grove's runtime (ADR-0032).
//
// `grove do <name>` drives the *whole loop*, not one task: it launches a fresh
// foreground `claude` per grove task, and relaunches with fresh context each
// time the agent fires the completion signal (`grove-llm complete`). Any other
// exit — human `/exit`/Ctrl-C, or a crash — stops the loop, resumable later by
// re-running `grove do <name>` (restart ≡ continuation, the loop body holds zero
// state and re-derives position from `grove-llm pick`).
//
// The driver is deliberately tiny — a plain shell `while` loop could stand in
// (constraint 6, walk-away-able):
//
//     sig="$TMPDIR/grove-loop-<name>.signal"
//     while :; do
//       rm -f "$sig"
//       GROVE_SIGNAL_FILE="$sig" \
//         sh -c 'export GROVE_CLAUDE_PID=$$; exec claude "$@"' sh "$prompt"
//       stty sane 2>/dev/null
//       [ -f "$sig" ] || break        # no completion signal → stop
//     done

use crate::harness::Harness;
use anyhow::{Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Relaunch-signal file path for a grove. Lives in the temp dir (ephemeral
/// loop IPC, not durable grove state); name-keyed so concurrent groves don't
/// collide. Cleared at the start of every iteration.
pub fn signal_file_path(name: &str) -> PathBuf {
    let safe: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    std::env::temp_dir().join(format!("grove-loop-{}.signal", safe))
}

/// Entry point: install the interrupt guard, then run the loop. The real
/// `grove do` path calls this; tests call [`run_loop`] directly to avoid the
/// process-global signal change.
pub fn run(harness: &'static Harness, repo_path: &Path, worktree: &Path, name: &str) -> Result<()> {
    ignore_interrupts();
    run_loop(harness, repo_path, worktree, name)
}

/// The loop body, free of process-global side effects.
pub fn run_loop(
    harness: &'static Harness,
    repo_path: &Path,
    worktree: &Path,
    name: &str,
) -> Result<()> {
    let signal_file = signal_file_path(name);
    let repo_name = repo_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repo".to_string());
    let session_name = format!("{}: {} grove", repo_name, name);

    loop {
        // Clear any stale signal from the previous iteration.
        let _ = std::fs::remove_file(&signal_file);

        // A brand-new grove (no `.grove/` yet) bootstraps via `start`; an
        // existing one uses `continue` (which, on an empty `pick`, proposes the
        // finish cycle in-session). This is the loop body's only state — and it
        // is re-derived from the filesystem each iteration, so restart ≡
        // continuation.
        let verb = if worktree.join(".grove").is_dir() {
            "continue"
        } else {
            "start"
        };
        let prompt = crate::launch::load_prompt(verb)?;

        launch_session(harness, worktree, &session_name, &prompt, &signal_file)?;

        // A SIGTERM'd TUI can leave the terminal in raw mode / the alternate
        // screen; reset before relaunching (and on the way out).
        reset_terminal();

        if signal_file.exists() {
            // Completion signal fired → relaunch with fresh context.
            continue;
        }
        // Human `/exit`/Ctrl-C, or a crash: no signal → stop. Re-running
        // `grove do <name>` resumes from `grove-llm pick`.
        eprintln!("grove: session ended without a completion signal — loop stopped.");
        eprintln!("       Re-run `grove do {name}` to resume (restart ≡ continuation).");
        break;
    }
    let _ = std::fs::remove_file(&signal_file);
    Ok(())
}

/// Launch one fresh foreground `claude` owning the real TTY. The
/// `sh -c 'export GROVE_CLAUDE_PID=$$; exec "$@"'` wrapper hands the agent
/// `claude`'s own PID: `exec` preserves the PID, so `$$` (captured before exec)
/// is the final `claude` PID, inherited by the agent's Bash tool. `GROVE_HARNESS_BIN`
/// overrides the binary (testing / wrapping `claude`).
fn launch_session(
    harness: &Harness,
    worktree: &Path,
    session_name: &str,
    prompt: &str,
    signal_file: &Path,
) -> Result<()> {
    let bin = std::env::var("GROVE_HARNESS_BIN").unwrap_or_else(|_| harness.exec_bin.to_string());

    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(r#"export GROVE_CLAUDE_PID=$$; exec "$@""#)
        .arg("sh") // $0 for the inner shell
        .arg(&bin); // $1 = the harness binary
    if !harness.name_args.is_empty() {
        cmd.args(harness.name_args).arg(session_name);
    }
    cmd.arg(prompt);
    cmd.current_dir(worktree);
    cmd.env("GROVE_SIGNAL_FILE", signal_file);

    // A completion kill makes `claude` exit non-zero; that is the normal exit
    // path, not an error. The signal file — not the exit code — decides relaunch.
    let _ = cmd.status().context("launching the harness session")?;
    Ok(())
}

/// Reset the terminal after a (possibly SIGTERM'd) TUI: restore cooked mode,
/// leave the alternate screen, show the cursor. No-op when stdin isn't a TTY
/// (headless / test runs).
fn reset_terminal() {
    if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
        return;
    }
    let _ = Command::new("stty").arg("sane").status();
    print!("\x1b[?1049l\x1b[?25h\x1b[0m");
    let _ = std::io::stdout().flush();
}

/// Ignore SIGINT in the driver so a terminal Ctrl-C (delivered to the whole
/// foreground process group) does not kill the loop; the child `claude`
/// installs its own handler and still responds. The driver must survive the
/// interrupt to reach the relaunch-vs-stop decision.
fn ignore_interrupts() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
}

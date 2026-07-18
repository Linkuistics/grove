// The self-driving loop — grove's runtime (self-driving-loop).
//
// `grove do` drives the *whole loop*, not one task: it launches a fresh
// foreground harness session per grove task, and relaunches with fresh context each
// time the agent fires the completion signal (`grove-llm complete`). Any other
// exit — human `/exit`/Ctrl-C, or a crash — stops the loop, resumable later by
// re-running `grove do` from the same working tree (restart ≡ continuation, the
// loop body holds zero state and re-derives position from `grove-llm pick`).
//
// The driver is deliberately tiny — a plain shell `while` loop could stand in
// (constraint 6, walk-away-able); model selection (model-per-task-kind) is just
// a kind→model lookup before the launch:
//
//     sig="$TMPDIR/grove-loop-<name>.signal"
//     while :; do
//       rm -f "$sig"
//       [ -d "$wt/.grove" ] && kind=$(grove-llm kind) || kind=planning
//       case "$kind" in
//         planning)  model="$GROVE_PLANNING_MODEL" ;;
//         research)  model="$GROVE_RESEARCH_MODEL" ;;
//         prototype) model="$GROVE_PROTOTYPE_MODEL" ;;
//         work)      model="$GROVE_WORK_MODEL" ;;
//         review)    model="$GROVE_REVIEW_MODEL" ;;
//         *)         model="" ;;                      # empty grove → no --model
//       esac
//       [ -n "$model" ] && set -- --model "$model" "$prompt" || set -- "$prompt"
//       GROVE_SIGNAL_FILE="$sig" \
//         sh -c 'export GROVE_HARNESS_PID=$$; exec "$harness_bin" "$@"' sh "$@"
//       stty sane 2>/dev/null
//       [ -f "$sig" ] || break        # no completion signal → stop
//     done

use crate::complete::{self, Disposition};
use crate::harness::Harness;
use crate::leaf::Kind;
use anyhow::{Context, Result};
use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Why the loop stopped — the loop's terminal disposition, made first-class so
/// a clean whole-grove finish is distinguishable from an abnormal stop (rather
/// than both looking like "the loop just ended").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopOutcome {
    /// The grove finished cleanly: a session signalled `complete --done`.
    Finished,
    /// A non-signalled exit stopped the loop (human `/exit`/Ctrl-C, or a
    /// crash); resumable by re-running `grove do` from the same working tree.
    Stopped,
}

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
/// process-global signal change. The outcome is already reported on stderr by
/// the loop body, so the caller can discard it.
pub fn run(harness: &'static Harness, repo_path: &Path, worktree: &Path, name: &str) -> Result<()> {
    ignore_interrupts();
    run_loop(harness, repo_path, worktree, name).map(|_| ())
}

/// The loop body, free of process-global side effects. Returns why it stopped
/// ([`LoopOutcome`]) so a clean finish is distinguishable from an abnormal exit.
pub fn run_loop(
    harness: &'static Harness,
    repo_path: &Path,
    worktree: &Path,
    name: &str,
) -> Result<LoopOutcome> {
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
        let prompt = crate::launch::load_prompt(harness, verb)?;

        // Route the launch by the picked leaf's kind: per-kind harness
        // override first (GROVE_<KIND>_HARNESS), then model-per-task-kind
        // against whichever harness actually launches.
        let (launch_harness, model) = resolve_launch(harness, worktree, verb)?;

        launch_session(
            launch_harness,
            worktree,
            &session_name,
            &prompt,
            &signal_file,
            model.as_deref(),
        )?;

        // A SIGTERM'd TUI can leave the terminal in raw mode / the alternate
        // screen; reset before relaunching (and on the way out).
        reset_terminal();

        match complete::read_signal(&signal_file) {
            // Per-task completion signal → relaunch with fresh context.
            Some(Disposition::Relaunch) => continue,
            // `complete --done` (the Finish cycle's last action) → stop clean.
            Some(Disposition::Done) => {
                eprintln!("grove: grove finished — loop complete.");
                let _ = std::fs::remove_file(&signal_file);
                return Ok(LoopOutcome::Finished);
            }
            // Human `/exit`/Ctrl-C, or a crash: no signal → stop. Re-running
            // `grove do` from the same working tree resumes from `grove-llm pick`.
            None => {
                eprintln!("grove: session ended without a completion signal — loop stopped.");
                eprintln!(
                    "       Re-run `grove do` from this working tree to resume (restart ≡ continuation)."
                );
                let _ = std::fs::remove_file(&signal_file);
                return Ok(LoopOutcome::Stopped);
            }
        }
    }
}

/// Launch one fresh foreground harness session owning the real TTY. The
/// `sh -c 'export GROVE_HARNESS_PID=$$; exec "$@"'` wrapper hands the agent
/// the harness session's own PID: `exec` preserves the PID, so `$$` (captured
/// before exec) is the final harness PID, inherited by the agent's Bash tool
/// (`GROVE_CLAUDE_PID` is co-exported for one release). `GROVE_HARNESS_BIN`
/// overrides the binary (testing / wrapping `claude`).
fn launch_session(
    harness: &Harness,
    worktree: &Path,
    session_name: &str,
    prompt: &str,
    signal_file: &Path,
    model: Option<&str>,
) -> Result<()> {
    let bin = harness_bin(harness);

    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(r#"export GROVE_HARNESS_PID=$$ GROVE_CLAUDE_PID=$$; exec "$@""#)
        .arg("sh") // $0 for the inner shell
        .arg(&bin); // $1 = the harness binary
    if !harness.name_args.is_empty() {
        cmd.args(harness.name_args).arg(session_name);
    }
    // Model selection (model-per-task-kind): the per-harness flag template + the
    // chosen model, before the positional prompt. `select_model` only yields
    // `Some` when `model_args` is non-empty, but guard so this stays correct in
    // isolation.
    if let Some(model) = model {
        if !harness.model_args.is_empty() {
            cmd.args(harness.model_args).arg(model);
        }
    }
    cmd.arg(prompt);
    cmd.current_dir(worktree);
    cmd.env("GROVE_SIGNAL_FILE", signal_file);

    // A completion kill makes `claude` exit non-zero; that is the normal exit
    // path, not an error. The signal file — not the exit code — decides relaunch.
    let _ = cmd.status().context("launching the harness session")?;
    Ok(())
}

/// The binary to exec for a harness: `GROVE_HARNESS_BIN_<NAME>` (the
/// per-harness test seam — kind routing can launch two different harnesses in
/// one loop) beats the legacy global `GROVE_HARNESS_BIN`, beats `exec_bin`.
fn harness_bin(harness: &Harness) -> String {
    std::env::var(format!("GROVE_HARNESS_BIN_{}", harness.name.to_uppercase()))
        .or_else(|_| std::env::var("GROVE_HARNESS_BIN"))
        .unwrap_or_else(|_| harness.exec_bin.to_string())
}

/// Env-name suffixes for the five kinds, in taxonomy order. Shared by the
/// model lookup and (task-routing) the harness override.
const KIND_SUFFIXES: [&str; 5] = ["PLANNING", "RESEARCH", "PROTOTYPE", "WORK", "REVIEW"];

fn env_suffix(kind: Kind) -> &'static str {
    match kind {
        Kind::Planning => "PLANNING",
        Kind::Research => "RESEARCH",
        Kind::Prototype => "PROTOTYPE",
        Kind::Work => "WORK",
        Kind::Review => "REVIEW",
    }
}

/// The model value for a kind on a harness: the harness-scoped var
/// (`GROVE_<HARNESS>_<KIND>_MODEL`) beats the base (`GROVE_<KIND>_MODEL`).
/// Scoped because one shared value cannot serve two harnesses — a codex
/// profile name is garbage to pi and vice versa.
fn model_for(harness: &Harness, kind: Kind) -> Option<String> {
    let h = harness.name.to_uppercase();
    let s = env_suffix(kind);
    env_model(&format!("GROVE_{h}_{s}_MODEL")).or_else(|| env_model(&format!("GROVE_{s}_MODEL")))
}

/// Whether any model env var (scoped-to-this-harness or base) is set — the
/// gate that keeps the common unconfigured path a zero-subprocess launch.
fn any_model_env(harness: &Harness) -> bool {
    let h = harness.name.to_uppercase();
    KIND_SUFFIXES.iter().any(|s| {
        env_model(&format!("GROVE_{h}_{s}_MODEL")).is_some()
            || env_model(&format!("GROVE_{s}_MODEL")).is_some()
    })
}

/// The per-kind harness override: `GROVE_<KIND>_HARNESS` names the harness
/// that runs leaves of that kind, whatever the grove is stamped to (the
/// trial's "K3 reviews everywhere": GROVE_REVIEW_HARNESS=pi). Unknown names
/// fail loudly — a typo that silently fell back would misroute every review.
fn harness_override(kind: Kind) -> Result<Option<&'static Harness>> {
    let var = format!("GROVE_{}_HARNESS", env_suffix(kind));
    match std::env::var(&var) {
        Ok(name) if !name.is_empty() => {
            let h = crate::harness::by_name(&name).ok_or_else(|| {
                anyhow::anyhow!(
                    "{var}={name}: unknown harness. Known: {}",
                    crate::harness::known_names()
                )
            })?;
            Ok(Some(h))
        }
        _ => Ok(None),
    }
}

/// Whether any per-kind harness override is set — like `any_model_env`, the
/// gate that decides whether the kind peek is worth a subprocess.
fn any_harness_override_env() -> bool {
    KIND_SUFFIXES
        .iter()
        .any(|s| std::env::var(format!("GROVE_{s}_HARNESS")).is_ok_and(|v| !v.is_empty()))
}

/// Resolve where and on what the next session launches: peek the picked
/// leaf's kind (only when some routing env makes it matter), apply the
/// per-kind harness override, then resolve the model against the
/// *post-override* harness — so `GROVE_PI_REVIEW_MODEL` governs a review
/// rerouted to pi, not the stamped harness's vars.
fn resolve_launch(
    stamped: &'static Harness,
    worktree: &Path,
    verb: &str,
) -> Result<(&'static Harness, Option<String>)> {
    if !any_harness_override_env() && !any_model_env(stamped) {
        return Ok((stamped, None));
    }
    let kind = if verb == "start" {
        Some(Kind::Planning)
    } else {
        resolve_kind(worktree)
    };
    let Some(kind) = kind else {
        // No kind (empty grove / peek degraded): no basis to route or select.
        return Ok((stamped, None));
    };
    let launch = harness_override(kind)?.unwrap_or(stamped);
    let model = if launch.model_args.is_empty() {
        None
    } else {
        model_for(launch, kind)
    };
    Ok((launch, model))
}

/// Read a model env var, treating empty as unset.
fn env_model(var: &str) -> Option<String> {
    std::env::var(var).ok().filter(|s| !s.is_empty())
}

/// Peek the next live leaf's kind by running `grove-llm kind` against the
/// worktree — the same verb (and code path) the launched agent would call.
/// Model selection is an enhancement, never a reason to jam the loop, so any
/// failure (binary missing, non-zero exit, unparseable output) degrades to
/// `None` (launch on the default) with a diagnostic. An empty grove (no live
/// leaf — the finish-cycle iteration) also yields `None`, silently: with no leaf
/// there is no kind to key a model on.
///
/// Every degrade is **loud**, and that is the point: an unrecognised kind is
/// treated as `work` (task-kind-taxonomy), which is typically the cheapest model,
/// so a swallowed warning reads as a silent downgrade. The child's stderr is
/// inherited, not captured — see the note on the spawn below.
fn resolve_kind(worktree: &Path) -> Option<Kind> {
    let out = Command::new(grove_llm_bin())
        .arg("kind")
        .current_dir(worktree)
        // Inherit stderr rather than capture it. `kind` *degrades* on an
        // unrecognised `**Kind:**` line — it warns and prints `work`, on a
        // **zero** exit — so the warning rides the success path. Capturing would
        // swallow it and silently launch the leaf on the work model, which in a
        // typical config is the cheapest one: a silent downgrade with no way to
        // see why. Inheriting surfaces every diagnostic, warning or error.
        .stderr(Stdio::inherit())
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let token = String::from_utf8_lossy(&o.stdout);
            let token = token.trim();
            if token.is_empty() {
                return None; // empty grove: no kind to select on
            }
            match Kind::parse(token) {
                Ok(k) => Some(k),
                Err(_) => {
                    eprintln!(
                        "grove: unrecognized task kind {token:?} from `grove-llm kind`; \
                         launching on the default model"
                    );
                    None
                }
            }
        }
        Ok(_) => {
            // The child's own diagnostic already reached stderr (inherited above).
            eprintln!("grove: `grove-llm kind` failed; launching on the default model");
            None
        }
        Err(e) => {
            eprintln!(
                "grove: could not run grove-llm for model selection ({e}); \
                 launching on the default model"
            );
            None
        }
    }
}

/// Locate the `grove-llm` binary. `GROVE_LLM_BIN` overrides (the test seam,
/// parallel to `GROVE_HARNESS_BIN`); otherwise prefer the sibling of the running
/// executable (`grove` and `grove-llm` install together), falling back to PATH.
fn grove_llm_bin() -> OsString {
    if let Some(bin) = std::env::var_os("GROVE_LLM_BIN") {
        return bin;
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(sibling) = exe.parent().map(|d| d.join("grove-llm")) {
            if sibling.is_file() {
                return sibling.into_os_string();
            }
        }
    }
    OsString::from("grove-llm")
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

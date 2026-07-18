// The self-driving loop — grove's runtime (self-driving-loop).
//
// `grove do` drives the *whole loop*, not one task: it launches a fresh
// foreground harness session per grove task, and relaunches with fresh context each
// time the agent fires the completion signal (`grove-llm complete`). Any other
// exit — human `/exit`/Ctrl-C, or a crash — stops the loop, resumable later by
// re-running `grove do` from the same working tree (restart ≡ continuation, the
// loop body holds zero state and re-derives position from `grove-llm pick`).
//
// The driver spawns the harness directly — no shell, no PID-export trick; the
// spawned `Child` already carries its own pid — and watches it while it runs:
// poll `try_wait` alongside the completion-signal file, and once the file
// appears, apply grace → SIGTERM → kill-grace → SIGKILL to the child itself
// (driver-side watcher — self-driving-loop). The driver is the harness's own
// parent process, outside whatever sandbox the harness runs under, so it can
// always signal its child — unlike the in-agent self-kill this replaces,
// which codex's Seatbelt sandbox silently denied.
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
//       GROVE_SIGNAL_FILE="$sig" "$harness_bin" "$@" &
//       pid=$!
//       # poll $pid (try_wait) and "$sig" every ~500ms; on signal appearing:
//       # sleep $GROVE_KILL_GRACE, kill -TERM $pid, sleep
//       # $GROVE_KILL_GRACE_KILL, kill -KILL $pid
//       wait "$pid"
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
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

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

        // Route the launch by the picked leaf's kind: per-kind harness
        // override first (GROVE_<KIND>_HARNESS), then model-per-task-kind
        // against whichever harness actually launches. Resolved *before*
        // loading the prompt (B7): the prompt must come from the harness that
        // actually launches, not the stamped one — reading it first would
        // silently serve the wrong harness's copy whenever a reroute happens.
        let (launch_harness, model, rerouted) = resolve_launch(harness, worktree, verb)?;
        let prompt = crate::launch::load_prompt(launch_harness, verb)?;

        launch_session(
            launch_harness,
            worktree,
            &session_name,
            &prompt,
            &signal_file,
            rerouted,
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

/// Launch one fresh foreground harness session owning the real TTY, then
/// watch it while it runs (see [`wait_with_watcher`]). Spawned directly — no
/// shell, no PID-export trick; the `Child` already carries its own pid, and
/// the driver signals it directly once the completion file appears.
/// `GROVE_HARNESS_BIN` overrides the binary (testing / wrapping `claude`).
///
/// `rerouted` says whether `harness` differs from the grove's stamped one
/// (a per-kind `GROVE_<KIND>_HARNESS` override fired) — threaded through to
/// [`harness_bin`], which must not let the legacy unscoped `GROVE_HARNESS_BIN`
/// leak into a rerouted launch (B5).
///
/// Prints one diagnostic line naming the resolved `(harness, model)` on every
/// launch, routed or not: the trial's central invariant ("K3 reviews
/// everywhere") is otherwise unobservable at runtime, and a typo in a var
/// *name* (e.g. `GROVE_REVIEWS_HARNESS`) would silently produce zero routing
/// effect for a whole month with nothing to notice.
fn launch_session(
    harness: &Harness,
    worktree: &Path,
    session_name: &str,
    prompt: &str,
    signal_file: &Path,
    rerouted: bool,
    model: Option<&str>,
) -> Result<()> {
    let bin = harness_bin(harness, rerouted);
    eprintln!(
        "grove: launching {} (model: {})",
        harness.name,
        model.unwrap_or("default")
    );

    let mut cmd = Command::new(&bin);
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
    // Neither handle is exported any more (driver-side-kill-k2) — the agent
    // never needs its own PID, since the driver kills its own child directly.
    // Scrub rather than simply not-setting: a `grove do` launched from inside
    // a session that itself inherited one of these (e.g. nested groves) must
    // not leak a stale, unrelated PID into the new harness session.
    cmd.env_remove("GROVE_HARNESS_PID");
    cmd.env_remove("GROVE_CLAUDE_PID");

    let child = cmd.spawn().context("launching the harness session")?;
    wait_with_watcher(child, signal_file)
}

/// The watcher's state machine: idle until the completion signal file
/// appears, then timed toward SIGTERM and finally SIGKILL.
enum Watch {
    Running,
    Signalled(Instant),
    Terminated(Instant),
}

/// How often the watcher checks the child's liveness and the signal file.
/// Not a tunable seam — `GROVE_KILL_GRACE`/`GROVE_KILL_GRACE_KILL` below are
/// the only knobs a test (or an operator) needs.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

/// Default seconds after the completion signal file appears before the
/// watcher sends SIGTERM (lets the agent's `complete` Bash-tool call return,
/// and the agent's turn end, before its session dies).
const DEFAULT_GRACE: f64 = 2.0;
/// Default seconds after SIGTERM before the watcher escalates to SIGKILL.
const DEFAULT_KILL_GRACE: f64 = 5.0;

/// Watch a spawned harness session while it runs: poll for the completion
/// signal file alongside the child's own exit (`try_wait`), and once the file
/// appears, apply grace → SIGTERM → kill-grace → SIGKILL — the out-of-band
/// kill an interactive harness cannot perform on itself (self-driving-loop).
/// This is the *driver's* job, not the agent's: the driver is the harness's
/// own parent process, outside whatever sandbox the harness runs under, so it
/// can always signal its child — codex's Seatbelt sandbox, for one, denies a
/// same-sandbox process from signalling its own session, which is why the
/// previous in-agent self-kill silently failed there.
fn wait_with_watcher(mut child: Child, signal_file: &Path) -> Result<()> {
    let (grace, kill_grace) = kill_graces();
    let mut watch = Watch::Running;
    loop {
        if child
            .try_wait()
            .context("waiting on the harness session")?
            .is_some()
        {
            // A completion kill makes the harness exit non-zero (or via
            // signal); that is the normal exit path, not an error. The signal
            // file — not the exit status — decides relaunch.
            return Ok(());
        }
        watch = match watch {
            Watch::Running if signal_file.exists() => Watch::Signalled(Instant::now()),
            Watch::Signalled(at) if at.elapsed() >= grace => {
                // SAFETY: kill(2) on a pid this process itself spawned. A
                // failure (e.g. ESRCH, the child already exited) is ignored,
                // same as the shell `kill ... 2>/dev/null` this replaces —
                // the next `try_wait` above will catch that exit anyway.
                unsafe { libc::kill(child.id() as i32, libc::SIGTERM) };
                Watch::Terminated(Instant::now())
            }
            Watch::Terminated(at) if at.elapsed() >= kill_grace => {
                unsafe { libc::kill(child.id() as i32, libc::SIGKILL) };
                child.wait().context("reaping the killed harness session")?;
                return Ok(());
            }
            other => other,
        };
        std::thread::sleep(POLL_INTERVAL);
    }
}

/// Read `GROVE_KILL_GRACE`/`GROVE_KILL_GRACE_KILL`, falling back to the
/// built-in defaults — the two knobs a test (or an operator) has to keep the
/// watcher's timing fast or tunable. Negative values are clamped to zero
/// rather than reaching `Duration::from_secs_f64`, which panics on them.
fn kill_graces() -> (Duration, Duration) {
    let grace = env_parse_f64("GROVE_KILL_GRACE")
        .unwrap_or(DEFAULT_GRACE)
        .max(0.0);
    let kill_grace = env_parse_f64("GROVE_KILL_GRACE_KILL")
        .unwrap_or(DEFAULT_KILL_GRACE)
        .max(0.0);
    (
        Duration::from_secs_f64(grace),
        Duration::from_secs_f64(kill_grace),
    )
}

fn env_parse_f64(key: &str) -> Option<f64> {
    std::env::var(key).ok().and_then(|s| s.trim().parse().ok())
}

/// The binary to exec for a harness: `GROVE_HARNESS_BIN_<NAME>` (the
/// per-harness test seam — kind routing can launch two different harnesses in
/// one loop) always wins. The legacy unscoped `GROVE_HARNESS_BIN` beats
/// `exec_bin` only when `harness` is the one the grove is stamped to
/// (`!rerouted`) — once a per-kind override reroutes to a *different*
/// harness, a single global bin override is incoherent (B5: it would exec the
/// stamped harness's wrapper under the rerouted harness's flag template), so
/// a reroute falls straight to `exec_bin` instead. Both env seams treat `""`
/// as unset via [`env_nonempty`], like every other var in this file.
fn harness_bin(harness: &Harness, rerouted: bool) -> String {
    let scoped = env_nonempty(&format!(
        "GROVE_HARNESS_BIN_{}",
        harness.name.to_uppercase()
    ));
    if rerouted {
        return scoped.unwrap_or_else(|| harness.exec_bin.to_string());
    }
    scoped
        .or_else(|| env_nonempty("GROVE_HARNESS_BIN"))
        .unwrap_or_else(|| harness.exec_bin.to_string())
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
/// (`GROVE_<HARNESS>_<KIND>_MODEL`) beats the base (`GROVE_<KIND>_MODEL`) —
/// but only when `harness` is the one the grove is stamped to (`!rerouted`).
/// Scoped because one shared value cannot serve two harnesses — a codex
/// profile name is garbage to pi and vice versa (B2) — and that is exactly
/// what a reroute puts at risk: the base var was written with *some* harness
/// in mind (typically the stamped one), so once a per-kind override sends
/// this kind to a *different* harness, the base var must not follow it there.
/// A rerouted launch with no scoped var set gets no `--model` at all, never
/// the base var's value.
fn model_for(harness: &Harness, kind: Kind, rerouted: bool) -> Option<String> {
    let h = harness.name.to_uppercase();
    let s = env_suffix(kind);
    let scoped = env_nonempty(&format!("GROVE_{h}_{s}_MODEL"));
    if rerouted {
        return scoped;
    }
    scoped.or_else(|| env_nonempty(&format!("GROVE_{s}_MODEL")))
}

/// Whether any model env var (scoped-to-this-harness or base) is set — the
/// gate that keeps the common unconfigured path a zero-subprocess launch.
fn any_model_env(harness: &Harness) -> bool {
    let h = harness.name.to_uppercase();
    KIND_SUFFIXES.iter().any(|s| {
        env_nonempty(&format!("GROVE_{h}_{s}_MODEL")).is_some()
            || env_nonempty(&format!("GROVE_{s}_MODEL")).is_some()
    })
}

/// Validate and resolve one `GROVE_<SUFFIX>_HARNESS` var. Shared by
/// `harness_override` (the picked kind's var, used to route) and
/// `validate_all_harness_overrides` (every kind's var, used only to fail
/// loudly on a typo). Unknown names fail loudly — a typo that silently fell
/// back would misroute every review.
fn checked_harness_override(suffix: &str) -> Result<Option<&'static Harness>> {
    let var = format!("GROVE_{suffix}_HARNESS");
    let Some(name) = env_nonempty(&var) else {
        return Ok(None);
    };
    crate::harness::by_name(&name).map(Some).ok_or_else(|| {
        anyhow::anyhow!(
            "{var}={name}: unknown harness. Known: {}",
            crate::harness::known_names()
        )
    })
}

/// The per-kind harness override: `GROVE_<KIND>_HARNESS` names the harness
/// that runs leaves of that kind, whatever the grove is stamped to (the
/// trial's "K3 reviews everywhere": GROVE_REVIEW_HARNESS=pi).
fn harness_override(kind: Kind) -> Result<Option<&'static Harness>> {
    checked_harness_override(env_suffix(kind))
}

/// Whether any per-kind harness override is set — like `any_model_env`, the
/// gate that decides whether the kind peek is worth a subprocess.
fn any_harness_override_env() -> bool {
    KIND_SUFFIXES
        .iter()
        .any(|s| env_nonempty(&format!("GROVE_{s}_HARNESS")).is_some())
}

/// Validate every `GROVE_<KIND>_HARNESS` override up front, not just the
/// picked leaf's kind — a typo in a var for a *different* kind (e.g.
/// `GROVE_PLANNING_HARNESS=lemur` while today's leaf is `work`) would
/// otherwise pass silently and only surface hours later, once a planning leaf
/// is finally picked. `any_harness_override_env` already sweeps all five
/// suffixes to decide whether routing applies at all, so validating them here
/// too is nearly free.
fn validate_all_harness_overrides() -> Result<()> {
    for suffix in KIND_SUFFIXES {
        checked_harness_override(suffix)?;
    }
    Ok(())
}

/// Resolve where and on what the next session launches: peek the picked
/// leaf's kind (only when some routing env makes it matter), apply the
/// per-kind harness override, then resolve the model against the
/// *post-override* harness — so `GROVE_PI_REVIEW_MODEL` governs a review
/// rerouted to pi, not the stamped harness's vars. Returns whether a reroute
/// happened (`launch` differs from `stamped`), threaded through to
/// `harness_bin`/`model_for` so neither lets a stamped-harness-scoped
/// fallback leak into the rerouted launch.
fn resolve_launch(
    stamped: &'static Harness,
    worktree: &Path,
    verb: &str,
) -> Result<(&'static Harness, Option<String>, bool)> {
    if !any_harness_override_env() && !any_model_env(stamped) {
        return Ok((stamped, None, false));
    }
    validate_all_harness_overrides()?;
    let kind = if verb == "start" {
        KindPeek::Kind(Kind::Planning)
    } else {
        resolve_kind(worktree)
    };
    let kind = match kind {
        KindPeek::Kind(k) => k,
        // Empty grove (no live leaf — the finish-cycle iteration): no leaf to
        // route, silently launch on the stamped harness. Legitimately nothing
        // to select on, unlike a degraded peek below.
        KindPeek::Empty => return Ok((stamped, None, false)),
        // The peek itself failed (missing grove-llm, non-zero exit,
        // unparseable output) — genuinely unknown, not "nothing to route".
        // With a harness override active, silently launching on the stamped
        // harness would be exactly the silent misroute `harness_override`'s
        // unknown-name case already refuses (B6) — so refuse here too rather
        // than possibly running e.g. a review on the harness it was routed
        // *away* from. Model-only configs (no harness override) still
        // degrade gracefully: a missing model selection is a nicety, not a
        // routing-safety issue.
        KindPeek::Degraded => {
            if any_harness_override_env() {
                anyhow::bail!(
                    "grove: the next leaf's kind could not be determined (see the \
                     diagnostic above), and a per-kind harness override \
                     (GROVE_<KIND>_HARNESS) is configured — refusing to silently \
                     launch on the stamped harness, which could run this leaf on \
                     the wrong harness for the whole iteration. Fix `grove-llm kind` \
                     and re-run."
                );
            }
            return Ok((stamped, None, false));
        }
    };
    let launch = harness_override(kind)?.unwrap_or(stamped);
    let rerouted = launch.name != stamped.name;
    let model = if launch.model_args.is_empty() {
        None
    } else {
        model_for(launch, kind, rerouted)
    };
    Ok((launch, model, rerouted))
}

/// Read an env var, treating an empty string as unset — the convention every
/// env seam in this file follows (`GROVE_HARNESS_BIN` was the one holdout,
/// B5).
fn env_nonempty(var: &str) -> Option<String> {
    std::env::var(var).ok().filter(|s| !s.is_empty())
}

/// The outcome of peeking the next live leaf's kind — three genuinely
/// different situations `resolve_launch` must not conflate (B6): `Empty` has
/// no leaf to route *on purpose* (the finish-cycle iteration), while
/// `Degraded` means the peek itself failed and the kind is simply unknown. A
/// harness override active during `Degraded` cannot be silently honoured
/// *or* silently dropped by guessing — `resolve_launch` refuses instead.
enum KindPeek {
    Kind(Kind),
    Empty,
    Degraded,
}

/// Peek the next live leaf's kind by running `grove-llm kind` against the
/// worktree — the same verb (and code path) the launched agent would call.
/// Never a reason to jam the loop on its own, so any failure (binary missing,
/// non-zero exit, unparseable output) yields [`KindPeek::Degraded`] with a
/// diagnostic, rather than erroring here — `resolve_launch` decides what a
/// degrade means (silently default, or refuse) since only it knows whether a
/// harness override is at stake. The child's stderr is inherited, not
/// captured — see the note on the spawn below.
fn resolve_kind(worktree: &Path) -> KindPeek {
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
                return KindPeek::Empty; // empty grove: no kind to select on
            }
            match Kind::parse(token) {
                Ok(k) => KindPeek::Kind(k),
                Err(_) => {
                    // Not expected from a real `grove-llm kind` (it self-degrades
                    // unparseable task-file tokens to `work` before printing) —
                    // this guards against a mismatched/stale `grove-llm` binary.
                    eprintln!("grove: unrecognized task kind {token:?} from `grove-llm kind`");
                    KindPeek::Degraded
                }
            }
        }
        Ok(_) => {
            // The child's own diagnostic already reached stderr (inherited above).
            eprintln!("grove: `grove-llm kind` failed");
            KindPeek::Degraded
        }
        Err(e) => {
            eprintln!("grove: could not run grove-llm for model/harness selection ({e})");
            KindPeek::Degraded
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

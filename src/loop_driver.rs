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
// (constraint 6, walk-away-able); routing (model-per-task-kind) is just two
// lookups off the picked leaf before the launch:
//
//     sig="$TMPDIR/grove-loop-<name>-<worktree-identity-hash>.signal"
//     while :; do
//       v=$(grove-llm --version | awk '{print $NF}')     # version-skew guard
//       [ -n "$v" ] && [ "$v" != "<own compiled-in version>" ] && break
//       rm -f "$sig"
//       # One peek, two facts: the kind on line 1, and the harness the leaf
//       # declares for itself on line 2 when it carries a `**Harness:**` line.
//       # A declaration beats every policy var and the stamp.
//       if [ -d "$wt/.grove" ]; then
//         peek=$(grove-llm kind --with-harness) || exit 1
//         kind=$(printf '%s\n' "$peek" | sed -n 1p)
//         leaf_harness=$(printf '%s\n' "$peek" | sed -n 2p)
//       else
//         kind=requirements; leaf_harness=
//       fi
//       # kind → env suffix: uppercase, `-` → `_` (review-impl ⇒ REVIEW_IMPL).
//       # An empty $kind is an empty grove — no leaf, so nothing to require a
//       # model *for*; every other kind must resolve one.
//       suffix=$(printf '%s' "$kind" | tr 'a-z-' 'A-Z_')
//       eval model="\$GROVE_${suffix}_MODEL"
//       # …then the family var, for the ten kinds that have one. Note the
//       # longest family matches first — INTEGRATE_REVIEW_IMPL is not a REVIEW.
//       case "$suffix" in
//         INTEGRATE_REVIEW_*) fam=INTEGRATE_REVIEW ;;
//         REVIEW_*)           fam=REVIEW ;;
//         *)                  fam= ;;
//       esac
//       [ -n "$model" ] || [ -z "$fam" ] || eval model="\$GROVE_${fam}_MODEL"
//       # Harness: leaf beats kind beats family beats stamp. (The real driver
//       # then re-resolves $model against the *launching* harness, and a
//       # rerouted launch consults no unscoped var at all.)
//       eval policy_h="\$GROVE_${suffix}_HARNESS"
//       [ -n "$policy_h" ] || [ -z "$fam" ] || eval policy_h="\$GROVE_${fam}_HARNESS"
//       harness=${leaf_harness:-${policy_h:-$stamped}}
//       # A kind that resolves no model is a configuration error, not a
//       # fall-through to the harness's own default (model-per-task-kind).
//       [ -z "$kind" ] || [ -n "$model" ] || { echo "no model for $kind" >&2; exit 1; }
//       [ -n "$model" ] && set -- --model "$model" "$prompt" || set -- "$prompt"
//       GROVE_SIGNAL_FILE="$sig" "$(bin_for "$harness")" "$@" &
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
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsString;
use std::hash::{Hash, Hasher};
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
    /// crash), or a pre-launch guard declined to start the next session (a
    /// version-skewed `grove-llm` — driver-version-skew-k11); resumable by
    /// re-running `grove do` from the same working tree.
    Stopped,
}

/// Relaunch-signal file path for a grove. Lives in the temp dir (ephemeral
/// loop IPC, not durable grove state — signal-file-identity-k6) — keyed on
/// `name` **and** the worktree's identity, so two `grove do` loops in
/// different repos whose worktree basenames happen to collide (generic names
/// like `bugs`/`plan`/`docs` are the norm) never share a file: a foreign
/// write from one would otherwise be read as the other's own completion
/// signal, SIGTERM its session mid-work, and misdirect the relaunch decision.
/// `name` alone stays in the filename for operator legibility (`ls $TMPDIR`
/// still reads as grove names); the hash is what actually disambiguates.
/// Cleared at the start of every iteration.
pub fn signal_file_path(worktree: &Path, name: &str) -> PathBuf {
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
    // Canonicalise so the same worktree reached by two different paths (a
    // symlink, a relative vs. absolute cwd) still hashes identically; fall
    // back to the raw path on failure (e.g. a test fixture that races the
    // directory's own creation) rather than erroring the whole loop over a
    // signal-file naming nicety.
    let identity = worktree
        .canonicalize()
        .unwrap_or_else(|_| worktree.to_path_buf());
    let mut hasher = DefaultHasher::new();
    identity.hash(&mut hasher);
    std::env::temp_dir().join(format!("grove-loop-{safe}-{:016x}.signal", hasher.finish()))
}

/// Entry point: install the signal handlers, then run the loop. The real
/// `grove do` path calls this; tests call [`run_loop`] directly to avoid the
/// process-global signal changes. The outcome is already reported on stderr by
/// the loop body, so the caller can discard it.
///
/// An `Err` out of the loop is a report site too (herdr-optional-ui): it can
/// fire hours in, unattended — a per-kind harness override against an
/// undeterminable leaf kind, say — and it leaves the loop parked with live
/// leaves, which is precisely what herdr should be showing as `blocked`.
pub fn run(harness: &'static Harness, repo_path: &Path, worktree: &Path, name: &str) -> Result<()> {
    ignore_interrupts();
    install_termination_handler();
    match run_loop(harness, repo_path, worktree, name) {
        Ok(_) => Ok(()),
        Err(e) => {
            crate::herdr::apply(crate::herdr::plan_for(crate::herdr::Stop::Failed));
            Err(e)
        }
    }
}

/// The loop body, free of process-global side effects. Returns why it stopped
/// ([`LoopOutcome`]) so a clean finish is distinguishable from an abnormal exit.
pub fn run_loop(
    harness: &'static Harness,
    repo_path: &Path,
    worktree: &Path,
    name: &str,
) -> Result<LoopOutcome> {
    let signal_file = signal_file_path(worktree, name);
    let repo_name = repo_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repo".to_string());
    let session_name = format!("{}: {} grove", repo_name, name);

    loop {
        // Version-skew guard (driver-version-skew-k11): before anything else,
        // confirm the `grove-llm` the agent would invoke still matches this
        // driver. Per session, not per driver start — a `brew upgrade`
        // *mid-loop* is exactly the case a start-time check misses.
        if let Some(theirs) = version_skew() {
            eprintln!(
                "grove: version skew — this running driver is grove {DRIVER_VERSION}, but the \
                 `grove-llm` the agent would invoke is {theirs} (the grove binary moved on \
                 disk while this loop was running, e.g. `brew upgrade`)."
            );
            eprintln!(
                "       A skewed driver/agent pair can hang at the completion signal, so the \
                 loop is stopping before the next session."
            );
            eprintln!(
                "       Re-run `grove do` from this working tree to continue on the new \
                 binary (restart ≡ continuation)."
            );
            crate::herdr::apply(crate::herdr::plan_for(crate::herdr::Stop::VersionSkew));
            let _ = std::fs::remove_file(&signal_file);
            return Ok(LoopOutcome::Stopped);
        }

        // Clear any stale signal from the previous iteration.
        let _ = std::fs::remove_file(&signal_file);

        let verb = launch_verb(worktree);

        // Route the launch by the picked leaf's kind: per-kind harness
        // override first (GROVE_<KIND>_HARNESS), then model-per-task-kind
        // against whichever harness actually launches. Resolved *before*
        // loading the prompt (B7): the prompt must come from the harness that
        // actually launches, not the stamped one — reading it first would
        // silently serve the wrong harness's copy whenever a reroute happens.
        let launch = resolve_launch(harness, worktree, verb)?;
        let prompt = crate::launch::load_prompt(launch.harness, verb)?;

        // Report site 1 of 4 (herdr-optional-ui). Before the spawn, not after:
        // the pane is about to be busy, and the other three sites all describe
        // a loop that has stopped. Nothing else is reported until then — a
        // relaunch keeps the pane `working` without a redundant round trip.
        crate::herdr::report(crate::herdr::State::Working);

        let ended = launch_session(
            launch.harness,
            worktree,
            &session_name,
            &prompt,
            &signal_file,
            launch.rerouted,
            launch.model.as_deref(),
        )?;

        // A SIGTERM'd TUI can leave the terminal in raw mode / the alternate
        // screen; reset before relaunching (and on the way out).
        reset_terminal();

        // Report site 2: the driver itself was signalled. Checked before the
        // signal file, because an interrupt mid-session usually leaves no
        // signal file at all and must not be read as the human's `/exit`.
        if ended == SessionEnd::Interrupted {
            eprintln!("grove: interrupted — releasing the herdr pane and stopping the loop.");
            eprintln!(
                "       Re-run `grove do` from this working tree to resume (restart ≡ continuation)."
            );
            crate::herdr::apply(crate::herdr::plan_for(crate::herdr::Stop::Interrupted));
            let _ = std::fs::remove_file(&signal_file);
            return Ok(LoopOutcome::Stopped);
        }

        // Report sites 3 and 4: what the finished session left in the signal
        // file decides both the loop's next move and the pane's state, from the
        // one table in `herdr::plan_for`.
        let signal = complete::read_signal(&signal_file);
        crate::herdr::apply(crate::herdr::plan_for(crate::herdr::Stop::Signal(signal)));

        match signal {
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
) -> Result<SessionEnd> {
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
    // codex-gitdir-grant: reopen the VCS store the sandbox blocks (git's
    // carved-out gitdir; a jj tree's main-workspace store) so the session can
    // commit; a no-op for every other harness.
    crate::launch::append_codex_vcs_store_grant(&mut cmd, harness, worktree)?;
    // herdr-turn-boundary-hooks: ask the harness to report the turn boundaries
    // the driver cannot see. Only here, never in `launch::exec_harness` — a
    // `grove retire` session sets no signal file, so every turn end there would
    // look unsignalled and report `blocked` with no driver to correct it.
    // Resolved against `grove_llm_bin()`, the same binary the agent's own verbs
    // run, so the hook cannot drift from the driver that injected it.
    crate::launch::append_turn_hooks(&mut cmd, harness, Path::new(&grove_llm_bin()));
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

/// How a watched session ended: normally (whatever it left in the signal file
/// decides the rest), or because the *driver* was signalled — a case the signal
/// file cannot express, since an interrupt normally leaves none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionEnd {
    Exited,
    Interrupted,
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
///
/// A caught SIGTERM/SIGHUP also lands here: the handler only flips
/// [`TERMINATED`], and this poll loop is what acts on it — forwarding the
/// signal to the child and letting the existing escalation reap it. That
/// ordering is deliberate. Releasing grove's herdr authority means a socket
/// connect, a write and a read, none of which are async-signal-safe, so none of
/// them may happen inside the handler; the driver already polls on a
/// [`POLL_INTERVAL`] tick, so an atomic flag read there costs nothing and runs
/// the release on a normal stack.
fn wait_with_watcher(mut child: Child, signal_file: &Path) -> Result<SessionEnd> {
    let (grace, kill_grace) = kill_graces();
    let mut watch = Watch::Running;
    let mut interrupted = false;
    loop {
        if child
            .try_wait()
            .context("waiting on the harness session")?
            .is_some()
        {
            // A completion kill makes the harness exit non-zero (or via
            // signal); that is the normal exit path, not an error. The signal
            // file — not the exit status — decides relaunch.
            return Ok(if interrupted {
                SessionEnd::Interrupted
            } else {
                SessionEnd::Exited
            });
        }
        // A signalled driver forwards the signal to its child and hands over to
        // the same escalation the completion path uses, so a harness that
        // ignores SIGTERM still gets SIGKILL'd rather than orphaned onto the
        // TTY. Latched, because the driver must still return `Interrupted` on
        // the iteration where the child finally exits.
        if !interrupted && TERMINATED.load(std::sync::atomic::Ordering::Relaxed) {
            interrupted = true;
            unsafe { libc::kill(child.id() as i32, libc::SIGTERM) };
            watch = Watch::Terminated(Instant::now());
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
                return Ok(if interrupted {
                    SessionEnd::Interrupted
                } else {
                    SessionEnd::Exited
                });
            }
            other => other,
        };
        std::thread::sleep(POLL_INTERVAL);
    }
}

/// Upper bound on either grace. A session-end grace beyond this is operator
/// error, not intent, and letting one through would hang the loop for the rest
/// of the day with no diagnostic — the cap keeps a typo recoverable.
const MAX_GRACE_SECS: f64 = 3600.0;

/// Read `GROVE_KILL_GRACE`/`GROVE_KILL_GRACE_KILL`, falling back to the
/// built-in defaults — the two knobs a test (or an operator) has to keep the
/// watcher's timing fast or tunable. See [`grace_secs`] for why the value is
/// sanitised rather than passed straight to `Duration::from_secs_f64`.
fn kill_graces() -> (Duration, Duration) {
    (
        grace_secs("GROVE_KILL_GRACE", DEFAULT_GRACE),
        grace_secs("GROVE_KILL_GRACE_KILL", DEFAULT_KILL_GRACE),
    )
}

fn grace_secs(key: &str, default: f64) -> Duration {
    sanitise_grace(env_parse_f64(key), default)
}

/// One operator-supplied grace, sanitised into a `Duration`.
/// `Duration::from_secs_f64` panics on NaN, on negatives, *and* on values too
/// large to represent — so a bare `.max(0.0)` clamp is not enough: it lets
/// `GROVE_KILL_GRACE=inf` (or `1e300`) through to panic the driver mid-loop.
/// Non-finite input falls back to the default (it expresses no intent); finite
/// input is clamped into `[0, MAX_GRACE_SECS]`. Split from the env read so the
/// sanitising is testable without process-global `set_var`.
fn sanitise_grace(value: Option<f64>, default: f64) -> Duration {
    let secs = value
        .filter(|s| s.is_finite())
        .unwrap_or(default)
        .clamp(0.0, MAX_GRACE_SECS);
    Duration::from_secs_f64(secs)
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

/// A kind or family label's env-name suffix: uppercased with `-` mapped to `_`
/// (`review-impl` ⇒ `REVIEW_IMPL`, `integrate-review` ⇒ `INTEGRATE_REVIEW`).
/// Derived from `Kind::label`/`Family::label` rather than tabulated beside
/// them, so the nineteen suffixes cannot drift from the nineteen labels — the
/// failure mode being a var name the user writes correctly and grove never
/// reads. The grammar is unambiguous because harness names, kind labels and
/// family labels share no token (`docs/specs/task-kind-taxonomy.md`).
fn env_suffix(label: &str) -> String {
    label.to_uppercase().replace('-', "_")
}

/// A kind's env-name suffixes **in precedence order along the kind axis**: its
/// own, then its family's when it has one (`REVIEW_IMPL`, then `REVIEW`). The
/// single statement of "specific beats general" on that axis — both routing
/// axes read this list rather than restating the order, so the harness var and
/// the model var cannot come to disagree about what beats what.
///
/// A kind with no family yields one suffix and resolves exactly as it did
/// before families existed.
fn kind_suffixes(kind: Kind) -> Vec<String> {
    let mut suffixes = vec![env_suffix(kind.label())];
    if let Some(family) = kind.family() {
        suffixes.push(env_suffix(family.label()));
    }
    suffixes
}

/// Every model env key for a `(harness, kind)` pair, **in the order routing
/// consults them** — the four-key lattice, harness-major
/// (`model-per-task-kind`):
///
/// 1. `GROVE_<HARNESS>_<KIND>_MODEL`
/// 2. `GROVE_<HARNESS>_<FAMILY>_MODEL`
/// 3. `GROVE_<KIND>_MODEL`
/// 4. `GROVE_<FAMILY>_MODEL`
///
/// The harness axis outranks the kind axis because the two are different
/// *kinds* of constraint. Crossing the harness axis can yield a value that is
/// not merely less specific but **invalid** for the binary being launched — a
/// codex profile name is garbage to pi (B2) — while a family's model is less
/// specific yet still the user's own choice, and still valid. Kind-major
/// ordering would let a set harness-scoped family var lose to an unscoped
/// exact-kind var written with a different harness in mind, which is the
/// precise failure the harness axis exists to prevent.
///
/// A **rerouted** launch (`harness` is not the one the grove is stamped to)
/// consults no unscoped var at all: the list truncates to keys 1–2, so a
/// reroute never inherits the base var's value. Harness-major ordering is what
/// makes that a truncation of one ordering rather than a switch to a different
/// one.
///
/// One list, two readers, deliberately: [`model_for`] resolves *through* it and
/// [`missing_model_error`] *prints* it. Now that a model is required, the error
/// is the operator's only instruction — and an error listing a key the lattice
/// does not actually consult would send them to set a var that changes nothing.
fn model_keys(harness: &Harness, kind: Kind, rerouted: bool) -> Vec<String> {
    let h = harness.name.to_uppercase();
    let suffixes = kind_suffixes(kind);
    let mut keys: Vec<String> = suffixes
        .iter()
        .map(|s| format!("GROVE_{h}_{s}_MODEL"))
        .collect();
    if !rerouted {
        keys.extend(suffixes.iter().map(|s| format!("GROVE_{s}_MODEL")));
    }
    keys
}

/// The model value for a kind on a harness: the first key of
/// [`model_keys`] the environment actually sets. `None` is now a **failure**,
/// not a launch without `--model` — see [`missing_model_error`].
fn model_for(harness: &Harness, kind: Kind, rerouted: bool) -> Option<String> {
    model_keys(harness, kind, rerouted)
        .iter()
        .find_map(|key| env_nonempty(key))
}

/// The required-model failure (`model-per-task-kind`: *a kind with no model is
/// a configuration error, not a default*). Falling through to the harness's own
/// default is still grove deciding which model runs a `review-impl` leaf — it
/// just decides invisibly, and leaves partial configuration indistinguishable
/// from complete configuration. This is the one error message that makes that
/// state representable, so it names all three things the operator needs: the
/// kind, the harness it would have launched on, and every var that would
/// satisfy the requirement, most specific first.
///
/// The reroute note is not decoration. A rerouted launch consults no unscoped
/// key, so an operator staring at a set `GROVE_REVIEW_MODEL` and an error
/// saying "no model configured" would otherwise have no way to tell that their
/// var was seen and deliberately declined.
fn missing_model_error(harness: &Harness, kind: Kind, rerouted: bool) -> anyhow::Error {
    let keys: String = model_keys(harness, kind, rerouted)
        .into_iter()
        .map(|key| format!("\n  {key}"))
        .collect();
    let reroute_note = if rerouted {
        format!(
            "\n\nThe unscoped keys (GROVE_<KIND>_MODEL, GROVE_<FAMILY>_MODEL) do not apply \
             here: this leaf is rerouted to {}, which this grove is not stamped to, and an \
             unscoped value was written with some other harness in mind.",
            harness.name
        )
    } else {
        String::new()
    };
    anyhow::anyhow!(
        "grove: the next leaf's kind is `{}`, and no model is configured for it on {} — \
         model selection is required, so grove will not silently launch on the harness's \
         own default (model-per-task-kind). Set one of, most specific first:{keys}{reroute_note}",
        kind.label(),
        harness.name,
    )
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
/// that runs leaves of that kind, whatever the grove is stamped to
/// (`GROVE_REVIEW_IMPL_HARNESS=codex` sends code reviews to codex), falling
/// back to the family var (`GROVE_REVIEW_HARNESS`) so one line covers all five
/// of a family's kinds.
///
/// Two keys, not four: scoping a *harness* choice by harness is meaningless,
/// so this axis has only the kind dimension the [`kind_suffixes`] order
/// already states.
fn harness_override(kind: Kind) -> Result<Option<&'static Harness>> {
    for suffix in kind_suffixes(kind) {
        if let Some(harness) = checked_harness_override(&suffix)? {
            return Ok(Some(harness));
        }
    }
    Ok(None)
}

/// Every harness override the environment actually sets, as
/// `(var name, what it routes, harness)` — the shared reader behind
/// [`validate_all_harness_overrides`] and [`preflight_check`]. Sweeping the
/// *vars* rather than the kinds is what lets both see a family var that no
/// kind currently resolves through (every member overriding it individually),
/// which is still a configured route to a harness and still worth failing on.
fn configured_harness_overrides() -> Result<Vec<(String, String, &'static Harness)>> {
    let mut found = Vec::new();
    for kind in Kind::ALL {
        let suffix = env_suffix(kind.label());
        if let Some(harness) = checked_harness_override(&suffix)? {
            found.push((
                format!("GROVE_{suffix}_HARNESS"),
                // The kind's own label, not the suffix lowercased — the two
                // differ on every hyphenated kind (`REVIEW_IMPL` would print
                // as `review_impl`, which is not a kind anyone can write).
                kind.label().to_string(),
                harness,
            ));
        }
    }
    for family in crate::leaf::Family::ALL {
        let suffix = env_suffix(family.label());
        if let Some(harness) = checked_harness_override(&suffix)? {
            found.push((
                format!("GROVE_{suffix}_HARNESS"),
                format!("{}-*", family.label()),
                harness,
            ));
        }
    }
    Ok(found)
}

/// Validate every harness override up front, not just the picked leaf's kind —
/// a typo in a var for a *different* kind (e.g. `GROVE_PLANNING_HARNESS=lemur`
/// while today's leaf is `impl`) would otherwise pass silently and only
/// surface hours later, once a planning leaf is finally picked. The same holds
/// for a family var, which is worse: it is the var a user is most likely to set
/// exactly once and never look at again.
///
/// Runs before the kind peek in [`resolve_launch`], so a typo'd override still
/// fails on its own terms even in the cases the peek would refuse anyway (an
/// empty grove, a degraded peek) — a "could not determine the kind" diagnostic
/// would be a red herring for an operator whose real mistake was spelling a
/// harness name wrong.
fn validate_all_harness_overrides() -> Result<()> {
    configured_harness_overrides().map(|_| ())
}

/// Pre-flight: verify every harness this grove's launch might actually need
/// resolves to a real binary — the stamped harness, plus any harness named by
/// a configured override, per-kind (`GROVE_<KIND>_HARNESS`) or per-family
/// (`GROVE_<FAMILY>_HARNESS`) — before `do_grove` commits to anything
/// (harness-spawn-preflight-k8). Checking only the stamped harness let a
/// rerouted-but-uninstalled one (e.g. `GROVE_REVIEW_HARNESS=pi` against a
/// codex-stamped grove with no `pi` installed) pass pre-flight, run happily
/// for however long, and only die mid-loop the moment a leaf of that kind was
/// finally picked. Each harness is resolved through [`harness_bin`] — the same
/// effective-binary lookup `launch_session` actually execs — so a
/// `GROVE_HARNESS_BIN` / `GROVE_HARNESS_BIN_<NAME>` test-seam override pointing
/// at a nonexistent path is caught here too, not just a harness with no
/// override at all.
///
/// **Per-leaf declarations are deliberately not swept here** (leaf-harness-k15).
/// Walking the live leaves for `**Harness:**` lines was the alternative, and it
/// cannot deliver the property this function exists for: the tree *grows while
/// the loop runs*, so a pre-flight snapshot is silent about every leaf a
/// planning session writes an hour from now — which is most of them. What it
/// would add is a duplicate of a check that has to exist at launch anyway
/// ([`leaf_harness_installed`]), paid for with a full tree read on every
/// `grove do`, reporting a leaf that may be many sessions away. The env axis is
/// static and knowable up front, which is exactly why it *is* swept here; the
/// leaf axis is not, so it is checked at the one moment it is known.
pub fn preflight_check(stamped: &'static Harness) -> Result<()> {
    let stamped_bin = harness_bin(stamped, false);
    if !crate::harness::exec_bin_on_path(&stamped_bin) {
        anyhow::bail!(
            "{} is not on PATH — install it before binding this grove to \"{}\" \
             (nothing was stamped; run again once it's installed)",
            stamped_bin,
            stamped.name
        );
    }
    // Sweeps the configured *vars* (kinds and families alike), so this both
    // validates every name — the unknown-harness case — and bin-checks every
    // harness one of them could route to.
    for (var, routes, overridden) in configured_harness_overrides()? {
        let rerouted = overridden.name != stamped.name;
        let bin = harness_bin(overridden, rerouted);
        if !crate::harness::exec_bin_on_path(&bin) {
            anyhow::bail!(
                "{var}={}: {} is not on PATH — install it before this grove can \
                 run {routes} leaves on it (nothing was stamped; run again once \
                 it's installed)",
                overridden.name,
                bin,
            );
        }
    }
    Ok(())
}

/// Which launcher prompt the next session runs: a brand-new grove (no
/// `.grove/` yet) bootstraps via `start`, an existing one continues via
/// `continue` (which, on an empty `pick`, proposes the finish cycle
/// in-session). The loop body's only state, and it is re-derived from the
/// filesystem every iteration, so restart ≡ continuation.
///
/// Extracted so the `--no-launch` dry run reads it from here too
/// (`no-launch-config-check-k20`): the report and the launch it predicts must
/// not be able to disagree about which of the two paths the next session takes,
/// since the start path is routed as `requirements` by construction and the
/// continue path by whatever the tree says.
fn launch_verb(worktree: &Path) -> &'static str {
    if worktree.join(".grove").is_dir() {
        "continue"
    } else {
        "start"
    }
}

/// Everything the next session's launch is routed to, resolved from the picked
/// leaf by [`resolve_launch`]: the harness that runs it, the model that harness
/// loads, whether that harness differs from the stamped one, and the kind both
/// axes were keyed on.
///
/// `kind` is `None` in exactly one case — the finish-cycle iteration, which has
/// no live leaf — which is also the one case that requires no model. It rides
/// along because the `--no-launch` dry run *reports* what the launch resolved,
/// and a readiness line that named a model without naming the kind it was chosen
/// for would leave the operator unable to check the choice against their config.
struct Launch {
    harness: &'static Harness,
    model: Option<String>,
    rerouted: bool,
    kind: Option<Kind>,
}

/// Resolve where and on what the next session launches: peek the picked
/// leaf's kind, apply the harness override (its own kind's var, else its
/// family's), then resolve the model against the *post-override* harness — so
/// `GROVE_PI_REVIEW_MODEL` governs a review rerouted to pi, not the stamped
/// harness's vars. The returned [`Launch`] records whether a reroute happened
/// (`harness` differs from `stamped`), threaded through to
/// `harness_bin`/`model_for` so neither lets a stamped-harness-scoped fallback
/// leak into the rerouted launch.
///
/// **Side-effect free, and load-bearingly so**: the `--no-launch` dry run calls
/// this to report what the next launch would do ([`readiness`]), so anything
/// durable done here would happen on a run documented as doing nothing.
///
/// The two axes are resolved in this order for a reason: the family fallback
/// runs along the *kind* axis and must compose with the reroute rule rather
/// than route around it. Falling back `review-impl` → `review` is fine at any
/// point; falling back across harnesses once a reroute has happened is not, and
/// `rerouted` is computed here — after the harness axis has settled, before the
/// model axis is consulted — so the family fallback cannot open a hole in it.
///
/// **The peek is unconditional** (`required-model-vars-k18`). It used to be
/// gated on some routing env making it matter, which kept an unconfigured
/// launch byte-for-byte the pre-feature one; a var that is *required* must be
/// checked on every iteration, so there was nothing left for a short-circuit
/// whose whole purpose is to avoid looking to decide. The zero-subprocess
/// launch went with it, knowingly (`model-per-task-kind`, *Consequences*).
///
/// **A leaf's own `**Harness:**` declaration outranks both env vars**
/// (`leaf-harness-k15`): leaf beats kind beats family beats stamp. It comes off
/// the *same* peek — one subprocess returns both facts — because the peek
/// already runs every iteration and a second one to read a neighbouring line
/// would double that cost for a declaration almost no leaf carries.
fn resolve_launch(stamped: &'static Harness, worktree: &Path, verb: &str) -> Result<Launch> {
    validate_all_harness_overrides()?;
    let peek = if verb == "start" {
        // Start-path is `requirements` by construction: `.grove/` does not exist
        // yet (the agent runs `root-init` inside that session), and root-init's
        // first leaf is always requirements — fresh-grove-start-contract. No
        // leaf exists to declare a harness either, so the per-leaf axis is
        // silent. This is the one launch grove routes without reading a file,
        // so `GROVE_REQUIREMENTS_MODEL` (or a harness-scoped spelling of it) is
        // the single var a brand-new grove cannot start without.
        KindPeek::Leaf(Kind::Requirements, None)
    } else {
        resolve_kind(worktree)
    };
    let (kind, leaf_harness) = match peek {
        KindPeek::Leaf(k, h) => (k, h),
        // Empty grove (no live leaf — the finish-cycle iteration): no leaf to
        // route, and no task to require a model *for*. An absence of the
        // question, not a default — and legitimately nothing to select on,
        // unlike a degraded peek below.
        KindPeek::Empty => {
            return Ok(Launch {
                harness: stamped,
                model: None,
                rerouted: false,
                kind: None,
            })
        }
        // The peek itself failed (missing grove-llm, non-zero exit,
        // unparseable output, or a leaf declaring a harness grove does not
        // know) — genuinely unknown, not "nothing to route". This now bails in
        // **every** case, where it previously bailed only under a configured
        // harness override: the asymmetry that spared a model-only config
        // ("a missing model is a nicety, a misroute is not") died with the
        // requirement below, since an unknown kind can no longer be routed by
        // guessing on either axis (model-per-task-kind).
        KindPeek::Degraded => anyhow::bail!(
            "grove: the next leaf's launch could not be resolved (see the diagnostic \
             above). Every routing axis reads that leaf — the harness it declares \
             for itself, the harness its kind routes to (GROVE_<KIND>_HARNESS / \
             GROVE_<FAMILY>_HARNESS), and the model that harness loads, which is now \
             required (model-per-task-kind) — so grove refuses to guess and launch on \
             the stamped harness. Fix `grove-llm kind` and re-run."
        ),
    };
    // Leaf beats kind beats family beats stamp: a declaration on the leaf is a
    // fact about *this* leaf ("this one goes elsewhere because its sibling does
    // not"), which is strictly more specific than a policy that knows nothing
    // about any tree (`docs/specs/task-kind-taxonomy.md`, *Routing*).
    let launch = match leaf_harness {
        Some(h) => {
            leaf_harness_installed(h, h.name != stamped.name)?;
            h
        }
        None => harness_override(kind)?.unwrap_or(stamped),
    };
    let rerouted = launch.name != stamped.name;
    // A harness whose model-flag template is empty has opted out of model
    // selection entirely; requiring a flag it cannot pass would make it
    // unlaunchable (model-per-task-kind, the exemptions). Note what is *not*
    // exempt alongside it: harness absence. No `GROVE_<KIND>_HARNESS` means the
    // stamped harness — an explicit binding recorded on disk — so the model is
    // still required for it.
    if launch.model_args.is_empty() {
        return Ok(Launch {
            harness: launch,
            model: None,
            rerouted,
            kind: Some(kind),
        });
    }
    let model = model_for(launch, kind, rerouted)
        .ok_or_else(|| missing_model_error(launch, kind, rerouted))?;
    Ok(Launch {
        harness: launch,
        model: Some(model),
        rerouted,
        kind: Some(kind),
    })
}

/// What the next real launch would do, resolved without doing it — the payload
/// of `grove do --no-launch` (`no-launch-config-check-k20`).
///
/// Deliberately produced by the **same** [`resolve_launch`] the loop's next
/// iteration would call, rather than by a parallel "check the config" routine:
/// the flag's whole claim is that the next launch would succeed, and a second
/// implementation of that claim is a second thing to keep in sync. The dry run
/// therefore fails on exactly what a launch fails on — an unknown harness name,
/// a rerouted-but-uninstalled binary, an undeterminable kind, a kind with no
/// model var — and names the same variables in the same words.
pub struct Readiness {
    next: Next,
    harness: &'static Harness,
    model: Option<String>,
}

/// What the next session would be working on — the three states
/// [`resolve_launch`] distinguishes, kept apart here for the same reason it
/// keeps them apart: "no leaf because the tree does not exist yet" and "no leaf
/// because they are all retired" are opposite ends of a grove's life, and a
/// readiness line that rendered both as "no leaf" would be actively misleading.
enum Next {
    /// No `.grove/` yet: the next session bootstraps one, routed as
    /// `requirements` by construction (*fresh-grove-start-contract*).
    Bootstrap(Kind),
    /// The picked live leaf and the kind the launch routes on. `path` is
    /// `None` only when the reporting walk below could not name it.
    Leaf { path: Option<PathBuf>, kind: Kind },
    /// No live leaves left: the next session proposes the finish cycle, and has
    /// no task to require a model for.
    Finish,
}

/// Resolve — without launching, stamping, or otherwise touching anything — what
/// `grove do` would do next. See [`Readiness`]; the caller is `launch::do_grove`'s
/// `--no-launch` branch, which prints it and returns.
pub fn readiness(stamped: &'static Harness, worktree: &Path) -> Result<Readiness> {
    let verb = launch_verb(worktree);
    let launch = resolve_launch(stamped, worktree, verb)?;
    let next = match launch.kind {
        None => Next::Finish,
        Some(kind) if verb == "start" => Next::Bootstrap(kind),
        Some(kind) => Next::Leaf {
            path: picked_leaf(worktree),
            kind,
        },
    };
    Ok(Readiness {
        next,
        harness: launch.harness,
        model: launch.model,
    })
}

/// The leaf the readiness line names, walked **in-process** rather than through
/// a second `grove-llm` call. This is *reporting*, not routing: every fact the
/// launch turns on already came off the peek subprocess above, so a failed walk
/// costs the line a path and nothing else — which is why it degrades to `None`
/// instead of failing a dry run that has otherwise succeeded.
///
/// Relative to the worktree, unlike `grove-llm pick`'s absolute output: the
/// readiness line already opens with the worktree, and repeating it inside the
/// leaf path pushed the informative half off the right of the terminal.
fn picked_leaf(worktree: &Path) -> Option<PathBuf> {
    let leaf = crate::tree_read::pick(&worktree.join(".grove"))
        .ok()
        .flatten()?;
    Some(leaf.strip_prefix(worktree).unwrap_or(&leaf).to_path_buf())
}

impl std::fmt::Display for Readiness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.next {
            Next::Bootstrap(kind) => write!(
                f,
                "no task tree yet: the next session bootstraps one, as {} on {}",
                kind.label(),
                self.harness.name
            )?,
            Next::Leaf {
                path: Some(path),
                kind,
            } => write!(
                f,
                "next leaf {} ({}) on {}",
                path.display(),
                kind.label(),
                self.harness.name
            )?,
            Next::Leaf { path: None, kind } => {
                write!(f, "next leaf ({}) on {}", kind.label(), self.harness.name)?
            }
            Next::Finish => write!(
                f,
                "no live leaves: the next session proposes the finish cycle, on {}",
                self.harness.name
            )?,
        }
        match &self.model {
            Some(model) => write!(f, ", model {model}"),
            // Only the two exemptions reach here: the finish-cycle iteration,
            // and a harness whose model-flag template is empty. Every other
            // launch resolved a model or this line was never reached.
            None => Ok(()),
        }
    }
}

/// Read an env var, treating an empty string as unset — the convention every
/// env seam in this file follows (`GROVE_HARNESS_BIN` was the one holdout,
/// B5).
fn env_nonempty(var: &str) -> Option<String> {
    std::env::var(var).ok().filter(|s| !s.is_empty())
}

/// Refuse a leaf-declared harness that is not installed. Pre-flight cannot
/// cover this axis (see [`preflight_check`]), so the check lands here, at the
/// one moment the declaration is known — and it is *only* on this axis: the
/// env-configured harnesses were resolved once at pre-flight, and re-checking
/// them every iteration would buy nothing.
///
/// Takes `rerouted` rather than assuming it, because [`harness_bin`] resolves a
/// *different* binary either way (a reroute consults no unscoped
/// `GROVE_HARNESS_BIN`), and a check that looked at a binary the launch would
/// not exec is worse than no check at all.
fn leaf_harness_installed(harness: &'static Harness, rerouted: bool) -> Result<()> {
    let bin = harness_bin(harness, rerouted);
    if !crate::harness::exec_bin_on_path(&bin) {
        anyhow::bail!(
            "grove: the next leaf declares `**Harness:** {}`, but {bin} is not on PATH. \
             Install it, or change the leaf's `**Harness:**` line — grove will not fall \
             back to another harness for a leaf that named one.",
            harness.name,
        );
    }
    Ok(())
}

/// The outcome of peeking the next live leaf — three genuinely different
/// situations `resolve_launch` must not conflate (B6): `Empty` has no leaf to
/// route *on purpose* (the finish-cycle iteration), while `Degraded` means the
/// peek itself failed and the leaf's launch is simply unknown. The distinction
/// is what keeps the finish-cycle iteration exempt from the required-model rule
/// while a genuinely unresolvable leaf refuses to launch at all.
///
/// `Leaf` carries **both** routing facts the peek returns: the kind, always,
/// and the harness the leaf declares for itself when it declares one
/// (leaf-harness-k15). They live in one variant because they come from one
/// subprocess reading one file — splitting them would imply a leaf can have a
/// harness without a kind, which the read path makes impossible (kind degrades
/// to `impl` rather than going missing).
enum KindPeek {
    Leaf(Kind, Option<&'static Harness>),
    Empty,
    Degraded,
}

/// Peek the next live leaf's launch facts by running `grove-llm kind
/// --with-harness` against the worktree — the same verb (and code path) the
/// launched agent would call. Any failure (binary missing, non-zero exit,
/// unparseable output, or a leaf declaring a harness the verb rejects) yields
/// [`KindPeek::Degraded`] with a diagnostic rather than erroring here, so the
/// *reporting* stays separate from the *policy*: this function says what it
/// saw, `resolve_launch` decides what that means. The child's stderr is
/// inherited, not captured — see the note on the spawn below.
///
/// The output is one or two lines: the kind, then the declared harness when
/// there is one. Both ends of that shape move together — the driver's
/// per-session version-skew guard already refuses to run a `grove-llm` that is
/// not this exact build, so there is no window in which a two-line reader meets
/// a one-line writer.
fn resolve_kind(worktree: &Path) -> KindPeek {
    let out = Command::new(grove_llm_bin())
        .arg("kind")
        .arg("--with-harness")
        .current_dir(worktree)
        // Inherit stderr rather than capture it. `kind` *degrades* on an
        // unrecognised `**Kind:**` line — it warns and prints `work`, on a
        // **zero** exit — so the warning rides the success path. Capturing would
        // swallow it and silently launch the leaf on the impl model, which in a
        // typical config is the cheapest one: a silent downgrade with no way to
        // see why. Inheriting surfaces every diagnostic, warning or error.
        .stderr(Stdio::inherit())
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let mut lines = stdout.lines().map(str::trim).filter(|l| !l.is_empty());
            let Some(token) = lines.next() else {
                return KindPeek::Empty; // empty grove: no leaf to select on
            };
            // The read-side parser, not the write gate: this token came *out*
            // of `grove-llm kind`, so it is a read. That is also why the alias
            // matters here — a stale `grove-llm` predating the `work` → `impl`
            // rename prints `work`, which is exactly the mismatched-binary case
            // this arm exists to guard, and it resolves rather than degrading.
            let Some(kind) = Kind::parse_read(token) else {
                // Not expected from a real `grove-llm kind` (it self-degrades
                // unparseable task-file tokens to `impl` before printing) —
                // this guards against a mismatched/stale `grove-llm` binary.
                eprintln!("grove: unrecognized task kind {token:?} from `grove-llm kind`");
                return KindPeek::Degraded;
            };
            // The optional second line: the harness the leaf declared. The verb
            // already validated it against the registry and would have exited
            // non-zero otherwise, so an unknown name here is the same
            // stale-binary case as above, not a user's typo — which is why this
            // degrades rather than reporting the leaf's own error a second time
            // in worse words.
            let harness = match lines.next() {
                None => None,
                Some(name) => match crate::harness::by_name(name) {
                    Some(h) => Some(h),
                    None => {
                        eprintln!(
                            "grove: unrecognized harness {name:?} from `grove-llm kind --with-harness`"
                        );
                        return KindPeek::Degraded;
                    }
                },
            };
            KindPeek::Leaf(kind, harness)
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

/// The driver's own compiled-in version — what this process's text segment
/// was built as, however the `grove` on disk has moved since.
const DRIVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The version-skew guard (driver-version-skew-k11). A long-running driver
/// keeps executing the text segment it started with — `brew upgrade` replaces
/// (or deletes) the binary on disk without touching it — while the agent's
/// `grove-llm` is resolved through PATH afresh at every invocation. That skew
/// splits the signal protocol's two halves: observed as a pre-watcher driver
/// paired with a watcher-era `grove-llm`, every session hanging at its
/// completion signal with nothing ever relaunching and no diagnostic.
///
/// Returns the agent-side version on a confirmed disagreement; `None` means
/// launch normally — either the versions agree, or the version could not be
/// read at all (missing binary, failed run, unparseable output), which warns
/// and continues. Only a successfully read, definitely different version may
/// stop the loop (constraint 5 — guide, don't gate: a check that jammed on a
/// missing `grove-llm` would be worse than no check).
fn version_skew() -> Option<String> {
    match agent_grove_llm_version() {
        Ok(v) if v == DRIVER_VERSION => None,
        Ok(v) => Some(v),
        Err(why) => {
            eprintln!("grove: skipping the version-skew check — {why}");
            None
        }
    }
}

/// `grove-llm --version` as the *agent* would resolve the binary: the
/// `GROVE_LLM_BIN` seam if set, else bare `grove-llm` through PATH —
/// deliberately not [`grove_llm_bin`]'s prefer-the-sibling rule, because next
/// to a stale driver sits an equally stale sibling: the check would compare
/// the old release against itself and miss exactly the skew it exists to
/// catch. `Err` carries a human-readable reason for the degrade diagnostic.
fn agent_grove_llm_version() -> Result<String, String> {
    let bin = std::env::var_os("GROVE_LLM_BIN").unwrap_or_else(|| OsString::from("grove-llm"));
    let display = bin.to_string_lossy().into_owned();
    let out = Command::new(&bin)
        .arg("--version")
        .output()
        .map_err(|e| format!("could not run `{display} --version` ({e})"))?;
    if !out.status.success() {
        return Err(format!("`{display} --version` failed ({})", out.status));
    }
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    parse_version(&stdout).map(str::to_string).ok_or_else(|| {
        format!(
            "unrecognised `{display} --version` output {:?}",
            stdout.trim()
        )
    })
}

/// The version token out of `--version` output (`grove-llm 13.0.0` →
/// `13.0.0`): the last whitespace-separated token of the first line, and only
/// if it looks like one (leading ASCII digit) — free text must degrade the
/// check, never be "compared" and stop the loop over a phantom mismatch.
fn parse_version(stdout: &str) -> Option<&str> {
    stdout
        .lines()
        .next()?
        .split_whitespace()
        .next_back()
        .filter(|t| t.starts_with(|c: char| c.is_ascii_digit()))
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
///
/// This is also why grove needs **no** SIGINT handler for herdr release
/// (herdr-optional-ui): a Ctrl-C that stops the loop does so by killing the
/// *session*, which the driver then sees as a no-signal exit and reports as
/// `blocked` down the normal path. SIGTERM is the case that genuinely needs a
/// handler — see [`install_termination_handler`].
fn ignore_interrupts() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
}

/// Set by [`on_terminate`], read by the watcher's poll loop. A `bool` store is
/// the only work done inside the handler, because it is the only work that is
/// async-signal-safe — the release it leads to is a socket round trip, and runs
/// on a normal stack one [`POLL_INTERVAL`] tick later.
static TERMINATED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

extern "C" fn on_terminate(_signum: libc::c_int) {
    TERMINATED.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// Catch SIGTERM and SIGHUP so the driver can release its herdr authority on
/// the way out (herdr-optional-ui). herdr never expires an authority, so a
/// driver that dies without releasing leaves the pane pinned at whatever grove
/// last reported.
///
/// Both signals, because both really happen: SIGTERM is `kill` and every
/// orderly shutdown, SIGHUP is what a closing pane delivers to its foreground
/// process group. SIGINT is deliberately absent — [`ignore_interrupts`] already
/// makes Ctrl-C flow to the no-signal stop path, which reports the more useful
/// `blocked` rather than releasing.
///
/// Uncovered by construction, and documented rather than papered over: SIGKILL,
/// a panic, OOM, and power loss. In those cases the pane keeps grove's last
/// reported state until the next `grove do` overwrites it, or the user clears
/// it with `herdr pane release-agent`.
fn install_termination_handler() {
    let handler = on_terminate as extern "C" fn(libc::c_int) as usize;
    unsafe {
        libc::signal(libc::SIGTERM, handler as libc::sighandler_t);
        libc::signal(libc::SIGHUP, handler as libc::sighandler_t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `Duration::from_secs_f64` panics on NaN, negatives, and unrepresentably
    // large values. An operator typo in `GROVE_KILL_GRACE` must never take the
    // whole loop down with it, so every one of those falls back or clamps.
    #[test]
    fn a_grace_the_operator_typoed_never_panics_the_driver() {
        for bad in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            assert_eq!(
                sanitise_grace(Some(bad), 2.0),
                Duration::from_secs_f64(2.0),
                "non-finite {bad} expresses no intent — fall back to the default"
            );
        }
        assert_eq!(
            sanitise_grace(Some(1e300), 2.0),
            Duration::from_secs_f64(MAX_GRACE_SECS),
            "a finite but absurd grace clamps to the cap"
        );
        assert_eq!(
            sanitise_grace(Some(-1.0), 2.0),
            Duration::ZERO,
            "a negative grace clamps to zero (kill immediately)"
        );
    }

    #[test]
    fn a_sane_grace_is_passed_through_untouched() {
        assert_eq!(sanitise_grace(Some(0.25), 2.0), Duration::from_millis(250));
        assert_eq!(sanitise_grace(None, 2.0), Duration::from_secs(2));
    }

    // signal-file-identity-k6: two worktrees whose *basenames* collide
    // (generic grove names like "bugs"/"plan"/"docs" are the norm) must not
    // resolve to the same signal file just because `name` matches — that was
    // the whole bug (path derived from `name` alone).
    #[test]
    fn same_name_different_worktrees_get_different_signal_files() {
        let a = tempfile::tempdir().unwrap();
        let b = tempfile::tempdir().unwrap();
        assert_ne!(
            signal_file_path(a.path(), "bugs"),
            signal_file_path(b.path(), "bugs"),
            "distinct worktrees sharing a grove name must never share a signal file"
        );
    }

    // The version-skew guard may only ever act on a token that *is* a
    // version. Anything else — an empty read, a shell's own error text, a
    // dev-build tag — must parse to `None` and degrade the check, because a
    // "mismatch" against free text would stop the loop over a phantom skew.
    #[test]
    fn version_parsing_accepts_a_version_and_rejects_free_text() {
        assert_eq!(parse_version("grove-llm 13.0.0\n"), Some("13.0.0"));
        assert_eq!(parse_version(""), None);
        assert_eq!(parse_version("zsh: command not found\n"), None);
        assert_eq!(parse_version("grove-llm dev-build\n"), None);
    }

    // The identity half must be stable, not just distinguishing — the same
    // worktree is polled for the *same* file every iteration of one loop
    // (signal_file_path is called fresh each time round the `loop {}`).
    #[test]
    fn same_worktree_and_name_is_stable_across_calls() {
        let a = tempfile::tempdir().unwrap();
        assert_eq!(
            signal_file_path(a.path(), "bugs"),
            signal_file_path(a.path(), "bugs"),
            "the same worktree+name must resolve to the same path every time"
        );
    }
}

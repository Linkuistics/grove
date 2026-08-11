// Shared test-only env-isolation helpers for the integration test binaries
// that drive the loop/provisioning/complete verbs against process-global env
// vars. Cargo's `tests/*.rs` target auto-discovery only scans direct
// children of `tests/`, so `tests/support/mod.rs` is not itself a test
// binary — pull it in per-file with `mod support;`.
//
// Each `tests/*.rs` file compiles this module into its own separate binary
// (one per Cargo test target), so not every item is used by every consumer.
#![allow(dead_code)]

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Child;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

/// Lock a process-env-mutating test's shared `Mutex`, tolerating poison. A
/// prior test panicking mid-mutation must not cascade-fail every later test
/// in the binary with an opaque `PoisonError` — [`EnvGuard`]'s `Drop` has
/// already restored the env by the time the panic unwound past it, so a
/// poisoned lock still guards a consistent env; only the panicked test's own
/// assertions should fail.
pub fn lock_env(lock: &'static Mutex<()>) -> MutexGuard<'static, ()> {
    lock.lock().unwrap_or_else(|e| e.into_inner())
}

/// The upper bound on a readiness wait. It exists so a *wedged* producer fails
/// the suite instead of parking it, and it is deliberately two orders of
/// magnitude above the cost of a healthy driver start-up: no test outcome
/// should ever turn on its exact value.
const READINESS_HANG_BACKSTOP: Duration = Duration::from_secs(120);

const READINESS_POLL: Duration = Duration::from_millis(10);

/// Wait for `path` to appear, using the producing process's **liveness** as the
/// condition rather than a fixed budget for how slow readiness may be.
///
/// Every readiness file waited on this way is produced by a process the test
/// itself spawned — either by that process or by a session it launched and reaps
/// on its own way out — so "will this file ever appear?" has an observable
/// answer: while the producer runs it may still be coming, and once the producer
/// is gone it never will. The fixed deadlines this replaces guessed at that
/// answer from outside. Driver start-up is not a fixed cost — it provisions the
/// embedded skill into a cold `HOME`, spawns `grove-llm --version`, and runs the
/// tree transition — and eight concurrent copies of a process-driving binary
/// stretch a perfectly healthy start-up from 0.7s to 4.7s, so the guess expired
/// on drivers that were working normally (driver-lease-readiness-flake-k145).
///
/// The other half is the failure the deadline could not report: a producer that
/// died before writing looked exactly like one that was merely slow, and both
/// surfaced as `timed out waiting for <path>` after a few silent seconds. Pass
/// `diagnostics` — the file the producer's captured streams were redirected
/// to — and an ended producer is reported with what it said.
///
/// **The producer is sampled before the file, and that order is the whole
/// correctness argument.** A producer observed alive may still write between
/// the two reads, and the file read catches it; a producer observed dead is
/// judged against a file read taken strictly *after* its death, so an absent
/// file at that point can never arrive. Reading the file first would need a
/// second read to close the same gap.
///
/// This is the one home of the readiness seam: every test binary that drives a
/// process reaches it here rather than re-declaring a wait of its own
/// (loop-driver-readiness-deadline-k170). Its regressions live in
/// `tests/driver_lease.rs`, whose fixtures can hold a producer back past the
/// removed deadline — the seam cannot own `#[test]`s itself, because this module
/// is compiled into every consumer binary and they would each run a copy.
///
/// Returns the failure rather than panicking so those regressions can assert on
/// it without a process-global panic hook; [`wait_for_ready`] is the ordinary
/// caller's panicking form.
pub fn readiness(
    path: &Path,
    producer: &mut Child,
    diagnostics: Option<&Path>,
) -> Result<(), String> {
    let started = Instant::now();
    loop {
        let ended = producer
            .try_wait()
            .expect("polling the process behind the readiness file");
        if path.exists() {
            return Ok(());
        }
        if let Some(status) = ended {
            return Err(readiness_failure(
                format!(
                    "nothing wrote {}: the process behind it ended ({status}) after {:.3}s",
                    path.display(),
                    started.elapsed().as_secs_f64()
                ),
                diagnostics,
            ));
        }
        if started.elapsed() >= READINESS_HANG_BACKSTOP {
            return Err(readiness_failure(
                format!(
                    "nothing wrote {}: the process behind it is still running after {}s, which is \
                     a wedged producer rather than a slow one",
                    path.display(),
                    READINESS_HANG_BACKSTOP.as_secs()
                ),
                diagnostics,
            ));
        }
        thread::sleep(READINESS_POLL);
    }
}

fn readiness_failure(reason: String, diagnostics: Option<&Path>) -> String {
    match diagnostics.map(|path| fs::read_to_string(path).unwrap_or_default()) {
        Some(output) if !output.trim().is_empty() => {
            format!("{reason}. It said:\n{}", output.trim_end())
        }
        Some(_) => format!("{reason}, and said nothing."),
        None => format!("{reason}."),
    }
}

/// [`readiness`] for the ordinary caller, whose only answer to a failed wait is
/// to fail the test with what the seam already reported.
pub fn wait_for_ready(path: &Path, producer: &mut Child, diagnostics: Option<&Path>) {
    readiness(path, producer, diagnostics).unwrap_or_else(|failure| panic!("{failure}"));
}

/// Every task-kind label, in taxonomy order (task-kind-taxonomy;
/// membership in `docs/ARCHITECTURE.md#task-kind-taxonomy`) — the same strings a live
/// task filename carries, and what the env-var suffixes below are
/// formed from. Spelled out here rather than reached for from the crate so
/// these tests stay honest about *which* names they use, independent of
/// production wiring drifting under them unnoticed.
pub const KIND_LABELS: [&str; 19] = [
    "requirements",
    "design",
    "planning",
    "prototype",
    "impl",
    "research-a",
    "research-b",
    "combine-research",
    "finish",
    "review-requirements",
    "review-design",
    "review-planning",
    "review-prototype",
    "review-impl",
    "integrate-review-requirements",
    "integrate-review-design",
    "integrate-review-planning",
    "integrate-review-prototype",
    "integrate-review-impl",
];

/// The loop driver's **control channel** (self-driving-loop), scrubbed for the
/// duration of tests so a nested launch cannot signal the developer's live
/// session.
///
/// `GROVE_SIGNAL_FILE` is the path the driver watches while its foreground child
/// runs; its mere *appearance* triggers grace → SIGTERM → kill-grace → SIGKILL.
/// These tests normally run inside a live loop session, so the variable is
/// present in this process's ambient environment, and the fake commands below
/// write `"$GROVE_SIGNAL_FILE"` unconditionally — a spawn that does not scope
/// the variable therefore kills the session `cargo test` was typed into
/// (guard-loop-signal-k37).
///
/// `.cargo/config.toml` force-overrides the same signal variable for everything
/// cargo runs. The two guards are independent on purpose: that one covers tests
/// which never touch these helpers, this one covers a test binary executed
/// directly rather than through cargo. Neither subsumes the other.
const LOOP_CONTROL_ENV: [&str; 1] = ["GROVE_SIGNAL_FILE"];

/// The launch-policy environment Grove no longer reads. It is scrubbed rather
/// than ignored because these fixtures assert on what a launch *does*, and this
/// repo dogfoods Grove: a developer's own ambient `GROVE_IMPL_HARNESS` must not
/// be able to make a passing run mean something different from a clean one.
/// `tests/launch.rs` owns the complementary claim — that setting these changes
/// nothing — and this list is deliberately not derived from production, since
/// production no longer names them anywhere.
const REMOVED_LAUNCH_POLICY_ENV: [&str; 6] = [
    "GROVE_HARNESS_BIN",
    "GROVE_LLM_BIN",
    "GROVE_SKILL_DIR",
    "GROVE_KILL_GRACE",
    "GROVE_KILL_GRACE_KILL",
    "GROVE_WORK_MODEL",
];

/// Harness names the removed routing lattice scoped its model variables by.
const HARNESS_NAMES: [&str; 3] = ["CLAUDE", "CODEX", "PI"];

/// The two removed routing **families**, which configured their five member
/// kinds as a group. Never leaf filename kinds — but live env-var suffixes in
/// every way a kind's was, and the ones a developer sets once and never looks
/// at again, so they are the likeliest to still be exported.
const FAMILY_LABELS: [&str; 2] = ["review", "integrate-review"];

/// Every ambient env var that once steered a launch or its side effects, plus
/// [`LOOP_CONTROL_ENV`]. Shared by `EnvGuard` (scrubbing this test's own process
/// env) and any test that instead needs to scrub a *subprocess*'s inherited env
/// via `Command::env_remove` — a `Command` does not isolate itself from the
/// parent's ambient env just because some vars are set explicitly.
pub fn grove_env_names() -> Vec<String> {
    let suffixes: Vec<String> = KIND_LABELS
        .iter()
        .chain(FAMILY_LABELS.iter())
        .map(|label| label.to_uppercase().replace('-', "_"))
        .collect();
    let mut names = Vec::new();
    for suffix in &suffixes {
        names.push(format!("GROVE_{suffix}_MODEL"));
        names.push(format!("GROVE_{suffix}_HARNESS"));
        for harness in HARNESS_NAMES {
            names.push(format!("GROVE_{harness}_{suffix}_MODEL"));
            names.push(format!("GROVE_HARNESS_BIN_{harness}"));
        }
    }
    names.extend(REMOVED_LAUNCH_POLICY_ENV.iter().map(|n| n.to_string()));
    names.extend(LOOP_CONTROL_ENV.iter().map(|n| n.to_string()));
    names
}

/// Save/restore an arbitrary set of env vars across a test via `Drop`, so a
/// failing `assert!` — which unwinds, it does not abort — cannot leak a
/// mutated or removed value into a later test sharing the same process.
#[derive(Default)]
pub struct EnvGuard {
    saved: Vec<(String, Option<String>)>,
}

impl EnvGuard {
    pub fn new() -> Self {
        Self::default()
    }

    fn remember(&mut self, key: &str) {
        if !self.saved.iter().any(|(k, _)| k == key) {
            self.saved.push((key.to_string(), std::env::var(key).ok()));
        }
    }

    pub fn set(&mut self, key: &str, value: impl AsRef<OsStr>) -> &mut Self {
        self.remember(key);
        std::env::set_var(key, value);
        self
    }

    pub fn remove(&mut self, key: &str) -> &mut Self {
        self.remember(key);
        std::env::remove_var(key);
        self
    }

    /// Scrub every name in [`grove_env_names`] from this process's own
    /// environment: the removed launch-policy surface, so a developer's
    /// dogfooded shell cannot change what a fixture observes, plus
    /// [`LOOP_CONTROL_ENV`], so a nested launch cannot kill their live session.
    pub fn clear_grove_env(&mut self) -> &mut Self {
        for name in grove_env_names() {
            self.remove(&name);
        }
        self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, prior) in self.saved.drain(..) {
            match prior {
                Some(v) => std::env::set_var(&key, v),
                None => std::env::remove_var(&key),
            }
        }
    }
}

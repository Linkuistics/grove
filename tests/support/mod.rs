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
use std::sync::{Mutex, MutexGuard};

/// Lock a process-env-mutating test's shared `Mutex`, tolerating poison. A
/// prior test panicking mid-mutation must not cascade-fail every later test
/// in the binary with an opaque `PoisonError` — [`EnvGuard`]'s `Drop` has
/// already restored the env by the time the panic unwound past it, so a
/// poisoned lock still guards a consistent env; only the panicked test's own
/// assertions should fail.
pub fn lock_env(lock: &'static Mutex<()>) -> MutexGuard<'static, ()> {
    lock.lock().unwrap_or_else(|e| e.into_inner())
}

/// Every task-kind label, in taxonomy order (ADR `task-kind-taxonomy`;
/// membership in `docs/specs/task-kind-taxonomy.md`) — the same strings a live
/// task file's `**Kind:**` line carries, and what the env-var suffixes below are
/// formed from. Spelled out here rather than reached for from the crate so
/// these tests stay honest about *which* names they use, independent of
/// production wiring drifting under them unnoticed.
pub const KIND_LABELS: [&str; 17] = [
    "requirements",
    "design",
    "planning",
    "prototype",
    "impl",
    "research",
    "combine-research",
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

/// The two routing **families** (`model-per-task-kind`), which configure their
/// five member kinds as a group. Not kinds — a leaf may never declare one — but
/// a live env-var suffix in every way a kind's is: `GROVE_REVIEW_MODEL` and
/// `GROVE_REVIEW_HARNESS` both route, so both have to be scrubbed, or a
/// developer's own `GROVE_REVIEW_HARNESS=codex` would reroute review leaves
/// inside tests that never asked for it.
const FAMILY_LABELS: [&str; 2] = ["review", "integrate-review"];

/// Retired pre-rename kind suffixes. Not kinds and not families — scrubbed only
/// so a developer whose shell still exports `GROVE_WORK_MODEL` (this repo
/// dogfoods the routing envs) gets a hermetic run.
const RETIRED_KIND_SUFFIXES: [&str; 1] = ["WORK"];

const HARNESS_NAMES: [&str; 3] = ["CLAUDE", "CODEX", "PI"];

/// Every env-name suffix routing reads — uppercase, `-` → `_` — mirroring
/// `loop_driver::all_routing_suffixes`: the seventeen kinds, the two families,
/// plus the retired spellings.
fn kind_env_suffixes() -> Vec<String> {
    KIND_LABELS
        .iter()
        .chain(FAMILY_LABELS.iter())
        .map(|l| l.to_uppercase().replace('-', "_"))
        .chain(RETIRED_KIND_SUFFIXES.iter().map(|s| s.to_string()))
        .collect()
}

/// The pane environment herdr places in every pane it spawns, which the driver
/// now reads to report its state (herdr-optional-ui). Scrubbed for a blunt
/// reason: these tests are *themselves* usually run from inside a herdr pane, so
/// without this every `run_loop` test would report `working`/`blocked` over the
/// developer's own live pane — `cargo test` would visibly hijack the sidebar
/// row of the terminal it was typed into. A test that wants the reporter awake
/// sets these three back, pointing at a listener it owns.
const HERDR_PANE_ENV: [&str; 3] = ["HERDR_ENV", "HERDR_SOCKET_PATH", "HERDR_PANE_ID"];

/// The loop driver's **control channel** (self-driving-loop), scrubbed for the
/// same reason as [`HERDR_PANE_ENV`] one notch up the severity scale: not a
/// hijacked sidebar row, a dead terminal.
///
/// `GROVE_SIGNAL_FILE` is the path the driver watches while its harness child
/// runs; its mere *appearance* triggers grace → SIGTERM → kill-grace → SIGKILL.
/// These tests normally run inside a live `grove do` session, so the variable is
/// present in this process's ambient environment, and the fake harnesses below
/// write `"$GROVE_SIGNAL_FILE"` unconditionally — a harness spawn that does not
/// scope the variable therefore kills the session `cargo test` was typed into
/// (guard-loop-signal-k37, pinned to the codex sandbox pre-flight, which spawns
/// the harness binary outside `launch_session` — the one site that scopes it).
///
/// `.cargo/config.toml` force-overrides the same variable for everything cargo
/// runs. The two guards are independent on purpose: that one covers tests which
/// never touch these helpers, this one covers a test binary executed directly
/// rather than through cargo. Neither subsumes the other.
const LOOP_CONTROL_ENV: [&str; 1] = ["GROVE_SIGNAL_FILE"];

/// Every ambient env var that steers a `grove do` launch or its side effects:
/// the GROVE_* routing / model-selection surface (see
/// [`EnvGuard::clear_grove_env`]'s doc for the count), [`HERDR_PANE_ENV`], plus
/// [`LOOP_CONTROL_ENV`].
/// Shared by `EnvGuard` (scrubbing this test's own process env) and any test
/// that instead needs to scrub a *subprocess*'s inherited env via
/// `Command::env_remove` — a `Command` does not isolate itself from the
/// parent's ambient env just because some vars are set explicitly.
pub fn grove_env_names() -> Vec<String> {
    let suffixes = kind_env_suffixes();
    let mut names = Vec::with_capacity(
        suffixes.len() * (2 + HARNESS_NAMES.len()) + HERDR_PANE_ENV.len() + LOOP_CONTROL_ENV.len(),
    );
    for kind in &suffixes {
        names.push(format!("GROVE_{kind}_MODEL"));
        names.push(format!("GROVE_{kind}_HARNESS"));
        for harness in HARNESS_NAMES {
            names.push(format!("GROVE_{harness}_{kind}_MODEL"));
        }
    }
    names.extend(HERDR_PANE_ENV.iter().map(|n| n.to_string()));
    names.extend(LOOP_CONTROL_ENV.iter().map(|n| n.to_string()));
    names
}

/// The value following `--add-dir` in a space-joined argv log line (the
/// fake-harness scripts log `$*`), or `None` when the flag is absent.
/// Whitespace-splitting is safe here: every fixture path is a `TempDir`
/// path with no spaces.
pub fn add_dir_value(argv: &str) -> Option<&str> {
    add_dir_values(argv).into_iter().next()
}

/// Every value following an `--add-dir` in a space-joined argv log line, in
/// argv order — a jj-enabled launch grants up to two stores (the main
/// workspace's `.jj`, plus its `.git` when colocated). Same whitespace-split
/// caveat as [`add_dir_value`].
pub fn add_dir_values(argv: &str) -> Vec<&str> {
    let mut values = Vec::new();
    let mut tokens = argv.split_whitespace();
    while let Some(token) = tokens.next() {
        if token == "--add-dir" {
            if let Some(value) = tokens.next() {
                values.push(value);
            }
        }
    }
    values
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

    /// Scrub the loop driver's whole routing/model-selection surface —
    /// ([`KIND_LABELS`] + [`FAMILY_LABELS`]) × [base `GROVE_<KIND>_MODEL`, 3
    /// harness-scoped `GROVE_<HARNESS>_<KIND>_MODEL`, `GROVE_<KIND>_HARNESS`],
    /// 95 live names plus the retired spellings — **plus the 3
    /// [`HERDR_PANE_ENV`] vars**, so a loop under test
    /// cannot report into the developer's own herdr pane, **plus
    /// [`LOOP_CONTROL_ENV`]**, so it cannot kill the developer's own session.
    /// Every loop_driver
    /// test needs this: this branch's own dogfooded `~/.zshenv` (and the loop
    /// driver's own launch, for a session running these tests under itself)
    /// sets a dozen of these for real, and a hand-maintained `remove_var` list
    /// drifts the moment a kind or harness is added — it already had, once.
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

// ---------------------------------------------------------------------------
// A fake herdr, for the tests that assert on what grove *reports* rather than
// on what it does. Shared by `tests/loop_driver.rs` (the driver's four report
// sites) and `tests/report_turn.rs` (the turn hooks' verb), because both need
// the same thing: a real unix socket speaking herdr's newline-delimited JSON,
// addressed through the same `HERDR_*` variables herdr itself puts in a pane.

/// Bind a fake herdr on `path` and serve it from a background thread, appending
/// each request line to the returned buffer.
///
/// The buffer is shared rather than returned from a `JoinHandle`, and the thread
/// is deliberately never joined: `UnixListener::accept` has no timeout in `std`,
/// so a joinable server would need an out-of-band way to be woken, and there
/// isn't a reliable one once the socket path is gone. Sharing is also *correct*
/// by ordering, not just convenient — the server appends a line **before**
/// answering it, and the driver's own report waits for that answer, so every
/// line is already in the buffer by the time `run_loop` returns.
pub fn fake_herdr(path: &std::path::Path) -> std::sync::Arc<Mutex<Vec<String>>> {
    let listener = std::os::unix::net::UnixListener::bind(path).unwrap();
    let lines = std::sync::Arc::new(Mutex::new(Vec::new()));
    let collected = std::sync::Arc::clone(&lines);
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { break };
            let mut line = String::new();
            if std::io::BufRead::read_line(&mut std::io::BufReader::new(&stream), &mut line).is_ok()
            {
                if !line.trim().is_empty() {
                    collected.lock().unwrap().push(line.trim().to_string());
                }
                // Answer as herdr does, so the driver's read completes rather
                // than spending its whole budget waiting on every report.
                let _ = std::io::Write::write_all(
                    &mut &stream,
                    br#"{"id":"x","result":{"type":"ok"}}"#.as_slice(),
                );
                let _ = std::io::Write::write_all(&mut &stream, b"\n");
            }
        }
    });
    lines
}

/// The `(method, state)` pairs out of collected request lines — asserting on
/// these rather than on raw JSON keeps the wiring assertions about *sequence*,
/// while `src/herdr.rs`'s own tests pin the exact bytes.
pub fn reported(lines: &[String]) -> Vec<(String, String)> {
    lines
        .iter()
        .map(|line| {
            let field = |key: &str| {
                line.split(&format!("\"{key}\":\""))
                    .nth(1)
                    .and_then(|rest| rest.split('"').next())
                    .unwrap_or("")
                    .to_string()
            };
            (field("method"), field("state"))
        })
        .collect()
}

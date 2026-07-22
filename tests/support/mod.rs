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

/// The five task kinds' env-name suffixes and the three harnesses' names, in
/// the same taxonomy the loop driver reads (src/loop_driver.rs KIND_SUFFIXES
/// / harness::HARNESSES). Kept local rather than `use`d from the crate so
/// this helper stays honest about *which* names it scrubs, independent of
/// production wiring drifting under it unnoticed.
const KINDS: [&str; 5] = ["PLANNING", "RESEARCH", "PROTOTYPE", "WORK", "REVIEW"];
const HARNESS_NAMES: [&str; 3] = ["CLAUDE", "CODEX", "PI"];

/// Every GROVE_* env var the loop driver's routing / model-selection reads
/// (see [`EnvGuard::clear_grove_env`]'s doc for the count). Shared by
/// `EnvGuard` (scrubbing this test's own process env) and any test that
/// instead needs to scrub a *subprocess*'s inherited env via
/// `Command::env_remove` — a `Command` does not isolate itself from the
/// parent's ambient env just because some vars are set explicitly.
pub fn grove_env_names() -> Vec<String> {
    let mut names = Vec::with_capacity(KINDS.len() * (2 + HARNESS_NAMES.len()));
    for kind in KINDS {
        names.push(format!("GROVE_{kind}_MODEL"));
        names.push(format!("GROVE_{kind}_HARNESS"));
        for harness in HARNESS_NAMES {
            names.push(format!("GROVE_{harness}_{kind}_MODEL"));
        }
    }
    names
}

/// The value following `--add-dir` in a space-joined argv log line (the
/// fake-harness scripts log `$*`), or `None` when the flag is absent.
/// Whitespace-splitting is safe here: every fixture path is a `TempDir`
/// path with no spaces.
pub fn add_dir_value(argv: &str) -> Option<&str> {
    let mut tokens = argv.split_whitespace();
    while let Some(token) = tokens.next() {
        if token == "--add-dir" {
            return tokens.next();
        }
    }
    None
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

    /// Scrub the loop driver's whole routing/model-selection surface — 5
    /// kinds × [base `GROVE_<KIND>_MODEL`, 3 harness-scoped
    /// `GROVE_<HARNESS>_<KIND>_MODEL`] + 5 `GROVE_<KIND>_HARNESS` overrides,
    /// 25 names. Every loop_driver test needs this: this branch's own
    /// dogfooded `~/.zshenv` (and the loop driver's own launch, for a
    /// session running these tests under itself) sets a dozen of these for
    /// real, and a hand-maintained `remove_var` list drifts the moment a
    /// kind or harness is added — it already had, once.
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

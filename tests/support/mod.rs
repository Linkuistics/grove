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
/// `loop_driver::all_routing_suffixes`: the nineteen kinds, the two families,
/// plus the retired spellings.
fn kind_env_suffixes() -> Vec<String> {
    KIND_LABELS
        .iter()
        .chain(FAMILY_LABELS.iter())
        .map(|l| l.to_uppercase().replace('-', "_"))
        .chain(RETIRED_KIND_SUFFIXES.iter().map(|s| s.to_string()))
        .collect()
}

/// The loop driver's **control channel** (self-driving-loop), scrubbed for the
/// duration of tests so a nested launch cannot signal the developer's live
/// session.
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
/// `.cargo/config.toml` force-overrides the same signal variable and clears the
/// advisory session target for everything cargo runs. The two guards are
/// independent on purpose: that one covers tests which never touch these
/// helpers, this one covers a test binary executed directly rather than through
/// cargo. Neither subsumes the other.
const LOOP_CONTROL_ENV: [&str; 2] = ["GROVE_SIGNAL_FILE", "GROVE_SESSION_TARGET"];

/// Every ambient env var that steers a `grove do` launch or its side effects:
/// the GROVE_* routing / model-selection surface (see
/// [`EnvGuard::clear_grove_env`]'s doc for the count), plus [`LOOP_CONTROL_ENV`].
/// Shared by `EnvGuard` (scrubbing this test's own process env) and any test
/// that instead needs to scrub a *subprocess*'s inherited env via
/// `Command::env_remove` — a `Command` does not isolate itself from the
/// parent's ambient env just because some vars are set explicitly.
pub fn grove_env_names() -> Vec<String> {
    let suffixes = kind_env_suffixes();
    let mut names =
        Vec::with_capacity(suffixes.len() * (2 + HARNESS_NAMES.len()) + LOOP_CONTROL_ENV.len());
    for kind in &suffixes {
        names.push(format!("GROVE_{kind}_MODEL"));
        names.push(format!("GROVE_{kind}_HARNESS"));
        for harness in HARNESS_NAMES {
            names.push(format!("GROVE_{harness}_{kind}_MODEL"));
        }
    }
    names.extend(LOOP_CONTROL_ENV.iter().map(|n| n.to_string()));
    names
}

/// The `sh` block every fake standing in for **codex** carries, so it answers
/// grove's sandbox pre-flight the way a *trusted* codex does
/// (codex-grant-refused-k35).
///
/// Before launching codex, grove runs `codex exec` with the same model flags and
/// the same `--add-dir` grants and reads the one `sandbox:` line of the header,
/// because codex refuses those grants **fatally** under a `read-only` sandbox —
/// which is the default for any project the user has not trusted. That makes a
/// codex launch two invocations of the harness binary, and a fake that treats
/// both as launches counts twice, logs twice, and signals twice.
///
/// `exec` as the first argument is an exact discriminator, not a heuristic: it
/// is codex's own subcommand, and grove's codex *launch* argv never carries one
/// (`[--profile M] [--add-dir D]… <prompt>`). So the branch fires on the probe
/// and on nothing else, and a fake for a harness that is never probed is
/// unaffected by carrying it.
///
/// Answering `workspace-write` rather than staying mute is deliberate: silence
/// degrades the verdict to `Unknown`, which also proceeds, so the tests would
/// pass while exercising the fallback instead of the path production takes. The
/// refusal is exercised by a fake that answers `read-only` on purpose.
pub const CODEX_SANDBOX_PROBE_REPLY: &str = r#"if [ "$1" = exec ]; then
  printf 'sandbox: workspace-write [workdir, /tmp, $TMPDIR]\n' >&2
  exit 0
fi
"#;

/// A fake harness body with [`CODEX_SANDBOX_PROBE_REPLY`] spliced in directly
/// after its `#!` line — the earliest point it can go, since it has to precede
/// whatever the fake logs, counts, or signals.
///
/// Takes the whole script rather than a shebang-less fragment so each fixture
/// stays readable as a complete shell program at its call site.
pub fn fake_codex(body: &str) -> String {
    let (shebang, rest) = body
        .split_once('\n')
        .unwrap_or_else(|| panic!("a fake harness starts with a `#!` line: {body:?}"));
    format!("{shebang}\n{CODEX_SANDBOX_PROBE_REPLY}{rest}")
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
    /// 95 live names plus the retired spellings — plus [`LOOP_CONTROL_ENV`], so
    /// it cannot kill the developer's own session.
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

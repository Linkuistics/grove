// Shared test-only helpers for the integration-test binaries that drive the
// loop and its verbs: the scrub list a spawned child's environment is built
// from, the readiness seam, the jj fixtures, and the kind-label walk.
//
// **It lives outside every package on purpose.** `loop-crate-driver-k22` made
// the repository root a bare workspace, so there is no root package to own a
// shared `tests/` directory any more — and these helpers are about the
// *repository* rather than about any one of the three packages that use them.
// Copying them into each would have produced three that drift. Each consumer
// pulls this one in through a `tests/support/mod.rs` shim that names it by
// `#[path]`, and `mod support;` from there.
//
// A `tests/support/` shim is not itself a test binary: cargo's `tests/*.rs`
// auto-discovery only scans direct children of `tests/`.
//
// Each consuming `tests/*.rs` compiles this module into its own separate binary
// (one per cargo test target), so not every item is used by every consumer.
//
// **Nothing here mutates this process's environment, and nothing may.** The
// three consumers run their tests on threads libtest spawns, so `set_var` and
// `remove_var` are unsound in every one of them — "the only sound option is to
// not use `set_var` or `remove_var` at all"
// (<https://doc.rust-lang.org/std/env/fn.set_var.html>). Env isolation here is
// therefore isolation of a *child*: build the environment a subprocess should
// see with `Command::env` and `Command::env_remove`, which write the child's
// map and leave this process alone. `grove_env_names` is the shared scrub list
// for that seam, and `crates/grove/tests/env_hygiene.rs` carries the reasoning
// beside the two guards it already asserts. The worked pattern for a property
// that can only be *observed* under a given environment is
// `crates/jj-workspace/tests/environment.rs`, which re-runs its own test binary
// as the child under an environment built with `Command::env`.
//
// The compiler does not hold this yet: under `edition = "2021"` both are safe
// fns, so it is silent until the edition-2024 migration makes them `unsafe`
// (<https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html>).
// The suite holds it in the interval —
// `no_first_party_source_mutates_its_own_process_environment` in
// `crates/grove/tests/env_hygiene.rs` scans every first-party `.rs` file for
// either call, outside a whole-line comment, and fails on one. So a
// re-introduction here is caught by a test today and by the compiler after the
// migration; the paragraph above is an explanation of a checked rule, not the
// only thing keeping it.
//
// **What the migration has to do to this file: nothing.** Measured at
// `testing-support-env-guard-soundness-k203` rather than argued — a workspace
// copy with `[workspace.package] edition` flipped to `2024` passes
// `cargo check --workspace --all-targets` with one unrelated fix, an RPIT
// capture in `crates/ordinal-fs-tree` that Rust 2024's lifetime rules widen
// (`E0515` at `src/plan.rs:219`, discharged by `+ use<'a, N>` on
// `snapshot.rs`'s `children`). Re-adding one `set_var` and one `remove_var`
// here turns that green run into seven `E0133`s — the control on the check, and
// the reason the deletion is what makes the migration free rather than the
// edition being lenient. None of those `unsafe` blocks could have been
// discharged: the safety condition is that no other thread reads the
// environment, and libtest gives no test that guarantee.
//
// This is why nothing here hands out an environment lock. One existed for the
// mutating guard that used to live below and was deleted with it
// (`testing-support-env-guard-soundness-k203`): with no mutation left there is
// no window for a concurrent reader to observe, and a mutex serialising tests
// against a mutation that cannot happen costs wall-clock and buys nothing.
#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::thread;
use std::time::{Duration, Instant};

/// The upper bound on a readiness wait. It exists so a *wedged* producer fails
/// the suite instead of parking it, and it is deliberately two orders of
/// magnitude above the cost of a healthy driver start-up.
///
/// **It is a hang backstop and not a slowness budget, and the difference is
/// what it may claim when it fires.** The clock is the only thing this constant
/// observes, and a wall-clock reading cannot tell a producer that is blocked
/// from one that is merely starved — so the failure below reports a second
/// measurement rather than asserting which it was. It used to assert, and three
/// separate leaves read *a wedged producer rather than a slow one* as evidence
/// that the fixtures behind it were doing a hundred seconds of work
/// (driver-lease-fixture-timing-k85).
///
/// **The headroom, measured rather than assumed.** On a 16-core machine the two
/// fixtures that had reported against this constant cost 1.6s and 3.1s alone
/// and 6.0s for the whole 23-test binary; twenty-four concurrent copies of that
/// binary — 552 process-driving fixtures at once — took 60s for the binary with
/// every readiness wait still completing and this backstop never firing. So
/// starvation alone does not approach two minutes, and a reading that reaches
/// here is evidence of a producer that stopped rather than one that slowed.
const READINESS_HANG_BACKSTOP: Duration = Duration::from_secs(120);

/// How long the expiry below watches a producer's CPU time before reporting it.
/// Paid once, on a wait that has already failed, so it costs a passing run
/// nothing and buys the one distinction the clock cannot make.
const PROGRESS_SAMPLE: Duration = Duration::from_secs(2);

const READINESS_POLL: Duration = Duration::from_millis(10);

/// Wait for `path` to appear, using the producing process's **liveness** as the
/// condition rather than a fixed budget for how slow readiness may be.
///
/// Every readiness file waited on this way is produced by a process the test
/// itself spawned — either by that process or by a session it launched and reaps
/// on its own way out — so "will this file ever appear?" has an observable
/// answer: while the producer runs it may still be coming, and once the producer
/// is gone it never will. The fixed deadlines this replaces guessed at that
/// answer from outside. Driver start-up is not a fixed cost — it resolves the
/// workspace, takes the lease and runs the tree transition, and it used to sweep
/// an embedded skill into a cold `HOME` and probe the build pairing besides —
/// and eight concurrent copies of a process-driving binary
/// stretch a perfectly healthy start-up from 0.7s to 4.7s, so the guess expired
/// on drivers that were working normally (driver-lease-readiness-flake-k145).
///
/// The other half is the failure the deadline could not report: a producer that
/// died before writing looked exactly like one that was merely slow, and both
/// surfaced as `timed out waiting for <path>` after a few silent seconds. Pass
/// `diagnostics` — the file the producer's captured streams were redirected
/// to — and an ended producer is reported with what it said.
///
/// **A producer that neither writes nor ends is the third case, and only
/// [`READINESS_HANG_BACKSTOP`] bounds it.** That bound is a clock, so the
/// failure it raises reports [`progress_reading`] instead of naming a cause:
/// blocked and starved are the two ways to reach it, they call for opposite
/// repairs, and the clock distinguishes neither.
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
                    "nothing wrote {}: the process behind it is still running after {}s — {}",
                    path.display(),
                    READINESS_HANG_BACKSTOP.as_secs(),
                    progress_reading(producer.id())
                ),
                diagnostics,
            ));
        }
        thread::sleep(READINESS_POLL);
    }
}

/// Whether a producer that outlasted [`READINESS_HANG_BACKSTOP`] is still doing
/// work, in the words the failure carries.
///
/// **Two readings of the same clock, because one reading answers nothing.** A
/// producer starved by an oversubscribed machine and a producer blocked on a
/// lock, a pipe that will never close, or a `SIGTTIN` stop are indistinguishable
/// by elapsed wall time and by `try_wait`, which reports a *stopped* process
/// exactly as it reports a running one — it does not pass `WUNTRACED`. They are
/// not indistinguishable by CPU: the starved one is still forking, exec'ing and
/// waiting on a real workload, and the blocked one accrues nothing at all. So
/// this samples the accrued CPU time twice and reports the pair.
///
/// It reports rather than decides. The backstop has already fired by the time
/// this is called and fires whatever comes back, including nothing — a reading
/// that cannot be taken must never turn a failed wait into a wait that parks
/// the suite, which is the failure the backstop exists to prevent.
///
/// `ps` rather than a platform call because this runs on macOS and Linux and
/// the reading is compared, never parsed: two strings from one `ps` on one
/// machine differ exactly when the counter moved. Its output format is
/// therefore free to be `MM:SS.ss` or `HH:MM:SS`. Nothing is scrubbed from its
/// environment because it is asked about a pid and reads no `GROVE_*` variable —
/// unlike the spawns in `grove_env_names`'s doc, this one cannot be steered by
/// a developer's ambient configuration into reporting something else.
fn progress_reading(pid: u32) -> String {
    let cpu = || {
        let output = std::process::Command::new("ps")
            .args(["-o", "time=", "-p", &pid.to_string()])
            .output()
            .ok()?;
        let reading = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        (!reading.is_empty()).then_some(reading)
    };
    let Some(before) = cpu() else {
        return "and its CPU time could not be read, so whether it is blocked or \
                merely starved is unmeasured here"
            .to_owned();
    };
    thread::sleep(PROGRESS_SAMPLE);
    match cpu() {
        Some(after) if after != before => format!(
            "and it burned CPU over a further {}s ({before} → {after}), so it is being starved \
             rather than blocked: suspect load on this machine, not the fixture",
            PROGRESS_SAMPLE.as_secs()
        ),
        Some(after) => format!(
            "and it burned no CPU over a further {}s (still {after}), so it is blocked rather \
             than slow: look for what it is waiting on, not for a longer deadline",
            PROGRESS_SAMPLE.as_secs()
        ),
        None => format!(
            "and it stopped being readable after reporting {before} of CPU, so it ended between \
             the two samples"
        ),
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

/// Every task-kind label the shipped plugin declares — one `grove-<kind>` skill
/// each, read off `plugins/grove/skills/` rather than listed.
///
/// **This is where the kind set lives.** It used to be a hand-written array of
/// nineteen here, spelled out so these tests stayed honest about *which* names
/// they used while production carried its own `Kind::ALL`. `open-kind-k20`
/// deleted that enum: the machinery holds no enumeration of kinds, a kind is any
/// well-formed token, and the only thing that decides which kinds *exist* is
/// which `grove-<kind>` skills are installed. So a copy here would no longer be
/// an independent witness to anything — it would be the only list in the
/// repository, and it would be the one that went stale.
///
/// The bare `grove` directory is the shared spine every kind reads, not a kind.
pub fn kind_labels() -> Vec<String> {
    let skills = repo_root().join("plugins/grove/skills");
    let mut labels: Vec<String> = std::fs::read_dir(&skills)
        .unwrap_or_else(|error| panic!("reading {}: {error}", skills.display()))
        .map(|entry| entry.expect("readable directory entry"))
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .strip_prefix("grove-")
                .map(ToOwned::to_owned)
        })
        .collect();
    labels.sort();
    assert!(
        labels.len() > 3,
        "the walk found {} `grove-<kind>` skills — a mis-scoped walk reports a \
         clean surface for the wrong reason",
        labels.len()
    );
    labels
}

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
/// [`LOOP_CONTROL_ENV`]. Read at the **subprocess** seam and nowhere else: a
/// test scrubs each of these from a child it is about to spawn with
/// `Command::env_remove`, because a `Command` does not isolate itself from the
/// parent's ambient env just because some other vars are set explicitly.
///
/// It had a second consumer until `testing-support-env-guard-soundness-k203`:
/// an `EnvGuard` that scrubbed the *test process's own* environment through
/// `std::env::remove_var` and restored it on `Drop`. Nothing constructed it,
/// and nothing may: "in multi-threaded programs on other operating systems,
/// the only sound option is to not use `set_var` or `remove_var` at all"
/// (<https://doc.rust-lang.org/std/env/fn.set_var.html>), and every binary this
/// module compiles into runs its tests on threads libtest spawns. Isolating
/// *this* process is not on offer, so isolating a child is the whole of what
/// this list is for — see the module header.
pub fn grove_env_names() -> Vec<String> {
    let suffixes: Vec<String> = kind_labels()
        .iter()
        .map(String::as_str)
        .chain(FAMILY_LABELS.iter().copied())
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

/// Stand up a **jj-native** repository at `path` — the only kind of working
/// tree Grove drives (`docs/adr/jj-is-the-only-lane.md`), and therefore the
/// fixture every verb-driving test needs before it can run a verb at all.
///
/// `git.colocate=false` is forced because ambient jj configuration may default
/// colocation on, which would silently turn every "native" fixture into a
/// colocated one and hide a `.git` Grove is supposed never to look at.
pub fn init_jj_repo(path: &Path) {
    jj(
        path,
        &["--config", "git.colocate=false", "git", "init", "."],
    );
}

/// Run `jj` in `path` with a test-local identity, so no global user
/// configuration is required, and return its trimmed stdout.
pub fn jj(path: &Path, args: &[&str]) -> String {
    let output = std::process::Command::new("jj")
        .current_dir(path)
        .args([
            "--config",
            "user.name=Grove Test",
            "--config",
            "user.email=grove-test@example.com",
        ])
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("running jj {args:?}: {error} (is jj installed?)"));
    assert!(
        output.status.success(),
        "jj {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

/// A `$HOME` holding a personal Grove configuration that declares every session
/// kind, shared by every command one test binary spawns.
///
/// **Why a verb-driving test needs one at all.** Writing a leaf of kind K now
/// asks whether K resolves to a launch template before it mutates the tree
/// (`docs/adr/complete-session-configuration.md`). Without this the check would
/// be answered by whatever is in the developer's own
/// `~/.config/grove/config.kdl` — passing on a configured machine, failing on a
/// fresh checkout, and testing the machine rather than the code either way.
///
/// The templates are `true ${prompt}`: word zero is a literal executable, which
/// is all validation asks, and nothing here ever spawns one.
pub fn fixture_home() -> &'static Path {
    static HOME: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
    let home = HOME.get_or_init(|| {
        let home = tempfile::TempDir::new().expect("fixture $HOME");
        let dir = home.path().join(".config/grove");
        fs::create_dir_all(&dir).expect("fixture config dir");
        let routes: String = EVERY_SESSION_KIND
            .iter()
            .map(|kind| format!("    route {kind:?} \"lead\"\n"))
            .collect();
        let document = format!(
            "config {{\n    command \"runner\" \"true ${{prompt}}\"\n    bind \"lead\" \"runner\"\n{routes}}}\n"
        );
        fs::write(dir.join("config.kdl"), document).expect("fixture config");
        home
    });
    home.path()
}

/// The kinds this repository's own methodology ships, enumerated
/// **here** rather than in the binary: grove no longer holds a set of kinds to
/// validate a configuration against, so a fixture that wants a template for
/// every kind a test might write has to say which those are.
pub const EVERY_SESSION_KIND: &[&str] = &[
    "requirements",
    "review-requirements",
    "integrate-review-requirements",
    "design",
    "review-design",
    "integrate-review-design",
    "planning",
    "review-planning",
    "integrate-review-planning",
    "prototype",
    "review-prototype",
    "integrate-review-prototype",
    "impl",
    "review-impl",
    "integrate-review-impl",
    "research-a",
    "research-b",
    "combine-research",
    "draft",
    "copy-edit",
    "art",
    "proof",
    "finish",
];

/// The repository root, found by walking up from this file's own package.
///
/// This module is compiled into test binaries in **three** packages —
/// `crates/grove`, `crates/grove-llm` and `crates/grove-loop` — so
/// `CARGO_MANIFEST_DIR` is not one answer any more. The workspace manifest is
/// the marker, and it is the only file in this tree that is a `Cargo.toml` with
/// a `[workspace]` table in it.
pub fn repo_root() -> PathBuf {
    let mut dir = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.is_file()
            && fs::read_to_string(&manifest).is_ok_and(|body| body.contains("\n[workspace]"))
        {
            return dir;
        }
        assert!(
            dir.pop(),
            "no workspace manifest above {}",
            env!("CARGO_MANIFEST_DIR")
        );
    }
}

/// The `grove-llm` binary, for a test that drives it from a package that does
/// not build it.
///
/// `env!("CARGO_BIN_EXE_…")` only names a binary of the test's **own** package,
/// and `grove-llm` is its own package now. `cargo test --workspace` — the run
/// `docs/RELEASING.md` and `docs/preservation-baseline.md` both name — compiles
/// every target before running any test, so the file is there; a narrower run
/// gets it built here rather than a confusing *no such file*.
pub fn grove_llm() -> PathBuf {
    workspace_binary("grove-llm")
}

/// The `grove` binary, on the same terms.
pub fn grove_bin() -> PathBuf {
    workspace_binary("grove")
}

fn workspace_binary(name: &str) -> PathBuf {
    // `target/<profile>/deps/<test binary>` — so the profile directory is two
    // levels up from the running test executable, whatever the profile is and
    // wherever `CARGO_TARGET_DIR` put it.
    let exe = std::env::current_exe().expect("a running test has a path");
    let profile = exe
        .parent()
        .and_then(Path::parent)
        .expect("a test binary sits under <target>/<profile>/deps");
    let binary = profile.join(name);
    if !binary.is_file() {
        let built = std::process::Command::new(env!("CARGO"))
            .args(["build", "--quiet", "-p", name])
            .current_dir(repo_root())
            .status()
            .unwrap_or_else(|error| panic!("building {name}: {error}"));
        assert!(built.success(), "building {name} failed");
    }
    assert!(binary.is_file(), "{} was not built", binary.display());
    binary
}

/// Every Rust source file grove itself ships, as `(repo-relative path, body)`.
///
/// **Grove's source is five packages under `crates/`**, and no repository-root
/// `src/` at all: `loop-crate-verbs-k21` split the tree and its verbs into
/// `crates/grove-loop` and the agent CLI into `crates/grove-llm`, and
/// `loop-crate-driver-k22` took the driver after them and left the human binary
/// at `crates/grove`. A sweep anchored on one `src/` would report a clean tree
/// because it stopped looking — and a sweep that merely *kept* a stale root
/// entry would too, silently, since a directory that is not there is skipped
/// rather than refused. Hence the enumeration below is the member list, and the
/// size floor at the foot of this function is what makes a shortened one fail.
/// `crates/ordinal-fs-tree` is excluded deliberately and is the one exclusion:
/// it is the **store**, and the claims these sweeps make — *the store's lock is
/// taken from exactly one module*, *nothing grove locks for itself ever waits* —
/// are claims about grove's side of that seam. The store owns the lock and
/// blocks on it by design.
pub fn grove_sources() -> Vec<(String, String)> {
    let root = repo_root();
    let mut roots = Vec::new();
    for member in [
        "grove",
        "grove-llm",
        "grove-loop",
        "jj-workspace",
        "keyed-launch",
    ] {
        let member_src = root.join("crates").join(member).join("src");
        assert!(
            member_src.is_dir(),
            "{} is not a directory — the member list has gone stale, and a stale \
             entry narrows this sweep silently",
            member_src.display()
        );
        roots.push(member_src);
    }
    let mut found = Vec::new();
    while let Some(path) = roots.pop() {
        if path.is_dir() {
            roots.extend(
                fs::read_dir(&path)
                    .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()))
                    .map(|entry| entry.expect("a readable directory entry").path()),
            );
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        let relative = path
            .strip_prefix(&root)
            .expect("a path under the repository root")
            .display()
            .to_string();
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()));
        found.push((relative, body));
    }
    assert!(
        found.len() > 8,
        "the walk found {} source files — a mis-scoped walk reports a clean \
         surface for the wrong reason",
        found.len()
    );
    found.sort();
    found
}

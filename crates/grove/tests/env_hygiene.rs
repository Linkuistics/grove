// guard-loop-signal-k37 — the suite must not be able to reach a *live* loop's
// control channel.
//
// This repo is a meta-grove: `cargo test` is normally run from inside a running
// `grove do` session, whose environment carries `GROVE_SIGNAL_FILE` — the path
// the loop driver watches, and whose appearance makes it kill the harness child
// (self-driving-loop). The suite's fake harnesses write that path
// unconditionally, so without a guard a harness spawn that fails to scope the
// variable kills the terminal the tests were typed into.
//
// The failure mode these tests exist for is **invisible from inside the suite**:
// the run passes, and the terminal dies. So the guards get assertions rather
// than trust.

use std::path::PathBuf;

mod support;

/// The repository root, where `.cargo/config.toml` lives. Found by walking up
/// rather than by `CARGO_MANIFEST_DIR`, which names this package since
/// `loop-crate-driver-k22` moved these tests off a root package that no longer
/// exists.
fn repo_root() -> PathBuf {
    support::repo_root()
}

/// `.cargo/config.toml` force-clears `GROVE_SIGNAL_FILE` to an empty value for
/// everything cargo runs — deliberately stronger than redirecting it to an inert
/// path, since every nonempty value now carries session-epoch authority. If that
/// entry is ever dropped — or loses `force`, which is the subtle way to break it,
/// since without `force` an inherited value wins silently — this fails instead of
/// the developer's session dying.
#[test]
fn the_suite_cannot_reach_a_live_loop_signal_file() {
    let raw = std::env::var_os("GROVE_SIGNAL_FILE").unwrap_or_else(|| {
        panic!(
            "GROVE_SIGNAL_FILE is unset under `cargo test`, so `.cargo/config.toml`'s \
             [env] guard is not being applied. Restore it: without the override, a run \
             from inside a `grove do` session inherits the LIVE signal path and any \
             fake harness writing it kills that session."
        )
    });
    assert!(
        raw.is_empty(),
        "GROVE_SIGNAL_FILE must be force-cleared under cargo, not redirected to {:?}: \
         any nonempty value now carries session-epoch authority and makes cargo-launched \
         `grove-llm` commands stale-fail before their test seam",
        raw
    );
}

/// The independent half: the shared scrub list every subprocess `env_remove`
/// site reads. A test binary run directly rather than through cargo gets no
/// `.cargo/config.toml` treatment at all, so this list is the only thing
/// standing between the fake harness it spawns and the live path.
///
/// The list has one consumer now, not two. It also fed a `testing/support.rs`
/// guard that scrubbed *this* process by `std::env::remove_var`, and
/// `testing-support-env-guard-soundness-k203` deleted that guard: nothing
/// constructed it, and nothing may reinstate it, because `remove_var` is
/// unsound in a process whose other threads read the environment — which every
/// test binary here is (<https://doc.rust-lang.org/std/env/fn.set_var.html>).
/// So the subprocess path is the only one there is, and this list is all of it.
///
/// That "nothing may" is no longer held by prose alone:
/// `no_first_party_source_mutates_its_own_process_environment`, at the foot of
/// this file, scans every first-party `.rs` file for either call and fails on
/// one. The prose here and in `testing/support.rs`'s header now explains a rule
/// the suite checks, rather than standing in for the check.
#[test]
fn the_shared_scrub_list_covers_the_loop_control_channel() {
    let names = support::grove_env_names();
    assert!(
        names.iter().any(|n| n == "GROVE_SIGNAL_FILE"),
        "grove_env_names() must include GROVE_SIGNAL_FILE — it is the single list \
         every subprocess env_remove call site reads, and since the process-mutating \
         guard was deleted it is the only scrub there is, so dropping it re-opens \
         the kill channel outright"
    );
}

/// Belt and braces are only belt and braces if they are independent. The cargo
/// override cannot help a directly-executed test binary, and the scrub list
/// cannot help a test that never calls the helpers — so neither may be removed
/// on the grounds that the other exists.
#[test]
fn both_guards_are_present_and_neither_subsumes_the_other() {
    let config = repo_root().join(".cargo/config.toml");
    let text = std::fs::read_to_string(&config)
        .unwrap_or_else(|e| panic!("reading {}: {e}", config.display()));

    assert!(
        text.contains("GROVE_SIGNAL_FILE"),
        "{} must carry the [env] override for GROVE_SIGNAL_FILE",
        config.display()
    );
    assert!(
        text.contains("force = true"),
        "the [env] override must set `force = true`; without it an inherited live \
         value silently wins and the guard is decorative"
    );

    // And the scrub list half, asserted from the same test so a reviewer sees
    // the pair stated together rather than inferring it across files.
    assert!(
        support::grove_env_names()
            .iter()
            .any(|n| n == "GROVE_SIGNAL_FILE"),
        "the testing/support.rs scrub list is the other half of this pair"
    );
}

/// Empty is the only inert value on both axes: fake harnesses cannot write a
/// live channel, and the agent CLI treats it as manual/no ambient context.
#[test]
fn the_overridden_signal_context_is_empty() {
    let signal = std::env::var_os("GROVE_SIGNAL_FILE").expect("set by cargo config");
    assert!(
        signal.is_empty(),
        "the cargo override must remove both write authority and epoch admission: {signal:?}"
    );
}

// ---------------------------------------------------------------------------
// The standing gate: no first-party source may mutate its own process
// environment (`env-mutation-standing-gate-k216`).
// ---------------------------------------------------------------------------

/// Directories the scan does not descend into: the two version-control stores,
/// the task tree, its worktrees, and build output. The same set, for the same
/// reason, as `crates/grove/tests/reference_navigation.rs`'s repository-wide
/// Markdown sweep — `target/` alone would otherwise put every vendored
/// dependency's source under a rule that binds only this workspace.
const UNSWEPT_DIRECTORIES: [&str; 5] = [".git", ".jj", ".grove", ".grove-worktrees", "target"];

/// This file, excluded from the scan because the fixture below carries both
/// forbidden calls as live source lines — that is what makes it a control.
/// Named as a constant so the exclusion is one visible decision rather than a
/// silent filter, which is the idiom `reference_navigation.rs` uses for the
/// same hazard.
const THIS_FILE: &str = "crates/grove/tests/env_hygiene.rs";

fn collect_rust_files(directory: &std::path::Path, prefix: &str, into: &mut Vec<String>) {
    let entries = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{} must be readable: {error}", directory.display()));

    for entry in entries {
        let entry = entry.expect("directory entries must be readable");
        let name = entry.file_name().to_string_lossy().into_owned();
        let relative = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };

        if entry
            .file_type()
            .expect("entry type must be readable")
            .is_dir()
        {
            if !UNSWEPT_DIRECTORIES.contains(&name.as_str()) {
                collect_rust_files(&entry.path(), &relative, into);
            }
        } else if name.ends_with(".rs") {
            into.push(relative);
        }
    }
}

/// Every first-party `.rs` file, repository-relative and sorted.
fn rust_sources(root: &std::path::Path) -> Vec<String> {
    let mut paths = Vec::new();
    collect_rust_files(root, "", &mut paths);
    paths.sort();
    paths
}

/// The 1-based line numbers in `source` that name `set_var` or `remove_var`
/// outside a whole-line comment.
///
/// Whole-line comments are skipped because the rule against these two functions
/// is *explained* in prose in four files here, and an explanation is not a call.
/// A trailing comment on a line of code is deliberately **not** skipped: the
/// scan errs towards reporting, since a false positive costs one sentence moved
/// onto its own line and a false negative is the hazard walking back in.
fn environment_mutation_lines(source: &str) -> Vec<usize> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim_start().starts_with("//"))
        .filter(|(_, line)| line.contains("set_var") || line.contains("remove_var"))
        .map(|(index, _)| index + 1)
        .collect()
}

/// `std::env::set_var` and `remove_var` have no sound use in any binary this
/// workspace builds: libtest runs every test on a thread it spawns, and std's
/// rule is that "the only sound option is to not use `set_var` or `remove_var`
/// at all" (<https://doc.rust-lang.org/std/env/fn.set_var.html>).
/// `testing-support-env-guard-soundness-k203` removed the last five call sites
/// and left the rule stated in prose. This is the assertion that keeps it,
/// because under `edition = "2021"` (`Cargo.toml`, `[workspace.package]`) both
/// are safe fns and the compiler says nothing until the edition-2024 migration
/// makes them `unsafe`
/// (<https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html>).
///
/// Two controls, because either alone is decorative. A fixture proves the
/// matcher can tell a call from the prose about one; a reached-files assertion
/// proves the walk enumerated something, since a truncated enumeration agrees
/// with any claim about what it did not find.
#[test]
fn no_first_party_source_mutates_its_own_process_environment() {
    // Control one: the matcher, against a fixture carrying both live forms and
    // a commented one. Checked first, so a matcher that has stopped matching
    // fails here rather than passing the scan in silence.
    let fixture = concat!(
        "fn spawn(path: &str) {\n",
        "    std::env::set_var(\"GROVE_SIGNAL_FILE\", path);\n",
        "    // set_var and remove_var are unsound here — this line explains, it does not call.\n",
        "    std::env::remove_var(\"GROVE_SIGNAL_FILE\");\n",
        "}\n",
    );
    assert_eq!(
        environment_mutation_lines(fixture),
        vec![2, 4],
        "the matcher must report exactly the two live calls and skip the whole-line \
         comment that explains them; a matcher that reports three flags every file \
         that states the rule, and one that reports fewer than two is not reading"
    );

    let root = repo_root();
    let sources = rust_sources(&root);

    // Control two: the walk reached these. Each is picked to hold a different
    // arm of the enumeration — this crate's `tests/`, its `src/`, the shared
    // helpers outside every package, another crate's `tests/`, and another
    // crate's `src/`.
    for expected in [
        THIS_FILE,
        "crates/grove/src/main.rs",
        "testing/support.rs",
        "crates/jj-workspace/tests/environment.rs",
        "crates/grove-loop/src/driver_lease.rs",
    ] {
        assert!(
            sources.iter().any(|source| source == expected),
            "the scan must reach {expected}; it enumerated {} files: {sources:?}",
            sources.len()
        );
    }

    let mut offenders = Vec::new();
    for source in sources.iter().filter(|source| source.as_str() != THIS_FILE) {
        let text = std::fs::read_to_string(root.join(source))
            .unwrap_or_else(|error| panic!("{source} must be readable: {error}"));
        for line in environment_mutation_lines(&text) {
            offenders.push(format!("{source}:{line}"));
        }
    }

    assert!(
        offenders.is_empty(),
        "std::env::set_var / remove_var mutate a process every one of whose other \
         threads may be reading the environment, which libtest guarantees they are; \
         the calls are unsound here and no mutex fixes them, because the race is \
         against readers no library advertises. Remove these:\n  {}\n\
         To give a *child* a different environment, build it with `Command::env` and \
         `Command::env_remove`, which write the child's map and leave this process \
         alone; `testing/support.rs`'s `grove_env_names` is the shared scrub list for \
         that seam. Where the property can only be *observed* under an environment, \
         re-run the test binary as that child under `Command::env` — \
         `crates/jj-workspace/tests/environment.rs` is the worked pattern.",
        offenders.join("\n  ")
    );
}

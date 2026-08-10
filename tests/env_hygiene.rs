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

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// `.cargo/config.toml` force-overrides `GROVE_SIGNAL_FILE` into `target/` for
/// everything cargo runs. If that entry is ever dropped — or loses `force`, which
/// is the subtle way to break it, since without `force` an inherited value wins
/// silently — this fails instead of the developer's session dying.
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

/// The independent half: the shared scrub list every `EnvGuard::clear_grove_env`
/// and every subprocess `env_remove` site reads. A test binary run directly
/// rather than through cargo gets no `.cargo/config.toml` treatment at all, so
/// this list is the only thing standing between it and the live path.
#[test]
fn the_shared_scrub_list_covers_the_loop_control_channel() {
    let names = support::grove_env_names();
    assert!(
        names.iter().any(|n| n == "GROVE_SIGNAL_FILE"),
        "grove_env_names() must include GROVE_SIGNAL_FILE — it is the single list \
         both EnvGuard (this process) and every subprocess env_remove call site \
         read, so dropping it re-opens the kill channel on both paths at once"
    );
}

/// Belt and braces are only belt and braces if they are independent. The cargo
/// override cannot help a directly-executed test binary, and the scrub list
/// cannot help a test that never calls the helpers — so neither may be removed
/// on the grounds that the other exists.
#[test]
fn both_guards_are_present_and_neither_subsumes_the_other() {
    let config = manifest_dir().join(".cargo/config.toml");
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
        "the tests/support scrub list is the other half of this pair"
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

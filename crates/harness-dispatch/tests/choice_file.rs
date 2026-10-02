//! The SDK's choice-file helper, as delivered: a policy imports `readChoice`
//! from the embedded `harness-dispatch/sdk` and selects through the compiled
//! worker (`docs/specs/harness-selection-and-execution.md`, *The sample policy
//! and the choice file*).
//!
//! The policy is `worker/typecheck/choice-policy.ts`, which `task
//! dispatch:typecheck` checks against the shipped declarations, so the
//! declarations and the runtime agree on it. It offers `fast` and `careful`,
//! and takes `careful` where no file says otherwise.

mod support;

use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use support::{text, Sandbox};

const POLICY: &str = include_str!("../worker/typecheck/choice-policy.ts");

fn sandbox() -> (Sandbox, PathBuf) {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(POLICY);
    let choice = sandbox.cwd.join(".harness-dispatch-choice");
    (sandbox, choice)
}

fn inspect(sandbox: &Sandbox) -> support::Run {
    sandbox.inspect(&["--kind", "impl", "--prompt", "the mandate", "--json"])
}

/// The helper's refusal, as the policy returned it.
fn refused(sandbox: &Sandbox, code: &str) -> Value {
    let refusal = inspect(sandbox).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused", "{refusal}");
    assert_eq!(refusal["error"]["policyCode"], code, "{refusal}");
    assert_eq!(refusal["error"]["stage"], "selection", "{refusal}");
    refusal
}

#[test]
fn an_offered_name_selects_and_an_absent_file_leaves_the_default() {
    let (sandbox, choice) = sandbox();

    let report = inspect(&sandbox).report();
    assert_eq!(report["selection"]["effort"], "high");
    assert_eq!(report["selection"]["reason"], "careful, the default");

    // Whitespace of any kind separates names, and surrounds them freely.
    for file in ["fast", "fast\n", "\n\t fast \r\n\n"] {
        fs::write(&choice, file).unwrap();
        let report = inspect(&sandbox).report();
        assert_eq!(report["selection"]["effort"], "low", "{file:?}");
        assert_eq!(
            report["selection"]["reason"], "fast, chosen by .harness-dispatch-choice",
            "{file:?}"
        );
        assert_eq!(
            report["command"]["args"],
            serde_json::json!(["--effort", "low", "the mandate"]),
            "{file:?}"
        );
    }
    // The helper returns every name; how many a selection takes is the
    // policy's own rule, and so is its refusal.
    fs::write(&choice, "fast careful\n").unwrap();
    refused(&sandbox, "choice_not_one");

    // It is an ordinary read: no context, and so no measured source.
    assert_eq!(report["context"], Value::Null);

    // And it launches: the choice decides the argv the harness receives.
    fs::write(&choice, "fast\n").unwrap();
    let run = sandbox.run(&["--kind", "impl", "--prompt", "the mandate"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["--effort", "low", "the mandate"]);
}

#[test]
fn an_unoffered_name_refuses_naming_the_file_the_name_and_the_names_offered() {
    let (sandbox, choice) = sandbox();

    // A near miss is not offered, and neither is anything an object inherits.
    for (file, name) in [
        ("turbo\n", "turbo"),
        ("fast turbo\n", "turbo"),
        ("Fast\n", "Fast"),
        ("constructor\n", "constructor"),
        ("fast,careful\n", "fast,careful"),
    ] {
        fs::write(&choice, file).unwrap();
        let refusal = refused(&sandbox, "choice_unoffered");
        let message = refusal["error"]["message"].as_str().unwrap();
        for fact in [
            text(&choice),
            format!("names {name:?}"),
            "it offers fast, careful".to_owned(),
        ] {
            assert!(
                message.contains(&fact),
                "{file:?} lacks {fact:?}: {refusal}"
            );
        }
        let remedy = refusal["error"]["remedy"].as_str().unwrap();
        assert!(
            remedy.contains("fast, careful") && remedy.contains(&text(&choice)),
            "{refusal}"
        );
        assert!(!sandbox.harness_ran());
    }
}

#[test]
fn a_choice_file_is_a_small_regular_utf8_file() {
    let (sandbox, choice) = sandbox();
    let unreadable = |what: &str, why: &str| {
        let refusal = refused(&sandbox, "choice_unreadable");
        let message = refusal["error"]["message"].as_str().unwrap();
        assert!(
            message.contains(&text(&choice)) && message.contains(why),
            "{what}: {refusal}"
        );
    };

    // 4 KiB is admitted whole; one byte more is refused, never truncated.
    fs::write(&choice, format!("fast{}", " ".repeat(4096 - 4))).unwrap();
    assert_eq!(inspect(&sandbox).report()["selection"]["effort"], "low");
    fs::write(&choice, format!("fast{}", " ".repeat(4097 - 4))).unwrap();
    unreadable("4097 bytes", "holds more than 4096 bytes");

    fs::write(&choice, b"fast \xff\n").unwrap();
    unreadable("not UTF-8", "cannot be read");

    fs::remove_file(&choice).unwrap();
    fs::create_dir(&choice).unwrap();
    unreadable("a directory", "is not a regular file");
    fs::remove_dir(&choice).unwrap();

    // A FIFO is refused without being opened, so it cannot hold the selection.
    support::mkfifo(&choice);
    unreadable("a FIFO", "is not a regular file");
    fs::remove_file(&choice).unwrap();

    // A link to nothing is no file at all.
    std::os::unix::fs::symlink("nowhere", &choice).unwrap();
    assert_eq!(
        inspect(&sandbox).report()["selection"]["reason"],
        "careful, the default"
    );
}

//! Owner settings, through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Policy authority and
//! runtime discovery*).
//!
//! `~/.config/harness-dispatch/settings.json` sets the selection bound, the
//! context budget, the record directory and the worker's grants with no flag
//! passed. Each case reads the value where it takes effect, and not only where
//! it is reported: in the request the policy receives, in the worker's
//! environment, in the directory the harness is handed and in the refusal a
//! bound causes. A flag replaces a setting, `--policy-env` adds to the
//! grants, a malformed file refuses every command before any policy runs, and
//! only the file under HOME is read.

mod support;

use std::fs;
use std::path::Path;

use serde_json::{json, Value};
use support::{run, text, Sandbox};

/// A policy that records, at import, the names in its environment, and whose
/// reason carries the bounds its request holds.
fn viewing_policy(view: &Path) -> String {
    format!(
        r#"import {{ writeFileSync }} from "node:fs";
writeFileSync({view:?}, JSON.stringify(Object.keys(process.env).sort()));
export const policy = {{
  schemaVersion: 2,
  version: "settings-1",
  select: (request) => ({{
    status: "selected", program: "fake-harness", args: [request.prompt],
    provider: "origin-a", model: "model-large", effort: "high",
    reason: `${{request.limits.selectionMs}} ms, ${{request.limits.contextBytes}} bytes`,
  }}),
}};
"#,
        view = text(view),
    )
}

fn worker_environment(view: &Path) -> Value {
    let names = fs::read_to_string(view).expect("the policy recorded its environment");
    fs::remove_file(view).unwrap();
    serde_json::from_str(&names).unwrap()
}

/// A sandbox with the viewing policy and settings for all four keys.
fn settled() -> (Sandbox, std::path::PathBuf, std::path::PathBuf) {
    let sandbox = Sandbox::new();
    let view = sandbox.root.join("view.json");
    let records = sandbox.root.join("owner-records");
    sandbox.personal_policy(&viewing_policy(&view));
    sandbox.settings(&json!({
        "timeoutMs": 45_000,
        "contextBytes": 4096,
        "stateDir": records,
        "policyEnv": ["ROUTER_TOKEN"],
    }));
    (sandbox, view, records)
}

#[test]
fn owner_settings_set_each_value_with_no_flag_passed() {
    let (sandbox, view, records) = settled();
    let from_settings = |value: Value| {
        let mut bound = value;
        bound["from"] = "settings.json".into();
        bound
    };

    let mut command = sandbox.command();
    command
        .env("ROUTER_TOKEN", "router-token-value")
        .args(["inspect", "--kind", "impl", "--json"]);
    let report = run(&mut command).report();
    assert_eq!(
        report["bounds"]["selection"],
        from_settings(json!({ "ms": 45_000 }))
    );
    assert_eq!(
        report["bounds"]["context"],
        from_settings(json!({ "bytes": 4096 }))
    );
    // The budget the settings lowered caps each read, and is its origin.
    assert_eq!(
        report["bounds"]["source"],
        from_settings(json!({ "bytes": 4096 }))
    );
    assert_eq!(
        report["stateDir"],
        from_settings(json!({ "path": text(&records) }))
    );
    assert_eq!(
        report["policyEnv"],
        json!([{ "name": "ROUTER_TOKEN", "set": true }])
    );
    // Where each takes effect: the policy's request and the worker's own
    // environment.
    assert_eq!(report["selection"]["reason"], "45000 ms, 4096 bytes");
    assert!(
        worker_environment(&view)
            .as_array()
            .unwrap()
            .contains(&json!("ROUTER_TOKEN")),
        "the granted name did not reach the worker"
    );
    assert!(
        !report.to_string().contains("router-token-value"),
        "a granted value was printed"
    );

    let mut command = sandbox.command();
    command.args(["inspect", "--kind", "impl"]);
    let human = run(&mut command);
    assert_eq!(human.code, Some(0), "{}", human.stderr);
    for row in [
        "selection within 45000 ms (settings.json)".to_owned(),
        "context at most 4096 bytes (settings.json)".to_owned(),
        format!("{} (settings.json)", records.display()),
        "ROUTER_TOKEN (not set)".to_owned(),
    ] {
        assert!(human.stdout.contains(&row), "{row}:\n{}", human.stdout);
    }

    // `run` records where the settings say, hands the harness that directory,
    // and the record commands find the run there with no flag either.
    let launched = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(launched.code, Some(0), "{}", launched.stderr);
    assert_eq!(sandbox.harness_state_dir(), text(&records));
    assert!(records.join("records.sqlite3").is_file());
    assert!(!sandbox.default_store().exists());
    let run_id = sandbox.harness_run_id();
    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id, "--json"]);
    let export = run(&mut show).report();
    assert_eq!(export["runId"], run_id.as_str());
    assert_eq!(
        export["launch"]["bounds"]["selection"],
        from_settings(json!({ "ms": 45_000 }))
    );
}

#[test]
fn a_bound_the_settings_set_is_the_one_that_refuses() {
    let (sandbox, _, _) = settled();
    // A caller context over the settings' 4096-byte budget.
    sandbox.file(
        "context.json",
        &json!({ "schemaVersion": 1, "summary": "x".repeat(8192) }).to_string(),
    );
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--context",
            "context.json",
            "--json",
        ])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "context_too_large", "{refusal}");
    assert_eq!(
        refusal["error"]["bound"],
        json!({ "name": "context", "bytes": 4096, "from": "settings.json" })
    );
    assert!(!sandbox.harness_ran());
    // The same document under a flag that replaces the setting selects, so
    // the setting is what refused it.
    let selected = sandbox.run(&[
        "--kind",
        "impl",
        "--prompt",
        "p",
        "--context",
        "context.json",
        "--context-bytes",
        "65536",
    ]);
    assert_eq!(selected.code, Some(0), "{}", selected.stderr);
    assert!(sandbox.harness_ran());
}

#[test]
fn a_flag_replaces_each_setting_and_policy_env_adds_to_the_grants() {
    let (sandbox, view, records) = settled();
    let mut command = sandbox.command();
    command
        .env("ROUTER_TOKEN", "a")
        .env("OTHER_TOKEN", "b")
        .args([
            "inspect",
            "--kind",
            "impl",
            "--timeout-ms",
            "20000",
            "--context-bytes",
            "8192",
            "--state-dir",
            "flagged-records",
            "--policy-env",
            "OTHER_TOKEN",
            "--policy-env",
            "ROUTER_TOKEN",
            "--json",
        ]);
    let report = run(&mut command).report();
    assert_eq!(
        report["bounds"]["selection"],
        json!({ "ms": 20_000, "from": "--timeout-ms" })
    );
    assert_eq!(
        report["bounds"]["context"],
        json!({ "bytes": 8192, "from": "--context-bytes" })
    );
    assert_eq!(
        report["stateDir"],
        json!({ "path": text(&sandbox.cwd.join("flagged-records")), "from": "--state-dir" })
    );
    assert_ne!(report["stateDir"]["path"], text(&records).as_str());
    // The settings' name first, then the flag's; the repeat is one grant.
    assert_eq!(
        report["policyEnv"],
        json!([
            { "name": "ROUTER_TOKEN", "set": true },
            { "name": "OTHER_TOKEN", "set": true },
        ])
    );
    assert_eq!(report["selection"]["reason"], "20000 ms, 8192 bytes");
    let names = worker_environment(&view);
    for granted in ["ROUTER_TOKEN", "OTHER_TOKEN"] {
        assert!(
            names.as_array().unwrap().contains(&json!(granted)),
            "{granted} did not reach the worker: {names}"
        );
    }
}

#[test]
fn a_malformed_settings_file_refuses_every_command_before_any_policy_runs() {
    const RUN: &str = "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34";
    for (contents, code, key) in [
        ("[]", "settings_invalid", None),
        ("{ \"timeoutMs\": ", "settings_invalid", None),
        (
            r#"{ "config": "/p/policy.ts" }"#,
            "settings_invalid",
            Some("config"),
        ),
        (
            r#"{ "stateDir": "records" }"#,
            "settings_invalid",
            Some("stateDir"),
        ),
        (
            r#"{ "timeoutMs": 600001 }"#,
            "malformed_input",
            Some("timeoutMs"),
        ),
        (
            r#"{ "contextBytes": 0 }"#,
            "malformed_input",
            Some("contextBytes"),
        ),
        (
            r#"{ "policyEnv": ["BUN_OPTIONS"] }"#,
            "excluded_grant",
            Some("policyEnv"),
        ),
    ] {
        let sandbox = Sandbox::new();
        let view = sandbox.root.join("view.json");
        sandbox.personal_policy(&viewing_policy(&view));
        let file = sandbox.settings_path();
        support::write(&file, contents);
        for command in [
            &["inspect", "--kind", "impl", "--json"][..],
            &["run", "--kind", "impl", "--prompt", "p", "--json"],
            &["record", "show", "--run", RUN, "--json"],
        ] {
            let mut invocation = sandbox.command();
            invocation.args(command);
            let refusal = run(&mut invocation).refusal(2);
            let context = format!("{contents} {command:?}: {refusal}");
            assert_eq!(refusal["error"]["code"], code, "{context}");
            assert_eq!(refusal["error"]["stage"], "cli", "{context}");
            assert_eq!(
                refusal["error"]["source"],
                text(&file).as_str(),
                "{context}"
            );
            assert_eq!(
                refusal["error"]["location"].as_str(),
                key,
                "the key: {context}"
            );
            assert!(
                refusal["error"]["message"]
                    .as_str()
                    .unwrap()
                    .contains(&text(&file)),
                "{context}"
            );
        }
        assert!(!view.exists(), "{contents}: the policy ran");
        assert!(!sandbox.harness_ran(), "{contents}: a harness ran");
    }
}

#[test]
fn only_the_file_under_home_is_read() {
    // The same settings in the caller's directory, in a `.config` there, and
    // under an XDG_CONFIG_HOME: none is the owner's file, and each bound stays
    // at its default.
    let sandbox = Sandbox::new();
    let view = sandbox.root.join("view.json");
    sandbox.personal_policy(&viewing_policy(&view));
    let settings = json!({ "timeoutMs": 45_000, "policyEnv": ["ROUTER_TOKEN"] }).to_string();
    let xdg = sandbox.root.join("xdg");
    sandbox.file("settings.json", &settings);
    sandbox.file(".config/harness-dispatch/settings.json", &settings);
    support::write(&xdg.join("harness-dispatch/settings.json"), &settings);

    let inspect = || {
        let mut command = sandbox.command();
        command
            .env("XDG_CONFIG_HOME", &xdg)
            .args(["inspect", "--kind", "impl", "--json"]);
        run(&mut command).report()
    };
    let report = inspect();
    assert_eq!(
        report["bounds"]["selection"],
        json!({ "ms": 30_000, "from": "default" })
    );
    assert_eq!(report["policyEnv"], json!([]));

    // The control: the same bytes under HOME are read.
    support::write(&sandbox.settings_path(), &settings);
    let report = inspect();
    assert_eq!(
        report["bounds"]["selection"],
        json!({ "ms": 45_000, "from": "settings.json" })
    );
    assert_eq!(
        report["policyEnv"],
        json!([{ "name": "ROUTER_TOKEN", "set": false }])
    );
}

#[test]
fn help_names_the_settings_file_and_its_keys() {
    let sandbox = Sandbox::new();
    let mut command = sandbox.command();
    command.arg("--help");
    let help = run(&mut command);
    assert_eq!(help.code, Some(0), "{}", help.stderr);
    for fact in [
        "~/.config/harness-dispatch/settings.json",
        "timeoutMs, contextBytes, stateDir (an absolute path) and policyEnv",
        "A flag replaces its setting, and --policy-env adds to policyEnv",
    ] {
        assert!(
            help.stdout.contains(fact),
            "help lacks {fact:?}:\n{}",
            help.stdout
        );
    }
    let inspect = sandbox.inspect(&["--help"]);
    assert!(
        inspect.stdout.contains("1000 to 600000"),
        "{}",
        inspect.stdout
    );
}

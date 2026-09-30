//! The policy worker's environment, through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Policy authority and
//! runtime discovery*).
//!
//! The worker gets HOME, PATH, TMPDIR, LANG and `LC_*`, plus the exact names
//! `--policy-env` grants, and the front's own setting that turns Bun's
//! runtime transpiler cache off. Each case here reads the environment from
//! inside the worker, and from a child the policy spawns, so an absence is
//! observed where it matters. Each absence has a control in the same test:
//! the same name, granted, is seen to arrive through the same instrument.

mod support;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{json, Value};
use support::{run, text, Sandbox};

/// Grove's completion values: the channel, and the two retired PID handles
/// the loop driver still scrubs (`crates/grove-loop/src/loop_driver.rs`).
const COMPLETION: [&str; 3] = ["GROVE_SIGNAL_FILE", "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID"];

/// A routes policy that records, at import, the environment it was given and
/// the one a child it spawns inherits: every name, and the value of each
/// name in `values`.
fn viewing_policy(view: &Path, values: &[&str]) -> String {
    format!(
        r#"import {{ writeFileSync }} from "node:fs";
import {{ execFileSync }} from "node:child_process";
const names = (lines: string) => lines.split("\n").filter(Boolean).map((line) => line.slice(0, line.indexOf("="))).sort();
const child = execFileSync("/usr/bin/env", [], {{ encoding: "utf8" }});
const childValues = Object.fromEntries(child.split("\n").filter(Boolean).map((line) => [line.slice(0, line.indexOf("=")), line.slice(line.indexOf("=") + 1)]));
const wanted = {values:?};
writeFileSync({view:?}, JSON.stringify({{
  worker: Object.keys(process.env).sort(),
  child: names(child),
  workerValues: Object.fromEntries(wanted.map((name) => [name, process.env[name] ?? null])),
  childValues: Object.fromEntries(wanted.map((name) => [name, childValues[name] ?? null])),
}}));
{routed}"#,
        view = text(view),
        routed = support::ROUTED,
    )
}

fn view(path: &Path) -> Value {
    let view = fs::read_to_string(path).expect("the policy recorded its environment");
    fs::remove_file(path).unwrap();
    serde_json::from_str(&view).unwrap()
}

fn names(list: &Value) -> Vec<String> {
    list.as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn a_grant_passes_exactly_the_named_variables_beyond_the_base_set() {
    let sandbox = Sandbox::new();
    let view_path = sandbox.root.join("view.json");
    sandbox.personal_policy(&viewing_policy(
        &view_path,
        &[
            "ROUTER_TOKEN",
            "NODE_EXTRA_CA_CERTS",
            "ROUTER_TOKENS",
            "BUN_RUNTIME_TRANSPILER_CACHE_PATH",
        ],
    ));
    // The caller's own cache setting names a cache; the front's replaces it.
    let caller = [
        ("ROUTER_TOKEN", "router-token-value"),
        ("ROUTER_TOKENS", "a longer name, never granted"),
        ("NODE_EXTRA_CA_CERTS", "/etc/ssl/owner.pem"),
        ("UNRELATED", "never granted"),
        ("LANG", "C"),
        ("BUN_RUNTIME_TRANSPILER_CACHE_PATH", "/caller/cache"),
    ];

    let mut command = sandbox.command();
    command
        .envs(caller)
        .args(["inspect", "--kind", "impl", "--json"]);
    run(&mut command).report();
    let base = view(&view_path);
    assert_eq!(
        base["worker"],
        json!([
            "BUN_RUNTIME_TRANSPILER_CACHE_PATH",
            "HOME",
            "LANG",
            "PATH",
            "TMPDIR"
        ])
    );
    for place in ["workerValues", "childValues"] {
        assert_eq!(
            base[place]["BUN_RUNTIME_TRANSPILER_CACHE_PATH"], "0",
            "{place}: {base}"
        );
    }

    // The same caller, with two names granted, one granted twice, and one
    // the caller does not have. NODE_EXTRA_CA_CERTS is a NODE_* name beside
    // the two excluded ones, and is grantable.
    let mut command = sandbox.command();
    command.envs(caller).args([
        "inspect",
        "--kind",
        "impl",
        "--policy-env",
        "ROUTER_TOKEN",
        "--policy-env",
        "NODE_EXTRA_CA_CERTS",
        "--policy-env",
        "ROUTER_TOKEN",
        "--policy-env",
        "UNSET_GRANT",
        "--json",
    ]);
    let report = run(&mut command).report();
    let granted = view(&view_path);
    assert_eq!(
        granted["worker"],
        json!([
            "BUN_RUNTIME_TRANSPILER_CACHE_PATH",
            "HOME",
            "LANG",
            "NODE_EXTRA_CA_CERTS",
            "PATH",
            "ROUTER_TOKEN",
            "TMPDIR"
        ])
    );
    assert_eq!(
        granted["workerValues"],
        json!({
            "ROUTER_TOKEN": "router-token-value",
            "NODE_EXTRA_CA_CERTS": "/etc/ssl/owner.pem",
            "ROUTER_TOKENS": null,
            "BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0",
        })
    );
    // A child the policy spawns inherits the worker's environment, grants
    // included, and nothing else of the caller's.
    assert_eq!(granted["childValues"], granted["workerValues"]);
    for name in names(&granted["child"]) {
        assert!(
            names(&granted["worker"]).contains(&name)
                || ["PWD", "SHLVL", "_"].contains(&name.as_str()),
            "the policy's child has {name}, which the worker lacks: {granted}"
        );
    }
    assert_eq!(
        report["policyEnv"],
        json!([
            { "name": "ROUTER_TOKEN", "set": true },
            { "name": "NODE_EXTRA_CA_CERTS", "set": true },
            { "name": "UNSET_GRANT", "set": false },
        ])
    );
}

#[test]
fn excluded_names_are_refused_before_any_policy_runs() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"ran\");\n{}",
        text(&sentinel),
        support::ROUTED
    ));
    for name in [
        "BUN_OPTIONS",
        "BUN_BE_BUN",
        "BUN_INSTALL",
        "NODE_OPTIONS",
        "NODE_PATH",
        "NODE_PRESERVE_SYMLINKS",
        "NODE_CHANNEL_FD",
        "NODE_CHANNEL_SERIALIZATION_MODE",
        "LD_PRELOAD",
        "LD_LIBRARY_PATH",
        "DYLD_INSERT_LIBRARIES",
        "DYLD_LIBRARY_PATH",
        "HARNESS_DISPATCH_STATE_DIR",
        "HARNESS_DISPATCH_RUN_ID",
    ] {
        // The refusal is the name's, set or not. A loader variable is left
        // unset, because the loader would act on it in the front itself,
        // before any of its code runs: dyld aborts a process whose inserted
        // library does not exist.
        let loader = name.starts_with("LD_") || name.starts_with("DYLD_");
        let value = format!("{name}-value-never-shown");
        let caller: Vec<(&str, &str)> = if loader { vec![] } else { vec![(name, &value)] };
        let mut command = sandbox.command();
        command.envs(caller.iter().copied()).args([
            "inspect",
            "--kind",
            "impl",
            "--policy-env",
            name,
            "--json",
        ]);
        let inspected = run(&mut command);
        let refusal = inspected.refusal(2);
        let error = &refusal["error"];
        assert_eq!(error["code"], "excluded_grant", "{name}: {refusal}");
        assert_eq!(error["stage"], "cli", "{name}");
        assert_eq!(error["input"], "--policy-env", "{name}");
        assert!(
            error["message"].as_str().unwrap().contains(name),
            "{name}: {refusal}"
        );
        assert!(
            !inspected.stderr.contains(&value),
            "{name}'s value was shown"
        );

        let mut command = sandbox.command();
        command.envs(caller.iter().copied()).args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--policy-env",
            name,
            "--json",
        ]);
        let ran = run(&mut command);
        assert_eq!(ran.refusal(2)["error"]["code"], "excluded_grant", "{name}");
        assert!(!ran.stderr.contains(&value), "{name}'s value was shown");
        assert!(!sandbox.harness_ran(), "{name}: the harness ran");
    }
    assert!(!sentinel.exists(), "a policy ran beside an excluded grant");

    // The control: the policy does run, with a near miss of each class granted.
    let mut command = sandbox.command();
    command.args(["inspect", "--kind", "impl", "--json"]);
    for name in [
        "BUNDLE_GEMFILE",
        "NODE_PRESERVE_SYMLINKS_MAIN",
        "NODE_CHANNEL",
        "LDFLAGS",
        "HARNESS_DISPATCHER",
    ] {
        command.args(["--policy-env", name]);
    }
    run(&mut command).report();
    assert!(sentinel.exists(), "the policy never ran");
}

#[test]
fn a_malformed_grant_name_refuses() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(support::ROUTED);
    for name in ["", "TOKEN=value"] {
        let refusal = sandbox
            .inspect(&["--kind", "impl", "--policy-env", name, "--json"])
            .refusal(2);
        assert_eq!(refusal["error"]["code"], "malformed_input", "{name:?}");
        assert_eq!(refusal["error"]["input"], "--policy-env", "{name:?}");
    }
}

#[test]
fn inspection_and_records_name_each_grant_and_never_show_its_value() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(support::ROUTED);
    let value = "grant-value-7f3a9c";
    let state = sandbox.root.join("records");
    let state = text(&state);
    let grants = [
        "--policy-env",
        "ROUTER_TOKEN",
        "--policy-env",
        "UNSET_GRANT",
    ];
    let command = |args: &[&str]| {
        let mut command = sandbox.command();
        command
            .env("ROUTER_TOKEN", value)
            .args(args)
            .args(grants)
            .args(["--state-dir", &state]);
        command
    };

    let json = run(&mut command(&["inspect", "--kind", "impl", "--json"]));
    assert_eq!(
        json.report()["policyEnv"],
        json!([{ "name": "ROUTER_TOKEN", "set": true }, { "name": "UNSET_GRANT", "set": false }])
    );
    let human = run(&mut command(&["inspect", "--kind", "impl"]));
    assert_eq!(human.code, Some(0), "{}", human.stderr);
    assert!(
        human.stdout.contains(
            "  policy env ROUTER_TOKEN (set), UNSET_GRANT (not set); values are never shown\n"
        ),
        "{}",
        human.stdout
    );
    // A refused run reproduces its selection with the names alone.
    let refused = run(&mut command(&[
        "run", "--kind", "impl", "--choice", "absent", "--prompt", "p", "--json",
    ]));
    let argv = refused.refusal(3)["error"]["inspect"]["argv"].clone();
    let argv: Vec<&str> = argv
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w.as_str().unwrap())
        .collect();
    assert!(
        argv.windows(4).any(|w| w == grants),
        "the inspect invocation lacks the grants: {argv:?}"
    );
    let ran = run(&mut command(&[
        "run", "--kind", "impl", "--prompt", "p", "--json",
    ]));
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    let notice: Value = serde_json::from_str(ran.stderr.trim_end()).unwrap();
    let run_id = notice["handoff"]["runId"].as_str().unwrap().to_owned();
    let mut record = sandbox.command();
    record.args([
        "record",
        "show",
        "--run",
        &run_id,
        "--state-dir",
        &state,
        "--json",
    ]);
    let record = run(&mut record);
    assert_eq!(record.code, Some(0), "{}", record.stderr);

    for (what, output) in [
        ("inspect --json", &json.stdout),
        ("inspect", &human.stdout),
        ("inspect stderr", &human.stderr),
        ("the refused run", &refused.stderr),
        ("the run's notice", &ran.stderr),
        ("record show", &record.stdout),
    ] {
        assert!(
            !output.contains(value),
            "{what} showed a granted value: {output}"
        );
    }
    let store = fs::read(Path::new(&state).join("records.sqlite3")).unwrap();
    assert!(
        !store
            .windows(value.len())
            .any(|bytes| bytes == value.as_bytes()),
        "the record store holds a granted value"
    );
}

#[test]
fn the_worker_and_a_child_it_spawns_lack_completion_values_unless_granted() {
    let sandbox = Sandbox::new();
    let view_path = sandbox.root.join("view.json");
    sandbox.personal_policy(&viewing_policy(&view_path, &COMPLETION));
    // A scratch path, never a live loop's: `.cargo/config.toml` clears the
    // real one for everything cargo runs, and this sets its own.
    let channel = text(&sandbox.root.join("completion-channel"));
    let caller: BTreeMap<&str, &str> = [
        ("GROVE_SIGNAL_FILE", channel.as_str()),
        ("GROVE_HARNESS_PID", "4242"),
        ("GROVE_CLAUDE_PID", "4343"),
    ]
    .into();

    let mut command = sandbox.command();
    command
        .envs(&caller)
        .args(["run", "--kind", "impl", "--prompt", "p", "--json"]);
    let ran = run(&mut command);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    let scrubbed = view(&view_path);
    for place in ["worker", "child"] {
        for name in COMPLETION {
            assert!(
                !names(&scrubbed[place]).iter().any(|held| held == name),
                "the policy's {place} holds {name}: {scrubbed}"
            );
        }
    }
    // The final harness, and only it, receives the caller's completion
    // values, unchanged.
    let harness = sandbox.harness_env();
    for name in COMPLETION {
        assert!(
            harness.iter().any(|held| held == name),
            "the harness lacks {name}"
        );
    }
    assert!(!Path::new(&channel).exists(), "something wrote the channel");

    // The control: granted, the channel is seen by the same instrument in
    // the worker and its child, while the PIDs stay out.
    fs::remove_dir_all(&sandbox.record).unwrap();
    let mut command = sandbox.command();
    command.envs(&caller).args([
        "run",
        "--kind",
        "impl",
        "--prompt",
        "p",
        "--policy-env",
        "GROVE_SIGNAL_FILE",
        "--json",
    ]);
    let ran = run(&mut command);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    let granted = view(&view_path);
    for place in ["workerValues", "childValues"] {
        assert_eq!(
            granted[place],
            json!({ "GROVE_SIGNAL_FILE": channel, "GROVE_HARNESS_PID": null, "GROVE_CLAUDE_PID": null }),
            "{place}"
        );
    }
    assert!(sandbox.harness_ran());
}

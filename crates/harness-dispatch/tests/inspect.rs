//! `inspect` through the command seam: the selected command and its labels,
//! both report forms, the prompt marker, policy validation and load failures.

mod support;

use serde_json::{json, Value};
use support::{text, Sandbox, ROUTED};

/// What `select` receives as the prompt when `inspect` was given none.
const MARKER: &str = "<harness-dispatch inspect: no prompt was supplied>";

#[test]
fn inspection_reports_the_selected_command_and_its_evidence_in_json() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(ROUTED);

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();

    assert_eq!(report["schemaVersion"], 2);
    assert_eq!(report["evidence"], "proposal");
    assert_eq!(report["kind"], "impl");
    assert_eq!(report["policy"]["path"], text(&entry));
    assert_eq!(report["policy"]["authority"], "personal");
    assert_eq!(report["policy"]["version"], "seam-1");
    assert_eq!(
        report["selection"],
        json!({
            "provider": "origin-a",
            "model": "model-large",
            "effort": "high",
            "reason": "impl runs the deep harness",
        })
    );
    assert!(report["timing"]["selectionMs"].is_u64(), "{report}");
    assert_eq!(
        report["worker"]["packageVersion"],
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(report["worker"]["bunVersion"], "1.4.2");
    assert_eq!(report["taskFile"], Value::Null);
    assert_eq!(report["taskId"], Value::Null);
    assert_eq!(report["params"], json!({}));
    // Without a prompt `select` receives the marker, so the marker is where
    // the policy placed the prompt, and the report says none was supplied.
    assert_eq!(
        report["prompt"],
        json!({ "supplied": false, "marker": MARKER })
    );
    // The command is the program and arguments `select` returned and the file
    // the program resolved to: what a caller that launches it itself executes.
    assert_eq!(
        report["command"],
        json!({
            "program": "fake-harness",
            "args": [MARKER],
            "executable": text(&sandbox.bin.join("fake-harness")),
        })
    );
    // Inspection creates no run, so it reports no run ID.
    let fields: Vec<&str> = report
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        fields,
        [
            "adapter",
            "bounds",
            "command",
            "context",
            "creator",
            "diagnostics",
            "evidence",
            "kind",
            "params",
            "policy",
            "policyEnv",
            "prompt",
            "reviewedArtifact",
            "schemaVersion",
            "selection",
            "stateDir",
            "taskFile",
            "taskId",
            "timing",
            "worker",
        ]
    );
}

#[test]
fn inspection_shows_the_prompt_and_every_caller_value_where_select_placed_it() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(
        r#"export const policy = { schemaVersion: 2, version: "v", select: (request) => ({
  status: "selected", program: "fake-harness",
  args: ["--kind", request.kind, request.taskFile, request.taskId, `--session=${request.params.session_name}`, "-C", request.params.repo, "{prompt}", request.prompt],
  provider: "origin-a", model: "model-large", effort: "high", reason: "every caller value has an argument",
}) };
"#,
    );
    let prompt = "Say \"hi\";\nthen stop.\n\n";

    let report = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--prompt",
            prompt,
            "--task-file",
            "leaf.md",
            "--task-id",
            "T 1",
            "--param",
            "repo=/work/my repo",
            "--param",
            "session_name=parser: a=b",
            "--json",
        ])
        .report();

    let task_file = text(&sandbox.cwd.join("leaf.md"));
    assert_eq!(report["taskFile"], task_file.as_str());
    assert_eq!(report["taskId"], "T 1");
    assert_eq!(
        report["params"],
        json!({ "repo": "/work/my repo", "session_name": "parser: a=b" })
    );
    assert_eq!(
        report["prompt"],
        json!({ "supplied": true, "from": "--prompt", "bytes": prompt.len() })
    );
    assert_eq!(report["command"]["program"], "fake-harness");
    assert_eq!(
        report["command"]["args"],
        json!([
            "--kind",
            "impl",
            task_file,
            "T 1",
            "--session=parser: a=b",
            "-C",
            "/work/my repo",
            "{prompt}",
            prompt,
        ])
    );
}

#[test]
fn a_complete_inspection_names_its_authority_its_measured_sources_and_where_each_bound_came_from() {
    let sandbox = Sandbox::new();
    let entry = sandbox.file("policies/mine.ts", ROUTED);
    let document = r#"{ "schemaVersion": 1, "summary": "a small change" }"#;
    let context = sandbox.file("context.json", document);
    let records = sandbox.root.join("records");

    let report = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--config",
            "policies/mine.ts",
            "--context",
            "context.json",
            "--timeout-ms",
            "20000",
            "--context-bytes",
            "4096",
            "--state-dir",
            &text(&records),
            "--json",
        ])
        .report();

    let policy = &report["policy"];
    assert_eq!(policy["path"], text(&entry));
    assert_eq!(policy["authority"], "explicit");
    assert_eq!(policy["argument"], "policies/mine.ts");
    assert_eq!(policy["version"], "seam-1");
    let digest = |value: &Value| {
        value.as_str().is_some_and(|digest| {
            digest.len() == 64
                && digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        })
    };
    assert!(digest(&policy["sha256"]), "{report}");

    // The caller's document is the one measured source, with the bytes read
    // and their digest, and the delivered value is shown whole.
    let delivered = &report["context"];
    assert_eq!(delivered["loader"], false);
    let sources = delivered["sources"].as_array().unwrap();
    assert_eq!(sources.len(), 1, "{report}");
    assert_eq!(sources[0]["name"], text(&context));
    assert_eq!(sources[0]["via"], "--context");
    assert_eq!(sources[0]["bytes"], document.len());
    assert!(digest(&sources[0]["sha256"]), "{report}");
    assert_eq!(delivered["sourceBytes"], document.len());
    assert!(delivered["encodedBytes"].is_u64(), "{report}");
    assert!(digest(&delivered["sha256"]), "{report}");
    assert_eq!(delivered["value"]["summary"], "a small change");
    assert_eq!(delivered["value"]["measured"], delivered["sources"]);

    assert_eq!(
        report["bounds"],
        json!({
            "selection": { "ms": 20_000, "from": "--timeout-ms" },
            "context": { "bytes": 4096, "from": "--context-bytes" },
            "source": { "bytes": 4096, "from": "--context-bytes" },
            "sources": { "sources": 256, "from": "fixed" },
            "message": { "bytes": 1_048_576, "from": "fixed" },
            "diagnostics": { "bytes": 262_144, "from": "fixed" },
        })
    );
    assert_eq!(
        report["stateDir"],
        json!({ "path": text(&records), "from": "--state-dir" })
    );
    assert_eq!(report["policyEnv"], json!([]));
    assert_eq!(report["adapter"], Value::Null);
    assert_eq!(report["creator"], Value::Null);
    assert_eq!(report["reviewedArtifact"], Value::Null);
    assert!(!records.exists(), "inspection created the record directory");

    // With nothing set, every adjustable bound is its default, and says so.
    sandbox.personal_policy(ROUTED);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    for (bound, unit, value) in [
        ("selection", "ms", 30_000),
        ("context", "bytes", 262_144),
        ("source", "bytes", 65_536),
    ] {
        assert_eq!(
            report["bounds"][bound],
            json!({ unit: value, "from": "default" }),
            "{bound}"
        );
    }
    assert_eq!(report["stateDir"]["from"], "default");
    assert_eq!(report["context"], Value::Null);
}

#[test]
fn inspection_reports_the_same_facts_as_human_text() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(ROUTED);

    let run = sandbox.inspect(&["--kind", "impl"]);

    assert_eq!(run.code, Some(0), "{}", run.stderr);
    for fact in [
        "Proposal only: nothing was launched".to_owned(),
        format!("policy     {}", text(&entry)),
        "authority  personal (the default ~/.config/harness-dispatch/policy.ts)".to_owned(),
        "version    seam-1".to_owned(),
        "kind       impl".to_owned(),
        "params     none".to_owned(),
        "provider   origin-a".to_owned(),
        "model      model-large".to_owned(),
        "effort     high".to_owned(),
        "reason     impl runs the deep harness".to_owned(),
        "timing     selection took ".to_owned(),
    ] {
        assert!(
            run.stdout.contains(&fact),
            "missing {fact:?} in:\n{}",
            run.stdout
        );
    }
    assert!(
        run.stdout.contains(&format!(
            "executable {} (found on PATH in {})",
            text(&sandbox.bin.join("fake-harness")),
            text(&sandbox.bin)
        )),
        "{}",
        run.stdout
    );
    assert!(
        run.stdout.contains(&format!(
            "prompt     not supplied; select received the marker {MARKER}\n"
        )),
        "{}",
        run.stdout
    );
    assert!(
        run.stdout.contains("  argv       [0] \"fake-harness\"\n")
            && run
                .stdout
                .contains(&format!("             [1] \"{MARKER}\"\n")),
        "{}",
        run.stdout
    );
    assert!(!run.stdout.contains("run id"), "{}", run.stdout);

    // A supplied prompt appears quoted and escaped, whole, and each parameter
    // as `name=value`, quoted where a space would run two together.
    let run = sandbox.inspect(&[
        "--kind",
        "impl",
        "--prompt",
        "two\nlines \"quoted\"",
        "--param",
        "repo=/r",
        "--param",
        "session_name=a b",
    ]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stdout.contains(r#"[1] "two\nlines \"quoted\"""#),
        "{}",
        run.stdout
    );
    assert!(
        run.stdout.contains("prompt     18 bytes from --prompt"),
        "{}",
        run.stdout
    );
    assert!(
        run.stdout
            .contains("params     repo=/r \"session_name=a b\"\n"),
        "{}",
        run.stdout
    );
}

#[test]
fn the_type_checked_fixture_selects_through_the_embedded_sdk() {
    // It imports `definePolicy` from `harness-dispatch/sdk`, and there is no
    // node_modules anywhere near the sandbox: only the worker's registered
    // virtual module can satisfy the import.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(include_str!("../worker/typecheck/routes-policy.ts"));

    let report = sandbox
        .inspect(&["--kind", "design", "--prompt", "the prompt", "--json"])
        .report();
    assert_eq!(report["policy"]["version"], "typecheck-fixture-1");
    assert_eq!(
        report["selection"],
        json!({
            "provider": "origin-a",
            "model": "model-large",
            "effort": "high",
            "reason": r#"table["design"]"#,
        })
    );
    assert_eq!(
        report["command"]["args"],
        json!(["--model", "model-large", "--effort", "high", "the prompt"])
    );

    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt", "the prompt", "--json"])
        .report();
    assert_eq!(report["selection"]["provider"], "origin-b");
    assert_eq!(report["selection"]["reason"], r#"table["impl"]"#);
    assert_eq!(report["command"]["args"], json!(["the prompt"]));

    // A kind its table lacks is the policy's own refusal.
    let refusal = sandbox.inspect(&["--kind", "review", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused");
    assert_eq!(refusal["error"]["policyCode"], "incomplete_mapping");
}

#[test]
fn a_kind_the_policy_refuses_is_reported_as_the_policys_own_refusal_and_nothing_is_run_instead() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(ROUTED);

    let refusal = sandbox.inspect(&["--kind", "design", "--json"]).refusal(3);

    let error = &refusal["error"];
    assert_eq!(error["code"], "policy_refused");
    assert_eq!(error["policyCode"], "incomplete_mapping");
    assert_eq!(error["stage"], "selection");
    assert_eq!(error["input"], "--kind design");
    assert_eq!(error["source"], text(&entry));
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .ends_with(r#"refused the selection: no command for kind "design""#),
        "{refusal}"
    );
    assert_eq!(error["remedy"], "add one to the policy");

    let run = sandbox.inspect(&["--kind", "design"]);
    assert_eq!(run.code, Some(3));
    assert_eq!(run.stdout, "");
    assert!(
        run.stderr
            .contains("refused (policy_refused, stage selection)"),
        "{}",
        run.stderr
    );
    assert!(
        run.stderr.contains("  policy code: incomplete_mapping\n"),
        "{}",
        run.stderr
    );
    assert!(
        run.stderr.contains("  remedy: add one to the policy\n"),
        "{}",
        run.stderr
    );
}

/// A selected result every valid twin below returns.
const SELECTED: &str = r#"{ status: "selected", program: "fake-harness", args: [request.prompt], provider: "origin-a", model: "m", effort: "high", reason: "the one command" }"#;

fn policy(fields: &str) -> String {
    format!(
        "export const policy = {{ {} }};\n",
        fields.replace("$S", &format!("select: (request) => ({SELECTED})"))
    )
}

#[test]
fn every_invalid_policy_shape_refuses_with_its_location() {
    let cases: Vec<(&str, String, &str, &str)> = vec![
        (
            "a schema version this release does not read",
            policy(r#"schemaVersion: 3, version: "v", $S"#),
            "unsupported_version",
            "policy.schemaVersion",
        ),
        (
            "missing schema version",
            policy(r#"version: "v", $S"#),
            "policy_invalid",
            "policy.schemaVersion",
        ),
        (
            "schema version as a string",
            policy(r#"schemaVersion: "2", version: "v", $S"#),
            "policy_invalid",
            "policy.schemaVersion",
        ),
        (
            "missing version",
            policy(r#"schemaVersion: 2, $S"#),
            "policy_invalid",
            "policy.version",
        ),
        (
            "blank version",
            policy(r#"schemaVersion: 2, version: "  ", $S"#),
            "policy_invalid",
            "policy.version",
        ),
        (
            "numeric version",
            policy(r#"schemaVersion: 2, version: 7, $S"#),
            "policy_invalid",
            "policy.version",
        ),
        (
            "unknown field",
            policy(r#"schemaVersion: 2, version: "v", $S, fallback: "deep""#),
            "policy_invalid",
            "policy.fallback",
        ),
        (
            "a catalog beside select",
            policy(r#"schemaVersion: 2, version: "v", catalog: [], $S"#),
            "policy_invalid",
            "policy.catalog",
        ),
        (
            "a routes table beside select",
            policy(r#"schemaVersion: 2, version: "v", routes: { impl: "deep" }, $S"#),
            "policy_invalid",
            "policy.routes",
        ),
        (
            "no select",
            policy(r#"schemaVersion: 2, version: "v""#),
            "policy_invalid",
            "policy.select",
        ),
        (
            "a routes table in place of select",
            policy(r#"schemaVersion: 2, version: "v", routes: { impl: "deep" }"#),
            "policy_invalid",
            "policy.routes",
        ),
        (
            "select that is a string",
            policy(r#"schemaVersion: 2, version: "v", select: "deep""#),
            "policy_invalid",
            "policy.select",
        ),
        (
            "select that is a selected result",
            policy(
                r#"schemaVersion: 2, version: "v", select: { status: "selected", program: "fake-harness", args: [] }"#,
            ),
            "policy_invalid",
            "policy.select",
        ),
        (
            "select that is null",
            policy(r#"schemaVersion: 2, version: "v", select: null"#),
            "policy_invalid",
            "policy.select",
        ),
        (
            "loadContext that is not a function",
            policy(r#"schemaVersion: 2, version: "v", $S, loadContext: {}"#),
            "policy_invalid",
            "policy.loadContext",
        ),
        (
            "no policy export",
            "export const other = 1;\n".to_owned(),
            "policy_invalid",
            "policy",
        ),
        (
            "policy is not an object",
            "export const policy = 7;\n".to_owned(),
            "policy_invalid",
            "policy",
        ),
        (
            "policy is an array",
            "export const policy = [];\n".to_owned(),
            "policy_invalid",
            "policy",
        ),
        (
            "class instance",
            "class P { schemaVersion = 2; version = \"v\"; select() {} }\nexport const policy = new P();\n"
                .to_owned(),
            "policy_invalid",
            "policy",
        ),
    ];
    for (name, source, code, location) in cases {
        let sandbox = Sandbox::new();
        let entry = sandbox.personal_policy(&source);
        let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], code, "{name}: {refusal}");
        assert_eq!(error["stage"], "validation", "{name}: {refusal}");
        assert_eq!(error["location"], location, "{name}: {refusal}");
        assert_eq!(error["source"], text(&entry), "{name}: {refusal}");
    }
}

#[test]
fn the_valid_twin_of_the_invalid_shapes_selects() {
    // The positive control for the table above: the same template, correct,
    // reaches a selection.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(r#"schemaVersion: 2, version: "v", $S"#));
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["selection"]["reason"], "the one command");
    assert_eq!(report["policy"]["version"], "v");
}

#[test]
fn a_version_1_policy_refuses_with_the_rewrite_remedy_and_launches_nothing() {
    // The policy harness-dispatch 21.13.0 evaluated to record the
    // catalog-contract fixture, as that release read it.
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(include_str!("fixtures/catalog-contract/policy.ts"));

    for command in ["inspect", "run"] {
        let mut invocation = sandbox.command();
        invocation.args([command, "--kind", "build", "--prompt", "p", "--json"]);
        let refusal = support::run(&mut invocation).refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], "unsupported_version", "{command}: {refusal}");
        assert_eq!(error["stage"], "validation", "{command}: {refusal}");
        assert_eq!(error["location"], "policy.schemaVersion");
        assert_eq!(error["source"], text(&entry));
        assert!(
            error["message"]
                .as_str()
                .unwrap()
                .contains("schemaVersion 1 is the catalog contract"),
            "{refusal}"
        );
        let remedy = error["remedy"].as_str().unwrap();
        for part in [
            format!("rewrite {} to schemaVersion 2", text(&entry)),
            "drop `catalog` and `routes`".to_owned(),
            r#"{ status: "selected", program, args, provider, model, effort, reason }"#.to_owned(),
            "nothing converts a version-1 policy".to_owned(),
        ] {
            assert!(remedy.contains(&part), "missing {part:?} in {remedy}");
        }
    }
    assert!(
        !sandbox.harness_ran(),
        "a version-1 policy launched a harness"
    );
    assert!(
        !sandbox.home.join(".local").exists(),
        "a version-1 policy created the record directory"
    );

    // Declaring version 2 converts nothing: its catalog is an unknown field.
    sandbox.personal_policy(
        &include_str!("fixtures/catalog-contract/policy.ts")
            .replace("schemaVersion: 1", "schemaVersion: 2"),
    );
    let refusal = sandbox.inspect(&["--kind", "build", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_invalid", "{refusal}");
    assert_eq!(refusal["error"]["location"], "policy.catalog");
}

#[test]
fn a_function_where_a_string_belongs_is_named_as_a_function() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(r#"schemaVersion: 2, version: () => "v", $S"#));
    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["location"], "policy.version");
    let message = refusal["error"]["message"].as_str().unwrap();
    assert!(message.contains("found a function"), "{message}");
}

#[test]
fn a_missing_relative_import_refuses_and_names_the_entry() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(
        "import { policy as shared } from \"./missing-shared.ts\";\nexport const policy = shared;\n",
    );

    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);

    let error = &refusal["error"];
    assert_eq!(error["code"], "policy_import_failed");
    assert_eq!(error["stage"], "load");
    assert_eq!(error["source"], text(&entry));
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("missing-shared"),
        "{refusal}"
    );
}

#[test]
fn a_missing_package_is_never_installed_automatically() {
    // `is-odd` exists on npm, and plain `bun` fetches it when no node_modules
    // is present. The compiled worker must refuse instead, leaving no cache.
    // The `package.json` beside the entry names it as a dependency, which is
    // the record an installation would start from, and the worker reads it.
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(
        "import isOdd from \"is-odd\";\nexport const policy = { odd: isOdd(1) };\n",
    );
    support::write(
        &entry.with_file_name("package.json"),
        r#"{ "name": "owner-policies", "type": "module", "dependencies": { "is-odd": "3.0.1" } }"#,
    );

    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);

    assert_eq!(refusal["error"]["code"], "policy_import_failed");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("is-odd"),
        "{refusal}"
    );
    assert!(
        !sandbox.home.join(".bun").exists(),
        "a package cache appeared under HOME"
    );
}

#[test]
fn an_entry_that_throws_while_loading_refuses() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy("throw new Error(\"policy exploded at import\");\n");

    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);

    assert_eq!(refusal["error"]["code"], "policy_import_failed");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("policy exploded at import"),
        "{refusal}"
    );
}

const NOISY: &str = r#"console.log("chatter on stdout");
process.stdout.write('{"schemaVersion": 99, "selection": "forged"}\n');
console.error("chatter on stderr");
"#;

#[test]
fn policy_output_is_captured_and_never_interleaved_with_the_report() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&format!("{NOISY}{ROUTED}"));

    let json = sandbox.inspect(&["--kind", "impl", "--json"]);
    let report = json.report();
    assert_eq!(report["selection"]["reason"], "impl runs the deep harness");
    assert!(report["diagnostics"]["stdout"]
        .as_str()
        .unwrap()
        .contains("chatter on stdout"));
    assert!(report["diagnostics"]["stdout"]
        .as_str()
        .unwrap()
        .contains("forged"));
    assert!(report["diagnostics"]["stderr"]
        .as_str()
        .unwrap()
        .contains("chatter on stderr"));
    assert_eq!(json.stderr, "");

    let human = sandbox.inspect(&["--kind", "impl"]);
    assert_eq!(human.code, Some(0));
    assert!(
        !human.stdout.contains("chatter") && !human.stdout.contains("forged"),
        "{}",
        human.stdout
    );
    assert!(
        human.stderr.contains("policy stdout: chatter on stdout"),
        "{}",
        human.stderr
    );
    assert!(
        human.stderr.contains("policy stderr: chatter on stderr"),
        "{}",
        human.stderr
    );
}

#[test]
fn a_refusal_carries_the_policy_output_separately() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&format!("{NOISY}{ROUTED}"));

    let refusal = sandbox.inspect(&["--kind", "design", "--json"]).refusal(3);

    assert_eq!(refusal["error"]["code"], "policy_refused");
    assert!(refusal["diagnostics"]["stdout"]
        .as_str()
        .unwrap()
        .contains("forged"));
}

#[test]
fn malformed_command_lines_exit_2_and_honour_json() {
    let sandbox = Sandbox::new();
    let refusal = sandbox.inspect(&["--json"]).refusal(2);
    assert_eq!(refusal["error"]["code"], "malformed_input");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("--kind"),
        "{refusal}"
    );

    let refusal = sandbox.inspect(&["--kind", "", "--json"]).refusal(2);
    assert_eq!(refusal["error"]["input"], "--kind");

    let run = sandbox.inspect(&["--kind", "impl", "--no-such-flag"]);
    assert_eq!(run.code, Some(2));
    assert!(run.stderr.contains("--no-such-flag"), "{}", run.stderr);

    // No input names a command for the caller: only the policy returns one.
    let refusal = sandbox
        .inspect(&["--kind", "impl", "--choice", "deep", "--json"])
        .refusal(2);
    assert_eq!(refusal["error"]["code"], "malformed_input");
    assert_eq!(refusal["error"]["input"], "--choice");
}

#[test]
fn help_lists_every_selection_input() {
    let sandbox = Sandbox::new();
    let run = sandbox.inspect(&["--help"]);
    assert_eq!(run.code, Some(0));
    for delivered in [
        "--kind",
        "--config",
        "--prompt",
        "--prompt-file",
        "--task-file",
        "--task-id",
        "--param <NAME=VALUE>",
        "--timeout-ms",
        "--state-dir",
        "--context",
        "--context-bytes",
        "--policy-env",
    ] {
        assert!(
            run.stdout.contains(delivered),
            "help omits {delivered}:\n{}",
            run.stdout
        );
    }
    assert!(
        !run.stdout.contains("--choice"),
        "help offers an input that names a command:\n{}",
        run.stdout
    );
    assert!(
        run.stdout.contains("PROMPT_NOT_SUPPLIED"),
        "help does not name the prompt marker:\n{}",
        run.stdout
    );
    assert!(
        run.stdout.contains("Do not grant GROVE_LAUNCH_DIR"),
        "help does not warn against granting GROVE_LAUNCH_DIR:\n{}",
        run.stdout
    );
    let mut version = sandbox.command();
    version.arg("--version");
    let version = support::run(&mut version);
    assert_eq!(
        version.stdout.trim(),
        format!("harness-dispatch {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn no_value_on_stdout_is_ever_partial() {
    // Every refusal above asserts an empty stdout; this pins the success side:
    // stdout is exactly one JSON document followed by one newline.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let run = sandbox.inspect(&["--kind", "impl", "--json"]);
    assert!(
        run.stdout.ends_with("}\n") && run.stdout.matches('\n').count() == 1,
        "{}",
        run.stdout
    );
    let _: Value = run.report();
}

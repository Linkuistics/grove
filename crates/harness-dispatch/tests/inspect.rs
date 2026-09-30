//! `inspect` of a static routes policy through the command seam: selection,
//! argv expansion, both report forms, policy validation and load failures, and
//! the explicit refusal of forms later increments own.

mod support;

use serde_json::Value;
use support::{text, Sandbox, ROUTED};

#[test]
fn inspection_reports_the_routed_candidate_and_its_evidence_in_json() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(ROUTED);

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();

    assert_eq!(report["schemaVersion"], 1);
    assert_eq!(report["evidence"], "proposal");
    assert_eq!(report["kind"], "impl");
    assert_eq!(report["policy"]["path"], text(&entry));
    assert_eq!(report["policy"]["authority"], "personal");
    assert_eq!(report["policy"]["version"], "seam-1");
    let selection = &report["selection"];
    assert_eq!(selection["form"], "routes");
    assert_eq!(selection["selectedBy"], "route");
    assert_eq!(selection["explicitChoice"], Value::Null);
    assert_eq!(selection["candidateId"], "deep");
    assert_eq!(selection["provider"], "origin-a");
    assert_eq!(selection["model"], "model-large");
    assert_eq!(selection["effort"], "high");
    assert_eq!(
        selection["reason"],
        r#"routes["impl"] names candidate "deep""#
    );
    assert!(report["timing"]["selectionMs"].is_u64(), "{report}");
    assert_eq!(
        report["worker"]["packageVersion"],
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(report["worker"]["bunVersion"], "1.4.2");
    assert_eq!(report["taskFile"], Value::Null);
    assert_eq!(report["taskId"], Value::Null);
    assert_eq!(report["prompt"], serde_json::json!({ "supplied": false }));
    let executable = &report["executable"];
    assert_eq!(executable["program"], "fake-harness");
    assert_eq!(executable["resolvedBy"], "PATH");
    assert_eq!(executable["pathEntry"], text(&sandbox.bin));
    assert_eq!(executable["path"], text(&sandbox.bin.join("fake-harness")));
    // Without a prompt the prompt's argument is a marked placeholder, never a
    // string that could pass for one.
    assert_eq!(
        report["argv"],
        serde_json::json!(["fake-harness", { "placeholder": "prompt" }])
    );
}

#[test]
fn inspection_shows_the_unchanged_prompt_and_every_expanded_slot() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(
        r#"export const policy = { schemaVersion: 1, version: "v", catalog: [
  { id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness",
    args: ["--kind", { slot: "kind" }, { slot: "taskFile" }, { slot: "taskId" }, { slot: "model" }, { slot: "effort" }, "{prompt}", { slot: "prompt" }] },
], routes: { impl: "deep" } };
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
            "--json",
        ])
        .report();

    let task_file = text(&sandbox.cwd.join("leaf.md"));
    assert_eq!(report["taskFile"], task_file.as_str());
    assert_eq!(report["taskId"], "T 1");
    assert_eq!(
        report["prompt"],
        serde_json::json!({ "supplied": true, "from": "--prompt", "bytes": prompt.len() })
    );
    assert_eq!(
        report["argv"],
        serde_json::json!([
            "fake-harness",
            "--kind",
            "impl",
            task_file,
            "T 1",
            "model-large",
            "high",
            "{prompt}",
            prompt,
        ])
    );
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
        "candidate  deep".to_owned(),
        "provider   origin-a".to_owned(),
        "model      model-large".to_owned(),
        "effort     high".to_owned(),
        r#"reason     routes["impl"] names candidate "deep""#.to_owned(),
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
        run.stdout.contains("prompt     not supplied"),
        "{}",
        run.stdout
    );
    assert!(
        run.stdout.contains("  argv       [0] \"fake-harness\"\n")
            && run.stdout.contains(
                "             [1] <prompt placeholder: no --prompt or --prompt-file given>\n"
            ),
        "{}",
        run.stdout
    );

    // A supplied prompt appears quoted and escaped, whole.
    let run = sandbox.inspect(&["--kind", "impl", "--prompt", "two\nlines \"quoted\""]);
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
}

#[test]
fn the_type_checked_fixture_selects_through_the_embedded_sdk() {
    // It imports `definePolicy` from `harness-dispatch/sdk`, and there is no
    // node_modules anywhere near the sandbox: only the worker's registered
    // virtual module can satisfy the import.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(include_str!("../worker/typecheck/routes-policy.ts"));

    let report = sandbox.inspect(&["--kind", "design", "--json"]).report();

    assert_eq!(report["selection"]["candidateId"], "deep");
    assert_eq!(report["policy"]["version"], "typecheck-fixture-1");
}

#[test]
fn an_unrouted_kind_refuses_as_an_incomplete_mapping_without_a_default() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(ROUTED);

    let refusal = sandbox.inspect(&["--kind", "design", "--json"]).refusal(3);

    let error = &refusal["error"];
    assert_eq!(error["code"], "incomplete_mapping");
    assert_eq!(error["stage"], "selection");
    assert_eq!(error["input"], "--kind design");
    assert_eq!(error["source"], text(&entry));
    assert!(error["remedy"]
        .as_str()
        .unwrap()
        .contains("never substitutes a default"));

    let run = sandbox.inspect(&["--kind", "design"]);
    assert_eq!(run.code, Some(3));
    assert_eq!(run.stdout, "");
    assert!(
        run.stderr
            .contains("refused (incomplete_mapping, stage selection)"),
        "{}",
        run.stderr
    );
    assert!(run.stderr.contains("  remedy: "), "{}", run.stderr);
}

const CANDIDATE: &str = r#"{ id: "deep", provider: "origin-a", model: "m", effort: "high", program: "fake-harness", args: [{ slot: "prompt" }] }"#;

fn policy(fields: &str) -> String {
    format!(
        "export const policy = {{ {} }};\n",
        fields.replace("$C", CANDIDATE)
    )
}

#[test]
fn every_invalid_policy_shape_refuses_with_its_location() {
    let with = |candidate: &str| {
        format!(
            r#"export const policy = {{ schemaVersion: 1, version: "v", catalog: [{candidate}], routes: {{ impl: "deep" }} }};"#
        )
    };
    let cases: Vec<(&str, String, &str, &str)> = vec![
        (
            "unknown schema version",
            policy(r#"schemaVersion: 2, version: "v", catalog: [$C], routes: {}"#),
            "unsupported_version",
            "policy.schemaVersion",
        ),
        (
            "missing schema version",
            policy(r#"version: "v", catalog: [$C], routes: { impl: "deep" }"#),
            "policy_invalid",
            "policy.schemaVersion",
        ),
        (
            "missing version",
            policy(r#"schemaVersion: 1, catalog: [$C], routes: { impl: "deep" }"#),
            "policy_invalid",
            "policy.version",
        ),
        (
            "blank version",
            policy(r#"schemaVersion: 1, version: "  ", catalog: [$C], routes: { impl: "deep" }"#),
            "policy_invalid",
            "policy.version",
        ),
        (
            "unknown field",
            policy(
                r#"schemaVersion: 1, version: "v", catalog: [$C], routes: { impl: "deep" }, fallback: "deep""#,
            ),
            "policy_invalid",
            "policy.fallback",
        ),
        (
            "catalog not an array",
            policy(
                r#"schemaVersion: 1, version: "v", catalog: { deep: $C }, routes: { impl: "deep" }"#,
            ),
            "policy_invalid",
            "policy.catalog",
        ),
        (
            "duplicate candidate ID",
            policy(
                r#"schemaVersion: 1, version: "v", catalog: [$C, $C], routes: { impl: "deep" }"#,
            ),
            "policy_invalid",
            "policy.catalog[1].id",
        ),
        (
            "empty provider",
            with(
                r#"{ id: "deep", provider: "", model: "m", effort: "e", program: "p", args: [] }"#,
            ),
            "policy_invalid",
            "policy.catalog[0].provider",
        ),
        (
            "model is a function",
            with(
                r#"{ id: "deep", provider: "o", model: () => "m", effort: "e", program: "p", args: [] }"#,
            ),
            "policy_invalid",
            "policy.catalog[0].model",
        ),
        (
            "missing effort",
            with(r#"{ id: "deep", provider: "o", model: "m", program: "p", args: [] }"#),
            "policy_invalid",
            "policy.catalog[0].effort",
        ),
        (
            "unknown candidate field",
            with(
                r#"{ id: "deep", provider: "o", model: "m", effort: "e", program: "p", args: [], weight: 1 }"#,
            ),
            "policy_invalid",
            "policy.catalog[0].weight",
        ),
        (
            "unknown slot",
            with(
                r#"{ id: "deep", provider: "o", model: "m", effort: "e", program: "p", args: [{ slot: "cwd" }] }"#,
            ),
            "policy_invalid",
            "policy.catalog[0].args[0].slot",
        ),
        (
            "no prompt slot",
            with(
                r#"{ id: "deep", provider: "o", model: "m", effort: "e", program: "p", args: ["x"] }"#,
            ),
            "policy_invalid",
            "policy.catalog[0].args",
        ),
        (
            "two prompt slots",
            with(
                r#"{ id: "deep", provider: "o", model: "m", effort: "e", program: "p", args: [{ slot: "prompt" }, { slot: "prompt" }] }"#,
            ),
            "policy_invalid",
            "policy.catalog[0].args",
        ),
        (
            "a misspelt runId slot",
            with(
                r#"{ id: "deep", provider: "o", model: "m", effort: "e", program: "p", args: [{ slot: "prompt" }, { slot: "runID" }] }"#,
            ),
            "policy_invalid",
            "policy.catalog[0].args[1].slot",
        ),
        (
            "numeric argument",
            with(
                r#"{ id: "deep", provider: "o", model: "m", effort: "e", program: "p", args: [7] }"#,
            ),
            "policy_invalid",
            "policy.catalog[0].args[0]",
        ),
        (
            "route to an unknown candidate",
            policy(r#"schemaVersion: 1, version: "v", catalog: [$C], routes: { impl: "missing" }"#),
            "policy_invalid",
            r#"policy.routes["impl"]"#,
        ),
        (
            "routes as a Map",
            policy(
                r#"schemaVersion: 1, version: "v", catalog: [$C], routes: new Map([["impl", "deep"]])"#,
            ),
            "policy_invalid",
            "policy.routes",
        ),
        (
            "both forms",
            policy(
                r#"schemaVersion: 1, version: "v", catalog: [$C], routes: { impl: "deep" }, select: () => ({})"#,
            ),
            "policy_invalid",
            "policy",
        ),
        (
            "neither form",
            policy(r#"schemaVersion: 1, version: "v", catalog: [$C]"#),
            "policy_invalid",
            "policy",
        ),
        (
            "select is later",
            policy(r#"schemaVersion: 1, version: "v", catalog: [$C], select: () => ({})"#),
            "unsupported_form",
            "policy.select",
        ),
        (
            "loadContext is later",
            policy(
                r#"schemaVersion: 1, version: "v", catalog: [$C], routes: { impl: "deep" }, loadContext: () => ({})"#,
            ),
            "unsupported_form",
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
            "class instance",
            "class P { schemaVersion = 1; select() {} }\nexport const policy = new P();\n"
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
        assert_eq!(error["location"], location, "{name}: {refusal}");
        assert_eq!(error["source"], text(&entry), "{name}: {refusal}");
    }
}

#[test]
fn the_valid_twin_of_the_invalid_shapes_selects() {
    // The positive control for the table above: the same template and
    // candidate, correct, reaches a selection.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(
        r#"schemaVersion: 1, version: "v", catalog: [$C], routes: { impl: "deep" }"#,
    ));
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["selection"]["candidateId"], "deep");
}

#[test]
fn a_function_where_a_string_belongs_is_named_as_a_function() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(
        r#"export const policy = { schemaVersion: 1, version: "v", catalog: [{ id: "deep", provider: "o", model: () => "m", effort: "e", program: "p", args: [] }], routes: { impl: "deep" } };"#,
    );
    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    let message = refusal["error"]["message"].as_str().unwrap();
    assert!(message.contains("found a function"), "{message}");
}

#[test]
fn a_missing_relative_import_refuses_and_names_the_entry() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(
        "import { routes } from \"./missing-routes.ts\";\nexport const policy = routes;\n",
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
            .contains("missing-routes"),
        "{refusal}"
    );
}

#[test]
fn a_missing_package_is_never_installed_automatically() {
    // `is-odd` exists on npm, and plain `bun` fetches it when no node_modules
    // is present. The compiled worker must refuse instead, leaving no cache.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(
        "import isOdd from \"is-odd\";\nexport const policy = { odd: isOdd(1) };\n",
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
    assert_eq!(report["selection"]["candidateId"], "deep");
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

    assert_eq!(refusal["error"]["code"], "incomplete_mapping");
    assert!(refusal["diagnostics"]["stdout"]
        .as_str()
        .unwrap()
        .contains("forged"));
}

#[test]
fn forms_later_increments_own_are_refused_by_name() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    for flag in ["--context", "--policy-env", "--context-bytes"] {
        let refusal = sandbox
            .inspect(&["--kind", "impl", flag, "value", "--json"])
            .refusal(2);
        assert_eq!(refusal["error"]["code"], "unsupported_input", "{flag}");
        assert_eq!(refusal["error"]["input"], flag, "{flag}");
    }
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--context",
            "c.json",
            "--json",
        ])
        .refusal(2);
    assert_eq!(refusal["error"]["input"], "--context");
    assert!(!sandbox.harness_ran());
    let mut invocation = sandbox.command();
    invocation.args(["record", "observe", "--run", "r", "--json"]);
    let refusal = support::run(&mut invocation).refusal(2);
    assert_eq!(refusal["error"]["code"], "unsupported_input");
    assert_eq!(refusal["error"]["input"], "record observe");
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
}

#[test]
fn help_lists_only_the_forms_this_release_delivers() {
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
        "--timeout-ms",
        "--state-dir",
        "--choice",
    ] {
        assert!(
            run.stdout.contains(delivered),
            "help omits {delivered}:\n{}",
            run.stdout
        );
    }
    for later in ["--context", "--policy-env"] {
        assert!(
            !run.stdout.contains(later),
            "help advertises {later}:\n{}",
            run.stdout
        );
    }
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

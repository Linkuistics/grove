//! Computed selection through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Policy and joint choice*):
//! a policy's `select` chooses a configured candidate, or refuses, from the
//! versioned request, synchronously or through a promise. Every other value it
//! produces refuses with a code of its own and launches nothing, and an
//! explicit choice is the policy's to accept or refuse, never to replace.

mod support;

use std::collections::BTreeSet;
use std::fs;
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use support::{text, Sandbox};

/// Two candidates, and `$SELECT`: the `select` member, written as a method or
/// property of the policy object, which may call `writeFileSync`.
const TEMPLATE: &str = r#"import { writeFileSync } from "node:fs";

export const policy = {
  schemaVersion: 1,
  version: "select-1",
  catalog: [
    { id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness", args: ["--effort", { slot: "effort" }, { slot: "prompt" }] },
    { id: "quick", provider: "origin-b", model: "model-small", effort: "low", program: "fake-harness", args: [{ slot: "prompt" }] },
  ],
  $SELECT
};
"#;

fn selecting(select: &str) -> String {
    TEMPLATE.replace("$SELECT", select)
}

/// A `select` that writes `marker` when it runs, then selects `quick`.
fn marking(marker: &std::path::Path) -> String {
    format!(
        "select() {{ writeFileSync({:?}, \"ran\"); return {{ status: \"selected\", candidateId: \"quick\", reason: \"marked\" }}; }},",
        text(marker)
    )
}

const QUICK: &str =
    r#"{ status: "selected", candidateId: "quick", reason: "a small change needs little effort" }"#;

#[test]
fn a_synchronous_or_asynchronous_select_chooses_a_configured_candidate() {
    for select in [
        format!("select(request) {{ return {QUICK}; }},"),
        format!("select: (request) => Promise.resolve({QUICK}),"),
        format!(
            "async select(request) {{ await new Promise((resolve) => setTimeout(resolve, 20)); return {QUICK}; }},"
        ),
    ] {
        let sandbox = Sandbox::new();
        sandbox.personal_policy(&selecting(&select));

        let report = sandbox
            .inspect(&["--kind", "impl", "--prompt", "the prompt", "--json"])
            .report();
        let selection = &report["selection"];
        assert_eq!(selection["form"], "select", "{select}: {report}");
        assert_eq!(selection["selectedBy"], "select");
        assert_eq!(selection["explicitChoice"], Value::Null);
        assert_eq!(selection["candidateId"], "quick");
        assert_eq!(selection["provider"], "origin-b");
        assert_eq!(selection["model"], "model-small");
        assert_eq!(selection["effort"], "low");
        assert_eq!(selection["reason"], "a small change needs little effort");
        assert_eq!(report["argv"], json!(["fake-harness", "the prompt"]));
        assert_eq!(report["policy"]["version"], "select-1");

        let human = sandbox.inspect(&["--kind", "impl"]);
        assert_eq!(human.code, Some(0), "{}", human.stderr);
        for row in [
            "choice     none; the policy's select chooses",
            "candidate  quick",
            "selected   by the policy's select (computed)",
            "reason     a small change needs little effort",
        ] {
            assert!(human.stdout.contains(row), "missing {row:?}:\n{}", human.stdout);
        }
    }
}

#[test]
fn run_launches_the_computed_choice_and_records_how_it_was_made() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&selecting(&format!(
        "async select() {{ return {QUICK}; }},"
    )));

    let run = sandbox.run(&["--kind", "impl", "--prompt", "the prompt", "--json"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["the prompt"]);
    let notice: Value = serde_json::from_str(&run.stderr).unwrap();
    assert_eq!(notice["handoff"]["selectedBy"], "select");
    assert_eq!(notice["handoff"]["candidateId"], "quick");
    assert_eq!(
        notice["handoff"]["reason"],
        "a small change needs little effort"
    );

    let run_id = sandbox.harness_run_id();
    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id, "--json"]);
    let export = support::run(&mut show).report();
    let selection = &export["launch"]["selection"];
    assert_eq!(selection["form"], "select");
    assert_eq!(selection["selectedBy"], "select");
    assert_eq!(selection["explicitChoice"], Value::Null);
    assert_eq!(selection["reason"], "a small change needs little effort");
    assert_eq!(export["launch"]["candidate"]["id"], "quick");

    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id]);
    let human = support::run(&mut show);
    assert!(
        human.stdout.contains("  selected   select\n"),
        "{}",
        human.stdout
    );

    // Text mode names no explicit choice for a computed selection.
    fs::remove_dir_all(&sandbox.record).unwrap();
    let run = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stderr
            .starts_with("harness-dispatch: running candidate \"quick\""),
        "{}",
        run.stderr
    );
}

#[test]
fn select_is_called_as_a_method_with_the_versioned_request_alone() {
    // The policy records exactly what it was called with: the request, how
    // many arguments there were, and whether `this` was the policy.
    let sandbox = Sandbox::new();
    let seen = sandbox.root.join("seen.json");
    sandbox.personal_policy(&selecting(&format!(
        "select(request) {{
    writeFileSync({:?}, JSON.stringify({{ request, arity: arguments.length, version: this.version }}));
    return {QUICK};
  }},",
        text(&seen)
    )));
    sandbox.file("mandate.md", "file-prompt-token\n");

    let report = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--task-file",
            "tasks/t.md",
            "--task-id",
            "T-7",
            "--timeout-ms",
            "20000",
            "--prompt-file",
            "mandate.md",
            "--json",
        ])
        .report();
    assert_eq!(report["selection"]["candidateId"], "quick");
    let raw = fs::read_to_string(&seen).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&raw).unwrap(),
        json!({
            "request": {
                "schemaVersion": 1,
                "kind": "impl",
                "cwd": text(&sandbox.cwd),
                "taskFile": text(&sandbox.cwd.join("tasks/t.md")),
                "taskId": "T-7",
                "limits": { "selectionMs": 20_000 },
            },
            "arity": 1,
            "version": "select-1",
        })
    );
    assert!(
        !raw.contains("prompt-token"),
        "the prompt reached select: {raw}"
    );

    // Inputs the caller did not supply are absent, not empty; the bound is the
    // default one.
    sandbox.inspect(&["--kind", "review", "--json"]).report();
    let seen: Value = serde_json::from_str(&fs::read_to_string(&seen).unwrap()).unwrap();
    assert_eq!(
        seen["request"],
        json!({
            "schemaVersion": 1,
            "kind": "review",
            "cwd": text(&sandbox.cwd),
            "limits": { "selectionMs": 30_000 },
        })
    );
}

#[test]
fn a_refusal_from_select_is_reported_with_its_code_message_and_remedy() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(&selecting(
        r#"async select(request) {
    return { status: "refused", code: "no_reviewer", message: `no candidate reviews kind ${request.kind}`, remedy: "declare the creator's provider" };
  },"#,
    ));

    for command in ["inspect", "run"] {
        let mut invocation = sandbox.command();
        invocation.args([command, "--kind", "review", "--prompt", "p", "--json"]);
        let refusal = support::run(&mut invocation).refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], "policy_refused", "{command}: {refusal}");
        assert_eq!(error["policyCode"], "no_reviewer");
        assert_eq!(error["stage"], "selection");
        assert_eq!(error["input"], "--kind review");
        assert_eq!(error["source"], text(&entry));
        assert!(
            error["message"]
                .as_str()
                .unwrap()
                .ends_with("refused the selection: no candidate reviews kind review"),
            "{refusal}"
        );
        assert_eq!(error["remedy"], "declare the creator's provider");
    }

    let text_mode = sandbox.run(&["--kind", "review", "--prompt", "p"]);
    assert_eq!(text_mode.code, Some(3));
    for line in [
        "refused (policy_refused, stage selection)",
        "  policy code: no_reviewer\n",
        "  remedy: declare the creator's provider\n",
        "  inspect: (cd ",
    ] {
        assert!(
            text_mode.stderr.contains(line),
            "missing {line:?}:\n{}",
            text_mode.stderr
        );
    }
    assert!(
        !sandbox.harness_ran(),
        "a refused selection launched a harness"
    );
    assert!(
        !sandbox.home.join(".local").exists(),
        "a refused selection created the record directory"
    );
}

#[test]
fn each_way_select_can_fail_refuses_with_its_own_code_and_launches_nothing() {
    let cases: [(&str, &str, &str, &str); 17] = [
        (
            r#"select() { throw new TypeError("no table"); },"#,
            "selection_threw",
            "policy.select",
            "TypeError: no table",
        ),
        (
            r#"async select() { await null; throw new RangeError("late"); },"#,
            "selection_threw",
            "policy.select",
            "RangeError: late",
        ),
        (
            r#"select: () => Promise.reject("a plain reason"),"#,
            "selection_threw",
            "policy.select",
            "a plain reason",
        ),
        (
            "select: () => new Promise(() => {}),",
            "selection_unsettled",
            "policy.select",
            "never settled",
        ),
        (
            "async select() { await new Promise(() => {}); return { status: \"selected\", candidateId: \"quick\", reason: \"r\" }; },",
            "selection_unsettled",
            "policy.select",
            "never settled",
        ),
        ("select() {},", "selection_abstained", "result", "no result"),
        ("select: () => null,", "selection_abstained", "result", "no result"),
        ("async select() {},", "selection_abstained", "result", "no result"),
        (r#"select: () => "quick","#, "selection_malformed", "result", "found a string"),
        (
            r#"select: () => ({ status: "selected", candidateId: "quick", reason: "r", args: ["--yolo"] }),"#,
            "selection_malformed",
            "result.args",
            "cannot supply a program or arguments",
        ),
        (
            r#"select: () => ({ status: "selected", candidateId: "quick", reason: "r", program: "/bin/sh" }),"#,
            "selection_malformed",
            "result.program",
            "cannot supply a program or arguments",
        ),
        (
            r#"select: () => ({ status: "ok", candidateId: "quick", reason: "r" }),"#,
            "selection_malformed",
            "result.status",
            "found \"ok\"",
        ),
        (
            r#"select: () => ({ status: "selected", candidateId: "quick", reason: "  " }),"#,
            "selection_malformed",
            "result.reason",
            "blank",
        ),
        (
            r#"select: () => ({ status: "refused", code: "c", message: "m" }),"#,
            "selection_malformed",
            "result.remedy",
            "missing",
        ),
        (
            r#"select() { const result = { status: "selected", candidateId: "quick", reason: "r" }; result.self = result; return result; },"#,
            "selection_malformed",
            "result",
            "cannot be serialized",
        ),
        (
            r#"select: () => ({ status: "selected", candidateId: "careful", reason: "r" }),"#,
            "unknown_candidate",
            "result.candidateId",
            "\"careful\", which is not in its catalog",
        ),
        (
            r#"select: () => ({ status: "selected", candidateId: "Quick", reason: "r" }),"#,
            "unknown_candidate",
            "result.candidateId",
            "\"Quick\", which is not in its catalog",
        ),
    ];
    let mut codes = BTreeSet::new();
    for (select, code, location, found) in cases {
        let sandbox = Sandbox::new();
        let entry = sandbox.personal_policy(&selecting(select));
        for command in ["inspect", "run"] {
            let started = Instant::now();
            let mut invocation = sandbox.command();
            invocation.args([command, "--kind", "impl", "--prompt", "p", "--json"]);
            let refusal = support::run(&mut invocation).refusal(3);
            let error = &refusal["error"];
            let context = format!("{command} {select}: {refusal}");
            assert_eq!(error["code"], code, "{context}");
            assert_eq!(error["stage"], "selection", "{context}");
            assert_eq!(error["location"], location, "{context}");
            assert_eq!(error["source"], text(&entry), "{context}");
            assert!(
                error["message"].as_str().unwrap().contains(found),
                "{context}"
            );
            // An abandoned promise is reported when the worker's event loop
            // drains, not when the selection bound runs out.
            assert!(
                started.elapsed() < Duration::from_secs(15),
                "{context}: took {:?}",
                started.elapsed()
            );
        }
        assert!(!sandbox.harness_ran(), "{select} launched a harness");
        assert!(
            !sandbox.home.join(".local").exists(),
            "{select} created the record directory"
        );
        codes.insert(code);
    }
    assert_eq!(
        codes.len(),
        5,
        "exceptions, unsettled promises, abstention, malformed results and unknown IDs \
         each have their own code: {codes:?}"
    );
}

/// `--choice` under `select`: the task identity tells this policy what to do
/// with the choice it is given, and it records that it ran.
fn policing(marker: &std::path::Path) -> String {
    selecting(&format!(
        r#"select(request) {{
    writeFileSync({:?}, JSON.stringify(request.explicitChoice ?? null));
    const choice = request.explicitChoice;
    if (choice === undefined) return {{ status: "selected", candidateId: "quick", reason: "no choice was made" }};
    if (request.taskId === "accept") return {{ status: "selected", candidateId: choice, reason: `accepted ${{choice}}` }};
    if (request.taskId === "refuse") return {{ status: "refused", code: "choice_forbidden", message: `${{choice}} is not allowed here`, remedy: "omit --choice" }};
    return {{ status: "selected", candidateId: choice === "deep" ? "quick" : "deep", reason: "a safer fallback" }};
  }},"#,
        text(marker)
    ))
}

#[test]
fn select_sees_an_explicit_choice_and_accepting_it_selects_it() {
    let sandbox = Sandbox::new();
    let marker = sandbox.root.join("select-saw");
    sandbox.personal_policy(&policing(&marker));

    let report = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--task-id",
            "accept",
            "--choice",
            "deep",
            "--json",
        ])
        .report();
    let selection = &report["selection"];
    assert_eq!(selection["form"], "select");
    assert_eq!(selection["selectedBy"], "select");
    assert_eq!(selection["explicitChoice"], "deep");
    assert_eq!(selection["candidateId"], "deep");
    assert_eq!(selection["reason"], "accepted deep");
    assert_eq!(fs::read_to_string(&marker).unwrap(), "\"deep\"");

    let human = sandbox.inspect(&["--kind", "impl", "--task-id", "accept", "--choice", "deep"]);
    assert!(
        human
            .stdout
            .contains("choice     --choice deep, which the policy's select accepted"),
        "{}",
        human.stdout
    );

    let run = sandbox.run(&[
        "--kind",
        "impl",
        "--task-id",
        "accept",
        "--choice",
        "deep",
        "--prompt",
        "p",
    ]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["--effort", "high", "p"]);
    assert!(
        run.stderr
            .starts_with("harness-dispatch: running explicitly chosen candidate \"deep\""),
        "{}",
        run.stderr
    );
    let mut show = sandbox.command();
    show.args([
        "record",
        "show",
        "--run",
        &sandbox.harness_run_id(),
        "--json",
    ]);
    let export = support::run(&mut show).report();
    assert_eq!(export["launch"]["selection"]["selectedBy"], "select");
    assert_eq!(export["launch"]["selection"]["explicitChoice"], "deep");
    assert_eq!(export["launch"]["selection"]["reason"], "accepted deep");
}

#[test]
fn select_may_refuse_an_explicit_choice() {
    let sandbox = Sandbox::new();
    let marker = sandbox.root.join("select-saw");
    sandbox.personal_policy(&policing(&marker));

    for command in ["inspect", "run"] {
        let mut invocation = sandbox.command();
        invocation.args([
            command,
            "--kind",
            "impl",
            "--task-id",
            "refuse",
            "--choice",
            "quick",
            "--prompt",
            "p",
            "--json",
        ]);
        let refusal = support::run(&mut invocation).refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], "policy_refused", "{command}: {refusal}");
        assert_eq!(error["policyCode"], "choice_forbidden");
        assert_eq!(error["input"], "--choice quick");
        assert_eq!(error["remedy"], "omit --choice");
    }
    assert_eq!(fs::read_to_string(&marker).unwrap(), "\"quick\"");
    assert!(!sandbox.harness_ran());
}

#[test]
fn any_other_id_for_an_explicit_choice_is_a_mismatch_whatever_the_reason() {
    let sandbox = Sandbox::new();
    let marker = sandbox.root.join("select-saw");
    let entry = sandbox.personal_policy(&policing(&marker));

    // The policy answers each choice with the other configured candidate, and
    // calls it a fallback.
    for (choice, returned) in [("quick", "deep"), ("deep", "quick")] {
        for command in ["inspect", "run"] {
            let mut invocation = sandbox.command();
            invocation.args([
                command,
                "--kind",
                "impl",
                "--task-id",
                "fallback",
                "--choice",
                choice,
                "--prompt",
                "p",
                "--json",
            ]);
            let refusal = support::run(&mut invocation).refusal(3);
            let error = &refusal["error"];
            assert_eq!(
                error["code"], "explicit_choice_mismatch",
                "{command}: {refusal}"
            );
            assert_eq!(error["stage"], "selection");
            assert_eq!(error["input"], format!("--choice {choice}"));
            assert_eq!(error["source"], text(&entry));
            assert_eq!(error["location"], "result.candidateId");
            let message = error["message"].as_str().unwrap();
            assert!(
                message.contains(&format!("selected {returned:?} instead"))
                    && message.contains("a safer fallback"),
                "{message}"
            );
        }
    }
    assert!(!sandbox.harness_ran(), "a mismatch launched a harness");
    assert!(!sandbox.home.join(".local").exists());
}

#[test]
fn a_choice_the_catalog_lacks_refuses_before_select_runs() {
    let sandbox = Sandbox::new();
    let marker = sandbox.root.join("select-saw");
    let entry = sandbox.personal_policy(&policing(&marker));

    for command in ["inspect", "run"] {
        let mut invocation = sandbox.command();
        invocation.args([
            command,
            "--kind",
            "impl",
            "--task-id",
            "accept",
            "--choice",
            "careful",
            "--prompt",
            "p",
            "--json",
        ]);
        let refusal = support::run(&mut invocation).refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], "unknown_choice", "{command}: {refusal}");
        assert_eq!(error["input"], "--choice careful");
        assert_eq!(error["source"], text(&entry));
        let remedy = error["remedy"].as_str().unwrap();
        assert!(remedy.contains(r#"("deep", "quick")"#), "{remedy}");
    }
    assert!(
        !marker.exists(),
        "select ran for a choice the catalog lacks"
    );

    // The control: a configured choice does reach select.
    sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--task-id",
            "accept",
            "--choice",
            "quick",
            "--json",
        ])
        .report();
    assert_eq!(fs::read_to_string(&marker).unwrap(), "\"quick\"");
}

#[test]
fn inspection_distinguishes_a_route_an_explicit_choice_and_a_computed_selection() {
    let sandbox = Sandbox::new();
    let routes = sandbox.file(
        "routes.ts",
        &TEMPLATE.replace("$SELECT", r#"routes: { impl: "deep" },"#),
    );
    let computed = sandbox.file(
        "computed.ts",
        &selecting(&format!("select: () => ({QUICK}),")),
    );
    let (routes, computed) = (text(&routes), text(&computed));

    for (config, choice, form, selected_by, row) in [
        (
            &routes,
            None,
            "routes",
            "route",
            "by the routes table (static)",
        ),
        (
            &routes,
            Some("quick"),
            "routes",
            "explicit_choice",
            "by the explicit choice (static; the routes were not consulted)",
        ),
        (
            &computed,
            None,
            "select",
            "select",
            "by the policy's select (computed)",
        ),
        (
            &computed,
            Some("quick"),
            "select",
            "select",
            "by the policy's select (computed)",
        ),
    ] {
        let mut args = vec!["--kind", "impl", "--config", config.as_str()];
        if let Some(choice) = choice {
            args.extend(["--choice", choice]);
        }
        let human = sandbox.inspect(&args);
        args.push("--json");
        let report = sandbox.inspect(&args).report();
        let context = format!("{config} {choice:?}: {report}");
        assert_eq!(report["selection"]["form"], form, "{context}");
        assert_eq!(report["selection"]["selectedBy"], selected_by, "{context}");
        assert_eq!(
            report["selection"]["explicitChoice"],
            choice.map_or(Value::Null, Value::from),
            "{context}"
        );
        assert!(
            human.stdout.contains(&format!("  selected   {row}\n")),
            "{context}\n{}",
            human.stdout
        );
    }
}

#[test]
fn select_runs_only_for_a_policy_the_front_accepted() {
    // An invalid catalog, or both forms at once, refuses from the snapshot the
    // worker took at import: `select` is never called for it.
    let sandbox = Sandbox::new();
    let marker = sandbox.root.join("select-ran");
    let valid = selecting(&marking(&marker));
    for (name, source, location) in [
        (
            "a candidate without a provider",
            valid.replace(r#"provider: "origin-b", "#, ""),
            "policy.catalog[1].provider",
        ),
        (
            "both forms",
            valid.replace("select() {", "routes: { impl: \"deep\" },\n  select() {"),
            "policy",
        ),
    ] {
        sandbox.personal_policy(&source);
        let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
        assert_eq!(
            refusal["error"]["code"], "policy_invalid",
            "{name}: {refusal}"
        );
        assert_eq!(refusal["error"]["location"], location, "{name}");
        assert!(!marker.exists(), "{name}: select ran");
    }

    // The control: the same policy, valid, does call select.
    sandbox.personal_policy(&valid);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["selection"]["reason"], "marked");
    assert!(marker.exists(), "the valid policy's select never ran");
}

#[test]
fn what_select_prints_is_kept_apart_from_the_report() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&selecting(&format!(
        "async select() {{ console.log(\"weighing the candidates\"); console.error(\"no table entry\"); return {QUICK}; }},"
    )));

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["selection"]["candidateId"], "quick");
    assert_eq!(report["diagnostics"]["stdout"], "weighing the candidates\n");
    assert_eq!(report["diagnostics"]["stderr"], "no table entry\n");

    let run = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stderr
            .starts_with("policy stdout: weighing the candidates\npolicy stderr: no table entry\n"),
        "{}",
        run.stderr
    );
}

#[test]
fn an_import_left_unsettled_refuses_as_a_load_failure_at_once() {
    // A top-level await that nothing will ever settle is reported when the
    // worker's event loop drains, well inside the default 30-second bound.
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(&format!(
        "await new Promise(() => {{}});\n{}",
        selecting(&format!("select: () => ({QUICK}),"))
    ));
    let started = Instant::now();
    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    assert_eq!(
        refusal["error"]["code"], "policy_import_failed",
        "{refusal}"
    );
    assert_eq!(refusal["error"]["stage"], "load");
    assert_eq!(refusal["error"]["source"], text(&entry));
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("never settled"),
        "{refusal}"
    );
    assert!(
        started.elapsed() < Duration::from_secs(15),
        "took {:?}",
        started.elapsed()
    );

    // The control: a top-level await that settles loads and selects.
    sandbox.personal_policy(&format!(
        "await new Promise((resolve) => setTimeout(resolve, 20));\n{}",
        selecting(&format!("select: () => ({QUICK}),"))
    ));
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["selection"]["candidateId"], "quick");
}

#[test]
fn the_type_checked_select_fixture_evaluates_as_its_types_describe() {
    // The fixture `task dispatch:typecheck` checks against the shipped
    // declarations, with `request` typed by the policy's form alone.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(include_str!("../worker/typecheck/select-policy.ts"));

    for (kind, candidate) in [("design", "deep"), ("impl", "quick")] {
        let report = sandbox
            .inspect(&["--kind", kind, "--timeout-ms", "25000", "--json"])
            .report();
        assert_eq!(report["policy"]["version"], "typecheck-select-1");
        assert_eq!(report["selection"]["form"], "select");
        assert_eq!(report["selection"]["candidateId"], candidate, "{kind}");
        assert_eq!(
            report["selection"]["reason"],
            format!("kind {kind} within 25000 ms")
        );
    }
    let refusal = sandbox
        .inspect(&["--kind", "impl", "--choice", "deep", "--json"])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused");
    assert_eq!(refusal["error"]["policyCode"], "no_explicit_choices");
}

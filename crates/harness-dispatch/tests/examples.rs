//! The starter examples, through the command seam: each is imported from a
//! temporary personal policy by its embedded `harness-dispatch/examples/…`
//! specifier and selects through the compiled worker (`docs/specs/harness-selection-and-execution.md`, *Policy and joint
//! choice*, *Policy authority and runtime discovery*). No node_modules exists
//! near any sandbox, so only the worker's registered modules can satisfy these
//! imports.

mod support;

use std::fs;
use std::path::PathBuf;

use support::{executable, text, Sandbox};

/// Put a stand-in for each of an example's illustrative wrappers on PATH.
fn wrappers(sandbox: &Sandbox, names: &[&str]) {
    for name in names {
        executable(&sandbox.bin.join(name), "#!/bin/sh\nexit 0\n");
    }
}

/// Every kind Grove launches a session for, and the candidate the Grove
/// example routes it to.
const SESSION_KIND_ROUTES: [(&str, &str); 23] = [
    ("requirements", "lead-max"),
    ("review-requirements", "review-xhigh"),
    ("integrate-review-requirements", "lead-xhigh"),
    ("design", "lead-xhigh"),
    ("review-design", "review-xhigh"),
    ("integrate-review-design", "lead-xhigh"),
    ("planning", "lead-xhigh"),
    ("review-planning", "review-xhigh"),
    ("integrate-review-planning", "lead-xhigh"),
    ("prototype", "lead-medium"),
    ("review-prototype", "review-medium"),
    ("integrate-review-prototype", "lead-medium"),
    ("impl", "lead-high"),
    ("review-impl", "review-high"),
    ("integrate-review-impl", "lead-medium"),
    ("research-a", "lead-high"),
    ("research-b", "review-high"),
    ("combine-research", "lead-xhigh"),
    ("draft", "lead-high"),
    ("copy-edit", "review-medium"),
    ("art", "lead-medium"),
    ("proof", "review-high"),
    ("finish", "lead-medium"),
];

#[test]
fn the_grove_example_routes_every_grove_session_kind_exactly() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy("export { policy } from \"harness-dispatch/examples/grove-static\";\n");
    wrappers(&sandbox, &["my-codex-wrapper", "my-claude-wrapper"]);

    for (kind, candidate) in SESSION_KIND_ROUTES {
        let report = sandbox
            .inspect(&["--kind", kind, "--prompt", "the mandate", "--json"])
            .report();
        let selection = &report["selection"];
        assert_eq!(selection["candidateId"], candidate, "{kind}: {report}");
        let (role, effort) = candidate.split_once('-').unwrap();
        let (provider, program, model) = match role {
            "lead" => ("openai", "my-codex-wrapper", "your-codex-model"),
            _ => ("anthropic", "my-claude-wrapper", "your-claude-model"),
        };
        assert_eq!(selection["provider"], provider, "{kind}");
        assert_eq!(selection["effort"], effort, "{kind}");
        assert_eq!(
            report["argv"],
            serde_json::json!([program, "--model", model, "--effort", effort, "the mandate"]),
            "{kind}"
        );
        assert_eq!(
            report["policy"]["version"],
            "harness-dispatch/examples/grove-static 1"
        );
    }

    // Exact means exact: no catch-all, and no near miss, takes a route.
    for kind in ["release-notes", "review", "Impl", "impl "] {
        let refusal = sandbox.inspect(&["--kind", kind, "--json"]).refusal(3);
        assert_eq!(refusal["error"]["code"], "incomplete_mapping", "{kind:?}");
    }
}

#[test]
fn the_generic_example_routes_its_own_kinds_without_grove() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy("export { policy } from \"harness-dispatch/examples/static\";\n");
    wrappers(&sandbox, &["my-agent-wrapper"]);

    for (kind, candidate, effort) in [
        ("question", "quick", "low"),
        ("bugfix", "standard", "medium"),
        ("feature", "careful", "high"),
        ("migration", "deliberate", "xhigh"),
        ("architecture", "deliberate", "xhigh"),
    ] {
        let report = sandbox.inspect(&["--kind", kind, "--json"]).report();
        assert_eq!(report["selection"]["candidateId"], candidate, "{kind}");
        assert_eq!(report["selection"]["effort"], effort, "{kind}");
        assert_eq!(report["selection"]["provider"], "your-provider");
    }
    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "incomplete_mapping");

    // And it runs: the wrapper receives the model, effort and prompt.
    executable(&sandbox.bin.join("my-agent-wrapper"), support::FAKE_HARNESS);
    let run = sandbox.run(&["--kind", "bugfix", "--prompt", "fix the parser"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(
        sandbox.harness_args(),
        [
            "--model",
            "your-model",
            "--effort",
            "medium",
            "fix the parser"
        ]
    );
}

#[test]
fn the_dynamic_example_consults_the_static_routes_and_polices_explicit_choices() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy("export { policy } from \"harness-dispatch/examples/dynamic\";\n");
    wrappers(&sandbox, &["my-agent-wrapper"]);
    let inspect = |kind: &str, choice: Option<&str>| {
        let mut args = vec!["--kind", kind, "--json"];
        if let Some(choice) = choice {
            args.extend(["--choice", choice]);
        }
        sandbox.inspect(&args)
    };

    // Without a choice, the route applies, and the reason names its entry.
    let report = inspect("bugfix", None).report();
    assert_eq!(
        report["policy"]["version"],
        "harness-dispatch/examples/dynamic 1"
    );
    let selection = &report["selection"];
    assert_eq!(selection["form"], "select");
    assert_eq!(selection["selectedBy"], "select");
    assert_eq!(selection["candidateId"], "standard");
    assert_eq!(selection["effort"], "medium");
    assert_eq!(
        selection["reason"],
        r#"routes["bugfix"] names candidate "standard""#
    );

    // A choice at or above the route's effort is accepted, and one below it
    // is refused in the policy's own words.
    for accepted in ["standard", "careful", "deliberate"] {
        let report = inspect("bugfix", Some(accepted)).report();
        assert_eq!(report["selection"]["candidateId"], accepted);
        assert_eq!(report["selection"]["explicitChoice"], accepted);
        let reason = report["selection"]["reason"].as_str().unwrap();
        assert!(reason.contains("is accepted"), "{accepted}: {reason}");
    }
    let refusal = inspect("bugfix", Some("quick")).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused");
    assert_eq!(refusal["error"]["policyCode"], "effort_below_route");
    assert_eq!(refusal["error"]["input"], "--choice quick");
    assert!(
        refusal["error"]["remedy"]
            .as_str()
            .unwrap()
            .contains("\"standard\""),
        "{refusal}"
    );

    // A kind with no route refuses without a choice, and sets no floor with one.
    let refusal = inspect("release-notes", None).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused");
    assert_eq!(refusal["error"]["policyCode"], "incomplete_mapping");
    let report = inspect("release-notes", Some("quick")).report();
    assert_eq!(report["selection"]["candidateId"], "quick");

    // It is deterministic: the same request, the same result.
    assert_eq!(
        inspect("migration", None).report()["selection"],
        inspect("migration", None).report()["selection"]
    );

    // And it runs: the wrapper receives the chosen candidate's model and effort.
    executable(&sandbox.bin.join("my-agent-wrapper"), support::FAKE_HARNESS);
    let run = sandbox.run(&[
        "--kind",
        "feature",
        "--choice",
        "deliberate",
        "--prompt",
        "p",
    ]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(
        sandbox.harness_args(),
        ["--model", "your-model", "--effort", "xhigh", "p"]
    );
}

#[test]
fn the_dynamic_example_ships_its_declarations_and_readable_source() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(support::ROUTED);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    let worker = PathBuf::from(report["worker"]["path"].as_str().unwrap());
    let examples = worker.parent().unwrap().join("examples");

    let shipped = fs::read_to_string(examples.join("dynamic.ts")).unwrap();
    assert_eq!(shipped, include_str!("../worker/examples/dynamic.ts"));
    let declarations = fs::read_to_string(examples.join("dynamic.d.ts")).unwrap();
    for export in [
        "export declare function select(request: SelectionRequest): SelectionResult;",
        "export declare const policy:",
    ] {
        assert!(
            declarations.contains(export),
            "dynamic.d.ts lacks {export:?}"
        );
    }
    // It builds on the static example's routes by that example's specifier,
    // as an owner's copy of it would.
    assert!(shipped.contains(r#"from "harness-dispatch/examples/static""#));
}

#[test]
fn an_owner_catalog_can_take_an_examples_routes() {
    // The fixture `task dispatch:typecheck` checks against the shipped
    // declarations: the Grove example's routes over the owner's own catalog.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(include_str!("../worker/typecheck/examples-policy.ts"));

    let report = sandbox
        .inspect(&["--kind", "review-design", "--json"])
        .report();

    assert_eq!(report["policy"]["version"], "examples-fixture-1");
    assert_eq!(report["selection"]["candidateId"], "review-xhigh");
    assert_eq!(report["selection"]["provider"], "origin-b");
    assert_eq!(report["selection"]["model"], "model-for-review-xhigh");
    assert_eq!(report["executable"]["program"], "fake-harness");
}

#[test]
fn only_registered_example_specifiers_resolve() {
    // The prefix reserves nothing: an example that is not registered is an
    // ordinary bare import, and with no node_modules it fails to load. The
    // registered twin in the same shape is the control.
    let sandbox = Sandbox::new();
    wrappers(&sandbox, &["my-agent-wrapper"]);
    for (specifier, registered) in [
        ("harness-dispatch/examples/static", true),
        ("harness-dispatch/examples/no-such-example", false),
    ] {
        sandbox.personal_policy(&format!("export {{ policy }} from \"{specifier}\";\n"));
        let run = sandbox.inspect(&["--kind", "question", "--json"]);
        if registered {
            run.report();
        } else {
            let refusal = run.refusal(3);
            assert_eq!(refusal["error"]["code"], "policy_import_failed");
            assert!(
                refusal["error"]["message"]
                    .as_str()
                    .unwrap()
                    .contains("Cannot find package 'harness-dispatch'"),
                "{refusal}"
            );
        }
    }
}

#[test]
fn each_example_ships_declarations_and_a_readable_source_beside_the_worker() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(support::ROUTED);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    let worker = PathBuf::from(report["worker"]["path"].as_str().unwrap());
    let examples = worker.parent().unwrap().join("examples");

    for (name, source) in [
        ("static", include_str!("../worker/examples/static.ts")),
        (
            "grove-static",
            include_str!("../worker/examples/grove-static.ts"),
        ),
    ] {
        let shipped = fs::read_to_string(examples.join(format!("{name}.ts")))
            .unwrap_or_else(|error| panic!("{name}.ts beside the worker: {error}"));
        assert_eq!(
            shipped, source,
            "the readable {name} source is not the embedded one"
        );
        let declarations = fs::read_to_string(examples.join(format!("{name}.d.ts")))
            .unwrap_or_else(|error| panic!("{name}.d.ts beside the worker: {error}"));
        for export in [
            "export declare const catalog:",
            "export declare const routes:",
            "export declare const policy:",
            "export type CandidateId =",
        ] {
            assert!(
                declarations.contains(export),
                "{name}.d.ts lacks {export:?}"
            );
        }
        // Effort is explained by the work, never by ranking models.
        for property in [
            "abstraction",
            "uncertainty",
            "consequences",
            "downstream repair",
            "reversibility",
            "available checks",
        ] {
            assert!(
                source.contains(property),
                "{name} never explains {property}"
            );
        }
        assert!(
            source.contains("never by which model is better"),
            "{name} does not disclaim ranking models"
        );
    }
    assert!(text(&examples).ends_with("libexec/harness-dispatch/examples"));
}

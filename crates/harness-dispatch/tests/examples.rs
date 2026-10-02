//! The starter examples, through the command seam: each is imported from a
//! temporary personal policy by its embedded `harness-dispatch/examples/…`
//! specifier and selects through the compiled worker
//! (`docs/specs/harness-selection-and-execution.md`, *Policy and the selected
//! command*, *Policy authority and runtime discovery*). No node_modules exists
//! near any sandbox, so only the worker's registered modules can satisfy these
//! imports.

mod support;

use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};
use support::{executable, text, Sandbox};

/// Put a stand-in for each of an example's illustrative wrappers on PATH.
fn wrappers(sandbox: &Sandbox, names: &[&str]) {
    for name in names {
        executable(&sandbox.bin.join(name), "#!/bin/sh\nexit 0\n");
    }
}

/// The policy's own refusal of a kind its table does not list.
fn unrouted(sandbox: &Sandbox, kind: &str) -> Value {
    let refusal = sandbox.inspect(&["--kind", kind, "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused", "{kind:?}");
    assert_eq!(
        refusal["error"]["policyCode"], "incomplete_mapping",
        "{kind:?}: {refusal}"
    );
    assert_eq!(refusal["error"]["stage"], "selection", "{kind:?}");
    refusal
}

/// Every kind Grove launches a session for, and the harness and effort the
/// Grove example's table gives it.
const SESSION_KIND_ROUTES: [(&str, &str, &str); 23] = [
    ("requirements", "lead", "max"),
    ("review-requirements", "review", "xhigh"),
    ("integrate-review-requirements", "lead", "xhigh"),
    ("design", "lead", "xhigh"),
    ("review-design", "review", "xhigh"),
    ("integrate-review-design", "lead", "xhigh"),
    ("planning", "lead", "xhigh"),
    ("review-planning", "review", "xhigh"),
    ("integrate-review-planning", "lead", "xhigh"),
    ("prototype", "lead", "medium"),
    ("review-prototype", "review", "medium"),
    ("integrate-review-prototype", "lead", "medium"),
    ("impl", "lead", "high"),
    ("review-impl", "review", "high"),
    ("integrate-review-impl", "lead", "medium"),
    ("research-a", "lead", "high"),
    ("research-b", "review", "high"),
    ("combine-research", "lead", "xhigh"),
    ("draft", "lead", "high"),
    ("copy-edit", "review", "medium"),
    ("art", "lead", "medium"),
    ("proof", "review", "high"),
    ("finish", "lead", "medium"),
];

#[test]
fn the_grove_example_routes_every_grove_session_kind_exactly() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy("export { policy } from \"harness-dispatch/examples/grove-static\";\n");
    wrappers(&sandbox, &["my-codex-wrapper", "my-claude-wrapper"]);

    for (kind, harness, effort) in SESSION_KIND_ROUTES {
        let report = sandbox
            .inspect(&["--kind", kind, "--prompt", "the mandate", "--json"])
            .report();
        let (provider, program, model) = match harness {
            "lead" => ("openai", "my-codex-wrapper", "your-codex-model"),
            _ => ("anthropic", "my-claude-wrapper", "your-claude-model"),
        };
        // The reason names the entry applied.
        assert_eq!(
            report["selection"],
            json!({
                "provider": provider,
                "model": model,
                "effort": effort,
                "reason": format!(
                    "routes[\"{kind}\"] gives {program} with model {model} at effort {effort}"
                ),
            }),
            "{kind}: {report}"
        );
        // The prompt is the last argument, because the entry puts it there.
        assert_eq!(report["command"]["program"], program, "{kind}");
        assert_eq!(
            report["command"]["args"],
            json!(["--model", model, "--effort", effort, "the mandate"]),
            "{kind}"
        );
        assert_eq!(
            report["policy"]["version"],
            "harness-dispatch/examples/grove-static 2"
        );
    }

    // Exact means exact: no catch-all, and no near miss, takes a route.
    for kind in ["release-notes", "review", "Impl", "impl "] {
        let refusal = unrouted(&sandbox, kind);
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains(&serde_json::to_string(kind).unwrap()),
            "{refusal}"
        );
    }
}

#[test]
fn the_generic_example_routes_its_own_kinds_without_grove() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy("export { policy } from \"harness-dispatch/examples/static\";\n");
    wrappers(&sandbox, &["my-agent-wrapper"]);

    for (kind, effort) in [
        ("question", "low"),
        ("bugfix", "medium"),
        ("feature", "high"),
        ("migration", "xhigh"),
        ("architecture", "xhigh"),
    ] {
        let report = sandbox.inspect(&["--kind", kind, "--json"]).report();
        assert_eq!(
            report["policy"]["version"],
            "harness-dispatch/examples/static 2"
        );
        assert_eq!(
            report["selection"],
            json!({
                "provider": "your-provider",
                "model": "your-model",
                "effort": effort,
                "reason": format!(
                    "routes[\"{kind}\"] gives my-agent-wrapper with model your-model at effort {effort}"
                ),
            }),
            "{kind}"
        );
        assert_eq!(report["command"]["program"], "my-agent-wrapper", "{kind}");
    }
    // Exact means exact: no catch-all, and nothing inherited from an object.
    for kind in ["impl", "Feature", "constructor", "toString", "__proto__"] {
        unrouted(&sandbox, kind);
    }

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
fn an_owners_route_can_join_an_examples_table() {
    // The fixture `task dispatch:typecheck` checks against the shipped
    // declarations: the Grove example's table through the generic example's
    // `selectRoute`, with one kind rerouted to a command of the owner's own
    // and one kind added from the example's own harness.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(include_str!("../worker/typecheck/examples-policy.ts"));
    wrappers(&sandbox, &["my-codex-wrapper", "my-claude-wrapper"]);

    // A kind the owner left alone takes the example's entry.
    let report = sandbox
        .inspect(&["--kind", "review-design", "--json"])
        .report();
    assert_eq!(report["policy"]["version"], "examples-fixture-1");
    assert_eq!(report["selection"]["provider"], "anthropic");
    assert_eq!(report["selection"]["effort"], "xhigh");
    assert_eq!(report["command"]["program"], "my-claude-wrapper");

    // The rerouted kind takes the owner's, which reads a parameter the caller
    // passed and says so when it passed none.
    for (params, session) in [
        (
            &["--param", "session_name=parser grove"][..],
            "parser grove",
        ),
        (&[][..], "unnamed"),
    ] {
        let mut args = vec!["--kind", "impl", "--prompt", "the mandate", "--json"];
        args.extend(params);
        let report = sandbox.inspect(&args).report();
        assert_eq!(
            report["selection"],
            json!({
                "provider": "origin-a",
                "model": "model-for-impl",
                "effort": "high",
                "reason": "routes[\"impl\"] gives fake-harness with model model-for-impl at effort high",
            }),
            "{report}"
        );
        assert_eq!(
            report["command"]["args"],
            json!(["--session", session, "the mandate"])
        );
    }

    // The added kind takes the example's lead harness at the owner's effort.
    let report = sandbox.inspect(&["--kind", "spike", "--json"]).report();
    assert_eq!(report["selection"]["provider"], "openai");
    assert_eq!(report["selection"]["effort"], "low");
    assert_eq!(report["command"]["program"], "my-codex-wrapper");
    unrouted(&sandbox, "release-notes");
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

    for (name, source, exports) in [
        (
            "static",
            include_str!("../worker/examples/static.ts"),
            &[
                "export type Command =",
                "export type Route =",
                "export declare function agent(effort: string): Route;",
                "export declare const routes:",
                "export declare function selectRoute(",
                "export declare const policy:",
            ][..],
        ),
        (
            "grove-static",
            include_str!("../worker/examples/grove-static.ts"),
            &[
                "export declare const lead: (effort: string) => Route;",
                "export declare const review: (effort: string) => Route;",
                "export declare const routes:",
                "export declare const policy:",
            ][..],
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
        for export in exports {
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
    // The Grove example selects through the generic example's `selectRoute`,
    // by that example's specifier, as an owner's copy of it would.
    assert!(include_str!("../worker/examples/grove-static.ts")
        .contains(r#"from "harness-dispatch/examples/static""#));
    assert!(text(&examples).ends_with("libexec/harness-dispatch/examples"));
}

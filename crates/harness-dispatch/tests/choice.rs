//! `--choice` under a static routes policy, through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Policy and joint
//! choice*): an explicit choice selects any configured candidate, whatever the
//! routes say, and an ID the catalog lacks refuses without selecting anything
//! else.

mod support;

use serde_json::Value;
use support::{text, Sandbox};

/// `impl` routes to `deep`; `quick` is routed by no kind at all.
const TWO: &str = r#"export const policy = {
  schemaVersion: 1,
  version: "choice-1",
  catalog: [
    { id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness", args: [{ slot: "prompt" }] },
    { id: "quick", provider: "origin-b", model: "model-small", effort: "low", program: "fake-harness", args: ["--effort", { slot: "effort" }, { slot: "prompt" }] },
  ],
  routes: { impl: "deep" },
};
"#;

#[test]
fn an_explicit_choice_selects_a_candidate_for_a_kind_the_routes_do_not_name() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(TWO);

    // The control: without the choice, the kind has no route.
    let refusal = sandbox.inspect(&["--kind", "design", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "incomplete_mapping");

    let report = sandbox
        .inspect(&["--kind", "design", "--choice", "quick", "--json"])
        .report();
    let selection = &report["selection"];
    assert_eq!(selection["form"], "routes");
    assert_eq!(selection["selectedBy"], "explicit_choice");
    assert_eq!(selection["explicitChoice"], "quick");
    assert_eq!(selection["candidateId"], "quick");
    assert_eq!(selection["provider"], "origin-b");
    assert_eq!(selection["effort"], "low");
    let reason = selection["reason"].as_str().unwrap();
    assert!(
        reason.contains("--choice \"quick\"") && reason.contains("routes are not consulted"),
        "{reason}"
    );
    assert_eq!(
        report["argv"],
        serde_json::json!(["fake-harness", "--effort", "low", { "placeholder": "prompt" }])
    );

    let human = sandbox.inspect(&["--kind", "design", "--choice", "quick"]);
    assert_eq!(human.code, Some(0), "{}", human.stderr);
    for fact in [
        "choice     --choice quick, which selected the candidate",
        "candidate  quick",
        "reason     the explicit choice --choice \"quick\"",
    ] {
        assert!(
            human.stdout.contains(fact),
            "missing {fact:?}:\n{}",
            human.stdout
        );
    }
    let routed = sandbox.inspect(&["--kind", "impl"]);
    assert!(
        routed
            .stdout
            .contains("choice     none; the routes select by kind"),
        "{}",
        routed.stdout
    );
}

#[test]
fn an_explicit_choice_is_taken_over_the_kinds_own_route() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(TWO);

    let routed = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(routed["selection"]["candidateId"], "deep");
    assert_eq!(routed["selection"]["selectedBy"], "route");

    let chosen = sandbox
        .inspect(&["--kind", "impl", "--choice", "quick", "--json"])
        .report();
    assert_eq!(chosen["selection"]["candidateId"], "quick");
    assert_eq!(chosen["selection"]["selectedBy"], "explicit_choice");
}

#[test]
fn an_unknown_choice_refuses_without_selecting_anything_else() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(TWO);

    // `impl` has a route, and it is still not taken in place of the choice.
    for command in ["inspect", "run"] {
        let mut invocation = sandbox.command();
        invocation.args([
            command, "--kind", "impl", "--choice", "Deep", "--prompt", "p", "--json",
        ]);
        let refusal = support::run(&mut invocation).refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], "unknown_choice", "{command}: {refusal}");
        assert_eq!(error["stage"], "selection");
        assert_eq!(error["input"], "--choice Deep");
        assert_eq!(error["source"], text(&entry));
        assert_eq!(error["location"], "policy.catalog");
        let remedy = error["remedy"].as_str().unwrap();
        assert!(remedy.contains(r#"("deep", "quick")"#), "{remedy}");
        assert!(remedy.contains("never substitutes"), "{remedy}");
    }
    assert!(
        !sandbox.harness_ran(),
        "an unknown choice launched a harness"
    );
    assert!(
        !sandbox.home.join(".local").exists(),
        "an unknown choice created the record directory"
    );
}

#[test]
fn a_malformed_choice_refuses_before_any_policy_runs() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "await Bun.write({:?}, \"ran\");\n{TWO}",
        text(&sentinel)
    ));

    let refusal = sandbox
        .inspect(&["--kind", "impl", "--choice", "", "--json"])
        .refusal(2);
    assert_eq!(refusal["error"]["code"], "malformed_input");
    assert_eq!(refusal["error"]["input"], "--choice");
    assert!(!sentinel.exists(), "the policy ran for a malformed choice");

    // The control: a well-formed choice does evaluate the policy.
    sandbox
        .inspect(&["--kind", "impl", "--choice", "deep", "--json"])
        .report();
    assert!(sentinel.exists(), "the sentinel policy never ran");
}

#[test]
fn run_launches_the_explicit_choice_and_records_it() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(TWO);

    let run = sandbox.run(&[
        "--kind",
        "design",
        "--choice",
        "quick",
        "--prompt",
        "the prompt",
        "--json",
    ]);

    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["--effort", "low", "the prompt"]);
    let notice: Value = serde_json::from_str(&run.stderr).unwrap();
    assert_eq!(notice["handoff"]["selectedBy"], "explicit_choice");
    assert_eq!(notice["handoff"]["explicitChoice"], "quick");
    assert_eq!(notice["handoff"]["candidateId"], "quick");

    let run_id = sandbox.harness_run_id();
    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id, "--json"]);
    let export = support::run(&mut show).report();
    let selection = &export["launch"]["selection"];
    assert_eq!(selection["selectedBy"], "explicit_choice");
    assert_eq!(selection["explicitChoice"], "quick");
    assert_eq!(export["launch"]["candidate"]["id"], "quick");
    assert_eq!(export["launch"]["kind"], "design");

    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id]);
    let human = support::run(&mut show);
    assert!(
        human.stdout.contains("  choice     quick\n"),
        "{}",
        human.stdout
    );

    // Text mode says the candidate was chosen explicitly.
    std::fs::remove_dir_all(&sandbox.record).unwrap();
    let run = sandbox.run(&["--kind", "design", "--choice", "quick", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stderr
            .starts_with("harness-dispatch: running explicitly chosen candidate \"quick\""),
        "{}",
        run.stderr
    );
}

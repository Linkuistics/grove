//! The supplied review policy through the command seam: the shipped
//! `harness-dispatch/examples/review`, activated from a temporary personal
//! policy as an owner activates it, selecting reviews over a generic reviewed
//! artifact with no task file and no Grove
//! (`docs/specs/harness-selection-and-execution.md`, *Supplied review policy*,
//! *Identity and original creator*, and the shipped-examples row of *Agreed
//! test seams and acceptance*).
//!
//! Every creator run here is a real `run` of the front, through the example's
//! own wrappers, each a copy of the fake harness, so each lookup reads a record
//! that `run` committed. That includes the runs that never executed: one is
//! cancelled at the linearization point by the stall in `support::stall`, and
//! the other's wrapper names an interpreter that does not exist. The one
//! exception is the catalog-contract fixture, a store that release 21.13.0
//! committed. Each refusal has a control beside it, the same review with the
//! input corrected, that is seen to select.

mod support;

use std::fs;

use rusqlite::Connection;
use serde_json::{json, Value};
use support::{executable, run, stall, Sandbox, FAKE_HARNESS};

/// The shipped example, whole, as an owner's personal policy activates it.
const EXAMPLE: &str = "export { policy } from \"harness-dispatch/examples/review\";\n";

/// A run ID no store in these tests holds.
const UNKNOWN: &str = "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e";

/// The example's illustrative wrappers, each installed as the fake harness.
const WRAPPERS: [&str; 2] = ["my-agent-wrapper", "my-other-agent-wrapper"];

/// The origin the example's own harness carries, and the other one's.
const OWN: &str = "your-provider";
const OTHER: &str = "your-other-provider";

/// A sandbox whose personal policy is the shipped example.
fn sandbox() -> Sandbox {
    let sandbox = Sandbox::new();
    for name in WRAPPERS {
        executable(&sandbox.bin.join(name), FAKE_HARNESS);
    }
    sandbox.personal_policy(EXAMPLE);
    sandbox
}

/// Run `args` through to the harness, and return the run ID it was handed.
fn launched(sandbox: &Sandbox, args: &[&str]) -> String {
    let result = sandbox.run(args);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let run_id = sandbox.harness_run_id();
    fs::remove_dir_all(&sandbox.record).unwrap();
    run_id
}

/// A producer of kind `feature`, which the example routes to its own harness,
/// of origin `your-provider`, run as task `task_id`. Its run ID.
fn produce(sandbox: &Sandbox, task_id: &str) -> String {
    launched(
        sandbox,
        &[
            "--kind",
            "feature",
            "--task-id",
            task_id,
            "--prompt",
            "build it",
        ],
    )
}

/// The same producer under an owner's policy that routes `feature` to the
/// other harness, of origin `your-other-provider`; the shipped example is the
/// personal policy again afterwards. Its run ID.
fn produce_on_the_other_origin(sandbox: &Sandbox, task_id: &str) -> String {
    owner_policy(
        sandbox,
        r#"export const policy = {
  schemaVersion: 2,
  version: "other-producer-1",
  ...reviewSelector({ routes: { ...routes, feature: otherAgent("high") }, reviews }),
};
"#,
    );
    let run_id = produce(sandbox, task_id);
    sandbox.personal_policy(EXAMPLE);
    run_id
}

/// Write `review.json`: a caller context whose reviewed artifact is `id`,
/// with `creator` as its creator reference.
fn reviewing(sandbox: &Sandbox, id: &str, creator: &Value) {
    let document = json!({
        "schemaVersion": 1,
        "reviewedArtifact": { "id": id, "creator": creator },
    });
    sandbox.file("review.json", &document.to_string());
}

/// Inspect `kind` with `review.json` as its context, plus `extra`.
fn inspect(sandbox: &Sandbox, kind: &str, extra: &[&str]) -> support::Run {
    let mut args = vec![
        "--kind",
        kind,
        "--context",
        "review.json",
        "--prompt",
        "review it",
        "--json",
    ];
    args.extend(extra);
    sandbox.inspect(&args)
}

/// Run `kind` with `review.json` as its context, plus `extra`.
fn review(sandbox: &Sandbox, kind: &str, extra: &[&str]) -> support::Run {
    let mut args = vec![
        "--kind",
        kind,
        "--context",
        "review.json",
        "--prompt",
        "review it",
        "--json",
    ];
    args.extend(extra);
    sandbox.run(&args)
}

/// Run a review through to its harness: the run ID, and the arguments the
/// harness received.
fn reviewed(sandbox: &Sandbox, kind: &str, extra: &[&str]) -> (String, Vec<String>) {
    let result = review(sandbox, kind, extra);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let args = sandbox.harness_args();
    let run_id = sandbox.harness_run_id();
    fs::remove_dir_all(&sandbox.record).unwrap();
    (run_id, args)
}

fn show(sandbox: &Sandbox, run_id: &str) -> Value {
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", run_id, "--json"]);
    run(&mut command).report()
}

fn runs_in(sandbox: &Sandbox) -> i64 {
    Connection::open(sandbox.default_store())
        .unwrap()
        .query_row("SELECT count(*) FROM runs", [], |row| row.get(0))
        .unwrap()
}

/// The policy's own refusal, reported as `policy_refused`, exit 3, with its
/// code beside it; launching nothing.
fn refused(sandbox: &Sandbox, result: &support::Run, code: &str) -> Value {
    let refusal = result.refusal(3);
    let error = &refusal["error"];
    assert_eq!(error["code"], "policy_refused", "{refusal}");
    assert_eq!(error["policyCode"], code, "{refusal}");
    assert!(
        !sandbox.harness_ran(),
        "a refused review launched: {refusal}"
    );
    refusal
}

/// `field` of a refusal's error, as text.
fn said<'a>(refusal: &'a Value, field: &str) -> &'a str {
    refusal["error"][field]
        .as_str()
        .unwrap_or_else(|| panic!("no {field}: {refusal}"))
}

/// The arguments the example's wrappers receive for `model` at `effort`.
fn wrapper_args(model: &str, effort: &str, prompt: &str) -> Vec<String> {
    ["--model", model, "--effort", effort, prompt]
        .map(str::to_owned)
        .to_vec()
}

/// What [`inspect`] reports of the command a review selected: its provider
/// label, its program, and the model and effort, which its arguments carry
/// before the prompt.
fn reviewer(report: &Value) -> (String, String, String, String) {
    let text = |value: &Value| value.as_str().unwrap_or_default().to_owned();
    assert_eq!(
        report["command"]["args"],
        json!([
            "--model",
            report["selection"]["model"],
            "--effort",
            report["selection"]["effort"],
            "review it"
        ]),
        "{report}"
    );
    (
        text(&report["selection"]["provider"]),
        text(&report["command"]["program"]),
        text(&report["selection"]["model"]),
        text(&report["selection"]["effort"]),
    )
}

/// The example's other harness at `effort`, as [`reviewer`] reports it.
fn other_agent(effort: &str) -> (String, String, String, String) {
    (
        OTHER.to_owned(),
        "my-other-agent-wrapper".to_owned(),
        "your-other-model".to_owned(),
        effort.to_owned(),
    )
}

/// The example's own harness at `effort`, as [`reviewer`] reports it.
fn own_agent(effort: &str) -> (String, String, String, String) {
    (
        OWN.to_owned(),
        "my-agent-wrapper".to_owned(),
        "your-model".to_owned(),
        effort.to_owned(),
    )
}

/// Write an owner's policy that builds on the examples' exports: `body` is
/// TypeScript that may use the review example's `routes`, `reviews`,
/// `reviewSelector`, `lookUpCreator` and `otherAgent`, and the generic
/// example's `agent`, and must export `policy`.
fn owner_policy(sandbox: &Sandbox, body: &str) {
    sandbox.personal_policy(&format!(
        "import {{ routes, reviews, reviewSelector, lookUpCreator, otherAgent }} from \"harness-dispatch/examples/review\";\n\
         import {{ agent }} from \"harness-dispatch/examples/static\";\n{body}"
    ));
}

#[test]
fn a_review_selects_a_reviewer_of_another_origin_on_every_invocation() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(
        report["policy"]["version"],
        "harness-dispatch/examples/review 2"
    );
    assert_eq!(reviewer(&report), other_agent("high"), "{report}");
    let selection = &report["selection"];
    let reason = selection["reason"].as_str().unwrap();
    for part in [
        creator.as_str(),
        r#"recorded origin "your-provider""#,
        r#"reviews["code-review"]["your-provider"] gives my-other-agent-wrapper, of origin "your-other-provider""#,
    ] {
        assert!(reason.contains(part), "{part:?} is not in {reason:?}");
    }
    assert_eq!(report["creator"]["evidence"], "execution_recorded");
    assert_eq!(report["creator"]["provider"], OWN);
    assert_eq!(report["creator"]["reference"], json!({ "run": creator }));

    // Inspection is deterministic, and each run is a retry: a new run, the
    // same answer, the same reviewer launched.
    assert_eq!(
        inspect(&sandbox, "code-review", &[]).report()["selection"],
        *selection
    );
    let mut retries = Vec::new();
    for _ in 0..2 {
        let (run_id, args) = reviewed(&sandbox, "code-review", &[]);
        assert_eq!(args, wrapper_args("your-other-model", "high", "review it"));
        let launch = show(&sandbox, &run_id)["launch"].clone();
        assert_eq!(launch["candidate"]["provider"], OTHER);
        assert_eq!(launch["executable"]["program"], "my-other-agent-wrapper");
        assert_eq!(launch["creator"]["provider"], OWN);
        assert_eq!(launch["creator"]["evidence"], "execution_recorded");
        assert_eq!(launch["creator"]["reference"], json!({ "run": creator }));
        retries.push(run_id);
    }
    assert_ne!(retries[0], retries[1], "a retry is a run of its own");

    // The architecture review applies the same rule at its own effort.
    let report = inspect(&sandbox, "architecture-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("xhigh"));

    // A creator of the other origin is reviewed on the first.
    let other = produce_on_the_other_origin(&sandbox, "feature-k8");
    reviewing(&sandbox, "feature-k8", &json!({ "run": other }));
    for (kind, effort) in [("code-review", "high"), ("architecture-review", "xhigh")] {
        let report = inspect(&sandbox, kind, &[]).report();
        assert_eq!(reviewer(&report), own_agent(effort), "{kind}");
        assert_eq!(report["creator"]["provider"], OTHER);
    }
    let (_, args) = reviewed(&sandbox, "code-review", &[]);
    assert_eq!(args, wrapper_args("your-model", "high", "review it"));
}

/// An owner's policy whose `code-review` entry sends a creator of origin
/// `your-provider` to `reviewer`, a route expression, and which routes the
/// kind `probe` to that same route, so that the command it builds can be
/// inspected with no review.
fn reviewed_by(sandbox: &Sandbox, version: &str, reviewer: &str) {
    owner_policy(
        sandbox,
        &format!(
            r#"const reviewer = {reviewer};
export const policy = {{
  schemaVersion: 2,
  version: "{version}",
  ...reviewSelector({{
    routes: {{ ...routes, probe: reviewer }},
    reviews: {{ "code-review": {{ "your-provider": reviewer, "your-other-provider": agent("high") }} }},
  }}),
}};
"#
        ),
    );
}

/// The command a policy builds for `kind`, with no review: what inspection
/// reports as its selection and its command.
fn offered(sandbox: &Sandbox, kind: &str) -> Value {
    sandbox
        .inspect(&["--kind", kind, "--prompt", "review it", "--json"])
        .report()
}

#[test]
fn a_gateway_to_the_creators_model_keeps_its_origin_and_refuses() {
    let sandbox = sandbox();
    executable(&sandbox.bin.join("my-gateway-wrapper"), FAKE_HARNESS);
    let creator = produce(&sandbox, "feature-k7");
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    // An owner's entry reviews the creator's work through a gateway to the
    // creator's own model: another program and another model string, under
    // the label of the origin the model has.
    reviewed_by(
        &sandbox,
        "gateway-1",
        r#"(request) => ({
  program: "my-gateway-wrapper",
  args: ["--model", "your-gateway/your-model", "--effort", "high", request.prompt],
  provider: "your-provider",
  model: "your-gateway/your-model",
  effort: "high",
})"#,
    );

    // The disguise, read as each command is built for ordinary work. Were
    // the gateway to lose it, its refusal below would show nothing that the
    // creator's own harness as the reviewer does not.
    let (own, gateway) = (offered(&sandbox, "feature"), offered(&sandbox, "probe"));
    assert_eq!(own["selection"]["provider"], OWN);
    assert_eq!(gateway["selection"]["provider"], OWN);
    assert_ne!(gateway["command"]["program"], own["command"]["program"]);
    assert_ne!(gateway["selection"]["model"], own["selection"]["model"]);
    assert_ne!(gateway["command"]["args"], own["command"]["args"]);

    // It refuses on every invocation. Nothing is launched or recorded, and
    // nothing is run in its place.
    let before = runs_in(&sandbox);
    for result in [
        inspect(&sandbox, "code-review", &[]),
        review(&sandbox, "code-review", &[]),
        review(&sandbox, "code-review", &[]),
    ] {
        let refusal = refused(&sandbox, &result, "same_origin");
        assert_eq!(refusal["error"]["input"], "--kind code-review");
        let message = said(&refusal, "message");
        for part in [
            r#"reviews["code-review"]["your-provider"] gives my-gateway-wrapper"#,
            r#"of origin "your-provider""#,
            "a gateway to a model keeps the model's origin",
            creator.as_str(),
        ] {
            assert!(message.contains(part), "{part:?}: {refusal}");
        }
        assert!(
            said(&refusal, "remedy").contains("a command of another origin"),
            "{refusal}"
        );
    }
    assert_eq!(runs_in(&sandbox), before, "a refused review was recorded");

    // The control: the shipped entry sends the same creator to the other
    // origin.
    sandbox.personal_policy(EXAMPLE);
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("high"));
}

#[test]
fn a_commands_origin_is_its_provider_label_whatever_its_argv() {
    // The gateway case above sets a different argv under the creator's label,
    // and refuses. This is the other half: the creator's own program, model
    // and arguments under the other origin's label. The rule reads the label
    // and nothing else, so that command reviews the creator's work, and the
    // command it copies is refused in its place.
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    reviewed_by(
        &sandbox,
        "relabelled-twin-1",
        r#"(request) => ({ ...agent("high")(request), provider: "your-other-provider" })"#,
    );

    let (own, twin) = (offered(&sandbox, "feature"), offered(&sandbox, "probe"));
    assert_eq!(own["policy"]["version"], "relabelled-twin-1");
    assert_eq!(twin["command"], own["command"]);
    assert_eq!(twin["selection"]["model"], own["selection"]["model"]);
    assert_eq!(own["selection"]["provider"], OWN);
    assert_eq!(twin["selection"]["provider"], OTHER);

    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(report["selection"]["provider"], OTHER);
    assert_eq!(report["command"], own["command"]);
    assert_eq!(report["creator"]["provider"], OWN);
    let (_, args) = reviewed(&sandbox, "code-review", &[]);
    assert_eq!(args, wrapper_args("your-model", "high", "review it"));

    reviewed_by(&sandbox, "twin-unlabelled-1", r#"agent("high")"#);
    refused(
        &sandbox,
        &inspect(&sandbox, "code-review", &[]),
        "same_origin",
    );
}

#[test]
fn a_mapped_reviewer_of_the_creators_origin_refuses_and_is_never_replaced() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    // The control: the shipped entry maps this creator to the other origin.
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("high"));

    // An owner's entry that maps the creator's origin to that origin refuses,
    // although the entry holds a command of the other origin too.
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 2,
  version: "misfiled-1",
  ...reviewSelector({
    routes,
    reviews: { "code-review": { "your-provider": agent("high"), "your-other-provider": otherAgent("high") } },
  }),
};
"#,
    );
    for result in [
        inspect(&sandbox, "code-review", &[]),
        review(&sandbox, "code-review", &[]),
    ] {
        let refusal = refused(&sandbox, &result, "same_origin");
        let message = said(&refusal, "message");
        assert!(
            message.contains(r#"reviews["code-review"]["your-provider"] gives my-agent-wrapper"#),
            "{refusal}"
        );
        assert!(!message.contains("my-other-agent-wrapper"), "{refusal}");
    }
}

#[test]
fn an_unknown_run_or_a_run_that_never_executed_refuses_with_the_declaration_remedy() {
    let sandbox = sandbox();
    let executed = produce(&sandbox, "feature-k7");

    // A run whose exec failed: its wrapper names an interpreter that does
    // not exist, so the attempt carries dispatch's exec-error detail.
    let wrapper = sandbox.bin.join("my-agent-wrapper");
    executable(&wrapper, "#!/nonexistent/interpreter\n");
    let result = sandbox.run(&[
        "--kind",
        "feature",
        "--task-id",
        "feature-k8",
        "--prompt",
        "p",
        "--json",
    ]);
    assert_eq!(result.code, Some(127), "{}", result.stderr);
    let notice: Value = serde_json::from_str(result.stderr.lines().next().unwrap()).unwrap();
    let exec_failed = notice["handoff"]["runId"].as_str().unwrap().to_owned();
    executable(&wrapper, FAKE_HARNESS);

    // A run cancelled after its handoff was committed, so its harness was
    // never started: the attempt carries the not-executed detail.
    let stalled = stall::stalled(
        &sandbox,
        &[
            "--kind",
            "feature",
            "--task-id",
            "feature-k9",
            "--prompt",
            "p",
            "--json",
        ],
        None,
        Some(libc::SIGTERM),
        runs_in(&sandbox),
    );
    assert_eq!(stalled.signal, Some(libc::SIGTERM), "{}", stalled.stderr);
    assert!(!sandbox.harness_ran());
    let notice: Value = serde_json::from_str(
        stalled
            .stderr
            .lines()
            .find(|line| !line.is_empty())
            .unwrap(),
    )
    .unwrap();
    let cancelled = notice["handoff"]["runId"].as_str().unwrap().to_owned();
    assert_eq!(show(&sandbox, &cancelled)["execution"], "not_executed");

    for (creator, code, detail) in [
        (
            UNKNOWN,
            "creator_run_missing",
            "is not in this record store",
        ),
        (exec_failed.as_str(), "creator_not_executed", "exec_error"),
        (cancelled.as_str(), "creator_not_executed", "cancelled"),
    ] {
        reviewing(&sandbox, "feature-k8", &json!({ "run": creator }));
        for result in [
            inspect(&sandbox, "code-review", &[]),
            review(&sandbox, "code-review", &[]),
        ] {
            let refusal = refused(&sandbox, &result, code);
            let message = said(&refusal, "message");
            for part in [creator, detail] {
                assert!(message.contains(part), "{part:?}: {refusal}");
            }
            // The declaration remedy, with the origins the entry lists.
            let remedy = said(&refusal, "remedy");
            for part in [
                r#""declared""#,
                r#""your-provider""#,
                r#""your-other-provider""#,
            ] {
                assert!(remedy.contains(part), "{part}: {refusal}");
            }
        }
    }

    // The control: the run that did execute selects.
    reviewing(&sandbox, "feature-k7", &json!({ "run": executed }));
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("high"));
}

#[test]
fn a_declared_creator_is_adopted_and_labelled_declared() {
    let sandbox = sandbox();
    for (declared, expected) in [(OTHER, own_agent("high")), (OWN, other_agent("high"))] {
        reviewing(&sandbox, "legacy-k3", &json!({ "declared": declared }));
        let report = inspect(&sandbox, "code-review", &[]).report();
        assert_eq!(reviewer(&report), expected, "{declared}");
        let reason = report["selection"]["reason"].as_str().unwrap();
        assert!(
            reason.contains(&format!(
                r#"creator origin "{declared}" declared by the owner"#
            )),
            "{reason}"
        );
        let provenance = json!({
            "reference": { "declared": declared },
            "evidence": "declared",
            "provider": declared,
            "lookup": null,
        });
        assert_eq!(report["creator"], provenance);
        // Nothing was looked up: the caller's document is the one source.
        let sources = report["context"]["sources"].as_array().unwrap();
        assert_eq!(sources.len(), 1, "{report}");
        assert_eq!(sources[0]["via"], "--context");
        assert!(report["context"]["value"].get("runs").is_none());

        let result = review(&sandbox, "code-review", &[]);
        assert_eq!(result.code, Some(0), "{}", result.stderr);
        let run_id = sandbox.harness_run_id();
        fs::remove_dir_all(&sandbox.record).unwrap();
        let launch = show(&sandbox, &run_id)["launch"].clone();
        assert_eq!(launch["creator"], provenance);
        assert_eq!(launch["executable"]["program"], expected.1.as_str());

        // Text inspection says it was declared.
        let mut command = sandbox.command();
        command.args([
            "inspect",
            "--kind",
            "code-review",
            "--context",
            "review.json",
        ]);
        let text = run(&mut command);
        assert_eq!(text.code, Some(0), "{}", text.stderr);
        assert!(
            text.stdout
                .contains(&format!("declared {declared} (declared by the owner)")),
            "{}",
            text.stdout
        );
    }
}

#[test]
fn a_relabelled_origin_and_a_misspelt_declaration_refuse_as_unlisted() {
    let sandbox = sandbox();

    // The producer ran under an earlier policy, which labelled the origin
    // `Your-Provider`. The run records that label, and today's review entry
    // lists the origin under another.
    sandbox.personal_policy(
        r#"export const policy = {
  schemaVersion: 2,
  version: "earlier-1",
  select: (request) => ({
    status: "selected",
    program: "my-agent-wrapper",
    args: ["--model", "your-model", "--effort", "high", request.prompt],
    provider: "Your-Provider",
    model: "your-model",
    effort: "high",
    reason: "the earlier label",
  }),
};
"#,
    );
    let relabelled = produce(&sandbox, "feature-k7");
    sandbox.personal_policy(EXAMPLE);

    // Compared rather than checked against the entry, `Your-Provider` would
    // differ from `your-provider`, and an entry that sent it to the creator's
    // own harness would pass as another origin's review.
    reviewing(&sandbox, "feature-k7", &json!({ "run": relabelled }));
    for result in [
        inspect(&sandbox, "code-review", &[]),
        review(&sandbox, "code-review", &[]),
    ] {
        let refusal = refused(&sandbox, &result, "creator_origin_unlisted");
        let message = said(&refusal, "message");
        for part in [
            relabelled.as_str(),
            r#""Your-Provider""#,
            r#"reviews["code-review"] lists"#,
            r#""your-other-provider", "your-provider""#,
            "Origins match exactly, with no normalisation",
        ] {
            assert!(message.contains(part), "{part:?}: {refusal}");
        }
        let remedy = said(&refusal, "remedy");
        for part in [
            r#"if "Your-Provider" was relabelled"#,
            "a new label is not a new provider",
            r#""declared""#,
        ] {
            assert!(remedy.contains(part), "{part:?}: {refusal}");
        }
    }

    // A declaration is matched exactly: case, spelling and space all count.
    for misspelt in [
        "your-provder",
        "Your-provider",
        "your_provider",
        "your-provider ",
        " your-provider",
    ] {
        reviewing(&sandbox, "legacy-k3", &json!({ "declared": misspelt }));
        let refusal = refused(
            &sandbox,
            &inspect(&sandbox, "code-review", &[]),
            "creator_origin_unlisted",
        );
        let message = said(&refusal, "message");
        assert!(
            message.contains(&format!("{misspelt:?}")),
            "{misspelt:?}: {refusal}"
        );
        let remedy = said(&refusal, "remedy");
        assert!(
            remedy.starts_with("correct the declaration to one of"),
            "{refusal}"
        );
        assert!(remedy.contains(r#""your-provider""#), "{refusal}");
    }

    // An entry lists the origins a creator can have, so one that maps only
    // the other origin refuses this creator the same way, naming what it
    // lists.
    let creator = produce(&sandbox, "feature-k9");
    reviewing(&sandbox, "feature-k9", &json!({ "run": creator }));
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 2,
  version: "partial-1",
  ...reviewSelector({ routes, reviews: { "code-review": { "your-other-provider": agent("high") } } }),
};
"#,
    );
    let refusal = refused(
        &sandbox,
        &inspect(&sandbox, "code-review", &[]),
        "creator_origin_unlisted",
    );
    let message = said(&refusal, "message");
    for part in [
        r#"recorded origin "your-provider""#,
        r#"reviews["code-review"] lists"#,
        r#": "your-other-provider"."#,
    ] {
        assert!(message.contains(part), "{part:?}: {refusal}");
    }
    sandbox.personal_policy(EXAMPLE);

    // The control: spelt exactly, the same declaration is listed, and the
    // creator's work goes to the other origin.
    reviewing(&sandbox, "legacy-k3", &json!({ "declared": OTHER }));
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(reviewer(&report), own_agent("high"));
}

#[test]
fn a_changed_policy_does_not_change_the_creators_recorded_origin() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    let before = inspect(&sandbox, "code-review", &[]).report();

    // Since then the owner has routed `feature` to the other harness. Read
    // from what the policy returns today, the creator would be
    // `your-other-provider`, and its review would go to the creator's own
    // origin.
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 2,
  version: "remapped-1",
  ...reviewSelector({ routes: { ...routes, feature: otherAgent("high") }, reviews }),
};
"#,
    );
    let after = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(after["policy"]["version"], "remapped-1");
    assert_eq!(after["creator"], before["creator"]);
    assert_eq!(after["creator"]["provider"], OWN);
    let lookup = &after["creator"]["lookup"];
    assert_eq!(
        json!([lookup["provider"], lookup["model"], lookup["effort"]]),
        json!([OWN, "your-model", "high"])
    );
    assert_eq!(reviewer(&after), other_agent("high"));

    // The control: today's routes did reach the policy.
    let routed = offered(&sandbox, "feature");
    assert_eq!(routed["selection"]["provider"], OTHER);
}

#[test]
fn a_run_of_another_task_identity_is_admitted_and_shown() {
    let sandbox = sandbox();
    // A decomposed producer is finished by a child task with its own
    // identity, and a run need not have one at all.
    let child = produce(&sandbox, "child-k9");
    let anonymous = launched(&sandbox, &["--kind", "feature", "--prompt", "p"]);

    for (creator, task) in [(&child, "\"child-k9\""), (&anonymous, "no task")] {
        reviewing(&sandbox, "parent-k8", &json!({ "run": creator }));
        let report = inspect(&sandbox, "code-review", &[]).report();
        assert_eq!(reviewer(&report), other_agent("high"));
        assert_eq!(report["reviewedArtifact"]["id"], "parent-k8");
        let reason = report["selection"]["reason"].as_str().unwrap();
        for part in ["\"parent-k8\"", task] {
            assert!(reason.contains(part), "{part:?} is not in {reason:?}");
        }
    }

    reviewing(&sandbox, "parent-k8", &json!({ "run": child }));
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(report["creator"]["lookup"]["taskId"], "child-k9");
    // Text inspection shows the reviewed ID and the run's task side by side.
    let mut command = sandbox.command();
    command.args([
        "inspect",
        "--kind",
        "code-review",
        "--context",
        "review.json",
    ]);
    let text = run(&mut command);
    assert_eq!(text.code, Some(0), "{}", text.stderr);
    let row = |name: &str| {
        text.stdout
            .lines()
            .find(|line| line.trim_start().starts_with(&format!("{name} ")))
            .unwrap_or_else(|| panic!("no {name} row:\n{}", text.stdout))
            .to_owned()
    };
    assert!(row("reviewed").contains("parent-k8"), "{}", text.stdout);
    assert!(row("creator").contains("task child-k9"), "{}", text.stdout);
}

#[test]
fn a_custom_review_label_applies_the_rule_only_when_the_owner_lists_it() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    // Unlisted, a kind that names a reviewed artifact refuses: it would
    // otherwise take a route without the rule. Near misses of listed kinds
    // are unlisted too.
    for kind in ["security-review", "Code-Review", "code-review ", "review"] {
        let refusal = refused(
            &sandbox,
            &inspect(&sandbox, kind, &[]),
            "review_kind_unlisted",
        );
        assert!(
            said(&refusal, "message").contains("feature-k7"),
            "{refusal}"
        );
        assert!(said(&refusal, "remedy").contains("reviews"), "{refusal}");
    }
    // Without a reviewed artifact it is an ordinary kind, which the static
    // routes do not name.
    let refusal = refused(
        &sandbox,
        &sandbox.inspect(&["--kind", "security-review", "--json"]),
        "incomplete_mapping",
    );
    assert!(said(&refusal, "message").contains("security-review"));

    // An owner who routes it, without listing it, gets the route only when
    // no reviewed artifact is named.
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 2,
  version: "routed-1",
  ...reviewSelector({ routes: { ...routes, "security-review": agent("high") }, reviews }),
};
"#,
    );
    let report = offered(&sandbox, "security-review");
    assert_eq!(reviewer(&report), own_agent("high"));
    refused(
        &sandbox,
        &inspect(&sandbox, "security-review", &[]),
        "review_kind_unlisted",
    );

    // Listed, the rule applies to it.
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 2,
  version: "listed-1",
  ...reviewSelector({
    routes,
    reviews: {
      ...reviews,
      "security-review": { "your-provider": otherAgent("xhigh"), "your-other-provider": agent("xhigh") },
    },
  }),
};
"#,
    );
    let report = inspect(&sandbox, "security-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("xhigh"));
    reviewing(&sandbox, "legacy-k3", &json!({ "declared": OTHER }));
    let report = inspect(&sandbox, "security-review", &[]).report();
    assert_eq!(reviewer(&report), own_agent("xhigh"));
}

#[test]
fn a_review_without_its_creator_refuses_and_names_the_remedy() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");

    // No context at all, and a context naming no reviewed artifact.
    let refusal = refused(
        &sandbox,
        &sandbox.inspect(&["--kind", "code-review", "--json"]),
        "reviewed_artifact_missing",
    );
    assert!(
        said(&refusal, "remedy").contains("reviewedArtifact"),
        "{refusal}"
    );
    sandbox.file(
        "review.json",
        &json!({ "schemaVersion": 1, "summary": "a review" }).to_string(),
    );
    refused(
        &sandbox,
        &inspect(&sandbox, "code-review", &[]),
        "reviewed_artifact_missing",
    );

    // A reviewed artifact with no creator.
    sandbox.file(
        "review.json",
        &json!({ "schemaVersion": 1, "reviewedArtifact": { "id": "feature-k7" } }).to_string(),
    );
    let refusal = refused(
        &sandbox,
        &inspect(&sandbox, "code-review", &[]),
        "creator_missing",
    );
    let remedy = said(&refusal, "remedy");
    for part in [r#""run""#, r#""declared""#, r#""your-other-provider""#] {
        assert!(remedy.contains(part), "{part}: {refusal}");
    }

    // A policy that selects with the example's `select` but never looks the
    // run up cannot know the creator's origin, and says so rather than
    // reading the run as missing.
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    owner_policy(
        &sandbox,
        r#"const { select } = reviewSelector({ routes, reviews });
export const policy = { schemaVersion: 2, version: "no-lookup-1", select };
"#,
    );
    let refusal = refused(
        &sandbox,
        &inspect(&sandbox, "code-review", &[]),
        "creator_not_looked_up",
    );
    assert!(
        said(&refusal, "remedy").contains("lookUpCreator"),
        "{refusal}"
    );

    // The control: a loader that looks it up, composed from the same parts.
    owner_policy(
        &sandbox,
        r#"const { select } = reviewSelector({ routes, reviews });
export const policy = {
  schemaVersion: 2,
  version: "composed-1",
  loadContext: (request, host) => lookUpCreator(request.context, host),
  select,
};
"#,
    );
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("high"));
}

#[test]
fn other_kinds_take_the_owners_static_routes() {
    let sandbox = sandbox();
    for (kind, effort) in [
        ("question", "low"),
        ("bugfix", "medium"),
        ("feature", "high"),
        ("migration", "xhigh"),
        ("architecture", "xhigh"),
    ] {
        let report = offered(&sandbox, kind);
        assert_eq!(reviewer(&report), own_agent(effort), "{kind}");
        assert_eq!(
            report["selection"]["reason"],
            format!(
                r#"routes["{kind}"] gives my-agent-wrapper with model your-model at effort {effort}"#
            )
        );
    }

    // Exact means exact: no catch-all, and nothing inherited from an object.
    for kind in [
        "release-notes",
        "Feature",
        "constructor",
        "toString",
        "__proto__",
    ] {
        let refusal = refused(
            &sandbox,
            &sandbox.inspect(&["--kind", kind, "--json"]),
            "incomplete_mapping",
        );
        assert!(
            said(&refusal, "remedy").contains("add a route for this kind"),
            "{refusal}"
        );
    }
}

#[test]
fn an_unreadable_store_refuses_before_the_selector_sees_it() {
    // The store cannot say whether the run exists, which is not the same as
    // its absence: harness-dispatch refuses with the store's own exit-4
    // refusal, and the selector never answers `creator_run_missing` or
    // selects. The control is the same review against the readable store.
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    let store = sandbox.default_store();
    let good = fs::read(&store).unwrap();
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("high"));

    support::write(&store, &"this is not a database\n".repeat(200));
    for result in [
        inspect(&sandbox, "code-review", &[]),
        review(&sandbox, "code-review", &[]),
    ] {
        let refusal = result.refusal(4);
        assert_eq!(
            refusal["error"]["code"], "record_store_invalid",
            "{refusal}"
        );
        assert_eq!(refusal["error"]["stage"], "record", "{refusal}");
        assert!(refusal["error"]["policyCode"].is_null(), "{refusal}");
        assert!(!sandbox.harness_ran(), "{refusal}");
    }

    fs::write(&store, &good).unwrap();
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("high"));
}

#[test]
fn the_generic_form_selects_with_no_task_file_or_grove() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    assert!(
        !sandbox.cwd.join(".grove").exists() && !sandbox.bin.join("grove").exists(),
        "the sandbox holds no Grove"
    );

    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(reviewer(&report), other_agent("high"));
    assert_eq!(report["taskFile"], Value::Null);
    assert_eq!(report["taskId"], Value::Null);
    // The policy read nothing: its sources are the caller's document and the
    // run it looked up.
    let via: Vec<&Value> = report["context"]["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| &source["via"])
        .collect();
    assert_eq!(via, [&json!("--context"), &json!("run")]);

    let (run_id, _) = reviewed(&sandbox, "code-review", &[]);
    let launch = show(&sandbox, &run_id)["launch"].clone();
    assert_eq!(launch["taskFile"], Value::Null);
    assert_eq!(launch["reviewedArtifact"]["id"], "feature-k7");
    assert_eq!(launch["creator"]["provider"], OWN);
}

/// The record store release 21.13.0 wrote under the catalog contract, and the
/// two runs it holds (`tests/fixtures/catalog-contract/README.md`).
const CATALOG_CONTRACT_STORE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/catalog-contract/records.sqlite3"
);
const CATALOG_CONTRACT_BUILDER: &str = "45308255-7446-42b2-bbd7-e5f7861e46ef";
const CATALOG_CONTRACT_AUDITOR: &str = "b2aec552-b5eb-4de0-b5e9-5db0d70a6d55";

#[test]
fn a_creator_recorded_under_the_catalog_contract_still_gives_its_provider() {
    let sandbox = sandbox();
    let state = sandbox.root.join("catalog-contract");
    fs::create_dir(&state).unwrap();
    fs::copy(CATALOG_CONTRACT_STORE, state.join("records.sqlite3")).unwrap();
    let state = support::text(&state);
    let state_dir = ["--state-dir", state.as_str()];

    // The run recorded `your-provider` beside a candidate ID, and the lookup
    // answers its labels, kind and task identity as it answers any run's.
    reviewing(
        &sandbox,
        "parser-k12",
        &json!({ "run": CATALOG_CONTRACT_BUILDER }),
    );
    let report = inspect(&sandbox, "code-review", &state_dir).report();
    assert_eq!(
        report["creator"],
        json!({
            "reference": { "run": CATALOG_CONTRACT_BUILDER },
            "evidence": "execution_recorded",
            "provider": OWN,
            "lookup": {
                "runId": CATALOG_CONTRACT_BUILDER,
                "status": "found",
                "recordedAt": "2026-10-02T04:11:46.561Z",
                "kind": "build",
                "taskId": "parser-k12",
                "provider": OWN,
                "model": "model-a",
                "effort": "high",
                "launchFailure": null,
            },
        }),
        "{report}"
    );
    assert_eq!(reviewer(&report), other_agent("high"));
    let reason = report["selection"]["reason"].as_str().unwrap();
    for part in [
        CATALOG_CONTRACT_BUILDER,
        r#"task "parser-k12", kind "build""#,
        r#"recorded origin "your-provider""#,
    ] {
        assert!(reason.contains(part), "{part:?} is not in {reason:?}");
    }

    // Text inspection shows the run's task identity beside its labels.
    let mut command = sandbox.command();
    command.args([
        "inspect",
        "--kind",
        "code-review",
        "--context",
        "review.json",
    ]);
    command.args(state_dir);
    let text = run(&mut command);
    assert_eq!(text.code, Some(0), "{}", text.stderr);
    let creator = format!(
        "run {CATALOG_CONTRACT_BUILDER} (execution-recorded): provider your-provider, model \
         model-a, effort high; task parser-k12, kind build, recorded 2026-10-02T04:11:46.561Z"
    );
    assert!(text.stdout.contains(&creator), "{}", text.stdout);

    // The review launches beside that run, and records the provenance it
    // used.
    let (run_id, args) = reviewed(&sandbox, "code-review", &state_dir);
    assert_eq!(args, wrapper_args("your-other-model", "high", "review it"));
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", &run_id, "--json"]);
    command.args(state_dir);
    let launch = run(&mut command).report()["launch"].clone();
    assert_eq!(launch["creator"], report["creator"]);

    // The fixture's other run, of the other origin, is reviewed on the first.
    reviewing(
        &sandbox,
        "parser-k13",
        &json!({ "run": CATALOG_CONTRACT_AUDITOR }),
    );
    let report = inspect(&sandbox, "code-review", &state_dir).report();
    assert_eq!(report["creator"]["provider"], OTHER);
    assert_eq!(report["creator"]["lookup"]["taskId"], "parser-k13");
    assert_eq!(reviewer(&report), own_agent("high"));
}

#[test]
fn the_typed_review_fixture_selects_through_the_worker() {
    // `worker/typecheck/review-policy.ts` is type-checked against the shipped
    // declarations by `task dispatch:typecheck`; here the same file runs, so
    // the declared selector and the runtime's agree.
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7");
    sandbox.personal_policy(include_str!("../worker/typecheck/review-policy.ts"));
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    let report = inspect(&sandbox, "audit", &[]).report();
    assert_eq!(report["policy"]["version"], "typecheck-review-1");
    assert_eq!(report["selection"]["provider"], OTHER);
    assert_eq!(report["selection"]["model"], "model-b");
    assert_eq!(report["command"]["program"], "fake-harness");
    // The entry's other origin is reviewed by the builder.
    reviewing(&sandbox, "legacy-k3", &json!({ "declared": OTHER }));
    let report = inspect(&sandbox, "audit", &[]).report();
    assert_eq!(report["selection"]["provider"], OWN);
    assert_eq!(report["selection"]["model"], "model-a");
}

#[test]
fn the_review_example_ships_its_declarations_and_readable_source() {
    let sandbox = sandbox();
    let report = sandbox.inspect(&["--kind", "feature", "--json"]).report();
    let worker = std::path::PathBuf::from(report["worker"]["path"].as_str().unwrap());
    let examples = worker.parent().unwrap().join("examples");

    let shipped = fs::read_to_string(examples.join("review.ts")).unwrap();
    assert_eq!(shipped, include_str!("../worker/examples/review.ts"));
    let declarations = fs::read_to_string(examples.join("review.d.ts")).unwrap();
    for export in [
        "export type ReviewEntry =",
        "export interface ReviewRules",
        "export interface ReviewSelector",
        "export declare function reviewSelector(rules: ReviewRules): ReviewSelector;",
        "export declare function lookUpCreator",
        "export declare function otherAgent(effort: string): Route;",
        "export declare const reviews:",
        "export declare const routes:",
        "export declare const policy:",
    ] {
        assert!(
            declarations.contains(export),
            "review.d.ts lacks {export:?}"
        );
    }
    // It builds on the static example by that example's specifier, as an
    // owner's copy of it would.
    assert!(shipped.contains(r#"from "harness-dispatch/examples/static""#));
}

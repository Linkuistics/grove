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
//! the other's wrapper names an interpreter that does not exist. Each refusal
//! has a control beside it, the same review with the input corrected, that is
//! seen to select.

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
const WRAPPERS: [&str; 3] = [
    "my-agent-wrapper",
    "my-other-agent-wrapper",
    "my-gateway-wrapper",
];

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

/// A producer of kind `feature`, which the example routes to `careful`, of
/// origin `your-provider`, or with `choice` the candidate it names, run as
/// task `task_id`. Its run ID.
fn produce(sandbox: &Sandbox, task_id: &str, choice: Option<&str>) -> String {
    let mut args = vec![
        "--kind",
        "feature",
        "--task-id",
        task_id,
        "--prompt",
        "build it",
    ];
    if let Some(choice) = choice {
        args.extend(["--choice", choice]);
    }
    launched(sandbox, &args)
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
    let mut args = vec!["--kind", kind, "--context", "review.json", "--json"];
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

/// Write an owner's policy that builds on the example's exports: `body` is
/// TypeScript that may use `catalog`, `routes`, `reviews` and
/// `reviewSelector`, and must export `policy`.
fn owner_policy(sandbox: &Sandbox, body: &str) {
    sandbox.personal_policy(&format!(
        "import {{ catalog, routes, reviews, reviewSelector, lookUpCreator }} from \"harness-dispatch/examples/review\";\n{body}"
    ));
}

#[test]
fn a_review_selects_a_reviewer_of_another_origin_on_every_invocation() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7", None);
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(
        report["policy"]["version"],
        "harness-dispatch/examples/review 1"
    );
    let selection = &report["selection"];
    assert_eq!(selection["form"], "select", "{report}");
    assert_eq!(selection["candidateId"], "other-careful", "{report}");
    assert_eq!(selection["provider"], "your-other-provider");
    let reason = selection["reason"].as_str().unwrap();
    for part in [
        creator.as_str(),
        r#"origin "your-provider""#,
        r#"reviews["code-review"]["your-provider"] names "other-careful""#,
    ] {
        assert!(reason.contains(part), "{part:?} is not in {reason:?}");
    }
    assert_eq!(report["creator"]["evidence"], "execution_recorded");
    assert_eq!(report["creator"]["provider"], "your-provider");
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
        assert_eq!(launch["candidate"]["id"], "other-careful");
        assert_eq!(launch["creator"]["provider"], "your-provider");
        assert_eq!(launch["creator"]["evidence"], "execution_recorded");
        assert_eq!(launch["creator"]["reference"], json!({ "run": creator }));
        retries.push(run_id);
    }
    assert_ne!(retries[0], retries[1], "a retry is a run of its own");

    // The architecture review applies the same rule at its own effort.
    let report = inspect(&sandbox, "architecture-review", &[]).report();
    assert_eq!(report["selection"]["candidateId"], "other-deliberate");

    // A creator of the other origin is reviewed on the first.
    let other = produce(&sandbox, "feature-k8", Some("other-careful"));
    reviewing(&sandbox, "feature-k8", &json!({ "run": other }));
    for (kind, reviewer) in [
        ("code-review", "careful"),
        ("architecture-review", "deliberate"),
    ] {
        let report = inspect(&sandbox, kind, &[]).report();
        assert_eq!(report["selection"]["candidateId"], reviewer, "{kind}");
        assert_eq!(report["selection"]["provider"], "your-provider", "{kind}");
        assert_eq!(report["creator"]["provider"], "your-other-provider");
    }
    let (_, args) = reviewed(&sandbox, "code-review", &[]);
    assert_eq!(args, wrapper_args("your-model", "high", "review it"));
}

#[test]
fn an_explicit_choice_is_held_to_the_same_rule() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7", None);
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    // A choice of another origin is accepted, on every invocation.
    let report = inspect(&sandbox, "code-review", &["--choice", "other-deliberate"]).report();
    let selection = &report["selection"];
    assert_eq!(selection["candidateId"], "other-deliberate");
    assert_eq!(selection["explicitChoice"], "other-deliberate");
    let reason = selection["reason"].as_str().unwrap();
    assert!(
        reason.contains(r#"the explicit choice "other-deliberate""#),
        "{reason}"
    );
    for _ in 0..2 {
        let (_, args) = reviewed(&sandbox, "code-review", &["--choice", "other-deliberate"]);
        assert_eq!(args, wrapper_args("your-other-model", "xhigh", "review it"));
    }

    // A choice of the creator's own origin refuses, and so does the same
    // origin behind a gateway: another program and another model string, but
    // the model's origin is the creator's. Nothing is launched or recorded,
    // and nothing is chosen in its place.
    let before = runs_in(&sandbox);
    for choice in ["careful", "gateway-careful"] {
        for result in [
            inspect(&sandbox, "code-review", &["--choice", choice]),
            review(&sandbox, "code-review", &["--choice", choice]),
        ] {
            let refusal = refused(&sandbox, &result, "same_origin");
            assert_eq!(refusal["error"]["input"], format!("--choice {choice}"));
            let message = said(&refusal, "message");
            for part in [
                format!(r#""{choice}""#),
                r#"origin "your-provider""#.to_owned(),
                creator.clone(),
            ] {
                assert!(message.contains(&part), "{part:?}: {refusal}");
            }
            // The remedy offers the other origin's candidates, never the
            // gateway to the same one.
            let remedy = said(&refusal, "remedy");
            for offered in [r#""other-careful""#, r#""other-deliberate""#] {
                assert!(remedy.contains(offered), "{offered}: {refusal}");
            }
            assert!(!remedy.contains("gateway-careful"), "{refusal}");
        }
    }
    assert_eq!(runs_in(&sandbox), before, "a refused review was recorded");
}

#[test]
fn a_mapped_reviewer_of_the_creators_origin_refuses_and_is_never_replaced() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7", None);
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    // The control: the shipped entry maps this creator to the other origin.
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(report["selection"]["candidateId"], "other-careful");

    // An owner's entry that maps the creator's origin to that origin refuses,
    // although `other-careful` is still in the catalog.
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 1,
  version: "misfiled-1",
  catalog,
  ...reviewSelector({ catalog, routes, reviews: { "code-review": { "your-provider": "careful" } } }),
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
            message.contains(r#"reviews["code-review"]["your-provider"] names "careful""#),
            "{refusal}"
        );
        assert!(!message.contains("other-careful"), "{refusal}");
    }
}

#[test]
fn an_unknown_run_or_a_run_that_never_executed_refuses_with_the_declaration_remedy() {
    let sandbox = sandbox();
    let executed = produce(&sandbox, "feature-k7", None);

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
            // A choice of another origin is no way round it.
            inspect(&sandbox, "code-review", &["--choice", "other-careful"]),
        ] {
            let refusal = refused(&sandbox, &result, code);
            let message = said(&refusal, "message");
            for part in [creator, detail] {
                assert!(message.contains(part), "{part:?}: {refusal}");
            }
            // The declaration remedy, with the origins it may name.
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
    assert_eq!(report["selection"]["candidateId"], "other-careful");
}

#[test]
fn a_declared_creator_is_adopted_and_labelled_declared() {
    let sandbox = sandbox();
    for (declared, reviewer, program) in [
        ("your-other-provider", "careful", "my-agent-wrapper"),
        ("your-provider", "other-careful", "my-other-agent-wrapper"),
    ] {
        reviewing(&sandbox, "legacy-k3", &json!({ "declared": declared }));
        let report = inspect(&sandbox, "code-review", &[]).report();
        assert_eq!(report["selection"]["candidateId"], reviewer, "{declared}");
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
        assert_eq!(launch["executable"]["program"], program);

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
fn a_relabelled_origin_and_a_misspelt_declaration_refuse_as_non_members() {
    let sandbox = sandbox();

    // The producer ran under an earlier catalog, which labelled the origin
    // `Your-Provider`. The run records that label, and today's catalog has
    // relabelled it.
    sandbox.personal_policy(
        r#"export const policy = {
  schemaVersion: 1,
  version: "earlier-1",
  catalog: [
    { id: "careful", provider: "Your-Provider", model: "your-model", effort: "high", program: "my-agent-wrapper", args: ["--model", { slot: "model" }, "--effort", { slot: "effort" }, { slot: "prompt" }] },
  ],
  routes: { feature: "careful" },
};
"#,
    );
    let relabelled = produce(&sandbox, "feature-k7", None);
    sandbox.personal_policy(EXAMPLE);

    // Compared rather than checked, `Your-Provider` would differ from
    // `your-provider`, and the explicit choice of `careful` would pass as
    // another origin's review.
    reviewing(&sandbox, "feature-k7", &json!({ "run": relabelled }));
    for extra in [&[][..], &["--choice", "careful"][..]] {
        let refusal = refused(
            &sandbox,
            &inspect(&sandbox, "code-review", extra),
            "creator_origin_unknown",
        );
        let message = said(&refusal, "message");
        for part in [
            relabelled.as_str(),
            r#""Your-Provider""#,
            r#""your-provider""#,
            r#""your-other-provider""#,
        ] {
            assert!(message.contains(part), "{part:?}: {refusal}");
        }
        assert!(said(&refusal, "remedy").contains("restore"), "{refusal}");
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
        for extra in [&[][..], &["--choice", "careful"][..]] {
            let refusal = refused(
                &sandbox,
                &inspect(&sandbox, "code-review", extra),
                "creator_origin_unknown",
            );
            let message = said(&refusal, "message");
            assert!(
                message.contains(&format!("{misspelt:?}")),
                "{misspelt:?}: {refusal}"
            );
            let remedy = said(&refusal, "remedy");
            assert!(remedy.contains("correct"), "{refusal}");
            assert!(remedy.contains(r#""your-provider""#), "{refusal}");
        }
    }

    // The control: spelt exactly, the same declaration is a member, and
    // `careful` is a choice of another origin.
    reviewing(
        &sandbox,
        "legacy-k3",
        &json!({ "declared": "your-other-provider" }),
    );
    let report = inspect(&sandbox, "code-review", &["--choice", "careful"]).report();
    assert_eq!(report["selection"]["candidateId"], "careful");
}

#[test]
fn a_changed_current_mapping_does_not_change_the_creators_recorded_origin() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7", None);
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    let before = inspect(&sandbox, "code-review", &[]).report();

    // Since then the owner has pointed `careful` at the other harness, and
    // routes `feature` elsewhere. Read from today's catalog, the creator
    // would be `your-other-provider`, whose reviewer `careful` is now that
    // origin too, and the review would refuse.
    owner_policy(
        &sandbox,
        r#"const today = catalog.map((candidate) =>
  candidate.id === "careful"
    ? { ...candidate, provider: "your-other-provider", model: "your-other-model", program: "my-other-agent-wrapper" }
    : candidate,
);
export const policy = {
  schemaVersion: 1,
  version: "remapped-1",
  catalog: today,
  ...reviewSelector({ catalog: today, routes: { ...routes, feature: "other-careful" }, reviews }),
};
"#,
    );
    let after = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(after["policy"]["version"], "remapped-1");
    assert_eq!(after["creator"], before["creator"]);
    assert_eq!(after["creator"]["provider"], "your-provider");
    assert_eq!(
        after["creator"]["lookup"]["candidate"],
        json!({ "id": "careful", "provider": "your-provider", "model": "your-model", "effort": "high" })
    );
    assert_eq!(after["selection"]["candidateId"], "other-careful");
    assert_eq!(after["selection"]["provider"], "your-other-provider");

    // The control: today's mapping did reach the policy.
    let routed = sandbox.inspect(&["--kind", "feature", "--json"]).report();
    assert_eq!(routed["selection"]["candidateId"], "other-careful");
    let chosen = sandbox
        .inspect(&["--kind", "feature", "--choice", "careful", "--json"])
        .report();
    assert_eq!(chosen["selection"]["provider"], "your-other-provider");
}

#[test]
fn a_run_of_another_task_identity_is_admitted_and_shown() {
    let sandbox = sandbox();
    // A decomposed producer is finished by a child task with its own
    // identity, and a run need not have one at all.
    let child = produce(&sandbox, "child-k9", None);
    let anonymous = launched(&sandbox, &["--kind", "feature", "--prompt", "p"]);

    for (creator, task) in [(&child, "\"child-k9\""), (&anonymous, "no task")] {
        reviewing(&sandbox, "parent-k8", &json!({ "run": creator }));
        let report = inspect(&sandbox, "code-review", &[]).report();
        assert_eq!(report["selection"]["candidateId"], "other-careful");
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
    let creator = produce(&sandbox, "feature-k7", None);
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

    // An owner who routes it statically, without listing it, gets the route
    // only when no reviewed artifact is named.
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 1,
  version: "routed-1",
  catalog,
  ...reviewSelector({ catalog, routes: { ...routes, "security-review": "careful" }, reviews }),
};
"#,
    );
    let report = sandbox
        .inspect(&["--kind", "security-review", "--json"])
        .report();
    assert_eq!(report["selection"]["candidateId"], "careful");
    refused(
        &sandbox,
        &inspect(&sandbox, "security-review", &[]),
        "review_kind_unlisted",
    );

    // Listed, the rule applies to it.
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 1,
  version: "listed-1",
  catalog,
  ...reviewSelector({
    catalog,
    routes,
    reviews: { ...reviews, "security-review": { "your-provider": "other-deliberate", "your-other-provider": "deliberate" } },
  }),
};
"#,
    );
    let report = inspect(&sandbox, "security-review", &[]).report();
    assert_eq!(report["selection"]["candidateId"], "other-deliberate");
    refused(
        &sandbox,
        &inspect(&sandbox, "security-review", &["--choice", "careful"]),
        "same_origin",
    );
}

#[test]
fn a_review_without_its_creator_refuses_and_names_the_remedy() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7", None);

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
    for extra in [&[][..], &["--choice", "other-careful"][..]] {
        let refusal = refused(
            &sandbox,
            &inspect(&sandbox, "code-review", extra),
            "creator_missing",
        );
        let remedy = said(&refusal, "remedy");
        for part in [r#""run""#, r#""declared""#, r#""your-other-provider""#] {
            assert!(remedy.contains(part), "{part}: {refusal}");
        }
    }

    // A policy that selects with the example's `select` but never looks the
    // run up cannot know the creator's origin, and says so rather than
    // reading the run as missing.
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    owner_policy(
        &sandbox,
        r#"const { select } = reviewSelector({ catalog, routes, reviews });
export const policy = { schemaVersion: 1, version: "no-lookup-1", catalog, select };
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
        r#"const { select } = reviewSelector({ catalog, routes, reviews });
export const policy = {
  schemaVersion: 1,
  version: "composed-1",
  catalog,
  loadContext: (request, host) => lookUpCreator(request.context, host),
  select,
};
"#,
    );
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(report["selection"]["candidateId"], "other-careful");
}

#[test]
fn other_kinds_take_the_owners_static_routes() {
    let sandbox = sandbox();
    for (kind, candidate) in [
        ("question", "quick"),
        ("bugfix", "standard"),
        ("feature", "careful"),
        ("migration", "deliberate"),
        ("architecture", "deliberate"),
    ] {
        let report = sandbox.inspect(&["--kind", kind, "--json"]).report();
        assert_eq!(report["selection"]["candidateId"], candidate, "{kind}");
        assert_eq!(
            report["selection"]["reason"],
            format!(r#"routes["{kind}"] names candidate "{candidate}""#)
        );
    }
    // An explicit choice is taken, as a routes table takes it.
    let report = sandbox
        .inspect(&["--kind", "feature", "--choice", "other-careful", "--json"])
        .report();
    assert_eq!(report["selection"]["candidateId"], "other-careful");

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
        assert!(said(&refusal, "remedy").contains("--choice"), "{refusal}");
    }
    // And a review kind's entry lacks an origin the catalog has: an owner's
    // table that maps only one of them refuses the other.
    owner_policy(
        &sandbox,
        r#"export const policy = {
  schemaVersion: 1,
  version: "partial-1",
  catalog,
  ...reviewSelector({ catalog, routes, reviews: { "code-review": { "your-other-provider": "careful" } } }),
};
"#,
    );
    let creator = produce(&sandbox, "feature-k7", None);
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    let refusal = refused(
        &sandbox,
        &inspect(&sandbox, "code-review", &[]),
        "incomplete_mapping",
    );
    assert!(
        said(&refusal, "message").contains(r#"reviews["code-review"]"#),
        "{refusal}"
    );
    assert!(
        said(&refusal, "message").contains(r#""your-provider""#),
        "{refusal}"
    );
    // A choice of another origin supplies the reviewer the entry lacks.
    let report = inspect(&sandbox, "code-review", &["--choice", "other-careful"]).report();
    assert_eq!(report["selection"]["candidateId"], "other-careful");
}

#[test]
fn an_unreadable_store_refuses_before_the_selector_sees_it() {
    // The store cannot say whether the run exists, which is not the same as
    // its absence: harness-dispatch refuses with the store's own exit-4
    // refusal, and the selector never answers `creator_run_missing` or
    // selects. The control is the same review against the readable store.
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7", None);
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    let store = sandbox.default_store();
    let good = fs::read(&store).unwrap();
    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(report["selection"]["candidateId"], "other-careful");

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
    assert_eq!(report["selection"]["candidateId"], "other-careful");
}

#[test]
fn the_generic_form_selects_with_no_task_file_or_grove() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7", None);
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));
    assert!(
        !sandbox.cwd.join(".grove").exists() && !sandbox.bin.join("grove").exists(),
        "the sandbox holds no Grove"
    );

    let report = inspect(&sandbox, "code-review", &[]).report();
    assert_eq!(report["selection"]["candidateId"], "other-careful");
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
    assert_eq!(launch["creator"]["provider"], "your-provider");
}

#[test]
fn the_typed_review_fixture_selects_through_the_worker() {
    // `worker/typecheck/review-policy.ts` is type-checked against the shipped
    // declarations by `task dispatch:typecheck`; here the same file runs, so
    // the declared selector and the runtime's agree.
    let sandbox = sandbox();
    let creator = produce(&sandbox, "feature-k7", None);
    sandbox.personal_policy(include_str!("../worker/typecheck/review-policy.ts"));
    reviewing(&sandbox, "feature-k7", &json!({ "run": creator }));

    let report = inspect(&sandbox, "audit", &[]).report();
    assert_eq!(report["policy"]["version"], "typecheck-review-1");
    assert_eq!(report["selection"]["candidateId"], "auditor");
    refused(
        &sandbox,
        &inspect(&sandbox, "audit", &["--choice", "builder"]),
        "same_origin",
    );
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
        "export declare function reviewSelector",
        "export declare function lookUpCreator",
        "export declare const reviews:",
        "export declare const catalog:",
        "export declare const routes:",
        "export declare const policy:",
        "export type CandidateId =",
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

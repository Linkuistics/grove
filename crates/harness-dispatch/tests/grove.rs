//! The Grove adapter, `harness-dispatch/grove`, and the Grove review example
//! that composes it with the review rule, `harness-dispatch/examples/grove-review`,
//! through the command seam: the shipped example activated from a temporary
//! personal policy, Grove-shaped task files under a `.grove/` directory, and the
//! example's two wrappers as fake harnesses, with no Grove binary or
//! configuration (`docs/specs/harness-selection-and-execution.md`, *Supplied
//! review policy*, *Identity and original creator*, and the shipped-examples row
//! of *Agreed test seams and acceptance*).
//!
//! Every creator run is a real `run` of the front. The fake producer finishes as
//! the methodology says a dispatched session does: it writes its own
//! `HARNESS_DISPATCH_RUN_ID` into the review's `**Creator:**` line. Each refusal
//! has a control beside it that selects, with the task file put right.

mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use serde_json::{json, Value};
use sha2::{Digest as _, Sha256};
use support::{executable, run, Sandbox};

/// The shipped example, whole, as an owner's personal policy activates it.
const EXAMPLE: &str = "export { policy } from \"harness-dispatch/examples/grove-review\";\n";

/// The adapter's source, whose `version` inspection must report.
const ADAPTER: &str = include_str!("../worker/grove/index.ts");

/// The task-file fixtures, one per convention the adapter interprets.
const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/grove");

/// The producer's task file and the review's, as a Grove tree names them.
const PRODUCER_TASK: &str = ".grove/01-impl--parser-k12.md";
const REVIEW_TASK: &str = ".grove/02-review-impl--parser-k13.md";

/// A run ID no store in these tests holds.
const UNKNOWN: &str = "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e";

/// Each of the example's wrappers: a fake producer that, when
/// `PRODUCER_REVIEW` names a review's task file, finishes as the methodology
/// says a dispatched session does, writing `**Creator:** run <its run ID>`
/// directly under the review's `**Reviews:**` line in place of any
/// `**Creator:**` line there. Then it becomes the fake harness.
const WRAPPER: &str = r#"#!/bin/sh
if [ -n "$PRODUCER_REVIEW" ]; then
  awk -v run="$HARNESS_DISPATCH_RUN_ID" '
    /^\*\*Creator:\*\*/ { next }
    { print }
    /^\*\*Reviews:\*\* / { print "**Creator:** run " run }
  ' "$PRODUCER_REVIEW" > "$PRODUCER_REVIEW.new" && mv "$PRODUCER_REVIEW.new" "$PRODUCER_REVIEW" || exit 91
fi
exec fake-harness "$@"
"#;

/// A sandbox whose personal policy is the shipped Grove example, with an
/// ordinary producer's task file in place.
fn sandbox() -> Sandbox {
    let sandbox = Sandbox::new();
    for name in ["my-codex-wrapper", "my-claude-wrapper"] {
        executable(&sandbox.bin.join(name), WRAPPER);
    }
    sandbox.personal_policy(EXAMPLE);
    sandbox.file(PRODUCER_TASK, &fixture("plain-task"));
    sandbox
}

fn fixture(name: &str) -> String {
    fs::read_to_string(Path::new(FIXTURES).join(format!("{name}.md")))
        .unwrap_or_else(|error| panic!("fixture {name}: {error}"))
}

/// A review task whose `**Creator:**` line is `creator`, or which has none.
fn review_body(creator: Option<&str>) -> String {
    let creator = creator.map(|line| format!("{line}\n")).unwrap_or_default();
    format!("# parser-k13\n\n**Reviews:** parser-k12\n{creator}\n## Goal\n\nReview the parser.\n")
}

/// Launch the producer, `impl` as task `parser-k12`, which the example routes
/// to `lead-high`, of origin `openai`. With `review`, it names its run there.
/// Its run ID.
fn produce(sandbox: &Sandbox, review: Option<&Path>) -> String {
    let mut command = sandbox.command();
    command.args([
        "run",
        "--kind",
        "impl",
        "--task-file",
        PRODUCER_TASK,
        "--task-id",
        "parser-k12",
        "--prompt",
        "build it",
    ]);
    if let Some(review) = review {
        command.env("PRODUCER_REVIEW", review);
    }
    let result = run(&mut command);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let run_id = sandbox.harness_run_id();
    fs::remove_dir_all(&sandbox.record).unwrap();
    run_id
}

/// Inspect `kind` for `task_file`, as task `parser-k13`, plus `extra`.
fn inspect(sandbox: &Sandbox, kind: &str, task_file: &str, extra: &[&str]) -> support::Run {
    let mut args = vec![
        "--kind",
        kind,
        "--task-file",
        task_file,
        "--task-id",
        "parser-k13",
        "--json",
    ];
    args.extend(extra);
    sandbox.inspect(&args)
}

/// Run `kind` for `task_file`, as task `parser-k13`, plus `extra`.
fn review(sandbox: &Sandbox, kind: &str, task_file: &str, extra: &[&str]) -> support::Run {
    let mut args = vec![
        "--kind",
        kind,
        "--task-file",
        task_file,
        "--task-id",
        "parser-k13",
        "--prompt",
        "review it",
        "--json",
    ];
    args.extend(extra);
    sandbox.run(&args)
}

/// Run a review through to its harness: its run ID and the arguments the
/// harness received.
fn reviewed(sandbox: &Sandbox, kind: &str, task_file: &str) -> (String, Vec<String>) {
    let result = review(sandbox, kind, task_file, &[]);
    assert_eq!(result.code, Some(0), "{}", result.stderr);
    let args = sandbox.harness_args();
    let run_id = sandbox.harness_run_id();
    fs::remove_dir_all(&sandbox.record).unwrap();
    (run_id, args)
}

fn show(sandbox: &Sandbox, run_id: &str, json: bool) -> support::Run {
    let mut command = sandbox.command();
    command.args(["record", "show", "--run", run_id]);
    if json {
        command.arg("--json");
    }
    run(&mut command)
}

fn runs_in(sandbox: &Sandbox) -> i64 {
    Connection::open(sandbox.default_store())
        .unwrap()
        .query_row("SELECT count(*) FROM runs", [], |row| row.get(0))
        .unwrap()
}

/// The policy's own refusal, `policy_refused` with `code` beside it, exit 3,
/// launching nothing.
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

/// A refusal the adapter returned from `loadContext`: the context stage.
fn adapter_refused(sandbox: &Sandbox, result: &support::Run, code: &str) -> Value {
    let refusal = refused(sandbox, result, code);
    assert_eq!(refusal["error"]["stage"], "context", "{refusal}");
    assert_eq!(
        refusal["error"]["location"], "policy.loadContext",
        "{refusal}"
    );
    refusal
}

fn said<'a>(refusal: &'a Value, field: &str) -> &'a str {
    refusal["error"][field]
        .as_str()
        .unwrap_or_else(|| panic!("no {field}: {refusal}"))
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Which examples bring the Grove adapter, by stem, given each example's
/// source by its stem. An example brings it when it quotes
/// `harness-dispatch/grove`, or the specifier of an example that brings it,
/// in single or double quotes as an import names its module. A quote in a
/// comment counts too, so the reading errs only towards expecting a report,
/// which fails loudly, never towards the null a missing report gives. A
/// specifier built at run time, or in a template literal, is not seen.
fn adapter_bringers(sources: &BTreeMap<String, String>) -> BTreeSet<String> {
    let quotes = |source: &str, specifier: &str| {
        ['"', '\'']
            .iter()
            .any(|quote| source.contains(&format!("{quote}{specifier}{quote}")))
    };
    let mut bringers = BTreeSet::new();
    loop {
        let found: Vec<String> = sources
            .iter()
            .filter(|(stem, source)| {
                !bringers.contains(*stem)
                    && (quotes(source, "harness-dispatch/grove")
                        || bringers.iter().any(|bringer| {
                            quotes(source, &format!("harness-dispatch/examples/{bringer}"))
                        }))
            })
            .map(|(stem, _)| stem.clone())
            .collect();
        if found.is_empty() {
            return bringers;
        }
        bringers.extend(found);
    }
}

/// The adapter's own `version`, as its source declares it.
fn adapter_version() -> String {
    let line = ADAPTER
        .lines()
        .find(|line| line.starts_with("export const version = "))
        .expect("the adapter exports its version");
    line.trim_start_matches("export const version = ")
        .trim_end_matches(';')
        .trim_matches('"')
        .to_owned()
}

fn adapter_report() -> Value {
    json!({ "specifier": "harness-dispatch/grove", "version": adapter_version() })
}

/// The arguments the example's wrappers pass on for `model` at `effort`.
fn wrapper_args(model: &str, effort: &str, prompt: &str) -> Vec<String> {
    ["--model", model, "--effort", effort, prompt]
        .map(str::to_owned)
        .to_vec()
}

#[test]
fn a_dispatched_producer_names_its_run_and_its_review_uses_the_runs_recorded_provider() {
    let sandbox = sandbox();
    let review_task = sandbox.file(REVIEW_TASK, &review_body(None));
    let creator = produce(&sandbox, Some(&review_task));

    // The producer wrote its own run into the review, under `**Reviews:**`.
    let body = fs::read_to_string(&review_task).unwrap();
    assert!(
        body.contains(&format!(
            "**Reviews:** parser-k12\n**Creator:** run {creator}\n"
        )),
        "{body}"
    );
    let produced = show(&sandbox, &creator, true).report();
    assert_eq!(produced["launch"]["candidate"]["provider"], "openai");

    let before = inspect(&sandbox, "review-impl", REVIEW_TASK, &[]).report();
    let selection = &before["selection"];
    assert_eq!(selection["candidateId"], "review-high", "{before}");
    assert_eq!(selection["provider"], "anthropic");
    assert_eq!(
        before["reviewedArtifact"],
        json!({ "id": "parser-k12", "creator": { "run": creator } })
    );
    let expected_creator = &before["creator"];
    assert_eq!(expected_creator["reference"], json!({ "run": creator }));
    assert_eq!(expected_creator["evidence"], "execution_recorded");
    assert_eq!(expected_creator["provider"], "openai");
    assert_eq!(expected_creator["lookup"]["taskId"], "parser-k12");
    assert_eq!(before["adapter"], adapter_report());
    // The task file the reference came from, by digest, then the run it
    // names: nothing else was read.
    let task_file = sandbox.cwd.join(REVIEW_TASK);
    let digest = sha256(body.as_bytes());
    let sources = before["context"]["sources"].as_array().unwrap();
    assert_eq!(sources.len(), 2, "{before}");
    assert_eq!(
        sources[0],
        json!({ "name": support::text(&task_file), "via": "readText", "bytes": body.len(), "sha256": digest })
    );
    assert_eq!(sources[1]["name"], creator.as_str());
    assert_eq!(sources[1]["via"], "run");

    // The review's run record carries the same.
    let (run_id, args) = reviewed(&sandbox, "review-impl", REVIEW_TASK);
    assert_eq!(args, wrapper_args("your-claude-model", "high", "review it"));
    let launch = show(&sandbox, &run_id, true).report()["launch"].clone();
    assert_eq!(launch["adapter"], adapter_report());
    assert_eq!(launch["creator"], *expected_creator);
    assert_eq!(launch["reviewedArtifact"], before["reviewedArtifact"]);
    assert_eq!(launch["taskId"], "parser-k13");
    assert_eq!(launch["taskFile"], support::text(&task_file));
    assert_eq!(launch["context"]["sources"][0]["sha256"], digest.as_str());
    let text = show(&sandbox, &run_id, false);
    assert_eq!(text.code, Some(0), "{}", text.stderr);
    for part in [
        format!("adapter    harness-dispatch/grove {}", adapter_version()),
        format!("readText, {} bytes, sha256 {digest}", body.len()),
        "execution-recorded".to_owned(),
    ] {
        assert!(
            text.stdout.contains(&part),
            "{part:?} is not in:\n{}",
            text.stdout
        );
    }

    // Since then the owner has pointed `lead-high` at the other harness, and
    // routes `impl` elsewhere. Read from today's catalog, the creator would be
    // `anthropic`, whose reviewer `lead-high` is now `anthropic` too, and the
    // review would refuse.
    sandbox.personal_policy(
        r#"import { catalog, reviews, routes, groveReviewSelector } from "harness-dispatch/examples/grove-review";
const today = catalog.map((candidate) =>
  candidate.id === "lead-high"
    ? { ...candidate, provider: "anthropic", model: "your-claude-model", program: "my-claude-wrapper" }
    : candidate,
);
export const policy = {
  schemaVersion: 1,
  version: "remapped-1",
  catalog: today,
  ...groveReviewSelector({ catalog: today, routes: { ...routes, impl: "review-high" }, reviews }),
};
"#,
    );
    let after = inspect(&sandbox, "review-impl", REVIEW_TASK, &[]).report();
    assert_eq!(after["policy"]["version"], "remapped-1");
    assert_eq!(after["creator"], *expected_creator);
    assert_eq!(
        after["creator"]["lookup"]["candidate"],
        json!({ "id": "lead-high", "provider": "openai", "model": "your-codex-model", "effort": "high" })
    );
    assert_eq!(after["selection"]["candidateId"], "review-high");
    assert_eq!(after["selection"]["provider"], "anthropic");

    // The control: today's mapping did reach the policy.
    let routed = inspect(&sandbox, "impl", PRODUCER_TASK, &[]).report();
    assert_eq!(routed["selection"]["candidateId"], "review-high");
    let chosen = inspect(&sandbox, "impl", PRODUCER_TASK, &["--choice", "lead-high"]).report();
    assert_eq!(chosen["selection"]["provider"], "anthropic");
}

#[test]
fn an_earlier_run_of_the_same_task_does_not_stand_in_for_a_missing_creator_line() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, None);
    sandbox.file(REVIEW_TASK, &review_body(None));
    let lookup = show(&sandbox, &creator, true).report();
    assert_eq!(lookup["launch"]["taskId"], "parser-k12");

    for result in [
        inspect(&sandbox, "review-impl", REVIEW_TASK, &[]),
        review(&sandbox, "review-impl", REVIEW_TASK, &[]),
        review(
            &sandbox,
            "review-impl",
            REVIEW_TASK,
            &["--choice", "review-high"],
        ),
    ] {
        let refusal = adapter_refused(&sandbox, &result, "creator_line_missing");
        let message = said(&refusal, "message");
        assert!(message.contains("\"parser-k12\""), "{refusal}");
        assert!(
            message.contains(&support::text(&sandbox.cwd.join(REVIEW_TASK))),
            "{refusal}"
        );
        assert!(
            message.contains("No run is looked up by its task"),
            "{refusal}"
        );
        let remedy = said(&refusal, "remedy");
        for part in [
            "**Creator:** run <run ID>",
            "HARNESS_DISPATCH_RUN_ID",
            "**Creator:** declared <origin>",
        ] {
            assert!(remedy.contains(part), "{part}: {refusal}");
        }
    }
    // Only the producer's run was ever recorded.
    assert_eq!(runs_in(&sandbox), 1);

    // The control: the same review naming that run selects.
    sandbox.file(
        REVIEW_TASK,
        &review_body(Some(&format!("**Creator:** run {creator}"))),
    );
    let report = inspect(&sandbox, "review-impl", REVIEW_TASK, &[]).report();
    assert_eq!(report["selection"]["candidateId"], "review-high");
    assert_eq!(report["creator"]["lookup"]["taskId"], "parser-k12");
}

/// What a fixture comes to under a kind.
enum Expect {
    Selects(&'static str),
    Refuses(&'static str),
}

/// Each fixture under the kinds that exercise it. A creator run in them is the
/// producer's, of origin `openai`; a declaration names `anthropic`.
const FIXTURE_CASES: [(&str, &str, Expect); 21] = [
    ("review-run", "review-impl", Expect::Selects("review-high")),
    (
        "review-run",
        "review-design",
        Expect::Selects("review-xhigh"),
    ),
    (
        "review-declared",
        "review-impl",
        Expect::Selects("lead-high"),
    ),
    (
        "review-declared",
        "review-prototype",
        Expect::Selects("lead-medium"),
    ),
    // A review of a kind the example does not list cannot take its route.
    (
        "review-run",
        "impl",
        Expect::Refuses("review_kind_unlisted"),
    ),
    (
        "review-declared",
        "integrate-review-impl",
        Expect::Refuses("review_kind_unlisted"),
    ),
    (
        "creator-missing",
        "research-b",
        Expect::Refuses("review_kind_unlisted"),
    ),
    (
        "reviews-missing",
        "review-impl",
        Expect::Refuses("reviews_line_missing"),
    ),
    (
        "reviews-duplicate",
        "review-impl",
        Expect::Refuses("reviews_line_duplicate"),
    ),
    (
        "reviews-malformed",
        "review-impl",
        Expect::Refuses("reviews_line_malformed"),
    ),
    (
        "reviews-in-a-fence",
        "review-impl",
        Expect::Refuses("reviews_line_duplicate"),
    ),
    (
        "creator-missing",
        "review-impl",
        Expect::Refuses("creator_line_missing"),
    ),
    (
        "creator-duplicate",
        "review-impl",
        Expect::Refuses("creator_line_duplicate"),
    ),
    (
        "creator-malformed",
        "review-impl",
        Expect::Refuses("creator_line_malformed"),
    ),
    // Mentions that do not begin a line are prose, not markers.
    (
        "mentions-only",
        "review-impl",
        Expect::Refuses("reviews_line_missing"),
    ),
    ("mentions-only", "impl", Expect::Selects("lead-high")),
    // Outside a review, a `**Creator:**` line alone means nothing.
    ("reviews-missing", "impl", Expect::Selects("lead-high")),
    ("plain-task", "impl", Expect::Selects("lead-high")),
    ("plain-task", "design", Expect::Selects("lead-xhigh")),
    (
        "plain-task",
        "review-planning",
        Expect::Refuses("reviews_line_missing"),
    ),
    (
        "plain-task",
        "review-requirements",
        Expect::Refuses("reviews_line_missing"),
    ),
];

#[test]
fn each_grove_convention_the_adapter_interprets_has_a_fixture() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, None);

    // Every fixture is exercised, and every case names a fixture.
    let mut on_disk: Vec<String> = fs::read_dir(FIXTURES)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name != "README.md")
        .map(|name| {
            name.strip_suffix(".md")
                .expect("a markdown fixture")
                .to_owned()
        })
        .collect();
    on_disk.sort();
    let mut cased: Vec<String> = FIXTURE_CASES
        .iter()
        .map(|(name, ..)| (*name).to_owned())
        .collect();
    cased.sort();
    cased.dedup();
    assert_eq!(
        on_disk, cased,
        "a fixture without a case, or a case without a fixture"
    );

    for (name, kind, expect) in &FIXTURE_CASES {
        let relative = format!(".grove/{name}.md");
        let body = fixture(name).replace("@RUN@", &creator);
        let path = sandbox.file(&relative, &body);
        let result = inspect(&sandbox, kind, &relative, &[]);
        let case = format!("{name} under {kind}");
        match expect {
            Expect::Selects(candidate) => {
                let report = result.report();
                assert_eq!(
                    report["selection"]["candidateId"], *candidate,
                    "{case}: {report}"
                );
                assert_eq!(report["adapter"], adapter_report(), "{case}");
                let reviewed = &report["reviewedArtifact"];
                if kind.starts_with("review-") {
                    assert_eq!(reviewed["id"], "parser-k12", "{case}: {report}");
                } else {
                    assert_eq!(*reviewed, Value::Null, "{case}: {report}");
                }
                assert_eq!(
                    report["context"]["sources"][0]["name"],
                    support::text(&path),
                    "{case}: the task file is a measured source"
                );
            }
            Expect::Refuses(code) => {
                let refusal = adapter_refused(&sandbox, &result, code);
                assert!(
                    said(&refusal, "message").contains(&support::text(&path)),
                    "{case}: the refusal does not name the task file: {refusal}"
                );
            }
        }
    }
}

#[test]
fn malformed_marker_lines_refuse_naming_their_line() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, None);
    let body = |reviews: &str, creator: &str| {
        format!("# parser-k13\n\n{reviews}\n{creator}\n\n## Goal\n\nReview the parser.\n")
    };
    let good_reviews = "**Reviews:** parser-k12";
    let good_creator = format!("**Creator:** run {creator}");

    // The control: both lines well formed, with LF and with CRLF endings.
    for ending in ["\n", "\r\n"] {
        sandbox.file(
            REVIEW_TASK,
            &body(good_reviews, &good_creator).replace('\n', ending),
        );
        let report = inspect(&sandbox, "review-impl", REVIEW_TASK, &[]).report();
        assert_eq!(
            report["selection"]["candidateId"], "review-high",
            "{ending:?}"
        );
    }

    let path = support::text(&sandbox.cwd.join(REVIEW_TASK));
    for bad in [
        "**Reviews:**",
        "**Reviews:** ",
        "**Reviews:**parser-k12",
        "**Reviews:**  parser-k12",
        "**Reviews:**\tparser-k12",
        "**Reviews:** Parser-k12",
        "**Reviews:** parser-k012",
        "**Reviews:** parser-k0",
        "**Reviews:** parser",
        "**Reviews:** parser-k12 ",
        "**Reviews:** parser-k12, lexer-k11",
        "**Reviews:** -parser-k12",
        "**Reviews:** parser--lexer-k12",
        "**Reviews:** `parser-k12`",
    ] {
        sandbox.file(REVIEW_TASK, &body(bad, &good_creator));
        let refusal = adapter_refused(
            &sandbox,
            &inspect(&sandbox, "review-impl", REVIEW_TASK, &[]),
            "reviews_line_malformed",
        );
        let message = said(&refusal, "message");
        assert!(
            message.contains(&format!("line 3 of task file {path}")),
            "{bad:?}: {refusal}"
        );
        assert!(
            message.contains(&serde_json::to_string(bad).unwrap()),
            "{bad:?}: {refusal}"
        );
        assert!(said(&refusal, "remedy").contains("parser-k12"), "{refusal}");
    }

    let upper = creator.to_uppercase();
    for bad in [
        "**Creator:**".to_owned(),
        "**Creator:** ".to_owned(),
        format!("**Creator:**run {creator}"),
        "**Creator:** run".to_owned(),
        "**Creator:** run ".to_owned(),
        format!("**Creator:** run  {creator}"),
        format!("**Creator:** run {upper}"),
        format!("**Creator:** run {creator} "),
        format!("**Creator:** run {creator}x"),
        format!("**Creator:** runs {creator}"),
        format!("**Creator:** Run {creator}"),
        format!("**Creator:** {creator}"),
        "**Creator:** declared".to_owned(),
        "**Creator:** declared ".to_owned(),
        "**Creator:** declared   ".to_owned(),
    ] {
        sandbox.file(REVIEW_TASK, &body(good_reviews, &bad));
        let refusal = adapter_refused(
            &sandbox,
            &inspect(&sandbox, "review-impl", REVIEW_TASK, &[]),
            "creator_line_malformed",
        );
        assert!(
            said(&refusal, "message").contains(&format!("line 4 of task file {path}")),
            "{bad:?}: {refusal}"
        );
    }

    // A declared origin is the rest of the line, verbatim: nothing trims or
    // folds it, so a near miss is not an origin of the catalog.
    for (declared, control) in [
        ("declared anthropic", true),
        ("declared  anthropic", false),
        ("declared anthropic ", false),
        ("declared Anthropic", false),
    ] {
        sandbox.file(
            REVIEW_TASK,
            &body(good_reviews, &format!("**Creator:** {declared}")),
        );
        let result = inspect(&sandbox, "review-impl", REVIEW_TASK, &[]);
        if control {
            assert_eq!(result.report()["selection"]["candidateId"], "lead-high");
        } else {
            let refusal = refused(&sandbox, &result, "creator_origin_unknown");
            let origin = declared.strip_prefix("declared ").unwrap();
            assert!(
                said(&refusal, "message").contains(&serde_json::to_string(origin).unwrap()),
                "{declared:?}: {refusal}"
            );
        }
    }

    // A long line is quoted short, so the refusal stays readable and within
    // its bounds.
    let long = format!("**Reviews:** {}", "x".repeat(10_000));
    sandbox.file(REVIEW_TASK, &body(&long, &good_creator));
    let refusal = adapter_refused(
        &sandbox,
        &inspect(&sandbox, "review-impl", REVIEW_TASK, &[]),
        "reviews_line_malformed",
    );
    assert!(said(&refusal, "message").len() < 1_000, "{refusal}");
    assert!(said(&refusal, "message").contains('…'), "{refusal}");
}

#[test]
fn the_adapter_reads_only_the_task_file_it_is_given() {
    // A Grove tree around the task file, whose other files would change the
    // answer if they were read, and cannot be: each is unreadable. The task
    // file's name says another kind and another handle.
    let sandbox = sandbox();
    let creator = produce(&sandbox, None);
    let decoy = "# other-k1\n\n**Reviews:** lexer-k11\n**Creator:** declared openai\n";
    let hidden = [
        sandbox.file(".grove/_BRIEF.md", "# Brief\n\n**Reviews:** lexer-k11\n"),
        sandbox.file(".grove/03-k20/_topic.md", decoy),
        sandbox.file(".grove/03-k20/01-review-impl--other-k21.md", decoy),
        sandbox.file(".grove/04-review-impl--parser-k13.md", decoy),
    ];
    let misleading = ".grove/05-impl--other-k99.md";
    sandbox.file(
        misleading,
        &review_body(Some(&format!("**Creator:** run {creator}"))),
    );
    for path in &hidden {
        fs::set_permissions(path, fs::Permissions::from_mode(0o000)).unwrap();
    }

    for task_id in [&["--task-id", "parser-k13"][..], &[][..]] {
        let mut args = vec!["--kind", "review-impl", "--task-file", misleading, "--json"];
        args.extend(task_id);
        let report = sandbox.inspect(&args).report();
        // The kind is the caller's: a review, and review-impl's entry.
        assert_eq!(
            report["selection"]["candidateId"], "review-high",
            "{report}"
        );
        // The reviewed artifact is the line's, not the file name's or the task's.
        assert_eq!(report["reviewedArtifact"]["id"], "parser-k12");
        let sources: Vec<(&Value, &Value)> = report["context"]["sources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|source| (&source["name"], &source["via"]))
            .collect();
        assert_eq!(
            sources,
            [
                (
                    &json!(support::text(&sandbox.cwd.join(misleading))),
                    &json!("readText")
                ),
                (&json!(creator), &json!("run")),
            ]
        );
    }

    // Without a task file, an ordinary kind reads nothing and takes its route,
    // and a review cannot know its artifact.
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["selection"]["candidateId"], "lead-high");
    assert_eq!(report["context"]["sources"], json!([]));
    assert_eq!(report["adapter"], adapter_report());
    let refusal = adapter_refused(
        &sandbox,
        &sandbox.inspect(&["--kind", "review-impl", "--task-id", "parser-k13", "--json"]),
        "task_file_missing",
    );
    assert!(
        said(&refusal, "remedy").contains("--task-file"),
        "{refusal}"
    );

    for path in &hidden {
        fs::set_permissions(path, fs::Permissions::from_mode(0o644)).unwrap();
    }
}

#[test]
fn a_task_file_that_cannot_be_read_refuses_naming_it() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, None);
    let good = review_body(Some(&format!("**Creator:** run {creator}")));
    sandbox.file(REVIEW_TASK, &good);
    let unreadable = sandbox.file(".grove/03-review-impl--locked-k3.md", &good);
    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o000)).unwrap();
    support::write(&sandbox.cwd.join(".grove/04-review-impl--binary-k4.md"), "");
    fs::write(
        sandbox.cwd.join(".grove/04-review-impl--binary-k4.md"),
        [0xff, 0xfe, 0x0a],
    )
    .unwrap();

    for task_file in [
        ".grove/09-review-impl--missing-k9.md",
        ".grove/03-review-impl--locked-k3.md",
        ".grove/04-review-impl--binary-k4.md",
        ".grove",
    ] {
        // Every kind reads its task file, a review or not.
        for kind in ["review-impl", "impl"] {
            let refusal = inspect(&sandbox, kind, task_file, &[]).refusal(3);
            let context = format!("{task_file} under {kind}: {refusal}");
            assert_eq!(
                refusal["error"]["code"], "context_source_unreadable",
                "{context}"
            );
            assert_eq!(
                refusal["error"]["source"],
                support::text(&sandbox.cwd.join(task_file)),
                "{context}"
            );
            assert!(!sandbox.harness_ran(), "{context}");
        }
    }
    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o644)).unwrap();

    // A task file is read whole, past the 64 KiB a read takes by default, up
    // to the context budget; its text never enters the context.
    let large = format!("{good}\n{}\n", "Notes. ".repeat(15_000));
    assert!(large.len() > 100_000);
    sandbox.file(REVIEW_TASK, &large);
    let report = inspect(&sandbox, "review-impl", REVIEW_TASK, &[]).report();
    assert_eq!(report["selection"]["candidateId"], "review-high");
    assert_eq!(report["context"]["sources"][0]["bytes"], large.len());
    assert!(report["context"]["encodedBytes"].as_u64().unwrap() < 4_096);
    let refusal = inspect(
        &sandbox,
        "review-impl",
        REVIEW_TASK,
        &["--context-bytes", "50000"],
    )
    .refusal(3);
    assert_eq!(refusal["error"]["code"], "source_too_large", "{refusal}");
    // The read already takes the whole budget, so the budget's flag is the
    // remedy, never a larger read the owner cannot make.
    assert!(
        refusal["error"]["remedy"]
            .as_str()
            .unwrap()
            .starts_with("raise the context budget with --context-bytes"),
        "{refusal}"
    );
}

#[test]
fn a_caller_context_is_kept_but_cannot_name_a_second_reviewed_artifact() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, None);
    sandbox.file(
        REVIEW_TASK,
        &review_body(Some(&format!("**Creator:** run {creator}"))),
    );
    sandbox.file(
        "summary.json",
        &json!({ "schemaVersion": 1, "summary": "the parser" }).to_string(),
    );
    sandbox.file(
        "artifact.json",
        &json!({ "schemaVersion": 1, "reviewedArtifact": { "id": "lexer-k11", "creator": { "declared": "openai" } } })
            .to_string(),
    );

    // The caller's facts are kept, beside the task file's reviewed artifact.
    for (kind, task_file, reviewed) in [
        ("impl", PRODUCER_TASK, false),
        ("review-impl", REVIEW_TASK, true),
    ] {
        let report = inspect(&sandbox, kind, task_file, &["--context", "summary.json"]).report();
        let value = &report["context"]["value"];
        assert_eq!(value["summary"], "the parser", "{report}");
        assert_eq!(value["reviewedArtifact"].is_object(), reviewed, "{report}");
        assert_eq!(report["context"]["sources"][0]["via"], "--context");
    }

    // A review's artifact is the task file's to name, never the caller's too.
    let refusal = adapter_refused(
        &sandbox,
        &inspect(
            &sandbox,
            "review-impl",
            REVIEW_TASK,
            &["--context", "artifact.json"],
        ),
        "reviewed_artifact_conflict",
    );
    assert!(
        said(&refusal, "message").contains("\"lexer-k11\""),
        "{refusal}"
    );
    adapter_refused(
        &sandbox,
        &sandbox.inspect(&[
            "--kind",
            "review-impl",
            "--context",
            "artifact.json",
            "--json",
        ]),
        "task_file_missing",
    );
}

#[test]
fn the_rule_holds_on_every_invocation_retry_and_explicit_choice() {
    let sandbox = sandbox();
    let creator = produce(&sandbox, None);
    sandbox.file(
        REVIEW_TASK,
        &review_body(Some(&format!("**Creator:** run {creator}"))),
    );

    // Each run is a retry: a new run, the same reviewer.
    let (first, first_args) = reviewed(&sandbox, "review-impl", REVIEW_TASK);
    let (second, second_args) = reviewed(&sandbox, "review-impl", REVIEW_TASK);
    assert_ne!(first, second);
    assert_eq!(first_args, second_args);
    assert_eq!(
        first_args,
        wrapper_args("your-claude-model", "high", "review it")
    );

    // An explicit choice is held to the rule.
    let refusal = refused(
        &sandbox,
        &review(
            &sandbox,
            "review-impl",
            REVIEW_TASK,
            &["--choice", "lead-high"],
        ),
        "same_origin",
    );
    assert!(
        said(&refusal, "remedy").contains("review-high"),
        "{refusal}"
    );
    let report = inspect(
        &sandbox,
        "review-impl",
        REVIEW_TASK,
        &["--choice", "review-xhigh"],
    )
    .report();
    assert_eq!(report["selection"]["candidateId"], "review-xhigh");

    // A declared creator of the other origin reverses the answer.
    sandbox.file(
        REVIEW_TASK,
        &review_body(Some("**Creator:** declared anthropic")),
    );
    let report = inspect(&sandbox, "review-impl", REVIEW_TASK, &[]).report();
    assert_eq!(report["selection"]["candidateId"], "lead-high");
    assert_eq!(report["creator"]["evidence"], "declared");
    refused(
        &sandbox,
        &inspect(
            &sandbox,
            "review-impl",
            REVIEW_TASK,
            &["--choice", "review-high"],
        ),
        "same_origin",
    );

    // A run the store does not hold names no origin, and the remedy is the
    // declaration.
    sandbox.file(
        REVIEW_TASK,
        &review_body(Some(&format!("**Creator:** run {UNKNOWN}"))),
    );
    let refusal = refused(
        &sandbox,
        &review(&sandbox, "review-impl", REVIEW_TASK, &[]),
        "creator_run_missing",
    );
    assert!(
        said(&refusal, "remedy").contains("\"declared\""),
        "{refusal}"
    );
}

#[test]
fn other_grove_kinds_take_the_grove_static_routes() {
    let sandbox = sandbox();
    let grove_static = sandbox.root.join("grove-static.ts");
    support::write(
        &grove_static,
        "export { policy } from \"harness-dispatch/examples/grove-static\";\n",
    );
    for kind in [
        "requirements",
        "integrate-review-requirements",
        "design",
        "integrate-review-design",
        "planning",
        "integrate-review-planning",
        "prototype",
        "integrate-review-prototype",
        "impl",
        "integrate-review-impl",
        "research-a",
        "research-b",
        "combine-research",
        "draft",
        "copy-edit",
        "art",
        "proof",
        "finish",
    ] {
        let expected = sandbox
            .inspect(&[
                "--kind",
                kind,
                "--config",
                &support::text(&grove_static),
                "--json",
            ])
            .report();
        let report = inspect(&sandbox, kind, PRODUCER_TASK, &[]).report();
        assert_eq!(
            report["selection"]["candidateId"], expected["selection"]["candidateId"],
            "{kind}"
        );
        assert_eq!(
            report["selection"]["reason"],
            format!(
                "routes[\"{kind}\"] names candidate {}",
                expected["selection"]["candidateId"]
            ),
            "{kind}"
        );
    }
    // Grove's review kinds are the rule's, not routes: without their lines
    // they refuse rather than route.
    for kind in [
        "review-requirements",
        "review-design",
        "review-planning",
        "review-prototype",
        "review-impl",
    ] {
        adapter_refused(
            &sandbox,
            &inspect(&sandbox, kind, PRODUCER_TASK, &[]),
            "reviews_line_missing",
        );
    }
}

#[test]
fn the_adapter_version_is_reported_exactly_when_the_policy_imports_it() {
    let sandbox = Sandbox::new();
    let routed = |imports: &str, members: &str| {
        format!(
            "{imports}\nexport const policy = {{\n  schemaVersion: 1,\n  version: \"adapter-probe-1\",\n  \
             catalog: [{{ id: \"only\", provider: \"p\", model: \"m\", effort: \"e\", program: \"fake-harness\", args: [{{ slot: \"prompt\" }}] }}],\n{members}\n}};\n"
        )
    };
    let routes = "  routes: { impl: \"only\" },";

    // Each shipped example, imported for its effect alone: the adapter is
    // reported when, and only when, the example brings it, directly or
    // through another example. One that brings it without joining
    // `bringsAdapter` in worker/src/main.ts reports null, and fails here.
    let examples = Path::new(env!("CARGO_MANIFEST_DIR")).join("worker/examples");
    let mut sources = BTreeMap::new();
    for entry in fs::read_dir(&examples).unwrap() {
        let path = entry.unwrap().path();
        let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
        sources.insert(stem, fs::read_to_string(&path).unwrap());
    }
    let bringers = adapter_bringers(&sources);
    // The Grove example brings it directly, and an example that composes only
    // the Grove example brings it too.
    assert!(bringers.contains("grove-review"), "{bringers:?}");
    let mut composing = sources.clone();
    composing.insert(
        "mine".to_owned(),
        "export { groveReviewSelector } from \"harness-dispatch/examples/grove-review\";\n"
            .to_owned(),
    );
    assert!(adapter_bringers(&composing).contains("mine"));
    for (stem, source) in &sources {
        for relative in ["\"./", "\"../", "'./", "'../"] {
            assert!(
                !source.contains(relative),
                "{stem} quotes a relative path: the examples import the adapter and each other \
                 by specifier, as a policy does, which is what this test reads"
            );
        }
        sandbox.personal_policy(&routed(
            &format!("import \"harness-dispatch/examples/{stem}\";"),
            routes,
        ));
        let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
        let expected = if bringers.contains(stem) {
            adapter_report()
        } else {
            Value::Null
        };
        assert_eq!(
            report["adapter"], expected,
            "{stem}: an example that brings the adapter joins bringsAdapter in worker/src/main.ts"
        );
    }

    // The SDK alone brings no adapter; the adapter's own specifier does, at
    // load, in loadContext, or in select, and the run records it.
    for (imports, members, expected) in [
        ("import \"harness-dispatch/sdk\";", routes.to_owned(), Value::Null),
        ("import \"harness-dispatch/grove\";", routes.to_owned(), adapter_report()),
        (
            "",
            format!(
                "{routes}\n  async loadContext() {{ await import(\"harness-dispatch/grove\"); return {{ schemaVersion: 1 }}; }},"
            ),
            adapter_report(),
        ),
        (
            "",
            "  async select() { await import(\"harness-dispatch/grove\"); return { status: \"selected\", candidateId: \"only\", reason: \"r\" }; },"
                .to_owned(),
            adapter_report(),
        ),
        (
            "",
            "  select() { return { status: \"selected\", candidateId: \"only\", reason: \"r\" }; },"
                .to_owned(),
            Value::Null,
        ),
    ] {
        sandbox.personal_policy(&routed(imports, &members));
        let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
        assert_eq!(report["adapter"], expected, "{imports} {members}");
        let result = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
        assert_eq!(result.code, Some(0), "{}", result.stderr);
        let run_id = sandbox.harness_run_id();
        fs::remove_dir_all(&sandbox.record).unwrap();
        let launch = show(&sandbox, &run_id, true).report()["launch"].clone();
        assert_eq!(launch["adapter"], expected, "{imports} {members}");
    }
}

#[test]
fn the_typed_grove_fixture_selects_through_the_worker() {
    // `worker/typecheck/grove-policy.ts` is type-checked against the shipped
    // declarations by `task dispatch:typecheck`; here the same file runs, so
    // the declared adapter and example and the runtime's agree.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(include_str!("../worker/typecheck/grove-policy.ts"));
    sandbox.file(
        ".grove/02-audit--thing-k2.md",
        "# thing-k2\n\n**Reviews:** thing-k1\n**Creator:** declared your-provider\n",
    );
    let audit = |extra: &[&str]| {
        let mut args = vec![
            "--kind",
            "audit",
            "--task-file",
            ".grove/02-audit--thing-k2.md",
            "--json",
        ];
        args.extend(extra);
        sandbox.inspect(&args)
    };
    let report = audit(&[]).report();
    assert_eq!(
        report["policy"]["version"],
        format!("typecheck-grove-{}", adapter_version())
    );
    assert_eq!(report["selection"]["candidateId"], "auditor");
    refused(&sandbox, &audit(&["--choice", "builder"]), "same_origin");
}

#[test]
fn the_adapter_and_the_grove_example_ship_their_declarations_and_readable_sources() {
    let sandbox = sandbox();
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    let worker = PathBuf::from(report["worker"]["path"].as_str().unwrap());
    let libexec = worker.parent().unwrap();

    let adapter = fs::read_to_string(libexec.join("grove/index.ts")).unwrap();
    assert_eq!(adapter, ADAPTER);
    let declarations = fs::read_to_string(libexec.join("grove/index.d.ts")).unwrap();
    for export in [
        "export declare const version",
        "export declare function groveContext",
        "export type ReviewKinds",
    ] {
        assert!(
            declarations.contains(export),
            "grove/index.d.ts lacks {export:?}"
        );
    }
    // The adapter depends on the SDK alone.
    let imports: Vec<&str> = ADAPTER
        .lines()
        .filter(|line| line.starts_with("import "))
        .collect();
    assert_eq!(
        imports,
        ["import type { Context, ContextHost, Creator, Refused, SelectionRequest } from \"harness-dispatch/sdk\";"]
    );

    let example = fs::read_to_string(libexec.join("examples/grove-review.ts")).unwrap();
    assert_eq!(example, include_str!("../worker/examples/grove-review.ts"));
    let declarations = fs::read_to_string(libexec.join("examples/grove-review.d.ts")).unwrap();
    for export in [
        "export declare function groveReviewSelector",
        "export declare const reviews:",
        "export declare const catalog:",
        "export declare const routes:",
        "export declare const policy:",
        "export type CandidateId =",
    ] {
        assert!(
            declarations.contains(export),
            "grove-review.d.ts lacks {export:?}"
        );
    }
    // It composes the other shipped modules by their specifiers, as an
    // owner's copy of it would.
    for specifier in [
        "harness-dispatch/grove",
        "harness-dispatch/examples/review",
        "harness-dispatch/examples/grove-static",
    ] {
        assert!(
            example.contains(&format!("from \"{specifier}\"")),
            "{specifier}"
        );
    }
}

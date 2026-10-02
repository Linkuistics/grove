//! The context a selection sees, through the command seam: the caller's
//! `--context` document, a policy's `loadContext` and the SDK's measured reads,
//! and how inspection and the run record report them
//! (`docs/specs/harness-selection-and-execution.md`, *Bounded context*).
//!
//! Every refusal here is checked beside a control that selects, with the same
//! policy and the input put right, so that no refusal can pass because its
//! fixture never ran. The limits themselves are `bounds.rs`'s.

mod support;

use std::fs;
use std::path::Path;

use serde_json::{json, Value};
use sha2::{Digest as _, Sha256};
use support::{mkfifo, text, Sandbox};

/// The two commands a policy here selects between, each the fake harness with
/// the prompt as its one argument, under its own labels.
const COMMANDS: &str = r#"const harness = (provider, model, effort) => (request, reason) =>
  ({ status: "selected", program: "fake-harness", args: [request.prompt], provider, model, effort, reason });
const deep = harness("origin-a", "model-large", "high");
const quick = harness("origin-b", "model-small", "low");
"#;

/// A policy with `members`: a `select`, and a `loadContext` where the test
/// has one. `deep`, `quick` and `writeFileSync` are in scope for them.
fn policy(members: &str) -> String {
    format!(
        "import {{ writeFileSync }} from \"node:fs\";\n{COMMANDS}\
         export const policy = {{\n  schemaVersion: 2,\n  version: \"context-1\",\n{members}\n}};\n"
    )
}

/// A `select` that records the request and context it received at `path`,
/// then selects `quick`.
fn recording_select(path: &Path) -> String {
    format!(
        r#"  select(request, context) {{
    writeFileSync({:?}, JSON.stringify({{ request, context: context === undefined ? "undefined" : context }}));
    return quick(request, "recorded");
  }},"#,
        text(path)
    )
}

/// A `select` that selects `quick` and records nothing.
const SELECT: &str = r#"  select(request) { return quick(request, "selected"); },"#;

/// Every version-1 field, with an empty collection kept empty.
const DOCUMENT: &str = r#"{
  "schemaVersion": 1,
  "summary": "Rename the flag",
  "acceptanceCriteria": [],
  "facts": {},
  "assessments": { "risk": { "by": "owner", "value": "high" } },
  "sources": [{ "name": "issue 12", "version": "2026-09-30" }],
  "reviewedArtifact": { "id": "parser-k3", "creator": { "declared": "origin-a" } }
}
"#;

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The front's measure of a delivered context: its compact encoding, keys
/// sorted, which is serde_json's without `preserve_order`.
fn encoding(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).unwrap()
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("the policy recorded"))
        .expect("the policy recorded JSON")
}

#[test]
fn a_caller_context_reaches_the_policy_as_data_and_inspection_measures_it() {
    let sandbox = Sandbox::new();
    let seen = sandbox.root.join("seen.json");
    sandbox.personal_policy(&policy(&recording_select(&seen)));
    let path = sandbox.file("context/review.json", DOCUMENT);

    let report = sandbox
        .inspect(&[
            "--kind",
            "review",
            "--context",
            "context/review.json",
            "--json",
        ])
        .report();

    let document: Value = serde_json::from_str(DOCUMENT).unwrap();
    let measured = json!([{
        "name": text(&path), "via": "--context", "bytes": DOCUMENT.len(),
        "sha256": sha256(DOCUMENT.as_bytes()),
    }]);
    let mut delivered = document.clone();
    delivered["measured"] = measured.clone();
    assert_eq!(
        report["context"],
        json!({
            "loader": false,
            "sources": measured,
            "sourceBytes": DOCUMENT.len(),
            "encodedBytes": encoding(&delivered).len(),
            "sha256": sha256(&encoding(&delivered)),
            "value": delivered,
        })
    );
    assert_eq!(report["reviewedArtifact"], document["reviewedArtifact"]);

    // The request carries the document as data, exactly: the empty
    // collections stay empty, and nothing absent is filled in. `select`
    // receives the measured context.
    let seen_value = read_json(&seen);
    assert_eq!(seen_value["request"]["context"], document);
    assert_eq!(seen_value["context"], delivered);

    // A minimal document stays minimal: no facts is not an empty set of them.
    sandbox.file("context/minimal.json", r#"{"schemaVersion":1}"#);
    sandbox
        .inspect(&[
            "--kind",
            "review",
            "--context",
            "context/minimal.json",
            "--json",
        ])
        .report();
    let seen_value = read_json(&seen);
    assert_eq!(
        seen_value["request"]["context"],
        json!({ "schemaVersion": 1 })
    );
    let keys: Vec<&String> = seen_value["context"].as_object().unwrap().keys().collect();
    assert_eq!(keys, ["measured", "schemaVersion"]);

    // A `select` that reads no context is given the same one: what is
    // measured and reported does not depend on what the policy consults.
    sandbox.personal_policy(&policy(SELECT));
    let report = sandbox
        .inspect(&[
            "--kind",
            "review",
            "--context",
            "context/review.json",
            "--json",
        ])
        .report();
    assert_eq!(report["selection"]["reason"], "selected");
    assert_eq!(report["context"]["sources"], measured);

    // Text inspection names each measured source and the context digest.
    let human = sandbox.inspect(&["--kind", "review", "--context", "context/review.json"]);
    assert_eq!(human.code, Some(0), "{}", human.stderr);
    assert!(
        human.stdout.contains(&format!(
            "source [0] {} (--context, {} bytes",
            text(&path),
            DOCUMENT.len()
        )),
        "{}",
        human.stdout
    );
    assert!(human.stdout.contains("parser-k3"), "{}", human.stdout);
}

#[test]
fn run_records_the_reviewed_artifact_and_the_context_by_digest_and_size() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(SELECT));
    sandbox.file("review.json", DOCUMENT);
    let inputs = ["--kind", "review", "--context", "review.json", "--json"];

    let proposal = sandbox.inspect(&inputs).report();
    let run = sandbox.run(&[&inputs[..], &["--prompt", "p"]].concat());
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let run_id = sandbox.harness_run_id();

    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id, "--json"]);
    let export = support::run(&mut show).report();
    let launch = &export["launch"];
    let document: Value = serde_json::from_str(DOCUMENT).unwrap();
    assert_eq!(launch["reviewedArtifact"], document["reviewedArtifact"]);
    let mut recorded = proposal["context"].clone();
    recorded.as_object_mut().unwrap().remove("value");
    assert_eq!(launch["context"], recorded);
    assert_eq!(launch["bounds"], proposal["bounds"]);

    // Digests and sizes only: the delivered value is not stored.
    let store = fs::read(sandbox.default_store()).unwrap();
    let contains = |needle: &[u8]| store.windows(needle.len()).any(|window| window == needle);
    assert!(contains(run_id.as_bytes()), "the store is readable");
    assert!(
        !contains(b"Rename the flag"),
        "the context value was stored"
    );

    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id]);
    let human = support::run(&mut show);
    assert!(human.stdout.contains("parser-k3"), "{}", human.stdout);
    assert!(
        human.stdout.contains(&format!(
            "{} bytes encoded from 1 sources",
            recorded["encodedBytes"]
        )),
        "{}",
        human.stdout
    );
}

#[test]
fn each_invalid_caller_context_refuses_at_its_location_before_any_policy_runs() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync as mark }} from \"node:fs\";\nmark({:?}, \"ran\");\n{}",
        text(&sentinel),
        policy(SELECT)
    ));
    let run = |document: &str| {
        let _ = fs::remove_file(&sentinel);
        sandbox.file("context.json", document);
        sandbox.run(&[
            "--kind",
            "review",
            "--context",
            "context.json",
            "--prompt",
            "p",
            "--json",
        ])
    };

    for (case, document, code, location) in [
        ("not JSON", "{", "context_invalid", "context"),
        ("an array", "[]", "context_invalid", "context"),
        (
            "no version",
            "{}",
            "context_invalid",
            "context.schemaVersion",
        ),
        (
            "a later version",
            r#"{"schemaVersion":2,"summary":"s"}"#,
            "unsupported_version",
            "context.schemaVersion",
        ),
        (
            "an unknown field",
            r#"{"schemaVersion":1,"owner":"me"}"#,
            "context_invalid",
            "context.owner",
        ),
        (
            "an executable field",
            r#"{"schemaVersion":1,"program":"/bin/sh"}"#,
            "context_invalid",
            "context.program",
        ),
        (
            "an assessment without its assessor",
            r#"{"schemaVersion":1,"assessments":{"risk":{"value":"high"}}}"#,
            "context_invalid",
            "context.assessments[\"risk\"].by",
        ),
        (
            "a source record pinned by nothing",
            r#"{"schemaVersion":1,"sources":[{"name":"issue 12"}]}"#,
            "context_invalid",
            "context.sources[0]",
        ),
        (
            "two creator forms",
            r#"{"schemaVersion":1,"reviewedArtifact":{"id":"a","creator":{"run":"5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34","declared":"origin-a"}}}"#,
            "context_invalid",
            "context.reviewedArtifact.creator",
        ),
        (
            "a creator run that is no run ID",
            r#"{"schemaVersion":1,"reviewedArtifact":{"id":"a","creator":{"run":"producer-k3"}}}"#,
            "context_invalid",
            "context.reviewedArtifact.creator.run",
        ),
        (
            "a reviewed artifact without its ID",
            r#"{"schemaVersion":1,"reviewedArtifact":{"creator":{"declared":"origin-a"}}}"#,
            "context_invalid",
            "context.reviewedArtifact.id",
        ),
        (
            "the run lookups harness-dispatch attaches",
            r#"{"schemaVersion":1,"runs":[]}"#,
            "context_invalid",
            "context.runs",
        ),
        (
            "the worker's reserved marker",
            r#"{"schemaVersion":1,"facts":{"f":{"$harnessDispatch":"function"}}}"#,
            "context_invalid",
            "context.facts[\"f\"]",
        ),
    ] {
        let refusal = run(document).refusal(3);
        let context = format!("{case}: {refusal}");
        assert_eq!(refusal["error"]["code"], code, "{context}");
        assert_eq!(refusal["error"]["stage"], "context", "{context}");
        assert_eq!(refusal["error"]["location"], location, "{context}");
        assert_eq!(refusal["error"]["input"], "--context", "{context}");
        assert_eq!(
            refusal["error"]["source"],
            text(&sandbox.cwd.join("context.json")),
            "{context}"
        );
        assert!(!sandbox.harness_ran(), "{context}");
        assert!(!sentinel.exists(), "{context}: the policy ran");
        // Only a field named like an executable one says why it is refused.
        let message = refusal["error"]["message"].as_str().unwrap();
        assert_eq!(
            message.contains("a context is data"),
            case == "an executable field",
            "{context}"
        );
    }

    // The positive control: the whole document, valid, reaches the harness.
    let control = run(DOCUMENT);
    assert_eq!(control.code, Some(0), "{}", control.stderr);
    assert!(sandbox.harness_ran() && sentinel.exists());
}

#[test]
fn a_context_document_that_cannot_be_read_refuses_without_waiting_on_it() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(SELECT));
    fs::create_dir(sandbox.cwd.join("a-directory")).unwrap();
    mkfifo(&sandbox.cwd.join("a-fifo"));

    for (given, what) in [
        ("missing.json", "cannot be opened"),
        ("a-directory", "is not a regular file"),
        ("a-fifo", "is not a regular file"),
    ] {
        let refusal = sandbox
            .inspect(&["--kind", "review", "--context", given, "--json"])
            .refusal(3);
        assert_eq!(refusal["error"]["code"], "context_unreadable", "{given}");
        assert_eq!(refusal["error"]["input"], "--context", "{given}");
        assert!(
            refusal["error"]["message"].as_str().unwrap().contains(what),
            "{given}: {refusal}"
        );
    }
}

#[test]
fn a_loader_reads_measured_sources_against_the_callers_directory() {
    // The policy lives outside the caller's directory and imports a helper
    // beside itself, which resolves from the policy. Its reads name paths
    // relative to the caller's directory, which resolve there, although the
    // worker runs in `/`. Each read is measured under
    // its canonical name, so a symlink is named by its target.
    let sandbox = Sandbox::new();
    let loaded = sandbox.root.join("loaded.json");
    support::write(
        &sandbox.root.join("policies/helper.ts"),
        "export const floor = \"high\";\n",
    );
    support::write(
        &sandbox.root.join("policies/policy.ts"),
        &format!(
            "import {{ floor }} from \"./helper.ts\";\n{}",
            policy(&format!(
                r#"  loadContext(request, host) {{
    const risk = host.readJson("data/risk.json");
    const notes = host.readText("notes-link.md");
    writeFileSync({:?}, JSON.stringify({{ prompt: request.prompt, params: request.params, risk, notes }}));
    return {{
      schemaVersion: 1,
      summary: notes.text,
      assessments: {{ risk: {{ by: "data/risk.json", value: risk.value.level }} }},
      sources: [risk.source, notes.source],
    }};
  }},
  select(request, context) {{
    const risk = context.assessments.risk.value;
    return (risk === floor ? deep : quick)(request, `risk ${{risk}} against floor ${{floor}}`);
  }},"#,
                text(&loaded)
            ))
        ),
    );
    let risk = sandbox.file("data/risk.json", "{\"level\": \"high\"}\n");
    let notes = sandbox.file("docs/notes.md", "Rename the flag\n");
    std::os::unix::fs::symlink("docs/notes.md", sandbox.cwd.join("notes-link.md")).unwrap();

    let report = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--config",
            "../policies/policy.ts",
            "--prompt",
            "Rename the flag,\ncarefully\n",
            "--param",
            "ticket=12",
            "--json",
        ])
        .report();
    assert_eq!(report["selection"]["provider"], "origin-a");
    assert_eq!(
        report["selection"]["reason"],
        "risk high against floor high"
    );

    let record = |path: &Path, via: &str| {
        let bytes = fs::read(path).unwrap();
        json!({ "name": text(path), "via": via, "bytes": bytes.len(), "sha256": sha256(&bytes) })
    };
    let measured = json!([record(&risk, "readJson"), record(&notes, "readText")]);
    let context = &report["context"];
    assert_eq!(context["loader"], true);
    assert_eq!(context["sources"], measured);
    assert_eq!(
        context["sourceBytes"],
        fs::read(&risk).unwrap().len() + fs::read(&notes).unwrap().len()
    );
    assert_eq!(context["value"]["measured"], measured);
    assert_eq!(
        context["encodedBytes"],
        encoding(&context["value"]).len(),
        "the reported size is the delivered value's"
    );
    assert_eq!(context["sha256"], sha256(&encoding(&context["value"])));

    // Each read returned a source record the loader could attribute, the
    // measured one without its `via`, and the content itself.
    let seen = read_json(&loaded);
    assert_eq!(seen["notes"]["text"], "Rename the flag\n");
    assert_eq!(seen["risk"]["value"], json!({ "level": "high" }));
    let mut attributed = measured.clone();
    for entry in attributed.as_array_mut().unwrap() {
        entry.as_object_mut().unwrap().remove("via");
    }
    assert_eq!(context["value"]["sources"], attributed);

    // The loader receives the request `select` does: the prompt, byte for
    // byte, and every parameter.
    assert_eq!(seen["prompt"], "Rename the flag,\ncarefully\n");
    assert_eq!(seen["params"], json!({ "ticket": "12" }));

    // Under an inspection given no prompt, it receives the marker, and no
    // parameter where the caller passed none.
    let report = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--config",
            "../policies/policy.ts",
            "--json",
        ])
        .report();
    let seen = read_json(&loaded);
    assert_eq!(report["prompt"]["supplied"], false);
    assert_eq!(seen["prompt"], report["prompt"]["marker"]);
    assert_eq!(seen["params"], json!({}));
}

#[test]
fn the_typed_context_fixture_selects_through_the_worker() {
    // `worker/typecheck/context-policy.ts` is type-checked against the shipped
    // declarations; here the same file runs, with a --context document whose
    // own fields its loader keeps.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(include_str!("../worker/typecheck/context-policy.ts"));
    sandbox.file("risk.json", r#"{"level":"high"}"#);
    sandbox.file("notes.md", "  Rename the flag  \n");
    sandbox.file(
        "context.json",
        r#"{"schemaVersion":1,"facts":{"ticket":12}}"#,
    );

    let report = sandbox
        .inspect(&["--kind", "impl", "--context", "context.json", "--json"])
        .report();
    assert_eq!(report["selection"]["provider"], "origin-a");
    assert_eq!(
        report["selection"]["reason"],
        "kind impl, risk \"high\", measured --context:41,readJson:16,readText:20"
    );
    let value = &report["context"]["value"];
    assert_eq!(value["facts"], json!({ "ticket": 12 }));
    assert_eq!(value["summary"], "Rename the flag");
    assert_eq!(report["diagnostics"]["stderr"], "read 20 bytes of notes\n");
}

#[test]
fn a_missing_required_source_refuses_and_names_it() {
    // The loader requires `ticket.md`. Each way the read can fail refuses the
    // whole selection and names the source, including when the loader wraps
    // the error as the cause of its own. The control supplies the file.
    let sandbox = Sandbox::new();
    let ticket = sandbox.cwd.join("ticket.md");
    let loader = |read: &str| {
        policy(&format!(
            r#"  loadContext(request, host) {{
    try {{
      return {{ schemaVersion: 1, summary: {read} }};
    }} catch (error) {{
      throw new Error("a ticket is required", {{ cause: error }});
    }}
  }},
{SELECT}"#
        ))
    };
    let run = |read: &str| {
        sandbox.personal_policy(&loader(read));
        sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"])
    };

    for (case, prepare, read, what) in [
        (
            "missing",
            None,
            r#"host.readText("ticket.md").text"#,
            "cannot be opened",
        ),
        (
            "not UTF-8",
            Some(&b"\xff\xfe"[..]),
            r#"host.readText("ticket.md").text"#,
            "is not valid UTF-8",
        ),
        (
            "not JSON",
            Some(&b"# Ticket\n"[..]),
            r#"String(host.readJson("ticket.md").value)"#,
            "is not JSON",
        ),
    ] {
        let _ = fs::remove_file(&ticket);
        if let Some(bytes) = prepare {
            fs::write(&ticket, bytes).unwrap();
        }
        let refusal = run(read).refusal(3);
        let context = format!("{case}: {refusal}");
        assert_eq!(
            refusal["error"]["code"], "context_source_unreadable",
            "{context}"
        );
        assert_eq!(refusal["error"]["stage"], "context", "{context}");
        assert_eq!(refusal["error"]["source"], text(&ticket), "{context}");
        assert!(
            refusal["error"]["message"].as_str().unwrap().contains(what),
            "{context}"
        );
        assert!(!sandbox.harness_ran(), "{context}");
    }

    for (case, make) in [("a directory", "dir"), ("a FIFO", "fifo")] {
        let _ = fs::remove_file(&ticket);
        if make == "dir" {
            fs::create_dir(&ticket).unwrap();
        } else {
            mkfifo(&ticket);
        }
        let refusal = run(r#"host.readText("ticket.md").text"#).refusal(3);
        assert_eq!(
            refusal["error"]["code"], "context_source_unreadable",
            "{case}"
        );
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains("is not a regular file"),
            "{case}: {refusal}"
        );
        let _ = fs::remove_dir(&ticket);
        let _ = fs::remove_file(&ticket);
    }

    // The control: with the ticket present, the same loader selects.
    fs::write(&ticket, "Rename the flag\n").unwrap();
    let control = run(r#"host.readText("ticket.md").text"#);
    assert_eq!(control.code, Some(0), "{}", control.stderr);
    assert!(sandbox.harness_ran());
}

#[test]
fn a_failing_loader_refuses_and_launches_nothing() {
    let sandbox = Sandbox::new();
    for (case, loader, code, location) in [
        (
            "throws",
            r#"loadContext() { throw new TypeError("no ticket system"); },"#,
            "context_loader_failed",
            "policy.loadContext",
        ),
        (
            "rejects",
            r#"async loadContext() { throw new Error("no ticket system"); },"#,
            "context_loader_failed",
            "policy.loadContext",
        ),
        (
            "never settles",
            "loadContext() { return new Promise(() => {}); },",
            "context_loader_unsettled",
            "policy.loadContext",
        ),
        (
            "returns nothing",
            "loadContext() {},",
            "context_invalid",
            "context",
        ),
        (
            "returns an invalid shape",
            r#"loadContext() { return { schemaVersion: 1, summary: 3 }; },"#,
            "context_invalid",
            "context.summary",
        ),
        (
            "supplies its own measurements",
            r#"loadContext() { return { schemaVersion: 1, measured: [] }; },"#,
            "context_invalid",
            "context.measured",
        ),
        (
            "supplies its own run lookups",
            r#"loadContext() { return { schemaVersion: 1, runs: [] }; },"#,
            "context_invalid",
            "context.runs",
        ),
        (
            "delivers a function",
            r#"loadContext() { return { schemaVersion: 1, facts: { check: () => true } }; },"#,
            "context_invalid",
            "context.facts[\"check\"]",
        ),
        (
            "delivers a number JSON cannot",
            r#"loadContext() { return { schemaVersion: 1, facts: { ratio: 0 / 0 } }; },"#,
            "context_invalid",
            "context.facts[\"ratio\"]",
        ),
        (
            "cannot be serialized",
            r#"loadContext() { const facts = {}; facts.self = facts; return { schemaVersion: 1, facts }; },"#,
            "context_invalid",
            "context",
        ),
        (
            "mutates the frozen request",
            r#"loadContext(request) { request.kind = "design"; return { schemaVersion: 1 }; },"#,
            "context_loader_failed",
            "policy.loadContext",
        ),
    ] {
        sandbox.personal_policy(&policy(&format!("  {loader}\n{SELECT}")));
        let refusal = sandbox
            .run(&["--kind", "impl", "--prompt", "p", "--json"])
            .refusal(3);
        let context = format!("{case}: {refusal}");
        assert_eq!(refusal["error"]["code"], code, "{context}");
        assert_eq!(refusal["error"]["stage"], "context", "{context}");
        assert_eq!(refusal["error"]["location"], location, "{context}");
        assert!(!sandbox.harness_ran(), "{context}");
    }
    let message = |loader: &str| {
        sandbox.personal_policy(&policy(&format!("  {loader}\n{SELECT}")));
        sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3)["error"]["message"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    assert!(message(
        r#"loadContext() { return { schemaVersion: 1, facts: { check: () => true } }; },"#
    )
    .contains("a function cannot be carried in JSON"));
    assert!(
        message(r#"loadContext() { throw new TypeError("no ticket system"); },"#)
            .contains("TypeError: no ticket system")
    );
    assert!(
        message(r#"loadContext() { return { schemaVersion: 1, runs: [] }; },"#)
            .contains("`runs` holds the answers harness-dispatch gave to host.run")
    );

    // The control: the same policy, its loader returning a valid context.
    sandbox.personal_policy(&policy(&format!(
        "  loadContext() {{ return {{ schemaVersion: 1 }}; }},\n{SELECT}"
    )));
    let control = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(control.code, Some(0), "{}", control.stderr);
    assert!(sandbox.harness_ran());
}

#[test]
fn a_loader_can_refuse_as_the_policy_and_nothing_is_selected() {
    // A loader that finds what it requires invalid returns select's refusal
    // shape. It is the policy's own refusal, at the context stage, and select
    // is never asked.
    let sandbox = Sandbox::new();
    let asked = sandbox.root.join("select-asked");
    let refusing = r#"{ status: "refused", code: "ticket_closed", message: "ticket 12 is closed", remedy: "reopen ticket 12" }"#;
    for loader in [
        format!("  loadContext() {{ return {refusing}; }},"),
        format!("  async loadContext() {{ return {refusing}; }},"),
    ] {
        sandbox.personal_policy(&policy(&format!("{loader}\n{}", recording_select(&asked))));
        let refusal = sandbox
            .run(&["--kind", "impl", "--prompt", "p", "--json"])
            .refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], "policy_refused", "{refusal}");
        assert_eq!(error["policyCode"], "ticket_closed", "{refusal}");
        assert_eq!(error["stage"], "context", "{refusal}");
        assert_eq!(error["location"], "policy.loadContext", "{refusal}");
        assert_eq!(error["input"], "--kind impl", "{refusal}");
        assert_eq!(error["remedy"], "reopen ticket 12", "{refusal}");
        assert!(
            error["message"]
                .as_str()
                .unwrap()
                .ends_with("refused the selection in loadContext: ticket 12 is closed"),
            "{refusal}"
        );
        assert!(!sandbox.harness_ran(), "{refusal}");
        assert!(!asked.exists(), "select was asked: {refusal}");
        assert!(!sandbox.default_store().exists(), "{refusal}");
    }

    // A `status` makes a loader's result a refusal or nothing: each part of
    // the shape is required, and nothing else may sit beside it.
    for (result, location) in [
        (
            r#"{ status: "selected", program: "fake-harness", args: [], provider: "p", model: "m", effort: "e", reason: "r" }"#,
            "context.status",
        ),
        (
            r#"{ status: "refused", code: "", message: "m", remedy: "r" }"#,
            "context.code",
        ),
        (
            r#"{ status: "refused", code: "c", message: 3, remedy: "r" }"#,
            "context.message",
        ),
        (
            r#"{ status: "refused", code: "c", message: "m" }"#,
            "context.remedy",
        ),
        (
            r#"{ status: "refused", code: "c", message: "m", remedy: "r", program: "fake-harness" }"#,
            "context.program",
        ),
        (
            r#"{ schemaVersion: 1, status: "refused", code: "c", message: "m", remedy: "r" }"#,
            "context.schemaVersion",
        ),
    ] {
        sandbox.personal_policy(&policy(&format!(
            "  loadContext() {{ return {result}; }},\n{SELECT}"
        )));
        let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
        assert_eq!(refusal["error"]["code"], "context_invalid", "{refusal}");
        assert_eq!(refusal["error"]["location"], location, "{refusal}");
        assert!(
            refusal["error"]["remedy"]
                .as_str()
                .unwrap()
                .contains("{ status: \"refused\", code, message, remedy }"),
            "{refusal}"
        );
    }

    // Only a loader refuses so: a caller's document with a `status` is an
    // invalid context, whatever else it holds.
    sandbox.personal_policy(&policy(SELECT));
    sandbox.file(
        "refusal.json",
        r#"{ "status": "refused", "code": "c", "message": "m", "remedy": "r" }"#,
    );
    let refusal = sandbox
        .inspect(&["--kind", "impl", "--context", "refusal.json", "--json"])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "context_invalid", "{refusal}");
    assert_eq!(refusal["error"]["input"], "--context", "{refusal}");

    // A bound the loader exceeded is reported, even when it catches the
    // error and returns a refusal instead.
    sandbox.file("large.md", &"x".repeat(65_537));
    sandbox.personal_policy(&policy(&format!(
        r#"  loadContext(request, host) {{
    try {{ host.readText("large.md"); }} catch {{}}
    return {refusing};
  }},
{SELECT}"#
    )));
    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "source_too_large", "{refusal}");

    // The control: the same policy, its loader returning a context.
    sandbox.personal_policy(&policy(&format!(
        "  loadContext() {{ return {{ schemaVersion: 1 }}; }},\n{}",
        recording_select(&asked)
    )));
    let control = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(control.code, Some(0), "{}", control.stderr);
    assert!(sandbox.harness_ran() && asked.exists());
}

#[test]
fn select_reads_nothing_through_its_host_and_the_loaders_reads_close() {
    // `select` sees the measured context, so its host has no reads, and a
    // read the loader leaves for later fails once its context is delivered.
    // Neither is a bound: the policy catches each and selects, and says what
    // it caught.
    let sandbox = Sandbox::new();
    sandbox.file("notes.md", "n\n");
    sandbox.personal_policy(&policy(
        r#"  loadContext(request, host) {
    globalThis.laterRead = () => host.readText("notes.md");
    return { schemaVersion: 1 };
  },
  select(request, context, host) {
    const errors = [];
    try { host.readText("notes.md"); } catch (error) { errors.push(error.message); }
    try { globalThis.laterRead(); } catch (error) { errors.push(error.message); }
    return quick(request, errors.join(" | "));
  },"#,
    ));
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    let reason = report["selection"]["reason"].as_str().unwrap();
    assert!(
        reason.contains("host.readText is available in loadContext only"),
        "{reason}"
    );
    assert!(
        reason.contains("after loadContext returned its context"),
        "{reason}"
    );
    assert_eq!(report["context"]["sources"], json!([]));
}

#[test]
fn diagnostics_the_host_writes_stay_out_of_the_json_report() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(
        r#"  loadContext(request, host) {
    host.diagnostic("loading {\"forged\": true}");
    console.log("{\"schemaVersion\": 99}");
    return { schemaVersion: 1 };
  },
  select(request, context, host) {
    host.diagnostic("selecting\n");
    return quick(request, "r");
  },"#,
    ));
    let report = sandbox.inspect(&["--kind", "impl", "--json"]);
    let document = report.report();
    assert_eq!(report.stdout.matches('\n').count(), 1, "{}", report.stdout);
    assert_eq!(
        document["diagnostics"]["stderr"],
        "loading {\"forged\": true}\nselecting\n"
    );
    assert_eq!(
        document["diagnostics"]["stdout"],
        "{\"schemaVersion\": 99}\n"
    );

    let run = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let notice: Value = serde_json::from_str(run.stderr.trim_end()).unwrap();
    assert_eq!(
        notice["diagnostics"]["stderr"],
        "loading {\"forged\": true}\nselecting\n"
    );
}

#[test]
fn the_loader_runs_only_for_a_policy_the_front_accepted() {
    // The front validates the policy the worker loaded before it asks for a
    // context, so an invalid one refuses with its loader never called.
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("loader-ran");
    let loader = format!(
        "  loadContext() {{ writeFileSync({:?}, \"ran\"); return {{ schemaVersion: 1 }}; }},",
        text(&sentinel)
    );
    sandbox.personal_policy(&policy(&format!("{loader}\n  select: \"quick\",")));
    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_invalid", "{refusal}");
    assert_eq!(refusal["error"]["location"], "policy.select", "{refusal}");
    assert!(!sentinel.exists(), "the loader ran");
    // The control: the same loader, beside a `select`, runs.
    sandbox.personal_policy(&policy(&format!("{loader}\n{SELECT}")));
    sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert!(sentinel.exists());
}

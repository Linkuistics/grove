//! Selection through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Policy and the selected
//! command*): a policy's `select` receives the whole request, the prompt and
//! every parameter included, and returns the command to run, or refuses,
//! synchronously or through a promise. Every other value it produces refuses
//! with a code of its own and launches nothing.

mod support;

use std::collections::BTreeSet;
use std::fs;
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use support::{text, Sandbox};

/// What `select` receives as the prompt when `inspect` was given none.
const MARKER: &str = "<harness-dispatch inspect: no prompt was supplied>";

/// `quick`, which builds a selected result from the request, and `$SELECT`:
/// the `select` member, written as a method or property of the policy object,
/// which may call `writeFileSync`.
const TEMPLATE: &str = r#"import { writeFileSync } from "node:fs";

const quick = (request, reason) => ({
  status: "selected", program: "fake-harness", args: [request.prompt],
  provider: "origin-b", model: "model-small", effort: "low", reason,
});

export const policy = {
  schemaVersion: 2,
  version: "select-1",
  $SELECT
};
"#;

fn selecting(select: &str) -> String {
    TEMPLATE.replace("$SELECT", select)
}

/// A `select` that writes `marker` when it runs, then selects.
fn marking(marker: &std::path::Path) -> String {
    format!(
        "select(request) {{ writeFileSync({:?}, \"ran\"); return quick(request, \"marked\"); }},",
        text(marker)
    )
}

/// A JavaScript expression for the result most policies here return. It reads
/// `request`.
const QUICK: &str = r#"quick(request, "a small change needs little effort")"#;

#[test]
fn a_synchronous_or_asynchronous_select_returns_the_command_to_run() {
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
        assert_eq!(
            report["selection"],
            json!({
                "provider": "origin-b",
                "model": "model-small",
                "effort": "low",
                "reason": "a small change needs little effort",
            }),
            "{select}: {report}"
        );
        assert_eq!(
            report["command"],
            json!({
                "program": "fake-harness",
                "args": ["the prompt"],
                "executable": text(&sandbox.bin.join("fake-harness")),
            })
        );
        assert_eq!(report["policy"]["version"], "select-1");

        let human = sandbox.inspect(&["--kind", "impl"]);
        assert_eq!(human.code, Some(0), "{}", human.stderr);
        for row in [
            "provider   origin-b",
            "model      model-small",
            "effort     low",
            "reason     a small change needs little effort",
        ] {
            assert!(human.stdout.contains(row), "missing {row:?}:\n{}", human.stdout);
        }
    }
}

#[test]
fn run_launches_the_returned_command_and_records_its_labels_and_reason() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&selecting(&format!(
        "async select(request) {{ return {QUICK}; }},"
    )));

    let run = sandbox.run(&["--kind", "impl", "--prompt", "the prompt", "--json"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["the prompt"]);
    let notice: Value = serde_json::from_str(&run.stderr).unwrap();
    let handoff = &notice["handoff"];
    assert_eq!(handoff["provider"], "origin-b");
    assert_eq!(handoff["model"], "model-small");
    assert_eq!(handoff["effort"], "low");
    assert_eq!(handoff["reason"], "a small change needs little effort");
    assert_eq!(
        handoff["executable"],
        text(&sandbox.bin.join("fake-harness"))
    );

    let run_id = sandbox.harness_run_id();
    assert_eq!(handoff["runId"], run_id.as_str());
    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id, "--json"]);
    let export = support::run(&mut show).report();
    let launch = &export["launch"];
    assert_eq!(
        launch["selection"]["reason"],
        "a small change needs little effort"
    );
    assert_eq!(launch["candidate"]["provider"], "origin-b");
    assert_eq!(launch["candidate"]["model"], "model-small");
    assert_eq!(launch["candidate"]["effort"], "low");
    assert_eq!(launch["candidate"]["program"], "fake-harness");
    assert_eq!(launch["candidate"]["args"], json!(["the prompt"]));
    assert_eq!(launch["argv"], json!(["fake-harness", "the prompt"]));

    // Text mode names the labels, the kind and the run on one line.
    fs::remove_dir_all(&sandbox.record).unwrap();
    let run = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stderr.starts_with(
            "harness-dispatch: running provider origin-b, model model-small, effort low for \
             kind \"impl\" as run "
        ),
        "{}",
        run.stderr
    );
}

#[test]
fn select_is_called_as_a_method_with_the_whole_request_no_context_and_a_host() {
    // The policy records exactly what it was called with: the request, how
    // many arguments there were, whether `this` was the policy, and, with no
    // --context and no loader, an undefined context and a host without reads.
    let sandbox = Sandbox::new();
    let seen = sandbox.root.join("seen.json");
    sandbox.personal_policy(&selecting(&format!(
        "select(request, context, host) {{
    writeFileSync({:?}, JSON.stringify({{
      request, arity: arguments.length, version: this.version, context: typeof context,
      host: Object.keys(host).sort(),
      frozen: Object.isFrozen(request) && Object.isFrozen(request.limits) && Object.isFrozen(request.params),
    }}));
    return {QUICK};
  }},",
        text(&seen)
    )));
    // The prompt reaches `select` byte for byte: quotes, shell punctuation,
    // and its trailing newlines.
    let prompt = "Say \"hi\"; $HOME `x` 'q'\n\tthen stop — ünïcödé.\n\n";
    sandbox.file("mandate.md", prompt);

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
            "--param",
            "repo=/work/my repo",
            "--param",
            "session_name=parser: a=b\nsecond line",
            "--param",
            "empty=",
            "--param",
            "--flag=-x",
            "--json",
        ])
        .report();
    assert_eq!(report["command"]["args"], json!([prompt]));
    let raw = fs::read_to_string(&seen).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&raw).unwrap(),
        json!({
            "request": {
                "schemaVersion": 2,
                "kind": "impl",
                "prompt": prompt,
                "cwd": text(&sandbox.cwd),
                "params": {
                    "repo": "/work/my repo",
                    "session_name": "parser: a=b\nsecond line",
                    "empty": "",
                    "--flag": "-x",
                },
                "taskFile": text(&sandbox.cwd.join("tasks/t.md")),
                "taskId": "T-7",
                "limits": {
                    "selectionMs": 20_000, "contextBytes": 262_144, "sourceBytes": 65_536,
                    "sources": 256, "messageBytes": 1_048_576, "diagnosticsBytes": 262_144,
                },
            },
            "arity": 3,
            "version": "select-1",
            "context": "undefined",
            "host": ["diagnostic", "readJson", "readText", "run", "signal"],
            "frozen": true,
        })
    );

    // `run` hands `select` the same request, from the caller's own directory.
    let run = sandbox.run(&[
        "--kind",
        "impl",
        "--prompt",
        prompt,
        "--param",
        "repo=/work/my repo",
    ]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let ran: Value = serde_json::from_str(&fs::read_to_string(&seen).unwrap()).unwrap();
    assert_eq!(ran["request"]["prompt"], prompt);
    assert_eq!(ran["request"]["cwd"], text(&sandbox.cwd));
    assert_eq!(ran["request"]["params"], json!({ "repo": "/work/my repo" }));
    assert_eq!(sandbox.harness_args(), [prompt]);

    // Inputs the caller did not supply are absent, not empty; the parameters
    // are an empty object, the prompt is the marker, and the bound is the
    // default one.
    sandbox.inspect(&["--kind", "review", "--json"]).report();
    let seen: Value = serde_json::from_str(&fs::read_to_string(&seen).unwrap()).unwrap();
    assert_eq!(
        seen["request"],
        json!({
            "schemaVersion": 2,
            "kind": "review",
            "prompt": MARKER,
            "cwd": text(&sandbox.cwd),
            "params": {},
            "limits": {
                "selectionMs": 30_000, "contextBytes": 262_144, "sourceBytes": 65_536,
                "sources": 256, "messageBytes": 1_048_576, "diagnosticsBytes": 262_144,
            },
        })
    );
}

#[test]
fn a_refusal_from_select_is_reported_with_its_code_message_and_remedy() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(&selecting(
        r#"async select(request) {
    return { status: "refused", code: "no_reviewer", message: `nothing reviews kind ${request.kind}`, remedy: "declare the creator's provider" };
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
                .ends_with("refused the selection: nothing reviews kind review"),
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
    /// A selected result with `$EXTRA` spread over it, so that one field can
    /// be replaced, or removed by setting it to `undefined`.
    const WITH: &str = r#"select: (request) => ({ ...quick(request, "r"), $EXTRA }),"#;
    let with = |extra: &str| WITH.replace("$EXTRA", extra);
    let cases: Vec<(String, &str, &str, &str)> = vec![
        (
            r#"select() { throw new TypeError("no table"); },"#.to_owned(),
            "selection_threw",
            "policy.select",
            "TypeError: no table",
        ),
        (
            r#"async select() { await null; throw new RangeError("late"); },"#.to_owned(),
            "selection_threw",
            "policy.select",
            "RangeError: late",
        ),
        (
            r#"select: () => Promise.reject("a plain reason"),"#.to_owned(),
            "selection_threw",
            "policy.select",
            "a plain reason",
        ),
        (
            "select: () => new Promise(() => {}),".to_owned(),
            "selection_unsettled",
            "policy.select",
            "never settled",
        ),
        (
            "async select(request) { await new Promise(() => {}); return quick(request, \"r\"); },"
                .to_owned(),
            "selection_unsettled",
            "policy.select",
            "never settled",
        ),
        (
            "select() {},".to_owned(),
            "selection_abstained",
            "result",
            "no result",
        ),
        (
            "select: () => null,".to_owned(),
            "selection_abstained",
            "result",
            "no result",
        ),
        (
            "async select() {},".to_owned(),
            "selection_abstained",
            "result",
            "no result",
        ),
        (
            r#"select: () => "fake-harness","#.to_owned(),
            "selection_malformed",
            "result",
            "found a string",
        ),
        (
            r#"select: (request) => [quick(request, "r")],"#.to_owned(),
            "selection_malformed",
            "result",
            "found an array",
        ),
        (
            with(r#"status: undefined"#),
            "selection_malformed",
            "result.status",
            "`status` is missing",
        ),
        (
            with(r#"status: "ok""#),
            "selection_malformed",
            "result.status",
            "found \"ok\"",
        ),
        (
            with(r#"reason: "  ""#),
            "selection_malformed",
            "result.reason",
            "`reason` must not be blank",
        ),
        (
            with(r#"provider: undefined"#),
            "selection_malformed",
            "result.provider",
            "`provider` is missing",
        ),
        (
            with(r#"model: "\n""#),
            "selection_malformed",
            "result.model",
            "`model` must not be blank",
        ),
        (
            with(r#"effort: 3"#),
            "selection_malformed",
            "result.effort",
            "`effort` must be a string, found a number",
        ),
        (
            with(r#"program: undefined"#),
            "selection_malformed",
            "result.program",
            "`program` is missing",
        ),
        (
            with(r#"program: () => "fake-harness""#),
            "selection_malformed",
            "result.program",
            "found a function",
        ),
        (
            with(r#"program: "fake\0harness""#),
            "selection_malformed",
            "result.program",
            "a NUL character at byte 4",
        ),
        (
            with(r#"args: undefined"#),
            "selection_malformed",
            "result.args",
            "`args` is missing",
        ),
        (
            with(r#"args: "--yolo""#),
            "selection_malformed",
            "result.args",
            "`args` must be an array of strings, found a string",
        ),
        (
            with(r#"args: ["--effort", 7]"#),
            "selection_malformed",
            "result.args[1]",
            "one whole word, found a number",
        ),
        (
            with(r#"args: [{ slot: "prompt" }]"#),
            "selection_malformed",
            "result.args[0]",
            "one whole word, found an object",
        ),
        (
            with(r#"args: ["ok", "before\0after"]"#),
            "selection_malformed",
            "result.args[1]",
            "a NUL character at byte 6",
        ),
        (
            with(r#"candidateId: "quick""#),
            "selection_malformed",
            "result.candidateId",
            "unknown field `candidateId`",
        ),
        (
            with(r#"env: { TOKEN: "t" }"#),
            "selection_malformed",
            "result.env",
            "unknown field `env`",
        ),
        (
            r#"select: () => ({ status: "refused", code: "c", message: "m" }),"#.to_owned(),
            "selection_malformed",
            "result.remedy",
            "`remedy` is missing",
        ),
        (
            r#"select: () => ({ status: "refused", code: "c", message: "m", remedy: "r", program: "/bin/sh" }),"#
                .to_owned(),
            "selection_malformed",
            "result.program",
            "unknown field `program`",
        ),
        (
            r#"select(request) { const result = quick(request, "r"); result.self = result; return result; },"#
                .to_owned(),
            "selection_malformed",
            "result",
            "cannot be serialized",
        ),
    ];
    let mut codes = BTreeSet::new();
    for (select, code, location, found) in cases {
        let sandbox = Sandbox::new();
        let entry = sandbox.personal_policy(&selecting(&select));
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
        4,
        "exceptions, unsettled promises, abstention and malformed results each have their own \
         code: {codes:?}"
    );

    // The control: the same spread with nothing replaced selects.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&selecting(&with(r#"reason: "the control""#)));
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["selection"]["reason"], "the control");
}

#[test]
fn only_the_shape_of_a_selected_result_is_judged() {
    // No argument is required, an argument may be empty, and nothing checks
    // that the prompt is among them or that the labels appear in them.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&selecting(
        r#"select: () => ({
    status: "selected", program: "fake-harness", args: ["", " ", "--model=elsewhere"],
    provider: "origin-a", model: "model-large", effort: "high", reason: "the prompt is not passed",
  }),"#,
    ));
    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt", "unused", "--json"])
        .report();
    assert_eq!(
        report["command"]["args"],
        json!(["", " ", "--model=elsewhere"])
    );
    let run = sandbox.run(&["--kind", "impl", "--prompt", "unused"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["", " ", "--model=elsewhere"]);

    let sandbox = Sandbox::new();
    sandbox.personal_policy(&selecting(
        r#"select: () => ({
    status: "selected", program: "fake-harness", args: [],
    provider: "origin-a", model: "model-large", effort: "high", reason: "no arguments at all",
  }),"#,
    ));
    let run = sandbox.run(&["--kind", "any-kind-at-all", "--prompt", "unused"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(sandbox.harness_args().is_empty());
}

#[test]
fn nothing_reported_distinguishes_a_policy_that_consults_a_table_from_one_that_reads_the_prompt() {
    // Both return the same command for this request: one looks the kind up in
    // a table, the other decides from the prompt's text.
    const COMMAND: &str = r#"{ program: "fake-harness", args: ["--effort", "low", request.prompt], provider: "origin-b", model: "model-small", effort: "low" }"#;
    let sandbox = Sandbox::new();
    let table = sandbox.file(
        "table.ts",
        &format!(
            r#"const table = {{ impl: (request) => ({COMMAND}) }};
export const policy = {{
  schemaVersion: 2,
  version: "same-1",
  select: (request) => ({{ status: "selected", ...table[request.kind](request), reason: "a small change" }}),
}};
"#
        ),
    );
    let reading = sandbox.file(
        "reading.ts",
        &format!(
            r#"export const policy = {{
  schemaVersion: 2,
  version: "same-1",
  async select(request) {{
    const small = request.prompt.length < 100;
    await new Promise((resolve) => setTimeout(resolve, 5));
    return {{ status: "selected", ...({COMMAND}), reason: small ? "a small change" : "a large change" }};
  }},
}};
"#
        ),
    );

    // What names the entry itself, and how long evaluation took.
    let without = |mut document: Value, own: &[&str]| {
        let fields = document.as_object_mut().unwrap();
        for field in own {
            assert!(fields.remove(*field).is_some(), "no {field}");
        }
        document
    };
    let inspected = |entry: &std::path::Path| {
        let entry = text(entry);
        let report = sandbox
            .inspect(&[
                "--kind", "impl", "--config", &entry, "--prompt", "fix it", "--json",
            ])
            .report();
        let human = sandbox.inspect(&["--kind", "impl", "--config", &entry, "--prompt", "fix it"]);
        assert_eq!(human.code, Some(0), "{}", human.stderr);
        let rows: Vec<String> = human
            .stdout
            .lines()
            .filter(|line| {
                !["  policy ", "  authority ", "  sha256 ", "  timing "]
                    .iter()
                    .any(|own| line.starts_with(own))
            })
            .map(str::to_owned)
            .collect();
        (without(report, &["policy", "timing"]), rows)
    };
    assert_eq!(inspected(&table), inspected(&reading));

    let launched = |entry: &std::path::Path| {
        let sandbox = Sandbox::new();
        let copy = sandbox.file("policy.ts", &fs::read_to_string(entry).unwrap());
        let run = sandbox.run(&[
            "--kind",
            "impl",
            "--config",
            &text(&copy),
            "--prompt",
            "fix it",
            "--json",
        ]);
        assert_eq!(run.code, Some(0), "{}", run.stderr);
        let notice: Value = serde_json::from_str(&run.stderr).unwrap();
        let handoff = without(
            notice["handoff"].clone(),
            &["runId", "recordedAt", "stateDir", "executable"],
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
        let launch = without(
            export["launch"].clone(),
            &["policy", "timing", "cwd", "executable"],
        );
        (handoff, launch, sandbox.harness_args())
    };
    assert_eq!(launched(&table), launched(&reading));
}

#[test]
fn select_runs_only_for_a_policy_the_front_accepted() {
    // An invalid policy refuses from the snapshot the worker took at import:
    // `select` is never called for it.
    let sandbox = Sandbox::new();
    let marker = sandbox.root.join("select-ran");
    let valid = selecting(&marking(&marker));
    for (name, source, location) in [
        (
            "a blank version",
            valid.replace(r#"version: "select-1","#, r#"version: " ","#),
            "policy.version",
        ),
        (
            "a table beside select",
            valid.replace(
                "select(request) {",
                "routes: { impl: \"quick\" },\n  select(request) {",
            ),
            "policy.routes",
        ),
        (
            "a loader that is not a function",
            valid.replace(
                "select(request) {",
                "loadContext: \"notes.md\",\n  select(request) {",
            ),
            "policy.loadContext",
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
        "async select(request) {{ console.log(\"weighing the kind\"); console.error(\"no table entry\"); return {QUICK}; }},"
    )));

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(
        report["selection"]["reason"],
        "a small change needs little effort"
    );
    assert_eq!(report["diagnostics"]["stdout"], "weighing the kind\n");
    assert_eq!(report["diagnostics"]["stderr"], "no table entry\n");

    let run = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stderr
            .starts_with("policy stdout: weighing the kind\npolicy stderr: no table entry\n"),
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
        selecting(&format!("select: (request) => ({QUICK}),"))
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
        selecting(&format!("select: (request) => ({QUICK}),"))
    ));
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(
        report["selection"]["reason"],
        "a small change needs little effort"
    );
}

#[test]
fn the_type_checked_select_fixture_evaluates_as_its_types_describe() {
    // The fixture `task dispatch:typecheck` checks against the shipped
    // declarations, with `request` typed by `definePolicy` alone. It reads a
    // parameter, and compares the prompt with the SDK's `PROMPT_NOT_SUPPLIED`.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(include_str!("../worker/typecheck/select-policy.ts"));

    for (kind, provider, model, effort) in [
        ("design", "origin-a", "model-large", "high"),
        ("impl", "origin-b", "model-small", "low"),
    ] {
        // Without a prompt, the marker the front sends is the one the SDK
        // names, and it stands where the policy placed the prompt.
        let report = sandbox
            .inspect(&[
                "--kind",
                kind,
                "--param",
                "profile=parser work",
                "--timeout-ms",
                "25000",
                "--json",
            ])
            .report();
        assert_eq!(report["policy"]["version"], "typecheck-select-1");
        assert_eq!(
            report["selection"],
            json!({
                "provider": provider,
                "model": model,
                "effort": effort,
                "reason": format!("kind {kind} with no prompt within 25000 ms"),
            }),
            "{kind}"
        );
        assert_eq!(
            report["prompt"],
            json!({ "supplied": false, "marker": MARKER })
        );
        assert_eq!(
            report["command"]["args"],
            json!(["--profile", "parser work", format!("--kind={kind}"), MARKER])
        );
    }

    // Given a prompt, it selects from that prompt.
    let report = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--param",
            "profile=parser work",
            "--prompt",
            "héllo",
            "--json",
        ])
        .report();
    assert_eq!(
        report["selection"]["reason"],
        "kind impl with a 5-character prompt within 30000 ms"
    );
    assert_eq!(
        report["command"]["args"],
        json!(["--profile", "parser work", "--kind=impl", "héllo"])
    );

    // A parameter the caller did not pass is absent, and the policy says so.
    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused");
    assert_eq!(refusal["error"]["policyCode"], "profile_missing");
    assert_eq!(refusal["error"]["remedy"], "pass --param profile=NAME");
}

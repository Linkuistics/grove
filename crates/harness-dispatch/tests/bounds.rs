//! Each resource bound, through the command seam: the context budget, one
//! source read, the source count, the protocol message and the diagnostics
//! (`docs/specs/harness-selection-and-execution.md`, *Bounded context*). The
//! selection deadline is `deadline.rs`'s.
//!
//! Each bound is met exactly, and seen to select, beside the same fixture one
//! byte or one source past it, which refuses by name and launches nothing.
//! No bound is met by truncating, and none can be raised or swallowed by the
//! policy: a caught overflow still refuses.

mod support;

use std::fs;
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use support::{text, Run, Sandbox};

const CATALOG: &str = r#"[
    { id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness", args: [{ slot: "prompt" }] },
  ]"#;

const SELECT: &str =
    r#"  select() { return { status: "selected", candidateId: "deep", reason: "selected" }; },"#;

fn policy(members: &str) -> String {
    format!(
        "import {{ writeFileSync }} from \"node:fs\";\n\
         export const policy = {{\n  schemaVersion: 1,\n  version: \"bounds-1\",\n  catalog: {CATALOG},\n{members}\n}};\n"
    )
}

/// A policy whose loader returns a context with a summary of `n` `x`s.
fn summary_of(n: usize) -> String {
    policy(&format!(
        "  loadContext() {{ return {{ schemaVersion: 1, summary: \"x\".repeat({n}) }}; }},\n{SELECT}"
    ))
}

/// The summary length at which the delivered context, with no measured
/// source, encodes to exactly `bytes`.
fn summary_for(bytes: usize) -> usize {
    let empty = json!({ "measured": [], "schemaVersion": 1, "summary": "" });
    bytes - serde_json::to_vec(&empty).unwrap().len()
}

/// The refusal of a bound, after checking that it names the bound and that
/// nothing ran.
fn refused_by(run: &Run, sandbox: &Sandbox, code: &str, bound: Value) -> Value {
    let refusal = run.refusal(3);
    assert_eq!(refusal["error"]["code"], code, "{refusal}");
    let mut named = bound;
    named["name"] = refusal["error"]["bound"]["name"].clone();
    assert_eq!(refusal["error"]["bound"], named, "{refusal}");
    assert!(!sandbox.harness_ran(), "{code}: a harness ran");
    refusal
}

fn selected(run: &Run, sandbox: &Sandbox) {
    assert_eq!(
        run.code,
        Some(0),
        "expected a selection\nstdout: {}\nstderr: {}",
        run.stdout,
        run.stderr
    );
    assert!(sandbox.harness_ran(), "the harness never ran");
    fs::remove_dir_all(&sandbox.record).unwrap();
}

fn run(sandbox: &Sandbox, extra: &[&str]) -> Run {
    let mut args = vec!["--kind", "impl", "--prompt", "p", "--json"];
    args.extend_from_slice(extra);
    sandbox.run(&args)
}

#[test]
fn the_context_budget_holds_at_its_default_and_at_its_ceiling() {
    let sandbox = Sandbox::new();
    for (budget, flag) in [(262_144, None), (8_388_608, Some("8388608"))] {
        let extra: Vec<&str> = flag.map_or(vec![], |bytes| vec!["--context-bytes", bytes]);
        let from = if flag.is_some() {
            "--context-bytes"
        } else {
            "default"
        };

        sandbox.personal_policy(&summary_of(summary_for(budget)));
        let at = sandbox.inspect(&[&["--kind", "impl", "--json"][..], &extra].concat());
        assert_eq!(at.report()["context"]["encodedBytes"], budget, "{budget}");
        selected(&run(&sandbox, &extra), &sandbox);

        sandbox.personal_policy(&summary_of(summary_for(budget) + 1));
        let refusal = refused_by(
            &run(&sandbox, &extra),
            &sandbox,
            "context_too_large",
            json!({ "bytes": budget, "from": from }),
        );
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains(&format!("encodes to {} bytes", budget + 1)),
            "{refusal}"
        );
        assert_eq!(refusal["error"]["stage"], "context");
    }
}

#[test]
fn the_context_budget_counts_the_measured_sources_and_a_caller_document() {
    // A document's bytes fit, but its delivered context is the document plus
    // its measured record. Its encoding, reported by an inspection with room
    // to spare, is exactly the budget it needs.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(SELECT));
    sandbox.file(
        "context.json",
        r#"{"schemaVersion":1,"summary":"a context document"}"#,
    );
    let report = sandbox
        .inspect(&["--kind", "impl", "--context", "context.json", "--json"])
        .report();
    let needed = report["context"]["encodedBytes"]
        .as_u64()
        .unwrap()
        .to_string();
    let short = (report["context"]["encodedBytes"].as_u64().unwrap() - 1).to_string();

    selected(
        &run(
            &sandbox,
            &["--context", "context.json", "--context-bytes", &needed],
        ),
        &sandbox,
    );
    refused_by(
        &run(
            &sandbox,
            &["--context", "context.json", "--context-bytes", &short],
        ),
        &sandbox,
        "context_too_large",
        json!({ "bytes": short.parse::<u64>().unwrap(), "from": "--context-bytes" }),
    );

    // A document itself over the budget refuses before any policy runs.
    let document = format!(r#"{{"schemaVersion":1,"summary":"{}"}}"#, "x".repeat(300));
    sandbox.file("large.json", &document);
    let refusal = refused_by(
        &run(
            &sandbox,
            &["--context", "large.json", "--context-bytes", "200"],
        ),
        &sandbox,
        "context_too_large",
        json!({ "bytes": 200, "from": "--context-bytes" }),
    );
    assert_eq!(refusal["error"]["input"], "--context");
    assert!(refusal["error"]["message"]
        .as_str()
        .unwrap()
        .contains(&format!("is {} bytes", document.len())));
}

#[test]
fn an_oversize_prompt_does_not_count_against_the_context_budget() {
    // A prompt larger than the whole context budget, beside a context at
    // exactly that budget: the prompt has its own bound and never reaches the
    // worker, so the selection stands and the harness gets it whole.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&summary_of(summary_for(262_144)));
    let prompt = "p".repeat(400 * 1024);
    sandbox.file("mandate.md", &prompt);
    let run = sandbox.run(&["--kind", "impl", "--prompt-file", "mandate.md", "--json"]);
    selected_keeping(&run, &sandbox);
    assert_eq!(sandbox.harness_args(), [prompt]);
}

fn selected_keeping(run: &Run, sandbox: &Sandbox) {
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(sandbox.harness_ran());
}

#[test]
fn context_bytes_is_one_byte_to_eight_mebibytes() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync as mark }} from \"node:fs\";\nmark({:?}, \"ran\");\n{}",
        text(&sentinel),
        policy(SELECT)
    ));
    for malformed in ["0", "8388609", "1e3", "+1", "", "256KiB"] {
        let refusal = sandbox
            .inspect(&["--kind", "impl", "--context-bytes", malformed, "--json"])
            .refusal(2);
        assert_eq!(refusal["error"]["code"], "malformed_input", "{malformed:?}");
        assert_eq!(
            refusal["error"]["input"], "--context-bytes",
            "{malformed:?}"
        );
        assert!(!sentinel.exists(), "{malformed:?}: the policy ran");
    }
    for accepted in ["1", "8388608"] {
        let report = sandbox
            .inspect(&["--kind", "impl", "--context-bytes", accepted, "--json"])
            .report();
        assert_eq!(
            report["bounds"]["context"],
            json!({ "bytes": accepted.parse::<u64>().unwrap(), "from": "--context-bytes" })
        );
    }
}

#[test]
fn one_read_holds_to_its_limit_whatever_the_policy_does_with_the_error() {
    let sandbox = Sandbox::new();
    let read = |path: &str, max: &str, catch: bool| {
        let call = format!("host.readText({path:?}{max})");
        let body = if catch {
            format!("try {{ {call}; }} catch {{}} return {{ schemaVersion: 1 }};")
        } else {
            format!("{call}; return {{ schemaVersion: 1 }};")
        };
        sandbox.personal_policy(&policy(&format!(
            "  loadContext(request, host) {{ {body} }},\n{SELECT}"
        )));
    };
    sandbox.file("at.txt", &"a".repeat(65_536));
    sandbox.file("past.txt", &"a".repeat(65_537));
    sandbox.file("hundred.txt", &"a".repeat(100));
    sandbox.file("hundred-one.txt", &"a".repeat(101));
    let past = text(&sandbox.cwd.join("past.txt"));

    // The default, 64 KiB: at it, and one byte past it, caught or not.
    read("at.txt", "", false);
    selected(&run(&sandbox, &[]), &sandbox);
    for catch in [false, true] {
        read("past.txt", "", catch);
        let refusal = refused_by(
            &run(&sandbox, &[]),
            &sandbox,
            "source_too_large",
            json!({ "bytes": 65_536, "from": "default" }),
        );
        assert_eq!(refusal["error"]["source"], past, "caught: {catch}");
    }

    // A read's own maxBytes.
    read("hundred.txt", ", 100", false);
    selected(&run(&sandbox, &[]), &sandbox);
    read("hundred-one.txt", ", 100", true);
    refused_by(
        &run(&sandbox, &[]),
        &sandbox,
        "source_too_large",
        json!({ "bytes": 100, "from": "maxBytes" }),
    );

    // maxBytes may reach the context budget and never pass it, even for a
    // file that would fit.
    read("hundred.txt", ", 262144", false);
    selected(&run(&sandbox, &[]), &sandbox);
    read("hundred.txt", ", 262145", true);
    let refusal = refused_by(
        &run(&sandbox, &[]),
        &sandbox,
        "source_too_large",
        json!({ "bytes": 262_144, "from": "default" }),
    );
    assert!(refusal["error"]["message"]
        .as_str()
        .unwrap()
        .contains("maxBytes 262145, over the context budget"));

    // A budget below 64 KiB caps each read's default.
    sandbox.file("kib.txt", &"a".repeat(1024));
    sandbox.file("kib-one.txt", &"a".repeat(1025));
    read("kib.txt", "", false);
    selected(&run(&sandbox, &["--context-bytes", "1024"]), &sandbox);
    read("kib-one.txt", "", false);
    refused_by(
        &run(&sandbox, &["--context-bytes", "1024"]),
        &sandbox,
        "source_too_large",
        json!({ "bytes": 1024, "from": "--context-bytes" }),
    );
}

#[test]
fn a_read_that_already_takes_the_whole_budget_is_remedied_by_the_budget() {
    let sandbox = Sandbox::new();
    let read = |path: &str, max: &str| {
        sandbox.personal_policy(&policy(&format!(
            "  loadContext(request, host) {{ host.readText({path:?}{max}); return {{ schemaVersion: 1 }}; }},\n{SELECT}"
        )));
    };
    let said = |refusal: &Value, field: &str| refusal["error"][field].as_str().unwrap().to_owned();
    sandbox.file("hundred-one.txt", &"a".repeat(101));
    sandbox.file("budget-one.txt", &"a".repeat(262_145));
    sandbox.file("kib-one.txt", &"a".repeat(1025));
    sandbox.file("ceiling-one.txt", &"a".repeat(8_388_609));

    // Below the budget, a read can ask for more.
    read("hundred-one.txt", ", 100");
    let refusal = refused_by(
        &run(&sandbox, &[]),
        &sandbox,
        "source_too_large",
        json!({ "bytes": 100, "from": "maxBytes" }),
    );
    assert!(
        said(&refusal, "remedy").starts_with(
            "read it with a larger maxBytes, up to the context budget of 262144 bytes"
        ),
        "{refusal}"
    );

    // A read that already takes the budget can ask for no more: a maxBytes
    // that follows the budget, as the Grove adapter's does, and a default
    // read under a budget below 64 KiB. The budget's flag is the remedy.
    for (path, max, extra, bound, remedy) in [
        (
            "budget-one.txt",
            ", request.limits.contextBytes",
            &[][..],
            json!({ "bytes": 262_144, "from": "maxBytes" }),
            "raise the context budget with --context-bytes, up to 8388608: a read whose \
             maxBytes is request.limits.contextBytes takes the new budget",
        ),
        (
            "kib-one.txt",
            "",
            &["--context-bytes", "1024"][..],
            json!({ "bytes": 1024, "from": "--context-bytes" }),
            "raise the context budget with --context-bytes, up to 8388608: a read without \
             maxBytes takes the budget, up to 65536 bytes",
        ),
    ] {
        read(path, max);
        let refusal = refused_by(&run(&sandbox, extra), &sandbox, "source_too_large", bound);
        assert!(said(&refusal, "remedy").starts_with(remedy), "{refusal}");
        assert!(
            !said(&refusal, "remedy").contains("larger maxBytes"),
            "{refusal}"
        );
        assert!(
            said(&refusal, "message").ends_with(", the whole context budget"),
            "{refusal}"
        );
    }

    // At the budget's ceiling, only a smaller source.
    read("ceiling-one.txt", ", request.limits.contextBytes");
    let refusal = refused_by(
        &run(&sandbox, &["--context-bytes", "8388608"]),
        &sandbox,
        "source_too_large",
        json!({ "bytes": 8_388_608, "from": "maxBytes" }),
    );
    assert!(
        said(&refusal, "remedy").starts_with("read a smaller source"),
        "{refusal}"
    );
    assert!(
        !said(&refusal, "remedy").contains("--context-bytes"),
        "{refusal}"
    );
}

#[test]
fn a_context_holds_at_most_256_sources_the_document_included() {
    let sandbox = Sandbox::new();
    sandbox.file("s.txt", "s");
    sandbox.file("context.json", r#"{"schemaVersion":1}"#);
    let reads = |count: usize, catch: bool| {
        let read = if catch {
            r#"try { host.readText("s.txt"); } catch {}"#
        } else {
            r#"host.readText("s.txt");"#
        };
        sandbox.personal_policy(&policy(&format!(
            "  loadContext(request, host) {{ for (let i = 0; i < {count}; i++) {{ {read} }} return {{ schemaVersion: 1 }}; }},\n{SELECT}"
        )));
    };

    reads(256, false);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["context"]["sources"].as_array().unwrap().len(), 256);
    selected(&run(&sandbox, &[]), &sandbox);
    for catch in [false, true] {
        reads(257, catch);
        refused_by(
            &run(&sandbox, &[]),
            &sandbox,
            "too_many_sources",
            json!({ "sources": 256, "from": "fixed" }),
        );
    }

    reads(255, false);
    selected(&run(&sandbox, &["--context", "context.json"]), &sandbox);
    reads(256, false);
    refused_by(
        &run(&sandbox, &["--context", "context.json"]),
        &sandbox,
        "too_many_sources",
        json!({ "sources": 256, "from": "fixed" }),
    );
}

/// A routes policy whose catalog snapshot, as the worker frames it
/// (`{"type":"policy","policy":…,"adapter":null}`), is exactly `bytes` long.
/// It pads its version to fit, measuring the frame as the worker encodes it.
fn snapshot_of(bytes: usize) -> String {
    format!(
        "const policy = {{ schemaVersion: 1, version: \"v\", catalog: {CATALOG}, routes: {{ impl: \"deep\" }} }};\n\
         const framed = () => Buffer.byteLength(JSON.stringify({{ type: \"policy\", policy, adapter: null }}));\n\
         policy.version = \"v\" + \"x\".repeat({bytes} - framed());\n\
         export {{ policy }};\n"
    )
}

/// A select whose result, as the worker frames it
/// (`{"type":"selection","result":…,"adapter":null}`), is exactly `bytes` long.
fn result_of(bytes: usize) -> String {
    policy(&format!(
        "  select() {{\n    const result = {{ status: \"selected\", candidateId: \"deep\", reason: \"r\" }};\n    \
         const framed = () => Buffer.byteLength(JSON.stringify({{ type: \"selection\", result, adapter: null }}));\n    \
         result.reason = \"r\" + \"x\".repeat({bytes} - framed());\n    return result;\n  }},"
    ))
}

#[test]
fn a_catalog_snapshot_or_result_holds_to_the_message_bound() {
    const MIB: usize = 1_048_576;
    let sandbox = Sandbox::new();
    for (make, stage) in [
        (snapshot_of as fn(usize) -> String, "load"),
        (result_of, "selection"),
    ] {
        sandbox.personal_policy(&make(MIB));
        selected(&run(&sandbox, &[]), &sandbox);
        sandbox.personal_policy(&make(MIB + 1));
        let refusal = refused_by(
            &run(&sandbox, &[]),
            &sandbox,
            "message_too_large",
            json!({ "bytes": MIB, "from": "fixed" }),
        );
        assert_eq!(refusal["error"]["stage"], stage, "{refusal}");
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains(&format!("{} bytes", MIB + 1)),
            "{refusal}"
        );
    }
}

/// A policy that writes `stdout` and `stderr` bytes to its two streams at
/// import, synchronously, so nothing is left in a buffer at exit.
fn printing(stdout: usize, stderr: usize) -> String {
    format!(
        "import {{ writeSync }} from \"node:fs\";\n\
         writeSync(1, \"o\".repeat({stdout}));\nwriteSync(2, \"e\".repeat({stderr}));\n{}",
        policy(SELECT)
    )
}

#[test]
fn diagnostics_hold_to_their_bound_across_both_streams() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&printing(131_072, 131_072));
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(
        report["diagnostics"]["stdout"].as_str().unwrap().len(),
        131_072
    );
    assert_eq!(
        report["diagnostics"]["stderr"].as_str().unwrap().len(),
        131_072
    );
    selected(&run(&sandbox, &[]), &sandbox);

    for (stdout, stderr) in [(131_073, 131_072), (131_072, 131_073)] {
        sandbox.personal_policy(&printing(stdout, stderr));
        let refusal = refused_by(
            &run(&sandbox, &[]),
            &sandbox,
            "output_limit",
            json!({ "bytes": 262_144, "from": "fixed" }),
        );
        assert_eq!(refusal["error"]["stage"], "evaluation");
        let kept = refusal["diagnostics"]["stdout"].as_str().unwrap().len()
            + refusal["diagnostics"]["stderr"].as_str().unwrap().len();
        assert_eq!(kept, 262_144, "the first 262144 bytes are kept");
    }
}

#[test]
fn a_policy_that_prints_without_end_is_stopped_for_its_output_not_its_time() {
    // The front keeps reading past the bound, so the worker never blocks on a
    // full pipe, and stops it as soon as it sees the bound passed: long before
    // the selection deadline, and with the output-limit error.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&format!(
        "import {{ writeSync }} from \"node:fs\";\n\
         const line = \"spam \".repeat(1000) + \"\\n\";\nfor (;;) writeSync(1, line);\n{}",
        policy(SELECT)
    ));
    let started = Instant::now();
    let run = run(&sandbox, &["--timeout-ms", "60000"]);
    let elapsed = started.elapsed();
    refused_by(
        &run,
        &sandbox,
        "output_limit",
        json!({ "bytes": 262_144, "from": "fixed" }),
    );
    assert!(elapsed < Duration::from_secs(20), "took {elapsed:?}");
}

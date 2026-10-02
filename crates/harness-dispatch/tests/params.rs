//! `--param` through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Command interface*,
//! *Policy and the selected command*): every parameter reaches `select` by
//! name with its value exact and means nothing to the command, inspection and
//! the run record report it as given, and a repeated or malformed one refuses
//! before any policy runs.

mod support;

use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt as _;
use std::path::Path;

use serde_json::{json, Value};
use support::{text, write, Sandbox};

/// Runs the fake harness with the parameters `select` received, as JSON, and
/// then the prompt.
const ECHO: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "params-1",
  select(request) {
    return { status: "selected", program: "fake-harness", args: [JSON.stringify(request.params), request.prompt], provider: "origin-a", model: "model-large", effort: "high", reason: `${Object.keys(request.params).length} parameters for kind ${request.kind}` };
  },
};
"#;

/// Selects by the caller's `tier` parameter, which it builds into an argument
/// and a label, and refuses a caller that passed none.
const TIERED: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "params-2",
  select(request) {
    const tier = request.params.tier;
    if (tier === undefined) {
      return { status: "refused", code: "tier_missing", message: "no tier parameter was passed", remedy: "pass --param tier=NAME" };
    }
    return { status: "selected", program: "fake-harness", args: [`--tier=${tier}`, request.prompt], provider: `origin-${tier}`, model: "model-large", effort: "high", reason: `the caller's tier ${tier}` };
  },
};
"#;

/// Parameters a shell, a flag parser or a careless split would each misread:
/// an empty value, a value holding `=`, spaces, quotes and newlines, a leading
/// hyphen on the name and on the value, names an input already has, and text
/// outside ASCII.
const AWKWARD: [(&str, &str); 10] = [
    ("empty", ""),
    ("equals", "a=b==c="),
    ("spaced", " two  words "),
    ("lines", "first\n\nsecond\t$(date) `x` 'q' \"d\"\n"),
    ("-n", "-v"),
    ("flag", "--json"),
    ("kind", "design"),
    ("prompt", "not the prompt"),
    ("unicode", "café ✓ 日本語"),
    ("名前", "値"),
];

/// `AWKWARD` as `--param NAME=VALUE` words, and as the object `select` receives.
fn awkward() -> (Vec<String>, Value) {
    let mut words = Vec::new();
    let mut params = serde_json::Map::new();
    for (name, value) in AWKWARD {
        words.push("--param".to_owned());
        words.push(format!("{name}={value}"));
        params.insert(name.to_owned(), value.into());
    }
    (words, Value::Object(params))
}

#[test]
fn every_parameter_reaches_select_by_name_with_its_value_exact() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ECHO);
    let (words, params) = awkward();
    let words: Vec<&str> = words.iter().map(String::as_str).collect();

    let run = sandbox.run(&[&["--kind", "impl", "--prompt", "the prompt"], &words[..]].concat());
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let args = sandbox.harness_args();
    let received: Value = serde_json::from_str(&args[0]).unwrap();
    assert_eq!(received, params);
    // A parameter named like an input is still only a parameter, and the word
    // `--json` as a value chose no output format.
    assert_eq!(args[1], "the prompt");
    assert!(
        run.stderr
            .starts_with("harness-dispatch: running provider origin-a")
            && run.stderr.contains("for kind \"impl\""),
        "{}",
        run.stderr
    );

    // Inspection gives `select` the same parameters.
    let report = sandbox
        .inspect(&[&["--kind", "impl", "--json"], &words[..]].concat())
        .report();
    let received: Value =
        serde_json::from_str(report["command"]["args"][0].as_str().unwrap()).unwrap();
    assert_eq!(received, params);
    assert_eq!(report["kind"], "impl");
    assert_eq!(report["selection"]["reason"], "10 parameters for kind impl");
}

#[test]
fn a_caller_that_passes_no_parameter_gives_select_an_empty_object() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ECHO);

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["command"]["args"][0], "{}");
    assert_eq!(report["params"], json!({}));

    let run = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["{}", "p"]);
}

#[test]
fn a_repeated_or_malformed_parameter_refuses_before_any_policy_runs() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "await Bun.write({:?}, \"ran\");\n{ECHO}",
        text(&sentinel)
    ));
    let os = |words: &[&str]| words.iter().map(OsString::from).collect::<Vec<_>>();
    let cases = [
        (
            "repeated",
            os(&["--param", "a=1", "--param", "b=2", "--param", "a=3"]),
            "--param \"a\" is given more than once",
        ),
        (
            "repeated with the same value",
            os(&["--param", "a=1", "--param", "a=1"]),
            "--param \"a\" is given more than once",
        ),
        (
            "no =",
            os(&["--param", "novalue"]),
            "--param \"novalue\" has no =, so it names no value",
        ),
        (
            "empty",
            os(&["--param", ""]),
            "--param \"\" has no =, so it names no value",
        ),
        (
            "no name",
            os(&["--param", "=value"]),
            "--param \"=value\" has no name before its =",
        ),
        (
            "not UTF-8",
            vec![
                "--param".into(),
                OsString::from_vec(b"name=caf\xe9".to_vec()),
            ],
            "--param is not valid UTF-8",
        ),
    ];
    for (name, words, message) in cases {
        for command in ["run", "inspect"] {
            let mut invocation = sandbox.command();
            invocation
                .args([command, "--kind", "impl", "--prompt", "p", "--json"])
                .args(&words);
            let refusal = support::run(&mut invocation).refusal(2);
            let error = &refusal["error"];
            assert_eq!(error["code"], "malformed_input", "{name} {command}");
            assert_eq!(error["stage"], "cli", "{name} {command}");
            assert_eq!(error["input"], "--param", "{name} {command}");
            assert_eq!(error["message"], message, "{name} {command}");
            assert!(
                error["remedy"]
                    .as_str()
                    .unwrap()
                    .contains("--param NAME=VALUE"),
                "{name} {command}: {refusal}"
            );
        }
    }
    // A word that only looks like a flag is the parameter, and is refused as
    // one, in the format the command line asked for.
    let human = sandbox.inspect(&["--kind", "impl", "--param", "--json"]);
    assert_eq!(human.code, Some(2), "{}", human.stderr);
    assert!(
        human.stderr.starts_with(
            "harness-dispatch: refused (malformed_input, stage cli): --param \"--json\" has no =",
        ),
        "{}",
        human.stderr
    );

    assert!(
        !sentinel.exists(),
        "the policy ran for a malformed parameter"
    );
    assert!(!sandbox.harness_ran());
    assert!(
        !sandbox.home.join(".local").exists(),
        "a malformed parameter created the record directory"
    );

    // The control: well-formed parameters do evaluate the policy.
    sandbox
        .inspect(&[
            "--kind", "impl", "--param", "a=1", "--param", "b=1", "--json",
        ])
        .report();
    assert!(sentinel.exists(), "the sentinel policy never ran");
}

#[test]
fn the_parameters_names_and_values_together_are_bounded_at_64_kib() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ECHO);
    let limit = 64 * 1024;
    let inspect = |params: &[String]| {
        let mut command = sandbox.command();
        command.args(["inspect", "--kind", "impl", "--json"]);
        for param in params {
            command.args(["--param", param]);
        }
        support::run(&mut command)
    };
    let refused = |params: &[String], bytes: usize| {
        let refusal = inspect(params).refusal(2);
        let error = &refusal["error"];
        assert_eq!(error["code"], "malformed_input", "{refusal}");
        assert_eq!(error["stage"], "cli", "{refusal}");
        assert_eq!(error["input"], "--param", "{refusal}");
        assert_eq!(
            error["message"],
            format!(
                "the parameters' names and values are {bytes} bytes together, over the \
                 {limit}-byte limit"
            ),
            "{refusal}"
        );
    };

    // One byte over, in one parameter: its name counts, and its `=` does not.
    refused(&[format!("k={}", "v".repeat(limit))], limit + 1);
    // Each within the bound, and over it together.
    refused(
        &[
            format!("a={}", "v".repeat(limit / 2)),
            format!("b={}", "v".repeat(limit / 2 - 1)),
        ],
        limit + 1,
    );
    // Bytes, not characters: each `é` is two.
    refused(&[format!("k={}", "é".repeat(limit / 2))], limit + 1);

    // The positive control: exactly the bound is accepted and reaches `select`
    // whole.
    let at_limit = "v".repeat(limit / 2 - 1);
    let report = inspect(&[format!("a={at_limit}"), format!("b={at_limit}")]).report();
    assert_eq!(report["params"], json!({ "a": at_limit, "b": at_limit }));
    let received: Value =
        serde_json::from_str(report["command"]["args"][0].as_str().unwrap()).unwrap();
    assert_eq!(received, report["params"]);
}

#[test]
fn inspection_reports_the_parameters_in_both_forms() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ECHO);
    let args = [
        "--kind",
        "impl",
        "--param",
        "session_name=repo: a grove",
        "--param",
        "repo=/work/repo",
        "--param",
        "empty=",
        "--param",
        "lines=one\ntwo",
    ];

    let report = sandbox.inspect(&[&args[..], &["--json"]].concat()).report();
    assert_eq!(
        report["params"],
        json!({
            "session_name": "repo: a grove",
            "repo": "/work/repo",
            "empty": "",
            "lines": "one\ntwo",
        })
    );

    // One row, by name: a parameter with a space or a control character is
    // quoted and escaped, so that two never run together.
    let human = sandbox.inspect(&args);
    assert_eq!(human.code, Some(0), "{}", human.stderr);
    assert!(
        human.stdout.contains(
            "  params     empty= \"lines=one\\ntwo\" repo=/work/repo \"session_name=repo: a grove\"\n"
        ),
        "{}",
        human.stdout
    );

    let none = sandbox.inspect(&["--kind", "impl"]);
    assert!(
        none.stdout.contains("  params     none\n"),
        "{}",
        none.stdout
    );
}

#[test]
fn run_records_the_parameters_it_was_given() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(TIERED);
    let show = |run_id: &str, json: bool| {
        let mut show = sandbox.command();
        show.args(["record", "show", "--run", run_id]);
        if json {
            show.arg("--json");
        }
        support::run(&mut show)
    };

    // The policy places one parameter in the argv, and the record holds every
    // one the caller passed.
    let run = sandbox.run(&[
        "--kind",
        "impl",
        "--prompt",
        "the prompt",
        "--param",
        "tier=deep",
        "--param",
        "repo=/work/repo",
    ]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["--tier=deep", "the prompt"]);
    let run_id = sandbox.harness_run_id();
    let export = show(&run_id, true).report();
    let launch = &export["launch"];
    assert_eq!(
        launch["params"],
        json!({ "tier": "deep", "repo": "/work/repo" })
    );
    assert_eq!(launch["candidate"]["provider"], "origin-deep");
    assert_eq!(
        launch["argv"],
        json!(["fake-harness", "--tier=deep", "the prompt"])
    );
    let human = show(&run_id, false);
    assert!(
        human
            .stdout
            .contains("  params     repo=/work/repo tier=deep\n"),
        "{}",
        human.stdout
    );

    // A run given only the parameter its policy reads records only that one.
    std::fs::remove_dir_all(&sandbox.record).unwrap();
    let run = sandbox.run(&["--kind", "impl", "--prompt", "p", "--param", "tier=quick"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let export = show(&sandbox.harness_run_id(), true).report();
    assert_eq!(export["launch"]["params"], json!({ "tier": "quick" }));
}

#[test]
fn a_run_given_no_parameter_records_an_empty_set() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ECHO);

    let run = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let run_id = sandbox.harness_run_id();
    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id, "--json"]);
    assert_eq!(
        support::run(&mut show).report()["launch"]["params"],
        json!({})
    );
    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &run_id]);
    let human = support::run(&mut show);
    assert!(
        human.stdout.contains("  params     none\n"),
        "{}",
        human.stdout
    );
}

/// Run `argv` as the refusal reported it, in the directory it names.
fn reproduce(sandbox: &Sandbox, inspect: &Value) -> support::Run {
    let argv: Vec<&str> = inspect["argv"]
        .as_array()
        .unwrap()
        .iter()
        .map(|word| word.as_str().unwrap())
        .collect();
    let mut command = sandbox.command_for(Path::new(argv[0]));
    command
        .args(&argv[1..])
        .current_dir(inspect["cwd"].as_str().unwrap());
    support::run(&mut command)
}

#[test]
fn a_refused_runs_inspect_invocation_carries_each_parameter_and_reproduces_the_selection() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(TIERED);
    // A regular file where the record directory belongs: run refuses at its
    // record commit, which inspection never makes, so the equivalent
    // inspection succeeds and reports the selection the parameters decided.
    write(&sandbox.cwd.join("blocked"), "not a directory");
    let note = "note=two lines\nand 'quotes' = \"more\"";

    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--param",
            "tier=deep",
            "--param",
            note,
            "--param=-n=-v",
            "--state-dir",
            "blocked",
            "--prompt",
            "SECRET-PROMPT-TEXT",
            "--json",
        ])
        .refusal(4);
    let inspect = &refusal["error"]["inspect"];
    assert_eq!(
        inspect["argv"].as_array().unwrap()[1..],
        json!([
            "inspect",
            "--kind",
            "impl",
            "--param",
            "tier=deep",
            "--param",
            note,
            "--param=-n=-v",
            "--state-dir",
            "blocked",
            "--json"
        ])
        .as_array()
        .unwrap()[..]
    );
    let report = reproduce(&sandbox, inspect).report();
    assert_eq!(
        report["params"],
        json!({ "tier": "deep", "note": "two lines\nand 'quotes' = \"more\"", "-n": "-v" })
    );
    assert_eq!(report["selection"]["provider"], "origin-deep");
    assert_eq!(report["selection"]["reason"], "the caller's tier deep");
    // The prompt is left out, so the marker stands where `select` put it.
    assert_eq!(report["prompt"]["supplied"], false);
    assert_eq!(
        report["command"]["args"],
        json!(["--tier=deep", report["prompt"]["marker"]])
    );

    // A refusal the policy made for want of a parameter reproduces as that
    // refusal, and the parameter its remedy names selects.
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--param",
            "repo=/work/repo",
            "--prompt",
            "p",
            "--json",
        ])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused");
    assert_eq!(refusal["error"]["policyCode"], "tier_missing");
    let inspect = &refusal["error"]["inspect"];
    let reproduced = reproduce(&sandbox, inspect).refusal(3);
    assert_eq!(reproduced["error"]["policyCode"], "tier_missing");
    assert_eq!(reproduced["error"]["remedy"], "pass --param tier=NAME");
    let corrected = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--param",
            "repo=/work/repo",
            "--param",
            "tier=quick",
            "--json",
        ])
        .report();
    assert_eq!(corrected["selection"]["provider"], "origin-quick");
    assert!(!sandbox.harness_ran());
}

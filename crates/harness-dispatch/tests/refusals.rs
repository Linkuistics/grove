//! The actionable-refusal contract through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Diagnostics and exits*):
//! every refusal names a stable code, its stage, a message, the input or source
//! involved and a remedy (`support::Run::refusal` checks that for every JSON
//! refusal any test reads); each stage exits as the spec's table says; a
//! refused `run` names the equivalent `inspect` invocation, which reproduces
//! the selection; and nothing is retried.

mod support;

use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStringExt as _;
use std::path::{Path, PathBuf};

use serde_json::Value;
use support::{text, write, Run, Sandbox, FRONT, ROUTED};

/// Routes `impl` to the fake harness and each failing kind to a candidate whose
/// program fails to resolve in its own way.
const STAGES: &str = r#"export const policy = {
  schemaVersion: 1,
  version: "stages-1",
  catalog: [
    { id: "deep", provider: "origin-a", model: "m", effort: "high", program: "fake-harness", args: [{ slot: "prompt" }] },
    { id: "absent", provider: "origin-a", model: "m", effort: "high", program: "no-such-harness", args: [{ slot: "prompt" }] },
    { id: "inert", provider: "origin-a", model: "m", effort: "high", program: "./not-executable", args: [{ slot: "prompt" }] },
  ],
  routes: { impl: "deep", missing: "absent", inert: "inert" },
};
"#;

/// A copy of the front at `<root>/prefix/bin/harness-dispatch`, with no worker
/// beside it.
fn front_without_worker(sandbox: &Sandbox) -> PathBuf {
    let front = sandbox.root.join("prefix/bin/harness-dispatch");
    fs::create_dir_all(front.parent().unwrap()).unwrap();
    fs::copy(FRONT, &front).unwrap();
    front
}

#[test]
fn every_exit_result_matches_the_spec_table() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(STAGES);
    write(&sandbox.cwd.join("not-executable"), "#!/bin/sh\n");
    write(
        &sandbox.cwd.join("blocked"),
        "a file where the record directory belongs",
    );
    let spinning = sandbox.file("spin.ts", &format!("while (true) {{}}\n{ROUTED}"));

    let run = |args: &[&str]| sandbox.run(&[args, &["--prompt", "p", "--json"]].concat());
    let cases: Vec<(i32, &str, &str, Run)> = vec![
        (
            2,
            "cli",
            "malformed_input",
            run(&["--kind", "impl", "--timeout-ms", "soon"]),
        ),
        (
            3,
            "selection",
            "incomplete_mapping",
            run(&["--kind", "design"]),
        ),
        (
            4,
            "record",
            "record_store_unwritable",
            run(&["--kind", "impl", "--state-dir", "blocked"]),
        ),
        (5, "worker", "worker_missing", {
            let mut command = sandbox.command_for(&front_without_worker(&sandbox));
            command.args(["run", "--kind", "impl", "--prompt", "p", "--json"]);
            support::run(&mut command)
        }),
        (
            124,
            "evaluation",
            "selection_timeout",
            run(&[
                "--kind",
                "impl",
                "--config",
                &text(&spinning),
                "--timeout-ms",
                "1000",
            ]),
        ),
        (
            126,
            "resolution",
            "program_unexecutable",
            run(&["--kind", "inert"]),
        ),
        (
            127,
            "resolution",
            "program_not_found",
            run(&["--kind", "missing"]),
        ),
    ];
    for (exit, stage, code, outcome) in cases {
        let refusal = outcome.refusal(exit);
        let error = &refusal["error"];
        assert_eq!(error["stage"], stage, "exit {exit}: {refusal}");
        assert_eq!(error["code"], code, "exit {exit}: {refusal}");
        assert!(
            error["inspect"]["argv"].is_array(),
            "a refused run names no inspect invocation: {refusal}"
        );
    }
    assert!(!sandbox.harness_ran(), "a refusal launched the harness");

    // The positive control: the same sandbox and policy do launch.
    let launched = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(launched.code, Some(0), "{}", launched.stderr);
    assert!(sandbox.harness_ran());
}

/// A task identity that a shell and a flag parser would each misread if the
/// invocation did not keep it as data.
const AWKWARD_ID: &str = "-T 'one' \"two\" $HOME; `x`";

#[test]
fn a_refused_run_names_the_equivalent_inspect_invocation_which_reproduces_the_selection() {
    let sandbox = Sandbox::new();
    sandbox.file("policies/p.ts", ROUTED);
    // A regular file where the record directory belongs: run refuses at its
    // record commit, which inspection never makes, so the equivalent
    // inspection succeeds and reports the selection it reproduces.
    write(&sandbox.cwd.join("blocked"), "not a directory");
    let args = [
        "--kind",
        "impl",
        "--config",
        "policies/p.ts",
        "--task-file",
        "task one.md",
        "--task-id",
        AWKWARD_ID,
        "--timeout-ms",
        "5000",
        "--state-dir",
        "blocked",
        "--prompt",
        "SECRET-PROMPT-TEXT",
    ];

    let json = sandbox.run(&[&args[..], &["--json"]].concat());
    let refusal = json.refusal(4);
    assert!(!json.stderr.contains("SECRET-PROMPT"), "{}", json.stderr);
    let inspect = &refusal["error"]["inspect"];
    assert_eq!(inspect["cwd"], text(&sandbox.cwd));
    let expected: Vec<String> = [
        FRONT,
        "inspect",
        "--kind",
        "impl",
        "--config",
        "policies/p.ts",
        "--task-file",
        "task one.md",
    ]
    .iter()
    .map(|word| (*word).to_owned())
    .chain([format!("--task-id={AWKWARD_ID}")])
    .chain(
        ["--timeout-ms", "5000", "--state-dir", "blocked", "--json"]
            .iter()
            .map(|word| (*word).to_owned()),
    )
    .collect();
    assert_eq!(inspect["argv"], serde_json::json!(expected));

    // Run exactly that argv, in exactly that directory.
    let argv: Vec<&str> = expected.iter().map(String::as_str).collect();
    let mut reproduce = sandbox.command_for(Path::new(argv[0]));
    reproduce
        .args(&argv[1..])
        .current_dir(inspect["cwd"].as_str().unwrap());
    let report = support::run(&mut reproduce).report();
    assert_eq!(report["taskId"], AWKWARD_ID);
    assert_eq!(report["taskFile"], text(&sandbox.cwd.join("task one.md")));
    assert_eq!(report["policy"]["authority"], "explicit");
    assert_eq!(report["policy"]["argument"], "policies/p.ts");
    assert_eq!(report["bounds"]["selection"]["ms"], 5000);
    assert_eq!(
        report["stateDir"]["path"],
        text(&sandbox.cwd.join("blocked"))
    );
    assert_eq!(report["prompt"]["supplied"], false);

    // Text mode names the same invocation as one shell command line, which a
    // shell started anywhere runs as the same inspection.
    let human = sandbox.run(&args);
    assert_eq!(human.code, Some(4), "{}", human.stderr);
    assert!(!human.stderr.contains("SECRET-PROMPT"), "{}", human.stderr);
    let line = human
        .stderr
        .lines()
        .find_map(|line| line.strip_prefix("  inspect: "))
        .unwrap_or_else(|| panic!("no inspect line:\n{}", human.stderr));
    assert!(line.starts_with("(cd "), "{line}");
    let mut shell = sandbox.command_for(Path::new("/bin/sh"));
    shell.args(["-c", line]).current_dir(&sandbox.root);
    let reproduced = support::run(&mut shell);
    assert_eq!(reproduced.code, Some(0), "{line}\n{}", reproduced.stderr);
    assert!(
        reproduced
            .stdout
            .contains(&format!("task id    {AWKWARD_ID}\n")),
        "{line}\n{}",
        reproduced.stdout
    );
    assert!(
        reproduced.stdout.contains("candidate  deep"),
        "{}",
        reproduced.stdout
    );
    assert!(!sandbox.harness_ran());
}

#[test]
fn an_input_no_string_can_hold_makes_the_reproduction_unavailable_not_lossy() {
    let sandbox = Sandbox::new();
    // A candidate whose ID is the lossy form of `deep\xff`.
    sandbox.personal_policy(&ROUTED.replace(
        "  ],",
        "    { id: \"deep\\uFFFD\", provider: \"origin-b\", model: \"m\", effort: \"e\", program: \"fake-harness\", args: [{ slot: \"prompt\" }] },\n  ],",
    ));
    // The firing configuration: the lossy form of the refused choice is a
    // different, valid input, whose inspection selects.
    let lossy = sandbox
        .inspect(&["--kind", "impl", "--choice", "deep\u{FFFD}", "--json"])
        .report();
    assert_eq!(lossy["selection"]["candidateId"], "deep\u{FFFD}");

    for (flag, value, kind, exit) in [
        ("--choice", &b"deep\xff"[..], "impl", 2),
        ("--task-id", &b"k\xff"[..], "impl", 2),
        // Not refused itself: the unrouted kind refuses, and a lossy state
        // directory would name another place.
        ("--state-dir", &b"records-\xff"[..], "design", 3),
    ] {
        let invocation = |json: bool| {
            let mut command = sandbox.command();
            command
                .args(["run", "--kind", kind, "--prompt", "p", flag])
                .arg(OsString::from_vec(value.to_vec()));
            if json {
                command.arg("--json");
            }
            support::run(&mut command)
        };
        let reason = format!("{flag} is not valid UTF-8, so no command line reproduces it exactly");
        let refusal = invocation(true).refusal(exit);
        assert_eq!(
            refusal["error"]["inspect"],
            serde_json::json!({ "unavailable": reason }),
            "{flag}: {refusal}"
        );
        let human = invocation(false);
        assert_eq!(human.code, Some(exit), "{}", human.stderr);
        assert!(
            human
                .stderr
                .contains(&format!("\n  inspect: unavailable: {reason}\n")),
            "{flag}: {}",
            human.stderr
        );
    }
    assert!(!sandbox.harness_ran());
}

#[test]
fn only_a_refused_run_names_an_inspect_invocation() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);

    let run = sandbox
        .run(&["--kind", "design", "--prompt", "p", "--json"])
        .refusal(3);
    assert!(run["error"]["inspect"].is_object(), "{run}");
    // A run refused for want of a prompt still names an inspection, which
    // needs none.
    let unprompted = sandbox.run(&["--kind", "impl", "--json"]).refusal(2);
    assert_eq!(
        unprompted["error"]["inspect"]["argv"].as_array().unwrap()[1..],
        serde_json::json!(["inspect", "--kind", "impl", "--json"])
            .as_array()
            .unwrap()[..]
    );

    let inspect = sandbox.inspect(&["--kind", "design", "--json"]).refusal(3);
    assert!(inspect["error"].get("inspect").is_none(), "{inspect}");
    let human = sandbox.inspect(&["--kind", "design"]);
    assert!(!human.stderr.contains("  inspect: "), "{}", human.stderr);
}

#[test]
fn text_mode_prefixes_the_policy_output_before_the_refusal() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&format!(
        "console.log(\"chatter on stdout\");\nconsole.error(\"chatter on stderr\");\n{ROUTED}"
    ));

    let run = sandbox.run(&["--kind", "design", "--prompt", "p"]);

    assert_eq!(run.code, Some(3), "{}", run.stderr);
    assert_eq!(run.stdout, "", "a refusal printed on stdout");
    assert!(
        run.stderr.starts_with(
            "policy stdout: chatter on stdout\npolicy stderr: chatter on stderr\n\
             harness-dispatch: refused (incomplete_mapping, stage selection): "
        ),
        "{}",
        run.stderr
    );
    for line in [
        "  input: --kind design\n",
        "  source: ",
        "  remedy: ",
        "  inspect: ",
    ] {
        assert!(
            run.stderr.contains(line),
            "missing {line:?}:\n{}",
            run.stderr
        );
    }
}

#[test]
fn a_command_line_that_does_not_parse_refuses_in_both_modes() {
    let sandbox = Sandbox::new();
    let json = |args: Vec<OsString>| {
        let mut command = sandbox.command();
        command.args(args).arg("--json");
        support::run(&mut command).refusal(2)
    };
    let os = |words: &[&str]| words.iter().map(OsString::from).collect::<Vec<_>>();

    let cases = [
        (
            os(&["frobnicate"]),
            "frobnicate",
            "see harness-dispatch --help",
        ),
        (
            os(&["inspect", "--kidn", "impl"]),
            "--kidn",
            "a similar argument exists: '--kind'",
        ),
        (
            os(&["inspect"]),
            "--kind",
            "see harness-dispatch inspect --help",
        ),
        (
            os(&["record", "show"]),
            "--run",
            "see harness-dispatch record show --help",
        ),
        (
            vec![
                "inspect".into(),
                "--kind".into(),
                OsString::from_vec(vec![0xff]),
            ],
            "--kind",
            "usage: harness-dispatch inspect",
        ),
        (
            vec![
                "inspect".into(),
                OsString::from_vec(b"--kind=\xff".to_vec()),
            ],
            "--kind",
            "usage: harness-dispatch inspect",
        ),
    ];
    for (args, input, remedy) in cases {
        let refusal = json(args.clone());
        let error = &refusal["error"];
        assert_eq!(error["code"], "malformed_input", "{args:?}: {refusal}");
        assert_eq!(error["stage"], "cli", "{args:?}: {refusal}");
        assert_eq!(error["input"], input, "{args:?}: {refusal}");
        assert!(
            error["remedy"].as_str().unwrap().contains(remedy),
            "{args:?}: {refusal}"
        );
    }

    // Text mode renders the same refusal, not clap's own error.
    let mut command = sandbox.command();
    command.args(["inspect", "--kind", "impl", "extra"]);
    let run = support::run(&mut command);
    assert_eq!(run.code, Some(2));
    assert!(
        run.stderr.starts_with(
            "harness-dispatch: refused (malformed_input, stage cli): unexpected argument 'extra' found\n  input: extra\n  remedy: usage: harness-dispatch inspect"
        ),
        "{}",
        run.stderr
    );
}

#[test]
fn a_refusal_is_evaluated_once_and_never_retried() {
    let sandbox = Sandbox::new();
    let counter = sandbox.root.join("evaluations");
    sandbox.personal_policy(&format!(
        "import {{ appendFileSync }} from \"node:fs\";\nappendFileSync({:?}, \"evaluated\\n\");\n{ROUTED}",
        text(&counter)
    ));

    for json in [true, false] {
        let _ = fs::remove_file(&counter);
        let mut args = vec!["--kind", "design", "--prompt", "p"];
        if json {
            args.push("--json");
        }
        let run = sandbox.run(&args);
        assert_eq!(run.code, Some(3), "{}", run.stderr);
        let evaluations = fs::read_to_string(&counter).expect("the policy was evaluated");
        assert_eq!(evaluations, "evaluated\n", "json {json}");
    }
    assert!(!sandbox.harness_ran());
}

/// The Grove command definition the help quotes. Grove's launch-boundary
/// suite runs this command, and checks that the help and Grove's documentation
/// quote it word for word; here it is only required to be present.
const EXAMPLE_FOR_GROVE: &str = "  command \"dispatch\" \"harness-dispatch run --kind ${kind} --task-file ${task_file} --task-id ${task_id} --prompt ${prompt}\"\n";

#[test]
fn help_carries_independent_use_grove_and_refusal_recovery_examples() {
    let sandbox = Sandbox::new();
    let help = |args: &[&str]| {
        let mut command = sandbox.command();
        command.args(args);
        let run = support::run(&mut command);
        assert_eq!(run.code, Some(0), "{args:?}: {}", run.stderr);
        run.stdout
    };

    let top = help(&["--help"]);
    for fact in [
        "Examples:",
        "harness-dispatch inspect --kind impl",
        "no task tree, Grove installation or other caller",
        "2 malformed command line; 3 refused",
        "124 selection timeout; 126 program not executable; 127 program not found",
        "Nothing is retried, paged or confirmed interactively",
        "A refused run prints the equivalent inspect invocation",
        EXAMPLE_FOR_GROVE,
        "evaluated only at launch",
    ] {
        assert!(top.contains(fact), "top-level help lacks {fact:?}:\n{top}");
    }
    let inspect = help(&["inspect", "--help"]);
    for fact in [
        "--choice <ID>",
        "Inspection is a proposal, not a launch reservation",
        "not promised to be free of side effects",
        "harness-dispatch inspect --kind impl --choice deep",
        "Recovering from a refusal:",
        "harness-dispatch inspect --kind design --choice deep",
    ] {
        assert!(
            inspect.contains(fact),
            "inspect help lacks {fact:?}:\n{inspect}"
        );
    }
    let run = help(&["run", "--help"]);
    for fact in [
        "--choice <ID>",
        "harness-dispatch run --kind impl --choice deep --prompt",
        "Recovering from a refusal:",
        "(cd /work && harness-dispatch inspect --kind design)",
        "Nothing is retried for you.",
        EXAMPLE_FOR_GROVE,
    ] {
        assert!(run.contains(fact), "run help lacks {fact:?}:\n{run}");
    }
}

#[test]
fn a_prompt_file_is_never_part_of_the_equivalent_invocation() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    sandbox.file("secret-mandate.md", "SECRET\n");

    let refusal = sandbox
        .run(&[
            "--kind",
            "design",
            "--prompt-file",
            "secret-mandate.md",
            "--json",
        ])
        .refusal(3);
    let argv: Vec<Value> = refusal["error"]["inspect"]["argv"]
        .as_array()
        .unwrap()
        .clone();
    assert!(
        !argv.iter().any(|word| {
            let word = word.as_str().unwrap();
            word.contains("prompt") || word.contains("secret-mandate")
        }),
        "{argv:?}"
    );
}

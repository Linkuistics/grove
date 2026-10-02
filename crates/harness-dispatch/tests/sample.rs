//! The sample policy, through the command seam: `init` installs it into a
//! temporary HOME, and the installed file selects through the compiled worker
//! (`docs/specs/harness-selection-and-execution.md`, *The sample policy and the
//! choice file*).
//!
//! `fixtures/parity/commands.json` is what Grove's configuration resolver
//! produced from the configuration the sample converts. The sample is held to
//! it argument for argument, for every kind under every selection it offers.

mod support;

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use support::{executable, text, Run, Sandbox};

const SAMPLE: &str = include_str!("../worker/sample/policy.ts");
const PARITY: &str = include_str!("fixtures/parity/commands.json");

/// What a caller passes for the fixture's runtime slots, with the punctuation
/// a shell or a second reading would disturb.
const SESSION: &str = "parser grove";
const REPO: &str = "/work/main repo";
const PROMPT: &str = "Load the skill.\n$HOME stays literal; so does ${repo}\n";

fn init(sandbox: &Sandbox) -> Run {
    let mut command = sandbox.command();
    command.arg("init");
    support::run(&mut command)
}

/// A sandbox whose personal policy is the sample, installed by `init`, with a
/// stand-in for each harness on PATH.
fn installed() -> Sandbox {
    let sandbox = Sandbox::new();
    let run = init(&sandbox);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    for harness in ["codex", "claude"] {
        executable(&sandbox.bin.join(harness), "#!/bin/sh\nexit 0\n");
    }
    sandbox
}

/// Inspect `kind` as Grove would launch it.
fn inspect(sandbox: &Sandbox, kind: &str) -> Run {
    let (session, repo) = (format!("session_name={SESSION}"), format!("repo={REPO}"));
    sandbox.inspect(&[
        "--kind", kind, "--param", &session, "--param", &repo, "--prompt", PROMPT, "--json",
    ])
}

/// The policy's own refusal, under `code`.
fn refused(run: &Run, code: &str) -> Value {
    let refusal = run.refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused", "{refusal}");
    assert_eq!(refusal["error"]["policyCode"], code, "{refusal}");
    refusal
}

fn message(refusal: &Value) -> &str {
    refusal["error"]["message"].as_str().unwrap()
}

#[test]
fn init_writes_the_sample_into_an_empty_home_and_reports_it() {
    let sandbox = Sandbox::new();

    // With no policy, both commands refuse, and the remedy names init.
    for run in [
        sandbox.inspect(&["--kind", "impl", "--json"]),
        sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]),
    ] {
        let refusal = run.refusal(3);
        assert_eq!(refusal["error"]["code"], "policy_missing", "{refusal}");
        assert!(
            refusal["error"]["remedy"]
                .as_str()
                .unwrap()
                .contains("run `harness-dispatch init`"),
            "{refusal}"
        );
    }
    assert!(
        !sandbox.personal_path().exists(),
        "a refusal installed a policy"
    );

    let run = init(&sandbox);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(fs::read_to_string(sandbox.personal_path()).unwrap(), SAMPLE);
    // The report names the path, and says what the sample does with codex.
    for fact in [
        text(&sandbox.personal_path()).as_str(),
        "codex with approvals off and full access",
        "--ask-for-approval never",
        "default_permissions=:danger-full-access",
        "harness-dispatch inspect --kind impl",
    ] {
        assert!(
            run.stdout.contains(fact),
            "init's report lacks {fact:?}: {}",
            run.stdout
        );
    }
    // And the file says so itself, where an owner edits it.
    assert!(SAMPLE.contains("IT RUNS CODEX WITH APPROVALS OFF AND FULL ACCESS"));

    // The installed file is the personal policy.
    for harness in ["codex", "claude"] {
        executable(&sandbox.bin.join(harness), "#!/bin/sh\nexit 0\n");
    }
    let report = inspect(&sandbox, "impl").report();
    assert_eq!(report["policy"]["authority"], "personal");
    assert_eq!(report["policy"]["path"], text(&sandbox.personal_path()));
    assert_eq!(report["policy"]["version"], "harness-dispatch sample 1");
}

#[test]
fn init_refuses_beside_anything_already_at_the_path() {
    type Plant = fn(&Path);
    let occupants: [(&str, Plant); 3] = [
        ("a policy", |path| support::write(path, "mine\n")),
        ("a directory", |path| fs::create_dir_all(path).unwrap()),
        ("a dangling link", |path| {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            symlink("nowhere.ts", path).unwrap();
        }),
    ];
    for (what, plant) in occupants {
        let sandbox = Sandbox::new();
        let path = sandbox.personal_path();
        plant(&path);

        let run = init(&sandbox);
        assert_eq!(run.code, Some(3), "{what}: {}", run.stderr);
        assert_eq!(run.stdout, "", "{what}");
        assert!(
            run.stderr
                .contains("refused (policy_exists, stage authority)")
                && run.stderr.contains(&text(&path)),
            "{what}: {}",
            run.stderr
        );
        if what == "a policy" {
            assert_eq!(fs::read_to_string(&path).unwrap(), "mine\n", "replaced");
        }
        assert!(fs::symlink_metadata(&path).is_ok(), "{what} was removed");
    }

    // It takes no input, so there is no way to ask for a replacement.
    let sandbox = Sandbox::new();
    let mut command = sandbox.command();
    command.args(["init", "--force"]);
    assert_eq!(support::run(&mut command).code, Some(2));
    assert!(!sandbox.personal_path().exists());
}

#[test]
fn the_sample_selects_the_recorded_command_for_every_kind_under_every_selection() {
    let fixture: Value = serde_json::from_str(PARITY).unwrap();
    let entries = fixture.as_array().unwrap();
    assert_eq!(entries.len(), 16, "four arrangements, each four ways");

    let sandbox = installed();
    let filled = |word: &Value| match word.as_str().unwrap() {
        "${prompt}" => PROMPT.to_owned(),
        "${session_name}" => SESSION.to_owned(),
        "${repo}" => REPO.to_owned(),
        slot if slot.starts_with("${") => panic!("the fixture has an unknown slot {slot}"),
        literal => literal.to_owned(),
    };
    let mut compared = 0;
    for entry in entries {
        let names: Vec<&str> = std::iter::once(&entry["arrangement"])
            .chain(entry["modifiers"].as_array().unwrap())
            .map(|name| name.as_str().unwrap())
            .collect();
        let selection = names.join(" ");
        // The pinned active selection is the sample's default, so it is
        // reached with no choice file; every other needs one.
        let by_default = selection == "claude-led codex-sol";
        let choice = sandbox.cwd.join(".harness-dispatch-choice");
        if by_default {
            let _ = fs::remove_file(&choice);
        } else {
            fs::write(&choice, format!("{selection}\n")).unwrap();
        }

        let commands = entry["commands"].as_object().unwrap();
        assert_eq!(
            commands.len(),
            24,
            "{selection}: 23 session kinds and release-notes"
        );
        for (kind, recorded) in commands {
            let report = inspect(&sandbox, kind).report();
            let args: Vec<String> = recorded["args"]
                .as_array()
                .unwrap()
                .iter()
                .map(filled)
                .collect();
            assert_eq!(
                report["command"]["program"], recorded["program"],
                "{selection}, {kind}"
            );
            assert_eq!(
                report["command"]["args"],
                json!(args),
                "{selection}, {kind}"
            );

            // The labels are the command's own: its provider by program, and
            // the model and effort its arguments carry.
            let chosen = &report["selection"];
            let (provider, model, effort) = match recorded["program"].as_str().unwrap() {
                "codex" => (
                    "openai",
                    args[1].clone(),
                    args[3].replace("model_reasoning_effort=", ""),
                ),
                "claude" => ("anthropic", args[5].clone(), args[7].clone()),
                "/bin/bash" => ("openai", args[1].clone(), args[2].clone()),
                other => panic!("the fixture has an unknown program {other}"),
            };
            assert_eq!(
                (&chosen["provider"], &chosen["model"], &chosen["effort"]),
                (&json!(provider), &json!(model), &json!(effort)),
                "{selection}, {kind}"
            );
            // The reason names the choice applied and where it came from.
            let reason = chosen["reason"].as_str().unwrap();
            let from = if by_default {
                "(the default)"
            } else {
                "(chosen by .harness-dispatch-choice in "
            };
            assert!(
                reason.starts_with(&format!("{selection} {from}")),
                "{selection}, {kind}: {reason}"
            );
            compared += 1;
        }
    }
    assert_eq!(compared, 16 * 24);
}

#[test]
fn the_sample_refuses_a_kind_it_does_not_route_and_a_caller_without_a_needed_parameter() {
    let sandbox = installed();

    for kind in ["review", "Impl", "constructor"] {
        let refusal = refused(&inspect(&sandbox, kind), "incomplete_mapping");
        assert!(
            message(&refusal).contains(&format!("{kind:?}")),
            "{refusal}"
        );
    }

    // Under the default, impl is claude's, which is named for the session and
    // given the repository; review-impl is codex's, which is given the
    // repository alone.
    let (session, repo) = (format!("session_name={SESSION}"), format!("repo={REPO}"));
    for (kind, params, missing) in [
        ("impl", &[][..], "session_name"),
        ("impl", &[repo.as_str()][..], "session_name"),
        ("impl", &[session.as_str()][..], "repo"),
        ("review-impl", &[session.as_str()][..], "repo"),
    ] {
        let mut args = vec!["--kind", kind, "--json"];
        for param in params {
            args.extend(["--param", param]);
        }
        let refusal = refused(&sandbox.inspect(&args), "parameter_missing");
        assert!(
            message(&refusal).contains(&format!("needs the parameter {missing}")),
            "{kind} {params:?}: {refusal}"
        );
        assert!(
            refusal["error"]["remedy"]
                .as_str()
                .unwrap()
                .contains(&format!("--param {missing}=VALUE")),
            "{refusal}"
        );
    }
    // A command that places only what it was given selects.
    sandbox
        .inspect(&["--kind", "review-impl", "--param", &repo, "--json"])
        .report();
    sandbox
        .inspect(&["--kind", "release-notes", "--json"])
        .report();
}

#[test]
fn a_choice_file_replaces_the_samples_default_whole_and_names_exactly_one_arrangement() {
    let sandbox = installed();
    let choice = sandbox.cwd.join(".harness-dispatch-choice");
    let program = |kind: &str| inspect(&sandbox, kind).report()["command"]["program"].clone();

    // Absent: the default, under which claude leads.
    assert_eq!(program("impl"), "claude");
    // Names on several lines select, and replace the default whole.
    fs::write(&choice, "codex-led\n  high-effort\n").unwrap();
    let report = inspect(&sandbox, "impl").report();
    assert_eq!(report["command"]["program"], "codex");
    let reason = report["selection"]["reason"].as_str().unwrap();
    assert!(
        reason.starts_with(&format!(
            "codex-led high-effort (chosen by .harness-dispatch-choice in {})",
            text(&sandbox.cwd)
        )),
        "{reason}"
    );
    // A file elsewhere chooses nothing: only the caller's directory is read.
    fs::remove_file(&choice).unwrap();
    support::write(
        &sandbox.home.join(".harness-dispatch-choice"),
        "codex-led\n",
    );
    support::write(
        &sandbox.root.join(".harness-dispatch-choice"),
        "codex-led\n",
    );
    assert_eq!(program("impl"), "claude");

    // A name the sample does not offer refuses, naming the file, the name and
    // every name offered.
    fs::write(&choice, "claude-led turbo\n").unwrap();
    let refusal = refused(&inspect(&sandbox, "impl"), "choice_unoffered");
    for fact in [
        text(&choice).as_str(),
        "\"turbo\"",
        "codex-led, claude-led, codex-design-claude-impl, claude-design-codex-impl, codex-sol, high-effort",
    ] {
        assert!(message(&refusal).contains(fact), "lacks {fact:?}: {refusal}");
    }

    // The file names exactly one arrangement, whatever modifiers it adds.
    for (names, count) in [
        ("", 0),
        ("codex-sol high-effort", 0),
        ("codex-led claude-led", 2),
    ] {
        fs::write(&choice, names).unwrap();
        let refusal = refused(&inspect(&sandbox, "impl"), "choice_arrangement");
        assert!(
            message(&refusal).contains(&format!("names {count} arrangements")),
            "{names:?}: {refusal}"
        );
    }
}

#[test]
fn the_sample_ships_readable_beside_the_worker_and_is_no_example_to_import() {
    let sandbox = installed();
    let report = inspect(&sandbox, "impl").report();
    let worker = PathBuf::from(report["worker"]["path"].as_str().unwrap());
    let examples = worker.parent().unwrap().join("examples");

    assert_eq!(
        fs::read_to_string(examples.join("sample.ts")).unwrap(),
        SAMPLE,
        "the readable sample is not the one init installs; run `task dispatch:worker`"
    );
    assert!(
        !examples.join("sample.d.ts").exists(),
        "the sample is a policy file, with no declarations to import"
    );
    sandbox.personal_policy("export { policy } from \"harness-dispatch/examples/sample\";\n");
    let refusal = inspect(&sandbox, "impl").refusal(3);
    assert_eq!(
        refusal["error"]["code"], "policy_import_failed",
        "{refusal}"
    );
}

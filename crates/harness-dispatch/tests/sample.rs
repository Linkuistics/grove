//! The sample policy, through the command seam: `init` installs it into a
//! temporary HOME, and the installed file selects through the compiled worker
//! (`docs/specs/harness-selection-and-execution.md`, *The sample policy and the
//! choice file*).
//!
//! `fixtures/parity/commands.json` is what Grove's configuration resolver
//! produced from the configuration the sample converts. The sample is held to
//! it argument for argument, for every kind under every selection it offers,
//! less the session name it no longer places: it reads no parameter. Its one
//! grant, the main repository, it derives from a secondary jj workspace's
//! `.jj/repo` in the caller's directory.

mod support;

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use support::{executable, text, Run, Sandbox};

const SAMPLE: &str = include_str!("../worker/sample/policy.ts");
const PARITY: &str = include_str!("fixtures/parity/commands.json");

/// The prompt, with the punctuation a shell or a second reading would disturb.
const PROMPT: &str = "Load the skill.\n$HOME stays literal; so does ${repo}\n";

/// The main repository of the secondary workspace [`secondary`] makes, beside
/// the sandbox's cwd, with a space in its name.
const MAIN: &str = "main repo";

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

/// Inspect `kind` as Grove would launch it: no parameter, in the cwd.
fn inspect(sandbox: &Sandbox, kind: &str) -> Run {
    sandbox.inspect(&["--kind", kind, "--prompt", PROMPT, "--json"])
}

/// Make the sandbox's cwd a secondary jj workspace, whose `.jj/repo` is a file
/// naming the store in jj's own form — relative to `.jj/`, as this grove's own
/// workspace holds `../../grove/.jj/repo` — and return the main repository.
fn secondary(sandbox: &Sandbox) -> PathBuf {
    let main = sandbox.root.join(MAIN);
    fs::create_dir_all(main.join(".jj/repo")).unwrap();
    fs::create_dir_all(sandbox.cwd.join(".jj")).unwrap();
    fs::write(
        sandbox.cwd.join(".jj/repo"),
        format!("../../{MAIN}/.jj/repo"),
    )
    .unwrap();
    main
}

/// Make the sandbox's cwd a primary jj workspace, whose `.jj/repo` is the store.
fn primary(sandbox: &Sandbox) {
    let _ = fs::remove_file(sandbox.cwd.join(".jj/repo"));
    fs::create_dir_all(sandbox.cwd.join(".jj/repo/store")).unwrap();
}

/// The arguments the sample places for a fixture command: the fixture's, less
/// the session name, with the main repository granted only when there is one.
fn expected(recorded: &Value, main: Option<&Path>) -> Vec<String> {
    let mut words = recorded
        .as_array()
        .unwrap()
        .iter()
        .map(|word| word.as_str().unwrap());
    let mut args = Vec::new();
    while let Some(word) = words.next() {
        match word {
            "-n" => assert_eq!(words.next(), Some("${session_name}"), "{recorded}"),
            "--add-dir" => {
                assert_eq!(words.next(), Some("${repo}"), "{recorded}");
                if let Some(main) = main {
                    args.extend(["--add-dir".to_owned(), text(main)]);
                }
            }
            "${prompt}" => args.push(PROMPT.to_owned()),
            slot if slot.starts_with("${") => panic!("the fixture has an unknown slot {slot}"),
            literal => args.push(literal.to_owned()),
        }
    }
    args
}

/// The word after `flag` in `args`.
fn after<'a>(args: &'a [String], flag: &str) -> &'a str {
    let at = args.iter().position(|word| word == flag).unwrap();
    &args[at + 1]
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
    assert_eq!(report["policy"]["version"], "harness-dispatch sample 2");
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
    let main = secondary(&sandbox);
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
            let args = expected(&recorded["args"], Some(&main));
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
                    after(&args, "--model").to_owned(),
                    args.iter()
                        .find_map(|word| word.strip_prefix("model_reasoning_effort="))
                        .unwrap()
                        .to_owned(),
                ),
                "claude" => (
                    "anthropic",
                    after(&args, "--model").to_owned(),
                    after(&args, "--effort").to_owned(),
                ),
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
fn the_sample_grants_the_main_repository_only_from_a_secondary_workspace() {
    let fixture: Value = serde_json::from_str(PARITY).unwrap();
    let sandbox = installed();
    // The default selection, whose kinds run all three programs.
    let default = fixture
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| {
            entry["arrangement"] == "claude-led" && entry["modifiers"] == json!(["codex-sol"])
        })
        .unwrap();
    let main = sandbox.root.join(MAIN);
    let programs = |grant: Option<&Path>| {
        let mut seen = std::collections::BTreeSet::new();
        for (kind, recorded) in default["commands"].as_object().unwrap() {
            let report = inspect(&sandbox, kind).report();
            assert_eq!(
                report["command"]["args"],
                json!(expected(&recorded["args"], grant)),
                "{kind}, granting {grant:?}"
            );
            seen.insert(recorded["program"].as_str().unwrap().to_owned());
        }
        assert_eq!(
            seen.len(),
            3,
            "codex, claude and the notes writer: {seen:?}"
        );
    };

    // No `.jj` at all, as in `grove run`'s staged directory: nothing more.
    programs(None);
    // A primary workspace holds its store: nothing more.
    primary(&sandbox);
    programs(None);
    // A secondary workspace names the main repository's store.
    fs::remove_dir_all(sandbox.cwd.join(".jj")).unwrap();
    assert_eq!(secondary(&sandbox), main);
    programs(Some(&main));
}

#[test]
fn the_sample_refuses_a_kind_it_does_not_route_and_reads_no_parameter() {
    let sandbox = installed();

    for kind in ["review", "Impl", "constructor"] {
        let refusal = refused(&inspect(&sandbox, kind), "incomplete_mapping");
        assert!(
            message(&refusal).contains(&format!("{kind:?}")),
            "{refusal}"
        );
    }

    // A parameter a caller still passes, the two Grove used to among them,
    // changes nothing: the sample names no session and grants by the cwd.
    secondary(&sandbox);
    for kind in ["impl", "review-impl", "release-notes"] {
        let bare = inspect(&sandbox, kind).report()["command"].clone();
        let given = sandbox
            .inspect(&[
                "--kind",
                kind,
                "--param",
                "session_name=parser grove",
                "--param",
                "repo=/elsewhere",
                "--prompt",
                PROMPT,
                "--json",
            ])
            .report();
        assert_eq!(given["command"], bare, "{kind}");
        let args = bare["args"].as_array().unwrap();
        assert!(
            !args.contains(&json!("-n")) && !args.contains(&json!("/elsewhere")),
            "{kind}: {bare}"
        );
    }

    // A `.jj/repo` file that names no store refuses, naming the file.
    for (contents, why) in [("", "it is empty"), ("\n  \n", "it is empty")] {
        fs::write(sandbox.cwd.join(".jj/repo"), contents).unwrap();
        let refusal = refused(&inspect(&sandbox, "impl"), "jj_store_unreadable");
        assert!(
            message(&refusal).contains(&text(&sandbox.cwd.join(".jj/repo")))
                && message(&refusal).contains(why),
            "{contents:?}: {refusal}"
        );
    }
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

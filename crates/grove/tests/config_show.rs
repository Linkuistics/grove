use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use grove_loop::{DriverLease, Workspace};

mod support;

struct Fixture {
    dir: tempfile::TempDir,
    repo: PathBuf,
    home: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path().canonicalize().unwrap().join("repo");
        let home = dir.path().join("home");
        fs::create_dir_all(&repo).unwrap();
        fs::create_dir_all(home.join(".config/grove")).unwrap();
        support::init_jj_repo(&repo);
        fs::write(repo.join(".gitignore"), "/.grove.kdl\n").unwrap();
        let fixture = Self { dir, repo, home };
        fixture.personal(
            r#"config {
                command "agent" "absent-agent ${prompt}"
                command "local" "local-agent ${prompt}"
                bind "lead" "agent"
                route "impl" "lead"
            }"#,
        );
        fixture
    }

    fn personal(&self, text: &str) {
        fs::write(self.home.join(".config/grove/config.kdl"), text).unwrap();
    }

    fn show(&self, args: &[&str]) -> Output {
        self.show_at(&self.repo, args)
    }

    fn show_at(&self, cwd: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_grove"))
            .current_dir(cwd)
            .env("HOME", &self.home)
            .env("GROVE_SIGNAL_FILE", self.dir.path().join("stale/signal"))
            .args(["config", "show"])
            .args(args)
            .output()
            .unwrap()
    }
}

// Include directory entries as well as bytes: creating an empty coordination
// directory is a write too. jj metadata is the sole permitted exception.
fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn walk(root: &Path, path: &Path, result: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() == ".jj" {
                let control = entry.path().join("grove");
                if control.exists() {
                    result.insert(control.strip_prefix(root).unwrap().to_owned(), Vec::new());
                    walk(root, &control, result);
                }
                continue;
            }
            let path = entry.path();
            if path.is_dir() {
                result.insert(path.strip_prefix(root).unwrap().to_owned(), Vec::new());
                walk(root, &path, result);
            } else {
                result.insert(
                    path.strip_prefix(root).unwrap().to_owned(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut result = BTreeMap::new();
    walk(root, root, &mut result);
    result
}

fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn reports_profiles_words_and_provenance_without_a_tree_or_writes() {
    let fixture = Fixture::new();
    fixture.personal(
        r#"config {
        command "agent" "absent-agent --effort=${param.effort} ${param.text} ${prompt}" {
            param "effort" "low"
            param "text" "${prompt}"
        }
        bind "lead" "agent"
        route "zeta" "lead"
        route "impl" "lead"
        profile "base" { values "agent" { param "effort" "medium"; }; }
        profile "daily" { include "base"; }
        select "daily"
    }"#,
    );
    fs::write(
        fixture.repo.join(".grove.kdl"),
        r#"config {
        values "agent" { param "effort" "high"; }
        route "local-only" "lead"
    }"#,
    )
    .unwrap();
    let before = snapshot(fixture.dir.path());
    let report = success(fixture.show(&[]));
    for expected in [
        "Sources",
        "config.kdl",
        ".grove.kdl",
        "Selection",
        "daily",
        "base",
        "Non-admitted",
        "local-only",
        "binding",
        "lead",
        "agent",
        "Parameter",
        "effort",
        "low",
        "medium",
        "high",
        "Histories",
        "Origins",
        "bytes",
        "executable",
        "literal \"${prompt}\"",
        "slot <prompt>",
    ] {
        assert!(report.contains(expected), "missing {expected}: {report}");
    }
    assert!(report.find("Kind \"impl\"").unwrap() < report.find("Kind \"zeta\"").unwrap());
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[test]
fn inspection_ignores_a_held_driver_lease_and_stale_signal() {
    let fixture = Fixture::new();
    let workspace = Workspace::resolve(&fixture.repo).unwrap();
    let _lease = DriverLease::acquire(&workspace).unwrap();
    let before = snapshot(fixture.dir.path());
    assert!(success(fixture.show(&[])).contains("absent-agent"));
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[test]
fn existing_tree_and_launchable_command_are_observed_without_execution() {
    let fixture = Fixture::new();
    fixture.personal(
        r#"config {
            command "observe" "sh -c 'touch launched' ${prompt}"
            command "agent" "absent-agent ${prompt}"
            bind "observe" "observe"
            bind "lead" "agent"
            route "impl" "observe"
            route "zeta" "lead"
        }"#,
    );
    fs::create_dir(fixture.repo.join(".grove")).unwrap();
    fs::write(fixture.repo.join(".grove/_BRIEF.md"), "# Existing work\n").unwrap();
    fs::write(
        fixture.repo.join(".grove/01-impl--work-k1.md"),
        "# work-k1\n",
    )
    .unwrap();
    let before = snapshot(fixture.dir.path());
    let report = success(fixture.show(&["--kind", "impl"]));
    assert!(report.contains("Kind \"impl\""));
    assert!(!report.contains("Kind \"zeta\""));
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[test]
fn subdirectory_uses_workspace_delta_and_kind_filter_keeps_global_validation() {
    let fixture = Fixture::new();
    let subdir = fixture.repo.join("subdir");
    fs::create_dir(&subdir).unwrap();
    fs::write(
        fixture.repo.join(".grove.kdl"),
        "config { bind \"lead\" \"local\"; }\n",
    )
    .unwrap();
    assert!(success(fixture.show_at(&subdir, &["--kind", "impl"])).contains("local-agent"));
    fixture.personal(
        r#"config {
            command "agent" "absent-agent ${prompt}"
            command "local" "local-agent ${prompt}"
            command "invalid" "invalid-without-prompt"
            bind "lead" "agent"
            bind "invalid" "invalid"
            route "impl" "lead"
            route "design" "invalid"
        }"#,
    );
    let output = fixture.show(&["--kind", "impl"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(
        error.contains("must contain `${prompt}` exactly once")
            && error.contains("config.kdl")
            && error.contains("bytes"),
        "{error}"
    );
}

#[test]
fn requested_unknown_and_non_admitted_kinds_fail_with_personal_source() {
    let fixture = Fixture::new();
    fs::write(
        fixture.repo.join(".grove.kdl"),
        "config { route \"local-only\" \"lead\"; }\n",
    )
    .unwrap();
    for kind in ["unknown", "local-only"] {
        let output = fixture.show(&["--kind", kind]);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(
            error.contains(kind) && error.contains("config.kdl"),
            "{error}"
        );
    }
}

#[test]
fn tracked_and_unreadable_candidates_fail_without_fallback_or_writes() {
    let fixture = Fixture::new();
    fs::write(fixture.repo.join(".gitignore"), "").unwrap();
    fs::write(
        fixture.repo.join(".grove.kdl"),
        "config { bind \"lead\" \"local\"; }\n",
    )
    .unwrap();
    let before = snapshot(fixture.dir.path());
    let output = fixture.show(&[]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("tracked"));
    assert_eq!(snapshot(fixture.dir.path()), before);

    // A directory is deterministically unreadable as configuration, even as root.
    let fixture = Fixture::new();
    fs::create_dir(fixture.repo.join(".grove.kdl")).unwrap();
    let before = snapshot(fixture.dir.path());
    let output = fixture.show(&[]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains(".grove.kdl"));
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[test]
fn help_and_usage_errors_describe_the_working_surface() {
    let fixture = Fixture::new();
    let help = success(fixture.show(&["--help"]));
    for expected in [
        "--kind",
        "grove config show",
        "Exit codes",
        "0",
        "1",
        "2",
        "read-only",
    ] {
        assert!(help.contains(expected), "missing {expected}: {help}");
    }
    for args in [
        vec!["--unknown"],
        vec!["--kind"],
        vec!["--profile", "daily"],
    ] {
        let output = fixture.show(&args);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

fn json_success(output: Output) -> serde_json::Value {
    serde_json::from_str(&success(output)).unwrap()
}

fn json_failure(output: Output, code: i32) -> serde_json::Value {
    assert_eq!(output.status.code(), Some(code));
    assert!(output.stdout.is_empty());
    let report: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(report["schema_version"], 1);
    assert!(!report["diagnostics"].as_array().unwrap().is_empty());
    report
}

#[test]
// Legacy-only inspection variants remain until modular-only-k3 removes them.
fn legacy_json_reports_tagged_words_nulls_and_complete_tables_without_writes() {
    let fixture = Fixture::new();
    fixture.personal("impl \"absent-agent ${prompt}\"\n");
    let before = snapshot(fixture.dir.path());
    let report = json_success(fixture.show(&["--json"]));
    assert_eq!(report["schema_version"], 1);
    assert_eq!(
        report["selection"],
        serde_json::json!({"profiles": [], "origin": null})
    );
    assert_eq!(report["sources"][0]["role"], "primary");
    let command = &report["commands"][0];
    assert_eq!(command["key"], "impl");
    assert!(command["binding"].is_null());
    assert!(command["command"].is_null());
    assert_eq!(command["parameters"], serde_json::json!([]));
    assert_eq!(
        command["words"][0]["word"],
        serde_json::json!({"type": "literal", "value": "absent-agent"})
    );
    assert_eq!(
        command["words"][1]["word"],
        serde_json::json!({"type": "slot", "name": "prompt"})
    );
    assert_eq!(
        report["histories"][0]["assignments"][0]["value"]["type"],
        "literal_template"
    );
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[test]
fn json_usage_errors_are_one_diagnostic_object_even_before_dispatch() {
    let fixture = Fixture::new();
    for args in [
        vec!["--json", "--unknown"],
        vec!["--unknown", "--json"],
        vec!["--json", "--kind"],
        vec!["--kind", "--json"],
        vec!["--json", "--profile", "daily"],
        vec!["--json=true"],
        vec!["--json=false"],
    ] {
        let report = json_failure(fixture.show(&args), 2);
        assert_eq!(report["diagnostics"][0]["category"], "usage");
        assert!(report["diagnostics"][0]["source"].is_null());
        assert!(!report["diagnostics"][0]["remedy"]
            .as_str()
            .unwrap()
            .is_empty());
    }
    assert!(success(fixture.show(&["--json", "--help"])).contains("--json"));
}

#[test]
fn legacy_json_keeps_all_provenance_variants_and_references_when_filtered() {
    use serde_json::json;
    let fixture = Fixture::new();
    fixture.personal(
        r#"impl "literal ${prompt}"
config {
    command "agent" "absent-agent ${param.p} ${prompt}" { param "p" "default"; }
    bind "lead" "agent"
    route "zeta" "lead"
    profile "patch" { route "impl" { param "p" "stale"; }; }
    profile "target" { route "impl" "lead"; }
    profile "base" { values "agent" { param "p" "shared"; }; }
    profile "daily" { include "patch" "target" "base" "base"; }
    select "daily"
}"#,
    );
    fs::write(
        fixture.repo.join(".grove.kdl"),
        r#"config {
        route "impl" { unset "p"; }
        route "local-only" "lead"
    }"#,
    )
    .unwrap();
    let before = snapshot(fixture.dir.path());
    let full = json_success(fixture.show(&["--json"]));
    let filtered = json_success(fixture.show(&["--json", "--kind", "impl"]));
    for field in [
        "sources",
        "selection",
        "profile_occurrences",
        "origins",
        "histories",
        "non_admitted_keys",
    ] {
        assert_eq!(filtered[field], full[field], "{field}");
    }
    assert_eq!(filtered["commands"].as_array().unwrap().len(), 1);
    assert_eq!(full["commands"][1]["key"], "zeta");
    let command = &filtered["commands"][0];
    assert_eq!(command["binding"], "lead");
    assert_eq!(command["command"], "agent");
    assert_eq!(command["parameters"][0]["value"], "shared");
    assert_eq!(filtered["sources"][1]["role"], "overlay");
    assert_eq!(filtered["selection"]["profiles"], json!(["daily"]));
    assert_eq!(filtered["non_admitted_keys"][0]["key"], "local-only");
    let occurrences = filtered["profile_occurrences"].as_array().unwrap();
    assert_eq!(occurrences.len(), 5);
    assert_eq!(occurrences[0]["parent"], json!(null));
    assert_eq!(occurrences[1]["parent"], 0);
    assert_eq!(occurrences[3]["profile"], occurrences[4]["profile"]);
    let origins = filtered["origins"].as_array().unwrap();
    let histories = filtered["histories"].as_array().unwrap();
    for (id, origin) in origins.iter().enumerate() {
        assert_eq!(origin["id"], id);
        let span = &origin["span"];
        let bytes = fs::read(span["source"]["path"].as_str().unwrap()).unwrap();
        let start = span["start"].as_u64().unwrap() as usize;
        let end = span["end"].as_u64().unwrap() as usize;
        assert!(start < end && end <= bytes.len());
        if let Some(id) = origin["occurrence"].as_u64() {
            assert_eq!(occurrences[id as usize]["id"], id);
        }
    }
    let mut setting_types = std::collections::BTreeSet::new();
    let mut assignment_types = std::collections::BTreeSet::new();
    for (id, history) in histories.iter().enumerate() {
        assert_eq!(history["id"], id);
        setting_types.insert(history["setting"]["type"].as_str().unwrap());
        let mut previous = None;
        for assignment in history["assignments"].as_array().unwrap() {
            assignment_types.insert(assignment["value"]["type"].as_str().unwrap());
            assert!(assignment["origin"].as_u64().unwrap() < origins.len() as u64);
            let order = assignment["order"].as_u64().unwrap();
            assert!(previous.is_none_or(|previous| previous < order));
            previous = Some(order);
        }
    }
    assert_eq!(
        setting_types,
        [
            "binding_target",
            "route_target",
            "parameter_default",
            "command_parameter",
            "route_parameter"
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(
        assignment_types,
        ["set", "literal_template", "unset", "reset"]
            .into_iter()
            .collect()
    );
    fn references(value: &serde_json::Value, origins: usize, histories: usize) {
        match value {
            serde_json::Value::Object(object) => {
                for (name, child) in object {
                    if name == "origins" || name == "histories" {
                        let bound = if name == "origins" {
                            origins
                        } else {
                            histories
                        };
                        for id in child.as_array().unwrap() {
                            assert!(id.as_u64().unwrap() < bound as u64);
                        }
                    } else {
                        references(child, origins, histories);
                    }
                }
            }
            serde_json::Value::Array(values) => {
                for value in values {
                    references(value, origins, histories);
                }
            }
            _ => (),
        }
    }
    references(command, origins.len(), histories.len());
    references(
        &filtered["non_admitted_keys"],
        origins.len(),
        histories.len(),
    );
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[test]
fn json_observation_ignores_lease_and_epoch_and_never_executes() {
    let fixture = Fixture::new();
    fixture.personal(
        r#"config {
            command "observe" "sh -c 'touch launched' ${prompt}"
            bind "lead" "observe"
            route "impl" "lead"
        }"#,
    );
    fs::create_dir(fixture.repo.join(".grove")).unwrap();
    fs::write(
        fixture.repo.join(".grove/01-impl--work-k1.md"),
        "# work-k1\n",
    )
    .unwrap();
    let workspace = Workspace::resolve(&fixture.repo).unwrap();
    let _lease = DriverLease::acquire(&workspace).unwrap();
    let before = snapshot(fixture.dir.path());
    let report = json_success(fixture.show(&["--json", "--kind", "impl"]));
    assert_eq!(report["commands"][0]["words"][0]["word"]["value"], "sh");
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[test]
fn json_refusals_preserve_structured_sources_and_global_validation() {
    let fixture = Fixture::new();
    fs::write(
        fixture.repo.join(".grove.kdl"),
        "config { route \"local-only\" \"lead\"; }\n",
    )
    .unwrap();
    for kind in ["unknown", "local-only"] {
        let before = snapshot(fixture.dir.path());
        let report = json_failure(fixture.show(&["--json", "--kind", kind]), 1);
        let diagnostic = &report["diagnostics"][0];
        assert_eq!(diagnostic["key"], kind);
        assert_eq!(diagnostic["source"]["role"], "primary");
        assert_eq!(snapshot(fixture.dir.path()), before);
    }
    fixture.personal(
        r#"config {
    command "agent" "absent-agent ${prompt}"
    bind "impl" "agent"
    route "impl" "impl"
    command "a" "absent-agent ${param.p} ${prompt}" { param "p"; }
    bind "lead" "a"
    profile "patch" { route "broken" { param "p" "personal"; }; }
    profile "nested" { include "patch"; }
    select "nested"
}"#,
    );
    fs::write(
        fixture.repo.join(".grove.kdl"),
        "config { route \"broken\" \"lead\" { param \"p\" \"local\"; }; }",
    )
    .unwrap();
    let before = snapshot(fixture.dir.path());
    let report = json_failure(fixture.show(&["--json", "--kind", "impl"]), 1);
    let d = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["category"] == "missing_target")
        .unwrap();
    assert_eq!(d["key"], "broken");
    assert_eq!(d["source"]["role"], "primary");
    assert!(d["primary"].is_object());
    assert_eq!(d["occurrence_chain"][0]["profile"], "nested");
    assert_eq!(d["occurrence_chain"][1]["profile"], "patch");
    assert_eq!(snapshot(fixture.dir.path()), before);

    for tracked in [true, false] {
        let fixture = Fixture::new();
        if tracked {
            fs::write(fixture.repo.join(".gitignore"), "").unwrap();
            fs::write(
                fixture.repo.join(".grove.kdl"),
                "config { bind \"lead\" \"local\"; }\n",
            )
            .unwrap();
        } else {
            fs::create_dir(fixture.repo.join(".grove.kdl")).unwrap();
        }
        let before = snapshot(fixture.dir.path());
        let report = json_failure(fixture.show(&["--json"]), 1);
        assert_eq!(report["diagnostics"][0]["source"]["role"], "overlay");
        assert!(report["diagnostics"][0]["source"]["path"]
            .as_str()
            .unwrap()
            .ends_with(".grove.kdl"));
        assert_eq!(snapshot(fixture.dir.path()), before);
    }
    let output = fixture.show_at(fixture.dir.path(), &["--json"]);
    assert_eq!(
        json_failure(output, 1)["diagnostics"][0]["category"],
        "inspection"
    );
}

#[test]
fn json_secondary_workspace_uses_one_local_source_and_its_selection() {
    let fixture = Fixture::new();
    let secondary = fixture.dir.path().canonicalize().unwrap().join("secondary");
    support::jj(
        &fixture.repo,
        &["workspace", "add", secondary.to_str().unwrap()],
    );
    fs::write(secondary.join(".gitignore"), "/.grove.kdl\n").unwrap();
    fixture.personal(
        r#"config {
        command "a" "personal ${prompt}"
        command "repository" "repository ${prompt}"
        command "worktree" "worktree ${prompt}"
        bind "lead" "a"
        route "impl" "lead"
        profile "daily" {}
        select "daily"
    }"#,
    );
    fs::write(
        fixture.repo.join(".grove.kdl"),
        "config { bind \"lead\" \"repository\"; }\n",
    )
    .unwrap();
    let before = snapshot(fixture.dir.path());
    let report = json_success(fixture.show_at(&secondary, &["--json"]));
    assert_eq!(
        report["sources"][1]["path"],
        fixture.repo.join(".grove.kdl").to_str().unwrap()
    );
    assert_eq!(
        report["commands"][0]["words"][0]["word"]["value"],
        "repository"
    );
    assert_eq!(snapshot(fixture.dir.path()), before);
    fs::write(
        secondary.join(".grove.kdl"),
        "config { bind \"lead\" \"worktree\"; select; }\n",
    )
    .unwrap();
    let before = snapshot(fixture.dir.path());
    let report = json_success(fixture.show_at(&secondary, &["--json"]));
    assert_eq!(
        report["sources"][1]["path"],
        secondary.join(".grove.kdl").to_str().unwrap()
    );
    assert_eq!(report["selection"]["profiles"], serde_json::json!([]));
    assert_eq!(report["selection"]["origin"]["source"]["role"], "overlay");
    assert_eq!(
        report["commands"][0]["words"][0]["word"]["value"],
        "worktree"
    );
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[cfg(target_os = "linux")]
#[test]
fn json_native_paths_round_trip_in_sources_spans_and_failures() {
    use std::ffi::OsString;
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    let mut fixture = Fixture::new();
    let home = fixture
        .dir
        .path()
        .join(OsString::from_vec(b"home-\xff".to_vec()));
    fs::rename(&fixture.home, &home).unwrap();
    fixture.home = home;
    let expected = fixture.home.join(".config/grove/config.kdl");
    let encoded =
        serde_json::json!({"encoding": "unix_bytes", "value": expected.as_os_str().as_bytes()});
    let report = json_success(fixture.show(&["--json"]));
    assert_eq!(report["sources"][0]["path"], encoded);
    assert_eq!(report["origins"][0]["span"]["source"]["path"], encoded);
    fixture.personal(
        r#"config {
            command "invalid" "invalid-without-prompt"
            bind "lead" "invalid"
            route "impl" "lead"
        }"#,
    );
    let report = json_failure(fixture.show(&["--json"]), 1);
    assert_eq!(report["diagnostics"][0]["source"]["path"], encoded);
    assert_eq!(
        report["diagnostics"][0]["primary"]["source"]["path"],
        encoded
    );
}

#[cfg(unix)]
#[test]
fn json_tagged_words_filled_with_context_equal_fake_launch_argv() {
    use grove_loop::session_config::{DeltaRoots, ExpansionContext, SessionConfig};
    use std::ffi::OsString;
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    let fixture = Fixture::new();
    fs::write(
        fixture.repo.join("capture.sh"),
        "#!/bin/sh\nprintf '%s\\0' \"$@\" > captured.argv\n",
    )
    .unwrap();
    fixture.personal(r#"config {
        command "a" "/bin/sh capture.sh ${param.text} --mode=${param.mode} ${param.empty} ${prompt} ${session_name} ${worktree} ${repo}" {
            param "text" "${prompt} 'quoted'; $(no-shell)"
            param "mode" "medium"
            param "empty" ""
        }
        bind "lead" "a"
        route "impl" "lead"
        profile "high" { values "a" { param "mode" "high"; }; }
        select "high"
    }"#);
    let before = snapshot(fixture.dir.path());
    let report = json_success(fixture.show(&["--json", "--kind", "impl"]));
    assert_eq!(snapshot(fixture.dir.path()), before);
    let native = fixture
        .repo
        .join(OsString::from_vec(b"native-\xff".to_vec()));
    let context = ExpansionContext {
        prompt: "known prompt\nwith spaces",
        session_name: "known-session",
        worktree: &native,
        repository: &fixture.repo,
    };
    let filled: Vec<OsString> = report["commands"][0]["words"]
        .as_array()
        .unwrap()
        .iter()
        .map(|record| {
            let word = &record["word"];
            match word["type"].as_str().unwrap() {
                "literal" => OsString::from(word["value"].as_str().unwrap()),
                "slot" => match word["name"].as_str().unwrap() {
                    "prompt" => context.prompt.into(),
                    "session_name" => context.session_name.into(),
                    "worktree" => context.worktree.as_os_str().into(),
                    "repo" => context.repository.as_os_str().into(),
                    other => panic!("unknown slot: {other}"),
                },
                other => panic!("unknown word tag: {other}"),
            }
        })
        .collect();
    let config = SessionConfig::load(
        &fixture.home,
        &DeltaRoots {
            worktree: &fixture.repo,
            repository: &fixture.repo,
        },
    )
    .unwrap();
    let argv = config.expand("impl", &context).unwrap();
    assert_eq!(filled, argv.words());
    let output = Command::new(argv.program())
        .args(argv.args())
        .current_dir(&fixture.repo)
        .output()
        .unwrap();
    assert!(output.status.success());
    let expected: Vec<u8> = filled[2..]
        .iter()
        .flat_map(|word| word.as_bytes().iter().copied().chain([0]))
        .collect();
    assert_eq!(
        fs::read(fixture.repo.join("captured.argv")).unwrap(),
        expected
    );
    assert_eq!(filled[2], "${prompt} 'quoted'; $(no-shell)");
    assert_eq!(filled[3], "--mode=high");
    assert_eq!(filled[4], "");
}

#[cfg(unix)]
#[test]
fn json_native_missing_source_path_is_lossless_even_on_unicode_filesystems() {
    use std::ffi::OsString;
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    let mut fixture = Fixture::new();
    fixture.home = fixture
        .dir
        .path()
        .join(OsString::from_vec(b"missing-\xff".to_vec()));
    let expected = fixture.home.join(".config/grove/config.kdl");
    let before = snapshot(fixture.dir.path());
    let report = json_failure(fixture.show(&["--json"]), 1);
    assert_eq!(
        report["diagnostics"][0]["source"]["path"],
        serde_json::json!({"encoding": "unix_bytes", "value": expected.as_os_str().as_bytes()})
    );
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[test]
fn json_diagnostics_keep_related_locations_and_do_not_write_on_parse_failure() {
    let fixture = Fixture::new();
    fixture.personal(
        r#"config {
            command "agent" "one ${prompt}"
            bind "lead" "agent"
            route "impl" "lead"
            route "impl" "lead"
        }"#,
    );
    let before = snapshot(fixture.dir.path());
    let report = json_failure(fixture.show(&["--json"]), 1);
    let d = &report["diagnostics"][0];
    assert_eq!(d["category"], "duplicate");
    assert!(d["primary"]["start"].is_number());
    assert!(!d["related"].as_array().unwrap().is_empty());
    assert_eq!(d["related"][0]["source"], d["source"]);
    assert_eq!(snapshot(fixture.dir.path()), before);
}

#[cfg(unix)]
#[test]
fn json_handles_native_usage_errors_and_unprobeable_local_sources() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    let fixture = Fixture::new();
    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .args(["config", "show", "--json", "--kind"])
        .arg(OsString::from_vec(vec![255]))
        .output()
        .unwrap();
    assert_eq!(
        json_failure(output, 2)["diagnostics"][0]["category"],
        "usage"
    );
    std::os::unix::fs::symlink(".grove.kdl", fixture.repo.join(".grove.kdl")).unwrap();
    let report = json_failure(fixture.show(&["--json"]), 1);
    assert_eq!(report["diagnostics"][0]["source"]["role"], "overlay");
    assert!(report["diagnostics"][0]["source"]["path"]
        .as_str()
        .unwrap()
        .ends_with(".grove.kdl"));
    assert_eq!(
        fs::read_link(fixture.repo.join(".grove.kdl")).unwrap(),
        Path::new(".grove.kdl")
    );
}

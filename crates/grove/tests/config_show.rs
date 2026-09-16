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
        let repo = dir.path().join("repo");
        let home = dir.path().join("home");
        fs::create_dir_all(&repo).unwrap();
        fs::create_dir_all(home.join(".config/grove")).unwrap();
        support::init_jj_repo(&repo);
        fs::write(repo.join(".gitignore"), "/.grove.kdl\n").unwrap();
        let fixture = Self { dir, repo, home };
        fixture.personal("impl \"absent-agent ${prompt}\"\n");
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
    fixture
        .personal("impl \"sh -c 'touch launched' ${prompt}\"\nzeta \"absent-agent ${prompt}\"\n");
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
        "impl \"local-agent ${prompt}\"\n",
    )
    .unwrap();
    assert!(success(fixture.show_at(&subdir, &["--kind", "impl"])).contains("local-agent"));
    fixture.personal("impl \"absent-agent ${prompt}\"\ndesign \"invalid-without-prompt\"\n");
    let output = fixture.show(&["--kind", "impl"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(
        error.contains("design") && error.contains("config.kdl"),
        "{error}"
    );
}

#[test]
fn requested_unknown_and_non_admitted_kinds_fail_with_personal_source() {
    let fixture = Fixture::new();
    fs::write(
        fixture.repo.join(".grove.kdl"),
        "local-only \"absent-agent ${prompt}\"\n",
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
        "impl \"local-agent ${prompt}\"\n",
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

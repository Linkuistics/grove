//! The delivered set is checked independently against repository bytes.
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const FILES: &[&str] = &[
    "config.modular.example.kdl",
    "grove.codex-led.example.kdl",
    "grove.claude-led.example.kdl",
    "grove.high-effort.example.kdl",
    "grove.local-override.example.kdl",
    "CONFIGURATION.examples.md",
];

fn run(home: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_grove"))
        .args(["config", "examples"])
        .current_dir(home)
        .env("HOME", home)
        .env("GROVE_SIGNAL_FILE", home.join("stale-signal"))
        .env("PATH", "") // No workspace or executable discovery is needed.
        .output()
        .unwrap()
}

#[test]
fn installs_exact_bytes_without_policy_workspace_or_epoch_and_repeats_untouched() {
    for active in [None, Some("this is not valid KDL {")] {
        let home = tempfile::tempdir().unwrap();
        let destination = home.path().join(".config/grove");
        fs::create_dir_all(&destination).unwrap();
        if let Some(active) = active {
            fs::write(destination.join("config.kdl"), active).unwrap();
        }
        fs::write(destination.join("unrelated"), "keep me").unwrap();
        fs::write(home.path().join(".grove.kdl"), "keep delta").unwrap();
        fs::write(home.path().join("stale-signal"), "stale").unwrap();
        let retired_sample = destination.join("grove.legacy-override.example.kdl");
        if active.is_some() {
            fs::write(&retired_sample, "previously installed sample").unwrap();
        }
        let output = run(home.path());
        assert!(output.status.success(), "{output:?}");
        assert!(output.stderr.is_empty());
        let report = String::from_utf8(output.stdout).unwrap();
        let mut metadata = Vec::new();
        for name in FILES {
            let source = if *name == "CONFIGURATION.examples.md" {
                "README.md"
            } else {
                name
            };
            let source = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../docs/examples/modular-configuration")
                .join(source);
            let path = destination.join(name);
            assert_eq!(fs::read(&path).unwrap(), fs::read(source).unwrap());
            assert!(report.contains(path.to_str().unwrap()), "{report}");
            metadata.push(fs::metadata(path).unwrap().modified().unwrap());
        }
        let output = run(home.path());
        assert!(output.status.success(), "{output:?}");
        for (name, modified) in FILES.iter().zip(metadata) {
            assert_eq!(
                fs::metadata(destination.join(name))
                    .unwrap()
                    .modified()
                    .unwrap(),
                modified
            );
        }
        assert_eq!(
            fs::read_to_string(destination.join("config.kdl"))
                .ok()
                .as_deref(),
            active
        );
        assert_eq!(
            fs::read_to_string(destination.join("unrelated")).unwrap(),
            "keep me"
        );
        assert_eq!(
            fs::read_to_string(&retired_sample).ok().as_deref(),
            active.map(|_| "previously installed sample")
        );
        assert_eq!(
            fs::read_to_string(home.path().join(".grove.kdl")).unwrap(),
            "keep delta"
        );
        assert_eq!(
            fs::read_to_string(home.path().join("stale-signal")).unwrap(),
            "stale"
        );
        assert!(!home.path().join(".grove").exists());
        assert!(!home.path().join(".gitignore").exists());
    }
}

#[test]
fn all_conflicts_are_reported_before_any_missing_example_is_created() {
    let home = tempfile::tempdir().unwrap();
    let destination = home.path().join(".config/grove");
    fs::create_dir_all(destination.join(FILES[2])).unwrap();
    fs::write(destination.join(FILES[1]), "personal edits").unwrap();
    let output = run(home.path());
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    for name in &FILES[1..=2] {
        assert!(error.contains(name), "{error}");
    }
    assert!(!destination.join(FILES[0]).exists());
    assert_eq!(
        fs::read_to_string(destination.join(FILES[1])).unwrap(),
        "personal edits"
    );
    assert_eq!(fs::read_dir(destination).unwrap().count(), 2);
}

#[cfg(unix)]
#[test]
fn symlinks_and_unreadable_entries_are_conflicts() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let home = tempfile::tempdir().unwrap();
    let destination = home.path().join(".config/grove");
    fs::create_dir_all(&destination).unwrap();
    symlink(home.path().join("missing"), destination.join(FILES[1])).unwrap();
    fs::write(destination.join(FILES[2]), "unreadable").unwrap();
    fs::set_permissions(destination.join(FILES[2]), fs::Permissions::from_mode(0o0)).unwrap();
    let output = run(home.path());
    fs::set_permissions(
        destination.join(FILES[2]),
        fs::Permissions::from_mode(0o600),
    )
    .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let error = String::from_utf8(output.stderr).unwrap();
    for name in &FILES[1..=2] {
        assert!(error.contains(name), "{error}");
    }
    assert!(!destination.join(FILES[0]).exists());
    assert!(!home.path().join("missing").exists());
}

#[test]
fn help_and_usage_do_not_install_and_no_force_or_destination_option_exists() {
    let home = tempfile::tempdir().unwrap();
    for (args, code) in [
        (vec!["config", "examples", "--help"], 0),
        (vec!["config", "examples", "--force"], 2),
        (vec!["config", "examples", "somewhere"], 2),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_grove"))
            .args(args)
            .env("HOME", home.path())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(code), "{output:?}");
        assert!(!home.path().join(".config").exists());
    }
}

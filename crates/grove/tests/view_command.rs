use std::process::Command;

#[test]
fn view_help_defines_the_observation_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .args(["view", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    for expected in ["WORKTREE", "grove view", "no upward search", "subdirectory"] {
        assert!(help.contains(expected), "missing {expected}: {help}");
    }
}

#[test]
fn noninteractive_view_refuses_before_workspace_or_terminal_setup() {
    let directory = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .arg("view")
        .current_dir(directory.path())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("interactive stdin and stdout"), "{error}");
    assert!(!error.contains('\u{1b}'));
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
}

use std::process::Command;

fn invocation(directory: &std::path::Path, channel: &std::path::Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_grove-llm"));
    command
        .current_dir(directory)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_RUN_SIGNAL_FILE", channel);
    command
}

#[test]
fn standalone_completion_needs_no_workspace_and_only_signals_its_own_channel() {
    let dir = tempfile::tempdir().unwrap();
    let channel = keyed_launch::Channel::allocate(dir.path()).unwrap();
    let result = invocation(dir.path(), channel.path())
        .args(["complete", "--done"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    assert_eq!(channel.read().unwrap().as_str(), "done");
    assert!(!dir.path().join(".grove").exists());
    assert!(!dir.path().join(".jj").exists());
}

#[test]
fn standalone_context_refuses_tree_verbs_and_channel_redirection() {
    let dir = tempfile::tempdir().unwrap();
    let channel = keyed_launch::Channel::allocate(dir.path()).unwrap();
    let parent = dir.path().join("parent-channel");
    for args in [
        vec!["root-init"],
        vec!["complete"],
        vec![
            "complete",
            "--done",
            "--signal-file",
            parent.to_str().unwrap(),
        ],
    ] {
        let result = invocation(dir.path(), channel.path())
            .args(args)
            .output()
            .unwrap();
        assert!(!result.status.success(), "{result:?}");
        assert!(!channel.path().exists());
        assert!(!parent.exists());
        assert!(!dir.path().join(".grove").exists());
    }
    let conflict = invocation(dir.path(), channel.path())
        .env("GROVE_SIGNAL_FILE", &parent)
        .args(["complete", "--done"])
        .output()
        .unwrap();
    assert!(!conflict.status.success());
    assert!(!channel.path().exists());
    assert!(!parent.exists());
}

#[test]
fn copied_completion_helper_runs_inside_native_confinement() {
    use keyed_launch::{Confinement, Escalation, Launch, Templates, Vocabulary};
    use std::fs;
    use std::time::Duration;

    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let helper = root.join("grove-llm");
    fs::copy(env!("CARGO_BIN_EXE_grove-llm"), &helper).unwrap();
    let config = root.join("config.kdl");
    let command = format!("'{}' complete --done", helper.display());
    fs::write(&config, format!("config {{\ncommand \"helper\" {command:?}\nbind \"helper\" \"helper\"\nroute \"test\" \"helper\"\n}}\n")).unwrap();
    let templates = Templates::load(&config, None, Vocabulary { slots: &[] }).unwrap();
    let argv = templates.expand("test", &[]).unwrap();
    let channel = keyed_launch::Channel::allocate(&root).unwrap();
    let log = root.join("transcript");
    let ended = keyed_launch::run_confined(
        Launch {
            argv: &argv,
            channel: &channel,
            channel_var: "GROVE_RUN_SIGNAL_FILE",
            scrub: &[std::ffi::OsStr::new("GROVE_SIGNAL_FILE")],
            cwd: Some(&root),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::from_millis(100),
            },
        },
        fs::File::create(&log).unwrap(),
        &Confinement {
            writable: &root,
            runtime_read: &[],
        },
    )
    .unwrap();
    assert!(
        ended.status.success(),
        "{ended:?}: {}",
        fs::read_to_string(log).unwrap()
    );
    assert_eq!(ended.token.unwrap().as_str(), "done");
}

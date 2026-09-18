use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

#[test]
fn transcript_does_not_replay_terminal_control_sequences() {
    let mut displayed = Vec::new();
    super::render(b"hello\x1b]52;c;secret\x07\n", &mut displayed).unwrap();
    let displayed = String::from_utf8(displayed).unwrap();
    assert!(!displayed.contains('\x1b'));
    assert!(!displayed.contains('\x07'));
    assert!(displayed.starts_with("hello"));
    assert!(displayed.ends_with('\n'));
}

#[test]
#[ignore = "subprocess fixture isolates mux environment"]
fn pane_fixture() {
    let ui = match std::env::var("DISPLAY_TEST_UI").as_deref() {
        Ok("pane") => super::Ui::Pane,
        Ok("inline") => super::Ui::Inline,
        _ => super::Ui::Auto,
    };
    let result = super::open(
        ui,
        "release-notes",
        Path::new("/tmp/a 'log'"),
        Path::new("/tmp/status"),
    );
    assert_eq!(
        result.is_ok(),
        std::env::var("DISPLAY_EXPECT_FAILURE").is_err(),
        "{result:?}"
    );
}

#[test]
fn panes_use_host_mux_without_focus_and_fallback_is_explicit() {
    let dir = tempfile::tempdir().unwrap();
    let capture = dir.path().join("arguments");
    for mux in ["tmux", "zellij"] {
        let script = dir.path().join(mux);
        fs::write(&script, "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$DISPLAY_CAPTURE\"\nexit \"${DISPLAY_EXIT:-0}\"\n").unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    }
    for (mux, ui, failed, expected_failure) in [
        ("tmux", "auto", false, false),
        ("zellij", "pane", false, false),
        ("tmux", "auto", true, false),
        ("tmux", "pane", true, true),
        ("none", "auto", false, false),
        ("none", "pane", false, true),
        ("tmux", "inline", false, false),
    ] {
        let _ = fs::remove_file(&capture);
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "run_display::tests::pane_fixture",
                "--ignored",
                "--nocapture",
            ])
            .env_remove("TMUX")
            .env_remove("ZELLIJ")
            .env_remove("DISPLAY_EXPECT_FAILURE")
            .env("PATH", dir.path())
            .env("DISPLAY_CAPTURE", &capture)
            .env("DISPLAY_TEST_UI", ui)
            .env("DISPLAY_EXIT", if failed { "1" } else { "0" });
        if mux != "none" {
            command.env(if mux == "tmux" { "TMUX" } else { "ZELLIJ" }, "fixture");
        }
        if expected_failure {
            command.env("DISPLAY_EXPECT_FAILURE", "1");
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{mux}/{ui}: {output:?}");
        if mux == "none" || ui == "inline" {
            assert!(!capture.exists());
        } else {
            let args = fs::read_to_string(&capture).unwrap();
            assert!(args.contains("run-log"), "{args}");
            if mux == "tmux" {
                assert!(args.contains("\n-d\n"));
                let shell = args.lines().last().unwrap();
                let parsed = Command::new("/bin/sh")
                    .args(["-c", &format!("set -- {shell}; printf '%s\\n' \"$@\"")])
                    .output()
                    .unwrap();
                assert!(parsed.status.success());
                let parsed = String::from_utf8(parsed.stdout).unwrap();
                let words: Vec<_> = parsed.lines().collect();
                assert_eq!(&words[1..], &["run-log", "/tmp/a 'log'", "/tmp/status"]);
            } else {
                assert!(args.contains("\n--no-focus\n"));
                assert!(args.contains("\n/tmp/a 'log'\n"));
            }
            if failed && ui == "auto" {
                assert!(String::from_utf8_lossy(&output.stderr).contains("displaying inline"));
            }
        }
    }
}

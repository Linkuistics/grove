//! The retired completion verb grants no authority, even when given a path.

mod support;

use std::process::Command;

#[test]
fn the_retired_completion_verb_is_refused_without_writing_a_channel() {
    let tmp = tempfile::tempdir().unwrap();
    let channel = tmp.path().join("retired-channel");
    for args in [
        vec!["complete"],
        vec!["complete", "--done"],
        vec!["complete", "--signal-file", channel.to_str().unwrap()],
    ] {
        let output = Command::new(support::grove_llm())
            .args(args)
            .current_dir(tmp.path())
            .output()
            .unwrap();
        assert!(!output.status.success(), "retired completion succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unrecognized subcommand"),
            "{:?}",
            output
        );
        assert!(!channel.exists(), "the retired verb wrote its channel");
    }
}

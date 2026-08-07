use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use tempfile::TempDir;

const CURRENT_FORMAT: &str = "session-kinds-v1\n";

fn init_repo() -> TempDir {
    let temporary_directory = TempDir::new().unwrap();
    ProcessCommand::new("git")
        .arg("init")
        .arg("-q")
        .arg(temporary_directory.path())
        .status()
        .unwrap();
    temporary_directory
}

fn current_grove(repository: &Path) -> PathBuf {
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), CURRENT_FORMAT).unwrap();
    grove
}

fn write_leaf(grove: &Path, name: &str, body: &str) -> PathBuf {
    let path = grove.join(name);
    fs::write(&path, body).unwrap();
    path
}

fn grove_llm(repository: &Path, arguments: &[&str]) -> std::process::Output {
    Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repository)
        .args(arguments)
        .output()
        .unwrap()
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn filename_kind_uses_the_longest_known_prefix_and_body_routing_metadata_is_ignored() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    let leaf = write_leaf(
        &grove,
        "01-integrate-review-requirements-review-notes-k7.md",
        "# review-notes-k7\n\n**Kind:** impl\n**Harness:** codex\n",
    );

    let output = grove_llm(repository.path(), &["kind", leaf.to_str().unwrap()]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "integrate-review-requirements\n");
}

#[test]
fn current_tree_refuses_a_task_shaped_leaf_with_no_known_kind() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-untyped-k1.md", "# untyped-k1\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("01-untyped-k1.md"), "{error}");
    assert!(error.contains("requirements"), "{error}");
    assert!(error.contains("finish"), "{error}");
}

#[test]
fn current_tree_refuses_a_task_shaped_leaf_with_an_unknown_kind() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-mystery-untyped-k1.md", "# untyped-k1\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("01-mystery-untyped-k1.md"));
}

#[test]
fn pick_skips_finish_while_non_finish_work_is_live() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish-finish-k1.md", "# finish-k1\n");
    write_leaf(&grove, "02-impl-work-k2.md", "# work-k2\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("02-impl-work-k2.md"));
}

#[test]
fn pick_selects_finish_when_it_is_the_only_live_work() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish-finish-k1.md", "# finish-k1\n");
    write_leaf(&grove, "02-DONE-impl-finished-k2.md", "# finished-k2\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("01-finish-finish-k1.md"));
}

#[test]
fn pick_refuses_duplicate_live_finish_leaves() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish-finish-k1.md", "# finish-k1\n");
    write_leaf(
        &grove,
        "02-finish-finish-again-k2.md",
        "# finish-again-k2\n",
    );

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("multiple live `finish` leaves"));
}

#[test]
fn current_tree_reader_refuses_a_missing_format_witness() {
    let repository = init_repo();
    let grove = repository.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
    write_leaf(&grove, "01-impl-work-k1.md", "# work-k1\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("FORMAT"));
}

#[test]
fn current_tree_reader_refuses_an_unknown_format_witness() {
    let repository = init_repo();
    let grove = repository.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v2\n").unwrap();
    write_leaf(&grove, "01-impl-work-k1.md", "# work-k1\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("session-kinds-v2"), "{error}");
    assert!(error.contains("session-kinds-v1"), "{error}");
}

#[test]
fn terminal_infixes_preserve_filename_kind_and_stable_resolution() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    let done = write_leaf(
        &grove,
        "01-DONE-review-impl-reviewed-k7.md",
        "# reviewed-k7\n",
    );
    let abandoned = write_leaf(
        &grove,
        "02-ABANDONED-integrate-review-design-integrated-k8.md",
        "# integrated-k8\n",
    );

    for (path, expected) in [
        (&done, "review-impl\n"),
        (&abandoned, "integrate-review-design\n"),
    ] {
        let output = grove_llm(repository.path(), &["kind", path.to_str().unwrap()]);
        assert!(output.status.success(), "{}", stderr(&output));
        assert_eq!(stdout(&output), expected);
    }

    let output = grove_llm(repository.path(), &["resolve", "reviewed-k7"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("01-DONE-review-impl-reviewed-k7.md"));

    let output = grove_llm(repository.path(), &["resolve", "[8]"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("02-ABANDONED-integrate-review-design-integrated-k8.md"));
}

#[test]
fn malformed_terminal_task_names_fail_as_strictly_as_live_names() {
    for name in [
        "01-DONE-untyped-k1.md",
        "01-ABANDONED-mystery-untyped-k1.md",
    ] {
        let repository = init_repo();
        let grove = current_grove(repository.path());
        write_leaf(&grove, name, "# untyped-k1\n");

        let output = grove_llm(repository.path(), &["pick"]);

        assert!(!output.status.success(), "{name} was accepted");
        assert!(stderr(&output).contains(name), "{}", stderr(&output));
    }
}

#[test]
fn non_finish_work_can_be_inserted_before_a_reserved_finish_leaf() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish-finish-k1.md", "# finish-k1\n");

    let output = grove_llm(
        repository.path(),
        &["leaf-insert", "finish-k1", "late-work", "--kind", "impl"],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(grove.join("01-impl-late-work-k2.md").exists());
    assert!(grove.join("02-finish-finish-k1.md").exists());
}

#[test]
fn every_agent_side_mutation_refuses_the_driver_reserved_finish_kind() {
    let add_repository = init_repo();
    current_grove(add_repository.path());
    assert_finish_refusal(grove_llm(
        add_repository.path(),
        &["leaf-add", ".", "reserved", "--kind", "finish"],
    ));

    let insert_repository = init_repo();
    let insert_grove = current_grove(insert_repository.path());
    write_leaf(&insert_grove, "01-impl-target-k1.md", "# target-k1\n");
    assert_finish_refusal(grove_llm(
        insert_repository.path(),
        &["leaf-insert", "target-k1", "reserved", "--kind", "finish"],
    ));

    let chain_repository = init_repo();
    current_grove(chain_repository.path());
    assert_finish_refusal(grove_llm(
        chain_repository.path(),
        &["leaf-add-chain", ".", "reserved", "--kind", "finish"],
    ));

    for verb in [
        vec!["leaf-decompose", ".grove/01-finish-finish-k1.md", "child"],
        vec!["leaf-retire", ".grove/01-finish-finish-k1.md"],
        vec!["leaf-prune", ".grove/01-finish-finish-k1.md"],
        vec!["leaf-promote-chain", "1"],
    ] {
        let repository = init_repo();
        let grove = current_grove(repository.path());
        write_leaf(&grove, "01-finish-finish-k1.md", "# finish-k1\n");
        assert_finish_refusal(grove_llm(repository.path(), &verb));
        assert!(grove.join("01-finish-finish-k1.md").exists());
    }
}

fn assert_finish_refusal(output: std::process::Output) {
    assert!(!output.status.success(), "finish mutation succeeded");
    let error = stderr(&output);
    assert!(error.contains("finish"), "{error}");
    assert!(error.contains("driver-reserved"), "{error}");
}

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
fn filename_kind_is_unambiguous_and_body_routing_metadata_is_ignored() {
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

/// The kind set maintains a non-prefix invariant — no label plus `-` prefixes
/// another (`docs/specs/config-driven-sessions.md`, "the current leaf grammar").
/// `src/leaf.rs` pins that over the *set*; what the set cannot pin is that the
/// parser honours the `-` boundary. Without it `01-designer-notes-k1.md` reads
/// as kind `design` plus slug `er-notes` and launches a session no human wrote —
/// silently, because both halves are individually well-formed.
///
/// Stated in both directions, because one alone is satisfiable by a parser that
/// is simply wrong: refusing everything passes the first loop, and matching any
/// prefix passes the second.
#[test]
fn the_kind_label_boundary_is_exact_in_both_directions() {
    // A token that merely *starts with* a kind label is not that kind.
    for name in [
        "01-designer-notes-k1.md",
        "01-implementation-k1.md",
        "01-prototypes-spike-k1.md",
        // `review` and `integrate-review` are routing families, not members.
        "01-review-notes-k1.md",
        "01-integrate-review-notes-k1.md",
        // Standalone legacy `research` becomes `research-a` at migration; on a
        // current tree the bare spelling is not a kind.
        "01-research-survey-k1.md",
    ] {
        let repository = init_repo();
        let grove = current_grove(repository.path());
        write_leaf(&grove, name, "# notes-k1\n");

        let output = grove_llm(repository.path(), &["pick"]);

        assert!(!output.status.success(), "{name} was accepted as a leaf");
        assert!(stderr(&output).contains(name), "{}", stderr(&output));
    }

    // ...and a *slug* is free to begin with a kind label. All four coexist in
    // one tree, so a strictness failure on any single name fails the read.
    let repository = init_repo();
    let grove = current_grove(repository.path());
    let cases = [
        (
            "01-impl-review-design-notes-k1.md",
            "impl",
            "review-design-notes-k1",
        ),
        ("02-review-impl-design-k2.md", "review-impl", "design-k2"),
        (
            "03-integrate-review-impl-impl-k3.md",
            "integrate-review-impl",
            "impl-k3",
        ),
        ("04-design-research-a-k4.md", "design", "research-a-k4"),
    ];
    for (name, _, _) in cases {
        write_leaf(&grove, name, "# leaf\n");
    }

    for (name, kind, handle) in cases {
        let path = grove.join(name);
        let output = grove_llm(repository.path(), &["kind", path.to_str().unwrap()]);
        assert!(output.status.success(), "{name}: {}", stderr(&output));
        assert_eq!(stdout(&output), format!("{kind}\n"), "{name}");

        // The handle is what survives the tree, so the split has to leave the
        // *slug* right too — not merely name a kind that happens to parse.
        let output = grove_llm(repository.path(), &["resolve", handle]);
        assert!(output.status.success(), "{handle}: {}", stderr(&output));
        assert!(stdout(&output).contains(name), "{}", stdout(&output));
    }
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
fn current_tree_reader_exposes_a_missing_format_witness_newline() {
    let repository = init_repo();
    let grove = repository.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1").unwrap();
    write_leaf(&grove, "01-impl-work-k1.md", "# work-k1\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("found \"session-kinds-v1\""), "{error}");
    assert!(
        error.contains("requires \"session-kinds-v1\\n\""),
        "{error}"
    );
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

/// Finish is *reserved*, not *blocking*, and the spec states both halves:
/// `leaf-insert` sequences work ahead of it, and "ordinary `leaf-add` may also
/// append later work because finish selection cannot starve it"
/// (`docs/specs/config-driven-sessions.md`). The appended shape is the one that
/// bites — the finish leaf keeps the *earlier* position, so nothing but the skip
/// rule stops teardown being proposed while live work sits behind it.
#[test]
fn work_appended_behind_a_reserved_finish_leaf_is_still_selected() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish-finish-k1.md", "# finish-k1\n");

    let output = grove_llm(repository.path(), &["leaf-add", ".", "late-work"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        grove.join("02-impl-late-work-k2.md").exists(),
        "append must land behind the finish leaf"
    );
    assert!(
        grove.join("01-finish-finish-k1.md").exists(),
        "appending must not renumber the finish leaf"
    );

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("02-impl-late-work-k2.md"),
        "the later live leaf must outrank the earlier finish sentinel: {}",
        stdout(&output)
    );
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

/// A pending session-kind migration is a fail-closed malformed-tree condition
/// for **every** agent-side verb, reader and mutator alike, and it is checked
/// *before* the format witness. The ordering is the load-bearing half: the tree
/// a migration interrupted is legacy by definition, so a `FORMAT` complaint —
/// the failure waiting one step later — would send the operator to the wrong
/// recovery. `src/tree_access.rs` unit-proves one reader; this is the seam a
/// session actually calls, swept whole so a verb added later has to opt in.
#[test]
fn every_tree_verb_refuses_a_pending_migration_before_format_validation() {
    for arguments in [
        vec!["pick"],
        vec!["kind"],
        vec!["resolve", "task-k1"],
        vec!["brief-chain", ".grove/01-task-k1.md"],
        vec!["leaf-add", ".", "later"],
        vec!["leaf-insert", "task-k1", "earlier"],
        vec!["leaf-decompose", ".grove/01-task-k1.md", "first"],
        vec!["leaf-retire", ".grove/01-task-k1.md"],
        vec!["leaf-prune", ".grove/01-task-k1.md"],
        vec!["leaf-add-chain", ".", "stem", "--kind", "impl"],
        vec!["leaf-add-pair", ".", "stem"],
        vec!["leaf-promote-chain", "task-k1"],
    ] {
        let repository = init_repo();
        // Deliberately *not* `current_grove`: an interrupted migration leaves a
        // legacy tree, with no format witness to validate.
        let grove = repository.path().join(".grove");
        fs::create_dir_all(grove.join("MIGRATING-session-kinds")).unwrap();
        fs::write(grove.join("BRIEF.md"), "# demo — brief\n").unwrap();
        write_leaf(&grove, "01-task-k1.md", "# task-k1\n\n**Kind:** impl\n");
        let before = tree_snapshot(&grove);

        let output = grove_llm(repository.path(), &arguments);

        assert!(!output.status.success(), "{arguments:?} was admitted");
        let error = stderr(&output);
        assert!(
            error.contains("pending Grove session-kind migration"),
            "{arguments:?}: {error}"
        );
        assert!(
            error.contains("rerun bare `grove`"),
            "{arguments:?} named no recovery: {error}"
        );
        assert!(
            !error.contains("FORMAT"),
            "{arguments:?} reached format validation first: {error}"
        );
        assert_eq!(
            tree_snapshot(&grove),
            before,
            "{arguments:?} mutated the interrupted tree"
        );
    }
}

/// Every path under `directory`, relative and sorted, with file bodies — enough
/// to catch a refusing verb that still wrote something.
fn tree_snapshot(directory: &Path) -> Vec<(String, Option<Vec<u8>>)> {
    fn walk(root: &Path, directory: &Path, into: &mut Vec<(String, Option<Vec<u8>>)>) {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if path.is_dir() {
                into.push((relative, None));
                walk(root, &path, into);
            } else {
                into.push((relative, Some(fs::read(&path).unwrap())));
            }
        }
    }

    let mut entries = Vec::new();
    walk(directory, directory, &mut entries);
    entries.sort();
    entries
}

fn assert_finish_refusal(output: std::process::Output) {
    assert!(!output.status.success(), "finish mutation succeeded");
    let error = stderr(&output);
    assert!(error.contains("finish"), "{error}");
    assert!(error.contains("driver-reserved"), "{error}");
}

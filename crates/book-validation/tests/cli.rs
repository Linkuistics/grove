mod support;

use book_validation::cli::run_from;
use book_validation::BookSnapshot;

#[test]
fn help_documents_scoped_and_final_read_only_workflows() {
    let output = run_from(["book-check", "--help"]);

    assert_eq!(output.exit, 0);
    assert!(output.stdout.contains("--through read-path-k14"));
    assert!(output.stdout.contains("--final"));
    assert!(output.stdout.contains("read-only"));
    assert!(output.stdout.contains("Exit status"));
    assert!(output.stdout.contains("JSON"));
    assert!(output.stdout.contains("--check all"));
    assert!(output.stdout.contains("markdown"));
}

#[test]
fn markdown_and_all_are_accepted_check_selections() {
    let repository = tempfile::tempdir().unwrap();
    materialize(&support::corpus(false), repository.path());

    for check in ["markdown", "all"] {
        let output = run_from([
            "book-check",
            "--repo",
            repository.path().to_str().unwrap(),
            "--book",
            "docs/ordinal-fs-tree/book",
            "--through",
            "orientation-k11",
            "--check",
            check,
        ]);

        assert_ne!(output.exit, 2, "{check} must be a recognized check");
        assert!(!output.stderr.contains("invalid value"));
    }
}

#[test]
fn markdown_check_loads_a_deliberately_linked_repository_artifact() {
    let repository = tempfile::tempdir().unwrap();
    materialize(&support::corpus(false), repository.path());
    let book = repository.path().join("docs/ordinal-fs-tree/book");
    std::fs::write(
        book.join("README.md"),
        concat!(
            "# Ordinal filesystem tree\n",
            "<!-- book-page id=\"contents\" role=\"contents\" -->\n",
            "[Orientation](01-orientation.md)\n",
            "[Concept index](concept-index.md)\n",
            "[Source index](source-index.md)\n",
        ),
    )
    .unwrap();
    std::fs::write(
        book.join("concept-index.md"),
        concat!(
            "# Concept index\n",
            "<!-- book-page id=\"concept-index\" role=\"lookup\" -->\n",
            "[Contents](README.md)\n",
        ),
    )
    .unwrap();
    let orientation = book.join("01-orientation.md");
    let text = std::fs::read_to_string(&orientation).unwrap();
    let text = text.replacen(
        "<!-- book-page id=\"orientation\" slice=\"orientation-k11\" order=\"1\" -->\n",
        concat!(
            "<!-- book-page id=\"orientation\" slice=\"orientation-k11\" order=\"1\" -->\n",
            "[Contents](README.md)\n",
            "[Architecture](../ARCHITECTURE.md#boundary)\n",
        ),
        1,
    );
    std::fs::write(orientation, format!("{text}[Contents](README.md)\n")).unwrap();
    let source_index = book.join("source-index.md");
    let text = std::fs::read_to_string(&source_index).unwrap();
    let text = text.replacen(
        "<!-- book-page id=\"source-index\" role=\"lookup\" -->\n",
        concat!(
            "<!-- book-page id=\"source-index\" role=\"lookup\" -->\n",
            "[Contents](README.md)\n",
        ),
        1,
    );
    std::fs::write(source_index, text).unwrap();
    std::fs::write(
        repository
            .path()
            .join("docs/ordinal-fs-tree/ARCHITECTURE.md"),
        "# Architecture\n<a id=\"boundary\"></a>\n## Boundary\n",
    )
    .unwrap();

    let output = run_from([
        "book-check",
        "--repo",
        repository.path().to_str().unwrap(),
        "--book",
        "docs/ordinal-fs-tree/book",
        "--through",
        "orientation-k11",
        "--check",
        "markdown",
    ]);

    assert_eq!(output.exit, 0, "{}{}", output.stdout, output.stderr);
}

#[cfg(unix)]
#[test]
fn book_page_symlinks_cannot_read_outside_the_explicit_repository() {
    use std::os::unix::fs::symlink;

    let repository = tempfile::tempdir().unwrap();
    materialize(&support::corpus(false), repository.path());
    let outside = tempfile::tempdir().unwrap();
    let page = repository
        .path()
        .join("docs/ordinal-fs-tree/book/01-orientation.md");
    let bytes = std::fs::read(&page).unwrap();
    std::fs::write(outside.path().join("page.md"), bytes).unwrap();
    std::fs::remove_file(&page).unwrap();
    symlink(outside.path().join("page.md"), &page).unwrap();

    let output = run_from([
        "book-check",
        "--repo",
        repository.path().to_str().unwrap(),
        "--book",
        "docs/ordinal-fs-tree/book",
        "--through",
        "orientation-k11",
        "--check",
        "markdown",
    ]);

    assert_eq!(output.exit, 2);
    assert!(output.stderr.contains("outside-explicit-repository"));
}

#[cfg(unix)]
#[test]
fn fragment_only_check_does_not_load_markdown_link_targets() {
    use std::os::unix::fs::PermissionsExt as _;

    let repository = tempfile::tempdir().unwrap();
    materialize(&support::corpus(false), repository.path());
    let source_index = repository
        .path()
        .join("docs/ordinal-fs-tree/book/source-index.md");
    let mut text = std::fs::read_to_string(&source_index).unwrap();
    text.push_str("[Private](../private.md)\n");
    std::fs::write(source_index, text).unwrap();
    let private = repository.path().join("docs/ordinal-fs-tree/private.md");
    std::fs::write(&private, "secret\n").unwrap();
    std::fs::set_permissions(&private, std::fs::Permissions::from_mode(0o000)).unwrap();

    let output = run_from([
        "book-check",
        "--repo",
        repository.path().to_str().unwrap(),
        "--book",
        "docs/ordinal-fs-tree/book",
        "--through",
        "orientation-k11",
        "--check",
        "fragments",
    ]);

    assert_eq!(output.exit, 0, "{}{}", output.stdout, output.stderr);
}

#[test]
fn invalid_scope_is_exit_two_and_uses_json_when_requested() {
    let output = run_from([
        "book-check",
        "--repo",
        ".",
        "--book",
        "docs/ordinal-fs-tree/book",
        "--through",
        "book-assembly-k18",
        "--output",
        "json",
    ]);
    let value: serde_json::Value = serde_json::from_str(&output.stdout).unwrap();

    assert_eq!(output.exit, 2);
    assert!(output.stderr.is_empty());
    assert_eq!(value["status"], "invocation-error");
    assert_eq!(value["diagnostics"][0]["code"], "U001");
    assert_eq!(
        value["diagnostics"][0],
        serde_json::json!({
            "code": "U001",
            "phase": "parse",
            "message": value["diagnostics"][0]["message"],
            "primary": { "path": "<command>", "byte": 0, "line": null, "column": null },
            "fragment_id": null,
            "root_id": null,
            "source": null,
            "related": [],
            "remedy": "run `book-check --help` for accepted arguments"
        })
    );
}

#[test]
fn load_failure_has_path_category_and_complete_json_schema() {
    let temporary = tempfile::tempdir().unwrap();
    let output = run_from([
        "book-check",
        "--repo",
        temporary.path().to_str().unwrap(),
        "--book",
        "docs/ordinal-fs-tree/book",
        "--final",
        "--output",
        "json",
    ]);
    let value: serde_json::Value = serde_json::from_str(&output.stdout).unwrap();
    let diagnostic = &value["diagnostics"][0];

    assert_eq!(output.exit, 2);
    assert_eq!(diagnostic["code"], "U002");
    assert_eq!(diagnostic["primary"]["path"], "docs/ordinal-fs-tree/book");
    assert!(diagnostic["message"]
        .as_str()
        .unwrap()
        .contains("not-found"));
    assert!(diagnostic["source"].is_null());
    assert_eq!(diagnostic["related"], serde_json::json!([]));
}

#[test]
fn final_fragment_command_loads_only_the_explicit_repository() {
    let temporary = tempfile::tempdir().unwrap();
    materialize(&support::corpus(true), temporary.path());
    let output = run_from([
        "book-check",
        "--repo",
        temporary.path().to_str().unwrap(),
        "--book",
        "docs/ordinal-fs-tree/book",
        "--final",
        "--check",
        "fragments",
        "--output",
        "json",
    ]);
    let value: serde_json::Value = serde_json::from_str(&output.stdout).unwrap();

    assert_eq!(output.exit, 0, "{}{}", output.stdout, output.stderr);
    assert_eq!(value["status"], "valid");
    assert_eq!(value["coverage"]["files"], 15);
    assert_eq!(value["coverage"]["resolved_lines"], 6_618);
}

fn materialize(snapshot: &BookSnapshot, repository: &std::path::Path) {
    for (path, bytes) in snapshot.book_files.iter().chain(&snapshot.source_files) {
        let destination = repository.join(path);
        std::fs::create_dir_all(destination.parent().unwrap()).unwrap();
        std::fs::write(destination, bytes).unwrap();
    }
}

#[test]
fn parent_components_in_book_path_are_rejected_without_loading() {
    let output = run_from([
        "book-check",
        "--repo",
        ".",
        "--book",
        "docs/../book",
        "--final",
    ]);

    assert_eq!(output.exit, 2);
    assert!(output.stderr.contains("normalized repository-relative"));
}

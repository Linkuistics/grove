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
    assert!(output.stdout.contains("markdown-validation-k9"));
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

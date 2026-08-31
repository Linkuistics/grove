//! A second, synthetic book.
//!
//! The generalisation `validator-structure-k21` performs is not done if the
//! suite can only be run against the one book whose structure used to be
//! compiled in. This fixture shares nothing with that book: a different
//! directory, title, page inventory, page identifiers, slice tokens and slice
//! order. Everything it exercises — the inventory, the canonical identity
//! lines, the H1 titles, the navigation labels, the contents links and the
//! accepted `--through` domain — is read from its own `walkthrough.toml`.
//!
//! It runs `--check markdown`. The fragment path still reads the compiled
//! corpus constants (`ROOTS`, `BLOCKS`, `EARLY_USES`), which
//! `validator-fragments-k22` turns into per-book data; when it does, this
//! fixture grows a corpus and the assertions below move to `--check all`.

use std::path::Path;

use book_validation::cli::run_from;

const BOOK: &str = "docs/walkthroughs/widget-guide";

const MANIFEST: &str = r#"schema = 1

[book]
id      = "widget-guide"
title   = "Widget guide"
subject = "crates/widget"

[corpus]
include = ["crates/widget/Cargo.toml", "crates/widget/src/**/*.rs"]

[[page]]
file  = "README.md"
id    = "contents"
title = "Widget guide"
role  = "contents"

[[page]]
file  = "01-first-look.md"
id    = "first-look"
title = "First look"
role  = "chapter"
slice = "first-look-w1"

[[page]]
file  = "02-inner-workings.md"
id    = "inner-workings"
title = "Inner workings"
role  = "chapter"
slice = "inner-workings-w2"

[[page]]
file  = "03-afterword.md"
id    = "afterword"
title = "Afterword"
role  = "chapter"
slice = "afterword-w3"

[[page]]
file  = "concept-index.md"
id    = "concept-index"
title = "Concept index"
role  = "lookup"

[[page]]
file  = "source-index.md"
id    = "source-index"
title = "Source index"
role  = "lookup"

[[root]]
id    = "source-widget"
path  = "crates/widget/src/lib.rs"
lines = 6

[[block]]
id    = "widget-opening"
root  = "source-widget"
owner = "first-look-w1"
lines = "1-3"

[[block]]
id    = "widget-body"
root  = "source-widget"
owner = "inner-workings-w2"
lines = "4-6"

[guide]
omitted = "the widget guide is a fixture and cites no user guide"
"#;

const CHAPTERS: [(&str, &str, &str); 3] = [
    ("01-first-look.md", "first-look", "First look"),
    ("02-inner-workings.md", "inner-workings", "Inner workings"),
    ("03-afterword.md", "afterword", "Afterword"),
];

const SLICES: [&str; 3] = ["first-look-w1", "inner-workings-w2", "afterword-w3"];

/// Write the book with `chapters` numbered pages present; the rest are named in
/// the contents as plain text, which is what a scoped prefix requires.
fn materialize(repository: &Path, chapters: usize) {
    let source = repository.join("crates/widget/src");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::write(
        source.join("lib.rs"),
        "pub struct Widget;\n\nimpl Widget {\n    pub fn new() -> Self {\n        Self\n    }\n",
    )
    .unwrap();
    let book = repository.join(BOOK);
    std::fs::create_dir_all(&book).unwrap();
    write(&book, "walkthrough.toml", MANIFEST);

    let mut contents =
        String::from("# Widget guide\n<!-- book-page id=\"contents\" role=\"contents\" -->\n\n");
    for (index, (file, _, title)) in CHAPTERS.iter().enumerate() {
        if index < chapters {
            contents.push_str(&format!("[{title}]({file})\n"));
        } else {
            contents.push_str(&format!("{title} — planned.\n"));
        }
    }
    contents.push_str("[Concept index](concept-index.md)\n[Source index](source-index.md)\n");
    write(&book, "README.md", &contents);

    for (index, (file, id, title)) in CHAPTERS.iter().take(chapters).enumerate() {
        let mut parts = Vec::new();
        if let Some((previous, _, previous_title)) = index.checked_sub(1).map(|at| CHAPTERS[at]) {
            parts.push(format!("[Previous: {previous_title}]({previous})"));
        }
        parts.push("[Contents](README.md)".to_owned());
        if index + 1 < chapters {
            let (next, _, next_title) = CHAPTERS[index + 1];
            parts.push(format!("[Next: {next_title}]({next})"));
        }
        let navigation = parts.join(" | ");
        write(
            &book,
            file,
            &format!(
                "# {title}\n<!-- book-page id=\"{id}\" slice=\"{}\" order=\"{}\" -->\n{navigation}\n\nThis chapter carries prose and no source fragments.\n\n{navigation}\n",
                SLICES[index],
                index + 1
            ),
        );
    }
    write(
        &book,
        "concept-index.md",
        "# Concept index\n<!-- book-page id=\"concept-index\" role=\"lookup\" -->\n\n[Contents](README.md)\n",
    );
    write(
        &book,
        "source-index.md",
        "# Source index\n<!-- book-page id=\"source-index\" role=\"lookup\" -->\n\n[Contents](README.md)\n",
    );
}

fn write(book: &Path, file: &str, text: &str) {
    std::fs::write(book.join(file), text).unwrap();
}

fn check(repository: &Path, scope: &[&str]) -> book_validation::cli::RunOutput {
    let mut arguments = vec![
        "book-check".to_owned(),
        "--repo".to_owned(),
        repository.to_str().unwrap().to_owned(),
        "--book".to_owned(),
        BOOK.to_owned(),
    ];
    arguments.extend(scope.iter().map(|value| (*value).to_owned()));
    arguments.extend(["--check".to_owned(), "markdown".to_owned()]);
    run_from(arguments)
}

#[test]
fn a_second_book_validates_from_its_own_manifest() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 0, "{}{}", output.stdout, output.stderr);
}

/// A slice that owns no `[[block]]` has no prefix to prove: it is declared, it
/// is a chapter, and it is still not a `--through` value. The compiled
/// `value_parser` this replaces rejected such a slice by omission from a
/// hand-written list; the derivation has to reach the same answer from the
/// blocks.
#[test]
fn a_declared_slice_that_owns_no_source_is_final_only() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);

    let final_ = check(repository.path(), &["--final"]);
    let scoped = check(repository.path(), &["--through", "afterword-w3"]);

    assert_eq!(final_.exit, 0, "{}{}", final_.stdout, final_.stderr);
    assert_eq!(scoped.exit, 2);
    assert!(scoped.stderr.contains("U001"), "{}", scoped.stderr);
    let accepted = scoped
        .stderr
        .split_once("accepted values are ")
        .expect("the diagnostic lists the accepted values")
        .1;
    assert!(
        !accepted.contains("afterword-w3"),
        "a final-only slice is not among the accepted values: {accepted}"
    );
}

#[test]
fn a_second_book_scopes_to_the_prefix_its_own_manifest_declares() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 1);

    let output = check(repository.path(), &["--through", "first-look-w1"]);

    assert_eq!(output.exit, 0, "{}{}", output.stdout, output.stderr);
}

#[test]
fn the_scoped_domain_is_the_named_books_and_no_other_books() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);

    let output = check(repository.path(), &["--through", "orientation-k11"]);

    assert_eq!(output.exit, 2);
    assert!(output.stderr.contains("U001"), "{}", output.stderr);
    assert!(
        output.stderr.contains("widget-guide"),
        "the diagnostic names the book: {}",
        output.stderr
    );
    assert!(
        output.stderr.contains("first-look-w1") && output.stderr.contains("inner-workings-w2"),
        "the diagnostic lists the book's accepted values: {}",
        output.stderr
    );
}

#[test]
fn a_page_the_manifest_does_not_declare_is_an_inventory_finding() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);
    std::fs::write(
        repository.path().join(BOOK).join("03-extra.md"),
        "# Extra\n",
    )
    .unwrap();

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 1);
    assert!(output.stdout.contains("M101"), "{}", output.stdout);
}

#[test]
fn a_schema_invalid_manifest_is_a_u002_load_failure_naming_the_reason() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);
    let manifest = repository.path().join(BOOK).join("walkthrough.toml");
    let text = std::fs::read_to_string(&manifest).unwrap();
    std::fs::write(&manifest, text.replace("schema = 1", "schema = 7")).unwrap();

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 2);
    assert!(output.stderr.contains("U002"), "{}", output.stderr);
    assert!(
        output.stderr.contains("walkthrough.toml") && output.stderr.contains("schema 1"),
        "{}",
        output.stderr
    );
}

#[test]
fn a_book_directory_without_a_manifest_is_a_u002_load_failure() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);
    std::fs::remove_file(repository.path().join(BOOK).join("walkthrough.toml")).unwrap();

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 2);
    assert!(
        output.stderr.contains("U002") && output.stderr.contains("book manifest"),
        "{}",
        output.stderr
    );
}

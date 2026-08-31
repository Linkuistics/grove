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
//! It runs `--check all`. Since `validator-fragments-k22` the fragment path is
//! per-book data too, so this fixture carries a corpus of its own: a source
//! root, two ownership blocks, an inline test module it excludes, and literal
//! fragments on its chapters that reconstruct the file byte for byte. That is
//! what makes the suite evidence about the *validator* rather than about the
//! one book whose corpus it used to be compiled with.

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

[[corpus.exclude]]
path   = "crates/widget/src/tests.rs"
class  = "inline-test-module"
reason = "inline test module: evidence, not production source"

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

[[early-use]]
symbols   = "`Widget`"
first-use = "01-first-look.md#vocabulary"
owner     = "inner-workings-w2"
statement = "A widget is the fixture's one value and its constructor is explained later."

[guide]
omitted = "the widget guide is a fixture and cites no user guide"
"#;

/// The fixture crate's one source file, and the only bytes the book
/// reconstructs. Six lines, partitioned `1-3` and `4-6` by the two blocks.
const WIDGET_SOURCE: &str =
    "pub struct Widget;\n\nimpl Widget {\n    pub fn new() -> Self {\n        Self\n    }\n";

/// `(block id, owner slice, first line, last line)`, in manifest order.
const BLOCKS: [(&str, &str, usize, usize); 2] = [
    ("widget-opening", "first-look-w1", 1, 3),
    ("widget-body", "inner-workings-w2", 4, 6),
];

const ROOT_ID: &str = "source-widget";
const ROOT_PATH: &str = "crates/widget/src/lib.rs";
const ROOT_LINES: usize = 6;

fn source_lines(first: usize, last: usize) -> String {
    WIDGET_SOURCE
        .split_inclusive('\n')
        .skip(first - 1)
        .take(last - first + 1)
        .collect()
}

/// The chapter index a slice sits at, which is what decides whether a block is
/// resolved or deferred under a given prefix.
fn slice_index(slice: &str) -> usize {
    SLICES
        .iter()
        .position(|candidate| *candidate == slice)
        .expect("every block owner is a declared chapter slice")
}

/// The book's `source-index.md`: the four ledger tables and the source-root
/// directive block, with every block resolved or deferred according to how many
/// chapters have landed.
fn source_index(chapters: usize) -> String {
    let resolved = |owner: &str| slice_index(owner) < chapters;
    let mut text = String::from(
        "# Source index\n<!-- book-page id=\"source-index\" role=\"lookup\" -->\n[Contents](README.md)\n\n## Source roots\n\n| Root ID | Source path | Lines |\n|---|---|---|\n",
    );
    text.push_str(&format!(
        "| `{ROOT_ID}` | `{ROOT_PATH}` | {ROOT_LINES} |\n\n"
    ));
    text.push_str(&format!(
        "<!-- source-root «{ROOT_ID}» source=\"{ROOT_PATH}\" lines=\"1-{ROOT_LINES}\" -->\n"
    ));
    for (id, owner, first, last) in BLOCKS {
        if resolved(owner) {
            text.push_str(&format!("<!-- insert «{id}» -->\n"));
        } else {
            text.push_str(&format!(
                "<!-- defer «{id}» owner=\"{owner}\" lines=\"{first}-{last}\" -->\n"
            ));
        }
    }
    text.push_str("<!-- /source-root -->\n");

    text.push_str("\n## Ownership blocks\n\n| Block ID | Root ID | Owner | Source lines | Count | State |\n|---|---|---|---|---|---|\n");
    for (id, owner, first, last) in BLOCKS {
        let state = if resolved(owner) {
            "resolved"
        } else {
            "deferred"
        };
        text.push_str(&format!(
            "| `{id}` | `{ROOT_ID}` | `{owner}` | `{first}-{last}` | {} | `{state}` |\n",
            last - first + 1
        ));
    }

    text.push_str("\n## Fragment index\n\n| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |\n|---|---|---|---|---|---|---|---|\n");
    let children = BLOCKS
        .iter()
        .map(|(id, ..)| format!("`{id}`"))
        .collect::<Vec<_>>()
        .join(", ");
    text.push_str(&format!(
        "| `{ROOT_ID}` | `source-index` | `{ROOT_ID}` | `root` | `—` | `1-{ROOT_LINES}` | `—` | {children} |\n"
    ));
    for (id, owner, first, last) in BLOCKS.iter().filter(|(_, owner, ..)| resolved(owner)) {
        let (_, page_id, _) = CHAPTERS[slice_index(owner)];
        text.push_str(&format!(
            "| `{id}` | `{page_id}` | `{ROOT_ID}` | `literal` | `{owner}` | `{first}-{last}` | `{ROOT_ID}` | `—` |\n"
        ));
    }

    text.push_str("\n## Early uses\n\n| Symbol family | First use | Owner | Minimum local statement | Status |\n|---|---|---|---|---|\n");
    let status = if resolved("inner-workings-w2") {
        "explained"
    } else {
        "pending"
    };
    text.push_str(&format!(
        "| `Widget` | `01-first-look.md#vocabulary` | `inner-workings-w2` | A widget is the fixture's one value and its constructor is explained later. | `{status}` |\n"
    ));
    text
}

/// The literal fragments a chapter carries, each introduced by the prose
/// paragraph `M105` requires.
fn chapter_fragments(slice: &str) -> String {
    let mut text = String::new();
    for (id, owner, first, last) in BLOCKS.iter().filter(|(_, owner, ..)| *owner == slice) {
        text.push_str(&format!(
            "\nThe fragment below is lines {first} to {last} of the fixture crate.\n\n<!-- fragment «{id}» owner=\"{owner}\" source=\"{ROOT_PATH}\" lines=\"{first}-{last}\" parent=\"{ROOT_ID}\" -->\n````rust\n{}````\n<!-- /fragment -->\n",
            source_lines(*first, *last)
        ));
    }
    text
}

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
    std::fs::write(source.join("lib.rs"), WIDGET_SOURCE).unwrap();
    // The excluded inline test module has to exist: an exclusion that excludes
    // nothing is `U002`, so a fixture that skipped this file would be
    // exercising that refusal rather than the book.
    std::fs::write(
        source.join("tests.rs"),
        "// evidence, not production source\n",
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
        let anchor = if index == 0 {
            "<a id=\"vocabulary\"></a>\n## Vocabulary\n\nA widget is the fixture's one value.\n"
        } else {
            ""
        };
        write(
            &book,
            file,
            &format!(
                "# {title}\n<!-- book-page id=\"{id}\" slice=\"{}\" order=\"{}\" -->\n{navigation}\n\nThis chapter opens the fixture crate.\n\n{anchor}{}\n{navigation}\n",
                SLICES[index],
                index + 1,
                chapter_fragments(SLICES[index])
            ),
        );
    }
    write(
        &book,
        "concept-index.md",
        "# Concept index\n<!-- book-page id=\"concept-index\" role=\"lookup\" -->\n\n[Contents](README.md)\n",
    );
    write(&book, "source-index.md", &source_index(chapters));
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
    arguments.extend(["--check".to_owned(), "all".to_owned()]);
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

/// The control for every assertion above.
///
/// `--check all` is only evidence about the fragment path if the fragment path
/// can fail here, and a validator that had kept any compiled knowledge of the
/// relocated book would pass this fixture by never reading its bytes at all.
/// One byte of the fixture crate changes, and the book that reproduces the old
/// bytes is wrong about the source.
#[test]
fn a_source_byte_the_book_does_not_reproduce_is_an_f008() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);
    std::fs::write(
        repository.path().join(ROOT_PATH),
        WIDGET_SOURCE.replace("pub struct Widget;", "pub struct Gadget;"),
    )
    .unwrap();

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 1, "{}{}", output.stdout, output.stderr);
    assert!(output.stdout.contains("F008"), "{}", output.stdout);
}

/// The corpus rule's external half, in the direction the compiled ledger could
/// never see: a production file added to the crate and forgotten by the book.
///
/// Nothing the author wrote mentions `extra.rs`. The manifest does not declare
/// it, no page reconstructs it, and every other check in the validator reads
/// only those two artifacts — so before derivation this file was invisible to
/// the whole suite and the book still claimed complete reconstruction.
#[test]
fn a_file_the_corpus_rule_reaches_and_the_manifest_does_not_declare_is_an_f006() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);
    std::fs::write(
        repository.path().join("crates/widget/src/extra.rs"),
        "pub const EXTRA: u8 = 1;\n",
    )
    .unwrap();

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 1, "{}{}", output.stdout, output.stderr);
    assert!(
        output.stdout.contains("F006") && output.stdout.contains("crates/widget/src/extra.rs"),
        "{}",
        output.stdout
    );
}

/// And the other direction: a declared root the rule does not reach, which no
/// `[[corpus.add]]` accounts for. This is the shape a book takes when its
/// author narrows the rule to fit the roots rather than the other way round.
#[test]
fn a_declared_root_the_corpus_rule_does_not_reach_is_an_f006() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);
    std::fs::write(
        repository.path().join("crates/widget/outside.rs"),
        WIDGET_SOURCE,
    )
    .unwrap();
    edit_manifest(repository.path(), ROOT_PATH, "crates/widget/outside.rs");

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 1, "{}{}", output.stdout, output.stderr);
    assert!(
        output.stdout.contains("F006") && output.stdout.contains("crates/widget/outside.rs"),
        "{}",
        output.stdout
    );
}

/// A stale exception is a failure rather than a silent no-op: the excluded file
/// is gone, so the exclusion now describes a rule that has moved.
#[test]
fn a_corpus_exception_naming_a_missing_file_is_a_u002() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);
    std::fs::remove_file(repository.path().join("crates/widget/src/tests.rs")).unwrap();

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 2);
    assert!(
        output.stderr.contains("U002") && output.stderr.contains("crates/widget/src/tests.rs"),
        "{}",
        output.stderr
    );
}

/// An exclusion the include patterns never matched excludes nothing. It is
/// either a typo or a rule that has moved, and both are worth a refusal — a
/// silent no-op would let the boundary rot in place while the book reads green.
#[test]
fn an_exclusion_the_include_patterns_never_matched_is_a_u002() {
    let repository = tempfile::tempdir().unwrap();
    materialize(repository.path(), 3);
    std::fs::write(repository.path().join("crates/widget/tests.rs"), "\n").unwrap();
    edit_manifest(
        repository.path(),
        "crates/widget/src/tests.rs",
        "crates/widget/tests.rs",
    );

    let output = check(repository.path(), &["--final"]);

    assert_eq!(output.exit, 2);
    assert!(
        output.stderr.contains("U002") && output.stderr.contains("excludes nothing"),
        "{}",
        output.stderr
    );
}

fn edit_manifest(repository: &Path, from: &str, to: &str) {
    let manifest = repository.join(BOOK).join("walkthrough.toml");
    let text = std::fs::read_to_string(&manifest).unwrap();
    assert!(text.contains(from), "fixture manifest lacks `{from}`");
    std::fs::write(&manifest, text.replacen(from, to, 1)).unwrap();
}

//! The manifest is a schema, and a manifest that breaks it is a refusal.
//!
//! Unknown keys are rejected rather than ignored: a mistyped field that is
//! silently dropped is an obligation that silently disappears, which is the
//! failure the manifest exists to prevent.

mod support;

use book_validation::Manifest;

const BOOK_ROOT: &str = "docs/walkthroughs/ordinal-fs-tree";

fn reason(text: &str) -> String {
    Manifest::load(BOOK_ROOT, text)
        .expect_err("manifest is expected to be rejected")
        .reason()
        .to_owned()
}

fn edited(from: &str, to: &str) -> String {
    let text = support::manifest_text();
    assert!(text.contains(from), "fixture manifest lacks `{from}`");
    text.replacen(from, to, 1)
}

#[test]
fn the_fixture_manifest_is_schema_valid() {
    let manifest = support::manifest();

    assert_eq!(manifest.book_root(), BOOK_ROOT);
    assert_eq!(manifest.book_id(), "ordinal-fs-tree");
    // `subject` is read back rather than merely consumed: it is the corpus
    // rule's floor, and the repository's subject-inventory test compares this
    // value against a document the book does not own.
    assert_eq!(manifest.subject(), "crates/ordinal-fs-tree");
    assert_eq!(manifest.chapter_count(), 7);
}

#[test]
fn the_relocated_books_own_manifest_loads() {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(BOOK_ROOT)
            .join("walkthrough.toml"),
    )
    .unwrap();

    let manifest = Manifest::load(BOOK_ROOT, &text).unwrap();

    // Eight chapters, seven of which own source and are therefore scopeable;
    // the eighth owns none, so it has no prefix to prove and is final-only.
    assert_eq!(manifest.chapter_count(), 8);
    assert_eq!(manifest.scoped_slices().len(), 7);
    assert!(!manifest
        .scoped_slices()
        .contains(&"book-assembly-k18".to_owned()));
}

#[test]
fn an_unknown_key_is_refused_rather_than_ignored() {
    let text = edited("[book]\n", "[book]\naudience = \"nobody\"\n");

    assert!(reason(&text).contains("audience"), "{}", reason(&text));
}

#[test]
fn an_unsupported_schema_version_names_the_supported_one() {
    let text = edited("schema = 1", "schema = 2");

    assert!(reason(&text).contains("schema 1"), "{}", reason(&text));
}

#[test]
fn a_book_id_that_is_not_the_directory_name_is_refused() {
    let text = edited("id = \"ordinal-fs-tree\"", "id = \"other-book\"");

    assert!(
        reason(&text).contains("directory name"),
        "{}",
        reason(&text)
    );
}

#[test]
fn a_duplicate_page_identity_names_the_repeated_value() {
    let text = edited("id = \"name-seam\"", "id = \"orientation\"");

    assert!(
        reason(&text).contains("declared twice"),
        "{}",
        reason(&text)
    );
}

#[test]
fn a_chapter_without_a_slice_is_refused() {
    let text = edited(
        "role = \"chapter\"\nslice = \"orientation-k11\"\n",
        "role = \"chapter\"\n",
    );

    assert!(reason(&text).contains("slice"), "{}", reason(&text));
}

#[test]
fn a_lookup_page_declaring_a_slice_is_refused() {
    let text = edited(
        "file = \"concept-index.md\"\nid = \"concept-index\"\ntitle = \"Concept index\"\nrole = \"lookup\"\n",
        "file = \"concept-index.md\"\nid = \"concept-index\"\ntitle = \"Concept index\"\nrole = \"lookup\"\nslice = \"smuggled-k99\"\n",
    );

    assert!(
        reason(&text).contains("only a chapter"),
        "{}",
        reason(&text)
    );
}

#[test]
fn a_block_owned_by_no_chapter_is_refused() {
    let text = edited(
        "owner = \"orientation-k11\"\nlines = \"1-42\"",
        "owner = \"nobody-k99\"\nlines = \"1-42\"",
    );

    assert!(
        reason(&text).contains("no chapter's slice"),
        "{}",
        reason(&text)
    );
}

#[test]
fn a_block_naming_no_declared_root_is_refused() {
    let text = edited(
        "root = \"source-crate-manifest\"",
        "root = \"source-imaginary\"",
    );

    assert!(
        reason(&text).contains("undeclared root"),
        "{}",
        reason(&text)
    );
}

/// The guide and the glossaries are *retained*, not merely checked at load:
/// the CLI loads the documents they name and the anchor checks read the
/// anchors they reserve, so a schema that validated and discarded them would
/// leave both obligations with no data to run on.
#[test]
fn a_declared_guide_and_glossary_are_retained_as_outbound_documents() {
    let text = edited(
        "[guide]\nomitted = \"the fixture book is self-contained\"",
        "[guide]\npath = \"docs/USAGE.md\"\nanchors = [\"scaffolding-a-grove\"]\n\n[[glossary]]\npath = \"docs/ordinal-fs-tree/CONTEXT.md\"\nanchors = [\"entry\", \"ordinal\"]",
    );

    let manifest = Manifest::load(BOOK_ROOT, &text).expect("manifest is schema-valid");

    let guide = manifest
        .guide()
        .declared()
        .expect("the edited manifest declares a guide path");
    assert_eq!(guide.path(), "docs/USAGE.md");
    assert_eq!(guide.anchors(), ["scaffolding-a-grove"]);
    assert_eq!(manifest.glossaries().len(), 1);
    assert_eq!(manifest.glossaries()[0].anchors(), ["entry", "ordinal"]);
    // The guide first, then each glossary: the one iterator the loader, the
    // permitted-target set and the anchor check all read.
    let paths: Vec<&str> = manifest
        .outbound_documents()
        .map(book_validation::OutboundDocument::path)
        .collect();
    assert_eq!(paths, ["docs/USAGE.md", "docs/ordinal-fs-tree/CONTEXT.md"]);
}

/// The relocated book's exemption, read back from its own manifest: it declares
/// no guide path, so it declares no outbound document and the guide is not a
/// permitted link target for it.
#[test]
fn the_fixture_manifest_declares_the_guide_omitted() {
    let manifest = support::manifest();

    assert!(manifest.guide().declared().is_none());
    assert_eq!(manifest.outbound_documents().count(), 0);
}

#[test]
fn a_guide_declaring_a_path_without_anchors_is_refused() {
    let text = edited(
        "[guide]\nomitted = \"the fixture book is self-contained\"",
        "[guide]\npath = \"docs/USAGE.md\"",
    );

    assert!(reason(&text).contains("anchors"), "{}", reason(&text));
}

#[test]
fn a_chapter_id_that_is_not_its_file_stem_is_refused() {
    let text = edited(
        "file = \"01-orientation.md\"\nid = \"orientation\"",
        "file = \"01-orientation.md\"\nid = \"overview\"",
    );

    assert!(
        reason(&text).contains("file name stem"),
        "{}",
        reason(&text)
    );
}

#[test]
fn a_chapter_numbered_out_of_position_is_refused() {
    // Reordering two chapters leaves every id, slice and file name valid and
    // makes the file names disagree with the array order they now carry.
    let text = support::manifest_text();
    let first = text.find("[[page]]\nfile = \"01-orientation.md\"").unwrap();
    let second = text.find("[[page]]\nfile = \"02-name-seam.md\"").unwrap();
    let third = text
        .find("[[page]]\nfile = \"03-reference-domain.md\"")
        .unwrap();
    let swapped = format!(
        "{}{}{}{}",
        &text[..first],
        &text[second..third],
        &text[first..second],
        &text[third..]
    );

    assert!(
        reason(&swapped).contains("must be numbered"),
        "{}",
        reason(&swapped)
    );
}

#[test]
fn a_contents_page_with_another_identifier_is_refused() {
    let text = edited("id = \"contents\"", "id = \"toc\"");

    assert!(reason(&text).contains("README.md"), "{}", reason(&text));
}

#[test]
fn a_page_file_outside_the_book_directory_is_refused() {
    for file in [
        "../secrets.md",
        "sub/dir/page.md",
        ".hidden.md",
        "notes.txt",
    ] {
        let text = edited("file = \"concept-index.md\"", &format!("file = \"{file}\""));

        assert!(
            reason(&text).contains("plain `.md` file name"),
            "`{file}`: {}",
            reason(&text)
        );
    }
}

#[test]
fn a_root_or_block_identifier_outside_the_fragment_grammar_is_refused() {
    let root = edited("id = \"source-crate-manifest\"", "id = \"Source Manifest\"");
    let block = edited(
        "id = \"manifest-package-and-library-dependency\"",
        "id = \"Manifest Block\"",
    );

    assert!(reason(&root).contains("fragment id"), "{}", reason(&root));
    assert!(reason(&block).contains("fragment id"), "{}", reason(&block));
}

#[test]
fn a_manifest_with_crlf_or_no_final_newline_is_refused() {
    let crlf = support::manifest_text().replace('\n', "\r\n");
    let unterminated = support::manifest_text().trim_end().to_owned();

    assert!(
        reason(&crlf).contains("LF line endings"),
        "{}",
        reason(&crlf)
    );
    assert!(
        reason(&unterminated).contains("end in one LF"),
        "{}",
        reason(&unterminated)
    );
}

#[test]
fn a_malformed_manifest_reports_the_parse_reason() {
    let reason = reason("schema = ");

    assert!(!reason.is_empty());
}

// ---------------------------------------------------------------------------
// The corpus rule.
//
// These are the schema half of the control that replaced the compiled-in
// corpus. Filesystem derivation proves the declared patterns matched; it can
// prove neither that the author declared the right patterns nor that an
// exception is honest. The rules below take both of those out of the author's
// hands, which is what makes the corpus external rather than self-declared.
// ---------------------------------------------------------------------------

#[test]
fn an_include_missing_a_base_pattern_names_the_missing_one() {
    for base in [
        "crates/ordinal-fs-tree/Cargo.toml",
        "crates/ordinal-fs-tree/src/**/*.rs",
    ] {
        let text = edited(
            &format!("\"{base}\""),
            "\"crates/ordinal-fs-tree/src/lib.rs\"",
        );

        let reason = reason(&text);
        assert!(
            reason.contains(base) && reason.contains("base pattern"),
            "{reason}"
        );
    }
}

/// The failure the base-pattern rule exists to prevent, stated as a test:
/// a manifest that includes one file, declares that one root, and would
/// otherwise satisfy every check while proving nothing.
#[test]
fn an_include_narrowed_to_a_single_file_is_refused() {
    let text = edited(
        "include = [\"crates/ordinal-fs-tree/Cargo.toml\", \"crates/ordinal-fs-tree/src/**/*.rs\"]",
        "include = [\"crates/ordinal-fs-tree/src/lib.rs\"]",
    );

    assert!(reason(&text).contains("base pattern"), "{}", reason(&text));
}

#[test]
fn an_include_pattern_in_neither_accepted_form_is_refused() {
    for pattern in [
        "crates/ordinal-fs-tree/src/*.rs",
        "crates/**/src/**/*.rs",
        "crates/ordinal-fs-tree/src/**/*",
    ] {
        let text = edited(
            "include = [\"crates/ordinal-fs-tree/Cargo.toml\"",
            &format!("include = [\"{pattern}\", \"crates/ordinal-fs-tree/Cargo.toml\""),
        );

        let reason = reason(&text);
        assert!(reason.contains(pattern), "{reason}");
    }
}

#[test]
fn a_corpus_exception_class_outside_the_closed_list_is_refused() {
    let text = edited("class = \"test-support\"", "class = \"generated\"");

    let reason = reason(&text);
    assert!(
        reason.contains("generated") && reason.contains("test-support"),
        "{reason}"
    );
}

/// An exclusion classed `production-outside-src` is refused for the same
/// reason: each side of the rule has its own closed list, and a class that
/// crosses over is describing something the specification has not seen.
#[test]
fn an_exclusion_carrying_an_addition_class_is_refused() {
    let text = edited(
        "class = \"test-support\"",
        "class = \"production-outside-src\"",
    );

    assert!(
        reason(&text).contains("production-outside-src"),
        "{}",
        reason(&text)
    );
}

/// Where a class has a mechanically checkable form, the class claim is checked
/// rather than believed. `reason` stays free prose because it carries the
/// argument; `class` carries the constraint.
#[test]
fn an_inline_test_module_exclusion_whose_file_is_not_tests_rs_is_refused() {
    let text = edited(
        "path = \"crates/ordinal-fs-tree/src/ops/tests.rs\"",
        "path = \"crates/ordinal-fs-tree/src/ops/helpers.rs\"",
    );

    let reason = reason(&text);
    assert!(
        reason.contains("tests.rs") && reason.contains("helpers.rs"),
        "{reason}"
    );
}

#[test]
fn a_block_range_outside_the_n_dash_m_grammar_is_refused() {
    for range in ["0-3", "5-4", "3", "1-", "one-two"] {
        let text = edited("lines = \"1-42\"", &format!("lines = \"{range}\""));

        let reason = reason(&text);
        assert!(
            reason.contains(range) && reason.contains("1 <= N <= M"),
            "{reason}"
        );
    }
}

/// The blocks of one root partition it exactly: ordered, adjacent,
/// non-overlapping, and covering `1` to the declared line count. A manifest
/// that cannot describe a whole file is refused at load rather than reported as
/// a fragment finding somewhere in the book.
#[test]
fn blocks_that_leave_a_gap_in_their_root_are_refused() {
    let text = edited("lines = \"43-45\"", "lines = \"44-45\"");

    let reason = reason(&text);
    assert!(reason.contains("manifest-cli-feature"), "{reason}");
}

#[test]
fn blocks_that_do_not_reach_their_roots_last_line_are_refused() {
    let text = edited("lines = \"66-112\"", "lines = \"66-111\"");

    let reason = reason(&text);
    assert!(
        reason.contains("source-crate-manifest") && reason.contains("112"),
        "{reason}"
    );
}

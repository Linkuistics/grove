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

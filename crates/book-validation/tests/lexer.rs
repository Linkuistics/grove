use std::collections::BTreeMap;

use book_validation::{validate, BookSnapshot, Check, Request, Scope};

fn codes(bytes: &[u8]) -> Vec<String> {
    validate(
        &BookSnapshot {
            book_files: BTreeMap::from([(
                "docs/ordinal-fs-tree/book/source-index.md".into(),
                bytes.to_vec(),
            )]),
            source_files: BTreeMap::new(),
        },
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    )
    .diagnostics
    .into_iter()
    .map(|diagnostic| diagnostic.code)
    .collect()
}

#[test]
fn directive_looking_lines_inside_ordinary_fences_are_opaque() {
    let markdown = b"```text\n<!-- insert \xc2\xabnot-real\xc2\xbb -->\n```\n";
    let codes = codes(markdown);

    assert!(!codes
        .iter()
        .any(|code| matches!(code.as_str(), "P001" | "P002" | "F002")));
}

#[test]
fn malformed_reserved_lines_are_parse_findings() {
    assert!(codes(b"<!-- insert \xc2\xabbad id\xc2\xbb -->\n").contains(&"P001".into()));
}

#[test]
fn exact_directives_outside_graph_context_are_context_findings() {
    assert!(codes(b"<!-- insert \xc2\xabvalid-id\xc2\xbb -->\n").contains(&"P002".into()));
}

#[test]
fn cr_and_missing_final_lf_are_byte_findings() {
    let codes = codes(b"prose\r\nlast");
    assert_eq!(
        codes.iter().filter(|code| code.as_str() == "P003").count(),
        2
    );
}

#[test]
fn four_backticks_are_rejected_outside_literal_fragments() {
    assert!(codes(b"````rust\nnot owned source\n````\n").contains(&"P002".into()));
}

#[test]
fn literal_fence_language_must_match_the_source_path() {
    let markdown = concat!(
        "<!-- fragment «wrong-language» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````toml\nline\n````\n<!-- /fragment -->\n",
    );
    assert!(codes(markdown.as_bytes()).contains(&"P002".into()));
}

#[test]
fn valid_book_page_directives_are_available_to_later_markdown_checks() {
    let codes = codes(b"# Source index\n<!-- book-page id=\"source-index\" role=\"lookup\" -->\n");
    assert!(!codes.contains(&"P001".into()));
}

#[test]
fn numeric_hyphen_components_are_valid_fragment_ids() {
    let markdown = concat!(
        "<!-- fragment «part-1» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nline\n````\n<!-- /fragment -->\n",
    );
    assert!(!codes(markdown.as_bytes()).contains(&"P001".into()));
}

#[test]
fn blank_lines_inside_roots_are_context_findings() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "\n",
        "<!-- defer «library-crate-surface» owner=\"name-seam-k12\" lines=\"1-94\" -->\n",
        "<!-- /source-root -->\n",
    );
    assert!(codes(markdown.as_bytes()).contains(&"P002".into()));
}

#[test]
fn invalid_utf8_reports_its_first_byte_and_recovers_after_the_line() {
    let bytes =
        b"<!-- insert \xc2\xabfirst\xc2\xbb -->\n\xff\n<!-- insert \xc2\xabsecond\xc2\xbb -->\n";
    let report = validate(
        &BookSnapshot {
            book_files: BTreeMap::from([(
                "docs/ordinal-fs-tree/book/source-index.md".into(),
                bytes.to_vec(),
            )]),
            source_files: BTreeMap::new(),
        },
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    );
    let encoding = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "P003")
        .unwrap();

    assert_eq!(encoding.primary.byte, 26);
    assert_eq!(
        report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "P002")
            .count(),
        2
    );
}

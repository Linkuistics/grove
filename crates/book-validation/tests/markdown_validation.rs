use std::collections::BTreeMap;

use book_validation::{
    validate, BookSnapshot, Check, Request, Scope, ScopedSlice, ValidationReport,
};

const ROOT: &str = "docs/ordinal-fs-tree/book/";

fn valid_book() -> BookSnapshot {
    let book_files = BTreeMap::from([
        (
            format!("{ROOT}README.md"),
            concat!(
                "# Ordinal filesystem tree\n",
                "<!-- book-page id=\"contents\" role=\"contents\" -->\n",
                "\n",
                "[Orientation](01-orientation.md)\n",
                "[Concept index](concept-index.md)\n",
                "[Source index](source-index.md)\n",
            )
            .as_bytes()
            .to_vec(),
        ),
        (
            format!("{ROOT}01-orientation.md"),
            concat!(
                "# Orientation\n",
                "<!-- book-page id=\"orientation\" slice=\"orientation-k11\" order=\"1\" -->\n",
                "[Contents](README.md)\n",
                "\n",
                "<a id=\"tour\"></a>\n",
                "## Tour\n",
                "\n",
                "[Source roots](source-index.md#source-roots)\n",
                "\n",
                "[Contents](README.md)\n",
            )
            .as_bytes()
            .to_vec(),
        ),
        (
            format!("{ROOT}concept-index.md"),
            concat!(
                "# Concept index\n",
                "<!-- book-page id=\"concept-index\" role=\"lookup\" -->\n",
                "\n",
                "[Contents](README.md)\n",
                "[Tour](01-orientation.md#tour)\n",
            )
            .as_bytes()
            .to_vec(),
        ),
        (
            format!("{ROOT}source-index.md"),
            concat!(
                "# Source index\n",
                "<!-- book-page id=\"source-index\" role=\"lookup\" -->\n",
                "\n",
                "[Contents](README.md)\n",
                "<a id=\"source-roots\"></a>\n",
                "## Source roots\n",
                "[Tour](01-orientation.md#tour)\n",
            )
            .as_bytes()
            .to_vec(),
        ),
    ]);
    let book_entries = book_files.keys().cloned().collect();
    BookSnapshot {
        book_files,
        source_files: BTreeMap::new(),
        book_entries,
        non_regular_book_entries: Default::default(),
    }
}

fn validate_markdown(snapshot: &BookSnapshot) -> ValidationReport {
    validate_through(snapshot, "orientation-k11")
}

#[test]
fn an_expected_page_path_with_a_non_regular_entry_is_an_inventory_finding() {
    let mut snapshot = valid_book();
    snapshot
        .non_regular_book_entries
        .insert(format!("{ROOT}01-orientation.md"));

    let report = validate_markdown(&snapshot);

    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "M101"
            && diagnostic.primary.path == format!("{ROOT}01-orientation.md")));
}

fn validate_through(snapshot: &BookSnapshot, slice: &str) -> ValidationReport {
    validate(
        snapshot,
        Request {
            scope: Scope::Through(ScopedSlice::parse(slice).unwrap()),
            check: Check::Markdown,
        },
    )
}

fn valid_two_page_book() -> BookSnapshot {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "README.md", |text| {
        text.replace(
            "[Orientation](01-orientation.md)\n",
            "[Orientation](01-orientation.md)\n[Name seam](02-name-seam.md)\n",
        )
    });
    edit(&mut snapshot, "01-orientation.md", |text| {
        text.replace(
            "[Contents](README.md)",
            "[Contents](README.md) | [Next: Name seam](02-name-seam.md)",
        )
    });
    snapshot.book_files.insert(
        format!("{ROOT}02-name-seam.md"),
        concat!(
            "# Name seam\n",
            "<!-- book-page id=\"name-seam\" slice=\"name-seam-k12\" order=\"2\" -->\n",
            "[Previous: Orientation](01-orientation.md) | [Contents](README.md)\n",
            "\n",
            "<a id=\"names\"></a>\n",
            "## Names\n",
            "\n",
            "[Previous: Orientation](01-orientation.md) | [Contents](README.md)\n",
        )
        .as_bytes()
        .to_vec(),
    );
    snapshot
}

fn codes(report: &ValidationReport) -> Vec<&str> {
    report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect()
}

fn edit(snapshot: &mut BookSnapshot, file: &str, change: impl FnOnce(String) -> String) {
    let path = format!("{ROOT}{file}");
    let contents = String::from_utf8(snapshot.book_files.remove(&path).unwrap()).unwrap();
    snapshot
        .book_files
        .insert(path, change(contents).into_bytes());
}

#[test]
fn canonical_orientation_prefix_is_valid_markdown() {
    let report = validate_markdown(&valid_book());

    assert!(report.valid, "{:#?}", report.diagnostics);
}

#[test]
fn missing_and_duplicate_page_identities_are_inventory_findings() {
    let mut missing = valid_book();
    edit(&mut missing, "concept-index.md", |text| {
        text.replace(
            "<!-- book-page id=\"concept-index\" role=\"lookup\" -->\n",
            "",
        )
    });
    let mut duplicate = valid_book();
    edit(&mut duplicate, "concept-index.md", |text| {
        text.replace(
            "<!-- book-page id=\"concept-index\" role=\"lookup\" -->\n",
            concat!(
                "<!-- book-page id=\"concept-index\" role=\"lookup\" -->\n",
                "<!-- book-page id=\"concept-index\" role=\"lookup\" -->\n",
            ),
        )
    });
    let mut missing_numbered = valid_book();
    edit(&mut missing_numbered, "01-orientation.md", |text| {
        text.replace(
            "<!-- book-page id=\"orientation\" slice=\"orientation-k11\" order=\"1\" -->\n",
            "",
        )
    });

    assert!(codes(&validate_markdown(&missing)).contains(&"M101"));
    assert!(codes(&validate_markdown(&duplicate)).contains(&"M101"));
    assert!(codes(&validate_markdown(&missing_numbered)).contains(&"M101"));
}

#[test]
fn malformed_heading_and_anchor_structure_is_rejected() {
    let mut skipped = valid_book();
    edit(&mut skipped, "01-orientation.md", |text| {
        text.replace("## Tour\n", "### Tour\n")
    });
    let mut duplicate_anchor = valid_book();
    edit(&mut duplicate_anchor, "01-orientation.md", |text| {
        text.replace(
            "<a id=\"tour\"></a>\n",
            "<a id=\"tour\"></a>\n<a id=\"tour\"></a>\n",
        )
    });

    assert!(codes(&validate_markdown(&skipped)).contains(&"M102"));
    assert!(codes(&validate_markdown(&duplicate_anchor)).contains(&"M102"));
}

#[test]
fn numbered_page_navigation_must_match_the_prefix() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "01-orientation.md", |text| {
        text.replacen("[Contents](README.md)", "[Next](02-name-seam.md)", 1)
    });

    assert!(codes(&validate_markdown(&snapshot)).contains(&"M103"));
}

#[test]
fn ordinary_rust_and_toml_fences_cannot_hide_production_source() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "01-orientation.md", |text| {
        text.replace(
            "\n[Contents](README.md)\n",
            "\n```rust\nfn hidden() {}\n```\n\n[Contents](README.md)\n",
        )
    });

    assert!(codes(&validate_markdown(&snapshot)).contains(&"M104"));
}

#[test]
fn literal_fragment_whose_nearest_nonblank_predecessor_is_not_prose_is_rejected() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "01-orientation.md", |text| {
        text.replace(
            "## Tour\n\n",
            concat!(
                "## Tour\n",
                "<!-- fragment «opening» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
                "````rust\n",
                "line\n",
                "````\n",
                "<!-- /fragment -->\n\n",
            ),
        )
    });

    assert!(codes(&validate_markdown(&snapshot)).contains(&"M105"));
}

#[test]
fn broken_files_missing_explicit_anchors_and_scope_escapes_are_link_findings() {
    let cases = [
        "[Missing file](missing.md)",
        "[Missing anchor](source-index.md#absent)",
        "[Escaped repository](../../../../outside.md)",
    ];

    for broken in cases {
        let mut snapshot = valid_book();
        edit(&mut snapshot, "concept-index.md", |text| {
            format!("{text}{broken}\n")
        });

        let report = validate_markdown(&snapshot);
        assert!(
            codes(&report).contains(&"M201"),
            "{broken}: {:#?}",
            report.diagnostics
        );
    }
}

#[test]
fn repository_file_links_outside_the_book_and_frozen_corpus_are_rejected() {
    let mut snapshot = valid_book();
    snapshot.source_files.insert(
        "docs/ordinal-fs-tree/ARCHITECTURE.md".into(),
        b"# Architecture\n<a id=\"boundary\"></a>\n## Boundary\n".to_vec(),
    );
    edit(&mut snapshot, "01-orientation.md", |text| {
        text.replace(
            "## Tour\n\n",
            "## Tour\n\n[Architecture](../ARCHITECTURE.md#boundary)\n\n",
        )
    });

    let report = validate_markdown(&snapshot);
    assert!(
        codes(&report).contains(&"M201"),
        "{:#?}",
        report.diagnostics
    );
}

#[test]
fn external_urls_are_syntax_checked_without_becoming_local_files() {
    let mut valid = valid_book();
    edit(&mut valid, "concept-index.md", |text| {
        format!(
            "{text}[Docs](https://example.com/path?q=1#part)\n[IPv6](http://[::1]:8080/x)\n[Mail](mailto:reader@example.com)\n"
        )
    });

    assert!(validate_markdown(&valid).valid);
    for destination in [
        "https://",
        "https://example.com:bogus",
        "http://[::1",
        "mailto:a@b.c@d",
    ] {
        let mut invalid = valid_book();
        edit(&mut invalid, "concept-index.md", |text| {
            format!("{text}[Broken]({destination})\n")
        });
        assert!(
            codes(&validate_markdown(&invalid)).contains(&"M201"),
            "{destination}"
        );
    }
}

#[test]
fn links_in_inline_code_and_shared_lexer_fences_are_ignored() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}`[Inline](missing.md)`\n\n```text\n[Fenced](missing.md)\n```\n")
    });

    let report = validate_markdown(&snapshot);
    assert!(report.valid, "{:#?}", report.diagnostics);
}

#[test]
fn ambiguous_link_subset_and_nondescriptive_labels_fail_closed() {
    for broken in [
        "[Outer [nested]](README.md)",
        "[Title](README.md \"contents\")",
        "[Escaped](README\\.md)",
        "[here](README.md)",
    ] {
        let mut snapshot = valid_book();
        edit(&mut snapshot, "concept-index.md", |text| {
            format!("{text}{broken}\n")
        });

        assert!(
            codes(&validate_markdown(&snapshot)).contains(&"M201"),
            "{broken}"
        );
    }
}

#[test]
fn markdown_diagnostics_are_deterministic_and_name_the_source_page_and_link() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}[Missing](missing.md)\n")
    });

    let first = validate_markdown(&snapshot);
    let second = validate_markdown(&snapshot);
    let diagnostic = first
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "M201")
        .unwrap();

    assert_eq!(first, second);
    assert_eq!(diagnostic.primary.path, format!("{ROOT}concept-index.md"));
    assert!(diagnostic.message.contains("missing.md"));
}

#[test]
fn all_selection_runs_fragment_and_markdown_checks() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}[Missing](missing.md)\n")
    });

    let report = validate(
        &snapshot,
        Request {
            scope: Scope::Through(ScopedSlice::Orientation),
            check: Check::All,
        },
    );
    let codes = codes(&report);

    assert!(codes.contains(&"F006"));
    assert!(codes.contains(&"M201"));
}

#[test]
fn anchors_inside_opaque_fences_do_not_satisfy_links() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "01-orientation.md", |text| {
        text.replace("source-index.md#source-roots", "source-index.md#hidden")
    });
    edit(&mut snapshot, "source-index.md", |text| {
        format!("{text}\n```text\n<a id=\"hidden\"></a>\n## Hidden\n```\n")
    });

    assert!(codes(&validate_markdown(&snapshot)).contains(&"M201"));
}

#[test]
fn readme_chapter_links_are_unique_and_in_canonical_order() {
    let mut reversed = valid_two_page_book();
    edit(&mut reversed, "README.md", |text| {
        text.replace(
            "[Orientation](01-orientation.md)\n[Name seam](02-name-seam.md)\n",
            "[Name seam](02-name-seam.md)\n[Orientation](01-orientation.md)\n",
        )
    });
    let mut duplicate = valid_two_page_book();
    edit(&mut duplicate, "README.md", |text| {
        text.replace(
            "[Orientation](01-orientation.md)\n",
            "[Orientation](01-orientation.md)\n[Orientation again](01-orientation.md)\n",
        )
    });

    assert!(codes(&validate_through(&reversed, "name-seam-k12")).contains(&"M103"));
    assert!(codes(&validate_through(&duplicate, "name-seam-k12")).contains(&"M103"));
}

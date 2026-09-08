use std::collections::BTreeMap;

mod support;

use book_validation::{validate, BookSnapshot, Check, Request, ValidationReport};

const ROOT: &str = "docs/walkthroughs/ordinal-fs-tree/";

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
    let manifest = support::manifest();
    let mut book_entries: std::collections::BTreeSet<String> = book_files.keys().cloned().collect();
    book_entries.insert(manifest.manifest_path());
    BookSnapshot {
        derived_corpus: support::derived_corpus(),
        manifest,
        book_files,
        source_files: BTreeMap::new(),
        outbound_files: Default::default(),
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
            scope: support::through(slice),
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
        "docs/walkthroughs/ARCHITECTURE.md".into(),
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
            scope: support::through("orientation-k11"),
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

#[test]
fn a_hard_wrapped_link_label_is_checked_rather_than_skipped() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}[A label wrapped\nacross two lines](missing.md)\n")
    });

    let report = validate_markdown(&snapshot);
    assert!(
        codes(&report).contains(&"M201"),
        "{:#?}",
        report.diagnostics
    );
}

#[test]
fn a_hard_wrapped_link_to_a_live_target_is_scanned_and_accepted() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}[A label wrapped\nacross two lines](01-orientation.md#tour)\n")
    });

    // Asserted positively, against the scanner. A validation report that merely
    // holds no finding is what the *skipped* link produced too, so `valid` alone
    // cannot tell coverage from silence.
    let scanned = book_validation::scan_markdown_links(
        "[A label wrapped\nacross two lines](01-orientation.md#tour)\n",
    );
    assert_eq!(scanned.len(), 1, "{scanned:#?}");
    assert_eq!(scanned[0].destination, "01-orientation.md#tour");
    assert_eq!(scanned[0].label, "A label wrapped\nacross two lines");
    assert!(scanned[0].valid_syntax);

    let report = validate_markdown(&snapshot);
    assert!(report.valid, "{:#?}", report.diagnostics);
}

#[test]
fn a_bracket_and_a_separator_in_different_paragraphs_do_not_pair() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}An unclosed [ bracket.\n\nA later paragraph](missing.md) closing it.\n")
    });

    let report = validate_markdown(&snapshot);
    assert!(report.valid, "{:#?}", report.diagnostics);
}

#[test]
fn a_bare_anchor_link_resolves_against_the_page_that_carries_it() {
    let mut live = valid_book();
    edit(&mut live, "source-index.md", |text| {
        format!("{text}[Source roots](#source-roots)\n")
    });
    let mut broken = valid_book();
    edit(&mut broken, "source-index.md", |text| {
        format!("{text}[Absent section](#absent)\n")
    });

    let report = validate_markdown(&live);
    assert!(report.valid, "{:#?}", report.diagnostics);

    // The message matters, not just the code: resolving to the containing
    // *directory* also produced an `M201`, so a bare code assertion holds
    // whether or not the anchor is checked against the right file.
    let broken = validate_markdown(&broken);
    let message = broken
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "M201")
        .map(|diagnostic| diagnostic.message.clone())
        .unwrap_or_default();
    assert!(
        message.contains(&format!("{ROOT}source-index.md")),
        "the finding must name the page that carries the link: {message}"
    );
}

#[test]
fn a_stray_bracket_does_not_swallow_a_later_link_on_another_line() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}An unmatched [ bracket in prose, and several lines\nlater a real [Missing file](missing.md) link.\n")
    });

    let report = validate_markdown(&snapshot);
    let named: Vec<&str> = report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect();
    assert!(
        named.iter().any(|message| message.contains("missing.md")),
        "the genuine link must still be reported: {named:#?}"
    );
    assert!(
        report
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.primary.line == 7),
        "reported against the link's own line, not the stray bracket's: {:#?}",
        report.diagnostics
    );
}

#[test]
fn a_separator_inside_a_fence_does_not_pair_with_a_bracket_above_it() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}An unmatched [ bracket in prose.\n```text\nfoo](missing.md)\n```\n")
    });

    let report = validate_markdown(&snapshot);
    assert!(report.valid, "{:#?}", report.diagnostics);
}

#[test]
fn a_line_of_unicode_whitespace_is_not_a_paragraph_break() {
    let mut snapshot = valid_book();
    edit(&mut snapshot, "concept-index.md", |text| {
        format!("{text}[A label wrapped\n\u{a0}\nacross a non-blank line](missing.md)\n")
    });

    let report = validate_markdown(&snapshot);
    assert!(
        codes(&report).contains(&"M201"),
        "a non-breaking space is not a blank line, so this is still one link: {:#?}",
        report.diagnostics
    );
}

#[test]
fn an_empty_anchor_is_rejected_with_a_message_naming_the_remedy() {
    for broken in ["#", "README.md#"] {
        let mut snapshot = valid_book();
        edit(&mut snapshot, "concept-index.md", |text| {
            format!("{text}[Empty anchor]({broken})\n")
        });

        let report = validate_markdown(&snapshot);
        let message = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "M201")
            .map(|diagnostic| diagnostic.message.clone())
            .unwrap_or_default();
        assert!(
            message.contains("empty anchor") && message.contains("trailing `#`"),
            "{broken}: {message}"
        );
    }
}

use std::collections::BTreeMap;

use book_validation::{validate, BookSnapshot, Check, Request, Scope};

fn snapshot(markdown: &str, source: &str) -> BookSnapshot {
    BookSnapshot {
        book_files: BTreeMap::from([(
            "docs/ordinal-fs-tree/book/source-index.md".into(),
            markdown.as_bytes().to_vec(),
        )]),
        source_files: BTreeMap::from([(
            "crates/ordinal-fs-tree/src/lib.rs".into(),
            source.as_bytes().to_vec(),
        )]),
    }
}

fn scoped(markdown: &str, source: &str) -> Vec<String> {
    validate(
        &snapshot(markdown, source),
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
fn duplicate_fragment_ids_are_ambiguous() {
    let markdown = concat!(
        "<!-- fragment «same» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nfirst\n````\n<!-- /fragment -->\n",
        "<!-- fragment «same» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nsecond\n````\n<!-- /fragment -->\n",
    );

    assert!(scoped(markdown, "first\n").contains(&"F001".into()));
}

#[test]
fn duplicate_top_level_fragments_do_not_contribute_resolved_coverage() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "<!-- insert «library-crate-surface» -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «library-crate-surface» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" parent=\"source-library\" -->\n",
        "````rust\nfirst\n````\n<!-- /fragment -->\n",
        "<!-- fragment «library-crate-surface» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" parent=\"source-library\" -->\n",
        "````rust\nsecond\n````\n<!-- /fragment -->\n",
    );
    let report = validate(
        &snapshot(markdown, "first\n"),
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    );

    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "F001"));
    assert_eq!(report.coverage.resolved_lines, 0);
}

#[test]
fn absent_insert_targets_are_unresolved() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- insert «missing» -->\n",
        "<!-- /source-root -->\n",
    );

    assert!(scoped(markdown, "line\n").contains(&"F002".into()));
}

#[test]
fn recursive_insertions_report_a_cycle() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- insert «a» -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «a» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "<!-- insert «b» -->\n",
        "<!-- /fragment -->\n",
        "<!-- fragment «b» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"a\" -->\n",
        "<!-- insert «a» -->\n",
        "<!-- /fragment -->\n",
    );

    let report = validate(
        &snapshot(markdown, "line\n"),
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    );
    let cycle = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "F004")
        .unwrap();
    assert!(cycle.message.ends_with("a -> b -> a"));
}

#[test]
fn recursive_composite_expansion_preserves_exact_bytes() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-2\" -->\n",
        "<!-- insert «library-crate-surface» -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «library-crate-surface» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-2\" parent=\"source-library\" -->\n",
        "<!-- insert «part-1» -->\n",
        "<!-- insert «part-2» -->\n",
        "<!-- /fragment -->\n",
        "<!-- fragment «part-1» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"library-crate-surface\" -->\n",
        "````rust\none\n````\n<!-- /fragment -->\n",
        "<!-- fragment «part-2» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"2-2\" parent=\"library-crate-surface\" -->\n",
        "````rust\ntwo\n````\n<!-- /fragment -->\n",
    );
    let codes = scoped(markdown, "one\ntwo\n");

    assert!(!codes
        .iter()
        .any(|code| { matches!(code.as_str(), "F002" | "F004" | "F005" | "F007" | "F008") }));
}

#[test]
fn definitions_not_reached_from_a_root_are_rejected() {
    let markdown = concat!(
        "<!-- fragment «orphan» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nline\n````\n<!-- /fragment -->\n",
    );

    assert!(scoped(markdown, "line\n").contains(&"F005".into()));
}

#[test]
fn a_named_later_defer_is_not_an_unresolved_insert() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- defer «library-crate-surface» owner=\"name-seam-k12\" lines=\"1-1\" -->\n",
        "<!-- /source-root -->\n",
    );

    let codes = scoped(markdown, "line\n");
    assert!(!codes.contains(&"F002".into()));
}

#[test]
fn a_defer_owned_by_the_current_slice_is_overdue() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- defer «library-crate-surface» owner=\"orientation-k11\" lines=\"1-1\" -->\n",
        "<!-- /source-root -->\n",
    );

    assert!(scoped(markdown, "line\n").contains(&"F003".into()));
}

#[test]
fn a_named_later_slice_that_arrives_without_filling_its_hole_is_overdue_not_unresolved() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "<!-- defer «library-crate-surface» owner=\"name-seam-k12\" lines=\"1-94\" -->\n",
        "<!-- /source-root -->\n",
    );
    let report = validate(
        &snapshot(markdown, "line\n"),
        Request {
            scope: Scope::Through("name-seam-k12".into()),
            check: Check::Fragments,
        },
    );
    let codes: Vec<_> = report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect();

    assert!(codes.contains(&"F003"));
    assert!(!codes.contains(&"F002"));
}

#[test]
fn literal_newline_and_whitespace_drift_is_byte_failure() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- insert «library-crate-surface» -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «library-crate-surface» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nline \n````\n<!-- /fragment -->\n",
    );

    let snapshot = snapshot(markdown, "line\n");
    let request = Request {
        scope: Scope::Through("orientation-k11".into()),
        check: Check::Fragments,
    };
    let first = validate(&snapshot, request.clone());
    let second = validate(&snapshot, request);
    let finding = first
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "F008" && diagnostic.fragment_id.is_some())
        .unwrap();

    assert_eq!(
        finding.fragment_id.as_deref(),
        Some("library-crate-surface")
    );
    assert_eq!(finding.root_id.as_deref(), Some("source-library"));
    assert_eq!(
        finding.primary.path,
        "docs/ordinal-fs-tree/book/source-index.md"
    );
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
}

#[test]
fn an_unknown_source_root_is_an_inventory_failure() {
    let markdown = concat!(
        "<!-- source-root «source-unknown» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- defer «later» owner=\"name-seam-k12\" lines=\"1-1\" -->\n",
        "<!-- /source-root -->\n",
    );

    assert!(scoped(markdown, "line\n").contains(&"F006".into()));
}

#[test]
fn inserting_one_source_range_twice_is_a_coverage_failure() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-2\" -->\n",
        "<!-- insert «first» -->\n",
        "<!-- insert «first» -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «first» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\none\n````\n<!-- /fragment -->\n",
    );

    assert!(scoped(markdown, "one\ntwo\n").contains(&"F007".into()));
}

#[test]
fn a_gap_between_child_ranges_is_a_missing_source_failure() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-2\" -->\n",
        "<!-- insert «second» -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «second» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"2-2\" parent=\"source-library\" -->\n",
        "````rust\ntwo\n````\n<!-- /fragment -->\n",
    );

    assert!(scoped(markdown, "one\ntwo\n").contains(&"F007".into()));
}

#[test]
fn a_definition_cannot_exist_while_its_parent_still_defers_it() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "<!-- defer «library-crate-surface» owner=\"name-seam-k12\" lines=\"1-94\" -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «library-crate-surface» owner=\"name-seam-k12\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" parent=\"source-library\" -->\n",
        "````rust\nline\n````\n<!-- /fragment -->\n",
    );

    assert!(scoped(markdown, "line\n").contains(&"F003".into()));
}

#[test]
fn a_fragment_reached_from_multiple_roots_is_a_reachability_failure() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "<!-- insert «library-crate-surface» -->\n",
        "<!-- /source-root -->\n",
        "<!-- source-root «other-root» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "<!-- insert «library-crate-surface» -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «library-crate-surface» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" parent=\"source-library\" -->\n",
        "````rust\nline\n````\n<!-- /fragment -->\n",
    );

    assert!(scoped(markdown, "line\n").contains(&"F005".into()));
}

#[test]
fn a_missing_authoritative_source_is_an_inventory_failure() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "<!-- defer «library-crate-surface» owner=\"name-seam-k12\" lines=\"1-94\" -->\n",
        "<!-- /source-root -->\n",
    );
    let mut snapshot = snapshot(markdown, "line\n");
    snapshot.source_files.clear();
    let report = validate(
        &snapshot,
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    );

    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "F006"));
}

#[test]
fn unknown_in_memory_scopes_fail_closed() {
    let report = validate(
        &snapshot("prose\n", "line\n"),
        Request {
            scope: Scope::Through("unknown-k999".into()),
            check: Check::Fragments,
        },
    );
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "U001"));
}

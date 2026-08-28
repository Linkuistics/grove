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

fn orientation(markdown: &str, source: &str) -> book_validation::ValidationReport {
    validate(
        &snapshot(markdown, source),
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    )
}

#[test]
fn duplicate_evidence_names_every_later_occurrence_in_order() {
    let markdown = concat!(
        "<!-- fragment «same» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nfirst\n````\n<!-- /fragment -->\n",
        "<!-- fragment «same» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nsecond\n````\n<!-- /fragment -->\n",
        "<!-- fragment «same» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nthird\n````\n<!-- /fragment -->\n",
    );
    let report = orientation(markdown, "first\n");
    let duplicate = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "F001")
        .unwrap();

    assert_eq!(duplicate.related.len(), 2);
    assert!(duplicate.related[0].byte < duplicate.related[1].byte);
    assert_eq!(duplicate.related[0].label, "duplicate occurrence");
    assert!(duplicate.remedy.as_deref().unwrap().contains("unique ID"));
}

#[test]
fn byte_mismatch_has_complete_source_and_path_evidence() {
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "<!-- insert «library-crate-surface» -->\n",
        "<!-- /source-root -->\n",
        "<!-- fragment «library-crate-surface» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" parent=\"source-library\" -->\n",
        "````rust\nwrong\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\nline\nline\nline\nline\nline\nline\nline\n",
        "line\nline\nline\n````\n<!-- /fragment -->\n",
    );
    let source = format!("right\n{}", "line\n".repeat(93));
    let report = orientation(markdown, &source);
    let mismatch = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "F008" && diagnostic.fragment_id.is_some())
        .unwrap();

    let source = mismatch.source.as_ref().unwrap();
    assert_eq!(source.path, "crates/ordinal-fs-tree/src/lib.rs");
    assert_eq!(source.byte, 0);
    assert_eq!(source.line, 1);
    assert!(mismatch.message.contains("expected 0x72, actual 0x77"));
    assert!(mismatch.message.contains("owner `orientation-k11`"));
    assert!(mismatch
        .message
        .contains("path source-library -> library-crate-surface"));
    assert!(mismatch
        .remedy
        .as_deref()
        .unwrap()
        .contains("literal bytes"));
}

#[test]
fn every_serialized_record_has_the_complete_nullable_schema() {
    let report = orientation("prose\n", "line\n");
    let diagnostic = serde_json::to_value(&report.diagnostics[0]).unwrap();
    let object = diagnostic.as_object().unwrap();
    let keys: Vec<_> = object.keys().map(String::as_str).collect();

    assert_eq!(
        keys,
        [
            "code",
            "fragment_id",
            "message",
            "phase",
            "primary",
            "related",
            "remedy",
            "root_id",
            "source",
        ]
    );
    assert!(object["source"].is_null());
    assert_eq!(object["related"], serde_json::json!([]));
}

#[test]
fn a_deep_graph_is_processed_without_recursion() {
    const DEPTH: usize = 2_000;
    let mut markdown = String::from(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n<!-- insert «n0» -->\n<!-- /source-root -->\n",
    );
    for index in 0..DEPTH {
        let parent = if index == 0 {
            "source-library".to_owned()
        } else {
            format!("n{}", index - 1)
        };
        markdown.push_str(&format!(
            "<!-- fragment «n{index}» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"{parent}\" -->\n"
        ));
        if index + 1 == DEPTH {
            markdown.push_str("````rust\nline\n````\n");
        } else {
            markdown.push_str(&format!("<!-- insert «n{}» -->\n", index + 1));
        }
        markdown.push_str("<!-- /fragment -->\n");
    }

    let report = validate(
        &BookSnapshot {
            book_files: BTreeMap::from([(
                "docs/ordinal-fs-tree/book/01-orientation.md".into(),
                markdown.into_bytes(),
            )]),
            source_files: BTreeMap::from([(
                "crates/ordinal-fs-tree/src/lib.rs".into(),
                b"line\n".to_vec(),
            )]),
        },
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    );
    assert!(!report.diagnostics.iter().any(|diagnostic| {
        matches!(diagnostic.code.as_str(), "F002" | "F004" | "F005" | "F008")
    }));
}

#[test]
fn cycle_and_reachability_findings_carry_graph_evidence() {
    let cycle_markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- insert «a» -->\n<!-- /source-root -->\n",
        "<!-- fragment «a» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "<!-- insert «b» -->\n<!-- /fragment -->\n",
        "<!-- fragment «b» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"a\" -->\n",
        "<!-- insert «a» -->\n<!-- /fragment -->\n",
    );
    let cycle_report = orientation(cycle_markdown, "line\n");
    let cycle = cycle_report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "F004")
        .unwrap();
    assert_eq!(cycle.message, "fragment cycle: a -> b -> a");
    assert_eq!(cycle.related.len(), 2);
    assert_eq!(cycle.related[0].label, "cycle edge a -> b");
    assert_eq!(cycle.related[0].line, 5);
    assert_eq!(cycle.related[1].label, "cycle edge b -> a");
    assert_eq!(cycle.related[1].line, 8);
    assert!(cycle.remedy.as_deref().unwrap().contains("insertion edge"));

    let shared_markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- insert «shared» -->\n<!-- /source-root -->\n",
        "<!-- source-root «other-root» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n",
        "<!-- insert «shared» -->\n<!-- /source-root -->\n",
        "<!-- fragment «shared» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"source-library\" -->\n",
        "````rust\nline\n````\n<!-- /fragment -->\n",
    );
    let shared_report = orientation(shared_markdown, "line\n");
    let reachability = shared_report
        .diagnostics
        .iter()
        .find(|diagnostic| {
            diagnostic.code == "F005" && diagnostic.fragment_id.as_deref() == Some("shared")
        })
        .unwrap();
    assert!(reachability.message.contains("expected `source-library`"));
    assert!(reachability
        .message
        .contains("observed `other-root, source-library`"));
    assert_eq!(reachability.related.len(), 2);
}

#[test]
fn an_invalid_root_suppresses_only_its_byte_cascade() {
    let library_source = "line\n".repeat(94);
    let report_source = "report\n".repeat(152);
    let markdown = concat!(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
        "<!-- insert «bad-gap» -->\n<!-- /source-root -->\n",
        "<!-- fragment «bad-gap» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"2-94\" parent=\"source-library\" -->\n",
        "````rust\nwrong\n````\n<!-- /fragment -->\n",
        "<!-- source-root «source-report» source=\"crates/ordinal-fs-tree/src/report.rs\" lines=\"1-152\" -->\n",
        "<!-- insert «mutation-report-source» -->\n<!-- /source-root -->\n",
        "<!-- fragment «mutation-report-source» owner=\"mutation-algebra-k15\" source=\"crates/ordinal-fs-tree/src/report.rs\" lines=\"1-152\" parent=\"source-report\" -->\n",
        "````rust\nwrong\n````\n<!-- /fragment -->\n",
    );
    let report = validate(
        &BookSnapshot {
            book_files: BTreeMap::from([(
                "docs/ordinal-fs-tree/book/source-index.md".into(),
                markdown.as_bytes().to_vec(),
            )]),
            source_files: BTreeMap::from([
                (
                    "crates/ordinal-fs-tree/src/lib.rs".into(),
                    library_source.into_bytes(),
                ),
                (
                    "crates/ordinal-fs-tree/src/report.rs".into(),
                    report_source.into_bytes(),
                ),
            ]),
        },
        Request {
            scope: Scope::Final,
            check: Check::Fragments,
        },
    );
    let byte_roots: Vec<_> = report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "F008")
        .filter_map(|diagnostic| diagnostic.root_id.as_deref())
        .collect();

    assert_eq!(byte_roots, ["source-report"]);
}

#[test]
fn an_inventory_invalid_root_suppresses_its_byte_cascade() {
    let source = "line\n".repeat(93);
    let actual = format!("wrong\n{}", "line\n".repeat(93));
    let markdown = format!(
        concat!(
            "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" -->\n",
            "<!-- insert «library-crate-surface» -->\n<!-- /source-root -->\n",
            "<!-- fragment «library-crate-surface» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-94\" parent=\"source-library\" -->\n",
            "````rust\n{}````\n<!-- /fragment -->\n",
        ),
        actual
    );
    let report = orientation(&markdown, &source);

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "F006" && diagnostic.root_id.as_deref() == Some("source-library")
    }));
    assert!(!report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "F008" && diagnostic.root_id.as_deref() == Some("source-library")
    }));
}

#[test]
fn branching_invalid_graphs_do_not_expand_exponentially() {
    const DEPTH: usize = 1_000;
    let mut markdown = String::from(
        "<!-- source-root «source-library» source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" -->\n<!-- insert «branch0» -->\n<!-- /source-root -->\n",
    );
    for index in 0..DEPTH {
        let parent = if index == 0 {
            "source-library".to_owned()
        } else {
            format!("branch{}", index - 1)
        };
        markdown.push_str(&format!(
            "<!-- fragment «branch{index}» owner=\"orientation-k11\" source=\"crates/ordinal-fs-tree/src/lib.rs\" lines=\"1-1\" parent=\"{parent}\" -->\n"
        ));
        if index + 1 == DEPTH {
            markdown.push_str("````rust\nline\n````\n");
        } else {
            markdown.push_str(&format!(
                "<!-- insert «branch{}» -->\n<!-- insert «branch{}» -->\n",
                index + 1,
                index + 1
            ));
        }
        markdown.push_str("<!-- /fragment -->\n");
    }
    let report = validate(
        &BookSnapshot {
            book_files: BTreeMap::from([(
                "docs/ordinal-fs-tree/book/01-orientation.md".into(),
                markdown.into_bytes(),
            )]),
            source_files: BTreeMap::from([(
                "crates/ordinal-fs-tree/src/lib.rs".into(),
                b"line\n".to_vec(),
            )]),
        },
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    );

    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "F007"));
    assert!(!report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "F008"));
}

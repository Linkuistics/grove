mod support;

use book_validation::{validate, Check, Request, Scope};

#[test]
fn the_frozen_fifteen_file_corpus_expands_byte_for_byte() {
    let report = validate(
        &support::corpus(true),
        Request {
            scope: Scope::Final,
            check: Check::Fragments,
        },
    );

    assert_eq!(report.coverage.files, 15);
    assert_eq!(report.coverage.resolved_lines, 6_929);
    assert_eq!(report.coverage.deferred_lines, 0);
    assert!(report.valid, "{:#?}", report.diagnostics);
}

#[test]
fn source_growth_beyond_the_frozen_range_is_an_inventory_failure() {
    let mut snapshot = support::corpus(true);
    snapshot
        .source_files
        .get_mut("crates/ordinal-fs-tree/src/lib.rs")
        .unwrap()
        .extend_from_slice(b"added line\n");

    let report = validate(
        &snapshot,
        Request {
            scope: Scope::Final,
            check: Check::Fragments,
        },
    );

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "F006"
            && diagnostic.root_id.as_deref() == Some("source-library")
            && diagnostic.message.contains("95 lines")
    }));
}

#[test]
fn orientation_scope_reports_resolved_and_deferred_bytes_separately() {
    let report = validate(
        &support::corpus(false),
        Request {
            scope: Scope::Through("orientation-k11".into()),
            check: Check::Fragments,
        },
    );

    assert_eq!(
        report.coverage,
        book_validation::Coverage {
            files: 15,
            resolved_lines: 203,
            deferred_lines: 6_726,
            final_: false,
        }
    );
    assert!(report.valid, "{:#?}", report.diagnostics);
}

#[test]
fn a_well_formed_defer_absent_from_the_ownership_ledger_is_rejected() {
    let mut snapshot = support::corpus(false);
    let source_index = snapshot
        .book_files
        .get_mut("docs/ordinal-fs-tree/book/source-index.md")
        .unwrap();
    let text = String::from_utf8(source_index.clone()).unwrap().replace(
        "<!-- defer «name-seam-source» owner=\"name-seam-k12\" lines=\"1-700\" -->",
        "<!-- defer «never-filled» owner=\"name-seam-k12\" lines=\"1-700\" -->",
    );
    *source_index = text.into_bytes();

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
        .any(|diagnostic| diagnostic.code == "F003"));
}

#[test]
fn a_later_owned_block_cannot_be_defined_early() {
    let mut snapshot = support::corpus(false);
    let source_index = snapshot
        .book_files
        .get_mut("docs/ordinal-fs-tree/book/source-index.md")
        .unwrap();
    let text = String::from_utf8(source_index.clone()).unwrap().replace(
        "<!-- defer «name-seam-source» owner=\"name-seam-k12\" lines=\"1-700\" -->",
        "<!-- insert «name-seam-source» -->",
    );
    *source_index = text.into_bytes();
    let final_snapshot = support::corpus(true);
    snapshot.book_files.insert(
        "docs/ordinal-fs-tree/book/02-name-seam.md".into(),
        final_snapshot.book_files["docs/ordinal-fs-tree/book/02-name-seam.md"].clone(),
    );

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
        .any(|diagnostic| diagnostic.code == "F003"));
}

#[test]
fn exhaustive_mode_rejects_an_extra_source_file() {
    let mut snapshot = support::corpus(true);
    snapshot
        .source_files
        .insert("crates/extra.rs".into(), b"extra\n".to_vec());

    let report = validate(
        &snapshot,
        Request {
            scope: Scope::Final,
            check: Check::Fragments,
        },
    );
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "F006"));
}

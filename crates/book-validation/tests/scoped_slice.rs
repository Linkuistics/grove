mod support;

use std::collections::{BTreeMap, BTreeSet};

use book_validation::{validate, BookSnapshot, Check, Request};

#[test]
fn core_scope_accepts_only_the_typed_scoped_domain() {
    let report = validate(
        &BookSnapshot {
            manifest: support::manifest(),
            book_files: BTreeMap::new(),
            source_files: BTreeMap::new(),
            book_entries: BTreeSet::new(),
            non_regular_book_entries: BTreeSet::new(),
        },
        Request {
            scope: support::through("read-path-k14"),
            check: Check::Fragments,
        },
    );

    assert_eq!(
        serde_json::to_value(report.scope).unwrap(),
        serde_json::json!({ "kind": "through", "slice": "read-path-k14" })
    );
    assert!(report
        .diagnostics
        .iter()
        .all(|diagnostic| diagnostic.code != "U001"));
}

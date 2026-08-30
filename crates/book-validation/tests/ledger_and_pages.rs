mod support;

use book_validation::{validate, BookSnapshot, Check, Request, Scope, ScopedSlice};

const SOURCE_INDEX: &str = "docs/ordinal-fs-tree/book/source-index.md";

fn validate_final(snapshot: &BookSnapshot) -> book_validation::ValidationReport {
    validate(
        snapshot,
        Request {
            scope: Scope::Final,
            check: Check::Fragments,
        },
    )
}

fn validate_orientation(snapshot: &BookSnapshot) -> book_validation::ValidationReport {
    validate(
        snapshot,
        Request {
            scope: Scope::Through(ScopedSlice::Orientation),
            check: Check::Fragments,
        },
    )
}

fn edit_source_index(snapshot: &mut BookSnapshot, edit: impl FnOnce(String) -> String) {
    let source_index = snapshot.book_files.get_mut(SOURCE_INDEX).unwrap();
    *source_index = edit(String::from_utf8(source_index.clone()).unwrap()).into_bytes();
}

fn assert_f009(report: &book_validation::ValidationReport) {
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "F009"),
        "{:#?}",
        report.diagnostics
    );
}

#[test]
fn each_mandatory_ledger_table_is_required() {
    for heading in [
        "Source roots",
        "Ownership blocks",
        "Fragment index",
        "Early uses",
    ] {
        let mut snapshot = support::corpus(true);
        edit_source_index(&mut snapshot, |text| {
            text.replacen(
                &format!("## {heading}\n"),
                &format!("## Missing {heading}\n"),
                1,
            )
        });

        assert_f009(&validate_final(&snapshot));
    }
}

#[test]
fn ledger_headings_inside_ordinary_fences_are_opaque() {
    let mut snapshot = support::corpus(true);
    snapshot
        .book_files
        .get_mut(SOURCE_INDEX)
        .unwrap()
        .extend_from_slice(b"\n```text\n## Source roots\n\n| not | a | ledger |\n```\n");

    let report = validate_final(&snapshot);
    assert!(report.valid, "{:#?}", report.diagnostics);
}

#[test]
fn malformed_source_root_row_is_rejected() {
    let mut snapshot = support::corpus(true);
    edit_source_index(&mut snapshot, |text| {
        text.replacen(
            "| `source-library` | `crates/ordinal-fs-tree/src/lib.rs` | 94 |",
            "|`source-library` | `crates/ordinal-fs-tree/src/lib.rs` | 94 |",
            1,
        )
    });

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn reordered_source_root_rows_are_rejected() {
    let mut snapshot = support::corpus(true);
    edit_source_index(&mut snapshot, |text| {
        let first = "| `source-crate-manifest` | `crates/ordinal-fs-tree/Cargo.toml` | 116 |\n";
        let second =
            "| `source-syllabus-cli` | `crates/ordinal-fs-tree/bin/syllabus.rs` | 1,439 |\n";
        text.replacen(&format!("{first}{second}"), &format!("{second}{first}"), 1)
    });

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn duplicated_ownership_row_is_rejected() {
    let mut snapshot = support::corpus(true);
    edit_source_index(&mut snapshot, |text| {
        let row = "| `library-crate-surface` | `source-library` | `orientation-k11` | `1-94` | 94 | `resolved` |\n";
        text.replacen(row, &format!("{row}{row}"), 1)
    });

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn ownership_state_must_match_the_directive_authority() {
    let mut snapshot = support::corpus(false);
    edit_source_index(&mut snapshot, |text| {
        text.replacen(
            "| `name-seam-source` | `source-name` | `name-seam-k12` | `1-716` | 716 | `deferred` |",
            "| `name-seam-source` | `source-name` | `name-seam-k12` | `1-716` | 716 | `resolved` |",
            1,
        )
    });

    assert_f009(&validate_orientation(&snapshot));
}

#[test]
fn extra_fragment_index_row_is_rejected() {
    let mut snapshot = support::corpus(true);
    edit_source_index(&mut snapshot, |text| {
        let heading = "\n## Early uses\n";
        let extra = "| `invented-fragment` | `orientation` | `source-library` | `literal` | `orientation-k11` | `1-1` | `source-library` | `—` |\n";
        text.replacen(heading, &format!("{extra}{heading}"), 1)
    });

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn fragment_index_relationships_must_match_directives() {
    let mut snapshot = support::corpus(true);
    edit_source_index(&mut snapshot, |text| {
        text.replacen(
            "| `library-crate-surface` | `orientation` | `source-library` | `literal` | `orientation-k11` | `1-94` | `source-library` | `—` |",
            "| `library-crate-surface` | `orientation` | `source-library` | `literal` | `orientation-k11` | `1-94` | `source-name` | `—` |",
            1,
        )
    });

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn early_use_status_must_match_the_current_prefix() {
    let mut snapshot = support::corpus(false);
    edit_source_index(&mut snapshot, |text| {
        text.replacen("| `pending` |", "| `explained` |", 1)
    });

    assert_f009(&validate_orientation(&snapshot));
}

#[test]
fn early_use_target_anchor_must_exist() {
    let mut snapshot = support::corpus(true);
    let orientation = snapshot
        .book_files
        .get_mut("docs/ordinal-fs-tree/book/01-orientation.md")
        .unwrap();
    *orientation = String::from_utf8(orientation.clone())
        .unwrap()
        .replace("<a id=\"insert-tour\"></a>\n", "")
        .into_bytes();

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn source_roots_must_be_declared_in_fixed_order() {
    let mut snapshot = support::corpus(true);
    edit_source_index(&mut snapshot, |text| {
        let first_start = text
            .find("<!-- source-root «source-crate-manifest»")
            .unwrap();
        let second_start = text.find("<!-- source-root «source-syllabus-cli»").unwrap();
        let third_start = text.find("<!-- source-root «source-library»").unwrap();
        let first = &text[first_start..second_start];
        let second = &text[second_start..third_start];
        format!(
            "{}{}{}{}",
            &text[..first_start],
            second,
            first,
            &text[third_start..]
        )
    });

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn source_root_directives_must_follow_their_table_before_the_next_h2() {
    let mut snapshot = support::corpus(true);
    edit_source_index(&mut snapshot, |text| {
        let roots_start = text
            .find("<!-- source-root «source-crate-manifest»")
            .unwrap();
        let ownership = text.find("\n## Ownership blocks\n").unwrap();
        let fragment_index = text.find("\n## Fragment index\n").unwrap();
        let roots = &text[roots_start..ownership];
        format!(
            "{}{}{}{}{}",
            &text[..roots_start],
            &text[ownership..fragment_index],
            "\n",
            roots,
            &text[fragment_index..]
        )
    });

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn source_roots_cannot_move_to_a_numbered_page() {
    let mut snapshot = support::corpus(true);
    let block = edit_out_root(&mut snapshot, "source-library", "source-conformance");
    snapshot
        .book_files
        .get_mut("docs/ordinal-fs-tree/book/01-orientation.md")
        .unwrap()
        .extend_from_slice(block.as_bytes());

    assert_f009(&validate_final(&snapshot));
}

#[test]
fn definitions_must_live_on_their_owners_canonical_numbered_page() {
    let mut snapshot = support::corpus(true);
    let source_page = "docs/ordinal-fs-tree/book/02-name-seam.md";
    let bytes = snapshot.book_files.get_mut(source_page).unwrap();
    let text = String::from_utf8(bytes.clone()).unwrap();
    let start = text.find("<!-- fragment «name-seam-source»").unwrap();
    let end =
        text[start..].find("<!-- /fragment -->\n").unwrap() + start + "<!-- /fragment -->\n".len();
    let definition = text[start..end].to_owned();
    *bytes = format!("{}{}", &text[..start], &text[end..]).into_bytes();
    snapshot
        .book_files
        .get_mut("docs/ordinal-fs-tree/book/01-orientation.md")
        .unwrap()
        .extend_from_slice(definition.as_bytes());

    let report = validate_final(&snapshot);
    let placement = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "F010")
        .expect("misplaced definition should produce F010");
    assert_eq!(
        placement.primary.path,
        "docs/ordinal-fs-tree/book/01-orientation.md"
    );
    assert!(placement.message.contains("02-name-seam.md"));
}

fn edit_out_root(snapshot: &mut BookSnapshot, root: &str, next_root: &str) -> String {
    let source_index = snapshot.book_files.get_mut(SOURCE_INDEX).unwrap();
    let text = String::from_utf8(source_index.clone()).unwrap();
    let start = text.find(&format!("<!-- source-root «{root}»")).unwrap();
    let end = text
        .find(&format!("<!-- source-root «{next_root}»"))
        .unwrap();
    let block = text[start..end].to_owned();
    *source_index = format!("{}{}", &text[..start], &text[end..]).into_bytes();
    block
}

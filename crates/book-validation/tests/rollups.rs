mod support;

use book_validation::{validate, BookSnapshot, Check, Request, Scope};

const ORIENTATION: &str = "docs/walkthroughs/ordinal-fs-tree/01-orientation.md";

/// Append one paragraph, with whatever directives precede it, to a page the
/// fixture already builds. Roll-up checking runs in the fragment phase, so these
/// snapshots are validated with `Check::Fragments` and no page-structure rule is
/// in play.
fn with_paragraph(text: &str) -> BookSnapshot {
    let mut snapshot = support::corpus(true);
    let page = snapshot.book_files.get_mut(ORIENTATION).unwrap();
    page.extend_from_slice(format!("\n{text}\n").as_bytes());
    snapshot
}

fn report(snapshot: &BookSnapshot) -> book_validation::ValidationReport {
    validate(
        snapshot,
        Request {
            scope: Scope::Final,
            check: Check::Fragments,
        },
    )
}

fn findings(snapshot: &BookSnapshot) -> Vec<String> {
    report(snapshot)
        .diagnostics
        .into_iter()
        .filter(|diagnostic| diagnostic.code == "F011")
        .map(|diagnostic| diagnostic.message)
        .collect()
}

fn one_finding(snapshot: &BookSnapshot) -> String {
    let found = findings(snapshot);
    assert_eq!(found.len(), 1, "{found:#?}");
    found.into_iter().next().unwrap()
}

fn roots() -> usize {
    support::manifest().roots().len()
}

/// The control. A book with no roll-up directive is unaffected by the rule, and
/// a book that states its derived figures is silent — so every finding below is
/// the mutation's and not the fixture's.
#[test]
fn a_book_that_states_its_derived_figures_reports_nothing() {
    assert!(findings(&support::corpus(true)).is_empty());
    assert!(findings(&with_paragraph(&format!(
        "<!-- rollup «source-roots» -->\nThe ledger carries {} roots.",
        roots()
    )))
    .is_empty());
}

#[test]
fn a_paragraph_that_does_not_state_the_derived_figure_is_reported() {
    let message = one_finding(&with_paragraph(&format!(
        "<!-- rollup «source-roots» -->\nThe ledger carries {} roots.",
        roots() + 1
    )));

    assert!(message.contains("`source-roots`"), "{message}");
    assert!(
        message.contains(&format!("derives {}", roots())),
        "{message}"
    );
}

/// The figure has to be the figure. An ordinal, a longer number and a suffixed
/// token all contain the digits and state something else.
#[test]
fn a_figure_inside_a_longer_token_does_not_state_the_roll_up() {
    for shape in [
        format!("{}th", roots()),
        format!("{}5", roots()),
        format!("{},000", roots()),
        format!("k{}", roots()),
        format!("{}.5", roots()),
    ] {
        let snapshot = with_paragraph(&format!(
            "<!-- rollup «source-roots» -->\nThe ledger carries {shape} roots."
        ));
        assert_eq!(findings(&snapshot).len(), 1, "{shape} was accepted");
    }
}

/// A sentence ending on the figure is the figure, and so is one that wraps
/// before it: prose is hard-wrapped, and the check reads the paragraph unwrapped.
#[test]
fn a_figure_at_a_sentence_end_or_across_a_line_break_states_the_roll_up() {
    assert!(findings(&with_paragraph(&format!(
        "<!-- rollup «source-roots» -->\nThe ledger's root count is {}.",
        roots()
    )))
    .is_empty());
    assert!(findings(&with_paragraph(&format!(
        "<!-- rollup «source-roots» -->\nThe ledger's root count is\n{}, and that is all.",
        roots()
    )))
    .is_empty());
}

/// The owned-source roll-up is a whole expression rather than one number, and
/// the books wrap it mid-sum. Its derived form comes back in the finding, which
/// is then restated — wrapped at one of its own spaces — and accepted.
#[test]
fn the_owned_source_sequence_is_matched_across_a_line_break() {
    let message = one_finding(&with_paragraph(
        "<!-- rollup «owned-lines-sequence» -->\nThe book owns nothing at all.",
    ));
    let sequence = message
        .split_once("derives ")
        .and_then(|(_, rest)| rest.split_once(", which"))
        .expect("the finding carries the derived expression")
        .0
        .to_owned();
    assert!(sequence.contains(" + "), "{sequence}");

    let (head, tail) = sequence.rsplit_once(' ').unwrap();
    assert!(findings(&with_paragraph(&format!(
        "<!-- rollup «owned-lines-sequence» -->\nThe chapters own {head}\n{tail} lines in all."
    )))
    .is_empty());
}

#[test]
fn a_directive_with_no_paragraph_below_it_is_reported() {
    let mut snapshot = support::corpus(true);
    let page = snapshot.book_files.get_mut(ORIENTATION).unwrap();
    page.extend_from_slice(b"\n<!-- rollup \xc2\xabsource-roots\xc2\xbb -->\n");

    assert!(
        one_finding(&snapshot).contains("not followed by a paragraph"),
        "{:#?}",
        findings(&snapshot)
    );
}

#[test]
fn an_unknown_quantity_and_a_misused_argument_are_reported() {
    for (directive, expected) in [
        (
            "<!-- rollup «source-rooms» -->",
            "is not a quantity the ledgers derive",
        ),
        (
            "<!-- rollup «source-roots» of=\"orientation-k11\" -->",
            "takes no `of=\"…\"` argument",
        ),
        (
            "<!-- rollup «ownership-blocks-owned-by» -->",
            "requires an `of=\"…\"` slice",
        ),
        (
            "<!-- rollup «ownership-blocks-owned-by» of=\"not-a-slice\" -->",
            "which is not a slice of this book",
        ),
        (
            "<!-- rollup «early-use-rows-at» of=\"01-orientation.md#nowhere\" -->",
            "which no early-use row gives as its first use",
        ),
    ] {
        let snapshot = with_paragraph(&format!("{directive}\nA paragraph with 1 figure."));
        let message = one_finding(&snapshot);
        assert!(message.contains(expected), "{message}");
    }
}

/// A well-formed line is a reconciliation finding; a malformed one is a parse
/// error, because `<!-- rollup` is a reserved prefix like every other directive.
#[test]
fn a_malformed_rollup_line_is_a_parse_error() {
    let snapshot = with_paragraph("<!-- rollup «source-roots» of=bare -->\nA paragraph.");
    let report = report(&snapshot);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "P001"),
        "{:#?}",
        report.diagnostics
    );
    assert!(findings(&snapshot).is_empty());
}

/// The three ledger accounts of a closed-ledgers section must be marked, so that
/// deleting a directive is a finding rather than a silent loss of cover.
#[test]
fn an_unmarked_ledger_account_in_the_closed_ledgers_section_is_reported() {
    let section = concat!(
        "<a id=\"the-closed-ledgers\"></a>\n",
        "## The closed ledgers\n",
        "\n",
        "**Ownership.** Some blocks.\n",
        "\n",
        "**Early use.** Some rows.\n",
        "\n",
        "**Owned source.** Some lines.\n",
        "\n",
        "<a id=\"after\"></a>\n",
        "\n",
        "**Ownership.** This one is outside the section.\n",
    );
    let found = findings(&with_paragraph(section));
    assert_eq!(found.len(), 3, "{found:#?}");
    for account in ["Ownership.", "Early use.", "Owned source."] {
        assert!(
            found.iter().any(|message| message.contains(account)),
            "{found:#?}"
        );
    }

    let marked = section.replace(
        "**Ownership.** Some blocks.",
        "<!-- rollup «ownership-blocks» -->\n**Ownership.** Some blocks.",
    );
    let found = findings(&with_paragraph(&marked));
    assert_eq!(found.len(), 3, "{found:#?}");
    assert!(
        found
            .iter()
            .any(|message| message.contains("`ownership-blocks` derives")),
        "{found:#?}"
    );
}

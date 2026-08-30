// `resolve`'s rendering: the `(stdout, stderr)` contract, unit-tested without a
// live verb dispatch.
//
// It moved here with the rendering itself at `loop-crate-verbs-k21`. The loop
// crate answers *what a reference names*; what a terminal should see about it is
// this binary's, so this is where the wording is pinned.

use grove_llm::cli::render_resolution;
use grove_loop::verbs::{Located, Resolution};
use grove_loop::{Handle, Outcome, Sought};
use std::path::PathBuf;

/// One matched entry, as the verb reports one.
fn located(path: &str, handle: &str, outcome: Outcome) -> Located {
    Located {
        path: PathBuf::from(path),
        handle: Handle::parse(handle).expect("a fixture handle must be well-formed"),
        kind: None,
        outcome,
    }
}

fn entry(path: &str, outcome: Outcome) -> Sought<Resolution> {
    Sought::Match(Resolution::Entry(located(path, "fixture-k1", outcome)))
}

fn ambiguous(matches: Vec<Located>) -> Sought<Resolution> {
    Sought::Match(Resolution::Ambiguous(matches))
}

#[test]
fn render_found_prints_path_no_stderr() {
    let r = entry("/g/.grove/03-impl--build-k5.md", Outcome::Live);
    let (out, err) = render_resolution("[5]", &r);
    assert_eq!(out, "/g/.grove/03-impl--build-k5.md\n");
    assert!(err.is_empty(), "got {err:?}");
}

#[test]
fn render_found_retired_notes_on_stderr_but_still_prints_path() {
    let r = entry("/g/.grove/02-DONE-impl--add-k4.md", Outcome::Done);
    let (out, err) = render_resolution("4", &r);
    assert_eq!(out, "/g/.grove/02-DONE-impl--add-k4.md\n");
    assert!(err.contains("retired"), "got {err:?}");
}

#[test]
fn render_found_abandoned_notes_on_stderr_but_still_prints_path() {
    // The abandoned counterpart of the DONE case above: a resolved
    // `ABANDONED` entry must get its own stderr note, not silence
    // (silence is what a live match gets) and not the DONE wording.
    let r = entry(
        "/g/.grove/01-ABANDONED-impl--spike-k1.md",
        Outcome::Abandoned,
    );
    let (out, err) = render_resolution("1", &r);
    assert_eq!(out, "/g/.grove/01-ABANDONED-impl--spike-k1.md\n");
    assert!(err.contains("abandoned"), "got {err:?}");
    assert!(!err.contains("retired"), "got {err:?}");
}

#[test]
fn render_not_found_empty_stdout_diagnostic_stderr() {
    let (out, err) = render_resolution("nope", &Sought::Nothing);
    assert!(out.is_empty(), "got {out:?}");
    assert!(err.contains("no entry matches"), "got {err:?}");
    assert!(err.contains("nope"), "got {err:?}");
}

#[test]
fn render_ambiguous_lists_keys_on_stderr_empty_stdout() {
    let r = ambiguous(vec![
        located(
            "/g/.grove/01-design-k1/01-impl--add-k2.md",
            "add-k2",
            Outcome::Live,
        ),
        located("/g/.grove/02-DONE-impl--add-k4.md", "add-k4", Outcome::Done),
    ]);
    let (out, err) = render_resolution("add", &r);
    assert!(
        out.is_empty(),
        "stdout must be empty for ambiguous; got {out:?}"
    );
    assert!(err.contains("ambiguous"), "got {err:?}");
    assert!(err.contains("[2]"), "got {err:?}");
    assert!(err.contains("[4]"), "got {err:?}");
    assert!(err.contains("retired"), "got {err:?}");
}

#[test]
fn render_ambiguous_tags_an_abandoned_match() {
    let r = ambiguous(vec![located(
        "/g/.grove/01-ABANDONED-impl--spike-k1.md",
        "spike-k1",
        Outcome::Abandoned,
    )]);
    let (_out, err) = render_resolution("spike", &r);
    assert!(
        err.contains("[1]") && err.contains("(abandoned)"),
        "got {err:?}"
    );
}

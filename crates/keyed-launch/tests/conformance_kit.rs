//! The kit itself: what it passes, what it fails, and that it cannot report
//! conformance for a document it did not exercise.

use std::fs;
use std::path::{Path, PathBuf};

use keyed_launch::{conformance, Requirement, SlotRule, Vocabulary};
use tempfile::TempDir;

const SLOTS: [SlotRule<'static>; 2] = [
    SlotRule {
        name: "prompt",
        requirement: Requirement::ExactlyOnce,
    },
    SlotRule {
        name: "label",
        requirement: Requirement::AtMostOnce,
    },
];

fn vocabulary() -> Vocabulary<'static> {
    Vocabulary { slots: &SLOTS }
}

fn write(document: &str) -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config.kdl");
    fs::write(&path, document).unwrap();
    (dir, path)
}

#[test]
fn a_conforming_configuration_passes() {
    let (_dir, path) = write("one \"wrapper ${prompt}\"\ntwo \"other ${label} ${prompt}\"\n");
    let outcome = conformance::check(&path, vocabulary());
    assert!(outcome.passed(), "{}", outcome.failures.join("\n"));
}

#[test]
fn a_violated_template_rule_is_reported_as_a_failure() {
    let (_dir, path) = write("one \"wrapper\"\n");
    let outcome = conformance::check(&path, vocabulary());
    assert!(!outcome.passed());
    assert!(
        outcome
            .failures
            .iter()
            .any(|failure| failure.contains("must contain `${prompt}` exactly once")),
        "{:?}",
        outcome.failures
    );
}

/// A kit that only reports violations reads exactly the same when it is handed
/// nothing to check. An empty document is therefore a failure in its own right.
#[test]
fn a_document_that_exercises_nothing_does_not_pass() {
    let (_dir, path) = write("// nothing at all\n");
    let outcome = conformance::check(&path, vocabulary());
    assert!(!outcome.passed());
    assert!(
        outcome
            .failures
            .iter()
            .any(|failure| failure.contains("declares no keys")),
        "{:?}",
        outcome.failures
    );
}

#[test]
fn a_missing_file_is_a_failure_rather_than_a_panic() {
    let dir = TempDir::new().unwrap();
    let outcome = conformance::check(&dir.path().join("absent.kdl"), vocabulary());
    assert!(!outcome.passed());
    assert!(
        outcome.failures[0].contains("configuration is missing at"),
        "{:?}",
        outcome.failures
    );
}

/// The kit reads the path it is handed and nothing beside it: no overlay is
/// searched for, because *which* files take part is the consumer's question.
#[test]
fn the_kit_reads_only_the_file_it_is_given() {
    let (dir, path) = write("one \"wrapper ${prompt}\"\n");
    fs::write(dir.path().join(".grove.kdl"), "two \"broken\"\n").unwrap();
    let outcome = conformance::check(&path, vocabulary());
    assert!(outcome.passed(), "{}", outcome.failures.join("\n"));
    let _: &Path = path.as_path();
}

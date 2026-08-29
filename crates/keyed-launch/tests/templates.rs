//! The crate's public interface, exercised without any consumer.
//!
//! Every case here is about a *key* and a *slot* — there is no session, no kind
//! and no grove — which is the point: if one of these tests needed a domain to
//! state its expectation, the boundary would be in the wrong place.

use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};

use keyed_launch::{ConfigError, Requirement, Slot, SlotRule, Templates, Vocabulary};
use tempfile::TempDir;

/// A vocabulary with one required slot and one optional one — enough to
/// exercise both cardinalities without inventing a third rule.
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

fn write(dir: &Path, name: &str, document: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, document).unwrap();
    path
}

fn load(document: &str) -> (TempDir, Result<Templates, ConfigError>) {
    let dir = TempDir::new().unwrap();
    let path = write(dir.path(), "config.kdl", document);
    let loaded = Templates::load(&path, None, vocabulary());
    (dir, loaded)
}

fn load_error(document: &str) -> String {
    let (_dir, loaded) = load(document);
    loaded.err().expect("expected a load failure").to_string()
}

fn values<'a>(prompt: &'a OsStr, label: &'a OsStr) -> [Slot<'a>; 2] {
    [
        Slot {
            name: "prompt",
            value: prompt,
        },
        Slot {
            name: "label",
            value: label,
        },
    ]
}

fn expand_words(templates: &Templates, key: &str, prompt: &str) -> Vec<OsString> {
    let prompt = OsString::from(prompt);
    let label = OsString::from("L");
    templates
        .expand(key, &values(&prompt, &label))
        .unwrap()
        .words()
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "expected to find {needle:?} in:\n{haystack}"
    );
}

// ---------------------------------------------------------------------------
// Expansion

/// A template is split into words once, at load, and a slot's value becomes
/// exactly one argument however many spaces it holds. Nothing re-splits it, so a
/// prompt is never re-parsed as several arguments.
#[test]
fn a_slot_value_is_one_argument_whatever_it_contains() {
    let (_dir, loaded) = load(r#"run "wrapper --flag 'a b' ${prompt}""#);
    let templates = loaded.unwrap();
    let words = expand_words(&templates, "run", "two words $(not a command)");
    assert_eq!(
        words,
        vec![
            OsString::from("wrapper"),
            OsString::from("--flag"),
            OsString::from("a b"),
            OsString::from("two words $(not a command)"),
        ]
    );
}

#[test]
fn program_and_arguments_split_at_word_zero() {
    let (_dir, loaded) = load(r#"run "wrapper --flag ${prompt}""#);
    let templates = loaded.unwrap();
    let prompt = OsString::from("P");
    let label = OsString::from("L");
    let argv = templates.expand("run", &values(&prompt, &label)).unwrap();
    assert_eq!(argv.program(), OsStr::new("wrapper"));
    assert_eq!(
        argv.args(),
        [OsString::from("--flag"), OsString::from("P")].as_slice()
    );
}

/// Shell metacharacters are literal arguments: nothing is handed to a shell, so
/// a pipe is a word.
#[test]
fn shell_metacharacters_stay_literal() {
    let (_dir, loaded) = load(r#"run "wrapper '|' '>' ${prompt}""#);
    let words = expand_words(&loaded.unwrap(), "run", "P");
    assert_eq!(
        words,
        vec![
            OsString::from("wrapper"),
            OsString::from("|"),
            OsString::from(">"),
            OsString::from("P"),
        ]
    );
}

/// The values must fill the slots the **vocabulary** declared, not merely the
/// ones this template happens to mention — so a consumer cannot have a call that
/// works for one key and fails for its neighbour.
#[test]
fn expansion_refuses_values_that_do_not_fill_the_vocabulary() {
    let (_dir, loaded) = load(r#"run "wrapper ${prompt}""#);
    let templates = loaded.unwrap();
    let prompt = OsString::from("P");

    let missing = templates
        .expand(
            "run",
            &[Slot {
                name: "prompt",
                value: &prompt,
            }],
        )
        .err()
        .expect("a missing value is a refusal")
        .to_string();
    assert_contains(&missing, "no value offered for declared slot: label");

    let label = OsString::from("L");
    let unknown = templates
        .expand(
            "run",
            &[
                Slot {
                    name: "prompt",
                    value: &prompt,
                },
                Slot {
                    name: "label",
                    value: &label,
                },
                Slot {
                    name: "elsewhere",
                    value: &label,
                },
            ],
        )
        .err()
        .expect("an undeclared slot is a refusal")
        .to_string();
    assert_contains(&unknown, "no slot named `elsewhere` is declared");
    assert_contains(&unknown, "declared slots: prompt, label");
}

// ---------------------------------------------------------------------------
// The primary declares; the overlay overrides

#[test]
fn an_overlay_replaces_a_whole_template_and_reports_its_own_path() {
    let dir = TempDir::new().unwrap();
    let primary = write(
        dir.path(),
        "config.kdl",
        "one \"first ${prompt}\"\ntwo \"second ${prompt}\"\n",
    );
    let overlay = write(dir.path(), "overlay.kdl", "two \"replaced ${prompt}\"\n");

    let templates = Templates::load(&primary, Some(&overlay), vocabulary()).unwrap();

    assert_eq!(
        expand_words(&templates, "one", "P")[0],
        OsString::from("first")
    );
    assert_eq!(
        expand_words(&templates, "two", "P")[0],
        OsString::from("replaced")
    );
    assert_eq!(templates.source("one"), Some(primary.as_path()));
    assert_eq!(templates.source("two"), Some(overlay.as_path()));
}

/// The per-key restatement of what a completeness quantifier used to buy: a
/// second source cannot introduce a program the operator never chose.
#[test]
fn a_key_only_the_overlay_declares_does_not_resolve() {
    let dir = TempDir::new().unwrap();
    let primary = write(dir.path(), "config.kdl", "one \"first ${prompt}\"\n");
    let overlay = write(dir.path(), "overlay.kdl", "two \"smuggled ${prompt}\"\n");

    let templates = Templates::load(&primary, Some(&overlay), vocabulary()).unwrap();

    assert_eq!(templates.source("two"), None);
    let refusal = templates.require("two").err().unwrap().to_string();
    assert_contains(&refusal, "key `two` does not resolve");
    assert_contains(&refusal, &overlay.display().to_string());
    assert_contains(&refusal, &format!("Declare `two` in {}", primary.display()));
    // And the same refusal reaches the caller through expansion, which is the
    // other moment a key is committed to.
    let prompt = OsString::from("P");
    let label = OsString::from("L");
    assert_contains(
        &templates
            .expand("two", &values(&prompt, &label))
            .err()
            .unwrap()
            .to_string(),
        "an overlay overrides a key the primary declares but never supplies one of its own",
    );
}

#[test]
fn a_key_nobody_declares_names_the_primary_file() {
    let dir = TempDir::new().unwrap();
    let primary = write(dir.path(), "config.kdl", "one \"first ${prompt}\"\n");
    let templates = Templates::load(&primary, None, vocabulary()).unwrap();
    let refusal = templates.require("absent").err().unwrap().to_string();
    assert_contains(&refusal, "key `absent` does not resolve");
    assert_contains(
        &refusal,
        &format!("Declare `absent` in {}", primary.display()),
    );
}

/// An overlay is held to every rule the primary is, and its diagnostics name its
/// own path — including for a key the primary never declares, which is validated
/// before it is set aside.
#[test]
fn an_invalid_overlay_fails_the_load_against_its_own_path() {
    let dir = TempDir::new().unwrap();
    let primary = write(dir.path(), "config.kdl", "one \"first ${prompt}\"\n");
    let overlay = write(dir.path(), "overlay.kdl", "three \"no slot here\"\n");

    let error = Templates::load(&primary, Some(&overlay), vocabulary())
        .err()
        .unwrap()
        .to_string();
    assert_contains(
        &error,
        &format!("invalid configuration overlay at {}", overlay.display()),
    );
    assert_contains(&error, "must contain `${prompt}` exactly once");
}

#[test]
fn an_unreadable_overlay_fails_closed() {
    let dir = TempDir::new().unwrap();
    let primary = write(dir.path(), "config.kdl", "one \"first ${prompt}\"\n");
    let missing = dir.path().join("nowhere.kdl");
    let error = Templates::load(&primary, Some(&missing), vocabulary())
        .err()
        .unwrap()
        .to_string();
    assert_contains(&error, "failed to read the configuration overlay at");
}

// ---------------------------------------------------------------------------
// Validation is document-eager

#[test]
fn a_missing_primary_names_its_path() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config.kdl");
    let error = Templates::load(&path, None, vocabulary())
        .err()
        .unwrap()
        .to_string();
    assert_contains(&error, "configuration is missing at");
    assert_contains(&error, &path.display().to_string());
}

#[test]
fn a_kdl_syntax_error_names_its_source_location() {
    let error = load_error("one \"unterminated\nrun\n");
    assert_contains(&error, "KDL syntax error");
    assert_contains(&error, ":1:");
}

/// Aggregate, not first-error: one report carries every problem in the document.
#[test]
fn schema_and_template_failures_are_aggregated_with_source_locations() {
    let error = load_error(concat!(
        "one \"wrapper ${prompt}\" extra=1\n",
        "two \"wrapper\"\n",
        "three \"wrapper ${prompt} ${label} ${label}\"\n",
        "four \"wrapper ${unknown} ${prompt}\"\n",
        "five \"wrapper pre${prompt} ${prompt}\"\n",
    ));
    for expected in [
        "invalid configuration at",
        // A node-shape problem is reported at its location without the key
        // prefix: the key is what the offending line *is*, and the line and
        // column already point at it.
        "properties and child blocks are not allowed",
        "key `two`: command template must contain `${prompt}` exactly once",
        "key `three`: `${label}` may appear at most once",
        "key `four`: unknown substitution `${unknown}`",
        "key `five`: substitutions must occupy a complete shell word, got `pre${prompt}`",
        ":2:1:",
    ] {
        assert_contains(&error, expected);
    }
}

#[test]
fn a_duplicate_key_reports_every_declaration_location() {
    let error = load_error("one \"a ${prompt}\"\none \"b ${prompt}\"\n");
    assert_contains(&error, "duplicate key `one`");
    assert_contains(&error, ":1:1,");
    assert_contains(&error, ":2:1");
}

#[test]
fn word_zero_must_be_a_literal_executable() {
    assert_contains(
        &load_error("one \"${prompt}\"\n"),
        "word zero must be a literal executable",
    );
    assert_contains(
        &load_error("one \"\"\n"),
        "word zero must be a literal non-empty executable",
    );
}

/// `shell_words::split` treats an unquoted `#` as a comment and silently drops
/// the rest of the line, so the template is refused instead of truncated.
#[test]
fn an_unquoted_hash_is_refused_rather_than_truncating_the_argv() {
    let error = load_error("one \"wrapper # ${prompt}\"\n");
    assert_contains(&error, "`#` starts a comment in a command template");
}

#[test]
fn quoted_and_midword_hashes_stay_literal() {
    let (_dir, loaded) = load(r##"one "wrapper '#tag' mid#word ${prompt}""##);
    let words = expand_words(&loaded.unwrap(), "one", "P");
    assert_eq!(
        words,
        vec![
            OsString::from("wrapper"),
            OsString::from("#tag"),
            OsString::from("mid#word"),
            OsString::from("P"),
        ]
    );
}

#[test]
fn unmatched_quotes_are_refused() {
    assert_contains(
        &load_error("one \"wrapper 'unclosed ${prompt}\"\n"),
        "command template has unmatched quotes",
    );
}

/// A key the crate has never heard of is not a diagnostic. There is no key set:
/// what a key means is the consumer's, and one nobody asks for costs nothing.
#[test]
fn an_unused_key_is_not_an_error() {
    let (_dir, loaded) = load("anything-at-all \"wrapper ${prompt}\"\n");
    let templates = loaded.unwrap();
    assert_eq!(templates.keys(), vec!["anything-at-all"]);
}

/// A duplicated slot name would be counted twice against its own cardinality and
/// would take whichever value arrived first — a consumer bug that looks like a
/// template bug for as long as it goes unnamed.
#[test]
fn a_duplicated_slot_name_is_refused_at_load() {
    let dir = TempDir::new().unwrap();
    let path = write(dir.path(), "config.kdl", "one \"wrapper ${prompt}\"\n");
    let slots = [
        SlotRule {
            name: "prompt",
            requirement: Requirement::ExactlyOnce,
        },
        SlotRule {
            name: "prompt",
            requirement: Requirement::AtMostOnce,
        },
    ];
    let error = Templates::load(&path, None, Vocabulary { slots: &slots })
        .err()
        .unwrap()
        .to_string();
    assert_contains(&error, "declares `prompt` more than once");
}

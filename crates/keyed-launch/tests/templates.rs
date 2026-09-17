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
    let (_dir, loaded) = load("config {\n    command \"run\" \"wrapper --flag 'a b' ${prompt}\"\n    bind \"run\" \"run\"\n    route \"run\" \"run\"\n}\n");
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
    let (_dir, loaded) = load("config {\n    command \"run\" \"wrapper --flag ${prompt}\"\n    bind \"run\" \"run\"\n    route \"run\" \"run\"\n}\n");
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
    let (_dir, loaded) = load("config {\n    command \"run\" \"wrapper '|' '>' ${prompt}\"\n    bind \"run\" \"run\"\n    route \"run\" \"run\"\n}\n");
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
    let (_dir, loaded) = load("config {\n    command \"run\" \"wrapper ${prompt}\"\n    bind \"run\" \"run\"\n    route \"run\" \"run\"\n}\n");
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
        .expect_err("a missing value is a refusal")
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
        .expect_err("an undeclared slot is a refusal")
        .to_string();
    assert_contains(&unknown, "no slot named `elsewhere` is declared");
    assert_contains(&unknown, "declared slots: prompt, label");
}

// ---------------------------------------------------------------------------
// The primary declares; the overlay overrides

#[test]
fn an_overlay_redirects_a_command_and_retains_both_sources() {
    let dir = TempDir::new().unwrap();
    let primary = write(
        dir.path(),
        "config.kdl",
        r#"config {
    command "first" "first ${prompt}"
    command "second" "second ${prompt}"
    command "replacement" "replaced ${prompt}"
    bind "one" "first"
    bind "two" "second"
    route "one" "one"
    route "two" "two"
}"#,
    );
    let overlay = write(
        dir.path(),
        "overlay.kdl",
        "config { bind \"two\" \"replacement\"; route \"two\" \"two\"; }\n",
    );

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
    assert_eq!(templates.source("two"), Some(primary.as_path()));
    let view = templates.inspect();
    let history = view
        .histories
        .iter()
        .find(|history| {
            history.setting
                == keyed_launch::Setting::BindingTarget {
                    binding: "two".into(),
                }
        })
        .unwrap();
    let assignment = history.assignments.last().unwrap();
    assert_eq!(
        assignment.value,
        keyed_launch::AssignmentValue::Set("replacement".into())
    );
    assert_eq!(view.origins[assignment.origin].span.source.path, overlay);
}

/// The per-key restatement of what a completeness quantifier used to buy: a
/// second source cannot introduce a program the operator never chose.
#[test]
fn a_key_only_the_overlay_declares_does_not_resolve() {
    let dir = TempDir::new().unwrap();
    let primary = write(dir.path(), "config.kdl", "config { command \"first\" \"first ${prompt}\"; bind \"lead\" \"first\"; route \"one\" \"lead\"; }\n");
    let overlay = write(
        dir.path(),
        "overlay.kdl",
        "config { route \"two\" \"lead\"; }\n",
    );

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
    let primary = write(dir.path(), "config.kdl", "config {\n    command \"one\" \"first ${prompt}\"\n    bind \"one\" \"one\"\n    route \"one\" \"one\"\n}\n");
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
    let primary = write(dir.path(), "config.kdl", "config {\n    command \"one\" \"first ${prompt}\"\n    bind \"one\" \"one\"\n    route \"one\" \"one\"\n}\n");
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
    let error = load_error("config {\n    command \"one\" \"unterminated\n}\n");
    assert_contains(&error, "KDL syntax error");
    assert_contains(&error, ":4:1:");
}

/// Semantic reports aggregate after the document passes structural validation.
#[test]
fn template_failures_are_aggregated_with_source_locations() {
    let document = r#"config {
    command "one" "wrapper ${prompt}"
    command "two" "wrapper"
    command "three" "wrapper ${prompt} ${label} ${label}"
    command "four" "wrapper ${unknown} ${prompt}"
    command "five" "wrapper pre${prompt} ${prompt}"
    bind "one" "one"
    bind "two" "two"
    bind "three" "three"
    bind "four" "four"
    bind "five" "five"
    route "one" "one"
    route "two" "two"
    route "three" "three"
    route "four" "four"
    route "five" "five"
}"#;
    let (dir, loaded) = load(document);
    let error = loaded.err().unwrap();
    let diagnostics = error.diagnostics();
    assert_eq!(diagnostics.len(), 4);
    for (diagnostic, command) in diagnostics.iter().zip(["two", "three", "four", "five"]) {
        assert_eq!(diagnostic.category, "invalid_template");
        assert_eq!(diagnostic.command.as_deref(), Some(command));
        let span = diagnostic.primary.as_ref().unwrap();
        assert_eq!(span.source.path, dir.path().join("config.kdl"));
        assert!(document[span.start..span.end].starts_with(&format!("command {command:?}")));
    }
    let error = error.to_string();
    for expected in [
        "must contain `${prompt}` exactly once",
        "`${label}` may appear at most once",
        "unknown substitution `${unknown}`",
        "runtime substitutions must occupy a whole argument",
    ] {
        assert_contains(&error, expected);
    }
}

/// The child-block arm of the same rule the aggregate test reaches through
/// `extra=1`. A child block is refused for the reason a property is: a key is a
/// line, and anything hanging off it is a shape the loader has no meaning for.
#[test]
fn a_child_block_is_refused_like_a_property() {
    let error = load_error("one \"wrapper ${prompt}\" {\n    two \"wrapper ${prompt}\"\n}\n");
    assert_contains(&error, "properties and child blocks are not allowed");
}

/// `kdl` 4.7 is a KDL **1.0** parser, where an annotation is a parenthesised
/// name immediately before the node or before the value — the two places this
/// rule looks. https://github.com/kdl-org/kdl/blob/1.0.0/SPEC.md#type-annotation
#[test]
fn a_type_annotation_is_refused_on_the_node_and_on_its_argument() {
    assert_contains(
        &load_error("(shell)one \"wrapper ${prompt}\"\n"),
        "type annotations are not allowed",
    );
    assert_contains(
        &load_error("one (string)\"wrapper ${prompt}\"\n"),
        "type annotations are not allowed",
    );
}

/// Both directions of the count, because the rule is `!= 1` rather than a
/// missing-argument check: a second template on the line is as wrong as none.
#[test]
fn a_key_needs_exactly_one_positional_argument() {
    assert_contains(
        &load_error("one\n"),
        "a key must have exactly one positional argument",
    );
    assert_contains(
        &load_error("one \"wrapper ${prompt}\" \"wrapper ${prompt}\"\n"),
        "a key must have exactly one positional argument",
    );
}

/// A number, a boolean and a null are all valid KDL values that a template can
/// never be, and each takes the same arm. The crate names none of them, so the
/// three are asserted rather than assumed to travel together.
#[test]
fn a_keys_sole_argument_must_be_a_string() {
    for document in ["one 42\n", "one true\n", "one null\n"] {
        assert_contains(
            &load_error(document),
            "a key's sole argument must be a string",
        );
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
        &load_error("config {\n    command \"one\" \"${prompt}\"\n    bind \"one\" \"one\"\n    route \"one\" \"one\"\n}\n"),
        "word zero must be a literal non-empty executable",
    );
    assert_contains(
        &load_error("config {\n    command \"one\" \"\"\n    bind \"one\" \"one\"\n    route \"one\" \"one\"\n}\n"),
        "word zero must be a literal non-empty executable",
    );
}

/// `shell_words::split` treats an unquoted `#` as a comment and silently drops
/// the rest of the line, so the template is refused instead of truncated.
#[test]
fn an_unquoted_hash_is_refused_rather_than_truncating_the_argv() {
    let error = load_error("config {\n    command \"one\" \"wrapper # ${prompt}\"\n    bind \"one\" \"one\"\n    route \"one\" \"one\"\n}\n");
    assert_contains(&error, "quote literal comment-starting `#`");
}

#[test]
fn quoted_and_midword_hashes_stay_literal() {
    let (_dir, loaded) = load("config {\n    command \"one\" \"wrapper '#tag' mid#word ${prompt}\"\n    bind \"one\" \"one\"\n    route \"one\" \"one\"\n}\n");
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
        &load_error("config {\n    command \"one\" \"wrapper 'unclosed ${prompt}\"\n    bind \"one\" \"one\"\n    route \"one\" \"one\"\n}\n"),
        "command template has unmatched quotes",
    );
}

/// A key the crate has never heard of is not a diagnostic. There is no key set:
/// what a key means is the consumer's, and one nobody asks for costs nothing.
#[test]
fn an_unused_key_is_not_an_error() {
    let (_dir, loaded) = load("config {\n    command \"anything-at-all\" \"wrapper ${prompt}\"\n    bind \"anything-at-all\" \"anything-at-all\"\n    route \"anything-at-all\" \"anything-at-all\"\n}\n");
    let templates = loaded.unwrap();
    assert_eq!(templates.keys(), vec!["anything-at-all"]);
}

/// A duplicated slot name would be counted twice against its own cardinality and
/// would take whichever value arrived first — a consumer bug that looks like a
/// template bug for as long as it goes unnamed.
#[test]
fn a_duplicated_slot_name_is_refused_at_load() {
    let dir = TempDir::new().unwrap();
    let path = write(dir.path(), "config.kdl", "config {\n    command \"one\" \"wrapper ${prompt}\"\n    bind \"one\" \"one\"\n    route \"one\" \"one\"\n}\n");
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

#[test]
fn nul_templates_fail_eagerly_in_both_sources_with_real_spans() {
    use keyed_launch::Catalog;

    let dir = TempDir::new().unwrap();
    let primary = write(
        dir.path(),
        "primary.kdl",
        "run \"bad\\u{0}program ${prompt}\"\nother \"ok ${prompt}\"\n",
    );
    let overlay = write(
        dir.path(),
        "overlay.kdl",
        "run \"good ${prompt}\"\nother \"ok 'bad\\u{0}argument' ${prompt}\"\n",
    );
    let catalog_error = Catalog::load(&primary, Some(&overlay), vocabulary())
        .err()
        .expect("NUL templates must fail before resolution");
    let convenience_error = Templates::load(&primary, Some(&overlay), vocabulary())
        .err()
        .expect("convenience loading must reject the same templates");
    assert_eq!(catalog_error.diagnostics(), convenience_error.diagnostics());
    let diagnostics = catalog_error.diagnostics();
    assert_eq!(diagnostics.len(), 2);
    for (diagnostic, path, key) in [
        (&diagnostics[0], &primary, "run"),
        (&diagnostics[1], &overlay, "other"),
    ] {
        assert_eq!(diagnostic.category, "invalid_template");
        assert_eq!(diagnostic.key.as_deref(), Some(key));
        let span = diagnostic
            .primary
            .as_ref()
            .expect("actual template location");
        assert_eq!(&span.source.path, path);
        assert!(fs::read_to_string(path).unwrap()[span.start..span.end].contains("\\u{0}"));
        assert!(diagnostic.message.contains("NUL"));
        assert!(diagnostic.remedy.contains("NUL"));
    }
}

#[test]
fn nul_runtime_values_fail_even_for_unused_optional_slots() {
    let dir = TempDir::new().unwrap();
    let primary = write(dir.path(), "primary.kdl", "config { command \"original\" \"original ${prompt}\"; command \"replacement\" \"replacement ${prompt}\"; bind \"lead\" \"original\"; route \"run\" \"lead\"; }\n");
    let overlay = write(
        dir.path(),
        "overlay.kdl",
        "config { bind \"lead\" \"replacement\"; route \"run\" \"lead\"; }\n",
    );
    let templates = Templates::load(&primary, Some(&overlay), vocabulary()).unwrap();
    for (prompt, label, slot) in [
        ("bad\0value", "ok", "prompt"),
        ("ok", "bad\0value", "label"),
    ] {
        let error = templates
            .expand("run", &values(OsStr::new(prompt), OsStr::new(label)))
            .expect_err("NUL values must not produce Argv");
        let diagnostics = error.diagnostics();
        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.category, "invalid_value");
        assert_eq!(diagnostic.key.as_deref(), Some("run"));
        assert_eq!(diagnostic.source.as_ref().unwrap().path, primary);
        assert!(diagnostic.primary.is_none());
        assert!(diagnostic.message.contains(slot));
        assert!(diagnostic.message.contains("NUL"));
        assert!(diagnostic.remedy.contains("NUL"));
    }
}

#[test]
fn empty_and_parameter_looking_runtime_values_stay_opaque_words() {
    let (_dir, loaded) = load("config {\n    command \"run\" \"wrapper ${prompt} ${label}\"\n    bind \"run\" \"run\"\n    route \"run\" \"run\"\n}\n");
    let templates = loaded.unwrap();
    let opaque = OsStr::new("quotes ' \" ${param.name} ${prompt} # ; $(anything)");
    assert_eq!(
        templates
            .expand("run", &values(opaque, OsStr::new("")))
            .unwrap()
            .words(),
        vec![
            OsString::from("wrapper"),
            opaque.to_owned(),
            OsString::new()
        ]
    );
}

#[cfg(unix)]
#[test]
fn native_runtime_values_preserve_non_unicode_and_reject_embedded_nul() {
    use std::os::unix::ffi::OsStrExt;

    let (_dir, loaded) = load("config {\n    command \"run\" \"wrapper ${prompt}\"\n    bind \"run\" \"run\"\n    route \"run\" \"run\"\n}\n");
    let templates = loaded.unwrap();
    let native = OsStr::from_bytes(b"path/\xff two words");
    let argv = templates
        .expand("run", &values(native, OsStr::new("")))
        .unwrap();
    assert_eq!(argv.args()[0].as_bytes(), native.as_bytes());
    let with_nul = OsStr::from_bytes(b"path/\xff\0suffix");
    let error = templates
        .expand("run", &values(with_nul, OsStr::new("")))
        .unwrap_err();
    assert_eq!(error.diagnostics()[0].category, "invalid_value");
}

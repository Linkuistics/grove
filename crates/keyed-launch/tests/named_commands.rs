use std::ffi::OsStr;
use std::fs;

use keyed_launch::{
    AssignmentValue, Catalog, Requirement, Selection, Setting, Slot, SlotRule, Templates,
    Vocabulary,
};
use tempfile::TempDir;

fn vocabulary() -> Vocabulary<'static> {
    Vocabulary {
        slots: &[SlotRule {
            name: "payload",
            requirement: Requirement::ExactlyOnce,
        }],
    }
}

fn load(primary: &str, overlay: Option<&str>) -> Result<Templates, keyed_launch::ConfigError> {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("personal.kdl");
    let o = dir.path().join("local.kdl");
    fs::write(&p, primary).unwrap();
    if let Some(text) = overlay {
        fs::write(&o, text).unwrap();
    }
    Templates::load(&p, overlay.map(|_| o.as_path()), vocabulary())
}

const BASE: &str = r#"
config "flat ${payload}"
config {
    command "shared" "runner 'one argument' '' $${escaped} ${payload}"
    command "other" "alternate ${payload}"
    command "dormant" "'broken"
    bind "lead" "shared"
    route "alpha" "lead"
    route "beta" "lead"
}
"#;

#[test]
fn shared_commands_capture_inspect_and_expand_without_sources() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("primary.kdl");
    fs::write(&path, BASE).unwrap();
    let catalog = Catalog::load(&path, None, vocabulary()).unwrap();
    let convenience = Templates::load(&path, None, vocabulary()).unwrap();
    fs::remove_file(&path).unwrap();
    let templates = catalog.resolve(&Selection::default()).unwrap();
    drop(catalog);
    assert_eq!(templates.inspect(), convenience.inspect());
    assert_eq!(templates.keys(), ["alpha", "beta", "config"]);
    for key in ["alpha", "beta"] {
        let argv = templates
            .expand(
                key,
                &[Slot {
                    name: "payload",
                    value: OsStr::new("native value"),
                }],
            )
            .unwrap();
        assert_eq!(argv.program(), OsStr::new("runner"));
        assert_eq!(
            argv.args(),
            ["one argument", "", "${escaped}", "native value"].map(OsStr::new)
        );
        let view = templates
            .inspect()
            .commands
            .iter()
            .find(|c| c.key == key)
            .unwrap();
        assert_eq!(view.binding.as_deref(), Some("lead"));
        assert_eq!(view.command.as_deref(), Some("shared"));
        assert_eq!(view.origins.len(), 3);
        assert_eq!(templates.source(key), Some(path.as_path()));
    }
}

#[test]
fn local_targets_replace_before_validation_and_preserve_histories() {
    let primary = r#"config {
        command "good" "runner ${payload}"
        bind "lead" "missing"
        route "alpha" "missing-binding"
        route "beta" "lead"
    }"#;
    let overlay = r#"beta "literal ${payload}"
    config {
        bind "lead" "good"
        route "alpha" "lead"
        route "local-only" "missing"
    }"#;
    let templates = load(primary, Some(overlay)).unwrap();
    assert!(templates.require("local-only").is_err());
    let view = templates.inspect();
    assert_eq!(view.non_admitted_keys[0].key, "local-only");
    let history = view
        .histories
        .iter()
        .find(|h| {
            h.setting
                == Setting::BindingTarget {
                    binding: "lead".into(),
                }
        })
        .unwrap();
    assert_eq!(
        history
            .assignments
            .iter()
            .map(|a| &a.value)
            .collect::<Vec<_>>(),
        [
            &AssignmentValue::Set("missing".into()),
            &AssignmentValue::Set("good".into())
        ]
    );
    let beta = view.commands.iter().find(|c| c.key == "beta").unwrap();
    assert_eq!(beta.binding, None);
    assert_eq!(beta.command, None);
    let alpha = view.commands.iter().find(|c| c.key == "alpha").unwrap();
    assert_eq!(alpha.command.as_deref(), Some("good"));
}

#[test]
fn active_bindings_validate_even_without_routes_but_unused_definitions_do_not() {
    load("config { command \"dormant\" \"'broken\"; }", None).unwrap();
    for body in [
        "command \"bad\" \"'broken\"; bind \"lead\" \"bad\";",
        "bind \"lead\" \"unknown\";",
        "route \"alpha\" \"unknown\";",
    ] {
        assert!(
            load(&format!("config {{ {body} }}"), None).is_err(),
            "{body}"
        );
    }
}

#[test]
fn structural_errors_and_pending_syntax_are_explicit() {
    for text in [
        "config { command \"bad--name\" \"runner ${payload}\"; }",
        "config { command \"a\" \"runner ${payload}\"; command \"a\" \"other ${payload}\"; }",
        "config {}\nconfig {}",
        "alpha \"runner ${payload}\"\nconfig { route \"alpha\" \"lead\"; }",
        "config { bind \"lead\" \"a\"; bind \"lead\" \"b\"; }",
        "config { route \"alpha\" \"a\"; route \"alpha\" \"b\"; }",
        "config { command \"a\" \"runner ${payload}\" { param \"x\" \"y\"; }; }",
        "config { values \"a\" { param \"x\" \"y\"; }; }",
        "config { route \"alpha\" { unset \"x\"; }; }",
        "config { profile \"a\" {}; }",
        "config { select; }",
        "config { bind \"lead\" \"a\" extra=\"bad\"; }",
        "config { (typed)route \"alpha\" \"lead\"; }",
    ] {
        assert!(load(text, None).is_err(), "{text}");
    }
    assert!(load(
        BASE,
        Some("config { command \"new\" \"runner ${payload}\"; }")
    )
    .is_err());
}

#[test]
fn named_scanner_refuses_invalid_active_words_without_changing_legacy_rules() {
    for template in [
        "${payload}",
        "runner x${payload}",
        "runner ${unknown} ${payload}",
        "runner ${broken ${payload}",
        "runner ${param.x} ${payload}",
        "runner # lost ${payload}",
        "runner ${payload} ${payload}",
    ] {
        let text = format!(
            "config {{ command \"a\" {template:?}; bind \"b\" \"a\"; route \"k\" \"b\"; }}"
        );
        assert!(load(&text, None).is_err(), "{template}");
    }
    assert!(load("k \"runner $${payload} ${payload}\"", None).is_err());
}

#[test]
fn empty_command_and_route_blocks_are_parameter_free_but_bindings_have_no_block() {
    let base = "config { command \"a\" \"runner ${payload}\" {}; bind \"b\" \"a\"; route \"k\" \"b\" {}; }";
    load(base, None).unwrap().require("k").unwrap();
    assert!(load(
        &base.replace("bind \"b\" \"a\";", "bind \"b\" \"a\" {};"),
        None
    )
    .is_err());
}

#[test]
fn named_failures_report_real_sources_and_deterministic_spans() {
    let error = load(
        "config { bind \"first\" \"missing\"; route \"other\" \"absent\"; }",
        Some("config { bind \"last\" \"unknown\"; }"),
    )
    .err()
    .unwrap();
    assert_eq!(error.diagnostics().len(), 3);
    for diagnostic in error.diagnostics() {
        assert_eq!(diagnostic.category, "unknown_reference");
        let source = diagnostic.source.as_ref().unwrap();
        assert!(error.to_string().contains(source.path.to_str().unwrap()));
        let span = diagnostic.primary.as_ref().unwrap();
        assert!(span.end > span.start);
    }
    assert_eq!(error.diagnostics()[0].binding.as_deref(), Some("first"));
    assert_eq!(error.diagnostics()[1].key.as_deref(), Some("other"));
    assert_eq!(error.diagnostics()[2].binding.as_deref(), Some("last"));
}

#[test]
fn duplicate_wrapper_reports_first_declaration_as_primary() {
    let error = load("config {}\nconfig {}", None).err().unwrap();
    let duplicate = error
        .diagnostics()
        .iter()
        .find(|d| d.category == "duplicate")
        .unwrap();
    assert_eq!(duplicate.primary.as_ref().unwrap().start, 0);
    assert_eq!(duplicate.related[0].start, 10);
}

#[test]
fn local_binding_redirects_shared_users_and_named_routes_replace_flat_targets() {
    let templates = load(
        BASE,
        Some("config { bind \"lead\" \"other\"; route \"config\" \"lead\"; }"),
    )
    .unwrap();
    for key in ["alpha", "beta", "config"] {
        let argv = templates
            .expand(
                key,
                &[Slot {
                    name: "payload",
                    value: OsStr::new("value"),
                }],
            )
            .unwrap();
        assert_eq!(argv.words(), ["alternate", "value"]);
    }
    let history = templates
        .inspect()
        .histories
        .iter()
        .find(|h| {
            h.setting
                == Setting::RouteTarget {
                    key: "config".into(),
                }
        })
        .unwrap();
    assert!(matches!(
        history.assignments[0].value,
        AssignmentValue::LiteralTemplate(_)
    ));
    assert_eq!(
        history.assignments[1].value,
        AssignmentValue::Set("lead".into())
    );
}

#[cfg(unix)]
#[test]
fn named_commands_preserve_native_runtime_values_and_refuse_nul() {
    use std::os::unix::ffi::OsStrExt;
    let templates = load(BASE, None).unwrap();
    let native = OsStr::from_bytes(b"native\xff path");
    let argv = templates
        .expand(
            "alpha",
            &[Slot {
                name: "payload",
                value: native,
            }],
        )
        .unwrap();
    assert_eq!(argv.args().last().unwrap(), native);
    let invalid = templates
        .expand(
            "alpha",
            &[Slot {
                name: "payload",
                value: OsStr::from_bytes(b"bad\0value"),
            }],
        )
        .err()
        .unwrap();
    assert_eq!(invalid.diagnostics()[0].category, "invalid_value");
    let invalid = load(
        "config { command \"a\" \"run\\u{0} ${payload}\"; bind \"b\" \"a\"; }",
        None,
    )
    .err()
    .unwrap();
    assert_eq!(invalid.diagnostics()[0].category, "invalid_template");
}

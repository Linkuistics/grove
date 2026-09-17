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
config {
    command "base" "base ${payload}"
    bind "base" "base"
    route "config" "base"
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
        assert_eq!(view.binding, "lead");
        assert_eq!(view.command, "shared");
        assert_eq!(view.origins.len(), 3);
        assert_eq!(templates.source(key), Some(path.as_path()));
    }
}

#[test]
fn local_targets_replace_before_validation_and_preserve_histories() {
    let primary = r#"config {
        command "good" "runner ${payload}"
        command "alternate" "alternate ${payload}"
        bind "lead" "missing"
        route "alpha" "missing-binding"
        route "beta" "lead"
    }"#;
    let overlay = r#"config {
        bind "alternate" "alternate"
        route "beta" "alternate"
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
    assert_eq!(beta.binding, "alternate");
    assert_eq!(beta.command, "alternate");
    let alpha = view.commands.iter().find(|c| c.key == "alpha").unwrap();
    assert_eq!(alpha.command, "good");
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
fn structural_and_active_reference_errors_are_explicit() {
    for text in [
        "config { command \"bad--name\" \"runner ${payload}\"; }",
        "config { command \"a\" \"runner ${payload}\"; command \"a\" \"other ${payload}\"; }",
        "config {}\nconfig {}",
        "alpha \"runner ${payload}\"\nconfig { route \"alpha\" \"lead\"; }",
        "config { bind \"lead\" \"a\"; bind \"lead\" \"b\"; }",
        "config { route \"alpha\" \"a\"; route \"alpha\" \"b\"; }",
        "config { values \"a\" { param \"x\" \"y\"; }; }",
        "config { route \"alpha\" { unset \"x\"; }; }",
        "config { profile \"a\"; }",
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
fn named_scanner_refuses_invalid_active_words() {
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
fn local_binding_redirects_shared_users() {
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
        AssignmentValue::Set(_)
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
    let templates = load(
        &BASE
            .replace("runner 'one argument'", "runner ${param.native}")
            .replace(
                "$${escaped} ${payload}\"",
                "$${escaped} ${payload}\" { param \"native\" \"configuration value\"; }",
            ),
        None,
    )
    .unwrap();
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

#[test]
fn parameter_defaults_are_opaque_words_with_exact_provenance_and_snapshot_lifetime() {
    let text = r#"config {
        command "shared" "runner pre=${param.first}/${param.second}/${param.first} ${param.empty} ${param.prompt} $${param.unknown} ${payload}" {
            param "first" "space 'quote' ${payload}; #"
            param "second" "雪"
            param "empty" ""
            param "prompt" "ordinary parameter"
            param "unused" "still inspected"
        }
        bind "lead" "shared"
        route "alpha" "lead"
        route "beta" "lead"
    }"#;
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("policy.kdl");
    fs::write(&path, text).unwrap();
    let catalog = Catalog::load(&path, None, vocabulary()).unwrap();
    let convenience = Templates::load(&path, None, vocabulary()).unwrap();
    fs::write(&path, text.replace("雪", "changed")).unwrap();
    let edited = Templates::load(&path, None, vocabulary()).unwrap();
    fs::remove_file(&path).unwrap();
    let templates = catalog.resolve(&Selection::default()).unwrap();
    drop(catalog);
    assert_eq!(templates.inspect(), convenience.inspect());
    for key in ["alpha", "beta"] {
        let slots = [Slot {
            name: "payload",
            value: OsStr::new("runtime value"),
        }];
        let argv = templates.expand(key, &slots).unwrap();
        assert_eq!(
            argv.words(),
            [
                "runner",
                "pre=space 'quote' ${payload}; #/雪/space 'quote' ${payload}; #",
                "",
                "ordinary parameter",
                "${param.unknown}",
                "runtime value"
            ]
        );
        assert_eq!(
            edited.expand(key, &slots).unwrap().args()[0],
            "pre=space 'quote' ${payload}; #/changed/space 'quote' ${payload}; #"
        );
        let view = templates.inspect();
        let command = view.commands.iter().find(|c| c.key == key).unwrap();
        assert_eq!(command.parameters.len(), 5);
        for parameter in &command.parameters {
            assert_eq!(parameter.origins.len(), 1);
            assert_eq!(parameter.histories.len(), 1);
            let history = &view.histories[parameter.histories[0]];
            assert_eq!(
                history.setting,
                Setting::ParameterDefault {
                    command: "shared".into(),
                    parameter: parameter.name.clone()
                }
            );
            assert_eq!(
                history.assignments[0].value,
                AssignmentValue::Set(parameter.value.clone())
            );
            let span = &view.origins[parameter.origins[0]].span;
            assert!(text[span.start..span.end].starts_with(&format!("param {:?}", parameter.name)));
            assert!(command.histories.contains(&history.id));
        }
        let mixed = &command.words[1];
        assert_eq!(
            mixed.origins.len(),
            3,
            "template and both parameters, deduplicated"
        );
        for name in ["first", "second"] {
            let parameter = command.parameters.iter().find(|p| p.name == name).unwrap();
            assert!(mixed.origins.contains(&parameter.origins[0]));
        }
        assert_eq!(
            command.words[4].origins.len(),
            1,
            "escaped reference is literal"
        );
        assert_eq!(
            command.words[5].origins.len(),
            1,
            "runtime slot has template origin"
        );
    }
}

#[test]
fn required_parameters_are_demanded_only_on_admitted_routes_even_when_unused() {
    let base = r#"config {
        command "shared" "runner ${payload}" { param "required"; }
        command "plain" "plain ${payload}"
        bind "lead" "shared"
    }"#;
    load(base, Some("config { route \"local-only\" \"lead\"; }")).unwrap();
    let text = base.replace(
        "bind \"lead\" \"shared\"",
        "bind \"lead\" \"shared\"; route \"alpha\" \"lead\"; route \"beta\" \"lead\"",
    );
    let error = load(&text, None).err().unwrap();
    assert_eq!(error.diagnostics().len(), 2);
    for (diagnostic, key) in error.diagnostics().iter().zip(["alpha", "beta"]) {
        assert_eq!(diagnostic.category, "missing_parameter");
        assert_eq!(diagnostic.key.as_deref(), Some(key));
        assert_eq!(diagnostic.parameter.as_deref(), Some("required"));
        assert_eq!(diagnostic.command.as_deref(), Some("shared"));
        assert_eq!(diagnostic.binding.as_deref(), Some("lead"));
        let span = diagnostic.primary.as_ref().unwrap();
        assert_eq!(&text[span.start..span.end], "param \"required\"");
        assert!(diagnostic
            .related
            .iter()
            .any(|span| text[span.start..span.end].starts_with("route")));
    }
    load(
        &text,
        Some("config { bind \"plain\" \"plain\"; route \"alpha\" \"plain\"; route \"beta\" \"plain\"; }"),
    )
    .unwrap();
}

#[test]
fn parameter_schemas_are_checked_even_on_dormant_definitions() {
    for (declarations, category) in [
        ("param \"a\"; param \"a\" \"b\";", "duplicate"),
        ("param \"Bad\";", "shape"),
        ("param \"bad--name\";", "shape"),
        ("param \"a\" 1;", "shape"),
        ("param \"a\" \"b\" \"c\";", "shape"),
        ("param \"a\" {};", "shape"),
        ("param name=\"a\";", "shape"),
        ("(typed)param \"a\";", "shape"),
        ("param (typed)\"a\";", "shape"),
        ("unset \"a\";", "shape"),
        ("param;", "shape"),
    ] {
        let text = format!("config {{ command \"dormant\" \"'broken\" {{ {declarations} }}; }}");
        let error = load(&text, None).err().unwrap();
        assert_eq!(error.diagnostics()[0].category, category, "{text}");
    }
}

#[test]
fn active_parameter_errors_are_classified_without_cascading_missing_values() {
    for (template, declaration, category, parameter) in [
        (
            "runner ${param.unknown} ${payload}",
            "param \"required\";",
            "unknown_parameter",
            Some("unknown"),
        ),
        (
            "runner ${param.broken ${payload}",
            "param \"required\";",
            "invalid_template",
            None,
        ),
        (
            "${param.executable} ${payload}",
            "param \"executable\" \"runner\";",
            "invalid_template",
            None,
        ),
        (
            "run${param.executable} ${payload}",
            "param \"executable\" \"ner\";",
            "invalid_template",
            None,
        ),
        (
            "runner ${payload}",
            "param \"unused\" \"bad\\u{0}value\";",
            "invalid_value",
            Some("unused"),
        ),
    ] {
        let text = format!("config {{ command \"a\" {template:?} {{ {declaration} }}; bind \"b\" \"a\"; route \"key\" \"b\"; }}");
        let error = load(&text, None).err().unwrap();
        assert_eq!(error.diagnostics().len(), 1, "{text}: {error}");
        let diagnostic = &error.diagnostics()[0];
        assert_eq!(diagnostic.category, category, "{text}: {error}");
        assert_eq!(diagnostic.parameter.as_deref(), parameter);
    }
    load("config { command \"dormant\" \"${param.unknown}\" { param \"unused\" \"bad\\u{0}value\"; }; }", None).unwrap();
    let error = load(
        "config { command \"a\" \"runner ${param.unknown} ${payload}\"; bind \"b\" \"a\"; }",
        None,
    )
    .err()
    .unwrap();
    assert_eq!(error.diagnostics()[0].category, "unknown_parameter");
}

#[test]
fn changing_binding_schema_uses_final_defaults_and_parameter_free_routes_have_none() {
    let primary = r#"config {
        command "old" "runner ${param.old} ${payload}" { param "old" "first"; }
        command "new" "runner ${param.new} ${payload}" { param "new" "second"; }
        command "plain" "plain ${payload}"
        bind "lead" "old"
        route "alpha" "lead"
        route "beta" "lead"
    }"#;
    let templates = load(
        primary,
        Some(
            "config { bind \"lead\" \"new\"; bind \"plain\" \"plain\"; route \"beta\" \"plain\"; }",
        ),
    )
    .unwrap();
    let view = templates.inspect();
    let alpha = &view.commands[0];
    assert_eq!(alpha.parameters.len(), 1);
    assert_eq!(alpha.parameters[0].name, "new");
    assert_eq!(alpha.parameters[0].value, "second");
    assert!(view.commands[1].parameters.is_empty());
    assert_eq!(
        templates
            .expand(
                "alpha",
                &[Slot {
                    name: "payload",
                    value: OsStr::new("value")
                }]
            )
            .unwrap()
            .words(),
        ["runner", "second", "value"]
    );
    assert_eq!(
        view.histories
            .iter()
            .find(|h| h.setting
                == Setting::BindingTarget {
                    binding: "lead".into()
                })
            .unwrap()
            .assignments
            .len(),
        2
    );
}

const SHARED_VALUES: &str = r#"config {
    command "shared" "runner mode=${param.mode}/${param.mode} ${param.empty} ${payload}" {
        param "mode" "default"
        param "empty" "fallback"
        param "unused"
    }
    values "shared" { param "mode" "primary"; param "empty" ""; }
    bind "lead" "shared"
    route "alpha" "lead"
    route "beta" "lead"
}"#;

#[test]
fn shared_values_fold_before_instantiation_and_survive_source_removal() {
    let overlay = r#"config { values "shared" {
        param "mode" "space 'quotes' ${payload}; #"
        param "unused" "completed locally"
    }; }"#;
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("primary.kdl");
    let local = dir.path().join("local.kdl");
    fs::write(&primary, SHARED_VALUES).unwrap();
    fs::write(&local, overlay).unwrap();
    let catalog = Catalog::load(&primary, Some(&local), vocabulary()).unwrap();
    let convenience = Templates::load(&primary, Some(&local), vocabulary()).unwrap();
    fs::remove_file(&primary).unwrap();
    fs::remove_file(&local).unwrap();
    let templates = catalog.resolve(&Selection::default()).unwrap();
    drop(catalog);
    assert_eq!(templates.inspect(), convenience.inspect());
    for key in ["alpha", "beta"] {
        let argv = templates
            .expand(
                key,
                &[Slot {
                    name: "payload",
                    value: OsStr::new("runtime"),
                }],
            )
            .unwrap();
        assert_eq!(
            argv.words(),
            [
                "runner",
                "mode=space 'quotes' ${payload}; #/space 'quotes' ${payload}; #",
                "",
                "runtime"
            ]
        );
        let view = templates.inspect();
        let command = view.commands.iter().find(|c| c.key == key).unwrap();
        let parameter = command
            .parameters
            .iter()
            .find(|p| p.name == "mode")
            .unwrap();
        assert_eq!(
            parameter.origins.len(),
            2,
            "declaration and winning shared assignment"
        );
        assert_eq!(
            command.words[1].origins.len(),
            3,
            "template plus both contributors, without repeats"
        );
        let spans: Vec<_> = parameter
            .origins
            .iter()
            .map(|id| &view.origins[*id].span)
            .collect();
        assert_eq!(
            &SHARED_VALUES[spans[0].start..spans[0].end],
            "param \"mode\" \"default\""
        );
        assert_eq!(spans[1].source.path, local);
        assert_eq!(
            &overlay[spans[1].start..spans[1].end],
            "param \"mode\" \"space 'quotes' ${payload}; #\""
        );
        let history = view
            .histories
            .iter()
            .find(|h| {
                h.setting
                    == Setting::CommandParameter {
                        command: "shared".into(),
                        parameter: "mode".into(),
                    }
            })
            .unwrap();
        assert_eq!(
            history
                .assignments
                .iter()
                .map(|a| a.value.clone())
                .collect::<Vec<_>>(),
            [
                AssignmentValue::Set("primary".into()),
                AssignmentValue::Set("space 'quotes' ${payload}; #".into())
            ]
        );
        assert!(parameter.histories.contains(&history.id));
        assert!(command.histories.contains(&history.id));
    }
}

#[test]
fn shared_unset_exposes_defaults_and_retains_absent_removals() {
    let primary = SHARED_VALUES.replace("param \"unused\"", "param \"unused\" \"default\"");
    let templates = load(
        &primary,
        Some(
            r#"config { values "shared" {
        unset "mode"
        unset "absent"
    }; }"#,
        ),
    )
    .unwrap();
    for key in ["alpha", "beta"] {
        assert_eq!(
            templates
                .expand(
                    key,
                    &[Slot {
                        name: "payload",
                        value: OsStr::new("runtime")
                    }]
                )
                .unwrap()
                .words(),
            ["runner", "mode=default/default", "", "runtime"]
        );
    }
    let view = templates.inspect();
    let mode = view.commands[0]
        .parameters
        .iter()
        .find(|p| p.name == "mode")
        .unwrap();
    assert_eq!(mode.origins.len(), 1);
    let history = &view.histories[*mode.histories.last().unwrap()];
    assert_eq!(
        history.assignments.last().unwrap().value,
        AssignmentValue::Unset
    );
    assert!(view.histories.iter().any(|h| h.setting
        == Setting::CommandParameter {
            command: "shared".into(),
            parameter: "absent".into()
        }
        && h.assignments[0].value == AssignmentValue::Unset));
}

#[test]
fn shared_values_validate_final_assignments_without_activating_templates() {
    let primary = r#"config {
        command "dormant" "'broken" { param "x"; }
        values "dormant" { param "old" "wrong schema"; param "x" "bad\u{0}value"; }
    }"#;
    load(
        primary,
        Some(r#"config { values "dormant" { unset "old"; param "x" "fixed"; }; }"#),
    )
    .unwrap();
    for (body, category, parameter) in [
        (r#"values "missing" {}"#, "unknown_reference", None),
        (
            r#"values "dormant" { param "unknown" "value"; }"#,
            "unknown_parameter",
            Some("unknown"),
        ),
        (
            r#"values "dormant" { param "x" "bad\u{0}value"; }"#,
            "invalid_value",
            Some("x"),
        ),
    ] {
        let text =
            format!("config {{ command \"dormant\" \"'broken\" {{ param \"x\"; }}; {body}; }}");
        let error = load(&text, None).err().unwrap();
        assert_eq!(error.diagnostics().len(), 1, "{error}");
        let diagnostic = &error.diagnostics()[0];
        assert_eq!(diagnostic.category, category);
        assert_eq!(diagnostic.parameter.as_deref(), parameter);
        assert!(diagnostic.command.is_some());
        assert!(diagnostic.primary.is_some());
    }
    let error = load(SHARED_VALUES, None).err().unwrap();
    assert_eq!(error.diagnostics().len(), 2);
    assert!(error
        .diagnostics()
        .iter()
        .all(|d| d.category == "missing_parameter" && d.parameter.as_deref() == Some("unused")));
}

#[test]
fn shared_values_reject_duplicate_and_malformed_patches() {
    for (body, category) in [
        (r#"values "c" {}; values "c" {}"#, "duplicate"),
        (r#"values "c" { param "x" "v"; unset "x"; }"#, "duplicate"),
        (r#"values "c" { unset "x"; unset "x"; }"#, "duplicate"),
        (r#"values "c""#, "shape"),
        (r#"values "Bad" {}"#, "shape"),
        (r#"values "c" "extra" {}"#, "shape"),
        (r#"values "c" { param "x"; }"#, "shape"),
        (r#"values "c" { param "x" 1; }"#, "shape"),
        (r#"values "c" { unset "x" "v"; }"#, "shape"),
        (r#"values "c" { param "x" "v" {}; }"#, "shape"),
        (r#"values "c" { (typed)param "x" "v"; }"#, "shape"),
        (r#"values "c" { param (typed)"x" "v"; }"#, "shape"),
        (r#"values "c" { param name="x" "v"; }"#, "shape"),
    ] {
        let text = format!("config {{ {body}; }}");
        let error = load(&text, None).err().unwrap();
        assert_eq!(error.diagnostics()[0].category, category, "{text}: {error}");
    }
}

#[test]
fn route_specificity_unset_and_capture_preserve_exact_words() {
    let primary = SHARED_VALUES.replace(
        "route \"alpha\" \"lead\"",
        "route \"alpha\" \"lead\" { param \"mode\" \"exception\"; param \"unused\" \"personal\"; }",
    );
    let overlay = r#"config {
        values "shared" { param "mode" "later shared"; param "unused" "local"; }
        route "alpha" { param "empty" "space 'quotes' ${payload}; #"; unset "unused"; unset "absent"; }
    }"#;
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("p.kdl");
    let o = dir.path().join("o.kdl");
    fs::write(&p, &primary).unwrap();
    fs::write(&o, overlay).unwrap();
    let catalog = Catalog::load(&p, Some(&o), vocabulary()).unwrap();
    let convenience = Templates::load(&p, Some(&o), vocabulary()).unwrap();
    fs::remove_file(&p).unwrap();
    fs::remove_file(&o).unwrap();
    let templates = catalog.resolve(&Selection::default()).unwrap();
    drop(catalog);
    assert_eq!(templates.inspect(), convenience.inspect());
    for (key, expected) in [
        (
            "alpha",
            [
                "runner",
                "mode=exception/exception",
                "space 'quotes' ${payload}; #",
                "runtime",
            ],
        ),
        (
            "beta",
            ["runner", "mode=later shared/later shared", "", "runtime"],
        ),
    ] {
        assert_eq!(
            templates
                .expand(
                    key,
                    &[Slot {
                        name: "payload",
                        value: OsStr::new("runtime")
                    }]
                )
                .unwrap()
                .words(),
            expected
        );
    }
    let view = templates.inspect();
    let alpha = &view.commands[0];
    let mode = alpha.parameters.iter().find(|p| p.name == "mode").unwrap();
    assert_eq!(mode.histories.len(), 3);
    assert_eq!(mode.origins.len(), 2);
    assert_eq!(alpha.words[1].origins.len(), 3);
    let origin = &view.origins[mode.origins[1]].span;
    assert_eq!(
        &primary[origin.start..origin.end],
        "param \"mode\" \"exception\""
    );
    let unused = alpha
        .parameters
        .iter()
        .find(|p| p.name == "unused")
        .unwrap();
    assert_eq!(unused.value, "local");
    let absent = view
        .histories
        .iter()
        .find(|h| {
            h.setting
                == Setting::RouteParameter {
                    key: "alpha".into(),
                    parameter: "absent".into(),
                }
        })
        .unwrap();
    assert_eq!(absent.assignments[0].value, AssignmentValue::Unset);
    assert!(alpha.histories.contains(&absent.id));
}

#[test]
fn route_switches_preserve_maps_until_explicitly_unset() {
    let primary = r#"config {
        command "old" "runner ${param.old} ${payload}" { param "old" "default"; }
        command "new" "other ${param.new} ${payload}" { param "new" "default"; }
        bind "lead" "old"
        bind "next" "new"
        route "alpha" "lead" { param "old" "exception"; }
    }"#;
    for overlay in [
        r#"config { route "alpha" "next" { unset "old"; param "new" ""; }; }"#,
        r#"config { bind "lead" "new"; route "alpha" { unset "old"; param "new" ""; }; }"#,
    ] {
        let templates = load(primary, Some(overlay)).unwrap();
        assert_eq!(
            templates
                .expand(
                    "alpha",
                    &[Slot {
                        name: "payload",
                        value: OsStr::new("runtime")
                    }]
                )
                .unwrap()
                .words(),
            ["other", "", "runtime"]
        );
    }
    let error = load(primary, Some(r#"config { route "alpha" "next"; }"#))
        .err()
        .unwrap();
    assert_eq!(error.diagnostics()[0].category, "unknown_parameter");
    assert_eq!(error.diagnostics()[0].parameter.as_deref(), Some("old"));
}

#[test]
fn personal_parameter_only_routes_cannot_be_repaired_locally() {
    let primary = "config { command \"good\" \"runner ${payload}\"; bind \"lead\" \"good\"; route \"good\" \"lead\"; route \"missing\" { unset \"absent\"; }; }";
    for overlay in [None, Some("config { route \"missing\" \"lead\"; }")] {
        let error = load(primary, overlay).err().unwrap();
        assert_eq!(error.diagnostics().len(), 1);
        let diagnostic = &error.diagnostics()[0];
        assert_eq!(diagnostic.category, "missing_target");
        assert_eq!(diagnostic.key.as_deref(), Some("missing"));
        let span = diagnostic.primary.as_ref().unwrap();
        assert_eq!(span.source.role, keyed_launch::SourceRole::Primary);
        assert!(primary[span.start..span.end].starts_with("route \"missing\""));
    }
    let templates = load(
        "config { command \"good\" \"runner ${payload}\"; bind \"lead\" \"good\"; route \"good\" \"lead\"; }",
        Some(
            r#"config {
        route "only-params" { param "unknown" "bad\u{0}"; }
        route "only-target" "missing" { param "unknown" "bad\u{0}"; }
    }"#,
        ),
    )
    .unwrap();
    assert_eq!(templates.keys(), ["good"]);
    assert_eq!(templates.inspect().non_admitted_keys.len(), 2);
    for key in ["only-params", "only-target"] {
        assert!(templates.require(key).is_err());
    }
}

#[test]
fn route_patches_validate_structure_and_surviving_assignments() {
    for (body, category) in [
        (r#"route "x" { param "p" "v"; unset "p"; }"#, "duplicate"),
        (r#"route "x" { param "p"; }"#, "shape"),
        (r#"route "x" { unset "p" "v"; }"#, "shape"),
        (r#"route "x" { param "Bad" "v"; }"#, "shape"),
        (r#"route "x" { param "p" "v" {}; }"#, "shape"),
        (r#"route "x" { (typed)unset "p"; }"#, "shape"),
        (r#"route "x" { unset name="p"; }"#, "shape"),
        (r#"route "x" 42"#, "shape"),
        (r#"route "x" "b" "extra""#, "shape"),
        (r#"route "x"; route "x" "b""#, "duplicate"),
    ] {
        let error = load(&format!("config {{ {body}; }}"), None).err().unwrap();
        assert_eq!(error.diagnostics()[0].category, category, "{body}: {error}");
    }
    let primary = r#"config {
        command "c" "runner ${payload}" { param "unused"; }
        bind "b" "c"
        route "x" "b" { param "unused" "bad\u{0}"; param "old" "obsolete"; }
    }"#;
    let error = load(primary, None).err().unwrap();
    assert_eq!(
        error
            .diagnostics()
            .iter()
            .map(|d| d.category.as_str())
            .collect::<Vec<_>>(),
        ["invalid_value", "unknown_parameter"]
    );
    assert!(error
        .diagnostics()
        .iter()
        .all(|d| d.key.as_deref() == Some("x") && d.primary.is_some()));
    load(
        primary,
        Some(r#"config { route "x" { param "unused" "completed"; unset "old"; }; }"#),
    )
    .unwrap();
}

//! A consumer with no Grove paths, keys, or runtime vocabulary.
use std::ffi::{OsStr, OsString};
use std::fs;

use keyed_launch::{
    conformance, Catalog, Requirement, Selection, Slot, SlotRule, Templates, Vocabulary,
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

#[test]
fn selection_declarations_are_captured_without_choosing_policy() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("personal.kdl");
    let overlay = dir.path().join("local.kdl");
    let text = "// λ keeps byte offsets honest\nconfig { command \"base\" \"base ${payload}\"; bind \"lead\" \"base\"; route \"opaque\" \"lead\"; select \"daily\" \"daily\"; }\n";
    fs::write(&primary, text).unwrap();
    fs::write(&overlay, "config { select; }\n").unwrap();
    let catalog = Catalog::load(&primary, Some(&overlay), vocabulary()).unwrap();
    let declared = catalog.primary_selection().unwrap();
    assert_eq!(declared.profiles, ["daily", "daily"]);
    let span = declared.origin.as_ref().unwrap();
    assert_eq!(span.source.path, primary);
    assert_eq!(span.source.role, keyed_launch::SourceRole::Primary);
    assert!(text[span.start..span.end].starts_with("select \"daily\" \"daily\""));
    let empty = catalog.overlay_selection().unwrap();
    assert!(empty.profiles.is_empty());
    assert_eq!(empty.origin.as_ref().unwrap().source.path, overlay);
    assert_eq!(
        empty.origin.as_ref().unwrap().source.role,
        keyed_launch::SourceRole::Overlay
    );
    let convenience = Templates::load(&primary, Some(&overlay), vocabulary()).unwrap();
    assert_eq!(convenience.inspect().selection, Selection::default());
    fs::remove_file(&primary).unwrap();
    fs::remove_file(&overlay).unwrap();
    let snapshot = catalog.resolve(empty).unwrap();
    assert_eq!(snapshot.inspect().selection, *empty);
    assert_eq!(snapshot.keys(), ["opaque"]);
    assert!(conformance::check(&catalog, empty).passed());
    let error = catalog.resolve(declared).err().unwrap();
    assert_eq!(error.diagnostics().len(), 2);
    for (index, diagnostic) in error.diagnostics().iter().enumerate() {
        assert_eq!(diagnostic.category, "unknown_profile");
        assert_eq!(diagnostic.primary.as_ref(), Some(span));
        assert_eq!(diagnostic.occurrence_chain[0].selection_index, index);
        assert_eq!(diagnostic.occurrence_chain[0].via.as_ref(), Some(span));
    }
    assert!(!conformance::check(&catalog, declared).passed());
    let values = [Slot {
        name: "payload",
        value: OsStr::new("one word"),
    }];
    assert_eq!(
        snapshot.expand("opaque", &values).unwrap().words(),
        convenience.expand("opaque", &values).unwrap().words()
    );
}

#[test]
fn selection_shapes_are_checked_in_both_sources_even_when_not_used() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("personal.kdl");
    let overlay = dir.path().join("local.kdl");
    for declaration in [
        "select 1",
        "select true",
        "select \"Bad\"",
        "select \"\"",
        "select \"trailing-\"",
        "select \"two--dashes\"",
        "select name=\"daily\"",
        "(typed)select \"daily\"",
        "select (typed)\"daily\"",
        "select {}",
        "select; select",
    ] {
        for local in [false, true] {
            fs::write(&primary, "config { command \"base\" \"base ${payload}\"; bind \"lead\" \"base\"; route \"opaque\" \"lead\"; }\n").unwrap();
            fs::write(&overlay, "").unwrap();
            fs::write(
                if local { &overlay } else { &primary },
                format!("config {{ {declaration}; }}\n"),
            )
            .unwrap();
            let error = Catalog::load(&primary, Some(&overlay), vocabulary())
                .err()
                .expect(declaration);
            assert!(
                error
                    .diagnostics()
                    .iter()
                    .all(|d| d.category == "shape" || d.category == "duplicate"),
                "{error}"
            );
        }
    }
}

#[test]
fn snapshots_and_conformance_use_captured_inputs_after_sources_disappear() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("personal.kdl");
    let overlay = dir.path().join("local.kdl");
    fs::write(
        &primary,
        "config {\n command \"original\" \"original ${payload}\"\n command \"base\" \"base ${payload}\"\n command \"replacement\" \"replacement 'one argument' ${payload}\"\n bind \"zeta\" \"original\"\n bind \"opaque\" \"base\"\n route \"zeta\" \"zeta\"\n route \"opaque\" \"opaque\"\n}\n",
    )
    .unwrap();
    fs::write(
        &overlay,
        "config { bind \"opaque\" \"replacement\"; route \"opaque\" \"opaque\"; route \"local\" \"opaque\"; }\n",
    )
    .unwrap();
    let catalog = Catalog::load(&primary, Some(&overlay), vocabulary()).unwrap();
    let convenience = Templates::load(&primary, Some(&overlay), vocabulary()).unwrap();
    assert!(catalog.primary_selection().is_none());
    assert!(catalog.overlay_selection().is_none());
    fs::write(&primary, "broken {").unwrap();
    fs::remove_file(&overlay).unwrap();
    let snapshot = catalog.resolve(&Selection::default()).unwrap();
    assert!(conformance::check(&catalog, &Selection::default()).passed());
    fs::remove_file(&primary).unwrap();
    drop(catalog);
    assert_eq!(snapshot.keys(), ["opaque", "zeta"]);
    assert_eq!(snapshot.source("opaque"), Some(primary.as_path()));
    assert_eq!(snapshot.source("zeta"), Some(primary.as_path()));
    assert!(snapshot.require("local").is_err());
    assert!(snapshot.source("local").is_none());
    let values = [Slot {
        name: "payload",
        value: OsStr::new("space ; ${literal}"),
    }];
    let expected = ["replacement", "one argument", "space ; ${literal}"]
        .map(OsString::from)
        .to_vec();
    assert_eq!(
        snapshot.expand("opaque", &values).unwrap().words(),
        expected
    );
    assert_eq!(
        convenience.expand("opaque", &values).unwrap().words(),
        expected
    );
    assert_eq!(
        snapshot.expand("zeta", &values).unwrap().words(),
        ["original", "space ; ${literal}"].map(OsString::from)
    );
}

#[test]
fn explicit_unknown_selection_refuses_in_resolution_and_conformance() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("config.kdl");
    fs::write(&primary, "config { command \"base\" \"run ${payload}\"; bind \"lead\" \"base\"; route \"opaque\" \"lead\"; }\n").unwrap();
    let catalog = Catalog::load(&primary, None, vocabulary()).unwrap();
    let selection = Selection {
        profiles: vec!["experiment".into()],
        origin: None,
    };
    let error = catalog.resolve(&selection).err().unwrap().to_string();
    assert!(error.contains("unknown profile `experiment`"), "{error}");
    let outcome = conformance::check(&catalog, &selection);
    assert!(!outcome.passed());
    assert_eq!(outcome.failures, [error]);
}

#[test]
fn overlay_only_commands_cannot_make_conformance_nonvacuous() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("empty.kdl");
    let overlay = dir.path().join("overlay.kdl");
    fs::write(&primary, "// no admitted keys\nconfig { command \"base\" \"run ${payload}\"; bind \"lead\" \"base\"; }\n").unwrap();
    fs::write(&overlay, "config { route \"opaque\" \"lead\"; }\n").unwrap();
    let catalog = Catalog::load(&primary, Some(&overlay), vocabulary()).unwrap();
    let outcome = conformance::check(&catalog, &Selection::default());
    assert!(!outcome.passed());
    assert!(outcome.failures[0].contains("declares no keys"));
}

#[test]
fn both_loaders_reject_reserved_vocabulary_before_reading_sources() {
    let dir = TempDir::new().unwrap();
    let absent = dir.path().join("absent.kdl");
    for name in ["param.", "param.effort"] {
        let slots = [SlotRule {
            name,
            requirement: Requirement::AtMostOnce,
        }];
        let catalog_error = Catalog::load(&absent, None, Vocabulary { slots: &slots })
            .err()
            .unwrap()
            .to_string();
        let convenience_error = Templates::load(&absent, None, Vocabulary { slots: &slots })
            .err()
            .unwrap()
            .to_string();
        assert!(catalog_error.contains("reserved"), "{catalog_error}");
        assert!(catalog_error.contains(name), "{catalog_error}");
        assert_eq!(catalog_error, convenience_error);
    }
}

#[test]
fn catalog_refuses_unsupported_top_level_shapes() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("config.kdl");
    let overlay = dir.path().join("overlay.kdl");
    fs::write(&primary, "opaque \"run ${payload}\"\n").unwrap();
    fs::write(&overlay, "local \"missing-required-slot\"\n").unwrap();
    assert!(Catalog::load(&primary, Some(&overlay), vocabulary()).is_err());
    for text in [
        "profile \"daily\" {\n}\n",
        "base {\n}\n",
        "select \"one\" \"two\"\n",
    ] {
        fs::write(&primary, text).unwrap();
        assert!(
            Catalog::load(&primary, None, vocabulary()).is_err(),
            "{text}"
        );
    }
}

#[cfg(unix)]
#[test]
fn captured_expansion_preserves_native_runtime_bytes() {
    use std::os::unix::ffi::OsStringExt;
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("config.kdl");
    fs::write(&primary, "config { command \"base\" \"run ${payload}\"; bind \"lead\" \"base\"; route \"opaque\" \"lead\"; }\n").unwrap();
    let snapshot = Catalog::load(&primary, None, vocabulary())
        .unwrap()
        .resolve(&Selection::default())
        .unwrap();
    let native = OsString::from_vec(vec![b'a', 0xff, b' ', b'b']);
    let argv = snapshot
        .expand(
            "opaque",
            &[Slot {
                name: "payload",
                value: &native,
            }],
        )
        .unwrap();
    assert_eq!(argv.args(), [native]);
}

#[test]
fn inactive_profiles_preserve_base_commands_and_do_not_authorize_local_keys() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("personal.kdl");
    let overlay = dir.path().join("local.kdl");
    let text = r#"config {
    command "base" "base ${payload}"
    bind "lead" "base"
    route "opaque" "lead"
    command "unused" "'unfinished" { param "required"; }
    profile "experiment" {
        include "missing" "experiment" "missing"
        route "local-only" "absent" { param "unknown" "value"; }
        values "missing" { param "unknown" "value"; }
        bind "other" "missing"
    }
    profile "empty" {}
    select "experiment"
}
"#;
    fs::write(&primary, text).unwrap();
    fs::write(&overlay, "config { route \"local-only\" \"lead\"; }\n").unwrap();
    let catalog = Catalog::load(&primary, Some(&overlay), vocabulary()).unwrap();
    let convenience = Templates::load(&primary, Some(&overlay), vocabulary()).unwrap();
    fs::remove_file(&primary).unwrap();
    fs::remove_file(&overlay).unwrap();
    let snapshot = catalog.resolve(&Selection::default()).unwrap();
    assert_eq!(snapshot.inspect(), convenience.inspect());
    assert_eq!(snapshot.keys(), ["opaque"]);
    assert!(snapshot.require("local-only").is_err());
    assert!(snapshot.inspect().profile_occurrences.is_empty());
    assert!(conformance::check(&catalog, &Selection::default()).passed());
    assert_eq!(
        snapshot
            .expand(
                "opaque",
                &[Slot {
                    name: "payload",
                    value: OsStr::new("one word")
                }]
            )
            .unwrap()
            .words(),
        &[OsString::from("base"), OsString::from("one word")]
    );
    let selected = catalog.primary_selection().unwrap();
    let error = catalog.resolve(selected).err().unwrap();
    let diagnostic = &error.diagnostics()[0];
    assert_eq!(diagnostic.category, "unknown_profile");
    assert_eq!(diagnostic.occurrence_chain[0].via, selected.origin);
    let span = diagnostic.primary.as_ref().unwrap();
    assert!(text[span.start..span.end].starts_with("include \"missing\""));
    assert!(error
        .diagnostics()
        .iter()
        .any(|d| d.category == "include_cycle"));
    assert!(!conformance::check(&catalog, selected).passed());
}

#[test]
fn inactive_profile_structure_is_checked_without_merging_patch_namespaces() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("personal.kdl");
    let overlay = dir.path().join("local.kdl");
    for (body, category) in [
        ("profile \"empty\" {}", None),
        ("profile \"one\" { include; route \"x\" \"lead\"; }; profile \"two\" { include \"one\" \"one\"; route \"x\" \"other\"; }", None),
        ("profile \"Bad\" {}", Some("shape")),
        ("profile \"one\"", Some("shape")),
        ("profile \"one\" \"extra\" {}", Some("shape")),
        ("profile name=\"one\" {}", Some("shape")),
        ("(typed)profile \"one\" {}", Some("shape")),
        ("profile (typed)\"one\" {}", Some("shape")),
        ("profile \"one\" {}; profile \"one\" {}", Some("duplicate")),
        ("profile \"one\" { include; include; }", Some("duplicate")),
        ("profile \"one\" { include 1; }", Some("shape")),
        ("profile \"one\" { include \"Bad\"; }", Some("shape")),
        ("profile \"one\" { include {}; }", Some("shape")),
        ("profile \"one\" { command \"x\" \"run\"; }", Some("shape")),
        ("profile \"one\" { select; }", Some("shape")),
        ("profile \"one\" { profile \"two\" {}; }", Some("shape")),
        ("profile \"one\" { opaque \"run\"; }", Some("shape")),
        ("profile \"one\" { bind \"lead\" \"a\"; bind \"lead\" \"b\"; }", Some("duplicate")),
        ("profile \"one\" { route \"x\" \"lead\"; route \"x\"; }", Some("duplicate")),
        ("profile \"one\" { route \"x\" { param \"p\" \"a\"; unset \"p\"; }; }", Some("duplicate")),
        ("profile \"one\" { values \"a\" {}; values \"a\" {}; }", Some("duplicate")),
        ("profile \"one\" { values \"a\" { param \"p\"; }; }", Some("shape")),
    ] {
        fs::write(&primary, format!("config {{ command \"base\" \"base ${{payload}}\"; bind \"lead\" \"base\"; route \"opaque\" \"lead\"; {body}; }}\n")).unwrap();
        let result = Catalog::load(&primary, None, vocabulary());
        if let Some(category) = category {
            let error = result.err().expect(body);
            assert!(error.diagnostics().iter().any(|d| d.category == category), "{body}: {error}");
            assert!(error.diagnostics().iter().all(|d| d.source.as_ref().unwrap().path == primary));
        } else {
            assert_eq!(result.unwrap().resolve(&Selection::default()).unwrap().keys(), ["opaque"]);
        }
    }
    fs::write(&primary, "config { command \"base\" \"base ${payload}\"; bind \"lead\" \"base\"; route \"opaque\" \"lead\"; }\n").unwrap();
    fs::write(&overlay, "config { profile \"local\" {}; }\n").unwrap();
    let error = Catalog::load(&primary, Some(&overlay), vocabulary())
        .err()
        .unwrap();
    assert_eq!(error.diagnostics()[0].category, "shape");
    assert_eq!(
        error.diagnostics()[0].source.as_ref().unwrap().path,
        overlay
    );
}

#[test]
fn both_loaders_reject_flat_and_mixed_input_in_either_source() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("personal.kdl");
    let overlay = dir.path().join("local.kdl");
    for key in [
        "opaque", "config", "command", "profile", "select", "route", "bind", "values", "param",
        "unset", "include",
    ] {
        for mixed in [0, 1, 2] {
            for local in [false, true] {
                fs::write(&primary, "config {}\n").unwrap();
                fs::write(&overlay, "").unwrap();
                let path = if local { &overlay } else { &primary };
                let flat = format!("{key} \"run ${{payload}}\"\n");
                let text = match mixed {
                    0 => flat.clone(),
                    1 => format!("{flat}config {{}}\n"),
                    _ => format!("config {{}}\n{flat}"),
                };
                fs::write(path, &text).unwrap();
                let catalog = Catalog::load(&primary, Some(&overlay), vocabulary())
                    .err()
                    .expect(&text);
                let convenience = Templates::load(&primary, Some(&overlay), vocabulary())
                    .err()
                    .expect(&text);
                assert_eq!(catalog.diagnostics(), convenience.diagnostics());
                let diagnostic = &catalog.diagnostics()[0];
                assert_eq!(diagnostic.category, "shape");
                let span = diagnostic.primary.as_ref().unwrap();
                assert_eq!(&span.source.path, path);
                assert_eq!(&text[span.start..span.end], flat.trim_end());
                assert_eq!(
                    span.source.role,
                    if local {
                        keyed_launch::SourceRole::Overlay
                    } else {
                        keyed_launch::SourceRole::Primary
                    }
                );
                assert!(span.end > span.start);
                assert!(diagnostic.remedy.contains("config"), "{catalog}");
                assert!(diagnostic.remedy.contains("route"), "{catalog}");
                assert_eq!(fs::read_to_string(path).unwrap(), text);
            }
        }
    }
}

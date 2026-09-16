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
    let text = "// λ keeps byte offsets honest\nopaque \"base ${payload}\"\nconfig { select \"daily\" \"daily\"; }\n";
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
            fs::write(&primary, "opaque \"base ${payload}\"\n").unwrap();
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
    fs::write(
        &primary,
        "select \"base ${payload}\"\nprofile \"other ${payload}\"\n",
    )
    .unwrap();
    let catalog = Catalog::load(&primary, None, vocabulary()).unwrap();
    assert!(catalog.primary_selection().is_none());
    assert_eq!(
        catalog.resolve(&Selection::default()).unwrap().keys(),
        ["profile", "select"]
    );
}

#[test]
fn snapshots_and_conformance_use_captured_inputs_after_sources_disappear() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("personal.kdl");
    let overlay = dir.path().join("local.kdl");
    fs::write(
        &primary,
        "zeta \"original ${payload}\"\nopaque \"base ${payload}\"\n",
    )
    .unwrap();
    fs::write(
        &overlay,
        "opaque \"replacement 'one argument' ${payload}\"\nlocal \"extra ${payload}\"\n",
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
    assert_eq!(snapshot.source("opaque"), Some(overlay.as_path()));
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
    fs::write(&primary, "opaque \"run ${payload}\"\n").unwrap();
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
    fs::write(&primary, "// no admitted keys\n").unwrap();
    fs::write(&overlay, "opaque \"run ${payload}\"\n").unwrap();
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
fn flat_catalog_keeps_eager_validation_and_refuses_wrapper_shapes() {
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
    fs::write(&primary, "opaque \"run ${payload}\"\n").unwrap();
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

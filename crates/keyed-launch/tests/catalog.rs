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

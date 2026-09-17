use std::ffi::OsStr;
use std::fs;

use keyed_launch::{conformance, Catalog, ConfigError, Selection, Slot, SourceRole, Vocabulary};
use tempfile::TempDir;

fn load_error(primary: &str, overlay: Option<&str>) -> (TempDir, ConfigError) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("primary.kdl");
    let local = dir.path().join("overlay.kdl");
    fs::write(&path, primary).unwrap();
    if let Some(text) = overlay {
        fs::write(&local, text).unwrap();
    }
    let error = Catalog::load(
        &path,
        overlay.map(|_| local.as_path()),
        Vocabulary { slots: &[] },
    )
    .err()
    .unwrap();
    (dir, error)
}

#[test]
fn structural_reports_aggregate_in_source_order_with_real_utf8_ranges() {
    let primary =
        "// é\nconfig {\nroute \"wrong\" 42\nroute \"dup\" \"run\"\nroute \"dup\" \"again\"\n}\n";
    let overlay = "config { route \"local\" \"run\" extra=1; }\n";
    let (dir, error) = load_error(primary, Some(overlay));
    let reports = error.diagnostics();
    assert_eq!(
        reports
            .iter()
            .map(|d| d.category.as_str())
            .collect::<Vec<_>>(),
        ["shape", "duplicate", "shape"]
    );
    for (report, text, name, role, declaration) in [
        (
            &reports[0],
            primary,
            "primary.kdl",
            SourceRole::Primary,
            "route \"wrong\" 42",
        ),
        (
            &reports[1],
            primary,
            "primary.kdl",
            SourceRole::Primary,
            "route \"dup\" \"run\"",
        ),
        (
            &reports[2],
            overlay,
            "overlay.kdl",
            SourceRole::Overlay,
            "route \"local\" \"run\" extra=1",
        ),
    ] {
        let span = report.primary.as_ref().unwrap();
        assert_eq!(span.source.path, dir.path().join(name));
        assert_eq!(span.source.role, role);
        assert_eq!(report.source.as_ref(), Some(&span.source));
        assert_eq!(&text[span.start..span.end], declaration);
        assert!(!report.remedy.is_empty());
        assert!(report.occurrence_chain.is_empty());
    }
    let duplicate = &reports[1];
    assert_eq!(duplicate.related.len(), 1);
    let related = &duplicate.related[0];
    assert_eq!(
        &primary[related.start..related.end],
        "route \"dup\" \"again\""
    );
    assert_eq!(
        reports[0].primary.as_ref().unwrap().start,
        "// é\nconfig {\n".len()
    );
}

#[test]
fn structural_errors_suppress_template_cascades_across_documents() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("primary.kdl");
    let overlay = dir.path().join("overlay.kdl");
    let document = r#"config {
    command "bad" "runner ${unknown}" extra=1
    command "other" "'unclosed"
    bind "bad" "bad"
}"#;
    fs::write(&primary, document).unwrap();
    fs::write(&overlay, "config { bind \"other\" \"other\"; }\n").unwrap();
    let load =
        || keyed_launch::Templates::load(&primary, Some(&overlay), Vocabulary { slots: &[] });
    let error = load().err().unwrap();
    assert_eq!(error.diagnostics().len(), 1);
    assert_eq!(error.diagnostics()[0].category, "shape");

    // The same active commands really do fail semantically once structure passes.
    fs::write(&primary, document.replace(" extra=1", "")).unwrap();
    let error = load().err().unwrap();
    assert_eq!(error.diagnostics().len(), 2);
    for (diagnostic, command) in error.diagnostics().iter().zip(["bad", "other"]) {
        assert_eq!(diagnostic.category, "invalid_template");
        assert_eq!(diagnostic.command.as_deref(), Some(command));
    }
}

#[test]
fn read_and_syntax_reports_keep_available_locations() {
    let dir = TempDir::new().unwrap();
    let absent = dir.path().join("absent.kdl");
    let error = Catalog::load(&absent, None, Vocabulary { slots: &[] })
        .err()
        .unwrap();
    let report = &error.diagnostics()[0];
    assert_eq!(report.category, "source_read");
    assert_eq!(report.source.as_ref().unwrap().path, absent);
    assert!(report.primary.is_none());
    assert!(!report.remedy.is_empty());
    let text = "// é\nbroken {";
    let (_, error) = load_error(text, Some("config { route \"wrong\" 42; }\n"));
    assert_eq!(error.diagnostics()[0].category, "kdl_syntax");
    let span = error.diagnostics()[0].primary.as_ref().unwrap();
    assert!(text.get(span.start..span.end).is_some());
    assert_eq!(error.diagnostics()[1].category, "shape");
}

#[test]
fn caller_errors_have_categories_and_context_without_fabricated_spans() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("primary.kdl");
    let overlay = dir.path().join("overlay.kdl");
    fs::write(&primary, "config { command \"run\" \"run\"; command \"other\" \"other\"; bind \"lead\" \"run\"; route \"opaque\" \"lead\"; }\n").unwrap();
    fs::write(&overlay, "config { bind \"lead\" \"other\"; route \"opaque\" \"lead\"; route \"local\" \"lead\"; }\n").unwrap();
    let catalog = Catalog::load(&primary, Some(&overlay), Vocabulary { slots: &[] }).unwrap();
    let selection = Selection {
        profiles: vec!["absent".into()],
        origin: None,
    };
    let error = catalog.resolve(&selection).err().unwrap();
    assert_eq!(error.diagnostics()[0].category, "unknown_profile");
    assert!(error.diagnostics()[0].primary.is_none());
    assert!(error.diagnostics()[0].source.is_none());
    assert!(error.diagnostics()[0].message.contains("absent"));
    assert_eq!(
        conformance::check(&catalog, &selection).failures,
        [error.to_string()]
    );
    let snapshot = catalog.resolve(&Selection::default()).unwrap();
    fs::remove_file(&primary).unwrap();
    fs::remove_file(&overlay).unwrap();
    drop(catalog);
    for key in ["local", "missing"] {
        let error = snapshot.require(key).unwrap_err();
        let report = &error.diagnostics()[0];
        assert_eq!(report.category, "unconfigured_key");
        assert_eq!(report.key.as_deref(), Some(key));
        assert_eq!(report.source.as_ref().unwrap().path, primary);
        assert!(report.primary.is_none());
        assert!(report.remedy.contains(key));
    }
    let error = snapshot
        .expand(
            "opaque",
            &[Slot {
                name: "unknown",
                value: OsStr::new("x"),
            }],
        )
        .unwrap_err();
    let report = &error.diagnostics()[0];
    assert_eq!(report.category, "invalid_value");
    assert_eq!(report.key.as_deref(), Some("opaque"));
    assert_eq!(report.source.as_ref().unwrap().path, primary);
    assert!(report.primary.is_none());
    assert!(!report.remedy.is_empty());
}

#[test]
fn vocabulary_and_missing_or_duplicate_runtime_values_have_no_source_spans() {
    use keyed_launch::{Requirement, SlotRule};
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("primary.kdl");
    fs::write(&path, "config { command \"run\" \"run ${payload}\"; bind \"lead\" \"run\"; route \"opaque\" \"lead\"; }\n").unwrap();
    for names in [["param.effort", "other"], ["same", "same"]] {
        let slots = names.map(|name| SlotRule {
            name,
            requirement: Requirement::AtMostOnce,
        });
        let error = Catalog::load(&path, None, Vocabulary { slots: &slots })
            .err()
            .unwrap();
        assert_eq!(error.diagnostics()[0].category, "invalid_value");
        assert!(error.diagnostics()[0].primary.is_none());
        assert!(error.diagnostics()[0].source.is_none());
        assert!(error.diagnostics()[0].message.contains(names[0]));
        assert!(!error.diagnostics()[0].remedy.is_empty());
    }
    let slots = [SlotRule {
        name: "payload",
        requirement: Requirement::ExactlyOnce,
    }];
    let snapshot = Catalog::load(&path, None, Vocabulary { slots: &slots })
        .unwrap()
        .resolve(&Selection::default())
        .unwrap();
    let duplicate = [
        Slot {
            name: "payload",
            value: OsStr::new("x"),
        },
        Slot {
            name: "payload",
            value: OsStr::new("y"),
        },
    ];
    for values in [&[][..], &duplicate[..]] {
        let error = snapshot.expand("opaque", values).unwrap_err();
        let report = &error.diagnostics()[0];
        assert_eq!(report.category, "invalid_value");
        assert_eq!(report.key.as_deref(), Some("opaque"));
        assert!(report.message.contains("payload"));
        assert!(report.primary.is_none());
        assert!(!report.remedy.is_empty());
    }
}

#[test]
fn unknown_selections_report_each_occurrence_without_inventing_a_span() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("primary.kdl");
    fs::write(&path, "config { command \"run\" \"run\"; command \"other\" \"other\"; bind \"lead\" \"run\"; route \"opaque\" \"lead\"; }\n").unwrap();
    let catalog = Catalog::load(&path, None, Vocabulary { slots: &[] }).unwrap();
    let selection = Selection {
        profiles: vec!["missing".into(), "other".into(), "missing".into()],
        origin: None,
    };
    let error = catalog.resolve(&selection).err().unwrap();
    assert_eq!(error.diagnostics().len(), 3);
    for (index, diagnostic) in error.diagnostics().iter().enumerate() {
        let occurrence = &diagnostic.occurrence_chain[0];
        assert_eq!(occurrence.selection_index, index);
        assert_eq!(occurrence.id, index);
        assert_eq!(occurrence.profile, selection.profiles[index]);
        assert!(occurrence.parent.is_none());
        assert!(occurrence.via.is_none());
        assert!(diagnostic.primary.is_none());
        assert!(diagnostic.source.is_none());
    }
}

#[test]
fn non_admitted_key_retains_its_real_overlay_location_after_capture() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("primary.kdl");
    let overlay = dir.path().join("overlay.kdl");
    let text = "// é\nconfig { route \"local\" \"lead\"; }\n";
    fs::write(
        &path,
        "config { command \"run\" \"run\"; bind \"lead\" \"run\"; route \"known\" \"lead\"; }\n",
    )
    .unwrap();
    fs::write(&overlay, text).unwrap();
    let snapshot = Catalog::load(&path, Some(&overlay), Vocabulary { slots: &[] })
        .unwrap()
        .resolve(&Selection::default())
        .unwrap();
    fs::remove_file(&overlay).unwrap();
    let error = snapshot.require("local").unwrap_err();
    let diagnostic = &error.diagnostics()[0];
    assert_eq!(diagnostic.source.as_ref().unwrap().path, path);
    assert_eq!(diagnostic.related.len(), 1);
    let span = &diagnostic.related[0];
    assert_eq!(span.source.path, overlay);
    assert_eq!(span.source.role, SourceRole::Overlay);
    assert_eq!(&text[span.start..span.end], "route \"local\" \"lead\"");
    assert!(diagnostic.remedy.contains(&path.display().to_string()));
}

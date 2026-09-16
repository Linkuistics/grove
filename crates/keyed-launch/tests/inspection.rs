//! Inspect a captured configuration using only the generic public interface.
use std::ffi::{OsStr, OsString};
use std::fs;

use keyed_launch::{
    AssignmentValue, Catalog, CompiledWord, Inspection, Requirement, Selection, Setting, Slot,
    SlotRule, SourceRole, Templates, Vocabulary,
};
use tempfile::TempDir;

fn vocabulary() -> Vocabulary<'static> {
    Vocabulary {
        slots: &[
            SlotRule {
                name: "payload",
                requirement: Requirement::ExactlyOnce,
            },
            SlotRule {
                name: "optional",
                requirement: Requirement::AtMostOnce,
            },
        ],
    }
}

fn fill(view: &Inspection, key: &str, payload: &OsStr) -> Vec<OsString> {
    view.commands
        .iter()
        .find(|command| command.key == key)
        .unwrap()
        .words
        .iter()
        .map(|word| match &word.word {
            CompiledWord::Literal(value) => OsString::from(value),
            CompiledWord::Slot(name) => {
                assert_eq!(name, "payload");
                payload.to_owned()
            }
        })
        .collect()
}

#[test]
fn captured_flat_inspection_retains_history_and_matches_expansion() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("personal.kdl");
    let overlay = dir.path().join("local.kdl");
    // Deliberately differ from alphabetical order and use multibyte source text.
    let primary_text = "// café\nzeta \"original ${payload}\"\nopaque \"base ${payload}\"\n";
    let overlay_text =
        "opaque \"replacement 'one argument' '' ${payload}\"\nlocal \"extra ${payload}\"\n";
    fs::write(&primary, primary_text).unwrap();
    fs::write(&overlay, overlay_text).unwrap();
    let catalog = Catalog::load(&primary, Some(&overlay), vocabulary()).unwrap();
    let convenience = Templates::load(&primary, Some(&overlay), vocabulary()).unwrap();
    fs::write(&primary, "broken {").unwrap();
    fs::remove_file(&overlay).unwrap();
    let snapshot = catalog.resolve(&Selection::default()).unwrap();
    fs::remove_file(&primary).unwrap();
    drop(catalog);
    let view = snapshot.inspect();
    assert_eq!(view, convenience.inspect());
    assert_eq!(view.selection, Selection::default());
    assert!(view.profile_occurrences.is_empty());
    assert_eq!(view.sources.len(), 2);
    assert_eq!(view.sources[0].role, SourceRole::Primary);
    assert_eq!(view.sources[0].path, primary);
    assert_eq!(view.sources[1].role, SourceRole::Overlay);
    assert_eq!(view.sources[1].path, overlay);
    assert_eq!(
        view.commands
            .iter()
            .map(|c| c.key.as_str())
            .collect::<Vec<_>>(),
        ["opaque", "zeta"]
    );
    assert_eq!(view.origins.len(), 4);
    let declarations = [
        "zeta \"original ${payload}\"",
        "opaque \"base ${payload}\"",
        "opaque \"replacement 'one argument' '' ${payload}\"",
        "local \"extra ${payload}\"",
    ];
    for (id, origin) in view.origins.iter().enumerate() {
        assert_eq!(origin.id, id);
        assert!(origin.occurrence.is_none());
        let source = match origin.span.source.role {
            SourceRole::Primary => primary_text,
            SourceRole::Overlay => overlay_text,
        };
        assert_eq!(
            &source[origin.span.start..origin.span.end],
            declarations[id]
        );
        assert!(view.sources.contains(&origin.span.source));
    }
    assert_eq!(view.histories.len(), 3);
    let expected = [
        ("local", vec![(3, "extra ${payload}")]),
        (
            "opaque",
            vec![
                (1, "base ${payload}"),
                (2, "replacement 'one argument' '' ${payload}"),
            ],
        ),
        ("zeta", vec![(0, "original ${payload}")]),
    ];
    for (id, (history, (key, assignments))) in view.histories.iter().zip(expected).enumerate() {
        assert_eq!(history.id, id);
        assert_eq!(history.setting, Setting::RouteTarget { key: key.into() });
        assert_eq!(history.assignments.len(), assignments.len());
        for (assignment, (order, text)) in history.assignments.iter().zip(assignments) {
            assert_eq!(assignment.order, order);
            assert_eq!(assignment.origin, order);
            assert_eq!(
                assignment.value,
                AssignmentValue::LiteralTemplate(text.into())
            );
        }
    }
    for (command, origin, history) in [(&view.commands[0], 2, 1), (&view.commands[1], 0, 2)] {
        assert!(command.binding.is_none());
        assert!(command.command.is_none());
        assert!(command.parameters.is_empty());
        assert_eq!(command.origins, [origin]);
        assert_eq!(command.histories, [history]);
        for word in &command.words {
            assert_eq!(word.origins, [origin]);
        }
    }
    assert_eq!(view.non_admitted_keys.len(), 1);
    assert_eq!(view.non_admitted_keys[0].key, "local");
    assert_eq!(view.non_admitted_keys[0].origins, [3]);
    assert!(!view.non_admitted_keys[0].reason.is_empty());
    assert!(snapshot.require("local").is_err());
    let payload = OsStr::new("space ; ${literal}");
    for key in ["opaque", "zeta"] {
        let expanded = snapshot
            .expand(
                key,
                &[
                    Slot {
                        name: "payload",
                        value: payload,
                    },
                    Slot {
                        name: "optional",
                        value: OsStr::new("unused"),
                    },
                ],
            )
            .unwrap();
        assert_eq!(fill(view, key, payload), expanded.words());
    }
    assert_eq!(
        fill(view, "opaque", payload),
        ["replacement", "one argument", "", "space ; ${literal}"].map(OsString::from)
    );
}

#[cfg(unix)]
#[test]
fn inspection_keeps_native_paths_and_symbolic_native_values() {
    use std::os::unix::ffi::OsStringExt;
    let dir = TempDir::new().unwrap();
    // macOS filesystems reject invalid UTF-8 filenames at creation time.
    #[cfg(target_os = "linux")]
    let filename = OsString::from_vec(vec![b'p', 0xff]);
    #[cfg(not(target_os = "linux"))]
    let filename = OsString::from("personal café.kdl");
    let primary = dir.path().join(filename);
    fs::write(&primary, "opaque \"run ${payload}\"\n").unwrap();
    let snapshot = Templates::load(&primary, None, vocabulary()).unwrap();
    fs::remove_file(&primary).unwrap();
    let native = OsString::from_vec(vec![b'a', 0xfe, b' ', b'b']);
    let view = snapshot.inspect();
    assert_eq!(view.sources.len(), 1);
    assert_eq!(view.sources[0].path, primary);
    assert_eq!(view.origins[0].span.source.path, primary);
    assert_eq!(
        view.commands[0].words[1].word,
        CompiledWord::Slot("payload".into())
    );
    let argv = snapshot
        .expand(
            "opaque",
            &[
                Slot {
                    name: "payload",
                    value: &native,
                },
                Slot {
                    name: "optional",
                    value: OsStr::new(""),
                },
            ],
        )
        .unwrap();
    assert_eq!(fill(view, "opaque", &native), argv.words());
    assert_eq!(argv.args(), [native]);
}

#[test]
fn empty_documents_have_sources_but_no_invented_activity() {
    let dir = TempDir::new().unwrap();
    let primary = dir.path().join("empty.kdl");
    fs::write(&primary, "// empty\n").unwrap();
    let snapshot = Templates::load(&primary, None, vocabulary()).unwrap();
    let view = snapshot.inspect();
    assert_eq!(view.sources.len(), 1);
    assert!(view.commands.is_empty());
    assert!(view.origins.is_empty());
    assert!(view.histories.is_empty());
    assert!(view.non_admitted_keys.is_empty());
}

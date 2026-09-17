//! Inspect a captured configuration using only the generic public interface.
use std::ffi::{OsStr, OsString};
use std::fs;

use keyed_launch::{CompiledWord, Inspection, Requirement, Slot, SlotRule, Templates, Vocabulary};
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
    fs::write(&primary, "config {\n    command \"opaque\" \"run ${payload}\"\n    bind \"opaque\" \"opaque\"\n    route \"opaque\" \"opaque\"\n}\n").unwrap();
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

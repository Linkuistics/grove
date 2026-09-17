//! Versioned wire projection of captured records; this module never resolves policy.
use std::path::Path;

use keyed_launch::{
    AssignmentValue, CompiledWord, Diagnostic, Inspection, Occurrence, Setting, Source, SourceRole,
    SourceSpan,
};
use serde_json::{json, Value};

fn path(path: &Path) -> Value {
    if let Some(text) = path.to_str() {
        return json!(text);
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        json!({"encoding": "unix_bytes", "value": path.as_os_str().as_bytes()})
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        json!({"encoding": "windows_wide", "value": path.as_os_str().encode_wide().collect::<Vec<_>>()})
    }
}

fn source(source: &Source) -> Value {
    let role = match source.role {
        SourceRole::Primary => "primary",
        SourceRole::Overlay => "overlay",
    };
    json!({"role": role, "path": path(&source.path)})
}

fn span(span: &SourceSpan) -> Value {
    json!({"source": source(&span.source), "start": span.start, "end": span.end})
}

fn occurrence(occurrence: &Occurrence) -> Value {
    json!({"id": occurrence.id, "profile": occurrence.profile,
        "parent": occurrence.parent, "selection_index": occurrence.selection_index,
        "via": occurrence.via.as_ref().map(span)})
}

fn setting(setting: &Setting) -> Value {
    match setting {
        Setting::BindingTarget { binding } => json!({"type": "binding_target", "binding": binding}),
        Setting::RouteTarget { key } => json!({"type": "route_target", "key": key}),
        Setting::ParameterDefault { command, parameter } => {
            json!({"type": "parameter_default", "command": command, "parameter": parameter})
        }
        Setting::CommandParameter { command, parameter } => {
            json!({"type": "command_parameter", "command": command, "parameter": parameter})
        }
        Setting::RouteParameter { key, parameter } => {
            json!({"type": "route_parameter", "key": key, "parameter": parameter})
        }
    }
}

fn assignment(value: &AssignmentValue) -> Value {
    match value {
        AssignmentValue::Set(value) => json!({"type": "set", "value": value}),
        AssignmentValue::Unset => json!({"type": "unset"}),
    }
}

/// Only commands are filtered: all response-local references retain their targets.
pub fn inspection(view: &Inspection, kind: Option<&str>) -> Value {
    // json! only receives JSON values and infallible primitive/container types.
    // https://docs.rs/serde_json/1.0.150/serde_json/macro.json.html
    json!({
        "schema_version": 1,
        "sources": view.sources.iter().map(source).collect::<Vec<_>>(),
        "selection": {"profiles": view.selection.profiles, "origin": view.selection.origin.as_ref().map(span)},
        "profile_occurrences": view.profile_occurrences.iter().map(occurrence).collect::<Vec<_>>(),
        "commands": view.commands.iter().filter(|c| kind.is_none_or(|kind| kind == c.key)).map(|c| json!({
            "key": c.key, "binding": c.binding, "command": c.command,
            "parameters": c.parameters.iter().map(|p| json!({"name": p.name, "value": p.value, "origins": p.origins, "histories": p.histories})).collect::<Vec<_>>(),
            "words": c.words.iter().map(|w| {
                let word = match &w.word {
                    CompiledWord::Literal(value) => json!({"type": "literal", "value": value}),
                    CompiledWord::Slot(name) => json!({"type": "slot", "name": name}),
                };
                json!({"word": word, "origins": w.origins})
            }).collect::<Vec<_>>(),
            "origins": c.origins, "histories": c.histories,
        })).collect::<Vec<_>>(),
        "non_admitted_keys": view.non_admitted_keys.iter().map(|k| json!({"key": k.key, "origins": k.origins, "reason": k.reason})).collect::<Vec<_>>(),
        "origins": view.origins.iter().map(|o| json!({"id": o.id, "span": span(&o.span), "occurrence": o.occurrence})).collect::<Vec<_>>(),
        "histories": view.histories.iter().map(|h| json!({
            "id": h.id, "setting": setting(&h.setting),
            "assignments": h.assignments.iter().map(|a| json!({"order": a.order, "value": assignment(&a.value), "origin": a.origin})).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

fn diagnostic(d: &Diagnostic) -> Value {
    json!({"category": d.category, "message": d.message,
        "source": d.source.as_ref().map(source), "primary": d.primary.as_ref().map(span),
        "related": d.related.iter().map(span).collect::<Vec<_>>(),
        "occurrence_chain": d.occurrence_chain.iter().map(occurrence).collect::<Vec<_>>(),
        "key": d.key, "binding": d.binding, "command": d.command,
        "parameter": d.parameter, "remedy": d.remedy})
}

/// Failures outside configuration still use the same diagnostic record shape.
pub fn failure(category: &str, message: &str, remedy: &str) -> Value {
    json!({"schema_version": 1, "diagnostics": [{
        "category": category, "message": message, "source": null, "primary": null,
        "related": [], "occurrence_chain": [], "key": null, "binding": null,
        "command": null, "parameter": null, "remedy": remedy,
    }]})
}

pub fn error(error: &anyhow::Error) -> Value {
    if let Some(error) = error.downcast_ref::<grove_loop::Error>() {
        if !error.diagnostics().is_empty() {
            return json!({"schema_version": 1, "diagnostics": error.diagnostics().iter().map(diagnostic).collect::<Vec<_>>()});
        }
    }
    failure(
        "inspection",
        &format!("{error:#}"),
        "Check the reported workspace or I/O failure and retry grove config show.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_paths_in_captured_sources_and_spans_are_lossless() {
        #[cfg(unix)]
        let (native, expected) = {
            use std::os::unix::ffi::OsStringExt;
            (
                std::ffi::OsString::from_vec(vec![b'/', 255]),
                json!({"encoding": "unix_bytes", "value": [47, 255]}),
            )
        };
        #[cfg(windows)]
        let (native, expected) = {
            use std::os::windows::ffi::OsStringExt;
            (
                std::ffi::OsString::from_wide(&[67, 58, 92, 0xd800]),
                json!({"encoding": "windows_wide", "value": [67, 58, 92, 55296]}),
            )
        };
        let source_record = Source {
            role: SourceRole::Primary,
            path: native.into(),
        };
        let span_record = SourceSpan {
            source: source_record.clone(),
            start: 0,
            end: 5,
        };
        assert_eq!(source(&source_record)["path"], expected);
        assert_eq!(span(&span_record)["source"]["path"], expected);
        assert_eq!(path(Path::new("unicode-λ")), "unicode-λ");
    }
}

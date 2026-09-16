# What the call reaches
<!-- book-page id="what-the-call-reaches" slice="assembly" order="5" -->
[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

<a id="assembly"></a>
## Observation and lifecycle have separate lifetimes

<!-- rollup «owned-lines-total» -->
The book reconstructs 636 source lines. The preceding chapters explain parsing,
dispatch and their tests. This chapter
connects the public library calls and owns the configuration report formatter,
which turns validated records into human text or versioned JSON.

Bare `grove` resolves a jj workspace, takes the driver lease and calls
`grove_loop::run`. `grove view [WORKTREE]` returns through `grove_tui::run`
before those steps. An observation path needs no workspace identity, driver
lease or launch configuration. The configuration inspector also returns before leasing, but loads and validates
active policy. All paths return errors through the same main;
the viewer restores its terminal before returning an error.

<a id="the-package-map"></a>
## Packages and public boundaries

The workspace contains eight packages. Seven share the product's release
version; `book-validation` is the separately versioned authoring tool.

| Package | Responsibility | Workspace runtime dependencies |
|---|---|---|
| `grove` | Human CLI, dispatch and configuration presentation | `grove-loop`, `grove-tui`, `keyed-launch` |
| `grove-tui` | Read-only application, capture, rendering and terminal lifetime | `grove-loop` |
| `grove-loop` | Grove vocabulary, reader, verbs and driver | `ordinal-fs-tree`, `jj-workspace`, `keyed-launch` |
| `grove-llm` | Session CLI | `grove-loop`, `jj-workspace` |
| `ordinal-fs-tree` | Domain-free ordered tree store | none |
| `jj-workspace` | Domain-free workspace and commit seam | none |
| `keyed-launch` | Domain-free command runner | none |
| `book-validation` | Source-fragment and Markdown checks | none |

Ratatui and Crossterm belong to the viewer. Neither the loop library nor the
session binary depends on the terminal UI. Release archives still contain only
`grove` and `grove-llm`: the viewer is linked into the human executable.

<a id="the-seven-names"></a>
## What crosses the binary boundary

| Public item | Role in dispatch |
|---|---|
| `grove_loop::VERSION` | Shared release metadata |
| `grove_loop::Workspace` | Resolve the bare lifecycle's jj working tree |
| `grove_loop::DriverLease` | Hold the one-driver claim |
| `grove_loop::TemplateSource` | Locate launch policy |
| `grove_loop::run` | Execute the lifecycle |
| `grove_loop::LoopOutcome` | Distinguish clean stops and interruption |
| `grove_loop::reraise` | Preserve an interrupted driver's signal exit |
| `grove_tui::run` | Own the interactive observation session |
| `grove_loop::SessionConfig` | Load, admit and inspect the same configuration used by launch |
| `keyed_launch::Inspection` | Captured words and provenance formatted without re-resolution |

All are public library items. The package boundary makes accidental access to
private items a compiler error; a source inclusion from outside the package
would be a visible change to that boundary, not something privacy forbids.

<a id="the-modules-behind-the-call"></a>
## Behind each call

The loop walkthrough at `docs/walkthroughs/grove-loop/README.md` owns its task-name grammar,
reader and mutation semantics, lease, epoch, prompt and repeated session launch.
The viewer consumes that typed reader rather than parsing filenames itself.
It copies display rows and selected file bytes while holding a short shared
read guard, then drops it before rendering or waiting for input.

The viewer's `Viewer::new`, `act`, `tick` and `render` methods are its application seam.
Production keyboard input supplies actions; tests supply the same actions and
render through Ratatui's TestBackend. Selection opens a leaf file or branch
brief. Automatic refresh follows permanent keys across moves and retirement;
folded branches retain aggregate counts for all descendants. Missing and failed
reads become visible states and retry automatically.

<a id="configuration-report"></a>
## Worked example: inspect without starting the lifecycle

From `/work/atlas/src`, `grove config show --kind impl` resolves `/work/atlas`
and loads the personal file plus its admitted local delta, if any. With the
flat example from Orientation, the report shows literal executable `claude`,
literal argument `--add-dir`, then slots `repo` and `prompt`. It does not need
the rate-limit leaf to exist, and it does not compose a mandate. A malformed
active design route still fails this request before output. Adding `--json`
preserves the same words with explicit literal/slot tags and complete provenance
tables; both formatters consume the same captured inspection.

<!-- fragment «configuration-report» owner="assembly" source="crates/grove/src/config.rs" lines="1-165" parent="source-configuration-report" -->
<!-- insert «inspection-load» -->
<!-- insert «inspection-labels» -->
<!-- insert «inspection-selection» -->
<!-- insert «inspection-words» -->
<!-- insert «inspection-histories» -->
<!-- /fragment -->

<a id="inspection-load"></a>
## Loading before output

`show` receives a current directory and optional kind. It resolves the enclosing
workspace before asking SessionConfig to load, so a subdirectory cannot bypass
the worktree delta. SessionConfig performs the same source admission and global
resolution as launch; only then does `require` check a requested kind. The first
stdout write follows both checks. A failure therefore reports an error without
a partial configuration report. This path constructs no lease, epoch or tree.

<!-- fragment «inspection-load» owner="assembly" source="crates/grove/src/config.rs" lines="1-26" parent="configuration-report" -->
````rust
//! Human presentation of the same captured configuration the driver launches.
use std::io::{self, Write};
use std::path::Path;

use grove_loop::{SessionConfig, Workspace};
use keyed_launch::{AssignmentValue, CompiledWord, Inspection, Setting, SourceSpan};

/// Load everything before filtering or writing a report. No lease, epoch or
/// task-tree operation belongs on this path; only source admission may ask jj.
pub fn show(cwd: &Path, kind: Option<&str>, json: bool) -> anyhow::Result<()> {
    let workspace = Workspace::resolve(cwd)?;
    let config = SessionConfig::load_for_worktree(workspace.root())?;
    if let Some(kind) = kind {
        config.require(kind)?;
    }
    if json {
        writeln!(
            io::stdout().lock(),
            "{}",
            crate::config_json::inspection(config.inspect(), kind)
        )?;
    } else {
        write_human(&mut io::stdout().lock(), config.inspect(), kind)?;
    }
    Ok(())
}
````
<!-- /fragment -->

<a id="inspection-labels"></a>
## Locating assignments

The human formatter uses native path debug formatting and byte spans to locate
captured declarations. `setting_name` distinguishes defaults, shared command
values and per-kind overrides, which have different specificity. These labels
explain a history without recomputing its winner. In the running example, the
flat impl template has a kind-target history and no parameter records.

<!-- fragment «inspection-labels» owner="assembly" source="crates/grove/src/config.rs" lines="27-46" parent="configuration-report" -->
````rust

fn location(span: &SourceSpan) -> String {
    format!("{:?} bytes {}..{}", span.source.path, span.start, span.end)
}

fn setting_name(setting: &Setting) -> String {
    match setting {
        Setting::BindingTarget { binding } => format!("binding {binding:?} target"),
        Setting::RouteTarget { key } => format!("kind {key:?} target"),
        Setting::ParameterDefault { command, parameter } => {
            format!("command {command:?} parameter {parameter:?} default")
        }
        Setting::CommandParameter { command, parameter } => {
            format!("command {command:?} parameter {parameter:?} shared value")
        }
        Setting::RouteParameter { key, parameter } => {
            format!("kind {key:?} parameter {parameter:?} override")
        }
    }
}
````
<!-- /fragment -->

<a id="inspection-selection"></a>
## Explaining one selection

`write_human` borrows the Inspection and a writer. Sources and the selection
appear before commands because they explain which policy participated. An absent
selection origin is explicitly implicit; an empty declared selection still names
its span. Occurrence IDs, parents and selection indices preserve repeated
includes rather than collapsing them into a set. Writing failures propagate.

<!-- fragment «inspection-selection» owner="assembly" source="crates/grove/src/config.rs" lines="47-83" parent="configuration-report" -->
````rust

/// Quoted literals escape control characters and remain distinct from runtime
/// slots, including when a configured parameter literally contains `${prompt}`.
/// IDs point to the complete provenance tables retained below the command list.
fn write_human(out: &mut impl Write, view: &Inspection, kind: Option<&str>) -> io::Result<()> {
    writeln!(out, "Sources:")?;
    for source in &view.sources {
        writeln!(out, "  {:?}: {:?}", source.role, source.path)?;
    }
    writeln!(
        out,
        "Selection (left to right): {:?}",
        view.selection.profiles
    )?;
    writeln!(
        out,
        "  from {}",
        view.selection.origin.as_ref().map_or_else(
            || "implicit empty selection (no declaration)".to_owned(),
            location
        )
    )?;
    writeln!(out, "Profile occurrences:")?;
    for occurrence in &view.profile_occurrences {
        writeln!(
            out,
            "  #{} {:?}: selection index {}, parent {:?}, via {}",
            occurrence.id,
            occurrence.profile,
            occurrence.selection_index,
            occurrence.parent,
            occurrence
                .via
                .as_ref()
                .map_or_else(|| "none".to_owned(), location)
        )?;
    }
````
<!-- /fragment -->

<a id="inspection-words"></a>
## Keeping words separate from slots

Filtering changes only the displayed command rows after full validation.
Each row carries its binding/command chain and contributing IDs. Parameters name
their winning origins, and each word retains its index and origin IDs. Word zero
is the executable; later words are arguments. Quoted literals escape controls
and remain distinct from a `slot <prompt>` placeholder, even when a parameter's
literal text is `${prompt}`. The renderer never reconstructs a shell command.

<!-- fragment «inspection-words» owner="assembly" source="crates/grove/src/config.rs" lines="84-126" parent="configuration-report" -->
````rust
    for command in &view.commands {
        if kind.is_some_and(|kind| kind != command.key) {
            continue;
        }
        writeln!(out, "Kind {:?}:", command.key)?;
        match (&command.binding, &command.command) {
            (Some(binding), Some(name)) => {
                writeln!(out, "  reference: binding {binding:?} -> command {name:?}")?;
            }
            _ => writeln!(out, "  reference: literal template")?,
        }
        writeln!(
            out,
            "  origins {:?}; histories {:?}",
            command.origins, command.histories
        )?;
        for parameter in &command.parameters {
            writeln!(
                out,
                "  Parameter {:?} = {:?}; winning origins {:?}; histories {:?}",
                parameter.name, parameter.value, parameter.origins, parameter.histories
            )?;
        }
        for (index, word) in command.words.iter().enumerate() {
            let role = if index == 0 { "executable" } else { "argument" };
            match &word.word {
                CompiledWord::Literal(value) => {
                    writeln!(
                        out,
                        "  word {index} ({role}): literal {value:?}; origins {:?}",
                        word.origins
                    )?;
                }
                CompiledWord::Slot(name) => {
                    writeln!(
                        out,
                        "  word {index} ({role}): slot <{name}>; origins {:?}",
                        word.origins
                    )?;
                }
            }
        }
    }
````
<!-- /fragment -->

<a id="inspection-histories"></a>
## Following provenance back to source

The final tables retain all non-admitted keys, origins and histories, including
when one command is requested. This keeps every printed ID resolvable. Each
assignment retains fold order and distinguishes set, literal template, unset
and reset. A winner is explained by the command/parameter origins above, not by
assuming the chronologically last assignment across scopes wins. The closing
sentence bounds the report to this load: launch reloads, and runtime values
remain symbolic here. The process tests compare configuration, tree, signal and
Grove coordination bytes before and after inspection, allowing jj metadata.

<!-- fragment «inspection-histories» owner="assembly" source="crates/grove/src/config.rs" lines="127-165" parent="configuration-report" -->
````rust
    writeln!(out, "Non-admitted keys:")?;
    for key in &view.non_admitted_keys {
        writeln!(
            out,
            "  {:?}: {}; origins {:?}",
            key.key, key.reason, key.origins
        )?;
    }
    writeln!(out, "Origins (IDs local to this report):")?;
    for origin in &view.origins {
        writeln!(
            out,
            "  #{}: {}; profile occurrence {:?}",
            origin.id,
            location(&origin.span),
            origin.occurrence
        )?;
    }
    writeln!(
        out,
        "Histories (assignments in fold order; unset exposes inheritance):"
    )?;
    for history in &view.histories {
        writeln!(out, "  #{}: {}", history.id, setting_name(&history.setting))?;
        for assignment in &history.assignments {
            write!(out, "    order {}: ", assignment.order)?;
            match &assignment.value {
                AssignmentValue::Set(value) => write!(out, "set {value:?}")?,
                AssignmentValue::LiteralTemplate(value) => {
                    write!(out, "literal template {value:?}")?
                }
                AssignmentValue::Unset => write!(out, "unset")?,
                AssignmentValue::Reset => write!(out, "reset")?,
            }
            writeln!(out, "; origin #{}", assignment.origin)?;
        }
    }
    writeln!(out, "Runtime slots are placeholders. Launch reloads sources; equal argv requires unchanged inputs and runtime context.")
}
````
<!-- /fragment -->

<a id="the-boundary"></a>
## Where the overview stops

This book reconstructs the human crate's manifest and Rust source, including
its parser tests. Viewer and loop internals are outside its corpus. The
[usage guide](../../USAGE.md#usage-viewing-tree) owns the delivered interaction
contract; `docs/ARCHITECTURE.md` names the seams.
The viewer renders Markdown and observes changes automatically, with terminal
restoration on handled exits. Source-offset reading positions survive unchanged
refreshes; mapping anchors across content edits remains a subsequent increment.
Those internals are owned by the viewer crate, not reconstructed in this book.

<a id="the-test-applied-back"></a>
## Apply the thin-entry-point test

| Guarantee | What would break it | Evidence |
|---|---|---|
| Public library access | Naming a private library item | Rust visibility checking |
| No lifecycle selector arguments; exactly `{config, view}` with `show` beneath `config` | An unlisted command or outer argument | CLI closed-set unit test |
| Every listed item has help | Missing or whitespace-only help | Recursive description test |
| Observation bypasses lifecycle setup | Trying to resolve jj or configure a driver for view | CLI refusal test and non-jj PTY smoke |
| Display actions preserve the filesystem | Any name/content/state-file write | Application before/after fixture comparison |

The help-presence test now checks a real command and argument. Its success
still says nothing about prose accuracy. Likewise, byte-exact book validation
proves reproduction and ownership, not the truth of explanatory claims.

<a id="the-closed-ledgers"></a>
## Source ownership and early uses

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
<!-- rollup «ownership-blocks-not-owned-by» of="compiler-held" -->
There are 7 ownership blocks over 5 source roots; 6 blocks belong to chapters
after Orientation. Each is resolved in the source index.

<!-- rollup «early-use-rows» -->
The 5 early-use rows name loop items before their full explanation in Three
steps. The grammar explains its own path and command types at first use.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
Owned source is 58 + 100 + 54 + 92 + 332 = 636 lines across 5 source-owning chapters.
Assembly owns the configuration formatter. The ledgers are maintained with source changes;
production files remain authoritative.

<a id="final-verification"></a>
## Verification

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . --book docs/walkthroughs/overview --final --check all
```

This final check requires complete source coverage, fragment reachability,
byte equality, ownership ledgers and valid Markdown references. Runtime checks
are separate: the human binary's CLI tests, viewer application tests and actual
terminal smoke. The task's verification record states their observed results.

<!-- rollup «source-roots» -->
<!-- rollup «owned-lines-total» -->
<!-- rollup «chapters» -->
The corpus contains 5 roots and 636 lines, explained across 5 chapters and two
lookup pages. No deferred source range belongs in the final book.



<a id="configuration-json"></a>
## The JSON projection

The wire module consumes the same records as the human formatter. Its root
assembles location encoding, tagged values, inspection and errors; none of these
functions loads configuration or participates in driver setup.

<!-- fragment «configuration-json» owner="assembly" source="crates/grove/src/config_json.rs" lines="1-167" parent="source-configuration-json" -->
<!-- insert «json-native-paths» -->
<!-- insert «json-locations» -->
<!-- insert «json-assignment-tags» -->
<!-- insert «json-inspection» -->
<!-- insert «json-diagnostics» -->
<!-- insert «json-native-tests» -->
<!-- /fragment -->

<a id="json-native-paths"></a>
### Preserving native source paths

The JSON projection belongs to the binary. Unicode paths become strings; other paths preserve native byte or UTF-16 units in tagged arrays. This conversion is shared by success and error records, so source identity never depends on lossy display prose.

<!-- fragment «json-native-paths» owner="assembly" source="crates/grove/src/config_json.rs" lines="1-25" parent="configuration-json" -->
````rust
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

````
<!-- /fragment -->

<a id="json-locations"></a>
### Sources, spans and occurrences

The source role, captured byte range and include occurrence become explicit records. Optional parent and via values remain null. These records explain which selection applied a declaration without reopening its file.

<!-- fragment «json-locations» owner="assembly" source="crates/grove/src/config_json.rs" lines="26-43" parent="configuration-json" -->
````rust
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

````
<!-- /fragment -->

<a id="json-assignment-tags"></a>
### Keeping assignment kinds distinct

The formatter exhaustively matches setting scopes and assignment variants. A binding target and a legacy template retain different tags even when their text is identical; unset and reset have no invented value.

<!-- fragment «json-assignment-tags» owner="assembly" source="crates/grove/src/config_json.rs" lines="44-70" parent="configuration-json" -->
````rust
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
        AssignmentValue::LiteralTemplate(value) => {
            json!({"type": "literal_template", "value": value})
        }
        AssignmentValue::Unset => json!({"type": "unset"}),
        AssignmentValue::Reset => json!({"type": "reset"}),
    }
}

````
<!-- /fragment -->

<a id="json-inspection"></a>
### Projecting one complete inspection

The projection accepts the already validated Inspection. Only the command array is filtered; origin, history and occurrence tables remain complete. Compiled literals and runtime slots have distinct tags, preventing a slot-like parameter value from being reinterpreted by a consumer.

<!-- fragment «json-inspection» owner="assembly" source="crates/grove/src/config_json.rs" lines="71-100" parent="configuration-json" -->
````rust
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

````
<!-- /fragment -->

<a id="json-diagnostics"></a>
### Reporting a refusal once

Resolver diagnostics retain every available source, related span and occurrence chain. Workspace or I/O failures without such records and parser usage failures use the same shape with null locations. The CLI chooses stderr and the exit code before returning to main.

<!-- fragment «json-diagnostics» owner="assembly" source="crates/grove/src/config_json.rs" lines="101-131" parent="configuration-json" -->
````rust
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

````
<!-- /fragment -->

<a id="json-native-tests"></a>
### Checking native units without filesystem support

The test constructs native paths directly because a host filesystem may reject their names. It checks shared source/span encoding and ordinary Unicode. Process acceptance separately exercises missing native paths, argv capture, parser refusals and read-only inspection.

<!-- fragment «json-native-tests» owner="assembly" source="crates/grove/src/config_json.rs" lines="132-167" parent="configuration-json" -->
````rust
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
````
<!-- /fragment -->

[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

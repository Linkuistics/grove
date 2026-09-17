# What the call reaches
<!-- book-page id="what-the-call-reaches" slice="assembly" order="5" -->
[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

<a id="assembly"></a>
## Observation and lifecycle have separate lifetimes

<!-- rollup «owned-lines-total» -->
The book reconstructs 959 source lines. The preceding chapters explain parsing,
dispatch and their tests. This chapter
connects the public library calls and owns the configuration report formatter,
which turns validated records into human text or versioned JSON, plus the\ninactive example installer and its filesystem failure seam.

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
| `grove` | Human CLI, dispatch, configuration presentation and example delivery | `grove-loop`, `grove-tui`, `keyed-launch` |
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

<!-- fragment «configuration-report» owner="assembly" source="crates/grove/src/config.rs" lines="1-160" parent="source-configuration-report" -->
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

<!-- fragment «inspection-words» owner="assembly" source="crates/grove/src/config.rs" lines="84-125" parent="configuration-report" -->
````rust
    for command in &view.commands {
        if kind.is_some_and(|kind| kind != command.key) {
            continue;
        }
        writeln!(out, "Kind {:?}:", command.key)?;
        writeln!(
            out,
            "  reference: binding {:?} -> command {:?}",
            command.binding, command.command
        )?;
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

<!-- fragment «inspection-histories» owner="assembly" source="crates/grove/src/config.rs" lines="126-160" parent="configuration-report" -->
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
                AssignmentValue::Unset => write!(out, "unset")?,
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
There are 8 ownership blocks over 6 source roots; 7 blocks belong to chapters
after Orientation. Each is resolved in the source index.

<!-- rollup «early-use-rows» -->
The 5 early-use rows name loop items before their full explanation in Three
steps. The grammar explains its own path and command types at first use.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
Owned source is 58 + 106 + 58 + 100 + 637 = 959 lines across 5 source-owning chapters.
Assembly owns configuration presentation and example delivery. The ledgers are maintained with source changes;
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
The corpus contains 6 roots and 959 lines, explained across 5 chapters and two
lookup pages. No deferred source range belongs in the final book.



<a id="configuration-json"></a>
## The JSON projection

The wire module consumes the same records as the human formatter. Its root
assembles location encoding, tagged values, inspection and errors; none of these
functions loads configuration or participates in driver setup.

<!-- fragment «configuration-json» owner="assembly" source="crates/grove/src/config_json.rs" lines="1-163" parent="source-configuration-json" -->
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

The formatter exhaustively matches setting scopes and assignment variants. `set` carries a string value; `unset` removes a parameter override and has no invented value. Successful commands always carry binding and command names.

<!-- fragment «json-assignment-tags» owner="assembly" source="crates/grove/src/config_json.rs" lines="44-66" parent="configuration-json" -->
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
        AssignmentValue::Unset => json!({"type": "unset"}),
    }
}

````
<!-- /fragment -->

<a id="json-inspection"></a>
### Projecting one complete inspection

The projection accepts the already validated Inspection. Only the command array is filtered; origin, history and occurrence tables remain complete. Compiled literals and runtime slots have distinct tags, preventing a slot-like parameter value from being reinterpreted by a consumer.

<!-- fragment «json-inspection» owner="assembly" source="crates/grove/src/config_json.rs" lines="67-96" parent="configuration-json" -->
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

<!-- fragment «json-diagnostics» owner="assembly" source="crates/grove/src/config_json.rs" lines="97-127" parent="configuration-json" -->
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

<!-- fragment «json-native-tests» owner="assembly" source="crates/grove/src/config_json.rs" lines="128-163" parent="configuration-json" -->
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


<a id="configuration-examples"></a>
## Installing inactive examples

The example command owns filesystem delivery in the human binary. Its package,
preflight, creation and reporting form a separate path from configuration
inspection, and the following fragments reconstruct that path and its tests.

<!-- fragment «configuration-examples» owner="assembly" source="crates/grove/src/examples.rs" lines="1-314" parent="source-configuration-examples" -->
<!-- insert «examples-package» -->
<!-- insert «examples-storage» -->
<!-- insert «examples-report» -->
<!-- insert «examples-install» -->
<!-- insert «examples-run» -->
<!-- insert «examples-faults» -->
<!-- insert «examples-race-test» -->
<!-- insert «examples-write-test» -->
<!-- insert «examples-refusal-test» -->
<!-- /fragment -->

<a id="examples-package"></a>
## Embedding the inactive package

The binary owns a fixed destination-to-bytes table. `include_bytes!` reads the authoritative repository samples at compile time, including the README under its installed name. In our worked installation, no active policy is consulted and no runtime repository checkout is required.

<!-- fragment «examples-package» owner="assembly" source="crates/grove/src/examples.rs" lines="1-39" parent="configuration-examples" -->
````rust
//! Install the authoritative example bytes without reading active launch policy.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};

/// Fixed, inactive destinations; include the repository fixtures at compile time.
const EXAMPLES: &[(&str, &[u8])] = &[
    (
        "config.modular.example.kdl",
        include_bytes!("../../../docs/examples/modular-configuration/config.modular.example.kdl"),
    ),
    (
        "grove.codex-led.example.kdl",
        include_bytes!("../../../docs/examples/modular-configuration/grove.codex-led.example.kdl"),
    ),
    (
        "grove.claude-led.example.kdl",
        include_bytes!("../../../docs/examples/modular-configuration/grove.claude-led.example.kdl"),
    ),
    (
        "grove.high-effort.example.kdl",
        include_bytes!(
            "../../../docs/examples/modular-configuration/grove.high-effort.example.kdl"
        ),
    ),
    (
        "grove.local-override.example.kdl",
        include_bytes!(
            "../../../docs/examples/modular-configuration/grove.local-override.example.kdl"
        ),
    ),
    (
        "CONFIGURATION.examples.md",
        include_bytes!("../../../docs/examples/modular-configuration/README.md"),
    ),
];
````
<!-- /fragment -->

<a id="examples-storage"></a>
## Separating filesystem effects

`Storage` supplies the three effects the installer needs: inspect an entry, prepare its parent and exclusively create a writer. `Disk` accepts only regular entries with identical readable bytes. The no-follow metadata check rejects symlinks, while `create_new` refuses a competing occupant even after a clean preflight. Tests can inject failure at these effect boundaries without replacing the installation algorithm.

<!-- fragment «examples-storage» owner="assembly" source="crates/grove/src/examples.rs" lines="40-75" parent="configuration-examples" -->
````rust

/// Filesystem effects are separate so acceptance can interleave a competing
/// creation or fail a write after bytes have reached a real temporary file.
trait Storage {
    type Writer: Write;
    fn existing(&self, path: &Path) -> io::Result<Option<Vec<u8>>>;
    fn prepare(&self, directory: &Path) -> io::Result<()>;
    fn create(&self, path: &Path) -> io::Result<Self::Writer>;
}

struct Disk;

impl Storage for Disk {
    type Writer = File;

    fn existing(&self, path: &Path) -> io::Result<Option<Vec<u8>>> {
        match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_file() => fs::read(path).map(Some),
            Ok(_) => Err(io::Error::other(
                "not a regular file (symlinks are refused)",
            )),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn prepare(&self, directory: &Path) -> io::Result<()> {
        fs::create_dir_all(directory)
    }

    fn create(&self, path: &Path) -> io::Result<File> {
        // Exclusive creation also refuses dangling symlinks and occupants that
        // arrived after preflight. https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
        OpenOptions::new().write(true).create_new(true).open(path)
    }
}
````
<!-- /fragment -->

<a id="examples-report"></a>
## Reporting completed and incomplete work

`Report` retains created paths, unchanged paths and failures separately, then formats them together. A created path means an exclusive open succeeded; a later write failure identifies that path as potentially partial. Keeping that distinction lets the command describe actual disk effects without promising a batch transaction.

<!-- fragment «examples-report» owner="assembly" source="crates/grove/src/examples.rs" lines="76-96" parent="configuration-examples" -->
````rust

#[derive(Default)]
struct Report {
    created: Vec<PathBuf>,
    unchanged: Vec<PathBuf>,
    failures: Vec<String>,
}

impl Report {
    fn render(&self) -> String {
        let mut lines = Vec::new();
        for path in &self.created {
            lines.push(format!("Created: {}", path.display()));
        }
        for path in &self.unchanged {
            lines.push(format!("Unchanged: {}", path.display()));
        }
        lines.extend(self.failures.iter().cloned());
        lines.join("\n")
    }
}
````
<!-- /fragment -->

<a id="examples-install"></a>
## Preflight before exclusive creation

Consider a missing first sample and an edited second sample. `install` inspects the entire fixed set and returns both conflicts before creating the first. If all entries are matching or absent, it prepares the directory and creates only missing files in package order. Each successful open is recorded before writing. A competing creation stops the loop without replacement; a short write leaves the new partial file and earlier creations in the report. Nothing is removed to simulate rollback, so a retry can itself refuse the partial contents.

<!-- fragment «examples-install» owner="assembly" source="crates/grove/src/examples.rs" lines="97-154" parent="configuration-examples" -->
````rust

/// Preflight the whole set before creating even the destination directory.
/// Successful opens are recorded before writes: a short write leaves a new,
/// possibly partial file, which must be reported rather than silently removed.
fn install(directory: &Path, storage: &impl Storage) -> Report {
    let mut report = Report::default();
    let mut missing = Vec::new();
    for &(name, bytes) in EXAMPLES {
        let path = directory.join(name);
        match storage.existing(&path) {
            Ok(Some(existing)) if existing == bytes => report.unchanged.push(path),
            Ok(Some(_)) => report
                .failures
                .push(format!("Conflict: {}: different contents", path.display())),
            Ok(None) => missing.push((path, bytes)),
            Err(error) => report
                .failures
                .push(format!("Conflict: {}: {error}", path.display())),
        }
    }
    if !report.failures.is_empty() {
        report.failures.push("No files created. Move conflicting entries aside or choose to keep them; no overwrite option is provided.".into());
        return report;
    }
    if missing.is_empty() {
        return report;
    }
    if let Err(error) = storage.prepare(directory) {
        report.failures.push(format!(
            "Cannot create directory {}: {error}. No files created.",
            directory.display()
        ));
        return report;
    }
    for (path, bytes) in missing {
        let result = match storage.create(&path) {
            Ok(mut file) => {
                report.created.push(path.clone());
                file.write_all(bytes).and_then(|()| file.flush())
            }
            Err(error) => {
                report.failures.push(format!("Cannot exclusively create {}: {error}. Existing entries were not replaced; inspect this path before retrying.", path.display()));
                break;
            }
        };
        if let Err(error) = result {
            report.failures.push(format!("Cannot finish writing {}: {error}. This newly created file may be partial; inspect it before retrying.", path.display()));
            break;
        }
    }
    if !report.failures.is_empty() {
        report.failures.push(
            "Installation incomplete. Created files listed above remain; no files were removed."
                .into(),
        );
    }
    report
}
````
<!-- /fragment -->

<a id="examples-run"></a>
## Delivery without session setup

`run` resolves HOME as a native path, refuses a missing or empty value, and calls the installer for the fixed `.config/grove` directory. The CLI dispatches this branch before even resolving the working directory. A successful report goes to stdout; a report with failures becomes the ordinary exit-1 error on stderr. No workspace, policy reader or epoch is involved.

<!-- fragment «examples-run» owner="assembly" source="crates/grove/src/examples.rs" lines="155-167" parent="configuration-examples" -->
````rust

/// Resolve the fixed personal directory independently of workspace/epoch state.
pub fn run() -> anyhow::Result<()> {
    let home = std::env::var_os("HOME").filter(|value| !value.is_empty())
        .context("HOME is missing or empty; set it to your home directory before running grove config examples")?;
    let report = install(&PathBuf::from(home).join(".config/grove"), &Disk);
    let output = report.render();
    if !report.failures.is_empty() {
        bail!("{output}");
    }
    println!("{output}");
    Ok(())
}
````
<!-- /fragment -->

<a id="examples-faults"></a>
## Interleaving real filesystem failures

The test adapter delegates to real temporary files while controlling the timing of a second creation or short write. `ShortWriter` writes three bytes before returning an injected error; `FaultyDisk` can create a competing occupant immediately before the exclusive open. Permission and directory failures are injected separately, so those paths remain testable even with elevated filesystem privileges.

<!-- fragment «examples-faults» owner="assembly" source="crates/grove/src/examples.rs" lines="168-241" parent="configuration-examples" -->
````rust

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    enum Fault {
        Race,
        Write,
        Prepare,
        Unreadable,
    }

    struct FaultyDisk {
        fault: Fault,
        creates: Cell<usize>,
    }

    struct ShortWriter {
        file: File,
        fail: bool,
        wrote: bool,
    }

    impl Write for ShortWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail && self.wrote {
                return Err(io::Error::other("injected disk full"));
            }
            self.wrote = true;
            self.file.write(if self.fail { &bytes[..3] } else { bytes })
        }

        fn flush(&mut self) -> io::Result<()> {
            self.file.flush()
        }
    }

    impl Storage for FaultyDisk {
        type Writer = ShortWriter;

        fn existing(&self, path: &Path) -> io::Result<Option<Vec<u8>>> {
            if matches!(self.fault, Fault::Unreadable) && path.ends_with(EXAMPLES[2].0) {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "injected unreadable entry",
                ));
            }
            Disk.existing(path)
        }

        fn prepare(&self, directory: &Path) -> io::Result<()> {
            if matches!(self.fault, Fault::Prepare) {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "injected directory refusal",
                ));
            }
            Disk.prepare(directory)
        }

        fn create(&self, path: &Path) -> io::Result<ShortWriter> {
            let index = self.creates.get();
            self.creates.set(index + 1);
            if matches!(self.fault, Fault::Race) && index == 1 {
                fs::write(path, "competing writer")?;
            }
            Ok(ShortWriter {
                file: Disk.create(path)?,
                fail: matches!(self.fault, Fault::Write) && index == 1,
                wrote: false,
            })
        }
    }
````
<!-- /fragment -->

<a id="examples-race-test"></a>
## Observing a competing occupant

The race test checks three distinct outcomes: the first completed sample has the intended bytes, the competing second file retains its own contents, and no third file is created. The report must name the first creation and the failed second destination. These disk observations would expose replacing creation with a truncating open.

<!-- fragment «examples-race-test» owner="assembly" source="crates/grove/src/examples.rs" lines="242-264" parent="configuration-examples" -->
````rust

    #[test]
    fn racing_occupant_survives_and_prior_creation_is_reported() {
        let directory = tempfile::tempdir().unwrap();
        let storage = FaultyDisk {
            fault: Fault::Race,
            creates: Cell::new(0),
        };
        let report = install(directory.path(), &storage);
        assert_eq!(report.created, [directory.path().join(EXAMPLES[0].0)]);
        assert!(!report.failures.is_empty());
        assert!(report.render().contains("Cannot exclusively create"));
        assert!(report.render().contains(EXAMPLES[1].0));
        assert_eq!(
            fs::read(directory.path().join(EXAMPLES[0].0)).unwrap(),
            EXAMPLES[0].1
        );
        assert_eq!(
            fs::read_to_string(directory.path().join(EXAMPLES[1].0)).unwrap(),
            "competing writer"
        );
        assert!(!directory.path().join(EXAMPLES[2].0).exists());
    }
````
<!-- /fragment -->

<a id="examples-write-test"></a>
## Observing partial writes and retries

The write test begins with a matching instructions file, then fails after three bytes of the second new sample. It checks both new paths, the short contents, the unchanged modification time and refusal of a subsequent retry. The test establishes the report and preservation contract through observable effects.

<!-- fragment «examples-write-test» owner="assembly" source="crates/grove/src/examples.rs" lines="265-296" parent="configuration-examples" -->
````rust

    #[test]
    fn late_write_failure_reports_partial_file_and_preserves_matching_file() {
        let directory = tempfile::tempdir().unwrap();
        let matching = directory.path().join(EXAMPLES[EXAMPLES.len() - 1].0);
        fs::write(&matching, EXAMPLES[EXAMPLES.len() - 1].1).unwrap();
        let modified = fs::metadata(&matching).unwrap().modified().unwrap();
        let storage = FaultyDisk {
            fault: Fault::Write,
            creates: Cell::new(0),
        };
        let report = install(directory.path(), &storage);
        assert_eq!(report.created.len(), 2);
        assert_eq!(report.unchanged, std::slice::from_ref(&matching));
        assert!(report.render().contains("may be partial"));
        assert_eq!(
            fs::read(directory.path().join(EXAMPLES[0].0)).unwrap(),
            EXAMPLES[0].1
        );
        assert_eq!(
            fs::read(directory.path().join(EXAMPLES[1].0)).unwrap(),
            &EXAMPLES[1].1[..3]
        );
        assert!(!directory.path().join(EXAMPLES[2].0).exists());
        assert_eq!(
            fs::metadata(matching).unwrap().modified().unwrap(),
            modified
        );
        let retry = install(directory.path(), &Disk);
        assert!(retry.created.is_empty());
        assert!(retry.render().contains("different contents"));
    }
````
<!-- /fragment -->

<a id="examples-refusal-test"></a>
## Refusing before any creation

The remaining test injects unreadable preflight and parent-directory refusal independently. Both must return failures with no recorded or attempted file creation and no destination directory. Process acceptance in `crates/grove/tests/example_installation.rs` additionally checks exact packaged bytes, streams, CLI usage, real symlinks and unreadable entries; `config_examples.rs` resolves and launches the installed KDL files.

<!-- fragment «examples-refusal-test» owner="assembly" source="crates/grove/src/examples.rs" lines="297-314" parent="configuration-examples" -->
````rust

    #[test]
    fn unreadable_preflight_and_directory_failure_create_nothing() {
        for fault in [Fault::Unreadable, Fault::Prepare] {
            let directory = tempfile::tempdir().unwrap();
            let destination = directory.path().join("new");
            let storage = FaultyDisk {
                fault,
                creates: Cell::new(0),
            };
            let report = install(&destination, &storage);
            assert!(report.created.is_empty());
            assert!(!report.failures.is_empty());
            assert_eq!(storage.creates.get(), 0);
            assert!(!destination.exists());
        }
    }
}
````
<!-- /fragment -->

[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

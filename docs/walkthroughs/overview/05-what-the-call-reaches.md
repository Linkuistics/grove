# What the call reaches
<!-- book-page id="what-the-call-reaches" slice="assembly" order="5" -->
[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

<a id="assembly"></a>
## Observation and lifecycle have separate lifetimes

<!-- rollup «owned-lines-total» -->
The book reconstructs 412 source lines. The preceding chapters explain parsing,
dispatch and their tests. This chapter
connects the public library calls and owns the configuration report formatter,
which turns validated records into a human explanation.

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
active design route still fails this request before output. JSON remains the
next inspection increment; this chapter explains the human presentation.

<!-- fragment «configuration-report» owner="assembly" source="crates/grove/src/config.rs" lines="1-157" parent="source-configuration-report" -->
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

<!-- fragment «inspection-load» owner="assembly" source="crates/grove/src/config.rs" lines="1-18" parent="configuration-report" -->
````rust
//! Human presentation of the same captured configuration the driver launches.
use std::io::{self, Write};
use std::path::Path;

use grove_loop::{SessionConfig, Workspace};
use keyed_launch::{AssignmentValue, CompiledWord, Inspection, Setting, SourceSpan};

/// Load everything before filtering or writing a report. No lease, epoch or
/// task-tree operation belongs on this path; only source admission may ask jj.
pub fn show(cwd: &Path, kind: Option<&str>) -> anyhow::Result<()> {
    let workspace = Workspace::resolve(cwd)?;
    let config = SessionConfig::load_for_worktree(workspace.root())?;
    if let Some(kind) = kind {
        config.require(kind)?;
    }
    write_human(&mut io::stdout().lock(), config.inspect(), kind)?;
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

<!-- fragment «inspection-labels» owner="assembly" source="crates/grove/src/config.rs" lines="19-38" parent="configuration-report" -->
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

<!-- fragment «inspection-selection» owner="assembly" source="crates/grove/src/config.rs" lines="39-75" parent="configuration-report" -->
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

<!-- fragment «inspection-words» owner="assembly" source="crates/grove/src/config.rs" lines="76-118" parent="configuration-report" -->
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

<!-- fragment «inspection-histories» owner="assembly" source="crates/grove/src/config.rs" lines="119-157" parent="configuration-report" -->
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
There are 6 ownership blocks over 4 source roots; 5 blocks belong to chapters
after Orientation. Each is resolved in the source index.

<!-- rollup «early-use-rows» -->
The 5 early-use rows name loop items before their full explanation in Three
steps. The grammar explains its own path and command types at first use.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
Owned source is 57 + 58 + 54 + 86 + 157 = 412 lines across 5 source-owning chapters.
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
The corpus contains 4 roots and 412 lines, explained across 5 chapters and two
lookup pages. No deferred source range belongs in the final book.

[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

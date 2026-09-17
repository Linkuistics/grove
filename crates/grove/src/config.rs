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

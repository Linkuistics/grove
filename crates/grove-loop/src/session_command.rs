//! Grove's public Ravel invocation and exact ending command.
//!
//! The caller supplies the executable already selected and canonicalized by
//! Grove's PATH/version resolver. These helpers never resolve that path again.
//! Both callers share these public launch and ending commands.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use anyhow::Context;
use keyed_launch::Argv;

use crate::{
    session_result::{CONTINUE_RESULT, FORCE_STOP_RESULT},
    Error, Selection,
};

/// The lifecycle request for the same selection that composed the prompt.
///
/// Only public selection inputs and the caller's writable exit directory are
/// sent. Ravel's stdout, captured separately by the caller, is the result.
#[must_use]
pub fn lifecycle(ravel: &Path, task: &Selection, prompt: &str, exit_dir: &Path) -> Argv {
    let mut args = start(task.kind.label(), prompt, exit_dir);
    args.push(value("--task-file=", task.path.as_os_str()));
    args.push(value("--task-id=", task.handle.to_string().as_ref()));
    Argv::new(ravel.into(), args)
}

/// A standalone request whose managed process is confined to the staged cwd.
///
/// Runtime files are canonicalized now, before the runner changes cwd. Ravel
/// admits the real front itself; no policy directory or worker grant is added.
/// Owner routing settings, bounds and environment grants remain owner inputs.
///
/// # Errors
/// A runtime-file path cannot be resolved in the caller's current directory.
pub fn confined(
    ravel: &Path,
    kind: &str,
    prompt: &str,
    exit_dir: &Path,
    runtime_read: &[PathBuf],
) -> Result<Argv, Error> {
    // https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/specs/session-execution.md
    let mut args = start(kind, prompt, exit_dir);
    args.push("--confine".into());
    for path in runtime_read {
        let canonical = path.canonicalize().with_context(|| {
            format!(
                "resolving runtime file {} before staging cwd",
                path.display()
            )
        })?;
        args.push(value("--runtime-read=", canonical.as_os_str()));
    }
    Ok(Argv::new(ravel.into(), args))
}

/// The prompt's exact ending command, using the already selected executable.
///
/// Quote its UTF-8 path as one literal shell word, including embedded quotes
/// and newlines. The opaque result is Grove's, never a Ravel session identifier.
///
/// # Errors
/// A path that cannot be represented in a UTF-8 prompt. Select a UTF-8 Ravel
/// installation path before launching a session.
pub fn end_session(ravel: &Path) -> Result<String, Error> {
    end_with_result(ravel, CONTINUE_RESULT)
}

/// The exact command to stop without acknowledging completion.
///
/// # Errors
/// A non-UTF-8 executable path, as for [`end_session`].
pub fn force_stop(ravel: &Path) -> Result<String, Error> {
    end_with_result(ravel, FORCE_STOP_RESULT)
}

fn end_with_result(ravel: &Path, result: &str) -> Result<String, Error> {
    let path = ravel.to_str().context(
        "the ending command requires a UTF-8 Ravel path; select a UTF-8 installation on PATH",
    )?;
    Ok(format!(
        "'{}' end-session {result}",
        path.replace('\'', "'\\''")
    ))
}

fn start(kind: &str, prompt: &str, exit_dir: &Path) -> Vec<OsString> {
    // Ravel 0.1.0's documented flags return only the opaque result on stdout.
    // https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/session-cli.md
    vec![
        "start-session".into(),
        value("--kind=", kind.as_ref()),
        value("--prompt=", prompt.as_ref()),
        value("--exit-dir=", exit_dir.as_os_str()),
    ]
}

// Join each value to its flag so a leading dash never becomes another option.
fn value(flag: &str, argument: &OsStr) -> OsString {
    let mut word = OsString::from(flag);
    word.push(argument);
    word
}

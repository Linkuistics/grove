//! The child-process seam: every `jj` invocation this crate makes is built
//! here, so the hygiene below is a property of the crate rather than a habit
//! each call site has to remember.
//!
//! Two things are true of every invocation and of nothing else:
//!
//! **The repository is chosen by `current_dir` and by nothing ambient.**
//! Repository selectors are process-global overrides — `current_dir` alone does
//! not stop a Git-aware child from following an inherited foreign repository,
//! and a jj workspace colocated with Git has a real Git backend for them to
//! redirect. They are removed rather than merely left unset, because an
//! environment is inherited, not addressed. There is no `JJ_*` counterpart to
//! remove: jj selects its repository by walking up from the working directory,
//! and its own variables (`JJ_CONFIG`, `JJ_USER`, …) configure the *user*, so
//! stripping them would change who a commit is attributed to.
//!
//! **A consumer's own ambient variables are left alone.** This crate cannot
//! know which of them carry authority, and jj reads none of them; a consumer
//! whose environment grants something to its descendants owns that scrubbing at
//! its own spawn sites.

use crate::refusal::Refusal;
use std::path::Path;
use std::process::Command;

/// Repository selectors — the variables that answer "which repository?" ahead
/// of the working directory.
const REPOSITORY_SELECTORS: [&str; 4] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_COMMON_DIR",
    "GIT_INDEX_FILE",
];

/// Run `jj <args>` in `directory` and return its stdout as text.
///
/// Failure to *start* and failure to *succeed* are separate refusals: the first
/// means jj is not installed and the remedy is installation, the second means
/// jj declined and the remedy is whatever it printed.
pub(crate) fn output(directory: &Path, args: &[&str]) -> Result<String, Refusal> {
    let bytes = raw_output(directory, args)?;
    String::from_utf8(bytes).map_err(|_| Refusal::output_not_text(&rendered("jj", args)))
}

/// As [`output`], but the answer is *whether there was any* rather than what it
/// said — for the probes whose whole result is stdout being empty or not.
pub(crate) fn produced_output(directory: &Path, args: &[&str]) -> Result<bool, Refusal> {
    Ok(!raw_output(directory, args)?.is_empty())
}

fn raw_output(directory: &Path, args: &[&str]) -> Result<Vec<u8>, Refusal> {
    let rendered = rendered("jj", args);
    let mut command = Command::new("jj");
    command.current_dir(directory).args(args);
    for selector in REPOSITORY_SELECTORS {
        command.env_remove(selector);
    }
    let out = command
        .output()
        .map_err(|cause| Refusal::not_runnable(&rendered, cause))?;
    if !out.status.success() {
        return Err(Refusal::command_failed(
            &rendered,
            directory,
            &String::from_utf8_lossy(&out.stderr),
        ));
    }
    Ok(out.stdout)
}

/// The command as a reader would type it, for a refusal to quote. Arguments are
/// shown verbatim: the crate builds every one of them, so none is user text
/// that could need quoting to stay honest.
fn rendered(program: &str, args: &[&str]) -> String {
    let mut line = String::from(program);
    for arg in args {
        line.push(' ');
        line.push_str(arg);
    }
    line
}

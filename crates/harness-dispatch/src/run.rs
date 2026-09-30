//! `run`: make the same choice `inspect` reports, then replace this process
//! with the selected harness (`docs/specs/harness-selection-and-execution.md`,
//! *Execution and authority*;
//! `docs/adr/policy-evaluation-precedes-process-replacement.md`).
//!
//! This is the plain exec. The harness inherits the caller's cwd, descriptors,
//! environment and process identity, so its own exit code or signal is the
//! command's and there is no supervisor left to report on it. Stdout and stdin
//! stay the harness's; the choice is announced in one line on stderr.
//!
//! Not yet here: the required handoff record and the run ID
//! (`handoff-records-k24`, which makes `run` a delivered form), and handled
//! signals with a signal-transparent handoff (`evaluation-boundary-k27`).

use std::io;
use std::os::unix::process::CommandExt as _;
use std::process::Command;

use serde_json::json;

use crate::choice::{self, Choice};
use crate::cli::RunArgs;
use crate::inputs::PromptRequirement;
use crate::refusal::{Failure, Refusal, Stage, EXIT_NOT_FOUND, EXIT_UNEXECUTABLE};

/// Select and exec. It returns only when there is nothing to exec, or exec
/// itself failed.
pub fn run(args: &RunArgs) -> Failure {
    let choice = match choice::choose(&args.selection, PromptRequirement::Required) {
        Ok(choice) => choice,
        Err(failure) => return failure,
    };
    announce(&choice, args.json);
    let words = choice.argv[1..].iter().map(|word| {
        word.text()
            .expect("run requires a prompt, so no placeholder remains")
    });
    // `Command::exec` is `execvp` of the resolved path, which is absolute, so
    // nothing is searched for twice. argv[0] is the program as configured.
    let error = Command::new(&choice.executable.path)
        .arg0(&choice.executable.program)
        .args(words)
        .exec();
    exec_failed(&choice, &error).into()
}

/// The choice, on stderr: whatever the policy printed, then one line naming
/// the candidate and the file about to replace this process. With `--json`,
/// one JSON object instead.
fn announce(choice: &Choice, json: bool) {
    let candidate = &choice.candidate;
    let executable = choice.executable.path.to_string_lossy();
    if json {
        let notice = json!({
            "schemaVersion": 1,
            "handoff": {
                "kind": choice.inputs.kind,
                "candidateId": candidate.id,
                "provider": candidate.provider,
                "model": candidate.model,
                "effort": candidate.effort,
                "reason": choice.reason,
                "executable": executable,
            },
            "diagnostics": choice.diagnostics.to_json(),
        });
        eprintln!("{notice}");
    } else {
        eprint!("{}", choice.diagnostics.to_text());
        eprintln!(
            "harness-dispatch: running candidate {:?} (provider {}, model {}, effort {}) for kind \
             {:?}: {executable}",
            candidate.id, candidate.provider, candidate.model, candidate.effort, choice.inputs.kind,
        );
    }
}

/// Exec returned, so the harness never started. `ENOENT` (the file, or its
/// `#!` interpreter, is gone) exits 127 like an unresolved program; anything
/// else exits 126.
fn exec_failed(choice: &Choice, error: &io::Error) -> Refusal {
    let (exit, remedy) = match error.raw_os_error() {
        Some(libc::ENOENT) => (
            EXIT_NOT_FOUND,
            "the program, or the interpreter its #! line names, does not exist; install it, or \
             correct the candidate's program",
        ),
        Some(libc::E2BIG) => (
            EXIT_UNEXECUTABLE,
            "the arguments and environment exceed this platform's exec limit, usually because of \
             the prompt's size; shorten the prompt, or have the harness read it from a file",
        ),
        Some(libc::ENOEXEC) => (
            EXIT_UNEXECUTABLE,
            "the file is not in a format this platform executes; give it a #! line, or name its \
             interpreter as the program",
        ),
        Some(libc::EACCES | libc::EPERM) => (
            EXIT_UNEXECUTABLE,
            "check the file's execute permission, and that its filesystem is not mounted noexec",
        ),
        Some(libc::ETXTBSY) => (
            EXIT_UNEXECUTABLE,
            "the program file is still open for writing; run again once it is complete",
        ),
        _ => (
            EXIT_UNEXECUTABLE,
            "correct the candidate's program, or whatever in the environment prevents its exec",
        ),
    };
    let source = choice.entry.display();
    Refusal::new(
        "exec_failed",
        Stage::Exec,
        exit,
        format!(
            "exec of {} for candidate {:?} failed: {error}",
            choice.executable.path.display(),
            choice.candidate.id
        ),
        format!("{remedy}; harness-dispatch never runs another candidate instead"),
    )
    .source(source)
    .location(format!("policy.catalog[{}].program", choice.index))
}

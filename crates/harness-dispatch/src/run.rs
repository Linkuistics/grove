//! `run`: make the same choice `inspect` reports, commit the required handoff
//! record, then replace this process with the selected harness
//! (`docs/specs/harness-selection-and-execution.md`, *Execution and authority*,
//! *Records and later observations*;
//! `docs/adr/policy-evaluation-precedes-process-replacement.md`).
//!
//! The record comes first: a run is launched only once its handoff attempt is
//! durable, and a failure to commit it launches nothing (exit 4). The harness
//! then inherits the caller's cwd, descriptors, environment and process
//! identity, plus the run's identity in `HARNESS_DISPATCH_RUN_ID` and the
//! record directory in `HARNESS_DISPATCH_STATE_DIR`, which replace any values
//! it would otherwise inherit. Its own exit code or signal is the command's and
//! there is no supervisor left to report on it. Stdout and stdin stay the
//! harness's; the choice is announced in one line on stderr.
//!
//! Not yet here: handled signals, the post-commit cancellation check with its
//! not-executed detail, and a signal-transparent handoff
//! (`evaluation-boundary-k27`).

use std::io;
use std::os::unix::process::CommandExt as _;
use std::process::Command;

use serde_json::json;

use crate::argv::RunSlot;
use crate::choice::{self, Choice};
use crate::cli::RunArgs;
use crate::inputs::PromptRequirement;
use crate::record;
use crate::refusal::{Failure, Refusal, RunNote, Stage, EXIT_NOT_FOUND, EXIT_UNEXECUTABLE};
use crate::run_id::RunId;
use crate::store::{self, Committed};

/// The harness's copy of the run's identity and record directory.
pub const RUN_ID_VARIABLE: &str = "HARNESS_DISPATCH_RUN_ID";
pub const STATE_DIR_VARIABLE: &str = "HARNESS_DISPATCH_STATE_DIR";

/// Select, record and exec. It returns only when there is nothing to exec, or
/// exec itself failed.
pub fn run(args: &RunArgs) -> Failure {
    let run_id = match RunId::allocate() {
        Ok(run_id) => run_id,
        Err(refusal) => return refusal.into(),
    };
    let slot = RunSlot::Allocated(run_id.clone());
    let choice = match choice::choose(&args.selection, PromptRequirement::Required, slot) {
        Ok(choice) => choice,
        Err(failure) => return failure,
    };
    // The worker has been reaped and the program resolved; only now is the
    // store opened, so no lock is ever held across evaluation.
    let committed = match store::commit(&choice.state_dir, &run_id, &record::launch(&choice)) {
        Ok(committed) => committed,
        Err(refusal) => return Failure::with_diagnostics(refusal, choice.diagnostics.clone()),
    };
    announce(&choice, &run_id, &committed, args.json);
    let words = choice.argv[1..].iter().map(|word| {
        word.text()
            .expect("run fills every slot, so no marked word remains")
    });
    // `Command::exec` is `execvp` of the resolved path, which is absolute, so
    // nothing is searched for twice. argv[0] is the program as configured.
    let error = Command::new(&choice.executable.path)
        .arg0(&choice.executable.program)
        .args(words)
        .env(RUN_ID_VARIABLE, run_id.as_str())
        .env(STATE_DIR_VARIABLE, &choice.state_dir.path)
        .exec();
    let refusal = exec_failed(&choice, &error);
    let detail = json!({
        "cause": "exec_error",
        "stage": Stage::Exec.as_str(),
        "code": refusal.code,
        "errno": error.raw_os_error(),
        "message": refusal.message,
        "exit": refusal.exit,
    });
    let unrecorded = store::append_launch_failure(&choice.state_dir, &run_id, &detail)
        .err()
        .map(|failure| (failure.code, failure.message.clone()));
    refusal
        .run(RunNote {
            id: run_id.to_string(),
            unrecorded,
        })
        .into()
}

/// The choice, on stderr: whatever the policy printed, then one line naming
/// the candidate and the file about to replace this process. With `--json`,
/// one JSON object instead.
fn announce(choice: &Choice, run_id: &RunId, committed: &Committed, json: bool) {
    let candidate = &choice.candidate;
    let executable = choice.executable.path.to_string_lossy();
    if json {
        let notice = json!({
            "schemaVersion": 1,
            "handoff": {
                "runId": run_id.as_str(),
                "recordedAt": committed.recorded_at,
                "stateDir": choice.state_dir.path.to_string_lossy(),
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
             {:?} as run {run_id}: {executable}",
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

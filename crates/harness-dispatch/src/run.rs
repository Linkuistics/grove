//! `run`: make the same choice `inspect` reports, commit the required handoff
//! record, then replace this process with the selected command
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
//! harness's; the handoff is announced in one line on stderr.
//!
//! Every failure `run` reports once its command line has parsed also names the
//! equivalent `inspect` invocation, so that an unattended refusal can be
//! reproduced without reconstructing its inputs. A command line that does not
//! parse has no such equivalent.
//!
//! INT, TERM and HUP cancel the selection until its program is resolved
//! (`choice`), with nothing recorded. Their handlers stay installed across the
//! commit, and the linearization point follows it: the handled signals are
//! blocked and looked for once more. A signal seen there launches nothing,
//! marks the committed attempt not executed, and is re-raised. Otherwise the
//! harness is exec'd with the caller's signal mask and every disposition that
//! survives exec, SIGPIPE's included, as this process inherited them
//! (`signal_state`). A signal delivered once the caller's mask is back and
//! before exec completes can still end this process with the attempt recorded
//! and its execution unknown; no userspace exec closes that window.

use std::io;
use std::os::unix::process::CommandExt as _;
use std::process::Command;

use serde_json::json;

use crate::cancellation::{self, Signal};
use crate::choice::{self, Choice, Selected};
use crate::cli::RunArgs;
use crate::inputs::PromptRequirement;
use crate::record;
use crate::refusal::{Failure, Refusal, RunNote, Stage, EXIT_NOT_FOUND, EXIT_UNEXECUTABLE};
use crate::run_id::RunId;
use crate::signal_state;
use crate::store::{self, Committed};

/// The harness's copy of the run's identity and record directory.
pub const RUN_ID_VARIABLE: &str = "HARNESS_DISPATCH_RUN_ID";
pub const STATE_DIR_VARIABLE: &str = "HARNESS_DISPATCH_STATE_DIR";

/// Select, record and exec. It returns only when there is nothing to exec, or
/// exec itself failed, and the failure it returns names the equivalent
/// `inspect` invocation.
pub fn run(args: &RunArgs) -> Failure {
    attempt(args).with_inspect(args.selection.inspect_invocation(args.json))
}

fn attempt(args: &RunArgs) -> Failure {
    if let Err(refusal) = signal_state::recorded() {
        return refusal.into();
    }
    let run_id = match RunId::allocate() {
        Ok(run_id) => run_id,
        Err(refusal) => return refusal.into(),
    };
    let Selected {
        choice,
        handlers,
        source,
    } = match choice::choose(&args.selection, PromptRequirement::Required) {
        Ok(selected) => selected,
        Err(failure) => return failure,
    };
    // The worker has been reaped and the program resolved; only now is the
    // store opened, so no lock is ever held across evaluation. A signal during
    // the commit is noted, and seen at the linearization point.
    let committed = match store::commit(&choice.state_dir, &run_id, &record::launch(&choice)) {
        Ok(committed) => committed,
        Err(refusal) => {
            drop(handlers);
            let failure = Failure::with_diagnostics(refusal, choice.diagnostics.clone());
            return choice::overruled(failure, &source);
        }
    };
    // Before the linearization point, so that a stderr slow to take the line
    // delays the final check rather than widening the window after it.
    announce(&choice, &run_id, &committed, args.json);
    if let Some(signal) = handlers.block_and_check() {
        return not_executed(&choice, &run_id, signal, &source);
    }
    // The handled signals take their entry dispositions again while they are
    // blocked. Whatever arrives from now on waits for the caller's mask.
    drop(handlers);
    // `Command::exec` is `execvp` of the resolved path, which is absolute, so
    // nothing is searched for twice. argv[0] is the program as `select`
    // returned it, and each argument is one whole word, exactly as returned.
    // It sets SIGPIPE to default before running the hook, which then
    // reinstates the caller's dispositions and, last, the caller's mask.
    let mut command = Command::new(&choice.executable.path);
    command
        .arg0(&choice.executable.program)
        .args(&choice.command.args)
        .env(RUN_ID_VARIABLE, run_id.as_str())
        .env(STATE_DIR_VARIABLE, &choice.state_dir.path);
    // SAFETY: the hook makes only sigaction and pthread_sigmask calls, and
    // runs in this process, since exec does not fork.
    unsafe { command.pre_exec(signal_state::reinstate) };
    let error = command.exec();
    // Exec may have failed before the hook ran. Either way the caller's state
    // is back from here on, and a signal takes its entry course.
    let _ = signal_state::reinstate();
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

/// A signal seen at the linearization point: nothing is launched, and the
/// committed attempt is marked not executed where the store allows. A failed
/// append leaves it a handoff attempt whose execution is unknown, never a
/// success. The signal is re-raised once the refusal is reported. The handoff
/// notice already carried what the policy printed, so, as after an exec
/// error, the refusal does not repeat it.
fn not_executed(choice: &Choice, run_id: &RunId, signal: Signal, source: &str) -> Failure {
    let refusal = cancellation::handoff_refusal(signal, source, run_id.as_str());
    let detail = json!({
        "cause": "cancelled",
        "stage": Stage::Exec.as_str(),
        "code": refusal.code,
        "signal": signal.name(),
        "message": refusal.message,
        "exit": refusal.exit,
    });
    let unrecorded = store::append_launch_failure(&choice.state_dir, run_id, &detail)
        .err()
        .map(|failure| (failure.code, failure.message.clone()));
    refusal
        .run(RunNote {
            id: run_id.to_string(),
            unrecorded,
        })
        .into()
}

/// The handoff, on stderr: whatever the policy printed, then one line naming
/// the command's labels and the file about to replace this process. With
/// `--json`, one JSON object instead.
fn announce(choice: &Choice, run_id: &RunId, committed: &Committed, json: bool) {
    let command = &choice.command;
    let executable = &choice.executable.path;
    if json {
        let notice = json!({
            "schemaVersion": 1,
            "handoff": {
                "runId": run_id.as_str(),
                "recordedAt": committed.recorded_at,
                "stateDir": choice.state_dir.path.to_string_lossy(),
                "kind": choice.inputs.kind,
                "provider": command.provider,
                "model": command.model,
                "effort": command.effort,
                "reason": command.reason,
                "executable": executable,
            },
            "diagnostics": choice.diagnostics.to_json(),
        });
        eprintln!("{notice}");
    } else {
        eprint!("{}", choice.diagnostics.to_text());
        eprintln!(
            "harness-dispatch: running provider {}, model {}, effort {} for kind {:?} as run \
             {run_id}: {executable}",
            command.provider, command.model, command.effort, choice.inputs.kind,
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
             correct the program select returns",
        ),
        Some(libc::E2BIG) => (
            EXIT_UNEXECUTABLE,
            "the arguments and environment exceed this platform's exec limit, usually because of \
             the prompt's size; shorten the prompt, or have the harness read it from a file an \
             argument names",
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
            "correct the program select returns, or whatever in the environment prevents its exec",
        ),
    };
    let source = choice.entry.display();
    Refusal::new(
        "exec_failed",
        Stage::Exec,
        exit,
        format!(
            "exec of {} for the program {:?} failed: {error}",
            choice.executable.path, choice.command.program
        ),
        format!("{remedy}; harness-dispatch never runs another command instead"),
    )
    .source(source)
    .location("result.program")
}

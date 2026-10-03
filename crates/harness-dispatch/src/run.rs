//! `run`: make the same choice `inspect` reports, commit the required handoff
//! record, then spawn the selected command as a job and supervise it to its end
//! (`docs/specs/harness-selection-and-execution.md`, *Execution and authority*,
//! *Supervision*, *Records and later observations*;
//! `docs/adr/dispatch-supervises-the-harness.md`).
//!
//! The record comes first: a run is launched only once its handoff attempt is
//! durable, and a failure to commit it launches nothing (exit 4). The harness
//! is then this process's child, spawned through the runner (`keyed-launch`)
//! into a process group of its own that holds the terminal. It has the caller's
//! cwd, descriptors and environment, plus the run's identity in
//! [`RUN_ID_VARIABLE`], the record directory in [`STATE_DIR_VARIABLE`] and the
//! run's exit channel in [`EXIT_FILE_VARIABLE`], each replacing any value it
//! would otherwise inherit. Stdout and stdin stay the harness's; the handoff
//! and the run's end are each announced in one line on stderr.
//!
//! With `--confine`, owner paths and grants are checked canonically before
//! evaluation. Only the harness is sandboxed: it runs in a new POSIX session,
//! with null stdin, inherited output, closed extra descriptors and an explicitly
//! granted environment. The backend grants cwd, private scratch and exit storage
//! for writes, with literal read-only runtime files and both executables.
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
//! harness is spawned with the caller's signal mask and every disposition that
//! survives exec, SIGPIPE's included, as this process inherited them
//! (`signal_state`), while the runner's handlers stand in for the selection's.
//! The caller's mask comes back once the spawn has returned, so a signal that
//! arrived after the final check is delivered then, and cancels the run.
//!
//! The run ends when the harness exits, when the exit signal's escalation ends
//! it, or when a handled signal cancels it. Whatever ends it, the runner kills
//! what remains of its group, reaps it, takes the terminal back and confirms
//! the group gone. The ending is chosen only then, and decides the exit status.
//! Once the group is confirmed gone, dispatch writes its end observation to
//! `--ending-file`, then appends the same document to the run.
//! Neither a failed append nor a failed file changes the ending or the exit.

use std::ffi::OsString;
use std::io;
use std::io::Write as _;
use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use keyed_launch::{Argv, Channel, End, Ended, Escalation, Group, Launch, LaunchEvent};
use serde_json::json;

use crate::cancellation::{self, Signal};
use crate::choice::{self, Choice, Selected};
use crate::cli::RunArgs;
use crate::inputs::PromptRequirement;
use crate::observation::{self, Observation, RunEnd};
use crate::record;
use crate::refusal::{
    Failure, Refusal, RunNote, Stage, EXIT_MALFORMED, EXIT_NOT_FOUND, EXIT_UNEXECUTABLE,
    EXIT_WORKER,
};
use crate::run_id::RunId;
use crate::signal_state;
use crate::store::{self, Appended, Committed, NewObservation};

/// The harness's copy of the run's identity and record directory.
pub const RUN_ID_VARIABLE: &str = "HARNESS_DISPATCH_RUN_ID";
pub const STATE_DIR_VARIABLE: &str = "HARNESS_DISPATCH_STATE_DIR";
/// The harness's copy of the run's exit channel, which `exit` creates.
pub const EXIT_FILE_VARIABLE: &str = "HARNESS_DISPATCH_EXIT_FILE";

/// The escalation once the exit channel appears: the **grace**, so the exit
/// verb's own call can return and the session's turn end, then SIGTERM to the
/// harness's group; the **kill-grace**, time for an orderly shutdown, then
/// SIGKILL. The kill-grace is also how long a cancelled interactive run waits
/// between the cancelling signal and SIGKILL. Fixed constants: no setting, flag
/// or policy field changes them, and a caller supervising this process waits
/// longer than they add up to.
const ESCALATION: Escalation = Escalation {
    grace: Duration::from_secs(2),
    kill_grace: Duration::from_secs(5),
};

/// How a run that started a harness ended, chosen once the harness is reaped.
/// Cancellation takes precedence, since the run was taken away whatever the
/// session said.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ending {
    /// A handled signal was latched at the harness's reap sample.
    Cancelled(i32),
    /// Otherwise, the exit channel existed once the harness was reaped.
    ExitSignal,
    /// Otherwise: the harness ended without the exit signal.
    HarnessExit,
}

impl Ending {
    fn of(ended: &Ended) -> Self {
        match ended.end {
            End::Interrupted { signal } => Ending::Cancelled(signal),
            _ if ended.signalled => Ending::ExitSignal,
            _ => Ending::HarnessExit,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Ending::Cancelled(_) => "cancelled",
            Ending::ExitSignal => "exit_signal",
            Ending::HarnessExit => "harness_exit",
        }
    }
}

/// How this process ends once its run has: with a code, or by dying of a
/// signal so its own caller's wait status says what the harness's said.
enum Exit {
    Code(u8),
    Signal(i32),
}

/// Select, record, launch and supervise. A refusal, a cancellation at the
/// linearization point and a harness that could not be started return the
/// failure, which names the equivalent `inspect` invocation. A run that
/// started a harness answers the exit code its ending gives, or dies of the
/// signal it gives.
pub fn run(args: &RunArgs) -> Result<ExitCode, Failure> {
    attempt(args)
        .map_err(|failure| failure.with_inspect(args.selection.inspect_invocation(args.json)))
}

fn attempt(args: &RunArgs) -> Result<ExitCode, Failure> {
    signal_state::recorded()?;
    let exit_dir = args.exit_dir.as_deref().map(exit_dir).transpose()?;
    let ending_file = args.ending_file.as_deref().map(ending_path).transpose()?;
    let run_id = RunId::allocate()?;
    let prepared = choice::prepare(&args.selection, PromptRequirement::Required)?;
    let confined = args
        .confine
        .then(|| {
            crate::confinement::Confined::prepare(
                &prepared,
                exit_dir.as_deref(),
                ending_file.as_deref(),
                &args.runtime_read,
            )
        })
        .transpose()?;
    let Selected {
        choice,
        handlers,
        source,
    } = choice::choose_prepared(prepared)?;
    // Allocated after selection, so the worker could not have been told it,
    // and before the commit, so a failure here records nothing. A signal in
    // the meantime is noted and seen at the linearization point.
    let exit_channel = match ExitChannel::allocate(exit_dir.as_deref().or_else(|| {
        confined
            .as_ref()
            .map(crate::confinement::Confined::exit_dir)
    })) {
        Ok(exit_channel) => exit_channel,
        Err(refusal) => {
            drop(handlers);
            return Err(Failure::with_diagnostics(
                refusal,
                choice.diagnostics.clone(),
            ));
        }
    };
    // The worker has been reaped and the program resolved; only now is the
    // store opened, so no lock is ever held across evaluation.
    let mut document = record::launch(&choice);
    document["confinement"] = confined.as_ref().map_or(
        serde_json::Value::Null,
        |confined| json!({ "runtimeRead": confined.runtime_read }),
    );
    let committed = match store::commit(&choice.state_dir, &run_id, &document) {
        Ok(committed) => committed,
        Err(refusal) => {
            drop(handlers);
            let failure = Failure::with_diagnostics(refusal, choice.diagnostics.clone());
            return Err(choice::overruled(failure, &source));
        }
    };
    // Before the linearization point, so that a stderr slow to take the line
    // delays the final check rather than widening the window after it.
    announce(&choice, &run_id, &committed, args.json);
    if let Some(signal) = handlers.block_and_check() {
        return Err(not_executed(&choice, &run_id, signal, &source));
    }

    // Unconfined argv[0] is the program as `select` returned it; a confined
    // launcher's child receives the resolved path instead. The resolved path,
    // which is absolute, is what is spawned, so nothing is searched for twice.
    // Each argument is one whole word, exactly as returned.
    let argv = Argv::new(
        OsString::from(&choice.executable.path),
        choice.command.args.iter().map(OsString::from).collect(),
    )
    .with_arg0(OsString::from(&choice.executable.program));
    let state_dir = choice.state_dir.path.as_os_str();
    let mut environment = confined
        .as_ref()
        .map_or_else(Vec::new, crate::confinement::Confined::environment);
    environment.extend([
        (
            OsString::from(RUN_ID_VARIABLE),
            OsString::from(run_id.as_str()),
        ),
        (OsString::from(STATE_DIR_VARIABLE), state_dir.to_owned()),
    ]);
    let grant: Vec<_> = environment
        .iter()
        .map(|(name, value)| (name.as_os_str(), value.as_os_str()))
        .collect();
    let entry = signal_state::entry();
    let mut started = false;
    let launch = Launch {
        argv: &argv,
        channel: &exit_channel.channel,
        channel_var: EXIT_FILE_VARIABLE,
        scrub: &[],
        grant: &grant,
        transparent: Some(&entry),
        cwd: confined
            .as_ref()
            .map(|confined| confined.writable[0].as_path()),
        escalation: ESCALATION,
    };
    let mut observe = |event| {
        if event == LaunchEvent::Started {
            started = true;
            // The handled signals have been blocked since the final check.
            // The runner's handlers are installed by now, so one that
            // arrived in the meantime cancels the run.
            let _ = signal_state::restore_mask();
        }
    };
    let supervised = match &confined {
        Some(confined) => keyed_launch::run_confined_observed(
            launch,
            &keyed_launch::FilesystemGrants {
                writable: &confined.writable,
                runtime_read: &confined.reads(),
            },
            &mut observe,
        ),
        None => keyed_launch::run_observed(launch, &mut observe),
    };
    let _ = signal_state::restore_mask();
    let cleanup = exit_channel.remove();
    let cleanup = match confined {
        Some(confined) => {
            let private = confined.remove();
            cleanup.and(private)
        }
        None => cleanup,
    };
    let ended = match supervised {
        Ok(ended) => ended,
        Err(error) if !started => return Err(launch_failed(&choice, &run_id, &error)),
        Err(error) => {
            report_cleanup(cleanup);
            eprintln!(
                "harness-dispatch: run {run_id}: supervision failed: {error}; members of the \
                 harness's process group may survive it"
            );
            return Ok(ExitCode::from(EXIT_WORKER));
        }
    };
    report_cleanup(cleanup);

    let ending = Ending::of(&ended);
    let recorded = record_end(&choice, &run_id, ending, &ended, ending_file.as_deref());
    announce_end(&run_id, ending, &ended, recorded, args.json);
    // Keep the runner's handlers through recording and the notice. A signal
    // after the reap sample ends this invocation, but cannot change the run's
    // ending.
    // Restoring dispositions before the final read closes the handler window;
    // reraise installs SIG_DFL itself when reproducing a signal below.
    drop(handlers);
    let late_signal = keyed_launch::take_interrupt();
    if let Group::Present { pgid } = ended.group {
        eprintln!(
            "harness-dispatch: run {run_id}: members of the harness's process group {pgid} may \
             survive it: they were sent SIGKILL twice and still answered a second later"
        );
        return Ok(ExitCode::from(EXIT_WORKER));
    }
    if let Some(signal) = late_signal.filter(|_| !matches!(ending, Ending::Cancelled(_))) {
        die_of(signal);
    }
    match exit(ending, &ended) {
        Exit::Code(code) => Ok(ExitCode::from(code)),
        Exit::Signal(signal) => die_of(signal),
    }
}

/// The exit status a run's ending gives: the harness's own exit when it ended
/// without the exit signal, 0 when the exit signal's escalation ended it or it
/// exited 0 after sending it, and death by the signal that cancelled it.
fn exit(ending: Ending, ended: &Ended) -> Exit {
    use std::os::unix::process::ExitStatusExt as _;
    let harness = || match (ended.status.code(), ended.status.signal()) {
        (Some(code), _) => Exit::Code(u8::try_from(code & 0xff).unwrap_or(u8::MAX)),
        (None, Some(signal)) => Exit::Signal(signal),
        (None, None) => Exit::Code(EXIT_WORKER),
    };
    match ending {
        Ending::Cancelled(signal) => Exit::Signal(signal),
        Ending::ExitSignal if ended.end == End::Escalated || ended.status.success() => {
            Exit::Code(0)
        }
        Ending::ExitSignal | Ending::HarnessExit => harness(),
    }
}

/// Die of `signal`, as the harness did or as the run was cancelled, so this
/// process's caller sees the same wait status. **Without a core dump of this
/// process's own**: the harness's dump, if any, is the harness's, and a dump of
/// a supervisor that merely reported it would be a second, misleading one.
fn die_of(signal: i32) -> ! {
    // SAFETY: setrlimit with an initialised structure; a failure leaves the
    // limit as it was, and the death is reproduced anyway.
    unsafe {
        let none = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        libc::setrlimit(libc::RLIMIT_CORE, &none);
    }
    keyed_launch::reraise(signal)
}

/// `--ending-file`, checked before selection: a path that does not exist, in a
/// directory that does, named absolutely. The check is repeated by the
/// exclusive creation that writes the file, which is what refuses a path that
/// appeared in the meantime.
fn ending_path(path: &Path) -> Result<PathBuf, Refusal> {
    let unusable = |problem: String| {
        Refusal::new(
            "ending_file_unusable",
            Stage::Cli,
            EXIT_MALFORMED,
            format!("--ending-file {}: {problem}", path.display()),
            "name a path that does not exist, in a directory that does; harness-dispatch \
             creates the file once the harness is reaped and never replaces one",
        )
        .input("--ending-file")
    };
    let name = path
        .file_name()
        .ok_or_else(|| unusable("it names no file".to_owned()))?;
    let dir = match path.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir,
        _ => Path::new("."),
    };
    let dir = std::fs::canonicalize(dir)
        .map_err(|error| unusable(format!("its directory cannot be used: {error}")))?;
    if !dir.is_dir() {
        return Err(unusable("its directory is not a directory".to_owned()));
    }
    let file = dir.join(name);
    match std::fs::symlink_metadata(&file) {
        Ok(_) => Err(unusable("it already exists".to_owned())),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(file),
        Err(error) => Err(unusable(error.to_string())),
    }
}

/// `--exit-dir`, checked before selection: an existing directory, named
/// absolutely so the harness can reach it from any cwd. Dispatch neither
/// creates nor removes it.
fn exit_dir(dir: &Path) -> Result<PathBuf, Refusal> {
    let unusable = |problem: String| {
        Refusal::new(
            "exit_dir_unusable",
            Stage::Cli,
            EXIT_MALFORMED,
            format!("--exit-dir {}: {problem}", dir.display()),
            "name an existing directory the harness can write to, or omit --exit-dir for a \
             private per-run directory; harness-dispatch neither creates nor removes it",
        )
        .input("--exit-dir")
    };
    let canonical = std::fs::canonicalize(dir).map_err(|error| unusable(error.to_string()))?;
    if !canonical.is_dir() {
        return Err(unusable("it is not a directory".to_owned()));
    }
    Ok(canonical)
}

/// The run's exit channel and, unless the caller named one, the private
/// owner-only directory that holds it.
struct ExitChannel {
    channel: Channel,
    private: Option<tempfile::TempDir>,
}

impl ExitChannel {
    fn allocate(dir: Option<&Path>) -> Result<Self, Refusal> {
        let failed = |error: String| {
            Refusal::new(
                "exit_channel_unavailable",
                Stage::Exec,
                EXIT_WORKER,
                format!("cannot allocate the run's exit channel: {error}; nothing was recorded"),
                "make TMPDIR, or the directory --exit-dir names, writable, and run again",
            )
            .source("harness-dispatch")
        };
        let private = match dir {
            Some(_) => None,
            // Owner-only, stated rather than left to tempfile, which creates
            // a directory as the umask allows (measured 0755 on macOS).
            None => Some(
                tempfile::Builder::new()
                    .prefix("harness-dispatch-run-")
                    .permissions(std::fs::Permissions::from_mode(0o700))
                    .tempdir()
                    .map_err(|error| failed(error.to_string()))?,
            ),
        };
        let dir = dir.or(private.as_ref().map(tempfile::TempDir::path));
        let channel = Channel::allocate(dir.expect("a directory either way"))
            .map_err(|error| failed(error.to_string()))?;
        Ok(Self { channel, private })
    }

    /// Remove the channel and any private directory, before this process ends
    /// by a signal that would run no destructor.
    fn remove(self) -> Result<(), String> {
        let channel = self.channel.discard().map_err(|error| error.to_string());
        let private = match self.private {
            Some(dir) => {
                let path = dir.path().display().to_string();
                dir.close()
                    .map_err(|error| format!("cannot remove {path}: {error}"))
            }
            None => Ok(()),
        };
        channel.and(private)
    }
}

/// A cleanup failure is reported, and changes neither the ending nor the exit.
fn report_cleanup(cleanup: Result<(), String>) {
    if let Err(error) = cleanup {
        eprintln!("harness-dispatch: {error}");
    }
}

/// A signal seen at the linearization point: nothing is launched, and the
/// committed attempt is marked not executed where the store allows. A failed
/// append leaves it a handoff attempt whose execution is unknown, never a
/// success. The signal is re-raised once the refusal is reported. The handoff
/// notice already carried what the policy printed, so, as after a launch
/// failure, the refusal does not repeat it.
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

/// The harness could not be started: the failure is appended to the attempt
/// where the store allows, and a failed append leaves the attempt as it was,
/// never a success.
fn launch_failed(choice: &Choice, run_id: &RunId, error: &keyed_launch::LaunchError) -> Failure {
    let refusal = start_failed(choice, error);
    let detail = json!({
        "cause": "exec_error",
        "stage": Stage::Exec.as_str(),
        "code": refusal.code,
        "errno": error.raw_os_error(),
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

/// Append dispatch's end observation to the run, and write the same document
/// to the ending file once the harness's group is gone. A failure of either is
/// reported on stderr and changes neither the ending nor the exit; the file is
/// the caller's, so it does not wait on the store. Whether the observation was
/// recorded is the answer.
fn record_end(
    choice: &Choice,
    run_id: &RunId,
    ending: Ending,
    ended: &Ended,
    ending_file: Option<&Path>,
) -> bool {
    use std::os::unix::process::ExitStatusExt as _;
    let end = RunEnd {
        run_id,
        ending: ending.as_str(),
        code: ended.status.code(),
        signal: ended.status.signal().and_then(known_signal_name),
        duration: ended.elapsed,
    };
    let observation = match observation::end_observation(&end) {
        Ok(observation) => observation,
        Err(refusal) => {
            eprintln!(
                "harness-dispatch: run {run_id}: cannot build its end observation: {}",
                refusal.message
            );
            return false;
        }
    };
    if let Some(file) = ending_file.filter(|_| ended.group == Group::Gone) {
        if let Err(error) = write_ending_file(file, &observation) {
            eprintln!(
                "harness-dispatch: run {run_id}: cannot write the ending file {}: {error}",
                file.display()
            );
        }
    }
    append_end(choice, run_id, &observation)
}

/// The append, in its own short transaction under the commit's lock wait,
/// which migrates a version-1 store as an import would.
fn append_end(choice: &Choice, run_id: &RunId, observation: &Observation) -> bool {
    let document = observation.document.to_string();
    let append = NewObservation {
        observation_id: &observation.id,
        run_id,
        supersedes: None,
        confirms_execution: observation.confirms_execution,
        document: &document,
    };
    let why = match store::append_observation(&choice.state_dir, &append) {
        Ok(Appended::Recorded { .. } | Appended::AlreadyRecorded { .. }) => return true,
        Ok(Appended::RunMissing { .. }) => "the store no longer holds the run".to_owned(),
        Ok(Appended::ContradictsLaunchFailure { .. }) => {
            "the run holds a launch failure".to_owned()
        }
        Ok(
            Appended::Conflict { .. }
            | Appended::SupersedesUnknown { .. }
            | Appended::AlreadySuperseded { .. },
        ) => "the store holds a conflicting observation".to_owned(),
        Err(refusal) => refusal.message.clone(),
    };
    eprintln!("harness-dispatch: run {run_id}: its end observation was not recorded: {why}");
    false
}

/// The observation, one JSON line, in a file created exclusively and
/// owner-only.
fn write_ending_file(file: &Path, observation: &Observation) -> io::Result<()> {
    let mut created = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(file)?;
    writeln!(created, "{}", observation.document)?;
    created.sync_all()
}

/// The end, on stderr: one line naming the run, its ending, the harness's exit
/// or signal, the run's duration and whether the end observation was recorded.
/// With `--json`, one JSON object instead.
fn announce_end(run_id: &RunId, ending: Ending, ended: &Ended, recorded: bool, json: bool) {
    use std::os::unix::process::ExitStatusExt as _;
    let (code, signal) = (ended.status.code(), ended.status.signal());
    let millis = u64::try_from(ended.elapsed.as_millis()).unwrap_or(u64::MAX);
    let group = match ended.group {
        Group::Gone => "gone",
        Group::Present { .. } => "present",
    };
    if json {
        let notice = json!({
            "schemaVersion": 1,
            "end": {
                "runId": run_id.as_str(),
                "ending": ending.as_str(),
                "exitCode": code,
                "signal": signal.map(signal_name),
                "durationMs": millis,
                "group": group,
                "recorded": recorded,
            },
        });
        eprintln!("{notice}");
    } else {
        let how = match (code, signal) {
            (Some(code), _) => format!("exited {code}"),
            (None, Some(signal)) => format!("died of {}", signal_name(signal)),
            (None, None) => "ended".to_owned(),
        };
        eprintln!(
            "harness-dispatch: run {run_id} ended by {}: the harness {how} after {:.1} s; end \
             observation {}",
            ending.as_str(),
            ended.elapsed.as_secs_f64(),
            if recorded { "recorded" } else { "not recorded" }
        );
    }
}

/// A signal's conventional name, or its number where it has none here.
fn signal_name(signal: i32) -> String {
    known_signal_name(signal).map_or_else(|| format!("signal {signal}"), str::to_owned)
}

/// A signal's conventional `SIG` name, which is what an observation's `exit`
/// names a death by.
fn known_signal_name(signal: i32) -> Option<&'static str> {
    Some(match signal {
        libc::SIGHUP => "SIGHUP",
        libc::SIGINT => "SIGINT",
        libc::SIGQUIT => "SIGQUIT",
        libc::SIGILL => "SIGILL",
        libc::SIGTRAP => "SIGTRAP",
        libc::SIGABRT => "SIGABRT",
        libc::SIGBUS => "SIGBUS",
        libc::SIGFPE => "SIGFPE",
        libc::SIGKILL => "SIGKILL",
        libc::SIGUSR1 => "SIGUSR1",
        libc::SIGSEGV => "SIGSEGV",
        libc::SIGUSR2 => "SIGUSR2",
        libc::SIGPIPE => "SIGPIPE",
        libc::SIGALRM => "SIGALRM",
        libc::SIGTERM => "SIGTERM",
        libc::SIGXCPU => "SIGXCPU",
        libc::SIGXFSZ => "SIGXFSZ",
        libc::SIGVTALRM => "SIGVTALRM",
        libc::SIGPROF => "SIGPROF",
        libc::SIGSYS => "SIGSYS",
        _ => return None,
    })
}

/// The handoff, on stderr: whatever the policy printed, then one line naming
/// the command's labels and the file about to be started as its harness. With
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

/// The harness never started. `ENOENT` (the file, or its `#!` interpreter, is
/// gone) exits 127 like an unresolved program; anything else exits 126.
fn start_failed(choice: &Choice, error: &keyed_launch::LaunchError) -> Refusal {
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
    let cause = error.raw_os_error().map_or_else(
        || error.to_string(),
        |errno| io::Error::from_raw_os_error(errno).to_string(),
    );
    let source = choice.entry.display();
    Refusal::new(
        "exec_failed",
        Stage::Exec,
        exit,
        format!(
            "exec of {} for the program {:?} failed: {cause}",
            choice.executable.path, choice.command.program
        ),
        format!("{remedy}; harness-dispatch never runs another command instead"),
    )
    .source(source)
    .location("result.program")
}

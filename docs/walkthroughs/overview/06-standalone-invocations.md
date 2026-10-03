# One isolated invocation
<!-- book-page id="standalone-invocations" slice="isolated-invocation" order="6" -->
[Previous: What the call reaches](05-what-the-call-reaches.md) | [Contents](README.md)

<a id="one-isolated-invocation"></a>
## A kind without a task tree

`grove run` stages one kind's inputs and exports its declared outputs without
resolving a workspace or acquiring a driver lease. Grove owns that transfer
and the transcript. It launches `harness-dispatch run --confine` through
`keyed_launch::run_noninteractive`; dispatch selects, records, confines and
supervises the harness. Grove reads only dispatch's ending observation.

<a id="worked-standalone"></a>
## Worked example: turn staged changes into release notes

Run `grove run release-notes 'Draft notes from changes.txt' --input /work/release/changes.txt --output /work/release/notes.md --ui inline`, where the input
is regular and the destination absent. Grove copies the input into a private
`grove-run-…/work` directory and launches dispatch there with the kind and the
whole prompt. Dispatch selects outside confinement with the owner's policy
grants; the harness receives a fresh run ID and writes `notes.md` in the sandbox.
The prompt names the canonical sibling dispatch executable and asks the harness
to run its `exit` verb after checking the required output.

Dispatch stops and reaps the harness's group before writing the ending file
in Grove's separate control directory. Grove waits for dispatch, requires its
exit 0 and an observed `exit_signal` ending, and checks its own cancellation.
It reads outputs through the work-directory descriptor held before launch,
stages all copies beside their destinations, then publishes without replacement.
The result is a new `notes.md`, a completed status and a retained transcript.
An unacknowledged exit, failure or cancellation publishes nothing. A destination
race or cancellation during publication reports any already published prefix.

| Actor | Authority | Result |
|---|---|---|
| Caller | Kind, inputs, output destinations and runtime reads | One invocation |
| Owner's policy | Personal policy data outside the sandbox | Command or refusal |
| Dispatch | Recorded run, sandbox and harness process group | Ending after group cleanup |
| Confined harness | Staged cwd and its private run directory | Outputs and exit signal |
| Grove | Held work directory, transcript and destination paths | Validated publication |

<!-- fragment «standalone-invocation» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-317" parent="source-standalone" -->
<!-- insert «standalone-interface» -->
<!-- insert «standalone-staging» -->
<!-- insert «standalone-launch-context» -->
<!-- insert «standalone-supervision» -->
<!-- insert «standalone-artifact-checks» -->
<!-- insert «standalone-publication» -->
<!-- /fragment -->

<a id="standalone-interface"></a>
## The invocation inputs

`Args` carries the kind, one prompt source, transfer paths, runtime reads and
display choice. `run` locates the canonical sibling dispatch and names the
persistent logs. `CONTROL_ENV` removes enclosing Grove and dispatch controls
before selection, preserving the owner's other environment grants.

<!-- fragment «standalone-interface» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-67" parent="standalone-invocation" -->
````rust
//! One invocation of a kind, with no workspace or task-tree authority.
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{bail, ensure, Context, Result};
use keyed_launch::{Argv, End, Escalation, Group, NoninteractiveLaunch};
use serde_json::Value;

/// What selection must not inherit: the completion channel of a session this
/// invocation runs inside, lifecycle or standalone. The policy decides what
/// launches, and ends nothing.
const CONTROL_ENV: [&str; 5] = [
    "GROVE_SIGNAL_FILE",
    "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE",
    "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR",
];

#[derive(clap::Args)]
pub(crate) struct Args {
    /// Task kind, which the owner's harness-dispatch policy routes to a command.
    pub kind: String,
    /// Task instructions, delivered as one literal prompt argument.
    #[arg(
        required_unless_present = "prompt_file",
        conflicts_with = "prompt_file"
    )]
    pub prompt: Option<String>,
    /// Read task instructions from this UTF-8 file.
    #[arg(long, value_name = "FILE")]
    pub prompt_file: Option<PathBuf>,
    /// Copy a regular file into the invocation, using its basename (repeatable).
    #[arg(long, value_name = "FILE")]
    pub input: Vec<PathBuf>,
    /// Export a regular file of this basename to a new destination (repeatable).
    #[arg(long, value_name = "FILE")]
    pub output: Vec<PathBuf>,
    /// Grant read-only access to a runtime file, such as credentials (repeatable).
    #[arg(long, value_name = "FILE")]
    pub runtime_read: Vec<PathBuf>,
    /// Display inline, or in an existing supported mux; auto falls back inline.
    #[arg(long, value_enum, default_value = "auto")]
    pub ui: Ui,
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub(crate) enum Ui {
    Auto,
    Inline,
    Pane,
}

pub(crate) fn run(args: Args) -> Result<()> {
    let dispatch = crate::dispatch::locate()?.canonicalize()?;
    let owner_home =
        std::env::var_os("HOME").context("HOME is required to locate personal run logs")?;
    let logs = PathBuf::from(owner_home).join(".local/state/grove/runs");
    execute(args, &dispatch, &logs)
}

````
<!-- /fragment -->

<a id="standalone-staging"></a>
## Stage artifacts

`execute` validates the prompt and creates separate work and control
directories. Inputs are copied as regular files, names cannot collide, and every
output destination must be absent. The control directory remains outside the
harness's writable grants.

<!-- fragment «standalone-staging» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="68-118" parent="standalone-invocation" -->
````rust
fn execute(args: Args, dispatch: &Path, logs: &Path) -> Result<()> {
    let prompt = match (&args.prompt, &args.prompt_file) {
        (Some(prompt), None) => prompt.clone(),
        (None, Some(path)) => fs::read_to_string(path)
            .with_context(|| format!("reading prompt {}", path.display()))?,
        _ => bail!("supply one prompt or --prompt-file"),
    };
    ensure!(
        !prompt.trim().is_empty(),
        "the standalone prompt must not be empty"
    );
    let temporary = tempfile::Builder::new().prefix("grove-run-").tempdir()?;
    let root = temporary.path().canonicalize()?;
    let work = root.join("work");
    fs::create_dir(&work)?;
    let control = root.join("control");
    fs::create_dir(&control)?;
    let ending_file = control.join("ending.json");

    let mut names = HashSet::new();
    for source in &args.input {
        let name = basename(source)?;
        ensure!(
            names.insert(name.to_owned()),
            "duplicate artifact name: {}",
            name.to_string_lossy()
        );
        let mut input = regular_file(source)?;
        let mut copy = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(work.join(name))?;
        std::io::copy(&mut input, &mut copy)?;
    }
    let mut destinations = Vec::new();
    for destination in &args.output {
        let name = basename(destination)?;
        ensure!(
            names.insert(name.to_owned()),
            "duplicate artifact name: {}",
            name.to_string_lossy()
        );
        let parent = destination
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let destination = parent.canonicalize()?.join(name);
        ensure_absent(&destination)?;
        destinations.push(destination);
    }

````
<!-- /fragment -->

<a id="standalone-launch-context"></a>
## Build the dispatched invocation

The prompt names the exact dispatch `exit` command as the harness's final
action after output validation. The argv contains `run --confine`, the kind,
prompt, ending file and canonical runtime reads; it carries no task identity,
task file or selection parameter. Grove holds the work directory before launch,
opens the persistent transcript and retains display control.

<!-- fragment «standalone-launch-context» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="119-169" parent="standalone-invocation" -->
````rust
    let instructions = format!(
        "{prompt}\n\nStandalone invocation: work only on the staged files in the current directory. \
         There is no project, jj workspace, or Grove task tree here.\nInputs: {:?}\nRequired outputs: {:?}\n\
         After writing and checking every required output, run this exact command as your final action:\n\
         {} exit\n\
         If unable to complete, explain the failure and exit without signalling completion.\n",
        args.input.iter().map(|path| path.file_name()).collect::<Vec<_>>(),
        destinations.iter().map(|path| path.file_name()).collect::<Vec<_>>(),
        shell_word(dispatch.as_os_str())?
    );
    let mut dispatch_args = vec![
        "run".into(),
        "--confine".into(),
        format!("--kind={}", args.kind).into(),
        format!("--prompt={instructions}").into(),
        "--ending-file".into(),
        ending_file.as_os_str().to_owned(),
    ];
    for path in &args.runtime_read {
        // Resolve before moving into the staged working directory.
        dispatch_args.push("--runtime-read".into());
        dispatch_args.push(path.canonicalize()?.into_os_string());
    }
    let argv = Argv::new(dispatch.as_os_str().to_owned(), dispatch_args);
    // Hold the original directory, since the harness can rename paths inside
    // its scratch root. Output reads must never follow a substituted parent.
    let artifacts = File::open(&work)?;
    fs::create_dir_all(logs)?;
    let log_path = logs
        .join(
            root.file_name()
                .context("temporary directory has no name")?,
        )
        .with_extension("log");
    let status_path = log_path.with_extension("status");
    let log = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&log_path)?;
    let reader = File::open(&log_path)?;
    eprintln!(
        "grove run [{}]: starting; transcript {}",
        args.kind,
        log_path.display()
    );
    crate::run_display::open(args.ui, &args.kind, &log_path, &status_path)?;

    // Selection inherits the owner's grants. Dispatch scrubs its confined
    // child's environment after selection; enclosing control authority stops here.
    let scrub: Vec<&OsStr> = CONTROL_ENV.iter().map(OsStr::new).collect();
````
<!-- /fragment -->

<a id="standalone-supervision"></a>
## Supervise before accepting outputs

`NoninteractiveLaunch` carries no completion channel. The runner detaches
dispatch into its own session with EOF stdin and captured output, forwarding
cancellation and waiting through its kill grace. The relay sanitizes the log for
display. After reap, Grove refuses a surviving dispatch group, cancellation,
failed status, missing ending file or any ending other than observed
`exit_signal`. Dispatch writes an ending only after its harness group is gone.
The status file records either completed or failed, including selection refusal.

<!-- fragment «standalone-supervision» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="170-245" parent="standalone-invocation" -->
````rust
    let finished = AtomicBool::new(false);
    let outcome = std::thread::scope(|scope| {
        let relay = scope.spawn(|| crate::run_display::relay(reader, &finished));
        let result = keyed_launch::run_noninteractive(
            NoninteractiveLaunch {
                argv: &argv,
                scrub: &scrub,
                cwd: Some(&work),
                escalation: Escalation {
                    grace: Duration::from_secs(2),
                    kill_grace: Duration::from_secs(5),
                },
            },
            log,
        );
        finished.store(true, Ordering::Release);
        let displayed = relay
            .join()
            .map_err(|_| anyhow::anyhow!("transcript relay panicked"))?;
        displayed?;
        result.map_err(anyhow::Error::from)
    });
    let result = (|| {
        let ended = outcome?;
        // First, whatever the harness said: a survivor in its group could
        // still be writing what would be published.
        if let Group::Present { pgid } = ended.group {
            bail!(
                "members of the harness's process group {pgid} may have survived it; \
                 outputs were not published"
            );
        }
        ensure!(
            !matches!(ended.end, End::Interrupted { .. }),
            "standalone invocation was cancelled"
        );
        ensure!(
            keyed_launch::take_interrupt().is_none(),
            "standalone invocation was cancelled before publication"
        );
        ensure!(
            ended.status.success(),
            "dispatch failed ({}); inspect {}; outputs were not published",
            ended.status,
            log_path.display()
        );
        let report: Value = serde_json::from_slice(
            &fs::read(&ending_file)
                .context("dispatch reported no run ending; outputs were not published")?,
        )
        .context("reading dispatch's run ending")?;
        ensure!(
            report["schemaVersion"] == 1
                && report["source"] == "harness-dispatch"
                && report["measurements"]["ending"]["state"] == "observed"
                && report["measurements"]["ending"]["value"] == "exit_signal",
            "harness exited without an exit-signal ending; inspect {}; outputs were not published",
            log_path.display()
        );
        publish_outputs(&artifacts, &destinations)?;
        Ok(())
    })();
    let status = if result.is_ok() {
        "completed"
    } else {
        "failed"
    };
    fs::write(&status_path, status)?;
    eprintln!(
        "grove run [{}]: {status}; transcript {}",
        args.kind,
        log_path.display()
    );
    result
}

````
<!-- /fragment -->

<a id="standalone-artifact-checks"></a>
## Validate transfer paths

Basenames must name one artifact. Existing destinations are refused and input
opens use `O_NOFOLLOW` and `O_NONBLOCK`, rejecting symlinks and non-regular files
without hanging on a FIFO. These checks protect the transfer before dispatch
selects anything.

<!-- fragment «standalone-artifact-checks» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="246-281" parent="standalone-invocation" -->
````rust
fn basename(path: &Path) -> Result<&OsStr> {
    path.file_name()
        .filter(|name| !name.is_empty())
        .with_context(|| format!("artifact {} must name a file", path.display()))
}

fn ensure_absent(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("checking output {}", path.display())),
        Ok(_) => bail!(
            "output {} already exists; choose a new destination",
            path.display()
        ),
    }
}

fn regular_file(path: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .with_context(|| {
            format!(
                "opening regular artifact {} (symlinks are refused)",
                path.display()
            )
        })?;
    ensure!(
        file.metadata()?.is_file(),
        "artifact {} is not a regular file",
        path.display()
    );
    Ok(file)
}

````
<!-- /fragment -->

<a id="standalone-publication"></a>
## Publish without replacement

`publish_outputs` reads each regular output through the original directory
handle, validates and copies the whole set, then persists each staged copy
without replacing a destination. Cancellation is checked before each publish;
an error names the prefix already exported. `shell_word` quotes the canonical
exit command and the parent's display commands for literal shell use.

<!-- fragment «standalone-publication» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="282-317" parent="standalone-invocation" -->
````rust
fn publish_outputs(work: &File, destinations: &[PathBuf]) -> Result<()> {
    let mut staged = Vec::new();
    for destination in destinations {
        let mut source = keyed_launch::regular_file_at(work, basename(destination)?)
            .with_context(|| format!("reading staged output {}", destination.display()))?;
        let parent = destination.parent().context("output has no parent")?;
        let mut stage = tempfile::NamedTempFile::new_in(parent)?;
        std::io::copy(&mut source, &mut stage)?;
        stage.flush()?;
        staged.push((stage, destination));
    }
    let mut published = Vec::new();
    for (stage, destination) in staged {
        ensure!(
            keyed_launch::take_interrupt().is_none(),
            "publication cancelled; already published: {published:?}"
        );
        stage.persist_noclobber(destination).map_err(|error| {
            anyhow::anyhow!(
                "cannot publish {} without overwriting: {}; already published: {:?}",
                destination.display(),
                error.error,
                published
            )
        })?;
        published.push(destination);
    }
    Ok(())
}

pub(crate) fn shell_word(word: &OsStr) -> Result<String> {
    let word = word
        .to_str()
        .context("the display/completion command requires a UTF-8 executable path")?;
    Ok(format!("'{}'", word.replace('\'', "'\\''")))
}
````
<!-- /fragment -->

<a id="dispatch-lookup"></a>
## Find dispatch beside Grove

`dispatch::locate` resolves Grove's own executable to its real path and names
the `harness-dispatch` in the same directory. The two ship together, so that
sibling is the matching release. PATH and the working directory take no part,
and running Grove through a symlink does not move the lookup to the link's
directory. A sibling that is missing or not executable is refused with its path
and the repair, before anything is staged.

<!-- fragment «dispatch-lookup» owner="isolated-invocation" source="crates/grove/src/dispatch.rs" lines="1-26" parent="source-dispatch-lookup" -->
````rust
//! Where Grove finds `harness-dispatch`: beside its own executable.
//!
//! The two ship in one `bin/` directory, so the sibling of Grove's real path is
//! the matching release. PATH and the cwd take no part, and a symlink to Grove
//! does not move the lookup (`docs/specs/harness-selection-and-execution.md`,
//! *Grove integration*).
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use anyhow::{ensure, Context, Result};

pub(crate) fn locate() -> Result<PathBuf> {
    let grove = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .context("resolving grove's own executable path")?;
    let dispatch = grove.with_file_name("harness-dispatch");
    let executable = std::fs::metadata(&dispatch)
        .is_ok_and(|file| file.is_file() && file.permissions().mode() & 0o111 != 0);
    ensure!(
        executable,
        "grove launches through {}, which is missing or not executable. Install grove and \
         harness-dispatch together, from one release: they ship in the same bin directory.",
        dispatch.display()
    );
    Ok(dispatch)
}
````
<!-- /fragment -->

<!-- fragment «standalone-display» owner="isolated-invocation" source="crates/grove/src/run_display.rs" lines="1-119" parent="source-run-display" -->
<!-- insert «display-relay» -->
<!-- insert «display-pane» -->
<!-- insert «display-control-test» -->
<!-- /fragment -->

<a id="display-relay"></a>
## Stream a readable transcript

`relay` reads the parent-owned log until the runner marks the writer finished,
then drains the remaining bytes. `render` decodes text lossily and escapes control
characters other than newline and tab. The original log remains intact, while
the displayed transcript cannot replay escape sequences as terminal commands.
In the example, progress appears on the caller's stderr even though the child
has no inherited terminal stream.

<!-- fragment «display-relay» owner="isolated-invocation" source="crates/grove/src/run_display.rs" lines="1-43" parent="standalone-display" -->
````rust
//! Host-owned transcript display; the harness receives no display capability.
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::standalone::{shell_word, Ui};
use anyhow::{bail, Context, Result};

pub(crate) fn relay(mut log: File, finished: &AtomicBool) -> Result<()> {
    let mut buffer = [0; 8192];
    loop {
        let size = log.read(&mut buffer)?;
        if size > 0 {
            render(&buffer[..size], &mut std::io::stderr().lock())?;
        } else if finished.load(Ordering::Acquire) {
            // The writer has stopped. A final read covers bytes appended
            // between the preceding EOF and observing completion.
            let size = log.read(&mut buffer)?;
            render(&buffer[..size], &mut std::io::stderr().lock())?;
            if size == 0 {
                return Ok(());
            }
        } else {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

fn render(bytes: &[u8], writer: &mut impl Write) -> Result<()> {
    for character in String::from_utf8_lossy(bytes).chars() {
        if character.is_control() && character != '\n' && character != '\t' {
            write!(writer, "{}", character.escape_default())?;
        } else {
            write!(writer, "{character}")?;
        }
    }
    writer.flush()?;
    Ok(())
}

````
<!-- /fragment -->

<a id="display-pane"></a>
## Keep display control in the parent

`watch` is the hidden `run-log` viewer: it tails the same log and finishes when
the separate status file appears. `open` creates a tmux or Zellij pane from the
parent's environment. tmux receives `-d` and Zellij receives `--no-focus`, so
opening the viewer leaves focus on the caller. Inline mode skips that request;
auto mode falls back to inline when no supported connection succeeds, while an
explicit pane request returns a diagnostic. The confined harness receives
neither the mux variables nor a command channel back to that pane.

<!-- fragment «display-pane» owner="isolated-invocation" source="crates/grove/src/run_display.rs" lines="44-116" parent="standalone-display" -->
````rust
pub(crate) fn watch(log: &Path, status: &Path) -> Result<()> {
    let mut log = File::open(log).context("opening standalone transcript")?;
    let mut buffer = [0; 8192];
    loop {
        let size = log.read(&mut buffer)?;
        render(&buffer[..size], &mut std::io::stdout().lock())?;
        if size == 0 {
            if let Ok(status) = std::fs::read_to_string(status) {
                println!("\ngrove run: {status}");
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

pub(crate) fn open(ui: Ui, kind: &str, log: &Path, status: &Path) -> Result<()> {
    if matches!(ui, Ui::Inline) {
        return Ok(());
    }
    let executable = std::env::current_exe()?;
    let result = if std::env::var_os("TMUX").is_some() {
        let words = [
            executable.as_os_str(),
            std::ffi::OsStr::new("run-log"),
            log.as_os_str(),
            status.as_os_str(),
        ];
        let shell_command = words
            .into_iter()
            .map(shell_word)
            .collect::<Result<Vec<_>>>()?
            .join(" ");
        Command::new("tmux")
            .args([
                "split-window",
                "-d",
                "-P",
                "-F",
                "#{pane_id}",
                &shell_command,
            ])
            .output()
    } else if std::env::var_os("ZELLIJ").is_some() {
        Command::new("zellij")
            .args([
                "action",
                "new-pane",
                "--name",
                &format!("grove:{kind}"),
                "--no-focus",
                "--",
            ])
            .arg(&executable)
            .arg("run-log")
            .arg(log)
            .arg(status)
            .output()
    } else if matches!(ui, Ui::Pane) {
        bail!("--ui pane needs an existing tmux or Zellij session; use --ui inline here");
    } else {
        return Ok(());
    };
    match result {
        Ok(output) if output.status.success() => Ok(()),
        result if matches!(ui, Ui::Auto) => {
            eprintln!("grove run: pane unavailable ({result:?}); displaying inline");
            Ok(())
        }
        result => bail!("cannot open the requested task pane ({result:?}); use --ui inline or repair the mux connection"),
    }
}

````
<!-- /fragment -->

<a id="display-control-test"></a>
## Check display behavior without a real terminal

The test-only module declaration loads `tests/internal/run_display.rs`, outside
this book's source corpus. Its render test passes an OSC clipboard sequence and
checks that displayed text contains neither ESC nor BEL. The pane test supplies
fake mux commands in isolated subprocesses and checks quoting, focus-preserving
flags, inline behavior and auto-versus-explicit failure handling. These are
external evidence for the display boundary; only the declaration below belongs
to the reconstructed production file.

<!-- fragment «display-control-test» owner="isolated-invocation" source="crates/grove/src/run_display.rs" lines="117-119" parent="standalone-display" -->
````rust
#[cfg(test)]
#[path = "../tests/internal/run_display.rs"]
mod tests;
````
<!-- /fragment -->

[Previous: What the call reaches](05-what-the-call-reaches.md) | [Contents](README.md)

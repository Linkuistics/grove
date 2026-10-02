# One isolated invocation
<!-- book-page id="standalone-invocations" slice="isolated-invocation" order="6" -->
[Previous: What the call reaches](05-what-the-call-reaches.md) | [Contents](README.md)

<a id="one-isolated-invocation"></a>
## A kind without a task tree

`grove run` executes one kind in a fresh temporary working directory, with the
command the owner's harness-dispatch policy selects for that kind.
`standalone::execute` owns input staging, selection, completion checks and
output publication; `keyed_launch::run_confined` owns the process and the
mandatory operating-system filesystem boundary. The CLI reaches this path
before workspace resolution or lease acquisition. It reads no Grove
configuration, creates no task tree and receives no authority to complete a
surrounding session.

<a id="worked-standalone"></a>
## Worked example: turn staged changes into release notes

Assume the owner's policy routes `release-notes` to a command that takes the
prompt as an argument, `/work/release/changes.txt` is a regular file, and
`/work/release/notes.md` does not exist. Run
`grove run release-notes 'Draft release notes from changes.txt' --input /work/release/changes.txt --output /work/release/notes.md --ui inline`.
The command copies the input to a fresh `grove-run-…/work/changes.txt`. From
that directory it asks `harness-dispatch inspect` which command runs the kind,
passing the directory as both the `repo` and `worktree` parameters, and it
launches the command that comes back. The child writes `notes.md` there and
invokes the copied `grove-llm complete --done` helper. Its dedicated channel
appears under `grove-run-…/control`.

The supervisor ends and reaps the job before reading the token and exporting
anything. A valid `done` plus an accepted process outcome lets the parent read
`notes.md` through the directory descriptor held before launch. It first copies
all declared outputs to temporary files beside their destinations, then
publishes each without replacing an existing path. The observable result is a
new `/work/release/notes.md`, a `completed` status and a retained transcript in
`~/.local/state/grove/runs`. Missing completion, an invalid artifact, or
cancellation detected before publication returns an error without publishing.
During publication, a cancellation check or destination race can stop the
sequence after earlier outputs have appeared; that error names the published
prefix. Publication does not roll that prefix back.

The following table separates the authorities used by this operation.

| Actor | Authority | Result |
|---|---|---|
| Caller | Name the kind, supply files and runtime-read grants | One declared invocation |
| Owner's policy | Its own personal files, read outside the sandbox | The command to launch, or a refusal |
| Confined child | Scratch directory and its own completion channel | Staged output and acknowledgement |
| Parent supervisor | Held log, output directory and destination paths | Sanitized display, checked export and status |

The source below follows those boundaries. The temporary directory owns staged
work, while destination writes and terminal display remain parent operations.

<!-- fragment «standalone-invocation» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-394" parent="source-standalone" -->
<!-- insert «standalone-interface» -->
<!-- insert «standalone-staging» -->
<!-- insert «standalone-launch-context» -->
<!-- insert «standalone-supervision» -->
<!-- insert «standalone-selection» -->
<!-- insert «standalone-artifact-checks» -->
<!-- insert «standalone-publication» -->
<!-- /fragment -->

<a id="standalone-interface"></a>
## The invocation inputs

`Args` makes the kind, one prompt source, artifact paths, runtime grants and
display mode explicit. `run` locates `harness-dispatch`, takes the matching
completion helper from the same directory and names the persistent log
directory; it passes those paths to `execute` without resolving a repository.
In the example, `release-notes` is the kind the policy routes, and `--input` and
`--output` describe the transfer boundary. `CONTROL_ENV` names the two
completion channels that [selection](#standalone-selection) removes from the
environment it runs in.

<!-- fragment «standalone-interface» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-64" parent="standalone-invocation" -->
````rust
//! One invocation of a kind, with no workspace or task-tree authority.
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{bail, ensure, Context, Result};
use keyed_launch::{Argv, Channel, Confinement, End, Escalation, Launch};
use serde_json::Value;

/// What selection must not inherit: the completion channel of a session this
/// invocation runs inside, lifecycle or standalone. The policy decides what
/// launches, and ends nothing.
const CONTROL_ENV: [&str; 2] = ["GROVE_SIGNAL_FILE", "GROVE_RUN_SIGNAL_FILE"];

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
    let dispatch = crate::dispatch::locate()?;
    // The matching completion helper ships in the same directory.
    let helper = dispatch.with_file_name("grove-llm");
    let owner_home =
        std::env::var_os("HOME").context("HOME is required to locate personal run logs")?;
    let logs = PathBuf::from(owner_home).join(".local/state/grove/runs");
    execute(args, &dispatch, &helper, &logs)
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

<a id="standalone-staging"></a>
## Stage artifacts

`execute` reads exactly one nonempty prompt and creates the scratch root: a
working directory, a temporary directory, a control directory and a copy of the
completion helper. Each input
is opened as a regular file without following a final symlink and copied under
its basename. Input and output basenames share one uniqueness set. Destination
parents must resolve and destinations must be absent before launch. This makes
`changes.txt` available to the child while reserving `notes.md` for a new output.

<!-- fragment «standalone-staging» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="65-124" parent="standalone-invocation" -->
````rust
fn execute(args: Args, dispatch: &Path, helper: &Path, logs: &Path) -> Result<()> {
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
    fs::create_dir(root.join("tmp"))?;
    let control = root.join("control");
    fs::create_dir(&control)?;
    let bin = root.join("bin");
    fs::create_dir(&bin)?;
    let completion = bin.join("grove-llm");
    fs::copy(helper, &completion).with_context(|| {
        format!(
            "copying {}; install matching grove and grove-llm binaries",
            helper.display()
        )
    })?;

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
## Select, then supply only invocation context

The composed instructions name the staged artifacts and the copied completion
helper. `select` then asks the owner's policy for the command, from the staged
directory, and returns the `Argv` to launch. A selection that is refused,
cancelled or timed out ends `execute` here. No channel, log or pane exists yet,
so nothing is launched and nothing can be published. The parent
holds the original work directory and allocates a separate completion channel
before opening a private persistent log. Holding the directory prevents a child
from redirecting later output reads by replacing `work` with a symlink.

<!-- fragment «standalone-launch-context» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="125-160" parent="standalone-invocation" -->
````rust
    let instructions = format!(
        "{prompt}\n\nStandalone invocation: work only on the staged files in the current directory. \
         There is no project, jj workspace, or Grove task tree here.\nInputs: {:?}\nRequired outputs: {:?}\n\
         After writing and checking every required output, run this exact command as your final action:\n\
         {} complete --done\n\
         If unable to complete, explain the failure and exit without signalling completion.\n",
        args.input.iter().map(|path| path.file_name()).collect::<Vec<_>>(),
        destinations.iter().map(|path| path.file_name()).collect::<Vec<_>>(),
        shell_word(completion.as_os_str())?
    );
    let argv = select(dispatch, &args.kind, &instructions, &work)?;
    let channel = Channel::allocate(&control)?;
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

````
<!-- /fragment -->

<a id="standalone-supervision"></a>
## Supervise before accepting outputs

The parent removes all inherited environment names except the small allowlist,
then runs the mandatory confined launcher while a scoped thread relays the log.
The child receives `GROVE_RUN_SIGNAL_FILE`, independent of a surrounding
`GROVE_SIGNAL_FILE`. Completion requires no interruption, token `done`, and either
supervisor-driven termination or a successful natural exit. Only then does
publication run. The final status records publication success as well as child
completion, so an export failure cannot be displayed as completed.

<!-- fragment «standalone-supervision» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="161-234" parent="standalone-invocation" -->
````rust
    // Build a small inherited environment. In particular no GROVE_*, GIT_*,
    // JJ_*, terminal/mux sockets, loader injection, or parent harness identifiers.
    // TMPDIR/TMP/TEMP are overwritten by the confinement backend itself.
    let removed: Vec<OsString> = std::env::vars_os()
        .map(|(name, _)| name)
        .filter(|name| !inherited(name))
        .collect();
    let scrub: Vec<&OsStr> = removed.iter().map(OsString::as_os_str).collect();
    let finished = AtomicBool::new(false);
    let outcome = std::thread::scope(|scope| {
        let relay = scope.spawn(|| crate::run_display::relay(reader, &finished));
        let result = keyed_launch::run_confined(
            Launch {
                argv: &argv,
                channel: &channel,
                channel_var: "GROVE_RUN_SIGNAL_FILE",
                scrub: &scrub,
                cwd: Some(&work),
                escalation: Escalation {
                    grace: Duration::from_secs(2),
                    kill_grace: Duration::from_secs(5),
                },
            },
            log,
            &Confinement {
                writable: &root,
                runtime_read: &args.runtime_read,
            },
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
        ensure!(
            !matches!(ended.end, End::Interrupted { .. }),
            "standalone invocation was cancelled"
        );
        ensure!(
            keyed_launch::take_interrupt().is_none(),
            "standalone invocation was cancelled before publication"
        );
        ensure!(
            ended.token.as_ref().map(|token| token.as_str()) == Some("done"),
            "harness exited without valid completion ({}); inspect {}",
            ended.status,
            log_path.display()
        );
        ensure!(
            ended.end == End::Signalled || ended.status.success(),
            "harness failed after signalling completion ({}); outputs were not published",
            ended.status
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

<a id="standalone-selection"></a>
## Select outside the sandbox

The policy, the owner's settings and the record store are personal files, and
the confined harness must not read them. So the command is chosen before
confinement and outside it. `select` runs `harness-dispatch inspect --json` in
the staged directory with the kind, the whole composed prompt and three
parameters: `session_name` is `standalone:` and the kind, and `worktree` and
`repo` are both the staged directory. A standalone invocation has no task, so it
passes no task file and no task identity. The inspection inherits Grove's
environment without the two completion channels in `CONTROL_ENV`. An owner's
grants and bounds therefore apply, and no authority to end a surrounding session
travels with them. The inspection stays in Grove's process group and Grove waits
for it, so a signal to the group cancels the selection.

Inspection launches nothing and records no run. Its report carries a `command`
object: the `executable` its program resolved to, an absolute path, and the
`args` the policy returned. `select` builds the `Argv` from those two and from
nothing else. It does not look the program up again, because that lookup
depends on the directory and the PATH inspection ran with. `run_confined`
refuses a program that is not an absolute path, so the file inspection reported
is the file that runs.

`refusal` turns a selection that launched nothing into Grove's error. Dispatch
writes one JSON error on stderr, and this function reports its code, message and
remedy, with the policy's own code beside a refusal the policy made. Output it
cannot read that way is quoted beside the exit status.

<!-- fragment «standalone-selection» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="235-312" parent="standalone-invocation" -->
````rust
/// Ask the owner's policy which command runs this kind, before confinement
/// and outside it: the policy, the owner's settings and the record store are
/// personal files a confined harness must not read.
///
/// `harness-dispatch inspect` runs in the staged directory, in Grove's own
/// process group and environment, and launches nothing. What comes back is the
/// file the program resolved to and the arguments the policy returned, which the
/// runner launches as they are: nothing looks the program up a second time.
fn select(dispatch: &Path, kind: &str, prompt: &str, work: &Path) -> Result<Argv> {
    let staged = work
        .to_str()
        .context("the staged directory's path is not UTF-8, so no parameter can carry it")?;
    let mut inspect = Command::new(dispatch);
    inspect
        .arg("inspect")
        .arg("--json")
        .arg(format!("--kind={kind}"))
        .arg(format!("--prompt={prompt}"))
        .arg(format!("--param=session_name=standalone:{kind}"))
        .arg(format!("--param=worktree={staged}"))
        .arg(format!("--param=repo={staged}"))
        .current_dir(work)
        .stdin(Stdio::null());
    for name in CONTROL_ENV {
        inspect.env_remove(name);
    }
    let output = inspect
        .output()
        .with_context(|| format!("running {}", dispatch.display()))?;
    if !output.status.success() {
        bail!(
            "grove run cannot launch kind `{kind}`: {}\nNothing was launched or published.",
            refusal(&output.stderr, output.status)
        );
    }
    let report: Value = serde_json::from_slice(&output.stdout)
        .context("reading the selection harness-dispatch inspect reported")?;
    let command = &report["command"];
    let (Some(executable), Some(args)) = (
        command["executable"].as_str(),
        command["args"]
            .as_array()
            .and_then(|args| args.iter().map(Value::as_str).collect::<Option<Vec<_>>>()),
    ) else {
        bail!(
            "{} inspect reported no command to launch; install grove and harness-dispatch \
             from one release",
            dispatch.display()
        );
    };
    Ok(Argv::new(
        executable.into(),
        args.into_iter().map(OsString::from).collect(),
    ))
}

/// Dispatch's own account of a selection that launched nothing: its code,
/// message and remedy, with the policy's code beside a refusal the policy made.
fn refusal(stderr: &[u8], status: std::process::ExitStatus) -> String {
    let report = serde_json::from_slice::<Value>(stderr).unwrap_or_default();
    let error = &report["error"];
    let (Some(code), Some(message), Some(remedy)) = (
        error["code"].as_str(),
        error["message"].as_str(),
        error["remedy"].as_str(),
    ) else {
        return format!(
            "harness-dispatch inspect ended with {status}: {}",
            String::from_utf8_lossy(stderr).trim()
        );
    };
    let code = match error["policyCode"].as_str() {
        Some(policy) => format!("{code}: {policy}"),
        None => code.to_owned(),
    };
    format!("harness-dispatch refused the selection ({code}).\n  {message}\n  {remedy}")
}

````
<!-- /fragment -->

<a id="standalone-artifact-checks"></a>
## Validate transfer paths

`inherited` preserves basic identity, executable search and locale variables;
the confinement backend replaces temporary-directory variables. Artifact checks
require basenames, absent destinations and regular inputs. `O_NONBLOCK` prevents
a FIFO from hanging the staging reader, and `O_NOFOLLOW` refuses the final
symlink. These checks turn the example's caller paths into bounded file inputs,
not access grants to the caller's project.

<!-- fragment «standalone-artifact-checks» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="313-358" parent="standalone-invocation" -->
````rust
fn inherited(name: &OsStr) -> bool {
    let Some(name) = name.to_str() else {
        return false;
    };
    matches!(
        name,
        "HOME" | "USER" | "LOGNAME" | "PATH" | "LANG" | "TMPDIR" | "TMP" | "TEMP"
    ) || name.starts_with("LC_")
}

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

`publish_outputs` opens every output relative to the held work directory and
stages every readable regular file beside its destination before publishing any.
`persist_noclobber` preserves a destination created during the invocation.
Cancellation is checked between publications; failures report the published
prefix rather than implying a transaction rolled back. `shell_word` quotes the
helper path as one shell word in the instructions and parent-owned pane command;
it refuses paths that cannot be represented as UTF-8. The cases for this path
are in `crates/grove/tests/standalone.rs`, which drives the built binary under
real confinement. They are external evidence, outside this book's reconstructed
corpus.

<!-- fragment «standalone-publication» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="359-394" parent="standalone-invocation" -->
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

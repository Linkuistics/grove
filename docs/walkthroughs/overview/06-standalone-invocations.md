# One isolated invocation
<!-- book-page id="standalone-invocations" slice="isolated-invocation" order="6" -->
[Previous: What the call reaches](05-what-the-call-reaches.md) | [Contents](README.md)

<a id="one-isolated-invocation"></a>
## A kind without a task tree

`grove run` executes one kind from personal configuration in a fresh temporary
working directory. `standalone::execute` owns input staging, completion checks
and output publication; `keyed_launch::run_confined` owns the process and the
mandatory operating-system filesystem boundary. Dispatch reaches this path
before workspace resolution or lease acquisition. It reads no project overlay,
creates no task tree and receives no authority to complete a surrounding session.

<a id="worked-standalone"></a>
## Worked example: turn staged changes into release notes

Assume personal configuration routes `release-notes` to a command that accepts
`${prompt}`, `/work/release/changes.txt` is a regular file, and
`/work/release/notes.md` does not exist. Run
`grove run release-notes 'Draft release notes from changes.txt' --input /work/release/changes.txt --output /work/release/notes.md --ui inline`.
The command resolves the personal selection, copies the input to a fresh
`grove-run-…/work/changes.txt`, and gives the child that directory as both
`repo` and `worktree`. The child writes `notes.md` there and invokes the copied
`grove-llm complete --done` helper. Its dedicated channel appears under
`grove-run-…/control`.

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
| Caller | Select kind, supply files and runtime-read grants | One declared invocation |
| Confined child | Scratch directory and its own completion channel | Staged output and acknowledgement |
| Parent supervisor | Held log, output directory and destination paths | Sanitized display, checked export and status |

The source below follows those boundaries. The temporary directory owns staged
work, while destination writes and terminal display remain parent operations.

<!-- fragment «standalone-invocation» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-338" parent="source-standalone" -->
<!-- insert «standalone-interface» -->
<!-- insert «standalone-policy» -->
<!-- insert «standalone-launch-context» -->
<!-- insert «standalone-supervision» -->
<!-- insert «standalone-artifact-checks» -->
<!-- insert «standalone-publication» -->
<!-- /fragment -->

<a id="standalone-interface"></a>
## The invocation inputs

`Args` makes the kind, one prompt source, artifact paths, runtime grants and
display mode explicit. `run` locates the personal policy, matching completion
helper and persistent log directory; it passes those paths to `execute` without
resolving a repository. In the example, `release-notes` selects a configured
route, and `--input` and `--output` describe the transfer boundary.

<!-- fragment «standalone-interface» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-56" parent="standalone-invocation" -->
````rust
//! One configured invocation, with no workspace or task-tree authority.
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{bail, ensure, Context, Result};
use keyed_launch::{Catalog, Channel, Confinement, End, Escalation, Launch, Selection, Slot};

#[derive(clap::Args)]
pub(crate) struct Args {
    /// Task kind routed to a named command by personal Grove configuration.
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
    let source = grove_loop::TemplateSource::from_env()?;
    let helper = std::env::current_exe()?.with_file_name("grove-llm");
    let owner_home =
        std::env::var_os("HOME").context("HOME is required to locate personal run logs")?;
    let logs = PathBuf::from(owner_home).join(".local/state/grove/runs");
    execute(args, &source.personal_path(), &helper, &logs)
}

````
<!-- /fragment -->

<a id="standalone-policy"></a>
## Resolve policy and stage artifacts

`execute` reads exactly one nonempty prompt, resolves the personal catalog's
selection and requires the named kind before creating scratch state. Each input
is opened as a regular file without following a final symlink and copied under
its basename. Input and output basenames share one uniqueness set. Destination
parents must resolve and destinations must be absent before launch. This makes
`changes.txt` available to the child while reserving `notes.md` for a new output.

<!-- fragment «standalone-policy» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="57-121" parent="standalone-invocation" -->
````rust
pub(crate) fn execute(args: Args, policy: &Path, helper: &Path, logs: &Path) -> Result<()> {
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
    let catalog = Catalog::load(policy, None, grove_loop::session_config::vocabulary())?;
    let default_selection = Selection::default();
    let templates = catalog.resolve(catalog.primary_selection().unwrap_or(&default_selection))?;
    templates.require(&args.kind)?;

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
## Supply only invocation context

The composed instructions name the staged artifacts and the copied completion
helper. Expansion receives the scratch working directory for both path slots;
it never receives the caller's repository path through those slots. The parent
holds the original work directory and allocates a separate completion channel
before opening a private persistent log. Holding the directory prevents a child
from redirecting later output reads by replacing `work` with a symlink.

<!-- fragment «standalone-launch-context» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="122-178" parent="standalone-invocation" -->
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
    let session_name = format!("standalone:{}", args.kind);
    let argv = templates.expand(
        &args.kind,
        &[
            Slot {
                name: "prompt",
                value: instructions.as_ref(),
            },
            Slot {
                name: "session_name",
                value: session_name.as_ref(),
            },
            Slot {
                name: "worktree",
                value: work.as_os_str(),
            },
            Slot {
                name: "repo",
                value: work.as_os_str(),
            },
        ],
    )?;
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

<!-- fragment «standalone-supervision» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="179-252" parent="standalone-invocation" -->
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

<a id="standalone-artifact-checks"></a>
## Validate transfer paths

`inherited` preserves basic identity, executable search and locale variables;
the confinement backend replaces temporary-directory variables. Artifact checks
require basenames, absent destinations and regular inputs. `O_NONBLOCK` prevents
a FIFO from hanging the staging reader, and `O_NOFOLLOW` refuses the final
symlink. These checks turn the example's caller paths into bounded file inputs,
not access grants to the caller's project.

<!-- fragment «standalone-artifact-checks» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="253-298" parent="standalone-invocation" -->
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
it refuses paths that cannot be represented as UTF-8. The trailing test-only
module declaration loads `tests/internal/standalone.rs`; those fixtures remain
external evidence, outside this book's reconstructed corpus.

<!-- fragment «standalone-publication» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="299-338" parent="standalone-invocation" -->
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

#[cfg(test)]
#[path = "../tests/internal/standalone.rs"]
mod tests;
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

# One isolated invocation
<!-- book-page id="standalone-invocations" slice="isolated-invocation" order="6" -->
[Previous: What the call reaches](05-what-the-call-reaches.md) | [Contents](README.md)

<a id="one-isolated-invocation"></a>
## A kind without a task tree

`grove run` stages inputs and exports declared outputs without a workspace or
lease. Grove resolves/version-checks an independent installed Ravel, supplies
its prompt and exact canonical continue and force-stop commands, and launches it through
`ravel_job::launch` in the detached shape. Ravel selects and confines the
managed command, reaps its group, and returns only its public result on stdout.
Grove retains managed output on stderr in its transcript.

<a id="worked-standalone"></a>
## Worked example: turn staged changes into release notes

Run `grove run release-notes 'Draft notes from changes.txt' --input /work/release/changes.txt --output /work/release/notes.md --ui inline` with a regular input and
absent destination. Grove copies the input to its private work directory and
holds that directory open. Then Grove launches the once-selected Ravel with public
`start-session --confine` inputs. The policy selects outside confinement. The
harness receives its own ephemeral ending context and the exact
`end-session grove-continue` and `end-session grove-force-stop` commands in the prompt.

After Ravel reaps its managed group, Grove rejects cancellation or an
unsuccessful front status before reading the separate held,
unlinked result file. Only exact `grove-continue` authorises publication. Grove
reads output through the original directory descriptor, copies the whole set
beside its destinations and publishes without replacement. Refusal, a missing
or unsupported result, force-stop, or cancellation publishes nothing. A publication race
reports any already published prefix. A continue result published before a later managed
nonzero exit remains successful after cleanup, as Ravel's public contract states.

| Actor | Authority | Result |
|---|---|---|
| Caller | Kind, input/output paths and runtime reads | One invocation |
| Owner policy | Personal grants outside confinement | Command or refusal |
| Ravel | Managed group and ephemeral ending context | Public stdout after cleanup |
| Managed harness | Scratch writes and explicit runtime reads | Outputs and opaque result |
| Grove | Held output directory, result descriptor and destinations | Checked publication |

<!-- fragment «standalone-invocation» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-382" parent="source-standalone" -->
<!-- insert «standalone-interface» -->
<!-- insert «standalone-default-kind» -->
<!-- insert «standalone-staging» -->
<!-- insert «standalone-launch-context» -->
<!-- insert «standalone-supervision» -->
<!-- insert «standalone-artifact-checks» -->
<!-- insert «standalone-publication» -->
<!-- /fragment -->

<a id="standalone-interface"></a>
## The invocation inputs

`Args` carries an optional kind, one prompt source, transfer paths, runtime
reads and display choice. `STANDALONE` names the kind of a run without one
outside an active grove. `run` resolves Ravel once, checks its version and names the
persistent logs. The version probe scrubs enclosing authority too.

`ravel_job::CONTROL_ENV` lists ten names, and `ravel_job::launch` removes them
before selection. They are the enclosing Grove, dispatch and Ravel controls. The
list preserves the other environment grants of the owner. It also covers retired
completion channels, PID handles and inherited `GROVE_PROVIDER_EVIDENCE`, so a
nested caller cannot borrow the creator of its parent.

The real-process fixture
`standalone_scrubs_enclosing_ravel_and_retired_controls_before_selection` uses
process-entry probes for all ten controls and for the allowed Grove and owner
grants through actual selection. The fixture observes every name dirty
independently. Both callers scrub enclosing authority before delegation to
installed Ravel.

<!-- fragment «standalone-interface» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-67" parent="standalone-invocation" -->
````rust
//! One invocation of a kind, with no workspace or task-tree authority.
use std::collections::HashSet;
use std::ffi::{CString, OsStr};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{bail, ensure, Context, Result};
use grove_loop::{
    ravel_job::{self, LaunchEvent, Shape},
    session_command,
    session_result::{SessionDisposition, SessionResult},
    Sought,
};

/// The kind of a bare invocation outside an active grove. It needs no skill:
/// it exists so that the owner's policy can select a harness, a model and an
/// effort for standalone work.
pub(crate) const STANDALONE: &str = "standalone";

#[derive(clap::Args)]
pub(crate) struct Args {
    /// Task kind, which the owner's Ravel policy routes to a command. Without
    /// it, the kind is `standalone` outside an active grove, and the next live
    /// leaf's kind inside one.
    pub kind: Option<String>,
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
    /// No effect: Grove no longer confines a standalone run (repeatable).
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
    let ravel = crate::dispatch::locate()?;
    let owner_home =
        std::env::var_os("HOME").context("HOME is required to locate personal run logs")?;
    let logs = PathBuf::from(owner_home).join(".local/state/grove/runs");
    execute(args, &ravel, &logs)
}

````
<!-- /fragment -->

<a id="standalone-default-kind"></a>
## The kind of a run without one

`default_kind` answers which kind a run without a kind routes. The caller
omitted the kind, so the directory decides. The input is that directory, and
the output is a kind label or a refusal. A grove is active when the closest
ancestor that holds `.jj/` also holds `.grove/`. That is the tree that bare
`grove` drives from the same place. `Workspace::find_root` does the walk, so
no jj binary runs.

Outside an active grove, the kind is `standalone`. That kind requires no
skill. It lets the owner's policy select a harness, a model and an effort for
standalone work.

Inside an active grove, the kind is that of the next live
leaf, as `grove-llm pick` finds it. The run still gains no mandate and no tree
authority. The shared read of the tree ends when the function returns, before
selection. A grove with no live leaf, or a tree that the reader
refuses, gives no kind. The function then refuses, and the refusal names the
remedy: an explicit kind.

The three `a_bare_run_*` fixtures of `crates/grove/tests/standalone.rs`
observe each outcome through the fixture policy, which names the kind that it
refuses.

<!-- fragment «standalone-default-kind» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="68-102" parent="standalone-invocation" -->
````rust
/// The kind a bare `grove run` routes, from the directory it runs in.
///
/// A grove is active when the closest ancestor that holds `.jj/` also holds
/// `.grove/`: the tree that bare `grove` drives from the same directory. The
/// walk reads the filesystem only. Inside an active grove the kind is the next
/// live leaf's, which gives the run no mandate and no tree authority. Outside
/// one it is [`STANDALONE`].
///
/// # Errors
///
/// A tree that the reader refuses, or one with no live leaf. There is then no
/// kind to route, and nothing has launched.
pub(crate) fn default_kind(directory: &Path) -> Result<String> {
    let Some(worktree) = grove_loop::Workspace::find_root(directory)? else {
        return Ok(STANDALONE.to_owned());
    };
    let grove = worktree.join(".grove");
    let tree = match grove_loop::read(&worktree)
        .with_context(|| format!("reading the active grove {}", grove.display()))?
    {
        grove_loop::Reading::Vacant => return Ok(STANDALONE.to_owned()),
        grove_loop::Reading::Tree(tree) => tree,
    };
    match grove_loop::verbs::kind(&tree, None)
        .with_context(|| format!("reading the next leaf of {}", grove.display()))?
    {
        Sought::Match(kind) => Ok(kind.label().to_owned()),
        Sought::Nothing => bail!(
            "the active grove {} has no live leaf, so a bare `grove run` has no kind to route; \
             give an explicit kind, such as `grove run standalone --prompt-file FILE`",
            grove.display()
        ),
    }
}

````
<!-- /fragment -->

<a id="standalone-staging"></a>
## Stage artifacts

`execute` checks the prompt and creates separate work and control
directories. `execute` copies inputs as regular files. Their names cannot
collide, and every output destination must be absent. The control directory remains outside the
harness's writable grants.

<!-- fragment «standalone-staging» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="103-166" parent="standalone-invocation" -->
````rust
fn execute(args: Args, ravel: &Path, logs: &Path) -> Result<()> {
    let kind = match &args.kind {
        Some(kind) => kind.clone(),
        None => default_kind(&std::env::current_dir().context("getting cwd")?)?,
    };
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
    // The flag stays so that existing commands keep working. Grove sends
    // nothing for it and does not check the file.
    if !args.runtime_read.is_empty() {
        eprintln!(
            "grove run: --runtime-read has no effect; Grove no longer confines a standalone run, \
             so the harness sandbox alone decides what the harness reads"
        );
    }
    let temporary = tempfile::Builder::new().prefix("grove-run-").tempdir()?;
    let root = temporary.path().canonicalize()?;
    let work = root.join("work");
    fs::create_dir(&work)?;
    let control = root.join("control");
    fs::create_dir(&control)?;
    let capture = SessionResult::allocate()?;
    let stdout = capture.stdout()?;

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
## Build the public Ravel invocation

The prompt names exact canonical commands: `end-session grove-continue` after
checking every output, and `end-session grove-force-stop` when unable to complete.
It explains that continue publishes checked outputs and force-stop reports
failure with no publication. The shared argv contains `start-session --confine`,
the kind, literal prompt, writable exit directory and canonical runtime reads.
It carries no task identity, task file or selection parameter. Grove holds the work
directory before it launches Ravel, opens the persistent transcript and retains
display control.

The argv grants no skill directory. On the directory route of Ravel 2.0.0, Ravel
itself grants the captured skills read-only inside the sandbox. Native skill trees
under HOME stay outside every grant. A harness that cannot read its guidance there
force-stops. Grove then publishes nothing and launches no second, unconfined run.

<!-- fragment «standalone-launch-context» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="167-210" parent="standalone-invocation" -->
````rust
    let instructions = format!(
        "{prompt}\n\nStandalone invocation: work only on the staged files in the current directory. \
         There is no project, jj workspace, or Grove task tree here.\nInputs: {:?}\nRequired outputs: {:?}\n\
         After writing and checking every required output, run this exact command as your final action:\n\
         {}\n\
         Grove interprets grove-continue as successful completion and publishes the checked outputs.\n\
         To stop without completing, explain the failure and run:\n\
         {}\n\
         Grove interprets grove-force-stop as failure and publishes no outputs. \
         This command must be your final action.\n",
        args.input.iter().map(|path| path.file_name()).collect::<Vec<_>>(),
        destinations.iter().map(|path| path.file_name()).collect::<Vec<_>>(),
        session_command::end_session(ravel)?,
        session_command::force_stop(ravel)?
    );
    let argv = session_command::standalone(ravel, &kind, &instructions, &control);
    // Hold the original directory, since the harness can rename paths inside
    // the working directory. Output reads must never follow a substituted parent.
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
        kind,
        log_path.display()
    );
    crate::run_display::open(args.ui, &kind, &log_path, &status_path)?;

    // Selection inherits the owner's grants. The shared function scrubs the
    // loop control variables, so enclosing control authority stops here. The
    // detached shape also removes the repository selectors and the multiplexer
    // variables. Otherwise the harness receives the caller's environment.
````
<!-- /fragment -->

<a id="standalone-supervision"></a>
## Supervise before accepting outputs

`ravel_job::launch` carries no completion channel. In the detached shape it
launches Ravel in a new session. Standard input is end of file, and standard error
goes to the transcript. The held stdout file receives the result.

The function polls Ravel and sends each caught INT, TERM or HUP to `ravel` alone. It waits
until Ravel exits, however long that takes. The relay sanitises the log for
display.

Grove passes the `Argv` of `session_command` to the function unchanged. It also
passes an observer that does nothing, because no lease takes the notices of a
standalone invocation.

After the wait, Grove refuses cancellation or an unsuccessful Ravel status.
Grove sends no KILL and probes no process group. A successful Ravel status is the
only evidence of cleanup. Two checks find a cancellation. The first reads the
signal that the job caught. The second reads a signal that arrived after the reap.

Then Grove reads the held stdout result. Only exact `grove-continue` authorises
output publication. Exact `grove-force-stop` returns an error before the output
check or publication, even for valid staged files.

The status file records either completed or failed, including selection refusal.
Relay failure prevents publication too, and it retains the log even for an
acknowledged run. An uncatchable Grove death can orphan its detached Ravel
and harness. That dead publisher exports nothing.

<!-- fragment «standalone-supervision» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="211-271" parent="standalone-invocation" -->
````rust
    let finished = AtomicBool::new(false);
    let outcome = std::thread::scope(|scope| {
        let relay = scope.spawn(|| crate::run_display::relay(reader, &finished));
        // No lease takes the notices of a standalone invocation.
        let mut observer = |_: LaunchEvent| {};
        let result = ravel_job::launch(
            &argv,
            &work,
            stdout,
            &mut observer,
            Shape::Detached { transcript: log },
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
            ended.interrupted.is_none(),
            "standalone invocation was cancelled"
        );
        ensure!(
            ravel_job::take_interrupt().is_none(),
            "standalone invocation was cancelled before publication"
        );
        ensure!(
            ended.status.success(),
            "Ravel failed ({}); inspect {}; outputs were not published",
            ended.status,
            log_path.display()
        );
        match capture
            .read()
            .context("Ravel returned no supported Grove result; outputs were not published")?
        {
            SessionDisposition::Continue => {}
            SessionDisposition::ForceStop => {
                bail!("Grove force-stop requested; outputs were not published");
            }
        }
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
        kind,
        log_path.display()
    );
    result
}

````
<!-- /fragment -->

<a id="standalone-artifact-checks"></a>
## Check transfer paths

Basenames must name one artifact. Grove refuses existing destinations. Input
opens use `O_NOFOLLOW` and `O_NONBLOCK`, so they reject symlinks and non-regular
files without hanging on a FIFO. These checks protect the transfer before Ravel
selects anything.

`regular_file_at` reads a staged output. It is a private function of this
module. It opens the name with `openat` under the held work directory, so a
renamed parent cannot redirect the read. It refuses a name of more than one
component, a symlink and anything but a regular file.

<!-- fragment «standalone-artifact-checks» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="272-346" parent="standalone-invocation" -->
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

/// Open `name` under the held `directory` as a regular file, following no link.
///
/// The harness can rename paths inside the working directory, so a read by
/// path could follow a substituted parent. `openat` on the held descriptor cannot.
/// `O_NONBLOCK` keeps a FIFO from hanging the open.
fn regular_file_at(directory: &File, name: &OsStr) -> std::io::Result<File> {
    let mut components = Path::new(name).components();
    if !matches!(components.next(), Some(std::path::Component::Normal(_)))
        || components.next().is_some()
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "expected one artifact basename",
        ));
    }
    let name = CString::new(name.as_encoded_bytes())?;
    // SAFETY: directory is held open, name is NUL-terminated, and the returned
    // fresh descriptor is immediately owned.
    let fd = unsafe {
        libc::openat(
            directory.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: `openat` returned this descriptor and nothing else owns it.
    let file = unsafe { File::from_raw_fd(fd) };
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "artifact is not a regular file",
        ));
    }
    Ok(file)
}

````
<!-- /fragment -->

<a id="standalone-publication"></a>
## Publish without replacement

`publish_outputs` reads each regular output through the original directory
handle, checks and copies the whole set, then persists each staged copy
without replacing a destination. `ravel_job::take_interrupt` checks for
cancellation before each publish.
An error names the prefix already exported. `shell_word` quotes the canonical
exit command and the parent's display commands for literal shell use.

<!-- fragment «standalone-publication» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="347-382" parent="standalone-invocation" -->
````rust
fn publish_outputs(work: &File, destinations: &[PathBuf]) -> Result<()> {
    let mut staged = Vec::new();
    for destination in destinations {
        let mut source = regular_file_at(work, basename(destination)?)
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
            ravel_job::take_interrupt().is_none(),
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
## Delegate to independent Ravel resolution

The dispatch module delegates both actual callers to the PATH/version resolver.
Grove and Ravel may live in different versioned prefixes. The resolver uses no sibling,
cwd or repository fallback. No staged dead-code suppression remains.

<!-- fragment «dispatch-lookup» owner="isolated-invocation" source="crates/grove/src/dispatch.rs" lines="1-5" parent="source-dispatch-lookup" -->
````rust
//! Grove resolves an independent installed Ravel through absolute PATH entries.
//! The selected canonical executable and supported CLI version serve both callers.
mod ravel;

pub(crate) use ravel::locate;
````
<!-- /fragment -->

<a id="ravel-resolution"></a>
## Resolve the independent installed Ravel

The active `dispatch::ravel::locate` reads the PATH of the invoking process. Its
`locate_on_path` helper examines only absolute directories, skips candidates
that are not executable regular files, and canonicalises the first candidate
once. It returns that path only after `check_version` accepts its public
`--version` response. An incompatible first executable refuses. A later
installation cannot silently replace it.

Missing Ravel reports an install command. A malformed, unsuccessful or unsupported probe reports the admitted
majors with their configuration files and the keg-only `ravel@2` directory. That
remedy tells the owner to put the matching installation first on PATH, and it
recommends no shared upgrade.

The compatibility check admits stable `ravel 1.x` from 1.1.0 and stable
`ravel 2.x` from 2.0.0. It refuses 1.0.x, because Ravel 1.1.0 corrected the
failure paths of the managed job. It refuses every other major, such as 3.x.

The check parses the major, the minor and the patch, and it requires canonical
numbers for the minor and the patch. For major 1 it treats a minor of exactly `0`
as below the floor. For major 2 it admits every minor. It checks that stdout is
UTF-8 and rejects extra output, prereleases, other CLI lines and a failed status.

The result is admission only. The PATH, the configuration file and the settings
select the route. The probe removes the loop control variables with
`ravel_job::scrub`, because a version probe is not an admitted session.

The caller ADR, `docs/adr/dispatch-supervises-the-harness.md`, records the PATH trust
cost. This probe executes the program that the owner selected. Neither version
output nor canonicalisation authenticates or pins its bytes. Both actual
callers consume the selected executable. The old sibling locator is gone.

The inline tests drive executable decoys and a subprocess with its own cwd and PATH.
They do not mutate the environment of the test runner. One test builds Homebrew's
layout, with an unversioned link into a 1.1.0 keg and an `opt/ravel@2` link into a
2.0.0 keg. The default order selects 1.1.0. With the `ravel@2` directory first, the
resolver returns the 2.0.0 keg path, and the ending command quotes that path.

The installed-pair test copies an actual build through the shared `testing/ravel.rs`
fixture into a temporary independent prefix. `GROVE_TEST_RAVEL_RUNTIME` names its
runtime. A 1.x build is a front and a worker, and the test routes a fake shell agent
through a `policy.ts` that imports `ravel/sdk`. A 2.x build is one executable, and the
test routes the same agent through a `config.rhai` with `SCHEMA_VERSION` 4. With only
that prefix on the child's PATH, the session's noise stays on stderr and stdout
contains exactly its supplied result.

`task ravel:resolution RAVEL=/absolute/path/to/ravel` runs this installed case as
well as the ordinary resolver tests. The ignored subprocess role runs only as a
child of its cwd-decoy test.

`task ravel:fixtures` combines this check with the shared-result runner checks.
Pass `RAVEL_RUNTIME=2.0` for a 2.x build. On a 1.x
build, its real-worker controls remove the copied worker. On a 2.x build, they
check that the copy holds no worker. Both inject inherited authority into a
re-executed entry probe. Clean runs use the isolated copy without host Bun or Node
and without a Grove sibling installation.

<!-- fragment «ravel-resolution» owner="isolated-invocation" source="crates/grove/src/dispatch/ravel.rs" lines="1-434" parent="source-ravel-resolution" -->
````rust
//! PATH and CLI compatibility at Grove's independent Ravel boundary.
use std::ffi::OsStr;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{bail, ensure, Context, Result};

pub(crate) fn locate() -> Result<PathBuf> {
    locate_on_path(std::env::var_os("PATH").as_deref())
}

fn locate_on_path(path: Option<&OsStr>) -> Result<PathBuf> {
    if let Some(path) = path {
        for directory in std::env::split_paths(path).filter(|directory| directory.is_absolute()) {
            let candidate = directory.join("ravel");
            if !std::fs::metadata(&candidate)
                .is_ok_and(|file| file.is_file() && file.permissions().mode() & 0o111 != 0)
            {
                continue;
            }
            let executable = std::fs::canonicalize(&candidate)
                .with_context(|| format!("resolving Ravel executable {}", candidate.display()))?;
            check_version(&executable)?;
            return Ok(executable);
        }
    }
    bail!(
        "Ravel is missing from absolute PATH directories. Install it with \
           `brew install linkuistics/taps/ravel` and add its bin directory to PATH."
    )
}

fn check_version(executable: &std::path::Path) -> Result<()> {
    let remedy = || {
        format!(
            "{} must report an admitted stable Ravel CLI version. Grove admits \
             1.x from 1.1.0, which reads `~/.config/ravel/policy.ts`, and 2.x from \
             2.0.0, which reads `~/.config/ravel/config.rhai`. Put the bin directory \
             of the installation that matches your configuration first on the PATH \
             of this invocation. Homebrew installs 2.x as the keg-only `ravel@2`, \
             in `$(brew --prefix ravel@2)/bin`. Do not upgrade a shared installation \
             that other sessions or configurations still use.",
            executable.display()
        )
    };
    let mut probe = Command::new(executable);
    // A version probe is not an admitted session and must inherit no ending
    // or Grove tree authority, even when PATH selects an executable wrapper.
    grove_loop::ravel_job::scrub(&mut probe);
    let output = probe.arg("--version").output().with_context(remedy)?;
    // Ravel's public version flag identifies the two-verb CLI, not its worker protocol.
    // https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md
    // 1.1.0 corrected the failure paths of the managed job that Grove relies on,
    // so 1.0.x is refused. A canonical minor below the floor is exactly "0".
    // https://github.com/Linkuistics/Ravel/blob/v1.1.0/CHANGELOG.md
    // 2.0.0 keeps the public session CLI and replaces policy.ts with config.rhai.
    // Admission selects no route: the PATH, configuration and settings do.
    // docs/specs/codex-skill-provisioning.md#launch-and-compatibility
    let supported = std::str::from_utf8(&output.stdout)
        .ok()
        .and_then(|text| text.trim().strip_prefix("ravel "))
        .is_some_and(|version| {
            let mut parts = version.split('.');
            let (Some(major), Some(minor), Some(patch), None) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            else {
                return false;
            };
            let admitted = match major {
                "1" => minor != "0",
                "2" => true,
                _ => false,
            };
            admitted
                && [minor, patch].into_iter().all(|part| {
                    !part.is_empty()
                        && part.bytes().all(|byte| byte.is_ascii_digit())
                        && (part == "0" || !part.starts_with('0'))
                })
        });
    ensure!(
        output.status.success() && supported,
        "{} Received status {} and version {:?}.",
        remedy(),
        output.status,
        String::from_utf8_lossy(&output.stdout)
    );
    Ok(())
}

#[cfg(test)]
#[path = "../../../../testing/support.rs"]
mod support;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::{symlink, PermissionsExt};
    use std::path::Path;

    use tempfile::TempDir;

    use super::*;

    fn executable(dir: &Path, body: &str) -> PathBuf {
        fs::create_dir_all(dir).unwrap();
        let file = dir.join("ravel");
        fs::write(&file, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();
        file
    }

    fn path(dirs: &[&Path]) -> std::ffi::OsString {
        std::env::join_paths(dirs).unwrap()
    }

    #[test]
    fn resolves_first_executable_to_its_real_path() {
        let temp = TempDir::new().unwrap();
        let real = executable(&temp.path().join("ravel-1.2.7/bin"), "echo 'ravel 1.2.7'");
        let links = temp.path().join("links");
        fs::create_dir(&links).unwrap();
        symlink(&real, links.join("ravel")).unwrap();
        let later = temp.path().join("later");
        executable(&later, "echo 'ravel 1.1.0'");
        assert_eq!(
            locate_on_path(Some(&path(&[&links, &later]))).unwrap(),
            fs::canonicalize(real).unwrap()
        );
    }

    #[test]
    fn skips_nonexecutables_and_directories() {
        let temp = TempDir::new().unwrap();
        let nonexec = temp.path().join("nonexec");
        let file = executable(&nonexec, "exit 99");
        fs::set_permissions(file, fs::Permissions::from_mode(0o644)).unwrap();
        let directory = temp.path().join("directory");
        fs::create_dir_all(directory.join("ravel")).unwrap();
        let valid = temp.path().join("valid");
        let file = executable(&valid, "echo 'ravel 1.1.0'");
        assert_eq!(
            locate_on_path(Some(&path(&[&nonexec, &directory, &valid]))).unwrap(),
            fs::canonicalize(file).unwrap()
        );
    }

    #[test]
    fn missing_absolute_path_candidate_has_install_remedy() {
        let temp = TempDir::new().unwrap();
        for value in [
            None,
            Some(OsStr::new("")),
            Some(OsStr::new(".:bin")),
            Some(temp.path().as_os_str()),
        ] {
            let error = locate_on_path(value).unwrap_err().to_string();
            assert!(
                error.contains("brew install linkuistics/taps/ravel"),
                "{error}"
            );
        }
    }

    #[test]
    fn rejects_the_selected_incompatible_version_without_falling_through() {
        let temp = TempDir::new().unwrap();
        let selected = temp.path().join("selected");
        let later = temp.path().join("later");
        executable(&later, "echo 'ravel 1.1.0'");
        for version in [
            "ravel 0.1.0",
            "ravel 0.9.9",
            "ravel 1.0.0",
            "ravel 1.0.9",
            "ravel 1.1.0-beta",
            "ravel 1.1",
            "ravel 1",
            "ravel 01.1.0",
            "ravel 1.01.0",
            "ravel 1.1.00",
            "ravel 1.1.0.0",
            "ravel 2.0.0-rc.1",
            "ravel 2.0.0-beta",
            "ravel 2.0",
            "ravel 2",
            "ravel 02.0.0",
            "ravel 2.00.0",
            "ravel 2.0.01",
            "ravel 2.0.0.0",
            "ravel 2.0.0 extra",
            "ravel 3.0.0",
            "ravel 3.1.0",
            "ravel 10.0.0",
            "ravel 0.0.0",
            "harness-dispatch 1.1.0",
            "harness-dispatch 2.0.0",
            "ravel 1.1.0 extra",
        ] {
            executable(&selected, &format!("echo '{version}'"));
            let error = locate_on_path(Some(&path(&[&selected, &later])))
                .unwrap_err()
                .to_string();
            assert_admission_remedy(&error);
            assert!(error.contains(&selected.display().to_string()), "{error}");
        }
    }

    /// The remedy names both admitted majors with their configuration files and
    /// the keg-only 2.x prefix, and recommends no undirected shared upgrade.
    fn assert_admission_remedy(error: &str) {
        for part in [
            "1.x from 1.1.0",
            "policy.ts",
            "2.x from 2.0.0",
            "config.rhai",
            "ravel@2",
            "first on the PATH",
        ] {
            assert!(error.contains(part), "missing {part:?}: {error}");
        }
        assert!(!error.contains("brew upgrade"), "{error}");
    }

    #[test]
    fn accepts_stable_versions_from_the_1_1_0_and_2_0_0_floors() {
        let temp = TempDir::new().unwrap();
        for version in [
            "ravel 1.1.0",
            "ravel 1.1.9",
            "ravel 1.12.3",
            "ravel 2.0.0",
            "ravel 2.0.7",
            "ravel 2.3.0",
            "ravel 2.10.12",
        ] {
            let file = executable(temp.path(), &format!("echo '{version}'"));
            assert_eq!(
                locate_on_path(Some(temp.path().as_os_str())).unwrap(),
                fs::canonicalize(file).unwrap(),
                "{version}"
            );
        }
    }

    #[test]
    fn unsuccessful_version_probe_is_not_a_supported_installation() {
        let temp = TempDir::new().unwrap();
        for version in ["ravel 1.1.0", "ravel 2.0.0"] {
            executable(temp.path(), &format!("echo '{version}'; exit 1"));
            let error = locate_on_path(Some(temp.path().as_os_str()))
                .unwrap_err()
                .to_string();
            assert_admission_remedy(&error);
        }
    }

    /// Homebrew's layout until promotion: the unversioned `ravel` links into the
    /// 1.1.0 keg, and the keg-only `ravel@2` has only its `opt` link. The first
    /// PATH entry selects the installation, and the resolver returns its real
    /// keg path, which the ending commands quote.
    #[test]
    fn a_keg_only_ravel_2_first_on_path_is_selected_by_its_keg_path() {
        let temp = TempDir::new().unwrap();
        let cellar = temp.path().join("Cellar");
        let one = executable(&cellar.join("ravel/1.1.0/bin"), "echo 'ravel 1.1.0'");
        let two = executable(&cellar.join("ravel@2/2.0.0/bin"), "echo 'ravel 2.0.0'");
        let default_bin = temp.path().join("bin");
        fs::create_dir(&default_bin).unwrap();
        symlink(&one, default_bin.join("ravel")).unwrap();
        let opt = temp.path().join("opt");
        fs::create_dir(&opt).unwrap();
        symlink(cellar.join("ravel@2/2.0.0"), opt.join("ravel@2")).unwrap();
        let keg_bin = opt.join("ravel@2/bin");

        assert_eq!(
            locate_on_path(Some(&path(&[&default_bin, &keg_bin]))).unwrap(),
            fs::canonicalize(&one).unwrap()
        );
        let selected = locate_on_path(Some(&path(&[&keg_bin, &default_bin]))).unwrap();
        assert_eq!(selected, fs::canonicalize(&two).unwrap());
        assert_eq!(
            grove_loop::session_command::end_session(&selected).unwrap(),
            format!("'{}' end-session grove-continue", selected.display())
        );
    }

    #[test]
    fn cwd_decoy_cannot_override_absolute_path() {
        let temp = TempDir::new().unwrap();
        executable(temp.path(), "exit 99");
        executable(&temp.path().join("bin"), "exit 99");
        let selected = executable(&temp.path().join("selected"), "echo 'ravel 1.1.0'");
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "dispatch::ravel::tests::resolver_process_fixture",
                "--ignored",
            ])
            .current_dir(temp.path())
            .env(
                "PATH",
                path(&[
                    Path::new(""),
                    Path::new("."),
                    Path::new("bin"),
                    selected.parent().unwrap(),
                ]),
            )
            .env(
                "GROVE_TEST_EXPECTED_RAVEL",
                fs::canonicalize(selected).unwrap(),
            )
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn version_probe_scrubs_every_enclosing_control() {
        let temp = TempDir::new().unwrap();
        let checks: String = super::support::LOOP_CONTROL_ENV.iter().map(|name| format!(
            "if [ \"${{{name}-}}\" = /inert-enclosing-control ]; then printf 'version-observed:{name}\\n' >&2; exit 93; fi\n"
        )).collect();
        let front = executable(temp.path(), &format!("{checks}echo 'ravel 1.1.0'"));
        for name in super::support::LOOP_CONTROL_ENV {
            let output = Command::new(&front)
                .env_clear()
                .env(name, "/inert-enclosing-control")
                .arg("--version")
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(93), "{name}: {output:?}");
            assert!(
                String::from_utf8_lossy(&output.stderr).contains(name),
                "{output:?}"
            );
        }
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "dispatch::ravel::tests::resolver_process_fixture",
                "--ignored",
            ])
            .env("PATH", temp.path())
            .env(
                "GROVE_TEST_EXPECTED_RAVEL",
                fs::canonicalize(front).unwrap(),
            );
        for name in super::support::LOOP_CONTROL_ENV {
            command.env(name, "/inert-enclosing-control");
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{output:?}");
    }

    #[test]
    #[ignore = "subprocess role driven by cwd_decoy_cannot_override_absolute_path"]
    fn resolver_process_fixture() {
        assert_eq!(
            locate().unwrap(),
            PathBuf::from(std::env::var_os("GROVE_TEST_EXPECTED_RAVEL").unwrap())
        );
    }

    #[test]
    #[ignore = "requires installed Ravel: task ravel:resolution RAVEL=/absolute/path/to/ravel"]
    fn installed_front_and_worker_return_only_the_session_result() {
        let temp = TempDir::new().unwrap();
        let prefix = temp.path().join("independent-ravel-1.1.0");
        let source = std::env::var_os("GROVE_TEST_RAVEL").expect("GROVE_TEST_RAVEL is required");
        let pair = super::support::ravel::InstalledRavel::copy_from(Path::new(&source), &prefix);
        let selected = locate_on_path(Some(pair.bin().as_os_str())).unwrap();
        assert_eq!(selected, pair.executable());
        let runtime = pair.runtime();
        let policy = temp.path().join(runtime.config_file());
        let selected_json = serde_json::to_string(&selected).unwrap();
        let source = match runtime {
            super::support::ravel::Runtime::V1_1 => format!(
                r#"import {{ definePolicy }} from 'ravel/sdk';
export const policy = definePolicy({{
  schemaVersion: 2, version: 'grove-resolver-fixture',
  select: () => ({{ status: 'selected', program: '/bin/sh',
    args: ['-c', 'printf session-noise; "$1" end-session grove-continue', 'fixture', {selected_json}],
    provider: 'fixture', model: 'fixture', effort: 'low', reason: 'installed pair' }})
}});
"#
            ),
            super::support::ravel::Runtime::V2_0 => {
                use super::support::ravel::{Arg, Route};
                let mut route = Route::new(
                    "grove-resolver-fixture",
                    "/bin/sh",
                    vec![
                        Arg::Text("-c"),
                        Arg::Text(r#"printf session-noise; "$1" end-session grove-continue"#),
                        Arg::Text("fixture"),
                        Arg::Text(selected.to_str().unwrap()),
                    ],
                );
                (route.model, route.effort, route.reason) = ("fixture", "low", "installed pair");
                route.source(runtime)
            }
        };
        fs::write(&policy, source).unwrap();
        let mut command = pair.command();
        command
            .args([
                "start-session",
                "--kind",
                "fixture",
                "--prompt",
                "unchanged",
                "--config",
            ])
            .arg(&policy)
            .env("HOME", temp.path())
            .current_dir(temp.path());
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, b"grove-continue");
        assert!(String::from_utf8_lossy(&output.stderr).contains("session-noise"));
    }
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
opening the viewer leaves focus on the caller. Inline mode skips that request.
Auto mode falls back to inline when no supported connection succeeds. An
explicit pane request returns a diagnostic instead.

The confined harness receives
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
## Check display behaviour without a real terminal

The test-only module declaration loads `tests/internal/run_display.rs`, outside
this book's source corpus. Its render test passes an OSC clipboard sequence and
checks that displayed text contains neither ESC nor BEL. The pane test supplies
fake mux commands in isolated subprocesses and checks quoting, focus-preserving
flags, inline behaviour and auto-versus-explicit failure handling. These are
external evidence for the display boundary. Only the declaration below belongs
to the reconstructed production file.

<!-- fragment «display-control-test» owner="isolated-invocation" source="crates/grove/src/run_display.rs" lines="117-119" parent="standalone-display" -->
````rust
#[cfg(test)]
#[path = "../tests/internal/run_display.rs"]
mod tests;
````
<!-- /fragment -->

[Previous: What the call reaches](05-what-the-call-reaches.md) | [Contents](README.md)

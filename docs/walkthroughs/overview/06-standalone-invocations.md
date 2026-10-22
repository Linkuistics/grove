# One isolated invocation
<!-- book-page id="standalone-invocations" slice="isolated-invocation" order="6" -->
[Previous: What the call reaches](05-what-the-call-reaches.md) | [Contents](README.md)

<a id="one-isolated-invocation"></a>
## A kind without a task tree

`grove run` stages inputs and exports declared outputs without a workspace or
lease. Grove resolves/version-checks an independent installed Ravel, supplies
its prompt and exact canonical continue and force-stop commands, and starts it through
`ravel_job::launch` in the detached shape. Ravel selects and confines the
managed command, reaps its group, and returns only its public result on stdout.
Grove retains managed output on stderr in its transcript.

<a id="worked-standalone"></a>
## Worked example: turn staged changes into release notes

Run `grove run release-notes 'Draft notes from changes.txt' --input /work/release/changes.txt --output /work/release/notes.md --ui inline` with a regular input and
absent destination. Grove copies the input to its private work directory and
holds that directory open. Then Grove starts the once-selected Ravel with public
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

<!-- fragment «standalone-invocation» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-332" parent="source-standalone" -->
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
display choice. `run` resolves Ravel once, checks its version and names the
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

<!-- fragment «standalone-interface» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-59" parent="standalone-invocation" -->
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
};

#[derive(clap::Args)]
pub(crate) struct Args {
    /// Task kind, which the owner's Ravel policy routes to a command.
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
    let ravel = crate::dispatch::locate()?;
    let owner_home =
        std::env::var_os("HOME").context("HOME is required to locate personal run logs")?;
    let logs = PathBuf::from(owner_home).join(".local/state/grove/runs");
    execute(args, &ravel, &logs)
}

````
<!-- /fragment -->

<a id="standalone-staging"></a>
## Stage artifacts

`execute` checks the prompt and creates separate work and control
directories. Inputs are copied as regular files, names cannot collide, and every
output destination must be absent. The control directory remains outside the
harness's writable grants.

<!-- fragment «standalone-staging» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="60-111" parent="standalone-invocation" -->
````rust
fn execute(args: Args, ravel: &Path, logs: &Path) -> Result<()> {
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
directory before it starts Ravel, opens the persistent transcript and retains
display control.

<!-- fragment «standalone-launch-context» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="112-160" parent="standalone-invocation" -->
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
    let argv = session_command::confined(
        ravel,
        &args.kind,
        &instructions,
        &control,
        &args.runtime_read,
    )?;
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

    // Selection inherits the owner's grants. The shared function scrubs the
    // loop control variables, so enclosing control authority stops here. Ravel
    // scrubs its confined child's environment after selection.
````
<!-- /fragment -->

<a id="standalone-supervision"></a>
## Supervise before accepting outputs

`ravel_job::launch` carries no completion channel. In the detached shape it
starts Ravel in a new session. Standard input is end of file, and standard error
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

<!-- fragment «standalone-supervision» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="161-221" parent="standalone-invocation" -->
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
        args.kind,
        log_path.display()
    );
    result
}

````
<!-- /fragment -->

<a id="standalone-artifact-checks"></a>
## Check transfer paths

Basenames must name one artifact. Existing destinations are refused and input
opens use `O_NOFOLLOW` and `O_NONBLOCK`, rejecting symlinks and non-regular files
without hanging on a FIFO. These checks protect the transfer before Ravel
selects anything.

`regular_file_at` reads a staged output. It is a private function of this
module. It opens the name with `openat` under the held work directory, so a
renamed parent cannot redirect the read. It refuses a name of more than one
component, a symlink and anything but a regular file.

<!-- fragment «standalone-artifact-checks» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="222-296" parent="standalone-invocation" -->
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
/// The harness can rename paths inside its scratch root, so a read by path could
/// follow a substituted parent. `openat` on the held descriptor cannot.
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

<!-- fragment «standalone-publication» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="297-332" parent="standalone-invocation" -->
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
Grove and Ravel may live in different versioned prefixes. No sibling, cwd or
repository fallback is used, and no staged dead-code suppression remains.

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
installation cannot silently replace it. Missing Ravel reports an install
command, while a malformed, unsuccessful or unsupported probe reports an
upgrade command.

The compatibility check accepts stable `ravel 1.x` from 1.1.0 and refuses
1.0.x. Ravel 1.1.0 corrected the failure paths of the managed job. The check
parses the minor and the patch as canonical numbers, and it treats a minor of
exactly `0` as below the floor. It checks that stdout is UTF-8 and rejects extra
output, prereleases, other CLI lines and a failed status. The probe removes the
loop control variables with `ravel_job::scrub`, because a version probe is not an
admitted session.

The caller ADR, `docs/adr/dispatch-supervises-the-harness.md`, records the PATH trust
cost. This probe executes the program that the owner selected. Neither version
output nor canonicalisation authenticates or pins its bytes. Both actual
callers consume the selected executable. The old sibling locator is gone.

The inline tests drive executable decoys and a subprocess with its own cwd and PATH.
They do not mutate the environment of the test runner. The installed-pair test copies
an actual front and worker through the shared `testing/ravel.rs` fixture into a
temporary independent prefix and routes a fake
shell agent through `ravel/sdk`. With only that prefix on the child's PATH, the
session's noise stays on stderr and stdout contains exactly its supplied result.

`task ravel:resolution RAVEL=/absolute/path/to/ravel` runs this installed case as
well as the ordinary resolver tests. The ignored subprocess role runs only as a
child of its cwd-decoy test. `task ravel:fixtures` combines this check with the
shared-result runner checks. Its real-worker controls remove the copied worker
and inject inherited authority into a re-executed entry probe. Clean runs use
the isolated pair without host Bun or Node and without a Grove sibling installation.

<!-- fragment «ravel-resolution» owner="isolated-invocation" source="crates/grove/src/dispatch/ravel.rs" lines="1-330" parent="source-ravel-resolution" -->
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
            "{} must report a stable Ravel 1.x CLI version from 1.1.0. \
         Run `brew upgrade ravel` and select that installation on PATH.",
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
    let supported = std::str::from_utf8(&output.stdout)
        .ok()
        .and_then(|text| text.trim().strip_prefix("ravel 1."))
        .is_some_and(|rest| {
            let mut parts = rest.split('.');
            let (Some(minor), Some(patch), None) = (parts.next(), parts.next(), parts.next())
            else {
                return false;
            };
            minor != "0"
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
            "ravel 2.0.0",
            "ravel 1.1.0-beta",
            "ravel 1.1",
            "ravel 1",
            "ravel 01.1.0",
            "ravel 1.01.0",
            "ravel 1.1.00",
            "ravel 1.1.0.0",
            "harness-dispatch 1.1.0",
            "ravel 1.1.0 extra",
        ] {
            executable(&selected, &format!("echo '{version}'"));
            let error = locate_on_path(Some(&path(&[&selected, &later])))
                .unwrap_err()
                .to_string();
            assert!(error.contains("brew upgrade ravel"), "{version}: {error}");
            assert!(error.contains(&selected.display().to_string()), "{error}");
        }
    }

    #[test]
    fn accepts_stable_versions_from_the_1_1_0_floor() {
        let temp = TempDir::new().unwrap();
        for version in ["ravel 1.1.0", "ravel 1.1.9", "ravel 1.12.3"] {
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
        executable(temp.path(), "echo 'ravel 1.1.0'; exit 1");
        let error = locate_on_path(Some(temp.path().as_os_str()))
            .unwrap_err()
            .to_string();
        assert!(error.contains("brew upgrade ravel"), "{error}");
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
        let policy = temp.path().join("policy.ts");
        let selected_json = serde_json::to_string(&selected).unwrap();
        fs::write(&policy, format!(r#"import {{ definePolicy }} from 'ravel/sdk';
export const policy = definePolicy({{
  schemaVersion: 2, version: 'grove-resolver-fixture',
  select: () => ({{ status: 'selected', program: '/bin/sh',
    args: ['-c', 'printf session-noise; "$1" end-session grove-continue', 'fixture', {selected_json}],
    provider: 'fixture', model: 'fixture', effort: 'low', reason: 'installed pair' }})
}});
"#)).unwrap();
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

# One isolated invocation
<!-- book-page id="standalone-invocations" slice="isolated-invocation" order="6" -->
[Previous: What the call reaches](05-what-the-call-reaches.md) | [Contents](README.md)

<a id="one-isolated-invocation"></a>
## A kind without a task tree

`grove run` stages inputs and exports declared outputs without a workspace or
lease. Grove resolves/version-checks an independent installed Ravel, supplies
its prompt and exact canonical continue and force-stop commands, and supervises through
`keyed_launch::run_noninteractive_with_stdout`. Ravel selects and confines the
managed command, reaps its group, and returns only its public result on stdout.
Grove retains managed output on stderr in its transcript.

<a id="worked-standalone"></a>
## Worked example: turn staged changes into release notes

Run `grove run release-notes 'Draft notes from changes.txt' --input /work/release/changes.txt --output /work/release/notes.md --ui inline` with a regular input and
absent destination. Grove copies the input to its private work directory,
holds that directory open, and launches the once-selected Ravel with public
`start-session --confine` inputs. The policy selects outside confinement; the
harness receives its own ephemeral ending context and the exact
`end-session grove-continue` and `end-session grove-force-stop` commands in the prompt.

After Ravel reaps its managed group, Grove rejects a surviving supervisor group,
cancellation or unsuccessful front status before reading the separate held,
unlinked result file. Only exact `grove-continue` authorizes publication. Grove
reads output through the original directory descriptor, copies the whole set
beside its destinations and publishes without replacement. Refusal, a missing
or unsupported result, force-stop, or cancellation publishes nothing; a publication race
reports any already published prefix. A continue result published before a later managed
nonzero exit remains successful after cleanup, as Ravel's public contract states.

| Actor | Authority | Result |
|---|---|---|
| Caller | Kind, input/output paths and runtime reads | One invocation |
| Owner policy | Personal grants outside confinement | Command or refusal |
| Ravel | Managed group and ephemeral ending context | Public stdout after cleanup |
| Managed harness | Scratch writes and explicit runtime reads | Outputs and opaque result |
| Grove | Held output directory, result descriptor and destinations | Checked publication |

<!-- fragment «standalone-invocation» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-321" parent="source-standalone" -->
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
display choice. `run` resolves/version-checks Ravel once and names the
persistent logs. The version probe scrubs enclosing authority as well as launch. `CONTROL_ENV` removes enclosing Grove, dispatch and Ravel
controls before selection, preserving the owner's other environment grants.
It also removes retired completion channels, PID handles and inherited
`GROVE_PROVIDER_EVIDENCE`, so a nested caller cannot borrow its parent's creator. The real-process
fixture `standalone_scrubs_enclosing_ravel_and_retired_controls_before_selection`
uses process-entry probes for all ten controls and allowed Grove/owner grants
through actual selection. Every name is observed dirty independently; both
callers scrub enclosing authority before delegation to installed Ravel.

<!-- fragment «standalone-interface» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="1-75" parent="standalone-invocation" -->
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
use grove_loop::{
    session_command,
    session_result::{SessionDisposition, SessionResult},
};
use keyed_launch::{End, Escalation, Group, NoninteractiveLaunch};

/// What selection must not inherit: the completion channel of a session this
/// invocation runs inside, lifecycle or standalone. The policy decides what
/// launches, and ends nothing.
pub(crate) const CONTROL_ENV: [&str; 10] = [
    "GROVE_SIGNAL_FILE",
    "GROVE_RUN_SIGNAL_FILE",
    "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE",
    "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR",
    "RAVEL_EXIT_FILE",
    "GROVE_PROVIDER_EVIDENCE",
    "GROVE_HARNESS_PID",
    "GROVE_CLAUDE_PID",
];

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

`execute` validates the prompt and creates separate work and control
directories. Inputs are copied as regular files, names cannot collide, and every
output destination must be absent. The control directory remains outside the
harness's writable grants.

<!-- fragment «standalone-staging» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="76-127" parent="standalone-invocation" -->
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
the kind, literal prompt, writable exit directory and canonical runtime reads; it carries no task identity,
task file or selection parameter. Grove holds the work directory before launch,
opens the persistent transcript and retains display control.

<!-- fragment «standalone-launch-context» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="128-176" parent="standalone-invocation" -->
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

    // Selection inherits the owner's grants. Ravel scrubs its confined
    // child's environment after selection; enclosing control authority stops here.
    let scrub: Vec<&OsStr> = CONTROL_ENV.iter().map(OsStr::new).collect();
````
<!-- /fragment -->

<a id="standalone-supervision"></a>
## Supervise before accepting outputs

`NoninteractiveLaunch` carries no completion channel. The runner detaches
Ravel into its own session with EOF stdin and captured output, forwarding
cancellation and waiting through its kill grace. The relay sanitizes the log for
display. After reap, Grove refuses a surviving supervisor group, cancellation
or unsuccessful Ravel status, then reads the held stdout result. Only exact
`grove-continue` authorizes output publication. Exact `grove-force-stop` returns
an error before output validation or publication, even for valid staged files.
The status file records either completed or failed, including selection refusal.
Relay failure prevents publication too, retaining the log even for an
acknowledged run. An uncatchable Grove death can orphan its detached Ravel
and harness; that dead publisher exports nothing.

<!-- fragment «standalone-supervision» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="177-249" parent="standalone-invocation" -->
````rust
    let finished = AtomicBool::new(false);
    let outcome = std::thread::scope(|scope| {
        let relay = scope.spawn(|| crate::run_display::relay(reader, &finished));
        let result = keyed_launch::run_noninteractive_with_stdout(
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
            stdout,
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
                "members of the Ravel process group {pgid} may have survived it; \
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
## Validate transfer paths

Basenames must name one artifact. Existing destinations are refused and input
opens use `O_NOFOLLOW` and `O_NONBLOCK`, rejecting symlinks and non-regular files
without hanging on a FIFO. These checks protect the transfer before Ravel
selects anything.

<!-- fragment «standalone-artifact-checks» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="250-285" parent="standalone-invocation" -->
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

<!-- fragment «standalone-publication» owner="isolated-invocation" source="crates/grove/src/standalone.rs" lines="286-321" parent="standalone-invocation" -->
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

The active `dispatch::ravel::locate` reads the launching process's PATH. Its
`locate_on_path` helper examines only absolute directories, skips candidates
that are not executable regular files, and canonicalizes the first candidate
once. It returns that path only after `check_version` accepts its public
`--version` response. An incompatible first executable refuses; a later
installation cannot silently replace it. Missing Ravel reports an install
command, while a malformed, unsuccessful or unsupported probe reports an
upgrade command.

The compatibility check accepts stable `ravel 0.1.x`, starting with the delivered
0.1.0 two-verb contract. It validates stdout as UTF-8 and rejects extra output,
prereleases, other CLI lines and a failed status. The
caller ADR, `docs/adr/dispatch-supervises-the-harness.md`, records the PATH trust
cost: this probe executes the owner's selected program, and neither version
output nor canonicalization authenticates or pins its bytes. Both actual
callers consume the selected executable; the old sibling locator is gone.

The inline tests drive executable decoys and a subprocess with its own cwd/PATH;
they do not mutate the test runner's environment. The installed-pair test copies
an actual front and worker through the shared `testing/ravel.rs` fixture into a
temporary independent prefix and routes a fake
shell agent through `ravel/sdk`. With only that prefix on the child's PATH, the
session's noise stays on stderr and stdout contains exactly its supplied result.
`task ravel:resolution RAVEL=/absolute/path/to/ravel` runs this installed case as
well as the ordinary resolver tests; the ignored subprocess role runs only as a
child of its cwd-decoy test. `task ravel:fixtures` combines this check with the
shared-result runner checks. Its real-worker controls remove the copied worker
and inject inherited authority into a re-executed entry probe; clean runs use
the isolated pair without host Bun/Node or a Grove sibling installation.

<!-- fragment «ravel-resolution» owner="isolated-invocation" source="crates/grove/src/dispatch/ravel.rs" lines="1-302" parent="source-ravel-resolution" -->
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
            "{} must report a supported stable Ravel 0.1.x CLI version. \
         Run `brew upgrade ravel` and select that installation on PATH.",
            executable.display()
        )
    };
    let mut probe = Command::new(executable);
    // A version probe is not an admitted session and must inherit no ending
    // or Grove tree authority, even when PATH selects an executable wrapper.
    for name in crate::standalone::CONTROL_ENV {
        probe.env_remove(name);
    }
    let output = probe.arg("--version").output().with_context(remedy)?;
    // Ravel's public version flag identifies the two-verb CLI, not its worker protocol.
    // https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/session-cli.md
    let supported = std::str::from_utf8(&output.stdout)
        .ok()
        .and_then(|text| text.trim().strip_prefix("ravel 0.1."))
        .is_some_and(|patch| {
            !patch.is_empty()
                && patch.bytes().all(|byte| byte.is_ascii_digit())
                && (patch == "0" || !patch.starts_with('0'))
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
        let real = executable(&temp.path().join("ravel-0.1.7/bin"), "echo 'ravel 0.1.7'");
        let links = temp.path().join("links");
        fs::create_dir(&links).unwrap();
        symlink(&real, links.join("ravel")).unwrap();
        let later = temp.path().join("later");
        executable(&later, "echo 'ravel 0.1.0'");
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
        let file = executable(&valid, "echo 'ravel 0.1.0'");
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
        executable(&later, "echo 'ravel 0.1.0'");
        for version in [
            "ravel 0.0.9",
            "ravel 0.2.0",
            "ravel 1.0.0",
            "ravel 0.1.0-beta",
            "ravel 0.1",
            "ravel 00.1.0",
            "harness-dispatch 0.1.0",
            "ravel 0.1.0 extra",
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
    fn unsuccessful_version_probe_is_not_a_supported_installation() {
        let temp = TempDir::new().unwrap();
        executable(temp.path(), "echo 'ravel 0.1.0'; exit 1");
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
        let selected = executable(&temp.path().join("selected"), "echo 'ravel 0.1.0'");
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
        let front = executable(temp.path(), &format!("{checks}echo 'ravel 0.1.0'"));
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
        let prefix = temp.path().join("independent-ravel-0.1.0");
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

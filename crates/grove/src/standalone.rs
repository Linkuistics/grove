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

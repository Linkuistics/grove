//! The caller's data inputs, read and checked once, before any policy runs
//! (`docs/specs/harness-selection-and-execution.md`, *Command interface*).
//!
//! The kind, the task file and the task identity are caller data: the task file
//! supplies neither kind nor identity, and nothing is recovered from a file
//! name. The prompt is read once, kept byte for byte, and never sent to the
//! policy worker; it only ever fills the candidate's `prompt` argument.
//! Terminal stdin is never read.

use std::ffi::OsString;
use std::fs;
use std::io::Read as _;
use std::os::fd::AsRawFd as _;
use std::path::{Path, PathBuf};

use crate::cli::SelectionArgs;
use crate::refusal::{Refusal, Stage, EXIT_MALFORMED, EXIT_REFUSED};

/// The prompt's fixed bound, in bytes. Platform argv limits can refuse a
/// smaller one at exec.
pub const PROMPT_LIMIT: usize = 1024 * 1024;

/// The task identity's bound, in bytes of UTF-8.
pub const TASK_ID_LIMIT: usize = 1024;

#[derive(Debug)]
pub struct Inputs {
    pub kind: String,
    /// The original cwd: relative inputs resolve against it, and the harness
    /// runs in it.
    pub cwd: PathBuf,
    /// Absolute, joined to the original cwd; not read, and not required to
    /// exist.
    pub task_file: Option<String>,
    pub task_id: Option<String>,
    pub prompt: Option<Prompt>,
}

#[derive(Debug)]
pub struct Prompt {
    pub text: String,
    pub source: PromptSource,
}

#[derive(Debug)]
pub enum PromptSource {
    Argument,
    /// The file it was read from, joined to the original cwd.
    File(PathBuf),
}

/// Whether the command needs a prompt: `run` launches with one, and `inspect`
/// shows a placeholder where it would go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromptRequirement {
    Required,
    Optional,
}

impl Inputs {
    pub fn read(args: &SelectionArgs, requirement: PromptRequirement) -> Result<Inputs, Refusal> {
        args.refuse_unsupported()?;
        let cwd = std::env::current_dir().map_err(|error| {
            Refusal::new(
                "cwd_unavailable",
                Stage::Authority,
                EXIT_REFUSED,
                format!("the current directory cannot be read: {error}"),
                "run harness-dispatch from an existing, readable directory",
            )
        })?;
        let task_file = args
            .task_file
            .as_deref()
            .map(|path| task_file(path, &cwd))
            .transpose()?;
        let task_id = args.task_id.clone().map(task_id).transpose()?;
        let prompt = match (&args.prompt, &args.prompt_file) {
            (Some(text), _) => Some(Prompt {
                text: prompt_text(text.clone(), "--prompt", None)?,
                source: PromptSource::Argument,
            }),
            (None, Some(path)) => Some(read_prompt(&cwd.join(path))?),
            (None, None) if requirement == PromptRequirement::Required => {
                return Err(Refusal::new(
                    "malformed_input",
                    Stage::Cli,
                    EXIT_MALFORMED,
                    "run needs the harness prompt, and none was given",
                    "pass exactly one of --prompt TEXT or --prompt-file PATH; harness-dispatch \
                     never reads a prompt from stdin",
                )
                .input("--prompt"));
            }
            (None, None) => None,
        };
        Ok(Inputs {
            kind: args.kind.clone(),
            cwd,
            task_file,
            task_id,
            prompt,
        })
    }
}

fn malformed(input: &str, message: impl Into<String>, remedy: &str) -> Refusal {
    Refusal::new(
        "malformed_input",
        Stage::Cli,
        EXIT_MALFORMED,
        message,
        remedy,
    )
    .input(input)
}

/// Joined, not canonicalized: the caller's spelling of the path is kept, and
/// the file need not exist. It travels in JSON and as one argument, so it must
/// be UTF-8. (Clap has already refused an empty path.)
fn task_file(path: &Path, cwd: &Path) -> Result<String, Refusal> {
    cwd.join(path)
        .into_os_string()
        .into_string()
        .map_err(|path| {
            malformed(
                "--task-file",
                format!(
                    "--task-file {} is not valid UTF-8",
                    Path::new(&path).display()
                ),
                "name the task's file as a UTF-8 path, or omit --task-file",
            )
        })
}

fn task_id(id: OsString) -> Result<String, Refusal> {
    let remedy = "pass the task's stable identity as nonempty UTF-8 of at most 1024 bytes, such \
                  as its handle, or omit --task-id";
    let id = id
        .into_string()
        .map_err(|_| malformed("--task-id", "--task-id is not valid UTF-8", remedy))?;
    if id.is_empty() {
        return Err(malformed(
            "--task-id",
            "--task-id must not be empty",
            remedy,
        ));
    }
    if id.len() > TASK_ID_LIMIT {
        return Err(malformed(
            "--task-id",
            format!(
                "--task-id is {} bytes, over the {TASK_ID_LIMIT}-byte limit",
                id.len()
            ),
            remedy,
        ));
    }
    Ok(id)
}

/// Read a prompt file once, bounded, refusing a terminal rather than waiting
/// on it.
fn read_prompt(path: &Path) -> Result<Prompt, Refusal> {
    let shown = path.to_string_lossy().into_owned();
    let unreadable = |what: String| {
        Refusal::new(
            "prompt_unreadable",
            Stage::Cli,
            EXIT_MALFORMED,
            format!("the prompt file {shown} {what}"),
            "name a readable prompt file with --prompt-file, or pass the prompt with --prompt",
        )
        .input("--prompt-file")
        .source(shown.clone())
    };
    let file =
        fs::File::open(path).map_err(|error| unreadable(format!("cannot be opened: {error}")))?;
    // SAFETY: isatty only inspects the descriptor, which `file` owns.
    if unsafe { libc::isatty(file.as_raw_fd()) } == 1 {
        return Err(unreadable(
            "is a terminal, and harness-dispatch never reads its prompt interactively".to_owned(),
        ));
    }
    let mut bytes = Vec::new();
    file.take(PROMPT_LIMIT as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| unreadable(format!("cannot be read: {error}")))?;
    let text = prompt_bytes(bytes, "--prompt-file", Some(&shown))?;
    Ok(Prompt {
        text,
        source: PromptSource::File(path.to_owned()),
    })
}

fn prompt_text(text: OsString, input: &str, source: Option<&str>) -> Result<String, Refusal> {
    use std::os::unix::ffi::OsStringExt as _;
    prompt_bytes(text.into_vec(), input, source)
}

/// Check a prompt's bytes: within the bound, UTF-8, and free of NUL, which no
/// argument can carry. The bytes are otherwise kept exactly, trailing newlines
/// included.
fn prompt_bytes(bytes: Vec<u8>, input: &str, source: Option<&str>) -> Result<String, Refusal> {
    let invalid = |message: String| {
        let refusal = Refusal::new(
            "prompt_invalid",
            Stage::Cli,
            EXIT_MALFORMED,
            message,
            format!(
                "supply the prompt as UTF-8 text of at most {PROMPT_LIMIT} bytes with no NUL; it \
                 is passed to the harness unchanged and never truncated"
            ),
        )
        .input(input);
        match source {
            Some(source) => refusal.source(source),
            None => refusal,
        }
    };
    if bytes.len() > PROMPT_LIMIT {
        return Err(invalid(format!(
            "the prompt is over the {PROMPT_LIMIT}-byte limit"
        )));
    }
    if let Some(at) = bytes.iter().position(|byte| *byte == 0) {
        return Err(invalid(format!(
            "the prompt contains a NUL byte at offset {at}"
        )));
    }
    String::from_utf8(bytes).map_err(|error| {
        invalid(format!(
            "the prompt is not valid UTF-8 (at byte {})",
            error.utf8_error().valid_up_to()
        ))
    })
}

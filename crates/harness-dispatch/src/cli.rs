//! The command line (`docs/specs/harness-selection-and-execution.md`, *Command
//! interface*).
//!
//! `inspect` and `run` accept the same selection inputs. This release reads the
//! kind, the policy entry, the prompt, the optional task file and identity, the
//! whole-selection bound and the record directory. `record show` exports a
//! recorded run.
//! The spec's other inputs and commands belong to later increments, and until
//! each lands it is refused explicitly, by name, rather than accepted and
//! ignored. They are hidden from help so that help lists only what works.

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::refusal::{Refusal, Stage, EXIT_MALFORMED};

#[derive(Debug, Parser)]
#[command(
    name = "harness-dispatch",
    version,
    about = "Evaluate an owner's harness-selection policy, then report or run the joint choice it makes",
    long_about = "Evaluate an owner's TypeScript harness-selection policy for a session kind, then \
        report the joint harness, model and effort choice it makes (inspect) or replace this \
        process with that harness (run).\n\n\
        The policy is the personal default ~/.config/harness-dispatch/policy.ts, or the entry \
        named by --config. No policy in the current directory runs unless --config names it."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Report the candidate a policy selects, its expanded argv and why, without launching anything
    #[command(after_help = "Examples:\n  \
        harness-dispatch inspect --kind impl\n  \
        harness-dispatch inspect --kind impl --task-id T-12 --prompt 'Implement the parser'\n  \
        harness-dispatch inspect --kind review-impl --config ./policies/review.ts --json")]
    Inspect(InspectArgs),
    /// Select a candidate, record the handoff, and replace this process with its harness
    #[command(
        after_help = "Before it execs, run commits one handoff record with a fresh run ID to \
        the record store (exit 4, and nothing launched, if it cannot). The harness inherits \
        this process's cwd, descriptors, environment and PID, plus HARNESS_DISPATCH_RUN_ID \
        and HARNESS_DISPATCH_STATE_DIR, and its own exit code or signal is the command's. \
        Stdout and stdin are the harness's; the choice and run ID are reported in one line on \
        stderr.\n\n\
        Examples:\n  \
        harness-dispatch run --kind impl --prompt 'Implement the parser'\n  \
        harness-dispatch run --kind impl --task-file ./tasks/parser.md --task-id T-12 --prompt-file ./mandate.md\n  \
        harness-dispatch run --kind impl --state-dir ./records --prompt 'Implement the parser'"
    )]
    Run(RunArgs),
    /// Read the run records that run commits
    Record(RecordArgs),
}

#[derive(Debug, Args)]
pub struct RecordArgs {
    #[command(subcommand)]
    pub command: RecordCommand,
}

#[derive(Debug, Subcommand)]
pub enum RecordCommand {
    /// Export one recorded run: its launch fields, its evidence and its outcomes
    #[command(
        after_help = "A run is a handoff attempt: it was recorded just before exec, and \
        whether the harness then ran, and how it went, stays unknown until observed. An exec \
        that failed is recorded as a launch failure. No outcome is ever inferred.\n\n\
        Examples:\n  \
        harness-dispatch record show --run \"$HARNESS_DISPATCH_RUN_ID\" --json\n  \
        harness-dispatch record show --run 0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e --state-dir ./records"
    )]
    Show(ShowArgs),
    /// Not supported by this release
    #[command(hide = true)]
    Observe(Later),
}

#[derive(Debug, Args)]
pub struct ShowArgs {
    /// The run's ID, as run reported it and the harness received it in HARNESS_DISPATCH_RUN_ID
    #[arg(long, value_name = "RUN_ID")]
    pub run: OsString,
    /// Read records from this directory instead of ~/.local/state/harness-dispatch; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub state_dir: Option<PathBuf>,
    /// Print one version-1 JSON object on stdout, or one JSON error on stderr
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct InspectArgs {
    #[command(flatten)]
    pub selection: SelectionArgs,
    /// Print one version-1 JSON object on stdout, or one JSON error on stderr
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct RunArgs {
    #[command(flatten)]
    pub selection: SelectionArgs,
    /// Write the handoff notice, and any refusal, as JSON lines on stderr
    #[arg(long)]
    pub json: bool,
}

/// The selection inputs `inspect` and `run` share.
#[derive(Debug, Args)]
pub struct SelectionArgs {
    /// The caller's session kind: any nonempty token, matched exactly against the policy's routes
    #[arg(long, value_name = "TEXT")]
    pub kind: String,
    /// The harness prompt, passed unchanged as one argument; run needs this or --prompt-file
    #[arg(
        long,
        value_name = "TEXT",
        allow_hyphen_values = true,
        conflicts_with = "prompt_file"
    )]
    pub prompt: Option<OsString>,
    /// Read the harness prompt from this file, relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub prompt_file: Option<PathBuf>,
    /// The task's file, relative to the current directory; supplies no kind or identity
    #[arg(long, value_name = "PATH")]
    pub task_file: Option<PathBuf>,
    /// The task's stable identity: opaque UTF-8, at most 1024 bytes
    #[arg(long, value_name = "ID", allow_hyphen_values = true)]
    pub task_id: Option<OsString>,
    /// Evaluate this policy entry instead of the personal default; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,
    /// Stop the policy and launch nothing (exit 124) if selection takes longer; 1000 to 120000 [default: 30000]
    #[arg(long, value_name = "MS")]
    pub timeout_ms: Option<OsString>,

    /// Keep run records in this directory instead of ~/.local/state/harness-dispatch; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub state_dir: Option<PathBuf>,

    // The spec's remaining selection inputs, owned by later increments.
    #[arg(long, hide = true)]
    pub context: Option<OsString>,
    #[arg(long, hide = true)]
    pub choice: Option<OsString>,
    #[arg(long, hide = true)]
    pub policy_env: Vec<OsString>,
    #[arg(long, hide = true)]
    pub context_bytes: Option<OsString>,
}

/// Arguments of a command this release refuses, taken whole so that the
/// refusal names the command rather than its first unfamiliar flag.
#[derive(Debug, Args)]
pub struct Later {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub rest: Vec<OsString>,
}

impl SelectionArgs {
    /// Refuse the first input a later increment owns, and the empty values of
    /// the ones this release reads. The values themselves are read and checked
    /// by `inputs`.
    pub fn refuse_unsupported(&self) -> Result<(), Refusal> {
        let later = [
            ("--context", self.context.is_some()),
            ("--choice", self.choice.is_some()),
            ("--policy-env", !self.policy_env.is_empty()),
            ("--context-bytes", self.context_bytes.is_some()),
        ];
        if let Some((flag, _)) = later.iter().find(|(_, given)| *given) {
            return Err(unsupported(&format!("`{flag}`"), flag));
        }
        if self.kind.is_empty() {
            return Err(Refusal::new(
                "malformed_input",
                Stage::Cli,
                EXIT_MALFORMED,
                "--kind must not be empty",
                "pass the caller's session kind, such as --kind impl",
            )
            .input("--kind"));
        }
        if self
            .config
            .as_ref()
            .is_some_and(|config| config.as_os_str().is_empty())
        {
            return Err(Refusal::new(
                "malformed_input",
                Stage::Cli,
                EXIT_MALFORMED,
                "--config must not be empty",
                "name a policy entry, or omit --config to use the personal default",
            )
            .input("--config"));
        }
        Ok(())
    }
}

/// The refusal for an input or command a later release delivers.
pub fn unsupported(what: &str, input: &str) -> Refusal {
    Refusal::new(
        "unsupported_input",
        Stage::Cli,
        EXIT_MALFORMED,
        format!("{what} is not supported by this release of harness-dispatch"),
        "omit it; this release selects through a static routes policy with --kind, --config, \
         --prompt or --prompt-file, --task-file, --task-id, --timeout-ms, --state-dir and \
         --json, and exports runs with record show",
    )
    .input(input)
}

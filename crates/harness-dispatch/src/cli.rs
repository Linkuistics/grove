//! The command line (`docs/specs/harness-selection-and-execution.md`, *Command
//! interface*).
//!
//! This release delivers `inspect` of a static `routes` policy. The spec's other
//! inputs and commands belong to later increments, and until each lands it is
//! refused explicitly, by name, rather than accepted and ignored. They are
//! hidden from help so that help lists only what works.

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::refusal::{Refusal, Stage, EXIT_MALFORMED};

#[derive(Debug, Parser)]
#[command(
    name = "harness-dispatch",
    version,
    about = "Evaluate an owner's harness-selection policy and report the joint choice it makes",
    long_about = "Evaluate an owner's TypeScript harness-selection policy and report the joint \
        harness, model and effort choice it makes for a session kind.\n\n\
        The policy is the personal default ~/.config/harness-dispatch/policy.ts, or the entry \
        named by --config. No policy in the current directory runs unless --config names it."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
#[expect(
    clippy::large_enum_variant,
    reason = "parsed once per process; the refused commands carry only their raw arguments"
)]
pub enum Command {
    /// Report the candidate a policy selects for a kind, and why, without launching anything
    #[command(after_help = "Examples:\n  \
        harness-dispatch inspect --kind impl\n  \
        harness-dispatch inspect --kind review-impl --config ./policies/review.ts --json")]
    Inspect(InspectArgs),
    /// Not supported by this release
    #[command(hide = true)]
    Run(Later),
    /// Not supported by this release
    #[command(hide = true)]
    Record(Later),
}

#[derive(Debug, Args)]
pub struct InspectArgs {
    /// The caller's session kind: any nonempty token, matched exactly against the policy's routes
    #[arg(long, value_name = "TEXT")]
    pub kind: String,
    /// Evaluate this policy entry instead of the personal default; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,
    /// Print one version-1 JSON object on stdout, or one JSON error on stderr
    #[arg(long)]
    pub json: bool,

    // The spec's remaining selection inputs, owned by later increments.
    #[arg(long, hide = true)]
    pub prompt: Option<OsString>,
    #[arg(long, hide = true)]
    pub prompt_file: Option<OsString>,
    #[arg(long, hide = true)]
    pub task_file: Option<OsString>,
    #[arg(long, hide = true)]
    pub task_id: Option<OsString>,
    #[arg(long, hide = true)]
    pub context: Option<OsString>,
    #[arg(long, hide = true)]
    pub choice: Option<OsString>,
    #[arg(long, hide = true)]
    pub policy_env: Vec<OsString>,
    #[arg(long, hide = true)]
    pub timeout_ms: Option<OsString>,
    #[arg(long, hide = true)]
    pub context_bytes: Option<OsString>,
    #[arg(long, hide = true)]
    pub state_dir: Option<OsString>,
}

/// Arguments of a command this release refuses, taken whole so that the
/// refusal names the command rather than its first unfamiliar flag.
#[derive(Debug, Args)]
pub struct Later {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub rest: Vec<OsString>,
}

impl InspectArgs {
    /// Refuse the first input a later increment owns, and malformed values of
    /// the ones this release reads.
    pub fn refuse_unsupported(&self) -> Result<(), Refusal> {
        let later = [
            ("--prompt", self.prompt.is_some()),
            ("--prompt-file", self.prompt_file.is_some()),
            ("--task-file", self.task_file.is_some()),
            ("--task-id", self.task_id.is_some()),
            ("--context", self.context.is_some()),
            ("--choice", self.choice.is_some()),
            ("--policy-env", !self.policy_env.is_empty()),
            ("--timeout-ms", self.timeout_ms.is_some()),
            ("--context-bytes", self.context_bytes.is_some()),
            ("--state-dir", self.state_dir.is_some()),
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
        "omit it; this release inspects a static routes policy with inspect --kind, --config and --json",
    )
    .input(input)
}

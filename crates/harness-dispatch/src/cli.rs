//! The command line (`docs/specs/harness-selection-and-execution.md`, *Command
//! interface*).
//!
//! `inspect` and `run` accept the same selection inputs. This release reads the
//! kind, the policy entry, the prompt, the optional task file and identity, the
//! explicit choice, the whole-selection bound and the record directory.
//! `record show` exports a recorded run.
//! The spec's other inputs and commands belong to later increments, and until
//! each lands it is refused explicitly, by name, rather than accepted and
//! ignored. They are hidden from help so that help lists only what works.

use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::refusal::{Invocation, Refusal, Stage, EXIT_MALFORMED};

#[derive(Debug, Parser)]
#[command(
    name = "harness-dispatch",
    version,
    about = "Evaluate an owner's harness-selection policy, then report or run the joint choice it makes",
    long_about = "Evaluate an owner's TypeScript harness-selection policy for a session kind, then \
        report the joint harness, model and effort choice it makes (inspect) or replace this \
        process with that harness (run).\n\n\
        The policy is the personal default ~/.config/harness-dispatch/policy.ts, or the entry \
        named by --config. No policy in the current directory runs unless --config names it. \
        Nothing else is needed: no task tree, Grove installation or other caller.",
    after_help = "Examples:\n  \
        harness-dispatch inspect --kind impl\n  \
        harness-dispatch run --kind impl --prompt 'Implement the parser'\n  \
        harness-dispatch record show --run \"$HARNESS_DISPATCH_RUN_ID\" --json\n\n\
        Exit results before the harness runs: 2 malformed command line; 3 refused by the \
        policy, the selection or its inputs; 4 run record failure; 5 worker or protocol failure; \
        124 selection timeout; 126 program not executable; 127 program not found. Once the \
        harness runs, its own exit status or signal is the command's.\n\n\
        A refusal launches nothing and never substitutes another candidate. Nothing is retried, \
        paged or confirmed interactively. A refused run prints the equivalent inspect \
        invocation, without the prompt: run it to see the same selection without launching."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Report the candidate a policy selects, its expanded argv and why, without launching anything
    #[command(
        after_help = "Inspection is a proposal, not a launch reservation: a later run \
        evaluates the policy afresh. The policy is trusted TypeScript, and evaluating it is not \
        promised to be free of side effects.\n\n\
        Examples:\n  \
        harness-dispatch inspect --kind impl\n  \
        harness-dispatch inspect --kind impl --task-id T-12 --prompt 'Implement the parser'\n  \
        harness-dispatch inspect --kind impl --choice deep\n  \
        harness-dispatch inspect --kind review-impl --config ./policies/review.ts --json\n\n\
        Recovering from a refusal:\n  \
        A refused run prints this command's equivalent invocation, without the prompt. Run it \
        to reproduce the selection and its refusal without launching anything, correct the \
        input or policy entry its remedy names, and inspect again until it reports a choice. \
        For an incomplete mapping, add the kind's route, or name one configured candidate:\n  \
        harness-dispatch inspect --kind design --choice deep"
    )]
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
        harness-dispatch run --kind impl --choice deep --prompt 'Implement the parser'\n  \
        harness-dispatch run --kind impl --state-dir ./records --prompt 'Implement the parser'\n\n\
        Recovering from a refusal:\n  \
        A refused run launches nothing, and names its code, stage, input or source, and remedy, \
        followed by the equivalent inspect invocation without the prompt, such as\n  \
        (cd /work && harness-dispatch inspect --kind design)\n  \
        Run that to reproduce the selection, correct what the remedy names, and run again. \
        Nothing is retried for you."
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
    /// Select this configured candidate by its catalog ID instead of the kind's route; an ID the catalog lacks refuses
    #[arg(long, value_name = "ID")]
    pub choice: Option<OsString>,
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

    /// The `inspect` invocation equivalent to `run` with these inputs: the same
    /// selection inputs, `--policy-env` names (which carry no values), and no
    /// prompt, run from the same directory. The program is this process's own
    /// argv[0], as the caller spelled it, so the reproduction reaches the same
    /// installation. It follows `--json` when the refusal did. When a word or
    /// the directory is not UTF-8 it is unavailable, never approximated.
    pub fn inspect_invocation(&self, json: bool) -> Invocation {
        self.exact_inspect_invocation(json)
            .unwrap_or_else(|what| Invocation::Unavailable { what })
    }

    fn exact_inspect_invocation(&self, json: bool) -> Result<Invocation, String> {
        let program = match std::env::args_os().next().filter(|p| !p.is_empty()) {
            Some(program) => exact(&program, "the program path harness-dispatch was run as")?,
            None => "harness-dispatch".to_owned(),
        };
        let mut argv = vec![program, "inspect".to_owned()];
        option(&mut argv, "--kind", OsStr::new(&self.kind))?;
        let given = [
            ("--choice", self.choice.as_deref()),
            (
                "--config",
                self.config.as_deref().map(|path| path.as_os_str()),
            ),
            (
                "--task-file",
                self.task_file.as_deref().map(|path| path.as_os_str()),
            ),
            ("--task-id", self.task_id.as_deref()),
            ("--timeout-ms", self.timeout_ms.as_deref()),
            (
                "--state-dir",
                self.state_dir.as_deref().map(|path| path.as_os_str()),
            ),
            ("--context", self.context.as_deref()),
            ("--context-bytes", self.context_bytes.as_deref()),
        ];
        for (flag, value) in given {
            if let Some(value) = value {
                option(&mut argv, flag, value)?;
            }
        }
        for name in &self.policy_env {
            option(&mut argv, "--policy-env", name)?;
        }
        if json {
            argv.push("--json".to_owned());
        }
        let cwd = match std::env::current_dir() {
            Ok(cwd) => Some(exact(cwd.as_os_str(), "the current directory")?),
            Err(_) => None,
        };
        Ok(Invocation::Exact { cwd, argv })
    }
}

/// `value` as a string, or `what` when it is not UTF-8.
fn exact(value: &OsStr, what: &str) -> Result<String, String> {
    value
        .to_str()
        .map(str::to_owned)
        .ok_or_else(|| what.to_owned())
}

/// `flag value` as two words, or one `flag=value` word when the value starts
/// with a hyphen and would otherwise read as a flag of its own. A value that
/// is not UTF-8 names its flag instead.
fn option(argv: &mut Vec<String>, flag: &str, value: &OsStr) -> Result<(), String> {
    let value = exact(value, flag)?;
    if value.starts_with('-') {
        argv.push(format!("{flag}={value}"));
    } else {
        argv.push(flag.to_owned());
        argv.push(value);
    }
    Ok(())
}

/// The refusal for an input or command a later release delivers.
pub fn unsupported(what: &str, input: &str) -> Refusal {
    Refusal::new(
        "unsupported_input",
        Stage::Cli,
        EXIT_MALFORMED,
        format!("{what} is not supported by this release of harness-dispatch"),
        "omit it; this release selects through a static routes policy with --kind, --choice, \
         --config, --prompt or --prompt-file, --task-file, --task-id, --timeout-ms, --state-dir \
         and --json, and exports runs with record show",
    )
    .input(input)
}

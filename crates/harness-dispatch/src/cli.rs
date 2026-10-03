//! The command line (`docs/specs/harness-selection-and-execution.md`, *Command
//! interface*).
//!
//! `inspect` and `run` accept the same selection inputs: the kind, the policy
//! entry, the prompt, the optional task file and identity, the caller's
//! parameters and context document, the selection and context bounds, the
//! record directory and the worker's environment grants. The last four are
//! also owner settings (`settings`), which a flag replaces or adds to. `init`
//! takes no input. `record show` exports a recorded run, and `record observe`
//! appends a later observation to one.

use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::refusal::{Invocation, Refusal, Stage, EXIT_MALFORMED};

#[derive(Debug, Parser)]
#[command(
    name = "harness-dispatch",
    version,
    about = "Pass a caller's inputs to an owner's select function, then report or run the command it returns",
    long_about = "Pass a caller's kind, prompt and parameters to the select function of an owner's \
        TypeScript policy, then report the command it returns (inspect) or replace this process \
        with that command (run). A select returns a program, its arguments and the owner's \
        provider, model and effort labels for what they run, or a refusal.\n\n\
        The policy is the personal default ~/.config/harness-dispatch/policy.ts, or the entry \
        named by --config. No policy in the current directory runs unless --config names it. \
        Nothing else is needed: no task tree, Grove installation or other caller. With no \
        policy, inspect and run refuse; init installs a sample policy as the personal default, \
        and nothing else ever writes one.\n\n\
        Owner settings in ~/.config/harness-dispatch/settings.json apply to every invocation \
        with no flag passed: timeoutMs, contextBytes, stateDir (an absolute path) and policyEnv \
        (an array of names). A flag replaces its setting, and --policy-env adds to policyEnv. \
        inspect reports where each value came from.",
    after_help = "Examples:\n  \
        harness-dispatch init\n  \
        harness-dispatch inspect --kind impl\n  \
        harness-dispatch run --kind impl --prompt 'Implement the parser'\n  \
        harness-dispatch run --kind impl --param profile=careful --prompt 'Implement the parser'\n  \
        harness-dispatch record show --run \"$HARNESS_DISPATCH_RUN_ID\" --json\n  \
        harness-dispatch record observe --run \"$HARNESS_DISPATCH_RUN_ID\" --file observation.json\n\n\
        From Grove: Grove runs this command itself for every lifecycle session, in the \
        working-tree root, with values from the leaf it launches. Grove has no launch \
        configuration: your policy returns the harness command, and is evaluated only at \
        launch:\n  \
        harness-dispatch run --kind=KIND --task-file=TASK_FILE --task-id=HANDLE --prompt=MANDATE\n\n\
        Exit results before the harness runs: 2 malformed command line; 3 refused by the \
        policy, the selection or its inputs; 4 run record failure; 5 worker or protocol failure; \
        124 selection timeout; 126 program not executable; 127 program not found. Once the \
        harness runs, its own exit status or signal is the command's.\n\n\
        A refusal launches nothing and nothing is run in its place. Nothing is retried, \
        paged or confirmed interactively. A refused run prints the equivalent inspect \
        invocation, without the prompt: run it to see the same selection without launching. A \
        policy that reads the prompt selects as it did only when the same prompt is added."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Report the command a policy's select returns, its labels and why, without launching anything
    #[command(
        after_help = "Inspection is a proposal, not a launch reservation: a later run \
        evaluates the policy afresh. The policy is trusted TypeScript, and evaluating it is not \
        promised to be free of side effects. Without --prompt or --prompt-file, select receives \
        a fixed marker as its prompt, which harness-dispatch/sdk names PROMPT_NOT_SUPPLIED: a \
        policy that only places the prompt in its arguments shows the marker there, and one \
        that reads the prompt selects from the marker.\n\n\
        Examples:\n  \
        harness-dispatch inspect --kind impl\n  \
        harness-dispatch inspect --kind impl --task-id T-12 --prompt 'Implement the parser'\n  \
        harness-dispatch inspect --kind impl --param profile=careful --param label=parser\n  \
        harness-dispatch inspect --kind review --context ./review-context.json --json\n  \
        harness-dispatch inspect --kind review-impl --config ./policies/review.ts --json\n  \
        harness-dispatch inspect --kind impl --policy-env ROUTER_TOKEN\n\n\
        Recovering from a refusal:\n  \
        A refused run prints this command's equivalent invocation, without the prompt. Run it \
        to reproduce the selection and its refusal without launching anything, correct the \
        input or policy entry its remedy names, and inspect again until it reports a command. \
        A policy that reads the prompt needs the same prompt added to select as it did:\n  \
        harness-dispatch inspect --kind design --prompt-file ./mandate.md"
    )]
    Inspect(InspectArgs),
    /// Select a command, record the handoff, and replace this process with it
    #[command(
        after_help = "Before it execs, run commits one handoff record with a fresh run ID to \
        the record store (exit 4, and nothing launched, if it cannot). The harness inherits \
        this process's cwd, descriptors, environment and PID, plus HARNESS_DISPATCH_RUN_ID \
        and HARNESS_DISPATCH_STATE_DIR, and its own exit code or signal is the command's. \
        Stdout and stdin are the harness's; the labels and run ID are reported in one line on \
        stderr.\n\n\
        Examples:\n  \
        harness-dispatch run --kind impl --prompt 'Implement the parser'\n  \
        harness-dispatch run --kind impl --task-file ./tasks/parser.md --task-id T-12 --prompt-file ./mandate.md\n  \
        harness-dispatch run --kind impl --param profile=careful --prompt 'Implement the parser'\n  \
        harness-dispatch run --kind review --context ./review-context.json --context-bytes 1048576 --prompt-file ./mandate.md\n  \
        harness-dispatch run --kind impl --state-dir ./records --prompt 'Implement the parser'\n\n\
        From Grove: what Grove runs for every lifecycle session, in the working-tree root, \
        with values from the leaf it launches:\n  \
        harness-dispatch run --kind=KIND --task-file=TASK_FILE --task-id=HANDLE --prompt=MANDATE\n\n\
        Recovering from a refusal:\n  \
        A refused run launches nothing, and names its code, stage, input or source, and remedy, \
        followed by the equivalent inspect invocation without the prompt, such as\n  \
        (cd /work && harness-dispatch inspect --kind design)\n  \
        Run that to reproduce the selection, correct what the remedy names, and run again. \
        Nothing is retried for you."
    )]
    Run(RunArgs),
    /// Install the sample policy as your personal default, if nothing is there
    #[command(
        after_help = "init writes the sample policy to ~/.config/harness-dispatch/policy.ts and \
        reports that path. It takes no input, refuses when anything already exists there, and \
        has no option to replace it. The file is then yours to edit.\n\n\
        The sample is one owner's launch policy for Grove, with the real codex and claude \
        command lines. It launches codex with approvals off and full access, so read it before \
        the first launch. It routes each of Grove's session kinds and the standalone \
        release-notes kind. It reads no parameter and names no session: run from a secondary jj \
        workspace, it grants both harnesses the main repository that the workspace's .jj/repo \
        names, and anywhere else nothing more. It offers four arrangements of which harness leads and which reviews, and two modifiers; \
        a .harness-dispatch-choice file in the directory you run from names one arrangement and \
        any modifiers, in place of the sample's default.\n\n\
        Examples:\n  \
        harness-dispatch init\n  \
        (cd /work/parser && harness-dispatch inspect --kind impl)\n  \
        echo 'codex-led high-effort' > .harness-dispatch-choice"
    )]
    Init,
    /// Export the run records that run commits, and add later observations to them
    Record(RecordArgs),
}

#[derive(Debug, Args)]
pub struct RecordArgs {
    #[command(subcommand)]
    pub command: RecordCommand,
}

#[derive(Debug, Subcommand)]
pub enum RecordCommand {
    /// Export one recorded run: its launch fields, its evidence, its observations and its measurements
    #[command(
        after_help = "A run is a handoff attempt: it was recorded just before exec, and \
        whether the harness then ran, and how it went, stays unknown until observed. An exec \
        that failed is recorded as a launch failure. A current observation that confirms \
        execution makes the run execution_confirmed. Every measurement no current observation \
        supplies is unobserved; nothing is inferred, and values from several observations are \
        listed side by side, never combined.\n\n\
        Examples:\n  \
        harness-dispatch record show --run \"$HARNESS_DISPATCH_RUN_ID\" --json\n  \
        harness-dispatch record show --run 0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e --state-dir ./records"
    )]
    Show(ShowArgs),
    /// Validate one version-1 observation of a recorded run and append it to the run's record
    #[command(
        after_help = "An observation is later evidence about a run, from an observer: that \
        its harness ran, how it exited, how long it took and what it used, whether its work was \
        accepted, the defects it missed and the findings that were false, the repair it caused \
        downstream, the human work it needed, and links to the evidence. Each measurement's \
        state is observed (with a value, and a unit for a quantity), unknown or unobserved; a \
        measurement left out stays unobserved. harness-dispatch checks the document's shape and \
        that it names this run, not whether it is true, and never changes the run's launch \
        fields.\n\n\
        Importing the same observationId with the same content again changes nothing; with \
        other content it is refused. To correct an observation, import a new one whose \
        supersedes names it: both are kept, and the correction replaces it.\n\n\
        An observation (observation.json):\n  \
        {\n    \
          \"schemaVersion\": 1,\n    \
          \"observationId\": \"review-k46-outcome\",\n    \
          \"runId\": \"0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e\",\n    \
          \"source\": \"review-impl session for static-dispatch-k12\",\n    \
          \"observedAt\": \"2026-10-01T09:30:00Z\",\n    \
          \"evidence\": \"the review's findings, integrated in change qrksvsyv\",\n    \
          \"measurements\": {\n      \
            \"executionConfirmation\": { \"state\": \"observed\", \"value\": true },\n      \
            \"duration\": { \"state\": \"observed\", \"value\": 1260, \"unit\": \"s\" },\n      \
            \"acceptance\": { \"state\": \"observed\", \"value\": \"accepted\" },\n      \
            \"missedDefects\": { \"state\": \"observed\", \"value\": [{ \"id\": \"F3\", \"summary\": \"unbounded read\" }] },\n      \
            \"totalUsage\": { \"state\": \"unknown\" }\n    \
          }\n  \
        }\n\n\
        Examples:\n  \
        harness-dispatch record observe --run \"$HARNESS_DISPATCH_RUN_ID\" --file observation.json\n  \
        harness-dispatch record observe --run 0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e --file correction.json --state-dir ./records --json"
    )]
    Observe(ObserveArgs),
}

#[derive(Debug, Args)]
pub struct ShowArgs {
    /// The run's ID, as run reported it and the harness received it in HARNESS_DISPATCH_RUN_ID
    #[arg(long, value_name = "RUN_ID")]
    pub run: OsString,
    /// Read records from this directory instead of the stateDir owner setting or ~/.local/state/harness-dispatch; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub state_dir: Option<PathBuf>,
    /// Print one version-1 JSON object on stdout, or one JSON error on stderr
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct ObserveArgs {
    /// The observed run's ID; the observation's runId must name the same run
    #[arg(long, value_name = "RUN_ID")]
    pub run: OsString,
    /// The version-1 observation document, a JSON file relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub file: PathBuf,
    /// Read and append records in this directory instead of the stateDir owner setting or ~/.local/state/harness-dispatch; relative to the current directory
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
    /// Print one version-2 JSON object on stdout, or one JSON error on stderr
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
    /// The caller's session kind: any nonempty token, given to the policy's select as it is
    #[arg(long, value_name = "TEXT")]
    pub kind: String,
    /// The harness prompt, given to the policy's select byte for byte; run needs this or --prompt-file
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
    /// Caller data for the policy's select, by name; repeatable. The name is nonempty and holds no =, the value is any UTF-8 text, a repeated name refuses, and all names and values together are at most 65536 bytes
    #[arg(long, value_name = "NAME=VALUE", allow_hyphen_values = true)]
    pub param: Vec<OsString>,
    /// A version-1 JSON context document for the policy, read as data; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub context: Option<PathBuf>,
    /// Stop the policy and launch nothing (exit 124) if selection takes longer; 1000 to 600000, replacing the timeoutMs owner setting [default: 30000]
    #[arg(long, value_name = "MS")]
    pub timeout_ms: Option<OsString>,
    /// Refuse a context delivered to selection that encodes to more bytes; 1 to 8388608, replacing the contextBytes owner setting [default: 262144]
    #[arg(long, value_name = "BYTES")]
    pub context_bytes: Option<OsString>,

    /// Keep run records in this directory instead of the stateDir owner setting or ~/.local/state/harness-dispatch; relative to the current directory
    #[arg(long, value_name = "PATH")]
    pub state_dir: Option<PathBuf>,

    /// Give the policy worker this environment variable, by exact name, beyond HOME, PATH, TMPDIR, LANG, LC_* and the policyEnv owner setting; repeatable. BUN_*, NODE_OPTIONS, NODE_PATH, NODE_PRESERVE_SYMLINKS, NODE_CHANNEL_*, LD_*, DYLD_* and HARNESS_DISPATCH_* are never granted, and values are never shown. Do not grant GROVE_SIGNAL_FILE: it would give the policy the power to end a Grove session
    #[arg(long, value_name = "NAME")]
    pub policy_env: Vec<OsString>,
}

impl SelectionArgs {
    /// Refuse the empty values clap admits. The values themselves are read and
    /// checked by `inputs`.
    pub fn refuse_empty(&self) -> Result<(), Refusal> {
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
    /// selection inputs, parameters included, `--policy-env` names (which carry
    /// no values), and no prompt, run from the same directory. The program is this process's own
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
        for param in &self.param {
            option(&mut argv, "--param", param)?;
        }
        let given = [
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
            (
                "--context",
                self.context.as_deref().map(|path| path.as_os_str()),
            ),
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

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "grove", version, about = "Grove: hierarchical workstream tool for AI agents")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Materialise grove into <repo>'s .<harness>/skills/grove/ (create-only).
    Install(InstallArgs),
    /// Refresh an existing grove install (update-only).
    Update(InstallArgs),
    /// Remove grove from <repo>.
    Uninstall(UninstallArgs),
    /// Show CLI and installed grove version.
    Version,
    /// Show grove install + grove tree state in <repo>.
    Status(RepoArgs),
    /// List groves in <repo>, one per line.
    List(RepoArgs),
    /// Start a new grove: create worktree + launch harness.
    Start(StartArgs),
    /// Continue an existing grove.
    Continue(NameArgs),
    /// Start or continue a grove — use this when you don't remember which.
    ///
    /// Inspects the grove's state and dispatches: no grove by that name →
    /// `start`; live worktree → `continue`; branch exists but worktree gone
    /// (orphaned or finished) → re-attach the worktree and `continue`.
    Do(NameArgs),
    /// Orient on an unfamiliar grove without picking a task.
    Takeover(NameArgs),
    /// Retire a done node (promote brief, mv into done/).
    Retire(RetireArgs),
    /// Wrap up a fully-done grove (merge, cleanup).
    Finish(NameArgs),
    /// Inspect a grove's inbox on the `grove-meta` branch (diagnostic).
    ///
    /// The corresponding capture and drain verbs are LLM-driven and live on
    /// `grove-llm` (per ADR-0006): `grove-llm inbox-add`, `grove-llm inbox-drain`.
    Inbox(InboxArgs),
    /// Manage the `grove-meta` branch (init, remote, sync).
    Meta(MetaArgs),
    /// Launch the read-only TUI navigator over this repo's groves.
    Tui(RepoArgs),
}

#[derive(Parser)]
pub struct InboxArgs {
    #[command(subcommand)]
    pub command: InboxCommand,
}

#[derive(Subcommand)]
pub enum InboxCommand {
    /// Print the inbox of `<name>` to stdout (diagnostic).
    Show(InboxShowArgs),
}

#[derive(Parser)]
pub struct MetaArgs {
    #[command(subcommand)]
    pub command: MetaCommand,
}

#[derive(Subcommand)]
pub enum MetaCommand {
    /// Create the `grove-meta` branch and attach its worktree at
    /// `<repo>/.grove-meta/`. Idempotent: a no-op when both are already in
    /// place (prints the materialised state). Invoked internally by
    /// `grove install` / `grove update`; invoke explicitly for repos that
    /// pre-date the feature or whose worktree has been removed.
    Init(MetaInitArgs),
    /// Configure the optional upstream remote for the meta branch.
    Remote(MetaRemoteArgs),
    /// Push pending commits and pull latest on the meta branch.
    #[command(long_about = "\
Push pending commits to the meta-branch remote and pull the latest. \
Best-effort: stdout is silent on success/no-op, stderr is informative \
on action or failure. Exits non-zero only on conflict (non-ff after \
fetch, or push rejected after fetch+retry); network and offline failures \
are warned-and-continued so cron does not see a failure for transient \
issues.\n\
\n\
Representative cron line (adjust the path to suit your setup):\n\
\n\
    */15 * * * * cd /path/to/your/repo && grove meta sync\
")]
    Sync(MetaSyncArgs),
}

#[derive(Parser)]
pub struct MetaInitArgs {
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct MetaRemoteArgs {
    #[command(subcommand)]
    pub command: MetaRemoteCommand,
}

#[derive(Subcommand)]
pub enum MetaRemoteCommand {
    /// Configure an upstream remote and set tracking on the meta branch.
    Add(MetaRemoteAddArgs),
    /// Clear upstream tracking on the meta branch. Local commits stand.
    Remove(MetaRemoteRemoveArgs),
    /// Print the configured remote(s) and upstream tracking state.
    List(MetaRemoteListArgs),
}

#[derive(Parser)]
pub struct MetaRemoteAddArgs {
    /// Remote URL (e.g. `git@github.com:user/repo.git`). If the worktree's
    /// `origin` already points at this URL, it is reused as-is.
    pub url: String,
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct MetaRemoteRemoveArgs {
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct MetaRemoteListArgs {
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct MetaSyncArgs {
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

// `InboxAddArgs` and `InboxDrainArgs` are defined here (not under
// `llm_cli`) because clap derive needs the `Parser` types to be reachable
// from both binaries' command enums; the LLM-driven dispatch lives in
// `crate::llm_cli`.
#[derive(Parser)]
pub struct InboxAddArgs {
    /// Name of the addressed grove (existing, future, or finished).
    #[arg(long = "to")]
    pub to: String,
    /// Override the mechanically-derived slug component of the entry filename.
    #[arg(long = "slug")]
    pub slug: Option<String>,
    /// Observation body as an inline string.
    #[arg(long = "body", conflicts_with_all = ["body_file", "body_stdin"])]
    pub body: Option<String>,
    /// Observation body read from a file at the given path.
    #[arg(long = "body-file", conflicts_with_all = ["body", "body_stdin"])]
    pub body_file: Option<PathBuf>,
    /// Read the observation body from stdin (single-shot, no prompt).
    #[arg(long = "body-stdin", conflicts_with_all = ["body", "body_file"])]
    pub body_stdin: bool,
    /// Target repo (defaults to cwd's git root). Cross-repo: supply a path
    /// to another repo whose `.grove-meta/` worktree is materialised.
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct InboxDrainArgs {
    /// Name of the grove whose inbox to drain.
    #[arg(long = "for")]
    pub for_grove: String,
    /// Finalize: path of an observation that was incorporated into the
    /// current task. Repeatable.
    #[arg(long = "incorporated")]
    pub incorporated: Vec<PathBuf>,
    /// Finalize: path of an observation that was deferred (e.g. captured as
    /// a follow-up leaf or re-seeded). Repeatable.
    #[arg(long = "deferred")]
    pub deferred: Vec<PathBuf>,
    /// Finalize: path of an observation that was rejected as out-of-scope.
    /// Repeatable.
    #[arg(long = "rejected")]
    pub rejected: Vec<PathBuf>,
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct InboxShowArgs {
    /// Name of the grove whose inbox to print.
    pub name: String,
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct InstallArgs {
    /// Target repo (defaults to cwd's git root).
    pub repo: Option<PathBuf>,
    /// Harness(es) to target. Repeatable. Default: auto-detect.
    #[arg(long = "harness")]
    pub harnesses: Vec<String>,
    /// Content version tag (default: latest).
    #[arg(long)]
    pub version: Option<String>,
    /// Skip the auto-commit; leaves the materialised files unstaged.
    #[arg(long = "no-commit")]
    pub no_commit: bool,
    /// Override the default commit message.
    #[arg(long = "message", short = 'm')]
    pub message: Option<String>,
}

#[derive(Parser)]
pub struct UninstallArgs {
    /// Target repo (defaults to cwd's git root).
    pub repo: Option<PathBuf>,
    #[arg(long = "harness")]
    pub harnesses: Vec<String>,
    /// Remove even if live groves exist.
    #[arg(long)]
    pub force: bool,
}

#[derive(Parser)]
pub struct RepoArgs {
    /// Target repo (defaults to cwd's git root).
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct StartArgs {
    pub name: String,
    /// Branch start point (default: origin's HEAD or `main`).
    #[arg(long = "start-point")]
    pub start_point: Option<String>,
    #[arg(long = "harness")]
    pub harness: Option<String>,
    /// Set up the worktree but don't exec the harness.
    #[arg(long = "no-launch")]
    pub no_launch: bool,
}

#[derive(Parser)]
pub struct NameArgs {
    pub name: String,
    #[arg(long = "harness")]
    pub harness: Option<String>,
    #[arg(long = "no-launch")]
    pub no_launch: bool,
}

#[derive(Parser)]
pub struct RetireArgs {
    /// `<name>/<node-path>` (e.g. `auth/003-session-store`).
    pub path: String,
    #[arg(long = "harness")]
    pub harness: Option<String>,
    #[arg(long = "no-launch")]
    pub no_launch: bool,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Install(args) => crate::install::run(&args, crate::install::Mode::Install),
        Command::Update(args)  => crate::install::run(&args, crate::install::Mode::Update),
        Command::Uninstall(args) => crate::uninstall::run(&args),
        Command::Version => crate::version::run(),
        Command::Status(args) => crate::status::run(&args),
        Command::List(args) => crate::list::run(&args),
        Command::Start(args)    => crate::launch::start(&args),
        Command::Continue(args) => crate::launch::continue_grove(&args),
        Command::Do(args)       => crate::launch::do_grove(&args),
        Command::Takeover(args) => crate::launch::takeover(&args),
        Command::Retire(args)   => crate::launch::retire(&args),
        Command::Finish(args)   => crate::launch::finish(&args),
        Command::Inbox(args)    => match args.command {
            InboxCommand::Show(a) => crate::inboxes::cmd_show(&a),
        },
        Command::Meta(args)     => crate::meta::run(&args),
        Command::Tui(args)      => crate::tui::run(&args),
    }
}

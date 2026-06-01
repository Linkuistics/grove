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
    /// Materialise grove into <repo>'s .<harness>/skills/grove/ (idempotent:
    /// installs if absent, updates if a different version, no-ops if current).
    Install(InstallArgs),
    /// Remove grove from <repo>.
    Uninstall(UninstallArgs),
    /// Show grove install + grove tree state in <repo>.
    Status(RepoArgs),
    /// Start or continue a grove — the sole lifecycle entry verb.
    ///
    /// Inspects the grove's state and dispatches: no grove by that name →
    /// create the worktree and open a bootstrap session; live worktree →
    /// continue; branch exists but worktree gone (orphaned or finished) →
    /// re-attach the worktree and continue. When the grove has no live
    /// leaves left, the in-session loop proposes the complete finish cycle.
    Do(StartArgs),
    /// Orient on an unfamiliar grove without picking a task.
    Takeover(NameArgs),
    /// Retire a done node (promote brief, mv into done/).
    Retire(RetireArgs),
    /// Inspect a grove's inbox on the `grove-meta` branch (diagnostic).
    ///
    /// The corresponding capture and drain verbs are LLM-driven and live on
    /// `grove-llm` (per ADR-0006): `grove-llm inbox-add`, `grove-llm inbox-drain`.
    Inbox(InboxArgs),
    /// Manage the `grove-meta` branch (init, remote, sync).
    Meta(MetaArgs),
    /// Launch the grove dashboard. By default this is the head binary: it
    /// launches the grove-owned zellij substrate (ADR-0015/0016) with the
    /// dashboard running as a dumb proxy pane inside it. `--local` runs the
    /// legacy in-terminal dashboard directly (no zellij), a dev/debug escape.
    Tui(TuiArgs),
    /// Internal: the dumb dashboard proxy (ADR-0016). Runs in a zellij pane and
    /// relays a controller's rendered output to the tty and its stdin back up.
    /// Hidden from `--help`, like the `grove-llm` surface (ADR-0006).
    #[command(name = "__dash-proxy", hide = true)]
    DashProxy(DashProxyArgs),
    /// Internal: the dashboard controlling process (ADR-0016). Renders the
    /// dashboard and serves one connected `grove __dash-proxy` over the socket.
    /// 030 will launch this behind zellij; for now it is run standalone to
    /// exercise the seam. Hidden from `--help`, like `__dash-proxy`.
    #[command(name = "__dash-controller", hide = true)]
    DashController(DashControllerArgs),
}

#[derive(Parser)]
pub struct TuiArgs {
    /// Target repo (defaults to cwd's git root).
    pub repo: Option<PathBuf>,
    /// Run the legacy in-terminal dashboard directly, without the zellij
    /// substrate. A dev/debug escape hatch (needs no installed zellij); hidden
    /// because the zellij substrate is the supported path (ADR-0016).
    #[arg(long, hide = true)]
    pub local: bool,
}

#[derive(Parser)]
pub struct DashProxyArgs {
    /// Path to the controlling process's unix-domain socket.
    #[arg(long)]
    pub socket: PathBuf,
}

#[derive(Parser)]
pub struct DashControllerArgs {
    /// Path to bind the controlling process's unix-domain socket at.
    #[arg(long)]
    pub socket: PathBuf,
    /// Target repo (defaults to cwd's git root), like `grove tui`.
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
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
    /// `grove install`; invoke explicitly for repos that
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
pub struct InboxRemoveArgs {
    /// Name of the grove whose inbox to remove (finish-cycle cleanup).
    #[arg(long = "for")]
    pub for_grove: String,
    /// Target repo (defaults to cwd's git root).
    #[arg(long = "repo")]
    pub repo: Option<PathBuf>,
}

#[derive(Parser)]
pub struct InboxEditArgs {
    /// Path of the observation to edit (a `.md` file inside
    /// `inboxes/<name>/` on the `grove-meta` worktree). The addressed grove is
    /// read off the path.
    pub path: PathBuf,
    /// New observation body as an inline string.
    #[arg(long = "body", conflicts_with_all = ["body_file", "body_stdin"])]
    pub body: Option<String>,
    /// New observation body read from a file at the given path.
    #[arg(long = "body-file", conflicts_with_all = ["body", "body_stdin"])]
    pub body_file: Option<PathBuf>,
    /// Read the new observation body from stdin (single-shot, no prompt).
    #[arg(long = "body-stdin", conflicts_with_all = ["body", "body_file"])]
    pub body_stdin: bool,
    /// Target repo (defaults to cwd's git root).
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
        Command::Install(args) => crate::install::run(&args),
        Command::Uninstall(args) => crate::uninstall::run(&args),
        Command::Status(args) => crate::status::run(&args),
        Command::Do(args)       => crate::launch::do_grove(&args),
        Command::Takeover(args) => crate::launch::takeover(&args),
        Command::Retire(args)   => crate::launch::retire(&args),
        Command::Inbox(args)    => match args.command {
            InboxCommand::Show(a) => crate::inboxes::cmd_show(&a),
        },
        Command::Meta(args)     => crate::meta::run(&args),
        Command::Tui(args)      => {
            let repo_args = RepoArgs { repo: args.repo };
            if args.local {
                crate::tui::run(&repo_args)
            } else {
                crate::zellij::launch(&repo_args)
            }
        }
        Command::DashProxy(args) => crate::dash::proxy::run(&args.socket),
        Command::DashController(args) => {
            crate::tui::run_controller(&args.socket, &RepoArgs { repo: args.repo })
        }
    }
}

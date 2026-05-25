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
    /// First-time project setup: install + CONTEXT.md + docs/adr/.
    Init(InitArgs),
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
    /// Orient on an unfamiliar grove without picking a task.
    Takeover(NameArgs),
    /// Retire a done node (promote brief, mv into done/).
    Retire(RetireArgs),
    /// Wrap up a fully-done grove (merge, cleanup).
    Finish(NameArgs),
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
pub struct InitArgs {
    /// Target repo (defaults to cwd's git root).
    pub repo: Option<PathBuf>,
    #[arg(long = "harness")]
    pub harnesses: Vec<String>,
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
        Command::Init(args) => crate::init::run(&args),
        Command::Version => crate::version::run(),
        Command::Status(args) => crate::status::run(&args),
        Command::List(args) => crate::list::run(&args),
        Command::Start(args)    => crate::launch::start(&args),
        Command::Continue(args) => crate::launch::continue_grove(&args),
        Command::Takeover(args) => crate::launch::takeover(&args),
        Command::Retire(args)   => crate::launch::retire(&args),
        Command::Finish(args)   => crate::launch::finish(&args),
    }
}

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "grove",
    version,
    about = "Grove: hierarchical workstream tool for AI agents"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start or continue a grove — the sole lifecycle entry verb.
    ///
    /// Inspects the grove's state and dispatches: no grove by that name →
    /// create the worktree and open a bootstrap session; live worktree →
    /// continue; branch exists but worktree gone (orphaned or finished) →
    /// re-attach the worktree and continue. When the grove has no live
    /// leaves left, the in-session loop proposes the complete finish cycle.
    Do(StartArgs),
    /// Migrate this worktree's `.grove/` from the old `NNN-slug/` + `done/`
    /// directory format to the new flat dotted-decimal scheme (ADR-0033/0034),
    /// in place. A reviewable git change (`git mv` + `# …` header rewrites, no
    /// commit) — review the diff, then commit. No-op on an already-migrated tree
    /// or a missing/foreign `.grove/`. `grove do` runs this automatically on
    /// adoption; invoke it explicitly to migrate a tree by hand.
    Migrate(MigrateArgs),
    /// Orient on an unfamiliar grove without picking a task.
    Takeover(NameArgs),
    /// Retire a done node (promote brief, mv into done/).
    Retire(RetireArgs),
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
pub struct MigrateArgs {
    /// Worktree whose `.grove/` to migrate (default: the current worktree, via
    /// `git rev-parse --show-toplevel`).
    pub path: Option<PathBuf>,
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
        Command::Do(args) => crate::launch::do_grove(&args),
        Command::Migrate(args) => crate::migrate::run(&args),
        Command::Takeover(args) => crate::launch::takeover(&args),
        Command::Retire(args) => crate::launch::retire(&args),
    }
}

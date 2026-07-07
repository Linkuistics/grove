use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Extra `grove do --help` section documenting the per-kind model-selection env
/// vars (model-per-task-kind). The rationale lives in the ADR; this is the
/// discoverable how-to.
const MODEL_ENV_HELP: &str = "\
Environment variables:
  GROVE_PLANNING_MODEL  Model for planning leaves (grilling / design).
  GROVE_WORK_MODEL      Model for work leaves (code / docs / tests).

The loop passes the matching value via Claude Code's native `--model` at each
task launch, keyed on the picked leaf's kind. Unset ⇒ no `--model`: the session
inherits your own default (ANTHROPIC_MODEL / Claude Code settings), so grove is
a no-op until you opt in and never clobbers a default you already set. Setting
only one variable is fine — the other kind still inherits. An in-session
`/model` switch overrides the launch model but does NOT persist across relaunch
(each task is a fresh session launched on its kind's default).

Example:
  GROVE_PLANNING_MODEL=opus GROVE_WORK_MODEL=sonnet grove do <name>";

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
    /// Migrate this worktree's `.grove/` to the v2 **directory** scheme
    /// (task-tree-scheme) in place — from either the v1-flat `<dotted>-[<key>]-<slug>`
    /// format or the old `NNN-slug/` + `done/` directory format. A reviewable git
    /// change (`git mv` + `# …` header rewrites, no commit) — review the diff,
    /// then commit. No-op on an already-v2 tree or a missing/foreign `.grove/`.
    /// `grove do` runs this automatically on adoption; invoke it explicitly to
    /// migrate a tree by hand.
    Migrate(MigrateArgs),
    /// Launch a session to retire a done node by hand (the loop normally retires
    /// leaves itself mid-session).
    Retire(RetireArgs),
}

#[derive(Parser)]
#[command(after_long_help = MODEL_ENV_HELP)]
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
        Command::Migrate(args) => crate::tree_migrate::run(&args),
        Command::Retire(args) => crate::launch::retire(&args),
    }
}

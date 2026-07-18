use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Extra `grove do --help` section documenting the per-kind model-selection env
/// vars (model-per-task-kind). The rationale lives in the ADR; this is the
/// discoverable how-to.
const MODEL_ENV_HELP: &str = "\
Environment variables:
  GROVE_PLANNING_MODEL   Model for planning leaves (grilling / design).
  GROVE_RESEARCH_MODEL   Model for research leaves (produces docs/research/*.md).
  GROVE_PROTOTYPE_MODEL  Model for prototype leaves (a cheap throwaway artifact).
  GROVE_WORK_MODEL       Model for work leaves (code / docs / tests).
  GROVE_REVIEW_MODEL     Model for review leaves (fresh-context adversarial read).

  GROVE_<HARNESS>_<KIND>_MODEL   Harness-scoped override (CLAUDE / CODEX / PI),
                         e.g. GROVE_PI_REVIEW_MODEL. Beats the base var for
                         that harness. Use when two harnesses run concurrently
                         and need different values for the same kind.
  GROVE_<KIND>_HARNESS   Route leaves of one kind to another harness,
                         e.g. GROVE_REVIEW_HARNESS=pi runs every review leaf
                         on pi whatever the grove's own harness is.

The loop passes the value via the harness's launch flag at each task launch,
keyed on the picked leaf's kind: `--model` for claude and pi (pi accepts
provider/id patterns), `--profile` for codex (a codex profile binds model +
reasoning effort — define profiles as $CODEX_HOME/<name>.config.toml files,
not a [profiles.<name>] table in config.toml). Unset ⇒ no flag:
the session inherits the harness's own default, so grove is a no-op until you
opt in and never clobbers a default you already set. Setting only some kinds
is fine — an unconfigured kind still inherits.

An in-session model switch outranks the launch flag for that one session
only; whether it persists into the next task depends on whether the next
launch passes a flag again (configured kind: yes, override gone; unconfigured
kind: the harness's own persistence rules apply).

Example:
  GROVE_CODEX_WORK_MODEL=sol-high GROVE_REVIEW_HARNESS=pi \\
  GROVE_PI_REVIEW_MODEL=kimi-code/k3 grove do";

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
    /// Start or continue a grove — the sole lifecycle entry verb, run from
    /// inside the working tree (any git working tree; the grove name is its
    /// basename — user-owned-worktrees).
    ///
    /// Inspects the grove's state and dispatches: no `.grove/` yet → open a
    /// bootstrap session; a live tree → continue. When the grove has no live
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
    /// Harness to launch: claude, codex, or pi (default: auto-detected from
    /// the repo's harness directories). Writes a lasting binding to
    /// `.grove-stamps/<name>`, read by every later `grove do`/`grove retire`
    /// for this grove until removed.
    #[arg(long = "harness")]
    pub harness: Option<String>,
    /// Report readiness but don't exec the harness.
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
pub struct RetireArgs {
    /// Node path within the current worktree's `.grove/` (e.g. `003-session-store`).
    pub path: String,
    /// Harness to launch: claude, codex, or pi (default: auto-detected from
    /// the repo's harness directories). Writes a lasting binding to
    /// `.grove-stamps/<name>`, read by every later `grove do`/`grove retire`
    /// for this grove until removed.
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

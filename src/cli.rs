use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Extra `grove do --help` section documenting the per-kind routing env vars —
/// both axes, which harness runs a leaf and which model that harness loads
/// (model-per-task-kind). The rationale lives in the ADR and the membership in
/// `docs/specs/task-kind-taxonomy.md`; this is the discoverable how-to.
const MODEL_ENV_HELP: &str = "\
Environment variables:
  Every leaf declares a KIND, drawn from a closed set of seventeen: five
  producers — requirements, design, planning, prototype, impl — each with
  its own review-<producer> and integrate-review-<producer> step, plus
  research and combine-research. Two FAMILIES group them for configuration:
  REVIEW covers all five review-* kinds, INTEGRATE_REVIEW all five
  integrate-review-* kinds. A var name uppercases the label and maps `-`
  to `_` (review-impl => REVIEW_IMPL).

  Which harness runs the leaf — leaf beats kind beats family beats stamp:
    <leaf>                 a `**Harness:** <name>` line on the task file
                           itself. Rare; it exists for the research vendor
                           pair, the one shape a per-kind policy cannot
                           express. Read strictly — an unknown name refuses
                           to launch rather than degrading.
    GROVE_<KIND>_HARNESS   e.g. GROVE_REVIEW_IMPL_HARNESS=codex.
    GROVE_<FAMILY>_HARNESS e.g. GROVE_REVIEW_HARNESS=codex — one line
                           covering all five review-* kinds.
    unset                  the harness this grove is STAMPED to
                           (.grove-stamps/<name>) — an explicit on-disk
                           binding, not a default.

  Which model that harness loads — four keys, harness-major:
    1. GROVE_<HARNESS>_<KIND>_MODEL     e.g. GROVE_CODEX_REVIEW_IMPL_MODEL
    2. GROVE_<HARNESS>_<FAMILY>_MODEL   e.g. GROVE_CODEX_REVIEW_MODEL
    3. GROVE_<KIND>_MODEL               e.g. GROVE_IMPL_MODEL
    4. GROVE_<FAMILY>_MODEL             e.g. GROVE_INTEGRATE_REVIEW_MODEL
  <HARNESS> is CLAUDE, CODEX or PI. The harness axis outranks the kind axis
  because crossing it can yield a value that is invalid, not merely less
  specific — a codex profile name is garbage to pi. A REROUTED launch (the
  leaf's harness is not the stamped one) consults keys 1-2 ONLY: an unscoped
  var was written with some other harness in mind.

A MODEL IS REQUIRED. If the picked leaf's kind resolves none of the four
keys, grove fails loudly and names every var that would satisfy it — it does
not fall through to the harness's own default, because that is still grove
choosing a model, only invisibly, and it makes a half-configured setup
indistinguishable from a complete one. Three exemptions, each an absence of
the question rather than a default: no live leaf (the finish-cycle
iteration); a harness with no model flag at all; and an unset
GROVE_<KIND>_HARNESS, which means the stamp, not a guess.

The value is passed via the harness's own launch flag: `--model` for claude
and pi (pi accepts provider/id patterns), `--profile` for codex (a codex
profile binds model + reasoning effort, which a bare model flag cannot
express — define profiles as $CODEX_HOME/<name>.config.toml files, not a
[profiles.<name>] table in config.toml).

An in-session model switch outranks the launch flag for that session only.
It does not survive into the next task: every launch passes a flag again.

Example — claude leads, codex reviews, claude integrates the review. The
stamp absorbs every kind that is not rerouted, so the whole policy layer is
two lines, and one FAMILY var does the work of five kinds:
  GROVE_REVIEW_HARNESS=codex GROVE_CODEX_REVIEW_MODEL=sol-xhigh \\
  GROVE_REQUIREMENTS_MODEL=opus GROVE_DESIGN_MODEL=opus \\
  GROVE_PLANNING_MODEL=opus GROVE_PROTOTYPE_MODEL=sonnet \\
  GROVE_IMPL_MODEL=sonnet GROVE_RESEARCH_MODEL=opus \\
  GROVE_COMBINE_RESEARCH_MODEL=opus GROVE_INTEGRATE_REVIEW_MODEL=opus \\
  grove do";

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
    /// inside the working tree (any git or jj working tree; the grove name is
    /// its basename — user-owned-worktrees).
    ///
    /// Inspects the grove's state and dispatches: no `.grove/` yet → open a
    /// bootstrap session; a live tree → continue. When the grove has no live
    /// leaves left, the in-session loop proposes the complete finish cycle.
    Do(StartArgs),
    /// Migrate this worktree's `.grove/` to the v2 **directory** scheme
    /// (task-tree-scheme) in place — from either the v1-flat `<dotted>-[<key>]-<slug>`
    /// format or the old `NNN-slug/` + `done/` directory format. A reviewable
    /// working-tree change (file moves + `# …` header rewrites, no commit) —
    /// review the diff, then commit. No-op on an already-v2 tree or a missing/foreign `.grove/`.
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
    /// Harness to launch: claude, codex, or pi. Defaults to the harness this
    /// grove is stamped to, else auto-detection from the repo's harness
    /// directories.
    ///
    /// Writes a lasting binding to `.grove-stamps/<name>`, read by every later
    /// `grove do`/`grove retire` for this grove until removed. `grove do` is
    /// the only verb that writes it: `grove retire --harness` picks a harness
    /// for its one session and leaves the binding alone.
    #[arg(long = "harness")]
    pub harness: Option<String>,
    /// Report launch readiness and exit without exec'ing the harness: names the
    /// next leaf, its kind, the harness that kind routes to and the model that
    /// harness would load — and fails, naming the same variables a real launch
    /// would, when any of that is unconfigured. Writes no stamp.
    #[arg(long = "no-launch")]
    pub no_launch: bool,
}

#[derive(Parser)]
pub struct MigrateArgs {
    /// Worktree whose `.grove/` to migrate (default: the current worktree's
    /// root — `git rev-parse --show-toplevel`, or the jj workspace root in a
    /// jj-enabled tree).
    pub path: Option<PathBuf>,
}

#[derive(Parser)]
pub struct RetireArgs {
    /// The node to retire, addressed inside the current worktree's `.grove/`:
    /// either its relative directory path — `04-session-store-k7`, or
    /// `04-design-k7/02-store-k9` for a node nested in another — or its stable
    /// `<slug>-k<key>` handle, `session-store-k7`, which the session resolves
    /// with `grove-llm resolve` (task-tree-scheme §5). Taken verbatim: nothing
    /// here parses it.
    pub path: String,
    /// Harness to launch: claude, codex, or pi. Defaults to the harness this
    /// grove is stamped to (`.grove-stamps/<name>`), else auto-detection from
    /// the repo's harness directories.
    ///
    /// Selects the harness for **this session only** — unlike `grove do
    /// --harness`, it writes no stamp. The stamp binds the grove, and the grove
    /// is bound by the verb that drives it: retiring one node by hand is no
    /// reason to rebind every later `grove do`. Pass `--harness` to `grove do`
    /// to make a binding last.
    #[arg(long = "harness")]
    pub harness: Option<String>,
    /// Report launch readiness and exit without exec'ing the harness. Readiness
    /// is a **checked** claim, as on `grove do`: the flag resolves everything
    /// the real launch would — the harness, its `retire` prompt (which this
    /// verb, unlike `grove do`, never provisions), the codex sandbox pre-flight
    /// and the VCS-store grants — and fails wherever that launch would.
    ///
    /// It names no leaf, kind or model, because `grove retire` resolves none:
    /// it peeks no leaf and passes the harness no model, so the per-kind
    /// routing `grove do --no-launch` reports on has no counterpart here.
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

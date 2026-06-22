// The LLM-driven CLI surface — `grove-llm`. See ADR-0006
// (`docs/adr/0006-grove-llm-binary-separation.md`) for the audience-split
// rationale: every verb here exists for the LLM driving a grove session to
// invoke deterministically, not for a human at a terminal.
//
// Verbs are flat (hyphenated) so a single `grove-llm --help` enumerates every
// verb the LLM might call — important for bootstrap-recovery if a session
// drops context (parent BRIEF Q3 of leaf 080).
//
// The task-tree verbs (`pick` / `brief-chain` / `resolve` / `leaf-add` /
// `leaf-insert` / `leaf-decompose` / `leaf-retire` / `root-init`) are
// **new-format-only** (ADR-0034): they dispatch to the flat dotted-decimal
// modules `leaf_read` / `leaf_grow` / `leaf_lifecycle` (built in 050, ADR-0033).
// There is no transitional dual-format reader — `grove migrate` (060/020) is the
// only thing that reads the old `NNN-slug` directory format, once, on adoption.

use crate::complete;
use crate::leaf::Kind;
use crate::leaf_grow;
use crate::leaf_lifecycle;
use crate::leaf_read;
use crate::repo;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "grove-llm",
    version,
    about = "Grove: LLM-driven verbs for mid-session use",
    long_about = "Verbs the LLM driving a grove session invokes deterministically. \
Audience-split from the human-facing `grove` binary per ADR-0006; \
none of these verbs are meant for direct human use."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scaffold a brand-new grove's tree: create `.grove/`, write the root
    /// `BRIEF.md` stub, and lay down a first planning leaf `1-[<key>]-<slug>.md`
    /// (default slug `plan`). After this, `grove-llm pick` returns the new
    /// leaf — a fresh grove is no longer indistinguishable from a finished
    /// one. Refuses if `.grove/` already exists. Working-tree change only —
    /// no commit.
    RootInit(RootInitArgs),
    /// Print the absolute path of the next live leaf in this grove's tree —
    /// a single version-sorted scan of the flat `.grove/` list (dotted-decimal
    /// position order), skipping briefs and retired (`.DONE`) leaves. Empty
    /// stdout (and a diagnostic on stderr) when the grove has no live leaves.
    Pick,
    /// Print the BRIEF.md chain for a leaf, root→leaf, one absolute path per
    /// line. Collects the brief at each proper-prefix position of the leaf's
    /// dotted id (a leaf at `[a,b,c]` pulls the briefs at `[]`, `[a]`, `[a,b]`).
    /// With no argument the chain is computed for `pick`'s next leaf. A missing
    /// brief at any level is skipped silently.
    BriefChain {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
    },
    /// Resolve a reference to its current file path, searching live **and**
    /// retired (`.DONE`) files. A permanent key (`[n]` or bare `n`, optionally
    /// `[n]-slug`) resolves the unique keyed file; a bare slug resolves by slug
    /// (0 ⇒ not found, 1 ⇒ that file, >1 ⇒ ambiguous, listing each match's key
    /// so you re-query by key). Prints the path on stdout; a not-found or
    /// ambiguous reference prints a diagnostic on stderr and still exits zero.
    Resolve {
        /// The reference: `[n]` / `n` / `[n]-slug` (by permanent key) or a bare
        /// slug.
        reference: String,
    },
    /// Append a child leaf under node `<parent-id>` (root parent `.`) at the
    /// next gapless child position, with a fresh permanent `[key]`. Prints the
    /// new leaf's absolute path on stdout. Working-tree change only — no commit.
    LeafAdd(LeafAddArgs),
    /// Insert a new leaf at exactly `<target-id>`, shifting the occupant and
    /// every later sibling up by one — a shift that cascades through whole
    /// subtrees (`2.2`→`2.3` drags `2.2.1`→`2.3.1`…). Rewrites only the dotted
    /// position in filenames and `# …` headers, never the `[key]` or slug. The
    /// new leaf gets a fresh key. Prints the new leaf's absolute path on stdout;
    /// the renumber summary and stray positional cross-references go to stderr.
    /// Working-tree change only — no commit.
    LeafInsert(LeafInsertArgs),
    /// Convert a live leaf `<id>-[k]-<slug>.md` into a node brief
    /// `<id>-[k]-<slug>.BRIEF.md` (**key `[k]` preserved**), retitle its `# …`
    /// header with ` — brief`, and atomically grow the first child
    /// `<id>.1-[new]-<first-child-slug>.md` so the node is never childless.
    /// Prints the brief's absolute path then the first child's, one per line.
    /// Working-tree change only — no commit.
    LeafDecompose(LeafDecomposeArgs),
    /// Mark a live leaf retired in place by appending `.DONE`
    /// (`<id>-[k]-<slug>.md` → `…DONE.md`) — no `done/` directory; the retired
    /// leaf keeps its position and key in the flat list. Refuses a brief and an
    /// already-retired leaf. Prints the retired file's absolute path on stdout.
    /// Working-tree change only — no commit.
    LeafRetire(LeafRetireArgs),
    /// Signal task completion to the self-driving loop (ADR-0032). Run this as
    /// the **last step** of a task, after commit + retire — it is how the loop
    /// ends this `claude` session and starts the next task with fresh context.
    ///
    /// Writes the relaunch flag, then forks a detached killer that ends this
    /// session after a short grace (so this very call returns first). Do
    /// nothing else after running it. Defaults come from the loop driver's
    /// environment (`GROVE_CLAUDE_PID`, `GROVE_SIGNAL_FILE`); when those are
    /// absent (a session not under `grove do`) it is a safe near-no-op that
    /// just tells you to exit manually.
    Complete(CompleteArgs),
}

#[derive(Parser)]
pub struct CompleteArgs {
    /// PID of the `claude` session to end. Default: `$GROVE_CLAUDE_PID`.
    #[arg(long)]
    pub pid: Option<i32>,
    /// Relaunch-signal file the loop driver polls. Default: `$GROVE_SIGNAL_FILE`.
    #[arg(long = "signal-file")]
    pub signal_file: Option<PathBuf>,
    /// Seconds before SIGTERM. Default: `$GROVE_KILL_GRACE` or 2.
    #[arg(long)]
    pub grace: Option<f64>,
    /// Seconds after SIGTERM before SIGKILL. Default: `$GROVE_KILL_GRACE_KILL` or 5.
    #[arg(long = "kill-grace")]
    pub kill_grace: Option<f64>,
}

#[derive(Parser)]
pub struct RootInitArgs {
    /// Slug for the first planning leaf (lowercase ASCII letters, digits,
    /// dashes). Default: `plan`. Mirrors `leaf-add . <slug>`.
    #[arg(default_value = "plan")]
    pub slug: String,
}

#[derive(Parser)]
pub struct LeafAddArgs {
    /// Parent node id — a dotted position (`2.3`) or `.` for the grove root.
    pub parent_id: String,
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    /// Leaf kind, written into the templated `**Kind:**` line.
    #[arg(long = "kind", default_value = "work")]
    pub kind: String,
}

#[derive(Parser)]
pub struct LeafInsertArgs {
    /// Target id — the dotted position (`2.3`) the new leaf should occupy; the
    /// current occupant and later siblings shift up by one.
    pub target_id: String,
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    /// Leaf kind, written into the templated `**Kind:**` line.
    #[arg(long = "kind", default_value = "work")]
    pub kind: String,
}

#[derive(Parser)]
pub struct LeafDecomposeArgs {
    /// Leaf path. Absolute, or relative to the grove root (`.grove/`).
    pub leaf_path: PathBuf,
    /// Slug for the node's first child (lowercase ASCII letters, digits,
    /// dashes).
    pub first_child_slug: String,
    /// First child's kind, written into its templated `**Kind:**` line.
    #[arg(long = "kind", default_value = "work")]
    pub kind: String,
}

#[derive(Parser)]
pub struct LeafRetireArgs {
    /// Leaf path. Absolute, or relative to the grove root (`.grove/`).
    pub leaf_path: PathBuf,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::RootInit(args) => cmd_root_init(&args),
        Command::Pick => cmd_pick(),
        Command::BriefChain { leaf_path } => cmd_brief_chain(leaf_path.as_deref()),
        Command::Resolve { reference } => cmd_resolve(&reference),
        Command::LeafAdd(args) => cmd_leaf_add(&args),
        Command::LeafInsert(args) => cmd_leaf_insert(&args),
        Command::LeafDecompose(args) => cmd_leaf_decompose(&args),
        Command::LeafRetire(args) => cmd_leaf_retire(&args),
        Command::Complete(args) => cmd_complete(&args),
    }
}

fn cmd_complete(args: &CompleteArgs) -> Result<()> {
    let opts = complete::resolve_opts(
        args.pid,
        args.signal_file.clone(),
        args.grace,
        args.kill_grace,
    );
    complete::signal_complete(&opts)
}

fn cmd_root_init(args: &RootInitArgs) -> Result<()> {
    let (worktree, _) = grove_paths()?;
    let paths = leaf_lifecycle::root_init(&worktree, &args.slug)?;
    for p in &paths {
        println!("{}", p.display());
    }
    Ok(())
}

fn cmd_pick() -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    match leaf_read::pick(&grove_root)? {
        Some(p) => {
            println!("{}", p.display());
        }
        None => {
            eprintln!(
                "grove {}: no live leaves; this grove is done",
                label(&worktree)
            );
        }
    }
    Ok(())
}

fn cmd_brief_chain(leaf_path: Option<&Path>) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    let leaf: PathBuf = match leaf_path {
        Some(p) => normalize_leaf_path(p),
        None => match leaf_read::pick(&grove_root)? {
            Some(p) => p,
            None => {
                eprintln!(
                    "grove {}: no live leaves; this grove is done",
                    label(&worktree)
                );
                return Ok(());
            }
        },
    };
    let chain = leaf_read::brief_chain(&grove_root, &leaf)?;
    for p in &chain {
        println!("{}", p.display());
    }
    Ok(())
}

fn cmd_resolve(reference: &str) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let resolution = leaf_read::resolve(&grove_root, reference)?;
    let (stdout, stderr) = leaf_read::render_resolution(reference, &resolution);
    // `resolve` is pick-style: a not-found / ambiguous reference is reported on
    // stderr and still exits zero (it is information, not an error).
    print!("{stdout}");
    eprint!("{stderr}");
    Ok(())
}

fn cmd_leaf_add(args: &LeafAddArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    let path = leaf_grow::leaf_add(&grove_root, &args.parent_id, &args.slug, kind)?;
    println!("{}", path.display());
    Ok(())
}

fn cmd_leaf_insert(args: &LeafInsertArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    let (path, renumbers) = leaf_grow::leaf_insert(&grove_root, &args.target_id, &args.slug, kind)?;

    // The new leaf's path is the only stdout content; the renumber summary and
    // cross-references go to stderr so the LLM can parse stdout cleanly.
    println!("{}", path.display());
    let mut stderr = std::io::stderr();
    if renumbers.is_empty() {
        eprintln!(
            "leaf-insert {}-{}: no siblings to renumber",
            args.target_id, args.slug
        );
    } else {
        eprintln!(
            "leaf-insert {}-{}: renumbered {} sibling{}:",
            args.target_id,
            args.slug,
            renumbers.len(),
            if renumbers.len() == 1 { "" } else { "s" }
        );
        for r in &renumbers {
            eprintln!(
                "  {} -> {}  ({})",
                dotted(&r.old_position),
                dotted(&r.new_position),
                r.new_name
            );
        }
        eprintln!("cross-references to review (verb does not auto-rewrite):");
        leaf_grow::surface_cross_refs(&grove_root, &renumbers, &mut stderr)?;
    }
    Ok(())
}

fn cmd_leaf_decompose(args: &LeafDecomposeArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let (brief_path, child_path) =
        leaf_lifecycle::leaf_decompose(&grove_root, &leaf_path, &args.first_child_slug, kind)?;
    println!("{}", brief_path.display());
    println!("{}", child_path.display());
    Ok(())
}

fn cmd_leaf_retire(args: &LeafRetireArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let dst = leaf_lifecycle::leaf_retire(&grove_root, &leaf_path)?;
    println!("{}", dst.display());
    Ok(())
}

// Resolve `(worktree, grove_root)` from the cwd. The task-tree verbs run from
// the worktree root (not from inside `.grove/`), so the grove root is always
// `<worktree>/.grove`, matched across every verb so a leaf is never created or
// read one level off.
fn grove_paths() -> Result<(PathBuf, PathBuf)> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let worktree = repo::git_toplevel(&cwd)?;
    let grove_root = worktree.join(".grove");
    Ok((worktree, grove_root))
}

// Normalize a user-supplied leaf path to what the new-format modules accept
// (absolute, or relative to the grove root). The real driving flow passes back
// the **absolute** path `pick`/`brief-chain` printed — handled by the absolute
// branch. The convenience cases: a path given relative to the cwd that exists
// (e.g. `.grove/2-[3]-foo.md` typed at the worktree root) is resolved to
// absolute here; a bare grove-root-relative name (`2-[3]-foo.md`) does not exist
// relative to cwd, so it passes through for the module to join onto the grove
// root. This preserves the old worktree-relative ergonomics without widening the
// module contract (which the 050 unit tests pin to absolute-or-grove-relative).
fn normalize_leaf_path(p: &Path) -> PathBuf {
    if p.is_absolute() {
        return p.to_path_buf();
    }
    match std::env::current_dir() {
        Ok(cwd) => {
            let cwd_rel = cwd.join(p);
            if cwd_rel.exists() {
                cwd_rel
            } else {
                p.to_path_buf()
            }
        }
        Err(_) => p.to_path_buf(),
    }
}

// The grove's display label for the pick/brief-chain "no live leaves" diagnostic
// — the worktree directory's basename (it equals the grove name / branch).
fn label(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| worktree.display().to_string())
}

// Render a dotted-decimal position vector (`[2,3,1]` → `2.3.1`) for the
// leaf-insert renumber summary on stderr.
fn dotted(position: &[u32]) -> String {
    position
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

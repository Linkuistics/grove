// The LLM-driven CLI surface — `grove-llm`. See cli-binary-split
// (`docs/adr/cli-binary-split.md`) for the audience-split
// rationale: every verb here exists for the LLM driving a grove session to
// invoke deterministically, not for a human at a terminal.
//
// Verbs are flat (hyphenated) so a single `grove-llm --help` enumerates every
// verb the LLM might call — important for bootstrap-recovery if a session
// drops context.
//
// The task-tree verbs (`pick` / `brief-chain` / `resolve` / `leaf-add` /
// `leaf-insert` / `leaf-decompose` / `leaf-retire` / `root-init`) speak the
// **v2 directory scheme** (task-tree-scheme): they dispatch to the directory-based
// modules `tree_read` / `tree_grow` / `tree_lifecycle`. There is no
// transitional dual-format reader — `grove migrate` (`tree_migrate`) is the only
// thing that reads an old (v1-flat or `NNN-slug`) tree, once, on adoption.

use crate::complete;
use crate::leaf::Kind;
use crate::repo;
use crate::tree_grow;
use crate::tree_lifecycle;
use crate::tree_read::{self, Resolution};
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "grove-llm",
    version,
    about = "Grove: LLM-driven verbs for mid-session use",
    long_about = "Verbs the LLM driving a grove session invokes deterministically. \
Audience-split from the human-facing `grove` binary per cli-binary-split; \
none of these verbs are meant for direct human use."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scaffold a brand-new grove's tree: create `.grove/`, write the root
    /// `BRIEF.md` charter, and lay down a first planning leaf `01-<slug>-k1.md`
    /// (default slug `plan`). After this, `grove-llm pick` returns the new
    /// leaf — a fresh grove is no longer indistinguishable from a finished
    /// one. Refuses if `.grove/` already exists. Working-tree change only —
    /// no commit.
    RootInit(RootInitArgs),
    /// Print the absolute path of the next live leaf in this grove's tree — a
    /// recursive depth-first **pre-order** walk over the directory tree (a node
    /// is a directory holding `BRIEF.md` + numbered children), returning the
    /// first live leaf and skipping briefs and retired (`DONE`) leaves. Empty
    /// stdout (and a diagnostic on stderr) when the grove has no live leaves.
    Pick,
    /// Print the BRIEF.md chain for a leaf, root→leaf, one absolute path per
    /// line — the `BRIEF.md` of each of the leaf's ancestor **directories**,
    /// from the grove root down to the leaf's containing directory. With no
    /// argument the chain is computed for `pick`'s next leaf. A directory level
    /// with no `BRIEF.md` is skipped silently.
    BriefChain {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
    },
    /// Print a leaf's task **kind** — `planning` or `work` — read from its
    /// `**Kind:**` line. With no argument the kind is read for `pick`'s next
    /// live leaf; on an empty grove it prints the standard "no live leaves"
    /// diagnostic on stderr (mirroring `brief-chain`) and exits 0. The output
    /// is a single lowercase token + newline. The self-driving loop calls this
    /// to choose each session's launch model by the picked leaf's kind
    /// (model-per-task-kind).
    Kind {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
    },
    /// Resolve a reference to its current file path, searching live **and**
    /// retired (`DONE`) entries across the whole directory tree. A permanent key
    /// (`[n]` or bare `n`, optionally `[n]-slug`) resolves the unique keyed
    /// entry; a bare slug resolves by slug (0 ⇒ not found, 1 ⇒ that entry, >1 ⇒
    /// ambiguous, listing each match's key so you re-query by key); the full
    /// `<slug>-k<key>` handle (task-tree-scheme §5) resolves by its terminal key. A node
    /// resolves to its **directory** path (append `/BRIEF.md` to read its
    /// charter). Prints the path on stdout; a not-found or ambiguous reference
    /// prints a diagnostic on stderr and still exits zero.
    Resolve {
        /// The reference: `[n]` / `n` / `[n]-slug` / `<slug>-k<key>` (by
        /// permanent key) or a bare slug.
        reference: String,
    },
    /// Append a child leaf under `<parent>` at the next gapless child position,
    /// with a fresh permanent key. `<parent>` is `.` for the grove root, or a
    /// node by its key (`[n]` / `n` / `<slug>-k<key>`) or its path. Prints the
    /// new leaf's absolute path on stdout. Working-tree change only — no commit.
    LeafAdd(LeafAddArgs),
    /// Insert a new leaf at the slot held by `<target>`, shifting `<target>` and
    /// every later sibling up one position. `<target>` is an existing leaf or
    /// node by its key or path. Because the hierarchy lives in directories, each
    /// shift is a single `git mv` of one sibling directory and the whole subtree
    /// — child names *and* keys — rides along untouched; in-file `# …` headers
    /// are position-free, so **zero file contents** are rewritten. The new leaf
    /// gets a fresh key. Prints the new leaf's absolute path on stdout; the
    /// renumber summary and stray position-prefixed cross-references go to
    /// stderr. Working-tree change only — no commit.
    LeafInsert(LeafInsertArgs),
    /// Convert a live leaf file `NN-<slug>-k<key>.md` into a node **directory**
    /// `NN-<slug>-k<key>/` (**key preserved** — the leaf that was `k<key>`
    /// becomes the node `k<key>`), `git mv`ing the leaf body in as the node's
    /// `BRIEF.md` (its `# <slug>-k<key>` header retitled with ` — brief`) and
    /// atomically growing a first child `01-<first-child-slug>-k<new>.md` so the
    /// node is never childless. Prints the brief's absolute path then the first
    /// child's, one per line. Working-tree change only — no commit.
    LeafDecompose(LeafDecomposeArgs),
    /// Mark a live leaf retired in place by adding a `DONE` infix
    /// (`NN-<slug>-k<key>.md` → `NN-DONE-<slug>-k<key>.md`) — no `done/`
    /// directory; the leaf keeps its position and key in its directory, and the
    /// file's contents (its `# <slug>-k<key>` header) are untouched. Refuses a
    /// brief and an already-retired leaf. Prints the retired file's absolute
    /// path on stdout. Working-tree change only — no commit.
    LeafRetire(LeafRetireArgs),
    /// Signal task completion to the self-driving loop (self-driving-loop). Run this as
    /// the **last step** of a task, after commit + retire — it is how the loop
    /// ends this `claude` session and starts the next task with fresh context.
    ///
    /// Writes the disposition flag (relaunch by default, or finish with
    /// `--done`), then forks a detached killer that ends this session after a
    /// short grace (so this very call returns first). Do nothing else after
    /// running it. The default relaunches the loop for the next task; `--done`
    /// — the Finish cycle's last action — stops it cleanly. Defaults come from
    /// the loop driver's environment (`GROVE_CLAUDE_PID`, `GROVE_SIGNAL_FILE`);
    /// when those are absent (a session not under `grove do`) it is a safe
    /// near-no-op that just tells you to exit manually.
    Complete(CompleteArgs),
}

#[derive(Parser)]
pub struct CompleteArgs {
    /// Finish the whole grove instead of relaunching: signal the loop to stop
    /// cleanly. Use as the **last** action of the Finish cycle. Without it
    /// (the per-task default) the loop relaunches with fresh context.
    #[arg(long)]
    pub done: bool,
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
    /// Parent node — `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path.
    pub parent: String,
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    /// Leaf kind, written into the templated `**Kind:**` line.
    #[arg(long = "kind", default_value = "work")]
    pub kind: String,
}

#[derive(Parser)]
pub struct LeafInsertArgs {
    /// Target entry — the existing leaf or node (by key or path) whose slot the
    /// new leaf should occupy; the target and later siblings shift up by one.
    pub target: String,
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
        Command::Kind { leaf_path } => cmd_kind(leaf_path.as_deref()),
        Command::Resolve { reference } => cmd_resolve(&reference),
        Command::LeafAdd(args) => cmd_leaf_add(&args),
        Command::LeafInsert(args) => cmd_leaf_insert(&args),
        Command::LeafDecompose(args) => cmd_leaf_decompose(&args),
        Command::LeafRetire(args) => cmd_leaf_retire(&args),
        Command::Complete(args) => cmd_complete(&args),
    }
}

fn cmd_complete(args: &CompleteArgs) -> Result<()> {
    let disposition = if args.done {
        complete::Disposition::Done
    } else {
        complete::Disposition::Relaunch
    };
    let opts = complete::resolve_opts(
        args.pid,
        args.signal_file.clone(),
        args.grace,
        args.kill_grace,
        disposition,
    );
    complete::signal_complete(&opts)
}

fn cmd_root_init(args: &RootInitArgs) -> Result<()> {
    let (worktree, _) = grove_paths()?;
    let paths = tree_lifecycle::root_init(&worktree, &args.slug)?;
    for p in &paths {
        println!("{}", p.display());
    }
    Ok(())
}

fn cmd_pick() -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    match tree_read::pick(&grove_root)? {
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
        None => match tree_read::pick(&grove_root)? {
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
    let chain = tree_read::brief_chain(&grove_root, &leaf)?;
    for p in &chain {
        println!("{}", p.display());
    }
    Ok(())
}

fn cmd_kind(leaf_path: Option<&Path>) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    // Normalize a cwd-relative path to what `tree_read::kind` accepts (absolute
    // or grove-root-relative), matching `cmd_brief_chain`'s handling; a `None`
    // stays `None` so the verb defaults to `pick`'s next live leaf.
    let leaf = leaf_path.map(normalize_leaf_path);
    match tree_read::kind(&grove_root, leaf.as_deref())? {
        Some(k) => println!("{}", k.label()),
        None => eprintln!(
            "grove {}: no live leaves; this grove is done",
            label(&worktree)
        ),
    }
    Ok(())
}

fn cmd_resolve(reference: &str) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let resolution = tree_read::resolve(&grove_root, reference)?;
    let (stdout, stderr) = tree_read::render_resolution(reference, &resolution);
    // `resolve` is pick-style: a not-found / ambiguous reference is reported on
    // stderr and still exits zero (it is information, not an error).
    print!("{stdout}");
    eprint!("{stderr}");
    Ok(())
}

fn cmd_leaf_add(args: &LeafAddArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    // `.` is the grove root (the root node); anything else is a node by key or
    // path (a v2 node is a directory). `tree_grow::leaf_add` validates that the
    // resolved parent is a real node directory.
    let parent_dir = if args.parent == "." {
        grove_root.clone()
    } else {
        resolve_ref_or_path(&grove_root, &args.parent)?
    };
    let path = tree_grow::leaf_add(&grove_root, &parent_dir, &args.slug, kind)?;
    println!("{}", path.display());
    Ok(())
}

fn cmd_leaf_insert(args: &LeafInsertArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    let target = resolve_ref_or_path(&grove_root, &args.target)?;
    let (path, renumbers) = tree_grow::leaf_insert(&grove_root, &target, &args.slug, kind)?;

    // The new leaf's path is the only stdout content; the renumber summary and
    // cross-references go to stderr so the LLM can parse stdout cleanly.
    println!("{}", path.display());
    let mut stderr = std::io::stderr();
    if renumbers.is_empty() {
        eprintln!("leaf-insert {}: no siblings to renumber", args.slug);
    } else {
        eprintln!(
            "leaf-insert {}: renumbered {} sibling{}:",
            args.slug,
            renumbers.len(),
            if renumbers.len() == 1 { "" } else { "s" }
        );
        for r in &renumbers {
            eprintln!(
                "  {:02} -> {:02}  ({})",
                r.old_position, r.new_position, r.new_name
            );
        }
        eprintln!("cross-references to review (verb does not auto-rewrite):");
        tree_grow::surface_cross_refs(&grove_root, &renumbers, &mut stderr)?;
    }
    Ok(())
}

fn cmd_leaf_decompose(args: &LeafDecomposeArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let (brief_path, child_path) =
        tree_lifecycle::leaf_decompose(&grove_root, &leaf_path, &args.first_child_slug, kind)?;
    println!("{}", brief_path.display());
    println!("{}", child_path.display());
    Ok(())
}

fn cmd_leaf_retire(args: &LeafRetireArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let dst = tree_lifecycle::leaf_retire(&grove_root, &leaf_path)?;
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

// Map a `<parent>` / `<target>` argument — a `resolve`-able key/slug/handle OR a
// path — to an absolute entry path. The driving flow passes back the **absolute**
// path `pick`/`resolve` printed (the path branch); a key/slug/handle is the
// convenience (the reference branch). Tried as a path first so an explicit,
// existing path always wins; only a non-existent path is re-tried as a reference,
// so the two namespaces never collide in practice.
fn resolve_ref_or_path(grove_root: &Path, arg: &str) -> Result<PathBuf> {
    if let Some(path) = existing_path(grove_root, arg) {
        return Ok(path);
    }
    match tree_read::resolve(grove_root, arg)? {
        Resolution::Found { path, .. } => Ok(path),
        Resolution::NotFound => bail!(
            "no entry matches {arg:?} (tried as a path under the grove root and as a key/slug)"
        ),
        Resolution::Ambiguous(matches) => {
            let keys = matches
                .iter()
                .map(|m| format!("[{}]", m.key))
                .collect::<Vec<_>>()
                .join(", ");
            bail!("reference {arg:?} is ambiguous; re-query by key: {keys}")
        }
    }
}

// Interpret `arg` as a path that actually exists: absolute, or relative to the
// grove root, or relative to the cwd. `None` if no such path exists (then it is a
// key/slug reference). This preserves the "pass back what pick/resolve printed"
// ergonomics (absolute) and the worktree-relative convenience.
fn existing_path(grove_root: &Path, arg: &str) -> Option<PathBuf> {
    let p = Path::new(arg);
    if p.is_absolute() {
        return p.exists().then(|| p.to_path_buf());
    }
    let grove_rel = grove_root.join(p);
    if grove_rel.exists() {
        return Some(grove_rel);
    }
    let cwd_rel = std::env::current_dir().ok()?.join(p);
    cwd_rel.exists().then_some(cwd_rel)
}

// Normalize a user-supplied leaf path to what the v2 modules accept (absolute, or
// relative to the grove root). The real driving flow passes back the **absolute**
// path `pick`/`brief-chain` printed — handled by the absolute branch. A path given
// relative to the cwd that exists (e.g. `.grove/01-foo-k1.md` typed at the
// worktree root) is resolved to absolute here; a bare grove-root-relative name
// (`01-foo-k1.md`) does not exist relative to cwd, so it passes through for the
// module to join onto the grove root.
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

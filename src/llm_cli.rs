// The LLM-driven CLI surface — `grove-llm`. See ADR-0006
// (`docs/adr/0006-grove-llm-binary-separation.md`) for the audience-split
// rationale: every verb here exists for the LLM driving a grove session to
// invoke deterministically, not for a human at a terminal.
//
// Verbs are flat (hyphenated) so a single `grove-llm --help` enumerates every
// verb the LLM might call — important for bootstrap-recovery if a session
// drops context (parent BRIEF Q3 of leaf 080).

use crate::brief_chain;
use crate::cli::{InboxAddArgs, InboxDrainArgs};
use crate::inboxes;
use crate::leaf;
use crate::leaf_ops;
use crate::pick;
use crate::repo;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::Read;
use std::path::PathBuf;

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
    /// Append an observation to the inbox of `<name>` on the `grove-meta` branch.
    InboxAdd(InboxAddArgs),
    /// Drain the inbox of `<name>`: two-phase. No disposition flags →
    /// enumerate pending observation paths. Any
    /// `--incorporated`/`--deferred`/`--rejected` paths → finalize by
    /// deleting the triaged files in one commit.
    InboxDrain(InboxDrainArgs),
    /// Print the absolute path of the next live leaf in this grove's tree —
    /// depth-first walk of `.grove/` in numeric-prefix order, skipping
    /// `done/`. Empty stdout (and a diagnostic on stderr) when the grove has
    /// no live leaves.
    Pick,
    /// Print the BRIEF.md chain for a leaf, root→leaf, one absolute path per
    /// line. Walks ancestor directories from the leaf up to the grove root,
    /// collecting any BRIEF.md found at each level. With no argument the
    /// chain is computed for `pick`'s next leaf. A missing BRIEF.md at any
    /// level is skipped silently.
    BriefChain {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
    },
    /// Append a new leaf at the next free numeric prefix in the target
    /// node (or at `--prefix` if free). Prints the absolute path of the
    /// new leaf on stdout. Working-tree change only — no commit.
    LeafAdd(LeafAddArgs),
    /// Insert a new leaf at the given prefix in the target node, shifting
    /// every sibling at or after that prefix up by 10. `git mv`s affected
    /// siblings, rewrites their `# NNN-...` first-line headers, and
    /// surfaces numeric cross-references on stderr for review. Prints the
    /// new leaf's absolute path on stdout. Working-tree change only —
    /// no commit.
    LeafInsert(LeafInsertArgs),
    /// Convert a leaf file `NNN-x.md` into a node directory `NNN-x/`
    /// containing `BRIEF.md` seeded from the leaf's body. The first-line
    /// `# NNN-x` header is retitled to `# NNN-x — brief`; further reshape
    /// is the LLM's call. Children are added afterwards via `leaf-add`.
    /// Prints the new `BRIEF.md`'s absolute path on stdout. Working-tree
    /// change only — no commit.
    LeafDecompose(LeafDecomposeArgs),
    /// Move a single leaf into `.grove/done/`, preserving its relative path
    /// inside `.grove/`. Pure mechanics — the parent-chain cascade (ask
    /// before retiring each empty node, promote brief content upward)
    /// stays prose. Prints the destination path on stdout. Working-tree
    /// change only — no commit.
    LeafRetire(LeafRetireArgs),
}

#[derive(Parser)]
pub struct LeafAddArgs {
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    /// Explicit three-digit prefix (e.g. `050`). Default: next free prefix
    /// in the target node.
    #[arg(long = "prefix")]
    pub prefix: Option<String>,
    /// Leaf kind, written into the templated `**Kind:**` line.
    #[arg(long = "kind", default_value = "work")]
    pub kind: String,
    /// Target node directory. Default: current working directory.
    #[arg(long = "node")]
    pub node: Option<PathBuf>,
}

#[derive(Parser)]
pub struct LeafInsertArgs {
    /// `<prefix>-<slug>`, e.g. `050-foo-bar`.
    pub prefix_slug: String,
    /// Leaf kind, written into the templated `**Kind:**` line.
    #[arg(long = "kind", default_value = "work")]
    pub kind: String,
    /// Target node directory. Default: current working directory.
    #[arg(long = "node")]
    pub node: Option<PathBuf>,
}

#[derive(Parser)]
pub struct LeafDecomposeArgs {
    /// Leaf path. Absolute, relative to the cwd, or relative to the grove
    /// root (`.grove/`).
    pub leaf_path: PathBuf,
}

#[derive(Parser)]
pub struct LeafRetireArgs {
    /// Leaf path. Absolute, relative to the cwd, or relative to the grove
    /// root (`.grove/`).
    pub leaf_path: PathBuf,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::InboxAdd(args) => cmd_inbox_add(&args),
        Command::InboxDrain(args) => cmd_inbox_drain(&args),
        Command::Pick => cmd_pick(),
        Command::BriefChain { leaf_path } => cmd_brief_chain(leaf_path.as_deref()),
        Command::LeafAdd(args) => cmd_leaf_add(&args),
        Command::LeafInsert(args) => cmd_leaf_insert(&args),
        Command::LeafDecompose(args) => cmd_leaf_decompose(&args),
        Command::LeafRetire(args) => cmd_leaf_retire(&args),
    }
}

fn cmd_inbox_add(args: &InboxAddArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let observation = read_body(args)?;
    inboxes::capture(&repo_path, &args.to, &observation, args.slug.as_deref())
}

fn cmd_inbox_drain(args: &InboxDrainArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let name = &args.for_grove;
    let has_dispositions =
        !args.incorporated.is_empty() || !args.deferred.is_empty() || !args.rejected.is_empty();

    if has_dispositions {
        return inboxes::drain_finalize(
            &repo_path,
            name,
            &args.incorporated,
            &args.deferred,
            &args.rejected,
        );
    }

    let paths = inboxes::drain_enumerate(&repo_path, name)?;
    for p in &paths {
        println!("{}", p.display());
    }
    if paths.is_empty() {
        eprintln!("inbox {}: no pending observations", name);
    } else {
        eprintln!(
            "inbox {}: {} pending observation{}",
            name,
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        );
    }
    Ok(())
}

fn cmd_pick() -> Result<()> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let worktree = repo::git_toplevel(&cwd)?;
    let grove_root = worktree.join(".grove");
    match pick::next_leaf(&grove_root)? {
        Some(p) => {
            println!("{}", p.display());
        }
        None => {
            let label = worktree
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| worktree.display().to_string());
            eprintln!("grove {}: no live leaves; this grove is done", label);
        }
    }
    Ok(())
}

fn cmd_brief_chain(leaf_path: Option<&std::path::Path>) -> Result<()> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let worktree = repo::git_toplevel(&cwd)?;
    let grove_root = worktree.join(".grove");
    let leaf: PathBuf = match leaf_path {
        Some(p) => p.to_path_buf(),
        None => match pick::next_leaf(&grove_root)? {
            Some(p) => p,
            None => {
                let label = worktree
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| worktree.display().to_string());
                eprintln!("grove {}: no live leaves; this grove is done", label);
                return Ok(());
            }
        },
    };
    let chain = brief_chain::chain_for(&grove_root, &leaf)?;
    for p in &chain {
        println!("{}", p.display());
    }
    Ok(())
}

fn cmd_leaf_add(args: &LeafAddArgs) -> Result<()> {
    let node = resolve_node(args.node.as_deref())?;
    let kind = leaf::Kind::parse(&args.kind)?;
    let explicit = args
        .prefix
        .as_deref()
        .map(parse_prefix_arg)
        .transpose()?;
    let path = leaf::add(&node, &args.slug, explicit, kind)?;
    println!("{}", path.display());
    Ok(())
}

fn cmd_leaf_insert(args: &LeafInsertArgs) -> Result<()> {
    let node = resolve_node(args.node.as_deref())?;
    let kind = leaf::Kind::parse(&args.kind)?;
    let (prefix, slug) = parse_prefix_slug(&args.prefix_slug)?;
    let (path, renumbers) = leaf::insert(&node, prefix, slug, kind)?;

    // The new leaf's path is the only stdout content; renumber summary and
    // cross-references go to stderr so the LLM can parse stdout cleanly.
    println!("{}", path.display());
    let mut stderr = std::io::stderr();
    if renumbers.is_empty() {
        eprintln!("leaf-insert {:03}-{}: no siblings to renumber", prefix, slug);
    } else {
        eprintln!(
            "leaf-insert {:03}-{}: renumbered {} sibling{}:",
            prefix,
            slug,
            renumbers.len(),
            if renumbers.len() == 1 { "" } else { "s" }
        );
        for r in &renumbers {
            eprintln!("  {:03} -> {:03}  ({})", r.old_prefix, r.new_prefix, r.new_name);
        }
        eprintln!("cross-references to review (verb does not auto-rewrite):");
        leaf::surface_cross_refs(&node, &renumbers, &mut stderr)?;
    }
    Ok(())
}

fn cmd_leaf_decompose(args: &LeafDecomposeArgs) -> Result<()> {
    let dst = leaf_ops::decompose(&args.leaf_path)?;
    println!("{}", dst.display());
    Ok(())
}

fn cmd_leaf_retire(args: &LeafRetireArgs) -> Result<()> {
    let dst = leaf_ops::retire(&args.leaf_path)?;
    println!("{}", dst.display());
    Ok(())
}

fn resolve_node(arg: Option<&std::path::Path>) -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let node = match arg {
        Some(p) if p.is_absolute() => p.to_path_buf(),
        Some(p) => cwd.join(p),
        None => cwd,
    };
    Ok(node)
}

fn parse_prefix_arg(s: &str) -> Result<u32> {
    let n: u32 = s
        .parse()
        .with_context(|| format!("--prefix must be a number: {:?}", s))?;
    Ok(n)
}

fn parse_prefix_slug(arg: &str) -> Result<(u32, &str)> {
    if arg.len() < 5 {
        anyhow::bail!(
            "argument must be `<prefix>-<slug>` (got {:?}) — e.g. `050-foo-bar`",
            arg
        );
    }
    let (head, tail) = arg.split_at(3);
    if !head.chars().all(|c| c.is_ascii_digit()) {
        anyhow::bail!(
            "prefix must be exactly three ASCII digits (got {:?}) — e.g. `050-foo-bar`",
            arg
        );
    }
    if !tail.starts_with('-') {
        anyhow::bail!(
            "prefix and slug must be separated by `-` (got {:?}) — e.g. `050-foo-bar`",
            arg
        );
    }
    let prefix: u32 = head.parse().unwrap();
    let slug = &tail[1..];
    Ok((prefix, slug))
}

fn read_body(args: &InboxAddArgs) -> Result<String> {
    if let Some(b) = &args.body {
        return Ok(b.clone());
    }
    if let Some(p) = &args.body_file {
        return std::fs::read_to_string(p)
            .with_context(|| format!("reading body file {}", p.display()));
    }
    if args.body_stdin {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).context("reading body from stdin")?;
        return Ok(s);
    }
    anyhow::bail!("provide observation via --body, --body-file, or --body-stdin");
}

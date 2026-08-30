// The LLM-driven CLI surface — `grove-llm`. See cli-binary-split
// (`docs/ARCHITECTURE.md#cli-binary-split`) for the audience-split
// rationale: every verb here exists for the LLM driving a grove session to
// invoke deterministically, not for a human at a terminal.
//
// Verbs are flat (hyphenated) so a single `grove-llm --help` enumerates every
// verb the LLM might call — important for bootstrap-recovery if a session
// drops context.
//
// The task-tree verbs (`pick` / `brief-chain` / `resolve` / `leaf-add` /
// `leaf-insert` / `leaf-decompose` / `leaf-retire` / `leaf-prune` / `root-init` /
// `finish-commit`)
// speak the **v2 directory scheme** (task-tree-scheme). Since `sweep-k37` every
// one of them dispatches to `task_tree` / `task_grow` / `tree_lifecycle`, all of
// which run through `ordinal-fs-tree`: the directory-walking modules the flip
// replaced group by group are deleted, so there is no second reader left to
// choose between. There is no transitional dual-format reader and no migration:
// a tree whose names this grammar does not spell is refused by `TaskNameError`,
// which already names what is on disk and what it should be
// (`delete-migration-k6`).

use crate::complete;
use crate::driver_lease;
use crate::leaf::Kind;
use crate::session_config::SessionConfig;
use crate::task_grow;
use crate::task_tree;
use crate::tree_lifecycle;
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use jj_workspace::Workspace;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "grove-llm",
    version,
    arg_required_else_help = true,
    about = "Grove: LLM-driven verbs for mid-session use",
    long_about = "Verbs the LLM driving a grove session invokes deterministically. \
They are separated from the human-facing `grove` binary; none of these verbs \
are meant for direct human use."
)]
pub struct Cli {
    /// The verb to run.
    ///
    /// Optional only because clap's derive requires it to be for
    /// `arg_required_else_help` to print help on a bare invocation; every
    /// invocation that parses at all names a verb. It stopped being genuinely
    /// optional at `delete-provisioning-k19`, which deleted `--content-hash` —
    /// the one metadata flag that stood alone — along with the embed it named
    /// and the build-pairing report that was its only caller.
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scaffold a brand-new grove's tree: create `.grove/`, write the root
    /// `BRIEF.md` charter, and lay down a first **requirements** leaf
    /// `01-requirements-<slug>-k1.md` (default slug `plan`) — the kind is fixed, since the
    /// bootstrap session's only input is the human's own words. After this,
    /// `grove-llm pick` returns the new
    /// leaf — a fresh grove is no longer indistinguishable from a finished
    /// one. Refuses if `.grove/` already exists. Working-tree change only —
    /// no commit. Prints the charter's path, then the leaf's.
    RootInit(RootInitArgs),
    /// Print the absolute path of the next live leaf in this grove's tree — a
    /// recursive depth-first **pre-order** walk over the directory tree (a node
    /// is a directory of numbered children, optionally headed by a `BRIEF.md`),
    /// returning the
    /// first live leaf and skipping briefs and terminal leaves — retired
    /// (`DONE`) and abandoned (`ABANDONED`) alike. Empty stdout
    /// (and a diagnostic on stderr) when the grove has no live leaves.
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
    /// Print a leaf's task **kind** — one of the closed nineteen — read from its
    /// current-format filename.
    /// With no argument the kind is read for `pick`'s next live leaf; on an
    /// empty grove it prints the standard "no live leaves" diagnostic on stderr
    /// (mirroring `brief-chain`) and exits 0. A task-shaped current filename
    /// with a missing or unknown kind is malformed and errors visibly; foreign
    /// files remain ignored. The output is a single lowercase token + newline.
    /// It is a diagnostic and tree-interface verb: the loop driver selects its
    /// own leaf in-process, so nothing here routes a launch.
    Kind {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
    },
    /// Resolve a reference to its current file path, searching live, retired
    /// (`DONE`), **and** abandoned (`ABANDONED`) entries alike
    /// across the whole directory tree. A permanent key (`[n]` or bare `n`,
    /// optionally `[n]-slug`) resolves the unique keyed entry; a bare slug
    /// resolves by slug (0 ⇒ not found, 1 ⇒ that entry, >1 ⇒ ambiguous, listing
    /// each match's key so you re-query by key); the full `<slug>-k<key>` handle
    /// resolves by its terminal key. A node resolves to
    /// its **directory** path (append `/BRIEF.md` to read its charter). Prints
    /// the path on stdout; a `DONE` or `ABANDONED` match also prints its own
    /// note on stderr (so the two are distinguishable — a resolved dead end
    /// never looks live); a not-found or ambiguous reference prints a
    /// diagnostic on stderr instead. Either way it still exits zero.
    Resolve {
        /// The reference: `[n]` / `n` / `[n]-slug` / `<slug>-k<key>` (by
        /// permanent key) or a bare slug.
        reference: String,
    },
    /// Append a child leaf under `<parent>` at the next gapless child position,
    /// with a fresh permanent key. `<parent>` is `.` for the grove root, or a
    /// node by its key (`[n]` / `n` / `<slug>-k<key>`) or its path. Prints the
    /// new leaf's absolute path on stdout. Working-tree change only — no commit.
    ///
    /// This is also how a **review chain** is built — one step at a time, each
    /// created only when it is required. A producer's last act is
    /// `leaf-add <parent> <stem> --kind review-<producer>` if review is
    /// warranted; the review's last act is
    /// `leaf-add <parent> <stem> --kind integrate-review-<producer>`
    /// if it found something worth acting on. Every step carries the **same
    /// bare stem**: the kind states the step's role, so the slug names only the
    /// artifact. The steps are ordinary **flat
    /// siblings** — there is no chain node — and the creating session writes the
    /// new leaf's body, which is the point of creating it late: it can put the
    /// exact uncovered case, finding, or datum into it, which is strictly more
    /// than a constructor writing up front could know. A review that finds
    /// nothing creates nothing and simply retires.
    ///
    /// **The integrate step is the one that cares where it lands.** This verb
    /// appends at the parent's *end*, which is right for it only when nothing
    /// later in that directory would run first. A review's findings are anchored
    /// to file-and-line positions an intervening leaf can move silently, so scan
    /// the review's own parent for the first sibling **entry** after it whose
    /// subtree still holds live work — a live leaf, or a node directory with one
    /// anywhere beneath it, since `pick` descends a node in place. If there is
    /// one, cut the integration with `leaf-insert <that entry> …` instead,
    /// targeting the entry itself and never a leaf inside it. Later terminal
    /// leaves, nodes whose subtree is wholly terminal, and the `finish` sentinel
    /// do not count; nor does anything outside the directory, because the walk
    /// finishes it — including this appended leaf — before any later sibling of
    /// an ancestor. A `review-*` step re-derives its citations from the
    /// producer's commit and needs none of this care.
    LeafAdd(LeafAddArgs),
    /// Append a whole **research vendor pair** under `<parent>` in one call —
    /// three **flat siblings** at consecutive positions, all slugged `<stem>`,
    /// with fixed filename kinds and no routing metadata in task bodies. The kind
    /// is what tells the three apart, and the slug does not restate it.
    ///
    /// Three siblings, with three consecutive fresh keys:
    ///
    ///   NN    `research-a-<stem>-k…`
    ///   NN+1  `research-b-<stem>-k…`
    ///   NN+2  `combine-research-<stem>-k…`
    ///
    /// **A pair is created eagerly where a review chain is created lazily, and
    /// the asymmetry is deliberate.** If `research-a` cut `research-b` at the
    /// end of its own session, `b` would inherit `a`'s framing and corpus,
    /// destroying the independence the pair is run for. Research also has no
    /// `review-` sibling, so its shape is a pair rather than a chain and gets
    /// its own verb. The two producer kinds are distinct routing targets by
    /// construction. Cut one when a question is load-bearing enough to pay for
    /// two independent corpora and blind spots.
    ///
    /// The whole shape is created or none of it is. Prints **three** absolute
    /// paths on stdout in position order — and prints nothing at all if the
    /// shape could not be created. Working-tree change only — no commit.
    LeafAddPair(LeafAddPairArgs),
    /// Insert a new leaf at the slot held by `<target>`, shifting `<target>` and
    /// every later sibling up one position. `<target>` is an existing leaf or
    /// node by its key or path. Because the hierarchy lives in directories, each
    /// shift is a single plain rename of one sibling directory, and the whole
    /// subtree — child names *and* keys — rides along untouched; in-file `# …`
    /// headers are position-free, so **zero file contents** are rewritten. The new leaf
    /// gets a fresh key. Prints the new leaf's absolute path on stdout; the
    /// renumber summary and stray position-prefixed cross-references go to
    /// stderr. Working-tree change only — no commit.
    ///
    /// Use it for new work that must sequence ahead of live leaves — and, as the
    /// **default**, for a **review chain's integrate step**. A review's findings
    /// are anchored to a commit and to file-and-line positions; an intervening
    /// leaf that edits a cited file moves them without erroring, and the
    /// integrating session then re-derives the reviewer's intent from a tree the
    /// reviewer never saw.
    ///
    /// The target there is **the first sibling entry after the review whose
    /// subtree still holds live work**, read in the review's own parent
    /// directory. An *entry*, not a leaf: `pick` descends a node directory in
    /// place, so a later sibling node with a live leaf anywhere beneath it
    /// blocks too — and that node is the target, never the live leaf inside it,
    /// which would insert one level down. Terminal leaves, nodes whose subtree
    /// is wholly terminal, and the driver's `finish` sentinel never block. Plain
    /// `leaf-add` is correct there only when no sibling entry blocks: the walk
    /// finishes the review's own directory, appended leaf included, before any
    /// later sibling of an ancestor.
    LeafInsert(LeafInsertArgs),
    /// Convert a live leaf file `NN-<kind>--<slug>-k<key>.md` into a node **directory**
    /// `NN-<slug>-k<key>/` (**key preserved** — the leaf that was `k<key>`
    /// becomes the node `k<key>`), moving the leaf body in as the node's
    /// `BRIEF.md` (a plain rename on every lane, staging nothing; its
    /// `# <slug>-k<key>` header retitled with ` — brief`) and
    /// atomically growing a first child
    /// `01-<kind>-<first-child-slug>-k<new>.md` so the
    /// node is never childless. The first child **inherits the decomposed
    /// leaf's own kind** unless `--kind` overrides it.
    /// Prints the brief's absolute path then the first child's, one per line.
    /// Working-tree change only — no commit.
    LeafDecompose(LeafDecomposeArgs),
    /// Mark a live leaf retired in place by adding a `DONE` infix
    /// (`NN-<kind>--<slug>-k<key>.md` →
    /// `NN-DONE-<kind>--<slug>-k<key>.md`) — no `done/`
    /// directory; the leaf keeps its position and key in its directory, and the
    /// file's contents (its `# <slug>-k<key>` header) are untouched. Refuses a
    /// brief, an already-retired (`DONE`) leaf, and an already-abandoned
    /// (`ABANDONED`) leaf. Prints the retired file's absolute
    /// path on stdout. Task bodies are byte-identical; retirement writes no
    /// launch-routing metadata. Working-tree change only — no commit.
    LeafRetire(LeafRetireArgs),
    /// Mark abandoned work `ABANDONED` in place. **HITL: only
    /// call this after explicit human confirmation** — grove never abandons
    /// planned work on its own (constraint 5: grove guides, it does not gate —
    /// the gate is yours, not the CLI's). `<path>` is a live leaf file **or** a
    /// node directory (absolute, or relative to the grove root):
    ///
    ///   * given a **leaf**, marks it directly — refuses a brief, an
    ///     already-`DONE` leaf, and an already-`ABANDONED` leaf;
    ///   * given a **node**, marks every *live* leaf in its subtree
    ///     (recursively) — leaving `DONE` leaves untouched, since that work
    ///     really was done — and refuses the grove root itself (abandoning a
    ///     whole workstream is a branch-delete, not a tree mark).
    ///
    /// The `ABANDONED` infix is filename-only — every marked leaf's
    /// `# <slug>-k<key>` header stays byte-identical. Prints each newly-marked
    /// leaf's absolute path on stdout, one per line; any already-`DONE` leaves
    /// found and left alone are reported on stderr. Pruning writes no producer
    /// receipt: pruning only a producer leaves its sibling review live and
    /// uncheckable; prune the enclosing chain to close the whole reviewed path.
    /// Working-tree change only — no commit.
    LeafPrune(LeafPruneArgs),
    /// Revalidate the live driver-owned finish leaf and the absence of ordinary
    /// work under the exclusive tree lock, then delete and commit only
    /// `.grove/`. This helper enforces tree and VCS facts; it does not infer or
    /// automate the finish session's required human confirmation.
    ///
    /// Teardown is a plain deletion followed by a path-scoped `jj commit`, and
    /// grove implements no transaction around it: no witness, no manifest, no
    /// rollback proof, no quarantine, no recovery path. Jujutsu snapshots the
    /// working copy before every command and its operation log is the
    /// transaction record, so the version control system already owns every
    /// guarantee grove used to hand-build.
    ///
    /// A failure therefore stops with a message rather than repairing itself,
    /// and the message names the command that puts the tree back — `jj restore
    /// .grove` if the deletion is what failed, `jj undo` if the commit is. Once
    /// the tree is back, rerun this same command with the same handle. Grove
    /// never resets, rebases, or rewrites history on your behalf.
    ///
    /// Only `.grove/` is committed: unrelated working-copy changes stay in the
    /// working copy, because the commit is scoped to that fileset.
    FinishCommit {
        /// Stable handle of the launched finish leaf, for example `finish-k42`.
        finish_handle: String,
    },
    /// Signal task completion to the self-driving loop. Run this as
    /// the **last step** of a task, after commit + retire — it is how the loop
    /// ends this harness session and starts the next task with fresh context.
    ///
    /// Writes the disposition flag (relaunch by default, or finish with
    /// `--done`) to the signal file and returns immediately. Ending the
    /// session is the loop driver's job, not this verb's: the driver launched
    /// this session and is watching for the signal file while it runs, so it
    /// applies grace → SIGTERM → kill-grace → SIGKILL to its own child once
    /// the file appears (driver-side watcher) — the driver can always signal
    /// its child, unlike an in-agent self-kill, which some harness sandboxes
    /// (e.g. codex's Seatbelt) silently deny. Do nothing else after running
    /// it. The default relaunches the loop for the next task; `--done` — the
    /// Finish cycle's last action — stops it cleanly. The signal-file default
    /// comes from the loop driver's environment (`GROVE_SIGNAL_FILE`); when
    /// that is absent (a session not under `grove do`) it is a safe
    /// near-no-op that just tells you to exit manually.
    Complete(CompleteArgs),
}

impl Command {
    fn operation_label(&self) -> &'static str {
        match self {
            Self::RootInit(_) => "grove-llm root-init",
            Self::Pick => "grove-llm pick",
            Self::BriefChain { .. } => "grove-llm brief-chain",
            Self::Kind { .. } => "grove-llm kind",
            Self::Resolve { .. } => "grove-llm resolve",
            Self::LeafAdd(_) => "grove-llm leaf-add",
            Self::LeafAddPair(_) => "grove-llm leaf-add-pair",
            Self::LeafInsert(_) => "grove-llm leaf-insert",
            Self::LeafDecompose(_) => "grove-llm leaf-decompose",
            Self::LeafRetire(_) => "grove-llm leaf-retire",
            Self::LeafPrune(_) => "grove-llm leaf-prune",
            Self::FinishCommit { .. } => "grove-llm finish-commit",
            Self::Complete(_) => "grove-llm complete",
        }
    }
}

#[derive(Parser)]
pub struct CompleteArgs {
    /// Finish the whole grove instead of relaunching: signal the loop to stop
    /// cleanly. Use as the **last** action of the Finish cycle. Without it
    /// (the per-task default) the loop relaunches with fresh context.
    #[arg(long)]
    pub done: bool,
    /// Relaunch-signal file the loop driver watches for. Default: `$GROVE_SIGNAL_FILE`.
    #[arg(long = "signal-file")]
    pub signal_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct RootInitArgs {
    /// Slug for the first (requirements) leaf (lowercase ASCII letters, digits,
    /// dashes). Default: `plan`. Mirrors `leaf-add . <slug>`.
    #[arg(default_value = "plan")]
    pub slug: String,
}

/// `--kind` help for the two verbs that *create* a leaf. One const rather than
/// two hand-copied copies, and written **generatively** — the producers, then
/// "each with its own `review-` and `integrate-review-` step" — because the
/// point of a help string is to teach the shape of the set; the full nineteen
/// are one `--kind reserch` away, from `Kind::parse`'s error.
const KIND_HELP: &str = "Leaf kind, written into the filename. Producers: requirements, \
design, planning, prototype, impl — each with its own review-<producer> and \
integrate-review-<producer> step; plus research-a, research-b, and combine-research. \
`finish` is driver-reserved and refused by this verb. An unrecognised value errors, \
listing all nineteen";

/// [`KIND_HELP`] for `leaf-decompose`, whose `--kind` overrides an inherited
/// kind rather than supplying a default.
const KIND_OVERRIDE_HELP: &str = "Override the first child's filename kind — one of the \
nineteen session kinds except driver-reserved finish. Default: inherit the kind of the \
leaf being decomposed";

#[derive(Parser)]
pub struct LeafAddArgs {
    /// Parent node — `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path.
    pub parent: String,
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    #[arg(long = "kind", default_value = "impl", help = KIND_HELP)]
    pub kind: String,
}

#[derive(Parser)]
pub struct LeafAddPairArgs {
    /// Parent node — `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path.
    pub parent: String,
    /// Shared stem for all three leaves (lowercase ASCII letters, digits,
    /// dashes). All three carry it verbatim as their slug — the `research-a`,
    /// `research-b` and `combine-research` kinds already say which step is which.
    pub stem: String,
}

#[derive(Parser)]
pub struct LeafInsertArgs {
    /// Target entry — the existing leaf or node (by key or path) whose slot the
    /// new leaf should occupy; the target and later siblings shift up by one.
    pub target: String,
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    #[arg(long = "kind", default_value = "impl", help = KIND_HELP)]
    pub kind: String,
}

#[derive(Parser)]
pub struct LeafDecomposeArgs {
    /// Leaf path. Absolute, or relative to the grove root (`.grove/`).
    pub leaf_path: PathBuf,
    /// Slug for the node's first child (lowercase ASCII letters, digits,
    /// dashes).
    pub first_child_slug: String,
    #[arg(long = "kind", help = KIND_OVERRIDE_HELP)]
    pub kind: Option<String>,
}

#[derive(Parser)]
pub struct LeafRetireArgs {
    /// Leaf path. Absolute, or relative to the grove root (`.grove/`).
    pub leaf_path: PathBuf,
}

#[derive(Parser)]
pub struct LeafPruneArgs {
    /// Leaf or node path. Absolute, or relative to the grove root (`.grove/`).
    pub path: PathBuf,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let Some(command) = cli.command else {
        // Unreachable through clap: a bare invocation prints help
        // (`arg_required_else_help`), and any other argument is either a verb or
        // a parse error.
        bail!("no verb given; run `grove-llm --help` for the verb set");
    };
    let cwd = std::env::current_dir().context("getting cwd for session epoch admission")?;
    let session_epoch = driver_lease::admit_ambient_session(&cwd, command.operation_label())?;
    match command {
        Command::RootInit(args) => cmd_root_init(&args),
        Command::Pick => cmd_pick(),
        Command::BriefChain { leaf_path } => cmd_brief_chain(leaf_path.as_deref()),
        Command::Kind { leaf_path } => cmd_kind(leaf_path.as_deref()),
        Command::Resolve { reference } => cmd_resolve(&reference),
        Command::LeafAdd(args) => cmd_leaf_add(&args),
        Command::LeafAddPair(args) => cmd_leaf_add_pair(&args),
        Command::LeafInsert(args) => cmd_leaf_insert(&args),
        Command::LeafDecompose(args) => cmd_leaf_decompose(&args),
        Command::LeafRetire(args) => cmd_leaf_retire(&args),
        Command::LeafPrune(args) => cmd_leaf_prune(&args),
        Command::FinishCommit { finish_handle } => cmd_finish_commit(&finish_handle),
        Command::Complete(args) => cmd_complete(&args, session_epoch.as_ref()),
    }
}

fn cmd_finish_commit(finish_handle: &str) -> Result<()> {
    let (worktree, _) = grove_paths()?;
    tree_lifecycle::finish_commit(&worktree, finish_handle)
}

fn cmd_complete(
    args: &CompleteArgs,
    session_epoch: Option<&driver_lease::SessionEpochGuard>,
) -> Result<()> {
    let disposition = if args.done {
        complete::Disposition::Done
    } else {
        complete::Disposition::Relaunch
    };
    let opts = complete::resolve_opts(args.signal_file.clone(), disposition);
    if let Some(session_epoch) = session_epoch {
        session_epoch.require_signal_path(opts.signal_file.as_deref())?;
    }
    complete::signal_complete(&opts)
}

fn cmd_root_init(args: &RootInitArgs) -> Result<()> {
    let (worktree, _) = grove_paths()?;
    require_declared(&worktree, &[Kind::Requirements])?;
    let paths = tree_lifecycle::root_init(&worktree, &args.slug)?;
    for p in &paths {
        println!("{}", p.display());
    }
    Ok(())
}

fn cmd_pick() -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    match task_tree::pick(&grove_root)? {
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
    let Some(chain) = brief_chain_for(&grove_root, leaf_path)? else {
        eprintln!(
            "grove {}: no live leaves; this grove is done",
            label(&worktree)
        );
        return Ok(());
    };
    for p in &chain {
        println!("{}", p.display());
    }
    Ok(())
}

fn brief_chain_for(grove_root: &Path, leaf_path: Option<&Path>) -> Result<Option<Vec<PathBuf>>> {
    let tree = task_tree::read(grove_root)?;
    let leaf = match leaf_path {
        Some(path) => normalize_leaf_path(path),
        None => match task_tree::pick_in(&tree)? {
            Some(path) => path,
            None => return Ok(None),
        },
    };
    task_tree::brief_chain(&tree, &leaf).map(Some)
}

fn cmd_kind(leaf_path: Option<&Path>) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    // Normalize a cwd-relative path to what `task_tree::kind` accepts (absolute
    // or grove-root-relative), matching `cmd_brief_chain`'s handling; a `None`
    // stays `None` so the verb defaults to `pick`'s next live leaf.
    let leaf = leaf_path.map(normalize_leaf_path);
    match task_tree::kind(&grove_root, leaf.as_deref())? {
        Some(kind) => println!("{}", kind.label()),
        None => eprintln!(
            "grove {}: no live leaves; this grove is done",
            label(&worktree)
        ),
    }
    Ok(())
}

fn cmd_resolve(reference: &str) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let resolution = task_tree::resolve(&grove_root, reference)?;
    let (stdout, stderr) = task_tree::render_resolution(reference, &resolution);
    // `resolve` is pick-style: a not-found / ambiguous reference is reported on
    // stderr and still exits zero (it is information, not an error).
    print!("{stdout}");
    eprint!("{stderr}");
    Ok(())
}

fn cmd_leaf_add(args: &LeafAddArgs) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    require_declared(&worktree, &[kind])?;
    let path = task_grow::leaf_add(&grove_root, &args.parent, &args.slug, kind)?;
    println!("{}", path.display());
    Ok(())
}

/// Print `leaf-add-pair`'s paths — **after** the mutation succeeded, never as
/// each leaf lands. A run that fails is rolled back, so stdout describing a
/// shape the command reported as failed would be describing files that are no
/// longer there.
fn print_paths(paths: &[PathBuf]) {
    for p in paths {
        println!("{}", p.display());
    }
}

fn cmd_leaf_add_pair(args: &LeafAddPairArgs) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    require_declared(&worktree, &task_grow::PAIR)?;
    let paths = task_grow::leaf_add_pair(&grove_root, &args.parent, &args.stem)?;
    print_paths(&paths);
    Ok(())
}

fn cmd_leaf_insert(args: &LeafInsertArgs) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    require_declared(&worktree, &[kind])?;
    let inserted = task_grow::leaf_insert(&grove_root, &args.target, &args.slug, kind)?;
    report_insert(&grove_root, &args.slug, &inserted)
}

/// `leaf-insert`'s output: the new leaf's path on stdout, the renumber summary
/// and the cross-reference lint on stderr.
///
/// Keep the established CLI stream semantics: the standard print macros panic on
/// a broken stdout/stderr, while cross-reference scanning remains the only
/// fallible output operation returned as an error.
fn report_insert(grove_root: &Path, slug: &str, inserted: &task_grow::Inserted) -> Result<()> {
    println!("{}", inserted.path.display());
    let renumbers = &inserted.renumbers;
    if renumbers.is_empty() {
        eprintln!("leaf-insert {slug}: no siblings to renumber");
        return Ok(());
    }
    eprintln!(
        "leaf-insert {}: renumbered {} sibling{}:",
        slug,
        renumbers.len(),
        if renumbers.len() == 1 { "" } else { "s" }
    );
    for renumber in renumbers {
        eprintln!(
            "  {:02} -> {:02}  ({})",
            renumber.old_position, renumber.new_position, renumber.new_name
        );
    }
    eprintln!("cross-references to review (verb does not auto-rewrite):");
    task_grow::surface_cross_refs(grove_root, renumbers, &mut std::io::stderr())
}

fn cmd_leaf_decompose(args: &LeafDecomposeArgs) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    // `None` (the default) inherits the decomposed leaf's own kind; `--kind`
    // overrides it (task-kind-taxonomy).
    let kind_override = args.kind.as_deref().map(Kind::parse).transpose()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    // The first child's kind, resolved *before* the mutation so the presence
    // rule can be asked about the kind this call will actually write. With no
    // `--kind` that is the decomposed leaf's own, read off its filename — the
    // same answer `leaf_decompose` will reach for itself, and the only one it
    // ever reaches.
    //
    // A kind that cannot be read is left to the verb: `leaf-decompose` refuses a
    // brief, a retired leaf and a malformed name with its own message, and a
    // presence check that errored first would replace those refusals with a
    // complaint about configuration.
    let child_kind = match kind_override {
        Some(kind) => Some(kind),
        None => task_tree::kind(&grove_root, Some(&leaf_path))
            .ok()
            .flatten(),
    };
    if let Some(kind) = child_kind {
        require_declared(&worktree, &[kind])?;
    }
    let (brief_path, child_path) = tree_lifecycle::leaf_decompose(
        &grove_root,
        &leaf_path,
        &args.first_child_slug,
        kind_override,
    )?;
    println!("{}", brief_path.display());
    println!("{}", child_path.display());
    Ok(())
}

// The two steps that always follow a terminal mark: the commit that carries it,
// then the completion signal. `leaf-retire` and `leaf-prune` are the
// terminal-marking pair and the **last grove verbs a session runs** — Retire
// precedes Commit, and the commit itself is jj/git — so their output lands in
// the agent's context at the moment of decision, rather than only in the mandate
// a whole session earlier. **stderr**: stdout is data (callers parse the printed
// paths), and `leaf-prune`'s existing advisories already set that precedent.
fn eprint_next_steps(verb: &str, marked: usize) {
    let renames = if marked == 1 {
        "this rename"
    } else {
        "these renames"
    };
    eprintln!("{verb}: two steps remain:");
    eprintln!("  1. commit this session's work, including {renames}");
    eprintln!("  2. run `grove-llm complete` as your last action");
}

fn cmd_leaf_retire(args: &LeafRetireArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let dst = tree_lifecycle::leaf_retire(&grove_root, &leaf_path)?;
    println!("{}", dst.display());
    eprint_next_steps("leaf-retire", 1);
    Ok(())
}

fn cmd_leaf_prune(args: &LeafPruneArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let path = normalize_leaf_path(&args.path);
    let result = tree_lifecycle::leaf_prune(&grove_root, &path)?;
    for p in &result.marked {
        println!("{}", p.display());
    }
    if result.marked.is_empty() {
        eprintln!("leaf-prune: nothing live to mark");
    }
    if !result.left_done.is_empty() {
        eprintln!(
            "leaf-prune: left {} already-DONE leaf{} untouched:",
            result.left_done.len(),
            if result.left_done.len() == 1 { "" } else { "s" }
        );
        for p in &result.left_done {
            eprintln!("  {}", p.display());
        }
    }
    // Last, so the reminder is the final thing in the agent's context — and only
    // when this call actually ended some work; a no-op prune leaves no session
    // to close.
    if !result.marked.is_empty() {
        eprint_next_steps("leaf-prune", result.marked.len());
    }
    Ok(())
}

/// The **just-in-time presence rule**, asked at the moment grove writes a leaf.
///
/// Before writing a leaf of kind K, K must resolve to exactly one complete
/// template read whole out of one file
/// (`docs/adr/complete-session-configuration.md`). This replaces the
/// all-nineteen completeness check, which grove can no longer make: nothing here
/// enumerates the kinds a methodology declares, so the only honest question is
/// about the kind in hand.
///
/// It runs **before** the mutation, so a refusal leaves the tree byte-identical
/// — and it loads the whole configuration to ask, which is what keeps the other
/// half of the amendment true: every template rule in both documents is still
/// checked eagerly, and a malformed entry for a kind this call will never touch
/// still fails here.
fn require_declared(worktree: &Path, kinds: &[Kind]) -> Result<()> {
    let config = SessionConfig::load_for_worktree(worktree)?;
    for kind in kinds {
        config.require(kind.label()).with_context(|| {
            format!(
                "refusing to write a leaf of kind `{}`: no launch template resolves for it",
                kind.label()
            )
        })?;
    }
    Ok(())
}

// Resolve `(worktree, grove_root)` from the cwd. The task-tree verbs run from
// the worktree root (not from inside `.grove/`), so the grove root is always
// `<worktree>/.grove`, matched across every verb so a leaf is never created or
// read one level off.
fn grove_paths() -> Result<(PathBuf, PathBuf)> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let worktree = Workspace::resolve(&cwd)?.root().to_path_buf();
    let grove_root = worktree.join(".grove");
    Ok((worktree, grove_root))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::os::fd::AsRawFd;
    use tempfile::TempDir;

    fn grove_with_node() -> (TempDir, PathBuf) {
        let worktree = TempDir::new().unwrap();
        let paths = tree_lifecycle::root_init(worktree.path(), "plan").unwrap();
        let grove_root = worktree.path().join(".grove");
        tree_lifecycle::leaf_decompose(&grove_root, &paths[1], "first", Some(Kind::Impl)).unwrap();
        (worktree, grove_root)
    }

    fn assert_one_acquisition(operation: impl FnOnce(&Path)) {
        let (_worktree, grove_root) = grove_with_node();
        crate::task_tree::reset_read_count();

        operation(&grove_root);

        // **One counter, where there used to be two summed.** Grove holds no
        // guard of its own since `collapse-tree-access-k13`, so every
        // observation of the tree is the store's and `task_tree` counts all of
        // them. The property is unchanged: one CLI command reads the tree once,
        // so a verb that selected a leaf and then re-read the tree to act on it
        // could not slip through.
        assert_eq!(
            crate::task_tree::read_count(),
            1,
            "one CLI command must observe the tree exactly once"
        );
    }

    #[test]
    fn reference_taking_commands_acquire_the_tree_lock_once() {
        assert_one_acquisition(|grove_root| {
            task_grow::leaf_add(grove_root, "1", "later", Kind::Impl).unwrap();
        });
        assert_one_acquisition(|grove_root| {
            task_grow::leaf_add_pair(grove_root, "1", "survey").unwrap();
        });
        assert_one_acquisition(|grove_root| {
            brief_chain_for(grove_root, None).unwrap().unwrap();
        });
    }

    struct ExclusiveLockAssertingWriter {
        worktree: PathBuf,
        bytes: Vec<u8>,
    }

    impl Write for ExclusiveLockAssertingWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            let directory = File::open(&self.worktree)?;
            let result =
                unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) };
            assert_ne!(
                result, 0,
                "cross-reference output was written after leaf-insert released its exclusive lock"
            );
            self.bytes.extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn leaf_insert_lints_cross_references_under_an_exclusive_lock_of_its_own() {
        // **Two observations, and the second is the lint's.** A mutation consumes
        // its guard, so the tree the shift *left* — which is the tree a stale
        // reference has to be read out of, since a shifted node took its whole
        // subtree's paths with it — can only be seen through a second guard. What
        // the property was ever about is that the output is written while the tree
        // is **held**: a hit printed after the lock went would name a path anything
        // else could already have renamed.
        let (worktree, grove_root) = grove_with_node();
        let brief = grove_root.join("01-plan-k1").join("BRIEF.md");
        let body = fs::read_to_string(&brief).unwrap() + "stale path: 01-impl--first-k2\n";
        fs::write(&brief, body).unwrap();
        let mut output = ExclusiveLockAssertingWriter {
            worktree: worktree.path().to_path_buf(),
            bytes: Vec::new(),
        };
        crate::task_tree::reset_read_count();

        let inserted = task_grow::leaf_insert(&grove_root, "2", "earlier", Kind::Impl).unwrap();
        task_grow::surface_cross_refs(&grove_root, &inserted.renumbers, &mut output).unwrap();

        assert!(
            String::from_utf8_lossy(&output.bytes).contains("BRIEF.md:"),
            "fixture must exercise a cross-reference hit: {}",
            String::from_utf8_lossy(&output.bytes)
        );
        assert_eq!(
            crate::task_tree::read_count(),
            2,
            "the insert, then the lint's own guard over the tree it left"
        );
    }
}

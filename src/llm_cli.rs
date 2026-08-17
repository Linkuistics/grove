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
// speak the **v2 directory scheme** (task-tree-scheme): they dispatch to the
// directory-based modules `tree_read` / `tree_grow` / `tree_lifecycle`. There is
// no transitional dual-format reader — `tree_migrate` is the only thing that
// reads a legacy tree, once, on adoption, and the only legacy shape it still
// converts is a v2 tree whose leaves predate filename kinds.

use crate::complete;
use crate::driver_lease;
use crate::leaf::Kind;
use crate::repo;
use crate::tree_access;
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
    arg_required_else_help = true,
    about = "Grove: LLM-driven verbs for mid-session use",
    long_about = "Verbs the LLM driving a grove session invokes deterministically. \
They are separated from the human-facing `grove` binary; none of these verbs \
are meant for direct human use."
)]
pub struct Cli {
    /// Print this build's methodology identity — the content hash of the
    /// `content/` payload this binary embeds — and exit.
    ///
    /// It is a claim about the content this binary actually **carries**, read
    /// from the linked embed rather than from a constant recorded beside it,
    /// which is what makes the driver's comparison worth making.
    ///
    /// **A flag rather than a verb, deliberately.** The loop driver asks it of
    /// the `grove-llm` a session's `PATH` resolves, so it can report a
    /// build-pairing mismatch (`docs/adr/one-build-owns-a-session.md`); no
    /// session ever calls it and the embedded methodology instructs nothing
    /// about it, so it stays out of the agent grammar `tests/methodology.rs`
    /// scans. Like `--version` it is metadata the driver may ask from outside
    /// any session, and it touches no task tree, so it is exempt from the
    /// session-epoch guard.
    #[arg(long = "content-hash", exclusive = true)]
    pub content_hash: bool,
    /// The verb to run. Optional only so `--content-hash` can stand alone; a
    /// bare `grove-llm` prints help.
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
    /// no commit. Writes `.grove/FORMAT` last and prints it as the third path.
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
    /// shift is a single move of one sibling directory (`git mv`, or a plain
    /// rename in a jj-enabled tree) and the whole subtree
    /// — child names *and* keys — rides along untouched; in-file `# …` headers
    /// are position-free, so **zero file contents** are rewritten. The new leaf
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
    /// Convert a live leaf file `NN-<kind>-<slug>-k<key>.md` into a node **directory**
    /// `NN-<slug>-k<key>/` (**key preserved** — the leaf that was `k<key>`
    /// becomes the node `k<key>`), moving the leaf body in as the node's
    /// `BRIEF.md` (`git mv`, or a plain rename in a jj-enabled tree; its
    /// `# <slug>-k<key>` header retitled with ` — brief`) and
    /// atomically growing a first child
    /// `01-<kind>-<first-child-slug>-k<new>.md` so the
    /// node is never childless. The first child **inherits the decomposed
    /// leaf's own kind** unless `--kind` overrides it.
    /// Prints the brief's absolute path then the first child's, one per line.
    /// Working-tree change only — no commit.
    LeafDecompose(LeafDecomposeArgs),
    /// Mark a live leaf retired in place by adding a `DONE` infix
    /// (`NN-<kind>-<slug>-k<key>.md` →
    /// `NN-DONE-<kind>-<slug>-k<key>.md`) — no `done/`
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
    /// Teardown is one fail-closed transaction, not a delete followed by a
    /// commit. `.grove/` stays present — and unwalkable — until the repository
    /// proves the exact `.grove/`-scoped commit named by this handle and this
    /// launch's finish-attempt identity. Every ordinary root entry is first
    /// evacuated beneath a manifest-backed `.grove/FINISHING-<handle>/`
    /// witness; only after commit proof does the whole root move, in one atomic
    /// rename, to a workspace-control quarantine for cleanup-only disposal.
    /// **Task-root absence therefore never proves teardown succeeded** — a
    /// death before the commit would expose exactly that shape.
    ///
    /// Every reported failure is retryable. An uncommitted failure restores the
    /// live finish tree and leaves this same leaf selectable. If the repository
    /// can prove neither the exact commit nor its exact recorded start, the
    /// transaction stops as `Recovery pending`: the diagnostic names the
    /// artifact holding it (normally the witness; the quarantine once a failed
    /// restoration left the tree there), the recorded and observed topology, and
    /// the two operator exits — preserve any divergent work, then either restore
    /// the recorded start so recovery can roll back, or make the exact teardown
    /// result immediate so recovery can finish forward — and then rerun this
    /// same command (or bare `grove`). Grove never resets, rebases, or rewrites
    /// history on your behalf.
    ///
    /// If you lose this command's result, just run it again with the same
    /// handle. With `.grove/` already gone it verifies the immediate VCS result
    /// instead of trusting that absence: an exact handle-and-attempt-named
    /// commit whose only change is deleting `.grove/` is idempotent success, and
    /// anything else is a refusal. That proof is bound to the still-active
    /// session epoch, so a *later* invocation — a new grove reusing the same
    /// handle included — can never satisfy it.
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
    // Metadata first, and before anything that resolves a working tree: the
    // driver asks this of a binary from outside any session, so it is exempt
    // from the session-epoch guard exactly as clap's own `--version` is.
    if cli.content_hash {
        // clap's `exclusive` governs arguments and does not reach a subcommand,
        // so the combination is rejected here rather than silently answering the
        // flag and dropping the verb.
        if let Some(command) = cli.command {
            bail!(
                "`--content-hash` is metadata and runs nothing; drop `{}` or drop the flag",
                command.operation_label().trim_start_matches("grove-llm ")
            );
        }
        println!("{}", crate::methodology::identity());
        return Ok(());
    }
    let Some(command) = cli.command else {
        // Unreachable through clap: a bare invocation prints help
        // (`arg_required_else_help`), `--content-hash` returned above, and any
        // other argument is either a verb or a parse error.
        bail!("no verb given; run `grove-llm --help` for the verb set");
    };
    // On any verb, and before the guard that may refuse it: a clobbered skill
    // directory is worth saying even when the verb itself cannot run.
    crate::provision::warn_on_foreign_skill_dirs();
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
    let guard = tree_access::read(grove_root)?;
    let leaf = match leaf_path {
        Some(path) => normalize_leaf_path(path),
        None => match tree_read::pick_unlocked(guard.root())? {
            Some(path) => path,
            None => return Ok(None),
        },
    };
    tree_read::brief_chain_unlocked(guard.root(), &leaf).map(Some)
}

fn cmd_kind(leaf_path: Option<&Path>) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    // Normalize a cwd-relative path to what `tree_read::kind` accepts (absolute
    // or grove-root-relative), matching `cmd_brief_chain`'s handling; a `None`
    // stays `None` so the verb defaults to `pick`'s next live leaf.
    let leaf = leaf_path.map(normalize_leaf_path);
    match tree_read::kind(&grove_root, leaf.as_deref())? {
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
    let path = leaf_add_for(&grove_root, &args.parent, &args.slug, kind)?;
    println!("{}", path.display());
    Ok(())
}

fn leaf_add_for(grove_root: &Path, parent: &str, slug: &str, kind: Kind) -> Result<PathBuf> {
    let guard = tree_access::write(grove_root)?;
    let parent_dir = resolve_parent_unlocked(guard.root(), parent)?;
    tree_grow::leaf_add_unlocked(guard.root(), &parent_dir, slug, kind)
}

/// Resolve the `<parent>` argument shared by the two appending verbs: `.` is
/// the grove root (the root node); anything else is a node by key or path.
/// `tree_grow` validates that the result is a real node directory.
fn resolve_parent_unlocked(grove_root: &Path, parent: &str) -> Result<PathBuf> {
    if parent == "." {
        Ok(grove_root.to_path_buf())
    } else {
        resolve_ref_or_path_unlocked(grove_root, parent)
    }
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
    let (_, grove_root) = grove_paths()?;
    let paths = leaf_add_pair_for(&grove_root, &args.parent, &args.stem)?;
    print_paths(&paths);
    Ok(())
}

fn leaf_add_pair_for(grove_root: &Path, parent: &str, stem: &str) -> Result<Vec<PathBuf>> {
    let guard = tree_access::write(grove_root)?;
    let parent_dir = resolve_parent_unlocked(guard.root(), parent)?;
    tree_grow::leaf_add_pair_unlocked(guard.root(), &parent_dir, stem)
}

fn cmd_leaf_insert(args: &LeafInsertArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    leaf_insert_for(
        &grove_root,
        &args.target,
        &args.slug,
        kind,
        |locked_root, path, renumbers| {
            // Keep the established CLI stream semantics: the standard print macros
            // panic on a broken stdout/stderr, while cross-reference scanning
            // remains the only fallible output operation returned as an error.
            println!("{}", path.display());
            if renumbers.is_empty() {
                eprintln!("leaf-insert {}: no siblings to renumber", args.slug);
            } else {
                eprintln!(
                    "leaf-insert {}: renumbered {} sibling{}:",
                    args.slug,
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
                tree_grow::surface_cross_refs_unlocked(
                    locked_root,
                    renumbers,
                    &mut std::io::stderr(),
                )?;
            }
            Ok(())
        },
    )
}

fn leaf_insert_for(
    grove_root: &Path,
    target: &str,
    slug: &str,
    kind: Kind,
    after_insert: impl FnOnce(&Path, &Path, &[tree_grow::Renumber]) -> Result<()>,
) -> Result<()> {
    let guard = tree_access::write(grove_root)?;
    let target = resolve_ref_or_path_unlocked(guard.root(), target)?;
    let (path, renumbers) = tree_grow::leaf_insert_unlocked(guard.root(), &target, slug, kind)?;
    after_insert(guard.root(), &path, &renumbers)
}

fn cmd_leaf_decompose(args: &LeafDecomposeArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    // `None` (the default) inherits the decomposed leaf's own kind; `--kind`
    // overrides it (task-kind-taxonomy).
    let kind_override = args.kind.as_deref().map(Kind::parse).transpose()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
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

// Resolve `(worktree, grove_root)` from the cwd. The task-tree verbs run from
// the worktree root (not from inside `.grove/`), so the grove root is always
// `<worktree>/.grove`, matched across every verb so a leaf is never created or
// read one level off.
fn grove_paths() -> Result<(PathBuf, PathBuf)> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let worktree = repo::toplevel(&cwd)?;
    let grove_root = worktree.join(".grove");
    Ok((worktree, grove_root))
}

// Map a `<parent>` / `<target>` argument — a `resolve`-able key/slug/handle OR a
// path — to an absolute entry path. The driving flow passes back the **absolute**
// path `pick`/`resolve` printed (the path branch); a key/slug/handle is the
// convenience (the reference branch). Tried as a path first so an explicit,
// existing path always wins; only a non-existent path is re-tried as a reference,
// so the two namespaces never collide in practice.
fn resolve_ref_or_path_unlocked(grove_root: &Path, arg: &str) -> Result<PathBuf> {
    if let Some(path) = existing_path(grove_root, arg) {
        return Ok(path);
    }
    match tree_read::resolve_unlocked(grove_root, arg)? {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_access;
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
        tree_access::reset_acquisition_count();

        operation(&grove_root);

        assert_eq!(
            tree_access::acquisition_count(),
            1,
            "one CLI command must acquire the tree lock exactly once"
        );
    }

    #[test]
    fn reference_taking_commands_acquire_the_tree_lock_once() {
        assert_one_acquisition(|grove_root| {
            leaf_add_for(grove_root, "1", "later", Kind::Impl).unwrap();
        });
        assert_one_acquisition(|grove_root| {
            leaf_add_pair_for(grove_root, "1", "survey").unwrap();
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
    fn leaf_insert_lints_cross_references_under_its_only_exclusive_lock() {
        let (worktree, grove_root) = grove_with_node();
        fs::write(grove_root.join("NOTE.md"), "stale path: 01-impl-first-k2\n").unwrap();
        let mut output = ExclusiveLockAssertingWriter {
            worktree: worktree.path().to_path_buf(),
            bytes: Vec::new(),
        };
        tree_access::reset_acquisition_count();

        leaf_insert_for(
            &grove_root,
            "2",
            "earlier",
            Kind::Impl,
            |locked_root, _, renumbers| {
                tree_grow::surface_cross_refs_unlocked(locked_root, renumbers, &mut output)
            },
        )
        .unwrap();

        assert!(
            String::from_utf8_lossy(&output.bytes).contains("NOTE.md:1"),
            "fixture must exercise a cross-reference hit: {}",
            String::from_utf8_lossy(&output.bytes)
        );
        assert_eq!(tree_access::acquisition_count(), 1);
    }
}

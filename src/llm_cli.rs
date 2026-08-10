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
// no transitional dual-format reader — `grove migrate` (`tree_migrate`) is the
// only thing that reads an old (v1-flat or `NNN-slug`) tree, once, on adoption.

use crate::complete;
use crate::driver_lease;
use crate::leaf::Kind;
use crate::repo;
use crate::tree_access;
use crate::tree_grow;
use crate::tree_lifecycle;
use crate::tree_promotion;
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
They are separated from the human-facing `grove` binary; none of these verbs \
are meant for direct human use."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
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
    /// files remain ignored. The default output is a single lowercase
    /// token + newline (`--with-harness` is retained as a compatibility flag);
    /// `--json` selects the structured launch-peek shape. The self-driving loop
    /// calls this to resolve each session's launch harness and model from the
    /// picked leaf.
    Kind {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
        /// Compatibility flag used by the installed loop driver's structured
        /// peek. Current task bodies carry no launch harness, so the plain form
        /// remains one kind line and JSON always reports `harness: null`.
        #[arg(long = "with-harness")]
        with_harness: bool,
        /// Emit the picked leaf as one JSON object containing `path`, stable
        /// `handle`, `kind`, and compatibility fields `harness` / `review`,
        /// both null for current trees. An empty grove is the JSON literal
        /// `null`. The loop driver combines this with `--with-harness` and
        /// retains the result for the whole launch without reopening the tree.
        #[arg(long)]
        json: bool,
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
    LeafAdd(LeafAddArgs),
    /// Append a whole **review chain** under `<parent>` in one call — a
    /// `<stem>-chain` **node directory** holding `<stem>`, `<stem>-review`,
    /// `<stem>-integrate` — deriving the two step kinds from the one producer
    /// kind you name.
    ///
    /// A node and its three children, with four consecutive fresh keys:
    ///
    ///   NN    `<stem>-chain/`       the chain node — no `BRIEF.md`
    ///     01  `<producer>-<stem>-k…`
    ///     02  `review-<producer>-<stem>-review-k…`
    ///     03  `integrate-review-<producer>-<stem>-integrate-k…`
    ///
    /// The directory is what makes the group structural in any tree viewer, and
    /// it is **brief-less by rule** — that absence is how the Retire cascade
    /// tells a chain from a decomposition, so a chain node's close has nothing to
    /// do: no `Done when` to check, no brief to promote. Neither species' close
    /// asks a human anything (*confirmation-boundary*).
    ///
    /// You name the producer kind; the verb **derives** the other two. That
    /// derivation is what it is for: `--kind review-impl` beside a `design`
    /// producer is a valid invocation the `--kind` gate cannot catch, and it
    /// misroutes the whole chain's discipline. Reach for a chain when the
    /// artifact is load-bearing — a spec, a decomposition you will build on for
    /// months, a subsystem; a one-file change wants a mid-session subagent
    /// instead (`driving.md`).
    ///
    /// The whole shape is created or none of it is. Prints **four** absolute
    /// paths on stdout, the node directory first then its three leaves in
    /// position order — and prints nothing at all if the shape could not be
    /// created. Working-tree change only — no commit. Cutting no chain stays a
    /// normal choice; a step decided on *after* its producer already ran is
    /// `leaf-add <chain-node> <stem>-review` when the producer sits in a chain
    /// node, and `leaf-insert` when it was cut as a plain leaf.
    LeafAddChain(LeafAddChainArgs),
    /// Atomically promote the currently picked plain producer into a brief-less
    /// review-chain node while preserving the producer's stable handle and task
    /// bytes. The command derives the review and integration kinds, writes
    /// stable `Reviews` / `Integrates` relationships, and uses a visible
    /// `PROMOTING-*` witness so interruption is fail-closed and recoverable.
    ///
    /// `<picked-producer>` accepts the absolute path printed by `pick`, a stable
    /// key/handle, the same now-stale path on an idempotent retry, or the exact
    /// transaction path named by a fail-closed diagnostic. Prints node,
    /// relocated producer, review, and integration paths after success only.
    /// Finish to a reviewable boundary, commit under the unchanged producer
    /// handle, then retire the relocated producer.
    LeafPromoteChain(LeafPromoteChainArgs),
    /// Append a whole **research vendor pair** under `<parent>` in one call — a
    /// `<stem>-pair` **node directory** holding `<stem>-a`, `<stem>-b`,
    /// `<stem>-combine` — with fixed filename kinds and no routing metadata in
    /// task bodies.
    ///
    /// A node and its three children, with four consecutive fresh keys:
    ///
    ///   NN    `<stem>-pair/`     the chain node — no `BRIEF.md`
    ///     01  `research-a-<stem>-a`
    ///     02  `research-b-<stem>-b`
    ///     03  `combine-research-<stem>-combine`
    ///
    /// Research has no `review-` sibling, so its shape is a pair rather than a
    /// chain and gets its own verb. The two producer kinds are distinct routing
    /// targets by construction. Cut one when a question is load-bearing enough
    /// to pay for two independent corpora and blind spots.
    ///
    /// The whole shape is created or none of it is. Prints **four** absolute
    /// paths on stdout, the node directory first then its three leaves in
    /// position order — and prints nothing at all if the shape could not be
    /// created. Working-tree change only — no commit.
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
            Self::LeafAddChain(_) => "grove-llm leaf-add-chain",
            Self::LeafPromoteChain(_) => "grove-llm leaf-promote-chain",
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

/// `--kind` help for `leaf-add-chain`, whose `--kind` names the chain's *head*
/// and is **required**: `leaf-add`'s `impl` default would silently pick a
/// producer, which is precisely the wrong-but-well-formed kind this verb exists
/// to stop a session choosing by accident.
const CHAIN_KIND_HELP: &str = "Kind of the leaf that heads the chain — one of the five \
producers: requirements, design, planning, prototype, impl. The other two steps are \
derived from it (review-<producer>, integrate-review-<producer>); you never name them. \
research-a, research-b, combine-research, and finish are refused; research uses \
leaf-add-pair";

#[derive(Parser)]
pub struct LeafAddChainArgs {
    /// Parent node — `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path.
    pub parent: String,
    /// Shared stem for the node and all three leaves (lowercase ASCII letters,
    /// digits, dashes). The node is `<stem>-chain` and the steps are `<stem>`,
    /// `<stem>-review`, `<stem>-integrate` — suffixed, not prefixed, so a
    /// chain's handles sort together under its stem.
    pub stem: String,
    #[arg(long = "kind", help = CHAIN_KIND_HELP)]
    pub kind: String,
}

#[derive(Parser)]
pub struct LeafPromoteChainArgs {
    /// The currently picked plain producer: absolute picked path, stable key or
    /// handle, stale pre-promotion path, or exact `PROMOTING-*` recovery path.
    pub picked_producer: String,
    /// Emit one JSON object containing paths, stable handles, and whether this
    /// call changed the tree. Failures print no stdout.
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser)]
pub struct LeafAddPairArgs {
    /// Parent node — `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path.
    pub parent: String,
    /// Shared stem for the node and all three leaves (lowercase ASCII letters,
    /// digits, dashes). The node is `<stem>-pair` and the steps are `<stem>-a`,
    /// `<stem>-b`, `<stem>-combine` — the two producers are `a` and `b` because
    /// they are peers, not a leader and a follow-up.
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
    let cwd = std::env::current_dir().context("getting cwd for session epoch admission")?;
    let session_epoch = driver_lease::admit_ambient_session(&cwd, cli.command.operation_label())?;
    match cli.command {
        Command::RootInit(args) => cmd_root_init(&args),
        Command::Pick => cmd_pick(),
        Command::BriefChain { leaf_path } => cmd_brief_chain(leaf_path.as_deref()),
        Command::Kind {
            leaf_path,
            with_harness,
            json,
        } => cmd_kind(leaf_path.as_deref(), with_harness, json),
        Command::Resolve { reference } => cmd_resolve(&reference),
        Command::LeafAdd(args) => cmd_leaf_add(&args),
        Command::LeafAddChain(args) => cmd_leaf_add_chain(&args),
        Command::LeafPromoteChain(args) => cmd_leaf_promote_chain(&args),
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

fn cmd_kind(leaf_path: Option<&Path>, with_harness: bool, json: bool) -> Result<()> {
    let (worktree, grove_root) = grove_paths()?;
    // Normalize a cwd-relative path to what `tree_read::kind` accepts (absolute
    // or grove-root-relative), matching `cmd_brief_chain`'s handling; a `None`
    // stays `None` so the verb defaults to `pick`'s next live leaf.
    let leaf = leaf_path.map(normalize_leaf_path);
    // Three output shapes from one verb, and the plain one is the compatibility
    // contract: the kind alone, a single lowercase token. `--with-harness`
    // adds a second line only when the leaf declares a harness; `--json`
    // emits the complete routed-leaf fact used by the loop driver.
    let facts = if with_harness || json {
        tree_read::launch_peek(&grove_root, leaf.as_deref())?
    } else {
        tree_read::kind(&grove_root, leaf.as_deref())?.map(|kind| tree_read::LaunchPeek {
            path: PathBuf::new(),
            handle: String::new(),
            kind,
            harness: None,
            review: None,
        })
    };
    match facts {
        Some(facts) if json => {
            #[derive(serde::Serialize)]
            struct StructuredPeek<'a> {
                path: String,
                handle: &'a str,
                kind: &'static str,
                harness: Option<&'static str>,
                review: &'a Option<crate::task_relationship::ReviewEvidence>,
            }
            let output = StructuredPeek {
                path: facts.path.display().to_string(),
                handle: &facts.handle,
                kind: facts.kind.label(),
                harness: facts.harness.map(|harness| harness.name),
                review: &facts.review,
            };
            println!(
                "{}",
                serde_json::to_string(&output)
                    .context("serialising the structured routing peek")?
            );
        }
        Some(facts) => {
            println!("{}", facts.kind.label());
            if let Some(harness) = facts.harness {
                println!("{}", harness.name);
            }
        }
        None => {
            if json {
                println!("null");
            }
            eprintln!(
                "grove {}: no live leaves; this grove is done",
                label(&worktree)
            );
        }
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

/// Resolve the `<parent>` argument shared by the three appending verbs: `.` is
/// the grove root (the root node); anything else is a node by key or path.
/// `tree_grow` validates that the result is a real node directory.
fn resolve_parent_unlocked(grove_root: &Path, parent: &str) -> Result<PathBuf> {
    if parent == "." {
        Ok(grove_root.to_path_buf())
    } else {
        resolve_ref_or_path_unlocked(grove_root, parent)
    }
}

/// Print a composite verb's paths — **after** the mutation succeeded, never as
/// each leaf lands. A run that fails is rolled back, so stdout describing a
/// shape the command reported as failed would be describing files that are no
/// longer there.
fn print_paths(paths: &[PathBuf]) {
    for p in paths {
        println!("{}", p.display());
    }
}

fn cmd_leaf_add_chain(args: &LeafAddChainArgs) -> Result<()> {
    let producer = Kind::parse(&args.kind)?;
    let (_, grove_root) = grove_paths()?;
    let paths = leaf_add_chain_for(&grove_root, &args.parent, &args.stem, producer)?;
    print_paths(&paths);
    Ok(())
}

fn leaf_add_chain_for(
    grove_root: &Path,
    parent: &str,
    stem: &str,
    producer: Kind,
) -> Result<Vec<PathBuf>> {
    let guard = tree_access::write(grove_root)?;
    let parent_dir = resolve_parent_unlocked(guard.root(), parent)?;
    tree_grow::leaf_add_chain_unlocked(guard.root(), &parent_dir, stem, producer)
}

fn cmd_leaf_promote_chain(args: &LeafPromoteChainArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let result = tree_promotion::promote(&grove_root, &args.picked_producer)
        .map_err(|error| anyhow::anyhow!("promotion-failed: {error:#}"))?;
    if args.json {
        fn item(name: &str, item: &tree_promotion::PromotedItem) -> String {
            format!(
                "\"{name}\":{{\"path\":\"{}\",\"handle\":\"{}\"}}",
                crate::json::escape(&item.path.display().to_string()),
                crate::json::escape(&item.handle)
            )
        }
        println!(
            "{{\"changed\":{},{},{},{},{}}}",
            result.changed,
            item("node", &result.node),
            item("producer", &result.producer),
            item("review", &result.review),
            item("integration", &result.integration),
        );
    } else {
        print_paths(&result.paths().map(Path::to_path_buf));
    }
    Ok(())
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

fn cmd_leaf_retire(args: &LeafRetireArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let dst = tree_lifecycle::leaf_retire(&grove_root, &leaf_path)?;
    println!("{}", dst.display());
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
    Ok(())
}

// The write-side harness-name gate, mirroring `Kind::parse`: an unrecognised
// name is refused at authoring time, when a human is present to fix it.
fn parse_harness(name: Option<&str>) -> Result<Option<&'static crate::harness::Harness>> {
    let Some(name) = name else {
        return Ok(None);
    };
    crate::harness::by_name(name).map(Some).ok_or_else(|| {
        anyhow::anyhow!(
            "--harness must be one of {}, got {name:?}",
            crate::harness::known_names()
        )
    })
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
            leaf_add_chain_for(grove_root, "1", "sync", Kind::Impl).unwrap();
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

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
// `leaf-insert` / `leaf-decompose` / `leaf-retire` / `leaf-prune` / `root-init`)
// speak the **v2 directory scheme** (task-tree-scheme): they dispatch to the
// directory-based modules `tree_read` / `tree_grow` / `tree_lifecycle`. There is
// no transitional dual-format reader — `grove migrate` (`tree_migrate`) is the
// only thing that reads an old (v1-flat or `NNN-slug`) tree, once, on adoption.

use crate::complete;
use crate::leaf::Kind;
use crate::repo;
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
    /// `01-<slug>-k1.md` (default slug `plan`) — the kind is fixed, since the
    /// bootstrap session's only input is the human's own words. After this,
    /// `grove-llm pick` returns the new
    /// leaf — a fresh grove is no longer indistinguishable from a finished
    /// one. Refuses if `.grove/` already exists. Working-tree change only —
    /// no commit.
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
    /// Print a leaf's task **kind** — one of the closed seventeen documented in
    /// `docs/ARCHITECTURE.md#task-kind-taxonomy` — read from its `**Kind:**` line.
    /// With no argument the kind is read for `pick`'s next live leaf; on an
    /// empty grove it prints the standard "no live leaves" diagnostic on stderr
    /// (mirroring `brief-chain`) and exits 0. A missing or unrecognised
    /// `**Kind:**` line **degrades**: it warns on stderr and prints `impl`
    /// rather than erroring, so a hand-edited or foreign task file can never jam
    /// the self-driving loop. The previous spelling `work` resolves to `impl`
    /// silently — it is not a degrade. The default output is a single lowercase
    /// token + newline (`--with-harness` may add a second line — see the flag);
    /// `--json` selects the structured launch-peek shape. The self-driving loop
    /// calls this to resolve each session's launch harness and model from the
    /// picked leaf.
    Kind {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
        /// Also print the harness the leaf declares for itself, on a **second
        /// line**, when it carries a `**Harness:**` line (leaf-harness-k15).
        /// Undeclared — the overwhelmingly common case — prints nothing extra,
        /// so the output is byte-identical to the plain form. An unrecognised
        /// harness name **errors** here rather than degrading, because a wrong
        /// harness would run the leaf on a vendor the tree said not to.
        ///
        /// The loop driver's peek passes this: one subprocess, both launch
        /// facts. Without it the verb prints the kind and nothing else, and
        /// never inspects the harness line at all.
        #[arg(long = "with-harness")]
        with_harness: bool,
        /// Emit the picked leaf as one JSON object containing `path`, stable
        /// `handle`, `kind`, and nullable declared `harness`. An empty grove is
        /// the JSON literal `null`. The loop driver combines this with
        /// `--with-harness` and retains the result for the whole launch.
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
    ///     01  `<stem>`              **Kind:** `<producer>`
    ///     02  `<stem>-review`       **Kind:** `review-<producer>`
    ///     03  `<stem>-integrate`    **Kind:** `integrate-review-<producer>`
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
    LeafPromoteChain(LeafPromoteChainArgs),
    /// Append a whole **research vendor pair** under `<parent>` in one call — a
    /// `<stem>-pair` **node directory** holding `<stem>-a`, `<stem>-b`,
    /// `<stem>-combine` — with both producers' vendors declared, and refused if
    /// the two are the same.
    ///
    /// A node and its three children, with four consecutive fresh keys:
    ///
    ///   NN    `<stem>-pair/`     the chain node — no `BRIEF.md`
    ///     01  `<stem>-a`         **Kind:** research, **Harness:** `<harness-a>`
    ///     02  `<stem>-b`         **Kind:** research, **Harness:** `<harness-b>`
    ///     03  `<stem>-combine`   **Kind:** combine-research
    ///
    /// `research` is the one kind with no `review-` sibling, so its shape is a
    /// pair rather than a chain and gets its own verb. **Both producers declare
    /// their harness and the two must differ** — that declaration is the only
    /// thing that makes "two corpora" a fact in the tree rather than a forecast
    /// about routing policy, which can change after the leaves are cut. An equal
    /// pair is refused. Cut one when a question is load-bearing enough to pay
    /// for two vendors' corpora and blind spots.
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
    /// Convert a live leaf file `NN-<slug>-k<key>.md` into a node **directory**
    /// `NN-<slug>-k<key>/` (**key preserved** — the leaf that was `k<key>`
    /// becomes the node `k<key>`), moving the leaf body in as the node's
    /// `BRIEF.md` (`git mv`, or a plain rename in a jj-enabled tree; its
    /// `# <slug>-k<key>` header retitled with ` — brief`) and
    /// atomically growing a first child `01-<first-child-slug>-k<new>.md` so the
    /// node is never childless. The first child **inherits the decomposed
    /// leaf's own kind** unless `--kind` overrides it.
    /// Prints the brief's absolute path then the first child's, one per line.
    /// Working-tree change only — no commit.
    LeafDecompose(LeafDecomposeArgs),
    /// Mark a live leaf retired in place by adding a `DONE` infix
    /// (`NN-<slug>-k<key>.md` → `NN-DONE-<slug>-k<key>.md`) — no `done/`
    /// directory; the leaf keeps its position and key in its directory, and the
    /// file's contents (its `# <slug>-k<key>` header) are untouched. Refuses a
    /// brief, an already-retired (`DONE`) leaf, and an already-abandoned
    /// (`ABANDONED`) leaf. Prints the retired file's absolute
    /// path on stdout. When one sibling review declares `Reviews` for this
    /// producer, retirement applies `DONE` first and then writes its
    /// `Producer launch` receipt best-effort; receipt failure warns but never
    /// reverses or blocks retirement. Working-tree change only — no commit.
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
    /// found and left alone are reported on stderr. Working-tree change only —
    /// no commit.
    LeafPrune(LeafPruneArgs),
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
    /// **Not for you to call.** grove injects this as a claude hook at launch
    /// (`--settings`, per session, persisting nothing) so the harness can tell
    /// grove what the loop driver structurally cannot see: what is happening
    /// *inside* a session.
    ///
    /// The driver is the harness's parent, so it sees a session start and a
    /// session end and nothing between them — a session that stalls mid-session
    /// on a question, or on a permission prompt, reads `working` forever. This
    /// verb closes that half: at a turn end it reports **`blocked`** unless
    /// `$GROVE_SIGNAL_FILE` shows the task completed on purpose, at a turn start
    /// it reports `working`, and mid-turn it reports `blocked` while a dialog
    /// waits on a human, taking that back down at the next tool call. It reads
    /// no tree, writes no file, and prints nothing; with no herdr in the
    /// environment it does nothing at all.
    ReportTurn(ReportTurnArgs),
}

#[derive(Parser)]
pub struct ReportTurnArgs {
    /// Which moment fired: `start` (claude's `UserPromptSubmit`), `end`
    /// (`Stop`), `waiting` (`Notification`, a dialog waiting on a human) or
    /// `tool` (`PostToolUse`).
    #[arg(value_enum)]
    pub boundary: TurnBoundary,
}

/// The moments a claude session can report. A closed set with no
/// degrade-on-read arm, unlike a task kind: this argument is written by grove
/// itself at injection time, so an unrecognised value means a mismatched binary,
/// not a hand-edited file.
///
/// `Start`/`End` are the turn boundaries; `Waiting`/`Tool` are the mid-turn
/// pair, and are a *pair* because a mid-turn `blocked` needs a paired restore —
/// nothing else would take it back down before the turn ended.
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum TurnBoundary {
    Start,
    End,
    Waiting,
    Tool,
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
/// point of a help string is to teach the shape of the set; the full seventeen
/// are one `--kind reserch` away, from `Kind::parse`'s error.
const KIND_HELP: &str = "Leaf kind (task-kind-taxonomy), written into the templated \
`**Kind:**` line. Producers: requirements, design, planning, prototype, impl — each with \
its own review-<producer> and integrate-review-<producer> step; plus research and \
combine-research. An unrecognised value errors, listing all seventeen";

/// `--harness` help for the two verbs that *create* a leaf. Optional and rarely
/// used by design: it exists for the **vendor pair** — two `research` leaves and
/// the `combine-research` step that follows them — which is the one shape a
/// kind→harness policy cannot express (`docs/ARCHITECTURE.md#task-kind-taxonomy`).
/// Everything else routes by policy or falls through to the stamp, so the help
/// names the case rather than inviting general use.
const HARNESS_HELP: &str = "Harness this one leaf launches on, written into a \
`**Harness:**` line beside `**Kind:**` — it beats the per-kind and per-family policy \
vars and the grove's own stamp. Omit unless this leaf must run on a different vendor \
than its siblings (the research vendor pair). An unrecognised name errors";

/// [`KIND_HELP`] for `leaf-decompose`, whose `--kind` overrides an inherited
/// kind rather than supplying a default.
const KIND_OVERRIDE_HELP: &str = "Override the first child's kind — one of the seventeen \
(task-kind-taxonomy): requirements, design, planning, prototype, impl, each with its own \
review-<producer> and integrate-review-<producer> step, plus research and \
combine-research. Default: inherit the kind of the leaf being decomposed";

#[derive(Parser)]
pub struct LeafAddArgs {
    /// Parent node — `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path.
    pub parent: String,
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    #[arg(long = "kind", default_value = "impl", help = KIND_HELP)]
    pub kind: String,
    #[arg(long = "harness", help = HARNESS_HELP)]
    pub harness: Option<String>,
}

/// `--kind` help for `leaf-add-chain`, whose `--kind` names the chain's *head*
/// and is **required**: `leaf-add`'s `impl` default would silently pick a
/// producer, which is precisely the wrong-but-well-formed kind this verb exists
/// to stop a session choosing by accident.
const CHAIN_KIND_HELP: &str = "Kind of the leaf that heads the chain — one of the five \
producers: requirements, design, planning, prototype, impl. The other two steps are \
derived from it (review-<producer>, integrate-review-<producer>); you never name them. \
`research` is refused, naming leaf-add-pair — its shape is the vendor pair";

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
    /// Accepted only to be refused with the reason. Routing reviews is a
    /// *policy* (`GROVE_REVIEW_HARNESS`, one line covering all five `review-*`
    /// kinds), not a fact about one leaf — and clap's bare "unexpected argument"
    /// would teach a session nothing at the one moment it is deciding.
    #[arg(long = "harness", hide = true)]
    pub harness: Option<String>,
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
    /// Harness for the first survey, written as its `**Harness:**` line.
    /// Required, and must differ from `--harness-b`.
    #[arg(long = "harness-a")]
    pub harness_a: String,
    /// Harness for the second survey, written as its `**Harness:**` line.
    /// Required, and must differ from `--harness-a`.
    #[arg(long = "harness-b")]
    pub harness_b: String,
    /// Accepted only to be refused with the reason: a pair's kinds are fixed
    /// (`research`, `research`, `combine-research`), so `research` would be the
    /// flag's only legal value — a mode selector wearing a parameter's clothes.
    #[arg(long = "kind", hide = true)]
    pub kind: Option<String>,
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
    #[arg(long = "harness", help = HARNESS_HELP)]
    pub harness: Option<String>,
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
        Command::Complete(args) => cmd_complete(&args),
        Command::ReportTurn(args) => cmd_report_turn(&args),
    }
}

/// Report a turn moment and exit zero, always. Both halves matter to the
/// session this runs inside: a non-zero exit makes claude print a `hook error`
/// notice into the transcript at **every** turn — and, since `tool` fires per
/// tool call, at every tool call too — and this verb has nothing to fail at:
/// `herdr::report_turn` returns no `Result`, because a herdr that is absent,
/// refusing, or stock is a no-op by design, not an error to surface.
fn cmd_report_turn(args: &ReportTurnArgs) -> Result<()> {
    crate::herdr::report_turn(match args.boundary {
        TurnBoundary::Start => crate::herdr::Turn::Started,
        TurnBoundary::End => crate::herdr::Turn::Ended,
        TurnBoundary::Waiting => crate::herdr::Turn::Waiting,
        TurnBoundary::Tool => crate::herdr::Turn::ToolUsed,
    });
    Ok(())
}

fn cmd_complete(args: &CompleteArgs) -> Result<()> {
    let disposition = if args.done {
        complete::Disposition::Done
    } else {
        complete::Disposition::Relaunch
    };
    let opts = complete::resolve_opts(args.signal_file.clone(), disposition);
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
        })
    };
    match facts {
        Some(facts) if json => {
            let harness = facts.harness.map_or_else(
                || "null".to_string(),
                |harness| format!("\"{}\"", crate::json::escape(harness.name)),
            );
            println!(
                "{{\"path\":\"{}\",\"handle\":\"{}\",\"kind\":\"{}\",\"harness\":{}}}",
                crate::json::escape(&facts.path.display().to_string()),
                crate::json::escape(&facts.handle),
                facts.kind.label(),
                harness
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
    let harness = parse_harness(args.harness.as_deref())?;
    let parent_dir = resolve_parent(&grove_root, &args.parent)?;
    let path = tree_grow::leaf_add(&grove_root, &parent_dir, &args.slug, kind, harness)?;
    println!("{}", path.display());
    Ok(())
}

/// Resolve the `<parent>` argument shared by the three appending verbs: `.` is
/// the grove root (the root node); anything else is a node by key or path.
/// `tree_grow` validates that the result is a real node directory.
fn resolve_parent(grove_root: &Path, parent: &str) -> Result<PathBuf> {
    if parent == "." {
        Ok(grove_root.to_path_buf())
    } else {
        resolve_ref_or_path(grove_root, parent)
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
    // Argument refusals come before the repo is resolved: they are pure
    // authoring-time validation, and a caller who mis-typed a flag is better
    // served by hearing that than by hearing where they are standing.
    if let Some(name) = &args.harness {
        bail!(
            "leaf-add-chain takes no --harness (got {name:?}). Routing reviews is a \
             *policy*, not a fact about one leaf: set `GROVE_REVIEW_HARNESS={name}` once \
             and every review this grove ever cuts goes there (`GROVE_INTEGRATE_REVIEW_HARNESS` \
             for the integrate step). A `**Harness:**` line exists for the one shape no \
             policy can express — the research vendor pair, which is `leaf-add-pair`."
        );
    }
    let producer = Kind::parse(&args.kind)?;
    let (_, grove_root) = grove_paths()?;
    let parent_dir = resolve_parent(&grove_root, &args.parent)?;
    let paths = tree_grow::leaf_add_chain(&grove_root, &parent_dir, &args.stem, producer)?;
    print_paths(&paths);
    Ok(())
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
    if let Some(kind) = &args.kind {
        bail!(
            "leaf-add-pair takes no --kind (got {kind:?}). A pair's kinds are fixed — \
             research, research, combine-research — so `research` would be the flag's only \
             legal value: a mode selector wearing a parameter's clothes. For a chain over \
             some other producer, use `leaf-add-chain <parent> <stem> --kind {kind}`."
        );
    }
    // Both names are parsed before anything is resolved or written, so a typo in
    // the *second* one cannot leave the first survey on disk.
    let harness_a = require_harness(&args.harness_a)?;
    let harness_b = require_harness(&args.harness_b)?;
    let (_, grove_root) = grove_paths()?;
    let parent_dir = resolve_parent(&grove_root, &args.parent)?;
    let paths =
        tree_grow::leaf_add_pair(&grove_root, &parent_dir, &args.stem, harness_a, harness_b)?;
    print_paths(&paths);
    Ok(())
}

/// [`parse_harness`] for a *required* name — the vendor pair's two producers,
/// where an absent declaration is not an option ([`tree_grow::leaf_add_pair`]).
fn require_harness(name: &str) -> Result<&'static crate::harness::Harness> {
    crate::harness::by_name(name).ok_or_else(|| {
        anyhow::anyhow!(
            "--harness-a/--harness-b must each be one of {}, got {name:?}",
            crate::harness::known_names()
        )
    })
}

fn cmd_leaf_insert(args: &LeafInsertArgs) -> Result<()> {
    let (_, grove_root) = grove_paths()?;
    let kind = Kind::parse(&args.kind)?;
    let harness = parse_harness(args.harness.as_deref())?;
    let target = resolve_ref_or_path(&grove_root, &args.target)?;
    let (path, renumbers) =
        tree_grow::leaf_insert(&grove_root, &target, &args.slug, kind, harness)?;

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

// The `--harness` write gate, mirroring `Kind::parse`'s: an unrecognised name is
// refused at authoring time, when a human is present to fix it. The read side
// (`tree_read::read_harness`) refuses too — unlike the kind axis, which degrades
// on read — so this gate is a *sooner* failure, not the only one.
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

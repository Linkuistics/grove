// The LLM-driven CLI surface — `grove-llm`. See cli-binary-split
// (`docs/ARCHITECTURE.md#cli-binary-split`) for the audience-split
// rationale: every verb here exists for the LLM driving a grove session to
// invoke deterministically, not for a human at a terminal.
//
// Verbs are flat (hyphenated) so a single `grove-llm --help` enumerates every
// verb the LLM might call — important for bootstrap-recovery if a session
// drops context.
//
// # This crate is thin, and the compiler is what holds it thin
//
// Every verb below is one `grove_loop::verbs::` call plus rendering. Nothing
// here walks a tree, spells a filename or decides what a kind is: those live in
// a **different crate** since `loop-crate-verbs-k21`, so *the binary is thin*
// stopped being a discipline held by review the moment `grove-llm` stopped being
// a `[[bin]]` target inside the library it drives
// (`docs/specs/module-decomposition.md`, decision 1).
//
// What is left here that is not rendering is the **order** three verbs depend
// on, and each is stated where it happens: read the operator's text with the
// type that owns it *before* taking a lock, ask the just-in-time presence rule
// *before* the mutation, and admit the session against the completion channel
// *before* writing to it.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use grove_loop::verbs::{self, Resolution, Signalled};
use grove_loop::{
    Handle, Kind, Outcome, Reading, Reference, SessionConfig, SessionEpochGuard, Slug, Sought,
    Tree, TreeWrite, Writing,
};
use jj_workspace::Workspace;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "grove-llm",
    // **grove's version, not this package's.** One workspace, one release
    // version: `crates/grove-llm` carries a `0.1.0` that names nothing an
    // operator can install, and `grove --version` and `grove-llm --version` have
    // to agree because a skew between them is exactly what an operator reaches
    // for them to diagnose.
    version = grove_loop::VERSION,
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
    /// Print a leaf's task **kind** — the token before the `--` — read from its
    /// current-format filename. Any well-formed token is a kind: grove holds no
    /// list of them (`docs/adr/a-kind-is-an-open-token.md`).
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
    /// Append **one leaf per `--kind`**, in the order given, under `<parent>` at
    /// the next gapless child positions with consecutive fresh permanent keys —
    /// as one unit. `<parent>` is `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path. `--kind` is required and
    /// repeatable; every leaf of one call carries the same slug, because the kind
    /// is what tells a shape's steps apart. Prints each new leaf's absolute path
    /// on stdout in position order — and nothing at all if the run could not be
    /// created. Working-tree change only — no commit.
    ///
    /// **A list is how a research vendor pair is cut**, and the whole reason the
    /// list exists: `leaf-add <parent> <stem> --kind research-a --kind research-b
    /// --kind combine-research` lands three flat siblings at consecutive
    /// positions, all slugged `<stem>`, or lands none of them. Three separate
    /// calls are three chances to stop half way, and a live prefix of a pair is
    /// indistinguishable from a deliberately hand-cut partial one — so the run is
    /// atomic here rather than a rule the caller keeps.
    ///
    /// **A pair is created eagerly where a review chain is created lazily, and
    /// the asymmetry is deliberate.** If `research-a` cut `research-b` at the end
    /// of its own session, `b` would inherit `a`'s framing and corpus, destroying
    /// the independence the pair is run for. Cut one when a question is
    /// load-bearing enough to pay for two independent corpora and blind spots.
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
/// two hand-copied copies.
///
/// **It teaches the shape and lists nothing**, because there is nothing to list:
/// a kind is any well-formed token, and the set lives in the installed
/// methodology rather than in this binary
/// (`docs/adr/a-kind-is-an-open-token.md`). What the help owes instead is the
/// two facts a caller cannot guess — what the grammar accepts, and that a kind
/// no template declares is refused before the tree moves.
const KIND_HELP: &str = "Leaf kind, written into the filename: lowercase ASCII letters, \
digits and single dashes, no `--`. Grove holds no list of kinds — the installed methodology \
does — so any well-formed token is accepted here and refused later if no launch template \
declares it. `finish` is driver-reserved and refused by this verb";

/// [`KIND_HELP`] for `leaf-decompose`, whose `--kind` overrides an inherited
/// kind rather than supplying a default.
const KIND_OVERRIDE_HELP: &str = "Override the first child's filename kind — any session kind \
except driver-reserved finish. Default: inherit the kind of the leaf being decomposed";

/// A `--kind` argument, as the grammar's own type.
///
/// The refusal is the grammar's — a shape refusal naming the character it
/// refused — with the flag quoted in front of it, so an operator sees which
/// argument was rejected as well as why.
fn parse_kind(token: &str) -> Result<Kind> {
    Kind::new(token).map_err(|error| anyhow::anyhow!("--kind {token:?}: {error}"))
}

#[derive(Parser)]
pub struct LeafAddArgs {
    /// Parent node — `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path.
    pub parent: String,
    /// Slug every leaf of this call carries (lowercase ASCII letters, digits,
    /// dashes).
    pub slug: String,
    /// One leaf is written per occurrence, in the order given.
    ///
    /// **Required, and it used to default to `impl`.** A default is a kind
    /// literal under a friendlier name, and it is the one that would silently
    /// produce a *wrong* leaf rather than an error — a session that forgot the
    /// flag got an `impl` leaf and no complaint (`open-kind-k20`).
    #[arg(long = "kind", required = true, help = KIND_HELP)]
    pub kind: Vec<String>,
}

#[derive(Parser)]
pub struct LeafInsertArgs {
    /// Target entry — the existing leaf or node (by key or path) whose slot the
    /// new leaf should occupy; the target and later siblings shift up by one.
    pub target: String,
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    /// Required, for the reason `leaf-add`'s is (`open-kind-k20`).
    #[arg(long = "kind", required = true, help = KIND_HELP)]
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
    let session_epoch = grove_loop::admit_ambient_session(&cwd, command.operation_label())?;
    match command {
        Command::RootInit(args) => cmd_root_init(&args),
        Command::Pick => cmd_pick(),
        Command::BriefChain { leaf_path } => cmd_brief_chain(leaf_path.as_deref()),
        Command::Kind { leaf_path } => cmd_kind(leaf_path.as_deref()),
        Command::Resolve { reference } => cmd_resolve(&reference),
        Command::LeafAdd(args) => cmd_leaf_add(&args),
        Command::LeafInsert(args) => cmd_leaf_insert(&args),
        Command::LeafDecompose(args) => cmd_leaf_decompose(&args),
        Command::LeafRetire(args) => cmd_leaf_retire(&args),
        Command::LeafPrune(args) => cmd_leaf_prune(&args),
        Command::FinishCommit { finish_handle } => cmd_finish_commit(&finish_handle),
        Command::Complete(args) => cmd_complete(&args, session_epoch.as_ref()),
    }
}

fn cmd_finish_commit(finish_handle: &str) -> Result<()> {
    let worktree = worktree()?;
    // The argument arrives as text from a session's command line, so it is read
    // by the type that owns the grammar rather than compared as a string: a
    // handle that is not one is told *why*, and only a well-formed handle
    // reaches the verb.
    //
    // **The operator's own spelling is quoted here, and nowhere deeper.**
    // `Handle::parse` is deliberately lenient on the key — `finish-k0001` is the
    // live `finish-k1` and is accepted, which is right, since the operator meant
    // that leaf — so everything past this point speaks in canonical handles. A
    // refusal that only showed those would be answering a question the operator
    // did not ask, and this is the last frame that still has what they typed.
    let finish = Handle::parse(finish_handle)?;
    let workspace = Workspace::resolve(&worktree).context("cannot commit the finished grove")?;
    let commit = verbs::finish_commit(&workspace, &finish)
        .with_context(|| format!("`grove-llm finish-commit {finish_handle}`"))?;
    eprintln!("finish-commit {finish}: committed as {}", commit.change_id);
    Ok(())
}

fn cmd_complete(args: &CompleteArgs, session_epoch: Option<&SessionEpochGuard>) -> Result<()> {
    // **Asked before the write, which is why the channel is resolved here.** The
    // lease admits this session against the channel it is about to signal, and
    // an answer that came back with the signal would come back too late.
    let channel = verbs::signal_channel(args.signal_file.as_deref());
    if let Some(session_epoch) = session_epoch {
        session_epoch.require_signal_path(channel.as_deref())?;
    }
    match verbs::complete(channel.as_deref(), args.done)? {
        Signalled::Wrote(_) => {
            let tail = if args.done {
                "the grove is finished — the loop will stop"
            } else {
                "the loop will start the next task"
            };
            eprintln!("grove complete: signalled; {tail}.");
        }
        Signalled::NoLoop => eprintln!(
            "grove complete: no GROVE_SIGNAL_FILE — not running under the loop driver; \
             exit this session manually."
        ),
    }
    Ok(())
}

fn cmd_root_init(args: &RootInitArgs) -> Result<()> {
    let worktree = worktree()?;
    // Read before the lock: refusing a bad slug without taking an exclusive one
    // is strictly kinder, and the kind's presence rule is asked before anything
    // is written.
    let slug = slug(&args.slug)?;
    let kind = Kind::requirements();
    require_declared(&worktree, std::slice::from_ref(&kind))?;
    // The refusal to clobber is the **shape** rather than a check: a live grove
    // opens as a tree, and `root-init` takes a vacancy.
    //
    // `match` rather than `let … else`, and the reason is not style. A `let …
    // else` binding drops the unmatched value at the end of the whole statement
    // — *after* the `else` block — so the live `TreeWrite` would still be
    // holding the exclusive lock while that block ran, and a later improvement
    // to this message that read the tree to name the live leaf would deadlock
    // against this process. `match` drops it on entry to the arm.
    let vacancy = match grove_loop::write(&worktree)? {
        Writing::Vacancy(vacancy) => vacancy,
        Writing::Tree(_) => bail!(
            "grove root already exists: {}",
            worktree.join(".grove").display()
        ),
    };
    let initialized = verbs::root_init(vacancy, &slug, &kind)?;
    println!("{}", initialized.brief.display());
    println!("{}", initialized.first_leaf.display());
    Ok(())
}

fn cmd_pick() -> Result<()> {
    let worktree = worktree()?;
    let tree = readable(&worktree)?;
    match verbs::pick(&tree)? {
        Sought::Match(selection) => println!("{}", selection.path.display()),
        Sought::Nothing => no_live_leaves(&worktree),
    }
    Ok(())
}

fn cmd_brief_chain(leaf_path: Option<&Path>) -> Result<()> {
    let worktree = worktree()?;
    let tree = readable(&worktree)?;
    let Some(leaf) = leaf_in(&tree, leaf_path)? else {
        no_live_leaves(&worktree);
        return Ok(());
    };
    for path in verbs::brief_chain(&tree, &leaf)? {
        println!("{}", path.display());
    }
    Ok(())
}

fn cmd_kind(leaf_path: Option<&Path>) -> Result<()> {
    let worktree = worktree()?;
    let tree = readable(&worktree)?;
    // Normalize a cwd-relative path to what the verb accepts (absolute or
    // grove-root-relative); a `None` stays `None` so the verb defaults to
    // `pick`'s next live leaf.
    let leaf = leaf_path.map(normalize_leaf_path);
    match verbs::kind(&tree, leaf.as_deref())? {
        Sought::Match(kind) => println!("{}", kind.label()),
        Sought::Nothing => no_live_leaves(&worktree),
    }
    Ok(())
}

/// The leaf a read verb acts on: the one named, or `pick`'s next.
fn leaf_in(tree: &Tree, leaf_path: Option<&Path>) -> Result<Option<PathBuf>> {
    Ok(match leaf_path {
        Some(path) => Some(normalize_leaf_path(path)),
        None => match verbs::pick(tree)? {
            Sought::Match(selection) => Some(selection.path),
            Sought::Nothing => None,
        },
    })
}

fn cmd_resolve(reference: &str) -> Result<()> {
    let worktree = worktree()?;
    let parsed = Reference::parse(reference)?;
    let resolution = verbs::resolve(&readable(&worktree)?, &parsed)?;
    // **The root is the one answer with no entry behind it**, so its path is the
    // one thing rendering cannot supply: it is the caller's own spelling of the
    // tree, not something read out of it.
    if matches!(resolution, Sought::Match(Resolution::Root)) {
        println!("{}", worktree.join(".grove").display());
        return Ok(());
    }
    let (stdout, stderr) = render_resolution(reference, &resolution);
    // `resolve` is pick-style: a not-found / ambiguous reference is reported on
    // stderr and still exits zero (it is information, not an error).
    print!("{stdout}");
    eprint!("{stderr}");
    Ok(())
}

/// Render a resolution to the `(stdout, stderr)` the `resolve` verb emits.
///
/// Kept pure and separate from the I/O so the exact contract is unit-testable
/// without going through the CLI dispatch. It lives here rather than in
/// `grove-loop` because it is presentation and nothing else — the loop crate
/// answers what a reference names, and what a terminal should see about it is
/// this binary's alone.
#[must_use]
pub fn render_resolution(reference: &str, resolution: &Sought<Resolution>) -> (String, String) {
    match resolution {
        // Unreachable through `resolve`, which answers the root before it gets
        // here: the root's path is the caller's own spelling of the tree, and
        // nothing in a resolution carries it.
        Sought::Match(Resolution::Root) => (String::new(), String::new()),
        Sought::Match(Resolution::Entry(entry)) => {
            let stdout = format!("{}\n", entry.path.display());
            let stderr = match entry.outcome {
                Outcome::Live => String::new(),
                Outcome::Done => format!(
                    "note: referenced task is retired (DONE): {}\n",
                    entry.path.display()
                ),
                // The abandoned counterpart of the DONE note above: `resolve`
                // must not let a pruned dead end look live.
                Outcome::Abandoned => format!(
                    "note: referenced task is abandoned (ABANDONED): {}\n",
                    entry.path.display()
                ),
            };
            (stdout, stderr)
        }
        Sought::Nothing => (
            String::new(),
            format!("resolve: no entry matches reference {reference:?}\n"),
        ),
        Sought::Match(Resolution::Ambiguous(matches)) => {
            let mut stderr =
                format!("resolve: reference {reference:?} is ambiguous; re-query by key:\n");
            for matched in matches {
                let tag = match matched.outcome {
                    Outcome::Live => "",
                    Outcome::Done => " (retired)",
                    Outcome::Abandoned => " (abandoned)",
                };
                stderr.push_str(&format!(
                    "  [{}] {}{}\n",
                    matched.handle.key(),
                    matched.path.display(),
                    tag
                ));
            }
            (String::new(), stderr)
        }
    }
}

fn cmd_leaf_add(args: &LeafAddArgs) -> Result<()> {
    let worktree = worktree()?;
    let kinds = args
        .kind
        .iter()
        .map(|token| parse_kind(token))
        .collect::<Result<Vec<_>>>()?;
    let slug = slug(&args.slug)?;
    let parent = Reference::parse(&args.parent)?;
    require_declared(&worktree, &kinds)?;
    let tree = writable(&worktree)?;
    print_paths(&verbs::leaf_add(&tree, &parent, &slug, &kinds)?);
    Ok(())
}

/// Print an add's paths — **after** the mutation succeeded, never as each leaf
/// lands. A run that fails is rolled back, so stdout describing a shape the
/// command reported as failed would be describing files that are no longer
/// there.
fn print_paths(paths: &[PathBuf]) {
    for p in paths {
        println!("{}", p.display());
    }
}

fn cmd_leaf_insert(args: &LeafInsertArgs) -> Result<()> {
    let worktree = worktree()?;
    let kind = parse_kind(&args.kind)?;
    let slug = slug(&args.slug)?;
    let target = Reference::parse(&args.target)?;
    require_declared(&worktree, std::slice::from_ref(&kind))?;
    let tree = writable(&worktree)?;
    let inserted = verbs::leaf_insert(&tree, &target, &slug, &kind)?;
    report_insert(&tree, &args.slug, &inserted)
}

/// `leaf-insert`'s output: the new leaf's path on stdout, the renumber summary
/// and the cross-reference lint on stderr.
///
/// Keep the established CLI stream semantics: the standard print macros panic on
/// a broken stdout/stderr, while the cross-reference lint is the one output that
/// is allowed to come back empty — its own failures are the tree's, never the
/// sink's, because the insert has already landed by the time it runs. So the
/// hits go out through `writeln!(…).ok()` rather than `eprintln!`, and this is
/// the site that decides it: the verb hands back a list and holds no opinion
/// about the sink (`lint-lock-scope-k32`).
///
/// **Nothing here is written under a tree lock.** The verb's shared guard is
/// consumed by its own scan, so a stderr that has stopped draining blocks this
/// process and no other — where a lint that printed under the tree's exclusive
/// lock wedged every grove on the worktree behind a stalled harness.
fn report_insert(tree: &TreeWrite, slug: &str, inserted: &verbs::Inserted) -> Result<()> {
    println!("{}", inserted.path.display());
    let renumbered = &inserted.renumbered;
    if renumbered.is_empty() {
        eprintln!("leaf-insert {slug}: no siblings to renumber");
        return Ok(());
    }
    eprintln!(
        "leaf-insert {}: renumbered {} sibling{}:",
        slug,
        renumbered.len(),
        if renumbered.len() == 1 { "" } else { "s" }
    );
    for renumber in renumbered {
        eprintln!(
            "  {:02} -> {:02}  ({})",
            renumber.from_position,
            renumber.to_position,
            renumber.to_name()
        );
    }
    let hits = verbs::stale_cross_refs(tree, renumbered)?;
    eprintln!("cross-references to review (verb does not auto-rewrite):");
    let mut stderr = std::io::stderr();
    for hit in &hits {
        writeln!(stderr, "{hit}").ok();
    }
    Ok(())
}

fn cmd_leaf_decompose(args: &LeafDecomposeArgs) -> Result<()> {
    let worktree = worktree()?;
    // `None` (the default) inherits the decomposed leaf's own kind; `--kind`
    // overrides it (task-kind-taxonomy).
    let kind_override = args.kind.as_deref().map(parse_kind).transpose()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let first_child = slug(&args.first_child_slug)?;
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
    //
    // **This read happens before the tree is opened for writing, and the order
    // is load-bearing.** Both take a lock on the same directory through their
    // own file description, and two descriptions do not share an `flock` — so
    // reading the inherited kind while holding the write opening would block
    // this process against itself, forever. `writable` is therefore the last
    // thing before the verb, here and at every other call site.
    let child_kind = match &kind_override {
        Some(kind) => Some(kind.clone()),
        None => inherited_kind(&worktree, &leaf_path),
    };
    if let Some(kind) = &child_kind {
        require_declared(&worktree, std::slice::from_ref(kind))?;
    }
    let tree = writable(&worktree)?;
    let decomposed =
        verbs::leaf_decompose(&tree, &leaf_path, &first_child, kind_override.as_ref())?;
    println!("{}", decomposed.brief.display());
    println!("{}", decomposed.first_child.display());
    Ok(())
}

/// The kind a `leaf-decompose` with no `--kind` will write, or nothing readable.
fn inherited_kind(worktree: &Path, leaf_path: &Path) -> Option<Kind> {
    let Ok(Reading::Tree(tree)) = grove_loop::read(worktree) else {
        return None;
    };
    match verbs::kind(&tree, Some(leaf_path)) {
        Ok(Sought::Match(kind)) => Some(kind),
        _ => None,
    }
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
    let worktree = worktree()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let tree = writable(&worktree)?;
    let dst = verbs::leaf_retire(&tree, &leaf_path)?;
    println!("{}", dst.display());
    eprint_next_steps("leaf-retire", 1);
    Ok(())
}

fn cmd_leaf_prune(args: &LeafPruneArgs) -> Result<()> {
    let worktree = worktree()?;
    let path = normalize_leaf_path(&args.path);
    let tree = writable(&worktree)?;
    let result = verbs::leaf_prune(&tree, &path)?;
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
/// enumerates the kinds a methodology declares — since `open-kind-k20` there is
/// no enumeration to make it from — so the only honest question is about the
/// kind in hand.
///
/// It runs **before** the tree is opened, so a refusal leaves the tree
/// byte-identical and takes no exclusive lock on the way — and it loads the
/// whole configuration to ask, which is what keeps the other half of the
/// amendment true: every template rule in both documents is still checked
/// eagerly, and a malformed entry for a kind this call will never touch still
/// fails here.
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

/// A slug argument, as the grammar's own type.
///
/// One type owns the name (principle 3), so the text is read here — where an
/// operator typed it — and every verb below takes the validated [`Slug`].
fn slug(text: &str) -> Result<Slug> {
    Slug::new(text).map_err(|error| anyhow::anyhow!("slug {text:?}: {error}"))
}

// Resolve the worktree from the cwd. The task-tree verbs run from the worktree
// root (not from inside `.grove/`), and `grove-loop` joins `.grove` itself, so
// no caller here can spell the grove root a second way.
fn worktree() -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    Ok(Workspace::resolve(&cwd)?.root().to_path_buf())
}

/// The shared opening a read verb needs, refusing a worktree with no grove.
///
/// **A grove that is not there is not a grove that is finished**, and every verb
/// here refuses rather than conflating them: a session run one directory off
/// would otherwise be told its work was done. [`grove_loop::read`] answers the
/// vacancy because the *driver* scaffolds one; a session cannot, so this is
/// where the two callers part.
fn readable(worktree: &Path) -> Result<Tree> {
    match grove_loop::read(worktree)? {
        Reading::Tree(tree) => Ok(tree),
        Reading::Vacant => Err(absent(&worktree.join(".grove"))),
    }
}

/// The exclusive opening a mutating verb needs, refusing a worktree with no
/// grove.
fn writable(worktree: &Path) -> Result<TreeWrite> {
    match grove_loop::write(worktree)? {
        Writing::Tree(tree) => Ok(tree),
        Writing::Vacancy(vacancy) => Err(absent(vacancy.root())),
    }
}

/// The refusal for a worktree that holds no grove — one wording, and it carries
/// the remedy, because an error that only reports detection is unfinished
/// (principle 2).
fn absent(grove_root: &Path) -> anyhow::Error {
    anyhow::anyhow!(
        "grove root not found: {}\n\nScaffold one with `grove-llm root-init`.",
        grove_root.display()
    )
}

// Normalize a user-supplied leaf path to what the verbs accept (absolute, or
// relative to the grove root). The real driving flow passes back the **absolute**
// path `pick`/`brief-chain` printed — handled by the absolute branch. A path given
// relative to the cwd that exists (e.g. `.grove/01-foo-k1.md` typed at the
// worktree root) is resolved to absolute here; a bare grove-root-relative name
// (`01-foo-k1.md`) does not exist relative to cwd, so it passes through for the
// verb to join onto the grove root.
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

/// The one diagnostic every read verb shares, printed once here rather than
/// spelled four times.
fn no_live_leaves(worktree: &Path) {
    eprintln!(
        "grove {}: no live leaves; this grove is done",
        label(worktree)
    );
}

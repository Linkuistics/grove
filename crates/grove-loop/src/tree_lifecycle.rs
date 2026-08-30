// **This module is what happens to a grove that the store has no word for: it
// begins, its leaves reach terminal outcomes, and it ends.** Everything about
// ordinals, keys and locks is `ordinal-fs-tree`'s, and since
// `collapse-tree-access-k13` that includes the lock — the grove's own creation
// was the last thing here that reached past the store, and it goes through
// `Vacancy::initialize` now.
//
// The **lifecycle verbs** (task-tree-scheme) — `root-init`, `leaf-decompose`,
// `leaf-retire`, `leaf-prune`, and the driver-owned finish lifecycle — expressed against the real **directory
// tree**, built on the name grammar (`src/task_name.rs`) and the grow verbs
// (`src/task_grow.rs`), both of which run through `ordinal-fs-tree` since the
// flip (gh issue #13, increment 2). Keeps task-tree-scheme's *semantics* (a fresh grove
// starts with one live leaf so it is never mistaken for finished; decompose
// enforces a first child; retire is leaves-only and done-ness is marked in
// place; prune marks abandonment in place, pruning) and changes the
// *mechanics* to the filesystem's shape:
//
//   * `root-init` creates the grove whole — the root, its `BRIEF.md` (the one
//     unkeyed singleton) and a first **requirements** leaf
//     `01-requirements-<slug>-k1.md`, a 2-digit per-level position — as one
//     store operation under one lock;
//   * `leaf-decompose` turns the leaf *file* `NN-<kind>--<slug>-k<key>.md` into a node
//     *directory* `NN-<slug>-k<key>/` (**key preserved** — the entity that was the
//     leaf becomes the node), renaming the leaf body in as the node's `BRIEF.md`
//     and growing a first child atomically so a node is never childless;
//   * `leaf-retire` adds a `DONE` infix in place (`NN-<kind>--<slug>-k<key>.md` →
//     `NN-DONE-<kind>--<slug>-k<key>.md`), keeping the retired leaf in its directory at its
//     position — no `done/` directory;
//   * `leaf-prune` adds an `ABANDONED` infix in place, symmetric with retire, but
//     — per pruning — accepts a **node** too: marking every *live* leaf
//     in the subtree (bulk, since one decision can kill many leaves at once),
//     leaving `DONE` leaves alone.
//   * the finish lifecycle materializes one resumable finish leaf for an otherwise
//     empty tree, then revalidates and commits the tree's removal after explicit
//     human confirmation.
//
// **Position-free headers:** a leaf/brief header is the stable handle
// `# <slug>-k<key>` (`# … — brief` for a node), so `leaf-retire`/`leaf-prune`
// leave the file's content byte-identical (the outcome infix is filename-only)
// and `leaf-decompose` only appends ` — brief` to the handle.
//
// These are the verbs `llm_cli` dispatches; the flat-scheme `leaf_lifecycle` they
// replaced is gone.

use crate::task_grow;
use crate::task_name::{Handle, Kind, Outcome, Parts, Slug, TaskName};
use crate::task_tree::{self, Guard, Opening, TreeVacancy};
use anyhow::{bail, Context, Result};
use jj_workspace::{Commit, Workspace};
use ordinal_fs_tree::{Entry, EntryName, Found, Key, NewEntry, Report, Snapshot, Target, Verdict};
use std::fs;
use std::path::{Path, PathBuf};

/// The slug a grove scaffolded by the driver is named with. `root-init` takes
/// one from its operator; the lifecycle transition has nobody to ask.
const DEFAULT_ROOT_SLUG: &str = "plan";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrentTransition {
    RootInitialized,
    AlreadyCurrent,
}

/// Classify or perform the one lifecycle transition needed before current-tree
/// selection: a working tree with no grove gets one.
///
/// **One opening, one lock, one operation.** This used to be two phases —
/// classify under grove's own guard, then append the first leaf under the
/// library's — because the two `flock` the same directory through different
/// open file descriptions and would have deadlocked nested. Grove holds no lock
/// of its own since `collapse-tree-access-k13`, and the store answers *there is
/// no tree here* as a **shape** rather than an error, so the whole transition is
/// one [`task_tree::write_or_vacancy`] and the vacancy arm creates the grove
/// under the lock it already holds.
pub(crate) fn transition_to_current(worktree: &Path) -> Result<CurrentTransition> {
    let grove_root = worktree.join(".grove");
    match task_tree::write_or_vacancy(&grove_root)? {
        Opening::Vacancy(vacancy) => {
            initialize_grove(
                vacancy,
                &grove_name(&grove_root),
                &default_root_slug(),
                &Kind::requirements(),
            )?;
            Ok(CurrentTransition::RootInitialized)
        }
        Opening::Tree(tree) => match root_shape(&tree)? {
            RootShape::ATree => Ok(CurrentTransition::AlreadyCurrent),
            RootShape::Taskless => bail!(
                "the task tree in {} holds no task, only its charter. Grove creates a grove \
                 whole — the root, its `BRIEF.md` and a first leaf in one operation — so this \
                 is a tree something emptied rather than one Grove left half-built, and Grove \
                 does not repair a tree in place: put it back with `jj undo`, or move {} aside \
                 and let `grove` scaffold a fresh one",
                grove_root.display(),
                grove_root.display()
            ),
            RootShape::Unrecognised(names) => bail!(
                "the task tree in {} holds no Grove entries, only {}. Grove reads \
                 `NN-<kind>--<slug>-k<key>` names and does not migrate older layouts: rename these \
                 entries to that grammar, or move them aside and let `grove` scaffold a fresh tree",
                grove_root.display(),
                names.join(", ")
            ),
        },
    }
}

/// Materialize the driver-owned finish sentinel after a shared selection found
/// no live work. The exclusive re-selection closes the gap between that read
/// and allocation: newly inserted ordinary work wins, and an existing finish
/// is reused.
pub(crate) fn materialize_finish(tree: Guard) -> Result<crate::task_tree::Selection> {
    if let Some(selection) = task_tree::select_in_write(&tree)? {
        return Ok(selection);
    }

    // **The sentinel is the one leaf grove creates that no operator verb may.**
    // The library will `append` anything `Parts` can express, so what keeps a
    // `finish` kind out of `leaf-add` is `task_grow::refuse_finish_kind` on
    // grove's side, and this verb is the deliberate exception rather than a hole
    // in the check.
    let key = task_tree::next_key(tree.snapshot());
    // The body embeds the handle, and the handle embeds the key the library has
    // not allocated yet — so the key is predicted here and held to account by
    // `task_grow::allocated` below, exactly as every grow verb does
    // (`docs/ARCHITECTURE.md#tree-access-lock`).
    let entry = new_finish_leaf(key)?;
    let report = tree
        .append(Target::Root, entry)
        .map_err(task_tree::raised)?;
    let path = task_grow::allocated(report.created(), &[key])?.remove(0);
    Ok(crate::task_tree::Selection {
        path,
        handle: finish_handle(key.context("a finish sentinel was created without a key")?)?,
        kind: Kind::finish(),
    })
}

/// The driver's finish sentinel, as an entry to create.
///
/// A `None` key is an exhausted keyspace, which is
/// [`Refusal::KeysExhausted`](ordinal_fs_tree::Refusal) and the library's to
/// state; the entry carries no bytes then and never needs any, because a refusal
/// writes nothing (`task_grow::new_leaf` says the same of an ordinary leaf).
fn new_finish_leaf(key: Option<Key>) -> Result<NewEntry<Parts>> {
    let parts = Parts::leaf(Outcome::Live, Kind::finish(), finish_slug()?);
    Ok(match key {
        Some(key) => NewEntry::new(parts, finish_body(&finish_handle(key)?).into_bytes()),
        None => NewEntry::empty(parts),
    })
}

/// The driver's own leaf wears the handle `finish-k<key>`, composed through
/// [`Handle`] rather than spelled here — one of the five hand-rolled produce
/// sites `name-ownership-k14` retired.
fn finish_handle(key: Key) -> Result<Handle> {
    Ok(Handle::new(finish_slug()?, key))
}

/// `finish`, validated as a [`Slug`]. A constant that has to go through the
/// validating constructor is still one constant, and going through it is what
/// keeps `Slug`'s guarantee — *a `Slug` that exists renders and re-parses* —
/// true of every `Slug` in the process rather than of most of them.
fn finish_slug() -> Result<Slug> {
    Slug::new("finish").map_err(|error| anyhow::anyhow!("slug \"finish\": {error}"))
}

fn finish_body(handle: &Handle) -> String {
    format!(
        "# {handle}\n\n\
         ## Goal\n\n\
         Propose the complete finish cycle and wait for explicit human confirmation.\n\n\
         ## Done when\n\n\
         - Promote durable material from the grove briefs.\n\
         - Run `grove-llm finish-commit {handle}`.\n\
         - Run `grove-llm complete --done` as the last action.\n"
    )
}

/// Revalidate the complete finish cycle's tree facts, delete `.grove/`, and
/// take the commit that records the deletion. This is a deterministic
/// last-moment guard; whether a human confirmed teardown is the calling finish
/// session's responsibility.
///
/// **The tree and VCS facts are the verb; the transaction around them is gone**
/// (`delete-finish-transaction-k8`). What survives is exactly what only grove
/// can say: that the live leaf is the driver-owned finish leaf the caller named,
/// that no ordinary work slipped in, and that only `.grove/` is deleted and
/// committed. What went is the witness, the manifest, the evacuation, the
/// rollback proof, the quarantine and the recovery path — jj snapshots the
/// working copy before every command and its operation log is the transaction
/// record, so a teardown that does not commit is restored by `jj undo`, which is
/// what the seam's own refusal says ([`jj_workspace::Workspace::commit`]).
pub(crate) fn finish_commit(workspace: &Workspace, finish: &Handle) -> Result<Commit> {
    let worktree = workspace.root();
    let grove_root = worktree.join(".grove");
    // **The root is classified before the tree is opened, because two of the
    // three answers are not the library's to give.** A `.grove` that is a
    // *symlink* to a directory is one the library would happily read — it
    // follows links, as every reader does — while this verb must refuse it
    // unfollowed, since it may not delete a directory elsewhere as if it were
    // its own tree. The guard below is still the authority on the tree; this is
    // the wording that authority cannot supply.
    match fs::symlink_metadata(&grove_root) {
        // Absence is now a plain refusal. It used to be routed to a proof that
        // the repository's immediate result *was* this attempt's teardown
        // commit, because a death mid-transaction exposed exactly this shape and
        // grove had to decide whether to re-run. There is no transaction to die
        // inside any more, so the answer is the operator's and the operation log
        // is where they read it.
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => bail!(
            "no Grove task tree at {}\n\n\
             If a `finish-commit` was interrupted after the deletion, `jj op log` shows whether \
             it committed and `jj undo` restores the tree if it did not.",
            grove_root.display()
        ),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("checking grove root {}", grove_root.display()))
        }
        Ok(metadata) if !metadata.file_type().is_dir() => {
            bail!("grove root is not a directory: {}", grove_root.display())
        }
        Ok(_) => {}
    }
    let tree = task_tree::write(&grove_root)?;
    let selection = task_tree::select_in_write(&tree)?
        .context("the requested finish leaf is no longer live")?;
    if !selection.kind.is_finish() {
        bail!(
            "cannot finish while live work remains: {} ({})",
            selection.handle,
            selection.path.display()
        );
    }
    // **The refusal quotes what the operator asked for; everything downstream
    // uses the tree's own handle.** `Handle::parse` — which the caller ran to
    // get here — is deliberately lenient on the key's spelling, so
    // `finish-k0001` is the live `finish-k1` and compares equal, which is right:
    // the operator meant that leaf. But the teardown commit is a permanent
    // record and must name the work item by the handle a name on disk actually
    // wore (`CONTEXT.md`, *Work-item handle*), so `selection.handle` is what
    // goes past this point.
    if &selection.handle != finish {
        bail!(
            "requested finish handle {finish} does not match the live finish leaf {}",
            selection.handle
        );
    }

    // The exclusive guard is held across the teardown: nothing else may observe
    // `.grove/` between the revalidation above and the commit below. It is
    // dropped when this function returns — after the root it names is gone,
    // which the lock does not care about, because the lock is on the directory
    // *containing* the root.
    delete_and_commit(workspace, tree, &selection.handle)
}

/// Delete `.grove/` and commit the deletion. Two steps, and each names the
/// operation-log command that puts the tree back if it is the one that failed.
///
/// The refusal wording is grove's and the remedy is jj's: the seam has no
/// `.grove/` to speak about, and grove has no operation log to repair with.
fn delete_and_commit(workspace: &Workspace, tree: Guard, finish_handle: &Handle) -> Result<Commit> {
    let grove_root = tree.root().to_path_buf();
    let grove_root = grove_root.as_path();
    require_recoverable_grove(workspace, grove_root)?;
    // What makes a half-removed tree recoverable is that the snapshot holding it
    // is already in the operation log, not anything done here: `jj restore
    // .grove` returns what was removed — measured on jj 0.44.0
    // (`minimalism-k1`). Edits made since the last snapshot are outside that
    // log, which is a fact about a working copy jj has not seen rather than a
    // guarantee grove could hand-build, and principle 1 is that grove does not
    // hand-build it.
    //
    // **The removal is the store's.** `root-lifecycle-belongs-to-the-store`
    // moved both halves of a root's lifetime across, and
    // [`WriteGuard::delete`](ordinal_fs_tree::fs::WriteGuard::delete) is the
    // half this verb needs: it consumes the guard — so the lock is held right up
    // to the unlink and released with nothing left to guard — refuses a root
    // spelled through a link, and reports what went. A bare `remove_dir_all`
    // had none of that, and grove kept its own spelling of a store operation
    // for no reason but history.
    tree.delete().map_err(|error| {
        anyhow::anyhow!(
            "{error}\n\n\
             Jujutsu still holds the tree: restore what was removed with `jj restore .grove`, \
             then fix what blocked the deletion and rerun \
             `grove-llm finish-commit {finish_handle}`."
        )
    })?;
    let commit = workspace
        .commit(
            &[grove_root],
            &format!("{finish_handle}: remove completed grove task tree"),
        )
        .with_context(|| {
            format!(
                "the {finish_handle} teardown was not committed, and `.grove/` is deleted in \
                 the working copy. Once the tree is back, fix what made the commit fail and \
                 rerun `grove-llm finish-commit {finish_handle}`."
            )
        })?;
    Ok(commit)
}

/// The one precondition deletion has: jj can only put back what it tracks.
///
/// **This is not a surviving piece of the transaction.** A transaction promises
/// to undo its own work; this promises nothing and repairs nothing. It is the
/// gate that makes the version control system's guarantee *applicable* — an
/// untracked `.grove/` is outside the operation log, so no `jj undo` would
/// return it and the deletion below would be the unrecoverable kind. Principle
/// 2's answer to that is a message naming what is wrong and how to fix it,
/// which is what this is. One read-only probe, and nothing is written to record
/// it.
fn require_recoverable_grove(workspace: &Workspace, grove_root: &Path) -> Result<()> {
    if workspace.is_tracked(grove_root)? {
        return Ok(());
    }
    bail!(
        "Jujutsu tracks nothing under {}, so deleting the task tree could not be undone\n\n\
         Grove takes a commit; it does not implement a transaction, and the operation log \
         can only restore what it tracks. Commit or track the task tree and rerun:\n      \
         jj commit -m \"track the grove task tree\" root:.grove\n\n\
         Nothing was deleted or changed.",
        grove_root.display()
    );
}

/// `root-init [<slug>]`: scaffold a fresh grove under `worktree/.grove` — the root
/// `BRIEF.md` (the one unkeyed singleton) and a first **requirements** leaf
/// `01-requirements-<slug>-k1.md`. Returns the absolute paths created:
/// `BRIEF.md`, then the leaf. Refuses to clobber an existing `.grove/`.
/// Working-tree only — no commit.
///
/// The kind is fixed, with no `--kind` flag: a brand-new grove's first session
/// takes the human's own words as its only input — nothing else is on disk —
/// which is the generating rule for **HITL**, and `requirements` is the kind
/// that rule names (`docs/ARCHITECTURE.md#task-kind-taxonomy`). It
/// being fixed is also load-bearing for routing: the loop driver launches the
/// `start` session *before* this verb has run, so it can only route the
/// bootstrap by construction (`fresh-grove-start-contract`).
pub(crate) fn root_init(vacancy: TreeVacancy, slug: &Slug, kind: &Kind) -> Result<Vec<PathBuf>> {
    task_grow::refuse_finish_kind(kind, "root-init")?;
    let name = grove_name(vacancy.root());
    initialize_grove(vacancy, &name, slug, kind)
}

/// The slug `root-init` uses when nobody supplied one, and the only slug the
/// driver's own scaffold can use.
fn default_root_slug() -> Slug {
    Slug::new(DEFAULT_ROOT_SLUG).expect("the default root slug is a valid slug")
}

/// Create the whole grove — the root, its `BRIEF.md` and the first
/// **requirements** leaf — as one store operation under the lock the vacancy
/// already holds. Returns the charter's path, then the leaf's.
///
/// # There is no second phase, and that is the deletion
///
/// This used to be two: grove created the root and its charter under a guard of
/// its own, released it, and appended the first leaf under the library's,
/// because the two `flock` the directory containing the root through different
/// open file descriptions and do not nest. The window between them was a real
/// tree shape — a root holding its charter and no keyed entry — and grove
/// carried a recovery for it. [`TreeVacancy::initialize`] closes the window by
/// removing the seam: the charter is the root's *distinguished child*, which the
/// store writes, and a failed `initialize` takes the root back down with it. So
/// the recovery went with the anomaly it repaired (`collapse-tree-access-k13`),
/// and what a torn tree gets now is [`RootShape::Taskless`]'s message.
///
/// # The key is predicted here, exactly as every grow verb predicts one
///
/// The leaf's body embeds its own `# <slug>-k<key>` handle, so the key has to be
/// chosen before the store allocates one. `initialize` places its entries by the
/// rule `append_many` uses over an empty tree — `Ordinal::FIRST` onward with keys
/// from 1 — so the prediction is 1, and [`task_grow::allocated`] holds the store
/// to it rather than trusting it.
fn initialize_grove(
    vacancy: TreeVacancy,
    name: &str,
    slug: &Slug,
    kind: &Kind,
) -> Result<Vec<PathBuf>> {
    let key = Some(Key::new(1));
    let leaf = task_grow::new_leaf(key, Outcome::Live, kind.clone(), slug);
    let report = vacancy
        .initialize(Some(root_brief_body(name).into_bytes()), vec![leaf])
        .map_err(task_tree::raised)?;
    // The charter first, then the leaf — `Report::created` is
    // distinguished-child-first, and the two are what `root-init` reports.
    let created = report.created();
    let Some((charter, positioned)) = created.split_first() else {
        bail!("the store initialized a grove and reported nothing created");
    };
    if !matches!(charter.name, TaskName::Brief) {
        bail!(
            "the store reported {} where the grove's charter was expected",
            charter.path.display()
        );
    }
    let mut paths = vec![charter.path.clone()];
    paths.extend(task_grow::allocated(positioned, &[key])?);
    Ok(paths)
}

/// What an existing `.grove/` is, as far as the lifecycle transition is
/// concerned. Three shapes, and the classification is the whole of what the
/// transition decides now.
///
/// It used to be a byte-exact match against the deterministic fresh-tree content
/// under a missing `.grove/FORMAT`, because a witnessless root was *also* how a
/// legacy tree presented and the two got opposite treatment. Migration is gone
/// (`delete-migration-k6`), and the discrimination it needed went with it. Two
/// of the three shapes are now refusals rather than work: grove creates a grove
/// whole, so anything short of one is an anomaly to name (principle 2).
enum RootShape {
    /// A root holding no task — its charter and nothing else, or nothing at all.
    ///
    /// **Nothing grove does produces this any more, and that is the change.**
    /// `root-init` used to write the root and its charter under a guard of its
    /// own and append the first leaf under the library's, so a death between the
    /// phases left exactly this and grove carried a recovery for it.
    /// [`initialize_grove`] is one operation that takes the root back down if it
    /// fails, so the window is closed and the repair went with it
    /// (`collapse-tree-access-k13`). What reaches here is a tree something
    /// emptied by hand — and entries are marked, never removed
    /// (`docs/adr/entries-are-never-removed.md`), so it was not grove.
    Taskless,
    /// At least one keyed entry. A name the grammar *refuses* never reaches this
    /// classification at all: the store halts the whole tree on one, wherever it
    /// sits, and `task_tree::restate` says so in the domain's own words — which
    /// is the same answer as before by a shorter route, since a refused name is
    /// *held* and scaffolding past it would bury the real problem.
    ATree,
    /// Names grove disclaims, and nothing else. A `.grove/` in one of the
    /// layouts grove wrote before the current grammar reads exactly so, since
    /// none of those names are positioned-and-keyed.
    Unrecognised(Vec<String>),
}

/// Classify a root grove has already opened.
///
/// **The snapshot answers the first question and a listing answers the second**,
/// and both are read under the guard the caller holds. Whether the tree has a
/// keyed entry is the snapshot's, exactly as [`task_tree::next_key`] reads it.
/// *Which* foreign names a taskless root is holding is not: the store skips a
/// name the domain disclaims and reports nothing about it
/// (`crates/ordinal-fs-tree/src/fs/read.rs`, the parse trichotomy), so naming
/// them in a refusal means listing the directory. That is safe here in a way it
/// was not before — grove holds the store's own lock rather than a second one
/// beside it.
fn root_shape(tree: &task_tree::Guard) -> Result<RootShape> {
    if tree.snapshot().walk().any(|entry| entry.key().is_some()) {
        return Ok(RootShape::ATree);
    }
    let grove_root = tree.root();
    let listing = fs::read_dir(grove_root)
        .with_context(|| format!("reading grove root {}", grove_root.display()))?;
    let mut disclaimed = Vec::new();
    for entry in listing {
        let entry =
            entry.with_context(|| format!("reading grove root {}", grove_root.display()))?;
        let found = if entry
            .file_type()
            .with_context(|| format!("checking {}", entry.path().display()))?
            .is_dir()
        {
            Found::Dir
        } else {
            Found::File
        };
        let name = entry.file_name();
        // A name that is not UTF-8 is not one this grammar can spell, which is
        // the same answer `TaskName::parse` reaches for anything else foreign.
        let name = name.to_string_lossy().into_owned();
        if matches!(TaskName::parse(&name, found), Verdict::Foreign) {
            disclaimed.push(name);
        }
    }
    if disclaimed.is_empty() {
        return Ok(RootShape::Taskless);
    }
    disclaimed.sort();
    Ok(RootShape::Unrecognised(disclaimed))
}

/// `leaf-decompose <leaf-path> <first-child-slug>`: convert a live leaf file
/// `NN-<kind>--<slug>-k<key>.md` into a node directory `NN-<slug>-k<key>/` (**key
/// preserved**) holding a `BRIEF.md` (seeded from the leaf body, its `# <handle>`
/// header retitled `# <handle> — brief`) and a first child
/// `01-<kind>-<first-child-slug>-k<new>.md` grown atomically so the node is never
/// childless. Refuses a brief, a node, and an already-`DONE` leaf. Returns
/// `(brief_path, first_child_path)`. Working-tree only — no commit.
///
/// `kind_override` is `--kind`'s *override* of the first child's kind
/// (task-kind-taxonomy): `None` inherits the leaf being decomposed's own kind —
/// read strictly from the parent filename. `Some(k)` uses `k` regardless of the
/// parent's kind. Legacy body routing is not copied into either new task body.
///
/// # The whole verb is one `promote`, and it has to be
///
/// [`WriteGuard::promote`](ordinal_fs_tree::fs::WriteGuard::promote) creates the
/// node, moves the leaf's own file into it as the distinguished child, and
/// creates the first child — three effects in one unit, which is exactly what
/// *atomically growing a first child* has always promised and what Grove used to
/// assemble out of a `create_dir`, a rename and a whole second verb. The
/// difference is not tidiness. A promotion **breaks an invariant on the way
/// through**: the node carries the promoted leaf's own ordinal and its own key,
/// so between effect one and effect two both are on disk sharing an ordinal and
/// a key, and no ordering avoids it
/// (`docs/ordinal-fs-tree/ARCHITECTURE.md`, *Promotion is not atomic against the
/// invariants*). The invariants hold of **quiescent** trees, and the exclusive
/// guard is what makes that safe — so a Grove reader running without a guard
/// would see the intermediate state, and none does: every reader goes through
/// [`task_tree::read`] or [`task_tree::write`], and
/// `the_librarys_tree_lock_is_taken_from_exactly_one_module` holds that,
/// together with the fact that Grove's own guard `flock`s the same directory and
/// so excludes rather than nests.
///
/// # Two keys are predicted here, for two different reasons
///
/// The node's key is the leaf's own — *identity preservation* is the whole point
/// of a promotion — and the first child's is [`task_tree::next_key`], the
/// consumer-side mirror of the library's `max + 1` that a content-carrying domain
/// cannot do without (`growing-k33`; `docs/ARCHITECTURE.md#tree-access-lock`).
/// Both are checked against the report by [`promoted`], because both are embedded
/// in bytes: the child's in the template handle it is created with, and the
/// node's in the brief header this verb retitles afterwards.
pub(crate) fn leaf_decompose(
    tree: Guard,
    leaf_path: &Path,
    first_child_slug: &Slug,
    kind_override: Option<Kind>,
) -> Result<(PathBuf, PathBuf)> {
    // Grove's own precondition, before the tree is even observed, so a bad slug
    // leaves the leaf un-decomposed. It could sit inside the guard with the rest
    // of them; it stays out here because it needs nothing from the tree, and
    // refusing without taking an exclusive lock is strictly kinder.
    let child_slug = first_child_slug;

    let root = tree.root().to_path_buf();
    let grove_root = root.clone();
    // The classification, then the operation: `promote` consumes the guard, so
    // every borrow of its snapshot has to end before the call.
    let (key, slug, kind, child_key) = {
        let entry = match task_tree::target(&root, tree.snapshot(), leaf_path)? {
            task_tree::Target::Root => bail!(
                "cannot decompose the grove root (lifecycle verbs act on leaves): {}",
                root.display()
            ),
            task_tree::Target::Entry(entry) => entry,
        };
        let (parent_kind, slug) = decomposable(&entry)?;
        // task-kind-taxonomy: the first child inherits the decomposed leaf's own
        // kind unless `--kind` overrides it.
        let kind = kind_override.unwrap_or(parent_kind);
        task_grow::refuse_finish_kind(&kind, "leaf-decompose")?;
        (
            task_tree::addressable_key(&root, tree.snapshot(), &entry)?,
            slug.clone(),
            kind,
            task_tree::next_key(tree.snapshot()),
        )
    };
    // The node's parts are Grove's to supply because the library cannot make
    // them — `Parts` is opaque, and nothing the library can reach describes
    // *this* entry as a node (`docs/adr/entry-name-is-the-only-seam.md`). The
    // same slug, so the species is the only thing that moved.
    let node_parts = Parts::node(slug.clone());
    let child = task_grow::new_leaf(child_key, Outcome::Live, kind, child_slug);
    let report = tree
        .promote(key, node_parts, Some(child))
        .map_err(task_tree::raised)?;
    let (brief_path, child_path) = promoted(&report, key, child_key)?;

    // The retitling is Grove's own edit to bytes the library moved verbatim and
    // never read — it has no content model — so it cannot be part of the unit
    // above. It takes a guard of its own for the reason `leaf-insert`'s lint
    // does: the tree this touches is the one the promotion *left*, and no
    // cooperating command should meet a node brief mid-retitle.
    // `reopen_write`, not `write`: the wait this command made was announced by
    // the promotion (`docs/ARCHITECTURE.md#tree-access-lock`).
    let _guard = task_tree::reopen_write(&grove_root)?;
    append_brief_suffix_in_file(&brief_path, &Handle::new(slug.clone(), key))?;
    Ok((brief_path, child_path))
}

/// The decomposed leaf's own kind and slug, or Grove's refusal that this entry
/// is not a live leaf.
///
/// Every clause is a precondition the library cannot see — brief-ness, an
/// outcome infix, `finish`-reservation — which is clause 2 of
/// `docs/ARCHITECTURE.md#library-refusals` and the reason
/// [`Refusal::PromoteNotLeaf`](ordinal_fs_tree::Refusal) is unreachable: a node
/// falls out of the same match, before any key is handed to the library.
fn decomposable<'a>(entry: &Entry<'a, TaskName>) -> Result<(Kind, &'a Slug)> {
    let name = entry.name();
    let Some(triple) = entry.triple() else {
        bail!("cannot decompose a brief (it is already a node): {name}")
    };
    match triple.parts {
        Parts::Node { .. } => {
            bail!("cannot decompose a node (it already has children): {name}")
        }
        Parts::Leaf {
            outcome: Outcome::Done,
            ..
        } => bail!("cannot decompose a retired (DONE) leaf: {name}"),
        Parts::Leaf {
            outcome: Outcome::Abandoned,
            ..
        } => bail!("cannot decompose an abandoned (ABANDONED) leaf: {name}"),
        Parts::Leaf {
            outcome: Outcome::Live,
            kind,
            slug,
        } => {
            if kind.is_finish() {
                bail!("`finish` is driver-reserved and cannot be decomposed");
            }
            Ok((kind.clone(), slug))
        }
    }
}

/// What a promotion left behind: the node's brief, and the first child — each
/// checked against what Grove promised itself.
///
/// **Three claims, and every one of them is about bytes rather than tidiness.**
/// The node kept the promoted leaf's key, which is what identity preservation
/// *is* and what keeps the brief's own `# <slug>-k<key>` handle true of the
/// entry it now names. The child got the key Grove predicted, without which its
/// template handle would contradict its filename permanently (`growing-k33`).
/// And the leaf's file was *renamed* rather than copied, which is the only
/// reason its bytes are still there to retitle.
///
/// A disagreement is a broken contract to report and not a case to recover from:
/// the operation has already landed when this runs. The ordinal needs no clause
/// of its own — the node's name is composed from the promoted leaf's triple, so
/// a preserved key and a preserved ordinal are one fact, and the on-disk
/// assertion lives in `decompose_converts_leaf_file_to_node_dir_preserving_the_key`.
fn promoted(
    report: &Report<TaskName>,
    key: Key,
    predicted_child: Option<Key>,
) -> Result<(PathBuf, PathBuf)> {
    let created = report.created();
    let [node, child] = created else {
        bail!(
            "the library created {} entries for a promotion with a first child, \
             where 2 were asked for",
            created.len()
        )
    };
    let TaskName::Positioned { key: node_key, .. } = &node.name else {
        bail!(
            "the library promoted a leaf into the charter brief, which carries no \
             key: {}",
            node.path.display()
        )
    };
    if *node_key != key {
        bail!(
            "the library gave the promoted node key {} where the leaf carried {}: \
             the handle in {} contradicts its filename and must be corrected by hand",
            node_key.get(),
            key.get(),
            node.path.display()
        );
    }
    let TaskName::Positioned { key: child_key, .. } = &child.name else {
        bail!(
            "the library created a charter brief as a first child: {}",
            child.path.display()
        )
    };
    if Some(*child_key) != predicted_child {
        bail!(
            "the library allocated key {} where Grove's template wrote {}: the \
             handle in {} contradicts its filename and must be corrected by hand",
            child_key.get(),
            predicted_child.map_or("no key".to_string(), |key| key.get().to_string()),
            child.path.display()
        );
    }
    let brief = report
        .renamed()
        .first()
        .context("the library reported no rename for a promotion")?;
    Ok((brief.to.clone(), child.path.clone()))
}

/// `leaf-retire <leaf-path>`: rename a live leaf `NN-<kind>--<slug>-k<key>.md` →
/// `NN-DONE-<kind>--<slug>-k<key>.md` in place, keeping its position and key. The
/// `DONE` infix is filename-only — the `# <handle>` header is byte-identical.
/// Refuses a brief, a node directory, and an already-`DONE` leaf. Returns the
/// retired file's absolute path. Working-tree only — no commit.
///
/// **The mark is `ordinal_fs_tree`'s `rewrite`**, which is what a mark *is*
/// algebraically: the entry keeps its ordinal, its key and its species, and only
/// the opaque remainder of its name moves. The rename underneath is
/// `rename(2)` — plain, consulting no repository; see
/// [`docs/adr/grove-does-not-stage-its-own-renames.md`](../docs/adr/grove-does-not-stage-its-own-renames.md).
pub(crate) fn leaf_retire(tree: Guard, leaf_path: &Path) -> Result<PathBuf> {
    // The classification, then the guard: `rewrite` consumes the guard, so the
    // borrow of its snapshot has to end before the call.
    let (key, parts) = {
        let entry = match task_tree::target(tree.root(), tree.snapshot(), leaf_path)? {
            task_tree::Target::Root => bail!(
                "cannot retire the grove root (lifecycle verbs act on leaves): {}",
                tree.root().display()
            ),
            task_tree::Target::Entry(entry) => entry,
        };
        let parts = retire_parts(&entry)?;
        (
            task_tree::addressable_key(tree.root(), tree.snapshot(), &entry)?,
            parts,
        )
    };
    let report = tree.rewrite(key, parts).map_err(task_tree::raised)?;
    marked_path(&report)
}

/// The key to rewrite and the `DONE` parts to give it, or Grove's own refusal.
///
/// Every clause here is a precondition the library cannot see — an outcome
/// infix, `finish`-reservation, brief-ness — which is why classifying before
/// calling is not optional (`docs/ARCHITECTURE.md#library-refusals`, clause 2).
/// The species refusal `rewrite` would make sits behind them and is therefore
/// unreachable, exactly as that document's table says.
fn retire_parts(entry: &Entry<'_, TaskName>) -> Result<Parts> {
    let name = entry.name();
    let Some(triple) = entry.triple() else {
        bail!("cannot retire a brief (briefs are never done): {name}")
    };
    match triple.parts {
        Parts::Node { .. } => {
            bail!("cannot retire a node (nodes are never marked done): {name}")
        }
        Parts::Leaf {
            outcome: Outcome::Done,
            ..
        } => bail!("leaf is already retired (DONE): {name}"),
        Parts::Leaf {
            outcome: Outcome::Abandoned,
            ..
        } => bail!("cannot retire an abandoned (ABANDONED) leaf: {name}"),
        Parts::Leaf {
            outcome: Outcome::Live,
            kind,
            slug,
        } => {
            if kind.is_finish() {
                bail!("`finish` is driver-reserved and cannot be retired");
            }
            Ok(Parts::leaf(Outcome::Done, kind.clone(), slug.clone()))
        }
    }
}

/// The outcome of a [`leaf_prune`] call: every leaf newly marked `ABANDONED`
/// (its new path), and every already-`DONE` leaf found in scope and left
/// untouched (pruning: that work really was done). A single-leaf call
/// marks exactly one entry and finds nothing to leave alone; a node call is
/// bulk — the arity asymmetry with `leaf-retire` is deliberate (pruning).
#[derive(Debug)]
pub struct PruneResult {
    pub marked: Vec<PathBuf>,
    pub left_done: Vec<PathBuf>,
}

/// `leaf-prune <path>`: mark abandoned work `ABANDONED` in place (pruning).
/// `path` is a live leaf file **or** a node directory (absolute, or relative to
/// the grove root):
///   * given a **leaf**, marks it directly — refuses a brief, an already-`DONE`
///     leaf, and an already-`ABANDONED` leaf;
///   * given a **node**, marks every *live* leaf in its subtree (recursively),
///     leaving `DONE` leaves untouched — refuses the grove root itself
///     (abandoning a whole workstream is a branch-delete, not a tree mark).
///
/// The `ABANDONED` infix is filename-only — every marked leaf's `# <handle>`
/// header stays byte-identical. Working-tree only — no commit.
///
/// # One guard is one mark, and a subtree is many
///
/// `rewrite` consumes its write guard, so a subtree prune is *N* rewrites under
/// *N* guards where it was once one critical section. Grove accepts that rather
/// than asking the library for a batched rewrite, and
/// [`docs/adr/bulk-marks-are-not-atomic.md`](../docs/adr/bulk-marks-are-not-atomic.md)
/// records why and what an operator does with a prune that stopped half way.
/// What survives the change is the up-front validation: the whole subtree is
/// planned and every destination checked against the **first** guard's snapshot
/// before any rename happens, so the failure that test suite has always covered
/// — a botched earlier prune leaving an `ABANDONED` twin in the way — still
/// leaves the tree untouched. What is lost is only the window *between* guards:
/// another writer, or a filesystem fault, can now stop the run partway.
///
/// **HITL (pruning):** this verb does not itself gate on human
/// confirmation — constraint 5 is "grove guides, it does not gate" — so the
/// caller (the LLM driving the session) must already have explicit human
/// confirmation before calling this at all.
pub(crate) fn leaf_prune(tree: Guard, path: &Path) -> Result<PruneResult> {
    let root = tree.root().to_path_buf();
    let plan = {
        let entry = match task_tree::target(&root, tree.snapshot(), path)? {
            task_tree::Target::Root => bail!(
                "cannot prune the grove root (abandoning a whole grove is a \
                 branch-delete, not a tree mark): {}",
                root.display()
            ),
            task_tree::Target::Entry(entry) => entry,
        };
        plan_prune(&root, tree.snapshot(), &entry)?
    };
    apply_prune(&root, tree, plan)
}

/// One step of a planned prune: an entry to rewrite, or an already-`DONE` leaf
/// to report and leave alone.
enum Planned {
    ToMark { key: Key, parts: Parts },
    LeftDone { path: PathBuf },
}

/// Plan — and validate — the whole prune against one snapshot, mutating nothing.
///
/// Both halves matter. Planning first is what lets a subtree of *N* leaves be
/// checked before the first of *N* guards is spent; validating here rather than
/// leaf by leaf is what keeps the all-or-nothing promise a bulk verb makes and
/// the library, seeing one entry at a time, cannot.
fn plan_prune(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    entry: &Entry<'_, TaskName>,
) -> Result<Vec<Planned>> {
    let name = entry.name();
    let Some(triple) = entry.triple() else {
        bail!("cannot prune a brief (briefs are never marked): {name}")
    };
    let mut plan = Vec::new();
    match triple.parts {
        Parts::Node { .. } => plan_subtree(root, snapshot, entry, &mut plan)?,
        Parts::Leaf { .. } => plan.push(plan_leaf(root, snapshot, entry)?),
    }
    Ok(plan)
}

/// Every live leaf under a node, in the library's own per-level order, with each
/// already-`DONE` leaf collected untouched and each already-`ABANDONED` one
/// skipped silently — already terminal.
fn plan_subtree(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    node: &Entry<'_, TaskName>,
    plan: &mut Vec<Planned>,
) -> Result<()> {
    let Some(contents) = node.contents() else {
        return Ok(());
    };
    for child in contents.children() {
        let Some(triple) = child.triple() else {
            continue; // the node's own `BRIEF.md`
        };
        match triple.parts {
            Parts::Node { .. } => plan_subtree(root, snapshot, &child, plan)?,
            Parts::Leaf {
                outcome: Outcome::Live,
                ..
            } => plan.push(plan_leaf(root, snapshot, &child)?),
            Parts::Leaf {
                outcome: Outcome::Done,
                ..
            } => plan.push(Planned::LeftDone {
                path: task_tree::entry_path(root, child),
            }),
            Parts::Leaf {
                outcome: Outcome::Abandoned,
                ..
            } => {}
        }
    }
    Ok(())
}

/// One leaf's step, refused here if it cannot be marked at all.
///
/// Every clause is a precondition the library cannot see, and the last of them —
/// that the leaf's key addresses it and nothing else — is what makes the *by key*
/// call the mark is about mean anything at all. Checking it here rather than at
/// the rewrite is what keeps a bulk mark all-or-nothing across entries the
/// library only ever sees one at a time.
fn plan_leaf(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    entry: &Entry<'_, TaskName>,
) -> Result<Planned> {
    let name = entry.name();
    let triple = entry
        .triple()
        .with_context(|| format!("{name} carries no ordinal or key"))?;
    let Parts::Leaf {
        outcome,
        kind,
        slug,
    } = triple.parts
    else {
        bail!(
            "cannot prune a node directory as a leaf (pass the directory itself \
             to prune its subtree): {name}"
        )
    };
    match outcome {
        Outcome::Done => bail!("cannot prune a retired (DONE) leaf: {name}"),
        Outcome::Abandoned => bail!("leaf is already pruned (ABANDONED): {name}"),
        Outcome::Live => {}
    }
    if kind.is_finish() {
        bail!("`finish` is driver-reserved and cannot be pruned");
    }
    Ok(Planned::ToMark {
        key: task_tree::addressable_key(root, snapshot, entry)?,
        parts: Parts::leaf(Outcome::Abandoned, kind.clone(), slug.clone()),
    })
}

/// Apply a validated plan, one rewrite per guard.
///
/// The planning guard is spent on the first mark and every later one takes a
/// fresh guard through [`task_tree::reopen_write`] — which re-reads the tree, so
/// each rewrite plans from the state the one before it left, and prints no
/// second waiting diagnostic.
fn apply_prune(
    root: &Path,
    planning_guard: task_tree::Guard,
    plan: Vec<Planned>,
) -> Result<PruneResult> {
    let mut result = PruneResult {
        marked: Vec::new(),
        left_done: Vec::new(),
    };
    let mut held = Some(planning_guard);
    for step in plan {
        match step {
            Planned::LeftDone { path } => result.left_done.push(path),
            Planned::ToMark { key, parts } => {
                let mark = || -> Result<PathBuf> {
                    let tree = match held.take() {
                        Some(tree) => tree,
                        None => task_tree::reopen_write(root)?,
                    };
                    let report = tree.rewrite(key, parts).map_err(task_tree::raised)?;
                    marked_path(&report)
                };
                // **A stopped run names what it already marked, and how to
                // finish.** `docs/adr/bulk-marks-are-not-atomic.md` accepts that
                // a subtree prune is *N* rewrites under *N* guards, and it
                // accepts it on one argument: the marks *are* the state, so
                // re-running converges. That argument is only available to an
                // operator who can see the residue, and a bare store refusal
                // shows none of it — which is principle 2's *an error that only
                // reports detection is unfinished*, in the one place the ADR
                // most needs it not to be.
                match mark() {
                    Ok(path) => result.marked.push(path),
                    Err(error) => return Err(error.context(stopped_partway(&result))),
                }
            }
        }
    }
    Ok(result)
}

/// What a stopped `leaf-prune` has already done, and what to do about it.
fn stopped_partway(result: &PruneResult) -> String {
    if result.marked.is_empty() {
        return String::from(
            "leaf-prune marked nothing before it stopped; the subtree is as it was, and rerunning the same command is safe.",
        );
    }
    let marked = result
        .marked
        .iter()
        .map(|path| format!("  {}", path.display()))
        .collect::<Vec<_>>()
        .join("\n");
    let count = result.marked.len();
    let leaves = if count == 1 { "leaf" } else { "leaves" };
    format!(
        "leaf-prune stopped partway: {count} {leaves} were already marked ABANDONED and are still marked —\n{marked}\n\nA mark is the state and an already-abandoned leaf is skipped, so rerun the same `grove-llm leaf-prune` once the cause below is fixed and it will finish the rest."
    )
}

/// Where a mark left the entry, out of the library's own report.
///
/// A `rewrite` is exactly one rename and reports it whether or not the
/// filesystem was touched, so the first entry is the answer and an empty report
/// is a contract the library broke rather than a case to handle quietly.
fn marked_path(report: &Report<TaskName>) -> Result<PathBuf> {
    report
        .renamed()
        .first()
        .map(|renamed| renamed.to.clone())
        .context("the library reported no rename for a mark")
}

// ---------------------------------------------------------------------------
// helpers

/// The grove's name is the worktree directory's basename (user-owned-worktrees
/// — grove reads no branch, ever). Used as the root brief's `# <name> — brief`
/// title.
/// The grove's display name for its own charter: the **worktree** directory's
/// basename, read off the tree root the store is about to create.
///
/// It takes the grove root rather than the worktree because that is what a
/// [`TreeVacancy`] carries, and `<worktree>/.grove` is the only spelling grove
/// ever opens — so the parent of the root is the worktree, and asking the
/// vacancy is one fewer argument that could disagree with the lock.
fn grove_name(grove_root: &Path) -> String {
    grove_root
        .parent()
        .and_then(Path::file_name)
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// The minimal section-header scaffold for the root `BRIEF.md` — headers only,
/// no prose (the bootstrap session fills them). The root brief is the one
/// unkeyed, position-free singleton, unchanged across schemes.
///
/// Bytes rather than a write: the charter is the root's **distinguished child**
/// and the store places it, so grove no longer opens the file itself
/// (`collapse-tree-access-k13`).
fn root_brief_body(name: &str) -> String {
    format!(
        "# {name} — brief\n\n## Goal\n\n## Done when\n\n## Decomposition\n\n## Pointers\n\n## Notes\n",
    )
}

/// Retitle a freshly-decomposed node brief's first-line handle header by appending
/// ` — brief`, rewriting the file in place. Recognises exactly the canonical
/// position-free handle `# <slug>-k<key>` (the form `leaf_add_unlocked` writes), and is
/// idempotent against an already-suffixed title; any other (hand-edited) first line
/// is left alone (conservative — never clobbers a custom title).
///
/// It takes the [`Handle`] rather than a slug and a key so the line it looks for
/// is rendered by the same code that wrote it. Recognising a title by
/// re-spelling the grammar is how this and [`task_grow::task_template_body`]
/// could once have drifted into a retitle that silently matched nothing.
fn append_brief_suffix_in_file(path: &Path, handle: &Handle) -> Result<()> {
    let body =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let title = format!("# {handle}");
    let new_first = if first.trim_end() == title {
        format!("{title} — brief")
    } else {
        return Ok(()); // already suffixed, or a custom title — leave alone
    };
    let mut out = String::with_capacity(body.len() + 8);
    out.push_str(&new_first);
    if let Some(r) = rest {
        out.push('\n');
        out.push_str(r);
    }
    fs::write(path, out.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A [`Kind`] for a test that needs one, by its label.
    ///
    /// A kind is an **open token** since `open-kind-k20`, so a test names the token
    /// it means rather than a variant, and an invalid one is a test bug that panics
    /// here rather than a compile error somewhere else.
    fn a_kind(label: &str) -> Kind {
        Kind::new(label).expect("a test kind must be well-formed")
    }
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    /// A bare worktree dir with **no** `.grove/` yet — for `root_init`, which
    /// creates the grove itself and needs no repository (it never renames an entry).
    fn worktree() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let wt = tmp.path().join("my-grove");
        fs::create_dir_all(&wt).unwrap();
        (tmp, wt)
    }

    /// A `.grove/` inside a real jj repo — the only kind of working tree Grove
    /// drives (`docs/adr/jj-is-the-only-lane.md`). The repository is these tests'
    /// **instrument** rather than their prerequisite: every verb below renames
    /// inside an `ordinal-fs-tree` operation, which uses `rename(2)` and records
    /// nothing of its own (`docs/adr/grove-does-not-stage-its-own-renames.md`),
    /// so nothing here needs committed files to operate on. [`commit_all`] is
    /// what makes the fixtures the ones a real session produces.
    fn jj_grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path().to_path_buf();
        run_jj(
            &repo,
            &["--config", "git.colocate=false", "git", "init", "."],
        );
        let root = repo.join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
    }

    fn run_jj(repo: &Path, args: &[&str]) {
        let out = Command::new("jj")
            .current_dir(repo)
            .args([
                "--config",
                "user.name=Test",
                "--config",
                "user.email=t@example.com",
            ])
            .args(args)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "jj {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Commit everything under the grove, putting the entries in the revision the
    /// working copy sits on — the state a real session's tree is in, and the one
    /// in which a rename that recorded anything of its own would be visible.
    fn commit_all(root: &Path) {
        run_jj(root.parent().unwrap(), &["commit", "-m", "fixture"]);
    }

    /// Grow a real root-level leaf the way `llm_cli` does — the whole verb,
    /// which takes and releases the library's own write guard, so the lifecycle
    /// verb under test then takes its.
    ///
    /// It calls the verb rather than a lock-neutral primitive under a guard of
    /// its own, and since `sweep-k37` there is no other option: the primitive
    /// belonged to the withdrawn appender, and grove cannot nest its own
    /// exclusive guard inside the library's anyway. `task_grow` owns
    /// the resolve-then-mutate composition entirely, which is the composition
    /// production performs and the only one it can.
    fn grow_leaf(root: &Path, slug: &str) -> PathBuf {
        task_grow::leaf_add(guard(root), ".", &a_slug(slug), &[a_kind("impl")])
            .unwrap()
            .remove(0)
    }

    /// A [`Slug`] for a test that needs one.
    fn a_slug(text: &str) -> Slug {
        Slug::new(text).expect("a test slug must be well-formed")
    }

    /// The exclusive guard a lifecycle write verb now takes, opened from a grove
    /// root — the tests still name roots, and the verbs no longer open one.
    fn guard(grove_root: &Path) -> Guard {
        open(grove_root).expect("opening the tree for a lifecycle verb")
    }

    /// The same, as a `Result`, for the fixtures whose subject *is* the opening.
    fn open(grove_root: &Path) -> Result<Guard> {
        task_tree::write(grove_root)
    }

    /// The refusal an absent root gets, now that the caller does the opening.
    ///
    /// It used to be each write verb's, because each opened the tree itself.
    /// Since `loop-crate-verbs-k21` the lock is in the signature and the caller
    /// holds it, so this is the one place the wording is checked — and the verbs
    /// cannot be reached at all over a root with no tree, because
    /// [`crate::write`] answers a vacancy there and a vacancy offers only
    /// `root-init`.
    fn refusal_for_an_absent_root(grove_root: &Path) -> String {
        open(grove_root)
            .err()
            .expect("an absent root has no tree to open")
            .to_string()
    }

    /// The same, from a **worktree** — the spelling the driver-facing operations
    /// take, and the one `<worktree>/.grove` is joined in.
    fn guard_at(worktree: &Path) -> Guard {
        guard(&worktree.join(".grove"))
    }

    /// `root-init` as its own CLI drives it: read the slug, resolve the vacancy,
    /// then scaffold — **in that order**, which is what the tests below about a
    /// refused slug are actually asserting.
    ///
    /// The verb takes the [`TreeVacancy`] since `loop-crate-verbs-k21`, which is
    /// what makes *cannot clobber a live grove* a fact about the types rather
    /// than a check — so a fixture that wants that refusal asks for the opening
    /// and finds a tree, exactly as the CLI does. The slug is read first, and
    /// before the lock, since refusing without taking an exclusive one is
    /// strictly kinder.
    fn root_init_at(worktree: &Path, slug: &str) -> Result<Vec<PathBuf>> {
        let slug = Slug::new(slug).map_err(|error| anyhow::anyhow!("slug {slug:?}: {error}"))?;
        let grove_root = worktree.join(".grove");
        match task_tree::write_or_vacancy(&grove_root)? {
            Opening::Tree(_) => {
                bail!("grove root already exists: {}", grove_root.display())
            }
            Opening::Vacancy(vacancy) => root_init(vacancy, &slug, &Kind::requirements()),
        }
    }

    /// Write a leaf/brief stub with a position-free `# <handle>` header.
    fn touch(dir: &Path, name: &str, header: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, format!("# {header}\n")).unwrap();
        p
    }

    /// Write a file with an explicit body (for realistic multi-line content).
    fn touch_body(dir: &Path, name: &str, content: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    /// Create a node directory with its `BRIEF.md`, returning the directory path.
    fn mknode(dir: &Path, name: &str, handle: &str) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(&p).unwrap();
        fs::write(p.join("BRIEF.md"), format!("# {handle} — brief\n")).unwrap();
        p
    }

    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    fn body(p: &Path) -> String {
        fs::read_to_string(p).unwrap()
    }

    /// The directory's child names (files and subdirs), lexically sorted.
    fn list(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        v.sort();
        v
    }

    // ---- root-init ----------------------------------------------------------

    #[test]
    fn root_init_creates_root_brief_then_first_leaf() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "plan").unwrap();
        assert_eq!(name_of(&created[0]), "BRIEF.md");
        assert_eq!(name_of(&created[1]), "01-requirements--plan-k1.md");
        assert_eq!(created.len(), 2);
        let g = wt.join(".grove");
        assert!(g.join("BRIEF.md").is_file());
        assert!(g.join("01-requirements--plan-k1.md").is_file());
    }

    // fresh-grove-start-contract: the bootstrap leaf is `requirements` — the
    // human's own words are the session's only input, which is the HITL rule —
    // and the `start` path routes on that kind without a file to peek, so this
    // assertion is the contract, not a detail of the template.
    #[test]
    fn root_init_first_leaf_kind_lives_in_its_filename() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "plan").unwrap();
        assert_eq!(name_of(&created[1]), "01-requirements--plan-k1.md");
        assert!(!body(&created[1]).contains("**Kind:**"));
    }

    #[test]
    fn root_init_first_leaf_header_is_the_position_free_handle() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "plan").unwrap();
        assert!(
            body(&created[1]).starts_with("# plan-k1\n"),
            "got {:?}",
            body(&created[1])
        );
    }

    #[test]
    fn root_init_root_brief_title_is_the_grove_name() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "plan").unwrap();
        assert_eq!(
            body(&created[0]).lines().next().unwrap(),
            "# my-grove — brief"
        );
    }

    #[test]
    fn root_init_refuses_an_existing_grove() {
        let (_t, wt) = worktree();
        fs::create_dir_all(wt.join(".grove")).unwrap();
        let err = root_init_at(&wt, "plan").unwrap_err();
        assert!(err.to_string().contains("already exists"), "got {err}");
    }

    #[test]
    fn root_init_rejects_a_bad_slug_without_leaving_a_grove_behind() {
        let (_t, wt) = worktree();
        assert!(root_init_at(&wt, "Bad Slug").is_err());
        assert!(
            !wt.join(".grove").exists(),
            ".grove must not be created on a bad slug"
        );
    }

    #[test]
    fn root_init_rejects_reserved_slug() {
        let (_t, wt) = worktree();
        assert!(root_init_at(&wt, "BRIEF").is_err());
        assert!(!wt.join(".grove").exists());
    }

    #[test]
    fn transition_initializes_an_absent_grove_under_one_exclusive_guard() {
        let (_temporary, worktree) = worktree();
        crate::task_tree::reset_read_count();

        let outcome = transition_to_current(&worktree).unwrap();

        let grove_root = worktree.join(".grove");
        assert_eq!(outcome, CurrentTransition::RootInitialized);
        // **One, and it used to be two.** Classifying the root and creating it
        // were separate acquisitions because grove's guard and the library's
        // could not nest; the vacancy is one opening that answers both.
        assert_eq!(crate::task_tree::read_count(), 1);
        assert_eq!(
            name_of(&crate::task_tree::tests::pick(&grove_root).unwrap().unwrap()),
            "01-requirements--plan-k1.md"
        );
    }

    /// **The whole grove is one store operation, and that is the deletion.**
    /// `root-init` used to take a guard of grove's own for the root and its
    /// charter — neither of which the library could create, since it has to
    /// reach the root in order to snapshot it — release it, and append the first
    /// leaf under the library's, because the two `flock` one directory through
    /// different open file descriptions and would deadlock nested. The store's
    /// vacancy creates all three under the lock it already holds, so the count
    /// this asserts is the second lock layer's absence.
    #[test]
    fn root_init_creates_the_whole_grove_through_one_store_operation() {
        let (_t, wt) = worktree();
        crate::task_tree::reset_read_count();

        let created = root_init_at(&wt, "plan").unwrap();

        assert_eq!(
            crate::task_tree::read_count(),
            1,
            "the root, its charter and the first leaf are created under exactly one lock"
        );
        assert_eq!(name_of(&created[0]), "BRIEF.md");
        assert_eq!(name_of(&created[1]), "01-requirements--plan-k1.md");
    }

    /// **Grove no longer waits on itself.** The failure mode the second lock
    /// layer existed to paper over is a single process taking two `flock`s on
    /// one directory through two open file descriptions, which blocks forever —
    /// so the direct assertion is that one process runs the tree-creating verb
    /// and a tree-reading one back to back and finishes. A regression that put a
    /// guard of grove's own back around either would hang here rather than fail,
    /// so the wait is bounded and the timeout *is* the failure.
    #[test]
    fn one_process_creating_and_reading_a_grove_never_waits_on_itself() {
        let (temporary, worktree) = worktree();
        let (sender, receiver) = std::sync::mpsc::channel();
        // Detached deliberately: a thread deadlocked on `flock` cannot be
        // joined, and the point of the timeout is to report that rather than
        // inherit the hang.
        std::thread::spawn(move || {
            let created = root_init_at(&worktree, "plan").unwrap();
            let grove_root = worktree.join(".grove");
            let picked = crate::task_tree::tests::pick(&grove_root).unwrap().unwrap();
            let again = transition_to_current(&worktree).unwrap();
            sender.send((created, picked, again)).unwrap();
        });

        let (created, picked, again) = receiver
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect(
                "grove waited on a lock of its own: root-init and pick deadlocked in one process",
            );

        assert_eq!(name_of(&created[1]), "01-requirements--plan-k1.md");
        assert_eq!(picked, created[1]);
        assert_eq!(again, CurrentTransition::AlreadyCurrent);
        drop(temporary);
    }

    /// The leaf's `# <slug>-k<key>` handle is rendered before the library
    /// allocates the key, so `root-init` predicts the allocation exactly as every
    /// grow verb does — and `task_grow::allocated` refuses to report success on a
    /// disagreement. This is the assertion that would fail if the prediction and
    /// the library's `max + 1` ever parted.
    #[test]
    fn root_inits_first_leaf_handle_matches_the_key_the_library_allocated() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "custom-plan").unwrap();
        assert_eq!(name_of(&created[1]), "01-requirements--custom-plan-k1.md");
        assert!(body(&created[1]).starts_with("# custom-plan-k1\n"));
    }

    /// **A grove that fails to initialize leaves no root at all**, which is what
    /// closed the window the deleted recovery existed for. The store creates the
    /// root, places the charter and the first leaf, and takes the root back down
    /// if any of it fails — so the partial shape `root-init` used to leave
    /// between its two phases is not one grove can produce any more.
    #[test]
    fn a_refused_grove_leaves_no_root_behind() {
        let (_t, wt) = worktree();

        // A slug the grammar refuses, checked before the lock is taken.
        assert!(root_init_at(&wt, "Bad Slug").is_err());

        assert!(
            !wt.join(".grove").exists(),
            "a refused root-init must leave no root"
        );
    }

    /// **A root holding its charter and no task is refused, not repaired.**
    /// Grove used to complete it, because grove itself produced it; it does not
    /// any more (`collapse-tree-access-k13`), so what is left is an anomaly, and
    /// principle 2 says an anomaly gets a sentence naming the fix.
    #[test]
    fn a_taskless_root_is_refused_with_advice_rather_than_completed() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");

        let error = transition_to_current(&wt).unwrap_err().to_string();

        assert!(error.contains("holds no task"), "{error}");
        assert!(
            error.contains("jj undo"),
            "the refusal must name the fix: {error}"
        );
        assert_eq!(
            fs::read_dir(&grove_root).unwrap().count(),
            1,
            "a refusal repairs nothing"
        );
    }

    /// The driver's sentinel embeds its own key in its handle, its body and the
    /// `finish-commit` command it tells the session to run — so a key the library
    /// allocated differently would leave a leaf instructing an operator to commit
    /// a handle that does not exist. The prediction is checked against the report
    /// by `task_grow::allocated`; this pins the three spellings agreeing.
    #[test]
    fn materialize_finish_writes_a_handle_that_matches_its_own_filename() {
        let (_t, wt) = worktree();
        root_init_at(&wt, "plan").unwrap();
        let grove_root = wt.join(".grove");
        leaf_retire(
            guard(&grove_root),
            &grove_root.join("01-requirements--plan-k1.md"),
        )
        .unwrap();

        let selection = materialize_finish(guard_at(&wt)).unwrap();

        assert_eq!(name_of(&selection.path), "02-finish--finish-k2.md");
        assert_eq!(selection.handle.to_string(), "finish-k2");
        assert_eq!(selection.kind, a_kind("finish"));
        let body = body(&selection.path);
        assert!(body.starts_with("# finish-k2\n"), "got {body:?}");
        assert!(
            body.contains("grove-llm finish-commit finish-k2"),
            "got {body:?}"
        );
    }

    /// **The two refusals this leaf's own table row predicts, transcribed.**
    /// `materialize-finish` is an `append` at the root level, so it reaches
    /// exactly what `leaf-add` reaches — and from no argument at all, the verb
    /// taking none. Grove predicts `None` for an exhausted keyspace, hands the
    /// library no bytes, and lets it state the condition (clause 3).
    #[test]
    fn a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        touch(
            &grove_root,
            "01-DONE-impl--old-k4294967295.md",
            "old-k4294967295",
        );

        let error = materialize_finish(guard_at(&wt)).unwrap_err().to_string();

        assert!(error.contains("greatest a key can be"), "got {error}");
        assert_eq!(
            fs::read_dir(&grove_root).unwrap().count(),
            2,
            "a refusal writes nothing"
        );
    }

    #[test]
    fn a_root_level_at_the_last_ordinal_refuses_the_sentinel_rather_than_wrapping() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        touch(&grove_root, "4294967295-DONE-impl--last-k1.md", "last-k1");

        assert!(materialize_finish(guard_at(&wt)).is_err());

        assert_eq!(
            fs::read_dir(&grove_root).unwrap().count(),
            2,
            "nothing was created"
        );
    }

    /// The re-selection and the allocation read one snapshot under one exclusive
    /// guard, so nothing can appear between finding no live work and creating the
    /// sentinel — and an existing sentinel is returned rather than duplicated,
    /// which is what makes the driver's finish resumable.
    #[test]
    fn materialize_finish_reuses_an_existing_sentinel_under_one_guard() {
        let (_t, wt) = worktree();
        root_init_at(&wt, "plan").unwrap();
        let grove_root = wt.join(".grove");
        leaf_retire(
            guard(&grove_root),
            &grove_root.join("01-requirements--plan-k1.md"),
        )
        .unwrap();
        let first = materialize_finish(guard_at(&wt)).unwrap();
        crate::task_tree::reset_read_count();

        let second = materialize_finish(guard_at(&wt)).unwrap();

        assert_eq!(first.path, second.path);
        assert_eq!(crate::task_tree::read_count(), 1);
    }

    #[test]
    fn transition_leaves_a_current_grove_unchanged_and_ready_for_pick() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        let leaf = touch(&grove_root, "01-impl--task-k1.md", "task-k1");
        crate::task_tree::reset_read_count();

        let outcome = transition_to_current(&worktree).unwrap();

        assert_eq!(outcome, CurrentTransition::AlreadyCurrent);
        assert_eq!(crate::task_tree::read_count(), 1);
        assert_eq!(
            crate::task_tree::tests::pick(&grove_root).unwrap(),
            Some(leaf)
        );
    }

    /// **A tree grove cannot read is not a tree grove scaffolds over.** A
    /// malformed name is an entry — held badly — and appending a first leaf
    /// beside it would hand the operator two problems.
    ///
    /// **What moved is where the operator hears about it.** The transition used
    /// to answer *already current* and leave the next read to state the real
    /// problem, because grove classified the root by hand off its own listing.
    /// The store halts the whole tree on a name the domain refuses, so the
    /// transition itself now carries the domain's own words — one refusal
    /// instead of a success followed by one (principle 2: a message, not
    /// machinery).
    #[test]
    fn transition_does_not_scaffold_over_a_name_grove_refuses() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        touch(&grove_root, "01-task-k1.md", "task-k1");

        let error = transition_to_current(&worktree).unwrap_err();

        assert!(
            format!("{error:#}").contains("01-task-k1.md"),
            "the refusal must name the file on disk: {error:#}"
        );
        assert!(
            !grove_root.join("01-requirements--plan-k1.md").exists(),
            "a refused name must not be scaffolded past"
        );
    }

    /// **A tree grove cannot spell at all stops with a sentence.** The layouts
    /// grove wrote before the current grammar are positioned but unkeyed, so
    /// every one of their names is `Foreign` — invisible to the reader rather
    /// than refused by it. Left at that, an old tree would read as an empty
    /// grove and the driver would materialize a finish sentinel into it. So the
    /// listing having no Grove entry at all is itself the anomaly, and it is
    /// named rather than repaired (principle 2; migration is gone).
    #[test]
    fn transition_refuses_a_root_holding_no_grove_entry_at_all() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "030-ship.md", "030-ship");
        fs::create_dir(grove_root.join("020-spec")).unwrap();
        let before = list(&grove_root);

        let error = transition_to_current(&worktree).unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("holds no Grove entries"), "{message}");
        assert!(message.contains("020-spec"), "{message}");
        assert!(message.contains("030-ship.md"), "{message}");
        assert!(
            message.contains("NN-<kind>--<slug>-k<key>"),
            "the refusal must say what a name should look like: {message}"
        );
        assert_eq!(list(&grove_root), before, "a refusal writes nothing");
    }

    #[cfg(unix)]
    #[test]
    fn transition_does_not_classify_a_dangling_grove_symlink_as_absent() {
        use std::os::unix::fs::symlink;

        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        symlink(worktree.join("missing-grove"), &grove_root).unwrap();

        let error = transition_to_current(&worktree).unwrap_err();

        // The store's own sentence, not grove's *not found*: something is at
        // the root, and `is_dir` reading a dangling link as absent is exactly
        // the mistake `task_tree::restate` orders its clauses to avoid.
        assert!(
            format!("{error:#}").contains("a tree is a directory"),
            "unexpected error: {error:#}"
        );
        assert!(fs::symlink_metadata(grove_root)
            .unwrap()
            .file_type()
            .is_symlink());
    }

    // ---- leaf-decompose -----------------------------------------------------

    #[test]
    fn decompose_converts_leaf_file_to_node_dir_preserving_the_key() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let (brief, _child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();
        // The entity that was leaf k3 becomes node k3 — a directory holding BRIEF.md.
        assert_eq!(name_of(&brief), "BRIEF.md");
        assert_eq!(name_of(brief.parent().unwrap()), "02-build-k3");
        let files = list(&g);
        assert!(
            files.contains(&"02-build-k3".to_string()),
            "node dir present"
        );
        assert!(
            !files.contains(&"02-impl--build-k3.md".to_string()),
            "old leaf file gone"
        );
        assert!(g.join("02-build-k3").is_dir());
    }

    #[test]
    fn decompose_seeds_brief_from_leaf_body_and_appends_brief_suffix() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-impl--build-k3.md",
            "# build-k3\n\n## Goal\nship it\n",
        );
        commit_all(&g);
        let (brief, _child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();
        let text = body(&brief);
        assert_eq!(
            text.lines().next().unwrap(),
            "# build-k3 — brief",
            "handle retitled with the brief suffix"
        );
        assert!(
            text.contains("## Goal\nship it"),
            "leaf body carried in: {text:?}"
        );
    }

    #[test]
    fn decompose_creates_the_first_child_at_01_with_a_fresh_key() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-impl--step-k4.md");
        assert_eq!(name_of(child.parent().unwrap()), "02-build-k3");
        assert!(g.join("02-build-k3").join("01-impl--step-k4.md").is_file());
    }

    #[test]
    fn decompose_first_child_header_is_the_handle_and_filename_carries_the_kind() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();
        let text = body(&child);
        assert!(text.starts_with("# step-k4\n"), "got {text:?}");
        assert_eq!(name_of(&child), "01-impl--step-k4.md");
        assert!(!text.contains("**Kind:**"), "got {text:?}");
    }

    #[test]
    fn decompose_first_child_can_be_a_planning_task() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("design"),
            Some(a_kind("planning")),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-planning--design-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

    #[test]
    fn decompose_with_no_override_inherits_the_parent_leafs_own_kind() {
        // task-kind-taxonomy: `leaf-decompose` gives the first child the leaf
        // being decomposed's own kind when `--kind` is not given.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-research-a--build-k3.md",
            "# build-k3\n\n**Kind:** impl\n",
        );
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-research-a--build-k3.md"),
            &a_slug("step"),
            None,
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-research-a--step-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

    #[test]
    fn decompose_override_wins_over_the_parent_leafs_kind() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-research-a--build-k3.md",
            "# build-k3\n\n**Kind:** impl\n",
        );
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-research-a--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("review-impl")),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-review-impl--step-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

    #[test]
    fn decompose_a_nested_leaf_preserves_key_and_grows_a_grandchild() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let build = mknode(&g, "02-build-k1", "build-k1");
        touch(&build, "02-impl--mid-k5.md", "mid-k5");
        commit_all(&g);
        let (brief, child) = leaf_decompose(
            guard(&g),
            &build.join("02-impl--mid-k5.md"),
            &a_slug("first"),
            Some(a_kind("impl")),
        )
        .unwrap();
        assert_eq!(
            name_of(brief.parent().unwrap()),
            "02-mid-k5",
            "key 5 preserved"
        );
        assert_eq!(
            name_of(&child),
            "01-impl--first-k6.md",
            "fresh key max(1,5)+1 = 6"
        );
        assert_eq!(name_of(child.parent().unwrap()), "02-mid-k5");
    }

    #[test]
    fn decompose_refuses_a_brief() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_decompose(
            guard(&g),
            &node.join("BRIEF.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_node_directory() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_decompose(guard(&g), &node, &a_slug("x"), Some(a_kind("impl"))).unwrap_err();
        assert!(err.to_string().contains("node"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_done_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-DONE-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let err = leaf_decompose(
            guard(&g),
            Path::new("02-DONE-impl--build-k3.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();
        assert!(
            err.to_string().to_lowercase().contains("done") || err.to_string().contains("retired"),
            "got {err}"
        );
    }

    #[test]
    fn decompose_refuses_an_abandoned_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-ABANDONED-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let err = leaf_decompose(
            guard(&g),
            Path::new("02-ABANDONED-impl--build-k3.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();
        assert!(err.to_string().contains("abandoned"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_foreign_file() {
        let (_t, g) = jj_grove();
        touch(&g, "README.md", "readme");
        commit_all(&g);
        let err = leaf_decompose(
            guard(&g),
            Path::new("README.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

    /// A bad child slug leaves the leaf un-decomposed — and now it cannot even
    /// be spelled.
    ///
    /// The claim used to be about ordering: the slug was validated before the
    /// rename, so a bad one left no half-built node directory. Since
    /// `loop-crate-verbs-k21` the verb takes a [`Slug`], so the text is read by
    /// the type that owns it and a bad slug never reaches a tree at all —
    /// ordering it correctly is no longer something this verb can get wrong.
    #[test]
    fn decompose_cannot_be_reached_with_a_bad_child_slug() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);

        assert!(Slug::new("Bad Slug").is_err());

        let files = list(&g);
        assert!(
            files.contains(&"02-impl--build-k3.md".to_string()),
            "leaf untouched"
        );
        assert!(
            !files.contains(&"02-build-k3".to_string()),
            "no half-built node dir"
        );
    }

    #[test]
    fn decompose_accepts_an_absolute_path() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let abs = g.join("02-impl--build-k3.md");
        let (brief, _child) =
            leaf_decompose(guard(&g), &abs, &a_slug("step"), Some(a_kind("impl"))).unwrap();
        assert_eq!(name_of(brief.parent().unwrap()), "02-build-k3");
    }

    #[test]
    fn an_absent_grove_root_is_refused_by_the_opening() {
        let (_t, g) = jj_grove();
        let err = refusal_for_an_absent_root(&g.join("nope"));
        assert!(err.contains("grove root not found"), "got {err}");
    }

    // ---- leaf-decompose: the seam --------------------------------------------
    //
    // `promote` is the operation with the most that can go wrong, and the three
    // refusals it owns are all discharged *upstream* by Grove's own
    // classification. That makes reachability the question worth asserting, and
    // the node brief's own table the thing to check rather than transcribe
    // (`docs/ARCHITECTURE.md#library-refusals`).

    #[test]
    fn decompose_takes_one_guard_for_the_promotion_and_one_for_the_retitle() {
        // Two observations, deliberately. `promote` consumes its guard, and the
        // ` — brief` retitle is Grove's own edit to bytes the library moved
        // verbatim and never read — so it cannot ride inside the unit, and it
        // takes a guard of its own rather than running on an unheld tree.
        // Asserted as a number so a later change moves it rather than quietly
        // contradicting the paragraph, exactly as `leaf-insert`'s lint is.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        crate::task_tree::reset_read_count();

        leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();

        assert_eq!(
            crate::task_tree::read_count(),
            2,
            "one guard for the promotion, one for the retitle it cannot contain"
        );
    }

    #[test]
    fn decomposing_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed() {
        // `marking-k32`'s finding, at the verb that most needs it: `promote` is
        // called **by key**, and `by_key` answers with whichever entry the walk
        // reaches first on a duplicate-key tree. Decomposing the live leaf could
        // otherwise promote its `DONE` twin.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl--a-k1.md", "a-k1");
        touch(&g, "01-DONE-impl--a-k1.md", "a-k1");
        commit_all(&g);

        let err = leaf_decompose(
            guard(&g),
            Path::new("01-impl--a-k1.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 1"),
            "got {err}"
        );
        assert!(
            g.join("01-impl--a-k1.md").is_file() && !g.join("01-a-k1").exists(),
            "a refused promotion creates nothing"
        );
    }

    #[test]
    fn destination_occupied_is_unreachable_because_the_occupant_duplicates_the_key() {
        // **The row this leaf was sent to check.** The node a promotion composes
        // is `compose(ordinal, key, node parts)` with the *leaf's own* ordinal
        // and key, so the only name that can already occupy the destination is a
        // node carrying that key — which makes the key duplicated tree-wide, and
        // `addressable_key` refuses before any operation is planned. The same
        // argument `marking-k32` made for the marking verbs, reaching the last
        // row the table still predicted. Both shapes of occupant are checked:
        // the node with a brief (an ordinary hand edit) and the node without one
        // (an interrupted promotion), because they take different branches and
        // only the second is a state the library can leave behind.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);

        let err = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 3"),
            "the duplicate key is what is wrong with this tree, and the taken \
             destination is a consequence: {err}"
        );
        assert!(
            !err.to_string().contains("already taken"),
            "`Refusal::DestinationOccupied` must not reach an operator: {err}"
        );
        // The control, because a reachability claim that cannot fail is worth
        // nothing: with Grove's check removed, this same tree does reach the
        // library — and which refusal it reaches is not even determined, since
        // `by_key` on a duplicate key answers with whichever entry the walk
        // reaches first and walk order on such a tree is one of `structure.als`'s
        // recorded misses. Either wording is the library's, about a state Grove
        // can describe better.
        assert!(
            library_promotion_refusal(&g, 3, "build").contains("already taken")
                || library_promotion_refusal(&g, 3, "build").contains("promotion turns a"),
            "the refusal has to be there for Grove's check to be what hides it: {}",
            library_promotion_refusal(&g, 3, "build")
        );
    }

    /// Call the library's `promote` directly, bypassing every precondition Grove
    /// puts in front of it, and return the message it answers with.
    ///
    /// The positive control for the two reachability claims above. Grove has no
    /// production path that does this — clause 1 resolves an argument to an entry
    /// and clause 2 classifies it first — which is exactly why the claims need an
    /// instrument that does.
    fn library_promotion_refusal(grove_root: &Path, key: u32, slug: &str) -> String {
        let tree = crate::task_tree::write(grove_root).unwrap();
        match tree.promote(
            ordinal_fs_tree::Key::new(key),
            Parts::node(Slug::new(slug).unwrap()),
            None,
        ) {
            Ok(_) => panic!("the library must refuse this tree"),
            Err(error) => error.to_string(),
        }
    }

    #[test]
    fn an_interrupted_promotion_is_diagnosed_as_one_rather_than_as_a_hand_edit() {
        // The tree `Error::FailedPartiallyRolledBack` warns about, met by a
        // *later* command — which is the only way it is ever met, since the run
        // that caused it already reported it and exited. The library reports
        // nothing here: a duplicate key is an obligation on the domain and no
        // operation checks it. So the recovery advice is Grove's to give, and it
        // is the library's own — remove either half — and not
        // `addressable_key`'s general *give one a fresh key*, which would make
        // two entities out of one caught mid-shape-change.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        fs::create_dir(g.join("02-build-k3")).unwrap();
        commit_all(&g);

        let err = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap_err();

        let message = err.to_string();
        assert!(
            message.contains("interrupted `leaf-decompose`"),
            "got {message}"
        );
        assert!(
            message.contains("holds no BRIEF.md") && message.contains("Removing either half"),
            "the recovery has to be the library's own: {message}"
        );
        assert!(
            !message.contains("fresh key"),
            "the general duplicate-key advice is wrong for this tree: {message}"
        );
        assert!(
            g.join("02-impl--build-k3.md").is_file(),
            "a refused promotion creates nothing and repairs nothing"
        );
    }

    #[test]
    fn a_tree_at_the_last_key_refuses_the_promotion_rather_than_wrapping() {
        // The one refusal `leaf-decompose` really can reach, and it comes from
        // the **first child** rather than from the node: a promotion allocates
        // no key for the node — the entity is unchanged — so the only `max + 1`
        // in the operation is the child's. Grove predicts `None`, hands the
        // library no bytes, and lets it state the condition (clause 3).
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k4294967295.md", "build-k4294967295");
        commit_all(&g);

        let err = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k4294967295.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap_err();

        assert!(
            err.to_string().contains("greatest a key can be"),
            "got {err}"
        );
        assert!(
            g.join("02-impl--build-k4294967295.md").is_file()
                && !g.join("02-build-k4294967295").exists(),
            "a refusal writes nothing"
        );
    }

    #[test]
    fn no_promotion_refusal_reaches_an_operator_from_an_ordinary_argument() {
        // The three refusals `promote` owns, asserted unreachable rather than
        // described. `NoDistinguishedChild` is discharged by the domain itself
        // and needs no fixture; the other two are discharged by every argument
        // that is not a live leaf, and the sweep is what makes that a claim
        // about the *verb* rather than about the cases someone thought of.
        assert!(
            <TaskName as ordinal_fs_tree::EntryName>::distinguished().is_some(),
            "Grove's distinguished child is BRIEF.md, so a promotion always has \
             somewhere to put the leaf's bytes"
        );
        assert_eq!(
            Parts::node(crate::task_name::Slug::new("build").unwrap()).species(),
            ordinal_fs_tree::PositionedSpecies::Node,
            "`leaf-decompose` composes node parts and nothing else, so \
             `PromotePartsNotNode` cannot fire"
        );

        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-DONE-impl--done-k1.md", "done-k1");
        touch(&g, "02-ABANDONED-impl--gone-k2.md", "gone-k2");
        touch(&g, "03-finish--wrap-k3.md", "wrap-k3");
        let node = mknode(&g, "04-build-k4", "build-k4");
        commit_all(&g);

        for argument in [
            g.as_path(),
            &g.join("BRIEF.md"),
            &g.join("01-DONE-impl--done-k1.md"),
            &g.join("02-ABANDONED-impl--gone-k2.md"),
            &g.join("03-finish--wrap-k3.md"),
            node.as_path(),
            &node.join("BRIEF.md"),
        ] {
            let err = leaf_decompose(guard(&g), argument, &a_slug("step"), Some(a_kind("impl")))
                .unwrap_err()
                .to_string();
            for library_wording in [
                "promotion turns a",
                "this domain has no distinguished child",
                "make a leaf, not a node",
            ] {
                assert!(
                    !err.contains(library_wording),
                    "{argument:?} reached a `promote` refusal: {err}"
                );
            }
        }
    }

    // ---- leaf-retire --------------------------------------------------------

    #[test]
    fn retire_adds_done_infix_keeping_position_and_key() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let done = leaf_retire(guard(&g), Path::new("02-impl--add-k4.md")).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl--add-k4.md");
        let files = list(&g);
        assert!(files.contains(&"02-DONE-impl--add-k4.md".to_string()));
        assert!(
            !files.contains(&"02-impl--add-k4.md".to_string()),
            "old name gone"
        );
    }

    #[test]
    fn retire_does_not_rewrite_the_header_or_body() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-impl--add-k4.md", "# add-k4\n\nbody\n");
        commit_all(&g);
        let done = leaf_retire(guard(&g), Path::new("02-impl--add-k4.md")).unwrap();
        assert_eq!(body(&done), "# add-k4\n\nbody\n", "content byte-identical");
    }

    #[test]
    fn retire_works_on_a_nested_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let done = leaf_retire(guard(&g), &design.join("02-impl--add-k4.md")).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl--add-k4.md");
        assert_eq!(name_of(done.parent().unwrap()), "01-design-k1");
    }

    #[test]
    fn retire_refuses_a_node_directory() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_retire(guard(&g), &node).unwrap_err();
        assert!(err.to_string().contains("node"), "got {err}");
    }

    #[test]
    fn retire_refuses_a_node_brief() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_retire(guard(&g), &node.join("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn retire_refuses_the_root_brief() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        commit_all(&g);
        let err = leaf_retire(guard(&g), Path::new("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn retire_refuses_an_already_done_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "02-DONE-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let err = leaf_retire(guard(&g), Path::new("02-DONE-impl--add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("already"), "got {err}");
    }

    #[test]
    fn retire_refuses_an_abandoned_leaf() {
        // A missing flag must degrade to something harmless, never to the
        // opposite outcome (pruning): retiring an abandoned leaf would
        // silently assert the rejected work was finished.
        let (_t, g) = jj_grove();
        touch(&g, "02-ABANDONED-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let err = leaf_retire(guard(&g), Path::new("02-ABANDONED-impl--add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("abandoned"), "got {err}");
    }

    #[test]
    fn retire_refuses_a_foreign_file() {
        let (_t, g) = jj_grove();
        touch(&g, "README.md", "readme");
        commit_all(&g);
        let err = leaf_retire(guard(&g), Path::new("README.md")).unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

    #[test]
    fn retire_accepts_an_absolute_path() {
        let (_t, g) = jj_grove();
        touch(&g, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let abs = g.join("02-impl--add-k4.md");
        let done = leaf_retire(guard(&g), &abs).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl--add-k4.md");
    }

    // ---- lifecycle over untracked leaves (issue #3's root cause) -------------
    //
    // Issue #3's defect, in the lifecycle verbs: a leaf grown this session is
    // uncommitted until the enclosing task commits, and the version-control-aware
    // move these verbs used to reach for had nothing recorded to move. The verbs
    // now rename through `ordinal-fs-tree`, which consults no repository at all
    // (`docs/adr/grove-does-not-stage-its-own-renames.md`), so these cases can no
    // longer fail that way — they are kept because the fixtures are the ones a
    // real session produces, and a verb that grew a tracked-only path would fail
    // them again.

    #[test]
    fn retire_an_untracked_leaf_added_this_session() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "ship");
        // No commit_all: the grow verb leaves it uncommitted, by design.
        let done = leaf_retire(guard(&g), &leaf).unwrap();
        assert_eq!(name_of(&done), "01-DONE-impl--ship-k1.md");
        assert!(
            done.is_file(),
            "the retired leaf is on disk under its DONE name"
        );
        assert!(!leaf.exists(), "the live name is gone");
    }

    #[test]
    fn decompose_an_untracked_leaf_added_this_session() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "big");
        // "The current item proving bigger" — the canonical mid-session decompose.
        let (brief, child) = leaf_decompose(guard(&g), &leaf, &a_slug("first"), None).unwrap();
        assert_eq!(name_of(&brief), "BRIEF.md");
        assert_eq!(name_of(&child), "01-impl--first-k2.md");
        assert!(g.join("01-big-k1").is_dir(), "the leaf became a node dir");
        assert!(
            !leaf.exists(),
            "the leaf file is gone (it became the BRIEF)"
        );
    }

    #[test]
    fn prune_an_untracked_leaf_added_this_session() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "dead");
        let result = leaf_prune(guard(&g), &leaf).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "01-ABANDONED-impl--dead-k1.md");
        assert!(!leaf.exists(), "the live name is gone");
    }

    // ---- leaf-prune (pruning) ------------------------------------------

    #[test]
    fn prune_leaf_adds_abandoned_infix_keeping_position_and_key() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let result = leaf_prune(guard(&g), Path::new("02-impl--add-k4.md")).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl--add-k4.md");
        assert!(result.left_done.is_empty());
        let files = list(&g);
        assert!(files.contains(&"02-ABANDONED-impl--add-k4.md".to_string()));
        assert!(
            !files.contains(&"02-impl--add-k4.md".to_string()),
            "old name gone"
        );
    }

    #[test]
    fn prune_leaf_does_not_rewrite_the_header_or_body() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-impl--add-k4.md", "# add-k4\n\nbody\n");
        commit_all(&g);
        let result = leaf_prune(guard(&g), Path::new("02-impl--add-k4.md")).unwrap();
        assert_eq!(
            body(&result.marked[0]),
            "# add-k4\n\nbody\n",
            "content byte-identical"
        );
    }

    #[test]
    fn prune_leaf_works_on_a_nested_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &design.join("02-impl--add-k4.md")).unwrap();
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl--add-k4.md");
        assert_eq!(name_of(result.marked[0].parent().unwrap()), "01-design-k1");
    }

    #[test]
    fn prune_leaf_refuses_a_node_brief() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_prune(guard(&g), &node.join("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_the_root_brief() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_an_already_done_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "02-DONE-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new("02-DONE-impl--add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("DONE"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_an_already_abandoned_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "02-ABANDONED-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new("02-ABANDONED-impl--add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("already"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_a_foreign_file() {
        let (_t, g) = jj_grove();
        touch(&g, "README.md", "readme");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new("README.md")).unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

    #[test]
    fn prune_leaf_accepts_an_absolute_path() {
        let (_t, g) = jj_grove();
        touch(&g, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let abs = g.join("02-impl--add-k4.md");
        let result = leaf_prune(guard(&g), &abs).unwrap();
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl--add-k4.md");
    }

    // ---- leaf-prune on a node: bulk arity (pruning) -------------------

    #[test]
    fn prune_node_marks_every_live_leaf_in_the_subtree() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &node).unwrap();
        let names: Vec<String> = result.marked.iter().map(|p| name_of(p)).collect();
        assert_eq!(
            names,
            vec!["01-ABANDONED-impl--a-k3.md", "02-ABANDONED-impl--b-k4.md"]
        );
        assert!(result.left_done.is_empty());
    }

    #[test]
    fn prune_node_leaves_done_leaves_untouched() {
        // That work really was done — a bulk abandon does not retroactively
        // un-finish it.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &node).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl--b-k4.md");
        assert_eq!(result.left_done.len(), 1);
        assert_eq!(name_of(&result.left_done[0]), "01-DONE-impl--a-k3.md");
        // The DONE leaf's name (and so its position and key) is untouched.
        assert!(node.join("01-DONE-impl--a-k3.md").is_file());
    }

    #[test]
    fn prune_node_recurses_into_a_grandchild_node() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let outer = mknode(&g, "01-outer-k1", "outer-k1");
        let inner = mknode(&outer, "01-inner-k2", "inner-k2");
        touch(&inner, "01-impl--deep-k3.md", "deep-k3");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &outer).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "01-ABANDONED-impl--deep-k3.md");
        assert_eq!(
            name_of(result.marked[0].parent().unwrap()),
            "01-inner-k2",
            "the grandchild's own directory is untouched — only the leaf file is marked"
        );
    }

    #[test]
    fn prune_node_marks_a_subtree_mixing_tracked_and_untracked_leaves() {
        // The bulk analogue of `prune_an_untracked_leaf_added_this_session`: one
        // decision kills a subtree whose leaves were grown across several sessions,
        // so some are committed and some are still working-tree-only. Every live
        // leaf is marked regardless — trackedness is not a precondition of a rename.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        commit_all(&g); // a and b are tracked
        touch(&node, "03-impl--c-k5.md", "c-k5"); // c is not

        let result = leaf_prune(guard(&g), &node).unwrap();

        assert_eq!(result.marked.len(), 3, "every live leaf marked");
        let names = list(&node);
        for expected in [
            "01-ABANDONED-impl--a-k3.md",
            "02-ABANDONED-impl--b-k4.md",
            "03-ABANDONED-impl--c-k5.md",
        ] {
            assert!(
                names.contains(&expected.to_string()),
                "missing {expected} (names: {names:?})"
            );
        }
    }

    #[test]
    fn prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address() {
        // pruning: a failure partway through the subtree walk must not leave
        // earlier leaves already marked while the operator sees only the
        // trailing error. Planning and validating the whole subtree against the
        // first guard's snapshot is what prevents it, and the repro is a botched
        // earlier prune — an `ABANDONED` twin sitting beside a live leaf.
        //
        // **The property survived the flip and the diagnosis changed.** The twin
        // wears the very name the mark would place, so it necessarily carries
        // the same key: an outcome infix is part of the name and the key is part
        // of the name, and a name that collides collides in both. So what is
        // wrong with this tree is not that a destination is taken but that key 5
        // names two entries — and `rewrite` is called *by key*, which on this
        // tree means Grove cannot say which leaf it would mark. Refusing that is
        // strictly prior, and it is Grove's own precondition rather than a
        // second wording of the library's `DestinationOccupied`.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        touch(&node, "03-impl--c-k5.md", "c-k5");
        touch(&node, "03-ABANDONED-impl--c-k5.md", "c-k5");
        commit_all(&g);

        let err = leaf_prune(guard(&g), &node).unwrap_err();
        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 5"),
            "got {err}"
        );

        // Nothing was mutated: every live name is untouched, none newly marked.
        let names = list(&node);
        assert!(
            names.contains(&"01-impl--a-k3.md".to_string()),
            "got {names:?}"
        );
        assert!(
            names.contains(&"02-impl--b-k4.md".to_string()),
            "got {names:?}"
        );
        assert!(
            names.contains(&"03-impl--c-k5.md".to_string()),
            "got {names:?}"
        );
        assert!(
            !names.contains(&"01-ABANDONED-impl--a-k3.md".to_string())
                && !names.contains(&"02-ABANDONED-impl--b-k4.md".to_string()),
            "a validation failure must leave the whole subtree untouched: {names:?}"
        );
    }

    #[test]
    fn retiring_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed() {
        // The single-leaf half, and the failure it prevents is worse than an
        // error. `rewrite` is called by key; with two entries under key 1,
        // `by_key` answers with whichever the walk reaches first — an order
        // nothing models — so retiring the live leaf by *path* rewrote the DONE
        // twin onto its own name, changed nothing, and reported the twin's path
        // as the retired one. Success, silently aimed at the wrong entry.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl--a-k1.md", "a-k1");
        touch(&g, "01-DONE-impl--a-k1.md", "a-k1");
        commit_all(&g);

        let err = leaf_retire(guard(&g), Path::new("01-impl--a-k1.md")).unwrap_err();

        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 1"),
            "got {err}"
        );
        assert!(
            g.join("01-impl--a-k1.md").is_file(),
            "a refused mark changes nothing"
        );
    }

    #[test]
    fn pruning_a_node_takes_one_guard_per_mark() {
        // `rewrite` consumes its guard, so a subtree of N live leaves is N
        // observations of the tree where it used to be one critical section.
        // That is the whole of what Grove accepted in
        // `docs/adr/bulk-marks-are-not-atomic.md`, and it is asserted rather
        // than described: a later leaf that restores atomicity, or that adds a
        // re-read nobody meant to add, moves this number.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        touch(&node, "03-DONE-impl--c-k5.md", "c-k5");
        touch(&node, "04-impl--d-k6.md", "d-k6");
        commit_all(&g);
        crate::task_tree::reset_read_count();

        let result = leaf_prune(guard(&g), &node).unwrap();

        assert_eq!(result.marked.len(), 3);
        assert_eq!(result.left_done.len(), 1);
        assert_eq!(
            crate::task_tree::read_count(),
            3,
            "one guard per mark — the planning guard is spent on the first, and \
             a leaf left alone costs none"
        );
    }

    #[test]
    fn prune_node_with_nothing_live_marks_nothing() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-impl--a-k3.md", "a-k3");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &node).unwrap();
        assert!(result.marked.is_empty());
        assert_eq!(result.left_done.len(), 1);
    }

    #[test]
    fn prune_refuses_the_grove_root() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl--a-k1.md", "a-k1");
        commit_all(&g);
        let err = leaf_prune(guard(&g), &g).unwrap_err();
        assert!(err.to_string().contains("grove root"), "got {err}");
        // Nothing was touched.
        assert!(g.join("01-impl--a-k1.md").is_file());
    }

    #[test]
    fn prune_refuses_the_grove_root_given_as_a_relative_dot_path() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new(".")).unwrap_err();
        assert!(err.to_string().contains("grove root"), "got {err}");
    }
}

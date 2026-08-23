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
//   * `root-init` writes the root `BRIEF.md` (the one unkeyed singleton) and a
//     first **requirements** leaf `01-requirements-<slug>-k1.md` — a 2-digit per-level
//     position;
//   * `leaf-decompose` turns the leaf *file* `NN-<kind>-<slug>-k<key>.md` into a node
//     *directory* `NN-<slug>-k<key>/` (**key preserved** — the entity that was the
//     leaf becomes the node), renaming the leaf body in as the node's `BRIEF.md`
//     and growing a first child atomically so a node is never childless;
//   * `leaf-retire` adds a `DONE` infix in place (`NN-<kind>-<slug>-k<key>.md` →
//     `NN-DONE-<kind>-<slug>-k<key>.md`), keeping the retired leaf in its directory at its
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

use crate::leaf::Kind;
use crate::task_grow;
use crate::task_name::{Outcome, Parts, Slug, TaskName};
use crate::task_tree;
use crate::tree_access;
use anyhow::{bail, Context, Result};
use ordinal_fs_tree::{
    Entry, EntryName, Found, Key, NewEntry, Ordinal, Report, Snapshot, Target, Verdict,
};
use std::fs;
use std::path::{Path, PathBuf};

/// The slug a grove scaffolded by the driver is named with. `root-init` takes
/// one from its operator; the lifecycle transition has nobody to ask.
const DEFAULT_ROOT_SLUG: &str = "plan";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrentTransition {
    RootInitialized,
    RootInitRecovered,
    Migrated,
    AlreadyCurrent,
}

/// Classify or complete the one lifecycle transition needed before current-tree
/// selection. The whole observation and any resulting mutation share one
/// exclusive working-tree guard.
///
/// **Public as a seam, not as a spare door.** No production caller reaches it —
/// the driver takes [`transition_driver_to_current`] — so a reachability sweep
/// reports it dead every time, and it survives that report deliberately
/// (`dead-non-launch-exports-k166`). The driver twin is `pub(crate)` *and* reaps
/// orphaned finish artifacts first, so an integration test cannot substitute it
/// for the classification without also asserting through a best-effort cleanup
/// it is not testing. That is the discriminator against the locked wrappers the
/// withdrawn appender lost: those had a production composition a test could
/// perform itself, and this does not.
pub fn transition_to_current(worktree: &Path) -> Result<CurrentTransition> {
    let classified = {
        let _guard = tree_access::write_for_lifecycle(worktree)?;
        classify_unlocked(worktree)?
    };
    settle(worktree, classified)
}

/// Reap finish artifacts and perform the driver's ordinary lifecycle
/// transition under one exclusive working-tree guard. Cleanup remains
/// best-effort and cannot classify the task root.
pub(crate) fn transition_driver_to_current(worktree: &Path) -> Result<CurrentTransition> {
    let classified = {
        let _guard = tree_access::write_for_lifecycle(worktree)?;
        if let Err(error) = crate::finish_cleanup::reap_orphaned(worktree) {
            eprintln!(
                "grove: warning: could not complete orphaned finish cleanup; lifecycle classification is unchanged: {error:#}"
            );
        }
        classify_unlocked(worktree)?
    };
    settle(worktree, classified)
}

/// What the guarded half of a lifecycle transition concluded.
///
/// **The transition is two phases because the scaffold is**, and the scaffold is
/// two phases because grove cannot nest its own lock inside the library's — both
/// `flock` the directory containing the tree root, and two open file
/// descriptions on one directory do not share a lock. Classification needs
/// grove's guard, since the tree it classifies may be absent, legacy or mid
/// transaction and the library can read none of those; appending the first leaf
/// needs the library's. So the guarded half creates what only grove can create
/// and says so, and [`settle`] finishes the job once the guard is gone.
enum Classification {
    /// The root and its charter exist and the first leaf does not:
    /// [`complete_scaffold`] owes this tree a leaf and a format witness.
    Scaffolded,
    /// Nothing further is owed; this is the transition to report.
    Settled(CurrentTransition),
}

/// Finish a classification outside grove's guard.
fn settle(worktree: &Path, classified: Classification) -> Result<CurrentTransition> {
    match classified {
        Classification::Settled(transition) => Ok(transition),
        Classification::Scaffolded => {
            complete_scaffold(&worktree.join(".grove"), &default_root_slug())?;
            Ok(CurrentTransition::RootInitialized)
        }
    }
}

fn classify_unlocked(worktree: &Path) -> Result<Classification> {
    let grove_root = worktree.join(".grove");
    match fs::symlink_metadata(&grove_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            create_root_unlocked(worktree, &grove_root)?;
            return Ok(Classification::Scaffolded);
        }
        Err(error) => {
            return Err(error)
                .with_context(|| format!("checking grove root {}", grove_root.display()))
        }
        Ok(metadata) if !metadata.file_type().is_dir() => {
            bail!("grove root is not a directory: {}", grove_root.display())
        }
        Ok(_) => {}
    }
    if crate::finish_transaction::recover_pending(worktree, &grove_root)?
        == crate::finish_transaction::FinishRecovery::Committed
    {
        create_root_unlocked(worktree, &grove_root)?;
        return Ok(Classification::Scaffolded);
    }
    let name = grove_name(worktree);
    let transaction = crate::tree_migration_transaction::run_unlocked(&grove_root, || {
        crate::repo::commit_session_kind_migration(worktree, &name)
    })?;
    Ok(Classification::Settled(match transaction {
        crate::tree_migration_transaction::TransactionOutcome::RootInitRecovered => {
            CurrentTransition::RootInitRecovered
        }
        crate::tree_migration_transaction::TransactionOutcome::Migrated => {
            CurrentTransition::Migrated
        }
        crate::tree_migration_transaction::TransactionOutcome::AlreadyCurrent => {
            CurrentTransition::AlreadyCurrent
        }
    }))
}

/// Materialize the driver-owned finish sentinel after a shared selection found
/// no live work. The exclusive re-selection closes the gap between that read
/// and allocation: newly inserted ordinary work wins, and an existing finish
/// is reused.
pub fn materialize_finish(worktree: &Path) -> Result<crate::task_tree::SelectedLeaf> {
    let grove_root = worktree.join(".grove");
    // One observation for both halves: the re-selection that may return early
    // and the append that happens when it does not read the *same* snapshot, so
    // nothing can appear between them.
    let tree = task_tree::write(&grove_root)?;
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
    let path = task_grow::allocated(&report, &[key])?.remove(0);
    Ok(crate::task_tree::SelectedLeaf {
        path,
        handle: finish_handle(key.context("a finish sentinel was created without a key")?),
        kind: Kind::Finish,
    })
}

/// The driver's finish sentinel, as an entry to create.
///
/// A `None` key is an exhausted keyspace, which is
/// [`Refusal::KeysExhausted`](ordinal_fs_tree::Refusal) and the library's to
/// state; the entry carries no bytes then and never needs any, because a refusal
/// writes nothing (`task_grow::new_leaf` says the same of an ordinary leaf).
fn new_finish_leaf(key: Option<Key>) -> Result<NewEntry<Parts>> {
    let parts = Parts::leaf(
        Outcome::Live,
        Kind::Finish,
        Slug::new("finish").map_err(|error| anyhow::anyhow!("slug \"finish\": {error}"))?,
    );
    Ok(match key {
        Some(key) => NewEntry::new(parts, finish_body(&finish_handle(key)).into_bytes()),
        None => NewEntry::empty(parts),
    })
}

fn finish_handle(key: Key) -> String {
    format!("finish-k{}", key.get())
}

fn finish_body(handle: &str) -> String {
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

/// Revalidate and commit the complete finish cycle's tree deletion. This is a
/// deterministic last-moment guard; whether a human confirmed teardown is the
/// calling finish session's responsibility.
pub fn finish_commit(worktree: &Path, finish_handle: &str) -> Result<()> {
    let grove_root = worktree.join(".grove");
    // **The root is classified before the tree is opened, because two of the
    // three answers are not the library's to give.** A missing root is not even a
    // failure here, and a `.grove` that is a *symlink* to a directory is one the
    // library would happily read — it follows links, as every reader does — while
    // this verb must refuse it unfollowed, since a no-follow transaction may not
    // treat a directory elsewhere as its own tree. The guard below is still the
    // authority on the tree; this is the wording that authority cannot supply.
    match fs::symlink_metadata(&grove_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error)
                .with_context(|| format!("checking grove root {}", grove_root.display()))
        }
        Ok(metadata) if !metadata.file_type().is_dir() => {
            bail!("grove root is not a directory: {}", grove_root.display())
        }
        Ok(_) => {}
    }
    let tree = match task_tree::write(&grove_root) {
        Ok(tree) => tree,
        Err(refusal) => {
            return finish_commit_refusal(worktree, &grove_root, finish_handle, refusal)
        }
    };
    let selection = task_tree::select_in_write(&tree)?
        .context("the requested finish leaf is no longer live")?;
    if selection.kind != Kind::Finish {
        bail!(
            "cannot finish while live work remains: {} ({})",
            selection.handle,
            selection.path.display()
        );
    }
    if selection.handle != finish_handle {
        bail!(
            "requested finish handle {finish_handle:?} does not match the live finish leaf {}",
            selection.handle
        );
    }

    // The guard is held across the teardown, as grove's own was: the transaction
    // creates its witnesses and deletes the tree under it, and nothing else may
    // observe `.grove/` in between. It is dropped when this function returns —
    // after the root it names is gone, which the lock does not care about,
    // because the lock is on the directory *containing* the root.
    let outcome = crate::finish_transaction::finish(worktree, &grove_root, finish_handle);
    drop(tree);
    outcome
}

/// What a failed exclusive guard means to `finish-commit`, which is not always a
/// failure.
///
/// **Task-root absence never classifies an attempted finish as success**: a death
/// before the deletion commit exposes exactly this shape. The only thing that can
/// license the retry is the repository's own immediate result, proven against
/// *this* launch's finish attempt — so the missing root is routed to that proof
/// rather than reported. Every other refusal is the guard's own, printed
/// unchanged (`docs/ARCHITECTURE.md#library-refusals`, clause 3), except the one
/// shape the guard cannot word for itself: a `.grove` that is not a directory
/// reads to the library as a root it cannot list, and grove has always said which
/// of the two it is.
fn finish_commit_refusal(
    worktree: &Path,
    grove_root: &Path,
    finish_handle: &str,
    refusal: anyhow::Error,
) -> Result<()> {
    match fs::symlink_metadata(grove_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            crate::finish_transaction::verify_lost_result(worktree, finish_handle).with_context(
                || {
                    format!(
                        "no Grove task tree at {}, and no verifiable {finish_handle} teardown \
                         result for this finish attempt",
                        grove_root.display()
                    )
                },
            )
        }
        Ok(metadata) if !metadata.file_type().is_dir() => {
            bail!("grove root is not a directory: {}", grove_root.display())
        }
        _ => Err(refusal),
    }
}

/// `root-init [<slug>]`: scaffold a fresh grove under `worktree/.grove` — the root
/// `BRIEF.md` (the one unkeyed singleton) and a first **requirements** leaf
/// `01-requirements-<slug>-k1.md`. Returns the absolute paths created:
/// `BRIEF.md`, the leaf, then `FORMAT` (written last). Refuses to clobber an
/// existing `.grove/`. Working-tree only — no commit.
///
/// The kind is fixed, with no `--kind` flag: a brand-new grove's first session
/// takes the human's own words as its only input — nothing else is on disk —
/// which is the generating rule for **HITL**, and `requirements` is the kind
/// that rule names (`docs/ARCHITECTURE.md#task-kind-taxonomy`). It
/// being fixed is also load-bearing for routing: the loop driver launches the
/// `start` session *before* this verb has run, so it can only route the
/// bootstrap by construction (`fresh-grove-start-contract`).
pub fn root_init(worktree: &Path, slug: &str) -> Result<Vec<PathBuf>> {
    // Validate before touching the filesystem so a bad slug never leaves a stray
    // `.grove/` behind.
    let slug = task_grow::leaf_slug(slug)?;
    let grove_root = worktree.join(".grove");

    // **Phase one, under grove's own guard: the two things the library cannot
    // do.** `ordinal_fs_tree::fs` locks the directory *containing* the root — so
    // it survives the root's creation and deletion, which is exactly what a
    // scaffold needs — but it still has to reach the root to snapshot it, so it
    // cannot create one. Nor can it create the distinguished child: a `BRIEF.md`
    // arrives through `promote` and no other operation, and there is nothing here
    // to promote. Both, therefore, and the refusal to clobber an existing
    // `.grove/`, stay grove's and stay under grove's lock.
    let brief_path = {
        let _guard = tree_access::write_for_lifecycle(worktree)?;
        match fs::symlink_metadata(&grove_root) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("checking grove root {}", grove_root.display()))
            }
            Ok(_) => bail!("grove root already exists: {}", grove_root.display()),
        }
        create_root_unlocked(worktree, &grove_root)?
    };

    // **Phase two, under the library's: the first leaf.** The guard is released
    // between the phases because grove cannot nest its lock inside the library's,
    // and the window that opens is the one `FORMAT` already exists to make
    // legible — a root without a format witness is a *partial* root, which every
    // other verb refuses and which `recover_partial_root_init_unlocked`
    // completes.
    let (leaf_path, format_path) = complete_scaffold(&grove_root, &slug)?;
    Ok(vec![brief_path, leaf_path, format_path])
}

/// The slug `root-init` uses when nobody supplied one, and the only slug the
/// driver's own scaffold can use.
fn default_root_slug() -> Slug {
    Slug::new(DEFAULT_ROOT_SLUG).expect("the default root slug is a valid slug")
}

/// Create the tree root and its charter. The caller holds grove's exclusive
/// working-tree guard, and the root does not exist yet.
///
/// Returns the charter's path — the first of the three `root-init` reports, and
/// the only one written here.
fn create_root_unlocked(worktree: &Path, grove_root: &Path) -> Result<PathBuf> {
    #[cfg(test)]
    tree_access::assert_guard_held(grove_root);

    fs::create_dir_all(grove_root).with_context(|| format!("creating {}", grove_root.display()))?;
    let brief_path = grove_root.join(crate::task_name::BRIEF);
    write_root_brief(&brief_path, &grove_name(worktree))?;
    Ok(brief_path)
}

/// Complete a root that has its charter and nothing else: the first
/// `requirements` leaf, then the format witness, in that order.
///
/// # It is idempotent, and that is what makes releasing the guard safe
///
/// Between phase one and phase two nothing holds the tree, so a second process
/// can meet the partial root and complete it — `bare grove` does exactly that,
/// through the migration transaction's partial-scaffold recovery. Appending
/// unconditionally would then give the tree *two* first leaves. So the append
/// happens only when the snapshot holds no positioned entry at all, and the
/// witness write is a same-directory rename that is safe to repeat.
///
/// # The witness is still written last, and still under the guard
///
/// `.grove/FORMAT` is what makes a partial root recognisable as partial
/// (`tree_format::write_current_last`), so it cannot move earlier. The append
/// consumed the guard that could have covered it — one guard is one operation —
/// so a second is taken for the write, which is the same answer `leaf-insert`'s
/// lint and `leaf-decompose`'s retitle give to the same shape: reopen, do not run
/// the tail unheld.
fn complete_scaffold(grove_root: &Path, slug: &Slug) -> Result<(PathBuf, PathBuf)> {
    let leaf_path = scaffold_first_leaf(grove_root, slug)?;
    let _guard = task_tree::write_scaffold(grove_root)?;
    let format_path = crate::tree_format::write_current_last(grove_root)?;
    Ok((leaf_path, format_path))
}

fn scaffold_first_leaf(grove_root: &Path, slug: &Slug) -> Result<PathBuf> {
    let tree = task_tree::write_scaffold(grove_root)?;
    let existing = tree
        .snapshot()
        .walk()
        .find(|entry| entry.key().is_some())
        .map(|entry| task_tree::entry_path(tree.root(), entry));
    if let Some(path) = existing {
        return Ok(path);
    }
    // A root holding only its charter has no keyed name, so `append` composes
    // ordinal 1 and key 1 — the scaffold's fixed shape, arrived at by the
    // library's own rules rather than written down a second time. The key is
    // predicted for the template handle and checked against the report, exactly
    // as every grow verb does.
    let key = task_tree::next_key(tree.snapshot());
    let entry = task_grow::new_leaf(key, Outcome::Live, Kind::Requirements, slug);
    let report = tree
        .append(Target::Root, entry)
        .map_err(task_tree::raised)?;
    Ok(task_grow::allocated(&report, &[key])?.remove(0))
}

/// Complete only the deterministic fresh-tree scaffold that `root_init` owns.
/// The caller must hold the universal exclusive tree guard. `Ok(false)` leaves
/// a non-scaffold tree untouched so the migration planner can classify it.
pub(crate) fn recover_partial_root_init_unlocked(grove_root: &Path) -> Result<bool> {
    #[cfg(test)]
    tree_access::assert_guard_held(grove_root);

    let worktree = grove_root.parent().with_context(|| {
        format!(
            "partial root scaffold {} has no working-tree parent",
            grove_root.display()
        )
    })?;
    let brief_path = grove_root.join("BRIEF.md");
    let mut entries = fs::read_dir(grove_root)
        .with_context(|| format!("reading partial root scaffold {}", grove_root.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    let mut scaffold_leaf_candidates = entries.iter().filter_map(|entry| {
        let name = entry.file_name();
        Some((entry.path(), scaffold_leaf_slug(name.to_str()?)?))
    });
    let scaffold_leaf = scaffold_leaf_candidates.next();
    if let Some((duplicate_path, _)) = scaffold_leaf_candidates.next() {
        bail!(
            "ambiguous partial root scaffold at {}: multiple first requirements leaves, including {}",
            grove_root.display(),
            duplicate_path.display()
        );
    }
    let scaffold_slug = scaffold_leaf
        .as_ref()
        .map_or_else(default_root_slug, |(_, slug)| slug.clone());
    let leaf_name = scaffold_leaf_name(&scaffold_slug).to_string();
    let leaf_path = grove_root.join(&leaf_name);
    let format_temporary_path = grove_root.join(".FORMAT.tmp");
    let expected_brief = root_brief_body(&grove_name(worktree));
    let expected_leaf = task_grow::task_template_body(scaffold_slug.as_str(), SCAFFOLD_KEY);

    let expected = [
        (brief_path.as_path(), expected_brief.as_bytes()),
        (leaf_path.as_path(), expected_leaf.as_bytes()),
        (
            format_temporary_path.as_path(),
            crate::tree_format::CURRENT_FILE_CONTENTS.as_bytes(),
        ),
    ];

    // `.FORMAT.tmp` is writer-owned transaction state, never legacy tree
    // content. Validate it before deciding that the surrounding entries belong
    // to a legacy tree; otherwise `write_current_last` could follow and truncate
    // a near-match symlink during migration.
    let temporary_is_present = validate_partial_scaffold_file(
        &format_temporary_path,
        crate::tree_format::CURRENT_FILE_CONTENTS.as_bytes(),
    )?;
    let scaffold_leaf_is_present = if scaffold_leaf.is_some() {
        match partial_scaffold_file_match(&leaf_path, expected_leaf.as_bytes())? {
            Some(true) => true,
            Some(false)
                if !temporary_is_present
                    && crate::tree_migrate::has_explicit_legacy_kind(&leaf_path)? =>
            {
                // Legacy-v2 slugs are kind-free. A valid legacy slug such as
                // `requirements-design` therefore overlaps the current
                // `requirements` filename prefix without being root-init state.
                return Ok(false);
            }
            Some(false) => {
                validate_partial_scaffold_file(&leaf_path, expected_leaf.as_bytes())?;
                unreachable!("a differing scaffold leaf is rejected")
            }
            None => false,
        }
    } else {
        false
    };
    let brief_match = partial_scaffold_file_match(&brief_path, expected_brief.as_bytes())?;

    let mut unexpected = Vec::new();
    for entry in entries {
        if !expected.iter().any(|(path, _)| path == &entry.path()) {
            unexpected.push(entry.path());
        }
    }

    if !unexpected.is_empty() {
        if scaffold_leaf_is_present || temporary_is_present {
            bail!(
                "ambiguous partial root scaffold at {}: exact fresh-tree content is mixed with \
                 unexpected entries: {}",
                grove_root.display(),
                unexpected
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        return Ok(false);
    }
    if scaffold_leaf.is_none() && !temporary_is_present && matches!(brief_match, Some(false)) {
        return Ok(false);
    }

    for (path, expected_body) in expected {
        validate_partial_scaffold_file(path, expected_body)?;
    }

    if !brief_path.exists() {
        write_root_brief(&brief_path, &grove_name(worktree))?;
    }
    if !leaf_path.exists() {
        // **Recovery completes a scaffold; it does not grow a tree.** There is
        // nothing here for the library to allocate — the ordinal, the key and the
        // bytes are all fixed, and `validate_partial_scaffold_file` has already
        // established that the destination is absent rather than merely
        // unfollowable. Delegating it would mean taking the library's lock while
        // the migration transaction holds grove's, which is the nesting the two
        // guards forbid, and the tree being recovered is one the library cannot
        // read anyway: it has no format witness, which is what makes it partial.
        fs::write(&leaf_path, &expected_leaf)
            .with_context(|| format!("writing {}", leaf_path.display()))?;
    }
    crate::tree_format::write_current_last(grove_root)?;
    Ok(true)
}

/// The fixed ordinal and key of a fresh grove's first leaf. A root holding only
/// its charter has no keyed name, so these are what the library's own `append`
/// composes — written down here only because recovery has to *recognise* the
/// result rather than produce it.
const SCAFFOLD_ORDINAL: u32 = 1;
const SCAFFOLD_KEY: u32 = 1;

/// The name a fresh grove's first leaf carries, for this slug.
fn scaffold_leaf_name(slug: &Slug) -> TaskName {
    TaskName::Positioned {
        ordinal: Ordinal::new(SCAFFOLD_ORDINAL),
        key: Key::new(SCAFFOLD_KEY),
        parts: Parts::leaf(Outcome::Live, Kind::Requirements, slug.clone()),
    }
}

/// The slug of a filename that is a fresh grove's first leaf, or `None`.
///
/// Matched on the **name alone**, which is what makes a directory wearing the
/// name a scaffold-file collision — reported by `partial_scaffold_file_match` in
/// its own words — rather than something this filter silently drops.
fn scaffold_leaf_slug(name: &str) -> Option<Slug> {
    let Verdict::Entry(TaskName::Positioned {
        ordinal,
        key,
        parts:
            Parts::Leaf {
                outcome: Outcome::Live,
                kind: Kind::Requirements,
                slug,
            },
    }) = TaskName::parse(name, Found::File)
    else {
        return None;
    };
    (ordinal.get() == SCAFFOLD_ORDINAL && key.get() == SCAFFOLD_KEY).then_some(slug)
}

fn validate_partial_scaffold_file(path: &Path, expected_body: &[u8]) -> Result<bool> {
    match partial_scaffold_file_match(path, expected_body)? {
        None => Ok(false),
        Some(true) => Ok(true),
        Some(false) => bail!(
            "partial root scaffold file {} differs from the deterministic fresh-tree content; \
             refusing to overwrite it",
            path.display()
        ),
    }
}

fn partial_scaffold_file_match(path: &Path, expected_body: &[u8]) -> Result<Option<bool>> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("checking partial root scaffold path {}", path.display()))
        }
    };
    if !metadata.file_type().is_file() {
        bail!(
            "partial root scaffold path collision at {}: expected a regular file",
            path.display()
        );
    }
    let body = fs::read(path)
        .with_context(|| format!("reading partial root scaffold file {}", path.display()))?;
    Ok(Some(body == expected_body))
}

/// `leaf-decompose <leaf-path> <first-child-slug>`: convert a live leaf file
/// `NN-<kind>-<slug>-k<key>.md` into a node directory `NN-<slug>-k<key>/` (**key
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
pub fn leaf_decompose(
    grove_root: &Path,
    leaf_path: &Path,
    first_child_slug: &str,
    kind_override: Option<Kind>,
) -> Result<(PathBuf, PathBuf)> {
    // Grove's own precondition, before the tree is even observed, so a bad slug
    // leaves the leaf un-decomposed. It could sit inside the guard with the rest
    // of them; it stays out here because it needs nothing from the tree, and
    // refusing without taking an exclusive lock is strictly kinder.
    let child_slug = task_grow::leaf_slug(first_child_slug)?;

    let tree = task_tree::write(grove_root)?;
    let root = tree.root().to_path_buf();
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
        task_grow::refuse_finish_kind(kind, "leaf-decompose")?;
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
    let child = task_grow::new_leaf(child_key, Outcome::Live, kind, &child_slug);
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
    let _guard = task_tree::reopen_write(grove_root)?;
    append_brief_suffix_in_file(&brief_path, slug.as_str(), key.get())?;
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
            if *kind == Kind::Finish {
                bail!("`finish` is driver-reserved and cannot be decomposed");
            }
            Ok((*kind, slug))
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

/// `leaf-retire <leaf-path>`: rename a live leaf `NN-<kind>-<slug>-k<key>.md` →
/// `NN-DONE-<kind>-<slug>-k<key>.md` in place, keeping its position and key. The
/// `DONE` infix is filename-only — the `# <handle>` header is byte-identical.
/// Refuses a brief, a node directory, and an already-`DONE` leaf. Returns the
/// retired file's absolute path. Working-tree only — no commit.
///
/// **The mark is `ordinal_fs_tree`'s `rewrite`**, which is what a mark *is*
/// algebraically: the entry keeps its ordinal, its key and its species, and only
/// the opaque remainder of its name moves. The rename underneath is
/// `rename(2)` — plain on every lane, git included; see
/// [`docs/adr/grove-does-not-stage-its-own-renames.md`](../docs/adr/grove-does-not-stage-its-own-renames.md).
pub fn leaf_retire(grove_root: &Path, leaf_path: &Path) -> Result<PathBuf> {
    let tree = task_tree::write(grove_root)?;
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
            if *kind == Kind::Finish {
                bail!("`finish` is driver-reserved and cannot be retired");
            }
            Ok(Parts::leaf(Outcome::Done, *kind, slug.clone()))
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
pub fn leaf_prune(grove_root: &Path, path: &Path) -> Result<PruneResult> {
    let tree = task_tree::write(grove_root)?;
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
    if *kind == Kind::Finish {
        bail!("`finish` is driver-reserved and cannot be pruned");
    }
    Ok(Planned::ToMark {
        key: task_tree::addressable_key(root, snapshot, entry)?,
        parts: Parts::leaf(Outcome::Abandoned, *kind, slug.clone()),
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
    planning_guard: task_tree::TreeWrite,
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
                let tree = match held.take() {
                    Some(tree) => tree,
                    None => task_tree::reopen_write(root)?,
                };
                let report = tree.rewrite(key, parts).map_err(task_tree::raised)?;
                result.marked.push(marked_path(&report)?);
            }
        }
    }
    Ok(result)
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
fn grove_name(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// Emit the minimal section-header scaffold for the root `BRIEF.md` — headers only,
/// no prose (the bootstrap session fills them). The root brief is the one
/// unkeyed, position-free singleton, unchanged across schemes.
fn write_root_brief(path: &Path, name: &str) -> Result<()> {
    let body = root_brief_body(name);
    fs::write(path, body.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

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
fn append_brief_suffix_in_file(path: &Path, slug: &str, key: u32) -> Result<()> {
    let body =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let handle = format!("# {slug}-k{key}");
    let new_first = if first.trim_end() == handle {
        format!("{handle} — brief")
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
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    /// A bare worktree dir with **no** `.grove/` yet — for `root_init`, which
    /// creates the grove itself and needs no git (it never renames an entry).
    fn worktree() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let wt = tmp.path().join("my-grove");
        fs::create_dir_all(&wt).unwrap();
        (tmp, wt)
    }

    /// A `.grove/` inside a real git repo. The repo is these tests' **instrument**
    /// rather than their prerequisite: every verb below renames inside an
    /// `ordinal-fs-tree` operation, which uses `rename(2)` and stages nothing
    /// (`docs/adr/grove-does-not-stage-its-own-renames.md`), so nothing here needs
    /// tracked files to operate on. [`stage_all`] is what makes the fixtures the
    /// ones a real session produces.
    fn git_grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path().to_path_buf();
        run_git(&repo, &["init", "-q"]);
        run_git(&repo, &["config", "user.email", "t@example.com"]);
        run_git(&repo, &["config", "user.name", "Test"]);
        let root = repo.join(".grove");
        fs::create_dir_all(&root).unwrap();
        crate::tree_format::write_current_last(&root).unwrap();
        (tmp, root)
    }

    fn run_git(repo: &Path, args: &[&str]) {
        let out = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Stage everything under the grove, putting the entries in git's index — the
    /// state a real session's tree is in, and the one in which a rename that
    /// staged anything would be visible.
    fn stage_all(root: &Path) {
        run_git(root.parent().unwrap(), &["add", "-A"]);
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
        task_grow::leaf_add(root, ".", slug, Kind::Impl).unwrap()
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
            .filter(|name| name != "FORMAT")
            .collect();
        v.sort();
        v
    }

    // ---- root-init ----------------------------------------------------------

    #[test]
    fn root_init_creates_root_brief_then_first_leaf() {
        let (_t, wt) = worktree();
        let created = root_init(&wt, "plan").unwrap();
        assert_eq!(name_of(&created[0]), "BRIEF.md");
        assert_eq!(name_of(&created[1]), "01-requirements-plan-k1.md");
        assert_eq!(name_of(&created[2]), "FORMAT");
        let g = wt.join(".grove");
        assert!(g.join("BRIEF.md").is_file());
        assert!(g.join("01-requirements-plan-k1.md").is_file());
        assert_eq!(
            fs::read_to_string(g.join("FORMAT")).unwrap(),
            "session-kinds-v1\n"
        );
    }

    // fresh-grove-start-contract: the bootstrap leaf is `requirements` — the
    // human's own words are the session's only input, which is the HITL rule —
    // and the `start` path routes on that kind without a file to peek, so this
    // assertion is the contract, not a detail of the template.
    #[test]
    fn root_init_first_leaf_kind_lives_in_its_filename() {
        let (_t, wt) = worktree();
        let created = root_init(&wt, "plan").unwrap();
        assert_eq!(name_of(&created[1]), "01-requirements-plan-k1.md");
        assert!(!body(&created[1]).contains("**Kind:**"));
    }

    #[test]
    fn root_init_first_leaf_header_is_the_position_free_handle() {
        let (_t, wt) = worktree();
        let created = root_init(&wt, "plan").unwrap();
        assert!(
            body(&created[1]).starts_with("# plan-k1\n"),
            "got {:?}",
            body(&created[1])
        );
    }

    #[test]
    fn root_init_root_brief_title_is_the_grove_name() {
        let (_t, wt) = worktree();
        let created = root_init(&wt, "plan").unwrap();
        assert_eq!(
            body(&created[0]).lines().next().unwrap(),
            "# my-grove — brief"
        );
    }

    #[test]
    fn root_init_refuses_an_existing_grove() {
        let (_t, wt) = worktree();
        fs::create_dir_all(wt.join(".grove")).unwrap();
        let err = root_init(&wt, "plan").unwrap_err();
        assert!(err.to_string().contains("already exists"), "got {err}");
    }

    #[test]
    fn root_init_rejects_a_bad_slug_without_leaving_a_grove_behind() {
        let (_t, wt) = worktree();
        assert!(root_init(&wt, "Bad Slug").is_err());
        assert!(
            !wt.join(".grove").exists(),
            ".grove must not be created on a bad slug"
        );
    }

    #[test]
    fn root_init_rejects_reserved_slug() {
        let (_t, wt) = worktree();
        assert!(root_init(&wt, "BRIEF").is_err());
        assert!(!wt.join(".grove").exists());
    }

    #[test]
    fn transition_initializes_an_absent_grove_under_one_exclusive_guard() {
        let (_temporary, worktree) = worktree();
        tree_access::reset_acquisition_count();

        let outcome = transition_to_current(&worktree).unwrap();

        let grove_root = worktree.join(".grove");
        assert_eq!(outcome, CurrentTransition::RootInitialized);
        // One of grove's own, for the classification and the root's creation.
        // The library's are the scaffold's second phase and are counted by
        // `root_init_scaffolds_the_root_itself_and_the_first_leaf_through_the_library`;
        // what this holds is that classifying still takes exactly one.
        assert_eq!(tree_access::acquisition_count(), 1);
        assert_eq!(body(&grove_root.join("FORMAT")), "session-kinds-v1\n");
        assert_eq!(
            name_of(&crate::task_tree::pick(&grove_root).unwrap().unwrap()),
            "01-requirements-plan-k1.md"
        );
    }

    /// **The scaffold takes one guard of grove's and two of the library's, and
    /// the split is the point.** `ordinal_fs_tree::fs` locks the directory
    /// *containing* the root but still has to reach the root to snapshot it, so
    /// it can create neither the root nor its distinguished child; grove's own
    /// guard covers those, and is released before the library's is taken because
    /// the two `flock` one directory through different descriptions and would
    /// deadlock nested. The library's two are the `append` and the format
    /// witness the append's consumed guard could not cover.
    #[test]
    fn root_init_scaffolds_the_root_itself_and_the_first_leaf_through_the_library() {
        let (_t, wt) = worktree();
        tree_access::reset_acquisition_count();
        crate::task_tree::reset_read_count();

        let created = root_init(&wt, "plan").unwrap();

        assert_eq!(
            tree_access::acquisition_count(),
            1,
            "the root and its charter are created under exactly one guard of grove's own"
        );
        assert_eq!(
            crate::task_tree::read_count(),
            2,
            "the library appends the first leaf, then holds the tree while FORMAT lands"
        );
        assert_eq!(name_of(&created[1]), "01-requirements-plan-k1.md");
    }

    /// The leaf's `# <slug>-k<key>` handle is rendered before the library
    /// allocates the key, so `root-init` predicts the allocation exactly as every
    /// grow verb does — and `task_grow::allocated` refuses to report success on a
    /// disagreement. This is the assertion that would fail if the prediction and
    /// the library's `max + 1` ever parted.
    #[test]
    fn root_inits_first_leaf_handle_matches_the_key_the_library_allocated() {
        let (_t, wt) = worktree();
        let created = root_init(&wt, "custom-plan").unwrap();
        assert_eq!(name_of(&created[1]), "01-requirements-custom-plan-k1.md");
        assert!(body(&created[1]).starts_with("# custom-plan-k1\n"));
    }

    /// **The window between the two phases leaves exactly the tree
    /// `recover_partial_root_init_unlocked` was written for, and this is what
    /// says so.** Releasing grove's guard between them means another process can
    /// meet the root mid-scaffold — which used to happen only when one died — so
    /// the shape it meets has to be one that recovers. It is: the root, its
    /// charter, and nothing else, which is `partial_root_scaffold(ROOT_BRIEF)` in
    /// the migration transaction's own fixture.
    #[test]
    fn phase_one_leaves_the_partial_root_recovery_completes() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        {
            let _guard = tree_access::write_for_lifecycle(&wt).unwrap();
            create_root_unlocked(&wt, &grove_root).unwrap();
        }
        let mut left: Vec<String> = fs::read_dir(&grove_root)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        left.sort();
        assert_eq!(left, vec!["BRIEF.md".to_string()]);

        let outcome = transition_to_current(&wt).unwrap();

        assert_eq!(outcome, CurrentTransition::RootInitRecovered);
        assert!(grove_root.join("01-requirements-plan-k1.md").is_file());
        assert_eq!(body(&grove_root.join("FORMAT")), "session-kinds-v1\n");
    }

    /// **Completing a scaffold twice appends one leaf, not two**, which is what
    /// makes releasing the guard between the phases safe: a second process that
    /// completed the partial root first leaves the first process nothing to do.
    /// Appending unconditionally would give the tree two first leaves and no
    /// refusal, since the second would land at ordinal 2 with key 2 quite
    /// legally.
    #[test]
    fn completing_a_scaffold_twice_appends_one_leaf() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        {
            let _guard = tree_access::write_for_lifecycle(&wt).unwrap();
            create_root_unlocked(&wt, &grove_root).unwrap();
        }
        let slug = default_root_slug();

        let (first, _) = complete_scaffold(&grove_root, &slug).unwrap();
        let (second, _) = complete_scaffold(&grove_root, &slug).unwrap();

        assert_eq!(first, second);
        let leaves = fs::read_dir(&grove_root)
            .unwrap()
            .filter(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with("01-requirements")
            })
            .count();
        assert_eq!(leaves, 1);
    }

    /// **The race the released guard actually admits, sequenced.** Between
    /// `root-init`'s two phases another process can run bare `grove`, whose
    /// partial-scaffold recovery completes the tree — leaf, format witness and
    /// all. The first process then arrives with a scaffold to complete and must
    /// find there is nothing left to do, rather than appending a second first
    /// leaf or reporting a path that is not the one on disk.
    #[test]
    fn a_scaffold_completed_by_a_recovery_leaves_the_original_nothing_to_add() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        {
            let _guard = tree_access::write_for_lifecycle(&wt).unwrap();
            create_root_unlocked(&wt, &grove_root).unwrap();
        }
        assert_eq!(
            transition_to_current(&wt).unwrap(),
            CurrentTransition::RootInitRecovered
        );
        let recovered = grove_root.join("01-requirements-plan-k1.md");
        let recovered_body = body(&recovered);

        let (leaf, format) = complete_scaffold(&grove_root, &default_root_slug()).unwrap();

        assert_eq!(leaf, recovered);
        assert_eq!(body(&leaf), recovered_body);
        assert_eq!(body(&format), "session-kinds-v1\n");
        let entries = fs::read_dir(&grove_root).unwrap().count();
        assert_eq!(entries, 3, "BRIEF.md, one leaf, FORMAT — and nothing else");
    }

    /// The driver's sentinel embeds its own key in its handle, its body and the
    /// `finish-commit` command it tells the session to run — so a key the library
    /// allocated differently would leave a leaf instructing an operator to commit
    /// a handle that does not exist. The prediction is checked against the report
    /// by `task_grow::allocated`; this pins the three spellings agreeing.
    #[test]
    fn materialize_finish_writes_a_handle_that_matches_its_own_filename() {
        let (_t, wt) = worktree();
        root_init(&wt, "plan").unwrap();
        let grove_root = wt.join(".grove");
        leaf_retire(&grove_root, &grove_root.join("01-requirements-plan-k1.md")).unwrap();

        let selection = materialize_finish(&wt).unwrap();

        assert_eq!(name_of(&selection.path), "02-finish-finish-k2.md");
        assert_eq!(selection.handle, "finish-k2");
        assert_eq!(selection.kind, Kind::Finish);
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
            "01-DONE-impl-old-k4294967295.md",
            "old-k4294967295",
        );
        crate::tree_format::write_current_last(&grove_root).unwrap();

        let error = materialize_finish(&wt).unwrap_err().to_string();

        assert!(error.contains("greatest a key can be"), "got {error}");
        assert_eq!(
            fs::read_dir(&grove_root).unwrap().count(),
            3,
            "a refusal writes nothing"
        );
    }

    #[test]
    fn a_root_level_at_the_last_ordinal_refuses_the_sentinel_rather_than_wrapping() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        touch(&grove_root, "4294967295-DONE-impl-last-k1.md", "last-k1");
        crate::tree_format::write_current_last(&grove_root).unwrap();

        assert!(materialize_finish(&wt).is_err());

        assert_eq!(
            fs::read_dir(&grove_root).unwrap().count(),
            3,
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
        root_init(&wt, "plan").unwrap();
        let grove_root = wt.join(".grove");
        leaf_retire(&grove_root, &grove_root.join("01-requirements-plan-k1.md")).unwrap();
        let first = materialize_finish(&wt).unwrap();
        tree_access::reset_acquisition_count();
        crate::task_tree::reset_read_count();

        let second = materialize_finish(&wt).unwrap();

        assert_eq!(first.path, second.path);
        assert_eq!(tree_access::acquisition_count(), 0);
        assert_eq!(crate::task_tree::read_count(), 1);
    }

    #[test]
    fn transition_leaves_a_current_grove_unchanged_and_ready_for_pick() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        let leaf = touch(&grove_root, "01-impl-task-k1.md", "task-k1");
        crate::tree_format::write_current_last(&grove_root).unwrap();
        tree_access::reset_acquisition_count();

        let outcome = transition_to_current(&worktree).unwrap();

        assert_eq!(outcome, CurrentTransition::AlreadyCurrent);
        assert_eq!(tree_access::acquisition_count(), 1);
        assert_eq!(crate::task_tree::pick(&grove_root).unwrap(), Some(leaf));
    }

    #[test]
    fn transition_completes_an_exact_partial_root_scaffold_before_pick() {
        let (_temporary, worktree) = worktree();
        let created = root_init(&worktree, "plan").unwrap();
        fs::remove_file(&created[2]).unwrap();
        tree_access::reset_acquisition_count();

        let outcome = transition_to_current(&worktree).unwrap();

        let grove_root = worktree.join(".grove");
        assert_eq!(outcome, CurrentTransition::RootInitRecovered);
        assert_eq!(tree_access::acquisition_count(), 1);
        assert_eq!(body(&grove_root.join("FORMAT")), "session-kinds-v1\n");
        assert_eq!(
            name_of(&crate::task_tree::pick(&grove_root).unwrap().unwrap()),
            "01-requirements-plan-k1.md"
        );
    }

    #[test]
    fn transition_migrates_a_legacy_leaf_then_exposes_the_current_pick() {
        let (_temporary, worktree) = worktree();
        run_git(&worktree, &["init", "-q"]);
        run_git(&worktree, &["config", "user.email", "t@example.com"]);
        run_git(&worktree, &["config", "user.name", "Test"]);
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        touch_body(
            &grove_root,
            "01-task-k1.md",
            "# task-k1\n\n**Kind:** impl\n\n## Goal\nShip.\n",
        );
        tree_access::reset_acquisition_count();

        let outcome = transition_to_current(&worktree).unwrap();

        let migrated = grove_root.join("01-impl-task-k1.md");
        assert_eq!(outcome, CurrentTransition::Migrated);
        assert_eq!(tree_access::acquisition_count(), 1);
        assert!(!grove_root.join("01-task-k1.md").exists());
        assert!(!body(&migrated).contains("**Kind:**"));
        assert_eq!(crate::task_tree::pick(&grove_root).unwrap(), Some(migrated));
        let subject = Command::new("git")
            .args(["log", "-1", "--format=%s"])
            .current_dir(&worktree)
            .output()
            .unwrap();
        assert!(subject.status.success());
        assert_eq!(
            String::from_utf8(subject.stdout).unwrap(),
            "grove(my-grove): migrate task tree to session-kind filenames\n"
        );
    }

    #[test]
    fn transition_refuses_an_unknown_format_before_mutation_or_commit() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        let leaf = touch(&grove_root, "01-impl-task-k1.md", "task-k1");
        fs::write(grove_root.join("FORMAT"), "session-kinds-v99\n").unwrap();
        let leaf_before = body(&leaf);
        tree_access::reset_acquisition_count();

        let error = transition_to_current(&worktree).unwrap_err();

        assert!(
            format!("{error:#}").contains("unsupported Grove tree format"),
            "unexpected error: {error:#}"
        );
        assert_eq!(tree_access::acquisition_count(), 1);
        assert_eq!(body(&leaf), leaf_before);
        assert_eq!(body(&grove_root.join("FORMAT")), "session-kinds-v99\n");
    }

    #[test]
    fn transition_checks_an_unknown_format_before_recovering_a_pending_witness() {
        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        fs::write(grove_root.join("FORMAT"), "session-kinds-v99\n").unwrap();
        let witness = grove_root.join(crate::tree_access::MIGRATION_TRANSACTION);
        fs::create_dir(&witness).unwrap();

        let error = transition_to_current(&worktree).unwrap_err();

        assert!(
            format!("{error:#}").contains("unsupported Grove tree format"),
            "unexpected error: {error:#}"
        );
        assert!(
            witness.is_dir(),
            "unknown format must prevent recovery mutation"
        );
    }

    #[cfg(unix)]
    #[test]
    fn transition_refuses_a_dangling_pending_witness_in_a_current_tree() {
        use std::os::unix::fs::symlink;

        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");
        fs::write(grove_root.join("FORMAT"), "session-kinds-v1\n").unwrap();
        let witness = grove_root.join(crate::tree_access::MIGRATION_TRANSACTION);
        symlink(grove_root.join("missing-witness"), &witness).unwrap();

        let error = transition_to_current(&worktree).unwrap_err();

        assert!(
            format!("{error:#}").contains("migration witness is not a directory"),
            "unexpected error: {error:#}"
        );
        assert!(fs::symlink_metadata(witness)
            .unwrap()
            .file_type()
            .is_symlink());
    }

    #[cfg(unix)]
    #[test]
    fn transition_does_not_classify_a_dangling_grove_symlink_as_absent() {
        use std::os::unix::fs::symlink;

        let (_temporary, worktree) = worktree();
        let grove_root = worktree.join(".grove");
        symlink(worktree.join("missing-grove"), &grove_root).unwrap();

        let error = transition_to_current(&worktree).unwrap_err();

        assert!(
            format!("{error:#}").contains("not a directory"),
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        stage_all(&g);
        let (brief, _child) = leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "step",
            Some(Kind::Impl),
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
            !files.contains(&"02-impl-build-k3.md".to_string()),
            "old leaf file gone"
        );
        assert!(g.join("02-build-k3").is_dir());
    }

    #[test]
    fn decompose_seeds_brief_from_leaf_body_and_appends_brief_suffix() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-impl-build-k3.md",
            "# build-k3\n\n## Goal\nship it\n",
        );
        stage_all(&g);
        let (brief, _child) = leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "step",
            Some(Kind::Impl),
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        stage_all(&g);
        let (_brief, child) = leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "step",
            Some(Kind::Impl),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-impl-step-k4.md");
        assert_eq!(name_of(child.parent().unwrap()), "02-build-k3");
        assert!(g.join("02-build-k3").join("01-impl-step-k4.md").is_file());
    }

    #[test]
    fn decompose_first_child_header_is_the_handle_and_filename_carries_the_kind() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        stage_all(&g);
        let (_brief, child) = leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "step",
            Some(Kind::Impl),
        )
        .unwrap();
        let text = body(&child);
        assert!(text.starts_with("# step-k4\n"), "got {text:?}");
        assert_eq!(name_of(&child), "01-impl-step-k4.md");
        assert!(!text.contains("**Kind:**"), "got {text:?}");
    }

    #[test]
    fn decompose_first_child_can_be_a_planning_task() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        stage_all(&g);
        let (_brief, child) = leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "design",
            Some(Kind::Planning),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-planning-design-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

    #[test]
    fn decompose_with_no_override_inherits_the_parent_leafs_own_kind() {
        // task-kind-taxonomy: `leaf-decompose` gives the first child the leaf
        // being decomposed's own kind when `--kind` is not given.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-research-a-build-k3.md",
            "# build-k3\n\n**Kind:** impl\n",
        );
        stage_all(&g);
        let (_brief, child) =
            leaf_decompose(&g, Path::new("02-research-a-build-k3.md"), "step", None).unwrap();
        assert_eq!(name_of(&child), "01-research-a-step-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

    #[test]
    fn decompose_override_wins_over_the_parent_leafs_kind() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-research-a-build-k3.md",
            "# build-k3\n\n**Kind:** impl\n",
        );
        stage_all(&g);
        let (_brief, child) = leaf_decompose(
            &g,
            Path::new("02-research-a-build-k3.md"),
            "step",
            Some(Kind::ReviewImpl),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-review-impl-step-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

    #[test]
    fn decompose_a_nested_leaf_preserves_key_and_grows_a_grandchild() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let build = mknode(&g, "02-build-k1", "build-k1");
        touch(&build, "02-impl-mid-k5.md", "mid-k5");
        stage_all(&g);
        let (brief, child) = leaf_decompose(
            &g,
            &build.join("02-impl-mid-k5.md"),
            "first",
            Some(Kind::Impl),
        )
        .unwrap();
        assert_eq!(
            name_of(brief.parent().unwrap()),
            "02-mid-k5",
            "key 5 preserved"
        );
        assert_eq!(
            name_of(&child),
            "01-impl-first-k6.md",
            "fresh key max(1,5)+1 = 6"
        );
        assert_eq!(name_of(child.parent().unwrap()), "02-mid-k5");
    }

    #[test]
    fn decompose_refuses_a_brief() {
        let (_t, g) = git_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        stage_all(&g);
        let err = leaf_decompose(&g, &node.join("BRIEF.md"), "x", Some(Kind::Impl)).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_node_directory() {
        let (_t, g) = git_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        stage_all(&g);
        let err = leaf_decompose(&g, &node, "x", Some(Kind::Impl)).unwrap_err();
        assert!(err.to_string().contains("node"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_done_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-DONE-impl-build-k3.md", "build-k3");
        stage_all(&g);
        let err = leaf_decompose(
            &g,
            Path::new("02-DONE-impl-build-k3.md"),
            "x",
            Some(Kind::Impl),
        )
        .unwrap_err();
        assert!(
            err.to_string().to_lowercase().contains("done") || err.to_string().contains("retired"),
            "got {err}"
        );
    }

    #[test]
    fn decompose_refuses_an_abandoned_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-ABANDONED-impl-build-k3.md", "build-k3");
        stage_all(&g);
        let err = leaf_decompose(
            &g,
            Path::new("02-ABANDONED-impl-build-k3.md"),
            "x",
            Some(Kind::Impl),
        )
        .unwrap_err();
        assert!(err.to_string().contains("abandoned"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_foreign_file() {
        let (_t, g) = git_grove();
        touch(&g, "README.md", "readme");
        stage_all(&g);
        let err = leaf_decompose(&g, Path::new("README.md"), "x", Some(Kind::Impl)).unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

    #[test]
    fn decompose_rejects_a_bad_child_slug_without_touching_the_leaf() {
        // Atomicity: the child slug is validated BEFORE the rename, so a bad slug
        // leaves the leaf un-decomposed (no half-built node directory).
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        stage_all(&g);
        assert!(leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "Bad Slug",
            Some(Kind::Impl)
        )
        .is_err());
        let files = list(&g);
        assert!(
            files.contains(&"02-impl-build-k3.md".to_string()),
            "leaf untouched"
        );
        assert!(
            !files.contains(&"02-build-k3".to_string()),
            "no half-built node dir"
        );
    }

    #[test]
    fn decompose_accepts_an_absolute_path() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        stage_all(&g);
        let abs = g.join("02-impl-build-k3.md");
        let (brief, _child) = leaf_decompose(&g, &abs, "step", Some(Kind::Impl)).unwrap();
        assert_eq!(name_of(brief.parent().unwrap()), "02-build-k3");
    }

    #[test]
    fn decompose_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err = leaf_decompose(
            &missing,
            Path::new("02-impl-build-k3.md"),
            "x",
            Some(Kind::Impl),
        )
        .unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        stage_all(&g);
        crate::task_tree::reset_read_count();

        leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "step",
            Some(Kind::Impl),
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        touch(&g, "01-DONE-impl-a-k1.md", "a-k1");
        stage_all(&g);

        let err =
            leaf_decompose(&g, Path::new("01-impl-a-k1.md"), "x", Some(Kind::Impl)).unwrap_err();

        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 1"),
            "got {err}"
        );
        assert!(
            g.join("01-impl-a-k1.md").is_file() && !g.join("01-a-k1").exists(),
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        mknode(&g, "02-build-k3", "build-k3");
        stage_all(&g);

        let err = leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "step",
            Some(Kind::Impl),
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k3.md", "build-k3");
        fs::create_dir(g.join("02-build-k3")).unwrap();
        stage_all(&g);

        let err = leaf_decompose(
            &g,
            Path::new("02-impl-build-k3.md"),
            "step",
            Some(Kind::Impl),
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
            g.join("02-impl-build-k3.md").is_file(),
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-build-k4294967295.md", "build-k4294967295");
        stage_all(&g);

        let err = leaf_decompose(
            &g,
            Path::new("02-impl-build-k4294967295.md"),
            "step",
            Some(Kind::Impl),
        )
        .unwrap_err();

        assert!(
            err.to_string().contains("greatest a key can be"),
            "got {err}"
        );
        assert!(
            g.join("02-impl-build-k4294967295.md").is_file()
                && !g.join("02-build-k4294967295").exists(),
            "a refusal writes nothing"
        );
    }

    #[test]
    fn no_promotion_refusal_reaches_an_operator_from_an_ordinary_argument() {
        // The three refusals `promote` owns, asserted unreachable rather than
        // described. `PromoteNoDistinguished` is discharged by the domain itself
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

        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-DONE-impl-done-k1.md", "done-k1");
        touch(&g, "02-ABANDONED-impl-gone-k2.md", "gone-k2");
        touch(&g, "03-finish-wrap-k3.md", "wrap-k3");
        let node = mknode(&g, "04-build-k4", "build-k4");
        stage_all(&g);

        for argument in [
            g.as_path(),
            &g.join("BRIEF.md"),
            &g.join("01-DONE-impl-done-k1.md"),
            &g.join("02-ABANDONED-impl-gone-k2.md"),
            &g.join("03-finish-wrap-k3.md"),
            node.as_path(),
            &node.join("BRIEF.md"),
        ] {
            let err = leaf_decompose(&g, argument, "step", Some(Kind::Impl))
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let done = leaf_retire(&g, Path::new("02-impl-add-k4.md")).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl-add-k4.md");
        let files = list(&g);
        assert!(files.contains(&"02-DONE-impl-add-k4.md".to_string()));
        assert!(
            !files.contains(&"02-impl-add-k4.md".to_string()),
            "old name gone"
        );
    }

    #[test]
    fn retire_does_not_rewrite_the_header_or_body() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-impl-add-k4.md", "# add-k4\n\nbody\n");
        stage_all(&g);
        let done = leaf_retire(&g, Path::new("02-impl-add-k4.md")).unwrap();
        assert_eq!(body(&done), "# add-k4\n\nbody\n", "content byte-identical");
    }

    #[test]
    fn retire_works_on_a_nested_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "02-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let done = leaf_retire(&g, &design.join("02-impl-add-k4.md")).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl-add-k4.md");
        assert_eq!(name_of(done.parent().unwrap()), "01-design-k1");
    }

    #[test]
    fn retire_refuses_a_node_directory() {
        let (_t, g) = git_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        stage_all(&g);
        let err = leaf_retire(&g, &node).unwrap_err();
        assert!(err.to_string().contains("node"), "got {err}");
    }

    #[test]
    fn retire_refuses_a_node_brief() {
        let (_t, g) = git_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        stage_all(&g);
        let err = leaf_retire(&g, &node.join("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn retire_refuses_the_root_brief() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn retire_refuses_an_already_done_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "02-DONE-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("02-DONE-impl-add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("already"), "got {err}");
    }

    #[test]
    fn retire_refuses_an_abandoned_leaf() {
        // A missing flag must degrade to something harmless, never to the
        // opposite outcome (pruning): retiring an abandoned leaf would
        // silently assert the rejected work was finished.
        let (_t, g) = git_grove();
        touch(&g, "02-ABANDONED-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("02-ABANDONED-impl-add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("abandoned"), "got {err}");
    }

    #[test]
    fn retire_refuses_a_foreign_file() {
        let (_t, g) = git_grove();
        touch(&g, "README.md", "readme");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("README.md")).unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

    #[test]
    fn retire_accepts_an_absolute_path() {
        let (_t, g) = git_grove();
        touch(&g, "02-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let abs = g.join("02-impl-add-k4.md");
        let done = leaf_retire(&g, &abs).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl-add-k4.md");
    }

    #[test]
    fn retire_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err = leaf_retire(&missing, Path::new("02-impl-add-k4.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- lifecycle over untracked leaves (issue #3's root cause) -------------
    //
    // Issue #3's defect, in the lifecycle verbs: a leaf grown this session is
    // untracked until the enclosing task commits, and the `git mv` these verbs
    // used to reach for had no index entry to move. The verbs now rename through
    // `ordinal-fs-tree`, which never consults git at all
    // (`docs/adr/grove-does-not-stage-its-own-renames.md`), so these cases can no
    // longer fail that way — they are kept because the fixtures are the ones a
    // real session produces, and a verb that grew a tracked-only path would fail
    // them again.

    #[test]
    fn retire_an_untracked_leaf_added_this_session() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "ship");
        // No stage_all: the grow verb leaves it untracked, by design.
        let done = leaf_retire(&g, &leaf).unwrap();
        assert_eq!(name_of(&done), "01-DONE-impl-ship-k1.md");
        assert!(
            done.is_file(),
            "the retired leaf is on disk under its DONE name"
        );
        assert!(!leaf.exists(), "the live name is gone");
    }

    #[test]
    fn decompose_an_untracked_leaf_added_this_session() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "big");
        // "The current item proving bigger" — the canonical mid-session decompose.
        let (brief, child) = leaf_decompose(&g, &leaf, "first", None).unwrap();
        assert_eq!(name_of(&brief), "BRIEF.md");
        assert_eq!(name_of(&child), "01-impl-first-k2.md");
        assert!(g.join("01-big-k1").is_dir(), "the leaf became a node dir");
        assert!(
            !leaf.exists(),
            "the leaf file is gone (it became the BRIEF)"
        );
    }

    #[test]
    fn prune_an_untracked_leaf_added_this_session() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "dead");
        let result = leaf_prune(&g, &leaf).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "01-ABANDONED-impl-dead-k1.md");
        assert!(!leaf.exists(), "the live name is gone");
    }

    // ---- leaf-prune (pruning) ------------------------------------------

    #[test]
    fn prune_leaf_adds_abandoned_infix_keeping_position_and_key() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let result = leaf_prune(&g, Path::new("02-impl-add-k4.md")).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl-add-k4.md");
        assert!(result.left_done.is_empty());
        let files = list(&g);
        assert!(files.contains(&"02-ABANDONED-impl-add-k4.md".to_string()));
        assert!(
            !files.contains(&"02-impl-add-k4.md".to_string()),
            "old name gone"
        );
    }

    #[test]
    fn prune_leaf_does_not_rewrite_the_header_or_body() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-impl-add-k4.md", "# add-k4\n\nbody\n");
        stage_all(&g);
        let result = leaf_prune(&g, Path::new("02-impl-add-k4.md")).unwrap();
        assert_eq!(
            body(&result.marked[0]),
            "# add-k4\n\nbody\n",
            "content byte-identical"
        );
    }

    #[test]
    fn prune_leaf_works_on_a_nested_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "02-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let result = leaf_prune(&g, &design.join("02-impl-add-k4.md")).unwrap();
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl-add-k4.md");
        assert_eq!(name_of(result.marked[0].parent().unwrap()), "01-design-k1");
    }

    #[test]
    fn prune_leaf_refuses_a_node_brief() {
        let (_t, g) = git_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        stage_all(&g);
        let err = leaf_prune(&g, &node.join("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_the_root_brief() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        let err = leaf_prune(&g, Path::new("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_an_already_done_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "02-DONE-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let err = leaf_prune(&g, Path::new("02-DONE-impl-add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("DONE"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_an_already_abandoned_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "02-ABANDONED-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let err = leaf_prune(&g, Path::new("02-ABANDONED-impl-add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("already"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_a_foreign_file() {
        let (_t, g) = git_grove();
        touch(&g, "README.md", "readme");
        stage_all(&g);
        let err = leaf_prune(&g, Path::new("README.md")).unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

    #[test]
    fn prune_leaf_accepts_an_absolute_path() {
        let (_t, g) = git_grove();
        touch(&g, "02-impl-add-k4.md", "add-k4");
        stage_all(&g);
        let abs = g.join("02-impl-add-k4.md");
        let result = leaf_prune(&g, &abs).unwrap();
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl-add-k4.md");
    }

    #[test]
    fn prune_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err = leaf_prune(&missing, Path::new("02-impl-add-k4.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- leaf-prune on a node: bulk arity (pruning) -------------------

    #[test]
    fn prune_node_marks_every_live_leaf_in_the_subtree() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl-a-k3.md", "a-k3");
        touch(&node, "02-impl-b-k4.md", "b-k4");
        stage_all(&g);
        let result = leaf_prune(&g, &node).unwrap();
        let names: Vec<String> = result.marked.iter().map(|p| name_of(p)).collect();
        assert_eq!(
            names,
            vec!["01-ABANDONED-impl-a-k3.md", "02-ABANDONED-impl-b-k4.md"]
        );
        assert!(result.left_done.is_empty());
    }

    #[test]
    fn prune_node_leaves_done_leaves_untouched() {
        // That work really was done — a bulk abandon does not retroactively
        // un-finish it.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-impl-a-k3.md", "a-k3");
        touch(&node, "02-impl-b-k4.md", "b-k4");
        stage_all(&g);
        let result = leaf_prune(&g, &node).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl-b-k4.md");
        assert_eq!(result.left_done.len(), 1);
        assert_eq!(name_of(&result.left_done[0]), "01-DONE-impl-a-k3.md");
        // The DONE leaf's name (and so its position and key) is untouched.
        assert!(node.join("01-DONE-impl-a-k3.md").is_file());
    }

    #[test]
    fn prune_node_recurses_into_a_grandchild_node() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let outer = mknode(&g, "01-outer-k1", "outer-k1");
        let inner = mknode(&outer, "01-inner-k2", "inner-k2");
        touch(&inner, "01-impl-deep-k3.md", "deep-k3");
        stage_all(&g);
        let result = leaf_prune(&g, &outer).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "01-ABANDONED-impl-deep-k3.md");
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl-a-k3.md", "a-k3");
        touch(&node, "02-impl-b-k4.md", "b-k4");
        stage_all(&g); // a and b are tracked
        touch(&node, "03-impl-c-k5.md", "c-k5"); // c is not

        let result = leaf_prune(&g, &node).unwrap();

        assert_eq!(result.marked.len(), 3, "every live leaf marked");
        let names = list(&node);
        for expected in [
            "01-ABANDONED-impl-a-k3.md",
            "02-ABANDONED-impl-b-k4.md",
            "03-ABANDONED-impl-c-k5.md",
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl-a-k3.md", "a-k3");
        touch(&node, "02-impl-b-k4.md", "b-k4");
        touch(&node, "03-impl-c-k5.md", "c-k5");
        touch(&node, "03-ABANDONED-impl-c-k5.md", "c-k5");
        stage_all(&g);

        let err = leaf_prune(&g, &node).unwrap_err();
        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 5"),
            "got {err}"
        );

        // Nothing was mutated: every live name is untouched, none newly marked.
        let names = list(&node);
        assert!(
            names.contains(&"01-impl-a-k3.md".to_string()),
            "got {names:?}"
        );
        assert!(
            names.contains(&"02-impl-b-k4.md".to_string()),
            "got {names:?}"
        );
        assert!(
            names.contains(&"03-impl-c-k5.md".to_string()),
            "got {names:?}"
        );
        assert!(
            !names.contains(&"01-ABANDONED-impl-a-k3.md".to_string())
                && !names.contains(&"02-ABANDONED-impl-b-k4.md".to_string()),
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        touch(&g, "01-DONE-impl-a-k1.md", "a-k1");
        stage_all(&g);

        let err = leaf_retire(&g, Path::new("01-impl-a-k1.md")).unwrap_err();

        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 1"),
            "got {err}"
        );
        assert!(
            g.join("01-impl-a-k1.md").is_file(),
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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl-a-k3.md", "a-k3");
        touch(&node, "02-impl-b-k4.md", "b-k4");
        touch(&node, "03-DONE-impl-c-k5.md", "c-k5");
        touch(&node, "04-impl-d-k6.md", "d-k6");
        stage_all(&g);
        crate::task_tree::reset_read_count();

        let result = leaf_prune(&g, &node).unwrap();

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
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-impl-a-k3.md", "a-k3");
        stage_all(&g);
        let result = leaf_prune(&g, &node).unwrap();
        assert!(result.marked.is_empty());
        assert_eq!(result.left_done.len(), 1);
    }

    #[test]
    fn prune_refuses_the_grove_root() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        stage_all(&g);
        let err = leaf_prune(&g, &g).unwrap_err();
        assert!(err.to_string().contains("grove root"), "got {err}");
        // Nothing was touched.
        assert!(g.join("01-impl-a-k1.md").is_file());
    }

    #[test]
    fn prune_refuses_the_grove_root_given_as_a_relative_dot_path() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        let err = leaf_prune(&g, Path::new(".")).unwrap_err();
        assert!(err.to_string().contains("grove root"), "got {err}");
    }
}

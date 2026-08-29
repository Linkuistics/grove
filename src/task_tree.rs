// Grove's **reading surface**, expressed through `ordinal-fs-tree` (gh issue
// #13, increment 2, the first leaf of the *migrate* stage).
//
// `pick`, `select`, `brief-chain`, `kind` and `resolve` all read one
// [`Snapshot`](ordinal_fs_tree::Snapshot) taken under the library's shared lock,
// and their semantics are unchanged: first live leaf in walk order, ancestor
// briefs root→leaf, reference-by-permanent-key. What changes is who owns the
// walk. The path-walking reader that used to own it survived only as long as
// there were verbs still under grove's own exclusive guard — that guard and the
// library's cannot be nested (both `flock` the directory containing the tree
// root, and two open file descriptions on one directory do not share a lock) —
// and `sweep-k37` deleted it once the last of them had moved across.
//
// # Path construction lives here, and in exactly one place
//
// The library's reading surface returns **no paths** — `cli-k16` refused to add
// a `path()` to the algebra and said why (`docs/ordinal-fs-tree/CLI.md`, *What
// `cli-k16` should watch*) — so a consumer builds them: the caller's own
// spelling of the root, then each ancestor node's rendered name, then the
// entry's. [`entry_path`] is that one place, and every later flip leaf uses it.
// It is safe without a check because every name a snapshot admits has already
// been checked to render as one path component (`Error::NameIsNotOneComponent`).
//
// Nothing here canonicalises for **output**. The library deliberately never
// does: on macOS `/var` and `/private/var` name the same inode, so
// canonicalising would make the mere presence of a lock rewrite every path a
// read verb returns. Canonicalisation appears once, in [`leaf_entry`], and only
// to *compare* a caller's spelling of a leaf against the tree's — which is what
// the path-walking reader did too.
//
// # Refusal precedence is grove's, and the halt is the library's
//
// The library halts the whole tree on a name grove recognises and refuses,
// wherever it sits. That is the decision, and it is taken under the lock. But
// the library can only say *this filename is wrong*, and an absent root or a
// tree held by the finish transaction are conditions grove states in its own
// words. So [`restate`] re-states a *failed* read in the order grove owes its
// operator: root, then a pending transaction, then the library's own message.
// Only the wording is chosen here; the refusal itself already happened.

use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use ordinal_fs_tree::{Entry, EntryName, Error, Found, Key, Snapshot, Sought, Verdict};

use crate::leaf::Kind;
use crate::task_name::{Outcome, Parts, TaskName};

/// The task tree, read once under the library's shared lock.
///
/// Derefs to its `Snapshot`, so the reading operations are called on it
/// directly. Note that `Tree::root` is the **path** the caller spelled, while
/// `Snapshot::root` is the root *level* — the inherent method wins, and both are
/// wanted here.
pub type Tree = ordinal_fs_tree::fs::ReadGuard<TaskName>;

#[cfg(test)]
thread_local! {
    static READ_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Read the task tree under a shared lock.
///
/// One lock and one snapshot, and every read verb below takes exactly one of
/// these.
pub fn read(grove_root: &Path) -> Result<Tree> {
    #[cfg(test)]
    READ_COUNT.with(|count| count.set(count.get() + 1));

    announce_contention(grove_root, libc::LOCK_SH);
    ordinal_fs_tree::fs::read::<TaskName>(grove_root).map_err(|error| restate(grove_root, &error))
}

/// The task tree, read once under the library's **exclusive** lock — the
/// surface every mutation is on.
///
/// A mutating method *consumes* this guard (`crates/ordinal-fs-tree/src/fs/mod.rs`,
/// *A mutation consumes its guard*), so one guard is one operation. That is the
/// whole of what a bulk verb has to work around; `tree_lifecycle::leaf_prune`
/// carries the consequence and `docs/adr/bulk-marks-are-not-atomic.md` records
/// what Grove chose to do about it.
pub type TreeWrite = ordinal_fs_tree::fs::WriteGuard<TaskName>;

/// Read the task tree under an exclusive lock, announcing contention first.
///
/// The write-side twin of [`read`].
pub fn write(grove_root: &Path) -> Result<TreeWrite> {
    announce_contention(grove_root, libc::LOCK_EX);
    reopen_write(grove_root)
}

/// [`write`] without the waiting diagnostic, for the second and later guards of
/// one verb.
///
/// A bulk mark is *N* rewrites under *N* guards, and every one of them would
/// otherwise probe and print. The verb announces once, through [`write`], and
/// takes the rest through here: the diagnostic is about the command's wait, not
/// about each lock it happens to need.
pub(crate) fn reopen_write(grove_root: &Path) -> Result<TreeWrite> {
    #[cfg(test)]
    READ_COUNT.with(|count| count.set(count.get() + 1));

    ordinal_fs_tree::fs::write::<TaskName>(grove_root).map_err(|error| restate(grove_root, &error))
}

/// Turn a library error raised by a *mutation* into Grove's own.
///
/// Not [`restate`]: that one re-states a failed **read**, whose precedence
/// question is which of several conditions to name. A mutation's guard has
/// already read the tree successfully, so what arrives here is a `Refusal`, a
/// failed apply, or an unwind — and every one of those is printed unchanged
/// (`docs/ARCHITECTURE.md#library-refusals`, clause 3).
pub(crate) fn raised(error: Error<TaskName>) -> anyhow::Error {
    anyhow!("{error}")
}

/// Say that this process is waiting, before it blocks.
///
/// The library's locking is invisible in its interface — no try-variant, no
/// timeout, and `read` simply blocks — which is the architecture's own decision
/// and not something to work around. But grove has always told an operator why
/// it appears to have hung, and losing that is a user-visible regression in what
/// the node brief calls a pure refactor. So the diagnostic is bought outside the
/// library: one non-blocking acquisition of the same mode on the same directory
/// the library will lock, released immediately. `mode` is the caller's own —
/// [`libc::LOCK_SH`] before a read and [`libc::LOCK_EX`] before a write, because
/// a shared probe taken before an exclusive acquisition succeeds while another
/// reader holds the tree and would swallow the very message it exists to print.
///
/// Best-effort by construction. Between releasing this probe and the library
/// taking its own lock a contender can arrive, and then this process blocks
/// silently; the probe is a diagnostic and never a decision, so that window
/// costs a message and nothing else. Everything that can go wrong with it —
/// including the directory not existing — is silence, because the library is
/// about to report the same condition properly.
fn announce_contention(grove_root: &Path, mode: libc::c_int) {
    // `<root>/..` and not `Path::parent`: the same spelling the library locks,
    // resolved by the kernel, so the probe asks about the directory the library
    // will actually contend for rather than a lexical parent of the string.
    let Ok(handle) = File::open(grove_root.join("..")) else {
        return;
    };
    let descriptor = handle.as_raw_fd();
    // SAFETY: `descriptor` is open for the whole call — `handle` owns it and
    // outlives both `flock`s — and `flock` touches nothing else.
    if unsafe { libc::flock(descriptor, mode | libc::LOCK_NB) } == 0 {
        unsafe { libc::flock(descriptor, libc::LOCK_UN) };
        return;
    }
    let error = std::io::Error::last_os_error();
    if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN)
    {
        eprintln!("waiting for active Grove tree operation");
    }
}

/// Re-state a failed read in the order grove owes its operator.
///
/// Each clause below is a condition grove states in its own words, tried in the
/// precedence the path-walking reader had. The checks are unlocked,
/// deliberately: the decision to refuse was already taken under the lock, and
/// only the wording is chosen here.
fn restate(grove_root: &Path, error: &Error<TaskName>) -> anyhow::Error {
    if !grove_root.is_dir() {
        return anyhow!("grove root not found: {}", grove_root.display());
    }
    match error {
        // The domain's own advice *is* the message (`Error`'s `Display` says
        // why), and it names the offending filename but not where in the tree it
        // sits. A tree deep enough to have two levels is a tree where that
        // matters, so the path is appended rather than the message rewritten.
        Error::Malformed { path, .. } | Error::Reserved { path, .. } => {
            anyhow!("{error} ({})", path.display())
        }
        _ => anyhow!("{error}"),
    }
}

/// Where an entry of a snapshot lives: the caller's spelling of the root, then
/// every containing node's name, then its own.
///
/// **The one place grove builds a path out of the tree.** The library returns
/// none, and `cli-k16` refused to answer that by adding a `path()` to the
/// algebra, so this is the consumer's half of that decision — and every later
/// flip leaf calls it rather than writing a second one.
#[must_use]
pub fn entry_path(root: &Path, entry: Entry<'_, TaskName>) -> PathBuf {
    let mut path = root.to_path_buf();
    for container in entry.ancestors() {
        if let Some(node) = container.entry() {
            path.push(node.name().to_string());
        }
    }
    path.push(entry.name().to_string());
    path
}

/// What a caller's path argument names in the tree.
///
/// The grove root is a case of its own rather than an error, because each verb
/// words its refusal of it differently — `leaf-prune`'s is about abandoning a
/// whole workstream, `leaf-retire`'s about the argument not being a leaf — and
/// the resolver has no business choosing between them.
pub enum Target<'a> {
    /// The grove root itself. Not an entry: it carries no name to rewrite.
    Root,
    /// An entry of the snapshot — a task file, a node directory, or a `BRIEF.md`.
    Entry(Entry<'a, TaskName>),
}

/// The snapshot entry a caller's path argument names: absolute, or relative to
/// the grove root, and a leaf file or a node directory alike.
///
/// **Clause 1 of `docs/ARCHITECTURE.md#library-refusals`** — resolve the
/// argument to an entry, so the verb can then call the library *by key* against
/// the same snapshot the operation plans from. Grove's path grammar is wider
/// than a key and the library has no counterpart for *no such path*, so
/// resolution is Grove's and so is every message below.
///
/// Canonicalised to **compare** and never to report, exactly as `leaf_entry`
/// does: two spellings of one path name one entry, and the paths this module
/// returns are still built from the caller's own spelling of the root.
pub fn target<'a>(
    root: &Path,
    snapshot: &'a Snapshot<TaskName>,
    path: &Path,
) -> Result<Target<'a>> {
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    let resolved = candidate
        .canonicalize()
        .with_context(|| format!("resolving path {}", candidate.display()))?;
    let root_real = root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", root.display()))?;
    if resolved == root_real {
        return Ok(Target::Root);
    }
    if !resolved.starts_with(&root_real) {
        bail!(
            "path {} is not under grove root {}",
            resolved.display(),
            root_real.display()
        );
    }
    let name = resolved
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("path {} has no UTF-8 filename", resolved.display()))?;
    for entry in snapshot.walk() {
        if entry.name().to_string() != name {
            continue;
        }
        if entry_path(root, entry).canonicalize().ok().as_deref() == Some(resolved.as_path()) {
            return Ok(Target::Entry(entry));
        }
    }
    Err(unreachable_by_any_walk(&candidate, &resolved, name))
}

/// Why a path that exists under the grove root names no entry of the snapshot.
///
/// Three reasons, and they are different things to whoever hit one. The
/// grammar answers the first two — a name it disclaims is not Grove's at all,
/// and a name it refuses carries its own recovery advice — so only the third
/// needs a sentence of Grove's own: the entry is task-shaped and some level
/// above it is not, which is exactly the subtree no walk descends into.
fn unreachable_by_any_walk(candidate: &Path, resolved: &Path, name: &str) -> anyhow::Error {
    let found = if resolved.is_dir() {
        Found::Dir
    } else if resolved.is_file() {
        Found::File
    } else {
        Found::Other
    };
    match TaskName::parse(name, found) {
        Verdict::Malformed(error) | Verdict::Reserved(error) => anyhow!("{error}"),
        Verdict::Foreign => anyhow!(
            "not a Grove leaf or node directory: {}",
            candidate.display()
        ),
        Verdict::Entry(_) => anyhow!(
            "Grove entry {} is not in the task tree: every level above it must be \
             a node directory named NN-<slug>-k<key>",
            candidate.display()
        ),
    }
}

/// The key by which the library can address this entry, or Grove's refusal that
/// it cannot.
///
/// **Clause 1 is sound only while keys are unique tree-wide.** The library
/// states uniqueness as an obligation on the domain and cannot enforce it: a
/// hand edit or a failed rollback can put two entries under one key, and
/// [`Snapshot::by_key`](ordinal_fs_tree::Snapshot::by_key) then answers with
/// whichever the walk reaches first. Walk *order* is unmodelled — the node
/// brief records `by_key`'s tie-break on a duplicate-key tree as a known miss of
/// `structure.als` — so which twin an operation lands on is not a fact anything
/// establishes. This is the consumer-side half of that miss, and it is not
/// theoretical: without it, `leaf-retire` aimed by path at one twin silently
/// marks the other and reports success.
///
/// Grove's own precondition, therefore, and not a second wording of anything the
/// library says (`docs/ARCHITECTURE.md#library-refusals`, clauses 2 and 3): the
/// library has no notion of *the entry the operator named*, which is the whole
/// of what is ambiguous here.
///
/// One walk per call, which is quadratic over a bulk mark. A `.grove/` tree is
/// tens of entries and this runs once per marked leaf, so the simpler shape is
/// kept deliberately.
pub fn addressable_key(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    entry: &Entry<'_, TaskName>,
) -> Result<Key> {
    let name = entry.name();
    let triple = entry
        .triple()
        .with_context(|| format!("{name} carries no key of its own"))?;
    let twins: Vec<Entry<'_, TaskName>> = snapshot
        .walk()
        .filter(|other| other.key() == Some(triple.key))
        .collect();
    if twins.len() > 1 {
        let paths = twins
            .iter()
            .map(|other| entry_path(root, *other).display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        if let Some(node) = interrupted_promotion(&twins) {
            bail!(
                "a node directory and a task file share position {} and key {}, and \
                 the directory holds no BRIEF.md: {}. That is an interrupted \
                 `leaf-decompose` — the promotion created {} and then failed to move \
                 the leaf into it, and its rollback failed too. Removing either half \
                 resolves it: delete the empty directory to keep the leaf, or move \
                 the leaf in as its BRIEF.md to keep the node.",
                node.ordinal()
                    .map_or_else(|| "?".to_string(), |ordinal| ordinal.get().to_string()),
                triple.key,
                paths,
                node.name(),
            );
        }
        bail!(
            "two entries in this tree carry key {}, so naming one of them names \
             both: {}. A key is assigned once and never reused, so this is a hand \
             edit or a rollback that failed — give one of them a fresh key before \
             operating on either.",
            triple.key,
            paths
        );
    }
    Ok(triple.key)
}

/// The node half of an interrupted promotion, when that is what these
/// key-sharing entries are.
///
/// **The library names this state and Grove has to recognise it, because the
/// process that meets it is never the process that caused it.**
/// `Error::FailedPartiallyRolledBack` says *a node and a leaf sharing an ordinal
/// and a key, with the node holding no distinguished child, is an interrupted
/// promotion* — but it says so in the run whose rollback failed. A later command
/// opens a tree in exactly that state and the library reports nothing at all: a
/// duplicate key is an obligation on the domain and not something any operation
/// checks. So the only wording available is Grove's, which is why writing one is
/// not a second wording of anything (`docs/ARCHITECTURE.md#library-refusals`,
/// clause 3) — and the recovery it gives is the library's own, not
/// [`addressable_key`]'s general *give one a fresh key*, which is actively wrong
/// here: the node and the leaf are **one entity** caught mid-shape-change, and
/// giving either a fresh key would make two of it.
///
/// The signature is exact and cannot be met by a hand edit that merely
/// duplicated a key: two entries, one a node and one a leaf, at the same
/// ordinal, with the node empty of a `BRIEF.md`. Both are positioned by
/// construction — the caller filtered on `key() == Some(_)`, and the charter
/// brief carries no key — so the ordinals compared here always exist. Grove itself never writes a
/// childless node — `leaf-decompose` creates the brief in the same unit — so
/// nothing in the verb set produces this shape by any other route.
fn interrupted_promotion<'a>(twins: &[Entry<'a, TaskName>]) -> Option<Entry<'a, TaskName>> {
    let [first, second] = twins else { return None };
    let (node, leaf) = match (first.contents(), second.contents()) {
        (Some(_), None) => (*first, *second),
        (None, Some(_)) => (*second, *first),
        _ => return None,
    };
    if node.ordinal() != leaf.ordinal() {
        return None;
    }
    node.contents()?.distinguished().is_none().then_some(node)
}

/// The key the library will give the next entry it creates from this snapshot —
/// `max + 1` over every name in the tree — or `None` when the keyspace is full.
///
/// # A mirror of the library's rule, and the reason grove needs one
///
/// The library allocates keys and grove does not, which is why grove's own
/// key allocator died. But grove's leaf **content** embeds the key its name
/// will carry — the first-line handle `# <slug>-k<key>` — and
/// [`NewEntry`](ordinal_fs_tree::NewEntry) takes its bytes *before* the library
/// composes the name, so the bytes cannot be written from the answer. A
/// content-carrying domain therefore has to predict the allocation.
///
/// It is a prediction and it is checked: every grow verb compares this against
/// the key the library actually reports and refuses to claim success on a
/// disagreement (`task_grow::allocated`). The prediction reads the *same*
/// snapshot the operation plans from under the *same* guard, so it can only
/// differ if the library's allocation rule changes — which is exactly what the
/// check exists to catch, since a silent disagreement is a leaf whose header
/// contradicts its filename.
///
/// `None` rather than a refusal: an exhausted keyspace is
/// [`Refusal::KeysExhausted`](ordinal_fs_tree::Refusal), which is the library's
/// to state and not grove's to anticipate
/// (`docs/ARCHITECTURE.md#library-refusals`, clause 3). The caller hands the
/// library no bytes and lets it refuse — a refusal writes nothing, so the
/// unrenderable content is never reached.
#[must_use]
pub fn next_key(snapshot: &Snapshot<TaskName>) -> Option<Key> {
    let greatest = snapshot
        .walk()
        .filter_map(|entry| entry.key())
        .map(Key::get)
        .max()
        .unwrap_or(0);
    greatest.checked_add(1).map(Key::new)
}

/// A live leaf's session kind and handle, or `None` when the entry is not one.
fn live_leaf(entry: &Entry<'_, TaskName>) -> Option<(Kind, String)> {
    let triple = entry.triple()?;
    match triple.parts {
        Parts::Leaf {
            outcome: Outcome::Live,
            kind,
            slug,
        } => Some((*kind, format!("{slug}-k{}", triple.key.get()))),
        _ => None,
    }
}

/// The outcome `resolve` reports for a matched entry: a leaf's own
/// live/`DONE`/`ABANDONED` state, or [`Outcome::Live`] for a node — a node
/// carries no terminal state of its own, its done-ness being the absence of a
/// live leaf in its subtree.
fn entry_outcome(entry: &Entry<'_, TaskName>) -> Outcome {
    match entry.triple().map(|triple| triple.parts) {
        Some(Parts::Leaf { outcome, .. }) => *outcome,
        _ => Outcome::Live,
    }
}

/// Everything a launch needs about one selected leaf, copied while a single
/// shared guard is held. Callers never reopen or reparse the tree before launch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedLeaf {
    pub path: PathBuf,
    pub handle: String,
    pub kind: Kind,
}

/// `pick`: the first **live leaf** in walk order, or `None` for a grove with no
/// live work left — the loop's finish signal, which the CLI renders as empty
/// stdout and a *no live leaves* diagnostic.
///
/// Walk order is the library's: within a level the distinguished child first
/// (`BRIEF.md`, never a leaf), then the positioned children by ordinal, with
/// nodes descended in place — so a node at an earlier ordinal is fully explored
/// before a later sibling. `DONE` and `ABANDONED` leaves are skipped; foreign
/// names never reach the snapshot at all.
pub fn pick(grove_root: &Path) -> Result<Option<PathBuf>> {
    let tree = read(grove_root)?;
    pick_in(&tree)
}

/// [`pick`] against a tree already read. Used by every verb that needs a leaf
/// and its brief chain from the *same* observation.
pub fn pick_in(tree: &Tree) -> Result<Option<PathBuf>> {
    Ok(select_in(tree)?.map(|selection| selection.path))
}

/// `select`: one live leaf and every launch fact about it, from one observation.
pub fn select(grove_root: &Path) -> Result<Option<SelectedLeaf>> {
    let tree = read(grove_root)?;
    select_in(&tree)
}

/// [`select`] against a tree already read.
///
/// The `finish` rule is grove's alone and the library knows nothing of it: a
/// `finish` leaf is the driver's own, so ordinary work outranks it wherever it
/// sits, and more than one live `finish` leaf is a malformed tree rather than a
/// choice.
pub fn select_in(tree: &Tree) -> Result<Option<SelectedLeaf>> {
    selected(tree.root(), tree.snapshot())
}

/// [`select_in`] against a tree held **exclusively**.
///
/// The lifecycle verbs select and then write from one observation — the driver's
/// finish sentinel is allocated only if the same snapshot found no live work —
/// and a mutation is on the exclusive guard. Both guards deref to a
/// [`Snapshot`], so this is the same selection and not a second one.
pub(crate) fn select_in_write(tree: &TreeWrite) -> Result<Option<SelectedLeaf>> {
    selected(tree.root(), tree.snapshot())
}

fn selected(root: &Path, snapshot: &Snapshot<TaskName>) -> Result<Option<SelectedLeaf>> {
    let mut live = Vec::new();
    for entry in snapshot.walk() {
        if let Some((kind, handle)) = live_leaf(&entry) {
            live.push((entry, kind, handle));
        }
    }
    let finish: Vec<_> = live
        .iter()
        .filter(|(_, kind, _)| *kind == Kind::Finish)
        .collect();
    if finish.len() > 1 {
        bail!(
            "multiple live `finish` leaves are malformed: {}",
            finish
                .iter()
                .map(|(entry, _, _)| entry_path(root, *entry).display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let selected = live
        .iter()
        .find(|(_, kind, _)| *kind != Kind::Finish)
        .or_else(|| finish.first().copied());
    Ok(selected.map(|(entry, kind, handle)| SelectedLeaf {
        path: entry_path(root, *entry),
        handle: handle.clone(),
        kind: *kind,
    }))
}

/// `kind [<leaf>]`: the task's session kind — one of the closed nineteen, read
/// from the filename and never from the body.
///
/// With `leaf_path = Some`, that leaf; with `None`, [`pick`]'s next live leaf,
/// and `Ok(None)` on a grove with no live work — the same signal `pick` gives.
pub fn kind(grove_root: &Path, leaf_path: Option<&Path>) -> Result<Option<Kind>> {
    let tree = read(grove_root)?;
    let target = match leaf_path {
        Some(path) => Some(path.to_path_buf()),
        None => pick_in(&tree)?,
    };
    let Some(target) = target else {
        return Ok(None);
    };
    match leaf_entry(&tree, &target)?.triple().map(|t| t.parts) {
        Some(Parts::Leaf { kind, .. }) => Ok(Some(*kind)),
        // Unreachable: `leaf_entry` refuses anything that is not a leaf.
        _ => bail!(
            "path is not a current-format Grove leaf: {}",
            target.display()
        ),
    }
}

/// `brief-chain`: the `BRIEF.md` of each of the leaf's ancestor levels, from the
/// grove root down to its containing node, root→leaf.
///
/// This is the library's `distinguished_chain` and nothing else: a node's
/// distinguished child *is* its charter, and the library already skips levels
/// that have none — which is exactly `brief-chain`'s documented *a directory
/// level with no `BRIEF.md` is skipped silently*. A leaf has no brief of its
/// own, so its containing node's is the deepest one collected.
pub fn brief_chain(tree: &Tree, leaf_path: &Path) -> Result<Vec<PathBuf>> {
    let entry = leaf_entry(tree, leaf_path)?;
    Ok(entry
        .distinguished_chain()
        .into_iter()
        .map(|brief| entry_path(tree.root(), brief))
        .collect())
}

/// The snapshot entry a caller's leaf argument names: absolute, or relative to
/// the grove root.
///
/// The clauses are in the order the path-walking reader had, because each says
/// something different to whoever hit it — *there is no such file*, *that is not
/// a task file*, *that is not in this tree*.
fn leaf_entry<'a>(tree: &'a Tree, leaf_path: &Path) -> Result<Entry<'a, TaskName>> {
    let root = tree.root();
    let candidate = if leaf_path.is_absolute() {
        leaf_path.to_path_buf()
    } else {
        root.join(leaf_path)
    };
    if !candidate.is_file() {
        bail!("Grove leaf not found: {}", candidate.display());
    }
    let name = candidate
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("Grove leaf has no UTF-8 filename: {}", candidate.display()))?;
    // The grammar itself, rather than a second reading of the filename: a
    // non-canonical or unknown-kind argument gets the domain's own recovery
    // advice instead of a bare *not a leaf*.
    match TaskName::parse(name, Found::File) {
        Verdict::Entry(TaskName::Positioned {
            parts: Parts::Leaf { .. },
            ..
        }) => {}
        Verdict::Malformed(error) | Verdict::Reserved(error) => bail!("{error}"),
        _ => bail!(
            "path is not a current-format Grove leaf: {}",
            candidate.display()
        ),
    }
    // Canonicalised to *compare* and never to report: two spellings of one path
    // name one entry, and the paths this module returns are still built from the
    // caller's own spelling of the root.
    let target = candidate
        .canonicalize()
        .with_context(|| format!("resolving leaf path {}", candidate.display()))?;
    let root_real = root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", root.display()))?;
    if target == root_real {
        bail!(
            "leaf path {} is the grove root, not a leaf",
            target.display()
        );
    }
    if !target.starts_with(&root_real) {
        bail!(
            "leaf path {} is not under grove root {}",
            target.display(),
            root_real.display()
        );
    }
    for entry in tree.walk() {
        if entry.name().to_string() != name {
            continue;
        }
        if entry_path(root, entry).canonicalize().ok() == Some(target.clone()) {
            return Ok(entry);
        }
    }
    // Task-shaped, under the root, and still not in the tree: it sits under a
    // name the grammar disclaimed, so no walk reaches it.
    bail!(
        "Grove leaf {} is not in the task tree: every level above it must be a \
         node directory named NN-<slug>-k<key>",
        candidate.display()
    )
}

/// The outcome of resolving a reference. The CLI maps this to stdout/stderr via
/// [`render_resolution`]; the split keeps the I/O contract unit-testable without
/// a live verb dispatch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// Exactly one entry matched, with the matched entry's own outcome — so an
    /// abandoned match is distinguishable from both a live and a `DONE` one.
    Found { path: PathBuf, outcome: Outcome },
    /// No entry matched the reference (pick-style: not an error).
    NotFound,
    /// A bare-slug reference matched more than one entry. Each carries its
    /// permanent key so the caller re-queries by the unambiguous key.
    Ambiguous(Vec<AmbiguousMatch>),
}

/// One entry of a [`Resolution::Ambiguous`] result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmbiguousMatch {
    pub key: u32,
    pub path: PathBuf,
    pub outcome: Outcome,
}

/// `resolve <ref>`: turn a reference into the current path of the entity it
/// names, searching the whole tree — live leaves, `DONE` and `ABANDONED` leaves,
/// and node directories alike.
///
///   * `[n]` / `n` → the entity whose permanent key is `n`. Keys are unique
///     tree-wide, so this is the library's `by_key`.
///   * `[n]-slug` → same; the slug part is decorative (ignored).
///   * bare slug → 0 ⇒ `NotFound`; 1 ⇒ `Found`; >1 ⇒ `Ambiguous`.
///   * `<slug>-k<key>` → the full canonical handle, read as its terminal
///     `-k<key>`. Tried only after the bare slug fails, so a literal slug ending
///     in `-k<digits>` still wins.
///
/// The root brief is unreferenceable: it carries no key and no slug.
pub fn resolve(grove_root: &Path, reference: &str) -> Result<Resolution> {
    let tree = read(grove_root)?;
    resolve_in(&tree, reference)
}

/// [`resolve`] against a tree already read.
///
/// **The one lookup grove has that is not by key**, and the one place the seam's
/// narrowness is felt from grove's side. The library offers `by_key` and a
/// `seek` taking the consumer's own predicate, and deliberately *no* lookup by
/// label — the trait names no label type, so a `by_label` would have nothing to
/// take. Slug lookup is therefore a walk with grove's predicate over grove's own
/// `Parts`, and it is a whole walk rather than a `seek` because ambiguity is a
/// property of the match *set*: `seek` short-circuits at the first hit, which is
/// precisely the answer `resolve` must not give.
pub fn resolve_in(tree: &Tree, reference: &str) -> Result<Resolution> {
    Ok(match lookup(tree.snapshot(), reference)? {
        Lookup::Found(entry) => Resolution::Found {
            path: entry_path(tree.root(), entry),
            outcome: entry_outcome(&entry),
        },
        Lookup::NotFound => Resolution::NotFound,
        Lookup::Ambiguous(matches) => Resolution::Ambiguous(
            matches
                .into_iter()
                .map(|entry| AmbiguousMatch {
                    key: slug_match_key(&entry),
                    path: entry_path(tree.root(), entry),
                    outcome: entry_outcome(&entry),
                })
                .collect(),
        ),
    })
}

/// What a reference matched in the snapshot, before anything is said about it.
///
/// **Entries and not paths**, because a mutating verb needs the entry itself: it
/// has to read a key off it and call the library *by key* against this same
/// snapshot (`docs/ARCHITECTURE.md#library-refusals`, clause 1). [`resolve_in`]
/// renders the same three outcomes as paths, so grove has one lookup and not
/// two — the read verb's answer and the write verb's are the same walk.
enum Lookup<'a> {
    Found(Entry<'a, TaskName>),
    NotFound,
    Ambiguous(Vec<Entry<'a, TaskName>>),
}

/// The key of an entry a bare slug matched.
///
/// [`Lookup::Ambiguous`] arises only from the slug branch of [`lookup`], whose
/// filter reads the slug off a `triple` — so every entry in it is positioned and
/// carries a key.
fn slug_match_key(entry: &Entry<'_, TaskName>) -> u32 {
    entry
        .key()
        .map_or_else(|| unreachable!("a slug match is positioned"), Key::get)
}

/// The reference grammar itself: `[n]` / `n` / `[n]-slug` by key, a bare slug by
/// slug, and a full `<slug>-k<key>` handle by its terminal key once the bare
/// slug has failed.
fn lookup<'a>(snapshot: &'a Snapshot<TaskName>, reference: &str) -> Result<Lookup<'a>> {
    // The library answers a search with `Sought`, its own word for *matched
    // nothing* — not a refusal, and not an error. Grove already has a word for
    // the same thing in its own vocabulary, so this maps one onto the other and
    // stops there: `Lookup` is what the rest of grove reads.
    let by_key = |key: u32| -> Lookup<'a> {
        match snapshot.by_key(Key::new(key)) {
            Sought::Match(entry) => Lookup::Found(entry),
            Sought::Nothing => Lookup::NotFound,
        }
    };
    match parse_ref(reference)? {
        Ref::Key(key) => Ok(by_key(key)),
        Ref::Slug(slug) => {
            // A whole walk and never `seek`: ambiguity is a property of the
            // match *set*, and `seek` short-circuits at the first hit — which is
            // precisely the answer this must not give.
            let matches: Vec<Entry<'a, TaskName>> = snapshot
                .walk()
                .filter(|entry| {
                    entry
                        .triple()
                        .is_some_and(|triple| triple.parts.slug().as_str() == slug.as_str())
                })
                .collect();
            Ok(match matches.len() {
                0 => match handle_key(&slug) {
                    Some(key) => by_key(key),
                    None => Lookup::NotFound,
                },
                1 => Lookup::Found(matches[0]),
                _ => Lookup::Ambiguous(matches),
            })
        }
    }
}

/// What a `<parent>` / `<target>` argument names in the tree: a path, or a
/// reference in the key/slug namespace.
///
/// **Clause 1, for the verbs whose argument is a reference rather than a path.**
/// The resolution runs against the snapshot the mutation will plan from, which
/// one guard already guarantees — where the pre-flip verbs resolved a reference
/// to a *path* under grove's own guard and then re-read the directory to act on
/// it.
///
/// Tried as a path first so an explicit, existing path always wins; only a
/// non-existent path is re-tried as a reference, so the two namespaces never
/// collide in practice. `.` is a path — the grove root — and so needs no case of
/// its own.
pub fn reference<'a>(
    root: &Path,
    snapshot: &'a Snapshot<TaskName>,
    argument: &str,
) -> Result<Target<'a>> {
    if let Some(path) = existing_path(root, argument) {
        return target(root, snapshot, &path);
    }
    match lookup(snapshot, argument)? {
        Lookup::Found(entry) => Ok(Target::Entry(entry)),
        Lookup::NotFound => bail!(
            "no entry matches {argument:?} (tried as a path under the grove root \
             and as a key/slug)"
        ),
        Lookup::Ambiguous(matches) => {
            let keys = matches
                .iter()
                .map(|entry| format!("[{}]", slug_match_key(entry)))
                .collect::<Vec<_>>()
                .join(", ");
            bail!("reference {argument:?} is ambiguous; re-query by key: {keys}")
        }
    }
}

/// Interpret an argument as a path that actually exists: absolute, or relative
/// to the grove root, or relative to the cwd. `None` if no such path exists —
/// then it is a key/slug reference.
///
/// This preserves the *pass back what `pick`/`resolve` printed* ergonomics (the
/// absolute branch) and the worktree-relative convenience.
fn existing_path(grove_root: &Path, argument: &str) -> Option<PathBuf> {
    let candidate = Path::new(argument);
    if candidate.is_absolute() {
        return candidate.exists().then(|| candidate.to_path_buf());
    }
    let grove_relative = grove_root.join(candidate);
    if grove_relative.exists() {
        return Some(grove_relative);
    }
    let cwd_relative = std::env::current_dir().ok()?.join(candidate);
    cwd_relative.exists().then_some(cwd_relative)
}

/// A parsed reference: a permanent key, or a bare slug.
pub(crate) enum Ref {
    Key(u32),
    Slug(String),
}

/// Classify a `resolve` reference. `[n]` / `[n]-slug` and a bare integer `n`
/// resolve by key; anything else is a bare slug. A bracketed-but-malformed key
/// (`[abc]`, `[4`) is a reference error, distinct from a valid-but-unmatched
/// reference, which is `NotFound`.
pub(crate) fn parse_ref(reference: &str) -> Result<Ref> {
    if let Some(rest) = reference.strip_prefix('[') {
        let close = rest
            .find(']')
            .with_context(|| format!("reference {reference:?}: unclosed '['"))?;
        let key: u32 = rest[..close]
            .parse()
            .with_context(|| format!("reference {reference:?}: '[…]' is not an integer key"))?;
        // Anything after `]` (e.g. `-slug`) is decorative.
        Ok(Ref::Key(key))
    } else if !reference.is_empty() && reference.bytes().all(|b| b.is_ascii_digit()) {
        let key: u32 = reference
            .parse()
            .with_context(|| format!("reference {reference:?}: not an integer key"))?;
        Ok(Ref::Key(key))
    } else {
        Ok(Ref::Slug(reference.to_string()))
    }
}

/// Peel the terminal `-k<digits>` of a full `<slug>-k<key>` handle into its key,
/// or `None` when the reference is not handle-shaped. Mirrors the filename
/// grammar's *the key is the terminal `-k<digits>`* rule, so a handle and a
/// filename peel the key identically — `migrate-v1-to-v2-k27` → `27`, `build` →
/// `None`.
pub(crate) fn handle_key(reference: &str) -> Option<u32> {
    let digits_start = reference.len()
        - reference
            .bytes()
            .rev()
            .take_while(u8::is_ascii_digit)
            .count();
    if digits_start == reference.len() {
        return None; // no trailing digits → not a handle
    }
    reference[..digits_start].strip_suffix("-k")?;
    reference[digits_start..].parse().ok()
}

/// Render a [`Resolution`] to the `(stdout, stderr)` the `resolve` verb emits.
/// Kept pure and separate from the I/O so the exact contract is unit-testable
/// without going through the CLI dispatch.
#[must_use]
pub fn render_resolution(reference: &str, resolution: &Resolution) -> (String, String) {
    match resolution {
        Resolution::Found { path, outcome } => {
            let stdout = format!("{}\n", path.display());
            let stderr = match outcome {
                Outcome::Live => String::new(),
                Outcome::Done => format!(
                    "note: referenced task is retired (DONE): {}\n",
                    path.display()
                ),
                // The abandoned counterpart of the DONE note above: `resolve`
                // must not let a pruned dead end look live.
                Outcome::Abandoned => format!(
                    "note: referenced task is abandoned (ABANDONED): {}\n",
                    path.display()
                ),
            };
            (stdout, stderr)
        }
        Resolution::NotFound => (
            String::new(),
            format!("resolve: no entry matches reference {reference:?}\n"),
        ),
        Resolution::Ambiguous(matches) => {
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
                    matched.key,
                    matched.path.display(),
                    tag
                ));
            }
            (String::new(), stderr)
        }
    }
}

#[cfg(test)]
pub(crate) fn reset_read_count() {
    READ_COUNT.with(|count| count.set(0));
}

#[cfg(test)]
pub(crate) fn read_count() -> usize {
    READ_COUNT.with(std::cell::Cell::get)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// The guard composed with the verb — what a test needs to drive
    /// `brief-chain` standalone. Production never wants it: `llm_cli` holds one
    /// tree across `pick` and the ancestor walk, because selecting a leaf and
    /// reading its brief chain under two observations would be reading a tree
    /// that could move in between.
    fn brief_chain_at(grove_root: &Path, leaf_path: &Path) -> Result<Vec<PathBuf>> {
        let tree = read(grove_root)?;
        brief_chain(&tree, leaf_path)
    }

    /// Stand up a fresh `.grove/` directory and return `(tempdir, grove_root)`.
    fn grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
    }

    /// Write a stub file into `dir`, returning its absolute path.
    fn touch(dir: &Path, name: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, b"# stub\n").unwrap();
        p
    }

    /// Create a node directory inside `dir`, returning its absolute path.
    fn mknode(dir: &Path, name: &str) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(&p).unwrap();
        p
    }

    /// The path's final component, for terse assertions.
    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    // ---- pick ---------------------------------------------------------------

    #[test]
    fn select_returns_path_handle_and_kind_from_one_guarded_observation() {
        let (_t, g) = grove();
        let path = touch(&g, "01-review-impl-selected-k7.md");
        reset_read_count();

        let selected = select(&g).unwrap().unwrap();

        assert_eq!(
            selected,
            SelectedLeaf {
                path,
                handle: "selected-k7".to_string(),
                kind: Kind::ReviewImpl,
            }
        );
        assert_eq!(
            read_count(),
            1,
            "selection must not reopen the tree to derive launch facts"
        );
    }

    #[test]
    fn pick_returns_first_live_leaf_in_per_level_order() {
        let (_t, g) = grove();
        touch(&g, "02-impl-b-k2.md");
        touch(&g, "01-impl-a-k1.md");
        touch(&g, "10-impl-c-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl-a-k1.md");
    }

    #[test]
    fn pick_orders_numerically_not_lexically() {
        // 99 < 100 — the case a lexical sort of the rendered names fails, since
        // "100" sorts before "99". The old spelling of this test made the point
        // with an unpadded `2-…`, which the canonical grammar now refuses
        // (`docs/adr/task-names-are-canonical.md`); the zero-padding is a
        // *minimum* width, so a three-digit ordinal is the discriminating case
        // that survives, and it is a stronger one — the walk orders on the
        // parsed ordinal, never on the name.
        let (_t, g) = grove();
        touch(&g, "100-impl-b-k2.md");
        touch(&g, "99-impl-a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "99-impl-a-k1.md");
    }

    #[test]
    fn pick_skips_done_leaves() {
        let (_t, g) = grove();
        touch(&g, "01-DONE-impl-a-k1.md");
        touch(&g, "02-impl-b-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-impl-b-k2.md");
    }

    #[test]
    fn pick_skips_abandoned_leaves() {
        // Symmetric with DONE (pruning): an abandoned leaf is a terminal
        // state, skipped exactly like a retired one.
        let (_t, g) = grove();
        touch(&g, "01-ABANDONED-impl-a-k1.md");
        touch(&g, "02-impl-b-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-impl-b-k2.md");
    }

    #[test]
    fn pick_descends_a_node_in_preorder() {
        // A node at an earlier position is fully explored before a later sibling
        // leaf: the node's first live child wins.
        let (_t, g) = grove();
        let node = mknode(&g, "01-design-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-impl-child-k2.md");
        touch(&g, "02-impl-later-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl-child-k2.md");
    }

    #[test]
    fn pick_skips_briefs_and_returns_the_child_leaf() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let node = mknode(&g, "01-node-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-impl-child-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl-child-k2.md");
    }

    #[test]
    fn pick_falls_through_an_all_done_node_to_a_later_live_leaf() {
        // A node whose subtree is entirely retired yields no live leaf, so pick
        // moves on to the next sibling.
        let (_t, g) = grove();
        let node = mknode(&g, "01-done-node-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-DONE-impl-child-k2.md");
        touch(&g, "02-impl-live-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-impl-live-k3.md");
    }

    #[test]
    fn pick_falls_through_a_pruned_node_to_a_later_live_leaf() {
        // A node whose only leaf was pruned yields no live leaf either — the
        // grove's two terminal leaf states (DONE, ABANDONED) behave identically
        // for the walk (pruning).
        let (_t, g) = grove();
        let node = mknode(&g, "01-dead-node-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-ABANDONED-impl-child-k2.md");
        touch(&g, "02-impl-live-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-impl-live-k3.md");
    }

    #[test]
    fn pick_descends_nested_nodes() {
        let (_t, g) = grove();
        let n1 = mknode(&g, "01-outer-k1");
        touch(&n1, "BRIEF.md");
        let n2 = mknode(&n1, "01-inner-k2");
        touch(&n2, "BRIEF.md");
        touch(&n2, "01-impl-deep-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl-deep-k3.md");
    }

    #[test]
    fn pick_none_when_only_briefs_and_done_leaves() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let node = mknode(&g, "01-node-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-DONE-impl-child-k2.md");
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_none_when_every_remaining_leaf_is_abandoned() {
        // A grove whose only remaining leaves are abandoned reports "no live
        // leaves" — correct: the work is settled, however it settled.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "01-ABANDONED-impl-a-k1.md");
        let node = mknode(&g, "02-node-k2");
        touch(&node, "BRIEF.md");
        touch(&node, "01-DONE-impl-b-k3.md");
        touch(&node, "02-ABANDONED-impl-c-k4.md");
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_none_on_empty_tree() {
        let (_t, g) = grove();
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_lenient_on_foreign_files() {
        let (_t, g) = grove();
        touch(&g, "README.md");
        touch(&g, "notes.txt");
        touch(&g, "01-impl-a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl-a-k1.md");
    }

    #[test]
    fn pick_none_when_only_foreign_files() {
        let (_t, g) = grove();
        touch(&g, "README.md");
        touch(&g, "notes.txt");
        assert_eq!(pick(&g).unwrap(), None);
    }

    /// Both species mismatches at a **task-shaped** name are malformed, not
    /// foreign — and a later live leaf must not paper over them. The old answer
    /// (skip both, return `02-impl-real-k2.md`) is what let a hand-typed
    /// `01-DONE-node-k1/` swallow a whole live subtree: `pick` reported the grove
    /// finished while real work sat inside. Skipping is safe only for names the
    /// grow verbs would never write, and both of these are names they *do* write —
    /// at the other species.
    #[test]
    fn pick_refuses_a_species_mismatch_at_a_task_shaped_name() {
        for (make, name, expected) in [
            (
                &mknode as &dyn Fn(&Path, &str) -> PathBuf,
                "01-impl-trap-k1.md",
                "names a leaf",
            ),
            (
                &|d: &Path, n: &str| touch(d, n),
                "01-trap-k1",
                "names a node",
            ),
        ] {
            let (_t, g) = grove();
            make(&g, name);
            touch(&g, "02-impl-real-k2.md");

            let error = pick(&g).unwrap_err().to_string();

            assert!(error.contains(name), "{name}: {error}");
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

    /// The species rule reaches symlinks for free, and closing that is the point
    /// rather than a side effect: `DirEntry::file_type` does not follow links, so a
    /// symlink is neither a regular file nor a directory. Under the old `!is_dir`
    /// test a symlink at a leaf name *passed* as a leaf, and `pick` would hand the
    /// driver a mandate whose path resolves outside `.grove/` entirely.
    #[test]
    fn pick_refuses_a_symlink_at_a_task_shaped_name() {
        let (t, g) = grove();
        let outside = t.path().join("outside.md");
        fs::write(&outside, b"# outside\n").unwrap();
        std::os::unix::fs::symlink(&outside, g.join("01-impl-linked-k1.md")).unwrap();

        let error = pick(&g).unwrap_err().to_string();

        assert!(error.contains("01-impl-linked-k1.md"), "{error}");
        assert!(error.contains("must be a regular file"), "{error}");
    }

    #[test]
    fn pick_ignores_a_legacy_done_directory() {
        // A stray `done/` directory (or any foreign dir) is not a node and holds
        // no live leaf reachable by the walk.
        let (_t, g) = grove();
        let legacy = mknode(&g, "done");
        touch(&legacy, "09-impl-old-k9.md");
        touch(&g, "01-impl-a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl-a-k1.md");
    }

    #[test]
    fn pick_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = pick(&missing).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- brief-chain --------------------------------------------------------

    #[test]
    fn brief_chain_root_level_leaf_returns_only_root_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let leaf = touch(&g, "01-impl-a-k1.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_two_levels_deep_root_then_each_ancestor_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "02-mid-k1");
        touch(&n1, "BRIEF.md");
        let n2 = mknode(&n1, "01-node-k2");
        touch(&n2, "BRIEF.md");
        let leaf = touch(&n2, "01-impl-leaf-k3.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        // Each brief's parent dir distinguishes them; assert on the parent.
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "02-mid-k1", "01-node-k2"]
        );
        assert!(chain.iter().all(|p| name_of(p) == "BRIEF.md"));
    }

    #[test]
    fn brief_chain_only_includes_ancestors_not_sibling_subtrees() {
        // The directory ascent inherently excludes a sibling node's brief: a leaf
        // under `01-design` never sees `02-other`'s brief.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let design = mknode(&g, "01-design-k1");
        touch(&design, "BRIEF.md");
        let other = mknode(&g, "02-other-k3");
        touch(&other, "BRIEF.md");
        let leaf = touch(&design, "01-impl-leaf-k2.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

    #[test]
    fn brief_chain_skips_missing_intermediate_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        // No BRIEF.md in `02-mid` — a mid-decomposition transient.
        let n1 = mknode(&g, "02-mid-k1");
        let n2 = mknode(&n1, "01-node-k2");
        touch(&n2, "BRIEF.md");
        let leaf = touch(&n2, "01-impl-leaf-k3.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-node-k2"]
        );
    }

    #[test]
    fn brief_chain_skips_missing_root_brief() {
        let (_t, g) = grove();
        // No root BRIEF.md.
        let n1 = mknode(&g, "02-mid-k1");
        touch(&n1, "BRIEF.md");
        let leaf = touch(&n1, "01-impl-leaf-k2.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec!["02-mid-k1"]
        );
    }

    #[test]
    fn brief_chain_resolves_chain_for_a_done_leaf() {
        // Normally called on a live leaf, but a `DONE` leaf still has ancestors.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-design-k1");
        touch(&n1, "BRIEF.md");
        let leaf = touch(&n1, "01-DONE-impl-leaf-k2.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

    #[test]
    fn brief_chain_accepts_grove_root_relative_leaf_path() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-design-k1");
        touch(&n1, "BRIEF.md");
        touch(&n1, "01-impl-leaf-k2.md");
        let chain = brief_chain_at(&g, Path::new("01-design-k1/01-impl-leaf-k2.md")).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

    #[test]
    fn brief_chain_errors_when_leaf_outside_grove_root() {
        let (tmp, g) = grove();
        touch(&g, "BRIEF.md");
        let stray = tmp.path().join("stray.md");
        fs::write(&stray, b"# stub\n").unwrap();
        let err = brief_chain_at(&g, &stray).unwrap_err();
        assert!(
            err.to_string().contains("not a current-format Grove leaf"),
            "got {err}"
        );
    }

    #[test]
    fn brief_chain_errors_when_given_the_grove_root_itself() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let err = brief_chain_at(&g, &g).unwrap_err();
        assert!(
            err.to_string().contains("Grove leaf not found"),
            "got {err}"
        );
    }

    #[test]
    fn brief_chain_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = brief_chain_at(&missing, Path::new("01-impl-a-k1.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- kind ---------------------------------------------------------------

    /// Write a leaf whose body carries arbitrary legacy routing metadata. The
    /// current reader must derive kind solely from the filename.
    fn touch_body(dir: &Path, name: &str, body_after_header: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, format!("# stub\n\n{body_after_header}").as_bytes()).unwrap();
        p
    }

    #[test]
    fn kind_reads_an_impl_leaf() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-impl-a-k1.md", "**Kind:** impl\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_reads_a_planning_leaf() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-planning-a-k1.md", "**Kind:** impl\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Planning));
    }

    #[test]
    fn kind_reads_every_one_of_the_nineteen_from_filenames() {
        let (_t, g) = grove();
        for (i, want) in Kind::ALL.into_iter().enumerate() {
            let name = format!("{:02}-{}-a-k{}.md", i + 1, want.label(), i + 1);
            let leaf = touch_body(&g, &name, "**Kind:** bogus\n**Harness:** bogus\n");
            assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(want));
        }
    }

    #[test]
    fn kind_ignores_a_legacy_work_label_in_the_body() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-impl-a-k1.md", "**Kind:** work\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_no_arg_defaults_to_picks_next_leaf() {
        let (_t, g) = grove();
        touch(&g, "01-DONE-impl-old-k1.md"); // skipped by pick
        touch_body(&g, "02-planning-live-k2.md", "**Kind:** impl\n");
        // No leaf arg ⇒ pick's next live leaf (02-live), whose kind is planning.
        assert_eq!(kind(&g, None).unwrap(), Some(Kind::Planning));
    }

    #[test]
    fn kind_none_on_empty_grove() {
        // No live leaves ⇒ Ok(None), the same signal pick gives (the CLI renders
        // the "no live leaves" diagnostic).
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        assert_eq!(kind(&g, None).unwrap(), None);
    }

    #[test]
    fn kind_accepts_a_grove_root_relative_path() {
        let (_t, g) = grove();
        let node = mknode(&g, "01-design-k1");
        touch(&node, "BRIEF.md");
        touch_body(&node, "01-impl-leaf-k2.md", "**Kind:** impl\n");
        let got = kind(&g, Some(Path::new("01-design-k1/01-impl-leaf-k2.md"))).unwrap();
        assert_eq!(got, Some(Kind::Impl));
    }

    #[test]
    fn kind_ignores_trailing_commentary_on_a_legacy_kind_line() {
        let (_t, g) = grove();
        let leaf = touch_body(
            &g,
            "01-impl-a-k1.md",
            "**Kind:** impl          (or: planning)\n",
        );
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_reads_an_impl_filename_with_no_kind_line() {
        let (_t, g) = grove();
        let leaf = touch(&g, "01-impl-a-k1.md");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_ignores_a_garbled_kind_token_in_the_body() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-impl-a-k1.md", "**Kind:** bogus\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_ignores_a_family_name_written_in_the_body() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-impl-a-k1.md", "**Kind:** review\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = kind(&missing, None).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- resolve ------------------------------------------------------------

    /// A nested tree with two `add` slugs (one live leaf, one retired leaf in a
    /// different subtree) so bare-slug `add` is ambiguous, plus a node directory
    /// and a unique `build` leaf — to exercise key, slug, node, and `DONE`.
    ///
    /// ```text
    /// .grove/
    ///   BRIEF.md
    ///   01-design-k1/         node
    ///     BRIEF.md
    ///     01-impl-add-k2.md        live leaf, slug "add"
    ///     02-impl-remove-k3.md
    ///   02-add-k4.DONE? -> 02-DONE-impl-add-k4.md   retired leaf, slug "add"
    ///   03-impl-build-k5.md
    /// ```
    fn resolve_fixture() -> (TempDir, PathBuf) {
        let (tmp, g) = grove();
        touch(&g, "BRIEF.md");
        let design = mknode(&g, "01-design-k1");
        touch(&design, "BRIEF.md");
        touch(&design, "01-impl-add-k2.md");
        touch(&design, "02-impl-remove-k3.md");
        touch(&g, "02-DONE-impl-add-k4.md");
        touch(&g, "03-impl-build-k5.md");
        (tmp, g)
    }

    #[test]
    fn resolve_by_bracket_key_finds_a_nested_leaf() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[2]").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "01-impl-add-k2.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_by_bare_number_finds_a_done_leaf() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "4").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "02-DONE-impl-add-k4.md");
                assert_eq!(outcome, Outcome::Done, "the key-4 task is DONE");
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_finds_a_pruned_leaf_by_key() {
        // pruning: an abandoned leaf's key must stay resolvable — durable
        // cross-references to it (commit messages, ADRs, briefs) are precisely
        // what the ADR protects. And `resolve` must not let the match *look*
        // live: `outcome` must come back `Abandoned`, not just the right path —
        // this is the exact failure mode pruning exists to prevent ("a
        // tree that hides its dead ends lies"), here in `resolve` rather than
        // the tree itself.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "01-ABANDONED-impl-spike-k1.md");
        match resolve(&g, "[1]").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "01-ABANDONED-impl-spike-k1.md");
                assert_eq!(outcome, Outcome::Abandoned);
            }
            other => panic!("expected Found, got {other:?}"),
        }
        // The full `<slug>-k<key>` handle resolves it too.
        match resolve(&g, "spike-k1").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "01-ABANDONED-impl-spike-k1.md");
                assert_eq!(outcome, Outcome::Abandoned);
            }
            other => panic!("expected Found, got {other:?}"),
        }
        // And the CLI-facing render carries the same note a DONE match gets,
        // in its own wording.
        let (_out, err) = render_resolution("[1]", &resolve(&g, "[1]").unwrap());
        assert!(
            err.contains("abandoned") && err.contains("ABANDONED"),
            "got {err:?}"
        );
    }

    #[test]
    fn resolve_bracket_key_ignores_decorative_slug() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[5]-whatever").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "03-impl-build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_key_resolves_a_node_to_its_directory() {
        // A node's identity rides in its directory name, so a key reference to a
        // node resolves to the directory path (append /BRIEF.md to read it).
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[1]").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "01-design-k1");
                assert!(path.is_dir());
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_key_not_found() {
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "[99]").unwrap(), Resolution::NotFound);
    }

    #[test]
    fn resolve_bare_slug_unique_across_dirs() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "build").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "03-impl-build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_bare_slug_resolves_a_nested_unique_leaf() {
        // `remove` lives only inside the node directory — slug search recurses.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "remove").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "02-impl-remove-k3.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_bare_slug_not_found() {
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "nope").unwrap(), Resolution::NotFound);
    }

    #[test]
    fn resolve_bare_slug_ambiguous_lists_every_match_by_key() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "add").unwrap() {
            Resolution::Ambiguous(matches) => {
                // Pre-order: the nested `01-design/01-add-k2` precedes the
                // root-level `02-DONE-add-k4`.
                assert_eq!(matches.len(), 2);
                assert_eq!(matches[0].key, 2);
                assert_eq!(name_of(&matches[0].path), "01-impl-add-k2.md");
                assert_eq!(matches[0].outcome, Outcome::Live);
                assert_eq!(matches[1].key, 4);
                assert_eq!(name_of(&matches[1].path), "02-DONE-impl-add-k4.md");
                assert_eq!(matches[1].outcome, Outcome::Done);
            }
            other => panic!("expected Ambiguous, got {other:?}"),
        }
    }

    #[test]
    fn resolve_root_brief_is_unreferenceable() {
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "").unwrap(), Resolution::NotFound);
    }

    #[test]
    fn resolve_malformed_bracket_ref_errors() {
        let (_t, g) = resolve_fixture();
        assert!(resolve(&g, "[abc]").is_err());
        assert!(resolve(&g, "[4").is_err());
    }

    #[test]
    fn resolve_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = resolve(&missing, "[1]").unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- resolve: the full `<slug>-k<key>` handle (task-tree-scheme §5) --------------

    #[test]
    fn resolve_by_full_slug_handle_finds_by_terminal_key() {
        // §5's canonical commit/prose handle is `<slug>-k<key>`; resolve accepts it
        // directly — the terminal `-k<key>` is read as the key, the slug decorative
        // — so the handle round-trips back to a path.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "build-k5").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "03-impl-build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_full_handle_disambiguates_what_a_bare_slug_could_not() {
        // The bare slug `add` is ambiguous (two matches); the full handle `add-k2`
        // names exactly the nested live leaf via its key.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "add-k2").unwrap() {
            Resolution::Found { path, .. } => {
                assert_eq!(name_of(&path), "01-impl-add-k2.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_handle_of_a_node_resolves_to_its_directory() {
        // A node's handle (`design-k1`) resolves to the node directory, like its key.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "design-k1").unwrap() {
            Resolution::Found { path, .. } => {
                assert_eq!(name_of(&path), "01-design-k1");
                assert!(path.is_dir());
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_prefers_a_real_slug_over_the_handle_fallback() {
        // A bare slug that itself ends in `-k<digits>` resolves as a slug first; the
        // key fallback fires only when the slug match is empty. So a slug `foo-k5`
        // (key 7) wins over a *different* entity that happens to hold key 5.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "01-impl-foo-k5-k7.md"); // slug "foo-k5", key 7
        touch(&g, "02-impl-other-k5.md"); // slug "other", key 5
        match resolve(&g, "foo-k5").unwrap() {
            Resolution::Found { path, .. } => {
                assert_eq!(
                    name_of(&path),
                    "01-impl-foo-k5-k7.md",
                    "the real slug match must win over the key-5 handle fallback"
                );
            }
            other => panic!("expected Found (slug), got {other:?}"),
        }
    }

    #[test]
    fn resolve_handle_shaped_but_unmatched_is_not_found() {
        // A handle whose key matches nothing is NotFound (not an error), like any
        // unmatched reference.
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "ghost-k99").unwrap(), Resolution::NotFound);
    }

    // ---- render_resolution --------------------------------------------------

    #[test]
    fn render_found_prints_path_no_stderr() {
        let r = Resolution::Found {
            path: PathBuf::from("/g/.grove/03-impl-build-k5.md"),
            outcome: Outcome::Live,
        };
        let (out, err) = render_resolution("[5]", &r);
        assert_eq!(out, "/g/.grove/03-impl-build-k5.md\n");
        assert!(err.is_empty(), "got {err:?}");
    }

    #[test]
    fn render_found_retired_notes_on_stderr_but_still_prints_path() {
        let r = Resolution::Found {
            path: PathBuf::from("/g/.grove/02-DONE-impl-add-k4.md"),
            outcome: Outcome::Done,
        };
        let (out, err) = render_resolution("4", &r);
        assert_eq!(out, "/g/.grove/02-DONE-impl-add-k4.md\n");
        assert!(err.contains("retired"), "got {err:?}");
    }

    #[test]
    fn render_found_abandoned_notes_on_stderr_but_still_prints_path() {
        // The abandoned counterpart of the DONE case above: a resolved
        // `ABANDONED` entry must get its own stderr note, not silence
        // (silence is what a live match gets) and not the DONE wording.
        let r = Resolution::Found {
            path: PathBuf::from("/g/.grove/01-ABANDONED-impl-spike-k1.md"),
            outcome: Outcome::Abandoned,
        };
        let (out, err) = render_resolution("1", &r);
        assert_eq!(out, "/g/.grove/01-ABANDONED-impl-spike-k1.md\n");
        assert!(err.contains("abandoned"), "got {err:?}");
        assert!(!err.contains("retired"), "got {err:?}");
    }

    #[test]
    fn render_not_found_empty_stdout_diagnostic_stderr() {
        let (out, err) = render_resolution("nope", &Resolution::NotFound);
        assert!(out.is_empty(), "got {out:?}");
        assert!(err.contains("no entry matches"), "got {err:?}");
        assert!(err.contains("nope"), "got {err:?}");
    }

    #[test]
    fn render_ambiguous_lists_keys_on_stderr_empty_stdout() {
        let r = Resolution::Ambiguous(vec![
            AmbiguousMatch {
                key: 2,
                path: PathBuf::from("/g/.grove/01-design-k1/01-impl-add-k2.md"),
                outcome: Outcome::Live,
            },
            AmbiguousMatch {
                key: 4,
                path: PathBuf::from("/g/.grove/02-DONE-impl-add-k4.md"),
                outcome: Outcome::Done,
            },
        ]);
        let (out, err) = render_resolution("add", &r);
        assert!(
            out.is_empty(),
            "stdout must be empty for ambiguous; got {out:?}"
        );
        assert!(err.contains("ambiguous"), "got {err:?}");
        assert!(err.contains("[2]"), "got {err:?}");
        assert!(err.contains("[4]"), "got {err:?}");
        assert!(err.contains("retired"), "got {err:?}");
    }

    #[test]
    fn render_ambiguous_tags_an_abandoned_match() {
        let r = Resolution::Ambiguous(vec![AmbiguousMatch {
            key: 1,
            path: PathBuf::from("/g/.grove/01-ABANDONED-impl-spike-k1.md"),
            outcome: Outcome::Abandoned,
        }]);
        let (_out, err) = render_resolution("spike", &r);
        assert!(
            err.contains("[1]") && err.contains("(abandoned)"),
            "got {err:?}"
        );
    }

    // ---- pick + brief-chain together ----------------------------------------

    #[test]
    fn pick_then_brief_chain_on_a_realistic_nested_tree() {
        // End-to-end: pick the first live leaf in a nested tree, then resolve its
        // ancestor brief chain — the loop's bootstrap path.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-scheme-k1");
        touch(&n1, "BRIEF.md");
        touch(&n1, "01-DONE-impl-id-model-k2.md");
        let leaf = touch(&n1, "02-impl-read-verbs-k3.md");
        touch(&g, "02-impl-shed-tui-k4.md");

        let picked = pick(&g).unwrap().unwrap();
        assert_eq!(picked, leaf);

        let chain = brief_chain_at(&g, &picked).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-scheme-k1"]
        );
    }
}

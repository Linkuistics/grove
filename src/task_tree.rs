// Grove's **reading surface**, expressed through `ordinal-fs-tree` (gh issue
// #13, increment 2, the first leaf of the *migrate* stage).
//
// `pick`, `select`, `brief-chain`, `kind` and `resolve` all read one
// [`Snapshot`](ordinal_fs_tree::Snapshot) taken under the library's shared lock,
// and their semantics are unchanged: first live leaf in walk order, ancestor
// briefs root→leaf, reference-by-permanent-key. What changes is who owns the
// walk. `src/tree_read.rs` keeps only what the verbs that have *not* flipped
// still need — grove's own exclusive guard and the library's cannot be nested
// (both `flock` the directory containing the tree root, and two open file
// descriptions on one directory do not share a lock), so a write verb keeps the
// path-walking reader until its own leaf moves it across.
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
// wherever it sits. That is the decision, and it is taken under the lock. But a
// legacy tree's leaves are task-shaped names with no session kind, so they are
// `Malformed` — and an operator holding one needs to be told to migrate rather
// than to fix a filename. So [`diagnose`] re-states a *failed* read in the order
// grove owes its operator: root, then a pending transaction, then the format
// witness, then the library's own message. Only the wording is chosen here; the
// refusal itself already happened.

use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use ordinal_fs_tree::{Entry, EntryName, Error, Found, Key, Verdict};

use crate::leaf::Kind;
use crate::task_name::{Outcome, Parts, TaskName};
use crate::tree_access;
use crate::tree_format;

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
/// these. The format witness is checked while the guard is held, so a tree
/// cannot be migrated out from under a reader between the two observations.
pub fn read(grove_root: &Path) -> Result<Tree> {
    #[cfg(test)]
    READ_COUNT.with(|count| count.set(count.get() + 1));

    announce_contention(grove_root);
    match ordinal_fs_tree::fs::read::<TaskName>(grove_root) {
        Ok(tree) => {
            tree_format::require_current(grove_root)?;
            Ok(tree)
        }
        Err(error) => Err(diagnose(grove_root, &error)),
    }
}

/// Say that this process is waiting, before it blocks.
///
/// The library's locking is invisible in its interface — no try-variant, no
/// timeout, and `read` simply blocks — which is the architecture's own decision
/// and not something to work around. But grove has always told an operator why
/// it appears to have hung, and losing that is a user-visible regression in what
/// the node brief calls a pure refactor. So the diagnostic is bought outside the
/// library: one non-blocking acquisition of the same mode on the same directory
/// the library will lock, released immediately.
///
/// Best-effort by construction. Between releasing this probe and the library
/// taking its own lock a contender can arrive, and then this process blocks
/// silently; the probe is a diagnostic and never a decision, so that window
/// costs a message and nothing else. Everything that can go wrong with it —
/// including the directory not existing — is silence, because the library is
/// about to report the same condition properly.
fn announce_contention(grove_root: &Path) {
    // `<root>/..` and not `Path::parent`: the same spelling the library locks,
    // resolved by the kernel, so the probe asks about the directory the library
    // will actually contend for rather than a lexical parent of the string.
    let Ok(handle) = File::open(grove_root.join("..")) else {
        return;
    };
    let descriptor = handle.as_raw_fd();
    // SAFETY: `descriptor` is open for the whole call — `handle` owns it and
    // outlives both `flock`s — and `flock` touches nothing else.
    if unsafe { libc::flock(descriptor, libc::LOCK_SH | libc::LOCK_NB) } == 0 {
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
/// The library halted, and it halted for the right reason; what it cannot know
/// is that a task-shaped name with no session kind means *this tree predates
/// filename kinds and must be migrated*, not *fix this filename*. Each clause
/// below is a condition grove states in its own words, tried in the precedence
/// the path-walking reader had. The checks are unlocked, deliberately: the
/// decision to refuse was already taken under the lock, and only the wording is
/// chosen here.
fn diagnose(grove_root: &Path, error: &Error<TaskName>) -> anyhow::Error {
    if !grove_root.is_dir() {
        return anyhow!("grove root not found: {}", grove_root.display());
    }
    if let Err(refusal) = tree_access::refuse_pending(grove_root) {
        return refusal;
    }
    if let Err(refusal) = tree_format::require_current(grove_root) {
        return refusal;
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
    let mut live = Vec::new();
    for entry in tree.walk() {
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
                .map(|(entry, _, _)| entry_path(tree.root(), *entry).display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let selected = live
        .iter()
        .find(|(_, kind, _)| *kind != Kind::Finish)
        .or_else(|| finish.first().copied());
    Ok(selected.map(|(entry, kind, handle)| SelectedLeaf {
        path: entry_path(tree.root(), *entry),
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
/// `find` taking the consumer's own predicate, and deliberately *no* lookup by
/// label — the trait names no label type, so a `by_label` would have nothing to
/// take. Slug lookup is therefore a walk with grove's predicate over grove's own
/// `Parts`, and it is a whole walk rather than a `find` because ambiguity is a
/// property of the match *set*: `find` short-circuits at the first hit, which is
/// precisely the answer `resolve` must not give.
pub fn resolve_in(tree: &Tree, reference: &str) -> Result<Resolution> {
    let by_key = |key: u32| -> Resolution {
        tree.by_key(Key::new(key))
            .map_or(Resolution::NotFound, |entry| Resolution::Found {
                path: entry_path(tree.root(), entry),
                outcome: entry_outcome(&entry),
            })
    };

    match parse_ref(reference)? {
        Ref::Key(key) => Ok(by_key(key)),
        Ref::Slug(slug) => {
            let matches: Vec<AmbiguousMatch> = tree
                .walk()
                .filter_map(|entry| {
                    let triple = entry.triple()?;
                    (triple.parts.slug().as_str() == slug.as_str()).then(|| AmbiguousMatch {
                        key: triple.key.get(),
                        path: entry_path(tree.root(), entry),
                        outcome: entry_outcome(&entry),
                    })
                })
                .collect();
            Ok(match matches.as_slice() {
                [] => match handle_key(&slug) {
                    Some(key) => by_key(key),
                    None => Resolution::NotFound,
                },
                [only] => Resolution::Found {
                    path: only.path.clone(),
                    outcome: only.outcome,
                },
                _ => Resolution::Ambiguous(matches),
            })
        }
    }
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
        crate::tree_format::write_current_last(&root).unwrap();
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

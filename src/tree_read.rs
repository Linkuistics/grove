// The **read verbs** (task-tree-scheme) — `pick`, `brief-chain`, `resolve` —
// expressed against the real **directory tree**, built on the id model
// (`src/tree_id.rs`). Keeps task-tree-scheme's *semantics* (first live leaf in DFS
// pre-order; ancestor briefs root→leaf; reference-by-permanent-key) and changes
// only the **walk**: in v1 the whole tree was encoded into flat filenames, so a
// verb was a single `read_dir` + version-sort over `.grove/`; in v2 the
// filesystem carries the hierarchy (a node is a *directory* holding its children,
// and optionally a `BRIEF.md` charter — a chain node carries none, which is why
// `brief-chain`'s tolerance of a missing level is load-bearing rather than
// incidental), so the same verbs become filesystem-shape walks:
//
//   * `pick`        — recursive depth-first pre-order over node dirs;
//   * `brief-chain` — the leaf's ancestor *directories* root→leaf, each `BRIEF.md`;
//   * `resolve`     — a recursive collect-the-tree, then match by key/slug.
//
// These are the verbs `llm_cli` dispatches; the flat-scheme `leaf_read` they
// replaced is gone. `Resolution` and `render_resolution` live here rather than in a
// shared module because this is now the only reader.
//
// `resolve`'s **reference grammar** keeps the flat scheme's `[n]` / `n` /
// `[n]-slug` / bare-slug forms verbatim and adds exactly one: the full
// `<slug>-k<key>` handle that task-tree-scheme §5 makes canonical for commits and
// prose. `handle_key` peels the handle's terminal `-k<key>`, and the slug branch
// falls back to it only when no bare slug matched — so every older reference still
// resolves identically (a literal slug ending in `-k<digits>` is matched as a slug
// first), and the §5 handle round-trips to a path.

use crate::leaf::Kind;
use crate::tree_access;
use crate::tree_id::{parse_current, sort_key, Entry, Outcome};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// `pick`: a recursive depth-first **pre-order** walk over the directory tree,
/// returning the first **live leaf**. Within each directory the children are
/// visited in per-level order (the charter brief first — and skipped — then by
/// numeric position): a live leaf returns immediately; a node directory is
/// descended in place (pre-order, so a node at an earlier position is fully
/// explored before a later sibling leaf); a `DONE` leaf, an `ABANDONED` leaf
/// (pruning), the brief, and foreign names are skipped. `Ok(None)` means no
/// live leaf anywhere — the loop's finish signal (the CLI renders it as empty
/// stdout + a "no live leaves" stderr diagnostic). Foreign names such as a
/// stray `README.md` are skipped; task-shaped names with missing or unknown
/// kinds are rejected. Never reads file contents.
pub fn pick(grove_root: &Path) -> Result<Option<PathBuf>> {
    let guard = tree_access::read(grove_root)?;
    Ok(select_unlocked(guard.root())?.map(|selection| selection.path))
}

pub(crate) fn pick_unlocked(grove_root: &Path) -> Result<Option<PathBuf>> {
    Ok(select_unlocked(grove_root)?.map(|selection| selection.path))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedLeaf {
    pub path: PathBuf,
    pub handle: String,
    pub kind: Kind,
}

/// Select one live leaf and copy every launch fact while one shared tree guard
/// is held. Callers never need to reopen or reparse the tree before launch.
pub fn select(grove_root: &Path) -> Result<Option<SelectedLeaf>> {
    let guard = tree_access::read(grove_root)?;
    select_unlocked(guard.root())
}

pub(crate) fn select_unlocked(grove_root: &Path) -> Result<Option<SelectedLeaf>> {
    #[cfg(test)]
    tree_access::assert_guard_held(grove_root);

    let mut live = Vec::new();
    collect_live_leaf_entries(grove_root, &mut live)?;
    let finish = live
        .iter()
        .filter(|(entry, _)| entry.kind() == Some(Kind::Finish))
        .collect::<Vec<_>>();
    if finish.len() > 1 {
        bail!(
            "multiple live `finish` leaves are malformed: {}",
            finish
                .iter()
                .map(|(_, path)| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let selected = if let Some(selected) = live
        .iter()
        .find(|(entry, _)| entry.kind() != Some(Kind::Finish))
    {
        Some(selected)
    } else {
        finish.first().copied()
    };
    selected
        .map(|(entry, path)| {
            Ok(SelectedLeaf {
                path: path.clone(),
                handle: entry
                    .handle()
                    .context("selected live leaf has no stable handle")?,
                kind: entry
                    .kind()
                    .context("selected live leaf has no session kind")?,
            })
        })
        .transpose()
}

fn collect_live_leaf_entries(dir: &Path, live: &mut Vec<(Entry, PathBuf)>) -> Result<()> {
    for (entry, path) in read_level(dir)? {
        match &entry {
            Entry::Leaf {
                outcome: Outcome::Live,
                ..
            } => live.push((entry, path)),
            Entry::Node { .. } => collect_live_leaf_entries(&path, live)?,
            Entry::Brief
            | Entry::Leaf {
                outcome: Outcome::Done | Outcome::Abandoned,
                ..
            } => {}
        }
    }
    Ok(())
}

/// `brief-chain`: the `BRIEF.md` of each of the leaf's **ancestor directories**,
/// from the grove root down to the leaf's containing directory, in root→leaf
/// order. The headline behavioural shift from v1: ancestor briefs are found by
/// **directory ascent** (the filesystem carries the hierarchy now), not by
/// id-prefix in a flat namespace. A directory level with no `BRIEF.md` is skipped
/// silently (some nodes are mid-decomposition); a leaf has no brief of its own,
/// so its own directory's brief is the deepest one collected. `leaf_path` is
/// absolute or relative to `grove_root`, and must resolve to a path under it.
pub(crate) fn brief_chain_unlocked(grove_root: &Path, leaf_path: &Path) -> Result<Vec<PathBuf>> {
    #[cfg(test)]
    tree_access::assert_guard_held(grove_root);

    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    let grove_root = grove_root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", grove_root.display()))?;

    let candidate = if leaf_path.is_absolute() {
        leaf_path.to_path_buf()
    } else {
        grove_root.join(leaf_path)
    };
    let leaf_abs = candidate
        .canonicalize()
        .with_context(|| format!("resolving leaf path {}", candidate.display()))?;
    current_leaf_entry(&leaf_abs)?;
    if !leaf_abs.starts_with(&grove_root) {
        bail!(
            "leaf path {} is not under grove root {}",
            leaf_abs.display(),
            grove_root.display()
        );
    }
    if leaf_abs == grove_root {
        bail!(
            "leaf path {} is the grove root, not a leaf",
            leaf_abs.display()
        );
    }

    // The leaf's ancestor directories: its containing directory, then up to (and
    // including) the grove root. The leaf lives strictly under the root (checked
    // above), so the ascent always reaches it.
    let mut dirs = Vec::new();
    let mut cur = leaf_abs
        .parent()
        .with_context(|| format!("leaf path {} has no parent", leaf_abs.display()))?
        .to_path_buf();
    loop {
        dirs.push(cur.clone());
        if cur == grove_root {
            break;
        }
        match cur.parent() {
            Some(p) => cur = p.to_path_buf(),
            None => break,
        }
    }
    dirs.reverse(); // root→leaf

    let mut chain = Vec::new();
    for d in dirs {
        let brief = d.join("BRIEF.md");
        if brief.is_file() {
            chain.push(brief);
        }
    }
    Ok(chain)
}

/// `kind [<leaf>]`: the task's kind — one of the closed nineteen. This is the
/// agent CLI's diagnostic read; the loop driver keys its one configuration
/// lookup on the kind [`select`] already gave it, and never comes back through
/// here. With `leaf_path = Some`, read that leaf's
/// current-format filename; with `None`, default to [`pick`]'s next live leaf and
/// return `Ok(None)` on an empty grove — the same "no live leaves" signal `pick`
/// gives (the CLI renders it as the standard stderr diagnostic, mirroring
/// `brief-chain`). `leaf_path` is absolute or relative to `grove_root`.
/// Task-shaped filenames with a missing or unknown kind fail strictly.
pub fn kind(grove_root: &Path, leaf_path: Option<&Path>) -> Result<Option<Kind>> {
    let guard = tree_access::read(grove_root)?;
    match target_leaf_unlocked(guard.root(), leaf_path)? {
        Some(leaf) => current_leaf_entry(&leaf).map(|entry| entry.kind()),
        None => Ok(None),
    }
}

/// The leaf a leaf-reading verb operates on: the given path (absolute, or
/// relative to `grove_root`), else [`pick`]'s next live leaf. `Ok(None)` is the
/// empty grove — the same "no live leaves" signal `pick` gives, which the CLI
/// renders as the standard stderr diagnostic.
fn target_leaf_unlocked(grove_root: &Path, leaf_path: Option<&Path>) -> Result<Option<PathBuf>> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    let target = match leaf_path {
        Some(p) if p.is_absolute() => Some(p.to_path_buf()),
        Some(p) => Some(grove_root.join(p)),
        None => pick_unlocked(grove_root)?,
    };
    if let Some(path) = &target {
        current_leaf_entry(path)?;
    }
    Ok(target)
}

fn current_leaf_entry(path: &Path) -> Result<Entry> {
    if !path.is_file() {
        bail!("Grove leaf not found: {}", path.display());
    }
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("Grove leaf has no UTF-8 filename: {}", path.display()))?;
    match parse_current(name)? {
        Some(entry @ Entry::Leaf { .. }) => Ok(entry),
        _ => bail!(
            "path is not a current-format Grove leaf: {}",
            path.display()
        ),
    }
}

/// The outcome of resolving a reference. The CLI maps this to stdout/stderr via
/// [`render_resolution`]; the split keeps the I/O contract unit-testable without
/// a live verb dispatch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// Exactly one entry matched. `outcome` is the matched leaf's own
    /// live/`DONE`/`ABANDONED` state (pruning) — so an abandoned match
    /// is distinguishable from both a live and a `DONE` one, not folded into a
    /// single `retired` bit. A matched node reports `Outcome::Live`: a node
    /// carries no terminal state of its own, its done-ness is the absence of a
    /// live leaf in its subtree.
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

/// The [`Outcome`] `resolve` reports for a matched entry: a leaf's own
/// live/`DONE`/`ABANDONED` state, or `Outcome::Live` for a node (a node has no
/// terminal state of its own — see [`Resolution::Found`]).
fn entry_outcome(e: &Entry) -> Outcome {
    match e {
        Entry::Leaf { outcome, .. } => *outcome,
        Entry::Node { .. } | Entry::Brief => Outcome::Live,
    }
}

/// `resolve <ref>`: turn a reference into the current path of the entity it
/// names, searching the **whole directory tree** — live leaves, `DONE` leaves,
/// and node directories alike.
///   * `[n]` / `n` → the unique entity whose permanent key is `n` (keys are
///     unique tree-wide → 0 or 1 match). The primary durable-reference path.
///     A node resolves to its **directory** path (the dir name carries the key);
///     append `/BRIEF.md` to read its charter.
///   * `[n]-slug` → same; the slug part is decorative (ignored).
///   * bare slug → resolve by slug: 0 ⇒ `NotFound`; 1 ⇒ `Found`; >1 ⇒
///     `Ambiguous` (every match with its key, so the caller re-queries by key).
///   * `<slug>-k<key>` → the full canonical handle (task-tree-scheme §5): same as `[n]`
///     via its terminal `-k<key>`, the slug decorative. Tried only after the bare
///     slug fails to match, so a literal slug ending in `-k<digits>` still wins.
///
/// The root brief (`BRIEF.md`, the one unkeyed singleton) is unreferenceable.
pub fn resolve(grove_root: &Path, reference: &str) -> Result<Resolution> {
    let guard = tree_access::read(grove_root)?;
    resolve_unlocked(guard.root(), reference)
}

pub(crate) fn resolve_unlocked(grove_root: &Path, reference: &str) -> Result<Resolution> {
    #[cfg(test)]
    tree_access::assert_guard_held(grove_root);

    let mut all = Vec::new();
    collect_all(grove_root, &mut all)?;

    // Keys are unique tree-wide → at most one match; a node resolves to its
    // directory path (the dir name carries the key).
    let find_by_key = |key: u32| -> Resolution {
        all.iter()
            .find(|(e, _)| e.key() == Some(key))
            .map_or(Resolution::NotFound, |(e, path)| Resolution::Found {
                path: path.clone(),
                outcome: entry_outcome(e),
            })
    };

    match parse_ref(reference)? {
        Ref::Key(key) => Ok(find_by_key(key)),
        Ref::Slug(slug) => {
            // The brief (key None) is excluded — it is unreferenceable.
            let matches: Vec<AmbiguousMatch> = all
                .iter()
                .filter_map(|(e, path)| {
                    let key = e.key()?;
                    (e.slug() == Some(slug.as_str())).then(|| AmbiguousMatch {
                        key,
                        path: path.clone(),
                        outcome: entry_outcome(e),
                    })
                })
                .collect();
            Ok(match matches.as_slice() {
                // No slug match: retry as the full `<slug>-k<key>` handle (task-tree-scheme
                // §5's canonical commit/prose form), reading its terminal
                // `-k<digits>` as the key. Slugs are matched first, so a real slug
                // that itself ends in `-k<digits>` still wins — the fallback only
                // fires when the slug matched nothing.
                [] => match handle_key(&slug) {
                    Some(key) => find_by_key(key),
                    None => Resolution::NotFound,
                },
                [m] => Resolution::Found {
                    path: m.path.clone(),
                    outcome: m.outcome,
                },
                _ => Resolution::Ambiguous(matches),
            })
        }
    }
}

/// A parsed reference: a permanent key, or a bare slug.
enum Ref {
    Key(u32),
    Slug(String),
}

/// Classify a `resolve` reference (held identical to v1 — see the module header).
/// `[n]` / `[n]-slug` and a bare integer `n` resolve by key; anything else is a
/// bare slug. A bracketed-but-malformed key (`[abc]`, `[4`) is a reference error
/// (distinct from a valid-but-unmatched reference, which is `NotFound`).
fn parse_ref(reference: &str) -> Result<Ref> {
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

/// Peel the terminal `-k<digits>` of a full `<slug>-k<key>` handle (task-tree-scheme §5's
/// canonical commit/prose reference) into its key, or `None` when the reference is
/// not handle-shaped. Mirrors the filename grammar's "the key is the terminal
/// `-k<digits>`" rule (`src/tree_id.rs`), so a handle and a filename peel the key
/// identically — `migrate-v1-to-v2-k27` → `27`, `build` → `None`.
fn handle_key(reference: &str) -> Option<u32> {
    let digits_start = reference.len()
        - reference
            .bytes()
            .rev()
            .take_while(|b| b.is_ascii_digit())
            .count();
    if digits_start == reference.len() {
        return None; // no trailing digits → not a handle
    }
    reference[..digits_start].strip_suffix("-k")?;
    reference[digits_start..].parse().ok()
}

/// Render a [`Resolution`] to the `(stdout, stderr)` the `resolve` verb emits.
/// Kept pure and separate from the I/O so the exact stdout/stderr contract is
/// unit-testable without going through the CLI dispatch.
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
                // The abandoned counterpart of the DONE note above (pruning):
                // `resolve` must not let a pruned dead end look live, the same
                // failure mode that record exists to prevent.
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
            for m in matches {
                let tag = match m.outcome {
                    Outcome::Live => "",
                    Outcome::Done => " (retired)",
                    Outcome::Abandoned => " (abandoned)",
                };
                stderr.push_str(&format!("  [{}] {}{}\n", m.key, m.path.display(), tag));
            }
            (String::new(), stderr)
        }
    }
}

/// Read one directory's entries **unparsed**, in the per-level comparator's
/// order (the charter brief first, then by numeric position, foreign last).
///
/// The raw sibling of [`read_level`], and the crate's one directory-order
/// primitive. [`read_level`] is what a *walk* wants — it drops foreign names and
/// reconciles each parse against the real filesystem kind, so a caller can trust
/// the `Entry`. This is what a caller wants when the parse is not the question:
/// `tree_promotion` allocating over every name in a subtree, and
/// `task_relationship` reading bodies beside a producer. Both need the same
/// deterministic order, and neither may drop an entry for failing to parse —
/// which is exactly what [`read_level`] would do.
pub(crate) fn sorted_entries(directory: &Path) -> Result<Vec<fs::DirEntry>> {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .with_context(|| format!("reading {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| sort_key(&entry.file_name().to_string_lossy()));
    Ok(entries)
}

/// Read one directory's grove entries, parsed and sorted by the per-level
/// comparator (the charter brief first, then by numeric position, foreign last).
/// Returns `(Entry, path)` for every child whose name parses **and** whose real
/// on-disk species agrees with the parse.
///
/// This is where the **species half** of the task-shaped strictness rule lands
/// (`tree_id`'s module header states it whole): `tree_id::parse` infers
/// leaf-vs-node from the `.md` suffix alone, so a *directory* named `…-k1.md` or a
/// *file* shaped like a node name is a task-shaped name whose entry is not the
/// species it declares — a **malformed tree**, refused here, not a foreign entry
/// skipped. Skipping is what made a whole live subtree vanish under a hand-typed
/// `01-DONE-node-k1/`, and the leaf-side rule exists to prevent exactly that.
/// `BRIEF.md` is outside the rule — it carries no position and no key, a node with
/// no charter is legal everywhere, and so nothing is lost by ignoring an oddity at
/// that name.
///
/// **The one such reader in the crate.** `tree_grow` and `tree_lifecycle` call it
/// rather than keeping their own copies, so grow, retire, prune, key allocation
/// and `pick` cannot disagree about what a sibling is — and, in particular, a
/// subtree `pick` refuses can never be a subtree `next_key` silently skips,
/// re-issuing a live permanent key.
pub(crate) fn read_level(dir: &Path) -> Result<Vec<(Entry, PathBuf)>> {
    let mut entries: Vec<(String, Entry, PathBuf)> = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(parsed) = parse_current(&name)
            .with_context(|| format!("reading current Grove entry {}", entry.path().display()))?
        else {
            continue;
        };
        // `file_type` does not follow symlinks, so a symlink is neither a regular
        // file nor a directory and fails every species test below — deliberately:
        // followed, it would hand `pick` a path resolving outside the tree.
        let file_type = entry.file_type().ok();
        if matches!(parsed, Entry::Brief) {
            if !file_type.is_some_and(|t| !t.is_dir()) {
                continue;
            }
        } else {
            let (declared, ok) = if parsed.is_node() {
                ("node directory", file_type.is_some_and(|t| t.is_dir()))
            } else {
                ("leaf", file_type.is_some_and(|t| t.is_file()))
            };
            if !ok {
                bail!(
                    "malformed Grove tree entry {}: the name is task-shaped and declares a \
                     {declared}, so the entry must be {}. Restore it to the species its name \
                     declares, or rename it out of the NN-<slug>-k<key> grammar if it is not \
                     Grove's.",
                    entry.path().display(),
                    if parsed.is_node() {
                        "a directory"
                    } else {
                        "a regular file"
                    }
                );
            }
        }
        entries.push((name, parsed, entry.path()));
    }
    entries.sort_by_key(|a| sort_key(&a.0));
    Ok(entries.into_iter().map(|(_, e, p)| (e, p)).collect())
}

/// Recursively collect every parsed entry in the tree — leaves (live and `DONE`),
/// node directories, and briefs — each with its absolute path, in pre-order.
/// The shared scan behind [`resolve`]'s tree-wide key/slug search.
fn collect_all(dir: &Path, out: &mut Vec<(Entry, PathBuf)>) -> Result<()> {
    for (entry, path) in read_level(dir)? {
        let descend = entry.is_node();
        out.push((entry, path.clone()));
        if descend {
            collect_all(&path, out)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// The shared read guard composed with the lock-neutral verb — what a test
    /// needs to drive `brief-chain` standalone. This was a `pub fn brief_chain`
    /// on the module until `dead-non-launch-exports-k166`, and no production
    /// caller ever reached it: `llm_cli` holds one read guard across `pick` and
    /// the ancestor walk, because selecting a leaf and reading its brief chain
    /// under two separate guards would be reading a tree that could move in
    /// between. Only the tests, driving the second half alone, wanted the
    /// wrapper — so it belongs to them.
    fn brief_chain(grove_root: &Path, leaf_path: &Path) -> Result<Vec<PathBuf>> {
        let guard = tree_access::read(grove_root)?;
        brief_chain_unlocked(guard.root(), leaf_path)
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
        tree_access::reset_acquisition_count();

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
            tree_access::acquisition_count(),
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
        // 9 < 10 — the case a dumb lexical sort of unpadded positions fails. The
        // v2 grammar zero-pads to two digits, but the comparator is numeric so an
        // unpadded hand-typed name still orders right.
        let (_t, g) = grove();
        touch(&g, "10-impl-b-k2.md");
        touch(&g, "2-impl-a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "2-impl-a-k1.md");
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
                "declares a leaf",
            ),
            (
                &|d: &Path, n: &str| touch(d, n),
                "01-trap-k1",
                "declares a node directory",
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
        let chain = brief_chain(&g, &leaf).unwrap();
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
        let chain = brief_chain(&g, &leaf).unwrap();
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
        let chain = brief_chain(&g, &leaf).unwrap();
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
        let chain = brief_chain(&g, &leaf).unwrap();
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
        let chain = brief_chain(&g, &leaf).unwrap();
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
        let chain = brief_chain(&g, &leaf).unwrap();
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
        let chain = brief_chain(&g, Path::new("01-design-k1/01-impl-leaf-k2.md")).unwrap();
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
        let err = brief_chain(&g, &stray).unwrap_err();
        assert!(
            err.to_string().contains("not a current-format Grove leaf"),
            "got {err}"
        );
    }

    #[test]
    fn brief_chain_errors_when_given_the_grove_root_itself() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let err = brief_chain(&g, &g).unwrap_err();
        assert!(
            err.to_string().contains("Grove leaf not found"),
            "got {err}"
        );
    }

    #[test]
    fn brief_chain_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = brief_chain(&missing, Path::new("01-impl-a-k1.md")).unwrap_err();
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

        let chain = brief_chain(&g, &picked).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-scheme-k1"]
        );
    }
}

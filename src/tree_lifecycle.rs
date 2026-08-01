// The **lifecycle verbs** (task-tree-scheme) — `root-init`, `leaf-decompose`,
// `leaf-retire`, and `leaf-prune` — expressed against the real **directory
// tree**, built on the id model (`src/tree_id.rs`) and the grow verbs
// (`src/tree_grow.rs`). Keeps task-tree-scheme's *semantics* (a fresh grove
// starts with one live leaf so it is never mistaken for finished; decompose
// enforces a first child; retire is leaves-only and done-ness is marked in
// place; prune marks abandonment in place, ADR *pruning*) and changes the
// *mechanics* to the filesystem's shape:
//
//   * `root-init` writes the root `BRIEF.md` (the one unkeyed singleton) and a
//     first **requirements** leaf `01-<slug>-k1.md` — a 2-digit per-level
//     position;
//   * `leaf-decompose` turns the leaf *file* `NN-<slug>-k<key>.md` into a node
//     *directory* `NN-<slug>-k<key>/` (**key preserved** — the entity that was the
//     leaf becomes the node), renaming the leaf body in as the node's `BRIEF.md`
//     and growing a first child atomically so a node is never childless;
//   * `leaf-retire` adds a `DONE` infix in place (`NN-<slug>-k<key>.md` →
//     `NN-DONE-<slug>-k<key>.md`), keeping the retired leaf in its directory at its
//     position — no `done/` directory;
//   * `leaf-prune` adds an `ABANDONED` infix in place, symmetric with retire, but
//     — per ADR *pruning* — accepts a **node** too: marking every *live* leaf
//     in the subtree (bulk, since one decision can kill many leaves at once),
//     leaving `DONE` leaves alone.
//
// **Position-free headers:** a leaf/brief header is the stable handle
// `# <slug>-k<key>` (`# … — brief` for a node), so `leaf-retire`/`leaf-prune`
// leave the file's content byte-identical (the outcome infix is filename-only)
// and `leaf-decompose` only appends ` — brief` to the handle.
//
// These are the verbs `llm_cli` dispatches; the flat-scheme `leaf_lifecycle` they
// replaced is gone.

use crate::leaf::Kind;
use crate::tree_access;
use crate::tree_grow::{leaf_add, leaf_add_unlocked};
use crate::tree_id::{parse, sort_key, validate_slug, Entry, Outcome};
use crate::tree_rename::rename_entry;
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// `root-init [<slug>]`: scaffold a fresh grove under `worktree/.grove` — the root
/// `BRIEF.md` (the one unkeyed singleton) and a first **requirements** leaf
/// `01-<slug>-k1.md`. Returns the absolute paths created, `BRIEF.md` first then the
/// leaf. Refuses to clobber an existing `.grove/`. Working-tree only — no commit.
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
    validate_slug(slug)?;

    let grove_root = worktree.join(".grove");
    if grove_root.exists() {
        bail!("grove root already exists: {}", grove_root.display());
    }
    fs::create_dir_all(&grove_root)
        .with_context(|| format!("creating {}", grove_root.display()))?;

    let brief_path = grove_root.join("BRIEF.md");
    write_root_brief(&brief_path, &grove_name(worktree))?;

    // Delegate the first leaf to `leaf_add` (root parent) so the scaffolded leaf is
    // byte-identical to one the LLM would later add by hand — no template drift. A
    // fresh `.grove/` has only `BRIEF.md`, so the first root child is `01-<slug>-k1`.
    let leaf_path = leaf_add(&grove_root, &grove_root, slug, Kind::Requirements, None)?;
    Ok(vec![brief_path, leaf_path])
}

/// `leaf-decompose <leaf-path> <first-child-slug>`: convert a live leaf file
/// `NN-<slug>-k<key>.md` into a node directory `NN-<slug>-k<key>/` (**key
/// preserved**) holding a `BRIEF.md` (seeded from the leaf body, its `# <handle>`
/// header retitled `# <handle> — brief`) and a first child
/// `01-<first-child-slug>-k<new>.md` grown atomically so the node is never
/// childless. Refuses a brief, a node, and an already-`DONE` leaf. Returns
/// `(brief_path, first_child_path)`. Working-tree only — no commit.
///
/// `kind_override` is `--kind`'s *override* of the first child's kind
/// (task-kind-taxonomy): `None` inherits the leaf being decomposed's own kind —
/// read via [`crate::tree_read::read_kind`], which degrades to `impl` rather
/// than erroring, so a leaf with a garbled `**Kind:**` line can still be
/// decomposed — `Some(k)` uses `k` regardless of the parent's kind.
///
/// The first child **also inherits a declared `**Harness:**`** (leaf-harness-k15),
/// for the same reason it inherits the kind: decomposing says *this leaf was
/// bigger than its brief assumed*, so the first child is that leaf's work
/// continued, and a vendor-bound `research` leaf whose child silently fell back
/// to the stamp would be exactly the misroute the per-leaf axis exists to
/// prevent. There is no `--harness` override here — the declaration is rare
/// enough that dropping it is an edit to one line of the new child, while
/// *losing* it is invisible.
///
/// Unlike the kind read, this one can **fail**: `read_harness` refuses an
/// unrecognised name rather than degrading. That is deliberate here too — the
/// leaf would refuse to launch anyway, and a human is present at decompose time
/// to fix the line.
pub fn leaf_decompose(
    grove_root: &Path,
    leaf_path: &Path,
    first_child_slug: &str,
    kind_override: Option<Kind>,
) -> Result<(PathBuf, PathBuf)> {
    let guard = tree_access::write(grove_root)?;
    leaf_decompose_unlocked(guard.root(), leaf_path, first_child_slug, kind_override)
}

fn leaf_decompose_unlocked(
    grove_root: &Path,
    leaf_path: &Path,
    first_child_slug: &str,
    kind_override: Option<Kind>,
) -> Result<(PathBuf, PathBuf)> {
    // Validate the child slug up front, before any filesystem mutation, so a bad
    // slug leaves the leaf un-decomposed (no half-built childless node directory).
    validate_slug(first_child_slug)?;

    let grove_abs = canonical_grove_root(grove_root)?;
    let (parent_abs, name) = resolve_leaf_file(&grove_abs, leaf_path)?;
    let (position, slug, key) = match parse(&name) {
        Some(Entry::Leaf {
            outcome: Outcome::Live,
            position,
            slug,
            key,
        }) => (position, slug, key),
        Some(Entry::Leaf {
            outcome: Outcome::Done,
            ..
        }) => bail!("cannot decompose a retired (DONE) leaf: {name}"),
        Some(Entry::Leaf {
            outcome: Outcome::Abandoned,
            ..
        }) => bail!("cannot decompose an abandoned (ABANDONED) leaf: {name}"),
        Some(Entry::Brief) => bail!("cannot decompose a brief (it is already a node): {name}"),
        Some(Entry::Node { .. }) => {
            bail!("cannot decompose a node (it already has children): {name}")
        }
        None => bail!("not a v2 leaf: {name}"),
    };

    // Inherit the parent leaf's own kind unless overridden — read before the
    // rename below moves it out from under this path.
    let kind = match kind_override {
        Some(k) => k,
        None => crate::tree_read::read_kind(&parent_abs.join(&name))?,
    };
    let harness = crate::tree_read::read_harness(&parent_abs.join(&name))?;

    // The entity that was leaf k becomes node directory k: same position, key, and
    // slug — only the on-disk shape changes (file → directory holding BRIEF.md).
    let node_name = Entry::Node {
        position,
        slug: slug.clone(),
        key,
    }
    .name();
    let node_dir = parent_abs.join(&node_name);
    if node_dir.exists() {
        bail!("destination already exists: {}", node_dir.display());
    }
    fs::create_dir(&node_dir).with_context(|| format!("creating {}", node_dir.display()))?;

    // Rename the leaf file into the new directory as its charter `BRIEF.md`; the
    // leaf body is carried in verbatim, then its `# <handle>` header retitled.
    rename_entry(&parent_abs, &name, format!("{node_name}/BRIEF.md"))?;
    let brief_path = node_dir.join("BRIEF.md");
    append_brief_suffix_in_file(&brief_path, &slug, key)?;

    // Grow the first child at `01` (enforce-first-child) — delegated to `leaf_add`
    // so it is byte-identical to a hand-added child and gets the next fresh key. The
    // node now exists (the BRIEF.md we just created), so the parent guard passes.
    let child_path = leaf_add_unlocked(&grove_abs, &node_dir, first_child_slug, kind, harness)?;
    Ok((brief_path, child_path))
}

/// `leaf-retire <leaf-path>`: rename a live leaf `NN-<slug>-k<key>.md` →
/// `NN-DONE-<slug>-k<key>.md` in place, keeping its position and key. The `DONE`
/// infix is filename-only — the `# <handle>` header is byte-identical. Refuses a
/// brief, a node directory, and an already-`DONE` leaf. Returns the retired file's
/// absolute path. Working-tree only — no commit.
pub fn leaf_retire(grove_root: &Path, leaf_path: &Path) -> Result<PathBuf> {
    let guard = tree_access::write(grove_root)?;
    leaf_retire_unlocked(guard.root(), leaf_path)
}

fn leaf_retire_unlocked(grove_root: &Path, leaf_path: &Path) -> Result<PathBuf> {
    let grove_abs = canonical_grove_root(grove_root)?;
    let (parent_abs, name) = resolve_leaf_file(&grove_abs, leaf_path)?;
    let (done_name, factual_leaf_handle, factual_leaf_key) = match parse(&name) {
        Some(Entry::Leaf {
            outcome: Outcome::Live,
            position,
            slug,
            key,
        }) => {
            let producer_handle = format!("{slug}-k{key}");
            let done_name = Entry::Leaf {
                position,
                slug,
                key,
                outcome: Outcome::Done,
            }
            .name();
            (done_name, producer_handle, key)
        }
        Some(Entry::Leaf {
            outcome: Outcome::Done,
            ..
        }) => bail!("leaf is already retired (DONE): {name}"),
        Some(Entry::Leaf {
            outcome: Outcome::Abandoned,
            ..
        }) => bail!("cannot retire an abandoned (ABANDONED) leaf: {name}"),
        Some(Entry::Brief) => bail!("cannot retire a brief (briefs are never done): {name}"),
        Some(Entry::Node { .. }) => {
            bail!("cannot retire a node (nodes are never marked done): {name}")
        }
        None => bail!("not a v2 leaf: {name}"),
    };

    let done_path = parent_abs.join(&done_name);
    if done_path.exists() {
        bail!("destination already exists: {}", done_path.display());
    }
    // Snapshot the advisory receipt plan while the producer is still live and
    // while this function holds the tree's exclusive lock. Every failure here
    // is retained as a diagnostic plan rather than returned: metadata must
    // never become lifecycle-critical.
    let producer_path = parent_abs.join(&name);
    let receipt_candidates = producer_receipt_candidates(
        &grove_abs,
        &producer_path,
        &factual_leaf_handle,
        factual_leaf_key,
    )?;
    let receipts = crate::task_relationship::prepare_producer_receipts(
        &grove_abs,
        &producer_path,
        &factual_leaf_handle,
        receipt_candidates,
        crate::tree_read::pick_unlocked(&grove_abs),
    );
    // The `DONE` infix is filename-only — the `# <handle>` header is byte-identical.
    rename_entry(&parent_abs, &name, &done_name)?;
    // DONE first, receipt second. A post-rename write failure reports
    // uncheckable metadata and deliberately cannot reverse or mask retirement.
    receipts.materialize();
    Ok(done_path)
}

fn producer_receipt_candidates(
    grove_root: &Path,
    factual_leaf_path: &Path,
    factual_leaf_handle: &str,
    factual_leaf_key: u32,
) -> Result<Vec<crate::task_relationship::ProducerReceiptCandidate>> {
    let mut candidates = vec![crate::task_relationship::ProducerReceiptCandidate {
        path: factual_leaf_path.to_path_buf(),
        handle: factual_leaf_handle.to_string(),
        generation: factual_leaf_key,
    }];
    let mut ancestor = factual_leaf_path.parent();
    while let Some(node) = ancestor {
        if node == grove_root {
            break;
        }
        if node.join("BRIEF.md").is_file()
            && node_closes_when_leaf_retires(node, factual_leaf_path)?
        {
            let entry = node
                .file_name()
                .and_then(|name| name.to_str())
                .and_then(parse)
                .with_context(|| format!("invalid decomposition node {}", node.display()))?;
            let handle = entry
                .handle()
                .with_context(|| format!("decomposition node has no handle: {}", node.display()))?;
            candidates.push(crate::task_relationship::ProducerReceiptCandidate {
                path: node.to_path_buf(),
                handle,
                generation: crate::tree_read::producer_generation_unlocked(node)?,
            });
        }
        ancestor = node.parent();
    }
    Ok(candidates)
}

fn node_closes_when_leaf_retires(node: &Path, factual_leaf_path: &Path) -> Result<bool> {
    let mut live = Vec::new();
    collect_live_leaves(node, &mut live)?;
    Ok(live.as_slice() == [factual_leaf_path])
}

fn collect_live_leaves(node: &Path, live: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(node).with_context(|| format!("reading {}", node.display()))? {
        let entry = entry?;
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        match parse(&name) {
            Some(Entry::Leaf {
                outcome: Outcome::Live,
                ..
            }) => live.push(entry.path()),
            Some(Entry::Node { .. }) => collect_live_leaves(&entry.path(), live)?,
            _ => {}
        }
    }
    Ok(())
}

/// The outcome of a [`leaf_prune`] call: every leaf newly marked `ABANDONED`
/// (its new path), and every already-`DONE` leaf found in scope and left
/// untouched (ADR *pruning*: that work really was done). A single-leaf call
/// marks exactly one entry and finds nothing to leave alone; a node call is
/// bulk — the arity asymmetry with `leaf-retire` is deliberate (ADR *pruning*).
#[derive(Debug)]
pub struct PruneResult {
    pub marked: Vec<PathBuf>,
    pub left_done: Vec<PathBuf>,
}

/// `leaf-prune <path>`: mark abandoned work `ABANDONED` in place (ADR *pruning*).
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
/// **HITL (ADR *pruning*):** this verb does not itself gate on human
/// confirmation — constraint 5 is "grove guides, it does not gate" — so the
/// caller (the LLM driving the session) must already have explicit human
/// confirmation before calling this at all.
pub fn leaf_prune(grove_root: &Path, path: &Path) -> Result<PruneResult> {
    let guard = tree_access::write(grove_root)?;
    leaf_prune_unlocked(guard.root(), path)
}

fn leaf_prune_unlocked(grove_root: &Path, path: &Path) -> Result<PruneResult> {
    let grove_abs = canonical_grove_root(grove_root)?;
    let target_abs = resolve_entry(&grove_abs, path)?;

    if target_abs.is_dir() {
        if target_abs == grove_abs {
            bail!(
                "cannot prune the grove root (abandoning a whole grove is a \
                 branch-delete, not a tree mark): {}",
                target_abs.display()
            );
        }
        let name = entry_name(&target_abs)?;
        if !matches!(parse(&name), Some(Entry::Node { .. })) {
            bail!("not a node directory: {}", target_abs.display());
        }
        let mut result = PruneResult {
            marked: Vec::new(),
            left_done: Vec::new(),
        };
        prune_subtree(&target_abs, &mut result)?;
        Ok(result)
    } else {
        let name = entry_name(&target_abs)?;
        let parent_abs = target_abs
            .parent()
            .with_context(|| format!("path {} has no parent", target_abs.display()))?
            .to_path_buf();
        let marked = prune_one(&parent_abs, &name)?;
        Ok(PruneResult {
            marked: vec![marked],
            left_done: Vec::new(),
        })
    }
}

/// Mark the single live leaf `name` (in `parent_abs`) `ABANDONED` in place,
/// returning its new absolute path. Refuses a brief, a node, an already-`DONE`
/// leaf, and an already-`ABANDONED` leaf.
fn prune_one(parent_abs: &Path, name: &str) -> Result<PathBuf> {
    let abandoned_name = match parse(name) {
        Some(Entry::Leaf {
            outcome: Outcome::Live,
            position,
            slug,
            key,
        }) => Entry::Leaf {
            position,
            slug,
            key,
            outcome: Outcome::Abandoned,
        }
        .name(),
        Some(Entry::Leaf {
            outcome: Outcome::Done,
            ..
        }) => bail!("cannot prune a retired (DONE) leaf: {name}"),
        Some(Entry::Leaf {
            outcome: Outcome::Abandoned,
            ..
        }) => bail!("leaf is already pruned (ABANDONED): {name}"),
        Some(Entry::Brief) => bail!("cannot prune a brief (briefs are never marked): {name}"),
        Some(Entry::Node { .. }) => bail!(
            "cannot prune a node directory as a leaf (pass the directory itself \
             to prune its subtree): {name}"
        ),
        None => bail!("not a v2 leaf: {name}"),
    };

    let abandoned_path = parent_abs.join(&abandoned_name);
    if abandoned_path.exists() {
        bail!("destination already exists: {}", abandoned_path.display());
    }
    // The `ABANDONED` infix is filename-only — the `# <handle>` header is
    // byte-identical.
    rename_entry(parent_abs, name, &abandoned_name)?;
    Ok(abandoned_path)
}

/// One leaf discovered while planning a subtree prune (see [`prune_subtree`]): a
/// live leaf slated to be marked `ABANDONED`, or an already-`DONE` leaf that will
/// be left untouched and reported as such.
enum PlannedLeaf {
    ToMark { dir: PathBuf, name: String },
    LeftDone { path: PathBuf },
}

/// Mark every *live* leaf under `dir` `ABANDONED`, collecting each already-`DONE`
/// leaf found along the way into `result.left_done` untouched (already-`ABANDONED`
/// leaves are left silently alone — already terminal). **Two-phase**: first
/// [`plan_subtree`] walks the whole subtree read-only, then every leaf slated to
/// be marked is validated *before any of them are mutated* — so a leaf that cannot
/// be marked (its `ABANDONED` name is already taken) fails the whole call with
/// nothing renamed, instead of leaving every leaf visited before it already marked
/// while the operator sees only a trailing rename error. Visits children in the
/// per-level comparator order for a deterministic report.
fn prune_subtree(dir: &Path, result: &mut PruneResult) -> Result<()> {
    let mut plan = Vec::new();
    plan_subtree(dir, &mut plan)?;

    // Validate every leaf slated for marking before mutating any of them — the
    // phase that makes a failure a clean no-op.
    for entry in &plan {
        if let PlannedLeaf::ToMark { dir, name } = entry {
            validate_prunable(dir, name)?;
        }
    }

    for entry in plan {
        match entry {
            PlannedLeaf::ToMark { dir, name } => result.marked.push(prune_one(&dir, &name)?),
            PlannedLeaf::LeftDone { path } => result.left_done.push(path),
        }
    }
    Ok(())
}

/// The read-only first phase of [`prune_subtree`]: recursively collect every leaf
/// in `dir`'s subtree that `leaf-prune` will act on, in the per-level comparator
/// order, without mutating the filesystem.
fn plan_subtree(dir: &Path, plan: &mut Vec<PlannedLeaf>) -> Result<()> {
    let mut entries: Vec<(String, Entry, PathBuf)> = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(parsed) = parse(&name) else { continue };
        let is_dir = match entry.file_type() {
            Ok(t) => t.is_dir(),
            Err(_) => continue,
        };
        let kind_ok = match parsed {
            Entry::Node { .. } => is_dir,
            Entry::Brief | Entry::Leaf { .. } => !is_dir,
        };
        if kind_ok {
            entries.push((name, parsed, entry.path()));
        }
    }
    entries.sort_by_key(|a| sort_key(&a.0));

    for (name, entry, path) in entries {
        match entry {
            Entry::Leaf {
                outcome: Outcome::Live,
                ..
            } => plan.push(PlannedLeaf::ToMark {
                dir: dir.to_path_buf(),
                name,
            }),
            Entry::Leaf {
                outcome: Outcome::Done,
                ..
            } => plan.push(PlannedLeaf::LeftDone { path }),
            Entry::Leaf {
                outcome: Outcome::Abandoned,
                ..
            } => {}
            Entry::Node { .. } => plan_subtree(&path, plan)?,
            Entry::Brief => {}
        }
    }
    Ok(())
}

/// Check that a live leaf `name` (in `dir`) is actually prunable: its `ABANDONED`
/// destination is free. Run over every leaf in scope before [`prune_subtree`]
/// mutates any of them, so a collision partway down a subtree fails the whole call
/// with nothing renamed. (Tracked-ness is *not* checked: an untracked leaf renames
/// perfectly well — see [`crate::tree_rename`].)
fn validate_prunable(dir: &Path, name: &str) -> Result<()> {
    let Some(Entry::Leaf {
        outcome: Outcome::Live,
        position,
        slug,
        key,
    }) = parse(name)
    else {
        bail!("not a live leaf: {name}");
    };
    let abandoned_name = Entry::Leaf {
        position,
        slug,
        key,
        outcome: Outcome::Abandoned,
    }
    .name();
    let abandoned_path = dir.join(&abandoned_name);
    if abandoned_path.exists() {
        bail!("destination already exists: {}", abandoned_path.display());
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// helpers

/// Validate the grove root exists and canonicalise it.
fn canonical_grove_root(grove_root: &Path) -> Result<PathBuf> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    grove_root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", grove_root.display()))
}

/// Resolve a leaf path (absolute, or relative to the grove root) to
/// `(parent_dir, name)`. The resolved entry must be a real **file** under the grove
/// root — a node directory or a foreign path is rejected here for rename safety
/// (the kind/format is then refined by `parse` in the caller).
fn resolve_leaf_file(grove_abs: &Path, leaf_path: &Path) -> Result<(PathBuf, String)> {
    let candidate = if leaf_path.is_absolute() {
        leaf_path.to_path_buf()
    } else {
        grove_abs.join(leaf_path)
    };
    let abs = candidate
        .canonicalize()
        .with_context(|| format!("resolving leaf path {}", candidate.display()))?;
    if !abs.starts_with(grove_abs) {
        bail!(
            "leaf path {} is not under grove root {}",
            abs.display(),
            grove_abs.display()
        );
    }
    if !abs.is_file() {
        // A node directory is the common mistake — name it specifically; any other
        // non-file (a leaf-named directory, a symlink) falls through to the generic
        // guard, which also keeps the rename off anything that is not a real leaf file.
        let n = abs.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if abs.is_dir() && matches!(parse(n), Some(Entry::Node { .. })) {
            bail!(
                "cannot operate on a node directory (lifecycle verbs act on leaves): {}",
                abs.display()
            );
        }
        bail!("not a leaf file: {}", abs.display());
    }
    let parent = abs
        .parent()
        .with_context(|| format!("leaf path {} has no parent", abs.display()))?
        .to_path_buf();
    let name = abs
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| format!("leaf path {} has no filename", abs.display()))?
        .to_string();
    Ok((parent, name))
}

/// Resolve `path` (absolute, or relative to the grove root) to an existing
/// absolute path under the grove root — a leaf **file** or a node **directory**
/// alike ([`leaf_prune`]'s arity, unlike [`resolve_leaf_file`]'s files-only).
fn resolve_entry(grove_abs: &Path, path: &Path) -> Result<PathBuf> {
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        grove_abs.join(path)
    };
    let abs = candidate
        .canonicalize()
        .with_context(|| format!("resolving path {}", candidate.display()))?;
    if !abs.starts_with(grove_abs) {
        bail!(
            "path {} is not under grove root {}",
            abs.display(),
            grove_abs.display()
        );
    }
    Ok(abs)
}

/// The path's final component as an owned `String`.
fn entry_name(path: &Path) -> Result<String> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .with_context(|| format!("path {} has no filename", path.display()))
}

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
    let body = format!(
        "# {name} — brief\n\n## Goal\n\n## Done when\n\n## Decomposition\n\n## Pointers\n\n## Notes\n",
    );
    fs::write(path, body.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

/// Retitle a freshly-decomposed node brief's first-line handle header by appending
/// ` — brief`, rewriting the file in place. Recognises exactly the canonical
/// position-free handle `# <slug>-k<key>` (the form `leaf_add` writes), and is
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

    /// A `.grove/` inside a real git repo. Entries rename whether or not git is
    /// tracking them ([`crate::tree_rename`]); call [`stage_all`] when a test wants
    /// the *tracked* branch (the rename moves git's index entry too).
    fn git_grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path().to_path_buf();
        run_git(&repo, &["init", "-q"]);
        run_git(&repo, &["config", "user.email", "t@example.com"]);
        run_git(&repo, &["config", "user.name", "Test"]);
        let root = repo.join(".grove");
        fs::create_dir_all(&root).unwrap();
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
    /// state in which a rename goes through `git mv` and carries the index along.
    fn stage_all(root: &Path) {
        run_git(root.parent().unwrap(), &["add", "-A"]);
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
        let created = root_init(&wt, "plan").unwrap();
        assert_eq!(name_of(&created[0]), "BRIEF.md");
        assert_eq!(name_of(&created[1]), "01-plan-k1.md");
        let g = wt.join(".grove");
        assert!(g.join("BRIEF.md").is_file());
        assert!(g.join("01-plan-k1.md").is_file());
    }

    // fresh-grove-start-contract: the bootstrap leaf is `requirements` — the
    // human's own words are the session's only input, which is the HITL rule —
    // and the `start` path routes on that kind without a file to peek, so this
    // assertion is the contract, not a detail of the template.
    #[test]
    fn root_init_first_leaf_is_a_requirements_task() {
        let (_t, wt) = worktree();
        let created = root_init(&wt, "plan").unwrap();
        assert!(
            body(&created[1]).contains("**Kind:** requirements"),
            "got {:?}",
            body(&created[1])
        );
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

    // ---- leaf-decompose -----------------------------------------------------

    #[test]
    fn decompose_converts_leaf_file_to_node_dir_preserving_the_key() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-build-k3.md", "build-k3");
        stage_all(&g);
        let (brief, _child) =
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", Some(Kind::Impl)).unwrap();
        // The entity that was leaf k3 becomes node k3 — a directory holding BRIEF.md.
        assert_eq!(name_of(&brief), "BRIEF.md");
        assert_eq!(name_of(brief.parent().unwrap()), "02-build-k3");
        let files = list(&g);
        assert!(
            files.contains(&"02-build-k3".to_string()),
            "node dir present"
        );
        assert!(
            !files.contains(&"02-build-k3.md".to_string()),
            "old leaf file gone"
        );
        assert!(g.join("02-build-k3").is_dir());
    }

    #[test]
    fn decompose_seeds_brief_from_leaf_body_and_appends_brief_suffix() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-build-k3.md", "# build-k3\n\n## Goal\nship it\n");
        stage_all(&g);
        let (brief, _child) =
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", Some(Kind::Impl)).unwrap();
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
        touch(&g, "02-build-k3.md", "build-k3");
        stage_all(&g);
        let (_brief, child) =
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", Some(Kind::Impl)).unwrap();
        assert_eq!(name_of(&child), "01-step-k4.md");
        assert_eq!(name_of(child.parent().unwrap()), "02-build-k3");
        assert!(g.join("02-build-k3").join("01-step-k4.md").is_file());
    }

    #[test]
    fn decompose_first_child_header_is_the_handle_and_carries_the_kind() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-build-k3.md", "build-k3");
        stage_all(&g);
        let (_brief, child) =
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", Some(Kind::Impl)).unwrap();
        let text = body(&child);
        assert!(text.starts_with("# step-k4\n"), "got {text:?}");
        assert!(text.contains("**Kind:** impl"), "got {text:?}");
    }

    #[test]
    fn decompose_first_child_can_be_a_planning_task() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-build-k3.md", "build-k3");
        stage_all(&g);
        let (_brief, child) = leaf_decompose(
            &g,
            Path::new("02-build-k3.md"),
            "design",
            Some(Kind::Planning),
        )
        .unwrap();
        assert!(body(&child).contains("**Kind:** planning"));
    }

    #[test]
    fn decompose_with_no_override_inherits_the_parent_leafs_own_kind() {
        // task-kind-taxonomy: `leaf-decompose` gives the first child the leaf
        // being decomposed's own kind when `--kind` is not given.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-build-k3.md", "# build-k3\n\n**Kind:** research\n");
        stage_all(&g);
        let (_brief, child) =
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", None).unwrap();
        assert!(
            body(&child).contains("**Kind:** research"),
            "got {:?}",
            body(&child)
        );
    }

    #[test]
    fn decompose_override_wins_over_the_parent_leafs_kind() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-build-k3.md", "# build-k3\n\n**Kind:** research\n");
        stage_all(&g);
        let (_brief, child) = leaf_decompose(
            &g,
            Path::new("02-build-k3.md"),
            "step",
            Some(Kind::ReviewImpl),
        )
        .unwrap();
        assert!(
            body(&child).contains("**Kind:** review-impl"),
            "got {:?}",
            body(&child)
        );
    }

    #[test]
    fn decompose_a_nested_leaf_preserves_key_and_grows_a_grandchild() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let build = mknode(&g, "02-build-k1", "build-k1");
        touch(&build, "02-mid-k5.md", "mid-k5");
        stage_all(&g);
        let (brief, child) =
            leaf_decompose(&g, &build.join("02-mid-k5.md"), "first", Some(Kind::Impl)).unwrap();
        assert_eq!(
            name_of(brief.parent().unwrap()),
            "02-mid-k5",
            "key 5 preserved"
        );
        assert_eq!(
            name_of(&child),
            "01-first-k6.md",
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
        touch(&g, "02-DONE-build-k3.md", "build-k3");
        stage_all(&g);
        let err = leaf_decompose(&g, Path::new("02-DONE-build-k3.md"), "x", Some(Kind::Impl))
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
        touch(&g, "02-ABANDONED-build-k3.md", "build-k3");
        stage_all(&g);
        let err = leaf_decompose(
            &g,
            Path::new("02-ABANDONED-build-k3.md"),
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
        touch(&g, "02-build-k3.md", "build-k3");
        stage_all(&g);
        assert!(leaf_decompose(
            &g,
            Path::new("02-build-k3.md"),
            "Bad Slug",
            Some(Kind::Impl)
        )
        .is_err());
        let files = list(&g);
        assert!(
            files.contains(&"02-build-k3.md".to_string()),
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
        touch(&g, "02-build-k3.md", "build-k3");
        stage_all(&g);
        let abs = g.join("02-build-k3.md");
        let (brief, _child) = leaf_decompose(&g, &abs, "step", Some(Kind::Impl)).unwrap();
        assert_eq!(name_of(brief.parent().unwrap()), "02-build-k3");
    }

    #[test]
    fn decompose_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err = leaf_decompose(&missing, Path::new("02-build-k3.md"), "x", Some(Kind::Impl))
            .unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- leaf-retire --------------------------------------------------------

    #[test]
    fn retire_adds_done_infix_keeping_position_and_key() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-add-k4.md", "add-k4");
        stage_all(&g);
        let done = leaf_retire(&g, Path::new("02-add-k4.md")).unwrap();
        assert_eq!(name_of(&done), "02-DONE-add-k4.md");
        let files = list(&g);
        assert!(files.contains(&"02-DONE-add-k4.md".to_string()));
        assert!(
            !files.contains(&"02-add-k4.md".to_string()),
            "old name gone"
        );
    }

    #[test]
    fn retire_does_not_rewrite_the_header_or_body() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-add-k4.md", "# add-k4\n\nbody\n");
        stage_all(&g);
        let done = leaf_retire(&g, Path::new("02-add-k4.md")).unwrap();
        assert_eq!(body(&done), "# add-k4\n\nbody\n", "content byte-identical");
    }

    #[test]
    fn retire_works_on_a_nested_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "02-add-k4.md", "add-k4");
        stage_all(&g);
        let done = leaf_retire(&g, &design.join("02-add-k4.md")).unwrap();
        assert_eq!(name_of(&done), "02-DONE-add-k4.md");
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
        touch(&g, "02-DONE-add-k4.md", "add-k4");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("02-DONE-add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("already"), "got {err}");
    }

    #[test]
    fn retire_refuses_an_abandoned_leaf() {
        // A missing flag must degrade to something harmless, never to the
        // opposite outcome (ADR *pruning*): retiring an abandoned leaf would
        // silently assert the rejected work was finished.
        let (_t, g) = git_grove();
        touch(&g, "02-ABANDONED-add-k4.md", "add-k4");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("02-ABANDONED-add-k4.md")).unwrap_err();
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
        touch(&g, "02-add-k4.md", "add-k4");
        stage_all(&g);
        let abs = g.join("02-add-k4.md");
        let done = leaf_retire(&g, &abs).unwrap();
        assert_eq!(name_of(&done), "02-DONE-add-k4.md");
    }

    #[test]
    fn retire_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err = leaf_retire(&missing, Path::new("02-add-k4.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- lifecycle over untracked leaves (issue #3's root cause) -------------
    //
    // Same defect as `leaf-insert`'s, in the lifecycle verbs: a leaf grown this
    // session is untracked until the enclosing task commits, and `git mv` has no
    // index entry to move. Retiring one is not exotic — a task that grows a leaf
    // and finishes it in the same session hits it head-on.

    #[test]
    fn retire_an_untracked_leaf_added_this_session() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = crate::tree_grow::leaf_add(&g, &g, "ship", Kind::Impl, None).unwrap();
        // No stage_all: `leaf_add` leaves it untracked, by design.
        let done = leaf_retire(&g, &leaf).unwrap();
        assert_eq!(name_of(&done), "01-DONE-ship-k1.md");
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
        let leaf = crate::tree_grow::leaf_add(&g, &g, "big", Kind::Impl, None).unwrap();
        // "The current item proving bigger" — the canonical mid-session decompose.
        let (brief, child) = leaf_decompose(&g, &leaf, "first", None).unwrap();
        assert_eq!(name_of(&brief), "BRIEF.md");
        assert_eq!(name_of(&child), "01-first-k2.md");
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
        let leaf = crate::tree_grow::leaf_add(&g, &g, "dead", Kind::Impl, None).unwrap();
        let result = leaf_prune(&g, &leaf).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "01-ABANDONED-dead-k1.md");
        assert!(!leaf.exists(), "the live name is gone");
    }

    // ---- leaf-prune (ADR *pruning*) ------------------------------------------

    #[test]
    fn prune_leaf_adds_abandoned_infix_keeping_position_and_key() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-add-k4.md", "add-k4");
        stage_all(&g);
        let result = leaf_prune(&g, Path::new("02-add-k4.md")).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-add-k4.md");
        assert!(result.left_done.is_empty());
        let files = list(&g);
        assert!(files.contains(&"02-ABANDONED-add-k4.md".to_string()));
        assert!(
            !files.contains(&"02-add-k4.md".to_string()),
            "old name gone"
        );
    }

    #[test]
    fn prune_leaf_does_not_rewrite_the_header_or_body() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-add-k4.md", "# add-k4\n\nbody\n");
        stage_all(&g);
        let result = leaf_prune(&g, Path::new("02-add-k4.md")).unwrap();
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
        touch(&design, "02-add-k4.md", "add-k4");
        stage_all(&g);
        let result = leaf_prune(&g, &design.join("02-add-k4.md")).unwrap();
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-add-k4.md");
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
        touch(&g, "02-DONE-add-k4.md", "add-k4");
        stage_all(&g);
        let err = leaf_prune(&g, Path::new("02-DONE-add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("DONE"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_an_already_abandoned_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "02-ABANDONED-add-k4.md", "add-k4");
        stage_all(&g);
        let err = leaf_prune(&g, Path::new("02-ABANDONED-add-k4.md")).unwrap_err();
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
        touch(&g, "02-add-k4.md", "add-k4");
        stage_all(&g);
        let abs = g.join("02-add-k4.md");
        let result = leaf_prune(&g, &abs).unwrap();
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-add-k4.md");
    }

    #[test]
    fn prune_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err = leaf_prune(&missing, Path::new("02-add-k4.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- leaf-prune on a node: bulk arity (ADR *pruning*) -------------------

    #[test]
    fn prune_node_marks_every_live_leaf_in_the_subtree() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-a-k3.md", "a-k3");
        touch(&node, "02-b-k4.md", "b-k4");
        stage_all(&g);
        let result = leaf_prune(&g, &node).unwrap();
        let names: Vec<String> = result.marked.iter().map(|p| name_of(p)).collect();
        assert_eq!(names, vec!["01-ABANDONED-a-k3.md", "02-ABANDONED-b-k4.md"]);
        assert!(result.left_done.is_empty());
    }

    #[test]
    fn prune_node_leaves_done_leaves_untouched() {
        // That work really was done — a bulk abandon does not retroactively
        // un-finish it.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-a-k3.md", "a-k3");
        touch(&node, "02-b-k4.md", "b-k4");
        stage_all(&g);
        let result = leaf_prune(&g, &node).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-b-k4.md");
        assert_eq!(result.left_done.len(), 1);
        assert_eq!(name_of(&result.left_done[0]), "01-DONE-a-k3.md");
        // The DONE leaf's name (and so its position and key) is untouched.
        assert!(node.join("01-DONE-a-k3.md").is_file());
    }

    #[test]
    fn prune_node_recurses_into_a_grandchild_node() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let outer = mknode(&g, "01-outer-k1", "outer-k1");
        let inner = mknode(&outer, "01-inner-k2", "inner-k2");
        touch(&inner, "01-deep-k3.md", "deep-k3");
        stage_all(&g);
        let result = leaf_prune(&g, &outer).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "01-ABANDONED-deep-k3.md");
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
        touch(&node, "01-a-k3.md", "a-k3");
        touch(&node, "02-b-k4.md", "b-k4");
        stage_all(&g); // a and b are tracked
        touch(&node, "03-c-k5.md", "c-k5"); // c is not

        let result = leaf_prune(&g, &node).unwrap();

        assert_eq!(result.marked.len(), 3, "every live leaf marked");
        let names = list(&node);
        for expected in [
            "01-ABANDONED-a-k3.md",
            "02-ABANDONED-b-k4.md",
            "03-ABANDONED-c-k5.md",
        ] {
            assert!(
                names.contains(&expected.to_string()),
                "missing {expected} (names: {names:?})"
            );
        }
    }

    #[test]
    fn prune_node_is_atomic_bails_clean_on_a_taken_destination() {
        // ADR *pruning*: a rename failure partway through the subtree walk must not
        // leave earlier leaves already marked while the operator sees only the
        // trailing error. The two-phase validate-before-mutate walk is what prevents
        // it; a leaf whose `ABANDONED` name is already taken (a botched earlier
        // prune) is the repro. Without the up-front validation the first two leaves
        // would already be renamed by the time the third one's rename failed.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-a-k3.md", "a-k3");
        touch(&node, "02-b-k4.md", "b-k4");
        touch(&node, "03-c-k5.md", "c-k5");
        // c's destination is already occupied — the one precondition left.
        touch(&node, "03-ABANDONED-c-k5.md", "c-k5");
        stage_all(&g);

        let err = leaf_prune(&g, &node).unwrap_err();
        assert!(
            err.to_string().contains("destination already exists"),
            "got {err}"
        );

        // Nothing was mutated: every live name is untouched, none newly marked.
        let names = list(&node);
        assert!(names.contains(&"01-a-k3.md".to_string()), "got {names:?}");
        assert!(names.contains(&"02-b-k4.md".to_string()), "got {names:?}");
        assert!(names.contains(&"03-c-k5.md".to_string()), "got {names:?}");
        assert!(
            !names.contains(&"01-ABANDONED-a-k3.md".to_string())
                && !names.contains(&"02-ABANDONED-b-k4.md".to_string()),
            "a validation failure must leave the whole subtree untouched: {names:?}"
        );
    }

    #[test]
    fn prune_node_with_nothing_live_marks_nothing() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-a-k3.md", "a-k3");
        stage_all(&g);
        let result = leaf_prune(&g, &node).unwrap();
        assert!(result.marked.is_empty());
        assert_eq!(result.left_done.len(), 1);
    }

    #[test]
    fn prune_refuses_the_grove_root() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        stage_all(&g);
        let err = leaf_prune(&g, &g).unwrap_err();
        assert!(err.to_string().contains("grove root"), "got {err}");
        // Nothing was touched.
        assert!(g.join("01-a-k1.md").is_file());
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

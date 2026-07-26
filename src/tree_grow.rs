// The **v2 grow verbs** (task-tree-scheme) — `leaf-add` and `leaf-insert` —
// re-expressed against the real **directory tree**, built on the 11.1 id model
// (`src/tree_id.rs`). Keeps task-tree-scheme's *semantics* (append a gapless child with a
// fresh permanent key; insert shifts the occupant + later siblings up by one) and
// changes the *mechanics* to the filesystem's shape:
//
//   * v1-flat baked the whole dotted path into every filename, so inserting at
//     `2.2` rewrote `2.2.1`→`2.3.1`… across the **whole subtree** — O(subtree)
//     filename + header rewrites.
//   * v2 carries the hierarchy in directories (a node is a *directory* holding
//     `BRIEF.md` + children), so a renumber is a single **rename of a directory**
//     and the subtree — child names *and* keys — rides along untouched. The shift
//     is O(siblings at one level), the "cascade collapse" task-tree-scheme celebrates.
//
// **Position-free headers (decided 11.3, with the post-flip grilling).** A task
// file's first-line `# …` header is the *stable handle* `# <slug>-k<key>` — the
// per-level position `NN` lives only in the filename, never in the body. This is
// the faithful realization of task-tree-scheme §5 ("reference a work item by `<slug>-k<key>`,
// never by its position/path") and it makes the renumber a **pure rename with
// zero content rewrites**: shifting `05-mid-k14/`→`06-mid-k14/` changes one
// directory name and nothing else — the moved node's own `BRIEF.md` header
// (`# mid-k14 — brief`) and every descendant file stay byte-identical. (Carried to
// 11.4's migration, which rewrites v1 `# <dotted>-[<key>]-<slug>` headers down to
// the position-free handle, and to 11.5's prose.)
//
// Built **isolated**, mirroring `tree_id` / `tree_read`: this module does NOT
// touch the live v1 verb path (`leaf_grow` and the `llm_cli` dispatch to it),
// which keeps speaking the flat scheme until the user-gated re-flip (11.6). So no
// leaf in this node can break the v1-flat grove that is driving itself.

use crate::harness::Harness;
use crate::leaf::Kind;
use crate::tree_id::{next_key, parse, validate_slug, Entry, Outcome};
use crate::tree_rename::rename_entry;
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Append a child leaf under the node directory `parent_dir` at the next gapless
/// per-level position, with a fresh permanent key. `parent_dir` is the grove root
/// (a root-level leaf) or a node directory under it (absolute, or relative to the
/// grove root). The new position is `max(direct-child position) + 1` (counting
/// leaves live AND `DONE`, and node directories, so a slot is never reused), the
/// new key is `max(key over the whole tree) + 1`. Working-tree only — no commit.
///
/// `harness` is the optional per-leaf routing declaration (`leaf-harness-k15`):
/// `Some` writes a `**Harness:**` line beside `**Kind:**`, `None` — the
/// overwhelmingly common case — writes no line at all.
pub fn leaf_add(
    grove_root: &Path,
    parent_dir: &Path,
    slug: &str,
    kind: Kind,
    harness: Option<&'static Harness>,
) -> Result<PathBuf> {
    validate_slug(slug)?;
    let grove_abs = canonical_grove_root(grove_root)?;
    let parent_abs = resolve_parent_node(&grove_abs, parent_dir)?;

    let position = next_child_position(&parent_abs)?;
    let key = next_key(collect_all_names(&grove_abs)?);
    let entry = Entry::Leaf {
        position,
        slug: slug.to_string(),
        key,
        outcome: Outcome::Live,
    };
    let path = parent_abs.join(entry.name());
    if path.exists() {
        bail!("destination already exists: {}", path.display());
    }
    write_template(&path, slug, key, kind, harness)?;
    Ok(path)
}

/// One entry of a `leaf_insert` renumber: an existing sibling whose per-level
/// position shifted up by one. The key and slug (and, for a node, its whole
/// subtree) are invariant — only the `NN` in this one entry's own name changes —
/// so a `Renumber` records just the position move and the names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Renumber {
    pub old_position: u32,
    pub new_position: u32,
    pub old_name: String,
    pub new_name: String,
}

/// Insert a new leaf at the slot currently held by `target`, shifting `target`
/// and every later sibling in its directory up by one position. `target` is an
/// existing entry — a leaf file or a node directory (absolute, or relative to the
/// grove root). Each shift is a single rename whose source and destination differ
/// only in the leading `NN`; a node directory carries its whole subtree along.
/// Renames run **highest-position-first** so each destination is already vacated.
/// The new leaf gets a fresh key. (Inserting past the last sibling is `leaf_add`'s
/// job — `target` must exist.) Working-tree only — no commit; siblings grown this
/// session are untracked and rename fine ([`crate::tree_rename`]).
///
/// Returns the new leaf's path and the renumber log (ascending by new position);
/// pass the log to [`surface_cross_refs`] to lint stray position-prefixed refs.
pub fn leaf_insert(
    grove_root: &Path,
    target: &Path,
    slug: &str,
    kind: Kind,
    harness: Option<&'static Harness>,
) -> Result<(PathBuf, Vec<Renumber>)> {
    validate_slug(slug)?;
    let grove_abs = canonical_grove_root(grove_root)?;
    let target_abs = resolve_under_root(&grove_abs, target)?;

    let target_name = file_name(&target_abs)?;
    let target_entry = parse(&target_name)
        .with_context(|| format!("target is not a grove entry: {target_name}"))?;
    let pos = target_entry
        .position()
        .with_context(|| format!("cannot insert at the brief: {}", target_abs.display()))?;
    let parent_abs = target_abs
        .parent()
        .with_context(|| format!("target {} has no parent", target_abs.display()))?
        .to_path_buf();

    // The new leaf's fresh key is stable across the renumber (renames preserve
    // keys), so it is computed once over the pre-renumber tree.
    let new_key = next_key(collect_all_names(&grove_abs)?);

    // Every positioned sibling at or after the target's slot shifts up by one.
    let mut affected: Vec<(Entry, PathBuf)> = read_children(&parent_abs)?
        .into_iter()
        .filter(|(e, _)| e.position().is_some_and(|p| p >= pos))
        .collect();
    // Highest position first so each rename's destination is already vacated:
    // a sibling's destination differs from its source only in `NN`, where it is
    // larger, so it sorts later and was moved first (collision-free).
    affected.sort_by_key(|b| std::cmp::Reverse(b.0.position()));

    let mut renumbers = Vec::with_capacity(affected.len());
    for (entry, path) in &affected {
        let old_position = entry.position().expect("positioned by the filter");
        let new_position = old_position + 1;
        let old_name = file_name(path)?;
        let new_name = bumped(entry, new_position).name();
        rename_entry(&parent_abs, &old_name, &new_name)?;
        renumbers.push(Renumber {
            old_position,
            new_position,
            old_name,
            new_name,
        });
    }
    // Report the log ascending by new position (the renames ran highest-first).
    renumbers.sort_by_key(|r| r.new_position);

    let entry = Entry::Leaf {
        position: pos,
        slug: slug.to_string(),
        key: new_key,
        outcome: Outcome::Live,
    };
    let path = parent_abs.join(entry.name());
    if path.exists() {
        bail!(
            "destination already exists after renumber: {} (renumber log: {:?})",
            path.display(),
            renumbers
        );
    }
    write_template(&path, slug, new_key, kind, harness)?;
    Ok((path, renumbers))
}

/// Surface stray **position-prefixed** cross-references left stale by a
/// `leaf_insert` renumber, as a lint on stderr — never an auto-rewrite (task-tree-scheme
/// §5: durable references should use the stable `<slug>-k<key>` handle, which a
/// renumber never changes, so the operator reviews each occurrence). Scans every
/// `.md` body in the tree (recursively) for the **old on-disk name** of each
/// renamed entry (`05-mid-k14`), emitting one `path:line: <old-name> (context)`
/// per hit. A stable `<slug>-k<key>` reference is *not* surfaced (it did not move);
/// only the position-prefixed form is stale. Empty renumber log ⇒ nothing to do.
pub fn surface_cross_refs(
    grove_root: &Path,
    renumbers: &[Renumber],
    out: &mut impl std::io::Write,
) -> Result<()> {
    if renumbers.is_empty() {
        return Ok(());
    }
    // The stale tokens are the *old* position-prefixed names the renumber moved
    // (`02-mid-k3`), with any `.md` extension dropped so a path reference
    // `02-mid-k3/01-x-k4.md` matches the directory token. The `-k<digits>` tail
    // makes these specific enough to scan as plain substrings.
    let stale: Vec<String> = renumbers
        .iter()
        .map(|r| stem(&r.old_name).to_string())
        .collect();

    let mut files = Vec::new();
    collect_md_files(grove_root, &mut files)?;
    files.sort();

    for path in &files {
        let body = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        for (idx, line) in body.lines().enumerate() {
            for token in &stale {
                if line.contains(token) {
                    writeln!(
                        out,
                        "{}:{}: {} ({})",
                        path.display(),
                        idx + 1,
                        token,
                        line.trim()
                    )
                    .ok();
                }
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// helpers

/// Validate the grove root exists and canonicalise it (so child paths resolve
/// against a real absolute root).
fn canonical_grove_root(grove_root: &Path) -> Result<PathBuf> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    grove_root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", grove_root.display()))
}

/// Resolve a `parent_dir` argument (the grove root, or a node directory under it,
/// absolute or grove-root-relative) to an absolute, validated node directory. The
/// grove root is always a valid parent (the root node); any other parent must be
/// an existing directory whose name parses as a node *and* that holds a `BRIEF.md`
/// — the v2 translation of v1's "no brief at that position" guard, which also
/// rejects pointing at a leaf *file* (a leaf has no children until decomposed).
fn resolve_parent_node(grove_abs: &Path, parent_dir: &Path) -> Result<PathBuf> {
    let candidate = if parent_dir.is_absolute() {
        parent_dir.to_path_buf()
    } else {
        grove_abs.join(parent_dir)
    };
    // A missing path or a leaf *file* is not a node — both fail `is_dir`, giving
    // the same actionable "parent" diagnostic (rather than a raw canonicalise error).
    if !candidate.is_dir() {
        bail!(
            "parent is not a node directory (need a directory with a BRIEF.md): {}",
            candidate.display()
        );
    }
    let parent_abs = candidate
        .canonicalize()
        .with_context(|| format!("resolving parent {}", candidate.display()))?;
    if !parent_abs.starts_with(grove_abs) {
        bail!(
            "parent {} is not under grove root {}",
            parent_abs.display(),
            grove_abs.display()
        );
    }
    if parent_abs == *grove_abs {
        return Ok(parent_abs); // the root node — always valid
    }
    let name = file_name(&parent_abs)?;
    if !matches!(parse(&name), Some(Entry::Node { .. })) || !parent_abs.join("BRIEF.md").is_file() {
        bail!(
            "parent is not a node directory (need a directory with a BRIEF.md): {}",
            parent_abs.display()
        );
    }
    Ok(parent_abs)
}

/// Resolve a path argument (absolute, or relative to the grove root) to an
/// existing absolute path under the grove root.
fn resolve_under_root(grove_abs: &Path, path: &Path) -> Result<PathBuf> {
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

/// The next gapless per-level position in `dir`: `max(position) + 1` over its
/// direct children (leaves live and `DONE`, and node directories — a `DONE` slot
/// is never reused), or `1` for an empty node. Reconciles each name's parsed kind
/// with its real filesystem kind, so a mis-kinded foreign entry is not counted.
fn next_child_position(dir: &Path) -> Result<u32> {
    let max = read_children(dir)?
        .iter()
        .filter_map(|(e, _)| e.position())
        .max();
    Ok(max.map_or(1, |m| m + 1))
}

/// Read one directory's grove children — `(Entry, path)` for each name that parses
/// *and* whose real filesystem kind matches (a node is a directory; a leaf/brief is
/// a file), sorted by the per-level comparator. The charter brief is included
/// (callers filter it via `position()` returning `None`). Mirrors `tree_read`'s
/// reconciliation so grow and read agree on what is a sibling.
fn read_children(dir: &Path) -> Result<Vec<(Entry, PathBuf)>> {
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
    entries.sort_by_key(|a| crate::tree_id::sort_key(&a.0));
    Ok(entries.into_iter().map(|(_, e, p)| (e, p)).collect())
}

/// Recursively collect every grove entry's name in the tree (leaves live and
/// `DONE`, node directories, and briefs), descending node directories. Feeds
/// `next_key`, which filters to the keyed names — so this only needs every name.
fn collect_all_names(dir: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    collect_names_into(dir, &mut names)?;
    Ok(names)
}

fn collect_names_into(dir: &Path, out: &mut Vec<String>) -> Result<()> {
    for (entry, path) in read_children(dir)? {
        let descend = entry.is_node();
        if let Entry::Brief = entry {
            out.push("BRIEF.md".to_string());
        } else {
            out.push(file_name(&path)?);
        }
        if descend {
            collect_names_into(&path, out)?;
        }
    }
    Ok(())
}

/// Recursively collect every `.md` file path under `dir` (every directory level's
/// briefs and leaves) — the bodies `surface_cross_refs` scans.
fn collect_md_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        match entry.file_type() {
            Ok(t) if t.is_dir() => collect_md_files(&path, out)?,
            Ok(t) if t.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") => {
                out.push(path)
            }
            _ => {}
        }
    }
    Ok(())
}

/// Rebuild a positioned entry with a new per-level position; key and slug (and, for
/// a node, its whole subtree) are invariant.
fn bumped(entry: &Entry, new_position: u32) -> Entry {
    match entry {
        Entry::Leaf {
            slug, key, outcome, ..
        } => Entry::Leaf {
            position: new_position,
            slug: slug.clone(),
            key: *key,
            outcome: *outcome,
        },
        Entry::Node { slug, key, .. } => Entry::Node {
            position: new_position,
            slug: slug.clone(),
            key: *key,
        },
        Entry::Brief => Entry::Brief, // unreachable: briefs are never renumbered
    }
}

/// The path's final component as an owned `String`.
fn file_name(path: &Path) -> Result<String> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .with_context(|| format!("path {} has no filename", path.display()))
}

/// Drop a trailing `.md` from a name (a leaf's on-disk name) so a directory-token
/// reference matches; a node directory's name has no extension and is returned as-is.
fn stem(name: &str) -> &str {
    name.strip_suffix(".md").unwrap_or(name)
}

/// Write a freshly-created leaf's template. The first-line header is the
/// **position-free handle** `# <slug>-k<key>` (decided 11.3) — the mutable per-level
/// position lives only in the filename, so a later renumber never rewrites this.
///
/// A declared `harness` adds one `**Harness:**` line immediately under
/// `**Kind:**` — the two routing facts read as one metadata block, and
/// `tree_read` finds either by its own marker wherever it sits. An undeclared
/// harness writes **no line**, not an empty one: the peek treats an empty
/// `**Harness:**` line as an unfinished declaration and refuses to launch, so a
/// template that emitted one would break every leaf grove creates.
fn write_template(
    path: &Path,
    slug: &str,
    key: u32,
    kind: Kind,
    harness: Option<&'static Harness>,
) -> Result<()> {
    let kind_label = kind.label();
    let harness_line = match harness {
        Some(h) => format!("**Harness:** {}\n", h.name),
        None => String::new(),
    };
    let body = format!(
        "# {slug}-k{key}\n\n**Kind:** {kind_label}\n{harness_line}\n## Goal\n\n## Context\n\n## Done when\n\n## Notes\n",
    );
    fs::write(path, body.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    /// A fresh `.grove/` directory (no git — `leaf_add` only writes a file).
    fn grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
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

    /// Write a leaf/brief stub file with a position-free `# <handle>` header.
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

    /// The names git has **in its index** for the grove, lexically sorted. Distinct
    /// from [`list`] (what is on disk): a rename that went through `git mv` moves the
    /// index entry and shows here under the new name; a plain rename of an untracked
    /// file shows in neither index state.
    fn indexed(root: &Path) -> Vec<String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["ls-files", "--", "."])
            .output()
            .unwrap();
        let mut v: Vec<String> = String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();
        v.sort();
        v
    }

    // ---- leaf-add -----------------------------------------------------------

    #[test]
    fn add_root_level_child_gets_position_01_and_first_key() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "survey", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "01-survey-k1.md");
    }

    #[test]
    fn add_appends_gapless_after_existing_root_children() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        touch(&g, "02-b-k2.md", "b-k2");
        let got = leaf_add(&g, &g, "c", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "03-c-k3.md");
    }

    #[test]
    fn add_child_under_a_node_appends_after_existing_children() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "02-y-k4.md");
        assert_eq!(name_of(got.parent().unwrap()), "02-build-k2");
    }

    #[test]
    fn add_first_child_under_a_childless_node() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        let got = leaf_add(&g, &node, "first", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "01-first-k3.md");
    }

    #[test]
    fn add_assigns_fresh_key_as_max_over_whole_tree_plus_one() {
        // Keys are global, not per-node: the new key is max(key) + 1 across the
        // whole tree, including a deeper subtree's higher key.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "01-deep-k7.md", "deep-k7"); // a high key in another subtree
        let build = mknode(&g, "02-build-k2", "build-k2");
        let got = leaf_add(&g, &build, "y", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "01-y-k8.md");
    }

    #[test]
    fn add_counts_done_children_so_a_retired_slot_is_never_reused() {
        // A `DONE` child still occupies its position — the next child is 02, not 01.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "02-y-k4.md");
    }

    #[test]
    fn add_counts_abandoned_children_so_a_pruned_slot_is_never_reused() {
        // Symmetric with DONE: an `ABANDONED` child still occupies its position
        // and its key (ADR *pruning*'s key-reuse defect) — the next child is 02
        // and its key is 4, not a reused 3.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-ABANDONED-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "02-y-k4.md");
    }

    #[test]
    fn add_counts_node_dir_siblings_when_numbering() {
        // A sibling that is a node *directory* still occupies its position.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        mknode(&g, "01-design-k1", "design-k1");
        let got = leaf_add(&g, &g, "build", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "02-build-k2.md");
    }

    #[test]
    fn add_writes_position_free_header_and_kind() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "survey", Kind::Impl, None).unwrap();
        let text = body(&got);
        assert!(
            text.starts_with("# survey-k1\n"),
            "header is the position-free handle; got {text:?}"
        );
        assert!(text.contains("**Kind:** impl"), "got {text:?}");
    }

    #[test]
    fn add_planning_kind_writes_planning_label() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "design", Kind::Planning, None).unwrap();
        assert!(body(&got).contains("**Kind:** planning"));
    }

    #[test]
    fn add_errors_when_parent_node_is_missing() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let missing = g.join("09-nope-k9");
        let err = leaf_add(&g, &missing, "y", Kind::Impl, None).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn add_errors_when_parent_is_a_leaf_file_not_a_node() {
        // A leaf is a *file* — you must decompose it into a node before adding under it.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = touch(&g, "02-build-k2.md", "build-k2");
        let err = leaf_add(&g, &leaf, "y", Kind::Impl, None).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn add_errors_when_node_dir_has_no_brief() {
        // A bare directory without a BRIEF.md is not a node.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let bare = g.join("02-bare-k2");
        fs::create_dir_all(&bare).unwrap();
        let err = leaf_add(&g, &bare, "y", Kind::Impl, None).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn add_errors_on_invalid_slug() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        assert!(leaf_add(&g, &g, "BRIEF", Kind::Impl, None).is_err());
        assert!(leaf_add(&g, &g, "Bad Slug", Kind::Impl, None).is_err());
    }

    #[test]
    fn add_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = leaf_add(&missing, &missing, "y", Kind::Impl, None).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- leaf-insert --------------------------------------------------------

    #[test]
    fn insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        touch(&g, "02-b-k2.md", "b-k2");
        touch(&g, "03-c-k3.md", "c-k3");
        stage_all(&g);
        let (path, _renums) =
            leaf_insert(&g, &g.join("02-b-k2.md"), "new", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&path), "02-new-k4.md"); // fresh key, not a reused one
        let files = list(&g);
        assert!(
            files.contains(&"01-a-k1.md".to_string()),
            "pos 1 < 2, unchanged"
        );
        assert!(
            files.contains(&"02-new-k4.md".to_string()),
            "the inserted leaf"
        );
        assert!(
            files.contains(&"03-b-k2.md".to_string()),
            "b: 02->03, key 2 kept"
        );
        assert!(
            files.contains(&"04-c-k3.md".to_string()),
            "c: 03->04, key 3 kept"
        );
        assert!(!files.contains(&"02-b-k2.md".to_string()), "old name gone");
    }

    #[test]
    fn insert_cascades_a_sibling_node_subtree_riding_along_byte_identical() {
        // The headline (and the leaf's Notes requirement): inserting ahead of a
        // sibling *node* shifts only that node's own dir name — its BRIEF.md and
        // every grandchild stay byte-identical, the cascade collapse.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let mid = mknode(&g, "02-mid-k3", "mid-k3");
        let grandchild = touch_body(
            &mid,
            "01-x-k4.md",
            "# x-k4\n\n**Kind:** impl\n\n## Goal\nstuff\n",
        );
        let grandchild_before = body(&grandchild);
        let brief_before = body(&mid.join("BRIEF.md"));
        stage_all(&g);
        let (path, renums) =
            leaf_insert(&g, &g.join("02-mid-k3"), "new", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&path), "02-new-k5.md");
        // The node shifted 02 -> 03; only its ancestor dir name changed.
        let shifted = g.join("03-mid-k3");
        assert!(shifted.is_dir(), "node dir shifted to 03-mid-k3");
        assert!(!g.join("02-mid-k3").exists(), "old node dir name gone");
        assert_eq!(
            body(&shifted.join("BRIEF.md")),
            brief_before,
            "the moved node's BRIEF.md is byte-identical"
        );
        assert_eq!(
            body(&shifted.join("01-x-k4.md")),
            grandchild_before,
            "the grandchild is byte-identical — only the ancestor dir name moved"
        );
        assert_eq!(renums.len(), 1, "exactly one sibling (the node) shifted");
        assert_eq!(renums[0].old_position, 2);
        assert_eq!(renums[0].new_position, 3);
    }

    #[test]
    fn insert_writes_position_free_header_for_the_new_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        stage_all(&g);
        let (path, _r) = leaf_insert(&g, &g.join("01-a-k1.md"), "head", Kind::Impl, None).unwrap();
        let text = body(&path);
        assert!(text.starts_with("# head-k2\n"), "got {text:?}");
        assert!(text.contains("**Kind:** impl"), "got {text:?}");
    }

    #[test]
    fn insert_does_not_rewrite_any_existing_file_contents() {
        // Position-free headers ⇒ a shifted leaf's body is byte-identical too.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let b = touch_body(&g, "02-b-k2.md", "# b-k2\n\nbody text\n");
        let b_before = body(&b);
        stage_all(&g);
        leaf_insert(&g, &g.join("02-b-k2.md"), "new", Kind::Impl, None).unwrap();
        assert_eq!(
            body(&g.join("03-b-k2.md")),
            b_before,
            "shifted leaf content byte-identical; only NN in the filename changed"
        );
    }

    #[test]
    fn insert_collision_free_for_a_dense_run_of_siblings() {
        // Stress the highest-first ordering: insert at the head of five siblings;
        // a wrong order would make a rename collide and lose a file.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        for i in 1..=5 {
            touch(&g, &format!("{i:02}-s{i}-k{i}.md"), &format!("s{i}-k{i}"));
        }
        stage_all(&g);
        let (path, renums) =
            leaf_insert(&g, &g.join("01-s1-k1.md"), "head", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&path), "01-head-k6.md");
        assert_eq!(renums.len(), 5);
        let leaves: Vec<String> = list(&g).into_iter().filter(|n| n != "BRIEF.md").collect();
        assert_eq!(
            leaves,
            vec![
                "01-head-k6.md",
                "02-s1-k1.md",
                "03-s2-k2.md",
                "04-s3-k3.md",
                "05-s4-k4.md",
                "06-s5-k5.md",
            ],
            "all six leaves present, gapless 01..06 — no file lost to a collision"
        );
    }

    #[test]
    fn insert_returns_renumber_log_ascending_by_new_position() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        touch(&g, "02-b-k2.md", "b-k2");
        touch(&g, "03-c-k3.md", "c-k3");
        stage_all(&g);
        let (_path, renums) =
            leaf_insert(&g, &g.join("01-a-k1.md"), "head", Kind::Impl, None).unwrap();
        let positions: Vec<(u32, u32)> = renums
            .iter()
            .map(|r| (r.old_position, r.new_position))
            .collect();
        assert_eq!(positions, vec![(1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn insert_inside_a_nested_node_shifts_only_that_levels_siblings() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "01-a-k2.md", "a-k2");
        touch(&design, "02-b-k3.md", "b-k3");
        stage_all(&g);
        let (path, renums) =
            leaf_insert(&g, &design.join("01-a-k2.md"), "first", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&path), "01-first-k4.md");
        assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
        let children = list(&design);
        assert!(children.contains(&"01-first-k4.md".to_string()));
        assert!(children.contains(&"02-a-k2.md".to_string()));
        assert!(children.contains(&"03-b-k3.md".to_string()));
        assert_eq!(renums.len(), 2);
    }

    #[test]
    fn insert_errors_on_invalid_slug() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        stage_all(&g);
        assert!(leaf_insert(&g, &g.join("01-a-k1.md"), "BRIEF", Kind::Impl, None).is_err());
    }

    #[test]
    fn insert_errors_when_target_is_a_brief() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        let err = leaf_insert(&g, &g.join("BRIEF.md"), "x", Kind::Impl, None).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn insert_errors_when_target_missing() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        assert!(leaf_insert(&g, &g.join("09-nope-k9.md"), "x", Kind::Impl, None).is_err());
    }

    #[test]
    fn insert_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err =
            leaf_insert(&missing, &missing.join("01-a-k1.md"), "x", Kind::Impl, None).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- insert over untracked entries (issue #3) ----------------------------
    //
    // The grow verbs are working-tree-only by design — `leaf_add` writes an
    // *untracked* file and the enclosing task's commit folds it in. So the
    // ordinary rhythm of a planning session (grow several leaves, then realise
    // one must sequence earlier) hands `leaf_insert` siblings that are not in
    // git's index. Renaming those is `fs::rename`'s job, not `git mv`'s.

    #[test]
    fn insert_ahead_of_an_untracked_sibling_added_this_session() {
        // Issue #3 verbatim: `leaf_add` then `leaf_insert` ahead of what it made,
        // with no `git add` in between — the ordinary planning-session sequence.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let release = leaf_add(&g, &g, "release", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&release), "01-release-k1.md");

        // No stage_all: the leaf is untracked, exactly as `leaf_add` left it.
        let (path, renums) = leaf_insert(&g, &release, "review", Kind::Impl, None).unwrap();

        assert_eq!(name_of(&path), "01-review-k2.md");
        let files = list(&g);
        assert!(
            files.contains(&"02-release-k1.md".to_string()),
            "the untracked sibling shifted 01->02, key preserved (files: {files:?})"
        );
        assert!(
            !files.contains(&"01-release-k1.md".to_string()),
            "old name gone (files: {files:?})"
        );
        assert_eq!(renums.len(), 1, "one sibling shifted");
    }

    #[test]
    fn insert_renumbers_a_mix_of_tracked_and_untracked_siblings() {
        // The realistic mid-session tree: some leaves committed by earlier tasks,
        // some grown just now. One insert must renumber straight through both.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        touch(&g, "02-b-k2.md", "b-k2");
        stage_all(&g); // a and b are tracked
        touch(&g, "03-c-k3.md", "c-k3"); // c is not

        let (path, renums) =
            leaf_insert(&g, &g.join("01-a-k1.md"), "new", Kind::Impl, None).unwrap();

        assert_eq!(name_of(&path), "01-new-k4.md");
        let files = list(&g);
        for expected in ["02-a-k1.md", "03-b-k2.md", "04-c-k3.md"] {
            assert!(
                files.contains(&expected.to_string()),
                "every sibling shifted up one, tracked or not: missing {expected} (files: {files:?})"
            );
        }
        assert_eq!(renums.len(), 3, "all three siblings shifted");
    }

    #[test]
    fn insert_moves_the_index_entry_for_a_tracked_sibling() {
        // The other half of the contract: a *tracked* entry still moves through
        // `git mv`, so the rename is staged and the operator's `git status` shows a
        // rename rather than a delete + an untracked file. Guards against
        // "simplify" ing the primitive down to a bare `fs::rename` everywhere.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        stage_all(&g);

        leaf_insert(&g, &g.join("01-a-k1.md"), "new", Kind::Impl, None).unwrap();

        let idx = indexed(&g);
        assert!(
            idx.contains(&"02-a-k1.md".to_string()),
            "the tracked sibling's index entry moved to the new name (index: {idx:?})"
        );
        assert!(
            !idx.contains(&"01-a-k1.md".to_string()),
            "the old index entry is gone — a staged rename, not a stale entry (index: {idx:?})"
        );
    }

    // ---- surface_cross_refs (position-prefixed lint, not auto-rewrite) -------

    fn renum(old: u32, new: u32, old_name: &str, new_name: &str) -> Renumber {
        Renumber {
            old_position: old,
            new_position: new,
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
        }
    }
    fn surfaced(root: &Path, renumbers: &[Renumber]) -> String {
        let mut buf = Vec::new();
        surface_cross_refs(root, renumbers, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn surface_empty_renumbers_emits_nothing() {
        let (_t, g) = grove();
        touch_body(
            &g,
            "01-a-k1.md",
            "# a-k1\n\nrefers to 02-mid-k3 somewhere\n",
        );
        assert_eq!(surfaced(&g, &[]), "");
    }

    #[test]
    fn surface_reports_a_stale_position_prefixed_reference_in_a_body() {
        let (_t, g) = grove();
        touch_body(
            &g,
            "01-a-k1.md",
            "# a-k1\n\nthe design lives at 02-mid-k3/01-x-k4.md\n",
        );
        let out = surfaced(&g, &[renum(2, 3, "02-mid-k3", "03-mid-k3")]);
        assert!(out.contains("01-a-k1.md"), "names the file: {out:?}");
        assert!(out.contains("02-mid-k3"), "shows the stale name: {out:?}");
        assert!(
            out.contains(":3:"),
            "1-based line number of the body ref: {out:?}"
        );
    }

    #[test]
    fn surface_does_not_flag_the_stable_slug_key_handle() {
        // A `<slug>-k<key>` reference is stable across a renumber — never surfaced.
        // Only the position-prefixed `02-mid-k3` form is stale.
        let (_t, g) = grove();
        touch_body(&g, "01-a-k1.md", "# a-k1\n\nsee mid-k3 for the design\n");
        assert_eq!(surfaced(&g, &[renum(2, 3, "02-mid-k3", "03-mid-k3")]), "");
    }

    #[test]
    fn surface_reports_hits_recursively_across_nested_files() {
        let (_t, g) = grove();
        touch_body(
            &g,
            "BRIEF.md",
            "# root — brief\n\nthe plan is at 02-mid-k3\n",
        );
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch_body(&design, "01-a-k2.md", "# a-k2\n\nalso 02-mid-k3 here\n");
        let out = surfaced(&g, &[renum(2, 3, "02-mid-k3", "03-mid-k3")]);
        assert!(
            out.contains("BRIEF.md") && out.contains("02-mid-k3"),
            "{out:?}"
        );
        assert!(out.contains("01-a-k2.md"), "nested file surfaced: {out:?}");
    }
}

// The **v2 lifecycle verbs** (task-tree-scheme) — `root-init`, `leaf-decompose`, and
// `leaf-retire` — re-expressed against the real **directory tree**, built on the
// 11.1 id model (`src/tree_id.rs`) and the 11.3 grow verbs (`src/tree_grow.rs`).
// Keeps task-tree-scheme's *semantics* (a fresh grove starts with a planning leaf so it is
// never mistaken for finished; decompose enforces a first child; retire is
// leaves-only and done-ness is marked in place) and changes the *mechanics* to the
// filesystem's shape:
//
//   * `root-init` writes the root `BRIEF.md` (the one unkeyed singleton) and a
//     first planning leaf `01-<slug>-k1.md` — a 2-digit per-level position;
//   * `leaf-decompose` turns the leaf *file* `NN-<slug>-k<key>.md` into a node
//     *directory* `NN-<slug>-k<key>/` (**key preserved** — the entity that was the
//     leaf becomes the node), `git mv`ing the leaf body in as the node's `BRIEF.md`
//     and growing a first child atomically so a node is never childless;
//   * `leaf-retire` adds a `DONE` infix in place (`NN-<slug>-k<key>.md` →
//     `NN-DONE-<slug>-k<key>.md`), keeping the retired leaf in its directory at its
//     position — no `done/` directory.
//
// **Position-free headers (11.3):** a leaf/brief header is the stable handle
// `# <slug>-k<key>` (`# … — brief` for a node), so `leaf-retire` leaves the file's
// content byte-identical (the `DONE` infix is filename-only) and `leaf-decompose`
// only appends ` — brief` to the handle.
//
// Built **isolated**, mirroring `tree_id` / `tree_read` / `tree_grow`: this module
// does NOT touch the live v1 verb path (`leaf_lifecycle` and the `llm_cli` dispatch
// to it), which keeps speaking the flat scheme until the user-gated re-flip (11.6).

use crate::leaf::Kind;
use crate::tree_grow::leaf_add;
use crate::tree_id::{parse, validate_slug, Entry};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// `root-init [<slug>]`: scaffold a fresh grove under `worktree/.grove` — the root
/// `BRIEF.md` (the one unkeyed singleton) and a first **planning** leaf
/// `01-<slug>-k1.md`. Returns the absolute paths created, `BRIEF.md` first then the
/// leaf. Refuses to clobber an existing `.grove/`. Working-tree only — no commit.
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
    let leaf_path = leaf_add(&grove_root, &grove_root, slug, Kind::Planning)?;
    Ok(vec![brief_path, leaf_path])
}

/// `leaf-decompose <leaf-path> <first-child-slug>`: convert a live leaf file
/// `NN-<slug>-k<key>.md` into a node directory `NN-<slug>-k<key>/` (**key
/// preserved**) holding a `BRIEF.md` (seeded from the leaf body, its `# <handle>`
/// header retitled `# <handle> — brief`) and a first child
/// `01-<first-child-slug>-k<new>.md` grown atomically so the node is never
/// childless. `kind` is the first child's kind. Refuses a brief, a node, and an
/// already-`DONE` leaf. Returns `(brief_path, first_child_path)`. Working-tree
/// only — no commit.
pub fn leaf_decompose(
    grove_root: &Path,
    leaf_path: &Path,
    first_child_slug: &str,
    kind: Kind,
) -> Result<(PathBuf, PathBuf)> {
    // Validate the child slug up front, before any filesystem mutation, so a bad
    // slug leaves the leaf un-decomposed (no half-built childless node directory).
    validate_slug(first_child_slug)?;

    let grove_abs = canonical_grove_root(grove_root)?;
    let (parent_abs, name) = resolve_leaf_file(&grove_abs, leaf_path)?;
    let (position, slug, key) = match parse(&name) {
        Some(Entry::Leaf {
            is_done: false,
            position,
            slug,
            key,
        }) => (position, slug, key),
        Some(Entry::Leaf { is_done: true, .. }) => {
            bail!("cannot decompose a retired (DONE) leaf: {name}")
        }
        Some(Entry::Brief) => bail!("cannot decompose a brief (it is already a node): {name}"),
        Some(Entry::Node { .. }) => {
            bail!("cannot decompose a node (it already has children): {name}")
        }
        None => bail!("not a v2 leaf: {name}"),
    };

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

    // `git mv` the leaf file into the new directory as its charter `BRIEF.md`; the
    // leaf body is carried in verbatim, then its `# <handle>` header retitled.
    git_mv(&parent_abs, &name, &format!("{node_name}/BRIEF.md"))?;
    let brief_path = node_dir.join("BRIEF.md");
    append_brief_suffix_in_file(&brief_path, &slug, key)?;

    // Grow the first child at `01` (enforce-first-child) — delegated to `leaf_add`
    // so it is byte-identical to a hand-added child and gets the next fresh key. The
    // node now exists (the BRIEF.md we just created), so the parent guard passes.
    let child_path = leaf_add(&grove_abs, &node_dir, first_child_slug, kind)?;
    Ok((brief_path, child_path))
}

/// `leaf-retire <leaf-path>`: rename a live leaf `NN-<slug>-k<key>.md` →
/// `NN-DONE-<slug>-k<key>.md` in place, keeping its position and key. The `DONE`
/// infix is filename-only — the `# <handle>` header is byte-identical. Refuses a
/// brief, a node directory, and an already-`DONE` leaf. Returns the retired file's
/// absolute path. Working-tree only — no commit.
pub fn leaf_retire(grove_root: &Path, leaf_path: &Path) -> Result<PathBuf> {
    let grove_abs = canonical_grove_root(grove_root)?;
    let (parent_abs, name) = resolve_leaf_file(&grove_abs, leaf_path)?;
    let done_name = match parse(&name) {
        Some(Entry::Leaf {
            is_done: false,
            position,
            slug,
            key,
        }) => Entry::Leaf {
            position,
            slug,
            key,
            is_done: true,
        }
        .name(),
        Some(Entry::Leaf { is_done: true, .. }) => bail!("leaf is already retired (DONE): {name}"),
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
    // The `DONE` infix is filename-only — the `# <handle>` header is byte-identical.
    git_mv(&parent_abs, &name, &done_name)?;
    Ok(done_path)
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
/// root — a node directory or a foreign path is rejected here for `git mv` safety
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
        // guard, which also keeps `git mv` off anything that is not a real leaf file.
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

/// `git mv <src> <dst>` run with `git -C <dir>` (src/dst relative to `dir`).
fn git_mv(dir: &Path, src: &str, dst: &str) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["mv", src, dst])
        .output()
        .with_context(|| format!("running git mv {src} {dst}"))?;
    if !out.status.success() {
        bail!(
            "git mv {src} -> {dst} failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// The grove's name is the worktree directory's basename (it equals the branch
/// name; the worktree lives at `<repo>/.grove-worktrees/<name>/`). Used as the root
/// brief's `# <name> — brief` title.
fn grove_name(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// Emit the minimal section-header scaffold for the root `BRIEF.md` — headers only,
/// no prose (the first planning session fills them). The root brief is the one
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
    /// creates the grove itself and needs no git (it never `git mv`s).
    fn worktree() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let wt = tmp.path().join("my-grove");
        fs::create_dir_all(&wt).unwrap();
        (tmp, wt)
    }

    /// A `.grove/` inside a real git repo — `decompose`/`retire` rename via
    /// `git mv`, which needs tracked files (call [`stage_all`] before operating).
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

    /// Stage everything under the grove so `git mv` sees tracked files.
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

    #[test]
    fn root_init_first_leaf_is_a_planning_task() {
        let (_t, wt) = worktree();
        let created = root_init(&wt, "plan").unwrap();
        assert!(
            body(&created[1]).contains("**Kind:** planning"),
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
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", Kind::Work).unwrap();
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
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", Kind::Work).unwrap();
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
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", Kind::Work).unwrap();
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
            leaf_decompose(&g, Path::new("02-build-k3.md"), "step", Kind::Work).unwrap();
        let text = body(&child);
        assert!(text.starts_with("# step-k4\n"), "got {text:?}");
        assert!(text.contains("**Kind:** work"), "got {text:?}");
    }

    #[test]
    fn decompose_first_child_can_be_a_planning_task() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-build-k3.md", "build-k3");
        stage_all(&g);
        let (_brief, child) =
            leaf_decompose(&g, Path::new("02-build-k3.md"), "design", Kind::Planning).unwrap();
        assert!(body(&child).contains("**Kind:** planning"));
    }

    #[test]
    fn decompose_a_nested_leaf_preserves_key_and_grows_a_grandchild() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let build = mknode(&g, "02-build-k1", "build-k1");
        touch(&build, "02-mid-k5.md", "mid-k5");
        stage_all(&g);
        let (brief, child) =
            leaf_decompose(&g, &build.join("02-mid-k5.md"), "first", Kind::Work).unwrap();
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
        let err = leaf_decompose(&g, &node.join("BRIEF.md"), "x", Kind::Work).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_node_directory() {
        let (_t, g) = git_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        stage_all(&g);
        let err = leaf_decompose(&g, &node, "x", Kind::Work).unwrap_err();
        assert!(err.to_string().contains("node"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_done_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-DONE-build-k3.md", "build-k3");
        stage_all(&g);
        let err =
            leaf_decompose(&g, Path::new("02-DONE-build-k3.md"), "x", Kind::Work).unwrap_err();
        assert!(
            err.to_string().to_lowercase().contains("done") || err.to_string().contains("retired"),
            "got {err}"
        );
    }

    #[test]
    fn decompose_refuses_a_foreign_file() {
        let (_t, g) = git_grove();
        touch(&g, "README.md", "readme");
        stage_all(&g);
        let err = leaf_decompose(&g, Path::new("README.md"), "x", Kind::Work).unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

    #[test]
    fn decompose_rejects_a_bad_child_slug_without_touching_the_leaf() {
        // Atomicity: the child slug is validated BEFORE the git mv, so a bad slug
        // leaves the leaf un-decomposed (no half-built node directory).
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-build-k3.md", "build-k3");
        stage_all(&g);
        assert!(leaf_decompose(&g, Path::new("02-build-k3.md"), "Bad Slug", Kind::Work).is_err());
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
        let (brief, _child) = leaf_decompose(&g, &abs, "step", Kind::Work).unwrap();
        assert_eq!(name_of(brief.parent().unwrap()), "02-build-k3");
    }

    #[test]
    fn decompose_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err =
            leaf_decompose(&missing, Path::new("02-build-k3.md"), "x", Kind::Work).unwrap_err();
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
}

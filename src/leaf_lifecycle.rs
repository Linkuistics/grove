// The new-format **lifecycle verbs** — `root-init`, `leaf-decompose`, and
// `leaf-retire` — built on the 010 id model (`src/leaf_id.rs`) and the 030 grow
// verbs (`src/leaf_grow.rs`). Implements ADR-0033 and the 050 BRIEF running log
// D3 (`.DONE` marker), D4 (decompose enforces a first child; retire is
// leaves-only; node-done is implicit) and D6 (decompose **preserves** the node's
// key).
//
// The structural shift from the old directory-based verbs (`src/root_init.rs`,
// `src/leaf_ops.rs`): the new scheme is **flat** (every task is a file directly
// in `.grove/`, no node directories and no `done/`), so:
//   * `root-init` writes the root `BRIEF.md` (the one unkeyed singleton) and a
//     first planning leaf `1-[1]-<slug>.md` — `1-`, not the old `010-`;
//   * `leaf-decompose` no longer creates a directory: it flips the leaf's
//     filename marker in place (`<id>-[k]-<slug>.md` → `…BRIEF.md`, **key
//     preserved** — the entity that was `[k]` becomes node `[k]`), and grows the
//     first child `<id>.1-[new]-<slug>.md` atomically so a node is never
//     childless (D4 / ADR-0011 generalised to every node);
//   * `leaf-retire` no longer moves into `done/`: it renames in place, appending
//     `.DONE` (`<id>-[k]-<slug>.md` → `…DONE.md`), keeping the retired leaf in the
//     one flat version-sorted list.
//
// Per D9 this module is **new, isolated code**: it does NOT touch the live verb
// path (`src/root_init.rs`, `src/leaf_ops.rs`, `src/llm_cli.rs`), which keeps
// speaking the old `NNN-slug` / `done/` format until leaf 060 wires the
// dual-format dispatch in. So no 050 commit can break this old-format grove
// driving itself.

use crate::leaf::Kind;
use crate::leaf_grow::leaf_add;
use crate::leaf_id::{parse, parse_position, validate_slug, LeafId};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// `root-init [<slug>]`: scaffold a fresh grove under `worktree/.grove` — the
/// root `BRIEF.md` (the one unkeyed singleton) and a first **planning** leaf
/// `1-[1]-<slug>.md`. Returns the absolute paths created, `BRIEF.md` first then
/// the leaf. Refuses to clobber an existing `.grove/`. Working-tree change only —
/// no commit. The first leaf is delegated to [`leaf_add`] so it is byte-identical
/// to one the LLM would later add by hand (no template drift).
pub fn root_init(worktree: &Path, slug: &str) -> Result<Vec<PathBuf>> {
    // Validate before touching the filesystem so a bad slug never leaves a stray
    // `.grove/` behind (parity with the old verb).
    validate_slug(slug)?;

    let grove_root = worktree.join(".grove");
    if grove_root.exists() {
        bail!("grove root already exists: {}", grove_root.display());
    }
    fs::create_dir_all(&grove_root)
        .with_context(|| format!("creating {}", grove_root.display()))?;

    let brief_path = grove_root.join("BRIEF.md");
    write_root_brief(&brief_path, &grove_name(worktree))?;

    // Delegate the first leaf to `leaf_add` so the scaffolded leaf is
    // byte-identical to one the LLM would later add by hand — no template drift.
    // A fresh `.grove/` has only `BRIEF.md`, so the next root child is `1-[1]`.
    let leaf_path = leaf_add(&grove_root, ".", slug, Kind::Planning)?;
    Ok(vec![brief_path, leaf_path])
}

/// The grove's name is the worktree directory's basename (it equals the branch
/// name; the worktree lives at `<repo>/.grove-worktrees/<name>/`). Used as the
/// root brief's `# <name> — brief` title (BRIEF-FORMAT).
fn grove_name(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// Emit the minimal section-header scaffold for the root `BRIEF.md` (BRIEF-FORMAT
/// suggested shape). Headers only, no prose — the first planning session fills
/// them. Identical to the old verb's root brief (the root brief is the one
/// unkeyed singleton, unchanged by the new scheme).
fn write_root_brief(path: &Path, name: &str) -> Result<()> {
    let body = format!(
        "# {name} — brief\n\n## Goal\n\n## Done when\n\n## Decomposition\n\n## Pointers\n\n## Notes\n",
    );
    std::fs::write(path, body.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

/// `leaf-decompose <leaf-path> <first-child-slug>`: convert a live leaf
/// `<id>-[k]-<slug>.md` into a node brief `<id>-[k]-<slug>.BRIEF.md` (**preserving
/// the key `[k]`** — the entity that was `[k]` becomes node `[k]`, D6), retitle
/// its `# …` header with ` — brief`, **and** create the first child
/// `<id>.1-[new]-<first-child-slug>.md` atomically so the node is never childless
/// (D4). The `kind` is the first child's kind (the spec's two-positional surface
/// leaves the kind to the CLI default in 060; the core takes it explicitly for
/// parity with `leaf-add`/`leaf-insert`). Refuses a brief and an already-`.DONE`
/// leaf. Returns `(brief_path, first_child_path)`. Working-tree only — no commit.
pub fn leaf_decompose(
    grove_root: &Path,
    leaf_path: &Path,
    first_child_slug: &str,
    kind: Kind,
) -> Result<(PathBuf, PathBuf)> {
    // Validate the child slug up front, before any filesystem mutation, so a bad
    // slug leaves the leaf un-decomposed (no half-built childless node brief). The
    // `git mv` below is the only mutation that would be hard to unwind.
    validate_slug(first_child_slug)?;

    let (grove_abs, name) = resolve_leaf(grove_root, leaf_path)?;
    let id = parse(&name).with_context(|| format!("not a new-format leaf: {name}"))?;
    if id.is_brief {
        bail!("cannot decompose a brief (it is already a node): {name}");
    }
    if id.is_done {
        bail!("cannot decompose a retired (.DONE) leaf: {name}");
    }

    // The entity that was leaf [k] becomes node brief [k]: same position, key, and
    // slug — only the brief marker is added (D6 key-preservation).
    let brief_id = LeafId {
        is_brief: true,
        ..id.clone()
    };
    let brief_name = brief_id.filename();
    let brief_path = grove_abs.join(&brief_name);
    if brief_path.exists() {
        bail!("destination already exists: {}", brief_path.display());
    }
    git_mv(&grove_abs, &name, &brief_name)?;
    append_brief_suffix_in_file(&brief_path)?;

    // Grow the first child at <id>.1 (D4 enforce-first-child) — delegated to
    // `leaf_add` so the child is byte-identical to a hand-added one and gets the
    // next fresh key. The parent node now exists (the brief we just created), so
    // the parent-existence guard passes and the child lands at `<id>.1`.
    let parent_dotted = join_position(&id.position);
    let child_path = leaf_add(&grove_abs, &parent_dotted, first_child_slug, kind)?;
    Ok((brief_path, child_path))
}

/// `leaf-retire <leaf-path>`: rename a live leaf `<id>-[k]-<slug>.md` →
/// `<id>-[k]-<slug>.DONE.md` (D3), keeping it in the flat list with its position
/// and key intact (the `.DONE` is a pure filename marker; the `# …` header is
/// unchanged — headers never carry the marker). Refuses a `BRIEF.md` (briefs are
/// never done, D4) and an already-`.DONE` leaf. Returns the retired file's
/// absolute path. Working-tree only — no commit.
pub fn leaf_retire(grove_root: &Path, leaf_path: &Path) -> Result<PathBuf> {
    let (grove_abs, name) = resolve_leaf(grove_root, leaf_path)?;
    let id = parse(&name).with_context(|| format!("not a new-format leaf: {name}"))?;
    if id.is_brief {
        bail!("cannot retire a brief (briefs are never done): {name}");
    }
    if id.is_done {
        bail!("leaf is already retired (.DONE): {name}");
    }

    // Append the `.DONE` marker in place — position, key, and slug are untouched,
    // so the retired leaf keeps its sort position in the flat list (D3).
    let done_id = LeafId {
        is_done: true,
        ..id.clone()
    };
    let done_name = done_id.filename();
    let done_path = grove_abs.join(&done_name);
    if done_path.exists() {
        bail!("destination already exists: {}", done_path.display());
    }
    git_mv(&grove_abs, &name, &done_name)?;
    Ok(done_path)
}

// ---------------------------------------------------------------------------
// helpers

/// Resolve a leaf path to `(canonical grove root, bare filename)`. `leaf_path` is
/// accepted as absolute or relative to the grove root; the resolved file must be a
/// real file under the grove root. The bare filename feeds `git mv` (run with
/// `git -C <grove>`, so positions are relative to the flat grove dir).
fn resolve_leaf(grove_root: &Path, leaf_path: &Path) -> Result<(PathBuf, String)> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    let grove_abs = grove_root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", grove_root.display()))?;
    let candidate = if leaf_path.is_absolute() {
        leaf_path.to_path_buf()
    } else {
        grove_abs.join(leaf_path)
    };
    let abs = candidate
        .canonicalize()
        .with_context(|| format!("resolving leaf path {}", candidate.display()))?;
    if !abs.starts_with(&grove_abs) {
        bail!(
            "leaf path {} is not under grove root {}",
            abs.display(),
            grove_abs.display()
        );
    }
    let name = abs
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| format!("leaf path {} has no filename", abs.display()))?
        .to_string();
    Ok((grove_abs, name))
}

/// `git mv <src> <dst>` within `grove_root` (bare filenames — the scheme is flat).
/// Renames are tracked moves so the enclosing task's commit folds them in cleanly.
fn git_mv(grove_root: &Path, src: &str, dst: &str) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(grove_root)
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

/// Render a position vector as its dotted string (`[2,3,1]` → `2.3.1`; `[]` →
/// `""`). Decompose feeds the result to `leaf_add` as the `<parent-id>` of the
/// first child (a leaf's position is never empty, so the result is never `""`).
fn join_position(position: &[u32]) -> String {
    position
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

/// Append ` — brief` to a freshly-decomposed node brief's first-line header,
/// rewriting the file in place. A first line that is not a new-format
/// `# <position>-[<key>]-<slug>` header — or one that already ends ` — brief` —
/// is left alone (conservative + idempotent; see [`append_brief_suffix`]).
fn append_brief_suffix_in_file(path: &Path) -> Result<()> {
    let body =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let new_first = match append_brief_suffix(first) {
        Some(s) => s,
        None => return Ok(()),
    };
    let mut out = String::with_capacity(body.len() + 8);
    out.push_str(&new_first);
    if let Some(r) = rest {
        out.push('\n');
        out.push_str(r);
    }
    std::fs::write(path, out.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

/// Append ` — brief` to a `# <position>-[<key>]-<slug>` header line. Returns
/// `None` (leave the line alone) for anything that is not a new-format header — a
/// header whose leading dotted run is not followed by `-[` (a key) — and for a
/// header that already ends ` — brief` (idempotent against an LLM that pre-titled
/// the leaf). The position/key/slug are preserved; decompose never rewrites them.
fn append_brief_suffix(line: &str) -> Option<String> {
    let body = line.strip_prefix("# ")?;
    let dash = body.find('-')?;
    parse_position(&body[..dash])?; // the leading run must be a real position
    if !body[dash..].starts_with("-[") {
        return None; // not a new-format `# <pos>-[<key>]-…` header — leave alone
    }
    if line.trim_end().ends_with("— brief") {
        return None; // already a brief title — idempotent
    }
    Some(format!("# {body} — brief"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use tempfile::TempDir;

    // ---- fixtures -----------------------------------------------------------

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

    /// Write a stub file whose body is just its `# <stem>` header.
    fn touch(root: &Path, name: &str) -> PathBuf {
        let p = root.join(name);
        fs::write(&p, format!("# {}\n", name.strip_suffix(".md").unwrap())).unwrap();
        p
    }

    /// Write a file with an explicit body (for realistic headers).
    fn touch_body(root: &Path, name: &str, content: &str) -> PathBuf {
        let p = root.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    fn body(p: &Path) -> String {
        fs::read_to_string(p).unwrap()
    }

    /// The grove's regular-file names, lexically sorted.
    fn list(root: &Path) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(root)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
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
        assert_eq!(name_of(&created[1]), "1-[1]-plan.md");
        // Both exist on disk, directly in the flat `.grove/`.
        let g = wt.join(".grove");
        assert!(g.join("BRIEF.md").is_file());
        assert!(g.join("1-[1]-plan.md").is_file());
    }

    #[test]
    fn root_init_first_leaf_is_a_planning_task() {
        // A newborn grove always starts by planning (parity with the old verb).
        let (_t, wt) = worktree();
        let created = root_init(&wt, "plan").unwrap();
        assert!(
            body(&created[1]).contains("**Kind:** planning"),
            "got {:?}",
            body(&created[1])
        );
    }

    #[test]
    fn root_init_first_leaf_header_is_the_filename_stem() {
        let (_t, wt) = worktree();
        let created = root_init(&wt, "plan").unwrap();
        assert!(
            body(&created[1]).starts_with("# 1-[1]-plan\n"),
            "header should be the filename stem; got {:?}",
            body(&created[1])
        );
    }

    #[test]
    fn root_init_root_brief_title_is_the_grove_name() {
        // The root brief title is `# <worktree-basename> — brief` (BRIEF-FORMAT).
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
        // Validation runs before any filesystem mutation (no stray `.grove/`).
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
    fn decompose_converts_leaf_to_brief_preserving_the_key() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[3]-build.md");
        stage_all(&g);
        let (brief, _child) =
            leaf_decompose(&g, Path::new("2-[3]-build.md"), "step", Kind::Work).unwrap();
        // The entity that was leaf [3] becomes node brief [3] — key preserved.
        assert_eq!(name_of(&brief), "2-[3]-build.BRIEF.md");
        let files = list(&g);
        assert!(files.contains(&"2-[3]-build.BRIEF.md".to_string()));
        assert!(
            !files.contains(&"2-[3]-build.md".to_string()),
            "old leaf name gone"
        );
    }

    #[test]
    fn decompose_creates_the_first_child_at_dot_one_with_a_fresh_key() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[3]-build.md");
        stage_all(&g);
        let (_brief, child) =
            leaf_decompose(&g, Path::new("2-[3]-build.md"), "step", Kind::Work).unwrap();
        // Child position is <id>.1; key is the next fresh key (max [3] + 1 = 4).
        assert_eq!(name_of(&child), "2.1-[4]-step.md");
        assert!(g.join("2.1-[4]-step.md").is_file());
    }

    #[test]
    fn decompose_rewrites_the_header_with_an_em_dash_brief_suffix() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch_body(&g, "2-[3]-build.md", "# 2-[3]-build\n\n## Goal\n");
        stage_all(&g);
        let (brief, _child) =
            leaf_decompose(&g, Path::new("2-[3]-build.md"), "step", Kind::Work).unwrap();
        assert_eq!(
            body(&brief).lines().next().unwrap(),
            "# 2-[3]-build — brief",
            "position/key/slug intact, ` — brief` appended"
        );
    }

    #[test]
    fn decompose_first_child_header_is_its_stem_and_carries_the_kind() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[3]-build.md");
        stage_all(&g);
        let (_brief, child) =
            leaf_decompose(&g, Path::new("2-[3]-build.md"), "step", Kind::Work).unwrap();
        let text = body(&child);
        assert!(text.starts_with("# 2.1-[4]-step\n"), "got {text:?}");
        assert!(text.contains("**Kind:** work"), "got {text:?}");
    }

    #[test]
    fn decompose_first_child_can_be_a_planning_task() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[3]-build.md");
        stage_all(&g);
        let (_brief, child) =
            leaf_decompose(&g, Path::new("2-[3]-build.md"), "design", Kind::Planning).unwrap();
        assert!(body(&child).contains("**Kind:** planning"));
    }

    #[test]
    fn decompose_a_nested_leaf_preserves_key_and_grows_a_grandchild() {
        // Decomposing a leaf under a node: key preserved, child at <id>.1, fresh key.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch_body(&g, "2-[1]-build.BRIEF.md", "# 2-[1]-build — brief\n");
        touch(&g, "2.2-[5]-mid.md");
        stage_all(&g);
        let (brief, child) =
            leaf_decompose(&g, Path::new("2.2-[5]-mid.md"), "first", Kind::Work).unwrap();
        assert_eq!(name_of(&brief), "2.2-[5]-mid.BRIEF.md", "key [5] preserved");
        assert_eq!(
            name_of(&child),
            "2.2.1-[6]-first.md",
            "fresh key max(1,5)+1 = 6"
        );
    }

    #[test]
    fn decompose_refuses_a_brief() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[3]-build.BRIEF.md");
        stage_all(&g);
        let err =
            leaf_decompose(&g, Path::new("2-[3]-build.BRIEF.md"), "x", Kind::Work).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_done_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[3]-build.DONE.md");
        stage_all(&g);
        let err =
            leaf_decompose(&g, Path::new("2-[3]-build.DONE.md"), "x", Kind::Work).unwrap_err();
        assert!(
            err.to_string().to_lowercase().contains("done") || err.to_string().contains("retired"),
            "got {err}"
        );
    }

    #[test]
    fn decompose_refuses_a_foreign_file() {
        let (_t, g) = git_grove();
        touch(&g, "README.md");
        stage_all(&g);
        let err = leaf_decompose(&g, Path::new("README.md"), "x", Kind::Work).unwrap_err();
        assert!(err.to_string().contains("new-format"), "got {err}");
    }

    #[test]
    fn decompose_rejects_a_bad_child_slug_without_touching_the_leaf() {
        // Atomicity: the child slug is validated BEFORE the git mv, so a bad slug
        // leaves the leaf un-decomposed (no half-built node brief).
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[3]-build.md");
        stage_all(&g);
        assert!(leaf_decompose(&g, Path::new("2-[3]-build.md"), "Bad Slug", Kind::Work).is_err());
        let files = list(&g);
        assert!(
            files.contains(&"2-[3]-build.md".to_string()),
            "leaf untouched"
        );
        assert!(
            !files.contains(&"2-[3]-build.BRIEF.md".to_string()),
            "no half-built brief"
        );
    }

    #[test]
    fn decompose_accepts_a_grove_root_relative_and_absolute_path() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[3]-build.md");
        stage_all(&g);
        // Absolute path form.
        let abs = g.join("2-[3]-build.md");
        let (brief, _child) = leaf_decompose(&g, &abs, "step", Kind::Work).unwrap();
        assert_eq!(name_of(&brief), "2-[3]-build.BRIEF.md");
    }

    #[test]
    fn decompose_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err =
            leaf_decompose(&missing, Path::new("2-[3]-build.md"), "x", Kind::Work).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- leaf-retire --------------------------------------------------------

    #[test]
    fn retire_renames_leaf_appending_done_keeping_position_and_key() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2.1-[4]-add.md");
        stage_all(&g);
        let done = leaf_retire(&g, Path::new("2.1-[4]-add.md")).unwrap();
        assert_eq!(name_of(&done), "2.1-[4]-add.DONE.md");
        let files = list(&g);
        assert!(files.contains(&"2.1-[4]-add.DONE.md".to_string()));
        assert!(
            !files.contains(&"2.1-[4]-add.md".to_string()),
            "old name gone"
        );
    }

    #[test]
    fn retire_does_not_rewrite_the_header() {
        // The `.DONE` is a pure filename marker — the `# …` header is unchanged
        // (headers carry position-[key]-slug only, never the marker).
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch_body(&g, "2.1-[4]-add.md", "# 2.1-[4]-add\n\nbody\n");
        stage_all(&g);
        let done = leaf_retire(&g, Path::new("2.1-[4]-add.md")).unwrap();
        assert_eq!(
            body(&done),
            "# 2.1-[4]-add\n\nbody\n",
            "header & body byte-identical"
        );
    }

    #[test]
    fn retire_refuses_a_node_brief() {
        let (_t, g) = git_grove();
        touch(&g, "2-[1]-x.BRIEF.md");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("2-[1]-x.BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn retire_refuses_the_root_brief() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn retire_refuses_an_already_done_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "2.1-[4]-add.DONE.md");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("2.1-[4]-add.DONE.md")).unwrap_err();
        assert!(err.to_string().contains("already"), "got {err}");
    }

    #[test]
    fn retire_refuses_a_foreign_file() {
        let (_t, g) = git_grove();
        touch(&g, "README.md");
        stage_all(&g);
        let err = leaf_retire(&g, Path::new("README.md")).unwrap_err();
        assert!(err.to_string().contains("new-format"), "got {err}");
    }

    #[test]
    fn retire_accepts_an_absolute_path() {
        let (_t, g) = git_grove();
        touch(&g, "2.1-[4]-add.md");
        stage_all(&g);
        let abs = g.join("2.1-[4]-add.md");
        let done = leaf_retire(&g, &abs).unwrap();
        assert_eq!(name_of(&done), "2.1-[4]-add.DONE.md");
    }

    #[test]
    fn retire_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err = leaf_retire(&missing, Path::new("2.1-[4]-add.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- append_brief_suffix (the decompose header rewriter) ----------------

    #[test]
    fn append_brief_suffix_adds_em_dash_brief_to_a_new_format_header() {
        assert_eq!(
            append_brief_suffix("# 2-[3]-build"),
            Some("# 2-[3]-build — brief".to_string())
        );
        assert_eq!(
            append_brief_suffix("# 2.2.1-[4]-multi-word-slug"),
            Some("# 2.2.1-[4]-multi-word-slug — brief".to_string())
        );
    }

    #[test]
    fn append_brief_suffix_is_idempotent_when_already_a_brief_title() {
        assert_eq!(append_brief_suffix("# 2-[3]-build — brief"), None);
    }

    #[test]
    fn append_brief_suffix_leaves_non_new_format_headers_alone() {
        // No `[key]` after the position → not a new-format header (e.g. old
        // `NNN-slug`); a non-positional title; not a header at all.
        assert_eq!(append_brief_suffix("# 2-no-bracket"), None);
        assert_eq!(append_brief_suffix("# 010-old-format"), None);
        assert_eq!(append_brief_suffix("# Some Title"), None);
        assert_eq!(append_brief_suffix("Not a header"), None);
    }
}

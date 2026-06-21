// The `grove-llm leaf-decompose` and `grove-llm leaf-retire` verbs. Promote
// the deterministic `mv`-flavored mechanics in the Decompose and Retire steps
// of `content/SKILL.md` into CLI verbs. Per ADR-0006 these verbs live on the
// LLM-driven binary.
//
// `leaf-decompose` converts a leaf file `NNN-x.md` into a node directory
// `NNN-x/BRIEF.md` so children can be added afterwards via `leaf-add`. The
// `git mv` and the first-line header rewrite (`# NNN-x` → `# NNN-x — brief`)
// are the only mechanical steps; any further reshape of the brief body is
// judgement and stays with the LLM.
//
// `leaf-retire` performs the mechanical `git mv` of a single leaf into
// `.grove/done/`, preserving its relative path inside `.grove/`. The parent-
// chain cascade (ask before retiring each empty node, promote brief content
// upward) is judgement and stays prose — this verb retires exactly one leaf.
//
// Both operate on the working tree only — no `git commit`. Both accept
// absolute paths and paths relative to the cwd or the grove root.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::repo;

/// Convert `<leaf>.md` into `<leaf>/BRIEF.md`. The leaf file is `git mv`'d
/// into a new sibling directory of the same stem; its first-line header is
/// rewritten from `# NNN-x` to `# NNN-x — brief` if it matches that shape.
/// Returns the absolute path of the resulting `BRIEF.md`.
pub fn decompose(leaf_path: &Path) -> Result<PathBuf> {
    let (worktree, grove_root, leaf_abs) = resolve_leaf(leaf_path)?;
    require_md_leaf(&leaf_abs)?;

    let parent = leaf_abs
        .parent()
        .ok_or_else(|| anyhow::anyhow!("leaf has no parent directory: {}", leaf_abs.display()))?;
    let stem = leaf_abs
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("leaf has no file stem: {}", leaf_abs.display()))?;

    let new_dir = parent.join(stem);
    if new_dir.exists() {
        bail!(
            "destination directory already exists: {}",
            new_dir.display()
        );
    }
    let dst = new_dir.join("BRIEF.md");

    let src_rel = leaf_abs.strip_prefix(&worktree).with_context(|| {
        format!(
            "leaf {} is not under worktree {}",
            leaf_abs.display(),
            worktree.display()
        )
    })?;
    let dst_rel = dst
        .strip_prefix(&worktree)
        .expect("dst is constructed under the worktree");

    std::fs::create_dir_all(&new_dir).with_context(|| format!("creating {}", new_dir.display()))?;
    git_mv(&worktree, src_rel, dst_rel)?;
    rewrite_decomposed_header(&dst)?;

    // Silence unused warning when grove_root isn't read after resolution —
    // the value exists for the under-grove invariant check.
    let _ = grove_root;
    Ok(dst)
}

/// Move `<leaf>` to `.grove/done/<path-relative-to-.grove>`, creating any
/// missing parent directories under `done/`. Returns the absolute path of the
/// retired file.
pub fn retire(leaf_path: &Path) -> Result<PathBuf> {
    let (worktree, grove_root, leaf_abs) = resolve_leaf(leaf_path)?;
    require_md_leaf(&leaf_abs)?;

    let rel_under_grove = leaf_abs.strip_prefix(&grove_root).with_context(|| {
        format!(
            "leaf {} is not under grove root {}",
            leaf_abs.display(),
            grove_root.display()
        )
    })?;
    if rel_under_grove
        .components()
        .next()
        .and_then(|c| c.as_os_str().to_str())
        == Some("done")
    {
        bail!("leaf is already under done/: {}", leaf_abs.display());
    }

    let done_dir = grove_root.join("done");
    let dst = done_dir.join(rel_under_grove);
    if dst.exists() {
        bail!("destination already exists: {}", dst.display());
    }
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }

    let src_rel = leaf_abs
        .strip_prefix(&worktree)
        .expect("leaf_abs is under the worktree");
    let dst_rel = dst
        .strip_prefix(&worktree)
        .expect("dst is constructed under the worktree");

    git_mv(&worktree, src_rel, dst_rel)?;
    Ok(dst)
}

// ---------------------------------------------------------------------------
// helpers

/// Returns `(worktree, grove_root, canonical_leaf_path)`. `leaf` is accepted
/// as: absolute; relative to the cwd; or relative to the grove root.
fn resolve_leaf(leaf: &Path) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let worktree = repo::git_toplevel(&cwd)?;
    let grove_root = worktree.join(".grove");
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    let worktree = worktree
        .canonicalize()
        .with_context(|| format!("canonicalising worktree {}", worktree.display()))?;
    let grove_root = grove_root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", grove_root.display()))?;

    let candidates: Vec<PathBuf> = if leaf.is_absolute() {
        vec![leaf.to_path_buf()]
    } else {
        vec![cwd.join(leaf), grove_root.join(leaf)]
    };

    let mut last_err: Option<anyhow::Error> = None;
    for c in &candidates {
        match c.canonicalize() {
            Ok(abs) => {
                if !abs.starts_with(&grove_root) {
                    bail!(
                        "leaf path {} is not under grove root {}",
                        abs.display(),
                        grove_root.display()
                    );
                }
                return Ok((worktree, grove_root, abs));
            }
            Err(e) => {
                last_err =
                    Some(anyhow::Error::new(e).context(format!("resolving {}", c.display())));
            }
        }
    }
    Err(last_err.expect("at least one candidate is tried"))
}

fn require_md_leaf(p: &Path) -> Result<()> {
    if !p.is_file() {
        bail!("leaf is not a file: {}", p.display());
    }
    if p.extension().and_then(|s| s.to_str()) != Some("md") {
        bail!("leaf must be a `.md` file: {}", p.display());
    }
    if p.file_name().and_then(|s| s.to_str()) == Some("BRIEF.md") {
        bail!("BRIEF.md is a node brief, not a leaf: {}", p.display());
    }
    Ok(())
}

fn git_mv(worktree: &Path, src_rel: &Path, dst_rel: &Path) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("mv")
        .arg(src_rel)
        .arg(dst_rel)
        .output()
        .with_context(|| format!("running git mv {} {}", src_rel.display(), dst_rel.display()))?;
    if !out.status.success() {
        bail!(
            "git mv {} -> {} failed: {}",
            src_rel.display(),
            dst_rel.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// Rewrite the first line of a freshly-decomposed BRIEF.md from `# NNN-x` to
/// `# NNN-x — brief`. A first line that doesn't match the `# NNN-...` shape is
/// left alone (the verb is conservative — hand-edited titles stay
/// hand-edited). A first line that already ends in `— brief` is left alone so
/// the verb is idempotent against an LLM that pre-titled the leaf.
fn rewrite_decomposed_header(brief_path: &Path) -> Result<()> {
    let body = std::fs::read_to_string(brief_path)
        .with_context(|| format!("reading {}", brief_path.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let new_first = match brief_header(first) {
        Some(s) => s,
        None => return Ok(()),
    };
    let mut out = String::with_capacity(body.len() + 8);
    out.push_str(&new_first);
    if let Some(r) = rest {
        out.push('\n');
        out.push_str(r);
    }
    std::fs::write(brief_path, out.as_bytes())
        .with_context(|| format!("writing {}", brief_path.display()))?;
    Ok(())
}

fn brief_header(line: &str) -> Option<String> {
    let body = line.strip_prefix("# ")?;
    if body.len() < 4 {
        return None;
    }
    let (head, tail) = body.split_at(3);
    if !head.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !tail.starts_with('-') {
        return None;
    }
    if line.trim_end().ends_with("— brief") {
        return None;
    }
    Some(format!("# {}{} — brief", head, tail))
}

#[cfg(test)]
mod inline_tests {
    use super::*;

    #[test]
    fn brief_header_appends_em_dash_brief_to_nnn_dash_form() {
        assert_eq!(
            brief_header("# 050-foo"),
            Some("# 050-foo — brief".to_string())
        );
        assert_eq!(
            brief_header("# 990-some-thing"),
            Some("# 990-some-thing — brief".to_string())
        );
    }

    #[test]
    fn brief_header_is_idempotent_when_already_brief() {
        assert_eq!(brief_header("# 050-foo — brief"), None);
    }

    #[test]
    fn brief_header_leaves_non_matching_titles_alone() {
        assert_eq!(brief_header("# Some title"), None);
        assert_eq!(brief_header("Not a header"), None);
        assert_eq!(brief_header("# 05-foo"), None);
    }
}

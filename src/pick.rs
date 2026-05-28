// The `grove-llm pick` verb. Promotes the deterministic Pick step from
// `content/SKILL.md`'s loop into a CLI verb. Per ADR-0006 the verb lives on
// the LLM-driven binary; the walk's semantics (depth-first, numeric prefix,
// skip `done/`) are the spec the prose used to describe and that this module
// implements.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Walk `grove_root` depth-first in numeric-prefix order, skipping `done/`,
/// and return the first live `.md` leaf. Returns `None` if the tree contains
/// no live leaves (every node is empty modulo `done/`, or `.grove/` itself is
/// empty). The grove root's own `BRIEF.md` is not a leaf — nor is any node's
/// `BRIEF.md`.
pub fn next_leaf(grove_root: &Path) -> Result<Option<PathBuf>> {
    if !grove_root.is_dir() {
        anyhow::bail!("grove root not found: {}", grove_root.display());
    }
    walk(grove_root)
}

fn walk(dir: &Path) -> Result<Option<PathBuf>> {
    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by(|a, b| {
        let na = a.file_name().to_string_lossy().into_owned();
        let nb = b.file_name().to_string_lossy().into_owned();
        sort_key(&na).cmp(&sort_key(&nb))
    });

    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let ft = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if ft.is_dir() {
            if name == "done" {
                continue;
            }
            if let Some(p) = walk(&entry.path())? {
                return Ok(Some(p));
            }
        } else if ft.is_file() {
            if name == "BRIEF.md" {
                continue;
            }
            if name.ends_with(".md") {
                return Ok(Some(entry.path()));
            }
        }
    }
    Ok(None)
}

/// Sort key for sibling ordering: `(no_numeric_prefix, name)`. Entries with a
/// numeric prefix come first (lex-sorted among themselves, which is identical
/// to numeric sort when prefixes are zero-padded — the established
/// convention); entries without a numeric prefix come last.
fn sort_key(name: &str) -> (bool, &str) {
    let has_prefix = name.chars().next().map_or(false, |c| c.is_ascii_digit());
    (!has_prefix, name)
}

// The `grove-llm brief-chain` verb. Promotes the Bootstrap step's brief-chain
// enumeration from `content/SKILL.md`'s prose into a CLI verb. Per ADR-0006
// the verb lives on the LLM-driven binary; the walk's semantics
// (root→leaf order, missing `BRIEF.md` skipped silently) are the spec the
// prose used to describe and that this module implements.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Walk ancestors of `leaf_path` (exclusive), collecting `BRIEF.md` at each
/// directory level up to and including `grove_root`. Return the collected
/// paths in root→leaf order. The leaf file itself is not included — that is
/// the LLM's separate read, not part of the brief chain.
///
/// `leaf_path` is either absolute or relative to `grove_root`. A missing
/// `BRIEF.md` at any level is skipped silently rather than erroring: some
/// nodes (e.g. mid-decomposition transient state) do not yet carry a brief.
pub fn chain_for(grove_root: &Path, leaf_path: &Path) -> Result<Vec<PathBuf>> {
    if !grove_root.is_dir() {
        anyhow::bail!("grove root not found: {}", grove_root.display());
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

    if !leaf_abs.starts_with(&grove_root) {
        anyhow::bail!(
            "leaf path {} is not under grove root {}",
            leaf_abs.display(),
            grove_root.display()
        );
    }

    let mut briefs: Vec<PathBuf> = Vec::new();
    let mut cursor = leaf_abs.parent();
    while let Some(dir) = cursor {
        let brief = dir.join("BRIEF.md");
        if brief.is_file() {
            briefs.push(brief);
        }
        if dir == grove_root.as_path() {
            break;
        }
        cursor = dir.parent();
    }
    briefs.reverse();
    Ok(briefs)
}

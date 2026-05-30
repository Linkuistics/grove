// The `grove-llm root-init` verb. Scaffolds a brand-new grove's tree so the
// LLM never improvises the first artifacts: it creates `.grove/`, writes the
// root `BRIEF.md` stub, and lays down a first planning leaf `010-<slug>.md`.
// Per ADR-0006 this verb lives on the LLM-driven binary; per ADR-0011 it is a
// new verb (not an extension of `grove do`) and it creates the first leaf (not
// just the root brief) so `grove-llm pick` immediately returns work instead of
// reporting the grove as done.
//
// This verb sits *below* the floor every other tree verb stands on: `leaf-add`
// / `leaf-insert` / `leaf-decompose` / `leaf-retire` all require `.grove/` to
// already exist (they `bail!("grove root not found")` otherwise). `root-init`
// is the bootstrap that creates that directory. It refuses to clobber an
// existing `.grove/`, and — like its siblings — makes no commit; the first
// session's commit folds the scaffold in.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

use crate::leaf::{self, Kind};

/// Scaffold a fresh grove under `worktree/.grove`: the root `BRIEF.md` stub and
/// a first planning leaf `010-<slug>.md`. Returns the absolute paths created,
/// `BRIEF.md` first then the leaf. Refuses if `.grove/` already exists.
/// Working-tree change only — no commit.
pub fn init(worktree: &Path, slug: &str) -> Result<Vec<PathBuf>> {
    // Validate before touching the filesystem so a bad slug never leaves a
    // stray `.grove/` behind.
    leaf::validate_slug(slug)?;

    let grove_root = worktree.join(".grove");
    if grove_root.exists() {
        bail!("grove root already exists: {}", grove_root.display());
    }

    std::fs::create_dir_all(&grove_root)
        .with_context(|| format!("creating {}", grove_root.display()))?;

    let brief_path = grove_root.join("BRIEF.md");
    write_root_brief(&brief_path, &grove_name(worktree))?;

    // Delegate the first leaf to `leaf::add` so the scaffolded leaf is
    // byte-identical to one the LLM would later add by hand — no template drift
    // between the two paths. A fresh `.grove/` has no siblings, so the next free
    // prefix is 010.
    let leaf_path = leaf::add(&grove_root, slug, None, Kind::Planning)?;

    Ok(vec![brief_path, leaf_path])
}

/// The grove's name is the worktree directory's basename (it equals the branch
/// name; the worktree lives at `<repo>/.grove-worktrees/<name>/`). Used as the
/// root brief's `# <name> — brief` title per `BRIEF-FORMAT.md`.
fn grove_name(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// Emit the minimal section-header scaffold from `BRIEF-FORMAT.md`'s suggested
/// shape. Headers only, no prose — the executing planning session fills them.
fn write_root_brief(path: &Path, name: &str) -> Result<()> {
    let body = format!(
        "# {name} — brief\n\n## Goal\n\n## Done when\n\n## Decomposition\n\n## Pointers\n\n## Notes\n",
        name = name,
    );
    std::fs::write(path, body.as_bytes())
        .with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

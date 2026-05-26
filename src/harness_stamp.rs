use crate::harness::{self, Harness};
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Path of the harness stamp for a grove: `<repo>/.grove-stamps/<name>`.
pub fn path(repo: &Path, name: &str) -> std::path::PathBuf {
    repo.join(".grove-stamps").join(name)
}

/// Resolve which harness to use for a launcher verb:
/// 1. If `explicit` is Some, look it up.
/// 2. Else read .grove-stamps/<name> if present.
/// 3. Else, repo scan: if exactly one harness, use it.
/// 4. Else (both or none): error with guidance.
pub fn resolve_for_launch(
    repo: &Path,
    name: &str,
    explicit: Option<&str>,
) -> Result<&'static Harness> {
    if let Some(name) = explicit {
        return harness::by_name(name)
            .ok_or_else(|| anyhow::anyhow!("unknown harness: {}", name));
    }
    let stamp = path(repo, name);
    if stamp.exists() {
        let s = fs::read_to_string(&stamp)?.trim().to_string();
        return harness::by_name(&s).ok_or_else(|| {
            anyhow::anyhow!(
                "{} contents do not match any known harness: {}",
                stamp.display(),
                s
            )
        });
    }
    let detected = harness::detect_in_repo(repo);
    match detected.len() {
        1 => Ok(detected[0]),
        0 => anyhow::bail!("no harness detected in {}", repo.display()),
        _ => anyhow::bail!(
            "multiple harnesses in {} — pass --harness to bind this grove",
            repo.display()
        ),
    }
}

/// Write `<repo>/.grove-stamps/<name>` only when needed for disambiguation
/// (multi-harness repo). In a single-harness repo, no-op.
pub fn maybe_stamp(
    repo: &Path,
    name: &str,
    chosen: &'static Harness,
) -> Result<()> {
    let detected = harness::detect_in_repo(repo);
    if detected.len() < 2 {
        return Ok(()); // single-harness repo; no disambiguation needed
    }
    let stamp = path(repo, name);
    if let Some(parent) = stamp.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&stamp, format!("{}\n", chosen.name))?;
    Ok(())
}

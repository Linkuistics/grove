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
        return harness::by_name(name).ok_or_else(|| {
            anyhow::anyhow!(
                "unknown harness: {}. Known: {}",
                name,
                harness::known_names()
            )
        });
    }
    let stamp = path(repo, name);
    if stamp.exists() {
        let s = fs::read_to_string(&stamp)?.trim().to_string();
        return harness::by_name(&s).ok_or_else(|| {
            anyhow::anyhow!(
                "{} contents do not match any known harness: {}. Known: {}",
                stamp.display(),
                s,
                harness::known_names()
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

/// Write `<repo>/.grove-stamps/<name>`. Two triggers:
///   * `explicit` — the user passed `--harness`; a deliberate binding must
///     survive into the next plain `grove do`, even in a repo where detection
///     would pick something else (e.g. a stray `.claude/` after a switch).
///   * multi-harness repo — disambiguation, as before.
///
/// A single-harness repo with no explicit flag stays stamp-free: detection is
/// already deterministic there.
///
/// **One call site, [`crate::launch::do_grove`], and deliberately so.** The
/// stamp binds the *grove*, so the only verb that writes it is the one that
/// drives the grove (do-is-sole-lifecycle-verb). `grove retire --harness`
/// resolves a harness for its one session and leaves the binding untouched, for
/// the same reason the call sits below `do`'s own `--no-launch` return (B3):
/// **a lasting binding is written by the action that asks for one**, and
/// neither a dry run nor a by-hand node retire is asking. Adding a second call
/// site means deciding that rule again — not merely reaching for symmetry
/// (model-per-task-kind).
pub fn maybe_stamp(
    repo: &Path,
    name: &str,
    chosen: &'static Harness,
    explicit: bool,
) -> Result<()> {
    let detected = harness::detect_in_repo(repo);
    if !explicit && detected.len() < 2 {
        return Ok(());
    }
    let stamp = path(repo, name);
    if let Some(parent) = stamp.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&stamp, format!("{}\n", chosen.name))?;
    Ok(())
}

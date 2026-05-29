use crate::cli::RepoArgs;
use crate::harness::{Harness, HARNESSES};
use crate::repo;
use crate::version_md;
use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// The version of the `grove` binary itself — the methodology bundled inside
/// the binary (the `cli` layer of the three-version model in `CONTEXT.md`).
const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run(args: &RepoArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    print!("{}", render(&repo_path)?);
    Ok(())
}

/// Read the worktree's materialised `VERSION.md` for `harness`, if present and
/// parseable. `None` means the worktree has no readable `VERSION.md` for that
/// harness — callers render that as `(unknown)` and never treat it as drift.
///
/// This is the per-worktree-per-harness lookup that the TUI (leaf 050) reuses.
pub fn worktree_version(repo: &Path, grove_name: &str, harness: &Harness) -> Option<String> {
    let worktree = repo::grove_worktree(repo, grove_name);
    let dir = harness.install_path(&worktree);
    version_md::read_version(&dir).ok()
}

/// One harness's version situation for a single grove worktree.
struct HarnessVersion {
    harness: &'static str,
    /// The worktree's materialised version; `None` = missing/unreadable.
    worktree: Option<String>,
    /// The repo's installed version for this harness; `None` = orphan (the
    /// harness is materialised in the worktree but no longer installed in the
    /// repo).
    repo: Option<String>,
}

struct GroveStatus {
    name: String,
    live: usize,
    done: usize,
    harnesses: Vec<HarnessVersion>,
}

/// Build the full `grove status` output as a string.
fn render(repo_path: &Path) -> Result<String> {
    // The `repo` layer: each harness's installed version. Absent harnesses are
    // not in the map; a present-but-unreadable `VERSION.md` maps to `None`.
    let mut repo_versions: BTreeMap<&'static str, Option<String>> = BTreeMap::new();
    for h in HARNESSES {
        let dest = h.install_path(repo_path);
        if dest.exists() {
            repo_versions.insert(h.name, version_md::read_version(&dest).ok());
        }
    }

    let mut out = String::new();
    if repo_versions.is_empty() {
        out.push_str(&format!("grove: not installed in {}\n", repo_path.display()));
        return Ok(out);
    }

    // Header section: the `cli` layer plus each harness's `repo` layer.
    out.push_str(&format!(
        "grove cli {}, installs in {}:\n",
        CLI_VERSION,
        repo_path.display()
    ));
    for (name, ver) in &repo_versions {
        match ver {
            Some(v) => out.push_str(&format!("  {} → {}\n", name, v)),
            None => out.push_str(&format!("  {} → (unknown version)\n", name)),
        }
    }
    let mut distinct: Vec<&str> = repo_versions
        .values()
        .filter_map(|o| o.as_deref())
        .collect();
    distinct.sort();
    distinct.dedup();
    if distinct.len() > 1 {
        out.push_str("  ⚠ harnesses are on different versions — `grove update` to align\n");
    }

    render_groves(repo_path, &repo_versions, &mut out)?;
    Ok(out)
}

fn render_groves(
    repo_path: &Path,
    repo_versions: &BTreeMap<&'static str, Option<String>>,
    out: &mut String,
) -> Result<()> {
    let dir = repo::grove_worktrees_dir(repo_path);
    if !dir.is_dir() {
        out.push_str("\nno groves yet.\n");
        return Ok(());
    }

    let mut names: Vec<String> = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        names.push(entry.file_name().to_string_lossy().into_owned());
    }
    names.sort();

    let groves: Vec<GroveStatus> = names
        .iter()
        .map(|name| gather_grove(repo_path, name, repo_versions))
        .collect::<Result<_>>()?;

    // Disambiguate rows with a `[harness]` prefix only when more than one
    // harness is in play across the repo install and any worktree.
    let mut harness_set: BTreeSet<&str> = repo_versions.keys().copied().collect();
    for g in &groves {
        for hv in &g.harnesses {
            harness_set.insert(hv.harness);
        }
    }
    let multi = harness_set.len() > 1;

    out.push_str("\ngroves:\n");
    for g in &groves {
        for hv in &g.harnesses {
            let prefix = if multi {
                format!("[{}] ", hv.harness)
            } else {
                String::new()
            };
            out.push_str(&format!(
                "  {}{} → leaves:{}/{} {}\n",
                prefix,
                g.name,
                g.live,
                g.done,
                version_segment(&hv.worktree, &hv.repo, CLI_VERSION),
            ));
        }
    }
    Ok(())
}

fn gather_grove(
    repo_path: &Path,
    name: &str,
    repo_versions: &BTreeMap<&'static str, Option<String>>,
) -> Result<GroveStatus> {
    let worktree = repo::grove_worktree(repo_path, name);
    let task_tree = worktree.join(".grove");
    let (live, done) = if task_tree.is_dir() {
        count_md_leaves(&task_tree, false)?
    } else {
        (0, 0)
    };

    // Relevant harnesses: the union of those installed in the repo and those
    // materialised in the worktree. The union keeps orphan worktrees (harness
    // gone from the repo) visible, and keeps legacy worktrees (no install at
    // all) showing one row per repo harness rendered as `(unknown)`.
    let mut relevant: BTreeSet<&'static str> = repo_versions.keys().copied().collect();
    for h in HARNESSES {
        if h.install_path(&worktree).exists() {
            relevant.insert(h.name);
        }
    }

    let mut harnesses = Vec::new();
    for h in HARNESSES {
        if !relevant.contains(h.name) {
            continue;
        }
        harnesses.push(HarnessVersion {
            harness: h.name,
            worktree: worktree_version(repo_path, name, h),
            repo: repo_versions.get(h.name).cloned().flatten(),
        });
    }

    Ok(GroveStatus {
        name: name.to_string(),
        live,
        done,
        harnesses,
    })
}

/// The trailing `worktree=… [drift markers]` segment for one harness row.
///
/// - Missing `VERSION.md` renders `worktree=(unknown)` and is never drift.
/// - An orphan (`repo` is `None`) shows `repo=(none)` informationally, no `⚠`.
/// - Worktree vs repo and worktree vs cli are each flagged with `⚠` on a raw
///   string mismatch — no semver interpretation (the reader works out the
///   major/minor/patch significance).
///
/// Comparison and display are plain string equality with **no** `v`-prefix
/// normalisation: from v4 onward `grove install` stores the canonical stamp
/// (no leading `v` — see leaf 040), and this binary only ships under a new
/// version number, so any stamp still bearing a `v` is necessarily a pre-v4
/// release — genuinely older than the running code. Flagging it as drift is
/// the correct verdict, not a false positive; do not re-add `v`-stripping.
fn version_segment(worktree: &Option<String>, repo: &Option<String>, cli: &str) -> String {
    let Some(wt) = worktree else {
        return "worktree=(unknown)".to_string();
    };
    let mut s = format!("worktree={}", wt);
    match repo {
        None => s.push_str(" repo=(none)"),
        Some(r) if r != wt => s.push_str(&format!(" ⚠ repo={}", r)),
        Some(_) => {}
    }
    if cli != wt {
        s.push_str(&format!(" ⚠ cli={}", cli));
    }
    s
}

/// Recursively count `.md` files in `dir`. Returns (live_count, done_count).
/// A file is "done" if any ancestor directory (within `dir`) is named `done`.
fn count_md_leaves(dir: &Path, inside_done: bool) -> Result<(usize, usize)> {
    let (mut live, mut done) = (0, 0);
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        if path.is_dir() {
            let in_done = inside_done || name == "done";
            let (l, d) = count_md_leaves(&path, in_done)?;
            live += l;
            done += d;
        } else if path.is_file() && path.extension().map(|e| e == "md").unwrap_or(false) {
            if inside_done {
                done += 1;
            } else {
                live += 1;
            }
        }
    }
    Ok((live, done))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness;
    use tempfile::TempDir;

    fn s(v: &str) -> Option<String> {
        Some(v.to_string())
    }

    // --- version_segment: the five required drift cases -------------------
    //
    // Stamps are canonical (no leading `v`) from v4 onward; these inputs model
    // that world. Comparison and display are plain string equality.

    #[test]
    fn segment_aligned_shows_only_worktree() {
        assert_eq!(
            version_segment(&s("4.0.0"), &s("4.0.0"), "4.0.0"),
            "worktree=4.0.0"
        );
    }

    #[test]
    fn segment_flags_drift_between_worktree_and_repo() {
        assert_eq!(
            version_segment(&s("3.0.1"), &s("4.0.0"), "3.0.1"),
            "worktree=3.0.1 ⚠ repo=4.0.0"
        );
    }

    #[test]
    fn segment_flags_drift_between_worktree_and_cli() {
        assert_eq!(
            version_segment(&s("3.0.1"), &s("3.0.1"), "4.0.0"),
            "worktree=3.0.1 ⚠ cli=4.0.0"
        );
    }

    #[test]
    fn segment_unknown_worktree_is_not_drift() {
        assert_eq!(
            version_segment(&None, &s("4.0.0"), "4.0.0"),
            "worktree=(unknown)"
        );
    }

    #[test]
    fn segment_orphan_worktree_shows_repo_none_without_warning() {
        assert_eq!(
            version_segment(&s("4.0.0"), &None, "4.0.0"),
            "worktree=4.0.0 repo=(none)"
        );
    }

    #[test]
    fn segment_orphan_still_flags_cli_drift() {
        assert_eq!(
            version_segment(&s("3.0.1"), &None, "4.0.0"),
            "worktree=3.0.1 repo=(none) ⚠ cli=4.0.0"
        );
    }

    #[test]
    fn segment_v_prefixed_stamp_reads_as_pre_v4_drift() {
        // A stamp still bearing the git-tag `v` is a pre-v4 release: older than
        // the canonical v4 code, so plain `!=` correctly flags drift and the
        // verbatim `v` value is shown. No normalisation masks it.
        assert_eq!(
            version_segment(&s("v3.0.1"), &s("4.0.0"), "4.0.0"),
            "worktree=v3.0.1 ⚠ repo=4.0.0 ⚠ cli=4.0.0"
        );
    }

    // --- render: end-to-end over a fixture repo ---------------------------

    fn write_version_md(dir: &Path, version: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join("VERSION.md"),
            format!("| version | `{}` |\n", version),
        )
        .unwrap();
    }

    fn install_repo(repo: &Path, harness_name: &str, version: &str) {
        let h = harness::by_name(harness_name).unwrap();
        write_version_md(&h.install_path(repo), version);
    }

    fn install_worktree(repo: &Path, name: &str, harness_name: &str, version: &str) {
        let h = harness::by_name(harness_name).unwrap();
        let worktree = repo::grove_worktree(repo, name);
        write_version_md(&h.install_path(&worktree), version);
        // Give the worktree a task tree so leaf counts are non-trivial.
        let grove = worktree.join(".grove");
        fs::create_dir_all(grove.join("done")).unwrap();
        fs::write(grove.join("010-task.md"), "# 010-task\n").unwrap();
        fs::write(grove.join("done/000-old.md"), "# old\n").unwrap();
    }

    #[test]
    fn render_header_carries_cli_version_and_grove_row_carries_worktree() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        install_repo(repo, "claude", CLI_VERSION);
        install_worktree(repo, "alpha", "claude", CLI_VERSION);

        let out = render(repo).unwrap();

        assert!(
            out.contains(&format!("grove cli {}, installs in", CLI_VERSION)),
            "header missing cli version:\n{out}"
        );
        assert!(
            out.contains("alpha → leaves:1/1 worktree="),
            "grove row missing leaves/worktree:\n{out}"
        );
        // Single harness => no `[harness]` prefix.
        assert!(!out.contains("[claude]"), "unexpected harness prefix:\n{out}");
    }

    #[test]
    fn render_flags_repo_drift_on_the_grove_row() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        // Worktree aligned with the running cli, repo install ahead of both.
        install_repo(repo, "claude", "9.9.9");
        install_worktree(repo, "alpha", "claude", CLI_VERSION);

        let out = render(repo).unwrap();
        assert!(
            out.contains(&format!("worktree={} ⚠ repo=9.9.9", CLI_VERSION)),
            "repo drift marker missing:\n{out}"
        );
    }

    #[test]
    fn render_legacy_worktree_without_version_md_is_unknown() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        install_repo(repo, "claude", CLI_VERSION);
        // Worktree with a task tree but no materialised VERSION.md.
        let grove = repo::grove_worktree(repo, "legacy").join(".grove");
        fs::create_dir_all(&grove).unwrap();
        fs::write(grove.join("010-task.md"), "# 010-task\n").unwrap();

        let out = render(repo).unwrap();
        assert!(
            out.contains("legacy → leaves:1/0 worktree=(unknown)"),
            "legacy worktree not rendered as unknown:\n{out}"
        );
    }

    #[test]
    fn render_multi_harness_prefixes_rows() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        install_repo(repo, "claude", CLI_VERSION);
        install_repo(repo, "codex", CLI_VERSION);
        install_worktree(repo, "alpha", "claude", CLI_VERSION);
        install_worktree(repo, "alpha", "codex", CLI_VERSION);

        let out = render(repo).unwrap();
        assert!(out.contains("[claude] alpha → leaves:"), "claude row missing:\n{out}");
        assert!(out.contains("[codex] alpha → leaves:"), "codex row missing:\n{out}");
    }

    #[test]
    fn render_orphan_worktree_shows_repo_none() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        // Repo only has claude; worktree was materialised for codex too, which
        // is no longer installed in the repo => orphan.
        install_repo(repo, "claude", CLI_VERSION);
        install_worktree(repo, "alpha", "claude", CLI_VERSION);
        install_worktree(repo, "alpha", "codex", CLI_VERSION);

        let out = render(repo).unwrap();
        assert!(
            out.contains("[codex] alpha → leaves:1/1 worktree=") && out.contains("repo=(none)"),
            "orphan codex row missing repo=(none):\n{out}"
        );
    }

    #[test]
    fn render_not_installed_reports_clearly() {
        let tmp = TempDir::new().unwrap();
        let out = render(tmp.path()).unwrap();
        assert!(out.contains("grove: not installed in"), "got:\n{out}");
    }
}

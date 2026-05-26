use crate::cli::RepoArgs;
use crate::harness::HARNESSES;
use crate::harness_stamp;
use crate::repo;
use crate::version_md;
use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn run(args: &RepoArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;

    let mut by_harness: BTreeMap<&'static str, Option<String>> = BTreeMap::new();
    for h in HARNESSES {
        let dest = h.install_path(&repo_path);
        if dest.exists() {
            by_harness.insert(h.name, version_md::read_version(&dest).ok());
        }
    }

    if by_harness.is_empty() {
        println!("grove: not installed in {}", repo_path.display());
        return Ok(());
    }

    println!("grove installs in {}:", repo_path.display());
    let mut versions: Vec<&String> = by_harness.values().filter_map(|o| o.as_ref()).collect();
    versions.sort();
    versions.dedup();
    let drift = versions.len() > 1;
    for (name, ver) in &by_harness {
        match ver {
            Some(v) => println!("  {} → {}", name, v),
            None => println!("  {} → (unknown version)", name),
        }
    }
    if drift {
        println!("  ⚠ harnesses are on different versions — `grove update` to align");
    }

    print_grove_summary(&repo_path)?;
    Ok(())
}

fn print_grove_summary(repo_path: &Path) -> Result<()> {
    let dir = repo::grove_worktrees_dir(repo_path);
    if !dir.is_dir() {
        println!("\nno groves yet.");
        return Ok(());
    }
    println!("\ngroves:");
    let mut names: Vec<String> = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        names.push(entry.file_name().to_string_lossy().into_owned());
    }
    names.sort();
    for name in names {
        let worktree = dir.join(&name);
        let task_tree = worktree.join(".grove");
        let (live, done) = if task_tree.is_dir() {
            count_md_leaves(&task_tree, false)?
        } else {
            (0, 0)
        };
        let stamp = fs::read_to_string(harness_stamp::path(repo_path, &name))
            .ok()
            .map(|s| s.trim().to_string());
        let stamp_str = stamp.map(|h| format!(", harness={}", h)).unwrap_or_default();
        println!(
            "  {} → {} live, {} done{}",
            name, live, done, stamp_str
        );
    }
    Ok(())
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

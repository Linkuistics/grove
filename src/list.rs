use crate::cli::RepoArgs;
use crate::repo;
use anyhow::Result;
use std::fs;

pub fn run(args: &RepoArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let dir = repo::grove_worktrees_dir(&repo_path);
    if !dir.is_dir() {
        return Ok(()); // no groves
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
    for n in names {
        println!("{}", n);
    }
    Ok(())
}

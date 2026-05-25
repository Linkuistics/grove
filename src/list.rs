use crate::cli::RepoArgs;
use crate::repo;
use anyhow::Result;
use std::fs;

pub fn run(args: &RepoArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let groves = repo_path.join("groves");
    if !groves.is_dir() {
        return Ok(()); // nothing to print
    }
    let mut names: Vec<String> = Vec::new();
    for entry in fs::read_dir(&groves)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let s = name.to_string_lossy().to_string();
        if s != "done" {
            names.push(s);
        }
    }
    names.sort();
    for n in names {
        println!("{}", n);
    }
    Ok(())
}

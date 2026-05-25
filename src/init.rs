use crate::cli::{InitArgs, InstallArgs};
use crate::install::{run as run_install, Mode};
use crate::repo;
use anyhow::Result;
use std::fs;

pub fn run(args: &InitArgs) -> Result<()> {
    let install_args = InstallArgs {
        repo: args.repo.clone(),
        harnesses: args.harnesses.clone(),
        version: None,
    };
    run_install(&install_args, Mode::Install)?;

    let repo_path = repo::resolve(args.repo.as_deref())?;

    let context_md = repo_path.join("CONTEXT.md");
    if !context_md.exists() {
        fs::write(
            &context_md,
            "# CONTEXT\n\n*Domain glossary. Add terms inline as they resolve in sessions.*\n",
        )?;
        eprintln!("grove: created {}", context_md.display());
    }

    let adr_dir = repo_path.join("docs").join("adr");
    if !adr_dir.exists() {
        fs::create_dir_all(&adr_dir)?;
        fs::write(adr_dir.join(".keep"), "")?;
        eprintln!("grove: created {}", adr_dir.display());
    }

    eprintln!("grove: next → `grove start <name>`");
    Ok(())
}

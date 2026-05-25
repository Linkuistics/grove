use crate::cli::InstallArgs;
use crate::extract::extract_content;
use crate::fetch::{Fetcher, GithubFetcher};
use crate::harness::{self, Harness, SelectMode};
use crate::repo;
use crate::version_md;
use anyhow::{Context, Result};
use std::path::Path;

/// Mode: `Install` errors if grove already present; `Update` errors if absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Install,
    Update,
}

pub fn run(args: &InstallArgs, mode: Mode) -> Result<()> {
    let fetcher = GithubFetcher::new();
    run_with_fetcher(args, mode, &fetcher)
}

pub fn run_with_fetcher(
    args: &InstallArgs,
    mode: Mode,
    fetcher: &dyn Fetcher,
) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let harnesses = harness::select(&repo_path, &args.harnesses, SelectMode::Multi)?;
    let version = match &args.version {
        Some(v) => v.clone(),
        None => fetcher.latest_version()?,
    };
    eprintln!("grove: target {} @ {}", repo_path.display(), version);

    // Pre-flight: enforce create-only / update-only per mode.
    for h in &harnesses {
        let dest = h.install_path(&repo_path);
        match (mode, dest.exists()) {
            (Mode::Install, true) => anyhow::bail!(
                "grove already installed at {} (use `grove update`)",
                dest.display()
            ),
            (Mode::Update, false) => anyhow::bail!(
                "grove not installed at {} (use `grove install`)",
                dest.display()
            ),
            _ => {}
        }
    }

    let tarball = fetcher.fetch_tarball(&version)?;

    for h in &harnesses {
        materialise_one(&repo_path, h, &tarball, &version)?;
        eprintln!(
            "grove: {} → {} @ {}",
            match mode {
                Mode::Install => "installed",
                Mode::Update => "updated",
            },
            h.install_path(&repo_path).display(),
            version
        );
    }

    if mode == Mode::Update {
        eprintln!(
            "grove: record this bump as an ADR in docs/adr/ (grove's discipline for version changes)."
        );
    }

    Ok(())
}

fn materialise_one(
    repo: &Path,
    harness: &Harness,
    tarball: &[u8],
    version: &str,
) -> Result<()> {
    let dest = harness.install_path(repo);
    if dest.exists() {
        std::fs::remove_dir_all(&dest)
            .with_context(|| format!("clearing previous install at {}", dest.display()))?;
    }
    extract_content(tarball, &dest)?;
    version_md::write(&dest, harness.name, version)?;
    Ok(())
}

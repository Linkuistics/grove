// The LLM-driven CLI surface — `grove-llm`. See ADR-0006
// (`docs/adr/0006-grove-llm-binary-separation.md`) for the audience-split
// rationale: every verb here exists for the LLM driving a grove session to
// invoke deterministically, not for a human at a terminal.
//
// Verbs are flat (hyphenated) so a single `grove-llm --help` enumerates every
// verb the LLM might call — important for bootstrap-recovery if a session
// drops context (parent BRIEF Q3 of leaf 080).

use crate::cli::{InboxAddArgs, InboxDrainArgs};
use crate::inboxes;
use crate::repo;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::Read;

#[derive(Parser)]
#[command(
    name = "grove-llm",
    version,
    about = "Grove: LLM-driven verbs for mid-session use",
    long_about = "Verbs the LLM driving a grove session invokes deterministically. \
Audience-split from the human-facing `grove` binary per ADR-0006; \
none of these verbs are meant for direct human use."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Append an observation to the inbox of `<name>` on the `grove-meta` branch.
    InboxAdd(InboxAddArgs),
    /// Drain the inbox of `<name>`: two-phase. No disposition flags →
    /// enumerate pending observation paths. Any
    /// `--incorporated`/`--deferred`/`--rejected` paths → finalize by
    /// deleting the triaged files in one commit.
    InboxDrain(InboxDrainArgs),
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::InboxAdd(args) => cmd_inbox_add(&args),
        Command::InboxDrain(args) => cmd_inbox_drain(&args),
    }
}

fn cmd_inbox_add(args: &InboxAddArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let observation = read_body(args)?;
    inboxes::capture(&repo_path, &args.to, &observation, args.slug.as_deref())
}

fn cmd_inbox_drain(args: &InboxDrainArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let name = &args.for_grove;
    let has_dispositions =
        !args.incorporated.is_empty() || !args.deferred.is_empty() || !args.rejected.is_empty();

    if has_dispositions {
        return inboxes::drain_finalize(
            &repo_path,
            name,
            &args.incorporated,
            &args.deferred,
            &args.rejected,
        );
    }

    let paths = inboxes::drain_enumerate(&repo_path, name)?;
    for p in &paths {
        println!("{}", p.display());
    }
    if paths.is_empty() {
        eprintln!("inbox {}: no pending observations", name);
    } else {
        eprintln!(
            "inbox {}: {} pending observation{}",
            name,
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        );
    }
    Ok(())
}

fn read_body(args: &InboxAddArgs) -> Result<String> {
    if let Some(b) = &args.body {
        return Ok(b.clone());
    }
    if let Some(p) = &args.body_file {
        return std::fs::read_to_string(p)
            .with_context(|| format!("reading body file {}", p.display()));
    }
    if args.body_stdin {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).context("reading body from stdin")?;
        return Ok(s);
    }
    anyhow::bail!("provide observation via --body, --body-file, or --body-stdin");
}

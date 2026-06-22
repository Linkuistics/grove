use crate::cli::{NameArgs, RetireArgs, StartArgs};
use crate::harness::Harness;
use crate::harness_stamp;
use crate::repo;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn start(args: &StartArgs) -> Result<()> {
    let repo_path = repo::resolve(None)?;
    let harness =
        harness_stamp::resolve_for_launch(&repo_path, &args.name, args.harness.as_deref())?;

    let worktree =
        repo::create_grove_worktree(&repo_path, &args.name, args.start_point.as_deref())?;
    harness_stamp::maybe_stamp(&repo_path, &args.name, harness)?;

    if args.no_launch {
        eprintln!(
            "grove: worktree ready at {} (no-launch)",
            worktree.display()
        );
        return Ok(());
    }
    let prompt = load_prompt("start")?;
    exec_harness(harness, &repo_path, &worktree, &args.name, &prompt)
}

pub fn continue_grove(args: &NameArgs) -> Result<()> {
    launch_existing(args, "continue")
}

/// State-dispatching launcher: the sole lifecycle entry verb. It ensures the
/// worktree exists (creating a new grove, re-attaching an orphaned one, or
/// using a live one), then drives the **whole self-driving loop** (ADR-0032):
/// one fresh foreground `claude` per task, relaunching on each completion
/// signal until the agent stops signalling (empty `pick` → finish, or a human
/// interrupt). The new-grove path honours `--start-point`; resume paths ignore
/// it (the branch already exists). The launched sessions handle all in-context
/// judgement — including proposing the complete finish cycle once the grove has
/// no live leaves left.
pub fn do_grove(args: &StartArgs) -> Result<()> {
    // Provision the global skill from the embedded methodology (ADR-0031/0034):
    // extract content/ to ~/.claude/skills/grove/ idempotently, so the skill the
    // launched session reads can never drift from this binary. `grove do` is the
    // entry the human always hits; a warm dir is a cheap no-op.
    crate::provision::provision_global_skill()?;

    let repo_path = repo::resolve(None)?;
    let harness =
        harness_stamp::resolve_for_launch(&repo_path, &args.name, args.harness.as_deref())?;

    let worktree = repo::grove_worktree(&repo_path, &args.name);
    let worktree = if worktree.is_dir() {
        worktree
    } else if repo::branch_exists(&repo_path, &args.name)? {
        let attached = repo::attach_grove_worktree(&repo_path, &args.name)?;
        eprintln!(
            "grove: re-attached worktree at {} (branch existed but worktree was gone)",
            attached.display()
        );
        attached
    } else {
        let wt = repo::create_grove_worktree(&repo_path, &args.name, args.start_point.as_deref())?;
        harness_stamp::maybe_stamp(&repo_path, &args.name, harness)?;
        wt
    };

    // Adoption-migrate (ADR-0034/0035): before driving, flip an old-format
    // `.grove/` (v1-flat or `NNN-slug`) to the v2 directory scheme in one
    // reviewable commit, so every task the loop launches sees only v2. A no-op on
    // a v2/empty/absent tree, so it is safe on every `grove do` (idempotent;
    // restart ≡ continuation).
    if let crate::tree_migrate::Outcome::Migrated(renames) =
        crate::tree_migrate::migrate_on_adoption(&worktree, &args.name)?
    {
        eprintln!(
            "grove: migrated {} task-tree file{} to the v2 directory scheme (committed for review)",
            renames.len(),
            if renames.len() == 1 { "" } else { "s" }
        );
    }

    if args.no_launch {
        eprintln!(
            "grove: worktree ready at {} (no-launch)",
            worktree.display()
        );
        return Ok(());
    }

    crate::loop_driver::run(harness, &repo_path, &worktree, &args.name)
}

fn launch_existing(args: &NameArgs, verb: &str) -> Result<()> {
    let repo_path = repo::resolve(None)?;
    let harness =
        harness_stamp::resolve_for_launch(&repo_path, &args.name, args.harness.as_deref())?;

    let worktree = repo::grove_worktree(&repo_path, &args.name);
    if !worktree.is_dir() {
        anyhow::bail!(
            "no worktree for grove '{}' at {} — run `grove do {}` first",
            args.name,
            worktree.display(),
            args.name
        );
    }

    if args.no_launch {
        eprintln!("grove: would exec {} (no-launch)", harness.exec_bin);
        return Ok(());
    }
    let prompt = load_prompt(verb)?;
    exec_harness(harness, &repo_path, &worktree, &args.name, &prompt)
}

pub fn retire(args: &RetireArgs) -> Result<()> {
    let repo_path = repo::resolve(None)?;
    let (name, node_path) = args
        .path
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("expected <name>/<node-path>, got `{}`", args.path))?;
    let harness = harness_stamp::resolve_for_launch(&repo_path, name, args.harness.as_deref())?;

    let worktree = repo::grove_worktree(&repo_path, name);
    if !worktree.is_dir() {
        anyhow::bail!("no worktree for grove '{}' at {}", name, worktree.display());
    }

    if args.no_launch {
        eprintln!(
            "grove: would exec {} for retire (no-launch)",
            harness.exec_bin
        );
        return Ok(());
    }
    let prompt = load_prompt("retire")?;
    let prompt = substitute(&prompt, &[("NODE_PATH", node_path)]);
    exec_harness(harness, &repo_path, &worktree, name, &prompt)
}

/// Read a launcher prompt from the **global** skill dir the binary provisions
/// (`~/.claude/skills/grove/prompts/`), not any repo-local mirror. `grove do`
/// provisions that dir at the top of [`do_grove`], so the loop always launches
/// off the *current* embedded prompts — the repoint that retired the old
/// `harness.install_path`-rooted read, which silently served stale mirrors.
pub(crate) fn load_prompt(verb: &str) -> Result<String> {
    let prompt_path = crate::provision::global_skill_dir()?
        .join("prompts")
        .join(format!("{}.md", verb));
    fs::read_to_string(&prompt_path)
        .with_context(|| format!("reading prompt {}", prompt_path.display()))
}

fn substitute(template: &str, vars: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (k, v) in vars {
        out = out.replace(&format!("{{{{{}}}}}", k), v);
    }
    out
}

fn exec_harness(
    harness: &Harness,
    repo_path: &Path,
    worktree: &Path,
    grove_name: &str,
    prompt: &str,
) -> Result<()> {
    let repo_name = repo_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repo".to_string());
    let session_name = format!("{}: {} grove", repo_name, grove_name);

    let mut cmd = Command::new(harness.exec_bin);
    cmd.current_dir(worktree);
    if !harness.name_args.is_empty() {
        cmd.args(harness.name_args).arg(&session_name);
    }
    cmd.arg(prompt);

    let status = cmd
        .status()
        .with_context(|| format!("execing {}", harness.exec_bin))?;
    if !status.success() {
        anyhow::bail!("{} exited non-zero", harness.exec_bin);
    }
    Ok(())
}

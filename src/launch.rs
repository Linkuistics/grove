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
    let harness = harness_stamp::resolve_for_launch(
        &repo_path,
        &args.name,
        args.harness.as_deref(),
    )?;

    let worktree = repo::create_grove_worktree(
        &repo_path,
        &args.name,
        args.start_point.as_deref(),
    )?;
    harness_stamp::maybe_stamp(&repo_path, &args.name, harness)?;

    let prompt = load_prompt(&repo_path, harness, "start")?;

    if args.no_launch {
        eprintln!("grove: worktree ready at {} (no-launch)", worktree.display());
        return Ok(());
    }
    exec_harness(harness, &repo_path, &worktree, &args.name, &prompt)
}

pub fn continue_grove(args: &NameArgs) -> Result<()> {
    launch_existing(args, "continue")
}

/// State-dispatching launcher: the sole lifecycle entry verb. It works
/// whether the named grove has been started yet, is currently live, or is
/// orphaned (branch present, worktree gone). The new-grove path honours
/// `--start-point`; the continue paths ignore it (the branch already exists).
/// The launched harness session handles any in-context judgement — including
/// proposing the complete finish cycle once the grove has no live leaves
/// left; this verb only dispatches.
pub fn do_grove(args: &StartArgs) -> Result<()> {
    let repo_path = repo::resolve(None)?;
    let cont = NameArgs {
        name: args.name.clone(),
        harness: args.harness.clone(),
        no_launch: args.no_launch,
    };
    let worktree = repo::grove_worktree(&repo_path, &args.name);
    if worktree.is_dir() {
        return continue_grove(&cont);
    }
    if repo::branch_exists(&repo_path, &args.name)? {
        let attached = repo::attach_grove_worktree(&repo_path, &args.name)?;
        eprintln!(
            "grove: re-attached worktree at {} (branch existed but worktree was gone)",
            attached.display()
        );
        return continue_grove(&cont);
    }
    start(args)
}

pub fn takeover(args: &NameArgs) -> Result<()> {
    launch_existing(args, "takeover")
}

fn launch_existing(args: &NameArgs, verb: &str) -> Result<()> {
    let repo_path = repo::resolve(None)?;
    let harness = harness_stamp::resolve_for_launch(
        &repo_path,
        &args.name,
        args.harness.as_deref(),
    )?;

    let worktree = repo::grove_worktree(&repo_path, &args.name);
    if !worktree.is_dir() {
        anyhow::bail!(
            "no worktree for grove '{}' at {} — run `grove do {}` first",
            args.name,
            worktree.display(),
            args.name
        );
    }

    let prompt = load_prompt(&repo_path, harness, verb)?;

    if args.no_launch {
        eprintln!("grove: would exec {} (no-launch)", harness.exec_bin);
        return Ok(());
    }
    exec_harness(harness, &repo_path, &worktree, &args.name, &prompt)
}

pub fn retire(args: &RetireArgs) -> Result<()> {
    let repo_path = repo::resolve(None)?;
    let (name, node_path) = args
        .path
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("expected <name>/<node-path>, got `{}`", args.path))?;
    let harness = harness_stamp::resolve_for_launch(
        &repo_path,
        name,
        args.harness.as_deref(),
    )?;

    let worktree = repo::grove_worktree(&repo_path, name);
    if !worktree.is_dir() {
        anyhow::bail!(
            "no worktree for grove '{}' at {}",
            name,
            worktree.display()
        );
    }

    let prompt = load_prompt(&repo_path, harness, "retire")?;
    let prompt = substitute(&prompt, &[("NODE_PATH", node_path)]);

    if args.no_launch {
        eprintln!("grove: would exec {} for retire (no-launch)", harness.exec_bin);
        return Ok(());
    }
    exec_harness(harness, &repo_path, &worktree, name, &prompt)
}

fn load_prompt(main_repo: &Path, harness: &Harness, verb: &str) -> Result<String> {
    let prompt_path = harness
        .install_path(main_repo)
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

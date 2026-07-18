use crate::cli::{RetireArgs, StartArgs};
use crate::harness::Harness;
use crate::harness_stamp;
use crate::repo;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

/// State-dispatching launcher: the sole lifecycle entry verb, run from inside
/// the working tree (user-owned-worktrees) — grove never creates, attaches, or
/// relocates it. The worktree is `git rev-parse --show-toplevel` of cwd, and
/// the grove name is its basename. Once resolved, it drives the **whole
/// self-driving loop** (self-driving-loop): one fresh foreground harness
/// session per task, relaunching on each completion signal until the agent stops
/// signalling (empty `pick` → finish, or a human interrupt). The launched
/// sessions handle all in-context judgement — including proposing the
/// complete finish cycle once the grove has no live leaves left.
pub fn do_grove(args: &StartArgs) -> Result<()> {
    let worktree = repo::git_toplevel(&std::env::current_dir().context("getting cwd")?)?;
    let name = worktree_name(&worktree);

    let repo_path = repo::resolve(None)?;
    let harness = harness_stamp::resolve_for_launch(&repo_path, &name, args.harness.as_deref())?;

    // Provision the global skill from the embedded methodology for every
    // installed harness (and the launching one unconditionally), so the skill
    // any session reads can never drift from this binary.
    crate::provision::provision_all(harness)?;

    // Adoption-migrate (task-tree-scheme): before driving, flip an old-format
    // `.grove/` (v1-flat or `NNN-slug`) to the v2 directory scheme in one
    // reviewable commit, so every task the loop launches sees only v2. A no-op on
    // a v2/empty/absent tree, so it is safe on every `grove do` (idempotent;
    // restart ≡ continuation).
    if let crate::tree_migrate::Outcome::Migrated(renames) =
        crate::tree_migrate::migrate_on_adoption(&worktree, &name)?
    {
        eprintln!(
            "grove: migrated {} task-tree file{} to the v2 directory scheme (committed for review)",
            renames.len(),
            if renames.len() == 1 { "" } else { "s" }
        );
    }

    if args.no_launch {
        eprintln!("grove: ready in {} (no-launch)", worktree.display());
        return Ok(());
    }

    // The stamp is written only here, after provisioning and the no-launch
    // check have both already succeeded: a documented dry run must never
    // permanently rebind the grove (B3), and a provisioning failure — or a
    // harness whose binary isn't installed — must never leave a stamp with
    // no recovery path (B4). The pre-flight check covers not just the stamped
    // harness but every per-kind `GROVE_<KIND>_HARNESS` override configured
    // (harness-spawn-preflight-k8): a rerouted-but-uninstalled harness must
    // fail here, before the loop starts, not mid-run on the first leaf routed
    // to it.
    crate::loop_driver::preflight_check(harness)?;
    harness_stamp::maybe_stamp(&repo_path, &name, harness, args.harness.is_some())?;

    crate::loop_driver::run(harness, &repo_path, &worktree, &name)
}

pub fn retire(args: &RetireArgs) -> Result<()> {
    let worktree = repo::git_toplevel(&std::env::current_dir().context("getting cwd")?)?;
    let name = worktree_name(&worktree);

    let repo_path = repo::resolve(None)?;
    let harness = harness_stamp::resolve_for_launch(&repo_path, &name, args.harness.as_deref())?;

    if args.no_launch {
        eprintln!(
            "grove: would exec {} for retire (no-launch)",
            harness.exec_bin
        );
        return Ok(());
    }
    let prompt = load_prompt(harness, "retire")?;
    let prompt = substitute(&prompt, &[("NODE_PATH", &args.path)]);
    exec_harness(harness, &repo_path, &worktree, &name, &prompt)
}

/// The grove name is the worktree directory's basename (user-owned-worktrees).
fn worktree_name(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// Read a launcher prompt from the **global** skill dir the binary provisions
/// for `harness` (`~/.claude/skills/grove/prompts/`, or the equivalent
/// per-harness dir), not any repo-local mirror. `grove do` provisions every
/// installed harness's dir at the top of [`do_grove`], so the loop always
/// launches off the *current* embedded prompts — the repoint that retired the
/// old `harness.install_path`-rooted read, which silently served stale mirrors.
pub(crate) fn load_prompt(harness: &Harness, verb: &str) -> Result<String> {
    let prompt_path = crate::provision::skill_dir_for(harness)?
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

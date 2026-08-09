use crate::cli::{RetireArgs, StartArgs};
use crate::harness::Harness;
use crate::harness_stamp;
use crate::repo;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Drive the config-defined lifecycle from the current working tree. This is
/// the sole path reached by the human-facing bare command: provision installed
/// harnesses, acquire the workspace lease, and run one configured foreground
/// session per selected task until the agent stops signalling.
pub fn bare_grove() -> Result<()> {
    crate::provision::provision_installed()?;

    let cwd = std::env::current_dir().context("getting cwd")?;
    let driver_lease = crate::driver_lease::DriverLease::acquire(&cwd)?;
    let worktree = driver_lease.worktree_root().to_path_buf();
    let repository = repo::main_repo_of(&worktree)?;
    let name = worktree_name(&worktree);

    crate::loop_driver::run_configured(&repository, &worktree, &name, driver_lease)
}

/// Retained implementation of the removed `grove do` interface.
///
/// This compatibility seam remains reachable only by internal tests until the
/// dedicated legacy-removal work item deletes the old launcher.
pub fn do_grove(args: &StartArgs) -> Result<()> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let discovered_worktree = repo::workspace_control(&cwd)?.worktree_root().to_path_buf();
    let name = worktree_name(&discovered_worktree);

    let repo_path = repo::main_repo_of(&discovered_worktree)?;
    let harness = harness_stamp::resolve_for_launch(&repo_path, &name, args.harness.as_deref())?;

    // Provision the global skill from the embedded methodology for every
    // installed harness (and the launching one unconditionally), so the skill
    // any session reads can never drift from this binary.
    crate::provision::provision_all(harness)?;

    // Provisioning is independent delivery and deliberately remains available
    // even when another driver owns this working tree. Foreground ownership
    // begins here, before any task-tree observation, migration, config check,
    // or launch. The read-only no-launch path deliberately takes no lease and
    // therefore must return before the adoption migration below.
    // Resolve it from cwd's on-disk marker rather than Git discovery so ambient
    // GIT_DIR/GIT_WORK_TREE/TMPDIR values cannot split one workspace's lease.
    let driver_lease = if args.no_launch {
        None
    } else {
        Some(crate::driver_lease::DriverLease::acquire(&cwd)?)
    };
    let worktree = match driver_lease.as_ref() {
        Some(lease) => {
            lease
                .revalidate()
                .context("revalidating driver lease before lifecycle transition")?;
            lease.worktree_root().to_path_buf()
        }
        None => discovered_worktree,
    };
    let name = worktree_name(&worktree);

    // Both config checks run *above* the no-launch return, not below it
    // (no-launch-config-check-k20). The pre-flight check covers not just the
    // stamped harness but every per-kind `GROVE_<KIND>_HARNESS` override
    // configured (harness-spawn-preflight-k8): a rerouted-but-uninstalled
    // harness must fail here, before the loop starts, not mid-run on the first
    // leaf routed to it.
    crate::loop_driver::preflight_check(harness)?;

    if args.no_launch {
        // `--no-launch` is documented as *reporting readiness*, so it has to
        // resolve everything a launch would fail on rather than merely decline
        // to launch: once a kind with no model var became a hard error
        // (model-per-task-kind), a dry run that skipped the checks printed
        // `ready` and exited 0 on exactly the half-configured environments the
        // requirement exists to expose — the same partial-configuration
        // invisibility, back through the dry-run door.
        //
        // The guard still sits above `maybe_stamp` (branch-review-k14 B3):
        // reporting is free to read anything, but a documented dry run must
        // never permanently rebind the grove. Both checks it now runs are
        // side-effect free — pre-flight is a PATH lookup, and the kind peek
        // only spawns `grove-llm kind`, which reads the tree and writes
        // nothing.
        let readiness = crate::loop_driver::readiness(harness, &worktree)?;
        eprintln!(
            "grove: ready in {} — {readiness} (no-launch)",
            worktree.display()
        );
        return Ok(());
    }

    // Adoption-migrate (task-tree-scheme): before driving, flip an old-format
    // `.grove/` (v1-flat or `NNN-slug`) to the v2 directory scheme in one
    // reviewable commit, so every task the loop launches sees only v2. This is
    // below the no-launch return because readiness inspection is side-effect
    // free, and above every foreground tree operation while the lifetime lease
    // is held. A no-op on a v2/empty/absent tree (restart ≡ continuation).
    if let crate::tree_migrate::Outcome::Migrated(renames) =
        crate::tree_migrate::migrate_on_adoption(&worktree, &name)?
    {
        eprintln!(
            "grove: migrated {} task-tree file{} to the v2 directory scheme (committed for review)",
            renames.len(),
            if renames.len() == 1 { "" } else { "s" }
        );
    }

    // The stamp is written only here, after provisioning, the no-launch return
    // and the pre-flight check have all already succeeded: a provisioning
    // failure — or a harness whose binary isn't installed — must never leave a
    // stamp with no recovery path (branch-review-k14 B4).
    harness_stamp::maybe_stamp(&repo_path, &name, harness, args.harness.is_some())?;

    let driver_lease = driver_lease.context("driver lease missing before foreground launch")?;
    crate::loop_driver::run(harness, &repo_path, &worktree, &name, driver_lease)
}

pub fn retire(args: &RetireArgs) -> Result<()> {
    let worktree = repo::toplevel(&std::env::current_dir().context("getting cwd")?)?;
    let name = worktree_name(&worktree);

    let repo_path = repo::resolve(None)?;
    let harness = harness_stamp::resolve_for_launch(&repo_path, &name, args.harness.as_deref())?;

    // The prompt is loaded and the invocation assembled *above* the no-launch
    // return, for the reason `do`'s two config checks are
    // (no-launch-config-check-k20, and *model-per-task-kind*: "`--no-launch`
    // resolves the launch it declines to perform"). The rule generalises to this
    // verb even though what there is to resolve does not: `retire` peeks no leaf
    // and loads no model, so its whole resolution is the harness, the prompt and
    // the grants — and a dry run that stopped at the harness reported readiness
    // for a launch it had checked almost none of.
    //
    // The prompt is the sharp case and it is unique to this verb: `grove retire`
    // **never provisions** (only `do_grove` calls `provision_all`), so
    // `load_prompt` reads a global skill dir some *earlier* `grove do` had to
    // have written for this harness. That is the one launch dependency a user
    // cannot see and the one the old dry run sat directly on top of.
    let prompt = load_prompt(harness, "retire")?;
    let prompt = substitute(&prompt, &[("NODE_PATH", &args.path)]);

    if args.no_launch {
        // Built and dropped: assembling the invocation runs the codex sandbox
        // pre-flight and derives the VCS-store grants, which is the resolution
        // being reported on; a `Command` that is never spawned does nothing else.
        let _cmd = retire_command(harness, &repo_path, &worktree, &name, &prompt)?;
        // The exec is the one thing a dry run cannot inherit from the launch, so
        // it stands in the strongest available predicate on it. `harness.exec_bin`
        // and not `loop_driver::harness_bin`: that seam is the *loop*'s, and
        // `exec_harness` deliberately has none — checking the overridable name
        // here would report on a binary this verb never runs.
        if !crate::harness::exec_bin_on_path(harness.exec_bin) {
            anyhow::bail!(
                "{} is not on PATH, so this grove's retire session could not be \
                 launched on \"{}\" (nothing was launched)",
                harness.exec_bin,
                harness.name
            );
        }
        eprintln!(
            "grove: ready in {} — would exec {} for retire (no-launch)",
            worktree.display(),
            harness.exec_bin
        );
        return Ok(());
    }
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

/// codex-gitdir-grant: codex's `workspace-write` sandbox blocks the VCS
/// store writes grove's mandatory Commit and Retire steps depend on, so
/// every codex launch grants the store back via `--add-dir` — per-VCS,
/// because the two VCSes fail differently:
///
/// - **git tree**: the sandbox carves `.git` out of every writable root, so
///   the grant is the absolutized git common dir. One path covers both
///   shapes (a linked worktree's gitdir is a subpath of the common dir; a
///   plain checkout's common dir *is* `.git`).
/// - **jj-enabled tree**: `.jj` is *not* carved out (codex 0.145.0 protects
///   only `.git`/`.agents`/`.codex`), but a secondary workspace's ops all
///   land in the *main* workspace's `.jj/repo` — outside the sandbox cwd
///   entirely — so the grant is the main workspace's `.jj`; plus the main
///   `.git` when colocated, where jj's git backend writes commit objects
///   and exported refs into the carved-out gitdir. In a primary workspace
///   the `.jj` grant is redundant but harmless.
///
/// Grants are additive so the default writable roots stay intact, and inert
/// under `danger-full-access`. No other harness is touched. The values are
/// dynamic — derived from the worktree per launch — which is why this lives here
/// at the assembly sites rather than in `harness.rs`'s static flag templates.
///
/// Appended unconditionally but never blindly: under a `read-only` sandbox codex
/// **exits on these flags**, so every codex launch is gated by
/// [`check_codex_sandbox_accepts_grants`] first.
pub(crate) fn append_codex_vcs_store_grant(
    cmd: &mut Command,
    harness: &Harness,
    worktree: &Path,
) -> Result<()> {
    if harness.name != "codex" {
        return Ok(());
    }
    match repo::vcs_of(worktree) {
        Some(repo::Vcs::Jj { .. }) => {
            let main = repo::main_repo_of(worktree)?;
            cmd.arg("--add-dir").arg(main.join(".jj"));
            let git_store = main.join(".git");
            if git_store.exists() {
                cmd.arg("--add-dir").arg(git_store);
            }
        }
        _ => {
            cmd.arg("--add-dir").arg(repo::git_common_dir(worktree)?);
        }
    }
    Ok(())
}

/// The loop driver's **launch-scoped environment** (self-driving-loop) — the
/// variables that carry authority or identify the routed producer, and the
/// exact set [`scrub_loop_control_env`] removes.
///
/// `GROVE_SIGNAL_FILE` is the driver's kill channel: it watches that path while
/// its harness child runs and applies grace → SIGTERM → kill-grace → SIGKILL the
/// moment the file *appears*. Whoever holds the variable can therefore end the
/// session, and the environment is inherited by every descendant — so the
/// authority is ambient unless each spawn scopes it deliberately.
/// `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` are the retired pre-watcher handles
/// (driver-side-kill), kept here because a stale, unrelated PID leaking into a
/// nested grove is the same class of mistake one notch quieter.
/// `GROVE_SESSION_TARGET` is advisory rather than authoritative, but stale
/// metadata could misattribute a later producer retirement, so it follows the
/// same scrub-by-default rule.
const LOOP_CONTROL_ENV: [&str; 4] = [
    "GROVE_SIGNAL_FILE",
    "GROVE_HARNESS_PID",
    "GROVE_CLAUDE_PID",
    crate::task_relationship::SESSION_TARGET_ENV,
];

/// Shipped deterministic failure seams must never leak from a developer shell
/// into a configured session. They are internal test controls, not launch
/// configuration.
const FINISH_CLEANUP_TEST_ENV: [&str; 3] = [
    "GROVE_TEST_FINISH_CLEANUP_FAIL_AT",
    "GROVE_TEST_FINISH_CLEANUP_PAUSE_AT",
    "GROVE_TEST_FINISH_CLEANUP_BARRIER",
];

/// Repository selectors are process-global overrides: `current_dir` alone does
/// not stop Git-aware children from following an inherited foreign repository.
const REPOSITORY_CONTEXT_ENV: [&str; 3] = ["GIT_DIR", "GIT_WORK_TREE", "GIT_COMMON_DIR"];

/// **Any harness spawn that is not the session itself must scrub the loop's
/// launch-scoped environment** (guard-loop-signal-k37,
/// codex-grant-refused-k35).
///
/// Authority to end a `grove do` session is granted by an environment variable,
/// and an environment is inherited, not addressed: a spawn that merely declines
/// to *set* `GROVE_SIGNAL_FILE` still hands its child whatever the driver's own
/// environment carried. Scrubbing is therefore the default and granting is the
/// exception — [`crate::loop_driver`]'s session spawn calls this too, and then
/// sets the one path it owns.
///
/// The failure this closes was not hypothetical. This repo is a meta-grove, so
/// its own suite runs as a *descendant* of a live session; the codex sandbox
/// pre-flight below spawned the harness binary without scrubbing, the suite's
/// fake harness writes `"$GROVE_SIGNAL_FILE"` unconditionally, and `cargo test`
/// killed the terminal it was typed into. In production the same leak is latent
/// rather than fatal — a real `codex exec` writes no such file — but "latent"
/// is a property of today's harnesses, not of the rule.
///
/// Deliberately one helper rather than an `env_remove` per site: the list is the
/// interesting part, and a second site open-coding it is how the first one came
/// to be missed.
pub(crate) fn scrub_loop_control_env(cmd: &mut Command) {
    for name in LOOP_CONTROL_ENV.into_iter().chain(FINISH_CLEANUP_TEST_ENV) {
        cmd.env_remove(name);
    }
}

/// Driver-internal and obsolete compatibility children must also ignore any
/// repository selected by the process that launched Grove. Internal Git calls
/// may subsequently anchor the authoritative worktree explicitly.
pub(crate) fn scrub_internal_child_env(cmd: &mut Command) {
    scrub_loop_control_env(cmd);
    for name in REPOSITORY_CONTEXT_ENV {
        cmd.env_remove(name);
    }
}

/// How long [`probe_codex_sandbox`] waits for codex to print its header before
/// giving up and letting the launch proceed. Generous by two orders of
/// magnitude: the header lands in **0.1–0.4s** even against a config with MCP
/// servers and hooks (measured, codex-cli 0.145.0), because it is printed before
/// any of them spin up. The budget is for a pathologically slow cold start, not
/// for the common case — a probe that timed out on a machine under load would
/// silently stop guarding.
const SANDBOX_PROBE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

/// What codex says the sandbox for *this* launch would be — the three answers
/// [`probe_codex_sandbox`] distinguishes.
enum SandboxVerdict {
    /// A mode that accepts `--add-dir`: `workspace-write` or
    /// `danger-full-access`.
    GrantsAccepted,
    /// `read-only`. The grants are refused *fatally* and the session could not
    /// commit even if they were not.
    ReadOnly,
    /// The probe got no usable answer — codex could not be spawned, printed no
    /// header inside [`SANDBOX_PROBE_TIMEOUT`], or named a mode this build does
    /// not know. Always proceeds: grove guides, it does not gate (constraint 5),
    /// and a probe that cannot answer must never be the thing that stops a loop.
    Unknown,
}

/// Refuse a codex launch **before spawning** when codex would reject the VCS
/// store grants [`append_codex_vcs_store_grant`] is about to pass
/// (codex-gitdir-grant).
///
/// This is the pre-flight that ADR *codex-gitdir-grant* rejected — "with the
/// flags structural, the failure mode it would detect no longer exists" —
/// reinstated because its own reopen condition fired: an unexplained codex
/// launch failure surfaced in the field (codex-grant-refused-k35).
///
/// **The third case the ADR's "harmless when the sandbox is off" missed** is not
/// an exotic configuration, it is the default one. codex's effective sandbox is
/// `read-only` for any project the user has not **trusted**, and trust is
/// per-directory with **no inheritance from parent directories** — so a
/// brand-new working tree, which is exactly what `grove do` bootstraps into, is
/// untrusted by construction. Under `read-only`, `--add-dir` is not ignored with
/// a warning despite saying "Ignoring": codex exits **1 in ~0ms, before drawing
/// any TUI**, printing one line. The loop then stopped on what looked to the
/// operator like a mute non-signal exit.
///
/// **Refuse rather than elevate.** grove could pass `--sandbox workspace-write`
/// and make every launch succeed, but the sandbox posture is the user's call,
/// not grove's: codex's trust prompt exists so a human answers it once, and a
/// process tool has no mandate to route around it on every launch. (Contrast the
/// `.git` grant itself, which the ADR argues sits *inside* the trust a user
/// extends by letting an agent commit at all — a project they have never trusted
/// extends no such trust.) Refusing costs the human one action, once per working
/// tree, and this diagnostic names it.
///
/// **Refuse rather than degrade**, equally: `-c sandbox_workspace_write.
/// writable_roots=[…]` is silently ignored under `read-only` instead of being
/// fatal (measured in a virgin repo), so swapping the flag form would buy a
/// launch that comes up and then cannot commit — worse than one that refuses,
/// because grove's Commit and Retire steps are mandatory.
///
/// **There are no false refusals, by construction.** The verdict is codex's own
/// policy builder answering about this exact argv, not a reimplementation of its
/// trust rules — which could not be reimplemented safely anyway, since an
/// explicit `sandbox_mode` (in the config, in a profile layer, or on the command
/// line) overrides the trust default. If it says `read-only`, the session
/// genuinely could not have committed.
pub(crate) fn check_codex_sandbox_accepts_grants(
    bin: &str,
    harness: &Harness,
    worktree: &Path,
    model: Option<&str>,
) -> Result<()> {
    if harness.name != "codex" {
        return Ok(());
    }
    match probe_codex_sandbox(bin, harness, worktree, model) {
        SandboxVerdict::GrantsAccepted | SandboxVerdict::Unknown => Ok(()),
        SandboxVerdict::ReadOnly => anyhow::bail!(
            "codex reports its sandbox in {} as `read-only`, so it would refuse the VCS-store \
             grants a grove session needs — codex exits at startup on `--add-dir` under a \
             read-only sandbox, and a read-only session could not run grove's mandatory Commit \
             and Retire steps in any case. Nothing was launched.\n\
             \x20      Fix it either way, then re-run:\n\
             \x20        - trust the project: run `codex` once in this directory and accept the \
             trust prompt, or add to {codex_home}/config.toml:\n\
             \x20              [projects.\"{worktree}\"]\n\
             \x20              trust_level = \"trusted\"\n\
             \x20        - or give it a writable sandbox: `sandbox_mode = \"workspace-write\"` in \
             {codex_home}/config.toml{profile_note}",
            worktree.display(),
            codex_home = codex_home_label(),
            worktree = worktree.display(),
            profile_note = match model {
                // grove routes codex *models* through `--profile`, and codex's
                // `--profile` is a whole config layer — so the layer this leaf's
                // kind selects is a place `sandbox_mode` can live, and the one
                // the operator is least likely to think of.
                Some(model) => format!(
                    ", or in the profile layer this leaf routes to, {}/{model}.config.toml",
                    codex_home_label()
                ),
                None => String::new(),
            }
        ),
    }
}

/// Where codex reads its config, for the diagnostic only — `$CODEX_HOME` when
/// set, else the documented default. Never used to *find* anything: the probe
/// asks codex itself rather than reading its config, so this is a label, not a
/// lookup.
fn codex_home_label() -> String {
    std::env::var("CODEX_HOME").unwrap_or_else(|_| "~/.codex".to_string())
}

/// Ask codex what sandbox this launch would get, by running `codex exec` with
/// **the same model flags and the same grants** the real launch will pass and
/// reading the one header line it prints before doing anything else.
///
/// **Why `codex exec` and not the launch itself**: the header names the resolved
/// mode (`read-only`, `workspace-write [roots…]`, `danger-full-access`) and is
/// printed by the same policy builder the TUI uses — the distinction ADR
/// *codex-gitdir-grant* already records, where the trap is `codex sandbox`,
/// which models a *different* policy path. Verified in both directions against
/// 0.145.0: the TUI is fatal exactly when this header reads `read-only`, and
/// comes up exactly when it does not.
///
/// **Why the model flags must be passed**: codex's `--profile` is not a model
/// flag, it is "layer `$CODEX_HOME/<name>.config.toml` on top of the base user
/// config" — and that layer can set `sandbox_mode`. So whether the grants are
/// accepted is a property of *(tree, model)*, not of the tree alone, which is
/// why this cannot be hoisted into `loop_driver::preflight_check` and run once
/// per `grove do`: before a leaf is picked, the question has no single answer.
///
/// **The probe is free.** It is killed the instant the header arrives — before
/// codex issues any request — so it costs no tokens, leaves no rollout, and
/// writes no project-trust entry. Measured at 0.1–0.4s against a real config.
/// (Probes that *do* mutate what they measure exist here: `--sandbox
/// workspace-write` persists `trust_level = "trusted"` for the project. This one
/// passes no such flag, deliberately.) Its one residue is a lock file in codex's
/// own `$CODEX_HOME/tmp/arg0/` scratch, left because the kill pre-empts codex's
/// cleanup — and swept by codex itself on its next full run, so it is bounded,
/// not cumulative.
///
/// The header goes to **stderr**, not stdout.
fn probe_codex_sandbox(
    bin: &str,
    harness: &Harness,
    worktree: &Path,
    model: Option<&str>,
) -> SandboxVerdict {
    use std::io::BufRead;
    use std::process::Stdio;

    let mut cmd = Command::new(bin);
    cmd.arg("exec");
    // `codex exec` refuses to start when the cwd is **neither trusted nor inside
    // a git repo** — one line to stderr, exit 1, before any header — and the TUI
    // this probe stands in for has no such gate. Without the flag the probe was
    // therefore mute in exactly the case it exists for: `untrusted` is what makes
    // the sandbox `read-only`, so in a **jj-native** tree the two conditions
    // arrive together, the verdict degraded to `Unknown`, and the launch went
    // ahead into the death this pre-flight is here to predict. The gate is
    // `codex exec`'s alone, so clearing it is what makes the probe *be* the TUI
    // rather than a stricter cousin of it; it moves no policy — a probe run with
    // the flag in an untrusted jj-native tree reports the same `read-only` the
    // same tree reports with a `.git` beside it.
    cmd.arg("--skip-git-repo-check");
    if let Some(model) = model {
        if !harness.model_args.is_empty() {
            cmd.args(harness.model_args).arg(model);
        }
    }
    if append_codex_vcs_store_grant(&mut cmd, harness, worktree).is_err() {
        // The store could not be resolved. The real launch is about to fail on
        // the same call and say so properly; this must not pre-empt it with a
        // sandbox diagnostic for what is not a sandbox problem.
        return SandboxVerdict::Unknown;
    }
    // An empty prompt, so a probe that somehow outlives its kill answers
    // nothing and costs almost nothing.
    cmd.arg("");
    cmd.current_dir(worktree)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    // This is a harness spawn that is *not* the session, so it gets no authority
    // over the live one — see `scrub_loop_control_env`, whose rule this site is
    // the reason for.
    scrub_internal_child_env(&mut cmd);

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(_) => return SandboxVerdict::Unknown,
    };
    let stderr = match child.stderr.take() {
        Some(stderr) => stderr,
        None => {
            let _ = child.kill();
            let _ = child.wait();
            return SandboxVerdict::Unknown;
        }
    };

    // Read on a detached thread so a codex that prints nothing cannot wedge the
    // driver: the deadline belongs to *this* thread, and the reader dies on its
    // own when the kill below closes the pipe.
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for line in std::io::BufReader::new(stderr)
            .lines()
            .map_while(Result::ok)
        {
            if let Some(mode) = line.trim().strip_prefix("sandbox:") {
                let _ = tx.send(mode.trim().to_string());
                return;
            }
        }
    });
    let verdict = match rx.recv_timeout(SANDBOX_PROBE_TIMEOUT) {
        // `workspace-write` arrives as `workspace-write [workdir, /tmp, …]`, so
        // match on the leading token rather than the whole line.
        Ok(mode) if mode.starts_with("read-only") => SandboxVerdict::ReadOnly,
        Ok(mode)
            if mode.starts_with("workspace-write") || mode.starts_with("danger-full-access") =>
        {
            SandboxVerdict::GrantsAccepted
        }
        // A mode this build has never seen, or no header at all.
        Ok(_) | Err(_) => SandboxVerdict::Unknown,
    };
    let _ = child.kill();
    let _ = child.wait();
    verdict
}

fn exec_harness(
    harness: &Harness,
    repo_path: &Path,
    worktree: &Path,
    grove_name: &str,
    prompt: &str,
) -> Result<()> {
    let mut cmd = retire_command(harness, repo_path, worktree, grove_name, prompt)?;
    let status = cmd
        .status()
        .with_context(|| format!("execing {}", harness.exec_bin))?;
    if !status.success() {
        anyhow::bail!("{} exited non-zero", harness.exec_bin);
    }
    Ok(())
}

/// Assemble the `grove retire` invocation — everything the launch resolves
/// **except the exec itself**, which is the only step [`retire`]'s `--no-launch`
/// dry run skips.
///
/// Extracted for that dry run rather than inlined, so the report and the launch
/// it predicts cannot come to disagree: `--no-launch` runs this identical code
/// path rather than a parallel re-derivation of it (*model-per-task-kind*, the
/// same reason `readiness` reads `launch_verb` from the driver).
fn retire_command(
    harness: &Harness,
    repo_path: &Path,
    worktree: &Path,
    grove_name: &str,
    prompt: &str,
) -> Result<Command> {
    let repo_name = repo_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repo".to_string());
    let session_name = format!("{}: {} grove", repo_name, grove_name);

    // Both launch sites, exactly like the grant it guards: a `grove retire`
    // session commits too, so it dies on the same refusal and wants the same
    // diagnostic. `model: None` because this path does no model routing — there
    // is no profile layer here that could move the answer.
    check_codex_sandbox_accepts_grants(harness.exec_bin, harness, worktree, None)?;

    let mut cmd = Command::new(harness.exec_bin);
    cmd.current_dir(worktree);
    if !harness.name_args.is_empty() {
        cmd.args(harness.name_args).arg(&session_name);
    }
    append_codex_vcs_store_grant(&mut cmd, harness, worktree)?;
    cmd.arg(prompt);
    // A `grove retire` session is a one-off with no driver watching it, so it is
    // not "the session itself" in `scrub_loop_control_env`'s sense: run from
    // inside a live `grove do`, it would otherwise inherit that loop's kill
    // channel and its `grove-llm complete` would end someone else's task.
    scrub_internal_child_env(&mut cmd);

    Ok(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Whether `cmd` will actively **remove** `key` from the child's inherited
    /// environment — which [`env_delta`] cannot express, since it reports `None`
    /// both for "sets nothing" and for "removes it". The difference is the whole
    /// point here: not setting a variable leaves the parent's value in place.
    fn env_is_scrubbed(cmd: &Command, key: &str) -> bool {
        let key = std::ffi::OsStr::new(key);
        cmd.get_envs().any(|(k, value)| k == key && value.is_none())
    }

    // The rule, pinned on the site that broke it: a harness spawn that is not
    // the session must hand down no authority to end one. Asserted as a
    // *removal*, not as "we didn't set it" — the bug this closes was precisely
    // that the probe set nothing and the child inherited the live path anyway.
    #[test]
    fn a_scrubbed_spawn_removes_the_whole_control_channel() {
        let mut cmd = Command::new("true");
        for name in REPOSITORY_CONTEXT_ENV {
            cmd.env(name, "preserved");
        }
        let finish_cleanup_test_env = [
            "GROVE_TEST_FINISH_CLEANUP_FAIL_AT",
            "GROVE_TEST_FINISH_CLEANUP_PAUSE_AT",
            "GROVE_TEST_FINISH_CLEANUP_BARRIER",
        ];
        for name in finish_cleanup_test_env {
            cmd.env(name, "must-not-leak");
        }
        scrub_loop_control_env(&mut cmd);
        for name in LOOP_CONTROL_ENV {
            assert!(
                env_is_scrubbed(&cmd, name),
                "{name} must be removed, not merely left unset — an environment \
                is inherited, not addressed"
            );
        }
        for name in finish_cleanup_test_env {
            assert!(
                env_is_scrubbed(&cmd, name),
                "{name} must not affect a configured session"
            );
        }
        for name in REPOSITORY_CONTEXT_ENV {
            assert!(
                !env_is_scrubbed(&cmd, name),
                "{name} is configured-command policy and must remain inherited"
            );
        }
    }

    #[test]
    fn an_internal_child_scrubs_control_and_repository_context() {
        let mut cmd = Command::new("true");
        scrub_internal_child_env(&mut cmd);
        for name in LOOP_CONTROL_ENV.into_iter().chain(REPOSITORY_CONTEXT_ENV) {
            assert!(env_is_scrubbed(&cmd, name), "{name} must be removed");
        }
    }
}

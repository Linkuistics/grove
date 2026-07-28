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
/// relocates it. The worktree is cwd's working-tree root ([`repo::toplevel`]:
/// the jj workspace root in a jj-enabled tree, else git's toplevel), and
/// the grove name is its basename. Once resolved, it drives the **whole
/// self-driving loop** (self-driving-loop): one fresh foreground harness
/// session per task, relaunching on each completion signal until the agent stops
/// signalling (empty `pick` → finish, or a human interrupt). The launched
/// sessions handle all in-context judgement — including proposing the
/// complete finish cycle once the grove has no live leaves left.
pub fn do_grove(args: &StartArgs) -> Result<()> {
    let worktree = repo::toplevel(&std::env::current_dir().context("getting cwd")?)?;
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
        // The guard still sits above `maybe_stamp` (B3): reporting is free to
        // read anything, but a documented dry run must never permanently rebind
        // the grove. Both checks it now runs are side-effect free — pre-flight
        // is a PATH lookup, and the kind peek only spawns `grove-llm kind`,
        // which reads the tree and writes nothing.
        let readiness = crate::loop_driver::readiness(harness, &worktree)?;
        eprintln!(
            "grove: ready in {} — {readiness} (no-launch)",
            worktree.display()
        );
        return Ok(());
    }

    // The stamp is written only here, after provisioning, the no-launch return
    // and the pre-flight check have all already succeeded: a provisioning
    // failure — or a harness whose binary isn't installed — must never leave a
    // stamp with no recovery path (B4).
    harness_stamp::maybe_stamp(&repo_path, &name, harness, args.harness.is_some())?;

    crate::loop_driver::run(harness, &repo_path, &worktree, &name)
}

pub fn retire(args: &RetireArgs) -> Result<()> {
    let worktree = repo::toplevel(&std::env::current_dir().context("getting cwd")?)?;
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

/// The loop driver's **control channel** (self-driving-loop) — the variables
/// that carry authority over a live session, and the exact set
/// [`scrub_loop_control_env`] removes.
///
/// `GROVE_SIGNAL_FILE` is the driver's kill channel: it watches that path while
/// its harness child runs and applies grace → SIGTERM → kill-grace → SIGKILL the
/// moment the file *appears*. Whoever holds the variable can therefore end the
/// session, and the environment is inherited by every descendant — so the
/// authority is ambient unless each spawn scopes it deliberately.
/// `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` are the retired pre-watcher handles
/// (driver-side-kill), kept here because a stale, unrelated PID leaking into a
/// nested grove is the same class of mistake one notch quieter.
const LOOP_CONTROL_ENV: [&str; 3] = ["GROVE_SIGNAL_FILE", "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID"];

/// **Any harness spawn that is not the session itself must scrub the loop's
/// control environment** (guard-loop-signal-k37, codex-grant-refused-k35).
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
    for name in LOOP_CONTROL_ENV {
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
    scrub_loop_control_env(&mut cmd);

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

/// Tell herdr which agent this pane is really running, via its documented
/// `HERDR_AGENT=<agent>` foreground-process hint (herdr-pane-misdetection).
///
/// herdr identifies a pane's agent from the foreground **process group**: it
/// prefers the group *leader*, and only falls back to scoring every member when
/// the leader is unrecognised. In a grove pane the leader is `grove` itself,
/// which herdr cannot identify, so the fallback runs — and a `codex mcp-server`
/// helper can outrank the harness grove actually launched. The pane then reads
/// `codex` whatever it is running, and herdr evaluates the wrong agent's screen
/// manifest against the TUI. `HERDR_AGENT` is the extension point upstream added
/// for exactly this shape — a host-visible wrapper hiding the real agent — and
/// `grove do` **is** one.
///
/// **On the child, never on grove.** herdr's doc says to set it "on the wrapper
/// command", but grove cannot rewrite its own exec-time environment (which is
/// what herdr reads, via `kern_procargs2` / `/proc/<pid>/environ`) and does not
/// need to: the probe consults every **non-leader** member of the foreground job
/// for a hint *before* the group scoring that misfires. Per-launch is also more
/// accurate than a wrapper-level export, because grove's harness varies per leaf
/// (model-per-task-kind).
///
/// **Both launch sites**, unlike [`append_turn_hooks`] — which is deliberately
/// driver-only, because a `grove retire` session sets no signal file and every
/// turn end there would read `blocked`. The hint carries no such discriminator: a
/// `grove retire` pane is mis-detected exactly as a `grove do` pane is, so it
/// wants the same fix.
///
/// **Set unconditionally, not gated on [`crate::herdr::in_pane`]** — the one
/// asymmetry with the turn hooks, and the reasons are three. The hooks are gated
/// because injecting them *changes the launch argv* and arms a subprocess that
/// fires on every tool call; an environment variable changes no argv and spawns
/// nothing, so the "absent herdr, the launch is byte-identical" discipline is
/// untouched either way. Nothing but herdr reads the variable, and it does so
/// only by inspecting *another* process's environ — herdr's own code never
/// `getenv`s it — so outside a herdr pane it is inert for want of a reader. And
/// the gate's failure mode is the asymmetric one: `in_pane` needs all three of
/// `HERDR_ENV`/`HERDR_SOCKET_PATH`/`HERDR_PANE_ID`, while the detection this
/// feeds needs none of them, so any pane that loses the pane environment but
/// keeps the process group would lose the fix in precisely the case it exists
/// for. Gating would buy purity and risk the bug back.
///
/// The value is `harness.name` with no translation table: grove's three harness
/// names are already herdr's three canonical labels (`detect::lookup_agent`). A
/// name herdr does not know parses to nothing and degrades to today's behaviour,
/// so a fourth harness costs a mis-detected pane, never a wrongly-detected one.
pub(crate) fn set_herdr_agent_hint(cmd: &mut Command, harness: &Harness) {
    cmd.env("HERDR_AGENT", harness.name);
}

/// Inject the **turn hooks** into a claude launch (herdr-turn-hooks,
/// herdr-mid-turn-blockers).
///
/// The loop driver is the harness's parent, so it can see a session start and a
/// session end and nothing between them. A session that stalls *mid-session* on
/// a question therefore reads `working` forever — the overnight case the status
/// surface exists to fix, and the half driver-level reporting structurally
/// cannot reach. Only the harness knows a turn ended, so grove asks it, with
/// hooks that report back through `grove-llm report-turn`. The same argument
/// runs one level deeper for a stall *inside* a turn — a permission prompt —
/// which is the mid-turn pair in [`turn_hook_settings`].
///
/// **Per launch, persisting nothing.** claude's `--settings` takes an inline
/// JSON string as an *additional* settings source, and hooks are **unioned**
/// across sources (measured: a project `Stop` hook and a `--settings` `Stop`
/// hook both fire). So grove contends with nothing — not herdr's own installed
/// `SessionStart` hook, not the user's — writes to no file the user owns, and
/// leaves nothing behind when the loop stops. Persisting the hooks in
/// `settings.json` was the alternative and is strictly worse: it is a mutation
/// of the user's configuration that outlives the grove, in the same file
/// herdr's installer writes.
///
/// Three gates, all of them narrowing:
///
/// - **claude only.** codex has no turn-end hook event at all (its set is
///   `pre_tool_use`, `permission_request`, `post_tool_use`, `pre_compact`,
///   `post_compact`, `session_start`, `session_end`, `user_prompt_submit`,
///   `subagent_start`, `subagent_stop`), and its hook trust is persisted per
///   source-and-content-hash, so an injected hook has no trust record; pi has
///   herdr's own full-lifecycle extension already reporting on the same events.
///   See ADR *herdr-turn-boundary-hooks*.
/// - **Under herdr only**, via [`crate::herdr::in_pane`] — see its note on why
///   the *injection* is gated and not merely the reporting.
/// - **The loop only.** Not `grove retire`, which launches a one-off session
///   with no signal file: every turn end there would look unsignalled and
///   report `blocked` with no driver to correct it.
pub(crate) fn append_turn_hooks(cmd: &mut Command, harness: &Harness, grove_llm: &Path) {
    if harness.name != "claude" || !crate::herdr::in_pane() {
        return;
    }
    cmd.arg("--settings").arg(turn_hook_settings(grove_llm));
}

/// One hook entry's `command`: the reporting verb, with the binary path shell-
/// quoted (claude runs `command` through a shell) before the caller
/// JSON-escapes the lot.
fn turn_hook_command(grove_llm: &Path, boundary: &str) -> String {
    // Single-quote and escape any embedded quote the POSIX way (`'\''`): the
    // path is whatever the filesystem holds, and a developer with a space in
    // their home directory must not get a hook that silently fails every turn.
    let quoted = format!("'{}'", grove_llm.to_string_lossy().replace('\'', r"'\''"));
    format!("{quoted} report-turn {boundary}")
}

/// Seconds a turn hook may take before claude abandons it. Belt and braces:
/// [`crate::herdr`] already hard-bounds a report at 500ms whatever the socket
/// does, so this only ever fires if the *binary* wedges. Well under claude's own
/// defaults (600s for `Stop`, 30s for `UserPromptSubmit`), because a hook that
/// stalls a turn is worse than a hook that reports nothing.
const HOOK_TIMEOUT_SECS: u32 = 5;

/// The notification types grove treats as "a human is needed, mid-turn"
/// (herdr-mid-turn-blockers). Matched as claude's `Notification` matcher, which
/// filters on the payload's `notification_type`.
///
/// Not a hand-picked list of things that sounded human-shaped: these are exactly
/// the three sites claude raises from its **idle-notify** path, which fires a
/// notification only once the human has gone six seconds without touching the
/// dialog. That is already grove's own definition of unattended, so the
/// selection rule is a property of claude's code rather than a guess — and it is
/// why `idle_prompt` is absent (a different site, and one that only fires with
/// no request in flight, i.e. after `Stop` has already reported `blocked`) as
/// are the purely informational types (`auth_success`, `agent_completed`).
///
/// A matcher naming a type this claude has never heard of is inert, not an
/// error — unlike an unknown *event* name — so listing all three costs nothing
/// on an older claude.
const HUMAN_NEEDED_NOTIFICATIONS: &str =
    "permission_prompt|elicitation_dialog|elicitation_url_dialog";

/// The `--settings` payload: the two turn boundaries plus the mid-turn pair,
/// all wired to the reporting verb.
///
/// **The boundaries.** `UserPromptSubmit` is what un-blocks the pane once a
/// human answers — without it the pane would stay `blocked` for the rest of a
/// session in which the agent asked anything. `Stop` is the boundary itself.
///
/// **The mid-turn pair** (herdr-mid-turn-blockers) closes the gap *inside* a
/// turn, where a permission prompt stalls an unattended loop exactly as badly as
/// a question. It is a pair because `blocked` there needs a paired restore:
/// granting the permission fires no event of its own, and the next thing claude
/// runs a hook for is the tool call itself. Hence `Notification` ⇒ `blocked` and
/// `PostToolUse` ⇒ `working`.
///
/// `PostToolUse` fires per tool call, which is the one deliberately chatty thing
/// here — a ~3ms spawn and one socket line each. `PostToolBatch` would fire once
/// per model round trip instead *and* would close the parallel-batch race below,
/// but it is a much newer event name than the rest of this block, and an older
/// claude would warn about it on every launch; revisit when it can be assumed.
///
/// None of the four prints to stdout, which matters for `UserPromptSubmit`
/// specifically: its stdout is injected into the conversation as context.
fn turn_hook_settings(grove_llm: &Path) -> String {
    let entry = |event: &str, matcher: Option<&str>, boundary: &str| {
        let matcher = matcher
            .map(|m| format!(r#""matcher":"{m}","#))
            .unwrap_or_default();
        format!(
            r#""{event}":[{{{matcher}"hooks":[{{"type":"command","command":"{}","timeout":{HOOK_TIMEOUT_SECS}}}]}}]"#,
            crate::json::escape(&turn_hook_command(grove_llm, boundary))
        )
    };
    format!(
        r#"{{"hooks":{{{},{},{},{}}}}}"#,
        entry("UserPromptSubmit", None, "start"),
        entry("Stop", None, "end"),
        entry("Notification", Some(HUMAN_NEEDED_NOTIFICATIONS), "waiting"),
        entry("PostToolUse", None, "tool")
    )
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
    scrub_loop_control_env(&mut cmd);
    // herdr-pane-misdetection: a `grove retire` pane is mis-detected exactly as a
    // `grove do` pane is, so it carries the hint too — see `set_herdr_agent_hint`
    // for why this one is *not* driver-only the way `append_turn_hooks` is.
    set_herdr_agent_hint(&mut cmd, harness);

    let status = cmd
        .status()
        .with_context(|| format!("execing {}", harness.exec_bin))?;
    if !status.success() {
        anyhow::bail!("{} exited non-zero", harness.exec_bin);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // The wire shape of the injected settings, pinned byte for byte. This is a
    // payload another program parses, and the only way to see it in production
    // is `claude --debug`, so it is asserted rather than trusted — exactly as
    // `herdr::request_line` is.
    #[test]
    fn the_injected_settings_wire_both_turn_boundaries_to_the_reporting_verb() {
        assert_eq!(
            turn_hook_settings(Path::new("/opt/homebrew/bin/grove-llm")),
            r#"{"hooks":{"UserPromptSubmit":[{"hooks":[{"type":"command","command":"'/opt/homebrew/bin/grove-llm' report-turn start","timeout":5}]}],"Stop":[{"hooks":[{"type":"command","command":"'/opt/homebrew/bin/grove-llm' report-turn end","timeout":5}]}],"Notification":[{"matcher":"permission_prompt|elicitation_dialog|elicitation_url_dialog","hooks":[{"type":"command","command":"'/opt/homebrew/bin/grove-llm' report-turn waiting","timeout":5}]}],"PostToolUse":[{"hooks":[{"type":"command","command":"'/opt/homebrew/bin/grove-llm' report-turn tool","timeout":5}]}]}}"#
        );
    }

    // The matcher is the difference between reporting `blocked` on a permission
    // prompt and reporting it on `auth_success`, so its exact shape is pinned
    // rather than left to the alternation constant. claude compares a matcher
    // made only of `[A-Za-z0-9_|]` as an **exact-string alternation**, not a
    // substring regex — which is what stops `permission_prompt` from also
    // firing on herdr-irrelevant types like `worker_permission_prompt`.
    #[test]
    fn only_the_notification_hook_carries_a_matcher() {
        let settings = turn_hook_settings(Path::new("/g"));
        assert_eq!(
            settings.matches(r#""matcher""#).count(),
            1,
            "every other event fires unconditionally: {settings}"
        );
        assert!(
            HUMAN_NEEDED_NOTIFICATIONS
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '|'),
            "a matcher outside [A-Za-z0-9_|] falls out of claude's exact-match \
             path and is compiled as a regex, where `permission_prompt` would \
             match `worker_permission_prompt` too"
        );
    }

    /// The value a `Command` will pass down as `key`, or `None` if it sets no
    /// such variable. `get_envs` reports only what the builder *changed*, which
    /// is exactly the payload under test — the inherited environment is not it.
    fn env_delta(cmd: &Command, key: &str) -> Option<String> {
        let key = std::ffi::OsStr::new(key);
        cmd.get_envs()
            .find(|(k, _)| *k == key)
            .and_then(|(_, value)| value)
            .map(|value| value.to_string_lossy().into_owned())
    }

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
        scrub_loop_control_env(&mut cmd);
        for name in LOOP_CONTROL_ENV {
            assert!(
                env_is_scrubbed(&cmd, name),
                "{name} must be removed, not merely left unset — an environment \
                 is inherited, not addressed"
            );
        }
    }

    // The hint is a payload another program reads out of the *kernel's* copy of
    // this environment, so like the injected `--settings` it is asserted rather
    // than trusted — and asserted across the whole registry, because the value
    // being `harness.name` verbatim is the claim (grove's names are already
    // herdr's canonical labels, so there is no translation table to get wrong).
    #[test]
    fn every_harness_launch_is_hinted_with_its_own_name() {
        for harness in crate::harness::HARNESSES {
            let mut cmd = Command::new("true");
            set_herdr_agent_hint(&mut cmd, harness);
            assert_eq!(
                env_delta(&cmd, "HERDR_AGENT").as_deref(),
                Some(harness.name),
                "herdr detects the harness by this name, not by grove's"
            );
        }
    }

    // The one asymmetry with `append_turn_hooks`, pinned so it is not quietly
    // "fixed" into symmetry later: the hint is set whether or not grove is under
    // herdr. It changes no argv and spawns nothing, nothing but herdr reads it,
    // and `in_pane`'s three variables are not what the detection it feeds depends
    // on — so gating it could only lose the fix, never save anything.
    #[test]
    fn the_hint_is_not_gated_on_running_under_herdr() {
        let claude = crate::harness::by_name("claude").unwrap();
        let mut cmd = Command::new("true");
        set_herdr_agent_hint(&mut cmd, claude);

        // Whatever this test process's own herdr environment happens to be — the
        // suite may well be running inside a herdr pane — the delta is the same.
        assert_eq!(env_delta(&cmd, "HERDR_AGENT").as_deref(), Some("claude"));
        assert!(
            cmd.get_args().next().is_none(),
            "the hint must not add an argument: `absent herdr, the launch argv is \
             byte-identical` is what gates the turn hooks, and it stays true here"
        );
    }

    // The path comes from the filesystem, not from grove, so it goes through
    // *two* layers that can be broken by one character: a shell (claude runs
    // `command` through one) and JSON. A developer with a space in their home
    // directory would otherwise get a hook that silently fails on every turn.
    #[test]
    fn a_grove_llm_path_is_shell_quoted_and_json_escaped_rather_than_trusted() {
        let settings = turn_hook_settings(Path::new("/tmp/a b/gro've-llm"));
        assert!(
            settings.contains(r#""command":"'/tmp/a b/gro'\\''ve-llm' report-turn end""#),
            "{settings}"
        );
    }
}

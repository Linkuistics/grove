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
/// Grants are additive so the default writable roots stay intact, and the
/// flags are harmless when the sandbox is off. No other harness is touched.
/// The values are dynamic — derived from the worktree per launch — which is
/// why this lives here at the assembly sites rather than in `harness.rs`'s
/// static flag templates.
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

    let mut cmd = Command::new(harness.exec_bin);
    cmd.current_dir(worktree);
    if !harness.name_args.is_empty() {
        cmd.args(harness.name_args).arg(&session_name);
    }
    append_codex_vcs_store_grant(&mut cmd, harness, worktree)?;
    cmd.arg(prompt);
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

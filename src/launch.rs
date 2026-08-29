use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Drive the config-defined lifecycle from the current working tree. This is
/// the sole path reached by the human-facing bare command: provision installed
/// harnesses, acquire the workspace lease, and run one configured foreground
/// session per selected task until the agent stops signalling.
///
/// Nothing here inspects the working tree for a harness, and nothing chooses a
/// binary: the configured argv is the whole of launch policy.
pub fn bare_grove() -> Result<()> {
    crate::provision::provision_installed()?;

    let cwd = std::env::current_dir().context("getting cwd")?;
    let driver_lease = crate::driver_lease::DriverLease::acquire(&cwd)?;
    let worktree = driver_lease.worktree_root().to_path_buf();
    let repository = driver_lease.main_repo().to_path_buf();
    let name = worktree_name(&worktree);

    crate::loop_driver::run_configured(&repository, &worktree, &name, driver_lease)
}

/// The grove name is the worktree directory's basename (user-owned-worktrees).
fn worktree_name(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// The loop driver's **launch-scoped environment** (self-driving-loop) — the
/// variables a descendant could act on, and the exact set
/// [`scrub_loop_control_env`] removes.
///
/// `GROVE_SIGNAL_FILE` is the driver's kill channel: it watches that path while
/// its harness child runs and applies grace → SIGTERM → kill-grace → SIGKILL the
/// moment the file *appears*. Whoever holds the variable can therefore end the
/// session, and the environment is inherited by every descendant — so the
/// authority is ambient unless each spawn scopes it deliberately.
/// `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` are the retired pre-watcher handles
/// (driver-side-kill), kept here because a stale, unrelated PID leaking into a
/// nested grove is the same class of mistake one notch quieter — the value is
/// something a reader could still *act on*. That is the bar for membership, and
/// it is why the removed session-target metadata is no longer listed: nothing
/// reads it, so leaking it grants nothing.
const LOOP_CONTROL_ENV: [&str; 3] = ["GROVE_SIGNAL_FILE", "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID"];

/// **Any spawn that is not the configured session itself must scrub the loop's
/// launch-scoped environment** (guard-loop-signal-k37).
///
/// Authority to end a `grove do` session is granted by an environment variable,
/// and an environment is inherited, not addressed: a spawn that merely declines
/// to *set* `GROVE_SIGNAL_FILE` still hands its child whatever the driver's own
/// environment carried. Scrubbing is therefore the default and granting is the
/// exception — [`crate::loop_driver`]'s session spawn calls this too, and then
/// sets the one path it owns.
///
/// The failure this closes was not hypothetical. This repo is a meta-grove, so
/// its own suite runs as a *descendant* of a live session; a since-removed
/// pre-flight spawned a harness binary without scrubbing, the suite's fake
/// commands write `"$GROVE_SIGNAL_FILE"` unconditionally, and `cargo test`
/// killed the terminal it was typed into. Removing that one site does not
/// retire the rule: every spawn grove makes inherits the same ambient
/// authority, and the rule is what keeps them from carrying it.
///
/// **Grove's own spawns are now the session and nothing else.** The one other
/// family — the VCS probes and the teardown commit — went to
/// `jj-workspace`, which scrubs the *repository selectors* itself because
/// choosing the right repository is its guarantee to make. It deliberately does
/// not scrub this list: it has no consumer to speak for, and `jj` reads no
/// `GROVE_*` variable, so nothing it spawns can act on one.
///
/// Deliberately one helper rather than an `env_remove` per site: the list is the
/// interesting part, and a second site open-coding it is how the first one came
/// to be missed.
pub(crate) fn scrub_loop_control_env(cmd: &mut Command) {
    for name in LOOP_CONTROL_ENV {
        cmd.env_remove(name);
    }
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

    // The rule, pinned on the shape that broke it: a spawn that is not the
    // configured session must hand down no authority to end one. Asserted as a
    // *removal*, not as "we didn't set it" — the bug this closes was precisely
    // that the site set nothing and the child inherited the live path anyway.
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

    // The complement, and the reason this list is *only* the control channel:
    // repository selection is the VCS seam's guarantee to make, and a
    // configured command that wants a repository chosen for it must still
    // receive one. Scrubbing it here would take that from the configuration
    // owner in order to protect a spawn grove no longer makes.
    #[test]
    fn a_scrubbed_spawn_leaves_configured_command_policy_alone() {
        let mut cmd = Command::new("true");
        cmd.env("GIT_DIR", "chosen-by-the-configuration");

        scrub_loop_control_env(&mut cmd);

        assert!(
            !env_is_scrubbed(&cmd, "GIT_DIR"),
            "GIT_DIR is configured-command policy and must remain inherited"
        );
    }
}

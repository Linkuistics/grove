use std::path::PathBuf;

use clap::{Parser, Subcommand};
use grove_loop::{DriverLease, LoopOutcome, TemplateSource, Workspace};

/// Bare `grove` drives the lifecycle; `view` observes a path without launching.
/// Launch policy stays in configuration rather than command-line selectors.
#[derive(Parser)]
#[command(
    name = "grove",
    // **The workspace's version, read through the loop.** One workspace, one
    // release version: every crate an operator installs takes
    // `version.workspace = true`, and both binaries read the same constant so
    // `grove --version` and `grove-llm --version` cannot skew — which is
    // exactly what an operator reaches for them to diagnose.
    version = grove_loop::VERSION,
    about = "Grove: hierarchical workstream tool for AI agents",
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Browse a .grove task tree read-only with manual refresh.
    #[command(
        long_about = "Browse WORKTREE/.grove read-only with manual refresh. There is no upward search: from a subdirectory, view observes that subdirectory's .grove. Requires an interactive terminal; no jj workspace or launch configuration is needed.",
        after_help = "Examples:\n  grove view\n  grove view /path/to/another/worktree"
    )]
    View {
        /// Directory containing .grove (defaults to the current directory).
        worktree: Option<PathBuf>,
    },
}

/// Dispatch the viewer, or resolve, lease and run the lifecycle.
///
/// The workspace is resolved **here**, once, and handed to both the lease and
/// the loop. That is the shape `loop-crate-driver-k22` gave the seam: the lease
/// used to resolve a path itself and hold the answer, so a caller that also
/// needed a workspace had two derivations of one fact and no way to see that
/// they agreed (`docs/adr/one-live-driver-per-working-tree.md`).
///
/// **A driver that was killed does not exit 0.** The loop returns *why* it
/// stopped, and one of the reasons is that this process was sent SIGTERM or
/// SIGHUP mid-grove. Every other reason is an outcome the loop was designed to
/// reach and exits cleanly; that one is the loop being taken away, and the only
/// way to say so through a wait status is to die of the same signal after the
/// cleanup — the lease is dropped by the `run` above, and the session was
/// already reaped by the runner. Whoever started `grove` — a systemd unit, a
/// `timeout(1)`, a shell `wait` — then reads `128 + N` instead of success.
///
/// # Errors
///
/// A working tree that is not a jj workspace, a lease another driver holds, or
/// anything the loop refuses, or a viewer terminal setup/input/draw failure.
pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir()?;
    if let Some(Command::View { worktree }) = cli.command {
        return grove_tui::run(&worktree.unwrap_or(cwd));
    }
    let workspace = Workspace::resolve(&cwd)?;
    let lease = DriverLease::acquire(&workspace)?;
    let templates = TemplateSource::from_env()?;
    match grove_loop::run(&workspace, lease, &templates)? {
        LoopOutcome::Finished | LoopOutcome::Stopped => Ok(()),
        LoopOutcome::Interrupted(signal) => grove_loop::reraise(signal),
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    /// Collect every `<command path> :: <thing>` in `cmd`'s subtree that appears
    /// in a help listing with no description behind it.
    ///
    /// **This walk exists twice**, here and in
    /// `crates/grove-llm/tests/help_surfaces.rs`, and that is the cost of the two
    /// binaries being two packages: a clap model is reachable only from the
    /// package that declares it, and this one is a binary target with no library
    /// to import from an integration test. The alternative was a shared test
    /// crate for thirty lines, or a `[lib]` on this package that exists only so a
    /// test can reach it — which would give the binary a library to reach into,
    /// and that is the property `docs/specs/module-decomposition.md`'s decision 1
    /// made this a crate to keep.
    ///
    /// A description counts only if it is non-empty after trimming: clap treats
    /// `#[arg(help = "")]` as present, and an empty string renders exactly like
    /// the missing doc comment this check exists to reject.
    fn undescribed(cmd: &clap::Command, path: &str, out: &mut Vec<String>) {
        for arg in cmd.get_arguments() {
            let described = [arg.get_help(), arg.get_long_help()]
                .into_iter()
                .flatten()
                .any(|help| !help.to_string().trim().is_empty());
            if !described {
                out.push(format!("{path} :: argument `{}`", arg.get_id()));
            }
        }
        for sub in cmd.get_subcommands() {
            let described = [sub.get_about(), sub.get_long_about()]
                .into_iter()
                .flatten()
                .any(|about| !about.to_string().trim().is_empty());
            if !described {
                out.push(format!("{path} :: subcommand `{}`", sub.get_name()));
            }
            undescribed(sub, &format!("{path} {}", sub.get_name()), out);
        }
    }

    /// `grove retire --no-launch` shipped with **no doc comment at all** and
    /// rendered as a padded blank row beside two described options
    /// (retire-no-launch-help-k21). Asserted against clap's own model rather than
    /// the rendered text, for the reasons
    /// `crates/grove-llm/tests/help_surfaces.rs` sets out at length.
    #[test]
    fn the_human_facing_binary_describes_every_option_it_lists() {
        let mut out = Vec::new();
        undescribed(&Cli::command(), "grove", &mut out);
        assert!(
            out.is_empty(),
            "these render as blank rows in a generated help surface:\n  {}",
            out.join("\n  ")
        );
    }

    /// Stated as a closure property rather than as a list of rejected verbs: the
    /// bare lifecycle has no launch-policy selectors. The only subcommand is
    /// `view`, which observes a path. A new command or lifecycle argument fails
    /// this closed-set assertion without being named in a rejection list.
    #[test]
    fn the_human_command_surface_has_nothing_left_to_select() {
        let command = Cli::command();
        let subcommands: Vec<&str> = command.get_subcommands().map(|s| s.get_name()).collect();
        assert!(
            subcommands == ["view"],
            "only view complements the bare lifecycle: {subcommands:?}"
        );
        let arguments: Vec<String> = command
            .get_arguments()
            .map(|argument| argument.get_id().to_string())
            .filter(|id| id != "help" && id != "version")
            .collect();
        assert!(
            arguments.is_empty(),
            "launch policy has one home and it is not the command line; `grove` \
             accepts: {arguments:?}"
        );
    }
}

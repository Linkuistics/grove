use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use grove_loop::{DriverLease, LoopOutcome, TemplateSource, Workspace};

/// Bare `grove` drives the lifecycle; `run` launches a standalone invocation.
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
    /// Run one configured task in a confined temporary directory, without a grove.
    #[command(
        after_help = "Examples:\n  grove run release-notes --prompt-file prompt.md --input changes.txt --output notes.md\n  grove run summarise 'Summarise input.txt into summary.md' --input input.txt --output summary.md --ui inline\n\nPersonal configuration only; no jj or project discovery. Requires OS confinement and a noninteractive harness command. Exit codes: 0 completed and outputs published, 1 failure/cancellation, 2 invalid usage. Existing output files are never overwritten. Runtime credentials need explicit --runtime-read grants."
    )]
    Run(crate::standalone::Args),
    /// Display a standalone transcript in a supervisor-owned pane.
    #[command(hide = true)]
    RunLog {
        /// Transcript file created by the invocation supervisor.
        log: PathBuf,
        /// Status file written when supervision finishes.
        status: PathBuf,
    },
    /// Inspect launch configuration or install inactive examples.
    // clap 4.6.1: variant-level subcommand nests the enum's commands.
    // https://docs.rs/clap/4.6.1/clap/_derive/index.html#command-attributes
    #[command(
        subcommand,
        after_help = "Examples:\n  grove config show\n  grove config show --kind impl\n  grove config show --json\n  grove config examples\n\nExit codes: 0 success, 1 configuration/source/installation failure, 2 invalid usage."
    )]
    Config(ConfigCommand),
    /// Browse a .grove task tree read-only with automatic refresh.
    #[command(
        long_about = "Browse WORKTREE/.grove read-only with automatic refresh. There is no upward search: from a subdirectory, view observes that subdirectory's .grove. Requires an interactive terminal; no jj workspace or launch configuration is needed.",
        after_help = "Examples:\n  grove view\n  grove view /path/to/another/worktree"
    )]
    View {
        /// Directory containing .grove (defaults to the current directory).
        worktree: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ConfigCommand {
    /// Install inactive example files beside the personal configuration.
    #[command(
        long_about = "Install six .example.kdl files and CONFIGURATION.examples.md under ~/.config/grove/. Works outside a workspace without loading active policy. All destinations are checked first: matching regular files stay untouched; differing, unreadable or non-regular entries are conflicts. Missing files are created exclusively. Later failures may leave created or partial files, which are reported. Never overwrites config.kdl or edits a workspace delta or ignore rule.",
        after_help = "Examples:\n  grove config examples\n  grove config examples --help\n\nExit codes: 0 whole set present, 1 conflict or I/O failure, 2 invalid usage.\nSuccess paths go to stdout; conflicts and partial-failure paths go to stderr. No force or destination option. Inspect conflicting paths and move them aside yourself before retrying. See also: grove config show --help."
    )]
    Examples,
    /// Show sources, selected profiles, commands and override provenance.
    #[command(
        long_about = "Inspect the workspace's configuration read-only, using the same complete validation and admission as launch. Requires a jj workspace but no task tree or driver lease. Runtime slots remain placeholders; no executable is probed or launched. jj may snapshot metadata when checking local configuration trackedness.",
        after_help = "Examples:\n  grove config show\n  grove config show --kind impl\n  grove config show --json\n\nExit codes: 0 valid inspection, 1 source/configuration/resolution failure, 2 invalid usage.\nReports go to stdout; errors go to stderr. --json emits one schema-version-1 object, including usage diagnostics on stderr. A report describes one load; later launches reload configuration."
    )]
    Show {
        /// Show one kind after validating the entire active configuration.
        #[arg(long, value_name = "KIND")]
        kind: Option<String>,
        /// Emit schema-version-1 JSON; failures emit JSON diagnostics on stderr.
        #[arg(long)]
        json: bool,
    },
}

/// Own process reporting, including usage failures before a command exists.
pub fn run() -> ExitCode {
    let args: Vec<_> = std::env::args_os().collect();
    let json = args
        .iter()
        .skip(1)
        .take_while(|arg| *arg != "--")
        .any(|arg| arg == "--json" || arg.as_encoded_bytes().starts_with(b"--json="));
    // try_parse_from preserves native arguments; use_stderr distinguishes help
    // from refusal. https://docs.rs/clap/4.6.1/clap/error/struct.Error.html#method.use_stderr
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(error) if json && error.use_stderr() => {
            eprintln!(
                "{}",
                crate::config_json::failure(
                    "usage",
                    &error.to_string(),
                    "Run grove config show --help for supported options."
                )
            );
            return ExitCode::from(2);
        }
        Err(error) => error.exit(),
    };
    match execute(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if json {
                eprintln!("{}", crate::config_json::error(&error));
            } else {
                eprintln!("Error: {error:?}");
            }
            ExitCode::FAILURE
        }
    }
}

/// Dispatch inspection, viewing or example delivery before the lifecycle.
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
fn execute(cli: Cli) -> anyhow::Result<()> {
    if let Some(Command::Run(args)) = cli.command {
        return crate::standalone::run(args);
    }
    if let Some(Command::RunLog { log, status }) = cli.command {
        return crate::run_display::watch(&log, &status);
    }
    if let Some(Command::Config(ConfigCommand::Examples)) = cli.command {
        return crate::examples::run();
    }
    let cwd = std::env::current_dir()?;
    if let Some(Command::View { worktree }) = cli.command {
        return grove_tui::run(&worktree.unwrap_or(cwd));
    }
    if let Some(Command::Config(ConfigCommand::Show { kind, json })) = cli.command {
        return crate::config::show(&cwd, kind.as_deref(), json);
    }
    let workspace = Workspace::resolve(&cwd)?;
    let lease = DriverLease::acquire(&workspace)?;
    crate::provision::ensure_codex_skills()?;
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
    /// bare lifecycle has no launch-policy selectors. `view` observes a path;
    /// `config` inspects policy or installs inactive samples. A new command or argument fails
    /// this closed-set assertion without being named in a rejection list.
    #[test]
    fn the_human_command_surface_has_nothing_left_to_select() {
        let command = Cli::command();
        let subcommands: Vec<&str> = command.get_subcommands().map(|s| s.get_name()).collect();
        assert!(
            subcommands == ["run", "run-log", "config", "view"],
            "unexpected command beside the bare lifecycle: {subcommands:?}"
        );
        let config = command.find_subcommand("config").unwrap();
        let config_commands: Vec<_> = config.get_subcommands().map(|s| s.get_name()).collect();
        assert_eq!(config_commands, ["examples", "show"]);
        assert_eq!(
            config
                .find_subcommand("examples")
                .unwrap()
                .get_arguments()
                .count(),
            0
        );
        let show = config.find_subcommand("show").unwrap();
        let options: Vec<_> = show
            .get_arguments()
            .map(|arg| arg.get_id().as_str())
            .collect();
        assert_eq!(options, ["kind", "json"]);
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

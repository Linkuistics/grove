# The surface
<!-- book-page id="the-surface" slice="no-arguments" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Three steps](03-three-steps.md)

<a id="no-arguments"></a>
## Lifecycle and observation

The human binary has lifecycle, tree-viewing, configuration-inspection and example-delivery paths. Bare `grove` starts, resumes or finishes
the lifecycle in the enclosing jj workspace. `grove view [WORKTREE]` opens an
inert, read-only browser at one directory's `.grove`. Its path selects what to
observe; it does not select a session, a kind or launch policy. `grove-llm`
remains the separate flat command surface a running session uses.

The parser turns the shell's argument vector into `Cli`. An absent command
means lifecycle dispatch; a `View` variant carries an optional path and a
`Config` variant carries inspection or inactive example delivery. Help and
version exit during parsing, before application dispatch.

<a id="the-grammar"></a>
## The grammar, in five fragments

The grammar owns the imports, its stated contract, clap metadata and the
command declarations. Their source-order concatenation is independent of the
reader order below. The `Command` enum belongs here alongside `Cli`, because
its job is parsing the requested operation, without selecting launch policy.

<!-- fragment «surface-grammar» owner="no-arguments" source="crates/grove/src/cli.rs" lines="1-106" parent="source-command-surface" -->
<!-- insert «surface-imports» -->
<!-- insert «surface-doc-comment» -->
<!-- insert «surface-clap-attributes» -->
<!-- insert «surface-empty-struct» -->
<!-- insert «surface-config-command» -->
<!-- insert «surface-process-reporting» -->
<!-- /fragment -->

<a id="the-imports"></a>
## The names used by the parser and dispatch

`PathBuf` retains an operating-system path without imposing UTF-8.
`Parser` derives the outer parser; `Subcommand` derives the `Command` grammar.
The loop imports are used by the next chapter's lifecycle path:

| Type | Meaning held until dispatch |
|---|---|
| `Workspace` | A resolved jj working tree, shared by the lease and loop |
| `DriverLease` | The one-driver claim held while the loop runs |
| `TemplateSource` | The location from which the loop reads launch policy |
| `LoopOutcome` | Why the loop stopped, including interruption by a signal |

The viewer and example installer return before any of those four is constructed or used.

<!-- fragment «surface-imports» owner="no-arguments" source="crates/grove/src/cli.rs" lines="1-5" parent="surface-grammar" -->
````rust
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use grove_loop::{DriverLease, LoopOutcome, TemplateSource, Workspace};
````
<!-- /fragment -->

<a id="nothing-to-select"></a>
## Launch policy stays on disk

The doc comment distinguishes an observation path from a lifecycle selector.
The tree still decides which leaf runs, and `~/.config/grove/config.kdl` still
decides how to launch its kind. Bare `grove` takes neither as an argument.
Adding the browser does not give the launcher a second source of policy.

<!-- fragment «surface-doc-comment» owner="no-arguments" source="crates/grove/src/cli.rs" lines="6-8" parent="surface-grammar" -->
````rust

/// Bare `grove` drives the lifecycle; `view` and `config` never launch sessions.
/// Launch policy stays in configuration rather than command-line selectors.
````
<!-- /fragment -->

<a id="the-version"></a>
## One version, read through the loop

The clap attributes retain the product name, description and shared release
version. Both binaries read `grove_loop::VERSION`; their manifests and the
libraries shipped with them inherit the workspace version. `book-validation`
is an authoring tool with its own version and is outside that release set.

`disable_help_subcommand` keeps the subcommand set exactly `{config, view}`.
The normal `--help` option still describes every command and argument.

<!-- fragment «surface-clap-attributes» owner="no-arguments" source="crates/grove/src/cli.rs" lines="9-20" parent="surface-grammar" -->
````rust
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
````
<!-- /fragment -->

<a id="the-struct"></a>
## An optional observation command

`Cli.command` is `None` for bare invocation. `Command::View` contains an
optional `PathBuf`, defaulted by dispatch to the current directory. The view
help states no upward search, explains the subdirectory case, and gives
examples for both current and explicit worktrees. Parsing accepts a path even
when that directory is absent; absence is a visible state of the viewer.

<!-- fragment «surface-empty-struct» owner="no-arguments" source="crates/grove/src/cli.rs" lines="21-45" parent="surface-grammar" -->
````rust
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
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
````
<!-- /fragment -->

<a id="config-grammar"></a>
## Inspecting configured policy and installing examples

`ConfigCommand::Show` holds an optional kind filter and a JSON output flag. It changes which
validated commands are displayed, not which profiles resolve or what launches.
Clap requires a child of `config`; its nested help provides examples and exit
codes. SessionConfig owns validation; the parser cannot establish that a kind
is admitted. The JSON flag selects the wire projection of that same validated result.

<!-- fragment «surface-config-command» owner="no-arguments" source="crates/grove/src/cli.rs" lines="46-68" parent="surface-grammar" -->
````rust

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
````
<!-- /fragment -->

<a id="worked-argv"></a>
## Worked example: dispatch before workspace resolution

| Argument vector | Parser result and next effect |
|---|---|
| `grove` | `command: None`; resolve workspace, acquire lease, run lifecycle |
| `grove view` | `View { worktree: None }`; observe current directory's `.grove` |
| `grove view /tmp/tasks` | `View` with `/tmp/tasks`; observe `/tmp/tasks/.grove` |
| `grove config show --kind impl` | Validate all active policy, then require and display impl |
| `grove view --help` | Print observation help and exit before terminal setup |
| `grove --version` | Print the shared release version and exit |
| `grove --harness claude` | Clap refuses the unknown lifecycle flag |

From `/work/atlas/src`, bare `grove` can resolve `/work/atlas` as its enclosing
workspace. `grove view` instead observes `/work/atlas/src/.grove` and never
walks upward. In a temporary directory with no jj metadata, view still runs
when attached to an interactive terminal. With piped input or output it returns
an actionable error before changing terminal modes. The integration tests in
`crates/grove/tests/view_command.rs` check help and this early refusal.

<a id="the-agent-surface"></a>
## The agent surface remains flat

The twelve tree/session verbs belong to `grove-llm`; their parser and lifecycle
semantics are unchanged by `view`. The human binary delegates observation to
`grove-tui` and lifecycle execution to `grove-loop`. [Three steps](03-three-steps.md)
shows the branch that separates these paths.



The `Examples` variant takes no selector or destination. Its help describes the
fixed personal-directory package, preflight conflicts, exclusive creation and
partial-failure reports. It shares the outer config group but never calls the
configuration reader.

<a id="process-reporting"></a>
## Reporting before a command exists

`run` owns parsing and process reporting. It scans native arguments for the JSON
request before asking clap to parse, so even an earlier unknown option can produce
structured diagnostics. The `--` terminator ends that scan. Malformed equals-form
requests still ask for JSON errors, while clap refuses their value. Help and
version requests keep clap's successful human output.

After parsing, `execute` returns any application failure to this boundary. JSON
configuration errors preserve structured records; other failures get the same
record shape. Human errors retain their context chain. Returning `ExitCode` lets
`main` finish without Rust adding a second error message.

<!-- fragment «surface-process-reporting» owner="no-arguments" source="crates/grove/src/cli.rs" lines="69-106" parent="surface-grammar" -->
````rust

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
````
<!-- /fragment -->

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Three steps](03-three-steps.md)

# The surface
<!-- book-page id="the-surface" slice="no-arguments" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Lifecycle startup](03-three-steps.md)

<a id="no-arguments"></a>
## Lifecycle and observation

The human binary has lifecycle, standalone invocation and tree-viewing
paths. Bare `grove` starts, resumes or finishes
the lifecycle in the enclosing jj workspace. `grove view [WORKTREE]` opens an
inert, read-only browser at one directory's `.grove`. Its path selects what to
observe; it does not select a session, a kind or launch policy. `grove-llm`
remains the separate flat command surface a running session uses.

The parser turns the shell's argument vector into `Cli`. An absent command
means lifecycle dispatch; a `View` variant carries an optional path. Help and
version exit during parsing, before application dispatch. `Run` carries one
kind, prompt and artifact contract; the hidden `RunLog` command tails a log for a
parent-created pane. Neither starts the lifecycle.

<a id="the-grammar"></a>
## The grammar and process reporting

The grammar owns the imports, its stated contract, clap metadata and the
command declarations. Their source-order concatenation is independent of the
reader order below. The `Command` enum belongs here alongside `Cli`, because
its job is parsing the requested operation. Standalone mode names a kind;
the owner's harness-dispatch policy selects the command that implements it.

<!-- fragment «surface-grammar» owner="no-arguments" source="crates/grove/src/cli.rs" lines="1-62" parent="source-command-surface" -->
<!-- insert «surface-imports» -->
<!-- insert «surface-doc-comment» -->
<!-- insert «surface-clap-attributes» -->
<!-- insert «surface-empty-struct» -->
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
| `LoopOutcome` | Why the loop stopped, including interruption by a signal |

The viewer returns before any of those three is constructed or used.

<!-- fragment «surface-imports» owner="no-arguments" source="crates/grove/src/cli.rs" lines="1-5" parent="surface-grammar" -->
````rust
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use grove_loop::{DriverLease, LoopOutcome, Workspace};
````
<!-- /fragment -->

<a id="nothing-to-select"></a>
## Launch policy stays on disk

The doc comment distinguishes an observation path from a lifecycle selector.
The tree still decides which leaf runs, and the owner's `harness-dispatch` policy
decides how to launch its kind. Bare `grove` takes neither as an argument.
Adding the browser does not give the launcher a second source of policy.

<!-- fragment «surface-doc-comment» owner="no-arguments" source="crates/grove/src/cli.rs" lines="6-9" parent="surface-grammar" -->
````rust

/// Bare `grove` drives the lifecycle; `run` launches a standalone invocation.
/// Launch policy stays in the owner's harness-dispatch policy rather than
/// command-line selectors.
````
<!-- /fragment -->

<a id="the-version"></a>
## One version, read through the loop

The clap attributes retain the product name, description and shared release
version. Both binaries read `grove_loop::VERSION`; their manifests and the
libraries shipped with them inherit the workspace version. `book-validation`
is an authoring tool with its own version and is outside that release set.

`disable_help_subcommand` avoids an extra help verb. The subcommand set is
`{run, run-log, view}`; `run-log` is hidden from ordinary help.
The normal `--help` option still describes every command and argument.

<!-- fragment «surface-clap-attributes» owner="no-arguments" source="crates/grove/src/cli.rs" lines="10-21" parent="surface-grammar" -->
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
## Optional commands before lifecycle setup

`Cli.command` is `None` for bare invocation. `Command::View` contains an
optional `PathBuf`, defaulted by dispatch to the current directory. The view
help states no upward search, explains the subdirectory case, and gives
examples for both current and explicit worktrees. Parsing accepts a path even
when that directory is absent; absence is a visible state of the viewer.
`Run` delegates its argument model to the standalone module. `RunLog` carries
the exact log and status paths used by the separate display process.

<!-- fragment «surface-empty-struct» owner="no-arguments" source="crates/grove/src/cli.rs" lines="22-51" parent="surface-grammar" -->
````rust
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run one task kind in a confined temporary directory, without a grove.
    #[command(
        after_help = "Examples:\n  grove run release-notes --prompt-file prompt.md --input changes.txt --output notes.md\n  grove run summarise 'Summarise input.txt into summary.md' --input input.txt --output summary.md --ui inline\n\nThe owner's harness-dispatch policy selects the command for the kind; no jj or project discovery. Requires OS confinement and a noninteractive harness command. Exit codes: 0 completed and outputs published, 1 failure/cancellation, 2 invalid usage. Existing output files are never overwritten. Runtime credentials need explicit --runtime-read grants."
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
    /// Browse a .grove task tree read-only with automatic refresh.
    #[command(
        long_about = "Browse WORKTREE/.grove read-only with automatic refresh. There is no upward search: from a subdirectory, view observes that subdirectory's .grove. Requires an interactive terminal; no jj workspace or harness-dispatch policy is needed.",
        after_help = "Examples:\n  grove view\n  grove view /path/to/another/worktree"
    )]
    View {
        /// Directory containing .grove (defaults to the current directory).
        worktree: Option<PathBuf>,
    },
}
````
<!-- /fragment -->

<a id="worked-argv"></a>
## Worked example: dispatch before workspace resolution

| Argument vector | Parser result and next effect |
|---|---|
| `grove` | `command: None`; resolve workspace, acquire lease, run lifecycle |
| `grove run review "Review input.txt" --input input.txt` | Ask the owner's policy for the command and run it once in confined scratch storage |
| `grove view` | `View { worktree: None }`; observe current directory's `.grove` |
| `grove view /tmp/tasks` | `View` with `/tmp/tasks`; observe `/tmp/tasks/.grove` |
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

The twelve tree/session verbs belong to `grove-llm` and retain their task-tree
epoch checks. A standalone harness acknowledges through `harness-dispatch exit`;
Grove grants it no tree authority. The human binary delegates observation to
`grove-tui` and lifecycle execution to `grove-loop`. [Lifecycle startup](03-three-steps.md)
shows the branch that separates these paths.


<a id="process-reporting"></a>
## Process reporting

`run` owns parsing and process reporting. `Cli::parse` exits by itself on help,
version and a usage refusal, so an unknown lifecycle flag never reaches
dispatch. After parsing, `execute` returns any application failure to this
boundary, which prints the error with its context chain once. Returning
`ExitCode` lets `main` finish without Rust adding a second error message.

<!-- fragment «surface-process-reporting» owner="no-arguments" source="crates/grove/src/cli.rs" lines="52-62" parent="surface-grammar" -->
````rust

/// Own process reporting: clap exits on help, version and usage refusals.
pub fn run() -> ExitCode {
    match execute(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error:?}");
            ExitCode::FAILURE
        }
    }
}
````
<!-- /fragment -->

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Lifecycle startup](03-three-steps.md)

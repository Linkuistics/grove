# The surface
<!-- book-page id="the-surface" slice="no-arguments" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Three steps](03-three-steps.md)

<a id="no-arguments"></a>
## Lifecycle and observation

The human binary has two entry paths. Bare `grove` starts, resumes or finishes
the lifecycle in the enclosing jj workspace. `grove view [WORKTREE]` opens an
inert, read-only browser at one directory's `.grove`. Its path selects what to
observe; it does not select a session, a kind or launch policy. `grove-llm`
remains the separate flat command surface a running session uses.

The parser turns the shell's argument vector into `Cli`. An absent command
means lifecycle dispatch; a `View` variant carries an optional path. Help and
version exit during parsing, before either dispatch path.

<a id="the-grammar"></a>
## The grammar, in four fragments

The grammar owns the imports, its stated contract, clap metadata and the
command declarations. Their source-order concatenation is independent of the
reader order below. The `Command` enum belongs here alongside `Cli`, because
its only job is parsing the observation request.

<!-- fragment «surface-grammar» owner="no-arguments" source="crates/grove/src/cli.rs" lines="1-36" parent="source-command-surface" -->
<!-- insert «surface-imports» -->
<!-- insert «surface-doc-comment» -->
<!-- insert «surface-clap-attributes» -->
<!-- insert «surface-empty-struct» -->
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

The viewer returns before any of those four is constructed or used.

<!-- fragment «surface-imports» owner="no-arguments" source="crates/grove/src/cli.rs" lines="1-4" parent="surface-grammar" -->
````rust
use std::path::PathBuf;

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

<!-- fragment «surface-doc-comment» owner="no-arguments" source="crates/grove/src/cli.rs" lines="5-7" parent="surface-grammar" -->
````rust

/// Bare `grove` drives the lifecycle; `view` observes a path without launching.
/// Launch policy stays in configuration rather than command-line selectors.
````
<!-- /fragment -->

<a id="the-version"></a>
## One version, read through the loop

The clap attributes retain the product name, description and shared release
version. Both binaries read `grove_loop::VERSION`; their manifests and the
libraries shipped with them inherit the workspace version. `book-validation`
is an authoring tool with its own version and is outside that release set.

`disable_help_subcommand` keeps the subcommand set exactly `{view}`.
The normal `--help` option still describes every command and argument.

<!-- fragment «surface-clap-attributes» owner="no-arguments" source="crates/grove/src/cli.rs" lines="8-19" parent="surface-grammar" -->
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

<!-- fragment «surface-empty-struct» owner="no-arguments" source="crates/grove/src/cli.rs" lines="20-36" parent="surface-grammar" -->
````rust
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
````
<!-- /fragment -->

<a id="worked-argv"></a>
## Worked example: dispatch before workspace resolution

| Argument vector | Parser result and next effect |
|---|---|
| `grove` | `command: None`; resolve workspace, acquire lease, run lifecycle |
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

The twelve tree/session verbs belong to `grove-llm`; their parser and lifecycle
semantics are unchanged by `view`. The human binary delegates observation to
`grove-tui` and lifecycle execution to `grove-loop`. [Three steps](03-three-steps.md)
shows the branch that separates these paths.

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Three steps](03-three-steps.md)

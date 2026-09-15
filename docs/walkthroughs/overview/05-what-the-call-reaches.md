# What the call reaches
<!-- book-page id="what-the-call-reaches" slice="assembly" order="5" -->
[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

<a id="assembly"></a>
## Two calls, with separate lifetimes

<!-- rollup «owned-lines-total» -->
The preceding chapters reconstruct the human crate's 224 source lines. This
chapter owns no source: it assembles the two dispatch paths and the guarantees
that keep the command surface small.

Bare `grove` resolves a jj workspace, takes the driver lease and calls
`grove_loop::run`. `grove view [WORKTREE]` returns through `grove_tui::run`
before those steps. An observation path needs no workspace identity, driver
lease or launch configuration. Both paths return errors through the same main;
the viewer restores its terminal before returning an error.

<a id="the-package-map"></a>
## Packages and public boundaries

The workspace contains eight packages. Seven share the product's release
version; `book-validation` is the separately versioned authoring tool.

| Package | Responsibility | Workspace runtime dependencies |
|---|---|---|
| `grove` | Human CLI and dispatch | `grove-loop`, `grove-tui` |
| `grove-tui` | Read-only application, capture, rendering and terminal lifetime | `grove-loop` |
| `grove-loop` | Grove vocabulary, reader, verbs and driver | `ordinal-fs-tree`, `jj-workspace`, `keyed-launch` |
| `grove-llm` | Session CLI | `grove-loop`, `jj-workspace` |
| `ordinal-fs-tree` | Domain-free ordered tree store | none |
| `jj-workspace` | Domain-free workspace and commit seam | none |
| `keyed-launch` | Domain-free command runner | none |
| `book-validation` | Source-fragment and Markdown checks | none |

Ratatui and Crossterm belong to the viewer. Neither the loop library nor the
session binary depends on the terminal UI. Release archives still contain only
`grove` and `grove-llm`: the viewer is linked into the human executable.

<a id="the-seven-names"></a>
## What crosses the binary boundary

| Public item | Role in dispatch |
|---|---|
| `grove_loop::VERSION` | Shared release metadata |
| `grove_loop::Workspace` | Resolve the bare lifecycle's jj working tree |
| `grove_loop::DriverLease` | Hold the one-driver claim |
| `grove_loop::TemplateSource` | Locate launch policy |
| `grove_loop::run` | Execute the lifecycle |
| `grove_loop::LoopOutcome` | Distinguish clean stops and interruption |
| `grove_loop::reraise` | Preserve an interrupted driver's signal exit |
| `grove_tui::run` | Own the interactive observation session |

All are public library items. The package boundary makes accidental access to
private items a compiler error; a source inclusion from outside the package
would be a visible change to that boundary, not something privacy forbids.

<a id="the-modules-behind-the-call"></a>
## Behind each call

The loop walkthrough at `docs/walkthroughs/grove-loop/README.md` owns its task-name grammar,
reader and mutation semantics, lease, epoch, prompt and repeated session launch.
The viewer consumes that typed reader rather than parsing filenames itself.
It copies display rows and selected file bytes while holding a short shared
read guard, then drops it before rendering or waiting for input.

The viewer's `Viewer::new`, `act` and `render` methods are its application seam.
Production keyboard input supplies actions; tests supply the same actions and
render through Ratatui's TestBackend. Selection opens a leaf file or branch
brief. Manual refresh can reset to root; folded branches retain aggregate
counts for all descendants. Missing and failed reads become visible states.

<a id="the-boundary"></a>
## Where the overview stops

This book reconstructs the human crate's manifest and Rust source, including
its parser tests. Viewer and loop internals are outside its corpus. The
[usage guide](../../USAGE.md#usage-viewing-tree) owns the delivered interaction
contract; `docs/ARCHITECTURE.md` names the seams.
Plain text and manual refresh are current behavior. Automatic observation,
Markdown layout and complete terminal fault/signal handling belong to later
increments and are not guarantees of this source.

<a id="the-test-applied-back"></a>
## Apply the thin-entry-point test

| Guarantee | What would break it | Evidence |
|---|---|---|
| Public library access | Naming a private library item | Rust visibility checking |
| No lifecycle selector arguments; exactly `{view}` | A new outer argument or second subcommand | CLI closed-set unit test |
| Every listed item has help | Missing or whitespace-only help | Recursive description test |
| Observation bypasses lifecycle setup | Trying to resolve jj or configure a driver for view | CLI refusal test and non-jj PTY smoke |
| Display actions preserve the filesystem | Any name/content/state-file write | Application before/after fixture comparison |

The help-presence test now checks a real command and argument. Its success
still says nothing about prose accuracy. Likewise, byte-exact book validation
proves reproduction and ownership, not the truth of explanatory claims.

<a id="the-closed-ledgers"></a>
## Source ownership and early uses

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
<!-- rollup «ownership-blocks-not-owned-by» of="compiler-held" -->
There are 5 ownership blocks over 3 source roots; 4 blocks belong to chapters
after Orientation. Each is resolved in the source index.

<!-- rollup «early-use-rows» -->
The 5 early-use rows name loop items before their full explanation in Three
steps. The grammar explains its own path and command types at first use.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
Owned source is 55 + 36 + 50 + 83 = 224 lines across 4 source-owning chapters.
The assembly owns zero lines. The ledgers are maintained with source changes;
production files remain authoritative.

<a id="final-verification"></a>
## Verification

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . --book docs/walkthroughs/overview --final --check all
```

This final check requires complete source coverage, fragment reachability,
byte equality, ownership ledgers and valid Markdown references. Runtime checks
are separate: the human binary's CLI tests, viewer application tests and actual
terminal smoke. The task's verification record states their observed results.

<!-- rollup «source-roots» -->
<!-- rollup «owned-lines-total» -->
<!-- rollup «chapters» -->
The corpus contains 3 roots and 224 lines, explained across 5 chapters and two
lookup pages. No deferred source range belongs in the final book.

[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

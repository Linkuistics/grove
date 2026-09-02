# The surface
<!-- book-page id="the-surface" slice="no-arguments" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Three steps](03-three-steps.md)

<a id="no-arguments"></a>
## Two binaries, one of them empty

The repository ships two command-line binaries, one per audience. `grove` is
the human's: typed bare, it accepts nothing that selects, and its whole grammar
is the nineteen lines this chapter reads. `grove-llm` is the agent's: the binary
a session drives from inside a task, whose grammar is twelve verbs in one flat
list. The split is by who types, not by what is done. A human starts, resumes
and finishes a grove; a session reads and grows the task tree; and neither
binary offers the other audience's operations. The human grammar is empty
because every fact a launcher would ordinarily take from its command line
already has one home on disk. The agent grammar is flat because a session that
has lost its context has to recover the whole surface from a single `--help`.

This page is about the parser, and the parser only. Its input is the argument
vector the shell hands to `grove`; its output is either a `Cli` value that
carries nothing, on which *Three steps* then performs resolve, lease and run, or
an exit before any of those steps. The invariant it establishes is the one the
doc comment at line 4 states: there is nothing left for an argument to select.
The page reads the grammar in the order the file declares it — two imports,
the doc comment, the clap attributes that declare the surface, and the
empty struct they decorate — then runs the carried invocation's argument
vector through it
beside the two arguments that stop before the flow and one that is refused, and
only after that catalogues the agent grammar. One constant is owned on this page
and nowhere else in the book: the version both binaries report.

<a id="the-grammar"></a>
## The grammar, in four fragments

Lines 1 to 19 of `crates/grove/src/cli.rs` are this chapter's block, and the
composite below is the whole of it: the imports, the doc comment, the attribute
block and the struct, each a literal fragment in file order. The boundary at
line 19 is structural — the struct's closing brace ends the grammar, and line
20 is the blank line before the doc comment of the function that uses it —
and the boundary at line 1 has one accepted consequence, which the next
section reads
first.

<!-- fragment «surface-grammar» owner="no-arguments" source="crates/grove/src/cli.rs" lines="1-19" parent="source-command-surface" -->
<!-- insert «surface-imports» -->
<!-- insert «surface-doc-comment» -->
<!-- insert «surface-clap-attributes» -->
<!-- insert «surface-empty-struct» -->
<!-- /fragment -->

<a id="the-imports"></a>
## Two imports, four early uses

The first line brings in `Parser`, the derive macro that turns a struct into a
command-line parser and the only piece of `clap` this file names by path. The
second line names four types from `grove-loop`, and every one of them is used
in `run` at lines 42 to 52 rather than in the grammar. They are read here
because the ownership boundary falls at line 19 and line 2 is above it, not
because this page explains them: *Three steps* owns all four, and the source
index carries a `pending` row for each until that chapter lands. What this page
owes is the minimum a reader needs to hold, and the table states it.

| Type | What it is, in one sentence | Where it is used |
|---|---|---|
| `Workspace` | A resolved jj working tree, produced once in `run` and handed to both the lease and the loop. | line 45, and as the argument at lines 46 and 48 |
| `DriverLease` | The one-driver-per-working-tree claim, taken for the life of the process. | line 46, moved into the loop at line 48 |
| `TemplateSource` | Where launch policy is read from; the loop re-reads it every iteration rather than holding a copy. | line 47, borrowed by the loop at line 48 |
| `LoopOutcome` | Why the loop stopped — the value that decides whether this process exits 0 or dies of a signal. | the `match` at lines 48 to 51 |

The table is the early-use contract for this page: the four statements are
what a reader carries into the worked example below, and *Three steps* replaces
each with the full account. One further fact is the first mechanism at work.
All four names resolve because each is a `pub` re-export in `grove-loop`'s
crate root — three are the loop's own types, and `Workspace` is the VCS seam's
type published through the same root. A fifth
name on this line that the library had not chosen to publish would not compile,
which is the boundary *Orientation* argued and the reason this `use` line is
the complete list of what the grammar's file reaches.

<!-- fragment «surface-imports» owner="no-arguments" source="crates/grove/src/cli.rs" lines="1-2" parent="surface-grammar" -->
````rust
use clap::Parser;
use grove_loop::{DriverLease, LoopOutcome, TemplateSource, Workspace};
````
<!-- /fragment -->

<a id="nothing-to-select"></a>
## Nothing left to select

The doc comment states the surface in one sentence and then gives the reason,
and the reason is the one *Orientation* met in the manifest, restated here at
the grammar that results. The complete human command surface is bare `grove`
plus two arguments the comment calls *clap's own*, `--help` and `--version`.
There are no subcommands and no flags. The actor is the driver — the loop this
binary calls — and it reads two files for the two facts a launcher would
otherwise be told: the task tree for what to do, and the personal configuration
for how to launch it.

| What a launcher usually takes from its arguments | Where `grove` reads it instead | Which page or document describes that source |
|---|---|---|
| Which work to run | The task tree under `.grove/`, walked for its first live leaf on every iteration | Behind the call; named in *What the call reaches* |
| How to run it | `~/.config/grove/config.kdl`, one complete command template per session kind | `docs/CONFIGURATION.md`, which this book does not restate |
| Where to run it | The working directory: the nearest enclosing jj working tree | *Three steps*, at `Workspace::resolve` |

The table is the argument for the empty grammar in three rows: each thing a
command line could select has a source on disk already, and an argument that
selected it would be a second source for a fact that has one. The third row is
the one a reader may not expect. The working directory is an input, and it is
the only one — `grove` typed in one Jujutsu workspace starts or resumes
the grove there and not in a sibling — but it is not an argument, and the
grammar stays
empty because of that distinction rather than in spite of it.

The phrase *clap's own* is precise, and the two are clap's in slightly
different ways. Neither is a field of the struct, and neither is spelled
anywhere in this file: `clap` adds `--help` to every command it builds, and it
adds `--version` to a command that has a version to print — which this one
does because line 16 supplies one. The spellings, the short forms `-h` and
`-V`, the rendering and the early exit are all the parser's; what this file
contributes
is three values the two outputs carry — the name at line 10, the version at
line 16 and the description at line 17. That is also why the closure test in
*Proving a negative* filters the two argument ids `help` and `version` out
before asserting that nothing remains: they are the parser's, and the property
being asserted is about what this crate declared.

<!-- fragment «surface-doc-comment» owner="no-arguments" source="crates/grove/src/cli.rs" lines="3-7" parent="surface-grammar" -->
````rust

/// The complete human command surface: bare `grove`, plus clap's own `--help`
/// and `--version`. There are no subcommands and no flags — the driver reads
/// the task tree for what to do and `~/.config/grove/config.kdl` for how to
/// launch it, so there is nothing left for an argument to select.
````
<!-- /fragment -->

<a id="the-version"></a>
## One version, read through the loop

The attribute block is where the grammar is declared, and it declares three
things. `#[derive(Parser)]` generates the parser for the struct that follows,
together with the `parse` constructor *Three steps* calls at line 43 and the
`command` factory *Proving a negative* inspects. `name = "grove"` is the name
`--version` prints in front of the number, whatever the binary was invoked as:
a copy of the binary under another file name still reports `grove 20.1.0`,
while the usage line in its help text takes the invoked name from the argument
vector, which is `clap`'s rule and not this file's. `about` is the one-line
description that opens `--help`, and its text is the first line of the worked
example below. Between them sits the attribute this page owns:
`version = grove_loop::VERSION`.

The constant is not this crate's. `grove-loop`'s crate root declares
`pub const VERSION: &str = env!("CARGO_PKG_VERSION")` — the loop package's own
version, captured at compile time — and both binaries' clap models read it:
this line, and the matching attribute in `crates/grove-llm/src/cli.rs`. So
`grove --version` and `grove-llm --version` print one number, and the number
is the workspace's release: the loop's manifest takes
`version.workspace = true`,
as this crate's does at line 4 of the manifest *Orientation* read, and the
workspace root's `[workspace.package]` carries the value. At the corpus this
book is frozen against that value is `20.1.0`, which is what both commands print
in the worked example.

The comment says *every member* takes the workspace version, and the manifests
do not bear that out as written. Six of the seven members do — the two
binaries and the four libraries an operator's install is built from — and
the seventh, `book-validation`, is the authoring tool behind this book and
carries a
`0.1.0` of its own; nothing an operator installs reads it. The claim the comment
needs is the narrower one, that every crate on the path from `grove` to
`grove-loop` inherits one version, and that one holds. The comment is part of
the frozen corpus and is reproduced as written.

Two mechanisms make the two numbers equal today, and the comment argues for the
second. Both binaries inherit the workspace version, so two independent
`env!("CARGO_PKG_VERSION")` reads would agree by inheritance. Reading one
constant makes the agreement a fact about a single definition rather than about
two manifests staying in step, and a binary whose manifest stopped inheriting
would still report the release rather than a version of its own. The reason an
operator cares is the comment's last clause: the two commands are what someone
runs to diagnose a skew between the installed binaries, and a report that could
itself skew would answer nothing. This is the one place in the book the version
is explained; *Orientation* named it and *Three steps* does not return to it.

<!-- fragment «surface-clap-attributes» owner="no-arguments" source="crates/grove/src/cli.rs" lines="8-18" parent="surface-grammar" -->
````rust
#[derive(Parser)]
#[command(
    name = "grove",
    // **The workspace's version, read through the loop.** One workspace, one
    // release version: every member takes `version.workspace = true`, and both
    // binaries read the same constant so `grove --version` and
    // `grove-llm --version` cannot skew — which is exactly what an operator
    // reaches for them to diagnose.
    version = grove_loop::VERSION,
    about = "Grove: hierarchical workstream tool for AI agents"
)]
````
<!-- /fragment -->

<a id="the-struct"></a>
## A struct with no fields

The struct the attributes decorate has no fields, and that is the grammar. Under
`derive(Parser)` every field of the struct becomes an argument — a positional,
a flag or a subcommand — so a struct with none declares none, and what the
derive still generates is the machinery around an empty model: a `parse` that
consumes the argument vector and either returns `Cli {}` or exits, and a
`command` that hands the model to whoever asks. `Cli` is the only type the
binary's two source files declare. *Three steps* binds the parsed value to
`_cli` at line 43 and
never reads it, because there is nothing in it to read; the call's whole effect
is the parse and the two early exits. *Proving a negative* reads the model
instead of the value; one of its two tests is the guard that keeps this line
empty, and the other checks that whatever the model does list is described.

<!-- fragment «surface-empty-struct» owner="no-arguments" source="crates/grove/src/cli.rs" lines="19-19" parent="surface-grammar" -->
````rust
pub struct Cli {}
````
<!-- /fragment -->

<a id="worked-argv"></a>
## Worked example: four argument vectors

The invocation is the one *Orientation* carries: a Jujutsu workspace at
`/work/atlas/` holding a grove with one live leaf, `rate-limit-k3` of kind
`impl`, and `grove` typed in `crates/gateway/src/`. This section runs that
invocation's argument vector through the grammar above, and beside it the three
other vectors a reader can type: two that stop before the flow and one that is
refused. The output shown is what the built binary prints at the frozen corpus,
measured by running it rather than reconstructed from the renderer's
conventions; the guide's transcripts of `--help` and `--version` match it byte
for byte, and the refusal is measured here because the guide does not show one.

```console
$ grove
grove: launching impl with configured "claude" — rate-limit-k3
…

$ grove --help
Grove: hierarchical workstream tool for AI agents

Usage: grove

Options:
  -h, --help     Print help
  -V, --version  Print version

$ grove --version
grove 20.1.0

$ grove --harness claude
error: unexpected argument '--harness' found

Usage: grove

For more information, try '--help'.
```

The transcript is the grammar rendered by its own parser. The bare vector
parses to `Cli {}` and the flow continues — the first line after it is the
loop's, and the rest of that ending is *Orientation*'s trace. The other three
never reach line 44. The table beside it says how far each vector gets, which
stream it writes, and what the shell reads back.

| Argument vector | What `Cli::parse()` does | Last line of `run` reached | Stream | Exit status |
|---|---|---|---|---|
| `grove` | Returns `Cli {}` | lines 44 to 51: resolve, lease, run | the loop's own lines | `0`; `1` when a step refuses; `128 + N` after a signal |
| `grove --help` | Prints the help text and exits | line 43 | stdout | `0` |
| `grove --version` | Prints `grove 20.1.0` and exits | line 43 | stdout | `0` |
| `grove --harness claude` | Prints the refusal and exits | line 43 | stderr | `2` |

Two of those rows prove a claim the doc comment only states. `--help` and
`--version` *stop before the flow*: they discover no repository and acquire no
lease, because `Cli::parse()` at line 43 is the first statement of `run` and
`std::env::current_dir()` at line 44 is the second, and a parse that exits never
reaches it. The measurement is that the same binary, run bare in an empty
directory that is not a Jujutsu workspace, fails at *Three steps*' resolve with
exit status `1` and the message *not a Jujutsu working tree*, while
`grove --help` and `grove --version` in that same directory succeed with exit
status `0`. No line of this crate makes that ordering; it is the position of
the parse in `run`, and the next chapter reads the four statements that follow
it.

The fourth row is a refusal, and it is the parser's rather than this crate's.
`--harness` is an argument that would name a harness, which is exactly the fact
the configuration file owns, so the surface has no such flag; `clap` reports the
unknown argument, prints the usage line, and exits with `2`, its own status for
a usage error, on stderr. There is no line in `cli.rs` that produces that text
and no test that asserts it: what *Proving a negative* asserts is the closure
property that makes every such vector fail, not any particular refusal. The
usage line the refusal prints, `Usage: grove`, is the grammar restated by the
renderer — the invoked name, then nothing: no placeholder for an option or a
command — and it is the same line `--help` prints above its options.

Three exit statuses meet at this boundary, and they belong to three actors. `0`
after `--help` or `--version` is `clap`'s early exit. `2` after an unknown
argument is `clap`'s usage error. `1` after bare `grove` in the wrong directory
is `main` returning an error from one of the three steps, which *Three steps*
owns — and the fourth, `128 + N`, is the driver dying of the signal it was
sent, which that chapter owns too. The grammar itself decides only the first
two.

<a id="the-agent-surface"></a>
## The other binary: twelve verbs, flat

The agent binary is the other half of the audience split, and the catalogue
comes after the example because the example is what the split is for: a human
types one of the four vectors above, and everything a session does to the tree
is one of the twelve verbs below. `grove-llm`'s source is
`crates/grove-llm/src/cli.rs`, another book's corpus, and no byte of it is
reproduced here; the verbs are named, grouped by what they do to the tree, and
described in one line each, condensed from the binary's help text and the
guide. The [user guide's account of the tree
verbs](../../USAGE.md#usage-tree-verbs) is
where each is shown running.

| Verb | Group | What it does to the tree |
|---|---|---|
| `root-init` | scaffold | Creates `.grove/`, the root brief and a first `requirements` leaf. Bare `grove` performs this itself on a tree with no `.grove/`. |
| `pick` | read | Prints the next live leaf — the same depth-first pre-order answer the driver computes. |
| `brief-chain` | read | Prints the `BRIEF.md` chain root to leaf, one path per line. |
| `kind` | read | Prints a leaf's kind token, read from its filename. |
| `resolve` | read | Turns a key, handle or slug into the entry's current path, live or terminal. |
| `leaf-add` | grow | Appends one leaf per `--kind` under a node, with fresh keys. |
| `leaf-insert` | grow | Inserts a leaf at a sibling's position and renumbers the later siblings. |
| `leaf-decompose` | grow | Turns a live leaf into a node directory with the leaf's body as its brief and one first child. |
| `leaf-retire` | retire | Marks a live leaf `DONE` in place. |
| `leaf-prune` | retire | Marks a leaf or node `ABANDONED` in place, after a human confirms. |
| `finish-commit` | end | Deletes `.grove/` and commits that deletion; the only verb that commits. |
| `complete` | end | Signals the loop that this session's task is done. |

The table is the whole agent surface, so the split can be held as two concrete
lists: nothing, and these twelve. Three facts about the list are checked rather
than described. It is **flat** — no verb has subcommands of its own — and
`the_grove_llm_verb_surface_is_flat` in
`crates/grove-llm/tests/instructed_verbs.rs` asserts that against clap's model,
because the same file compares the verbs the shipped methodology instructs
against the verbs the binary exposes by their bare names, and that comparison
is only sound while one word after `grove-llm` names a whole command. It is
**twelve**: the count is the number of variants the binary's command enum
declares, and `grove-llm --help` lists them one line each — `help` is clap's
own, exactly as `--help` is here, and is not one of them. And the methodology
instructs **ten** of the twelve, pinned as a complete set in that same test:
the two it never instructs are `root-init`, which bare `grove` performs before
any session exists, and `kind`, a diagnostic the loop answers for itself.

The flatness is the agent-side counterpart of the empty grammar. Both surfaces
are shaped for a reader who has nothing but the binary: a human who types
`grove` and needs to select nothing, and a session that has dropped its context
and needs one `--help` to recover every call it might make. The description of
both surfaces is this chapter's; the reasons the agent surface has the members
it has, and the members it once had, are recorded in `docs/ARCHITECTURE.md`
and are not this book's to restate.

The grammar has now been read and it selects nothing. What `run` does with the
`Cli` it parses — the working tree resolved once, the lease taken, the call,
and the two endings — is the next chapter.

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Three steps](03-three-steps.md)

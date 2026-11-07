# Orientation
<!-- book-page id="orientation" slice="compiler-held" order="1" -->
[Contents](README.md) | [Next: The surface](02-the-surface.md)

<a id="compiler-held"></a>
## Lifecycle dispatch and standalone orchestration

`grove` is the human's command. Bare invocation in a Jujutsu working tree
resolves that tree, takes its driver lease and calls the loop. The loop chooses
the next task and launches it through `ravel`, which selects the
command. Viewing has a separate early-return path. `grove run` names
one kind explicitly and asks the owner's Ravel policy for its command.
It orchestrates its private temporary files, completion and output transfer,
and it does not enter a task-tree lifecycle.

This book's corpus is `crates/grove`: its manifest and all production modules.
The first chapters explain the package boundary, parser, lifecycle dispatch and
its checks. The module map follows, then the standalone
invocation chapter explains staging, supervision, publication and log display.
The library calls expose the operations needed by these paths without making
private loop implementation available to the binary.

The compiler enforces that public-interface boundary. It does not enforce a
particular amount of binary code. The lifecycle entry is small because the loop
owns lifecycle decisions. Standalone artifact transfer belongs here because the
human command owns its inputs and destinations. Tests separately hold the
parser's command set and help coverage.

<a id="two-products"></a>
## Two products, and which one `grove` enters

The repository that holds this crate ships two products. The manifest's own
phrase *the package that is grove-the-product* appears in the test commentary
below. A reader needs the distinction before reading that phrase. The table
names both products so that the phrase has a referent.

| Product | Source | What installs it |
|---|---|---|
| The Grove command-line tool: `grove` and `grove-llm` | `crates/` | Homebrew, or `cargo install` from the workspace |
| The agent skill plugins, `grove` among them | `plugins/` | Homebrew, with the Grove formula, and Ravel delivers them to each session |

The binary names the `grove-<kind>` skill a session must load. It installs no
skill. Homebrew installs the skill sources in Grove's keg, and Ravel delivers
them to each session. The plugin files stay the authoritative source, and no
binary embeds them. This book does not reconstruct
the plugin content.

<a id="package-identity"></a>
## The package, and what it inherits

The manifest is production source, and this chapter reconstructs all lines of
it in eight fragments. The fragments follow the file's own order. This chapter
reads the manifest first because the manifest declares the first mechanism, and
no other place argues it. The mechanism is that the package is a crate, and the
crate has no library.

<!-- fragment «manifest-thin-by-construction» owner="compiler-held" source="crates/grove/Cargo.toml" lines="1-59" parent="source-crate-manifest" -->
<!-- insert «manifest-package-identity» -->
<!-- insert «manifest-human-binary» -->
<!-- insert «manifest-crate-not-a-bin» -->
<!-- insert «manifest-no-lib-one-target» -->
<!-- insert «manifest-dependencies» -->
<!-- insert «manifest-tests-live-here» -->
<!-- insert «manifest-dev-dependencies» -->
<!-- insert «manifest-lints» -->
<!-- /fragment -->

The first fragment is the package block. The block inherits five of its fields
from the workspace root and does not state them. One of those five matters to a
later page: `version`. Every crate on the path from this binary to the loop takes
`version.workspace = true`, so there is one release version.

*The surface* reads that version as the constant that both binaries report. The
input is a `cargo release` cut, and the output is one number in every crate that
an operator installs. The invariant is that `grove --version` and
`grove-llm --version` cannot disagree. In the invocation that this chapter
carries, a reader sees that number first if they type `grove --version` before
they type `grove`.

<!-- fragment «manifest-package-identity» owner="compiler-held" source="crates/grove/Cargo.toml" lines="1-8" parent="manifest-thin-by-construction" -->
````toml
[package]
name = "grove"
description = "Hierarchical, self-extending workstream tool for AI agents"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
````
<!-- /fragment -->

<a id="the-binary"></a>
## The human's binary, and nothing else

The first comment states the crate's shape in one sentence. This page reads that
sentence and does not paraphrase it, because every later page is a longer
reading of one clause of it. Bare `grove` parses no argument that selects
anything, acquires the driver lease, and calls `grove_loop::run`. That call is
the loop's single entry point, and the bare lifecycle after its startup checks
is behind it. The loop's own pages belong to another book, and this page names
the call so the reader can see where the book stops.

The comment cites decision 9 of `docs/specs/module-decomposition.md`, the design
record that put the whole loop behind one function. That record is evidence for
the author. The fact that the reader needs is on the page: one call, and nothing
before it that chooses.

<!-- fragment «manifest-human-binary» owner="compiler-held" source="crates/grove/Cargo.toml" lines="9-13" parent="manifest-thin-by-construction" -->
````toml

# **The human's binary, and nothing else.** Bare `grove` parses no argument that
# selects anything, acquires the driver lease and calls `grove_loop::run`; the
# whole of the loop is behind that call (`docs/specs/module-decomposition.md`,
# decision 9).
````
<!-- /fragment -->

The relevant clause is *parses no argument that selects anything*. A launcher
ordinarily selects at least two things on its command line: what to run, and
how to run it. Here both were moved out of the binary before the binary was
written.

What to run comes from the [task tree](../../../CONTEXT.md#task-tree-scheme) in
the working tree. The loop walks that tree for its next live leaf. The human's
personal `ravel` policy, `~/.config/ravel/policy.ts`, decides how to run it. Its
`select` function returns the command for each launch. With both facts on disk,
an argument that selected either would be a second source for a fact that
already has one.

*The surface* reads the grammar that results. *Proving a negative* reads the
test that keeps the bare lifecycle free of selectors. The same test keeps the
subcommand set at `{run, run-log, view}`, with `run-log` hidden from ordinary
help.

<a id="crate-not-a-bin"></a>
## A crate, not a `[[bin]]` target

The second comment is the chapter's argument. It names the alternative that it
rejects. That alternative is a `[[bin]]` target in the manifest of `grove-loop`,
with the same small `main` at `src/bin/grove.rs`. Cargo would accept it, and the
binary would take one file less to describe. The cost is the thesis, but only
for one of the two shapes that such a target can take. The comment therefore
names that shape and does not state the clause for binary targets in general.

Rust draws privacy at the crate. The first shape is a binary target that lists
the library's modules as its own. An example is `#[path = "../driver_lease.rs"] mod driver_lease;`
beside its `main` at `src/bin/grove.rs`. That target compiles the same files a
second time. It is the same crate as the code that it includes, so it can name
any `pub(crate)` item in that code. Examples are the control-directory accessor
of `driver_lease`, the child-environment scrub of the loop, and the tree
operations that no verb exposes.

The attribute is not decoration. A bare `mod driver_lease;` in a file under
`src/bin/` resolves against `src/bin/` and gives `E0583`. Such a target holds its
own second copy. A type that it declares that way is distinct from the type of
the library, and the compiler reports `E0308`. So the shape defeats the
discipline at source level. That level is the only one that the thesis was ever
about.

The second shape is a binary target that depends on the library. It sees only
what the library publishes, exactly as a separate crate does. A path through the
library to a `pub(crate)` item gives `E0603`. A scratch package with one library
and one binary beside it measured this. The same item compiles when the binary
includes its source file as a module. The second shape is the ordinary one,
because Cargo gives it to a `src/bin/grove.rs` beside a `src/lib.rs` without any
request.

A clause that did not name the first shape would be false of the arrangement
that a reader pictures.

The package boundary does not make the first shape impossible. A `#[path]` can
point outside a package, and three test targets of this repository do exactly
that. The boundary adds one thing. From here, the first shape has to reach
outside the package. The production modules do not include private library
source. The test-only module in `run_display.rs` includes a fixture file from
the tests of this package, so the move is visible and not unavailable.

Inside the package of `grove-loop`, *the binary is thin* would be a fact about
the shape that the current commit uses. Whoever reviews the next commit would
hold it. Through its loop dependency, the entry point sees the public items of
`crates/grove-loop/src/lib.rs`, and naming a private item is a compile error.
That is the whole of the first mechanism. The compiler holds the boundary, and no
test needs to assert it. A reader is therefore most likely to take this mechanism
on trust.

The check is one line. Every `grove_loop::` path that the Rust files name is a
`pub` re-export in the root of that library. The sentence of the comment and
decision 1 of `docs/specs/module-decomposition.md` both name the shape that the
clause holds for. This section spells out the shape that it excludes. The binary
owns standalone artifact handling.

<!-- fragment «manifest-crate-not-a-bin» owner="compiler-held" source="crates/grove/Cargo.toml" lines="14-20" parent="manifest-thin-by-construction" -->
````toml
#
# **A crate, not a `[[bin]]` target, and that is the point** (decision 1). A
# binary target inside `grove-loop` that compiled the library's modules into
# itself could name the items that library keeps private, so *the binary is
# thin* would stop being compiler-enforced. Here the compiler holds it:
# main reaches only public loop and viewer interfaces; standalone artifact
# handling is a binary-owned adapter.
````
<!-- /fragment -->

The third comment is the consequence of the second one. It connects the package
boundary to *Proving a negative*. An integration test under `tests/` can import
a package that has a `[lib]` beside its `[[bin]]`. A test that inspects the clap
model could therefore live there.

This package has no `[lib]`. It is one binary target, and `cli.rs` is a module of
that target. Only code inside a binary can reach a clap model that the binary
declares. So the tests that hold the grammar closed are a `#[cfg(test)] mod tests`
at the bottom of `cli.rs`, in the same file as the production parser. They are
not a file under `tests/`.

The alternative is a `[lib]` that exists only so a test can reach the model. It
would give the binary a library to reach into, and the second comment made this a
crate to prevent exactly that. The `[[bin]]` table that follows is the one
target: `grove`, at `src/main.rs`.

<!-- fragment «manifest-no-lib-one-target» owner="compiler-held" source="crates/grove/Cargo.toml" lines="21-27" parent="manifest-thin-by-construction" -->
````toml
#
# **There is no `[lib]`.** The package is one binary target, so `cli.rs` is a
# module of the binary and its clap model is asserted by the binary's own unit
# tests rather than by an integration test that would need a library to import.
[[bin]]
name = "grove"
path = "src/main.rs"
````
<!-- /fragment -->

<a id="one-dependency"></a>
## Public library seams and binary-owned transfers

The binary depends on `anyhow` for returned errors and `clap` for parsing.
Two domain libraries provide its entry paths. `grove-loop` owns the lifecycle,
and `grove-tui` owns observation. Only the viewer pulls in Ratatui and
Crossterm.

`Workspace` still arrives through the re-export of the loop, and the human
binary never opens a store lock itself. The manifest therefore records two
public entry seams without importing their private implementation modules.
Standalone invocation launches `ravel` through the `ravel_job` module of
`grove-loop`. It loads no configuration and takes its command from `ravel`.

`serde_json` serves integration fixtures. `tempfile` owns scratch storage and
staged exports. `libc` supplies the artifact-open flags of the standalone module.

The dependency comment names the two public entry points and places terminal
dependencies behind the viewer. That keeps display concerns out of the loop and
the agent binary. The exact manifest fragment below records those edges.

<!-- fragment «manifest-dependencies» owner="compiler-held" source="crates/grove/Cargo.toml" lines="28-38" parent="manifest-thin-by-construction" -->
````toml

[dependencies]
anyhow = "1.0"
clap = { version = "4", features = ["derive"] }
# Public loop and read-only viewer entry points; terminal dependencies stay
# behind grove-tui.
grove-loop = { path = "../grove-loop" }
grove-tui = { path = "../grove-tui" }
serde_json = "1.0"
tempfile = "3.10"
libc = "0.2"
````
<!-- /fragment -->

<a id="tests-live-here"></a>
## Where the repository-surface tests live

The longest comment in the manifest is about files that this book does not
reconstruct. This chapter owns the comment because here a reader first meets
the sentence *this is the package that is grove-the-product*. The comment names
four tests: `tests/reference_navigation.rs`, `tests/plugin_fallback.rs`,
`tests/commit_guidance.rs` and `tests/retire_guidance.rs`. They make claims
about the repository. They claim that every Markdown reference resolves and that
the methodology plugin installs. They also claim that the guidance a skill gives
about committing and retiring matches what the verbs do.

None of those is a claim about the code of this crate, and the comment records
why the tests are here anyway. They lived at the workspace root while the root
was a package. `loop-crate-driver-k22` made the root a bare workspace, and a test
has to belong to some package. This is the package that is the product, so the
tests came here. They did not go into a library, which has no reason to know that
a README exists.

The paths of the tests use `../../` relative to this manifest. That is
the one visible cost of the move. The last two lines are about a different set
of tests for a different reason. The loop fixtures in `tests/loop_driver.rs` and
`tests/lifecycle_cutover.rs` drive `grove` as a spawned process against a
temporary tree. That is the only harness that a binary with no library can give
them.

The comment names the original repository-surface tests. Later tests include
viewer and standalone fixtures. All of them are evidence for this book, not
corpus. The book cites `tests/` directories and never reproduces them.

<!-- fragment «manifest-tests-live-here» owner="compiler-held" source="crates/grove/Cargo.toml" lines="39-50" parent="manifest-thin-by-construction" -->
````toml

# **The repository-surface tests live here**, and that is deliberate rather than
# incidental. `tests/reference_navigation.rs`, `tests/plugin_fallback.rs`,
# `tests/commit_guidance.rs` and `tests/retire_guidance.rs` make claims about the
# repository — its user-facing documents, and the methodology plugin it ships —
# rather than about any crate's code. They lived at the root while the root was a
# package; `loop-crate-driver-k22` made it a bare workspace, and this is the
# package that *is* grove-the-product, so they came here rather than into a
# library that has never heard of a README. Their paths are spelled `../../`
# relative to this manifest.
# `grove` itself is driven as a *process* by the loop fixtures, so the only
# harness they need is a real spawn against a temporary tree.
````
<!-- /fragment -->

The dev-dependencies are the three that the tests above need. No consumer
inherits them, since nothing depends on a binary. `book-validation` is this
book's own validator. The two tests that check the walkthroughs against the
repository read it as a library.

`libc` is for `kill(2)`: the loop fixture sends the driver `SIGTERM` to drive its
interrupt path. The handler catches `SIGHUP` too, and no fixture sends it.
`std::process::Child::kill` sends only `SIGKILL`, which is exactly the signal
that the driver cannot answer. That path, in which the driver dies of the signal
that it received, is the second ending in *Lifecycle startup*. This line is the
first trace of that path in the corpus. `tempfile` supplies the temporary trees.

<!-- fragment «manifest-dev-dependencies» owner="compiler-held" source="crates/grove/Cargo.toml" lines="51-56" parent="manifest-thin-by-construction" -->
````toml
[dev-dependencies]
book-validation = { path = "../book-validation" }
# `libc` for the loop fixtures' own `kill(2)`: they signal the driver process to
# drive its interrupt path, which `std::process::Child::kill` cannot express.
libc = "0.2"
tempfile = "3.10"
````
<!-- /fragment -->

The package inherits the lint configuration for the same reason that it inherits
`version`: one workspace, one standard, and nothing crate-specific to add.

<!-- fragment «manifest-lints» owner="compiler-held" source="crates/grove/Cargo.toml" lines="57-59" parent="manifest-thin-by-construction" -->
````toml

[lints]
workspace = true
````
<!-- /fragment -->

<a id="one-invocation"></a>
## One invocation, end to end

This is the invocation that the book carries. It appears at low resolution here,
and as its argv in *The surface*. It appears at full resolution in *Lifecycle
startup*, with every call and both endings. In *Proving a negative*, it appears
as the guard that keeps the argv from growing. This page fixes the tree, the leaf
and the policy below, and all three chapters reuse them unchanged.

The initial tree is a Jujutsu workspace holding a grove with one live leaf,
and the human's policy returns one command for every kind:

```text
/work/atlas/
├── .jj/
├── .grove/
│   ├── _BRIEF.md
│   └── 01-impl--rate-limit-k3.md          live: no DONE or ABANDONED infix
└── crates/
    └── gateway/
        └── src/                            the directory grove is typed in

~/.config/ravel/policy.ts
    export const policy = {
      schemaVersion: 2,
      version: "atlas-1",
      select: (request) => ({
        status: "selected",
        program: "claude",
        args: ["--add-dir", request.cwd, request.prompt],
        provider: "anthropic", model: "default", effort: "default",
        reason: "every kind runs claude",
      }),
    };
```

The human is somewhere inside the tree, not at its root, which is the ordinary
case. Here is what happens, step by step, with the values that this tree
produces. Setup is the binary's responsibility, and each iteration is behind the
call.

```text
$ grove
  parse     argv is ["grove"]: there is nothing to select, and nothing is
  resolve   the nearest ancestor of crates/gateway/src holding .jj/ is /work/atlas
  lease     /work/atlas/.jj/grove/driver.lease is locked; this process is the
            tree's one driver for as long as it runs
  dispatch  independent Ravel is resolved/version-checked once on PATH
  run       grove_loop::run(workspace, lease, dispatch) — once per iteration:
              read the task tree    the first live leaf is rate-limit-k3, kind impl
              launch, then wait     grove: launching impl through Ravel — rate-limit-k3
                                    the policy selects claude; Ravel supervises and reaps it
              read the result       grove-continue after reap: relaunch
            … later iterations retire every live leaf, and the teardown session
            signals that the grove itself is done:
                                    grove: grove finished — loop complete.
  -> run returns Ok(()); the process exits 0
```

The setup lines establish facts that the loop consumes. *Resolve* is a
filesystem walk that ends at the nearest `.jj/`, and it refuses a tree with none.
*Lease* takes the [driver lease](../../../CONTEXT.md#driver-lease), the
one-driver-per-working-tree claim. The process holds that lease for its whole
life. A second `grove` typed in the same tree is therefore refused, and it does
not run beside this one.

The *run* line is the call. The lines indented under it are what the loop
prints, not what the binary does. The binary has no line of its own between the
call and the exit.

The ending above is one of two. By design, the loop stops when the grove
finishes or when a session ends without signalling. Each of those reasons comes
back as a value, and the process exits 0.

The other ending occurs when the driver receives `SIGTERM` or `SIGHUP`
mid-grove. A killed driver does not exit 0. After the loop cleans up, the process
dies of the same signal, so whoever launched it reads `128 + N` instead of
success. That ending is the most argued claim in the corpus, and *Lifecycle
startup* owns it.

<a id="three-mechanisms"></a>
## Entry boundaries and their checks

The reader now knows one invocation end to end, so the map follows. The three
mechanisms describe the lifecycle entry. The chapter map gives the chapter that
reads each one. The table says which page holds which, and which pages hold
neither because their job is the invocation itself.

| # | Mechanism | Held by | Chapter |
|---:|---|---|---|
| 1 | A package boundary: the entry point can reach only what the library publishes | the compiler | Orientation |
| — | The grammar that results, and the agent surface beside it | — | The surface |
| — | Startup checks and the signal path | — | Lifecycle startup |
| 2 | A closure property: bare lifecycle selectors are absent and the top-level command set is run, run-log and view | `the_human_command_surface_has_nothing_left_to_select` | Proving a negative |
| 3 | A convention, checked: every option the binary lists is described | `the_human_facing_binary_describes_every_option_it_lists` | Proving a negative |
| — | The module map | — | What the call reaches |
| — | Private scratch state, checked export and parent-owned display | Native policy and artifact checks | One isolated invocation |

Mechanism 1 belongs to this chapter, and this chapter now covers all of it. Its
parts are a separate crate, no `[lib]`, one target and public library
dependencies. They also include a manifest whose comments state each of those
parts as a decision, not a default. Tests in the file that holds the grammar
assert the remaining two mechanisms. The next chapter reads that grammar.

[Contents](README.md) | [Next: The surface](02-the-surface.md)

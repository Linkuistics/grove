# Orientation
<!-- book-page id="orientation" slice="compiler-held" order="1" -->
[Contents](README.md) | [Next: The surface](02-the-surface.md)

<a id="compiler-held"></a>
## Lifecycle dispatch and standalone orchestration

`grove` is the human's command. Bare invocation in a Jujutsu working tree
resolves that tree, takes its driver lease and calls the loop. The loop chooses
the next task and configured command. Viewing, configuration inspection and
inactive sample delivery have separate early-return paths. `grove run` selects
one configured kind explicitly and orchestrates its confined temporary files,
completion and output transfer without entering a task-tree lifecycle.

This book's corpus is `crates/grove`: its manifest and all production modules.
The first chapters explain the package boundary, parser, lifecycle dispatch and
its checks. Configuration and example presentation follow, then the standalone
invocation chapter explains staging, supervision, publication and log display.
The library calls expose the operations needed by these paths without making
private loop implementation available to the binary.

The compiler enforces that public-interface boundary. It does not enforce a
particular amount of binary code. The lifecycle entry is small because the loop
owns lifecycle decisions; standalone artifact transfer belongs here because the
human command owns its inputs and destinations. Tests separately hold the
parser's command set and help coverage.

<a id="two-products"></a>
## Two products, and which one `grove` enters

The repository that holds this crate ships two products, and the manifest's own
phrase — *the package that is grove-the-product*, in the test commentary below —
needs the distinction stated before it is read. The table names both so
that phrase has a referent.

| Product | Source | What installs it |
|---|---|---|
| The Grove command-line tool: `grove` and `grove-llm` | `crates/` | Homebrew, or `cargo install` from the workspace |
| The agent skill plugins, `grove` among them | `plugins/` | A harness's plugin marketplace, or `plugins/install.sh` |

Neither product installs the other. The binary names the `grove-<kind>` skill a
session must load and does not check that the plugin holding it is present;
whether a machine has the methodology installed is outside anything the binary
reads. `crates/grove` is the entry point to the first product only, and every
page of this book is about that product. The plugin is in no crate and therefore
in no book; *What the call reaches* names it and stops.

<a id="package-identity"></a>
## The package, and what it inherits

The manifest is production source and this chapter reconstructs all
lines of it, in eight fragments that follow the file's own order. It is read
first because the first mechanism is declared there rather than argued anywhere
else: the package is a crate, and the crate has no library.

<!-- fragment «manifest-thin-by-construction» owner="compiler-held" source="crates/grove/Cargo.toml" lines="1-61" parent="source-crate-manifest" -->
<!-- insert «manifest-package-identity» -->
<!-- insert «manifest-human-binary» -->
<!-- insert «manifest-crate-not-a-bin» -->
<!-- insert «manifest-no-lib-one-target» -->
<!-- insert «manifest-dependencies» -->
<!-- insert «manifest-tests-live-here» -->
<!-- insert «manifest-dev-dependencies» -->
<!-- insert «manifest-lints» -->
<!-- /fragment -->

The first fragment is the package block. Five of its fields are inherited from
the workspace root rather than stated, and one of those five matters to a later
page: `version`. Every crate on the path from this binary to the loop takes
`version.workspace = true`, so there is one release version, and *The surface*
reads it as the constant both binaries report — the input is a `cargo release`
cut, the output is one number in every crate an operator installs, and the
invariant is that `grove --version` and `grove-llm --version` cannot disagree.
In the invocation this chapter carries, that number is what a reader sees first
if they type `grove --version` before typing `grove`.

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

The first comment states the crate's shape in one sentence, and it is read here
rather than paraphrased because every later page is a longer reading of one
clause of it. Bare `grove` parses no argument that selects anything,
acquires the driver lease, and calls `grove_loop::run`. That call is the
loop's single entry
point, and the bare lifecycle after its three setup steps is behind it — the
loop's own pages are another book's, and this page names the call so the reader
can see where the book stops. The comment cites decision 9 of
`docs/specs/module-decomposition.md`, which is the design record that put the
whole loop behind one function; that record is evidence for the author, and the
fact the reader needs is on the page — one call, nothing before it that
chooses.

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
written. What to run is read from the
[task tree](../../../CONTEXT.md#task-tree-scheme) in the working tree, which the
loop walks for its next live leaf; how to run it is read from the human's
personal configuration, `~/.config/grove/config.kdl`, which maps each session
kind to one complete command template. With both facts on disk, an argument
that selected either would be a second source for a fact that already has one.
*The surface* reads the grammar that results, and *Proving a negative* reads the
test that keeps the bare lifecycle free of selectors and the subcommand set
at `{run, run-log, config, view}`, with `run-log` hidden from ordinary help
and `examples` and `show` beneath `config`.

<a id="crate-not-a-bin"></a>
## A crate, not a `[[bin]]` target

The second comment is the chapter's argument, and it names the alternative it
rejects: a `[[bin]]` target declared inside `grove-loop`'s own manifest, with
the same small `main` at `src/bin/grove.rs`. Cargo would accept that,
and the binary would be one file shorter to describe. What it would cost is
the thesis — but only for one of the two shapes such a target can take, which
is why the comment names that shape rather than stating the clause of binary
targets in general. Rust privacy is drawn at the crate. A binary target that
lists the library's modules as its own — `#[path = "../driver_lease.rs"] mod
driver_lease;` beside its `main` at `src/bin/grove.rs`, compiling the same
files a second time — is the same crate as the code it includes and can name
any `pub(crate)` item in it: `driver_lease`'s control-directory accessor, the
loop's child-environment scrub, the tree operations no verb exposes. The
attribute is not decoration: a bare `mod driver_lease;` in a file under
`src/bin/` resolves against `src/bin/`, and is `E0583`. What such a target
holds is its own second copy — a type it declares that way is distinct from
the library's, `E0308` — so the discipline the shape defeats is the
source-level one, which is the only one the thesis was ever about. A binary
target that instead depends on the library sees only what the library
publishes, exactly as a separate crate does; a `pub(crate)` item named through
the library's path is refused with `E0603`, measured on a scratch package with
one library and one binary beside it, and the same item reached by including
its source file as a module compiles. That second shape is the ordinary one —
it is what `src/bin/grove.rs` beside a `src/lib.rs` gets from Cargo without
anyone asking for it — so a clause that did not name the first shape would be
false of the arrangement a reader pictures. What the package boundary adds is
not that the first shape becomes impossible — a `#[path]` can point outside a
package, and three of this repository's test targets do exactly that — but
that from here it has to, and the production modules do not include private library source. The test-only
module in `standalone.rs` includes a fixture file from this package’s own tests. The
move is made visible rather than unavailable. Inside `grove-loop`'s package,
*the binary is thin* would be a fact about which of the two shapes the current
commit uses, held by whoever reviews the next one. Through its loop dependency
the entry point sees public items from `crates/grove-loop/src/lib.rs`; naming a
private item is a compile error. That is
the whole of the first mechanism: the boundary is the compiler's, and no test
is needed to assert it, which is also why it is the mechanism a reader is most
likely to take on trust. The check is one line: every `grove_loop::` path the
Rust files name is a `pub` re-export in that library's root. The comment's
own sentence, and decision 1 of `docs/specs/module-decomposition.md` which it
cites, both name the shape the clause holds for. This section spells out the
shape it excludes. The same public
boundary holds for `keyed-launch`, while standalone artifact handling remains
owned by the binary.

<!-- fragment «manifest-crate-not-a-bin» owner="compiler-held" source="crates/grove/Cargo.toml" lines="14-20" parent="manifest-thin-by-construction" -->
````toml
#
# **A crate, not a `[[bin]]` target, and that is the point** (decision 1). A
# binary target inside `grove-loop` that compiled the library's modules into
# itself could name the items that library keeps private, so *the binary is
# thin* would stop being compiler-enforced. Here the compiler holds it:
# main reaches only public loop, viewer and keyed-launch interfaces; standalone
# artifact handling is a binary-owned adapter.
````
<!-- /fragment -->

The third comment is the consequence the second one forces, and it connects the
package boundary to *Proving a negative*. A package with a `[lib]` beside its
`[[bin]]` can be imported by an integration test under `tests/`, so a test that
inspects the clap model could live there. This package has no `[lib]`: it is one
binary target, `cli.rs` is a module of that target, and a clap model declared
inside a binary is reachable only from inside it. So the tests that hold the
grammar closed are a `#[cfg(test)] mod tests` at the bottom of `cli.rs` — the
same file as the production parser — rather than a file under `tests/`. The
alternative, a `[lib]` that exists only so a test can reach the model, would
give the binary a library to reach into, and that is the property the second
comment made this a crate to keep. The `[[bin]]` table that follows is the one
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
Two domain libraries provide its entry paths: `grove-loop` owns the lifecycle,
and `grove-tui` owns observation. Only the viewer pulls in Ratatui and
Crossterm. `Workspace` still arrives through the loop's re-export; the human
binary never opens a store lock itself. The manifest therefore records two
public entry seams without importing their private implementation modules.
`keyed-launch` supplies public inspection records to the formatter and
configured-command and confinement APIs to standalone invocation. SessionConfig
owns ordinary project-policy resolution; the standalone runner loads a personal
catalog without an overlay. `serde_json` encodes configuration reports,
`tempfile` owns scratch storage and staged exports, and `libc` supplies the
artifact-open flags used by that runner.

The dependency comment names the two public entry points and places terminal
dependencies behind the viewer. That keeps display concerns out of the loop
and agent binary; the exact manifest fragment below records those edges.

<!-- fragment «manifest-dependencies» owner="compiler-held" source="crates/grove/Cargo.toml" lines="28-40" parent="manifest-thin-by-construction" -->
````toml

[dependencies]
anyhow = "1.0"
clap = { version = "4", features = ["derive"] }
# Public loop and read-only viewer entry points; terminal dependencies stay
# behind grove-tui.
grove-loop = { path = "../grove-loop" }
grove-tui = { path = "../grove-tui" }
# Public inspection records; resolution remains SessionConfig's responsibility.
keyed-launch = { path = "../keyed-launch" }
serde_json = "1.0"
tempfile = "3.10"
libc = "0.2"
````
<!-- /fragment -->

<a id="tests-live-here"></a>
## Where the repository-surface tests live

The longest comment in the manifest is about files this book does not
reconstruct, and it is owned here because it is where a reader first meets the
sentence *this is the package that is grove-the-product*. The four tests it
names — `tests/reference_navigation.rs`, `tests/plugin_fallback.rs`,
`tests/commit_guidance.rs` and `tests/retire_guidance.rs` — make claims about
the repository: that every Markdown reference resolves, that the methodology
plugin installs, that the guidance a skill gives about committing and retiring
matches what the verbs do. None of those is a claim about this crate's code, and
the comment records why they are here anyway. They lived at the workspace root
while the root was a package; `loop-crate-driver-k22` made the root a bare
workspace, and a test has to belong to some package. This is the package that is
the product, so they came here rather than into a library with no reason to
know a README exists. Their paths are spelled `../../` relative to this
manifest, which is the one visible cost of the move. The last two lines are
about a different set of tests for a different reason: the loop fixtures in
`tests/loop_driver.rs` and `tests/lifecycle_cutover.rs` drive `grove` as a
spawned process against a temporary tree, which is the only harness a binary
with no library can offer them.

The comment names the original repository-surface tests. Later tests include
configuration inspection and viewer fixtures. All are evidence for this book
rather than corpus: `tests/` directories are cited and never reproduced.

<!-- fragment «manifest-tests-live-here» owner="compiler-held" source="crates/grove/Cargo.toml" lines="41-52" parent="manifest-thin-by-construction" -->
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

The dev-dependencies are the three the tests above need and no consumer
inherits, since nothing depends on a binary. `book-validation` is this book's
own validator, read as a library by the two tests that check the walkthroughs
against the repository. `libc` is for `kill(2)`: the loop fixture sends the
driver `SIGTERM` to drive its interrupt path — the handler catches `SIGHUP`
too, and no fixture sends it — and `std::process::Child::kill` sends
only `SIGKILL`, which is exactly the signal the driver cannot answer. That
path — the driver dying of the signal it was sent — is *Three steps*' second
ending, and this line is the first trace of it in the corpus. `tempfile`
supplies the temporary trees.

<!-- fragment «manifest-dev-dependencies» owner="compiler-held" source="crates/grove/Cargo.toml" lines="53-58" parent="manifest-thin-by-construction" -->
````toml
[dev-dependencies]
book-validation = { path = "../book-validation" }
# `libc` for the loop fixtures' own `kill(2)`: they signal the driver process to
# drive its interrupt path, which `std::process::Child::kill` cannot express.
libc = "0.2"
tempfile = "3.10"
````
<!-- /fragment -->

The lint configuration is inherited for the same reason `version` is: one
workspace, one standard, and nothing crate-specific to add.

<!-- fragment «manifest-lints» owner="compiler-held" source="crates/grove/Cargo.toml" lines="59-61" parent="manifest-thin-by-construction" -->
````toml

[lints]
workspace = true
````
<!-- /fragment -->

<a id="one-invocation"></a>
## One invocation, end to end

This is the invocation the book carries. It appears at low resolution here, as
its argv in *The surface*, at full resolution in *Three steps* with every call
and both endings, and in *Proving a negative* as the guard that keeps the argv
from growing. The tree, the leaf and the template below are fixed here and
reused unchanged by all three.

The starting tree is a Jujutsu workspace holding a grove with one live leaf,
and the human's configuration maps the `impl` kind to a command:

```text
/work/atlas/
├── .jj/
├── .grove/
│   ├── _BRIEF.md
│   └── 01-impl--rate-limit-k3.md          live: no DONE or ABANDONED infix
└── crates/
    └── gateway/
        └── src/                            the directory grove is typed in

~/.config/grove/config.kdl
    config {
        command "agent" "claude --add-dir ${repo} ${prompt}"
        bind "lead" "agent"
        route "impl" "lead"
    }
```

The human is somewhere inside the tree, not at its root, which is the ordinary
case. Here is what happens, step by step, with the values this tree produces;
the first three steps are the binary's and the rest is behind the call.

```text
$ grove
  parse     argv is ["grove"]: there is nothing to select, and nothing is
  resolve   the nearest ancestor of crates/gateway/src holding .jj/ is /work/atlas
  lease     /work/atlas/.jj/grove/driver.lease is locked; this process is the
            tree's one driver for as long as it runs
  run       grove_loop::run(workspace, lease, templates) — once per iteration:
              read the task tree    the first live leaf is rate-limit-k3, kind impl
              read config.kdl       impl's template, expanded into an argv
              launch, then wait     grove: launching impl with configured "claude" — rate-limit-k3
              read the signal       the session signalled completion: relaunch
            … later iterations retire every live leaf, and the teardown session
            signals that the grove itself is done:
                                    grove: grove finished — loop complete.
  -> run returns Ok(()); the process exits 0
```

Three of those lines are the bare lifecycle's setup, and each is the subject of a
later page. *Resolve* is a filesystem walk that ends at the nearest `.jj/`, and
it refuses a tree with none. *Lease* takes the
[driver lease](../../../CONTEXT.md#driver-lease) — the one-driver-per-working-tree
claim, held for the life of the process, so a second `grove` typed in the same
tree is refused rather than run beside this one. *Run* is the call, and the
lines indented under it are what the loop prints, not what the binary does: the
binary has no line of its own between the call and the exit.

The ending shown is one of two. Every reason the loop was designed to stop —
the grove finished, or a session ended without signalling — comes back as a
value and the process exits 0. The other ending is the driver being sent
`SIGTERM` or `SIGHUP` mid-grove, and a driver that was killed does not exit 0:
after the loop has cleaned up, the process dies of the same signal, so whoever
started it reads `128 + N` instead of success. That ending is the most argued
claim in the corpus and *Three steps* owns it.

<a id="three-mechanisms"></a>
## Entry boundaries and their checks

Now that one invocation has been seen end to end, the map. The three mechanisms
describe the lifecycle entry; the chapter map shows where each is read. The
table says which page holds which, and which pages hold neither because their
job is the invocation itself.

| # | Mechanism | Held by | Chapter |
|---:|---|---|---|
| 1 | A package boundary: the entry point can reach only what the library publishes | the compiler | Orientation |
| — | The grammar that results, and the agent surface beside it | — | The surface |
| — | The three calls, and the signal path | — | Three steps |
| 2 | A closure property: bare lifecycle selectors are absent and the top-level command set is run, run-log, config and view | `the_human_command_surface_has_nothing_left_to_select` | Proving a negative |
| 3 | A convention, checked: every option the binary lists is described | `the_human_facing_binary_describes_every_option_it_lists` | Proving a negative |
| — | The module map and configuration presentation | — | What the call reaches |
| — | Confined scratch state, checked export and parent-owned display | Native policy and artifact checks | One isolated invocation |

Mechanism 1 is this chapter's, and it is now fully read: a separate crate, no
`[lib]`, one target, public library dependencies, and a manifest whose comments state
each of those as a decision rather than a default. The remaining two are
asserted by tests in the file the grammar lives in, and the grammar is read
next.

[Contents](README.md) | [Next: The surface](02-the-surface.md)

# Orientation
<!-- book-page id="orientation" slice="compiler-held" order="1" -->
[Contents](README.md) | [Next: The surface](02-the-surface.md)

<a id="compiler-held"></a>
## The binary that selects nothing

`grove` is the human's command. Typed bare at any directory inside a Jujutsu
working tree, it resolves that tree, takes the one-driver lease over it, and
calls the loop; the loop is everything else — which task runs next, which
command launches it, and when the grove is done. The binary parses no argument
that selects anything. Its whole source is three files and 204 lines, and 92 of
those lines are comments.

This book is the system's overview, and `crates/grove` is its corpus, because
that crate is where the system is entered and nothing else is decided. Each
chapter opens at one of the binary's own steps: the package it is, which is this
chapter; the grammar it accepts; the three calls it makes; the tests that hold
the grammar closed; and the modules the call reaches. What the system does
behind the call is named on those pages and explained on none of them, and
*What the call reaches* states that boundary in one place.

The chapter's thesis is the manifest's own: **the binary is thin, and the
compiler holds it so.** `grove` is a crate rather than a `[[bin]]` target inside
the library it calls, so everything its two Rust files can reach is something
`grove-loop` chose to publish. A binary that merely looks thin is held by
review — someone reads the entry point and finds no logic in it, and the next
commit is free to add some. This one is held by a package boundary, which is the
first of three mechanisms this book names. The other two are a property a test
asserts and a convention a test checks, and both are read in *Proving a
negative*. The three are what the reader takes away: given an entry point of
their own, which of the three holds it thin, or whether none does.

<a id="two-products"></a>
## Two products, and which one `grove` enters

The repository that holds this crate ships two products, and the manifest's own
phrase — *the package that is grove-the-product*, at line 41 below —
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

The manifest is production source and this chapter reconstructs all fifty-four
lines of it, in eight fragments that follow the file's own order. It is read
first because the first mechanism is declared there rather than argued anywhere
else: the package is a crate, and the crate has no library.

<!-- fragment «manifest-thin-by-construction» owner="compiler-held" source="crates/grove/Cargo.toml" lines="1-54" parent="source-crate-manifest" -->
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
point, and everything the binary does after its three steps is behind it — the
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
test that keeps it empty.

<a id="crate-not-a-bin"></a>
## A crate, not a `[[bin]]` target

The second comment is the chapter's argument, and it names the alternative it
rejects: a `[[bin]]` target declared inside `grove-loop`'s own manifest, with
the same thirteen-line `main` at `src/bin/grove.rs`. Cargo would accept that,
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
that from here it has to, and the two Rust files visibly do not carry one. The
move is made visible rather than unavailable. Inside `grove-loop`'s package,
*the binary is thin* would be a fact about which of the two shapes the current
commit uses, held by whoever reviews the next one. As a separate crate the
entry point sees exactly the items `crates/grove-loop/src/lib.rs` re-exports
and nothing else, and a `use` of anything further is a compile error. That is
the whole of the first mechanism: the boundary is the compiler's, and no test
is needed to assert it, which is also why it is the mechanism a reader is most
likely to take on trust. The check is one line: every `grove_loop::` path the
two Rust files name is a `pub` re-export in that library's root. The comment's
own sentence, and decision 1 of `docs/specs/module-decomposition.md` which it
cites, both name the shape the clause holds for; the comment says it in one
clause because five lines is all the room the manifest gives it, and this
section is where the shape it excludes is spelled out.

<!-- fragment «manifest-crate-not-a-bin» owner="compiler-held" source="crates/grove/Cargo.toml" lines="14-19" parent="manifest-thin-by-construction" -->
````toml
#
# **A crate, not a `[[bin]]` target, and that is the point** (decision 1). A
# binary target inside `grove-loop` that compiled the library's modules into
# itself could name the items that library keeps private, so *the binary is
# thin* would stop being compiler-enforced. Here the compiler holds it:
# everything `main.rs` can reach is something `grove-loop` chose to publish.
````
<!-- /fragment -->

The third comment is the consequence the second one forces, and it connects the
package boundary to *Proving a negative*. A package with a `[lib]` beside its
`[[bin]]` can be imported by an integration test under `tests/`, so a test that
inspects the clap model could live there. This package has no `[lib]`: it is one
binary target, `cli.rs` is a module of that target, and a clap model declared
inside a binary is reachable only from inside it. So the tests that hold the
grammar closed are a `#[cfg(test)] mod tests` at the bottom of `cli.rs` — the
same file, 84 of its 137 lines — rather than a file under `tests/`. The
alternative, a `[lib]` that exists only so a test can reach the model, would
give the binary a library to reach into, and that is the property the second
comment made this a crate to keep. The `[[bin]]` table that follows is the one
target: `grove`, at `src/main.rs`.

<!-- fragment «manifest-no-lib-one-target» owner="compiler-held" source="crates/grove/Cargo.toml" lines="20-26" parent="manifest-thin-by-construction" -->
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
## One grove dependency

Three dependencies, and one of them is grove. `anyhow` supplies the error type
the entry point returns — both Rust files end their one function in
`anyhow::Result<()>`, so a refusal from any of the three steps reaches the top
as a printed error and a non-zero exit. `clap` with `derive` supplies the
grammar, which *The surface* reads. `grove-loop`, by path, is the only grove
dependency, and the comment says why the binary is short: the loop is behind
it. Three other workspace crates are reached from here — the VCS seam that
resolves the working tree, the runner that spawns the session, the tree
store — and none is named in this manifest: what the binary takes from two
of them arrives as a `grove-loop` re-export, and it takes nothing from the
third. The
type that `cli.rs` calls `Workspace` is the VCS seam's own type, published
through the loop's root; *Three steps* reads that line.

The comment names no count, and the source is why it cannot. Outside the test
module at the bottom of `cli.rs` the crate defines two functions — `main`, and
the `run` it calls — while the helper and the two tests below take that to three
or to five depending on where a reader stops counting. Any number in the comment
would be right for one of those readings and wrong for the other two. *The
reason there is nothing else in it* is the same claim without the arithmetic,
and there is nothing left in it to go stale. The comment is part of the frozen
corpus and is reproduced as written.

<!-- fragment «manifest-dependencies» owner="compiler-held" source="crates/grove/Cargo.toml" lines="27-33" parent="manifest-thin-by-construction" -->
````toml

[dependencies]
anyhow = "1.0"
clap = { version = "4", features = ["derive"] }
# The loop. This binary's only grove dependency, and the reason there is
# nothing else in it.
grove-loop = { path = "../grove-loop" }
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

The directory holds nine files today and the comment names four; the other
five are later repository-surface tests and the two fixture files, and every
one of them is evidence for this book rather than corpus — `tests/`
directories are cited and never reproduced.

<!-- fragment «manifest-tests-live-here» owner="compiler-held" source="crates/grove/Cargo.toml" lines="34-45" parent="manifest-thin-by-construction" -->
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

<!-- fragment «manifest-dev-dependencies» owner="compiler-held" source="crates/grove/Cargo.toml" lines="46-51" parent="manifest-thin-by-construction" -->
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

<!-- fragment «manifest-lints» owner="compiler-held" source="crates/grove/Cargo.toml" lines="52-54" parent="manifest-thin-by-construction" -->
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
│   ├── BRIEF.md
│   └── 01-impl--rate-limit-k3.md          live: no DONE or ABANDONED infix
└── crates/
    └── gateway/
        └── src/                            the directory grove is typed in

~/.config/grove/config.kdl
    impl "claude --add-dir ${repo} ${prompt}"
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

Three of those lines are the binary's whole job, and each is the subject of a
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
## Three mechanisms, five chapters

Now that one invocation has been seen end to end, the map. The three mechanisms
are what the reader takes away; the five chapters are where each is read; the
table says which page holds which, and which pages hold neither because their
job is the invocation itself.

| # | Mechanism | Held by | Chapter |
|---:|---|---|---|
| 1 | A package boundary: the entry point can reach only what the library publishes | the compiler | Orientation |
| — | The grammar that results, and the agent surface beside it | — | The surface |
| — | The three calls, and the signal path | — | Three steps |
| 2 | A closure property: the human surface has nothing left to select | `the_human_command_surface_has_nothing_left_to_select` | Proving a negative |
| 3 | A convention, checked: every option the binary lists is described | `the_human_facing_binary_describes_every_option_it_lists` | Proving a negative |
| — | The module map, and the boundary of this book | — | What the call reaches |

Mechanism 1 is this chapter's, and it is now fully read: a separate crate, no
`[lib]`, one target, one grove dependency, and a manifest whose comments state
each of those as a decision rather than a default. The remaining two are
asserted by tests in the file the grammar lives in, and the grammar is read
next.

[Contents](README.md) | [Next: The surface](02-the-surface.md)

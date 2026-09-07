# What the call reaches
<!-- book-page id="what-the-call-reaches" slice="assembly" order="5" -->
[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

<a id="assembly"></a>
## The call, and what is behind it

This chapter owns no production source. The three roots and 204 lines are
already reconstructed by the fragment graph the four chapters before it built,
and the [source index](source-index.md) records that graph in full. What is
left is what no single chapter could state, because each read one step of the
binary and stopped at the call: where the work the binary hands off is done,
where this book stops, and whether the test the book promised holds when it is
applied back to the pages that taught it.

The chapter has no worked example. The invocation the book carries reached its
two endings in *Three steps*, and the guard on its argument vector was measured
in *Proving a negative*; nothing on this page changes a value in that trace.
The page's figures are maps. The first is the workspace the binary is built in
and what the binary names from each part of it; the second is the seven names
that cross the boundary, one line each; the third is the modules behind the
call and what each is responsible for; the fourth is the boundary itself, as a
list of what the four chapters named and did not explain. The test is then
applied to the three mechanisms in turn and to the other entry point the map
shows, the two ledgers are closed, and the final validation is recorded.

<a id="the-package-map"></a>
## Seven members, and what the binary names from each

The manifest at the workspace root names seven members, and the root itself is
not a package. `loop-crate-driver-k22` moved the last of the root's own source
into `crates/grove-loop` and left the root holding the member list, the one
release version six of the members read, and the lint gate they inherit. Those
six are the product an operator installs — the two binaries and the four
libraries beneath them — and `docs/RELEASING.md` cuts them as one release
under one tag. The seventh, `book-validation`, is the authoring tool
behind this book and carries a version of its own, as *The surface* recorded:
it is a member,
`scripts/check.sh` runs it with `cargo run`, and *Orientation* read the one line
of this crate's manifest that names it, a dev-dependency of the tests.

The table is the map, read from the seven package manifests and from
`grove-loop`'s crate root. The third column is each package's `[dependencies]`
inside the workspace — dev-dependencies, which a binary does not link, are
left out of it; the fourth is what `crates/grove`'s two Rust files name
from it. What the reader is to take from it is the difference between those
two columns
for the binary: `grove` depends on one package, names items from three, and is
linked against four, and the first mechanism is the reason those three numbers
differ.

| Package | What it is | Depends on, inside the workspace | What `grove` names from it |
|---|---|---|---|
| `grove` | The human's binary, and this book's corpus | `grove-loop` | Its own module `cli` — the one row of this map the book explained |
| `grove-loop` | The loop: the task tree in grove's own vocabulary, the twelve verbs over it, and the driver — its lease, the mandate it composes, and the configuration it reads | `jj-workspace`, `keyed-launch`, `ordinal-fs-tree` | `run`, `LoopOutcome`, `DriverLease`, `TemplateSource` and `VERSION` — five of the seven names in the next table |
| `jj-workspace` | The version-control seam: resolve a Jujutsu workspace, refuse a working tree that is not one, take a path-scoped commit | none | `Workspace`, through the loop's re-export |
| `keyed-launch` | The runner: a configuration of key to complete command template, expanded into an argument vector, spawned directly and supervised | none | `reraise`, through the loop's re-export |
| `ordinal-fs-tree` | The tree store the task tree is kept in | none | Nothing. It is linked into the binary and reached only behind the call |
| `grove-llm` | The agent's binary: the twelve verbs *The surface* catalogued | `grove-loop`, `jj-workspace` | Nothing. Neither binary depends on the other |
| `book-validation` | The fragment and Markdown validator this book is proved by | none | Nothing at run time; two of the crate's `tests/` read it as a library |

Three facts in the table were asserted on earlier pages and are now checkable
against the manifests.

**The binary's reach is one edge.** *Orientation* read a manifest with one grove
dependency and said the loop was the reason the crate is short. The map shows
the same edge from the other side: nothing in the workspace depends on `grove`,
and `grove` depends on nothing but the loop. Every other package in the fourth
column is reached through that one edge or not at all.

**Two of the three domain-free crates are named, by one item each, and the
third by nothing.** `Workspace` is the seam's type and `reraise` is the
runner's function; both arrive through `grove-loop`'s crate root, which
re-exports them at the lines the next table gives. *Orientation* counted three
crates reached from the manifest and none named in it. The exact form of that
claim is the map's: the seam and the runner are named through the loop's root,
and the store — the tree library the task tree is kept in — is named by no
line of this crate. It is reached by the loop's own tree modules, behind the
call.

**The two binaries do not meet.** `grove-llm` takes two workspace dependencies,
the loop and the seam; `grove` takes the loop alone; and neither names the
other. The agent binary's manifest records that it once depended on `grove`, for
two concerns that were the driver's — the session-epoch admission every verb
passes through, and the launch-template check `root-init` and the leaf-writing
verbs make —
and that `loop-crate-driver-k22` moved both into the loop and removed the edge.
Since then the only thing the two binaries share is the crate beneath them,
which is where *The surface*'s one version constant lives.

<a id="the-seven-names"></a>
## Seven names, one check

*Orientation* stated the first mechanism's check in one sentence: every
`grove_loop::` path the two Rust files name is a `pub` item in that library's
root. The table performs it. The seven names are every distinct `grove_loop::`
path in `main.rs` and `cli.rs`; the second column is where each is named; the
third is the crate and module that declares it; the fourth is the line of
`crates/grove-loop/src/lib.rs` that publishes it. The reader is to take from it
that the check is exhaustive and closed: seven names, seven `pub` lines, and no
name that resolves through anything the library did not publish.

| Name | Where `crates/grove` names it | Declared in | Published by `lib.rs` at line |
|---|---|---|---|
| `Workspace` | `cli.rs` line 2; used at line 45 | `jj-workspace` | 81, `pub use jj_workspace::{Commit, Workspace}` |
| `DriverLease` | `cli.rs` line 2; used at line 46 | `grove-loop`, module `driver_lease` | 80 |
| `TemplateSource` | `cli.rs` line 2; used at line 47 | `grove-loop`, module `session_config` | 91 |
| `LoopOutcome` | `cli.rs` line 2; matched at lines 49 and 50 | `grove-loop`, module `loop_driver` | 88 |
| `VERSION` | `cli.rs` line 16 | `grove-loop`, the crate root | 73, where it is declared |
| `run` | `cli.rs` line 48; `main.rs` line 6, as a documentation link | `grove-loop`, module `loop_driver` | 88 |
| `reraise` | `cli.rs` line 50 | `keyed-launch` | 87, `pub use keyed_launch::reraise` |

Three of the five loop-owned names come from modules the root declares without
`pub` — `driver_lease` and `loop_driver` are private modules of the loop, and
`DriverLease`, `run` and `LoopOutcome` are reachable from this crate only
because line 80 or 88 re-exports them; `session_config` is a `pub mod`, so
`TemplateSource` has a second spelling the binary does not use, and `VERSION`
is the root's own. The check therefore has a negative side, and it was
measured rather than read. A scratch package outside the repository, depending
on `grove-loop` by path exactly as this crate does, was given one line naming
`DriverLease` through the module that declares it instead of through the root:

```console
$ cargo check
error[E0603]: module `driver_lease` is private
  --> src/main.rs:1:17
   |
 1 | use grove_loop::driver_lease::DriverLease;
   |                 ^^^^^^^^^^^^  ----------- struct `DriverLease` is not publicly re-exported
   |                 |
   |                 private module
```

The compiler's note locating `mod driver_lease;` at line 54 of the loop's root,
and the two closing lines naming the error code and the failed package, are
elided. The same scratch file's second line, naming the same type as
`grove_loop::DriverLease`, compiled. That is the whole of mechanism 1 as an
observation: an item the loop did not publish is refused at the boundary, by
the compiler, before any test runs, and the refusal names the private module.

<a id="the-modules-behind-the-call"></a>
## The modules behind the call

The five names the loop itself supplies come from four places in its module
tree: the crate root declares `VERSION`; `loop_driver` declares `run` and
`LoopOutcome`; `driver_lease` declares `DriverLease`; and `session_config`
declares `TemplateSource`. The root declares eleven modules beneath itself,
four of them public — `driver`, `prompt`, `session_config` and `verbs` — and
this crate names an item from one of the four and from two of the seven
private ones. The rest of the loop is reached by the call and by nothing else
in these 204 lines.

The table is the map of those modules, and of the one module each binary
has: the package, the module, and what it is responsible for. The reader is to
take from it which module the call reaches for each thing the four chapters
named — the task tree and its grammar, the lifecycle and the finish sentinel,
the completion channel, the lease and the epoch, the prompt, the configuration
— and that the map names each responsibility without explaining it. The one
row this book has explained is `grove`'s own `cli`, in the three chapters that
read its 137 lines; every other row is named here and explained in its own
crate's book.

| Package | Module | Responsibility |
|---|---|---|
| `grove-loop` | *the crate root* | The opening — `read`, `write`, `Reading`, `Writing` — which mirrors the store's one level up, so a caller can neither scaffold over a live grove nor read one that is not there. Plus `Reference`, `Selection`, and the crate's one opaque `Error`. |
| `grove-loop` | `verbs` | The twelve verbs a session invokes. A verb that reads takes a `Tree` and one that writes takes a `TreeWrite`, so the lock it needs is in its signature; a search that matched nothing answers the store's `Sought`; and every one returns the paths it wrote, because its caller writes the commit message by hand. |
| `grove-loop` | `task_name` | Grove's `ordinal_fs_tree::EntryName` — the whole seam onto the tree library, and the only name grammar grove has, handle included (`Slug`, `Kind`, `Outcome`, `Handle`, `Parts`, `TaskName`). |
| `grove-loop` | `task_tree`, `task_grow` | The reading and growing verbs expressed through the library: one snapshot per command, path construction, key prediction, and the cross-reference lint. |
| `grove-loop` | `tree_lifecycle` | The grove-only lifecycle around the tree: the terminal outcomes, the finish sentinel, and the grove's own creation through the store's vacancy. |
| `grove-loop` | `complete`, `driver` | The completion channel's token — written by one verb, read back by the loop — and the two tree operations the loop performs that no verb exposes. |
| `grove-loop` | `loop_driver` | **The loop**: `run(workspace, lease, templates)`, and `LoopOutcome`. Foreground iteration and selection; names the child-environment scrub list and the escalation's two graces and hands both to `keyed-launch`, which owns the spawn, the supervision and the kill. |
| `grove-loop` | `driver_lease` | Driver lease, session epoch, and ambient-session validation. Takes a resolved workspace and asks the seam for grove's namespace inside it; supplies the control directory each launch's channel is allocated in, the channel itself being `keyed-launch`'s. |
| `grove-loop` | `prompt` | The guaranteed core: the whole of `${prompt}` — the `grove-<kind>` load instruction, the runtime facts, grove's signalling contract — and the too-late test its contents are admitted by. Reads nothing and depends on no corpus. |
| `grove-loop` | `session_config` | Grove's side of launch configuration: the personal file's path, the four slots grove's templates are written against, the `TemplateSource` the loop re-reads on every iteration — twice, before and after the tree transition — and the delta: where it is searched, which candidate wins, and the refusal of a tracked one. The grammar, the validation and the expansion are `keyed-launch`'s. Asks the VCS seam whether a delta candidate is tracked; nothing else leaves the filesystem. |
| `grove` | `cli` | The human command surface, which selects nothing: parse, resolve the workspace, take the lease, call `grove_loop::run`. |
| `grove-llm` | `cli` | The deterministic agent command surface: argument parsing, the just-in-time presence rule, and rendering. Every verb is one `grove_loop::verbs::` call plus output. |

Two rows are the binaries, and they are the same shape: a clap surface plus a
call. The other ten are the loop, which is the whole of grove behind the call.
The three packages the loop depends on are not rows, because none of them has a
module grove reaches by name: each is a crate with an architecture of its own
and is named in the package map above by what it is.

<a id="the-boundary"></a>
## Where this book stops

**Everything behind `grove_loop::run` is named in this book and explained
nowhere in it.** That sentence is the boundary, and it is stated here once
rather than beside each thing the four chapters named. A page could not be
followed without naming the loop's parts — *Three steps* cannot read the
`match` without the outcome, the completion signal and the runner's
handler — so each page named what it needed and gave the minimum a reader
must hold. None of them
explained the thing named, and this page does not either.

The table is the boundary as a list. Its first column is what was named, its
second is where, and its third is what the naming page needed from it; the
reader is to take from it that every row was needed for a claim about this
crate, and that no row is a claim about the loop. The first row carries the
book's second citation of the task tree's glossary entry, at the place the
structure brief put it; *Orientation* carries the first.

| Named as | Where it is named | What the page needed it for |
|---|---|---|
| The [task tree](../../../CONTEXT.md#task-tree-scheme), and the walk that selects its first live leaf | [*The human's binary*](01-orientation.md#the-binary); [*Nothing left to select*](02-the-surface.md#nothing-to-select) | The first source on disk for a fact an argument would otherwise select |
| The personal configuration, its templates, and the untracked delta | [*Nothing left to select*](02-the-surface.md#nothing-to-select); [*The three steps*](03-three-steps.md#the-three-steps) | The second such source, and what `TemplateSource` locates rather than reads |
| The twelve verbs of the agent binary | [*The other binary*](02-the-surface.md#the-agent-surface) | The other half of the audience split |
| The completion signal, and the channel it is read from | [*One invocation*](01-orientation.md#one-invocation); [*A driver that was killed*](03-three-steps.md#the-signal-path) | How `Finished` and `Stopped` are told apart |
| The lifecycle transition, and the finish leaf | [*One foreground iteration*](03-three-steps.md#one-iteration) | The two writes of the driver's own, so the binary's one write — the lease — is not mistaken for the loop's |
| The session epoch | [*The three steps*](03-three-steps.md#the-three-steps); [*One foreground iteration*](03-three-steps.md#one-iteration) | What the lease writes beside its lock, and what an iteration activates |
| The runner's signal handler, and its escalation | [*A driver that was killed*](03-three-steps.md#the-signal-path) | Why `Interrupted` exists and why `reraise` is the runner's function |
| The mandate the loop composes | [*One resolution*](03-three-steps.md#one-resolution) | Why the loop needs the resolved workspace and not only the lease |
| The loop's one opaque error | [*A driver that was killed*](03-three-steps.md#the-signal-path); [*What `run` refuses*](03-three-steps.md#what-run-refuses) | What `?` on the call propagates, and what *anything the loop refuses* names |
| The methodology plugin | [*Two products*](01-orientation.md#two-products) | The referent of *grove-the-product*, and the product the binary does not install |

Each row is a subject of the crate that owns it, and the account of each is
that crate's own book. Three of those books do not exist at the corpus this
book is frozen against, and until each lands the description of that crate's
internals stays in `docs/ARCHITECTURE.md`, beside the decisions it records —
named here by path because the book's link contract admits two documents
outside it, the guide and the glossary, and that document is neither. What that
document no longer describes is the entry point, the two surfaces and the
module map: those are this book's, and each of its stripped sections opens with
a pointer to the page here that carries the description. The same holds for the
plugin: it is in no crate and therefore in no book, and *Orientation* named it
as one of two products and stopped.

<a id="the-test-applied-back"></a>
## The test, applied back

The outcome the book promised is a test a reader applies to an entry point of
their own: ask what is left for an argument to select, and then ask which of
three mechanisms holds the answer — a package boundary the compiler
enforces, a closure property a test asserts, or a convention a test
checks. *Proving a negative* closed the three with what each holds and where
each is proved. The
table lacked the criterion that makes a mechanism checkable rather than
believed: **for each mechanism, name the change that breaks it and the compiler
check or test that then fails.** The table answers it for each mechanism, and
the reader is to take from it that all three have an answer, and that the
answers are not equally strong.

| # | Mechanism | The change that breaks it | What fails | Where that was measured |
|---:|---|---|---|---|
| 1 | The entry point reaches only what the library publishes | A `use` of an item the loop did not re-export | The build: `E0603`, before any test runs | This page, *Seven names, one check* |
| 2 | The human surface has nothing left to select | One field on `Cli` | `the_human_command_surface_has_nothing_left_to_select`, printing the field's id | *Proving a negative*, the worked example |
| 3 | Everything the surface lists is described | A field with no doc comment, or with an empty one | `the_human_facing_binary_describes_every_option_it_lists`, printing the command path and the id | The same example, all three rows of its last table |

The three answers differ in strength. A compile error is the strongest: it
cannot be skipped, and nothing has to be run to see it.
A failing test is the next, and it has the limitation *Proving a negative*
named — a commit that deletes the test compiles — and one more the map makes
visible: the test reads the model this crate declares, so a selection that
arrived by any other route would pass it. There is no such route today, because
the compiler holds the only edge. Mechanism 3 is the weakest of the three on
this binary, and the reason is not its check but its subject: while mechanism 2
holds, the surface lists nothing, and a convention over an empty list is
vacuous. It can fail only after mechanism 2 is deliberately relaxed.

The test transfers, and the map shows the nearest entry point to take it to.
`grove-llm` is the other binary in the same workspace, over the same loop. The
table sets its three answers beside this crate's; the reader is to take from it
that they differ in two places and agree in the third, which is what shows the
test transfers rather than merely describing this binary.

| # | Mechanism | On `grove` | On `grove-llm` |
|---:|---|---|---|
| 1 | Where the package boundary holds | At the binary target's one edge, to the loop | At the package's edges to the two libraries it depends on — the loop and, as the map shows, the seam: the binary and its library together reach only what those two publish |
| 2 | What is asserted about the surface | A closure property: nothing to select | No closure property, since there are twelve things to select; in its place, flatness — `the_grove_llm_verb_surface_is_flat`, asserting that no verb has subcommands of its own — and the ten instructed verbs pinned as a complete set in the same test file |
| 3 | What checks the described-option convention | The walk in `cli.rs`'s test module | The same walk, in `crates/grove-llm/tests/help_surfaces.rs` |

The first difference is the library target. `grove-llm`'s crate root says why it
has one: a clap command tree is not something a spawned process can be asked
about, so the verb surface lives in the library where the crate's own tests can
inspect it, and the root states that this costs the guarantee nothing, because
the code the binary must not reimplement is in a different crate either way.
The second is the flatness *The surface* named, standing where a closure
property cannot be asserted. The third is the same answer on both, and *Proving
a negative* read why the walk exists twice.

Applied to an entry point outside this workspace, the test has three steps, in
this order.

1. **Name each thing the command line selects, and name the place on disk that
   already holds that fact.** *The surface*'s three-row table is the form. An
   argument with no such place is a selection the binary genuinely owns; one
   with such a place is a second source for one fact.
2. **For each mechanism, name the check that fails.** A compile error, a
   failing test, a measurement written beside the code, or a reading of it —
   and *nothing* is an answer, which means the entry point is thin by review.
3. **Say which mechanism is vacuous today, and what would make it non-vacuous.**
   On this binary that is mechanism 3, and the answer is a relaxed mechanism 2.
   An answer of that shape is what keeps a check that currently passes on an
   empty list from being read as a check that has found nothing.

<a id="the-closed-ledgers"></a>
## The closed ledgers

The book's two ledgers are complete, and the closure is mechanical rather than a
claim this page makes. A third account closes here too, and it is the one the
validator does not keep.

**Ownership.** Five top-level blocks over three source roots, every one of them
`resolved`. *Orientation* created the whole ownership table at the start, with
its own block resolved and the other four reserved by `defer` directives; each
later chapter replaced its own defer with an insert and turned its own row. No
`defer` directive remains anywhere in the book, and none may: `F003` reports
any defer at all in final mode, so *every reservation has become an insertion*
is a statement the validator refuses to let be false rather than one this page
asserts.

**Early use.** Five rows, every one `explained`, and every one declared in the
manifest as well as in the ledger. The structure brief fixed all five in
advance, and the order forced no more: each is a `grove-loop` item named by
*The surface*'s import line, or by *Orientation*'s manifest comment, before
*Three steps* explains what the binary does with it, and each row turned
`explained` in that chapter and in no other. Nothing named later needed a row,
because *Proving a negative* names only `Cli`, which it does not own but which
was read before it — a forward reference in the ordinary direction.

**Owned source.** 54 + 19 + 47 + 84 = 204 lines across four chapters, and 0 for
this one. The fifth row of that table exists to be zero: a chapter that owns no
source is the shape the structure brief chose for the assembly, and the total
is the 204 the root brief froze.

**Evidence.** Ten claims in this book are held by no test in the repository,
and each chapter said so where it made one — as *measured*, or as a reading.
What no chapter could do is say how many there are, because each saw only its
own. Applied to the book itself, the second part of the test identifies which
checks would fail if the binary's behaviour changed. The table lists the
claims; the fourth column is what holds each one instead, and it is the column
to read down, because the rows are not equally weak.

| # | The claim | Where it is stated | What holds it instead of a test |
|---:|---|---|---|
| 1 | A binary target that depends on the library cannot name a `pub(crate)` item, and one that includes the library's source as its own module can | [ch. 1](01-orientation.md#crate-not-a-bin) | measured on a scratch package with one library and one binary, in both shapes; two packages here build the depends-on shape — `grove-llm`, and `ordinal-fs-tree` under its `cli` feature — but neither names a private item, so no build here would go red if the first half were false, and nothing here builds the second shape over a library's own modules |
| 2 | `--help` and `--version` discover no repository and acquire no lease | [ch. 2](02-the-surface.md#worked-argv) | measured in a directory with no `.jj`; `cli_metadata_exposes_only_the_bare_entrypoint_and_writes_no_skill_directory` in `crates/grove/tests/lifecycle_cutover.rs` asserts that both succeed and asserts `--version`'s exact output, but runs inside a Jujutsu workspace, so *before the flow* is the measurement's alone |
| 3 | `grove --harness claude` prints the refusal shown and exits `2` | [ch. 2](02-the-surface.md#worked-argv) | measured; the same fixture asserts only that `grove do` fails, and asserts neither the text nor the status |
| 4 | `SIGHUP` to the driver ends with wait status `129`, after *interrupted by signal 1* | [ch. 3](03-three-steps.md#worked-run) | measured; `a_sigtermed_driver_stops_and_reaps_its_child` in `crates/grove/tests/loop_driver.rs` asserts the `SIGTERM` row alone |
| 5 | `SIGTERM` to the session's process, not the driver's, returns `Stopped` and exits `0` | [ch. 3](03-three-steps.md#worked-run) | measured; no fixture signals the session rather than the driver |
| 6 | Typed outside a Jujutsu workspace, the binary prints `Error: not a Jujutsu working tree` with the two repairs and exits `1` | [ch. 3](03-three-steps.md#worked-run) | the message is asserted by `crates/jj-workspace/tests/workspace.rs` against the seam's `resolve`; no `grove` fixture runs the binary outside a workspace, so the prefix and the status are measured |
| 7 | A second `grove` in the same tree is refused with *another Grove driver already owns* and exits `1` | [ch. 3](03-three-steps.md#worked-run) | `a_second_driver_refuses_before_tree_access_or_launch` in `crates/grove-loop/tests/driver_lease.rs` spawns a second driver against a held lease and asserts a failed status carrying *existing Grove driver must stop*; the `Error:` prefix and the status `1` are the binary's and are measured |
| 8 | `$HOME` unset stops at `TemplateSource::from_env` with a message naming the file it could not locate and a login shell | [ch. 3](03-three-steps.md#the-three-steps) | a reading of `crates/grove-loop/src/session_config.rs`; no fixture runs the binary or the loop with `$HOME` removed |
| 9 | One flag on `Cli` fails both tests with the messages quoted, renders the padded blank row, and reads the same before and after `build()` | [ch. 4](04-proving-a-negative.md#worked-assertion) | measured on a scratch copy outside the repository; by construction no test in the repository can hold it, since the flag does not exist |
| 10 | The two `undescribed` bodies differ only in two parameter names and one level of indentation | [ch. 4](04-proving-a-negative.md#twice) | a `diff` after dedenting; no check detects divergence between the copies |

The ten rows fall into three classes, and the table partitions them; the
reader is to take from it which class is a gap a test could close and which is
not.

| Rows | What holds the substance | Why no test in the repository holds the claim |
|---|---|---|
| 6, 7 | A test in another crate — the seam's, the loop's | The refusal is the seam's or the loop's; the `Error:` prefix and the status `1` are the standard library's handling of the `Err` that `main` returns, as *Three steps* read, and this crate adds nothing. This is the shape the first mechanism produces |
| 1, 9, 10 | A measurement on a shape the repository does not contain | A `[[bin]]` inside the loop, a `Cli` with a field, or two copies that have drifted: no test could take the measurement |
| 2, 3, 4, 5, 8 | A measurement written on the page | A test *could* take each measurement and none does, and no leaf in the tree is against any of the five |

The result is five claims about this binary's observable behaviour held only by
measurements written on the page. Mechanism 1's row of the table under *The
test, applied back* is the strongest form — a check the compiler performs
— and every row here falls short of it; saying by how much is what the
assembly owed
a reader who has just been taught to ask.

The [concept index](concept-index.md) and the [source index](source-index.md)
are the two lookup surfaces, and neither is part of the reading order. The
source index is the authoritative record of how the fragment graph reconstructs
each of the three files; the concept index is curated navigation into the
arguments, and makes no completeness claim.

<a id="final-verification"></a>
## Final verification

Three commands prove the book, and they prove different things. The first is
the only one that reads the corpus byte for byte.

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/overview --final --check all
valid: 3 files, 204 resolved lines, 0 deferred lines, final=true
```

`--final` is what makes this different from every scoped run the drafting
sessions made. In scoped mode a later chapter's range may be reserved by a
defer and counted as deferred rather than resolved; in final mode a defer is an
error, every source root must expand to its complete file, and the page
inventory must match the manifest exactly. 204 resolved and 0 deferred is the
whole corpus reconstructed.

```console
$ bash scripts/check.sh
...
=== book-check
  book-check docs/walkthroughs/jj-workspace
valid: 4 files, 752 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/ordinal-fs-tree
valid: 17 files, 8720 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/overview
valid: 3 files, 204 resolved lines, 0 deferred lines, final=true
  3 book(s) checked, 0 failing
  ✓ book-check

check: all 8 principal checks pass
```

The second command is the repository-wide gate. It runs
`book-check --final --check all` over every book root under
`docs/walkthroughs/` by discovery rather than from a list, which is why
this book has been inside the gate since *Orientation* created its directory
and why every drafting session but this one left the script red on
`book-check` alone. It also runs the repository's own tests, and three of those
cover this book without naming it:
`every_repository_markdown_reference_resolves`
sweeps every Markdown file in the repository;
`every_book_root_has_a_documentation_ownership_row` fails a book root with no
row in the *Documentation ownership* table of `docs/ARCHITECTURE.md`; and the
corpus-exception inventory in `crates/grove/tests/corpus_exception_inventory.rs`
requires this manifest's empty exception set to match the specification's
tables. All three are in this crate's `tests/` directory, which *Orientation*
read the manifest's reason for.

```console
$ cargo test --locked -p grove
     Running unittests src/main.rs (target/debug/deps/grove-aeff5af423bb6011)
running 2 tests
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/commit_guidance.rs (target/debug/deps/commit_guidance-70bcf3e4b6d3ceef)
running 7 tests
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/corpus_exception_inventory.rs (target/debug/deps/corpus_exception_inventory-dd14c2adc7e7076b)
running 5 tests
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
     Running tests/env_hygiene.rs (target/debug/deps/env_hygiene-6efe5c881b3f3e06)
running 4 tests
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/lifecycle_cutover.rs (target/debug/deps/lifecycle_cutover-b3bc33810dbda6c6)
running 17 tests
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.70s
     Running tests/loop_driver.rs (target/debug/deps/loop_driver-51165283bf2e9c2a)
running 11 tests
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 39.78s
     Running tests/plugin_fallback.rs (target/debug/deps/plugin_fallback-0b2e5ea0375c12cf)
running 6 tests
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
     Running tests/reference_navigation.rs (target/debug/deps/reference_navigation-d433b90b44c9893c)
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.51s
     Running tests/retire_guidance.rs (target/debug/deps/retire_guidance-61566f37158e9444)
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/user_guide_coverage.rs (target/debug/deps/user_guide_coverage-f5642e4f91d65252)
running 4 tests
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The third command runs the crate's own suite, whose tests supply evidence this
book cites and does not reproduce. The per-test `ok` lines and the blank lines
between blocks are elided; one line a fixture's `jj` printed while the loop
tests ran is elided with them. The two unit tests are the closure and convention
tests *Proving a negative* read, and they are the only tests inside the corpus;
the sixty-seven
integration tests under `tests/` are outside it by design — *Orientation* read
the manifest's reason for their being here — and every claim in the four
chapters that names a test names one that runs in this suite, or in the crate
the evidence table says it runs in.

The book is complete: three roots, 204 lines, five chapters, two lookup
surfaces, zero deferred ranges. What it argued is that one binary is thin and
that three different mechanisms hold it so, and what it leaves the reader with
is the criterion that distinguishes a mechanism from a belief — which check
fails — applied to the binary, the other binary beside it, and the book
itself.

[Previous: Proving a negative](04-proving-a-negative.md) | [Contents](README.md)

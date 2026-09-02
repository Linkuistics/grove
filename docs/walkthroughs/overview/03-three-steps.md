# Three steps
<!-- book-page id="three-steps" slice="one-call" order="3" -->
[Previous: The surface](02-the-surface.md) | [Contents](README.md) | [Next: Proving a negative](04-proving-a-negative.md)

<a id="one-call"></a>
## Resolve, lease, run

The grammar has been read and it selects nothing, so what `run` does with the
`Cli` it parses is the whole of the binary's behaviour, and it is the parse,
four more statements, and a `match`. This chapter owns the forty-seven lines that perform
them: the thirteen-line entry point, whose module documentation is the
chapter's argument in miniature, and lines 20 to 53 of `crates/grove/src/cli.rs`,
where the working tree is resolved, the lease is taken, the loop is called, and
the loop's answer is turned into an exit. The thesis is `main.rs`'s own: each
step before the call is something the loop cannot do for itself. The loop must
not read the command line, because launch policy has one home and it is not
there; it is handed the working tree, because the lease had to resolve one
first and a second resolution could only disagree; and it is handed the lease,
because a loop that could take its own would also be free to run without one.

The actor on this page is `grove::cli::run`. Its input is the parsed `Cli` and
the process's environment — the working directory, `$HOME`, and any signal the
process is sent while it runs. Its output is a process exit, and the invariant
it establishes is that the exit says truthfully how the loop ended: every
ending the loop was designed to reach exits `0`, a refusal by any step exits
`1` with the refusal printed, and a driver that was killed dies of the signal it
was sent. That last case is the chapter's centre of gravity. It is the most
argued claim in the corpus, a reader would not guess it, and it is what the
`match` at the end of `run` exists for.

**One premise, stated here and not derived again.** A process that is killed
by a signal has no exit code. Its parent's wait status records the signal
instead, and a shell renders that status as `128 + N`, where `N` is the
signal's number: `SIGTERM` is `15`, so a driver that dies of it is reported as
`143`. An exit code cannot express *was signalled*, which is the reason the
last line of `run` re-raises a signal rather than mapping it to a number. Every
`128 + N` on this page is that convention, named rather than computed.

The page reads the entry point first, then the four fragments of `run` in the
order their argument runs — the seam that resolves the tree once, the three
steps that use it, the call and its two endings — then carries the book's
invocation through all of it at full resolution, and closes with the shape of
one loop iteration as seen from the caller, which is where the page stops
explaining and starts naming.

<a id="the-block"></a>
## Lines 20 to 53, in five fragments

The larger of the two blocks is `run`: its doc comment, split along its own
three paragraphs, and its body, split at the `match`. The composite below is
the whole of it in file order, and each child is read on this page beside the
claim it carries.

<!-- fragment «surface-resolve-lease-run» owner="one-call" source="crates/grove/src/cli.rs" lines="20-53" parent="source-command-surface" -->
<!-- insert «run-seam-doc» -->
<!-- insert «run-signal-doc» -->
<!-- insert «run-errors-doc» -->
<!-- insert «run-three-steps» -->
<!-- insert «run-call-and-endings» -->
<!-- /fragment -->

The block begins at line 20, the blank line before the doc comment, and ends at
line 53, the blank line after the function's closing brace; *Proving a
negative*'s block begins at the `#[cfg(test)]` attribute on line 54. Blank lines
lead the fragment that follows them throughout this book, and line 53 is the one
exception: the block boundary is fixed at 53, so that blank line closes the last
fragment here rather than opening the test module.

<a id="the-entry-point"></a>
## The entry point, and the argument in miniature

`crates/grove/src/main.rs` is thirteen lines and is owned here rather than in
*Orientation* because seven of them are the module documentation, and that
documentation is this chapter's thesis stated before the code that bears it out.
The comment names three steps — parse the empty command line, resolve the
working tree, take the lease — and says of each that the loop cannot do it for
itself. It then names the call, `grove_loop::run`, and says that everything
after it is behind the call, citing decision 9 of
`docs/specs/module-decomposition.md`, which is the record that put the whole
loop behind one function. The reader needs the claim, not the record: nothing
before the call chooses, and nothing after it is this crate's.

<!-- fragment «entry-point-three-steps» owner="one-call" source="crates/grove/src/main.rs" lines="1-13" parent="source-entry-point" -->
<!-- insert «entry-point-module-doc» -->
<!-- insert «entry-point-module-and-main» -->
<!-- /fragment -->

The first fragment is the documentation. Its count is worth holding against
the other two counts this corpus offers, because the three disagree and the
page should say so rather than pick one. This comment's three are parse,
resolve and lease. `run`'s own title, read below, is *resolve, lease, run* —
the call counted, the parse not. And the body of `run` has five statements
before its `match`: the parse, `current_dir`, the resolve, the lease, and
`TemplateSource::from_env`, which neither comment counts. The structural fact
is that four things happen before the loop is entered and one of them, locating
the configuration, is handed to the loop as a value rather than performed
there: the loop takes a `TemplateSource` so that the location is its caller's to
name, and this binary names `$HOME`'s — the fixtures that drive it as a process
set `$HOME` to a temporary directory. In the invocation this page carries, the
three steps the comment names are the first three lines of the trace below the
call. The comment is part of the frozen corpus and is reproduced as written.

<!-- fragment «entry-point-module-doc» owner="one-call" source="crates/grove/src/main.rs" lines="1-7" parent="entry-point-three-steps" -->
````rust
//! The human's binary: bare `grove`, and nothing else.
//!
//! Three steps, and every one of them is something the loop cannot do for
//! itself — parse the human's (empty) command line, resolve the working tree it
//! was invoked in, and take the one-driver lease over it. The loop is
//! [`grove_loop::run`], and everything after the call below is behind it
//! (`docs/specs/module-decomposition.md`, decision 9).
````
<!-- /fragment -->

The second fragment is the code, and it is one declaration and one call. The
`mod cli;` line is the whole of the module structure: `cli` is a private module
of the binary target, which is why its clap model is reachable only from inside
this crate, as *Orientation* argued and *Proving a negative* relies on. `main`
returns `anyhow::Result<()>` and delegates to `cli::run`, so a refusal from any
step is printed by the standard library's handling of an `Err` from `main` —
the `Error:` prefix, then the error's debug rendering with its cause chain —
and the process exits `1`. There is no line here that decides anything; the
function exists so that the binary has an entry point and `cli.rs` can be a
module with tests. In the carried invocation it is the frame around every
line: the `Ok(())` that becomes exit `0`, and the `Err` that becomes `Error:`
and exit `1` in the two refusals shown below.

<!-- fragment «entry-point-module-and-main» owner="one-call" source="crates/grove/src/main.rs" lines="8-13" parent="entry-point-three-steps" -->
````rust

mod cli;

fn main() -> anyhow::Result<()> {
    cli::run()
}
````
<!-- /fragment -->

<a id="one-resolution"></a>
## One resolution, handed to both

The first paragraph of `run`'s documentation is about a seam, and the seam is
the reason the function has the shape it has. The working tree is resolved
**here**, once, and the resolved value is passed to two consumers: the lease,
which needs the tree's root to lock it, and the loop, which takes the tree root
back from the lease it is handed and needs the workspace for the rest — the main
repository that `${repo}` expands to, and the version control it states in the
mandate it composes. The comment names what this replaced. Before
`loop-crate-driver-k22`, the task that moved the driver into the loop crate, the
lease resolved a path of its own and held the answer, so a caller that also needed
the workspace derived the same fact a second time and had no way to see whether
the two derivations agreed. The design record is
`docs/adr/one-live-driver-per-working-tree.md`, which states the rule as *the
lease is handed a resolved workspace; it does not resolve one*, and the loop's
own module documentation says the same from its side: its first line, owning
the workspace lease, happens in the caller, and the loop is what follows it.

The invariant this buys is the one the comment states as a negative — no two
derivations of one fact — and it is held by the type. `Workspace` cannot be
constructed except by `resolve`, so holding one is proof that the precondition
passed, and every consumer that takes a `&Workspace` never re-asks. The lease
asks the resolved workspace for grove's control directory inside its `.jj/`;
the loop takes the lease's root and the workspace's main repository, and the
two paths a launch template expands to are therefore the very paths the delta
configuration was searched at. This is the through-line the book carries: a
decision in `cli.rs`, resolving once, is what makes a property in the loop
hold.

<!-- fragment «run-seam-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="20-28" parent="surface-resolve-lease-run" -->
````rust

/// Resolve, lease, run.
///
/// The workspace is resolved **here**, once, and handed to both the lease and
/// the loop. That is the shape `loop-crate-driver-k22` gave the seam: the lease
/// used to resolve a path itself and hold the answer, so a caller that also
/// needed a workspace had two derivations of one fact and no way to see that
/// they agreed (`docs/adr/one-live-driver-per-working-tree.md`).
///
````
<!-- /fragment -->

<a id="the-three-steps"></a>
## The three steps, statement by statement

The body's first five statements are read here, and the four types that *The
surface* named at its import line are explained where each is used. The
fragment is the signature through the `TemplateSource` line; the `match` that
follows is the next section's.

The parse is line 43, and its value is bound to `_cli` and never read, because
a `Cli` carries nothing. What the statement does is exit the process for
`--help`, `--version` and any unknown argument, as *The surface* measured, and
return for the bare vector. Line 44 is the input to the resolve: the working
directory the process was started in, from the standard library, which fails
only if that directory has been removed or cannot be read. That is an
environmental failure rather than a refusal, and it is the one error on this
page that no grove type produces.

**`Workspace::resolve`** at line 45 is the resolve, and `Workspace` is the type
*The surface* held as *a resolved jj working tree, produced once and handed to
both the lease and the loop*. It is not the loop's type: it is the version
control seam's, `jj-workspace`, published through `grove-loop`'s crate root as
a `pub use` so that this binary can name it without a dependency on the seam —
which is the first mechanism at work, and the reason the manifest *Orientation*
read lists one grove dependency. What `resolve` does is a filesystem walk: from
the given directory up through its ancestors to the first one holding a `.jj`
directory, whether that checkout is native, colocated or a secondary
workspace. The walk invokes no repository discovery, so no environment variable
and no shared store can redirect it, and it canonicalises the root, so a symlink
or a relative alias of one workspace resolves to the same value. The value also
carries the main repository — the root itself for a native or colocated
checkout, and for a secondary workspace the default workspace's root, which the
seam asks `jj` for rather than reading the link itself, so a `jj` missing from
`PATH` is a refusal at this line too. A directory with no `.jj` anywhere above it is refused with *not a Jujutsu working
tree*, the path the walk started from, and the two `jj git init` commands that
would fix it; nothing has been created or changed when that is printed. This is
the [stated VCS](../../../CONTEXT.md#stated-vcs): the working tree's version
control is resolved here, before any session exists, and the loop states the
result in each mandate rather than letting a session guess it from a `.git`.

**`DriverLease::acquire`** at line 46 is the lease, and `DriverLease` is the type
held as *the one-driver-per-working-tree claim, taken for the life of the
process*. It takes the resolved workspace and asks it for grove's control
directory — `.jj/grove/` inside that exact workspace, untracked, created if
absent — and opens `driver.lease` there with an exclusive, non-blocking
advisory lock. The lease also opens and holds the working tree root's own
descriptor, so that before each transition and each launch it can check that
the tree it owns is still the one it locked; and a second `grove` in the same
tree fails at once with *another Grove driver already owns* the canonical root; it does not
queue, because two drivers would issue two mandates for the same leaf. The
lease also writes a fresh random nonce and an inactive session epoch, which are
the loop's business and not this page's. What this page needs is the lifetime:
the lock is the kernel's, so it is released on return, on panic and on process
death alike, and *for the life of the process* is exact rather than
approximate. The [driver lease](../../../CONTEXT.md#driver-lease) is grove's
vocabulary for this, and the
[user guide's account of one driver per working tree](../../USAGE.md#usage-driver-lease)
shows the refusal as a transcript.

**`TemplateSource::from_env`** at line 47 is the last statement before the call,
and `TemplateSource` is the type held as *where launch policy is read from*. It
performs no read. It takes `$HOME` from the environment and records it, so that
the loop can locate `~/.config/grove/config.kdl` — and it fails only when
`$HOME` is unset, with a message naming the file it could not locate and the
login shell that would set it. The loop then reads the file itself, on every
iteration, and it reads it twice: once before the tree's lifecycle transition,
which validates the personal file in full before anything is mutated and is the
document the loop asks whether `finish` is configured against when no live leaf
is left and it must write that leaf itself, and once after selecting the leaf,
to expand the selected kind's template from the document as it stands. Each read lays at most one untracked `.grove.kdl` delta
over the personal file. A loaded configuration handed in once could express
neither read, which is why the argument is a source rather than a snapshot. The
early-use ledger's statement for this type now reads *twice, before and after
the tree transition*; *The surface* stated it without a count, and the count is
this page's.

<!-- fragment «run-three-steps» owner="one-call" source="crates/grove/src/cli.rs" lines="42-47" parent="surface-resolve-lease-run" -->
````rust
pub fn run() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    let cwd = std::env::current_dir()?;
    let workspace = Workspace::resolve(&cwd)?;
    let lease = DriverLease::acquire(&workspace)?;
    let templates = TemplateSource::from_env()?;
````
<!-- /fragment -->

The three types resolve as names for the reason *The surface* gave: each is a
`pub` re-export in `grove-loop`'s crate root, and a fourth name that library
had not published would not compile. The lease is moved into the call on the
next line and the other two are borrowed, and the difference is deliberate: the
loop drops the lease when it returns, so ownership of the working tree ends
exactly when the loop that justified it ends, and a caller that kept the lease
could go on owning a tree it was no longer driving.

<a id="the-signal-path"></a>
## A driver that was killed does not exit 0

The second paragraph of the documentation is the claim this chapter is for, and
it is read before the code that implements it because the code is one `match`
arm and the argument is eight lines. The loop returns *why* it stopped. One of
the reasons is that this process — the driver, not the session — was sent
`SIGTERM` or `SIGHUP` while a grove was running. Every other reason is an
outcome the loop was designed to reach, and the process exits cleanly on it.
That reason is the loop being taken away, and the only way to say so through a
wait status is to die of the same signal after the cleanup. Whoever started
`grove` — the comment names a systemd unit, a `timeout(1)` and a shell `wait`
— then reads `128 + N` instead of success, and can tell an interrupted grove
from a finished one.

The alternative the comment rejects is the one every process that catches
`SIGTERM` is tempted by: clean up, then exit `0`. A driver that did that would
tell its own parent that a grove finished, and a unit that restarts on failure
would not restart it. The cost of the alternative is not visible at the driver;
it is visible one process up, which is why the comment addresses that process.

<!-- fragment «run-signal-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="29-37" parent="surface-resolve-lease-run" -->
````rust
/// **A driver that was killed does not exit 0.** The loop returns *why* it
/// stopped, and one of the reasons is that this process was sent SIGTERM or
/// SIGHUP mid-grove. Every other reason is an outcome the loop was designed to
/// reach and exits cleanly; that one is the loop being taken away, and the only
/// way to say so through a wait status is to die of the same signal after the
/// cleanup — the lease is dropped by the `run` above, and the session was
/// already reaped by the runner. Whoever started `grove` — a systemd unit, a
/// `timeout(1)`, a shell `wait` — then reads `128 + N` instead of success.
///
````
<!-- /fragment -->

The code is the call and a `match` over its answer, and the answer is the type
*The surface* held as *why the loop stopped — the value that decides whether
this process exits 0 or dies of a signal*. **`LoopOutcome`** has three variants,
and the `match` groups them two and one. `Finished` is the grove completing: a
session signalled `complete --done` — the teardown session's last act, after
it has committed the deletion of `.grove/` — and the loop printed *grove
finished — loop complete*. `Stopped` is a session ending without
a completion signal — the human typed `/exit` or Ctrl-C into the session, or it
crashed — and the loop printed that it stopped, with a second line naming the
kind, the command and the configuration file when the session's status was not
success; a later `grove` in the same tree
resumes, because the loop holds no state and re-derives its position from the
task tree. Both are outcomes the loop was designed to reach, both are mapped to
`Ok(())`, and the process exits `0`. `Interrupted` carries a signal number, and
it is the driver itself having been sent `SIGTERM` or `SIGHUP`: the runner
inside the loop installs a handler for exactly those two, forwards the signal
it was sent to the session's whole process group, escalates to `SIGKILL` if the
session ignores it, reaps it, and reports the launch as interrupted; a signal that arrives between sessions, with no
launch to report against, is collected at the top of the next iteration
instead, so the driver stops rather than starting a session it is about to
kill. The completion signal the first two variants are read from is the
[loop control channel](../../../CONTEXT.md#loop-control-channel): a per-launch
file the driver watches while the session runs, whose appearance ends the
session and whose content says relaunch or done.

**`grove_loop::run`** is the call, and it is the symbol *Orientation* held as *the loop's single entry point; everything the binary does after its three
steps is behind this call*. Its three arguments are the three values this page
has produced, and the loop's own documentation says of them that they are the
three things a loop cannot derive for itself. It returns a `Result` whose error
is the loop's one opaque error type and whose success is the outcome above.
Its first act is to ignore `SIGINT` in the driver, so that a Ctrl-C typed while
the driver is between sessions — transitioning the tree, selecting a leaf,
expanding a template — does not kill the loop; while a session runs it owns the
terminal in its own right and Ctrl-C reaches the session, never the driver.
The
[user guide's account of what happens in a session](../../USAGE.md#usage-session-lifecycle)
is the reader's view of one iteration from outside; what one iteration does is
the last section of this page.

**`grove_loop::reraise`** is the third arm, and it is the one line of the binary
that does not return. It is not the loop's function either: it belongs to the
runner, `keyed-launch`, and is re-exported through `grove-loop`'s root with a
comment saying why — the runner installed the signal handler, so the runner is
the crate that knows how to undo it, and what stays the binary's is *whether*
to die of the signal, which is a statement about this process's own exit
status. What it does is flush both output streams, restore the signal's default
disposition, unblock it in case the handler left it masked, and raise it
against this process; its return type is `!`, and the `std::process::exit(128 +
N)` after the raise is a guard against a caller's defect — a signal whose
default action does not terminate, which no signal a driver is interrupted by is
— rather than a path this binary takes. The order in the comment is the order
the code produces, read from the code: the loop returned, so the session has
already been reaped by the runner and the lease has already been dropped by
`run`, and only then does the driver die. What the fixture proves is the death
and its report: `a_sigtermed_driver_stops_and_reaps_its_child` in
`crates/grove/tests/loop_driver.rs` spawns this binary against a temporary tree
with a session that never signals, sends the driver `SIGTERM` mid-session, and
asserts that the wait status carries the signal `SIGTERM` and no exit code —
the fixture's own words are that there is no exit code that means *killed*,
which is why the signal is re-raised rather than mapped to one — and that the
driver's diagnostics contain *interrupted by signal 15*. `SIGHUP` takes the
same path through the same handler and is asserted by no fixture; the worked
example below shows it measured.

<!-- fragment «run-call-and-endings» owner="one-call" source="crates/grove/src/cli.rs" lines="48-53" parent="surface-resolve-lease-run" -->
````rust
    match grove_loop::run(&workspace, lease, &templates)? {
        LoopOutcome::Finished | LoopOutcome::Stopped => Ok(()),
        LoopOutcome::Interrupted(signal) => grove_loop::reraise(signal),
    }
}

````
<!-- /fragment -->

Three actors decide the exit, and the `match` is where they meet. `clap` decides
`0` for `--help` and `--version` and `2` for an unknown argument, before this
function's second statement. `main` decides `1` for an `Err` from any of the
five statements or from the loop. And `reraise` decides `128 + N`, after the
loop has returned. The `match` itself decides nothing: it routes the loop's
verdict to the actor that can express it.

<a id="worked-run"></a>
## Worked example: one invocation, at full resolution

The invocation is the one *Orientation* carries at low resolution and *The
surface* ran as an argument vector: a Jujutsu workspace at `/work/atlas/` holding
a grove with one live leaf, `01-impl--rate-limit-k3.md`, and `grove` typed in
`crates/gateway/src/`, with `~/.config/grove/config.kdl` mapping `impl` to
`claude --add-dir ${repo} ${prompt}`. This section runs it through the five
statements and the `match` above, with the value each produces, and follows it
to both endings. The values on the left are the binary's; the lines indented
under the call are the loop's, printed to stderr, and are what a reader watching
the terminal sees.

```text
$ grove
  Cli::parse()                     argv is ["grove"]; returns Cli {}
  current_dir()                    /work/atlas/crates/gateway/src
  Workspace::resolve(&cwd)         walks up: src, gateway, crates, atlas — .jj/ is
                                   there; root /work/atlas, main repo /work/atlas
  DriverLease::acquire(&workspace) creates /work/atlas/.jj/grove/ if absent,
                                   locks /work/atlas/.jj/grove/driver.lease; this
                                   process is the tree's one driver until it exits
  TemplateSource::from_env()       $HOME is /home/you; nothing is read yet
  grove_loop::run(&workspace, lease, &templates)
      grove: launching impl with configured "claude" — rate-limit-k3
      … the session works, commits, and runs grove-llm complete; the loop
        relaunches for the next live leaf, and so on until none is left, when
        the teardown session it launches signals that the grove itself is done:
      grove: grove finished — loop complete.
  -> LoopOutcome::Finished; the match returns Ok(()); the lease was dropped
     when run returned; main returns Ok(()); the process exits 0
```

That is the first ending, and it is *Orientation*'s trace with the binary's
half filled in. Two things are now visible that the low-resolution trace could
not show. The lease is released by the return from `run`, not by anything in
this crate: the `match` receives an outcome and the lease is already gone. And
the loop printed every line; between the `run` call and the exit the binary
prints nothing of its own, on either ending.

The second ending is the same invocation with the driver killed from outside,
mid-session, and it is the case the `match`'s third arm exists for. The
transcript is two shells, because the status has to be read where `grove`
exits rather than where the signal is sent.

```console
# the shell running the loop
$ grove
grove: launching impl with configured "claude" — rate-limit-k3
grove: interrupted by signal 15 — stopping the loop.
$ echo $?
143

# a second shell
$ kill -TERM "$(pgrep -x grove)"
$ echo $?
0
```

The `kill`'s own `$?` is `0` because the signal was delivered; only the wait
status of the `grove` process carries `128 + 15`. Between the two printed lines
the runner forwarded `SIGTERM` to the session's process group and reaped it;
the loop restored the terminal, closed the session's epoch and channel, and
returned `Interrupted(15)`; `run` dropped the lease; and `reraise` flushed the
streams and died of the signal. The table
carries the three endings the loop can report and the exit each reaches, and it
is the `match` read as a mapping rather than as code.

| Outcome the loop returns | What happened | Arm of the `match` | What the parent reads |
|---|---|---|---|
| `Finished` | A session signalled `complete --done`; the grove is finished | `Ok(())` | `0` |
| `Stopped` | A session ended with no completion signal: `/exit`, Ctrl-C into the session, or a crash | `Ok(())` | `0`; the next `grove` resumes |
| `Interrupted(15)` | The driver was sent `SIGTERM` mid-grove | `reraise(15)` | `143` |
| `Interrupted(1)` | The driver was sent `SIGHUP` mid-grove — a terminal closing | `reraise(1)` | `129` |

The last two rows are the same path with a different number, and both are
measured rather than reconstructed: the built binary at the frozen corpus,
driven against a temporary Jujutsu workspace with a session that never signals,
was sent each signal and reported `143` and `129` respectively as the wait
status of the `grove` process, after printing *interrupted by signal 15* and
*interrupted by signal 1*. The `SIGTERM` row is also the one the fixture
asserts. The guide's transcript under *Stopping the loop* shows the `143` case
against a different leaf and is the same measurement. The same measurement
also fixes what *the driver, not the session* means in the second row: a
`SIGTERM` sent to the session's process rather than the driver's is a session
ending without a completion signal, and the driver reports it as such — *status
signal: 15 (SIGTERM)*, then the second line naming the failed kind — returns
`Stopped`, and exits `0`, because nothing was taken away from the loop.

The two error endings `run`'s documentation names are this trace stopping
earlier, and each names the actor that stopped it. Typed in a directory with no
`.jj` above it, the trace ends at the third statement:

```console
$ grove
Error: not a Jujutsu working tree
  looked for a `.jj` directory at and above: /tmp/scratch

Make the tree jj-enabled and rerun:
      jj git init --colocate     # an existing Git repository, history kept
      jj git init                # no repository here yet

Nothing was created or changed.
$ echo $?
1
```

Typed in `/work/atlas` while the first invocation above is still running, it
ends at the fourth:

```console
$ grove
Error: another Grove driver already owns /work/atlas; the existing Grove driver must stop before this one can start
$ echo $?
1
```

Both are refusals, not failures: a precondition was checked and found false,
the message names what would make it true, and nothing was created or changed
— the second one has not even created a control directory, because the
directory was already there. The `Error:` prefix is the standard library's,
printing the `Err` that `main` returned, and the `1` is its exit status for
that case. A third early stop, `$HOME` unset, ends at the fifth statement with
the same prefix and status and a message naming the file it could not locate;
it is an environmental failure rather than a refusal, and the message says to
run `grove` from a login shell.

<a id="what-run-refuses"></a>
## What `run` refuses

The last paragraph of the documentation is the `# Errors` section, and it names
the two refusals just shown and then everything else in one clause. That
clause is honest about where this crate stops: *anything the loop refuses* is
every error the loop's own documentation lists, and this page names the class
and explains none of it. The list is not all refusals, and the page labels
what the comment does not: a configuration that does not load or does not cover
the selected kind is a refusal, with a precondition the human can make true; a
working tree replaced underneath the lease it holds, or a session that could
not be spawned, is an environmental failure that no edit to the tree or the
configuration would have prevented. Each reaches `main` as the same `Err`,
prints the same way, and exits `1`; what distinguishes them is the message,
and the messages are the loop's. The
fragment is the section as written, and it ends at line 41, the line before the
signature.

<!-- fragment «run-errors-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="38-41" parent="surface-resolve-lease-run" -->
````rust
/// # Errors
///
/// A working tree that is not a jj workspace, a lease another driver holds, or
/// anything the loop refuses.
````
<!-- /fragment -->

<a id="one-iteration"></a>
## One foreground iteration, from the caller's side

Everything behind the call is another book's, and this page has named what a
reader needs to follow the `match` — the outcome, the completion signal, the
runner's signal handler — without explaining any of it. What this section adds
is the shape of one iteration as the caller can see it, because the three
arguments and the returned outcome are the whole of the loop's contract with
this crate, and the shape is what makes the two endings above intelligible as
one loop rather than two behaviours.

An iteration begins by collecting any signal that arrived while no session was
running, revalidating the lease, and reading the configuration; it performs the
one lifecycle transition a tree may need and selects the next live leaf, or
materialises the finish leaf when none is left; it reads the configuration
again and expands the selected kind's template; it revalidates the lease once
more, allocates a fresh completion channel, activates the session epoch, spawns
the template's argument vector directly as a foreground child, and watches both
the child and the channel; and when the child has been reaped it invalidates
the epoch, reads the channel and decides — relaunch, and the loop continues
with fresh context; done, and it returns `Finished`; absent, and it returns
`Stopped`. A signal to the driver during the session is what turns the reap
into `Interrupted`. The only durable writes of the driver's own are the two the
transition and the selection can make — a root brief and a first
`requirements` leaf on a tree with no `.grove/`, and the `finish` leaf when no
live leaf remains; the epoch and the channel an iteration writes are
coordination that means nothing once the lock behind them is released, as the
lease the binary wrote before the loop began is.

Nothing in that shape is decided by this crate. The binary hands the loop a
resolved tree, a held lease and the location of launch policy, and takes back
one word about how it ended; the whole of the runtime flow lives behind the
call, and *What the call reaches* names the modules it lives in.

[Previous: The surface](02-the-surface.md) | [Contents](README.md) | [Next: Proving a negative](04-proving-a-negative.md)

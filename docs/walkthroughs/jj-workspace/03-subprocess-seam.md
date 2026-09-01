# The subprocess seam
<!-- book-page id="subprocess-seam" slice="nothing-ambient" order="3" -->
[Previous: The gate](02-the-gate.md) | [Contents](README.md)

<a id="nothing-ambient"></a>
## Nothing ambient chooses the repository

The third refusal is that the crate declines to let anything ambient decide
which repository a command operates on. The working directory chooses, and it is
set explicitly on every child. Nothing is read from the process environment to
answer that question, and four variables that could answer it behind the crate's
back are removed from the child before it starts.

The interesting word is *seam*. There are four places in this crate where jj is
spawned — one in the gate, and three in the operations *Scope and commit* owns —
and none of them builds a `Command`. All four call one of two functions in this
file, and those two share a single private builder. That is what makes the hygiene a **property of the
crate** rather than a habit each call site has to remember. A rule enforced at
call sites is enforced only at the call sites that exist: it is discharged by
review, it is re-discharged every time a fifth spawn is added, and the test that
proves it can only be written against the call sites someone remembered to
audit. A rule enforced at a seam is discharged once, by construction, and a
fifth call site inherits it by having nowhere else to go.

The alternative was to build each `Command` where it is needed and keep the
environment scrub as a checklist. It would have cost four copies of five lines,
and it would have cost the one test in the crate that asserts the property any
generality at all.
`resolution_ignores_repository_selection_and_temporary_directory_environment`
(`crates/jj-workspace/tests/environment.rs`) exercises exactly one of the four
call sites — the gate's — and that is all a test can ever do, because a test can
only reach a call site that exists. What makes its green result a statement about
all four is not the test: it is that there is one builder, so the other three
inherit the property by construction. Under the checklist alternative the same
test would prove one call site and nothing else, and the gap would be invisible
until the fifth spawn was written by someone who had not read the checklist.

<a id="the-premise"></a>
## The premise: jj asks the working directory

This chapter's argument rests on how jj chooses a repository, and that behaviour
is stated here once. jj has no repository-selecting environment variable. It
takes the repository from the working directory, walking up, unless it is given
an explicit path: "By default, Jujutsu searches for the closest `.jj/` directory
in an ancestor of the current working directory"
([jj CLI reference, `-R/--repository`](https://docs.jj-vcs.dev/latest/cli-reference/)).
Setting `current_dir` on the child is therefore not one way of selecting the
repository among several — it is the whole mechanism, and there is no ambient
override of it to defend against.

That is what makes the crate's environment scrub asymmetric, and the asymmetry
is the chapter's most easily misread line. The four variables removed are Git's,
not jj's. **There is no `JJ_*` counterpart to remove**, and the absence is
argued rather than overlooked: jj's own variables configure the *user*, so
removing them would change the product of the command rather than protect it.
The documented one is `JJ_CONFIG`, which "will be used instead of any
configuration files in the default locations"
([jj configuration](https://docs.jj-vcs.dev/latest/config/)) — and the file it
names is where `user.name` and `user.email` live. The identity variables
themselves are documented on that page nowhere, so the claim was checked directly
on jj 0.44.0 rather than asserted: a `jj commit` run with
`JJ_EMAIL` set attributes the commit jj creates to that address instead of to the
configured `user.email`. Stripping that would silently change who a commit is
attributed to, which is a worse failure than the one the scrub prevents.

**Why four `GIT_*` variables are nevertheless removed.** A jj workspace colocated
with Git has a real Git backend, and a Git-aware child in that workspace can be
redirected to a foreign repository by variables it inherited from whatever
started the calling process. `current_dir` alone does not stop that, because
those variables are consulted ahead of the working directory. That hazard is the
whole reason the list exists, and colocation is not otherwise explored here:
*What jj owns* is where that boundary is drawn, and a `.git`
beside a `.jj` is jj's business rather than this crate's.

<a id="worked-invocation"></a>
## Worked example: one invocation, built and run

The invocation is the first spawn of the carried operation — the `jj commit` that
records the retired task file, in the tree
[*Orientation*](01-orientation.md#commit-tour) fixed. It is chosen over the
read-back beside it because it is the one the crate's own suite has been made to
fail, so all three of its endings below are observed rather than imagined. The
caller is `Workspace::commit`, in `/work/atlas`, and this is the whole of what it
hands the seam:

```text
jj::output(
    "/work/atlas",                                the workspace root, already canonical
    &["commit",
      "-m", "rate-limit-k3: refuse a request over the burst ceiling",
      "root:\".grove/01-DONE-impl--rate-limit-k3.md\""],
)
```

The seam renders that into a string before it builds anything, because a refusal
will need to quote it:

```text
rendered("jj", args)
  -> jj commit -m rate-limit-k3: refuse a request over the burst ceiling root:".grove/01-DONE-impl--rate-limit-k3.md"
```

That line is not something a reader can paste, and the seam does not pretend
otherwise: the message contains spaces, so a shell would split it into six
arguments where the child received one. It is a record of what was attempted, and
nothing re-reads it. The last section of this chapter is about why that is a
deliberate position rather than a defect.

Then the child is assembled. Four things are set and nothing else is touched:

```text
program            jj                              resolved on PATH by the operating system
argv               ["commit",
                    "-m", "rate-limit-k3: refuse a request over the burst ceiling",
                    "root:\".grove/01-DONE-impl--rate-limit-k3.md\""]
                                                   four arguments, passed through unchanged
current_dir        /work/atlas                     the only thing that selects the repository
env_remove         GIT_DIR                         removed, not set empty
                   GIT_WORK_TREE
                   GIT_COMMON_DIR
                   GIT_INDEX_FILE
```

Everything else the calling process was holding — `PATH`, `HOME`, `TMPDIR`,
`JJ_CONFIG`, and every variable the consumer set for its own purposes — is
inherited untouched. The child is then run to completion and its three endings
are these.

**First ending: jj ran and succeeded.** `Command::output` returns a status of
zero, and stdout is the bytes jj printed — which for `jj commit` is nothing,
because jj reports what it did on stderr.

```text
status             0
stdout             ""
  -> Ok(Vec<u8>)                                   from raw_output
  -> Ok("")                                        from output, after a UTF-8 check
```

The caller discards that string, and the UTF-8 check ran anyway. The seam has no
way to know which callers read the answer, so it makes the same promise to all
four of them; the alternative is a second entry point whose contract is *this
output is not text and may be anything*, which nothing in the crate wants. The
seam also does not trim. Trimming is a decision about one command's output
format, and the seam knows nothing about which command it ran — `Workspace::commit`
trims the change id the *next* spawn returns, and `main_repo_of` trims the path
its own spawn returns.

**Second ending: jj is absent.** The failure happens before jj runs at all —
`Command::output` cannot start the program, and returns an `io::Error` rather
than a status.

```text
Command::output()  Err(io::Error { kind: NotFound, .. })
  -> Refusal::not_runnable("jj commit -m rate-limit-k3: … root:\"…\"", cause)
```

What a consumer prints:

```text
could not run `jj commit -m rate-limit-k3: refuse a request over the burst ceiling root:".grove/01-DONE-impl--rate-limit-k3.md"`: No such file or directory (os error 2)

Jujutsu drives this workspace, so its binary has to be on `PATH`. Install it (https://jj-vcs.github.io/jj/latest/install-and-setup/) and rerun.
```

The remedy is installation, and it is the same remedy whichever of the four call
sites spawned the child — which is why the seam constructs this refusal and none
of them does.

**Third ending: jj ran and declined.** The program started, so there is a status,
and it is not zero. Now the remedy is not installation but whatever jj printed, so
the refusal carries jj's own stderr, trimmed, together with the directory the
command ran in. This is the ending
`a_commit_that_cannot_land_names_the_operation_log_repair`
(`crates/jj-workspace/tests/workspace.rs`) builds: it makes a directory in the
workspace unreadable, and because jj snapshots the whole working copy before it
commits any part of it, the command fails on a directory the caller never named.

```text
status             255
stderr             Internal error: Failed to snapshot the working copy
                   Caused by:
                   1: Failed to read directory /work/atlas/unreadable
                   2: Permission denied (os error 13)

  -> Refusal::command_failed("jj commit -m … root:\"…\"",
                             "/work/atlas",
                             the four lines above, trimmed)
```

The status is `255` rather than `1`, and the crate never learns that. `success()`
is a boolean over the whole space of non-zero exits, which is the only reading
that stays correct as jj's codes change.

The test's own comment is explicit that this is a genuine failure of the command
rather than a simulated one, which is what makes it evidence for this seam rather
than for a stub. What a consumer prints is one line, and then the commit path
wraps it:

```text
`jj commit -m … root:"…"` failed in /work/atlas: Internal error: Failed to snapshot the working copy …
```

Separating the second ending from the third is the point of the whole function.
They differ in what a reader must do next, and there is no status code that
distinguishes them — one has a status and the other does not.

The second ending has no test, and neither does a fourth — jj printing bytes that
are not text, which becomes `Refusal::output_not_text`. Nothing in the crate's
suite names either constructor or either message, and the reason is a fixture
cost rather than an oversight: every test in the suite builds its tree by running
jj, so a test for jj's absence would have to remove from `PATH` the binary it
needs to set itself up, and non-UTF-8 stdout is not something a jj command can be
asked to produce. Both are stated here as unasserted rather than left for a reader
to assume covered.

> **The consumer's half.** Grove runs its sessions under a harness that sets
> variables of its own — `GROVE_SIGNAL_FILE`, `GROVE_HARNESS`, `GROVE_SKILL_DIR`
> and a dozen more — and a grove session's working directory is wherever that
> harness started it, which is routinely a subdirectory rather than the workspace
> root. Both halves of this chapter are answers to that: the root is passed
> explicitly as `current_dir` because the caller's own directory is not
> trustworthy for the purpose, and grove's variables survive into the child
> untouched because this crate has no way to tell which of them carry authority.
> None of those names appears anywhere in the crate.

<a id="the-file-and-its-claims"></a>
## The file, and the two claims it heads

This chapter owns `crates/jj-workspace/src/jj.rs` entire: eighty-one lines, the
smallest of the three modules, and the only one with no public item in it at all.

<!-- fragment «subprocess-seam-source» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="1-81" parent="source-subprocess" -->
<!-- insert «subprocess-seam-purpose» -->
<!-- insert «subprocess-nothing-ambient» -->
<!-- insert «subprocess-consumer-environment» -->
<!-- insert «subprocess-imports» -->
<!-- insert «subprocess-selectors» -->
<!-- insert «subprocess-output» -->
<!-- insert «subprocess-produced-output» -->
<!-- insert «subprocess-raw-output-build» -->
<!-- insert «subprocess-raw-output-endings» -->
<!-- insert «subprocess-rendered» -->
<!-- /fragment -->

The module comment opens by naming the seam and then says what makes a seam
worth having: the hygiene is a property of the crate *because* every invocation
is built here. It then promises exactly two things, and the count is doing work
— a module comment that listed six properties would be a description, and this
one is a contract narrow enough to check.

<!-- fragment «subprocess-seam-purpose» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="1-6" parent="subprocess-seam-source" -->
````rust
//! The child-process seam: every `jj` invocation this crate makes is built
//! here, so the hygiene below is a property of the crate rather than a habit
//! each call site has to remember.
//!
//! Two things are true of every invocation and of nothing else:
//!
````
<!-- /fragment -->

The first claim is the chapter's thesis, and it is argued in four sentences that
each answer a different objection. That repository selectors are *process-global
overrides* is why `current_dir` is not sufficient on its own. That a colocated
workspace has a real Git backend is why the hazard is real rather than
theoretical — an inherited `GIT_DIR` in a tree with no `.git` points at nothing,
and in a colocated tree it points at a repository a Git-aware child will happily
use. That they are removed rather than left unset is a distinction the next
section takes on its own. And the last three lines are the argued absence: the
premise above, written down at the site where a reader would otherwise ask why
the list has no `JJ_*` in it.

<!-- fragment «subprocess-nothing-ambient» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="7-15" parent="subprocess-seam-source" -->
````rust
//! **The repository is chosen by `current_dir` and by nothing ambient.**
//! Repository selectors are process-global overrides — `current_dir` alone does
//! not stop a Git-aware child from following an inherited foreign repository,
//! and a jj workspace colocated with Git has a real Git backend for them to
//! redirect. They are removed rather than merely left unset, because an
//! environment is inherited, not addressed. There is no `JJ_*` counterpart to
//! remove: jj selects its repository by walking up from the working directory,
//! and its own variables (`JJ_CONFIG`, `JJ_USER`, …) configure the *user*, so
//! stripping them would change who a commit is attributed to.
````
<!-- /fragment -->

The second claim is the boundary. The crate scrubs what it can reason about and
refuses to scrub what it cannot, and the reason given is not caution but
ignorance of a specific kind: this crate cannot know which of a consumer's
variables carry authority, and jj reads none of them, so removing them would be a
guess with no benefit to weigh against it. The obligation is named and placed —
a consumer whose environment grants something to its descendants owns that
scrubbing at its own spawn sites. Stating where a responsibility went is the
difference between a subtraction and an abdication, and it is the move the whole
book is about.

<!-- fragment «subprocess-consumer-environment» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="16-20" parent="subprocess-seam-source" -->
````rust
//!
//! **A consumer's own ambient variables are left alone.** This crate cannot
//! know which of them carry authority, and jj reads none of them; a consumer
//! whose environment grants something to its descendants owns that scrubbing at
//! its own spawn sites.
````
<!-- /fragment -->

The imports are the crate's thesis restated in three lines: one crate-internal
type and two `std` modules. `std::process::Command` is the entire dependency
behind everything in this chapter, and `Refusal` is the only thing this file
returns on the unhappy path.

<!-- fragment «subprocess-imports» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="21-24" parent="subprocess-seam-source" -->
````rust

use crate::refusal::Refusal;
use std::path::Path;
use std::process::Command;
````
<!-- /fragment -->

<a id="the-selectors"></a>
## The four names, and why they are removed rather than emptied

One array, and a doc comment that says what would qualify a fifth entry. It is
one of two lists of foreign names in the crate — *The namespace it will not name*
owns the other, and the two are worth comparing, because that one is a list of
names jj **owns** and this one is a list of names Git **reads**.

<!-- fragment «subprocess-selectors» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="25-33" parent="subprocess-seam-source" -->
````rust

/// Repository selectors — the variables that answer "which repository?" ahead
/// of the working directory.
const REPOSITORY_SELECTORS: [&str; 4] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_COMMON_DIR",
    "GIT_INDEX_FILE",
];
````
<!-- /fragment -->

The array is typed `[&str; 4]` rather than `&[&str]`, so the count is in the type
and the loop below cannot iterate over a list that has quietly become empty. The
doc comment defines the category rather than listing membership — the variables
that answer "which repository?" *ahead of the working directory* — which is the
test a fifth name would have to meet to belong here.

The four are Git's, and each answers that question in its own way: `GIT_DIR` names
the repository directory outright, `GIT_WORK_TREE` names the tree it belongs to,
`GIT_COMMON_DIR` redirects the shared part of a repository split across
worktrees, and `GIT_INDEX_FILE` names the index a command stages through.
`GIT_INDEX_FILE` is the one that looks out of place, because an index is not a
repository — it is in the list because a Git-aware child that is otherwise
correctly pointed can still be made to read and write a foreign staging area,
which is a redirection of state even though it is not a redirection of the
repository.

**Removed, not set empty**, and the comment's reason is that an environment is
inherited, not addressed. Setting `GIT_DIR=""` leaves the variable
present, and a program that tests for presence rather than for content sees it
set to a path that does not exist — which is a third state, and a worse one than
either of the two the crate is choosing between. `Command::env_remove` deletes
the entry from the child's environment map, so the child sees exactly what a
process started with none of them set would see. The calling process's own
environment is untouched by this: `env_remove` is a modification of the builder,
not of the parent, which is what lets a consumer keep whatever it was holding.

The claim that this works is asserted directly, and by a fixture built to make
the assertion meaningful:
`resolution_ignores_repository_selection_and_temporary_directory_environment`
(`crates/jj-workspace/tests/environment.rs`) sets `GIT_DIR`, `GIT_WORK_TREE` and
`GIT_COMMON_DIR` to a **colocated** foreign repository, resolves a workspace from
inside a different colocated tree, and requires the answer to be the intended
tree and nothing to have been created in the foreign one. The fixture is
colocated on purpose, and the test says so: in a tree with no `.git` the
selectors point at nothing and the test would pass whether or not the scrub
existed. That is a control on the test rather than on the code, and it is what
makes the green result evidence. Two things about its scope are worth stating
plainly: it sets three of the four variables and not `GIT_INDEX_FILE`, and it
lives in its own integration binary because `cargo test` runs a file's tests as
threads of one process that share an environment — which the file's own comment
explains, and which is why setting a variable here cannot leak into an unrelated
test.

<a id="the-two-entry-points"></a>
## Two entry points over one builder

The four call sites reach this file through two functions, and the split between
them is about what the caller does with the answer rather than about how the
command is run.

<!-- fragment «subprocess-output» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="34-43" parent="subprocess-seam-source" -->
````rust

/// Run `jj <args>` in `directory` and return its stdout as text.
///
/// Failure to *start* and failure to *succeed* are separate refusals: the first
/// means jj is not installed and the remedy is installation, the second means
/// jj declined and the remedy is whatever it printed.
pub(crate) fn output(directory: &Path, args: &[&str]) -> Result<String, Refusal> {
    let bytes = raw_output(directory, args)?;
    String::from_utf8(bytes).map_err(|_| Refusal::output_not_text(&rendered("jj", args)))
}
````
<!-- /fragment -->

`output` is the one three of the four call sites use, and it adds exactly one
thing to the shared helper below it: the UTF-8 check. jj's output is text by every
reasonable expectation, so the failure is remote — but `String::from_utf8`
returns a `Result` and the alternative to handling it is `from_utf8_lossy`, which
would silently replace the offending bytes and hand the caller a string that
parses as a change id while not being one. A refusal that says the answer cannot
be read is worth more than a corrupted answer, and it costs one line.

The doc comment carries the separation the whole chapter turns on, stated as two
remedies rather than as two error kinds: failure to *start* means jj is not
installed, and failure to *succeed* means jj declined and the remedy is whatever
it printed. Naming the remedy rather than the condition is the crate's habit
throughout, and *Refusal* is where it becomes a rule.

<!-- fragment «subprocess-produced-output» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="44-49" parent="subprocess-seam-source" -->
````rust

/// As [`output`], but the answer is *whether there was any* rather than what it
/// said — for the probes whose whole result is stdout being empty or not.
pub(crate) fn produced_output(directory: &Path, args: &[&str]) -> Result<bool, Refusal> {
    Ok(!raw_output(directory, args)?.is_empty())
}
````
<!-- /fragment -->

`produced_output` is the seam's one concession to a caller's shape, and it exists
for a single probe: `Workspace::is_tracked`, which asks `jj file list` for a
fileset and cares only whether anything came back. Returning the text and letting
the caller test it for emptiness would have worked; what it would have cost is
the UTF-8 check being applied to bytes nobody was going to read, so a tracked
file with an unrepresentable name would refuse a question whose answer is *yes*.
Asking whether stdout was empty is a question the raw bytes can answer, and this
function is the whole of asking it. *Scope and commit* owns `is_tracked` and
the measurement behind it.

<a id="building-the-child"></a>
## Building the child

Everything above is a public-to-the-crate surface over these nineteen lines. The
first half builds; nothing has run yet when it ends.

<!-- fragment «subprocess-raw-output-build» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="50-57" parent="subprocess-seam-source" -->
````rust

fn raw_output(directory: &Path, args: &[&str]) -> Result<Vec<u8>, Refusal> {
    let rendered = rendered("jj", args);
    let mut command = Command::new("jj");
    command.current_dir(directory).args(args);
    for selector in REPOSITORY_SELECTORS {
        command.env_remove(selector);
    }
````
<!-- /fragment -->

The rendering happens first, before the builder exists. Nothing forces that
order — `Command::args` copies its arguments rather than borrowing them, so the
string could as well be built inside either error arm — and the reason it is
hoisted is that there are two error arms and one string, and a value both of them
need is easier to trust computed once than computed twice identically. It is
computed unconditionally, on a path that usually succeeds,
which is a deliberate cost: a few dozen bytes per invocation buys a refusal that
can always quote the command, and this crate spawns at most two children per
operation rather than thousands.

`Command::new("jj")` names the program and lets the operating system find it on
`PATH`. Nothing here resolves an absolute path to the binary or checks that it
exists first, and the omission is the design: a pre-flight existence check would
be a second answer to a question `Command::output` already answers, it can go
stale between the check and the spawn, and its failure would have to become the
same refusal anyway. `current_dir(directory)` is the repository selection, per
the premise, and `args(args)` passes the caller's list through with no shell and
no quoting — there is no command line for anything to be split on.

Then the loop, which is the whole of the hygiene: four `env_remove` calls over
the typed array. It is a loop rather than four literal calls so that the list and
the removal cannot drift apart, and it sits between the directory being set and
the child being started, which is the only window in which it could.

<a id="the-two-endings"></a>
## Failure to start, and failure to succeed

The second half runs the child and turns its two ways of going wrong into two
different refusals.

<!-- fragment «subprocess-raw-output-endings» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="58-69" parent="subprocess-seam-source" -->
````rust
    let out = command
        .output()
        .map_err(|cause| Refusal::not_runnable(&rendered, cause))?;
    if !out.status.success() {
        return Err(Refusal::command_failed(
            &rendered,
            directory,
            &String::from_utf8_lossy(&out.stderr),
        ));
    }
    Ok(out.stdout)
}
````
<!-- /fragment -->

`Command::output` runs the child to completion and collects both streams, and its
`Result` is the first ending: an `Err` here means the program could not be
started at all. There is no status to inspect, because nothing ran.
`Refusal::not_runnable` carries the rendered command and keeps the `io::Error` as
the refusal's `source()`, so a consumer walking the chain sees what the operating
system actually said — `No such file or directory` for an absent binary, but also
a permission error for a `jj` on `PATH` that is not executable, which is the same
class of failure with a different sentence.

Reaching this line at all means jj ran, so the status is the second ending, and
`success()` is the whole test: any non-zero exit is a decline. The crate does not
interpret exit codes, and jj's are not documented as a contract, so a code-by-code
reading would be inventing a protocol. `Refusal::command_failed` therefore carries
three things a reader needs to act — the command as typed, the directory it ran
in, and jj's own stderr — and adds no diagnosis of its own. The directory is in
there because the same argv means different things in different trees, and it is
the one piece of context the reader cannot recover from the message alone.

`String::from_utf8_lossy` on stderr is the deliberate opposite of the strict
`String::from_utf8` on stdout two functions above, and the two are worth reading
together. Stdout is *parsed* — a change id, a path — so a byte that is not text
makes the answer unusable and must refuse. Stderr is *displayed*, so a byte that
is not text costs a replacement character in a message a human reads, and
refusing to show a diagnostic because one character was malformed would withhold
the only remedy the crate has. The trimming is a third detail with the same
motive: `Refusal::command_failed` trims the stderr it is given, so a refusal
interpolated into a sentence does not carry jj's trailing newline into the middle
of it.

The success path is the last line and does nothing: the raw bytes go back to
whichever entry point asked for them. Everything above this point is refusal
construction, which is the honest shape of a function whose job is a boundary.

`a_commit_that_cannot_land_names_the_operation_log_repair`
(`crates/jj-workspace/tests/workspace.rs`) is the test that exercises this half
for real. It makes a directory unreadable so that jj's own snapshot fails, which
means the `CommandFailed` it produces comes from a jj that started and declined
rather than from a mocked status — and the message the test asserts on is the
`CommitNotRecorded` that wraps it, because the commit path adds its own remedy on
top of the one this file supplied. *Refusal* owns that wrapping and the `source()`
chain it builds.

<a id="a-command-a-reader-could-type"></a>
## The command as a reader would type it

The last function in the file is the one both refusals depend on. Its doc comment
makes a claim about safety, and the claim is worth more scrutiny than its eleven
lines suggest.

<!-- fragment «subprocess-rendered» owner="nothing-ambient" source="crates/jj-workspace/src/jj.rs" lines="70-81" parent="subprocess-seam-source" -->
````rust

/// The command as a reader would type it, for a refusal to quote. Arguments are
/// shown verbatim: the crate builds every one of them, so none is user text
/// that could need quoting to stay honest.
fn rendered(program: &str, args: &[&str]) -> String {
    let mut line = String::from(program);
    for arg in args {
        line.push(' ');
        line.push_str(arg);
    }
    line
}
````
<!-- /fragment -->

A loop that joins a program and its arguments with single spaces, under a comment
that is the reason it is allowed to be that simple. Rendering an argv back into a
command line is normally unsafe: an argument containing a space, a
quote or a newline produces a line that means something different from the list
it came from, and a reader who pastes it gets a different command. The comment
states why this crate is exempt — it builds every argument itself, so none is
user text — and that exemption is checkable rather than asserted. The four call
sites pass literal flags, a commit message, and a fileset string the crate
constructs; only the message is a consumer's text, and it reaches jj as one
argument regardless of what the rendered line looks like.

So the rendered string is documentation, never an instruction the code acts on,
and that is the distinction that makes it safe. It exists only inside refusal
messages, where a reader is being told what was attempted. A message containing a
`-m` argument with spaces in it will not round-trip if pasted, and the crate
neither claims it will nor depends on it: nothing anywhere re-parses this string.
The alternative was to quote each argument for a shell, which would have meant
choosing a shell — and the crate has no shell, precisely because `args` passes a
list.

`String::push` and `push_str` rather than `format!` or `join`: one `String` is
grown in place, where `join` would need a temporary `Vec` holding the program and
the arguments together before it could flatten them. It is the shape a crate with
no dependencies reaches for, and *Orientation* is where that constraint was
argued.

<a id="what-this-chapter-settled"></a>
## What this chapter settled

The seam is now complete, and it is smaller than the argument for it. One private
builder, two entry points over it, four names in an array, and three refusals —
one for a jj that would not start, one for a jj that declined, and one for output
that is not text. The
property it buys is stated once and holds for every invocation the crate will
ever make: the repository is chosen by the directory the child is started in, and
by nothing that was lying around in the environment when the calling process
began.

Everything so far has been about what the crate refuses to decide for its
consumer. The next chapter is about the one thing it adds — and about the fact
that it still refuses to name it.

[Previous: The gate](02-the-gate.md) | [Contents](README.md)

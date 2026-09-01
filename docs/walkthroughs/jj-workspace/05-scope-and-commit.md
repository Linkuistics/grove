# Scope and commit
<!-- book-page id="scope-and-commit" slice="no-transactions" order="5" -->
[Previous: The namespace it will not name](04-namespace.md) | [Contents](README.md) | [Next: Refusal](06-refusal.md)

<a id="no-transactions"></a>
## It takes commits; it does not implement transactions

The fifth refusal is the largest subtraction in the crate, and the only one that
names what it leaves out. There is no witness file, no manifest of
intended writes, no rollback proof, no index image, no quarantine area and no
recovery path. Six mechanisms a reader might expect from a component whose public
verb is `commit`, and every one of them is missing for the same reason: the tool
underneath already has it, and having it twice is worse than having it once.

The second clause of the refusal is narrower and is the one this chapter has to
defend. **Reads add no history.** A component that answers questions about a
repository can answer them in a way that changes the repository, and a caller
that has to know which of its questions are also writes has been handed a worse
interface than the one underneath. So every probe here passes
`--ignore-working-copy` and cannot record an operation — except one, and the
exception is what makes the rule checkable rather than decorative.

`is_tracked` deliberately does not pass it. Its answer is about the working copy,
so a probe that suppressed the snapshot would answer about a tree that no longer
exists. The justification offered for that exception is not a preference and not
a design intuition: it is a **measurement**, taken against jj 0.44.0, recorded in
the crate's own module documentation and re-taken for this chapter. The
asymmetry, and what was measured to license it, is this chapter's central claim.

One hundred and thirty-nine lines carry all of it: a ten-line type, a four-line
probe, a thirty-three-line commit, and two private functions that turn a caller's
path into one string. The commit is the smallest of the three arguments and the
path algebra is the largest, which is the shape a reader should expect from a
crate that has delegated the transaction and kept the addressing.

<a id="the-premise"></a>
## The premise: a snapshot, an operation log, and an identity that survives

Three behaviours of jj carry this chapter. They are stated here once, with jj's
own words, because a reader who disputes the refusal above is disputing one of
these rather than disputing the crate.

**The working copy is snapshotted at the start of almost every command.** jj's
glossary: the working copy "is automatically snapshot at the beginning of almost
every `jj` command"
([jj glossary, *Working copy*](https://docs.jj-vcs.dev/latest/glossary/#working-copy)).
This is why the crate never has to stage anything, and it is also why a probe
cannot be assumed free: a command that snapshots is a command that may write.

**The operation log is the transaction record.** It "is the DAG formed by
operation objects, much in the same way that commits form a DAG"
([jj glossary, *Operation log*](https://docs.jj-vcs.dev/latest/glossary/#operation-log)).
Every jj command that changes anything appends to it, and every state it records
remains reachable. A rollback proof written by this crate would be a second,
weaker copy of a record jj keeps unconditionally, and the crate would then own
the harder half of the problem — keeping the two in agreement across a crash.

**A change id survives a rewrite.** "Rewriting a commit results in a new commit,
and thus a new commit ID, but the change ID generally remains the same"
([jj glossary, *Rewrite*](https://docs.jj-vcs.dev/latest/glossary/#rewrite)).
That single sentence is the whole argument for what `Commit` holds, and the
section after the worked example is an expansion of it.

Two things follow that a reader should hold before reading any code. First, there
is no state this crate keeps between calls: every operation is one child process
and the durable record of it is jj's. Second, the crate's only contribution to
failure handling is a refusal — it names jj's repair and runs none of it, which
is *Refusal*'s subject rather than this chapter's.

<a id="worked-commit"></a>
## Worked example: the carried commit, at full resolution

[*Orientation*](01-orientation.md#commit-tour) followed this operation at low
resolution and fixed its values; [*The gate*](02-the-gate.md#worked-resolution)
resolved the tree and [*The namespace it will not name*](04-namespace.md#worked-reservation)
reserved the directory. Nothing about the tree changes here. What changes is the
resolution: every argument, every intermediate string, and the returned value.

The tree is the native workspace at `/work/atlas`, with a task file the session
has just renamed and one unrelated edit in the working copy that must stay there:

```text
/work/atlas/                                    the workspace root, canonical
├── .jj/
│   ├── repo/
│   ├── working_copy/
│   └── grove/                                  reserved in the previous chapter
├── .grove/
│   ├── BRIEF.md
│   └── 01-DONE-impl--rate-limit-k3.md          renamed: the commit is about this
└── crates/
    └── gateway/
        └── src/
            └── main.rs                         edited, and out of scope
```

The caller holds the `Workspace` resolved from `/work/atlas/crates/gateway/src`,
whose `root` is `/work/atlas`. It names one path, relative, and one message.

```text
workspace.commit(
    &[Path::new(".grove/01-DONE-impl--rate-limit-k3.md")],
    "rate-limit-k3: refuse a request over the burst ceiling",
)

  paths.is_empty()                       -> false      the scope guard passes

  fileset(".grove/01-DONE-impl--rate-limit-k3.md")
    relative(…)
      path.is_absolute()                 -> false
      root.join(path)                    -> /work/atlas/.grove/01-DONE-impl--rate-limit-k3.md
      strip_prefix("/work/atlas")        -> Ok(".grove/01-DONE-impl--rate-limit-k3.md")
      the relative path is not empty, so the root is not being committed as a scope
      components joined with `/`         -> ".grove/01-DONE-impl--rate-limit-k3.md"
    no `"` and no `\` in that string, so nothing is escaped
    -> "root:\".grove/01-DONE-impl--rate-limit-k3.md\""

  args = ["commit",
          "-m",
          "rate-limit-k3: refuse a request over the burst ceiling",
          "root:\".grove/01-DONE-impl--rate-limit-k3.md\""]

  jj::output("/work/atlas", args)
    current_dir = /work/atlas
    GIT_DIR, GIT_WORK_TREE, GIT_COMMON_DIR, GIT_INDEX_FILE removed
    exit status 0, stdout empty       -> Ok("")        the value is discarded

  jj::output("/work/atlas",
             ["log", "-r", "@-", "--no-graph", "--ignore-working-copy",
              "-T", "change_id"])
    -> Ok("vrxqnwzomtklpsuvyzqrnwmtkxlpsoun\n")

  -> Ok(Commit { change_id: "vrxqnwzomtklpsuvyzqrnwmtkxlpsoun" })
```

Three things in that trace are observable from outside and were checked against
jj 0.44.0 rather than read off the source. **`jj commit` writes nothing to
stdout** — its two progress lines go to stderr — so the first `jj::output` call
returns an empty string and the code discards it; the call is made for its effect
and its exit status, and a reader who expects the change id to come back from it
will not find it there. **The commit is scoped.** After the call, `jj file list
-r @-` shows the task file in the new commit and `jj status` still shows
`crates/gateway/src/main.rs` modified in the working copy: jj snapshotted the
whole tree and committed the named fileset out of it. **The second call is
read-only**, and `--ignore-working-copy` is what makes that true rather than
nearly true.

The observable end is one value and one tree state:

```text
Commit { change_id: "vrxqnwzomtklpsuvyzqrnwmtkxlpsoun" }

/work/atlas/ working copy after the call:
  crates/gateway/src/main.rs      still modified, still uncommitted
  .grove/01-DONE-impl--rate-limit-k3.md   committed, and `@` is a fresh empty change
```

**The second ending: a caller that names no paths at all.** The refusal is
returned before any child process is started, and that is asserted rather than
argued — `a_commit_with_no_paths_is_refused_rather_than_widened`
(`crates/jj-workspace/tests/workspace.rs`) counts operations in the log before
and after and requires the count to be unchanged.

```text
workspace.commit(&[], "everything")
  -> a path-scoped operation was given no scope: no paths were named

     Name the paths the operation is about. Widening it to the whole working copy
     is not the fallback, because the scope is what the caller asked for.
```

Widening was the available alternative and it is the one this refusal exists to
exclude. `jj commit -m "…"` with no fileset commits the entire working copy,
which for the tree above would have swept in the unrelated edit to `main.rs`. A
caller that reaches `commit(&[], …)` has lost its scope somewhere upstream, and
the two outcomes are a message naming that, or a commit containing work nobody
asked to commit.

> **The consumer's half.** grove calls `Workspace::commit` at exactly one site,
> in `crates/grove-loop/src/tree_lifecycle.rs`, to commit the removal of the task
> tree at teardown; the path it names is `.grove/` and the message is the
> work-item handle followed by what the session did. The general shape of the
> call — one session's whole task, sealed in one commit named by its handle — is
> the [task commit boundary](../../../CONTEXT.md#task-commit-boundary), and the
> reason it is a *boundary* is the scoping this chapter argues: the working copy
> is itself a commit, so anything left unscoped joins the next session's change.
> None of that vocabulary reaches the crate. It receives a slice of paths and a
> string.

<a id="the-commit-identity"></a>
## What a commit returns, and why it is not the commit id

Ten lines sit between the reserved-name list and the `Workspace` type, and they
are the crate's entire model of a taken commit. The block is a doc comment and a
struct with one field, and the comment is longer than the struct because the
choice of field is the only decision in it.

<!-- fragment «commit-identity» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="62-71" parent="source-library" -->
<!-- insert «commit-change-id-argument» -->
<!-- insert «commit-type» -->
<!-- /fragment -->

The argument is the premise above applied to a return value. A commit id names
the bytes of a commit; a change id names the work the commit is a version of, and
survives every rewrite that gives those bytes a new name. The comment lists the
three rewrites by their commands rather than describing them abstractly, which is
what makes the claim checkable by a reader who has jj in front of them.

<!-- fragment «commit-change-id-argument» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="62-66" parent="commit-identity" -->
````rust
/// A commit this crate took.
///
/// The change id rather than the commit id: a change id survives the rewrites —
/// `jj describe`, `jj squash`, a rebase — that give a commit a new commit id,
/// so it is the identity that still names this work afterwards.
````
<!-- /fragment -->

The alternative was to return the commit id, and it fails at the first thing a
consumer does with the value. A caller that records the id and later describes,
squashes or rebases the commit is holding a name for something that no longer
exists as the current version of that work. Measured on jj 0.44.0: after
`jj describe`, the commit id recorded before the rewrite still resolves, and it
resolves to the old commit with the old description, while the change id resolves
to a new commit id carrying the new one. A caller holding the commit id is
therefore not told anything is wrong; it is quietly reading the superseded
version.
`the_returned_change_id_still_names_the_commit_after_it_is_rewritten`
(`crates/jj-workspace/tests/workspace.rs`) is that claim as a test: it commits,
runs `jj describe -r <change id> -m "second wording"` through the real binary,
and then asks jj for the description at the same change id. Rewriting through the
crate is not offered, so the test rewrites the way a user would, which is the
only way the claim could be false in practice.

Returning both was the other option, and it is rejected by what a consumer would
then have to decide. Two identifiers in one value is two questions for every
caller — which one to log, which one to compare, which one to pass back to jj —
and the crate has no consumer to answer them for. One field means there is
nothing to choose.

<!-- fragment «commit-type» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="67-71" parent="commit-identity" -->
````rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    pub change_id: String,
}

````
<!-- /fragment -->

The field is `pub`, and it is a `String` rather than a newtype. Both are the same
decision as the empty dependency table
[*Orientation*](01-orientation.md#package-contract) read: a change id is jj's
identifier, jj prints it as text, and a `ChangeId` wrapper here would be this
crate inventing a type for a value it does not interpret. Nothing in the crate
parses it, compares it, or checks its length; it is read from stdout, trimmed,
and handed on.

`Clone, Debug, PartialEq, Eq` is the derive set for a value that is data. There
is no `Display`: printing a change id is a consumer's formatting decision, and a
`Display` here would be the crate choosing whether the surrounding text says
*change* or *commit* — which is exactly the consumer's-vocabulary line the crate
does not cross. Equality is derived rather than hand-written because two `Commit`
values are equal when their change ids are, and there is no normalisation for a
hand-written implementation to perform.

<a id="the-asymmetry"></a>
## The one probe that lets jj snapshot, and the measurement that licenses it

The rest of this chapter's source is one contiguous run of a hundred and
twenty-nine lines: the tracking probe, the commit, and the two private functions
that address a path. They are contiguous in the file and they are one argument,
so the block is declared here and its parts are read in the three sections that
follow.

<!-- fragment «scope-tracking-and-commit» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="146-274" parent="source-library" -->
<!-- insert «tracking-contract» -->
<!-- insert «tracking-probe» -->
<!-- insert «commit-contract» -->
<!-- insert «commit-scope-guard» -->
<!-- insert «commit-invocation» -->
<!-- insert «commit-change-id-read» -->
<!-- insert «commit-return» -->
<!-- insert «fileset-contract» -->
<!-- insert «fileset-quoting» -->
<!-- insert «relative-contract» -->
<!-- insert «relative-absolute» -->
<!-- insert «relative-strip-or-canonical-parent» -->
<!-- insert «relative-root-is-not-a-scope» -->
<!-- insert «relative-render» -->
<!-- /fragment -->

The first sixteen of those lines are a doc comment, and they carry the exception
to the crate's read discipline. Four paragraphs: what the question is about, why
that forces the snapshot, how a directory answers, and which paths are refused.

<!-- fragment «tracking-contract» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="146-161" parent="scope-tracking-and-commit" -->
````rust
    /// Does this workspace **track** `path`?
    ///
    /// About the tree as it is on disk *now*, which is why this is the one
    /// probe here that lets jj snapshot first. A caller reads this answer to
    /// decide something about the file in front of it — whether removing it
    /// could be undone, whether a repository could have shipped it — and an
    /// answer about a state the tree has already left would be wrong for both.
    /// The snapshot is not extra history: jj takes it at the next command
    /// whatever that command is, and takes none at all when nothing changed.
    ///
    /// A directory answers for everything beneath it: `true` when anything
    /// under it is held.
    ///
    /// `path` may be absolute or relative to [`root`](Self::root); a path
    /// outside this workspace is refused, because a workspace answers only for
    /// its own files.
````
<!-- /fragment -->

*About the tree as it is on disk now* is the whole of the exception, and the
second sentence says who needs that and why. A caller reading this answer is
deciding something about the file in front of it — whether removing it could be
undone, whether a repository could have shipped it — and both questions are about
the current tree. An answer computed with `--ignore-working-copy` would be about
the last snapshot, which for a file created since that snapshot is the answer
*no* to a question whose true answer is *yes*.

`what_is_tracked_is_answered_about_the_tree_as_it_is_now`
(`crates/jj-workspace/tests/workspace.rs`) is that case exactly: it commits one
file, writes a second that has never been snapshotted, and requires `is_tracked`
to answer `true` for the second. That test fails against a probe that passes
`--ignore-working-copy`, which is what makes the flag's absence here a decision a
test defends rather than an omission.

**The third sentence is the measurement, and it is stated as one.** The claim is
that letting jj snapshot costs no history: *jj takes it at the next command
whatever that command is, and takes none at all when nothing changed.* That is
falsifiable and it was measured, on jj 0.44.0, by counting the operation log
around a probe. Re-taken while writing this chapter, in a workspace with nine
operations in its log:

```text
jj op log --no-graph --ignore-working-copy -T 'id ++ "\n"' | wc -l   ->  9

is_tracked over an unchanged working copy                            ->  9
    (the probe ran, jj snapshotted, and nothing was recorded)

one byte appended to a file, then is_tracked again                   -> 10
    (exactly one operation: the snapshot of a change that was already made)
```

The second row is what the crate's claim rests on and the third is what bounds
it. The probe is not free in general — a probe over a *changed* working copy does
record an operation — and the argument is not that it is free but that the
operation it records is one jj was going to record at the next command for any
reason. The snapshot is taken earlier, not taken twice. A reader who wants that
as a regression test rather than as a paragraph has
`asking_twice_over_an_unchanged_working_copy_records_no_operation`, which asserts
the second row: two probes over an unchanged tree, and an operation count that
does not move.

The fourth paragraph is the two remaining questions a caller would otherwise have
to discover. A directory answers for everything beneath it, which follows from
what the probe asks jj rather than from anything the crate does; and a path
outside the workspace is refused rather than answered `false`, because *false*
and *not mine to say* are different answers and only one of them is true.
`a_directory_is_tracked_when_the_snapshot_holds_anything_beneath_it` and
`a_path_outside_the_workspace_is_refused_rather_than_answered` hold those two.

The body is two lines, and the second is the only place in the crate that calls
the seam's boolean entry point.

<!-- fragment «tracking-probe» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="162-166" parent="scope-tracking-and-commit" -->
````rust
    pub fn is_tracked(&self, path: &Path) -> Result<bool, Refusal> {
        let fileset = self.fileset(path)?;
        jj::produced_output(&self.root, &["file", "list", &fileset])
    }

````
<!-- /fragment -->

`jj file list <fileset>` with no `-r` is a question about the working-copy
revision, which is what makes this the probe that snapshots; the same command
with `-r @-` — the form the crate's tests use to inspect a commit — would be a
question about history and would take `--ignore-working-copy` like every other
read here. The flag is absent because the revision is `@`.

`produced_output` is the seam entry point whose whole answer is whether stdout
was empty
([*The subprocess seam*](03-subprocess-seam.md#the-two-entry-points)), and the
fit is exact: `jj file list` prints one line per matching file and nothing when
nothing matches. A path that does not exist is not an error to jj — measured on
0.44.0, `jj file list 'root:"absent.md"'` exits 0, prints
`Warning: No matching entries for paths: absent.md` on **stderr**, and prints
nothing on stdout. So the absent case reaches `is_tracked` as an ordinary
`false`, and the failure cases that do exist — jj missing, jj declining — are
still separated by the seam. A probe that had parsed stdout would have had to
decide what to do with that warning; a probe that measures emptiness does not see
it.

The one line above it is `let fileset = self.fileset(path)?`, and it is the same
call `commit` makes on every one of its paths. The two operations share their
addressing entirely, which is why the path algebra is one section of this chapter
rather than an appendix to two.

> **The consumer's half.** grove asks this question twice, and both times to
> decide whether something else is safe. In
> `crates/grove-loop/src/tree_lifecycle.rs` it is the precondition for deleting a
> finished task tree: jj can only restore what it tracks, so an untracked
> `.grove/` would make the deletion the unrecoverable kind, and grove refuses
> with a message naming the commit that would fix it. In
> `crates/grove-loop/src/session_config.rs` it enforces the opposite polarity — a
> *tracked* configuration delta is refused, because a delta names a program to
> execute and a tracked one would let a repository choose what grove spawns in
> every checkout of it. One probe, two consumers' rules, and the crate holds
> neither of them.

<a id="the-commit"></a>
## One path-scoped commit, and two refusals that mean different things

`commit` is thirty-three lines and does three things in a fixed order: it turns
every caller path into a fileset, it runs one `jj commit`, and it reads back the
identity of what it just took. The eleven-line comment above it argues the first
and the third; the second needs no argument, because it is one command.

<!-- fragment «commit-contract» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="167-177" parent="scope-tracking-and-commit" -->
````rust
    /// Take a commit scoped to `paths` and seal the working copy.
    ///
    /// Path-scoped, so unrelated working-copy changes stay in the working copy:
    /// jj snapshots everything and then commits only the fileset named here.
    /// An empty `paths` commits nothing and is refused rather than silently
    /// widened to the whole working copy — the scope is the point of the call.
    ///
    /// The refusal returned when the commit itself does not land is the only
    /// one that means *there is no commit*; it names jj's operation-log repair.
    /// A refusal from reading the new commit's identity afterwards means the
    /// commit landed and could not be named.
````
<!-- /fragment -->

*Take a commit scoped to `paths` and seal the working copy* is the postcondition,
and *seal* is the load-bearing word. `jj commit` does not only record the named
files; it leaves a fresh empty change on top, so the working copy the caller
continues in is not the one the commit was taken from. That is why the scope
matters more here than it would in a version control system where the working
copy is not itself a commit: anything left out of the fileset stays in the
working copy and joins whatever is committed next.

The second paragraph states the widening refusal the worked example ended on, and
the third is the distinction this chapter has to make precisely, because a
consumer that gets it wrong loses work or duplicates it.

**Two refusals can come out of this function and they say different things about
the world.** The refusal from the commit itself means *there is no commit*: the
working copy still holds everything the caller prepared, and the repair is jj's
operation log. The refusal from reading the change id afterwards means *the
commit landed and could not be named*: the work is committed, the tree is sealed,
and what the caller lost is the identifier — retrying the whole operation would
commit nothing, because the fileset the caller named is already in a commit. The
comment says exactly that, in two sentences, and the code distinguishes them by
wrapping one and not the other.

<!-- fragment «commit-scope-guard» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="178-186" parent="scope-tracking-and-commit" -->
````rust
    pub fn commit(&self, paths: &[&Path], message: &str) -> Result<Commit, Refusal> {
        if paths.is_empty() {
            return Err(Refusal::not_scoped("no paths were named"));
        }
        let filesets = paths
            .iter()
            .map(|path| self.fileset(path))
            .collect::<Result<Vec<_>, _>>()?;

````
<!-- /fragment -->

The guard is first and the mapping is second, and both run before any child
process starts. `collect::<Result<Vec<_>, _>>()` is what makes that true for the
whole slice rather than for the first path: it stops at the first refusal and
returns it, so a caller that names five paths of which the third is outside the
workspace gets `OutsideWorkspace` and a repository that has not been touched. The
alternative — spawn `jj commit`, let jj reject the fileset — would have produced
a refusal quoting jj's error about a path this crate could have named itself, and
would have taken a snapshot on the way.

`Vec<String>` rather than an iterator is forced by the line below it: the
argument list borrows from these strings, so they have to outlive it. That is the
whole reason the collection is materialised, and it is worth naming because a
reader looking for a lazier form will find the borrow rather than a preference.

<!-- fragment «commit-invocation» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="187-191" parent="scope-tracking-and-commit" -->
````rust
        let mut args = vec!["commit", "-m", message];
        args.extend(filesets.iter().map(String::as_str));
        jj::output(&self.root, &args)
            .map_err(|cause| Refusal::commit_not_recorded(&self.root, cause))?;

````
<!-- /fragment -->

Four lines are the entire commit. `vec!["commit", "-m", message]` and then one
fileset argument per path — no `--`, no path separator, no quoting at the process
boundary, because each argument is one element of an argv array and the operating
system carries it whole. The message is one argument however many spaces it
contains, which is the reason
[*Orientation*](01-orientation.md#commit-tour) shows argument lists rather than
command lines.

`map_err(|cause| Refusal::commit_not_recorded(&self.root, cause))` is the first
half of the distinction above. The seam's own refusal — jj could not be started,
or jj ran and declined — is wrapped in one that names *state* rather than a
command, keeping the original as its cause. That is the only refusal in the crate
that says something about the tree the caller is standing in, and its remedy is
jj's operation log rather than anything this crate can run.
`a_commit_that_cannot_land_names_the_operation_log_repair`
(`crates/jj-workspace/tests/workspace.rs`) makes a commit genuinely fail — it
removes read permission from a directory in the working copy, so jj's snapshot
fails on a real filesystem error rather than a simulated one — and asserts on
three things in the message: that the commit is absent, that `jj undo` and
`jj op log` are named, and that the crate disclaims running any recovery.
*Refusal* reads the message itself.

<!-- fragment «commit-change-id-read» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="192-206" parent="scope-tracking-and-commit" -->
````rust
        // The commit just taken is the working copy's parent: `jj commit` leaves
        // a fresh empty working-copy commit on top of it. Read read-only, so
        // naming the commit does not add an operation of its own.
        let change_id = jj::output(
            &self.root,
            &[
                "log",
                "-r",
                "@-",
                "--no-graph",
                "--ignore-working-copy",
                "-T",
                "change_id",
            ],
        )?;
````
<!-- /fragment -->

The comment answers the two questions a reader has at this line. **Why `@-`:**
`jj commit` leaves a fresh empty working-copy commit on top of what it took, so
the commit just taken is the working copy's parent, and `@` would name the empty
change instead. **Why `--ignore-working-copy`:** naming a commit is a read, and
the read discipline this chapter opened on says a read adds no history. Without
the flag the identity lookup would snapshot, and a function whose job is to
commit would have recorded a second operation for the privilege of describing
the first.

`--no-graph` and `-T change_id` reduce the output to one field: no graph
characters, no decoration, no second line. The crate never parses this output — it
trims it — and the template is what makes trimming sufficient.

This second call's refusal is **not** wrapped, and that is the other half of the
distinction. The `?` propagates the seam's refusal as it stands, so a consumer
that receives `CommandFailed` from `jj log` is receiving a refusal about a
command, while `CommitNotRecorded` is a refusal about state. A consumer cannot
match on either — the type is opaque — but it can print them, and the two messages
do not say the same thing.

<!-- fragment «commit-return» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="207-211" parent="scope-tracking-and-commit" -->
````rust
        Ok(Commit {
            change_id: change_id.trim().to_owned(),
        })
    }

````
<!-- /fragment -->

`change_id.trim().to_owned()` is the only processing applied to anything jj says
in this crate. jj's template output ends with a newline; trimming it is not
normalisation of the value but removal of the line ending, and the `to_owned` is
the one allocation on the success path. What is *not* done here is as deliberate:
the string is not validated for length, alphabet or emptiness, because a change
id is jj's identifier and a crate that checked its shape would be modelling a
format jj has never promised this crate.

<a id="the-path-algebra"></a>
## The path algebra: one argument, spread over two functions and the gate

`fileset` and `relative` are private, called from two places, and total
sixty-three lines. Read as two functions they are string manipulation. Read as
one argument they are the consequence of a decision made two chapters ago: the
gate canonicalises the workspace root, so every path the crate later compares
against that root has to be made comparable to a canonical path. Nothing else in
this chapter is a through-line; this is.

<!-- fragment «fileset-contract» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="212-217" parent="scope-tracking-and-commit" -->
````rust
    /// `path` as a jj fileset rooted at this workspace.
    ///
    /// `root:"…"` is what makes the scope the *workspace's*, rather than
    /// relative to whatever directory the command runs in — the two are the
    /// same here, and stating it keeps them the same if that ever stops being
    /// true.
````
<!-- /fragment -->

`root:"…"` is a jj fileset pattern that resolves its path against the **workspace
root**; the unprefixed form is resolved against the current working directory
([jj filesets](https://docs.jj-vcs.dev/latest/filesets/)). The comment states
that the two are the same here — the seam runs every command with `current_dir`
set to the workspace root — and then says why the prefix is written anyway:
*stating it keeps them the same if that ever stops being true.* That is a
one-word insurance policy against a change at a different seam. If the subprocess
seam ever ran a command from a subdirectory, every unprefixed fileset in the
crate would silently change meaning, and the failure would be a commit scoped to
the wrong files rather than an error.

<!-- fragment «fileset-quoting» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="218-230" parent="scope-tracking-and-commit" -->
````rust
    fn fileset(&self, path: &Path) -> Result<String, Refusal> {
        let relative = self.relative(path)?;
        let mut quoted = String::from("root:\"");
        for ch in relative.chars() {
            if ch == '"' || ch == '\\' {
                quoted.push('\\');
            }
            quoted.push(ch);
        }
        quoted.push('"');
        Ok(quoted)
    }

````
<!-- /fragment -->

Two characters are escaped and no others, and the set is exactly right rather
than approximately right. Inside a jj double-quoted string literal, `\` opens an
escape sequence and `"` closes the literal; those are the only two bytes with a
meaning, and jj's accepted escapes — `\"`, `\\`, `\t`, `\r`, `\n`, `\0`, `\e`,
`\xHH` — are all introduced by the backslash this loop has already doubled
([jj templates, string literals](https://docs.jj-vcs.dev/latest/templates/)).
So a path containing a literal backslash-t survives as backslash-t rather than
arriving at jj as a tab, and a path containing a quote cannot end the fileset
expression early. Every other byte passes through untouched, which is what a
scope has to do: the string is a filename, not a pattern.

The escaping is done by hand, character by character, rather than with
`replace`. Two passes of `replace` would be the shorter spelling and would be
wrong in the classic way — replacing `"` with `\"` after replacing `\` with `\\`
re-escapes nothing, but doing it in the other order doubles the backslash the
first pass introduced. One pass over the characters cannot have an order bug,
and this is a crate with no dependencies to borrow an escaper from.

**No test in the suite passes a path containing either character**, so the
argument above rests on jj's documented literal syntax and on reading the loop,
and not on anything that goes red. It is stated rather than left to be assumed,
because it is the one place in this chapter where a claim about correctness has
no assertion behind it: every path the twenty-eight interface tests name is
ordinary.

<!-- fragment «relative-contract» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="231-238" parent="scope-tracking-and-commit" -->
````rust
    /// `path` expressed relative to the workspace root, with `/` separators.
    ///
    /// Tried on the path as given first, and only then on a canonicalised
    /// version: the root is canonical, so a caller that reached the path
    /// through a symlinked ancestor would otherwise be told its own workspace
    /// does not contain it. Canonicalisation is applied to the **parent**, so a
    /// path that no longer exists — the one the caller is about to commit the
    /// deletion of — still resolves.
````
<!-- /fragment -->

Both sentences of that comment are consequences rather than choices. The first:
because [*The gate*](02-the-gate.md#the-value-and-the-gate) canonicalises the
root, a caller that reached its file through a symlinked ancestor holds a path
that is not textually under the root, and a crate that only compared strings
would tell that caller its own workspace does not contain its own file. The
second: canonicalisation resolves a path by asking the filesystem, and the
filesystem cannot resolve a path that is not there — so canonicalising the whole
path would refuse exactly the case a version control system exists to record, a
file the caller has just deleted and wants the deletion of committed.

<!-- fragment «relative-absolute» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="239-244" parent="scope-tracking-and-commit" -->
````rust
    fn relative(&self, path: &Path) -> Result<String, Refusal> {
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        };
````
<!-- /fragment -->

A relative path is joined onto the root rather than onto the process's working
directory. That is the same decision as the `root:` prefix, made one layer up: a
caller's relative path means *relative to the workspace*, never relative to
wherever the caller happens to be running, and the crate never consults
`current_dir` to interpret one. `an_absolute_path_and_a_root_relative_one_answer_alike`
is the test that the two spellings reach the same answer.

<!-- fragment «relative-strip-or-canonical-parent» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="245-259" parent="scope-tracking-and-commit" -->
````rust
        let relative = match absolute.strip_prefix(&self.root) {
            Ok(relative) => relative.to_path_buf(),
            Err(_) => {
                let parent = absolute
                    .parent()
                    .ok_or_else(|| Refusal::outside_workspace(path, &self.root))?;
                let name = absolute
                    .file_name()
                    .ok_or_else(|| Refusal::outside_workspace(path, &self.root))?;
                canonical(parent)?
                    .strip_prefix(&self.root)
                    .map_err(|_| Refusal::outside_workspace(path, &self.root))?
                    .join(name)
            }
        };
````
<!-- /fragment -->

The textual comparison is tried first and the filesystem is consulted only when
it fails. Ordering it that way is what keeps the common case free: a caller
passing a relative path, or an absolute path with no symlink in it, never touches
the filesystem here at all — and, as a second effect, never has its deleted path
canonicalised, because it never reaches the fallback.

Inside the fallback, `parent` and `file_name` split the path and only the parent
is canonicalised; the name is re-joined afterwards. The exact boundary that buys
was measured against jj 0.44.0 rather than reasoned about, using a workspace
reached through a symlink and a path that had been deleted:

```text
/real/notes/one.md committed, then removed, and named as /link/notes/one.md
  parent /link/notes still exists
  canonical("/link/notes") -> /real/notes, strips to "notes", joined with "one.md"
  -> the deletion commits: `notes/one.md` is gone from the new commit

/real/notes committed, then the whole directory removed, named as /link/notes/one.md
  parent /link/notes no longer exists
  canonical("/link/notes") -> Err(NotFound)
  -> refused: a `.jj` directory was found at /link/notes but the path could not
     be resolved: No such file or directory (os error 2)
```

So the guarantee is precise and narrower than *a deleted path still resolves*:
the path itself may be gone, and its parent may not. That is the right line to
draw — the parent is what locates the name, and a name with nowhere to be is not
a scope — but it is a line worth knowing, because the refusal a caller then sees
is `UnresolvablePath`, which talks about broken symlinks rather than about a
directory the caller has just removed. The crate's own suite reaches this
fallback in one place and by its refusing end:
`a_path_outside_the_workspace_is_refused_rather_than_answered` hands the
workspace an absolute path in the temporary directory *above* the root, so
`strip_prefix` fails, the parent is canonicalised, and the canonical parent is
still not under the root — the third `outside_workspace` of the three. Its
succeeding end is untested. `a_deletion_is_committable_after_the_path_is_gone`,
the test that covers a path that no longer exists, passes a *relative* path and
therefore never leaves the textual branch, so nothing in the suite exercises a
canonicalised parent that does strip; the symlink-and-deletion combination above
is covered by measurement here and by no test.

`Refusal::outside_workspace` is returned three times in this branch, for three
different failures — no parent, no file name, and a canonical parent that is
still not under the root — and that is one refusal for one condition rather than
three cases collapsed carelessly. All three mean the caller named a path this
workspace does not answer for; the message says so and names the root it was
compared against.

<!-- fragment «relative-root-is-not-a-scope» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="260-264" parent="scope-tracking-and-commit" -->
````rust
        if relative.as_os_str().is_empty() {
            return Err(Refusal::not_scoped(
                "the workspace root is not a scope inside itself",
            ));
        }
````
<!-- /fragment -->

An empty relative path is the workspace root itself, and it is refused by the
same rule that refuses an empty `paths` slice. `root:""` would match every file
in the workspace, so accepting it would let a caller widen a path-scoped commit
to the whole working copy by naming the root — the refusal `commit` already makes
explicit, arriving by a second route. The reason string is written for the
condition rather than for the caller's mistake: *the workspace root is not a
scope inside itself*. This guard is unasserted too: no test names the root as a
scope, so the second route is argued from the code and closed by no assertion,
while the first route — the empty slice — is held by
`a_commit_with_no_paths_is_refused_rather_than_widened`.

<!-- fragment «relative-render» owner="no-transactions" source="crates/jj-workspace/src/lib.rs" lines="265-274" parent="scope-tracking-and-commit" -->
````rust
        let mut rendered = String::new();
        for component in relative.components() {
            if !rendered.is_empty() {
                rendered.push('/');
            }
            rendered.push_str(&component.as_os_str().to_string_lossy());
        }
        Ok(rendered)
    }
}
````
<!-- /fragment -->

Components are joined with `/` rather than with the platform separator, because
the string is going into a jj fileset and not back to the operating system. jj's
fileset syntax uses `/`, so a Windows build that emitted `\` would produce a
pattern in which every separator was an escape character — the same two bytes the
quoting loop above is careful about, arriving from the other direction.

`to_string_lossy` is the crate's one lossy conversion, and it is where a path
stops being bytes and becomes text. It has to happen somewhere: a fileset is an
argument in a command line and jj's arguments are text. The consequence is worth
stating plainly, because nothing refuses it — a path whose bytes are not valid
UTF-8 is rendered with replacement characters, and the fileset built from it
names a file that does not exist. What jj does with a fileset that matches
nothing was measured on 0.44.0: `jj file list` prints nothing and exits 0, and
`jj commit` warns on stderr, takes an **empty** commit and exits 0. So
`is_tracked` would answer `false` and `commit` would return a `Commit` naming an
empty change, both without a refusal anywhere. That the lossy rendering can reach
that state is inspection rather than measurement: no test in the suite constructs
a non-UTF-8 path, and the crate has no refusal for one.

<a id="what-this-chapter-settled"></a>
## What this chapter settled

A hundred and thirty-nine lines, and the transaction is not in them. What is in
them is a type holding one identifier, a probe that asks jj about the tree as it
is now, one `jj commit` with a fileset per path, and the addressing that turns a
caller's path into that fileset. The six mechanisms this chapter opened by
enumerating — witness, manifest, rollback proof, index image, quarantine,
recovery path — are absent because jj snapshots before every command and its
operation log keeps every state reachable, which is a claim about jj that a
reader can check in jj's own documentation rather than a claim about this crate
that they would have to take on faith.

The read discipline holds with exactly one exception, and the exception is
argued at the level the rest of the crate is argued at. `is_tracked` lets jj
snapshot because its answer is about the working copy, and the cost of that was
**measured** rather than assumed: an unchanged tree records nothing, and a
changed tree records the one operation the next jj command would have recorded
anyway. Refusal 5's exception is the only place in the crate where a number
settles a design question, and the number is reproducible in three commands.

Two refusals leave this chapter meaning different things, and a consumer that
merges them will either lose a commit or take it twice. `CommitNotRecorded` means
the commit is absent and the working copy still holds the work. An unwrapped
refusal from the identity read means the commit landed and could not be named.
The code distinguishes them by wrapping one and propagating the other, which is
a one-line difference carrying the whole distinction.

And the path algebra turned out to be one argument rather than three functions.
The gate canonicalises the root; therefore `relative` must be able to canonicalise
a caller's path; therefore it canonicalises the **parent**, so a path the caller
has just deleted still resolves — with the measured limit that the parent must
still exist. Every step of that is forced by the step before it, and the first
step was taken two chapters ago for a reason that had nothing to do with
committing.

What remains is what the crate says when any of this declines. Every operation in
this chapter returns `Result<_, Refusal>`, and the refusals have been quoted here
by their messages without the type behind them being read. The next chapter reads
it, and finds that the crate has no remedy of its own to offer.

[Previous: The namespace it will not name](04-namespace.md) | [Contents](README.md) | [Next: Refusal](06-refusal.md)

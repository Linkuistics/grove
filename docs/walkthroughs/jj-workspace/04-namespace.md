# The namespace it will not name
<!-- book-page id="namespace" slice="no-consumer-vocabulary" order="4" -->
[Previous: The subprocess seam](03-subprocess-seam.md) | [Contents](README.md) | [Next: Scope and commit](05-scope-and-commit.md)

<a id="no-consumer-vocabulary"></a>
## The crate has no vocabulary for its consumer

The fourth refusal is the first one that comes with something added. The three
chapters before this one are subtractions — the crate declines a second version
control lane, declines to dispatch on what it resolved, declines to let anything
ambient choose a repository. This chapter owns the crate's single addition to
what jj already gives a caller: one directory, reserved for a consumer's own
coordination files. And the addition is shaped by the same refusal, because the
crate will not name it.

The name is a parameter. `control_dir` takes a `&str`, and every word in the
directory path it returns is either jj's or the caller's; none is the crate's.
That is the whole of what this chapter has to justify, and the justification is
a postcondition. The reserved directory is *inside the workspace*, *never
tracked*, *never shared with another namespace*, and *created if absent* — four
clauses, and not one of them can be stated without a name to attach them to.
Handing back the workspace's administrative directory raw would have avoided the
parameter and lost every clause: the consumer's generic filenames would land
directly in a namespace jj owns and may extend, and *never shared* would be
false the moment a second consumer wrote a file called `lease` beside the first
one's. That is the alternative the repository's module-decomposition
specification rejected in its eighth decision, and the sentence it rejected it
with is the one this chapter is an expansion of: naming the consumer is what
makes the guarantee sayable in the crate's own vocabulary.

The parameter is also what keeps the crate out of a bounded context of its own.
The repository's context map records `jj-workspace` as
deliberately not a fourth context, and the argument turns on this method: every
other term in the crate — workspace, main repo, tracked, commit, change id — is
Jujutsu's, used with Jujutsu's meaning, and *namespace* is the one word the crate
contributes. It gets away with contributing it because a namespace is an
interface property rather than a vocabulary. A crate that knew its consumers
would have to enumerate them, and the enumeration would be a language: an
`enum Namespace { Grove, … }` would put the list of who may coordinate through a
jj workspace inside a crate whose entire subject is jj, make every new consumer
a change to this crate and a new release of it, and give the crate a word — the
name of a consumer — that means nothing to jj at all. The `&str` costs a
validation function, which is the last section of this chapter. The enum would
have cost the fourth refusal itself: a crate holding the list of who may
coordinate through a jj workspace has a vocabulary for its consumers.

**What the reader should be able to check by the end of this chapter is that the
crate never learns what the name means.** The worked example passes the literal
string `"grove"` in and gets a path back, and nothing between those two points
branches on its content beyond four refusals that would apply identically to any
other word.

<a id="the-premise"></a>
## The premise: `.jj/` is jj's, and jj snapshots the working copy

Two behaviours of jj carry this chapter, and both are stated here once.

**The first is that `.jj/` is, in jj's own vocabulary, the repository.** jj's
glossary defines it that way without qualification: a repository is "Basically
everything under `.jj/`"
([jj glossary, *Repo*](https://docs.jj-vcs.dev/latest/glossary/#repo)). So the
directory this crate reserves is placed inside a space whose owner has already
claimed all of it, and the reserved-name list two sections down is the whole of
what the crate does about that. This is the chapter's central tension and it is
not resolved by the crate — it is priced.

**The second is that jj snapshots the working copy.** The glossary again: the
working copy "is automatically snapshot at the beginning of almost every `jj`
command, thus creating a new working-copy commit if any changes had been made"
([jj glossary, *Working copy*](https://docs.jj-vcs.dev/latest/glossary/#working-copy)).
A coordination file written into the tracked working copy is therefore not a
file that happens to be committed later; it is a file that becomes part of the
next commit taken by the very process it coordinates. A lease held during a
commit would be committed by that commit. That is why the reserved directory is
in the administrative area rather than in the tree, and it is a positive reason
rather than a tidiness preference.

What jj actually puts in `.jj/` was checked rather than assumed, on jj 0.44.0,
and the answer has two shapes rather than one. **A colocated workspace holds
three entries:**

```text
.jj/
├── repo/                                       the repository, or a pointer file to a borrowed one
├── working_copy/                               jj's own record of the snapshotted tree
└── .gitignore                                  one line, `/*`
```

**A workspace that is not colocated holds the first two and no third** — the
entry is a Git ignore file, and only a colocated workspace has a Git repository
for it to speak to. Which shape a reader gets from
`jj git init` is a configuration question rather than a flag question:
`git.colocate` **defaults to `true`** on 0.44.0, so a stock `jj git init` and an
explicit `jj git init --colocate` both produce the three-entry shape, and only
`jj git init --config git.colocate=false` produces the two-entry one. That is not
an exotic setting — it is exactly what the crate's own `native` fixture passes
(`crates/jj-workspace/tests/workspace.rs`), whose comment says why: the ambient
config may default colocation on, which would silently turn every native fixture
into a colocated one.

All three of the colocated shape's names are names the crate reserves — and two
of them were the whole list for as long as this book was being written. The third
was added by hand afterwards, because this chapter's own worked example found it
missing, and the gap it left in the meantime is not a detail the chapter can
leave out: it is the one-directional cost the reserved list's own comment prices,
showing up as an observable rather than as a hypothetical. The worked example
ends on it, and on what the reservation does with the name now.

<a id="worked-reservation"></a>
## Worked example: reserving `grove`

The tree is the one [*Orientation*](01-orientation.md#commit-tour) fixed and
[*The gate*](02-the-gate.md#worked-resolution) resolved: a native workspace at
`/work/atlas` holding its own repository. The caller has a resolved `Workspace`,
whose existence is already the proof that `.jj/` was found, and whose `root` is
canonical. The namespace it passes is the literal string `"grove"`.

**First call: the directory does not exist.**

```text
workspace.control_dir("grove")

  validated_namespace("grove")   -> Ok("grove")     not empty, no separator, not `.`/`..`, not jj's
  self.root.join(".jj").join(…)  -> /work/atlas/.jj/grove
  fs::create_dir_all(path)       -> Ok(())          the directory is created
  -> Ok(PathBuf("/work/atlas/.jj/grove"))
```

The tree afterwards, with the caller's first coordination file already written
into it:

```text
/work/atlas/
├── .jj/
│   ├── repo/
│   ├── working_copy/
│   └── grove/                                  returned by control_dir
│       └── driver.lease                        the caller's file, and its name
├── .grove/                                     the task files, tracked, unrelated
└── crates/
```

Two directories in that tree are spelled `grove` and they are not the same
thing. `.grove/` is tracked content in the working copy — the task tree, which
every commit this crate takes will contain. `.jj/grove/` is the reservation, and
nothing in the working copy ever sees it. The crate distinguishes them by
location and not by name, because it has no opinion about either name.

**Second call: the directory exists, and so does what is in it.** The result is
identical and the contents are untouched.

```text
workspace.control_dir("grove")
  fs::create_dir_all(path)       -> Ok(())          already a directory: nothing to do
  -> Ok(PathBuf("/work/atlas/.jj/grove"))
     driver.lease still reads "held"
```

That is `asking_twice_for_one_namespace_gives_the_same_directory_and_keeps_its_contents`
(`crates/jj-workspace/tests/workspace.rs`), which writes a file into the
directory between the two calls and asserts on reading it back afterwards. It is
the test that makes *reserve* rather than *create* the honest verb: the method is
callable on every acquisition without the caller checking first, and a caller
that had to check would have to answer the question the method already answers.

**Three refusals, in the order the checks run.** All three were observed against
this crate on jj 0.44.0 rather than read off the source; the messages below are
what a consumer prints. The empty name is refused first, the path second and the
jj-owned name last, which is the order *Four refusals, in the order they run*
below reads them in — the fourth guard, the one that catches `.` and `..`, sits
between the second and the third and is not one of these three.

```text
control_dir("")
  -> cannot reserve the control namespace ``: it is empty

     A namespace is one plain directory name, owned by the consumer that asks for it
     and kept apart from Jujutsu's own.

control_dir("nested/deeper")
  -> cannot reserve the control namespace `nested/deeper`: it is a path rather than one directory name

     A namespace is one plain directory name, owned by the consumer that asks for it
     and kept apart from Jujutsu's own.

control_dir("repo")
  -> cannot reserve the control namespace `repo`: Jujutsu owns that name inside `.jj`

     A namespace is one plain directory name, owned by the consumer that asks for it
     and kept apart from Jujutsu's own.
```

Each is one `Refusal`, built by a crate-internal constructor `Refusal::namespace`
and carrying two pieces: the name that was asked for, and one short reason. The
second paragraph is fixed and identical across all three, because the remedy is
the same in all three — pick a different word — and *Refusal* is where that habit
of stating a remedy rather than a condition becomes the crate's rule. **Nothing
was created in any of the three cases**, and that is asserted rather than
implied: `a_namespace_that_is_a_path_is_refused` checks that no `elsewhere`
directory appeared beside the workspace after `"../elsewhere"` was refused, and
`a_namespace_jujutsu_owns_is_refused` checks that `.jj/repo` is still jj's own
directory after `"repo"` was refused. A validation that ran after the `join`
would have passed both assertions too; these two tests are what make the ordering
observable from outside.

**A fourth case, which the reserved list did not catch until it was made to.**
This one steps outside the carried tree, because the two endings it had need two
different shapes of workspace. `/work/atlas` is native and has no
`.jj/.gitignore` for a reservation to collide with; a colocated workspace, which
is what a stock `jj git init` produces, holds the entry. `.gitignore` is a plain
directory name either way: it contains no separator, it is not `.` or `..`, and
while the list held only `repo` and `working_copy` it was not in the list — so
`validated_namespace` accepted it, and the colocated case was refused by the
filesystem instead:

```text
control_dir(".gitignore")                       in a colocated /work/atlas-git, while the list held two names
  -> the control directory /work/atlas-git/.jj/.gitignore is not usable

     It must exist and be writable before anything can coordinate through it.
     Check the permissions on the workspace's `.jj` directory.

     source() -> File exists (os error 17)
```

The reservation still did not happen, so no guarantee was broken; what was lost
was the remedy. A consumer was told to check permissions on a directory whose
permissions are fine, when the true remedy is the one the third refusal above
would have given it — and the `io::Error` on the last line, which the refusal
carries as its cause rather than in its message
([*Refusal*](06-refusal.md#the-cause-chain)), says *File exists* rather than
anything about permissions, so the two halves of what a consumer was handed
disagreed with each other. In a native workspace it was worse rather than absent: there
the call succeeded, and the consumer was handed a directory standing on a name jj
had not used in that tree yet and writes a file to the moment the tree is
colocated. That is exactly the collision the reserved list's comment prices as
the cost of a name jj adds and the list has not heard of, priced before it was
observed rather than explained after the fact. Neither ending is reachable now.
What jj does is unchanged: the entry was measured on jj 0.44.0 and re-measured on
0.45.1 when the name was added, and every other measurement in this book is
0.44.0's.

`.gitignore` is the third name in the list because of this case, and the call now
ends in the third refusal above with `.gitignore` in place of `repo`:

```text
control_dir(".gitignore")                       in any workspace, colocated or not
  -> cannot reserve the control namespace `.gitignore`: Jujutsu owns that name inside `.jj`

     A namespace is one plain directory name, owned by the consumer that asks for it
     and kept apart from Jujutsu's own.
```

**In any workspace** is the over-reservation the comment licenses, taken
deliberately: a native tree has no `.jj/.gitignore`, nothing there would have
collided, and a consumer that wanted the word is refused one it has an infinite
supply of alternatives to. That is the cheap direction, chosen over a guard that
would have had to ask which shape of workspace it was standing in.
`the_git_ignore_jujutsu_writes_is_refused` is the test, and it stands in a
**colocated** fixture on purpose: that is the one shape in which a filesystem
refusal and a reserved-list refusal can be told apart, so it is the only fixture
in which the assertion means what it says. It makes three assertions, and the
first is the one that earns the fixture: that jj really did write a
`.jj/.gitignore` there. Then that the refusal names Jujutsu, and that jj's own
file is byte-identical afterwards.

**None of that was fixed on this page.** The corpus is frozen for this book, so
the missing name was recorded here and carried as its own work item; the source
change, this page and a green validator run over the whole book landed together
in one commit, which is the only shape in which a corrected crate and a
byte-exact book are both true at once.

> **The consumer's half.** The string is `"grove"` because
> `crates/grove-loop/src/driver_lease.rs` holds it in a constant and passes it at
> one call site — one constant rather than a literal per site, because two
> spellings of the namespace are two directories and the second driver would not
> see the first one's lease. What grove keeps in the directory is the
> [driver lease](../../../CONTEXT.md#driver-lease) — the advisory lock that makes
> one live driver per working tree true — a session-epoch file beside it, and the
> per-launch [loop control channel](../../../CONTEXT.md#loop-control-channel)
> files whose appearance ends a session. The guide's account of what a user sees
> when the lease is held is [*One driver per working
> tree*](../../USAGE.md#usage-driver-lease). Every one of those words is grove's;
> the crate's four clauses are the whole of what it is told, and *never shared*
> is the clause the lease depends on, because a lease visible to a second
> workspace would be a lock over the wrong thing.

<a id="the-reserved-list"></a>
## The three names jj owns, and why the list is allowed to be incomplete

The list and the argument for it head the file, immediately after the imports
[*Orientation*](01-orientation.md#crate-thesis) owns, and before every type in the
crate. Its position is not accidental: it is data the last function in the file
consults, and putting it at the top makes the crate's one piece of foreign
knowledge visible to a reader who opens the file and reads nothing else.

<!-- fragment «namespace-reserved-names» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="55-61" parent="source-library" -->
<!-- insert «namespace-owned-names-argument» -->
<!-- insert «namespace-owned-names-list» -->
<!-- /fragment -->

The comment spends five lines on a three-element array, and four of them are one
argument. The argument is that the list does not have to be right, only
*cheaply wrong in one direction*, and it is worth reading as the general shape it
is: a crate that must model a foreign system's private namespace can either track
it exactly or bound the damage of being out of date, and only the second is
achievable without an interface the foreign system does not offer.

<!-- fragment «namespace-owned-names-argument» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="55-59" parent="namespace-reserved-names" -->
````rust
/// Names inside `.jj/` that belong to Jujutsu, and so cannot be handed to a
/// consumer as a control namespace. Refusing them is cheap and one-directional:
/// the cost of a name jj adds later and this list has not heard of is a
/// collision, and the cost of a name listed here that jj drops is a consumer
/// picking a different word.
````
<!-- /fragment -->

Both directions are priced, and they are not symmetric. **A name jj adds that
this list has not heard of costs a collision** — the `.gitignore` case above,
where a consumer asking for a name jj had quietly started using got a refusal
naming the wrong remedy, and, in the shape where `create_dir_all` had nothing to
trip over, a directory standing on a name jj would take back. **A name listed
here that jj drops costs a consumer a different word** — it asks for
`working_copy`, is refused, and calls its namespace something else. The first cost
is paid by a consumer at the moment of failure and is hard to diagnose; the
second is paid once, at design time, by a consumer who has an infinite supply of
other words. So the list is allowed to over-reserve freely and is dangerous only
when it under-reserves, and that asymmetry is why it is a hand-maintained
constant rather than an attempt at completeness.

The alternative would have been to ask jj. There is no interface to ask with: jj
documents no command that enumerates the names it owns inside `.jj/`, and a crate
that shelled out to discover them would be reading a private layout jj has never
promised to keep stable — which is a stronger dependency on jj's internals than
the three literals below, not a weaker one. Reserving every name beginning with a
dot was the other option, and it is the one that would have caught `.gitignore`
in advance rather than after the fact — at the price of refusing a consumer every
dotted word for no reason jj has given, and without generalising, because two of
the three entries a colocated workspace holds do not begin with a dot and a
native one holds neither a third entry nor a dotted name. The list bought the
narrower rule and paid for it once, in the currency its own comment named.

<!-- fragment «namespace-owned-names-list» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="60-61" parent="namespace-reserved-names" -->
````rust
const JJ_OWNED_NAMES: [&str; 3] = ["repo", "working_copy", ".gitignore"];

````
<!-- /fragment -->

`[&str; 3]` rather than `&[&str]`, so the count is in the type — and it is a `3`
rather than a `2` because the list was made current by hand rather than by
anything that could have noticed on its own. It is the same choice the seam makes
for its four environment variables
([*The subprocess seam*](03-subprocess-seam.md#the-selectors)), and the two lists
are worth reading together because they are lists of opposite things. A reader
who merges them in memory will mis-remember both, so they are set side by side
here:

| Constant | Declared in | Holds names that | What the crate does with them |
|---|---|---|---|
| `REPOSITORY_SELECTORS` | `jj.rs` | **Git reads**, ahead of the working directory | removes them from every child it starts |
| `JJ_OWNED_NAMES` | `lib.rs` | **jj owns**, inside `.jj/` | refuses them as a consumer's namespace |

Those are the crate's only two pieces of hardcoded knowledge about software it
does not contain.

<a id="the-reservation"></a>
## The reservation, and the four clauses it promises

`control_dir` belongs to `Workspace`'s public surface
([*Orientation*](01-orientation.md#public-surface)), and it
sits between the two accessors
[*The gate*](02-the-gate.md#the-value-and-the-gate) owns and the
discovery and scope-and-commit operations. Each paragraph of its comment answers a different
question a reader would otherwise have to ask the tests.

<!-- fragment «namespace-control-dir» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="119-144" parent="source-library" -->
<!-- insert «namespace-postcondition» -->
<!-- insert «namespace-placement» -->
<!-- insert «namespace-shape» -->
<!-- insert «namespace-no-probe» -->
<!-- insert «namespace-control-dir-body» -->
<!-- /fragment -->

The first paragraph is the postcondition, and it is the sentence the whole
chapter exists to justify. Four clauses, in the order they are argued below:
inside the workspace, never tracked, never shared with another namespace, created
if absent. It is stated as a property of a directory rather than as a description
of what the method does, which is what makes it checkable — a test can assert
each clause, and four of them do.

<!-- fragment «namespace-postcondition» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="119-123" parent="namespace-control-dir" -->
````rust

    /// A directory this workspace reserves for `namespace`'s own untracked
    /// coordination files: inside the workspace, never tracked, never shared
    /// with another namespace, and created if absent.
    ///
````
<!-- /fragment -->

The second paragraph says what the namespace buys and where the directory
therefore has to live. *The consumer's filenames are its own and cannot collide
with jj's* is the benefit; it is a statement about the caller's freedom rather
than about safety, and the caller's freedom is the point, because a consumer that
had to prefix every file with its own name to stay clear of jj would be doing the
namespace's work by hand. The placement argument is the premise of this chapter
applied: the administrative directory rather than the tracked working copy,
*because* jj snapshots the working copy on the next command.

<!-- fragment «namespace-placement» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="124-129" parent="namespace-control-dir" -->
````rust
    /// The consumer's filenames are its own and cannot collide with jj's, which
    /// is the whole of what the namespace buys. It lives in the workspace's
    /// administrative directory rather than in the tracked working copy because
    /// jj snapshots the working copy on the next command, which would make a
    /// coordination file an artifact of the work it coordinates.
    ///
````
<!-- /fragment -->

The phrase to read closely is the last one — a coordination file in the working
copy would become *an artifact of the work it coordinates*. That is the failure
in one line, and it is circular in a way that matters: a lease taken to guard a
commit would be committed by the commit it guarded, so the record of the work
would contain the record of the guard, and every later reader of the history
would see a lease that was never meant to outlive a process. The claim that the
placement prevents it is asserted by
`a_control_directory_is_not_tracked_even_after_the_workspace_is_snapshotted`
(`crates/jj-workspace/tests/workspace.rs`), and the test is built so that its
green result means something: it writes a file into the reserved directory *and*
a file into the working copy, then runs `jj status` purely to force the snapshot,
then requires `is_tracked` to answer no for the first and yes for the second. The
second half is the control. Without it the test would pass in a workspace where
nothing at all was tracked. `is_tracked` is the probe
*Scope and commit* owns; the minimum needed here is that
it is the one question in the crate whose answer depends on the working copy, and
so the one that lets jj snapshot before answering.

The third paragraph is the shape of an acceptable name, and the reason the crate
refuses rather than repairs. *Refused rather than quietly reinterpreted* is a
choice with a real alternative — the method could strip separators, or take the
last component of a path, and produce a directory for every input.

<!-- fragment «namespace-shape» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="130-134" parent="namespace-control-dir" -->
````rust
    /// A namespace is one plain directory name. Anything that is a path, or a
    /// name jj already uses inside `.jj/`, is refused rather than quietly
    /// reinterpreted — the guarantee is *never shared*, and a namespace that
    /// escapes or collides cannot keep it.
    ///
````
<!-- /fragment -->

What that alternative would cost is the guarantee itself, and the comment says so
in the clause it names: *the guarantee is never shared, and a namespace that
escapes or collides cannot keep it*. Two callers asking for `a/x` and `b/x` would
be reinterpreted into one directory, and each would believe it held a namespace of
its own. Repairing input silently converts a caller's mistake into a
crate-guaranteed falsehood, and there is no message anywhere to notice it by.
Refusing converts the same mistake into one sentence naming the word that was
wrong.

The fourth paragraph is an argued absence, in the pattern the seam established:
the crate states what it deliberately did not build, at the site where a reader
would otherwise wonder.

<!-- fragment «namespace-no-probe» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="135-138" parent="namespace-control-dir" -->
````rust
    /// Creation is all this promises. It does not write a probe file to prove
    /// the directory writable: the first coordination file the consumer opens
    /// there proves it, at the moment the answer matters, and a probe would be
    /// a second answer that can already be stale.
````
<!-- /fragment -->

*Creation is all this promises* is a scope statement, and the writability probe
is the specific thing it excludes. The argument against the probe is not that it
is expensive but that it is **a second answer that can already be stale**: a
probe file written and removed at reservation time proves the directory was
writable then, and the consumer's first real file is what proves it is writable
when it matters. Between those two moments a permission can change, a filesystem
can be remounted read-only, and a quota can fill. The probe's success would
therefore be a claim the crate cannot keep, and its failure would be a refusal
raised at a moment nothing was being coordinated. The consumer's own first
`File::create` answers the same question, later, and with a failure the consumer
can attribute to the operation it was performing. That the directory *may still*
be unusable is not hidden — `Refusal::control_dir` exists precisely for the
creation failing — it is only that the crate does not manufacture an occasion for
it.

The creating operation has three steps:
`control_dir` turns the caller's namespace string into a path under the
workspace's `.jj/` that exists by the time it is returned, validating before it
joins and joining before it creates, which is what keeps a refused name from
reaching the filesystem at all. It is the first call of the worked example
above, at the resolution the trace showed.

<!-- fragment «namespace-control-dir-body» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="139-144" parent="namespace-control-dir" -->
````rust
    pub fn control_dir(&self, namespace: &str) -> Result<PathBuf, Refusal> {
        let path = control_path(&self.root, namespace)?;
        fs::create_dir_all(&path).map_err(|cause| Refusal::control_dir(&path, cause))?;
        Ok(path)
    }

````
<!-- /fragment -->

Both namespace operations call `control_path`, which validates the supplied
name before joining it beneath the exact root's `.jj/`. This keeps path
construction and the reserved-name rule shared as discovery is added. The
returned path does not pin a directory: the consumer must still check the
identity of files it opens, and administration-area symlink corruption is not
prevented by joining validated path components.

`fs::create_dir_all` is what makes *created if absent* idempotent rather than
merely convenient. It succeeds when the directory already exists, which is the
second call in the worked example above; it creates intermediate directories,
which here means it would create `.jj` itself if a resolved workspace somehow
lacked one, and cannot happen because the gate already proved `.jj/` present.
Its `io::Error` becomes `Refusal::control_dir`, carrying the path and keeping the
error as the refusal's `source()` so a consumer walking the chain sees what the
operating system said.

The *never shared* clause is the one nothing in this function checks, and it does
not need to be checked: two namespaces are two names, two names are two paths,
and two paths are two directories. `two_namespaces_do_not_share_a_directory`
asserts it directly, and
`a_secondary_workspace_gets_its_own_control_directory_not_the_shared_one`
asserts the harder half — that two workspaces sharing one repository get
different control directories, because the path is derived from `self.root` and a
secondary workspace's root is its own even though its `.jj/repo` points elsewhere.
That is the through-line from [*The gate*](02-the-gate.md#pointer-or-repository)
arriving here: the crate's decision to keep the workspace root and the main repo
as two separate fields is what makes this method's answer right for a secondary
workspace, and a `control_dir` derived from `main_repo` instead would have handed
two drivers one lease.

<a id="read-only-discovery"></a>
## Discovering an existing namespace without reserving it

A read-only consumer starts with a location, before resolving a `Workspace`.
`Workspace::discover_control_dir(location, "grove")` checks only that location's
`.jj/` and namespace. A missing directory returns `None`, so browsing a temporary
non-jj tree cannot create administration state. A subdirectory never borrows an
ancestor's namespace. Canonicalising the exact root accepts aliases such as
`/var` and `/private/var` without following `.jj/repo` or invoking jj.

For the carried workspace, discovery returns `/work/atlas/.jj/grove` after the
reservation above, and `None` before it. The result is a path sample, not a
workspace value or an ownership guard; a caller opening coordination files must
validate their descriptors against their current paths. No lock is held here.

<!-- fragment «namespace-discovery» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="145-168" parent="source-library" -->
````rust
    /// Discover an existing namespace at this exact location without writing.
    ///
    /// Unlike [`Self::resolve`], this neither walks ancestors nor follows a
    /// secondary workspace's repository pointer, and never invokes jj. Missing
    /// `.jj` or namespace directories return `None`; inspection failures and
    /// nondirectories are refused. Workspace aliases produce the same path.
    /// Namespace validation is identical to [`Self::control_dir`].
    ///
    /// This is a path discovery, not an identity pin: a consumer opening files
    /// here must validate its descriptors against the current paths. No file
    /// is opened or locked, and no directory or other state is created.
    pub fn discover_control_dir(
        location: &Path,
        namespace: &str,
    ) -> Result<Option<PathBuf>, Refusal> {
        let path = control_path(location, namespace)?;
        for directory in [location.join(".jj"), path] {
            if !existing_directory(&directory)? {
                return Ok(None);
            }
        }
        control_path(&canonical(location)?, namespace).map(Some)
    }

````
<!-- /fragment -->

The shared `control_path` maps a location and validated namespace to one path.
Creation supplies an already-resolved root; discovery supplies its exact input
and later canonicalises it. Neither operation can accidentally use the default
workspace's repository location to derive the namespace.

<!-- fragment «namespace-path» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="359-362" parent="source-library" -->
````rust
fn control_path(root: &Path, namespace: &str) -> Result<PathBuf, Refusal> {
    Ok(root.join(".jj").join(validated_namespace(namespace)?))
}

````
<!-- /fragment -->

`existing_directory` distinguishes absence from corruption. Its first metadata
query does not follow a final symlink, so a dangling namespace link is present
and fails the second query rather than silently becoming `None`. The second
query requires a directory. A regular file or FIFO is refused without opening
or reading it. Inspection failures retain their operating-system cause in
`Refusal::control_dir`; the remedy covers types and permissions, with write
access required only for writers.

<!-- fragment «namespace-existing-directory» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="363-380" parent="source-library" -->
````rust
fn existing_directory(path: &Path) -> Result<bool, Refusal> {
    // lstat distinguishes an absent entry from a broken symlink. Neither stat
    // operation opens a FIFO or reads any directory contents.
    match fs::symlink_metadata(path) {
        Err(cause) if cause.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(cause) => return Err(Refusal::control_dir(path, cause)),
        Ok(_) => {}
    }
    let metadata = fs::metadata(path).map_err(|cause| Refusal::control_dir(path, cause))?;
    if !metadata.is_dir() {
        return Err(Refusal::control_dir(
            path,
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "expected a directory"),
        ));
    }
    Ok(true)
}

````
<!-- /fragment -->

The public discovery tests use an invalid secondary-repository pointer: discovery
still succeeds because it never asks jj to interpret the pointer. They compare
filesystem entries and bytes, including `.jj`, before and after discovery,
check aliases and invalid namespaces, and bound the FIFO test with a timeout.
These tests establish the path operation's read-only behavior; runtime record
locks and descriptor identity validation belong to the consumer. Grove's
runtime observer now pins its worktree before discovery, validates the returned
namespace and record descriptors against their paths, and takes only a bounded
shared epoch guard. The workspace seam still supplies no lock or authority.

<a id="the-validation"></a>
## Four refusals, in the order they run

The last function in `lib.rs` is a free function, private, and consulted from
exactly one call site. Its shape is four guards and an accept, and the order of
the guards is load-bearing in a way a reader who skims it will miss: the first
and third are the two that prevent `control_dir` from returning a directory that
already means something.

<!-- fragment «namespace-validation» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="381-404" parent="source-library" -->
<!-- insert «namespace-validation-empty» -->
<!-- insert «namespace-validation-path» -->
<!-- insert «namespace-validation-self-reference» -->
<!-- insert «namespace-validation-owned» -->
<!-- insert «namespace-validation-accept» -->
<!-- /fragment -->

The empty check is first, and it is not a formality about degenerate input.
`Path::join("")` yields the path it was called on with a trailing separator, so
without this guard `control_dir("")` would compute `/work/atlas/.jj/`, find it
already a directory, and return jj's entire administrative area to the caller as
its private namespace — the exact outcome decision 8 named when it rejected
returning the administrative directory raw. Verified on this crate at jj 0.44.0:
`Path::new("/work/atlas").join(".jj").join("")` is `/work/atlas/.jj/`, and
`create_dir_all` on an existing directory succeeds.

<!-- fragment «namespace-validation-empty» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="381-384" parent="namespace-validation" -->
````rust
fn validated_namespace(namespace: &str) -> Result<&str, Refusal> {
    if namespace.is_empty() {
        return Err(Refusal::namespace(namespace, "it is empty"));
    }
````
<!-- /fragment -->

The signature is the shadowing contract described above: `&str` in, the same
`&str` back on success, with the lifetime tying the accepted value to the caller's
string. There is no owned `String` anywhere in this function, and there is no
allocation on the accept path at all.

The second check is the one a reader expects to be the whole function: a name
containing a separator is a path, not a name.

<!-- fragment «namespace-validation-path» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="385-390" parent="namespace-validation" -->
````rust
    if namespace.contains('/') || namespace.contains('\\') || namespace.contains('\0') {
        return Err(Refusal::namespace(
            namespace,
            "it is a path rather than one directory name",
        ));
    }
````
<!-- /fragment -->

Three characters, and only the first is a separator on the platform this crate
runs on. `'\\'` is refused because it is a separator on Windows, and refusing it
everywhere means a namespace has the same meaning on both — a crate that accepted
`a\b` on Unix would be defining a name that becomes two components if the same
consumer is ever built for the other platform. `'\0'` is refused because it cannot
be in a path at all: `create_dir_all` on a name containing one fails with
`InvalidInput` and the message *file name contained an unexpected NUL byte*,
which would reach the consumer as a `ControlDir` refusal telling it to check
permissions. Checking here converts an obscure operating-system error into the
namespace refusal that names the actual problem. Neither of the two is exercised
by the crate's suite — `a_namespace_that_is_a_path_is_refused` covers `".."`,
`"../elsewhere"`, `"nested/deeper"` and `""`, so it reaches this guard only
through `/` — and both were checked directly instead, on jj 0.44.0. The NUL case
is worth one further note: the refusal message interpolates the name as given, so
a NUL reaches the consumer's terminal inside the quoted name.

The third check is the one whose absence would be silent, and it exists because
`.` and `..` are names rather than paths — neither contains a separator, so the
guard above lets both through.

<!-- fragment «namespace-validation-self-reference» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="391-396" parent="namespace-validation" -->
````rust
    if namespace == "." || namespace == ".." {
        return Err(Refusal::namespace(
            namespace,
            "it names a directory other than itself",
        ));
    }
````
<!-- /fragment -->

`join("..")` yields `/work/atlas/.jj/..`, which resolves to the workspace root:
`create_dir_all` succeeds on it because it exists, and the caller would receive
the *tracked working copy* as its untracked namespace — the *never tracked*
clause inverted rather than merely weakened. `join(".")` yields `/work/atlas/.jj/.`
and lands on the administrative directory, the same outcome as the empty name.
Both were verified rather than reasoned about. One shared reason string covers the
two, and it is written for `..`: *it names a directory other than itself* reads
exactly for the parent and only loosely for `.`, whose hazard is that it names
the directory the namespace was supposed to be created *inside*. The refusal is
correct in both cases; the sentence is precise in one.

The jj-owned check is last, and its position is the only one of the four that
could be argued either way. It is last because it is the only guard whose answer
can change without this crate changing — a jj release adds a name and the array
is what has to move — and keeping the structural checks ahead of it means a
malformed name is always reported as malformed rather than as a collision. It is
also the only guard that consults data rather than the string's own shape.

<!-- fragment «namespace-validation-owned» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="397-402" parent="namespace-validation" -->
````rust
    if JJ_OWNED_NAMES.contains(&namespace) {
        return Err(Refusal::namespace(
            namespace,
            "Jujutsu owns that name inside `.jj`",
        ));
    }
````
<!-- /fragment -->

`JJ_OWNED_NAMES.contains(&namespace)` is a linear scan of three elements, which
is the right shape at this size and would remain so at ten; the array is the data
structure because the list is small, fixed at compile time, and never searched in
a loop. The reason string names Jujutsu explicitly rather than saying the name is
taken, because a consumer reading *Jujutsu owns that name inside `.jj`* knows both
why it cannot have the word and that no version of its own code will ever get it.
`a_namespace_jujutsu_owns_is_refused` asserts on that phrase, and then asserts
that `.jj/repo` is still a directory afterwards — the refusal must not have
disturbed jj's own.

<!-- fragment «namespace-validation-accept» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="403-404" parent="namespace-validation" -->
````rust
    Ok(namespace)
}
````
<!-- /fragment -->

The accept is the borrowed name back, and the four guards above are the entire
specification of an acceptable namespace. **What is not checked is as deliberate
as what is.** There is no length limit, no character-set restriction, no
lower-casing, no rejection of leading dots, and no reservation of names this crate
or its consumers might want later. Each of those would be the crate having an
opinion about a word whose meaning belongs to the caller, and the four that
remain are exactly the ones that would break a clause of the postcondition. Read
in the order they run, each guard is one name the `join` would otherwise have
resolved to something that already means something:

| # | Guard | A name it refuses | What the `join` would have produced instead | Evidence |
|---:|---|---|---|---|
| 1 | empty | `""` | `/work/atlas/.jj/` — jj's whole administrative area | verified on jj 0.44.0; `a_namespace_that_is_a_path_is_refused` covers `""` |
| 2 | contains `/`, `\` or `\0` | `nested/deeper`, `../elsewhere` | a path under or outside `.jj/` rather than one component | `a_namespace_that_is_a_path_is_refused`, through `/` only; `\` and `\0` checked directly on jj 0.44.0 |
| 3 | `.` or `..` | `..` | `/work/atlas/.jj/..` — the tracked working copy | verified on jj 0.44.0; `a_namespace_that_is_a_path_is_refused` covers `".."` |
| 4 | in `JJ_OWNED_NAMES` | `repo` | `/work/atlas/.jj/repo` — a directory jj owns | `a_namespace_jujutsu_owns_is_refused`; `the_git_ignore_jujutsu_writes_is_refused` reaches the same guard through `.gitignore` |

Nothing downstream catches any of the four. `create_dir_all` succeeds on every
path in the fourth column — three of them already exist and the remaining one it
would create — so the guards are the whole of the check, and their position ahead
of the `join` is what the worked example's three refusal tests make observable
from outside.

<a id="what-this-chapter-settled"></a>
## What this chapter settled

The crate's one addition is one directory, and the crate does not know what it is
for. Fifty-eight lines carry it: a three-name array with a four-line argument
about being wrong cheaply, a four-line method under a nineteen-line comment, and a
four-guard validator whose ordering keeps a caller from being handed `.jj/` or
the working copy under the name of a namespace. The postcondition is four clauses,
three of which hold by construction — the path is built from a canonical root and
a single component, the location is outside the snapshotted tree, and two names
are two directories — and the fourth, *created if absent*, is one idempotent call.

The tension the chapter opened on is not closed, and the crate does not pretend
otherwise: `.jj/` is jj's by jj's own definition, and this crate puts something
else there. What it does instead is price the intrusion — over-reserve freely,
under-reserve dangerously, and say which — and the list has now been on both
sides of that price: it was one name short of what a colocated workspace
contains, with the observable consequence recorded above, and it is one name
longer than a native one needs now that the third name is in it. Only the first
of those two cost a consumer anything, which is the argument holding rather than
the argument being spared a test.

Everything so far has been the crate declining to decide, and this chapter was
the crate adding one thing without deciding what it means. The next chapter is
the crate declining to build something jj already has: a transaction.

[Previous: The subprocess seam](03-subprocess-seam.md) | [Contents](README.md) | [Next: Scope and commit](05-scope-and-commit.md)

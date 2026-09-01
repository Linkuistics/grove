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
with is the one this chapter is an expansion of: naming the consumer is what makes the guarantee sayable in the
crate's own vocabulary.

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
have cost the spine.

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

What jj actually puts in `.jj/` was checked rather than assumed, on jj 0.44.0. A
workspace created by `jj git init` and one created by `jj git init --colocate`
both contain exactly three entries:

```text
.jj/
├── repo/                                       the repository, or a pointer file to a borrowed one
├── working_copy/                               jj's own record of the snapshotted tree
└── .gitignore                                  one line, `/*`
```

Two of those three names are the two the crate reserves. The third is not, and
the gap is not a detail this chapter can leave out: it is the one-directional
cost the reserved list's own comment prices, showing up as an observable. The
worked example ends on it.

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
│   ├── .gitignore
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
what a consumer prints.

```text
control_dir("nested/deeper")
  -> cannot reserve the control namespace `nested/deeper`: it is a path rather than one directory name

     A namespace is one plain directory name, owned by the consumer that asks for it
     and kept apart from Jujutsu's own.

control_dir("")
  -> cannot reserve the control namespace ``: it is empty

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

**A fourth case, which the reserved list does not catch.** `.gitignore` is a
plain directory name, it contains no separator, and it is not in the crate's
list — so `validated_namespace` accepts it, and the refusal comes from the
filesystem instead:

```text
control_dir(".gitignore")
  -> the control directory /work/atlas/.jj/.gitignore is not usable: File exists (os error 17)

     It must exist and be writable before anything can coordinate through it.
     Check the permissions on the workspace's `.jj` directory.
```

The reservation still does not happen, so no guarantee is broken; what is lost is
the remedy. A consumer is told to check permissions on a directory whose
permissions are fine, when the true remedy is the one the third refusal above
would have given it. This is exactly the collision the reserved list's comment
prices as the cost of a name jj adds and the list has not heard of, and it is
already outstanding on jj 0.44.0 rather than hypothetical. No test in the crate's
suite covers it. The corpus is frozen for this book, so it is recorded here and
carried as its own work item rather than fixed on the page.

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
## The two names jj owns, and why the list is allowed to be incomplete

The list and the argument for it head the file, immediately after the imports
[*Orientation*](01-orientation.md#crate-thesis) owns, and before every type in the
crate. Its position is not accidental: it is data the last function in the file
consults, and putting it at the top makes the crate's one piece of foreign
knowledge visible to a reader who opens the file and reads nothing else.

<!-- fragment «namespace-reserved-names» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="55-61" parent="source-library" -->
<!-- insert «namespace-owned-names-argument» -->
<!-- insert «namespace-owned-names-list» -->
<!-- /fragment -->

The comment spends five lines on a two-element array, and four of them are one
argument. The argument is that the list does not have to be right, only
*cheaply wrong in one direction*, and it is worth reading as the general shape it
is: a crate that must model a foreign system's private namespace can either track
it exactly or bound the damage of being out of date, and only the second is
achievable without a interface the foreign system does not offer.

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
where a consumer asking for a name jj has quietly started using gets a refusal
naming the wrong remedy, or, in the worse shape the crate is protected from only
by `create_dir_all` failing, would get a directory it shares with jj. **A name
listed here that jj drops costs a consumer a different word** — it asks for
`working_copy`, is refused, and calls its namespace something else. The first
cost is paid by a consumer at the moment of failure and is hard to diagnose; the
second is paid once, at design time, by a consumer who has an infinite supply of
other words. So the list is allowed to over-reserve freely and is dangerous only
when it under-reserves, and that asymmetry is why it is a hand-maintained
constant rather than an attempt at completeness.

The alternative would have been to ask jj. There is no interface to ask with: jj
documents no command that enumerates the names it owns inside `.jj/`, and a crate
that shelled out to discover them would be reading a private layout jj has never
promised to keep stable — which is a stronger dependency on jj's internals than
the two literals below, not a weaker one. Reserving every name beginning with a
dot was the other option, and it would have caught `.gitignore` while refusing a
consumer a perfectly good word for no reason jj has given; it also would not
generalise, because two of jj's three entries do not begin with a dot.

<!-- fragment «namespace-owned-names-list» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="60-61" parent="namespace-reserved-names" -->
````rust
const JJ_OWNED_NAMES: [&str; 2] = ["repo", "working_copy"];

````
<!-- /fragment -->

`[&str; 2]` rather than `&[&str]`, so the count is in the type. It is the same
choice the seam makes for its four environment variables
([*The subprocess seam*](03-subprocess-seam.md#the-selectors)), and the two lists
are worth reading together because they are lists of opposite things: that one
holds names **Git reads**, and this one holds names **jj owns**. A reader who
merges them in memory will mis-remember both. The crate has exactly two pieces of
hardcoded knowledge about software it does not contain, and they are these.

<a id="the-reservation"></a>
## The reservation, and the four clauses it promises

`control_dir` is one of five methods on `Workspace`, and it sits between the two
accessors [*The gate*](02-the-gate.md#the-value-and-the-gate) owns and the
scope-and-commit operations the next chapter owns. Nineteen lines of its
twenty-seven are comment, and each paragraph of that comment answers a different
question a reader would otherwise have to ask the tests.

<!-- fragment «namespace-control-dir» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="119-145" parent="source-library" -->
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

The body is four lines, and their order is the whole of what they do.

<!-- fragment «namespace-control-dir-body» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="139-145" parent="namespace-control-dir" -->
````rust
    pub fn control_dir(&self, namespace: &str) -> Result<PathBuf, Refusal> {
        let namespace = validated_namespace(namespace)?;
        let path = self.root.join(".jj").join(namespace);
        fs::create_dir_all(&path).map_err(|cause| Refusal::control_dir(&path, cause))?;
        Ok(path)
    }

````
<!-- /fragment -->

Validation comes first, and its result **shadows the parameter**:
`let namespace = validated_namespace(namespace)?` rebinds the name, so the
unvalidated `&str` is not reachable in the two lines that follow. That is a
deliberate use of shadowing as an enforcement rather than as brevity — the
function returns `Result<&str, Refusal>` rather than `Result<(), Refusal>` for
exactly this reason, and the returned reference borrows from the argument, so
the rebinding costs nothing at runtime and makes the wrong value impossible to
join by hand. A validator returning `()` would have left the original in scope
and the ordering would then be a convention.

`self.root.join(".jj").join(namespace)` is the only place in the crate that names
`.jj` as a path component in an operation rather than in a probe, and it is safe
in the way this crate cares about because both of its inputs are already
constrained: `self.root` is canonical — the gate canonicalised it, so there is no
symlink or `..` left in it to resolve — and `namespace` has been validated to be
a single component. A `join` of a canonical path and a validated single component
cannot leave the workspace, which is the *inside the workspace* clause discharged
by construction rather than by a check on the result.

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

<a id="the-validation"></a>
## Four refusals, in the order they run

The last function in `lib.rs` is a free function, private, and consulted from
exactly one call site. Its shape is four guards and an accept, and the order of
the guards is load-bearing in a way a reader who skims it will miss: the first
and third are the two that prevent `control_dir` from returning a directory that
already means something.

<!-- fragment «namespace-validation» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="320-343" parent="source-library" -->
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

<!-- fragment «namespace-validation-empty» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="320-323" parent="namespace-validation" -->
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

<!-- fragment «namespace-validation-path» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="324-329" parent="namespace-validation" -->
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

<!-- fragment «namespace-validation-self-reference» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="330-335" parent="namespace-validation" -->
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

<!-- fragment «namespace-validation-owned» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="336-341" parent="namespace-validation" -->
````rust
    if JJ_OWNED_NAMES.contains(&namespace) {
        return Err(Refusal::namespace(
            namespace,
            "Jujutsu owns that name inside `.jj`",
        ));
    }
````
<!-- /fragment -->

`JJ_OWNED_NAMES.contains(&namespace)` is a linear scan of two elements, which is
the right shape at this size and would remain so at ten; the array is the data
structure because the list is small, fixed at compile time, and never searched in
a loop. The reason string names Jujutsu explicitly rather than saying the name is
taken, because a consumer reading *Jujutsu owns that name inside `.jj`* knows both
why it cannot have the word and that no version of its own code will ever get it.
`a_namespace_jujutsu_owns_is_refused` asserts on that phrase, and then asserts
that `.jj/repo` is still a directory afterwards — the refusal must not have
disturbed jj's own.

<!-- fragment «namespace-validation-accept» owner="no-consumer-vocabulary" source="crates/jj-workspace/src/lib.rs" lines="342-343" parent="namespace-validation" -->
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
remain are exactly the ones that would break a clause of the postcondition:
empty and `.` reach the administrative directory, `..` reaches the tracked tree,
a separator escapes the component, and a jj-owned name collides with the system
the whole crate is a seam onto.

<a id="what-this-chapter-settled"></a>
## What this chapter settled

The crate's one addition is one directory, and the crate does not know what it is
for. Fifty-eight lines carry it: a two-name array with a four-line argument about
being wrong cheaply, a four-line method under a nineteen-line comment, and a
four-guard validator whose ordering keeps a caller from being handed `.jj/` or
the working copy under the name of a namespace. The postcondition is four clauses,
three of which hold by construction — the path is built from a canonical root and
a single component, the location is outside the snapshotted tree, and two names
are two directories — and the fourth, *created if absent*, is one idempotent call.

The tension the chapter opened on is not closed, and the crate does not pretend
otherwise: `.jj/` is jj's by jj's own definition, and this crate puts something
else there. What it does instead is price the intrusion — over-reserve freely,
under-reserve dangerously, and say which — and on jj 0.44.0 the list is one name
short of jj's actual contents, with the observable consequence recorded above.

Everything so far has been the crate declining to decide, and this chapter was
the crate adding one thing without deciding what it means. The next chapter is
the crate declining to build something jj already has: a transaction.

[Previous: The subprocess seam](03-subprocess-seam.md) | [Contents](README.md) | [Next: Scope and commit](05-scope-and-commit.md)

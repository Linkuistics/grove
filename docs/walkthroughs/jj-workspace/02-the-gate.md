# The gate
<!-- book-page id="the-gate" slice="one-lane" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: The subprocess seam](03-subprocess-seam.md)

<a id="one-lane"></a>
## There is no second lane

The second refusal is that there is no repository abstraction in this crate and
nothing behind it. jj is the version control system. A working tree with no
`.jj/` directory at or above it is not a case this crate dispatches on; it is a
refusal, returned before anything has been read or changed.

That makes resolution a **precondition gate** rather than a dispatch. A dispatch
takes one input and chooses between implementations, so it needs a second
implementation to choose. This crate has one, and the consequence is visible in
the type system: `Workspace` has private fields and one constructor, `resolve`,
so a `Workspace` value cannot exist for a tree that is not jj-enabled. Every
operation reached through such a value inherits that guarantee and never re-asks.
The question is asked once, at a place that has a name, and the answer is carried
in a value rather than re-derived.

The alternative was a lane per version control system, and it was measured rather
than assumed. `docs/adr/jj-is-the-only-lane.md` records what a Git lane cost:
roughly four thousand lines of durable pre-operation state, proven rollback, an
index image and hooks suppression, all of it hand-built to give Git what jj gives
for free through its automatic snapshot and its operation log. Removing the
second lane removed that machinery, and it is why absence is a stop here rather
than a branch.

> **The consumer's half.** grove resolves the [stated VCS](../../../CONTEXT.md#stated-vcs)
> before a session exists and states it in that session's mandate, which is
> definitive: a session is told the working tree is jj-enabled and where its
> workspace root is, and does not re-derive it. This crate is the whole
> implementation of that statement. None of that vocabulary is available here —
> the crate does not know what a session or a mandate is, and refuses a path
> without ever learning why the path was handed to it.

<a id="pointer-or-repository"></a>
## The premise: a pointer, or the repository itself

One jj behaviour carries the second half of this chapter, and it is stated here
once. Every workspace has a `.jj/` directory. A workspace that holds its own
repository has the repository at `.jj/repo`, as a **directory**. A workspace
created by `jj workspace add` borrows the first workspace's repository, and its
`.jj/repo` is a **file** whose contents are a path to the borrowed one — the same
file-versus-directory shape Git uses for `.git`. jj's own glossary states the
borrowing: "Each workspace has a `.jj/` directory, but the commits and operations
will be stored in the initial workspace; the other workspaces will have pointers
to the initial workspace"
([jj glossary, *Workspace*](https://docs.jj-vcs.dev/latest/glossary/#workspace)).
Confirmed on jj 0.44.0 by inspection: a workspace from `jj git init` has
`.jj/repo` as a directory, and one added beside it has `.jj/repo` as a
nineteen-byte file reading `../../main/.jj/repo`.

The crate reads the file type and never the file. Distinguishing the two cases
is `std::path::Path::is_file`, and following the pointer is jj's job, asked for
by name in the one place resolution may spawn a subprocess.

<a id="worked-resolution"></a>
## Worked example: one tree, three resolutions

The tree is the one *Orientation* fixed, and this chapter follows its resolution
step closely. It is a native workspace at `/work/atlas` — not colocated with Git,
and holding its own repository — with a caller four directories down:

```text
/work/atlas/
├── .jj/
│   ├── repo/                                   a directory: this workspace holds its own
│   └── working_copy/
├── .grove/                                    the task files, elided here
└── crates/
    └── gateway/
        └── src/                                the caller's working directory
```

**First ending: the tree as it is.** The walk asks each ancestor of the given
path for a `.jj/` child and takes the closest one that has it.

```text
Workspace::resolve("/work/atlas/crates/gateway/src")
  ancestors, closest first:
    /work/atlas/crates/gateway/src   no
    /work/atlas/crates/gateway       no
    /work/atlas/crates               no
    /work/atlas                      yes
  canonical("/work/atlas")                      -> /work/atlas
  main_repo_of("/work/atlas"):
    /work/atlas/.jj/repo is a directory, not a file
    -> no pointer to follow, so the workspace is its own; no jj is spawned
  -> Ok(Workspace { root: "/work/atlas", main_repo: "/work/atlas" })
```

Nothing in that trace consults jj, and nothing in it writes. The walk is the
filesystem's, so no environment variable and no shared repository store can
redirect it — the claim `resolve`'s own doc comment makes, read in full below.
One half of it is exercised:
`resolution_ignores_repository_selection_and_temporary_directory_environment`
(`crates/jj-workspace/tests/environment.rs`) sets all four
repository-selection variables at a foreign colocated repository — three of them
at the repository itself, `GIT_INDEX_FILE` at a path inside it that does not
exist yet — and requires the intended workspace to come back. Exercised rather
than asserted, though, and the trace above is the reason: on a colocated tree the
walk answers alone, so those variables have nothing to act on and that half of
the test would pass with the seam's scrub deleted. The other half is not
exercised at all, because no test constructs a shared repository store;
[*The subprocess seam*](03-subprocess-seam.md#the-selectors) states that test's
scope exactly, and this page's claim is the doc comment's rather than that
test's.

**Second ending: the same tree with no `.jj/`.** The walk reaches the filesystem
root without finding one, and returns instead of falling back.

```text
Workspace::resolve("/work/atlas/crates/gateway/src")
  ancestors, closest first:
    /work/atlas/crates/gateway/src   no
    /work/atlas/crates/gateway       no
    /work/atlas/crates               no
    /work/atlas                      no
    /work                            no
    /                                no
  -> Err(Refusal::not_a_workspace("/work/atlas/crates/gateway/src"))
```

The refusal a consumer prints is the message that constructor carries, and it
names jj's two initialisation commands and then states what did not happen:

```text
not a Jujutsu working tree
  looked for a `.jj` directory at and above: /work/atlas/crates/gateway/src

Make the tree jj-enabled and rerun:
      jj git init --colocate     # an existing Git repository, history kept
      jj git init                # no repository here yet

Nothing was created or changed.
```

The last line is a claim about the whole call, and it is proved rather than
asserted: `a_refused_tree_is_left_exactly_as_it_was`
(`crates/jj-workspace/tests/workspace.rs`) records the tree's entries before the
refusal and compares them afterwards, and
`a_tree_that_is_not_a_workspace_is_refused_with_the_command_that_fixes_it`
checks that the message carries both commands. The refusal's constructors —
`Refusal::not_a_workspace` here, and `Refusal::unresolvable_path` below — are
crate-internal: a consumer receives an opaque `Refusal` with no matchable
variants, because every case is a stop. *Refusal* owns them.

**Third ending: a secondary workspace.** `jj workspace add --name review
/work/atlas-review`, run in `/work/atlas`, creates a second working copy that
borrows the first workspace's repository. Resolving from it is the one shape in
which resolution spawns jj.

```text
Workspace::resolve("/work/atlas-review")
  ancestors, closest first:
    /work/atlas-review               yes
  canonical("/work/atlas-review")               -> /work/atlas-review
  main_repo_of("/work/atlas-review"):
    /work/atlas-review/.jj/repo is a file, so there is a pointer to follow
    jj is spawned in /work/atlas-review with five arguments:
      ["workspace", "root", "--name", "default", "--ignore-working-copy"]
    stdout                                      -> /work/atlas
    canonical("/work/atlas")                    -> /work/atlas
  -> Ok(Workspace { root: "/work/atlas-review", main_repo: "/work/atlas" })
```

The two roots differ, and that difference is the whole answer the probe buys:
`a_secondary_workspace_reports_the_default_workspace_as_its_main_repo`
(`crates/jj-workspace/tests/workspace.rs`) builds exactly this pair and asserts
`root()` is the secondary and `main_repo()` is the main one, while
`a_workspace_that_holds_the_repository_is_its_own_main_repo` asserts the two are
equal in the first ending.

That spawn is not built here. Every jj invocation the crate makes is built at one
seam — `jj::output`, which returns stdout as text, and `jj::produced_output`,
which answers only whether there was any — and that seam fixes the working
directory, removes the four repository-selection variables from the child's
environment, and separates failure to *start* from failure to *succeed*. Here it
is `jj::output`; *The subprocess seam* owns both and shows how an invocation is
assembled.

The three endings differ in one thing the ancestor walk never sees — the shape of
`.jj/repo` — and that is what decides whether resolution costs a child process at
all:

| Ending | `.jj/` found | `.jj/repo` | jj spawned | Result |
|---|---|---|---|---|
| the tree as it is | at `/work/atlas`, the fourth directory tried | a directory | no | `Workspace { root: "/work/atlas", main_repo: "/work/atlas" }` |
| no `.jj/` anywhere | nowhere, up to `/` | — | no | `Refusal::not_a_workspace("/work/atlas/crates/gateway/src")` |
| a secondary workspace | at `/work/atlas-review`, the path itself | a pointer file | once: `jj workspace root --name default --ignore-working-copy` | `Workspace { root: "/work/atlas-review", main_repo: "/work/atlas" }` |

The third row is the only one in which the two stored paths differ, and it is the
only one in which anything is spawned. Those two facts are the same fact: a
borrowed repository is the one case the filesystem cannot answer for.

<a id="the-value-and-the-gate"></a>
## The value, and the gate that constructs it

This chapter owns the `Workspace` value, its one constructor and its two
accessors: forty-seven lines that open the `impl` block *Scope and commit* and
*The namespace it will not name* continue.

<!-- fragment «workspace-value-and-gate» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="72-118" parent="source-library" -->
<!-- insert «gate-workspace-value» -->
<!-- insert «gate-resolve-contract» -->
<!-- insert «gate-resolve-walk» -->
<!-- insert «gate-root-accessor» -->
<!-- insert «gate-main-repo-accessor» -->
<!-- /fragment -->

The type carries two paths and makes both private. Private fields are what turn
"resolution succeeded" into a durable fact rather than a convention: the struct
literal is unavailable outside the crate, so a consumer cannot assemble a
`Workspace` from two paths it likes the look of. `resolve` is the only way in,
which is what lets the doc comment call the value itself the proof. The two
fields are the workspace root and the root of the workspace holding the
repository, and both are canonical by the time either is stored.

<!-- fragment «gate-workspace-value» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="72-81" parent="workspace-value-and-gate" -->
````rust
/// One resolved jj workspace.
///
/// Holding one is the proof that the precondition passed: it cannot be
/// constructed for a working tree that is not jj-enabled, so an operation
/// reached through it never has to re-ask.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workspace {
    root: PathBuf,
    main_repo: PathBuf,
}
````
<!-- /fragment -->

The constructor's contract is argued before its body, and the argument is the
chapter's thesis at the site that creates it. Three claims are made here. The
first names the gate and denies the dispatch. The second says the walk is the
filesystem's rather than jj's, which is what makes the answer immune to the
environment. The third says the root is canonical, which is what makes aliases of
one workspace resolve to one value rather than to several that disagree.

<!-- fragment «gate-resolve-contract» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="82-96" parent="workspace-value-and-gate" -->
````rust

impl Workspace {
    /// Resolve the workspace at or above `path` — the closest ancestor holding
    /// a `.jj/` directory, whether it is a native, secondary or colocated
    /// checkout.
    ///
    /// **This is the precondition gate, not a dispatch.** There is one lane, so
    /// absence is never a case to handle; it is a refusal that names the command
    /// that fixes it, returned before anything has been read or changed.
    ///
    /// The walk is the filesystem's, not jj's: no repository discovery is
    /// invoked, so repository-selection variables in the environment and a
    /// shared repository store cannot redirect the answer. The root is
    /// canonical, so symlink and relative-path aliases of one workspace resolve
    /// to one [`Workspace`] rather than to several that disagree.
````
<!-- /fragment -->

The whole function is nine lines and its body is seven, and one line of that
body stands behind each of the three claims: the `find` predicate is the filesystem walk, the `ok_or_else` is the
refusal that replaces a dispatch, and the `canonical` call is the canonical root.
The input is a path that may be anywhere inside a workspace or nowhere near one;
the output is a `Workspace` or the first refusal that stops the sequence.
`Path::ancestors` yields the path and each of its parents, so `find` takes the
closest ancestor whose `.jj` is a directory and `ok_or_else` turns the empty case
into the refusal the second ending printed. The invariant established by the last
two lines is that both stored paths are canonical: `canonical` normalises the
found root, and `main_repo_of` returns a canonical path in both of its cases.
`a_symlinked_alias_resolves_to_the_same_workspace` and
`a_workspace_resolves_from_a_subdirectory`
(`crates/jj-workspace/tests/workspace.rs`) are the two ends of that claim.

<!-- fragment «gate-resolve-walk» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="97-105" parent="workspace-value-and-gate" -->
````rust
    pub fn resolve(path: &Path) -> Result<Self, Refusal> {
        let candidate = path
            .ancestors()
            .find(|dir| dir.join(".jj").is_dir())
            .ok_or_else(|| Refusal::not_a_workspace(path))?;
        let root = canonical(candidate)?;
        let main_repo = main_repo_of(&root)?;
        Ok(Self { root, main_repo })
    }
````
<!-- /fragment -->

The first accessor answers from a field the walk already filled. Its comment says
what a reader would otherwise have to check: no jj binary is spawned to answer
it. That is a property of the gate rather than of the getter — the walk found the
directory, so the value is known before any operation asks for it.

<!-- fragment «gate-root-accessor» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="106-111" parent="workspace-value-and-gate" -->
````rust

    /// The workspace root — the directory holding `.jj/`, already found by the
    /// walk. No jj binary is spawned to answer it.
    pub fn root(&self) -> &Path {
        &self.root
    }
````
<!-- /fragment -->

The second accessor answers the question the third ending asked, and it also
answers it from a field. The cost of the borrowed case was paid once, during
resolution, rather than at each call: a consumer that asks for `main_repo` in a
loop spawns nothing. Its comment states which workspace the value names — the
default one, which holds the repository every other workspace borrows — and that
a native or colocated checkout is its own.

<!-- fragment «gate-main-repo-accessor» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="112-118" parent="workspace-value-and-gate" -->
````rust

    /// The root of the **default** workspace: the one that holds the
    /// repository, which every other workspace borrows. A native or colocated
    /// checkout is its own.
    pub fn main_repo(&self) -> &Path {
        &self.main_repo
    }
````
<!-- /fragment -->

<a id="the-default-workspace"></a>
## Where resolution may spawn jj

The two free functions `resolve` calls sit below the whole of the `impl` block,
ahead of the namespace validation *The namespace it will not name* owns. They are
forty-five lines, and between them they carry the premise stated above and the
one refusal the gate has left.

<!-- fragment «gate-main-repo-and-canonical» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="291-335" parent="source-library" -->
<!-- insert «gate-main-repo-premise» -->
<!-- insert «gate-main-repo-probe» -->
<!-- insert «gate-canonical» -->
<!-- /fragment -->

The longest comment in the file is here, and it argues three separate decisions
rather than describing the code below it. The first is that the filesystem
answers wherever it can, which is the pointer-file premise applied: only the
uncommon, borrowed case costs a subprocess. The second is that the pointer is
followed by jj rather than parsed here, which keeps the format jj's to change.
The third is the scope of the test, and it is the one a reader is most likely to
argue with.

<!-- fragment «gate-main-repo-premise» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="291-313" parent="gate-main-repo-and-canonical" -->
````rust

/// The default workspace's root, from a workspace root that is already
/// canonical.
///
/// **Answered from the filesystem wherever it can be.** jj's own on-disk shape
/// distinguishes the two cases: a workspace that *borrows* another's repository
/// has `.jj/repo` as a pointer **file**, and one that holds its own has the
/// repository there instead — the same file-versus-directory shape Git uses for
/// `.git`. So only a borrowed repository has to be followed, which is the
/// uncommon case, and jj is asked to follow it rather than the pointer being
/// parsed here.
///
/// The test is *is there a pointer*, not *is the repository intact*. Resolution
/// is the precondition gate for **being a workspace**, and a `.jj/` whose
/// contents are damaged is a different failure with a different remedy — one jj
/// will state, loudly, at the first command that needs the repository.
/// Diagnosing it here would mean every resolution paid a subprocess to find out
/// something it was not asked.
///
/// `--ignore-working-copy` keeps the borrowed-repository probe read-only:
/// without it every jj command snapshots the working copy, a mutation no
/// resolution step should perform, and one that would fail outright in a stale
/// workspace.
````
<!-- /fragment -->

The third decision is the one a comment cannot close, because closing it means
citing a test three files away. The rejected alternative is that resolution
should validate the repository behind the workspace, and it is asserted against
directly: `resolution_does_not_validate_the_repository_behind_the_workspace`
(`crates/jj-workspace/tests/workspace.rs`) builds a directory whose `.jj/` is
empty — no repository at all — and requires `resolve` to succeed and report the
tree as its own main repository. That test is what makes the scope of the gate
observable rather than merely intended, and it is placed under a comment saying
why the alternative reading was rejected: it would cost a subprocess on every
resolution and refuse for a reason the caller did not raise. The cost of the
choice is also stated: a damaged repository is reported by jj at the first
command that needs one, not here, and a consumer that wants an earlier answer
runs its own jj command for it.

<!-- fragment «gate-main-repo-probe» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="314-329" parent="gate-main-repo-and-canonical" -->
````rust
fn main_repo_of(root: &Path) -> Result<PathBuf, Refusal> {
    if !root.join(".jj").join("repo").is_file() {
        return Ok(root.to_path_buf());
    }
    let printed = jj::output(
        root,
        &[
            "workspace",
            "root",
            "--name",
            "default",
            "--ignore-working-copy",
        ],
    )?;
    canonical(Path::new(printed.trim()))
}
````
<!-- /fragment -->

`canonical` is four lines: the crate's only use of `Path::canonicalize`, and the
only place `Refusal::unresolvable_path` is constructed. The input is a path that
has already been found to hold `.jj/`, or one jj has just printed; the output is
an absolute path with symlinks and relative components resolved. Its failure is
kept distinct from `not_a_workspace` because the two have different remedies: a
tree that is not a workspace is initialised, whereas a path that will not resolve
is a broken symlink or a directory removed underneath the process, and the
refusal says so. The `io::Error` is kept as the refusal's `source()`, so a
consumer that walks the chain sees what the operating system said.

<!-- fragment «gate-canonical» owner="one-lane" source="crates/jj-workspace/src/lib.rs" lines="330-335" parent="gate-main-repo-and-canonical" -->
````rust

fn canonical(path: &Path) -> Result<PathBuf, Refusal> {
    path.canonicalize()
        .map_err(|cause| Refusal::unresolvable_path(path, cause))
}

````
<!-- /fragment -->

The gate is now complete: one walk, canonicalisation, one conditional probe, and
two refusals. Nothing in it branches on which version control system owns the
tree, because nothing else can own it. The next chapter opens the seam that probe
was spawned through, and shows why its hygiene is a property of the crate rather
than a habit at each call site.

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: The subprocess seam](03-subprocess-seam.md)

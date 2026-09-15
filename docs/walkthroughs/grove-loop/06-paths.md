# Paths, and addressing
<!-- book-page id="paths" slice="paths-are-built-here" order="6" -->
[Previous: Opening, contention and refusal](05-opening.md) | [Contents](README.md) | [Next: The walk: pick and select](07-the-walk.md)

<a id="paths-are-built-here"></a>
## The rule: the library returns no paths, so grove builds them

The public crate root now re-exports `entry_path` for the read-only viewer.
It composes a snapshot entry's canonical path; it acquires no lock and reads no
file itself. The viewer holds its `Tree` while copying selected bytes, then
releases the guard before rendering or waiting. The path algorithm is unchanged.

Chapter 5 ended holding a `Tree` — one snapshot of the whole task tree, taken
under the store's shared lock. Everything a read verb answers with has to be
computed from that snapshot, and the one thing the snapshot will not give up is
a path. The library's reading surface has none: `cli-k16` refused to add a
`path()` to the algebra and recorded why, which is the sentence of the module
header chapter 5 reproduced and pointed here. So a consumer that wants to tell
an operator where a leaf lives builds the string itself.

> The library returns no paths, so grove builds them — in exactly one place.

That is this chapter's answer to *what did not go, and why could it not?* on the
way through. It is a smaller thing than a grammar and it is the same kind of
thing: the layer that stayed holds a fact the layer beneath it deliberately does
not model. A store that addressed entries by path would have to decide what a
path *is* — whose spelling of the root, resolved or not, on a filesystem that
may spell one directory two ways — and that is a domain question wearing a
filesystem's clothes.

The carried example reaches its sixth step here, and it runs in both directions.
Outwards, from an entry of the snapshot to one absolute path:

```text
Entry<'_, TaskName>            an entry of the snapshot chapter 5 opened
  │
  └─ entry_path(root, entry)
       │
       ├─ root.to_path_buf()             the caller's own spelling, unaltered
       │
       ├─ for container in entry.ancestors()          root-first
       │    ├─ container.entry() == None  the tree root — no name to render,
       │    │                             so exactly one element is skipped
       │    └─ Some(node)                 push node.name().to_string()
       │
       └─ push entry.name().to_string()
                        ⇒ <root, as the caller spelled it>/<node>/…/<entry>
```

Inwards, from a path an operator typed to the entry it names:

```text
&Path                            absolute, or relative to the grove root
  │
  ├─ canonicalize()                        to compare — never to report
  │    └─ Err ⇒ "resolving path …"         the filesystem would not resolve it
  ├─ == root_real                ⇒ Target::Root
  ├─ !starts_with(root_real)     ⇒ "path … is not under grove root …"
  ├─ file_name() not UTF-8       ⇒ "path … has no UTF-8 filename"
  └─ snapshot.walk(), matching the final component, then the built path
       ├─ hit                    ⇒ Target::Entry(entry)
       └─ miss                   ⇒ unreachable_by_any_walk
                                     ├─ Malformed / Reserved → the grammar's own sentence
                                     ├─ Foreign  → "not a Grove leaf or node directory: …"
                                     └─ Entry    → "… is not in the task tree: every level
                                                    above it must be a node directory"
```

The two directions meet: the inward walk decides which entry a path names by
building each candidate's path with the outward function and comparing. That is
why one page carries both, and it is the reason the chapter can make its rule a
fact about the code rather than a convention — there is no second path builder to
disagree with the first.

This chapter owns 320 lines of `task_tree.rs` in 2 blocks.
The source index records their current ranges; the fragments below reconstruct
every owned byte.

<a id="one-place-a-path-is-built"></a>
## The one place a path is built

The composite below is the chapter's first ownership block. It expands, in
order, to lines 303 through 530 of the file, and the nine fragments it names run
from here to the end of the section before last.

<!-- fragment «paths-and-addressing» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="304-531" parent="source-task-tree" -->
<!-- insert «paths-entry-path» -->
<!-- insert «paths-target-enum» -->
<!-- insert «paths-target-fn» -->
<!-- insert «paths-unreachable-by-any-walk» -->
<!-- insert «paths-addressable-key» -->
<!-- insert «paths-interrupted-promotion» -->
<!-- insert «paths-next-key» -->
<!-- insert «paths-live-leaf» -->
<!-- insert «paths-entry-outcome» -->
<!-- /fragment -->

The function is eighteen lines including its doc comment — the fragment below is
nineteen because it carries the blank line that ends the item — and eight of
those are the body.

<!-- fragment «paths-entry-path» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="304-322" parent="paths-and-addressing" -->
````rust
/// Where an entry of a snapshot lives: the caller's spelling of the root, then
/// every containing node's name, then its own.
///
/// **The one place grove builds a path out of the tree.** The library returns
/// none, and `cli-k16` refused to answer that by adding a `path()` to the
/// algebra, so this is the consumer's half of that decision — and every later
/// flip leaf calls it rather than writing a second one.
#[must_use]
pub fn entry_path(root: &Path, entry: Entry<'_, TaskName>) -> PathBuf {
    let mut path = root.to_path_buf();
    for container in entry.ancestors() {
        if let Some(node) = container.entry() {
            path.push(node.name().to_string());
        }
    }
    path.push(entry.name().to_string());
    path
}

````
<!-- /fragment -->

**The actor is grove and the input is an entry of a snapshot the caller already
holds.** `entry_path` takes the root as the caller spelled it and an `Entry`, and
returns one `PathBuf`. It reads no directory, takes no lock and can fail in no
way: everything it needs is in the snapshot, which is why it is safe to call
inside a walk that is already holding one.

The composition is the header's, in three moves. The base is `root.to_path_buf()`
— **the caller's own spelling**, copied and never resolved. Then
`entry.ancestors()`, which the store documents as the entry's containing nodes
*root-first*, and whose first element is the tree root: a level that is not an
entry, carrying no ordinal, no key and no parts. `container.entry()` answers
`None` for exactly that element, so the `if let` skips one element per call and
pushes the rendered name of every node in between. Then the entry's own name.
An entry at the top level of the grove therefore has a one-element loop that
pushes nothing, and a leaf two nodes deep pushes two components before its own.

**Why it is safe without a check** is the second clause of the header, and it is a
property established two layers away. `push` on a `PathBuf` replaces the
whole path when the component is absolute and appends without normalising when it
is not, so a name that rendered as `/etc/passwd`, or as `../../elsewhere`, would
address outside the tree the lock covers.
Nothing here tests for that, because the store already refused to build a snapshot
containing such a name: every `Verdict::Entry` it admits is rendered at read time
and checked to be one path component, and a failure is `Error::NameIsNotOneComponent`
over the whole tree rather than a skipped entry. The header names that error, and
chapter 5 reproduced the naming. **The check is not absent; it is spent once, at
the boundary where a name enters the snapshot, instead of once per path built.**

The `#[must_use]` says the only thing a caller can do with the result is use it.
There is no side effect to want.

**Addressing, selection and mutation diagnostics share this helper.** This
chapter uses it in `target` and `addressable_key`; chapter 7's `selected` uses it
for the chosen leaf, duplicate-key paths and multiple-finish diagnostics.
Chapters 8 and 9 use it in brief-chain and resolution operations, and chapters
10 and 13 in grow and prune operations. That spread is what the header's claim is *for*: every path grove prints, from
`pick`'s single line to a prune's list of what it left alone, comes out of this
eight-line body, so a change to how grove spells a path is a change in one place.

**No test in this crate names this function.** It is proved the way a shared
subexpression is proved — through the assertions of the verbs that build on it,
which compare a path grove returned against one the test constructed. That is
worth stating plainly, because it is the shape of most of this chapter's
evidence: the tests that pin these functions belong to the chapters that own the
verbs, not to the chapter that owns the functions.

<a id="which-entry-a-path-names"></a>
## Which entry a path names, and the root that is not an error

Going the other way is the resolver a path-taking verb calls first. Its result
type comes before it, and the type is where the interesting decision is.

<!-- fragment «paths-target-enum» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="323-335" parent="paths-and-addressing" -->
````rust
/// What a caller's path argument names in the tree.
///
/// The grove root is a case of its own rather than an error, because each verb
/// words its refusal of it differently — `leaf-prune`'s is about abandoning a
/// whole workstream, `leaf-retire`'s about the argument not being a leaf — and
/// the resolver has no business choosing between them.
pub(crate) enum Target<'a> {
    /// The grove root itself. Not an entry: it carries no name to rewrite.
    Root,
    /// An entry of the snapshot — a task file, a node directory, or a node file.
    Entry(Entry<'a, TaskName>),
}

````
<!-- /fragment -->

**Two cases, and the first of them is not an error.** A verb that takes a path
can be handed the grove root, and this type refuses to decide what that means. The
comment's reason is that each verb words its refusal differently, and the
consumers bear that out — with one qualification the comment does not make and
one it overstates.

The qualification first, because it is the stronger half of the argument.
**`Target::Root` is not a refusal everywhere.** `task_grow`'s `parent_node`
(chapter 10) maps it to the library's own `Target::Root`, which is the ordinary
case of `leaf-add .` — appending at the top level of the grove. A resolver that had
folded the root into an error would have had to be worked around by the one verb
for which the root is the normal argument. The comment argues from the wordings
and the code makes the point more cheaply: one of the five consumers of this
variant does not refuse at all.

The overstatement is *each verb words its refusal of it differently*. Four
consumers refuse, and there are three wordings, not four. `leaf-prune`'s is about
abandoning a whole grove — *a branch-delete, not a tree mark*. `leaf-insert`'s
(chapter 10) is about the verb's own shape — it takes the slot of an existing
entry, and it names `leaf-add .` as what the operator wanted. `leaf-retire`'s and
`leaf-decompose`'s are the same sentence with the verb swapped: *lifecycle verbs
act on leaves*. Those two are chapter 13's and chapter 12's — one marks a live leaf
`DONE` in place, keeping its position and its key; the other turns a leaf file
into a node directory, keeping the key and moving the body in as the node's
`_BRIEF.md` — and the minimum this chapter needs of either is that both act on a
leaf, which is why both refuse the root and why neither can say so in the
resolver's words. Two verbs sharing a reason is not evidence against keeping the
case out of the resolver — the resolver still cannot supply `leaf-insert`'s
sentence or `parent_node`'s silence — but *each* is a word the enumeration does
not support.

**One name collision to carry, and it is with the library rather than with this
crate.** `ordinal-fs-tree` declares a `Target` of its own — `Root`, or
`Key(Key)`, which is how an operation names what it acts on — and both
`task_grow` and `tree_lifecycle` import it unrenamed. So in those two modules
`Target` means the library's, and in this one it always means this crate's,
whose second variant is an `Entry` rather than a key. The two are the two halves
of clause 1 laid end to end: `task_grow`'s `parent_node` takes a
`task_tree::Target` out of a path argument and hands back an
`ordinal_fs_tree::Target` for the operation, and the conversion between them —
an entry to the key the library will be called with — is `addressable_key`, two
sections below.

The resolver itself follows: the function every path-taking verb calls first.

<!-- fragment «paths-target-fn» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="336-388" parent="paths-and-addressing" -->
````rust
/// The snapshot entry a caller's path argument names: absolute, or relative to
/// the grove root, and a leaf file or a node directory alike.
///
/// **Clause 1 of `docs/ARCHITECTURE.md#library-refusals`** — resolve the
/// argument to an entry, so the verb can then call the library *by key* against
/// the same snapshot the operation plans from. Grove's path grammar is wider
/// than a key and the library has no counterpart for *no such path*, so
/// resolution is Grove's and so is every message below.
///
/// Canonicalised to **compare** and never to report, exactly as `leaf_entry`
/// does: two spellings of one path name one entry, and the paths this module
/// returns are still built from the caller's own spelling of the root.
pub(crate) fn target<'a>(
    root: &Path,
    snapshot: &'a Snapshot<TaskName>,
    path: &Path,
) -> Result<Target<'a>> {
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    let resolved = candidate
        .canonicalize()
        .with_context(|| format!("resolving path {}", candidate.display()))?;
    let root_real = root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", root.display()))?;
    if resolved == root_real {
        return Ok(Target::Root);
    }
    if !resolved.starts_with(&root_real) {
        bail!(
            "path {} is not under grove root {}",
            resolved.display(),
            root_real.display()
        );
    }
    let name = resolved
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("path {} has no UTF-8 filename", resolved.display()))?;
    for entry in snapshot.walk() {
        if entry.name().to_string() != name {
            continue;
        }
        if entry_path(root, entry).canonicalize().ok().as_deref() == Some(resolved.as_path()) {
            return Ok(Target::Entry(entry));
        }
    }
    Err(unreachable_by_any_walk(&candidate, &resolved, name))
}

````
<!-- /fragment -->

**Clause 1 of the refusal contract, and the whole reason it exists.** The doc
comment names `docs/ARCHITECTURE.md#library-refusals` and states the shape: turn
the operator's argument into an entry *of this snapshot*, so that the verb can
then call the library **by key** against the same snapshot the mutation will plan
from. The alternative — resolve to a path, then hand the library the path — is
what the pre-flip reader did, and it is a race with a name: the tree could move
between the resolution and the operation. Under one guard it cannot, and that is
the second of the three questions the book's stated outcome asks — *against which
snapshot?*

The sentence after that says why the messages are Grove's own rather than a
second wording of the library's, and it gives the reason in two clauses:
**Grove's path grammar is wider than a key, and the library has no counterpart for
*no such path*.** A key is one integer; a path argument may be absolute, may be
relative to the grove root, may name a directory that is not a node, may name
nothing at all. Every one of those is a condition the library was never given a
vocabulary for, so every message in this function is Grove's.

The body runs as four decisions.

**A relative argument is relative to the grove root**, not to the process's
working directory. `root.join(path)` is the whole of it, and it is the reason an
operator can paste `01-impl--a-k1.md` from `pick`'s output into `leaf-retire`
from anywhere. The wider convenience — a path relative to the *cwd* — exists, but
it is `reference`'s in chapter 9, not this function's; this one has exactly two
spellings.

**The canonicalisation is to compare.** `candidate.canonicalize()` is the first
call that can fail, and it wraps whatever the operating system said — usually
*no such file*, but a permission-denied component or a symlink loop reaches the
same line — in `resolving path …`. That is worth separating from the refusals
below it: a path the filesystem would not resolve is an environmental failure,
and a path that resolves and names no entry is a deliberate refusal. They read
differently on a terminal and they should. The root's own `canonicalize` beside
it can fail the same way, under `canonicalising grove root …`, and one more exit
sits between the `starts_with` guard and the walk: a resolved path whose final
component is not UTF-8 leaves with `path … has no UTF-8 filename`.

**The comparison is against the canonicalised root**, so `.` and
`<worktree>/.grove` and a symlinked spelling of either all reduce to `Target::Root`
together. The `!starts_with` guard then bounds the whole rest of the function to
the subtree the lock covers.

**The walk decides the rest, and it decides it in two stages.** `entry.name().to_string()`
is compared against the argument's final component first, which is a cheap
rejection over the whole snapshot; only a name that matches gets its own path
built and canonicalised for the equality that settles it. `entry_path(root, entry)`
is the other side of that comparison: **the resolver recognises an entry by
rebuilding the path it would have printed for it.** The `.ok()` swallows a
canonicalisation failure on the built side, which under a held shared lock is a
path that cannot go missing; it is a non-match rather than an error because a
failure to build one candidate is not a statement about the argument.

**On a well-formed tree the first stage would already be decisive, and that is
exactly why the second one is there.** A rendered name carries the entry's key,
and a key is unique tree-wide — so two entries cannot share a name unless they
share a key, and two entries sharing a key is the tree the next section refuses.
The library states key uniqueness as an obligation on the domain and cannot
enforce it, so a hand edit can put `01-impl--a-k1.md` at the root and a second
copy of that name inside a node. The path comparison is what makes `target` name
the entry the operator actually pointed at even then, and `addressable_key` is
what refuses to operate on the tree afterwards. The order matters: resolving by
name alone would pick a twin, and the refusal would then be about the wrong one.

<a id="canonicalise-to-compare"></a>
## Canonicalising to compare, and where that happens

The doc comment above states the rule the module header opened on, and this is
the chapter that owes the account.

**Nothing here canonicalises for output.** On macOS `/var` is a symbolic link to
`/private/var`, and the two spell one inode — checked on the machine this book
was written on, where `os.path.realpath('/var')` is `/private/var` and the two
paths report the same device and inode number. A reader that canonicalised the
paths it returned would therefore answer `/private/var/folders/…` to a caller
that had asked about `/var/folders/…`. The user-visible consequence is the one
the header names and it is worse than an inconsistency: canonicalisation happens
where a path is *resolved*, and a path is resolved when a lock is taken, so **the
mere presence of a lock would rewrite every path grove prints.** The same command
would answer two ways depending on whether anything else was running. Building
from the caller's own spelling of the root — which is what `entry_path`'s first
line does and why its parameter is a `&Path` rather than something already
resolved — is what keeps the answer stable.

Comparing is a different operation and it is safe for exactly the reason
reporting is not: a comparison's result is a boolean, and the operator never sees
either side of it. `target` canonicalises three times — the candidate, the root,
and each candidate entry's built path — and returns a `Target` carrying no path
at all.

**The header used to say this happens in one place, and it happens in two.**
Until `canonicalisation-sites-k149` the sentence chapter 5 reproduces read
*Canonicalisation appears once, in `leaf_entry`, and only to compare a caller's
spelling of a leaf against the tree's.* The second half named the operation both
functions perform — comparing, and never reporting — but its own wording was
narrow in a second way nobody had called out: it says *a caller's spelling of a
leaf*, and `target`'s doc comment says it resolves *a leaf file or a node
directory alike*, returning `Target::Root` for the root itself. The first half
was refuted from inside this corpus, and by the very
fragment above: `target`'s own doc comment says *Canonicalised to compare and
never to report, **exactly as `leaf_entry` does***, which is an admission that
there are two. Across the whole of `crates/grove-loop/src/`, `canonicalize` is
called at eight sites. Six are production and all six are in this file — lines
346 349 and 369 inside `target`, and 712, 715 and 734 inside `leaf_entry` —
and the remaining two are assertions inside `driver_lease.rs`'s own
`#[cfg(test)]` module, which chapter 17 owns. `target` and `leaf_entry` are two
functions whose tails are near-duplicates of one another, one resolving any
entry and one resolving a leaf; chapter 8 reads the second.

**The corrected clause names a property rather than a count, and that is
deliberate.** It now reads *Canonicalisation happens only where a caller's path
is resolved to an entry — in `target` and in `leaf_entry`, nowhere else here —
and only to compare.* Two things in that wording are doing work. *Resolved to an
entry* is narrower than *interprets a path*, because several functions in this
module do the latter without canonicalising anything — `kind_in` and
`brief_chain` take a leaf path and delegate, and `existing_path`, later in this
file and in chapter 9's block, turns an argument into a path that exists by
joining it onto the grove root or the cwd and testing `exists()`. Only the two
that resolve a path to an entry canonicalise. And *here* scopes the claim to
this module, which is the only
scope it is true in — the grove root arrives already canonical, because
`Workspace::resolve` canonicalises the workspace root before `grove_root` joins
`.grove` onto it, which is what chapter 16's figure records as *the closest
`.jj/`, canonicalised*. The claim the module can make is that it adds no
canonicalisation of its own, not that an operator's spelling survives to the
output.

**The fix and this page landed in one commit**, which is what the corpus freeze
requires: a source change shifts every line below it and breaks pages that have
already proved themselves, so `canonicalisation-sites-k149` reworded the comment
inside its own line count — seven lines before and seven after, and
`task_tree.rs` still at the 2,023 lines it then had — and carried chapter 5's
fragment, this account and
the early-use ledger with it. The adjudicating paragraph became this explanation
rather than a deletion, because the reason a sentence is phrased oddly outlives
the defect that forced it.

**The class is familiar, and this is not the first of it.** Chapter 1 met the
manifest's clause locating `libc` in `task_tree` alone, which three production
modules reach; that one has since been corrected at source by
`manifest-dependency-clauses-k133`, so chapter 1 now explains the widened clause
instead of adjudicating the narrow one. Beside it sits `lib.rs`'s
*`<worktree>/.grove`, spelled in exactly one place*, which has since gone the
same way at `grove-root-join-clauses-k148`: six other production sites spell the
root — three of them openings inside `grove-loop` that each take their own lock
before any opening exists, and three in `crates/grove-llm/src/cli.rs` that spell
it only to name it in output — so chapter 1 now explains a join scoped to the two
public openings rather than adjudicating one claimed for the crate. Each of the three was a
**uniqueness claim written from the shape of the design rather than from an
enumeration of the code**, each was true of the intent and false of the source,
and all three have now been corrected at source. None of them gained an
instrument over its own call sites — unlike the lock, where
`the_librarys_tree_lock_is_taken_from_exactly_one_module` in
`crates/grove-llm/tests/tree_lock.rs` counts them and fails on a rename. That
test is what the class lacks, and chapter 5 read it — but the analogy is not
automatic, and each leaf declined it for a reason of its own.
`grove-root-join-clauses-k148` declined because the lock test pins a deadlock
where a count of join sites pins tidiness, and would go red on a new opening
that is correct. `canonicalisation-sites-k149` declined for that reason and one
more: the property worth pinning here is behavioural rather than structural — *a
reported path keeps the caller's spelling* — and no verb can be made to assert
it, because the root is canonical before `task_tree` ever sees it. The suite
already works around exactly this. `crates/grove-llm/tests/resolve.rs` compares
a resolved path against `grove.canonicalize()` and says why: *`Workspace::resolve`
reports the real path. The claim is the tree itself, not a spelling of it.* So
what the corrected comment claims is the strongest thing that is true — this
module adds none — and that is a claim a reader checks by reading the module,
which is what this section is for.

<a id="no-walk-reaches-it"></a>
## Three reasons a path under the root names no entry

The last exit of `target` is a function whose whole job is to say which of three
things went wrong.

<!-- fragment «paths-unreachable-by-any-walk» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="389-417" parent="paths-and-addressing" -->
````rust
/// Why a path that exists under the grove root names no entry of the snapshot.
///
/// Three reasons, and they are different things to whoever hit one. The
/// grammar answers the first two — a name it disclaims is not Grove's at all,
/// and a name it refuses carries its own recovery advice — so only the third
/// needs a sentence of Grove's own: the entry is task-shaped and some level
/// above it is not, which is exactly the subtree no walk descends into.
fn unreachable_by_any_walk(candidate: &Path, resolved: &Path, name: &str) -> anyhow::Error {
    let found = if resolved.is_dir() {
        Found::Dir
    } else if resolved.is_file() {
        Found::File
    } else {
        Found::Other
    };
    match TaskName::parse(name, found) {
        Verdict::Malformed(error) | Verdict::Reserved(error) => anyhow!("{error}"),
        Verdict::Foreign => anyhow!(
            "not a Grove leaf or node directory: {}",
            candidate.display()
        ),
        Verdict::Entry(_) => anyhow!(
            "Grove entry {} is not in the task tree: every level above it must be \
             a node directory named NN-k<key>",
            candidate.display()
        ),
    }
}

````
<!-- /fragment -->

**The actor is the grammar and only one of the three answers is Grove's own
sentence.** The path exists, it is under the grove root, and no entry of the
snapshot matched it. `TaskName::parse` — chapter 4's function, reached here with
a `Found` this function computes — sorts the reason into three:

- **`Malformed` or `Reserved`.** The name is Grove's shape and Grove refuses it,
  and the refusal type's own `Display` writes each refusal's recovery advice
  rather than merely its detection — chapter 2's tests assert on that text and
  chapter 4 reads the renderer. Passing `{error}` through is the whole arm.
- **`Foreign`.** The name is not Grove's at all, so there is nothing to advise
  about and the sentence is a statement of fact: *not a Grove leaf or node
  directory*.
- **`Entry`.** The name is a perfectly good task name, and this is the only case
  that needs a sentence Grove has to compose. The doc comment says what it is:
  *the entry is task-shaped and some level above it is not, which is exactly the
  subtree no walk descends into*, and the message names the shape a level must
  have — `NN-k<key>`.

**Two of the three arms can only be reached inside a subtree the grammar
disclaimed, and chapter 5's account of the read is what shows it.** The library
skips a `Foreign` name and **skips it recursively when it is a directory**, so
nothing below such a directory reaches the snapshot; a `Malformed` or `Reserved`
name anywhere a walk *does* reach halts the whole tree, so the open in chapter 5
would have failed before this function could run; and a task-shaped name a walk
reaches is *in* the snapshot and would have matched. So the `Malformed`/`Reserved`
arm and the `Entry` arm both require a disclaimed level above the path. A
`Malformed` name inside `.grove/notes/` is refused here; the same name beside it
in `.grove/` is refused at the open, by the library, with a different sentence.
The doc comment says this for the third arm; it is true of the second as well.

**The `Foreign` arm is the exception, and it is the ordinary case.** A disclaimed
name needs no disclaimed ancestor — it is the disclaimed thing itself, and the
library skipped it exactly where it sits. `.grove/README.md` and a bare `notes/`
directory in the grove root both land here, which is why this is the arm an
operator actually meets and, as the last paragraph of this section says, the only
one anything asserts on.

**One small divergence, and it is between two classifications of the same name.**
The library classifies with what its listing found **unfollowed** — a `DirEntry`'s
file type does not traverse a symbolic link, so a link wearing an entry's name is
`Found::Other` and therefore a species mismatch. This function classifies
`resolved`, which is the *canonicalised* path, so a symbolic link to a regular
file is `Found::File` here. The two would disagree about such a name — and they
can only disagree where the library never looked, since a link wearing a task
name anywhere a walk reached would have halted the open as a species mismatch.
Inside a disclaimed subtree the outcome is a refusal either way and only the
wording differs. It is worth knowing
because it is the one place in the chapter where Grove re-runs the library's
classification from different inputs.

**Only one of the three arms is asserted anywhere.** The `Foreign` sentence is
pinned by `add_refuses_a_parent_that_is_not_a_grove_entry_at_all`, which builds a
bare `notes/` directory under a grove root and asserts on *not a Grove leaf or
node directory* — and that test lives in `crates/grove-loop/src/task_grow/tests.rs`,
this book's **one declared corpus exclusion**, which chapter 10 cites by name and
never reproduces. Nothing asserts the `Malformed`/`Reserved` arm, nothing asserts
the *is not in the task tree* arm from this function, and — one level up —
nothing asserts `target`'s own *path … is not under grove root …* either; that
sentence's only appearance outside the source is a transcript in
`docs/preservation-baseline.md`. A test here would have something to grip:
`leaf_entry` in chapter 8 produces a near-identical sentence, but the two differ
in their first noun — *Grove entry … is not in the task tree* here against *Grove
leaf …* there — so an assertion on the whole sentence would tell the two
functions apart. What is missing is the assertion, not a way to write one.

<a id="naming-one-entry-not-two"></a>
## Addressing by key, and the tree where one key names two entries

Resolving an argument to an entry is half of clause 1. The other half is turning
that entry into the key the library will be called with, and it is where the
chapter's strongest precondition is enforced.

<!-- fragment «paths-addressable-key» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="418-468" parent="paths-and-addressing" -->
````rust
/// The key by which the library can address this entry, or Grove's refusal that
/// it cannot.
///
/// **Clause 1 is sound only while keys are unique tree-wide.** The library
/// states uniqueness as an obligation on the domain and cannot enforce it: a
/// hand edit or a failed rollback can put two entries under one key, and
/// [`Snapshot::by_key`](ordinal_fs_tree::Snapshot::by_key) then answers with
/// whichever the walk reaches first. Walk *order* is unmodelled — the node
/// brief records `by_key`'s tie-break on a duplicate-key tree as a known miss of
/// `structure.als` — so which twin an operation lands on is not a fact anything
/// establishes. This is the consumer-side half of that miss, and it is not
/// theoretical: without it, `leaf-retire` aimed by path at one twin silently
/// marks the other and reports success.
///
/// Grove's own precondition, therefore, and not a second wording of anything the
/// library says (`docs/ARCHITECTURE.md#library-refusals`, clauses 2 and 3): the
/// library has no notion of *the entry the operator named*, which is the whole
/// of what is ambiguous here.
///
/// One walk per call, which is quadratic over a bulk mark. A `.grove/` tree is
/// tens of entries and this runs once per marked leaf, so the simpler shape is
/// kept deliberately.
pub(crate) fn addressable_key(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    entry: &Entry<'_, TaskName>,
) -> Result<Key> {
    let name = entry.name();
    let triple = entry
        .triple()
        .with_context(|| format!("{name} carries no key of its own"))?;
    let twins: Vec<Entry<'_, TaskName>> = snapshot
        .walk()
        .filter(|other| other.key() == Some(triple.key))
        .collect();
    if twins.len() > 1 {
        let paths = twins
            .iter()
            .map(|other| entry_path(root, *other).display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "two entries in this tree carry key {}, so naming one of them names \
             both: {}. A key is assigned once and never reused, so this is a hand \
             edit or a rollback that failed — give one of them a fresh key before \
             operating on either.",
            triple.key,
            paths
        );
    }
    Ok(triple.key)
````
<!-- /fragment -->

**The failure it prevents is not an error message; it is a success report about
the wrong file.** The doc comment says it in one sentence and the sentence is the
reason this function exists: *without it, `leaf-retire` aimed by path at one twin
silently marks the other and reports success*. Every mutating verb calls the
library by key. `Snapshot::by_key` answers with whichever entry the walk reaches
first, and the comment records that walk order on such a tree is **unmodelled**,
attributing the miss to the store's `structure.als`. The unmodelled half is
confirmed by the library itself: `by_key`'s own doc comment says the tie-break
*is the one reading behaviour no model checks* and that `operations.qnt` picks
the least internal id instead — and `operations.qnt` is where the two model
files on disk record it, `structure.als`'s duplicate-key content being an
admitted witness rather than a recorded miss. Which model carries the note is
worth checking against `operations.qnt` rather than against the name this
comment gives; what the note says is right either way. So on a duplicate-key
tree the library's answer is not merely wrong, it is not determined by
anything.

**Uniqueness is an obligation the library states and cannot enforce**, and that
asymmetry is what puts the check here rather than there. A key is allocated once
and never reused, so two entries under one key is a tree that Grove's own verbs
cannot produce — it takes a hand edit or a rollback that failed. The library has
no notion of *the entry the operator named*, which the comment names as the whole
of what is ambiguous, so there is nothing for it to check against.

The body is a walk collecting every entry whose key equals this one, a
`len() > 1` test, and two refusals. The general one names both paths — built with
`entry_path`, which is the second of this chapter's two call sites — and gives
the recovery that fits a hand edit: give one of them a fresh key. The special one
is the section below.

**One walk per call, and the comment defends the cost rather than hiding it.**
`leaf-prune` over a node calls this once per marked leaf, so a bulk mark is
quadratic in the subtree. The defence is a measurement of scale rather than an
argument about complexity — a `.grove/` tree is tens of entries — and it is
stated as a deliberate choice, which is what makes it reviewable when a tree is
one day not tens of entries.

**Six call sites, and five tests hold the sentence.** The functions that call it
are `leaf_decompose`, `leaf_retire` and the prune planner's `plan_leaf` in
`tree_lifecycle.rs`, and `leaf_insert`, `parent_node` and `containing_level` in
`task_grow.rs`. Five tests assert on *two entries in this tree carry key N*. Four
are in `tree_lifecycle.rs`'s inline test module, in the blocks chapters 12 and 13
own, and are read below; the fifth is `leaf-insert`'s,
`insert_refuses_a_target_whose_key_names_two_entries`, in
`crates/grove-loop/src/task_grow/tests.rs` — the excluded file, which chapter 10
may cite and not reproduce.

- `decomposing_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed`
  (chapter 12) puts a live leaf and its `DONE` twin under key 1 and decomposes
  the live one. Its two assertions split the property in two: the message
  assertion pins that the sentence is **Grove's**, so the library's own
  `DestinationOccupied` wording would fail it, and the tree assertion — the leaf
  is still a file and no node directory exists — pins that the refusal cost
  nothing. What would let it pass while the property was broken is a check that
  ran *after* a mutation neither assertion looks at; the two between them cover
  the two artifacts a promotion creates, so the gap is narrow, but the test
  proves those two paths untouched rather than that nothing happened at all.
- `destination_occupied_is_unreachable_because_the_occupant_duplicates_the_key`
  (chapter 12) is the one with a control. It asserts the operator sees Grove's
  sentence and **not** the library's *already taken*, and then calls the
  library's `promote` directly, bypassing every precondition, to prove the
  library's refusal was really there to be hidden. Without that last assertion
  the test would pass identically if the tree never reached the library at all
  for some unrelated reason, and a reachability claim that cannot fail is worth
  nothing — which the test's own comment says.
- `prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address` (chapter 13) is
  the bulk case, and it is the test the book's stated outcome names for question
  2. Three live leaves under a node, one of them beside an `ABANDONED` twin; the
  prune refuses, and the assertions that carry the property are the negative ones
  — the two leaves *before* the bad one are not marked. A test that only checked
  the error message would pass while a partially-applied prune was exactly what
  had happened.
- `retiring_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed`
  (chapter 13) is the single-leaf form of the same thing.

None of the five is in this chapter's blocks. The function is here because
addressing is what it does; the evidence is where the verbs are.

The special refusal has a shape of its own: it is Grove's sentence carrying the
**library's** recovery advice rather than Grove's.

<!-- fragment «paths-interrupted-promotion» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="469-470" parent="paths-and-addressing" -->
````rust
}

````
<!-- /fragment -->

**The library names this state, and says so in the one run that can never be the
run that meets it.** `Error::FailedPartiallyRolledBack` describes a node and a
leaf sharing an ordinal and a key with the node holding no distinguished child,
and it is raised by the promotion whose rollback failed — a process that has
already reported and exited. Every later command opens that tree cold, and the
library reports nothing at all, because a duplicate key is an obligation on the
domain and no operation checks it. The comment's sentence for this is the
chapter's best one-line statement of why domain knowledge cannot move: *the
process that meets it is never the process that caused it.*

**The recovery is why the case is separated rather than folded into the general
refusal**, and the comment is blunt about it: `addressable_key`'s *give one a
fresh key* is **actively wrong** here. The node and the leaf are one entity
caught mid-shape-change — `leaf-decompose` preserves the key, which is chapter
12's rule — so giving either a fresh key would turn one work item into two. The
advice this arm gives instead is the library's own, transcribed into Grove's
sentence: remove either half, and which half you remove is which outcome you
choose.


A missing positioned-node file is refused before a snapshot is exposed.
Its diagnostic conditionally asks the operator to inspect an empty directory
and a same-position, same-key sibling leaf. The reader performs no second
listing after the guarded open fails and makes no claim that it observed that
sibling. The recovery preserves the leaf's identity and bytes.


The comment's claim that the compared ordinals always exist is sound, and it is
worth checking rather than accepting, because the code hedges against its own
argument. `Entry::ordinal` and `Entry::key` are both `self.triple().map(…)` over
the same `Option`, so an entry has an ordinal exactly when it has a key; the
caller filtered on `key() == Some(triple.key)`, so both twins have one. The
`map_or_else(|| "?", …)` at the message's first argument is therefore
unreachable. It is a fallback rather than a bug — the alternative in a formatting
argument is an `unwrap` — but a reader tracing the `?` should know it cannot be
printed.


`a_missing_node_file_gives_conditional_promotion_recovery` constructs an
empty node beside a leaf and calls the guarded opening directly. The assertions
check the conditional wording, required filename, containing path and unchanged
leaf bytes. No mutation or addressable-key lookup is reached.


<a id="predicting-the-allocation"></a>
## Predicting what the library will allocate, and checking the prediction

The last substantial function in the block reads a key that does not exist yet.

<!-- fragment «paths-next-key» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="471-507" parent="paths-and-addressing" -->
````rust
/// The key the library will give the next entry it creates from this snapshot —
/// `max + 1` over every name in the tree — or `None` when the keyspace is full.
///
/// # A mirror of the library's rule, and the reason grove needs one
///
/// The library allocates keys and grove does not, which is why grove's own
/// key allocator died. But grove's leaf **content** embeds the key its name
/// will carry — the first-line handle `# <slug>-k<key>` — and
/// [`NewEntry`](ordinal_fs_tree::NewEntry) takes its bytes *before* the library
/// composes the name, so the bytes cannot be written from the answer. A
/// content-carrying domain therefore has to predict the allocation.
///
/// It is a prediction and it is checked: every grow verb compares this against
/// the key the library actually reports and refuses to claim success on a
/// disagreement (`task_grow::allocated`). The prediction reads the *same*
/// snapshot the operation plans from under the *same* guard, so it can only
/// differ if the library's allocation rule changes — which is exactly what the
/// check exists to catch, since a silent disagreement is a leaf whose header
/// contradicts its filename.
///
/// `None` rather than a refusal: an exhausted keyspace is
/// [`Refusal::KeysExhausted`](ordinal_fs_tree::Refusal), which is the library's
/// to state and not grove's to anticipate
/// (`docs/ARCHITECTURE.md#library-refusals`, clause 3). The caller hands the
/// library no bytes and lets it refuse — a refusal writes nothing, so the
/// unrenderable content is never reached.
#[must_use]
pub(crate) fn next_key(snapshot: &Snapshot<TaskName>) -> Option<Key> {
    let greatest = snapshot
        .walk()
        .filter_map(|entry| entry.key())
        .map(Key::get)
        .max()
        .unwrap_or(0);
    greatest.checked_add(1).map(Key::new)
}

````
<!-- /fragment -->

**Grove does not allocate keys and needs to know what the next one will be.** The
comment states both halves and the tension between them. Allocation moved to the
library, which the comment says is why grove's own key allocator died — but
grove's leaf *content* embeds the key its filename will carry, as the `# <slug>-k<key>` handle
on the leaf's first line, and `NewEntry` takes those bytes **before** the library
composes the name. Bytes that have to be written before the answer is known are
bytes that have to be predicted.

**The prediction is not a guess and the comment says exactly why.** It reads the
same snapshot the operation plans from, under the same guard, and it mirrors the
library's rule directly: the greatest key over every name in the tree, plus one.
Under one guard the tree cannot move between the prediction and the allocation,
so the only thing that can make them differ is the library's allocation rule
changing — which is the one failure the check is there to catch. **And the check
is real**: every grow verb compares this against the key the library reports and
refuses to claim success on a disagreement, in `task_grow::allocated`, which
chapter 10 reads. A silent disagreement is a leaf whose header contradicts its own
filename, which is the worst outcome available here, since both are on disk and
neither is obviously the wrong one.

**`None` is a prediction, not a refusal**, and that distinction is clause 3 of the
refusal contract in one line. An exhausted keyspace is `Refusal::KeysExhausted`,
which is the library's sentence. Grove could raise its own and would then be
saying the same thing twice, in two wordings, one of which would go stale. So the
caller hands the library no bytes and lets it refuse — and the comment closes the
loop on why that is safe: a refusal writes nothing, so the content that could not
be rendered is never reached.

**Four call sites, in four functions.** `materialize_finish` and
`leaf_decompose` call it in `tree_lifecycle.rs`; `leaf_add` and `leaf_insert`
call it in `task_grow.rs`. `root_init` is *not* among them, and the reason is
worth naming because the discipline is the same: it delegates to
`initialize_grove`, whose own comment predicts the key rather than reading one —
*the prediction is 1*, because the store places an empty tree's entries from
`Ordinal::FIRST` with keys from 1, so there is no maximum to take. Chapter 11
reads it, and `task_grow::allocated` holds that prediction to the store exactly
as it holds this one.

**The test that would fail if the prediction and the library's `max + 1` ever
parted asserts both spellings at once.**
`decompose_first_child_header_is_the_handle_and_filename_carries_the_kind`
(chapter 12) decomposes a leaf at key 3 and checks that the child's body begins
`# step-k4` **and** that its filename is `01-impl--step-k4.md`. Either assertion
alone would pass while the two disagreed — the body-only form under a library
that allocated something else, the filename-only form under a prediction that
wrote the wrong handle — and it is holding both together that makes it a test of
the prediction rather than of either half.
`decompose_creates_the_first_child_at_01_with_a_fresh_key` beside it checks the
filename only, and is a test of the position rather than of the handle.

**Two tests of the `None` path are inside this book's corpus**, and what they
assert on is the library's own sentence rather than grove's.
`a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping` (chapter 14,
for `materialize_finish`) and
`a_tree_at_the_last_key_refuses_the_promotion_rather_than_wrapping` (chapter 12,
for `leaf_decompose`'s first child) each stand up a tree whose only keyed entry
is `k4294967295`, and each asserts the error contains *greatest a key can be* —
which is `crates/ordinal-fs-tree/src/plan.rs`'s wording, not this crate's. That
is what makes them tests of clause 3 rather than of the arithmetic: a grove that
had raised its own refusal would fail them. Both then check the tree is
untouched. `leaf-add`'s equivalent,
`a_run_that_cannot_get_three_fresh_keys_creates_nothing_at_all`, is in
`src/task_grow/tests.rs` — the excluded file, which chapter 10 may cite and not
reproduce.

<a id="two-readings-of-an-entry"></a>
## Two small readings the later chapters need

The block ends with two short functions. Neither is about paths, and they are
here because the file is ordered by concern and this is where its addressing
section ends.

<!-- fragment «paths-live-leaf» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="508-520" parent="paths-and-addressing" -->
````rust
/// A live leaf's session kind and handle, or `None` when the entry is not one.
fn live_leaf(entry: &Entry<'_, TaskName>) -> Option<(Kind, Handle)> {
    let triple = entry.triple()?;
    match triple.parts {
        Parts::Leaf {
            outcome: Outcome::Live,
            kind,
            slug,
        } => Some((kind.clone(), Handle::new(slug.clone(), triple.key))),
        _ => None,
    }
}

````
<!-- /fragment -->

**One caller, and it is chapter 7's.** `live_leaf` answers *is this entry a leaf
that is still live, and if so what is its kind and handle* — a `Some` only for a
`Parts::Leaf` whose outcome is `Outcome::Live`. Everything else, briefs and node
directories and `DONE` and `ABANDONED` leaves alike, is `None`. It is called inside
`selected`, the shared validation and selection rule in chapter 7,
on each entry that passes key validation rather than until one
matches: `selected` collects the whole live set first, because it has to refuse a
tree carrying more than one live `finish` leaf and to prefer a non-`finish` leaf
to one, and neither is decidable from a first hit. This function is the predicate
that set is built from, and nothing more. Chapter 2 established the `Outcome` it
matches on and chapter 3 the `Parts` it destructures; this is the two of them
read together for the first time, and chapter 7 owns what is done with the
result.

<!-- fragment «paths-entry-outcome» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="521-531" parent="paths-and-addressing" -->
````rust
/// The outcome `resolve` reports for a matched entry: a leaf's own
/// live/`DONE`/`ABANDONED` state, or [`Outcome::Live`] for a node — a node
/// carries no terminal state of its own, its done-ness being the absence of a
/// live leaf in its subtree.
fn entry_outcome(entry: &Entry<'_, TaskName>) -> Outcome {
    match entry.triple().map(|triple| triple.parts) {
        Some(Parts::Leaf { outcome, .. }) => *outcome,
        _ => Outcome::Live,
    }
}

````
<!-- /fragment -->

**One caller, and it is chapter 9's.** `entry_outcome` answers what `resolve`
should report about a matched entry, and its interesting half is the fallback: a
node — and a brief — reports `Outcome::Live`. The comment gives the reason as a
fact about the domain rather than a convenience: **a node carries no terminal
state of its own, its done-ness being the absence of a live leaf in its subtree.**
That is why a node directory's name has no outcome infix to read, which chapter 3
established as a property of the type, and it is why `resolve` on a node cannot
say *done* without walking the subtree — which it does not do.

The two together are the block's quietest evidence for the chapter's rule. Both
read an entry's `triple()` and neither builds a path, which is what distinguishes
them from everything above: addressing is what grove does because the store will
not, and reading parts is what grove does because the store's `Parts` are opaque
to it. Both are domain knowledge; only one of them costs a function that could be
got wrong.

That completes the production block. Chapters 7, 8 and 9 read selection,
kind, brief-chain and resolution operations. This chapter returns below to the
inline test module's path-composition fixtures.

<a id="compositions-that-are-the-tests-alone"></a>
## The compositions that are the tests' alone

The second ownership block is where the test module opens. It is ninety lines and
it is the only block of this book whose name says *tests* and whose contents are
none: **zero of the file's sixty-three `#[test]` functions are in it**, and the
first is at line 1,066 in chapter 7's block. Sixteen of the book's thirty-nine
ownership blocks carry `test` in their id and the other fifteen hold between one
and thirty-two tests each; this is the one that holds none.

<!-- fragment «path-composition-tests» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="1002-1093" parent="source-task-tree" -->
<!-- insert «paths-tests-module-open» -->
<!-- insert «paths-tests-composed-verbs» -->
<!-- insert «paths-tests-a-kind-and-imports» -->
<!-- insert «paths-tests-brief-chain-at» -->
<!-- insert «paths-tests-fixtures» -->
<!-- /fragment -->

**So the book's per-test obligation has nothing to bite on here, and saying so is
the honest discharge of it.** The prose contract for an inline test block is to
give, per reproduced test, the property it establishes and what would have to be
true for it to pass while that property was broken. There is no reproduced test
in these ninety lines. What the block needs instead is the question its own
section comment answers: why is any of this in the test module at all?

<!-- fragment «paths-tests-module-open» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="1002-1015" parent="path-composition-tests" -->
````rust
#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    // ---- the path-taking compositions, which are the tests' alone -----------
    //
    // Every verb takes an already-open tree since `loop-crate-verbs-k21`, so
    // *open the root, then read it* is the caller's composition and no longer
    // production code here. The fixtures below all name a root, so the
    // compositions they were written against live here, where they are used.
    // The shared *opening* itself is production again since
    // `lint-lock-scope-k32` — `super::read` — because a verb finally needed one;
    // what stays here is only the open-then-call pairing each fixture wants.

````
<!-- /fragment -->

**The block is the residue of an interface change, and the comment is its
record.** Every verb takes an already-open tree since `loop-crate-verbs-k21` —
which is the signature shape chapter 1 named and chapter 5's four openings made
concrete: a caller opens the tree once and passes the guard in. *Open the root,
then read it* stopped being production code at that point and became the
caller's business. But the tests were written against fixtures that name a root
rather than hold a guard, so the pairing they want lives here, beside the
fixtures that want it. The second half of the comment records the one piece that
came back: the shared *opening* is production again since `lint-lock-scope-k32`,
because a verb finally needed one, and that is `super::read` — chapter 5's
function, called by all four of this block's compositions.

**The label `pub(crate) mod tests` rather than `mod tests` is the block's other
fact**, and it is what makes the first composition callable from outside the
module. The block declares nine functions — four compositions and five fixtures —
and exactly one of the nine is `pub(crate)`. The next fragment says which.

<!-- fragment «paths-tests-composed-verbs» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="1016-1043" parent="path-composition-tests" -->
````rust
    /// `pick`: the first **live leaf** in walk order, or `None` for a grove with no
    /// live work left — the loop's finish signal, which the CLI renders as empty
    /// stdout and a *no live leaves* diagnostic.
    ///
    /// Walk order is the library's: within a level the distinguished child first
    /// (the node file, never a leaf), then the positioned children by ordinal, with
    /// nodes descended in place — so a node at an earlier ordinal is fully explored
    /// before a later sibling. `DONE` and `ABANDONED` leaves are skipped; foreign
    /// names never reach the snapshot at all.
    pub(crate) fn pick(grove_root: &Path) -> Result<Option<PathBuf>> {
        let tree = read(grove_root)?;
        pick_in(&tree)
    }
    /// `select`: one live leaf and every launch fact about it, from one observation.
    fn select(grove_root: &Path) -> Result<Option<Selection>> {
        let tree = read(grove_root)?;
        select_in(&tree)
    }
    /// `kind [<leaf>]`: the task's session kind — whatever token the name carries, read
    /// from the filename and never from the body.
    ///
    /// With `leaf_path = Some`, that leaf; with `None`, [`pick`]'s next live leaf,
    /// and `Ok(None)` on a grove with no live work — the same signal `pick` gives.
    fn kind(grove_root: &Path, leaf_path: Option<&Path>) -> Result<Option<Kind>> {
        let tree = read(grove_root)?;
        kind_in(&tree, leaf_path)
    }

````
<!-- /fragment -->

**Three compositions, and each one is *open, then call*.** `pick` is
`read(grove_root)?` then `pick_in(&tree)`; `select` and `kind` are the same shape
over `select_in` and `kind_in`. The functions they call are the production
verbs, and none of them is this chapter's: `pick_in` and `select_in` are chapter
7's, `kind_in` is chapter 8's. `Selection`, the type `select` returns, is chapter
7's too. Each has a row in the book's early-use ledger for that reason.

**`pick` is `pub(crate)` and the other two are not, which is a fact about the
crate rather than about this file.** It is the composition other modules' tests
reach for when they need *what would `pick` say about this tree* without standing
up a guard of their own.

The three doc comments are the block's real content, and they are the clearest
statement of the verbs' semantics anywhere in the crate — which is why chapters 7
and 8 do not restate them. `pick`'s names the finish signal: `None` for a grove
with no live work left, which the CLI renders as empty stdout and a *no live
leaves* diagnostic. Its second paragraph is the walk order in five clauses — the
distinguished child first and never a leaf, then the positioned children by
ordinal, nodes descended in place, `DONE` and `ABANDONED` skipped, and foreign
names never reaching the snapshot at all. Chapter 7 owns the code and the nineteen
tests; what is stated here is what they are tests *of*. `kind`'s says the kind is
read from the filename and never from the body, and that `None` is the same
signal `pick` gives.

<!-- fragment «paths-tests-a-kind-and-imports» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="1044-1054" parent="path-composition-tests" -->
````rust
    /// A [`Kind`] for a test that needs one, by its label.
    ///
    /// A kind is an **open token** since `open-kind-k20`, so a test names the token
    /// it means rather than a variant, and an invalid one is a test bug that panics
    /// here rather than a compile error somewhere else.
    fn a_kind(label: &str) -> Kind {
        Kind::new(label).expect("a test kind must be well-formed")
    }
    use std::fs;
    use tempfile::TempDir;

````
<!-- /fragment -->

**A kind is an open token, so a test names one rather than choosing a variant.**
`open-kind-k20` is the decision — chapter 3 read the grammar it produced — and
its consequence for a test is that a bad label cannot be a compile error. `a_kind`
turns that into a panic at the point of the fixture, so a test naming a kind that
is not well-formed fails as a test bug where it was written, rather than
somewhere downstream where it would read as a claim about the code under test.
The two `use` lines that follow it sit mid-module rather than at the top, which
is where the file's own history put them.

<!-- fragment «paths-tests-brief-chain-at» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="1055-1064" parent="path-composition-tests" -->
````rust
    /// The guard composed with the verb — what a test needs to drive
    /// `brief-chain` standalone. Production never wants it: `llm_cli` holds one
    /// tree across `pick` and the ancestor walk, because selecting a leaf and
    /// reading its brief chain under two observations would be reading a tree
    /// that could move in between.
    fn brief_chain_at(grove_root: &Path, leaf_path: &Path) -> Result<Vec<PathBuf>> {
        let tree = read(grove_root)?;
        brief_chain(&tree, leaf_path)
    }

````
<!-- /fragment -->

**This is the block's most substantive doc comment and it is an argument, not a
description.** `brief_chain_at` exists only for tests, and the comment says why
production never wants it: the CLI holds **one** tree across `pick` and the
ancestor walk, because selecting a leaf and reading its brief chain under two
observations would be reading a tree that could move in between. That is the
book's second question — *against which snapshot?* — stated about a read rather
than a mutation, and it is the reason the composition here is a test-only
convenience rather than a missing verb. Chapter 8 reads `brief_chain` itself.

**The module the comment names no longer exists; the behaviour it describes
does.** There is no `llm_cli` in this workspace. The code is
`crates/grove-llm/src/cli.rs`, whose `cmd_brief_chain` opens one tree and hands
the same guard to `leaf_in` and to `verbs::brief_chain`. So the claim is right
and the address is stale — the mildest form of the class this chapter has already
met twice, since a stale name misdirects a reader rather than misleading one.
Three other comments in the crate spell it the same way: `tree_lifecycle.rs`'s
module header at line 42 which chapter 14 owns, its test module at line 1,105
which chapter 11 owns, and one in the excluded `task_grow/tests.rs`. Both
chapters will meet it again.

<!-- fragment «paths-tests-fixtures» owner="paths-are-built-here" source="crates/grove-loop/src/task_tree.rs" lines="1065-1093" parent="path-composition-tests" -->
````rust
    /// Stand up a fresh `.grove/` directory and return `(tempdir, grove_root)`.
    fn grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("_BRIEF.md"), "root brief").unwrap();
        (tmp, root)
    }

    /// Write a stub file into `dir`, returning its absolute path.
    fn touch(dir: &Path, name: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, b"# stub\n").unwrap();
        p
    }

    /// Create a node directory inside `dir`, returning its absolute path.
    fn mknode(dir: &Path, name: &str, slug: &str) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(&p).unwrap();
        fs::write(p.join(format!("_{slug}.md")), "node brief").unwrap();
        p
    }

    /// The path's final component, for terse assertions.
    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

````
<!-- /fragment -->

**Four fixtures, and the whole file is built out of them.** `grove()` stands up a
`TempDir` with a `.grove` inside it and hands back both, so the temporary
directory's lifetime is the test's; `touch` writes a stub file and returns its
absolute path; `mknode` makes a directory; `name_of` takes a path's final
component so an assertion can compare names rather than paths. Sixty-two of the
file's sixty-three tests reach `grove()` — forty-five calling it directly and
seventeen through `resolve_fixture`, a second-level fixture in chapter 9's block
that builds one nested tree out of these same three. The one test that does not
is `an_empty_reference_names_nothing_and_is_refused_before_the_tree`, and its
name says why: there is no tree to stand up. The fixtures are ad-hoc by design —
each test builds the tree it needs and discards it — which is the reason this
book's carried example is not drawn from them.

`touch`'s constant body is worth one sentence, because it marks the boundary of
what these tests can prove. **Nothing in this file reads a leaf's body**: there
is not one `read_to_string` in its 2,038 lines, and `touch` writes the same seven
bytes into every file it creates. That is the reading surface being a function of
names, made visible in a fixture. The exception proves it — chapter 8's block
adds a `touch_body` beside these, for the tests that need a body in order to
watch it be **ignored**, since `kind` reads the token from the filename and never
from the body and a fixture whose body says something else is the only way to
show it. The bodies that are read rather than written are chapters 11 to 14's,
where a leaf's `# <slug>-k<key>` handle is checked against its own filename —
which is the prediction `next_key` makes, three sections above.

That is the whole of the chapter's second block. The tree is open, a path can be
built for any entry in it and an entry found for any path under it, and every
entry can be turned into a key the library will accept or a refusal naming why it
cannot. Chapter 7 takes the first thing grove does with all of that: walk the
snapshot in the library's order and stop at the first live leaf.

Seven of this file's ten ownership blocks remain, and chapters 7 through 9
resolve them.

[Previous: Opening, contention and refusal](05-opening.md) | [Contents](README.md) | [Next: The walk: pick and select](07-the-walk.md)

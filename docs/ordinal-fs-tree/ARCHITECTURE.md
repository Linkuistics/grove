# ordinal-fs-tree

An ordered tree of entries stored as a directory tree, where each entry's
position, identity and metadata live in its **filename**. The filesystem carries
the hierarchy; the names carry everything else. There is no index, no database,
and no metadata file — a directory listing *is* the data structure.

That is the whole proposition: a tree you can read with `ls`, edit with `mv`,
diff in version control, and reason about without running the program that owns
it. The library's job is to make the operations that maintain such a tree
correct, and to make the properties it relies on checkable.

It owns the algebra — ordering, identity, traversal, and the mutations that
preserve both. It owns none of the vocabulary. What a name looks like, what
metadata it carries, and what any of it *means* are supplied by the consumer
through a single trait.

---

## The model

An **entry** is either a **leaf** — a regular file — or a **node** — a directory
holding children. A node may hold **zero or more** children; nothing requires a
node to be populated.

Every entry's name encodes four things:

| | | |
|---|---|---|
| **ordinal** | the entry's position among its siblings | mutable; the sole sort input within one level |
| **key** | the entry's identity | assigned once, unique across the whole tree, never rewritten |
| **label** | a human-facing name | not unique, not identity |
| **attributes** | whatever else the consumer needs | entirely opaque to the library |

A node may additionally hold one **distinguished child**: an entry that is the
node's *own content* rather than one of its children. It carries no ordinal and
no key, it never participates in ordering, and a node may have none. It is a
**regular file** — a walk does not descend into it, so a distinguished child
that were a directory would hide everything beneath it from every traversal.

The tree's **root** is a node that is **not an entry**. It is the directory the
consumer hands the library; it has no ordinal, no key and no parts, and its own
name is never parsed. It may hold a distinguished child like any other node.
The consequence shows up in the reading operations: whatever `ancestors`
returns, its element type cannot be the entry type, because the root is always
the last thing in the chain.

```mermaid
graph TD
    R["tree root<br/><i>a node</i>"]
    R --- D0["OVERVIEW.md<br/><i>distinguished child</i>"]
    R --- L1["01-published-orientation-i1.md<br/><i>leaf</i>"]
    R --- N2["02-linear-algebra-i2/<br/><i>node</i>"]
    R --- L3["03-draft-assessment-i9.md<br/><i>leaf</i>"]
    N2 --- D2["OVERVIEW.md<br/><i>distinguished child</i>"]
    N2 --- L4["01-published-vectors-i5.md<br/><i>leaf</i>"]
    N2 --- L5["02-draft-matrices-i6.md<br/><i>leaf</i>"]

    classDef node fill:#e8eef7,stroke:#5b7fae,color:#1a2b3d
    classDef leaf fill:#f4f1e8,stroke:#a89b6f,color:#3d3520
    classDef dist fill:#efe8f2,stroke:#8b6fa8,color:#2e1f3d
    class R,N2 node
    class L1,L3,L4,L5 leaf
    class D0,D2 dist
```

Throughout this document the examples use a course syllabus — modules and
lessons, each carrying a `draft`/`published` attribute — purely to have
something concrete to point at. None of it is known to the library.

### Why ordinal and key are separate

They answer different questions, and conflating them is the mistake this design
exists to avoid.

The **ordinal** answers *where does this sit among its siblings*. It is a
locator. Inserting an entry rewrites the ordinals of everything after it, so an
ordinal is only true until the next insert.

The **key** answers *which entry is this*. It survives insertion, reordering,
relabelling, and being moved between levels. It is the only thing safe to store
in a durable cross-reference.

Because the filesystem carries the hierarchy, an ordinal is **per-level** — it
locates an entry within one directory and says nothing about the path to it.
This is what makes insertion cheap: shifting siblings renames entries at one
level only, and each renamed node carries its entire subtree along untouched.

### Keys are the counter

A fresh key is `max(key over the whole tree) + 1`. The names *are* the counter —
there is no separate file recording the next value, because such a file would be
a second source of truth that a hand-edit could desynchronise.

The direct consequence is that **the library offers no removal operation.**
Deleting an entry lowers the visible maximum, so the next allocation re-issues a
key that other entries may still reference. A domain that genuinely needs
entries to disappear needs a key source that is not derived from the tree, and
that is outside what this library does. A domain that needs entries to be
*retired* should mark them through an attribute and leave them in place — which
costs nothing, since attributes are yours and the library never reads them.

---

## Names belong to the consumer

The library never parses a name, never formats one, and never learns that a name
is a string. It knows only that a name can be decomposed into an ordinal, a key
and an opaque remainder — and recomposed from them.

### The parse trichotomy

A directory in a real filesystem contains entries the consumer never wrote.
Classifying them is where tree libraries quietly lose data, so this is the one
place the library is opinionated: parsing yields **three** outcomes, not two.

```mermaid
flowchart TD
    A["a directory entry<br/><i>its name, and what it is on disk</i>"] --> B{"the consumer's parse"}
    B -->|"Entry"| C["a well-formed entry<br/>walk it, order it, count it"]
    B -->|"Foreign"| D["not mine<br/>ignore it completely"]
    B -->|"Malformed"| E["mine, and broken<br/>HALT, with recovery advice"]
    B -->|"Reserved"| F["mine, and not an entry<br/>HALT, with recovery advice"]

    classDef ok fill:#e4efe6,stroke:#6f9b78,color:#1f3324
    classDef ign fill:#eeeeee,stroke:#999999,color:#333333
    classDef bad fill:#f7e8e8,stroke:#b07a7a,color:#3d1f1f
    class C ok
    class D ign
    class E,F bad
```

The distinction that matters is between **Foreign** and **Malformed**. A
`README.md` sitting in the tree is foreign: skipping it is correct and costs
nothing. A name that is *almost* one of yours — a typo, a hand-edit, a mangled
attribute — is not foreign, and skipping it is data loss. When the skipped name
is a *directory*, an entire subtree vanishes from every traversal while the tree
still reports itself as healthy.

So the rule is: **a name the consumer recognises as its own must either parse
completely or halt the operation.** It may never be silently ignored. Only names
the consumer positively disclaims are skipped — and a disclaimer is recursive:
skipping a foreign *directory* skips everything beneath it, which is sound
precisely because the consumer said the name was not its own.

Classification takes the name **and what the listing says is under it**,
unfollowed. That second argument is not decoration. A name declaring a node over
a regular file is malformed by the same argument that makes a typo malformed —
it is another way a subtree vanishes — and the library can see the contradiction
but has no domain error value with which to report it. Handing `found` to
`parse` puts the judgement where the recovery advice already lives.

`Reserved` is the fourth outcome and the narrow one: a name the consumer owns
that is deliberately *not* an entry — a transaction witness, a lock marker, a
sentinel left by an interrupted operation. Its presence halts work the same way
a malformed name does, and for the same reason: the library cannot know what it
means, so proceeding past it is a guess.

Both halting outcomes carry the consumer's own error value, so refusal can say
what to *do* about it. Detection alone produces errors that are useless to
whoever hits them.

---

## The seam: one trait

All genericity lives in one trait. There are no callbacks, no hooks, no
registration, and no configuration objects.

```rust
pub trait EntryName: Sized + Clone + fmt::Display {
    /// Everything the library does not understand: the label, and whatever
    /// attributes the domain carries. Entirely opaque.
    type Parts: Clone + Eq;

    /// The domain's own error, so a refusal can carry recovery advice.
    type Err: std::error::Error + Send + Sync + 'static;

    /// Classify one directory entry: its name, and what the listing reports
    /// is under that name, unfollowed. See the parse trichotomy.
    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err>;

    /// Build a positioned name. The species follows from `parts`.
    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self;

    /// The name a node's distinguished child takes, if this domain has one.
    /// A distinguished child carries neither an ordinal nor a key, so it can
    /// never be produced by `compose` — this is the only way the library can
    /// name one. `None` means the domain has no distinguished child, and
    /// promotion is refused rather than guessed at.
    fn distinguished() -> Option<Self> { None }

    /// `Some` for a positioned name, `None` for the distinguished one. The
    /// three travel together: a name has all of them, or none of them.
    fn ordinal(&self) -> Option<Ordinal>;
    fn key(&self)     -> Option<Key>;
    fn parts(&self)   -> Option<&Self::Parts>;
    fn species(&self) -> Species;
}

pub enum Verdict<N, E> { Entry(N), Foreign, Malformed(E), Reserved(E) }
pub enum Species { Leaf, Node, Distinguished }
pub enum Found   { File, Dir, Other }
```

### What an implementation must guarantee

Five obligations, none of which the library can check. They are stated because
the structural model found that four were missing, and that a design missing any
one of them admits a tree the library will quietly corrupt.

**Compose places what it is given.** `compose(o, k, p)` yields a name whose
`ordinal()` is `o`, whose `key()` is `k` and whose `parts()` is `p`. Without
this the isomorphism below says nothing, and the sibling shift — which is
nothing but a `compose` — is free to move one entry's key onto another's
position while every stated invariant still holds.

**The grammar is canonical.** Distinct filenames never parse to the same name:
`format(parse(f)) == f`, and not merely `parse(format(n)) == n`. State one
direction only and a grammar may accept two spellings of one name — at which
point two files on disk *are* one entry, sharing a key and an ordinal, and the
tree carries a duplicate key that no invariant rules out.

**A name is positioned or distinguished, never neither.** `ordinal()`, `key()`
and `parts()` are `Some` together or `None` together. A name of species `Leaf`
with no ordinal cannot be ordered, shifted or promoted, and no triple names it.

**`distinguished()` names the only entry of its species.** `parse` yields
species `Distinguished` for that name and for nothing else. This is what makes
*at most one distinguished child per node* true — the filesystem supplies the
rest, since a directory cannot hold two entries of one name — so it is a
theorem rather than an invariant anything has to enforce.

**`parse` refuses what `found` contradicts.** A name declaring `Leaf` over a
directory, or `Node` over a regular file, is `Malformed` and never `Entry`.

### The isomorphism this rests on

A positioned entry's name is **isomorphic to a triple** — `(ordinal, key,
parts)` — and that single fact is what keeps the seam small. *Isomorphic* is
load-bearing, and it means both directions at once: `compose` recovers the name
from the triple, and the accessors recover the triple from the name. An earlier
draft stated only the string round trip, which leaves a `compose` free to ignore
its arguments entirely.

```mermaid
flowchart LR
    S["<b>a filename</b><br/>02-draft-matrices-i6.md"]
    T["<b>a triple</b><br/>ordinal 2<br/>key 6<br/>parts { draft, matrices, leaf }"]
    S -->|"parse"| T
    T -->|"compose + Display"| S

    classDef s fill:#f4f1e8,stroke:#a89b6f,color:#3d3520
    classDef t fill:#e8eef7,stroke:#5b7fae,color:#1a2b3d
    class S s
    class T t
```

Everything follows from it:

- **Shifting a sibling is not an operation.** It is
  `compose(new_ordinal, key, parts)` — derived, not implemented, and therefore
  incapable of disturbing a key, a label or an attribute.
- **The library holds no strings.** The grammar is a boundary concern; the
  algebra works on triples. So do the formal models: their state contains no
  strings at all, and the entire grammar reduces to one round-trip law.
- **The species follows from the parts.** A consumer whose leaves and nodes
  carry different metadata expresses that as variants of `Parts`, and the
  library never needs to be told which it is looking at.

### What is *not* in the trait, and why

Two things a reader might expect here are deliberately absent.

**Locking is invisible.** The library takes an advisory lock on the directory
*containing* the tree root — not the root itself. The containing directory
exists before the root is created and persists after it is deleted, so the tree's
creation and destruction fall under the same lock as every ordinary operation.
That reasoning is general, so it is the library's rule rather than a parameter.
Consumers never mention locking.

**Version control is not a concern.** Renaming an entry is `rename(2)`. The
library does not detect a repository, does not update an index, and does not
require any tool on `PATH`.

---

## How an operation runs

Every mutation follows the same four steps. Only the last touches the
filesystem.

```mermaid
flowchart LR
    subgraph guard["under one advisory lock"]
        direction LR
        A["<b>snapshot</b><br/>read the tree's<br/>names"] --> B["<b>algebra</b><br/>pure; no filesystem<br/>reachable"]
        B --> C["<b>plan</b><br/>an ordered list of<br/>primitive effects"]
        C --> D["<b>apply</b><br/>one interpreter,<br/>one rollback"]
    end
    D --> E["<b>report</b><br/>what actually<br/>happened"]

    classDef pure fill:#e4efe6,stroke:#6f9b78,color:#1f3324
    classDef io fill:#f7f0e4,stroke:#b0935f,color:#3d2e1f
    class A,B,C pure
    class D,E io
```

This is internal structure, not interface — a consumer calls
`tree.insert(...)` and receives a report. But the split is what makes the
library's claims checkable:

- **The algebra cannot reach the filesystem.** Every decision — which siblings
  move, in what order, what each is renamed to — is a pure function of the
  snapshot, so it is testable without a directory and modellable without an
  abstraction of one.
- **Atomicity is one property, not many.** A single interpreter claims each
  destination with an exclusive create and unwinds the effects it applied if a
  later one fails. Every operation inherits the same guarantee instead of
  hand-rolling its own, and they cannot drift apart.
- **The promise is bounded and stated.** Rollback covers *reported* errors.
  A process killed mid-apply is not recoverable, and the library says so rather
  than implying otherwise.

Ordering within a plan is load-bearing. A sibling shift renames
highest-ordinal-first, so every destination is vacated before it is needed —
which is a property of the plan, checkable by reading it, rather than an
accident of a loop's direction.

---

## Operations

### Reading

| operation | behaviour |
|---|---|
| `walk` | Every entry in depth-first pre-order. Within a level: the distinguished child first, then children by ordinal. Nodes are descended in place, so a node at an earlier ordinal is fully explored before a later sibling. |
| `find` | The first entry in `walk` order satisfying a predicate the caller supplies. Short-circuits. |
| `by_key` | The entry with a given key, or nothing. Keys are unique in any tree the library built; in one it did not, this returns the first in `walk` order and the caller has a tree to repair. |
| `ancestors` | An entry's containing nodes, root-first. The chain ends at the tree root, which is a node and not an entry, so its element type is not the entry type. |
| `distinguished_chain` | The distinguished child of each of an entry's ancestors, root-first, skipping levels that have none. |

There is no built-in notion of which entry is "next", "current" or "interesting",
and **no lookup by label**. Those are questions about the consumer's attributes,
and a predicate passed to `find` answers them without the library ever learning
what it asked.

Label lookup is absent because it is not expressible, not because it was left
out. The trait names no label type, so a `by_label` has nothing to take as an
argument; the only value the caller could pass is a whole `Parts`, and `Parts`
equality answers *same label* only in a domain where the label alone determines
every attribute — which contradicts `Parts` being label *plus* attributes.
`by_key` is the one lookup the library can offer, because `key()` is on the
trait and a label is not.

### Mutating

| operation | behaviour |
|---|---|
| `append` | Add a child at the end of a node: the next free ordinal, a fresh key. |
| `append_many` | Add several children at consecutive ordinals with consecutive keys, planned from one snapshot and applied as a unit. Either the whole run lands or none of it does. |
| `insert` | Add a child at an occupied ordinal, shifting the occupant and every later sibling up by one. Each shift is one rename; a shifted node carries its whole subtree. |
| `promote` | Turn a leaf into a node, **with the node's parts supplied by the caller**. The leaf's content moves verbatim into the new node's distinguished child, keeping the same ordinal and the same key — the entity is unchanged, only its shape. Optionally creates a first child in the same unit, for consumers that want both atomically. |
| `rewrite` | Replace an entry's parts, keeping its ordinal, key and species. This is how an attribute changes: the entry keeps its identity and its place, and only the opaque remainder of its name moves. Parts implying a *different* species are refused — a file cannot be renamed into a directory. |

`rewrite` is the general form of every "mark this entry" operation a consumer
might want. Because attributes are opaque, the library neither knows nor cares
what changed — it verifies that the ordinal, key and species survived, and
renames.

`promote` takes the node's parts for a reason worth stating, because the first
draft did not and the structural model showed the operation could not be
written. Species follows from parts, so naming the promoted node needs parts
that imply `Node` — and the library cannot make one. `Parts` is opaque and its
bounds are `Clone + Eq`: the library can copy a `Parts` it already holds and
compare two of them, and that is all. Every `Parts` value it can reach comes
from a name already in the tree, and none of those describes *this* entry as a
node. So the parts come from the caller, who has the domain's own constructors,
exactly as they already do for `append` and `rewrite`. The alternative — a trait
method mapping a leaf's parts to a node's — would widen the seam to serve one
operation, and would force every domain to declare a canonical leaf-to-node
mapping when the honest one is often lossy.

### Refusals

Every mutation is total: each states what it does when its precondition fails,
and none is left undefined.

- `insert` requires an existing occupant at the target ordinal. Inserting past
  the last sibling is `append`'s job and is refused rather than quietly
  redirected — the two differ in their effect on every later sibling, so
  guessing which was meant would be guessing at intent.
- `promote` applies to a leaf. A node is already a node, and a distinguished
  child has no ordinal to carry across; both are refused.
- `promote` is refused outright in a domain with no distinguished child
  (`distinguished()` is `None`), because the leaf's content would have nowhere
  to go and discarding it silently is not an option.
- `promote` is refused when the supplied parts do not imply species `Node` —
  the same check `rewrite` makes, with the opposite verdict.
- `rewrite` is refused when the new parts imply a different species.
- Every mutation is refused when its destination is occupied by anything at
  all, including a symbolic link. Occupancy is decided without following links,
  and an occupancy that *cannot* be determined is a refusal rather than an
  assumption.

---

## Invariants

Every one of these is a **preservation** property, not an establishment one. The
library never validates the tree it is handed, and a tree meant to be edited by
hand is routinely one it did not build — so each invariant reads *given a tree
that already satisfies this, every operation leaves it satisfied*. An earlier
draft said that of ordinal density alone, which implied the others were stronger
than they are; they are not, and the honest statement is the one above.

Each is tagged with the model that owns it: **[S]** for the structural model,
which asks whether a shape is well-formed, **[B]** for the behavioural one,
which asks whether an operation preserves it. The split is not decoration —
several of these cannot even be stated without a notion of *before* and *after*,
and those are exactly the ones a structural model cannot check.

**Key uniqueness.** *[S precondition, B preservation]* No key appears twice in a
tree, and no key is ever reissued. Allocation is `max + 1` over every name in
the tree, including entries a consumer considers finished — which is exactly why
they must not be deleted. Nothing checks the precondition: a hand-edited tree
with a repeated key satisfies everything the library can observe, and `by_key`
then has two answers where its type admits one. This is a defect in the tree, not
in the library, and the library's part is to not create one.

**Ordinal distinctness — and density only by induction.** *[S]* Within one node,
no two children share an ordinal. Density — ordinals being exactly `1..n` — is
*preserved* by every operation but never *established*: `append` allocates
`max + 1`, so a level that already contains a gap keeps it forever, and the
library never renumbers a level it did not just modify.

The distinction matters because this tree is meant to be edited by hand. A tree
the library built from empty is dense; a tree someone reordered with `mv` may
not be, and the library will neither notice nor repair it. **Distinctness is the
property to rely on.** Anything stronger holds only for trees nothing else has
touched, which is not a class the library can recognise.

**Distinguished children are regular files.** *[S]* A walk does not descend into
a distinguished child, so one that were a directory would hide an entire subtree
from every traversal while the tree reported itself healthy — the same failure
the Foreign/Malformed distinction exists to prevent, arriving by another road.
`parse` refuses it, since `parse` sees what is on disk.

**Subtree preservation under shift.** *[B]* An `insert` changes the ordinals of
siblings at or after the target and *nothing else*. Every key, every label,
every attribute and every descendant of every shifted entry is bit-identical
afterwards. A shifted node is one directory rename; nothing inside it is
touched.

**Identity preservation under promotion.** *[B]* A promoted leaf keeps its ordinal and
its key. The entry that was a leaf *is* the node — not a new entry that replaced
it — so every reference to it by key still resolves.

**No recognised name is silently skipped.** *[S]* Every name in a traversed directory
is classified. Names the consumer disclaims are ignored; names it recognises
either parse completely or halt the operation. There is no fourth behaviour, and
in particular there is no path by which an unparseable directory disappears from
a traversal along with everything beneath it.

**Species agreement.** *[S, discharged at the boundary]* A name declaring a leaf
is a regular file; a name declaring a node is a directory. A name whose on-disk
species contradicts what it declares is malformed, not foreign — the same
reasoning as above, since this is the other way a subtree can vanish. It holds of
every tree the library will walk because `parse` receives `found` and refuses the
contradiction; stated without that, it is an invariant no component is positioned
to enforce, since the library can see the mismatch and cannot construct the
domain error that reporting it requires.

**The trait's obligations.** *[S]* The five laws under *What an implementation
must guarantee* — compose placing what it is given, a canonical grammar, names
positioned-or-distinguished, a single distinguished name, and `parse` refusing
what `found` contradicts. These are the consumer's, not the library's, and they
are everything the library assumes about the grammar.

Two properties are deliberately **not** in this list, because nothing has to
enforce them. *At most one distinguished child per node* follows from a single
distinguished name plus a directory not holding two entries of one name. *The
parse verdict is total and disjoint* follows from `Verdict` being a sum type.
Both were checked; both are free, and a model that restates them is testing the
compiler.

**Plan atomicity.** *[B]* After a mutation returns an error, either every effect
landed or none did. Rollback removes only entries the run itself created, so it
cannot destroy something that was already there. This covers reported errors and
not process death.

---

## The models

The claims above are checked, not reviewed. The models live beside this document
and move with the crate when it is extracted.

| | |
|---|---|
| `models/structure.als` | Alloy. Whether the shape is coherent: what a well-formed tree is, what the trait can name, and which of the invariants above hold of a single tree. |
| `models/run-alloy.sh` | Runs every command and reports pass/fail. |

The file is written so that nothing this document merely *claims* is an Alloy
`fact`. Claims are named predicates, and every command says which it assumes —
which lets one file carry both the design and the reasons for it:

- a **`check`** must find no counterexample. These are the properties above.
- a **`run witness_…`** must find an instance. Each one exhibits either a defect
  this document had before the model was written — reproducible on demand, so a
  later reader can see why a law is there by watching what happens without it —
  or a structure the design deliberately admits, such as a gapped level or a
  hand-edited tree with a duplicate key.

Two of the witnesses guard against the failure mode of the style: a `check` of
the form *laws imply property* passes for no reason if the laws are
unsatisfiable, so each bundle is shown to admit a populated tree at the scope its
checks run under.

Whether each operation preserves what it should, from any reachable state, is a
question about *before* and *after* that this model cannot pose. It belongs to
the behavioural model, and the **[B]** tags above are its worklist.

---

## What this library deliberately does not do

- **No removal.** Explained above: allocation is derived from the names, so
  deletion re-issues live keys.
- **No content model.** Bytes given to `append` are written verbatim; bytes moved
  by `promote` are moved verbatim. Templates, headers and formats are the
  consumer's.
- **No format migration.** Changing a name grammar is a consumer concern, and a
  library that offered to rewrite names it does not understand would be
  offering to guess.
- **No version control integration.** A rename is a rename.
- **No schema validation.** The trait defines what a name must yield, not what it
  must contain. A consumer that wants stricter names enforces that in its own
  `parse`.
- **Unix only, for now.** The advisory lock is taken on a directory descriptor,
  which Windows has no equivalent for. Because locking is invisible in the
  interface, adding another platform later changes no signature and no caller.

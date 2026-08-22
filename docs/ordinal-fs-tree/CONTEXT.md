# ordinal-fs-tree

The vocabulary of a domain-independent ordered tree stored as a directory tree,
where each entry's position, identity and metadata live in its **filename**.
This context owns the algebra and owns none of the meaning: what a name looks
like and what it says are the consumer's. Its terms are deliberately *not*
grove's — see [Flagged ambiguities](#flagged-ambiguities) and the
[context map](../../CONTEXT-MAP.md). The design itself is
[`ARCHITECTURE.md`](ARCHITECTURE.md).

## Language

### The tree

**Entry**:
One member of a tree — either a **leaf** or a **node**. Every entry has a
**name**, and that name is the only place its **ordinal**, **key** and **parts**
are stored.
_Avoid_: "item", "record", "file" — a **node** is not a file.

**Leaf**:
An **entry** that is a regular file. It has no children.
_Avoid_: reading grove's sense here — a leaf carries no session kind, no
outcome, and no obligation to be a unit of work.

**Node**:
An **entry** that is a directory holding **zero or more** children. Nothing
requires a node to be populated, and nothing distinguishes an "interesting" one.
_Avoid_: assuming a node carries a charter, an overview, or any required
content; a **distinguished child** is optional.

**Root**:
The directory the consumer hands the library. It is a **node** but **not an
entry**: it has no ordinal, no key and no parts, and its own name is never
parsed. It may hold a **distinguished child**.
_Avoid_: typing an **ancestor chain** as a sequence of entries — the root is
always its last element.

**Distinguished child**:
The one optional **entry** that is a **node**'s own content rather than one of
its children. It is a regular file, carries neither ordinal nor key, never
participates in ordering, and a **walk** does not descend into it.
_Avoid_: "default child", "index", "self file"; and _avoid_ calling it a
**leaf** — its **species** is `Distinguished`.

**Walk**:
Traversal of a tree from a **node** downward, visiting **entries**. It descends
into nodes and never into a **distinguished child**.

**Level**:
A **root** or a **node**, seen as the thing that holds children: the unit a
**walk** orders within, and the element type of an **ancestor chain**. It exists
as its own word because the root is not an **entry**, so a sequence of levels
cannot be a sequence of entries.
_Avoid_: "directory" (that is the filesystem's word for it), "parent", "level" as
a synonym for **depth**.

**Ancestor chain**:
The sequence of **levels** from an **entry** up to and including the **root**.

### What a name carries

**Name**:
A type wrapping a filename that owns its own parsing, validation and formatting.
The library never parses one, never formats one, and never learns it is a
string.
_Avoid_: "filename" for the type and "name" for the string — the type is the
name; the string is its `Display`.

**Ordinal**:
An **entry**'s **mutable** position among the siblings in one directory, and the
sole sort input within that level. It is a locator, not an identity, and an
insert rewrites the ordinals after it.
_Avoid_: "index", "position", "order", "rank"; and _avoid_ storing one in a
durable cross-reference — use the **key**.

**Key**:
An **entry**'s identity: assigned once as `max key over the whole tree + 1`,
unique tree-wide, and never rewritten. It survives insertion, reordering,
relabelling, and being moved between levels.
_Avoid_: "id", "uid", "stable id"; and _avoid_ treating the maximum as a counter
held anywhere but in the names themselves.

**Label**:
The human-facing part of a **name**. Not unique and not identity; the library
never reads it.
_Avoid_: "slug", "title", "description".

**Attributes**:
Whatever else the consumer encodes in a **name**, entirely opaque to the
library.
_Avoid_: "metadata", "flags", "tags", "properties".

**Parts**:
The **label** and the **attributes** together — the whole of a **name** the
library does not understand, carried as one opaque associated type. It
determines the **species**.
_Avoid_: "payload", "rest", "remainder"; and _avoid_ speaking of a label or an
attribute where the library's own surface is meant — it sees only parts.

**Species**:
Which of the three kinds of thing a **name** names: `Leaf`, `Node` or
`Distinguished`. A **distinguished child** is the third; every other name is
**positioned**, and its species follows from the **parts** and from nothing
else — not from its **ordinal**, which changes under it.
_Avoid_: "type", "kind", "variant".

**Verdict**:
The outcome of classifying one directory entry: `Entry`, `Foreign`, `Malformed`
or `Reserved`. `Foreign` is a name that is not this domain's and is ignored;
`Malformed` and `Reserved` each carry the domain's own error, and a **Reserved**
name halts the operation on the domain's behalf rather than being skipped.
_Avoid_: "unknown" or "invalid" for `Foreign` — a foreign name is well outside
the grammar, not wrong within it.

### How a mutation happens

**Snapshot**:
The **names** read from a tree under one lock, and the sole input to the
**algebra**.

**Algebra**:
The pure layer that turns a **snapshot** and a request into a **decision**. It
works on `(ordinal, key, parts)` triples, never on strings, and cannot reach the
filesystem.
_Avoid_: "core", "logic", "engine".

**Decision**:
What the **algebra** returns for every input: a **plan** to apply, or a
**refusal**. Every operation is total, so there is no third answer.

**Plan**:
An ordered list of primitive **effects**, checked against itself before anything
runs.
_Avoid_: using it for anything a consumer sees — a plan is internal, and what
crosses the surface is a **report**.

**Effect**:
One primitive filesystem action in a **plan** — a rename, a create, a move.

**Interpreter**:
The single component, shared by every operation, that applies a **plan**'s
**effects** in order and unwinds the ones it applied if a later one fails.

**Report**:
What a mutating operation returns to the consumer: a description of what
happened.
_Avoid_: returning or exposing a **plan** in its place.

**Refusal**:
A stated outcome in which an operation changes nothing — a modelled result of
the **algebra**, not an error thrown from inside it.

**Shift**:
The rewriting of siblings' **ordinals** that an insert implies. It is derived
rather than implemented: each shifted **name** is recomposed from its unchanged
**key** and **parts**.
_Avoid_: "renumber", "reindex", "reorder".

**Promotion**:
Turning a **leaf** into a **node**, moving the leaf's bytes into the new node's
**distinguished child** and preserving the entry's **ordinal** and **key**.
_Avoid_: "convert", "expand", "explode".

## Flagged ambiguities

**`leaf` and `node` mean different things here than in grove.** grove's
[`CONTEXT.md`](../../CONTEXT.md) defines a **Leaf** as a task file executed in
one session and a **Node directory** as a directory headed by a `BRIEF.md`
charter. Here a leaf is *any* regular-file entry and a node is *any* directory of
children, with no charter, no session and no lifecycle. Resolution: the words
belong to whichever context you are in, and neither glossary defines the other's
sense. This divergence is the reason these are two contexts and not one.

**grove's `Position` is this context's `ordinal`; grove's `Permanent key` is
this context's `key`.** Same concepts, different words, deliberately. Resolution:
inside this context use *ordinal* and *key*; when writing about grove's task
tree use grove's words. Never mix the pairs in one sentence without saying which
tree is meant.

**"key" is not "the key of a map".** It is an identity token in a name, and it
is not a lookup handle into any data structure. Resolution: `by_key` searches
the tree for it; nothing indexes by it.

**"root" is a node that is not an entry.** Every other node is both. Resolution:
say "the root" for the handed-in directory and "a node" for anything found
inside it.

## Example dialogue

> **Consumer:** I want to delete entry 7 and reuse its slot.
>
> **Library author:** Two different things, and only one exists. Its *ordinal*
> is a slot and it is already reused — anything you insert there shifts the
> siblings after it. Its *key* is not a slot, and there is no removal
> operation, because keys are `max + 1` over the names and deleting an entry
> lowers the maximum. The next allocation would re-issue 7 while your
> cross-references still point at it.
>
> **Consumer:** So how do I retire it?
>
> **Library author:** Put it in the attributes. They are yours and the library
> never reads them, so a `retired` attribute costs a rewrite and nothing else —
> the entry keeps its key and its place.
>
> **Consumer:** Fine. And when a module gets big enough to hold lessons, I want
> to turn the module's file into a directory.
>
> **Library author:** That is promotion: the leaf becomes a node, and its bytes
> become the node's distinguished child. The ordinal and the key are preserved,
> so nothing that referenced it breaks — and a walk will not descend into the
> distinguished child, so the old content stops being visible as a child and
> starts being the node's own content.
>
> **Consumer:** What if one of my filenames is something the library can't read?
>
> **Library author:** Your `parse` decides that, not the library. If it is not
> yours at all, return `Foreign` and it is ignored. If it *is* yours and it is
> wrong, return `Malformed` — or `Reserved` if it is a name that must stop the
> operation rather than be skipped. The library never guesses which, because
> guessing wrong on a directory would silently drop everything under it.

# Growing: leaf-add and leaf-insert
<!-- book-page id="growing" slice="what-the-library-cannot-see" order="10" -->
[Previous: Resolve](09-resolve.md) | [Contents](README.md) | [Next: A grove begins](11-a-grove-begins.md)

<a id="what-the-library-cannot-see"></a>
## The rule: a precondition is worth the snapshot it is checked against

Chapters 5 to 9 read. Every verb in them answered a question and changed
nothing, which is why the whole of `task_tree.rs` could take the store's
*shared* lock and why none of its refusals could leave a tree half-built. This
is the first block in the book that changes a tree, and the seam it sits on is
the same one seen from the other side: `task_grow.rs` is 518 lines, one file,
one ownership block, and its rule is a condition on when a check is worth
anything.

> A precondition the library cannot see has to be checked against the **same
> snapshot** the operation then plans from. Checked against any other, it is a
> race with a name on it.

That is the second of the three questions this book's outcome is built from —
*on the way through, does the layer check what the library cannot see, and
against which snapshot?* — and this chapter is where it is answered in full. The
first question, the grammar, was chapters 2 to 4; the third, the policy, is Part
V. Here the cost is visible in the shape of the code rather than argued for: a
resolution, a classification and a key prediction all read `tree.snapshot()`,
and the borrow of that snapshot has to end before the operation can run, because
a mutation consumes the guard that produced it.

**This is the spine at its most explicit, because the module's own header is a
list of exactly it.** Each of this crate's large modules opens on what it kept
when the domain-free crates took the rest; this one opens on four such things,
each with its own paragraph saying why it could not move — the reference
grammar, the preconditions, the task-file template, and the cross-reference
lint. The chapter is organised around that list, and the closing section comes
back to it.

The carried example reaches its tenth step here, and it is the step where grove
**predicts** what the library is about to allocate and then checks the
prediction. Everything before this point in the example only read.

```text
.grove/                                  the grove as root-init left it
├── BRIEF.md                             the charter — no ordinal, no key
└── 01-requirements--plan-k1.md          ordinal 01, key 1

leaf-add . survey --kind impl

  refuse_finish_kind(impl, "leaf-add")           grove's own precondition, before the tree is read
  parent_node(guard, ".")       ⇒ Target::Root   `.` is a path, and a path is tried first
  next_key(snapshot)            ⇒ Some(Key(2))   chapter 6's mirror of the library's max + 1
  new_leaf(Some(2), Live, …)    ⇒ NewEntry       carrying `# survey-k2` — the key, inside the bytes
  guard.append_many(Root, [it]) ⇒ Report         one snapshot answered the ordinal and the key
  allocated(report.created(), &[Some(2)])        the prediction, held to account

  └─ .grove/02-impl--survey-k2.md        ordinal 02, key 2; its first line is `# survey-k2`
```

Read downwards, the figure is the order the chapter takes and the reason the
order is forced. The prediction at line 3 has to be made before the bytes at
line 4 exist, because the library's `NewEntry` takes its content before the
library composes the name; and the check at line 6 exists because a prediction
that silently missed would leave a file whose first line contradicts its own
filename, forever. **The observable end is not the path the verb returns.** It
is that the handle inside the file and the key in its name are the same number,
and that grove refused to report success without looking.

**This chapter's proof lies entirely outside its own pages, and it is the only
chapter in this book of which that is true.** The file
`crates/grove-loop/src/task_grow/tests.rs` is 1,680 lines and 62 tests, and it
is the book's single declared corpus exclusion: its tests are cited here by name
and are never reproduced. Every other chapter that makes a claim about what a
test establishes can put the test on the page beside it. This one cannot, and
the last three sections say what that costs and what it does not.

<a id="a-list-of-exactly-it"></a>
## The header, and the four things that could not move

The composite below is this chapter's whole ownership block. It expands, in
order, to lines 1 through 518 of the file — the whole of it — and the eighteen
fragments it names run from here to the end of the chapter.

<!-- fragment «growing-the-tree» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="1-518" parent="source-task-grow" -->
<!-- insert «grow-header-the-four» -->
<!-- insert «grow-header-what-went» -->
<!-- insert «grow-imports» -->
<!-- insert «grow-leaf-add-doc» -->
<!-- insert «grow-leaf-add» -->
<!-- insert «grow-inserted-and-insert-doc» -->
<!-- insert «grow-leaf-insert» -->
<!-- insert «grow-renumber» -->
<!-- insert «grow-renumbered» -->
<!-- insert «grow-lint-doc» -->
<!-- insert «grow-lint» -->
<!-- insert «grow-parent-node» -->
<!-- insert «grow-containing-level» -->
<!-- insert «grow-new-leaf» -->
<!-- insert «grow-allocated» -->
<!-- insert «grow-refuse-finish-kind» -->
<!-- insert «grow-template-and-stem» -->
<!-- insert «grow-test-module» -->
<!-- /fragment -->


The header opens by naming the three library operations the verbs are expressed
through and then hands the rest of the sentence to what did not go with them.

<!-- fragment «grow-header-the-four» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="1-24" parent="growing-the-tree" -->
````rust
// Grove's **grow verbs** — `leaf-add` and `leaf-insert` —
// expressed through `ordinal-fs-tree` (gh issue #13, increment 2, the *migrate*
// stage's third leaf). `append`, `append_many` and `insert` are the operations;
// what is left here is grove's own: the reference grammar, the preconditions the
// library cannot see, the task-file template, and the cross-reference lint.
//
// # What grove still owns, and why each piece could not move
//
// * **The `<parent>` / `<target>` reference.** Grove's grammar is a path, `[n]`,
//   `n`, `<slug>-k<key>` or a bare slug, which is wider than a key and has an
//   *ambiguous* outcome the library has no counterpart for. Resolution happens
//   against the guard's own snapshot — the one the operation then plans from —
//   which is clause 1 of `docs/ARCHITECTURE.md#library-refusals`.
// * **The preconditions.** A session kind, `finish`-reservation and slug
//   validity are grove's alone; so is *this parent is a node*, which grove
//   checks with the library's own predicate off the same snapshot (contents are
//   `Some`) rather than a second one (clause 2).
// * **The template.** The library has no content model, so
//   [`task_template_body`] is the bytes handed to `NewEntry::new` — and because
//   those bytes embed the key, grove predicts the allocation and checks it
//   ([`allocated`]).
// * **The cross-reference lint.** A renumber leaves position-prefixed references
//   stale, and nothing in the library knows what a reference is.
//
````
<!-- /fragment -->

**Four things, one paragraph each, and each paragraph carries its own reason.**
That is what makes this module the clearest statement of the book's spine, and
the connections it does not make are worth adding once here rather than four
times below.

The first two paragraphs cite `docs/ARCHITECTURE.md#library-refusals` by clause
number, and both citations are accurate. Clause 1 reads *resolve the argument to
an entry, then call by key — against the same snapshot the operation plans from,
which one guard already guarantees*, which is the second half of a sentence
chapter 9 began: that chapter owns the grammar and read `reference`, the door
these verbs come through, and this one supplies the snapshot the resolution is
made against. Clause 2 gives the reason the header compresses to a parenthesis —
a second predicate for one condition would let grove refuse where the library
would have proceeded — which is why the node check here asks the library's own
`contents()` and never `is_dir`.

The third and fourth paragraphs point outside the header. The template's key
prediction is chapter 6's `next_key`, whose doc comment named the check that
earns it; that check is `allocated`, forty lines from the end of this file, and
reading it is what closes the early-use row chapter 6 opened. The lint is the
only thing in this chapter that needs a *second* observation of the tree, and
the reason is at the other end of the file too.

The second half of the header turns the other way: what moved out, what was
deleted outright, and then the one thing that went nowhere.

<!-- fragment «grow-header-what-went» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="25-49" parent="growing-the-tree" -->
````rust
// # What went, and stayed gone
//
// The whole of the path-walking appender's collision machinery — the up-front
// destination sweep, the `O_EXCL` claim, the per-run rollback, the injected
// post-claim failure — because `append_many` *is* the atomic run: one snapshot answers
// every ordinal and every key, the plan is checked against itself before a byte
// is written, and the interpreter unwinds its own effects.
//
// And `leaf-add-pair`, whose three kinds were a constant here — `research-a`,
// `research-b`, `combine-research` — naming three kinds grove has no business
// knowing (`open-kind-k20`). What it was *for* survives untouched: `leaf-add`
// now takes an ordered list of kinds and lands them through the same
// `append_many`, so the pair is `--kind research-a --kind research-b --kind
// combine-research`, spelled by the methodology that owns those tokens. Deleting
// the verb and telling the skill to call `leaf-add` three times was rejected: it
// puts back the live-prefix hazard the atomic run exists to exclude.
//
// # Three helpers here are `pub(crate)`, and `tree_lifecycle` is why
//
// `new_leaf`, `refuse_finish_kind` and `allocated` are shared with
// `tree_lifecycle`. `leaf_decompose` reaches the first two: its `promote`
// optionally creates a first child in the same unit, so it composes a new leaf
// exactly as the grow verbs do. `root-init` reaches all three. One constructor
// rather than two is the point — a second spelling of *what a new grove leaf is*
// would let the two drift on the template, the slug grammar or the `finish` reservation.
````
<!-- /fragment -->

**Two headings, and the second is a boundary rather than a continuation.**
*What went, and stayed gone* takes the two departures — the appender's collision
machinery, which `append_many` made unnecessary, and `leaf-add-pair`, which was
folded into `leaf-add` — and *three helpers here are `pub(crate)`, and
`tree_lifecycle` is why* takes the one thing that went nowhere. Everything above the second
heading is an absence; everything below it is a visibility widened on purpose,
and the split is what makes the second half readable as a single claim.

**That claim is checkable from this chapter alone, and worth checking.** This
chapter reproduces every one of the file's seven `pub(crate)` functions —
`leaf_add`, `leaf_insert`, `stale_cross_refs`, `new_leaf`, `allocated`,
`refuse_finish_kind` and `task_template_body` — so a reader can hold the header's
list of three against the whole set without leaving the page. Three of the seven
are the ones `tree_lifecycle` calls. Of the remaining four, three are the verb
surface `verbs.rs` reaches, and the last is `task_template_body`, which has no
caller outside this file at all. Its only mention elsewhere in `grove-loop` is
the intra-doc link in `append_brief_suffix_in_file`'s doc comment, naming it as
the other half of a grammar that could once have drifted. The workspace's only
other mention of it is a doc comment in `grove-llm`'s `removed_surface.rs`, which
cites it as an item that is not a module.

**And the attribution stops one short of the list.** `leaf_decompose` reaches
`refuse_finish_kind` and `new_leaf` and no further: it checks its own promotion
with `promoted`, a private helper that reads the same report a second time, in
chapter 12's block rather than this one. `allocated`'s two callers are
`materialize_finish` and `initialize_grove`, and its own doc comment, forty lines
from the end of this file, names the second. Only the `root-init` path reaches
all three — `root_init` refuses a `finish` kind itself and hands the rest to
`initialize_grove` — which is why the heading credits the module and not a verb
inside it. The argument the paragraph ends on, *one constructor rather than two*,
is exactly as wide as `new_leaf` and `refuse_finish_kind`, and the third helper
rides along because it is the check that constructor's key prediction earns.

**Both of those sentences were wrong when this chapter first read them, and no
instrument in this repository could say so.** The heading pair was inverted —
*what went, and stayed gone* stood with no body, its two paragraphs sitting under
the helper heading as though they were reasons for a `pub(crate)` — and the
helper list named `leaf_slug`, a function this workspace has never contained: the
commit that created the file wrote the comment with it, over a first version that
already carried seven `pub(crate)` functions and no eighth — six of today's, plus
`surface_cross_refs` where `stale_cross_refs` now stands, and none of them a
`leaf_slug`. `grow-header-stale-helper-k154` repaired both inside the file's 518
lines, so no ownership range, manifest `lines` value or fragment range on this
page moved. Why neither defect was caught
earlier is the durable part: lines 1 to 49 are plain `//` comments rather than
`//!` inner doc comments, so rustdoc renders none of them and `cargo doc
--no-deps --document-private-items -p grove-loop` reports **no warning for this
file at all**, out of the twenty-six it emits across the crate. That is a true
result about this file's doc comments and says nothing whatever about the
forty-nine lines standing above the first one. Chapters 15 and 16 meet the same
silence over `//` headers of their own, and chapter 11 meets its opposite: a
correctly attached doc comment whose *shape* is wrong, which `cargo doc` also
passes.

**The deletion in the first half is the one place the book meets the pair verb.**
`leaf-add-pair` held three kind tokens as a constant — the three the methodology
spells for a research pair — and a crate that knows a kind is a token has no
business knowing which tokens exist. Folding it into `leaf-add` as an ordered
list of kinds kept what the verb was *for*, because the property that mattered
was never the three names: it was that the three land as one unit. That paragraph
closes by rejecting the cheaper move, calling `leaf-add` three times from the
skill, and the reason it gives is the subject of the next section.

<a id="a-list-is-not-n-calls"></a>
## `leaf-add`, and why a list of kinds is one call

The imports are the seam in miniature: eight library types, six from grove's own
grammar, and two from the reading surface the last five chapters read.

<!-- fragment «grow-imports» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="50-58" parent="growing-the-tree" -->
````rust

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use ordinal_fs_tree::{Created, Entry, Key, NewEntry, Report, Snapshot, Species, Target};

use crate::task_name::{Handle, Kind, Outcome, Parts, Slug, TaskName};
use crate::task_tree::{self, Guard, Tree};

````
<!-- /fragment -->

`Created`, `NewEntry`, `Report`, `Snapshot`, `Species` and `Target` are the
library's vocabulary for *what an operation was asked to do and what it did*,
and `Entry` and `Key` are the two the reading chapters used throughout. From
grove's own side the file takes `Handle`, `Kind`, `Outcome`, `Parts`, `Slug` and
`TaskName`, which is chapters 2 to 4 entire, and from chapters 5 to 9 it takes
`Guard`, `Tree` and the module itself. Nothing else. The verbs below reach the
tree only through functions this book has already read.

The public verb's doc comment is the longest in the file, and only its first ten
lines describe what the function does; the rest is about the methodology that
calls it and about why the verb takes a list at all.

<!-- fragment «grow-leaf-add-doc» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="59-90" parent="growing-the-tree" -->
````rust
/// `leaf-add <parent> <slug> --kind K…`: append **one leaf per kind**, in the
/// order given, under `parent` at the next free ordinals with consecutive fresh
/// keys — as one unit. `parent` is `.` for the grove root, or a node by key,
/// handle, slug or path. Returns the new leaves' absolute paths in ordinal
/// order. Working-tree only — no commit.
///
/// The kind is part of each filename; every body is the bare template — the
/// stable header and empty task sections — which the creating session then fills
/// in. All the leaves of one call share the slug, because a list of kinds is one
/// shape being cut and the kind is what tells its steps apart.
///
/// **This verb is how a review chain is built** (flat-lazy-review). A producer's
/// last act is `leaf-add <parent> <stem> --kind review-<producer>` when review is
/// required, and the review's last act is the matching `integrate-review-…` leaf
/// when it has findings worth acting on — every step slugged with the same bare
/// stem, since the kind states its role. The steps are flat siblings, so nothing
/// here knows they compose one artifact.
///
/// **And it is how a research pair is cut**, which is why the list exists at all:
/// `--kind research-a --kind research-b --kind combine-research`. A chain is
/// created lazily, one step at a time, because each step's session knows
/// something the previous one did not; a pair is created **eagerly and at once**,
/// because a `research-b` cut by `research-a`'s own session would inherit that
/// session's framing and corpus, destroying the independence the pair is run for.
///
/// # A list is not `leaf_add` called N times, and `append_many` is why
///
/// N calls would be N snapshots, N guards and N chances to stop half way — and a
/// live prefix of a shape looks exactly like a deliberately hand-cut partial one.
/// `append_many` plans the whole run from **one** snapshot, so the ordinals are
/// contiguous and the keys consecutive by construction, and applies it as a unit,
/// so either the whole list lands or none of it does.
````
<!-- /fragment -->

**Five paragraphs: two of contract, one of argument, and two between them that
are the reason the verb has a list at all.** A review chain is built one step at
a time, each producer cutting the next step when it knows what the step is for;
a research pair is cut all at once, because a `research-b` planned by
`research-a`'s own session would inherit that session's framing and lose the
independence the pair exists for. Lazy and eager are opposite policies, and the
verb supports both by taking an ordered list of kinds and saying nothing about
what the list means. That is the same restraint `open-kind-k20` bought when it
deleted the pair verb: the crate carries the mechanism and the methodology
carries the tokens. Note what the comment does *not* claim — it never says the
steps compose one artifact, and says so explicitly: *the steps are flat
siblings, so nothing here knows they compose one artifact.*

**`leaf-add` is also the public verb, and this is the module-private half of
it.** `verbs::leaf_add` is the tree-opening wrapper a caller reaches; this
function takes an already-open guard, a validated `Slug` and the kinds. Chapter
15 reads the wrapper. The distinction matters twice below: once because a
refusal the comment calls *unreachable from the verb* is unreachable because of
what the wrapper's own caller does, and once because a slug that fails
validation never arrives here at all.

**The closing section is the whole argument for the shape.** N calls would be N
snapshots, N guards and N chances to stop half way, and the failure that
produces is not a missing file — it is a *live prefix of a shape*, which on the
tree looks exactly like a partial shape somebody cut deliberately. Nothing
downstream can tell those apart, because the tree's shape is the only state
grove keeps. `append_many` plans the whole run from one snapshot, so the
ordinals are contiguous and the keys consecutive by construction rather than by
retry, and applies it as a unit.

<a id="one-snapshot-answers-every-key"></a>
## One snapshot answers every ordinal and every key

The body is twenty-six lines that never touch the filesystem until the last
three.

<!-- fragment «grow-leaf-add» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="91-131" parent="growing-the-tree" -->
````rust
pub(crate) fn leaf_add(
    tree: Guard,
    parent: &str,
    slug: &Slug,
    kinds: &[Kind],
) -> Result<Vec<PathBuf>> {
    // The CLI makes `--kind` required, so this is unreachable from the verb; it
    // is the library boundary's own answer, because a caller asking for nothing
    // would otherwise get an empty success and no leaf.
    if kinds.is_empty() {
        bail!("leaf-add writes one leaf per `--kind` and was given none");
    }
    for kind in kinds {
        refuse_finish_kind(kind, "leaf-add")?;
    }
    let (target, keys) = {
        let target = parent_node(&tree, parent)?;
        // One prediction per step, walking the same `max + 1` the library walks
        // across a run. A run whose first key would already overflow predicts
        // nothing at all, and the library refuses it.
        let first = task_tree::next_key(tree.snapshot());
        let keys: Vec<Option<Key>> = (0..kinds.len() as u32)
            .map(|step| {
                first
                    .and_then(|key| key.get().checked_add(step))
                    .map(Key::new)
            })
            .collect();
        (target, keys)
    };
    let entries = kinds
        .iter()
        .zip(keys.iter())
        .map(|(kind, key)| new_leaf(*key, Outcome::Live, kind.clone(), slug))
        .collect();
    let report = tree
        .append_many(target, entries)
        .map_err(task_tree::raised)?;
    allocated(report.created(), &keys)
}

````
<!-- /fragment -->

**The empty-kinds refusal is the library boundary answering for itself.** The
comment says it is unreachable from the verb, because the CLI makes `--kind`
required — a fact about `crates/grove-llm`, which this book does not cover — and
then says why it is written anyway: a caller asking for nothing would otherwise
get an empty success and no leaf. That is a function refusing to have a
meaningless case rather than a case anybody expects to meet, and the mutation
study at the end of the chapter confirms that nothing in the suite reaches it.

**Then the preconditions, and then a block whose braces are load-bearing.** The
`finish` refusal runs over every kind before the guard's snapshot is touched, so
a run refused for its kind never reaches a resolution or a prediction. The `let
(target, keys) = { … }` block exists because `parent_node` and `next_key` both
borrow `tree.snapshot()`, and `tree.append_many` **consumes** the guard: the
borrows have to end before the call, and a block is how that is spelled. It is
also why `parent_node` returns a library `Target` rather than the `Entry` it
resolved — an entry borrows the snapshot, a target does not.

**One prediction per step, walking the same rule the library walks across a
run.** `next_key` is chapter 6's mirror of the library's *max + 1 over every
name in the tree*, and a run of N leaves needs N consecutive keys, so grove adds
the step index to the first. The comment states the boundary case exactly: *a
run whose first key would already overflow predicts nothing at all, and the
library refuses it.* The `checked_add` makes that per step rather than per run —
a run whose first predicted key is the last one in the space predicts `Some` for
that step and `None` for the rest — and `new_leaf` turns a `None` into an entry
carrying no bytes, which is a request the library refuses with its own keyspace
refusal before any effect is built.
`a_run_that_cannot_get_three_fresh_keys_creates_nothing_at_all` is the test, and
its fixture is a tree whose greatest key is `4294967294`: the first step
predicts, the second and third cannot, and not even the first leaf lands.

**The last three lines are the whole write.** The entries are zipped from kinds
and keys, `append_many` applies them as one unit, and `allocated` compares what
came back against what was predicted. `task_tree::raised` is chapter 5's
translation of a library error into grove's own, which is why no refusal wording
is written here.

<a id="an-entry-where-the-library-names-an-ordinal"></a>
## `leaf-insert`: grove names an entry where the library names an ordinal

The insert's report type comes first, because the verb returns two things and
the second is what the lint at the end of the chapter consumes.

<!-- fragment «grow-inserted-and-insert-doc» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="132-168" parent="growing-the-tree" -->
````rust
/// What a `leaf-insert` did: where the new leaf landed, and every sibling whose
/// ordinal moved.
#[derive(Debug)]
pub struct Inserted {
    /// The new leaf's absolute path.
    pub path: PathBuf,
    /// The shifted siblings, ascending by new ordinal.
    pub renumbered: Vec<Renumber>,
}

/// `leaf-insert <target> <slug>`: insert a new leaf at the slot `target` holds,
/// shifting `target` and every later sibling in its level up by one. `target` is
/// an existing entry — leaf or node — by key, handle, slug or path. Inserting
/// past the last sibling is `leaf-add`'s job, so the target must exist.
/// Working-tree only — no commit.
///
/// # The shift rewrites zero file contents, and that is structural
///
/// Each shift is one rename of one entry whose new name is
/// `compose(new_ordinal, key, parts)` and nothing else, so it cannot disturb a
/// key, a slug or an outcome; a shifted **node** is one directory rename, with
/// its whole subtree — child names *and* keys — riding along untouched. In-file
/// `# <slug>-k<key>` headers are position-free, so nothing in any body refers to
/// the ordinal that moved. The renames run highest-ordinal-first, which keeps
/// ordinals distinct at every intermediate state rather than merely avoiding a
/// collision (`crates/ordinal-fs-tree/src/ops.rs`, *Why highest-first*).
///
/// # grove names an entry where the library names an ordinal
///
/// `insert` takes the ordinal directly, and `cli-k16` found that *good*: an
/// operator who guesses is told the level's occupied span by the refusal itself.
/// grove's operators never see it — grove reads the ordinal off the entry the
/// operator named, in the snapshot the insert plans from, so `at` is occupied by
/// construction and [`Refusal::NoOccupantAtOrdinal`](ordinal_fs_tree::Refusal)
/// is unreachable in all three of its messages. What grove owes in its place is a
/// refusal for the things a *reference* can be and an ordinal cannot: nothing at
/// all, two entries at once, the root, or the charter brief.
````
<!-- /fragment -->

**The first documented section states a property rather than a mechanism, and
the property is chapter 3's handle earning its keep.** A shifted name is
`compose(new_ordinal, key, parts)` and nothing else, so the `NN` is the only
thing in it that can change; and the identity grove writes *into* a file is the
position-free `# <slug>-k<key>` header, so nothing in any body names the ordinal
that moved. Put those two together and the **zero file contents** claim is not a
property of this verb at all — it is a consequence of a grammar decision made
seven chapters ago, and a reader can check it against chapter 3 without leaving
the book. The comment is also careful about *why* the renames run
highest-ordinal-first: not to avoid a collision, but to keep ordinals distinct
at every intermediate state. That argument is in the library's own source, under
*Why highest-first*, and this book stops at the crate boundary and does not
follow it.

**The second section is the seam's asymmetry, and it is this chapter's title.**
The library's `insert` takes an ordinal. grove's operators never type one: they
name an entry, in the same five-spelling grammar chapter 9 read, and grove reads
the ordinal off the entry it resolved — in the snapshot the insert then plans
from. So the slot is occupied by construction, and the library's
`NoOccupantAtOrdinal` cannot fire. The comment claims that for **all three** of
that refusal's messages, and the count is checkable: the library renders one
preamble and then branches three ways on where the requested ordinal falls
against the level's occupied span — below it, inside it as a gap, or past the
last sibling. Three arms, three messages, none of them reachable from here.

**What grove owes in exchange is named in the same paragraph.** An ordinal can
only be absent; a *reference* can be four other things — nothing at all, two
entries at once, the root, or the charter brief — and the first two are chapter
9's refusals while the last two are this verb's own. The body is where the four
are paid for.

<a id="the-slot-the-operator-named"></a>
## The slot the operator named, and the key that is not a twin's

`leaf_insert` is thirty-three lines and refuses in four places, two of them in a
function this book has already read.

<!-- fragment «grow-leaf-insert» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="169-202" parent="growing-the-tree" -->
````rust
pub(crate) fn leaf_insert(tree: Guard, target: &str, slug: &Slug, kind: &Kind) -> Result<Inserted> {
    refuse_finish_kind(kind, "leaf-insert")?;
    let root = tree.root().to_path_buf();
    let (level, at, key) = {
        let entry = match task_tree::reference(&root, tree.snapshot(), target)? {
            task_tree::Target::Root => bail!(
                "cannot insert at the grove root (leaf-insert takes the slot of an \
                 existing entry; use `leaf-add .` to append at the root): {}",
                root.display()
            ),
            task_tree::Target::Entry(entry) => entry,
        };
        let Some(triple) = entry.triple() else {
            bail!("cannot insert at the brief: {}", entry.name())
        };
        // The reference named *this* entry, so the slot has to be this entry's
        // and not a twin's. `addressable_key` is `marking-k32`'s finding applied
        // to an ordinal rather than to a key: a duplicated key makes *the entry
        // the operator named* ambiguous, whatever the verb then reads off it.
        task_tree::addressable_key(&root, tree.snapshot(), &entry)?;
        (
            containing_level(&root, tree.snapshot(), &entry)?,
            triple.ordinal,
            task_tree::next_key(tree.snapshot()),
        )
    };
    let entry = new_leaf(key, Outcome::Live, kind.clone(), slug);
    let report = tree.insert(level, at, entry).map_err(task_tree::raised)?;
    Ok(Inserted {
        path: allocated(report.created(), &[key])?.remove(0),
        renumbered: renumbered(&report)?,
    })
}

````
<!-- /fragment -->

**Two of the four refusals are not in this fragment, and chapter 9 owns them.**
`task_tree::reference` is the first call, and its two `bail!`s — *no entry
matches … (tried as a path under the grove root and as a key/slug)* and
*reference … is ambiguous; re-query by key* — fire before this function has an
entry to examine. That is the composition the last chapter described from the
other side, and the section on the excluded test file below is where it is
settled — including the half of chapter 9's claim about who holds the evidence
that turns out to be narrower than that page states.

**The two this function owns are the two things a reference can be that an entry
is not.** The grove root is refused with advice rather than a diagnosis — *use
`leaf-add .` to append at the root* — because inserting at the root's slot is
not a mistake about syntax but a request for the other verb. The charter brief
is refused on `triple()` returning `None`, which is the grammar's own way of
saying *this name carries no ordinal and no key*: `BRIEF.md` is an entry the
library will happily hand back and an entity grove has no slot for.

**`addressable_key` is here for a reason the verb's own return value hides.**
The reference named *this* entry, so the slot has to be this entry's and not a
twin's; a hand edit that duplicated a key makes *the entry the operator named*
ambiguous whatever the verb then reads off it. The comment calls it
`marking-k32`'s finding applied to an ordinal rather than to a key, and the
point is that the ambiguity is about **identity**, not about which field is
being read — so the guard belongs before the ordinal is taken, not before the
key is used. Chapter 6 read the function.

**Then the same three reads the append made, from the same snapshot**: the
containing level, the ordinal, and the predicted key. The block ends, the guard
is consumed by `insert`, and the report is turned into the two halves of
`Inserted` — the path, checked against the prediction by the same `allocated`
the append used, and the renumber log.

<a id="a-shift-is-one-rename"></a>
## What a shift is, as a value

`Renumber` is the shift made reportable: four fields, of which two are paths and
two are the numbers an operator reads a level by.

<!-- fragment «grow-renumber» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="203-242" parent="growing-the-tree" -->
````rust
/// One entry of a `leaf-insert` renumber: a sibling whose ordinal shifted up by
/// one. The key, the slug and the outcome (and, for a node, its whole subtree)
/// are invariant — only the `NN` in this one entry's own name changed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Renumber {
    /// Where the sibling was.
    pub from: PathBuf,
    /// Where it is now.
    pub to: PathBuf,
    /// Its old position, which is what a report of the shift reads as.
    pub from_position: u32,
    /// Its new position.
    pub to_position: u32,
}

impl Renumber {
    /// The filename it wore, for a report of the shift.
    ///
    /// A shifted entry's name is one path component by construction — the store
    /// admits no other — so this cannot fail for a renumber the store produced.
    #[must_use]
    pub fn from_name(&self) -> String {
        component(&self.from)
    }

    /// The filename it wears now.
    #[must_use]
    pub fn to_name(&self) -> String {
        component(&self.to)
    }
}

/// One path's final component, which every entry in a tree has.
fn component(path: &Path) -> String {
    path.file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy()
        .into_owned()
}

````
<!-- /fragment -->

**The type's doc comment states the invariant the shift preserves rather than
the data it carries.** The key, the slug and the outcome — and, for a node, its
whole subtree — are invariant; only the `NN` in this one entry's own name
changed. Stating it on the *report* type is deliberate: the caller that prints a
renumber is the one place a reader might otherwise conclude that something
moved, and the type it is handed says in its own words that nothing did.

**`from_name` and `to_name` exist so the report can name filenames, and
`component` is what they share.** A path's final component is the filename, and
`file_name()` returns `None` only for a path ending in `..` or with no component
at all — which is why the fallback is the whole path rather than a panic or an
empty string. The doc comment argues the case away rather than handling it: a
shifted entry's name is one path component by construction, because the store
admits no other, so this cannot fail for a renumber the store produced. The
argument is what stands in for a test here; the paths in a `Renumber` come from
the library's own report and from nowhere else.

The log itself is derived from the library's report rather than accumulated as
the renames run.

<!-- fragment «grow-renumbered» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="243-271" parent="growing-the-tree" -->
````rust
/// The renumber log, read off the library's own report rather than kept by the
/// caller as it renames.
///
/// [`Report::renamed`] is in the order the renames ran — highest ordinal first —
/// and every one of them is a shift, since an `insert` renames nothing else. The
/// log is reported ascending by new ordinal, which is how an operator reads a
/// level.
fn renumbered(report: &Report<TaskName>) -> Result<Vec<Renumber>> {
    let mut log = Vec::with_capacity(report.renamed().len());
    for renamed in report.renamed() {
        let TaskName::Positioned { ordinal, .. } = &renamed.name else {
            bail!("the library reported a shift of the charter brief, which has no ordinal")
        };
        let new_position = ordinal.get();
        log.push(Renumber {
            from: renamed.from.clone(),
            to: renamed.to.clone(),
            // A shift is `+1` by definition, so the old ordinal is derived
            // rather than remembered — the one fact `Renamed` does not carry.
            from_position: new_position
                .checked_sub(1)
                .context("the library reported a shift to ordinal 0")?,
            to_position: new_position,
        });
    }
    log.sort_by_key(|renumber| renumber.to_position);
    Ok(log)
}

````
<!-- /fragment -->

**The interesting field is the one the library does not carry, and it is derived
rather than remembered.** `Renamed` records where an entry went, not where it
was, so `from_position` is `to_position - 1` — sound only because a shift is
`+1` by definition, which the comment says in as many words. The `checked_sub`
guards the one arithmetic case that cannot happen and reports it as a broken
contract rather than silently producing a wrapped number.

**The refusal above it is the same class.** `Report::renamed` is in the order
the renames ran, highest ordinal first, and every entry in it is a shift, since
an `insert` renames nothing else — so a `TaskName::Brief` in that list would
mean the library had shifted the charter brief, which has no ordinal to shift.
Both refusals in this function are statements about the library's contract, not
about an operator's argument, and neither can be produced by anything a caller
types. The sort at the end is the one concession to the reader: the renames run
highest-first and the log is reported ascending, because that is the order an
operator reads a level in.

<a id="a-lint-never-an-auto-rewrite"></a>
## The lint, and the two things its lock scope gave up

The last of the four things that could not move is the longest-argued, and the
argument is almost entirely about a lock.

<!-- fragment «grow-lint-doc» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="272-310" parent="growing-the-tree" -->
````rust
/// The stale **position-prefixed** cross-references a `leaf-insert` renumber
/// left behind, as one `path:line: <old-name> (context)` line per hit.
///
/// A lint, never an auto-rewrite: durable references should use the stable
/// `<slug>-k<key>` handle, which a renumber never changes, so the operator
/// reviews each occurrence. A stable handle is therefore *not* surfaced (it did
/// not move); only the position-prefixed form is stale. An empty renumber log
/// means there is nothing to do.
///
/// **It scans the tree, where it used to scan the directory.** The bodies are
/// every leaf and every `BRIEF.md` the snapshot holds, which is what grove has
/// under the guard and exactly the set the reader admits. A foreign `.md` file
/// dropped inside `.grove/` by hand is no longer scanned; grove writes none, and
/// the alternative is a second, wider notion of *what is in the tree* than every
/// other verb uses.
///
/// # It consumes a **shared** guard, and returns rather than prints
///
/// Both halves are `lint-lock-scope-k32`'s, and each closes something the
/// exclusive, printing version had.
///
/// * **Shared.** The scan only reads. The property the lock is for — a hit
///   names a path nothing renamed while the hit was being read — needs
///   *writers* excluded, and `LOCK_SH` excludes them. Excluding readers as well
///   bought nothing and blocked every other reader on the worktree for the
///   length of a whole-tree content scan.
/// * **Returned, not written.** The guard is consumed here and dropped with the
///   scan, so the caller does its printing with **no lock held**. That matters
///   because a `writeln!` to a pipe whose reader has stopped draining blocks
///   forever, and under the old shape it blocked with the tree's exclusive lock
///   held — wedging every grove process on the worktree behind a stalled
///   harness. Making the hits a value rather than an effect is what makes that
///   unexpressible rather than merely not-currently-done.
///
/// What the hits give up in exchange is the claim that each was printed while
/// the tree was still held. That claim did not survive the call's own return in
/// any case — the operator reads stderr long after the guard is gone — and what
/// is load-bearing is the property that remains: every hit comes from **one
/// consistent snapshot**, rather than from a tree walked while it moved.
````
<!-- /fragment -->

**A lint, never an auto-rewrite, and the reason is the handle again.** Durable
references should use the stable `<slug>-k<key>` handle, which a renumber never
changes; only the position-prefixed form goes stale. So a stable handle is
deliberately *not* surfaced — it did not move — and every hit is something an
operator should look at rather than something a tool should silently rewrite.

**It scans the tree, where it used to scan the directory**, and the difference
is what counts as *in the tree*. The bodies are every leaf and every `BRIEF.md`
the snapshot holds, which is what grove has under the guard and exactly the set
the reader admits; a foreign `.md` file dropped into `.grove/` by hand is no
longer scanned. The comment states the alternative and rejects it — a second,
wider notion of what is in the tree than every other verb uses.

**The two halves of the lock argument are each a failure the previous shape
had.** Taking the *shared* lock rather than the exclusive one is not an
optimisation: the property the lock exists for is that a hit names a path
nothing renamed while the hit was being read, and that needs writers excluded,
which `LOCK_SH` does. Excluding readers as well blocked every other reader on
the worktree for the length of a whole-tree content scan and bought nothing.
Returning the hits rather than printing them is the sharper of the two. A
`writeln!` to a pipe whose reader has stopped draining blocks forever; under the
old shape it blocked with the tree's exclusive lock held, which wedged every
grove process on the worktree behind a stalled harness. The comment's last
sentence is the one worth keeping: making the hits a **value** rather than an
effect is what makes that failure *unexpressible* rather than merely
not-currently-done.

**And the passage ends by naming what it gave up, which is rarer than naming
what it bought.** The old shape could claim each hit was printed while the tree
was still held. That claim did not survive the call's own return in any case —
the operator reads stderr long after the guard is gone — and what remains is the
property that actually matters: every hit comes from one consistent snapshot,
rather than from a tree walked while it moved.

**Six tests hold this function and all six are in the excluded file**, under its
own `stale_cross_refs` label. Three of their names are the paragraphs above, one
for one: `surface_does_not_flag_the_stable_slug_key_handle` is the handle rule,
`surface_scans_the_tree_and_not_the_directory` is the scope change, and
`surface_scans_one_snapshot_under_a_shared_lock` is the lock — the last of those
probing `flock` in both modes while the scan runs, which is the only place in
this chapter's evidence where a lock mode is asserted rather than argued. They
are cited and not shown, like everything else this chapter is proved by.

<a id="one-consistent-snapshot"></a>
## One consistent snapshot, scanned as plain substrings

The body is a walk, a sort, and a substring search.

<!-- fragment «grow-lint» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="311-355" parent="growing-the-tree" -->
````rust
pub(crate) fn stale_cross_refs(tree: Tree, renumbered: &[Renumber]) -> Vec<String> {
    if renumbered.is_empty() {
        return Vec::new();
    }
    // A second observation, and it has to be: a mutation consumes its guard, and
    // the paths inside a shifted node moved with it, so the tree this scans is
    // the one the shift *left*.
    // The stale tokens are the *old* position-prefixed names the renumber moved
    // (`02-mid-k3`), with any `.md` extension dropped so a path reference
    // `02-mid-k3/01-impl--x-k4.md` matches the directory token. The `-k<digits>`
    // tail makes these specific enough to scan as plain substrings.
    let stale: Vec<String> = renumbered
        .iter()
        .map(|renumber| stem(&renumber.from_name()).to_string())
        .collect();

    let mut bodies: Vec<PathBuf> = tree
        .walk()
        .filter(|entry| matches!(entry.species(), Species::Leaf | Species::Distinguished))
        .map(|entry| task_tree::entry_path(tree.root(), entry))
        .collect();
    bodies.sort();

    let mut hits = Vec::new();
    for path in &bodies {
        let Ok(body) = std::fs::read_to_string(path) else {
            continue;
        };
        for (index, line) in body.lines().enumerate() {
            for token in &stale {
                if line.contains(token) {
                    hits.push(format!(
                        "{}:{}: {} ({})",
                        path.display(),
                        index + 1,
                        token,
                        line.trim()
                    ));
                }
            }
        }
    }
    hits
}

````
<!-- /fragment -->

**The first comment is the one that makes the signature make sense.** This is a
*second* observation of the tree and it has to be: a mutation consumes its
guard, and the paths inside a shifted node moved with it, so the tree this scans
is the one the shift left. The verb wrapper is what performs that second
opening, and it gives up the write guard first — two open file descriptions on
one directory do not share an `flock`, which is the self-deadlock chapter 5
introduced. Chapter 15 reads the wrapper.

**The stale tokens are the old names with `.md` dropped, and the reason is a
directory reference.** A leaf reference in a body may be written as a path —
`02-mid-k3/01-impl--x-k4.md` — in which the moved entry appears as the bare
directory token `02-mid-k3`. Stripping the extension makes one token match both
spellings. What makes the tokens safe to scan as plain substrings rather than as
a parsed grammar is the `-k<digits>` tail, and the comment says so: a
position-prefixed name ending in a key is specific enough that a substring hit
is a real hit.

**The scan admits exactly what the reader admits.** `Species::Leaf` and
`Species::Distinguished` are every task file and every `BRIEF.md` and nothing
else, and `task_tree::entry_path` — chapter 6's one place a path is built —
turns each into an absolute path. The sort is on paths, so the hits come out in
path order rather than in walk order, which is what an operator scanning a list
wants. An unreadable body is skipped rather than reported, which is the one
place this function silently drops something; nothing in the suite reaches it,
and the mutation study below says so.

**The output is a formatted string and not a type, and that is the boundary
showing.** `path:line: <old-name> (context)` is a line an operator reads, not a
value another verb consumes, so the function returns `Vec<String>` and no error
at all. Every refusal this file has is above it.

<a id="clause-two-with-the-librarys-own-predicate"></a>
## The seam: clause 2, with the library's own predicate

The file's second half is labelled *the seam*, and the label is exact: four
functions, none of them a verb, each of which converts something grove knows
into something the library can be told.

<!-- fragment «grow-parent-node» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="356-389" parent="growing-the-tree" -->
````rust
// ---------------------------------------------------------------------------
// the seam

/// The library target a `<parent>` argument names, refused here if it is not a
/// node.
///
/// **Clause 2, with the library's own predicate.** A node is an entry whose
/// `contents()` are `Some` — never a path that `is_dir` — because a second
/// predicate for one condition would let grove refuse where the library would
/// have proceeded. Grove keeps the check in front of
/// [`Refusal::TargetNotNode`](ordinal_fs_tree::Refusal) because it needs it
/// anyway: the charter brief is an entry with no key, so it cannot be handed to
/// the library as a target at all.
fn parent_node(tree: &Guard, parent: &str) -> Result<Target> {
    let root = tree.root();
    match task_tree::reference(root, tree.snapshot(), parent)? {
        task_tree::Target::Root => Ok(Target::Root),
        task_tree::Target::Entry(entry) => {
            if entry.contents().is_none() {
                bail!(
                    "parent is not a node directory (need a directory named \
                     NN-<slug>-k<key>): {}",
                    task_tree::entry_path(root, entry).display()
                );
            }
            Ok(Target::Key(task_tree::addressable_key(
                root,
                tree.snapshot(),
                &entry,
            )?))
        }
    }
}

````
<!-- /fragment -->

**Three lines of this function are the whole of clause 2, and two of them are
calls into chapters this book has already read.** `reference` is chapter 9's;
the path in the refusal message is built by chapter 6's `entry_path`, which is
the only place in the crate a path is built at all; and `addressable_key` is
chapter 6's guard against a duplicated key, here on the parent rather than on
the target. What this function adds is the ordering: the classification runs off
`tree.snapshot()`, the same snapshot `leaf_add` will hand to `append_many`, and
it converts an entry into a `Target::Key` so nothing downstream needs the
snapshot again.

**The comment's *anyway* is the load-bearing word.** Grove is not merely
preferring its own message to the library's — it needs the check regardless,
because the charter brief is an entry with no key and so cannot be handed over
as a target however the library would have worded its refusal. That is why using
the library's own `contents()` rather than an `is_dir` on the path is not a
style choice: two predicates for one condition would let grove refuse where the
library would have proceeded, and a divergence like that would be invisible
until the day the two disagreed.

**The consequence is that the library's `TargetNotNode` is unreachable from
these verbs, and the excluded test file transcribes exactly that.**
`target_not_node_is_unreachable_because_clause_two_makes_it_so` drives five
different parent arguments — the charter brief, a leaf by path, a leaf by key, a
retired leaf inside a node, and that leaf by key — through both `leaf-add` and
its three-kind form, and asserts twice for each: that grove's own *parent is not
a node directory* is what comes back, and that the library's wording never does.
The second assertion is the point. The library's `TargetNotNode` advises
*promote it first*, which is a verb grove does not have, so a message that is
correct for the store is misleading for a grove — and that collision is why the
architecture record's prediction for this row was corrected rather than kept.

The other direction of the same conversion is three lines shorter and has no
refusal at all.

<!-- fragment «grow-containing-level» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="390-410" parent="growing-the-tree" -->
````rust
/// The level an entry sits in, as a library target: the root, or the node that
/// contains it.
///
/// `leaf-insert` names the entry whose slot the new leaf takes, and `insert`
/// takes the *level* the child goes into — so the target is the entry's
/// container, which is a node by construction. That is why
/// [`Refusal::TargetNotNode`](ordinal_fs_tree::Refusal) is unreachable from this
/// verb.
fn containing_level(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    entry: &Entry<'_, TaskName>,
) -> Result<Target> {
    match entry.container().entry() {
        None => Ok(Target::Root),
        Some(node) => Ok(Target::Key(task_tree::addressable_key(
            root, snapshot, &node,
        )?)),
    }
}

````
<!-- /fragment -->

**Two verbs cannot reach `TargetNotNode`, and they cannot reach it for two
different reasons.** In `parent_node` the operator's argument is one grove
refuses before the library ever sees it. Here the library is never handed the
operator's argument at all: it is handed that entry's *container*, which is the
grove root when the entry sits at the top level and an enclosing node directory
otherwise — and a container is a node by construction rather than by check. The
architecture record's row for this verb already reads that way; it is the
`leaf-add` row that a test in the excluded file corrected.

`addressable_key` appears here for the third and last time in the file — on the
entry in `leaf_insert`, on the parent in `parent_node`, and on the container
here. Each is the same question asked of a different entry: is this the entry
the operator named, or does a duplicated key make that undecidable?

Then the constructor the header called the point of keeping three helpers
`pub(crate)`.

<!-- fragment «grow-new-leaf» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="411-433" parent="growing-the-tree" -->
````rust
/// One leaf to create: its parts, and the template body carrying the handle the
/// key predicts.
///
/// A `None` key is an exhausted keyspace, which is
/// [`Refusal::KeysExhausted`](ordinal_fs_tree::Refusal) and the library's to
/// state. The entry carries no bytes then, and never needs any: a refusal writes
/// nothing.
pub(crate) fn new_leaf(
    key: Option<Key>,
    outcome: Outcome,
    kind: Kind,
    slug: &Slug,
) -> NewEntry<Parts> {
    let parts = Parts::leaf(outcome, kind, slug.clone());
    match key {
        Some(key) => NewEntry::new(
            parts,
            task_template_body(&Handle::new(slug.clone(), key)).into_bytes(),
        ),
        None => NewEntry::empty(parts),
    }
}

````
<!-- /fragment -->

**One constructor for what a new grove leaf is, and the key is what makes it
one.** The parts and the template body are built together, from the same slug
and the same predicted key, so a caller cannot compose a leaf whose filename and
whose first line disagree by construction. A second spelling of this function
would let the grow verbs and `leaf-decompose` drift on the template, the slug
grammar or the `finish` reservation, which is the argument the header makes and
this signature keeps.

**A `None` key is not an error here and the comment says whose it is.** An
exhausted keyspace is the library's refusal to state, so `new_leaf` builds an
entry carrying no bytes at all and lets the library refuse. The justification is
one clause long and it is complete: *the entry carries no bytes then, and never
needs any: a refusal writes nothing.* This is the same division chapter 6 read
on `next_key`, which returns `None` rather than refusing, for the same reason.

<a id="the-prediction-held-to-account"></a>
## The one place the prediction is held to account

Chapter 6 read the prediction and named this function as the check that earns
it. This is the function.

<!-- fragment «grow-allocated» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="434-481" parent="growing-the-tree" -->
````rust
/// The paths of what an operation created, checked against the keys grove
/// predicted for them.
///
/// **The one place the prediction is held to account.** Grove renders the
/// handle inside each leaf's body from `task_tree::next_key`, so a key the
/// library allocated differently would leave a file whose first line contradicts
/// its own filename — silently, and forever. The check reads the key off the
/// name in the report, which is the library's own answer, and the operation has
/// already landed when it fires: a disagreement is a broken contract to report,
/// not a case to recover from, and the message says which file carries it.
///
/// Takes the created rows rather than the whole [`Report`] because one caller
/// hands over a *slice* of them: a grove's `initialize` reports the charter
/// first, which carries no key and is not a prediction anyone made
/// (`tree_lifecycle::initialize_grove`).
pub(crate) fn allocated(
    created: &[Created<TaskName>],
    predicted: &[Option<Key>],
) -> Result<Vec<PathBuf>> {
    if created.len() != predicted.len() {
        bail!(
            "the library created {} entries where {} were asked for",
            created.len(),
            predicted.len()
        );
    }
    let mut paths = Vec::with_capacity(created.len());
    for (created, predicted) in created.iter().zip(predicted) {
        let TaskName::Positioned { key, .. } = &created.name else {
            bail!(
                "the library created a charter brief, which carries no key: {}",
                created.path.display()
            )
        };
        if Some(*key) != *predicted {
            bail!(
                "the library allocated key {} where grove's template wrote {}: the \
                 handle in {} contradicts its filename and must be corrected by hand",
                key.get(),
                predicted.map_or("no key".to_string(), |key| key.get().to_string()),
                created.path.display()
            );
        }
        paths.push(created.path.clone());
    }
    Ok(paths)
}

````
<!-- /fragment -->

**The doc comment states the failure mode rather than the algorithm, and the
failure mode is what makes the check worth its lines.** Grove renders the handle
into each leaf's body from `next_key`, *before* the library has composed the
name. If the library allocated differently, the result would not be an error —
it would be a file whose first line contradicts its own filename, silently and
forever. The check reads the key off the name in the report, which is the
library's own answer rather than a re-derivation, and it fires *after* the
operation has landed. The comment is honest about what that means: a
disagreement is a broken contract to report, not a case to recover from, and so
the message names the file that carries it and says it must be corrected by
hand.

**The three refusals are three different broken contracts.** A count mismatch
means the library created a different number of entries than were asked for; a
`Brief` in the created list means it created a charter, which carries no key; a
key mismatch means the allocation rule moved. None of the three can be produced
by an operator argument, and the mutation study at the end of the chapter
reports what the suite makes of that.

**The last paragraph explains a signature that would otherwise look like an
accident.** The function takes the created *rows* rather than the whole `Report`
because one caller hands over a slice of them: a grove's `initialize` reports
the charter first, and the charter carries no key and is not a prediction anyone
made. That caller is `tree_lifecycle::initialize_grove`, which chapter 11 reads
— the function `root-init` runs, where the tree, its charter and its first leaf
are created as one store operation. For this chapter it is enough that it exists
and that it creates one keyed entry beside one unkeyed one, which is what the
slice is for. **With this fragment the early-use row chapter 6 opened is
closed**: the check it promised is on the page, and it is the same function both
grow verbs call.

<a id="the-kind-grove-will-not-create"></a>
## grove's own preconditions: the one kind an operator verb may not write

The file's last labelled section is two functions long, and the first of them is
the smallest thing in the crate that is nonetheless load-bearing.

<!-- fragment «grow-refuse-finish-kind» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="482-496" parent="growing-the-tree" -->
````rust
// ---------------------------------------------------------------------------
// grove's own preconditions

/// The `finish` kind is the driver's own and no operator verb may create one.
///
/// One of the two places grove names a kind at all, and it names it by asking
/// [`Kind::is_finish`] rather than by holding a token here: grove recognising
/// the leaf it writes itself (`open-kind-k20`).
pub(crate) fn refuse_finish_kind(kind: &Kind, verb: &str) -> Result<()> {
    if kind.is_finish() {
        bail!("`finish` is driver-reserved and cannot be created by `{verb}`");
    }
    Ok(())
}

````
<!-- /fragment -->

**The `finish` kind is the driver's own, and no operator verb may create one.**
That is a rule about the methodology, and the comment is careful to say that
grove is not interpreting one: it names the kind by asking `Kind::is_finish`
rather than by holding the token here, which is grove recognising the leaf **it
writes itself**. Chapter 3 read the type, where the two tokens grove may name
live as private constants and the crate's own doc comment claims no third kind
literal exists in the machinery.

**The count that claim rests on is checkable, and it holds.** `is_finish` has
seven call sites in six functions across the crate: the selection that finds a
sentinel and the selection that skips it, both in `task_tree`'s `selected`; the
`finish_commit` that requires one; the three refusals to decompose, retire or
prune one, in `tree_lifecycle`; and this one, which refuses to *create* one.
This function is the creating side of that set, and it is called by four callers
— `leaf-add` once per kind in the run, `leaf-insert` once, and `root-init` and
`leaf-decompose` from chapters 11 and 12.

Then the template itself, and the one helper the lint needs.

<!-- fragment «grow-template-and-stem» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="497-516" parent="growing-the-tree" -->
````rust
/// The bare task-file template: the position-free handle `# <slug>-k<key>` and
/// the empty sections the creating session fills in.
///
/// **One template, no parameters beyond the handle.** Session kind and harness
/// are launch-time configuration, and a step's relationship to its neighbours is
/// prose the *creating session* writes into the body afterwards
/// (`content/TASK-FORMAT.md`, the `**Reviews:**` / `**Integrates:**`
/// convention). That is the whole point of creating a review step late: the
/// session that cuts it knows the specific finding the step exists for, which no
/// constructor rendering a goal sentence from a handle could.
pub(crate) fn task_template_body(handle: &Handle) -> String {
    format!("# {handle}\n\n\n## Goal\n\n\n\n## Context\n\n## Done when\n\n## Notes\n")
}

/// Drop a trailing `.md` from a name so a directory-token reference matches; a
/// node directory's name has no extension and is returned as-is.
fn stem(name: &str) -> &str {
    name.strip_suffix(".md").unwrap_or(name)
}

````
<!-- /fragment -->

**One template, and no parameters beyond the handle.** The body is the
position-free handle and four empty sections, and the comment argues for each
thing it leaves out: session kind and harness are launch-time configuration, and
a step's relationship to its neighbours is prose the *creating session* writes
afterwards. The last sentence is the strongest, and it is the same argument the
verb's own doc comment made about lazy chains from the other end — the session
that cuts a review step knows the specific finding the step exists for, which no
constructor rendering a goal sentence from a handle could. A template that tried
to be helpful would have to guess it.

For the run in the worked example the format string renders exactly this, and it
is the whole of what `leaf-add` writes:

```text
# survey-k2


## Goal



## Context

## Done when

## Notes
```

Twelve lines: one names the entity, four name the sections a session must fill,
and the rest are blank. The one that names the entity carries a key the library
has not yet allocated. The test is
`add_writes_kind_in_filename_and_not_in_body`: it asserts the body starts with
the handle line and that neither `**Kind:**` nor `**Harness:**` appears anywhere
in it, which pins the two parameters the template deliberately does not take.

**`stem` is a three-line function with one caller.** Dropping a trailing `.md`
lets one stale token match both a leaf reference and the directory token inside
a path reference, and a node directory's name has no extension and comes back
unchanged. `stale_cross_refs` calls it once, on each old name, and nothing else
in the crate calls it at all.

The file ends by naming the tests it does not contain.

<!-- fragment «grow-test-module» owner="what-the-library-cannot-see" source="crates/grove-loop/src/task_grow.rs" lines="517-518" parent="growing-the-tree" -->
````rust
#[cfg(test)]
mod tests;
````
<!-- /fragment -->


<a id="proof-outside-these-pages"></a>
## The proof is outside these pages, and what that costs

Those two lines are in the corpus; the 1,680 they name are not.
`src/task_grow/tests.rs` is the book's single declared corpus exclusion — the
one row `docs/specs/walkthrough-books.md`'s exception inventory carries for this
book — so every other chapter's evidence sits on its own page and this chapter's
does not. The file holds 62 tests under eight labelled sections: `leaf-add`,
`leaf-add-pair`, *one call, one mutation*, `leaf-insert`, *insert over untracked
entries*, `stale_cross_refs`, *the reachability table, transcribed*, and *one
command, one observation of the tree*.

**What the exclusion costs is exact and it is not small.** For every other
chapter, a claim of the form *this test establishes X, and would still pass if Y
were broken* is checkable by reading down the page. Here the second half of that
sentence is an assertion about bytes the reader has to open another file to see.
The four tests `docs/specs/grove-loop-book-structure.md` names are below, each
with what it establishes and what it would still pass under; every name was
checked against the file before it was written down.

| Test | What it establishes | What it would still pass under |
|---|---|---|
| `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys` | A bare slug matching two nodes refuses, the refusal says *ambiguous* and carries both keys, and nothing is created | An implementation that listed the keys of **every** keyed entry rather than of the matches: the fixture's only two keyed entries are the two matches, so the two behaviours are indistinguishable here |
| `add_preserves_a_gap_a_hand_edit_left_rather_than_filling_it` | Appending to a level holding ordinals 01 and 05 lands at 06, not at 02 — density is preserved by every operation and established by none | Any key rule at all. The fixture's keys are 1 and 2, so `max + 1` and `count + 1` agree on the new key; the key rule is pinned by a different test, whose fixture hides a higher key in another subtree |
| `a_refused_run_does_not_consume_positions_or_keys` | A three-kind run refused for an invalid slug leaves the next `leaf-add` taking ordinal 01 and key 1 | An implementation with no rollback at all — see below |
| `insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved` | Inserting at an occupied slot gives the new leaf a **fresh** key and shifts the occupant and every later sibling up one, keys unchanged | An implementation that reached the same final level by any route: only the end state is asserted, so nothing in the test distinguishes one rename order from another that arrives there |

**The third row needs its correction stated rather than buried, and it is a
correction to `docs/specs/grove-loop-book-structure.md`.** That brief names
`a_refused_run_does_not_consume_positions_or_keys` as the pin for *a refused run
must consume nothing*. Read the test and the run it refuses never reaches this
module. Its helper drives the verb the way the CLI does — read the slug, open
the tree, call the verb, in that order — and its own doc comment says that order
is what the fixture asserts, so an invalid slug is refused before the tree is
opened and long before `leaf_add` is entered. What the test pins is therefore
that grove's front door burns no ordinal and no key, which is worth pinning and
is not the same claim. **The claim about the run itself is held by the five
tests above it in the same section**, and the most discriminating sits immediately
above it. `a_failed_run_leaves_the_next_call_a_clean_slate` fails a three-kind
run against a directory squatting on the first destination, removes the
squatter, retries, and gets back the very ordinals and keys the failed run had
planned. Two of the others reach the same conclusion from the other two
directions — a run whose third derived name crosses the filesystem's length
limit, where the interpreter unwinds its own effects and *not one of the three
survives*, and a run that cannot get three fresh keys, refused from the snapshot
before any effect is built.

**Two of chapter 9's refusals are settled from this file, and chapter 9 states
the division.** `reference` has two operator-facing `bail!`s — *no entry
matches* and *is ambiguous; re-query by key* — and both are pinned from outside
the block that owns the function. The ambiguity arm is pinned by the first row
of the table above; the *no entry matches* arm by two further tests in this same
excluded file and two more in a different crate's integration target.
[`09-resolve.md`'s *What the refusals are worth, measured*](09-resolve.md#what-the-refusals-are-worth)
carries the measurement and the attribution, and this chapter does not
re-adjudicate it. What belongs here is only the cost the exclusion imposes: for
three of those five tests the reader cannot check the claim by reading down a
page of this book.

**The excluded file also carries the fourth of this crate's stale `llm_cli`
references, and it is the one that is not stale.** Three comments in
`grove-loop` name a module `llm_cli` in the present tense; there is none in this
workspace, the code is `crates/grove-llm/src/cli.rs`, and chapter 6 adjudicated
the occurrence in its own block. The fourth reads *carried across from
`llm_cli`'s own test module at `loop-crate-verbs-k21`* — a dated provenance
note, and at that leaf's parent `src/llm_cli.rs` did exist and did carry a `mod
tests` holding exactly the read-count assertions that now sit at the end of this
file. A sentence that names a module by the name it wore on the date it gives is
not the same defect as three that name it by a name nothing wears now, and this
chapter's clause is that it does not need the fix the other three do.

**The last section of the excluded file is the reachability table,
transcribed**, and it is the strongest evidence this chapter has for the two
claims its production comments make about unreachable library refusals.
`docs/ARCHITECTURE.md#library-refusals` predicted four `Refusal` variants for
the grow verbs; two of the four are wrong, and the file's own comment says so
and then pins each correction with a test.
`target_not_node_is_unreachable_because_clause_two_makes_it_so` is described
above, and
`destination_occupied_is_unreachable_from_a_shift_however_the_tree_was_edited`
builds the hostile tree deliberately — two entries sharing a key *and* their
parts at adjacent ordinals, with the insert aimed at an unrelated third — and
shows the duplicated pair shifting past each other without a collision, which is
the second thing highest-first buys after the intermediate state.

<a id="what-the-refusals-are-worth-here"></a>
## What the refusals are worth, measured

This chapter's block is the shape chapter 9 met in `resolve_in` and chapter 8
met in `leaf_entry`: several functions with several `bail!` arms each, whose
tests assert on outcomes rather than on which clause produced them. The parent
instruction is that a coverage sentence about a block like this is worth exactly
one re-run, and reading the tests cannot answer it — most of them assert
`is_err()` or a substring that more than one arm could produce.

**The block carries eleven arms that stop or divert control flow.** Each was
replaced, one at a time, in a copy of the workspace, with a panic carrying a
sentinel, and the whole of `grove-loop`, `grove-llm` and `grove` was run against
each — 42 test binaries and 626 tests in all, of which 245 were `grove-loop`'s
inline module: the workspace as it stood when the mutation ran, one test smaller
than it is now. Ten of the copy's tests fail before any mutation, all of them in
`crates/grove-loop/tests/prompt.rs`, which requires the repository it runs in to
be a jj workspace and the copy is not one; every result below is the difference
against an unmutated control run of the same copy, so those ten cancel.

| # | Line | The arm | Tests that fail when it is replaced |
|---:|---:|---|---|
| 1 | 101 | `leaf_add`: *leaf-add writes one leaf per `--kind` and was given none* | **none** |
| 2 | 492 | `refuse_finish_kind`: *`finish` is driver-reserved and cannot be created by …* | `add_rejects_finish`, `insert_rejects_finish`, `the_growing_verbs_refuse_the_drivers_reserved_kind`, `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind` |
| 3 | 174 | `leaf_insert`: *cannot insert at the grove root* | `insert_errors_when_the_target_is_the_grove_root` |
| 4 | 182 | `leaf_insert`: *cannot insert at the brief* | `insert_errors_when_target_is_a_brief` |
| 5 | 254 | `renumbered`: *the library reported a shift of the charter brief* | **none** |
| 6 | 264 | `renumbered`: *the library reported a shift to ordinal 0* | **none** |
| 7 | 375 | `parent_node`: *parent is not a node directory* | `add_errors_when_parent_is_a_leaf_file_not_a_node`, `add_errors_when_the_parent_is_the_charter_brief`, `target_not_node_is_unreachable_because_clause_two_makes_it_so` |
| 8 | 454 | `allocated`: *the library created N entries where M were asked for* | **none** |
| 9 | 463 | `allocated`: *the library created a charter brief, which carries no key* | **none** |
| 10 | 470 | `allocated`: *the library allocated key X where grove's template wrote Y* | **none** |
| 11 | 337 | `stale_cross_refs`: an unreadable body is skipped rather than reported | **none** |

One target was interrupted during arm 11's sweep and was re-run against that arm
on its own rather than left out of the count: the `grove` binary's two
CLI-surface unit tests, which pass under it. They could not have observed it in
any case — `crates/grove/src/` contains no reference to `stale_cross_refs` or to
`leaf_insert`, so the human-facing binary has no path to the lint at all.

Each observed arm produced a small, distinct failure set, which is what
attributes the failures to it rather than merely counting them — arm 7's three
tests are disjoint from arm 2's four, and neither overlaps arms 3 and 4. That is
the control the study needs and the reason each arm was mutated separately.

**Four of the eleven are observed, and the four are exactly the preconditions an
operator can reach.** One is the kind an operator may not ask for; the other
three are everything a `<parent>` or `<target>` argument can be that grove has
to refuse itself — the grove root, the charter brief, and an entry that is not a
node. Every one of the four is held. That is a stronger result than either of
the two preceding chapters reached, and it is not luck: these four are the only
arms in the block that any argument reaches at all.

**The seven that are unobserved fall into three groups, and none of the three is
a gap in the suite.** Arm 1 is unreachable through the verb, and the comment
beside it says so and says why it is written anyway — the CLI makes `--kind`
required, so an empty list arrives only from a caller that does not exist. Arms
5, 6, 8, 9 and 10 are a single class: every one of them says *the library did
something its own contract forbids* — shifted the charter brief, shifted to
ordinal 0, created a different number of entries than were asked for, created a
charter, allocated a key it was not going to allocate. Reaching any of them
requires a library that misbehaves, which no test over an honest library can
construct. They are not untested assertions; they are assertions a test suite of
this shape cannot exercise **by construction**, and writing tests for them would
mean replacing `ordinal-fs-tree` with a lying double, which is the coupling the
seam exists to avoid. Arm 11 is the third group and is on its own: it is not a
refusal but a silent `continue` over a body that cannot be read, and nothing in
the suite builds a tree holding one.

**The distinction is worth stating because it does not hold in general.** In
`leaf_entry` five of seven refusals were unobserved and every one of them was an
ordinary operator-facing refusal; in `resolve_in` one of the three unobserved
arms was a genuine hole reachable by a `u32` overflow. Here the split falls
cleanly along *who could produce the condition*, and the reason is this block's
own shape: grove classifies before it calls, so every refusal is either grove's
own precondition — checked, and held — or a statement about the library's
answer, which only a broken library could trigger. **That is the same fact the
chapter has been making from the other direction**, and it is what chapter 21
needs: the layer that stayed pays for its preconditions with tests, and pays for
its trust in the layer beneath with assertions nobody can fire.

<a id="what-growing-kept"></a>
## What growing kept, and the part that closes here

The chapter's answer to *what did not go, and why could it not?* is the four
paragraphs of its own header, and by now each has been read against the code
that implements it. **The reference** could not move because the library has no
counterpart for an ambiguous outcome, and the price is that grove resolves
before it calls — against the guard's own snapshot, which is the whole of clause
1. **The preconditions** could not move because a session kind, a reservation
and a slug are meanings, and a store that holds no meaning cannot check them;
the price is a classification step in front of every mutation, and the
discipline that it use the library's own predicate off the same snapshot rather
than a second one. **The template** could not move because the library has no
content model, and the price is the chapter's most consequential one: bytes that
embed a key have to be written before the key is allocated, so the allocation is
predicted, and a prediction that is not checked is a file that will one day
contradict itself. **The lint** could not move because nothing beneath grove
knows what a reference is, and the price is a second observation of the tree
under a second lock.

**The cost this chapter pays is timing, and it is the one Part II exists to
show.** Every check here is cheap; what is expensive is the requirement that the
check and the operation see the same tree. That is why `parent_node` takes a
`&Guard` rather than a path, why the predictions sit inside a block whose braces
end the borrow, and why `stale_cross_refs` has to say out loud that it is
looking at a *different* tree from the one the insert planned against. A layer
that checked against a fresh read would look identical and be a race.

**With this chapter, Part II is complete.** `task_tree.rs` closed at the end of
chapter 9, all 2,038 lines of it across five chapters; `task_grow.rs` closes
here in one, and the two files together are the 2,541 lines Part II owns. Ten of
the book's twenty source-owning chapters are now written and 4,715 of its 10,557
lines are reconstructed — a shade under half — with the remaining 5,842 still
deferred to their own chapters.

Chapter 11 opens Part III, and the change is bigger than another file.
`tree_lifecycle.rs` is 2,725 lines — the largest root in the crate — and its
subject is what happens to a grove that the store has no word for at all:
beginning, outcomes, and ending. Where this chapter's verbs grew a tree that
already existed, that chapter's first verb creates one, and the charter and the
first live leaf land with it as a single store operation. `allocated` is there
too, called on a slice of a report whose first row is a charter carrying no key
— which is the signature this chapter read and could not fully explain.

[Previous: Resolve](09-resolve.md) | [Contents](README.md) | [Next: A grove begins](11-a-grove-begins.md)

# The walk: pick and select
<!-- book-page id="the-walk" slice="first-live-leaf" order="7" -->
[Previous: Paths, and addressing](06-paths.md) | [Contents](README.md) | [Next: Kind, and the brief chain](08-kind-and-briefs.md)

<a id="first-live-leaf"></a>
## The rule: the first live leaf in walk order

Chapter 6 left the tree open, every entry addressable and every path buildable.
This chapter is the first thing grove does with all of that, and it is the one
question the whole system exists to answer: what should be worked next?

> The first live leaf in walk order, and position in that walk is the only
> schedule there is.

That is grove's answer to *what did not go, and why could it not?* on the way
through, and it is a scheduling decision wearing a traversal's clothes. The
library supplies the traversal — depth-first pre-order, the distinguished child
first, then siblings by ordinal, nodes descended in place — and stops there. It
has no word for *live*, because live is the absence of a `DONE` or `ABANDONED`
infix in a grammar the library does not read. It has no word for `finish`,
because the finish sentinel is a leaf grove's own driver writes. So the layer
that stayed holds the two facts that turn an ordering into a schedule: which
entries are candidates at all, and which candidate is outranked.

The carried example reaches its seventh step here, and it starts from the tree as
`root_init` left it — a `.grove/` holding its charter and one keyed entry, which
chapter 1 named as the whole grove created in one store operation and chapter 11
reads.

```text
.grove/                              the tree as root-init left it
├── _BRIEF.md                         the distinguished child — walked first
└── 01-requirements--plan-k1.md      the one keyed entry

snapshot.walk()                      the library's order, not grove's
  │
  ├─ _BRIEF.md          live_leaf → entry.triple() is None       ⇒ not a candidate
  └─ 01-requirements…  live_leaf → Parts::Leaf { outcome: Live,
                                     kind: requirements, slug: plan }
                                                                ⇒ (requirements, plan-k1)

  live candidates in walk order:  [ 01-requirements--plan-k1.md ]
  of those, finish leaves:        [ ]                  ⇒ no refusal
  first candidate that is not a finish leaf:  the one

select_in  ⇒ Selection {
               path:   <root>/01-requirements--plan-k1.md,
               handle: plan-k1,
               kind:   requirements,
             }
pick_in    ⇒ <root>/01-requirements--plan-k1.md
```

The figure is this chapter's whole production block run end to end on two
entries, and it is here first so that each function below can be read as one move
inside it. Two of its steps belong to earlier chapters: `entry.triple()` is the
library's, and `live_leaf` is chapter 6's — the reading that turns an entry into
a session kind and a handle, or into `None`.

This chapter owns 336 lines of `task_tree.rs` in 2 blocks.
The source index records their current ranges; the fragments below reconstruct
every owned byte.

<a id="what-a-launch-needs"></a>
## What a launch needs, copied once

The chapter's first ownership block is the production run. It expands, in order,
to lines 532 through 615 of the file, and the five fragments it names run to the
end of the next section.

<!-- fragment «walk-selection» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="532-615" parent="source-task-tree" -->
<!-- insert «walk-selection-type» -->
<!-- insert «walk-pick-in» -->
<!-- insert «walk-select-in» -->
<!-- insert «walk-select-in-write» -->
<!-- insert «walk-selected» -->
<!-- /fragment -->

The type comes first, and it answers a question the walk itself does not ask:
what does a caller do with the leaf once it has one? Chapter 1 named `Selection`
in the crate's cast at low resolution — the leaf a session was launched to work,
its path, its identity and its kind — and this is the definition that closes that
row.

<!-- fragment «walk-selection-type» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="532-540" parent="walk-selection" -->
````rust
/// Everything a launch needs about one selected leaf, copied while a single
/// shared guard is held. Callers never reopen or reparse the tree before launch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub path: PathBuf,
    pub handle: Handle,
    pub kind: Kind,
}

````
<!-- /fragment -->

**The actor is grove, the input is one entry of a snapshot, and the output is
three values that outlive the guard.** The path is built by chapter 6's
`entry_path`. The handle and the kind come from chapter 6's `live_leaf`, which
reads them out of the parsed name. All three are owned values — a `PathBuf`, a
`Handle`, a `Kind` — and the struct borrows nothing from the snapshot, which is
what lets the guard drop while the selection is still in hand.

The doc comment's second sentence is the invariant, and it is stated as a
prohibition on callers: *callers never reopen or reparse the tree before launch*.
That is the book's second question — against which snapshot? — asked about a read
rather than a mutation. A caller holding only the path would have to reopen the
tree to learn the kind, and the tree it reopened could differ from the one that
chose the leaf; the launch would then be configured for a kind the selected leaf
no longer carries. Copying all three under one guard closes that, and it is why
the type exists rather than `pick` simply returning a path.

**The three fields are consumed at the far end of the crate.** `kind` names the
launch template and the skill, and chapter 20 spells the launch line with
`selection.kind.label()`; `handle` is what a diagnostic and a commit message name
the work item by, because a handle survives the renumbering a path does not;
`path` is what the session is pointed at. `crates/grove-loop/src/lib.rs`
re-exports the type at the crate root, which is the `Selection` chapter 1 met.

`pick` is this type with two fields dropped, and the file says so in one line.

<!-- fragment «walk-pick-in» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="541-546" parent="walk-selection" -->
````rust
/// `pick` against a tree already read. Used by every verb that needs a leaf
/// and its brief chain from the *same* observation.
pub(crate) fn pick_in(tree: &Tree) -> Result<Option<PathBuf>> {
    Ok(select_in(tree)?.map(|selection| selection.path))
}

````
<!-- /fragment -->

**`pick_in` is `select_in` with the handle and the kind discarded**, so every
claim the nineteen tests below make about `pick` is a claim about `select_in`
too. There is no second walk, no second ordering rule and no second definition of
*live*: the two entry points are one function and a projection.

The `_in` suffix is the crate's convention for *against a tree already read*.
Chapter 5 read the four openings a caller picks from; chapter 6 read the comment
that dates the convention — *every verb takes an already-open tree since*
`loop-crate-verbs-k21` — and the four open-then-call compositions that are what
is left of the older shape. The doc comment names why it matters here: a caller
that wants a leaf **and** its brief chain wants them from the same observation.
`pick_in` is `pub(crate)`, so the caller it describes is not one of its own three
call sites — those are `kind_in` in chapter 8's block, the `pick` composition in
chapter 6's, and one in the excluded `task_grow/tests.rs` that chapter 10 may
only cite by name. The composition itself is performed outside this crate, by
`crates/grove-llm/src/cli.rs`, whose `cmd_brief_chain` opens one tree, reaches
the same selection through the public `pick` verb, and hands that same guard to
`brief_chain`. Chapter 6 met the corresponding case in `brief_chain_at`'s
comment, which says production never wants the test-only pairing for exactly
this reason.

**The name that opens this comment is a code span, and until
`unresolved-doc-links-k151` it was a link that could never have resolved.** The
`pick` it names is not [chapter 15](15-the-verbs.md)'s public verb: that one
takes an already-open `&Tree` as well, so *against a tree already read* would
distinguish it from nothing. It is the composition
[chapter 6](06-paths.md#compositions-that-are-the-tests-alone) read at line
995 — `read(grove_root)?` and then `pick_in(&tree)` — and this comment is the
whole of the difference between the two. Written `[`pick`]` it emitted
*unresolved link to `pick`*, and **no spelling of it could have done otherwise**:
that composition sits in this file's `#[cfg(test)]` module, and `cargo doc`
compiles with `cfg(test)` off. That is the blind spot
[chapter 17](17-the-epoch.md#three-per-cent-and-a-blind-instrument) measures from
the inside, met here from the other side. Chapter 17 plants a broken link
*within* a test module and watches the crate's count not move; a link *into* one
warns and no qualified path repairs it, because there is no documented item at
the end of any path. So the repair was to unlink, which puts the production side
in step with the file's own practice. The three compositions, reproduced in
chapter 6, each name themselves with a code span — `` `pick` ``, `` `select` ``,
`` `kind [<leaf>]` `` — and the one place `task_tree.rs` writes `` [`pick`] `` as
a link and gets away with it is `kind`'s own comment at line 1,007 inside that
same test module, where it resolves.

The shared-guard entry point carries the chapter's second rule in its own doc
comment.

<!-- fragment «walk-select-in» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="547-556" parent="walk-selection" -->
````rust
/// `select` against a tree already read.
///
/// The `finish` rule is grove's alone and the library knows nothing of it: a
/// `finish` leaf is the driver's own, so ordinary work outranks it wherever it
/// sits, and more than one live `finish` leaf is a malformed tree rather than a
/// choice.
pub(crate) fn select_in(tree: &Tree) -> Result<Option<Selection>> {
    selected(tree.root(), tree.snapshot(), None)
}

````
<!-- /fragment -->

**The same repair, and this one is what settles which reading was right.**
There is no `verbs::select`: `verbs.rs` declares fourteen public functions and
`select` is not among them, `grove-llm` has no such subcommand, and a sweep of
every commit in this repository's history for the declaration finds none — with
`pub fn pick`, which the same sweep finds in 236 of them, as the control that
shows the sweep can find one. So `[`select`]` never named a verb. `tests::select`
sits 450 lines below in this same file and is exactly this function against a
tree it opens first, which is the sentence the comment is making. What is forced
for one is forced for all three: the three comments share a sentence, and only
one reading makes that sentence say anything at all.

**This is the chapter's clearest statement of what did not move.** The library
orders entries; it does not rank them. As far as the grammar is concerned
`finish` is a session kind like any other — chapter 3 read `Kind::is_finish` and
the two tokens grove spells for itself — and nothing in the store could have known
that a leaf carrying it is the driver's own teardown sentinel rather than work.
So the ranking is grove's, in two clauses. **Ordinary work outranks a finish leaf
wherever it sits**, which overrides walk order rather than refining it. **More
than one live finish leaf is malformed**, which is a refusal rather than a choice
between them.

The body passes the snapshot and its path spelling with no exclusion.
`tree.root()` is the path the caller spelled —
chapter 5's note that the inherent `root` wins over the snapshot's is what makes
that the path and not the root level — and `tree.snapshot()` is the observation.
Both come off one guard. The public `select_snapshot` operation accepts these
same values with an optional excluded permanent key for read-only consumers.

The exclusive twin is the same call against a different guard, and its comment
says why that is not a second selection.

<!-- fragment «walk-select-in-write» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="557-566" parent="walk-selection" -->
````rust
/// [`select_in`] against a tree held **exclusively**.
///
/// The lifecycle verbs select and then write from one observation — the driver's
/// finish sentinel is allocated only if the same snapshot found no live work —
/// and a mutation is on the exclusive guard. Both guards deref to a
/// [`Snapshot`], so this is the same selection and not a second one.
pub(crate) fn select_in_write(tree: &Guard) -> Result<Option<Selection>> {
    selected(tree.root(), tree.snapshot(), None)
}

````
<!-- /fragment -->

**Two functions, one body, and the only difference is which lock is held.**
`select_in` takes a `&Tree`, chapter 5's shared read guard; `select_in_write`
takes a `&Guard`, the exclusive one a mutation consumes. Both deref to the same
`Snapshot`, so each performs the same computation over the same kind of
observation. What differs is what the caller may do next.

The comment names the case that needs the exclusive form, and it is the
against-which-snapshot question in its most consequential shape: **the driver's
finish sentinel is allocated only if the same snapshot found no live work.** A
selection taken under a shared lock, released, and then acted on under an
exclusive one would be a race with a name — ordinary work inserted in between
would end up allocated behind a sentinel asserting that the grove was finished.
Both call sites are chapter 14's, in the finish transition:
`materialize_finish`, whose own comment calls the exclusive re-selection *the gap
between that read and allocation*, and `finish_commit`, which reads all three
fields of the selection to refuse a finish while live work remains.

The crate-internal function behind the guarded entry points and the public
snapshot operation owns the selection algorithm.

<!-- fragment «walk-selected» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="567-615" parent="walk-selection" -->
````rust
pub(crate) fn selected(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    excluded_key: Option<Key>,
) -> Result<Option<Selection>> {
    let mut keys = HashMap::new();
    let mut live = Vec::new();
    for entry in snapshot.walk() {
        if let Some(key) = entry.key() {
            if let Some(previous) = keys.insert(key, entry) {
                bail!(
                    "duplicate key k{key}: {} and {}; restore each item's original permanent identity from version history; never reuse a retired key",
                    entry_path(root, previous).display(),
                    entry_path(root, entry).display()
                );
            }
        }
        if let Some((kind, handle)) = live_leaf(&entry) {
            live.push((entry, kind, handle));
        }
    }
    let finish: Vec<_> = live
        .iter()
        .filter(|(_, kind, _)| kind.is_finish())
        .collect();
    if finish.len() > 1 {
        bail!(
            "multiple live `finish` leaves are malformed: {}",
            finish
                .iter()
                .map(|(entry, _, _)| entry_path(root, *entry).display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    // Validate the whole snapshot before excluding any candidate. Excluding a
    // node's identity must not exclude its children, which have their own keys.
    live.retain(|(entry, _, _)| entry.key() != excluded_key);
    let selected = live
        .iter()
        .find(|(_, kind, _)| !kind.is_finish())
        .or_else(|| live.first());
    Ok(selected.map(|(entry, kind, handle)| Selection {
        path: entry_path(root, *entry),
        handle: handle.clone(),
        kind: kind.clone(),
    }))
}

````
<!-- /fragment -->

**The actor is grove, the input is a whole snapshot, and the output is at most
one `Selection` or a refusal.** First walk every entry, rejecting a repeated key
and collecting live leaves. The key map includes terminal leaves and branches;
a duplicate outside the candidate set is still an ambiguous identity. Then
refuse multiple live finishes. Only after those checks does `retain` exclude
the named candidate. It compares each leaf's own key, so excluding a branch
does not remove its children. Finally, select the first ordinary candidate,
or the sole finish if no ordinary candidate remains. `entry_path` supplies both
the answer and the offending paths in diagnostics.

<a id="the-cost-of-the-finish-rule"></a>
## Why the walk does not stop at the first live leaf

**`selected` walks the whole tree even when the answer is the first entry it
sees**, because validity and finish eligibility depend on the whole snapshot. The
loop cannot be *walk until a live leaf, then return it*, because *ordinary work
outranks a finish leaf wherever it sits* is a claim about entries the walk has not
reached yet: a finish leaf at position 1 may not be answered until the tree is
known to hold no ordinary live leaf at any position after it. The refusal has the
same shape — *more than one live finish leaf is malformed* cannot be detected
without counting them all.

| What is asked | What it costs | Why nothing cheaper answers it |
|---|---|---|
| the first live leaf | the whole walk | a finish leaf early in the order may be outranked by work later in it |
| ordinary work outranks `finish` | one filtering pass over the candidates | rank is not order, and the library supplies only order |
| two live `finish` leaves refuse | a count over the candidates | malformedness is a fact about the whole tree, not about an entry |
| every permanent key is unique | a map over all positioned entries | terminal leaves and branches can collide even when no candidate does |

The table is the chapter's account of what grove pays for holding a schedule the
library cannot hold. The absolute cost is small — a task tree is a few dozen files —
and the point is not the expense but that the shape of the computation is dictated
by a domain rule the layer beneath has no word for.

**The refusal names paths and not handles**, which is why `entry_path` appears
inside the `bail!`. An operator told that multiple live finish leaves are
malformed must restore the intended single sentinel from version history. A
duplicate key likewise needs the original identities restored, never a reused
retired key. The wording is grove's own throughout, because the library was never asked:
from its side, two live finish leaves are two perfectly well-formed entries.

**The fallback is `live.first()` after exclusion.** If no ordinary leaf remains,
the filtered vector holds at most one finish, because validation already refused
multiple live finishes before any candidate was removed. The
`or_else` therefore reads as *the* finish leaf rather than *a* finish leaf, which
is the property `materialize_finish` depends on when it re-selects under the
exclusive guard and expects to be handed a sentinel it can reuse.

Excluding the last ordinary leaf therefore exposes a finish that an ordinary
pick would have skipped. Excluding the sole finish returns none. Selection
never materializes a sentinel; that remains the driver's mutation.

The public tests in `crates/grove-loop/tests/selection.rs` exercise these
boundaries over temporary non-jj trees: ordinary and finish exclusion, terminal
and branch keys, finish-only remainders, empty candidate sets, DFS ordering,
and ambiguities that exclusion could hide. Driver and `pick` binary tests and
the public Viewer tests cover the same duplicate-key refusal through consumers.

That closes the production block. Chapters 8 and 9 read the remaining kind,
brief-chain and resolution operations.

<a id="nineteen-tests"></a>
## The nineteen tests, and what each would still pass under

The chapter's second ownership block is the file's `pick` test section: 252 lines
under one section label, carrying nineteen `#[test]` functions — eighteen named
`pick_*` and one block-opening `select_*`. Two of the file's four blocks that
hold tests are larger. The block introduces no fixture of its own — all
nineteen of its `fn`s are the tests — and every tree below but one is built with
`grove`, `touch` and `mknode`, three of the fixtures chapter 6 read. The
exception is the symlink test, which reaches past them to `fs::write` and
`std::os::unix::fs::symlink` because no fixture makes the object it needs.

<!-- fragment «pick-tests» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1094-1345" parent="source-task-tree" -->
<!-- insert «walk-tests-select-one-observation» -->
<!-- insert «walk-tests-order» -->
<!-- insert «walk-tests-terminal-leaves» -->
<!-- insert «walk-tests-descent» -->
<!-- insert «walk-tests-fall-through» -->
<!-- insert «walk-tests-none» -->
<!-- insert «walk-tests-foreign» -->
<!-- insert «walk-tests-species-mismatch» -->
<!-- insert «walk-tests-symlink» -->
<!-- insert «walk-tests-legacy-and-absent-root» -->
<!-- /fragment -->

Each test below is given twice over: the property it establishes, and **what
would have to be true for it to pass while that property was broken**. The second
half is the part a test cannot state about itself, and for several of these the
honest answer is that the test alone establishes very little — its force comes
from a sibling that closes the reading it leaves open. Saying so is the point of
the obligation, because a reader who takes each name as an argument will
overestimate all nineteen.

| Group | Tests | The property the group holds |
|---|---:|---|
| one observation | 1 | every launch fact comes from one lock acquisition |
| order | 2 | the walk orders on the parsed ordinal, never on the name |
| terminal leaves | 2 | `DONE` and `ABANDONED` are both removed from the candidates |
| descent | 2 | a node is entered in place; a charter is never a candidate |
| fall-through | 3 | a node with no live leaf yields nothing, and the walk continues |
| no live work | 3 | `Ok(None)` is an answer rather than a failure |
| foreign names | 2 | a name the grammar disclaims is passed over, not refused |
| a name grove refuses | 2 | a task-*shaped* name at the wrong species halts the read |
| unreachable, and absent | 2 | a foreign container hides its subtree; an absent root is grove's own refusal |

The grouping is this chapter's rather than the file's — the block is one flat run
of tests under a single label — and it is here so that the reading each test
leaves open, and the sibling that closes it, can be seen as a relation rather
than reassembled from nineteen paragraphs. The ten fragments below follow source
order and hold one group each, except that the two refusals take a fragment
apiece: each carries a doc comment long enough to be read on its own.

The section label and the block's one `select` test come first. Fifteen of the
nineteen observe a returned path or the absence of one; three observe the text of
a refusal; this one is the only test in the block that observes a leaf's handle
and kind at all.

<!-- fragment «walk-tests-select-one-observation» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1094-1118" parent="pick-tests" -->
````rust
    // ---- pick ---------------------------------------------------------------

    #[test]
    fn select_returns_path_handle_and_kind_from_one_guarded_observation() {
        let (_t, g) = grove();
        let path = touch(&g, "01-review-impl--selected-k7.md");
        reset_read_count();

        let selected = select(&g).unwrap().unwrap();

        assert_eq!(
            selected,
            Selection {
                path,
                handle: Handle::parse("selected-k7").expect("a well-formed handle"),
                kind: a_kind("review-impl"),
            }
        );
        assert_eq!(
            read_count(),
            1,
            "selection must not reopen the tree to derive launch facts"
        );
    }

````
<!-- /fragment -->

**The property is the `Selection` doc comment's prohibition made checkable:
every launch fact comes from one lock acquisition.** The first assertion pins the
three values — the path the fixture built, the handle `selected-k7` parsed out of
the filename, and the kind `review-impl` — and the second pins the cost at which
they were obtained.

**What it would pass under with the property broken:** the equality assertion
alone holds for an implementation that opened the tree three times, once per
field, and reparsed the name each time. The values would be identical; only the
number of acquisitions would differ. `read_count()` is what closes that reading,
and it closes it exactly as far as chapter 5's enumeration goes — `READ_COUNT` is
incremented in two places, both of them acquisitions — so an implementation that
reached the filesystem by some third route would still read `1`. The test's force
is borrowed from a fact established elsewhere in the crate, which is why chapter
5 followed the counter forward rather than treating it as local bookkeeping.

`reset_read_count()` and `read_count()` are the counter's two accessors, defined
at lines 974 to 982 inside chapter 9's ownership block: the first sets the
thread-local count to zero, the second returns it. The reset is called after the
fixture and before the call under test rather than at the top of the test,
because other tests on the same thread have already moved the counter, and
without it `1` would mean *this thread so far* rather than *this call*. This is
the only assertion in `task_tree.rs` that reads either accessor.

The two ordering tests come next, and only the second of them discriminates
anything.

<!-- fragment «walk-tests-order» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1119-1144" parent="pick-tests" -->
````rust
    #[test]
    fn pick_returns_first_live_leaf_in_per_level_order() {
        let (_t, g) = grove();
        touch(&g, "02-impl--b-k2.md");
        touch(&g, "01-impl--a-k1.md");
        touch(&g, "10-impl--c-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl--a-k1.md");
    }

    #[test]
    fn pick_orders_numerically_not_lexically() {
        // 99 < 100 — the case a lexical sort of the rendered names fails, since
        // "100" sorts before "99". The old spelling of this test made the point
        // with an unpadded `2-…`, which the canonical grammar now refuses
        // (`docs/adr/task-names-are-canonical.md`); the zero-padding is a
        // *minimum* width, so a three-digit ordinal is the discriminating case
        // that survives, and it is a stronger one — the walk orders on the
        // parsed ordinal, never on the name.
        let (_t, g) = grove();
        touch(&g, "100-impl--b-k2.md");
        touch(&g, "99-impl--a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "99-impl--a-k1.md");
    }

````
<!-- /fragment -->

**`pick_returns_first_live_leaf_in_per_level_order` establishes that the answer
is the smallest ordinal rather than the order the entries were created in.** The
fixture writes `02`, then `01`, then `10`, and expects `01`, so an implementation
returning the most recently created entry — or the entry the directory listing
happened to yield first — fails.

**What it would pass under with the property broken:** a lexical sort of the
rendered names. `"01" < "02" < "10"` bytewise as well as numerically, so this
fixture cannot tell the two orderings apart. It also passes under an
implementation that ignores position entirely and returns the lowest key, because
key 1 and ordinal 01 coincide here. What this test establishes is *not creation
order*, and nothing finer.

**`pick_orders_numerically_not_lexically` is the test that closes the first half
of that, and the discriminating pair is `100` against `99`.** Bytewise `"100" <
"99"`, so a lexical implementation answers `100-impl--b-k2.md` and fails;
numerically 99 < 100, so the walk answers `99-impl--a-k1.md`. What the pair has
to do is cross a digit-count boundary, and under this grammar the first such
boundary is the one between 99 and 100. Chapter 4 read the canonical rule
that a position is zero-padded to **at least two digits and carries no other
leading zero**, so 9 renders as `09` and 10 as `10`, and `"09" < "10"` bytewise
as well as numerically: a pair that crosses no digit-count boundary discriminates
nothing at all. That is what the comment records. The point used to be made with
an unpadded `2-…`; `docs/adr/task-names-are-canonical.md` made that spelling
uncomputable, and a three-digit ordinal is the surviving form of the same
argument — a stronger one, because it holds against the canonical grammar rather
than against a spelling the grammar now refuses.

**What the second would pass under with the property broken:** an implementation
that sorted on the parsed ordinal *descending* returns `100` and fails, but one
that sorted on the key returns `99-impl--a-k1.md` and passes, because key 1 sits
on the lower ordinal in this fixture. Neither ordering test separates ordinal
from key. That separation is made in chapter 3, at the type rather than in a
test: an ordinal is a position and a key is an identity, and `leaf-insert` moves
the first while never touching the second.

Terminal leaves come next, and the pair has a reading that a substring would
satisfy.

<!-- fragment «walk-tests-terminal-leaves» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1145-1164" parent="pick-tests" -->
````rust
    #[test]
    fn pick_skips_done_leaves() {
        let (_t, g) = grove();
        touch(&g, "01-DONE-impl--a-k1.md");
        touch(&g, "02-impl--b-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-impl--b-k2.md");
    }

    #[test]
    fn pick_skips_abandoned_leaves() {
        // Symmetric with DONE (pruning): an abandoned leaf is a terminal
        // state, skipped exactly like a retired one.
        let (_t, g) = grove();
        touch(&g, "01-ABANDONED-impl--a-k1.md");
        touch(&g, "02-impl--b-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-impl--b-k2.md");
    }

````
<!-- /fragment -->

**The property is that both terminal outcomes remove a leaf from the
candidates.** Chapter 6's `live_leaf` matches `Parts::Leaf { outcome:
Outcome::Live, .. }` and answers `None` for anything else, so the two marks are
handled by one pattern rather than by two checks, and this pair is the evidence
that the pattern covers both.

**The clearest way both tests can pass with the property broken is an
implementation that skips any filename *containing* the substring `DONE`.**
`ABANDONED` contains those four letters in the middle of the word, so a
substring test passes `pick_skips_abandoned_leaves` for entirely the wrong
reason, and passes `pick_skips_done_leaves` as well. Neither test
distinguishes *the parsed outcome was terminal* from *the filename contained four
particular letters*. What does distinguish them lies outside this block: chapter 2
read the `Outcome` type, whose `strip` matches the literal prefixes `DONE-` and
`ABANDONED-` and otherwise reports `Live` while consuming nothing. So a name is
not refused for carrying an unrecognised infix; it is refused further along, when
whatever `strip` left over fails to be a kind. `01-UNDONE-impl--a-k1.md` is
malformed because `UNDONE-impl` is not a token — chapter 4 read `Kind::new` and
the `BadKind` refusal — and the distinction is observable: `01-done-impl--a-k1.md`
is a perfectly good **live** leaf whose session kind is `done-impl`, because
`strip` is case-sensitive and `done-` is not `DONE-`. These two tests are evidence
for the walk's use of the parse, not for the parse.

Both also expect the *second* entry, so an implementation returning the last live
leaf rather than the first passes both.
`pick_returns_first_live_leaf_in_per_level_order` is the sibling that closes that
reading, which is why the ordering pair sits ahead of these.

The comment on the second test ends with a bare `(pruning)`. That is one of the
two shapes this crate cites an architecture anchor in — the other writes
`docs/ARCHITECTURE.md#pruning` out in full — and `<a id="pruning"></a>` is where
it lands.

Descent is next, and it is the one clause of the library's walk order that is
visible from grove's side at all.

<!-- fragment «walk-tests-descent» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1165-1188" parent="pick-tests" -->
````rust
    #[test]
    fn pick_descends_a_node_in_preorder() {
        // A node at an earlier position is fully explored before a later sibling
        // leaf: the node's first live child wins.
        let (_t, g) = grove();
        let node = mknode(&g, "01-k1", "design");
        touch(&node, "_design.md");
        touch(&node, "01-impl--child-k2.md");
        touch(&g, "02-impl--later-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl--child-k2.md");
    }

    #[test]
    fn pick_skips_briefs_and_returns_the_child_leaf() {
        let (_t, g) = grove();
        touch(&g, "_BRIEF.md");
        let node = mknode(&g, "01-k1", "node");
        touch(&node, "_node.md");
        touch(&node, "01-impl--child-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl--child-k2.md");
    }

````
<!-- /fragment -->

**`pick_descends_a_node_in_preorder` establishes that a node is explored in
place**, so the node at ordinal 01 is exhausted before the leaf at ordinal 02 is
considered. All the disagreement between pre-order and level-order needs is a
node with a live child and a later sibling leaf, which is what the fixture builds
— a level-order walk visits both top-level entries before entering the node, and
would answer `02-impl--later-k3.md`.

**What it would pass under with the property broken:** any implementation that
flattened the tree and sorted globally by rendered name, since
`"01-impl--child-k2.md" < "02-impl--later-k3.md"`.

**And one broken implementation passes every test in the block**, which is worth
stating plainly because it is the largest hole these nineteen leave: *return the
deepest live leaf, breaking ties by walk order*. Discriminating it from
*the first live leaf in pre-order* needs a tree in which a shallow live leaf sits
at an earlier ordinal than a deeper one — a top-level leaf at 01 and a node at 02
holding a live child. No fixture in this block has that shape. The five trees
that contain both a node and a live leaf all put the node first, so the
pre-order answer and the deepest answer coincide in every one of them, and the
distinction the chapter's rule turns on is left to the library's `walk` and to
the store's own tests.

**`pick_skips_briefs_and_returns_the_child_leaf` establishes that a charter is
never a candidate**, at the root level and inside a node — the fixture places one
of each. The mechanism is that the library walks the distinguished child *first*,
so `_BRIEF.md` is the very first entry `selected` sees at each level and is
rejected before any leaf is examined: `entry.triple()` answers `None` for the
distinguished child, and `live_leaf`'s `?` turns that into `None`.


A test using only the root marker cannot distinguish a distinguished-name
check from a hard-coded `_BRIEF.md` check. The node-file integration fixture
varies the titled file and verifies that it does not become a separate resolve
candidate or acquire a kind. Initialization supplies `TaskName::Brief`, while
promotion supplies `TaskName::NodeFile`.


Three fall-through tests follow, and together they close the reading the descent
pair left open.

<!-- fragment «walk-tests-fall-through» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1189-1227" parent="pick-tests" -->
````rust
    #[test]
    fn pick_falls_through_an_all_done_node_to_a_later_live_leaf() {
        // A node whose subtree is entirely retired yields no live leaf, so pick
        // moves on to the next sibling.
        let (_t, g) = grove();
        let node = mknode(&g, "01-k1", "done-node");
        touch(&node, "_done-node.md");
        touch(&node, "01-DONE-impl--child-k2.md");
        touch(&g, "02-impl--live-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-impl--live-k3.md");
    }

    #[test]
    fn pick_falls_through_a_pruned_node_to_a_later_live_leaf() {
        // A node whose only leaf was pruned yields no live leaf either — the
        // grove's two terminal leaf states (DONE, ABANDONED) behave identically
        // for the walk (pruning).
        let (_t, g) = grove();
        let node = mknode(&g, "01-k1", "dead-node");
        touch(&node, "_dead-node.md");
        touch(&node, "01-ABANDONED-impl--child-k2.md");
        touch(&g, "02-impl--live-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-impl--live-k3.md");
    }

    #[test]
    fn pick_descends_nested_nodes() {
        let (_t, g) = grove();
        let n1 = mknode(&g, "01-k1", "outer");
        touch(&n1, "_outer.md");
        let n2 = mknode(&n1, "01-k2", "inner");
        touch(&n2, "_inner.md");
        touch(&n2, "01-impl--deep-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl--deep-k3.md");
    }

````
<!-- /fragment -->

**The two fall-through tests establish that a node holds no state of its own.** A
node is done when its subtree holds no live leaf, and there is no mark on the
directory to read — chapter 3 established that as a property of the type, since
`Parts::Node` carries no outcome field at all. So a node whose only child is
terminal has to be *walked* before it can be known to contribute nothing, and the
walk must then continue to the next sibling rather than stopping.

**What they would pass under with the property broken:** an implementation that
never descended into node directories at all answers `02-impl--live-k3.md` in
both. That is exactly the reading `pick_descends_a_node_in_preorder` rules out,
and it is why this group is evidence only taken together: one test shows the walk
enters a node, two show it comes back out.

**`pick_descends_nested_nodes` establishes that descent is recursive rather than
one level deep.** The fixture nests a node inside a node, and the answer is the
leaf two levels down.


A test using only the root marker cannot distinguish a distinguished-name
check from a hard-coded `_BRIEF.md` check. The node-file integration fixture
varies the titled file and verifies that it does not become a separate resolve
candidate or acquire a kind. Initialization supplies `TaskName::Brief`, while
promotion supplies `TaskName::NodeFile`.


The `None` cases come next, and what they hold is that an empty answer is an
answer.

<!-- fragment «walk-tests-none» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1228-1257" parent="pick-tests" -->
````rust
    #[test]
    fn pick_none_when_only_briefs_and_done_leaves() {
        let (_t, g) = grove();
        touch(&g, "_BRIEF.md");
        let node = mknode(&g, "01-k1", "node");
        touch(&node, "_node.md");
        touch(&node, "01-DONE-impl--child-k2.md");
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_none_when_every_remaining_leaf_is_abandoned() {
        // A grove whose only remaining leaves are abandoned reports "no live
        // leaves" — correct: the work is settled, however it settled.
        let (_t, g) = grove();
        touch(&g, "_BRIEF.md");
        touch(&g, "01-ABANDONED-impl--a-k1.md");
        let node = mknode(&g, "02-k2", "node");
        touch(&node, "_node.md");
        touch(&node, "01-DONE-impl--b-k3.md");
        touch(&node, "02-ABANDONED-impl--c-k4.md");
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_none_on_empty_tree() {
        let (_t, g) = grove();
        assert_eq!(pick(&g).unwrap(), None);
    }

````
<!-- /fragment -->

**The property is that `Ok(None)` is the finish signal and not a failure.** All
three assert on `pick(&g).unwrap()`, and the `unwrap` takes the `Result`, so a
refusal would panic and the test would fail with a message rather than compare
against `None`. That is the load-bearing half. A grove with nothing left to do
must be *answerable*, because chapter 15's `Sought::Nothing` is what the loop
reads as the trigger to move to the finish transition; an error in that position
would stop the loop instead of finishing it.

**What they would pass under with the property broken:** an implementation that
answered `None` for any tree it could not make sense of, including one it should
have refused. `pick_none_on_empty_tree` passes under an implementation that always
answers `None`. The three fixtures differ in what they exclude rather than in what
they assert — the first has a node whose only child is retired, the second mixes
both terminal marks at both levels, and the third has no entries at all — and what
rules out *always `None`* is every other test in the block. That is the ordinary
shape of a negative case: it is checkable only against the positives beside it.

The comment on the second states the domain fact plainly, and it is the reason
there are two terminal marks rather than one: the work is settled, however it
settled. Abandoning is not failing, and grove has no third answer for it.

Foreign names are next, and this pair tests a leniency grove does not itself
implement.

<!-- fragment «walk-tests-foreign» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1258-1275" parent="pick-tests" -->
````rust
    #[test]
    fn pick_lenient_on_foreign_files() {
        let (_t, g) = grove();
        touch(&g, "README.md");
        touch(&g, "notes.txt");
        touch(&g, "01-impl--a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl--a-k1.md");
    }

    #[test]
    fn pick_none_when_only_foreign_files() {
        let (_t, g) = grove();
        touch(&g, "README.md");
        touch(&g, "notes.txt");
        assert_eq!(pick(&g).unwrap(), None);
    }

````
<!-- /fragment -->

**The property is that a name the grammar disclaims is passed over rather than
refused**, whether or not a live leaf remains. `README.md` and `notes.txt` are
neither task-shaped nor the distinguished name, so grove's `EntryName`
implementation answers `Verdict::Foreign` for each and the library drops them
before the snapshot exists. Chapter 2 read the four verdicts, and chapter 4 reads
the parse that produces this one — its comment names this very fixture, saying
that everything outside the positioned-and-keyed shape is Foreign and skipped,
*which is safe precisely because we are disclaiming it*, and that a stray
`README.md` lands there. Chapter 6 reproduced `pick`'s own doc comment stating
the consequence: foreign names never reach the snapshot at all. So by the time
`selected` walks, there is nothing left to skip.

**What both would pass under with the property broken:** any `selected` that does
no filtering of its own, because there is nothing left for it to filter. The
first still requires a walk that returns the live leaf — `pick(&g).unwrap()
.unwrap()` panics on `None` — so a `selected` that did nothing at all passes only
the second. What neither can observe is *where* the foreign names went, since the
filtering happens two layers down, at classification.
What they are really evidence for is that grove's own grammar answers `Foreign`
rather than `Malformed` for these two names — a claim about `TaskName::parse`,
which chapter 4 owns, observed through `pick` because `pick` is where an
operator would meet it. The pairing is deliberate: the first shows a foreign name
does not displace a live leaf, the second that a directory of nothing but foreign
names is a grove with no live work rather than a broken one.

The contrast with the next two tests is the chapter's cleanest line. A name grove
disclaims is skipped and costs nothing; a name grove *claims* and finds at the
wrong species halts everything.

<!-- fragment «walk-tests-species-mismatch» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1276-1304" parent="pick-tests" -->
````rust
    /// Both species mismatches at a **task-shaped** name are malformed, not
    /// foreign — and a later live leaf must not paper over them. The old answer
    /// (skip both, return `02-impl--real-k2.md`) is what let a hand-typed
    /// `01-DONE-node-k1/` swallow a whole live subtree: `pick` reported the grove
    /// finished while real work sat inside. Skipping is safe only for names the
    /// grow verbs would never write, and both of these are names they *do* write —
    /// at the other species.
    #[test]
    fn pick_refuses_a_species_mismatch_at_a_task_shaped_name() {
        for (make, name, expected) in [
            (
                &(|dir: &Path, name: &str| mknode(dir, name, "topic"))
                    as &dyn Fn(&Path, &str) -> PathBuf,
                "01-impl--trap-k1.md",
                "names a leaf",
            ),
            (&|d: &Path, n: &str| touch(d, n), "01-k1", "names a node"),
        ] {
            let (_t, g) = grove();
            make(&g, name);
            touch(&g, "02-impl--real-k2.md");

            let error = pick(&g).unwrap_err().to_string();

            assert!(error.contains(name), "{name}: {error}");
            assert!(error.contains(expected), "{name}: {error}");
        }
    }

````
<!-- /fragment -->

**This is the spine's clearest case in the whole book, and the doc comment is
the argument rather than a description.** The property is that a task-*shaped*
name whose species contradicts what the listing found under it is malformed
rather than foreign, and that a later live leaf does not paper over it. Both
cases are exercised: a *directory* named `01-impl--trap-k1.md`, which declares a
leaf, and a *file* named `01-k1`, which declares a node. Each refusal names
the offending entry and says which species the name declares — chapter 4 read the
`Display` arm that writes *names a leaf, which must be a regular file, but the
listing found a directory*, and its closing clause is this test's reason: a walk
that skipped it would lose everything under it.

**The store would have accepted both entries.** Neither is a filesystem error and
neither is ambiguous to the library, which has no opinion about what a name means;
the refusal comes from the grammar grove kept, applied to a listing the library
reported faithfully. That is the what-could-not-move test on the way in, and it
is met **before** this chapter's code runs: `disagreement` is called inside
`TaskName::parse`, which the library runs while classifying entries, so the `pick`
composition fails inside `read` and `selected` is never reached. These two tests
are chapter 4's grammar observed through chapter 7's verb, because `pick` is
where an operator meets it.

The comment records the old answer and the failure that changed it: skipping both
returned `02-impl--real-k2.md`, and that is what let a hand-typed
`01-DONE-node-k1/` swallow a whole live subtree — `pick` reported the grove
finished while real work sat inside. Chapter 4 read the sibling refusal for that
exact shape, whose message says an outcome infix on a directory hides every leaf
under it. Skipping is safe only for names the grow verbs would never write, and
both of these are names those verbs *do* write, at the other species.

**What it would pass under with the property broken:** the two assertions are
substring tests over a rendered error, so any refusal mentioning the filename and
the phrase *names a leaf* or *names a node* passes — including one raised at open
time, before the walk ran, and including one whose statement of the required
object was wrong, since `must be` and what follows it are not asserted on. The
test pins that `pick` surfaces a refusal for these two trees; it does not pin
where the refusal was produced or that its advice is correct.

The loop is what forces the `&dyn Fn` in the first element. `mknode` and `touch`
have the same signature but are distinct function items with distinct types, so a
two-element array holding both needs a common one; `&mknode as &dyn Fn(&Path,
&str) -> PathBuf` supplies it, and the second element takes the same type. The
coercion is there for the table, not for the call.

The symlink test is the same rule reaching a case nobody wrote it for.

<!-- fragment «walk-tests-symlink» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1305-1322" parent="pick-tests" -->
````rust
    /// The species rule reaches symlinks for free, and closing that is the point
    /// rather than a side effect: `DirEntry::file_type` does not follow links, so a
    /// symlink is neither a regular file nor a directory. Under the old `!is_dir`
    /// test a symlink at a leaf name *passed* as a leaf, and `pick` would hand the
    /// driver a mandate whose path resolves outside `.grove/` entirely.
    #[test]
    fn pick_refuses_a_symlink_at_a_task_shaped_name() {
        let (t, g) = grove();
        let outside = t.path().join("outside.md");
        fs::write(&outside, b"# outside\n").unwrap();
        std::os::unix::fs::symlink(&outside, g.join("01-impl--linked-k1.md")).unwrap();

        let error = pick(&g).unwrap_err().to_string();

        assert!(error.contains("01-impl--linked-k1.md"), "{error}");
        assert!(error.contains("must be a regular file"), "{error}");
    }

````
<!-- /fragment -->

**The property is that the species check does not follow links.**
`DirEntry::file_type` reports the link itself, so a symlink is `Found::Other` —
neither a regular file nor a directory — and a task-shaped leaf name over it
contradicts the listing exactly as a directory would. The comment is explicit
that closing this is the point rather than a side effect: under the earlier
`!is_dir` test a symlink at a leaf name *passed* as a leaf, and `pick` would hand
the driver a mandate whose path resolves outside `.grove/` entirely.

**What it would pass under with the property broken:** the assertion is again a
substring, and *must be a regular file* appears in the directory case too, so the
assertion alone does not distinguish `Found::Other` from `Found::Dir`. The fixture
is what makes it a test. The link's target is a real regular file that the fixture
writes *outside* the grove, so an implementation that followed links would see a
regular file, admit the name as a well-formed leaf, and return `Ok(Some(..))` —
at which point `unwrap_err` panics. The discrimination is in the tree, not in the
assertion.

The last two tests are a pair by position rather than by subject, and each closes
one edge of the walk.

<!-- fragment «walk-tests-legacy-and-absent-root» owner="first-live-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1323-1345" parent="pick-tests" -->
````rust
    #[test]
    fn pick_ignores_a_legacy_done_directory() {
        // A stray `done/` directory (or any foreign dir) is not a node and holds
        // no live leaf reachable by the walk.
        let (_t, g) = grove();
        let legacy = mknode(&g, "done", "foreign");
        touch(&legacy, "09-impl--old-k9.md");
        touch(&g, "01-impl--a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-impl--a-k1.md");
    }

    #[test]
    fn pick_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = pick(&missing).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

````
<!-- /fragment -->

**`pick_ignores_a_legacy_done_directory` establishes that a foreign container
hides its subtree.** `done` is not a task-shaped name, so it is `Verdict::Foreign`,
and the library's own documentation of that verdict says skipping a foreign
*directory* skips everything beneath it — sound precisely because the consumer
said the name was not its own. The `09-impl--old-k9.md` inside it is a perfectly
well-formed leaf name and is nonetheless invisible to the walk. Set beside the two
refusals above, that is where the line sits: a name grove disclaims may hide
work, and grove accepts that; a name grove claims and finds at the wrong species
may not be skipped, because there the work being hidden is grove's own.

**What it would pass under with the property broken:** the stray leaf is at
ordinal 09 and the live one at ordinal 01, so an implementation that flattened
every directory — foreign or not — and sorted by ordinal answers
`01-impl--a-k1.md` too. This test shows the legacy subtree does not *win*; it does
not show it is unreachable. A fixture placing the stray leaf at a lower ordinal
would discriminate, and there is none.

**`pick_errors_when_grove_root_absent` establishes that an absent root is grove's
own refusal rather than an empty answer.** The path handed in exists nowhere, and
the test asserts on `unwrap_err`, so `Ok(None)` fails it. The wording is chapter
5's `restate`, which re-states a failed read in the order grove owes its
operator — the root first, then the library's own message.

**What it would pass under with the property broken:** any error whose rendered
text contains *grove root not found*, from anywhere in the call. What the test is
really evidence for is which of chapter 5's four openings the composition chose:
`read_or_vacant` treats an absent root as an *answer*, because `grove` asks it of
a fresh checkout on every iteration, while `read` refuses. The `pick` composition
calls `read`, and this test is where that choice becomes observable.

<a id="what-the-walk-kept"></a>
## What the walk kept

The loop owns validity, eligibility and selection over one snapshot. Whole-tree
key validation makes permanent identity unambiguous for the driver and viewer,
before any candidate is excluded. *Live* is a predicate over a grammar the store does not read, and
it is why `selected` filters rather than simply taking the first entry. *Finish
is outranked* is a rank the store has no word for, and it is why `selected` walks
the whole tree before answering. Between them they turn the library's ordering
into grove's schedule, and `selected` is the only place in the crate where that
happens. Eight production functions walk a snapshot — three in chapter 6, this
one, one each in chapters 8, 9 and 10, and one in chapter 11 — and `selected` is
the only one whose answer depends on the *order* the walk yields. The other seven
search a set: they match a path, collect twins, take a maximum, gather every
entry a slug matched, or ask whether any entry exists at all. Two say so of
themselves — chapter 9's resolver comments that it takes a whole walk and never a
short-circuiting `seek`, because ambiguity is a property of the match set, and
chapter 10's cross-reference lint sorts its own collection before using it.
Position in the walk is the schedule, and it is the schedule in exactly one
function.

Nineteen tests hold that in place, and the honest summary of them is that most
hold less alone than their names suggest. They work as a lattice: the ordering
pair closes *last leaf* and *creation order* between them, the descent pair and
the fall-through triple close each other's readings, and every negative is
checkable only against the positives beside it. The clearest exceptions are the
three that end in a refusal rather than an answer — the species mismatch, the
symlink and the absent root — because a refusal is an observation no other tree
in the block produces, so nothing else in the file could account for it; the
lexical-ordering test is a fourth, since no other fixture crosses a digit-count
boundary. Even the refusals pin the refusal's *text* rather than its origin,
which is the residue this chapter could not remove.

Chapter 8 takes the same snapshot and asks two more things of it — a leaf's
session kind, and its ancestors' briefs root to leaf — including the closing block
that exercises `pick` and `brief_chain` against one observation, which is the
composition this chapter's `pick_in` doc comment was written for.

Five of this file's ten ownership blocks remain, and chapters 8 and 9 resolve
them.

[Previous: Paths, and addressing](06-paths.md) | [Contents](README.md) | [Next: Kind, and the brief chain](08-kind-and-briefs.md)

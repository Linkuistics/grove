# A leaf becomes a node
<!-- book-page id="leaf-to-node" slice="the-key-survives" order="12" -->
[Previous: A grove begins](11-a-grove-begins.md) | [Contents](README.md) | [Next: Outcomes are marked in place](13-outcomes.md)

<a id="the-key-survives"></a>
## The rule: the key survives, because the entity does

Chapter 11 made a grove out of nothing and left one live leaf in it. This chapter
takes a leaf that has proved bigger than its brief assumed and turns it into a
node — and the whole of what makes that hard is a single word said twice.

**Say which tree.** In `ordinal-fs-tree`'s vocabulary a *leaf* is an entry that is
a file and a *node* is an entry that is a directory with children beneath it; the
promotion this chapter is about is a rename from the first shape to the second,
and the library has a perfectly good word for it. In grove's vocabulary a *leaf*
is one session's whole work and a *node* is a task that turned out to need
several. The two glossaries collide on exactly these two words
(`CONTEXT-MAP.md`), and this chapter's subject is the one operation where both
sentences are true of the same rename at the same instant. Every paragraph below
says which tree it means.

The rule the chapter carries is what survives that rename:

> **The key is preserved, because the entity that was the leaf becomes the node.**
> Not *a node is created and the leaf's contents are copied into it* — that would
> be two entities, and the second would need a key of its own. One entity changed
> shape, so it keeps the key it already had, and every byte that spells that key
> has to still be true afterwards.

That is what puts this chapter squarely on the second part of the
what-could-not-move test — *on the way through, the preconditions*. The store can
promote. What the store cannot do is decide **which** entry may be promoted, and
every clause of that decision is domain knowledge: whether the entry is a charter
brief, whether it already wears an outcome infix, whether its kind is the one the
driver reserved for itself. And once the promotion has landed, the store cannot
repair the one thing it moved without reading — the first line of the leaf's own
body, which is a `# <handle>` header that now titles a brief.

So the chapter has three parts and they are the three things that did not move:
the classification in front of the call, the checking of two predicted keys
behind it, and one edit to bytes the library carried verbatim and never looked
at.

The carried example reaches its twelfth step. The grove chapter 11 created and
chapter 10 grew now holds work that will not fit one session.

```text
<worktree>/.grove/
├── BRIEF.md
├── 01-requirements--plan-k1.md
└── 02-impl--build-k3.md              a LEAF (grove): one session's work
                                      a LEAF (ordinal-fs-tree): a file entry

leaf_decompose(guard, "02-impl--build-k3.md", "step", Some(impl))

<worktree>/.grove/
├── BRIEF.md
├── 01-requirements--plan-k1.md
└── 02-build-k3/                      a NODE (grove): a task that proved bigger
    │                                 a NODE (ordinal-fs-tree): a directory entry
    │                                 position 02 kept, key 3 KEPT
    ├── BRIEF.md                      the leaf's own bytes, RENAMED in
    │                                 "# build-k3"  ->  "# build-k3 — brief"
    └── 01-impl--step-k4.md           grown in the SAME operation, key 4 predicted

  ⇒ Ok(( <root>/02-build-k3/BRIEF.md,
         <root>/02-build-k3/01-impl--step-k4.md ))
```

Those are the names `decompose_converts_leaf_file_to_node_dir_preserving_the_key`
and `decompose_creates_the_first_child_at_01_with_a_fresh_key` assert on, and
`02-impl--build-k3.md` is the fixture ten of this chapter's twenty-two tests
build. The key does not change. The position does not change. What changes is the
species, and one line of one file's text.

<a id="one-promote-that-had-to-be-one"></a>
## One `promote`, and why it had to be one

The chapter's first ownership block is `tree_lifecycle.rs` lines 490 to 695 — 206
lines, the file's third production concern. It is three items: the verb, the
classification it runs first, and the check it runs last.

<!-- fragment «decompose-production» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="493-698" parent="source-tree-lifecycle" -->
<!-- insert «decompose-verb-contract» -->
<!-- insert «decompose-verb-body» -->
<!-- insert «decompose-decomposable» -->
<!-- insert «decompose-promoted-claims» -->
<!-- insert «decompose-promoted-body» -->
<!-- /fragment -->

The doc comment is the longest in the file and it is an argument rather than a
description, so this chapter's job over it is to connect its parts and not to
restate them.

<!-- fragment «decompose-verb-contract» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="493-534" parent="decompose-production" -->
````rust
/// `leaf-decompose <leaf-path> <first-child-slug>`: convert a live leaf file
/// `NN-<kind>--<slug>-k<key>.md` into a node directory `NN-<slug>-k<key>/` (**key
/// preserved**) holding a `BRIEF.md` (seeded from the leaf body, its `# <handle>`
/// header retitled `# <handle> — brief`) and a first child
/// `01-<kind>--<first-child-slug>-k<new>.md` grown atomically so the node is never
/// childless. Refuses a brief, a node, and an already-`DONE` leaf. Returns
/// `(brief_path, first_child_path)`. Working-tree only — no commit.
///
/// `kind_override` is `--kind`'s *override* of the first child's kind
/// (task-kind-taxonomy): `None` inherits the leaf being decomposed's own kind —
/// read strictly from the parent filename. `Some(k)` uses `k` regardless of the
/// parent's kind. Legacy body routing is not copied into either new task body.
///
/// # The whole verb is one `promote`, and it has to be
///
/// [`WriteGuard::promote`](ordinal_fs_tree::fs::WriteGuard::promote) creates the
/// node, moves the leaf's own file into it as the distinguished child, and
/// creates the first child — three effects in one unit, which is exactly what
/// *atomically growing a first child* has always promised and what Grove used to
/// assemble out of a `create_dir`, a rename and a whole second verb. The
/// difference is not tidiness. A promotion **breaks an invariant on the way
/// through**: the node carries the promoted leaf's own ordinal and its own key,
/// so between effect one and effect two both are on disk sharing an ordinal and
/// a key, and no ordering avoids it
/// (`docs/ordinal-fs-tree/ARCHITECTURE.md`, *Promotion is not atomic against the
/// invariants*). The invariants hold of **quiescent** trees, and the exclusive
/// guard is what makes that safe — so a Grove reader running without a guard
/// would see the intermediate state, and none does: every reader goes through
/// [`task_tree::read`] or [`task_tree::write`], and
/// `the_librarys_tree_lock_is_taken_from_exactly_one_module` holds that,
/// together with the fact that Grove's own guard `flock`s the same directory and
/// so excludes rather than nests.
///
/// # Two keys are predicted here, for two different reasons
///
/// The node's key is the leaf's own — *identity preservation* is the whole point
/// of a promotion — and the first child's is [`task_tree::next_key`], the
/// consumer-side mirror of the library's `max + 1` that a content-carrying domain
/// cannot do without (`growing-k33`; `docs/ARCHITECTURE.md#tree-access-lock`).
/// Both are checked against the report by [`promoted`], because both are embedded
/// in bytes: the child's in the template handle it is created with, and the
/// node's in the brief header this verb retitles afterwards.
````
<!-- /fragment -->

**Three effects, one unit.** The library's `promote` creates the node, moves the
leaf's own file into it as the distinguished child, and creates the first child.
Grove used to assemble that out of a `create_dir`, a rename and a whole second
verb, and the comment is explicit that the difference is not tidiness.

The reason is stated in a sentence worth reading twice: **a promotion breaks an
invariant on the way through.** The node carries the promoted leaf's own ordinal
and its own key — that is what identity preservation means — so between effect
one and effect two both the old leaf and the new node are on disk sharing an
ordinal and a key, and no ordering of the three effects avoids it. The comment
cites the library's own record for this (`docs/ordinal-fs-tree/ARCHITECTURE.md`,
*Promotion is not atomic against the invariants*), and then supplies the part
that is grove's: the invariants hold of **quiescent** trees, so the intermediate
state is only safe if nothing can read the tree while it exists.

That is a claim about every reader in the workspace, and it is held by a test
rather than by the sentence. `the_librarys_tree_lock_is_taken_from_exactly_one_module`
(`crates/grove-llm/tests/tree_lock.rs`) is what makes *every reader goes through
`task_tree::read` or `task_tree::write`* checkable, and the second half — that
grove's own guard `flock`s the same directory and so excludes rather than nests —
is chapter 5's. The chapter that owns the lock proves the exclusion; this chapter
is the one place in the crate that spends it, and the record says why in its own
words: a promotion is *the one path by which the library creates a duplicate key
in a tree it was handed* — everywhere else a duplicate key is something the
library inherits rather than causes.

**Two keys are predicted here, and the two headings above them are not the same
kind of thing.** The node's key is not predicted at all — it is the leaf's own,
because the entity is unchanged, and the prediction is that the library will
agree. The first child's key *is* a prediction: `task_tree::next_key` is the
consumer-side mirror of the library's `max + 1`, which chapter 6 read and chapter
10 spent. The comment's own reason for checking both is the load-bearing one:
**both are embedded in bytes** — the child's in the template handle it is created
with, and the node's in the brief header this verb retitles afterwards. A key that
disagreed would not be a wrong return value; it would be a file whose name
contradicts its own first line, permanently.

<!-- fragment «decompose-verb-body» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="535-593" parent="decompose-production" -->
````rust
pub(crate) fn leaf_decompose(
    tree: Guard,
    leaf_path: &Path,
    first_child_slug: &Slug,
    kind_override: Option<Kind>,
) -> Result<(PathBuf, PathBuf)> {
    // No check here: `Slug` is the precondition, discharged wherever one was
    // built. Text the grammar disclaims never becomes a `Slug`, so a bad child
    // slug leaves the leaf un-decomposed — and `grove-llm` spells it before it
    // opens the tree, so refusing one there costs no exclusive lock.
    let child_slug = first_child_slug;

    let root = tree.root().to_path_buf();
    let grove_root = root.clone();
    // The classification, then the operation: `promote` consumes the guard, so
    // every borrow of its snapshot has to end before the call.
    let (key, slug, kind, child_key) = {
        let entry = match task_tree::target(&root, tree.snapshot(), leaf_path)? {
            task_tree::Target::Root => bail!(
                "cannot decompose the grove root (lifecycle verbs act on leaves): {}",
                root.display()
            ),
            task_tree::Target::Entry(entry) => entry,
        };
        let (parent_kind, slug) = decomposable(&entry)?;
        // task-kind-taxonomy: the first child inherits the decomposed leaf's own
        // kind unless `--kind` overrides it.
        let kind = kind_override.unwrap_or(parent_kind);
        task_grow::refuse_finish_kind(&kind, "leaf-decompose")?;
        (
            task_tree::addressable_key(&root, tree.snapshot(), &entry)?,
            slug.clone(),
            kind,
            task_tree::next_key(tree.snapshot()),
        )
    };
    // The node's parts are Grove's to supply because the library cannot make
    // them — `Parts` is opaque, and nothing the library can reach describes
    // *this* entry as a node (`docs/adr/entry-name-is-the-only-seam.md`). The
    // same slug, so the species is the only thing that moved.
    let node_parts = Parts::node(slug.clone());
    let child = task_grow::new_leaf(child_key, Outcome::Live, kind, child_slug);
    let report = tree
        .promote(key, node_parts, TaskName::Brief, Some(child))
        .map_err(task_tree::raised)?;
    let (brief_path, child_path) = promoted(&report, key, child_key)?;

    // The retitling is Grove's own edit to bytes the library moved verbatim and
    // never read — it has no content model — so it cannot be part of the unit
    // above. It takes a guard of its own for the reason `leaf-insert`'s lint
    // does: the tree this touches is the one the promotion *left*, and no
    // cooperating command should meet a node brief mid-retitle.
    // `reopen_write`, not `write`: the wait this command made was announced by
    // the promotion (`docs/ARCHITECTURE.md#tree-access-lock`).
    let _guard = task_tree::reopen_write(&grove_root)?;
    append_brief_suffix_in_file(&brief_path, &Handle::new(slug.clone(), key))?;
    Ok((brief_path, child_path))
}

````
<!-- /fragment -->

**The body is a classification, an operation, and an edit — in that order, and
the order is what the comments are about.** The braces around lines 548 to 567 are
not style: `promote` *consumes* the guard, so every borrow of the guard's snapshot
has to end before the call, and the block is how the four values the operation
needs — the key, the slug, the kind and the predicted child key — get out of the
snapshot's lifetime alive.

<a id="the-precondition-that-moved-into-the-type"></a>
### A precondition that moved into the type, and the comment that says so

Lines 538 to 541 talk about a precondition and line 542 performs none, which is
the shape this section exists to adjudicate.

    // No check here: `Slug` is the precondition, discharged wherever one was
    // built. Text the grammar disclaims never becomes a `Slug`, so a bad child
    // slug leaves the leaf un-decomposed — and `grove-llm` spells it before it
    // opens the tree, so refusing one there costs no exclusive lock.
    let child_slug = first_child_slug;

**The line refuses nothing, and the comment opens by saying so.** It is a
rebinding of a parameter whose type is already `&Slug`, and a `Slug` cannot be
constructed from text the grammar disclaims — chapter 3's rule. There is no
check here to sit inside the guard or outside it.

**The claim is still true, and it is not this function that holds it.** A bad
child slug really does leave the leaf un-decomposed: the refusal happens at
construction, and construction is the caller's. What moved is who holds the
claim — from a check this function performed to a type its signature demands.

**The cost of that refusal is the caller's too, and it is not uniform.** Grove's
own CLI builds the child slug before it opens the tree for writing, so there a
bad spelling is refused before a lock exists to take. The crate's own tests are
the other way round: `leaf_decompose(guard(&g), &node, &a_slug("x"), …)` is
evaluated left to right, so the guard is taken first and the slug is built inside
it. Which is why the comment names `grove-llm` rather than claiming an ordering
for every caller — the signature takes an already-open `Guard`, so this verb
cannot enforce one, and the old comment's *refusing without taking an exclusive
lock* read as a property of the function when it was only ever a property of one
call site.

This file says the same thing 1,386 lines further down, in the doc comment of the
test that used to assert it: *the claim used to be about ordering: the slug was
validated before the rename, so a bad one left no half-built node directory.
Since `loop-crate-verbs-k21` the verb takes a [`Slug`], so the text is read by
the type that owns it and a bad slug never reaches a tree at all.*

**The two passages agree now, and they did not when this chapter was drafted.**
The production comment used to open *Grove's own precondition, before the tree is
even observed*, and to weigh putting the check inside the guard against leaving
it outside — a choice described in a function that performs no check either way.
It was the class chapter 11 met at `default_root_slug` and chapter 6 met at the
stale `llm_cli` address: a comment that outlived what it described, and one no
instrument reports, because `//` is invisible to `cargo doc`. Adjudicating it
here is what produced `stale-slug-precondition-comment-k162`, and the comment
above is what that leaf wrote: four lines replacing four, so the file stayed at
2,732 lines and no fragment range, ownership range or manifest count in this book
had to move.

<a id="every-clause-the-library-cannot-see"></a>
## Every clause is a precondition the library cannot see

`decomposable` is thirty-eight lines and it is the whole of clause 2 of
`docs/ARCHITECTURE.md#library-refusals` for this verb: *classify the resolved
entry before calling*.

<!-- fragment «decompose-decomposable» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="594-631" parent="decompose-production" -->
````rust
/// The decomposed leaf's own kind and slug, or Grove's refusal that this entry
/// is not a live leaf.
///
/// Every clause is a precondition the library cannot see — brief-ness, an
/// outcome infix, `finish`-reservation — which is clause 2 of
/// `docs/ARCHITECTURE.md#library-refusals` and the reason
/// [`Refusal::PromoteNotLeaf`](ordinal_fs_tree::Refusal) is unreachable: a node
/// falls out of the same match, before any key is handed to the library.
fn decomposable<'a>(entry: &Entry<'a, TaskName>) -> Result<(Kind, &'a Slug)> {
    let name = entry.name();
    let Some(triple) = entry.triple() else {
        bail!("cannot decompose a brief (it is already a node): {name}")
    };
    match triple.parts {
        Parts::Node { .. } => {
            bail!("cannot decompose a node (it already has children): {name}")
        }
        Parts::Leaf {
            outcome: Outcome::Done,
            ..
        } => bail!("cannot decompose a retired (DONE) leaf: {name}"),
        Parts::Leaf {
            outcome: Outcome::Abandoned,
            ..
        } => bail!("cannot decompose an abandoned (ABANDONED) leaf: {name}"),
        Parts::Leaf {
            outcome: Outcome::Live,
            kind,
            slug,
        } => {
            if kind.is_finish() {
                bail!("`finish` is driver-reserved and cannot be decomposed");
            }
            Ok((kind.clone(), slug))
        }
    }
}

````
<!-- /fragment -->

**Five refusal arms, four conditions, and none of the four is a fact about the
filesystem.** The `else` on line 601 refuses a charter brief; the `Parts::Node`
arm refuses a node; the two outcome arms refuse a retired and an abandoned leaf;
and the `is_finish` guard inside the live arm refuses the kind the driver reserved
for itself. Brief-ness, an outcome infix and `finish`-reservation are the three
the doc comment names, and node-ness is the fourth, named in its next clause for a
different reason.

That different reason is the function's most consequential distinction.
`Refusal::PromoteNotLeaf`
is a refusal the library owns and can raise — *this key names something that is
not a leaf* — and grove never sees it, because **a node falls out of the same
match**, before any key is handed to the library at all. The classification grove
needed anyway is what makes the library's own species refusal unreachable. That is
clause 2's second sentence read as a consequence rather than as an instruction,
and *The seam* below is where it stops being a sentence and becomes an assertion.

**The predicate is read off the snapshot and not off the filesystem**, which is
clause 2's own requirement: a node here is an entry whose `triple().parts` is
`Parts::Node`, never a path that `is_dir`. Chapter 6 met the same rule in
`interrupted_promotion`, which asks whether an entry's `contents()` is `Some`.
Two predicates for one condition would let grove refuse where the library would
have proceeded, and this function has one.

**The return is the *parent's* kind, not the child's.** `decomposable` answers
`(Kind, &Slug)` — the decomposed leaf's own kind and its own slug — and the caller
then applies `kind_override.unwrap_or(parent_kind)`. The slug is what the node is
renamed with, so the species is the only thing that moved; the kind is what the
first child inherits when `--kind` is absent, which is `task-kind-taxonomy`'s
rule and is asserted by `decompose_with_no_override_inherits_the_parent_leafs_own_kind`
below.

<a id="three-claims-about-bytes"></a>
## Three claims, and every one of them about bytes

`promoted` runs after the operation has already landed. Nothing it finds can be
recovered from, and its doc comment says so in as many words.

<!-- fragment «decompose-promoted-claims» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="632-647" parent="decompose-production" -->
````rust
/// What a promotion left behind: the node's brief, and the first child — each
/// checked against what Grove promised itself.
///
/// **Three claims, and every one of them is about bytes rather than tidiness.**
/// The node kept the promoted leaf's key, which is what identity preservation
/// *is* and what keeps the brief's own `# <slug>-k<key>` handle true of the
/// entry it now names. The child got the key Grove predicted, without which its
/// template handle would contradict its filename permanently (`growing-k33`).
/// And the leaf's file was *renamed* rather than copied, which is the only
/// reason its bytes are still there to retitle.
///
/// A disagreement is a broken contract to report and not a case to recover from:
/// the operation has already landed when this runs. The ordinal needs no clause
/// of its own — the node's name is composed from the promoted leaf's triple, so
/// a preserved key and a preserved ordinal are one fact, and the on-disk
/// assertion lives in `decompose_converts_leaf_file_to_node_dir_preserving_the_key`.
````
<!-- /fragment -->

**The three claims are the node's key, the child's key, and the rename.** Each is
about bytes rather than about tidiness, and the comment gives the byte for each
one: the node's key keeps the brief's own `# <slug>-k<key>` header true of the
entry it now names; the child's key keeps its template handle from contradicting
its filename permanently; and the leaf's file being *renamed* rather than copied
is the only reason its bytes are still there to retitle.

**The ordinal needs no clause and the comment explains why**, which is worth
checking rather than accepting: the node's name is composed from the promoted
leaf's triple, so a preserved key and a preserved ordinal are one fact rather than
two. The on-disk half is asserted by
`decompose_converts_leaf_file_to_node_dir_preserving_the_key`, which reads
`02-build-k3` off the parent directory's name — a string carrying the ordinal and
the key at once, so one assertion pins both.

<!-- fragment «decompose-promoted-body» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="648-698" parent="decompose-production" -->
````rust
fn promoted(
    report: &Report<TaskName>,
    key: Key,
    predicted_child: Option<Key>,
) -> Result<(PathBuf, PathBuf)> {
    let created = report.created();
    let [node, child] = created else {
        bail!(
            "the library created {} entries for a promotion with a first child, \
             where 2 were asked for",
            created.len()
        )
    };
    let TaskName::Positioned { key: node_key, .. } = &node.name else {
        bail!(
            "the library promoted a leaf into the charter brief, which carries no \
             key: {}",
            node.path.display()
        )
    };
    if *node_key != key {
        bail!(
            "the library gave the promoted node key {} where the leaf carried {}: \
             the handle in {} contradicts its filename and must be corrected by hand",
            node_key.get(),
            key.get(),
            node.path.display()
        );
    }
    let TaskName::Positioned { key: child_key, .. } = &child.name else {
        bail!(
            "the library created a charter brief as a first child: {}",
            child.path.display()
        )
    };
    if Some(*child_key) != predicted_child {
        bail!(
            "the library allocated key {} where Grove's template wrote {}: the \
             handle in {} contradicts its filename and must be corrected by hand",
            child_key.get(),
            predicted_child.map_or("no key".to_string(), |key| key.get().to_string()),
            child.path.display()
        );
    }
    let brief = report
        .renamed()
        .first()
        .context("the library reported no rename for a promotion")?;
    Ok((brief.to.clone(), child.path.clone()))
}

````
<!-- /fragment -->

**Six refusal arms for three claims, and the extra three are shape checks that
make the three claims askable.** The `let [node, child] = created else` destructure
refuses a report that did not create exactly two entries; the two
`TaskName::Positioned` `let-else` arms refuse a created entry that is a charter
brief, which carries no key and so cannot be compared. Only then can the two key
comparisons run. The sixth is the `.context` on `renamed().first()`, which is the
rename claim itself.

**The messages are recovery instructions, not diagnoses.** Both key mismatches end
in *the handle in `<path>` contradicts its filename and must be corrected by
hand*, which is the only honest thing to say: the promotion has landed, the guard
is gone, and the bytes on disk are already wrong. This is the class chapter 11
named at `initialize_grove` — refusals that report the library breaking its own
contract — and *What the refusals are worth, measured* below shows that all six of
them are held by nothing at all, for the same reason.

One detail in the child-key message repays attention: `predicted_child` is an
`Option<Key>`, and line 685 renders `None` as the string `"no key"` rather than
letting a formatter choose. A promotion with no first child would predict nothing,
so the message has to be sayable in a case this verb never produces — `leaf_decompose`
always passes `Some(child)`.

<a id="the-tests-and-what-each-would-pass-under"></a>
## What the sixteen tests establish, and what each would pass under

The chapter's second ownership block is lines 1666 to 2234 — 569 lines, and the
largest single inline-test block in the book: larger than chapter 17's
`driver_lease.rs` tests at 564 and chapter 13's at 491, and larger than any of the
eleven in `task_name.rs` and `task_tree.rs`. It carries **twenty-two** `#[test]`
functions in two labelled sections — sixteen in *leaf-decompose* and six in
*leaf-decompose: the seam* — and one helper that is not a test.

The block's prose obligation is *supply the claim*: for each reproduced test, the
property it establishes **and what would have to be true for it to pass while the
property was broken**. The second half is the part a reviewer can check and the
test cannot state.

<!-- fragment «decompose-tests» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1669-2241" parent="source-tree-lifecycle" -->
<!-- insert «decompose-tests-opening» -->
<!-- insert «decompose-tests-brief-and-child» -->
<!-- insert «decompose-tests-kind» -->
<!-- insert «decompose-tests-nested» -->
<!-- insert «decompose-tests-refusals» -->
<!-- insert «decompose-tests-slug-and-path» -->
<!-- insert «decompose-tests-seam-opening» -->
<!-- insert «decompose-tests-one-guard» -->
<!-- insert «decompose-tests-twin» -->
<!-- insert «decompose-tests-destination» -->
<!-- insert «decompose-tests-interrupted» -->
<!-- insert «decompose-tests-last-key» -->
<!-- insert «decompose-tests-sweep» -->
<!-- /fragment -->

**These tests depend on chapter 11's support block**, at lines 1077 to 1261, and
this is the block that collects on it. Chapter 11 counted five of its own eighteen
items called by its own tests; **this block calls twelve of the eighteen, and nine
of those twelve are ones chapter 11's tests never touched** — `a_kind`, `jj_grove`,
`commit_all`, `a_slug`, `guard`, `refusal_for_an_absent_root`, `touch_body`,
`mknode` and `list`. Only `worktree`, `root_init_at`, `grow_leaf` and `guard_at`
are named nowhere in these 569 lines; `run_jj` and `open` are reached, but only
from inside the support block itself, which is where chapter 11 put them.

Three of the nine are the ones this section reads first. `mknode` builds a node
directory with a `BRIEF.md` in it, `touch_body` writes a leaf with a body of the
caller's choosing rather than a one-line stub, and `list` returns the grove root's
entries sorted so an assertion does not depend on directory order. Chapter 11
reproduces and explains all three; this chapter names them and reads them at work.
And the reason this section needs `jj_grove` where chapter 11's tests mostly needed
a bare directory is the whole difference between the two chapters: this verb
renames entries.

<!-- fragment «decompose-tests-opening» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1669-1698" parent="decompose-tests" -->
````rust
    // ---- leaf-decompose -----------------------------------------------------

    #[test]
    fn decompose_converts_leaf_file_to_node_dir_preserving_the_key() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let (brief, _child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();
        // The entity that was leaf k3 becomes node k3 — a directory holding BRIEF.md.
        assert_eq!(name_of(&brief), "BRIEF.md");
        assert_eq!(name_of(brief.parent().unwrap()), "02-build-k3");
        let files = list(&g);
        assert!(
            files.contains(&"02-build-k3".to_string()),
            "node dir present"
        );
        assert!(
            !files.contains(&"02-impl--build-k3.md".to_string()),
            "old leaf file gone"
        );
        assert!(g.join("02-build-k3").is_dir());
    }

````
<!-- /fragment -->

**`decompose_converts_leaf_file_to_node_dir_preserving_the_key`** — the chapter's
rule, on disk. The leaf file `02-impl--build-k3.md` is gone, the directory
`02-build-k3` is there, it holds a `BRIEF.md`, and the directory's name carries
the same position and the same key the leaf had. The three assertions are chosen
so that no two of them can be satisfied by the same accident: `name_of(&brief)`
pins the returned path's basename, `name_of(brief.parent())` pins the node's
name, and the two `files.contains` assertions pin *both* directions of the rename
— the new entry present and the old one absent.

It would pass while the property was broken if the library **copied** rather than
renamed: the old file's absence is asserted from `list`, which reads the
directory, so a copy-then-delete would look identical here. What that would break
is the brief's *bytes*, and it is the next test that holds them. And nothing here
asserts that the key was not merely reallocated to the same value; `02-build-k3`
is one string, so *key 3 preserved* and *key 3 freshly allocated on a tree whose
maximum was 2* are the same observation. The prediction that separates them is
`promoted`'s, in production, and it is held by nothing (below).

<!-- fragment «decompose-tests-brief-and-child» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1699-1745" parent="decompose-tests" -->
````rust
    #[test]
    fn decompose_seeds_brief_from_leaf_body_and_appends_brief_suffix() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-impl--build-k3.md",
            "# build-k3\n\n## Goal\nship it\n",
        );
        commit_all(&g);
        let (brief, _child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();
        let text = body(&brief);
        assert_eq!(
            text.lines().next().unwrap(),
            "# build-k3 — brief",
            "handle retitled with the brief suffix"
        );
        assert!(
            text.contains("## Goal\nship it"),
            "leaf body carried in: {text:?}"
        );
    }

    #[test]
    fn decompose_creates_the_first_child_at_01_with_a_fresh_key() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-impl--step-k4.md");
        assert_eq!(name_of(child.parent().unwrap()), "02-build-k3");
        assert!(g.join("02-build-k3").join("01-impl--step-k4.md").is_file());
    }

````
<!-- /fragment -->

**`decompose_seeds_brief_from_leaf_body_and_appends_brief_suffix`** — the two
halves of what happens to the leaf's own bytes. The first line becomes
`# build-k3 — brief`, and the rest of the body survives. The `touch_body` fixture
is what makes this checkable: the leaf is written with a real body —
`"# build-k3\n\n## Goal\nship it\n"` — so the `contains("## Goal\nship it")`
assertion has something to find that the retitle could have destroyed.

It would pass while the property was broken if the retitle were applied to a
*copy* whose original was deleted, since the assertions are on the brief's
content and not on its identity. It would also pass if
`append_brief_suffix_in_file` were not idempotent: this fixture's header has no
suffix on it, so the *already suffixed* branch of that helper is never taken.
**That branch is chapter 11's source and is held by nothing**, established by
mutation there — panicking on its conservative `return Ok(())` reddens nothing
across all 560 — and this chapter is where the helper is consumed, so the gap is
worth stating here rather than only where the code lives. Its doc comment promises
two things, idempotence and *never clobbers a custom title*, and both promises run
through the branch nothing reaches.

**`decompose_creates_the_first_child_at_01_with_a_fresh_key`** — the node is never
childless, and the child is `01-impl--step-k4.md`: position 01 because it is the
first, kind `impl` because that is what `--kind` said, slug `step` because that is
what the caller passed, key 4 because the tree's maximum was 3. The
`.is_file()` assertion at the end is what makes it a statement about the
filesystem rather than about the returned path.

It would pass while the property was broken if the child were created in a
*second* operation after the promotion: nothing here observes atomicity, only the
end state. The lock count that does observe it is
`decompose_takes_one_guard_for_the_promotion_and_one_for_the_retitle`, in the seam
section, and even that counts guards rather than operations.

<!-- fragment «decompose-tests-kind» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1746-1825" parent="decompose-tests" -->
````rust
    #[test]
    fn decompose_first_child_header_is_the_handle_and_filename_carries_the_kind() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();
        let text = body(&child);
        assert!(text.starts_with("# step-k4\n"), "got {text:?}");
        assert_eq!(name_of(&child), "01-impl--step-k4.md");
        assert!(!text.contains("**Kind:**"), "got {text:?}");
    }

    #[test]
    fn decompose_first_child_can_be_a_planning_task() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("design"),
            Some(a_kind("planning")),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-planning--design-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

    #[test]
    fn decompose_with_no_override_inherits_the_parent_leafs_own_kind() {
        // task-kind-taxonomy: `leaf-decompose` gives the first child the leaf
        // being decomposed's own kind when `--kind` is not given.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-research-a--build-k3.md",
            "# build-k3\n\n**Kind:** impl\n",
        );
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-research-a--build-k3.md"),
            &a_slug("step"),
            None,
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-research-a--step-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

    #[test]
    fn decompose_override_wins_over_the_parent_leafs_kind() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(
            &g,
            "02-research-a--build-k3.md",
            "# build-k3\n\n**Kind:** impl\n",
        );
        commit_all(&g);
        let (_brief, child) = leaf_decompose(
            guard(&g),
            Path::new("02-research-a--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("review-impl")),
        )
        .unwrap();
        assert_eq!(name_of(&child), "01-review-impl--step-k4.md");
        assert!(!body(&child).contains("**Kind:**"));
    }

````
<!-- /fragment -->

**`decompose_first_child_header_is_the_handle_and_filename_carries_the_kind`** —
three assertions and the third is the real one. The body opens `# step-k4`, which
is the position-free handle chapter 3 argued for; the filename carries the kind;
and the body does **not** carry a `**Kind:**` line. Without that negative
assertion the test would pass with the kind in both places, and a template that
reintroduced a body field would be invisible — the same shape as chapter 11's
`root_init_first_leaf_kind_lives_in_its_filename`, and for the same reason: the
kind has to be readable from the name alone, because that is what the walk reads.

It would pass while the property was broken if a longer header happened to begin
`# step-k4` — which the trailing `\n` in the `starts_with` pattern rules out — or
if the body carried the kind under a different spelling than `**Kind:**`. The
negative assertion is a check against one exact string, so it holds the legacy
form and nothing else.

**`decompose_first_child_can_be_a_planning_task`** and
**`decompose_override_wins_over_the_parent_leafs_kind`** both override, and
neither is redundant, because they override against different backgrounds. The
first supplies `planning` over an `impl` parent with the slug `design`, which is
the only fixture in the block that varies both the kind and the slug from the
section's default pair — a check that `--kind` is a token and not a menu. The
second makes the override a **three-way** discrimination: the filename says
`research-a`, the body carries a legacy `**Kind:** impl` line, the flag says
`review-impl`, and the child is `01-review-impl--step-k4.md`. Only the flag can
have produced that name.

**`decompose_with_no_override_inherits_the_parent_leafs_own_kind`** — the
`task-kind-taxonomy` rule the production comment cites, asserted where it is
visible. With `None` for `kind_override`, a `research-a` leaf's first child is
`01-research-a--step-k4.md`.

The fixture is doing more than it looks. The leaf's body carries `**Kind:** impl`
— a legacy body field naming a *different* kind from the filename — and the
child's name is `research-a`, the **filename's**. So the test is not only *the
kind is inherited*; it is *the kind is inherited from the filename and the body is
not read*, which is what the doc comment means by **read strictly from the parent
filename**. The trailing `assert!(!body(&child).contains("**Kind:**"))` closes the
other direction: the legacy field is not copied forward either. **It would pass
while the property was broken** only if `research-a` were somehow derivable from
the body, which this fixture rules out by making the two disagree — a fixture
built to falsify rather than to satisfy, which is the shape chapter 11 named in
its own support block.

<!-- fragment «decompose-tests-nested» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1826-1852" parent="decompose-tests" -->
````rust
    #[test]
    fn decompose_a_nested_leaf_preserves_key_and_grows_a_grandchild() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let build = mknode(&g, "02-build-k1", "build-k1");
        touch(&build, "02-impl--mid-k5.md", "mid-k5");
        commit_all(&g);
        let (brief, child) = leaf_decompose(
            guard(&g),
            &build.join("02-impl--mid-k5.md"),
            &a_slug("first"),
            Some(a_kind("impl")),
        )
        .unwrap();
        assert_eq!(
            name_of(brief.parent().unwrap()),
            "02-mid-k5",
            "key 5 preserved"
        );
        assert_eq!(
            name_of(&child),
            "01-impl--first-k6.md",
            "fresh key max(1,5)+1 = 6"
        );
        assert_eq!(name_of(child.parent().unwrap()), "02-mid-k5");
    }

````
<!-- /fragment -->

**`decompose_a_nested_leaf_preserves_key_and_grows_a_grandchild`** — the rule
again, one level down, and the only test in the block whose fixture is not a flat
grove. A node `02-build-k1` holds a leaf `02-impl--mid-k5.md`; decomposing it
gives `02-mid-k5` holding `01-impl--first-k6.md`. Both assertions carry their
arithmetic in the failure message — *key 5 preserved*, *fresh key max(1,5)+1 = 6*
— which is what makes them readable as a claim about `next_key` rather than as two
literals.

**This is the one test in the block whose fixture can tell `next_key`'s walk from
a shallower one.** `next_key` takes the maximum key over `snapshot.walk()` — the
whole tree — and every other fixture here is flat, with one keyed entry, so
*max + 1* has nothing to be distinguished from. This one puts the node at `k1` and
the leaf inside it at `k5`, so a `next_key` that read only the grove root's own
level and never descended into node directories would answer **2**, and
`01-impl--first-k6.md` would fail. It would still pass while the property was
broken under two other rules the fixture cannot separate: *the decomposed leaf's
own key plus one*, and *the maximum inside the leaf's own container plus one* both
answer 6 here as well.

<!-- fragment «decompose-tests-refusals» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1853-1926" parent="decompose-tests" -->
````rust
    #[test]
    fn decompose_refuses_a_brief() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_decompose(
            guard(&g),
            &node.join("BRIEF.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_node_directory() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_decompose(guard(&g), &node, &a_slug("x"), Some(a_kind("impl"))).unwrap_err();
        assert!(err.to_string().contains("node"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_done_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-DONE-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let err = leaf_decompose(
            guard(&g),
            Path::new("02-DONE-impl--build-k3.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();
        assert!(
            err.to_string().to_lowercase().contains("done") || err.to_string().contains("retired"),
            "got {err}"
        );
    }

    #[test]
    fn decompose_refuses_an_abandoned_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-ABANDONED-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let err = leaf_decompose(
            guard(&g),
            Path::new("02-ABANDONED-impl--build-k3.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();
        assert!(err.to_string().contains("abandoned"), "got {err}");
    }

    #[test]
    fn decompose_refuses_a_foreign_file() {
        let (_t, g) = jj_grove();
        touch(&g, "README.md", "readme");
        commit_all(&g);
        let err = leaf_decompose(
            guard(&g),
            Path::new("README.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

````
<!-- /fragment -->

**Five refusal tests, and this is where chapter 8's warning has to be taken
seriously.** That chapter found three of its four refusal tests refusing somewhere
other than their names said, so a coverage sentence about these five is worth
exactly a mutation. The procedure is the node's: read the `bail!` texts, match each
assertion's substring to the arms it could match, then replace each arm with a
panic in a workspace copy and diff against an unmutated control run of the same
copy.

**The result here is the opposite of chapter 8's, and it is worth saying plainly:
each of the four refusals the structure brief names fires exactly where its name
says.** `decompose_refuses_a_brief` reddens only when line 602's arm is replaced;
`decompose_refuses_a_node_directory` only line 606's; `decompose_refuses_a_done_leaf`
only line 611's; `decompose_refuses_an_abandoned_leaf` only line 615's. The full
attribution is in *What the refusals are worth, measured* below.

**But three of the five assertions do not distinguish the arm they landed on**,
and that is what each would pass under.

**`decompose_refuses_a_brief`** asserts `contains("brief")`, and among the five
arms that is unambiguous: line 602's *cannot decompose a brief (it is already a
node)* is the only message carrying the word. It is tighter than it looks against
the refusals *before* the arm, too, and by an accident worth naming. The fixture
passes `node.join("BRIEF.md")`, so a refusal from `task_tree::target` would name
that path — and `contains` is case-sensitive, so `BRIEF.md` does not satisfy a
check for `brief`. **The lower case is what makes the assertion mean the arm.**

**`decompose_refuses_a_node_directory`** asserts `contains("node")`, and three
different messages satisfy it. **Line 602's brief message contains `node` too**, in
the parenthetical *(it is already a node)*; so does chapter 6's *not a Grove leaf
or node directory*. The mutation shows the test lands on 606 today, and nothing in
it would notice if a node started falling out of the `entry.triple()` `else`
instead. The two arms are one `Option` apart: the `else` fires when `triple()` is
`None`, and a node's is `Some` only because a node directory is positioned.

**`decompose_refuses_a_done_leaf`** asserts
`to_lowercase().contains("done") || contains("retired")`. The first disjunct is
satisfied by the **abandoned** arm as well: lowercase `abandoned` contains the
substring `done`, at index 4. So a `DONE` leaf routed to line 615 would leave this
test green. The second disjunct is what actually distinguishes them — `retired`
appears only in line 611's message — and it is an `||`, so the test passes on
either. **The test is one character-class accident away from asserting nothing
about which arm fired**, and the substring that saves it is the one a reader is
least likely to think is load-bearing.

**`decompose_refuses_an_abandoned_leaf`** asserts `contains("abandoned")`,
case-sensitively. Line 611's message carries `DONE` in capitals and `retired` in
lower case, so it cannot satisfy this one. This is the only refusal assertion of
the four that is unambiguous by construction rather than by accident.

**`decompose_refuses_a_foreign_file` never enters `decomposable` at all.** Its
fixture is a `README.md` in the grove root, and the mutation attributes it to
neither of the five arms: it reddens only when `task_tree::target`'s result is
made to panic. `README.md` is `Verdict::Foreign`, so the store's walk disclaims it,
no entry of the snapshot has that name, and `target` answers *not a Grove leaf or
node directory* — **chapter 6's sentence, raised by chapter 6's function** at line
549, seven lines before `decomposable` is called at 556. The assertion
`contains("leaf")` is satisfied by that message and by lines 611's and 615's, so
the test's name is the only thing in it that says which function refused. The
honest form is *this is chapter 6's resolution observed through chapter 12's
verb*.

<!-- fragment «decompose-tests-slug-and-path» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1927-1973" parent="decompose-tests" -->
````rust
    /// A bad child slug leaves the leaf un-decomposed — and now it cannot even
    /// be spelled.
    ///
    /// The claim used to be about ordering: the slug was validated before the
    /// rename, so a bad one left no half-built node directory. Since
    /// `loop-crate-verbs-k21` the verb takes a [`Slug`], so the text is read by
    /// the type that owns it and a bad slug never reaches a tree at all —
    /// ordering it correctly is no longer something this verb can get wrong.
    #[test]
    fn decompose_cannot_be_reached_with_a_bad_child_slug() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);

        assert!(Slug::new("Bad Slug").is_err());

        let files = list(&g);
        assert!(
            files.contains(&"02-impl--build-k3.md".to_string()),
            "leaf untouched"
        );
        assert!(
            !files.contains(&"02-build-k3".to_string()),
            "no half-built node dir"
        );
    }

    #[test]
    fn decompose_accepts_an_absolute_path() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        let abs = g.join("02-impl--build-k3.md");
        let (brief, _child) =
            leaf_decompose(guard(&g), &abs, &a_slug("step"), Some(a_kind("impl"))).unwrap();
        assert_eq!(name_of(brief.parent().unwrap()), "02-build-k3");
    }

    #[test]
    fn an_absent_grove_root_is_refused_by_the_opening() {
        let (_t, g) = jj_grove();
        let err = refusal_for_an_absent_root(&g.join("nope"));
        assert!(err.contains("grove root not found"), "got {err}");
    }

````
<!-- /fragment -->

**`decompose_cannot_be_reached_with_a_bad_child_slug`** is the test whose doc
comment recorded the move into the type before the production comment above
caught up with it, and it is worth reading as a record of a claim that changed
shape. It no longer calls `leaf_decompose` at
all. It asserts `Slug::new("Bad Slug").is_err()` and then that the tree is
untouched — which it must be, since nothing was called.

**What it establishes is that the failure is unreachable, not that it is
handled.** The two `files.contains` assertions cannot fail while the first
assertion holds, because no code between them touches the grove; they are a record
of what the test used to prove rather than a live check. It would pass while the
property was broken in the only way that matters: if some *other* path into
`leaf_decompose` accepted a `&str` and built the `Slug` inside, this test would
never see it. Chapter 3's `Slug` and the verb's signature are what actually hold
the claim, and the mutation study cannot even reach this test, because there is no
arm to mutate.

One detail of the refusal is worth stating because chapters 13 and 14 reproduce
tests that assert on refusal substrings: **`"Bad Slug"` is refused for its capital
`B`, not for its space.** `refuse_token`'s five arms run in order —
emptiness, the reserved set `BRIEF` / `DONE` / `ABANDONED`, a leading or trailing
dash, the `--` separator, then the character class — and only the last of them can
fire here. That arm reports the *first* offending character, which it finds with
`.chars().find(...)`, so it names `'B'` and the space is never
reached. A fixture chosen to break one
rule is not evidence about the rule it was chosen for.

**`decompose_accepts_an_absolute_path`** — the verb takes the caller's spelling,
absolute or relative to the grove root, and both name the same entry. The whole of
the claim is in `&abs`, and the assertion is the same node-name check the section's
first test makes. It would pass while the property was broken if `target`
canonicalised the *returned* paths as well as the compared ones — which chapter 6
established it deliberately does not — since the assertion reads only the
basename.

**`an_absent_grove_root_is_refused_by_the_opening`** says in its own name what
the mutation confirms: it never reaches this chapter's code.
`refusal_for_an_absent_root` is chapter 11's helper and it opens a root that does
not exist, so the refusal is
chapter 5's *grove root not found*. It sits at the end of the `leaf-decompose`
section because that is the section whose fixtures it was written beside, and the
line number is what assigns it to this chapter — not the verb it does not call.

<a id="the-seam"></a>
## The seam: six tests about what cannot happen

The block's second labelled section is 264 lines for six tests, and it is the
densest argument in the chapter. Its subject is not what the verb does but what
the *library* cannot be made to say through it.

<!-- fragment «decompose-tests-seam-opening» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1974-1981" parent="decompose-tests" -->
````rust
    // ---- leaf-decompose: the seam --------------------------------------------
    //
    // `promote` is the operation with the most that can go wrong, and the three
    // refusals it owns are all discharged *upstream* by Grove's own
    // classification. That makes reachability the question worth asserting, and
    // the node brief's own table the thing to check rather than transcribe
    // (`docs/ARCHITECTURE.md#library-refusals`).

````
<!-- /fragment -->

The section comment states the thesis: `promote` owns three refusals, all three
are discharged upstream by grove's own classification, so **reachability is the
question worth asserting**, and the table of which refusals each verb can reach is
the thing to *check* rather than to transcribe. The comment names that table by
the record that first carried it and gives its path in the same breath:
`docs/ARCHITECTURE.md#library-refusals`, whose `leaf-decompose` row reads
*`KeysExhausted` alone, from the first child*. Chapter 6 met the same
name-plus-path habit in `addressable_key` and settled it the same way — check the
artifact the parenthetical points at, not the name the prose gives it.

***Owns* is doing real work in the first sentence**, and it is worth enumerating
because the number three is not the number of ways a promotion can be refused.
The library's promotion decision refuses six ways, and three of the six —
`TargetMissing`, `ContentForANode` and `KeysExhausted` — it shares with operations
that are not promotions, each of those having a second construction site
elsewhere. `PromoteNotLeaf` and `PromotePartsNotNode` are constructed nowhere else
in the library at all. Those, with `SuppliedNameNotDistinguished`, are the three the sweep
checks: the ones that exist because a promotion is a promotion. A seventh,
`DestinationOccupied`, is raised by the planner beneath the decision rather than by
the decision itself, which is why it takes a test of its own below.

This is written in `//` rather than `///`, which matters for a reason chapter 10
established: `cargo doc` sees only `///` and `//!`, so a section comment like this
one is checked by reading it and by nothing else. It carries no headings, so the
particular defect that instrument's blind spot hides — a heading whose body
renders under the next one — cannot arise here.

<!-- fragment «decompose-tests-one-guard» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1982-2010" parent="decompose-tests" -->
````rust
    #[test]
    fn decompose_takes_one_guard_for_the_promotion_and_one_for_the_retitle() {
        // Two observations, deliberately. `promote` consumes its guard, and the
        // ` — brief` retitle is Grove's own edit to bytes the library moved
        // verbatim and never read — so it cannot ride inside the unit, and it
        // takes a guard of its own rather than running on an unheld tree.
        // Asserted as a number so a later change moves it rather than quietly
        // contradicting the paragraph, exactly as `leaf-insert`'s lint is.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        commit_all(&g);
        crate::task_tree::reset_read_count();

        leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap();

        assert_eq!(
            crate::task_tree::read_count(),
            2,
            "one guard for the promotion, one for the retitle it cannot contain"
        );
    }

````
<!-- /fragment -->

**`decompose_takes_one_guard_for_the_promotion_and_one_for_the_retitle`** — the
count is two, and the comment says why a number rather than a description: a later
change moves the number rather than quietly contradicting the paragraph, *exactly
as `leaf-insert`'s lint is*. That comparison points at
`leaf_insert_lints_cross_references_under_a_shared_opening_of_its_own`, which
asserts the same number for the same reason — *the insert, then the lint's own
opening over the tree it left* — and which lives in
`crates/grove-loop/src/task_grow/tests.rs`, the one file this book excludes. So
chapter 10 could name the shape and not show it; this is the page where it is
visible.

**Count the opens before believing the label.** `READ_COUNT` has two increment
sites and neither is in this file: one in `task_tree::read_or_vacant`, under the
*shared* lock, and one in `task_tree::open_write`, which is the single exclusive
acquisition that both `write` and `reopen_write` route through. So the counter
counts **every observation of the tree**, shared and exclusive alike; the two it
records here both happen to be exclusive. The
reset happens on line 1991, and then the call on line 1993 evaluates its own
arguments — including `guard(&g)`, which is chapter 11's helper and which opens
the tree. So the two openings are **the test's own argument** and the verb's
`reopen_write` on line 586. `promote` opens nothing; it consumes the guard it was
handed.

The assertion's failure message — *one guard for the promotion, one for the
retitle it cannot contain* — is a true accounting of the guards the operation
runs under, and it is not an accounting of what this function opens. **It would
pass while the property was broken** if `leaf_decompose` took no guard of its own
for the retitle and some caller happened to open one instead, since the counter is
thread-local and blind to who called `open_write`. What it genuinely rules out is
the retitle riding inside the promotion's guard, which is impossible for a reason
the type system already gives: `promote` consumes it.

<!-- fragment «decompose-tests-twin» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2011-2041" parent="decompose-tests" -->
````rust
    #[test]
    fn decomposing_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed() {
        // `marking-k32`'s finding, at the verb that most needs it: `promote` is
        // called **by key**, and `by_key` answers with whichever entry the walk
        // reaches first on a duplicate-key tree. Decomposing the live leaf could
        // otherwise promote its `DONE` twin.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl--a-k1.md", "a-k1");
        touch(&g, "01-DONE-impl--a-k1.md", "a-k1");
        commit_all(&g);

        let err = leaf_decompose(
            guard(&g),
            Path::new("01-impl--a-k1.md"),
            &a_slug("x"),
            Some(a_kind("impl")),
        )
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 1"),
            "got {err}"
        );
        assert!(
            g.join("01-impl--a-k1.md").is_file() && !g.join("01-a-k1").exists(),
            "a refused promotion creates nothing"
        );
    }

````
<!-- /fragment -->

**`decomposing_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed`** —
`promote` is called **by key**, and `Snapshot::by_key` on a duplicate-key tree
answers with whichever entry the walk reaches first. So decomposing the live leaf
could promote its `DONE` twin, and the test asserts the refusal instead. Both
assertions matter: the message names key 1, and the tree is unchanged.

The refusal is `task_tree::addressable_key`'s, which is chapter 6's, and the
mutation confirms it: replacing that call's result reddens this test and two of
its neighbours and nothing else. **The honest form is that this is chapter 6's
precondition observed through chapter 12's verb** — but the choice of *this* verb
to observe it through is the test's own argument, and it is the right one:
`leaf-decompose` is the verb where landing on the wrong twin would not merely mark
the wrong file but restructure it.

It would pass while the property was broken if the refusal came from anywhere at
all that mentioned *two entries in this tree carry key 1* — the assertion is on
that substring — so it is a claim about the message and about the tree's
unchangedness, not about which function produced either.

<!-- fragment «decompose-tests-destination» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2042-2112" parent="decompose-tests" -->
````rust
    #[test]
    fn destination_occupied_is_unreachable_because_the_occupant_duplicates_the_key() {
        // **The row this leaf was sent to check.** The node a promotion composes
        // is `compose(ordinal, key, node parts)` with the *leaf's own* ordinal
        // and key, so the only name that can already occupy the destination is a
        // node carrying that key — which makes the key duplicated tree-wide, and
        // `addressable_key` refuses before any operation is planned. The same
        // argument `marking-k32` made for the marking verbs, reaching the last
        // row the table still predicted. Both shapes of occupant are checked:
        // the node with a brief (an ordinary hand edit) and the node without one
        // (an interrupted promotion), because they take different branches and
        // only the second is a state the library can leave behind.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);

        let err = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 3"),
            "the duplicate key is what is wrong with this tree, and the taken \
             destination is a consequence: {err}"
        );
        assert!(
            !err.to_string().contains("already taken"),
            "`Refusal::DestinationOccupied` must not reach an operator: {err}"
        );
        // The control, because a reachability claim that cannot fail is worth
        // nothing: with Grove's check removed, this same tree does reach the
        // library — and which refusal it reaches is not even determined, since
        // `by_key` on a duplicate key answers with whichever entry the walk
        // reaches first and walk order on such a tree is one of `structure.als`'s
        // recorded misses. Either wording is the library's, about a state Grove
        // can describe better.
        assert!(
            library_promotion_refusal(&g, 3, "build").contains("already taken")
                || library_promotion_refusal(&g, 3, "build").contains("promotion turns a"),
            "the refusal has to be there for Grove's check to be what hides it: {}",
            library_promotion_refusal(&g, 3, "build")
        );
    }

    /// Call the library's `promote` directly, bypassing every precondition Grove
    /// puts in front of it, and return the message it answers with.
    ///
    /// The positive control for the two reachability claims above. Grove has no
    /// production path that does this — clause 1 resolves an argument to an entry
    /// and clause 2 classifies it first — which is exactly why the claims need an
    /// instrument that does.
    fn library_promotion_refusal(grove_root: &Path, key: u32, slug: &str) -> String {
        let tree = crate::task_tree::write(grove_root).unwrap();
        match tree.promote(
            ordinal_fs_tree::Key::new(key),
            Parts::node(Slug::new(slug).unwrap()),
            TaskName::Brief,
            None,
        ) {
            Ok(_) => panic!("the library must refuse this tree"),
            Err(error) => error.to_string(),
        }
    }

````
<!-- /fragment -->

**`destination_occupied_is_unreachable_because_the_occupant_duplicates_the_key`**
is the test this chapter would be poorer without, because it is the only one in
the block that carries its own **control**.

The argument is arithmetic. The node a promotion composes is
`compose(ordinal, key, node parts)` with the *leaf's own* ordinal and key, so the
only name that could already occupy the destination is a node carrying that key —
which makes the key duplicated tree-wide, which `addressable_key` refuses before
any operation is planned. `Refusal::DestinationOccupied` is therefore unreachable
through this verb, and the second assertion says so directly: the operator must
not see *already taken*.

**And then the test proves that its own claim could have failed.** Lines 2082 to
2087 call `library_promotion_refusal`, which reaches past every precondition grove
puts in front of the library and calls `promote` directly — and asserts that the
library *does* refuse this same tree. Without that, *grove's check hides the
library's refusal* would be indistinguishable from *there was no refusal to hide*,
and the comment says exactly that: **a reachability claim that cannot fail is
worth nothing**.

**The control opens the tree again to do it, and that is only safe because a
refusal let go of the lock.** `library_promotion_refusal` takes a fresh
`task_tree::write` guard on every call, and the `assert!` can call it up to three
times — once for the first disjunct, once more for the second if the first was
false, and once more in the failure message. Each of those is an exclusive
acquisition. The verb's own guard is not held by then, and not because `promote`
consumed it: the call on lines 2057 to 2063 is refused inside `addressable_key`,
before the operation is reached, so `leaf_decompose` — which takes its `Guard` by
value — drops it on the error path. **A refusal releases the lock by returning.**
Grove's guard `flock`s the directory and so excludes rather than nests, which
chapter 5 established, so a version of this control reached from inside a held
guard would not fail; it would hang.

The control's own assertion is a disjunction — `already taken` **or**
`promotion turns a` — and the comment explains why it has to be: on a
duplicate-key tree, `by_key` answers with whichever entry the walk reaches first,
and walk order on such a tree is one of the library's own recorded modelling
misses. So the test cannot say *which* refusal the library raises, only that it
raises one of the two. A disjunction is usually a weakened assertion; here it is
the precise one.

`library_promotion_refusal` is the only call to the library's `promote` anywhere
in grove's own crates other than `leaf_decompose` itself: sweeping every `.rs`
file under `crates/` for `.promote(` returns eleven hits, and exactly two of them
— lines 575 and 2099 of this file — are grove's. The other nine are
`ordinal-fs-tree`'s own binary and its `promoting_on_disk` tests, which is what a
library's own tests are for. That two-of-eleven is the enumeration behind the doc
comment's *Grove has no production path that does this*, and it is what makes
`library_promotion_refusal` a test instrument built to be the thing production is
not.

**Both shapes of occupant are checked, and only one of them is a state the library
can leave behind.** The fixture here creates the node *with* a `BRIEF.md` — an
ordinary hand edit. The next test creates one *without*. They take different
branches inside `addressable_key`, and the second is the interrupted promotion.

<!-- fragment «decompose-tests-interrupted» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2113-2155" parent="decompose-tests" -->
````rust
    #[test]
    fn an_interrupted_promotion_is_diagnosed_as_one_rather_than_as_a_hand_edit() {
        // The tree `Error::FailedPartiallyRolledBack` warns about, met by a
        // *later* command — which is the only way it is ever met, since the run
        // that caused it already reported it and exited. The library reports
        // nothing here: a duplicate key is an obligation on the domain and no
        // operation checks it. So the recovery advice is Grove's to give, and it
        // is the library's own — remove either half — and not
        // `addressable_key`'s general *give one a fresh key*, which would make
        // two entities out of one caught mid-shape-change.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k3.md", "build-k3");
        fs::create_dir(g.join("02-build-k3")).unwrap();
        commit_all(&g);

        let err = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k3.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap_err();

        let message = err.to_string();
        assert!(
            message.contains("interrupted `leaf-decompose`"),
            "got {message}"
        );
        assert!(
            message.contains("holds no BRIEF.md") && message.contains("Removing either half"),
            "the recovery has to be the library's own: {message}"
        );
        assert!(
            !message.contains("fresh key"),
            "the general duplicate-key advice is wrong for this tree: {message}"
        );
        assert!(
            g.join("02-impl--build-k3.md").is_file(),
            "a refused promotion creates nothing and repairs nothing"
        );
    }

````
<!-- /fragment -->

**`an_interrupted_promotion_is_diagnosed_as_one_rather_than_as_a_hand_edit`** —
the tree `Error::FailedPartiallyRolledBack` warns about, met by a *later* command,
which the comment observes is the only way it is ever met: the run that caused it
already reported it and exited.

Three assertions and one of them is negative, which is where the claim lives. The
message must contain *interrupted `leaf-decompose`*; it must contain both
*holds no BRIEF.md* and *Removing either half*; and it must **not** contain
*fresh key*. The last is the real assertion: `addressable_key`'s general
duplicate-key advice is *give one of them a fresh key*, and that advice is
actively wrong here, because the node and the leaf are one entity caught
mid-shape-change and a fresh key would make two of it. Without the negative
assertion the test would pass while the general message was being given.

It would pass while the property was broken if the two positive substrings were
produced for a tree that was *not* an interrupted promotion — the fixture is what
rules that out, and it is built by hand: `fs::create_dir` with no `BRIEF.md` in it,
which is the one shape grove itself never writes, because `leaf-decompose` creates
the brief in the same store operation.

**This is the chapter's rule read from the wreckage.** An interrupted promotion is
two halves of *one* entity, and the only reason that sentence is available at all
is that the key was preserved. Had the promotion allocated the node a new key, the
two would be two entities sharing an ordinal, and *remove either half* would be
the wrong advice.

<!-- fragment «decompose-tests-last-key» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2156-2186" parent="decompose-tests" -->
````rust
    #[test]
    fn a_tree_at_the_last_key_refuses_the_promotion_rather_than_wrapping() {
        // The one refusal `leaf-decompose` really can reach, and it comes from
        // the **first child** rather than from the node: a promotion allocates
        // no key for the node — the entity is unchanged — so the only `max + 1`
        // in the operation is the child's. Grove predicts `None`, hands the
        // library no bytes, and lets it state the condition (clause 3).
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--build-k4294967295.md", "build-k4294967295");
        commit_all(&g);

        let err = leaf_decompose(
            guard(&g),
            Path::new("02-impl--build-k4294967295.md"),
            &a_slug("step"),
            Some(a_kind("impl")),
        )
        .unwrap_err();

        assert!(
            err.to_string().contains("greatest a key can be"),
            "got {err}"
        );
        assert!(
            g.join("02-impl--build-k4294967295.md").is_file()
                && !g.join("02-build-k4294967295").exists(),
            "a refusal writes nothing"
        );
    }

````
<!-- /fragment -->

**`a_tree_at_the_last_key_refuses_the_promotion_rather_than_wrapping`** — the one
refusal `leaf-decompose` really can reach, and the comment names its provenance
precisely: it comes from the **first child**, not from the node. A promotion
allocates no key for the node — the entity is unchanged — so the only `max + 1` in
the whole operation is the child's. Grove predicts `None`, hands the library no
bytes, and lets it state the condition, which is clause 3.

The mutation confirms the attribution and its uniqueness: making `promote`'s
error propagation panic reddens this test and no other. It is the sole observer of
the only library refusal this verb surfaces, and the fixture is a leaf at key
4,294,967,295 — `u32::MAX`, the greatest a key can be.

It would pass while the property was broken if some *other* exhaustion produced a
message containing *greatest a key can be*; the assertion is a substring check
against the library's own wording, which clause 3 requires grove to print
unchanged. The second assertion — the leaf still a file and no node directory
beside it — is what makes it a claim about a refusal that *writes nothing*, and it
is the sentence the second part of the what-could-not-move test asks for.

<!-- fragment «decompose-tests-sweep» owner="the-key-survives" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2187-2241" parent="decompose-tests" -->
````rust
    #[test]
    fn no_promotion_refusal_reaches_an_operator_from_an_ordinary_argument() {
        // The three refusals `promote` owns, asserted unreachable rather than
        // described. `SuppliedNameNotDistinguished` is discharged by the domain itself
        // and needs no fixture; the other two are discharged by every argument
        // that is not a live leaf, and the sweep is what makes that a claim
        // about the *verb* rather than about the cases someone thought of.
        assert!(
            matches!(
                TaskName::Brief.view(),
                ordinal_fs_tree::NameView::Distinguished
            ),
            "Grove's distinguished child is BRIEF.md, so a promotion always has \
             somewhere to put the leaf's bytes"
        );
        assert_eq!(
            Parts::node(crate::task_name::Slug::new("build").unwrap()).species(),
            ordinal_fs_tree::PositionedSpecies::Node,
            "`leaf-decompose` composes node parts and nothing else, so \
             `PromotePartsNotNode` cannot fire"
        );

        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-DONE-impl--done-k1.md", "done-k1");
        touch(&g, "02-ABANDONED-impl--gone-k2.md", "gone-k2");
        touch(&g, "03-finish--wrap-k3.md", "wrap-k3");
        let node = mknode(&g, "04-build-k4", "build-k4");
        commit_all(&g);

        for argument in [
            g.as_path(),
            &g.join("BRIEF.md"),
            &g.join("01-DONE-impl--done-k1.md"),
            &g.join("02-ABANDONED-impl--gone-k2.md"),
            &g.join("03-finish--wrap-k3.md"),
            node.as_path(),
            &node.join("BRIEF.md"),
        ] {
            let err = leaf_decompose(guard(&g), argument, &a_slug("step"), Some(a_kind("impl")))
                .unwrap_err()
                .to_string();
            for library_wording in [
                "promotion turns a",
                "this domain has no distinguished child",
                "make a leaf, not a node",
            ] {
                assert!(
                    !err.contains(library_wording),
                    "{argument:?} reached a `promote` refusal: {err}"
                );
            }
        }
    }

````
<!-- /fragment -->

**`no_promotion_refusal_reaches_an_operator_from_an_ordinary_argument`** is the
block's closing argument and the widest test in it. Three refusals `promote` owns,
asserted unreachable rather than described.

**Two of the three are checked without a tree.**
`SuppliedNameNotDistinguished` cannot fire for the `TaskName::Brief` value Grove
passes: the first assertion checks its distinguished view. `PromotePartsNotNode` cannot fire because `leaf-decompose`
composes `Parts::node` and nothing else, which the second assertion reads off
`Parts::node(...).species()`. Neither needs a fixture, because neither is a fact
about a tree.

**The third is discharged by a sweep, and the sweep is what makes it a claim about
the verb.** Seven arguments — the grove root, the root's charter brief, a `DONE`
leaf, an `ABANDONED` leaf, a `finish` leaf, a node directory and that node's own
brief — each run through the verb, and each resulting message is checked against
three library wordings it must not contain. Naming the cases someone thought of
would be a claim about those cases; enumerating every argument shape that is not a
live leaf is a claim about the function.

**This test is the only observer of one of the block's arms.** The mutation study
attributes to it, and to nothing else, the grove-root refusal on line 550; it is
also a second observer of all five `decomposable` refusals, each of which has a
named test of its own — including the `finish`-reservation on line 622, which
`every_agent_side_mutation_refuses_the_driver_reserved_finish_kind` holds from
outside the crate. That makes it the most load-bearing test in the chapter — and
it asserts only *absence*, which is worth saying: it can tell you no library
wording leaked, and it cannot tell you which
grove refusal fired instead. Six arms reddening one test is what the sweep buys and
also what it costs.

It would pass while the property was broken if a refusal reached the operator in
the library's *voice* under a wording not in the list of three. The list is exact —
`promotion turns a`, `this domain has no distinguished child`, `make a leaf, not a
node` — and a fourth `Refusal` variant with a fourth message would sail through.
What bounds that risk is not this test but
`docs/ARCHITECTURE.md#library-refusals`'s row for this verb, which the section
comment points at rather than copies — and which says the only refusal
`leaf-decompose` reaches at all is `KeysExhausted`, from the first child.

<a id="what-the-refusals-are-worth-measured"></a>
## What the refusals are worth, measured

Every attribution above rests on this, and this is a measurement rather than a
reading. The procedure is the one chapters 8 to 11 established: replace each
refusal or fallback arm with a **panic** in a copy of the workspace, run the whole
of `grove-loop` and `grove-llm`, and diff the per-test results against an
unmutated control run of the same copy.

**The control is 560 tests with 11 failing, and getting there needs one step the
node's harness does not name.** The copy is not a jj repository and does not carry
the marketplace manifest, so eleven of `crates/grove-loop/tests/prompt.rs`'s tests
fail before any mutation. But `cargo test -p grove-loop -p grove-llm` alone leaves
`CARGO_BIN_EXE_grove` unset — the `grove` binary belongs to a third package — and
six further `grove-llm` tests then fail on a missing binary rather than on
anything about the code. `cargo build -p grove --bins` first is what makes the
control eleven rather than seventeen, and a control that is wrong in that
direction hides observers rather than inventing them.

**Three things make a reading a lie, and this chapter hit all three.**

1. **A mutant that fails to compile prints no per-test lines and reads exactly
   like a clean result.** Every run below was confirmed to have executed all 560.
2. **A message-preserving panic is not a mutation for an out-of-process
   observer.** `bail!(…)` → `panic!(…)` keeps the format string, so a `grove-llm`
   integration test that shells out and asserts on stderr substrings stays green:
   the panic prints the same words the refusal did, and the exit status is
   non-zero either way. The first pass over these arms was void for that reason.
   Replacing the **whole macro call** with `panic!("MUTANT")` found three
   observers the first pass had recorded as absent —
   `decompose_rejects_a_brief` and `decompose_rejects_a_retired_leaf`
   (`crates/grove-llm/tests/leaf_ops.rs`) and
   `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind`
   (`crates/grove-llm/tests/session_kind_tree.rs`). This is the dual of the rule
   chapter 9 arrived at, that a reworded `bail!` is not a mutation.
3. **A flaky test reads as a newly attributed observer.** The first reading of the
   grove-root arm credited two `crates/grove-loop/tests/driver_lease.rs` tests to
   it; both are 120-second wedged-producer timeouts under load, and a second run
   of the same mutant attributed the arm to the sweep alone. Every observer named
   below survived a second run of its mutant.

**Sixteen arms, and the line each sits on.**

| # | Arm | Line | Observed by |
|---:|---|---:|---|
| 1 | `bail!` the argument is the grove root | 550 | `no_promotion_refusal_reaches_an_operator_from_an_ordinary_argument` |
| 2 | `refuse_finish_kind` — the `--kind finish` **override** | 560 | **nothing** |
| 3 | `promote` refused — `task_tree::raised` | 576 | `a_tree_at_the_last_key_refuses_the_promotion_rather_than_wrapping` |
| 4 | `reopen_write` failed before the retitle | 586 | **nothing** |
| 5 | `append_brief_suffix_in_file` failed | 587 | **nothing** |
| 6 | `bail!` a charter brief | 602 | `decompose_refuses_a_brief`; `decompose_rejects_a_brief`; the sweep |
| 7 | `bail!` a node | 606 | `decompose_refuses_a_node_directory`; the sweep |
| 8 | `bail!` a retired (`DONE`) leaf | 611 | `decompose_refuses_a_done_leaf`; `decompose_rejects_a_retired_leaf`; the sweep |
| 9 | `bail!` an abandoned leaf | 615 | `decompose_refuses_an_abandoned_leaf`; the sweep |
| 10 | `bail!` the driver-reserved `finish` kind | 622 | `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind`; the sweep |
| 11 | `bail!` the library created other than two entries | 652 | **nothing** |
| 12 | `bail!` the node the library made is a charter brief | 659 | **nothing** |
| 13 | `bail!` the node's key is not the leaf's | 666 | **nothing** |
| 14 | `bail!` the first child is a charter brief | 675 | **nothing** |
| 15 | `bail!` the child's key is not the predicted one | 681 | **nothing** |
| 16 | `.context` the library reported no rename | 692 | **nothing** |

Rows 1, 3 and 6 to 10 are the control that makes the other nine readable: each
reddens a small, attributable set, so the instrument demonstrably fails when it
should. Two further arms outside this chapter's block were mutated for
attribution and are named where they belong — `task_tree::target`, whose failure
*through this verb* reddens `decompose_refuses_a_foreign_file` alone, and
`task_tree::addressable_key`, whose failure through this verb reddens the twin,
destination and interrupted-promotion tests and nothing else. Both are chapter
6's, and *through this verb* is load-bearing: mutated in place rather than at the
call site, `addressable_key`'s duplicate-key block reddens six — the same three
plus `insert_refuses_a_target_whose_key_names_two_entries`,
`retiring_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed` and
`prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address` — and `target`'s
walk-exhausted arm reddens four. The three named here are what this verb's own
fixtures hold.

**Nine of sixteen arms are held by nothing, and eight of the nine fall into two
classes.** Rows 11 to 16 are the class chapters 10 and 11 named: *the library did
something its contract forbids*, which no test over an honest library can
construct. Rows 4 and 5 are I/O failures on the retitle — the tree was opened
once already in the same call, so a second opening failing is a filesystem event
a test would have to simulate. Neither group is a gap a fixture could close.

**Row 2 is the one that is not, and it is an ordinary operator-facing refusal.**
`refuse_finish_kind` on line 560 guards `--kind finish` — an *override* naming the
driver's reserved kind on a leaf whose own kind is something else. It is not the
same arm as row 10: row 10 refuses decomposing a `finish` **leaf**, and the
mutation separates them cleanly, because the classification on line 556 runs first
and takes the finish-leaf case before line 560 is reached. Removing line 560
outright reddens nothing — the suite lands exactly on its 560-test control — so
nothing in this workspace passes `--kind finish` to this verb.

**And the path is open.** `grove-llm`'s `cmd_leaf_decompose` parses `--kind` into
a `Kind` and passes it straight through; the check it runs first,
`require_declared`, asks whether the *launch configuration* declares that kind, not
whether grove reserved it. So an operator whose configuration declared a `finish`
template would meet line 560 and nothing else. The help text says so —
`leaf-decompose`'s `--kind` is documented as *except driver-reserved finish* — and
the sentence is held by one unobserved line.

**The contrast with chapter 11 is the point.** `refuse_finish_kind` is called from
four places: `leaf-add` and `leaf-insert` in chapter 10, `root-init` in chapter 11,
and here. Chapter 10's two are held — `add_rejects_finish` and
`insert_rejects_finish` name one call site each, and
`the_growing_verbs_refuse_the_drivers_reserved_kind` reaches both from outside the
crate. Chapter 11's is unobserved
because **there is no flag** — every call site passes `Kind::requirements()` as a
literal, so the reserved kind cannot reach it. This one is unobserved with a flag
wide open in front of it. The same function, the same refusal, and three different
reasons for the coverage each call site has.

<a id="what-could-not-move"></a>
## What could not move

**On the way in — the names.** Almost nothing new, and the almost is instructive.
This verb reads no filename it did not get from the tree: the slug arrives as a
`Slug` the caller built, the kind as a `Kind`, and the leaf as a path the resolver
turned into an entry. What the chapter contributes on this axis is one *writing* —
the node's name, composed from the promoted leaf's own triple with `Parts::node`
swapped in — and the observation that a preserved key and a preserved ordinal are
one fact because that composition makes them one string. The grammar chapter 4
proved canonical is what lets `02-build-k3` be read back as the same entity it
names.

**On the way through — the preconditions.** This is the chapter's weight, and the
form the question takes here is unusually clean. The library's promotion
can refuse six ways, and grove reaches exactly **one** of them: `KeysExhausted`,
from the first child's key allocation — which is what
`docs/ARCHITECTURE.md#library-refusals`'s row for this verb says, and what the
mutation confirms with one arm and one observer. The three that are about being a
promotion are not caught, not translated and not re-worded; they are made
*unreachable*, by a classification grove needed anyway, and `DestinationOccupied`,
raised a layer lower by the planner, is made unreachable by arithmetic.
`decomposable` is thirty-eight lines and it is the whole of that: five arms over
four conditions, none of which the store can see, run against the same snapshot
the operation then plans from, under the same guard.

And the cost is visible in the same block. A promotion breaks an invariant on the
way through, so the intermediate state has to be unobservable, so every reader in
the workspace has to go through one of two functions — which is a constraint on
code that has nothing to do with decomposition, held by a test in a different
crate. That is what *the preconditions* costs when the operation underneath is not
atomic against the thing it is preserving.

**On the way out — the policy.** One choice, made once, and it is the chapter's
title: a decomposed leaf keeps its key. Nothing beneath grove could have defaulted
that, because nothing beneath grove has a notion of an entity that outlives a
change of species. The library offers key preservation as a *property of
promotion*; grove is what decides that the property is the point. The proof is
distributed rather than local — the on-disk assertion in the first test, the
prediction check in `promoted`, and the recovery advice in `addressable_key` that
is only correct because the two halves are one entity — and the third of those is
the one that would be wrong under any other choice.

The second choice is smaller and it is the one the measurement caught: **the
first child's kind is the parent's unless a flag says otherwise, and the flag can
name a kind grove reserved.** The refusal is there, on line 560, and it is held by
nothing.

The grove now has a node where it had a leaf, and the node has a live child that
`pick` will answer. Chapter 13 takes a leaf that is *finished* rather than too
big, and marks it — in the name, without rewriting a byte of the body.

[Previous: A grove begins](11-a-grove-begins.md) | [Contents](README.md) | [Next: Outcomes are marked in place](13-outcomes.md)

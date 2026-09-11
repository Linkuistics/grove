# Outcomes are marked in place
<!-- book-page id="outcomes" slice="marked-in-place" order="13" -->
[Previous: A leaf becomes a node](12-leaf-to-node.md) | [Contents](README.md) | [Next: Finishing](14-finishing.md)

<a id="marked-in-place"></a>
## The rule: the mark is in the name, and nothing else is touched

Chapter 12 took a leaf that had proved too big and changed its species. This
chapter takes a leaf whose work is **over** — finished, or abandoned — and
changes neither its species, its position, its key nor a byte of what it says.
All that moves is a word in its filename.

> **Done-ness and abandonment are marked in the name, and the body is not
> rewritten.** An outcome is a substring of a filename and is recorded nowhere
> else: not in a header, not in front matter, not in a status file, and not in
> the version control system.

**Say which tree.** In `ordinal-fs-tree`'s vocabulary the operation is a
`rewrite`, and the library's own algebra is what makes *in place* precise: the
entry keeps its ordinal, keeps its key and keeps its species, and only the opaque
remainder of its name moves. In grove's vocabulary a *leaf* is one session's whole
work, and the mark is how that session's work ended. The two sentences describe
the same `rename(2)`, and the rest of this chapter says which of them is speaking.

**This is the chapter where the crate's second thesis is most nearly literally
true.** *The tree's shape is the only state grove keeps* — which is why `pick`
can step over finished work without opening a single file, why an interrupted
bulk mark needs no recovery procedure, and why there is no operation in this
module that writes an outcome anywhere a reader would have to look for it. The
book does not carry that as its spine, and the reason is chapter 16: 1,383 lines
of locking whose whole purpose is to hold state the tree must **not** hold.
`driver_lease.rs` opens by naming it — the seam owns *where an untracked
coordination directory may live* — and the two
chapters are the thesis and its deliberate exception. State that must survive a
checkout is spelled in a name; state that must not survive one is spelled in a
file no commit records.

The carried example reaches its thirteenth step. The grove chapters 11 and 12
built now holds work that is over.

```text
<worktree>/.grove/
├── BRIEF.md
├── 01-requirements--plan-k1.md        a LEAF (grove): its session is finished
└── 02-build-k3/
    ├── BRIEF.md
    └── 01-impl--step-k4.md

leaf_retire(guard, "01-requirements--plan-k1.md")

<worktree>/.grove/
├── BRIEF.md
├── 01-DONE-requirements--plan-k1.md   position 01 KEPT, key 1 KEPT, kind KEPT
│                                      "# plan-k1"  ->  "# plan-k1"  (unchanged)
└── 02-build-k3/
    ├── BRIEF.md
    └── 01-impl--step-k4.md

  ⇒ Ok( <root>/01-DONE-requirements--plan-k1.md )
```

`retire_adds_done_infix_keeping_position_and_key` and
`retire_does_not_rewrite_the_header_or_body` are the pair that make *in place*
mean something, and they are a pair because either one alone is satisfied by an
implementation that gets the other wrong. The first says the name moved and moved
correctly; the second says nothing else did.

The second verb is the same mark with the opposite meaning and a different arity,
and it is where the chapter's cost lives. `leaf-prune` marks work `ABANDONED`,
and given a node it marks **every** live leaf beneath it — which, because a
mutation consumes its guard, is *N* renames under *N* locks where grove once had
one critical section.

<a id="two-verbs-one-mark"></a>
## The simple half: one classification, one rewrite

The chapter's first ownership block is `tree_lifecycle.rs` lines 696 to 1012 —
317 lines, the fourth of the file's five production blocks in file order and the
second largest of them, behind the 331 lines chapter 14 owns at the top of the
file. It holds eleven items: two verbs, one public result type, one private plan
enum, and seven private helpers. The first three items are `leaf-retire`, and
they are the whole of it.

<!-- fragment «outcomes-in-place» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="699-1015" parent="source-tree-lifecycle" -->
<!-- insert «outcomes-retire-contract» -->
<!-- insert «outcomes-retire-body» -->
<!-- insert «outcomes-retire-parts» -->
<!-- insert «outcomes-prune-result» -->
<!-- insert «outcomes-prune-contract» -->
<!-- insert «outcomes-prune-body» -->
<!-- insert «outcomes-planned» -->
<!-- insert «outcomes-plan-prune» -->
<!-- insert «outcomes-plan-subtree» -->
<!-- insert «outcomes-plan-leaf» -->
<!-- insert «outcomes-apply-prune» -->
<!-- insert «outcomes-stopped-partway» -->
<!-- insert «outcomes-marked-path» -->
<!-- /fragment -->

The doc comment states the contract and then states what the mark *is*, and the
second half is the load-bearing one.

<!-- fragment «outcomes-retire-contract» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="699-709" parent="outcomes-in-place" -->
````rust
/// `leaf-retire <leaf-path>`: rename a live leaf `NN-<kind>--<slug>-k<key>.md` →
/// `NN-DONE-<kind>--<slug>-k<key>.md` in place, keeping its position and key. The
/// `DONE` infix is filename-only — the `# <handle>` header is byte-identical.
/// Refuses a brief, a node directory, and an already-`DONE` leaf. Returns the
/// retired file's absolute path. Working-tree only — no commit.
///
/// **The mark is `ordinal_fs_tree`'s `rewrite`**, which is what a mark *is*
/// algebraically: the entry keeps its ordinal, its key and its species, and only
/// the opaque remainder of its name moves. The rename underneath is
/// `rename(2)` — plain, consulting no repository; see
/// [`docs/adr/grove-does-not-stage-its-own-renames.md`](../docs/adr/grove-does-not-stage-its-own-renames.md).
````
<!-- /fragment -->

**The mark is `rewrite`, and the comment says why that is the right primitive
rather than merely the available one.** A mark is algebraically an operation that
holds an entry's identity fixed and moves only the part of its name nothing
depends on — which is exactly `rewrite`'s contract, and exactly why grove does not
reach for a rename of its own. The claim underneath it is narrower and worth
keeping straight: the rename is `rename(2)`, plain, consulting no repository. That
is not an optimisation. The record it cites,
`docs/adr/grove-does-not-stage-its-own-renames.md`, exists because the verbs once
reached for a version-control-aware move, and a leaf grown this session has
nothing recorded for such a move to find — which is the defect the untracked-leaf
tests further down this page were written for.

<!-- fragment «outcomes-retire-body» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="710-730" parent="outcomes-in-place" -->
````rust
pub(crate) fn leaf_retire(tree: Guard, leaf_path: &Path) -> Result<PathBuf> {
    // The classification, then the guard: `rewrite` consumes the guard, so the
    // borrow of its snapshot has to end before the call.
    let (key, parts) = {
        let entry = match task_tree::target(tree.root(), tree.snapshot(), leaf_path)? {
            task_tree::Target::Root => bail!(
                "cannot retire the grove root (lifecycle verbs act on leaves): {}",
                tree.root().display()
            ),
            task_tree::Target::Entry(entry) => entry,
        };
        let parts = retire_parts(&entry)?;
        (
            task_tree::addressable_key(tree.root(), tree.snapshot(), &entry)?,
            parts,
        )
    };
    let report = tree.rewrite(key, parts).map_err(task_tree::raised)?;
    marked_path(&report)
}

````
<!-- /fragment -->

**The body is a classification and one call, in that order, and the comment on
line 708 is the same one chapter 12's verb carries.** `rewrite` consumes the
guard, so every borrow of the guard's snapshot has to end before the call; the
braces around lines 710 to 723 are what gets the two values the operation needs —
the key and the new parts — out of the snapshot's lifetime alive. Chapter 12's
`leaf_decompose` has the identical shape for the identical reason, and the two
comments are nearly the same sentence.

What is *not* here is any check that the entry may be marked. That is the next
item's whole job, and the split is deliberate: the verb reads as one refusal
gate, one operation, one answer.

<!-- fragment «outcomes-retire-parts» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="731-767" parent="outcomes-in-place" -->
````rust
/// The key to rewrite and the `DONE` parts to give it, or Grove's own refusal.
///
/// Every clause here is a precondition the library cannot see — an outcome
/// infix, `finish`-reservation, brief-ness — which is why classifying before
/// calling is not optional (`docs/ARCHITECTURE.md#library-refusals`, clause 2).
/// The species refusal `rewrite` would make sits behind them and is therefore
/// unreachable, exactly as that document's table says.
fn retire_parts(entry: &Entry<'_, TaskName>) -> Result<Parts> {
    let name = entry.name();
    let Some(triple) = entry.triple() else {
        bail!("cannot retire a brief (briefs are never done): {name}")
    };
    match triple.parts {
        Parts::Node { .. } => {
            bail!("cannot retire a node (nodes are never marked done): {name}")
        }
        Parts::Leaf {
            outcome: Outcome::Done,
            ..
        } => bail!("leaf is already retired (DONE): {name}"),
        Parts::Leaf {
            outcome: Outcome::Abandoned,
            ..
        } => bail!("cannot retire an abandoned (ABANDONED) leaf: {name}"),
        Parts::Leaf {
            outcome: Outcome::Live,
            kind,
            slug,
        } => {
            if kind.is_finish() {
                bail!("`finish` is driver-reserved and cannot be retired");
            }
            Ok(Parts::leaf(Outcome::Done, kind.clone(), slug.clone()))
        }
    }
}

````
<!-- /fragment -->

**Every clause is a precondition the library cannot see, and the comment names
all three.** An outcome infix, `finish`-reservation and brief-ness are grove's
vocabulary; the store sees an ordered tree of entries and has no word for any of
them. That is the second part of the what-could-not-move test in its cleanest
form — *does the layer check what the library cannot see* — and this function is
thirty-six lines of nothing else.

The last sentence is the one to read twice: **the species refusal `rewrite` would
make sits behind these and is therefore unreachable.** The library would refuse
to rewrite a directory as a leaf; grove refuses it first, on line 742, with a
sentence about nodes rather than about species. So the library's refusal is not
caught, not translated and not re-worded — it is made unreachable by a check grove
needed anyway. `docs/ARCHITECTURE.md#library-refusals` states this as clause 2 and
its table records the consequence, and chapter 12 met the same shape at
`decomposable`. It is worth noticing how far this goes compared with
chapter 12's. `docs/ARCHITECTURE.md#library-refusals`'s table gives
`leaf-decompose` one reachable library refusal — `KeysExhausted`, from the first
child's key — and gives `leaf-retire` and `leaf-prune` **none at all**.

Four other rows in that table — seven other verbs — also reach none, and reading
them beside this one is what says how much the empty cell is worth. Two of the
four are empty because there was nothing to refuse: `root-init`'s target is not an
entry and its level is empty, and `complete` touches no tree at all. A third is
empty because the library's reading surface constructs no refusal in the first
place — `pick`, `brief-chain`, `kind` and `resolve` answer with a `Sought`.

Only `finish-commit` is empty in the same *way* this row is: `delete` refuses a
root spelled through a link, and the verb has already refused that unfollowed for
its own reason, so a real refusal is put out of reach by a check grove wanted
anyway. The marks do that twice, and the second time is the one worth the
sentence. The species refusal is hidden the ordinary way, behind a classification.
`DestinationOccupied` is not hidden behind a check aimed at it at all: an outcome
infix and a key are both parts of one name, so the only entry that could occupy a
mark's destination is one carrying the marked leaf's own key — which makes the
condition coincide with a **different** condition, the duplicate key that
`addressable_key` scans for on chapter 6's account rather than on this chapter's.
The closing section is about what that costs.

**The `Live` arm is the only one that constructs anything**, and what it
constructs is `Parts::leaf(Outcome::Done, kind, slug)` — chapter 3's constructor,
given the entry's own kind and its own slug. The kind and the slug are cloned
across unchanged, which is the whole of *in place* expressed as an expression: the
new parts differ from the old in one field, and that field is the outcome.

<a id="one-guard-is-one-mark"></a>
## One guard is one mark, and a subtree is many

The remaining eight items are `leaf-prune`, and they are eight rather than three
because the verb has an arity `leaf-retire` does not: given a node, it marks every
live leaf beneath it. The public result type is where that shows first.

<!-- fragment «outcomes-prune-result» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="768-778" parent="outcomes-in-place" -->
````rust
/// The outcome of a [`leaf_prune`] call: every leaf newly marked `ABANDONED`
/// (its new path), and every already-`DONE` leaf found in scope and left
/// untouched (pruning: that work really was done). A single-leaf call
/// marks exactly one entry and finds nothing to leave alone; a node call is
/// bulk — the arity asymmetry with `leaf-retire` is deliberate (pruning).
#[derive(Debug)]
pub struct PruneResult {
    pub marked: Vec<PathBuf>,
    pub left_done: Vec<PathBuf>,
}

````
<!-- /fragment -->

**Two vectors, and the second one is the interesting one.** `marked` is the answer;
`left_done` is a report about work the call deliberately did **not** touch. A
retired leaf inside a subtree being abandoned stays retired, because — as the
comment puts it — *that work really was done*, and a bulk abandon does not
retroactively un-finish it. The type carries that as a returned value rather than
as a silence, so a caller can show an operator what was skipped and why.

The comment closes on the arity itself: *a single-leaf call marks exactly one
entry and finds nothing to leave alone; a node call is bulk — the arity asymmetry
with `leaf-retire` is deliberate*. The bare `(pruning)` at the end of that sentence
and of four others in this block is a compact citation of
`docs/ARCHITECTURE.md`'s *Human authority and completion* section, which carries
`pruning` as one of three `id`s on one heading. **It is worth saying exactly what
the reader will find there, because it is less than the citation suggests.** That
section states one thing about pruning: *abandoning a planned leaf or subtree is
human judgment and requires explicit confirmation before the agent marks it
`ABANDONED`*. It is the authority boundary, and it is what the `HITL` paragraph
below appeals to. It does **not** argue the arity asymmetry, and no record in the
repository does; the asymmetry's ground is in this comment and nowhere else, which
is a reasonable place for it — retiring is a claim about one session's work and
abandoning is a decision about a line of work — but a reader following the
citation for the argument will not find one.

<!-- fragment «outcomes-prune-contract» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="779-808" parent="outcomes-in-place" -->
````rust
/// `leaf-prune <path>`: mark abandoned work `ABANDONED` in place (pruning).
/// `path` is a live leaf file **or** a node directory (absolute, or relative to
/// the grove root):
///   * given a **leaf**, marks it directly — refuses a brief, an already-`DONE`
///     leaf, and an already-`ABANDONED` leaf;
///   * given a **node**, marks every *live* leaf in its subtree (recursively),
///     leaving `DONE` leaves untouched — refuses the grove root itself
///     (abandoning a whole workstream is a branch-delete, not a tree mark).
///
/// The `ABANDONED` infix is filename-only — every marked leaf's `# <handle>`
/// header stays byte-identical. Working-tree only — no commit.
///
/// # One guard is one mark, and a subtree is many
///
/// `rewrite` consumes its write guard, so a subtree prune is *N* rewrites under
/// *N* guards where it was once one critical section. Grove accepts that rather
/// than asking the library for a batched rewrite, and
/// [`docs/adr/bulk-marks-are-not-atomic.md`](../docs/adr/bulk-marks-are-not-atomic.md)
/// records why and what an operator does with a prune that stopped half way.
/// What survives the change is the up-front validation: the whole subtree is
/// planned and every destination checked against the **first** guard's snapshot
/// before any rename happens, so the failure that test suite has always covered
/// — a botched earlier prune leaving an `ABANDONED` twin in the way — still
/// leaves the tree untouched. What is lost is only the window *between* guards:
/// another writer, or a filesystem fault, can now stop the run partway.
///
/// **HITL (pruning):** this verb does not itself gate on human
/// confirmation — constraint 5 is "grove guides, it does not gate" — so the
/// caller (the LLM driving the session) must already have explicit human
/// confirmation before calling this at all.
````
<!-- /fragment -->

**The heading is the chapter's cost, stated by the code that pays it.** `rewrite`
consumes its write guard — the rule chapter 1 read off `TreeWrite`'s own header and
chapter 12 spent once — so a subtree prune is *N* rewrites under *N* guards where
it was one critical section. The comment does not present that as a detail, and
`docs/adr/bulk-marks-are-not-atomic.md` is a record about it rather than a passing
citation.

Three sentences carry the whole trade, and they should be read as three:

- **What survives** is the up-front validation. The whole subtree is planned and
  every destination checked against the **first** guard's snapshot, before any
  rename happens. So the failure the suite has always covered — a botched earlier
  prune leaving an `ABANDONED` twin in the way — still leaves the tree untouched.
- **What is lost** is only the window *between* guards. Another writer, or a
  filesystem fault, can now stop the run partway.
- **What repairs it** is running the verb again, and that is not stated here but
  in the ADR and in `stopped_partway`'s own message. It works because the marks
  *are* the state: an already-`ABANDONED` leaf is skipped silently, a `DONE` one is
  reported and left alone, so a second run finishes the rest and changes nothing
  else. This is the chapter's thesis doing load-bearing work rather than
  decorating it — a system that recorded outcomes anywhere but in the names would
  need a recovery procedure here, and grove needs none.

**The `HITL` paragraph is a statement about what this function does not do.**
Constraint 5 is *grove guides, it does not gate*, so the verb performs no
confirmation and refuses nothing on the ground of human authority; the obligation
sits on the caller, which is the LLM driving the session. The code contains no
enforcement of it, and the comment is explicit that this is by design rather than
by omission.

<!-- fragment «outcomes-prune-body» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="809-824" parent="outcomes-in-place" -->
````rust
pub(crate) fn leaf_prune(tree: Guard, path: &Path) -> Result<PruneResult> {
    let root = tree.root().to_path_buf();
    let plan = {
        let entry = match task_tree::target(&root, tree.snapshot(), path)? {
            task_tree::Target::Root => bail!(
                "cannot prune the grove root (abandoning a whole grove is a \
                 branch-delete, not a tree mark): {}",
                root.display()
            ),
            task_tree::Target::Entry(entry) => entry,
        };
        plan_prune(&root, tree.snapshot(), &entry)?
    };
    apply_prune(&root, tree, plan)
}

````
<!-- /fragment -->

**The body is fifteen lines and it delegates twice**, which is the shape the rest
of the block exists to fill in: resolve and refuse the root, plan under the
snapshot, then apply. The root refusal is the one clause that is genuinely this
function's, and its message is the argument rather than a label — *abandoning a
whole grove is a branch-delete, not a tree mark*. Marking every leaf in a grove
`ABANDONED` would leave a tree that says the work was rejected; deleting the
branch says it never happened, which is what a human who has abandoned a whole
workstream actually means.

`plan_prune` runs inside the braces and `apply_prune` outside them, and the reason
is the same one both this verb and `leaf_retire` open with: the plan borrows the
guard's snapshot, and the guard has to be moved into `apply_prune` afterwards.

<a id="plan-then-apply"></a>
## Plan the whole subtree, validate it, then spend the guards

Four private items do the work, and the split between them is the all-or-nothing
promise.

<!-- fragment «outcomes-planned» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="825-831" parent="outcomes-in-place" -->
````rust
/// One step of a planned prune: an entry to rewrite, or an already-`DONE` leaf
/// to report and leave alone.
enum Planned {
    ToMark { key: Key, parts: Parts },
    LeftDone { path: PathBuf },
}

````
<!-- /fragment -->

`Planned` is a two-case enum and both cases are outcomes of the walk rather than
of the marking: an entry to rewrite, or an already-`DONE` leaf to report. Nothing
in it can fail, because everything that could fail was decided before the value
was built.

<!-- fragment «outcomes-plan-prune» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="832-854" parent="outcomes-in-place" -->
````rust
/// Plan — and validate — the whole prune against one snapshot, mutating nothing.
///
/// Both halves matter. Planning first is what lets a subtree of *N* leaves be
/// checked before the first of *N* guards is spent; validating here rather than
/// leaf by leaf is what keeps the all-or-nothing promise a bulk verb makes and
/// the library, seeing one entry at a time, cannot.
fn plan_prune(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    entry: &Entry<'_, TaskName>,
) -> Result<Vec<Planned>> {
    let name = entry.name();
    let Some(triple) = entry.triple() else {
        bail!("cannot prune a brief (briefs are never marked): {name}")
    };
    let mut plan = Vec::new();
    match triple.parts {
        Parts::Node { .. } => plan_subtree(root, snapshot, entry, &mut plan)?,
        Parts::Leaf { .. } => plan.push(plan_leaf(root, snapshot, entry)?),
    }
    Ok(plan)
}

````
<!-- /fragment -->

**The doc comment's *both halves matter* is the design in one sentence.** Planning
first is what lets *N* leaves be checked before the first of *N* guards is spent.
Validating **here** rather than leaf by leaf is what keeps the all-or-nothing
promise a bulk verb makes — and the clause after it is the one that ties the
chapter to the spine: *and the library, seeing one entry at a time, cannot*. The
library's operation set has no batched rewrite, so no operation exists that could
own this promise; the promise is grove's because it is the only layer that can see
the whole subtree at once.

The function itself dispatches on species, and its one refusal is brief-ness — the
node's own `BRIEF.md` handed in directly, which is neither a leaf to mark nor a
node to walk.

<!-- fragment «outcomes-plan-subtree» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="855-891" parent="outcomes-in-place" -->
````rust
/// Every live leaf under a node, in the library's own per-level order, with each
/// already-`DONE` leaf collected untouched and each already-`ABANDONED` one
/// skipped silently — already terminal.
fn plan_subtree(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    node: &Entry<'_, TaskName>,
    plan: &mut Vec<Planned>,
) -> Result<()> {
    let Some(contents) = node.contents() else {
        return Ok(());
    };
    for child in contents.children() {
        let Some(triple) = child.triple() else {
            continue; // the node's own `BRIEF.md`
        };
        match triple.parts {
            Parts::Node { .. } => plan_subtree(root, snapshot, &child, plan)?,
            Parts::Leaf {
                outcome: Outcome::Live,
                ..
            } => plan.push(plan_leaf(root, snapshot, &child)?),
            Parts::Leaf {
                outcome: Outcome::Done,
                ..
            } => plan.push(Planned::LeftDone {
                path: task_tree::entry_path(root, child),
            }),
            Parts::Leaf {
                outcome: Outcome::Abandoned,
                ..
            } => {}
        }
    }
    Ok(())
}

````
<!-- /fragment -->

**Four arms over the children, and three of them are silences.** A node recurses.
A live leaf is planned. A `DONE` leaf is collected into `left_done` and left. An
`ABANDONED` one falls through an **empty arm** — already terminal, so there is
nothing to do and nothing to report. The comment on line 866, `the node's own
BRIEF.md`, marks the fourth: an entry with no triple is a charter, and the walk
steps over it without a word.

Two of these are worth naming as decisions rather than as code. **The recursion is
unbounded and deliberate** — *every live leaf in its subtree (recursively)*, so one
node argument can name work several levels down, which is what makes the verb a
decision about a line of work rather than about a directory. And **`ABANDONED` is
skipped where `DONE` is reported**, an asymmetry that looks arbitrary until it is
read against re-runnability: a second run of the same command must be a no-op over
what the first run marked, and reporting those again would make the second run's
answer differ from the first's for no reason an operator could act on.

The order is *the library's own per-level order*, which the comment says and does
not restate: `contents.children()` is the store's, and grove neither sorts nor
reverses it. That matters twice below — once because it is what makes the
integration test's read-only-directory fixture land on the *second* mark, and once
because it is the order `prune_node_marks_every_live_leaf_in_the_subtree` asserts
as a `vec![…]` rather than as a set.

<!-- fragment «outcomes-plan-leaf» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="892-932" parent="outcomes-in-place" -->
````rust
/// One leaf's step, refused here if it cannot be marked at all.
///
/// Every clause is a precondition the library cannot see, and the last of them —
/// that the leaf's key addresses it and nothing else — is what makes the *by key*
/// call the mark is about mean anything at all. Checking it here rather than at
/// the rewrite is what keeps a bulk mark all-or-nothing across entries the
/// library only ever sees one at a time.
fn plan_leaf(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    entry: &Entry<'_, TaskName>,
) -> Result<Planned> {
    let name = entry.name();
    let triple = entry
        .triple()
        .with_context(|| format!("{name} carries no ordinal or key"))?;
    let Parts::Leaf {
        outcome,
        kind,
        slug,
    } = triple.parts
    else {
        bail!(
            "cannot prune a node directory as a leaf (pass the directory itself \
             to prune its subtree): {name}"
        )
    };
    match outcome {
        Outcome::Done => bail!("cannot prune a retired (DONE) leaf: {name}"),
        Outcome::Abandoned => bail!("leaf is already pruned (ABANDONED): {name}"),
        Outcome::Live => {}
    }
    if kind.is_finish() {
        bail!("`finish` is driver-reserved and cannot be pruned");
    }
    Ok(Planned::ToMark {
        key: task_tree::addressable_key(root, snapshot, entry)?,
        parts: Parts::leaf(Outcome::Abandoned, kind.clone(), slug.clone()),
    })
}

````
<!-- /fragment -->

**This is the classification again, and reading it beside `retire_parts` is worth
doing.** The two functions refuse the same three things — a non-leaf, a terminal
outcome, and the driver-reserved `finish` kind — and require the same one, a live
leaf. What differs is three things and no more: a missing triple is a
`with_context` here and a `bail!` there, because a leaf reached through a subtree
walk has already been classified once; the node case is refused with an
instruction rather than a flat *no*, since passing a node **is** the bulk call and
the message says so (*pass the directory itself to prune its subtree*); and this
one ends by looking up a key, which `retire_parts` leaves to its caller.

**The outcome messages are mirrored, and the mirror is exact.** Each verb says
*already* for the outcome it would itself have written and *cannot* for the other
one. `retire` meets a `DONE` leaf as *leaf is already retired (DONE)* and an
`ABANDONED` one as *cannot retire an abandoned (ABANDONED) leaf*; `prune` meets
the same two as *cannot prune a retired (DONE) leaf* and *leaf is already pruned
(ABANDONED)*. That is not decoration: four of this chapter's tests assert on a
substring of exactly these sentences, and two of them pick `"already"` — which
lands on opposite outcomes in the two verbs, and is the reason those two tests
cannot be swapped.

**The last clause is the one the doc comment singles out, and it is the chapter's
strongest connection to chapter 6.** `task_tree::addressable_key` is called on
line 925, and the comment says what it is for: *that the leaf's key addresses it
and nothing else — is what makes the *by key* call the mark is about mean anything
at all*. `rewrite` takes a key. If two entries in the tree carry that key, the
library answers with whichever the walk reaches first, and the mark lands on an
entry nobody chose. Chapter 6 read that function and named this verb as the reason
it exists; this is the call site, and `retiring_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed`
below is the failure it prevents, described by its own test as *success, silently
aimed at the wrong entry*.

And the comment's second clause is the bulk half: *checking it here rather than at
the rewrite is what keeps a bulk mark all-or-nothing across entries the library
only ever sees one at a time*. The check could have sat beside each `rewrite`, and
then the fourth leaf of six would have failed after three had already moved.

<!-- fragment «outcomes-apply-prune» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="933-979" parent="outcomes-in-place" -->
````rust
/// Apply a validated plan, one rewrite per guard.
///
/// The planning guard is spent on the first mark and every later one takes a
/// fresh guard through [`task_tree::reopen_write`] — which re-reads the tree, so
/// each rewrite plans from the state the one before it left, and prints no
/// second waiting diagnostic.
fn apply_prune(
    root: &Path,
    planning_guard: task_tree::Guard,
    plan: Vec<Planned>,
) -> Result<PruneResult> {
    let mut result = PruneResult {
        marked: Vec::new(),
        left_done: Vec::new(),
    };
    let mut held = Some(planning_guard);
    for step in plan {
        match step {
            Planned::LeftDone { path } => result.left_done.push(path),
            Planned::ToMark { key, parts } => {
                let mark = || -> Result<PathBuf> {
                    let tree = match held.take() {
                        Some(tree) => tree,
                        None => task_tree::reopen_write(root)?,
                    };
                    let report = tree.rewrite(key, parts).map_err(task_tree::raised)?;
                    marked_path(&report)
                };
                // **A stopped run names what it already marked, and how to
                // finish.** `docs/adr/bulk-marks-are-not-atomic.md` accepts that
                // a subtree prune is *N* rewrites under *N* guards, and it
                // accepts it on one argument: the marks *are* the state, so
                // re-running converges. That argument is only available to an
                // operator who can see the residue, and a bare store refusal
                // shows none of it — which is principle 2's *an error that only
                // reports detection is unfinished*, in the one place the ADR
                // most needs it not to be.
                match mark() {
                    Ok(path) => result.marked.push(path),
                    Err(error) => return Err(error.context(stopped_partway(&result))),
                }
            }
        }
    }
    Ok(result)
}

````
<!-- /fragment -->

**The guard arithmetic is four lines and it is the whole cost.** `held` starts as
`Some(planning_guard)`; the first `ToMark` step takes it; every later one reaches
`task_tree::reopen_write`. So *N* marks cost *N* guards, of which the first was
already open — and a `LeftDone` step costs none, because it never enters the
closure. That is precisely the arithmetic
`pruning_a_node_takes_one_guard_per_mark` asserts, and it is why the number that
test names is a number rather than a description.

`reopen_write` rather than `write` is the deliberate half. Chapter 5 read the
difference: `write` announces contention before it blocks, `reopen_write` does
not, and the doc comment there says why — *the diagnostic is about the command's
wait, not about each lock it happens to need*. A bulk prune that printed a waiting
line per leaf would be reporting one wait *N* times. The comment here adds the
other consequence, which is not a nicety: the reopen **re-reads the tree**, so each
rewrite plans from the state the one before it left rather than from the snapshot
the run was planned against.

**The long comment on lines 958 to 966 is about an error message, and it is the
one place in this block where grove spends code on an operator rather than on a
tree.** The argument is worth following because it is conditional on the ADR
above it: `bulk-marks-are-not-atomic` accepts *N* guards on the ground that
re-running converges — and *that argument is only available to an operator who can
see the residue*. A bare store refusal shows none of it. So the acceptance in the
record and the context added on line 969 are one decision, not two, and removing
the second would quietly invalidate the first. The comment names the principle it
is applying: *an error that only reports detection is unfinished*.

<!-- fragment «outcomes-stopped-partway» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="980-999" parent="outcomes-in-place" -->
````rust
/// What a stopped `leaf-prune` has already done, and what to do about it.
fn stopped_partway(result: &PruneResult) -> String {
    if result.marked.is_empty() {
        return String::from(
            "leaf-prune marked nothing before it stopped; the subtree is as it was, and rerunning the same command is safe.",
        );
    }
    let marked = result
        .marked
        .iter()
        .map(|path| format!("  {}", path.display()))
        .collect::<Vec<_>>()
        .join("\n");
    let count = result.marked.len();
    let leaves = if count == 1 { "leaf" } else { "leaves" };
    format!(
        "leaf-prune stopped partway: {count} {leaves} were already marked ABANDONED and are still marked —\n{marked}\n\nA mark is the state and an already-abandoned leaf is skipped, so rerun the same `grove-llm leaf-prune` once the cause below is fixed and it will finish the rest."
    )
}

````
<!-- /fragment -->

**Two branches, and they say opposite things to an operator.** With nothing
marked, the subtree is as it was and re-running is safe — the failure happened
during planning or on the very first mark, and there is no residue. With something
marked, the message names every path it already moved, states the invariant that
makes the repair work (*a mark is the state and an already-abandoned leaf is
skipped*), and gives the exact command to re-run. The pluralisation on line 991 is
the small tell that the second branch expects to be read by a person.

<!-- fragment «outcomes-marked-path» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1000-1015" parent="outcomes-in-place" -->
````rust
/// Where a mark left the entry, out of the library's own report.
///
/// A `rewrite` is exactly one rename and reports it whether or not the
/// filesystem was touched, so the first entry is the answer and an empty report
/// is a contract the library broke rather than a case to handle quietly.
fn marked_path(report: &Report<TaskName>) -> Result<PathBuf> {
    report
        .renamed()
        .first()
        .map(|renamed| renamed.to.clone())
        .context("the library reported no rename for a mark")
}

// ---------------------------------------------------------------------------
// helpers

````
<!-- /fragment -->

**The last helper is seven lines and the comment above it is the argument for
all of them.** A `rewrite` is exactly one rename and reports it whether or not the
filesystem was touched — so the first entry of the report *is* the answer, and an
empty report is not a case to handle quietly but a contract the library broke.
That is why the tail is `.context(…)` rather than a fallback: there is no sensible
value to return, and inventing one would hide a broken store behind a plausible
path. Both verbs end here, which is why the function is named for the mark rather
than for either of them.

**The block ends on a section divider that belongs to the next chapter's
material.** Lines 1010 to 1012 are the `// helpers` rule, and the three body-writing
helpers under it — `grove_name`, `root_brief_body` and `append_brief_suffix_in_file`
— are chapter 11's block, starting at line 1013. The divider is inside this
chapter's range because the ownership boundary was drawn at a function edge rather
than at a comment; a reader following the file will meet the heading here and the
functions it heads two pages earlier in the book.

<a id="the-tests-and-what-each-would-pass-under"></a>
## What the thirty-two tests establish, and what each would pass under

The chapter's second ownership block is lines 2235 to 2732 — 491 lines, and the
third largest inline-test block in the book behind chapter 12's 569 and chapter
17's 564. It carries **thirty-two** `#[test]` functions in four labelled sections
and no helpers of its own: ten in *leaf-retire*, three in *lifecycle over
untracked leaves*, nine in *leaf-prune*, and ten in *leaf-prune on a node: bulk
arity*.

The block's prose obligation is *supply the claim*: for each reproduced test, the
property it establishes **and what would have to be true for it to pass while the
property was broken**. The second half is the part a reviewer can check and the
test cannot state.

**Two of the four section labels do not predict the chapter a test belongs to, and
one does not predict the verb.** The sections are the file's, not the book's:
*lifecycle over untracked leaves* holds a `leaf_decompose` test that is chapter
12's verb sitting inside chapter 13's block, and *leaf-prune on a node: bulk
arity* holds `retiring_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed`,
which is a single-leaf retire test placed there because it is the other half of an
argument the bulk tests make. Both belong to this chapter, and they belong to it
for one reason: their line numbers fall inside `retire-and-prune-tests`. Ownership
is the manifest's block range and never the verb a test happens to exercise.

**The support is chapter 11's, and this block calls eleven of its eighteen
items** — `jj_grove`, `commit_all`, `grow_leaf`, `a_slug`, `guard`, `touch`,
`touch_body`, `mknode`, `name_of`, `body` and `list`. Of the seven it does not,
`run_jj` and `open` are still reached, from inside the support block itself;
`a_kind`, `worktree`, `refusal_for_an_absent_root`, `guard_at` and `root_init_at`
are named nowhere in these 491 lines.

Chapter 12's block called twelve, and the difference between the two lists is
three items rather than one. This block drops `a_kind`, because neither mark takes
a kind — the new `Parts` is built from the entry's own — and drops
`refusal_for_an_absent_root`, because no test here is about opening a root that
holds no tree. What it adds is `grow_leaf`, and that one item is what makes the
untracked-leaf section possible: it grows a leaf through the real verb and leaves
it uncommitted, which is the state a mid-session grove is actually in.

<!-- fragment «retire-and-prune-tests» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2242-2732" parent="source-tree-lifecycle" -->
<!-- insert «retire-tests-opening» -->
<!-- insert «retire-tests-body-untouched» -->
<!-- insert «retire-tests-nested» -->
<!-- insert «retire-tests-refusals» -->
<!-- insert «retire-tests-absolute» -->
<!-- insert «untracked-tests-opening» -->
<!-- insert «untracked-tests-decompose-and-prune» -->
<!-- insert «prune-leaf-tests-opening» -->
<!-- insert «prune-leaf-tests-body-and-nested» -->
<!-- insert «prune-leaf-tests-refusals» -->
<!-- insert «prune-leaf-tests-absolute» -->
<!-- insert «prune-node-tests-opening» -->
<!-- insert «prune-node-tests-done-untouched» -->
<!-- insert «prune-node-tests-grandchild» -->
<!-- insert «prune-node-tests-mixed-tracking» -->
<!-- insert «prune-node-tests-atomic» -->
<!-- insert «prune-node-tests-twin» -->
<!-- insert «prune-node-tests-guard-count» -->
<!-- insert «prune-node-tests-nothing-live» -->
<!-- insert «prune-node-tests-root-refusals» -->
<!-- /fragment -->

<a id="the-pair-that-makes-in-place-mean-something"></a>
### The pair that makes *in place* mean something

The first section is ten tests over `leaf-retire`, and it opens on the two the
structure brief names as the chapter's pair.

<!-- fragment «retire-tests-opening» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2242-2259" parent="retire-and-prune-tests" -->
````rust
    // ---- leaf-retire --------------------------------------------------------

    #[test]
    fn retire_adds_done_infix_keeping_position_and_key() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let done = leaf_retire(guard(&g), Path::new("02-impl--add-k4.md")).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl--add-k4.md");
        let files = list(&g);
        assert!(files.contains(&"02-DONE-impl--add-k4.md".to_string()));
        assert!(
            !files.contains(&"02-impl--add-k4.md".to_string()),
            "old name gone"
        );
    }

````
<!-- /fragment -->

**`retire_adds_done_infix_keeping_position_and_key`** — the chapter's rule, on
disk. `02-impl--add-k4.md` becomes `02-DONE-impl--add-k4.md`: the position `02`,
the kind `impl`, the slug `add` and the key `4` all survive, and the only change is
an inserted `DONE-`. Three assertions, and they are chosen so that no two are
satisfied by the same accident: `name_of(&done)` pins the path the verb
**returned**, and the two `list` assertions pin both directions of the rename on
disk — the new name present and the old one absent.

It would pass while the property was broken if the body had been rewritten, and
nothing here reads a byte of content. It would also pass if the verb had produced
the right name by *composing* one rather than by preserving parts: `02-DONE-impl--add-k4.md`
is a single string, and *the key was kept* and *a key of 4 was recomputed and
happened to match* are the same observation here. Nothing in this block separates
them — the separation is `addressable_key`'s, and the twin test below is where it
shows.

<!-- fragment «retire-tests-body-untouched» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2260-2269" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn retire_does_not_rewrite_the_header_or_body() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-impl--add-k4.md", "# add-k4\n\nbody\n");
        commit_all(&g);
        let done = leaf_retire(guard(&g), Path::new("02-impl--add-k4.md")).unwrap();
        assert_eq!(body(&done), "# add-k4\n\nbody\n", "content byte-identical");
    }

````
<!-- /fragment -->

**`retire_does_not_rewrite_the_header_or_body`** — the other half, and the reason
the two are a pair. `touch_body` writes `"# add-k4\n\nbody\n"`, and the assertion
is `assert_eq!` on the whole string rather than a `contains`, so a rewritten
header, a normalised trailing newline or an appended status line all fail it. This
is the test that makes the chapter's rule falsifiable: without it, an
implementation that stamped `status: done` into the file would satisfy every other
test on this page.

It would pass while the property was broken if the *filename* were wrong, because
it never looks at one — it reads the body at whatever path the verb returned. The
two tests are complementary in exactly that way, and each is weak precisely where
the other is strong.

<!-- fragment «retire-tests-nested» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2270-2281" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn retire_works_on_a_nested_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let done = leaf_retire(guard(&g), &design.join("02-impl--add-k4.md")).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl--add-k4.md");
        assert_eq!(name_of(done.parent().unwrap()), "01-design-k1");
    }

````
<!-- /fragment -->

**`retire_works_on_a_nested_leaf`** — the mark reaches inside a node, and the node
is untouched. `mknode` builds `01-design-k1` with a `BRIEF.md`; the leaf inside it
is retired; and the second assertion pins the parent directory's name, which is
what says the node did not get marked, renumbered or promoted on the way past.

It would pass while the property was broken if the verb had marked the *right*
leaf for the wrong reason — the tree holds one leaf, so *found it by path* and
*found the only markable entry* are indistinguishable here.

<!-- fragment «retire-tests-refusals» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2282-2338" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn retire_refuses_a_node_directory() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_retire(guard(&g), &node).unwrap_err();
        assert!(err.to_string().contains("node"), "got {err}");
    }

    #[test]
    fn retire_refuses_a_node_brief() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_retire(guard(&g), &node.join("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn retire_refuses_the_root_brief() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        commit_all(&g);
        let err = leaf_retire(guard(&g), Path::new("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn retire_refuses_an_already_done_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "02-DONE-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let err = leaf_retire(guard(&g), Path::new("02-DONE-impl--add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("already"), "got {err}");
    }

    #[test]
    fn retire_refuses_an_abandoned_leaf() {
        // A missing flag must degrade to something harmless, never to the
        // opposite outcome (pruning): retiring an abandoned leaf would
        // silently assert the rejected work was finished.
        let (_t, g) = jj_grove();
        touch(&g, "02-ABANDONED-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let err = leaf_retire(guard(&g), Path::new("02-ABANDONED-impl--add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("abandoned"), "got {err}");
    }

    #[test]
    fn retire_refuses_a_foreign_file() {
        let (_t, g) = jj_grove();
        touch(&g, "README.md", "readme");
        commit_all(&g);
        let err = leaf_retire(guard(&g), Path::new("README.md")).unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

````
<!-- /fragment -->

**Six refusals, and each asserts on a substring rather than on a whole message.**
That is the deliberate weakness of the group and it is worth stating plainly:
`contains("node")`, `contains("brief")`, `contains("already")`,
`contains("abandoned")` and `contains("leaf")` are each satisfied by more than one
of this module's sentences, so a test in this group establishes *a refusal
happened and mentioned the right word*, not *the arm I am named for fired*. The
measured section below is what closes the gap, and it closes it for these six by
mutation rather than by reading.

- **`retire_refuses_a_node_directory`** — a node handed in directly. Establishes
  that a directory entry is refused before any rewrite. It would pass while broken
  if the refusal came from somewhere other than `retire_parts`'s `Parts::Node` arm;
  `contains("node")` also matches *cannot prune a node directory as a leaf*, which
  is a different function, and matches nothing else in this module.
- **`retire_refuses_a_node_brief`** and **`retire_refuses_the_root_brief`** — the
  same arm reached by two paths, a node's charter and the grove root's. Both
  establish that a charter is never marked. Together they establish something
  neither does alone: that the refusal is about brief-ness rather than about
  depth, since the two briefs sit at different levels and produce the same word.
- **`retire_refuses_an_already_done_leaf`** — a second retire is refused rather
  than being idempotent. It would pass while broken if the verb had refused for
  the wrong reason entirely; `contains("already")` is the mirrored wording from
  the section above, and in *this* verb it lands on `DONE`.
- **`retire_refuses_an_abandoned_leaf`** — and this one carries its own argument
  in a comment, which is unusual in this block and earns it. *A missing flag must
  degrade to something harmless, never to the opposite outcome*: retiring an
  abandoned leaf would silently assert that rejected work was finished. The test
  is not about tidiness; it is about which direction an error is allowed to fall.
- **`retire_refuses_a_foreign_file`** — `README.md` in the grove root. This one is
  not `retire_parts`'s at all: a foreign name never becomes an `Entry`, so the
  refusal comes from `task_tree::target`, which is chapter 6's. `contains("leaf")`
  is what the assertion can say without reaching into another chapter's wording.

<!-- fragment «retire-tests-absolute» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2339-2348" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn retire_accepts_an_absolute_path() {
        let (_t, g) = jj_grove();
        touch(&g, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let abs = g.join("02-impl--add-k4.md");
        let done = leaf_retire(guard(&g), &abs).unwrap();
        assert_eq!(name_of(&done), "02-DONE-impl--add-k4.md");
    }

````
<!-- /fragment -->

**`retire_accepts_an_absolute_path`** — the reference grammar's path arm accepts
both spellings, and every other test in this section uses the relative one. It
establishes that the absolute form resolves to the same entry. It would pass while
broken if relative paths were the ones mishandled, which is what the rest of the
section covers by using them exclusively.

<a id="the-defect-the-fixtures-remember"></a>
### Three tests that outlived the defect they were written for

The second section is three tests, one per lifecycle verb, and it is the only one
in the block whose label names a *bug* rather than a verb.

<!-- fragment «untracked-tests-opening» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2349-2374" parent="retire-and-prune-tests" -->
````rust
    // ---- lifecycle over untracked leaves (issue #3's root cause) -------------
    //
    // Issue #3's defect, in the lifecycle verbs: a leaf grown this session is
    // uncommitted until the enclosing task commits, and the version-control-aware
    // move these verbs used to reach for had nothing recorded to move. The verbs
    // now rename through `ordinal-fs-tree`, which consults no repository at all
    // (`docs/adr/grove-does-not-stage-its-own-renames.md`), so these cases can no
    // longer fail that way — they are kept because the fixtures are the ones a
    // real session produces, and a verb that grew a tracked-only path would fail
    // them again.

    #[test]
    fn retire_an_untracked_leaf_added_this_session() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "ship");
        // No commit_all: the grow verb leaves it uncommitted, by design.
        let done = leaf_retire(guard(&g), &leaf).unwrap();
        assert_eq!(name_of(&done), "01-DONE-impl--ship-k1.md");
        assert!(
            done.is_file(),
            "the retired leaf is on disk under its DONE name"
        );
        assert!(!leaf.exists(), "the live name is gone");
    }

````
<!-- /fragment -->

**The section comment is an adjudication the source makes about itself**, and it
is the reason these three tests are not redundant. Issue #3's defect was that a
leaf grown this session is uncommitted until the enclosing task commits, and the
version-control-aware move the lifecycle verbs used to reach for had nothing
recorded to move. The verbs now rename through `ordinal-fs-tree`, which consults
no repository at all, so **these cases can no longer fail that way**. The comment
says exactly that, and then says why they are kept: the fixtures are the ones a
real session produces, and *a verb that grew a tracked-only path would fail them
again*.

That is a test suite recording its own history honestly, and it is worth reading
as such. The three tests do not establish that the bug is fixed — the fix was
architectural and is held by the ADR and by the absence of any repository call in
this module. What they establish is a **regression boundary**: the shape of tree a
session actually has mid-task, exercised by all three lifecycle verbs.

**`retire_an_untracked_leaf_added_this_session`** — `grow_leaf` creates
`01-impl--ship-k1.md` through the real grow verb and the test deliberately does
not commit. Establishes that retirement does not require trackedness. It would
pass while the property was broken if `grow_leaf` had committed, which is why the
comment on line 2358 says *no commit_all* rather than leaving the absence silent —
the assertion depends on something not happening, and a reader cannot see that
from the code alone.

<!-- fragment «untracked-tests-decompose-and-prune» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2375-2401" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn decompose_an_untracked_leaf_added_this_session() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "big");
        // "The current item proving bigger" — the canonical mid-session decompose.
        let (brief, child) = leaf_decompose(guard(&g), &leaf, &a_slug("first"), None).unwrap();
        assert_eq!(name_of(&brief), "BRIEF.md");
        assert_eq!(name_of(&child), "01-impl--first-k2.md");
        assert!(g.join("01-big-k1").is_dir(), "the leaf became a node dir");
        assert!(
            !leaf.exists(),
            "the leaf file is gone (it became the BRIEF)"
        );
    }

    #[test]
    fn prune_an_untracked_leaf_added_this_session() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = grow_leaf(&g, "dead");
        let result = leaf_prune(guard(&g), &leaf).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "01-ABANDONED-impl--dead-k1.md");
        assert!(!leaf.exists(), "the live name is gone");
    }

````
<!-- /fragment -->

**`decompose_an_untracked_leaf_added_this_session`** — chapter 12's verb, and this
chapter's block. It establishes the same trackedness-independence for decompose,
and its own comment names the case as *the canonical mid-session decompose*: the
current item proving bigger, which is the trigger the methodology names. The four
assertions cover both halves of the promotion, the node directory and the vanished
leaf file. It would pass while the property was broken if the promotion had
carried the body wrongly — nothing here reads the brief's content, which is
chapter 12's own `decompose_seeds_brief_from_leaf_body_and_appends_brief_suffix`.

**`prune_an_untracked_leaf_added_this_session`** — the same for the mark that
abandons. `result.marked.len() == 1` and the name is
`01-ABANDONED-impl--dead-k1.md`, so this is also the shortest statement in the
block that a single-leaf prune returns a one-element `marked` and touches nothing
else. It would pass while the property was broken if `left_done` were being
populated wrongly, which it does not assert; the leaf-section test below does.

<a id="the-same-mark-the-other-word"></a>
### The same mark, the other word

The third section is nine tests over `leaf-prune` given a leaf, and read against
the first section it is very nearly the same list — which is the point.

<!-- fragment «prune-leaf-tests-opening» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2402-2421" parent="retire-and-prune-tests" -->
````rust
    // ---- leaf-prune (pruning) ------------------------------------------

    #[test]
    fn prune_leaf_adds_abandoned_infix_keeping_position_and_key() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let result = leaf_prune(guard(&g), Path::new("02-impl--add-k4.md")).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl--add-k4.md");
        assert!(result.left_done.is_empty());
        let files = list(&g);
        assert!(files.contains(&"02-ABANDONED-impl--add-k4.md".to_string()));
        assert!(
            !files.contains(&"02-impl--add-k4.md".to_string()),
            "old name gone"
        );
    }

````
<!-- /fragment -->

**`prune_leaf_adds_abandoned_infix_keeping_position_and_key`** — the retire
opening test's twin, with two assertions the twin does not make.
`result.marked.len() == 1` and `result.left_done.is_empty()` are about the
`PruneResult` rather than about the disk, and together they establish the arity
claim the type's own comment makes: *a single-leaf call marks exactly one entry
and finds nothing to leave alone*.

It would pass while the property was broken in the same way its twin would — a
composed rather than preserved key is indistinguishable here — and additionally if
`left_done` were only ever empty for single-leaf calls by accident rather than by
the walk never running. `prune_node_leaves_done_leaves_untouched` is what makes
`left_done` a field that can be non-empty at all.

<!-- fragment «prune-leaf-tests-body-and-nested» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2422-2447" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_leaf_does_not_rewrite_the_header_or_body() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch_body(&g, "02-impl--add-k4.md", "# add-k4\n\nbody\n");
        commit_all(&g);
        let result = leaf_prune(guard(&g), Path::new("02-impl--add-k4.md")).unwrap();
        assert_eq!(
            body(&result.marked[0]),
            "# add-k4\n\nbody\n",
            "content byte-identical"
        );
    }

    #[test]
    fn prune_leaf_works_on_a_nested_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &design.join("02-impl--add-k4.md")).unwrap();
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl--add-k4.md");
        assert_eq!(name_of(result.marked[0].parent().unwrap()), "01-design-k1");
    }

````
<!-- /fragment -->

**`prune_leaf_does_not_rewrite_the_header_or_body`** and
**`prune_leaf_works_on_a_nested_leaf`** — the same two properties the retire
section established, re-established for the other verb rather than assumed from
it. That is not redundancy: the two verbs reach `rewrite` down different paths —
`retire_parts` for one, `plan_leaf` and `apply_prune` for the other — and a
body-preservation property that held on the short path could fail on the long one.
Each would pass while broken in its twin's way: the body test reads no filename,
the nested test reads no content.

<!-- fragment «prune-leaf-tests-refusals» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2448-2492" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_leaf_refuses_a_node_brief() {
        let (_t, g) = jj_grove();
        let node = mknode(&g, "02-build-k3", "build-k3");
        commit_all(&g);
        let err = leaf_prune(guard(&g), &node.join("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_the_root_brief() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new("BRIEF.md")).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_an_already_done_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "02-DONE-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new("02-DONE-impl--add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("DONE"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_an_already_abandoned_leaf() {
        let (_t, g) = jj_grove();
        touch(&g, "02-ABANDONED-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new("02-ABANDONED-impl--add-k4.md")).unwrap_err();
        assert!(err.to_string().contains("already"), "got {err}");
    }

    #[test]
    fn prune_leaf_refuses_a_foreign_file() {
        let (_t, g) = jj_grove();
        touch(&g, "README.md", "readme");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new("README.md")).unwrap_err();
        assert!(err.to_string().contains("leaf"), "got {err}");
    }

````
<!-- /fragment -->

**Five refusals, and the set is the retire section's minus one.** There is no
`prune_leaf_refuses_a_node_directory`, and there cannot be: handing a node to
`leaf-prune` is the bulk call rather than an error, which is the arity asymmetry
stated as a missing test.

- **`prune_leaf_refuses_a_node_brief`** and **`prune_leaf_refuses_the_root_brief`**
  — `plan_prune`'s brief arm, reached at two depths. Same claim as the retire pair,
  and the same weakness: `contains("brief")` matches both this module's brief
  refusals.
- **`prune_leaf_refuses_an_already_done_leaf`** — and here the assertion is
  `contains("DONE")` rather than `contains("already")`, which is the mirroring from
  the production section showing up in the fixtures. It would pass while broken if
  any refusal in the chain happened to name `DONE`; the only other sentence that
  does is *cannot retire an abandoned (ABANDONED) leaf*, which this verb cannot
  produce.
- **`prune_leaf_refuses_an_already_abandoned_leaf`** — the idempotence boundary for
  the single-leaf call. It is worth holding this one against `plan_subtree`'s
  silent skip: an already-`ABANDONED` leaf named **directly** is refused, and the
  same leaf found **inside a subtree** is stepped over without a word. That is not
  an inconsistency, and the reason is the re-runnability argument — a bulk re-run
  must be a no-op, and a direct call is a person naming one leaf and getting the
  wrong answer if it succeeds silently. No test asserts the two behaviours
  together; the pairing is visible only by reading them side by side.
- **`prune_leaf_refuses_a_foreign_file`** — chapter 6's refusal again, same as its
  retire twin.

<!-- fragment «prune-leaf-tests-absolute» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2493-2502" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_leaf_accepts_an_absolute_path() {
        let (_t, g) = jj_grove();
        touch(&g, "02-impl--add-k4.md", "add-k4");
        commit_all(&g);
        let abs = g.join("02-impl--add-k4.md");
        let result = leaf_prune(guard(&g), &abs).unwrap();
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl--add-k4.md");
    }

````
<!-- /fragment -->

**`prune_leaf_accepts_an_absolute_path`** — the retire twin, and the point at
which the two verbs' single-leaf sections have been shown to be the same verb with
a different word. Everything after this is the arity.

<a id="the-arity"></a>
### The arity: ten tests about a mark that is many marks

The last section is ten tests over `leaf-prune` given a node, and it is where the
chapter's cost stops being described and starts being asserted.

<!-- fragment «prune-node-tests-opening» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2503-2521" parent="retire-and-prune-tests" -->
````rust
    // ---- leaf-prune on a node: bulk arity (pruning) -------------------

    #[test]
    fn prune_node_marks_every_live_leaf_in_the_subtree() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &node).unwrap();
        let names: Vec<String> = result.marked.iter().map(|p| name_of(p)).collect();
        assert_eq!(
            names,
            vec!["01-ABANDONED-impl--a-k3.md", "02-ABANDONED-impl--b-k4.md"]
        );
        assert!(result.left_done.is_empty());
    }

````
<!-- /fragment -->

**`prune_node_marks_every_live_leaf_in_the_subtree`** — the bulk claim at its
smallest. Two live leaves under a node; both marked; `left_done` empty. The
assertion is `assert_eq!` against a `vec![…]` rather than a set membership check,
so it pins the **order** as well as the set — `01` before `02`, which is
`plan_subtree` walking `contents.children()` in the library's own per-level order
and grove neither sorting nor reversing it.

It would pass while the property was broken if the recursion were absent, because
this tree is one level deep; `prune_node_recurses_into_a_grandchild_node` is what
separates *marks the children* from *marks the subtree*. It would also pass if the
node's own `BRIEF.md` were being silently mishandled rather than skipped — nothing
here asserts the brief survived.

<!-- fragment «prune-node-tests-done-untouched» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2522-2540" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_node_leaves_done_leaves_untouched() {
        // That work really was done — a bulk abandon does not retroactively
        // un-finish it.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &node).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "02-ABANDONED-impl--b-k4.md");
        assert_eq!(result.left_done.len(), 1);
        assert_eq!(name_of(&result.left_done[0]), "01-DONE-impl--a-k3.md");
        // The DONE leaf's name (and so its position and key) is untouched.
        assert!(node.join("01-DONE-impl--a-k3.md").is_file());
    }

````
<!-- /fragment -->

**`prune_node_leaves_done_leaves_untouched`** — the second vector earns its
existence. One `DONE` leaf and one live leaf; the live one is marked, the `DONE`
one is *reported* in `left_done` and its file is still there under its original
name. The comment states the reason as a claim about meaning rather than about
mechanism: *that work really was done — a bulk abandon does not retroactively
un-finish it*.

Four assertions, and the fourth is the one that makes it more than a pair of
counts: `node.join("01-DONE-impl--a-k3.md").is_file()` reads the disk, so a
version that reported the `DONE` leaf in `left_done` *and also marked it* would
fail. Without that line the test would pass while the property was broken, because
`left_done` is a value the function constructs and could construct honestly while
doing the wrong thing beside it.

<!-- fragment «prune-node-tests-grandchild» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2541-2558" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_node_recurses_into_a_grandchild_node() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let outer = mknode(&g, "01-outer-k1", "outer-k1");
        let inner = mknode(&outer, "01-inner-k2", "inner-k2");
        touch(&inner, "01-impl--deep-k3.md", "deep-k3");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &outer).unwrap();
        assert_eq!(result.marked.len(), 1);
        assert_eq!(name_of(&result.marked[0]), "01-ABANDONED-impl--deep-k3.md");
        assert_eq!(
            name_of(result.marked[0].parent().unwrap()),
            "01-inner-k2",
            "the grandchild's own directory is untouched — only the leaf file is marked"
        );
    }

````
<!-- /fragment -->

**`prune_node_recurses_into_a_grandchild_node`** — the recursion, and the limit of
it. A leaf two levels down is marked; the assertion with the message attached is
the interesting one: *the grandchild's own directory is untouched — only the leaf
file is marked*. So the walk descends into nodes and marks **leaves**, and a node
in the path is never itself given an outcome — which is chapter 3's `Parts::Node`
having no outcome field at all, showing up as a behaviour.

It would pass while the property was broken if the depth limit were three rather
than unbounded; the fixture is exactly two levels, and nothing in the block tests
deeper. That is a real gap in the block rather than a criticism of the test: the
recursion is structural and a third level would exercise the same line.

<!-- fragment «prune-node-tests-mixed-tracking» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2559-2588" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_node_marks_a_subtree_mixing_tracked_and_untracked_leaves() {
        // The bulk analogue of `prune_an_untracked_leaf_added_this_session`: one
        // decision kills a subtree whose leaves were grown across several sessions,
        // so some are committed and some are still working-tree-only. Every live
        // leaf is marked regardless — trackedness is not a precondition of a rename.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        commit_all(&g); // a and b are tracked
        touch(&node, "03-impl--c-k5.md", "c-k5"); // c is not

        let result = leaf_prune(guard(&g), &node).unwrap();

        assert_eq!(result.marked.len(), 3, "every live leaf marked");
        let names = list(&node);
        for expected in [
            "01-ABANDONED-impl--a-k3.md",
            "02-ABANDONED-impl--b-k4.md",
            "03-ABANDONED-impl--c-k5.md",
        ] {
            assert!(
                names.contains(&expected.to_string()),
                "missing {expected} (names: {names:?})"
            );
        }
    }

````
<!-- /fragment -->

**`prune_node_marks_a_subtree_mixing_tracked_and_untracked_leaves`** — the bulk
analogue of the untracked-leaf section, and its comment says what the scenario
*is* rather than what it does: one decision kills a subtree whose leaves were
grown across several sessions, so some are committed and some are working-tree
only. The last clause is the claim — *trackedness is not a precondition of a
rename* — and it is the same architectural fact the section comment above
adjudicated, asserted here at the arity where it would have hurt most.

The assertion loop reads `list(&node)` once and checks all three expected names
are present, which makes the failure message useful. It would pass while the
property was broken if the untracked leaf were marked but the *result* omitted it
— `result.marked.len() == 3` guards that, and the two assertions together are what
make the test about both the disk and the answer.

<!-- fragment «prune-node-tests-atomic» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2589-2642" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address() {
        // pruning: a failure partway through the subtree walk must not leave
        // earlier leaves already marked while the operator sees only the
        // trailing error. Planning and validating the whole subtree against the
        // first guard's snapshot is what prevents it, and the repro is a botched
        // earlier prune — an `ABANDONED` twin sitting beside a live leaf.
        //
        // **The property survived the flip and the diagnosis changed.** The twin
        // wears the very name the mark would place, so it necessarily carries
        // the same key: an outcome infix is part of the name and the key is part
        // of the name, and a name that collides collides in both. So what is
        // wrong with this tree is not that a destination is taken but that key 5
        // names two entries — and `rewrite` is called *by key*, which on this
        // tree means Grove cannot say which leaf it would mark. Refusing that is
        // strictly prior, and it is Grove's own precondition rather than a
        // second wording of the library's `DestinationOccupied`.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        touch(&node, "03-impl--c-k5.md", "c-k5");
        touch(&node, "03-ABANDONED-impl--c-k5.md", "c-k5");
        commit_all(&g);

        let err = leaf_prune(guard(&g), &node).unwrap_err();
        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 5"),
            "got {err}"
        );

        // Nothing was mutated: every live name is untouched, none newly marked.
        let names = list(&node);
        assert!(
            names.contains(&"01-impl--a-k3.md".to_string()),
            "got {names:?}"
        );
        assert!(
            names.contains(&"02-impl--b-k4.md".to_string()),
            "got {names:?}"
        );
        assert!(
            names.contains(&"03-impl--c-k5.md".to_string()),
            "got {names:?}"
        );
        assert!(
            !names.contains(&"01-ABANDONED-impl--a-k3.md".to_string())
                && !names.contains(&"02-ABANDONED-impl--b-k4.md".to_string()),
            "a validation failure must leave the whole subtree untouched: {names:?}"
        );
    }

````
<!-- /fragment -->

**`prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address`** — the
all-or-nothing promise, and the longest comment in the block because the
diagnosis changed under it.

The property is the one `plan_prune`'s doc comment claims: a failure partway
through the subtree walk must not leave earlier leaves marked while the operator
sees only the trailing error. The repro is a botched earlier prune — an
`ABANDONED` twin sitting beside a live leaf at position `03`, both carrying key 5.
Four assertions after the refusal check that `01`, `02` and `03` are all still
live and that neither of the two leaves the walk would have reached *first* was
marked, which is the actual all-or-nothing claim: the failing entry is third, so a
leaf-by-leaf implementation would have marked two before finding out.

**The comment's second paragraph is an adjudication and it should be read as
one.** The property survived a change and the *diagnosis* did not. The twin wears
the very name the mark would place, so it necessarily carries the same key — an
outcome infix is part of the name and the key is part of the name, and a name that
collides on both. So what is wrong with this tree is not a taken
destination but that **key 5 names two entries**, and `rewrite` is called by key,
which on this tree means grove cannot say which leaf it would mark. Refusing that
is strictly prior, and it is grove's own precondition rather than a second wording
of the library's `DestinationOccupied` — which is clause 3 of
`docs/ARCHITECTURE.md#library-refusals` being obeyed rather than described. The
assertion is `contains("two entries in this tree carry key 5")`, chapter 6's
sentence, and it is the one assertion in this block precise enough to name the arm
it landed on.

It would pass while the property was broken if the refusal happened *before* the
walk for an unrelated reason — the fixture's tree is unusual enough that a verb
refusing all hand-edited trees outright would satisfy every line of it.

<!-- fragment «prune-node-tests-twin» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2643-2669" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn retiring_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed() {
        // The single-leaf half, and the failure it prevents is worse than an
        // error. `rewrite` is called by key; with two entries under key 1,
        // `by_key` answers with whichever the walk reaches first — an order
        // nothing models — so retiring the live leaf by *path* rewrote the DONE
        // twin onto its own name, changed nothing, and reported the twin's path
        // as the retired one. Success, silently aimed at the wrong entry.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl--a-k1.md", "a-k1");
        touch(&g, "01-DONE-impl--a-k1.md", "a-k1");
        commit_all(&g);

        let err = leaf_retire(guard(&g), Path::new("01-impl--a-k1.md")).unwrap_err();

        assert!(
            err.to_string()
                .contains("two entries in this tree carry key 1"),
            "got {err}"
        );
        assert!(
            g.join("01-impl--a-k1.md").is_file(),
            "a refused mark changes nothing"
        );
    }

````
<!-- /fragment -->

**`retiring_a_leaf_whose_key_names_a_twin_is_refused_rather_than_misaimed`** — a
`leaf-retire` test in the bulk section, and it is there because it is the other
half of one argument rather than because of the verb it calls.

Its comment states a failure worse than an error, and states it as history: with
two entries under key 1, `by_key` answers with whichever the walk reaches first —
*an order nothing models* — so retiring the live leaf **by path** rewrote the
`DONE` twin onto its own name, changed nothing, and reported the twin's path as
the retired one. **Success, silently aimed at the wrong entry.** That is the
failure `addressable_key` exists to prevent, and this test is the only place in
the chapter where the consequence of not having it is written down.

Two assertions: the refusal names key 1, and the live leaf is still on disk. It
would pass while the property was broken if the refusal came from the *twin*
detection at a different layer — but there is only one twin scan in the workspace,
which is what the closing section says makes the whole `DestinationOccupied` row
rest on one line of grove's code.

<!-- fragment «prune-node-tests-guard-count» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2670-2699" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn pruning_a_node_takes_one_guard_per_mark() {
        // `rewrite` consumes its guard, so a subtree of N live leaves is N
        // observations of the tree where it used to be one critical section.
        // That is the whole of what Grove accepted in
        // `docs/adr/bulk-marks-are-not-atomic.md`, and it is asserted rather
        // than described: a later leaf that restores atomicity, or that adds a
        // re-read nobody meant to add, moves this number.
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl--a-k3.md", "a-k3");
        touch(&node, "02-impl--b-k4.md", "b-k4");
        touch(&node, "03-DONE-impl--c-k5.md", "c-k5");
        touch(&node, "04-impl--d-k6.md", "d-k6");
        commit_all(&g);
        crate::task_tree::reset_read_count();

        let result = leaf_prune(guard(&g), &node).unwrap();

        assert_eq!(result.marked.len(), 3);
        assert_eq!(result.left_done.len(), 1);
        assert_eq!(
            crate::task_tree::read_count(),
            3,
            "one guard per mark — the planning guard is spent on the first, and \
             a leaf left alone costs none"
        );
    }

````
<!-- /fragment -->

**`pruning_a_node_takes_one_guard_per_mark`** — the cost, asserted as a number.
This is the test `docs/adr/bulk-marks-are-not-atomic.md` names, and the ADR says
why it is a number: *so the cost is a number a later change moves rather than a
paragraph it can quietly contradict*.

The fixture is three live leaves and one `DONE` leaf under a node. After
`reset_read_count`, the run produces `read_count() == 3`, and the assertion's own
message states the arithmetic: *one guard per mark — the planning guard is spent
on the first, and a leaf left alone costs none.*

**The number counts openings, and one of the three happened before the verb was
called.** `reset_read_count()` runs as its own statement; `guard(&g)` is evaluated
next, as the call's first argument, and it reaches `task_tree::write` →
`open_write`, which is one of the two places `READ_COUNT` is incremented. Inside
`leaf_prune`, `apply_prune` takes the planning guard for the first mark and calls
`reopen_write` for the second and third. So the three are: the test's own opening,
then two reopens. The `DONE` leaf contributes none because a `LeftDone` step never
enters the closure, and `result.left_done.len() == 1` is what proves the walk
actually saw it rather than skipping it silently.

It would pass while the property was broken if the counter were being incremented
somewhere other than an opening — `READ_COUNT` is bumped in exactly two places,
`read_or_vacant` and `open_write`, and this run touches only the second. And it
would pass while the *atomicity* claim was broken, which is the point: the number
being 3 rather than 1 is the cost the chapter's design **accepts**, so this test
moving is a signal in both directions. A later leaf that restored one critical
section would move it down; one that added a re-read nobody meant to add would
move it up.

<!-- fragment «prune-node-tests-nothing-live» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2700-2711" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_node_with_nothing_live_marks_nothing() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-impl--a-k3.md", "a-k3");
        commit_all(&g);
        let result = leaf_prune(guard(&g), &node).unwrap();
        assert!(result.marked.is_empty());
        assert_eq!(result.left_done.len(), 1);
    }

````
<!-- /fragment -->

**`prune_node_with_nothing_live_marks_nothing`** — the empty case, and the one
test in the block whose value is entirely in what it does **not** do. A node
holding one `DONE` leaf: `marked` is empty and `left_done` has one entry. No guard
beyond the planning one is ever spent, and no rename happens.

It would pass while the property was broken if the run had errored and the test
had asserted on the error — it does not; `unwrap()` is the assertion that an
all-terminal subtree is a **success** rather than a refusal. That is the
re-runnability argument's base case: the second run of a completed prune must
succeed and do nothing, and this is the tree the second run sees.

<!-- fragment «prune-node-tests-root-refusals» owner="marked-in-place" source="crates/grove-loop/src/tree_lifecycle.rs" lines="2712-2732" parent="retire-and-prune-tests" -->
````rust
    #[test]
    fn prune_refuses_the_grove_root() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl--a-k1.md", "a-k1");
        commit_all(&g);
        let err = leaf_prune(guard(&g), &g).unwrap_err();
        assert!(err.to_string().contains("grove root"), "got {err}");
        // Nothing was touched.
        assert!(g.join("01-impl--a-k1.md").is_file());
    }

    #[test]
    fn prune_refuses_the_grove_root_given_as_a_relative_dot_path() {
        let (_t, g) = jj_grove();
        touch(&g, "BRIEF.md", "root — brief");
        commit_all(&g);
        let err = leaf_prune(guard(&g), Path::new(".")).unwrap_err();
        assert!(err.to_string().contains("grove root"), "got {err}");
    }
}
````
<!-- /fragment -->

**`prune_refuses_the_grove_root`** and
**`prune_refuses_the_grove_root_given_as_a_relative_dot_path`** — the same refusal
through two spellings, and the second exists because `.` is the spelling an
operator standing in the grove root actually types. Both assert
`contains("grove root")`; the first also checks the live leaf is untouched.

They would pass while the property was broken if the refusal fired for the wrong
reason on a `.`-shaped argument — resolving `.` to the root is chapter 6's
`target`, and these tests reach `leaf_prune`'s own arm only if that resolution
worked. The pair therefore establishes rather more than the refusal: it
establishes that both spellings reach the *same* arm.

**And the block ends without a retire twin for either.** There is no
`retire_refuses_the_grove_root`, in this block or anywhere in the workspace, and
the measurement below is what turns that observation into a fact rather than a
failure to find one.

<a id="what-the-refusals-are-worth-measured"></a>
## What the refusals are worth, measured

Every attribution above rests on this, and this is a measurement rather than a
reading. The procedure is the one chapters 8 to 12 established: replace each
refusal or fallback arm with a **panic** in a copy of the workspace, run the whole
of `grove-loop` and `grove-llm`, and diff the per-test results against an
unmutated control run of the same copy.

**The control is 560 tests with 11 failing**, all of
`crates/grove-loop/tests/prompt.rs`, because the copy is neither a jj repository
nor a checkout of the plugin marketplace — ten fail on `NotAWorkspace` and the
eleventh, `the_namespace_is_the_shipped_plugin_entrys_declared_name`, on the
marketplace manifest the copy does not carry — and `cargo build -p grove --bins`
runs first, without which `CARGO_BIN_EXE_grove` is unset and six further
`grove-llm` tests fail on a missing binary rather than on anything about the
code. A control that is wrong in that direction **hides** observers, because a
test already red cannot go redder. Every one of the twenty-four runs below
executed all 560, which is the check that separates a clean result from a mutant
that failed to compile and printed no per-test lines at all. The twelve arms
that *are* `bail!`s are mutated **whole macro call** rather than message, for
the reason chapter 12 measured: a message-preserving panic leaves a `grove-llm`
test asserting on stderr substrings green, so each becomes a bare
`panic!("MUTANT")` outright. The other twelve are not `bail!` swaps at all —
rows 9 to 15 and 20 to 24 are a dispatch, an early `Ok(())`, a `continue`, a
recursion, a `push`, a silent skip, a `.with_context`, a `reopen_write`, two
`.context`s and two branches of the residue helper — and each needs a panic
reachable **only along its own arm**, or it measures reachability instead of
observation. Row 11's `continue` and row 14's silent skip take the panic inside
the branch rather than at the head of the loop; rows 15 and 24 hang their
context off an `Option`, so the `?` has to go with it and a mutant that will not
compile reads exactly like a clean result. No mutant's newly-failing set named a
`driver_lease.rs` or `prompt.rs` test, so none needed the re-run those two
files' timeouts otherwise force.

**Twenty-four arms, in line order.**

| # | Arm | Line | Observed by |
|---:|---|---:|---|
| 1 | `bail!` the argument is the grove root — **retire** | 712 | **nothing** |
| 2 | `bail!` a charter brief | 738 | `retire_refuses_a_node_brief`; `retire_refuses_the_root_brief`; `retire_refuses_a_brief` |
| 3 | `bail!` a node | 742 | `retire_refuses_a_node_directory` |
| 4 | `bail!` an already-`DONE` leaf | 747 | `retire_refuses_an_already_done_leaf`, and its `grove-llm` namesake |
| 5 | `bail!` an `ABANDONED` leaf | 751 | `retire_refuses_an_abandoned_leaf` |
| 6 | `bail!` the driver-reserved `finish` kind — **retire** | 758 | `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind` |
| 7 | `bail!` the argument is the grove root — **prune** | 810 | `prune_refuses_the_grove_root`; `prune_refuses_the_grove_root_given_as_a_relative_dot_path` |
| 8 | `bail!` a charter brief | 842 | `prune_leaf_refuses_a_node_brief`; `prune_leaf_refuses_the_root_brief` |
| 9 | dispatch a node to `plan_subtree` | 846 | fourteen tests, enumerated below |
| 10 | a node with no contents returns `Ok(())` | 861 | **nothing** |
| 11 | `continue` past the node's own `BRIEF.md` | 866 | the same fourteen |
| 12 | recurse into a child node | 869 | `prune_node_recurses_into_a_grandchild_node`; `a_prune_that_stops_partway_names_what_it_already_marked` |
| 13 | collect a `DONE` child into `left_done` | 877 | five of the fourteen |
| 14 | skip an `ABANDONED` child silently | 883 | `prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address` |
| 15 | `.with_context` the entry carries no ordinal or key | 904 | **nothing** |
| 16 | `bail!` a node directory passed as a leaf | 911 | **nothing** |
| 17 | `bail!` a retired (`DONE`) leaf | 917 | `prune_leaf_refuses_an_already_done_leaf` |
| 18 | `bail!` an already-`ABANDONED` leaf | 918 | `prune_leaf_refuses_an_already_abandoned_leaf` |
| 19 | `bail!` the driver-reserved `finish` kind — **prune** | 922 | `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind` |
| 20 | `reopen_write` for the second and later marks | 953 | eight of the fourteen |
| 21 | `.context(stopped_partway(…))` on a failed mark | 969 | `a_prune_that_stops_partway_names_what_it_already_marked` |
| 22 | the *marked nothing* branch of `stopped_partway` | 980 | **nothing** |
| 23 | the singular *leaf* branch of `stopped_partway` | 991 | `a_prune_that_stops_partway_names_what_it_already_marked` |
| 24 | `.context` the library reported no rename | 1007 | **nothing** |

**The four counted cells are one set.** Rows 9 and 11 redden the *same* fourteen
tests, name for name, and rows 13 and 20 are subsets of that same fourteen —
which is what makes them a positive control rather than four separate
measurements. The fourteen, numbered so the two subsets can be read off them:

1. `a_prune_that_stops_partway_names_what_it_already_marked`
2. `leaf_prune_in_a_colocated_tree_leaves_the_git_index_alone`
3. `leaf_prune_marks_a_whole_subtree_abandoned_in_a_jj_native_tree`
4. `leaf_prune_marks_every_live_leaf_and_names_the_done_ones_it_left`
5. `prune_of_a_node_reminds_once_for_the_whole_bulk_mark`
6. `prune_that_marks_nothing_stays_quiet`
7. `pruning_a_node_marks_every_leaf_the_same_way`
8. `prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address`
9. `prune_node_leaves_done_leaves_untouched`
10. `prune_node_marks_a_subtree_mixing_tracked_and_untracked_leaves`
11. `prune_node_marks_every_live_leaf_in_the_subtree`
12. `prune_node_recurses_into_a_grandchild_node`
13. `prune_node_with_nothing_live_marks_nothing`
14. `pruning_a_node_takes_one_guard_per_mark`

Row 13's five are **4, 6, 9, 13 and 14**; row 20's eight are **1, 2, 3, 5, 7,
10, 11 and 14**. Both are subsets, so the four counted cells reach **fourteen**
distinct tests between them rather than the forty-one their sizes add up to —
which is a fact about the sets and not about their sizes, and one no reader
could have checked while the cells said only *fourteen tests*.

Rows 9, 11, 13 and 20 are therefore the positive control that makes the rest
readable: rows 9 and 11 take out the node walk itself and redden all fourteen,
and rows 13 and 20 redden the coherent subsets their arms serve — so the
instrument demonstrably fires when the code it removes is load-bearing, and it
fires proportionately. Eighteen of the twenty-four arms are observed and **six
are not**, and the six do not form one class.

**Every inline observer of this block is in this block.** All eighteen distinct
`tree_lifecycle::tests::` names above sit between lines 2235 and 2725, so no
other chapter's test section reaches an arm of this one. That is not true of the
converse: fourteen of this chapter's thirty-two inline tests observe no arm here
at all, because they establish what the verbs *do* rather than what they refuse.
Two of those fourteen are refusal tests all the same —
`retire_refuses_a_foreign_file` and `prune_leaf_refuses_a_foreign_file` — and
they observe nothing here because the refusal they assert on is not this
block's: a foreign name never becomes an `Entry`, so `task_tree::target` refuses
before either verb's own classification is reached, and that function is chapter
6's.

**Four arms are held only from outside this module**, and two of the four from
outside the crate — which is the boundary worth naming, because it is the second
pair that `cargo test -p grove-loop` alone reports unobserved. Rows 6 and 19 —
the `finish` reservation on *both* marks — are held by one test,
`every_agent_side_mutation_refuses_the_driver_reserved_finish_kind`
(`crates/grove-llm/tests/session_kind_tree.rs`), which is a sweep over every
agent-side mutation rather than a test about either verb. Rows 21 and 23 — the
whole of the residue message — are held by one test,
`a_prune_that_stops_partway_names_what_it_already_marked`
(`crates/grove-loop/tests/verbs.rs`), which induces a mid-run failure with a
read-only directory at the **second** position so the first mark lands and the
second cannot. That second file is an integration test of *this* package, so
`cargo test -p grove-loop` does run it and does see rows 21 and 23; it is rows 6
and 19, held from `grove-llm`, that the package-scoped command reports as held
by nothing. Re-running each of the four mutants under `-p grove-loop` alone is
what establishes that, and it is the kind of claim only a second run can settle.

<a id="the-six-that-hold-nothing"></a>
### The six, and why they are three different things

**Two are unreachable, and the enumeration says so rather than the mutation.**
Rows 15 and 16 are both inside `plan_leaf`, which has exactly two call sites —
line 847 and line 873 — and **both are guarded by a `Parts::Leaf` match on the
same entry's triple**. So by the time `plan_leaf` runs, the triple has already been
taken and the parts have already been matched as a leaf. Its `with_context` for a
missing ordinal and its `bail!` for a node directory cannot fire through any path
in this workspace. The mutation agrees, but the mutation could not have told them
apart from an ordinary gap; two call sites and their guards is what does.

That matters for one of them. Row 16's message is the block's only piece of
operator *instruction* — *pass the directory itself to prune its subtree* — and it
is written for a mistake `plan_prune`'s own dispatch makes impossible, because a
node argument goes to `plan_subtree` on line 846 and never reaches `plan_leaf`. It
is good advice for a call that cannot happen.

**One is a broken contract, and no honest library produces it.** Row 24 is
`marked_path`'s empty report, and its own doc comment says why the case exists:
*an empty report is a contract the library broke rather than a case to handle
quietly*. That is the class chapters 10 to 12 met repeatedly, and it is not a gap
a fixture could close.

**Three are genuine gaps, and all three are one fixture line away.**

Row 10 is the narrowest: a node entry whose `contents()` is `None`. Nothing in the
workspace produces one, and the grammar makes it hard to — a name that parses as a
node describes a directory, and a directory has contents — so this may be
unreachable too. The honest statement from this block alone is that no fixture
produces it and the reason is a claim about the store's classification rather than
about this code.

Row 22 is reachable and is not reached. `stopped_partway`'s *marked nothing* branch
fires when the **first** mark fails, and the one test that exercises the function
deliberately puts its read-only directory at the second position. Moving that
fixture up one leaf would exercise the branch, and the branch is the one an
operator meets when a prune fails early — the message that says *the subtree is as
it was, and rerunning the same command is safe*.

**Row 1 is the one that matters, and it is an ordinary operator-facing refusal.**
`leaf_retire` refuses the grove root on line 712, and nothing holds it. Its
`leaf-prune` twin on line 810 has **two** tests, one for each spelling of the root,
and the two verbs' refusal sets are otherwise near-mirrors of one another
throughout this block. There is no `retire_refuses_the_grove_root` anywhere in the
workspace.

The path to it is open, and it is shorter than chapter 12's equivalent, because it
needs no flag. `grove-llm`'s `LeafRetireArgs` and `LeafPruneArgs` are the same
declaration under two names — one free `PathBuf` each, documented identically as
*absolute, or relative to the grove root* — and both commands hand it through the
same `normalize_leaf_path`, which turns a relative `.` into the absolute directory
the operator is standing in. So `grove-llm leaf-retire .`, typed inside `.grove/`,
resolves to `Target::Root` and meets line 712 and nothing else. It is exactly the
spelling `prune_refuses_the_grove_root_given_as_a_relative_dot_path` exists to
cover on the other verb. What
the mutation establishes is that replacing the refusal with a panic **reddens
nothing** against the 560-test control — not one test changes colour — so the
refusal an operator would meet is held by no test at all. **The asymmetry, not
the absence, is the finding**: the same argument on the same shape of tree is
covered twice for one verb and not once for the other, and nothing in the block
explains the difference.

<a id="what-could-not-move"></a>
## What could not move

**On the way in — the names.** This chapter writes names rather than reading new
ones, and what it writes is the narrowest possible edit: one field of a `Parts`
value, rendered back through the grammar chapter 4 proved canonical. That
canonicity is doing real work here rather than being cited. *In place* is only a
meaningful claim because `format(parse(f)) == f` — the mark takes a name apart,
changes the outcome, and puts it back, and every byte that was not the outcome has
to come out the way it went in. A grammar that normalised anything on the way
through would make *the position and the key are kept* an assertion about
rendering rather than a fact about identity.

The second thing the names buy is the one the whole chapter rests on. Because an
outcome is **in** the name, `pick` skips terminal work by reading a directory
listing, an interrupted bulk mark leaves a tree that describes itself, and the
repair for a half-finished prune is the command that half-finished it. None of
that is a mechanism grove built; it is what follows from refusing to keep the
state anywhere else.

**On the way through — the preconditions.** This is the chapter's weight, and the
measurement above is the reason it can be stated as a quantity. Both verbs classify
before they call, and the classification is thorough enough that
`docs/ARCHITECTURE.md#library-refusals` gives the pair an **empty** cell: no
`ordinal_fs_tree` refusal reaches an operator through a mark. The species refusal
`rewrite` would make is hidden behind `retire_parts`'s and `plan_leaf`'s own; and
`DestinationOccupied` — the one that looks unavoidable, because a mark's
destination is a name that could already exist — is unreachable by arithmetic. An
outcome infix and a key are both parts of one name, so a twin occupying the
destination necessarily carries the live leaf's key, and `addressable_key` refuses
that first with a sentence about keys rather than about destinations.

**The cost of that is a single line of grove's code carrying a whole row of a
record**, and the record says so: `addressable_key`'s tree-wide twin scan is what
makes `DestinationOccupied` unreachable from every flipped verb, this pair
included. Chapter 6 owns the function; this chapter is where its absence would be
worst, because `rewrite` is called *by key* and a mark aimed at the wrong twin
does not fail — it succeeds, changes nothing, and reports the twin's path.

And the second precondition is the one this chapter pays for in guards rather than
in code. A bulk mark is all-or-nothing because the whole subtree is planned and
validated against the **first** guard's snapshot; the library, seeing one entry at
a time and offering no batched rewrite, cannot make that promise, so it is grove's
by default rather than by choice. What grove gives up for it is the window between
guards, and the size of that window is the number
`pruning_a_node_takes_one_guard_per_mark` asserts.

**On the way out — the policy.** Two choices, and neither is defaultable from
below.

The first is **which** marks exist, and it is worth separating carefully from a
constraint that is not grove's at all. That an outcome is a *mark* rather than a
removal was decided beneath grove and could not have gone the other way: the
library offers no operation that removes an entry, because a fresh key is the
maximum over the whole tree plus one, so the names on disk **are** the counter and
deleting one re-issues a key other entries may still reference
(`docs/adr/entries-are-never-removed.md`). A domain needing entries retired marks
them and leaves them in place. Grove did not choose that; it inherited it, and
this chapter is where the inheritance is spent.

What grove chose is that there are **two** terminal marks rather than one. A store
constrained as above needs only *not live*; grove writes `DONE` and `ABANDONED` and
keeps them rigorously apart, and the clearest statement of why is a comment on a
test rather than on a verb — *a missing flag must degrade to something harmless,
never to the opposite outcome*, because retiring an abandoned leaf would silently
assert that rejected work was finished. Nothing beneath grove could have defaulted
that distinction, because nothing beneath grove knows that one of these is a claim
about work and the other is a judgement about it.

The second is **the arity asymmetry**, and it is the one place in the chapter
where grove's vocabulary and the store's diverge in a way no amount of care can
reconcile. `leaf-retire` is a claim about one session's work and takes one leaf.
`leaf-prune` is a decision about a *line* of work and takes a subtree, because
that is the unit a person actually abandons. The store has no word for either
distinction: to `ordinal_fs_tree` both are `rewrite`, called once or called *N*
times. The asymmetry is grove's whole answer to *what does the layer choose that
nothing beneath it could have defaulted*, and the chapter's honest note is that
the repository argues it in one doc comment and nowhere else — the record the
comment cites states the authority boundary, not the arity.

The grove now has a leaf that says its work is done, in the only place grove keeps
anything. Chapter 14 takes the tree to the state this chapter can produce and no
other — every leaf terminal, nothing live — and finishes it, which is the one
operation in the crate whose observable end is that the tree stops existing.

[Previous: A leaf becomes a node](12-leaf-to-node.md) | [Contents](README.md) | [Next: Finishing](14-finishing.md)

# A witnessless root refuses what it cannot account for

**Every root is witnessless now.** `delete-migration-k6` removed the format
witness along with migration, so the qualifier in this record's name no longer
narrows anything — it names the only kind of root there is. The rule it states
is unchanged, and its mechanism is what got simpler.

A present task root that Grove did not finish writing is classified by an
**ordered three-way test**, and the order is what makes it fail closed:

1. **`Taskless`** — the root holds nothing but its charter, if even that.
   **Refused**, naming what is missing and how to put it back.
2. **`ATree`** — the root holds at least one name the grammar owns: an entry, or
   one it refuses. Left alone; a name Grove refuses is *held*, not absent, and
   the next reader states it in the domain's own words.
3. **`Unrecognised`** — the root holds names Grove disclaims, and nothing else.
   Refused, mutating nothing, naming the disclaimed entries and the grammar
   Grove does read.

**The first branch used to be repaired and is now refused, and that is
`collapse-tree-access-k13`'s doing rather than a change of mind here.** Root
initialisation used to be two phases under two different locks — Grove created
the root and its charter under a guard of its own, released it, and appended the
first leaf under the library's — so an interruption between them left exactly
this shape, and completing it was the right answer for a shape Grove itself
produced. `Vacancy::initialize` closed the window: the charter is the root's
distinguished child, the store writes root, charter and first leaf under the one
lock that found the vacancy, and a failed `initialize` takes the root back down
with it. Nothing Grove does produces a taskless root any more, so what reaches
the branch is a tree something emptied by hand — and repairing that would be
mutating a tree whose ownership the operation cannot prove, which is the very
thing the third branch exists to refuse. The rule this record states is
unchanged; the repair went with the anomaly it repaired.

`tree_lifecycle::root_shape`
([`crates/grove-loop/src/tree_lifecycle.rs`](../../crates/grove-loop/src/tree_lifecycle.rs))
is that test.

## What the witness's removal changed, and what it did not

**The middle branch's job moved and its principle did not.** The witnessed
version distinguished `PartialScaffold(Exact)` from `PartialScaffold(Ambiguous)`
by a byte-exact comparison against what a fresh scaffold writes, and needed
**root-init-exclusive** entries — the reserved format temporary, and the first
`requirements` leaf at position 1 with key 1, byte-equal — because a witnessless
root was *also* how a legacy tree presented, and the two got opposite treatment.
Migration is gone, so nothing is discriminating against legacy any more, and the
byte comparison went with the thing it was discriminating against.

What replaces it is cheaper and rests on a different guarantee:
[`entries-are-never-removed`](entries-are-never-removed.md). A tree that has ever
held a leaf still holds one, so *this root holds no Grove entry* is a complete
answer without a byte comparison. The charter stays excluded from the evidence
for the reason it always was — its bytes derive from the working-tree name and
every earlier format wrote the same ones, so a charter is evidence that *some*
Grove was here and never evidence of *this* one — which is why a charter-only
root is `Taskless` rather than proof of ownership.

**The refusal branch is now load-bearing in a way it was not.** Under the witness
it caught a stray file beside Grove's own half-written scaffold. It now also
catches the whole class the `Legacy` branch used to absorb: the layouts Grove
wrote before this grammar are positioned but *unkeyed*, so every one of their
names is `Foreign` — invisible to the reader rather than refused by it. Without
the third branch such a tree would read as an empty grove and take the driver's
finish sentinel. Refusing is the same fail-closed ownership rule applied to a
larger set.

## The trade-off it settles

The competing repair is the one the Quint column proposed from its own
counterexample: define the state by the **presence** of the scaffold's own
entries and **ignore** entries outside the task grammar. It is simpler, it needs
no third branch, and it keeps the state table at eleven rows.

It is rejected because it establishes the wrong thing. The safety argument it
rests on — *every value a completion writes is fixed in advance, and a foreign
entry is not something completion writes* — proves the **bytes** a completion
writes are safe. It says nothing about whether **this root** is Grove's to write
into, and an entry Grove did not write is exactly the proof it lacks.
Completing under it is a mutation of a tree whose ownership the operation cannot
prove, which is the one thing `TT-24` forbids everywhere else. The counterexample
that motivated the proposal is real; only its second half is.

**The product had already answered, and answering the same way twice from
different evidence is what settled it.** The witnessed implementation
(`recover_partial_root_init_unlocked`) ran exactly this test and refused with an
*ambiguous partial root scaffold* diagnostic the contract's state table had no
member for. That function is gone with migration, and the argument survived it
intact: `root_shape` refuses the same class for the same reason, having reached
it from the tree's shape rather than from a byte comparison.

## What it cost, and what removal settled

The witnessed version carried a **diagnostic** defect it could not close.
`TT-20`'s prohibition on `Legacy` narrowed to *once a root-init-exclusive entry
has landed*; before that — after the charter, before the leaf — the root carried
no evidence distinguishing it from a legacy tree, so an operator inside that
window was told to migrate a tree that was not legacy. Wrong but actionable,
because migration existed and did something coherent.

**This record named the removal as its own reopen condition, and the removal
happened.** The concern recorded here was that removing migration would make
`Legacy` fail closed and tell an operator to migrate by a command that no longer
exists, about a directory Grove created and then failed to recognise. That
concern is answered rather than realised, and by construction rather than by
care: the window it describes is `Taskless`, which has its own branch and its own
message. The incoherent message the removal was feared to produce has nowhere to
appear.

**The cost the witnessed version carried is gone rather than reduced.** It was
that a charter-only root whose charter Grove did not write would be *completed*
rather than refused — a mutation of a tree Grove could not prove was its own.
There is no completion left to be wrong: the branch refuses, and the refusal
names `jj undo` and the alternative of moving the directory aside. What is paid
instead is smaller and in the other direction: a genuinely half-built root, if
one could still arise, is a sentence to a human rather than a repair. Nothing can
still arise it, because initialisation is one operation that unwinds itself.

## Alternatives rejected

- **Treat the charter as ownership evidence.** Retired rather than rejected, and
  by the outcome moving to meet it: the charter cannot carry that weight — its
  bytes derive from the working-tree name, so every format Grove ever wrote
  produced the same ones — and the branch now refuses whatever the charter says,
  so there is no completion for ownership evidence to authorise. Under the
  witness this was enforced by a shipped test that put a byte-exact charter
  beside a legacy-v2 leaf and migrated rather than completing; that test went
  with migration, and what replaces it is structural — a legacy leaf beside the
  charter lands in `ATree` or `Unrecognised` on its own name, so the charter
  never hides one.
- **A guard across root initialisation's two phases.** Rejected because it buys
  nothing against the actor that produces the counterexample: `EN-06` grants
  only that *cooperating* processes are serialized, and the writer here is
  `EN-13`'s non-cooperating one. A guard would close a window against exactly
  the writers that were never in it. Reopen if the lock becomes mandatory.
- **A class parameter on the charter-only state, rather than a separate refusal
  state.** This is what the witnessed version did, and the witness's removal
  retired it: with no byte comparison there is no `Exact`/`Ambiguous` split to
  carry, and the two outcomes are different branches reached from different
  evidence rather than two classes of one state. The distinction the parameter
  protected — never telling an operator *scaffold incomplete* about a root Grove
  has declined to complete — survives twice over now that neither branch
  completes anything.
- **Reorder root initialisation so its first write proves ownership**, closing
  the surviving cost above. Retired rather than rejected: the witnessed version
  could have made the reserved format temporary the first write, and there is no
  witness to reorder any more. What would replace it is giving the charter bytes
  that name the format that wrote it, which is the reopen condition stated in
  *What it cost* — one product change, still not taken.

## What enforces it

Unit tests in `crates/grove-loop/src/tree_lifecycle.rs`, each asserting the
mutation — or its absence — as well as the verdict:
`transition_leaves_a_current_grove_unchanged_and_ready_for_pick` and
`transition_does_not_scaffold_over_a_name_grove_refuses` (branch 2),
`transition_refuses_a_root_holding_no_grove_entry_at_all` (branch 3), which also
holds that the refusal names the disclaimed entries and the grammar Grove reads,
and `transition_initializes_an_absent_grove_under_one_exclusive_guard`, which is
what leaves branch 1 with nothing of Grove's own to classify.
`crates/grove/tests/lifecycle_cutover.rs`'s `a_withdrawn_layout_is_refused_without_touching_the_tree`
drives the third branch black-box over both withdrawn layouts and asserts the
tree is byte-identical afterwards.

**The model controls that first established the three-way test are not what
enforces it now.** They were the formal-modelling campaign's, and the campaign's
apparatus was retired once its lessons were distilled into the `linkuistics`
skills; the decision survived the instrument that found it, which is the outcome
that campaign was run to test. `docs/formalism-findings.md` keeps the record of
how it was found.

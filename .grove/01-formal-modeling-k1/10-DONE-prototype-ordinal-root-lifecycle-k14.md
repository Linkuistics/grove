# ordinal-root-lifecycle-k14


## Goal

Determine experimentally whether atomic root lifecycle is a deep, domain-independent `ordinal-fs-tree` capability or should remain private to Grove finish.



## Context

The user prefers filesystem mechanics to be delegated to `ordinal-fs-tree`, but this is not permission to turn it into a bag of Grove-shaped callbacks. Evaluate lock ownership, root identity, opaque/foreign entry preservation, crash states, rollback/recovery evidence, and whole-root evacuation/removal against both completed model families.

## Done when

- A candidate contract can be stated without Grove, Git, jj, session kinds, task handles, finish tickets, branches, or bookmarks—or the experiment records that it cannot.
- The candidate owns ordering, locking, identity validation, opaque/foreign preservation, durable staging/rollback states, and recovery-safe failure semantics behind a small interface; it exposes neither raw syscall choreography nor arbitrary callbacks that leak the operation.
- A focused prototype/model is tested against representative task-tree and finish counterexamples from both formalisms.
- The alternatives—extend `ordinal-fs-tree`, keep a finish-private adapter, or introduce no new abstraction—are compared for interface depth, misuse resistance, portability, and synchronization cost.
- The decision is keep, defer, or reject with evidence. No production implementation is performed here.

## Notes

If the abstraction earns implementation, `formal-synthesis-k16` must insert a dedicated implementation leaf before `extract-task-tree-k24`. If it does not, document the precise semantic exception that remains in `grove-finish`.

## Decisions (running log)

**The instrument is a throwaway Quint model plus a vocabulary control, and it
lives outside the repository.** `models/run.sh` fails the whole run on any `.qnt`
or `.als` outside its four known scope directories ("runner error: … is in no
known scope"), so a prototype model committed anywhere in the tree would either
break the runner or have to be adopted into a scope — which is the production
implementation this leaf forbids. The model was therefore built in the session
scratchpad, and this entry plus `docs/formalism-findings.md` 047 carry every
command, dial and counterexample needed to rebuild it. Recorded so the next
reader does not go looking for a file.

**The contract is statable with no domain vocabulary at all — Done when #1 is
answered affirmatively, and by enumeration rather than by a banned-word sweep.**
Every identifier in the prototype was extracted and classified: 175 non-keyword
identifiers, of which **0** name a workstream, a task, a session, a version
control system, a commit, a branch, a bookmark, a handle, a ticket, a witness, a
quarantine or a manifest. (Two, `Kind`/`kind`, match the word *kind* and mean
"whether the item can be fingerprinted"; they are classified, not swept.) The
instrument was controlled in both directions: the same extraction over
`crates/grove-finish/models/finish.qnt` reports 103 domain-loaded identifiers and
over `crates/grove-task-tree/models/task-tree.qnt` 70, and over the prototype's
own *comments* 16 — so a clean read of the prototype's code cannot be a broken
instrument reading clean everywhere.

**The constructive and destructive halves do not have the same answer, and that
split is the finding.** Creation — publish the identity token last, by one
atomic same-directory rename, so an interruption classifies partial and never
valid — needs no oracle, no receipt and no callback, and holds under every dial.
Destruction does not, for two independent reasons recorded below.

**Retained counterexample 1: the library cannot own the terminal step of
destruction.** Between the settle rename (the whole root moved aside, staging
intact) and disposal, the container's root is `Absent` — which a reader cannot
distinguish from a container that was never created — while the staging area
still holds every item. `FN-20`'s role forbids reading that leftover as evidence
that anything happened, and the library's whole world is `<root>` and `<root>/..`,
so it has nowhere to put a receipt. The interval `TODO.finish_process.md` calls
"the whole problem" is therefore irreducibly the **caller's** to close, with an
artifact outside the container — Grove's correlation ticket, `FN-03`. Established
by `quint verify` over the reduced instance, and *not* removed by the caller
obligation that removes counterexample 2, which is what isolates the two causes.

**Retained counterexample 2: four revalidation points are necessary and not
sufficient, and what closes the gap is an obligation on the caller.** After the
last revalidation there is always a suffix in which the caller's grade can move,
and by then disposal has destroyed the ability to return. Adding
`VERDICT_MONOTONIC` — *once the caller grades an effect applied it never ungrades
it* — makes the claim hold and leaves counterexample 1 untouched, so the two are
separately caused. That obligation is domain-independent to state and impossible
for the library to check; the catalogue currently carries it only as the
lane-shaped `HISTORY_IMMUTABLE`.

**The verdict oracle is a callback, which is the shape
`entry-name-is-the-only-seam` rules out by name, and its cost is measurable
rather than theoretical.** The `mutant_reentrant_caller` control shows a caller
consulted from inside the transaction — while the library holds the container's
lock and the root is unwalkable — can only be refused, converting an attempt
whose verdict was *applied* into a block. `ordinal-fs-tree`'s own
`Error::Reserved` already states the incumbent position: the library sees a
transaction witness and halts, because "the library cannot know what it means, so
proceeding past it is a guess". Owning destruction means it now knows what some
reserved names mean, which is a widening of the single seam and not a use of it.

**A green run over the candidate proved nothing until the controls ran, and one
control initially killed nothing — because two claims had been written against
the dial instead of against the contract.** `inv_RL_4` was conditioned on
`REFUSE_OPAQUE_FIRST`, and `inv_RL_7` carried a literal `or not(REVALIDATE)`
escape, so each was vacuous in exactly the instance meant to falsify it. Both
were restated over the contract; `mutant_opaque_late` then killed `RL-4` alone,
and revalidation only became measurable once the caller's *actual* grade was
separated from the library's last *observation* of it.

**The decision is recorded where it survives `.grove/`, and the citation that
would have sent the next reader to the wrong place was tightened.** The verdict
is `reject` and it lands in
[`root-lifecycle-stays-with-its-receipt`](../../docs/adr/root-lifecycle-stays-with-its-receipt.md),
with the narrowed successor question (creation alone, which needs no coordinator)
deferred to `formal-synthesis-k16` — which already consumes a keep/defer/reject
from this leaf and therefore needs nothing inserted beside it. The third pressure
on the one seam that was tested and did not fire is recorded in
`entry-name-is-the-only-seam` itself, and the new record joins the ownership list
in `CONTEXT-MAP.md` under the **grove** context. The caller obligation was cited
as `HISTORY_IMMUTABLE` alone, which is the Quint finish model's *dial* and appears
nowhere in the catalogue; both the entry and the record now name `FN-26` — *history
is never rewritten to clear a block* — and mark the dial as that family's switch,
because `formal-synthesis-k16` owns the disposition of catalogue findings and will
search the catalogue.

**No review chain is cut beside this leaf, and the reason is not that the work is
small.** Each of the three Quint prototypes took a `review-prototype` leaf, and
each review found real defects, so the default here is review. Two things move it:
the instrument was throwaway *by construction* and is gone, so the whole class of
attack those reviews ran — kill a control, re-run a narrowing, re-do the
enumeration over the model's own tokens — has no subject; and what remains is an
argument whose every external citation was re-verified this session against the
tree (`docs/ordinal-fs-tree/ARCHITECTURE.md`'s containing-directory lock,
`Error::Reserved` in `crates/ordinal-fs-tree/src/error.rs`, `FN-03`, `FN-20`,
`FN-22`'s four revalidation points, `FN-26`). The obvious attack on the central
move — *let the caller hand the library a receipt sink* — is already closed by the
record's second ground, since a sink consulted mid-transaction is the callback the
re-entrancy control priced. What a fresh context should contest is the **verdict**
rather than the instrument, and `formal-synthesis-k16` is chartered to do exactly
that with this entry in hand; a review leaf here would buy the same read twice.

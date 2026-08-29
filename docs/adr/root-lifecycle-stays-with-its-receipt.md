# Root lifecycle stays with its receipt

Whole-root creation and destruction stay **private to grove**, and
`ordinal-fs-tree` gains no root-lifecycle capability. This is the documented
semantic exception the root brief requires for a Grove-owned filesystem
operation, and it is **two** exceptions on separate grounds, neither of which is
that the mechanics are Grove-shaped — they are not, in either half.
**Destruction** is owned because the operation cannot be *terminated* by anything
that does not also hold the receipt. **Creation** is owned because, with the
coordinator gone, what is left is about thirty lines of `std::fs` with no
algorithm behind them and three new library concepts to hide them — a shallow
module, and the correctness half stays with grove regardless.

The evidence is `docs/formalism-findings.md` entry 047, an executable prototype
of the candidate contract stated in fully domain-neutral vocabulary.

## What the prototype established, in the direction the default expected

The root brief's default is that generic filesystem mechanics live in
`ordinal-fs-tree`, and the case for moving root lifecycle there is stronger than
it looks from the outside — which is why this record exists rather than a line in
a commit message.

- **The contract is statable with no domain vocabulary at all.** Of 175
  non-keyword identifiers in the prototype's code, **zero** name a workstream, a
  task, a session, a version control system, a commit, a branch, a bookmark, a
  handle, a ticket, a witness, a quarantine or a manifest. The same enumeration
  over `crates/grove-finish/models/finish.qnt` reports 103 and over
  `crates/grove-task-tree/models/task-tree.qnt` 70, so the clean read is a
  measurement rather than a broken instrument reading clean everywhere.
- **The library's own architecture already reasoned about the case.** The
  advisory lock is taken on `<root>/..` rather than on the root precisely so that
  "the tree's creation and destruction fall under the same lock as every ordinary
  operation", and `docs/ordinal-fs-tree/ARCHITECTURE.md` calls that reasoning
  general rather than domain-specific. The one seam's own record makes the same
  argument: "the containing directory outlives both the root's creation and its
  deletion in every domain."
- **Ten of the eleven prototype claims hold** under symbolic checking, including
  ordering, identity-published-last, opaque-entry refusal before any mutation,
  conservation across evacuation, restoration-only-under-proof, and the two
  outcomes a foreign entry at a reserved name must produce.

## Why it is nonetheless rejected

**The library cannot terminate the operation.** Between the settle rename and
disposal, the container's root is absent — indistinguishable, to any reader, from
a container that was never created — while the staging area still holds every
entry. `FN-20`'s role forbids reading that leftover as evidence that anything
happened, and **no filesystem receipt fixes it**: the thing that decides whether
the destruction should have happened is an *external* effect the library cannot
observe, so a library-written receipt could only say *I removed the root*, never
*the removal is committed*. Grove's receipt is the deletion commit, and it lives
in the repository's own history — the one place that survives both the
container's destruction and its re-creation at the same name. What that decides
is ownership: the receipt must live outside the container, and a component with
no notion of "outside" cannot own an operation whose completion only that
receipt establishes.

**The coordinator is a callback, which is the shape the one seam rules out.** A
destroy verb must consult the caller's three-valued grade of an external effect
at four revalidation points. That is a hook by another name, and its cost is
measurable rather than theoretical: the prototype's re-entrancy control shows a
caller consulted from inside the transaction — while the library holds the lock
and the root is unwalkable — can only be refused, converting an attempt whose
verdict was *applied* into a block.

**It also needs an obligation no caller in this system can honour, and that is
stronger than *the library cannot check it*.** Four revalidation points are
necessary and not sufficient: after the last one there is always a suffix in
which the caller's grade can move, and by then disposal has destroyed the ability
to return. The prototype closes that only by adding *once the caller grades an
effect applied it never ungrades it*.

**The catalogue was asked whether to gain a general form of that obligation and
declined it, on a refutation rather than on cost**
(`docs/specs/semantic-contract.md`, beside `FN-26`). `FN-22`'s revalidation table has **two rows that are exactly the
transition the obligation forbids** — after the quarantine rename,
`Committed -> NotCommitted` and `Committed -> Indeterminate` — and the contract
goes out of its way to say the two must not be collapsed, because collapsing them
would let a block be reported as a refusal. Granting the obligation as an
environment assumption instead deletes the states those rows need, which one
model column paid for already: an append-only history under every step made the
disposition monotone and left both rows answerable **by construction**.

So the coordinator's caller obligation is not merely unverifiable here. **Grove
could not honour it even if it wanted to**, because this protocol's correctness
depends on the grade being re-read and being allowed to move. What Grove supplies
instead is narrower and is already stated twice — history is never rewritten to
clear a block (`FN-26`), and no grade is ever carried forward as a licence
(`FN-22.a`'s four points, which is `SY-03`'s *a preflight is never a licence* at
this grain). Grove's answer to a grade that can move is to **survive** the move,
and the residue it leaves is why the repository's own history rather than the
tree is the evidence of success.

**And the library already says so.** `Error::Reserved` exists for "a name the
consumer owns that is deliberately not an entry — a transaction witness, a lock
marker, a sentinel left by an interrupted operation", and halts on it because
"the library cannot know what it means, so proceeding past it is a guess."
Owning destruction means the library now knows what *some* reserved names mean,
which splits the reserved-name class into a library-owned half and a
consumer-owned half. That is a widening of the single seam, not a use of it.

## The alternatives, compared

| | interface depth | misuse resistance | portability | synchronization cost |
|---|---|---|---|---|
| **extend `ordinal-fs-tree`** | high for creation, **negative** for destruction — the caller must be handed a resumable, four-point coordinator protocol, which is the operation leaking through its own interface | **worst.** A callback consulted mid-transaction; a re-entrant caller can only be refused; a caller obligation the library cannot enforce | unchanged — the containing-directory lock already generalises | **highest.** Two model families and an architecture document are stated in the one seam's terms; widening it re-opens all three |
| **a finish-private adapter** (chosen) | high where it is real: `grove-finish` already owns the ticket, the anchor and the lanes, so the coordinator is a caller of itself rather than a hook | best available — no seam widens, and `Error::Reserved` keeps meaning what it means | unchanged | none |
| **no new abstraction at all** | the incumbent — 10,366 lines across three nested crash-safe transactions | unchanged | unchanged | none |

The chosen option is the middle one, and it is not the same as the third: the
adapter is a boundary this workstream still owes inside `grove-finish`, and this
record decides only that the boundary is not a crate one.

## The verdict was contested and stands

`finish-verdicts-k65` was chartered to contest this rejection rather than inherit
it, and the ground of the contest was the obvious one: **the prototype is gone,
so the instrument cannot be attacked and the argument has to carry itself.** It
does, and which parts do is worth recording.

- **The first argument is independent of the prototype.** `FN-20`'s role is
  mutation-killed in both families, `FN-03`'s ticket is checked in both, and the
  finding that success is proved by the commit and never by the tree was reached
  by falsifying three separate formulations of `FN-28` against a trace, not by
  the prototype.
- **The third argument is independent too**, and doubly so: `FN-22`'s two
  `Committed -> …` departure rows are in the catalogue with witnesses in both
  columns, and the append-only-history incident that made them answerable *by
  construction* is a recorded model failure rather than a prototype result.
- **The second argument is the one that dies with the prototype.** The
  re-entrancy cost — a caller consulted mid-transaction can only be refused — was
  measured there and nowhere else. It is not load-bearing: the first and third
  each suffice alone.

**One sentence was found too strong and is corrected above rather than defended.**
It read *the library's whole world is `<root>` and `<root>/..`, so it has nowhere
to put a receipt that is not the leftover* — but `<root>/..` **is** in the
library's world, and a receipt there would outlive the root. The rejection does
not need that claim and is stronger without it: the problem is not where a
receipt can sit but what it can attest, and no filesystem artifact can attest an
external effect.

## Root creation is rejected too, and the measurement is what decides it

The deferred question was whether *creation alone* — no coordinator, no receipt,
no callback, just publish the identity token last by one same-directory rename —
earns a place in the library. It does not, and the reason is depth rather than
correctness: **once the coordinator is removed there is nothing left to hide.**

What would move out of grove, measured: `create_root_unlocked` is ten lines
(`create_dir_all`, then the charter); `write_root_brief` and `root_brief_body`
are eleven and are *entirely* domain content that would have to be passed back in
as bytes; and `tree_format::write_current_last` is thirty-six, of which the
generic residue is *create a same-directory temporary idempotently and rename it
into place* — about thirty lines of `std::fs` calls with no algorithm behind
them. Against that, the library's interface would gain three concepts it does not
have: creating a root at all, a distinguished child that arrives at creation
rather than through `promote`, and a consumer-supplied identity token with a
publish-last contract.

**And the half that makes the operation correct cannot move.** `TT-20`'s
load-bearing clauses are stated over grove's format taxonomy — the interrupted
root SHALL never classify `Current(*)`, and once a root-init-exclusive entry has
landed it SHALL classify `PartialScaffold(_)` and never `Legacy` — and the
library has no vocabulary for any of those words. Nor would library ownership
close the two-phase window: a lock does not survive a crash, `EN-06` grants only
that *cooperating* processes are serialized, and the catalogue already records
that closing it is a **product** change — make root initialisation's first write
a root-init-exclusive one. The guard release between the two phases is
load-bearing in the other direction as well, since it is what lets a second
cooperating process meet a partial root and complete it.

So both halves of root lifecycle stay with grove, for different reasons:
destruction because it cannot be terminated without the receipt, creation because
it is too shallow to be worth an interface.

## The machinery this was argued against is gone, and the decision is not yet re-taken

`delete-finish-transaction-k8` deleted the four-point coordinator, the quarantine
rename and the attempt-bound correlation ticket that the argument above reasons
over. What survives it is the shape of the argument rather than its incumbent:
destruction is still an external effect the library cannot observe, and the
receipt is still a commit outside the container. What is *no longer* true is that
the interval between the settle rename and disposal exists at all — there is no
staging area and no disposal, only a recursive delete and one path-scoped commit.
So the rejection stands on a thinner incumbent than the one it was written
against, and re-taking it on the merits belongs to the leaf that lands the store's
own `delete` (`docs/specs/module-decomposition.md`, decision 2), not here.

## What would reopen this

Reopen the **destruction** rejection if a consumer appears that needs whole-root
destruction **without an external effect to coordinate with** — a destroy whose
only verdict is the filesystem's own. That consumer has no receipt problem, no
coordinator and no callback, and the argument above does not reach it.

Reopen the **creation** rejection on depth rather than on need: a second consumer
whose own root creation has the same publish-last shape turns thirty shallow
lines into a duplicated contract, and duplication across consumers is what makes
a thin interface earn its place. One consumer is not evidence of that.

**The catalogue and the models this record measures against have been retired.**
The catalogue was `docs/specs/semantic-contract.md`, deleted with the campaign's
apparatus (`delete-formal-models-k29`); `crates/grove-finish/models/`, which
supplied the 103-identifier figure above, was deleted at
`delete-finish-models-k30`, and `crates/grove-task-tree/models/`, which supplied
the 70, at `delete-formal-models-k29`.

The counts stay in the text, because they are what the argument was made on and
because the argument does not need them re-taken: the domain-vocabulary
measurement was a *comparison* — zero against 103 and 70 — and its conclusion is
that the destruction contract cannot be stated without domain words, which the
prototype's own zero already carries. What is no longer available is the
**instrument**, so a reader who doubts the figures cannot re-run the enumeration;
`docs/formalism-findings.md` records how it was taken, and the models are
recoverable by revision. The decision survived the instrument that found it,
which is the outcome that campaign was run to test.

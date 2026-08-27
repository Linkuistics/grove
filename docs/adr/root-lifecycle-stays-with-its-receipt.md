# Root lifecycle stays with its receipt

Whole-root creation and destruction stay **private to grove**, and
`ordinal-fs-tree` gains no root-lifecycle capability. This is the documented
semantic exception the root brief requires for a Grove-owned filesystem
operation, and it is narrow: the exception is **destruction**, and it is owned
not because the mechanics are Grove-shaped — they are not — but because the
operation cannot be *terminated* by anything that does not also hold the receipt.

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
happened, and the library's whole world is `<root>` and `<root>/..`, so it has
nowhere to put a receipt that is not the leftover. This is
`TODO.finish_process.md`'s *"the interval is the whole problem"* reached from the
library's side, and what it decides is ownership: the receipt must live outside
the container, Grove's correlation ticket is that receipt, and a component with
no notion of "outside" cannot own an operation whose completion only that receipt
establishes.

**The coordinator is a callback, which is the shape the one seam rules out.** A
destroy verb must consult the caller's three-valued grade of an external effect
at four revalidation points. That is a hook by another name, and its cost is
measurable rather than theoretical: the prototype's re-entrancy control shows a
caller consulted from inside the transaction — while the library holds the lock
and the root is unwalkable — can only be refused, converting an attempt whose
verdict was *applied* into a block.

**It also needs an obligation the library cannot check.** Four revalidation
points are necessary and not sufficient: after the last one there is always a
suffix in which the caller's grade can move, and by then disposal has destroyed
the ability to return. The prototype closes that only by adding *once the caller
grades an effect applied it never ungrades it* — statable in general, unverifiable
by the library, and carried in the catalogue today only in lane-shaped form, as
`FN-26` — *history is never rewritten to clear a block* — which the Quint finish
model dials as `HISTORY_IMMUTABLE`. A general form is the catalogue's to gain or
decline; `formal-synthesis-k16` owns that disposition.

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

## What is deferred, and what would reopen this

**Root *creation* is a genuinely narrower question and it was not decided here.**
It needs no coordinator, no receipt and no callback: publish the identity token
last by one same-directory rename, so an interruption classifies partial and
never valid. It survived every dial in the prototype. It is deferred to
`formal-synthesis-k16`, which is sizing the crate boundaries and can weigh what
it actually removes from grove; the prototype measured that it *works*, not that
it *earns its place*.

Reopen the rejection if a consumer appears that needs whole-root destruction
**without an external effect to coordinate with** — a destroy whose only verdict
is the filesystem's own. That consumer has no receipt problem, no coordinator and
no callback, and the argument above does not reach it.

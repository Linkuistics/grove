# Root lifecycle belongs to the store

Creating a tree root and destroying one are `ordinal-fs-tree`'s, not grove's.
`Vacancy::initialize` creates the root, its distinguished child and its first
entries under the lock a vacancy already holds; `WriteGuard::delete` removes the
root and everything beneath it and reports the paths that went. Grove asks for
both and touches the task tree for neither.

That is a reversal, and it is recorded as one because the argument it reverses
was long and is worth not re-making. This record used to say *root lifecycle
stays with its receipt* — that destruction could not be owned by anything that
did not also hold the receipt proving it should have happened, and that creation
was too shallow to be worth an interface. Both halves were argued against
machinery that no longer exists.

## What the earlier rejection rested on, and what became of it

Three arguments carried it, all about **destruction as a coordinated
transaction**: grove's finish cycle at the time staged the tree aside, renamed a
quarantine into place, disposed of it, and correlated the whole thing with an
attempt-bound ticket.

- **The library cannot terminate the operation.** Between the settle rename and
  disposal the root was absent while the staging area still held every entry, and
  no filesystem receipt could say *the removal is committed* — only an external
  effect decides that.
- **The coordinator is a callback**, consulted at four revalidation points,
  which is the shape [`entry-name-is-the-only-seam`](entry-name-is-the-only-seam.md)
  rules out.
- **It needs an obligation no caller could honour** — *once the caller grades an
  effect applied it never ungrades it* — which grove's own contract required it
  to break.

`delete-finish-transaction-k8` deleted the coordinator, the quarantine rename and
the ticket, on the principle that the version control system owns safety,
history and transactionality. **There is no interval between a settle and a
disposal any more, because there is neither**: what finish does is a recursive
delete and one path-scoped commit.

So the record's own reopen condition is met, and by the consumer it was written
about. It read: *reopen the destruction rejection if a consumer appears that
needs whole-root destruction **without an external effect to coordinate with** —
a destroy whose only verdict is the filesystem's own.* That is exactly the
operation that now exists. It has no coordinator, no callback, no revalidation
points and no caller obligation, and none of the three arguments reaches it.

**The receipt argument survives, correctly, and is not what it was thought to
decide.** Grove's proof that a teardown happened is still the deletion commit in
the repository's own history, and no artifact inside the container could ever
attest that. What that decides is *where the receipt lives* — outside the
container, in the VCS — and not *who performs the removal*. `Removed` is a
**postcondition and not a receipt**: it says what is gone, which is what lets
grove name those paths in the commit message it writes afterwards. Conflating
the two is what made the old record read as an argument about ownership.

**Creation was rejected on depth, and the depth measurement was of a different
operation.** It counted what would move out of grove — about thirty lines of
`std::fs` with no algorithm behind them — because at the time root creation
meant a `create_dir_all`, a hand-written charter and a format witness published
last by a same-directory rename. Migration and the format taxonomy went at
`delete-migration-k6`, and what `initialize` actually is has no resemblance to
that list: a plan of creates through the one interpreter, checked by the same
algebra, unwound by the same rollback, with the root's own create as the single
step outside it. The interface gained one concept, the **vacancy** — and the
opening shape needed that anyway, because *is there a tree here* is answered as a
shape rather than as a predicate.

## The trade-off

**What it buys is the guarantee the whole decomposition rests on: the store is
the only thing that touches the task tree.** That claim fails at the first
operation of every fresh tree if the consumer writes the root's own content
itself — outside the lock and outside the store — and it fails again at the last
operation of every tree if the consumer removes the root itself. Those were the
two holes, and they are the two halves of this record.

**It costs nothing at the seam, which is the cost that would have mattered.**
`initialize` takes bytes and a name the trait already supplies, exactly as
`promote` does when it moves a leaf's content into a new node's distinguished
child; `delete` takes nothing at all. Neither adds a trait method, so
[`entry-name-is-the-only-seam`](entry-name-is-the-only-seam.md) holds unchanged
and is more load-bearing rather than less.

**It costs one honest asymmetry.** Every other mutation reports names, and
`delete` reports **paths** — because it acts on the root and therefore on
everything beneath it, including the entries the domain declined to parse as its
own, which have no name any bucket of `N` could carry. That asymmetry is the
operation's and not an oversight, and it is stated wherever the type is.

**And it costs a promise this library makes everywhere else.** *After a mutation
returns an error, either every effect landed or none did* is a property of
plans, and a removal has nothing to put back — so a stopped deletion reports how
far it got instead of claiming the tree is as it was found. That is a real
weakening of the library's uniformity, accepted because the alternative is
copying a tree aside before destroying it, which is a durable record of a
pre-operation state: the thing a version control system already is.

**The evidence that the contract is domain-free was taken before the reversal
and stands unchanged.** `docs/formalism-findings.md` entry 047 enumerated the
prototype's 175 non-keyword identifiers and found **zero** naming a workstream,
a task, a session, a commit, a bookmark, a ticket or a witness, against 103 and
70 for two grove model families. The old record accepted that measurement and
overrode it on the coordinator; with the coordinator gone there is nothing left
to override it with. The instrument is deleted and the figures are quoted rather
than re-runnable, which
[`evidence-outlives-the-instrument`](evidence-outlives-the-instrument.md)
records the rule for.

## Considered options

- **A finish-private adapter inside grove** — the option this record previously
  chose. Rejected now because it puts a second writer back into the task tree:
  grove would hold its own lock, do its own walk and its own removal, which is
  precisely the second lock layer `collapse-tree-access-k13` deleted.
  With the coordinator gone the adapter has nothing left to adapt, and what it
  would carry is a duplicate of the store's own walk that no conformance kit
  covers. Reopen if grove ever again needs to interleave an external effect with
  the removal — see the next option, which is what that would actually require.
- **A destroy that consults the caller mid-operation** — the four-point
  coordinator, in any form. **Still rejected, on the original argument, which
  this reversal does not touch.** A caller consulted from inside the transaction
  can only be refused; the obligation it needs is one grove's own contract
  requires it to break; and owning a coordinated destruction would mean the
  library knowing what *some* reserved names mean, which splits the
  reserved-name class into a library-owned half and a consumer-owned half. What
  the store gained is a destroy whose only verdict is the filesystem's own, and
  a consumer needing the coordinated form still does not get it here.
- **`std::fs::remove_dir_all`, and no report.** Rejected because the caller
  cannot then say what it destroyed, which is the whole of what grove needs from
  the operation: its commit message names the paths. The standard library's own
  call is race-resistant in a way the reporting walk is not — it descends with
  `openat` — and that trade is stated where the walk is, rather than hidden: the
  window it leaves is the one an advisory-lock-ignoring writer already has, and
  a consumer that wants the race closed and no report has that call available.
- **A `Removed` keyed by `N`, like every other report.** Rejected because it
  cannot describe what the operation removes. Foreign entries go with the root
  and the domain has no name for them, so a third bucket of names would report
  less than the paths do while looking like it reported more.

## Why this is hard to reverse

Both operations are on the library's public surface, and the second is
destructive: a consumer that has moved its teardown into the store has no
tree-touching code left to move back. Grove's own finish cycle is the case —
after `loop-crate-verbs-k21` wires it up, the only thing that removes `.grove/`
is this operation, under the store's lock, and grove's part is the commit that
follows.

It is also expensive to re-take rather than merely awkward: the argument this
record reverses was made against a prototype that no longer exists, and re-taking
it in either direction now means rebuilding an instrument first. That is why the
whole chain — what was argued, what became of the machinery, and which reopen
condition fired — is kept above rather than compressed to its conclusion.

## What would reopen it

A consumer whose root lifecycle genuinely has to interleave with an external
effect, so that the removal must be *coordinated* rather than merely performed.
That consumer needs the second option above, and the argument against it is
untouched by this reversal — so what it reopens is the coordinator, not this
record's decision about where an uncoordinated create and destroy live.

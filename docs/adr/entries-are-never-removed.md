# Entries are never removed

`ordinal-fs-tree` offers no operation that removes an **entry**. A fresh key is
`max(key over the whole tree) + 1`, so the names on disk **are** the counter;
deleting an entry lowers the visible maximum and the next allocation re-issues a
key that other entries may still reference. A domain that needs entries to be
*retired* marks them through an attribute and leaves them in place, which costs
nothing because attributes are the consumer's and the library never reads them.
A domain that needs them to genuinely disappear needs a key source that is not
derived from the tree, and that is outside what this library does.

**Deleting the *root* is a different operation, and the library offers it.**
`WriteGuard::delete` removes the root and everything beneath it. The argument
above is entirely about what the **next allocation** would do, and after a root
deletion there is no next allocation: the tree is gone, and with it every name
the counter was derived from. That is also why that operation is the whole tree
or nothing — a partial version of it would be entry removal wearing another
name, and would land in exactly the state this record forbids. The clause is
here because this record's opening sentence used to say the library offered no
removal at all, and a reader would carry that to the wrong conclusion about
`delete`.

[`docs/ordinal-fs-tree/ARCHITECTURE.md`](../ordinal-fs-tree/ARCHITECTURE.md)
states the rule and its consequence under *Keys are the counter*. This record
carries what that document does not: the alternative key source that was
rejected, and why adding entry removal later is not an additive change.

## The trade-off

Deriving allocation from the names is what makes the whole proposition — *a tree
you can read with `ls`, edit with `mv`, and reason about without running the
program that owns it* — survive a hand edit. There is no index, no database and
no metadata file, so there is nothing a human with `mv` can desynchronise. The
two formal models under `docs/ordinal-fs-tree/models/` — which survive, this
being the library's decision and not grove's — take the same premise: an
arbitrary well-formed tree is *reached* through hand edits rather than posited as a second initial state, which is only
coherent because every fact about the tree is recoverable from the names.

The cost is a monotonically growing key space and entries that outlive their
usefulness. That is accepted rather than mitigated: the keys are integers, the
retirement marker is an attribute the consumer already controls, and a tree
large enough for the growth to matter has a different problem.

grove pays this cost today and its glossary records the same rule from the
consumer's side — a leaf is retired by a `DONE` or `ABANDONED` mark and never by
deletion, because deleting one lowers the max and the next allocation re-issues
a live key. That the rule survives being restated in a domain vocabulary that
shares none of the library's words is the evidence that it belongs to the
allocation scheme rather than to grove. Grove draws the root/entry line from its
own side too, and in the same place: its finish cycle destroys the whole task
tree and nothing short of it.

## Considered options

- **A key source not derived from the tree** — a counter file, or a header in
  the tree's own distinguished child. Rejected because it is a second source of
  truth that a hand edit desynchronises silently, and silently is the whole of
  the objection: a counter file that disagrees with the names produces duplicate
  keys at the next allocation with nothing to notice at the moment of damage.
  Reopen if a consumer appears for which hand editing is genuinely out of
  scope — a tree written only by a service, never by a person — for which the
  counter's failure mode cannot arise.
- **Removal with tombstones** — leave a marker entry behind so the maximum does
  not fall. Rejected as removal in name only: the tombstone is an entry, it
  occupies a name, and the consumer's attributes already express exactly that at
  no cost to the library. It would also make *is this a real entry* a question
  the library must answer, which means reading parts it is not entitled to
  understand.
- **Removal that refuses when it would lower the maximum.** Rejected because the
  refusal is unpredictable from where the caller stands — the same call succeeds
  or fails depending on an unrelated entry elsewhere in the tree — and because
  it leaves the guarantee true only until someone removes the highest-keyed
  entry. Reopen never; a conditional invariant is not one.

## Why this is hard to reverse

Removing an entry cannot be added without changing allocation, and changing
allocation changes what every existing tree means. (Root deletion is not in
this paragraph's scope, and adding it changed no allocation at all — which is
the mechanical form of the distinction above.) Any key source that tolerates deletion
must be able to say *what has already been issued*, which the names alone cannot
once entries can vanish — so the change is not an added operation but a new
source of truth, applied retroactively to trees whose history is gone. Every
durable cross-reference a consumer has stored is stated in keys, and their
uniqueness is the only reason those references resolve.

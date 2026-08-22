# records-k5

## Goal

Settle where `ordinal-fs-tree`'s decision records live, and file the two that
have now earned one.

## Context

`plan-k1` applied the ADR when-to-write test and recorded **No ADR** — every
candidate failed the *hard to reverse* limb, because the trait design was intent
rather than a landed decision. The root brief scheduled a re-application "at the
`architecture-k2` close", and `operations-model-k4` performed it. Two decisions
now clear all three limbs:

- **The single-trait seam.** All genericity is one trait over the entry name.
  Rejected: a `Domain` trait with associated functions for lock scope and
  moving (callbacks respelled), and a two-trait split by layer. Hard to reverse
  because it is the library's whole public surface and two checked models plus
  `docs/ordinal-fs-tree/ARCHITECTURE.md` are built on it.
- **No removal operation.** Keys are `max + 1` over the names, so the names
  *are* the counter and deleting an entry re-issues a live key. Rejected: a key
  source not derived from the tree, which is a second source of truth a hand
  edit desynchronises. Hard to reverse because removal cannot be added without
  changing allocation, and changing allocation changes what every existing tree
  means.

Two more were tested and **rejected**, so do not re-litigate them: *locking is
invisible* and *a rename is a rename* both fail *hard to reverse* — neither
appears in any signature, and the architecture document is where they land.

## The question that actually blocks filing

`CONTEXT-MAP.md` requires every record under `docs/adr/` to have a **maintaining
context** recorded there, and this repo has two: `grove` and `skills`. Neither
obviously owns a record about a domain-independent tree library. The root brief
already carries the open question — *whether `ordinal-fs-tree` becomes a third
bounded context with its own glossary* — and notes the evidence for it: a
deliberately separate vocabulary (entry, ordinal, key, distinguished child) that
shares no term with grove's, which is exactly the criterion `CONTEXT-MAP.md`
uses to tell contexts apart.

The evidence against, which the answer has to face: a context in that map "ships
by a path" and has a glossary, and `ordinal-fs-tree` today is a design document
and two models with no crate and no glossary file. `ADR-FORMAT.md`'s split rule
also points the other way — a context occupying the repo root (grove does) is
the case where the set **stays flat**, because splitting exiles the nested
context's records while leaving the root set just as mixed.

So the three answers to choose between are: **grove maintains them** (the sole
consumer, and the extraction is grove's workstream); **declare the third
context** and give it a glossary now; or **defer again** and let the records
wait for the crate to exist. The third is the status quo and needs an argument,
because a decision recorded nowhere durable is one the flip increment will
re-derive.

## Done when

- The maintaining-context question is answered, in `CONTEXT-MAP.md`, either way.
- The two records above exist, wherever the answer puts them, against
  `ADR-FORMAT.md`'s minimal shape — slug title, the decision, the trade-off, the
  rejected alternative.
- Every citation added is reconciled: the root brief's open question is closed
  out rather than left reading as open.

## Notes

`docs/ordinal-fs-tree/ARCHITECTURE.md` already explains both decisions at
length and moves with the crate when it is extracted. Say explicitly what the
ADR carries that the document does not, or the record is the same decision
written twice — which is exactly the ground `CONTEXT-MAP.md` records for having
folded two earlier specs into `docs/ARCHITECTURE.md`.

## Decisions (running log)

**`ordinal-fs-tree` is a third bounded context, declared now.** The three
candidate answers were *grove maintains them*, *declare the third context*, and
*defer again*; the third fails on the leaf's own argument (a decision recorded
nowhere durable is one the flip re-derives) and the first fails on the glossary
rule. Ownership names who keeps a record current, and these two records' subject
is the library's public surface and its key-allocation rule — recording grove as
their maintainer says the extraction did not happen, which is the opposite of
increment 1's premise. Rejected explicitly, not by default.

**The evidence the brief cited was wrong, and the real evidence is stronger.**
The root brief argued a "deliberately separate vocabulary … that shares no term
with grove's". It shares two: `CONTEXT.md` defines **Leaf** (a task file
`NN-[DONE-|ABANDONED-]<kind>-<slug>-k<key>.md` executed in one session) and
**Node directory** (a directory headed by a `BRIEF.md` charter), while
`docs/ordinal-fs-tree/ARCHITECTURE.md` defines *leaf* as any regular-file entry
and *node* as any directory of children, with no charter and no session. Two
more are the same concept under different words — grove's **Position** is the
library's *ordinal*, grove's **Permanent key** is the library's *key*.
`CONTEXT-FORMAT.md` requires a term be defined in its owning context's glossary
and **never both**, so grove's glossary cannot host the library's vocabulary
without defining `leaf` twice with incompatible meanings. A word that means two
things in one repository is the criterion a context boundary exists to answer,
so the collision decides the question the disjointness could only suggest.

**The placement question and the ownership question are independent, and this
leaf's framing conflated them.** `ADR-FORMAT.md`'s split rule was cited as
evidence against the third context. It is not: it governs whether `docs/adr/`
splits into several directories, and its *stay flat* branch ends by saying
`CONTEXT-MAP.md` records which context **owns** each record with slugs unique
repo-wide. That is the branch taken here — grove occupies the repo root, so the
set stays flat — and it is what makes a third *maintaining* context cost no
second search path. Nothing in the split rule bears on how many contexts exist.

**A bounded context is a language boundary; a product is a delivery path.**
`CONTEXT-MAP.md` opened by conflating them ("They ship by different paths"),
which is true of the two existing contexts by coincidence and is the whole of
the *evidence against* — no crate, no shipping path. Corrected in place rather
than worked around: the map now says what makes a context, and records that this
one is declared on vocabulary with its delivery still open. `library-k6` decides
where the crate lands; that answer changes a path in the map, not the ownership.

**The glossary is written now, and lateness was the argument for it, not
against.** `CONTEXT-FORMAT.md`: record a term inline, never batched — "a term
resolved and not written down is a term the next session re-resolves
differently". The terms were resolved by `architecture-k2` and written into a
*design document*, which is not a glossary and is not read as one. The root
brief legislates the boundary by hand ("grove's own words … must not appear in
the library"), enforced today only by whoever remembers reading it;
`docs/ordinal-fs-tree/CONTEXT.md` makes that rule an artifact the implementation
leaves can check themselves against. It sits beside `ARCHITECTURE.md` and moves
with the crate for the same reason.

**What the two ADRs carry that the architecture document does not: the rejected
alternatives, and the reversal cost.** Checked rather than asserted —
`docs/ordinal-fs-tree/ARCHITECTURE.md` names rejected alternatives for the
*plan/interpreter* shape (pure functions over name lists; read-transform-diff)
and for nothing else. It never names the `Domain` trait with associated
functions, the two-trait split by layer, or a key source not derived from the
tree. Those live only in `.grove/02-architecture-k2/BRIEF.md`, and the complete
finish cycle **deletes `.grove/`** after promoting durable artifacts — so
unfiled, they do not survive this workstream. The document says what the design
*is*, for someone building against it; the records say what it *cost* and what
changing it would cost, for someone proposing to change it.

**Two rejected decisions stay rejected.** *Locking is invisible* and *a rename is
a rename* were re-tested at this close and still fail *hard to reverse* — neither
appears in a signature, so neither is re-litigated here.

**No findings-log entry.** `docs/formalism-findings.md` takes an entry per
session that reaches for a formalism; this leaf reached for none. Recorded so the
absence reads as a decision rather than an omission.

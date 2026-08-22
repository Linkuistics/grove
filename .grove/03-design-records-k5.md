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

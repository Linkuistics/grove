# architecture-k2 — brief

## Goal

Settle the architecture of `ordinal-fs-tree` — and settle it *against checked
formal models*, not against prose reviewed by eye. The architecture is drafted
and agreed in shape; what remains is validating it, and correcting it where the
models disagree.

## Done when

- `docs/ordinal-fs-tree/ARCHITECTURE.md` describes an architecture that the
  structural and behavioural models both accept — every operation total, every
  stated invariant actually preserved, every trait obligation expressible.
- The structural model (Alloy) and the behavioural model (Quint) both exist,
  both run, and their findings are reconciled into the document rather than
  noted beside it.
- The operation set is fixed well enough that implementation leaves can be cut
  one per operation.
- `docs/formalism-findings.md` carries an entry per modelling episode.

## Decomposition

Structural before behavioural, and the ordering is load-bearing rather than
arbitrary: the structural model settles *what a well-formed tree is*, which is
the predicate the behavioural model's invariants are stated against. Modelling
the operations first would mean stating them against a well-formedness notion
that had not yet been checked.

It is also where the risk is. Every defect entry 001 of the findings log
records is structural — a trait that could not name its own output, an invariant
that was not one. That is evidence about which half of this design is
under-examined.

## Pointers

- `docs/ordinal-fs-tree/ARCHITECTURE.md` — the drafted architecture. The
  specification of record for anything not yet modelled, and the thing the
  models are checking.
- `docs/formalism-findings.md` — the method log, its entry format, and the three
  hypotheses under test. Append before retiring.
- The behaviour reference, in the current implementation — read for *what it
  does*, never for how it is structured: `tree_id` (name grammar), `tree_read`
  (walk, ancestor chain, resolution), `tree_grow` (append, insert-with-shift),
  `tree_lifecycle` (promotion, attribute rewrite), `tree_access` (lock guards).
  Migration is out of scope. `tree_rename` is out too — see below.

## Decisions settled, and what was rejected

Recorded so the modelling sessions do not re-litigate them. Rationale beyond
this lives in the architecture document.

**Operations produce a plan; one interpreter applies it.** Internally a mutation
is snapshot → pure algebra → plan → apply. Rejected: pure functions over name
lists (the current implementation's shape — leaves the shift-ordering rule inside
filesystem code where nothing can model or test it), and read-transform-diff
(makes the diff a second thing to get right). This is what makes the shift order
a stated property rather than an accident of a loop's direction, and it gives
every operation one rollback instead of each hand-rolling its own — the current
`leaf-insert` has none at all, while `leaf-add-pair` does.

**Snapshot scope is internal, not interface.** Whole-tree for now; laziness would
be an invisible refinement. The public surface is verb-shaped and returns a
*report* of what happened, never a plan to apply.

**There is no `Domain` trait — the name type is the whole seam.** All genericity
is one trait over the entry name. Rejected: a `Domain` trait with associated
functions for lock scope and moving (those are the callbacks ruled out, merely
respelled), and a two-trait split by layer. A side benefit: with no `D`
propagating through every type, the spurious `D: Clone` derive bounds the
requirements session predicted never arise.

**A name is isomorphic to `(ordinal, key, parts)`.** `parts` is opaque — label
plus domain attributes — and determines the species. The library never parses,
never formats, and holds no strings; the sibling shift is derived rather than
implemented. Rejected: allocate-a-slot-then-create (two calls with a window), and
a placeholder name the library rewrites (requires the name type to have states
that are not valid names).

**Locking is invisible.** The library locks the directory *containing* the tree
root, internally, with no parameter — grove's reason for that scope generalises,
since the container outlives both the root's creation and its deletion. Unix-only
build via `rustix` (no `unsafe`, no `libc`); the interface says nothing about
locking, so another platform later changes no signature. Rejected: a supplied
lock value, and a portable lockfile — no crate can lock a *directory* portably,
because `LockFileEx` needs `GENERIC_READ`/`GENERIC_WRITE` on the handle and a
Windows directory handle cannot carry them.

**A rename is a rename.** No VCS awareness, no subprocess, no `git2`, no `gix`.
The current `tree_rename`'s own header establishes that both its branches commit
byte-identical trees, so `git mv` buys only a prettier pre-commit `git status`
on the git lane — which a jj tree never takes. Whole-crate dependency list:
`rustix`. Departure from current behaviour, deliberately: a git-lane grove user
sees delete+add before `git add` instead of a rename.

**No removal operation.** Keys are `max + 1` over the names, so the names *are*
the counter; deleting an entry lowers the visible maximum and re-issues a live
key. A domain needing removal needs a key source not derived from the tree.

**Nodes hold 0..n children.** No invariant about a node having children, and no
notion of which entry is "next" or "interesting" — those are consumer queries
over consumer attributes, answered by a predicate the library never inspects.

## Notes

Where the design departs from the current implementation's structure, the
architecture document says so and why — that is signal for the flip increment,
not noise.

One departure worth carrying forward explicitly: grove's guards do three things
at once — acquire the lock, refuse pending transaction witnesses, check the
format witness. The library's `open` does only the first; the other two become
domain validations grove composes after opening. That converts a guarantee into
a discipline, which is part of why the parse verdict has a `Reserved` outcome
that halts on the domain's behalf.

Anything that turns out to need its own session gets a leaf rather than being
absorbed here.

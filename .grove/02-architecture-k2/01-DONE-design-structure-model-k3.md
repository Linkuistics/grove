# structure-model-k3

## Goal

Model `ordinal-fs-tree`'s **structure** in Alloy, and check whether the data
model in the architecture document is coherent — whether it admits anything
nonsensical, and whether it can represent everything the operations need.
Reconcile whatever it finds back into the architecture document.

Structure only. The operations belong to `operations-model-k4`; the seam between
the two leaves is *"is this shape well-formed?"* versus *"does this operation
preserve it?"*.

## Context

- `docs/ordinal-fs-tree/ARCHITECTURE.md` — the subject. Sections **The model**,
  **Names belong to the consumer**, **The seam** and **Invariants** are what this
  leaf checks; **Operations** is the next leaf's.
- `docs/formalism-findings.md` — read the entry format and hypothesis **H1**
  first, and append entry 002 before retiring. Record what Alloy *missed* as
  carefully as what it caught, and fill in the counterfactual field honestly.
- Put the model at `docs/ordinal-fs-tree/models/structure.als`, so it moves with
  the crate when it is extracted.

## Done when

- `structure.als` exists, runs, and its `check`s pass or their counterexamples
  have been acted on.
- Every structural claim in the architecture document is either verified or
  corrected in place — corrections go **into** the document, not beside it.
- `docs/formalism-findings.md` carries entry 002.

## What to check

Not exhaustive — anything the modelling turns up is in scope. These are the
claims currently resting on nothing but prose.

- **Key uniqueness.** No key twice, tree-wide. Also: does the model *admit*
  duplicate keys as a well-formed input? It should, because a hand-edited tree
  can contain them and the library never checks — so the question is what
  `by_key` means then.
- **Ordinal distinctness, and the density question.** Distinctness is claimed;
  density is claimed to be *preserved but not established*. Show a gapped level
  that is well-formed. If Alloy cannot distinguish the two claims, the document
  is still being vague.
- **The distinguished child.** At most one per node; carries neither ordinal nor
  key. This is where entry 001's blocking defect was — `compose` could not name
  one — so check the *fixed* trait actually closes it: is every entry the library
  must ever name reachable from the trait's constructors?
- **Name isomorphism.** `(ordinal, key, parts)` is claimed to represent every
  positioned entry, with the species following from `parts`. Is that a bijection,
  or does it admit triples that name nothing, or entries no triple names?
- **The parse trichotomy.** `Entry | Foreign | Malformed | Reserved` — total and
  disjoint? A name that could be classified two ways is the failure mode the
  whole rule exists to prevent.
- **Species agreement.** A name declaring a leaf is a file; a node is a
  directory. What does the model say about a name that declares one and is the
  other — is it representable, and is it distinguishable from a foreign entry?

## Notes

Bias toward trying to make the model produce a **bad tree that satisfies every
stated invariant**. That is the useful failure: it means the invariants are
too weak, which prose review cannot detect at all.

If part of the design resists being modelled, that is a design finding to record
and act on — not a modelling problem to work around.

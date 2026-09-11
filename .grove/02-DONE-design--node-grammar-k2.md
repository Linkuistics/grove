# node-grammar-k2

## Goal

Rework the design records so that they state the node grammar
`NN-k<key>/_<slug>.md` (root `_BRIEF.md`) as the only one, for both bounded
contexts, before any code moves. Deliverable: the ADR set, the specs and the
two glossaries current for the new grammar, and the `ordinal-fs-tree`
architecture and models re-drawn for the obligation the reader now enforces.
The interview has happened — synthesise `plan-k1`'s decision log; do not
re-interview.

## Context

- `plan-k1`'s ten decisions, each with the human's words and the evidence.
  Decisions 1–5 fix the grammar; 6 forbids naming history; 7 keeps *No
  migration*; 9 fixes the seams.
- `docs/adr/task-names-are-canonical.md` — the record that fixes the
  directory spelling. Rework in place: state `NN-k<key>` for a node directory
  and `_<slug>.md` / `_BRIEF.md` for its file, keep canonicity, and drop the
  history it currently narrates.
- `docs/adr/iteration-reuses-the-existing-species.md` — its argument that a
  repeated series *is* "sibling node directories sharing one slug" now reads
  the slug off each sibling's `_` file; restate it so it still holds.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, `models/structure.als`,
  `models/operations.qnt` — the models lead: change the model, run its runner,
  and only then plan the code; record any disagreement in
  `docs/formalism-findings.md`. The claims in play: `EntryName::distinguished`
  names one fixed name per domain and *at most one distinguished child per
  node* is a theorem the filesystem supplies
  (`DistinguishedIsUniquePerNode`, `witness_two_distinguished_children`,
  `HAS_DISTINGUISHED`). Under the new grammar the distinguished child's name
  is supplied per node by the consumer and carries the node's label, and
  exactly-one-per-level becomes an obligation the reader enforces (none or
  two is malformed). The design says how the consumer supplies that name to
  `promote` and to initialization, and where the label lives — the library
  owns no vocabulary and never reads a label, so decide whether a node's
  `Parts` carry it or grove composes the handle from two names.
- `docs/specs/module-decomposition.md` decisions 3 and 4 — the grammar and the
  handle's single owner. A node's handle `<slug>-k<key>` is now composed from
  the folder's key and the `_` file's slug; say where that composition lives
  and that it is still read from names, never contents.
- `CONTEXT.md` (*Node directory*, *Work-item handle / title*, *Permanent key*,
  *No migration*, *Taskless root*), `docs/ordinal-fs-tree/CONTEXT.md`
  (*Distinguished child*, *Label*), `CONTEXT-MAP.md`'s collision table. Land
  the term for the node's own file. **Keep every `<a id>` anchor** — the
  walkthrough books reserve them and a renamed anchor breaks a book silently;
  retitle headings freely.
- `docs/specs/walkthrough-books.md`, *An accepted source change* — for what
  planning's leaves will owe the books; the design touches no book root.

## Done when

- The ADR set is a minimum coherent set for the new grammar and no record
  narrates the rename.
- The library's architecture and both models state the per-node distinguished
  name and the reader-enforced uniqueness, and their runners pass — or the
  disagreement is recorded where the models say to record it.
- `module-decomposition.md` states the grammar and the handle's composition
  for a node.
- Both glossaries and the collision table carry the term for the node's file;
  every anchor that existed still exists.
- Nothing under `crates/` changed. A review chain is cut or explicitly
  declined in the commit message; if cut, its steps are `leaf-insert`ed ahead
  of `node-grammar-k3`, which consumes the reviewed design.

## Notes

- The records will describe a design the installed binary does not yet
  implement. That is what a decision record is; say nothing about the gap.
- This is a load-bearing grammar others build on for years — the bar for a
  review chain is met on that ground alone, whatever the diff's size.

## Decisions (running log)

**1. The node file owns the title.** Use *Node file* for `_<slug>.md`, with
`_BRIEF.md` at the root; *brief* names its body. A node's positioned `Parts`
carry no slug. Grove's name module composes its handle from the parsed folder
key and parsed node-file slug, so neither value is copied or read from content.

**2. A name argument and a level check keep the seam in one trait.** Promotion
takes the consumer's distinguished name; initialization takes an optional
name-and-bytes pair. The library checks at most one distinguished child per
level; `EntryName::validate_distinguished` checks the complete distinguished-name set's cardinality
and suitability for the containing node (or root). Grove requires exactly one
and distinguishes the root marker from a titled node file. The same check runs
on a read level and the projected result of a plan before effects, including
ordinary node creation. Canonical rendered filenames distinguish two
distinguished names in `same_name`; the library interprets no title.

**3. Review is tree work.** `node-grammar-k4` is the `review-design` leaf ahead of
`node-grammar-k3`. The review is scheduled, so this producer spends no in-session
reviewer. The reviewer creates an integration only for actionable findings.

**4. Verification.** Both model runners pass: all 28 Alloy commands and all
217 Quint claims across ten instances at the runner's standing budgets.
Disabling only projected-final-level validation produces a successful batch
append with a required node file missing; the restored check refuses that
same input. `docs/formalism-findings.md` entry 049 records the control and the
models' limits. All eight principal checks in `scripts/check.sh` pass, including
all six books. The existing glossary and architecture anchors are preserved;
no file under `crates/` changed. No book source root changed in this leaf.

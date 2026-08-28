# store-operations-k12 — brief

## Goal

Add the four operations `ordinal-fs-tree` is missing — a search that matched
nothing, an opening that answers *is there a tree here* **as a shape**, root
initialization, and root deletion — so that grove's second lock layer can be
deleted and the store can become the only thing that touches the task tree.

## Done when

- `Sought`, `Reading`/`Writing`/`Vacancy`, `Vacancy::initialize` and
  `WriteGuard::delete` are all present, exercised through the crate's own public
  interface without grove, and documented in the crate's own architecture and
  models.
- `entry-name-is-the-only-seam` still holds with **no new trait method**.
- `entries-are-never-removed` carries its distinguishing clause.
- Nothing in grove has been changed to use them yet — that is
  `collapse-tree-access-k13`, a sibling of this node, and it is deliberately
  outside this subtree because it is grove-side work in grove's vocabulary.

## Decomposition

Three children, ordered by independence rather than by dependency — none blocks
another, and each lands green on its own.

1. `sought` — the word for a search that matched nothing. Smallest, touches the
   snapshot surface only.
2. `open-shape` — `Reading` / `Writing` / `Vacancy` and `initialize`. The
   opening reshape and root creation are one type story and land together.
3. `root-delete` — `delete` and `Removed`, plus the ADR clause the operation
   obliges.

This node exists because the store is a **separate bounded context**: it has its
own glossary, its own architecture document and its own formal models, and all
three of these leaves touch all three artifacts. A flat run of siblings under the
grove root would have left that shared context with nowhere to live.

## Pointers

- Interface, stated and not to be redesigned: `docs/specs/module-decomposition.md`,
  decision 2.
- ADRs a session here must read: `docs/adr/entry-name-is-the-only-seam.md`
  (unchanged and **more** load-bearing under this design),
  `docs/adr/entries-are-never-removed.md` (gains one clause),
  `docs/adr/grove-does-not-stage-its-own-renames.md` and
  `docs/adr/bulk-marks-are-not-atomic.md` (both to be **re-checked** against a
  store that now owns root creation and deletion — re-checked, not assumed).
- The crate's own documents, which are part of the deliverable and not a
  follow-on: `docs/ordinal-fs-tree/CONTEXT.md` (glossary),
  `docs/ordinal-fs-tree/ARCHITECTURE.md`, `docs/ordinal-fs-tree/CLI.md`, and the
  formal models `docs/ordinal-fs-tree/models/structure.als` and
  `docs/ordinal-fs-tree/models/operations.qnt`.
- Glossary terms in play: entry, root, guard, snapshot, refusal, report — and the
  new ones each leaf introduces. `CONTEXT-FORMAT.md` governs how a term is
  recorded; `CONTEXT-MAP.md` governs the vocabulary boundary between this crate
  and grove, and is the model the other extractions follow.
- Test seam: the crate's own public interface, exercised without grove
  (`docs/specs/module-decomposition.md`, `## Test seams`, seam 1), plus the
  existing `conformance` kit that holds a consumer to the round-trip law.

## Notes

**The store keeps its own vocabulary.** Every name added here must be sayable
without grove's words — no *task*, no *leaf*, no *kind*, no *session*. `Sought`
is the worked example: grove's current no-work signal is `Option<SelectedLeaf>`,
whose predicate is grove vocabulary and **cannot move as-is**, so the store gets
a domain-free word for *found nothing* and grove uses it.

**The `cli` feature and the `syllabus` binary are part of the crate's surface.**
An operation added to the library is not finished if the crate's own CLI and its
documentation still describe a store without it.

**Extraction to a separate repository is out of scope** and deferred; its
documents stay at `docs/ordinal-fs-tree/` where four artifacts already link to
them. What *does* change, and belongs to `spec-to-current-state-k23` rather than
here, is the release manifest exclusion that kept the crate out of the release
cut.

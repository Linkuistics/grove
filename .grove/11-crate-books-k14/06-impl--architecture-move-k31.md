# architecture-move-k31

## Goal

Move `docs/ARCHITECTURE.md`'s descriptive account into the overview, leaving that
document carrying decisions, constraints and measurement records only, with every
citation of a moved anchor re-pointed and every link suite green.

## Context

- Decision 5 of `plan-k1`, as amended by `plan-k8`: the move is mechanical
  **after** the anchor resolver is widened, and `architecture-anchors-k19` is what
  widened it. If that leaf did not land, this one cannot be done safely — the
  Rust-source citations would move silently.
- The guards, and what each one catches:
  `every_repository_markdown_reference_resolves` sweeps every Markdown file in
  the repository; `architecture-anchors-k19`'s check covers the same anchors in
  Rust sources; and the `include_str!` content assertions in
  `crates/grove-llm/tests/composition_guidance.rs` read `docs/ARCHITECTURE.md` in
  eight places and **go red on this move by design**. Turning those red tests
  green is how the citations are re-pointed — not a hopeful sweep.
- What moves and what stays is settled, and it is narrower than this leaf was
  cut expecting. `docs/specs/overview-book-structure.md`, *What this book
  absorbs from `ARCHITECTURE.md`*, is the charter. **Three sections are in play**
  — *Runtime flow*, *Command surfaces*, *Main module seams* — and the six other
  descriptive-looking sections stay whole, because they describe other crates'
  internals and leave with those crates' books (`architecture-residue-k75`).
  Within the three, the rule is a positive move test with the default to stay: a
  clause moves iff it states what the system does, is shaped like, or reaches;
  everything else, including anything not confidently classifiable, stays. None
  of the three is "descriptive throughout" — the brief lists the residue each
  keeps, and this leaf should expect to leave text behind in all of them.
- Anchors are the contract, and the brief has decided them: **every anchor stays
  above its surviving argument, and no citation is re-pointed**, in Rust or in
  Markdown. The measurement behind that is in the brief — no real design citation
  points at description. Each stripped section opens with exactly one forward
  pointer into the overview page now carrying its description; that link is
  inbound to the book, so the book contract does not bind it and
  `every_repository_markdown_reference_resolves` checks it.

## Done when

- The description in *Runtime flow*, *Command surfaces* and *Main module seams*
  has moved into the overview under the brief's rule, and each of those sections
  opens with its forward pointer.
- Every surviving descriptive passage in the six sections not in play is marked
  with the book that will make it redundant, so `architecture-residue-k75`
  inherits a list rather than a judgement. **The decisions-only end state is not
  this leaf's; it falls due at k75.**
- The *Documentation ownership* table's own row is narrowed to the decisions,
  the constraints and the measurement records, in the wording the brief gives
  under *The book's row in the ownership table*. The overview's row is
  `overview-book-k30`'s and should already be there.
- No anchor moves and no citation is re-pointed;
  `every_architecture_anchor_citation_in_a_source_resolves` and
  `every_repository_markdown_reference_resolves` are green with no source edit.
- `bash scripts/check.sh` passes, with
  `crates/grove-llm/tests/composition_guidance.rs` green against re-pointed
  citations rather than against loosened assertions.
- The overview's own final validation still passes: moving prose into a book must
  not break its fragment graph.

## Notes

**Loosening an assertion is not re-pointing a citation.** The `include_str!`
checks go red because they assert content at a path; the fix is the path or the
content, never the assertion's strictness. A test weakened to accommodate the
move deletes the only guard this leaf has.

**A finding against a section does not reach the summary layer.** The ownership
table, this document's opening, and `CONTEXT-MAP.md`'s account of what
`docs/ARCHITECTURE.md` holds all summarise sections; correcting a section leaves
them stale and reading as descriptions.

**Verify the sweep, do not trust it.** A clean grep for a moved anchor proves
nothing on its own — run a positive control that finds an anchor you know is
present, and watch a deliberately broken citation come back dirty, before
crediting the clean read.

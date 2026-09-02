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
- What moves and what stays is the hard part and it is not a clean section split.
  *Runtime flow*, *Command surfaces* and *Main module seams* are descriptive
  throughout. *Task-tree data model*, *Task kinds and composition*, *Lifecycle
  and resumption*, *Human authority and completion*, *Version-control seam* and
  *How the methodology reaches a session* interleave description with the
  decisions and their trade-offs, and only the description moves. The structure
  brief from `overview-structure-k29` is what settles the boundary.
- Anchors are the contract. The document's own *Documentation ownership* section
  records that the former decision-record slugs are explicit `<a id="…">`
  anchors, that source comments and tests cite them as compact design references,
  and that changing a section title does not change the anchor. An anchor whose
  section moves has to keep resolving somewhere, or every citation of it has to
  move with it — decide which, per anchor, and say so.

## Done when

- `docs/ARCHITECTURE.md` carries decisions, constraints and measurement records
  only, and its *Documentation ownership* table describes what is now true —
  including the overview's own row.
- Every citation of a moved anchor resolves, in Markdown and in Rust sources.
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

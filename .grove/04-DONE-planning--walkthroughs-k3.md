# walkthroughs-k3

## Goal

Decompose the documentation arm and the publishing pipeline — products P1 and
P2 of the root brief — into leaves that each fit one session and each land
something independently useful.

## Context

The requirements are settled and must not be re-elicited; the root brief and
`plan-k1`'s decision log are the input. What is *not* settled is the shape of
the work, and that is this session's whole job.

## Done when

The subtree is cut, each leaf carries a body written by this session, and the
node briefs say why the split falls where it does.

## Notes

**Place this subtree with `leaf-insert`, ahead of `specification-capture-k4`.**
`leaf-add` appends at the parent's end, which would queue every book behind a
deep research pair and break the root brief's promise that the documentation is
never blocked.

**The critical path is the validator, not the prose.** No book can be proved
until the fragment ledger takes a per-book corpus instead of compiled-in
constants, so that work is cut ahead of the pilot rather than beside it. It is
also where a recorded rejection reopens: the existing book spec rejected a
sidecar manifest when there was one book, and with six the question is live
again. That is ADR-shaped work and probably earns a design leaf of its own.

**Sequence the wide rename expand → migrate → contract.** Relocating the
existing book under `docs/walkthroughs/` touches validator constants, a large
test suite, the crate README and the context map. One leaf per stage, each
landing green.

**The pilot is `jj-workspace` and its deliverable is two things.** A book, and a
measure of which editorial stages paid for themselves. The second is the point:
nothing currently distinguishes six stages from two, and a pipeline that cannot
be falsified is not evidence of anything. Record what each stage changed, not
merely that it ran.

**The stages come from prior art already on disk.** `TheGreatExplainer`'s
requirements §1.6 specifies the pipeline with its feedback edges and human
gates; `Writegood` carries a judging protocol and the argument that measurement
must precede machinery; `grove.gh-issue-12` is a worked example of a
preregistered evaluation with arms, controls and adjudication. Read them before
inventing a stage list.

**Each book takes a human-authored structure brief as an input artifact.** The
source does not carry audience, conceptual order, or what deserves emphasis.
Cut the leaf that asks for it; do not have a session invent it.

**The art phase is new ground.** The existing book has no diagrams, no figures
and no captions anywhere, and the validator has no concept of an asset. Format,
convention and validation are all unbuilt.

## Decisions (running log)

**1 · Five nodes, twenty-three leaves, inserted ahead of the research pair.**
The subtree is cut as five node directories under `.grove/` —
`walkthrough-machinery`, `user-guide`, `pilot`, `publishing-pipeline`,
`crate-books` — each holding its ordered leaves, all placed with `leaf-insert`
against `specification-capture-k4`'s slot so the whole of P1 and P2 precedes the
research pair in the pre-order walk. Nodes rather than a flat block of
twenty-three siblings because each group genuinely spans many sessions and each
carries context those sessions need and the root brief does not state; that is
what a `BRIEF.md` is for, and this leaf's own `Done when` asks for node briefs.

**2 · The relocation runs first, and as one leaf rather than three.** The task
file's note prescribes `expand → migrate → contract`. Measured: the book path
appears 78 times across 16 non-`.grove/` files — the validator's `SOURCE_INDEX`
and `PAGE_BY_OWNER` constants, seven test files, the crate README, the context
map, the book spec, and one page of the book itself. Every one of them is inside
this repository; nothing outside consumes the path, and no intermediate state
requires the old and new forms to coexist. `expand → migrate → contract` exists
for a blast radius no vertical slice can land green across, and this one lands
green in a single commit under `cargo test --locked --workspace` plus
`every_repository_markdown_reference_resolves`. Cut as one leaf, with the
reasoning recorded here so `review-planning` can attack it directly.

**3 · Relocation precedes the spec split, which precedes the generalisation.**
The alternative — generalise first, so the move only touches prose and fixtures
— was rejected because the shared spec written at the design leaf would then
have to cite `docs/walkthroughs/…` paths that do not yet exist, and
`every_repository_markdown_reference_resolves` sweeps every Markdown file in the
repository and would go red on them. Moving first costs one `sed` over constants
a later leaf deletes; writing a spec against a tree that does not match it costs
a red suite and a forward reference in the campaign's own agreement point.

**4 · The validator is generalised in two halves, along its own `--check`
seam.** `book-check` already separates `--check markdown` from
`--check fragments`, and the compiled-in ledger divides the same way:
`SLICE_ORDER` and `PAGE_BY_OWNER` (page inventory, navigation, canonical order)
against `ROOTS`, `BLOCKS` and `EARLY_USES` (source roots, ownership ranges,
early-use rows). Each half becomes per-book data in its own leaf, and each leaf
leaves `book-check` green over the relocated book. Decision 4 of `plan-k1`
requires both halves; splitting along the existing seam is what lets each land
independently useful rather than as a single unlandable rewrite of a
1,613-line validator and its seven test files.

**5 · The user guide is written before the books, not after.** It is unblocked
and could sit anywhere in the walk. Decision 7 of `plan-k1` makes the user guide
the audience's entry point, so every book will link into it; a guide restructured
after the books are written moves anchors the books already cite. Placed after
the validator work — the task file's stated critical path — and before the pilot.

**6 · The art stage runs by hand in the pilot; its machinery is designed
afterwards, from the verdict.** The root brief's *Notes* argue that expensive
machinery ordered ahead of the measurement justifying it cannot answer whether it
was needed. A figure format, an asset convention and a validator concept of an
asset are exactly such machinery. So the pilot authors figures with what Markdown
already gives it, the measure says whether art paid, and a `design` leaf
afterwards produces either the contract or a recorded rejection — a design leaf
can land an ADR that closes a path, which is why this does not need a prune.

**7 · Each scale-out book is one `impl` leaf that expects to be decomposed into
the pipeline's stages.** The books are to be authored through the extracted
kinds, but those kinds do not exist and grove refuses a kind no launch template
declares, so their leaves cannot be cut today. A single `impl` leaf per book,
whose body instructs the picked session to `leaf-decompose` it into one leaf per
extracted stage and do only the first, defers the kind-naming decision to a
session that can make it — which is `leaf-decompose`'s designed use.

**8 · Structure briefs are cut one per book, not one for all four.** Decision 15
of `plan-k1` makes a human-authored structure brief an input to every book, and
batching four into one interview would have the human decide `grove-loop`'s
conceptual order — 13 roots and 10,533 lines, 72% of the remaining corpus — cold
and months before it is authored. Four `requirements` leaves, each immediately
ahead of its book, at the cost of four human gates in an otherwise AFK arm.

**9 · The ARCHITECTURE anchor resolver is widened in its own leaf, ahead of the
overview.** Widening
`every_adr_citation_names_a_decision_record`'s sibling check to resolve
`docs/ARCHITECTURE.md#<anchor>` citations in Rust sources lands green against
today's tree — twenty-one such citations exist, concentrated in
`crates/grove-loop/src/tree_lifecycle.rs` and `task_tree.rs` — and closes a real
gap in the link-integrity suite whether or not the move ever happens. It is a
prerequisite of the move, so it sits at the head of `crate-books` rather than
inside the leaf that performs it.

**10 · This decomposition earns an adversarial read.** Twenty-three leaves and
five briefs will be executed by a long chain of sessions with no human present,
and decision 2 departs from this leaf's own task-file note. A `review-planning`
leaf is cut as this session's last act and `leaf-insert`ed immediately after this
leaf, ahead of the whole subtree — a review that arrives after the work it would
redirect is worth nothing, and the `plan-k1` → `plan-k2` → `plan-k8` sequence in
this same tree is the precedent.

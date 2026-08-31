# walkthroughs-k9

**Reviews:** walkthroughs-k3

## Goal

Read the subtree `walkthroughs-k3` cut — five node briefs and twenty-three leaf
bodies — adversarially, and report findings. Change nothing.

## Context

- Twenty-three leaves and five briefs will be executed by a long chain of
  sessions with **no human present** except at four `requirements` gates and one
  configuration hand-back. A decomposition error here is discovered by the
  session that trips over it, months later, with no author to ask.
- The producing session's reasoning is in `walkthroughs-k3`'s
  `## Decisions (running log)` — ten numbered decisions. Read them; several are
  departures that were made deliberately and recorded so they could be attacked.
- This tree already ran the same shape one level up: `plan-k1` →
  `plan-k2` → `plan-k8` reported nine findings against the requirements and
  integrated them. Two of `plan-k2`'s findings were narrowed rather than
  accepted whole, which is the calibration to hold — a finding that turns out to
  be a wording problem is still a finding, and a finding that would move a leaf
  needs to survive the actual pre-order walk before it is one.

## Done when

Findings are reported, each anchored to a specific leaf or brief and to the
decision it contradicts. If there are none worth acting on, say so and create
nothing.

## Notes

Specific doubts the producing session wants attacked, offered so you do not have
to find them and so you are not limited to them:

- **Decision 2 — the relocation as one leaf.** The `walkthroughs-k3` task file
  prescribed `expand → migrate → contract`; the producer collapsed it to one leaf
  on the ground that the blast radius is entirely in-repo and no intermediate
  state needs both path forms alive. Is there a call site that does?
- **Decision 3 — relocation before the spec split.** The argument is that a spec
  citing `docs/walkthroughs/…` before the directory exists turns
  `every_repository_markdown_reference_resolves` red. Is that actually true of
  the check, and is the alternative ordering really worse?
- **Decision 4 — the validator split along `--check markdown` / `--check
  fragments`.** The claim is that the compiled-in ledger divides along the same
  seam and that each half lands green independently. Does it, and do they?
- **Decision 5 — the user guide before the books.** This puts two leaves ahead of
  the pilot on a path the task file calls the critical path. Is the anchor-
  stability argument for it real?
- **Decision 7 — book leaves as placeholders for the pipeline's decomposition.**
  Each scale-out book is one `impl` leaf whose body tells the picked session to
  decompose it into the extracted stages. Is that a workable instruction, or does
  it hide a shape that should have been cut?
- **Decision 8 — four separate structure-brief interviews.** Four HITL stalls in
  an arm otherwise driven alone. Is that the right trade against one batched
  interview?
- **Order and the walk.** `pick` is a depth-first pre-order walk and the only
  ordering there is. Walk this tree by hand and check that every leaf's inputs
  are produced by a leaf the walk reaches first — in particular
  `pilot-preregistration-k24` before `jj-workspace-book-k25`,
  `architecture-anchors-k19` before `architecture-move-k31`, and every structure
  brief before its book.
- **The root brief's promise** that the whole of P1 and P2 sits ahead of the
  research pair. Check it against the tree as it now stands, not against the
  intent.
- **What was not cut.** Validator support for assets exists nowhere in this
  subtree, deliberately, on the argument that machinery must not precede the
  measurement that justifies it. Is there a deliverable in the root brief's
  *Done when* that no leaf here reaches?

## Findings

### F1 · High — Three promised design reviews run after the work they are meant to protect

`walkthrough-books-spec-k20`, `pilot-preregistration-k24` and
`pipeline-kinds-k27` each say to cut `review-design` as the producer's last act
(`.grove/07-walkthrough-machinery-k10/02-design--walkthrough-books-spec-k20.md:58-61`;
`.grove/09-pilot-k12/02-design--pilot-preregistration-k24.md:62-63`;
`.grove/10-publishing-pipeline-k13/02-design--pipeline-kinds-k27.md:58-59`). A
lazy review leaf is appended, so the existing siblings ahead of it run first:
both validator implementations consume the book spec, the draft and measurement
consume the preregistration, and `pipeline-skills-k28` implements the kind design.
The review therefore arrives only after the consumers have committed — for the
preregistration, after the experiment it must constrain is over. This directly
contradicts `walkthroughs-k3` decision 10's own rule that a review arriving after
the work it would redirect is worth nothing
(`.grove/04-DONE-planning--walkthroughs-k3.md:144-150`). Arrange each review so
the pre-order walk reaches it, and any integration it cuts, before the first
consumer; otherwise remove the claim that the artifact is review-gated.

### F2 · High — `pilot-measure-k26` requires six commit boundaries from one leaf

The leaf requires one commit for each of five remaining editorial stages and a
committed measurement report (`.grove/09-pilot-k12/04-impl--pilot-measure-k26.md:3-7,17-35`).
Grove's task boundary is one task, one session, one focused commit
(`plugins/grove/skills/grove/SKILL.md:9-13`;
`plugins/grove/skills/grove/references/commit.md:1-20`), which is also the
planning leaf's stated requirement that every child fit one session. The note
that it may decompose *if* it proves too large already names the proof — five
separate commits — and “one child per stage” still leaves the report with no
owner (`.grove/09-pilot-k12/04-impl--pilot-measure-k26.md:53-55`). This
contradicts `walkthroughs-k3` decision 1's claim that the twenty-three leaves are
the independently executable decomposition. Cut the known sequence now: one
leaf per stage, each landing the book green, followed by a reporting leaf that
applies the preregistered decision rule across those commits.

### F3 · High — The settled user-surface and ownership assurance has no leaf

`plan-k1` decision 8 requires every book root to join the curated user-document
surface and every book to earn a tested row in the documentation-ownership table
(`.grove/01-DONE-requirements--plan-k1.md:76-79`). The current curated surface
contains only five top-level guides (`crates/grove/tests/reference_navigation.rs:6-21`),
and the current ownership table has no row for the overview or the new books
(`docs/ARCHITECTURE.md:15-36`). Yet `crate-books-k14` closes on reconstruction,
link sweeps and `scripts/check.sh` only
(`.grove/11-crate-books-k14/BRIEF.md:10-22`), and none of the five node briefs or
twenty-three leaf bodies assigns the missing surface or ownership work. The
subtree can therefore close green while violating a settled requirement,
contradicting `walkthroughs-k3` decision 1's claim that these five nodes cover P1
and P2. Assign both obligations explicitly — including the test changes — to the
shared book-system work or to each book and the architecture move.

### F4 · Medium — The validator slices are not divided by the `--check` seam they claim

`validator-structure-k21` says it owns the Markdown side and names
`SLICE_ORDER`, `PAGE_BY_OWNER` and `SOURCE_INDEX`, leaving the fragment side to
`validator-fragments-k22`
(`.grove/07-walkthrough-machinery-k10/03-impl--validator-structure-k21.md:14-22`).
In the implementation, `validate` runs `ledger::check` only for `Fragments` or
`All` (`crates/book-validation/src/validator.rs:337-353`), while that ledger
uses `SOURCE_INDEX` and `PAGE_BY_OWNER` directly and also imports
`SLICE_ORDER` (`crates/book-validation/src/ledger.rs:3-9,110-124,699-707,765-778`).
`SLICE_ORDER` is shared with the Markdown path too
(`crates/book-validation/src/markdown.rs:58-69`). The proposed data split may be
workable, but it is not the `--check markdown` / `--check fragments` seam and
the first leaf hides fragment-path migration in a supposedly Markdown-only
slice. This contradicts `walkthroughs-k3` decision 4
(`.grove/04-DONE-planning--walkthroughs-k3.md:93-102`). Redraw around the actual
shared metadata loader and its consumers, or revise the two leaf contracts so
their cross-check scope and independent green landing points are explicit.

### F5 · Medium — The guide-first dependency is asserted but never specified

`walkthroughs-k3` decision 5 places both user-guide leaves ahead of the pilot
because every book will link into the guide and would otherwise inherit unstable
anchors (`.grove/04-DONE-planning--walkthroughs-k3.md:104-108`). No book or
shared-spec acceptance condition requires those links. The overview structure
leaf says only that the guide is the entry point
(`.grove/11-crate-books-k14/02-requirements--overview-structure-k29.md:23-27`),
and the sole per-book mention says `docs/USAGE.md` documents the human-facing
surface and must not be restated
(`.grove/11-crate-books-k14/06-impl--grove-llm-book-k33.md:16-20`). As cut, the
guide contributes no input consumed by the pilot, so two sessions sit on the
stated critical path for an anchor-stability property nothing checks. Either add
the guide-link contract and its link acceptance to the shared spec and every
book, making decision 5 true, or move the user-guide node off the pilot/pipeline
path.

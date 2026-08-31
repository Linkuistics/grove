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

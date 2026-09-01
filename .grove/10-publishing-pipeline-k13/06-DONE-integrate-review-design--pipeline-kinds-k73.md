# pipeline-kinds-k73

**Integrates:** pipeline-kinds-k72

## Goal

Triage the design review of `pipeline-kinds-k27` and apply every finding that
survives scrutiny before `pipeline-skills-k28` authors the four editorial kinds.

## Context

- Read `pipeline-kinds-k72` from its committed review task; its findings and
  citations are the handoff and are not restated here.
- The reviewed artifacts are the two pipeline ADRs, the four-kind work order in
  `pipeline-skills-k28`, and the pipeline shape in `crate-books-k14`'s brief.
- The preregistration and pilot report remain frozen evidence: integration may
  correct what the design says or hands forward, never rewrite the measurement
  to make a finding disappear.
- `pipeline-skills-k28` is the direct consumer and follows this leaf in the walk.

## Done when

- Every review finding is classified against the current artifacts as accepted,
  rejected, or already resolved, with the reasoning recorded in this task's
  running decision log.
- Every accepted finding is integrated into the minimum coherent ADR set, the
  implementation work order, and any directly affected brief or durable handoff.
- The correction-chain procedure specifies one ordered, terminating sequence;
  the review allowance has one owner; and the structure-brief precondition is an
  interface an installed skill can test.
- `bash scripts/check.sh` passes.

## Notes

This leaf was inserted at the first live sibling after the review so no consumer
can silently move its path-and-line citations before triage.

## Decisions (running log)

**All four findings are accepted; none is rejected and none was already
resolved.** Each was graded against the current artifacts rather than against
`pipeline-kinds-k72`'s restatement of them, and each reproduces:

- **F1** — `07-impl--pipeline-skills-k28.md:101-109` makes "cut your successor"
  unconditional and `:120-128` makes "cut the re-run plus every later stage that
  must re-read" unconditional. Both hold at once only if some rule tells a
  queued stage not to cut again, and no such rule exists in either ADR or the
  work order. The ADR's own filesystem example
  (`a-feedback-edge-is-forward-tree-growth.md`, *What it buys*) shows a second
  `draft` between `art` and `proof` with neither intermediate re-read, so the
  omission is in the record and not only in the work order. Proof's
  `:227-239` asserts both that it may cut a re-run leaf and that its retirement
  leaves the node with no live leaf. Real issue.
- **F2** — `references/execute.md:21-31` exempts "a producer that already has a
  `review-*` leaf beside it". No editorial stage has one; the family takes no
  `review-<stage>` steps by its own ADR. The substitution also fails
  semantically: the next stage may not repair the previous stage's class
  (`:96-100`), so it is not an adversarial read of its predecessor's
  obligations. Real issue, and the loaded path would have carried two owners
  and two answers.
- **F3** — `:142-153` states the precondition as "a structure brief exists".
  `leaf-decompose` gives every document node a `BRIEF.md` and bootstrap reads
  it, so a naive existence test passes in exactly the no-human-structure case
  that must stop. A contract stated unclearly: the fold's limit is right, the
  interface for testing it was never written.
- **F4** — the confirmed `01-orientation.md#public-surface` conflict appears in
  none of the four handoff surfaces, only in the frozen report
  (`docs/evaluations/editorial-pipeline-pilot/README.md`), which no future
  structure-brief revision is directed to read. Real issue.

None of the four demands the design be rethought, so none becomes a new producer
review chain: F1 and F2 specify mechanisms the design already chose, F3 writes
the interface for a limit already recorded, and F4 files a datum. The set of two
ADRs, the four-kind set, and the merge target are untouched.

**F1 · The correction chain is one ordered run, and a queued stage cuts
nothing.** Two rules replace the pair that could not both be followed. A stage
sending work back cuts a single **contiguous run** in pipeline order, beginning
at the owning stage and ending at `proof`; that run *replaces* its ordinary last
act rather than being cut beside it. And every stage's last act is conditional:
cut the stage that runs after you **unless a live later sibling under this node
already holds it**. The condition is read off the tree, so constraint 1 holds —
no flag, no field, no leaf state — and it is what makes the chain terminate: a
re-run leaf inside a queued run regrows nothing, and the one-re-run escalation
bound caps the number of runs. Rejected alternative: cutting only the owning
stage and the normal successor, which is the ADR's original call-order recipe
and lets `proof` run over material no intervening charter re-read.

**F2 · The ordinary producer allowance is preserved, and the exemption claim is
withdrawn.** Every editorial stage — `proof` included — is a plain producer with
`references/execute.md`'s ordinary leaf-wide allowance of at most one in-session
reviewer. Rejected alternative: designing an explicit editorial-family exception
that supplies equivalent adversarial coverage, which would need evidence nothing
in the pilot produced and would put an unmeasured rule beside four measured
ones. The escalation route the execute procedure names — cut a
`review-<producer>` leaf — has no kind in this family, so the family file states
what stands in its place: finish to a coherent boundary and route the doubt
through the machinery that already exists (the next leaf's body, the node
brief's `## Handed forward`, or a re-run run). The pipeline ADR's redundancy
argument for taking no review chains is corrected in place; the decision itself
stands on what it always stood on, that nothing measured them.

**F3 · The structure-brief precondition is stated as a three-part interface.**
A structure brief is an artifact **named by path** in this leaf's body or in the
document node's brief, which itself states the audience, the ordered section plan
and what deserves emphasis. All three, citable. The work order now says
explicitly that the existence of a `BRIEF.md` is not the test and why — every
decomposed node has one — so the naive check that would pass in the failing case
is named as the trap it is. Rejected alternative: naming a filename convention
(`*-book-structure.md`, a sibling `structure` leaf), which would bake this
campaign's task convention into a token installed on every machine, the same
defect the unprefixed-token decision rejected.

**F4 · The conflict is filed in the structure brief that owns it.** The report
assigns the correction to whoever next revises
`docs/specs/jj-workspace-book-structure.md`; that document is now where the
conflict is recorded, in a section a revising session cannot miss, citing the
frozen report rather than rewriting it. Rejected alternatives: a defect leaf
(the conflict is not actionable until someone revises the brief, and a leaf
would either sit live indefinitely or decide a standards question outside its
charter) and the crate-books brief (current-state context for the books, which
`jj-workspace` is not among).

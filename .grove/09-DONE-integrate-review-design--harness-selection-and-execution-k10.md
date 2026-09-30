# harness-selection-and-execution-k10

**Integrates:** harness-selection-and-execution-k9

## Goal

Triage this design review and apply the real findings before planning turns the
current harness-dispatch design into implementation work.

## Context

Read the findings from the committed review `harness-selection-and-execution-k9`,
not from this body. Its reviewed artifact is the whole current design at
`4158f6cb3e8c`, including k7's repairs, against the root brief and the k1/k4/k7/k8
running decisions. The area spec, its three design ADRs, glossary and visual
design remain the artifacts to reconcile. The review distinguishes design
contract gaps from accepted trade-offs and makes no implementation claims.

## Done when

- Every finding has a reasoned disposition in this leaf's running log.
- Real issues are repaired in the existing spec and ADR set, with related
  glossary, views, acceptance cases and planned methodology amendments kept
  coherent. This leaf repairs the design; production fixes and their acceptance
  checks remain implementation work.
- Settled human choices remain settled, and any new human trade-off is presented
  with evidence and a recommendation.
- Planning `harness-selection-and-execution-k6` can consume a coherent current
  design, or any substantial redesign and required review is explicitly placed
  ahead of it.

## Notes

This integration was inserted as the next live sibling after k9 so its citations
are consumed before planning changes the workstream. The inspection-only review
did not run repository checks; this integration owns verification appropriate
to its repairs, using the project's Taskfile workflow.

## Decisions (running log)

Findings were read from review commit `92f850dc1140` (the k9 leaf's
`## Findings`), and graded against the design at that commit: the spec, the
three design ADRs, the glossary, the views, and the repository methodology
corpus they amend (`retire.md`, `decompose.md`, `rules.tsv`). The human's
settled choices stay settled: the review-carried run reference, the three
confirmed methodology amendments, the two amended acceptance sentences, the
launch-time validation boundary and the kernel trade-off. Each repair below
makes the confirmed rule precise inside the chosen mechanism; none adds a
namespace, registration, lookup by handle or multi-author accounting, so none is
a new human trade-off.

**F1 — real issue, fix the artifact.** Reproduced from the text. The spec and
ADR say a restarted producer's finishing session "replaces the line", and that
a producer without a run "has no run ID to write, so its review carries no run
reference". Nothing makes the second true when an earlier dispatched attempt
already wrote `run R1` on a live review: jj snapshots the unsealed working copy,
so R1's edit survives the crash, and a direct-harness restart that follows the
amendment as written leaves it. R1 exists and carries no failure detail, so the
supplied policy reads its provider. A pre-supplied declaration goes stale the
same way. Repair: every finishing session replaces or removes. With
`HARNESS_DISPATCH_RUN_ID` it writes its run; without one it removes any
`**Creator:**` line, run or declared, from the reviews it is responsible for.
A declaration therefore describes a finished artifact: the owner writes it
after the producer's finishing commit, when the review refuses. A declaration
written earlier is removed by the finishing session and must be supplied again;
the session never writes a declaration itself, because a declaration is the
owner's assertion and a session's own provider claim would be a transcription.
Carried into the spec, the ADR's retirement amendment, the glossary, the
creator-flow view and a Grove launch-boundary acceptance case.

**F2 — real issue, fix the artifact.** Reproduced from the text. The design
makes a child's run the creator of a decomposed producer, but the writing rule
names only reviews of the finishing session's own handle, and `retire.md`'s
node-close steps (lines 75–110) say nothing about creator lines. A review
waiting on node P is never updated by the child C whose retirement closes P.
Repair: a finishing session writes for the leaf it retires **and** for each node
its close cascade closes, in the same commit, and for nothing else. A node that
stays open writes nothing, and later work that repairs a producer already
finished changes no creator reference. The consequence is stated visibly as the
accepted one-creator simplification: the run whose retirement closes a node
stands for the whole decomposed producer, whichever kind that child is. It
needs no kind semantics, and the requirements exclude contributor accounting.
The ADR's retirement amendment now names the node-close procedure and its
`node-close-four-steps` conformance row. The acceptance row gains a pre-cut
review of a decomposed producer across a multi-level close, which also
exercises a run whose task identity differs from the `**Reviews:**` handle.

**F3 — real issue (evidence wording) plus an accepted trade-off.** The ADR's
"the only run ID within a producer session's reach is its own" is false. Other
review bodies and version history expose other run IDs, `record show --run`
admits any known ID, and a direct harness launched from inside a dispatched
session inherits the outer run's `HARNESS_DISPATCH_RUN_ID`. That last route was
verified from the spec's environment rules, which replace the variable only for
dispatch's own final harness. The spec's "the reference is therefore itself
evidence that the run executed" overclaims likewise. Repair: the reference is
the writing session's attestation of association and execution; only the
provider it leads to is execution-recorded. A wrong existing run, or an attempt
of unknown execution, passes launch and shows only in inspection. That is the
transcription trade-off, now accepted visibly. Task-ID equality stays rejected
because it would refuse F2's legitimate child/ancestor case.

No substantial redesign was found, so nothing is placed ahead of planning
`harness-selection-and-execution-k6`. Its body cites the creator-reference ADR
for the amendment text, which now carries the removal branch and the node-close
step, so the body needs no change. The root brief's design handoff names this
integration. No in-session reviewer was spent. The finishing paths were
enumerated against the repaired text instead: dispatched finish, pre-cut review,
crash restart in both route directions, owner pre-declaration, a decomposed
producer through a multi-level close, a direct-harness last child, a review cut
after its producer finished, and later repair work. Each path ends in a current
run reference or in a fail-closed refusal with the declaration remedy.

## Verification

Stale-claim sweep: `rg --hidden` over `docs`, `CONTEXT.md`, `CONTEXT-MAP.md`,
`.grove`, `plugins` and `crates` looked for the removed claims: own-ID-only
reach, the reference as "itself evidence", "no run ID to write", "replaces the
line before" and "before retiring, the finishing". It now finds them only as
quotations in the k9 and k10 running logs, plus one unrelated phrase in
`docs/formalism-findings.md`. The same pattern run against `@-` matched the
pre-session spec and ADR, so the control was seen to fail. Every
remaining `Creator`/`creator` mention in the spec, ADRs, glossary, map, views,
root brief, planning body and research survey was enumerated and classified as
consistent. The survey's `originalCreator` pseudo-code is dated proposal text,
not contract.

The design viewer (`task design:harness-selection PORT=8773`) was checked in
Playwright's Chromium at 1200 px. The first render of the edited creator-flow
view failed with a Mermaid parse error, because `;` separates statements in
sequence syntax. The note was corrected, and then all five views rendered with
no error elements and no horizontal overflow. The new note and caption were
present, and a screenshot was legible. The server was stopped and
`.playwright-mcp/` was deleted before any commit.

`task check` exited 0, with all eleven principal checks passing, including the
Rust suite (1449 passed, 0 failed) and book-check. The jj working-copy commit
was `8b278a8a646d` before and after the run. The log is in this session's
scratch space. This section and the retirement rename follow the run and change
no checked contract. Nothing is claimed as implemented.

# creator-methodology-k39

## Goal

Amend the shipped Grove methodology as the creator-reference ADR states. A
finishing session writes `**Creator:** run <run-id>` from its own
`HARNESS_DISPATCH_RUN_ID` on the reviews of each producer it finishes.
Without a run, it removes any `**Creator:**` line. The conformance rows and
pins that prove the amendment ship with it.

## Context

The ADR's three bullets are the amendment text, including the node-close step.
Where a rule is owned is the conformance manifest's business, and each rule has
one owner. Put the new wording where the manifest says the rule lives, not in
every file that mentions reviews. The
`linkuistics:decision-records` discipline applies to reworking the ADR into
current state.

## Done when

- `TASK-FORMAT.md` says a body still carries nothing that routes its own
  session, and that a review body may carry one `**Creator:**` line, the only
  record of a past session any body carries.
- `references/retire.md` keeps retirement to one filename. It rescopes the claim
  that a waiting review needs no record of how its producer ran. It states the
  finishing session's step, in its task's commit, for each producer it
  finishes, meaning its own leaf and each node its close cascade closes. The
  step writes the line on the review it cuts and on any live review naming
  that producer's handle, or removes the line with no run. The node-close steps
  carry the same step.
- `references/decompose.md` keeps Grove recording and comparing nothing about
  how a producer ran. The producing session names its run, and the dispatcher's
  policy compares.
- The statements that no code reads the relationship lines are scoped to
  Grove's own code: the glossary, `TASK-FORMAT.md`, `docs/ARCHITECTURE.md` and
  `docs/USAGE.md`. A sweep enumerates every such statement, with a positive
  control, and classifies each one.
- The conformance manifest updates the affected rows. The conformance runner
  and its test suite pass, and so do the composition-guidance pins. Each new or
  changed pin is seen to fail against the old wording before it passes.
- The skills Grove provisions to Codex carry the amendment, as their tests
  show.
- The ADR and the spec describe the amendment as shipped. The glossary's
  creator entries are current.

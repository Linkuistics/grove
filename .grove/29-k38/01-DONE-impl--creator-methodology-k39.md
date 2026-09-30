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

## Decisions (running log)

**The creator step has a manifest row of its own,
`finishing-session-names-its-run`.** The ADR states the step under
`retirement-is-filename-only`, but a row pins one phrase. Moving that row's
pin to the step would take *retirement touches one filename* out of the
single-source sweep. The step is also a rule a session can break on its own,
by omitting the line, and the sweep is what stops `references/decompose.md`
restating it where a review is cut. So `retirement-is-filename-only` and
`diversity-is-the-configs` keep their pins, whose sentences survive, and the
composition-guidance pins hold their amended halves.
`body-carries-no-launch-metadata` is repinned to *carries nothing that routes
its own session*, and `node-close-four-steps` gains its first pin. The ADR
names the new row.

**A node close keeps four steps.** The creator step joins step 4, the branch
taken when the check holds. A fifth numbered step would make the rule's name
false, and steps 2 and 3 are the branches where the node does not close.

**`**Creator:**` joins the declared body-marker set** in
`session_kind_guidance.rs`, so `TASK-FORMAT.md` can show the line in an
example. The line routes nothing in its own session, which is that test's
claim. `composition_verbs.rs` now also asserts a generated body has no such
line.

**Each pin was seen to fail before it passed.** Against the unamended skills
the runner reported `stated in [nowhere]` for the three changed rows. The new
composition-guidance test collects every miss, so one run named each pin: the
positive wordings were missing, and each unscoped wording was still present. The two
retained halves were not in that list. The provisioning test passed byte
equality and failed on wording. The conformance suite gained three standing
controls: the step missing, the step missing at node close, and the step
restated in `references/decompose.md`.

**The sweep enumerated, then classified.** It took every negated reading,
parsing or validating statement in the repository, dot-directories included,
with no subject filter, then kept those whose paragraph concerns task bodies,
their lines, reviews or producers, and read each. The control was the old
wordings in a hidden scratch directory. The first subject filter dropped two
of them, so it was widened until all were kept; a negation and a subject that
occur nowhere each returned nothing; the task tree, where the class is
discussed, returned it. Classes: scoped to Grove's own code by this leaf (the
glossary, `TASK-FORMAT.md`, `docs/ARCHITECTURE.md`, `docs/USAGE.md` and
`docs/specs/doubt-grove-review-mechanics.md`); already scoped (the area spec,
the glossary's adapter entry, *grove validates no cross-leaf grammar*); dated
release history in `CHANGELOG.md`, left as written; task bodies under
`.grove/`; walkthrough prose about the Grove source it reconstructs; and other
subjects, such as briefs, stems and the adapter's own reads.

**The review-mechanics spec was scoped too**, though the ADR names four
documents. It made the same unscoped statement and is a current spec.

**Four dispatch-side statements are now false until `creator-lifecycle-k40`
runs.** They say Grove's sessions do not yet write the line: the dispatch
README twice, `worker/examples/grove-review.ts`'s header, and
`worker/grove/index.ts`'s header and `creator_line_missing` remedy. k40's task
owns them with the worker rebuild and the `tests/grove.rs` assertions they
need. Releasability is claimed at the node boundary.

**No in-session reviewer was spent.** The node's `review-impl` is scheduled,
and k40 cuts it.

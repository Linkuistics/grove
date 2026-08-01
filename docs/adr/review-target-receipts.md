# Review target receipts

Grove records the effective harness and model of the producer session that
retires a reviewed artifact inside the linked review task, then recomputes the
review target at launch and compares the two. The foreground launch exports one
session-target value containing the worktree and stable handle from one
structured routing peek as well as the resolved target. The same guarded peek
supplies routing, readiness, and launch diagnostics; no later pick reconstructs
the routed identity. Retirement accepts the context only when the worktree,
routed handle, and factual current pick all name the retiring producer. It
applies the `DONE` infix first and unconditionally replaces any producer receipt
best-effort only after that succeeds, so a failed terminal rename writes no
receipt and a normal failed receipt write leaves diversity uncheckable without
blocking retirement.

This binds because current routing configuration cannot reconstruct a historical
launch after the configuration changes, while a route ledger or signal payload
would put authoritative workflow state outside the task tree. Environment is
inherited rather than addressed, so every non-session harness spawn must scrub
the session-target value and all three identity checks remain mandatory even
after that structural guard. The routed handle is evidence about what target
actually launched; it never overrides Grove's later factual pick. A non-empty
structured peek without that handle is a launch-time routing failure, not a
target Grove guesses with a second read.

The freshness guarantee covers Grove's cooperative transition: Grove never
writes an authoritative receipt beside a live producer, and a successful later
retirement replaces any existing line. Directly restoring a live producer while
leaving its old receipt creates a generation ambiguity that the task tree does
not encode. Grove still does not block retirement on advisory metadata; if the
post-`DONE` replacement then fails, it diagnoses that the retained receipt may be
stale and the operator must remove it before relying on comparison. Eliminating
that hand-edit limitation would require a new authoritative retirement-generation
marker and is not part of this decision.

Model equality is exact: equal non-null selector strings match, two harness
defaults match only under the same harness, and a default never matches an
explicit selector. An uncheckable warning always names its review; it names a
producer only from a valid stable relationship and otherwise says
`producer=unknown` with the reason, never inferring identity from tree position.
If a receipt's producer disagrees with a valid `Reviews` relationship, the
result is `uncheckable(reason=receipt-producer-mismatch)` and the relationship's
producer remains the only named producer.

## Considered options

- **Write the receipt before applying `DONE`.** Rejected because a successful
  receipt write followed by a failed terminal rename leaves a live producer with
  a target that a later finisher may be unable to overwrite. Reopen only if the
  receipt and terminal outcome can be committed atomically as one portable
  operation.
- **Add a retirement generation solely to validate manually restored leaves.**
  Rejected because it changes filename-only retirement or introduces another
  authoritative tree artifact for a state Grove itself never produces. Reopen if
  manual removal of `DONE` while retaining dependent metadata becomes a supported
  lifecycle operation rather than an operator-created malformed state.
- **Recompute the producer's target from current configuration.** Rejected
  because kind, family, harness, and model configuration may change between the
  producer and review sessions, yielding a precise comparison against a target
  that never ran. Reopen only if routing becomes immutable for a grove's entire
  lifetime.
- **Keep a route ledger or add the target to the completion signal.** Rejected
  because the task tree is Grove's only workflow state, and the receipt is a
  fact about the review relationship that consumes it. Reopen only if Grove's
  artifact-only state constraint changes.
- **Make a missing receipt block retirement or review.** Rejected because target
  diversity is advisory: lifecycle correctness must not depend on metadata that
  can be absent from legacy or hand-edited chains. Reopen only if diversity
  becomes a launch correctness requirement rather than a warning.
- **Infer a missing producer from the previous sibling or chain suffix.**
  Rejected because positions move and Grove deliberately does not parse a
  cross-leaf grammar from filenames. Reopen only if producer identity becomes an
  explicit stable relation supplied by the tree rather than an ordering guess.

# Review target receipts

Grove records the effective harness and model of the producer session that
retires a reviewed artifact inside the linked review task, then recomputes the
review target at launch and compares the two. The foreground launch exports one
worktree-scoped session-target value; retirement accepts it only for that
worktree's currently picked producer and materialises it best-effort, so receipt
failure can make diversity uncheckable but can never block a `DONE` infix or a
review launch.

This binds because current routing configuration cannot reconstruct a historical
launch after the configuration changes, while a route ledger or signal payload
would put authoritative workflow state outside the task tree. Environment is
inherited rather than addressed, so every non-session harness spawn must scrub
the session-target value and the worktree/current-pick checks remain mandatory
even after that structural guard.

## Considered options

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

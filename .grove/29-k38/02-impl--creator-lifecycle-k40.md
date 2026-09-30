# creator-lifecycle-k40

## Goal

Prove the original-creator mechanism across Grove's lifecycle at the launch
boundary. Fake producers follow the documented `**Creator:**` convention, and
dispatched reviews resolve the right creator through retirement, reordering,
multi-level node close and a direct-harness finish. Document the conventions
and remedies for owners.

## Context

The spec's Grove launch-boundary row lists these cases. A fake producer is a
fake harness that behaves as the amended methodology tells a session to. It
reads `HARNESS_DISPATCH_RUN_ID`, writes or removes the review's line, and
retires or closes through `grove-llm`. A real session's compliance is the
conformance rows' concern, not this seam's.

`review-policy-k58` qualified four dispatch-side statements until the
methodology ships. Each says Grove's sessions do not yet write the line, so
the owner does. They are, in `crates/harness-dispatch`: the README's Grove
review section, in its form description and "When the creator line is
missing"; `worker/examples/grove-review.ts`'s header; and
`worker/grove/index.ts`'s header and `creator_line_missing` remedy. The
remedy's command-seam assertions are in `tests/grove.rs`. Make each current
with this leaf's documentation. Keep the owner's part for a producer with no
run, and the warning that a run reference is its writer's word.

## Done when

- **Retirement and reordering.** A dispatched producer writes its run on a
  pre-cut review. It is then retired, and the tree reordered before the
  review launches. The dispatched review selects from the named run's recorded
  provider, and a changed current mapping does not alter it.
- **Decomposed producer.** A review cut before its producer decomposed ends up
  carrying the run whose retirement closed the node through a multi-level
  close. It selects although that run's task identity is the child's handle,
  and inspection shows both.
- **Direct-harness finish.** A dispatched producer attempt followed by a
  direct-harness finish leaves the pre-existing review with no `**Creator:**`
  line. That review refuses with the declaration remedy. Adding
  `**Creator:** declared <provider>` then admits a different-origin reviewer.
- **Findings after teardown.** A review attaches an observation to the
  producer's run after `.grove/` is removed, and `record show` returns it.
- The usage documentation and configure-grove explain the `**Creator:**` forms,
  who writes or removes the line, the declaration remedy for artifacts with no
  run, and why a wrong but existing run is not detected at launch.
- This node's `Done when` holds. Retiring this leaf closes the node. As this
  leaf's last act, cut the node's `review-impl` as the node's sibling,
  directly after it, never inside it. Run `grove-llm leaf-insert --kind
  review-impl dispatch-documentation-k41 creator-reference`, targeting the first root
  entry after this node (today `dispatch-documentation-k41`). Give it
  `**Reviews:** creator-reference-k38`, and write the node brief's review doubts
  into its body.

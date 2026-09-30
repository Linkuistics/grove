# grove-review-adapter-k37

## Goal

Ship `harness-dispatch/grove`. It reads the supplied task file's `**Reviews:**`
and `**Creator:**` lines and builds the generic reviewed artifact. Ship too a
Grove example policy that composes the adapter with the review selector, which
an owner activates by importing it from personal policy.

## Context

The spec's `#review-policy` states what the adapter may and may not read. The
adapter depends only on the public SDK and Grove's documented task conventions,
and the core never imports it. An extraction could therefore move it to
Grove's side. The methodology that has sessions write `**Creator:** run …`
arrives with `creator-reference-k38`. Until then an owner can still use
`**Creator:** declared …` lines, and this leaf's fake producers write run lines
themselves.

## Done when

- The adapter reads only the supplied `--task-file`, through the SDK's measured
  read, so its delivered context is inspectable. For a configured review entry
  it requires exactly one standalone `**Reviews:** <handle>` line and one
  standalone `**Creator:** run <run-id>` or `**Creator:** declared <provider>`
  line. Missing, duplicate and malformed lines refuse, as does an unreadable
  task file. `**Reviews:**` under a kind that is not a configured review entry
  refuses.
- The adapter has independent fixtures for each Grove convention it interprets.
  It never reads kind or identity from the filename, resolves a handle,
  enumerates the tree, reads briefs, selects another leaf or consults
  running-session state.
- The Grove example policy composes the adapter with the selector. Inspection
  and the run record show the adapter version, the creator reference, its
  evidence class, the resolved provider and the task-file digest.
- Command-seam tests with the shipped files cover two cases. In the first, a
  fake producer launched through dispatch writes its `**Creator:**` line from
  `HARNESS_DISPATCH_RUN_ID`. The dispatched review of that task file then uses
  the named run's recorded provider even after the current mapping changes. In
  the second, a store holding an earlier run of the same task identity does not
  satisfy a review task with no `**Creator:**` line.
- The archive assertions include the adapter, the review example and their
  declarations and sources. The per-target smoke passes.
- The usage documentation gives the explicit activation steps, a personal
  policy importing the Grove example by specifier, and every refusal's remedy.
- This node's `Done when` holds. As this leaf's last act, cut the node's
  `review-impl`: run `grove-llm leaf-add review-policy-k35 review-policy
  --kind review-impl`, with `**Reviews:** review-policy-k35`. Write the node
  brief's review doubts into its body.

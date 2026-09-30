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
- This node's `Done when` holds. Retiring this leaf closes the node. As this
  leaf's last act, cut the node's `review-impl` as the node's sibling,
  directly after it, never inside it. Run `grove-llm leaf-insert --kind
  review-impl creator-reference-k38 review-policy`, targeting the first root
  entry after this node (today `creator-reference-k38`). Give it
  `**Reviews:** review-policy-k35`, and write the node brief's review doubts
  into its body.

## Decisions (running log)

**Specifier and files.** The adapter ships as `harness-dispatch/grove`, from
`worker/grove/index.ts`, with its declarations and readable source in
`libexec/harness-dispatch/grove/`, beside `sdk/` and `examples/`. The Grove
example ships as `harness-dispatch/examples/grove-review`, from
`worker/examples/grove-review.ts`. The adapter's directory joins the digest
set in `build.rs` and `scripts/dispatch.sh`, both tsconfigs, the embedded
table, `documented_specifiers` and the archive manifest.

**A loader may return a refusal.** The adapter refuses from `loadContext`, by
returning `select`'s own refusal shape, which the front reports as
`policy_refused` at stage `context`. The alternatives each failed a test.
Throwing reports `context_loader_failed`, whose remedy is to fix the loader,
not the task file. Carrying the refusal to `select`, in closure state or in
the context, makes it depend on the owner pairing the adapter with a `select`
that checks for it. An owner who composes the adapter's loader with a
`select` of their own would lose the unlisted-kind refusal silently, and a
review would take a static route. The loader refusal is generic: it adds no
review semantics to the core, and any loader whose required facts are
invalid can use it.

**The adapter version is the import, reported by the worker.** The worker
reports `{ specifier, version }` when the policy imported
`harness-dispatch/grove`, directly or through an embedded example that
composes it, and `null` otherwise. Bun calls a `build.module` callback lazily,
once, on the first import, both under `bun run` and compiled (probed with Bun
1.4.2). An example's own imports are bundled rather than resolved through that
registration, so the worker lists the examples that compose the adapter. The
report rides on the policy, context and selection frames, and the front keeps
the last, so an import inside `loadContext` counts too. The version is the
adapter's own `version` export, bumped when what it reads or how it reads it
changes. The worker's package version and build ID already pin its bytes.

**Line grammar.** A line is a `**Reviews:**` or `**Creator:**` line when it
begins with that marker at its first character. A mention inside a line, or
an indented one, is not. Such a line must be the whole standalone form: the
marker, one space and the value, with nothing after it but the CR of a CRLF
ending. `**Reviews:**` takes a Grove handle in `TASK-FORMAT.md`'s grammar.
`**Creator:**` takes `run <run ID>`, in the canonical form, or `declared
<label>`, the rest of the line taken verbatim. A label with stray spaces is
therefore not normalised: the selector's exact membership check refuses it as
a non-member, quoting it. A marker line inside a fenced block still counts, so
a quoted example refuses as a duplicate rather than being skipped by a
markdown parser the adapter does not have.

**The task file is read whole, for every kind.** The read uses
`maxBytes: request.limits.contextBytes`. Leaves reach 178 KB in the owner's
groves, past the 64 KiB per-read default, and the text never enters the
context, only its measured digest. Every kind with a task file is read,
because `**Reviews:**` under a kind with no review entry refuses.

**Refusal codes.** `task_file_missing` (a review kind without `--task-file`),
`reviews_line_missing`, `reviews_line_duplicate`, `reviews_line_malformed`,
`creator_line_missing`, `creator_line_duplicate`, `creator_line_malformed`,
`reviewed_artifact_conflict` (a caller context that already names one), and
`review_kind_unlisted`, the selector's own code for the same condition in its
generic form, so one condition has one code and one remedy.

**The adapter report counts against the message bound.** It is part of the
policy and selection frames, which the fixed 1 MiB bound measures whole. So
`tests/bounds.rs` now frames its exact-size snapshot and result as the worker
does, with `adapter: null`. The context frame's 1 KiB envelope already covers
the report.

**The tests were seen to discriminate.** Sixteen wrong implementations, each
type-correct and built into the worker, turned exactly the tests aimed at them
red. Twelve were in the adapter: an unlisted kind routed; the first of
duplicate lines taken; a declaration trimmed; the kind read from the file
name; the default read bound; a caller's reviewed artifact overwritten;
indented markers counted; any run value accepted; a missing creator line left
to the selector; the artifact ID taken from `--task-id`; the example missing
from the worker's adapter set; and a CRLF kept. Four were in the front: the
context frame's report ignored, the selection frame's ignored, the loader
refusal unjudged, and extra refusal fields admitted. The misleading file name
first named two review kinds, so reading the kind from it passed that test.
It now names `impl`. The worker's source digest and the front's sources
matched before and after the sweep. Stripping each of the typed fixture's
three `@ts-expect-error` lines showed each guards its intended error.

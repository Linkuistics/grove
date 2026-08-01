# doubt-grove-review-mechanics

## Problem

Doubt-driven development and Grove both materialise fresh-context adversarial
review. Used independently they are bounded, but used together without an owner
they can duplicate a scheduled review, spawn a second reviewer after every fix,
or let an in-session child bypass the harness and model policy Grove established
for review work.

The composition must preserve the cheap move for a small, unexpected doubt while
turning substantial review into ordinary task-tree work. It must also preserve
Grove's stable handles, predictable pick order, restart behavior, Git/Jujutsu
symmetry, and guide-not-gate posture.

## Solution

The [Grove owns escalated review](../adr/grove-owns-escalated-review.md)
decision assigns the orchestration boundary. A picked plain producer may spend
one in-session doubt reviewer across the whole leaf. A second review need, or a
substantive actionable finding whose non-mechanical fix needs another review,
promotes that producer into a Grove-managed review chain. The producer finishes
only to a coherent reviewable boundary; the fresh `review-*` session performs
the next adversarial pass.

The exceptions are findings that are trivial, noise, a visible accepted
trade-off, or a fix conclusively covered by an executable test seam. They do not
create a second review need.

### Behavior by session kind

| Session | In-session doubt | If more review is needed |
|---|---|---|
| Plain `requirements`, `design`, `planning`, `prototype`, or `impl` producer | At most one reviewer for the entire picked leaf. | Promote the picked producer into a review chain. |
| Producer already in a review chain | None; its scheduled `review-*` leaf is the fresh-context review. | Finish to the scheduled review boundary and hand off. |
| `review-*` | None; this session is already the adversarial read and produces findings, not fixes. | Record the findings for its integration leaf. |
| `integrate-review-*` | At most one narrow reviewer. | Externalise substantial redesign as a new producer review chain; an integration leaf is not promotable. |
| Either `research` leaf or `combine-research` | None; the pair supplies independent corpora and the combiner supplies the adversarial move. | Put a load-bearing derived decision in its own reviewed producer chain. |
| Any session not executing the leaf returned by Grove's pick step | Unchanged standalone doubt behavior, including its bounded cycles and optional interactive cross-model review. | Grove has no ownership merely because `.grove/` exists in the checkout. |

## Atomic promotion

The agent-facing interface is one deep operation:

```text
grove-llm leaf-promote-chain <picked-producer> [--json]
```

`<picked-producer>` accepts the absolute path returned by `grove-llm pick` or a
stable key/handle that resolves to it. The operation takes no parent, stem, kind,
or harness flags: all four are facts already carried by the picked producer, and
review routing remains policy-owned.

Given a picked `05-sync-design-k12.md` and a tree whose maximum key is `k20`,
promotion produces:

```text
05-sync-design-chain-k21/
  01-sync-design-k12.md
  02-sync-design-review-k22.md
  03-sync-design-integrate-k23.md
```

The containing node reuses sibling position `05`; every former sibling keeps its
position. The producer is moved byte-for-byte, retaining `sync-design-k12`, its
kind, optional harness declaration, and every task detail. The brief-less node,
review, and integration receive fresh tree-wide keys. The generated review and
integration tasks use their derived kinds and carry stable relationships:

```markdown
**Reviews:** sync-design-k12
**Integrates:** sync-design-review-k22
```

Their default goals name the linked task and the kind's normal discipline, so an
interrupted producer session still leaves runnable review and integration work.
`leaf-add-chain` emits the same relationship metadata for proactively created
chains. The metadata, never a filename suffix or mutable position, is the
machine-readable relation.

Plain stdout contains four absolute paths in node, producer, review, integration
order and is written only after success. `--json` returns those paths and all four
stable handles plus a `changed` boolean as one object. `--json` failures use a
stable error code and a non-zero exit rather than mixing prose into stdout. The
second plain-text path is therefore the value the producer passes to
`leaf-retire`.

Promotion accepts only a live, currently picked producer of one of the five
producer kinds. It refuses terminal leaves, `research`, `combine-research`,
`review-*`, `integrate-review-*`, and a producer already related to a scheduled
review, with an error that names the appropriate Grove action. A refused run
changes nothing and consumes no key.

A retry by stable producer handle is idempotent while the already-created shape
is complete and that producer remains the picked leaf: it returns the same four
paths with `changed: false`. A partial or conflicting shape errors with the exact
paths that need inspection rather than nesting another review chain.

The operation pre-validates the source, derived kinds, destination, generated
content, and three-key allocation before writing. It creates the brief-less node
and generated leaves, then moves the producer through Grove's existing
VCS-aware rename seam as the final fallible mutation. Every reported failure
rolls back only the newly created node and leaves the original producer at its
original path byte-for-byte; rollback failure names the exact recovery paths and
says the command is not safe to retry. This is the same command-level atomicity
as Grove's other composite tree verbs, not a claim of power-loss transactions
across several filesystem entries.

## Producer handoff

After promotion, the producer session follows one sequence:

1. Apply only the bounded change needed to restore a coherent, reviewable
   artifact. Run executable checks, but spawn no second doubt reviewer.
2. Commit the artifact and promotion together, naming the unchanged producer
   handle in the commit message.
3. Retire the producer using the relocated path returned by promotion.
4. Run `grove-llm complete` as the final action.

Before the producer receives its `DONE` infix, retirement writes the producer's
actual Grove launch target into the linked review task. If the session stops
after promotion but before retirement, pick returns the still-live producer at
child position `01`; the eventual retiring session overwrites the receipt with
its own target. Once retired, the unchanged depth-first walk returns the review,
then integration, before any later sibling outside the chain.

## Advisory target diversity

An effective launch target is the harness name plus the exact model selector
Grove passed to that harness; a harness-managed default is represented
explicitly rather than guessed. The loop driver exposes that resolved target to
its foreground session as ephemeral launch context:
`GROVE_SESSION_HARNESS` is always set by `grove do`, and
`GROVE_SESSION_MODEL` is present only when Grove passed a named selector. A
missing harness means the producer is being retired outside the loop; a missing
model with a present harness means that harness's default.

Producer retirement materialises the context in the linked review leaf beside
the stable `**Reviews:**` relation:

```markdown
**Producer launch:** {"harness":"claude","model":"opus"}
```

The `model` value is `null` for a harness-managed default. Defaults are scoped
to their harness rather than treated as one cross-vendor model identity. The
receipt records the target that actually launched the producer session, not the
target that the producer kind would resolve to under later configuration. It
does not record an interactive model change made inside the harness, which Grove
cannot observe.

At every `review-*` launch, Grove first resolves the review's own effective
target using the existing leaf → kind → family → stamp harness policy and the
existing harness-scoped → unscoped model policy. It then compares that target to
the producer launch receipt:

- if harnesses match, warn;
- if exact model selectors match, warn;
- if both match, emit one warning naming both matching axes;
- if both differ, stay silent;
- if the relationship or receipt is absent, malformed, or explicitly unknown,
  warn that diversity could not be verified.

Every warning names the producer and review handles, both available targets, the
matching or unavailable axis, and points to review routing configuration. It is
emitted once immediately before an actual review spawn. It never alters the
already resolved review target and never blocks launch. An invalid review route
continues to fail under the existing routing contract; that is launch
correctness, not a diversity gate.

The launch receipt lives in the task tree, while the review target is recomputed
on every iteration. A producer restart therefore records the target of the
session that actually finishes it, and a review restart compares its newly
resolved target against the same receipt. Configuration may change between the
two without creating a cache, signal payload, route ledger, or other state
outside `.grove/`.

## Module interfaces

The tree-mutation module exposes one promotion operation returning a promotion
result with the node, relocated producer, review, and integration identities. It
hides path resolution, kind derivation, key allocation, templates, VCS-specific
movement, and rollback.

Task relationship parsing owns `Reviews`, `Integrates`, and the producer launch
receipt. Producer retirement asks that module to materialise the foreground
launch target before applying the normal filename-only terminal outcome.

Routing retains one resolver whose result contains an effective launch target.
A pure diversity comparison accepts an optional producer receipt plus the
resolved review target and returns `diverse`, the matching axes, or
`uncheckable`; the launch layer only renders that result. Callers and tests do
not reach through these interfaces to environment lookup or task-file parsing.

## Test seams

- Exercise promotion through the `grove-llm` interface in temporary trees:
  root and nested producers, all five producer kinds, source byte/handle/key
  preservation, unchanged sibling positions, fresh keys, no `BRIEF.md`, exact
  relationships/default task content, and pick order before and after retire.
- Inject a failure at every create, write, and rename point. Assert no stdout,
  no consumed key, no residual node, and the original producer bytes/path for
  each reported failure. Cover tracked Git, untracked Git, native Jujutsu, and
  colocated Jujutsu through the existing rename adapter.
- Test relationship and receipt parsing through generated leaves, including a
  `DONE` producer after renumbering. Missing or malformed metadata must produce
  `uncheckable`, never a launch refusal.
- Table-test target comparison for same harness only, same model only, both,
  neither, default-model selectors, and unknown receipts.
- Drive fake harnesses through the loop to prove the warning is emitted once,
  the resolved review command is unchanged, changed configuration compares to
  the historical producer receipt, and a fresh driver process reaches the same
  result from the tree.
- Assert the canonical Grove and doubt skills cover every row of the behavior
  table and that a non-Grove doubt session retains its existing bounded-cycle
  and optional cross-model behavior.

## Compatibility

Old binaries and hand-edited task files ignore the new freeform relationship and
receipt lines. Old or manually composed review leaves without them still launch
and receive an `uncheckable` warning; no tree migration is required. Composite
shapes remain reproducible with ordinary Markdown and filesystem operations,
but the previous claim that `leaf-add-chain` is byte-identical to three bare
`leaf-add` calls narrows: the manual form must also write the stable relationship
lines to reproduce the current generated shape.

Pick, brief-chain, node retirement, terminal infixes, the brief-less-node
discriminator, and the Herdr tree viewer continue to read filenames or
`BRIEF.md` presence exactly as before. No suffix or position becomes grammar.

## Out of scope

- Enforcing that every producer has a review or treating a chain as a scheduling
  unit.
- Blocking a review because its target is not diverse.
- Inferring model aliases or observing an interactive model switch inside a
  launched harness.
- Replacing research-pair breadth and combine discipline with doubt reviewers.
- Changing doubt-driven development in sessions that are not executing a picked
  Grove leaf.
- Promising crash-atomic file-to-directory replacement beyond the reported-error
  rollback contract shared by Grove's existing composite verbs.

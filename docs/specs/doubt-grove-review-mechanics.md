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
one in-session fresh-context reviewer across the whole leaf. One reviewer means
one independently materialised reviewer context: a diverse-lens pass with N
subagents spends N reviewers, while one reviewer asked to inspect several named
axes still spends one. If the work needs a second reviewer, or a substantive
actionable finding whose non-mechanical fix needs another review, the producer
becomes a Grove-managed review chain. The producer finishes only to a coherent
reviewable boundary; the fresh `review-*` session performs the next adversarial
pass.

The exceptions are findings that are trivial, noise, a visible accepted
trade-off, or a fix conclusively covered by an executable test seam. They do not
create a second review need.

### Ownership discriminator

Grove ownership is activated by a procedural fact, not an environment probe:
the current session ran Grove's Bootstrap, invoked `grove-llm pick` itself, and
adopted the returned leaf as its task. Merely finding `.grove/`, inheriting a
`GROVE_*` variable, or running a descendant command inside such a session does
not satisfy that predicate. A nested `grove do` session establishes its own
predicate with its own Bootstrap and pick; unrelated work does not silently
borrow the outer leaf's review allowance.

The launch context used for review-target receipts is deliberately not this
discriminator. It is inherited metadata and cannot decide which methodology a
session is following.

### Behavior by session kind

| Session | In-session doubt | If more review is needed |
|---|---|---|
| Plain `requirements`, `design`, `planning`, `prototype`, or `impl` producer | At most one fresh-context reviewer for the entire picked leaf; each diverse-lens subagent counts separately. | Promote the picked producer into a review chain. |
| Producer already in a review chain | None; its scheduled `review-*` leaf is the fresh-context review. | Finish to the scheduled review boundary and hand off. |
| `review-*` | None; this session is already the adversarial read and produces findings, not fixes. | Record the findings for its integration leaf. |
| `integrate-review-*` | At most one narrow reviewer. | Externalise substantial redesign as a new producer review chain inside the owning chain node so it runs before work outside that node; an integration leaf is not promotable. |
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
order and is written only after success. `--json` returns those paths and all
four stable handles plus a `changed` boolean as one object. `--json` failures use
a stable error code and a non-zero exit rather than mixing prose into stdout. The
second plain-text path is therefore the value the producer passes to
`leaf-retire`.

Promotion accepts only a live, currently picked producer of one of the five
producer kinds. It reads the task's declared kind through a strict,
non-degrading parser: the seventeen current labels and the supported legacy
`work` alias are accepted, while a missing, empty, or unrecognised token is an
error rather than `impl`. It refuses terminal leaves, `research`,
`combine-research`, `review-*`, and `integrate-review-*`.

An already-scheduled review is detected two ways. Stable relationship metadata
is authoritative when present; an immediate parent with no `BRIEF.md` is the
compatibility signal for a legacy or hand-cut chain whose metadata is absent.
The idempotent completed-promotion case is recognised before that refusal. A
refused run changes nothing, consumes no key, and names the appropriate Grove
action.

A retry by stable producer handle is idempotent while the already-created shape
is complete and that producer remains the picked leaf: it returns the same four
paths with `changed: false`. It also resumes the exact pending transaction for
that handle; it never allocates a second key run or nests another review chain.
A conflicting shape errors with the exact paths that need inspection.

### Crash-consistent transaction

The [Promotion transactions fail closed](../adr/promotion-transactions-fail-closed.md)
decision supplies the portable atomicity boundary.

The operation pre-validates the source, exact kind, derived kinds, destination,
generated content, and three-key allocation before writing. It then builds the
final children under a reserved sibling directory whose name is
`PROMOTING-<final-node-name>/`. The generated review and integration are written
first; the producer is moved byte-for-byte into child position `01` through
Grove's VCS-aware rename seam; the complete directory is landed by one
same-parent rename that strips `PROMOTING-`.

`PROMOTING-` is a reserved transaction prefix, not a task-tree entry. Every tree
reader and mutator first checks for it recursively. If one exists, `pick`,
`kind`, `resolve`, key allocation, and every grow/retire verb refuse with the
producer handle and the recovery command. They never skip the transaction and
continue into later work. Other foreign files remain leniently ignored.

This gives the operation one observable invariant across interruption:

- Before the producer move, the original producer is intact and the reserved
  transaction blocks every other tree operation.
- After the producer move, the producer and both generated leaves are together
  inside that same blocked transaction.
- After the landing rename, the only parsed shape is the complete review chain,
  whose first live child is the unchanged producer.

Every reported failure before landing attempts to reverse the producer move and
remove the transaction, leaving the original tree byte-for-byte. If rollback
itself fails, the reserved directory remains as a fail-closed recovery marker;
the error names its exact paths and `leaf-promote-chain <producer-handle>` is the
only mutator allowed to inspect and finish or reverse it. No partial state is
runnable, no new key can be issued around it, and plain or JSON stdout is emitted
only after the final rename succeeds. This is crash-consistent semantic
atomicity over portable filesystem operations, not an unsupported claim that a
file-to-directory replacement is one power-loss-atomic syscall.

## Producer handoff

After promotion, the producer session follows one sequence:

1. Apply only the bounded change needed to restore a coherent, reviewable
   artifact. Run executable checks, but spawn no second doubt reviewer.
2. Commit the artifact and promotion together, naming the unchanged producer
   handle in the commit message.
3. Retire the producer using the relocated path returned by promotion.
4. Run `grove-llm complete` as the final action.

Before the producer receives its `DONE` infix, retirement makes a best-effort
write of the producer's actual Grove launch target into the linked review task.
The write is advisory and may never block the terminal rename: missing or stale
session context, no unique sibling relationship, malformed metadata, or a write
failure emits a diagnostic and retirement still marks the producer `DONE`.

If the session stops after promotion but before retirement, pick returns the
still-live producer at child position `01`; the eventual retiring session
overwrites the receipt with its own target. Once retired, the unchanged
depth-first walk returns the review, then integration, before any later sibling
outside the chain.

## Advisory target diversity

The [Review target receipts](../adr/review-target-receipts.md) decision keeps the
comparison tied to the producer session that actually finished the artifact. An
effective launch target is the harness name plus the exact model selector Grove
passed to that harness; a harness-managed default is represented explicitly
rather than guessed.

After scrubbing inherited Grove session context, the loop driver's one real
foreground-session spawn exports a single `GROVE_SESSION_TARGET` JSON value with
the resolved worktree identity, harness, and nullable model selector. Every
other harness spawn uses the shared scrub helper and receives no such value. The
worktree identity is the same resolved root Grove uses for the loop, so a
meta-grove's tests in temporary trees and a nested grove cannot accidentally
claim the outer session's target.

Producer retirement materialises validated context in the linked review leaf
beside the stable `**Reviews:**` relation:

```markdown
**Producer launch:** {"harness":"claude","model":"opus"}
```

The `model` value is `null` for a harness-managed default. Defaults are scoped
to their harness rather than treated as one cross-vendor model identity.
Retirement accepts the ephemeral context only when its worktree identity matches
the retiring tree and the retiring producer is still the leaf returned by
`pick`. Descendants inherit environment, but those two checks make a stale value
unusable in another tree or for another leaf. A nested driver scrubs and replaces
the value for its own foreground session.

The receipt records the target that actually launched the producer session that
retires the task, not the target that the producer kind would resolve to under
later configuration. It does not record an interactive model change made inside
the harness, which Grove cannot observe.

The relationship lookup is deliberately local and cardinality-checked:
retirement scans only the producer's sibling tasks inside its brief-less chain
node and writes the receipt only when exactly one live or terminal review task
declares `**Reviews:** <producer-handle>`. Zero or multiple claimants are
`uncheckable`, never a tree-wide guess. The sibling task is rewritten through a
temporary file and rename, so an interrupted write cannot truncate it. A
successful receipt followed by a failed `DONE` rename is safe: the still-live
producer's eventual retirement overwrites the receipt.

At every `review-*` launch, Grove first resolves the review's own effective
target using the existing leaf → kind → family → stamp harness policy and the
existing harness-scoped → unscoped model policy. It then compares that target to
the producer launch receipt:

- If harnesses match, warn.
- If exact model selectors match, warn.
- If both match, emit one warning naming both matching axes.
- If both differ, stay silent.
- If the relationship or receipt is absent, malformed, or explicitly unknown,
  warn that diversity could not be verified.

Every warning names the producer and review handles, both available targets, the
matching or unavailable axis, and points to review routing configuration. The
driver renders it twice from one comparison result: a compact stderr diagnostic
immediately before spawn and a routing-notice block prepended to the review
session's normal prompt. The prompt copy survives a full-screen harness taking
over the terminal and remains visible in the session transcript. It is context
for the operator, not an instruction to soften the adversarial review.

The warning is emitted once per actual review spawn. It never alters the already
resolved review target and never blocks launch. An invalid review route continues
to fail under the existing routing contract; that is launch correctness, not a
diversity gate.

A one-harness installation will deliberately warn on every review because the
harness axis cannot differ. That is an accepted, visible consequence of the
confirmed requirement to warn unless both axes differ, not an exception signal
the implementation may silently suppress. Keeping the notice to one compact
block per spawn and making it transcript-visible bounds the cost.

The launch receipt lives in the task tree, while the review target is recomputed
on every iteration. A producer restart therefore records the target of the
session that actually finishes it, and a review restart compares its newly
resolved target against the same receipt. Configuration may change between the
two without creating a cache, signal payload, route ledger, or other state
outside `.grove/`.

## Module interfaces

The tree-mutation module exposes one promotion operation returning a promotion
result with the node, relocated producer, review, and integration identities. It
hides exact task-kind validation, relationship/legacy-chain checks, path
resolution, key allocation, templates, transaction recovery, VCS-specific
movement, and rollback. All tree operations share one pending-promotion guard;
callers do not reproduce the reserved-prefix scan.

Task relationship parsing owns `Reviews`, `Integrates`, and the producer launch
receipt. It exposes a cardinality-checked sibling lookup rather than a tree-wide
bag of metadata. Producer retirement asks that module to materialise validated
foreground launch context before applying the normal filename-only terminal
outcome, but treats every metadata failure as an advisory diagnostic.

Routing retains one resolver whose result contains an effective launch target.
A session-context parser validates worktree identity and current-pick identity
before yielding a producer target. A pure diversity comparison accepts an
optional producer receipt plus the resolved review target and returns `diverse`,
the matching axes, or `uncheckable`; the launch layer renders the same result to
stderr and the prompt. Callers and tests do not reach through these interfaces
to environment lookup or task-file parsing.

## Test seams

- Exercise promotion through the `grove-llm` interface in temporary trees:
  root and nested producers, all five producer kinds, the legacy `work` alias,
  strict refusal of missing/garbled kinds, source byte/handle/key preservation,
  unchanged sibling positions, fresh keys, no `BRIEF.md`, exact
  relationships/default task content, and pick order before and after retire.
  Cover metadata-bearing, legacy brief-less, and decomposition-node parents.
- Inject a reported failure at every create, write, and rename point. Assert no
  stdout, no consumed key, no residual transaction, and the original producer
  bytes/path after successful rollback. Separately interrupt after every
  mutation and assert every reader/mutator fails closed on `PROMOTING-`, retry
  reuses the same keys, and the first runnable leaf after recovery is always the
  producer. Cover tracked Git, untracked Git, native Jujutsu, and colocated
  Jujutsu through the existing rename adapter.
- Test relationship and receipt parsing through generated leaves, including a
  `DONE` producer after renumbering, zero and duplicate sibling claimants, an
  interrupted task-file rewrite, and a failed receipt write followed by a
  successful `DONE` rename. Metadata failures must produce `uncheckable`, never
  block retirement or launch.
- Exercise inherited session context with matching and mismatched worktree
  identities, a source that is and is not the current pick, a nested driver that
  replaces the outer target, and the meta-grove test scrub list.
- Table-test target comparison for same harness only, same model only, both,
  neither, default-model selectors, and unknown receipts.
- Drive fake harnesses through the loop to prove the warning is emitted once,
  appears in both stderr and the launched prompt, leaves the resolved review
  command unchanged, compares changed configuration to the historical producer
  receipt, and reaches the same result from a fresh driver process.
- Assert contradiction-shaped documentation facts: Grove's three-round
  in-session loop, old size/vendor-only escalation trigger, and "no retrofit
  verb" text are absent; one-review-per-picked-leaf, promotion, integration
  placement, research exclusions, and non-Grove bounded-cycle behavior are
  present. Do not substitute a row-coverage assertion that can pass while both
  old and new rules coexist.

## Canonical surfaces

Implementation is incomplete until the current-state contract is reconciled in
all of these places:

- `CONTEXT.md`: Review chain, ownership discriminator, promotion transaction,
  launch receipt, and the `DONE` side effect.
- `content/SKILL.md`, `content/driving.md`, and `content/TASK-FORMAT.md`: the
  leaf-wide reviewer allowance, exact promotion/handoff sequence, integration
  placement, research exclusions, and replacement of the old three-round and
  manual-retrofit guidance.
- `plugins/linkuistics/skills/doubt-driven-development/SKILL.md`: an explicit
  Grove composition section that suspends per-artifact verification, re-looping,
  diverse-lens, and optional cross-model review as necessary to obey the
  leaf-wide allowance; standalone behavior remains unchanged.
- `docs/ARCHITECTURE.md`: transaction/pending-reader semantics beside the task
  tree, stable relationships beside composition, and receipt comparison beside
  task-kind routing.
- `grove-llm --help`, `grove-llm leaf-promote-chain --help`, `docs/USAGE.md`,
  and `docs/CONFIGURATION.md`: the operation, recovery diagnostic, advisory
  warning, and routing knobs that resolve it. `GROVE_SESSION_TARGET` is reserved
  internal context, not a user configuration knob.

## Compatibility

Old binaries and hand-edited task files ignore the new freeform relationship and
receipt lines. Old or manually composed review leaves without them still launch
and receive an `uncheckable` warning; no tree migration is required. A producer
already inside a brief-less legacy chain is not promotable even without
metadata. Composite shapes remain reproducible with ordinary Markdown and
filesystem operations, but the previous claim that `leaf-add-chain` is
byte-identical to three bare `leaf-add` calls narrows: the manual form must also
write the stable relationship lines to reproduce the current generated shape.

`PROMOTING-` is the one newly reserved foreign-name prefix. A current binary
must recover such a directory before any tree work proceeds; all other foreign
files retain the existing lenient behavior. The strict promotion parser accepts
the legacy `work` spelling as `impl` but never applies the general read-side
degradation to malformed task files.

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
- Claiming one power-loss-atomic file-to-directory replacement syscall. The
  portable contract is a fail-closed transaction plus an atomic final directory
  rename.

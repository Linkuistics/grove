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

Every participating task-tree operation enters one tree-access seam before it
checks for a pending promotion or reads names. The seam holds a process-scoped
advisory lock on the open root `.grove/BRIEF.md`: readers (`pick`, `kind`,
`resolve`, `brief-chain`) take a shared lock for the whole read, while every
mutator takes an exclusive lock from pre-validation through rollback or success
output. Lock acquisition waits for the current operation and process termination
releases it automatically; no PID, owner record, or lock-state bytes are stored
in the task tree. `root-init`, which has no root brief to lock, retains its
atomic create-or-refuse contract. Direct human edits remain outside this
cooperative CLI serialization, as they are for every Grove invariant.

Promotion acquires the exclusive lock before it pre-validates the source, exact
kind, derived kinds, destination, generated content, and three-key allocation.
It then builds the final children under a reserved sibling directory whose name
is `PROMOTING-<final-node-name>/`. The generated review and integration are
written first; the producer is moved byte-for-byte into child position `01`
through Grove's VCS-aware rename seam; the complete directory is landed through
that same seam by a same-parent rename that strips `PROMOTING-`. The lock makes
the validation, key allocation, and landing one serializable tree mutation;
`PROMOTING-` remains the durable fail-closed witness if the process disappears
and releases the lock mid-operation.

Both the source-to-staging move and staging-to-final landing obey the existing
Jujutsu-first VCS rule. Native and colocated Jujutsu use filesystem renames and
never touch Git's index. In a plain Git tree, a tracked producer is moved at
both stages through the Git-aware adapter so the index ends with only the final
child path and no `PROMOTING-` path; an untracked producer remains untracked.
Rollback applies the inverse moves through the same adapter. Generated tasks
move with their containing directory without becoming staged merely because a
tracked producer shares that directory.

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
only after the final rename succeeds.

The guarantee is process-interruption consistency: after a filesystem call has
returned, a concurrent Grove process or a process restarted after exit, signal,
or panic observes either the original producer plus a blocking transaction or
the complete chain. Grove does not `fsync` generated files or parent directories,
so it makes no durability claim across power loss, kernel failure, storage-cache
loss, or a filesystem that violates its documented rename semantics. The final
same-parent rename is an atomic namespace-visibility seam, not a power-loss
commit. Power-loss durability would require an ordered file-and-directory sync
protocol and is outside this design.

## Producer handoff

After promotion, the producer session follows one sequence:

1. Apply only the bounded change needed to restore a coherent, reviewable
   artifact. Run executable checks, but spawn no second doubt reviewer.
2. Commit the artifact and promotion together, naming the unchanged producer
   handle in the commit message.
3. Retire the producer using the relocated path returned by promotion.
4. Run `grove-llm complete` as the final action.

While the producer is still the factual pick, retirement validates and snapshots
the candidate launch context and unique sibling relationship under the exclusive
tree lock. It applies the producer's `DONE` rename first. Only after that
terminal transition succeeds does it make a best-effort atomic rewrite of the
review task with the producer launch receipt. A failed `DONE` rename writes no
receipt; a failed post-`DONE` rewrite leaves the initial generated review task
receipt-free and therefore uncheckable. Under this protocol a valid receipt is
never present beside a live producer, so a prior session whose terminal rename
failed cannot leave an authoritative target for a later finisher to preserve.

The metadata write remains advisory and may never reverse or mask the successful
terminal rename: missing or stale session context, no unique sibling
relationship, malformed metadata, or a write failure emits a diagnostic and
retirement still reports the producer `DONE`. The next review warns that
diversity is uncheckable when no valid receipt was materialised.

If the session stops after promotion but before retirement, pick returns the
still-live producer at child position `01`; no producer receipt exists yet. The
session that eventually performs the successful terminal rename is the only one
that may write the receipt. Once retired, the unchanged depth-first walk returns
the review, then integration, before any later sibling outside the chain.

## Advisory target diversity

The [Review target receipts](../adr/review-target-receipts.md) decision keeps the
comparison tied to the producer session that actually finished the artifact. An
effective launch target is the harness name plus the exact model selector Grove
passed to that harness; a harness-managed default is represented explicitly
rather than guessed.

After scrubbing inherited Grove session context, the loop driver's one real
foreground-session spawn exports a single `GROVE_SESSION_TARGET` JSON value with
the resolved worktree identity, the stable handle of the leaf used by that exact
routing peek, the harness, and the nullable model selector. The launch resolver
retains the handle beside the kind and target, and the launch diagnostic renders
that retained value rather than performing a second pick. Every other harness
spawn uses the shared scrub helper and receives no such value. The worktree
identity is the same resolved root Grove uses for the loop, so a meta-grove's
tests in temporary trees and a nested grove cannot accidentally claim the outer
session's target.

The driver may have no routed leaf during fresh-grove start, and a `leaf-insert`
may make the session's factual pick differ from the earlier routing peek. Both
cases deliberately yield an uncheckable producer target: retirement accepts the
ephemeral context only when its worktree matches, its routed-leaf handle equals
the retiring producer, and that producer is still the leaf returned by the
retirement-time `pick`. The routed handle verifies what target actually
launched; it never overrides the task tree's factual pick.

Producer retirement materialises validated context in the linked review leaf
beside the stable `**Reviews:**` relation:

```markdown
**Producer launch:** {"producer":"sync-design-k12","harness":"claude","model":"opus"}
```

The `producer` must agree with the stable `**Reviews:**` relationship. The
`model` value is `null` for a harness-managed default. Model identity is exact
and implementable: two non-null selectors match when their strings match;
`null` matches `null` only when the harness names also match; and a null and
non-null selector never match. Equivalently, a default model's identity is
`default(<harness>)`, not one cross-vendor `default`. Descendants inherit
environment, but the worktree, routed-leaf, and factual-pick checks make a stale
value unusable in another tree or for another leaf. A nested driver scrubs and
replaces the value for its own foreground session.

The receipt records the target that actually launched the producer session that
retires the task, not the target that the producer kind would resolve to under
later configuration. It does not record an interactive model change made inside
the harness, which Grove cannot observe.

The relationship lookup is deliberately local and cardinality-checked:
retirement scans only the producer's sibling tasks inside its brief-less chain
node and writes the receipt only when exactly one live or terminal review task
declares `**Reviews:** <producer-handle>`. Zero or multiple claimants are
`uncheckable`, never a tree-wide guess. After the terminal producer rename, the
sibling task is rewritten through a temporary file and rename, so an interrupted
write cannot truncate it. There is no receipt-before-`DONE` state for a later
finisher to mistake as current.

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

A warning always names the review handle. It names a producer handle only when
one came from a valid stable `**Reviews:**` declaration (and a receipt's
`producer` field must agree with it). With an absent or malformed relationship,
the warning renders `producer=unknown` plus the exact reason; it never infers a
producer from sibling position, a filename suffix, or the preceding leaf. When
available, the warning also renders both targets, the matching or unavailable
axis, and the review routing configuration. A null model is rendered as
`default(<harness>)`, making cross-harness default comparison visible.

The driver renders the result twice from one comparison value: a compact stderr
diagnostic immediately before spawn and a routing-notice block prepended to the
review session's normal prompt. The prompt copy survives a full-screen harness
taking over the terminal and remains visible in the session transcript. It is
context for the operator, not an instruction to soften the adversarial review.

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

The task-tree access module exposes shared-read and exclusive-mutation guards
over one Grove root. Exported operations acquire exactly once and pass the guard
through internal lock-neutral helpers, so a mutator can call pick/resolve logic
without recursively locking or opening a race. Pending-promotion detection runs
only while a guard is held.

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
A session-context parser validates worktree identity, routed-leaf identity, and
current-pick identity before yielding a producer target. A pure diversity
comparison accepts an optional relationship identity, optional producer receipt,
and the resolved review target; it returns `diverse`, the matching axes, or an
`uncheckable` reason with an optional producer handle. The launch layer renders
the same result to stderr and the prompt. Callers and tests do not reach through
these interfaces to environment lookup or task-file parsing.

## Test seams

- Exercise promotion through the `grove-llm` interface in temporary trees:
  root and nested producers, all five producer kinds, the legacy `work` alias,
  strict refusal of missing/garbled kinds, source byte/handle/key preservation,
  unchanged sibling positions, fresh keys, no `BRIEF.md`, exact
  relationships/default task content, and pick order before and after retire.
  Cover metadata-bearing, legacy brief-less, and decomposition-node parents.
- Drive two subprocesses through barriers around pre-validation and allocation:
  promotion versus promotion and promotion versus every existing mutator. Assert
  the second operation waits, positions and tree-wide keys remain unique, and a
  shared reader can observe neither a missing producer nor a later runnable leaf.
  Kill the lock holder and prove the waiter acquires the released lock and then
  recovers any `PROMOTING-` witness.
- Inject a reported failure at every create, write, and rename point. Assert no
  stdout, no consumed key, no residual transaction, and the original producer
  bytes/path after successful rollback. Separately interrupt after every
  mutation and assert every reader/mutator fails closed on `PROMOTING-`, retry
  reuses the same keys, and the first runnable leaf after recovery is always the
  producer. These are process-interruption tests, not power-loss simulations.
  Cover tracked Git, untracked Git, native Jujutsu, and colocated Jujutsu through
  the existing rename adapter; after landing and rollback, assert Git's index has
  no `PROMOTING-` path or accidentally staged generated task.
- Test relationship and receipt parsing through generated leaves, including a
  `DONE` producer after renumbering, zero and duplicate sibling claimants, an
  interrupted task-file rewrite, a failed `DONE` rename that writes no receipt,
  and a successful `DONE` rename followed by a failed receipt write. The last
  case must leave no authoritative prior-session target. Metadata failures must
  produce `uncheckable`, never block retirement or launch.
- Exercise inherited session context with matching and mismatched worktree
  identities, matching and mismatched routed-leaf handles, a source that is and
  is not the current pick, a launch-window `leaf-insert`, a nested driver that
  replaces the outer target, fresh-grove start with no routed leaf, and the
  meta-grove test scrub list.
- Table-test target comparison for same harness only, same model only, both,
  neither, same-harness null defaults, cross-harness null defaults, null versus
  explicit selectors, and unknown receipts.
- Drive fake harnesses through the loop to prove the warning is emitted once,
  appears in both stderr and the launched prompt, leaves the resolved review
  command unchanged, compares changed configuration to the historical producer
  receipt, reaches the same result from a fresh driver process, and renders an
  absent/malformed relationship as `producer=unknown` without naming a sibling.
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
- Providing power-loss durability. The portable contract is process-interruption
  consistency through serialization, a fail-closed transaction, and an atomic
  final directory rename; it performs no ordered `fsync` protocol.

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
stable key/handle that resolves to it. A path argument is reduced to its stable
producer handle before the operation resolves the current location, so the same
path remains an idempotent retry after promotion relocated or retired the
producer. The exact `PROMOTING-*` path named by a fail-closed diagnostic is also
accepted, but only as a recovery reference while that transaction exists. The
operation takes no parent, stem, kind, or harness flags: all four are facts
already carried by the picked producer, and review routing remains policy-owned.

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

An already-scheduled review is detected two ways. Promotion scans the producer's
whole sibling level for tasks declaring `**Reviews:** <producer-handle>`;
exactly one is authoritative, multiple claimants are a malformed conflict, and
the scan works for both node-contained and metadata-bearing flat chains. A
non-root immediate parent with no `BRIEF.md` is the structural compatibility
signal for a composition-managed node whose metadata is absent. The grove root
is excluded because its own `BRIEF.md` is optional and its absence cannot
classify every root-level producer as already composed. The idempotent
completed-promotion case is recognised before either refusal. A metadata-free
flat chain has no stable relationship Grove can discover without turning its
suffixes or positions into grammar; promotion does not guess. Such a legacy
chain must be annotated with the stable relationship before its producer is
passed to promotion. A refused run changes nothing, consumes no key, and names
the appropriate Grove action.

A retry by stable producer handle, or by the stale picked path from which that
handle can be recovered, is idempotent while the already-created shape is
complete: it returns the same four current paths with `changed: false` whether
the relocated producer is live, terminal, still picked, or pre-empted by a later
insert. Completed-shape recognition is therefore ordered before liveness and
current-pick validation; those gates apply only to a new promotion. The same
reference also resumes the exact pending transaction for that handle; the
explicit `PROMOTING-*` path is the recovery form when only a generic reader's
diagnostic is available. Pending recovery is selected before ordinary path
resolution, liveness, kind, or current-pick validation, because a producer
already moved into the reserved directory is deliberately invisible to those
normal readers. Recovery scans only the reserved transaction for the stable
producer identity and never allocates a second key run or nests another review
chain. A conflicting shape errors with the exact paths that need inspection.

### Crash-consistent transaction

The [Promotion transactions fail closed](../adr/promotion-transactions-fail-closed.md)
decision supplies the portable atomicity boundary.

Every participating task-tree operation enters one tree-access seam before it
checks for a pending promotion or reads names. The seam holds a process-scoped
advisory lock on an open descriptor for the root `.grove/` directory, whose
existence is the invariant every steady-state tree command already requires;
briefs remain lazy and optional. Readers (`pick`, `kind`, `resolve`,
`brief-chain`) take a shared lock for the whole read, while every mutator takes
an exclusive lock from pre-validation through rollback or success output.
Lock acquisition first tries without blocking; on contention it prints one
`waiting for active Grove tree operation` diagnostic and then waits without a
timeout. Process termination releases the lock automatically; no PID, owner
record, lock file, or lock-state bytes are stored in the task tree. `root-init`,
which creates `.grove/` itself, retains its atomic create-or-refuse contract.
Direct human edits remain outside this cooperative CLI serialization, as they
are for every Grove invariant.

Promotion acquires the exclusive lock and scans for a pending promotion first.
A matching producer handle, stale picked path, or exact transaction path enters
recovery before the normal producer gates; any other pending transaction refuses
the call. Only with no pending transaction does promotion pre-validate the
source, exact kind, derived kinds, destination, generated content, and three-key
allocation. It then builds the final children under a reserved sibling directory
whose name is `PROMOTING-<final-node-name>/`. The generated review and
integration are written first and the producer is moved byte-for-byte into child
position `01` through Grove's VCS-aware rename seam. The lock makes validation,
key allocation, index preparation, and landing one serializable tree mutation;
`PROMOTING-` remains the durable fail-closed witness if the process disappears
and releases the lock before the final filesystem rename.

The source-to-staging move obeys the existing Jujutsu-first VCS rule. Native and
colocated Jujutsu use a filesystem rename and never touch Git's index. In a
plain Git tree, the existing Git-aware adapter moves a tracked producer into
staging; an untracked producer remains untracked.

The staging-to-final landing has a narrower transaction-specific adapter. For a
tracked producer in plain Git, Grove first rewrites the already-staged producer
index entry from its `PROMOTING-*` child path to the final child path while the
reserved directory is still present. That index rewrite is itself committed
under Git's index lock and preserves the stage-0 entry's blob, mode, and index
flags; an unmerged/multi-stage producer is rejected before any mutation. It does
not add the generated review or integration. Interruption before or during the
index transaction therefore leaves the on-disk `PROMOTING-*` witness in place.
Once the index names only the final producer path, Grove lands the complete
directory with one plain same-parent filesystem rename. After that syscall
returns, both the parsed tree and Git's index name only final paths; there is no
filesystem-complete/index-stale window in which a commit can capture the
reserved prefix. Jujutsu and an untracked Git producer need no index preparation
and use the same final filesystem rename. Recovery and rollback accept the
source, staging, or final index spelling that an interrupted Git preparation may
leave and normalise it while the reserved witness still blocks every other tree
operation.

`PROMOTING-` is a reserved transaction prefix, not a task-tree entry. Every tree
reader and mutator first checks for it recursively. If one exists, `pick`,
`kind`, `resolve`, key allocation, and every grow/retire verb refuse with the
exact reserved path and
`grove-llm leaf-promote-chain <reserved-path>` as the recovery command. A generic
reader does not read generated task contents or infer a producer from child
position merely to improve that diagnostic. The promotion recovery branch may
scan the reserved transaction after it holds the exclusive lock. Readers never
skip the transaction and continue into later work. Other foreign files remain
leniently ignored.

This gives the operation one observable invariant across interruption:

- Before the producer move, the original producer is intact and the reserved
  transaction blocks every other tree operation.
- After the producer move, the producer and both generated leaves are together
  inside that same blocked transaction.
- After the landing rename, the only parsed shape is the complete review chain,
  whose first live child is the unchanged producer.

Every reported failure before landing attempts to reverse the producer move,
normalise any prepared Git index entry, and remove the transaction, leaving the
original tree byte-for-byte. If rollback itself fails, the reserved directory
remains as a fail-closed recovery marker; the error names its exact paths and
`leaf-promote-chain <reserved-path>` is the only mutator allowed to inspect and
finish or reverse it. No partial state is runnable, no new key can be issued
around it, and plain or JSON stdout is emitted only after the final rename
succeeds.

The guarantee is process-interruption consistency: after a filesystem or
Git-index transaction has returned, a concurrent Grove process or a process
restarted after exit, signal, or panic observes either the original producer
plus a blocking transaction, a blocking transaction recoverable across its
possible Git index spellings, or the complete chain with final index paths.
Grove does not `fsync` generated files, the index, or parent directories, so it
makes no durability claim across power loss, kernel failure, storage-cache loss,
or a filesystem that violates its documented rename semantics. The final
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

While the retiring leaf is still the factual pick, retirement validates and
snapshots the candidate launch context under the exclusive tree lock. Its
receipt candidates are the leaf itself and every brief-carrying ancestor that
this `DONE` transition will leave with no live descendant. A candidate produces
a plan only when one sibling review explicitly declares `**Reviews:**` for its
stable handle. The same factual leaf may therefore supply the handoff target for
several newly closing reviewed ancestors, matching the Retire cascade in which
that session verifies and reports each close.

Retirement applies the leaf's `DONE` rename first. Only after that terminal
transition succeeds does it make each prepared best-effort atomic replacement
of a review task's producer launch receipt. Replacement is unconditional: an
existing `**Producer launch:**` line is overwritten rather than preserved or
treated as idempotent. A failed `DONE` rename writes no receipt; a failed
post-`DONE` write leaves a normally generated review task receipt-free and
therefore uncheckable. Metadata failure never reverses or masks the one
lifecycle write. A prepared plan retains the review path and new receipt facts,
not a rendering of the task's pre-`DONE` contents. Materialisation re-reads the
task after `DONE`, replaces only the receipt line in that current text, and
atomically renames the result. Grove's tree lock excludes cooperating commands;
a direct editor racing between that final read and rename remains outside the
same cooperative guarantee as every other task-tree mutation.

Every prepared receipt carries a producer generation: the greatest permanent
key at or below the reviewed producer entity. For a leaf this is its own key; for
a node it is the maximum key in its subtree. Terminal entries remain in place,
and every supported node reopen adds a fresh globally monotonic key, so a stale
receipt from an earlier close fails generation validation if replacement after
the new close fails. Reordering leaves generation unchanged. Directly removing
a leaf producer's `DONE` infix remains outside the cooperative guarantee: that
hand edit issues no key, so a failed later replacement can leave an
indistinguishable receipt and must diagnose that it may be stale.

The receipt does not schedule review. If a producer is reopened after its
linked review is already terminal, the new close may replace that terminal
task's evidence but does not reactivate it. A new generation that needs review
must be represented by new tree work, consistent with Grove's rule that review
composition is habitual rather than enforced.

The metadata write remains advisory and may never reverse or mask the successful
terminal rename: missing or stale session context, no unique sibling
relationship, malformed metadata, or a write failure emits a diagnostic and
retirement still reports the producer `DONE`. The next review warns that
diversity is uncheckable when no valid receipt was materialised.

If the session stops after promotion or decomposition but before retirement,
pick returns the still-live producer or its first live descendant; no current
producer receipt exists yet. The session that eventually performs the
successful terminal rename that closes the producer entity supplies the
receipt. Once the entity has no live descendant, the unchanged depth-first walk
returns the review, then integration, before any later sibling outside the
chain.

`leaf-prune` does not supply a producer handoff target. If an `ABANDONED`
transition removes the final live descendant, the linked review remains
uncheckable: that session records a human decision against a path, not the
target of work that produced the aggregate artifact.

## Advisory target diversity

The [Review target receipts](../adr/review-target-receipts.md) decision keeps the
comparison tied to the factual session that handed the producer entity to
review. An effective launch target is the harness name plus the exact model
selector Grove passed to that harness; a harness-managed default is represented
explicitly rather than guessed.

After scrubbing inherited Grove session context, the loop driver's one real
foreground-session spawn exports a single `GROVE_SESSION_TARGET` JSON value with
the resolved worktree identity, the stable handle of the leaf used by that exact
routing peek, the harness, and the nullable model selector. The peek is one
structured `grove-llm kind --with-harness --json` call returning the picked
leaf's path, stable handle, kind, optional declared harness, and validated review
evidence from one tree-read guard. A leaf result is exactly one JSON object with
string `path`, `handle`, and `kind` fields, `harness` as a string or `null`, and
`review` as `null` for a non-review or a checkable/uncheckable evidence object,
for example:

```json
{"path":"/work/.grove/05-sync-design-k12.md","handle":"sync-design-k12","kind":"design","harness":null,"review":null}
```

For a review, the evidence object carries either the validated producer receipt
or an `uncheckable` reason and the producer handle only when it came from a valid
`Reviews` relationship. It does not carry the review's effective target, which
the driver resolves after the peek. The driver retains this evidence beside the
routed leaf and performs the pure target comparison later; it never re-opens the
task tree to reconstruct review metadata outside the peek's guard.

```json
{"review":{"status":"checkable","producer":"sync-design-k12","session":"sync-docs-k27","generation":"k27","harness":"claude","model":"opus"}}
{"review":{"status":"uncheckable","producer":"sync-design-k12","reason":"producer-generation-mismatch"}}
```

Those fragments show the two shapes; they are nested in the full peek object.
`producer` is `null` rather than guessed when no valid relationship supplied it.

No live leaf is the JSON literal `null`. Failure is non-zero with no partial JSON
on stdout. The launch resolver retains that one result beside the effective
target; readiness output, the launch diagnostic, and the session-target value
render it without a second pick. A non-null structured result that lacks a
handle or `review` field, is malformed, or came from an old binary that does not
implement the JSON contract is a routing-peek/version-skew failure and stops
before launch under the existing no-guess rule. Every other harness spawn uses
the shared scrub helper and receives no session-target value. The worktree
identity is the same resolved
root Grove uses for the loop, so a meta-grove's tests in temporary trees and a
nested grove cannot accidentally claim the outer session's target.

The driver may have no routed leaf during fresh-grove start, and a `leaf-insert`
may make the session's factual pick differ from the earlier routing peek. Both
cases deliberately yield an uncheckable producer target: retirement accepts the
ephemeral context only when its worktree matches, its routed-leaf handle equals
the retiring source session, and that session is still the leaf returned by the
retirement-time `pick`. The routed handle verifies what target actually
launched; it never overrides the task tree's factual pick or substitutes for the
explicit producer relationship.

Producer retirement materialises validated context in the linked review leaf
beside the stable `**Reviews:**` relation. A direct leaf producer records itself
as both reviewed producer and source session:

```markdown
**Producer launch:** {"producer":"sync-design-k12","session":"sync-design-k12","generation":"k12","harness":"claude","model":"opus"}
```

A decomposed producer records the node as the reviewed entity and the factual
leaf whose retirement closed it as the source session:

```markdown
**Producer launch:** {"producer":"sync-design-k12","session":"sync-docs-k27","generation":"k27","harness":"claude","model":"opus"}
```

`producer` must agree with the stable `**Reviews:**` relationship. `session`
names whose effective target follows, and must agree with the validated routed
handle and factual pick used to prepare the close. `generation` is the maximum
permanent key at or below the producer entity, encoded as `k<key>`, and must
still match the task tree at review launch. The `model` value is `null` for a
harness-managed default.
Model identity is exact and implementable: two non-null selectors match when
their strings match; `null` matches `null` only when the harness names also
match; and a null and non-null selector never match. Equivalently, a default
model's identity is `default(<harness>)`, not one cross-vendor `default`.
Descendants inherit environment, but the worktree, routed-session, factual-pick,
closing-ancestor, and generation checks make stale context unusable in another
tree, for another leaf, or after a supported node reopen. A nested driver scrubs
and replaces the value for its own foreground session.

Receipt preparation establishes the event facts while they are observable. For
a direct producer, `session` is the producer itself. For a closing node,
`session` is a descendant factual leaf and every other descendant is already
terminal before its transition. Review-peek validation checks the static facts
that remain: the explicit producer resolves to a terminal leaf or a
brief-carrying node with no live descendants; the source is that leaf or a
terminal descendant; and the generation matches. It cannot reconstruct the
historical order of hand-edited Markdown and does not pretend that advisory
metadata is authenticated.

The receipt records the target that actually launched the session that hands the
producer entity to review, not the target that the producer kind would resolve
to under later configuration. For a decomposed node this is intentionally one
handoff context, not an aggregate of every contributing child target. It does
not record an interactive model change made inside the harness, which Grove
cannot observe.

The relationship lookup is deliberately local and cardinality-checked: for
each direct or newly closing producer candidate, retirement scans that entity's
whole sibling level and writes the receipt only when exactly one live or
terminal review task declares `**Reviews:** <producer-handle>`. This supports
leaf and decomposed-node producers in current brief-less chain nodes, plus
metadata-bearing flat chains, without a tree-wide guess. Zero or multiple
claimants are `uncheckable`. After the terminal leaf rename, each sibling task
is rewritten through a temporary file and rename, so an interrupted write
cannot truncate it. There is no Grove-written current-generation receipt before
the `DONE` transition that closes its producer entity.

At every `review-*` launch, Grove first resolves the review's own effective
target using the existing leaf → kind → family → stamp harness policy and the
existing harness-scoped → unscoped model policy. It then compares that target to
the producer launch receipt:

- If harnesses match, warn.
- If exact model selectors match, warn.
- If both match, emit one warning naming both matching axes.
- If both differ, stay silent.
- If the receipt's source session or producer generation cannot be validated,
  warn that comparison is uncheckable.
- If the relationship or receipt is absent, malformed, or explicitly unknown,
  warn that diversity could not be verified.

A warning always names the review handle. It names a producer handle only when
one came from a valid stable `**Reviews:**` declaration. If a syntactically
valid receipt's `producer` disagrees with that relationship, the comparison is
`uncheckable(reason=receipt-producer-mismatch)`, names the relationship's
producer, and does not treat the receipt claimant as an identity source. With an
absent or malformed relationship, the warning renders `producer=unknown` plus
the exact reason; it never infers a producer from sibling position, a filename
suffix, or the preceding leaf. When available, the warning also renders both
targets, the matching or unavailable axis, and the review routing
configuration. A null model is rendered as `default(<harness>)`, making
cross-harness default comparison visible.

The driver renders the result twice from one comparison value: a compact stderr
diagnostic immediately before spawn and a routing-notice block prepended to the
review session's normal prompt. The prompt copy survives a full-screen harness
taking over the terminal and remains visible in the session transcript. It is
context for the operator, not an instruction to soften the adversarial review.

The warning is emitted once per actual review spawn. It never alters the already
resolved review target and never blocks launch. An invalid review route continues
to fail under the existing routing contract; that is launch correctness, not a
diversity gate.

The guarded review evidence remains a routing forecast, not a reservation. A
tree mutation after the peek can pre-empt the review before the harness starts;
the session's own Bootstrap and factual `pick` still win, so it works the newly
live producer descendant rather than the routed review. The next loop iteration
re-derives both route and evidence from the tree.

A one-harness installation will deliberately warn on every review because the
harness axis cannot differ. That is an accepted, visible consequence of the
confirmed requirement to warn unless both axes differ, not an exception signal
the implementation may silently suppress. Keeping the notice to one compact
block per spawn and making it transcript-visible bounds the cost.

The launch receipt lives in the task tree, while the review target and producer
generation are recomputed on every iteration. A direct producer restart records
the target of the session that retires it; a decomposed producer restart records
the target of the factual descendant session that closes it. A review restart
compares its newly resolved target against the same validated receipt.
Configuration may change between the two without creating a cache, signal
payload, route ledger, or other state outside `.grove/`.

## Module interfaces

The task-tree access module exposes shared-read and exclusive-mutation guards
over one open Grove-root directory descriptor. Exported operations acquire
exactly once and pass the guard through internal lock-neutral helpers, so a
mutator can call pick/resolve logic without recursively locking or opening a
race. Pending-promotion detection runs only while a guard is held. The routing
peek returns path, handle, kind, declared harness, and validated review evidence
from that same guarded read; launch callers never reconstruct leaf or receipt
identity with another tree read.

The tree-mutation module exposes one promotion operation returning a promotion
result with the node, relocated producer, review, and integration identities. It
hides exact task-kind validation, sibling relationship checks, stale-path
normalisation, key allocation, templates, transaction recovery, Git index
preparation, VCS-specific movement, and rollback. All tree operations share one
pending-promotion guard; callers do not reproduce the reserved-prefix scan.

Task relationship parsing owns `Reviews`, `Integrates`, and the producer launch
receipt. It exposes a cardinality-checked sibling lookup for an explicit
producer entity rather than a tree-wide bag of metadata. The lifecycle module
identifies the retiring leaf plus the reviewed decomposition ancestors its
transition would close, and computes each producer generation through the task
tree seam. Producer retirement asks the relationship module to prepare receipt
plans from that candidate set before applying the normal filename-only terminal
outcome. A plan stores facts rather than rendered task content; materialisation
re-reads after `DONE`, replaces only the receipt marker, and treats every
metadata failure as an advisory diagnostic.

Routing retains one resolver whose result contains an effective launch target.
A session-context parser validates worktree identity, routed-leaf identity, and
current-pick identity before yielding a handoff target. A pure diversity
comparison accepts an optional relationship identity, optional producer receipt,
and the resolved review target; it returns `diverse`, the matching axes, or an
`uncheckable` reason with an optional producer handle. The launch layer renders
the same result to stderr and the prompt. Callers and tests do not reach through
these interfaces to environment lookup or task-file parsing.

The loop driver retains the structured peek's review evidence through target
resolution. No launch-layer helper re-reads a review task or producer subtree
after the tree guard is released.

## Test seams

- Exercise promotion through the `grove-llm` interface in temporary trees:
  root and nested producers (including a root with no `BRIEF.md`), all five
  producer kinds, the legacy `work` alias, strict refusal of missing/garbled
  kinds, source byte/handle/key preservation, unchanged sibling positions, fresh
  keys, no generated `BRIEF.md`, exact relationships/default task content, and
  pick order before and after retire. Cover metadata-bearing, non-root legacy
  brief-less, and decomposition-node parents.
- Drive two subprocesses through barriers around pre-validation and allocation:
  promotion versus promotion and promotion versus every existing mutator. Assert
  the second operation emits one wait diagnostic, positions and tree-wide keys
  remain unique, and a shared reader can observe neither a missing producer nor
  a later runnable leaf. The second promoter must accept the same now-stale path
  and return the completed shape with `changed: false`, including when retirement
  or a new insert acquires the lock first and makes the producer terminal or no
  longer picked. Kill the lock holder and prove the waiter acquires the released
  lock and then recovers any `PROMOTING-` witness before applying ordinary
  picked-producer gates.
- Inject a reported failure at every create, write, and rename point. Assert no
  stdout, no consumed key, no residual transaction, and the original producer
  bytes/path after successful rollback. Separately interrupt after every
  mutation and assert every reader/mutator fails closed on `PROMOTING-`, the
  diagnostic names only the exact transaction path and path-based recovery
  command, retry reuses the same keys, and the first runnable leaf after recovery
  is always the producer. These are process-interruption tests, not power-loss
  simulations. Cover tracked Git, untracked Git, native Jujutsu, and colocated
  Jujutsu. In tracked Git, interrupt before, during, and after the index-prefix
  rewrite and final filesystem rename; every pre-land state must retain the
  reserved directory, and every post-land state must have only final index paths
  with no accidentally staged generated task. Preserve a normal stage-0 entry's
  blob, mode, and index flags, and refuse an unmerged producer before mutation.
- Test relationship and receipt parsing through generated leaves, including a
  `DONE` producer after renumbering, zero and duplicate sibling claimants, an
  interrupted task-file rewrite, a failed `DONE` rename that writes no receipt,
  a pre-existing receipt that a successful retirement unconditionally replaces,
  direct and decomposed producers whose receipts name distinct producer/session
  handles, one leaf closing multiple reviewed decomposition ancestors, and
  generation stability under reorder versus change after supported reopen,
  a receipt whose producer disagrees with the `Reviews` relationship, and a
  successful `DONE` rename followed by a failed receipt write. The normal
  receipt-free failure case must leave no authoritative prior-session target;
  the hand-edited direct-leaf failure must diagnose that the remaining value may
  be stale, while a stale decomposed-node generation must be rejected
  mechanically. Edit the review task after receipt preparation but before
  materialisation and prove the post-`DONE` re-read preserves that edit. Also
  prove reopening a producer whose linked review is already terminal does not
  reactivate the review, and a producer closed by pruning remains uncheckable.
  Metadata failures produce `uncheckable` and never block retirement or launch.
- Exercise inherited session context with matching and mismatched worktree
  identities, matching and mismatched routed-leaf handles, a source that is and
  is not the current pick, a launch-window `leaf-insert`, a nested driver that
  replaces the outer target, fresh-grove start with no routed leaf, malformed or
  handle-free structured peek output that stops before launch, and the meta-grove
  test scrub list. Assert one structured peek supplies readiness, launch-line,
  routing, receipt identity, source/generation validation, and review evidence
  without a second tree read. Reopen a producer after the peek and prove the
  session's factual pick wins rather than running the stale review.
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

- `CONTEXT.md`: Review chain, ownership discriminator, tree-access lock,
  promotion transaction, direct/decomposed launch receipt, producer generation,
  and the `DONE` side effect.
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
- `grove-llm --help`, `grove-llm kind --help`,
  `grove-llm leaf-promote-chain --help`, `docs/USAGE.md`, and
  `docs/CONFIGURATION.md`: the structured routing-peek contract, operation,
  recovery diagnostic, advisory warning, and routing knobs that resolve it.
  `GROVE_SESSION_TARGET` is reserved internal context, not a user configuration
  knob.

## Compatibility

Old binaries and hand-edited task files ignore the new freeform relationship and
receipt fields. New readers derive `session = producer` and the producer's own
key as `generation` for a legacy direct-leaf receipt; the same missing fields on
a node producer are uncheckable because a node never launched and its generation
cannot be assumed. Old or manually composed review leaves without relationships
still launch and receive an `uncheckable` warning; no tree migration is required
for reading or launching them. A producer inside any non-root brief-less
composition node is not promotable even without metadata; the grove root is
never classified by brief absence. A metadata-bearing flat chain is detected
through the producer's whole sibling level. A metadata-free flat legacy chain
cannot be associated without forbidden suffix/position inference; annotate its
review with `**Reviews:** <producer-handle>` before invoking promotion.
Composite shapes remain reproducible with ordinary Markdown and filesystem
operations, but the previous claim that `leaf-add-chain` is byte-identical to
three bare `leaf-add` calls narrows: the manual form must also write the stable
relationship lines to reproduce the current generated shape.

Decomposing a `review-*` leaf moves its relationship and receipt into the new
node brief under the existing lifecycle contract; the generated first child has
no relationship and its launch comparison is therefore `uncheckable` unless the
operator explicitly reattaches stable metadata. Grove does not infer the parent
review relationship from the child's position.

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
- Proving that a review differs from every session that contributed to a
  decomposed producer; the receipt compares against the node-closing handoff
  session.
- Automatically reopening or duplicating a terminal review when later work
  reopens its producer; receipts carry evidence, not scheduling authority.
- Inferring model aliases or observing an interactive model switch inside a
  launched harness.
- Replacing research-pair breadth and combine discipline with doubt reviewers.
- Changing doubt-driven development in sessions that are not executing a picked
  Grove leaf.
- Providing power-loss durability. The portable contract is process-interruption
  consistency through serialization, a fail-closed transaction, and an atomic
  final directory rename; it performs no ordered `fsync` protocol.

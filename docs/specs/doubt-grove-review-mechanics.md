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

The [Review target receipts](../adr/review-target-receipts.md) decision owns the
handoff identity, generation, freshness, terminal-review, and advisory-failure
policy. At the lifecycle interface, retirement prepares facts for the direct
leaf and each reviewed decomposition ancestor this transition would close while
the leaf is still the factual pick and the exclusive tree guard is held. It
applies `DONE` first, then materialises a prepared receipt only into a live
linked review. Materialisation re-reads the review task, replaces only the
receipt line, and atomically renames the result; it never rolls back or masks the
terminal outcome. A close cascade may identify several reviewed ancestors, but
at most one linked review can be live because a live inner review is a live
descendant of every outer producer.

Source selection, confirmation, restart, reopen, and pruning consequences are
defined solely by the cited ADR. The lifecycle choreography above consumes those
decisions; it adds no independent kind or scheduling rule.

## Advisory target diversity

The [Review target receipts](../adr/review-target-receipts.md) decision owns the
historical target, equality, warning, and advisory-failure policy. This section
specifies only its transport, wire, and rendering interfaces.

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

For a review, the evidence object carries either validated producer evidence or
an `uncheckable` reason and the producer handle only when it came from a valid
`Reviews` relationship. The historical effective target is nested under
`producer-target`, distinguishing it from the leaf's top-level declared
`harness`. The object does not carry the review's effective target, which the
driver resolves after the peek. The driver retains this evidence beside the
routed leaf and performs the pure target comparison later; it never re-opens the
task tree outside the peek's guard.

```json
{"review":{"status":"checkable","producer":"sync-design-k12","session":"sync-docs-k27","generation":"k31","producer-target":{"harness":"claude","model":"opus"}}}
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

The session-context validator applies the ADR's worktree, routed-handle, and
factual-pick gates while the facts remain observable. The routed handle is launch
evidence, never authority over the task tree.

Producer retirement materialises validated context in the linked review leaf
beside the stable `**Reviews:**` relation. A direct leaf producer records itself
as both reviewed producer and source session:

```markdown
**Producer launch:** {"producer":"sync-design-k12","session":"sync-design-k12","generation":"k12","harness":"claude","model":"opus"}
```

A decomposed producer records the node as the reviewed entity and the factual
leaf whose retirement closed it as the source session:

```markdown
**Producer launch:** {"producer":"sync-design-k12","session":"sync-docs-k27","generation":"k31","harness":"claude","model":"opus"}
```

The decomposed example deliberately separates the source session (`k27`) from
the producer generation (`k31`): an early-position child inserted after later
children existed can own the maximum key while a different last-position leaf
performs the close. New writers emit five known fields: `producer` and `session`
are valid stable-handle strings, `generation` is a positive `k<key>` string,
`harness` is a known harness-name string, and `model` is either a non-empty exact
selector string or JSON `null`. All five are required on a newly written
receipt. Readers require `producer`, `harness`, and `model`; they accept
`session` and `generation` as an all-or-nothing legacy omission under the ADR's
compatibility rule, and ignore unknown keys. Wrong JSON types, empty or invalid
known values, and a one-field legacy omission are malformed. Under this rule,
legacy node receipts are uncheckable; only a direct leaf can derive both omitted
facts.

At `review-*` launch, the driver resolves the review target and passes it with
the retained evidence to the ADR-defined pure comparison. The launch layer
renders that result without re-reading or reinterpreting receipt policy.

The renderer emits one compact block to stderr and prepends the same block to the
session prompt. It names the review, a relationship-backed producer or
`producer=unknown`, the validated source `session` for a checkable result when it
differs from the producer, both available targets, matching or unavailable axes,
and routing configuration. It does not present a session from uncheckable
evidence as factual. A null model renders as `default(<harness>)`.

The notice is explicitly addressed to its routed review handle and says it
applies only if the session's own `grove-llm pick` returns that handle. The
guarded evidence remains a forecast: an insert or reopen in the launch window
can hand the session another leaf, in which case it discards the prepended notice
and follows factual pick. A one-harness installation still warns on every
review, and a fully diverse comparison stays silent, as required. Prompt tests
can prove the notice is scoped and the session is instructed to discard it; they
cannot prove absence of model influence. The cited ADR owns that visible
launch-window trade-off.

## Module interfaces

The task-tree access module exposes shared-read and exclusive-mutation guards
over one open Grove-root directory descriptor. Exported operations acquire
exactly once and pass the guard through internal lock-neutral helpers, so a
mutator can call pick/resolve logic without recursively locking or opening a
race. Pending-promotion detection runs only while a guard is held. The routing
peek returns path, handle, kind, declared harness, and validated review evidence
from that same guarded read; launch callers never reconstruct leaf or receipt
identity with another tree read. `grove-llm kind --with-harness --json` remains
that single launcher-peek entry point deliberately: renaming it would add a
second public concept for the same guarded read. Its help text describes the full
payload, and its nested historical target is named `producer-target` so it cannot
be confused with the top-level declared `harness`.

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
metadata failure as an advisory diagnostic. A terminal linked review yields a
`review-terminal` skip diagnostic rather than a materialisation plan.

Routing retains one resolver whose result contains an effective launch target.
A session-context parser validates worktree identity, routed-leaf identity, and
current-pick identity before yielding a handoff target. A pure diversity
comparison accepts the retained checkable/uncheckable review evidence and the
resolved review target; it returns `diverse`, the matching axes, or an
`uncheckable` reason with an optional producer handle. The launch layer renders
the same result to stderr and the prompt, including a checkable source-session
handle when it differs from the producer. Callers and tests do not reach through
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
  a pre-existing receipt in a live review that successful retirement
  unconditionally replaces, direct and decomposed producers whose receipts name
  distinct producer/session handles, and producer-, review-, and
  integration-kind descendants serving as the closing session. Close multiple
  reviewed decomposition ancestors and prove at most one linked review is live;
  terminal reviews remain byte-identical and produce a `review-terminal` skip
  diagnostic. Cover generation stability under reorder versus change after
  supported reopen, and
  insert a highest-key child ahead of existing children so the receipt's
  generation differs from its later closing session. Include a receipt with
  unknown keys, legacy receipts lacking `session`/`generation` for both direct
  and node producers, a one-field legacy omission, every wrong JSON type, empty
  or invalid handle/key/target values, and every missing required core field,
  a receipt whose producer disagrees with the `Reviews` relationship, and a
  successful `DONE` rename followed by a failed receipt write. The normal
  receipt-free failure case must leave no authoritative prior-session target;
  the hand-edited direct-leaf failure must diagnose that the remaining value may
  be stale, while a stale decomposed-node generation must be rejected
  mechanically. Edit the review task after receipt preparation but before
  materialisation and prove the post-`DONE` re-read preserves that edit. Also
  prove reopening a producer whose linked review is already terminal does not
  reactivate or rewrite the review. A producer closed by pruning remains
  uncheckable and leaves its sibling review next in pick order; pruning the
  enclosing chain marks every live step. Metadata failures produce
  `uncheckable` and never block retirement or launch.
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
  A decomposed-producer warning names its distinct source session. Mutate the
  tree in the launch window and prove the notice tells a session whose factual
  pick differs to discard it. Assert the structured evidence nests the historical
  target under `producer-target` rather than overloading `harness`/`model`.
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
  promotion transaction, direct/decomposed launch receipt, source session,
  producer generation, the advisory node-close write, and the `DONE` side
  effect.
- `content/SKILL.md`, `content/driving.md`, and `content/TASK-FORMAT.md`: the
  leaf-wide reviewer allowance, exact promotion/handoff sequence, integration
  placement, chain-level pruning guidance, research exclusions, the
  confirmation-boundary wording for reviewed node closes, and replacement of
  the old three-round and manual-retrofit guidance.
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
  nested `producer-target` evidence, source-session warning, launch-window notice
  scope, recovery diagnostic, and routing knobs that resolve it.
  `GROVE_SESSION_TARGET` is reserved internal context, not a user configuration
  knob.

## Compatibility

Receipt-field backward/forward behavior is owned by the
[Review target receipts](../adr/review-target-receipts.md) decision; the wire
shape above defines accepted types. The observable compatibility consequence is
that binaries predating the extensible-reader rule can call a newer receipt
malformed, while current readers require no task-tree migration and neither case
blocks review launch. Old or manually composed review leaves without
relationships still launch and receive an `uncheckable` warning. A producer
inside any non-root brief-less
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
- Changing the handoff-coverage or review-scheduling policy owned by the
  [Review target receipts](../adr/review-target-receipts.md) decision.
- Inferring model aliases or observing an interactive model switch inside a
  launched harness.
- Replacing research-pair breadth and combine discipline with doubt reviewers.
- Changing doubt-driven development in sessions that are not executing a picked
  Grove leaf.
- Providing power-loss durability. The portable contract is process-interruption
  consistency through serialization, a fail-closed transaction, and an atomic
  final directory rename; it performs no ordered `fsync` protocol.
- Mechanically proving that a launch-window routing notice did not influence a
  session whose factual pick changed; Grove scopes the notice but does not bind
  the session to the routing forecast.

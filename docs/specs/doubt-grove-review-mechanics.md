# doubt-grove-review-mechanics

## Problem

Doubt-driven development and Grove both materialise fresh-context adversarial
review. Used independently they are bounded, but used together without an owner
they can duplicate a scheduled review, spawn another reviewer after every fix,
or let an in-session child bypass the session-kind target configured for Grove's
review work.

The composition must preserve the cheap move for a small, unexpected doubt
while turning substantial review into ordinary task-tree work. It must also
preserve stable handles, predictable pick order, process-interruption recovery,
Git/Jujutsu symmetry, and Grove's guide-not-gate posture.

## Solution

The [Grove owns escalated review](../adr/grove-owns-escalated-review.md)
decision assigns the orchestration boundary. A session activates Grove
ownership only when its prompt visibly mandates a stable work-item handle and
its Bootstrap resolves and adopts that live leaf. Merely finding `.grove/` or
inheriting Grove control environment does not activate the rule.

A mandated plain producer may spend one in-session fresh-context reviewer
across the whole leaf. One reviewer means one independently materialised
context: a diverse-lens pass with N subagents spends N reviewers, while one
reviewer asked to inspect several named axes still spends one. If the work needs
a second reviewer, or a substantive non-mechanical fix needs another review,
the producer becomes a Grove-managed review chain. The producer finishes only
to a coherent reviewable boundary; the scheduled `review-*` session performs
the next adversarial pass.

Trivial findings, noise, a visible accepted trade-off, or a fix conclusively
covered through an executable test seam do not create a second review need.

### Behavior by session kind

| Session | In-session doubt | If more review is needed |
|---|---|---|
| Mandated plain `requirements`, `design`, `planning`, `prototype`, or `impl` producer | At most one fresh-context reviewer for the entire leaf. | Promote the producer into a review chain. |
| Producer already in a review chain | None; its `review-*` leaf is already scheduled. | Finish to the scheduled review boundary. |
| `review-*` | None; this session is the adversarial read and produces findings, not fixes. | Record findings for integration. |
| `integrate-review-*` | At most one narrow reviewer. | Externalise substantial redesign as a new producer review chain inside the owning chain node. |
| `research-a`, `research-b`, or `combine-research` | None; the pair supplies independent corpora and the combiner supplies the adversarial move. | Put a load-bearing derived decision in its own reviewed producer chain. |
| Any session without a resolved prompt mandate | Standalone doubt behavior is unchanged. | Grove has no ownership merely because the checkout contains a tree. |

Review diversity is personal configuration policy. Grove schedules a fresh
session of the appropriate `review-*` kind but does not record producer targets,
compare harnesses or models, inject review warnings, or add a competing doubt
reviewer.

## Atomic promotion

The agent-facing interface is one deep operation:

```text
grove-llm leaf-promote-chain <mandated-producer> [--json]
```

`<mandated-producer>` accepts the absolute path obtained by resolving the
prompt mandate or a stable key/handle that resolves to it. A path is reduced to
the producer handle before its current location is resolved, so the stale path
remains an idempotent retry after promotion relocates or retires the producer.
The exact `PROMOTING-*` path named by a fail-closed diagnostic is also accepted
while that transaction exists.

The operation takes no parent, stem, kind, harness, or target flags. Parent,
stem, and kind are facts in the current leaf filename; review launch policy is
outside the task tree. Given a picked `05-design-sync-design-k12.md` and a tree
whose maximum key is `k20`, promotion produces:

```text
05-sync-design-chain-k21/
  01-design-sync-design-k12.md
  02-review-design-sync-design-review-k22.md
  03-integrate-review-design-sync-design-integrate-k23.md
```

The containing node reuses sibling position `05`; every former sibling keeps
its position. The producer moves byte-for-byte, retaining `sync-design-k12`, its
filename kind, and every task detail. The brief-less node, review, and
integration receive fresh tree-wide keys. The generated review and integration
tasks carry stable relationships:

```markdown
**Reviews:** sync-design-k12
**Integrates:** sync-design-review-k22
```

Their default goals name the linked task and the corresponding review or
integration discipline. They carry no `**Kind:**`, `**Harness:**`, or producer
target metadata. `leaf-add-chain` emits the same relationship shape for a chain
created proactively.

Plain stdout contains four absolute paths in node, producer, review,
integration order and appears only after success. `--json` returns those paths,
all four handles, and a `changed` boolean as one object. JSON failures use a
stable error code, nonzero exit, and no partial stdout. The second plain path is
therefore the value a producer may pass to `leaf-retire`.

Promotion accepts the named live producer and only one of the five producer
kinds. In a Grove session the caller's prompt-visible mandate authorizes that
name; the command cannot observe prompt prose and therefore does not recompute
pick as a proxy. This matters when a launch-window insert legitimately makes
the mandated producer differ from the next walk result. The kind comes strictly
from the current filename. Promotion refuses terminal leaves,
unknown filename kinds, `research-a`, `research-b`, `combine-research`,
`finish`, `review-*`, and `integrate-review-*`.

The prompt mandate is workflow discipline rather than an unforgeable
capability. The command trusts the explicit producer reference after structural
validation; driver exclusivity and stale-session behavior are a separate process-
ownership concern, not grounds for reconstructing hidden target metadata here.

An already-scheduled review is detected through a sibling task declaring
`**Reviews:** <producer-handle>` or through a non-root immediate parent with no
`BRIEF.md`, the structural signal for a composition-managed node. Multiple
claimants are malformed. The root is excluded from the brief-absence test
because its brief is optional. Grove never infers a relation from suffix or
position. A refused new promotion changes nothing and consumes no key.

A retry by stable producer handle or stale path is idempotent when the completed
shape already exists: it returns the same four current paths with
`changed: false`, even when the producer is terminal or no longer first after a
later insert. Completed-shape recognition precedes the new-promotion liveness
and kind gates. The same identity resumes an exact pending transaction; the
explicit witness path is the recovery form when only a generic tree diagnostic
is available.

## Fail-closed transaction

The [Task-tree transactions fail closed](../adr/task-tree-transactions-fail-closed.md)
decision supplies the portable atomicity boundary.

Every participating task-tree operation acquires the shared tree-access seam
before checking transaction state or reading names. The seam locks an open
descriptor for the working-tree root: readers hold a shared process-scoped
advisory lock and mutators an exclusive lock. Exported operations acquire
exactly once and pass the guard into lock-neutral helpers. A contended caller
prints one waiting diagnostic, then waits without timeout. Process exit releases
the lock; no PID, owner record, or lock file is stored. The working-tree root
exists before `.grove/`, so root initialization, finish deletion, and ordinary
tree operations now share the same seam. Descriptors are close-on-exec, and the
driver releases its read guard after copying a selected value and before
foreground launch so the mandated session can mutate the tree. The lock supplies
live-process serialization only; promotion's witness and landing protocol supply
its process-interruption guarantee.

Promotion holds the exclusive lock and handles pending recovery before ordinary
source resolution, liveness, or kind validation. With no pending transaction it
pre-validates the producer, derived kinds, destination,
relationships, content, and three-key allocation. It builds the replacement
under a reserved sibling directory:

```text
PROMOTING-<final-node-name>/
```

The generated review and integration are written first. The producer then moves
byte-for-byte into child position `01` through the VCS-aware rename seam. The
reserved name remains a durable witness if the process disappears.

The source move follows the jj-first repository rule. Native and colocated jj
use a filesystem rename and never mutate Git's index. Plain Git uses its
tracked-file adapter; an untracked producer remains untracked.

For a tracked plain-Git producer, Grove prepares the already-staged producer
index entry to name its final child path while the `PROMOTING-*` witness still
blocks readers. That index transaction preserves the stage-0 blob, mode, and
flags; an unmerged entry is rejected before mutation. Generated tasks remain
untracked. Only after the index names the final path does Grove land the entire
directory with one same-parent filesystem rename. Jujutsu and an untracked Git
producer use the same final rename without index preparation.

Every tree reader and mutator recursively refuses while any `PROMOTING-*`
witness exists and names:

```text
grove-llm leaf-promote-chain <exact-witness-path>
```

Generic readers do not inspect generated tasks or infer the producer from child
position to improve that diagnostic. Recovery under the exclusive lock may
scan the reserved transaction, match its stable producer identity, and
normalize source, staging, or final Git-index spellings. It reuses the reserved
key run and never nests another chain.

Every reported failure before landing attempts to reverse the producer move,
normalize any prepared Git index entry, and remove the transaction. If rollback
fails, the witness remains and the error names exact recovery paths. Plain and
JSON stdout remains empty until final landing succeeds.

The observable process-interruption invariant is:

- before the producer move, the original producer is intact and the witness
  blocks every other operation;
- after the move, producer and generated leaves are together inside that same
  blocked transaction; or
- after landing, the only parsed shape is the complete chain and its first live
  child is the unchanged producer.

The guarantee does not cover power loss, kernel failure, storage-cache loss, or
a filesystem violating rename semantics. Grove performs no ordered `fsync`
protocol; the final rename is an atomic namespace-visibility seam, not a
power-loss commit.

## Producer handoff

After promotion, the producer session:

1. applies only the bounded change needed to restore a coherent reviewable
   artifact and runs executable checks without another doubt reviewer;
2. commits the artifact and promotion together under the unchanged producer
   handle;
3. retires the relocated producer path; and
4. runs `grove-llm complete` last.

Retirement applies only the filename `DONE` transition. It does not write a
target receipt or alter the linked review task. The next driver iteration picks
the review leaf and selects its complete command from personal configuration.

## Module interfaces

The task-tree access module exposes shared-read and exclusive-mutation guards
over one open working-tree-root descriptor. Pending-promotion detection runs
only while a guard is held. This is an internal seam; ordinary tree commands
remain the external interface.

The promotion module exposes one operation returning node, relocated producer,
review, and integration identities. It hides filename-kind validation, sibling
relationship checks, stale-path normalization, key allocation, templates,
transaction recovery, Git index preparation, VCS-specific movement, and
rollback. All task-tree operations share one pending-promotion guard rather than
reproducing reserved-prefix scans.

Task relationship parsing owns only the durable `Reviews` and `Integrates`
markers and cardinality-checked sibling lookup. It has no launch-target,
receipt, or diversity-comparison interface. The loop driver routes a scheduled
review solely by its filename kind and complete personal configuration.

## Test seams

- Exercise promotion through `grove-llm` in temporary trees: root and nested
  producers, all five producer kinds, strict unknown-kind refusal, source
  bytes/handle/key preservation, unchanged sibling positions, fresh keys, no
  generated `BRIEF.md`, exact filename kinds and relationships, and pick order
  before and after retirement. Cover a live mandated producer displaced by a
  launch-window insert, relationship-bearing legacy shapes, non-root brief-less
  parents, and decomposition-node parents.
- Drive subprocesses through barriers around pre-validation and allocation:
  promotion versus promotion and promotion versus every existing mutator.
  Assert one wait diagnostic, unique positions/keys, no observable missing
  producer, lock release on process exit, stale-path idempotence, and witness
  recovery before ordinary producer gates.
- Inject failure and interruption at every create, write, source move, Git index
  preparation, and final rename. Reported failures either restore the exact
  source or leave one recoverable witness; interruptions make every reader and
  mutator fail closed. Cover tracked and untracked Git, native jj, colocated jj,
  stage-0 metadata preservation, and pre-mutation refusal of unmerged entries.
- Prove generated chains contain no task-body kind, harness, or producer-target
  markers; retirement changes only the producer filename; review selection
  comes from the filename kind; and no routing warning or target environment is
  emitted.
- Assert the prompt-mandate ownership rule across a normal Grove launch, a
  checkout-only session, nested Grove, missing/terminal mandate resolution,
  producers already in chains, all review/integration kinds, and the research
  pair.
- Sweep the embedded Grove methodology and doubt skill for the positive rules
  above and for absence of the former session-side re-pick predicate,
  multi-review loop, manual retrofit guidance, receipt handoff, and diversity
  warning contract.

## Compatibility

Current-format filenames are required before promotion. Legacy body kinds are
handled by the automatic session-kind migration in
[config-driven-sessions](config-driven-sessions.md), not by promotion. There is
no dual-format or `impl`-degrading promotion reader.

Relationship-bearing manually composed chains remain recognized. A producer in
any non-root brief-less composition node remains non-promotable even without
metadata; the root is never classified by brief absence. A metadata-free flat
legacy chain cannot be associated without forbidden position/suffix inference
and must be migrated or annotated before promotion.

`PROMOTING-*` remains reserved. A current binary recovers such a directory
before any tree work; all other foreign files retain the normal lenient rule.
Pick order, brief-chain behavior, terminal infixes, stable relationships, node
brief discrimination are unchanged.

## Out of scope

- Requiring every producer to have a review or treating a chain as a scheduling
  unit.
- Enforcing target diversity or observing an interactive model switch.
- Replacing research-pair breadth and combine discipline with doubt reviewers.
- Changing standalone doubt behavior outside a resolved Grove mandate.
- Changing pruning authority, completion signaling, or tree order.
- Power-loss durability.

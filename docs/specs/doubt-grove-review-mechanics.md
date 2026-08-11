# doubt-grove-review-mechanics

## Problem

Doubt-driven development and Grove both materialise fresh-context adversarial
review. Used independently they are bounded, but used together without an owner
they can duplicate a scheduled review, spawn another reviewer after every fix,
or let an in-session child bypass the session-kind target configured for Grove's
review work.

The composition must preserve the cheap move for a small, unexpected doubt
while turning substantial review into ordinary task-tree work. It must also
preserve stable handles, predictable pick order, Git/Jujutsu symmetry, and
Grove's guide-not-gate posture.

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
the producer cuts a Grove `review-*` leaf. The producer finishes only
to a coherent reviewable boundary; the scheduled `review-*` session performs
the next adversarial pass.

Trivial findings, noise, a visible accepted trade-off, or a fix conclusively
covered through an executable test seam do not create a second review need.

### Behavior by session kind

| Session | In-session doubt | If more review is needed |
|---|---|---|
| Mandated plain `requirements`, `design`, `planning`, `prototype`, or `impl` producer | At most one fresh-context reviewer for the entire leaf. | `leaf-add` a `review-<producer>` leaf beside it. |
| Producer whose `review-*` leaf already exists | None; its review is already scheduled. | Finish to the scheduled review boundary. |
| `review-*` | None; this session is the adversarial read and produces findings, not fixes. | Record findings for integration. |
| `integrate-review-*` | At most one narrow reviewer. | Externalise substantial redesign as a new producer review chain beside the leaf being integrated. |
| `research-a`, `research-b`, or `combine-research` | None; the pair supplies independent corpora and the combiner supplies the adversarial move. | Put a load-bearing derived decision in its own reviewed producer chain. |
| Any session without a resolved prompt mandate | Standalone doubt behavior is unchanged. | Grove has no ownership merely because the checkout contains a tree. |

Review diversity is personal configuration policy. Grove schedules a fresh
session of the appropriate `review-*` kind but does not record producer targets,
compare harnesses or models, inject review warnings, or add a competing doubt
reviewer.

## Escalation is one `leaf-add`

The agent-facing interface is the grow verb a session already calls:

```text
grove-llm leaf-add <parent> <stem>-review --kind review-<producer>
```

`<producer>` is the kind in the mandated leaf's own filename, and the session
reads it there. Nothing derives it, so naming it off some *other* producer is a
well-formed mistake nothing downstream catches — `--kind review-impl` beside a
`design` producer buys a reviewer reading for correctness, security and tests
where it should be asking whether the ADRs are a minimum coherent set.

The new leaf is an ordinary **flat sibling** at the parent's next free position,
with a fresh tree-wide key. Nothing about the producer's own leaf changes: its
position, key, handle and bytes are untouched by construction, which is why
escalation needs no transaction and no recovery protocol. Given a mandated
`05-design-sync-design-k12.md` at the root of a tree whose maximum key is `k20`,
escalation produces:

```text
05-design-sync-design-k12.md          unchanged
06-review-design-sync-design-review-k21.md
```

and, if that review finds something worth acting on, its own last act produces:

```text
07-integrate-review-design-sync-design-integrate-k22.md
```

Here the review has no live sibling after it, so the next free position *is* the
slot beside it and `leaf-add` puts the integration where it belongs. When that is
not so — some live leaf already sits at 07 or beyond — the integration is cut
with `leaf-insert` at the first such sibling instead, for the reason in *What the
flat shape gives up, deliberately* below.

**The creating session writes the new leaf's body**, and that is the reason to
create it late rather than up front. A review leaf can name the exact claim the
producer could not establish; an integrate leaf can carry the findings verbatim.
A constructor knows only a stem and a kind, so it could only ever render a
generic goal sentence — strictly less than the session that just discovered the
doubt can supply.

The body also carries the composition relationship, written by hand:

```markdown
**Reviews:** sync-design-k12
```

and, for an integration:

```markdown
**Integrates:** sync-design-review-k21
```

**Nothing writes those lines and nothing parses them.** They are a documented
convention (`content/TASK-FORMAT.md`) for the human reading `find .grove` and for
the session that picks the step up, which is Grove constraint 3: task files are
freeform markdown and nothing validates them. A leaf carries no `**Kind:**`, `**Harness:**`, or producer target metadata.

A review that finds nothing worth acting on **creates nothing** and simply
retires. That is the empty triage session this shape exists to remove, and the
same holds one step earlier: a producer that judges review unnecessary cuts no
review leaf.

### What the flat shape gives up, deliberately

A chain's steps are appended at the parent's next free position, so a step cut
after unrelated work already exists lands after that work, and a sibling
`leaf-insert` can split a chain that was contiguous. Grove validates no
cross-leaf grammar and `pick` is a walk rather than a scheduler, so contiguity
was always a convention rather than an enforced unit, and it stays one: nothing
here is enforced.

**The two hops are not equally exposed, and the difference decides the verb.**
It is a property of what the next step consumes, not of how far apart the leaves
sit:

- A **`review-*`** step *re-derives*. It reads the producer's commit — an
  immutable object history holds whatever lands afterwards — and computes its own
  `path:line` citations against the tree as it then stands. Intervening work
  changes what the review reads and invalidates nothing, because nothing had been
  written down. Plain `leaf-add` is correct wherever it lands it.
- An **`integrate-review-*`** step *consumes*. Its input is the set of citations
  the review already froze into prose, resolved against a working tree that has
  since moved. An intervening edit to a cited file shifts those lines and the
  drift is **silent** — nothing errors, the finding points somewhere slightly
  wrong, and the integrating session must re-derive the reviewer's intent from a
  codebase the reviewer never saw.

So an integration is cut **immediately after the review it integrates, by
default**, which makes `leaf-insert` the verb whenever the review already has a
live sibling after it. `leaf-add` appends at the *end* of the parent, so it is
correct only when the review has none; terminal siblings in between are
irrelevant, because `pick` skips them and never stops between the two steps:

```text
grove-llm leaf-insert <the review's first live sibling> <stem>-integrate --kind integrate-review-<producer>
```

The exception is a check, not a judgement: depart from adjacency only when the
intervening work **provably touches no file the findings cite**, established by
listing the paths the findings name and checking the intervening leaf against
them. Where that check cannot be performed — the intervening leaf's file set is
not yet knowable — the exception does not apply. This is guidance for the session
cutting the leaf and nothing more; making it a *mechanism* would be a separate
decision, with its own ADR.

Neither composition shape gets a node directory. A node means *this work proved
bigger than one session* and carries the `BRIEF.md` those extra sessions need;
a composed shape has no such context, and the hierarchy a node bought did not
repay its navigation cost. One consequence is worth stating: there is a single
node species again, so the Retire cascade's close has the same work at every node
— a `Done when` rollup to check and a brief to promote — and no `BRIEF.md`-presence
discriminator survives.

## Task-tree access

The [Task-tree transactions fail closed](../adr/task-tree-transactions-fail-closed.md)
decision supplies the serialization boundary.

Every participating task-tree operation acquires the shared tree-access seam
before reading names. The seam locks an open descriptor for the working-tree
root: readers hold a shared process-scoped advisory lock and mutators an
exclusive lock. Exported operations acquire exactly once and pass the guard into
lock-neutral helpers. A contended caller prints one waiting diagnostic, then
waits without timeout. Process exit releases the lock; no PID, owner record, or
lock file is stored. The working-tree root exists before `.grove/`, so root
initialization, finish deletion, and ordinary tree operations share the same
seam. Descriptors are close-on-exec, and the driver releases its read guard after
copying a selected value and before foreground launch so the mandated session can
mutate the tree.

The lock supplies live-process serialization only, and escalation needs nothing
more. `leaf-add` takes its destination with an atomic non-clobbering create, so
it can neither truncate nor write through whatever a writer that ignored the
lock may have planted there; on a reported error it leaves nothing behind.
Process death is a different question and is deliberately **not** covered: a
killed `leaf-add` can leave a created-but-empty leaf, and a killed
`leaf-add-pair` a partial shape. Finish teardown and the one-time session-kind
migration remain the only operations that promise process-interruption recovery,
each with its own in-tree witness; the residue here is a file to delete, not a
transaction to recover.

## Producer handoff

After cutting the review leaf, the producer session:

1. applies only the bounded change needed to restore a coherent reviewable
   artifact and runs executable checks without another doubt reviewer;
2. retires the producer;
3. commits the artifact, the new review leaf, and that retirement together under
   the producer's handle; and
4. runs `grove-llm complete` last.

**Retirement precedes the commit**, which is the task boundary Grove's own
methodology states. The `DONE` rename is part of the task, so a commit taken
before it either leaves the rename uncommitted (Git) or seals it into the *next*
task's working-copy commit (jj), where separating it again costs an operation-log
rewind.

Retirement applies only the filename `DONE` transition. It does not write a
target receipt or alter the review leaf, which reads the committed artifact
rather than anything about the session that produced it. The next driver
iteration picks the review leaf and selects its complete command from personal
configuration.

## Test seams

- Exercise the lazy chain through `grove-llm` in temporary trees: a producer,
  its review, and its integration cut as three separate `leaf-add` calls, landing
  as contiguous flat siblings with consecutive fresh keys, no node directory, and
  an untouched producer. Cover a review step cut after unrelated work, which
  lands after that work rather than beside its producer.
- Sweep every guidance surface for the **per-hop** placement rule, since no verb
  can carry it: the `review → integrate` hop names `leaf-insert`, states the
  commit-and-line anchoring that makes adjacency the default, and gives the
  exception its check; the `producer → review` hop is stated as the contrasting
  case that needs no rule. Pin the verb *to the hop*, so a surface that merely
  offers `leaf-insert` as a general remedy for ordering — the superseded
  formulation, which supplied a judgement call and no test for making it — fails
  rather than passing on the word alone.
- Assert the freshly created leaf's body is the bare template — the stable handle
  and empty sections — carrying no rendered goal, no relationship line, and no
  launch metadata, so the creating session has nothing to edit around.
- Assert both deleted constructors (`leaf-add-chain`, `leaf-promote-chain`) are
  refused by the command surface, with an untouched tree, and that neither is
  advertised in `--help`.
- Exercise `leaf-add-pair` as the one surviving composite: three flat siblings
  with the fixed research kinds, one snapshot for positions and keys, every
  destination swept before the first write, and a mid-write failure that unwinds
  every leaf it created. A generated pair is byte-identical to the same three
  leaves cut by hand.
- Assert retirement and pruning are filename-only over the whole `.grove/`
  subtree, by snapshot comparison with a positive control, so a producer's
  retirement provably writes nothing into the review beside it.
- Assert the prompt-mandate ownership rule across a normal Grove launch, a
  checkout-only session, nested Grove, and missing/terminal mandate resolution.
- Sweep the embedded Grove methodology and doubt skill for the positive rules
  above and for absence of the former session-side re-pick predicate,
  multi-review loop, eager chain constructors, receipt handoff, and diversity
  warning contract.

## Compatibility

The change is **forward-only, with no migration.** A chain node was only ever an
ordinary node *directory* whose slug happened to end in `-chain`, and every
reader handles node directories generically — the token was slug text nothing
keyed on. Existing trees containing one therefore keep working untouched: the
node still parses, `pick` still descends it in pre-order, and its children still
resolve by handle. Its close now goes through the ordinary path, which looks for
a `Done when` it will not find and reports nothing to promote.

Current-format filenames are required, as everywhere else. Legacy body kinds are
handled by the automatic session-kind migration in
[config-driven-sessions](config-driven-sessions.md).

`PROMOTING-*` is no longer reserved. A directory left by an interrupted
promotion under an older binary is now an ordinary foreign entry: unpositioned
and unkeyed, so every reader skips it — correctly, but also silently, and what
it holds depends on **which phase the old transaction died in**. It created the
witness and generated the chain's steps *before* moving the producer, and could
stage a tracked index entry for the final child before landing anything, so a
stranded witness is one of three shapes:

- the producer is still outside it, and the witness holds only generated steps —
  nothing to move back, and the work is already visible to `pick`;
- the producer is inside it, and must be moved back to its original position and
  name before that work is visible again;
- the producer is inside it *and* Git's index already names a child that never
  landed, so restoring the file is not enough — the index entry has to be
  dropped too (`git rm --cached`), or the next commit records a path with no
  working-tree file.

**Recover before upgrading.** The old binary still has the recovery path that
knows which shape it left; running it is the supported route. After upgrading,
recovery is a hand repair, and it starts by looking inside the witness to
determine which of the three states it is.

## Out of scope

- Requiring every producer to have a review, or treating a chain as a scheduling
  or containment unit.
- Enforcing target diversity or observing an interactive model switch.
- Replacing research-pair breadth and combine discipline with doubt reviewers.
- Changing standalone doubt behavior outside a resolved Grove mandate.
- Changing pruning authority, completion signaling, or tree order.

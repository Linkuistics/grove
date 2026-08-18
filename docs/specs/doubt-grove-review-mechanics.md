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
decision assigns the orchestration boundary.

**The allowance itself is stated in the corpus, not here.**
`content/references/execute.md` is the canonical source for the review budget —
the ownership predicate, the leaf-wide one-reviewer allowance and what spends it,
the per-kind allowances, the four-step doubt pass, and the rule that an escalated
review's route belongs to configuration. A spec describes how an area works and
**cites** the rules in its area rather than restating them
(`docs/specs/corpus-rule-ownership.md`), and this one predates that map: it
carried the budget, the per-kind table and the diversity rule at full length,
which is one source of truth too many for a rule that changes.

What this spec still owns is **why the ownership predicate is drawn where it
is**, and what the composition would otherwise cost.

### Why the predicate is a resolved mandate, and not a task tree

Doubt-driven development and Grove both materialise fresh-context adversarial
review, so the composition needs an owner rather than a preference. The predicate
is that the prompt visibly mandates a stable work-item handle **and** Bootstrap
resolved and adopted that live leaf — deliberately narrower than *there is a
`.grove/` here*, in both directions:

- **A checkout is not a mandate.** A session that merely finds a task tree, or
  inherits Grove control environment, was not scheduled by the loop and has no
  `review-*` leaf available to escalate into. Binding it to Grove's budget would
  cap a standalone session's doubt while offering it nothing in exchange.
- **The mandate is what makes escalation possible.** Only a session that resolved
  its own leaf can name the parent to cut a sibling under, so the predicate and
  the escape hatch are the same fact.

Both directions are observable at the seam Grove already has, which is why the
rule is testable rather than aspirational: a normal launch, a checkout-only
session, a nested Grove, and a missing or terminal mandate are four distinguishable
states of the same predicate.

## Escalation is one `leaf-add`

The agent-facing interface is the grow verb a session already calls:

```text
grove-llm leaf-add <parent> <stem> --kind review-<producer>
```

`<producer>` is the kind in the mandated leaf's own filename, and the session
reads it there. The conduct rules governing that call — naming the step kind off
the producer that actually ran, giving every step the producer's bare stem, and
cutting each step as the previous session's last act — are
`content/references/decompose.md`'s, and are not restated here.

The new leaf is an ordinary **flat sibling** at the parent's next free position,
with a fresh tree-wide key. Nothing about the producer's own leaf changes: its
position, key, handle and bytes are untouched by construction, which is why
escalation needs no transaction and no recovery protocol. Given a mandated
`05-design-sync-design-k12.md` at the root of a tree whose maximum key is `k20`,
escalation produces:

```text
05-design-sync-design-k12.md          unchanged
06-review-design-sync-design-k21.md
```

and, if that review finds something worth acting on, its own last act produces:

```text
07-integrate-review-design-sync-design-k22.md
```

**All three carry the same slug**, which is the shared-stem rule applied rather
than an elision in the example. The steps stay distinct by key, and their
handles — `sync-design-k12`, `-k21`, `-k22` — stay unique because keys are unique
tree-wide. What it costs is that `resolve sync-design` is ambiguous and lists all
three with their kind-bearing paths — pick-style, so empty stdout, the diagnostic
on stderr and **exit zero**. Every reference this spec recommends names a handle,
a key or a path and is unaffected; the bare slug that `leaf-add` and `leaf-insert`
also accept for a target is the exception, and there the ambiguity is a refusal
naming the matching keys.

The example takes the simple case: nothing later in the directory still holds
live work, so the next free position *is* the slot beside the review. Which verb
places the integration when that is not so — and the condition selecting its
target — is `content/references/decompose.md`'s rule and is not restated here.
What this spec carries is the walk that rule is true of, below.

The **shape of that body** is what this spec pins, because it is what the verb
guarantees and a test can assert: the freshly created leaf is the bare template —
a stable handle and empty sections — carrying no rendered goal, no relationship
line and no launch metadata, so the creating session has nothing to edit around.
Why the creating session is the right author is
`content/references/decompose.md`'s.

The body also carries the composition relationship, written by hand:

```markdown
**Reviews:** sync-design-k12
```

and, for an integration:

```markdown
**Integrates:** sync-design-k21
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

**The two hops are not equally exposed, and the difference decides the verb** —
a `review-*` step re-derives from its producer's stable handle, an
`integrate-review-*` step consumes `path:line` citations an intervening edit
moves silently. The rule that follows from it — where each step is cut, the
blocking-sibling condition, and why there is no exception to check — is
`content/references/decompose.md`'s, and this spec cites it rather than carrying a
second full statement of it.

What belongs here is the **implementation** that condition is true of. The seam
is `select_unlocked` in `src/tree_read.rs`, which is a **composition**: the
recursive `collect_live_leaf_entries` walk gathers live entries, and
`select_unlocked` applies the `finish`-sentinel rule over the result. Three
properties of that composition are what make the loose form (*the first live leaf
after it*) wrong, and they do not all come from the same half:

- **The walk quantifies over entries, not leaves.** It reads a level in position
  order and recurses into a node *in place*, so a later sibling node with a live
  leaf anywhere beneath it runs before anything appended after that node.
- **Terminal entries never enter the collection at all** — a later `DONE` or
  `ABANDONED` leaf, and a node whose subtree is wholly terminal, are simply not
  live. The driver's `finish` sentinel is different in kind: the collector pushes
  it like any other live leaf without inspecting the kind, and it is
  `select_unlocked` that takes the first live **non-`finish`** entry and falls
  back to the sentinel only when no ordinary work is left. A test seam aimed at
  the collector alone cannot see that property.
- **The walk is directory-local.** Pre-order finishes the review's own directory,
  including a leaf just appended to its end, before it visits any later sibling
  of an *ancestor*.

A change to any of the three changes the guidance, which is why the sweep and the
behavioural pins under *Test seams* are stated against this selection seam rather
than against filename adjacency.

**The rejected alternative is what this spec records** — that there is no
exception, and why the check cannot be performed, are
`content/references/decompose.md`'s. An earlier form allowed departing from
adjacency when the intervening work **provably touches no file the findings
cite**, and it is kept here so a reader who proposes it again finds why it went:
it buys a licence that reads as a judgement call, which is the exact failure the
narrowing replaced. Making departure a *mechanism* rather than the unenforced
possibility it already is would be a separate decision, with its own ADR.

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

Steps 2 and 3 are in that order because the task boundary is, and both rules are
the corpus's: `content/references/retire.md` for retire-before-commit and for
retirement being a filename transition and nothing else,
`content/references/commit.md` for what the boundary contains. What this spec
adds is the consequence for *this* shape — the review leaf beside the producer is
byte-identical after the producer retires, so escalation needs no receipt, no
producer-target record and no recovery protocol. The next driver iteration picks
the review leaf and selects its complete command from personal configuration.

## Test seams

- Exercise the lazy chain through `grove-llm` in temporary trees: a producer,
  its review, and its integration cut as three separate `leaf-add` calls, landing
  as contiguous flat siblings with consecutive fresh keys, no node directory, and
  an untouched producer. Cover a review step cut after unrelated work, which
  lands after that work rather than beside its producer.
- Sweep the **owner** for the per-hop placement rule, since no verb can carry it:
  `content/references/decompose.md` names `leaf-insert` and the exact
  blocking-sibling condition for the `review → integrate` hop, and claims fresh
  re-derivation via the stable handle for the `producer → review` hop. Bind each
  property **to its hop in one assertion** — `` `review-*` step re-derives ``,
  `` `integrate-review-*` step consumes `` — rather than checking the two verbs
  independently against a whole document, which an inverted surface passes, and
  ban the inverted pair outright. **The sweep is over one surface because the rule
  has one owner**; the surfaces that merely *record* it — this spec, the glossary,
  the architecture — are checked for the binding and for citing the owner, never
  for a second full statement. For this spec that check is **negative and
  explicit**: assert the blocking-sibling condition is absent from it, so a
  citation quietly regrowing back into a procedure fails rather than passing by
  omission from the full-rule list. `CHANGELOG.md` is out of scope entirely: it records
  what changed and when, so a current-state claim has no well-defined scope in it.
- Pin the placement rule **behaviourally**, by asking `pick` what runs next
  rather than asserting filename adjacency: a later direct live leaf (insert
  before it); a later sibling **node** with a live descendant (insert before the
  *node*, and inserting at the descendant instead lands the leaf inside that
  node); terminal leaves and wholly terminal nodes between the two steps (append
  still runs next); and a review inside a node with live work in a later *outer*
  sibling node (append inside the review's own directory still runs next). Each
  shape's contrast case — what the other verb would have selected — is what makes
  the assertion about scheduling rather than about names.
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
handled by the automatic session-kind migration described in
[`ARCHITECTURE.md`](../ARCHITECTURE.md#legacy-migration).

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

# Abandoned work is pruned — marked `ABANDONED` in place, one mark, rejection recorded in the ADR set

Deciding *against* a path is a normal outcome of exploratory work, so grove names
it. A leaf whose work is abandoned is **pruned**: `grove-llm leaf-prune` marks it
in place with an `ABANDONED` infix (`NN-ABANDONED-<slug>-k<key>.md`), exactly as
`leaf-retire` marks a completed leaf `DONE`. `pick` skips both. Neither is ever
removed from the tree.

A grove's two terminal leaf states are therefore `DONE` (the work was done) and
`ABANDONED` (the path was closed) — and **nothing else**. The *reason* for an
abandonment is prose, and lives in the ADR set, not in the filename.

## Why it binds

**The tree is the key counter.** `next_key` is `max(key over all names) + 1`; there
is no counter file (constraint 1). So a name that leaves the tree lowers the max,
and the next `leaf-add` re-issues a key that is already spoken for — after which
every durable cross-reference to that key (commit messages, ADRs, briefs) resolves
to different work. Marking in place closes that by construction rather than by
adding a mechanism: the only alternatives are a counter file (constraint 1 forbids
it) or deriving the max from `git log` (constraint 2 forbids it).

**A tree that hides its dead ends lies.** The `DONE` mark already commits to this
principle — retirement is in place *so that the tree always shows complete state*.
An abandonment that vanished would render a path that was considered and rejected
as one never considered at all, and a future reader would have no way to tell the
difference.

**Pruning is HITL.** An agent never prunes on its own. Abandoning planned work is a
commitment-shaped decision, and the same guard applies as to the retire cascade's
"ask before treating a **brief-carrying** node as done". An AFK session — every kind
but `requirements` and `prototype` — that finds a leaf dead says so and stops; the
loop stalling on an abandonment decision is the system working, not a fault
(*task-kind-taxonomy*, HITL/AFK).

The qualifier matters more here than the parallel does. A **chain node** carries no
charter and closes silently (*task-tree-scheme*), so the cascade never asks about
one — but `leaf-prune` given a chain node still marks every live step in it, and
that is still HITL. The guard tracks *who decides*, not *which node species*: the
cascade's confirmation is narrow because a brief-less node has no context anyone
could be asked to promote, whereas abandonment is a decision no node shape makes
for you.

## The durable record

`.grove/` is deleted wholesale at the finish cycle, so the tree can hold *that* a
path was abandoned but never *why* — the mark serves this grove's lifetime only.
The durable record goes where grove already keeps the design's current state: the
**ADR set**.

An abandonment always establishes a positive fact about the design — "we rejected
cross-family review" *is* "grove is single-provider". That fact's ADR is the home,
and the rejection rides on it as a **Considered options** entry carrying three
things:

1. **What** was rejected;
2. **Why** — including, where it clarifies, *what is not the reason*;
3. **What would reopen it** — the condition under which the path becomes live again.

If no ADR yet states the fact, write one: a deliberate non-action is a
current-state decision, and it passes the when-to-write test almost by construction
(the whole point is that people will otherwise re-propose it). If the abandonment
is too small to clear that bar, **nothing durable is written** — the tree mark and
the commit message are enough. The existing when-to-write test
(`linkuistics:decision-records`: hard to reverse · surprising · a real trade-off —
all three) does the grading, so grove adds no machinery here, only the discipline.

Part (3) is grove's one addition to the shape of a rejection entry, and it is the
difference between a tombstone and a gate with a key.

## Considered options

- **Delete the leaf (`git rm`), and document that keys are not unique across a
  grove's history (rejected).** This was the status quo — an undocumented hand-edit.
  It is the cheapest option and keeps the tree's vocabulary at a single mark, but it
  buys that by making the permanent key impermanent, which is the one property the
  whole reference scheme rests on (*task-tree-scheme*). *What would reopen it:*
  nothing short of abandoning stable keys entirely.
- **Delete the leaf, but keep keys monotonic by another route (rejected).** A
  `.next-key` counter file is exactly the non-task state constraint 1 forbids;
  deriving max-key-ever from `git log --diff-filter=A` makes the counter depend on
  *running git* rather than *reading markdown* (constraint 2). Both pay real
  architectural cost to buy a tidier view of a directory that is deleted at the
  finish cycle anyway. *What would reopen it:* a grove long enough that abandoned
  leaves genuinely drown the tree — which the finish cycle's hard expiry makes
  unlikely.
- **A taxonomy of outcomes — `abandoned` / `blocked` / `deferred` / `superseded`
  (rejected).** The *task-kind-taxonomy* test governs: a state earns its place only
  by carrying behaviour beyond a name, and none of the extra three does.
  `blocked` is already expressed by **ordering** — the tree sequences prerequisites,
  which is precisely why that ADR dropped upstream wayfinder's `task` type — and it
  is worse than redundant: a blocked leaf is *live work*, so skipping it would make
  `pick` report "no live leaves; this grove is done" while work remained, mis-firing
  the finish cycle. `deferred` is a **reorder** (position is mutable) or a **GitHub
  issue**; both mechanisms exist, and this ADR's own grove was born from the latter.
  `superseded` differs only in *reason*, not behaviour — and the filename's sole
  consumer is a boolean (*does `pick` skip this?*), so encoding a reason there puts
  prose in a namespace that cannot read prose. *What would reopen it:* a proposed
  state that changes what the **walk** does, not merely what a reader infers.
- **`leaf-retire --abandoned` instead of a sibling verb (rejected).** Mechanically
  identical — both are a file move adding an infix, and they share a code path
  regardless — so the flag saves nothing real. It frames abandonment as a *mode of
  completion*, which the methodology prose would then spend its life apologising for;
  and forgetting the flag would silently assert the work was **finished**. A missing
  flag must degrade to something harmless, never to the opposite outcome. Omitting
  `leaf-prune` fails safe: the leaf simply stays live. *What would reopen it:* a
  demonstrated need to abandon and retire through one interface.
- **`PRUNED` as the infix (rejected).** It is grove's own metaphor and coheres
  verb-to-token, but the *state* token is deliberately plain — `leaf-retire` writes
  `DONE`, not `HARVESTED`; the metaphor lives in the verb. Decisively, git's own
  vocabulary already uses **prune to mean delete** (`git remote prune`, `gc
  --prune`), so a cold reader could take `PRUNED` for "slated for removal" while the
  file sits in front of them. Needing an `_Avoid_` note to defend a token against a
  misreading drawn from the very tool grove is built on is the tell that the name is
  fighting its context (constraint 6: with the skill deleted, `.grove/` must still
  read as a legible folder of notes). *What would reopen it:* nothing — this is a
  naming call, cheap to revisit and not worth revisiting.
- **Letting an agent prune autonomously when a premise is disproven (rejected).** It
  would keep unattended runs from stalling, and a falsified spike really is dead. But
  "the premise was disproven" is exactly the judgement an overconfident agent makes
  badly, and the failure mode — quietly closing a path the human still wanted open,
  under a mark that reads as though the *human* decided it — is the one failure this
  ADR most needs to prevent. *What would reopen it:* a class of abandonment whose
  deadness is machine-checkable (a spike gated on a test that fails), which is not a
  class grove can currently define.

## Consequences

- `leaf-prune` accepts a **leaf or a node**. Given a node it marks every *live* leaf
  in that subtree (leaving `DONE` ones alone — that work really was done) and reports
  what it marked; it refuses the grove root, since abandoning a whole workstream is a
  branch-delete, not a tree mark. This arity asymmetry with `leaf-retire` is
  deliberate and is the point: **retirement is incremental** (one leaf per session, as
  work completes) while **abandonment is bulk by nature** — one decision kills N
  leaves at once. Forcing N calls would rebuild the tedium that drives people to
  hand-edit.
- A node is still never marked. Its state remains "no live leaf in the subtree",
  however its leaves finished, so the retire cascade and the finish trigger are
  unchanged.
- The infix column is now fixed *per mark* (`DONE` is 4 chars, `ABANDONED` is 9)
  rather than globally. Cosmetic only: `sort_key` parses the position first, so
  ordering is untouched.
- Existing trees are unaffected — they contain no `ABANDONED` leaves, so there is
  nothing to migrate.

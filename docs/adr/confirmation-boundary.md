# grove asks a human only where the answer decides something the session cannot

A grove session runs unattended by default, so every *routine* question it asks is
a stall the loop cannot clear on its own. Which questions earn that cost is
decided by two tests, applied in order:

1. **Does the answer change what is written?** If every answer leaves the same
   bytes on disk, do not ask.
2. **If it does — is the fact the session's to establish, or the human's to
   decide?** A session can establish *what it did*. It cannot establish *what is
   worth doing*.

Four moments in the loop sit near this line, and the tests separate them cleanly:

| Moment | What it writes | Asks |
|---|---|---|
| Retiring a leaf (`leaf-retire`) | a `DONE` infix | **no** — the session did the work; the mark records a fact it holds |
| Closing a node (the Retire cascade) | *nothing* | **no** — node done-ness *is* the absence of a live leaf |
| Pruning (`leaf-prune`) | an `ABANDONED` infix | **yes** — the mark asserts a path is not worth taking (*pruning*) |
| The finish cycle | deletes `.grove/` | **yes** — one gate, after promotion (*in-session-finish-cycle*) |

The finish cycle's single confirmation is therefore the loop's **only routine
human gate**. Everything else a session asks is an *escalation*: discretionary,
triggered by evidence the session actually met, and always legitimate — the
HITL/AFK mark "predicts, it does not permit" (*task-kind-taxonomy*).

## Why the node close asks nothing

The Retire cascade used to ask the human before treating a brief-carrying node as
done, re-asking at each ancestor. It no longer does, and the reason is test 1: a
node is **never marked** (*task-tree-scheme*). Its done-ness is a pure inference
from the absence of a live leaf in its subtree. Whatever the human answered, the
tree was byte-identical afterwards — and a node "closed" in error is reopened by
one `leaf-add`, with nothing to undo, because nothing was done.

The confirmation was carrying **three jobs**, and none of them needed a question:

- **"Did you forget a leaf?"** The session checks the node's brief `Done when`
  rollup against what the subtree delivered. If it is unmet, that is not a
  question — it is a **missing leaf**, and `leaf-add` is the verb the Decompose
  step already prescribes for exactly this. Asking permission to treat a node as
  done converts a concrete gap into a yes/no about an abstraction.
- **"Promote the brief upward."** Never the question's job, only its sequel. The
  human's yes/no does not tell the session *what* survives — that judgement is the
  session's either way — so promotion now runs unconditionally on a brief-carrying
  node's close. Its edits land in the leaf's own commit, reviewable in the diff and
  revertable in the VCS, which is the same safety *in-session-finish-cycle* relies
  on for its single gate.
- **"Never auto-complete a parent."** grove has nothing to auto-complete. This
  instinct was validated against prior art (task-master) whose parent status is a
  *separate mutable field* that can drift from its children; grove's node
  done-ness has no second field to set and none to drift.

Two further costs made it worse than merely idle. It **recursed**, asking again at
each ancestor and then terminating into the finish cycle's own confirmation — up to
four questions about one fact, which is the wizard anti-pattern
*in-session-finish-cycle* rejects within its own three steps, rebuilt across
levels. And it sat in the **Retire step, which every kind runs**, so an AFK leaf
that happened to be its subtree's last was *guaranteed* to stall at a moment
nothing in the tree predicts. The HITL/AFK mark is a property of the **kind**; a
mandatory question in a step common to all kinds is the one thing that makes the
mark wrong by construction.

## What replaces it

On a brief-carrying node's close the session **verifies and reports** instead of
asking:

1. **Check** the node's brief `Done when` against what its subtree delivered.
2. **`leaf-add`** the missing work if the check fails and the gap can be named.
3. **Escalate** — stop and say so — if the check fails and the gap *cannot* be
   named, because the residue is a scope judgement rather than work. This is the
   ordinary escalation the HITL/AFK mark already permits, not a routine gate.
4. **Promote** what survives from the brief upward, and **report** the close:
   name the finished node by its `<slug>-k<key>` handle in the commit message
   alongside the leaf's own (*task-tree-scheme* §5).

The human reviews the close **after the fact, in the diff**, rather than being
interrupted before it. That is the trade: the question was synchronous and the
report is not, so a human who would have said "no" now says it one session later.
The repair is one `leaf-add` against a node nothing has marked.

**The phrase "all retirements need confirmation" over-reads the design in one
direction too.** Marking a *leaf* done was never confirmed — the loop calls it
mechanical bookkeeping — and that is unchanged. Only the node close moved.

## Considered options

- **Keep the cascade's confirmation (rejected).** It is the status quo and it does
  catch a node the human had further intent for. But it buys that by stalling every
  AFK kind at an unpredictable moment, to gate an inference that writes nothing —
  and the catch it buys is available for one `leaf-add` afterwards, since nothing
  was marked. *What would reopen it:* a node close that acquires a durable mark
  (see below), which would make the human's answer change the tree.
- **Ask once at the top of the cascade instead of per level (rejected).** Fixes
  the wizard shape and nothing else. The question still gates an inference, still
  stalls AFK kinds, and still asks a human with *less* context than the session —
  they have not read the subtree's work. *What would reopen it:* nothing; it is
  strictly dominated by the adopted answer.
- **Ask only when the `Done when` check fails (rejected).** Very close to what is
  adopted, and it differs only in the default for a failed check. A failed check
  names *missing work*, and grove has one answer for missing work; a session that
  can name the gap should cut the leaf, and one that cannot escalates — which is
  the carve-out above. So this option collapses into the adopted answer with a
  worse default. *What would reopen it:* evidence that sessions systematically cut
  the wrong follow-up leaf rather than escalating.
- **Give a node close a durable mark, so the confirmation gates something real
  (rejected).** A `DONE` infix on the node directory would make test 1 pass. It is
  rejected on the ground both *task-tree-scheme* and *pruning* already fix: a node
  is never marked, because a stored parent state is a **second source of truth that
  can drift from its children** — the precise defect grove's implicit model avoids.
  Manufacturing a write in order to justify a question is backwards. *What would
  reopen it:* a consumer that genuinely needs node done-ness without walking the
  subtree, which no viewer or verb has yet needed.

## Consequences

- **Zero code.** The cascade was always prose (*in-session-finish-cycle*'s "the
  cycle's hard part is judgement"); this changes the Retire step's prose and the
  surfaces that restate it.
- **The `BRIEF.md` discriminator survives with its job changed, not removed.** It
  used to select which nodes get asked; it now selects which nodes have *work to
  do* on close — a brief-carrying node has a `Done when` to check and a brief to
  promote, a chain node has neither and still closes with nothing to do. It stays a
  **file test, never a name parse** (*task-tree-scheme*), and `brief-chain`'s
  tolerance of a missing level stays load-bearing for the same reason.
- **The chain-as-node decision stands** (*task-kind-taxonomy*). One of the three
  arguments that had rejected it was that the cascade's confirmation is noise for a
  chain; the brief-less rule had already killed it, and this makes it doubly dead.
  The other two are untouched.
- **`leaf-prune` stays HITL, on its own ground.** Its guard no longer leans on a
  parallel with the cascade — it stands on test 2, because an `ABANDONED` mark
  asserts *a human decided against this* and outlives the session that wrote it
  (*pruning*).
- **Unattended runs cross node closes silently**, which is the point. The commit
  message becomes the surface where a close is visible, so it names the closed node
  by handle.

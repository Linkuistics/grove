## Harvesting a done leaf

The common case: with the task's work done — and *before* you commit it, so the
rename and everything the cascade below writes land inside that one focused
commit — retire the just-finished leaf by
running `grove-llm leaf-retire <leaf-path>` — the verb marks it done **in
place** by adding a `DONE` infix (`NN-<session-kind>--<slug>-k<key>.md` →
`NN-DONE-<session-kind>--<slug>-k<key>.md`, the infix sitting right after the
position so the kind stays where every reader looks for it); there is no `done/`
directory, and the leaf keeps
its position and key in its directory. Mechanical bookkeeping, no need to ask.

**Retirement touches one filename and nothing else** — not the leaf's own body
(its `# <slug>-k<key>` header included), not a sibling, not an ancestor. A review
leaf waiting beside the producer is no exception: it reads the committed
artifact, and needs no record of how the session that produced it ran.

## Retiring the last live leaf is still an ordinary retirement

**You do not discover that a grove is finished — the driver does**, and it says
so by launching a `finish` session. So retiring the last live leaf in the tree is
the same filename transition as any other, never a cue to promote artifacts,
delete `.grove/`, or tear anything down. That work belongs to the one kind
chartered for it, on explicit human confirmation.

## Pruning an abandoned one

**An agent never prunes on its own.** An AFK session that has reached this point
says so and stops rather than making the call; the loop stalling there is the
system working, not a fault.

Only on explicit human confirmation, run `grove-llm leaf-prune <path>` (a leaf
or a node — given a node it marks every live leaf in the subtree, leaving `DONE`
ones alone, and refuses the grove root) to mark it `ABANDONED` in place.
Pruning a reviewed producer leaves any review leaf beside it live, next, and
deliberately uncheckable — nothing was produced for it to read. A chain is flat
siblings, not a node, so there is no enclosing directory to prune in one call: if
the human is abandoning the whole reviewed path, prune each of its live steps.
Usually there is only one — a review leaf exists only when a producer already
decided review was required, and an integrate leaf only when that review found
something.
`.grove/` dies at the finish cycle, so the mark records only *that* the path
closed — the durable *why* (what was rejected, why, and what would reopen it)
goes to the **ADR set**, the positive fact the abandonment establishes, if it
clears the when-to-write bar; otherwise the mark and the commit message suffice
(pruning).

## When a leaf's place is in doubt, the sentence picks the verb

Doubt about whether a leaf still belongs — surfacing new, or already sitting
there under a doubted premise — always resolves to one of three mechanisms the
tree already has:

- **Not now, but still ours** → a **reorder**. The work is good and belongs in
  this tree, just not next: leave the leaf live and `leaf-insert` something ahead
  of it, or reorder by hand. Nothing has been rejected; `pick` returns it in its
  turn.
- **Not ours at all** → a **GitHub issue**, not a leaf. Work belonging to a
  different repo, owner or workstream is filed and dropped rather than grown into
  this tree. If it already has a leaf here, prune it once the issue is filed.
- **Considered on its merits and rejected** → a **prune**, above.

Misfiling any of the three corrupts the tree in a different way: reordering a
rejected leaf keeps a dead end reading as *still coming*; pruning a not-yet-due
leaf erases work nobody rejected; leafing a not-ours concern grows the tree with
work this grove will never do. So name which of the three sentences is actually
true before reaching for the CLI.

**There is no fourth state.** A leaf whose place is in doubt gets no status
word — no `blocked`, no `deferred`, no `superseded`. `blocked` is expressed by
ordering and is *live* work, so a `pick` that skipped it would break the finish
trigger; `deferred` is a reorder or an issue; `superseded` differs only in
*reason*, which is prose for the ADR set and never a filename.

## Closing a node: the four steps

A node is **never marked**: its done-ness *is* the absence of a live leaf
anywhere in its subtree, so a close is work in the parent chain rather than a
write to the node. **The close asks the human nothing** — it infers done-ness
rather than deciding it. Every node carries a `BRIEF.md`
— it is a leaf that proved bigger, and the charter is what those extra sessions
needed — so every close has the same four steps:

1. **Check** the node's brief `Done when` against what its subtree delivered.
2. **`leaf-add`** the missing work if the check fails and you can name the gap —
   a failed check names *work*, and grove has one answer for missing work. Name
   its `--kind`; the verb requires one and guesses nothing.
3. **Escalate** — stop and say so — if the check fails and you *cannot* name the
   gap, because the residue is a scope judgement rather than work. That is an
   ordinary escalation, discretionary and always legitimate, not a routine gate.
4. **Promote** anything still relevant from the brief upward — to the parent
   brief, an ADR, or the glossary — so it stays in the brief chain of future
   siblings; and **report** the close by naming the node's `<slug>-k<key>` handle
   in the commit message alongside the leaf's own. A brief is context rather than
   a task, so it is never marked done and promotion is what a close does with it
   (`BRIEF-FORMAT.md`). The human reviews the close after the fact, in the diff.
   The brief and its now-terminal leaves stay exactly where they are (nothing
   moves).

If you meet a node whose charter was never written, do steps 2–4 and skip step
1: there is no `Done when` to check and nothing to promote. That is a lapse in
the tree, not a second species — grove writes no brief-less node.

Retirement is also the moment to **reconcile the ADR set** with what the
finished work established — `ADR-FORMAT.md` carries how a set is reworked and
its citations chased. That may leave the next ancestor with no
live leaf either; re-check and recurse, until a node still has a live
leaf or you reach the grove root — silently, so an unattended run crosses a whole
chain of closes without stopping. Terminal branches stay in the tree, marked in
place, never deleted while the grove is live — so a recursive view of `.grove/`
(`find .grove`, or a tree-style file manager) shows the whole state, done and
abandoned alike. The cascade walk and the brief-promotion-upward stay prose
deliberately: both are judgement steps (does the `Done when` hold? what survives
upward?) with no stable input/output shape that would justify a verb.

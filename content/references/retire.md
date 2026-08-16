## Harvesting a done leaf

The common case: with the task's work done — and *before* you commit it, so the
rename and everything the cascade below writes land inside that one focused
commit — retire the just-finished leaf by
running `grove-llm leaf-retire <leaf-path>` — the verb marks it done **in
place** by adding a `DONE` infix (`NN-<session-kind>-<slug>-k<key>.md` →
`NN-DONE-<session-kind>-<slug>-k<key>.md`, the infix sitting right after the
position so the kind stays where every reader looks for it); there is no `done/`
directory, and the leaf keeps
its position and key in its directory. The infix is filename-only — the file's
contents (including its `# <slug>-k<key>` header) are untouched. Mechanical
bookkeeping, no need to ask.

## Pruning an abandoned one

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

## Closing a node: the four steps

Every node carries a `BRIEF.md`
— it is a leaf that proved bigger, and the charter is what those extra sessions
needed — so every close has the same four steps:

1. **Check** the node's brief `Done when` against what its subtree delivered.
2. **`leaf-add`** the missing work if the check fails and you can name the gap —
   a failed check names *work*, and grove has one answer for missing work.
3. **Escalate** — stop and say so — if the check fails and you *cannot* name the
   gap, because the residue is a scope judgement rather than work. That is an
   ordinary escalation, discretionary and always legitimate, not a routine gate.
4. **Promote** anything still relevant from the brief upward — to the parent
   brief, an ADR, or the glossary — so it stays in the brief chain of future
   siblings; and **report** the close by naming the node's `<slug>-k<key>` handle
   in the commit message alongside the leaf's own. The human reviews the close
   after the fact, in the diff. The brief and its now-terminal leaves stay
   exactly where they are (nothing moves).

If you meet a node whose charter was never written, do steps 2–4 and skip step
1: there is no `Done when` to check and nothing to promote. That is a lapse in
the tree, not a second species — grove writes no brief-less node.

Retirement is also the moment to **reconcile the ADR set**
with what the finished work established: edit it in place to keep it a minimum
coherent set (merge / split / delete), and fix any citation the rework leaves
dangling — in the briefs, the other ADRs, or `docs/`; never append a superseding
ADR (`linkuistics:decision-records`). That may leave the next ancestor with no
live leaf either; re-check and recurse, until a node still has a live
leaf or you reach the grove root — silently, so an unattended run crosses a whole
chain of closes without stopping. Terminal branches stay in the tree, marked in
place, never deleted while the grove is live — so a recursive view of `.grove/`
(`find .grove`, or a tree-style file manager) shows the whole state, done and
abandoned alike. The cascade walk and the brief-promotion-upward stay prose
deliberately: both are judgement steps (does the `Done when` hold? what survives
upward?) with no stable input/output shape that would justify a verb.

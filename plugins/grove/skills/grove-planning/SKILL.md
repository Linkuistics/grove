---
name: grove-planning
description: The `planning` session kind — given the design, grow the tree; find the smallest independently useful working increments before cutting any leaf. Use when a grove mandate names this skill, or when running a `planning` session in a grove working tree.
harnesses: [any]
---

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/to-tickets/SKILL.md, vertical-slice-rules)
     — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

# planning

**Load the `grove` skill now.** It is the shared spine and holds everything
this kind does not own: the constraints, the bootstrap, and the execute,
decompose, retire and commit procedures. `grove` is the bare skill name. If your
harness requires a plugin namespace, use the namespace under which `grove` is
actually installed. Do not assume one. If you cannot find or read the `grove`
skill, follow the launch prompt's rule for a missing kind skill. What follows is
`planning`'s, and is stated nowhere else.

**planning** (AFK) — given the design, **grow a tree serving a bounded delivery**.
The only kind with methodological force — the sole branch in the
loop's Execute step, and the only kind that grows the tree generatively.

## Bound the first useful outcome

State the useful result this grove will deliver and the acceptance that will
show it works. Match that bar to the requested outcome and its material risks;
do not turn every imaginable refinement into a completion condition. Necessary
safety checks and already agreed obligations remain binding. When that bar is
met and required work is settled, stop expanding the grove and use its normal
finish procedure. Keep future improvements for a later grove or the project's
backlog; a possible next increment is not automatically a leaf here.

If the existing charter commits to a broader result, retain its acceptance and
live remainder until the human explicitly confirms a scope change. A smaller
first delivery must never silently erase the rest.
`grove/references/decompose.md` owns the
admission test for further work and the response to repeated handoffs that leave
acceptance unchanged.

## Find the working increments before cutting any leaf

Before slicing a design into leaves, search actively for the **smallest
independently useful working increments** and order them by dependency. Create a
separate grove for every obvious stage that leaves the product working and
delivers useful, verifiable behavior on which its successor can build. Changes
that **cannot independently leave the product working** stay in the same
increment even when their code edits lie in different modules.

The boundary is product behavior, not code location or one design document's
scope. Schema expansion, caller migration, lifecycle cutover, cleanup,
methodology and documentation often form dependency-ordered groves when every
handoff remains green. A new schema and the only reader that makes it usable do
not: neither half is a working increment on its own.

Only then cut the current increment into session-sized leaves. Tie each
prerequisite to the next working behavior and its finite handoff. Before creating
them, apply the placement test in `grove/references/decompose.md`: work that fits
one leaf stays inline; separate sessions sharing the existing scope become
ordered siblings; a new node needs a distinct subgoal, joint completion
condition and shared context. Record why each proposed node earns another level
and check the resulting root-to-leaf reading burden. Dependency order alone
belongs in sibling positions. Write the ordered task files and only the node
briefs that this test justifies (`BRIEF-FORMAT.md`).

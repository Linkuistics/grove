<!-- file: order=11 -->
<!-- unit: task-producer-planning kinds=planning class=triggering defers="driving-find-working-increments driving-what-a-good-child-leaf-looks-like" -->
- **planning** (AFK) — given the design, first find the **smallest independently
  useful working increments** and order them by dependency. Create a separate
  grove for every obvious stage that leaves the product working and delivers
  useful, verifiable behavior for its successor; changes that cannot
  independently leave the product working stay in one increment even when their
  code edits are separable. Then cut the current increment into vertical slices
  and **grow the tree**: turn an oversized leaf into a node — a **directory**
  `NN-<slug>-k<key>/` holding ordered child leaves, headed by a `BRIEF.md`
  charter. A node is always a leaf that proved bigger, so it always carries one.
  The deliverable is *more tree*. The only kind with methodological force — the sole
  branch in the loop's Execute step.
<!-- unit: task-deliverable-planning kinds=planning class=triggering defers=brief-the-node-briefing -->
- **planning** writes the child `BRIEF.md`(s) and ordered leaf files for any node
  it grows (`BRIEF-FORMAT.md`).

<!-- unit: driving-find-working-increments class=procedural -->
<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/to-tickets/SKILL.md, vertical-slice-rules)
     — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

## Find working increments before child leaves

Before slicing a design into leaves, search actively for the **smallest
independently useful working increments** and order them by dependency. Create a
separate grove for each obvious stage that leaves the product working and
delivers useful, verifiable behavior on which its successor can build. Changes
that cannot independently leave the product working stay in the same increment
even when their code edits lie in different modules. Only then cut the current
increment into child leaves.

The boundary is product behavior, not code location or one design document's
scope. Schema expansion, caller migration, lifecycle cutover, cleanup,
methodology, and documentation often form dependency-ordered groves when every
handoff remains green. A new schema and the only reader that makes it usable do
not: neither half is a working increment on its own.


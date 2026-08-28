# module-split-k4

## Goal

Cut `docs/specs/module-decomposition.md` into `impl` leaves. The spec is the
input and is not to be redesigned; this leaf decides **sequence and batch size**,
and writes each leaf's body so an executing session needs only its own leaf, the
brief chain and the spec.

## Context

Read `docs/specs/module-decomposition.md` first, then `decomposition-k2`'s
`## Decisions (running log)` for the warrants behind the calls the spec states
without arguing.

`references/decompose.md`'s **expand → migrate → contract** rule applies: this is
a wide refactor whose blast radius makes any single vertical slice unable to land
green. One leaf per stage, added in order — a leaf that adds the new form beside
the old, then a leaf per migration batch sized by blast radius, then a leaf that
deletes the old form once no caller remains.

## Done when

- Every decision in the spec is covered by at least one leaf, and the mapping is
  written down so a later session can check coverage without re-reading both.
- Each leaf is a vertical slice that can land with the suite green, or is
  explicitly named as an expand or contract stage that cannot be and says why.
- The two forced orderings below are honoured, and any others found are recorded
  with the constraint that forces them.
- Nothing in the plan requires a session to redesign an interface the spec fixes.

## Notes

**Two orderings are forced rather than chosen.**

1. **The store's four new operations come before grove's second lock layer can
   go.** The reason grove holds its own lock on top of the library's is that the
   library cannot answer *is there a tree here* — and that the two deadlock,
   because two open file descriptions on one directory do not share a lock. All
   three of the recorded reasons dissolve at once, and not before: `exists?` as a
   shape, migration deleted, and `initialize`/`delete` owning their own state.
2. **The grammar rename is inseparable from the reinstall.** This repo is a
   meta-grove: a session runs against the *installed* binary, so the tree cannot
   wear the `--` grammar until an installed binary parses it. Rename and
   reinstall are one step with no session between them, and the leaf that does it
   should say so in its own body — a session that renames and stops has wedged
   the loop.

**Three independent starting points.** The runner's extraction depends on
nothing; the VCS seam's depends only on the git lane being dropped; the store's
new operations depend on nothing. The skills question depends on both the loop
and the runner, because it is what makes a task type a label rather than a
compiled variant.

**Deletion is not one leaf.** The spec's out-of-scope list and
`minimalism-k1`'s `## Deletion list` together name roughly 15,200 non-test lines,
split into *contained* (no caller survives) and *reconciled* (every surviving
call site named). The contained set can go in large batches; the reconciled set
is sized by its call-site count, and `tree_access` is the awkward one — seven
sites in surviving code need rework, not deletion.

**The methodology corpus is part of the work, not a follow-on.** `TASK-FORMAT`
states the filename grammar and the closed set of nineteen kinds; both change.
The corpus also moves out of the binary entirely and into a plugin of twenty
skills, and the delivery assertion's instrument moves with it. Whether that is
one leaf or several is this leaf's call, but it is not free and it is not last —
the driver's prompt and the plugin's skill names have to agree on the same token
from the moment the grammar changes.

**Consider whether the store's work wants its own node.** `ordinal-fs-tree` is a
separate bounded context with its own glossary, its own architecture document and
its own formal models, and the four new operations touch all three. If the
sequence there runs past two leaves, `leaf-decompose` is cheaper than a flat run
of siblings whose shared context has nowhere to live.

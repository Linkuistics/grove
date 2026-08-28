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

## Decisions (running log)

**One grove, not a chain of them.** `references/planning.md` asks first for the
smallest independently useful working increments, and for a separate grove per
obvious stage. The stages here — deletion, the three extractions, the grammar,
the plugin — do each leave the product working, but none of them is separately
*useful*: it is the same tool throughout, and the brief's `## Done when` is a
single five-part test over all five modules at once. Splitting into separate
groves would orphan a charter that already spans them and would put the finish
cycle in the wrong place. So: one grove, eighteen ordered root entries, and the
ordering carries the plan.

**Eighteen leaves, and every one lands green.** The wide-refactor rule
(`references/decompose.md`) says expand → migrate → contract, one leaf per stage,
when the blast radius makes a vertical slice impossible. It does — but the unit
that fans out here is the **crate boundary**, not a symbol. A new crate compiles
beside the old module, its callers move, the old module dies, and all three fit
one session because a crate is small. So the expand/contract split is spent only
where the crate itself is too big for one session (`keyed-launch` into templates
and run; `grove-loop` into verbs and driver) or where the two forms must coexist
across sessions for a reason outside the compiler (the plugin ships at
`plugin-spine-k16` and `plugin-kind-skills-k17`, and provisioning dies at
`delete-provisioning-k19`, because deleting the old delivery path before the new
one is installed leaves the next session with no methodology and fails silently).
That is why no leaf here is an expand or contract stage that cannot land green,
and the planning task's second `Done when` is met by the stronger answer rather
than by the exemption it offered.

**The store's work earns a node; the collapse it enables does not join it.** The
task file asked this directly, with a two-leaf threshold. It runs to three —
`Sought` and its whole search surface; the `Reading`/`Writing`/`Vacancy` reshape
with `initialize`; `delete` with `Removed` and the ADR clause it obliges — and
all three touch the crate's own glossary, architecture document and two formal
models. That shared context is exactly what a node's brief is for. Deleting
grove's second lock layer stays a **flat sibling** of the node rather than a
fourth child: it is grove-side work stated in grove's vocabulary, and putting it
under a brief whose whole discipline is *say nothing grove can hear* would
corrupt the boundary `CONTEXT-MAP.md` exists to hold.

**A third forced ordering, beyond the two the task file named.** The two given
were the store's operations before the second lock layer, and the rename
inseparable from the reinstall. The third is the plugin's: `plugin-kind-skills-k17`
must precede `prompt-names-the-kind-k18`, because the prompt may not name a skill
that does not exist; and both must precede `delete-provisioning-k19`, because the
binary is what currently writes `~/.claude/skills/grove` and its two siblings, and
a session launched after that write stops and before the plugin is installed does
not crash — it improvises. Three smaller ones are recorded in the brief's table
with the constraint that forces each: the git lane before the seam can claim
domain-freedom, the handle's owner before the grammar moves, and all three
extracted crates before the driver that composes them.

**The name collision between the provisioned `grove` skill and the plugin's
`grove` spine is real and is answered by sequencing, not by renaming.** On Claude
Code the marketplace namespaces a plugin's skills, so the plugin can be shipped
and verified there while the binary still provisions. The symlink farm cannot —
`plugins/install.sh` would want the same three paths the binary owns — so the
farm install is deferred to `delete-provisioning-k19`, where it is the natural
last step. `plugin-spine-k16` also carries the smaller open question this
surfaces: whether the token the prompt names is the bare `grove-<kind>` or a
plugin-qualified one. It is decided there, while the spine is the only skill, and
written down where `prompt-names-the-kind-k18` reads it.

**The meta-grove hazard is recorded once, in the root brief, rather than in every
leaf.** A session here runs against the *installed* binaries, so a leaf that
changes the grammar or the verb surface must reinstall in the same session. Two
leaves do — `grammar-separator-k15` and `open-kind-k20` — and they differ in one
way worth stating: until `prompt-names-the-kind-k18` retires the build-pairing
guard, a mid-session reinstall halts the loop between iterations (which is what
makes k15 safe and its stall expected); afterwards it does not, so k20 must check
the installed binary itself. `release.toml`'s own commentary is the source for
the first half; the second follows from retiring the guard.

**`tree_format`'s fate is left as a named question rather than decided here.**
`minimalism-k1` lists it among the survivors; `decomposition-k2` then decided
`.grove/FORMAT` dies with migration. The later decision governs the file, and
whether the module has anything left to do is a code question the session holding
the code should answer. `delete-migration-k6` carries it as an explicit open
question with the governing decision named, which is the honest form — a planning
session that guessed would have written a wrong `Done when`.

**A `review-planning` leaf is cut ahead of the run.**
`references/decompose.md`'s test names *a decomposition others will build on for
months* as earning a chain, and this is eighteen sessions deep with two
hard-to-reverse orderings and one leaf whose failure mode is a wedged loop rather
than a red test. It is cut with `leaf-insert` at the run's first slot rather than
appended, because a review appended after eighteen leaves would run last and
review nothing anyone could still act on cheaply. Its body names the four
specific doubts — coverage, ordering, the every-leaf-lands-green claim, and the
meta-grove install sequences — rather than asking for a general read.

**The in-session reviewer allowance was not spent.** This session has a scheduled
`review-planning` leaf beside it, and `references/execute.md` is explicit that a
producer with a scheduled review spends none: grove owns the route once review is
escalated to the tree, and a competing in-session reviewer beside a scheduled one
is the thing that rule forbids.

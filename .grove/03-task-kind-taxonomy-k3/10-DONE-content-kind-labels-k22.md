# content-kind-labels-k22

**Kind:** impl

## Goal

Retire the last pre-taxonomy **kind labels** from `content/` — the bundled
methodology an agent actually reads at runtime — so no provisioned file names a
kind that the seventeen-kind set no longer has, or attributes an artifact to the
wrong kind.

## Context

Surfaced by `planning-grills-leftovers-k21` while running that leaf's repo-wide
Done-when grep, and externalized rather than absorbed: `k21`'s subject is the
claim *"`planning` grills"*, and neither site below makes that claim. These are
a **different species** — a label naming a kind that was renamed, and a label
naming the wrong kind as an artifact's author. Same origin (`config-sweep-k16`
swept the doc surface it enumerated, and these two files were not on that list at
these lines), different defect.

Both live in `content/`, which is the aggravating factor: `grove do` provisions
`content/` to `~/.claude/skills/grove/`, so these are read by every session on
every grove, not by a human browsing the repo.

- **`content/driving.md`**, *Anti-patterns* → *The pre-baked answer*: "you don't
  need a grilling session — that's a **work** task." `work` was renamed `impl`.
  The repo-facing sibling `docs/driving-a-grove.md` already says `impl` at the
  same paragraph, so this is the mirror image of `k21`'s asymmetry — there the
  bundled copy was current and the repo copy stale.
- **`content/SPEC-FORMAT.md`**, at least four sites: a spec is "written by a
  **planning** task" (line ~6), "Most **planning** increments write no spec at
  all" (~12), "A spec synthesises the **planning** task's running decision log"
  (~91), and "a node's settled design reaches its child **work** leaves" (~118).
  Under `docs/specs/task-kind-taxonomy.md` the spec is `design`'s deliverable —
  `content/SKILL.md` ("produced lazily by a `design` task") and `docs/concepts.md`
  ("written by a `design` task") both already say so, so `SPEC-FORMAT.md` is the
  one file left disagreeing with grove's own account of who writes a spec.

## Done when

- No file under `content/` uses `work` as a live kind label. The three
  *deliberate* mentions stay: `TASK-FORMAT.md` and `SKILL.md` both state the
  read-compatibility rule (`work` is the previous spelling of `impl`), and that
  rule is the reason live groves keep working — do not "fix" those.
- `content/SPEC-FORMAT.md` attributes the spec to `design`, and its
  running-decision-log sentence names the kind that actually produced the log.
  Check the surrounding argument survives the relabel: the *grill → spec →
  decompose → execute* flow spans `requirements` → `design` → `planning`, so a
  blanket find-and-replace to `design` will mis-state at least the first clause.
- `content/SPEC-FORMAT.md`'s "child work leaves" reads `impl`.
- A grep across `content/` for a kind label outside the seventeen-kind set comes
  back empty, or every survivor is one of the deliberate mentions above.

## Notes

Docs only — no code, no ADR, no CHANGELOG bullet of its own; fold it into the
existing *Five task kinds become seventeen* entry's "Documented across …" line,
as `k21` did.

The `grove-llm` on `PATH` is **v15.0.0**, which predates this node's own rework,
so `leaf-add --kind impl` is refused by the installed binary and must be run as
`./target/debug/grove-llm`. Expected, not a defect — the taxonomy is unreleased.

**A fifth site turned up, and it exposes a hole in this leaf's own Done-when
grep** — the methodological finding worth more than the fix:

- **`content/BRIEF-FORMAT.md`**, line 14: "A brief is written by the planning
  task that creates its node." Absorbed, not externalized: it is the *same
  species* as `SPEC-FORMAT.md`'s defect and it fails this leaf's **Goal** verbatim
  ("attributes an artifact to the wrong kind"). Pre-taxonomy the sentence was
  simply true — `planning` was the only node-creating kind. The taxonomy split
  node creation in two: **generative** decomposition (a `planning` session whose
  deliverable *is* the tree) and **reactive** decomposition (`leaf-decompose`,
  explicitly kind-agnostic, inheriting the parent's kind). The brief now names
  both.
- **Why the Done-when's fourth clause could not have found it.** That clause greps
  for "a kind label outside the seventeen-kind set" — and `planning` is *inside*
  the set. A mis-attribution to a **live** label is invisible to a set-membership
  grep; it needs an *attribution* grep (`written by (a|the) <kind>`). Both greps
  ran here, and only the second one fired. A future sweep should run both.

**The glossary was corrected inline** (`CONTEXT.md`, **Spec**), as any kind may.
Not scope creep: it carried the identical "written lazily by a planning task"
claim, and leaving it would have put the glossary — read every session, and the
one forcing function against terminology drift — in direct conflict with the file
this leaf was fixing. It also gained an `_Avoid_` line, so the next reader is
warned off the pre-taxonomy reading rather than merely not being told it.

**The flow clause was labelled, not replaced**, exactly as `Done when` warned: it
now reads grill (`requirements`) → spec (`design`) → decompose (`planning`) →
execute (`impl`). `docs/concepts.md` leaves the same flow unannotated and says
`impl` at the end; annotating it here is the extra precision a *format reference*
earns and a concepts overview does not, and the two do not disagree.

One article slip in `docs/driving-a-grove.md` ("a `impl` task", introduced by
`k21` at the mirrored paragraph) was fixed in passing — a one-character typo, not
a concern worth a leaf.

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

# docs-reconcile-k6

**Kind:** impl

## Goal

Reconcile `docs/superpowers/specs/2026-07-29-portable-codebase-memory-skill-design.md`
and `docs/superpowers/plans/2026-07-29-using-codebase-memory-skill.md` with what
`skill-k2` actually measured. Both are committed and both state falsified claims
as **verified fact**, in sections that explicitly instruct a reader not to
second-guess them.

Run this **last** — after `skill-integrate-k4`, so the shipped `SKILL.md` is
final and the docs are reconciled against the artifact rather than a draft.

## Context

1. `.grove/BRIEF.md` § *Verified contract* and § *Corrections* — the measured
   facts and the list of what they falsify.
2. `plugins/linkuistics/skills/using-codebase-memory/SKILL.md` — the
   authoritative statement. Do not re-derive; cite it.
3. The two docs above.

## Done when

- Neither doc asserts, as verified, any of: error payloads on stdout;
  `| jq -r '.error'` as a working idiom; "byte-identical results" from
  `relationship`/`direction`; a `limit:200` client-side sort as a correct
  "top N fan-in".
- Each surviving CLI claim either matches the skill or is deleted in favour of
  a pointer to it. The docs are design/planning records, so **prefer deleting a
  restated fact over maintaining a second copy of it** — the skill is the
  minimum coherent statement.
- Committed.

## Notes

**Judgement call this leaf owns: how much to rewrite.** These are
point-in-time superpowers workflow records, not `docs/specs/` members, so the
"minimum coherent set, edited in place" rule does not straightforwardly apply
and wholesale rewriting an executed plan is churn. The defensible minimum is to
stop them asserting false things a future session is told to trust — the plan's
Global Constraints open with *"Do not restate any of these from memory — they
are counter-intuitive"*, which is precisely what makes the wrong ones dangerous.
Decide and say which you did.

**Also check** whether the spec's *Testing* section still reads sensibly: it
names `/Users/antony/Development/grove` as unindexed, and `skill-k2` indexed
*this* working tree (`Users-antony-Development-grove.using-codebase-memory`,
1861 nodes / 7468 edges, `mode:"fast"`) while verifying the fallback command.
Drop that index with `delete_project` if it is not wanted.

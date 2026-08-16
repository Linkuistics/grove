# skill-opening-k16

## Goal

Write `SKILL.md`'s first screen — the frontmatter `description:` and the routing
table — and audit the whole rewritten file against its budgets and its
no-procedure obligation. Last child, because the opening can only be judged once
every condition line exists.

## (1) The frontmatter `description:`

Its job **changes and narrows**. It is no longer the trigger that matters —
`${prompt}` names the skill outright — but it is still the fallback for a session
a human started inside a grove working tree, and it is what a session matches the
prompt's instruction against.

**The skill stays model-invoked.** `disable-model-invocation: true` would remove
the fallback and, on a harness that lists only model-invoked skills, would remove
the skill from the list the prompt tells the session to reach into.

The current description is written for a session deciding *whether* to start a
grove — *"Use when driving a long, multi-session workstream that cannot be planned
exhaustively upfront…"* (`content/SKILL.md:3`). A session **already inside one**
can read that and conclude the skill is about the choice rather than about how to
run this session. That is a real undertriggering path and it is the failure this
rewrite closes. Rewrite to the house shape — a **capability clause plus an
explicit "Use when"** — whose first trigger is the situation every driver-launched
session is actually in: *a Grove mandate names this skill*.

## (2) The routing table

The first screen **routes rather than introduces**: it names the reference file for
each kind, so a session that arrived by description match rather than by mandate
still lands in the right place without a mandate having named one. Ten rows, the
table `per-kind-references-k12` built.

It states **conditions and no procedure**, because a description or an opening
that summarises the *workflow* becomes a shortcut the session takes instead of
reading the body — which is the same failure the whole rewrite is against, one
level in.

## (3) The whole-file audit

Three checks, and one of them is **not a test**:

- **Body budget** — at or under 500 lines. Mechanical.
- **Loop section** — at or under 100 lines, measured heading to next heading of the
  same level, blank lines included. Mechanical. `loop-conditions-k13` owns writing
  it; this child confirms it survived two more children appending condition lines.
- **No procedure in `SKILL.md`** — **a review obligation, discharged by a human
  against named evidence.** *Procedure* has no classifier once the unit markers are
  deleted; that classification was the markers' whole job. The spec is explicit
  that a corpus-budget test that passes says nothing about this, and that **no
  budget test may be cited as evidence for it**.

## Done when

- The frontmatter `description:` is rewritten to the house shape, model-invoked,
  first trigger *a Grove mandate names this skill*.
- The routing table is the first screen and covers all nineteen kinds through ten
  files.
- Both line budgets hold, with the measures written into the suite so a later
  session cannot re-grow the file silently.
- **A `review-impl` leaf is cut with `leaf-add`** carrying the no-procedure
  obligation explicitly: per section, does it state a condition and route to a
  reference file, and does no section state steps a session performs — the evidence
  being the section itself against the reference file it routes to, which is where
  the corresponding procedure must be found. The review records this as a **finding,
  not a test result**.

## Notes

The spec names a second review obligation this leaf's review is the natural home
for: that the rewritten corpus states the two rules the core sheds — *the driver's
pick is authoritative and must not be re-walked*, and *the driver's stated version
control is definitive and is not re-derived from the working tree or from a
harness banner*. `loop-conditions-k13` writes them; confirming they are still
there after three children of editing is a review question, not a test.

The whole-corpus target for comparison: **~200 lines, roughly 8 kB, against
today's 50 kB.** If the file lands materially above that, the condition lines are
still carrying prose that belongs in `references/` — which is the no-procedure
finding, arriving as a size symptom.

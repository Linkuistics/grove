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

## Running log

**The `description:`.** Rewritten to the house shape — a capability noun phrase
("Grove's methodology for driving a long, multi-session workstream as a
VCS-tracked tree of task files under .grove/"), a content list that elaborates it
without sequencing anything, then an explicit *Use when* whose **first** trigger
is *a Grove mandate names this skill*. Two further triggers follow, narrowing
outward: any session inside a grove working tree, then the original
start/pick-up/continue clause the old description carried alone. 401 chars,
inside the house's ~470 and far inside the 1024-char frontmatter limit. The
skill stays **model-invoked** — no `disable-model-invocation`.

**The routing table.** New unit `skill-kind-routing`, `kinds=* class=triggering`,
placed immediately after the one-paragraph definition and before the spine, so
the first screen is nineteen lines of routing rather than an introduction. Ten
rows, verified to resolve. It defers nothing: it *is* the pointer, so there is no
remainder to move.

**Budgets — both hold, and the interesting number is neither of them.**

| measure | value | bound |
|---|---|---|
| body lines (after frontmatter) | 404 | ≤ 500 |
| `## The loop` heading-to-heading | 94 | ≤ 100 |
| file bytes | 25,811 | — |

Both mechanical limbs pass. Written into the suite as
`the_skill_body_fits_the_progressive_disclosure_ceiling` (with a `body_lines`
helper that returns `None` on a stripped header, so a missing frontmatter fails
rather than measuring the whole file) beside the loop-section alarm
`loop-conditions-k13` already added, plus
`the_skills_routing_table_names_a_reference_file_that_exists`, which guards this
child's own deliverable — ten rows, exactly the ten expected paths, each one
present in `content/references/`.

**The overshoot, which is the finding.** The brief's target was ~200 lines and
roughly 8 KiB. The body is 404 lines and 25.8 kB. Netting out the scaffolding
that dies in `mandate-machinery-k10` — 54 comment lines, 5,326 bytes of unit
markers and licence notices — the post-cutover file is still **~350 lines and
~20.5 kB, 1.75× and 2.5× the target**. The arithmetic locating it: 51 triggering
units over 287 lines of prose is **~5.6 lines and ~400 bytes per condition**,
against a spec estimate of one condition *line* each. The budget tests say
nothing about this, by design, and I have not cited them as if they did.

**I did not trim.** The four sibling children each wrote their own condition
lines and are `DONE`; rewriting them here would absorb into this leaf work that
belongs to the review's integration step, and this leaf's own bar is the audit
plus the review. The overshoot is written into `skill-opening-k17`'s body as
named evidence, with the per-condition arithmetic, so the reviewer starts from a
number rather than an impression.

**Verification.** Full `cargo test` green (all suites, 0 failed), `cargo fmt
--check` clean, `cargo clippy --all-targets` silent. `composed-mandates.tsv`
regenerated: the diff is exactly one `skill-kind-routing` row per kind, 19 rows,
each landing after `skill-what-a-grove-is` — a pure addition, no relocation.

**Both rules the core sheds are still present** after four children of editing:
`skill-do-not-pick-again` (the mandate wins over a second walk) and
`skill-stated-vcs-is-definitive` (the driver's stated lane is not re-derived, and
a harness banner does not win). Confirming they survive is the review's job, not
a test's; recorded here as the producer's own reading.

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

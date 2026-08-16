# skill-opening-k17

**Reviews:** skill-opening-k16

## Goal

Discharge the two review obligations `skill-delivered-methodology` places on the
rewritten `content/SKILL.md` — obligations the suite deliberately cannot carry.
The producer's commit names `skill-opening-k16`; read that diff, and read the
whole of `content/SKILL.md` as it now stands, against the reference files it
routes to.

Inspection only. Do not run the build, the tests, a formatter or a linter, and do
not edit `content/` or `tests/`. Every fix and all post-fix verification belong to
the paired `integrate-review-impl` step.

## (1) The no-procedure obligation — the primary finding

**`SKILL.md` SHALL state conditions and no procedure**, and the spec is explicit
that this is *a review obligation discharged by a human against named evidence,
not by a test*: **procedure** has no classifier once the unit markers are deleted,
that classification was the markers' whole job, and **no budget test may be cited
as evidence for it**. Two budget tests now pass
(`the_skill_body_fits_the_progressive_disclosure_ceiling`,
`the_loop_section_of_the_skill_fits_a_page`). Neither is evidence here. Do not
report them as if they were.

**Work it per section**, and for each ask two questions:

1. Does the section state a **condition** — a situation that holds, or a fact
   about the tree, the driver or the record sets — and route to a reference file?
2. Does it state **steps a session performs**?

**The evidence for each answer is the section itself set against the reference
file it names**, which is where the corresponding procedure must be found. A
section whose steps do not appear in the file it routes to has either kept
procedure that should have moved, or lost it in transit — and those are different
findings, so distinguish them.

**Candidate starting points, from the producer's own reading.** These are leads,
not a worklist; derive your own set and say where you disagree with these.

- `skill-bootstrap` — *"Then read, in order, the glossary, the ADRs the briefs
  cite, the `BRIEF.md` chain root→leaf, and the task file"* is an ordered
  four-step read. Condition or procedure?
- `driving-when-code-depends-on-a-framework-version` — *"Read the manifest, fetch
  the official docs, cite at the decision site, and flag what you could not
  verify"* is the clearest four-step sequence left in the file.
- `driving-when-asserting-a-repo-wide-claim` — *"Pair the sweep with a positive
  and a cross-tree control, and enumerate then classify"*.
- `driving-when-a-leafs-place-is-in-doubt` — the three-way sentence→verb mapping,
  plus *"Name which of the three sentences is true before reaching for the CLI"*.
- `skill-session-name` — *"suggest `/rename …` once per session and move on"*.
- `skill-commit` — *"This is why Retire comes first"*: an ordering constraint,
  which may be a condition on the commit boundary rather than a step.

## (2) The two rules the guaranteed core sheds

The core carries the conditions a session must not miss; these two ride the
corpus instead, and three children have edited around them since they were
written. **Confirm both are still stated, and stated forcefully** — this is a
review question, not a test:

- **The driver's pick is authoritative and must not be re-walked.** Currently
  `skill-do-not-pick-again`: `grove-llm pick` stays a diagnostic, a second walk
  can disagree with the mandate, and the mandate wins.
- **The driver's stated version control is definitive** and is not re-derived from
  the working tree or from a harness banner. Currently
  `skill-stated-vcs-is-definitive`.

## (3) The size overshoot, which arrives as evidence for (1)

The brief predicted this exact shape: *"If the file lands materially above that,
the condition lines are still carrying prose that belongs in `references/` —
which is the no-procedure finding, arriving as a size symptom."* It landed there.

| measure | actual | target / bound |
|---|---|---|
| body lines | 404 | ~200 target, ≤500 bound |
| file bytes | 25,811 | ~8 KiB target |
| body lines, less the 54 marker/licence comment lines that die at cutover | ~350 | 1.75× target |
| bytes, less the 5,326 bytes of markers | ~20,485 | 2.5× target |
| triggering units | 51 | — |
| prose lines (body less markers less blanks) | 287 | — |
| **per condition** | **~5.6 lines, ~400 bytes** | **~1 line** |

**Both bounds pass and the target is missed by a factor.** Treat the last row as
the question: is ~5.6 lines per condition prose that *contains* a condition —
which is precisely what the rewrite was supposed to convert — or is it the
irreducible length of the condition itself? Answer it per section, in (1), rather
than in aggregate; an average is not a finding.

The producer deliberately did not trim: the four sibling children wrote these
condition lines and are `DONE`, so trimming them belongs to this review's
integration step, not to the producer's own session.

## Done when

- Every section of `content/SKILL.md` is classified condition-or-procedure, with
  the reference file it routes to named as the evidence for each call.
- The findings are recorded **as findings, not as test results**, with no budget
  test cited as evidence for the no-procedure limb.
- Both shed rules are confirmed present, or their absence is a finding.
- If anything is worth acting on, cut `integrate-review-impl` with the same bare
  stem `skill-opening`, using `leaf-insert` if any live sibling would otherwise
  run between it and this leaf — findings anchored to `path:line` drift silently.
  A review that finds nothing creates nothing and simply retires.

# model-led-development-k96

## Goal

Read `model-led-development-k94`'s distillation adversarially. It rewrote a
shipped skill on evidence that **falsifies two rules that skill was teaching**,
and nothing has read it since: the producer's in-session review allowance could
not be spent, because this session's harness carried a standing user instruction
forbidding subagents. So this leaf is the *first* adversarial read of the work,
not a second opinion on one.

**Reviews:** `model-led-development-k94`. Read that commit's diff against the
current source.

## Context

- [`docs/candidate-lessons.md`](../docs/candidate-lessons.md) §§1–5 — **the
  authority**. Every rule k94 landed from the second campaign is supposed to be
  the adjudicated form, not the producing session's own wording. Two of those
  wordings were falsified and one materially weakened; a rule that reads like the
  original claim is a defect.
- [`docs/formalism-findings.md`](../docs/formalism-findings.md), §*Distillation,
  second pass* (at the end of the file) — k94's own account of where each entry
  landed and what did not survive. It is the thing to attack: a row claiming an
  entry contributed X is checkable against that entry.
- The skill itself, and
  [`plugins/linkuistics/skills/authoring-conventions/SKILL.md`](../plugins/linkuistics/skills/authoring-conventions/SKILL.md)
  for the house rules the additions have to meet.
- [`docs/adr/grove-binds-without-the-plugin.md`](../docs/adr/grove-binds-without-the-plugin.md)
  — the governance question k94 answered *defers cleanly, no row moves*.

## Done when

Each of these has been checked at the source rather than reasoned about, and
every finding is recorded whether or not it is actionable.

- **The three new citation key forms resolve.** `[synthesis]`, `[c1]`–`[c5]` and
  `[045]` are defined in the provenance block; every use of one should land on
  text that says what the rule says. **`[045]` is the highest-risk of the three**,
  because k94 checked that entries 026–043 contribute only through `[c1]`–`[c5]`
  but attributed four rules directly to entry 045 — including two figures
  (`(1/k)^20`; 373 s at 3 invariants against 377 s at 61) that nothing else in
  the corpus repeats.
- **No rule k94 landed re-states a claim the adjudication weakened.** In
  particular: does anything left in the skill still read as *false greens are
  found by narrowing, not widening*? k94 says it retired the framing rather than
  softening it. The `[003]` sampling notes and the *Satisfied by construction*
  trap sit near the replacement and were not rewritten.
- **The two new *Six traps* rows carry what makes them traps.** The
  self-certifying row is supposed to carry its recursion — the repair recreating
  the hazard — inside the row, because a reader who lands the repair and stops is
  the failure the row exists to prevent.
- **The distillation rows are true of their entries.** k94 found and corrected one
  misattribution in its own table (a `FOCUS = 4` control credited to entry 044,
  which is in no entry at all) and one unverifiable count it had written
  (*eleven of fifteen*). Assume there are more. The *what did not survive* column
  is where to look hardest: a considered omission that is really a silent one
  reads identically.
- **The four additions outside §3A are each defensible, or are named as not.**
  k94 landed four rules `results-of-formal-methods-trial.md` §3A does not ask
  for, and listed them as such. The question is not whether they were declared —
  they were — but whether each has evidence behind it. **The fourth is the
  weakest**: *the sessions that check a model are the ones that stop appending*
  rests on the shape of the log being distilled rather than on anything inside
  it, which is a different kind of evidence from every other rule in the skill.
- **The governance verdict survives a second reading.** k94 answered *defers
  cleanly* on two checks: `content/` cites this skill nowhere, and Grove runs no
  model suites. Both are cheap to re-run. `cargo test --test plugin_fallback
  --test rule_ownership` and `plugins/install.test.sh` were green at k94's
  commit; a finding here is a rule that *does* change what a Grove session
  writes, not a test that fails.
- **The skill still reads as one document.** It grew from 284 to ~449 lines
  across seven sections, and the house threshold is a body under ~500. Whether
  the routing economics now overwhelm *How much will a model be worth to this
  work?*, and whether the additions to *What this evidence does not support* have
  turned a five-bullet section into a nine-bullet one that nobody finishes, are
  judgements this leaf is entitled to make.

## Notes

**Do not re-measure and do not re-run a model.** Same constraint k94 worked
under: the adjudication re-opened every model file it cites, and a citation that
looks wrong is a finding to record rather than a re-derivation to perform.

**Two things k94 deliberately did not land**, so that declining them again is
cheap and reversing them is informed: entry 048's *replay is an instrument, not a
comparison step*, on the grounds that a reader with one model has no second
column to replay into; and entry 047's tool-nondeterminism incident (the same
command at the same budget returning `[ok]` on one run and not the next), on the
grounds that one observation at one pinned version is a bug report rather than
routing evidence. Both are recorded in the distillation's *what did not survive*
column with those reasons.

**If this review finds something worth acting on, its last act is
`grove-llm leaf-add . model-led-development --kind integrate-review-impl`,** and
it writes that leaf's body carrying **this review's handle** — not its findings
verbatim. That is `methodology-changes-k91`'s repair of the defect this very
campaign measured: an integration whose charter *is* the finding list has no
structural place to reject one, and 45 of 45 tree-level findings survived here
because of it.

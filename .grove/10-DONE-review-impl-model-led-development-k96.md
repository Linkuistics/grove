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

## Findings

### 1. High — the new log rule is supported by a false account of the log and leaves the old rule standing

`references/keeping-a-log.md:26-27` still states the old obligation — every
session that *reaches for* a formalism appends — while `:106-110` later says the
repair is to state the obligation over the work, including every session that
*runs* one. A reader can follow the first complete rule and never reach the
replacement, so the adjudicated strengthening did not replace the weaker form.

The evidence offered for the replacement is also false as written. Lines 89-91
say every entry in the second half is one producer entry per model column, but
entry 047 explicitly says it is an experiment rather than a column, and entry
048 explicitly says its subject is the pair and that it replays the two green
columns (`docs/formalism-findings.md:8236-8243`, `:8522-8538`). The second-pass
distillation repeats the same false classification at `:10323-10342`.

The narrower observation is supported: the later scope, review, integration and
adjudication leaves named there appended no new entry, and two load-bearing
findings had to be reconstructed. Replace the old rule once, and state the
measurement over those observed session kinds rather than claiming every entry
044-048 was a model producer's.

### 2. High — the distillation table says entry 044 material landed when none of it appears in the skill

The entry-044 row at `docs/formalism-findings.md:10348` says the skill carries
044's two derived tests as quoted exceptions and its `ONE_SNAPSHOT` /
`mutant_two_listings` worked example. A complete read of `SKILL.md`,
`references/keeping-a-log.md`, and `references/routing-table.md` finds no
`ONE_SNAPSHOT`, no `mutant_two_listings`, no `[044]` citation, and no quotation
of the two tests. The routing-economics addition states only the aggregate
four-in-five result.

This makes the provenance artifact assert a landing that did not happen, in the
same section whose purpose is to distinguish considered omission from silent
omission. Either land the claimed entry-044 material with `[044]`, or correct
the row to say it did not survive and why.

### 3. Medium — the provenance block overstates the campaign's coverage

`SKILL.md:20-24` says the 129 obligations were "checked in both families" across
258 cells. The synthesis records 256 complete cells and two declared Alloy gaps,
`TT-24.c` and `TT-24.d` (`docs/formalism-findings.md:9437-9453`). The run was
green because declared gaps are honest, not because those two cells were
checked. Say the obligations were *matrixed* across both families, or state the
256-complete / 2-gap result in the provenance block.

### 4. Medium — the skill routes two-formalism comparisons but omits the instrument that produced their unique evidence

`references/routing-table.md:46` adds an explicit route for "comparing two
behavioural formalisms", yet the entry-048 distillation row deliberately omits
*replay is an instrument* on the ground that a reader with one model has no
second column (`docs/formalism-findings.md:10352`). That ground cannot apply to
the new route: a reader asking its question necessarily has two formalisms.

Entry 048 says replay found things neither column's reviewer could, and the
synthesis's combined workflow makes independent construction followed by one
replay its second step. Add replay to the comparison route, conditioned on two
families existing, or record a reason that addresses that actual trigger.

### 5. Medium — the `[045]` timing pair does not establish that depth is what costs

`SKILL.md:243-245` and `references/routing-table.md:106` infer "depth is what
costs, not the number of invariants" from 3 invariants at depth 3 taking 373 s
and 61 invariants at depth 4 taking 377 s. That comparison changes both operands
at once, and its nearly identical times establish only that adding 58 invariants
had little marginal cost in those two runs. It does not isolate depth as the
cost driver. Entry 045 also records 445 s after three constants widened the
state, reinforcing that the model encoding is another live operand.

Keep the well-supported rule to quote a result with its depth. Narrow the budget
rule to treating property count as low marginal cost in this run, or cite a
same-property-count comparison across depths before claiming depth dominance.

### 6. Low — the narrowed-world paragraph says five modules and enumerates four

`SKILL.md:255-260` says five modules lost reachability, then lists four shapes:
the foreign-marker environment, zero budget, `FOCUS = 4`, and the one-entry
scenario. Candidate 3's source table has five; the omitted case is
`mutant_correlation_wins_the_overlap`, whose crash-only environment could not
reach the in-transaction hand edit needed for the overlap. Add that fifth shape
or avoid presenting the following list as the enumeration of all five.

## Checks with no finding

- The three citation-key forms resolve, and the uses of `[c1]`-`[c5]` inspected
  here land on the adjudicated forms rather than the producers' superseded
  wording.
- The false *narrowing, not widening* slogan is retired explicitly; the
  replacement conditional carries both subset and reachability.
- Both new *Six traps* rows carry their operative mechanism, including the
  self-certifying repair's recursion.
- The governance verdict survives: `content/` contains no citation to
  `model-led-development`, its Linkuistics citations enumerate only
  `decision-records`, `codebase-design`, and `using-jujutsu`, and no Grove
  methodology file names a model runner, Quint, Alloy, or a model suite.
- The 449-line skill stays below the house body threshold, keeps references one
  level deep, and remains readable as one document. The longer routing-economics
  and evidence-limit sections did not produce a separate structural finding.
- The producer recorded `plugin_fallback`, `rule_ownership`, and
  `plugins/install.test.sh` green. This inspection-only review did not rerun
  them, as required by the review contract.

# Results of the formal-methods trial

A plain-language reading of what this repository's formal-modelling campaign
found, what it cost, and where the findings that have not yet been landed should
go. It was written by the grove's `finish` session, on the way to a teardown that
was then deliberately not taken.

**What this document is, and is not.** It re-measured nothing. It is a reading of
the four harvest documents — [`loop-record.md`](loop-record.md),
[`review-yield.md`](review-yield.md),
[`candidate-lessons.md`](candidate-lessons.md) and
[`driving-a-checkable-loop.md`](driving-a-checkable-loop.md) — plus
[`formalism-findings.md`](formalism-findings.md), the two `linkuistics` skills
that are candidate homes for the material, and the two decision records that
govern where a rule may live. Every figure below is one of those documents'; none
is new. Where this document adds something, it is §2, which is a finding about
*scope* rather than about models.

---

## 1 · What the campaign found, in plain language

**What it was.** Four days, 58 commits, 73 task files. It wrote executable models
of Grove's own protocols — how the task tree is edited, how a workstream is torn
down without losing work if the machine dies mid-operation — in two unrelated
modelling languages, and checked **129 obligations in both**: 258
`(family, obligation)` cells, roughly 30,000 lines of model, one invocation, 791
commands, exit 0.

**The one-sentence result.** *Green means nothing until something has been shown
able to turn it red — and even then, only if it turns red for the reason you
claim.* Everything else below is a variation on that sentence.

### The five things it kept finding in its own work

**1 · Empty greens are everywhere, and looking finds them.** Fifteen obligations,
plus three defects in the runner itself, were passing while checking nothing. Ten
different sessions went looking for one; **ten found one**. In the worst case a
model checker that had been *killed before it could start* — a 6 MB heap limit,
so it never read its own jar — was reported by the runner as **43 verified
properties**. Failure defaulting to success.

**2 · A control can kill a claim for the wrong reason, and it reads identically.**
Three named controls were found doing exactly that: one turning its obligation red
through a defect in the coverage sweep rather than through the behaviour it named;
one always firing through one half of a two-part claim because its world could not
reach the other half; one going red under both the old and the repaired wording,
*"which is evidence that the new operand was never exercised rather than that it
was controlled."* What establishes the link is the **isolating** run — check each
part of the claim separately over the same world, and show the others stay green.

**3 · A narrow world is the commonest way to manufacture a false green.** Five
modules were narrowed until the thing they were mutating could no longer happen.
The campaign's own slogan — *"false greens are found by narrowing, not
widening"* — is **falsified**, five to one against on the first half, and twice on
the second: a dead control was repaired by *widening* its bound and fired in 1.5
seconds, and a declared impossibility (an argument from a state-count estimate
that a check could not be run) was killed by simply running the deeper
attempt — the unreachable state was reached in **fourteen states, 8.9 seconds**.
What survives is a conditional: *a narrowed world is evidence about the wide one
only when the narrowing is a strict subset that still reaches the subject.* The
corpus's own compression, reached independently three times, is better: **a mutant
module's environment is part of its control.**

**4 · A claim checked against the system's own record of what it did cannot
fail.** The natural repair — have each step record the fact as it establishes
it — recreates the same hazard one level down, because the flags are now written
by exactly the steps the claim is about. **So every recorded fact owes a control
that can make a step omit or falsify it.** One node hit this four times: three
times in one claim's expression, and a fourth time in the repair of the first
three.

**5 · A check written by calling the definition it is about stops measuring the
moment that definition is repaired.** One witness went from reached in 3,410
traces to unreached in 8,000 samples the instant its subject was fixed — *"not
because the defect went away but because the sentence had come to mean something
else."* No error, no red, no signal at all; just a number that quietly begins
describing something different. It is the cheapest failure in the whole record to
miss.

### And five things it found about the loop that was driving it

**6 · The review chain had a structural defect, exposed by measuring it.** 45
findings raised across nine chains, 45 confirmed, **0 rejected** — because in five
of the nine, the session that "verified" the findings had its own task file
*written by the reviewer*. The charter **was** the finding list, so rejecting a
finding meant rejecting its own charter. Meanwhile the in-session channel, where
the reader owes the finding nothing, rejected **4 of 24 (17%)**, and one reviewer
falsified a finding the integrating session had raised itself. A 100% survival
rate was a broken instrument reading, not a good reviewer.

**7 · Spending a reviewer changed the session that spent it, three times out of
three — and none of the three was a second opinion.** One broke a disposition its
session had already landed, forcing a reversal across nine artifacts; one swept
the full claim library and found two violations, one nobody was looking for; one
made its session withdraw a sentence it had already written. **All three were a
different instrument**, which is the useful way to decide whether to spend one.
Declining is also a real answer: five sessions declined, for four different
reasons.

**8 · A recording obligation with no instrument behind it decays into whatever the
last entry looked like.** Eight measures were pre-registered, to be recorded
before each fix. **Two survived as specified; five were kept unevenly; one whole
column carries no tags at all** — and it decayed **exactly at the boundary where
one session handed off to the next**. Nothing detected it, because the runner
checks obligations, not ledger entries.

**9 · Most of the tree was discovered, not designed.** It opened with 23 task
files under 5 directories and closed with 73 under 16: **68% of the final tree was
created by sessions mid-flight**, and **every single review chain was grown, none
was planned**. 23% of task files were written and never run.

**10 · Neither formal tool was the most productive instrument in the campaign.**
Its own verdict names that as the pair **`(claim, the control that would falsify
it)`, read adversarially by a fresh context**. One such review found six real
defects in a suite that was green, coverage-asserted, and had already published
its own list of things it might be wrong about — and **four of the six were
invisible to any run of that suite**.

### What it cost, stated rather than hidden

- **About four in five material findings produced no executable test.** The value
  was *specification correction*, not defect detection — so the phase that
  consumes a model of this kind is **documentation, not implementation**. That
  phase never ran.
- **Where hours were kept**: 0.289 h per obligation component-scoped, 0.373 h
  system-scoped; ~4.2 h lost to tooling rather than to the problem.
- **Machine time is not a tool comparison.** Alloy's task-tree column cost 6888 s
  of CPU for 103 commands against Quint's 1209 s for 111 — but only the Alloy file
  carries integers, and bitwidth arithmetic dominates it. **Run cost tracks the
  data structure in the model, not the family.**
- **The most expensive question is permanently unanswered**, and 10,366
  lines — 34% of `src/`, including 31 `unsafe` blocks, plus 6,701 lines of
  test — stand on a deferred verdict.
- **There is no token or money figure anywhere in the record.** Every cost above
  is time, commits, lines or commands. Anyone repeating this should instrument
  spend on day one; it is the one column that cannot be recovered afterwards.

---

## 2 · The gap: the binding test was applied to one corpus

[`driving-a-checkable-loop.md`](driving-a-checkable-loop.md) declines five of
these lessons on a rule that is correct as stated: **a lesson binds only when
there is a session that could violate it**, and Grove's methodology runs no model
suites. [`candidate-lessons.md`](candidate-lessons.md) §*What binds* puts it
directly: *"Almost all of it goes to the write-up, because Grove's methodology
does not run model suites."*

That test was applied against **one corpus** — `content/`, the methodology the
`grove` binary embeds. It was never applied against the other one.

**`linkuistics:model-led-development` is a corpus whose sessions do run model
suites** — that is its entire subject — and it is developed in this repository, at
`plugins/linkuistics/skills/`. The string `linkuistics` appears in **none** of the
four harvest documents.

Two further facts make the gap concrete rather than theoretical:

- **That skill is already a distillation of `formalism-findings.md` — but only of
  its first 25 entries**, performed by `formalism-skill-k38` in an *earlier*
  grove. The file's §*Distillation — where each entry landed* records that, entry
  by entry.
- **The log's second half, roughly 3,400 lines, is this campaign's, and has no
  distillation row and no skill landing.** The file's own header states the
  arrangement it is now one half short of: *"accumulate here, distil later… A
  later leaf turns the whole log into a skill."* That leaf was cut in the previous
  grove and not in this one.

The consequence is that the skill currently teaches the **weaker** form of rules
this campaign falsified or sharpened:

| what the skill says today | what this campaign measured |
|---|---|
| "Mutation controls — break the implementation and watch the tests go red" | going red is not enough; the control must be shown to go red **for the reason claimed**, by the isolating run |
| "Small scopes lie quietly" (a caution) | narrowing is the **dominant manufacturing route** for false greens — five modules — and what survives is a stated conditional, not a caution |
| "A coverage report needs one mutation per reported obligation" | a **two-direction** coverage assertion still passed while crediting a cell from a world in which the obligation was false |
| four modelling traps | two more are evidenced: the self-certifying claim with its recursion, and the self-calling witness |
| provenance: "a 25-entry log" | the log is now about twice that, and the second half has never been distilled |

**The left column is the skill as it stood when this was written, and it no
longer stands.** `model-led-development-k94` ran §3A's distillation: the log's
second half now has a *Distillation, second pass* section, and every row above
has moved — the isolating control rule, the narrowing conditional, six traps, the
module sweep with its cost, the corrected routing economics, and a provenance
block spanning both campaigns. That leaf also landed four rules §3A does not
name, and records each as such. The table is kept in its original tense because
it is the statement of the gap; *Distillation, second pass* is the account of
closing it.

**This is not a defect in the harvest.** The harvest asked what binds on Grove,
answered that correctly, and shipped three rules into `content/`. It is a gap in
scope, and it is what §3 exists to close.

---

## 3 · Where the remaining findings should land

### A · `model-led-development` — the substantial one

Run as a **distillation leaf against the log**, the way `formalism-skill-k38` was
run for entries 001–025 — including writing the *Distillation — where each entry
landed* rows for the new material, because that table is what lets a later reader
check a rule instead of trusting it.

- **Strengthen the mutation-controls bullet** to the isolating form, with the
  three wrong-mechanism controls as its evidence.
- **Replace the scope caution with the conditional**, and land *a mutant module's
  environment is part of its control*. Retire the falsified
  *narrowing-not-widening* framing rather than softening it, and add the rule the
  catalogue gained: **a modelling language's failure to meet a catalogue row is
  established by running the deeper attempt, never by costing it in prose.**
- **Add two rows to the *Four traps* table** — the self-certifying claim, carrying
  its recursion; and the self-calling witness.
- **Add the module-sweep rule**: run every module against every claim the model
  has, not the ones it declares — with the cost the original claim omitted, that
  the obligation is to *run the sweep and record what it says*, not to fix
  everything it finds in the same session.
- **Correct the routing economics** in *How much will a model be worth to this
  work?*: ~4 in 5 findings yield no test, so the consumer of a model is
  documentation rather than implementation. Add the per-obligation hours, the
  ~4.2 h tooling overhead, and the caution that run cost tracks the data
  structure, not the tool family.
- **Extend *What this evidence does not support*** with the new limits — in
  particular that the corpus never ran an arm without controls, so it measured the
  hit rate of *looking* (10 for 10) and never how much rot there was.
- **`references/keeping-a-log.md` should carry finding 8.** That file instructs a
  six-field log; this campaign measured what becomes of that instruction, and the
  measurement belongs beside it.

### B · `doubt-driven-development` — smaller, sharper

The skill already gets bias control right in one direction (*never pass the CLAIM
to the reviewer*). Finding 6 is the **mirror image, and it is unstated**:

> **Never let the reviewer author the reconciler's contract.** A reader graded
> against a finding list it did not write can reject a finding; a reader whose
> charter *is* the finding list cannot.

Also worth landing: **spend a reviewer when it is a different instrument, not a
second opinion** (finding 7), and **declining is a real answer**, with the four
evidenced reasons. Two cautions belong on its own count-based *doubt theater*
signal: findings-per-review fell 6.2 → 3.5 here and that was a **selection effect
of a rising review rate, not decay**; and without a severity scale fixed *before*
the reviewer starts, counting findings measures enumeration style — sub-corrections
per finding ran 1.0 to 3.3. Finally, the **relay caveat**: the three readers who
each found what the previous two missed were **not independent** — each wrote its
doubt into the next one's charter, and a chain of three *unaimed* readers is a
different experiment this workstream never ran.

**Landed by `doubt-driven-development-k95`, and two of the sentences above did
not survive contact with the record.** The contract rule, its residue, the
different-instrument test with the four evidenced declines, the counting cautions
and the relay caveat are all in the skill; the Grove alignment is a citation to
`references/decompose.md` and `references/integrate-review.md` rather than a
restatement, as §C reads it. What changed on the way in: **the 6.2 → 3.5
sentence above overstates its source** — `review-yield.md`'s verdict is that the
gap *cannot be read* as decay at all (≈1.8σ, unmatched subjects, rising review
rate), of which the selection effect is one reason of three, so the skill carries
all three rather than the mechanism alone; and **the relay result is weakened
twice, not once** — it is also one node, selected for review a third time
because it was the hardest. The skill also separates the two channels' counting
instruments, which this section's figures silently merge: the tree-level counts
are derived, the in-session ones are a hand count.

### C · The governance constraint on doing either

[`grove-binds-without-the-plugin`](adr/grove-binds-without-the-plugin.md) makes
the direction of promotion consequential. Grove ships without the plugin, so every
citation must state what binds in its absence, under one question: *does the
absence change what a session writes, or only how well?* A rule Grove needs must
be stated locally, recorded in `docs/specs/corpus-rule-ownership.md`'s plugin
deferral policy, with `tests/plugin_fallback.rs` asserting the citation carries
its fallback sentence.

The reading this document offers, to be checked rather than assumed by the leaves
that do the work: **cluster A defers cleanly** — Grove genuinely runs no model
suites, so the plugin's absence changes nothing Grove writes — while the
doubt-driven bias rule is **already owned locally** as `content/`'s rule C, so
there the skill should align with and cite Grove rather than restate it.

---

## What this document does not establish

- **Nothing here was re-measured**, and no model was re-run. What replaces an
  instrument is the citation.
- **The scope finding in §2 is about placement, not about evidence.** Whether each
  rule survives contact with the skill's own house style, and whether the isolating
  form generalises beyond this corpus, is the distillation leaf's to decide.
- **The limits of the underlying evidence are the harvest's**, and they carry
  forward unchanged: one workstream, one subject, one operator, in a repository
  whose entire purpose was rigour about evidence. **The hit rates are hit rates
  here.**

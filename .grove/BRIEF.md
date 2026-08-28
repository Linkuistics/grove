# grove.refactor-for-modularity — brief

## Goal

Codify what this repository's formal-modelling campaign taught about **driving an
LLM loop to produce checkable work**, and land it in two places: a written account
that stands on the campaign's own evidence, and concrete changes to the grove
methodology and its skill so the lessons bind the next loop rather than
describing the last one.

**The refactor this grove was named for is over.** It ran a formal phase —
executable Quint and Alloy 6 models of the task tree, the finish/recovery
protocol and the end-to-end lifecycle, against a catalogue of 130 obligations
with a runner that asserts coverage in both directions — and it did not reach the
documentation or implementation phases. That is a deliberate stop, recorded
below, and the campaign's transferable output is the method rather than the crate
split it was commissioned to justify.

## Done when

- The campaign's **evidence base is harvested into durable artifacts**, and in
  particular `.grove/` — the only record of how the loop itself behaved — is
  mined before it can be deleted. Every claim in the harvest cites where it is
  measured, and a claim that survives only as a recollection is dropped.
- The lessons that survive the harvest are stated as **claims with the evidence
  that supports them and the conditions under which they would be wrong**, not as
  advice. A lesson nothing in this repository can be pointed at for is not one.
- **Every lesson that can bind lands as a change to the grove methodology or its
  skill** — session-kind guidance, review-chain policy, what a session must
  verify before claiming a result, what a self-checking runner owes. What cannot
  bind lands in the write-up, and the write-up says which is which.
- The write-up is readable by someone who has never seen this repository, and it
  is honest about cost: what the campaign spent, what it returned, and which
  parts of it were not worth the money.
- The methodology changes are shipped the way any grove change is: the embedded
  `content/` payload, its hash, and the tests that read it stay consistent, and
  nothing in the preservation ledger below is broken without an explicit
  exception.

## The refactor is stopped — human decision, 2026-08-28

**Three phases were abandoned, in two steps, on explicit instruction.** First
`documentation-k2` and `implementation-k3`; then, on the judgement that its
remaining leaves would return nothing generally applicable, the tail of
`formal-modeling-k1` — `sweep-ownership-k81`, `alloy-candidate-k82`,
`q1-q4-verdict-k83`, `quarantine-gate-control-k86` and `handoff-audit-k66`. The
reason throughout was cost against value, not a finding that the work was wrong.

**What that leaves standing, and it is a lot.** Green Quint and Alloy 6 models
with their assumptions, controls, bounds and retained counterexamples; a
semantic contract of 130 obligations; `docs/formalism-findings.md`'s bounded
comparison of the two families; a decision set in `docs/adr/`; and a runner whose
four self-checking obligations are themselves a method artifact. None of it is
deleted, and the lessons work reads all of it.

**What it costs, stated rather than hidden.** `TODO.finish_process.md`'s Q1 and
Q4's three cleanup rows are now **permanently deferred** —
[`finish-keeps-a-cleanup-layer-it-has-not-proved-forced`](../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md)
records that, why, and what would reopen them. The Rust crate split, the
current-state documentation and the legacy-migration removal are not owed by
anything and will not be built from this tree. The Alloy column keeps a repair it
was owed and never received, named in
`crates/grove-finish/models/README.md`.

## Decomposition

**Lazy, and only the first leaf is cut.** The shape below is intent, not a tree:
each leaf cuts what follows it once it knows what that is.

1. **`harvest-the-loop-record`** — the evidence base, and it is first because one
   input is perishable. `.grove/` is process state that grove deletes at finish,
   and it is the only record of which sessions decomposed, which cut reviews,
   what each review caught and what each integration caught while applying it.
   Everything else can wait; this cannot.
2. **The lesson leaves** — one per cluster that survives the harvest, each
   producing a durable record. Cut by leaf 1, which is the session that knows how
   many clusters there are.
3. **The methodology changes** — what binds, landed in `content/` and the skill.
   Separate from the write-up because it is product change with tests and a hash.
4. **The write-up** — what cannot bind, plus the cost account.
5. **The `linkuistics` promotion** — cut by the `finish` session, which found
   that the binding test had been applied to one corpus only. One leaf per skill:
   `model-led-development-k94` for the model-suite cluster, and
   `doubt-driven-development-k95` for the review-loop findings.

## Pointers

- **The harvest, in three durable documents under `docs/` — read these rather
  than `.grove/`.** [`loop-record.md`](../docs/loop-record.md) enumerates every
  session with its kind, node, outcome and commit, plus the cost account;
  [`review-yield.md`](../docs/review-yield.md) opens all eighteen review-chain
  bodies and answers the yield question; and
  [`candidate-lessons.md`](../docs/candidate-lessons.md) adjudicates the six
  candidates below, three of which moved. The first two are derived by scripts
  that read `.grove/` **at a pinned revision**, so they re-derive after teardown.
- `.grove/` — the session tree, its briefs, its retired task bodies and its
  abandonment marks. Still the primary source, and still perishable; but every
  claim the lessons workstream rests on has been lifted out of it, so a session
  reads it now to check a citation rather than to avoid losing one.
- `docs/formalism-findings.md` — the bounded Alloy-6 vs Quint comparison, with
  per-entry findings, costs, false confidence and retained counterexamples.
- `crates/*/models/README.md` — mutation matrices, run tables with wall times and
  command counts, declared narrowings, and each column's *what a green run does
  not prove*.
- `docs/specs/semantic-contract.md` and `models/run.sh` — the obligation
  manifest, and a runner that asserts coverage in both directions, reports
  contested cells, and fails on zero work.
- The `docs/adr/` records that are **methodological rather than about Grove**:
  [`a-shared-safety-claim-names-the-role-not-the-artifact`](../docs/adr/a-shared-safety-claim-names-the-role-not-the-artifact.md),
  [`a-lifecycle-claim-says-what-it-is-over`](../docs/adr/a-lifecycle-claim-says-what-it-is-over.md),
  [`a-closed-partition-is-over-outcomes-not-states`](../docs/adr/a-closed-partition-is-over-outcomes-not-states.md),
  [`obligations-follow-context-not-artifact`](../docs/adr/obligations-follow-context-not-artifact.md).
- `content/` and the grove skill — where a lesson that binds has to land.

## Notes

### Candidate lessons, to be tested by the harvest rather than assumed

These are what the campaign looked like from inside it. Each is a claim the
harvest must either evidence or drop.

- **A green suite is not evidence; only a control that can kill the claim is.**
  One node met four claims that were true by construction and green.
- **A claim stated over the model's own history or classifier is
  self-certifying**, and the repair — record the fact at the step that
  establishes it — recreates the hazard one level down, so each recorded fact
  owes a control that makes a step omit it.
- **False greens are found by narrowing, not widening.** A shared-safety claim
  was violated in a strict *subset* of the widest world's traces, which that
  world's 8000 samples had never drawn. A property checked in the widest world is
  not therefore checked hardest; a wide environment dilutes the sampler.
- **A module that changes what the model does must be run against every claim the
  model has**, not the ones it declares — the module rule hides failing claims by
  design.
- **Measure, freeze, then repair.** A predicate is a subject too: a witness
  written by calling the definition it is about stops measuring the moment that
  definition is fixed.
- **Review yield did not decay.** A producer's own reviewer found three
  substantive defects in a green suite; the review leaf beside it found five more
  in the repairs; the integration found two more while applying them. This is the
  one candidate lesson that is about the loop rather than about models, and it is
  the one most worth checking hardest.

### The binding test was corpus-scoped, and that was found at finish

**"A lesson binds only where a session could violate it" is right, and it was
applied to `content/` alone.** `linkuistics:model-led-development` is a corpus
whose sessions *do* run model suites, it is developed in this repository at
`plugins/linkuistics/`, and the string `linkuistics` appears in none of the four
harvest documents. So four of the five lessons the write-up files under *could
not bind* have a possible violator after all — and the skill currently teaches
the weaker form of two rules this campaign falsified.

That is a gap in scope rather than a defect in the harvest, which asked what
binds on Grove and answered it correctly. It is recorded in
[`docs/results-of-formal-methods-trial.md`](../docs/results-of-formal-methods-trial.md)
§2 — a durable document, because this brief dies at teardown — and discharged by
the two leaves in step 5. **Their governance constraint is
[`grove-binds-without-the-plugin`](../docs/adr/grove-binds-without-the-plugin.md)**:
a rule whose absence changes what a session *writes* stays Grove-owned, and only
what changes *how well* may defer to the plugin.

### Preservation ledger

Grove still ships, and the methodology changes are product change. Preserve
unless a change explicitly records an approved exception:

- CLI verb names, arguments, help shape, structured/human output fields, and exit-status meanings.
- Configuration keys, environment overrides, defaults, and the current `session-kinds-v1` `.grove` format.
- Abstract outcomes across Git, native jj, and colocated jj workspaces.
- Methodology embedding/provisioning, package and binary names, release/install behaviour, MSRV 1.85, and the Linux glibc 2.17 compatibility target.
- Fail-closed ownership: Grove never resets, merges, deletes, or rewrites work it cannot prove belongs to the current finish attempt.

The legacy-migration removal that this grove approved was **never implemented**,
and is not owed: `implementation-k3` is abandoned. Migration commands and
compatibility paths stand as they are.

### `.grove/` is process state, and that is now a hazard rather than a rule

Every durable finding must still be promoted into models, tests, documentation or
decision records before its task retires. But this grove's own history is now
*subject matter*, so the usual disposability cuts the wrong way: **nothing may run
`finish` until `harvest-the-loop-record` has landed**, because finish deletes the
evidence.

**It has landed** — `harvest-the-loop-record-k87` closed with
`candidate-lessons-k90`, and the evidence is in the three `docs/` documents named
under Pointers. What discharges the hazard is not that the documents were written
but that **the two derivations are pinned to a change id and re-run after
teardown**: finish removes `.grove/` from the tip, never from history. So the
embargo's reason is met. The remaining leaves consume `docs/`, not `.grove/`, and
whether the grove is finished is the driver's call and not a leaf's — this note
records that the condition is satisfied, not that anything should now be torn
down.

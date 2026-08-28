# model-led-development-k94

## Goal

Distil this campaign's half of `docs/formalism-findings.md` into
`plugins/linkuistics/skills/model-led-development/`, the way
`formalism-skill-k38` distilled entries 001–025 in the previous grove — including
the *Distillation — where each entry landed* rows for the new material, without
which a later reader has to trust a rule rather than check it.

**Why this leaf exists at all.** The lessons workstream declined this whole
cluster on a correct rule — *a lesson binds only where a session could violate
it, and Grove runs no model suites* — applied to **one corpus**, `content/`.
`model-led-development` is a corpus whose sessions do run model suites, it is
developed in this repository, and the string `linkuistics` appears in none of the
four harvest documents. See
[`docs/results-of-formal-methods-trial.md`](../docs/results-of-formal-methods-trial.md)
§2, which is where that gap is recorded.

## Context

Read in this order:

- [`docs/results-of-formal-methods-trial.md`](../docs/results-of-formal-methods-trial.md)
  §1 and §3A — the plain-language findings and the proposed landing sites. **§3A
  is a proposal, not a specification**: check each rule against the skill's own
  house style before landing it.
- [`docs/candidate-lessons.md`](../docs/candidate-lessons.md) §§1–5 — the
  adjudicated form of every claim below, each with the model file that decides it
  and what would have falsified it. **This is the authority**, not the
  results document, which merely reads it.
- [`docs/formalism-findings.md`](../docs/formalism-findings.md) — the log. Its
  §*Distillation — where each entry landed* (line ~6839) is the format to follow
  and the record of what the first distillation carried. The new material runs
  from §*What is being compared, and what is not* (~9155) to the end.
- The skill as it stands, and
  [`plugins/linkuistics/skills/authoring-conventions/SKILL.md`](../plugins/linkuistics/skills/authoring-conventions/SKILL.md)
  — the house description-shape rule and progressive-disclosure thresholds.
- [`docs/adr/grove-binds-without-the-plugin.md`](../docs/adr/grove-binds-without-the-plugin.md)
  — the governance constraint, and `docs/specs/corpus-rule-ownership.md`'s plugin
  deferral policy with `tests/plugin_fallback.rs`.

## Done when

- **The isolating form of the control rule has landed**, replacing the weaker
  "break it and watch it go red": a control must be shown to kill the claim *for
  the reason claimed*, established by running each part of the claim as its own
  check over the same world. Evidence: three named wrong-mechanism controls.
- **The scope caution has become the conditional** — *a narrowed world is evidence
  about the wide one only when the narrowing is a strict subset that still reaches
  the subject* — carrying the corpus's own compression, **a mutant module's
  environment is part of its control**. The falsified *narrowing-not-widening*
  framing is **retired, not softened**, and the catalogue's rule is stated: a
  modelling language's failure to meet a row is established by running the deeper
  attempt, never by costing it in prose.
- **Two rows are added to the *Four traps* table** — the self-certifying claim,
  *carrying its recursion* (each recorded fact owes a control that can make a step
  omit it), and the self-calling witness.
- **The module-sweep rule has landed**, with the cost the original claim omitted:
  the obligation is to run the sweep and record what it says, not to repair
  everything it finds in the same session.
- **The routing economics are corrected** in *How much will a model be worth to
  this work?* — ~4 in 5 findings yield no executable test, so the consumer of such
  a model is documentation rather than implementation — plus the per-obligation
  hours, the ~4.2 h tooling overhead, and *run cost tracks the data structure in
  the model, not the family*.
- ***What this evidence does not support* carries the new limits**, in particular
  that no arm ever ran without controls, so what was measured is the hit rate of
  *looking* (10 for 10) and not how much rot there was.
- **`references/keeping-a-log.md` carries the decay measurement** — 2 of 8
  measures kept as specified, decaying at the session handoff boundary,
  undetected — because that file is what asks for the log.
- **The provenance block and `references/routing-table.md` are current**: the log
  is no longer 25 entries, and every new rule names the section that evidences it.
- **A *Distillation* section for the new material exists in the log**, in the
  existing format, including what did **not** survive and why — silent omission
  and considered omission look identical afterwards.
- **The governance question is answered explicitly**: for each rule landed,
  whether Grove needs a local statement under
  `grove-binds-without-the-plugin`. The expectation is that this whole cluster
  defers cleanly; record it rather than assume it, and if any row does bind on
  Grove, `docs/specs/corpus-rule-ownership.md` and `tests/plugin_fallback.rs` move
  with it.

## Notes

**This may prove bigger than one session.** If it does, decompose at the seam
between *the skill's own rules* and *the log's distillation table plus the
references*, and do only the first child — do not run long.

**A rule with no entry behind it is not written.** That is the skill's own
standing discipline, stated in its provenance block, and it applies to every
sentence this leaf adds.

**Do not re-measure.** Nothing in this cluster needs a model re-run; the
adjudication in `candidate-lessons.md` re-opened every model file it cites. If a
citation looks wrong, that is a finding to record, not a re-derivation to perform.

**Review is likely earned here** — this is a load-bearing corpus edit that
falsifies two rules a shipped skill currently teaches. Judge it at the end, on
`references/decompose.md`'s terms, rather than assuming it either way.

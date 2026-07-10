# task-kinds-model-selection-k6

**Kind:** planning

## Goal

Grill and decide: enrich grove's task-kind taxonomy beyond the current
planning/work binary — the user likes upstream wayfinder's richer typing
(research / prototype / grilling / task) — and wire the enriched kinds into
**per-kind model selection**.

## Context

- Today: `TASK-FORMAT.md` defines two kinds; `grove-llm kind` peeks a leaf's
  kind; the loop driver reads `GROVE_PLANNING_MODEL` / `GROVE_WORK_MODEL` and
  passes `--model` only if set (glossary "Per-kind model selection"; ADR
  `model-per-task-kind`).
- Upstream wayfinder types tickets four ways; grove analogues would be
  something like planning(grilling) / research / prototype(spike) / work.
  Motivating use: research leaves and mechanical work leaves may want
  different models than grilling-heavy planning leaves.
- Tension flagged in the report: a fixed taxonomy fights constraint 3
  ("suggested shape, not enforced schema"). Kinds must stay *suggested* —
  the question is what the CLI does with an unknown/absent kind.
- **Sequenced after k5** (issues-substrate brainstorm): where `kind` lives —
  task-file line vs tracker label — depends on the substrate decision.

## Questions to grill

1. Which kinds earn their place? (Each needs a real behavioural difference —
   model choice, launcher prompt, or methodology guidance — or it's taxonomy
   for its own sake.)
2. Env-var scheme: `GROVE_<KIND>_MODEL` generalisation? Fallback chain for
   unknown kinds (unknown → work? → unset ⇒ inherit)? Never clobber
   `ANTHROPIC_MODEL` (existing rule).
3. What changes where: `TASK-FORMAT.md`, `grove-llm kind`, loop driver,
   ADR `model-per-task-kind` (edit in place), glossary entry, `leaf-add`
   defaults?

## Done when

Decisions recorded; ADR `model-per-task-kind` reworked in place to state the
enriched scheme (or the decision to stay binary, with why); implementing work
leaf grown for the CLI changes.

## Notes

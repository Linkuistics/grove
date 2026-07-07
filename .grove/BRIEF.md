# use-different-models-for-planning-vs-execution — brief

## Goal

Let a grove run **planning** and **work** tasks on different LLM models — e.g. a
stronger reasoning model for grilling/design, a cheaper/faster one for mechanical
work — so the self-driving loop launches each task's `claude` session on a model
matched to the picked leaf's kind.

## Done when

- The loop driver selects each session's launch model by the leaf's `Kind`, via
  Claude Code's native `--model`, configured by `GROVE_PLANNING_MODEL` /
  `GROVE_WORK_MODEL` (unset ⇒ no `--model`, unchanged behavior).
- Backward compatible; documented; verified end-to-end.

## Decomposition

Planning (`plan-k1`, DONE) grilled the design and produced ADR
*model-per-task-kind* + the **Per-kind model selection** glossary term. The
implementation is three sequential work leaves:

1. `grove-llm-kind-verb-k2` — the `grove-llm kind [<leaf>]` primitive.
2. `driver-model-selection-k3` — `Harness.model_args` + loop-driver selection
   (start⇒planning, continue⇒peek-kind), consumes k2.
3. `document-model-selection-k4` — user docs (env vars, `/model`-non-persistence
   caveat) + end-to-end verification.

## Pointers

- ADR: `docs/adr/model-per-task-kind.md` (extends `self-driving-loop.md`).
- Glossary: `CONTEXT.md` → **Per-kind model selection**.
- Key code: `src/loop_driver.rs`, `src/harness.rs`, `src/leaf.rs`,
  `src/llm_cli.rs`, `src/cli.rs`.

## Notes

- No model **router** — native `--model` on the (Max) subscription; a proxy would
  risk breaking subscription billing. Mid-session `/model` (native, highest
  priority) covers ad-hoc switches but does not persist across relaunch.
- Two buckets only (keyed on `Kind`); per-leaf `**Model:**` override is a clean
  v2 follow-up if it earns its place.

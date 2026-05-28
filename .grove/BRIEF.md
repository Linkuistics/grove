# grove-status-should-show-active-grove-grove-versions — brief

## Goal

Refine `grove status` so its output usefully answers two questions the current
output does not:

1. **Which grove(s) are *active*** — distinguishing the workstreams a human
   actually has open right now from the full historical list of worktrees
   under `.grove-worktrees/`.
2. **Which "grove version" is each thing on** — clarifying the version story
   across the grove CLI binary, the materialised methodology per harness, and
   (potentially) the methodology version a running grove was *started* with
   vs. the current install.

The working title's exact meaning of "active" and "versions" is unresolved —
that is what the first planning leaf grills out.

## Done when

- `grove status` distinguishes active from inactive groves (definition agreed
  during grilling).
- `grove status` surfaces the version(s) the user agrees are load-bearing.
- The CLI reference in `README.md` and any relevant lifecycle walkthrough
  under `docs/workflows/` reflect the new output.

## Decomposition

Not yet decomposed. `010-shape-the-feature.md` is a planning task whose job
is to grill definitions and then grow this tree.

## Pointers

- Existing implementation: `src/status.rs` — already lists harness installs
  with `VERSION.md` versions (with drift warning) and groves with live/done
  leaf counts; this work refines what it shows.
- Glossary: `CONTEXT.md` — note the three senses of "grove" (CLI, methodology,
  workstream) flagged under "Flagged ambiguities"; "active grove" needs a
  glossary entry once defined.
- ADRs cited only when grilling reaches one — none mandatory at the root.

## Notes

The grove name is the working title; the definitions of "active" and
"versions" are the first things to pin down in the planning leaf.

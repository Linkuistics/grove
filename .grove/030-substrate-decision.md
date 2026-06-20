# 030-substrate-decision

**Kind:** planning

> **Placeholder.** Fleshed out when picked, after `020` lands its evidence.
> Decompose lazily then — do not pre-grow the implementation leaves here.

## Goal

Using `020`'s `docs/research/loop-substrate-options.md`, **decide the loop
substrate** (Archon vs iTerm-trigger vs PTY-wrap vs headless vs harness-native)
and then **grow the implementation leaves** for the refactor.

## Context

Read the root `BRIEF.md` and the retired `010-plan` running log (D1–D8) first,
then `020`'s output. The decision turns on the three gates (fresh-context-per-
task; interactive-grilling-in-loop; restart-safety) and the "which complexity to
own" tradeoff. If the substrate is genuinely hard-to-reverse + surprising + a
real trade-off, this is where the first **ADR** of the refactor gets written
(promote D2/D3-class decisions to ADRs as they firm up).

## Done when

- The substrate is chosen and recorded (ADR if it clears the bar).
- The implementation leaves are grown (anticipated set, to be confirmed against
  the chosen substrate): the **dotted-decimal numbering** scheme + verb changes
  (D4/D5); the **global skill + `grove-llm`** distribution with backwards-compat
  (D8a); **shed-the-TUI**; **shed inbox/grove-meta + install machinery** (D3);
  the **substrate wiring** (the workflow / loop-driver itself); **migration**.

## Notes

(empty — grilled when picked)

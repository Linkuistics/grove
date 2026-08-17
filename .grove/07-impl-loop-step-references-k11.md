# loop-step-references-k11

## Goal

Rehome the seven **loop-step reference files** — `bootstrap.md`, `execute.md`,
`decompose.md`, `retire.md`, `commit.md`, `driver.md`, `grove.md` — onto the
ownership map in `docs/specs/corpus-rule-ownership.md`: each takes the rules the
inventory assigns it and sheds every rule it assigns elsewhere.

These files were cut out of the original decomposition. Leaf 3 covers `SKILL.md`,
leaf 4 the ten kind references, leaf 6 the format and habit files; nothing
covered these ~5,400 words, and they hold the corpus's worst duplication.
`decompose.md` alone restates the whole review-chain placement rule that
`driving.md`, `TASK-FORMAT.md` and `docs/specs/doubt-grove-review-mechanics.md`
also carry at full length.

## Context

Beyond the brief chain:

- `docs/specs/corpus-rule-ownership.md` — the inventory. The per-file tables for
  `grove.md`, `driver.md`, `bootstrap.md`, `execute.md`, `decompose.md`,
  `retire.md` and `commit.md` are this leaf's work list, and each row's
  *permitted mirror* column says whether a `SKILL.md` condition survives.
- `docs/specs/doubt-grove-review-mechanics.md` — reworked here, in place.

## Done when

- **`execute.md`'s *What each kind produces* section is gone.** It is a
  nineteen-kind summary of material the ten kind references own, and the driver
  already routed the session to its own. What `execute.md` keeps is the seven
  rows the inventory gives it: the review budget and its predicate and per-kind
  table, `escalated-review-routes-through-config`, `records-are-current-state`,
  and the two record-raising conditions — the ADR one now **naming
  `ADR-FORMAT.md`'s test rather than paraphrasing it as "sparingly"**.
- **`decompose.md` gains** `fog-or-ticket`, `vertical-slice`,
  `wide-refactor-expand-contract` and the review-chain habits from `driving.md`,
  and **sheds** its transcription of `leaf-decompose`'s mechanics — what the verb
  moves, retitles and creates is a command fact owned by `--help`. The *when* to
  reach for it stays.
- **`retire.md` gains** `triage-picks-the-verb` (the prune / reorder / issue
  triage) from `driving.md`.
- **`grove.md`'s *Specs* section** collapses to a pointer; `spec-membership-test`
  and `spec-grain-rule` are `SPEC-FORMAT.md`'s. Its plugin paragraph states what
  binds without each deferred skill.
- **The review-chain placement rule is stated once** in `decompose.md`, and
  nowhere else in `content/`. `SKILL.md` may carry the condition (leaf 3's
  business); `TASK-FORMAT.md` and `driving.md` are leaf 6's.
- **`docs/specs/doubt-grove-review-mechanics.md` is reworked in place** to cite
  `content/references/execute.md` and `content/references/decompose.md` as
  canonical for `review-budget`, `integration-placement` and
  `no-adjacency-exception`, rather than restating them. It keeps what is its own:
  the ownership predicate's rationale, the task-tree access seam, the producer
  handoff, and its test seams. A spec cites the ADRs and rules in its area and
  never restates them — the grain rule, applied to a spec that predates the
  ownership map.
- The **S** tests for the rules homed here land with this leaf: normalised,
  controlled phrase sweeps asserting exactly one procedure-register file states
  each. Emphasis stripped and whitespace collapsed — an unnormalised sweep misses
  a wrapped or bolded match, which was reproduced while writing the spec.

## Notes

- **Sequencing.** This runs before `corpus-split-k6` on purpose: that leaf moves
  rationale out of `driving.md` and `TASK-FORMAT.md`, and the rules it is moving
  out of the way of must already have their homes here. It runs after
  `skill-router-k4` because the conditions in `SKILL.md` name these files.
- **`grove.md` is where the seven constraints live**, and they are quoted by
  `SKILL.md`'s spine list — that mirror is permitted and stays.
- Do not write inventory rule IDs into `content/` as markers. The corpus carried
  140 unit markers with a build gate over them; that classification did its work
  and was deleted, and the spec is explicit that enforcement is per rule rather
  than by a universal parser.
- `content/SIGNAL.md`, `content/SIGNAL-FINISH.md` and `src/prompt.rs` are out of
  scope.

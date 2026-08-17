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
  already routed the session to its own. What `execute.md` keeps is the **eleven**
  rows the inventory gives it: the review budget with its predicate, per-kind
  table and doubt-pass procedure, `escalated-review-routes-through-config`, the
  four repo-claim disciplines (`verify-repo-claims-with-controls`,
  `enumerate-then-classify`, `no-self-invalidating-count`,
  `check-the-rescued-clause`), `decisions-land-as-they-settle` and
  `escalation-names-the-tradeoff`.
  **It keeps neither record rule.** `records-are-current-state` and the two
  record-raising conditions are **not** `execute.md`'s under the corrected map: an
  artifact Occasion outranks a loop-step one, so they belong to `ADR-FORMAT.md` and
  `SPEC-FORMAT.md`. Leave `execute.md`'s existing statement of them **in place and
  untouched** — `corpus-split-k6` lands the format-file statements and removes this
  one in the same commit, so the rule is never homeless between two commits.
- **`decompose.md` gains** `fog-or-ticket`, `vertical-slice`,
  `wide-refactor-expand-contract`, `prior-art-research-is-its-own-leaf`,
  `research-brief-names-downstream-questions`, `diversity-is-the-configs` and the
  review-chain habits from `driving.md`, and **sheds** its transcription of
  `leaf-decompose`'s mechanics — what the verb moves, retitles and creates is a
  command fact owned by `--help`. The *when* to reach for it stays.
- **`retire.md` gains** `triage-picks-the-verb` (the prune / reorder / issue
  triage) and `prune-scopes-to-the-whole-path` from `driving.md`.
- **`grove.md`'s *Specs* section** collapses to a pointer; `spec-membership-test`
  and `spec-grain-rule` are `SPEC-FORMAT.md`'s. Its plugin paragraph states what
  binds without each deferred skill.
- **`grove.md` keeps three rows and sheds its argument.** It owns
  `durable-artifact-set`, `plugin-prerequisite` and `build-boundary-is-the-binary`.
  *The seven constraints, argued* and *Why the glossary is the forcing function*
  **relocate to `docs/ARCHITECTURE.md`** — they are argument, so `Occasion = none`
  and `docs/` is a legal home. The normative statements are `SKILL.md`'s
  (`spine-seven-constraints`, `own`) and `CONTEXT-FORMAT.md`'s
  (`glossary-is-the-forcing-function`), and neither may be restated here. Carry
  constraint 4's *just-in-time, not few* clause into `SKILL.md`'s line for it
  rather than dropping it; `corpus-split-k6` owns the `CONTEXT-FORMAT.md` half.
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
- **`SKILL.md` owns the seven constraints, not `grove.md`** — the reverse of what
  this leaf was chartered with. Six corpus files cite them **by number**, every one
  of those files is conditionally loaded, and only `SKILL.md` and a kind reference
  are ever on a static path, so the numbered list has to sit where every session
  already reads it. It is `own` class and one row, not seven: the list is cited as
  a unit. `grove.md` keeps no copy.
- Do not write inventory rule IDs into `content/` as markers. The corpus carried
  140 unit markers with a build gate over them; that classification did its work
  and was deleted, and the spec is explicit that enforcement is per rule rather
  than by a universal parser.
- `content/SIGNAL.md`, `content/SIGNAL-FINISH.md` and `src/prompt.rs` are out of
  scope.

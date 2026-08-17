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
  already routed the session to its own. What `execute.md` keeps is the **twelve**
  rows the inventory gives it: the review budget with its predicate, per-kind
  table and doubt-pass procedure, `escalated-review-routes-through-config`, the
  five repo-claim disciplines (`verify-repo-claims-with-controls`,
  `enumerate-then-classify`, `sweep-scope-is-the-claim`,
  `no-self-invalidating-count`, `check-the-rescued-clause`),
  `decisions-land-as-they-settle` and `escalation-names-the-tradeoff`.
  `sweep-scope-is-the-claim` is `driving.md:330-336`'s three named narrowing
  failures — grep the claim not a file list, a path scope goes stale and misses the
  files in no tree, and a finding against a section does not reach the summary
  layer. It is a rule a session can obey both controls and still violate.
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
- **`retire.md` also gains `finish-is-the-drivers-to-discover`, and `finish.md`
  loses it in the same commit.** Its occasion is `{step:Retire, step:Finish}` and
  the earliest wins: the rule's trigger is *the last live leaf retires*, an event in
  Retire, and Finish is what the driver may launch afterwards. Filing it at Finish
  inverted its audience — it forbids an error only the eighteen **non**-finish kinds
  can commit, and pointed them at the one reference file none of them is routed to.
  Trigger sentence 17 names `references/retire.md`. This is the one row in this
  leaf's list whose old home belongs to `kind-references-k5`; take it as a move so
  the rule is never homeless, and do not leave a copy behind.
- **`commit.md` gains `no-kind-prefix-in-commit-subject`** — do not compensate for
  the bare stem with a `review:` / `impl:` subject convention. It is
  `TASK-FORMAT.md:237-239`'s imperative, a Commit-step rule that reached a
  filename-grammar file by riding inside the argument for the bare stem;
  `corpus-split-k6` removes it there once this leaf has landed it here.
- **`decompose.md` gains the two shape-*selection* rules, which are not the
  construction rules it already has.** `review-chain-when-load-bearing`
  (`driving.md:399-402`) says an artifact others will build on earns a chain, and
  artifact size and vendor preference are not the test;
  `vendor-pair-when-load-bearing` (`:92-94`) says a question earns two corpora only
  when it is load-bearing enough to pay for them. `chain-is-lazy` and `pair-is-eager`
  say how the steps *land* once the shape is chosen and say nothing about choosing
  it. Both criteria are already duplicated, unowned, at
  `content/references/decompose.md:93-96` and `:115-118` — state each **once**, and
  land its **S** sweep. They are the targets of new trigger sentences 25 and 26.
- **`decompose.md` also gains `steps-share-the-producers-stem`** — every step of a
  composed shape carries the producer's bare stem as its whole slug. It is
  `driving.md:469-479`'s imperative, and `rule-ownership-k15` gave it the row the
  inventory was missing; without a statement here, `corpus-split-k6` deletes the
  rule when it deletes the file. The grammar half — that a stem is convention and
  nothing parses it — stays `TASK-FORMAT.md`'s.
- **`references/driver.md` gains a `SKILL.md` trigger it did not have** (sentence 1,
  shared by `pick-walk-order` and `one-configuration`). That is leaf 4's sentence to
  write, but this leaf owns the file it points into: after the rewrite the seven
  driver rows chain from that one edge, so do not leave the file without the rows it
  is now the declared owner of.
- **`grove.md`'s *Specs* section** collapses to a pointer; `spec-membership-test`
  and `spec-grain-rule` are `SPEC-FORMAT.md`'s. Its plugin paragraph states what
  binds without each deferred skill.
- **`grove.md` keeps three rows and sheds its argument.** It owns
  `durable-artifact-set`, `plugin-prerequisite` and `build-boundary-is-the-binary`
  — all three at `Occasion = context`, the value `rule-ownership-k15` added so the
  placement function actually derives this file. They were `step:Execute` cells,
  which rule 6 maps to `execute.md`; do not carry that reading forward. The first
  two take **one `SKILL.md` trigger each** (sentences 18 and 19), not a shared one:
  *deciding where a durable artifact belongs* and *meeting a plugin citation* are
  two situations.
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

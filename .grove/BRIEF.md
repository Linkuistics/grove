# grove — brief

## Goal

Simplify, rationalise and improve the prompt corpus this repository installs —
Grove's embedded `content/` and the bundled plugin skills under `plugins/` — so
that every normative rule has one canonical source, every session's *loaded path*
is materially smaller, and the lifecycle invariants are held by behavioural tests
rather than by prose bulk. The audit motivating this is already complete; its
conclusions are the settled requirements recorded here and are not reopened.

## Done when

- `content/SKILL.md` is a compact session protocol/router of **at most 900 words**
  (from 3,152) that still carries: the authoritative mandate; no second
  pick; the driver's VCS statement as definitive; stale-launch handling;
  bootstrap order; the execution/decomposition boundary; human-only pruning;
  retire-before-commit; the commit boundary; and finish ownership with the
  terminal-signal distinction. The requirements said *roughly* `700–900`;
  `rule-ownership-k15` lowered the floor to 600 and `rule-ownership-k17` removed it.
  A floor can only be set from a measurement of the finished file, and no such
  measurement exists — the "~613" it was derived from was two measured parts plus
  this table's *ceiling* on an undrafted third. What a floor was for is done
  directly by three exact assertions: exactly 26 trigger sentences, each ≤25 words,
  and the eight `own` rows present. Those name the dropped row; a word count cannot.
- Each session-kind reference is **incremental** — its deliverable, its
  permissions, its special verification, and its unique human gate, and nothing a
  sibling reference already carries.
- A **rule-ownership inventory** exists giving every normative concept one
  canonical source, with rule ID, canonical source, permitted mirrors, load
  predicate, and behavioural tests.
- Policy (skill and kind references), mechanics (CLI help and schema), format
  grammar (`TASK-FORMAT.md`) and rationale/history (non-normative docs) are
  separated — with normative operational material still **embedded under
  `content/`** and explicitly reachable by an installed session.
- The two semantic contradictions are resolved deliberately: `requirements`
  always establishes *what*, but full one-question-at-a-time grilling runs only
  for **three or more interdependent questions**; ADR creation uses the narrower
  **AND** test from `decision-records`.
- The plugin skills are rationalised (see Decomposition) and carry **harness
  compatibility metadata that installation honours**, so Claude-only or
  harness-specific skills, and personal model/profile assumptions, are not
  installed blindly into every harness.
- Grove does not **silently** depend on an optional `linkuistics` plugin: every
  deferral either has a minimal Grove-local safety fallback (preferred) or an
  explicit provisioning verification.
- The loose 500-line ceiling in `tests/methodology.rs` is replaced by
  **per-session loaded-path word/token budgets**, and behavioural coverage exists
  for: no second pick, no VCS reprobe, stale launch, the requirements interview
  threshold, the decomposition boundary, human-only pruning, retire → commit →
  complete, the review budget, and all three finish-signal outcomes.
- `src/prompt.rs`'s runtime prompt architecture is unchanged in shape: load
  instruction, bare runtime facts, byte-exact terminal signal.
- `content/SIGNAL.md` and `content/SIGNAL-FINISH.md` are byte-identical to their
  state at the start of this grove.

## Constraints

- **Preserve `src/prompt.rs`.** Its three-part architecture is already
  disciplined and tested. This workstream changes the prose it points *at*, never
  the runtime that points.
- **Redesign, do not trim.** The deliverable is explicit rule ownership and
  resolvable load paths. Sentence-level compression that leaves two files still
  claiming the same rule has not done the work.
- **Ownership refactor first, mechanisation second.** Move rules to their
  canonical homes and fix the load paths before mechanising the command facts
  that change.
- **Normative material stays embedded.** Moving an operational rule out of
  `content/` and into a repo doc that an installed session cannot reach is a
  regression, not a simplification.
- **Leave the two signal files alone** until dedicated terminal-behaviour
  evaluations exist — they are the one surface where a wording change is
  unrecoverable mid-loop.
- **No unrelated product changes.** Open issue #8 (per-operation reasoning
  effort) is out of scope.

## Decomposition

Ordered by dependency, and named by handle rather than by position — two inserts
have already moved every position here, and a durable reference must survive the
next one. What the ordering encodes: the design decides where every rule lives, so
it comes first and is reviewed before anything is built on it; the behavioural
safety net comes next because it must be green *before* the rewrites start and
green after they land; the corpus rewrites follow in the order that lets each one
find the rules it needs already homed.

- `rule-ownership-k2` (design) — the inventory, the
  policy/mechanics/grammar/rationale separation, the two contradiction
  resolutions, and the fallback policy. Every later leaf executes against it.
  **Done** — `docs/specs/corpus-rule-ownership.md` and ADR
  `corpus-rules-have-one-owner`.
- `rule-ownership-k12` (review-design) — the adversarial read of that design,
  sequenced **ahead** of every leaf that executes against it. **Done** — five
  actionable defects, four of them P1.
- `rule-ownership-k13` (integrate-review-design) — the repair. **Done** — the
  placement function takes a pair and resolves by ordered first match; the
  inventory is five-column and audited file by file; `SKILL.md`'s three
  restatement classes and word budget are stated arithmetically; `driving.md`'s
  every surviving imperative has an embedded owner and its deletion is
  conditional on that table.
- `rule-ownership-k14` (review-design) — a **second** adversarial read, inserted
  ahead of the executing leaves for the same reason `k12` was. The repair was not
  a patch: it replaced the placement function's input and recomputed every owner
  cell, so what the leaves execute against is a design no reviewer has seen. The
  first review of the first design found four P1s, which is the evidence that this
  artifact is worth the cycle; `k13` spent no in-session reviewer so the budget
  went here. **Done** — four P1s and one P2, all confirmed against the corpus.
- `rule-ownership-k15` (integrate-review-design) — the second repair. **Done** —
  `Occasion` is a set with an earliest-step tie-break and a `context` value;
  reachability is an asserted **edge** rather than a loadable file; four missing
  rules got rows and the `own` set is eight; the canonical trigger sentences are
  written and measured; the ADR split in two by reversibility.
- `rule-ownership-k16` (review-design) — a **third** read, inserted ahead of the
  executing leaves like `k12` and `k14`, and **scoped to `k15`'s delta**. The
  justification is the pattern rather than a rule: each repair has been structural,
  and each structural repair has so far carried new P1s, so a design whose input
  model changed twice has not demonstrated convergence. It is the last review before
  execution; the chain's laziness is its exit, so a P2-or-below outcome cuts no
  integration leaf. **Done** — five P1s and one P2, all confirmed against the corpus.
- `rule-ownership-k17` (integrate-review-design) — the third repair, and the last
  leaf before execution. **Done** — `finish-is-the-drivers-to-discover` moves to
  `references/retire.md` under earliest-step-wins; the reachability graph is over
  cross-file rows only, so the 45 reflexive rows are in-file conditions rather than
  self-loops; `SKILL.md`'s word floor is removed as underived; the spec cites the
  two ADRs instead of restating the placement function and the restatement classes;
  and the promised sentence-level audit was actually re-run, adding five rows and
  two trigger sentences. It cut no fourth review: every P1 was a defect *in* the
  design rather than in the input model, the model itself was confirmed on all five
  axes the review checked, and the chain is lazy.
- `behavior-evals-k3` (impl) — behavioural coverage for the lifecycle invariants
  that this refactor must not change. Written against **current** behaviour so it
  passes before and after; it is the net the rewrites fall into.
- `skill-router-k4` (impl) — `content/SKILL.md` becomes the compact session
  protocol/router.
- `kind-references-k5` (impl) — the ten kind references become incremental; the
  requirements interview threshold lands here with its test.
- `loop-step-references-k11` (impl) — the seven loop-step files (`bootstrap`,
  `execute`, `decompose`, `retire`, `commit`, `driver`, `grove`) take the rules
  the design assigns them and shed what it assigns elsewhere.
- `corpus-split-k6` (impl) — `TASK-FORMAT.md`, `driving.md`, `grilling.md` and the
  `*-FORMAT.md` files split along grammar / policy / rationale, with rationale
  and history relocated to non-normative docs.
- `plugin-skills-k7` (impl) — compress `codebase-design`, `decision-records`,
  `simplify-project`; make `using-codebase-memory` and `using-jujutsu` short
  routers; fold `git-to-jj-mapping` under `using-jujutsu`; delete or sharply
  narrow generic `coding-style`; make language defaults repo-config-first; soften
  the universal mandates in `cli-tool-design`. `using-testanyware` (643 words) is
  the exemplar.
- `harness-compat-k8` (impl) — harness compatibility metadata on skills, and
  `plugins/install.sh` filtering that honours it.
- `plugin-fallback-k9` (impl) — audit every `linkuistics:` deferral in the
  rewritten corpus and give each a minimal Grove-local fallback. Runs after
  `plugin-skills-k7` so it audits final text.
- `loaded-path-budgets-k10` (impl) — replace the 500-line prose-shape ceiling with
  per-session loaded-path word/token budgets over the finished corpus.

## The design's work order

`rule-ownership-k2` has landed and `rule-ownership-k13` has repaired it against
`rule-ownership-k12`'s findings. `docs/specs/corpus-rule-ownership.md` is what
every leaf below executes against; this section carries only the part that dies
with `.grove/` — which leaf makes which edit.

**What `rule-ownership-k15` changed on top of that.** `Occasion` is a **set** —
`escalation-names-the-tradeoff` records three steps and the earliest wins — and it
gained a `context` value that owns `references/grove.md`, whose three rows were
previously `step:Execute` cells the function never derived. Reachability is an
asserted **edge**: the triggering file must literally name the owner's path and
every non-static owner needs an incoming sentence, which is what gave
`references/driver.md` its trigger (its first two rows had claimed `mirror = none`
*and* a `SKILL.md` trigger). Four rules gained rows —
`one-task-is-one-session` (`SKILL.md`, `own`), `steps-share-the-producers-stem`
(`decompose.md`), `finish-promotes-before-teardown` and `declined-finish-stays-live`
(`finish.md`) — and `declaration-lines-are-convention` widened to
`convention-not-grammar`. The trigger set is written out and measured rather than
multiplied. The ADR became two records.

**What `rule-ownership-k17` changed on top of that.** Four things move a cell a
leaf below executes against. (1) `finish-is-the-drivers-to-discover` is
`references/retire.md`'s, not `references/finish.md`'s — its occasion is
`{step:Retire, step:Finish}` and the earliest wins, so `loop-step-references-k11`
lands it in `retire.md` and removes it from `finish.md` in the same commit, and
trigger sentence 17 names `retire.md`. (2) Reachability is a graph over
**cross-file** rows only; a row whose `@` file is its own owner records an in-file
condition and no edge, so `loaded-path-budgets-k10` partitions before it walks —
otherwise the cycle check fails on 45 of 92 conditional rows. (3) `SKILL.md` has a
**ceiling of 900 words and no floor**; the exact-count assertions are what detect a
dropped row. (4) Five rules gained rows and the trigger set is **26** sentences:
`review-chain-when-load-bearing` and `vendor-pair-when-load-bearing` (sentences 25
and 26, `decompose.md`), `sweep-scope-is-the-claim` (`execute.md`),
`no-kind-prefix-in-commit-subject` (`commit.md`) and
`finish-resume-reruns-the-same-command` (`finish.md`). The spec now cites the two
ADRs for the placement function and the restatement classes rather than restating
them, so a leaf executing against it **reads all three documents**.

**What the earlier repair changed for the leaves below.** Placement now takes a pair,
`Bound(R)` **and** `Occasion(R)`, resolved by an ordered first-match rule, so
some owners moved: an artifact rule beats a loop-step rule, `records-are-current-state`
splits across the two format files, `glossary-is-the-forcing-function` belongs to
`CONTEXT-FORMAT.md`, and `verify-repo-claims-with-controls` is `execute.md`'s
rather than `impl.md`'s. A `SKILL.md` restatement now declares one of three
classes — `own`, `trigger`, `none` — and the file's word budget is stated
arithmetically rather than assumed. Every inventory row carries five columns.
`driving.md`'s deletion is **conditional** on the spec's relocation table being
discharged first.

**No rule is homeless between two commits.** When a rule's owner changes and the
two files belong to two leaves, the move belongs to the **later** leaf: it adds
the new statement and removes the old one in the same commit, and the earlier
leaf leaves the old statement untouched. A rule with no statement in `content/`
between two commits is deleted for every session launched in that window, and in
a meta-grove that window is real sessions.

**The two tree changes.** The seven loop-step reference files (~5,400 words,
holding the corpus's worst duplication) had **no leaf**: `skill-router-k4`,
`kind-references-k5` and `corpus-split-k6` covered `SKILL.md`, the ten kind
references, and the format/habit files respectively, and nothing covered the
loop-step files — which is where most of the design's redistribution lands. So
`loop-step-references-k11` was cut for them, ahead of `corpus-split-k6` (the rules
`corpus-split` moves rationale *out of the way of* must already have their homes)
and not folded into it (`corpus-split` already faces ~11,800 words of source).

And `rule-ownership-k12` — like `k14` and `k16` after it — reviews the design
**before** the leaves that execute against it, rather than landing at the tree's end
where a plain `leaf-add` would have put it. A `review-*` step normally re-derives and so may land anywhere, but
that claim is about citation staleness, not dependency: here every subsequent leaf
rewrites the corpus this design governs, which is exactly the case the rule marks
as *narrower than free*.

**Per leaf:**

- `behavior-evals-k3` — scope is the **B★ rows** the spec marks: eight of the nine
  areas this brief's *Done when* names. The ninth, the interview threshold, is a
  contradiction being resolved rather than a behaviour being preserved, so its test
  is red until `kind-references-k5` lands the fix and belongs to that leaf — which
  is what `k3`'s own task file already says. Not the whole **B** set:
  every other behavioural row lands with the rewrite that homes its rule, or this
  leaf balloons. No **S** row belongs here at all — single-source and budget
  assertions can only go green *after* the rewrite that homes each rule, so
  handing one here would charter this leaf to fail.
- `skill-router-k4` — `SKILL.md` is the **condition register**, and every
  restatement declares a class. It **owns** eight rows outright (the routing
  table, the numbered spine, one-task-is-one-session, the bootstrap order, the
  mandate, no second pick, the stated VCS, the HITL/AFK mark) because each one's
  whole content is its trigger; it carries the **26 canonical `trigger`
  sentences** — written out verbatim in the spec's *The trigger sentences*, one
  per situation, five of them covering two rows each — for 31 rows owned
  elsewhere; and it says nothing at all about a rule whose Bound is one kind or
  one family. That last class is most of what the file carries today and most of
  why it shrinks. Lands the budget assertion — total **at most 900 words**,
  **exactly 26** trigger sentences, each ≤25 words, eight `own` rows present, and
  **no lower bound**. It may reword a
  trigger within the grammar; it may not add or drop one, because the set is the
  reachability graph's edge list out of `SKILL.md`.
- `kind-references-k5` — "incremental" now reads mechanically: state what is true
  of this kind and no sibling, and nothing a loop-step or format file owns. Lands
  the grilling threshold in `references/requirements.md` (stated **once** — the
  file currently carries the always-form bullet twice *and* the three-question
  trigger separately) and deletes `references/design.md`'s OR-form ADR test.
  Takes `cite-framework-decisions-to-source` into `references/impl.md` — and
  **only** that one: `verify-repo-claims-with-controls` binds `review-*`,
  `design` and the research kinds too, so it is `execute.md`'s. Also takes
  `small-workstream-may-fuse-the-three` from `driver.md`,
  `agree-the-seams-during-grilling` and `probe-with-concrete-scenarios` from
  `grilling.md` and `SPEC-FORMAT.md`, and `sequence-interdependent-questions`
  into `requirements.md`.
- `loop-step-references-k11` — deletes `references/execute.md`'s *What each kind
  produces* entirely; takes `fog-or-ticket`, `vertical-slice`,
  `wide-refactor-expand-contract`, `prior-art-research-is-its-own-leaf`,
  `research-brief-names-downstream-questions`, `diversity-is-the-configs`,
  `steps-share-the-producers-stem` and the
  review-chain habits into `decompose.md`; `triage-picks-the-verb` and
  `prune-scopes-to-the-whole-path` into `retire.md`; and the repo-claim,
  decision-log, escalation and doubt-pass rules into `execute.md`. Replaces
  "sparingly" with the pointer to `ADR-FORMAT.md`'s test in `execute.md` and
  `grove.md`; collapses `grove.md`'s restatement of the spec membership and grain
  rules into a pointer. **Relocates `grove.md`'s argued spine and glossary
  rationale to `docs/ARCHITECTURE.md`** — argument, `Occasion = none`; the
  normative statements are `SKILL.md`'s and `CONTEXT-FORMAT.md`'s. **Also reworks
  `docs/specs/doubt-grove-review-mechanics.md` in place** — it restates
  `review-budget`, `integration-placement` and `no-adjacency-exception` in full,
  and under the grain rule a spec cites rather than restates; it keeps the
  ownership predicate's rationale, the task-tree access seam, and its test seams.
- `corpus-split-k6` — `TASK-FORMAT.md` sheds its policy (composition shapes, doubt
  budget table, kind disciplines, *a leaf never names a harness*) to the owners
  above and keeps the name grammar, the kind list, the body shape — including the
  running log's *section* — and `convention-not-grammar`, which now covers the
  shared stem and the relative ordering alongside the two declaration lines. It
  also sheds the two global rules in its opening (`:21-22`): *one task is one
  session* to `SKILL.md`'s `own` row and the human-only-pruning duplicate to
  `references/retire.md`, both already stated by then. Splits
  `records-are-current-state` into `ADR-FORMAT.md` and `SPEC-FORMAT.md` and
  removes `execute.md`'s statement in the same commit; moves
  `glossary-is-the-forcing-function` and `challenge-and-sharpen-terms` into
  `CONTEXT-FORMAT.md`; lands `research-to-adr-bridge` in `ADR-FORMAT.md`.
  `grilling.md` gains a Grove-authored entry condition above the bundled
  `<what-to-do>` block — the bundled body stays byte-intact — and sheds its four
  duplicate sections to their owners. **`driving.md` does not survive as an
  embedded file, and this leaf performs the move** — but only after every row of
  the spec's relocation table is discharged and every `SKILL.md` sentence pointing
  at it has been repointed. Two human-operator habits go to `docs/USAGE.md`; the
  rest of the residue is deleted rather than relocated.
- `plugin-fallback-k9` — executes the spec's 14-row deferral table. Discharged
  when every deferring sentence states what binds without the plugin.
- `loaded-path-budgets-k10` — consumes the load-predicate column. Its static half
  is computable from `src/prompt.rs`'s existing exhaustive `reference_file` match,
  so the seam costs no new production code. Three assertions ride with it: a row
  claiming `static(K)` whose owner is not `SKILL.md` or `reference_file(k)` fails;
  every `on(…) @ F` chain must terminate at a static path with no cycles; and **`F`
  must literally name the owner's path**, with every non-static owner carrying at
  least one incoming edge. The third is the one a loadability check cannot make —
  without it a row naming a loadable file that says nothing about the rule passes,
  which is how `references/driver.md` came to have no incoming sentence at all.

**Unchanged and reaffirmed:** `content/SIGNAL.md` and `content/SIGNAL-FINISH.md`
are out of scope for every leaf. `src/prompt.rs` is out of scope for every leaf.
No rule ID from the inventory is ever written into `content/` as a marker.

Producers here are load-bearing: several rewrite the corpus every future session
reads. Cut the `review-<producer>` step as the producer's last act wherever the
artifact will be built on — the chain is lazy, so a producer that genuinely needs
no review creates nothing.

## Pointers

- **Read first from leaf 3 onward:** `docs/specs/corpus-rule-ownership.md` — the
  placement function, the rule inventory, the two contradiction resolutions and
  the deferral policy. Every leaf below executes against it.
- ADRs a session here must read:
  - `docs/adr/corpus-rules-have-one-owner.md` — the pair, the ordered owner
    function, and reachability as an asserted edge.
  - `docs/adr/restatement-declares-its-class.md` — `own` / `trigger` / `none`, the
    ≤25-word grammar, and when two rows may share one sentence.
  - `docs/adr/skill-delivers-the-methodology.md` — why the provisioned skill *is*
    the delivery path, and why `${prompt}` is only the guaranteed core.
  - `docs/adr/one-build-owns-a-session.md` — the build boundary and the stamp;
    corpus edits reach no session in this same loop.
- Architecture: `docs/ARCHITECTURE.md` — *Embedded methodology* (§689), *The
  corpus's shape, and the three alarms over it* (§774), *Documentation ownership*
  (§13).
- Spec: `docs/specs/doubt-grove-review-mechanics.md` — the review-budget
  ownership predicate, which `behavior-evals` must cover.
- Two bounded contexts share this repo — see `CONTEXT-MAP.md`. Leaves 1–5, 8 and 9
  are **grove**; leaves 6 and 7 are **skills**. A term is defined in its owning
  context's glossary, never both.
- Glossary terms in play (`CONTEXT.md`): Loaded path, Guaranteed core, Kind
  reference file, Loop-step reference file, Condition / procedure, Embedded
  methodology, Global skill provisioning, Build pairing.
- Measurements taken at the start of this grove, for comparison rather than as
  contract: `content/` totals 23,532 words; `SKILL.md` 3,152; `TASK-FORMAT.md`
  3,012; `driving.md` 5,817; the ten kind references 2,133 combined. The
  `linkuistics:` deferrals number 14 (distinct `(file, skill)` pairs) across
  **9** files — recounted by `rule-ownership-k2` with controls; the original note
  said 7 files.

## On the horizon

- Whether the plugin compression in leaf 7 forces a second pass once leaf 10
  measures real budgets. Statable now, not yet answerable, so it stays a note.

*(The `driving.md` question is settled — see the work order above. It was the
largest single lever and the placement function decided it without a separate
judgement.)*

## Notes

- This is a **meta-grove**: the corpus being edited is the corpus driving the
  session editing it, across the build boundary. A session here runs against the
  *installed* `grove` and reads the *installed* skill, so nothing it commits to
  `content/` reaches any session in this same loop. Verify by reading the files
  and the tests, never by expecting the next session to behave differently.
- ADR candidates the requirements leaf deliberately left to the leaf that settles
  each one's content:
  - *one canonical source per normative rule, with permitted mirrors* — **raised**
    by `rule-ownership-k2` as `docs/adr/corpus-rules-have-one-owner.md`. It is a
    new record rather than a rework: no existing ADR decides intra-corpus
    placement, and it cites `skill-delivers-the-methodology` for the `if`/`then`
    asymmetry it applies one channel further in.
  - *Grove keeps a minimal local fallback rather than hard-depending on an
    optional plugin* — still open, owner `plugin-fallback-k9`. The design has
    settled the policy per deferral, so what remains for that leaf to judge is
    whether the settled policy clears the AND test as a standing decision.

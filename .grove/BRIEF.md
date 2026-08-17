# grove — brief

## Goal

Simplify, rationalise and improve the prompt corpus this repository installs —
Grove's embedded `content/` and the bundled plugin skills under `plugins/` — so
that every normative rule has one canonical source, every session's *loaded path*
is materially smaller, and the lifecycle invariants are held by behavioural tests
rather than by prose bulk. The audit motivating this is already complete; its
conclusions are the settled requirements recorded here and are not reopened.

## Done when

- `content/SKILL.md` is a compact session protocol/router of roughly **700–900
  words** (from 3,152) that still carries: the authoritative mandate; no second
  pick; the driver's VCS statement as definitive; stale-launch handling;
  bootstrap order; the execution/decomposition boundary; human-only pruning;
  retire-before-commit; the commit boundary; and finish ownership with the
  terminal-signal distinction.
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
  sequenced **ahead** of every leaf that executes against it.
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

`rule-ownership-k2` has landed. `docs/specs/corpus-rule-ownership.md` is what
every leaf below executes against; this section carries only the part that dies
with `.grove/` — which leaf makes which edit.

**The two tree changes.** The seven loop-step reference files (~5,400 words,
holding the corpus's worst duplication) had **no leaf**: `skill-router-k4`,
`kind-references-k5` and `corpus-split-k6` covered `SKILL.md`, the ten kind
references, and the format/habit files respectively, and nothing covered the
loop-step files — which is where most of the design's redistribution lands. So
`loop-step-references-k11` was cut for them, ahead of `corpus-split-k6` (the rules
`corpus-split` moves rationale *out of the way of* must already have their homes)
and not folded into it (`corpus-split` already faces ~11,800 words of source).

And `rule-ownership-k12` reviews the design **before** the leaves that execute
against it, rather than landing at the tree's end where a plain `leaf-add` would
have put it. A `review-*` step normally re-derives and so may land anywhere, but
that claim is about citation staleness, not dependency: here every subsequent leaf
rewrites the corpus this design governs, which is exactly the case the rule marks
as *narrower than free*.

**Per leaf:**

- `behavior-evals-k3` — scope is the inventory's **B** rows only. The **S** rows
  (single-source and budget assertions) can only go green *after* the rewrite that
  homes each rule, so each rewriting leaf lands its own. Handing them here would
  charter this leaf to fail.
- `skill-router-k4` — `SKILL.md` is the **condition register**. A condition is one
  sentence naming the file with the procedure, and may not carry a test, a
  threshold, a list or a procedure. Rules whose Bound is one kind or one family
  lose their `SKILL.md` mirror outright; that is most of what the file carries
  today and most of why it shrinks.
- `kind-references-k5` — "incremental" now reads mechanically: state what is true
  of this kind and no sibling, and nothing a loop-step or format file owns. Lands
  the grilling threshold in `references/requirements.md` (stated **once** — the
  file currently carries the always-form bullet and the three-question trigger as
  two independent statements) and deletes `references/design.md`'s OR-form ADR
  test. Takes `cite-framework-decisions-to-source` and
  `verify-repo-claims-with-controls` into `references/impl.md` from `driving.md`.
- `loop-step-references-k11` — deletes `references/execute.md`'s *What each kind
  produces* entirely; takes `fog-or-ticket`, `vertical-slice`,
  `wide-refactor-expand-contract` and the review-chain habits into
  `decompose.md`, and `triage-picks-the-verb` into `retire.md`; replaces
  "sparingly" with the pointer to `ADR-FORMAT.md`'s test in `execute.md` and
  `grove.md`; collapses `grove.md`'s restatement of the spec membership and grain
  rules into a pointer. **Also reworks `docs/specs/doubt-grove-review-mechanics.md`
  in place** — it restates `review-budget`, `integration-placement` and
  `no-adjacency-exception` in full, and under the grain rule a spec cites rather
  than restates; it keeps the ownership predicate's rationale, the task-tree access
  seam, and its test seams.
- `corpus-split-k6` — `TASK-FORMAT.md` sheds its policy (composition shapes, doubt
  budget table, kind disciplines, *a leaf never names a harness*) to the owners
  above and keeps the name grammar, the kind list, the body shape and the two
  declaration lines. `grilling.md` gains a Grove-authored entry condition above the
  bundled `<what-to-do>` block — the bundled body stays byte-intact. **The
  `driving.md` question is answered**, not left here: it does not survive as an
  embedded file. This leaf performs the move.
- `plugin-fallback-k9` — executes the spec's 14-row deferral table. Discharged
  when every deferring sentence states what binds without the plugin.
- `loaded-path-budgets-k10` — consumes the load-predicate column. Its static half
  is computable from `src/prompt.rs`'s existing exhaustive `reference_file` match,
  so the seam costs no new production code.

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
  - `docs/adr/corpus-rules-have-one-owner.md` — file a rule by its load
    predicate; a mirror is a condition only.
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

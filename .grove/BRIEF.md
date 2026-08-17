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

Ordered by dependency. The design leaf comes first because it decides where every
rule lives, and the behavioural safety net comes second because it must be green
*before* the rewrites start and green after they land.

1. `rule-ownership` (design) — the inventory, the policy/mechanics/grammar/rationale
   separation, the two contradiction resolutions, and the fallback policy. Every
   later leaf executes against it.
2. `behavior-evals` (impl) — behavioural coverage for the lifecycle invariants
   that this refactor must not change. Written against **current** behaviour so it
   passes before and after; it is the net the rewrites fall into.
3. `skill-router` (impl) — `content/SKILL.md` becomes the compact session
   protocol/router.
4. `kind-references` (impl) — the ten kind references become incremental; the
   requirements interview threshold lands here with its test.
5. `corpus-split` (impl) — `TASK-FORMAT.md`, `driving.md`, `grilling.md` and the
   `*-FORMAT.md` files split along grammar / policy / rationale, with rationale
   and history relocated to non-normative docs.
6. `plugin-skills` (impl) — compress `codebase-design`, `decision-records`,
   `simplify-project`; make `using-codebase-memory` and `using-jujutsu` short
   routers; fold `git-to-jj-mapping` under `using-jujutsu`; delete or sharply
   narrow generic `coding-style`; make language defaults repo-config-first; soften
   the universal mandates in `cli-tool-design`. `using-testanyware` (643 words) is
   the exemplar.
7. `harness-compat` (impl) — harness compatibility metadata on skills, and
   `plugins/install.sh` filtering that honours it.
8. `plugin-fallback` (impl) — audit every `linkuistics:` deferral in the rewritten
   corpus and give each a minimal Grove-local fallback. Runs after 6 so it audits
   final text.
9. `loaded-path-budgets` (impl) — replace the 500-line prose-shape ceiling with
   per-session loaded-path word/token budgets over the finished corpus.

Leaves 3–5 are cut against the requirements as stated, not against a design that
does not yet exist. `rule-ownership` may reshape them — `leaf-insert`,
`leaf-add` and `leaf-decompose` are how it should, rather than absorbing the
difference silently.

Producers here are load-bearing: several rewrite the corpus every future session
reads. Cut the `review-<producer>` step as the producer's last act wherever the
artifact will be built on — the chain is lazy, so a producer that genuinely needs
no review creates nothing.

## Pointers

- ADRs a session here must read:
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
  `linkuistics:` deferrals number 14 across 7 files.

## On the horizon

- Whether `content/driving.md` (5,817 words, the largest single file and mostly
  habit-and-rationale) survives as an embedded file at all, or becomes a
  non-normative doc with its few operational rules rehomed. `corpus-split` decides
  it; recorded here because it is the largest single lever and worth watching from
  above.
- Whether the plugin compression in leaf 6 forces a second pass once leaf 9
  measures real budgets. Statable now, not yet answerable, so it stays a note.

## Notes

- This is a **meta-grove**: the corpus being edited is the corpus driving the
  session editing it, across the build boundary. A session here runs against the
  *installed* `grove` and reads the *installed* skill, so nothing it commits to
  `content/` reaches any session in this same loop. Verify by reading the files
  and the tests, never by expecting the next session to behave differently.
- ADR candidates deliberately **not** raised by this requirements leaf, because
  the design that justifies them does not exist yet: *one canonical source per
  normative rule, with permitted mirrors* (owner: `rule-ownership`) and *Grove
  keeps a minimal local fallback rather than hard-depending on an optional
  plugin* (owner: `plugin-fallback`). Each clears the AND test on its face; each
  should be raised by the leaf that settles its content, in place, not appended
  beside an existing record.

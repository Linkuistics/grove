# rule-ownership-k15

**Integrates:** rule-ownership-k14

## Goal

Repair the second adversarial review of the rule-ownership design before any
corpus-rewrite leaf executes against it. Rework
`docs/specs/corpus-rule-ownership.md` and the ADR set in place, then reconcile
the root brief and affected downstream leaf contracts.

The review found four P1 design defects and one P2 ADR-coherence defect. Preserve
the confirmed B-star boundary, the existing `reference_file` runtime seam, and
the controlled counts recorded below.

## Context

- `docs/specs/corpus-rule-ownership.md`
- `docs/adr/corpus-rules-have-one-owner.md`
- `.grove/05-DONE-review-design-rule-ownership-k14.md`
- `.grove/BRIEF.md`
- `content/TASK-FORMAT.md`, `content/driving.md`, and
  `content/references/finish.md`
- `src/prompt.rs:136` (`reference_file`)

## Findings

### P1 — `Occasion(R)` is neither closed nor single-valued over the real corpus

The precedence is deterministic only after a valid pair has been supplied; the
input model still cannot represent every real rule without an arbitrary choice.
The spec says `Occasion` has exactly the five shapes at
`docs/specs/corpus-rule-ownership.md:46`, and its artifact domain is the closed
list `task`, `brief`, `adr`, `spec`, `glossary` at `:53`. The inventory itself
uses the undeclared value `artifact:research doc` twice (`:553`, `:560`). Rule 2
happens to mask that invalid value for today's two per-kind rows, but the claimed
schema and its future all-kind behaviour are undefined.

The problem is not only a missing enum member. `escalation-names-the-tradeoff`
is assigned `step:Execute` at `:394`, while its stated trigger is *handing back
to a human*. That event also occurs during Retire (an unnameable node-close gap
or pruning decision) and Finish (the confirmation proposal). `Occasion` permits
one step, not an event crossing steps, and the ordered first-match rules at
`:57-66` do not choose which Occasion to record. `durable-artifact-set` has the
same cross-step pressure: artifact placement occurs during Execute, Decompose,
Retire, and Finish, but the row chooses Execute (`:336`) without a rule that
makes that choice derivable.

Define how cross-step occasions and research artifacts are represented, then
recompute the affected owner and load cells. Recording one arbitrary step and
letting precedence compute from it is the original ambiguity moved into the
input rather than removed.

### P1 — the reachability test accepts a file that contains no edge to the owner

The design defines `mirror = none` as no `SKILL.md` statement at all
(`docs/specs/corpus-rule-ownership.md:159`), but `pick-walk-order` and
`one-configuration` both declare `mirror = none` while claiming a load predicate
whose trigger is in `SKILL.md` (`:352-353`). None of the seven `own` rows at
`:317-323`, and no `trigger` row, names `references/driver.md`. After the rewrite,
a session therefore has no sentence that sends it there.

The proposed assertion at `:811-817` still passes: it checks only that the file
after `@` is static or reachable, so `@ SKILL.md` is accepted without proving
that `SKILL.md` contains a pointer to the row's owner. This certifies the exact
unreachable-rule failure the test claims to prevent. Make reachability an
explicit edge — source rule/sentence to target owner — and assert that the
source actually names the target, not merely that the source file can be loaded.
Then walk every edge again. The remaining literal file paths terminate, but that
is not sufficient evidence of semantic reachability.

### P1 — the file-by-file inventory is still incomplete

The nominated files refute the completeness rule at
`docs/specs/corpus-rule-ownership.md:244-248`:

- `content/TASK-FORMAT.md:21-22` states two global rules, *one task is one
  session* and human-only pruning. The former also appears in `content/SKILL.md:8-10`
  and has no inventory row, mirror declaration, or relocation. Under the design's
  own test it is a legitimate eighth `own` candidate: global orientation, with
  no procedure left to defer. The latter is an undeclared procedure-register
  duplicate of `pruning-is-hitl`.
- The bare-stem rule is imperative in `content/driving.md:469-479` and repeated
  throughout `content/TASK-FORMAT.md:125-205`. The `driving.md` relocation table
  lists `name-step-kind-off-the-producer` (`docs/specs/corpus-rule-ownership.md:772`),
  which chooses `review-design` versus `review-impl`; it never records the
  independent requirement that all steps keep the producer's bare stem. No
  inventory row owns that rule, so deleting `driving.md` and shedding
  `TASK-FORMAT.md` policy deletes it.
- `content/references/finish.md:18-20` requires the finish session to promote
  durable brief material before teardown, and `:75-78` says a declined finish
  remains live and no session retires it. Neither rule appears among the seven
  finish rows at `docs/specs/corpus-rule-ownership.md:453-459`. These are session
  conduct, not CLI mechanics.

Re-audit at least `TASK-FORMAT.md`, `references/finish.md`, and every imperative
in `driving.md`; give each surviving rule a five-column row and each duplicate or
command fact an explicit relocation. Recompute the `own` and trigger budgets
after adding the missing rows. The current conditional deletion table is not a
safe deletion condition while the bare-stem rule is absent from it.

### P1 — the `SKILL.md` arithmetic contradicts its own trigger grammar

The prose budget says 19 trigger sentences (`docs/specs/corpus-rule-ownership.md:177-197`),
while the test seam asserts at most 18 (`:818-820`). The inventory contains 27
trigger rows. Its reduction to 19 also depends on compound sentences that the
trigger class expressly forbids: a trigger must name one situation, one clause,
and the owner file's path, with no branch or enumeration (`:157-159`). In
particular:

- `durable-artifact-set` and `plugin-prerequisite` have different situations
  but are forced into one *grove sentence* (`:336-337`);
- the ADR test and spec agreement point have different situations and different
  owner files (`:605`, `:623`) but are forced into one *records-raised* sentence;
- the ADR and spec current-state rows likewise have different owner files
  (`:608`, `:626`);
- the count explanation pairs `pruning-is-hitl` with `no-fourth-status`
  (`:190-196`), while the inventory instead says `no-fourth-status` shares
  `triage-picks-the-verb` (`:432-435`).

The word ceilings themselves are feasible. A concrete draft of the seven listed
`own` bodies measured 192 words:

1. Before acting, read the reference file named for your session kind:
   requirements, design, planning, prototype, impl, research, combine-research,
   finish, review, or integrate-review. Five review kinds share review; five
   integration kinds share integrate-review; research-a and research-b share
   research.
2. Seven constraints govern every session: artifacts, not state; read, do not
   run; suggested shape, not enforced schema; lazy means just-in-time, not few;
   Grove guides, never gates; every durable artifact remains legible without
   Grove; keep the rules to one page.
3. Resolve the mandated handle. Stop if it is absent or terminal. Then read the
   relevant glossary, ADRs cited by the briefs, ancestor briefs from root to
   leaf, and the task file. Read nothing else by reflex.
4. The driver selected this leaf before launch. Its stable handle is
   authoritative and nothing modulates the mandate.
5. Do not run pick again. Pick is diagnostic only; if it disagrees with the
   mandate, the mandate wins.
6. Use the version control and root stated by the driver. Do not probe again,
   and disregard a conflicting harness banner.
7. Requirements, prototype, and finish are HITL; every other kind is AFK. The
   mark predicts who is present but neither permits nor forbids asking the human.

Five hard trigger drafts measured 11, 13, 14, 11, and 11 words respectively:

1. When cutting an integration step, follow the placement rule in
   `references/decompose.md`.
2. When raising an ADR or an agreement-point spec, use `ADR-FORMAT.md` or
   `SPEC-FORMAT.md` respectively.
3. When changing an ADR or spec, keep its set current through `ADR-FORMAT.md` or
   `SPEC-FORMAT.md`.
4. When deciding artifact placement or meeting a plugin citation, follow
   `references/grove.md`.
5. When work surfaces or outgrows its brief, externalize it through
   `references/decompose.md`.

Drafts 2-4 demonstrate the real failure: they fit easily under 25 words only by
using the branch/enumeration that the class forbids. Write the complete set of
canonical trigger sentences into the design and measure those actual sentences,
or relax the grammar and recompute the cap. Do not hand `skill-router-k4` three
incompatible numbers and leave it to choose one.

### P2 — the ADR contains two independently reversible decisions

`docs/adr/corpus-rules-have-one-owner.md:3-17` decides the placement input and
ordered owner function. Lines `:19-28` separately decide the condition-register
restatement taxonomy and its 25-word grammar. Either can change without the
other: the placement function can gain occasion sets or a new precedence while
`own` / `trigger` / `none` remains intact; the mirror policy can permit generated
self-contained copies or alter its classes while every canonical owner remains
unchanged. Both clear the ADR AND test and have separate rejected alternatives.

Split the record into the canonical-owner placement decision and the
condition-register restatement decision, reconciling citations by slug. The spec
may keep them together as one area design; the ADR set should keep one binding
trade-off per record.

## Eight-claim disposition

1. **Precedence total and deterministic — refuted.** It maps a valid pair, but
   the Occasion domain excludes a value the inventory uses and cannot express a
   cross-step trigger without an arbitrary choice (finding 1).
2. **`own` is not an escape hatch — confirmed as a classifier, refuted in this
   inventory.** The suggested doubt-budget, triage, and kind-set candidates have
   a non-orientation occasion and retained procedure/enumeration, so the test
   refuses them. The omitted *one task is one session* rule genuinely passes it,
   making the seven-row closure false (finding 3).
3. **Arithmetic survives — refuted.** The words fit, but the sentence count and
   the grammar do not agree (finding 4).
4. **`driving.md` relocation complete — refuted.** The bare-stem imperative has
   no row or relocation (finding 3).
5. **Reachability chain sound — refuted.** All literal file paths can terminate
   while `references/driver.md` still has no incoming sentence (finding 2).
6. **Inventory complete now — refuted.** Both nominated files contain untracked
   normative conduct, as does `driving.md` (finding 3).
7. **B-star / B / S boundary — confirmed.** The current corpus already carries
   `externalize-by-default` and `bigger-than-brief-decomposes` in `SKILL.md` and
   `references/decompose.md`, and the current Retire → Commit → `SIGNAL.md`
   sequence is green-before-capable. The grilling threshold remains the only
   named required behaviour that is contradictory before its rewrite.
8. **ADR is one decision — refuted.** Placement and permitted restatement are
   separately reversible decisions (finding 5).

## Confirmed evidence to preserve

- The normalised constraint sweep found 19 numbered citations across eight
  `content/` files now; after excluding `SKILL.md` and the to-be-deleted
  `driving.md`, the claimed 11 citations across six conditional corpus files
  remain. The same instrument found eight cross-tree control hits.
- The plugin-deferral recount found 14 distinct `(file, skill)` pairs across
  nine files.
- `src/prompt.rs:136-157` exhaustively maps the nineteen kinds to the ten static
  reference files. Graph inspection found four direct callers, including the
  prompt composer and the existing mapping tests. That is the correct runtime
  seam for static-path budgets; no production-code change is warranted.
- This review ran no test, build, lint, or format command and changed no reviewed
  artifact.

## Done when

- Each P1 is repaired in the spec and every affected inventory cell is
  recomputed rather than patched locally.
- The ADR set is reconciled into one decision per record, with every citation
  updated.
- The root brief and downstream leaf contracts reflect the corrected owner,
  reachability, inventory, and budget rules before `behavior-evals-k3` runs.
- `content/SIGNAL.md`, `content/SIGNAL-FINISH.md`, and `src/prompt.rs` remain
  unchanged.

## Decisions (running log)

- **All five findings verified against the corpus before repair, none refuted.**
  `artifact:research doc` at the two cited rows, the `mirror = none` / `@ SKILL.md`
  pair at `:352-353`, `TASK-FORMAT.md:21-22`, `driving.md:469-479`,
  `references/finish.md:18-20` and `:75-78`, and the 19/18/27 disagreement all
  reproduce as described.
- **A sixth defect surfaced while checking finding 1, and is repaired with it.**
  `durable-artifact-set`, `plugin-prerequisite` and `build-boundary-is-the-binary`
  were `19 · step:Execute` while owned by `references/grove.md`, which rule 5 (now 6)
  never produces — the function derived *no* owner that was `grove.md`. Fixed by the
  `context` occasion, symmetric with `launch` → `driver.md`. Recorded as part of
  finding 1 rather than separately: same cause, an input model that cannot express a
  real rule.
- **`Occasion` becomes a set, not a sixth enum member.** A cross-step trigger has no
  principal step, so recording one is the unrecorded judgement the pair exists to
  remove. Tie-break is *earliest step in loop order*, justified by the same argument
  that puts `bootstrap-order` at `orientation`: a rule must be held from the first
  moment it can apply.
- **The artifact domain stays five, and is closed by what format files exist.** A
  research doc is durable but has no format file, so rule 3 has nothing to name.
  Rule 2 was masking the invalid value for both affected rows, which is why the
  owners were right while the schema was not.
- **Reachability records an edge and asserts both ends.** The old form asked only
  whether the `@` file loads. Added: the source must literally name the owner's path,
  and every non-static owner needs an incoming edge. Two schema checks fall out —
  `@ SKILL.md` implies class `trigger`, and every `trigger` row's sentence number
  must exist. Verified mechanically: 29 `@ SKILL.md` rows, 29 `trigger` rows, the
  same 29.
- **The 24 canonical trigger sentences are written into the design and measured**,
  rather than a ceiling multiplied by a guessed count. Five pairs share a sentence
  (same situation *and* same owner file); four superseded pairings split because they
  spanned two files or two situations. Measured 281 words; two of my own hand counts
  were off by one and a script caught them.
- **The word floor drops 700 → 600, and the brief records it.** The canonical
  content measures ~613. Asserting a 700 floor over it would be an instruction to
  pad, which inverts the workstream's purpose; 600 still catches a dropped row.
- **The ADR split by reversibility, not by length.** `corpus-rules-have-one-owner`
  keeps the placement input and ordered function; `restatement-declares-its-class`
  takes the three classes, the ≤25-word grammar and the new sharing test. Each
  carries its own rejected alternatives, and the spec cites both instead of
  restating either.
- **No in-session reviewer was spent** — the session's harness instructions forbid
  dispatching subagents, so the verification was done directly and mechanically.
- **A third `review-design` leaf was cut** (`rule-ownership-k16`), inserted ahead of
  `behavior-evals-k3` and scoped to this repair's delta. Reasoning: two structural
  repairs, eight P1s, and this one changed the input model again — by the brief's own
  standard for cutting `k14`, the leaves would otherwise execute against an unread
  design. Bounded by scope and by the chain's laziness rather than by a promise to
  stop.

## Notes

- Treat the line citations as anchors to the reviewed `rule-ownership-k13`
  commit. This leaf was inserted immediately after the review so no intervening
  work can silently move them.
- The findings are review output only. This leaf owns every design edit and all
  post-fix verification.

# loop-step-references-k25

**Integrates:** loop-step-references-k24

## Goal

Triage and integrate the actionable findings from the adversarial review of
`loop-step-references-k11`. Restore one accurate build-boundary statement, finish
the promised single-owner split for placement guidance and CLI mechanics, and
make the new ownership evidence reject the semantic duplicates it currently
admits.


## Context

### Findings

1. **P1 — `grove.md` turns a reported build-pairing requirement into a false
   invariant.** `content/references/grove.md:33-39` correctly says corpus edits do
   not reach a session until rebuild/install, but then says the methodology being
   read and the `grove-llm` on `PATH` both come from the launching binary. The
   governing ADR says the opposite: the skill directory and CLI are two copies
   that can quietly disagree, and Grove only reports the pairing because it cannot
   observe an opaque target's effective CLI (`docs/adr/one-build-owns-a-session.md:3-24`,
   `:75-91`). A meta-grove session can therefore ignore a real skew diagnostic on
   the strength of its own canonical procedure. Keep the inventory row's actual
   obligation — edits cross the build/install boundary — without claiming pairing
   is enforced.

2. **P1 — the review-mechanics spec still gives the placement procedure after
   claiming it only cites the owner, and the reconciled test no longer sees that
   duplicate.** `docs/specs/doubt-grove-review-mechanics.md:97-102` tells a review
   exactly when to use `leaf-insert` and which sibling entry to target; `:142-148`
   simultaneously says the rule is only in `content/references/decompose.md`.
   The spec also restates the no-exception decision at `:169-178`. Meanwhile
   `tests/composition_guidance.rs:260-269` removes the spec from the full-rule
   surfaces on the assertion that it stopped restating the rule. This leaves two
   current-state procedure statements while the test certifies one. Reduce the
   example and rejected-alternative passage to the spec-owned rationale/walk
   properties plus a citation, and add a negative assertion that rejects the
   procedure outside its owner.

3. **P1 — the new phrase sweep's “exactly three” transient-site claim is false,
   so it passes over differently worded duplicates by construction.** The test
   says the three declared `TASK-FORMAT.md` sites are exactly the later leaf's
   duplicates (`tests/rule_ownership.rs:19-27`), but its `review-budget`,
   `review-budget-predicate`, and `review-budget-by-kind` rows all declare no
   transient site (`:85-101`) while `content/TASK-FORMAT.md:94-109` restates the
   same complete allowance now owned by `content/references/execute.md:12-57`.
   The same blind spot affects differently worded statements of `pair-is-eager`,
   `name-step-kind-off-the-producer`, the bare-stem conduct,
   `no-adjacency-exception`, and `diversity-is-the-configs`
   (`content/TASK-FORMAT.md:151-180`, `:193-203`, `:251-276`, `:323-340`).
   Enumerate every intentional transient rule semantically (or narrow the test's
   claim to what it actually proves), and add a control showing a differently
   worded duplicate is rejected rather than merely an exact phrase.

4. **P2 — `decompose.md` retains the exact CLI facts it says belong only to
   `--help`.** `content/references/decompose.md:24-28` explicitly assigns what a
   verb gates and prints to the CLI, but `:148-151` still says how ambiguous
   `resolve` output behaves and that `leaf-insert` refuses an ambiguous stem.
   The handle/key recommendation is conduct worth retaining; the exact listing,
   refusal, and output behaviour are command facts that can drift. Keep the
   recommendation and point to `--help` for the mechanics.

5. **P2 — the spec attributes finish-sentinel skipping to the wrong function.**
   `docs/specs/doubt-grove-review-mechanics.md:150-160` names
   `collect_live_leaf_entries` as the walk having three properties, including
   skipping `finish` while ordinary work is live. The collector pushes every live
   leaf without inspecting its kind (`src/tree_read.rs:113-128`); the special-case
   selection is in `select_unlocked` (`src/tree_read.rs:70-97`). Name the composed
   selection seam, or assign the finish property to `select_unlocked`, so the spec
   and its test-seam guidance point at the code that actually establishes it.

### Confirmed verdicts

- Every inventory row assigned to the seven loop-step files is stated by its
  current owner, and all 26 `SKILL.md` trigger edges terminate at an owner that
  states their rule. The `finish-is-the-drivers-to-discover` move landed in
  `content/references/retire.md:18-24` and left
  `content/references/finish.md` in the same commit.
- The architecture relocation kept argument rather than rule: constraint 4's
  “just-in-time, not few” obligation remains at `content/SKILL.md:45-46`, and the
  glossary's inline/never-batched obligation remains at
  `content/CONTEXT-FORMAT.md:11-20`.
- The remaining composition-guidance reconciliations preserve their claims:
  dependency-ordered working increments remain in
  `content/references/planning.md:12-18`; retirement-before-commit remains in the
  Retire/Commit owners; diversity and filename-only retirement remain in their
  new owners; and the research trio's zero-review allowance remains in
  `content/references/execute.md:32-42`.
- The review-mechanics spec retained its ownership-predicate rationale
  (`docs/specs/doubt-grove-review-mechanics.md:34-53`), the walk properties
  (`:150-167`), task-tree access seam (`:188-214`), producer handoff (`:216-234`),
  and test seams (`:236-283`).

## Done when

- All five findings are triaged against the current sources and fixed or accepted
  visibly with a reason.
- `grove.md` describes the build boundary without claiming Grove enforces build
  pairing.
- The placement/no-exception procedure has one owner, and tests reject a semantic
  restatement outside it.
- The ownership sweep's claims and controls match the differently worded
  transient duplicates it is intended to police.
- `decompose.md` retains when/which-reference conduct without transcribing CLI
  gating or output behaviour.
- The review-mechanics spec names the source seam that actually skips `finish`.
- Relevant post-fix verification is run by this integration session.

## Notes

This review was inspection-only. It ran no test, build, lint or format command
and edited no production or test file.

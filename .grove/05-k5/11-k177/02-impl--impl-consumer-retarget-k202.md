# impl-consumer-retarget-k202

## Goal

End the dependence of Grove's tests, scripts and controls on the moving
engineering skills. Do it before those skills leave Grove. This is a
Grove-only increment. Grove still ships every moving file after it, and
`bash scripts/check.sh` stays green.

## Context

- `docs/specs/pack-membership.md`: "The Grove tests that name moving kinds",
  "The trial cases that name moving skills" and "The Grove assets of moving
  craft skills".
- `docs/repository-moves-record.md`: "Removal of the design pack" and "Asset
  classification of the design pack" as the model of a classification.
- The moving names. The kinds are `grove-impl`, `grove-prototype`,
  `grove-review-impl`, `grove-integrate-review-impl`,
  `grove-review-prototype` and `grove-integrate-review-prototype`. The craft
  skills are the six `coding-style-*` skills, `using-codebase-memory`,
  `simplify-project` and `doubt-driven-development`.

Consumers at planning. Enumerate the fifteen names again with `--hidden` over
the whole tree, with positive and cross-tree controls, before you rely on this
list:

- Named reads in `plugins/grove/conformance.test.sh`. The controls
  `unreachable`, `kind-without-load` and `two-kinds-one-rule` call
  `add_kind impl`, which copies the real `grove-impl` from the checkout.
  `two-kinds-one-rule` restates `cite-framework-decisions-to-source`, a rule
  that `grove-impl/SKILL.md` owns.
- Comments that use `grove-impl/SKILL.md` as their example: the head of
  `plugins/grove/conformance.awk` and the comment of `two-kinds-one-rule`.
- Fixture labels that stay: `directory_route.rs`, `native_skill_delivery.rs`,
  `lifecycle_cutover.rs`, `prompt.rs`, the unit tests of `tree_lifecycle.rs`,
  `grove-floor-directory-case.sh`, the fixtures of `test_harness_trial.py`
  and the fabricated map of `an_unregistered_citation_is_caught` in
  `plugin_fallback.rs`.
- Trial cases that launch `impl`. They stay unchanged, as the membership
  decision states. engineering-removal-k204 records the refusal readings.
- Named reads that the removal changes, not this leaf. One is the
  `include_str!` of `doubt-driven-development/SKILL.md` in
  `composition_guidance.rs` and `session_kind_guidance.rs`. The others are
  `MOVING_KIND_OPENINGS`, the four rows of core's manifest that the moving
  kinds own, and the kind table rows.

## Done when

- The three conformance controls take core kinds or fixtures, as the
  membership decision states. `two-kinds-one-rule` restates a rule that a
  core kind owns, for example a `grove-finish` rule in `grove-planning`.
  No control in the file copies a moving kind.
- Each changed control reads clean, and it reads dirty after a mutation.
  Record both readings.
- No comment uses a moving file as its example.
- A scratch copy of the working copy, with the fifteen moving directories
  removed, runs `bash scripts/check.sh`. Each failure is in a class that
  engineering-removal-k204 owns, as the list above states. This leaf fixes
  a failure in another class when it is a named read. Otherwise, write the
  failure into the removal leaf's consumer list. Record the failures by test
  and class.
- `docs/repository-moves-record.md` gains an "EngineeringSkills" section. It
  holds the classification of each Grove file that names a moving skill, by
  the three test classes and the three asset tests.
- `bash scripts/check.sh` passes on the Grove line.

## Notes

The scratch run proves that nothing else in Grove reads a moving file. It
also proves that the walks of core's shipped skills keep their floors.
Remove the directories only in the scratch copy. Run it outside the Grove
working copy, so that the jj working copy does not snapshot that change.

## Decisions (running log)

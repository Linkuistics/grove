# grove-skill-mandate-k22

## Goal

Make Grove's named-skill mandate compatible with mandatory guidance, external
native namespaces and a missing skill, using the existing Ravel 1.1 lifecycle.

## Context

Read the delivery specification's launch/bootstrap contract. Its reference to
the directory design at Ravel `b93bb927` is historical until
ravel-rhai-skill-delivery-k34 and grove-rhai-skill-delivery-k35 repair it.
This leaf implements only the route-independent mandate and Ravel 1.1 fixtures.
Recheck `grove-loop` prompt composition, tests, lifecycle/result/epoch fixtures
and source-exact launch/prompt excerpts named by the delivery evidence map.

## Done when

- Name the required bare `grove-<kind>` target. Where a native namespace is
  required, refer to that skill's actual installed namespace without a fixed
  `grove:` alias, registry or kind-to-plugin map.
- Qualify first with "after any mandatory guidance" and include the missing or
  unreadable skill outcome in the mandate itself: report it, keep the task live,
  record no teardown, and run this launch's exact force-stop command last.
- Verify caller text and canonical ending commands, open custom kinds, stable
  handles and normal continuation. In disposable lifecycle sessions use a
  managed fixture to supply the missing/unreadable-guidance report and exact
  force-stop result. Verify this leaves the leaf live,
  records no teardown and does not relaunch or publish successful work.
  This proves Grove's interpretation only. common-loading-acceptance-k30 owns
  actual Claude Code and Codex native-only missing-kind observations.
- Keep native-only success independent of configured Ravel sources. Treat a
  plugin-qualified name as a name request, not an asserted native tool call.
  Actual directory name/mandatory-order trials remain scheduled later.
- Update affected caller/architecture/glossary documentation and source-exact
  books with the prompt change. Run focused prompt/lifecycle/result checks,
  affected book checks and the full repository check with explicit Ravel 1.1.

## Notes

No new stop CLI, delivery preflight or kind redesign belongs here. Preserve the
ending mechanism supplied by every live launch. Native-only standalone read
access is a separate measured case, not implied by this lifecycle proof.

## Decisions (running log)

- The missing-guidance rule rides part 1 of the prompt, beside the load
  instruction, so that it reaches the agent before any load can fail. It
  carries the launch's exact force-stop command. Part 3 keeps that command's
  mechanism and result meaning.
- The measured sentence "Your kind is ...; Grove resolved that before this
  session existed" stays byte-identical. New sentences follow controlled
  English.
- The `grove-` skill-name prefix stays a naming convention of kind skills.
  `PLUGIN` becomes `SKILL_PREFIX`, because no plugin namespace reaches the
  prompt any more. The marketplace-namespace test becomes a claim that the
  prompt spells no qualified alias, with a control.
- The lifecycle proof lives in `crates/grove/tests/native_skill_delivery.rs`.
  A managed session reads its skill name, ending and force-stop commands out of
  the mandate text. It uses a custom kind, so open kinds and the missing case
  share one fixture. A repaired rerun proves the stable handle and normal
  continuation.
- The fixture session caps itself at four launches. Without the bound, a
  session that continues with the leaf live is relaunched without end, so a
  broken interpretation hung the suite rather than failing it.
- Chapter 18 of the grove-loop book now reproduces the new source and states
  re-enumerated figures for this root. Figures that chapters 11 to 17 measured
  in their own copies stay as historical readings, with the removed
  marketplace test dated beside them.

## Verification

- `GROVE_TEST_RAVEL=/opt/homebrew/bin/ravel bash scripts/check.sh` (Ravel
  1.1.0): 19 of 20 checks passed, including 1,327 nextest tests and every book.
  `Ravel caller assets and provenance` failed before any test, because the
  global mise configuration now resolves Bun 1.4.4 and the typecheck pins
  1.4.3. The rerun of that check with `mise exec bun@1.4.3` exited 0. No source
  changed between the two runs.
- Mutations: removing the force-stop substitution, restoring the `grove:`
  alias, making the fixture continue instead of force-stopping, and dropping the
  rule from the prompt each failed the intended tests.
- No review leaf is cut. The reviewed delivery design specifies the wording,
  and executable seams with mutation controls cover the behaviour. Actual
  harness behaviour with a missing kind belongs to common-loading acceptance.

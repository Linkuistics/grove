# modular-audit-k17

**Reviews:** modular-audit-k16

## Goal
Adversarially review the completed modular-only configuration subsystem and its
forms/prose audit against the root and ancestor acceptance criteria.

## Context
The producer commit is the review boundary, but its diff is predominantly
prose. Inspect the cumulative removal delivered by modular-input-k11,
modular-parser-k13 and modular-types-k15, plus earlier fixture/example migration.
Find those commits by their stable handles; do not restrict review to k16's diff.
`docs/configuration-forms-audit.md` records the enumeration, classifications and
controls. The durable grammar is `docs/specs/modular-configuration.md`; authority
is also in the complete-session-configuration and untracked-configuration-delta
ADRs. The public type contract is in `docs/specs/module-decomposition.md`.

## Done when
- Review both Catalog and Templates loading: reject flat and mixed documents in
  either source, including grammar-word keys; retain useful source/span/remedy
  diagnostics and unchanged input. Check empty documents and base-only modular
  behavior, plus validation of dormant structure versus effective semantics.
- Try to break active personal authority, local selection precedence, repeated
  profile/include order, parameter specificity and unset inheritance. Verify that
  invalid configuration cannot bypass existing tree-mutation/launch load points.
- Check expansion and inspection agree after capture and source removal: argument
  boundaries, native runtime values, required values and NUL rejection; command,
  binding, route, parameter and selection origins and overwritten histories.
  Look for surviving compatibility machinery without a modular consumer.
- Challenge the forms audit's scope, controls and classifications. Read current
  examples and the book summaries as well as exact fragments; validation of bytes
  alone does not establish correct prose. In particular, source() names the
  personal definition and inspect() carries local contributions. Check packaged
  examples and preservation of previously installed files.
- Record actionable findings against current source and the contract. Preserve
  the agreed public load/expansion test seam; do not demand a new CLI/fake-agent
  acceptance suite merely because the removal is broad. Existing checks passed.
- Grow an integrate-review-impl sibling only for findings worth acting on.
  Otherwise retire and check the parent chain against its Done when conditions.

## Notes
This is the scheduled subsystem review under Grove's load-bearing-artifact
threshold. No competing in-session reviewer ran. Follow the review skill's
inspection-only contract; any needed fixes belong to integration.

# design-removal-k198

## Goal

Remove the three design kinds and the four design craft skills from Grove.
Reconcile every Grove consumer in the same change, so that Grove stays green
with DesignSkills as the owner. Then publish the second pack commit, which
records the removal commit. This is the contraction step of the move.

## Context

- `docs/specs/external-pack-contract.md`: "Rule placement", "The kind table",
  "Manifest changes", "The checker without a copy" and "Distribution".
- `docs/specs/pack-membership.md`: "The Grove assets of moving craft skills",
  "The Grove tests that name moving kinds", "The trial cases that name moving
  skills" and "The acceptance of each move".
- The first published pack commit, its tree ID and the aggregate output before
  the removal, from design-pack-repository-k197.
- The refusal check without a launch, from trial-supplier-refusal-k196.
- requirements-removal-k192 as the model of a removal change.

Consumers at planning. Enumerate the names of the seven moving skills and
their resource files again, with positive and cross-tree controls, before you
rely on this list:

- The spine: the rows `design`, `review-design` and `integrate-review-design`
  of the kind table in `TASK-FORMAT.md` name `DesignSkills`. The citations of
  `decision-records` and `codebase-design` in `ADR-FORMAT.md`,
  `SPEC-FORMAT.md`, `references/grove.md` and `references/review.md` stay.
  They are bare names with a local fallback.
- `plugins/grove/conformance/rules.tsv`: remove the rows whose owner is in
  `grove-design/`. The prefix sets stay.
- `plugins/grove/conformance.test.sh`: the `two-kinds-one-rule` control stops
  adding the design kind. It takes a second core kind, or a fixture.
- `crates/grove/tests/plugin_fallback.rs`: the three design openings leave
  `MOVING_KIND_OPENINGS`. The `CITATIONS` row for `grove-design/SKILL.md`
  leaves, because the pack's manifest now pins it.
- `docs/adr/a-skill-states-what-binds-without-its-dependencies.md`: its last
  paragraph states that the registry covers core and that each pack pins its
  own citations.
- `crates/grove/tests/native_skill_delivery.rs` names `grove-design` in a
  temporary tree. Check that it is a fixture label, which stays.
- `plugins/linkuistics`: remove the four craft skills and their sections of
  `PROVENANCE.md`. Its manifest description and keywords, and its marketplace
  description, then name only what it still holds.
- Distribution: a `git-subdir` entry for `design` in
  `.claude-plugin/marketplace.json` that pins the first pack commit. Update
  the marketplace description, which counts and names its plugins.
- Some documents state which pack ships a kind or a craft skill. They are
  `README.md`, `plugins/grove/README.md`, `CONTEXT.md` and `CONTEXT-MAP.md`.
  They are also `plugins/CONTEXT.md` and `docs/ARCHITECTURE.md`. `plugins/README.md` gains
  a `design` section and changes its install lists and `--pack`.
  `docs/USAGE.md` gains a `design` source with both roots on the directory
  route.
  `docs/ordinal-fs-tree/CLI.md` names `cli-tool-design`. Classify it by the
  asset rule.
- History and evidence stay: `CHANGELOG.md`, the decision records' accounts,
  the move specifications, the acceptance record, the trial cases, the
  formal-methods records, `docs/ravel-caller-surface-inventory.tsv` and the two
  inventory files.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim.
- The design labels, leaf names, policy routes and `grove-llm` behaviour are
  unchanged. The label tests take the design labels from the external row,
  and they pass.
- Core conformance passes alone. The aggregate in `task check:packs` passes
  with the pinned first pack commit. In that aggregate, the core rows of
  `review-*` and `integrate-review-*` bind the pack's two family members. The
  format files that the moved kinds name resolve in the spine. Record the
  counts of core alone and of the aggregate beside the counts before the move.
- The changed `two-kinds-one-rule` control reads clean, and it reads dirty
  after its mutation.
- The refusal check of k196 refuses a case that requires `decision-records`
  at the removal commit. The same case passes the check at the commit that its
  record names. Record both readings.
- In an isolated home, `plugins/install.sh --pack <DesignSkills checkout>/plugins/design`
  links the three kinds and the four craft skills. The checkout is a plain
  checkout of the published pack commit. The owner's real links are unchanged.
- `plugins/linkuistics` no longer holds the four craft skills. Its manifest,
  its marketplace entry and its provenance file agree with what it still
  holds. `claude plugin validate plugins/linkuistics` passes.
- `bash scripts/check.sh` passes on the Grove line. Record the removal commit
  and the Ravel build.
- Push the line to the bookmark `gh-issue-25` on Grove's remote. Then publish a
  second pack commit whose `GROVE_COMMIT` is the removal commit. Its
  `plugins/design` tree ID equals the first pack commit's. Its full
  `task check` passes from a clean checkout, with the committed public URL and
  no override.
- `docs/repository-moves-record.md` records the removal commit, both pack
  commits, the pushed Grove commit, the check results and the asset
  classification. Its section "For discovery-rollout-k6" adds the native step
  `/plugin install design@linkuistics`.

## Notes

The marketplace pins the first pack commit, as the research and requirements
moves did. Both pack commits deliver the same bytes. This workstream reaches
`main` only in finish, so the owner's native marketplace does not change here.

`simplify-project` and `authoring-conventions` stay in `plugins/linkuistics`
for later moves. They cite moved skills. Each citation must be a bare name and
not a path, so it stays correct across packs.

Keep the documentation in the change that it describes. If the change is too
large for one session, decompose it. A natural seam is the craft removal from
`plugins/linkuistics` beside the kind removal. The second pack commit then
records the later of the two Grove commits.

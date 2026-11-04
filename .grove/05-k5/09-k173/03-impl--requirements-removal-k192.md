# requirements-removal-k192

## Goal

Remove the three requirements kinds from Grove and reconcile every Grove
consumer in the same change. Grove then stays green with RequirementsSkills as
the owner. Then publish the second pack commit, which records the removal
commit. This is the contraction step of the move.

## Context

- `docs/specs/external-pack-contract.md`: "Rule placement", "The kind table",
  "Manifest changes", "The checker without a copy" and "Distribution".
- `docs/specs/pack-membership.md`: "The Grove tests that name moving kinds"
  and "The acceptance of each move".
- The first published pack commit, its tree ID and the aggregate output before
  the removal, from requirements-pack-repository-k191.
- grove-research-removal-k158 as the model of a removal change.

Consumers at planning. Search again with positive and cross-tree controls
before you rely on the list:

- The spine: the `requirements` row of the kind table in `TASK-FORMAT.md`
  names `RequirementsSkills`. The sentence that Grove spells `requirements`
  for root scaffolding stays, because the label is unchanged.
- `plugins/grove/conformance/rules.tsv`: remove the rows that moved. The
  prefix sets `review-*` and `integrate-review-*` stay, and core keeps their
  rows.
- `plugins/grove/conformance.test.sh`: the `inherited-load` control reads
  `grove-requirements/grilling.md`. It takes a fixture kind with a supporting
  file, because no core kind keeps one.
- `crates/grove/tests/plugin_fallback.rs`: the three requirements openings
  leave `MOVING_KIND_OPENINGS`. Check the doc comment that names
  `grilling.md`.
- Distribution: a `git-subdir` entry for `requirements` in
  `.claude-plugin/marketplace.json` that pins the first pack commit. Update
  the marketplace description, which counts and names its plugins. Check
  `plugins/README.md` and the installer guidance for `--pack`.
- Documents: `plugins/grove/README.md` (the inventory and the kinds' own
  resources), `CONTEXT.md`, `plugins/grove/skills/grove/CONTEXT-FORMAT.md`,
  `docs/USAGE.md` (its link to `grilling.md` becomes a public URL),
  `docs/ARCHITECTURE.md` and `plugins/CONTEXT.md`. Each history record, such
  as `CHANGELOG.md` and the decision record's account of the old
  `references/requirements.md`, stays as history.
- `docs/evaluations/owner-skill-delivery/inventory.py` and
  `docs/owner-skill-inventory.tsv` are the baseline evidence. They stay.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim.
- The three requirements labels and root scaffolding are unchanged. Leaf
  names, policy routes and `grove-llm` behaviour are also unchanged. The label tests take
  the requirements labels from the external row, and they pass.
- Core conformance passes alone. The aggregate in `task check:packs` passes
  with the pinned first pack commit. In that aggregate, the core rows of
  `review-*` and `integrate-review-*` bind the pack's two members. The format
  files that the moved text names resolve in the spine. Record the
  counts of core alone and of the aggregate beside the counts before the move.
- The changed `inherited-load` control reads clean on its fixture and dirty
  after its mutation.
- A documentation sentence that tells a user what a fresh grove's first leaf
  needs names RequirementsSkills, where such a sentence exists.
- `bash scripts/check.sh` passes on the Grove line. Record the removal commit
  and the Ravel build.
- Push the line to the bookmark `gh-issue-25` on Grove's remote. Then publish a
  second pack commit whose `GROVE_COMMIT` is the removal commit. Its
  `plugins/requirements` tree ID equals the first pack commit's. Its full
  `task check` passes from a clean checkout, with the committed public URL and
  no override.
- `docs/repository-moves-record.md` records the removal commit, both pack
  commits, the pushed Grove commit and the check results.

## Notes

The marketplace pins the first pack commit, as the research move did. Both
pack commits deliver the same bytes. This workstream reaches `main` only in
finish, so the owner's native marketplace does not change here. A native user
needs `/plugin install requirements@linkuistics` after finish. Add that step
to the moves record for discovery-rollout-k6.

Keep the documentation in the change that it describes. If the change is too
large for one session, decompose it.

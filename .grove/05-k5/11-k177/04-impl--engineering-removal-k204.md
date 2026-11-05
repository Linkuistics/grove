# engineering-removal-k204

## Goal

Remove the six engineering kinds and the nine engineering craft skills from
Grove. Reconcile every Grove consumer in the same change, so that Grove stays
green with EngineeringSkills as the owner. Then publish the second pack
commit, which records the removal commit. This is the contraction step of the
move.

## Context

- `docs/specs/external-pack-contract.md`: "Rule placement", "The kind table",
  "Manifest changes", "The checker without a copy" and "Distribution".
- `docs/specs/pack-membership.md`: "The Grove assets of moving craft skills",
  "The Grove tests that name moving kinds", "The trial cases that name moving
  skills" and "The acceptance of each move".
- The EngineeringSkills section of `docs/repository-moves-record.md`. It holds
  the consumer classification and the scratch-removal failures of
  impl-consumer-retarget-k202. It also holds the first pack commit, its tree
  ID and the aggregate output before the removal, from
  engineering-pack-repository-k203.
- design-removal-k198 and "Removal of the design pack" in the moves record as
  the model of a removal change.

Consumers at planning. Enumerate the fifteen skill names and their resource
file names again, with positive and cross-tree controls, before you rely on
this list:

- The spine: the rows `impl` and `prototype` of the kind table in
  `TASK-FORMAT.md` name `EngineeringSkills`. The label tests take those
  labels from the external rows.
- `plugins/grove/conformance/rules.tsv`: remove the four rows whose owner is
  a moving kind. The prefix sets stay.
- `plugin_fallback.rs`: the six engineering openings leave
  `MOVING_KIND_OPENINGS`. The four editorial openings stay.
- The doubt assertions in `composition_guidance.rs` and
  `session_kind_guidance.rs`. Each test drops its read of
  `doubt-driven-development/SKILL.md` and keeps its assertions over Grove's
  own files. The pack's doubt check of k203 then owns the skill's half.
- `plugins/linkuistics`: remove the nine craft skills and their sections of
  `PROVENANCE.md`. Its manifest description and keywords, and its
  marketplace entry, then name only what it still holds.
- Distribution: a `git-subdir` entry for `engineering` in
  `.claude-plugin/marketplace.json` that pins the first pack commit. Update
  the marketplace description, which counts and names its plugins.
  `task check:packs` reads that pin.
- Documents that state which pack ships a moving kind or craft skill:
  `README.md`, `plugins/grove/README.md`, `plugins/README.md`, `CONTEXT.md`,
  `CONTEXT-MAP.md`, `plugins/CONTEXT.md`, `docs/ARCHITECTURE.md` and
  `docs/USAGE.md`. `plugins/README.md` gains an `engineering` section with
  the craft table. `docs/USAGE.md` gains an `engineering` source with both
  roots on the directory route.
- The two walkthrough chapters of `docs/walkthroughs/grove-loop/` that name
  `grove-impl`. Classify them by the asset rule. A chapter that states where
  core ships the kind is a stale location claim.
- History and evidence stay: `CHANGELOG.md`, the decision records' accounts,
  the move specifications, the acceptance record, the trial cases, the
  formal-methods records, `docs/ravel-caller-surface-inventory.tsv` and the two
  inventory files.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim.
- The engineering labels, leaf names, policy routes and `grove-llm` behaviour
  are unchanged. The label tests take those labels from the external row, and
  they pass.
- Core conformance passes alone. The aggregate in `task check:packs` passes
  with the pinned first pack commit. In that aggregate, the core rows of
  `review-*` and `integrate-review-*` bind the pack's four family members.
  Record the counts of core alone and of the aggregate beside the counts
  before the move.
- In a scratch copy of the pinned pack, the four family members stop loading
  `grove`. The aggregate then fails, and each failure names a member. Record
  the reading.
- Grove's suite no longer reads `doubt-driven-development/SKILL.md`. Each
  changed doubt test keeps its assertions over Grove's files. The pack's doubt
  check passes against the removal commit.
- The refusal check of trial-supplier-refusal-k196 refuses a case that
  launches `impl` at the removal commit. The same case passes the check at
  the commit that its record names. Record both readings.
- In an isolated home, `plugins/install.sh --pack <EngineeringSkills checkout>/plugins/engineering`
  links the six kinds and the nine craft skills. The checkout is a plain
  checkout of the published pack commit. The owner's real links are
  unchanged.
- `plugins/linkuistics` no longer holds the nine craft skills. Its manifest,
  its marketplace entry and its provenance file agree with what it still
  holds. `claude plugin validate plugins/linkuistics` passes.
- `bash scripts/check.sh` passes on the Grove line. Record the removal commit
  and the Ravel build.
- Push the line to the bookmark `gh-issue-25` on Grove's remote. Then publish
  a second pack commit whose `GROVE_COMMIT` is the removal commit. Its
  `plugins/engineering` tree ID equals the first pack commit's. Its full
  `task check` passes from a clean checkout, with the committed public URL and
  no override.
- `docs/repository-moves-record.md` records the removal commit, both pack
  commits, the pushed Grove commit, the check results and the asset
  classification. Its section "For discovery-rollout-k6" adds the native step
  `/plugin install engineering@linkuistics`.

## Notes

The marketplace pins the first pack commit, as the earlier moves did. Both
pack commits deliver the same bytes in `plugins/engineering`. This workstream
reaches `main` only in finish, so the owner's native marketplace does not
change here. skill-marketplace-decision-k201 may later decide that this
workstream needs no marketplace. Until then, the pin is the input of
`check:packs`.

The trial cases that launch `impl` stay unchanged as records of their builds.
After this removal, most of them are refused at the Grove line's head. That
refusal is the designed reading, not a regression.

`simplify-project` leaves `plugins/linkuistics` here.
`authoring-conventions` and the writing craft stay for later moves. Each
citation of a moved skill in them must be a bare name and not a path.

Keep the documentation in the change that it describes. If the change is too
large for one session, decompose it. A natural seam is the craft removal from
`plugins/linkuistics`, with the doubt assertions, beside the kind removal. The
second pack commit then records the later of the two Grove commits.

This is the node's last planned leaf. The base proofs need no trial, as the
membership decision states. Its retirement closes engineering-pack-k177 if no
live leaf remains, by the four close steps.

## Decisions (running log)

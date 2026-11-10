# core-candidate-restaging-k300

## Goal

Build a new core candidate keg in the projected layout. Record core's new
digest. Rerun the staging trials that read core. The plan and the dry run
then consume this evidence.

## Context

- skill-keg-link-layout-k292 changed `plugins/grove/README.md`, so the
  `grove-core` digest differs from the staging acceptance. The section
  "Consequences for queued work" of
  `docs/homebrew-skill-delivery-evidence.md` records it.
- "Skill formulae" and "Cutover contract" of
  `docs/specs/owner-skill-delivery.md` define the candidate. It is the
  last published commit of this line that changes `plugins/grove` or
  `plugins/packs.json`.
- `task owner:candidate-keg` builds a candidate keg and compares digests
  with an `ACCEPTED` file. grove-keg-projection-k293 made it build the
  projected layout.
- The staging trials run through `task owner:staging:case`. "Rerun
  commands" of the evidence section "Staging discovery trials" names each
  one. The section "Staging acceptance" records the accepted pins, digests
  and staging paths.

## Done when

- The line that holds the candidate is pushed to its public bookmark. The
  evidence names the candidate commit and its `plugins/grove` tree ID.
- The new candidate keg holds the projected layout. Its five domain-pack
  digests equal the accepted values. Its `grove-core` digest is recorded
  as the new accepted value.
- The staging settings name the new candidate keg. Each staging trial that
  reads `grove-core` ran again and passed. The evidence names each rerun
  and states why each other trial stays valid.
- The section "Staging acceptance" names the new candidate, digest and
  staging paths. It retains the earlier values as history.
- The finish-handoff bullets of the parent brief name the new candidate
  and the new core digest.

## Notes

Do not change `plugins/grove` or `plugins/packs.json` after this leaf
without a new candidate and another rerun.

owner-switch-plan-k254 names the retained staging HOME and this candidate
keg in `accepted.staging`. owner-switch-rehearsal-k255 runs the dry run
with them.

The release bump later changes `grove --version`. discovery-rollout-k12
already owns that consequence for the finish handoff.

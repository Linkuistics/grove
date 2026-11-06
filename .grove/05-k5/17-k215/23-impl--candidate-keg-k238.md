# candidate-keg-k238

## Goal

Give the staging trials a task that builds the candidate keg. Remove the
obsolete `task owner:checkouts`. Run the actual harness trials through Ravel
with every source at a keg path.

## Context

- "Skill formulae" and "Cutover contract" of
  `docs/specs/owner-skill-delivery.md`: the candidate keg, its first three
  staging steps and the evidence of each accepted digest.
- The section "Timing" in the running log of
  `homebrew-delivery-design-k216`.
- The row "Actual harness behaviour" of "Test seams" in
  `docs/specs/codex-skill-provisioning.md`. Its harness-trial paragraphs and
  `docs/common-loading-acceptance-record.md` describe the runner,
  `scripts/harness_trial.py`.
- `task owner:checkouts` runs
  `docs/evaluations/owner-skill-delivery/checkouts.py`, which measured mise
  checkouts.

## Done when

- A Taskfile task builds the candidate keg of a named commit. It builds an
  unreleased archive and installs it by the production formula's install
  method, in a private prefix with an `opt`-shaped link. It prints the
  candidate commit and the `resourceDigest` of each of Grove's six sources.
  A second build of the same commit gives the same digests.
- `task owner:checkouts`, its script and each reference that treats them as
  current are gone. A historical record keeps its reading as history.
- Claude Code and Codex each discover an applicable craft skill from a task
  that does not name it, and read it before the governed action. Ravel
  delivers every source from a keg path, and no native copy exists. The
  acceptance record names the versions, the isolation and the candidate
  commit.
- `bash scripts/check.sh` passes.

## Notes

The staging trials themselves, the drafts and the accepted digests belong to
the staging leaves of `owner-ravel-skill-inventory-k38`. They read this task.

The candidate is the last published commit of the line that changes
`plugins/grove` or `plugins/packs.json`. A later change to either needs a new
candidate keg.

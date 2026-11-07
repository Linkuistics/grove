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

## Decisions (running log)

**Candidate commit (2026-11-07).** The candidate is `7b113a28a966`, the commit
of `homebrew-delivery-guidance-k237`. It is the last commit of this line that
changes `plugins/grove` or `plugins/packs.json`, and this leaf changes
neither. No remote-tracking branch holds it yet: `gh-issue-25@origin` is at
`4cfac6295881`. This leaf pushes nothing. The staging leaves name the
published candidate, and the task reports whether a remote holds a commit.

**The task (2026-11-07).** `task owner:candidate-keg COMMIT OUT
RAVEL_2_0_PREFIX [ACCEPTED]` runs `scripts/candidate-keg.sh`. It builds the
archive with the candidate checkout's own `release-build.sh --unreleased` for
the host target. It runs the install method of the checkout's own formula
template through `scripts/candidate-keg.rb`. It runs no formula test, because
`release:formula:test` owns that.

A private copy of the 2.0 build resolves the six sources with no settings.
The task compares each digest with the plain checkout of the same commit.
`ACCEPTED` compares the digests with an earlier `digests.tsv`, for rollout's
rebuild before finish.

**The trial front (2026-11-07).** The trials use the 2.0 build of the
default-directory runs, Ravel `ff14cba826b0`. A fresh `task install` gave
`bin/ravel` SHA-256 `3a9a9129…`, the recorded value. `builds.json` moves from
`d324668b86ca` to that build.

**The trials (2026-11-07).** The case `keg-craft-discovery` takes all six
sources from the candidate keg through the new `"keg": "grove"` source field.
Claude Code and Codex each passed once. The runner did not class the Codex
read of `cli-tool-design`, because a Python program joined its path. The
session file shows the read before the write, and the acceptance record
states that limit.

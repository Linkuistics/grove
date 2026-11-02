# acceptance-handoff-k106

## Goal

Reconcile the trial and stage evidence, run the integrated checks and close
the acceptance audit. Hand external-research-pack-k4 a reproducible working
source path.

## Context

- `docs/common-loading-acceptance-record.md`, from both harness nodes.
- `docs/grove-dual-ravel-stage-record.md` and the Ravel design evidence
  records of the local, local-sources and Linux floor stages.
- The Done when of the grove root, of common-skill-loading-k3 and of
  common-loading-acceptance-k29.
- Both acceptance tables: Grove's delivery specification and Ravel's
  `docs/specs/skill-discovery.md`.
- `docs/RELEASING.md`, the Ravel Taskfile and the Grove Taskfile.

## Done when

- The record reconciles actual confinement on macOS and Linux. It states
  which backend each result used and which result came from a real harness.
- The record compares the native-only lifecycle with native standalone access
  on each route.
- The record reconciles the absence of native mutation on fresh install and
  on upgrade. It also reconciles the dual-major archive, formula and floor
  evidence.
- This leaf reviews the preceding evidence first and decides which checks need
  another run. The record states each decision and its reason.
- The applicable Ravel checks, Grove core conformance and Grove's full
  `bash scripts/check.sh` pass against the integrated subjects. The record
  names each subject by commit and build identity.
- The record holds the completion audit of common-loading-acceptance-k29 and
  common-skill-loading-k3. Each criterion names its owning commits and its
  evidence.
- The record gives research reproducible temporary recipes. They cover the
  install, the settings, the policy and the sources, with no unpublished or
  unrecorded shared configuration.
- Each named gap becomes precise repair work before research, or a scope
  question to the human that leaves this subtree live.

## Notes

Transcript availability, a prompted retry or a fixture total never replaces a
required behaviour.

Do not publish a release, install `ravel@2` or change the owner's settings.
Feature publication follows the compatible tap sequence of
discovery-rollout-k6. The staged build must stay reproducible until then.

A real Claude Code or Codex call on Linux needs a Linux sign-in. If none
exists, the record states the Linux real-harness results as gaps, separately
from the Linux fixture results.

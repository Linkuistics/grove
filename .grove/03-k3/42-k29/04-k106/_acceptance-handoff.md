# acceptance-handoff-k106 — brief

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

## Decomposition

acceptance-handoff-k106 was too big for one session, so it became this node.
The children run in this order:

1. `acceptance-gap-triage-k124`: classify each named gap of the trial record.
   Record the decision for each gap. Cut each repair leaf, or ask the human a
   scope question.
2. `stage-evidence-reconciliation-k125`: reconcile the stage evidence of
   confinement, the native-only lifecycle, native mutation and the dual-major
   archives. Decide which stage checks need another run.
3. `integrated-acceptance-checks-k126`: run the applicable Ravel checks, core
   conformance and Grove's full check against identified subjects.
4. `acceptance-completion-audit-k127`: audit k29 and k3 criterion by
   criterion, and write the research recipes.

A repair leaf goes before `stage-evidence-reconciliation-k125`. Then the
checks and the audit measure the repaired state. A scope question that the
human has not answered keeps this node live.

The triage cut three repair leaves:

- `confined-harness-trials-k128`: start both harnesses under Ravel's
  confinement, and run the two confined cases again.
- `loader-report-wording-k129`: make the preamble require a report after a
  native loader error, and run the two explicit cases again.
- `claude-preamble-loss-trial-k130`: produce the loss of the whole preamble in
  Claude Code, and observe recovery.

confined-harness-trials-k128 cut four more leaves, after the human's
decisions on its two scope questions:

- `standalone-default-kind-k131`: give `grove run` a default kind
  `standalone` outside an active grove. The kind requires no skill. Remove the
  missing-guidance promise of a standalone run.
- `ravel-token-handoff-k132`: design how Ravel hands a confined Claude Code
  a token, with no Keychain grant.
- `ravel-token-handoff-k133`: implement that handoff on the Ravel bookmark.
- `claude-confined-rerun-k134`: run both Claude Code confined cases again with
  the handoff.

loader-report-wording-k129 changed the preamble's report wording and cut one
more leaf, after the human's decision on its scope question:

- `claude-visible-report-k135`: make Claude Code show the loader report as
  visible text before the force-stop command. Then run both Claude Code
  explicit cases again. In k129 each agent wrote its report only in a thinking
  block.

claude-visible-report-k135 changed Ravel's report wording again. Then
`explicit-native-only` passed for the row in Claude Code, and
`explicit-native-error` stayed a gap. After the human's decision on its scope
question, it cut one more leaf:

- `claude-force-stop-report-k136`: make Grove's force-stop instruction require
  a visible reply that states the reason, before the force-stop call. Then run
  `explicit-native-error` in Claude Code again. A failure stays an explicit
  gap, with no further repair.

claude-force-stop-report-k136 changed Grove's force-stop wording, and then
ran `explicit-native-error` once more in Claude Code. The agent again wrote
its report in a thinking block, directly before the force-stop call. The
case is now an explicit gap in Claude Code, with no further repair, as the
human decided in claude-visible-report-k135.

claude-preamble-loss-trial-k130 ran the five-file variant
`compaction-preamble-five`. The gate failed again. Claude Code restored both
mandatory snapshot files, and none of the five work files, which the agent
wrote through Bash heredocs. After the human's decision on its scope question,
it cut one more leaf:

- `claude-preamble-write-tool-trial-k137`: run the variant again, with the
  five work files written by the Write tool. A failure makes the Claude Code
  `compaction-preamble` row an explicit gap, with no further repair.

claude-preamble-write-tool-trial-k137 ran the variant
`compaction-preamble-write`. Its gate passed, because Claude Code restored
the five work files that the Write tool wrote. The agent then found its
guidance through the transcript path and a search of the home directory, not
through the variable. The Claude Code `compaction-preamble` row is now an
explicit gap, with no further repair, as the human decided in
claude-preamble-loss-trial-k130.

ravel-token-handoff-k132 found that its first route conflicts with Ravel's
credential rule and with Anthropic's terms. Ravel would have read the owner's
sign-in. On 2026-11-03 the human chose another route.

The owner mints a token with `claude setup-token` and names its file in
Ravel's settings. Ravel sends the token through a socket to a confined harness
that the configuration script names. The specification is Ravel's `docs/specs/token-handoff.md`, in commit
`7c5b6cdd3842` on the bookmark. A review chain checks it before
`ravel-token-handoff-k133`.

ravel-token-handoff-k138 reviewed that design and recorded five findings.
ravel-token-handoff-k139 applied them, with the real findings of its
in-session reviewer. The integrated design is Ravel commit `9084e0dcfcd7` on
the bookmark. Several of its changes are new rules, so it cut one more leaf:

- `ravel-token-handoff-k140`: review the integrated design before
  `ravel-token-handoff-k133` implements it.

The "Token handoff design integration" section of
`docs/common-loading-acceptance-record.md` holds each disposition and the
verification limits.

ravel-token-handoff-k140 reviewed the integrated design and recorded one
actionable finding. ravel-token-handoff-k141 integrates that review before
ravel-token-handoff-k133 implements the design. The "Token handoff integrated
design review" section of `docs/common-loading-acceptance-record.md` holds
the finding, the other dispositions and the evidence limits.

standalone-default-kind-k131 gave `grove run` the default kind `standalone`
outside an active grove. Inside one, a run without a kind takes the next live
leaf's kind, by the human's decision. The Confined standalone row keeps its
sandbox part as an obligation. Its agent part is an explicit gap, with
`missing-kind-confined-20261103T062817Z` as its evidence.

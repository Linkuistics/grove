# harness-selection-and-execution-k1

## Goal

Resolve the remaining requirements for the separate harness-selection and
execution tool, then establish the design/planning work needed to deliver it.

## Context

The root brief preserves the prior discussion and its constraints. The research
at `docs/research/grove-model-effort-routing.md` surveys current model/effort
results, the Grove integration seam, and Jev/oMLX options. The new
`configure-grove` skill supplies the configuration-maintenance workflow.

## Done when

- The independent executable's first deliverable, ownership and integration
  boundaries are agreed with the human.
- Static/computed selection, task context, provider provenance and error/fallback
  behavior have concrete acceptance criteria.
- Test seams are agreed and durable requirements are recorded in the appropriate
  project documentation.
- The next design/planning work is cut at useful session boundaries, with review
  scheduled where needed and no duplicated interrogation of settled requirements.

## Notes

This is the initial requirements leaf, prepared for a fresh Grove session.
The preceding session released the configuration skill and applied personal
model/effort defaults; it did not implement the router or resolve the open
requirements. Do not treat the research's proposed flags or TypeScript types as
an existing API.

## Decisions (running log)

Repository boundary: keep the independent executable in this repository for
now, as a separate package, for ease of development, testing and initial
deployment. The human agrees that a separate repository is the eventual
direction. Preserve a package/interface boundary that allows later extraction;
do not make ordinary use depend on Grove or move the tool to another repository
in this increment.

Initial delivery: ship the new command alongside Grove through Grove's existing
installation and release. A separate opt-in installation is not required for
the first deployment; the independent package boundary remains in place for
later extraction.

Review reference: compare the reviewer with the artifact's primary producer.
Supporting contributions from other producers do not accumulate into an
ever-growing exclusion set. The human rejected excluding every contributing
provider because the set becomes unbounded as more producers contribute.

Review separation: the reviewer must use both a different underlying model and
a different provider from the artifact's primary producer, using that producer's
actual execution provenance. A different model within the same provider is not
sufficient. This applies to explicit choices, retries and fallbacks; comparison
does not expand to supporting producers.

Policy ownership and provenance discovery: the provider/model separation rule
belongs in configuration, either static configuration or TypeScript computation,
and is not a review-specific rule hard-coded into the generic selector
executable. TypeScript may include the rule in a prompt to a selection model.
Artifact-associated provenance must record the producing provider/model identity
in a form TypeScript can discover when choosing a reviewer; current launch
configuration cannot establish which model previously produced an artifact.
The provenance representation and primary-producer association remain to be
settled. The earlier proposal that the review request itself defines the primary
producer has not been accepted.

First-version simplification (supersedes the stronger review identity decisions
above): require only a different provider from the artifact's original creator,
with the rule owned by configuration. The human asked not to overdevelop this
rare case. Keep provenance sufficient for policy to discover that original
provider; do not require a separate model-inequality check or multi-author
accounting for the first increment. The representation can be settled during
design without extending this interview over provenance edge cases.

Configuration trust: load personal policy by default. Execute repository policy
only when explicitly selected through personal configuration or a `--config`
argument. Merely cloning or entering a repository must not execute its
TypeScript. The human accepted this trust boundary for the first version.

First-release scope: deliver static routing plus a programmable TypeScript
selection interface, with execution and outcome records ready for evaluation.
The human accepted deferring a working local LLM selector and its evaluation
pilot to follow-up work. The first release must be useful without local
inference, Jev integration, model downloads or a calibration campaign.

Incomplete selection stops: the human rejected automatic fallback and prefers
to stop when the mapping is incomplete. The first-release policy must report
missing or incomplete mappings and launch nothing; it must not fill a policy
gap by silently substituting a static default or another candidate. A failed
computation that produces no valid selection is likewise an unresolved launch,
not permission for the executable to invent a choice.

Agreed test seams: exercise the new command with temporary TypeScript policies
and fake harness executables, covering selection, generic context delivery,
inspection, provenance and refusal on incomplete mappings. Exercise Grove's
existing launch boundary with integration tests for unchanged prompts, the
selected kind/task context, terminal and cancellation behavior, exit status and
completion authority reaching only the final harness. The human confirmed
these seams match expectations.

Final confirmation: the human explicitly agreed to the consolidated requirements
in the root brief and durable boundary decision, including requirements review
followed by design. The interview is complete; retire and commit this leaf with
those next sessions queued, without reopening settled requirements.

## Verification

The requirements/documentation change was checked with `bash scripts/check.sh`.
Its first ten principal check groups passed, including the complete Rust test
suite. The process then ended with SIGTERM (exit 143) during the final book-check
group, after reporting four books valid; the cause was not established. A
separate rerun of the full book-check loop over every `docs/walkthroughs/*/`
directory passed all six books with `--final --check all` and exit 0. This
establishes the individual check results, not a normal exit from the full script.
The workspace snapshot stayed `ab8554531a230171f503aff657e5d0d85337d5d4` throughout
both runs. Subsequent edits record verification, human confirmation and task-tree
finalization; they do not change production code. Logs are at
`/tmp/grove-harness-selection-requirements-check.log` and
`/tmp/grove-harness-selection-book-validation.log`.

`grove config show --json` validated the active lifecycle configuration and the
routes needed for requirements review and design. Configuration inspection is
not a paid harness/model-access smoke test. No launch policy was changed.

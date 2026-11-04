# research-directory-trials-k160

## Goal

Prove that a research kind loads through Ravel directory delivery from the
external pack in real Claude Code and Codex sessions, with native copies
absent.

## Context

- `docs/common-loading-acceptance-record.md`, "Research recipes", "The runner"
  and the `discovery`, `delegate` and `problem-source-missing` cases. Use these
  cases as models.
- The external-source fields from trial-external-sources-k159.
- The Grove commit from grove-research-removal-k158 and the published
  ResearchSkills commit.

## Done when

New research cases run in both harnesses with two sources. `grove` comes
from the prepared Grove checkout at the removal commit. `research` comes from
a plain Git checkout of the published ResearchSkills commit. Each run is from a
directory outside the Grove working copy, the ResearchSkills working copy and
the checkout. Each run uses the actual session and delegate read permissions.

- A `research-a` mandate names the bare kind skill. The agent reads the kind
  skill, the family procedure in the pack and the spine before it acts.
- A delegate reads a research resource from the retained directories.
- A case with the ResearchSkills working copy as a `local` source loads the
  same kind.
- A moved checkout refuses as `skill_source_missing` before any worker. A
  missing root refuses in the same way. A fixture run through the public
  `ravel start-session` boundary is enough for these refusals, because the
  refusal comes before any LLM.
- A launch without a network resolves the same directories.
- Each case section in the record names the Ravel build, the Grove commit, the
  ResearchSkills commit, each `resourceDigest` and the observations. A gap is
  a named gap with a disposition, never a pass.

## Notes

A real run spends subscription usage. Do not remove or change the owner's
native installations. The sandbox home links only each harness's sign-in
store.

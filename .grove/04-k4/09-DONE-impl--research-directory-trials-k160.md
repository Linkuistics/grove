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
  skill, the spine and the `research-family` skill before it acts.
- A delegate reads a research resource from the retained directories.
- A run with `--working-copy research=<dir>` loads the same kind from the
  ResearchSkills working copy.
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

## Decisions (running log)

- The trials take `grove` from the prepared checkout of the removal commit
  `57414e00cd876a9c0e1f8021786384ba7e368580`. They take `research` from a
  plain checkout of `1c18a90945a0b2ac4086bfe6f2ba90066fcf4bcc`, the published
  `main` of ResearchSkills. Its `plugins/research` has the tree ID
  `79dc2a530cd2`, the same as the pinned first pack commit.
- The Ravel 2.0 front is the build of `skill-directory-delivery` at
  `71f70fcd3f92`, installed into a scratch prefix with no `libexec`. Its
  `bin/ravel` SHA-256 is `26b487afb957…`, the value of `builds.json`.
- Three new cases: `research-discovery`, `research-delegate` and
  `research-source-missing`. Each research leaf asks for a survey whose
  primary sources are the installed help texts of `git` and `jj`. The survey
  then needs no network, and the Codex sandbox denies the network.
- The missing root uses a new runner field `moveRoot`, beside `moveAway`.
  The runner then moves only that root of the source. The restored
  resolution then proves the cause, as it does for a moved checkout. A
  fixture outside the runner gives no restored resolution. The runner change
  comes before every real run, so all runs measure one runner.
- The runner commit of every run is `8a93002fe9f1`. The Grove build is
  `57414e00cd87`, with `grove` SHA-256 `a7f5c988a18e…` and `grove-llm`
  SHA-256 `2f7895cc33f7…`.
- Every case passed in both harnesses, each on its first run.
  `research-discovery` read the spine, `grove-research-a` and
  `research-family` before the first action, from a plain checkout and from
  the working copy. A delegate read `grove-research-a` from the research
  snapshot in each harness. The moved checkout and the missing root each
  refused as `skill_source_missing` before any harness, and each restored
  resolution resolved. Each offline resolution gave its launch's descriptor
  identity.
- The `research` digest was `7fb3d473f66f…` in every run, the working-copy
  runs too. The working-copy change was empty, so its bytes equal the
  published commit.
- Limit, not a gap: the Claude Code parent named both research skills in its
  delegate prompt. That run proves reachability from a delegate, not
  discovery by a delegate. Codex encrypts its delegate message.
- The evidence is the record's section "Research directory trials". The raw
  transcripts stay out of the repository, and the section names their
  SHA-256 digests.

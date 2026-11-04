# requirements-directory-trials-k193

## Goal

Prove the boundary that this move adds, under Ravel directory delivery. An
external member of a cross-pack family reads its family file from the core
source before it acts. Prove it in real Claude Code and Codex sessions, with
native copies absent.

## Context

- `docs/specs/pack-membership.md`, the row "An external member of a
  cross-pack family" in "The acceptance of each move".
- `docs/common-loading-acceptance-record.md`: "Research recipes", "The runner"
  and the case `research-discovery`, as the model.
- The removal commit and the published pack commits from
  requirements-removal-k192.

## Done when

A new case runs in both harnesses with two sources. `grove` comes from the
prepared Grove checkout at the removal commit. `requirements` comes from a
plain Git checkout of the published pack commit through the case's `external`
object. Each run is from a directory outside the Grove working copy, the pack
working copy and the checkout. Each run uses the actual session and delegate
read permissions.

- A `review-requirements` mandate names the bare kind skill. The agent reads
  the kind skill, the spine and `references/review.md` from the `grove` source
  directory before it acts.
- The case section in the acceptance record names the Ravel build, the Grove
  commit, the pack commit, each `resourceDigest` and the observations. A gap
  is a named gap with a disposition, never a pass.
- `docs/repository-moves-record.md` links the case section.
- `bash scripts/check.sh` passes after the case and record are added.

## Notes

The membership specification asks for this one read. It asks for no delegate
case, working-copy case, refusal or offline launch. The research move proved
those mechanisms. The read of `references/review.md` also stands for
the format files that the moved text names. They resolve by the same rule
through the same loaded spine.

The case needs a small planted grove whose live leaf is a
`review-requirements` leaf beside a retired `requirements` leaf. The
observation is the order of the reads before the first action. The review
does not need to finish.

A real run spends subscription usage. Do not remove or change the owner's
native installations. The sandbox home links only each harness's sign-in
store.

## Decisions (running log)

- The case is `requirements-family-read`. Its sources are `grove` from the
  prepared checkout of the removal commit `302f85429e4b` and `requirements`
  through `external`, at the second pack commit
  `eb945d9b13efcf11a74c78936c802108d3f022c6`. That commit is the public
  `main`, and its `GROVE_COMMIT` is the removal commit. Its source directory
  has the tree of the first pack commit, so both pack commits deliver the same
  bytes.
- The case routes only `review-requirements`. It names no `resolveOffline`,
  because the specification asks for no offline proof. The launch view still
  names the descriptor identity and each `resourceDigest`.
- The planted tree holds the retired `requirements` leaf, the live
  `review-requirements` leaf and the paired `integrate-review-requirements`
  leaf, as a real grove holds them. The root brief is the requirements
  artifact, with one requirement that cannot be tested, "The tool is fast", so
  that the review has a real finding. The review leaf names no skill.
  `grove-llm pick` on a copy of the tree gives the review leaf and the kind
  `review-requirements`.
- The instrument was the Ravel 2.0 build of `builds.json`, installed by the
  research recipe from `71f70fcd3f92` into a scratch prefix. Its `bin/ravel`
  SHA-256 is `26b487afb957…`, and the prefix has no `libexec`. The Ravel
  workspace stayed empty. `task trial:prepare` built Grove at
  `302f85429e4b`, with the `grove` and `grove-llm` digests of the earlier
  trials.
- Claude Code run `requirements-family-read-20261104T140225Z`: pass. Call 1
  read the spine from the grove snapshot, call 4 the kind skill from the
  requirements snapshot, and call 5 `references/review.md` from the grove
  snapshot. The first command on the task came in call 7.
- Codex run `requirements-family-read-20261104T140225Z-2`: pass. Calls 2, 4
  and 5 made the same three reads, and the first command on the task came in
  call 8.
- Both runs: the `requirements` digest `07b2d25e8bd2…` and the tree
  `f3cf8128fd20…` equal the moves record's values, the checkout was clean at
  its commit after the driver, and the driver refused
  `integrate-review-requirements` as `config_refused` by design. Each review's
  first finding was the planted requirement "The tool is fast".
- The acceptance record holds the section "Requirements directory trials",
  with its limits. No gap is open. The moves record links it from the
  RequirementsSkills section.
- The controlled-English checker reports no hard finding in the new text. The
  acceptance record keeps 0 hard and 57 advisory findings, as at `a03534d2776`.
  The moves record has one hard finding at its TestAnyware section,
  `remove` beside `delete`, and the committed file has the same finding. This
  leaf leaves that text unchanged.
- Gate: `bash scripts/check.sh` passed all 21 principal checks, 1345 Rust tests
  among them, with the harness trial runner controls. The seven changed files
  outside `.grove/` had the same digests before and after the run.
- No in-session reviewer was spent. The observations come from the runner's
  per-call record, and no code changed.

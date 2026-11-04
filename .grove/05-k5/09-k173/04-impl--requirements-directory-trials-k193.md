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

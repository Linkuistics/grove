# finish-transaction-integrity-k136 — brief

**Kind:** impl

## Goal

Make transaction preparation and pending-witness recovery reject root,
manifest, and evacuated-content substitution without moving or restoring data
through an attacker-controlled path.

## Context

- The reviewed contract requires opening `.grove/` as a no-follow directory,
  identity-revalidating it, and performing transaction operations relative to
  validated directory descriptors.
- This subtree owns task-root, witness, manifest, and evacuated-content
  integrity. Partial transition recovery belongs to
  `finish-transaction-transition-recovery-k137`; repository proof through
  quarantine belongs to `finish-transaction-handoff-hardening-k138`.
- Existing process tests already cover ordinary special-file and reserved-name
  preflight refusal. Extend the transaction interface at the narrowest seam;
  do not duplicate process acceptance owned by `finish-transaction-docs-acceptance-k122`.

## Done when

- Failing tests demonstrate task-root replacement, witness/manifest symlinks,
  malformed or mismatched manifests, foreign evacuated entries, and content or
  mode tampering are rejected while the only recoverable bytes remain intact.
- Preparation creates the witness, manifest, ready marker, and recovery tree
  beneath validated no-follow directory handles and revalidates the task-root
  identity before the first source move.
- Recovery validates the exact witness and original-tree identities plus the
  manifest's canonical entry set before repository classification.
- Focused transaction tests pass without reaching through repository or
  cleanup internals.

## Notes

Fix only gaps first demonstrated by a failing test. Preserve raw Unix filename
bytes and the existing canonical digest format.

## Decomposition

- `finish-task-root-identity-k139`: carry the opened no-follow task root across
  repository preparation and reject substitutions present at the preparation
  and evacuation phase gates.
- `finish-witness-identity-k140`: create and recover the witness and original
  tree through identity-validated directory handles, rejecting substitutions
  and foreign root entries.
- `finish-manifest-integrity-k141`: validate ready/manifest objects and the
  canonical evacuated entry set before repository classification.

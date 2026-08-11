# provisioned-skill-refresh-k9

## Goal

Reconcile Grove's claim that committed `content/SKILL.md` drives the next
self-driving-loop session with the skill actually available to that session.
Prove where provisioning occurs across loop iterations, then either refresh the
skill before every launch or narrow the documented self-hosting claim to the
boundary the implementation really provides.

## Context

This surfaced while reviewing `chain-contiguity-k6`, immediately after that
producer committed the flat/lazy integration-placement guidance. The launched
session's installed skill and the committed canonical content disagree:

- `/Users/antony/.codex/skills/grove/SKILL.md` is 654 lines and still instructs
  `grove-llm leaf-add-chain` and `grove-llm leaf-promote-chain` (including at
  lines 251 and 305), verbs the grove has removed.
- `content/SKILL.md` is 723 lines, contains the flat/lazy “Cut the next step”
  discipline, and contains neither removed verb.
- `cmp` reports the files differ even though this review was launched after the
  `chain-contiguity-k6` commit.

This may be a real relaunch-provisioning defect, a deliberate one-provision-per-
bare-`grove`-invocation boundary, or a harness skill-catalog/file-refresh
boundary. Do not assume which: trace the driver and provisioning path and pin
the observed two-iteration behavior. The distinction matters in this
meta-grove because its brief treats code and methodology as coupled on the
premise that the next session consumes the just-committed `content/`.

## Done when

- The exact provisioning point is demonstrated for a bare `grove` invocation
  that relaunches at least two sessions.
- The installed skill used by the second session is compared with the canonical
  `content/SKILL.md` committed by the first, including any harness-side caching
  boundary that makes an on-disk refresh insufficient.
- If the next iteration can safely consume refreshed content, provisioning and
  tests make that true for every configured harness without changing Grove's
  one-configuration routing contract.
- Otherwise, the meta-grove/self-hosting claims in the brief, methodology, and
  docs are narrowed to the real boundary, and future work does not rely on
  same-invocation skill refresh.
- The removed review-chain verbs cannot be presented to a session launched
  after the boundary the final contract promises.

## Notes

This leaf records the discrepancy only. It does not prescribe rebuilding the
running binary, mutating personal configuration, or treating a skill catalog
captured at process start as dynamically reloadable.

# lifecycle-launch-k22

**Integrates:** lifecycle-launch-k21

## Goal

Triage and integrate the two lifecycle cutover findings before the
configuration deletion leaves build on it.

## Context

- `lifecycle-launch-k21` records F1 and F2, anchored to `14c3328f`, with
  source and test evidence. `7f0d81e4` is the companion book rewrite.
- F1: a failed-session diagnostic claims the leaf remains live even when the
  harness retired or decomposed it before failing without signalling. Keep
  truthful rerun guidance and cover retirement before failure through the real
  driver, front and worker.
- F2: `selected_root_launches_only_while_its_directory_is_current` still
  launches `/bin/sh` directly through the loop's session launcher. Preserve
  its root-removal/replacement and epoch assertions using real dispatch and
  a deterministic harness selected by a policy in a temporary HOME.
- The review found no reversal of text-before-lock. Its missing automated
  test is optional, not a required finding.
- Follow the root brief's book/ADR/spec maintenance and full-check convention
  for source and test changes.

## Done when

- Every finding is classified and the actionable ones are integrated.
- Appropriate regression checks and `bash scripts/check.sh` pass on the final
  changes; books reproducing changed source are current.
- This leaf's decisions make any rejected finding explicit.

# published-source-acceptance-k315

## Goal

Resolve every accepted skill source again with the published Ravel 2.0.0,
and record the same digests for finish's comparison.

## Context

- `docs/specs/coordinated-release.md`, step 5 of "Publication ownership and
  order".
- The parent brief, section "The finish handoff": the digest, tree-ID and
  disabled-control obligations.
- `docs/owner-skill-delivery-evidence.md`, sections "Core candidate
  restaging" and the staging trials. `docs/repository-moves-record.md`,
  section "The handoff to the owner cutover", for the ten sources.

## Done when

- The candidate keg of the final line gives the `grove-core` tree ID
  `961fddd04bb7b271e9896d07dc0225348fa8c378`, or the change is recorded with
  its consequence.
- With the published `ravel@2`, `ravel skills resolve --json` resolves each of
  the ten sources from its keg path. Each `resourceDigest` equals the
  accepted value. The accepted `grove-core` digest is
  `d8eec38844f5e8ffd4e998ef34f3456a984846166b2e34cc2c8e65189b2f145e`.
- With the off value and the installed keg sources, `ravel skills resolve --json`
  on the published `ravel@2` reports `disabled`.
- A committed section of `docs/owner-skill-delivery-evidence.md` records each
  ID, digest, keg path, the published build identity and the commands.
- A differing digest is carried as live work: that source's staging trials,
  its revised acceptance and another dry run. It is not a completion claim.

## Notes

Use the staging HOME, never the owner's real settings. Do not launch a
default session. The owner's real `~/.config/ravel` keeps `policy.ts` for the
running 1.1.0 driver.

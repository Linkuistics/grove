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

## Decisions (running log)

- The final line is `3fec15ecb950`. Its `plugins/grove` tree is
  `961fddd04bb7b271e9896d07dc0225348fa8c378`, and its `plugins/packs.json`
  blob is `2cebaf5f062d08b50b22d058bd8c4db3d412f8a6`. Both equal those of the
  candidate `2a24d5e61f29`. No later commit changes `plugins/grove`,
  `plugins/packs.json`, `scripts/templates/grove.rb.tmpl` or
  `scripts/release-build.sh`. So the candidate of the final line stays
  `2a24d5e61f29`.
- Do not change the staging runner. `staging_trial.py` and
  `scripts/harness-trials/builds.json` are runner files that the switch plan
  identifies by content. A change to either needs another dry run. The
  runner's `sources` trial also pins the earlier build `3a9a9129…`, so it
  refuses the published binary.
- Use the probe's `resolve` through `task owner:probes:resolve`, with a new
  optional `RAVEL` variable. The switch itself runs that probe. It compares
  each of the ten digests with the plan's accepted values, and it covers the
  `disabled` control with cases that must fail. The Taskfile is no runner
  file.
- Build the candidate keg again with the published `ravel@2` as its resolver,
  with `ACCEPTED` set to the retained candidate's `digests.tsv`.
- No digest differs, so no live staging, acceptance or dry-run work arises.
  The rebuilt keg stays at
  `~/.local/state/owner-staging/candidate-published-k315` as evidence only.
  The staging settings keep naming the retained candidate keg.

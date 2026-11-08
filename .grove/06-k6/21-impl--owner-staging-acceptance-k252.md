# owner-staging-acceptance-k252

## Goal

Record the accepted staging result that the real plan and
owner-pack-checkouts-k93 read, and close order 5 of the cutover contract.

## Context

- Order 5 of the cutover contract in `docs/specs/owner-skill-delivery.md`
  is done when the drafts pass under the staging HOME. Each trial passes
  or carries a recorded scope decision. The evidence names each accepted
  formula version, the candidate commit and each accepted
  `resourceDigest`.
- The parent brief's "Owner-delivery handoff" adds the tree ID of the
  `grove-core` candidate. The staging HOME and the candidate keg stay until
  the switch receipt.
- The staging leaves owner-staging-setup-k248, owner-staging-discovery-k249,
  owner-staging-hooks-k251 and owner-staging-capabilities-k250 hold the
  results.

## Done when

- One committed table names, for each of the ten sources, its formula, its
  accepted version and commit, and its accepted `resourceDigest`. It also
  names the candidate commit and the tree ID of its `plugins/grove`.
- The table names the trial commands of each source for the plan's
  `accepted.trials`, and the absolute paths for `accepted.staging`.
- Each trial of the cutover contract and each staging item of "Test seams"
  has a result or an owner-approved scope decision.
- Each item of "Not measured" in `docs/owner-skill-delivery-evidence.md`
  has a measurement, an owner, or a reason that the Homebrew change
  withdrew it.
- The staging HOME and the candidate keg still exist, and each source still
  gives its accepted digest.

## Notes

If a source changed during the staging trials, rerun the trials that read
it before you accept it. A change to `plugins/grove` or
`plugins/packs.json` needs a new candidate keg.

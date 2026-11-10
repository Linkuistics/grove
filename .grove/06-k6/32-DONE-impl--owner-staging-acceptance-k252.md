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
- The record holds the staging inventory from the corrected instrument and
  the quiet-session result of owner-staging-hooks-k251. A failed
  quiet-session result blocks acceptance until the owner resolves the
  writer.
- The record requires no rehearsal record or other admission token. The
  dry run of owner-switch-rehearsal-k255 reads these accepted values.

## Notes

If a source changed during the staging trials, rerun the trials that read
it before you accept it. A change to `plugins/grove` or
`plugins/packs.json` needs a new candidate keg.

## Decisions (running log)

- 2026-11-10: The live state matches acceptance. The retained staging HOME,
  its `.codex` tree, the candidate keg and the accepted Ravel 2.0 build
  exist. The staging settings resolve the ten accepted digests. Each skill
  keg is at its accepted version. `plugins/grove` and `plugins/packs.json`
  are unchanged since the candidate `ef8a79dd7621`, so no new candidate keg
  is needed.
- 2026-11-10: No source changed after the trials that read it. Only
  `writing` changed during the staging trials, and its source trials and
  session probes ran again. The native-residue, MCP and TestAnyware cases
  ran after that change.
- 2026-11-10: Item 5 of "Not measured" is measured. One staging Claude Code
  session in the owner's launch shape, with no permission flag and no
  `--add-dir`, ran in `auto` mode. Both snapshot reads returned content,
  and `permission_denials` was empty. The record is
  `trials/auto-mode-read-claude-1`.
- 2026-11-10: Gate 4 cannot run the staging tools as they stand. It sets
  HOME to the staging HOME, and the tools take the real HOME from HOME. Its
  trials also read the candidate keg, not the changed installed source.
  A new `staging_trial.py rerun` entry repairs both. It takes the real HOME
  from the account record. It refuses unless the candidate keg gives the
  installed digest, so an unchanged candidate stops gate 4 in native
  delivery, as step 2 of the switch requires.
- 2026-11-10: `accepted.trials` holds Grove's six sources only, because
  gate 4 reads no other source. The four other sources keep their cases for
  owner reruns through `task owner:staging:case`. `guidance-probe` is no
  source trial: it reads the native plugin from GitHub.
- 2026-11-10: The record names the runner in this workspace. Before finish,
  the main checkout lacks `rerun`, so its content would change at
  integration. owner-switch-plan-k254 may choose another stable path.
- 2026-11-10: No trial needs an owner scope decision beyond the recorded
  `quint-kb` approval. The Claude-only cases follow the specification. The
  low-effort compaction failure is a limit, not a failed trial.
- 2026-11-10: k253 and k254 received context lines for the quiet-interval
  rerun condition and the gate-4 trial commands.

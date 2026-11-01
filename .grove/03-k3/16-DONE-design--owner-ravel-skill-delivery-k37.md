# owner-ravel-skill-delivery-k37

## Goal

Design the owner's transition from current skill installations to delivery of
the full collection through Ravel, explicitly including TestAnyware.

## Context

The human authorised the actual installation migration in
common-skill-loading-k32. Read root destinations and the maintained machine
setup before configuration work. Native delivery remains a product option, but
the owner's target setup is Ravel insertion. Inventory before the pack moves.
Then give rollout a baseline against which it can reconcile the final
installed set.

## Done when

- Enumerate the owner's actual Claude Code/Codex skill installations and
  configured plugin sources, including personal, third-party and bundled skills
  outside this repository's pack list. Record source ownership, duplicate
  identities, immutable resources, supporting scripts and native discovery
  locations. TestAnyware must appear explicitly. Do not infer completeness from
  a list of known skill names or only this checkout's corpus.
- Distinguish readable skill guidance from plugin tools, app connections,
  native hooks and harness-specific capabilities. Design how every applicable
  skill is triggered and read through Ravel while its required capabilities
  remain usable. Any unsupported case is precise repair work or an explicit
  scope decision, never an omitted skill or an untested portability claim.
- Specify source roots and pin/local policy for the final repositories,
  mandatory guidance, supporting-resource lifetime, name collisions and stale
  native copies. Feed any missing directory feature requirement into k34 before
  implementation. Preserve platform-managed resources outside owner control.
- Design backup, reversible activation and removal of superseded native
  installations after accepted replacement delivery. Ownership, content and
  active-reader evidence govern removals. Retain required snapshots outside
  native discovery until their actual readers finish.
- Account for this driver using a canonical Ravel 1.1.0 binary that cannot gain
  directory insertion by a PATH upgrade. State how the final session loads its
  guidance. State how lazy reads and live native paths remain valid. State when
  new launches switch to the feature build and settings. The plan must not
  remove the only source from which Grove's finish session can load its skill.
- Give owner-ravel-skill-inventory-k38 the baseline and a concrete ordered
  cutover contract. The contract covers replacement trials on both harnesses,
  TestAnyware resource and tool use, and ordinary implicit skill discovery. It
  also covers actual owner activation, removal evidence, restart and rollback.
  Record persistent changes under the current machine configuration contract
  and update its maintained manual.
- Commit durable design and inventory evidence, record exact inputs here and
  cut a focused review-design leaf before dependent changes. No installation,
  owner configuration or deletion is performed in this design session.

## Notes

Do not stage another requirements interview over the human's settled target.
The inventory establishes what work is needed, not whether the migration is
authorised. Native plugin machinery can remain installed for tools or hooks
only if guidance delivery and duplicate discovery are explicitly reconciled.

## Decisions (running log)

- A read-only inventory on 2026-11-02 walked every Claude Code, Codex and Pi
  discovery root, both plugin registries and each installed plugin's
  components. It also read hooks, MCP entries, the owner policy and tool
  installations. No installation, setting or file outside this checkout
  changed.
- Inputs: Claude Code 2.1.316, Codex 0.166.0, Grove 22.8.0, Ravel 1.1.0 and
  TestAnyware 2.7.0. Plugin cache `238c3a839ab4` matches marketplace clone
  commit `238c3a839ab4b30b101fbe3cc5212d66091df28a`. Ravel's historical
  directory contract was read at `b93bb927`. Ravel main's configuration
  specification was read at `24a54523`.
- The owner policy routes every kind to `claude` or `codex` only, and
  `finish` goes to Claude. No policy route selects Pi, so Pi is outside the
  automatic contract. Its Linkuistics links and its extension link all
  dangle at the moved `~/Development/grove` checkout.
- Observed defects to carry forward: dangling `terminal-browser` links in
  three roots and Grove-snapshot links in both Codex user roots. Older
  superpowers copies sit in `~/.codex/skills`, and a stale `grove` copy sits
  in `~/.claude/skills`. The Codex snapshot's technical-communication and
  machine-environment bytes differ from the Claude plugin's bytes.
- Classification: owner packs and selected third-party packs move to Ravel
  sources. Platform-managed, tool-managed and capability-only entries stay
  native and unchanged. Disabled third-party plugins are outside the active
  collection. `guardrail` stays a native Claude Code skill, because a read
  cannot install its frontmatter hook or its slash command.
- Hook-bearing skills split. Ravel delivers technical-communication's
  guidance, and a skill-free native plugin keeps its hooks. A mandatory file
  cannot honour the off-file switch, so the always-on layer stays a hook.
  The superpowers `SessionStart` hook becomes one mandatory file instead.
- The owner default pins published Git revisions for every source. Local
  sources serve only trials in a separate HOME. Ravel has no garbage
  collector, so it would retain every captured working-copy edit forever.
- This driver cannot deliver directories, and finish runs on native Claude
  discovery. Superseded native entries therefore deactivate only inside
  finish, after Grove's release, promotion and the last 1.1 settings read.
  Deactivation disables plugins and moves entries into a retained backup.
  No live session loses a byte that it can read.
- Purging retained bytes waits for reader evidence after this driver's
  reap. An inactive retained copy outside discovery does not block cutover
  completion. The same rule already holds for the 1.1.0 keg.
- Artifacts: `docs/specs/owner-skill-delivery.md`, its evidence document and the
  baseline `docs/owner-skill-inventory.tsv` (SHA-256 `c9af548b…a4f0`). The
  instrument is `docs/evaluations/owner-skill-delivery/inventory.py`. It
  reproduced the baseline byte for byte. Its mutation control showed one
  unclassified row and one dangling row, and the clean run showed neither.
- The design assigns ten pinned sources and one mandatory file. It lists eight
  k34 requirements. Item 6, the user-invoked loading rule, is new. Item 7 asks
  k34 to state 1.1's exact handling of a 2.0 settings file.
- The `finish-promotes-after-the-grove-release` record was reworked in place.
  It now states the general rule, the owner switch as its second case and a
  fourth rejected alternative. Its slug and citations are unchanged.
- The three owner views render in Playwright's Chromium at 1440 pixels, in the
  light scheme. Fresh deep links reach each view. Dark and mobile appearance
  were not measured. The port 52710 server from an earlier session accepts
  connections but sends nothing. This session left it alone and served the
  presentation on port 52711.
- Tree: review k41 sits before k34, its first dependent leaf. The
  common-loading brief names the artifacts, and the rollout brief assigns the
  switch to finish. The root brief stays unedited, because it does not follow
  the writing rules. Rollout converts it and names the prepared switch there.
- k34, k38 and repository moves already consume this leaf by its handle.
  Thirty relative links resolve, and `git diff --check` passes.
- No `GROVE_PROVIDER_EVIDENCE` is set, so k41 carries no Creator line. The
  common-loading node keeps live leaves, so no node closes.
- Independently opened sessions lose native discovery of superseded skills.
  The root requirements settle that consequence, so this session asked no
  question. The final report states it plainly.

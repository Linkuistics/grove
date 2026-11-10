# owner-staging-capabilities-k250 — brief

## Goal

Prove in the staging HOME that skills which need native tools, servers and
hooks still work through Ravel delivery.

## Context

- The cutover contract in `docs/specs/owner-skill-delivery.md` lists these
  trials. One reads the TestAnyware command reference and uses a real VM.
  Others cover the Quint and codebase-memory MCP servers, the Claude Code
  hooks and `/guardrail`.
- The section "Native residue" states each capability's route after the
  switch. It also requires a trial that shows Claude Code runs the
  technical-communication hooks and lists no native
  `technical-communication` skill.
- The section "Resources and requirements of each source" of
  `docs/repository-moves-record.md` names each source's tools.
- owner-staging-hooks-k251 leaves the hooks plugin installed in the staging
  HOME. owner-staging-setup-k248 installed `guardrail`.
- In the isolation trial of owner-staging-setup-k248, Claude Code reported
  `codebase-memory-mcp` as `failed` and the Context7 plugin server as
  `needs-auth`. `quint-kb` and the Playwright server connected. The staging
  HOME copies no credential.

## Done when

- On each harness, an agent reads `using-testanyware` and its command
  reference from the `testanyware` source, and runs one real VM operation
  through the installed `testanyware` command.
- On each harness, an agent uses `quint-lang` or `quint-modeling` with the
  `quint-kb` MCP server, and `using-codebase-memory` with
  `codebase-memory-mcp`.
- On Claude Code, the always-on hooks run, and no native
  `technical-communication` skill appears. If Claude Code lists the root
  `SKILL.md` as a skill, the trial fails, and the layout needs repair.
- On Claude Code, `/guardrail` runs from `guardrail@personal`, and no Ravel
  root holds `guardrail`.
- The inventory after owner-inventory-exceptions-k264 accepts the
  enabled `guardrail@personal` exception and nothing wider. Its staging
  rows show no other unclassified entry.
- A user-invoked skill, `authoring-conventions`, is not loaded unless the
  task names it.
- The evidence document records each trial, its versions and its result.
  It reconciles each item of "Not measured" that a capability trial
  measures.

## Notes

A failure in a pack's layout goes to that pack's repository and needs a new
pack commit, a new candidate keg or formula, and reruns. Name that cost to
the owner before you choose the repair.

owner-staging-hooks-k251 ran the complete switch in the retained staging
HOME. The section "Quiet interval in staging" of
`docs/owner-skill-delivery-evidence.md` records the layout that it left:

- `technical-communication-hooks@writing-hooks` is installed and enabled.
  Its marketplace names `quiet-interval-1/prefix/opt/grove` in
  `~/.local/state/owner-staging/trials`, so that folder must stay.
- `guardrail@personal` is enabled. The five superseded plugins are installed
  and disabled, as in the real HOME after a switch.
- The original staging Ravel settings, SHA-256 `d8a49f06…`, are back in
  place and resolve the ten sources.
- The staging `codebase-memory-mcp` entry sets `CBM_CACHE_DIR` to the
  account daemon's cache folder, and it connects. Context7 still needs
  authentication.
- staging-mcp-trials-k286 ran `staging_home.py mcp-entries`. The staging
  Codex `codebase-memory-mcp` entry now sets the same `CBM_CACHE_DIR`, and
  the staging Codex `quint-kb` entry approves its tools. The owner's file
  has neither change.

## Decomposition

The trials need new verdicts in `staging_trial.py`, real runs on each
harness and an evidence section each. That work does not fit one session.
The leaf therefore became this node with three children. Each child is a
vertical slice: it adds its cases and verdicts, runs them, and writes its
own evidence section.

1. `staging-claude-native-trial-k285`: the Claude Code trials of the native
   residue. They are the hooks without a native `technical-communication`
   skill, `/guardrail` from `guardrail@personal`, the inventory rows of the
   staging HOME and the unloaded `authoring-conventions`.
2. `staging-mcp-trials-k286`: `quint-kb` and `codebase-memory-mcp` with
   their skills, on each harness.
3. `staging-testanyware-trial-k287`: the TestAnyware guidance, its command
   reference and one real VM operation, on each harness.

4. `inventory-guardrail-loaded-dir-k288`: staging-claude-native-trial-k285
   found that the inventory's sole-skill rule reads the plugin-cache copy
   of `guardrail@personal`, but Claude Code loads the keg's directory. This
   leaf makes the rule read the loaded directory.
5. `owner-quint-kb-approval-k289`: staging-mcp-trials-k286 found that Codex
   denies each `quint-kb` call under the owner's approval policy `never`,
   in staging and in the real HOME. A staging change repaired the trial.
   This leaf gets the owner's decision on the real entry.

Each child reconciles the items of "Not measured" that its own trials
measure. The node is done when the five children are done.

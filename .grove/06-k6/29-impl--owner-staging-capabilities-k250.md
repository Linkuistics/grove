# owner-staging-capabilities-k250

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

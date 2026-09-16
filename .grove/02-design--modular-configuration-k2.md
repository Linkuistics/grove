# modular-configuration-k2

## Goal

Design modular Grove configuration against the user-approved requirements in
the root brief: reusable parameterized commands, composable named profiles,
workspace selection, legacy compatibility, inspection, and example delivery.
Deliver a coherent design that a planning session can decompose without
re-interviewing the user.

## Context

`plan-k1` records the interview, approval, and the existing code evidence. The
root brief is the authoritative requirements contract and carries the agreed
test seams and relevant ADRs.

The distinction the user explicitly checked is load-bearing: always read the
global `~/.config/grove/config.kdl`; select at most one of the worktree and
main-repository `.grove.kdl` candidates and apply it over that global file. A
local profile list replaces the global default selection, not the global
definitions.

The existing public seams are generic `Templates` loading/expansion and Grove's
`SessionConfig` adapter. Their present interfaces require full templates; their
shape is evidence to assess, not a reason to retain the rejected duplication.

## Done when

- A durable design describes the grammar and behavioral contracts for commands,
  parameters, profiles, ordered includes, selection, overrides, and inspection.
- The design shows how to express the root brief's acceptance scenarios without
  copying complete commands or full kind-routing maps for a small variation.
- Resolution order and override granularity are unambiguous, including profile
  includes, repeated references, and local values. Parameters can vary a value
  embedded within one argument while preserving that argument's boundary.
- Active-combination validation is compatible with unfinished inactive profiles,
  incomplete building-block profiles, and unchanged legacy configurations. The
  scope of structural versus semantic diagnostics is explicit.
- Source attribution and read-only inspection use the same resolution as launch;
  unknown selected profiles, reachable cycles, missing required parameters, and
  invalid active commands have actionable errors without silent fallback.
- Existing local-source search, untrackedness, just-in-time kind presence,
  no-shell launching, and next-session reload behavior have explicit homes in
  the new design.
- The design names the smallest useful module interfaces and assigns generic
  resolution versus Grove-specific source/selection/VCS policy deliberately,
  using the user-agreed test seams.
- The sample new configuration and override examples have a concrete delivery
  plan for `~/.config/grove/` that preserves the active configuration and other
  personal files. Examples are checked through the actual reader.
- The relevant ADRs are reworked in place and the glossary/spec set is coherent;
  affected documentation and code-walkthrough contracts are identified for
  implementation planning.
- Any review or planning follow-up required by the design is cut according to
  the Grove workflow, with the specific remaining work in its task body.

## Notes

This is an approved requirements handoff, not a fresh requirements interview.
Choose exact KDL syntax, parameter namespaces, inspection command shape, and
example delivery from the agreed behavior. Escalate only a newly discovered
trade-off that actually needs the user's decision.

Do not turn profiles into a built-in harness/model router or infer a catch-all
for unknown kinds. The configuration author owns command arguments and explicit
kind routing. A local delta must not independently introduce a kind absent from
personal policy; make the meaning of that property precise with profiles.

The current ADR forbids precisely the reuse the user now wants. Rework the
decision around inspectable resolved commands, clear provenance, and explicit
composition rather than treating the old prohibition as an immutable constraint.

The requirements leaf did not change the user's active configuration, implement
the new grammar, or deliver examples for syntax that does not exist yet. Those
are subsequent implementation work, with example delivery part of the grove's
completion criteria.

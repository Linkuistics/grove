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

## Decisions (running log)

The modular grammar uses a single `config { ... }` wrapper, distinguished by
shape from a legacy `config "..."` kind. Existing flat entries remain valid,
including kinds whose names match new grammar words. Reusable command definitions,
bindings, kind routes and profiles have separate namespaces. Bindings provide
the indirection needed to switch lead/review arrangements without copying the
kind map; they are author-defined, with no built-in roles.

Profiles patch command parameter values, binding targets and kind routes.
Includes expand depth first in listed order, followed by the including profile's
own patch. Every occurrence is applied, including repeats; cycle detection uses
the current expansion stack, not a global visited set. The personal base comes
first, selected profile occurrences next, and direct local values last. Local
selection replaces the global default list; an explicit empty list is meaningful.

Command parameters use `${param.name}` and may occupy part of an argument.
Existing runtime slots remain whole-word and retain their cardinality rules.
Tokenization happens before either substitution; inserted values are opaque and
are never split or recursively expanded. A command's executable stays literal.

The generic library will own parsing, ordered resolution, source attribution and
the compiled command snapshot. Grove will choose the two sources, enforce local
file admissibility, select the profile list, and supply runtime slots. Inspection
will consume the same resolved snapshot as launch. I considered putting all
composition in Grove and accepting precompiled argv in the runner; that would
duplicate template validation and lose the existing single configuration seam.

This design is a load-bearing grammar and resolver contract, so its adversarial
read belongs in a `review-design` leaf before implementation planning. No
in-session reviewer is needed. A formal model is not required for this leaf:
the central obligations are a deterministic ordered fold, typed unresolved versus
resolved snapshots, and argument preservation at the existing executable seams.

Local kind authorization is measured against explicit route targets after the
personal base and selected personal profiles, before direct local overrides.
An inactive profile cannot authorize a local-only kind. Local parameter values
can complete an active personal route. Existing valid overlay-only kinds remain
non-admitted and fail on use rather than blocking unrelated configured kinds.

Validation preserves eager checks for every legacy flat template. Modular
documents check syntax, shapes and duplicate declarations everywhere, then check
include traversal and effective active references/templates/values. Overwritten
modular assignments need not resolve; incomplete profiles can be composed before
required values are checked. `unset` removes a scoped parameter override and is
a no-op if absent, allowing a binding switch to shed an old parameter.

The operator surfaces are `grove-llm config-show [--kind KIND] [--json]` and
`grove-llm config-examples`. Inspection exposes compiled literal words and tagged
runtime slots, plus source/override history, without inventing a mandate. Example
installation uses distinct fixed filenames, exclusive creation and collision
reporting, and never overwrites active policy. Both surfaces must be explained
as operator-usable in help even though existing tree verbs address sessions.

The existing `Workspace::is_tracked` deliberately lets jj snapshot before
answering. I asked whether inspection must avoid that metadata effect; the user
approved reusing the existing check. Here read-only means no configuration or
working-tree file edits and no session launch; jj may snapshot metadata during
the same admission check used by launch. No new VCS probe is required.

## Validation

`bash scripts/check.sh` passed all eight principal checks, including the full
workspace tests and final validation of all six walkthrough books. The tracked
file set was digested before the run and checked file by file afterwards:
all 1,775 recorded subjects were unchanged. That measurement preceded this
validation note and the retirement rename.

The repository's pinned KDL 4.7.1 parser accepted all six example documents and
the spec's grammar example; a deliberately malformed KDL control was rejected.
This establishes KDL syntax only. Production validation of the new configuration
semantics belongs to implementation, as the sample readme states.

A local-link check over the new spec, module spec, both configuration ADRs,
example/visual readmes and context map resolved 97 local links. The visual
manifest's three diagram sources and discussion IDs were checked, the served
manifest matched the source bytes, the copied viewer matched the skill asset,
and its pinned Mermaid dependency returned HTTP 200. Browser discovery found no
available browser, so rendering, readability and deep-link behavior are not
claimed as checked; the review leaf carries that remaining visual check.

The deliverable is the durable spec, revised ADRs and vocabulary, interface
contract, design examples and editable diagrams. `modular-configuration-k3`
reviews it; `modular-configuration-k4` plans implementation after that review
and any integration. No production Rust source or active personal configuration
was changed by this design task.

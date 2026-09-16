# modular-configuration-k4


## Goal

Decompose the reviewed modular configuration design into the smallest useful
working increments. Produce implementation leaves with concrete acceptance
boundaries, keeping production code, examples and source-exact walkthroughs
coherent at each landing. This session plans; it does not re-interview the user.



## Context

The root brief is the requirements contract. Design is in
`docs/specs/modular-configuration.md`; `modular-configuration-k2` is the producer,
and `modular-configuration-k3` owns its review. Consume the design as it stands
after any review integration, not the producer's initial wording in isolation.

Public seams: generic Catalog/Templates load, resolution, provenance and argv;
Grove SessionConfig source admission and selection; composed loop with fake
executables. The user permits the existing jj metadata snapshot during
inspection's trackedness check. No change to VCS admission semantics is needed.

## Done when

- Implementation leaves form independently demonstrable/verified increments,
  covering resolution, Grove integration/reload, inspection and actual example
  delivery without losing legacy behavior or inactive-experiment support.
- Each leaf names its observable test seam and documentation obligations.
- All acceptance rows in the spec have an implementation/testing owner. Include
  the generic non-Grove consumer and the existing flat conformance surface.
- The example installer validates the packaged repository bytes through the
  production reader and delivers them under separate filenames in
  `~/.config/grove/`, preserving the user's active configuration and collisions.
- The complete root brief remains achievable by the resulting tree; planning
  does not mistake creating an installer for delivering the requested examples.

## Notes

Concrete source/documentation touchpoints identified during design (verify the
current graph and source before implementation):

- `crates/keyed-launch/src/templates.rs`, its public exports, diagnostics and
  conformance entry point: introduce Catalog and the resolved inspection view,
  retaining the validated Argv construction seam and flat compatibility.
- `crates/grove-loop/src/session_config.rs`: preserve discovery/admissibility,
  choose local versus default selection, and expose inspection. Keep
  TemplateSource reload behavior and driver/verb validation load points.
- `crates/grove-llm/src/cli.rs`: configuration verbs must dispatch before ambient
  epoch admission; the current `run` admits every existing verb before dispatch.
  Help currently calls all verbs session-only and must explain operator use.
- `docs/CONFIGURATION.md`, `docs/USAGE.md`, configuration sections of
  `docs/ARCHITECTURE.md`, crate docs/comments and release-facing descriptions
  must describe the delivered behavior. Remove pending-implementation notices
  when they become false. Reconcile the durable specs/ADRs rather than adding
  parallel decisions.
- `docs/walkthroughs/keyed-launch/`: source fragments, template-law/argv chapters,
  indexes, manifest root lengths, concept rows and corpus inventory.
- `docs/walkthroughs/grove-loop/`: `18-which-files.md`, relevant driver/epoch
  explanations, indexes and `walkthrough.toml`.
- `docs/walkthroughs/grove-llm/`: grammar, growing/order chapters, source index
  and `walkthrough.toml`, particularly new dispatch before admission.
- The walkthrough contracts require exact source reconstruction. A production
  source change must land with its book updates and final book validation.
  Discover any other affected book evidence via the manifests; do not assume
  this list is exhaustive.

Use the approved public configuration and jj/fake-executable launch seams.
Preserve `.cargo/config.toml`'s meta-grove signal guard and invoke `grove-llm`
directly for actual Grove verbs. Read `scripts/check.sh` for the required checks.

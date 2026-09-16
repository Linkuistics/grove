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
  the generic non-Grove consumer, flat-load convenience equivalence, and the
  Catalog/selection conformance surface.
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
- `crates/grove/src/cli.rs`: expose `grove config show` and `grove config examples`
  before driver lease acquisition or loop startup. Both ignore ambient session
  epochs; inspection uses the shared adapter and examples need no workspace.
  Preserve the human/agent split; no new `grove-llm` configuration verbs.
- `docs/CONFIGURATION.md`, `docs/USAGE.md`, configuration sections of
  `docs/ARCHITECTURE.md`, crate docs/comments and release-facing descriptions
  must describe the delivered behavior. Remove pending-implementation notices
  when they become false. Reconcile the durable specs/ADRs rather than adding
  parallel decisions.
- `docs/walkthroughs/keyed-launch/`: source fragments, template-law/argv chapters,
  indexes, manifest root lengths, concept rows and corpus inventory.
- `docs/walkthroughs/grove-loop/`: `18-which-files.md`, relevant driver/epoch
  explanations, indexes and `walkthrough.toml`.
- `docs/walkthroughs/overview/`: the human command surface and its source corpus.
  Reconcile `grove-llm` walkthrough material only where shared adapter changes
  affect its existing load points; there is no new dispatch before admission.
- The walkthrough contracts require exact source reconstruction. A production
  source change must land with its book updates and final book validation.
  Discover any other affected book evidence via the manifests; do not assume
  this list is exhaustive.

Use the approved public configuration and jj/fake-executable launch seams.
Preserve `.cargo/config.toml`'s meta-grove signal guard and invoke `grove-llm`
directly for actual Grove verbs. Read `scripts/check.sh` for the required checks.

## Decisions (running log)

- Consume the integrated design, including `missing_target`, Catalog-based
  conformance, empty-selection loader equivalence, human CLI configuration
  commands, and actual example delivery. These are implementation obligations,
  not questions to reopen in planning.
- Keep the approved complete feature in this grove. The planning skill normally
  asks for separate groves at working increments; this leaf explicitly requires
  the resulting tree to retain the complete root contract. An optional topology
  question was offered; absent steering, preserve that scope and order green
  working increments within it. Do not strand inspection or personal example
  delivery in an independently launched workstream.
- The generic configuration increment needs several sessions. First make the
  captured Catalog and legacy Templates observable with structured diagnostics
  and provenance; then deliver reusable base commands; then selected profiles.
  Each step must work through the public API with a non-Grove consumer and keep
  the legacy Grove caller green. No parser-only or schema-only leaf is sufficient.
- Follow with workspace selection and reload, the human inspection command,
  and safe example installation plus delivery to the requested personal
  directory. Each source-changing step owns its documentation and source-exact
  walkthrough updates; there is no later catch-up documentation leaf.
- This decomposition is load-bearing. Cut a `review-planning` leaf ahead of
  implementation after the complete tree exists; spend no in-session reviewer.

## Planning evidence

Tier 2 graph verification used project
`Users-antony-Development-grove.modular-configuration`, generation
`2026-09-16T11:54:39Z`. Targeted discovery and inbound traces identify the current
`Templates::load` caller in `SessionConfig::read` and the old conformance entry
point. Exact snippets confirm the personal/local admission order and the human
CLI's lease boundary. Coverage reports no recorded gaps and matching metadata
for those production files. Graph call edges are candidate evidence, not an
exhaustive caller claim; direct source confirms the two loop load points.

Documentation and scripts are excluded from the index, and glossary metadata
has changed; read these directly. Walkthrough manifests cover crate manifests
and production Rust recursively, so new modules also need book coverage. The
planning tree's acceptance map uses the current spec, not graph completeness.

## Validation

- `bash scripts/check.sh` passed all eight principal checks, including workspace
  tests and final validation of all six walkthrough books. SHA-256 digests of
  all 1,784 tracked inputs matched before and after the run. No implementation
  or example delivery is claimed by this planning session.
- Grove resolved the planning/review and all six implementation leaves and
  returned their expected complete brief chains. A focused artifact check
  verified all 13 stable task/node handles, nonempty goals/completion criteria,
  handle references and all 17 spec acceptance rows in the ownership map.
- The final prose edits clarify that the inherited design constraints are now
  implementation obligations, and record this evidence. The source and test
  subjects remain at the checked pre-implementation boundary.
- `modular-configuration-k13` is the next live root entry and reviews this
  decomposition before any implementation. Retiring this planning leaf leaves
  live work under the root; no parent node closes in this session.

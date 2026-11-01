# grove-skill-delivery-k8

## Goal

Specify Grove's adoption of Ravel discovery and the minimum delivery changes
needed before any skills move out of this repository.

## Context

Consume the settled artifact and any integrated review from
ravel-skill-discovery-k7. Update Grove's durable delivery and caller specifications;
use the existing provisioning and harness-selection specs as starting points.
The root brief owns destinations and exclusions.

Current provisioning embeds a broad plugin snapshot and publishes managed Codex
links. Its startup call, inventory/build coupling, packaging, installation
guidance and tests need a coherent transition, including a fresh installation.

## Done when

- Specify how the common Ravel path replaces obsolete automatic provisioning
  while preserving a usable core bootstrap. Any retained core-only fallback
  goes through that path; do not keep a second automatic native delivery system.
- Settle Grove/Ravel compatibility requirements, failure remedies and dependency
  rollout order. Grove continues to send its ordinary mandate and public launch
  inputs without resolving Ravel's catalog or declaring kinds.
- Define safe treatment of existing owned caches/links and active readers;
  preserve foreign installations and the running driver's original ending
  contract. Identify changed archive, build, installer and Homebrew obligations.
- Specify cross-repository resource reachability and conformance at the common
  loading boundary. Keep one owner per normative rule and the current kind
  labels, review/integration routes, stable handles and tree behavior.
- Identify necessary documentation and source-exact walkthrough updates, and
  map them to delivery work rather than leaving cleanup for an unspecified end.
- Specify acceptance through existing Grove tree/launch seams, core and external
  pack conformance, installed-layout checks where affected, and the two real
  harness discovery trials with native copies absent.
- The durable spec set states the human-corrected delivery boundary and identifies
  any owning Ravel design correction explicitly before adoption review and
  common-skill-loading-k9. Record unresolved implementation questions explicitly;
  do not redesign kind guidance or #27. This criterion reflects the human's
  correction during this session; the previous catalog design is no longer the
  settled prerequisite originally assumed by the leaf.

## Notes

Do not move packs or alter the owner's installations during design. Inspect
actual build/provisioning consumers before asserting scope. Cut any earned
review lazily ahead of the implementation planner.

## Decisions (running log)

- Consume Ravel's integrated design at commit
  `66aaec06f5c020215b9b0e4139c2ba2e0b969644`. The adoption targets stable
  Ravel 2.x from 2.0.0 and policy/request schema 3. Existing 1.1 callers and
  running sessions retain their independent prefixes and original endings.
- Remove Grove's automatic native provisioning and embedded broad snapshot.
  Do not add a core fallback: fresh setup explicitly configures a complete
  skill source through Ravel's existing owner settings and resolution seam.
  This costs an explicit setup step and a cold Git acquisition; exact warmed
  sources remain usable offline. Grove gains no catalog reader or kind gate.
- Leave all old native links, current pointers and snapshots untouched by
  startup and upgrade. Cleanup is an explicit quiescent owner operation with
  path-specific ownership evidence; it is not a new automatic migration tool.
  Catalog precedence cannot undo native hooks or already injected guidance.
- Keep the existing provisioning spec's stable path, rewriting its contract
  around common delivery and clearly separating the target from shipped 22.8
  behavior. The caller spec and affected ADRs must describe the same target.
  Source coordinates and migration obligations belong in the evidence document
  and planner handoff, rather than in the behavioral specification.
- Resume after interruption from the four decisions above; the working copy
  contained only this running log. No delivered design or review is assumed.
- Ravel owns source/catalog validation and read grants; pack conformance owns
  normative reference reachability. Cross-source references name the target
  skill and resolve its resources relative to its catalog directory. A catalog
  is not evidence that prose references work. Keep build-time conformance
  separate from Grove's open-kind launch admission.
- No-source discovery retains Ravel's disabled behavior. Grove adds no missing-
  kind preflight: setup resolves and checks the intended sources, and a session
  unable to load its mandated skill reports that failure without retiring work.
- Use a separate review-design leaf before common-skill-loading-k9. The doubts
  are bootstrap completeness, old-reader preservation, schema/version rollout,
  and cross-source conformance. No in-session reviewer is used beside it.
- Human correction: Grove and Ravel are decoupled with respect to skills. Grove
  may name a skill; the user ensures it is installed in each harness or supplied
  through Ravel. This replaces the earlier requirement for explicit Ravel source
  setup and a Ravel-2-only Grove skill-delivery dependency. Native installation
  and Ravel directory injection are alternative owner-managed routes.
- Human correction: Ravel does not resolve a requested skill; it injects skill
  directory paths into the prompt. The agent locates and loads the named skill.
  Remove generated-catalog and skill-identity assumptions from Grove's design.
  The earlier Ravel design's name validation/catalog contract requires a focused
  upstream correction before dependent planning. Preserve the separate accepted
  local/pinned-source and resource-lifetime requirements unless that correction
  establishes a conflict; source acquisition is not requested-skill resolution.
- Keep Ravel 1.x from 1.1.0 supported for the existing public session contract;
  add 2.x only with the same caller acceptance. Native-only skill users do not
  need a newer Ravel for skills. Explicit policy examples must name their target
  SDK/schema rather than letting a delivery choice select runtime compatibility.
- The user-corrected Grove design is a reviewable boundary. A focused Ravel
  design repair runs next, followed by this producer's adoption review and then
  common-skill-loading-k9. The earlier catalog spec is historical input and is
  explicitly not treated as an agreed implementation prerequisite.

## Delivered artifacts and verification

- Reworked `docs/specs/codex-skill-provisioning.md` in place and aligned the caller
  spec, related ADRs, glossary notes and context ownership map. The new delivery
  ADR records the user-owned native/directory choice and the explicit bootstrap
  cost. The evidence document maps inspected source and documentation consumers.
- Updated the root and common-loading briefs with both human corrections. Added
  ravel-skill-directories-k15 to revise the owning Ravel documents, followed by
  grove-skill-delivery-k16 to review this adoption against that correction.
  common-skill-loading-k9 remains after both. No in-session reviewer was used.
- Local validation passed for 149 Markdown links/anchors, 11 diagram entries and
  13 exact served resources. Missing target, missing anchor and unknown diagram-ID
  controls each failed as intended. The 94 files read during that run retained
  their SHA-256 digests. This is document validation, not product acceptance.
- `mise exec -- task --list` parsed the existing tasks. The existing
  `design:harness-selection` recipe served the viewer at port 18779. Browser
  inspection in Dia verified the delivery and handoff deep links, the topic and
  current-discussion panels, changed markers and rendered diagrams. An initial
  Mermaid sequence error was corrected and the rendered sequence was rechecked.
  Long labels in the new views were wrapped so their desktop view fits. Mobile
  and dark appearance were not measured.
- Source coverage was checked at the cited generation, with direct fallback for
  the installer's partial range. Changed documents were checked again at graph
  generation `2026-11-01T11:42:28Z`; Mermaid partial ranges were read as complete
  source and checked through the actual browser renderer.
- No Rust/product code, owner configuration, installed skills or package release
  changed. No product suite, Ravel 2 compatibility test or real harness discovery
  trial is claimed by this design session. Those acceptance obligations remain
  with the implementation and rollout work.
- No current provider evidence was present, so the review has no Creator line.
  No parent closes: upstream design correction, adoption review, planning and
  the subtree's implementation/acceptance remain live.

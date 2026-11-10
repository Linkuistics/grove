# skill-keg-link-layout-k292

**Integrates:** skill-keg-link-layout-k291

## Goal

Triage the review of the revised skill-source layout and integrate the
actionable findings before skill-keg-link-layout-k290 plans from it.

## Context

- Read the findings from the committed review skill-keg-link-layout-k291.
  Its durable report is the section "Directory-link layout review" of
  `docs/homebrew-skill-delivery-evidence.md`. The task file of that review
  holds its running decisions.
- The producer is skill-keg-link-layout-k274. "The keg layout" of
  `docs/specs/codex-skill-provisioning.md` owns the layout, its formula
  owners and its migration. `docs/specs/owner-skill-delivery.md` owns the
  owner's formulae, accepted versions and the switch.
- The section "Directory-link layout experiment" of the evidence document is
  in a pending region of the controlled-English conversion. Convert it
  before an edit changes it.

## Done when

- Each finding has a recorded disposition supported by the source and the
  owning contract.
- The owning specifications, the evidence, the diagrams and the handoff text
  are current for each accepted finding. Search the claim, not a file list.
- The briefs and the queued leaves give k290, k93 and the finish handoff each
  changed consequence.

## Notes

This integration changes the design and its documents only. It publishes no
formula and changes no real Homebrew link. Some findings need a change in
another repository, such as Ravel or TestAnyware. Record each such change as
an obligation of the leaf that owns the work in that repository.

## Decisions (running log)

- Read skill-keg-link-layout-k291 from commit `b32012233495`. The layout
  remains private payloads with relative directory symlinks. No finding
  requires substantial redesign or a new decision record.
- L1 states an unclear contract. Distinguish the full Homebrew package
  version, including revision, from the source commit. Packaging-only
  acceptance can retain source trials after digest equality and packaging
  acceptance. The switch must compare the full package identity.
- L2 is a real acceptance gap. Both hook marketplaces name the private
  payload directly. Repeat the hook seam on actual formula output, including
  cleanup, cache copying and trust. Earlier real-directory trials remain
  historical evidence.
- L3 is a real acceptance gap. The durable shared-directory seam must run
  during formula validation and release acceptance. It uses actual Homebrew
  linking and records the linker identity. Direct keg resolution is
  insufficient.
- L4 to L7 are real consistency defects. Include Ravel's shared ancestors
  and whole-keg duplicate rollback. Distinguish archive and installed paths.
  Assign the TestAnyware and Ravel producer changes to queued work.
- L8 is real. A preparatory commit converts the affected prose before the
  substantive edit. It preserves the experiment's results and limits.
- Reconciliation changes core's packaged README and therefore its digest.
  k290 must assign a new candidate and core staging acceptance. The plan,
  prepared inputs and dry run must consume the renewed evidence.
- The one narrow reviewer found an unclear boundary in packaging acceptance.
  Source digests exclude native-only `guardrail`. The owner contract now
  requires complete native-plugin comparison and renewed trials for changes.
  The hook seam includes guardrail's refusal assertion and a no-op control.
- `mise exec -- task check` passed all 18 principal checks. The workspace
  run passed 1,344 tests and skipped 29. Nine documentation tests and five
  final walkthrough validations passed. The subsequent reference-navigation
  run passed all 13 tests.
- The path-reference sweep retains public aliases, archive paths, earlier
  measurements and the implementation inputs that k290 must change.
  Diagram manifests preserve their IDs and source files. The browser host
  was unavailable, so rendered layout and deep-link positions remain
  unverified.

# grove-skill-delivery-k19

**Integrates:** grove-skill-delivery-k16

## Goal

Triage the adoption review's findings on Grove's skill-delivery design and
apply the real ones before implementation planning consumes the design.

## Context

Read the findings from the review's own commit in this tree, not from this
body. The reviewed artifacts are Grove's delivery and caller specifications,
the four records they cite, the design evidence and the caller views, at Grove
commit `d0e0a2ee`. The producer task grove-skill-delivery-k8 records the
artifact set. The upstream Ravel contract is commit `b93bb927` in
`/Users/antony/Development/AgenticOrchestration/Ravel`. Product code, owner
configuration and installed skills stay unchanged by this step.

## Done when

- The session classifies each finding as a contract stated unclearly, a real
  issue, a visible trade-off, or noise, and records the reason.
- The session applies each accepted finding in its owning document and reworks
  the record set as `ADR-FORMAT.md` directs, with every citation reconciled.
- The delivery spec, caller spec, records, evidence and views agree with each
  other after the edits. Where a finding needs a Ravel or tap change, the
  session records that handoff rather than editing those repositories here.
- Grove's design validation for links, diagram sources and served resources
  passes, and the task records the commit.
- A finding that demands a redesign rather than a repair becomes a new producer
  review chain beside this leaf, not an edit here.

## Notes

This step may spend one narrow in-session reviewer. Preserve the running
sessions' original ending commands. common-skill-loading-k9 follows this step
and consumes its result. Implementation and harness trials remain later work.

## Decisions (running log)

- Read the findings from review commit `b17a11e0f722373805e6008acc4f36972996e0eb`.
  The artifact at `d0e0a2ee` and the integrated Ravel contract at `b93bb927`
  are the comparison points. This session changes Grove design documents only.
- Finding 1 is a real issue: the unversioned formula dependency and the
  shipped major-1 predicate allow the prescribed release order to break fresh
  installs. Require a compatible dependency at every published tap state.
  Publish dual-major Grove support before switching unversioned Ravel, or keep
  the new Ravel release under a separate versioned formula until that support
  ships. Ravel feature advertising follows feature publication; compatibility
  support may precede it. Actual formula and Ravel changes remain handoffs.
- Finding 2 is a contract stated unclearly. The mandate already supplies the
  force-stop command, but does not bind missing guidance to it. Require the
  mandate to tell an agent that cannot load the skill to report, leave the leaf
  live, and use that exact force-stop command. Acceptance observes no relaunch.
- Finding 4 is a real issue. A native namespace belongs to the installed
  plugin, not the kind label. Keep the bare `grove-<kind>` target and require
  namespace-neutral wording before packs move; do not add a kind-to-plugin
  registry. The first external pack and subsequent moves prove native Claude
  Code loading as well as directory loading with native copies absent.
- Finding 5 is a real issue in the record's identity. Rename the delivery ADR
  to `skill-delivery-is-the-users-choice` and reconcile current citations.
  The review's historical finding remains an account of the old artifact.
- Finding 6 is a real issue. Keep delivery ownership in its ADR and cite it
  from the other three records. Put dual-major admission's rejected alternative
  and reopen condition in the supervision record; evidence points there.
- Finding 7 is a real issue. The upstream correction is integrated at Ravel
  `b93bb927`; views must distinguish that settled contract from the still
  unimplemented adoption.
- Finding 8 is a contract stated unclearly. Cleanup is an optional documented
  manual procedure owned by the user, not a new verb. The implementation
  increment that removes provisioning supplies it in `docs/USAGE.md` and
  checks its ownership cases with disposable fixtures. No cleanup runs here.
- Spend the one narrow reviewer on confined native delivery. The claim to
  challenge is that the contract can preserve user choice while distinguishing
  actual read grants from installation. The review receives only the proposed
  wording, requirements and permission sources; the remaining repairs proceed
  independently.
- Finding 3 is a contract stated unclearly. The unconditional native-only row
  overpromised confined access. Limit its required success case to lifecycle;
  require actual read/discovery evidence for a claimed confined native setup.
  The review's stronger claim that injection is the only possible native-skill
  route is noise: the cited confinement table permits explicit regular-file
  grants and system paths, including the macOS Cellar. The narrow reviewer
  found no actionable issue in this distinction. It proves no working harness
  configuration; actual harness/backend acceptance remains later work.
- No finding requires substantial redesign. Preserve the review's visible
  trade-offs: explicit setup, the stable specification path, native-hook
  coexistence, isolated mixed-major settings, dual-major tests and the clear
  separation of target design from shipped behavior. Internal conformance and
  harness measurements remain with implementation planning.
- Reconcile the handoff in the common-loading, research-pack and rollout
  briefs. The research brief still named the discarded catalog; it now names
  supplied directories. Preserve this tree's Ravel-first release sequence with
  a separate versioned formula unless an earlier Grove compatibility release
  is explicitly scheduled. These are handoffs, not changes in other repositories.

## Verification and boundary

- The integration is recorded by jj change
  `utunysounylkoquqtorququwvoumtzop`; its sealed commit is discoverable by
  `grove-skill-delivery-k19` in the description. The reviewed findings remain
  in `b17a11e0f722373805e6008acc4f36972996e0eb`.
- Verify-tier graph discovery for the resolver and mandate returned complete
  bounded results. Coverage at generation `2026-11-01T12:38:14Z` reported no
  recorded gaps for the inspected Grove source and documents except the two
  Mermaid files, which were read in full. Ravel's documentation is excluded
  from its graph; direct source supplied the confinement evidence. A jj diff
  confirmed those Ravel documents unchanged from `b93bb927`.
- The one narrow reviewer returned no actionable findings. The evidence is
  contract inspection, not proof that a native confined harness works.
- Browser rendering could not be inspected: browser control reported no
  available browser and the running Dia app returned `cgWindowNotFound`.
  Diagram source/manifest and exact served-resource checks are separate from
  browser rendering. No mobile or dark appearance check is claimed.
- Local validation passed for 151 Markdown links/anchors, 11 diagram entries
  and 13 exact HTTP resources served by `task design:harness-selection`.
  Missing-file, missing-anchor and unknown-diagram controls each failed as
  intended. The 94 subjects read by that run retained their SHA-256 digests;
  the instrument and per-file evidence are `/tmp/validate-grove-k19.py` and
  `/tmp/grove-k19-validation.json`. The renamed ADR's old slug and the obsolete
  view status remain only in the historical review's account of the artifact.
- Product code, installed skills, owner configuration, Ravel and tap files are
  unchanged. Product suites and actual harness trials belong to the planned
  implementation; no new runtime acceptance is claimed here.
- No parent closes. `common-skill-loading-k9` and the subtree's implementation
  and acceptance remain live. No new review leaf or Creator handoff is needed.

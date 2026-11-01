# ravel-skill-directories-k18

**Integrates:** ravel-skill-directories-k17

## Goal

Triage the review's findings on Ravel's directory-injection contract
and apply the real ones in their owning repository. Do this before Grove's
adoption review treats the contract as settled.

## Context

Read the findings from the review's own commit in this tree, not from this
body. The reviewed artifacts live in
`/Users/antony/Development/AgenticOrchestration/Ravel` at commit
`4c16f8e09d58c8f649a375029c8e8fa2933de2b4`. Work there under that repository's
instructions and VCS discipline. The producer task `ravel-skill-directories-k15`
records the artifact set and its Grove-side handoff documents. Product code
remains Ravel 1.1 and stays unchanged by this step.

## Done when

- The session classifies each finding as a contract stated unclearly, a real
  issue, a visible trade-off, or noise, and records the reason.
- The session applies each accepted finding in its owning document and reworks
  the ADR set as `ADR-FORMAT.md` directs, with every citation reconciled.
- The specification, glossary, evidence document and diagram views agree with
  each other after the edits. Where a finding names the Grove handoff, the
  session reconciles Grove's delivery evidence too.
- Ravel's design checks for links, manifest and diagram sources pass, and the
  task records the owning-repository commit.
- A finding that demands a redesign rather than a repair becomes a new producer
  review chain beside this leaf, not an edit here.

## Notes

This step may spend one narrow in-session reviewer. Preserve the running Grove
sessions' original ending commands. Owner configuration and implementation
remain later work. grove-skill-delivery-k16 follows this step and consumes its
result.

## Decisions (running log)

- Read the findings from review commit `ec3b0700e29c`, against the Ravel
  artifact at `4c16f8e09d58c8f649a375029c8e8fa2933de2b4`. Both working copies
  began empty. This leaf repairs documents; product code and owner settings
  stay outside its scope.
- Finding 1 is a contract stated unclearly. Define an explicit mechanism by
  a concrete tool, command or path. A name plus "load", including a plugin
  namespace, remains a name. An unavailable native match permits a reported
  directory fallback for the same name unless the caller expressly forbids
  alternatives. A concrete path is never silently replaced. This preserves
  the human's explicit-route constraint while making ordinary Grove naming
  work with no native copies. Record each outcome in the acceptance matrix.
- Finding 2 is a real issue. Specify mandatory reads before caller-block
  actions, subject to the harness's higher-priority instructions; claim no
  other priority for the preamble. Grove's future mandate must qualify its
  "first" instruction with "after any mandatory guidance". Reconcile both
  caller specs and the handoff; leave the shipped mandate to implementation.
- Finding 3 is a visible trade-off. The exact settings reader confirms that
  1.1 rejects unknown keys. Retain one owner settings document and record the
  rejected separate-file alternative in the preparation ADR. Separate prefixes
  alone do not isolate it: simultaneous 1.1 and enabled 2.x use separate HOME
  values, or the shared settings omit `skills` until all readers migrate.
  This retains the existing transition rather than introducing a new interface.
- Finding 4 is a real issue. Record mandatory file references and the retained
  directory descriptor in the existing preparation ADR. Reduce the evidence
  choice table to pointers; detailed behavior stays in the specification.
- Finding 5 is a contract stated unclearly. Define the source-record fields,
  classify unknown mandatory source IDs as settings errors (exit 2), define
  `skill_access_unavailable` as a prelaunch read/grant refusal (exit 3), and
  give the caller wrapper a fixed label outside the reusable preamble.
- Finding 6 is a real issue in acceptance coverage. Add whole-preamble loss
  without a recovery reminder to actual harness trials. State that Ravel has
  no durable instruction channel into a harness; failure to recover remains
  an acceptance gap, not proof that the environment variable is sufficient.
- Finding 7 is a real issue. Remove byte offsets from policy-visible `skills`;
  consumers use `originalPrompt`. Keep byte-range verification internal to
  preparation and test Unicode content without requiring JavaScript slicing.
- Retain the review's visible costs: full local capture, agent enumeration,
  duplicate prompt bytes, locator-specific cache keys and inherited delegate
  authority. No finding requires substantial redesign. Decline an in-session
  reviewer because grove-skill-delivery-k16 is already chartered to contest
  the loading and caller-order handoff; actual compaction behavior requires
  harness trials and cannot be established by another prose review.

## Delivered artifacts and verification

- Ravel commit `b93bb927095ff09b91e9b78dc1ad05bdacfec649` contains the repaired
  skill/session specifications, preparation ADR, glossary, evidence and visual
  views. Its working copy is sealed onto a fresh empty change. Grove's delivery
  and caller specifications, evidence and parent handoff now consume it.
- The source check used Ravel graph generation `2026-11-01T07:59:38Z` and the
  exact `Settings::read` snippet. Source metadata matched, with no recorded
  gap. The docs subtree is excluded and glossary/Taskfile metadata had changed;
  direct reads supplied that evidence. Grove generation
  `2026-11-01T12:14:23Z` reported matching metadata and no recorded gaps for
  the inspected prompt, documents and task files. No completeness claim follows.
- The existing design validator, copied to a scratch path for a new preview
  port, passed 96 local links, three diagram entries and five byte-exact served
  resources across the edited Ravel/Grove documents. Missing-file, missing-anchor,
  unknown-diagram and obsolete-interface controls failed as intended. All 67
  read subjects retained their SHA-256 digests during that run, before the
  Ravel commit was sealed. The subsequent Grove handoff check passed 48 local
  links and the same diagram/resource controls, with 46 stable read subjects.
- Mermaid 12.0.0, matching the viewer's pin, parsed all three diagram sources
  in a temporary Node/DOM environment. An invalid syntax control failed.
  The manifest and each source retained their digests. Browser rendering was
  unavailable through the supplied browser API, so no visual-layout claim is
  made. The existing preview port was occupied and failed to serve; the
  documented `mise exec -- task docs:skills PORT=18778` served the verified bytes.
- These checks measure documents and diagram syntax. No product tests,
  acquisition behavior, actual harness compliance or release are claimed.
  Product source, owner configuration and installed skills were not changed.
- No parent closes: grove-skill-delivery-k16 and common-skill-loading-k9 remain
  live, and implementation plus harness acceptance still belong to this node.

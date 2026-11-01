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

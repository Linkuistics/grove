# ravel-skill-discovery-k14

**Integrates:** ravel-skill-discovery-k13

## Goal

Triage the review's findings on Ravel's skill-discovery contract and apply the
real ones in the Ravel repository. Do this before Grove's adoption design
treats the contract as settled.

## Context

Read the findings from the review's own commit in this tree, not from this
body. The reviewed artifacts live in
`/Users/antony/Development/AgenticOrchestration/Ravel` at commit
`b68af6e492316494dfce7ec2c0ee77a1a996e60b`. Work there under that repository's
instructions and VCS discipline. The producer task `ravel-skill-discovery-k7`
records the artifact set. Product code remains Ravel 1.1 and stays unchanged by
this step.

## Done when

- The session classifies each finding as a contract stated unclearly, a real
  issue, a visible trade-off, or noise, and records the reason.
- The session applies each accepted finding in its owning document and reworks
  the ADR set as `ADR-FORMAT.md` directs, with every citation reconciled.
- The specification, glossary, evidence document and diagram views agree with
  each other after the edits.
- Ravel's design checks for links, manifest and diagram sources pass, and the
  task records the owning-repository commit.
- A finding that demands a redesign rather than a repair becomes a new producer
  review chain beside this leaf, not an edit here.

## Notes

This step may spend one narrow in-session reviewer. Preserve the running Grove
sessions' original ending commands. Owner configuration and implementation
remain later work.

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

## Decisions (running log)

- Read the findings from review commit `b5ba2b50ae0341e0c0846097e520f4f0d8cda1c6`.
  Ravel's clean working copy descends directly from the reviewed producer
  commit. This session changes design documents only.
- Finding 3 is a real issue: prompt preparation changes a public semantic
  promise, and retained resources establish a persistent storage obligation.
  Both meet all three ADR tests. Add two records, move their alternatives out
  of the evidence table, and reconcile the existing supervision record.
- Finding 4 is a real issue: Grove's version check accepts only stable Ravel
  1.x from 1.1.0. The transition must use independent prefixes for acceptance
  and keep old callers on 1.1 until their adoption is available. Publication
  order does not authorize replacing a shared installation prematurely.
- Finding 5 is a real issue: preserve content verification on every cache reuse
  and make the resolution deadline an owner setting. Use `skills.resolutionTimeoutMs`,
  default 120000, range 1000..3600000 milliseconds. A slow valid cache gets a
  timeout-specific remedy and an acceptance case; it is not a missing revision.
- Findings 6 and 7 are contracts stated unclearly. Distinguish the three Ravel 2
  public operations from Ravel 1.1, and include stage `skills` in exit-3 refusal
  definitions and caller remedies.
- Finding 9 has three parts. Path collisions are a real validation gap: refuse
  when the cache filesystem cannot preserve distinct source paths. Literal Git
  LFS pointer capture is a visible trade-off of extracting object bytes without
  filters; state that the external payload is not captured. The lock-wait phrase
  is a contract stated unclearly: publication has no resolver lock, so remove it.
- Finding 10 is a contract stated unclearly: label the post-parent reader as
  detached or untracked, and distinguish it from ordinary descendants reaped
  with the managed process group.
- The leaf's one narrow reviewer is examining the proposed delegation,
  loader-precedence and catalog-recovery clauses together. Its scope is the
  contract boundary, not a second review of the entire design.
- Finding 1 is a real issue with a visible existing trade-off. Ravel cannot
  revoke context inherited by arbitrary harness children. Correct the claim,
  record the residue and reopen condition in the supervision ADR, and separate
  instruction adherence from adapter isolation in acceptance. This does not add
  ending authority through skill grants or redesign the result protocol.
- Finding 2 is a contract stated unclearly. Catalog paths govern mandatory and
  mechanism-unspecified loads, including plugin-qualified names. An explicit
  caller loader/path governs its requested load after mandatory bootstrap, with
  the identity exception reported. This preserves explicit caller intent.
- Finding 8 is a real recovery gap. Add managed-process `RAVEL_SKILL_CATALOG`,
  overwritten for enabled discovery and removed when disabled; keep it out of
  policy and transport environments. It carries a location, not permissions.
  The same retained path survives loss from prompt context without a new resolve.
- The fresh-context reviewer found no concrete issues in the proposed clauses
  for findings 1, 2 and 8 against the supplied contract. The review was limited
  to those interacting clauses and did not certify actual harness behavior.

## Verification

- Ravel changes are confined to `CONTEXT.md`, the two specifications, the design
  evidence, three ADRs, and the skill-discovery diagram sources, manifest and
  README. Product source and owner configuration remain unchanged.
- Local validation passed for 46 links/anchors, manifest group/discussion/update
  references and all three diagram sources. Deliberately missing target,
  missing anchor and unknown diagram-ID controls each failed as expected.
  Whitespace and conflict-marker checks passed. The 29 documents and referenced
  source files read by that validation had unchanged SHA-256 digests across it.
- `mise exec -- task --list` parsed the Taskfile. `mise exec -- task docs:skills
  PORT=18776` served the viewer, manifest and all three sources with exact bytes;
  the server was then stopped. The subject set included the Taskfile and viewer.
- Browser rendering and Mermaid execution remain unmeasured: the available
  browser inventory was empty. No product suite or future behavioral acceptance
  case was run or credited by this documentation-only integration.
- No substantial redesign was needed. No parent closes: Grove adoption design,
  common-loading planning, implementation and real-harness acceptance remain.

## Owning-repository commit

Ravel commit `66aaec06f5c020215b9b0e4139c2ba2e0b969644`
(jj change `lrvovtlwmmqplypyxzswryxyltwrvzlp`), titled
`Repair the skill discovery contract (ravel-skill-discovery-k14)`.
The Ravel working copy is sealed and clean. Grove's adoption design can consume
the revised specifications and cited ADRs from this commit.

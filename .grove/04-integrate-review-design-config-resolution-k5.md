# config-resolution-k5

**Integrates:** config-resolution-k4

## Goal

Integrate the actionable findings from the adversarial design review before
`local-config-kdl-k3` implements the configuration delta. Rework the ADR set and
the implementation contract; no production code in this leaf.

## Context

Read `config-resolution-k4`'s `## Review result` in full. Its cold read found
the two-stage resolution rule clear, all four new delta alternatives worth
recording, present-tense ADR language legitimate ahead of implementation, both
inbound citations from `one-build-owns-a-session` still valid, and the brief
handoff correct. Five findings need action:

1. `complete-session-configuration:37`-`42` says an untrusted repository cannot
   ship `.grove.kdl`, but neither an ignore instruction nor the planned
   `SessionConfig::load` seam establishes that the candidate is untracked. A
   tracked file at the searched path would select an executable. This conflicts
   with settled requirement 1 and the ADR's claimed security boundary.
2. Whole-template opacity and the delta's source/placement/security policy are
   independently reversible decisions. Split them into the surviving
   `complete-session-configuration` record and a slug-named delta record rather
   than keeping one 144-line record with two grains.
3. The inline-methodology option at
   `complete-session-configuration:136`-`144` still describes a kind-selected
   prompt slice. The current `skill-delivers-the-methodology` decision says the
   skill carries the methodology and the prompt carries only the guaranteed
   core; that owner also holds the surviving inline rejection.
4. `content/references/driver.md:23`-`24` still says personal configuration
   lives at the home path. It does not bridge cleanly to a design where the
   worktree/repository delta is personal launch policy and can supply the
   selected kind. The nearby fallback-to-inheritance edit is correct.
5. The placement rationale restates finish-transaction mechanics without citing
   `task-tree-transactions-fail-closed`, while its first
   `supported-workspace-layouts` citation attributes configuration-root
   semantics to a record about working-tree/control-directory topology. The
   second citation's no-advisory-channel argument is sound.

## Done when

- The tracked-file case has an explicit, honest design outcome. If satisfying
  requirement 1 means refusing tracked candidates, record the VCS-aware seam and
  extend `local-config-kdl-k3`'s contract and test seam accordingly. If the
  security claim or requirement must instead be weakened, stop and ask the
  human; do not silently turn it into a documentation convention.
- The ADR set is a minimum coherent current-state set at one-decision grain.
  `complete-session-configuration` owns opaque complete launch values; the new
  delta record owns lookup, per-kind selection, trackedness/security,
  fail-closed behavior, placement, and the four delta alternatives. Reconcile
  `CONTEXT-MAP.md`, every citation, and the root brief by slug.
- The stale inline-methodology option is deleted or reduced to an accurate
  current citation owned by `skill-delivers-the-methodology`.
- `content/references/driver.md` remains true now and has explicit ownership for
  its post-implementation wording; `local-config-kdl-k3` cannot finish while
  leaving the paragraph stale.
- The placement rationale cites `task-tree-transactions-fail-closed` directly,
  and every `supported-workspace-layouts` citation supports the claim attached
  to it.
- Post-fix corpus verification is recorded. No production or test code is
  implemented here.

## Notes

The two existing `one-build-owns-a-session` citations depend only on command
opacity and should remain on `complete-session-configuration`. A single direct
cross-citation between the two configuration ADRs for the personal-file
completeness invariant is coherent and preferable to overlapping records.

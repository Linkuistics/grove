# confirmation-prose-review-k16

**Kind:** review-impl

## Goal

Disprove `confirmation-prose-k15`'s claim that every normative surface now states
the `confirmation-boundary` decision and none states the old one.

## Context

The decision: the Retire cascade asks the human nothing; a brief-carrying node's
close checks its brief `Done when`, `leaf-add`s a nameable gap, escalates an
unnameable one, promotes upward and reports the close by handle. Read
`docs/adr/confirmation-boundary.md` before the diff.

**This chain exists because of a measured failure rate, not ceremony.** Twice in
this grove a reconciliation pass missed surfaces that a grep found immediately —
`chain-node-k9` missed `docs/adr/task-kind-taxonomy.md`, and
`chain-node-integrate-k11` found three more (two of them `--help`) after reading
had missed them. Assume the same rate here.

## Done when

Each of these has been **executed**, not reasoned about:

1. **Re-run the claim grep yourself**, over the whole repo including paths `k15`'s
   leaf did not name. Do not take its surface list as the search space — that is
   the exact failure mode this chain exists to catch. A surviving statement of the
   old rule is a finding.
2. **Check for the opposite defect: parallel guidance.** A surface that now states
   both rules, or that states the new rule while a neighbouring paragraph still
   assumes the old one, is worse than a stale one — *compose-task-chains-k29*
   failed that way. Read whole sections, not matched lines.
3. **Verify the discriminator argument survived intact.** The `BRIEF.md` test was
   justified by the confirmation in several places. Its job changed (it now selects
   whether a closing node has close-time work); it did not vanish. A surface that
   deleted the justification instead of replacing it has silently made the
   brief-less rule look arbitrary, which puts `chain-as-node-k7` at risk.
4. **Verify the mutation tests were not weakened.** `src/tree_grow.rs`'s
   brief-absence assertion must still fail when a `BRIEF.md` is written into a
   chain node. Run the suite; do not read it.
5. **Check the walkthrough is honest.** `docs/workflows/multi-step.md` scripted a
   user answering *not yet*. If the rewrite kept the beat and only relabelled it,
   the walkthrough now depicts an interaction the design forbids.
6. **Read the new prose as a cold session would.** The procedure has four steps and
   an escalation carve-out; prose that states "the cascade no longer asks" without
   stating what it does *instead* has removed a gate and put nothing behind it.
   That is a finding.

Report findings with a reproduction (the grep, the command, the failing test) —
not an assertion. `chain-node-integrate-k11`'s triage rejected nothing accepted on
assertion alone, and that is the standard.

## Notes

**In scope: the decision itself, if the prose cannot be written honestly.** The
producer was told not to re-litigate. You may. If reconciling a surface required
prose that misrepresents the rule, or exposed a case the ADR does not cover
(a node whose brief has no `Done when`; a subtree closed by pruning rather than
completion), that is a finding against the **ADR**, and it goes to
`confirmation-prose-integrate-k17` like any other.

**Out of scope:** the release lag (`content/` reaches sessions only when a release
is cut — known, recorded, owned elsewhere), and `CHANGELOG.md` entries
(`changelog-unreleased-k13`).

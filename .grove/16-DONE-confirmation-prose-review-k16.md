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

## Findings

Review target: `db9ccac4bf47e508c8881f1d932620c4f56f30b8`
(`confirmation-prose-k15`).

### Medium — the live root brief still makes asking depend on node species

The repository-wide claim grep found this current briefing text:

```text
.grove/BRIEF.md:59:and a brief-less node is never asked the Retire cascade's confirmation.
```

That sentence is one of the three arguments explaining why a chain becomes a
brief-less node. It still makes absence of `BRIEF.md` the reason a node is not
asked, while the same brief's later `retire-confirmation-k12` summary states the
new rule: **no** node close asks. The discriminator now decides whether the close
has work (`Done when` check and promotion), not whether it gets a question.

This is the exact parallel-guidance defect the task asked the review to find: a
future session bootstraps from this brief before the producer's rewritten
surfaces, so it can still recover the obsolete species-dependent rule from the
live mandate. `confirmation-prose-integrate-k17` should rewrite the causal claim
to the close-time-work argument; this review leaves it unfixed.

### Medium — the prior-art G5 heading contradicts its rewritten body

The same grep found a whole-section contradiction in
`docs/research/skill-repo-prior-art.md`:

```text
1159:**G5 [grove] — done-ness: a manual parent roll-up that *prompts* the human vs grove's
1160:implicit-via-absence that *asks* the human — convergent on "never auto-complete the parent," and
```

Lines 1167–1172 now correctly date the old rule and say grove dropped the gate,
but the section heading still presents “grove asks” and “convergent” as the
section's current conclusion. A cold reader, table-of-contents consumer, or
later summary can therefore take the old mapping without reaching the body that
retracts it. This is not immutable survey evidence: the heading is grove's
mapping of that evidence onto its own design, the very prose the producer's
annotation rule says must track the decision.

`confirmation-prose-integrate-k17` should make the heading describe the dated
mapping or the surviving drift-free convergence. This review leaves it unfixed.

## Confirmed checks

- The broad, hidden-path repository grep was run independently over source,
  docs, content, changelog history and `.grove/`, using ask/confirm/roll-up/
  cascade patterns rather than the producer's file list. Remaining matches in
  tagged changelog entries, retired leaves, the ADR's rejected options and
  explicitly dated research prose are historical rather than live guidance.
- `content/SKILL.md`'s Retire section is usable cold: it states all four
  verify-and-report steps, preserves the unnameable-gap escalation, explains why
  neither node species is asked, and recurses silently. The finish-cycle section
  identifies its confirmation as the only routine human gate.
- The `BRIEF.md` discriminator remains justified in `content/BRIEF-FORMAT.md`,
  `content/TASK-FORMAT.md`, `content/driving.md`, `docs/grove.md`,
  `src/llm_cli.rs` and `src/tree_grow.rs`: a brief-carrying close has a rollup to
  check and content to promote; a chain close has neither.
- `cargo test` passed in full. In particular,
  `tree_grow::tests::a_chain_node_carries_no_brief_so_its_close_has_nothing_to_do`
  executed and passed; its assertion still rejects a `BRIEF.md` emitted by either
  composite constructor. The environment-hygiene guards passed as part of the
  same run.
- The generated `grove-llm leaf-add-chain --help` text states the new rule, and
  `leaf-add-pair --help` carries no stale confirmation claim. The walkthrough no
  longer scripts “the user said not yet”: iteration 3 checks, cuts `k6`, and
  iteration 4 promotes and reports the node close with consistent example hashes
  and tree state.

## Externalized concerns

Two defects surfaced only because the review read the whole walkthrough and
executed generated help. They do not concern `confirmation-boundary`, so they
were cut as new leaves rather than absorbed here:

- `walkthrough-harness-routing-k19` — the walkthrough's Codex paragraph still
  says one bootstrap-chosen/stamped harness runs every task, despite current
  leaf/kind/family routing.
- `retire-help-node-path-k20` — `grove retire --help` still demonstrates the
  removed original-scheme node path `003-session-store`.

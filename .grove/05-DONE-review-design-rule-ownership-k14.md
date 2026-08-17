# rule-ownership-k14

**Reviews:** rule-ownership-k13

## Goal

Adversarially read the **repaired** `docs/specs/corpus-rule-ownership.md` and ADR
`corpus-rules-have-one-owner`. `rule-ownership-k13` did not patch the design — it
replaced the placement function's input, added a mirror-class taxonomy, recomputed
every owner cell and rewrote the inventory to five columns. That is a fresh design,
and nobody has read it adversarially.

The evidence that this artifact earns a second cycle is direct: the first review of
the first design found **four P1 defects**, and eight leaves execute against
whatever this map says. The integrating session spent no in-session reviewer,
precisely so the budget went here instead.

## Context

- `docs/specs/corpus-rule-ownership.md` — the artifact under review.
- `docs/adr/corpus-rules-have-one-owner.md` — the decision it rests on.
- `.grove/03-DONE-review-design-rule-ownership-k12.md` and
  `.grove/04-DONE-integrate-review-design-rule-ownership-k13.md` — the findings
  and the repair's claimed disposition of each.
- `.grove/BRIEF.md` — the requirements, and the reconciled work order.
- `src/prompt.rs:136` (`reference_file`) — the fixed runtime.

## The claims to attack

1. **Is the ordered precedence actually total and deterministic?** It claims first
   match wins over five `Occasion` values and a Bound test. Find a real rule in
   `content/` where two `Occasion` values are equally defensible and the ordering
   therefore decides the owner by which reading you happened to pick — that is the
   old ambiguity moved, not removed. `records-are-current-state`,
   `escalation-names-the-tradeoff` and `durable-artifact-set` are the likeliest.
   The repair concedes Occasion is a judgement; check the concession is honest and
   not doing more work than it admits.

2. **Does the `own` class reopen the hole the register rule closed?** `own` lets
   `SKILL.md` carry a list. The test offered is "no procedure remains to defer",
   which is a judgement about a *file that does not exist yet*. Try to justify a
   third or fourth `own` row under that test — the doubt budget, the triage
   mapping, the kind set. If the test cannot refuse them, `own` is an escape hatch
   and the 700–900-word target is not defended by anything.

3. **Does the arithmetic survive being written?** The spec claims 7 `own` rows
   ≤310 words, 19 `trigger` sentences ≤25 words each, total 700–900. Draft the
   seven `own` rows and five of the hardest triggers as real prose and measure.
   `integration-placement` in ≤25 words without the test, and
   `spine-seven-constraints` in ≤310 words *with* the routing table, are the two
   most likely to break. If they do, the budget is a wish and
   `skill-router-k4` will discover it at the worst moment.

4. **Is the relocation table for `driving.md` complete this time?** The first
   design claimed the residue was non-normative and was refuted by direct
   inspection. Re-run that inspection against the table rather than trusting it.
   An imperative with no row is a rule about to be deleted, and the whole
   conditional-deletion mechanism is worthless if the condition is checked against
   an incomplete list.

5. **Is the reachability chain rule sound, or does it just defer the problem?**
   Rows now name a triggering file, and chains must terminate at a static path.
   Walk every `@` reference in the inventory and find a cycle, a dangling
   terminus, or a chain so long that the "trigger" is not plausibly reached. The
   `TASK-FORMAT.md` / `BRIEF-FORMAT.md` chain through `decompose.md` is the one
   the repair leans on hardest to keep `SKILL.md` small.

6. **Is the inventory complete *now*, per its own completeness rule?** Pick two
   `content/` files the repair did not name as problems — `TASK-FORMAT.md` and
   `references/finish.md` are good candidates — and check that every normative
   sentence resolves to a row, a declared mirror, or a relocation. The repair
   audited every file; test whether the audit held on files it was not looking for
   defects in.

7. **Does the B★ / B / S split hold at the boundary?** The repair moved
   `grilling-threshold` out of `behavior-evals-k3` because it cannot be green
   before its fix. Check no *other* B★ row has the same property — a rule the
   corpus does not carry correctly today would make `k3` red on arrival, which is
   the failure the split exists to prevent. `externalize-by-default` and
   *retire → commit → complete* are worth testing against the current corpus.

8. **Is the ADR still one decision?** It now carries a five-value enumeration, a
   six-rule ordering and a three-class taxonomy. That may be one decision stated
   completely, or three decisions in one record — in which case the set is not
   minimum-coherent and it should split. Argue it either way, but argue it.

## Done when

- Each claim above is confirmed or refuted, with the evidence.
- Findings are recorded as findings only — this session runs no fix and no
  rewrite. Anything worth acting on goes into an `integrate-review-design` leaf,
  cut as this session's last act and inserted where `pick` reaches it next,
  carrying the findings verbatim. A review that finds nothing worth acting on
  creates nothing and simply retires.

## Notes

- **Inspection only.** Read the committed artifacts, the requirements and the
  corpus. Do not edit `content/`, `docs/`, or the tree beyond cutting the
  integration leaf.
- **Re-run every sweep rather than trusting a count.** The repair's own
  supporting claims — 19 `constraint N` citations across 8 files, 6 of them
  corpus files other than `SKILL.md`; 19 distinct trigger sentences over 27
  trigger rows; 14 deferrals across 9 files — were taken with normalised,
  controlled sweeps, and an unnormalised one reads clean on a wrapped or bolded
  match. Two such misses happened during the producing session.
- The repair spent no in-session reviewer, so nothing in it has been read
  adversarially. Its own reasoning is the thing most likely to be wrong, since
  it was produced by the session that also decided it was right.

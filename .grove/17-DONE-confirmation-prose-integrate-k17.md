# confirmation-prose-integrate-k17

**Kind:** integrate-review-impl

## Goal

Triage `confirmation-prose-review-k16`'s findings and apply the real ones.

## Context

Read `docs/adr/confirmation-boundary.md`, then `k15`'s diff, then `k16`'s findings
— in that order. The ADR is the authority on what the prose should say; `k16` is
evidence about whether it says it.

## Done when

- Every finding is **reproduced before it is touched** — run the grep, run the
  test, read the section. `chain-node-integrate-k11` accepted nothing on assertion
  and that is the standard here.
- Every finding is either applied or **explicitly upheld as rejected**, with the
  reason. A rejected finding that is worth remembering goes in the commit message;
  one that changes a recorded decision goes in the ADR set.
- A finding against the **ADR** itself (see `k16`'s Notes) is reworked **in place**
  in `docs/adr/confirmation-boundary.md` — never appended as a superseding record —
  and every citation of it is reconciled: `docs/adr/pruning.md`,
  `docs/adr/task-tree-scheme.md`, `docs/adr/task-kind-taxonomy.md`,
  `docs/adr/in-session-finish-cycle.md`, `docs/specs/task-kind-taxonomy.md`,
  `CONTEXT.md`. Grep the slug; do not walk that list.
- The claim grep is **re-run after the fixes** and comes back clean. Applying a fix
  is not evidence the claim is gone.

## Notes

**If the triage decides the whole decision is wrong**, that is a legitimate
outcome and it is HITL — say so and stop rather than reverting the ADR set
unilaterally. Reversing a decision three ADRs and the glossary now cite is a
commitment-shaped call, and this leaf is AFK.

**Do not absorb adjacent work.** `CHANGELOG.md` is `changelog-unreleased-k13`;
five stale `src/` module headers are `stale-module-headers-k14`. If this leaf
surfaces something new, `leaf-add` it.

## Triage

Both of `k16`'s findings **reproduced by grep before being touched**, both are
real, and both are applied. Nothing was accepted on assertion and nothing is
upheld as rejected — there was no third candidate to reject.

1. **The live root brief (Medium) — applied.** The `chain-as-node-k7`
   carry-forward's third lapsed argument now reads *"a brief-less node's close has
   no close-time **work**"*, which is the discriminator's actual job since
   `retire-confirmation-k12` (`docs/adr/confirmation-boundary.md` § Consequences,
   "doubly dead"). The falsified sentence is kept **quoted-and-refuted** rather
   than deleted — the pattern `docs-reconcile-k6` set — so a session that recalls
   the old rule finds it explicitly retracted instead of merely absent.
2. **The prior-art G5 heading (Medium) — applied.** It now leads with the durable
   divergence (grove's implicit done-ness cannot drift) and dates the convergence
   as survey-time evidence that lapsed on grove's side. The body at 1169–1174 was
   already correctly dated by `k15`, so this closes the heading/body split rather
   than changing the survey's evidence.

**A third instance of finding 2's claim, found by the post-fix grep, not by
`k16`.** The *same document's* own Takeaways roll-up still read *"**G5** is a
convergence — both refuse to auto-complete a parent and defer to the human"*. Same
claim, same file, one structural level up — a roll-up restating a section it
summarises. Applied in the same pass and **not** externalized: it is not new work,
it is the flagged finding's own extent, and `k16` reported the heading only because
it read the section rather than the chapter. The generalisable form: **when a
finding is against a heading, check the document's summary layer too** — headings
and roll-ups are the two places a corrected body does not reach.

**No finding against the ADR** (`k16` raised none, and the triage found no case it
fails to cover), so `docs/adr/confirmation-boundary.md` is untouched and the
"rework in place" criterion is vacuous rather than skipped. The slug was greped
anyway: 21 files cite `confirmation-boundary`, and nothing this leaf changed
disturbs a citation.

**Two grep traps worth carrying, both of which manufacture a false clean.**
`rg -E` is `--encoding`, **not** GNU grep's extended-regex flag — paired with
`2>/dev/null` it turns a flag error into empty output that reads exactly like a
clean result. And `.grove/` is a **dotdir**, so `rg` skips the live mandate without
`--hidden` — which is why `k16` called its sweep "hidden-path". Both matter beyond
this leaf: this grove's standing instruction is to re-run a claim grep as
*evidence*, and both traps produce evidence-shaped silence.

`cargo test` green in full (0 failed across every binary). No code changed — the
diff is two markdown files — so the run confirms the tree rather than the work.

# corpus-split-k30

**Integrates:** corpus-split-k26

## Goal

Triage and integrate the actionable findings from the adversarial review of
`corpus-split-k6`: disposition one normative source-verification/doubt rule that
was deleted without an inventory row, remove the surviving procedure-register
duplicates, and make the provenance comments describe the rules that actually
moved.

## Context

### Findings

1. **P1 — a source-verification rule was deleted with `driving.md` without an
   inventory row or a new owner.** The predecessor required a hard-to-reverse
   framework decision to pair its source citation with a fresh-context doubt pass
   (`3fb0fb0f^:content/driving.md:284-285`). The discharged relocation table
   accounts for that whole section only as
   `cite-framework-decisions-to-source` (`docs/specs/corpus-rule-ownership.md:1200-1211`),
   and the current `impl` owner ends after the source/citation procedure without
   preserving the hard-to-reverse trigger (`content/references/impl.md:8-27`).
   This is a session-conduct imperative, not argument: it changes whether an
   `impl` session spends its one review allowance. Give it a row and a reachable
   owner, or explicitly supersede it and correct the spec's claim that all other
   residue was only argument/history (`docs/specs/corpus-rule-ownership.md:1233-1239`).

2. **P1 — `references/grove.md` still states format-file conduct after the spec
   says those rules have one owner.** Its artifact table says the glossary is
   “appended inline” and the ADR set is “edited in place”
   (`content/references/grove.md:5-9`). Those are shortened second statements of
   `glossary-is-the-forcing-function` (`content/CONTEXT-FORMAT.md:15-24`) and
   `adr-set-is-minimum-coherent` (`content/ADR-FORMAT.md:50-70`), not parts of
   `durable-artifact-set`; the spec expressly says `grove.md` keeps neither half
   of the glossary rule (`docs/specs/corpus-rule-ownership.md:963-979`). A session
   reaches both copies, so either can drift while the phrase-scoped ownership test
   stays green. Keep the table's artifact/path/lifetime role and point to the
   format files for their conduct.

3. **P1 — the ADR current-state procedure also survives in a kind reference.**
   `integrate-review.md` instructs `integrate-review-design` to “merge / split /
   delete, never a superseding record” (`content/references/integrate-review.md:5-9`),
   reproducing the operative choices owned by `ADR-FORMAT.md:56-70`. The
   kind-specific rule needs to say what this integration kind may change, but it
   can do that by pointing at `ADR-FORMAT.md`; restating the format procedure
   violates the one-owner rule and is exactly the paraphrase blind spot the
   review was chartered to attack.

4. **P2 — the moved material's provenance comments do not describe the resulting
   files completely.** `grilling.md` says the removed test-seam section now lives
   in `references/requirements.md` (`content/grilling.md:1-4`), while the file
   itself and the ownership spec show that the section split between that file
   and `SPEC-FORMAT.md` (`content/grilling.md:29-40`,
   `docs/specs/corpus-rule-ownership.md:996-1003`). Conversely,
   `CONTEXT-FORMAT.md` attributes only its new *Keeping the language sharp*
   section to the later bundled `domain-modeling` source
   (`content/CONTEXT-FORMAT.md:1-11`), leaving the newly moved
   `glossary-is-only-a-glossary` clause at `:67` outside that attribution even
   though it came from the same bundled `grilling.md` source. Update both headers
   so the deliberate divergence and moved provenance remain traceable.

### Confirmed verdicts

- The cross-file reachability graph is intact. `content/SKILL.md:73-116`
  literally names the seven loop-step/context owners and all three format owners;
  `content/references/decompose.md:24-28` names `TASK-FORMAT.md` and
  `BRIEF-FORMAT.md`; and `content/references/requirements.md:18-34` names
  `grilling.md`. Every named path exists and every non-static owner has an
  incoming edge.
- Apart from finding 1, the sentence-by-sentence comparison of the deleted
  `driving.md`, `TASK-FORMAT.md`, and `grilling.md` material found each surviving
  imperative in its declared current owner or correctly classified as CLI fact,
  human-operator guidance, argument, example, or history.
- The `grilling.md` `<what-to-do>` block is byte-identical to its pre-change
  state. The two additions at `docs/USAGE.md:185-211` are genuinely human-side
  actions — ask the LLM for its view and for pushback — and point at, rather than
  restate, the session-side rules.
- The producer commit does not touch `content/SIGNAL.md` or
  `content/SIGNAL-FINISH.md`.

## Done when

- All four findings are triaged against the current corpus and fixed or accepted
  visibly with a reason.
- Every retained normative rule has one embedded, reachable owner; the
  hard-to-reverse framework-decision sentence has an explicit disposition.
- `references/grove.md` describes the durable artifact set without duplicating
  format-file conduct, and `integrate-review.md` points to the ADR procedure
  instead of restating it.
- Both provenance headers account for every moved rule and destination.
- Relevant post-fix verification is run by this integration session.

## Notes

This review was inspection-only. It ran no test, build, lint, or format command
and edited no production or test file.

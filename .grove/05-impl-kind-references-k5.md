# kind-references-k5

## Goal

Make each of the ten session-kind reference files **incremental**: only its
deliverable, its permissions, its special verification, and its unique human
gate. Resolve the `requirements` interview-threshold contradiction here, with its
test.

## Context

Ten files serve the nineteen kinds (`content/references/{requirements,design,
planning,prototype,impl,review,integrate-review,research,combine-research,
finish}.md`), 2,133 words combined but very unevenly: `requirements` 574,
`planning` 297, `review` 150, `impl` 44, `prototype` 27.

Exactly one reference file is on a given session's loaded path, chosen by the
driver and named by path in the guaranteed core — so the session performs no
selection and needs nothing that orients it among the others. Today several
restate shared loop material and general methodology that `SKILL.md` or a
loop-step reference already owns.

**Incremental** means the file answers only: what does *this* kind produce; what
is it permitted and forbidden to do; what verification is special to it; and what
human gate, if any, is unique to it. `review.md`'s inspection-only permission and
`finish.md`'s human confirmation are the model — each is a real delta. Anything
true of every kind belongs upstream, once.

**The contradiction to resolve.** `requirements` **always** establishes *what* is
wanted; the full one-question-at-a-time grilling procedure runs **only** when
three or more interdependent questions are open. Today `references/requirements.md`
carries both the always-form and the threshold without reconciling them, and
`references/execute.md` states the always-form alone, so a session reading either
in isolation gets a different rule. State it once, canonically, per the
inventory's assignment — and add the behavioural test that a `requirements`
session's loaded path carries the threshold and not merely the always-form.

## Done when

- Each of the ten files carries only its own delta; no file restates a rule the
  inventory assigns elsewhere.
- The interview threshold has one canonical statement, the disagreeing file is
  fixed, and a test asserts the `requirements` loaded path carries it. This test
  may be red until this leaf lands — it is the one exception to the
  green-throughout rule, and it goes green here.
- Each file still answers its kind's question completely for a session that has
  read only the guaranteed core and `SKILL.md` — incremental means non-duplicating,
  not incomplete.
- `behavior-evals-k3` is still green.
- The kind→file routing check in `tests/methodology.rs` still passes; the ten
  files stay flat and one level deep.

## Notes

- Be careful with the small ones. `impl.md` at 44 words and `prototype.md` at 27
  may already be incremental, or may be under-specified — those are different
  diagnoses and only the second is work. Do not pad a file to look consistent
  with its siblings.
- The five `review-*` kinds share one file, as do the five `integrate-review-*`
  and the two research producers. A delta that is real for one member of a family
  and not the others is a signal worth surfacing, not something to average away.
- The **ADR AND-test** contradiction is the other one this grove resolves; if the
  inventory assigned its canonical statement to a kind reference rather than to a
  format file, it lands here too.
- Consider whether this artifact warrants review. Ten files each holding one
  kind's whole discipline is load-bearing; if you finish with genuine doubt about
  a family's delta, cut `review-impl` with that specific doubt written into it.

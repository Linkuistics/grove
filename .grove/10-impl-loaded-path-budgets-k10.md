# loaded-path-budgets-k10

## Goal

Replace the loose 500-line prose-shape ceiling with **per-session loaded-path
word/token budgets** measured over the finished corpus, so the corpus is held to
what a session actually pays to read rather than to the length of one file.

## Context

`tests/methodology.rs` currently asserts `content/SKILL.md`'s body is at most 500
lines. Its own doc comment concedes the weakness: *"it establishes nothing
semantic — 'no procedure in `SKILL.md`' has no classifier now that the unit
markers are deleted, and is a review obligation. **This passing is not evidence
for it.**"* Two further alarms sit beside it (the loop-section page check, the
routing-table check). This leaf replaces the ceiling; decide deliberately what
happens to the other two rather than sweeping all three.

A **loaded path** is per-kind: the guaranteed core in `${prompt}`, the provisioned
`SKILL.md`, that kind's reference file, and whatever a condition it meets sends it
to. Nineteen kinds share ten reference files, so there are nineteen paths through
one corpus and the budget is a table, not a number.

Measure it the way the runtime composes it — through `src/prompt.rs`, which
already resolves a kind to its reference file and composes the core — rather than
by re-implementing the composition in the test. A budget computed by a second,
parallel notion of what a session reads will drift from the real one and then lie.

The **load predicate** column of `rule-ownership-k2`'s inventory is the input: it
says which rules are on which paths, and therefore which files a kind's budget
must include.

Baseline for comparison, at the start of this grove: `content/` totalled 23,532
words; `SKILL.md` 3,152; the ten kind references 2,133 combined. The normal loaded
path for a typical kind was therefore roughly 3,200–3,700 words before the
guaranteed core.

## Done when

- A per-kind budget table exists and is asserted, derived from the real
  composition rather than a re-implementation of it.
- The 500-line ceiling is deleted, and the disposition of the loop-section alarm
  and the routing-table check is decided and recorded — the routing check earns
  its keep on different grounds and probably stays.
- The budgets are set from what the finished corpus measures, with headroom stated
  as a deliberate choice. A budget fitted exactly to today's bytes fails on the
  next legitimate sentence; one set at twice the measurement measures nothing.
- The test reports the measurement on failure, so a contributor sees which path
  grew and by how much rather than only that something did.
- A control shows each budget can fail.
- The acceptance comparison is recorded: normal loaded path per kind, before and
  after this grove, against the 3,200–3,700 word baseline.
- The whole suite is green, `behavior-evals-k3` included.

## Notes

- Word count and token count answer different questions and the requirement names
  both. Tokens are what a session actually pays and are model-specific; words are
  stable and reproducible with no dependency. If a token count needs a tokenizer
  dependency, weigh that honestly — a words-only budget that ships and runs on
  every `cargo test` beats a token budget that needs a network call.
- This is the **last** content-affecting leaf, so it is also where the grove's
  acceptance is visible. If the measurement shows the loaded paths did not shrink
  materially, say so plainly — that is a finding about the work, not a number to
  present favourably.
- The budget is a shape measure, and shape measures are what this grove is
  replacing. It earns its place only alongside `behavior-evals-k3`: the budget
  says the path is small, the evals say it still carries the rules. Neither is
  evidence for the other, and the test's own documentation should say so.

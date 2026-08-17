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

The **load predicate** column of the inventory is the input: it says which rules
are on which paths, and therefore which files a kind's budget must include. Its
notation distinguishes the two halves explicitly — `static(K)` for a rule on every
K-kind's fixed path, and `on(<trigger>) @ <file>` for one reached because a
condition in `<file>` fired.

Two cheap assertions ride with the budget, and both are corrections the design
needed rather than extras:

- **`static(...)` is checked against the runtime.** A row claiming `static(K)`
  whose owner is not `SKILL.md` or `reference_file(k)` for every `k ∈ K` fails.
  The superseded inventory labelled loop-step rows `always(19)`, which
  `src/prompt.rs` contradicts; this check is what would not have survived it.
- **Reachability is an edge, and the edge is what you assert — but only across
  files.** **Partition the conditional rows first.** A row whose `@` file *is* its
  own owner records an **in-file condition**, not a transition: the session has
  already opened that file, and the record says which part applies. 45 of the
  inventory's 92 conditional rows are that kind, so running a cycle check over the
  unpartitioned set makes every one of them a self-loop and fails on half the
  design. Such a row is asserted only to agree with its grouping heading, and it
  contributes nothing to the graph.
  For a **cross-file** row `on(t) @ F` with owner `O` (`F ≠ O`): the chain from `F`
  terminates at a static path with no cycles, **`F` literally names `O`'s path**,
  and every non-static owner file has at least one incoming edge. The
  chain-termination half alone is not the property — it passes for a row whose `F`
  is loadable and silent about `O`, which is how `references/driver.md` came to own
  seven rows with nothing anywhere pointing at it. The incoming-edge check is what
  makes the carve-out safe: **what is reached is a file, not a rule**, so one edge
  into `decompose.md` makes every in-file condition there available, and a file with
  no edge fails however many reflexive rows it has.
  The current graph is **14 edges realised by 47 rows**, every non-static owner a
  target, no cycles — recompute it rather than trusting that count. Two schema
  checks come free: a row whose `@` file is `SKILL.md` must be class `trigger` or
  share one, and every `trigger` row's sentence number must exist in the spec's
  canonical set of **26**. Together they are what would have caught `impl`'s two
  disciplines sitting in a file no `impl` session is ever routed to — present in
  `content/` and deleted in effect.

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

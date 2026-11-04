# pack-conformance-runner-k156

## Goal

Make Grove's conformance runner check one external pack alone and an
aggregate of core plus supplied external packs. Core keeps its current result.
This is the expand step: nothing moves yet.

## Context

- `docs/specs/external-pack-contract.md`, "References between skills" and
  "Conformance". The review leaf external-pack-contract-k163 and any
  integration step after it come first.
- `plugins/grove/conformance.sh` and `conformance.test.sh`. The runner reads
  one skills root, expects the spine at `<root>/grove` and resolves references
  in the skill's directory or the spine only. `print_bound_kinds` hard-codes
  the load-set names, `research` among them. The test script builds temporary
  roots with `new_skill_set` and `add_kind`.
- `Taskfile.yml`, `check:skills`, and `scripts/check.sh`, which run both
  scripts and shellcheck.

## Done when

- The runner takes `--pack <dir>` and `--alone`, as the contract's "Inputs"
  section states. Core alone gives the same result as before.
- Loaded paths are skill-qualified. They follow directed loads of the form
  "Load the `<name>` skill" and skill-relative paths, by rule 3 of "References
  between skills". The spine is on a kind's path only because the kind loads it.
- The runner reads a directed load by the match rules under "References between
  skills". These cover the letter case, emphasis, a line break, comments,
  fences and block quotes.
- The pack-alone checks and the aggregate checks hold as the contract lists
  them. That includes unique skill names and rule IDs across packs, and
  resolved loads. Alone, a path that reaches a loaded skill of another pack is
  an external resource, not a failure.
- A row whose owner file is absent fails. `static(23)` becomes `static(all)` in
  core's manifest. The runner drops `static(22)` and accepts `static({a,b})`.
- `conformance.test.sh` proves each row of "Controls that must fail" with a
  fixture external pack, not the real research pack. The fixture aggregate
  passes, and core alone keeps its result. The fixture pack holds the clean
  cases that the contract lists under that table.
- The fixtures follow the rules under that table. Each fixture ships every
  owner that its manifests name. Each control reads clean before its one
  mutation. Each reading asserts the exit status and the intended diagnostic.
  The current cases keep their behaviour on the changed fixtures.
- `plugins/grove/README.md`, "Conformance", documents the pack-facing
  invocation. A pack's Taskfile can call it from a checkout of Grove.
- `task check:skills`, shellcheck and `bash scripts/check.sh` pass.

## Notes

Keep the runner compatible with bash 3.2 and free of dependencies. The rows of
the research files stay in core's manifest in this leaf. Their move, and the
removal of the named set `research`, belong to grove-research-removal-k158.

## Decisions (running log)

- Baseline before the change: core alone passes with 116 rows single-source
  checked, 2783 loaded-path checks, 72 unpinned rows, 0 unbound rows and 1
  prompt row. The runner takes 48 s. The 21 controls pass in 170 s.
- The runner keeps its bash front end for the arguments and the inventory. One
  awk program does the extraction, the resolution, the paths and the checks.
  The bash version forks a process for each row and kind, and an aggregate
  multiplies that cost. awk is part of the POSIX base, so the runner stays free
  of dependencies.
- A supporting file of a skill uses its own directed loads first, then the
  loads of its skill's `SKILL.md`. A session reads such a file only after that
  `SKILL.md`. Without this reading, `grove-requirements/grilling.md` fails,
  because it names `ADR-FORMAT.md` and loads no skill itself. The contract's
  rule 3 gains that sentence.
- Core alone is the aggregate with no external pack. A directed load in core
  that no shipped skill answers therefore fails. Only `--alone` lists an
  external edge or an external resource without a failure.
- The runner matches a directed load with its words folded to lower case, as
  the phrase sweep folds case.
- The runner drops `static(23)` and `static(22)`. A leftover spelling is then
  an unreadable predicate and fails. `static(research)` stays until
  grove-research-removal-k158.
- A test fixture that reads core's manifest keeps only the rows whose owner
  skills it ships. The filter works at the grain of the skill, so a deleted
  owner file inside a shipped skill still fails.
- After the change, core alone gives the baseline result exactly: 116, 2783,
  72, 0 and 1. The runner takes 0.13 s. The 98 controls take 6 s.
- A backtick fence holds three backticks, so it flips the parity of the split
  on backticks. In such a fence the name of an example falls outside the code
  span, so the parity alone hides the load. The fixture therefore has a second
  example in a tilde fence, which only the fence rule hides. A mutation that
  turns the fence rule off failed no control before that example existed.
- An emphasis marker outside the phrase ends a word anyway. The emphasis
  control therefore puts the markers inside the phrase, in
  "**Load the** `x` _skill_".
- Nine deliberate mutations of the engine each failed at least one control:
  the fence, comment, block-quote, line-wrap and emphasis rules, the duplicate
  row and duplicate skill checks, the owner's pack and the condition register.
  The engine ran under the macOS awk only, because neither GNU awk nor mawk is
  installed here.
- `plugins/grove/README.md` had no controlled-English conversion. Its new
  "Conformance" section is converted. A `tc: pending` region marks the older
  sections, as the spine's `references/execute.md` does.
- `docs/ravel-caller-surface-inventory.tsv` records a digest of the old
  `conformance.sh`. It is a frozen audit record and no check reads it, so it
  stays unchanged.
- Verification: `task check:skills` passed. `bash scripts/check.sh` failed one
  step, "Ravel caller assets and provenance", because the policy typecheck pins
  Bun 1.4.3 and the shell found Homebrew's 1.4.4. `mise exec bun@1.4.3 -- bash
  scripts/check.sh` then passed all 19 principal checks, with the default
  Ravel 1.1.0 front. The Ravel 2.0 lanes belong to external-pack-proof-k162.

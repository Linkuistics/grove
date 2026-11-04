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

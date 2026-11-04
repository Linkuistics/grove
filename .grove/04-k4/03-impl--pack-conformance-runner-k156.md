# pack-conformance-runner-k156

## Goal

Make Grove's conformance runner check one external pack alone and an
aggregate of core plus supplied external packs. Core keeps its current result.
This is the expand step: nothing moves yet.

## Context

- `docs/specs/external-pack-contract.md`, the conformance section that
  external-pack-contract-k155 writes.
- `plugins/grove/conformance.sh` and `conformance.test.sh`. The runner reads
  one skills root, expects the spine at `<root>/grove` and resolves references
  in the skill's directory or the spine only. `print_bound_kinds` hard-codes
  the load-set names, `research` among them. The test script builds temporary
  roots with `new_skill_set` and `add_kind`.
- `Taskfile.yml`, `check:skills`, and `scripts/check.sh`, which run both
  scripts and shellcheck.

## Done when

- The runner accepts the inputs that the contract names. It checks an
  external pack alone and the aggregate of core with the supplied packs. A
  cross-pack reference resolves to the named skill's directory in the supplied
  set.
- The three assertions hold over the aggregate. They are file existence,
  single ownership and the composed loaded path.
- Deliberate controls fail as the contract requires. Include a removed
  required resource, a removed cross-pack edge and a duplicate normative
  owner across packs.
- A fixture external pack exercises the new paths in `conformance.test.sh`.
  It is not the real research pack.
- Core alone gives the same result as before. `task check:skills`,
  shellcheck and `bash scripts/check.sh` pass.
- The pack-facing invocation is documented where the contract says. A pack
  repository can then call it from its own Taskfile.

## Notes

Keep the runner compatible with bash 3.2 and free of dependencies.

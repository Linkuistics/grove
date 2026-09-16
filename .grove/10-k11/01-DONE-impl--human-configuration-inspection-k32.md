# human-configuration-inspection-k32


## Goal
Deliver working human `grove config show [--kind KIND]` over SessionConfig.



## Context
The parent carries the full approved inspection design. This increment exposes
only the human interface; k33 adds the JSON option and its acceptance.

## Done when
- Report source paths, selection and origin, include occurrences, admitted
  commands in key order, non-admitted keys, bindings, parameters, ordered
  literal/slot words, origins and assignment histories without truncation.
- Load and validate globally before requiring an optional kind. Errors name
  sources and remedies, stdout stays empty on failure, and usage exits 2.
- Process tests cover no tree, a held lease, stale ambient signal, unchanged
  working/configuration bytes, no launch or coordination creation, local source
  admission and active errors outside the requested kind.
- Help, user guide coverage, configuration and architecture documentation,
  crate docs and all affected source-exact books describe the shipped increment.
  Focused tests and `bash scripts/check.sh` pass.

## Notes
No JSON placeholder flag; the next child adds a working option.

## Decisions (running log)

Use the existing public SessionConfig load/require/inspect seam. Resolve the
workspace before loading so a subdirectory uses the worktree root, as launch
does. Format captured records in the human binary without another resolver.
The formatter identifies runtime slots separately from quoted literal values
and retains all origin/history records even when command display is filtered.

The one bounded adversarial review found the missing-kind reference's old
“two moments” claim excluded explicit inspection. Classified valid and fixed
by scoping it to lifecycle actions and adding the read-only require call. No
actionable implementation/test findings were reported. Source-exact inspection
found only the overview corpus affected; the formatter stays inside its recursive
production include, with no corpus exception. The overview final check passes.

Verification: seven human-inspection subprocess tests pass, alongside both CLI
surface tests and four guide-coverage tests. `bash scripts/check.sh` passes all
eight principal checks, including workspace tests and all six final book checks.
SHA-256 digests of all 1,813 tracked files (source, tests, fixtures, scripts,
manifests, documentation and Grove inputs) match before and after that run.
The parent stays live for json-configuration-inspection-k33; no cascade closes.

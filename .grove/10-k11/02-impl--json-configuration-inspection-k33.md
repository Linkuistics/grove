# json-configuration-inspection-k33


## Goal
Add `--json` to the working human inspector, completing the parent's agent
interface and inspection/launch equality acceptance.



## Context
Build on human-configuration-inspection-k32. Use the reviewed Inspection
encoding in docs/specs/modular-configuration.md, preserving response-local IDs,
tagged literal/slot words, assignment variants, nulls, source spans and lossless
native path encoding. Do not introduce a second resolver.

## Done when
- JSON success is one schema-version-1 object on stdout with the complete
  reviewed fields. JSON failures are one diagnostics object on stderr with
  empty stdout, including clap usage failures whenever --json was requested.
- Preserve exit codes 0/1/2 and full validation before --kind filtering.
- Process tests fill tagged words with a known context and compare against
  fake-launch argv under unchanged inputs, including literal slot-like values.
  Test native non-Unicode source paths and all encoding variants.
- Extend human acceptance to JSON: stale signal, held lease, no tree, source
  admission failures, no child or working/configuration/coordination writes.
- Update help, guide coverage inventory/tests, current-state docs, specs and
  source-exact books; remove only inspection-pending notices. Run focused
  tests and the root principal checks, then close k11 if its full contract holds.

## Notes
Examples remain configuration-examples-k12's responsibility.

The human increment provides `crates/grove/src/config.rs` and process fixtures
in `crates/grove/tests/config_show.rs`. Extend those fixtures through JSON and
cover the parent's global missing-personal-target refusal, override reset/unset
encoding, and secondary-workspace source precedence as part of full acceptance.
The formatter currently retains complete provenance tables under --kind; keep
all response-local references resolvable in the JSON projection too.

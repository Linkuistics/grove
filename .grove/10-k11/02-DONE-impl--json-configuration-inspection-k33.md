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

## Decisions (running log)

- Keep serialization in the human binary, projecting the existing public records
  without adding serialization policy to the generic resolver. Filter only the
  command array; retain complete provenance tables.
- Unicode paths encode as strings; non-Unicode paths encode as native integer
  arrays tagged `unix_bytes` or `windows_wide`. Diagnostics use the same encoder.
- Catch clap errors before dispatch when an exact `--json` option occurs before
  `--`; preserve successful help/version output and usage exit 2. Configuration
  errors preserve the existing structured diagnostics; other failures receive
  the same diagnostic shape with absent source fields.
- The bounded fresh-context review identified malformed `--json=true` as a
  structured-usage gap. Classified actionable: preserve clap's refusal but
  recognize the equals-form request too. A failing subprocess regression covers
  both `--json=true` and `--json=false`; no additional review is needed for this
  executable-seam fix.
- macOS rejects invalid-byte filenames. Native success/span projection is tested
  directly from captured records, missing native source diagnostics through the
  process, and existing invalid-byte source success additionally on Linux.

## Implementation plan

1. Extend subprocess acceptance with JSON success, parser/source failures,
   complete provenance variants, native paths and inspection/launch argv equality;
   observe the new contract fail before implementation.
2. Add the binary's JSON projection and parser-error handling, then run the
   focused process and CLI tests.
3. Update help, user guide/inventory, current-state references and all affected
   source-exact book fragments; run `bash scripts/check.sh` with fixed inputs.
4. Check k11's complete contract, retire k33, promote the inspection handoff,
   describe and seal the focused jj change, then signal completion.

## Verification

- The initial JSON success/usage tests failed against the human-only binary;
  the malformed equals-form regression also failed before its targeted fix.
- `cargo test -p grove --test config_show --test user_guide_coverage --bin grove`:
  17 process cases, five guide cases and three binary unit cases passed.
  The fake executable's NUL-delimited argv matches tagged-word substitution and
  SessionConfig expansion under unchanged configuration and a fixed context.
- `bash scripts/check.sh`: all eight principal checks passed, including locked
  workspace tests and final validation of all six books. The overview reconstructs
  five source files and 636 lines, with no deferred ranges.
- SHA-256 comparison of all 1,814 versioned input files before and after the
  principal run found no changes. Test logs and digest inventory were kept outside
  the repository in `/tmp/grove-k33-*`.
- Host verification was macOS. The Linux-only existing invalid-byte filename
  case and Windows-wide branch were not run here; native Unix source/span
  projection, native source failure and native argv were exercised on this host.
- k11's human and JSON contracts are covered together: shared admission, global
  validation, complete provenance, read-only observation independent of tree,
  lease and epoch, structured usage failures and argv equality. No ADR changes
  are needed: the delivered interface follows the existing configuration ADRs.

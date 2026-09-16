# configuration-inspection-k11 — brief


## Goal

Deliver `grove config show [--kind KIND] [--json]` as the read-only human and
agent inspection surface over the same compiled configuration Grove launches.



## Context

Depends on `workspace-configuration-k10`. Use SessionConfig's captured selection,
compiled word representation, provenance and diagnostics; do not implement a
CLI-specific resolver or reconstruct and split a displayed shell command.

The human CLI currently acquires its lease after the early `view` dispatch.
Configuration inspection must also dispatch before lease/loop setup. The user
approved the existing jj trackedness query's possible metadata snapshot. No
new `grove-llm` verb, session admission or read-only VCS variant is needed.

## Done when

- Help documents `config show`, its options, useful invocation examples and
  exit codes. Without a task tree, selected leaf or driver lease, it discovers
  workspace sources and performs the same complete load and admission as launch.
  Ambient stale session signals do not affect it; it neither creates coordination
  files nor writes a completion signal, configuration or working-tree bytes.
- Human output shows source paths, selection origin/list, include occurrences,
  commands in key order, non-admitted keys, reference chains, parameter winners
  and override histories, and executable/ordered words. Runtime slots are
  identified placeholders, not fabricated prompts. There is no availability
  probe, pager, output truncation or profile-override flag.
- `--kind` performs the full load first and then requires that kind. It cannot
  conceal broken active policy elsewhere. Unknown and non-admitted kinds have
  actionable source-attributed errors.
- JSON success is one schema-version-1 object on stdout using the reviewed
  Inspection encoding. Literals and slots, set and literal-template target
  histories, null optionals, native path encodings, spans and response-local IDs
  remain unambiguous and complete.
- JSON errors, including invalid CLI arguments when `--json` was requested, are
  one schema-version-1 diagnostics object on stderr with empty stdout. Exit
  codes are 0 for valid inspection, 1 for source/configuration/resolution failure,
  and 2 for usage. Check parser errors as well as handler errors.
- Process-level tests inspect a known configuration and fill its tagged words
  with a known runtime context; the result equals argv captured by the fake
  launch executable with unchanged inputs. Tests distinguish literal slot-like
  parameter contents from slots and preserve native non-Unicode paths.
- Tests cover no task tree, an already-held driver lease, a stale ambient signal,
  unknown options/missing option values, tracked/unreadable candidates, and
  active errors outside a requested kind. Observe unchanged configuration/tree
  bytes and no child or new coordination/completion files; allow jj metadata.

## Verification and documentation

Use SessionConfig plus human-CLI subprocess tests in temporary jj workspaces;
assert parsed JSON/exit status/stream placement and meaningful human content.
Reuse fake argv capture for inspection/launch equality. A report describes one
load only; explain the unchanged-input/context condition and next-session reload.

Update CLI help, `docs/USAGE.md`, `docs/CONFIGURATION.md`, architecture and crate
docs, and the overview book's human command surface. Update affected Grove
books and manifests if adapter exports/errors change. Remove inspection-pending
notices once true; examples remain delivery-pending. Keep the generic and Grove
specs/ADRs/glossary coherent and run the root brief's common checks.

In this same leaf, extend `docs/specs/user-guide-coverage.md` with inventory
rows for the working `config`/`config show` surface, options, streams and exit
codes. Supply their `docs/USAGE.md` anchors and update
`crates/grove/tests/user_guide_coverage.rs` as required to enforce those rows.
Update the CLI subcommand-surface assertions to the delivered help. The guide,
its coverage inventory and the binary must agree; no future examples row or
placeholder command is required before `k12`.

## Decomposition

`human-configuration-inspection-k32` delivers the complete human `config show`
surface with `--kind`, shared admission, read-only process acceptance, help and
current-state documentation. `json-configuration-inspection-k33` adds `--json`,
the lossless versioned encoding and structured parser errors, and the tagged-word
inspection/launch equality and native-path acceptance. Each child owns its
source-exact books and the root principal checks. The parent remains live until
both independently useful interfaces satisfy the full contract above.

## Notes

`configuration-examples-k12` adds the second config subcommand. This leaf need
not create a placeholder examples command; each advertised command must work.
The main/CLI result path may need adjustment to meet JSON usage errors; include
that source in the overview book rather than bypassing the process-level test.

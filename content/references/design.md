<!-- file: order=11 -->
<!-- unit: task-producer-design kinds=design class=triggering -->
- **design** (AFK) — given requirements, establish *how*. The deliverable is a
  spec, an ADR set, or both. A `design` session that finds itself cutting
  *implementation* leaves has drifted into planning's job and should externalize
  a `planning` leaf instead.
<!-- unit: task-deliverable-design kinds=design class=triggering defers="adr-placement-note spec-set-is-current-state" -->
- **design** raises ADRs **sparingly** — only decisions hard to reverse,
  surprising, or a real trade-off (`ADR-FORMAT.md`) — and MAY write a spec
  (`docs/specs/<slug>.md`) when the increment is a genuine agreement point
  (`SPEC-FORMAT.md`).

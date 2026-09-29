# Harness selection and execution — visual design

The [area specification](../../specs/harness-selection-and-execution.md) is the
behavioral contract. This presentation uses the same package, execution and
original-creator topics. It is a design for independent review, not implemented
product behavior or human approval of the newly resolved design details.

Run `task design:harness-selection` from the repository root, then open
[the discussion](http://127.0.0.1:8769/#discussion). The server exposes this
presentation directory only. `PORT=...` is a Task variable for choosing a
different port. The `.mmd` sources and `diagrams.json` are the editable artifacts;
the viewer renders with pinned Mermaid 12.0.0 from jsDelivr and requires network
access. There are no generated image exports to keep in sync.

Graph views start at a dark, bold **Start here** node; ordered edges are numbered.
Sequence views start with the first message. These views distinguish source
packages and process participants; none claims to be a C4 deployment container
or a formally checked state machine.

Current changes:

- [Package boundary → Caller, dispatcher and policy packages](http://127.0.0.1:8769/#diagram-packages) — Assigned selection computation to the compiled worker and kept generic admission, records and exec in Rust.
- [Execution → Prepare, record and replace the foreground process](http://127.0.0.1:8769/#diagram-handoff) — Put worker cleanup and required record persistence before the final handoff.
- [Original creator → Resolve one creator before selecting a reviewer](http://127.0.0.1:8769/#diagram-creator) — Made registration and incomplete-provenance refusal visible before provider comparison.
- [Runtime evidence](runtime-evidence.md#native-probe) — Recorded the native TypeScript/no-runtime-path smoke test and ambient-loading positive controls.

The specification's [acceptance table](../../specs/harness-selection-and-execution.md#test-seams)
maps requirements to the two agreed process seams. Linux target execution,
packaging and full cancellation/terminal tests belong to implementation and
release; this document does not present them as completed experiments.

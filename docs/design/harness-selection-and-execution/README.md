# Harness selection and execution — visual design

The [area specification](../../specs/harness-selection-and-execution.md) is the
behavioral contract. This presentation uses the same package, execution and
original-creator topics. It is a design for independent review, not implemented
product behavior. The human chose the original-creator mechanism; its details,
like the rest, are not a human approval of the newly resolved design.

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

Current changes, from the original-creator redesign:

- [Original creator → Where each option keeps the creator's provider](http://127.0.0.1:8769/#diagram-creator-options) — New: compares the four options by where provenance lives and which standing rule each amends; the human chose A.
- [Original creator → Producer names its run; the review resolves it](http://127.0.0.1:8769/#diagram-creator-flow) — New: traces the chosen mechanism from the producer's launch to the review's provider check.
- [Original creator → Resolve the Creator line before selecting a reviewer](http://127.0.0.1:8769/#diagram-creator) — Replaced registration lookup with the Creator line, fail-closed review recognition and the catalog membership check.
- [Package boundary → Caller, dispatcher and policy packages](http://127.0.0.1:8769/#diagram-packages) — Corrected the adapter label: it reads only the supplied task file.
- [Source grounding](runtime-evidence.md#source-grounding) — Replaced the dispatch-UUID note with run-ID provenance and counted three proposed slots.

The [native probe](runtime-evidence.md#native-probe) and the
[integration probe](runtime-evidence.md#integration-probe) remain the runtime
evidence for the worker's ambient-loading controls and embedded specifiers.

The specification's [acceptance table](../../specs/harness-selection-and-execution.md#test-seams)
maps requirements to the two agreed process seams. Linux target execution,
packaging and full cancellation/terminal tests belong to implementation and
release; this document does not present them as completed experiments.

# Harness selection and execution — visual design

The [area specification](../../specs/harness-selection-and-execution.md) is the
behavioral contract of the `harness-dispatch` command and of Grove's use of it.
This presentation shows the same package, execution and original-creator topics
as views. Where a view and the specification differ, the specification binds.

Run `task design:harness-selection` from the repository root, then open
[the viewer](http://127.0.0.1:8769/). The server exposes this presentation
directory only. `PORT=...` is a Task variable for choosing a different port.
The `.mmd` sources and `diagrams.json` are the editable artifacts; the viewer
renders with pinned Mermaid 12.0.0 from jsDelivr and requires network access.
There are no generated image exports to keep in sync.

Graph views start at a dark, bold **Start here** node; ordered edges are numbered.
Sequence views start with the first message. These views distinguish source
packages and process participants; none claims to be a C4 deployment container
or a formally checked state machine.

| View | What it shows |
|---|---|
| [Caller, dispatcher and policy packages](http://127.0.0.1:8769/#diagram-packages) | Which package owns what, that the policy returns the command, and that the Grove adapter is an explicit policy import |
| [Prepare, record and replace the foreground process](http://127.0.0.1:8769/#diagram-handoff) | Selection, the handoff record and the exec, for `inspect` and for `run` |
| [Grove's two launches: lifecycle and confined standalone](http://127.0.0.1:8769/#diagram-grove-launch) | What Grove passes, where a refusal lands, and why a confined invocation selects through `inspect` |
| [Where each option keeps the creator's provider](http://127.0.0.1:8769/#diagram-creator-options) | The options the [creator-reference decision](../../adr/a-review-carries-its-creator-reference.md) weighed, and the one chosen |
| [Producer names its run; the review resolves it](http://127.0.0.1:8769/#diagram-creator-flow) | The creator reference from a producer's launch to its review's provider check |
| [Resolve the Creator line before selecting a reviewer](http://127.0.0.1:8769/#diagram-creator) | The supplied review policy's path to a selection or a refusal |

The [runtime evidence](runtime-evidence.md) records what was observed of the
runtime the worker is built on: the ambient-loading controls, the embedded
specifiers, the worker's directory and the installed smoke test at each
target's floor. The specification's
[acceptance table](../../specs/harness-selection-and-execution.md#test-seams)
maps the acceptance cases to the two agreed process seams.

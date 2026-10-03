# Harness selection and execution — visual design

The [area specification](../../specs/harness-selection-and-execution.md) is the
behavioral contract of the `harness-dispatch` command and of Grove's use of it.
This presentation shows the same package, execution and supervision, Grove
integration and original-creator topics as views. Where a view and the
specification differ, the specification binds.

Run `task design:harness-selection` from the repository root, then open
[the viewer](http://127.0.0.1:8769/). The server exposes this presentation
directory only. `PORT=...` is a Task variable for choosing a different port.
The `.mmd` sources and `diagrams.json` are the editable artifacts; the viewer
renders with pinned Mermaid 12.0.0 from jsDelivr and requires network access.
There are no generated image exports to keep in sync.

Graph views start at a dark, bold **Start here** node; ordered edges are numbered.
Sequence views start with the first message, and the state view with its initial
state. These views distinguish source packages and process participants; none
claims to be a C4 deployment container or a formally checked state machine: the
state view is a reading aid for the specification's supervision contract, not a
model.

| View | What it shows |
|---|---|
| [Caller, dispatcher and policy packages](http://127.0.0.1:8769/#diagram-packages) | Which package owns what, that the policy returns the command, that dispatch supervises it and reports the run ending, and the runner both sides use |
| [Prepare, record and supervise the harness](http://127.0.0.1:8769/#diagram-handoff) | Selection, the handoff record, the supervised run and its end observation, for `inspect` and for `run` |
| [Two nested jobs: driver, dispatch, harness](http://127.0.0.1:8769/#diagram-chain) | The process and terminal chain, who holds the terminal when, and where the exit signal goes |
| [A supervised run from selection to its ending](http://127.0.0.1:8769/#diagram-ending) | The run's states, the escalation, cancellation, and how the run ending is chosen |
| [Grove's two launches: lifecycle and confined standalone](http://127.0.0.1:8769/#diagram-grove-launch) | What Grove passes, the teardown record, where a refusal lands, and how a confined invocation launches through `run --confine` |
| [How the driver reads a reaped launch](http://127.0.0.1:8769/#diagram-loop-decision) | Interrupted, finished, relaunched or stopped, from the teardown record and the run ending |
| [Where each option keeps the creator's provider](http://127.0.0.1:8769/#diagram-creator-options) | The options the [creator-reference decision](../../adr/a-review-carries-its-creator-reference.md) weighed, and the one chosen |
| [Producer names its run; the review resolves it](http://127.0.0.1:8769/#diagram-creator-flow) | The creator reference from a producer's launch to its review's provider check |
| [Resolve the Creator line before selecting a reviewer](http://127.0.0.1:8769/#diagram-creator) | The supplied review policy's path to a selection or a refusal |

The [runtime evidence](runtime-evidence.md) records what was observed of the
runtime the worker is built on: the ambient-loading controls, the embedded
specifiers, the worker's directory and the installed smoke test at each
target's floor. The specification's
[acceptance table](../../specs/harness-selection-and-execution.md#test-seams)
maps the acceptance cases to the four agreed seams.

# Harness selection and execution — Grove caller views

The [caller specification](../../specs/harness-selection-and-execution.md) owns
Grove's use of independently installed Ravel. These views show public process
behavior, Grove's lifecycle/standalone interpretation and caller-owned creator
evidence. [Ravel's executable specification](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/specs/session-execution.md)
owns routing, private runtime/transport and supervision mechanics. Each project
keeps its own runner, version and complete installation prefix.

Run `task design:harness-selection` from the repository root, then open
[the viewer](http://127.0.0.1:8769/). The server exposes only this presentation
directory; `PORT=...` chooses another port. The `.mmd` files and `diagrams.json`
are editable source artifacts. The viewer uses pinned Mermaid 12.0.0 from
jsDelivr and requires network access; no generated image exports are maintained.

Graph views start at a dark **Start here** node with numbered main steps.
Sequences start at their first message, and the state view at its initial state.
These are caller/process views, not deployment containers or checked models.
The result state view illustrates public behavior without asserting private
transport or escalation details.

| View | Contract illustrated |
|---|---|
| [Independent ownership](http://127.0.0.1:8769/#diagram-packages) | Public CLI between separate releases, caller imports and local runners. |
| [Route and return](http://127.0.0.1:8769/#diagram-handoff) | Unchanged caller prompt, refusal, supervision and opaque result after cleanup. |
| [Nested jobs](http://127.0.0.1:8769/#diagram-chain) | Grove's Ravel job, Ravel's managed job and terminal recovery. |
| [Result lifecycle](http://127.0.0.1:8769/#diagram-ending) | Successful publication versus absent result, cancellation or cleanup failure. |
| [Two Grove callers](http://127.0.0.1:8769/#diagram-grove-launch) | Lifecycle admission/teardown and confined standalone output publication. |
| [Post-reap decision](http://127.0.0.1:8769/#diagram-loop-decision) | Epoch gate, interruption, successful force-stop, teardown, surviving group and successful continuation. |
| [Creator alternatives](http://127.0.0.1:8769/#diagram-creator-options) | The [Creator decision](../../adr/a-review-carries-its-creator-reference.md)'s inline receipt/declaration and rejected store/today's-route alternatives. |
| [Creator handoff](http://127.0.0.1:8769/#diagram-creator-flow) | Caller wrapper, finishing producer and review's committed provider evidence. |
| [Configured review](http://127.0.0.1:8769/#diagram-creator) | Validation and different-provider routing by opt-in Grove policy assets. |

Ravel injects no ending instruction and gives result tokens no meaning. Current
Grove prompts name canonical `end-session grove-continue` and
`end-session grove-force-stop` commands. Successful force-stop overrides teardown
without undoing it and exits 1; standalone force-stop publishes no outputs.
Finish records Grove's teardown before continuing. **Already running original-v22 sessions** keep
`harness-dispatch exit` and their original Creator/admission/teardown contract;
no view changes their live channel.

The [runtime evidence locator](runtime-evidence.md) points to executable-owned
Ravel controls and immutable historical experiments. Grove's
[caller acceptance](../../ravel-caller-surface-audit.md) and
[delivery acceptance](../../grove-delivery-contraction.md) establish its own
public integration; neither duplicates Ravel's private runtime contract.

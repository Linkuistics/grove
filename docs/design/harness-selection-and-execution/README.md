# Harness selection and execution — Grove caller views

The [caller specification](../../specs/harness-selection-and-execution.md) owns
Grove's use of independently installed Ravel. These views show the public
process behaviour, Grove's lifecycle and standalone interpretation, and the
creator evidence that the caller owns. [Ravel's executable specification](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/specs/session-execution.md)
owns routing, the private runtime and transport, and the supervision mechanics.
Each project keeps its own runner, version and complete installation prefix.

Run `task design:harness-selection` from the repository root, then open
[the viewer](http://127.0.0.1:8769/). The server exposes only this presentation
directory. `PORT=...` chooses another port. The `.mmd` files and `diagrams.json`
are editable source artifacts. The viewer uses pinned Mermaid 12.0.0 from
jsDelivr and needs network access. Nobody maintains generated image exports.

Graph views start at a dark **Start here** node with numbered main steps.
Sequences start at their first message, and the state view at its initial state.
These are views of the caller and its processes, not deployment containers or
checked models. The result state view illustrates the public behaviour. It
asserts no private transport or escalation details.

| View | Contract illustrated |
|---|---|
| [Independent ownership](http://127.0.0.1:8769/#diagram-packages) | Public CLI between separate releases, caller imports and local runners. |
| [Route and return](http://127.0.0.1:8769/#diagram-handoff) | Unchanged caller prompt, refusal, supervision and opaque result after cleanup. |
| [Nested jobs](http://127.0.0.1:8769/#diagram-chain) | Grove's Ravel job, Ravel's managed job and terminal recovery. |
| [Result lifecycle](http://127.0.0.1:8769/#diagram-ending) | Successful publication against an absent result, cancellation or cleanup failure. |
| [Two Grove callers](http://127.0.0.1:8769/#diagram-grove-launch) | Lifecycle admission and teardown, and confined standalone output publication. |
| [Post-reap decision](http://127.0.0.1:8769/#diagram-loop-decision) | Epoch gate, interruption, successful force-stop, teardown, surviving group and successful continuation. |
| [Creator alternatives](http://127.0.0.1:8769/#diagram-creator-options) | The inline receipt or declaration of the [Creator decision](../../adr/a-review-carries-its-creator-reference.md), and its rejected alternatives of a store or today's route. |
| [Creator handoff](http://127.0.0.1:8769/#diagram-creator-flow) | Caller wrapper, finishing producer and the committed provider evidence of the review. |
| [Configured review](http://127.0.0.1:8769/#diagram-creator) | Validation and routing to a different provider by opt-in Grove policy assets. |

Ravel injects no ending instruction and gives result tokens no meaning. Current
Grove prompts name the canonical `end-session grove-continue` and
`end-session grove-force-stop` commands. A successful force-stop overrides
teardown without undoing it, and exits 1. A standalone force-stop publishes no
outputs. A finish records Grove's teardown before it continues.

**Already running original-v22 sessions** keep `harness-dispatch exit` and their
original Creator, admission and teardown contract. No view changes their live
channel.

The [runtime evidence locator](runtime-evidence.md) points to the controls that
the executable owns and to immutable historical experiments for Ravel. Grove's
[caller acceptance](../../ravel-caller-surface-audit.md) and
[delivery acceptance](../../grove-delivery-contraction.md) establish its own
public integration. Neither duplicates Ravel's private runtime contract.

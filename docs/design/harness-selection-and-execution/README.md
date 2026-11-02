# Harness selection and execution — Grove caller views

The [caller specification](../../specs/harness-selection-and-execution.md) owns
Grove's use of independently installed Ravel. These views show the public
process behaviour, Grove's lifecycle and standalone interpretation, and the
creator evidence that the caller owns. [Ravel's executable specification](https://github.com/Linkuistics/Ravel/blob/v1.1.0/docs/specs/session-execution.md)
owns routing, the private runtime and transport, and the supervision mechanics.
Grove keeps no runner and runs `ravel` as an ordinary child. Ravel keeps its
own runner. Each project keeps its own version and complete installation prefix.

The skill views describe the pending adoption design. The user supplies skills
natively or through Ravel directory injection. Grove names them, and the agent
finds and reads them.

Ravel's reviewed directory contract is commit `822c0ed2` on its unpublished
`skill-directory-delivery` line. The historical `b93bb927` contract supplies
intent only. Grove no longer provisions Codex skills. Grove accepts only Ravel
1.x from 1.1.0 today. The admission of 2.0.0, its Rhai caller assets and
directory delivery remain ahead. See
[design evidence](../../grove-skill-delivery-design-evidence.md#upstream-correction).

Run `task design:harness-selection PORT=52710` from the repository root, then open
[the viewer](http://127.0.0.1:52710/). The server exposes only this presentation
directory. `PORT=...` chooses another port. The `.mmd` files and `diagrams.json`
are editable source artifacts. The viewer uses pinned Mermaid 12.0.0 from
jsDelivr and needs network access. Nobody maintains generated image exports.

Graph views start at a dark **Start here** node with numbered main steps.
Sequences start at their first message, and the state view at its initial state.
These are views of the caller and its processes, not deployment containers or
verified models. The result state view illustrates the public behaviour. It
asserts no private transport or escalation details.

| View | Contract illustrated |
|---|---|
| [Where each installed entry goes](http://127.0.0.1:52710/#diagram-owner-classes) | The per-entry rule of the owner cutover: a Ravel source, a native route for hooks, or no change. |
| [Pack checkouts outside Ravel](http://127.0.0.1:52710/#diagram-owner-pack-checkouts) | The owner's tool holds each pack checkout at its pin. Ravel only reads the directory, and a missing checkout refuses before any worker. |
| [Ordered owner cutover](http://127.0.0.1:52710/#diagram-owner-cutover) | Preparation in ordinary leaves, then the switch inside finish with its reader gate, probes and inverse. |
| [Finish keeps its guidance through the switch](http://127.0.0.1:52710/#diagram-owner-final-session) | Finish reads native guidance by absolute path. Later sessions move to Ravel 2.0.0. |
| [Compatible tap states](http://127.0.0.1:52710/#diagram-release-tap) | Four tap states: keg-only `ravel@2` first, compatible Grove, then promotion of identical archives with `ravel@2` deprecated. |
| [Publication and finish ownership](http://127.0.0.1:52710/#diagram-release-order) | Release prerequisites, Ravel publication, finish's Grove release and promotion, and the original ending. |
| [Independent ownership](http://127.0.0.1:52710/#diagram-packages) | Public CLI between separate releases, caller imports, Grove's shared job function and Ravel's runner. |
| [Route and return](http://127.0.0.1:52710/#diagram-handoff) | Intact caller task, optional directory injection, refusal, supervision and opaque result after cleanup. |
| [Supported Ravel routes](http://127.0.0.1:52710/#diagram-routes) | Native 1.1, native 2.0 and directory 2.0. The version probe decides admission only, and the configuration and settings select the route. |
| [User-managed skill delivery](http://127.0.0.1:52710/#diagram-skill-delivery) | User choice subject to actual read access. The agent locates the named skill or reports and force-stops. |
| [Preserving readers through adoption](http://127.0.0.1:52710/#diagram-skill-transition) | Use the integrated upstream contract, verify isolated combinations and intermediate tap states, then activate separately. |
| [Nested jobs](http://127.0.0.1:52710/#diagram-chain) | Grove's foreground job for `ravel`, signal forwarding, Ravel's managed job and terminal recovery. |
| [Result lifecycle](http://127.0.0.1:52710/#diagram-ending) | Successful publication against an absent result, cancellation or cleanup failure. |
| [Two Grove callers](http://127.0.0.1:52710/#diagram-grove-launch) | The foreground and detached jobs, lifecycle admission and teardown, and confined standalone output publication. |
| [Post-reap decision](http://127.0.0.1:52710/#diagram-loop-decision) | Epoch gate, failed spawn or wait, interruption, successful force-stop, teardown and successful continuation. Each error keeps a caught signal. |
| [Creator alternatives](http://127.0.0.1:52710/#diagram-creator-options) | The inline receipt or declaration of the [Creator decision](../../adr/a-review-carries-its-creator-reference.md), and its rejected alternatives of a store or today's route. |
| [Creator handoff](http://127.0.0.1:52710/#diagram-creator-flow) | Caller wrapper, finishing producer and the committed provider evidence of the review. |
| [Configured review](http://127.0.0.1:52710/#diagram-creator) | Validation and routing to a different provider by opt-in Grove policy assets. |

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

## Current changes

The current discussion settles who installs the pack checkouts and keeps them
current, now that Ravel reads local directories only. The human approved the
owner's tool and the pin policy. mise is the owner's choice, and Ravel and
Grove name no checkout tool. Three focused reviews of this design are
integrated. See the
[owner delivery specification](../../specs/owner-skill-delivery.md#pack-checkouts)
and its [evidence](../../owner-skill-delivery-evidence.md#pack-checkouts).

- [Pack checkouts outside Ravel](http://127.0.0.1:52710/#diagram-owner-pack-checkouts) — Added the owner's checkout tool, the pin advance, the offline fetch failure and the missing-source refusal. An advance runs only while nothing captures the checkout.
- [Ordered owner cutover](http://127.0.0.1:52710/#diagram-owner-cutover) — Added pack publication, the drafted checkout entries, the credential control and staging clones before the trials, and the checkout clone in preparation. Inside finish, the core tree comparison precedes the `grove-core` pin advance. Each run first reads the state of the switch set. After a native probe, a native state switches. A partial state resumes the inverse. An active state goes to the probes and the receipt. A drifted item stops for the owner's classification. Every stop, inverse or missing receipt leaves a pending-activation record, and the switch runs again. Step 1 also lists scheduled jobs and finish's descendants. Step 8 needs a rehearsal that covers the plan. The switch publishes each shared file by an atomic exchange. Only a switched layout with its receipt is complete.
- [Finish keeps its guidance through the switch](http://127.0.0.1:52710/#diagram-owner-final-session) — Added the core tree comparison and the `grove-core` pin advance before the probes. A comparison of accepted digests follows the advance. The probes capture the pack checkouts.

## Earlier delivery changes

This discussion mapped Ravel's reviewed Rhai directory contract into Grove's
routes. Its focused review is integrated. See
the [delivery specification](../../specs/codex-skill-provisioning.md#supported-routes)
and the [contract mapping](../../grove-skill-delivery-design-evidence.md#rhai-mapping).

- [Supported Ravel routes](http://127.0.0.1:52710/#diagram-routes) — Added the three supported routes, the admission probe and the refusals of a mismatched configuration.
- [Preserving readers through adoption](http://127.0.0.1:52710/#diagram-skill-transition) — Replaced the historical `b93bb927` contract and the open tap choice with the reviewed `822c0ed2` contract and the route that publishes `ravel@2` first.
- [Independent ownership](http://127.0.0.1:52710/#diagram-packages) — Named the configuration file and the caller-asset runtime of each major instead of an SDK.
- [Route and return](http://127.0.0.1:52710/#diagram-handoff) — Showed that 2.0.0 wraps the intact mandate after its preamble and keeps its bytes in `originalPrompt`. Renamed the owner policy to the owner configuration.

## Earlier owner changes

The owner skill cutover is a proposal, and the integration of its focused
review is complete. See the [owner delivery specification](../../specs/owner-skill-delivery.md)
and [evidence](../../owner-skill-delivery-evidence.md).

- [Where each installed entry goes](http://127.0.0.1:52710/#diagram-owner-classes) — Added the per-entry rule that routes each baseline entry to a Ravel source, a native route or no change.
- [Ordered owner cutover](http://127.0.0.1:52710/#diagram-owner-cutover) — Added the ordered cutover and the switch inside finish, with its probes and inverse.
- [Finish keeps its guidance through the switch](http://127.0.0.1:52710/#diagram-owner-final-session) — Added how finish keeps its native guidance through the switch. Later sessions move to Ravel 2.0.0. Changed by the review integration: the cache version survives because disabling writes no orphan marker, not because a reader protects it.

## Earlier release changes

The coordinated release design has its focused review integrated. The owner
acknowledged that this is the only live workstream through completion. The
design selects combined 2.0.0. See the
[release specification](../../specs/coordinated-release.md) and
[evidence](../../coordinated-release-evidence.md).

- [Compatible tap states](http://127.0.0.1:52710/#diagram-release-tap) — Added the versioned-first publication route and delayed unversioned promotion. Promotion deprecates `ravel@2` and moves default users to `config.rhai`.
- [Publication and finish ownership](http://127.0.0.1:52710/#diagram-release-order) — Added release dependencies, the finish extension and original-ending preservation.

## Earlier skill-delivery changes

- [User-managed skill delivery](http://127.0.0.1:52710/#diagram-skill-delivery) — Clarified confined native access and the missing-skill ending.
- [Preserving readers through adoption](http://127.0.0.1:52710/#diagram-skill-transition) — Recorded the integrated upstream contract, compatible tap ordering and optional manual cleanup.
- [Route and return](http://127.0.0.1:52710/#diagram-handoff) — Added optional directory injection without a named-skill resolver.
- [Independent ownership](http://127.0.0.1:52710/#diagram-packages) — Included configured directory hints before policy builds the command.
<!-- tc: depart synonym-rotation because "launch" is the domain term for running a harness session, and "Start here" is a node label -->
- [Two Grove callers](http://127.0.0.1:52710/#diagram-grove-launch) — Applied optional directory delivery to both lifecycle and standalone launches.

# Harness selection and execution — Grove caller views

The [caller specification](../../specs/harness-selection-and-execution.md) owns
Grove's use of independently installed Ravel. These views show the public
process behaviour, Grove's lifecycle and standalone interpretation, and the
creator evidence that the caller owns. [Ravel's executable specification](https://github.com/Linkuistics/Ravel/blob/v1.1.0/docs/specs/session-execution.md)
owns routing, the private runtime and transport, and the supervision mechanics.
Grove keeps no runner and runs `ravel` as an ordinary child. Ravel keeps its
own runner. Each project keeps its own version and complete installation prefix.

The skill views describe the adoption design. Homebrew installs the skill
sources, and Ravel delivers them, as homebrew-delivery-design-k216 proposes.
Ravel reads its default skill directory, as ravel-default-skill-directory-k218
settled.
Grove names each skill, and the agent finds and reads it.

Ravel's reviewed directory contract is commit `822c0ed2` on its unpublished
`skill-directory-delivery` line. The historical `b93bb927` contract supplies
intent only.

Grove no longer provisions Codex skills. Grove admits Ravel 1.x
from 1.1.0 and 2.x from 2.0.0. It supports the native routes on 1.1 and on 2.0,
with its TypeScript and Rhai caller assets. It supports the directory route on 2.0
through the lifecycle caller and the standalone caller.
See
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
| [Skill formulae outside Ravel](http://127.0.0.1:52710/#diagram-owner-pack-checkouts) | Homebrew installs each source in a keg and links it into Ravel's default skill directory after the keg is complete. Ravel only reads the directory. A source that is absent from it leaves the session without that source. |
| [Ordered owner cutover](http://127.0.0.1:52710/#diagram-owner-cutover) | Preparation in ordinary leaves, then the switch inside finish with its reader gate, probes and inverse. |
| [Finish keeps its guidance through the switch](http://127.0.0.1:52710/#diagram-owner-final-session) | Finish reads native guidance by absolute path. Later sessions move to Ravel 2.0.0. |
| [Compatible tap states](http://127.0.0.1:52710/#diagram-release-tap) | Four tap states: keg-only `ravel@2` first, compatible Grove, then promotion of identical archives with `ravel@2` deprecated. |
| [Publication and finish ownership](http://127.0.0.1:52710/#diagram-release-order) | Release prerequisites, Ravel publication, finish's Grove release and promotion, and the original ending. |
| [Independent ownership](http://127.0.0.1:52710/#diagram-packages) | Public CLI between separate releases, caller imports, Grove's shared job function and Ravel's runner. |
| [Route and return](http://127.0.0.1:52710/#diagram-handoff) | Intact caller task, optional directory injection, refusal, supervision and opaque result after cleanup. |
| [Supported Ravel routes](http://127.0.0.1:52710/#diagram-routes) | Native 1.1, native 2.0 and directory 2.0. The version probe decides admission only, and the configuration and settings select the route. |
| [Homebrew installs, Ravel delivers](http://127.0.0.1:52710/#diagram-skill-delivery) | Homebrew links the sources into Ravel's default skill directory, and Ravel reads them with no settings. Only hook adapters keep a native registration. The agent locates the named skill or reports and force-stops. |
| [Preserving readers through adoption](http://127.0.0.1:52710/#diagram-skill-transition) | Use the integrated upstream contract, verify isolated combinations and intermediate tap states, then activate separately. |
| [Nested jobs](http://127.0.0.1:52710/#diagram-chain) | Grove's foreground job for `ravel`, signal forwarding, Ravel's managed job and terminal recovery. |
| [Result lifecycle](http://127.0.0.1:52710/#diagram-ending) | Successful publication against an absent result, cancellation or cleanup failure. |
| [Two Grove callers](http://127.0.0.1:52710/#diagram-grove-launch) | The foreground and detached jobs, lifecycle admission and teardown, and standalone output publication. |
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

ravel-default-skill-directory-k218 settled Ravel's default skill directory and
its effect on the switch. Ravel 2.0 delivers the keg sources when the settings
name no `sources`, so the native state holds `"sources": []`. See
[the native settings inside finish](../../specs/owner-skill-delivery.md#the-native-settings-inside-finish)
and [the keg layout](../../specs/codex-skill-provisioning.md#the-keg-layout).

- [Ordered owner cutover](http://127.0.0.1:52710/#diagram-owner-cutover) — Added the native settings after promotion. The inverse returns `skills` to the off value, and a rerun goes back to the native settings.
- [Finish keeps its guidance through the switch](http://127.0.0.1:52710/#diagram-owner-final-session) — Finish writes the off value before the hooks installation.
- [Skill formulae outside Ravel](http://127.0.0.1:52710/#diagram-owner-pack-checkouts) — An advance unlinks the earlier keg first. A resolution during the installation delivers the other sources and does not refuse.
- [Supported Ravel routes](http://127.0.0.1:52710/#diagram-routes) — The source set decides the route, and `"sources": []` gives the native route on 2.0.
- [Homebrew installs, Ravel delivers](http://127.0.0.1:52710/#diagram-skill-delivery) — Ravel's installation record names the default skill directory.

## Homebrew delivery changes

This discussion applies the human's decisions of
skill-marketplace-decision-k201. Homebrew installs every skill source, and
Ravel is the only route that delivers a skill. homebrew-delivery-design-k216
proposes the design. See [the keg layout](../../specs/codex-skill-provisioning.md#the-keg-layout),
[the owner's formulae](../../specs/owner-skill-delivery.md#skill-formulae),
[the hooks installation inside finish](../../specs/owner-skill-delivery.md#the-hooks-installation-inside-finish)
and the [evidence](../../homebrew-skill-delivery-evidence.md).

- [Skill formulae outside Ravel](http://127.0.0.1:52710/#diagram-owner-pack-checkouts) — Replaced the mise checkouts with Homebrew formulae. An advance installs a new keg and links it into Ravel's default skill directory after the keg is complete.
- [Ordered owner cutover](http://127.0.0.1:52710/#diagram-owner-cutover) — Preparation builds the candidate keg and installs four formulae. Finish installs the writing hooks plugin after promotion. The switch then admits the released keg's digests. They replace the core tree comparison and the pin advance.
- [Finish keeps its guidance through the switch](http://127.0.0.1:52710/#diagram-owner-final-session) — Added the hooks installation, which names only `writing-hooks`. The probes capture the keg sources.
- [Homebrew installs, Ravel delivers](http://127.0.0.1:52710/#diagram-skill-delivery) — Removed the native route. Only the hook adapters keep a native registration.
- [Supported Ravel routes](http://127.0.0.1:52710/#diagram-routes) — The native routes find only existing native installations, and the directory route names keg sources.

## Earlier confinement changes

The withdrawal of confinement removes the confined standalone caller. Grove
sends no `--confine` and no `--runtime-read`, and the harness sandbox is the
only boundary of a standalone run. See the
[withdrawal record](../../confinement-withdrawal-evidence.md) and the
[standalone contract](../../specs/standalone-invocations.md#the-harness-sandbox).

- [Two Grove callers](http://127.0.0.1:52710/#diagram-grove-launch) — The standalone request holds the kind, the prompt and the exit directory. Ravel selects and supervises, and the harness sandbox is the only boundary.
- [User-managed skill delivery](http://127.0.0.1:52710/#diagram-skill-delivery) — A native route reads the native roots of the harness. No sandbox of Ravel hides them.
- [Supported Ravel routes](http://127.0.0.1:52710/#diagram-routes) — The caption names the standalone caller without confinement.

## Earlier recovery changes

The discussion before it rebuilt how the owner switch recovers from
interruptions and outside writes. The switch changes cells, and detection of
outside writes replaces the earlier exemptions. A run record tells an
interruption from an outside write, and the layout decides the state. Its
focused review is integrated. See the
[recovery sections](../../specs/owner-skill-delivery.md#the-switch-inside-finish),
the [recovery model](../../specs/owner-skill-delivery.md#the-recovery-model)
and its [evidence](../../owner-skill-delivery-evidence.md#the-recovery-model).

- [States of the owner switch](http://127.0.0.1:52710/#diagram-owner-switch-states) — Added the six states that step 3 reads and the events between them. Only an open run with a prefix of switched parts resumes without the owner. Changed by the review integration: a complete switch whose step 14 finds a changed entry outside the switch set inverts to the changed state.
- [Ordered owner cutover](http://127.0.0.1:52710/#diagram-owner-cutover) — Replaced the partial state with interrupted, drifted and changed states. The process listing repeats before each write interval. One program stages, exchanges and compares each part, and a conflict inverts the run's own parts. The receipt records the probed cells. Changed by the review integration: the listing excludes only the kept processes. Step 14 compares the entries outside the switch set before the manual copy, also in the complete continuation. The recovery nodes no longer share identities with preparation nodes.

## Earlier checkout changes

This discussion settled who installs the pack checkouts and keeps them
current, now that Ravel reads local directories only. The human approved the
owner's tool and the pin policy. mise is the owner's choice, and Ravel and
Grove name no checkout tool. Three focused reviews of this design are
integrated. See the
[owner delivery specification](../../specs/owner-skill-delivery.md#skill-formulae),
which homebrew-delivery-design-k216 later amended, and its
[evidence](../../owner-skill-delivery-evidence.md#pack-checkouts).

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

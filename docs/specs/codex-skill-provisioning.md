# Grove skill delivery

This is the adoption design, pending implementation and acceptance. Shipped
Grove 22.8 still provisions Codex snapshots and admits stable Ravel 1.x from
1.1.0. The document location remains stable for existing citations.

## Ownership

Grove names the skill its session must load. **The user ensures that skill is
available**, either through each harness's native installation or through
Ravel's mechanism. Grove and Ravel are decoupled with respect to skills. Grove
neither installs skills nor asks Ravel to resolve a named skill.

When configured, Ravel injects skill-directory paths into the launch prompt.
The agent uses those locations to discover and read applicable skills. Ravel
is not a requested-skill lookup service. Directory delivery does not establish
that a named skill exists, applies, or has been read.

The [owner-managed delivery decision](../adr/skill-sources-are-explicit.md)
records the removal of automatic provisioning and the bootstrap trade-off.
The [caller contract](harness-selection-and-execution.md) owns the public
session interface. Ravel owns source acquisition, directory delivery, prompt
preparation and resource access. Pack authors own their content and references;
the agent owns loading and following that guidance.

Grove's ordinary mandate, open kind labels, task filenames, handles, review and
integration routes, tree verbs and result meanings remain unchanged. Grove adds
no catalog reader, kind registry, presence check or discovery command. No skill
location crosses from Ravel back into Grove's task loop.

## Installation and bootstrap

A fresh installation supplies Grove's executables and readable caller policy
assets. The user also chooses a delivery route for the skills their work needs:

| Route | Setup obligation |
| --- | --- |
| Native harness installation | Install the spine, required kind skills and resources through that harness's supported plugin or manual installation route. Repeat for each harness that policy may select. |
| Ravel directory injection | Configure the desired source directories through Ravel; verify that the launched agent can discover and read the required skills and supporting resources there. Native copies are unnecessary. |

These routes are alternatives. Grove does not require Ravel discovery to be
enabled or a source-resolution command to have been run. No-source Ravel
launches remain valid. A native-only installation is a supported setup, not a
degraded fallback. Independently started harnesses can continue using their
native installations.

There is no embedded core-only fallback. Planning remains part of the core
skill pack as the replaceable default, but installing the Grove binary alone
does not install that pack. Setup guidance shows both routes and preserves
existing owner policy, settings and installations. Neither route silently
activates personal guidance or copies a sample policy over an existing one.

If the agent cannot find or read the mandated skill, it reports the missing
guidance and leaves the task live. The user repairs the selected delivery route
and reruns. Grove does not turn that failure into declaration-based kind
admission. A configured source that Ravel cannot acquire is instead a Ravel
prelaunch failure, with its source-specific remedy.

For Ravel delivery, accepted source requirements still include local sources
and exact pinned Git sources, offline reuse of warmed exact revisions, clear
failure for unavailable required revisions and readable resources throughout
reader lifetimes. Those are source-directory obligations, not resolution of a
skill requested by Grove. The earlier Ravel catalog design must be reconciled
with directory injection before implementation; the [design evidence](../grove-skill-delivery-design-evidence.md)
identifies that dependency and the user correction that governs it.

## Launch and compatibility

Grove passes the ordinary mandate and exact ending commands through the public
session CLI. Ravel may add directory-discovery instructions; it preserves the
caller task and endings intact. The agent discovers and loads the named skill.
Owner policy must pass the intended launch prompt to the harness. Grove neither
rewrites the selected command nor repairs a policy that drops the prompt.

Both lifecycle and standalone callers use that same session boundary. A
standalone runtime-file argument retains its existing meaning; Grove does not
translate skill directories into it or expand access itself.

Compatibility is about the public session interface, not skill delivery. Retain
stable Ravel 1.x support from 1.1.0, and admit stable 2.x from 2.0.0 after the
same caller contract has passed acceptance. Refuse 1.0, prereleases, unknown
majors, malformed version output and failed probes. Preserve absolute-PATH
selection, canonicalization and refusal without falling through.

Native-only users do not need Ravel 2 for skills. Ravel's directory-injection
feature requires the version that implements it. Policy examples and installed
checks must identify the Ravel SDK/schema they target; provide explicit
compatible examples for each admitted major rather than silently rewriting an
owner's policy. Grove does not parse policy/request schemas or private worker
protocols. Context and provider receipts retain their independent contracts.

| Failure | Remedy |
| --- | --- |
| Missing or incompatible Ravel | Select an installation supporting Grove's public session interface and a policy compatible with that installation. Avoid an undirected shared upgrade. |
| Missing named skill | Repair native installation or the configured directory set. Do not require switching routes. |
| Unavailable configured source/revision | Preserve Ravel's source-specific diagnostic; restore the source or deliberately amend owner configuration. |
| Read access, capture, cache, timeout or limit failure | Preserve Ravel's precise remedy; do not silently substitute another revision, skip guidance or retry unconfined. |
| Unsupported delegate access | Report that adapter's limitation before dependent work; do not imply that prompt text grants filesystem access. |

Prelaunch failures yield no successful session result. Grove keeps its normal
interruption and result interpretation and leaves selected work live. Lifecycle
scaffolding performed before launch is not promised a rollback.

## Existing installations and active readers

Remove automatic native provisioning and its embedded broad snapshot from
Grove's build and startup. Startup and package upgrade leave old native links,
current-snapshot pointers, snapshots, and foreign entries untouched. They do
not adopt, repair, repoint or delete them. Existing native installations remain
usable where their contents and resource paths are still valid.

The user can keep native installations indefinitely. Cleanup, if requested, is
an explicit operation after all affected readers and old provisioning writers
have stopped. Include drivers, lazy link readers, independent harnesses and
detached readers. A lexical path shape or skill name is insufficient deletion
evidence. Inspect each link and actual target, account for aliases and recorded
installation provenance, and preserve uncertain or foreign entries. Only then
remove identified links and complete owned snapshots no reader or retained link
needs. No automatic migration or cleanup command is introduced.

If both routes supply a skill, native hooks or previously injected guidance may
still run. Directory injection does not suppress them. Document their coexistence
and use isolated native roots when proving Ravel-only delivery. An explicit
native loader or path requested by the user must not be silently replaced.

Every live session keeps its original ending executable and command. Preserve
its compatible executable/worker pair and any legacy dispatcher resources until
its readers and drivers have ended. Installing a new version, moving a source
or cleaning native links cannot change a live launch's ending contract.

## Resources and conformance

Packs contain their required procedures, templates, provenance and licences.
Directory delivery preserves the resource layout. A relative supporting-resource
reference resolves against the skill that owns it. A reference to another pack
names its skill and resource; the agent locates that skill through the active
delivery route and resolves the resource from that skill's directory. No sibling
checkout, native directory or Grove cache layout is assumed.

The [single-owner](../adr/corpus-rules-have-one-owner.md) and
[restatement](../adr/restatement-declares-its-class.md) decisions still govern
normative prose. Required dependencies must be reachable; optional craft
citations state what binds when absent. A move changes ownership and references
together, without duplicating a family procedure or redesigning kinds.

Each pack checks its own rules and resources. Aggregate conformance checks the
composed loaded paths across the supplied core and external packs, required
cross-pack edges and duplicate normative owners. These are content checks over
actual supplied artifacts, not a registry used by either executable to admit a
kind. Ravel delivering readable directories is not proof that prose references
work or that an agent follows them.

Native installer eligibility metadata retains its installer meaning. Directory
injection does not imply harness filtering or installation of native hooks.
Pack descriptions and guidance must make harness-specific requirements clear;
acceptance cannot treat a readable native-only integration as working elsewhere.

## Packaging and rollout

Keep Grove's executable/caller-assets archive boundary. Add no skill payload,
private worker or skill resolver. Source builds, archives and Homebrew document
both owner-managed delivery routes. Manual installers and marketplace packages
remain explicit distribution options. Startup and upgrade invoke neither.

Prepare Ravel's new feature and compatible Grove caller assets in separate
prefixes and isolated homes. Check the existing 1.1 native route as well as the
new directory route before claiming compatibility. Publish Ravel's feature
release before a Grove release that advertises its use. Publication and shared
activation are separate: preserve old installations and settings while their
callers still need them. An alternate policy path alone does not isolate settings
that Ravel reads from the same HOME.

Release fixtures explicitly select their dependency builds, verify matching
independent package identities and test each admitted caller combination. Update
pinned archive identities deliberately; retain separate legacy-driver fixtures.
The source split later updates native packages and injected directory sources
with the same resource-preservation checks. No owner installation is changed by
this design.

## Test seams

Use existing Grove lifecycle, standalone and public session-command seams with
isolated homes and actual installed Ravel. Ravel owns source acquisition tests;
Grove verifies the observable caller behavior.

| Acceptance | Observation |
| --- | --- |
| Native-only bootstrap | With Ravel directory injection disabled, launch through both callers using complete native skills. Grove does not demand source configuration. |
| Directory bootstrap | With native copies absent, inject complete source directories outside cwd and read the spine, named kind and supporting resources. Exercise local and exact Git sources. |
| Offline and failures | Warm the exact source, remove transport access and launch. Missing cold revisions fail clearly. A missing named skill is reported by the agent, not rejected by Grove's kind parser. |
| Original mandate | Compare exact caller bytes and ending commands before/after Ravel preparation. The selected command receives the intended launch prompt. |
| Compatibility | Verify stable 1.x from 1.1.0 and 2.x, respective policy examples, refused versions and no fallback to another PATH entry. |
| No native mutation | Compare owned/foreign files, links, modes, pointers and bytes before/after startup, failure and installed upgrade. Include legacy homes, aliases and an active old reader. |
| Resource lifetime | A reader keeps its original resolved directories while sources/configuration change; subsequent launches can receive the new source view. |
| Confined standalone | Required resources outside cwd are readable, while writes and protected configuration reads fail on actual supported backends. Grove adds no directory-grant bypass. |
| Open tree behavior | Existing custom-kind, handle, review/integration, result and epoch tests pass. No availability check enters tree verbs or launch admission. |
| Conformance | Core, external-pack and aggregate checks use exact supplied artifacts. Removed required resources/edges and duplicate owners fail deliberate controls. |
| Installed distribution | Archives and formula tests verify separate packages, explicit versioned policy examples, both setup routes and no automatic native mutation. |
| Actual harness behavior | Claude Code and Codex each discover an applicable craft skill from a task that does not name it, and read it before the governed action. |

Directory-route harness trials include supporting reads, continuation/compaction,
one delegate, and coexistence with a stale native copy. Record harness/model/
version, isolation, confinement, directory identity and observed ordering.
Separate resource readability, instruction adherence and ending-authority
isolation. A prompt path grants no permission; ordinary delegates may inherit
ending authority. Test result attempts only in disposable sessions. An unavailable
mechanism is an unmeasured gap, never a pass.

## Scope and remaining work

Implementation updates installation guidance, architecture, glossary, source-
exact books, release checks and policy examples alongside the behavior they
ship. The [evidence and handoff](../grove-skill-delivery-design-evidence.md)
identify inspected consumers. Research remains the first external-pack proof
and settles reusable pack tooling before later moves.

Ravel's earlier generated-catalog design is not a settled prerequisite after
the user's directory-injection correction. Reconcile it before dependent
planning and review this adoption against that revision. Internal test wiring,
aggregate conformance tooling and measured harness delegate behavior remain
implementation questions. No kind redesign, stop command, review simplification,
repository relocation or native cleanup occurs here.

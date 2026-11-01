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

The [owner-managed delivery decision](../adr/skill-delivery-is-the-users-choice.md)
records the removal of automatic provisioning and the bootstrap trade-off.
The [caller contract](harness-selection-and-execution.md) owns the public
session interface. Ravel owns source acquisition, directory delivery, prompt
preparation and resource access. Pack authors own their content and references;
the agent owns loading and following that guidance.

Grove's mandate continues to name one required skill. Open kind labels, task
filenames, handles, review and integration routes, tree verbs and result
meanings remain unchanged. Grove adds
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

The choice of route does not grant filesystem access. Lifecycle acceptance
proves the native-only route. Standalone `grove run` always confines its managed
process; HOME in its environment does not make native skill trees readable.
Ravel 2's snapshot grants provide the common route for complete external skill
trees in that caller. A native setup under confinement needs separate proof
that its entry skill, required resources and loader prerequisites are reachable
through existing permitted paths or explicit regular-file grants. Grove adds
no recursive native-directory grant and never retries unconfined. No working
native-only standalone configuration is established by this design.

There is no embedded core-only fallback. Planning remains part of the core
skill pack as the replaceable default, but installing the Grove binary alone
does not install that pack. Setup guidance shows both routes and preserves
existing owner policy, settings and installations. Neither route silently
activates personal guidance or copies a sample policy over an existing one.

If the agent cannot find or read the mandated skill, it reports the missing
guidance, leaves the task live, and runs the exact force-stop command supplied
by its launch prompt as its last action. It does not retire the leaf, record
teardown or send continuation. The mandate must state this rule: the missing
skill cannot supply it. The loop stops without relaunching the leaf. The user
repairs the selected delivery route and reruns. Grove does not turn that failure
into declaration-based kind admission. A configured source that Ravel cannot
acquire is instead a Ravel prelaunch failure, with its source-specific remedy.

For Ravel delivery, accepted source requirements still include local sources
and exact pinned Git sources, offline reuse of warmed exact revisions, clear
failure for unavailable required revisions and readable resources throughout
reader lifetimes. Those are source-directory obligations, not resolution of a
skill requested by Grove. Ravel’s corrected directory-injection design preserves those requirements;
the [design evidence](../grove-skill-delivery-design-evidence.md#upstream-correction)
identifies its review dependency and the human correction that governs it.

## Launch and compatibility

Grove passes the ordinary mandate and exact ending commands through the public
session CLI. Ravel may add directory-discovery instructions; it preserves the
caller task and endings intact. The agent discovers and loads the named skill.
Owner policy must pass the intended launch prompt to the harness. Grove neither
rewrites the selected command nor repairs a policy that drops the prompt.

The mandate must qualify its instruction to load the kind skill "first" with
"after any mandatory guidance". Ravel's loading preamble orders mandatory reads
before caller-block actions, subject to higher-priority harness instructions;
it grants configured content no other priority over the caller. A name with
"load", including a plugin-qualified name, does not select a native tool.
With native copies absent, the agent searches the supplied directories for that
name under Ravel's loading contract. Grove neither parses those directories nor
changes the launch's ending instructions.

The required target remains the bare `grove-<kind>` skill. A native namespace
comes from the plugin that actually supplies it; moving the skill can change
that namespace without changing the kind. Before the first pack moves, the
mandate must stop asserting that every kind belongs to the `grove` plugin.
It names one target and tells the agent to use that skill's installed namespace
where required, without inventing a qualified alias. This adds no kind-to-plugin
map or lookup to Grove. Each moved pack proves its actual native spelling and
the same bare-name request through directory delivery.

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
a manual procedure performed by the user after all affected readers and old
provisioning writers have stopped. Include drivers, lazy link readers, independent harnesses and
detached readers. A lexical path shape or skill name is insufficient deletion
evidence. Inspect each link and actual target, account for aliases and recorded
installation provenance, and preserve uncertain or foreign entries. Only then
remove identified links and complete owned snapshots no reader or retained link
needs. The increment that removes provisioning supplies the procedure in the
installation guidance in `docs/USAGE.md`, with a link from plugin installation
guidance. It explains how to inspect the legacy pointer and snapshot layout
without treating that layout alone as ownership evidence. No automatic
migration or owner-invoked cleanup command is introduced.

If both routes supply a skill, native hooks or previously injected guidance may
still run. Directory injection does not suppress them. Document their coexistence
and use isolated native roots when proving Ravel-only delivery. An explicit
native loader or path requested by the user must not be silently replaced.
Ravel's contract distinguishes a concrete loader from a name request: an absent
native match permits a disclosed directory fallback for the same guidance,
unless the caller expressly restricts the load to that mechanism. A concrete
path has no substitute, and other loader failures are reported before dependent
work. These rules are exercised by the agent, not a Grove lookup protocol.

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
release before a Grove release that advertises its use. A Grove release that
adds verified dual-major compatibility may precede that feature publication.

Every published tap state must pair its Grove formula with a compatible Ravel
dependency, including fresh installs and upgrades between releases. Before the
unversioned `ravel` formula advances to major 2, publish Grove's dual-major
admission, compatible caller assets and formula tests. If Ravel 2 must publish
first, expose it under a separate versioned formula and keep Grove's existing
dependency on compatible 1.x until that Grove release is available. Do not
advance the unversioned dependency first and rely on users to time or pin an
upgrade. Resolver remedies name compatible executable and policy versions.

Publication and shared activation remain separate: preserve old installations
and settings while their callers still need them. An alternate policy path
alone does not isolate settings that Ravel reads from the same HOME. Tap
publication and any corresponding Ravel release-document changes belong to
their owners; the rollout must verify their order before publishing.

Release fixtures explicitly select their dependency builds, verify matching
independent package identities and test each admitted caller combination. Update
pinned archive identities deliberately; retain separate legacy-driver fixtures.
The source split later updates native packages and injected directory sources
with the same resource-preservation checks. No owner installation is changed by
this design.

The [coordinated release contract](coordinated-release.md) supplies the concrete
combined-release identities, tap sequence, owner handoff and finish extension.
Its focused review precedes the Rhai contract repair and dependent implementation.

## Test seams

Use existing Grove lifecycle, standalone and public session-command seams with
isolated homes and actual installed Ravel. Ravel owns source acquisition tests;
Grove verifies the observable caller behavior.

| Acceptance | Observation |
| --- | --- |
| Native-only bootstrap | With Ravel directory injection disabled, launch the lifecycle caller using complete native skills. Grove does not demand source configuration. Any claimed native-only standalone setup separately proves discovery and every required read under actual confinement; installing under HOME alone is insufficient. |
| Directory bootstrap | With native copies absent, inject complete source directories outside cwd and read the spine, named kind and supporting resources. Exercise local and exact Git sources, Grove's bare-name mandate and an explicit plugin-qualified name request; neither name request requires a native loader. |
| Offline and failures | Warm the exact source, remove transport access and launch. Missing cold revisions fail clearly. For a missing or unreadable mandated skill, the agent reports and uses the prompt's exact force-stop command. The leaf remains live, no teardown is recorded and the loop does not relaunch. Grove's kind parser does not reject it. |
| Original mandate | The mandate qualifies "first" with "after any mandatory guidance". Observe mandatory reads before the named-kind read. Compare exact caller bytes and ending commands before/after Ravel preparation; the selected command receives the intended launch prompt. |
| Explicit loading mechanism | Exercise a native loader with a match, absent-match directory fallback with disclosure, an express native-only restriction and a concrete path. Observe the specified route or a report before dependent work; do not silently substitute. |
| Compatibility | Verify stable 1.x from 1.1.0 and 2.x, respective policy examples, refused versions and no fallback to another PATH entry. |
| No native mutation | Compare owned/foreign files, links, modes, pointers and bytes before/after startup, failure and installed upgrade. Include legacy homes, aliases and an active old reader. |
| Manual cleanup guidance | Exercise the documented procedure in disposable legacy layouts with stopped readers/writers, owned and foreign links, aliases, uncertain provenance and retained snapshot references. Only evidenced, unneeded owned entries are removable; the owner's installation is untouched by the trial. |
| Resource lifetime | A reader keeps its original resolved directories while sources/configuration change; subsequent launches can receive the new source view. |
| Confined standalone | Ravel 2 directory delivery makes required resources outside cwd readable, while writes and protected configuration reads fail on actual supported backends. With injection disabled and native trees outside all grants, observe a missing/unreadable-guidance report and force-stop with no output publication. Grove adds no directory-grant bypass. |
| Open tree behavior | Existing custom-kind, handle, review/integration, result and epoch tests pass. No availability check enters tree verbs or launch admission. |
| Conformance | Core, external-pack and aggregate checks use exact supplied artifacts. Removed required resources/edges and duplicate owners fail deliberate controls. |
| Installed distribution | Archives and formula tests verify separate packages, explicit versioned policy examples, both setup routes and no automatic native mutation. Exercise fresh install and upgrade at each intermediate tap state: Grove's dependency always resolves to an admitted Ravel major, and feature advertising follows its publication. |
| Native names after a pack move | From the first external pack onward, native Claude Code loads a moved kind under the plugin's actual namespace from Grove's bare-name mandate. Repeat directory loading with native copies absent. The mandate asserts no fixed plugin namespace and adds no kind registry. |
| Actual harness behavior | Claude Code and Codex each discover an applicable craft skill from a task that does not name it, and read it before the governed action. |

Directory-route harness trials include supporting reads, continuation/compaction,
one delegate, and coexistence with a stale native copy. Record harness/model/
version, isolation, confinement, directory identity and observed ordering.
Compaction trials separately remove the prompt path and the entire preamble.
For the latter, give no new recovery reminder and observe descriptor recovery
and mandatory rereads before actions. A retained variable alone proves no such
behavior; a failed unprompted recovery remains an acceptance gap.
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

Ravel's directory review and integration supply the corrected contract to
adoption review and dependent planning. The earlier generated-catalog
proposal is historical input. Internal test wiring,
aggregate conformance tooling and measured harness delegate behavior remain
implementation questions. No kind redesign, stop command, review simplification,
repository relocation or native cleanup occurs here.

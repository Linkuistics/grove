# Grove skill delivery

This is the adoption design. Grove implements its native-only route: Grove no
longer embeds or provisions Codex snapshots, and startup leaves earlier
installations unchanged. Grove 22.8 and earlier still provision them. Grove
also implements the route-independent mandate: a bare skill name, "first" after
any mandatory guidance, and the missing-skill force-stop rule.

Grove admits stable Ravel 1.x from 1.1.0 and stable 2.x from 2.0.0. Grove
ships TypeScript caller assets for 1.x and Rhai caller assets for 2.x. Grove
supports the native routes on 1.1 and on 2.0. Grove supports the directory
route on 2.0 through the lifecycle caller and the confined standalone caller.
Each passed its acceptance on macOS.

The reviewed upstream contract is Ravel commit `822c0ed2`, as the
[contract mapping](../grove-skill-delivery-design-evidence.md#rhai-mapping)
records. The document location remains stable for existing citations.

## Ownership

Grove names the skill that its session must load. **The user ensures that skill
is available**, either through each harness's native installation or through
Ravel's mechanism. Grove and Ravel are decoupled with respect to skills. Grove
neither installs skills nor asks Ravel to resolve a named skill.

When configured, Ravel injects skill-directory paths into the launch prompt.
The agent uses those locations to discover and read applicable skills. Ravel
is not a requested-skill lookup service. Directory delivery does not establish
that a named skill exists, applies, or has been read.

The [owner-managed delivery decision](../adr/skill-delivery-is-the-users-choice.md)
records the removal of automatic provisioning and the bootstrap trade-off.
The [caller contract](harness-selection-and-execution.md) owns the public
session interface. Ravel owns source resolution, directory delivery, prompt
preparation and resource access. Pack authors own their content and references.
The agent owns loading and following that guidance.

Grove's mandate continues to name one required skill. Open kind labels, task
filenames, handles, review and integration routes, tree verbs and result
meanings remain unchanged. Grove adds no catalogue reader, kind registry,
presence check or discovery command. No skill location crosses from Ravel back
into Grove's task loop.

## Installation and bootstrap

A fresh installation supplies Grove's executables and readable caller policy
assets. The user also chooses a delivery route for the skills their work needs:

| Route | Setup obligation |
| --- | --- |
| Native harness installation | Install the spine, required kind skills and resources through that harness's supported plugin or manual installation route. Repeat for each harness that policy may select. |
| Ravel directory injection | Configure the desired source directories through Ravel 2.0.0. Check that the launched agent can discover and read the required skills and supporting resources there. Native copies are unnecessary. |

These routes are alternatives. Grove does not require Ravel discovery to be
enabled or a source-resolution command to have been run. No-source Ravel
launches remain valid. A native-only installation is a supported setup, not a
degraded fallback. Independently launched harnesses can continue using their
native installations.

The choice of route does not grant filesystem access. Lifecycle acceptance
proves the native-only route. Standalone `grove run` always confines its managed
process. HOME in its environment does not make native skill trees readable.
The snapshot read grants of Ravel 2.0.0 provide the common route for complete
external skill trees in that caller.

A native setup under confinement needs separate proof. This proof must show
that its entry skill, required resources and loader prerequisites are reachable
through existing permitted paths or explicit regular-file grants. Grove adds no
recursive native-directory grant and never retries unconfined. This design
establishes no working native-only standalone configuration.

There is no embedded core-only fallback. Planning remains part of the core
skill pack as the replaceable default, but installing the Grove binary alone
does not install that pack. Setup guidance shows both routes and preserves
existing owner policy, settings and installations. Neither route silently
activates personal guidance or copies a sample policy over an existing one.

If the agent cannot find or read the mandated skill, it reports the missing
guidance and leaves the task live. As its last action, the agent runs the exact
force-stop command that its launch prompt supplies. It does not retire the leaf,
record teardown or send continuation. The mandate must state this rule, because
the missing skill cannot supply it.

The loop stops without relaunching the leaf. The user repairs the selected
delivery route and reruns. Grove does not turn that failure into
declaration-based kind admission. A configured source that Ravel cannot capture
is instead a Ravel prelaunch failure, with its source-specific remedy.

For Ravel delivery, the accepted source requirements are local source
directories, clear failure for a missing required source and readable
resources throughout reader lifetimes. Those are source-directory obligations,
not resolution of a skill that Grove requests. Ravel's reviewed directory
contract at `822c0ed2` set them, together with exact Git sources that Ravel
later withdrew. The
[design evidence](../grove-skill-delivery-design-evidence.md#upstream-correction)
identifies that contract, its review and the human correction that governs it.

Ravel reads local directories only. It does not fetch, install or update a
source, so delivery needs no network. A tool outside Ravel puts each pack
checkout in place. Ravel's decision record "Skill sources are local
directories" removed the earlier exact Git sources and their offline reuse.

Grove requires no particular checkout tool. A dedicated checkout at a recorded
revision suits delivery better than a development working copy. Ravel captures
uncommitted changes, and it retains each distinct capture. The
[owner delivery design](owner-skill-delivery.md#pack-checkouts) records the
tool and the pin policy of one owner.

## Supported routes

Grove supports three Ravel routes. The owner selects one for each Grove
invocation through the PATH, the configuration file and the settings. Grove
reads none of these. Its version probe decides admission only, and it selects
no route. The [route view](../design/harness-selection-and-execution/README.md)
shows the same decision.

| Route | Ravel | Configuration | Settings | Caller assets | Skills |
| --- | --- | --- | --- | --- | --- |
| Native on 1.1 | Stable 1.x from 1.1.0. Homebrew's unversioned `ravel` until promotion | `~/.config/ravel/policy.ts` with request and policy schema 2 and `ravel/sdk` | Only `timeoutMs`, `contextBytes` and `policyEnv` | Grove's TypeScript assets | Native harness installation |
| Native on 2.0 | Stable 2.x from 2.0.0. The keg-only `ravel@2` until promotion | `~/.config/ravel/config.rhai` with `SCHEMA_VERSION` 4 | The keys of 2.0.0 without `skills`, or with empty `sources` and `mandatory` | Grove's Rhai assets | Native harness installation |
| Directory on 2.0 | As native on 2.0 | As native on 2.0 | The keys of 2.0.0 with a `skills` object that names sources | Grove's Rhai assets, which grant harness reads from `request.skills` | Configured Ravel sources. Native copies are unnecessary |

### Actual minimum and separate proofs

Directory delivery needs Ravel 2.0.0, the first release that holds it. No 1.x
release supports it. Rhai native compatibility and directory delivery are two
separate proofs over the same 2.0.0 executable. The first proof runs with
delivery disabled and complete native skills. The second runs with delivery
enabled and native copies absent. Neither proof stands in for the other.

The [coordinated release](coordinated-release.md) ships both in one 2.0.0. An
earlier native-only 2.0.0 release would need its own explicit owner and
authorisation.

### Settings and HOME

The two configuration files coexist under one HOME. A 1.1 launch reads
`policy.ts`, and a 2.0 launch reads `config.rhai`. The settings file does not
coexist. Ravel 1.1.0 refuses `skills`, `configEnv`, `harnesses` and `budgets`
as `settings_invalid`, stage `cli`, exit 2. Ravel 2.0.0 refuses `policyEnv` in
the same way. A settings file that both releases accept holds only `timeoutMs`
and `contextBytes`.

Neither separate prefixes nor `--config` isolate that file, because both
releases read it from HOME. While a 1.1 `start-session` reader remains under a
HOME, a 2.0 route that needs a 2.0-only key runs under a separate HOME.
`end-session` of either release reads no settings, so a settings change cannot
break a live ending command.

Grove sets no HOME, and it passes its own to Ravel. A separate HOME therefore
also moves Grove's standalone run logs and the HOME-based configuration of the
harnesses and the VCS. The [owner-delivery design](owner-skill-delivery.md)
owns what the HOME of the owner links.

### Prefix selection

Until promotion, Homebrew installs 2.0.0 as the keg-only `ravel@2`. Its
`bin/ravel` is never the default PATH command. The owner selects it by putting
that `bin` directory before every other `ravel` on the PATH of the Grove
invocation. Grove then canonicalises that executable and quotes the same path
in the ending commands of the prompt.

Grove's formula keeps its unversioned dependency. Until promotion, a default
installation therefore stays on the native route on 1.1. The
[coordinated release](coordinated-release.md#compatible-tap-states) owns the
tap states.

### Caller assets

Grove ships readable caller assets for each admitted major. The TypeScript
assets keep their installed paths and their `ravel/sdk` imports for 1.x,
because existing owner policies import them by path. Ravel 2.0.0 has no SDK, so
the routes on 2.0 need Rhai equivalents. Grove adds Rhai equivalents of the
sample configuration, the static and review examples, the review adapter and
the provider-evidence wrapper. Grove issue 24 asks for this port, and the
dual-Ravel stage delivers it.

Each asset names its runtime and schema. A Rhai asset imports another by path,
because Ravel 2.0.0 embeds no module. Grove therefore ships its own copy of
Ravel's static example as `examples/static.rhai`. Each Rhai asset keeps the
refusals and the receipt format of its TypeScript counterpart, except for the
differences that [runtime parity](#runtime-parity) accepts. The copy keeps the
remedy words of the 1.x static example for that reason.

A route of `grove-static.rhai` is a closure. Rhai finds the function of a
closure only while a function of the module that made it runs. A script that
composes these routes therefore selects through `select_route` of
`grove-static.rhai`, not through the starter's own function. The review
example `grove-review.rhai` runs its selector inside `within` of
`grove-static.rhai` for the same reason.

`review.rhai` calls each route through
the starter's `select_route`, so routes of the starter, of the review example
and of the owner's script resolve there. A script that passes routes of
another module to `select_with` of `review.rhai` calls it from a function of
that module. Otherwise the evaluation ends with an error, and nothing
launches.

#### Runtime parity

Ravel's frozen comparison at `822c0ed2` keeps three intentional differences
between the runtimes. Grove's sample meets them through its choice file and its
jj store path. Grove accepts them for that local input of the owner. It also
accepts the same read-bound rule for the jj store path:

- The Rhai sample reads the choice file with the 4,096-byte bound of the 1.1
  helper. A larger file ends the evaluation as Ravel's `source_too_large`, exit
  3, instead of `choice_unreadable`. A catch cannot recover a breach of a read bound.
- The Rhai sample also reads the jj store path through a measured read. A file
  beyond its read bound ends the evaluation in the same way. The 1.1 sample read
  that file without a bound.
- The names in the choice file and the jj store path use the splitting and
  trimming of Rhai. Rhai and JavaScript classify U+FEFF and U+0085 differently.
- A jj store path that is not UTF-8 refuses as `jj_store_unreadable`. The 1.1
  sample decoded it with replacement characters.

The provider receipt crosses runtimes, so Grove keeps its rules deliberately. A
review adapter on one route can check a receipt that a wrapper on the other
route recorded. These obligations are separate:

- **Receipt bytes.** The Rhai wrapper writes the bytes of the TypeScript wrapper
  for each receipt. It keeps the constant member order, and it joins members
  that `json_encode` writes.
- **Provider safety.** The receipt, label and command checks of the wrapper, the
  review loader and the review example give the same outcome in both runtimes.
  A label is blank when ECMAScript's `String.prototype.trim` empties it. The
  Rhai helpers test that whitespace set explicitly, because the trimming of Rhai
  disagrees for U+FEFF and U+0085.

One difference remains in the receipt. A JavaScript string can hold an unpaired
surrogate, and a Rhai string cannot. The TypeScript review loader accepts a
receipt that escapes one, and the Rhai review loader refuses it as
`creator_line_malformed`. Ravel 1.x then refuses the context that carries the
accepted receipt as `context_invalid`, because JSON cannot carry the string. So
neither runtime launches a reviewer.

The TypeScript wrapper records such a receipt only from an owner label that
holds an unpaired surrogate. The remedy is the declaration of the provider by
the owner.

Ravel's own checks of a selection follow the wrapper. Ravel of both majors
counts U+0085 as whitespace when it checks an owner version and a provider. No
wrapper therefore records a version or a provider of U+0085 alone. Rhai keeps
the keys of a map in code point order. The review examples of both runtimes
therefore list the origins of a refusal in that order.

The Rhai sample passes `request.prompt` to each harness. Where the sandbox of a
harness needs explicit directories, the sample adds one read grant for each
`snapshotRoot` in `request.skills`. With delivery disabled, `request.skills` is
`()`, and the sample adds no grant.

Grove's assets read the task handle from `request.taskId`, the Reviews and
Creator lines from `request.taskFile` and caller facts from `request.context`.
Schema 4 leaves those fields unchanged. No Grove asset analyses prompt text. An
owner script that analyses the task reads `request.originalPrompt`, which holds
the mandate byte for byte. The provider receipt reaches the managed process
through `GROVE_PROVIDER_EVIDENCE`, never through a prompt.

One case table drives the checks of both asset sets, so the two sets cannot
drift apart. Each case states the expected outcome for each runtime. A case
that proves an accepted difference states both outcomes and keeps its reason
beside it. The table compares routing choices, fake-harness argument vectors,
delivered context values, receipt bytes and refusal codes, stages, exit statuses
and remedies. Messages from a decoder or the system can differ. The table
excludes fields that only Rhai reports, such as measured reads.

The TypeScript checks keep their strict typecheck against an
explicitly selected installed 1.1. Ravel 2.0.0 has no typechecker and no
inspection verb. The Rhai checks therefore launch the assets through public
`start-session` of an explicitly selected 2.0 build, with fake agents.

Neither Grove nor Ravel installs or converts the configuration of an owner.
Setup guidance and formula caveats show both formats.

## Launch and compatibility

Grove passes the ordinary mandate and exact ending commands through the public
session CLI. Ravel may add directory-discovery instructions. Ravel preserves the
caller task and endings intact. The agent discovers and loads the named skill.

The configuration of the owner must pass the intended launch prompt to the
harness. On 2.x that prompt is `request.prompt`. Grove neither rewrites the
selected command nor repairs a configuration that drops the prompt.

With delivery enabled, Ravel 2.0.0 writes its preamble, a label and one
delimited block. That block holds the bytes of the mandate once, unchanged.
`request.originalPrompt` holds the same bytes, and `request.prompt` holds the
prepared text. With delivery disabled, the two fields are identical. Ravel
recognises no ending command, so each command in the mandate stays exact.

The mandate must qualify its instruction to load the kind skill "first" with
"after any mandatory guidance". Ravel's loading preamble orders mandatory reads
before caller-block actions, subject to higher-priority harness instructions.
The preamble grants configured content no other priority over the caller. A name
with "load", including a plugin-qualified name, does not select a native tool.
With native copies absent, the agent searches the supplied directories for that
name under Ravel's loading contract. Grove neither parses those directories nor
changes the launch's ending instructions.

The required target remains the bare `grove-<kind>` skill. A native namespace
comes from the plugin that actually supplies it. Moving the skill can change
that namespace without changing the kind.

Before the first pack moves, the mandate must stop asserting that every kind
belongs to the `grove` plugin. The mandate names one target and tells the agent
to use that skill's installed namespace where required, without inventing a
qualified alias. This adds no kind-to-plugin map or lookup to Grove. Each moved
pack proves its actual native spelling and the same bare-name request through
directory delivery.

Both lifecycle and standalone callers use that same session boundary. A
standalone runtime-file argument retains its existing meaning. Grove does not
translate skill directories into that argument or expand access itself.

Compatibility is about the public session interface and the caller assets,
not skill delivery. Retain stable Ravel 1.x support from 1.1.0. Admit stable
2.x from 2.0.0 once the native route on 2.0 passes the same caller acceptance
with the Rhai assets. Advertise directory delivery only after the directory
route passes its own acceptance. Refuse 1.0, prereleases, unknown majors such
as 3.x, malformed version output and failed probes.

Preserve absolute-PATH selection, canonicalisation and refusal without falling
through. The remedy names both admitted majors with their configuration files,
and the `ravel@2` prefix until promotion. It recommends no undirected shared
upgrade.

Native-only users need no Ravel 2.0.0 for skills. Policy examples and installed
checks identify the runtime and schema that they target. Provide explicit
compatible examples for each admitted major rather than silently rewriting the
configuration of an owner. Grove parses no configuration script, request or
settings schema, routing notice, failure document or private worker protocol.
Context and provider receipts retain their independent contracts.

Grove needs neither `RAVEL_SKILL_DIRECTORIES` nor `ravel skills resolve`. That
variable grants no authority, and Ravel replaces or removes it in each
environment that it builds. Grove's environment scrub therefore gains no name.

| Failure | Remedy |
| --- | --- |
| Missing or incompatible Ravel | Select an admitted installation on the PATH of the invocation: 1.x from 1.1.0 with `policy.ts`, or 2.x from 2.0.0 with `config.rhai`. Avoid an undirected shared upgrade. |
| Configuration does not match the selected Ravel | Preserve Ravel's refusal: `config_format_retired` for a lone `policy.ts` on 2.0.0, `unsupported_version` for schema 3, or `settings_invalid` for a key of the other major. Select the matching installation, convert the configuration deliberately or use a separate HOME. Grove rewrites no configuration. |
| Missing named skill | Repair native installation or the configured directory set. Do not require switching routes. |
| Unavailable configured source | Preserve Ravel's source-specific diagnostic. Restore the source or deliberately amend owner configuration. |
| Read access, capture, cache, timeout or limit failure | Preserve Ravel's precise remedy. Do not silently substitute another source, skip guidance or retry unconfined. |
| Unsupported delegate access | Report that adapter's limitation before dependent work. Do not imply that prompt text grants filesystem access. |

Prelaunch failures yield no successful session result. Grove keeps its normal
interruption and result interpretation and leaves selected work live. Lifecycle
scaffolding performed before launch is not promised a rollback.

## Existing installations and active readers

Remove automatic native provisioning and its embedded broad snapshot from
Grove's build and startup. Startup and package upgrade leave old native links,
current-snapshot pointers, snapshots, and foreign entries untouched. They do
not adopt, repair, repoint or remove them. Existing native installations remain
usable where their contents and resource paths are still valid.

The user can keep native installations indefinitely. Cleanup, if requested, is
a manual procedure performed by the user after all affected readers and old
provisioning writers stop. Include drivers, lazy link readers, independent
harnesses and detached readers.

A lexical path shape or skill name is insufficient evidence for removal.
Inspect each link and its actual target. Account for aliases and recorded
installation provenance. Preserve uncertain or foreign entries. Only then
remove identified links in the skill directories that no reader, retained link
or retained text needs. A retained link needs every entry that its resolution
passes, including entries reached through intermediate links outside the
searched locations.

A search shows only what it reaches in the searched locations, and it cannot
prove that nothing needs an entry. The cleanup therefore keeps the legacy root,
with its current-snapshot pointer, every snapshot and its lock file. The root is
small, and the duplicate copies that cleanup addresses come from the links.

The [cleanup procedure](../USAGE.md#usage-codex-cleanup) in the
installation guidance supplies these steps, and plugin installation guidance
links to it. The procedure explains how to inspect the legacy pointer and
snapshot layout without treating that layout alone as ownership evidence. This
design introduces no automatic migration or owner-invoked cleanup command.

If both routes supply a skill, native hooks or previously injected guidance may
still run. Directory injection does not suppress them. Document their coexistence
and use isolated native roots when proving Ravel-only delivery.

An explicit native loader or path requested by the user must not be silently
replaced. Ravel's contract distinguishes a concrete loader from a name request.
Under that contract, an absent native match permits a disclosed directory
fallback for the same guidance. This fallback is not permitted when the caller
expressly restricts the load to that mechanism. A concrete path has no
substitute, and other loader failures are reported before dependent work. The
agent, not a Grove lookup protocol, exercises these rules.

Every live session keeps its original ending executable and command. Preserve
its compatible executable/worker pair and any legacy dispatcher resources until
its readers and drivers end. Installing a new version, moving a source or
cleaning native links cannot change a live launch's ending contract.

## Resources and conformance

Packs contain their required procedures, templates, provenance and licences.
Directory delivery preserves the resource layout. A relative supporting-resource
reference resolves against the skill that owns it. A reference to another pack
names its skill and resource. The agent locates that skill through the active
delivery route and resolves the resource from that skill's directory. No sibling
checkout, native directory or Grove cache layout is assumed.

The [single-owner](../adr/corpus-rules-have-one-owner.md) and
[restatement](../adr/restatement-declares-its-class.md) decisions still govern
normative prose. Required dependencies must be reachable. Optional craft
citations state what binds when absent. A move changes ownership and references
together, without duplicating a family procedure or redesigning kinds.

Each pack checks its own rules and resources. Aggregate conformance checks the
composed loaded paths across the supplied core and external packs, required
cross-pack edges and duplicate normative owners. These are content checks over
actual supplied artefacts, not a registry used by either executable to admit a
kind. Ravel delivering readable directories is not proof that prose references
work or that an agent follows them.

Native installer eligibility metadata retains its installer meaning. Directory
injection does not imply harness filtering or installation of native hooks.
Pack descriptions and guidance must make harness-specific requirements clear.
Acceptance cannot treat a readable native-only integration as working elsewhere.

## Packaging and rollout

Keep Grove's executable/caller-assets archive boundary. Add no skill payload,
private worker or skill resolver. Source builds, archives and Homebrew document
both owner-managed delivery routes. Manual installers and marketplace packages
remain explicit distribution options. Startup and upgrade invoke neither.

Prepare Ravel 2.0.0 and compatible Grove caller assets in separate prefixes
and isolated homes. Check the three supported routes separately before claiming
compatibility. The [coordinated release](coordinated-release.md) settles the
order. Ravel 2.0.0 publishes first as the keg-only `ravel@2`. Compatible Grove
publishes while unversioned `ravel` stays 1.1.0. Finish then promotes the same
archives to unversioned `ravel`.

Every published tap state must pair its Grove formula with a compatible Ravel
dependency, including fresh installs and upgrades between releases. Grove's
formula keeps its unversioned dependency, which stays 1.1.0 until promotion. Do
not advance the unversioned dependency before compatible Grove is public and
checked. Do not rely on users to time or pin an upgrade. Resolver remedies name
compatible executable and configuration versions.

Publication and shared activation remain separate. Preserve old installations
and settings while their callers still need them. An alternate policy path
alone does not isolate settings that Ravel reads from the same HOME. Tap
publication and any corresponding Ravel release-document changes belong to
their owners. The rollout must check their order before publishing.

Release fixtures explicitly select their dependency builds, check matching
independent package identities and test each admitted caller combination.
Update pinned archive identities deliberately. Retain separate legacy-driver
fixtures. The source split later updates native packages and injected directory
sources with the same resource-preservation checks. This design changes no
owner installation.

Before the cut, a build of Ravel's feature line reports `ravel 1.1.0`, and Grove
admits that output as 1.x. Acceptance therefore identifies each Ravel build by
commit and build identity, not by its version output. Stub executables prove
the admission of `ravel 2.0.0` output before the cut. After the cut, rollout
repeats the admission and installed checks against the published archives,
pinned by SHA-256.

The [coordinated release contract](coordinated-release.md) supplies the concrete
combined-release identities, tap sequence, owner handoff and finish extension.
Its focused review is integrated.

## Test seams

Use existing Grove lifecycle, standalone and public session-command seams with
isolated homes and actual installed Ravel. Each case names its route and
identifies its Ravel build by commit and build identity. Ravel owns source
resolution tests. Grove checks the observable caller behaviour.

`crates/grove/tests/directory_route.rs` proves the lifecycle part of five rows
below: Directory bootstrap, Source and skill failures, Original mandate,
Settings isolation and Resource lifetime. `task ravel:directory` runs it, with
`RAVEL` as the 2.0 front and `OLD` as the 1.1 front. Its fake agents read the
paths that the prepared prompt names, and they prove delivery rather than LLM
behaviour. The explicit plugin-qualified name request, the missing-skill report
of a real agent and actual harness behaviour belong to acceptance.

The same file proves the directory part of the Confined standalone row. A
confined `grove run` on 2.0 reads the spine, a resource and the named kind from
the snapshots. It cannot write a snapshot, a source or a path outside its
working directory. It cannot read a source directly or the protected
configuration. The same probes, run unconfined, succeed for each source,
configuration and outside path, so those denials belong to the sandbox.

`crates/grove/tests/standalone.rs` proves the sandbox part of the native case
on each runtime. With injection disabled, native copies under HOME stay
unreadable to the confined agent, and Grove adds no grant.

The fixture also proves Grove's interpretation of a force-stop. After its scripted agent
force-stops, Grove publishes nothing and launches the agent once. A
regular-file grant of each copy makes the same check fail. The fixture is not
evidence of agent behaviour, because its script decides to stop.
`task ravel:directory` runs both parts on the 2.0 build.

The Linux floor cases run the native part on each runtime under bubblewrap.
They run the directory part on the 2.0 lane in the supported environment. The
floor userland has no Git, so the two sources of that case are plain local
directories. [Runtime lanes](../linux-floor-native.md#runtime-lanes) states each case.
The [dual-Ravel stage record](../grove-dual-ravel-stage-record.md) names the
builds and the results.

| Acceptance | Observation |
| --- | --- |
| Native-only bootstrap on 1.1 | With Ravel 1.1.0 and `policy.ts`, launch the lifecycle caller using complete native skills. Grove does not demand source configuration. Any claimed native-only standalone setup separately proves discovery and every required read under actual confinement. Installing under HOME alone is insufficient. |
| Native route on 2.0 | With Ravel 2.0.0, `config.rhai` at schema 4 and delivery disabled, launch the lifecycle and standalone callers using complete native skills. `request.prompt` equals `request.originalPrompt` byte for byte, and both equal the mandate. The Rhai assets give the 2.0 outcome of each case in the shared table. They differ from the TypeScript outcomes only where [runtime parity](#runtime-parity) accepts it, and their receipts match byte for byte. This row proves Rhai native compatibility, not directory delivery. |
| Directory bootstrap | Use Ravel 2.0.0 with the `skills` key, in a HOME separate from any 1.1 reader. With native copies absent, inject complete source directories outside cwd and read the spine, named kind and supporting resources. Exercise local sources, Grove's bare-name mandate and an explicit plugin-qualified name request. Neither name request requires a native loader. |
| Source and skill failures | A missing configured source directory refuses before launch, with Ravel's remedy. For a missing or unreadable mandated skill, the agent reports and uses the prompt's exact force-stop command. The leaf remains live, no teardown is recorded and the loop does not relaunch. Grove's kind parser does not reject it. |
| Original mandate | The mandate qualifies "first" with "after any mandatory guidance". Observe mandatory reads before the named-kind read. Compare exact caller bytes and ending commands before/after Ravel preparation. `originalPrompt` and the caller block of the prepared prompt each equal the composed mandate byte for byte. The selected command receives the intended launch prompt. |
| Explicit loading mechanism | Exercise a native loader with a match, absent-match directory fallback with disclosure, an express native-only restriction and a concrete path. Observe the specified route or a report before dependent work. Do not silently substitute. |
| Compatibility | Check the admission of stable 1.x from 1.1.0 with `policy.ts` and the TypeScript assets. Check the admission of 2.x from 2.0.0 with `config.rhai` and the Rhai assets. Refuse 1.0, prereleases, 3.x and malformed output, with no fallback to another PATH entry. Before the cut, stub executables prove the admission of `ravel 2.0.0` output. |
| Settings isolation | Ravel 1.1.0 refuses shared settings that hold a 2.0-only key, and a separate prefix cannot prevent it. A separate HOME runs a native lifecycle launch on 1.1 and a directory launch on 2.0 at the same time. A mismatched configuration reaches the user as Ravel's refusal and leaves the leaf live. Existing ending commands stay usable. |
| Prefix selection | With unversioned 1.1.0 and the keg-only `ravel@2` installed, the default PATH selects 1.1.0. A PATH that puts the `bin` directory of `ravel@2` first selects 2.0.0, and the ending commands quote that canonical path. |
| No native mutation | Compare owned/foreign files, links, modes, pointers and bytes before/after startup, failure and installed upgrade. Include legacy homes, aliases and an active old reader. |
| Manual cleanup guidance | Exercise the documented procedure in disposable legacy layouts. These layouts include stopped readers/writers, owned and foreign links, aliases and uncertain provenance. They also include retained links that reach a skill entry through intermediate links or `..`. Only evidenced, unneeded owned skill-directory links are removable. The legacy root stays unchanged, and the trial leaves the owner's installation untouched. |
| Resource lifetime | A reader keeps its original resolved directories while sources/configuration change. Subsequent launches can receive the new source view. |
| Confined standalone | Ravel 2.0.0 directory delivery makes required resources outside cwd readable, while writes and protected configuration reads fail on actual supported backends. With injection disabled, native trees stay unreadable outside all grants, and Grove adds no directory-grant bypass. A force-stop publishes no output. The agent part is an explicit gap by the human's decision. A standalone prompt names no guidance, so Grove does not require a report or a force-stop. The evidence is `missing-kind-confined-20261103T062817Z`. |
| Open tree behaviour | Existing custom-kind, handle, review/integration, result and epoch tests pass. No availability check enters tree verbs or launch admission. |
| Conformance | Core, external-pack and aggregate checks use exact supplied artefacts. Removed required resources/edges and duplicate owners fail deliberate controls. |
| Installed distribution | Archives and formula tests check separate packages, both caller-asset sets with their runtimes, both configuration formats, both setup routes and no automatic native mutation. Exercise fresh install and upgrade at each tap state A to D of the coordinated release. At each state, Grove's dependency always resolves to an admitted Ravel major, and feature advertising follows its publication. |
| Native names after a pack move | From the first external pack onward, native Claude Code loads a moved kind under the plugin's actual namespace from Grove's bare-name mandate. Repeat directory loading with native copies absent. The mandate asserts no predetermined plugin namespace and adds no kind registry. |
| Actual harness behaviour | Claude Code and Codex each discover an applicable craft skill from a task that does not name it, and read it before the governed action. |

Directory-route harness trials include supporting reads, continuation/compaction,
one delegate, and coexistence with a stale native copy. Record
harness/model/version, isolation, confinement, directory identity and observed
ordering. Compaction trials separately remove the prompt path and the entire
preamble. For the latter, give no new recovery reminder and observe descriptor
recovery and mandatory rereads before actions. A retained variable alone proves
no such behaviour. A failed unprompted recovery remains an acceptance gap.

Separate resource readability, instruction adherence and ending-authority
isolation. A prompt path grants no permission. Ordinary delegates may inherit
ending authority. Test result attempts only in disposable sessions. An
unavailable mechanism is an unmeasured gap, never a pass. The
[common loading acceptance record](../common-loading-acceptance-record.md)
holds the trials and describes their runner.

## Scope and remaining work

Implementation updates installation guidance, architecture, glossary,
source-exact books, release checks and both caller-asset sets alongside the
behaviour they ship. The [evidence and handoff](../grove-skill-delivery-design-evidence.md)
identify inspected consumers. Research remains the first external-pack proof
and settles reusable pack tooling before later moves.

Ravel's reviewed contract at `822c0ed2`, this adoption and its integrated
focused review supply dependent planning. The TypeScript contract at `b93bb927` supplies
intent only, and the generated-catalogue proposal is historical input. Internal test wiring,
aggregate conformance tooling and measured harness delegate behaviour remain
implementation questions. No kind redesign, stop command, review simplification,
repository relocation or native cleanup occurs here.

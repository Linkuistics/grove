# Grove skill delivery

This is the adoption design. Grove implements its native-only route: Grove no
longer embeds or provisions Codex snapshots, and startup leaves earlier
installations unchanged. Grove 22.8 and earlier still provision them. Grove
also implements the route-independent mandate: a bare skill name, "first" after
any mandatory guidance, and the missing-skill force-stop rule.

Grove admits stable Ravel 1.x from 1.1.0 and stable 2.x from 2.0.0. Grove
ships TypeScript caller assets for 1.x and Rhai caller assets for 2.x. Grove
supports the native routes on 1.1 and on 2.0. Grove supports the directory
route on 2.0 through the lifecycle caller and the standalone caller.
Each passed its acceptance on macOS.

The reviewed upstream contract is Ravel commit `822c0ed2`, as the
[contract mapping](../grove-skill-delivery-design-evidence.md#rhai-mapping)
records. The document location remains stable for existing citations.

`homebrew-delivery-design-k216` amended this design for the decision record
[Homebrew installs skills and Ravel delivers them](../adr/homebrew-installs-skills-and-ravel-delivers-them.md).
Homebrew installs every Linkuistics skill source. Ravel is the only route that
delivers a skill to a session, and native skill installation retires.
[The keg layout](#the-keg-layout), [Hook adapters](#hook-adapters),
[The retired marketplace](#the-retired-marketplace) and
[The pack record and the release](#the-pack-record-and-the-release) are new.
Their implementation is ahead. The
[evidence](../homebrew-skill-delivery-evidence.md) holds the readings.

`ravel-default-skill-directory-k218` then settled Ravel's default skill
directory in Ravel's skill discovery specification, and this design cites it.

## Ownership

Grove names the skill that its session must load. **Homebrew installs the
skill sources, and Ravel delivers them.** Grove's formula carries core and the
five domain packs, as [The keg layout](#the-keg-layout) states. Ravel's skill
discovery specification decides which sources a launch receives. Grove's binary
neither installs skills nor asks Ravel to resolve a named skill.

When Ravel delivers skills, it injects skill-directory paths into the launch
prompt.
The agent uses those locations to discover and read applicable skills. Ravel
is not a requested-skill lookup service. Directory delivery does not establish
that a named skill exists, applies, or has been read.

The [delivery decision](../adr/homebrew-installs-skills-and-ravel-delivers-them.md)
records why Homebrew installs the skills and why native installation retires.
It also records why nothing writes a native installation automatically.
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

A Homebrew installation of Grove supplies Grove's executables, its readable
caller policy assets, core and the five domain packs. Grove's formula depends
on Ravel, so every Grove user has Ravel. Ravel 2.0.0 reads the sources in its
default skill directory, as [The keg layout](#the-keg-layout) states.
`brew install` and `brew upgrade` are therefore the whole skill setup. A user
who wants personal guidance installs the `personal` formula the same way.

Ravel's settings stay optional. Under Ravel's contract, they add a mandatory
file or replace the default sources. Each hook adapter still needs one
registration command for each harness, as [Hook adapters](#hook-adapters)
states, because Homebrew cannot write a harness's configuration.

Directory delivery needs Ravel 2.0.0 or later. A Ravel 1.x launch delivers no
directory, so its session finds only the native installations that already
exist. Grove's guidance sets up no native installation. A launch with no
source stays valid, and a session that then lacks its skill reports it.

The configuration grants no filesystem access. Neither caller requests
confinement, and Ravel 2.0.0 applies none. On both callers, the harness reads
the retained snapshots under its own sandbox.

A harness session that its user opens directly sees no Linkuistics skill. That
user can read the folders that `ravel skills resolve` lists. Automatic
configuration of such a session is outside scope.

The binary embeds no core-only fallback. Planning remains part of core as the
replaceable default. The formula installs core beside the binary, and only
Ravel delivers it to a session. Neither the formula nor the guidance writes
Ravel's settings, activates personal guidance or copies a sample policy over an
existing one.

If the agent cannot find or read the mandated skill, it reports the missing
guidance and leaves the task live. As its last action, the agent runs the exact
force-stop command that its launch prompt supplies. It does not retire the leaf,
record teardown or send continuation. The mandate must state this rule, because
the missing skill cannot supply it.

The loop stops without relaunching the leaf. The user repairs the selected
delivery route and reruns. Grove does not turn that failure into
declaration-based kind admission. A source that Ravel cannot capture is
instead a Ravel prelaunch failure, with its source-specific remedy.

For Ravel delivery, the accepted source requirements are local source
directories, clear failure for a missing required source and readable
resources throughout reader lifetimes. Those are source-directory obligations,
not resolution of a skill that Grove requests. Ravel's reviewed directory
contract at `822c0ed2` set them, together with exact Git sources that Ravel
later withdrew. The
[design evidence](../grove-skill-delivery-design-evidence.md#upstream-correction)
identifies that contract, its review and the human correction that governs it.

Ravel reads local directories only. It does not fetch, install or update a
source, so delivery needs no network. Homebrew puts each keg source in place.
Ravel's decision record "Skill sources are local directories" removed the
earlier exact Git sources and their offline reuse.

A development working copy can serve as a trial source. Ravel captures its
uncommitted changes, and it retains each distinct capture, so a working copy
is no default source. The
[owner delivery design](owner-skill-delivery.md#skill-formulae) records the
owner's formulae and their advances.

## The keg layout

Each Linkuistics skill formula follows one layout:

| Path in the keg | Content |
| --- | --- |
| `libexec/skill-sources/<source-id>/` | One Ravel source: a copy of its source directory at the accepted commit, with its executable bits and links |
| `share/ravel/skills/<source-id>` | A relative directory symlink to that source in the same keg |
| `libexec/plugins/<plugin>/` | A native plugin that lies in no source, such as `guardrail` |
| `.claude-plugin/marketplace.json` at the keg root | The Claude Code marketplace file of the keg's hook adapters, if it has any |
| `.agents/plugins/marketplace.json` at the keg root | The Codex marketplace file of the keg's hook adapters, if it has any |

Each formula exposes its private source through a directory symlink in
`share/ravel/skills`. Homebrew links that entry into
`$(brew --prefix)/share/ravel/skills/<source-id>` without descending into the
source. That directory is **Ravel's default skill directory**.
`brew install`, `brew upgrade` and `brew uninstall` keep it current, so they
are the whole skill setup. No harness discovers skills under the Homebrew
prefix, so a keg adds no native entry.

Ravel 2.0.0 reads the default skill directory when the settings name no
`sources`. The section "Default skill directory" of Ravel's skill discovery
specification owns that contract. It states how Ravel finds the directory, how
it reads each source and its roots, and what an upgrade does. Explicit settings
can name the same paths, such as
`/opt/homebrew/share/ravel/skills/grove-core`. Ravel canonicalises a path at
capture, so `originPath` names the versioned keg.

| Formula | Sources | Roots | Other content |
| --- | --- | --- | --- |
| `grove` | `grove-core`, `requirements`, `design`, `engineering`, `research` and `writing` | `skills` for `grove-core`, `requirements` and `research`. `skills` and `craft` for the other three | The marketplace files `writing-hooks` for Claude Code and Codex. The Pi extension lies in the `writing` source |
| `personal` | `personal` | `craft` | `libexec/plugins/guardrail` and the Claude Code marketplace file `personal` |
| `testanyware` | `testanyware` | `skills` | TestAnyware's executables |
| `superpowers` | `superpowers` | `skills` | None |
| `quint-llm-kit` | `quint` | `skills` | None |

The unversioned `ravel` formula also links into the shared ancestors
`share/ravel` and `share`. It supplies `examples`, `notices` and
`installation.json`, not a skill source. A fresh Grove installation links its
Ravel dependency first. Promotion links Ravel 2.0 beside the installed skill
formulae. Both events must preserve each source alias. The keg-only
`ravel@2` does not link those entries before promotion.

The directory name is the source ID of the
[owner delivery design](owner-skill-delivery.md#ravel-sources). One token
therefore names a source in the keg, in the default directory and in the
settings. Each keg source holds its skills in the roots that Ravel's
specification reads for a default source.

These facts decide the layout:

- Homebrew initially links an unshared directory as a whole. When a later keg
  needs it, `Keg#resolve_any_conflicts` removes that link and relinks the
  displaced subtree with the `:mkpath` strategy. With real source directories
  under `share`, this descends through every directory and links individual
  files back into the keg. Those file links leave the source boundary, and
  Ravel refuses the whole resolution as `skill_resource_invalid`.
- `Keg#link_dir` handles a symlink as a file, even when it names a directory.
  The formula-owned source symlink therefore stops traversal under both the
  ordinary and conflict-repair strategies. Initially Homebrew may link a parent directory as a whole.
  After a collision, each source remains one entry. The entry links through
  its keg alias to its private payload. No source file
  is expanded into a Homebrew link.
- Two formulae that ship one source ID conflict at that entry. Homebrew then
  unlinks the whole later keg, including its other sources and executables.
  During an upgrade, the earlier keg can also remain unlinked. Resolve the
  ownership conflict and link the intended keg before resuming launches.
  Never use overwrite to conceal a duplicate.
- Homebrew's cleaner sets file modes only under `bin`, `sbin` and `lib`, so a
  keg source keeps the modes of its copy. Its prune step skips `libexec`,
  which holds the payload. The alias under `share` resolves to that payload,
  so pruning preserves it too. A dangling alias is an installation defect.
- Homebrew unlinks the earlier keg before it installs the new one. It links a
  keg, and moves its `opt` link, only after the install and the cleaner. A
  link in the default directory therefore names a complete keg, or nothing
  while an upgrade installs the new keg.
- The marketplace files sit at the keg root, which Homebrew links nowhere. Two
  kegs therefore cannot conflict over one marketplace path.

The linking rules above are in Homebrew 7.0.12 at
[`3ccfa1d07143`, `Keg#resolve_any_conflicts` and `Keg#link_dir`](https://github.com/Homebrew/brew/blob/3ccfa1d07143/Library/Homebrew/keg.rb).
The [two-keg experiment](../homebrew-skill-delivery-evidence.md#directory-link-layout-experiment)
reproduces both the failure and the proposed remedy with Homebrew itself.
This layout amendment is a design. Formula implementation and publication
must precede reliance on the guarantee.

**Ownership and migration.** The Grove formula template owns its projection.
The tap owns the `personal`, `superpowers` and `quint-llm-kit` formulae.
TestAnyware's `scripts/templates/testanyware.rb.tmpl` generates its tap
formula, so both must carry the layout.

Formula install methods move the source payload from their input
archive into the private location and create the relative alias. Release
archives can retain their existing payload layout. Neither Ravel nor a
formula's post-install hook repairs another formula's links.

Ravel's skill discovery specification and Homebrew directory view must explain
this packaging rule before Ravel 2.0.0 publishes. Ravel's resolver remains
unchanged. Its own formula must participate in the shared-directory acceptance.

Each affected published formula needs a new revision or release, even if its
source commit is unchanged, so an ordinary upgrade installs the remedy.
Existing expanded sources remain invalid until their formula is upgraded or
relinked. During migration, complete the upgrades of all installed skill
formulae before resuming default-directory launches. A mixed old/new set
can still expand an older source. For an affected old formula,
`brew unlink <formula>` followed by `brew link <formula>` is recovery after the other
installations, not the installation contract. Do not repair individual files
or use `brew link --overwrite`.

Acceptance resolves the shared default directory, checks every expected ID
and digest, and checks that each entry canonicalises into its owning keg's
private source. It also checks regular payload files remain regular files.
Resolving versioned keg paths alone cannot detect the original failure.
The durable [shared-directory seam](#shared-default-directory-acceptance)
owns the cases and repeat conditions. The owner design's
[formula acceptance](owner-skill-delivery.md#skill-formulae) owns acceptance
of a revision that preserves the source commit.

**Identity.** The accepted identity of a keg source is its `resourceDigest`.
It equals the digest of a clean plain Git checkout of the same commit, made
with Git's global and system configuration off. The release copies each source
from such a checkout. The cleaner sets no mode under `share/` or `libexec/`,
and the digest covers no write bit and no owner. Homebrew leaves those files writable, so a
digest comparison detects an edit of a keg, and the file mode does not.

**Other formulae.** The formulae of `personal`, `superpowers` and
`quint-llm-kit` each pin one commit through a GitHub source archive and its
`sha256`. mise cannot evaluate a formula without a `sha256`.

A third-party keg
copies the narrowest subtree whose top holds the roots that Ravel reads. The
subtree also holds every resource that the skills name by relative path. For
superpowers, the top is the repository root, so the mandatory path
`skills/using-superpowers/SKILL.md` stays. For quint-llm-kit, the top is
`quint-llm-kit-plugin`, so the source's root is `skills`.

Each such formula raises its version with each advance. For superpowers and
quint-llm-kit, the version is the upstream release of the pinned commit. For
PersonalSkills, which has no releases, it is the date of the pinned commit.
TestAnyware's release archive carries its `plugins/testanyware` at
`share/ravel/skills/testanyware`.

**Lifetime.** Ravel copies a keg source into a retained snapshot at each
launch, so a running session never reads a keg. A capture through the default
directory reads one complete keg, or it finds the source absent while an
upgrade installs the new keg. Ravel's specification states what a resolution
then delivers. A capture refuses as `skill_source_changed` while cleanup
removes the earlier keg under it. A rerun captures the new keg.

Each keg version adds at most one snapshot for each source.

The design rejected these layouts:

- **Real source directories under `share`, followed by relinking.** A manual
  relink leaves an extra installation step and can be undone by a later
  formula link. A formula hook that relinks other kegs couples independent
  installations and must discover and repair shared state. The directory
  symlink keeps that knowledge inside the source's own formula.
- **Allow file links anywhere inside the same keg in Ravel.** This changes
  the generic source boundary for a packaging defect. The formula can keep
  an intact source with the existing resolver. A `mac-check` report alone
  also leaves new installations broken. It supplies detection and recovery help.
- **`libexec/skills/<source-id>`, named by `opt` paths in the settings.**
  Homebrew links and prunes nothing there. Each user would still have to write
  the settings, so `brew install` would not be the whole setup.
- **Links in `~/.config/ravel/skills/`.** Ravel refuses a real directory inside
  its configuration directory, and Homebrew's sandbox cannot write a link in
  HOME. A step outside Homebrew would remain.

## Hook adapters

Ravel installs no hook. A keg therefore ships a marketplace file for each hook
adapter in it, and each harness registers that file from the keg's stable
path. No remote marketplace lists an adapter.

| Adapter | Keg and marketplace | Harness identity |
| --- | --- | --- |
| Technical-communication hooks for Claude Code | `grove`, `writing-hooks` | `technical-communication-hooks@writing-hooks` |
| Technical-communication hooks for Codex | `grove`, `writing-hooks` | `technical-communication@writing-hooks` |
| Technical-communication extension for Pi | `grove`, in the `writing` source at `craft/technical-communication/harnesses/pi/` | `~/.pi/agent/extensions/technical-communication.ts` |
| `guardrail` for Claude Code | `personal`, `personal` | `guardrail@personal` |

The plugin root of each technical-communication adapter is the skill directory
`craft/technical-communication` in the `writing` source. The
[pack membership design](pack-membership.md#the-always-on-hooks) states why.

Both marketplace files sit at the keg root and name the private payload
directly, at `./libexec/skill-sources/writing/craft/technical-communication`.
They do not traverse the source alias under `share`. Codex
resolves a `local` source only inside its marketplace root. No
marketplace has the name `linkuistics`. The Codex marketplace keeps the name
`writing-hooks` of WritingSkills, and WritingSkills removes its own root
marketplace file.

| Harness | Documented command | When |
| --- | --- | --- |
| Claude Code | `claude plugin marketplace add "$(brew --prefix)/opt/grove"`, then `claude plugin install technical-communication-hooks@writing-hooks` | Once |
| Claude Code, `guardrail` | `claude plugin marketplace add "$(brew --prefix)/opt/personal"`, then `claude plugin install guardrail@personal` | Once |
| Codex | `codex plugin marketplace add "$(brew --prefix)/opt/grove"`, then `codex plugin add technical-communication@writing-hooks`. Then run `codex` once and choose "Trust all and continue" at its hook review | After each installation or upgrade of Grove |
| Pi | `ln -s "$(brew --prefix)/share/ravel/skills/writing/craft/technical-communication/harnesses/pi/technical-communication.ts" ~/.pi/agent/extensions/` | Once |

The [readings on the projected keg](../homebrew-skill-delivery-evidence.md#the-hook-adapters-on-the-projected-keg)
support the registration lifecycle below on actual formula output. They
cover the private marketplace paths and Pi's public alias through upgrade
and cleanup.

- Claude Code stores a directory marketplace by the path given. It runs the
  plugin in place through that path. A plugin registered by the `opt` path
  therefore survives an upgrade and its cleanup. Its next hook call runs the
  files of the new keg.
- A running Claude Code session keeps the hook declarations that it loaded.
  After an upgrade, it can run a new script under an old declaration. The
  technical-communication hooks fail open, so such a call can lose one layer
  once and breaks nothing.
- Codex stores the canonical path, which names the versioned keg, and copies
  the plugin into its cache. Cleanup of the earlier keg then breaks the
  registration, and the copy keeps the earlier bytes. A session still runs
  the hooks of that copy. A second `marketplace add` stores the new path, and
  `plugin add` copies the new bytes.
- Codex runs a plugin hook only after the user trusts it, and the trust
  covers the plugin's bytes. `codex exec` skips an untrusted or changed hook
  and reports nothing. An interactive `codex` shows a review of such hooks, and
  "Trust all and continue" trusts them. Each `plugin add` of new bytes
  therefore needs the review again.
- Pi resolves a link through the default directory each time it loads the
  extension.

Grove's formula caveats print the four commands and the Codex review, so
each upgrade shows the Codex step again. A Claude Code installation enables the plugin. A user who
wants it installed and off runs `claude plugin disable` next, because no option
installs a plugin disabled.

## Supported routes

Grove supports three Ravel routes. The owner selects one for each Grove
invocation through the PATH, the configuration file and the settings. Grove
reads none of these. Its version probe decides admission only, and it selects
no route. The [route view](../design/harness-selection-and-execution/README.md)
shows the same decision.

| Route | Ravel | Configuration | Settings | Caller assets | Skills |
| --- | --- | --- | --- | --- | --- |
| Native on 1.1 | Stable 1.x from 1.1.0. Homebrew's unversioned `ravel` until promotion | `~/.config/ravel/policy.ts` with request and policy schema 2 and `ravel/sdk` | Only `timeoutMs`, `contextBytes` and `policyEnv` | Grove's TypeScript assets | Native installations that already exist |
| Native on 2.0 | Stable 2.x from 2.0.0. The keg-only `ravel@2` until promotion | `~/.config/ravel/config.rhai` with `SCHEMA_VERSION` 4 | The keys of 2.0.0 with `"sources": []` and no mandatory file. Settings without `sources` give this route only while no keg source is installed | Grove's Rhai assets | Native installations that already exist |
| Directory on 2.0 | As native on 2.0 | As native on 2.0 | The keys of 2.0.0 without `sources`, with installed keg sources. Or a `skills` object that names sources | Grove's Rhai assets, which grant harness reads from `request.skills` | Keg sources from the default skill directory, or configured sources. Native copies are unnecessary |

Only the directory route delivers skills that Grove's guidance sets up. With
Ravel's default skill directory, that route needs no `skills` key. The two
native rows stay caller routes. Grove admits them and launches through them,
and the session finds whatever native installation exists. Ravel's contract
decides which settings disable delivery on 2.0.

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

The default skill directory belongs to the Ravel installation, not to a HOME,
as Ravel's specification states. A separate HOME therefore isolates the
settings but not the keg sources. A HOME on the native route on 2.0 holds
`"sources": []`, which Ravel 1.1 refuses.

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

A launch on a native route needs no Ravel 2.0.0, and it delivers no directory. Policy examples and installed
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
| Missing named skill | Repair the formula installation or the configured sources. Do not substitute a native installation. |
| Unavailable configured source | Preserve Ravel's source-specific diagnostic. Restore the source or deliberately amend owner configuration. |
| Read access, capture, cache, timeout or limit failure | Preserve Ravel's precise remedy. Do not silently substitute another source or skip guidance. |
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

The [external pack contract](external-pack-contract.md) owns the layout of a
pack, its references between skills, its family skills, its licences and its
conformance. This section keeps what delivery adds to that contract.

Directory delivery preserves the resource layout of each source. The agent
locates another skill through the active delivery route, and it reads that
skill's resources from that skill's directory. It assumes no sibling checkout,
native directory or Grove cache layout.

The [single-owner](../adr/corpus-rules-have-one-owner.md) and
[restatement](../adr/restatement-declares-its-class.md) decisions still govern
normative prose. Conformance checks content over the supplied packs. Neither
executable reads a pack to admit a kind. Ravel delivering readable directories
is not proof that prose references work or that an agent follows them.

The `harnesses:` key of a skill's metadata has no reader, because the manual
installer is gone. It stays as metadata, and Ravel reads no frontmatter. Directory
injection implies neither harness filtering nor installation of native hooks.
Pack descriptions and guidance must make harness-specific requirements clear.
Acceptance cannot treat a readable native-only integration as working elsewhere.

## Packaging and rollout

Grove's archive holds the executables, the caller assets, core and the five
domain packs, in [the keg layout](#the-keg-layout). It holds no private worker
and no skill resolver. Startup and upgrade write no native skill installation.
No manual installer and no marketplace package remain.

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

### The pack record and the release

Grove's record of the packs is `plugins/packs.json`. Each member is keyed by
its source ID and holds `url`, `path` and `sha`. They are the public Git URL of
the pack, its source directory and a full commit ID. The record names the five
domain packs and nothing else.
[The checker without a copy](external-pack-contract.md#the-checker-without-a-copy)
states how `task check:packs` reads it.

The release takes the packs from that record:

1. The release runs `scripts/check.sh`, and with it `check:packs`, before the
   cut.
2. The archive build uses the plain checkout of each `sha` that `check:packs`
   uses, under the same isolation and reuse rule.
3. It copies each pack's source directory to `share/ravel/skills/<source-id>/`.
4. It copies `plugins/grove` of the tagged tree to
   `share/ravel/skills/grove-core/`.
5. It copies the two marketplace files to the archive's root.
6. It checks each archive against the archive manifest, which lists each
   copied file with its Git mode.

Each archive copy keeps executable bits and links. The formula moves those
sources into `libexec/skill-sources/` and creates the relative directory
symlinks under `share/ravel/skills/`. It installs the two dot directories at
the keg root, beside the executables and `libexec/`. A cut changes only
`CHANGELOG.md`, so the `grove-core` bytes of a release equal those of its
candidate.

A change to a pack reaches users in four steps. The pack publishes a commit. A
Grove change records that commit. `check:packs` passes. `task release:patch`
publishes Grove. A pack change that breaks core follows the expand and contract
steps of the external pack contract.

### The retired marketplace

Grove's `.claude-plugin/marketplace.json` lists the packs until finish moves
`main`. After finish, it keeps the name `linkuistics` and an empty `plugins`
array. Its description says that the marketplace is retired and points to
Homebrew and Ravel. `claude plugin validate` accepts that file with one
warning, and a refresh of it succeeds. A user with automatic updates therefore
sees no recurring error. The design rejected removal of the file, because each
automatic refresh would then fail.

The plugin manifests of core and each pack stay in their source directories as
layout metadata. No route installs them, and `claude plugin validate` still
checks them. The hook adapters need theirs.

The release notes of finish's Grove release tell a user of the `linkuistics`
marketplace what to do:

1. Install or upgrade Grove with Homebrew.
2. Use Ravel 2.0, which reads the keg sources from its default skill directory
   with no settings.
3. Register the hook adapters by the documented commands.
4. Remove the marketplace with `/plugin marketplace remove linkuistics`. That
   removal uninstalls each plugin of the marketplace.

The notes also state what a refresh of the retired listing does to an
installed plugin, from the reading of the seam "Retired listing". The
plugin stays installed and enabled, but it no longer loads, and no notice
appears. The notes therefore put the first three steps before any refresh.

Two sentences of PersonalSkills' `authoring-conventions` lose their referent.
"This marketplace" becomes "the Linkuistics skill packs". The claim that
`install.sh` reads the `harnesses:` key becomes the statement that no installer
reads it. The change is a PersonalSkills commit, which the `personal` formula
then pins.

## Test seams

Use existing Grove lifecycle, standalone and public session-command seams with
isolated homes and actual installed Ravel. Each case names its route and
identifies its Ravel build by commit and build identity. Ravel owns source
resolution tests. Grove checks the observable caller behaviour.

`crates/grove/tests/directory_route.rs` proves the lifecycle part of five rows
below: Directory bootstrap, Source and skill failures, Original mandate,
Settings isolation and Resource lifetime. Its default-directory case copies the
2.0 build with an installation record, as Ravel's specification defines it.
The record names a fixture directory of Homebrew-shaped links. `task ravel:directory` runs it, with
`RAVEL` as the 2.0 front and `OLD` as the 1.1 front. Its fake agents read the
paths that the prepared prompt names, and they prove delivery rather than LLM
behaviour. The explicit plugin-qualified name request, the missing-skill report
of a real agent and actual harness behaviour belong to acceptance.

The same file proves the directory part of the Standalone row. A `grove run`
on 2.0 reads the spine, a resource and the named kind from the snapshots. A
write to a snapshot file fails by its file mode.

`crates/grove/tests/standalone.rs` proves the native part of the Standalone row
on each runtime. With injection disabled, the scripted agent reads its native
copies under HOME.

The fixture also proves Grove's interpretation of a force-stop. When its
scripted agent finds no native copy and force-stops, Grove publishes nothing
and launches the agent once. The fixture is not evidence of agent behaviour,
because its script decides to stop. `task ravel:directory` runs both parts on
the 2.0 build.

On each Linux target, the general floor route runs the directory part on the
2.0 lane. It runs the case at the glibc 2.17 floor and at the emulated CPU
floor, as uid 1000. A snapshot therefore refuses a write by its file modes. The
floor userland has no Git, so the two sources of that case are plain local
directories. The [installed smoke test](../RELEASING.md#installed-smoke-test)
states the route.

The native part has no floor case. Its earlier floor case needed the sandbox
to make the native copies unreadable.
The [dual-Ravel stage record](../grove-dual-ravel-stage-record.md) names the
builds and the results.

| Acceptance | Observation |
| --- | --- |
| Native-only bootstrap on 1.1 | With Ravel 1.1.0 and `policy.ts`, launch the lifecycle caller using complete native skills. Grove does not demand source configuration. The standalone caller reads the same native roots under the harness sandbox. |
| Native route on 2.0 | With Ravel 2.0.0, `config.rhai` at schema 4 and delivery disabled, launch the lifecycle and standalone callers using complete native skills. `"sources": []` disables delivery, or a copied build without an installation record does. `request.prompt` equals `request.originalPrompt` byte for byte, and both equal the mandate. The Rhai assets give the 2.0 outcome of each case in the shared table. They differ from the TypeScript outcomes only where [runtime parity](#runtime-parity) accepts it, and their receipts match byte for byte. This row proves Rhai native compatibility, not directory delivery. |
| Directory bootstrap | Use Ravel 2.0.0 with the `skills` key, in a HOME separate from any 1.1 reader. With native copies absent, inject complete source directories outside cwd and read the spine, named kind and supporting resources. Exercise local sources, Grove's bare-name mandate and an explicit plugin-qualified name request. Neither name request requires a native loader. With no `skills` key, a launch receives each source of the fixture default directory. With `"sources": []` beside that directory, it receives none. In the installed distribution, Ravel reads the keg sources from its default skill directory with no `skills` key. |
| Source and skill failures | A missing configured source directory refuses before launch, with Ravel's remedy. For a missing or unreadable mandated skill, the agent reports and uses the prompt's exact force-stop command. The leaf remains live, no teardown is recorded and the loop does not relaunch. Grove's kind parser does not reject it. |
| Original mandate | The mandate qualifies "first" with "after any mandatory guidance". Observe mandatory reads before the named-kind read. Compare exact caller bytes and ending commands before/after Ravel preparation. `originalPrompt` and the caller block of the prepared prompt each equal the composed mandate byte for byte. The selected command receives the intended launch prompt. |
| Explicit loading mechanism | Exercise a native loader with a match, absent-match directory fallback with disclosure, an express native-only restriction and a concrete path. Observe the specified route or a report before dependent work. Do not silently substitute. |
| Compatibility | Check the admission of stable 1.x from 1.1.0 with `policy.ts` and the TypeScript assets. Check the admission of 2.x from 2.0.0 with `config.rhai` and the Rhai assets. Refuse 1.0, prereleases, 3.x and malformed output, with no fallback to another PATH entry. Before the cut, stub executables prove the admission of `ravel 2.0.0` output. |
| Settings isolation | Ravel 1.1.0 refuses shared settings that hold a 2.0-only key, and a separate prefix cannot prevent it. With a fixture default directory, a 2.0 launch in a shared HOME without `skills` delivers its sources. A separate HOME runs a native lifecycle launch on 1.1 and a directory launch on 2.0 at the same time. A mismatched configuration reaches the user as Ravel's refusal and leaves the leaf live. Existing ending commands stay usable. |
| Prefix selection | With unversioned 1.1.0 and the keg-only `ravel@2` installed, the default PATH selects 1.1.0. A PATH that puts the `bin` directory of `ravel@2` first selects 2.0.0, and the ending commands quote that canonical path. |
| No native mutation | Compare owned/foreign files, links, modes, pointers and bytes before/after startup, failure and installed upgrade. Include legacy homes, aliases and an active old reader. |
| Manual cleanup guidance | Exercise the documented procedure in disposable legacy layouts. These layouts include stopped readers/writers, owned and foreign links, aliases and uncertain provenance. They also include retained links that reach a skill entry through intermediate links or `..`. Only evidenced, unneeded owned skill-directory links are removable. The legacy root stays unchanged, and the trial leaves the owner's installation untouched. |
| Resource lifetime | A reader keeps its original resolved directories while sources/configuration change. Subsequent launches can receive the new source view. |
| Standalone | Grove sends neither `--confine` nor `--runtime-read` to Ravel 1.1 or 2.0. On the directory route of 2.0, the agent reads the required resources outside cwd. A write to a snapshot file fails by its file mode. With injection disabled, the agent reads its native roots. A force-stop publishes no output. A standalone prompt names no guidance, so Grove does not require a report or a force-stop. The confined results of 2026-11-03 and earlier, such as `missing-kind-confined-20261103T062817Z`, are historical, because the human withdrew confinement on that day. |
| Open tree behaviour | Existing custom-kind, handle, review/integration, result and epoch tests pass. No availability check enters tree verbs or launch admission. |
| Conformance | Core, external-pack and aggregate checks use exact supplied artefacts. `check:packs` reads Grove's record and checks each pack alone and in the aggregate with core. Removed required resources/edges and duplicate owners fail deliberate controls. The [external pack contract](external-pack-contract.md#controls-that-must-fail) lists the controls. |
| Installed distribution | The archive manifest, `task release:smoke` and `task release:formula:test` preserve the recorded commits, executable bits, links and source digests. Archive and installed paths follow [The keg layout](#the-keg-layout). Formula acceptance includes the shared-directory seam below, with actual formula output. `task tap:formula:test` applies the same source comparisons to each affected tap formula. Archives and formula tests retain separate packages, both caller-asset sets and runtimes, both configuration formats and no automatic native mutation. Fresh installs and upgrades cover tap states A to D of the coordinated release. Each state's Grove dependency resolves to an admitted Ravel major, and feature advertising follows publication. |
| Shared default directory | [Shared default directory acceptance](#shared-default-directory-acceptance) runs actual Homebrew linking on formula output, including Ravel's shared ancestors. A private installation record names the shared prefix directory. Direct keg resolution cannot satisfy this seam. |
| Hook adapters | Repeat the documented commands in isolated Claude Code, Codex and Pi homes against the revised formula output. Both marketplaces name the private writing payload. Claude Code runs its hooks in place through the registered `opt` path, with no native skill. Codex accepts the local source and copies the complete plugin into its cache. Pi reaches the extension through the public alias. After actual upgrade and cleanup, check Claude Code and Pi read the new bytes. Record the Codex registration failure, cached hook behaviour, registration repair and renewed interactive trust of changed bytes. Native-only plugins receive the payload and capability acceptance below. The [readings on the projected keg](../homebrew-skill-delivery-evidence.md#the-hook-adapters-on-the-projected-keg) hold the result, and `task seam:hook-adapters` reruns them. The earlier [readings](../homebrew-skill-delivery-evidence.md#the-hook-adapters-from-a-keg) establish the old layout only. |
| Retired listing | Before finish, an isolated Claude Code home adds the `linkuistics` marketplace from a Git remote at a commit that lists the packs, and installs `grove@linkuistics`. A refresh against a commit with the retired listing shows whether the plugin stays installed. The release notes state the result. The [evidence](../homebrew-skill-delivery-evidence.md#a-refresh-of-the-retired-listing) holds the reading. |
| Actual harness behaviour | Claude Code and Codex each discover an applicable craft skill from a task that does not name it, and read it before the governed action. Ravel delivers every source from a keg path, and no native copy exists. |

### Native-only hook acceptance

Hook acceptance also compares complete native-only payloads with their
accepted subjects. Repeat `guardrail` registration and its recorded capability
trial from the revised personal keg. A successful no-op hook must fail the
trial's refusal assertion. An unchanged skill source digest cannot replace
that evidence for a native-only plugin. The
[readings on the projected keg](../homebrew-skill-delivery-evidence.md#the-guardrail-capability-trial)
hold the payload comparison, the trial and its control.

### Shared default directory acceptance

`release:formula:test` and each applicable `tap:formula:test` run this seam
before formula publication. Release acceptance repeats it with the published
Ravel archives. The design experiment is a reusable failure control, not a
substitute for formula output. `scripts/check.sh` alone does not establish
this installation guarantee.

Use a private prefix and actual Homebrew install methods, cleaner and
`Keg#link` operations. Cover both skill-formula installation orders, absent
shared ancestors, and Ravel-first dependency installation. Also cover Ravel
promotion beside installed sources, mixed-layout migration, upgrade, old-keg
cleanup and uninstall. All five skill formulae and Ravel's shared entries
must participate in the applicable cases. A case for a duplicate ID covers
fresh installation and upgrade, including rollback of the whole later keg.

After each successful link, resolve the shared default directory with no
source settings. Compare the exact expected ID set, roots and digests with
the accepted sources. Compare each canonical `originPath` with its owning
`libexec/skill-sources/<source-id>` payload. Check that regular payload files
remain regular.

After a refused duplicate, check the earlier owner's
sources. Check that the prefix link directories contain none of the refused
keg's source or executable entries. This does not require removal of its
`opt` link or installed keg.

Record the formula content and full package versions, Ravel build identity,
and Homebrew revision. Digest the instrument and Homebrew's linker,
installer and cleaner before and after the run. A changed subject invalidates
the reading. Every formula validation reruns the seam against its Homebrew,
so it does not depend on detecting a later linker change. Before accepting a
new Homebrew version, repeat the failure control and both installation orders.

`task seam:shared-directory` runs this seam alone, and both formula tests run
it. `scripts/skill-formulae.json` declares the layout of each skill formula,
in the tap working copy and on the published tap. The seam refuses a keg that
differs from its declaration. Change a declaration with its formula.

The failure control links the published direct formulae and two direct
fixture formulae. The fixtures keep the control usable after every tap
formula is projected. A fixture formula also supplies the duplicate ID and a
migration pair. The seam identifies Ravel's archive by its lane in
`scripts/ravel_target_inputs.py`, because Ravel prints no build identity. The
[seam evidence](../homebrew-skill-delivery-evidence.md#shared-directory-seam)
records the instrument's runs and controls.

Directory-route harness trials include supporting reads, continuation/compaction,
one delegate, and coexistence with a stale native copy. Record
harness/model/version, isolation, harness sandbox mode, directory identity and observed
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
identify inspected consumers. Research remains the first external-pack proof.
The [external pack contract](external-pack-contract.md) records the reusable pack
tooling for later moves.

`homebrew-skill-delivery-k215` plans the implementation of the Homebrew
installation, the hook adapters, the pack record and the retired marketplace.
`manual-installer-removal-k236` removed the manual installer, with its
`--pack` option and its tests.

Ravel's reviewed contract at `822c0ed2`, this adoption and its integrated
focused review supply dependent planning. The TypeScript contract at `b93bb927` supplies
intent only, and the generated-catalogue proposal is historical input. Internal test wiring and
measured harness delegate behaviour remain implementation questions. No kind redesign, stop command, review simplification,
repository relocation or native cleanup occurs here.

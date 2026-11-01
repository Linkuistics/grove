# Owner skill delivery through Ravel

This design moves the owner's installed skill collection from native harness
installations to Ravel's directory delivery. It awaits its focused design
review. It changes no installation, setting or file on the owner's machine.
The [evidence document](../owner-skill-delivery-evidence.md) records the
inputs, and [the baseline](../owner-skill-inventory.tsv) classifies each
installed entry.

Ravel's directory contract owns the feature itself. The
[coordinated release design](coordinated-release.md) owns publication order,
promotion and recovery. The [Grove delivery design](codex-skill-provisioning.md)
owns Grove's behaviour and the general cleanup procedure for other users. This
document owns the owner's source set, the native capabilities that stay, and the
ordered cutover on this machine.

## Problem

Four routes deliver skills on this machine today, and they disagree:

- **Claude Code** installs marketplace plugins. The Linkuistics marketplace
  follows `Linkuistics/grove` main with automatic updates. Third-party
  marketplaces and the personal skill directory add more entries.
- **Codex** receives Grove 22.8's release snapshot through links in both of
  its user roots. Older superpowers copies and links into a Quint clone sit
  beside them.
- **Pi** has manual links to the earlier checkout path. Each link dangles.
- **Tools** install their own skills and hooks: codebase-memory-mcp, herdr,
  Orca, and a terminal-browser cask that no longer exists.

The same skill therefore has different bytes in different harnesses, and some
names appear twice. An earlier repository move broke every Pi link without a
report. The planned pack moves will move every owner source again.

The owner's target is Ravel insertion of the directories of every
owner-selected skill. Four facts constrain the transition:

1. Ravel delivers readable directories and ordered mandatory reads. It installs
   no hook, command, agent definition, MCP server or tool.
2. This driver resolved Ravel 1.1.0 once, at its canonical path. That version
   cannot insert directories. Every remaining session of this grove loads its skills
   natively. The owner policy routes `finish` to Claude Code.
3. Grove 22.8 rewrites its Codex links at each Codex launch. Only compatible
   Grove, which finish releases, stops that writer.
4. Ravel 1.1 and Ravel 2.0 read the same `~/.config/ravel/settings.json`. The
   file does not exist today. A 1.1 launch refuses settings that hold keys it
   does not know.

## Solution

1. Classify every baseline entry by its owner and by what it supplies. Only
   owner-selected guidance moves to Ravel.
2. Deliver that guidance from ten Ravel sources at published commits, with one
   mandatory file.
3. Keep every native capability that a read cannot supply. That covers hooks,
   commands, agent definitions, MCP and LSP servers, tools, platform skills and
   one Claude Code skill.
4. Prove the replacement on both harnesses in a staging HOME before finish.
5. Inside finish, after Grove's release and Ravel's promotion, switch the real
   HOME in one reversible operation. Verify both harnesses, and roll back on
   failure.
6. Purge retained bytes only after this driver's reap, on reader evidence.

The [classification view](../design/harness-selection-and-execution/index.html#diagram-owner-classes),
the [cutover view](../design/harness-selection-and-execution/index.html#diagram-owner-cutover)
and the [final-session view](../design/harness-selection-and-execution/index.html#diagram-owner-final-session)
show the same design.

## Decisions

### Terms

- **Deactivation** takes an entry out of native discovery and keeps its bytes.
  For a Claude Code plugin, it disables the plugin. For a file-system entry, it
  moves the entry into the retained tree of the cutover backup.
- **Purge** erases retained bytes. Removal of a native installation is
  deactivation followed, later, by purge.
- **The switch** is the one operation that activates directory delivery in the
  real HOME and deactivates every superseded entry.
- **Native residue** is the set of native entries that stay after the switch.

### Classes and dispositions

| Class | Baseline members | Disposition |
| --- | --- | --- |
| Owner pack | The `grove`, `linkuistics` and `testanyware` plugins and their skills. Grove's Codex snapshot links. | Ravel source by destination repository. Deactivate the native entries in the switch. |
| Selected third-party pack | Superpowers 7.0.0 and quint-llm-kit 1.0.0, with their Codex copies and links | Ravel source at the upstream commit. Deactivate the native entries in the switch. |
| Stale copy | The earlier `grove` copy in `~/.claude/skills`. The superpowers copies in `~/.codex/skills`. | Deactivate in the switch. |
| Dangling entry | `terminal-browser` links in three roots. The Pi skill links and the Pi technical-communication extension link. | Deactivate in the switch. No reader can read through them. |
| Tool-managed | codebase-memory skill copies, agent definitions and hooks. herdr and Orca hooks. The `testanyware`, `quint` and `codebase-memory-mcp` executables. | Unchanged. Their installers own them. |
| Capability-only | Enabled plugins without skills: Context7, Playwright, commit-commands, feature-dev, the explanatory output style and Swift LSP. All MCP entries. | Unchanged. |
| Platform-managed | Claude Code built-in skills, account-synced skills, Codex system skills and Codex bundled plugin skills | Unchanged. They are outside the owner's control and are no Ravel source. |
| Disabled third-party | Claude Code plugins that the owner disabled, with their skills | Outside the active collection. Unchanged. |

Two owner-pack skills have their own route:

- **`guardrail`** stays a native Claude Code skill. A read cannot install its
  frontmatter hook or its `/guardrail` command. It stays outside every Ravel
  search root.
- **`technical-communication`** splits. Ravel delivers its guidance. A
  skill-free native plugin keeps its always-on hooks and their scripts.

Pi is outside the automatic contract, because no owner policy route selects
it. The switch deactivates its dangling entries and gives it no replacement.
If the owner reinstalls the terminal-browser cask, its installer owns those
links again.

### Ravel sources

| Source ID | Repository | Search roots | Delivered skills |
| --- | --- | --- | --- |
| `grove-core` | `Linkuistics/grove` | Core kind and spine directories | Spine, `configure-grove`, finish, and the planning chain |
| `requirements` | RequirementsSkills | Kind directory | The requirements chain |
| `design` | DesignSkills | Kind and craft directories | The design chain, `decision-records`, `codebase-design`, `cli-tool-design` and `model-led-development` |
| `engineering` | EngineeringSkills | Kind and craft directories | The impl and prototype chains, the coding styles, `using-codebase-memory`, `simplify-project` and `doubt-driven-development` |
| `research` | ResearchSkills | Kind directory | The research pair and the combiner |
| `writing` | WritingSkills | Kind and craft directories | The editorial kinds, `technical-communication` and `writing-code-walkthroughs` |
| `personal` | PersonalSkills | Craft directory | `machine-environment`, `using-jujutsu` and `authoring-conventions` |
| `testanyware` | TestAnyware | Its skill directory | `using-testanyware` |
| `superpowers` | `obra/superpowers` | `skills` | All fifteen superpowers skills |
| `quint` | `quint-co/quint-llm-kit` | `quint-llm-kit-plugin/skills` | `quint-lang`, `quint-modeling` and `quint-execute-spec` |

These rules bind the layout. The research-pack proof settles the exact paths.

- Configure one source for each repository.
- Set its source directory to the narrowest directory that holds the delivered
  skills. That directory also holds every resource they reference by relative
  path. Never use a repository root that holds build output. TestAnyware's root holds `target/`.
- Expose only Ravel-delivered skills through search roots. Keep native-only
  skills and plugin-only trees outside every root.
- Keep each relative symlink inside its source. Ravel refuses absolute and
  escaping links. The baseline found no link in any delivered skill.

The first pins are known: superpowers
`bb92a77741419a4ab5f06e711a283343f1ada0c3` and quint-llm-kit
`cc75369f741af7d490936f82002c2d28e3b3d78d`. These are the commits of the
installed plugins. Rollout reads both again before it writes the settings.

### Pin policy

Every source in the owner's default settings is a Git source. It names the
repository's published remote and a full commit ID. The owner advances a pin by
a deliberate edit of the mise-tracked settings, after the commit is published.
Local sources appear only in trial settings inside a staging HOME.

The policy costs the automatic updates that the Claude Code marketplace gives
today. An unpublished edit cannot reach a launch until its pin advances. In
return, both harnesses read identical bytes, warmed pins work offline, and each
pin has exactly one snapshot. A rebuilt machine also fetches the same set from
the remotes.

The design rejected local sources in the default settings. Each launch would
capture a live working copy, including uncommitted or conflicted jj states.
Ravel retains every distinct capture and has no collector, so storage would
grow with each edit. The marketplace keeps its own revisions for native users.
Rollout verifies that the marketplace and the owner's settings name the same
commits at the switch. Later divergence is allowed when it is deliberate.

### Mandatory guidance

The default mandatory list holds one file: superpowers
`skills/using-superpowers/SKILL.md`.

- It replaces the superpowers plugin's `SessionStart` hook. That hook injects
  the same text into every Claude Code session today.
- Through Ravel, the file also reaches Codex launches. That is a deliberate
  change, because Codex today has older copies without that text.
- The file is 3,192 bytes, well under the historical 64 KiB bound. Its
  harness references are relative reads inside the same source.

The always-on instruction of `technical-communication` is not mandatory. The
[off-file decision](../adr/the-off-file-is-the-only-switch.md) makes one file
the only switch for every always-on layer. Ravel reads a mandatory file
unconditionally, so it would ignore that switch. No Grove or personal guidance
is mandatory, because Grove's mandate names the kind skill. The explanatory
output style is a harness style rather than guidance. It stays a native hook.

### Native residue

| Capability | Route after the switch |
| --- | --- |
| Always-on technical communication on Claude Code | A skill-free Claude Code plugin from WritingSkills. Its root is the skill directory, as for the existing Codex hooks plugin. |
| Always-on technical communication on Codex | Absent today, and the switch keeps it absent. Installing the existing Codex hooks plugin is a separate owner choice. |
| `guardrail` | A Claude Code plugin from PersonalSkills that holds only `guardrail` |
| Superpowers bootstrap text | The Ravel mandatory file. The superpowers plugin is disabled. |
| Explanatory output style, Context7, Playwright, commit-commands, feature-dev and Swift LSP | The same enabled plugins, which hold no skills |
| codebase-memory MCP server, hooks, agent definitions and skill copies | Unchanged. Its own updater rewrites them. |
| herdr and Orca hooks | Unchanged. Their applications own them. |
| `quint-kb` MCP server | Unchanged. It runs from the Quint clone, which stays. |
| TestAnyware command, application and configuration | Unchanged installations. The skill reaches them by ordinary command execution. |

The repository moves must supply three packaging changes before the switch:

- WritingSkills offers the technical-communication hooks as a Claude Code
  plugin without a `skills/` directory. A trial must show that Claude Code runs
  the hooks and lists no native `technical-communication` skill. If Claude Code
  lists the root `SKILL.md` as a skill, repair the layout before the switch.
- PersonalSkills offers a Claude Code plugin that holds only `guardrail`.
- The new marketplace lists both plugins beside the ordinary pack plugins.
  Native users keep the full plugins, and Ravel users enable only these two.

The design rejected user hooks that point into a working copy. Such hooks run
uncommitted code, and a layout change breaks them without a report. The Pi
extension link shows that failure. A snapshot path changes with each pin, so a
hook that names a snapshot breaks at each pin advance.

### Name collisions and stale copies

After the switch, each owner-selected name has one active copy, which Ravel
supplies.

- The switch deactivates both Codex user roots' copies of the Grove snapshot
  entries together.
- The switch deactivates the stale `grove` copy and the older superpowers
  copies. Before then, one staging trial measures coexistence with a stale
  native copy.
- Guidance names such as `superpowers:brainstorming` or
  `linkuistics:decision-records` stay name requests. Ravel's loading rule finds
  them by their bare names, so no alias table exists.
- `codebase-memory`, which its tool owns, and `using-codebase-memory` have
  different names and overlapping triggers. Claude Code disables the tool copy
  through `skillOverrides`, and Codex shows both. The switch keeps both states.
- The remaining same-name pairs are platform skills in separate harnesses,
  such as `skill-creator` and `pdf`. No Ravel source holds those names.

### Resource lifetime

Ravel snapshots are immutable and retained. Each source pin has one snapshot.

Deactivation keeps every native byte. Claude Code's plugin caches stay where the
platform keeps them. Grove's Codex snapshots stay under `~/.agents/.grove`. A
moved link keeps its target.

A Claude Code session reads absolute plugin-cache paths, so disabling a plugin
leaves its reads intact. A Codex or Pi session reads through link paths, and
the switch moves those links. Each such session must end before the switch.

Claude Code can sweep cached plugin versions that no session uses. That is
platform behaviour, and the cutover neither relies on it nor prevents it. A
late rollback therefore installs from the marketplace rather than from a cache.

### Settings isolation and the staging HOME

All trials before finish run in a staging HOME. It holds its own Ravel
configuration and settings with the ten sources and the mandatory file. Its
harness configuration is a prototype of the native residue: the skill-free
plugins are enabled, and no superseded skill is installed. The harnesses sign
in through links into the real HOME, by Ravel's documented sandbox-home
technique. The staging HOME copies no credential.

The real HOME's Ravel settings gain the skill configuration only inside the
switch. By then, finish's own launch was this driver's last 1.1 settings read.
The switch also requires evidence that no other 1.1 driver can launch.

### The final session and new launches

- **Until finish launches**, the native setup does not change. Preparation may
  install disabled plugins, add a marketplace and write `config.rhai` beside
  `policy.ts`. Each of those changes leaves native discovery unchanged.
- **Finish** launches through this driver, Ravel 1.1.0 and `policy.ts` into
  Claude Code. It loads `grove-finish` and the spine from the enabled Grove
  plugin's cache path. Its lazy reads use that absolute path.
- **The switch** runs inside finish, after promotion. It disables the Grove
  plugin but keeps its cache, so finish can still read its own guidance.
- **After the switch**, a session that the owner opens directly sees only the
  native residue.
- **After this driver's reap**, a new driver resolves compatible Grove and the
  promoted Ravel 2.0.0. Each of its launches reads `config.rhai`, resolves the
  pinned sources and receives the directories.

Finish keeps its original ending command and the full 1.1.0 keg, under the
coordinated release design.

### The switch inside finish

Finish runs the prepared and rehearsed switch after promotion and the verified
Grove/Ravel pair. Its steps are:

1. List live Claude Code, Codex, Pi, Grove and Ravel processes. Include
   desktop applications and their background servers. Exclude finish, its
   descendants and the driver chain that launched it.
2. If any other listed process remains, stop and ask the owner. Do not signal
   another session.
3. Rerun the baseline instrument. Compare each entry with the prepared
   manifest. Stop for classification on any new or changed entry.
4. Write the backup: copies of the affected settings and registry files, the
   manifest and the inverse operation.
5. Apply the switch as one operation. Write the real-HOME Ravel skill
   settings. Disable the superseded plugins and enable the two skill-free
   plugins. Move each superseded file-system entry into the backup's retained
   tree at its original relative path.
6. Verify with the rehearsed probes. They resolve the sources through the
   promoted `ravel` and the real HOME's configuration. Then they launch one
   Claude Code session and one Codex session by that route.
7. On any failed probe, apply the inverse operation and verify the native
   route. Then end without a success signal.
8. Write the receipt into the maintained manual and run `mac-check`.

Each probe must observe these results:

- The mandatory file is read before the task's first action.
- A craft skill is read before the action it governs, although the task does
  not name it.
- A TestAnyware resource is read, and `testanyware doctor` runs.
- On Claude Code, the technical-communication hooks run.
- Native discovery lists no superseded skill.

The receipt records the activated default path: the Grove version, the Ravel
path and version, and the digests of `config.rhai` and the settings. It also
records each source pin, the backup location and each retained byte set with its
purge condition. The [finish decision](../adr/finish-promotes-after-the-grove-release.md)
records why this runs inside finish.

### Rollback

Inside finish, rollback is the inverse of the switch. It restores the copied
settings and registry files, enables the earlier plugins and moves each entry
back. It also restores the Ravel settings without the skill configuration. A
native probe on both harnesses verifies the result.

After finish, the owner can apply the same inverse operation while the backup
and the plugin caches exist. Otherwise, the supported return to native delivery
installs the current marketplace plugins. No rollback erases Ravel snapshots.

### Purge

Purge covers the backup's retained tree, Grove's Codex snapshot store, the
superseded Claude Code plugin registrations and the earlier marketplace
registration. Purge requires three conditions:

- No session that ran during the switch remains.
- This driver's group is reaped.
- The owner decides that the rollback window is closed.

Purge is outside this grove's success condition, as the 1.1.0 keg cleanup is.
The receipt lists each item with its condition. The maintained manual carries
the procedure. The owner uninstalls platform registrations only through the
platform's own commands. Nobody erases a cache directory by hand.

### Independent sessions

After the switch, a session that the owner opens directly with `claude`,
`codex` or `pi` sees only the native residue. It can receive the same
directories through the output of `ravel skills resolve`. The root
requirements leave automatic configuration of such sessions outside scope.
This is the main daily consequence of the owner's chosen target.

## Requirements for the Ravel directory contract

`ravel-rhai-skill-delivery-k34` must preserve or settle each item below. The
historical contract `b93bb927` already satisfies the first five items.

1. Admit at least ten sources, including Git sources with a narrow `subdir`.
   The historical bound is 16.
2. Admit a mandatory file in a pinned third-party source, with relative
   supporting reads in the same source.
3. Expose the descriptor path and the snapshot roots in the request. The
   configuration script can then grant reads to a harness that needs explicit
   directories.
4. Preserve executable bits in snapshots. The technical-communication checker
   runs from its skill directory.
5. Resolve a warmed pin with no network and no Git executable.
6. Add a loading rule for user-invoked skills. Tell the agent not to load a
   skill whose metadata sets `disable-model-invocation: true` unless the caller
   names it. `authoring-conventions` is the owner's case.
7. State exactly how Ravel 1.1 treats a settings file that holds 2.0 keys. The
   switch's timing assumes that 1.1 refuses such a file.
8. Keep the staging-HOME technique documented for actual harness trials.

## Cutover contract

`owner-ravel-skill-inventory-k38` grows the concrete leaves. Rollout orders them
with the coordinated release work.

| Order | Owner | Step | Done when |
| --- | --- | --- | --- |
| 1 | This design | Baseline, classification and contract | The baseline and this design are committed and reviewed |
| 2 | Repository moves | Move every owner-pack and selected third-party entry with its resources. Supply the three packaging changes. | Each moved skill's content and resources match the baseline or a recorded change |
| 3 | k38 | Re-inventory, compare per entry, classify each new entry | No entry stays unassigned |
| 4 | k38 leaves | Convert `policy.ts` to `config.rhai`. Draft the Ravel skill settings and the residue plan. | The drafts pass under the staging HOME |
| 5 | k38 leaves | Run the staging trials on both harnesses with the accepted feature build | Each trial below passes or carries a recorded scope decision |
| 6 | k38 leaves | Prepare the real HOME without changing discovery. Freeze automatic updates of the earlier marketplace. Rehearse the switch, the probes and the inverse in disposable layouts. | The rehearsal restores every rehearsed layout byte for byte |
| 7 | Finish | Run the switch after promotion | The probes pass and the receipt is in the manual |
| 8 | Owner | Purge retained bytes | The purge conditions hold |

The staging trials cover implicit craft discovery and the mandatory read
order. They cover a TestAnyware resource read and real VM tool use. They also
cover the Quint and codebase-memory MCP servers, the Claude Code hooks and
`/guardrail`. Each harness also runs one delegate, one compaction and one
stale-copy coexistence case. A trial fails if native discovery lists a
superseded skill.

Persistent setting changes follow the machine configuration contract. mise
tracks `~/.claude/settings.json`, `~/.codex/config.toml`, `~/.codex/hooks.json`
and `~/.config/ravel` in place. The skill directories and the plugin registry
are not tracked. The maintained manual is the Obsidian `AI Dev Tooling` note.
It still names the earlier `~/Development/grove` checkout.

## Test seams

- The baseline instrument, rerun, compares entries one by one. Its control root
  must show one unclassified entry and one dangling entry.
- Staging trials use Ravel's public `start-session` and `skills resolve` with
  the actual harnesses.
- The switch and its inverse have a dry run against the manifest. Rehearsals
  use disposable layouts. Only finish runs the switch against the real HOME.
- After the switch, the real-HOME probes, `mac-check` and the AI tooling status
  report read the result.

## Out of scope

- Automatic configuration of independently opened sessions. A native pointer
  skill that runs `ravel skills resolve` would reintroduce a native install.
  Reopen this item if the owner wants such sessions configured automatically.
- Installing Codex technical-communication hooks, which the owner never
  enabled.
- A replacement route for Pi.
- Harmonising the duplicate codebase-memory copies in Codex.
- Uninstalling disabled third-party plugins.
- Erasing platform-managed caches by hand.

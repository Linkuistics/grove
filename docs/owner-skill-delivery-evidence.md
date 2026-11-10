# Owner skill delivery evidence

The [owner specification](specs/owner-skill-delivery.md) describes the current
design. The exchange, held-copy, recovery-model and rehearsal-admission
measurements below are historical evidence for the earlier design. They do
not certify the simplified serial switch. The process and job pattern
measurements are historical evidence too. [Single-owner switch design](#single-owner-switch-design) records the initial redesign.
[Its integration](#owner-switch-simplification-integration) records the current repairs and evidence boundary.

This document records the inputs to the
[owner skill delivery design](specs/owner-skill-delivery.md). The design
session only read the machine. It changed no installation, setting, plugin
registry or file outside this checkout, and it signalled no other session.
The integration of the design's focused review repaired the instrument and
regenerated the baseline on the same day. It also only read the machine. The
section [Review corrections](#review-corrections) records what changed.

After Ravel narrowed delivery to local directories, `owner-pack-checkouts-k79`
measured the owner's checkout tool in a fixture HOME. The section
[Pack checkouts](#pack-checkouts) records that measurement as history.
`skill-marketplace-decision-k201` replaced the checkouts with Homebrew
formulae, and `candidate-keg-k238` removed the measurement's script and task.

`owner-switch-model-removal-k265` removed the Quint model of the earlier
switch's recovery, its runner and its task. The section
[The recovery model](#the-recovery-model) keeps that model's measurements as
history. The section [Owner switch model removal](#owner-switch-model-removal)
records the search for its other references.

## Method

On 2026-11-02, the session walked every skill discovery root of Claude Code,
Codex and Pi. It also read both Claude Code plugin registries and each
installed plugin's components. The walk covered user hooks, agent definitions,
MCP entries, the owner policy and the tool installations that skills need.
The repaired instrument also walks Codex's installed plugin cache. A final
sweep finds every `SKILL.md` under `~/.claude`, `~/.codex`, `~/.agents` and
`~/.pi`. Each file must lie in a walked root or a classified container.

[`evaluations/owner-skill-delivery/inventory.py`](evaluations/owner-skill-delivery/inventory.py)
is the instrument. It writes [the baseline](owner-skill-inventory.tsv), one
row for each entry. Each row names the entry's harness, location, form,
frontmatter name, digest, file count, class, source owner, capabilities,
disposition and a note. Rows carry `~` for HOME.

The digest is the entry's change identity: the first 16 hexadecimal digits of a
SHA-256 over the state that defines the entry. The instrument's docstring
defines it for each class:

| Entry class | State in the digest |
| --- | --- |
| Skill tree | Sorted relative paths, each file's executable bit and content, and each file or directory link's target |
| Claude Code plugin, Codex installed plugin | The whole version directory |
| User hook | Its matcher and definition, and each file under HOME that its command names. One row exists for each hook. |
| MCP server | Its definition. The instrument replaces env and header values, and values of keys that name a token, secret, password or key, with their names. |
| Agent definition, Pi extension, skill override | Its content |
| Tool | The resolved executable, the bundle's `Info.plist` or the clone's revision and changed paths |
| Coverage row | The sorted `SKILL.md` paths of its container |

A rotated credential therefore does not change an MCP row. A digest holds no
raw value. A row that the instrument cannot observe says `NEEDS-REVIEW`.
Built-in Claude Code skills have no files. Their digest says `unverified`,
and a session's own skill listing is their only evidence. This session's
listing holds the same 18 built-in names.

| Artifact | SHA-256 |
| --- | --- |
| `docs/owner-skill-inventory.tsv` | `cbfac5b83548a3f434f34d2ac4e15afcc4004ea6b212a0beabff58b3de58a0d0` |
| `docs/evaluations/owner-skill-delivery/inventory.py` | `f3c2a86e838594c922b4e20622ff8b60e592cb200d21b3dc65b96ad7d1c43726` |
| `docs/evaluations/owner-skill-delivery/controls.py` | `6a63437b87d457a22a3ae11039393343806550f6abdbf0cd8715d7d63137418d` |

Two runs of the saved instrument produced byte-identical output, and the
committed baseline is that output. The measurement digested the instrument, the
controls script and both Claude Code settings and registry files before and
after the runs. It also digested both Codex configuration files and the
`mcpServers` object of `~/.claude.json`. No digest changed.

The instrument and the controls script of this table produced the baseline.
`owner-inventory-reader-marker-k244` later changed both of them. The section
"Reader markers in the plugin cache" records their current values and how a
comparison reads the baseline's Claude Code plugin rows.

### Controls

A clean read from a broken instrument looks the same as a clean read from a
working one. Four controls separate the two cases:

- **Mutation control.** A scratch root held an unknown skill directory and a
  dangling link. The instrument reported one `UNCLASSIFIED` row and one
  dangling row for that root.
- **Clean run.** Without the scratch root, no row said `UNCLASSIFIED`,
  `UNASSIGNED`, `UNCOVERED` or `NEEDS-REVIEW`.
- **Positive control.** The instrument found `using-testanyware` in all four
  places that hold it: the Claude Code plugin, both Codex user roots and the
  dangling Pi link.
- **Change controls.** [`controls.py`](evaluations/owner-skill-delivery/controls.py)
  builds a fixture HOME and applies one change in each case. The repaired
  instrument passed all 14 cases. They cover a changed hook command or script
  and a second hook with the same event and owner. They cover a changed MCP
  command or arguments with the same name. They also cover a lost executable bit, a
  retargeted directory link and changed agent, skill-override and
  remote-installed plugin content. The last two cases are a skill in an unknown
  Codex marketplace and a skill outside every walked root. A repeated run and
  a rotated MCP credential left the output unchanged.
- **Failing revision.** The reviewed instrument, SHA-256 `6138bacf…9792`,
  passed only the two unchanged cases. For each of the other 12 cases, its
  whole output stayed byte-identical, so its comparison gate would accept the
  change.

`task owner:inventory` prints the inventory, and `task owner:inventory:controls`
runs the change controls.

## Inputs

| Subject | Observation |
| --- | --- |
| Harnesses | Claude Code 2.1.316 and Codex CLI 0.166.0. Pi's settings record changelog version 0.84.2. |
| Grove and Ravel | Installed Grove 22.8.0 and Ravel 1.1.0. The driver's ending executable is `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. |
| Linkuistics plugins | Installed version `238c3a839ab4`, the marketplace clone's commit `238c3a839ab4b30b101fbe3cc5212d66091df28a`. The marketplace follows `Linkuistics/grove` with automatic updates. |
| Superpowers | Plugin 7.0.0 at `bb92a77741419a4ab5f06e711a283343f1ada0c3`, from `https://github.com/obra/superpowers.git`. Fifteen skills and one `SessionStart` hook. |
| Quint | Plugin 1.0.0 at `cc75369f741af7d490936f82002c2d28e3b3d78d`. The clone `~/.quint-llm-kit/repo` is at the same commit, with a modified `package-lock.json`. `quint` 0.33.1. |
| TestAnyware | Homebrew `linkuistics/taps/testanyware` 2.7.0 and `/Applications/TestAnyware.app` 2.7.0. `testanyware doctor --json` passed its installation, configuration, directory and local-network checks. Its login-item check failed, which does not affect a skill read. |
| codebase-memory | `codebase-memory-mcp` 0.10.6. Its installer wrote skill copies, agent definitions, hooks and MCP entries in both harnesses. |
| mise | 2026.10.25. The owner's configuration has no `[bootstrap.repos]` entry. `mac-check` runs `mise bootstrap plan --detailed-exitcode`. |
| Remotes | On 2026-11-02, `gh repo view` reported `Linkuistics/grove`, `Linkuistics/TestAnyware`, `obra/superpowers` and `quint-co/quint-llm-kit` as public. The pack repositories that the moves create do not exist yet. |
| Owner policy | `~/.config/ravel/policy.ts` routes each kind to `claude` or `codex`. `finish` goes to Claude Code with `claude-opus-5-5`. No route selects Pi. |
| Ravel settings | `~/.config/ravel` holds only `policy.ts`. No `settings.json` exists. |
| Ravel contracts | The local-source contract `docs/specs/skill-discovery.md` at `7a894c78`, on the unpushed bookmark `skill-directory-delivery`. The historical directory contract at `b93bb927`. Ravel main's `docs/specs/configuration-evaluation.md` at `24a54523`. The sandbox-home recipe in `docs/specs/configuration-evaluation.md` and `scripts/harness-smoke.sh` at `2a77ffbda3b2510f0997c740c0642b18afaeece7` in `Ravel.coordinated-release-design`. |
| Codex home | The Codex 0.166.0 executable reads `CODEX_HOME` and canonicalises it. It names `.codex/config.toml`, `.codex/agents`, `.codex/hooks` and `.agents/skills` as user paths, and `auth.json` as its file sign-in store. The owner policy passes `-c` overrides only and sets no `CODEX_HOME`. |
| Claude Code plugin cache | The [plugin loading reference](https://code.claude.com/docs/en/plugins/loading#cleanup-of-previous-versions), read on 2026-11-02. An update or uninstall writes `.orphaned_at` into the earlier version directory. Cleanup can remove it after 14 days. The [executable evidence](#finding-for-the-owner-design) adds the live-reader check for 2.1.319. A running session keeps the plugin set that it loaded until `/reload-plugins` or a new session. |
| Claude Code configuration directory | The [environment reference](https://code.claude.com/docs/en/env-vars) and the [credential management section](https://code.claude.com/docs/en/authentication#credential-management), read on 2026-11-02 by `ravel-rhai-skill-delivery-k50`. `CLAUDE_CONFIG_DIR` replaces `~/.claude` for settings, history and plugins. It also moves the sign-in file and keys the macOS Keychain entry to that directory. Three `CLAUDE_CODE_PLUGIN_*` variables add plugin locations. No trial measured these statements. Ravel's sandbox recipe at `7a894c78` also names `CLAUDE_SECURESTORAGE_CONFIG_DIR`, which moves only the sign-in store. |

Ravel main keeps settings at `$HOME/.config/ravel/settings.json` for 2.0.0.
Its keys are `timeoutMs`, `contextBytes`, `configEnv`, `harnesses` and
`budgets`, and an unknown key refuses. The historical directory contract adds a
`skills` key and states that a 1.1 launch refuses shared settings that hold it.

## Discovery roots

| Harness | Root | Content at the baseline |
| --- | --- | --- |
| Claude Code | Plugin cache under `~/.claude/plugins/cache` | Enabled and disabled marketplace plugins, each at an immutable versioned path |
| Claude Code | `~/.claude/skills` | The tool's `codebase-memory` copy, a stale `grove` copy, account-synced skills and a dangling `terminal-browser` link |
| Claude Code | Built in | Skills that the harness supplies. The session's own skill listing is the only evidence for them. |
| Codex | `~/.codex/skills` | Grove-snapshot links, older superpowers copies, the tool's `codebase-memory` copy, a dangling `terminal-browser` link and system skills |
| Codex | `~/.agents/skills` | The same Grove-snapshot links, Quint links, the tool's `codebase-memory` copy and a dangling `terminal-browser` link |
| Codex | Installed plugin cache `~/.codex/plugins/cache` | Six `openai-bundled`, five `openai-primary-runtime` and six `openai-curated-remote` plugins. `config.toml` enables five bundled plugins and disables the other six local ones. The remote plugins have a remote install record and no `config.toml` entry. |
| Codex | Bundled and runtime marketplace sources | The catalogs that the bundled and runtime plugins install from |
| Pi | `~/.pi/agent/skills` and `extensions` | Links to `~/Development/grove`, which no longer exists, and tool extensions |

The Codex executable names the roots `codex-home-skills`, `user-agent-skills`,
`system-skills` and a plugin cache. Codex therefore reads both user roots. Its
handling of one name in both roots was not measured. The review session
observed that Codex exposes the remote `pages` plugin's `write-page` skill. The
remote plugins hold 30 skill files, 20 of them in `openai-templates`.

The coverage sweep classifies each other area that holds `SKILL.md` files.
Their layout identifies them as install sources or retained versions, not
discovery roots:

| Container | Content |
| --- | --- |
| `~/.claude/plugins/marketplaces` | Claude Code marketplace clones |
| Inactive versions in `~/.claude/plugins/cache` | Earlier Linkuistics versions and three uninstalled official plugins, each with an orphan marker |
| `~/.agents/.grove/snapshots` | Grove 22.8 snapshot store, which the Codex links target |
| `~/.codex/.tmp/plugins` | The `openai-curated` marketplace clone |
| `~/.codex/vendor_imports` | Codex's curated skill catalog |
| `~/.codex/plugins/.remote-plugin-install-staging` | One remote-install staging bundle |

## Baseline summary

The baseline has 444 entry rows.

| Class | Rows |
| --- | ---: |
| Owner pack | 137 |
| Platform-managed | 116 |
| Tool-managed | 53 |
| Dangling | 48 |
| Disabled third-party | 26 |
| Selected third-party pack | 24 |
| Stale copy | 15 |
| Capability-only | 14 |
| Not a discovery root | 11 |

No platform-managed or tool-managed skill has the name of a Ravel-delivered
skill.

These defects exist before any move:

- Each Pi skill link and the Pi technical-communication extension link dangle.
  They point at `~/Development/grove`, which the earlier checkout move left
  behind.
- `terminal-browser` links dangle in `~/.claude/skills`, `~/.codex/skills` and
  `~/.agents/skills`. The cask is absent from Homebrew.
- Both Codex user roots hold links to the same Grove snapshot entries.
- `~/.codex/skills` holds superpowers copies without provenance. Ten of them
  differ from superpowers 7.0.0, and four match it.
- `~/.claude/skills/grove` is an earlier Grove copy with retired signal files.
  Its name collides with the plugin's `grove` skill.
- Codex's snapshot copies of `technical-communication` and
  `machine-environment` differ from the Claude Code plugin's bytes.
- `guardrail` has no Codex link, as its `harnesses: [claude-code]` metadata
  directs.

## Capabilities beside the guidance

- The `linkuistics` Claude Code plugin carries the technical-communication
  hooks for six events. The same skill holds a Codex hooks plugin, but the
  owner's Codex configuration names no marketplace that offers it. Codex
  sessions therefore run without that always-on layer.
- `guardrail` installs a `PreToolUse` hook from its frontmatter and responds
  to `/guardrail`. Both need native Claude Code discovery.
- `authoring-conventions` and `guardrail` set `disable-model-invocation: true`.
- The superpowers `SessionStart` hook injects `using-superpowers` into each
  Claude Code session.
- Claude Code reads MCP entries for `quint-kb` and `codebase-memory-mcp`.
  Codex reads `node_repl`, `context7`, `openaiDeveloperDocs`, `quint-kb`,
  `computer-use` and `codebase-memory-mcp`.
- User hooks in both harnesses belong to Orca, herdr and codebase-memory-mcp.
  Each hook script names its installer. Claude Code has 21 user hooks and
  Codex has 12.
- `skillOverrides` in `~/.claude/settings.json` turns off the tool's
  `codebase-memory` skill.

## Machine contract

mise tracks `~/.claude/settings.json`, `~/.claude/hooks`, `~/.codex/config.toml`,
`~/.codex/hooks.json`, `~/.codex/AGENTS.md` and `~/.config/ravel` in place. It
does not track the skill roots, the plugin registries or `~/.pi`. The
maintained manual for this area is the Obsidian `Configuration/AI Dev Tooling`
note and its maintenance script. That note still names `~/Development/grove` as
the live Grove source.

## Pack checkouts

This section is history. The owner design now installs each source through
a Homebrew formula, as its section "Skill formulae" states.
`candidate-keg-k238` removed the script and its task `owner:checkouts`. Grove
commit `7b113a28a96621217c16154d4f44eba667236132` still holds both.

The design gave the pack checkouts to the `[bootstrap.repos]` part of mise.
The [mise repository reference](https://mise.jdx.dev/bootstrap/repos.html),
read on 2026-11-02, states these facts:

- Each entry is keyed by its checkout path, with a required `url` and an
  optional `ref`. A `ref` is a branch, a tag or a full commit ID.
- `filter`, `depth` and `submodules` apply only to the first clone. `depth`
  cannot be combined with a full commit ID.
- A branch or tag `ref` makes the status command run `git ls-remote`. A commit
  `ref` is examined locally.
- mise never removes a checkout. `unapply` leaves the directories.
- The reference does not mention jj.

The script `docs/evaluations/owner-skill-delivery/checkouts.py` measured the
behaviour that the design then stated. It built a fixture origin and a
fixture HOME, and it ran the installed mise with only isolated paths in its
environment. It moved the origin away to stand in for a machine without a
network. Each mise call named the fixture HOME with `-C`. The fixture hid the
owner's global and system Git configuration. `task owner:checkouts` ran it.

| Artifact | SHA-256 |
| --- | --- |
| `docs/evaluations/owner-skill-delivery/checkouts.py` | `349e2e233c3967f370c39174034a019cf84f2980758f4098c2827bc819cd3244` |

With mise 2026.10.25 and Git 2.56.0, all 33 cases matched. The script asserted
each observation in this table:

| Case | Observation |
| --- | --- |
| No checkout | The state is `missing`, and `status --missing` exits 1. `bootstrap plan --detailed-exitcode` exits 0 |
| `apply` with a commit `ref` | Clones the repository and checks out that commit with a detached HEAD. The state is `current`, and `status --missing` exits 0 |
| No network, `ref` unchanged | The state stays `current`. `apply` exits 0, and its verbose output shows no fetch |
| No network, `ref` moved to a commit that the clone holds | The state is `differs`, and `status --missing` exits 1. `apply` fetches first, exits 1 and leaves the checkout at its earlier commit |
| No network, `ref` moved to a commit that the clone lacks | The same result |
| Network again | `apply` moves the checkout to the new commit |
| Untracked file | The state is `dirty`. `status --missing` and `apply` exit 1 |
| File that `.gitignore` names | The state stays `current`, and `status --missing` exits 0. `git status --porcelain --ignored` lists the file |
| Another origin URL | The state is `conflict`. `status --missing` and `apply` exit 1 |
| No `ref` | `apply` clones the default branch. After the remote moves ahead, the state stays `current`. `update` moves the checkout forward |

A manual verbose run named the fetch command `git fetch --prune --tags origin`.

The `--mutate` run expects `apply` to succeed for the offline move to a held
commit. That run failed exactly that case, which shows that the comparison can
fail. The script digest was the same before and after both runs. Each run
removed its temporary directory.

Two further observations used the same fixture without the script:

- In a checkout that holds a colocated jj repository, the state stayed
  `current` and `apply` succeeded.
- The fixture configuration held two entries and a `bootstrap` task. One
  checkout had an untracked file, and the other was missing.
  `mise -C <home> bootstrap --only repos,task` exited 1 and named the local
  changes. It cloned neither checkout and ran no task. After the file was
  removed, the same run cloned the missing checkout and ran the task.

The `plan` case shows that the plan that `mac-check` runs cannot report a
missing checkout.

### Credential routes of a clone

On 2026-11-02, read-only `git config --show-origin --get-all credential.helper`
queries tested the clone environment of the staging trials. They used
Homebrew Git 2.56.0 and Apple Git 2.54.0, with an empty HOME and decoy files in
the scratch directory:

| Environment | Helper that Git reports |
| --- | --- |
| No `GIT_CONFIG_NOSYSTEM` | `osxkeychain`, from `/opt/homebrew/etc/gitconfig` or the Xcode `gitconfig` |
| `GIT_CONFIG_NOSYSTEM=1` and `GIT_TERMINAL_PROMPT=0`, with an inherited `XDG_CONFIG_HOME` | The decoy from `$XDG_CONFIG_HOME/git/config` |
| The same, with an inherited `GIT_CONFIG_GLOBAL` | The decoy from that file |
| The same, with an inherited `GIT_CONFIG_COUNT` | The decoy, with the origin `command line` |
| The allowlist of the fixture, with `GIT_CONFIG_PARAMETERS` added | The decoy, with the origin `command line` |
| The allowlist of the fixture, with `GIT_CONFIG_SYSTEM` added | None, because `GIT_CONFIG_NOSYSTEM=1` blocks it |
| The allowlist without `GIT_CONFIG_NOSYSTEM`, with `GIT_CONFIG_SYSTEM` added | The decoy from that file |
| The allowlist of the fixture | None, with exit 1 |

Both builds gave the same results. The owner's global configuration is
`~/.config/git/config`, on the XDG path, and names Git Credential Manager. The
current environment sets neither `XDG_CONFIG_HOME` nor `GIT_CONFIG_GLOBAL`.
These queries read configuration only. They show which helper Git would call,
not a call. The decoy-helper control of the staging trials observes the call.

### The released commit of `grove-core`

The v22.8.0 release commit `bacb255aba8` changed only `CHANGELOG.md`,
`Cargo.lock` and `Cargo.toml`. No release file is in a skill directory. The
tree ID of the `grove-core` source directory therefore changes between the
candidate and the released commit only if the integration of `main` changes
that directory.

### Checkout configuration and the tree ID

On 2026-11-02, `owner-pack-checkouts-k84` built a scratch repository with Git
2.56.0. Its `skills` directory held a text `SKILL.md`, a relative link and an
executable script. Two clones ran with the isolated configuration of the
fixture. The second clone added `core.autocrlf=true` and
`core.symlinks=false`:

| Clone | Tree ID of `skills` | `git status --porcelain --ignored` | Written files | Ravel `resourceDigest` |
| --- | --- | --- | --- | --- |
| Plain | `3a94b4217e46` | Empty | LF line ends, the link is a link | `f7a0ae858b65` |
| Converted | `3a94b4217e46` | Empty | CRLF line ends, the link is a regular file | `4eca6b6d32f7` |
| Plain, at another path | `3a94b4217e46` | Empty | As the first clone | `f7a0ae858b65` |

The digests came from `ravel skills resolve --json` of the local feature build
in `Ravel.skill-directory-delivery`, with a scratch HOME and an empty
environment. The build reports `ravel 1.1.0`. The first digits of its SHA-256
are `68297a769cd9`. The tree ID and the status therefore cannot tell the two
clones apart. The digest can, and it does not depend on the checkout path.

A read-only query of the owner's effective Git configuration found no
`core.autocrlf`, `core.eol`, `core.symlinks`, `core.attributesFile` or
`core.sparseCheckout`. The global file `~/.config/git/config` sets the LFS
filters. Today's owner checkouts are therefore not known to differ. The
design's check needs the digest, because the configuration can change.

### Publication by atomic exchange

Scope: owner-switch-contraction-k270 removed the program's only use of
`RENAME_SWAP`, the exchange. The program still uses `RENAME_EXCL`. The
measurement stays as history.

On 2026-11-02, `owner-pack-checkouts-k86` called `renamex_np` through Python
on macOS 27.0.1, on an APFS volume, in a scratch directory:

| Call | Result |
| --- | --- |
| `RENAME_SWAP` of a live file and a prepared file | Exit 0. The path held the new content. The other name held the replaced content, with its original inode. |
| `RENAME_EXCL` onto an existing file | Refused with `EEXIST`. Neither file changed. |
| `RENAME_EXCL` onto an absent name | Exit 0. The file was created. |

One call can therefore publish a file of the switch set and keep the content
that it replaced. The section "Concurrent writers" of the design uses both
calls. The measurement used no file of the owner's HOME.

On the same day, `owner-switch-recovery-k87` repeated the setup with these
calls:

| Call | Result |
| --- | --- |
| A write through a descriptor that a writer opened before `RENAME_SWAP` | The write landed under the displaced name. The path did not receive it. |
| `RENAME_SWAP` of files with different modes and extended attributes | The path took the prepared file's mode and attributes. The displaced name kept the old ones. |
| `RENAME_EXCL` of a directory and of a dangling link into another directory | Exit 0 for each. The link kept its target. |
| `RENAME_EXCL` of a directory onto an existing directory | Refused with `EEXIST`. |
| `RENAME_SWAP` of two directories | Exit 0. Each name held the other directory. |

A late write therefore reaches a kept copy, and the design flags that copy.
The stage file must take the live file's metadata before the exchange.

### The recovery model

This section is historical. Its runs checked the recovery of the earlier
switch, with cells, exchanges, outside writes and partial continuations. The
simplified design removes those mechanisms, so these runs are not acceptance
evidence for the current switch.
`owner-switch-model-removal-k265` removed the model, its runner and the task
`model:owner-switch`. The repository's history keeps their last versions. Their
digests are the same as in the last table of this section.

`owner-switch-recovery-k87` wrote the Quint model `docs/models/owner-switch.qnt`
and its runner `docs/models/run-owner-switch.sh`. Its run of 2026-11-02 had
the model digest `793ef9ef86a4`.

`owner-switch-recovery-k89` then changed the model for the findings of
`owner-switch-recovery-k88`. Outside programs and late descriptors now remove
keys, and a merged copy keeps only its merge digest as recorded content. One
layout flag stands for a changed entry outside the switch set, and step 14
compares it before the manual copy. The model has a fifth property and two
more mutants.

Three instrument changes came with it:

- A step picks a kept copy only from the copies that exist. Before, it picked
  an index from 0 to 40, and no merge happened while outside budget remained.
- The outside phase `idle` and the crash phase `receipt` serve two focused
  worlds. They hunt witnesses only.
- Two witnesses ignore a key that a later removal superseded. Without that,
  a legitimate removal would read as a loss.

The sampling therefore differs from k87's run, and no witness count compares
across the two runs.

On 2026-11-02, `task model:owner-switch` ran the changed files with Quint
0.33.1. The three subjects had the same digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `docs/models/owner-switch.qnt` | `4b3002d9d23d0ecd59202a7ed04e532c90dfd384d45d8055724305bea6b39840` |
| `docs/models/run-owner-switch.sh` | `66c9fcfb6065e602ba5e523f86eabc37937c226543e425f4896a214973c7596d` |
| `Taskfile.yml` | `2aec4fba957f23f586498606734917daa4c12410dedc788010f3023b9c65b69a` |

Each world ran 4,000 samples of up to 60 steps. The classification world ran
12,000, and the unchecked-receipt mutant ran 20,000. All ten invariants held
in each of the nine design worlds. Each mutant witness stayed unreached
there. These are the witness counts:

| World | Witness | Traces |
| --- | --- | --- |
| `crashes` | A complete switch, each after a crash | 1,778 |
| `crashes` | An interruption resumed to native | 2,962 |
| `crashes` | A failed probe inverted to native | 536 |
| `crashes` | The owner's inverse to native | 2,135 |
| `writers` | A complete switch with no crash | 1,585 |
| `writers` | An exchange conflict in a flagged copy | 92 |
| `writers` | A late write in a flagged copy | 113 |
| `writers` | A merged copy, then a complete switch | 96 |
| `writers` | A removal displaced into a flagged copy | 18 |
| `classify_world` | A second plan switched to complete | 6,176 |
| `coincidence` | A resumed inverse resets an outside switched value | 187 |
| `after_receipt` | Drift after a receipt | 1,859 |
| `during_run` | A conflict at a cell | 58 |
| `merge_return` | A merged copy returned to the content that the run read, and flagged | 48 |
| `entries` | The complete continuation copied its receipt | 523 |
| `entries` | The complete continuation found an outside entry, inverted, and stopped for classification | 448 |

Each mutant reached its violation:

| Mutant | Traces |
| --- | --- |
| No recorded digest of the displaced content | 175 of 4,000 |
| No run record in the state | 1,327 of 4,000 |
| "Inverse ended" before the inverse | 4,000 of 4,000 |
| No cell reading before the receipt | 155 of 20,000 |
| No rehearsal check | 1,234 of 4,000 |
| An inverse that resets a third value | 91 of 4,000 |
| No merge authority over a merged copy | 44 of 4,000 |
| No comparison in step 14 | 61 of 4,000 |

The sweep of every invariant against every mutant found violations only of
the properties that the table in the design names. The mutant without a run
record also broke the state table and the settled-state property, because it
reads states by another rule.

The results are sampled, not proved. The shortest path to a receipt has 15
steps, and the manual copy is the 16th, inside the 60-step bound.

#### The preludes in step 3

`native-settings-recovery-k221` then changed the model for findings F1 and F2
of `homebrew-delivery-design-k219`. Each run now enters through steps 1 to 3.
The gate skips the preludes while a run is open, and the two prelude cells
can be absent. The native settings write a settings record before their
exchange. The model has a sixth property, four more mutants and four more
design worlds.

Four instrument changes came with it:

- Each instance states where its first state lies: before the first entry's
  preludes, or where they leave the layout.
- The stop at the table acts only where no row continues.
- The outside phase `run` excludes the preludes again. The outside phase
  `prelude` and the crash phase `probe` are new.
- `wit_driftedDuringRun` stays only in `during_run`, and `both_preludes`
  carries the exchange conflict of `both`.

The sampling therefore differs from the run of 2026-11-02, and no witness count
compares across the two runs.

On 2026-11-06, `task model:owner-switch` ran the changed files with Quint
0.33.1. The three subjects had the same digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `docs/models/owner-switch.qnt` | `7df7fc51b9b81ca889e50d77f59c8aed370553c0b91f584910cf062f99c5974c` |
| `docs/models/run-owner-switch.sh` | `ff25f882894444b2f2d93438516ab501a1aeb766b4c6d1834d65f19840d17f3e` |
| `Taskfile.yml` | `05c3080b84d0443eabe075e83a4c01ce3d60a27e2ec4409e1e1ec93ee9d386f1` |

Each world ran 4,000 samples of up to 60 steps. The classification world and
the mutant that resets a third value ran 12,000, and the unchecked-receipt
mutant ran 20,000. All twelve invariants held in each of the thirteen design
worlds. Each mutant witness stayed unreached there. These are the counts of
the new and moved witnesses:

| World | Witness | Traces |
| --- | --- | --- |
| `native_settings` | A write before the native settings' exchange in a flagged copy of a settings record | 665 |
| `native_settings` | A late descriptor write in that copy, flagged | 1,134 |
| `native_settings` | A crash between that exchange and its comparison, with the copy flagged | 315 |
| `native_settings` | A merged copy, then a complete switch | 1,258 |
| `active_entry` | A crash while the probes ran, then an entry that resumed and completed | 1,476 |
| `prelude_crashes` | A crash in the native settings, then the native state at the table | 3,725 |
| `prelude_crashes` | A crash before the hooks installation, then the plugin installed and the native state | 561 |
| `prelude_crashes` | An interrupted installation disabled by the next entry | 299 |
| `entries` | The complete continuation through the gate copied its receipt | 534 |
| `both_preludes` | An exchange conflict in a flagged copy | 34 |
| `during_run` | Drift during a run | 2,681 |

Each mutant reached its violation:

| Mutant | Traces |
| --- | --- |
| No recorded digest of the displaced content | 940 of 4,000 |
| No run record in the state | 445 of 4,000 |
| "Inverse ended" before the inverse | 3,966 of 4,000 |
| No cell reading before the receipt | 161 of 20,000 |
| No rehearsal check | 503 of 4,000 |
| An inverse that resets a third value | 28 of 12,000 |
| No merge authority over a merged copy | 46 of 4,000 |
| No comparison in step 14 | 69 of 4,000 |
| No gate of the preludes | 1,996 of 4,000 |
| No settings record | 981 of 4,000 |
| No hooks installation after the first entry | 3,756 of 4,000 |
| No disabling of an enabled plugin | 117 of 4,000 |

The sweep of every invariant against every mutant found violations only of
the properties that the table in the design names, with two known extras. The
mutant without a run record also broke the state table and the settled-state
property. The mutant that writes "inverse ended" early also broke property 6.
It reads an open run as closed, so a prelude meets a value that its own write
left.

The rarest witness was a removal displaced into a flagged copy, in 12 of
4,000 traces of `writers`. From a first state before the preludes, the
shortest path to a receipt has 21 steps.

#### The held link of the hooks installation

`hooks-installation-writes-k222` then changed the model for Claude Code's own
writes. The hooks installation's command now reads the file and later renames
a new file onto the path, under a held link. The disabling is an exchange
after a settings record. An outside write to the file of part `HOOKS` lands
in place or by a rename.

The model has a second accepted case, one more mutant and one more design
world. That world, `hooks_commands`, runs 12,000 samples. The added steps change the sampling, so no witness count compares
with the run above.

On 2026-11-06, `task model:owner-switch` ran the changed files with Quint
0.33.1 in 6 minutes 8 seconds. The three subjects had the same digests before
and after the run:

| Subject | SHA-256 |
| --- | --- |
| `docs/models/owner-switch.qnt` | `a1fea03e76cebdec7cd115e002c851a128c9efc566195e0eca250e301331cb69` |
| `docs/models/run-owner-switch.sh` | `0fe7ef316e63ce1438b8d4a3193af35e09f6c2e9d99b699b1f9aa4ff0c71c2e1` |
| `Taskfile.yml` | `05c3080b84d0443eabe075e83a4c01ce3d60a27e2ec4409e1e1ec93ee9d386f1` |

All twelve invariants held in each of the fourteen design worlds. Each mutant
witness stayed unreached there, the new one included. The runner printed 241
passes and no failure. These are the counts of the new witnesses:

| World | Witness | Traces |
| --- | --- | --- |
| `hooks_commands` | A write in place inside a command's interval, in a flagged held copy | 32 of 12,000 |
| `hooks_commands` | A late descriptor write in a held copy, flagged | 42 of 12,000 |
| `hooks_commands` | The accepted case: a write lost after another program's rename during a command | 34 of 12,000 |
| `hooks_commands` | A crash between a held link and the rename, then the native state after the link's erasure | 415 of 12,000 |

The mutant without the held link reached its violation in 13 of 4,000 traces.
The sweep found that it violates only property 1. Each other mutant violated
the same properties as in the run above.

The rarest witness was that mutant's. The rarest design witness was a removal
displaced into a flagged copy, in 21 of 4,000 traces of `writers`. From a
first state before the preludes, the shortest path to a receipt now has 26
steps.

#### The comment of the review's integration

`native-settings-recovery-k224` changed one comment of the model, in its
list of what the model leaves to prose. No definition changed. On
2026-11-06, `task model:owner-switch` ran the changed file with Quint 0.33.1
in 6 minutes 5 seconds. The runner printed 241 passes and no failure. The
three subjects had the same digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `docs/models/owner-switch.qnt` | `c90bbf9ab658518753630d8cd9226cd926f0ac1eaf6e70c5f200c934e60412c4` |
| `docs/models/run-owner-switch.sh` | `0fe7ef316e63ce1438b8d4a3193af35e09f6c2e9d99b699b1f9aa4ff0c71c2e1` |
| `Taskfile.yml` | `05c3080b84d0443eabe075e83a4c01ce3d60a27e2ec4409e1e1ec93ee9d386f1` |

## Not measured

- Claude Code's treatment of a root-level `SKILL.md` inside a skill-free
  plugin. staging-claude-native-trial-k285 measured it with Claude Code
  2.1.322 in
  [Claude Code native residue in staging](#claude-code-native-residue-in-staging).
  The listing held no skill of the plugin, whose manifest sets
  `"skills": []`.
- The effect of disabling a plugin on a Claude Code session that is already
  running. staging-guidance-probe-k278 measured it with Claude Code 2.1.319
  in [The guidance probe case](#the-guidance-probe-case). The session kept
  its loaded set, and the disabled version directory gained no orphan marker.
- Codex sign-in from a separate `CODEX_HOME` that reuses only the
  authentication file, including a token refresh. owner-staging-setup-k248
  measured it in
  [Staging setup and isolation trial](#staging-setup-and-isolation-trial).
- Which tree Codex reads for an enabled bundled plugin: the installed cache or
  the marketplace source. The baseline records both.
- Whether Claude Code under the owner policy reads snapshot paths outside its
  working directory without a permission prompt. The staging discovery trials
  did not measure it. Each Claude Code trial ran with
  `--permission-mode bypassPermissions`. The owner's policy passes no
  permission flag, and the owner's settings select the `auto` mode.
  `trials/discovery-claude-no-add-dir-1` showed only that a read of a
  snapshot needs no `--add-dir` in the bypass mode.
- Codex's handling of one skill name in both user roots. The stale-copy
  case measured project roots, not user roots: Codex 0.167.0 lists
  `.codex/skills` and `.agents/skills` of its working directory or Git root,
  and not `.claude/skills`.
- Pi's startup with a dangling extension link.
- Built-in Claude Code skills on disk. The baseline lists them from the
  session's skill listing.
- `mise bootstrap repos apply` against the real remotes.
- A decoy credential helper against a remote that asks for a credential.
- The size and capture time of a TestAnyware checkout subdirectory.
  Homebrew replaced the checkout with a keg. staging-testanyware-trial-k287
  measured the keg's source in
  [The size and capture time of the source](#the-size-and-capture-time-of-the-source):
  two files of 5,952 bytes, resolved in 32 to 37 ms.
- Ravel's capture of a pack checkout that mise wrote.

The staging trials in the design's cutover contract own each of these
measurements before the switch. Of these items, the staging discovery trials
of owner-staging-discovery-k249 measured only the plugin disable. The native
residue trial of staging-claude-native-trial-k285 measured the root-level
`SKILL.md`. The TestAnyware trial of staging-testanyware-trial-k287
measured the size and capture time of its source.

The rehearsal of order 6 owns two more:

- Ravel 1.1 and Ravel 2.0 with a settings file that holds no key. The
  inverse leaves that file when the switch created it.
- An exchange and an exclusive rename between each live path of the switch
  set and the backup. Both calls need one volume.

Two measurements need the real HOME or a new machine, so no staging trial can
own them:

- A complete `mise bootstrap` run with repository entries. Packages and
  launchd agents act for the whole machine or account, not for one HOME.
- An adoption with repository entries on a new machine. The new-machine
  runbook in the maintained manual owns it.

[Dispositions of "Not measured"](#dispositions-of-not-measured) in the
staging acceptance gives each item above its measurement, its owner or the
reason that a later decision withdrew it.

## Review corrections

Scope: each correction below states the design at the time of its review.
owner-switch-simplification-k260 replaced the continuation, the inverse and
the pending-activation record with explicit restore, and
owner-switch-contraction-k270 removed them from the program. The current
rules are in the specification.

The focused review `owner-ravel-skill-delivery-k41` found three instrument
defects. Its integration `owner-ravel-skill-delivery-k48` repaired them and
regenerated the baseline.

- **Omitted Codex plugin cache.** The reviewed instrument read the Codex
  marketplace sources but not `~/.codex/plugins/cache`. The repair walks every
  marketplace, plugin and cached version there. A plugin in an unknown
  marketplace is `UNCLASSIFIED`. The coverage sweep reports any other
  `SKILL.md` as `UNCOVERED`.
- **Hidden hooks and capabilities.** The reviewed instrument kept one hook row
  for each event and guessed owner, without a digest. It hid three
  codebase-memory-mcp `SessionStart` hooks in Claude Code. MCP and agent rows
  held names only, tool rows held static text, and the tree digest left out
  executable bits and directory links. The repair gives every class the change
  identity above.
- **Static tool literals.** Tool rows now come from the installed paths,
  `Info.plist` and the Quint clone's revision. The codebase-memory-mcp binary
  has no versioned path, so its digest is its identity.

Per-entry comparison with the reviewed baseline found no added, removed or
reclassified skill row outside the classes above. The 30 earlier hook rows
became 33 rows with one hook each. The new rows are 54 installed Codex plugin
rows, one skill-override row and 11 coverage rows.

Every digest changed, because its definition changed. No class or disposition
changed. Other column
changes are the `orphan=no` mark and a file count on 26 Claude Code plugin rows,
and the renamed form of 15 marketplace-source rows. Six agent rows and four
tool rows gained a file count. Five tool rows and 14 capability notes have
fresh text.

The focused review `owner-pack-checkouts-k81` found one P1 and five P2 defects
in the pack-checkout revision. Its integration `owner-pack-checkouts-k82`
repaired the specification and its views:

- **An advance rule that excluded the switch.** The rule now forbids a capture
  of a changing checkout, not a running launch. `ravel skills resolve` is also
  a capture. A Ravel 1.1 launch and settings without a `skills` key capture
  nothing.
- **A weak clone environment.** The specification now gives the allowlist of
  the fixture and requires a decoy-helper control. The table under
  [Credential routes of a clone](#credential-routes-of-a-clone) records the
  routes.
- **No continuation after a failed apply.** Every stop before the native
  change keeps native delivery working. Finish writes a pending-activation
  record and runs the switch again. The owner can run it again after finish,
  with no release, promotion or teardown.
- **A core pin chosen after its verification.** Staging records the tree ID of
  the accepted `grove-core` candidate. The switch compares it with the released
  commit before any change. A probe and a staging trial read a Grove kind
  skill.
- **Clones after the isolation trial.** The credential control and the clones
  now precede the first harness launch.
- **A root-brief handoff that named a tool.** The integration leaf holds the
  repaired text for the replanning leaf.

One in-session reviewer then read the repaired text and found nine defects.
The integration repaired each of them:

- The `grove-core` candidate is the last core change of the line, and rollout
  verifies its tree ID before finish. A failed core trial after the release
  leads to a Grove patch release.
- A failure in the native change and an inverse after a failed probe share
  the continuation of a stop.
- Probes and core trials run outside every Grove workspace, without Grove's
  loop control variables. They cannot record finish's teardown.
- Every `apply` and every complete `mise bootstrap` counts as an advance. A
  process check precedes it.
- Each case of the credential control uses the allowlist of a fixture HOME as
  its base, adds one route and clones through mise.
- Smaller repairs define the owner-run switch and compare every pin with its
  accepted pin. They also move the draft check after the clones and forbid Git
  attributes that change the bytes of a checkout.

The focused review `owner-pack-checkouts-k83` found three P2 defects in that
repair. Its integration `owner-pack-checkouts-k84` repaired the specification
and its views:

- **An inverse that could not fail.** A rerun of the switch now reads the
  state of the switch set from the layout and the latest backup. A partial
  state resumes the inverse, and the native probe must pass before a retry.
  Each backup is new, and the pending-activation record names each backup.
- **Activation without its receipt.** The active state reruns the checks,
  the probes and the receipt, and it changes no item. The owner's run after
  finish is now named beside finish as a run against the real HOME.
- **A tree ID taken as the captured files.** The accepted identity of each
  source is its Ravel `resourceDigest`. The section
  [Checkout configuration and the tree ID](#checkout-configuration-and-the-tree-id)
  records why. Step 6 compares each real checkout with its accepted digest
  in a disposable HOME, and a probe compares it again.

One in-session reviewer then read the repaired resumption and found two P1,
seven P2 and two P3 defects. The integration repaired each of them:

- A backup is complete only after its final rename. Step 3 keeps an
  incomplete backup aside.
- A file that other programs also write is compared only at the keys that the
  switch changes. A drifted item goes to the owner's classification, then to
  a new native backup.
- Step 9 writes each file once by a rename, after a comparison with its copy.
  Step 1 also lists the programs that write those files.
- The absence of the Ravel settings file is its earlier state. An entry at
  both paths is in neither state.
- Step 4 reads the latest Grove release, so a repair's patch release reaches
  the next run.
- In the active state, a stop in step 6 counts as a failed probe.
- The section "Rollback" defines the native probe.
- `mac-check` runs before the receipt, which goes into the backup first. The
  backup lies outside each tree that the instrument sweeps.
- The coordinated recovery rows name each state and keep a Grove patch
  release apart from a Ravel cut.

The focused review `owner-pack-checkouts-k85` found one P1 and three P2
defects in that repair. Its integration `owner-pack-checkouts-k86` repaired
the specification and its views:

- **A comparison that cannot stop a writer.** Step 9 compared a file and
  then renamed it, so a write between the two was lost. Step 1 now lists the
  jobs that can launch a writer and counts the descendants of finish. One
  prepared command runs each write interval. An atomic exchange publishes
  each file, and the run compares the displaced content. A difference fails
  the part and keeps the update in the backup. The section
  [Publication by atomic exchange](#publication-by-atomic-exchange) records
  the call.
- **A receipt that outlived its layout.** The layout now decides the state,
  and a receipt only tells the complete state from the active one. Step 3
  and the inverse withdraw a receipt that no longer applies.
- **A moved entry identified by its path.** The state of a moved entry now
  also needs its recorded change identity. A changed entry or a retargeted
  link at either path is drifted.
- **A changed plan without a rehearsal.** Each rehearsal records the digests
  of its manifest and inverse. A changed plan reuses that evidence only for
  items of a rehearsed kind. Any other change needs a new rehearsal before
  step 8.

The in-session reviewer of `owner-pack-checkouts-k86` found three P1 and nine
P2 defects, R1 to R12, and eight P3 items. k86 judged them a redesign, and
`owner-switch-recovery-k87` rebuilt the recovery model with the model above.
Each item has this disposition:

| Item | Disposition |
| --- | --- |
| R1: one process check before the probes | Closed. The program lists the processes before each write interval, and no probe session may still run. Detection protects writes in any case. |
| R2: an exemption by absence | Closed. No exemption remains. The run's own process tree may write, and detection covers it. The staging descendant trial is gone. |
| R3: no exclusion for the owner's inverse | Closed. The owner's inverse runs the program copy, with the same listing and detection. |
| R4: a drifted file blocks the native return | Closed. The inverse resets each switched cell and leaves only third values. |
| R5: the Ravel settings compared whole | Closed. Only the `skills` key is a cell. |
| R6: a receipt without a second layout check | Closed. Step 13 reads each cell again and records the probed values. "Complete" also needs them now. |
| R7: states ignore a changed manifest | Closed. The state uses the open run's plan or the prepared plan. A changed entry stops the active state before its receipt. |
| R8: a classification leaves no record | Closed. A classification makes a new plan with the accepted values. |
| R9: an interruption looks like an outside write | Closed, with one accepted case. The run record and the prefix rule separate them. An outside write of a switched value during an open run is reset and reported. |
| R10: a digest record cannot decide reuse | Closed. A rehearsal names the plan and the program, and any change needs a new rehearsal. |
| R11: a rehearsal inside finish | Kept from k86. Finish stops on an unrehearsed plan, and the owner rehearses after finish. |
| R12: no withdrawal after a successful finish | Closed. No withdrawal record exists. The layout decides the state, and `mac-check` reports it. The manual's receipt applies only while `mac-check` reports the switch complete. |
| P3: mode and attributes lost | Closed. The stage file takes the live metadata, and the digest covers it. |
| P3: a stage file under the displaced name | Closed. A copy that holds its stage content holds no outside write. |
| P3: a failed verification | Closed. A failed final reading is a conflict, and the run inverts its parts. |
| P3: "steps 1 to 8 change nothing" | Closed. The text names the resumed inverse and the pin advance. |
| P3: "byte for byte" against the fault points | Closed. Order 6 now requires each cell value and each outside write to survive. |
| P3: step 1 or step 2 stops | Closed. The seams say step 2. |
| P3: drift seams need a backup | Closed. Before the switch, a changed entry gives the changed state. |
| P3: the switch's own shell | Closed. The run's own process tree includes the program's shell. |

The focused review `owner-switch-recovery-k88` found one P1, three P2 and
one P3 defect in that redesign. Its integration `owner-switch-recovery-k89`
repaired the specification, the model and the views:

- **A merged copy that returned to older content.** The reviewed predicate
  tested the content that the run read before the merge record. A copy that
  a descriptor returned to that content was not flagged. The merge digest is
  now the only recorded content of a merged copy, and the owner's merge
  applies the copy's difference, a removed key included. The model reaches
  the trace, and its mutant without the rule loses the removal.
- **Descendant readers inside the exclusion.** Step 1 exempted the whole
  descendant tree, so a delegate session could read a moved path. Step 1 now
  excludes only the kept processes: the harness process, its driver chain,
  its MCP servers and the program. Each other descendant counts as another
  process. The k87 text had also dropped the driver chain, which k86 named.
  A reader control tests the exclusion apart from detection.
- **A complete continuation without the outside comparison.** A run that
  stopped between steps 13 and 14 copied its receipt without a comparison
  of the entries outside the switch set. Step 14 now compares them first,
  and a changed entry counts as a failed probe. The model's fifth property
  and its mutant cover the rule.
- **An inverse onto an occupied path.** An exclusive rename cannot move a
  retained entry onto an occupied original path. The inverse now moves an
  entry only to an absent path. An occupied path is a third value, and the
  inverse leaves both entries. The design argues the entry case in full in
  place of a model extension.
- **Shared node identities in the cutover view.** The recovery nodes reused
  two identities of preparation nodes. They now have their own identities.

One in-session reviewer then read the argument for entry parts and found
four P1, two P2 and three P3 items. The integration repaired five of them:

- The inverse moves an entry of any change identity back. The inverse
  changes no other third value, and the cell rules now name that exception.
- The switch compares a moved entry with its recorded change identity, which
  the baseline instrument defines.
- Purge requires each retained entry to keep that change identity.
- The retained path belongs to the open run's backup. While no run is open,
  an entry cell is the original path alone.
- The drifted row names a receipt as the last record, as the model does.

The claim that an occupied path cannot repeat a failure is now limited to
that case. Another failed rename stops each run until the owner repairs it.
A parent path that another program replaced with a link was noise, because
the entry still lands at its original path. The resume rule for a moved-back
entry is the design's stated trade-off: a third value waits for the owner.

Row R2 of the table above records the whole process tree as exempt. The
repair of the descendant readers narrows that exemption to the kept
processes.

<!-- tc: off because this section preserves the completed review's findings as history -->

## Owner switch implementation review

Scope: this section describes the switch program before
owner-switch-contraction-k270. That leaf removed the old `run`, `inverse`,
`merge`, `pending`, `state` and `purge-check` entries, with the exchange,
held links, kept copies, cell inverses, rehearsal admission and the plan's
parts. The observations stay as history. They are not evidence for the
current program, which [Owner switch contraction](#owner-switch-contraction)
describes.

`owner-switch-program-k242` inspected producer commit `7d69e659`, from base `27723524`. These findings are pending integration triage; they are not repairs or new test results.

### F1 — P1: gates can read a plan other than the rehearsed plan

`scripts/owner_switch.py:838-845`, `1426-1433`, `1853-1854` and `1900-1904`.
The controller loads the prepared plan once and keeps its bytes and digest.
It then passes the original pathname to each command. Each gate opens that
pathname again. Step 8 checks the digest of the controller's earlier bytes,
and the backup receives those earlier bytes, not what the gates read.

For example, start with a rehearsed plan A. After its initial load, change
the on-disk accepted digest of `superpowers` to the digest of a modified
installed source. Step 6 reads that new value and passes. Step 8 still
admits A's rehearsal, although A would have rejected that source. The
receipt can therefore certify a source that the rehearsed plan did not
accept. A file edit, or a plan command that edits its own plan, suffices.
The rule in "The rehearsed plan" requires a changed plan to have its own
passing record. Every gate needs the same admitted bytes, or a checked
binding to their digest; reopening an unchecked pathname does not supply it.

### F2 — P1: descendants of kept MCP servers bypass the process gate

`scripts/owner_switch.py:805-819` exempts every descendant of each matching
MCP server, as well as every descendant of the switch process. The contract
keeps the MCP servers themselves and counts each other session descendant
as another process. These are different sets.

A kept MCP server can have a running tool child that launches Codex, Grove
or a background reader of a link that step 9 moves. Even when the child's
arguments match `watch`, the `pid in kept` branch skips it. Step 9 or an
inverse can then open its write interval while that reader still runs.
The current delegate and background controls launch children of the fixture
harness, not children of a kept MCP server, so they do not cover this case.
This breaks "Concurrent writers", whose exclusion protects those readers
independently of the kept-copy mechanism.

### F3 — P1: probe and trial environments can select another harness HOME

`scripts/owner_switch.py:838-847`, `867-871`, `1215-1220` and `1566-1570`.
The five Claude Code location variables are removed only by `claude_env`,
which the installation prelude uses. Plan probes, native probes and the
step-4 trials instead inherit them through `command_env`. They also inherit
`CODEX_HOME` unchanged.

For example, run with `CODEX_HOME=/Users/antony/.codex` and a disposable
`--home`, or with `CLAUDE_CONFIG_DIR` naming an earlier staging directory.
A direct harness command in `probes` can then inspect or change that
directory instead of the switch's HOME. In the real switch it can report a
pass from the staging configuration while the real configuration fails.
This breaks the effective-directory and environment rules in "Settings
isolation and the staging HOME" and the real-HOME probes of step 10.
`Case.run` deliberately sets `CLAUDE_CONFIG_DIR`, but its fixture probe
reads JSON through `OWNER_SWITCH_HOME` and never tests a harness's effective
configuration directory, so the rehearsal misses this escape.

### F4 — P1: plan paths can escape a disposable HOME

`scripts/owner_switch.py:428-440` and `447-457`. The loader validates only
the schema token and entry ordering. `Layout` then joins plan paths to HOME
without validating that they are relative or remain inside it. An absolute
path discards HOME; `..` and symlinked parent directories can escape it too.

A plan entry whose path is `/Users/antony/.codex/skills/example` will name
the real entry even under `--home /tmp/rehearsal/home`. Given the matching
identity and an admitting record, step 9 moves that real entry into the
disposable backup. A `manual` path can similarly append a receipt outside
the selected HOME. The documented plan format says these paths are
HOME-relative, and the node's contract says disposable rehearsal leaves the
real HOME unchanged. Neither property is enforced. This concerns the
program's data paths, separately from the explicitly executable commands
that a plan author is already trusted to supply.

### F5 — P1: an incomplete plan can report complete without required checks

`scripts/owner_switch.py:1832-1837`, `1893-1904` and `1757-1787`. Missing
gate lists and probe lists default to empty lists, a missing baseline does
nothing, and a missing `mac_check` becomes `None`. The loader does not
require these fields or require the gate commands to invoke the built-in
checks. An empty command list is treated as a successful step.

For example, a plan with `gates: {}`, `probes: []` and no `mac_check`, whose
parts match the native layout and whose digest has an admitting record,
can exchange its parts and write `result: complete`. No installed-release,
formula, source or reader check has run, and neither harness was probed.
A rehearsal record proves that bytes were exercised; it does not make an
omitted obligation present. The fixed steps 4 to 7 and 10 to 13 of "The
switch inside finish" cannot be optional fields of a plan that the program
accepts as a complete switch. The fixture always supplies these fields,
so its passing path does not establish rejection of this input.

### F6 — P2: step 4 reruns staging trials in the switch HOME

`scripts/owner_switch.py:1566-1570` runs each trial with the inherited HOME
and `cwd=gate.home`. Under `run`, that HOME is the target of the switch;
inside finish it is the real HOME. The gate carries no retained staging
HOME and does not select one for these trials.

When Grove's installed `design` source differs by one byte, a trial that
launches its harness using HOME will therefore read the native real-HOME
configuration before step 9, instead of the retained staging configuration.
It can fail for the wrong reason or modify the real harness configuration
as preparation for its trial. Step 4 explicitly requires these reruns in
the retained staging HOME, under the isolation rules. A command wrapper
could supply that HOME, but neither the command contract nor the gate
requires or checks one. The fixture trial only appends a source and digest
to a log and returns 0, so it cannot detect the wrong subject.

### F7 — P2: reader evidence can become invalid before success

`scripts/owner_switch.py:1631-1667`, `1757-1787` and `1798-1818`.
Step 7 checks registrations and `.orphaned_at` once. The program never
checks those paths again before the receipt, and the active continuation
reruns only gate 6. The fixture plan's outside manifest does not include
the Grove or official-plugin reader cache directories.

For example, let gate 7 pass for the registered official-plugin directory.
An update after that gate, or between an active interruption and its
continuation, can orphan that directory and move its registration. The
switched cells and the manifest can still match, so the run copies a receipt
whose reader evidence describes the earlier registration and no marker.
The same case applies to a named directory that finish read. "Resource
lifetime" requires the versions that live readers use to remain registered
and current, and freezes updates of every plugin finish loaded. Gate 7 does
not verify that freeze or detect its violation during the remaining run.
The existing orphan control adds its marker before the first gate only.

### F8 — P2: the success receipt omits required activation and recovery evidence

`scripts/owner_switch.py:1783-1787`, `1798-1818` and `1904`.
The receipt consists of plan/program digests, probe results, `mac_check`,
cells and gate results. Step 5's formula evidence is discarded when the
backup is written. No code adds the digest of `config.rhai`, a digest of
the activated settings file, the other formula versions, the backup
locations or the retained byte sets and their purge conditions.

Consequently even the ordinary successful fixture run produces a manual
receipt without that information. Its plan digest and earlier gate output
do not record the actual activated configuration or give the owner a
complete cleanup inventory. The final paragraph of "The switch inside
finish" explicitly requires these fields. The receipt control checks only
the presence of gates 4, 6 and 7, so it accepts this incomplete evidence.

### F9 — P2: a full passing rehearsal need not measure Claude Code's writes

`scripts/owner_switch_rehearsal.py:1818-1837` and `Taskfile.yml:414-419`.
The normal task leaves `--settings-writes` off. `cmd_rehearse` treats a
missing rerun as passing and writes `passed: true` with
`settings_writes: null`. The switch's `passing_rehearsals` accepts that
record without any measured Claude Code version or write method.

For example, upgrade Claude Code to a version whose installation writes in
place, then run the default complete fixture rehearsal. Its stub still
renames like 2.1.319, and the resulting record admits the program's
preludes. The real command is rejected only after it has already written
in place; the held inode then supplies no saved earlier bytes. "Test
seams" requires the rehearsal to rerun `settings-writes.sh` with the
version it uses and record that version, or revisit the accepted case.
The producer's separately recorded measurement is useful evidence but is
not a requirement enforced by a newly passing record.

### Review evidence and limits

The graph project was
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-07T09:15:24Z`. Both pages of the bounded function search were read.
Exact source and graph snippets support the findings; an outbound trace of
`run_switch` supplied call candidates, with source controlling where the
graph attached an unrelated Rust `get` method. Coverage metadata matched
both Python files, the Taskfile and the specification, with no recorded
gap. The model's coverage metadata was not tracked, so its source was read
directly. This is task-directed verification, not a claim of graph
completeness.

The state classifier follows the six rows on the inspected inputs. In
particular, an open run skips the preludes, an active continuation obtains
admitted digests from its own backup's step-4 evidence, and a differing
program digest causes execution of the backup's program before the state
continuation. The model explicitly leaves readers, executable program
identity, real probes and metadata outside its checked abstraction; its
recorded green result does not establish those properties of Python.

No unsupported tool-response finding is asserted: Homebrew's revision is
included in the installed version comparison, mise must report `installed`,
and a Ravel document whose status is not `resolved` is rejected. The
findings above concern concrete unchecked inputs and transitions rather
than an assumed later tool format.

## Owner switch review integration

Scope: this section describes the switch program before
owner-switch-contraction-k270. That leaf removed the old `run`, `inverse`,
`merge`, `pending`, `state` and `purge-check` entries, with the exchange,
held links, kept copies, cell inverses, rehearsal admission and the plan's
parts. The observations stay as history. They are not evidence for the
current program, which [Owner switch contraction](#owner-switch-contraction)
describes.

`owner-switch-program-k243` triaged the findings F1 to F9 above against the
program at `7d69e659`. A reading of the source reproduced each finding. Each
one is a real issue, and each repair stays inside the program, its rehearsal
and the existing rules of the specification. No repair changes a recovery
transition, so the model stays unchanged.

| Finding | Disposition | Repair | Guarding control |
| --- | --- | --- | --- |
| F1 | Real | A gate that the program launches stops unless the plan at `OWNER_SWITCH_PLAN` has the digest `OWNER_SWITCH_PLAN_DIGEST` | `plan-changed-after-load` |
| F2 | Real | The process gate keeps only a chain of processes that match the MCP patterns. Each other descendant of a server counts | `mcp-server-child`, `mcp-server-kept` |
| F3 | Real | Each command of a plan drops Claude Code's five location variables, `CODEX_HOME` and each `RAVEL_*` name | Each fixture command refuses them |
| F4 | Real | Each plan path is relative, normalised and free of `..`. Its parent folder must resolve inside HOME | `plan-paths-escape` |
| F5 | Real | The loader requires every field. Each step that runs commands needs one, and each gate list names the program's gate | `plan-incomplete` |
| F6 | Real | Each trial runs with `HOME` set to the plan's staging HOME and `CODEX_HOME` set to its `.codex` tree | `staging-home-missing`, the trial's own refusal |
| F7 | Real | Inside the probes, the run reads step 7's reader evidence again. A change counts as a failed probe | `reader-orphaned-in-probes`, `reader-moved-before-continuation` |
| F8 | Real | The receipt holds the activated configuration, each source, the trials, the readers, the backup and each retained byte set | `receipt-evidence` |
| F9 | Real | A passing record needs a passing rerun of `settings-writes.sh` and its version. The hooks installation runs a Claude Code command only with that version | `rehearsal-without-settings-writes`, `claude-version-differs` |

The model abstracts each probe as a pass or a failure. The reader rule of F7
therefore adds a cause of failure and no transition. Exclusion, the
environment and the receipt's fields are outside the model, as before.

Each repair has a mutant of the repaired program that lacks only that
repair. Each mutant failed its guarding control, and the positive control
`mcp-server-kept` passed against the F2 mutant. The F1 mutant completed the
switch with the changed source, which is the case that F1 states.

## Owner switch entries

Scope: this section describes the switch program before
owner-switch-contraction-k270. That leaf removed the old `run`, `inverse`,
`merge`, `pending`, `state` and `purge-check` entries, with the exchange,
held links, kept copies, cell inverses, rehearsal admission and the plan's
parts. The observations stay as history. They are not evidence for the
current program, which [Owner switch contraction](#owner-switch-contraction)
describes.

`owner-switch-entries-k92` added four entries to the program, a switch row
to the owner's `mac-check` and their rehearsal controls. The entries are the
owner's inverse, the merge of a flagged copy, the purge check and the
pending-activation record. The specification's sections "Concurrent
writers", "Rollback", "Purge" and "Test seams" state each rule.

No recovery rule changed, so the model stayed unchanged. The merge follows
`ownerMerge` of the model: it applies the copy's difference from its last
recorded content, and a cell takes only a third value. The owner's inverse
is the inverse of "Rollback". The purge check, the run log and the
pending-activation record are outside the model, because no state finding
reads them.

Each record now keeps the bytes that its reading read. A merge needs those
bytes, because a digest cannot give a difference.

| Mechanism | Guarding control | Mutant that lacks it |
| --- | --- | --- |
| A merge removes a key that the copy lost | `merge-difference`, `merge-return-through-descriptor` | Failed both |
| A merge digest is the only recorded content of the copy | `merge-return-through-descriptor` | Failed |
| A cell takes only a value that no run writes there | `merge-third-cell` | Failed |
| The inverse leaves an occupied original path | `owner-inverse-occupied-path` | Failed |
| The owner's inverse runs the program copy of the latest backup | `owner-inverse-runs-backup-copy` | Failed |
| The record holds the mise status | `pending-after-step` | Failed |
| A receipt names each record that it replaces | `pending-after-inverse` | Failed |
| The purge check compares each retained entry's identity | `purge-check-retained-changed` | Failed |
| A stopped run keeps its command evidence | `pending-after-step` | Failed |
| `mac-check` reports a flagged copy | The three `mac-check-flagged-*` controls | Failed all three. Passed `mac-check-native` and `mac-check-complete` |

The `mac-check` controls run the owner's real command with HOME set to a
disposable HOME. That HOME links the checker's folder and holds copies of
the plan and the program in `<state>/prepared`. The checker's own tests in
`~/.local/lib/machine-config/tests` cover each state's fixed message, a path
that is not safe to print and a failing program.

On 2026-11-07, `task owner:switch:rehearse` passed 216 of 216 controls with
Python 3.15.1, on program `885171751f02` and plan `a1d5f409abbf`. Its rerun
of `settings-writes.sh` passed with Claude Code 2.1.319, so the record
passed. Each subject file was last changed before the run started. `task
model:owner-switch` reported "all claims hold". The model, its runner and
the Taskfile had the same digests before and after that run.

<!-- tc: on -->

## Re-inventory before the cutover

`owner-ravel-skill-inventory-k38` performed order 3 of the cutover contract
on 2026-11-07. It reran the baseline instrument in the real HOME and
compared each entry with [the baseline](owner-skill-inventory.tsv). The
instrument, the controls script and the baseline had the SHA-256 values of
"Method". Claude Code was 2.1.319 and Codex was 0.167.0.

### Controls of the rerun

- Two runs gave byte-identical output of 447 rows, with the SHA-256
  `b4f7d65ced0c…`. No row said `UNCLASSIFIED`, `UNASSIGNED`, `UNCOVERED` or
  `NEEDS-REVIEW`.
- A scratch root held an unknown skill and a dangling link. With
  `CONTROL_ROOT` set to it, the output held one `UNCLASSIFIED` row and one
  dangling row for that root.
- `task owner:inventory:controls` passed 14 of 14 controls.

The comparison keyed each row by its harness, location and entry. It
compared every field of the rows with the same key, and it listed each key
that only one side held.

### Differences and their reasons

No owner-pack, third-party-pack, stale-copy or dangling skill row changed.
Each one keeps the change identity that the section "Owner-pack entries of
the baseline" of the [moves record](repository-moves-record.md) maps to its
source and root. No new row has an owner-controlled class. The rows that
differ are these:

| Rows | Class | Difference | Reason |
| --- | --- | --- | --- |
| `grove`, `linkuistics` and `testanyware` of the `linkuistics` marketplace. `superpowers` and `quint-llm-kit`. `commit-commands`, `explanatory-output-style`, `feature-dev` and `swift-lsp` | Owner pack, third-party pack and capability-only | The digest only. The version and the file count stay the same | Each version directory holds `.in_use/<pid>`, and the PID is that of the running Claude Code session. The baseline held the marker of an earlier session. Without that folder, each count is one less, as in the baseline |
| `context7` and `playwright` | Capability-only | Version `a7ba5b93db0c` became `0412be76f50a` | The official marketplace updated them automatically |
| `github`, `serena`, `skill-creator` and `greptile` | Disabled third-party | New versions. `greptile` 1.3.1 adds the skills `login` and `review` | The same automatic updates. Each plugin stays disabled, so its skills stay outside the active collection |
| Five account-synced skills, `openai-docs` and the Codex bundled and remote plugins | Platform-managed | New digests or versions | Platform updates. No Ravel source holds these names |
| `testanyware` and `TestAnyware.app` | Tool-managed | 2.7.0 became 2.8.0 | Homebrew installed the release whose formula carries the `testanyware` source |
| The coverage rows of inactive cached versions | Not a discovery root | Fewer versions | Claude Code's 14-day cleanup of orphaned versions |
| The coverage row of the marketplace clones | Not a discovery root | 89 `SKILL.md` files in place of 82 | The official marketplace clone gained skills when it updated |

TestAnyware's keg already links `share/ravel/skills/testanyware` into
Ravel's default skill directory. The real HOME's `~/.config/ravel` holds only
`policy.ts`, so Ravel 1.1 delivers nothing from that directory.

### Consequences for the cutover

- The instrument's digest of a plugin version directory depends on the
  session that runs it. A later comparison would report each loaded plugin
  as changed. `owner-inventory-reader-marker-k244` repairs the digest.
- A probe session writes the same marker. The real plan's `outside`
  manifest therefore names no plugin version directory.
- The official marketplace updated enabled plugins in five days. The freeze
  of order 6 covers it as well as the `linkuistics` marketplace.
- Claude Code moved from 2.1.316 to 2.1.319 in the same five days. The
  rehearsal record names one Claude Code version, so that version must hold
  from the rehearsal to the switch.

## Reader markers in the plugin cache

`owner-inventory-reader-marker-k244` repaired the instrument on 2026-11-07.
Its controls and its rerun used Claude Code 2.1.319.

### The marker

Claude Code writes `.in_use/<pid>` into each plugin version directory that a
running session loaded. The marker holds the session's `pid` and
`procStart`. The session removes its own marker when it ends, and the empty
`.in_use` directory stays. The marker therefore identifies a reader, not the
plugin.

The Claude Code plugin row now leaves out the entry `.in_use` at the top of
the version directory, whatever its type. The file count leaves it out too.
A nested `.in_use` and `.orphaned_at` stay in the digest. The Codex
installed-plugin row does not change, because Codex writes no such marker.
Each other entry class keeps its digest.

Claude Code 2.1.319 uses the same rule for its own folder digest of managed
plugin releases. The settings schema in the executable states that this
digest leaves out `.git`, `.in_use`, `.orphaned_at`, `.claude-plugin-link`
and `.links_materialized` at the root.

### Controls of the repair

`task owner:inventory:controls` passed 19 of 19 controls. The fixture's
plugin version directory holds one marker of an earlier session, as the
baseline did. Five cases are new:

| Case | Expectation |
| --- | --- |
| Added marker | The whole output stays the same |
| Changed marker | The whole output stays the same |
| Removed marker, with the empty `.in_use` directory kept | The whole output stays the same |
| Changed plugin manifest | The plugin row changes |
| Nested `.in_use` entry under `.claude-plugin` | The plugin row changes |

Each new control failed against an instrument that it guards against:

| Instrument | Result | Failed cases |
| --- | --- | --- |
| The earlier instrument, SHA-256 `f3c2a86e…3726` of "Method" | 16 of 19 | The three marker cases. The plugin row changed in each |
| A mutant that leaves out `.in_use` at every depth | 18 of 19 | The nested entry. The whole output stayed the same |
| A mutant that leaves out each dot entry at the top | 17 of 19 | The changed manifest and the nested entry. The whole output stayed the same |

The two mutants existed only in a scratch directory. They are not part of the
repository.

| Artifact | SHA-256 |
| --- | --- |
| `docs/evaluations/owner-skill-delivery/inventory.py` | `b933ed455dc53af5c70ef9e2a7a3bc1389e32e413bbca8c4486d0c063c749ccc` |
| `docs/evaluations/owner-skill-delivery/controls.py` | `812fe247edead7c20570bd4a79840e99879d02feb90abc0f76665c836ee80a0b` |
| `docs/owner-skill-inventory-reference.tsv` | `7a16008e3213848f7e96ca2b0b00f7e2b9db43f2769334b79bccb4e5f4885f02` |
| `docs/owner-skill-inventory.tsv` | `cbfac5b83548a3f434f34d2ac4e15afcc4004ea6b212a0beabff58b3de58a0d0`, unchanged |

### Rerun in the real HOME

The repaired instrument ran twice in the real HOME, and the earlier
instrument ran once between the same subject digests. The subjects were both
instruments, the controls script, both Claude Code settings and registry
files, both Codex configuration files and the `mcpServers` object of
`~/.claude.json`. No digest changed.

- The two runs of the repaired instrument gave byte-identical output of 447
  lines: a header and 446 rows. No row said `UNCLASSIFIED`, `UNASSIGNED`, `UNCOVERED` or
  `NEEDS-REVIEW`.
- With `CONTROL_ROOT` set to a scratch root, the output held one
  `UNCLASSIFIED` row and one dangling row for that root.
- The output of the earlier instrument had the same keys. It differed in 11
  rows. Each was a Claude Code plugin row, and only its digest and file count
  differed. Each count was one more. These 11 version directories held one
  marker file each. The other 15 Claude Code plugin rows were the same. Their
  directories held an empty `.in_use` directory or none.

The repaired output is [the reference inventory](owner-skill-inventory-reference.tsv).
Each difference between the reference and the baseline is in a class of
"Differences and their reasons" above, or is the repair itself.

### How a comparison reads the baseline

The baseline stays unchanged, because the moves record and the design cite
its line numbers. Its Claude Code plugin rows include one marker of an
earlier session in the digest and in the file count. Nobody can reconstruct
the bytes of that marker. A later comparison therefore reads the rows in
this way:

- For a Claude Code plugin row, compare with the reference, not the
  baseline. The key, the version, the commit and the repaired digest must be
  equal, unless a recorded update explains the difference.
- For a Claude Code plugin row of the baseline, compare the version, the
  commit, the class and the disposition. Its digest16 is not comparable. Its
  file count is one more than the repaired count when a marker was present.
- For every other row, compare with the baseline as before. The reference
  shows that the repair did not change these rows.

### Finding for the owner design

Claude Code 2.1.319's orphan cleanup reads `.in_use`. The executable's
cleanup first finds an `.orphaned_at` older than 14 days. It then runs a
liveness check on the version directory before it removes it. That check
parses each `.in_use/<pid>` marker and treats a marker that it cannot read as
live. For a live session, the cleanup logs "Skipping orphan cleanup, in use
by live session" and keeps the directory. The
[plugin loading reference](https://code.claude.com/docs/en/plugins/loading#cleanup-of-previous-versions),
read on 2026-11-07, does not mention this check.

`owner-cleanup-reader-check-k256` inspected the installed executable again on
2026-11-07. `claude --version` reported `2.1.319 (Claude Code)`.
The resolved executable was
`~/.local/share/claude/versions/2.1.319`, with SHA-256
`1f53587e92dee8e089fa349188a4cda64fa34ace6e7b0ca2ee278ddaa3cc2ebd`.

The inspection found the age constant `P9n=1209600000`, which is 14 days
in milliseconds. The cleanup calls `Fw` before deletion. A live result
or an error returns without deletion. `Fw` reaches `JWt`, which reads the
markers, and `m1t`, which checks process liveness. An unreadable marker
counts as live. These minified names identify this executable only.

This was a static inspection. It invoked no cleanup and changed no cache.
It establishes the branch in this executable, without measuring a cleanup
run or promising the behaviour of another version.

The earlier design stated that cleanup ignored readers.
[Resource lifetime](specs/owner-skill-delivery.md#resource-lifetime) now states
the age and reader checks for 2.1.319. The freeze, versions registered and
current, reader evidence and purge conditions all stay. That section records
each rule's reason independently of the undocumented check. Codex and Pi
write no marker, and a later Claude Code version can change the check.
The existing switch and its recovery model therefore need no change.

## Policy conversion

`owner-policy-conversion-k245` converted the owner's Ravel policy to Rhai on
2026-11-07. This is order 4 of the
[cutover contract](specs/owner-skill-delivery.md). The owner approved the
candidate. The leaf then copied it to `~/.config/ravel/config.rhai`, beside
the unchanged `policy.ts`. No `settings.json` existed before or after the
copy, and the leaf wrote none.

| File | SHA-256 |
| --- | --- |
| `~/.config/ravel/policy.ts`, modified 2026-11-07 19:34 | `30a0e5fca7c1446999bd19289566fc078053a503c75f03ca7056e641f836f131` |
| `~/.config/ravel/config.rhai`, and the committed candidate [`owner-config.rhai`](evaluations/owner-skill-delivery/owner-config.rhai) | `bbde65efccbfd09e8ac84ab41e1322d35fa31b9726b8451466ce873dd22f88d8` |
| Ravel 1.1.0 keg, `bin/ravel` | `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed` |
| Accepted 2.0 build, `bin/ravel` from `target/ravel-archives-2.0` | `3a9a9129573e4d827a317f6b793ba990044080a31a7b667b64cc153a2f974a0b` |

The 2.0 notice reports build identity `58d479ed…`, which
`scripts/harness-trials/builds.json` names.

### The translation

The Rhai file keeps the routing table, both argument shapes and the
jj-workspace grant. `VERSION` is `harness-dispatch 3`. The conversion needs
no host capability that the accepted build lacks.

`ravel::read_text` replaces `statSync` and `readFileSync`. A missing `.jj/repo` gives `source_missing`, and a directory
gives `source_not_regular`. Both give no grant, as in `policy.ts`.
`ravel::path_resolve` and `ravel::path_parent` replace Node's `resolve` and
`dirname`.

The conversion adds no read grant for skill snapshots. Such a grant changes
the argument vectors when skill delivery is on.
`owner-staging-discovery-k249` measures whether Claude Code needs one.

Two edge cases differ by design. A `.jj/repo` that is not UTF-8 refuses under
Rhai, and Node decoded it with replacement characters. The refusal of an
unreadable `.jj/repo` carries the host's own error text.

### The equivalence run

`task owner:policy-equivalence OUT=<new directory>` runs
[`policy_equivalence.py`](evaluations/owner-skill-delivery/policy_equivalence.py).
Each build has its own private HOME, which holds only its own file. The PATH
holds fake `claude` and `codex` programs that record their arguments. No real
harness runs. The prompt holds quotes, a leading `--help`, a dollar sign and
non-ASCII characters.

The cases are each of the 24 kinds of `policy.ts` in five directories, and
three refusals. The directories are a plain directory, a primary jj workspace,
a secondary workspace that jj created, and two hand-written secondary
workspaces. One of these names its store by an absolute path. The other uses
a relative path with surrounding whitespace. The refusals are a kind that the
policy does not list, a whitespace-only `.jj/repo` and an unreadable one.

| Run | Rhai input | Result |
| --- | --- | --- |
| Candidate | `owner-config.rhai` | 123 passed, 0 failed |
| Installed copy | `~/.config/ravel/config.rhai` | 123 passed, 0 failed |
| Control: `copy-edit` at `medium` | Mutated copy | Only the five `copy-edit` rows failed |
| Control: grant one level too deep | Mutated copy | Only the 72 secondary-workspace rows failed |

In each routed case, both builds launched the same program with the same
argument vector. They reported the same provider, model, effort and reason.
In each refusal case, both refused with exit status 3, the same code and the
same remedy. Each refusal message was equal, except for the unreadable store.

A further check copied the real `~/.config/ravel` with both files into a
private HOME. There the 1.1.0 keg evaluated `policy.ts`, and the 2.0 build
evaluated `config.rhai`.

### Obligations that remain

A later change to `policy.ts` needs a new conversion and a new equivalence
run before the switch. The digest of `policy.ts` in the table above
identifies the converted version. Compare it with the live file before the
switch.

After this driver's reap, the owner does these steps:

- Remove `~/.config/ravel/policy.ts` when no 1.1 launch remains.
- Remove the inactive 1.1.0 keg with `brew cleanup ravel`, outside the
  finish session of its last driver.
- Write any new settings keys, which Ravel 1.1.0 refuses as
  `settings_invalid`. These can be the `harnesses` entries for Claude Code and
  Codex. The owner sets no `policyEnv`, so no `configEnv` conversion is
  necessary.

## Owner settings drafts

`owner-settings-drafts-k246` drafted the owner's Ravel skill settings and the
declarations of the skill formulae on 2026-11-07. This is order 4 of the
[cutover contract](specs/owner-skill-delivery.md#cutover-contract). The drafts
are in [`owner-cutover/`](owner-cutover). No draft is in the real HOME. The
real `~/.config/ravel/settings.json` was absent before and after the work.

| Draft | Content | Consumer |
| --- | --- | --- |
| [`settings-off.json`](owner-cutover/settings-off.json) | The off value. `sources` is empty, and `mandatory` is absent | Step 3 of the switch, the earlier value of the `skills` cell and the inverse |
| [`settings-switched.json`](owner-cutover/settings-switched.json) | The switched value. It names no `sources` and holds only the mandatory file, superpowers `skills/using-superpowers/SKILL.md` | Step 9 of the switch |
| [`settings-staging.json`](owner-cutover/settings-staging.json) | The staging settings. They name the ten sources in byte order of their IDs, with the roots of each source and the same mandatory file | The staging HOME |
| [`bootstrap-packages.toml`](owner-cutover/bootstrap-packages.toml) | The five `[bootstrap.packages]` declarations of "Skill formulae" | `owner-pack-checkouts-k93` |

Each settings draft is a complete settings file that holds only the `skills`
key. The cells of the switch plan take the value of that key.
`owner-switch-plan-draft-k247` and `owner-switch-plan-k254` consume the off
value and the switched value.

### Paths of the staging settings

Grove's six sources are in the candidate keg. The staging settings name each
of them through the link of the candidate's skill directory:

`/Users/antony/.local/state/owner-staging/candidate/brew/share/ravel/skills/<source-id>`

`task owner:candidate-keg` makes that link for each source when its `OUT` is
`/Users/antony/.local/state/owner-staging/candidate`.
`owner-staging-setup-k248` builds the keg with that `OUT`. The path is outside
every working copy and the scratchpad, and it holds no version.

The four other sources are in Ravel's default skill directory of the real
Homebrew prefix, at `/opt/homebrew/share/ravel/skills/<source-id>`. Homebrew
links each of them to its installed keg. No path of the staging settings names
a versioned keg.

The staging HOME does not contain a source, and no source contains it.
Otherwise Ravel refuses the source as `skill_access_unavailable`.

### The declarations

The owner's `~/.config/mise/config.toml` already declares
`brew:linkuistics/taps/grove` and `brew:linkuistics/taps/testanyware`.
`owner-pack-checkouts-k93` adds the three other declarations to that table.
The local tap holds each of the five formulae: `grove` 22.8.0, `personal`
2026-11-06, `testanyware` 2.8.0, `superpowers` 7.0.0 and `quint-llm-kit`
1.0.0. Of these, only `grove` and `testanyware` are installed.

### The check

`task owner:settings-drafts OUT=<new directory>` runs
[`settings_drafts.py`](evaluations/owner-skill-delivery/settings_drafts.py).
The script copies the accepted 2.0 build into the keg `Cellar/ravel@2/2.0.0`
of a fixture Homebrew prefix. The installation record of that keg names the
prefix's default skill directory. That directory links ten fixture sources,
one at each path of the staging settings, moved under `OUT`. Each case runs
`ravel skills resolve --json` under `env -i`, with a new HOME. The command
launches no harness, and the case PATH holds only `/usr/bin` and `/bin`.

| Case | Settings | Result |
| --- | --- | --- |
| `no-settings` | None | Exit 0, `resolved`, the ten sources in byte order |
| `off` | The off value | Exit 0, `disabled`. The HOME held no cache after the run |
| `switched` | The switched value | Exit 0, `resolved`, the ten default sources. The mandatory path lay in the superpowers snapshot and held the bytes of the fixture file |
| `staging` | The staging settings, with each path moved under `OUT` | Exit 0, `resolved`, the ten sources and the mandatory file |
| `staging-as-is` | The unchanged staging settings | Exit 3, `skill_source_missing` at `skills.sources[0].path`, the absent candidate path of `design`. The settings checks passed |
| Control | The off value with the mandatory reference | Exit 2, `settings_invalid` at `skills.mandatory` |
| Control | The switched value with an absent mandatory file | Exit 3, `skill_mandatory_invalid` at `skills.mandatory[0]` |
| Control | The staging settings with a repeated source | Exit 2, `settings_invalid` at `skills.sources[10].id` |

The script also requires the five declarations at `latest` and reads each
formula from the local tap with `brew info`, without an update.

The accepted build was `bin/ravel` with SHA-256 `3a9a9129573e…`, which
`scripts/harness-trials/builds.json` names. The drafts had these digests, which
were the same after the run:

| Draft | SHA-256 |
| --- | --- |
| `settings-off.json` | `6cdd20fd11675506d53584624ff8d8333c4f3c1f3d0c93b36c275865a7f306fa` |
| `settings-switched.json` | `944f2a245b8d60e6d61886015a15284657651788cbf4e232e85542ce6baa8743` |
| `settings-staging.json` | `d8a49f066310b0976cb4e13674c524d96d31b6aa0ba5bfc6923fc0b097623145` |
| `bootstrap-packages.toml` | `09dff6442b119698aa4ad6696f10b369dd9a61b9e3963046fd9ee324a1a308ee` |

Each check failed against a wrong draft in a scratch copy:

| Mutation | Cases that failed |
| --- | --- |
| The off value without `sources` | `off`, which resolved the ten sources and wrote a cache, and the first control |
| The switched value with another file as mandatory | `switched` |
| The staging settings with nine sources | `no-settings`, `switched` and `staging` |
| A misspelt formula name | The declarations, with Homebrew's refusal of that name only |

The fixture sources hold probe skills, not the published sources. The staging
trials resolve the published sources, as the cutover contract states. After
the Ravel cut, rollout repeats the off-value control with the published
`ravel@2` and the installed kegs.

## Owner switch plan draft

Scope: this section describes the switch program before
owner-switch-contraction-k270. That leaf removed the old `run`, `inverse`,
`merge`, `pending`, `state` and `purge-check` entries, with the exchange,
held links, kept copies, cell inverses, rehearsal admission and the plan's
parts. The observations stay as history. They are not evidence for the
current program, which [Owner switch contraction](#owner-switch-contraction)
describes.
The draft files `switch-plan-parts.tsv` and `switch-plan-dry-run.txt`
that this section names were replaced. The repository's history keeps them.

`owner-switch-plan-draft-k247` drafted the real switch plan on 2026-11-07.
Order 4 of the [cutover contract](specs/owner-skill-delivery.md#cutover-contract)
calls it the residue plan. The draft is not in the real HOME. The real
`~/.config/ravel/settings.json` and `~/.local/state/owner-switch` were absent
before and after the work.

| Artifact | Content |
| --- | --- |
| [`switch-plan-draft.json`](owner-cutover/switch-plan-draft.json) | The plan, schema `owner-switch-plan/1`, with every field of the format |
| `switch-plan-parts.tsv` | Each part and cell, with its line in [the reference inventory](owner-skill-inventory-reference.tsv), its class and its disposition |
| `switch-plan-dry-run.txt` | The output of `run --dry-run` in the disposable HOME |
| [`switch-plan-processes.txt`](owner-cutover/switch-plan-processes.txt) | The real process tree and job listing against the plan's patterns |
| [`switch-plan-check.json`](owner-cutover/switch-plan-check.json) | The record of the check, with the result of each control |

<!-- tc: depart synonym-rotation because "check" is the name of the task owner:switch-plan:check -->
The pattern listing and the check record of this section are history.
[Owner switch process inspection](#owner-switch-process-inspection) replaced
the patterns, regenerated the draft and rewrote the check record, the
listing and the dry run. The version control history holds this section's
files.

`task owner:switch-plan:draft` runs
[`switch_plan_draft.py`](evaluations/owner-skill-delivery/switch_plan_draft.py)
and writes the plan and the table from the real HOME. It reads only. Two runs
gave the same bytes. `task owner:switch-plan:check OUT=<new directory>` admits
the draft and checks it.

### The parts

The script reads each child of five discovery roots and finds its row in the
reference inventory. A row of the class `owner-pack`, `third-party-pack`,
`stale-copy` or `dangling`, with `deactivate` as the first word of its
disposition, makes an entry part. A child without exactly one row stops the
script. Each entry carries the change identity that `owner_switch.py
identity` prints.

| Root | Entry parts | Children that stay |
| --- | --- | --- |
| `~/.claude/skills` | The stale `grove` copy and the dangling `terminal-browser` link | `codebase-memory`, and the account-synced folder `synced` |
| `~/.codex/skills` | 44 links into Grove's snapshot store, 14 superpowers copies and `terminal-browser` | `codebase-memory`, and Codex's system folder `.system` |
| `~/.agents/skills` | 44 links into Grove's snapshot store, 3 quint links and `terminal-browser` | `codebase-memory` |
| `~/.pi/agent/skills` | 44 dangling links into `~/Development/grove` | `codebase-memory` |
| `~/.pi/agent/extensions` | The dangling `technical-communication.ts` link | Five extensions of codebase-memory, herdr and Orca |

The 154 entry parts come first. Then the part of `.claude/settings.json`
changes seven cells of `enabledPlugins`:

- It disables `grove`, `linkuistics` and `testanyware` of the `linkuistics`
  marketplace, `superpowers` and `quint-llm-kit`. Each is `true` in the real
  HOME and in the reference inventory.
- It enables `technical-communication-hooks@writing-hooks` and
  `guardrail@personal`. Both keys are absent in the real HOME today. The
  draft's earlier value is `false`. The hooks prelude of step 3 writes that
  value, and owner-pack-checkouts-k93 installs `guardrail` disabled.

The last part changes the `skills` key of `.config/ravel/settings.json`. Its
earlier value is the `skills` value of `settings-off.json`. Its switched value
is the `skills` value of `settings-switched.json`. The program's check of the
drafted real settings accepted it, because it names no `sources`.

### The manifest outside the switch set

`outside` holds three path entries and five root manifests:

- Path entries: `.claude/skills/codebase-memory`,
  `.codex/skills/codebase-memory` and Grove's snapshot store
  `.agents/.grove`, which the moved links name.
- Root manifests: `.agents/skills`, `.pi/agent/skills`,
  `.pi/agent/extensions`, `.claude/agents` and `.codex/agents`. A root
  manifest also detects a new child of its root.

`.claude/skills` and `.codex/skills` have no root manifest. A platform writes
their folders `synced` and `.system`. A probe session can rewrite either
folder, and step 14 would then count a failed probe and run the inverse. The
draft names no plugin version directory, because each Claude Code session
writes `.in_use/<pid>` there. It also leaves out `~/.codex/hooks.json` and the
`~/.quint-llm-kit` clone. Their tools rewrite them, and the switch reads
neither.

### Processes and jobs

The harness pattern matches a process whose program is `claude`. The MCP
patterns match each of the owner's stdio servers of Claude Code, and each
launcher chain:

| Server | Processes |
| --- | --- |
| `quint-kb` | `node .../.quint-llm-kit/repo/mcp-servers/kb/dist/server.js` |
| `codebase-memory-mcp` | The server, and its child `codebase-memory-mcp --cbm-daemon-internal` |
| Playwright | `npm exec @playwright/mcp@latest`, or `npx @playwright/mcp@latest`, and the `node .../playwright-mcp` that it launches |

Context7 uses HTTP and launches no process. Each other child of a server
counts as another process.

<!-- tc: depart synonym-rotation because "kept" names the design's term "kept process", not a second word for a check -->
Finding for the owner design: Claude Code runs `caffeinate -i -t 300` as a
child of the harness process. The design's kept processes do not name it, so
step 2 would always stop. The draft keeps it through `session.mcp`, because
no other plan field keeps a child of the harness process. The section
"Concurrent writers" of the design should name it.

The watch patterns match the program of a command, not any substring. A path
such as `.../grove` in another argument therefore does not match. They match
Claude Code, Codex, Pi, Grove, Ravel, herdr, Orca, mise and
`codebase-memory-mcp`, and the packaged Node forms of the three harnesses.
They also match the applications Claude, ChatGPT, Herdr and Orca. ChatGPT
holds Codex's `node_repl` MCP server. `OrcaSlicer` does not match.

The job listing runs `launchctl list` and `crontab -l`. `sfltool dumpbtm`
waited for authorisation, so the listing cannot read login items without
privileges. That is a limit of step 1.

The listing ran from this session, which Grove 22.8 launched through Ravel
1.1.0. The program's own process gate ran from the same position and named
the same processes and jobs. At that time, step 2 would have stopped for
19 processes and 3 jobs:

- The owner's Codex app-server daemon, which reads the Codex roots.
- `herdr-core` and `mise dot watch`, which write files of the switch set.
  The job `dev.mise.mise-history` runs `mise dot watch`, and two jobs
  `dev.herdr.server.*` can launch herdr.
- A Ravel 1.1.0 session of another task and two Grove builds in `/private/tmp`.
- Eleven Codex daemons of earlier trials in disposable HOMEs.

The owner ends these processes and unloads the jobs before the switch. The
finish handoff of discovery-rollout-k12 names them.

### Commands

| Field | Value |
| --- | --- |
| `state` | `.local/state/owner-switch` |
| `manual` | `Library/Mobile Documents/iCloud~md~obsidian/Documents/Brain/Configuration/AI Dev Tooling.md`. Its folder resolves inside HOME |
| `baseline` | `$python -I .local/state/owner-switch/prepared/inventory.py` |
| `gates` | The program's own gate of each of steps 4 to 7 |
| `probes` | `owner_switch_probes.py resolve --expect switched`, and `session --route ravel` for Claude Code and for Codex |
| `native_probe` | `owner_switch_probes.py resolve --expect disabled`, and `session --route native` for Claude Code and for Codex |
| `mac_check` | `.local/bin/mac-check` |

Each command runs in HOME, so a path relative to HOME reaches the prepared
folder. owner-switch-plan-k254 copies `inventory.py` and the probe program
beside the plan and the program. The instrument reads TOML through
`tomllib`, so it needs Python 3.11 or later. It runs under `$python`, the
interpreter of the switch program.

No probe program exists yet. The draft names its interface, and
`owner-switch-probes` implements it before owner-switch-plan-k254. That leaf
can change the interface. k254 then changes the commands of the plan.

### Provisional accepted values

`accepted` holds provisional values, and owner-switch-plan-k254 replaces them
with the evidence of owner-staging-acceptance-k252:

- `sources`: the ten sources of the table "The ten Ravel sources" in the
  [moves record](repository-moves-record.md#the-ten-ravel-sources). The
  script reads that table, so each `resourceDigest` is the recorded value.
- `versions`: the full package versions `personal` 2026-11-06_1,
  `testanyware` 2.8.0_1, `superpowers` 7.0.0_1 and `quint-llm-kit` 1.0.0_1,
  each a formula of `linkuistics/taps`. The script stops unless the table of
  [Publication](#publication) names each one.
  [Package identity and private payloads](#package-identity-and-private-payloads)
  states the change from the unrevised versions.
- `trials`: one command for each of Grove's six sources that prints a
  message and fails. A step 4 that must rerun a trial therefore stops.
- `staging`: `/Users/antony/.local/state/owner-staging/home` and
  `/Users/antony/.local/state/owner-staging/candidate`. owner-staging-setup-k248
  records the final paths.

### The check

The check builds a disposable HOME. It copies each entry that a part or an
`outside` item names from the real HOME, and each copy had the draft's change
identity. It writes both settings files with each cell at its earlier value.
The program's loader then admitted the draft, and `run --dry-run` reported the
state `native`. The output listed each of the 156 parts in the switch and in
the inverse, and it reported no passing rehearsal.

Each control must fail, and each did:

| Control | Result |
| --- | --- |
| The plan without `mac_check` | Exit 3, "the plan is incomplete" |
| A part path that is absolute | Exit 3, "is not relative to HOME" |
| A gate 4 list without the program's gate | Exit 3, "does not run the program's gate of step 4" |
| One added byte in a stale superpowers copy | The state `changed` |
| A new child of `.agents/skills` | The state `changed` |

Eighteen pattern cases state what each pattern list must match or must not
match. Three wider mutants failed them:

- A watch list of plain substrings failed seven cases.
- A job pattern without word boundaries failed two cases.
- An MCP pattern for any `node` failed one case.

| Subject | SHA-256 |
| --- | --- |
| `switch-plan-draft.json` | `c40451d63c33e1cb8800a80ddce6b8b4e21e534c4711ec5b8dd24a72249f0859` |
| `switch-plan-parts.tsv` | `ce2ad54ebdb49fe6ba82a6c1d60481cfb449587736d1e6024ff9830897377fd4` |
| `scripts/owner_switch.py`, unchanged | `885171751f02b62451090a4a41cc7b5145bb3aac29556d85b1dbcb262cc7884f` |
| `inventory.py`, unchanged | `b933ed455dc53af5c70ef9e2a7a3bc1389e32e413bbca8c4486d0c063c749ccc` |

The dry run reads no gate, no probe and no rehearsal record. The rehearsal of
owner-switch-rehearsal-k255 measures them on the completed plan.

<!-- tc: off because this section preserves the completed review's findings as history -->

## Owner switch plan draft review

Scope: this section describes the switch program before
owner-switch-contraction-k270. That leaf removed the old `run`, `inverse`,
`merge`, `pending`, `state` and `purge-check` entries, with the exchange,
held links, kept copies, cell inverses, rehearsal admission and the plan's
parts. The observations stay as history. They are not evidence for the
current program, which [Owner switch contraction](#owner-switch-contraction)
describes.

`owner-switch-plan-draft-k258` inspected producer commit
`9490761a9099b656630d0dd05c49c9721fa3ec8b` against the owner design. The
findings below describe counterexamples from the source. They are not
results of running the switch. No production or test code was changed.

### D1 — P1: command paths do not identify the rehearsed programs

The commands at
[`switch_plan_draft.py:101`](evaluations/owner-skill-delivery/switch_plan_draft.py)
and line 114 execute `owner_switch_probes.py` and `inventory.py` from the
real `<state>/prepared` directory. The plan records their paths but no
content identities. `load_plan` hashes the JSON only, `rehearsed` compares
the plan and main program digests, and a backup copies the main program.
`run_command_full` still opens the two external scripts at their live
paths. Those scripts are outside both `parts` and `outside`.

Consider a prepared plan that has passed rehearsal. Before its real run,
replace the probe script with a program that exits zero without observing
anything. Neither admitted digest changes, so step 8 still accepts the
record and step 10 reports successful commands. Replacing the inventory
script with one that emits no rows also bypasses the anomaly check. The
same substitutions affect a continuation of an existing backup: running
its saved main program does not restore either dependency. This breaks
the requirement that the real HOME receive the rehearsed operation and
that a continuation execute the operation preserved by its backup
([The rehearsed plan](specs/owner-skill-delivery.md#the-rehearsed-plan)).

### D2 — P1: job labels do not enumerate the programs that jobs can launch

The job command and patterns at
[`switch_plan_draft.py:96`](evaluations/owner-skill-delivery/switch_plan_draft.py)
read only the current launchd listing and crontab text. A launchd listing
holds labels, PIDs and exit statuses; it supplies no program arguments.
The patterns also omit Pi. Login items are not read at all, and every
crontab error is suppressed and converted to success.

A loaded, idle job labelled `dev.personal.maintenance` can run
`/opt/homebrew/bin/mise dot watch`. Its listing line contains none of the
patterns. With no current writer process, the gate admits a write interval
although that job can start its writer during the interval. A Pi job
labelled `dev.pi.agent` is another missed case. A failure to read crontab
also looks like no schedule. These are gaps in the observation, not
evidence that those particular jobs are installed today. The recorded
three matched jobs do not establish the absence of others. Step 1 requires
every enabled job, login item and schedule that can launch a named program;
step 2 must stop while one remains
([The switch inside finish](specs/owner-skill-delivery.md#the-switch-inside-finish)).

### D3 — P2: omitting two whole roots also omits their ordinary children

The `outside` choices at
[`switch_plan_draft.py:67`](evaluations/owner-skill-delivery/switch_plan_draft.py)
omit `.claude/skills` and `.codex/skills` because their platform containers
are mutable. That also omits the names of every other child not already
in `parts`. The control adds a child only to `.agents/skills`, a root that
does have a manifest.

For a concrete changed HOME, add `.codex/skills/quint-lang` as a link to
the retained Quint clone after drafting. That path is absent from the
draft's parts. `outside_changes` never reads it. The selected inventory's
`codex_cls` classifies it as a known third-party entry, with disposition
`deactivate; ravel:quint`, rather than emitting an anomaly. Step 3 can
therefore admit the native state while step 9 leaves a superseded native
copy active. The new copy depends on a later discovery probe to stop the
switch, and the manifest still cannot detect it at step 14. Changes inside
`synced` or `.system` do not justify ignoring their ordinary siblings.
This leaves a gap in the one-active-copy rule and the comparison of entries
outside the switch set
([Name collisions and stale copies](specs/owner-skill-delivery.md#name-collisions-and-stale-copies),
switch steps 3 and 14). D5 is a separate continuation failure, not a
replacement for this missing observation.

### D4 — P2: the MCP list grants a non-server a kept-process exemption

The pattern at
[`switch_plan_draft.py:86`](evaluations/owner-skill-delivery/switch_plan_draft.py)
labels `caffeinate -i -t <digits>` as an MCP server. The producer's
committed process listing shows that this exempts the harness child
`caffeinate -i -t 300`. The program follows matching children recursively,
so the exemption is executable, not merely a comment about future work.

The design permits only the harness, its driver chain, actual MCP servers,
and the program and shell. It expressly counts every other descendant as
another process. `caffeinate` is not a server, and ending it need not end
the session. The draft therefore implements a different exclusion rule
from [Concurrent writers](specs/owner-skill-delivery.md#concurrent-writers).
The producer has acknowledged the difference, but a proposed design change
in a brief does not settle it. Nor does this one exemption establish a
rule for another helper child: an unmatched `/bin/sleep 300` child still
closes the interval. No harmful write by `caffeinate` is alleged; the
defect is the unresolved, load-bearing change to the exclusion contract.

### D5 — P1: the chosen inventory rejects the switched guardrail plugin

The plan's `baseline` at
[`switch_plan_draft.py:114`](evaluations/owner-skill-delivery/switch_plan_draft.py)
names the unchanged inventory instrument. Its plugin-skill classification
at [`inventory.py:164`](evaluations/owner-skill-delivery/inventory.py)
recognizes the old `linkuistics` marketplace, superpowers and Quint. For
another enabled plugin with a `skills` directory, it emits
`UNCLASSIFIED`. The personal plugin has `skills/guardrail/SKILL.md`.

After step 9 enables `guardrail@personal`, stop the program before the
receipt. On the next run, step 3 runs that inventory with guardrail enabled
and collects its unclassified row. The cells still describe the active
state, but `continue_after_switch` treats the collected row as an outside
change and calls `failed_probe`, which applies the inverse. A correct
activation thus fails its required active continuation solely because the
instrument does not recognize the intended native residue. A stop after
the receipt but before its copy likewise makes step 14 fail. The dry run
never executes the baseline, so its passing record does not cover this
case. This contradicts the guardrail disposition and the continuation
contract
([Native residue](specs/owner-skill-delivery.md#native-residue),
[Resuming the switch](specs/owner-skill-delivery.md#resuming-the-switch)).

### Other doubts and limits of this review

Both new plugin keys are absent in the real HOME at review time. Their
earlier values of `false` are nevertheless consistent: cutover order 6
requires `guardrail` installed and disabled, and the hooks prelude installs
and disables the other plugin before step 9. The disposable check writes
those expected values; it does not prove that preparation has occurred.

No false reversal caused by an ordinary read of the eight `outside` items
was established. The change identity hashes content, entry names, types,
executable bits and link targets; it omits timestamps and follows no link.
Reading the snapshot store or acquiring its empty lock therefore supplies
no demonstrated content change. The tool owns updates to codebase-memory
copies, agents and Pi adapters. An actual installer write can change an
identity, but step 14 currently requires that change to fail the probe.
No update or probe was launched here, so this review makes no claim that
the kept server or a normal probe necessarily rewrites one of those items.

The provisional accepted values, failing trial placeholders and absent
probe implementation are declared handoffs to later leaves. Their absence
is not a separate finding against this draft. D1 concerns what the eventual
commands identify, regardless of the implementation they receive.

This was a task-directed Verify review using graph project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-07T12:56:06Z`. The bounded symbol searches returned no further
pages. An inbound trace of `outside_changes` identified the state,
continuation and step-14 consumers; exact source controlled each claim.
Coverage reported no recorded gap in the cited code. TSV and text evidence
had untracked freshness metadata and were read directly. That signal is
not proof of completeness. The personal plugin's local layout, the selected
real-HOME keys, directory names and relevant launch-agent definitions were
also read without changing them.

The inspected plan and part table have SHA-256
`c40451d63c33e1cb8800a80ddce6b8b4e21e534c4711ec5b8dd24a72249f0859`
and `ce2ad54ebdb49fe6ba82a6c1d60481cfb449587736d1e6024ff9830897377fd4`.
They match the producer's recorded subjects. The main program and inventory
digests also match that record. The generator digest is
`9b13ac3122a74a5b640c418e26af4b4e0fbb482f7701ac8117109d0bd63b4b26`.
No test, build, lint, format, check task or switch operation was run by this
review. Integration owns fixes and executable verification of the cases.

<!-- tc: on -->

## Owner switch plan draft integration

Scope: this section describes the switch program before
owner-switch-contraction-k270. That leaf removed the old `run`, `inverse`,
`merge`, `pending`, `state` and `purge-check` entries, with the exchange,
held links, kept copies, cell inverses, rehearsal admission and the plan's
parts. The observations stay as history. They are not evidence for the
current program, which [Owner switch contraction](#owner-switch-contraction)
describes.

`owner-switch-plan-draft-k259` triaged the five findings of the review above
on 2026-11-08. It verified each finding against the source, and each was
real against the current design. It then prototyped a repair for each
finding in disposable layouts. The repairs passed their checks. The real
HOME stayed read only.

The human then stated two facts: this grove is the only grove that runs,
and the human is the only user of Grove and Ravel. The human asked for a
simpler switch on that basis, and chose to drop the repairs. The human chose
four simplifications: the process and job exclusion, the concurrent-writer
detection, the recovery state machine and the rehearsal admission.
`owner-switch-simplification-k260` redesigns the switch. Its review and a
planning leaf follow it, before the staging leaves.

| Finding | Disposition |
| --- | --- |
| D1, unbound command files | Real. Superseded by the redesign, which must still identify each file that a certified plan runs |
| D2, job labels only | Real. Superseded by the redesign, because an owner checklist replaces the automatic job listing |
| D3, two omitted roots | Real. Superseded by the redesign, which must still see a new sibling of a platform folder |
| D4, a helper as an MCP server | Real. Superseded by the redesign, because the automatic process listing goes |
| D5, the switched guardrail plugin | Real. Superseded by the redesign, which must still let the instrument accept an enabled `guardrail@personal` |

The section "Other doubts and limits of this review" raised no further
finding.

The prototypes measured three facts, which k260 carries in its task file:

- During a turn, Claude Code runs `caffeinate -i -t 300` as a child of its
  harness process. That child writes no file.
- launchd reads the account's own home, whatever HOME names. A job listing
  that reads `~/Library/LaunchAgents` through HOME misses the owner's jobs
  in a disposable layout.
- `launchctl print gui/<uid>/<label>` shows the program arguments of each
  loaded job. All 577 jobs took about 2 seconds.

## Single-owner switch design

This section records k260. The integration below repairs its reviewed contract.

owner-switch-simplification-k260 uses the human's sole-user and sole-grove
facts to replace four mechanisms. The initial specification required a quiet interval, one complete backup before all writes and ordinary serial writes.
It requires full restore on failure and one disposable execution of the real plan.
This is a design change only. No new switch or actual owner cutover is
measured by this section.

| Finding | Design disposition |
| --- | --- |
| D1 | Keep content identities for the plan, program, invoked scripts, local helpers and command inputs, with tool paths/versions and dry-run substitutions. A path alone certifies nothing. Remove runtime rehearsal admission. The owner reviews the result and keeps the prepared inputs unchanged. |
| D2 | Remove automatic job classification. The owner checklist names current writers and relaunch mechanisms, including mise, herdr, Orca and the codebase-memory updater. The narrow harness/driver process check does not establish their absence. |
| D3 | Compare ordinary children of both native skill roots. Exclude only `.claude/skills/synced` and `.codex/skills/.system`, not their parents or siblings. Probe changes inside those exact subtrees are expected. New superseded siblings still fail. |
| D4 | Remove MCP regex exemptions. The narrow process check excludes exact finish ancestors, not arbitrary matching descendants. Non-harness `caffeinate` needs no exemption. |
| D5 | Classify enabled `guardrail@personal` as the deliberate native exception only at the personal keg origin and with only the guardrail skill. Another skill or origin fails. |

The design keeps one active route per owner-selected name, finish's absolute
guidance paths, frozen current plugin registrations and the original Ravel
1.1.0 keg. Native restore explicitly turns Ravel sources off after promotion. Restoring an absent `skills` key would enable the default sources beside
native skills.

<!-- tc: depart synonym-rotation because "Verify-tier" names the evidence tier, not the action "check" -->
The source inspection used Verify-tier graph evidence for
`scripts/owner_switch.py`: `run_switch` calls process inspection, preludes,
state lookup, rehearsal admission and backup, with partial continuation and
program re-execution. `held_command` implements the held-link, metadata and
inode inspections. The inspection read both call directions for `run_switch` at depth one.
It included tests and left no remaining page.

Coverage at generation
`2026-11-08T03:42:48Z` reported no recorded issue for the program, rehearsal,
draft generator and inventory paths. This coverage is best effort, not a
completeness proof. The relevant spec, live leaves and earlier review findings
were read directly.

The simplified program is not implemented yet.

Its dry run must measure the
complete command write set, metadata and quiet-session assumption. A final
comparison can detect a different resulting layout. It cannot detect an edit
that an ordinary write already overwrote. A killed program cannot restore
while dead.

The retained restore entry must work after restart, and a failed
restore must preserve its backup and prevent success. The new design makes
these limits explicit instead of applying the old model's claims to it.

The existing owner-switch-simplification-k261 review challenges the design.
The parent rollout brief gives k262 artifact and leaf consequences. No
implementation leaf or machine configuration was changed in this session.

The existing localhost viewer served the visual document's manifest and three changed diagram sources.
They matched the working files. Browser
access was unavailable, so this session did not inspect their rendered
layout. The editable sources and stable discussion links remain available
for the scheduled review.

## Owner switch simplification review

`owner-switch-simplification-k261` inspected producer commit
`eccf8b044d83041867e71072a17305f274d091dc` against the human's two facts.
This grove is the only grove that runs, and the owner is the only user of
Grove and Ravel. The subjects are the owner design, the coordinated recovery
rows, the finish decision record and the three changed diagram sources. Each
finding states a defect, a state of this machine that shows it, and the rule
that the state breaks. No finding proposes a repair.

### S1 — P1: a stop before the backup leaves no native return after promotion

Switch step 5 writes the Ravel off value after the backup of step 4. Steps 1
to 4 can stop without a write, by design. Ravel's promotion comes before
step 1. In that interval the real `~/.config/ravel/settings.json` does not
exist, so promoted Ravel 2.0 delivers the ten default sources. The native
plugins and links stay active beside them.

Consider this state. Finish releases Grove and promotes Ravel. Step 2 finds
that a released source differs from its accepted digest, and the staging
rerun fails. The attempt stops before step 4, so no backup exists, and
`restore --backup <directory>` has no directory to read. A fresh attempt
stops at the same step until a Grove patch release.

That release can launch a default session itself. When its notes are
missing, `task release:patch` runs `grove run release-notes` through
[`release-notes.sh`](../scripts/release-notes.sh). That launch then receives
every owner skill twice. An incomplete checklist item leaves the same state,
because "the switch waits".

[The native settings inside finish](specs/owner-skill-delivery.md#the-native-settings-inside-finish)
names this state. It forbids a new default session until someone verifies
the native return or a successful switch. As a route, it offers the owner's
recovery entry, and that entry needs a backup. The coordinated row "Promotion complete, owner
switch failed or was interrupted" also begins from "the complete backup".
The switch-states view sends a run without a write to "Repair preparation".
The cutover view calls it "Stop before mutation".

The state breaks the one-active-copy rule of
[Name collisions and stale copies](specs/owner-skill-delivery.md#name-collisions-and-stale-copies).
The earlier design did not reach it, because its preludes wrote the off value
before its verification steps. k260 moved that write behind the backup.

### S2 — P1: the Codex probe leaves a Codex process that the narrow process check refuses

[The owner's checklist](specs/owner-skill-delivery.md#the-owners-checklist)
admits finish's live ancestors and the recorded probe processes, and nothing
else. It states that "a delegate harness or app-server therefore stops the
attempt". The program repeats the narrow process check before restore. It
also waits for the probes to exit before its final check.

The recorded and live process tables show what Codex leaves in a fresh
`CODEX_HOME`. Each trial installed `packages/app-server-daemon/releases/<version>`
there. Each left `codex app-server daemon pid-update-loop` with parent PID 1,
and a child `codex app-server ... --managed-daemon`. On 2026-11-08 six
disposable homes still had such processes, the oldest after about 21 days.
The real HOME runs the same daemon from `~/.codex/packages/app-server-daemon`.

Consider the real switch. The owner obeys checklist item 3 and stops the real
daemon. Step 7 launches one Codex probe in the real HOME. Each recorded trial
in a home without a daemon left one, so the probe can launch the daemon
again. After the probe exits, the daemon is neither a finish ancestor nor a
live descendant of the program.

The final check then refuses a switch whose probes passed. The check before
restore refuses the same process, and the rule "Never signal another
session" forbids the program to end it. The attempt ends as recovery
incomplete, whatever the probes observed. The disposable dry run shares the
machine's process table, as it shares launchd, so it meets the same daemon.

The state breaks the rule that a failed probe restores native delivery. It
also breaks cutover order 7, where the probe results decide success. The
test seam that admits only exact finish ancestors assumes that each probe
exits with its children. No trial measured that assumption for Codex.

### S3 — P1: whole restore and whole comparison outlast the quiet interval

The "Recovery" row of
[Four simplifications](specs/owner-skill-delivery.md#four-simplifications)
rests on "no concurrent actor's change to merge". That fact holds for one
quiet interval. The same `restore --backup <directory>` also serves "the
owner's later return to native delivery" until purge. Restore replaces whole
backed-up files. The write set includes `~/.claude/settings.json`, the plugin
registries, the Ravel settings and the maintained manual.

Consider this state. The switch succeeds on day 0. On day 5 the owner runs
`/model`, which writes `model` into `~/.claude/settings.json`. The
codebase-memory updater rewrites its hook entries in that file. The owner
adds a note to "AI Dev Tooling".

On day 7 the owner returns to native delivery. Restore accepts the unchanged
backup and writes the day-0 files back. The model key, the updated hooks, the
new note and the receipt disappear without a report. Restore reports
"unrelated entries outside the write set", but these edits are inside
write-set files.

The same gap affects `mac-check` and the purge report. The plan drops its
cells, and step 8 compares "the complete expected switched configuration".
After the interval, nothing in the plan separates a switch value from an
unrelated key. `mac-check` must then report "restore/review needed" after
each ordinary settings edit, or call a layout switched without a stated
basis. The first reading invites the restore above, and blocks purge.

The two facts remove other writers during one operation. They do not remove
the owner's own later edits, so the row rests on more than those facts.

### S4 — P2: the dry-run comparison has no stated form, and step 2 contradicts it

[The rehearsed plan](specs/owner-skill-delivery.md#the-rehearsed-plan)
lists the changes that need another dry run. They are "a changed plan,
program, helper, command input or tool version". It says that "Finish compares these inputs with the real
attempt's prepared inputs". It names no instrument, no compared record and no
rule for an allowed substitution. The "Dry run" row justifies the removal of
admission by this comparison.

Three states show the gap:

- Step 2 says to rerun the affected staging trials when a released Grove
  source differs, and to "accept the new result before proceeding". The plan
  holds the accepted source versions. The hooks installation installs the
  `writing` source as a plugin. A changed source therefore changes the plan
  or a command input, which requires a new dry run before the attempt
  proceeds. The coordinated row for a failed core trial agrees with that
  rule, not with step 2.
- Cutover order 6 runs the dry run before Grove's release and Ravel's
  promotion. Step 5 registers `writing-hooks` from Grove's released `opt`
  path, which holds no such marketplace before the release. The dry run must
  read the candidate keg and the keg-only `ravel@2` instead. The design
  records "the substituted prefixes" but does not say whether a new prefix is
  a substitution or a changed input. If it is a changed input, each real
  attempt needs a dry run inside finish, and no order schedules one.
- After an unsuccessful finish, the owner may run a fresh switch. No text
  names the actor of the comparison then.

D1 asked that a certified plan identify each file that it runs by content.
The design records those identities. It does not bind the real attempt to
them, so the record satisfies D1 and the comparison does not yet.

### S5 — P3: no observation covers the live finish process in steps 6 and 7

The design removes exchange and held-link detection. Two obligations replace
it: a staging session stays idle "through the installation", and the owner
makes no settings action. Both end with step 5. In step 6 the program
disables the plugin that holds finish's guidance and enables two plugins.
These writes go under finish's live Claude Code process. In step 7, two
probe harnesses launch in the real HOME.

The Homebrew evidence found about 40 sites in Claude Code that write the user
settings, `pluginConfigs` among them. The dry run cannot put a live parent
harness over its disposable settings, because its operator's harness reads
the real HOME. The staging trial that disables a session's own plugin records
reads, not writes. A write by finish's process in step 6 can fail a valid
switch at step 8. Or the next ordinary write of the program removes it
without a report. The k261 doubt names this writer, and the design covers
only the owner's actions in it.

### S6 — P3: the owner's recovery after an unsuccessful finish lacks named inputs

Each attempt makes a new backup directory, and "no run log or program copy
selects a recovery route". After a crash, "the owner retains the backup
location". The design does not say where that location survives when finish
ends before it reports. Restore checks the complete backup, its content
identities and its confined destination paths. It does not check that the
backup belongs to the attempt that wrote the live layout.

With two attempts, the owner can restore the earlier backup. That restore
reverses each change between the attempts without a report. The fresh switch
that the owner may run after finish also lacks a substitute for finish.
Steps 1, 3 and 8 name finish's ancestors, its loaded cache paths and its
guidance. The earlier design let one Claude Code session take "finish's place
in steps 1, 3 and 7". k260 removed that sentence and wrote no replacement.

### S7 — P3: the manual gains whole-file restore without its writers

Step 9 adds the maintained manual to the backup write set. The restore
operation puts back whole backed-up files. Step 9 also says that a failure
"withdraws any success entry created by this attempt". The design does not say which of the
two operations applies to the manual. The plan's `manual` is an Obsidian note
in iCloud Drive, under `Library/Mobile Documents/iCloud~md~obsidian`.

The checklist names no writer of that note. Obsidian ran during this review.
iCloud delivers edits from the owner's other devices, and the checklist
names no pause for it. A whole-file restore discards such an edit. The
earlier design kept the manual append-only. k260 drops that rule and gives
no reason, although the rule does not depend on other actors.

### S8 — P3: the checklist asserts unprivileged inspection of login items

The checklist says that "the owner can inspect account-owned items without
privileges", and it lists Login Items with launch agents and schedules. The
only recorded attempt, `sfltool dumpbtm`, waited for authorisation, as the
section "Processes and jobs" states. The design records no other route that
lists each login item and background item with its program.

Jobs show a second gap. The herdr jobs `dev.herdr.server.<pid>.<time>` exist
only in launchd. `~/Library/LaunchAgents` holds `dev.mise.mise-history.plist`
and no herdr file. The checklist tells the owner to inspect LaunchAgents and
to print a known label, but not to list the loaded jobs. An item that the
owner cannot complete makes "the switch wait", and S1 then applies.

### S9 — P3: the narrow process check has no recognition rule

The design removes the patterns. It says that the program checks "other
Claude Code, Codex, Grove and Ravel processes", and not how it recognises
them. The plan draft evidence found native binaries, packaged Node forms of
the three harnesses and the applications Claude and ChatGPT. ChatGPT holds
Codex's `node_repl` MCP server. Checklist item 1 names "their desktop
applications", and neither the program nor the checklist names ChatGPT. The
test seam needs this boundary to build its fixtures.

### Doubts without a finding

- **The four removed mechanisms.** None defended only against another user
  or another grove. The listing and the detection covered the owner's tools,
  the finish process and Claude Code's own commands. The recovery states
  covered crashes, and admission covered drift before the real run. The
  human chose the checklist and the final comparison for the first two.
  S1, S3, S4, S5, S6 and S7 name each place where a removal rests on more.
- **The checklist's programs.** The checklist names each program of the
  earlier step 1, except ChatGPT, as S9 states. It narrows `mise` to
  `mise dot watch`, bootstrap and upgrades. The help of `mise dot watch` says
  that it saves checkpoints, and applies incoming changes only in `sync` mode
  with an origin. `mise dot origin` reports mode `manual` on this machine.
- **D3.** Resolved. Step 1 refuses an entry outside the expected native
  layout, and step 8 compares the expected switched layout. The design skips
  only `synced` and `.system`. Today they hold platform skills only, such as
  `pdf`, `docx` and `skill-creator`, and no Ravel source holds those names.
- **D5.** Resolved. The inventory accepts an enabled `guardrail@personal`
  only from the personal keg's marketplace with its sole skill. The dry run
  now runs the final comparison that D5 needed.
- **The final session's guidance.** The design keeps finish's loaded versions
  registered, keeps the update freeze and the full 1.1.0 keg, and rereads the
  guidance by path. The review found no route that removes that guidance
  before the final signal, apart from the process stops of S2.
- **The manual's attributes.** The note carries `com.apple.macl` and
  `com.apple.provenance` in the `xattr` listing. Copies by `cp -p` and
  `ditto` kept both, so they do not block the exact-metadata backup.
- **Grove link cleanup.** The Grove delivery design adds no automatic
  migration or cleanup command. Restored Grove 22.8 Codex links therefore
  stay in place under the released Grove.
- **The launchd boundary.** `launchctl list` names only the mise and herdr
  jobs, so the Codex daemon is not a launchd job. S2 concerns the shared
  process table instead.

### Review evidence and limits

The review read the producer commit and the owner design before and after
it. It read the coordinated release design, the finish decision record, the
diagram sources and the parent brief's consequence tables. It read the
earlier review and the plan draft evidence with its process listing. The
real HOME and the machine stayed read only. The review read the live process
table, `launchctl list`, `~/Library/LaunchAgents`, the two excluded skill
subtrees and the attributes of the switch-set files and the manual.

The attribute copies went to a scratch folder outside the repository. The
review ran no test, build, lint, format, check task or switch operation. It
did not open the rendered visual document. The integration owns each repair
and its executable verification.

| Subject at the producer commit | SHA-256 |
| --- | --- |
| `docs/specs/owner-skill-delivery.md` | `365121757acef08c463509c466d5d98fdb83f2b7a4d707ba271b6658d4651f2d` |
| `docs/specs/coordinated-release.md` | `0204f83ac4301ae89c347ae34c325bcbb44f18ed16edaa44d207e972507d23ee` |
| `docs/adr/finish-promotes-after-the-grove-release.md` | `de4ede4ad4cbed561ca5235850119260f082677b0092f2e5f77658e8559aa7e9` |
| `owner-cutover.mmd` | `d358abc1490163b64b65f37a6c6b9e5786dd2c4f77efc9f231091aa03c9b9e42` |
| `owner-switch-states.mmd` | `887c633ae2c460510cc47af5cb3f380dc6c2d0f36848c2c17b0b3cf0b916f6a7` |
| `owner-final-session.mmd` | `6acdb89ee7879c607253c5f23e40e7d55eac8aded3d169c53b562807d31d7617` |


## Owner switch simplification integration

`owner-switch-simplification-k263` read S1 to S9 from review commit
`bb0457986498ea5768a42a4d610e98c72e284029`. It compared those findings with the
owner specification and k260's record of the human's four choices.
The real HOME, launchd and process table remained read only.
The integration changes design documents and the planner's consequence tables.

### Dispositions

| Finding | Classification and source-supported disposition |
| --- | --- |
| S1 | Real issue. The reviewed off write followed preflight and backup, while promotion preceded both. A preflight refusal therefore had no backup to restore. The [native settings contract](specs/owner-skill-delivery.md#the-native-settings-inside-finish) now requires `native-off` immediately after promotion. Its retained command works without source acceptance or a switch backup. It changes only the off key and preserves unrelated settings and metadata. Ordinary switch writes still require the complete backup. |
| S2 | Real issue. The review records Codex daemons surviving probe exit and reparenting to PID 1. Direct-child reaping cannot establish their absence. [Probe lifetime](specs/owner-skill-delivery.md#probe-process-lifetime) requires a measured launcher that prevents persistence or proves service ownership and ends it through a supported route. Unknown and pre-existing daemons remain unrelated. No real attempt proceeds without that acceptance evidence. |
| S3 | Real issue. Whole-file restore would overwrite later unrelated keys, hooks and notes. [Rollback](specs/owner-skill-delivery.md#rollback) now refuses later whole-file drift before writing. Delivery assertions separately establish current health. Healthy delivery can coexist with ineligible restore. Wanted edits require a reviewed native-return plan, never an automatic merge or forced overwrite. |
| S4 | Contract stated unclearly. D1 supplied content identities but no reproducible comparison or permitted substitutions. [The rehearsed plan](specs/owner-skill-delivery.md#the-rehearsed-plan) now retains a per-input table and comparison command. Only named HOME and release-prefix fields may vary with identical content. Changed source acceptance requires staging and another dry run. The owner performs comparison after finish. |
| S5 | Real issue. The initial observation ended with installation, before native deactivation and probes beneath the live parent. [Claude Code's writes](specs/owner-skill-delivery.md#claude-codes-own-writes) now requires observation through receipt completion, including intermediate overwritten writes. That parent uses the staging HOME and held harness version. A quiet final file alone establishes no such evidence. |
| S6 | Contract stated unclearly, with an explicit operational trade-off. Backup data cannot infer which interrupted invocation the owner intended. [Recovery](specs/owner-skill-delivery.md#resuming-the-switch) now retains immutable attempt instructions before mutation and binds the supplied backup to that note and HOME. Uncertain attribution refuses. A replacement operator supplies fresh ancestry and reader evidence after finish. No mutable progress state or latest-backup inference is introduced. |
| S7 | Real issue. Whole-manual restore could remove unrelated Obsidian or iCloud edits. The [switch contract](specs/owner-skill-delivery.md#the-switch-inside-finish) restores append-only observations and excludes the manual from whole-file recovery. Failure invalidates the authoritative receipt and appends a correction. The checklist pauses manual writers and synchronisation. Unwritable manual evidence prevents success without preventing native configuration recovery. |
| S8 | Real issue. The recorded login-item command required authorisation, and loaded herdr jobs lacked LaunchAgents files. [The checklist](specs/owner-skill-delivery.md#the-owners-checklist) now names loaded-job inspection and System Settings, with explicit inspection limits. It promises no exhaustive unprivileged command. Incomplete preparation stays in verified native delivery. |
| S9 | Contract stated unclearly. The narrow inspection had no recogniser contract. [Process recognition](specs/owner-skill-delivery.md#the-narrow-process-inspection) now covers executable identities, measured packaged forms, Node entry scripts and Codex daemon subcommands. Unknown forms require review. Claude and ChatGPT also appear in the owner's desktop-app checklist. |

All four chosen simplifications remain. None of these repairs restores broad job
classification, concurrent-edit merge, partial resumption or digest-bound rehearsal
admission. The `native-off` operation is a bounded safety exception before switch
backup, needed because promotion enables sources before switch preflight.
The finish decision record, coordinated recovery rows and visual views reflect
that exception and the later-restore boundary.

### Narrow review and limits

One fresh-context reviewer inspected the native settings, rollback and recovery
sections for recovery-safety defects. It found that an interrupted `native-off`
write could damage settings before any switch backup existed.
The contract now requires a complete temporary file and atomic replacement,
with original-or-complete interruption controls. This preserves ordinary serial
publication and adds no displaced-copy or merge mechanism.

The integration implements no switch code and establishes no actual activation.
Probe-service lifecycle, quiet-parent observation, file publication and input comparison
remain explicit implementation and acceptance work. The planner's artifact and per-leaf
tables assign that work to k262 and its consumers. Earlier rehearsals certify only
the programs and inputs that they actually ran.


### Integration validation

The edited prose passed the controlled-English checker with no unaccepted hard findings.
Local validation resolved 122 relative links and anchors across the edited Markdown.
The same predicates rejected a deliberately missing path and anchor.

The viewer manifest resolved all 19 diagram sources and its discussion references.
The existing localhost viewer served the manifest and three changed diagram sources
byte for byte from the working files. Digests of the eleven validation subjects
stayed unchanged during that run. The subjects were the seven edited Markdown
files, the manifest and the three changed Mermaid sources.

The session inspected diagram sources, not rendered layout. It ran no switch,
real-harness trial, code-suite check or owner configuration operation.
Those checks belong to the implementation and acceptance work named above.

## Owner inventory exceptions

`owner-inventory-exceptions-k264` applied the two inventory rules of
[Concurrent writers](specs/owner-skill-delivery.md#concurrent-writers) on
2026-11-08. These rules answer D3 and D5 of the
[plan draft review](#owner-switch-plan-draft-review). The real HOME stayed
read only.

### The changes

- [`inventory.py`](evaluations/owner-skill-delivery/inventory.py) recognises
  a platform subtree only at its exact root: `~/.claude/skills/synced` and
  `~/.codex/skills/.system`. A child with either name in the other root is an
  ordinary child. Each other hidden child of a root is an ordinary child too.
  A platform subtree that is not a plain directory gets an `UNCLASSIFIED` row.
- The instrument's new `compare BEFORE AFTER` command keys each row by its
  harness, location and entry. It skips only the rows inside the two
  subtrees. It fails on each added, removed or changed key and on each
  anomaly row of the second inventory. `task owner:inventory:compare` runs it.
- An enabled `guardrail@personal` is the native residue only under two
  conditions. The `personal` entry of `known_marketplaces.json` must have the
  source directory `/opt/homebrew/opt/personal`. The plugin's `skills/` must
  hold only `guardrail`. Otherwise its rows say `UNCLASSIFIED`.
- `outside_changes` in
  [`owner_switch.py`](../scripts/owner_switch.py) skips the same two
  subtrees, each only while it is a plain directory. A link in place of
  either is still compared.
- [`switch_plan_draft.py`](evaluations/owner-skill-delivery/switch_plan_draft.py)
  adds `.claude/skills` and `.codex/skills` to the `outside` roots. Their
  entries name neither platform subtree. The committed
  `switch-plan-draft.json` keeps the earlier form until
  owner-switch-plan-k254 regenerates the plan.

### Controls

[`controls.py`](evaluations/owner-skill-delivery/controls.py) passed 37 of 37
cases. Each case also checks the verdict of `compare` against the unchanged
run. The new cases are these:

- A superseded skill beside each subtree fails. The cases cover a directory
  and a link to the Quint clone at each native root. They also cover a nested
  copy in an ordinary child and a hidden child.
- A link in place of `synced` fails. A child named `synced` in the Codex root
  fails, and so does a child named `.system` in the Claude Code root.
- A new child of `~/.agents/skills` and a new Pi skill fail.
- A changed skill inside `synced` and a new skill inside `.system` pass. The
  rows of that subtree still change, so the exclusion passes them.
- The native guardrail from its sole-skill personal plugin passes. An extra
  skill, another origin and an absent registration fail. A `guardrail` skill
  from another marketplace fails.

The fixture now holds a codebase-memory executable and a Quint clone with a
fixed commit date. The unchanged run therefore has no anomaly row, and the
verdict of `compare` depends only on the case. The fixture's temporary
directory is a resolved path, because macOS links `/var` to `/private/var`.

The instrument before this change stops on the unchanged fixture. It reports
the guardrail skill of the valid personal plugin as `UNCLASSIFIED`, which
reproduces D5.

### Mutation controls

Each deliberate mutation of a scratch copy made its control fail:

| Mutation | Controls that failed |
| --- | --- |
| `compare` also skips both parent roots | 9 cases beside the subtrees |
| `compare` skips nothing | The changes inside `synced` and `.system` |
| The walker skips hidden children again | The hidden child beside `.system` |
| The walker follows a link in place of `synced` | The link in place of `synced` |
| The mutant drops the origin rule | Another origin and an absent registration |
| The mutant drops the extra-skill rule | The extra skill in the guardrail plugin |

The rehearsal gained the controls `outside-platform-subtree` and
`outside-platform-link`. With `--only '^outside-'`, 4 of 4 controls passed.
Three program mutations each failed a control. Skipping nothing failed
`outside-platform-subtree`. Skipping each child of both roots failed the two
earlier outside controls and `outside-platform-link`. Skipping a link in
place of a subtree failed `outside-platform-link`.

The draft generator ran in a scratch directory against the real HOME. Its
`outside` held both native roots, each with only `codebase-memory`. Its
`check` passed seven controls. The new `new-sibling-of-system` control must
change the state, and `inside-platform-subtrees` must leave it native.

### The real HOME

`task owner:inventory` gave byte-identical output before and after the
change, with 446 rows. `compare` of the two runs found no difference and
skipped 36 rows inside the platform subtrees. These were the SHA-256 values
of the measured files, which stayed unchanged during the final runs:

| File | SHA-256 |
| --- | --- |
| `inventory.py` | `6c5195e75f3df5269bec964effead080772107a5270055d9e2af378fbeb4300f` |
| `controls.py` | `c61749d9692d4688c2145040e47123c1a94940d3a4e36304b9d074bd155d33d3` |
| `owner_switch.py` | `706cb2bd9f02d4a0585589c50048f2a10e75b4fc13019a7c592271b83f260edd` |
| `owner_switch_rehearsal.py` | `ac1427165d61d8208229b1841daeb556305631bc129b8399ec05464035ca6d5c` |
| `switch_plan_draft.py` | `9064d63f29bf28d41368d77db858bf4915c24b94b5efb4db146f9badb5441807` |

### Limits

- The personal keg is not installed in the real HOME, so this session
  observed no real `guardrail@personal`. The registration shape comes from the
  Grove keg trial of [The hook adapters from a keg](homebrew-skill-delivery-evidence.md#the-hook-adapters-from-a-keg).
  The staging trial of the native guardrail must check it.
- The sole-skill rule reads the `skills/` of the installation path that
  `installed_plugins.json` records. Claude Code runs a directory
  marketplace's plugin in place through the `opt` path. A difference between
  the keg and the cache copy is outside this rule.
- The origin rule names the Homebrew prefix `/opt/homebrew`, as the tool rows
  do.
- A disabled `guardrail@personal` stays outside the active collection, and
  the origin rule does not apply to it.

## Owner switch model removal

`owner-switch-model-removal-k265` removed `docs/models/owner-switch.qnt`, its
runner `docs/models/run-owner-switch.sh` and the task `model:owner-switch` on
2026-11-08. [The recovery model](specs/owner-skill-delivery.md#the-recovery-model)
states the reason. Before the removal, the two files had the digests of the
last table in [The recovery model](#the-recovery-model). `scripts/check.sh`
ran neither file. No other model or task changed.

### The changes

- `Taskfile.yml` no longer has the task `model:owner-switch`.
- The module docstring of `scripts/owner_switch.py` no longer names the model
  as the check of its recovery rules. The docstring of `known_cells` no longer
  cites the model's merge.
- The specification section "The recovery model" states that the repository
  no longer holds the model.
- This document's section [The recovery model](#the-recovery-model) now
  states its historical scope first. Its observations, tables and digests stay
  unchanged.

### The search

The session searched the working tree from its root with ripgrep. The search
included hidden files and excluded `.jj`, `.git` and `target`. It used two
patterns:

- The names of the subjects:
  `owner-switch\.qnt|run-owner-switch|model:owner-switch`.
- Prose that names the model, without regard to case:
  `recovery model|Quint model|checked model|model of the owner switch`.

The session classified each matching file:

| Class | Files | Disposition |
| --- | --- | --- |
| Historical evidence | This document | Kept, with the scope statements above |
| Current documents that state the removal | The specification, the docstring of `scripts/owner_switch.py` and the viewer manifest of `docs/design/harness-selection-and-execution`, whose caption claims no checked model | Kept |
| Retired task files | The `DONE` leaves of the switch design, review and implementation | Unchanged |
| Live briefs | `_discovery-rollout.md`, whose table orders this removal, and `_owner-switch-program.md` of the closed node `owner-switch-program-k91` | Unchanged and reported |
| This leaf's task file | `18-impl--owner-switch-model-removal-k265.md` | Unchanged |
| Other subjects | `plugins/README.md`, which describes the `model-led-development` skill, and `docs/formalism-findings.md`, which describes other models | Unrelated |

No current document, docstring or task description presents the model as
acceptance evidence. The brief of `owner-switch-program-k91` still says that
the model must stay green. That node has no live child, so no live leaf
requires the model.

### Controls

- The positive control: the name search found the retired task file
  `36-DONE-design--owner-switch-recovery-k87.md`, which names the model.
- The same search without hidden files found nothing under `.grove/`, so the
  flag decides whether the control passes.
- A mutated pattern, `owner-switch\.qntX`, found no file.

## Owner switch native-off

Scope: the suite counts in this section include controls of the old `run`,
which owner-switch-contraction-k270 removed with that entry. The
observations stay as history. [Owner switch
contraction](#owner-switch-contraction) holds the result of the current
suite.

`owner-switch-native-off-k266` added the entry `native-off` to
`scripts/owner_switch.py` on 2026-11-08. The section
[The native settings inside finish](specs/owner-skill-delivery.md#the-native-settings-inside-finish)
owns its rules. The module docstring states the entry's behaviour and exit
status. The old prelude `native_settings` stays until
`owner-switch-contraction-k270` removes it.

### The entry

- The command form is
  `python3 -I owner_switch.py native-off [--home HOME] [--print-command]`.
  The entry reads no plan and needs no switch backup.
- `--print-command` prints one JSON object and writes nothing. The object
  holds the exact argv and command, the interpreter, the program, the
  canonical HOME and the settings path. The prepared recovery instructions
  take the command from the prepared program copy.
- Exit 0 means that the off value is in place. Exit 3 is a stop with no write
  to the settings. Exit 4 is a failed write. After 3 or 4, the output says
  that native delivery is incomplete, and it forbids a new default launch.
  It also names the rerun command.
- A repeated member name, a non-finite number, a value that is not an
  object, a link or a file of another owner is a stop. The accepted 2.0 build
  also refuses a repeated member name as `settings_invalid`.
- A Ravel process is a process whose executable path (`proc_pidpath`) or
  argv[0] has the basename `ravel`. Only the program and its exact ancestors
  are exempt.

### Controls

The run `task owner:switch:rehearse` with `SKIP_SETTINGS_WRITES=1` passed
all 249 controls on Python 3.15.1 and macOS 27.0.1. It took 11 minutes 48
seconds. The record is partial, because the rerun of `settings-writes.sh`
did not run. This leaf did not change the hooks installation that the rerun
measures. These subjects had the same SHA-256 digests before and after the
run:

| Subject | SHA-256 |
| --- | --- |
| `scripts/owner_switch.py` | `7f649053badc5ed1860643a014aecd0d1437ee0fa531a3d946dbc953f9d2a2c4` |
| `scripts/owner_switch_rehearsal.py` | `86ed44cac4adcfdf1268670a18d8655e33fe9b843faf405e63bf72863a82fb86` |
| `scripts/harness-trials/builds.json` | `29569fef4fc8e64b1ecf8b8f74d1eb85e10c8bfe0c136a4560763f69c3b5cfdf` |
| `target/ravel-archives-2.0/ravel-v1.1.0-aarch64-apple-darwin.tar.xz` | `289b4ac57ffd148255960507f02809034e8fb35fe97132813230116add804921` |

The 31 native-off controls had these results:

| Controls | Result |
| --- | --- |
| `native-off-absent-file` | A new file held only the off value, with mode 0o644. No switch backup existed. |
| `native-off-absent-key` | The member `policy`, mode 0o600, the ACL and `user.fault` stayed. The off value came after `policy`. |
| `native-off-existing-off` | The layout and the inode stayed unchanged. |
| Seven `native-off-refuses-*` controls | Malformed text, a repeated member, `NaN`, an array, `skills` null, another `skills` value and a link each stopped with exit 3. The layout stayed unchanged. |
| `native-off-other-ravel` | A fixture `ravel` process stopped the entry with no write. After the fixture ended, the same command wrote the off value. |
| `native-off-ravel-ancestor` | A fixture `ravel` ancestor was exempt. A helper process with `ravel` in its arguments was not a Ravel process. |
| `native-off-after-switch-stop` | An unrehearsed switch stopped before a backup and left the key absent. `native-off` then wrote the off value. |
| `native-off-print-command` | The printed argv named the program and the HOME, and running that argv wrote the off value. |
| `native-off-create-fails` | A read-only folder made the creation of the temporary file fail with exit 4. The live file stayed unchanged. The rerun completed. |
| `native-off-replace-fails` | A read-only folder after the flush made the rename fail with exit 4. The live file stayed unchanged, and the temporary file remained. The rerun removed it and completed. |
| Twelve crash controls | A crash at each of `begin`, `read`, `temp-partial`, `temp-synced`, `replaced` and `published` of the absent file and the absent key. Up to `temp-synced`, the live file was the original. After the rename, it was the complete new file. Each rerun completed and left no temporary file. |
| `native-off-reader` | A thread read the file during 20 publications: 421162 original and 142000 complete readings, with no partial or absent reading. |
| `native-off-persistence` | See the next section. |
| `native-off-accepted-build` | Build `3a9a9129573e…` resolved the fixture source while the key was absent. After `native-off`, the kept members `timeoutMs` and `configEnv` and a new file each resolved as `disabled`. |

### Persistence on the HOME volume

The control ran in a case HOME on `/dev/disk3s1`, mounted at
`/System/Volumes/Data` as APFS. The account's HOME is on the same device.
`F_FULLFSYNC` succeeded on the temporary file and on the folder after the
rename, with no fallback to `fsync`. A new process read the published bytes.
The command took 0.37 s. That time includes the load of the interpreter and
the process listing.

`F_FULLFSYNC` asks the drive to flush its cache, which `fsync` on macOS does
not do. No control cut power or ended the operating system. The crash controls
end only the process, so they show the order of the file operations, not the
durability after a power loss.

### Mutation controls

Each deliberate mutation of a scratch copy of the program made its control
fail:

| Mutation | Control that failed, and why |
| --- | --- |
| Write the live file in place, in two parts | `native-off-absent-key-crash-temp-partial` found a partial live file. `native-off-reader` stopped on the program's change check. |
| Replace the rename by a copy onto the live file | `native-off-reader` saw empty readings in each of three runs. |
| Skip the check for another Ravel process | `native-off-other-ravel` wrote the file. |
| Recognise `ravel` as a substring of the arguments | `native-off-ravel-ancestor` stopped on the helper. |
| Skip the copy of the metadata | `native-off-absent-key` failed on the ACL and the extended attributes. |
| Use `fsync` in place of `F_FULLFSYNC` | `native-off-persistence` did not find the method. |
| Skip the publication | `native-off-accepted-build` resolved the fixture source, not `disabled`. |
| Accept a repeated member name | `native-off-refuses-repeated-member` wrote the file. |

The first mutation stopped the whole rehearsal with an uncaught parse error.
The controls now read the file defensively, and the rerun reported both
failures. The mutations ran against the rehearsal before its final docstring
edit. The control logic did not change after that.

### Limits

- The machine held an orphaned Ravel 1.1.0 process with parent 1 during this
  session. It ran `start-session --kind=release-notes` from a temporary
  folder. A manual unscoped run on a scratch HOME recognised that process and
  stopped. It exempted this session's own Ravel ancestor. The owner must end
  such a process before finish runs `native-off`.
- A manual trial gave the live file an ACL entry `everyone deny delete`. The
  rename then failed with exit 4, and the live file stayed unchanged. The
  copied ACL also kept the temporary file from removal. The owner must remove
  such a file by hand.
- The owner's real `~/.config/ravel/settings.json` was absent. No control or
  trial wrote the real HOME.
- The recogniser covers only Ravel. `owner-switch-process-inspection-k267`
  owns the narrow recogniser of the four tools.

## Owner switch process inspection

Scope: the suite counts in this section include controls of the old `run`,
which owner-switch-contraction-k270 removed with that entry. The
observations stay as history. [Owner switch
contraction](#owner-switch-contraction) holds the result of the current
suite.

`owner-switch-process-inspection-k267` replaced the switch program's process
and job patterns with the narrow recogniser on 2026-11-08. The section
[The narrow process inspection](specs/owner-skill-delivery.md#the-narrow-process-inspection)
owns the contract. The module docstring of `scripts/owner_switch.py` states
the recognition rule, the admission rule and the entry `processes`.

### What changed

- The function `inspect_processes` recognises Claude Code, Codex, Grove and
  Ravel. It reads each executable with `proc_pidpath`, each argument vector
  with `sysctl` `KERN_PROCARGS2` and each creation time with
  `PROC_PIDTBSDINFO`. It never reads the joined `ps` text, so an argument
  with a space stays one argument.
- The gate of each switch step, the hooks installation and the inverse call
  that function. `native-off` calls it too, and it still refuses only another
  Ravel process.
- The program admits only itself and its exact ancestors. A recognised
  process is also admitted when its PID, creation time and canonical
  executable equal a probe-owned record. owner-switch-probes-k257 supplies
  those records and the inspection after probe cleanup.
- The entry `processes [--home HOME] [--owned FILE] [--json]` runs the
  inspection and changes nothing. The owner can run it during the checklist.
- The plan format has no `session`, `watch` or `jobs`. The program refuses a
  plan that holds one of them. The draft generator writes none, and
  `task owner:switch-plan:draft` regenerated the draft.
- The functions `list_processes`, `descendants`, `matches` and the
  pattern-based `process_gate` are gone. So is the job listing through
  `launchctl list` and `crontab -l`. The fixture plan's patterns, the fixture
  `jobs` command and the pattern controls are gone too.

### Measured launch forms

These forms come from the real process table on 2026-11-08, read only.

| Tool | Executable and arguments |
| --- | --- |
| Claude Code | `~/.local/share/claude/versions/2.1.319` with argv[0] `claude`. `~/.local/bin/claude` links to it. The same binary also runs with argv[0] `ugrep` for Claude Code's search. |
| Codex | `/opt/homebrew/Caskroom/codex/0.167.0/bin/codex`, linked from `/opt/homebrew/bin/codex`. Daemons run `.../app-server-daemon/releases/<version>/bin/codex` with `app-server daemon pid-update-loop`, and a child with `app-server --listen unix:// ... --managed-daemon`. |
| Grove | `/opt/homebrew/Cellar/grove/22.8.0/bin/grove` and `grove-llm`. Debug builds run `target/debug/grove`. |
| Ravel | `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, and a 1.1.0 build in a temporary folder. |
| Node servers | `node` from `/opt/homebrew/Cellar/node/26.13.0_2/bin/node`. Playwright rewrites its title to `npm exec @playwright/mcp@latest`, so argv[0] holds spaces and no path. |

17 of the owner's 878 processes had an unreadable `proc_pidpath`. Each was a
zombie or ran a binary that an upgrade deleted: old Python 3.15.0, watchman,
uv, mdbook and two Codex daemons. `KERN_PROCARGS2` still gave each one's
launch path.

The launch path recognises a deleted tool binary, so the two Codex daemons
stay recognised as Codex. Each other process with an unreadable executable
needs review, as the contract states. On 2026-11-08 that rule sent 11
processes to review. `PROC_PIDTBSDINFO` failed for each of the 299 processes
of other accounts. `proc_pidpath` failed for one of them.

### In-session review

One fresh context read the inspection code, the recognition table and the
contract, without this leaf's conclusions. Its findings had these
dispositions:

| Finding | Disposition |
| --- | --- |
| Codex's helpers `codex-exec-server` and `codex-code-mode-host` escaped | Fixed. A basename with the prefix `codex-` is Codex. |
| An unreadable executable whose launch path names no tool returned nothing | Fixed. It needs review, as the contract states. |
| A prerelease version in `.../claude/versions/` escaped | Fixed. Each executable in that folder is Claude Code. |
| A Node title that ends in a path was read as a program name | Fixed. A title with spaces gives its first word. |
| A relative entry script resolved against the inspector's folder | Fixed. It resolves against the process's own folder. |
| `bun`, `--env-file-if-exists` and shell or Python option values | Fixed. |
| An outer harness above the driver chain was admitted | Fixed. Only the chain up to the first harness, and the drivers above it, is admitted. |
| A probe-owned record lacked the configuration HOME | Fixed. The record names HOME, and admission compares it with the process's environment. |
| A process that exits during the inspection gave a false review. A failed `ps` raised an unexplained error. | Fixed. |
| The operator's own short-lived children count as other processes | Trade-off. The contract grants no descendant exemption. The operator waits on the program alone, so a refusal on a transient search child is retried. |
| An empty argv[0] shifts the vector | Trade-off. The kernel pads the launch path with NUL bytes, so an empty argv[0] cannot be told from the padding. |
| Processes of other accounts are recognised by executable only | Trade-off. Their arguments, folder and creation time need privilege. |
| PID reuse of an ancestor inside one snapshot | Noise. An ancestor of the running program cannot end and be reused during its own inspection. |
| The scope guard compared path spellings | Fixed. It compares canonical paths. |

The review also saw eight reparented Codex daemons from earlier probe trials
in scratch folders and the real `~/.codex` daemon. owner-switch-probes-k257
owns probe service cleanup, and the owner's checklist owns the real daemon.

### Controls

The run `task owner:switch:rehearse` with `SKIP_SETTINGS_WRITES=1` ran 265
controls on Python 3.15.1 and macOS 27.0.1 in 12 minutes 51 seconds. 264
passed. `owner-inverse-complete` failed, because it still expected the old
gate message. The control then expected the new message, and its rerun
passed.

The record is partial, because the rerun of `settings-writes.sh` did
not run. This leaf did not change the hooks installation that the rerun
measures. These subjects had the same SHA-256 digests before and after the
full run:

| Subject | SHA-256 |
| --- | --- |
| `scripts/owner_switch.py` | `ef92cf7434d02d2f1cd95dd75ab37c210049946c03eefc1ebdd473e69c25b51e` |
| `scripts/owner_switch_rehearsal.py` | `e23a0f622eeabe78fefc7acfe2b4d2ac2d1753942febc806d8bf378a2c280647` |
| `scripts/harness-trials/builds.json` | `29569fef4fc8e64b1ecf8b8f74d1eb85e10c8bfe0c136a4560763f69c3b5cfdf` |
| `target/ravel-archives-2.0/ravel-v1.1.0-aarch64-apple-darwin.tar.xz` | `289b4ac57ffd148255960507f02809034e8fb35fe97132813230116add804921` |

The rerun of `owner-inverse-complete` used the rehearsal with the digest
`c28e5835014ded420c49eccb97ec109ab14c12ea9a2cf7536a32af7af6617388`.

The process controls had these results:

| Controls | Result |
| --- | --- |
| `recognition-forms` | 42 measured and constructed forms gave the expected tool, review or nothing through the program's `recognise()`. |
| `processes-native` | A versioned Claude Code, Codex, Grove, `grove-llm` and Ravel were each recognised. |
| `processes-node` | Two package entry scripts, an npm bin link and a Node title `claude` were recognised. |
| `processes-codex-daemon` | A reparented `app-server daemon pid-update-loop` and a managed app-server refused. |
| `processes-unrelated` | Tool names in other arguments, a Node server, `caffeinate` and a Python script left the inspection clear. |
| `processes-review` | A title `claude` on another executable, a wrapper script `codex` and a Node script `grove.js` needed review. |
| `processes-deleted-executable` and `processes-deleted-other` | A deleted `codex` was recognised by its launch path. A deleted executable of another name needed review. |
| `processes-relative-script` | `node cli.js` in the Claude Code package folder was Claude Code. |
| `processes-ancestors` and `processes-outer-harness` | A Ravel driver and a Claude Code harness ancestor were admitted. An outer Codex above them refused. |
| `processes-descendant-harness` | A Codex child of the harness refused. A `caffeinate` child needed no exemption. |
| `processes-probe-owned` | Only the exact PID, creation time, executable and HOME admitted a probe-owned Codex. A changed creation time, executable or HOME, and a record without HOME, refused. |
| `tool-before-run`, `tool-later`, `descendant-harness-before-run`, `descendant-harness-later`, `native-tool-later` and `hooks-tool-later` | A tool process before step 1, at step 9, in the session or at each prelude stopped the switch with no write. |
| `background-child-allowed`, `caffeinate-child-allowed`, `mcp-server-allowed` and `finish-ancestors-allowed` | The switch completed. |
| `mcp-server-tool-child` | A Codex child of an MCP server of the session stopped step 1. |
| `plan-process-patterns` | A plan with `session`, `watch` or `jobs` stopped before any step. |
| `native-off-other-tool` and `native-off-ravel-title` | Codex and Claude Code processes did not stop `native-off`. A process titled `ravel` without a Ravel executable did. |

Each process control also checked that its fixture processes still ran, so
the program signalled none of them.

### The draft check

Scope: this section describes the switch program before
owner-switch-contraction-k270. That leaf removed the old `run`, `inverse`,
`merge`, `pending`, `state` and `purge-check` entries, with the exchange,
held links, kept copies, cell inverses, rehearsal admission and the plan's
parts. The observations stay as history. They are not evidence for the
current program, which [Owner switch contraction](#owner-switch-contraction)
describes.

`task owner:switch-plan:draft` regenerated the draft from the real HOME. The
draft lost `session`, `watch` and `jobs`. It also took the root manifests of
`.claude/skills` and `.codex/skills` that owner-inventory-exceptions-k264
added to the generator, with the identities unchanged. The part table did
not change.

`task owner:switch-plan:check` passed: the dry run found the native state,
eight controls passed, and the draft held no process pattern.
[`switch-plan-check.json`](owner-cutover/switch-plan-check.json),
[`switch-plan-processes.txt`](owner-cutover/switch-plan-processes.txt) and
`switch-plan-dry-run.txt` now hold
this check's results. The listing names the installed path and canonical
path of each tool executable. It then names the admitted ancestors, the
other tool processes and the processes that need review.

| Subject | SHA-256 |
| --- | --- |
| `docs/owner-cutover/switch-plan-draft.json` | `2b8d3d67d0e51c75d35d3cff42a3c340398fc6d3734582b56aff3403b1ad50c1` |
| `docs/evaluations/owner-skill-delivery/switch_plan_draft.py` | `b1460297f04cf20b97706c9a891a4e5236aaea80073c75ecf9e249c127bcf6f7` |

### Mutation controls

Each deliberate mutation of a scratch copy of the program made its control
fail:

| Mutation | Control that failed, and why |
| --- | --- |
| Admit each descendant of an admitted ancestor | `processes-descendant-harness` exited 0 beside a descendant Codex. |
| Recognise a tool name anywhere in the arguments | `processes-unrelated` refused the helper whose arguments name the tools. |
| Drop Claude Code's versioned path | `processes-native` reported the versioned fixture as a review, not as Claude Code. |
| Ignore the creation time of a probe-owned record | `processes-probe-owned` admitted the record with a changed creation time. |
| Drop the review of a tool name without a tool executable | `processes-review` exited 0. |
| Admit every ancestor | `processes-outer-harness` admitted the outer Codex. |
| Resolve a relative script against the inspector's folder | `processes-relative-script` exited 0. |
| Drop the review of an unreadable executable | `processes-deleted-other` exited 0. |
| Make the gate ignore the report | `tool-before-run` completed the switch beside a Codex daemon. |

### Checklist duties that no fixture establishes

The fixtures prove recognition and admission on the shared process table.
They cannot establish these duties of
[the owner's checklist](specs/owner-skill-delivery.md#the-owners-checklist):

- Item 1: the end of Pi sessions and of the desktop applications Claude
  and ChatGPT with their hosted tools. It also covers the delegates inside
  the operator's own harness process. The recogniser does not name Pi or the
  applications, and an in-process delegate has no process of its own.
- Item 2: the pause of `mise dot watch`, herdr, Orca, codebase-memory
  installers and each other non-harness writer. The inspection recognises
  none of them.
- Item 3: relaunch prevention by login items, launch agents and schedules.
  The inspection runs at its points only. A job that launches a tool
  between two points is seen at the next point at the earliest. A job that
  writes without a tool process is never seen.
- Item 4: the operator's own settings actions, MCP operations and
  in-process delegates. The operator's harness is an admitted ancestor, so
  the inspection cannot see what it does.
- Items 5 to 7: the marketplace freeze and the held Claude Code version.
  They also cover the review of the plan and staging evidence, and the
  pause of the manual's synchronisation. These are not process states.
- A tool binary copied or hard-linked under another name, with another
  title, is not recognised. A symbolic link resolves to the binary, so it is
  recognised. A process of another account is recognised only by
  its executable.

### Limits

- The real inspection on 2026-11-08 would refuse the switch. It found 17
  other tool processes. They were the real Codex daemon and its app-server
  and 11 Codex processes of earlier trials in scratch folders. The others
  were an orphaned Ravel 1.1.0 session, two stale Grove debug builds and an
  orphaned Claude Code `ugrep`. Eleven processes with deleted binaries need review. The owner
  ends each of them before the switch.
- No control ran a real Claude Code, Codex, Grove or Ravel process. The
  fixtures are clones of the interpreter under the tools' names and paths.
  The real process table supplied the measured forms.
- The inspection after probe cleanup, and the ownership records of real
  probes, belong to owner-switch-probes-k257.

## Owner switch serial restore

Scope: the suite counts in this section include controls of the old `run`,
which owner-switch-contraction-k270 removed with that entry. The
observations stay as history. [Owner switch
contraction](#owner-switch-contraction) holds the result of the current
suite.

`owner-switch-serial-restore-k272` added the serial switch to
`scripts/owner_switch.py` on 2026-11-08. The entries are `attempt`,
`switch`, `restore` and `restore-identity`. The sections
[The switch inside finish](specs/owner-skill-delivery.md#the-switch-inside-finish),
[Resuming the switch](specs/owner-skill-delivery.md#resuming-the-switch) and
[Rollback](specs/owner-skill-delivery.md#rollback) own the rules. The module
docstring, section "The serial switch", states each step, exit status,
hook point and plan field. The old run, inverse, merge, pending and state
entries stay until `owner-switch-contraction-k270` removes them.

### The entries

- `attempt` writes the attempt note `<state>/attempts/<id>.json` with mode
  0444. The note names HOME, the plan, the program and the command inputs.
  It also names the observed tool versions, the backup path and the
  literal switch, restore and native-off commands. It records no progress.
- `switch` refuses with exit 3 before any switch-set write. A note that does
  not fit refuses, and so does a changed command input or tool version. An
  unclear process inspection, a changed planned original or a changed
  inventory refuses too. So do a missing off value, a baseline anomaly and
  a failed gate of steps 4 to 7.
- The backup copies each planned file and entry with `fcopyfile` metadata
  and records each absence. The switch verifies the originals against the
  copies before one publication rename. The backup holds no program.
- After the publication, a caught failure restores and exits with 5. A
  failed restore or a remaining probe-owned service exits with 4, and the
  backup stays.
- `restore` verifies the backup against its manifest, the attempt note and
  HOME. With a valid receipt, a destination that differs from its
  post-switch identity refuses before the first write. An eligible later
  restore first writes a recovery note and backup.

The restore identity leaves out `com.apple.provenance`. A measurement on
2026-11-08 showed that macOS gives that attribute to each new file of the
process. The same measurement showed that `fcopyfile` copies the ACL, the
extended attributes and the mode of a folder.

### Controls

The run `task owner:switch:rehearse` with `SKIP_SETTINGS_WRITES=1` passed
all 288 controls on Python 3.15.1 and macOS 27.0.1. It took 15 minutes 16 seconds. The
record is partial, because the rerun of `settings-writes.sh` did not run.
The serial switch runs no held-link command that the rerun measures. These
subjects had the same SHA-256 digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `scripts/owner_switch.py` | `6508af236b18f22a4cbc62e75c2e0ac63bef0c93117936fc027df22d918ed22f` |
| `scripts/owner_switch_rehearsal.py` | `f3cdbda6e5bf05928b338a7d49f772271e6316bb5b3733404759cfdd66df34da` |
| `scripts/harness-trials/builds.json` | `29569fef4fc8e64b1ecf8b8f74d1eb85e10c8bfe0c136a4560763f69c3b5cfdf` |

The 23 serial controls had these results:

| Controls | Result |
| --- | --- |
| `serial-switch-and-later-restore` | The switch ran `marketplace add` and `install` once each and ended switched. The Ravel settings kept mode 0o640 and `user.owner`. The backup recorded the absent `known_marketplaces.json` and held no program. The receipt kept six restore identities apart from five delivery assertions. The note's literal restore command wrote a recovery backup and returned the layout. The receipt became invalid, and the manual received a correction and a native return. A repeated restore wrote nothing. |
| `serial-backup-stopped` | A failure before the publication and a crash during the copy each left the layout unchanged and published no backup. The same note then completed the switch. |
| `serial-attempt-mismatch` | After a crash, restore refused another attempt's note and another HOME, and a fresh attempt refused the changed originals. None of them wrote. The right note then restored the layout. |
| `serial-later-drift` | After the switch, a changed `model` key refused the later restore before any write. The refusal named `settings.json`, and the receipt stayed valid. An appended manual note did not count as drift. |
| `serial-preflight-refusals` | A changed command input, an extra native skill and an absent off value each refused with exit 3 and published no backup. |
| Eleven `serial-fail-*` controls | A failed probe and a caught failure at ten named points each exited 5, with the native layout restored and no valid receipt. The points ran from `backup:published` to `switch:final`. After `switch:final`, the manual received a correction. A repeated switch with the same note refused. |
| Seven `serial-crash-*` controls | A crash at seven named points ended the program. The note's own switch then refused, and its literal restore command returned the layout. After `receipt:written` and `switch:final`, restore ran as a later restore. |

### Mutation controls

Each deliberate mutation of a scratch copy of the program made its control
fail, except the two that the table names:

| Mutation | Control that failed, and why |
| --- | --- |
| Skip the copy of a file's metadata | `serial-switch-and-later-restore` refused with exit 3, because the backup check found the copies different. |
| Skip the drift comparison of a later restore | `serial-later-drift` restored over the changed `model` key. |
| Skip the comparison of the backup with its note and HOME | `serial-attempt-mismatch` restored with another attempt's note. |
| Skip the restore after a caught failure | `serial-fail-probes` and `serial-fail-switch:final` left the switched layout. |
| Skip the removal of an original absence | `serial-fail-hooks:after:marketplace-add` exited 4 with `known_marketplaces.json` left. |
| Skip the refusal of an existing backup in the note check | The first run survived, because the changed originals refused the repeated switch later. The controls now require the note's refusal, and `serial-crash-probes` and `serial-fail-probes` then failed. |
| Skip the comparison of originals with their copies before publication | It survived. No control made a copy differ. `serial-backup-incomplete` now does, as [Owner switch restore controls](#owner-switch-restore-controls) records. |

The other mutations ran before that change of the refusal text. The change
only narrows what two controls accept.

### Limits

- The controls run the stub `claude`, `brew`, `mise` and `ravel`. No
  control ran a real harness or wrote the real HOME.
- The probe-owned service seam only verifies that no recorded service
  still runs. `owner-switch-probes-k257` adds the measured shutdown route.
- [Owner switch restore controls](#owner-switch-restore-controls)
  records the remaining cases of "Test seams".
- A first full run gave a false failure count, because the program changed
  while the run read it. That run measured nothing. A later run failed one
  control, because its pattern refused the space in that case's folder
  name. The run above began after the correction.

## Owner switch restore controls

Scope: the suite counts in this section include controls of the old `run`,
which owner-switch-contraction-k270 removed with that entry. The
observations stay as history. [Owner switch
contraction](#owner-switch-contraction) holds the result of the current
suite.

`owner-switch-restore-controls-k273` added 19 serial controls to
`scripts/owner_switch_rehearsal.py` on 2026-11-08. They cover the cases of
[Test seams](specs/owner-skill-delivery.md#test-seams) that the
[serial restore](#owner-switch-serial-restore) left open. Four of those
cases found program defects, which this leaf corrected first.

### Program changes

- An unreadable planned original ended `switch` with a traceback and
  exit 1. A failed content or metadata read now refuses with exit 3. The
  identity function raises the program's stop, so no write happens.
- `restore` trusted the manifest's items. A changed manifest whose copies
  matched it passed, and an item path outside the plan raised a
  `KeyError`. Each item must now name the plan's write by path, type and
  `data/<n>`. A switch backup's items must also have the plan's original
  identities. The backup's plan digest and kind must equal the note's. The
  chain from the immutable note to each copy now holds no link that a
  changed manifest alone can satisfy.
- A restore that ended with exit 4 for an unwritable manual wrote its
  restored record first. A repeated restore then reported that nothing
  remained and lost the pending manual entry. A repeated restore now
  appends each missing entry, found by its marker, and writes no restore
  destination.
- The restored record now names the admitted ancestors as the operator.
  `restore --guidance` records each guidance file by digest, and a file
  that changed or became unreadable by the end leaves recovery incomplete.

### Controls

The run `task owner:switch:rehearse` with `SKIP_SETTINGS_WRITES=1` passed
all 307 controls on Python 3.15.1 and macOS 27.0.1. It took 17 minutes 45
seconds. The 288 earlier controls are among them. The record is partial,
because the rerun of `settings-writes.sh` did not run. These subjects had
the same SHA-256 digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `scripts/owner_switch.py` | `df596f601efcb889d0e12cacb07c36911f55b2b1a90bcc78b5ea600481ceacd0` |
| `scripts/owner_switch_rehearsal.py` | `593dd9be01108f7f970794d2a3a3b71ac69fd3609298e1b55d0ef8cd2ec85022` |
| `scripts/harness-trials/builds.json` | `29569fef4fc8e64b1ecf8b8f74d1eb85e10c8bfe0c136a4560763f69c3b5cfdf` |

The 19 new controls had these results:

| Control | Result |
| --- | --- |
| `serial-backup-metadata` | ACLs and `user.owner` on an entry folder, its file, the Ravel settings and `installed_plugins.json` reached the backup. The ordinary write kept them on the Ravel settings. The backup kept the link as a link. After a crash and the literal restore, each path had its original restore identity and its ACL and attribute. |
| `serial-backup-incomplete` | An unreadable file and an unreadable folder during the copy, a changed copy and a changed original before publication each refused with exit 3. So did an unreadable original before the backup. None published a backup or changed the layout, and the same note then completed the switch. |
| `serial-backup-changed` | After a crash, each changed backup refused restore with exit 3 and no write. The changes were a copy, a copy with a matching manifest, and that change with the kind "recovery". They were also an item outside the plan, a copy name outside the backup, and a changed plan with its digest in the manifest. The unchanged backup then restored. |
| `serial-restore-interrupted` | A crash during the Ravel write left a temporary file. Restore crashed at `restore:.claude/settings.json` and again at `restore:native-probes`, and the note's switch refused after each. The same command then restored the native layout and removed the temporary file. |
| `serial-restore-off-absent-file` | A backup without the Ravel settings, made by the program's `create_backup`, restored the native layout. The Ravel settings then held only the off value, and nothing else differed. |
| `serial-restore-off-absent-key` | The same with settings that lacked `skills` and held `policy`. Restore kept `policy` and added only the off value. |
| `serial-restore-io-failure` | A read-only `.claude/skills` made restore exit 4 and name `.claude/skills/grove` with its error. The backup stayed, and the program wrote no restored record or native return. After the owner corrected the permission, the same command restored. |
| `serial-later-restore-interrupted` | A crash during a later restore left its recovery note and backup. The note's own command then refused, because the layout differed from the receipt. The printed retry command with `--recovery` restored, invalidated the receipt and appended a correction and a native return. |
| `serial-owner-after-finish` | A switch under a Ravel driver and Claude Code crashed. A remaining Claude Code process refused the owner's restore with no write. Guidance that changed during restore left recovery incomplete with exit 4. The owner's own Claude Code session then restored. The record named that session without Ravel, and the guidance by path. |
| `serial-hooks-command-fails` | A failed `claude plugin install` exited 5 with the native layout and its originals restored. |
| `serial-hooks-unexpected-change` | A `model` key that appeared after `claude plugin install` exited 5 with "changed other keys". |
| `serial-hooks-acl-appears` | An ACL that appeared before `claude plugin marketplace add` stopped before that command ran, and restore removed it. |
| `serial-hooks-attribute-appears` | An attribute that appeared before `claude plugin install` stopped before that command ran, and restore removed it. |
| `serial-manual-unwritable` | A read-only manual after the receipt stopped the success observation. The switch restored the configuration and exited 4 with the entry pending. After the owner corrected the permission, a repeated restore appended the native return once. |
| `serial-manual-correction-pending` | After the success observation, a read-only manual and a failed final comparison left the correction pending with exit 4. A repeated restore appended the correction and the native return once. |
| `serial-probe-owned-live` | A probe that left its service running made the switch exit 4 with no restore and no receipt. After the owner ended the service, the literal restore restored. |
| `serial-probe-changes-setting` | A capability setting that changed during the probes failed the final comparison. The switch exited 5, wrote no receipt and restored. |
| `serial-probe-adds-entry` | A native entry added during the probes failed the final comparison. Restore kept that entry, because it lies outside the write set. |
| `serial-probe-changes-registration` | A registration removed during the probes failed the final comparison, and restore returned it. |

Each caught-failure control now also compares every planned original by
its restore identity. That identity covers mode, ACL, attributes and link
text, which the layout comparison of content and mode does not.

### Mutation controls

A scratch copy of the program received one mutation at a time, and each
mutant ran only the control named below. The mutants changed the program
with the digest above. Each of the 24 mutants made its control fail. Two failed by the refusal text alone, which the table says.

| Mutation | Control that failed, and why |
| --- | --- |
| Let a read error escape the identity function | `serial-backup-incomplete` met another refusal text. |
| Skip the item identity check | `serial-backup-changed` restored a changed copy with a matching manifest. |
| Skip the kind check | `serial-backup-changed` restored the same change under the kind "recovery". |
| Skip the plan digest check against the note | `serial-backup-changed` restored with a changed plan. |
| Skip the copy-name check | `serial-backup-changed` met another refusal text, because the copy check still refused. |
| Append nothing in a repeated restore | `serial-manual-unwritable` and `serial-manual-correction-pending` lost their pending entries. |
| Skip the guidance check | `serial-owner-after-finish` restored with changed guidance. |
| Record no operator | `serial-owner-after-finish` found an empty operator. |
| Copy no folder metadata | `serial-backup-metadata` refused, because the backup check found the folder copy different. |
| Skip the comparison before publication | `serial-backup-incomplete` published a changed copy. |
| Skip the metadata refusal before a command | `serial-hooks-acl-appears` and `serial-hooks-attribute-appears` switched. |
| Skip the settings reread after a command | `serial-hooks-unexpected-change` switched. |
| Ignore a failed command | `serial-hooks-command-fails` met another failure text, because the later hooks state still failed. |
| Ignore a remaining probe-owned service | `serial-probe-owned-live` switched. |
| Ignore assertions, inventory or readers in the final comparison | Each of the three `serial-probe-*` change controls switched. |
| Enforce no off value in restore | Both off-value controls exited 4. |
| Keep interrupted temporary files | `serial-restore-interrupted` found the temporary file. |
| Ignore a failed restore path | `serial-restore-io-failure` exited 0. |
| Accept no recovery identity in a retry | `serial-later-restore-interrupted` refused its retry. |

### Limits

- No fixture fails an ACL or attribute read of an open descriptor. A
  folder with mode 000 stands for the failed metadata read and a file with
  mode 000 for the failed copy read. Both reach the same conversion.
- A recovery backup holds the layout that its later restore compared, so
  no earlier record anchors its manifest. A changed recovery manifest can
  only widen what its retry accepts.
- The owner's restore records the operator's ancestors and guidance. It
  does not record cache paths or a built-in skill listing, which the
  switch takes from its reader evidence.
- The settings-write measurement, the idle signed-in parent harness and
  the real probes remain acceptance work on real harnesses. No fixture
  establishes them.

## Owner switch status

Scope: the suite counts in this section include controls of the old `run`,
which owner-switch-contraction-k270 removed with that entry. The
observations stay as history. [Owner switch
contraction](#owner-switch-contraction) holds the result of the current
suite.

`owner-switch-status-k269` moved the switch area of the owner's `mac-check`
and the purge report to the serial switch on 2026-11-08. The specification's
section [Current health and restore eligibility](specs/owner-skill-delivery.md#current-health-and-restore-eligibility)
owns the rules. The old `state` and `purge-check` entries remain until
owner-switch-contraction-k270 removes them.

### The entries

- `status --plan PLAN [--json]` reads the serial records and the layout, and
  writes nothing. Delivery health is `native`, `switched`, `restore` or
  `review`. Restore eligibility is a separate report: `eligible`,
  `ineligible` with the differing restore destinations, `unavailable`,
  `required` with each restore command, `in-progress` or `none`. A restored
  attempt whose manual lacks its entries is a pending manual entry.
- Switched health needs a valid receipt of the prepared plan. The plan's
  assertions, `switch.inventory.after`, the receipt's reader evidence and
  the sources must also hold. The status resolves the current Ravel
  `skills` value in a disposable HOME, as gate 6 does, and compares each
  source's ID and `resourceDigest` with the receipt's.
- Native health needs the new plan field `switch.native_assertions` and
  `switch.inventory.before`. An assertion can give `values`, a list of
  accepted values.
- Step 8 of the switch runs `mac-check` before the receipt exists. The
  status looks for the switch of the governing attempt's note among its
  ancestor processes. When it finds that switch, it evaluates the attempt as
  switched from the plan and the backup's evidence. A crashed attempt is no
  ancestor and needs restore.
- The receipt's retained resources now hold an `identity`. A path's identity
  is its change identity without the top-level `.in_use` reader markers. A
  registration's identity is the digest of its JSON entry.
- `purge-report --plan PLAN` blocks with exit 3 unless the health is
  `switched` and each retained identity is unchanged. It lists each
  condition that only the owner establishes, and writes nothing.
- `mac-check` runs `status --json` and maps it to fixed messages. A switched
  layout with ineligible restore stays `OK` and names the differing files.
  A pending manual entry or an unavailable backup needs attention. Without a
  prepared plan, the row reports that the switch is not prepared and
  verifies no delivery state. Formula drift has no part in this row.

### Controls

`task owner:switch:rehearse` passed all 315 controls on Python 3.15.1 and
macOS 27.0.1. The controls took 19.1 minutes. The suite holds 17 new
controls, and the nine old `mac-check-*` controls are gone, because
`mac-check` no longer reads the old states. These subjects had the same
SHA-256 digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `scripts/owner_switch.py` | `ccfe957584387dcdec1e77062e62f47fadded8546d0fe7ffb4917af35c1a9ffa` |
| `scripts/owner_switch_rehearsal.py` | `6cfb4d3df04f54606fb09ad74ec6d3ca1b567ee7e2450e1fca3ae1932e9bcb27` |
| `~/.local/lib/machine-config/check.py` | `5c7a3bd43c08e43e02859c6c0c22edcb59e0bf0b38dcbe7b040784be247e9201` |
| `~/.local/bin/mac-check` | `b6738bfa2b04b21742a4693e9d4d5e86c9f69d56ccd8c73d36a484d6ade0a5e8` |
| `docs/evaluations/hooks-installation-writes/settings-writes.sh` | `b4cd60b9620518838bfdcc43eaead35879ea2304ad3ca1e1aba2a1900b87b9e3` |

In that run, the rerun of `settings-writes.sh` with Claude Code 2.1.319
failed its range. The interval of `marketplace add` was 38.2 to 39.2 ms,
and the range is 30 to 38 ms. The intervals of `install` and `disable` were
inside the range. Each command renamed one file onto the path. The same
rerun then ran alone on an otherwise idle machine and passed. That time,
`add` gave 37.8 to 38.8 ms.

The miss is therefore a margin of 1 ms under load, not a changed write
method. The record of the full run is not a passing rehearsal record. Only
the old path reads such a record, and the serial switch reads none.

Each status control reads the real `mac-check` with HOME set to its case:

| Control | Result |
| --- | --- |
| `status-not-prepared` | Without a plan, the row reported that the switch is not prepared and verifies no delivery state. A record folder without a plan, and a plan without the program, each needed attention. |
| `status-native` | The prepared plan reported verified native delivery. A removed `skills` key needed review. A plan whose native assertion gives both the off value and absence reported native delivery again. |
| `status-partial-native` | A removed native entry without an attempt needed review, and the status named the entry. |
| `status-switched-unrelated-edits` | After success, a model key, a hook and a manual note changed. Health stayed switched, restore became ineligible, and the row named only `.claude/settings.json`. A later restore then refused with no write. |
| `status-switched-setting-changed` | A superseded plugin enabled again needed review while the receipt stayed valid. |
| `status-switched-entry-added` | A new native skill needed review while the receipt stayed valid. |
| `status-switched-source-changed` | A one-byte change of a keg source needed review while the receipt stayed valid. |
| `status-switched-reader-orphaned` | An orphan marker in a recorded cache version needed review while the receipt stayed valid. |
| `status-stale-receipt` | A prepared plan other than the receipt's plan needed review as stale evidence. |
| `status-crash-before-receipt` | A crash at the probes left a complete switched layout. The row reported that the attempt needs its restore, with the note's command. After the literal restore, it reported native delivery. |
| `status-step-8-in-progress` | A plan whose `mac_check` is the real `mac-check --only "Owner skill switch"` switched. Step 8 read a verified switched layout of a running attempt, and afterwards the row reported eligible restore. |
| `status-pending-manual` | A read-only manual left the restore's entry pending. The row reported native delivery and the pending entry. After a repeated restore, it reported native delivery alone. |
| `status-later-restore-interrupted` | A crash during a later restore needed restore, and the status named the retry command with `--recovery`. After the retry, the row reported native delivery. |
| `purge-report-clean` | After success and a new reader marker, each retained identity held. The report listed the owner's conditions, without the condition that it reads itself. |
| `purge-report-cache-changed` | A changed file in a frozen cache version blocked purge while health stayed switched. |
| `purge-report-registration-changed` | A changed registration of the superseded plugin blocked purge while health stayed switched. |
| `purge-report-unverified` | Without a receipt, the report blocked purge. After success and a changed switched setting, it blocked purge for the unverified switch. |

The checker's tests passed 56 of 56. Ruff and strict pyright 1.1.414
reported nothing.

### Mutation controls

A scratch copy of the program or of the checker received one mutation at a
time, and each mutant ran only the control named below. The mutants changed
the files with the digests above. Each of the 17 mutants made its control
fail, each by the behaviour that the control guards.

| Mutation | Control that failed, and why |
| --- | --- |
| The checker reports native delivery without a plan | `status-not-prepared` read the old claim of native delivery. |
| Native health ignores the native assertions | `status-native` reported native delivery without `skills`. |
| Native health ignores the native inventory | `status-partial-native` reported native delivery without the Codex entry. |
| Switched health also needs whole-file identities | `status-switched-unrelated-edits` needed review after the unrelated edits. |
| Switched health ignores the assertions | `status-switched-setting-changed` reported a verified layout. |
| Switched health ignores the inventory | `status-switched-entry-added` reported a verified layout. |
| Switched health ignores the sources | `status-switched-source-changed` reported a verified layout. |
| Switched health ignores the reader evidence | `status-switched-reader-orphaned` reported a verified layout. |
| No stale-evidence check | `status-stale-receipt` reported a verified layout. |
| Each attempt counts as a running switch | `status-crash-before-receipt` reported a verified layout of a running attempt. |
| No attempt counts as a running switch | `status-step-8-in-progress` failed step 8 and exited 5 after restore. |
| No pending manual entry | `status-pending-manual` reported native delivery alone. |
| No recovery retry | `status-later-restore-interrupted` needed review, not restore. |
| Reader markers count in an identity | `purge-report-clean` blocked purge after the new marker. |
| A path's identity is its presence | `purge-report-cache-changed` allowed purge. |
| A registration's identity is its name | `purge-report-registration-changed` allowed purge. |
| The purge report ignores health | `purge-report-unverified` allowed purge after the changed setting. |

### Limits

- The sources check runs `$(brew --prefix)/bin/ravel` in a disposable HOME.
  The fixture's stub Ravel stands for the promoted Ravel, so no fixture
  establishes the real resolution's time or its cache writes.
- The fixture plan's retained "original driver's Ravel keg" is the case's
  own fixture Ravel. The purge report therefore reads no file outside the
  case.
- Step 8 needs exit 0 from the plan's `mac_check` command. The full
  `mac-check` exits 10 for CCC's manual review alone, so the real plan must
  name `mac-check --only "Owner skill switch"`. owner-switch-plan-k254 holds
  that note.
- The real `mac-check` reports that the switch is not prepared, because the
  real HOME has no prepared plan.

## Owner switch contraction

`owner-switch-contraction-k270` removed the old switch machinery from
`scripts/owner_switch.py` on 2026-11-08, and it shrank the plan format to
the simplified design. The sections
[The switch inside finish](specs/owner-skill-delivery.md#the-switch-inside-finish),
[The rehearsed plan](specs/owner-skill-delivery.md#the-rehearsed-plan) and
[Rollback](specs/owner-skill-delivery.md#rollback) own the rules. The module
docstring states each entry and the plan format.

### The real HOME before the change

`~/.local/state/owner-switch` did not exist, so the real HOME held no
prepared plan, no prepared program and no backup. No backup of an earlier
format therefore needs a reader, and the program reads only the new format.
The real `mac-check --only "Owner skill switch"` reported that the switch is
not prepared, before and after the change.

### The program

The program now holds these entries: `report`, `native-off`, `attempt`,
`switch`, `restore`, `status`, `purge-report`, `processes`, `gate`,
`identity`, `digest` and `restore-identity`. The removed code covers the old
`run`, `run --dry-run`, `inverse`, `merge`, `pending`, `state` and
`purge-check` entries. It also covers the exchange, kept and flagged copies,
held links, the cell inverse and pending-activation records. The removal
also took run logs, partial continuations, program copies in backups,
run-state detection and rehearsal admission. The program shrank from 5,333 lines to 3,764. The fixture suite shrank from
4,584 lines to 3,280.

`report` is the read-only plan report. It lists each write with its planned
original and its current restore identity, and each value that the switch
sets with its current value. It also compares both inventories, evaluates
both lists of assertions, compares each command input with its digest and
lists each command. It runs no command and writes nothing.

### The plan format

The schema is now `owner-switch-plan/2`, and its fields are top-level. The
`switch` section of the serial restore, `parts` and `outside` are gone.

Each item of `writes` states its action. The switch deactivates each entry, and a
file can hold `set`, the values that the switch writes. No item holds an earlier
value or an entry identity. `inventory` replaces `outside`, with the parts
included before the switch. The loader refuses each field outside the
schema. A plan therefore cannot hold a process, MCP or job pattern, a
rehearsal token or a recovery field.

### The regenerated draft

`task owner:switch-plan:draft` wrote the draft from the real HOME, reading
only. The draft runs no tool. It reads Claude Code's held version and the
original driver's Ravel keg from the canonical paths of the installed
executables.

The draft holds 154 entry writes and 4 file writes, 8 inventory
items and 4 command inputs. A prepared input that does not exist yet carries
the digest of the repository file that preparation copies. The probe
program has no source yet, so the inputs do not name it until
owner-switch-probes-k257 supplies it. The draft's `mac_check` names
`mac-check --only "Owner skill switch"`, and its native Ravel assertion
accepts the off value or an absent key.

`task owner:switch-plan:check` passed. In the disposable HOME, the report
listed each write at its original, the inventory before matched and the
native assertions held. Ten controls passed:

| Control | Result |
| --- | --- |
| `no-mac-check`, `absolute-write-path`, `gate-4-missing` | The loader refused the plan with exit 3. |
| `process-patterns`, `parts` | The loader refused a field outside the schema. |
| `earlier-value` | The loader refused a `set` item with an earlier value. |
| `changed-entry` | The report showed the changed Codex entry as "differs". |
| `new-inventory-child`, `new-sibling-of-system` | The report showed the inventory difference. |
| `inside-platform-subtrees` | Changes inside `synced` and `.system` left the inventory matching. |

| File | SHA-256 |
| --- | --- |
| [`switch-plan-draft.json`](owner-cutover/switch-plan-draft.json) | `33f97f5cb3f1bd412fb7a1b6234aacfa8f3f3dbbd5cf4f0f381c485a15d33772` |
| [`switch-plan-writes.tsv`](owner-cutover/switch-plan-writes.tsv) | `59c48d8b7fb66766ee1fd1d6ca06d9f1f9998a21b1ba613bebf80ec004324c8a` |
| [`switch-plan-report.txt`](owner-cutover/switch-plan-report.txt) | `4ae0ac8ced952d96a960562022ce0e00b601e46242226aefdee87cdbb1a39645` |
| [`switch-plan-check.json`](owner-cutover/switch-plan-check.json) | `6523b9c4d2cdcf55539e857baf5e3efb30aefb1554e8735501caead881f7492e` |
| [`switch-plan-processes.txt`](owner-cutover/switch-plan-processes.txt) | `390d0c43875e92400ca0fd17499981293a616c4d2b3fc2f78510fd8d16ceaf8f` |
| `scripts/owner_switch.py` | `d2806f928c0b4687d79861be5a65405aac1edc64bc86593c5a211eca83988097` |
| `docs/evaluations/owner-skill-delivery/switch_plan_draft.py` | `01eb80330b6aa127f87dab0a480e04f6a8f228f1e54593eb24e5c844b1b463c8` |

### A write to the real HOME

The first run of the new check chose a Codex entry that is an absolute link
into the real `~/.agents/.grove`. Its `changed-entry` control appended one
newline through that link to a real snapshot file. The control then wrote
the saved bytes back. Afterwards the change identity of `~/.agents/.grove`
was `b14c7debee5d62b8446a505993f23cc23d140394e3f0f8ab60445f5f4b581781`,
which is the value of the earlier draft. The content is therefore unchanged,
but the file's modification time changed.

The control now chooses only a
real folder inside the disposable HOME, and it stops when the file leads
outside it. The second run left that modification time unchanged.

### The fixture suite

The suite lost the controls of each removed mechanism. These are the
exchange, held links, kept copies, merge, the inverse and pending records,
with run state, the preludes and rehearsal admission. It also lost its rerun of
`settings-writes.sh`. That rerun produced only the admission record, and its
interval measured the race of the held link.

The gate, baseline, plan,
receipt and process controls that ran the old `run` now run `switch`. New
controls cover each removed plan field, a source that changes during the
probes and the order of step 6's writes. The native-off control after a
stopped switch now uses the switch's refusal without the off value.

`task owner:switch:rehearse` passed all 142 controls on Python 3.15.1 and
macOS 27.0.1. The controls took 6.4 minutes. The earlier suite had 315
controls. These subjects had the same SHA-256 digests before and after the
run:

| Subject | SHA-256 |
| --- | --- |
| `scripts/owner_switch.py` | `d2806f928c0b4687d79861be5a65405aac1edc64bc86593c5a211eca83988097` |
| `scripts/owner_switch_rehearsal.py` | `1174b49a9891677d3b2b3ad532317f369845297d5f4fe9d0e62376bf3d243a50` |
| `~/.local/lib/machine-config/check.py` | `5c7a3bd43c08e43e02859c6c0c22edcb59e0bf0b38dcbe7b040784be247e9201` |
| `~/.local/bin/mac-check` | `b6738bfa2b04b21742a4693e9d4d5e86c9f69d56ccd8c73d36a484d6ade0a5e8` |

### The search for removed entries

A search for the dispatch of `run`, `inverse`, `merge`, `pending`, `state`
and `purge-check`, and for their command functions, found nothing in the
program, the suite or the draft generator. The same search found 19 lines in
the previous program, and a mutated pattern found none there. A search of the
documents covered the entry names, the mechanisms and the old plan schema.
It found them only in sections that state their scope as history. The two
matches in the specification state what the design omits.

### Mutation controls

A scratch copy of the program received one mutation at a time. Each mutant
ran only the controls named below.

| Mutation | Control that failed, and why |
| --- | --- |
| The loader accepts fields outside the schema | `plan-incomplete` and `plan-process-patterns` admitted `parts` and `session`. |
| An entry write may hold `set` | `plan-incomplete` admitted the entry with values. |
| `set` items may hold more keys | `plan-incomplete` admitted an earlier value. |
| `restore` need not start with the Ravel settings | `plan-incomplete` admitted the reversed order. |
| The switch deactivates no entry | `serial-switch-and-later-restore` exited 5, because the final comparison found the entries. |
| The receipt retains no superseded plugin registration | `receipt-evidence` lacked the retained registrations. |
| The report always shows an original in place | The draft check's `changed-entry` needle "now differs" did not match. This mutant ran against the check's disposable HOME directly. |
| Step 6 writes the Ravel settings first | No control failed at first. `serial-switch-and-later-restore` now checks the order of the writes, and it fails for this mutant. |

### Limits

- The draft's inputs do not name the probe program, because it has no
  source yet. owner-switch-plan-k254 completes the inputs.
- The draft's inputs name the repository digest of `inventory.py` at its
  prepared path. The switch refuses until preparation copies that file.
- The fixture plan is not the real plan. The dry run of
  owner-switch-dry-run-k271 executes the real plan in a disposable copy of
  HOME.

## Owner switch dry run

`owner-switch-dry-run-k271` added the dry-run instrument
`scripts/owner_switch_dry_run.py` on 2026-11-09. The sections
[The rehearsed plan](specs/owner-skill-delivery.md#the-rehearsed-plan) and
[Test seams](specs/owner-skill-delivery.md#test-seams) own the rules. The
module docstring states each entry, the input table, the substitutions and
the exit codes.

### The instrument

The instrument has three entries, and it uses only the Python standard
library. `task owner:switch:dry-run` and `task owner:switch:compare` run
them.

- `run` copies the prepared plan, the prepared program and each planned path
  into a new folder `OUT/home`. It then runs `native-off`, `attempt`,
  `switch` and the literal `restore` command of the attempt note in that
  copy. The ordinary writes, the hook commands, the probe cleanup and the
  final comparison all execute.
- `inputs` prints the input table of the real prepared inputs and writes
  nothing.
- `compare` reads the real prepared inputs through the same table, and it
  compares them with the result of a run. It writes only the file that
  `--out` names.

The executed plan differs from the prepared plan only in its path fields.
These are the paths of `inputs`, the items of each command, the staging
folders and the retained paths. The instrument rewrites no file content. The switch's
checks of original identity therefore apply unchanged in the copy.

`native-off` runs first in the copy, before any switch backup exists. The
result records that no backup existed before it, and the Ravel settings
before and after it.

The instrument removes the program's own variables from each command's
environment, and it sets `HOME` to the copy. It watches the source paths
and the account's `~/Library/LaunchAgents` before and after the run. A
change to either fails the run.

### The input table

Each row of the table holds a logical input, the executed path, a SHA-256
digest and a role. A tool row also holds the first line of its `--version`
output. The logical names come from the structure of the plan, not from
paths:

| Logical input | Content |
| --- | --- |
| `plan` | The prepared plan, with a second digest of its path-normalised form. |
| `program`, `interpreter:program` | The prepared program and the interpreter of its first line. |
| `interpreter:$python` | The Python that runs the program, with its version. |
| `input:<role>` | Each item of the plan's `inputs`, with its planned digest. |
| `command:<field>/<name>:<i>` | Each path item of each command. |
| `helper:<parent> <token>` | Each local helper of a command item, found statically. |
| `interpreter:<parent>` | The interpreter of a script's first line. |
| `tool:<name>` | `claude`, `brew`, `mise`, `ravel` and `grove`, with their versions. |
| `source:<id>` | Each skill source under the Homebrew prefix. |

A Python helper is a module that a script imports from its own folder or
from a literal `sys.path` folder. A shell helper is an absolute or `$HOME`
path token that names an executable or a script. The instrument digests
every row before and after the run. A changed row makes the run invalid,
and an invalid run does not pass.

### Substitutions

The comparison declares three prefixes: `home`, `grove-keg` and `ravel`.
It replaces the longest declared prefix of each path with `${name}`. A row
whose normalised path and content are equal is "substituted" with the
prefix's name. A row with equal content at an undeclared path is an
"unknown substitution", and it fails.

A row whose content or version differs fails, whatever its path. The plan
compares by its normalised digest. An undeclared prefix name exits 2, and so
does a name that the dry run did not declare. `compare` also fails when the
dry-run result did not pass.

### The fixture suite

The suite received eleven controls in a new group. Each control prepares a
fixture plan and program in the case's HOME, which stands for the real HOME.
It then runs the instrument into the case's folder.

| Control | Result |
| --- | --- |
| `dry-run-full` | All four steps passed. `native-off` wrote the off value without a backup. Both probes and the restore comparison passed. The case's HOME was unchanged, and the comparison with it was equal. |
| `dry-run-subject-changes` | A hook appended to the probe helper during the switch. The run named the changed rows and was invalid, and `compare` refused it. |
| `dry-run-declared-prefix` | Equal content at a released Grove keg compared as `substituted grove-keg`. |
| `dry-run-unknown-substitution` | Equal content at an undeclared prefix failed. An unknown name and an undeclared name exited 2. |
| `dry-run-changed-helper` | A changed helper at the same path compared as "content differs". |
| `dry-run-changed-plan` | A changed plan value failed, and `compare` named its plan path. |
| `dry-run-changed-source` | A changed skill source compared as "content differs". |
| `dry-run-tool-version` | A changed `claude` version compared as "version differs". |
| `dry-run-failed-probe` | A failed probe made the switch exit 5. The probe outcomes recorded the failure, and the case's HOME was unchanged. |
| `dry-run-source-written` | A write from the copy into the source HOME failed the run. |
| `dry-run-refusals` | A used `OUT`, an `OUT` inside a copied path, a `home` prefix and a missing plan each made the instrument stop. |

The controls also strip leaked program variables from the environment, and
they give the stub tools fixture versions. A fixture result states as its
first limit that it supplies no real-harness acceptance.

The control runner now records an unexpected error in a control as a failed
control, and the suite continues. Before this change, an error ended the
suite without a summary.

`task owner:switch:rehearse` passed all 153 controls on Python 3.15.1 and
macOS 27.0.1. The controls took 7.6 minutes. These subjects had the same SHA-256
digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `scripts/owner_switch.py` | `d2806f928c0b4687d79861be5a65405aac1edc64bc86593c5a211eca83988097` |
| `scripts/owner_switch_rehearsal.py` | `b2013ca74e1b0f737321e26558aa85627da6c3bc6434f1c954b6eae4dcc84905` |
| `scripts/owner_switch_dry_run.py` | `397a57c6221b0a69dd020a5b51ae512d1f4f9583c0ca80a5b3e2a90db87bbd21` |
| `~/.local/lib/machine-config/check.py` | `5c7a3bd43c08e43e02859c6c0c22edcb59e0bf0b38dcbe7b040784be247e9201` |
| `~/.local/bin/mac-check` | `b6738bfa2b04b21742a4693e9d4d5e86c9f69d56ccd8c73d36a484d6ade0a5e8` |

### Mutation controls

A scratch copy of the instrument received one mutation at a time. Each
mutant ran the eleven dry-run controls.

| Mutation | Controls that failed |
| --- | --- |
| The comparison ignores content | `changed-helper`, `changed-plan`, `changed-source` |
| A moved row needs no declared prefix | `unknown-substitution` |
| The run ignores changed subjects | `subject-changes` |
| The table finds no Python helpers | `full`, `subject-changes`, `changed-helper` |
| The comparison ignores versions | `tool-version` |
| The executed plan keeps the source paths | `full`, `subject-changes`, `declared-prefix` |
| The run skips the restore | `full` and six other controls |
| The run does not watch the source paths | `source-written` |
| The run skips `native-off` | `full` and nine other controls |
| `compare` accepts a result that did not pass | `subject-changes` |
| The table finds no shell helpers | `full` |

### Two measurement incidents

The first measurement ran during an edit of the suite's docstring. The
instrument recorded the suite as a changed subject, and I discarded that
run. Every later run began after the last edit.

A later mutant run stopped at its third control. The no-helpers mutant
produced no helper row, and the `changed-helper` control raised `KeyError`.
The runner ended without a summary, and the watcher of the run waited for
one. The runner change above corrects this, and the measurement above ran
after it.

### The real HOME

`~/.local/state/owner-switch` does not exist, so the real HOME holds no
prepared plan. `inputs` and `run --out` both stopped with exit 3 and
"cannot be read as JSON". `run` created no `OUT` folder and no state folder.
The listing of `~/Library/LaunchAgents` had the same digest before and
after.

### Limits

- `tool:grove` compares by `grove --version`. Inside finish, the candidate
  keg carries the pre-release Cargo version, so that row differs from a dry
  run against the released keg. owner-switch-rehearsal-k255 and
  discovery-rollout-k12 record the choice: another dry run against the
  released keg, or a candidate keg built at the release commit.
- The instrument finds helpers statically. A helper that a program finds at
  run time, for example through a computed path, has no row.
- The instrument watches launchd jobs through `~/Library/LaunchAgents`. It
  does not observe login items.
- Copied files keep their bytes, so an absolute path inside a file still
  names the source HOME. The source watch fails a run that writes there.
- Every result in this section is a fixture result. None of them supplies
  real-harness acceptance.

## Staging setup and isolation trial

`owner-staging-setup-k248` built the retained staging HOME and ran its
isolation trial on 2026-11-09. The sections
[Settings isolation and the staging HOME](specs/owner-skill-delivery.md#settings-isolation-and-the-staging-home)
and [Cutover contract](specs/owner-skill-delivery.md#cutover-contract) own
the rules. The tool is
[`staging_home.py`](evaluations/owner-skill-delivery/staging_home.py).
`task owner:staging:isolation` runs the trial, and
`task owner:staging:launch` runs one staging session.

### Retained paths

These paths stay until the switch writes its receipt. The plan's
`accepted.staging` names the first two.

| Subject | Absolute path |
| --- | --- |
| Staging HOME | `/Users/antony/.local/state/owner-staging/home` |
| Candidate keg | `/Users/antony/.local/state/owner-staging/candidate` |
| Accepted Ravel 2.0 build | `/Users/antony/.local/state/owner-staging/ravel-2.0` |
| Isolation record | `/Users/antony/.local/state/owner-staging/isolation-2/record.json` |
| Record of the first run | `/Users/antony/.local/state/owner-staging/isolation-1/record.json` |
| Replaced candidate keg, removable | `/Users/antony/.local/state/owner-staging/candidate-2dea55cd` |

The staging HOME contains no source, and no source contains it. The tool
checks both before it writes.

### The accepted inputs

`task install` in `Ravel.skill-directory-delivery`, at `ff14cba826b0` with an
empty working copy, filled the Ravel prefix. Its `bin/ravel` has the SHA-256
`3a9a9129573e4d827a317f6b793ba990044080a31a7b667b64cc153a2f974a0b`, which
`scripts/harness-trials/builds.json` pins. The prefix holds no `libexec`.

`task owner:candidate-keg` built the candidate keg from
`ef8a79dd7621a437177982e24d8ec27831b4add4`, which `origin/gh-issue-25`
holds. Its `plugins/grove` has the tree ID
`885051a94eddaa81fa7a1a9900c3ac4db775aa84`, and its `plugins/packs.json`
has the blob ID `2cebaf5f062d08b50b22d058bd8c4db3d412f8a6`. The unreleased
archive `grove-v22.8.0-aarch64-apple-darwin.tar.xz` has the SHA-256
`fcb7603d70ccadcb3fcb071c5c2f79da165c807656f4723620142acab577546b`.
[The hook bytecode fix](#the-hook-bytecode-fix) replaced the first candidate,
`7b113a28a966`, and records its values.

Homebrew installed the three tap formulae with `HOMEBREW_NO_AUTO_UPDATE`,
`HOMEBREW_NO_INSTALL_CLEANUP`, `HOMEBREW_NO_INSTALL_UPGRADE` and
`HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK` set. None has a dependency. `ravel`
1.1.0 and `grove` 22.8.0 stayed installed and unchanged.

| Formula | Installed version | Archive commit | Source |
| --- | --- | --- | --- |
| `personal` | 2026-11-06 | `cddcfde901cdaf8069e2aac6f3b6bc5aba9fe925` | `personal` |
| `superpowers` | 7.0.0 | `bb92a77741419a4ab5f06e711a283343f1ada0c3` | `superpowers` |
| `quint-llm-kit` | 1.0.0 | `cc75369f741af7d490936f82002c2d28e3b3d78d` | `quint` |
| `testanyware` | 2.8.0, installed earlier | Release `v2.8.0`, `1809a49068a23d9d276913f3a90227dbefb98a82` | `testanyware` |

The accepted 2.0 build resolved `docs/owner-cutover/settings-staging.json`
under `env -i` with a new HOME. Each `resourceDigest` equals the value of
[The handoff to the owner cutover](repository-moves-record.md#the-handoff-to-the-owner-cutover).
`originPath` names the versioned keg of each source.

| Source | Keg | `resourceDigest` |
| --- | --- | --- |
| `design` | Candidate | `d8ad3a2af1b5029bc0de93747f4d7297038651180f6877fbd738d94fcbf6aa14` |
| `engineering` | Candidate | `7e70b280e7a932489a048d108d4dae2bc63b387cf8fc6b37298663336e0fcc66` |
| `grove-core` | Candidate | `8d92cd58fca8f21a87361a9d956af7e5a38e88751e25cbaff5e2384554806b51` |
| `requirements` | Candidate | `07b2d25e8bd20259c71f7419092329368d8ae48bec6ca973343a601a8ec515c8` |
| `research` | Candidate | `7fb3d473f66feafb49afd8846fd94cf3e1fb7b01c584d4ad5f0e54814a8b3fdb` |
| `writing` | Candidate | `56bcd07b87f46a7b9ff92eed5a5a128f10d3ce64be2f7d934be898e058782212` |
| `personal` | `/opt/homebrew/Cellar/personal/2026-11-06` | `dc1f26eafe9b7db6f750c95db38a7876beb29b067d829eaf63203323b89d61b6` |
| `quint` | `/opt/homebrew/Cellar/quint-llm-kit/1.0.0` | `0a8b945fbc6732d5b7b3041a966d26cdf718ca884bd5f3fb8435297d7258edef` |
| `superpowers` | `/opt/homebrew/Cellar/superpowers/7.0.0` | `be1ce6e3c25064b83ba2999b37972deb7f73ce88a5f5dbbba990d88037b17230` |
| `testanyware` | `/opt/homebrew/Cellar/testanyware/2.8.0` | `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6` |

Both staging sessions captured the same ten digests.

### The link layout of the first skill formula

The first resolution refused as `skill_resource_invalid`. TestAnyware's
`.claude-plugin/plugin.json` in the default directory was a link to
`../../../../../Cellar/testanyware/2.8.0/...`, which leaves the source.

Before the installation, TestAnyware was the only keg with `share/ravel`, so
Homebrew linked that whole directory. Linking `personal` made Homebrew call
`Keg#resolve_any_conflicts`. That method relinks the displaced keg with
`keg.link_dir(src, ...) { :mkpath }`, in `Library/Homebrew/keg.rb` of
Homebrew 7.0.12 at `3ccfa1d07143`. Every directory of the displaced source
became a real directory, and every file became a link. Ravel keeps only
relative links that stay inside the source, so it refused the source and the
whole resolution.

`brew unlink testanyware` and `brew link testanyware` then made one directory
link, because `share/ravel/skills` was by then a real directory. The ten
digests matched after the relink. "The keg layout" of
`docs/specs/codex-skill-provisioning.md` describes a link of each entry
instead, which is wrong for the first skill formula on any machine.
`skill-keg-link-layout-k274` owns the design repair.

### The staging HOME

| Item | Content |
| --- | --- |
| `.config/ravel/config.rhai` | A copy of the owner's installed file, SHA-256 `0ccd5cdaa954cf3f7cce10e162a1aa42695141181bf1040910e46a7aba28efa7` |
| `.config/ravel/settings.json` | `docs/owner-cutover/settings-staging.json`, SHA-256 `d8a49f066310b0976cb4e13674c524d96d31b6aa0ba5bfc6923fc0b097623145` |
| `.claude/settings.json` | The owner's settings with `enabledPlugins` and `extraKnownMarketplaces` replaced. Real `.claude/` paths in commands name the staging `.claude`. |
| `.claude/hooks`, `.claude/agents`, `.claude/skills/codebase-memory`, the status line | Copies of the tool-managed residue |
| `.claude.json` | Onboarding done, automatic updates off, and the owner's MCP entries `quint-kb` and `codebase-memory-mcp` |
| `.codex/config.toml` | The owner's file without `[projects.*]`, `[hooks.state*]`, `[marketplaces.*]`, `[plugins.*]`, `[desktop*]` and `notify`. Its MCP entries are `node_repl`, `context7`, `openaiDeveloperDocs`, `quint-kb`, `codebase-memory-mcp` and `computer-use`. |
| `.codex/hooks.json`, `herdr-agent-state.sh`, `AGENTS.md`, `instructions.md` | Copies, with real `.codex/` paths moved to the staging tree |

The owner's installed `config.rhai` is not the committed candidate
`owner-config.rhai`, SHA-256 `bbde65ef…`. Since 2026-11-08 18:21 its `lead`
selects `sol` and its `review` selects `sonnet`. The staging HOME copies the
installed file, because the owner chose it. The switch plan identifies its
inputs by content, so owner-switch-plan-k254 reads the installed digest.

Claude Code's own commands registered the plugins, with HOME the staging
HOME. They added `claude-plugins-official` and installed `commit-commands`
1.0.1, `context7` `0412be76f50a`, `explanatory-output-style` 1.0.1,
`feature-dev` 1.0.1, `playwright` `0412be76f50a` and `swift-lsp` 1.0.0. They
added the `personal` marketplace as the directory `/opt/homebrew/opt/personal`
and installed `guardrail@personal`, which loads from
`/opt/homebrew/opt/personal/libexec/plugins/guardrail`. No superseded plugin
or skill is installed, and `writing-hooks` is not registered.

### The isolation trial

The trial ran from 05:04:05Z to 05:04:49Z with Claude Code 2.1.319 and Codex
0.167.0. It digested 18 real subjects before `build` and after the token
refresh. The subjects are the user settings, `settings.local.json`,
`.claude.json`, the Claude Code hooks, agents, plugin registries, marketplace
clones, plugin cache and skill root. They are also Codex's `config.toml`,
`hooks.json`, hook helper, `AGENTS.md`, plugin tree and skill root, the
`~/.agents/skills` root and `~/.config/ravel`.

| Step | Result |
| --- | --- |
| 1. Links | Before any session, `Library/Keychains` and `.codex/auth.json` were the only links. After the sessions, they were the only links that lead out of the staging HOME. npm added three `.bin` links in `.npm/_npx/9833c18b2d85bc59/node_modules/.bin`, which resolve inside the staging HOME, when Claude Code started the Playwright MCP server. |
| 2. Superseded entries | `.codex/skills` held only `.system`. `.agents/skills` held nothing. `.claude/skills` held `codebase-memory` and the account's `synced` folder. Claude Code's listing held no superseded name. Codex's `debug prompt-input` listed `imagegen`, `openai-docs`, `skill-creator` and `skill-installer`, which are its system skills. |
| 3. Configuration | `claude auth status` named `/Users/antony/.local/state/owner-staging/home/.claude`. Codex's skill root was `/Users/antony/.local/state/owner-staging/home/.codex/skills/.system`. No `CODEX_HOME` was inherited. The Codex overrides were `-c model_reasoning_effort="low"`, `--sandbox workspace-write` and `-c approval_policy="never"`. Each of the five Claude Code variables refused, and a `CODEX_HOME` in the real HOME refused. A working directory in the real HOME or in the staging HOME refused, and one in `/private/tmp` was admitted. |
| 4. Sessions and digests | Claude Code with `claude-sonnet-5-5` and Codex with `gpt-6.1-sol` each ran through `ravel start-session` of the 2.0 build and returned `isolation-ok`. No digest of the 18 subjects changed, `.claude.json` included. |
| 5. Token refresh | An app-server session sent `account/read` with `refreshToken: true`. `last_refresh` in the real `auth.json` moved from 05:01:53Z to 05:04:48Z. The real file stayed a regular file with mode 0600, and the staging `.codex/auth.json` stayed a link to it. The real HOME's `codex login status` printed "Logged in using ChatGPT". |

Each staging launch received only `CODEX_HOME`, `DISABLE_AUTOUPDATER`,
`HOME`, `LANG`, `LOGNAME`, `PATH`, `TERM`, `TMPDIR` and `USER`. No Codex
app-server daemon of the staging `CODEX_HOME` was running after the trial.

Codex 0.167.0 resolves a link chain before it saves `auth.json`. It writes
the replacement beside the resolved target and renames it there, and it
takes the target's lock. `storage_file.rs`, `storage.rs` and
`storage_commit.rs` in `codex-rs/login/src/auth` at `rust-v0.167.0` hold
this code. Codex refreshes by itself only in two cases: within five minutes
of the access token's expiry, or eight days after the last refresh. The trial
therefore asked for the refresh explicitly.

### The first run

The first run, in `isolation-1`, failed two checks:

- Its sessions ran in a folder under the staging HOME. Claude Code listed
  `grove`, the stale copy in the real `~/.claude/skills`. Claude Code reads
  `.claude/skills` in each parent folder of its working directory, and the
  staging HOME lies inside the real HOME. The specification and the launcher
  now require a working directory outside the real HOME, and the second run
  listed no superseded skill.
- Codex refused `gpt-6.1-luna` for a ChatGPT account, so its session ended
  without a result. The tool now uses `gpt-6.1-sol`.

The first run also refreshed the token once, through the same link. Its
digests showed no change either. The tool removed that staging HOME, links
first, and the second run built it again.

### Limits

- Claude Code reported `plugin:context7:context7` as `needs-auth` and
  `codebase-memory-mcp` as `failed` in the staging HOME. `quint-kb` and the
  Playwright server connected. The staging HOME copies no credential.
  owner-staging-capabilities-k250 owns MCP use.
  [MCP servers in staging](#mcp-servers-in-staging) records that use and
  the two staging changes that it needed.
- The residue plugins came from the marketplace's current state. Their
  versions can differ from the owner's installed versions.
- Ravel's operation reports held `stop attempt 3` and `stop attempt 4` with
  "signal 9: Operation not permitted" against each ended session's group.
  Both sessions still returned their results with exit 0.
- Homebrew updated its own checkout to `bc63375ac3a4` at 04:51:53Z, during
  this leaf. The trigger was not identified. The cited `keg.rb` call is
  unchanged there, and no formula changed version.
- Codex daemons of earlier trials still run for six disposable HOMEs under
  `/private/tmp`, and the real HOME's daemon still runs. The trial signalled
  none of them.

## Staging discovery trials

`owner-staging-discovery-k249` runs the discovery trials of the staging HOME.
"Test seams" and "Cutover contract" of the
[owner specification](specs/owner-skill-delivery.md#test-seams) own them.
The tool is [`staging_trial.py`](evaluations/owner-skill-delivery/staging_trial.py),
and its cases are in
[`staging-cases`](evaluations/owner-skill-delivery/staging-cases).

`staging-trial-runner-k275` built the tool on 2026-11-09. It ran the source
trials and the discovery case on both harnesses.
`staging-kind-delegate-trials-k276` added the kind skill case and the
delegate case on the same day, and ran each on both harnesses.
`staging-compaction-stale-trials-k277` added the compaction case and the
stale-copy case on the same day, and ran each on both harnesses.
`staging-guidance-probe-k278` added the guidance probe case on the same day,
and ran it on Claude Code.

### Rerun commands

Each trial runs with HOME set to the staging HOME and `CODEX_HOME` set to
its `.codex` tree. The guidance probe case is the one exception. It runs with
HOME set to a clone of the staging HOME at `OUT/home`, and `CODEX_HOME` set
to the clone's `.codex` tree. Each launch runs `ravel start-session` of the
accepted 2.0 build with `--config`, so no trial writes the retained
`config.rhai`. After a released source changes, the operator runs these
commands with a new `OUT`:

```sh
S=~/.local/state/owner-staging
task owner:staging:sources STAGING=$S/home CANDIDATE=$S/candidate \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/sources-N
task owner:staging:case CASE=discovery HARNESS=claude STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/discovery-claude-N
task owner:staging:case CASE=discovery HARNESS=codex STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/discovery-codex-N
task owner:staging:case CASE=kind-skill HARNESS=claude STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/kind-skill-claude-N
task owner:staging:case CASE=kind-skill HARNESS=codex STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/kind-skill-codex-N
task owner:staging:case CASE=delegate HARNESS=claude STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/delegate-claude-N
task owner:staging:case CASE=delegate HARNESS=codex STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/delegate-codex-N
task owner:staging:case CASE=compaction HARNESS=claude STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 MODEL=claude-opus-5-5 EFFORT=high \
  OUT=$S/trials/compaction-claude-opus-N
task owner:staging:case CASE=compaction HARNESS=codex STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/compaction-codex-N
task owner:staging:case CASE=stale-copy HARNESS=claude STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/stale-copy-claude-N
task owner:staging:case CASE=stale-copy HARNESS=codex STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/stale-copy-codex-N
task owner:staging:case CASE=guidance-probe HARNESS=claude STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/guidance-claude-N
```

The guidance probe case runs only on Claude Code. It installs
`grove@linkuistics` from GitHub, so it needs the network and the owner's
GitHub access through SSH.

The Claude Code compaction case runs with `claude-opus-5-5` at high effort,
which the owner's policy gives an `impl` session. With the staging default,
`claude-sonnet-5-5` at low effort, it failed in each of five runs. The
section "The compaction case" below gives the reason. The stale-copy case
copies the owner's real `~/.claude/skills/grove`. The switch deactivates that
copy, so the case runs only while native delivery holds it.

`task owner:staging:trial-tests` tests the verdicts on synthetic events.
`staging_trial.py rejudge --out DIR` extracts and judges a finished case
again from its unchanged launch records. The switch's gate 4 runs these
trials through `staging_trial.py rerun` instead, as
[Trials for gate 4](#trials-for-gate-4) states.

### Source trials

The source trials ran from 05:24:59Z to 05:25:03Z. Their record is
`~/.local/state/owner-staging/trials/sources-1/record.json`. The Ravel
build was `ff14cba826b0`, with the pinned `bin/ravel` SHA-256
`3a9a9129…`. The staging settings had the SHA-256 `d8a49f06…`.

| Trial | Result | Control |
| --- | --- | --- |
| Off value | In a disposable HOME, `docs/owner-cutover/settings-off.json` resolved as `{"schemaVersion":1,"status":"disabled"}` with exit 0. | The staging settings resolved as `resolved` with ten sources in another disposable HOME. The `disabled` check failed on them. |
| Offline launch | A launch under the profile `(deny network*)` received the same descriptor path, 13 directories, mandatory file and ten source identities as a launch with a network. | An unguarded probe reached a local listener with exit 0, and the guarded probe failed with exit 1. The comparison reported `directories` for a view without its last directory. |
| Missing source | With the keg directory of `research` moved away, Ravel exited 3 with `skill_source_missing` at the stage `skills`, for `skills.sources[6].path`. The route wrapper recorded no launch. | The launches with the source present each recorded one launch. |
| Restored source | The tree digest of the keg directory was the same before the move and after the return. The launch after the return received `7fb3d473…`, the accepted `resourceDigest` of `research`. | The launch before the move received the same digest. |

Each source launch ran `/usr/bin/true` behind the route wrapper. The wrapper
records the request view before the worker runs, so no model ran. Ravel then
reported `session_result_missing` with exit 1, as expected for a worker that
sends no result. The offline launch found every snapshot already in the
staging cache. Ravel copies the current bytes of each local source on each
resolution, and a local source needs no network.

### The discovery case

The case asks for a small Python command-line tool that other agents run,
and it names no skill. The verdicts require these facts:

- The agent reads the mandatory file `using-superpowers` before it looks for
  another skill and before any task action.
- The agent reads `cli-tool-design` from the design snapshot before its
  first edit.
- The native skill listing of the session holds no superseded name.
- No real-HOME subject changes. The owner's own session rewrites the real
  `.claude.json`, so the check requires only that it names no staging path.

| Harness | Model and effort | Record | Result |
| --- | --- | --- | --- |
| Claude Code 2.1.319 | `claude-sonnet-5-5`, low | `trials/discovery-claude-1` | Passed. The mandatory read was event 1 and a supporting file event 2. The listing of the search directories was event 3. The read of `cli-tool-design` was event 5, and the first write was event 6. The session ended with `staging-ok`. |
| Codex 0.167.0 | `gpt-6.1-sol`, low | `trials/discovery-codex-1` | Passed. The mandatory read was event 2 and a supporting file event 3, after one text event. A Python script printed the front matter of every skill at event 6. Another script printed the whole of `cli-tool-design/SKILL.md` from the design snapshot at event 7. The first edit was event 10. The session ended with `staging-ok`. |

Claude Code's native listing held built-in skills, the skill-free plugins'
commands and `guardrail:guardrail`. Codex listed `imagegen`, `openai-docs`,
`skill-creator` and `skill-installer`, which are its system skills. Neither
listing held a superseded name.

The staging PATH holds `python3` but no `python`. Codex first tried
`python`, reported it missing and used `python3`.

### Two extraction repairs

The first extraction of each trial failed its `requires` verdict, although
each agent had read the skill before its first edit. Both failures were
defects of the instrument. Two flags of `harness_trial.py` now repair them.
Each flag is off there, so the accepted common-loading records extract
unchanged. The staging runner turns both on.

- **Globs.** Claude Code read the skill with
  `cd …/snapshots; cat d8ad*/tree/craft/cli-tool-design/SKILL.md`. The path
  extractor did not expand a glob, so it classified that read as a plain
  command. `EXPAND_GLOBS` expands a glob token as the shell does.
- **Reads inside a script.** Codex read the skill inside a
  `python3 - <<'PY'` script, which names no path on its command line.
  `KEEP_RESULTS` keeps each tool result during extraction. The staging
  verdict counts a read when a result holds the skill's `name:` line and
  the first heading of its body. The listing at event 6 held the first line
  but not the heading, so it did not count. The record keeps each result
  only as its digest and length.

The `requires` verdict also reads each further read of one command. A new
extraction of the same unchanged launch records then passed on both
harnesses.

### The kind skill case

The case resembles a Grove mandate. Its prompt names `grove-planning` and
the planning leaf `word-tools-k1` of a small planted grove. The prompt
gives the end command `ravel end-session staging-ok`. The session runs
through `ravel start-session --kind planning`, not through the `grove`
driver. The runner copies `staging-cases/trees/kind-skill` to `.grove/` of
a new jj repository in the work directory, and commits it.

The case names `grove-planning` because the kind skill must come from the
`grove-core` source. That source holds `grove`, `configure-grove`,
`grove-finish` and the three planning kinds. The `engineering` source holds
`grove-impl`.

The verdicts require these facts:

- The agent reads `grove-planning` from the `grove-core` snapshot before
  the first event that touches the work tree. Such an event edits a file,
  names a path in the work tree or mentions `.grove`. The Grove mandate
  sets this order.
- The agent reads a file of the `grove-core` snapshot other than a
  `SKILL.md`. Each such file is a resource that the kind skill or the spine
  names by a relative path, such as `grove/TASK-FORMAT.md` or
  `grove/references/bootstrap.md`. The record keeps the position of the
  read, and the verdict does not judge it.
- The native listing holds no superseded name, and the real HOME is
  unchanged.

The runner records the mandatory order of this case, and does not enforce
it. A Grove mandate tells the agent to load the kind skill first.

| Harness | Model and effort | Record | Result |
| --- | --- | --- | --- |
| Claude Code 2.1.319 | `claude-sonnet-5-5`, low | `trials/kind-skill-claude-3` | Passed with the final runner. The mandatory read was event 1. The kind skill was event 5 and the spine event 6. Event 8 read the task file, and event 9 read `grove/TASK-FORMAT.md` and other spine files. The agent added two leaves with `grove-llm leaf-add` at event 12, wrote them through the shell, retired its leaf and committed. The session ended with `staging-ok`. |
| Codex 0.167.0 | `gpt-6.1-sol`, low | `trials/kind-skill-codex-2` | Passed. The mandatory read was event 2. The kind skill was event 5. Event 8 read `grove/references/{execute,decompose,retire,commit,grove}.md` through a shell brace. Event 9 read the task file. The agent added two leaves, edited them with `apply_patch`, retired its leaf and committed. The session ended with `staging-ok`. |

Both agents read the mandatory file before the kind skill, although the
prompt asked for the kind skill first. Neither listing held a superseded
name.

### The delegate case

The case asks for two Bash scripts in an empty work directory. It asks the
parent to write `count.sh` itself and to delegate `check-count.sh` through
the harness's agent tool. The parent must tell the delegate to find and
read the skill for Bash style, which the prompt does not name. The case
enforces the mandatory order of the parent. The delegate verdict requires a
delegate transcript with a read of a file of `coding-style-bash` from the
`engineering` snapshot.

| Harness | Model and effort | Record | Result |
| --- | --- | --- | --- |
| Claude Code 2.1.319 | `claude-sonnet-5-5`, low | `trials/delegate-claude-1` | Passed. The parent read the mandatory file at event 1 and launched one `general-purpose` delegate. The delegate prompt gave the absolute path of `coding-style-bash/SKILL.md` and the descriptor path. The delegate read that file at its event 1 and wrote `check-count.sh` at its event 3. The session ended with `staging-ok`. |
| Codex 0.167.0 | `gpt-6.1-sol`, low | `trials/delegate-codex-1` | Passed. The parent read the mandatory file at event 2 and launched one delegate with `spawn_agent`. Its message is encrypted in the session file. The delegate read the mandatory file at its event 1 and the descriptor at its event 2. A Python script listed the skills at its event 3, and the delegate read `coding-style-bash/SKILL.md` at its event 4. Its first edit was its event 6. The session ended with `staging-ok`. |

Each delegate inherited `RAVEL_EXIT_FILE`, and neither ran an end command.
The Claude Code parent found the skill itself and passed its path, so that
delegate shows access to the snapshot, not discovery. The Codex delegate
found the skill through the descriptor.

### Repairs of the kind verdict and the real-HOME check

The first kind trials found defects in the instrument. None was a defect of
a source.

- **A colocated plant.** The first plant was a colocated jj repository.
  Codex's workspace sandbox denies writes below `.git`, so the Codex agent
  of `trials/kind-skill-codex-1` could not commit. It sent no result, and
  Ravel reported `session_result_missing`. Grove's own launches grant the
  store through the `grove-jj` profile, which a trial route does not pass.
  The plant is now not colocated, as the common-loading plant is.
- **No edit tool.** The Claude Code agent of `trials/kind-skill-claude-1`
  did its planning through `grove-llm leaf-add` and shell writes. The first
  verdict ordered the reads against the first edit, and found none. The
  order point is now the first touch of the work tree. A new flag of
  `harness_trial.py`, `KEEP_CLASSES`, keeps the class of every path of a
  command. A command that reads a skill and a task file then shows both. The
  flag is off there, so the common-loading records extract unchanged. The
  extractor counts a relative path only when it still exists. Retirement
  renames the task file, so a mention of `.grove` also counts as a touch.
- **An order for the resource.** The first two versions of the order point
  also required the resource read before or with the first touch of the
  work tree. `trials/kind-skill-claude-2` read the task file at event 8 and
  `grove/references/bootstrap.md` at event 9, before its first write at
  event 12. The spine's bootstrap names the task file, so this order is
  Grove's procedure. The mandate sets an order only for the kind skill, and
  the done-when of the case sets none. The verdict now records the position
  of the resource read without a judgement.
- **Reader markers in the real plugin cache.** The real-HOME check of
  `trials/kind-skill-claude-1` reported a changed `.claude/plugins/cache`.
  The changed files were `.in_use/81902` markers. Process 81902 was the
  owner's own Claude Code session, which ran this leaf. Its process
  began before the trial. The check now digests the cache without the top-level
  `.in_use` of each version directory, as the plugin row of
  `inventory.py` does. It fails when the trial adds or rewrites a marker
  that names a process that has ended. Ravel reaps the trial's processes
  before the check, so a marker of a trial process names an ended process.
- **One delegate listed twice.** The Claude Code session file of
  `trials/delegate-claude-1` holds the parent's Agent call twice, with one
  tool-use ID. The verdict and the rendered record now count a delegate
  once by that ID.

`staging_trial.py rejudge` judged `trials/kind-skill-claude-2`, both Codex
records and `trials/delegate-claude-1` again with the final runner, from
unchanged launch records. Each passed. `trials/kind-skill-claude-3` ran
after the last change to the kind verdict, and it passed without a second
reading. A later reading cannot repeat a real-HOME check, so
`trials/kind-skill-claude-1` keeps its original failure of that check.

### The compaction case

The case runs three turns in one managed process, through the feeder
`harness_trial.py turns`. The first turn asks for a small Python tool and
then a stop. The second turn is the harness's default compaction. The third
turn asks for a Bash script and the end command, and it names no skill. The
case enforces the mandatory order of the first turn.

The case measures the harness's own compaction, not a chosen loss. Claude
Code compacts with a bare `/compact`. Codex compacts with
`thread/compact/start`, and its provider uses remote compaction. No turn has
a gate. The common-loading cases measured the losses of the path and of the
whole preamble. The section "Compaction" of
[the acceptance record](common-loading-acceptance-record.md#compaction) holds
those results.

The verdicts require these facts:

- The session holds a compaction boundary.
- After the boundary, the agent reads `coding-style-bash` from the
  `engineering` snapshot before its first edit after the boundary.
- The native listing holds no superseded name, and the real HOME is
  unchanged.

The record also keeps what the context after compaction still holds of
Ravel's preamble. The verdict does not judge it.

| Harness | Model and effort | Record | Result |
| --- | --- | --- | --- |
| Codex 0.167.0 | `gpt-6.1-sol`, low | `trials/compaction-codex-1` | Passed. The boundary was event 14. The agent read the descriptor at event 17, the mandatory file again at event 18 and `coding-style-bash` at event 23. Its first edit after the boundary was event 26. The session ended with `staging-ok`. |
| Claude Code 2.1.319 | `claude-opus-5-5`, high | `trials/compaction-claude-opus-1` | Passed. The boundary was event 21. The agent read the descriptor at event 24 and listed the search directories at event 25. It read `coding-style-bash` at event 27, before its first edit at event 28. The session ended with `staging-ok`. |
| Claude Code 2.1.319 | `claude-opus-5-5`, high | `trials/compaction-claude-opus-2` | Passed. The boundary was event 17. The agent read the descriptor at event 20 and the mandatory file again at event 21. It wrote that it read the skills again because the harness compacted the context. It read `coding-style-bash` at event 23, before its first edit at event 25. The session ended with `staging-ok`. |
| Claude Code 2.1.319 | `claude-sonnet-5-5`, low | `trials/compaction-claude-1` to `-5` | Failed in each of five runs. In each, the agent read the mandatory file and then wrote `tally.py` with no skill search. After the boundary, it wrote `lines.sh` with no skill read. Each session ended with `staging-ok`. |

The Codex context after compaction kept the whole preamble: 15 preamble paths
and 27 long preamble lines. Remote compaction keeps each user message word for
word. Each Claude Code summary kept the descriptor path and two snapshot paths.
Claude Code also restored `using-superpowers/SKILL.md` and its supporting file
after the boundary. So in no run did compaction remove the means of discovery.

### The failures at low effort

The five low-effort failures on Claude Code are a finding about skill search
in the feeder's input mode, not about compaction. The first turn skipped the
search too, before any compaction. Two diagnostics outside the case files
separated three causes. Each ran with `claude-sonnet-5-5` at low effort and
with no snapshot root as `--add-dir`.

| Diagnostic | Record | Result |
| --- | --- | --- |
| The discovery case, without snapshot `--add-dir` | `trials/discovery-claude-no-add-dir-1` | Passed. The agent listed the search directories at event 3 and read `cli-tool-design` at event 5, before its first write at event 7. |
| The first-turn prompt of the compaction case, in plain `-p` mode without the feeder | `trials/compaction-prompt-plain-claude-1` | The discovery verdicts passed. The agent listed the search directories at event 3 and read `cli-tool-design` at event 5, before its first write at event 6. Ravel exited 1, because this prompt sends no end command. |

The owner's real Claude Code policy passes no snapshot root as `--add-dir`. So
the first diagnostic also shows discovery with the owner's launch shape. The
trial routes of the discovery, kind and delegate cases pass the snapshot roots,
as the Rhai sample does.

Neither `--add-dir` nor the prompt caused the failures. The remaining
difference is the feeder's `--input-format stream-json` mode. The compaction
case needs that mode to send three turns into one process.

The first model request of a plain run and of a feeder run hold the same
messages and the same parameters. Only Ravel's caller-block nonce differs.
Their tool lists differ only in the default of one background timeout. Each
run also received Claude Code's notice that the `context7` MCP server needs
authentication. No difference in the model's input explains the result.

The owner's policy routes no kind to `claude-sonnet-5-5` at low effort. Each
Grove kind that runs on Claude Code uses `claude-opus-5-5` at medium effort or
higher. Only the standalone `release-notes` kind uses `claude-sonnet-5-5`, at
medium effort. At the setting of an `impl` session, both runs passed. The cause
of the low-effort failures stays unknown.

### The stale-copy case

The case copies the owner's real stale copy, `~/.claude/skills/grove`, into
the work directory. It puts one copy in `.claude/skills` and one in
`.codex/skills`. That copy is an older provisioned spine. It holds a table of
kinds and `references/planning.md`, which the current spine does not have. It
matches no single commit, so the case pins its tree digest `d9ae18c2…` and
refuses a changed or missing copy. The staging HOME never holds the copy.

`codex debug prompt-input` in the staging environment measured where Codex
0.167.0 reads a project skill. It lists `.codex/skills` and `.agents/skills`
of its working directory, or of the Git root above it. It does not list
`.claude/skills`. Claude Code reads `.claude/skills` in each parent folder of
its working directory. So each harness lists exactly one planted copy.

The prompt is the planning mandate of the kind case, on the same planted
tree. A Grove mandate leads the agent to the `grove` spine. The verdict
records which copy the agent reads:

- A stale read names a path of a planted copy, loads `grove` through the
  native Skill tool, or returns a line that only the stale `SKILL.md` holds.
- A Ravel read is a classified read from the `grove-core` snapshot, or
  returns a line that only the current `SKILL.md` holds.

The case fails when the agent reads neither copy, or when the native listing
does not hold the planted `grove`. The planted name is the one exception to
the rule that the listing holds no superseded name.

| Harness | Model and effort | Record | Result |
| --- | --- | --- | --- |
| Claude Code 2.1.319 | `claude-sonnet-5-5`, low | `trials/stale-copy-claude-1` | Passed. The native listing held `grove`. The agent read `grove-planning` at event 5 and the snapshot's `grove/SKILL.md` at event 6, then spine references at events 9 to 11. It did not use the Skill tool, and it read no file of the planted copy. Copy read: Ravel's only. |
| Codex 0.167.0 | `gpt-6.1-sol`, low | `trials/stale-copy-codex-1` | Passed. The native listing held `grove`. The agent printed skill files through a script at event 5, which returned lines of Ravel's copy. It read `grove-planning` at event 6, the snapshot's `grove/SKILL.md` at event 8 and spine references after that. It read no file of the planted copy. Copy read: Ravel's only. |

In both records the staging HOME held no `grove` copy before or after, and
the real-HOME check found no change.

The first reading of `trials/stale-copy-codex-1` reported a stale read at
event 8. That was a defect of the instrument. A unique line was then one that
no whole line of the other file equals, but the match tests a substring. The
current `SKILL.md` holds `compressed technical shorthand.` inside a longer,
differently wrapped line. A unique line now must not occur anywhere in the
other file. `staging_trial.py rejudge` on the unchanged launch records then
showed a read of Ravel's copy only.

### The guidance probe case

The case reproduces finish's guidance in the switch. Finish loads its
guidance from absolute plugin-cache paths, and the switch disables that
plugin. Finish then reads its guidance again after the probes. "The final session and
new launches" and "The switch inside finish" of the
[owner specification](specs/owner-skill-delivery.md#the-final-session-and-new-launches)
state that order.

The retained staging HOME holds no superseded plugin, and it must keep that
state. So the runner clones the staging HOME into `OUT/home` with `cp -c`.

Three Claude Code files of the staging HOME name its absolute path:
`settings.json`, `installed_plugins.json` and `known_marketplaces.json`. The
runner rewrites that path in the clone. It refuses the clone when another
Claude Code file still names the staging HOME, outside session records and
plugin bytes. The clone drops `.cache/ravel`, so Ravel takes new snapshots
there. Claude Code's own `plugin marketplace add Linkuistics/grove` and
`plugin install grove@linkuistics` then install the plugin in the clone. The
owner's registry names that GitHub marketplace.

The runner plans the nested probe before the launch. The probe is a
`ravel start-session` of the same 2.0 build, with its own route and work
directory. Its allow-listed environment holds no `RAVEL_*` name and no
loop-control name, as step 7 of the switch requires of a probe. The prompt
gives the agent the command `staging_trial.py probe --plan FILE`, which runs
exactly that plan.

The prompt asks the agent for six steps:

1. Load `grove:grove` through the Skill tool, and note the absolute path of
   its `SKILL.md`.
2. Run `claude plugin disable grove@linkuistics`.
3. Load `grove:configure-grove` through the Skill tool.
4. Run the probe command.
5. Read the `SKILL.md` of step 1 again by its absolute path.
6. Run the end command.

The verdicts require these facts:

- The session's native listing holds skills of `grove@linkuistics`, so the
  plugin's guidance was loaded. The listing holds no other superseded name.
- The load, the disable, the probe and the reread occur in that order. The
  reread's result holds the skill's `name:` line and its first heading.
- The probe recorded one launch and ended with Ravel exit 0 and the result
  `probe-ok`. Its native listing holds no superseded name and no skill of the
  disabled plugin.
- After the session, the installed version directory is unchanged without
  `.in_use`, holds no `.orphaned_at`, is still registered and is disabled.
- The plugin subjects of the retained staging HOME and the real-HOME
  subjects are unchanged.

The record keeps the result of the step 3 load without a judgement. That
result measures whether a running session keeps the plugin set that it
loaded.

| Harness | Model and effort | Record | Result |
| --- | --- | --- | --- |
| Claude Code 2.1.319 | `claude-sonnet-5-5`, low | `trials/guidance-claude-1` | Passed. The mandatory read was event 2. The Skill tool loaded `grove:grove` at event 6, from `cache/linkuistics/grove/238c3a839ab4/skills/grove`. The disable at event 8 printed "Successfully disabled plugin: grove (scope: user)". The Skill tool loaded `grove:configure-grove` at event 9 without an error. The probe ran at event 11 and printed `probe ended: Ravel exit 0, result 'probe-ok'`. The Read tool returned the whole `SKILL.md` at event 12. The session ended with `staging-ok`. |

The trial ran from 07:05:53Z to 07:06:42Z. The marketplace clone was at
`238c3a839ab4b30b101fbe3cc5212d66091df28a`, the version that the owner has
installed. The installed `skills/grove/SKILL.md` had the SHA-256
`5fd30f0d…` before and after the session. The version directory held 89
entries without `.in_use`, and its digest did not change.

These observations answer two claims of
[Resource lifetime](specs/owner-skill-delivery.md#resource-lifetime) for
Claude Code 2.1.319:

- **A disable writes no orphan marker.** After the disable and a new Claude
  Code launch in the same HOME, the version directory held no
  `.orphaned_at`. It was still registered in `installed_plugins.json`, and
  `enabledPlugins` held `false` for it.
- **A running session keeps the plugin set that it loaded.** After the
  disable, the Skill tool of the running session still loaded
  `grove:configure-grove`, which it had not loaded before. The skill's body
  came from the same version directory. No change to the skill listing
  reached the session.

The probe ran from 07:06:17Z to 07:06:33Z. Its native listing held 40 names,
the same names as the native listing of `trials/discovery-claude-1`. No name
began with `grove:`. The trial session's own listing held those 40 names and
the 25 skills of `grove@linkuistics`. Ravel's operation reports of the probe held
`stop attempt 3` and `stop attempt 4` with "Operation not permitted", as each
earlier staging launch did.

The marketplace add cloned `git@github.com:Linkuistics/grove.git` through
SSH. The clone has no `.ssh`. OpenSSH finds `~/.ssh` through the account
record, which names `/Users/antony`, and not through HOME. So this command
read the owner's real SSH configuration. It changed no real-HOME subject.

A staging HOME therefore does not isolate SSH. The isolation trial did not
cover it, because no earlier staging command used SSH.

### Limits

- The real HOME's `.claude.json` has no digest comparison, for the reason
  above.
- Each case has one passing record on each harness that the final runner
  judges. A single run shows that the behaviour occurs. It does not measure
  how often it occurs.
- A content read proves that the agent received the skill's body. It does
  not identify the path that the script opened. Only the design source
  holds `cli-tool-design`, so the source is not in doubt here.
- The discovery cases ran with the runner before the two repairs. Their
  verdicts come from `rejudge` with the final runner, on unchanged launch
  records. The source trials ran once, with the runner before the repairs,
  which touch no part of them.
- The kind case runs through `ravel start-session` with a prompt shaped like
  Grove's launch prompt. It does not run Grove's driver, its launch prompt
  or its session admission.
- The order point of the kind case sees a touch of the work tree in three
  forms. They are an edit, a path that exists at extraction and a mention of
  `.grove`.
  It cannot see a read of the task file inside a script that names neither. In
  each passing record, the touch that the verdict found is a plain read of
  the task file.
- The marker rule of the real plugin cache trusts that a running process is
  not a trial process. Ravel reaps the trial's group before the check.
- The Claude Code compaction case passes with `claude-opus-5-5` at high
  effort, in two of two runs. With `claude-sonnet-5-5` at low effort, it
  failed in five of five runs, and the cause is unknown. A Claude Code session
  of the owner that runs with less effort than its policy gives is not
  measured.
- Each compaction is the harness's default. Each context after compaction
  kept the descriptor path, so the case does not measure recovery after the
  loss of the preamble. That loss stays the gap of the common-loading record.
- The compaction case runs through the feeder in one managed process. The
  owner's interactive sessions compact automatically when the context fills,
  and no trial measures that trigger.
- In both stale-copy records the agent read Ravel's copy only. One run on
  each harness shows that this choice occurs. It does not show that an agent
  never reads the stale copy. The verdict finds a content read by lines that
  only one copy holds, so a script that prints only shared lines shows no
  copy.
- The stale-copy case needs the owner's real stale copy. After the switch
  deactivates it, the case refuses.
- The guidance probe case ran once, with `claude-sonnet-5-5` at low effort.
  Its steps are explicit, so it measures Claude Code's behaviour rather than
  the agent's choice of skill.
- The guidance probe case runs in a clone of the staging HOME, not in the
  retained HOME. The clone differs only in its own path in three files and
  in the absent Ravel cache, and Ravel resolved the same ten digests there.
- Finish runs in an interactive Claude Code session that Ravel 1.1.0 and
  `policy.ts` launch. The case runs `claude -p` through Ravel 2.0. The plugin
  cache, the registry and the Skill tool are the same in both modes, but the
  case does not measure the interactive mode.
- The agent disabled the plugin with `claude plugin disable`. The switch
  program writes `enabledPlugins` with an ordinary write instead. Both end
  with the value `false`. The case does not measure the switch's own write.
- The nested probe is one Claude Code launch. The switch's step 7 also runs
  a source probe and a Codex probe, which this case does not run.
- The marketplace add read the owner's real SSH configuration, as the
  section above states.

## Probe service lifecycle

`owner-switch-probes-k257` proved too large for one session and became a
node. Its first child, `probe-service-lifecycle-k280`, delivered this section
on 2026-11-09. The sections
[Probe process lifetime](specs/owner-skill-delivery.md#probe-process-lifetime)
and [Test seams](specs/owner-skill-delivery.md#test-seams) own the rules.
`probe-service-lifecycle-k282` reviews the result. The last child,
`session-probes-k281`, adds the four session probes.

### The probe program

[`scripts/owner_switch_probes.py`](../scripts/owner_switch_probes.py) is the
program that the plan runs as
`$python -I <state>/prepared/owner_switch_probes.py`. Its docstring states
each entry, the environment rule and the exit codes.

- `resolve --expect switched|disabled` runs
  `$(brew --prefix)/bin/ravel skills resolve --json` with HOME's
  configuration. `switched` requires the status `resolved`, exactly the
  plan's ten accepted sources with their accepted `resourceDigest` values,
  and the plan's mandatory file below its source's snapshot. `disabled`
  requires the status `disabled`.
- `launch` is the measured launcher, which the session probes use. The
  lifecycle measurement below runs it alone.
- `session` refuses with exit 2 in this version.

The probe finds `brew` on PATH, as the switch program does. A staging run
puts a stub `brew` first on PATH, whose prefix holds the accepted 2.0 build.
The interface of the plan draft therefore stays unchanged.

Each command runs in a new folder under `/private/tmp`. The probe refuses a
folder that overlaps HOME, or one with `.grove`, `.jj` or a project skill
root in it or in an ancestor. Each harness, Ravel and tool receives an
allow-listed environment and `DISABLE_AUTOUPDATER=1`.

### The launcher

The native route runs the headless form of each harness: `claude -p
--output-format stream-json` and `codex exec --json`. The ravel route runs
`ravel start-session --config` with a headless probe script, never the
owner's `config.rhai`. The owner's script launches the interactive Codex
TUI, and the TUI can launch Codex's app-server daemon by itself.

The script selects `/bin/sh -c`, which runs the harness and then
`ravel end-session harness-exit-<status>`. Ravel 2.0 fails a session whose
managed process exits without a result, with the code
`session_result_missing`. The first measurement met that failure on both
harnesses.

The launcher puts the leader in a new session and process group. It reads
the process table every 50 ms. It owns a process that did not exist before
the launch. It must see that process as the leader, as a child of an owned
process or as a member of the leader's group.

Three rules keep a reused PID out of the records:

- An owned process is a root only while its PID has its recorded creation
  time.
- The leader and its group count only while the leader is the launcher's
  unreaped child.
- A child counts only when the kernel created it after its parent and before
  the launcher read the table.

Each record names the PID, creation time, canonical executable, HOME,
`CODEX_HOME`, parent, group, argument vector, folder and relation. Each new
or changed record goes at once to the journal that
`OWNER_SWITCH_PROBE_JOURNAL` names. A record stays after its process
reparents to launchd. A Claude Code or Codex process of the same
configuration HOME before the launch refuses it without a signal.

After the leader exits, the launcher waits three seconds. It then ends each
owned process that still runs, and it reads the table during each grace
period. Cleanup repeats, up to three rounds, while a reading finds a new
survivor. A new process without a record is unattributed when it names one
of the four tools. It is also unattributed when it runs from a configuration
folder of HOME or works in the work folder. Such a process fails the probe,
and nothing signals it.

### The switch's cleanup

`owner_switch.end_owned` serves both the probe and the switch. It sends
SIGTERM and then SIGKILL, with grace periods of 20 and 10 seconds. Before
each signal it reads the PID's creation time and executable again. It
signals only an exact match.

- A record whose creation time differs counts as ended, because its PID now
  names another process.
- A record whose executable differs, or cannot be read, stays unconfirmed
  and receives no signal.
- A PID that exists but that this account cannot read stays unconfirmed
  too. A setuid program can cause that state.

Step 7 and restore run each probe through `run_probe`. It gives the probe a
new journal in `<state>/probes`, and it keeps the journal's records with the
result's records. It accepts only a record with a PID above 1, created
after the probe began. The probe's output goes to files, and the switch
waits for the probe process alone.

An interrupt of the switch sends the probe SIGTERM, so that the probe ends
its harness. The switch then ends the records of every probe that it started
and continues the interrupt. After an outer bound of 3,600 seconds, the probe
receives SIGTERM and counts as failed. The switch ends its records in rounds,
and records each child that a recorded process creates. The
[review integration](#probe-service-lifecycle-review-integration) states
these two rules.

### The in-session review

The leaf spent its one in-session reviewer on the signal code. The reviewer
received the code and the contract, and reported ten findings. Each finding
has one of three classifications:

| Finding | Classification | Change |
| --- | --- | --- |
| A reaped leader stayed a root, so a reused PID could gain a record | valid | The roots and group rules above |
| A PID reused between the table and the identity read gained a record | valid | The child must predate the table |
| A process could append a forged record to the journal | valid in part | A PID above 1, created after the probe began. A hostile agent is outside the quiet-interval model. |
| A child that a survivor created during its grace period escaped | valid | Readings during each grace period, and the rounds |
| An interrupt of the switch killed the probe before its cleanup | valid | `run_probe` sends SIGTERM and ends the journal records |
| A setuid exec read as an ended process | valid | An unreadable live PID stays unconfirmed |
| The exception path could leave the leader unreaped | valid | The path always reaps the leader |
| `PermissionError` stopped the signal loop | valid | Each failed signal skips its record |
| An escape faster than the reading, from `/`, stays unseen | trade-off | A record of new launchd children with HOME, as evidence only |
| The unattributed check excluded by PID only | valid | It excludes by PID and creation time |

One fixed control then exposed another defect. The switch waited for the
end of the probe's output pipes, and a leftover service that inherited a pipe
held it open. The control took 300 seconds, the life of its service.
`run_probe` now uses files. The fixes were substantive, so
`probe-service-lifecycle-k282` reviews them.

### Lifecycle measurement

`task owner:probes:lifecycle` ran 20 launches one at a time, from 09:26:31Z
to 09:39:13Z, on the final program. Claude Code was 2.1.319 and Codex was
0.167.0. The models were `claude-sonnet-5-5` and `gpt-6.1-sol`, both at low
effort. The three homes were:

- **bare**: a new HOME with only the sign-in links.
- **package**: bare with a clone of the owner's Codex daemon package, as
  the real HOME holds it after the owner stops the daemon. The clone has no
  pidfile, lock or socket.
- **staging**: the retained staging HOME, with the owner's MCP servers and
  residue plugins.

Two scenarios were a normal answer and a model that does not exist. The
other two ran `sleep 120`. One sent SIGINT to the leader's group, and one
sent SIGKILL to the leader alone.

| Launch | Leader | Owned | Survivors that cleanup ended |
| --- | --- | --- | --- |
| bare, Claude Code, normal | exit 0 | 1 | none |
| bare, Claude Code, failure | exit 1 | 1 | none |
| bare, Claude Code, interrupt | exit 0 | 3 | none |
| bare, Claude Code, crash | signal 9 | 3 | `zsh`, `sleep` |
| bare, Codex, normal | exit 0 | 1 | none |
| bare, Codex, failure | exit 1 | 1 | none |
| bare, Codex, interrupt | exit 1 | 3 | none |
| bare, Codex, crash | signal 9 | 3 | `sleep` |
| package, Codex, normal | exit 0 | 1 | none |
| package, Codex, interrupt | exit 1 | 3 | none |
| staging, Claude Code, normal | exit 0 | 8 | none |
| staging, Claude Code, interrupt | exit 0 | 10 | none |
| staging, Claude Code, crash | signal 9 | 10 | `zsh`, `sleep` |
| staging, Codex, normal | exit 0 | 6 | none |
| staging, Codex, interrupt | exit 1 | 8 | none |
| staging, Codex, crash | signal 9 | 8 | `sleep` |
| staging, Claude Code, ravel, normal | exit 0 | 10 | none |
| staging, Claude Code, ravel, interrupt | signal 2 | 16 | none |
| staging, Codex, ravel, normal | exit 0 | 8 | none |
| staging, Codex, ravel, interrupt | signal 2 | 10 | none |

Each launch passed, with no unattributed process, no process that remained
after cleanup and no new launchd child without a record. Both normal Ravel
launches printed `harness-exit-0`. The four results are:

- **No launch led to a Codex daemon.** That includes the package home. An
  independent `ps` listing, at once and 10 seconds after each launch, found
  no process that named the launch's HOME.
- **Normal exit, failure and interrupt leave nothing.** Each owned process
  ended with its leader. These included `codebase-memory-mcp`, the Quint and
  Playwright Node servers, the codebase-memory hooks, `node_repl` and
  `codex-code-mode-host`.
- **A crash leaves only the orphaned shell command.** The MCP servers and
  Codex's helper ended by themselves. The launcher recorded the orphans
  before the crash, kept their records after they reparented, and ended
  them with SIGTERM.
- **The measured mode prevents persistent services.** The headless command
  of each harness leaves none, and the launcher ends crash orphans by their
  records. The measurement establishes ownership and cleanup, so this leaf
  records no blocking finding.

The record is
`~/.local/state/owner-staging/trials/probe-lifecycle-3/record.json`, with
SHA-256 `425ca39f5e89d20fdc559dc9ea10a106691a96673dc52c0051a799e2f8f898b8`.
Its folder holds the digests of the three subjects before and after the run,
and they are equal. The probe program's digest is
`f3b4ed1e8f9527282566e864c204e0a96c3ce4bb60572a57585913761f49eab2`, which the
plan draft names. Two earlier runs, `probe-lifecycle-1` and
`probe-lifecycle-2`, measured earlier versions of the program. Their
outcomes agree with the table, apart from the Ravel failures of the first run.

### The resolve staging pass

`task owner:probes:resolve` ran the final probe with the plan draft and the
accepted 2.0 build.

| Case | HOME | Expected | Result |
| --- | --- | --- | --- |
| `switched` | staging | pass | exit 0 |
| `switched-quint` | staging settings without quint | fail | exit 1, "the accepted source quint is missing" |
| `switched-off` | the off value | fail | exit 1, "the status is 'disabled', not 'resolved'" |
| `disabled` | the off value | pass | exit 0 |
| `disabled-staging` | staging | fail | exit 1, "the status is 'resolved', not 'disabled'" |

The record is `~/.local/state/owner-staging/trials/probe-resolve-2/resolve.json`,
with SHA-256 `b9c5564ce6948d5ccd85cbd77ddaf0c89dd55ca179b18395e933c8c4c6d160b4`.

### Fixture controls

`task owner:probes:controls` runs 25 controls against fake harnesses and
processes. They cover a reparented service that the launcher records and
ends, and an unrecorded escape that nothing signals. They cover an
interrupt, a crash with a member that ignores SIGTERM, and a child that a
survivor creates during cleanup. They cover a signal to the probe, an
existing harness of the same HOME and a reaped leader's PID.

The controls also cover the allow-listed environment, the journal and the
work folder. They cover PID reuse, a changed, unknown or unreadable
identity, record validation and the journal merge. Two controls run
`run_probe` with its timeout and with an interrupt. Four controls cover
`resolve`.

The rehearsal's `probe_owned` flag now has five cases: `live`, `reparent`,
`moved`, `reused` and `journal`. The switch ends the first two and succeeds.
It signals nothing for `moved` and stops with 4, and it signals nothing for
`reused` and succeeds. For `journal` it ends the service from the journal
and restores with 5. `task owner:switch:rehearse` passed 157 of 157
controls on the final program.

Each of 13 mutations of the final program, in a scratch copy, turned its
control red:

| Mutation | Control that failed |
| --- | --- |
| `owned_state` ignores the creation time | `test_reused_pid_is_never_signalled` |
| `owned_state` ignores the executable | `test_changed_executable_is_never_signalled` |
| The check for an existing harness finds none | `test_existing_harness_of_the_home_refuses_without_a_signal` |
| The tracker records no child of an owned process | `test_reparented_service_is_recorded_and_ended` |
| A reaped leader stays a root | `test_a_reaped_leaders_pid_is_never_a_root` |
| Cleanup reads no table during a grace period | `test_child_created_during_cleanup_is_ended` |
| `resolve` skips the digest comparison | `test_switched_fails_for_a_wrong_layout` |
| The unattributed check finds none | `test_unrecorded_escape_is_never_signalled` |
| The harness inherits the whole environment | `test_harness_environment_is_allow_listed` |
| The switch accepts records from before the probe | `test_records_need_a_real_pid_created_after_the_probe_began` |
| An interrupt of the switch ends no journal record | `test_interrupt_ends_the_journal_records_and_continues` |
| The switch ignores the journal | `serial-probe-owned-journal` |
| The switch signals without the identity check | `serial-probe-owned-moved` and `serial-probe-owned-reused` |

### The regenerated draft

`task owner:switch-plan:draft` regenerated the plan draft. Its inputs now
name the probe program, and the program digest is new. One original changed
in the real HOME: `.claude/plugins/known_marketplaces.json` has another
identity than in the previous draft. Automatic marketplace updates can
change that file until owner-marketplace-freeze-k253 freezes them.
`task owner:switch-plan:check` passed, and the three stored outputs in
`docs/owner-cutover/` hold its new result.

### Measurement incidents

- The first measurement's Ravel launches failed with
  `session_result_missing`, as "The launcher" states.
- The digest step of the second measurement named its three subjects in one
  quoted word. `shasum` found no such file, so that step compared two empty
  listings. The third measurement passed each path separately.
- Once, the leader of a Codex launch gave no executable and no argument
  vector for its whole life. Four repeats, two of them first launches in a
  new HOME, read both. The cause is unknown. Such a record now has an
  unconfirmed identity, and nothing signals it.

### Limits

- Each scenario of the final program ran once. A single run shows that a
  behaviour occurs. It does not measure how often it occurs.
- No launch ran in the real HOME. The package home stands for the real
  HOME's stopped daemon, and the staging HOME for its MCP servers.
- The staging settings name the ten sources. The real switched settings omit
  `sources`, so the promoted Ravel resolves its default folder. The finish
  handoff repeats the resolution after the Ravel cut.
- The accepted 2.0 build reports `ravel 1.1.0` until the cut.
- An escape faster than the 50 ms reading, outside the leader's group, has
  no record. The probe fails when the escape names a tool, runs from a
  configuration folder or works in the work folder. Otherwise only the list
  of new launchd children with HOME shows it. No real launch produced one.
- macOS offers no process handle that pins a PID. A short interval remains
  between the identity check and the signal. A PID would have to be reused
  within that interval.
- No control reproduces a PID that the kernel reuses between the table and
  the identity read. The rule rests on the order of creation times.
- The session probes run `testanyware llm-instructions` and
  `testanyware doctor`, and no measurement here covers their services.
  session-probes-k281 runs them through the same launcher.

## Probe service lifecycle implementation review

`probe-service-lifecycle-k282` inspected producer commit
`7ea0da768914be8adc054896d3abfacef33f1502` against
[Probe process lifetime](specs/owner-skill-delivery.md#probe-process-lifetime).
The review found four actionable defects. These scenarios follow from the
committed source; this review did not execute them. It ran no test, build,
lint or format command and changed no production or test code.

### L1 — P1: records with an unreadable executable lose their cleanup obligation

Location: [`scripts/owner_switch.py:2858`](../scripts/owner_switch.py),
`owned_record`, as used by `read_journal` and `probe_owned`.

`owner_switch_probes.identity` can record `executable: null` when both
executable reads fail but the creation time remains readable. The tracker
retains that record, and `owned_state` correctly calls it `changed`, so
the launcher refuses to signal it and fails its cleanup. However,
`owned_record` accepts only a string executable. Both the journal reader
and the result merger therefore discard the unresolved record before
`end_probe_services` sees it.

Consider a detached service whose executable stays unreadable through the
launcher's last poll. The failed probe reports that live service, but the
switch can report no remaining owned process and enter automatic restore
at lines 3240–3242 and 3300–3303. If the executable becomes readable after
the probe exits, an ordinary helper also need not fail the later narrow
process inspection. The record must retain its unresolved cleanup
obligation even when it cannot authorise a signal. The existing unknown
executable control calls `end_owned` directly and bypasses both filters.

### L2 — P1: switch-side cleanup never records children created during shutdown

Location: [`scripts/owner_switch.py:2991`](../scripts/owner_switch.py),
`end_probe_services`.

The switch calls `end_owned(owned)` without a watcher, using a fixed list
of records. The tracker's shutdown watcher and repeated cleanup rounds
exist only in the probe process. They are unavailable when the switch
must clean up after that process has died or exhausted its stop grace.

For example, a killed probe leaves a journaled service. When the switch
sends that service SIGTERM, its shutdown handler starts `/bin/sleep` in a
new session, stays alive for half a second and then exits. The switch
never reads that parent link, even though it lasts much longer than the
recorded 50 ms sampling limit. It declares the recorded parent ended and
can proceed to restore with the child alive. The narrow process gate
does not recognise an ordinary `/bin/sleep` as one of the four tools.
The child-during-cleanup control exercises `Launch.cleanup`, not this
fallback path. Switch-side cleanup needs equivalent descendant evidence
or must retain an unresolved outcome when it cannot establish cleanup.

### L3 — P1: an interrupted later probe strands earlier probes' records

Location: [`scripts/owner_switch.py:1502`](../scripts/owner_switch.py),
`run_probes`, together with the current-journal cleanup at line 1470.

Cleanup of the whole probe sequence occurs only after its list
comprehension returns. Each `run_probe` handles an interrupt by ending
its own journal and re-raising. It cannot see earlier probes' evidence.

Suppose the first probe dies after journaling a detached service.
`run_probe` returns its failed exit and journal, and the sequence starts
the next probe. A KeyboardInterrupt during that next probe cleans only
the second journal. The comprehension never returns, so the caller never
reaches `end_probe_services(probe_owned(probes))`. The first service
survives the interrupted switch. A later command-start error similarly
bypasses cleanup of the completed prefix. The records remain on disk,
but no error path consumes that prefix. Preserve and discharge the
sequence's accumulated obligations on every exit. The interrupt control
contains only one probe and cannot expose this loss.

### L4 — P2: the launcher's exception path ends only the leader's group

Location: [`scripts/owner_switch_probes.py:716`](../scripts/owner_switch_probes.py),
`Launch.run`.

The exception handler ends the leader's group and reaps the leader, but
does not clean up the tracker's recorded processes outside that group.
If a process-table read fails during the settle interval after the
leader has been reaped, the handler does nothing to any process.

A detached service can already have a complete ownership record when
that read raises `Fail`. The service survives, and `launch_command` then
removes the work folder by default. For a standalone `launch`, that folder
also holds the only journal. The command reports failure while leaving
the recorded service alive and losing its record. A switch-provided
journal preserves evidence for the switch's fallback, but does not
repair the standalone launcher. The exception path needs to preserve
the original error while attempting owned-service cleanup and reporting
any unresolved identities.

### Review evidence and limits

The review read the launcher's tracking, identity, pre-existing-harness,
signal and exception paths; the switch's journal, identity, cleanup and
probe-sequence paths; and their fixture controls. It also read the
rehearsal changes and the producer's recorded lifecycle measurements.
The recorded 25 controls and 157 rehearsal cases are producer evidence,
not fresh results from this review.

Graph evidence used project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, full
generation `2026-11-09T09:42:35Z`. Exact-path coverage reported matching
metadata and no recorded gap for the two programs, their controls, the
rehearsal, the lifecycle measurement script and the Taskfile. Graph
searches for the bounded symbols and both-direction traces of
`run_probe` and `end_owned` exhausted their returned pages. Some outbound
edges resolve Python methods to unrelated Rust methods; the review used
the exact source for those calls. Coverage is a best-effort signal.

The identity-check-to-signal interval remains a platform limitation: the
code does not pin a process handle. The faster-than-observation escape
remains an explicit measurement limit, with no such escape reported in
the real launches. Neither is counted again as a defect here. L2 concerns
an entirely absent observer on the fallback path, and L1, L3 and L4 lose
already captured evidence; accepting the sampling limit does not settle
those findings. This review does not certify that the measured launches
bound every possible service lifecycle.

## Probe service lifecycle review integration

`probe-service-lifecycle-k283` integrated the four findings of
[the implementation review](#probe-service-lifecycle-implementation-review)
on 2026-11-09. Each finding reproduced from the source at the producer
commit. Each one is a real issue, and none needs a redesign. The table
gives each disposition and its change.

| Finding | Disposition | Change |
| --- | --- | --- |
| L1 | Real issue | `owned_record` accepts a null executable. `owned_state` calls the record `changed`, so it receives no signal and leaves recovery incomplete. |
| L2 | Real issue | The switch ends its records in rounds and records each child of a recorded process. A failed process listing during that cleanup leaves recovery incomplete. |
| L3 | Real issue | `run_probes` ends the records of every started probe on each exit path. A cleanup failure replaces an ordinary error. An interrupt continues after a logged failure. |
| L4 | Real issue | The launcher's exception path runs the cleanup and keeps the original error. An unresolved record appears in a `Fail`, and the work folder keeps the journal. |

### The changes

[`scripts/owner_switch.py`](../scripts/owner_switch.py) now holds three
shared parts. `descendants` is the walk from confirmed roots. A child counts
only when the kernel created it after its parent and before the table
reading. `end_in_rounds` ends each running record in up to three rounds. It
watches for new children before each round, during each grace period and
for one second after each round. A round that finds no running record ends
the loop.

The launcher's `Tracker.poll` and `Launch.cleanup` use these two parts. The
switch's `end_probe_services` uses them through `descendant_watch`. That
watcher treats each record with its recorded creation time as a root. It
appends a record for each new child. The child's record names its PID,
creation time, executable, HOME, argument vector, parent and relation.

`run_probe` now only stops its probe on an interrupt. `run_probes` holds
the obligations of the completed probes and the journal of the current
probe. It discharges them before any error continues.

[`scripts/owner_switch_probes.py`](../scripts/owner_switch_probes.py) adds
`Launch.failure_cleanup`. The exception path calls it after it ends the
leader's group. When the rounds cannot run, it ends the recorded processes
without a watch. `launch_command` keeps the work folder while a record stays
unresolved.

### Verification

`task owner:probes:controls` now runs 31 controls. Six are new or changed:

- L1: a journaled record with a null executable survives `read_journal`
  and `probe_owned`. `end_probe_services` then raises `Unresolved`, and the
  process stays alive.
- L2: a recorded service starts `/bin/sleep` in a new session on SIGTERM.
  It stays half a second and exits. The switch records and ends the child.
- L2: a process listing that fails during switch-side cleanup raises
  `Unresolved`, not `Stop`.
- L3: the first of two probes exits and leaves a journaled service. An
  interrupt during the second probe ends both services. The single-probe
  interrupt control now runs through `run_probes`.
- L4: the process table fails once after the launcher reaps the leader. The
  original error remains, the recorded service ends and the work folder goes.
- L4: the same failure with an unconfirmable record names that record in
  the error. The process receives no signal, and the work folder and its
  journal stay.

Before the fixes, each of the five new controls failed for its predicted
reason. The L1 control's journal reader returned no record. The L2 controls
left the child alive and raised no `Unresolved`. The L3 control left the
first service alive, and the L4 control left the recorded service alive.

This session wrote the sixth control after the fixes. It depends on
`keep_work`, which the producer did not have.

`task owner:switch:rehearse` passed 157 of 157 controls. The five
`serial-probe-owned-*` cases are among them. Its record names the program
with SHA-256
`b75a7bd4e72ae00a1ced7245ac01f474f89eb236fca14d7448fb3036421b85a0`, which is
the committed `scripts/owner_switch.py`. The probe program's SHA-256 is now
`6ffbbea92d4c0b9133dad4665c57db057344e5b65e07bf7d7621a32f8510b389`.

Pyflakes-class rules (`ruff --select F,E9`) pass on the three changed
scripts.

### Limits

- The L2 watcher sees a child only while its parent link lasts at least one
  reading. A child whose parent exits within one reading interval keeps the
  sampling limit of the launcher.
- The switch watcher adopts any child of a recorded process that the kernel
  created after its parent. This is the same rule as the launcher's, so it
  rests on the same order of creation times.
- When the process table cannot be read, the launcher's fallback sends
  signals without a watch. A child created during that shutdown has no
  record.
- The stored files `switch-plan-draft.json`, `switch-plan-check.json` and
  `switch-plan-report.txt` still name the producer's program digests. This session did not regenerate them, because
  the draft reads the real HOME. `session-probes-k281` changes the probe
  program again, and `owner-switch-plan-k254` writes the draft again.
- These fixes ran only against fixtures. No real harness session ran in this
  session, and the lifecycle measurements of
  [Probe service lifecycle](#probe-service-lifecycle) were not repeated.

## Session probes

`session-probes-k281` delivered this section on 2026-11-09. It adds the
probe program's `session` mode on the measured launcher of
[Probe service lifecycle](#probe-service-lifecycle). The sections
[The switch inside finish](specs/owner-skill-delivery.md#the-switch-inside-finish),
[Rollback](specs/owner-skill-delivery.md#rollback) and
[Concurrent writers](specs/owner-skill-delivery.md#concurrent-writers) own
the observations.

### The session mode

[`scripts/owner_switch_probes.py`](../scripts/owner_switch_probes.py) states
each observation in its docstring. The plan draft's probe commands stay
unchanged. The program now imports `harness_trial.py` from its own folder,
as it imports `owner_switch.py`, so `<state>/prepared/` holds all three.

- The **ravel** session asks for a small Python command-line tool for other
  agents, one quoted line of `grove-planning`, and the TestAnyware skill with
  its two commands. It requires the mandatory read first and an unnamed
  craft skill before the first edit. It also requires `grove-planning` from
  the `grove-core` snapshot with a quoted line of that copy, and the
  TestAnyware read with both commands. On Claude Code it requires the
  technical-communication SessionStart hook. The native listing must hold
  no superseded name, and the request view must give each accepted digest.
- The **native** session asks only for one quoted line of `grove-planning`.
  The native listing must hold it, and the agent must read it natively and
  quote a line of a native copy.
- `grove-planning` is the kind skill because `grove-core` holds only six
  skills, and only its three planning kinds are not `finish`. The quoted
  line differs between the snapshot copy and the native copy.
- The craft verdict accepts only `cli-tool-design` or
  `coding-style-python`. A first debug run credited `technical-communication`,
  which the hooks' instruction names. That read shows no discovery.
- Before the launch and after its cleanup, the probe reads each ordinary
  child of the three native skill roots. It skips only
  `~/.claude/skills/synced` and `~/.codex/skills/.system`, and only while
  each is a real folder. Any other change fails the probe.
- The result's `writes` lists each entry that the session added, changed or
  removed at the top level of HOME and in the harness and tool folders.
  `.config`, `.cache` and `.local` hold over a million files in the real
  HOME, so the probe walks only their named subfolders.

The ravel route's `sh -c` script writes Ravel's request view and the
harness's event stream to the work folder. Ravel prints only the session
result on standard output, as the lifecycle record shows.

### The staging pass

`task owner:probes:sessions` ran eight cases one at a time, from 10:31:27Z
to 10:37:50Z. Each case ran in its own `cp -c` clone of the staging HOME,
with the plan draft and the accepted 2.0 build. Claude Code was 2.1.319 with
`claude-opus-5-5`, and Codex was 0.167.0 with `gpt-6.1-sol`, both at medium
effort.

The layouts are:

- **switched**: the staging layout. On Claude Code, the runner installs
  `technical-communication-hooks@writing-hooks` from the candidate keg with
  Claude Code's own commands, because owner-staging-hooks-k251 installs it
  in the retained HOME only later.
- **stale**: switched, with a copy of the real HOME's `grove-planning` in
  both native skill roots.
- **native**: the staging layout with copies of the 58 real `.codex/skills`
  entries that the plan names absent, in both native skill roots.

| Case | Layout | Wanted | Result | Failed observations |
| --- | --- | --- | --- | --- |
| `claude-ravel` | switched | pass | exit 0 | none |
| `claude-ravel-stale` | stale | fail | exit 1 | `listing`: `grove-planning` listed |
| `claude-native` | native | pass | exit 0 | none |
| `claude-native-bare` | switched | fail | exit 1 | `listing`, `kindSkill`: no native copy |
| `codex-ravel` | switched | pass | exit 0 | none |
| `codex-ravel-stale` | stale | fail | exit 1 | `listing`: `grove-planning` listed |
| `codex-native` | native | pass | exit 0 | none |
| `codex-native-bare` | switched | fail | exit 1 | `listing`, `kindSkill`: no native copy |

In the two passing ravel cases, each agent read the mandatory file first. It
read `cli-tool-design` before its first edit: Claude Code by path, and Codex
through a Python script whose result held the skill's marks. Each agent
quoted the snapshot copy's line. In the two native cases, Claude Code loaded
`grove-planning` through its Skill tool, and Codex read the native copy with
a shell command. Each quoted the native copy's line.

No launch left an owned process after its leader, so cleanup signalled
nothing. No launch had an unattributed process or a new launchd child
without a record. The owned processes include the `tc-hook` shells and
`testanyware doctor`.

The record is `~/.local/state/owner-staging/trials/probe-sessions-1/sessions.json`,
with SHA-256 `ebfcf130cf13e9a204f6a43c8e760d5baed02868132cd0c0751961d7cca9df4b`.
It names the measured probe program, SHA-256
`4e2aeb0d76a3538f323469c8981a90fb84e150d084b222502fb8385266fff397`. The
program, `harness_trial.py`, `owner_switch.py` and the plan draft had the
same digests before and after the run. The final program differs from the
measured one in its docstring, and in one more folder of the write record,
`~/.local/state/technical-communication`. Reversing the docstring edits
reproduces the measured digest.

### Files that a probe session writes

No session changed a skill copy or an agent definition outside the excluded
`~/.claude/skills/synced` subtree. No MCP server rewrote one. Each Claude
Code session rewrote `.last-complete-round` and `manifest.json` in that
subtree, which claude.ai's skill sync owns.

Each Claude Code session wrote these entries:

- `.claude.json`, and a new `.claude/backups/.claude.json.backup.<ms>`.
  An older backup went.
- The session file and a `memory` folder in `.claude/projects/<work>`.
- `.claude/session-env/<id>` and `.claude/cache/model-catalog`.
- `.claude/plugins/store/cc-plugin-agents-md_builtin-*.json`.
- Two files of `.claude/plugins/synced/<id>`, and the two files of
  `.claude/skills/synced/<id>` above.
- The lock files of one `.npm/_npx` package.

With the hooks installed, a Claude Code session also wrote these entries:

- `.claude/plugins/data/technical-communication-hooks-writing-hooks`.
- The hooks' reply log,
  `~/.local/state/technical-communication/sessions/claude-code--<id>.jsonl`.

Each Codex session wrote these entries:

- The session file in `.codex/sessions/<date>`.
- The SQLite files `state_5`, `logs_2`, `memories_1`, `queue_1`, `goals_1`
  and `thread_history_1` of `.codex`, with their `-shm` and `-wal` files.
- `.codex/cache/codex_apps_*`, `.codex/models_cache.json`,
  `.codex/plugins/cache` and `.codex/tmp/arg0-macos`.
- The lock files of one `.npm/_npx` package.

A ravel-route session also wrote a new descriptor and a `staging` folder in
`.cache/ravel/skills/v1`. In a HOME without snapshots, Ravel writes them too.

These are findings for the plan's expected inventories. The probe judges
none of them, apart from the native skill roots.

### The bytecode write of the hooks

The hooks plugin runs in place from the `writing` source of the keg. Its
`tc_hook.py` loads `ste-lint.py` through `importlib`, and `tc-hook` runs
Python with `-I`, which does not stop bytecode writes. Python therefore wrote
`scripts/__pycache__/ste-lint.cpython-315.pyc` into the candidate keg's
`writing` source. The source then gave `resourceDigest` `6be6f84a…`, not the
accepted `2dea55cd…`, and the next debug case's `digests` verdict failed.

In the staging pass, each of the three later Claude Code sessions with the
hooks installed wrote the file again, including `claude-native-bare`, which
made no edit. The runner removed it before each case and recorded each
removal. In the real switch, the Claude Code probe of step 7 makes the same
write. The Codex probe and `mac-check` would then report a source digest
that is not the accepted digest. tc-hook-bytecode-k284 takes the fix, ahead
of the hooks staging leaf.
[The hook bytecode fix](#the-hook-bytecode-fix) records that fix.

### Fixture controls

`task owner:probes:controls` now runs 42 controls. Eleven are new:

- A fake Claude Code prints a scripted event stream. The native session
  passes with a native copy. It fails without one, and it fails for a quoted
  line of the snapshot copy.
- The root comparison ignores changes inside `.claude/skills/synced` and
  `.codex/skills/.system`. It fails for a change to an ordinary child, a
  nested `synced` folder, an `.agents/skills` entry and a `.systemic`
  sibling. A write below `.claude/agents` appears in the record.
- Scripted ravel events pass each observation. For each observation, one
  wrong layout fails it and no other. A read of `technical-communication`
  does not satisfy the craft verdict, and a `grove-planning` read from
  another source does not satisfy the kind verdict. An empty listing fails.
- The ravel route's shell writes the view and the stream and ends the session
  with the harness's status.

Each of 14 mutations of the new code, in a scratch copy, turned its control
red. They removed the root comparison, skipped by name alone, and accepted a
quote of any copy. Others dropped the mandatory order, the craft skill
list, the edit order, the kind skill's source or the doctor command. The
rest dropped the hook mark, the superseded check, the empty-listing check,
the digest comparison, the native listing check or the view file. The kind-source mutation first
stayed green, and the source control above closes that gap.

`staging_trial.clone_home` meant to drop the clone's Ravel cache, but its
removal failed in silence on Ravel's read-only snapshots. Each clone of this
pass therefore kept the staging HOME's snapshots. The helper now makes the
cache writable and refuses when the removal fails. Its test holds a read-only
snapshot, and the old removal fails that test.

`bash scripts/check.sh` passed all 18 checks on the final tree. Its first run
failed on four links of the review section above, whose `#L` line fragments
name Python files. Those links now name the files alone, and their text keeps
the line numbers. Pyflakes-class rules (`ruff --select F,E9`) pass on the five
changed scripts.

### Limits

- Each case ran once. A single run shows that a behaviour occurs. It does not
  measure how often an agent skips a skill.
- No probe session ran in the real HOME. The native layout stands for the
  real native delivery with copies in the user roots. The real Claude Code
  native route reads the Grove skills from the `grove@linkuistics` plugin
  instead. The probe finds a plugin-cache copy too.
- The clones kept the staging HOME's Ravel snapshots, as the previous
  section states. Ravel's digests matched the accepted digests in each
  passing ravel case. In `codex-native-bare`, the agent found a snapshot copy
  of `grove-planning` and quoted it, and the case failed as wanted.
- The write record walks named folders. A session write elsewhere in
  `.config`, `.cache` or `.local` stays unrecorded, unless it adds a new
  top-level entry of HOME.
- The hooks verdict proves the SessionStart hook. It does not prove the
  prompt hook or the document gate, although the bytecode write shows the
  gate ran.
- A probe fails for a new Claude Code or Codex process that it did not
  start. A harness that another session starts during a probe therefore
  fails it.
- The stored plan draft still names the producer's program digest.
  owner-switch-plan-k254 writes the draft again.

## The hook bytecode fix

`tc-hook-bytecode-k284` delivered this section on 2026-11-10. It removes the
write that [The bytecode write of the hooks](#the-bytecode-write-of-the-hooks)
found. Step 2 of
[The switch inside finish](specs/owner-skill-delivery.md#the-switch-inside-finish)
states the revision that a changed released source needs.

### The defect and the fix

Every hook event wrote the bytecode, not only the document gate. The core's
`main` loads the checker before it reads the off file, and the import of
`ste-lint.py` wrote `scripts/__pycache__/ste-lint.cpython-315.pyc`. A scratch
copy showed the write for the `prompt` event.

WritingSkills `578f84d1f5a9bb04573a704bca639530d93b6bca` makes two changes:

- `tc-hook` runs Python with `-I -B`. `-I` ignores
  `PYTHONDONTWRITEBYTECODE`, so the flag stops the write itself.
- `tc_hook.py` sets `sys.dont_write_bytecode` before it loads the checker.
  This covers a run without the guard.

The core writes only its state directory. The section "Harness mapping" of
the WritingSkills specification `docs/specs/technical-communication.md` now
states that no entry writes inside the source.

### The control

The WritingSkills script `scripts/hook-writes.py` runs as
`task check:hook-writes`, inside `task check`. It copies the `writing`
source twice and makes one copy read-only. It runs each command that the
Claude Code and Codex hooks files declare, and each core event of the Pi
extension. That is 17 commands. It also loads the Pi extension in a real Pi.

Each command must do its event's work. The gate must report a finding, and
the reply log must hold a start record and a reply record. The writable
copy's tree digest must stay the same. The read-only copy must give the same
output.

| Subject | Result |
| --- | --- |
| The source before the fix | Failed: the writable copy gained `scripts/__pycache__/ste-lint.cpython-315.pyc` |
| The source after the fix | Passed, with the Pi load |
| Mutant without the fix | Failed, as required |
| Mutant whose core appends `scripts/hook.log` | Failed, as required |

The first version of the control found only three of Pi's five events,
because two calls put the event name on the next line. The pattern now allows
white space there, and the control runs all five.

### The candidate keg

Grove `ef8a79dd7621a437177982e24d8ec27831b4add4` pins the fixed commit in
`plugins/packs.json`. The human allowed the pushes. jj pushed WritingSkills
`main` and Grove's `gh-issue-25` bookmark. `git ls-remote` then named
`578f84d1f5a9` as WritingSkills `main`.

The earlier candidate keg moved to
`/Users/antony/.local/state/owner-staging/candidate-2dea55cd`.
`task owner:candidate-keg` then built the new one at the retained path.
The build fetched WritingSkills from GitHub, because this session removed
the local pack checkout first. The script reported that `origin/gh-issue-25`
holds the commit.

| Subject | Earlier candidate | This candidate |
| --- | --- | --- |
| Grove commit | `7b113a28a96621217c16154d4f44eba667236132` | `ef8a79dd7621a437177982e24d8ec27831b4add4` |
| Tree ID of `plugins/grove` | `885051a94eddaa81fa7a1a9900c3ac4db775aa84` | The same |
| Blob ID of `plugins/packs.json` | `e00c67b74939542db0b7af2428d918a3458d73a4` | `2cebaf5f062d08b50b22d058bd8c4db3d412f8a6` |
| WritingSkills commit | `b9240e0138df7d9f1d6926b77737668463dcfdfb` | `578f84d1f5a9bb04573a704bca639530d93b6bca` |
| Archive SHA-256 | `3d31f84441701bf5f4013b7ccacbb4945eb39e875fd7ec2deb8f779321048241` | `fcb7603d70ccadcb3fcb071c5c2f79da165c807656f4723620142acab577546b` |
| `writing` `resourceDigest` | `2dea55cd72d0d56032c04419bfd438b62b321563573a09fffcab2f2c692ecade` | `56bcd07b87f46a7b9ff92eed5a5a128f10d3ce64be2f7d934be898e058782212` |

The other five Grove sources gave their accepted digests. The keg holds no
`__pycache__`. Before the pushes, a provisional keg of the same commit in a
scratch folder gave the same six digests. Each Claude Code hook event ran in
place from that keg, and Ravel then gave the same `writing` digest.

The table "The ten Ravel sources" of
[the moves record](repository-moves-record.md#the-ten-ravel-sources) holds
the new `writing` row and names the earlier values. The plan draft's
`accepted.sources` takes its digests from that table. The draft received the
new `writing` digest, and its sources equal the table again.

### The affected staging trials

Only the source trials and the session probes compare the `writing` digest
or run the hooks. The discovery trials read skill text, which the fix
leaves unchanged. The staging HOME registers no hooks plugin. The dry run of
`owner-switch-rehearsal-k255` has not run yet, so no dry run needs repeating.

- `task owner:staging:sources` passed from 06:04:25Z to 06:04:29Z. The
  record is `~/.local/state/owner-staging/trials/sources-2/record.json`,
  SHA-256 `bd89a3af1a2199560ce338ec0b6ce71378222684eaeb0cef17f0de1e14ba91e1`.
  It holds the new `writing` digest and not the earlier one.
- `task owner:probes:sessions` ran its eight cases from 06:04:48Z to
  06:12:22Z, and each gave its wanted result. The table of
  [The staging pass](#the-staging-pass) holds the same results and failed
  observations. The record is
  `~/.local/state/owner-staging/trials/probe-sessions-2/sessions.json`,
  SHA-256 `fb82f59b674313db7c5ef2b3b626d2f9e0b4fdb4616317197eaeea8de84f30bb`.
  It names the probe program, SHA-256
  `176d89c865de1b9dcc22580e52bdeef0cb08aa8bf1026730144b32bc21933488`. The
  program, `owner_switch_probes.py`, `harness_trial.py`, `owner_switch.py`
  and the plan draft had the same digests before and after the run.

The runner still removes each `__pycache__` from the candidate keg before
each case and records each removal. Each of the eight cases recorded none.
Three Claude Code cases ran with the hooks installed. A later case checked
the keg after each of them, and the keg held no `__pycache__` after the last
case. The ravel cases saw only the new `writing` digest. A removal in a
later run now shows a regression of the fix.

### Limits

- `task owner:switch-plan:check` refused the revised draft, because
  `.claude/plugins/known_marketplaces.json` in the real HOME changed after
  the draft. Automatic marketplace updates change that file until
  owner-marketplace-freeze-k253 freezes them. The stored
  `switch-plan-check.json` and `switch-plan-report.txt` therefore still
  name the earlier draft's digest. `owner-switch-plan-k254` writes the draft
  again from the real HOME.
- Each probe case ran once.
- The control runs each command with a synthetic payload. A real session
  of each harness ran only in the probes, which cover Claude Code's hooks.
  No real Codex session ran with the Codex hooks plugin, which the switch
  keeps absent.

## Quiet interval in staging

`owner-staging-hooks-k251` delivered this section on 2026-11-10. It measures
the complete simplified switch in the staging HOME beside an idle, signed-in
Claude Code parent with the owner's MCP servers. The sections
[Claude Code's own writes](specs/owner-skill-delivery.md#claude-codes-own-writes),
[The hooks installation inside finish](specs/owner-skill-delivery.md#the-hooks-installation-inside-finish)
and [Test seams](specs/owner-skill-delivery.md#test-seams) own the rules.
The tool is [`quiet_interval.py`](evaluations/owner-skill-delivery/quiet_interval.py),
and `task owner:quiet-interval` runs each of its steps.

### Result

The quiet-interval assumption held in this run. From 07:05:29Z to 07:08:34Z,
the switch installed the hooks plugin, deactivated 154 entries, wrote the
settings and ran three probes. It then ran `mac-check`, wrote its receipt and
exited with 0. The idle parent and its MCP servers ran through that interval.
They also ran for 136 seconds before it and for 36 seconds after it.

Every write of `settings.json`, of both registries and of the Ravel settings
came from the switch. No loaded plugin version directory and no registration
of a loaded plugin changed. No orphan marker appeared.

Claude Code was 2.1.322, at
`/Users/antony/.local/share/claude/versions/2.1.322`. No hold exists before
owner-marketplace-freeze-k253, and the plan draft still names 2.1.319. The
held version of k253 therefore needs another run of this measurement unless
it is 2.1.322.

### The arrangement

The parent is the switch's ancestor, as finish is. The tool started the held
Claude Code in its own tmux server, with HOME set to the staging HOME. The
parent ran in a new folder under `/private/tmp` with an allow-listed
environment. The
parent showed its prompt, and the tool waited 120 seconds. The tool then
typed one prompt, and the parent ran `run-switch.sh` with its Bash tool and
waited for it. Each settings temporary named its writer's PID, and its
ancestry was the same for both Claude Code commands:

```
claude:6759 < python3:5403 (the program) < sh:5400 (run-switch.sh) < zsh:5399 < 2.1.322:85959 (the parent) < tmux
```

The parent's `/mcp` view showed `codebase-memory-mcp` and `quint-kb` with
their tools, and `plugin:playwright:playwright` with 25 tools.
`plugin:context7:context7` needed authentication, because the staging HOME
copies no credential. The parent's process tree held the three local servers
before and after the switch.

These parts stand in for the real HOME:

| Part | Content |
| --- | --- |
| Staging prefix | `quiet-interval-1/prefix` stands for `$(brew --prefix)`. Its `bin/ravel` is a clone of the accepted 2.0 build, SHA-256 `3a9a9129…`. Its installation record names its `share/ravel/skills`, which links the ten sources. `opt/grove` reaches the candidate keg. With no `sources`, the build resolved the ten accepted digests. |
| Stubs | `brew` answered `--prefix`, `--cellar` and `info --json=v2 linkuistics/taps/grove`. `mise` answered gate 5's status query. Each other call exits 97, and none occurred. Resolution, the probes and Claude Code's commands were real. |
| Native starting state | Copies of the plan's 154 entries, following links, so that no link leads out. A dangling link became a link to an absent path inside the staging HOME. Claude Code's commands installed and enabled `grove`, `linkuistics` and `testanyware` at `238c3a839ab4`, `superpowers` 7.0.0 and `quint-llm-kit` 1.0.0, the owner's versions. |
| Tools | Clones of `~/.local/bin/codebase-memory-mcp` and `~/.quint-llm-kit`, which the baseline instrument reads relative to HOME. |
| `codebase-memory-mcp` | The server is one daemon per account, and it refuses a client with another cache folder. The staging entry sets `CBM_CACHE_DIR` to the daemon's `~/.cache/codebase-memory-mcp`. Finish's own parent shares that daemon. |
| Staging plan | `switch_plan_draft.py draft --home --out` drafted it from the staging HOME after native-off. Plan SHA-256 `35243f40617fd3eb086242772fbbd5b3c09710588096ad29724a982934f4913e`. |
| Process inspection | `OWNER_SWITCH_REHEARSAL_SCOPE` named an empty folder, because the inspection cannot admit the parent beside the operator's own session. The probes' own checks ran unchanged. |
| Hook points | `OWNER_SWITCH_REHEARSAL_HOOK` named a script that only records each point and its time. These marks define the program's windows. |
| Readers and guidance | `OWNER_SWITCH_READERS` named the 11 version directories that held the parent's `.in_use/85959`. `--guidance` named the loaded `grove-finish` skill file. |

The program was `scripts/owner_switch.py`, SHA-256
`c7fabacda7aa64a5389468005f92d81bb67c781f8887878866273cde1432c2c2`. The
probe program was SHA-256
`176d89c865de1b9dcc22580e52bdeef0cb08aa8bf1026730144b32bc21933488`.

### The writes

The recorder watched `.claude`, `.claude/plugins` and `.config/ravel`, and
the four files, from 07:03:13Z until the parent ended at 07:09:10Z.

| Window | File | Writer |
| --- | --- | --- |
| `marketplace add` | `known_marketplaces.json`, replaced through `known_marketplaces.json.tmp.cad28f73` | The command's window. Registry temporaries name no PID. |
| `marketplace add` | `settings.json`, replaced through `settings.json.tmp.6759.…` | The command, PID 6759, below the switch |
| `install` | `settings.json`, replaced through `settings.json.tmp.7034.…` | The command, PID 7034, below the switch |
| `install` | `installed_plugins.json`, replaced | The command's window. The recorder saw no temporary. |
| Step 5, the disabled value | `settings.json`, replaced | The program, `.settings.json.owner-switch-…` |
| Step 6 | `settings.json`, replaced | The program |
| Step 6, last | `.config/ravel/settings.json`, replaced | The program |

No write occurred outside these windows. In particular, no file changed
before the switch, during the three probes or after the switch. Neither
`plugin:context7` nor any server wrote a watched file.

The recorder also listed entries that are no writes of a watched file. The
parent and the Claude Code probe created and removed
`known_marketplaces.json.lock` at their start without changing the file. The
parent wrote `history.jsonl` when it received the prompt and when it ended.

### Metadata

Before and after the switch, and at each replacement, each of the four files
had mode 0644, no ACL and only `com.apple.provenance`. No file lost
metadata, because none held any to lose.

A scratch HOME measured what the commands would lose.
`claude plugin marketplace add` and `claude plugin install` dropped a user
extended attribute from `settings.json` and from both registries. The
program refused lost metadata only for `settings.json`. `lost_metadata` now
checks all three files of `HOOK_FILES`, and the specification's section
[Claude Code's own writes](specs/owner-skill-delivery.md#claude-codes-own-writes)
names both registries. The controls `serial-hooks-registry-acl-appears` and
`serial-hooks-registry-attribute-appears` cover them. A mutant that checks
only `settings.json` fails both.

The control `serial-backup-metadata` put an
ACL on `installed_plugins.json` before a switch that installs the hooks
plugin. The program now refuses that layout, so the control marks only the
entry folder, its file and the Ravel settings.

`task owner:switch:rehearse` then passed all 159 controls on Python 3.15.1
and macOS 27.0.1, in 490 seconds. The program, the suite, the dry-run
instrument, `mac-check` and its helper had the same SHA-256 digests before
and after the run. `bash scripts/check.sh` passed all 18 checks.

### Loaded plugins and markers

The parent loaded 11 version directories. These are the six residue plugins
of the official marketplace, the five superseded plugins and none other.
Each kept its change identity without the top-level `.in_use`, and each kept
its registration in `installed_plugins.json`. The comparison skips only the
top-level `.in_use` of each directory, as `inventory.py` and the purge
report do. The parent's marker was present before and after the switch.

Two `.orphaned_at` markers existed before the switch, in
`claude-plugins-official/context7/dceb15d50832` and
`claude-plugins-official/playwright/dceb15d50832`. A seeding command created
both directories at 06:40:03Z. Neither directory is registered, and the
parent loaded neither. The Claude Code probe rewrote both markers 0.8
seconds after its launch. A fresh Claude Code session marks each
unregistered version directory again, so the real switch's probe does the
same. No marker appeared in a registered or loaded directory.

### After the measurement

The staging HOME holds `technical-communication-hooks@writing-hooks`
installed, at `cache/writing-hooks/technical-communication-hooks/unknown`,
and enabled. `guardrail@personal` is enabled. The five superseded plugins
stay installed and disabled, as in the real HOME after a switch. The
receipt is
`.local/state/owner-switch/receipts/20261110T070313166030Z-5cff82.json`.

The switch left the switched Ravel settings, which name no `sources`. The
staging tools run the accepted build, which has no installation record, so
the original staging settings went back after the measurement, SHA-256
`d8a49f06…`. They resolve the ten sources again. The `writing-hooks`
marketplace names `quiet-interval-1/prefix/opt/grove` as its directory, so
that folder stays as long as the staging HOME.

The interactive parent created `Applications/Claude Code URL Handler.app` in
the staging HOME. Its executable is a link to the Claude Code executable. It
joins no configuration owner, but it is the one new link that leads out of
the staging HOME.

### Runs that ended early

Four runs ended before a switch write. Their files are in
`quiet-interval-1/aborted-1` to `aborted-4`.

1. Claude Code 2.1.322 shows the folder trust prompt with "No, exit"
   selected, and the tool's Enter chose it. The tool now records the trust
   in the staging `.claude.json` before the launch.
2. The staging settings select the vim editor mode, which shows no
   "? for shortcuts". The readiness check now accepts its footer.
3. The switch refused with 3. The plan's original of the Ravel settings was
   absent, but native-off wrote the off value before the switch.
4. The switch refused with 3, because the plan named `.agents/.grove` with
   the identity "absent". The draft now leaves out a path that the other
   HOME lacks, and `prepare` fails on a difference that `report` shows.

The third refusal applies to the real plan too. The stored draft records the
Ravel settings as absent, and finish runs native-off before the switch.
owner-switch-plan-k254 received the finding: the real plan needs the
identity of the file that native-off writes.

The auto-mode classifier refused a parent in `bypassPermissions`. The parent
therefore ran in the default permission mode, with one allow rule for
`Bash(/bin/sh …/run-switch.sh)`. In the run of record, the parent ran the
command without a prompt.

### Controls

`task owner:quiet-interval STEP=controls` gives the instrument wrong
subjects in a scratch HOME, with no harness. Each must come back dirty:

| Control | Result |
| --- | --- |
| A settings temporary that names the control's own PID | Recorded with that PID's ancestry, and reported as unsolicited |
| A registry written in place | Reported outside a program window |
| A registry replaced without its extended attribute | Reported as lost metadata |
| The analysis of these subjects | Exit 1 |

A mutant whose PID pattern never matches failed the first control. A
mutant that ignores lost extended attributes failed the third. In the run of
record, the recorder saw each program write. That positive control shares
the run's files and flags.

### Records

The work folder is `/Users/antony/.local/state/owner-staging/trials/quiet-interval-1`.

| Subject | SHA-256 |
| --- | --- |
| `record.json`, from the final analysis | `33c8e747d49b72b39828698adb0007b125ba4ebb01b35272979c52c19b845168` |
| The tool of the final analysis | `9434198f41a3247f82ac4b012287387c4ec650ad4302cf36a974b25202865894` |
| `watch.jsonl` | `8525f48385f5b3664fe931db7052026b426717aa6dc68bc379503be926bfabf0` |
| `switch.log` | `69124a3bfaba1952a4682d99a2553b31dbe2648c875fffb6724097af9fd90ef3` |
| `seed.json` | `052166d2061a5c3f610494a02247a7773a0a1d2968ef683a52915c281b590ea6` |
| The tool of the run, `quiet_interval.run.py` | `6a96ef00faf3254057944f5f6560e765d52f3190a9c39e8133110c39a0779c6b` |

The tool's last modification came before the run started. Later edits
changed only its analysis and added its controls. `analyse` reads the
recorded files again, and the record names both tool digests.

### Limits

- One run. It shows that this layout had a quiet interval once. It does not
  bound how often a background writer acts.
- Claude Code 2.1.322 is not a held version. A different held version needs
  another run.
- The parent ran in the default permission mode with one allow rule.
  Finish's own permission mode may differ.
- `plugin:context7` had no credential. It is an HTTP server with no local
  process, so this run did not measure an authenticated Context7.
- Registry temporaries name no PID. The analysis attributes their writes by
  the program's windows. No such write fell outside a window.
- The recorder lists a folder when the kernel wakes it. Two changes inside
  one wake read as one change. The `installed_plugins.json` temporary was
  not seen, although the replacement was.
- The stubs replace the metadata queries of gates 4 and 5. The scope
  variable replaces the narrow process inspection. The real switch runs
  neither.
- The real HOME was not measured. The staging HOME stands for it, with the
  owner's plugin versions and MCP servers.

## Claude Code native residue in staging

`staging-claude-native-trial-k285` delivered this section on 2026-11-10. It
measures the Claude Code residue of
[Native residue](specs/owner-skill-delivery.md#native-residue) beside Ravel
delivery: the technical-communication hooks, `/guardrail` and a user-invoked
skill. It also inventories the staging HOME. The staging HOME held the layout
that [Quiet interval in staging](#quiet-interval-in-staging) left.

### Result

Each trial passed, and each control came back dirty:

| Trial | Harness | Result |
| --- | --- | --- |
| `native-residue` | Claude Code | Passed. Both plugins ran from their marketplace directories. The four layers of the hooks ran, `/guardrail` stopped the blocked command, and the listing held no skill of the hooks plugin. |
| `user-invoked` | Claude Code, Codex | Passed. Neither agent loaded `authoring-conventions`. |
| `user-invoked-named` | Claude Code, Codex | Claude Code passed. On Codex, the agent read the skill before it wrote the file, but the `requires` verdict failed. "The user-invoked skill" below gives the reason. |
| Inventory | None | Passed. The staging rows held no anomaly, and `guardrail@personal` was the classified native residue. Each of three mutated clones reported an anomaly. |

### The tool

[`staging_trial.py`](evaluations/owner-skill-delivery/staging_trial.py)
gained three case fields and the `inventory` command:

- `turns` sends each text as a later turn through the feeder of the
  compaction case. Ravel wraps the first prompt in its discovery text, so
  `/guardrail` cannot be the first prompt. The owner types it in a later
  turn, and the case does the same.
- `{gate}` in a prompt becomes `OUT/gate`. The document gate of the hooks
  skips files under `/tmp`, `/private/tmp` and `/var/folders`, and each
  staging work tree is under `/private/tmp`. A gate file in the work tree
  would therefore show nothing.
- `forbids` fails a load of a named skill. A load is a classified read, a
  result that holds the skill's `name:` line and its first heading, or a
  native Skill tool call. A listing of front matter is no load.
- `nativeResidue` reads the init event, the session file and the reply log
  of the hooks. The module docstring states each condition.
- `inventory` runs `inventory.py` with HOME set to the staging HOME, and
  three controls on `cp -c` clones. `task owner:staging:inventory` runs it.

The cases are
[`native-residue`](evaluations/owner-skill-delivery/staging-cases/native-residue.json),
[`user-invoked`](evaluations/owner-skill-delivery/staging-cases/user-invoked.json)
and
[`user-invoked-named`](evaluations/owner-skill-delivery/staging-cases/user-invoked-named.json).
Each runs with `task owner:staging:case`, as in
[Rerun commands](#rerun-commands).

### The signals of the hooks

An exploratory session in the staging HOME, without Ravel, found the
signals before the verdict existed. The stream of `claude -p` holds only the
SessionStart hook events. The session file holds a
`hook_additional_context` attachment for each layer that adds context:
SessionStart, UserPromptSubmit and PostToolUse. The Stop hook adds no
context. It writes the reply log
`~/.local/state/technical-communication/sessions/claude-code--<id>.jsonl`
in the staging HOME. The verdict reads each of these signals.

### The native residue case

The session ran from 07:56:57Z to 07:57:27Z with Claude Code 2.1.322 and
`claude-sonnet-5-5` at low effort. It had three turns:

1. The Ravel prompt asked for `OUT/gate/NOTES.md` with one known line. The
   agent read the mandatory file first and then wrote the file.
2. `/guardrail`.
3. The agent created `scratch` and ran `rm -rf scratch`. It then ran the end
   command.

| Observation | Value |
| --- | --- |
| `technical-communication-hooks@writing-hooks` in the init event | `quiet-interval-1/prefix/opt/grove/share/ravel/skills/writing/craft/technical-communication`, the directory that its marketplace names. That path leads to the candidate keg. |
| `guardrail@personal` in the init event | `/opt/homebrew/opt/personal/libexec/plugins/guardrail`, the directory that its marketplace names |
| SessionStart | One context held the first line of the instruction of the hooks |
| UserPromptSubmit | Three contexts held the first line of the reminder, one for each turn, `/guardrail` included |
| PostToolUse | The gate reported `1:19 semicolon` for `NOTES.md` |
| Reply log | One `start` record and two `reply` records |
| `/guardrail` | The session file holds `<command-name>/guardrail:guardrail</command-name>` |
| `rm -rf scratch` | The result was "guardrail: recursive forced delete (rm -rf). Confirm you intend to run: rm -rf scratch". The command did not run. |
| Native listing | `guardrail:guardrail`, and no skill of the hooks plugin. No superseded name. |
| Ravel snapshots | No snapshot of the ten sources holds a `guardrail/SKILL.md` |
| Hooks directory | Tree SHA-256 `20dc91e3…` before and after, with no `__pycache__` |

Claude Code therefore treats the root `SKILL.md` of the skill-free plugin as
no skill. The plugin manifest sets `"skills": []`. The trial measured that
shipped layout, not a manifest without the key.

The listing held `guardrail:guardrail`, a plugin skill. The same instrument
would therefore show a skill of the hooks plugin. No verdict reads the
plugin-cache copies of either plugin, because Claude Code loaded neither.

After all four Claude Code sessions, the candidate keg held no
`__pycache__`.

### The user-invoked skill

`authoring-conventions` sets `disable-model-invocation: true`. Obligation 7
of Ravel's discovery text allows its load only when the caller names it. The
task of both cases asks for a new skill of the Linkuistics skill packs,
which the skill's description names.

| Run | Started | Model | Loads of `authoring-conventions` | Other skills read |
| --- | --- | --- | --- | --- |
| `user-invoked`, Claude Code | 07:57:48Z | `claude-sonnet-5-5`, low | None | `technical-communication` |
| `user-invoked`, Codex | 07:58:23Z | `gpt-6.1-sol`, low | None | `writing-skills` |
| `user-invoked-named`, Claude Code | 07:59:23Z | `claude-sonnet-5-5`, low | Event 5, before the first edit at event 7 | `technical-communication` |
| `user-invoked-named`, Codex | 07:59:49Z | `gpt-6.1-sol`, low | Event 6, before the file write at event 15 | `writing-skills`, `test-driven-development` |

In the Codex `user-invoked` run, the agent wrote: "The Linkuistics
`authoring-conventions` skill requires explicit invocation, so I'll leave it
unloaded."

The Codex `user-invoked-named` run wrote its file with `cat > … <<'EOF'` in
a shell command. The shared classifier of `harness_trial.py` counts no shell
redirection as an edit, so the `requires` verdict reported "the agent made no
edit". The run stays the run of record. The positive control is the
`forbids` verdict instead. Applied to both named runs, it reported the load
at event 5 and at event 6. Applied to both unnamed runs, it reported none.

### The inventory

`task owner:staging:inventory` ran `inventory.py` with SHA-256
`6c5195e7…`, the digest that
[Owner inventory exceptions](#owner-inventory-exceptions) records. The
record is `trials/inventory-2/record.json`.

| Subject | Anomaly rows | `guardrail@personal` |
| --- | --- | --- |
| The staging HOME, 202 rows | None | `native-keep (native residue)` |
| A clone with a second skill in the guardrail plugin | Three, each `UNCLASSIFIED` | `UNCLASSIFIED` |
| A clone with `personal` registered from another directory | Two, each `UNCLASSIFIED` | `UNCLASSIFIED` |
| A clone with an unknown skill in `~/.claude/skills` | One, `UNCLASSIFIED` | `native-keep (native residue)` |

The first run, `inventory-1`, planted the unknown skill in
`~/.agents/skills`. The inventory classifies each unknown entry of a Codex
root as a stale copy that the switch deactivates, which is no anomaly. That
control was therefore wrong, and the second run plants the skill in Claude
Code's root.

The sole-skill rule reads `skills/` of the `installPath` in
`installed_plugins.json`. That path is the plugin-cache copy
`cache/personal/guardrail/unknown`. Claude Code loads the keg's directory
instead. Both trees had the tree SHA-256 `ebe97a42…` on 2026-11-10, so the
verdict was right for this HOME. A later personal keg could add a skill
that Claude Code loads while the cache copy stays unchanged.
[The loaded guardrail directory](#the-loaded-guardrail-directory) records
the repair.

### Controls

`task owner:staging:trial-tests` passed 79 tests, ten of them new. Each
deliberate mutation of a scratch copy failed a test:

| Mutation | Test that failed |
| --- | --- |
| `forbids` ignores the native Skill tool | `test_native_skill_tool_fails` |
| The reminders are counted at SessionStart | `test_facts_from_a_session_file` |
| The listing check of the hooks plugin is dropped | `test_each_mutation_fails` |
| The check for a Ravel snapshot that holds `guardrail` is dropped | `test_each_mutation_fails` |

### Records

The trial directories are in `~/.local/state/owner-staging/trials`:
`native-residue-claude-1`, `user-invoked-claude-1`, `user-invoked-codex-1`,
`user-invoked-named-claude-1`, `user-invoked-named-codex-1` and
`inventory-2`. The exploratory session is `explore-native-1`.

Each model run used the accepted Ravel 2.0 build `ff14cba826b0`, with the
`bin/ravel` SHA-256 `3a9a9129…`, and the staging settings `d8a49f06…`.
Claude Code was 2.1.322, and Codex was 0.167.0.

| Subject | SHA-256 |
| --- | --- |
| `staging_trial.py` of the model runs | `2d2c6cbc01f24cc17f92e04cf6c860d13d46daf2ca2425f7eef2e0608494ee90` |
| `staging_trial.py`, final | `7c575b53b183d0843577b524903f9538c01d5cfa56ff61b4c881e521dfe45672` |
| `harness_trial.py` | `dce23edad5ca2294f1281971225837ce43a2924f9c7ae05fdbd56e0e29e7edfd` |
| `native-residue.json` | `7c5569c04adf87662e04c5129356c46f1177de69b585ae88cdad51b707bb25c1` |
| `user-invoked.json` | `e9941d804ea18a3e685361ce79460fc631380dd9227b4b68b3788e681ac42ab0` |
| `user-invoked-named.json` | `5cd604038c6cade999601334d51e80403cec21759ac2f559abb80e49e760d351` |

The final file adds only the `inventory` command and its controls. `rejudge`
with the final file gave the same verdict for each of the five model runs.

### Limits

- One run of each case. The verdict shows that each layer worked once.
- Claude Code 2.1.322 is not a held version. owner-marketplace-freeze-k253
  chooses the held version.
- Each session ran in `bypassPermissions`. In the owner's `auto` mode, the
  guardrail hook's `ask` shows a prompt instead of a refusal.
- The model runs used `claude-sonnet-5-5` and `gpt-6.1-sol` at low effort,
  the staging defaults.
- Codex has no technical-communication hooks and no guardrail, as the
  native residue states. Only the user-invoked cases ran on Codex.

## MCP servers in staging

`staging-mcp-trials-k286` delivered this section on 2026-11-10. It measures
the `quint-kb` and `codebase-memory-mcp` servers with their skills, which
Ravel delivers, on each harness. Both servers stay native after the switch,
as [Native residue](specs/owner-skill-delivery.md#native-residue) states.
The staging HOME held the layout that
[Quiet interval in staging](#quiet-interval-in-staging) left.

### Result

Each case passed on each harness after two changes to the staging Codex
configuration. Each change has a run before it that failed:

| Run | Harness | Result |
| --- | --- | --- |
| `codebase-memory-mcp-codex-1` | Codex | Failed. Codex started no `codebase-memory-mcp` server, so the agent had no tool of it. |
| `codebase-memory-mcp-codex-2` | Codex | Passed, after the first change |
| `codebase-memory-mcp-claude-1` | Claude Code | Passed |
| `quint-mcp-codex-1` | Codex | Failed. Codex denied each `quint-kb` call, because the approval policy is `never`. |
| `quint-mcp-codex-2` | Codex | Passed, after the second change |
| `quint-mcp-claude-1` | Claude Code | Passed |

The second failure also applies to the real HOME. "The real HOME" below
states the consequence.

### The tool

[`staging_trial.py`](evaluations/owner-skill-delivery/staging_trial.py)
gained the case field `mcp`, `{server, source, skill}`. Its verdict needs
a read of the skill from its source. After that read, at least one call to a
tool of the server must have no error. The verdict needs no edit, because an
edit does not show MCP use. A shell command that names the server, such as
the server's CLI, is no MCP call.

The record keeps each call with the first
300 characters of its result, and the server's state at startup. Only Claude
Code reports that state, in its init event.

[`harness_trial.py`](../scripts/harness_trial.py) now maps a Codex
`McpToolCall` item to the name `mcp__<server>__<tool>`, the form of Claude
Code. A call has an error when its status is not `completed` or its result
sets `isError`.

[`staging_home.py`](evaluations/owner-skill-delivery/staging_home.py)
gained the command `mcp-entries`, which `build` also runs, and
`task owner:staging:mcp-entries` runs it. It makes the two changes below.
It refuses the real HOME, and a second run changes nothing.

The cases are
[`quint-mcp`](evaluations/owner-skill-delivery/staging-cases/quint-mcp.json)
and
[`codebase-memory-mcp`](evaluations/owner-skill-delivery/staging-cases/codebase-memory-mcp.json).
Each task names its skill and its server. The `codebase-memory-mcp` task
asks for the MCP tools and not the CLI. It queries the project of this
workspace, which the account daemon already indexes, and it indexes nothing. The
shared cache therefore gains no project. Each case runs with
`task owner:staging:case`, as in [Rerun commands](#rerun-commands).

### Changes to the staging configuration

| Change | Reason | `.codex/config.toml` |
| --- | --- | --- |
| The `codebase-memory-mcp` entry: `env = { CBM_CACHE_DIR = "/Users/antony/.cache/codebase-memory-mcp" }` replaces `env_vars = ["CBM_CACHE_DIR", "CBM_RUNTIME_DIR"]` | The server is one daemon per account, and a client with another cache folder refuses to start. A server that Codex starts in the staging HOME gets the folder of the staging HOME. `env_vars` forwards names from the launcher's environment, and the staging launcher passes neither name. The Claude Code entry already sets the same value. | `1faeb55d…` to `e4f41d05…` |
| The `quint-kb` entry: `default_tools_approval_mode = "approve"` | A staging Codex session runs with the approval policy `never`. Codex 0.167.0 denies each MCP call that needs approval under that policy. The owner's file approves each of eight `codebase-memory-mcp` tools and no `quint-kb` tool. | `e4f41d05…` to `70c31040…` |

The first change only joins the daemon. Before it, a direct MCP handshake
ran `codebase-memory-mcp` 0.10.6 with the staging launcher's environment.
Without `CBM_CACHE_DIR`, the server exited 1 with "CBM could not start
because the active account daemon uses a different cache directory". The
requested cache digest was `d71e36d5…`. With the variable, it answered
`list_projects`.

The staging Codex log of 2026-11-09 holds the same refusal,
with the same digest, from the server's standard error. That log came from
the app-server run of the compaction case. The `codex exec` runs of this
section wrote no log there.

`default_tools_approval_mode` is a field of an MCP entry in
[`mcp_types.rs`](https://github.com/openai/codex/blob/rust-v0.167.0/codex-rs/config/src/mcp_types.rs#L287-L289).
The denial is in
[`mcp_tool_call.rs`](https://github.com/openai/codex/blob/rust-v0.167.0/codex-rs/core/src/mcp_tool_call.rs#L1691-L1699).
A tool's own `approval_mode` overrides the server's value.

The first change is also in `build`, so a new staging HOME reaches the
daemon. The second change departs from the owner's file. "The real HOME"
below gives the decision that it leaves.

### The runs

Each run used `claude-sonnet-5-5` or `gpt-6.1-sol` at low effort, the
staging defaults, and ended with `staging-ok`. Each read the mandatory file
first. The read column gives the event of the skill read.

| Run | Started | Read | Calls of the server |
| --- | --- | --- | --- |
| `codebase-memory-mcp-codex-1` | 08:23:31Z | Event 9 | None. The agent wrote: "The available tools contain no `codebase-memory-mcp` tools." It left `ANSWER.md` unwritten. |
| `codebase-memory-mcp-codex-2` | 08:26:35Z | Event 8 | `list_projects`, `index_status`, `search_graph`, `get_code_snippet` and `check_index_coverage`, events 10 to 15. Each had no error. `search_graph` found `extract_events` in `scripts/harness_trial.py`. |
| `codebase-memory-mcp-claude-1` | 08:28:46Z | Event 2 | `search_graph` and `check_index_coverage`, events 5 and 6. Each had no error. The init event named the server `connected`. |
| `quint-mcp-codex-1` | 08:27:39Z | Event 7, a content read | `quint_search_docs` twice, events 9 and 10. Each result was "MCP tool call requires approval, but approval policy is never". |
| `quint-mcp-codex-2` | 08:31:46Z | Event 7, a content read | `quint_search_docs` twice, then `quint_get_doc` twice, events 9 to 13. None had an error. |
| `quint-mcp-claude-1` | 08:28:25Z | Event 4 | `quint_hybrid_search`, event 5, with no error. The init event named the server `connected`. |

No run used a shell command that names its server. Each verdict reads the
error flag of the protocol, not the content. In `quint-mcp-codex-2`, the
two searches found no match, and the first `quint_get_doc` result reported a
missing document as text. The second `quint_get_doc` returned
`docs/lang.md`.

### The real HOME

The real `config.toml` has the same `quint-kb` entry, and the owner's policy
starts Codex with `--ask-for-approval never`. A Codex session that Ravel
starts for the owner therefore cannot call `quint-kb` today. The switch
does not cause this, and it leaves the entry unchanged. The specification
requires a repair or an explicit owner scope decision for a failed case.
`owner-quint-kb-approval-k289` asked the owner whether the real entry gains
the approval. [The real quint-kb approval](#the-real-quint-kb-approval)
records the decision and the real run.

The `codebase-memory-mcp` entry of the real HOME needs no change. A session
there has HOME set to the real HOME, so the server gets the daemon's folder.

### Controls

`task owner:staging:trial-tests` passed 92 tests, 13 of them new.
`task trial:controls` passed 85 tests, three of them new. Each deliberate
mutation of a scratch copy failed a test:

| Mutation | Test that failed |
| --- | --- |
| The verdict counts a failed call as a call | `test_only_failed_calls_fail` |
| The verdict drops the order of read and call | `test_a_read_after_the_first_good_call_fails` |
| The verdict ignores the source of the read | `test_a_read_from_another_source_fails` |
| The skill of `mcp` is not a named skill, so the verdict finds no content read | `test_the_skill_of_mcp_is_a_named_skill` |
| The Codex mapping ignores `isError` | `test_an_error_result_or_a_failed_status_is_an_error` |
| The Codex mapping keeps the item type as the name | `test_a_completed_call_keeps_its_server_tool_and_result` and `test_the_stream_form_maps_too` |
| `mcp-entries` keeps the `env_vars` line | Each of the three `JoinCbm` tests |
| `mcp-entries` adds no approval | `test_quint_kb_gets_the_server_wide_approval_once` |

The two failed runs are the controls of the verdict on real records. Each
failed for the reason that the change after it repaired.

### Records

The trial directories are in `~/.local/state/owner-staging/trials`, named
as the runs above. Each failed run's directory holds `config.toml.before`,
the staging Codex configuration before the change that followed it.

Each run used the accepted Ravel 2.0 build `ff14cba826b0`, with the
`bin/ravel` SHA-256 `3a9a9129…`, and the staging settings `d8a49f06…`.
Claude Code was 2.1.322, Codex was 0.167.0, `codebase-memory-mcp` was
0.10.6, and the Quint clone was at `cc75369f741a`.

| Subject | SHA-256 |
| --- | --- |
| `staging_trial.py` of each run | `9cb0ca0fa9766bad74a2cd2620812c8f67353eb0cf3d6a1c670e74f111a7fb9b` |
| `harness_trial.py` of each run | `1f10bacfee19a1756d0862ad7ca0a4a912bcb6174442b47614f0366ff21e657f` |
| `staging_home.py` of `quint-mcp-codex-2` | `a392b0438330e4c04ba3d8490808096c1b31036d13eb847daf7beafb75cc4a3b` |
| `quint-mcp.json` | `cfd378ef9934c239881cfcc717c076025cfa3992e5d23362cef7f01e99d7eaff` |
| `codebase-memory-mcp.json` | `111e4bcdeec6ad4329827449cd7129dc5e8009f5a9566338a9db573af1b6672c` |

`codebase-memory-mcp-codex-1` ran with the committed `staging_home.py`,
SHA-256 `0b81f3c9…`. The next four runs ran with an intermediate file that
had only the cache change, as the command `join-cbm`. No trial calls that
command, and the launcher's code was the same in each file.

The staging `.claude.json` changed from `4fc39d25…` to `03464631…` during
the Claude Code runs. `mcp-entries` reported no change to it, and Claude
Code rewrites that file during each session.

### Limits

- One run of each case. Each verdict shows that the path worked once.
- Each task named its skill and its server. The discovery trials measured
  implicit activation, so these cases did not repeat it.
- Claude Code 2.1.322 is not a held version. owner-marketplace-freeze-k253
  chooses the held version.
- Each Claude Code session ran in `bypassPermissions`. In the owner's `auto`
  mode, Claude Code can ask before an MCP call.
- The `codebase-memory-mcp` cases depend on the account daemon and on an
  index that it keeps. A staging HOME without the real daemon cannot run
  them.

## TestAnyware in staging

`staging-testanyware-trial-k287` delivered this section on 2026-11-10. It
measures the `using-testanyware` skill, which Ravel delivers from the
`testanyware` source, on each harness. The agent reads the skill, runs the
tool's command reference and does one real VM operation through the
installed `testanyware` command. The command and the TestAnyware app stay
native after the switch, as
[Native residue](specs/owner-skill-delivery.md#native-residue) states. The
staging HOME held the layout that
[Quiet interval in staging](#quiet-interval-in-staging) left.

### Result

The case passed on each harness. Two conditions of the staging arrangement
needed a change before a VM could start. The section "The account's
TestAnyware app" gives the first, and "The Codex sandbox" gives the second.

| Run | Harness | Result |
| --- | --- | --- |
| `testanyware-vm-codex-1` | Codex | Failed, as planned. The sandbox denied the app's socket, so no VM started. |
| `testanyware-vm-claude-1` | Claude Code | Passed. VM `testanyware-cdd0e247` started and stopped. |
| `testanyware-vm-codex-2` | Codex | Passed, with the sandbox's network access. VM `testanyware-e98a94a6` started and stopped. |

No VM of the trials ran after its session, and the runner stopped none.

### The account's TestAnyware app

On a macOS host, each `testanyware` command that reaches a guest goes
through the TestAnyware app. The app runs once for each account, in the
real HOME. The Host CLI finds the app's socket at
`$HOME/Library/Application Support/TestAnyware/testanyware.sock`
(`default_socket_path` in
`cli-rs/crates/testanyware-app-protocol/src/socket.rs` at TestAnyware
`v2.8.0`). It also derives its state and data folders from HOME. A command
in the staging HOME therefore reaches no app.

Three read-only probes of `testanyware vm list --json` 2.8.0 showed each
step:

| HOME and variables | Result |
| --- | --- |
| The staging HOME | `APP_UNREACHABLE` `not_running`. No socket exists below the staging HOME. |
| A scratch HOME, with `TESTANYWARE_APP_SOCKET` set to the real socket | `APP_INCOMPATIBLE` `directories`. The client's folders were below the scratch HOME, and the app's folders were below the real HOME. |
| The same, with a `config.rhai` that sets the real `state_dir` and `data_dir` | Listed the five golden images |

The runner therefore joins the app for the TestAnyware case only. It sets
`TESTANYWARE_APP_SOCKET` to the real socket. It also sets
`TESTANYWARE_CONFIG` to `OUT/testanyware-config.rhai`, which names the real
built-in folders `~/.local/state/testanyware` and
`~/.local/share/testanyware`. The owner's `config.rhai` sets neither
folder. Ravel passes the caller's environment to the managed process, as
"Managed process and result" of Ravel's `docs/specs/session-execution.md`
states. Each record lists both names in its environment.

This join leaves the retained staging HOME unchanged. A link from the
staging HOME to the real app folder would also join the app. The isolation
rule of [Staging setup and isolation trial](#staging-setup-and-isolation-trial)
allows only the two sign-in links to lead out of the staging HOME. The trial
therefore uses the variables. In the real HOME, a session needs neither
variable, because HOME already names the app's socket and folders.

The runs wrote no TestAnyware file into the staging HOME. The VMs wrote
their files into the real state folder, where the app keeps every VM.

### The Codex sandbox

Each Codex staging trial runs `codex exec --sandbox workspace-write` with
the approval policy `never`. `codex sandbox` 0.167.0 measured this sandbox
with the join variables and `testanyware vm list --json`:

| Codex home and overrides | Result |
| --- | --- |
| An empty home, `sandbox_mode="workspace-write"` | `APP_UNREACHABLE` `socket_error`: "Operation not permitted (os error 1)" |
| The same, with `sandbox_workspace_write.network_access=true` | Listed the five images |
| The same, with `default_permissions=":danger-full-access"` | Listed the five images |
| A copy of the staging `config.toml`, `sandbox_mode="workspace-write"` | `APP_UNREACHABLE` `socket_error` |
| The same copy, with `sandbox_workspace_write.network_access=true` | Listed the five images |

The staging `config.toml` sets `default_permissions = ":workspace"`. The
last two rows show that the network override still applies with it.

The first plan gave the Codex case the owner's own profile,
`-c default_permissions=:danger-full-access`, as the owner's `config.rhai`
passes it. Claude Code's auto-mode classifier refused that session as
"Create Unsafe Agents". This leaf did not work around the refusal. The case
instead keeps the trial's sandbox and approval policy, and adds
`sandbox_workspace_write.network_access=true`. That grant is narrower than
the owner's profile, and the third row shows that the owner's profile also
reaches the socket.

A Codex session under `workspace-write` without network access cannot use
TestAnyware. The owner's policy does not start such a session.

### The tool

[`staging_trial.py`](evaluations/owner-skill-delivery/staging_trial.py)
gained two case fields:

- `testanyware`, `{source, skill, answer, screenshot}`. The runner sets the
  two join variables. Before and after the session, it lists the names in
  the app's state folder `vms` and the running VMs that `testanyware vm
  list` reports. It then stops each new VM that still runs, and records the
  stop.
- `codexConfig`, a list of `-c` overrides that the Codex route adds. An item
  may not set the sandbox mode, the approval policy, the permission profile,
  the model or the effort.

The verdict `testanyware` needs these events in this order:

1. A read of the skill from its source.
2. A successful `testanyware llm-instructions` or `testanyware
   capabilities`, before the first successful `vm start`.
3. A successful `testanyware vm start`.
4. A successful `testanyware vm stop` at or after that start.

It also needs these runner facts:

- At least one VM ID gained a file in the state folder during the session.
- The first line of `ANSWER.md` is one of those IDs.
- No VM of the session still runs, and none keeps its spec file
  `<id>.json`.
- `screen.png` starts with the PNG signature.

A calibration start outside Ravel set these facts. A start of a Linux VM
took 18 s. It wrote `<id>.json`, `<id>.meta.json`, `<id>.keeper.log` and
`<id>.tart.log`. The stop removed the first two and the VM from `vm list`,
and kept the two logs.

The record keeps the first 300 characters of each `testanyware` call's
result. It keeps the number of files in the state folder, not their names,
because the folder holds the logs of every earlier VM.

The case is
[`testanyware-vm`](evaluations/owner-skill-delivery/staging-cases/testanyware-vm.json).
It names the skill and asks the agent to read the tool's command reference
as the skill directs, without the name of the command. It asks for a Linux
VM, which takes no macOS guest slot. The commands are:

```sh
S=~/.local/state/owner-staging
task owner:staging:case CASE=testanyware-vm HARNESS=claude STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/testanyware-vm-claude-N
task owner:staging:case CASE=testanyware-vm HARNESS=codex STAGING=$S/home \
  RAVEL_2_0_PREFIX=$S/ravel-2.0 OUT=$S/trials/testanyware-vm-codex-N
```

The TestAnyware app must run in the real HOME. The runner refuses to start a
session when it cannot list the VMs.

### The runs

Each run used `claude-sonnet-5-5` or `gpt-6.1-sol` at low effort, the
staging defaults, and ended with `staging-ok`. Each read the mandatory file
first. The columns give the event numbers.

| Run | Started | Skill read | Reference | VM operation |
| --- | --- | --- | --- | --- |
| `testanyware-vm-codex-1` | 08:52:24Z | 8 | 10, `llm-instructions` | None. `testanyware doctor` at event 11 reported that the sandbox denied the socket. The agent ended without a VM. |
| `testanyware-vm-claude-1` | 08:53:49Z | 6, by the Read tool | 7, `llm-instructions \| head -150` | One shell call at event 9 started the VM, saved a 1920×1080 screenshot of 42,778 bytes and stopped the VM |
| `testanyware-vm-codex-2` | 08:55:50Z | 5, by `cat` | 7, `llm-instructions` | `vm start --platform linux --json` at 12, `screen capture` at 13, `vm stop` at 14. The screenshot was 1920×1080 and 63,442 bytes. |

The Claude Code agent read the first 150 lines of the command reference,
not the whole text. Before each start, both successful agents also ran `vm
list`. The Codex agent also ran `testanyware doctor`. In each successful
run, the state folder gained `<id>.keeper.log` and `<id>.tart.log` and kept
no spec file.

### The doctor check

In `testanyware-vm-codex-2`, `testanyware doctor --json` exited 1. Its
check `login_item` failed with "the app does not start at login", and its
check `local_network` warned that no guest connection showed the grant
yet. The same command in the real HOME gave the same two results after the
runs. The VM operations still succeeded, because the app was running.

The setup of the app in the TestAnyware reference turns on "Start at
Login". Without it, the app does not run after the next login, and each VM
command fails as `APP_UNREACHABLE` until the owner opens the app. The
switch's session probe runs `testanyware doctor` and accepts a failed check,
as its prompt in
[`owner_switch_probes.py`](../scripts/owner_switch_probes.py) states. The
setting is the owner's to change.

### The size and capture time of the source

"Not measured" named the size and capture time of a TestAnyware checkout
subdirectory. Homebrew replaced the checkout with the keg, so this leaf
measured the keg's source. `/opt/homebrew/share/ravel/skills/testanyware`
links to `Cellar/testanyware/2.8.0/share/ravel/skills/testanyware`. It holds
two files: `.claude-plugin/plugin.json` of 575 bytes and
`skills/using-testanyware/SKILL.md` of 5,377 bytes.

The accepted Ravel 2.0 build resolved settings that held only this source.
Each of three runs used a new HOME, so each captured a new snapshot.

| Run | Exit | Status | Time | `resourceDigest` |
| --- | --- | --- | --- | --- |
| 1 | 0 | `resolved` | 0.037 s | `3e9d7929eddd…` |
| 2 | 0 | `resolved` | 0.033 s | `3e9d7929eddd…` |
| 3 | 0 | `resolved` | 0.032 s | `3e9d7929eddd…` |

The time covers the whole `ravel skills resolve --json` process. Each
`originPath` named the versioned keg, and each digest equals the accepted
value of [Staging setup and isolation trial](#staging-setup-and-isolation-trial).

### Controls

`task owner:staging:trial-tests` passed 103 tests, 11 of them new. Each
deliberate mutation of a scratch copy failed a test:

| Mutation | Test that failed |
| --- | --- |
| The verdict needs no reference command | `test_each_mutation_of_the_events_fails`, subtests "no reference" and "failed reference" |
| The verdict drops the order of the reads and the start | `test_each_mutation_of_the_events_fails`, subtest "reference after start" |
| A stop before the start counts | `test_each_mutation_of_the_events_fails`, subtest "stop before start" |
| A failed start counts | `test_each_mutation_of_the_events_fails`, subtest "failed start" |
| A name that ends in `testanyware`, such as a skill path, counts as a call | `test_a_message_or_a_skill_path_is_no_call` |
| A VM that ran before the session counts as still running | `test_vm_facts_from_the_state_folder_and_the_work_tree` |
| The verdict ignores the answer | `test_each_mutation_of_the_facts_fails`, subtests "another answer" and "no answer" |
| Any screenshot file counts as a PNG | `test_vm_facts_from_the_state_folder_and_the_work_tree` |
| The skill of `testanyware` is not a named skill | `test_the_skill_is_named_and_each_call_keeps_the_start_of_its_result` |
| `codexConfig` adds no override | `test_codex_overrides_keep_the_sandbox` |
| `codexConfig` may set a reserved key | `test_the_case_file_loads_and_a_wrong_shape_refuses`, subtest "a reserved key" |

The first form of the look-behind test survived its mutation, because its
skill path had no space after the name. The test now uses `ls
…/using-testanyware && cat …`, and the mutation fails it.

`testanyware-vm-codex-1` is the control of the verdict on real records. It
failed for the reason that the network override removed.

### Records

The trial directories are in `~/.local/state/owner-staging/trials`, named as
the runs above. Each holds `testanyware-config.rhai`, the configuration of
its join.

Each run used the accepted Ravel 2.0 build `ff14cba826b0`, with the
`bin/ravel` SHA-256 `3a9a9129…`, and the staging settings `d8a49f06…`.
Claude Code was 2.1.322, Codex was 0.167.0, and `testanyware` was 2.8.0 at
`/opt/homebrew/Cellar/testanyware/2.8.0/bin/testanyware`.

| Subject | SHA-256 |
| --- | --- |
| `staging_trial.py` of `testanyware-vm-codex-2` and of each rejudgement | `eec7932612dce5e4c69980df12b99c0136521d9d731e79d7ff0f14ae487c35a3` |
| `staging_trial.py` of the first two runs | `b6468c3bd42961b4dfbcc4ba2987fce84bd099879fde5b994746b0e89c1c07a4` |
| `harness_trial.py` of each run | `1f10bacfee19a1756d0862ad7ca0a4a912bcb6174442b47614f0366ff21e657f` |
| `staging_home.py` of each run | `a392b0438330e4c04ba3d8490808096c1b31036d13eb847daf7beafb75cc4a3b` |
| `testanyware-vm.json` of `testanyware-vm-codex-1`, without a Codex field | `a9d74f6e00940f656f903b6825e130bbf6c5df5da7af15a44fc660b677c52565` |
| `testanyware-vm.json` of `testanyware-vm-claude-1`, with `codexPermissions` | `786e0d9fda27fa38e8ef65bd1c2fbaa40ad9c95d3de99f795df3e9521a26f11e` |
| `testanyware-vm.json` of `testanyware-vm-codex-2`, the committed case | `ce9ac8037f0f47670987f856fc510915b6d465f78c04f1570a29ff4af3e632a2` |

The first two runs ran before `codexConfig` replaced `codexPermissions`.
The verdict code was the same in both files. `staging_trial.py rejudge`
then judged both runs again with the committed file. Each new verdict
equalled its first verdict item by item. Each directory keeps the first
record as `record.json.first`.

### Limits

- One run of each case. Each verdict shows that the path worked once.
- The task named its skill. The discovery trials measured implicit
  activation, so this case did not repeat it.
- The Codex case ran with the sandbox's network access, not with the owner's
  full-access profile. A `codex sandbox` probe showed that the owner's
  profile reaches the socket. No real Codex session ran with that profile.
- The trial joins the real app through two variables. A session in the real
  HOME needs neither, and no real-HOME session ran here.
- Each Claude Code session ran in `bypassPermissions`. In the owner's `auto`
  mode, Claude Code can ask before a shell command.
- The case used a Linux guest. A macOS guest needs a macOS slot and the
  background wait of the skill. No macOS guest started.
- Claude Code 2.1.322 is not a held version. owner-marketplace-freeze-k253
  chooses the held version.

## The loaded guardrail directory

`inventory-guardrail-loaded-dir-k288` delivered this section on 2026-11-10.
[The inventory](#the-inventory) of the native residue found that the
sole-skill rule read only the plugin-cache copy of `guardrail@personal`.
Claude Code loads the plugin from the keg's directory instead. The rule now
reads both directories.

### Result

| Check | Result |
| --- | --- |
| `task owner:inventory:controls` | Passed 42 of 42 controls. Five are new, and one has a new name. |
| Each mutant of a new check | Failed the control of that check, and only that control. |
| The previous instrument, SHA-256 `6c5195e7…` | Failed the five new cases. |
| `task owner:staging:inventory` | Passed. The staging HOME held no anomaly row. `guardrail@personal` was the classified native residue. Each of three mutated clones reported an anomaly. |

### The rule

The rule first requires the registration of `personal` from
`/opt/homebrew/opt/personal`, as before. It then reads
`.claude-plugin/marketplace.json` in that directory. The entry of
`guardrail` must name a string source. The rule joins that source to the
marketplace directory, as Claude Code does. In staging, the result was
`/opt/homebrew/opt/personal/libexec/plugins/guardrail`. That is the
directory of the init event in
[The native residue case](#the-native-residue-case).

The rule refuses the plugin in each of these cases:

- The marketplace file names no `guardrail` entry, or the entry names no
  directory.
- The entry declares `skills`.
- The loaded directory lies outside `/opt/homebrew/opt/personal`.
- In the loaded directory or in the plugin-cache copy, `skills/` holds a
  skill other than `guardrail`.
- In either directory, `skills/guardrail/SKILL.md` is absent.
- In either directory, `.claude-plugin/plugin.json` declares `skills` or
  cannot be read.

The plugins reference of Claude Code
(<https://code.claude.com/docs/en/plugins-reference>) is the source for the
two `skills` refusals. Under "How each key combines with its default
location", a manifest `skills` path adds to the scan of `skills/`. Under
"Marketplace entries and the manifest", the `skills` of an entry append to
those of the plugin. Either key could therefore add a native skill that
`skills/` does not show.

### Controls

The origin is a fixed path that a fixture HOME cannot write.
[`controls.py`](evaluations/owner-skill-delivery/controls.py) therefore
builds a fixture keg beside each fixture HOME. It runs a copy of the
instrument in which the literal `/opt/homebrew/opt/personal` names that
keg. The instrument itself takes no input that changes its origin. An
owner's run therefore cannot widen the exception.

Two new cases change the loaded directory: an extra skill and a manifest
`skills` path. Two change the marketplace entry: `skills` in the entry and
a source outside the keg. The fifth removes the marketplace file. The earlier case of an extra skill now names the plugin-cache copy.
Each new case expects `UNCLASSIFIED`.

Each mutant removed one check from a copy of the new instrument:

| Mutant | Controls that failed |
| --- | --- |
| No check of the loaded directory | The extra skill and the manifest `skills` path, each in the loaded directory |
| No check of the plugin-cache copy | The extra skill in the plugin-cache copy |
| No refusal of a manifest `skills` path | The manifest `skills` path in the loaded directory |
| No refusal of `skills` in the entry | `skills` in the marketplace entry |
| No containment of the loaded directory | The loaded directory outside the keg |

The previous instrument failed each of the five new cases. Its output for
each of them equalled the unchanged run.

### The staging run

`task owner:staging:inventory` ran the new `inventory.py` against the
staging HOME. The record is `trials/inventory-3/record.json` in
`~/.local/state/owner-staging`. Its staging rows and controls equal those of
`inventory-2` in [The inventory](#the-inventory): 202 rows and no anomaly.
The three clones held three, two and one anomaly rows.

The clone controls of `staging_trial.py` can only change the clone. The
loaded directory is in the keg under `/opt/homebrew`, so no clone control
adds a skill there. `controls.py` covers that case.

On 2026-11-10, the loaded directory and the plugin-cache copy held the same
files with the same SHA-256 digests. The keg was `personal` 2026-11-06.

### Records

| Subject | SHA-256 |
| --- | --- |
| `inventory.py` | `880fe562fd618330e1a6471fd741666760a868c259bf91d2b712caf49c3fd98d` |
| `controls.py` | `1324f82b55266796a1451931cfdf8daa86a5bf9d79e428340826dd25b60ef8d8` |
| `trials/inventory-3/record.json` | `d3d6f54a92132f62c2b363ad6163f8f5d70ac246a534779c5e3f8bc80bb88bc4` |

`docs/owner-cutover/switch-plan-draft.json` still gives the baseline
instrument the earlier digest `6c5195e7…`. owner-switch-plan-k254 writes
the draft again and copies `inventory.py` into the prepared folder.

### Limits

- The rule reads the marketplace file that Claude Code 2.1.322 read in
  staging. A later version could resolve a directory marketplace in another
  way. The init event of a later staging run would show that change.
- The rule refuses a manifest `skills` key whatever its value. A key that
  names only `./skills` adds no skill, but the rule still refuses it.
- The rule does not read a `commands` key or a `commands/` folder.
  `commands` is outside the sole-skill rule of
  [Concurrent writers](specs/owner-skill-delivery.md#concurrent-writers).

## The real quint-kb approval

`owner-quint-kb-approval-k289` delivered this section on 2026-11-10.
[The real HOME](#the-real-home-2) of
[MCP servers in staging](#mcp-servers-in-staging) left one decision to the
owner. A Codex session that Ravel started for the owner could not call
`quint-kb`.

### Decision

The owner chose the server-wide approval from three options. The other
options were an approval of each named tool and an accepted gap. The owner
also chose to record the entry in the maintained manual. The switch set does
not grow.

The server's source was the evidence for the choice. The Quint clone at
`cc75369f741a` gives `quint-kb` 16 tools. Each tool reads a local
documentation index. No tool source writes a file or starts a process. A
tool that a later Quint version adds gets the same approval.

### The change

| Subject | Value |
| --- | --- |
| Line added to `[mcp_servers.quint-kb]` | `default_tools_approval_mode = "approve"` |
| `~/.codex/config.toml` before | `c6b3982d7eb6a7b5f5d80c5f9612e53f0b80ef80657d7ae3e21a4d369755b61d` |
| `~/.codex/config.toml` after | `a57ddd3b733738cc1ce8bf2a2a18c3d567cbdbcb15239a879df08e44f7b178b2` |

A `diff` of the copy before the change and the file after it showed only
that line. Mise tracks the file in place, and its watcher records each
edit. The owner publishes that history with `mise dot sync`.

The switch plan draft names no `config.toml`, so no digest in it goes
stale. A search of `docs/` and `scripts/` found no earlier digest of the
file. The same search found the known digest of `staging_home.py` in this
document.

### The real runs

Each run used `ravel start-session` of the installed Ravel 1.1.0 with the
owner's `policy.ts`. Each ran in the main checkout, which Codex trusts, with
the prompt below. Each ran under `script`, because the interactive harness
needs a terminal. The launcher removed the variables of the enclosing
session.

```text
This is a one-step check of the quint-kb MCP server. Do not read, create or edit any file.

1. Call the MCP tool quint_list_docs of the quint-kb server once. Use the MCP tool, not a shell command.
2. If the call returned a result with no error, run the shell command: ravel end-session quint-kb-ok
3. If the call returned an error, run the shell command: ravel end-session quint-kb-failed

Do nothing else.
```

| Run | Kind | Harness | Result |
| --- | --- | --- | --- |
| `real-home-quint-kb-claude` | `impl` | Claude Code 2.1.322, `claude-opus-5-5` at high effort | `quint-kb-ok`. This run measured no Codex session. |
| `real-home-quint-kb-codex-1` | `review-impl` | Codex 0.167.0 | No result. Codex exited 0 before its first turn. |
| `real-home-quint-kb-codex-2` | `review-impl` | Codex 0.167.0, `gpt-6.1-sol` at high effort | `quint-kb-ok`, exit 0 |

The live `policy.ts` routes `impl` to Claude Code. Its `lead` selects
`opus`, and its `review` selects `sol`. The installed `config.rhai` of
Ravel 2.0 selects `sol` for `lead`, as
[The staging HOME](#the-staging-home) states. After the promotion, the lead
kinds also start Codex, and they need the same approval.

In the first Codex run, `script` read the end of `/dev/null` and sent `^D`
to the terminal. Codex then exited without a turn. In the second run, a
`sleep` pipe kept the input open.

The Codex record of the second run is
`rollout-2026-11-10T20-32-37-01a1c4cd-64bd-70b3-8a7c-df7ed3875a28.jsonl`.
Its turn context gives the approval policy `never`, the sandbox
`danger-full-access` and the model `gpt-6.1-sol`. Its one `McpToolCall`
names the server `quint-kb` and the tool `quint_list_docs`. The status is
`completed`, `isError` is false, and the result lists 14 documents. The
agent then ran `ravel end-session quint-kb-ok`, which exited 0. No other
tool ran.

The Claude Code run also made only the MCP call and the ending command.

The digests of `config.toml` and `policy.ts` were the same before and after
the second run. The `policy.ts` digest was
`30a0e5fca7c1446999bd19289566fc078053a503c75f03ca7056e641f836f131`.

### Controls

`quint-mcp-codex-1` in [The runs](#the-runs) is the control. It ran the
same entry with the same Codex version and approval policy, and its record
showed each denial. The real HOME had no run before the change, because the
staging run had already shown the failure.

### Records

The run folders are in `~/.local/state/owner-staging/trials`, named as the
runs above. `real-home-quint-kb-codex-2` also holds `config.toml.before`
and a copy of the Codex record.

| Subject | SHA-256 |
| --- | --- |
| The prompt | `de7b2c2c1f5f05706fd357a088fe6cbb7521181970558c69a82e62d1ca0363a5` |
| The Codex record | `43f49a08b1d7eacb196ad00ee55fbdefd640065290c4d8bf063f82a8dde9f2a5` |

### Limits

- One run on Codex. It shows that the path worked once.
- In each run, Ravel 1.1.0 reported two failed stop attempts of the process
  group, each "signal 9: Operation not permitted". No harness process stayed
  after any run.
- Ravel 2.0 and the switch can change how Codex starts. A staging or
  cutover run after the promotion would show that change.

## Staging acceptance

`owner-staging-acceptance-k252` delivered this section on 2026-11-10. It
closes order 5 of the
[Cutover contract](specs/owner-skill-delivery.md#cutover-contract). The
switch plan of owner-switch-plan-k254 and the source installation of
owner-pack-checkouts-k93 read their accepted values here. No rehearsal
record or other admission token is part of this acceptance. The dry run of
owner-switch-rehearsal-k255 reads these values.

### Result

The drafts pass under the staging HOME. Each trial of the cutover contract
passed on each harness that it names, or it has a recorded scope decision.
The quiet-interval measurement passed, so no unsolicited writer blocks
acceptance. The staging inventory held no anomaly. It used the instrument
that inventory-guardrail-loaded-dir-k288 repaired.

At 09:47Z on 2026-11-10, each retained path existed, each skill keg was at
its accepted version, and each source gave its accepted digest. No source
changed after the trials that read it. Only `writing` changed during the
trials, and the trials that read it ran again.
[The affected staging trials](#the-affected-staging-trials) records that rerun.

This leaf found that the switch's gate 4 could not run the staging tools.
The new `staging_trial.py rerun` entry repairs that.
[Trials for gate 4](#trials-for-gate-4) states the defects and the controls.

### The accepted sources

core-candidate-restaging-k300 renewed the candidate on 2026-11-11.
[Core candidate restaging](#core-candidate-restaging) records the build and
the reruns. The candidate commit is
`2a24d5e61f29070bebe1fb005fcd74fc8f2b4ca3`, on `origin/gh-issue-25`. The
tree ID of its `plugins/grove` is
`961fddd04bb7b271e9896d07dc0225348fa8c378`, and the blob ID of its
`plugins/packs.json` is `2cebaf5f062d08b50b22d058bd8c4db3d412f8a6`. The
candidate's archive has the SHA-256
`9ab7d03193109f7bdebf1aeb6f2429a592e5bc056f447499cd72ac7a4bc680b5`.

The earlier candidate was `ef8a79dd7621a437177982e24d8ec27831b4add4`, with
the `plugins/grove` tree ID `885051a94eddaa81fa7a1a9900c3ac4db775aa84`, the
same pack-record blob ID and the archive SHA-256
`fcb7603d70ccadcb3fcb071c5c2f79da165c807656f4723620142acab577546b`. Its
`grove-core` digest was
`8d92cd58fca8f21a87361a9d956af7e5a38e88751e25cbaff5e2384554806b51`. The
trials of this section read that earlier candidate, except where
[Core candidate restaging](#core-candidate-restaging) names a rerun.

| Source | Formula | Accepted version | Commit | Tree ID of the source directory | `resourceDigest` |
| --- | --- | --- | --- | --- | --- |
| `grove-core` | `linkuistics/taps/grove` | The Grove release that step 4 admits | `2a24d5e61f29070bebe1fb005fcd74fc8f2b4ca3`, the candidate | `961fddd04bb7b271e9896d07dc0225348fa8c378` | `d8eec38844f5e8ffd4e998ef34f3456a984846166b2e34cc2c8e65189b2f145e` |
| `requirements` | `linkuistics/taps/grove` | The same | `c51ec8bebd811c0087ad19a1d66d0aa87bd01386` | `f3cf8128fd206c48d73b061cc5ef6b536d4f9d61` | `07b2d25e8bd20259c71f7419092329368d8ae48bec6ca973343a601a8ec515c8` |
| `design` | `linkuistics/taps/grove` | The same | `aa088156d1a55e28dd9b59f0330f4ab9f1f3d330` | `dd04e268aeb5eba82c527755b1f52bee3565a216` | `d8ad3a2af1b5029bc0de93747f4d7297038651180f6877fbd738d94fcbf6aa14` |
| `engineering` | `linkuistics/taps/grove` | The same | `8d1a2bc3e07bec4ce57e0391a6aa778dff63a017` | `d8a891cd5c13e9866f7937537afc7979fb3ef0f7` | `7e70b280e7a932489a048d108d4dae2bc63b387cf8fc6b37298663336e0fcc66` |
| `research` | `linkuistics/taps/grove` | The same | `8f321f2e99e09a8ba5e60fa4a08871727e84df62` | `79dc2a530cd2d30cd088073423437bb18d61b401` | `7fb3d473f66feafb49afd8846fd94cf3e1fb7b01c584d4ad5f0e54814a8b3fdb` |
| `writing` | `linkuistics/taps/grove` | The same | `578f84d1f5a9bb04573a704bca639530d93b6bca` | `ce8f4633533f9f550d414a474561a77f92cebba0` | `56bcd07b87f46a7b9ff92eed5a5a128f10d3ce64be2f7d934be898e058782212` |
| `personal` | `linkuistics/taps/personal` | `2026-11-06` | `cddcfde901cdaf8069e2aac6f3b6bc5aba9fe925` | `f66e0792b9507194842e5b7d44457cf12d70dd98` | `dc1f26eafe9b7db6f750c95db38a7876beb29b067d829eaf63203323b89d61b6` |
| `testanyware` | `linkuistics/taps/testanyware` | `2.8.0` | `1809a49068a23d9d276913f3a90227dbefb98a82`, tag `v2.8.0` | `0701492a5dae89e5b967e0122a5f1085cccec291` | `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6` |
| `superpowers` | `linkuistics/taps/superpowers` | `7.0.0` | `bb92a77741419a4ab5f06e711a283343f1ada0c3`, tag `v7.0.0` | `4ac675daa3415bc836f707c7e60237ac7b66c7d5`, the root tree | `be1ce6e3c25064b83ba2999b37972deb7f73ce88a5f5dbbba990d88037b17230` |
| `quint` | `linkuistics/taps/quint-llm-kit` | `1.0.0` | `cc75369f741af7d490936f82002c2d28e3b3d78d` | `4600ff66782fa87ea4ee8791b37d6fcc86bed5a6` | `0a8b945fbc6732d5b7b3041a966d26cdf718ca884bd5f3fb8435297d7258edef` |

The five pack commits are the pins of `plugins/packs.json` at the
candidate. The four other commits are the archive URLs of the tap formulae.
The tree IDs come from
[the moves record](repository-moves-record.md#the-ten-ravel-sources). That
table names the earlier candidate `7b113a28a966` for `grove-core`, which has
the same tree.

The plan's `accepted.versions` names the four formulae other than Grove's.
Step 4 admits Grove's version, because Grove's release inside finish sets it.
The plan draft holds these ten digests. Its four versions are the revised
full package versions of
[Packaging-only acceptance](#packaging-only-acceptance).

[Packaging-only acceptance](#packaging-only-acceptance) accepts a revision of
each of the four other formulae, with the same commit and digest. Its
records name the full package versions that the tap now publishes.

### The accepted staging paths

The plan's `accepted.staging` names these two paths. Each must stay until
the switch writes its receipt.

| Key | Absolute path |
| --- | --- |
| `home` | `/Users/antony/.local/state/owner-staging/home` |
| `candidate_keg` | `/Users/antony/.local/state/owner-staging/candidate` |

Since core-candidate-restaging-k300, `candidate_keg` holds the keg of
`2a24d5e61f29`. The keg of the earlier candidate `ef8a79dd7621` moved to
`/Users/antony/.local/state/owner-staging/candidate-8d92cd58`, and it is
removable. The staging settings and both paths of `accepted.staging` kept
their text.

The staging HOME holds its own `.codex` tree, which is no link. The accepted
Ravel 2.0 build is `/Users/antony/.local/state/owner-staging/ravel-2.0`, and
its `bin/ravel` has the pinned SHA-256 `3a9a9129…`. The trial commands below
name that build.

### Trials for gate 4

Gate 4 runs the plan's trials of a Grove source whose installed digest
differs from its accepted digest. Before this leaf, no trial command could
run there, for two reasons:

- **HOME.** Gate 4 sets HOME to the staging HOME. The staging tools take the
  real HOME from HOME, for the real-HOME check, the stale copy and the
  account's tools. The old path therefore failed. `case --name stale-copy`
  under gate 4's environment looked for the stale copy at
  `/Users/antony/.local/state/owner-staging/home/.claude/skills/grove`, and
  it refused before any launch.
- **The tested bytes.** The trials read Grove's sources from the candidate
  keg, not from the installed keg. An unchanged candidate would pass its
  trials, and gate 4 would admit a changed source that no trial read.

`staging_trial.py rerun` repairs both. It reads the source and its installed
digest from the variables of gate 4. It refuses unless the candidate keg's
record gives that source the same digest. The refusal names the acceptance
work: rebuild the candidate keg, rerun the trials, revise the accepted values
and repeat the dry run. An unchanged candidate therefore stops gate 4 in
native delivery, as step 2 of
[The switch inside finish](specs/owner-skill-delivery.md#the-switch-inside-finish)
requires.

`rerun` sets HOME to the account's home from the account record.
Each launch still gets HOME and `CODEX_HOME` of the staging HOME. Each trial
writes a new folder `rerun-<source>-<trial>[-<harness>]-<UTC time>` in the
trials folder.

Each command of the plan is `$python -I <runner> rerun` with these
arguments:

```text
--home /Users/antony/.local/state/owner-staging/home
--candidate /Users/antony/.local/state/owner-staging/candidate
--ravel /Users/antony/.local/state/owner-staging/ravel-2.0
--trials /Users/antony/.local/state/owner-staging/trials
--trial <trial> [--harness <harness>]
```

The runner is
`/Users/antony/Development/AgenticOrchestration/grove.gh-issue-25/docs/evaluations/owner-skill-delivery/staging_trial.py`,
in this workspace, which the finish sequence keeps. Before finish, the main
checkout holds an earlier runner without `rerun`. Its content would change
when finish moves `main`, and the dry-run comparison would then report a
changed input. owner-switch-plan-k254 can choose another path whose content
stays the same through the switch.

The command name is `<trial>-<harness>`, or `sources` for the source trials.
The Claude Code compaction trial adds `--model claude-opus-5-5 --effort high`,
as [Rerun commands](#rerun-commands) states.

| Source | Trials of `accepted.trials` | What each trial reads of the source |
| --- | --- | --- |
| `grove-core` | `sources`, `kind-skill` on `claude` and `codex`, `stale-copy` on `claude` and `codex` | `grove-planning` and a spine resource. Ravel's copy of `grove` beside the stale copy. |
| `requirements` | `sources` | The resolution, the missing-source refusal and the restored digest |
| `design` | `sources`, `discovery` on `claude` and `codex` | `cli-tool-design`, found without its name |
| `engineering` | `sources`, `delegate`, `compaction` and `codebase-memory-mcp`, each on `claude` and `codex` | `coding-style-bash` in a delegate and after a compaction. `using-codebase-memory` with its server. |
| `research` | `sources` | As for `requirements` |
| `writing` | `sources`, `native-residue` on `claude` | The technical-communication hooks, which run from the source in place |

`sources` moves the named source itself away. No model case reads a skill of
`requirements` or `research`, so their trials launch no model.

`guidance-probe` is no trial of a source. It installs the native
`grove@linkuistics` plugin from GitHub, and it reads no Ravel snapshot of
`grove-core`. A change to Claude Code, not to a source, calls for its rerun.

Gate 4 reads only Grove's six sources. A change to another source is an
owner action outside the switch: the owner edits its formula after its
staging trials, as
[Skill formulae](specs/owner-skill-delivery.md#skill-formulae) states. The
staging settings name the versioned keg path of each of these sources, so a
new version also needs new staging settings. Their trials run with
`task owner:staging:case`, as in [Rerun commands](#rerun-commands):

| Source | Cases |
| --- | --- |
| `personal` | `user-invoked` and `user-invoked-named` on `claude` and `codex`, `native-residue` on `claude`, and `task owner:staging:inventory` |
| `testanyware` | `testanyware-vm` on `claude` and `codex` |
| `superpowers` | `discovery` on `claude` and `codex`, which reads the mandatory file first |
| `quint` | `quint-mcp` on `claude` and `codex` |

#### Controls of `rerun`

| Control | Result |
| --- | --- |
| `task owner:staging:trial-tests` | Passed 110 tests. Seven are new, in the class `Rerun`. |
| Mutant without the digest refusal | Failed `test_a_candidate_with_another_digest_refuses_with_the_acceptance_work` only |
| Mutant that keeps HOME | Failed `test_a_conforming_rerun_names_a_new_folder_and_the_real_home` only |
| Mutant without the check that the account's home contains the staging HOME | Failed `test_an_account_home_that_does_not_contain_the_staging_home_refuses` only |
| `rerun --trial sources` for `research` with a wrong digest, in gate 4's environment | Exit 1 before any launch, with the acceptance work in the message |
| The same command with the accepted digest | Passed. Record `trials/rerun-research-sources-20261110T094630Z/record.json`, SHA-256 `095c56d0f6d819ad57a5aa7f421d97189d1abf991654ada1969e0ee14ab204ae`. |

Gate 4's environment for both runs was HOME and the working directory at the
staging HOME and `CODEX_HOME` at its `.codex` tree. It held no `RAVEL_*`,
Grove or Claude Code location variable. The passing run refused the moved
`research` as `skill_source_missing` and gave the accepted digest after the
return. Its real-HOME check found no change.

### The trials of the cutover contract

The paths of the records are relative to
`/Users/antony/.local/state/owner-staging`. Codex was 0.167.0 in each run.

| Trial | Claude Code | Codex | Result |
| --- | --- | --- | --- |
| Isolation | `isolation-2/record.json`, 2.1.319 | The same record | Passed. [Staging setup and isolation trial](#staging-setup-and-isolation-trial) |
| Mandatory read and implicit craft discovery | `trials/discovery-claude-1`, 2.1.319 | `trials/discovery-codex-1` | Passed. [The discovery case](#the-discovery-case) |
| A Grove kind skill that the task names, with a relative resource | `trials/kind-skill-claude-4`, 2.1.322, on the renewed candidate. Earlier `trials/kind-skill-claude-3`, 2.1.319 | `trials/kind-skill-codex-4`, on the renewed candidate. Earlier `trials/kind-skill-codex-2` | Passed. [The kind skill case](#the-kind-skill-case) and [The reruns](#the-reruns) |
| Delegate | `trials/delegate-claude-1`, 2.1.319 | `trials/delegate-codex-1` | Passed. [The delegate case](#the-delegate-case) |
| Compaction | `trials/compaction-claude-opus-1` and `-2`, 2.1.319 | `trials/compaction-codex-1` | Passed. [The compaction case](#the-compaction-case) |
| Stale-copy coexistence | `trials/stale-copy-claude-2`, 2.1.322, on the renewed candidate. Earlier `trials/stale-copy-claude-1`, 2.1.319 | `trials/stale-copy-codex-2`, on the renewed candidate. Earlier `trials/stale-copy-codex-1` | Passed. [The stale-copy case](#the-stale-copy-case) and [The reruns](#the-reruns) |
| Off value, offline launch, missing source | No harness: `trials/sources-3`, on the renewed candidate. Earlier `trials/sources-2` | No harness | Passed. [The affected staging trials](#the-affected-staging-trials) and [The reruns](#the-reruns) |
| Guidance of a disabled plugin and a nested probe | `trials/guidance-claude-1`, 2.1.319 | Not applicable | Passed. [The guidance probe case](#the-guidance-probe-case) |
| TestAnyware command reference and real VM | `trials/testanyware-vm-claude-1`, 2.1.322 | `trials/testanyware-vm-codex-2` | Passed. [TestAnyware in staging](#testanyware-in-staging) |
| Quint MCP server | `trials/quint-mcp-claude-1`, 2.1.322 | `trials/quint-mcp-codex-2` | Passed. [MCP servers in staging](#mcp-servers-in-staging) |
| codebase-memory MCP server | `trials/codebase-memory-mcp-claude-1`, 2.1.322 | `trials/codebase-memory-mcp-codex-2` | Passed. [MCP servers in staging](#mcp-servers-in-staging) |
| Claude Code hooks and `/guardrail` | `trials/native-residue-claude-1`, 2.1.322 | Not applicable | Passed. [The native residue case](#the-native-residue-case) |
| No superseded skill in a native listing | Each case above | Each case above | Passed. Each verdict reads the native listing. |

Four records have a scope decision or a stated scope:

- The guidance probe runs only on Claude Code. The cutover contract names
  one Claude Code trial, because finish runs on Claude Code.
- The hooks and `/guardrail` run only on Claude Code.
  [Native residue](specs/owner-skill-delivery.md#native-residue) keeps Codex
  without technical-communication hooks, and `guardrail` exists only for
  Claude Code.
- The Codex `quint-kb` case first failed, because Codex denied each call
  under the approval policy `never`. The owner chose the server-wide
  approval for the real entry.
  [The real quint-kb approval](#the-real-quint-kb-approval) records the
  decision and a passing real run.
- The Claude Code compaction case passes with `claude-opus-5-5` at high
  effort, the setting of an `impl` session. With `claude-sonnet-5-5` at low
  effort, it failed in five of five runs.
  [The failures at low effort](#the-failures-at-low-effort) gives the
  diagnosis. No owner route uses that setting.

The user-invoked cases also passed. They are no trial of the contract.
[The user-invoked skill](#the-user-invoked-skill) records them.

### The staging items of "Test seams"

| Item of [Test seams](specs/owner-skill-delivery.md#test-seams) | Result |
| --- | --- |
| The complete switch beside an idle, signed-in Claude Code parent with the owner's MCP servers | Passed in `trials/quiet-interval-1`, record SHA-256 `33c8e747…`, with Claude Code 2.1.322. No unsolicited write, changed loaded registration or orphan marker occurred. [Quiet interval in staging](#quiet-interval-in-staging) |
| Off settings resolve as `disabled` | Passed in `trials/sources-2` and in the `rerun` control above |
| Switched settings resolve exactly ten sources | Passed in `trials/quiet-interval-1`, where the switched settings, with no `sources`, resolved the ten accepted digests from the staging prefix |
| An eleventh source or a one-byte digest change stops acceptance | Fixture controls of `task owner:switch:rehearse`, such as `c_eleventh_source`. The dry run of owner-switch-rehearsal-k255 runs that suite again. |
| A changed released Grove source requires its staging reruns, and a missing staging HOME stops them | Fixture controls of the same suite for gate 4. The `rerun` entry now makes the real reruns possible. |
| Trial commands see the isolated HOME and Codex tree | Repaired by `rerun`, and controlled above |
| Isolation, mandatory and implicit reads, relative resources, delegates, compaction, stale copies, missing source, offline resolution, hooks, native guardrail, TestAnyware, Quint and codebase-memory | The table of the cutover contract above |
| Inventory with the repaired instrument | Passed in `trials/inventory-3`, record SHA-256 `d3d6f54a…`, `inventory.py` SHA-256 `880fe562…`. 202 rows and no anomaly. `guardrail@personal` was the classified native residue. [The loaded guardrail directory](#the-loaded-guardrail-directory) |

The other items of "Test seams" use fixtures or the real HOME. The fixture
suites of the switch, the dry run of owner-switch-rehearsal-k255 and the
probes inside finish own them.

### Dispositions of "Not measured"

Each item of [Not measured](#not-measured) now has a measurement, an owner
or a reason that a later decision withdrew it.

| Item | Disposition |
| --- | --- |
| A root-level `SKILL.md` in a skill-free plugin | Measured by staging-claude-native-trial-k285 with Claude Code 2.1.322 |
| Disabling a plugin under a running Claude Code session | Measured by staging-guidance-probe-k278 with Claude Code 2.1.319 |
| Codex sign-in from a separate `CODEX_HOME`, with a token refresh | Measured by owner-staging-setup-k248 |
| The tree that Codex reads for an enabled bundled plugin | No decision reads it. Bundled plugins are platform-managed and unchanged, the switch writes no Codex plugin, and the inventory reads both trees. The staging Codex sessions listed no superseded skill. |
| Snapshot reads of Claude Code under the owner's launch shape | Measured below |
| One skill name in both Codex user roots | No switched state holds it. The switch deactivates both Codex user roots' copies together, and the inventory refuses an extra entry there. Native delivery and restore keep the owner's present roots, which the owner uses today, unchanged. |
| Pi's startup with a dangling extension link | Outside the automatic contract. The switch deactivates the link and gives Pi no replacement, as [Classes and dispositions](specs/owner-skill-delivery.md#classes-and-dispositions) states. |
| Built-in Claude Code skills on disk | Owned by step 3 of the switch inside finish, which compares each unverified built-in row with finish's own skill listing |
| `mise bootstrap repos apply` against the real remotes | Withdrawn by the Homebrew change. No repository entry remains. |
| A decoy credential helper against a remote that asks for a credential | Withdrawn by the Homebrew change. Homebrew downloads public archives. |
| The size and capture time of a TestAnyware source | Measured by staging-testanyware-trial-k287 |
| Ravel's capture of a pack checkout that mise wrote | Withdrawn by the Homebrew change. The staging trials measured the capture of keg sources instead. |
| Ravel 1.1 and 2.0 with a settings file that holds no key | Withdrawn by the simplified switch. The inverse no longer exists. Restore returns the original settings with only the off-value exception, and 1.1 reads no settings that finish writes. |
| An exchange and an exclusive rename across the switch set and the backup | Withdrawn by the simplified switch, which deleted exchange and held-link detection |
| A complete `mise bootstrap` with repository entries | Withdrawn by the Homebrew change for repository entries. owner-pack-checkouts-k93 owns the complete `mise bootstrap` that installs the formulae in the real HOME. |
| An adoption with repository entries on a new machine | Withdrawn by the Homebrew change for repository entries. The new-machine runbook of the maintained manual owns adoption. |

#### Snapshot reads in `auto` mode

One staging Claude Code session measured the reads at 09:39Z on
2026-11-10. It ran through `ravel start-session` of the accepted 2.0 build
with Claude Code 2.1.322 and `claude-sonnet-5-5` at low effort. Its route
passed `-p`, the stream flags, the model and the effort only. That is the
owner's launch shape, without a permission flag and without `--add-dir`.
The staging settings select `defaultMode: "auto"`, as the owner's settings
do.

The init event named the permission mode `auto` and a working directory
under `/private/tmp`. The agent read the mandatory file and
`cli-tool-design/SKILL.md` from the staging Ravel cache with the Read tool.
It listed the snapshots once with Bash. Each tool returned content, the
result's `permission_denials` was empty, and the session ended with
`read-ok`. The record is `trials/auto-mode-read-claude-1`.

The run measured `-p` mode. An interactive session in `auto` mode uses the
same classifier, but no run measured it.

### Limits

- Most model trials ran once on each harness. Each shows that the behaviour
  occurs, not how often.
- The early trials ran with Claude Code 2.1.319, and the later trials and
  the quiet interval ran with 2.1.322. owner-marketplace-freeze-k253 chooses
  the held version. The quiet-interval measurement needs another run unless
  that version is 2.1.322.
- After promotion, the owner's `config.rhai` routes each `review-*` kind
  except `review-design` and `review-requirements` to `claude-sonnet-5-5`.
  It uses medium or high effort. No compaction run measured that model at
  those efforts.
- The new `rerun` entry ran one source trial end to end. No model case ran
  through it. Its only change to a case is HOME for the runner, the check
  of the candidate digest and the trial folder.
- No trial reads a skill of `requirements` or `research` by a model. Their
  trials prove resolution, refusal and digest only.
- The staging HOME does not isolate SSH, as
  [The guidance probe case](#the-guidance-probe-case) states.

## Packaging-only acceptance

`skill-formula-publication-k298` delivered this section on 2026-11-11. It
applies the packaging-only rule of
[Skill formulae](specs/owner-skill-delivery.md#skill-formulae) to the four
revised non-Grove formulae. Each record below bridges the source trials of
[Staging acceptance](#staging-acceptance) to a revised full package version.
owner-switch-plan-k254 and owner-pack-checkouts-k93 read these records. No
real keg, link or HOME changed. owner-pack-checkouts-k93 installs the
revisions.

### Result

Each revision changes only the keg layout and adds `revision 1`. Its URL,
`sha256` and upstream version are the same as in the accepted formula. Each
source has its accepted commit, roots, tree ID and `resourceDigest`. The
shared-directory seam passed every case with all five revised formula
outputs and Ravel's keg. Its failure control failed on the published
formulae, as required. Each formula is therefore accepted at its revised
full package version.

No record claims acceptance from equal source digests alone. Each record
names the seam run, and the record of `personal` names the `guardrail`
readings too.

### The seam run

`task release:formula:test` ran at 13:03Z on 2026-11-10 with `TAP` set to
the tap working copy. It passed each earlier step, tap states A to D
included, and then the 10 cases of the seam. The record is
`target/shared-directory-seam/20261110T130355Z.json`, SHA-256
`6ec2ddc291da025a…`. The instrument digested each subject before and after
the run, and no subject changed.

| Subject | Value |
| --- | --- |
| Grove | The native archive `5f6093a6e6bb…` of the working copy, rendered as Grove 22.8.0 in the projected layout. No change after grove-keg-projection-k293 touches `plugins`, `assets`, `crates` or the formula template |
| Ravel | The lane 2.0 archive `289b4ac57ffd…`, build identity `58d479ed058f…`. Its `bin/ravel` is byte-equal to the accepted staging build `3a9a9129…`. The formula is the template of `skill-directory-delivery` at `ff14cba826b0`, SHA-256 `b5b22ba8b45c41b3…` |
| Tap formulae | The four revisions of the tap working copy, at the local commit `12b53f8c449d`. The published formulae came from `origin/main` `9ca2b5e37510`, equal to the remote's `main` |
| Homebrew | 7.0.12-221-gbc63375. Its linker, installer and cleaner have the SHA-256 values of [Shared directory seam](homebrew-skill-delivery-evidence.md#shared-directory-seam) |
| Instrument | `shared-directory-seam.rb` `a5a45c4a184c2cd9…`, `shared-directory-seam.sh` `b34521b00eaffb66…`, `skill-formulae.json` `c166fe53d671dd81…` |

Each projected case linked Grove, the four revisions and the fixtures:

- `order-forward`, `order-reverse`, `ravel-first` and `ravel-promotion`
  passed. After each link, every source matched its accepted values.
- `upgrade-cleanup-uninstall` upgraded each keg to its next revision and
  removed the old keg. Every source matched after each step.
- `mixed-layout-migration` upgraded each published direct formula to its
  revision. After the last direct keg, all 12 sources matched.
- `duplicate-fresh` and `duplicate-upgrade` refused `seam-duplicate` 2 at
  `share/ravel/skills/grove-core`. The other sources matched, and no entry
  of the refused keg stayed in the prefix.

The failure control linked the four published formulae and the two direct
fixtures. In `failure-control-forward`, `personal` 2026-11-06 expanded into
file links. In `failure-control-reverse`, `testanyware` 2.8.0 expanded.
Ravel refused each resolution with `skill_resource_invalid`.

The seam takes the accepted values of a tap formula from its own pristine
payload. That comparison cannot show equality with the staging-accepted
digest. The source comparison below supplies it.

### The source comparison

`task tap:formula:test` ran once for each revision, from 13:05Z to 13:08Z.
Each run used the same Ravel and Homebrew inputs as the seam run. Each test
installed the revision in a private prefix and checked the projected
layout. It compared the payload with a plain checkout of the accepted
commit, with Git modes. Ravel resolved the payload, and the test compared
the result with the plain checkout.

Each run then repeated the seam with the four revisions. Each passed.

A script compared each printed `resourceDigest` with the table of
[The accepted sources](#the-accepted-sources). Each digest was equal. The
same script, given the `quint` row for the `personal` digest, reported a
difference as required.

| Formula | Seam record | `resourceDigest`, equal to the accepted value |
| --- | --- | --- |
| `personal` | `20261110T130553Z.json` | `dc1f26eafe9b7db6f750c95db38a7876beb29b067d829eaf63203323b89d61b6` |
| `superpowers` | `20261110T130640Z.json` | `be1ce6e3c25064b83ba2999b37972deb7f73ce88a5f5dbbba990d88037b17230` |
| `quint-llm-kit` | `20261110T130728Z.json` | `0a8b945fbc6732d5b7b3041a966d26cdf718ca884bd5f3fb8435297d7258edef` |
| `testanyware` | `20261110T130819Z.json` | `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6` |

### The record of `personal`

| Item | Value |
| --- | --- |
| Full package version | `2026-11-06_1`. The accepted version was `2026-11-06` |
| Formula content | `Formula/personal.rb`, SHA-256 `37b7b69e0e5b2e00db8c950e51ffe3b63e8f329e16f3ac9e5ef3c3e79e5234a3` |
| Archive | `PersonalSkills` at `cddcfde901cdaf8069e2aac6f3b6bc5aba9fe925`, SHA-256 `ff55ec0adea4e899…`, the accepted archive |
| Source | `personal` from `plugins/personal`, root `craft`, tree ID `f66e0792b9507194842e5b7d44457cf12d70dd98` |
| `resourceDigest` | `dc1f26eafe9b7db6f750c95db38a7876beb29b067d829eaf63203323b89d61b6`, equal to the accepted value |
| Retained source trials | Isolation, `trials/sources-2`, the switched resolution of `trials/quiet-interval-1` and the inventory `trials/inventory-3` |
| Seam run | `20261110T130355Z.json`, and the repeat in `20261110T130553Z.json` |
| `guardrail` payload | [The `guardrail` payload](homebrew-skill-delivery-evidence.md#the-guardrail-payload): the 10 entries of `libexec/plugins`, the marketplace file and the hook declaration equal the accepted keg. The run of 13:05Z repeated the comparison over 12 entries, and they were equal |
| `guardrail` capability | [The `guardrail` capability trial](homebrew-skill-delivery-evidence.md#the-guardrail-capability-trial): the trial passed from the revised keg's `opt` path. Its no-op control failed the refusal assertion, as required |

`guardrail` lies outside the `personal` digest. Its acceptance therefore
rests on the payload comparison and the capability trial, not on the
source digest. The staging case `trials/native-residue-claude-1` loaded
`guardrail` from the earlier keg. It is not retained for the revised keg.

### The record of `superpowers`

| Item | Value |
| --- | --- |
| Full package version | `7.0.0_1`. The accepted version was `7.0.0` |
| Formula content | `Formula/superpowers.rb`, SHA-256 `169a98760750243047298f805dbf54a479b2522a727278224bd81cddcc56c272` |
| Archive | `obra/superpowers` at `bb92a77741419a4ab5f06e711a283343f1ada0c3`, tag `v7.0.0`, SHA-256 `496b576d4a4a62c0…`, the accepted archive |
| Source | `superpowers` from the repository root, root `skills`, tree ID `4ac675daa3415bc836f707c7e60237ac7b66c7d5` |
| `resourceDigest` | `be1ce6e3c25064b83ba2999b37972deb7f73ce88a5f5dbbba990d88037b17230`, equal to the accepted value |
| Retained source trials | Each harness trial of [The trials of the cutover contract](#the-trials-of-the-cutover-contract), because each reads the mandatory file `using-superpowers`. Also isolation, `trials/sources-2` and the switched resolution of `trials/quiet-interval-1` |
| Seam run | `20261110T130355Z.json`, and the repeat in `20261110T130640Z.json` |
| Hook readings | None applies. The formula installs a skill source only, with no native-only plugin and no marketplace file |

### The record of `quint-llm-kit`

| Item | Value |
| --- | --- |
| Full package version | `1.0.0_1`. The accepted version was `1.0.0` |
| Formula content | `Formula/quint-llm-kit.rb`, SHA-256 `29cf32515afef6b134fcbc88f369fa7ca5589fa00b106cf511d36eaacd57d007` |
| Archive | `quint-co/quint-llm-kit` at `cc75369f741af7d490936f82002c2d28e3b3d78d`, SHA-256 `152947a80275a1d8…`, the accepted archive |
| Source | `quint` from `quint-llm-kit-plugin`, root `skills`, tree ID `4600ff66782fa87ea4ee8791b37d6fcc86bed5a6` |
| `resourceDigest` | `0a8b945fbc6732d5b7b3041a966d26cdf718ca884bd5f3fb8435297d7258edef`, equal to the accepted value |
| Retained source trials | `trials/quint-mcp-claude-1` and `trials/quint-mcp-codex-2` of [MCP servers in staging](#mcp-servers-in-staging), with [The real quint-kb approval](#the-real-quint-kb-approval). Also isolation, `trials/sources-2` and the switched resolution of `trials/quiet-interval-1` |
| Seam run | `20261110T130355Z.json`, and the repeat in `20261110T130728Z.json` |
| Hook readings | None applies. The formula installs a skill source only, with no native-only plugin and no marketplace file |

### The record of `testanyware`

| Item | Value |
| --- | --- |
| Full package version | `2.8.0_1`. The accepted version was `2.8.0` |
| Formula content | `Formula/testanyware.rb`, SHA-256 `ad71af0f77f7896567dc9b87de2917d0c674060db158b07f5e48dfc33afbd957` |
| Archive | The TestAnyware release `v2.8.0` at `1809a49068a23d9d276913f3a90227dbefb98a82`, macOS archive SHA-256 `bda57d356222e6ef…`, the accepted archive |
| Source | `testanyware` from `plugins/testanyware`, root `skills`, tree ID `0701492a5dae89e5b967e0122a5f1085cccec291` |
| `resourceDigest` | `3e9d7929eddd8988a6bebd9433b7f343a19e673993b3c81e8a1ce9ba913b72c6`, equal to the accepted value |
| Retained source trials | `trials/testanyware-vm-claude-1` and `trials/testanyware-vm-codex-2` of [TestAnyware in staging](#testanyware-in-staging). Also isolation, `trials/sources-2` and the switched resolution of `trials/quiet-interval-1` |
| Seam run | `20261110T130355Z.json`, and the repeat in `20261110T130819Z.json` |
| Hook readings | None applies. The formula installs no native-only plugin and no marketplace file |

The executables, `pkgshare` and the Linux steps of `testanyware` are the
same as in 2.8.0. The TestAnyware trials ran those executables, so the
revision keeps their reading.

### Publication

The four records existed before the push. The tap's `main` then moved from
`9ca2b5e37510` to `12b53f8c449d` through `jj git push`, and the push
succeeded. `12b53f8c449d` holds the revisions of `personal`, `superpowers`
and `quint-llm-kit` from `c0e61c1f3309`, and that of `testanyware`.

A new clone of `https://github.com/Linkuistics/homebrew-taps.git` read
`main` at `12b53f8c449d`. Each published formula file has the SHA-256 of
its record. Homebrew's formula loader read these full package versions
from the clone:

| Formula | Published full package version |
| --- | --- |
| `personal` | `2026-11-06_1` |
| `superpowers` | `7.0.0_1` |
| `quint-llm-kit` | `1.0.0_1` |
| `testanyware` | `2.8.0_1` |

After the push, each `published` declaration of `personal`, `superpowers`,
`quint-llm-kit` and `testanyware` in `scripts/skill-formulae.json` changed
to `projected`. `task seam:shared-directory` then ran with the tap subjects,
the same Ravel inputs and the tap at `12b53f8c449d`. All 10 cases passed. The
record is `20261110T142958Z.json`.

In that run, the failure control linked only the direct fixtures
`seam-fixture` 1 and `seam-control` 1. Ravel refused each order with
`skill_resource_invalid`, as required. The fixtures therefore keep the
failure control after the publication.

### Limits

- The seam runs install methods, not `FormulaInstaller` with downloads,
  bottles and dependencies. It runs on macOS only.
- The release run reported `grove-core` at `d8eec388…`, not at the accepted
  `8d92cd58…`. skill-keg-link-layout-k292 changed `plugins/grove`.
  core-candidate-restaging-k300 owns the new core candidate. Grove's
  revised formula publishes through Grove's release inside finish, not
  here.
- Release acceptance repeats the seam with the published Ravel archives,
  after their lane is in `scripts/ravel_target_inputs.py`.

## Package identity and private payloads

`owner-switch-package-identity-k299` delivered this section on 2026-11-11.
It reconciles the switch program and its Grove-side tools with the revised
full package versions and with the projected keg layout. No real keg, link,
tap or HOME changed. owner-pack-checkouts-k93 installs the revisions.

### The version step

Step 5 of `scripts/owner_switch.py` reads mise only for the declared and
installed state of each skill formula. The full package version comes from
`linked_keg` of `brew info --json=v2 <formula>`. mise must report that keg,
with or without its revision. On 2026-11-11, Homebrew 7.0.12 gave `zstd`
the revision 1 and the linked keg `1.5.7_1`, and mise 2026.11.2 reported the
same `installed_version`. A string from mise without a revision cannot tell
revision 0 from an omitted revision, so Homebrew's linked keg decides. That
keg is also the one whose sources the default skill directory names.

Step 4 now requires each of Grove's sources to come from its private payload
`libexec/skill-sources/<id>` in the installed keg. A real directory at the
alias's place, as the earlier layout had, stops the switch.

| Control of `task owner:switch:rehearse` | Result |
| --- | --- |
| `formula-revision-earlier` and `formula-revision-earlier-mise-omits` | Homebrew links the earlier revision. Step 5 stops with "is at 2026-11-06, not its accepted version 2026-11-06_1", and the same for `testanyware` when mise omits the revision |
| `formula-revision-later` and `formula-revision-later-mise-omits` | A later revision stops step 5 in the same way |
| `formula-revision-omitted-passes` | mise reports `2026-11-06`, and Homebrew links `2026-11-06_1`. The switch completes, and step 5 admits `2026-11-06_1` |
| `formula-missing`, `formula-undeclared` and `formula-unlinked` | Each stops step 5 with its own reason, which no version difference gives |
| `formula-mise-disagrees` | mise reports a version that differs from the linked keg. Step 5 stops |
| `grove-source-outside-payload` | Step 4 stops |
| `dry-run-declared-prefix` | Each source row of the dry run names the private payload, also after the declared Grove keg substitution |
| `dry-run-source-outside-payload` | The same bytes at the alias's place give an unknown substitution |

Three mutations of the program each failed their controls. The origin path
of step 4 at the alias failed `grove-source-outside-payload`. The earlier
comparison of mise's string failed `formula-revision-omitted-passes`. A
comparison that ignores the revision failed the four revision controls. The
restored program was byte-equal to the tested one.

### The fixture kegs

The rehearsal, the dry run, the quiet-interval prefix, the settings drafts
and the inventory controls build their fixture kegs in the projected layout.
Each source is a private payload with a relative alias, and each prefix entry
links the alias, as `brew link` does. The four other formulae carry their
published full package versions. Both fixture marketplace files name the
private writing payload.

The prefix step of `quiet_interval.py` links each keg's alias. It stops
unless each keg has its accepted full package version and each alias names
its payload. Its `brew` stub answers `info --json=v2` for each skill
formula, because step 5 now asks Homebrew. On 2026-11-11 the real prefix
step stopped, as required. The staging candidate predates the projection,
and core-candidate-restaging-k300 rebuilds it. The installed kegs have the
earlier versions, and owner-pack-checkouts-k93 upgrades them.

`settings_drafts.py` checks that each source of the fixture resolutions
comes from its private payload. Its new control replaces the default entry
of `personal` with a real directory and must report that origin. The
declarations line reports each tap formula's full package version. On
2026-11-11 Homebrew's local tap clone still gave the unrevised versions,
because k93 runs the tap update.

### Hook registrations

Each tool registers or compares the path that
[the hook adapters on the projected keg](homebrew-skill-delivery-evidence.md#the-hook-adapters-on-the-projected-keg)
measured:

- The switch registers `writing-hooks` from `$(brew --prefix)/opt/grove`.
- `probe_lifecycle.py` now registers it from the candidate prefix's
  `opt/grove`, not from the versioned keg.
- The inventory classifies the hook adapters as native residue. Claude Code
  must register `writing-hooks` from `/opt/homebrew/opt/grove`. Codex must
  name the canonical keg that `/opt/homebrew/opt/grove` names now. Both
  marketplace files must name the private payload, and neither plugin may
  hold a skill. A Pi link through the writing alias must resolve into the
  payload. Each other state says UNCLASSIFIED. A Codex registration of a
  removed keg names the documented repair in its note.

`task owner:inventory:controls` passed 51 controls. The unchanged fixture
holds Grove's projected keg, both registrations and the Pi link, and it has
no anomaly row. A copy of the inventory whose hook checks accept every state
failed the six new negative controls. The real inventory equals the output
of the earlier instrument row for row, because the real HOME has no hook
adapter yet.

### The plan draft

`task owner:switch-plan:draft` wrote the full package versions. The draft
stops unless the table of [Publication](#publication) names each version,
and a mutated version stopped it. The redraft also renewed the program and
inventory digests, two Claude Code registry identities and the held Claude
Code version 2.1.322 from the real HOME. `task owner:switch-plan:check`
passed. Its outputs replace `switch-plan-check.json`,
`switch-plan-report.txt` and `switch-plan-processes.txt`, which now name
the stored draft's digest `09581cd77b87…` and the program digest
`029f81af4c26…`.

### Remaining work

A changed program, helper or input needs another dry run.
owner-switch-rehearsal-k255 runs it after owner-switch-plan-k254. The
probe registration change takes effect at the next real probe session.

## Core candidate restaging

`core-candidate-restaging-k300` delivered this section on 2026-11-11. Its
times are UTC on 2026-11-10. skill-keg-link-layout-k292 changed
`plugins/grove/README.md`, so the `grove-core` digest differed from
[Staging acceptance](#staging-acceptance). This leaf built a new candidate
keg and ran each staging trial that reads `grove-core` again.

### Result

The new candidate keg holds the projected layout. Its five pack digests
equal the accepted values. Its `grove-core` digest
`d8eec38844f5e8ffd4e998ef34f3456a984846166b2e34cc2c8e65189b2f145e` is the
new accepted value. Each staging trial that reads `grove-core` passed on the
new keg. No other source changed, so each other trial stays valid.

Three readings need attention, and none of them concerns the core:

- A real-HOME write by Claude Code's marketplace update failed one Codex
  kind-skill run. The run after it passed.
- Ravel 2.0 ended one passing stale-copy run with `supervision_failed`.
  `ravel-stop-eperm-k302` owns the diagnosis.
- The probe control `codex-native-bare` failed on `listing` only. The
  probe's `kindSkill` observation accepted a copy that Codex found by a
  file search.

### The candidate

The jj revset `(gh-issue-25..@-) & (files("plugins/grove") |
files("plugins/packs.json"))` selected `e287b47462b8` and `fb8d99c0f9bd`.
The later commit is `fb8d99c0f9bd9fd84276c4539ab11ce841bf9ced`, the commit
of skill-keg-link-layout-k292. Its `plugins/grove` tree ID is
`961fddd04bb7b271e9896d07dc0225348fa8c378`, and its `plugins/packs.json`
blob ID is `2cebaf5f062d08b50b22d058bd8c4db3d412f8a6`.

`task owner:candidate-keg` refused the keg of `fb8d99c0f9bd`. The keg's
`share/ravel/skills/design` was a real folder, not a link to its private
source. That commit precedes grove-keg-projection-k293, so its own formula
template installs the earlier layout. The log is
`~/.local/state/owner-staging/candidate-build-k300-fb8d99c0.log`. This
leaf removed the refused output.

The specification defines the candidate keg as the output of the production
formula's install method. The template of k293 is the production template.
The candidate is therefore `2a24d5e61f29070bebe1fb005fcd74fc8f2b4ca3`, the
commit of owner-switch-package-identity-k299. Its `plugins/grove` tree ID
and its pack-record blob ID equal those of `fb8d99c0f9bd`. The sentence of
[Skill formulae](specs/owner-skill-delivery.md#skill-formulae) that defines
the candidate now states both conditions.

jj moved the bookmark `gh-issue-25` from `ef8a79dd7621` to `2a24d5e61f29`
and pushed it. `git ls-remote origin refs/heads/gh-issue-25` then printed
`2a24d5e61f29070bebe1fb005fcd74fc8f2b4ca3`. The pushed line holds
`5163a5784f0c`, a commit without a description. That commit adds
`.claude/settings.json` with one permission for `gh repo create`. A new
description would rewrite the candidate and the subject commits that two
review records cite. The push therefore used `--allow-empty-description`.

### The candidate keg

The earlier keg moved to
`/Users/antony/.local/state/owner-staging/candidate-8d92cd58`.
`task owner:candidate-keg` then built the new keg at the retained path
`/Users/antony/.local/state/owner-staging/candidate`. The script reported
that `origin/gh-issue-25` holds the commit. The log is
`~/.local/state/owner-staging/candidate-build-k300.log`.

| Subject | Value |
| --- | --- |
| Commit | `2a24d5e61f29070bebe1fb005fcd74fc8f2b4ca3` |
| Archive SHA-256 | `9ab7d03193109f7bdebf1aeb6f2429a592e5bc056f447499cd72ac7a4bc680b5` |
| `candidate.json` SHA-256 | `a779805c0f1102f6662dc673c61161625475524190f2523cb21823b17e84013c` |
| `digests.tsv` SHA-256 | `fbbe1a21917b5dbc989bceab0ac0648e395c6910734012eedc7fc659e1f4e193` |
| Ravel that resolved the keg | The accepted 2.0 build, `bin/ravel` SHA-256 `3a9a9129…` |
| Layout | Each `share/ravel/skills/<id>` is a relative link to `../../../libexec/skill-sources/<id>`. Each `originPath` is the private source. |

The script compared each source with a plain checkout of the same commit.
Each digest and each list of roots was equal.

| Source | Earlier `resourceDigest` | New `resourceDigest` |
| --- | --- | --- |
| `grove-core` | `8d92cd58fca8f21a87361a9d956af7e5a38e88751e25cbaff5e2384554806b51` | `d8eec38844f5e8ffd4e998ef34f3456a984846166b2e34cc2c8e65189b2f145e` |
| `requirements` | `07b2d25e8bd20259c71f7419092329368d8ae48bec6ca973343a601a8ec515c8` | The same |
| `design` | `d8ad3a2af1b5029bc0de93747f4d7297038651180f6877fbd738d94fcbf6aa14` | The same |
| `engineering` | `7e70b280e7a932489a048d108d4dae2bc63b387cf8fc6b37298663336e0fcc66` | The same |
| `research` | `7fb3d473f66feafb49afd8846fd94cf3e1fb7b01c584d4ad5f0e54814a8b3fdb` | The same |
| `writing` | `56bcd07b87f46a7b9ff92eed5a5a128f10d3ce64be2f7d934be898e058782212` | The same |

The release run of [Publication](#publication) reported the same new
`grove-core` digest.

### The recorded digest

The `grove-core` row of
[the moves record](repository-moves-record.md#the-ten-ravel-sources) and
its table "The Grove candidate" name the new commit, tree ID and digest.
The plan draft received the new digest in `accepted.sources`, and nothing
else in the draft changed. `accepted_sources()` of `switch_plan_draft.py`
read the moves record and returned a value equal to the draft's. A copy of
the draft with the earlier digest compared unequal. The revised draft has
the SHA-256 `07b76929041bf2f52a3c55d4213c9bf94a31ddb64236aa0486e065706c3f036c`.

### The reruns

One script ran the first six reruns one at a time, from 15:25:30Z to
15:37:42Z. The log is `~/.local/state/owner-staging/trials/k300-reruns.log`.
The script took the SHA-256 of each subject before and after the runs. These
were the staging runner, `probe_lifecycle.py`, `harness_trial.py`,
`owner_switch_probes.py`, `owner_switch.py`, the plan draft, the staging
settings, `candidate.json`, `digests.tsv`, the Ravel build and the case
folder. Each digest was the same after the runs. The three later runs
recorded the digests of the draft, `candidate.json` and the settings, which
were also unchanged.

| Trial | Record | Harness | Result |
| --- | --- | --- | --- |
| Source trials, with `grove-core` moved | `trials/sources-3`, SHA-256 `a65db93c…` | None | Passed. The off value resolved as `disabled`. The offline launch gave the same view as the online launch. With `grove-core` moved away, Ravel exited 3 with `skill_source_missing`. After the return, the launch view held the snapshot `d8eec388…`. |
| Kind skill | `trials/kind-skill-claude-4`, SHA-256 `f10b0cd5…` | Claude Code 2.1.322, `claude-sonnet-5-5`, low | Passed. The mandatory read was event 1. `grove-planning` from the snapshot `d8eec388…` was event 5, and `grove/references/bootstrap.md` was event 9. |
| Kind skill | `trials/kind-skill-codex-3` | Codex 0.167.0, `gpt-6.1-sol`, low | Failed on the real-HOME check only. Its kind verdict passed. |
| Kind skill | `trials/kind-skill-codex-4`, SHA-256 `6f26f7fb…` | Codex 0.167.0, `gpt-6.1-sol`, low | Passed. The mandatory read was event 2. `grove-planning` was event 9, and spine references were event 12. |
| Stale copy | `trials/stale-copy-claude-2`, SHA-256 `a0919491…` | Claude Code 2.1.322, `claude-sonnet-5-5`, low | Passed. The planted copy had the pinned tree digest `d9ae18c2…`. The agent read Ravel's copy of `grove` at events 10, 12 and 13, and no file of the planted copy. |
| Stale copy | `trials/stale-copy-codex-2`, SHA-256 `a2d27…` | Codex 0.167.0, `gpt-6.1-sol`, low | Passed the verdict. The agent read Ravel's copy at six events and no file of the planted copy. Ravel exited 5, as the next section states. |
| Session probes, eight cases | `trials/probe-sessions-3/sessions.json`, SHA-256 `236d47ac…` | Claude Code 2.1.322 with `claude-opus-5-5`, Codex 0.167.0 with `gpt-6.1-sol`, medium | Seven cases gave their wanted result. `claude-ravel` and `codex-ravel` quoted `grove-planning` from the snapshot `d8eec388…`, and each request view gave the ten accepted digests. `codex-native-bare` exited 1 as wanted, but with another failed set. |
| Session probe `codex-native-bare` | `trials/probe-sessions-4/sessions.json`, SHA-256 `befab768…` | Codex 0.167.0, `gpt-6.1-sol`, medium | The same result as in `probe-sessions-3` |
| Probe resolve, five cases | `trials/probe-resolve-3/resolve.json`, SHA-256 `6a391a38…` | None | Each case gave its wanted result. The `switched` case resolved the ten accepted digests of the revised draft. |

A control ran the probe resolve once with the earlier core digest in the
draft. The `switched` case then failed, because `grove-core` gave
`d8eec388…` and not `8d92cd58…`. The draft was then restored, and its
SHA-256 was again `07b76929…`.

The session probes and the probe resolve are not in the `accepted.trials`
of `grove-core`. They read the core snapshot and the draft's accepted
digests, so this leaf ran them too.

### Three readings

#### The real-HOME write

The real `~/.claude/plugins/known_marketplaces.json`
changed at 15:27:16Z, during `kind-skill-codex-3`. Its `lastUpdated` values
for `claude-plugins-official` and `linkuistics` are 15:27:11Z and 15:27:16Z.
That is Claude Code's automatic marketplace update in the real HOME. The
staging Codex session had the staging HOME, and no event named the real
file. owner-marketplace-freeze-k253 freezes these updates.

#### The stop of Ravel 2.0

In `stale-copy-codex-2`, the agent ran
`ravel end-session staging-ok`. Ravel then exited 5 with
`supervision_failed`. Both stop signals to the process group `-32130`
returned `EPERM`, and Ravel then reaped the group. No other staging record
holds this error. The verdict reads the agent's events, so it passed.

A real Grove session that ends this way stops the Grove loop.
`ravel-stop-eperm-k302` owns the diagnosis before the release cut.

#### The bare native probe

The `codex-native-bare` case expects both
`listing` and `kindSkill` to fail. In both runs, Codex searched the probe
HOME for `grove-planning`. It read the copy in Claude Code's plugin cache
`.claude/plugins/cache/linkuistics/grove/238c3a839ab4`. The probe counts
that copy as native, so `kindSkill` passed. The seeding of
[Quiet interval in staging](#quiet-interval-in-staging) registered
`grove@linkuistics` in the staging HOME after `probe-sessions-2` ran.

The real switched state keeps that registration too. The probe still fails
on `listing`, so no probe verdict changes. owner-switch-rehearsal-k255
receives this limit.

### Trials that stay valid

Each trial below reads no `grove-core` content and compares no `grove-core`
digest. The other five Grove sources and the four tap sources did not
change.

| Trial | Reason |
| --- | --- |
| Isolation, `isolation-2` | It measures settings isolation and sign-in. The staging settings and the keg path did not change. |
| Discovery on both harnesses | The verdicts read the mandatory file of `superpowers` and `cli-tool-design` of `design` |
| Delegate and compaction | The verdicts read `coding-style-bash` of `engineering` |
| Guidance probe | It reads the native `grove@linkuistics` plugin from GitHub, and no Ravel snapshot |
| TestAnyware, Quint, codebase-memory, native residue and the user-invoked cases | They read the other sources |
| Inventory, `inventory-3` | It reads the staging HOME, which holds no source |
| Quiet interval, `quiet-interval-1` | It measures writes during the switch. Its switched resolution gave the earlier core digest, and no verdict reads source content. `sources-3` and `probe-resolve-3` resolved the ten sources with the new digest. |
| Snapshot reads in `auto` mode | The agent read `cli-tool-design` of `design` |
| The hook seam and the `guardrail` trial of hook-adapter-keg-seam-k297 | They read `writing` and `personal` |

### Limits

- `task owner:switch-plan:check` refused the revised draft. The real
  `.claude/plugins/known_marketplaces.json` changed after the draft. The
  stored `switch-plan-check.json` and `switch-plan-report.txt` therefore
  still name the earlier draft digest `09581cd77b87…`.
  owner-switch-plan-k254 writes the draft again from the real HOME.
- Each rerun ran once on each harness, except the Codex kind-skill case.
- After the Ravel cut, the published archives must give these digests
  again, as the finish handoff states.
- Do not change `plugins/grove` or `plugins/packs.json` without a new
  candidate and another rerun.

## Installed skill formulae

`owner-pack-checkouts-k93` delivered this section on 2026-11-11. It ran the
real-HOME preparation of cutover order 6 for the skill formulae. Unlike the
earlier sections, it changed the real machine. It changed the mise
declarations, four kegs and their links, and the Claude Code plugin
registry. It also changed `mac-check` and the maintained notes. Native
discovery stayed the same, and no Ravel settings file exists.

### Result

- `~/.config/mise/config.toml` declares the five skill formulae of
  [Skill formulae](specs/owner-skill-delivery.md#skill-formulae). No
  `[bootstrap.repos]` table exists in it or in `config.local.toml`.
- Homebrew links `personal` 2026-11-06_1, `superpowers` 7.0.0_1,
  `quint-llm-kit` 1.0.0_1 and `testanyware` 2.8.0_1. These are the full
  package versions of [Packaging-only acceptance](#packaging-only-acceptance).
- Ravel 2.0 resolved the shared default directory of `/opt/homebrew`. Each
  of the four sources gave its accepted ID and `resourceDigest`. Each
  `originPath` is the private payload of the linked keg.
- Claude Code registers the marketplace `personal` from
  `/opt/homebrew/opt/personal`. It holds `guardrail@personal` installed and
  disabled.
- `mac-check` has the area "Skill formulae". It compares full installed and
  tap package versions, and each source's layout.
- Grove 22.8.0 stays installed. Its keg holds no source until Grove's
  release, inside finish.

### Declarations

`mise -C "$HOME" bootstrap packages use -g --no-install` added `personal`,
`superpowers` and `quint-llm-kit` with the version `"latest"`. The entries
moved beside the existing `ravel`, `grove` and `testanyware` entries. The
change is exactly the three lines of `docs/owner-cutover/bootstrap-packages.toml`
that the table lacked. The mise history watcher saved it.
`mise bootstrap plan --detailed-exitcode` then reported no pending change.

### Homebrew 7.0.13

`brew update` before the advance moved Homebrew from 7.0.12-221-gbc63375 to
7.0.13-45-g95d4938, revision `95d4938386e5`. The
[shared-directory seam](specs/codex-skill-provisioning.md#shared-default-directory-acceptance)
requires its failure control and both installation orders before a new
Homebrew version is accepted.

The seven files that the seam digests have the SHA-256 values of
[Shared directory seam](homebrew-skill-delivery-evidence.md#shared-directory-seam).
`git diff bc63375ac3a4 HEAD` shows no change to them, to `upgrade.rb` or to
`keg_relocate.rb`.

`task seam:shared-directory` then ran with the tap subjects, the tap at
`12b53f8c449d` and the lane 2.0 Ravel archive `289b4ac57ffd…`. It passed all
10 cases, both orders and `ravel-first` included. The failure control
expanded `seam-fixture` 1 and `seam-control` 1, and Ravel refused each with
`skill_resource_invalid`, as required. No measured subject changed. The
record is `target/shared-directory-seam/20261110T160917Z.json`, SHA-256
`a55eaf7fc781fbde…`.

### The advance

The advance ran this command once:

```sh
HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_INSTALL_CLEANUP=1 \
  HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=1 \
  brew upgrade linkuistics/taps/personal linkuistics/taps/superpowers \
  linkuistics/taps/quint-llm-kit linkuistics/taps/testanyware
```

No formula has a dependency, and Homebrew reported no warning other than
the suppressed dependents check. The only Grove loop was this session's.
Its Ravel 1.1.0 driver captures no keg source. No earlier keg was removed,
so the four pre-revision kegs stay in the Cellar.

Each formula file of the local tap equals its acceptance record by SHA-256.
Each keg's receipt names tap `12b53f8c449d`. The archive URLs name the
accepted commits `cddcfde901cd`, `bb92a7774141` and `cc75369f741a`, and the
TestAnyware release `v2.8.0`.

Each entry of `/opt/homebrew/share/ravel/skills` is now one link to the
keg alias, such as
`../../../Cellar/personal/2026-11-06_1/share/ravel/skills/personal`. Each
alias is `../../../libexec/skill-sources/<id>`. Before the advance, each
entry linked the real source directory of the earlier keg.

### Verification through the default directory

`task owner:sources:verify` is new. It runs
`docs/evaluations/owner-skill-delivery/default_sources.py`. The tool
resolves the real prefix's default skill directory with Ravel 2.0, in a
disposable HOME whose settings name no sources. It compares each source
with `accepted` of the plan draft. A prefix of recorded archives has no
installation record, so the tool copies `bin/ravel` into a temporary prefix
whose record names `$(brew --prefix)/share/ravel/skills`.

The run used the lane 2.0 prefix of `scripts/ravel_target_inputs.py`, which
reports `ravel 1.1.0`, and `FORMULAE` named the four formulae:

| Source | `resourceDigest`, equal to the accepted value | `originPath` |
| --- | --- | --- |
| `personal` | `dc1f26eafe9b7db6…` | `/opt/homebrew/Cellar/personal/2026-11-06_1/libexec/skill-sources/personal` |
| `quint` | `0a8b945fbc6732d5…` | `/opt/homebrew/Cellar/quint-llm-kit/1.0.0_1/libexec/skill-sources/quint` |
| `superpowers` | `be1ce6e3c25064b8…` | `/opt/homebrew/Cellar/superpowers/7.0.0_1/libexec/skill-sources/superpowers` |
| `testanyware` | `3e9d7929eddd8988…` | `/opt/homebrew/Cellar/testanyware/2.8.0_1/libexec/skill-sources/testanyware` |

Each control failed as required:

- A plan copy with the `quint` digest for `personal` reported the digest.
- A plan copy with the accepted version `2.8.0` for `testanyware` reported
  the linked `2.8.0_1`.
- A plan copy without `quint` reported an unaccepted resolved ID.
- A copy of the tool that compared with the earlier keg's source directory
  reported the origin. A copy that compared with the keg alias passed,
  because the alias canonicalises to the payload. That mutation therefore
  distinguishes nothing.
- The default set of all ten sources reported each of Grove's six sources
  as unresolved, because Grove 22.8.0 has no keg layout.

After Grove's release, finish can run the task with the published `ravel@2`
keg and no `FORMULAE`. That keg has its own installation record.

### The `guardrail` registration

Claude Code 2.1.322 ran the documented commands of
[Hook adapters](specs/codex-skill-provisioning.md#hook-adapters) from HOME,
with `DISABLE_AUTOUPDATER=1`. `claude plugin disable guardrail@personal`
followed:

| Command | Result |
| --- | --- |
| `claude plugin marketplace add "$(brew --prefix)/opt/personal" --json` | `ok`, marketplace `personal`, declared in user settings |
| `claude plugin install guardrail@personal --json` | `ok`, scope `user` |
| `claude plugin disable guardrail@personal` | Disabled, scope `project` |

In HOME, the project settings file of Claude Code is `~/.claude/settings.json`,
so the disable wrote `"guardrail@personal": false` there.
`settings.local.json` did not change. `claude plugin list --json` reports
`enabled: false` and `readFromFolder`
`/opt/homebrew/opt/personal/libexec/plugins/guardrail`. Its cache copy is
byte-equal to that payload. Claude Code's version stayed 2.1.322.

The installation also refreshed the auto-updating `linkuistics`
marketplace, whose `lastUpdated` changed. Its clone stayed at
`238c3a839ab4`, and `installed_plugins.json` changed only by the new entry.
owner-marketplace-freeze-k253 owns that refresh route.

### `mac-check`

`~/.local/lib/machine-config/check.py` has the new area "Skill formulae".
`mac-check --only "Skill formulae"` runs it alone. It reads
`mise -C "$HOME" bootstrap packages status --json` and
`brew info --json=v2` of the five formulae. The full package version of
the tap is the stable version and its revision. As in step 5 of the switch
program, the linked keg decides when mise omits the revision. mise must
still report that keg, with or without the revision.

The area reports four classes, each with fixed text and its recovery:

| Class | Example |
| --- | --- |
| Configuration error | An undeclared, uninstalled or unlinked formula, a missing tap version, or mise and Homebrew disagreeing |
| Earlier revision | The linked keg has the tap's upstream version and a lower revision |
| Version drift | Any other difference from the tap's full package version |
| Invalid source layout | A source expanded into file links, an entry that reaches another keg, an unlinked entry, or an entry that no formula installs |

It checks the layout only at the tap's current version. A keg without the
keg layout gets one "Keg layout" message for all its missing sources. The
report compares no commit or digest. Its header states that, and that
versions compare with the tap as of the last `brew update`.

The suite in `~/.local/lib/machine-config/tests` passed 75 tests with Ruff
and strict pyright. Each of four mutations failed tests:

| Mutation | Failed tests |
| --- | --- |
| Ignore the revision | 2 |
| Skip the version comparison | 3 |
| Skip the layout check | 4 |
| Accept any existing entry | 2 |

On the real machine, the area reported one "Keg layout" message. It named
Grove 22.8.0 and its six sources. Grove's release inside finish removes it. The full `mac-check` exited 10. The other
attention items were the CCC manual review and unsaved iTerm2 preferences.

### Native discovery

`task owner:inventory` ran before the first change and after the last.
`task owner:inventory:compare` reported three differences:

| Difference | Reason |
| --- | --- |
| Added `guardrail@personal` under `~/.claude/plugins/cache/personal` | The disabled plugin |
| Added its skill `guardrail:guardrail`, disposition "outside (plugin disabled)" | The same digest `36371cf57051b6ff` as `linkuistics:guardrail` |
| Changed form of the `testanyware` command | It resolves into the `2.8.0_1` keg, with the same digest |

No harness skill root gained an entry. `~/.config/ravel` holds `policy.ts`
and `config.rhai`, and no `settings.json`.

### Remaining work and limits

- The plan draft predates this section. Its Claude Code registry
  identities, its `mac-check` input digests and its kegs need the redraft of
  owner-switch-plan-k254. The dry run of owner-switch-rehearsal-k255 then
  reads the changed `mac-check`.
- Finish installs Grove's revised formula. It then verifies all ten sources
  through the default directory with the published Ravel, before the switch
  or any default launch.
- The seam ran on Homebrew 7.0.13 with the tap formulae and fixtures, not
  with a Grove keg. The earlier runs covered Grove's projected template.
- The four pre-revision kegs stay until a cleanup. A cleanup removes no
  running driver's binaries, because no driver runs from them.

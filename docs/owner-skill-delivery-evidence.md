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
  plugin.
- The effect of disabling a plugin on a Claude Code session that is already
  running. The loading reference says that the session keeps its loaded set,
  but no trial observed it.
- Codex sign-in from a separate `CODEX_HOME` that reuses only the
  authentication file, including a token refresh.
- Which tree Codex reads for an enabled bundled plugin: the installed cache or
  the marketplace source. The baseline records both.
- Whether Claude Code under the owner policy reads snapshot paths outside its
  working directory without a permission prompt.
- Codex's handling of one skill name in both user roots.
- Pi's startup with a dangling extension link.
- Built-in Claude Code skills on disk. The baseline lists them from the
  session's skill listing.
- `mise bootstrap repos apply` against the real remotes.
- A decoy credential helper against a remote that asks for a credential.
- The size and capture time of a TestAnyware checkout subdirectory.
- Ravel's capture of a pack checkout that mise wrote.

The staging trials in the design's cutover contract own each of these
measurements before the switch.

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
- `versions`: `personal` 2026-11-06, `testanyware` 2.8.0, `superpowers`
  7.0.0 and `quint-llm-kit` 1.0.0, each a formula of `linkuistics/taps`.
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

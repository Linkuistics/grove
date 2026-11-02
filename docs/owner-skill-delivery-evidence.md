# Owner skill delivery evidence

This document records the inputs to the
[owner skill delivery design](specs/owner-skill-delivery.md). The design
session only read the machine. It changed no installation, setting, plugin
registry or file outside this checkout, and it signalled no other session.
The integration of the design's focused review repaired the instrument and
regenerated the baseline on the same day. It also only read the machine. The
section [Review corrections](#review-corrections) records what changed.

After Ravel narrowed delivery to local directories, `owner-pack-checkouts-k79`
measured the owner's checkout tool in a fixture HOME. The section
[Pack checkouts](#pack-checkouts) records that measurement.

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
| Claude Code plugin cache | The [plugin loading reference](https://code.claude.com/docs/en/plugins/loading#cleanup-of-previous-versions), read on 2026-11-02. An update or uninstall writes `.orphaned_at` into the earlier version directory, and a background cleanup removes it 14 days later. A running session keeps the plugin set that it loaded until `/reload-plugins` or a new session. |
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

The design gives the pack checkouts to the `[bootstrap.repos]` part of mise.
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

[`evaluations/owner-skill-delivery/checkouts.py`](evaluations/owner-skill-delivery/checkouts.py)
measures the behaviour that the design states. It builds a fixture origin and
a fixture HOME, and it runs the installed mise with only isolated paths in its
environment. It moves the origin away to stand in for a machine without a
network. Each mise call names the fixture HOME with `-C`. The fixture hides
the owner's global and system Git configuration. `task owner:checkouts` runs
it.

| Artifact | SHA-256 |
| --- | --- |
| `docs/evaluations/owner-skill-delivery/checkouts.py` | `349e2e233c3967f370c39174034a019cf84f2980758f4098c2827bc819cd3244` |

With mise 2026.10.25 and Git 2.56.0, all 33 cases matched. The script asserts
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

Two measurements need the real HOME or a new machine, so no staging trial can
own them:

- A complete `mise bootstrap` run with repository entries. Packages and
  launchd agents act for the whole machine or account, not for one HOME.
- An adoption with repository entries on a new machine. The new-machine
  runbook in the maintained manual owns it.

## Review corrections

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

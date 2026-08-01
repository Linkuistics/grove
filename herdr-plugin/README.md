# grove — a herdr plugin

Renders a [grove](../README.md) task tree in a [herdr](https://herdr.dev) pane: which
leaf is running, its kind, what is done, what is left.

```
grove.improve-signaling-to-herdr                    20 done · 1 pruned · 8 live
──────────────────────────────────────────────────────────────────────────────
✓ plan-k1
✓ herdr-pane-state-k2/                                       3 done · 1 pruned
✓ task-kind-taxonomy-k3/                                               10 done
▼ status-surface-live-k23/
  ✓ ship-release-k25
  ▶ observe-live-surface-k26                                             impl
▶ herdr-grove-plugin-k5                                                  impl
○ jj-first-coverage-k6                                                   impl
○ session-leaf-binding-k28                                             design
──────────────────────────────────────────────────────────────────────────────
                                     j/k scroll · g/G ends · r reload · q quit
```

| mark | meaning |
|---|---|
| `▶` | the live leaf `grove-llm pick` would return next — the one a session is on |
| `○` | a live leaf still ahead of it |
| `✓` | retired, done |
| `✗` | pruned — a path decided against |
| `▼` | a node with live work, expanded |
| `✓ …/` | a node whose whole subtree is finished, collapsed to its counts |

A leaf's kind (`impl`, `design`, `review-impl`, …) is shown on live leaves only, since
that is what says what kind of session runs next. A leaf that pins its own harness —
the research vendor pair — shows it as `research @codex`.

## Install

```bash
herdr plugin install Linkuistics/grove/herdr-plugin
```

Then bind a key in `~/.config/herdr/config.toml`:

```toml
[[keys.command]]
key = "prefix+t"
type = "plugin_action"
command = "linkuistics.grove.open-tree"          # split beside the current pane
# command = "linkuistics.grove.open-tree-overlay" # or a zoomed overlay to glance at
```

Requires `python3` on `PATH` — nothing else. No third-party packages, so there is no
build step, and `herdr plugin link <dir>` works straight from a checkout. Verified on
macOS's own Python 3.9.6 and on 3.14. macOS and Linux only: the renderer drives a raw
terminal through `termios`.

Note that `plugin link` records the **path** you linked. Linking a grove's throwaway
working tree means the link dies with that worktree; link a durable checkout, or
install from GitHub.

## How it finds the grove

From the invoking pane's cwd, walking up for the nearest `.grove/`. herdr supplies
that cwd in `HERDR_PLUGIN_CONTEXT_JSON`, so a split opened beside a `grove do` pane
renders *that* pane's grove, and stays pinned to it afterwards. Pass a path to render
a specific grove instead — useful outside herdr:

```bash
python3 grove_tree.py ~/Development/some.workstream
```

Without a TTY it prints one frame and exits, so it pipes.

Do **not** open the pane with `plugin pane open --cwd`: that replaces the plugin root
as the process cwd, and herdr then resolves the manifest's relative command against
the new cwd and fails to spawn. The cwd override buys nothing here anyway.

## What it does not do

It reports no state. herdr's `idle` / `working` / `blocked` for a `grove do` pane comes
from the `grove` binary over herdr's socket, and full lifecycle authority is a
compiled-in allowlist nothing outside herdr's binary can join — so this plugin owns UI
and only UI. See [Grove's architecture](../docs/ARCHITECTURE.md#herdr-optional-ui)
for the split and the in-session state surface.

## How it works

The only contract is Grove's published
[task-tree scheme](../docs/ARCHITECTURE.md#task-tree-scheme):

```
.grove/
  BRIEF.md                        ← the node's charter; never a task
  01-DONE-plan-k1.md              ← NN-[DONE-|ABANDONED-]<slug>-k<key>.md
  02-herdr-pane-state-k2/         ← a node is a directory: NN-<slug>-k<key>/
    BRIEF.md
    01-DONE-authority-route-k7.md
  03-jj-first-coverage-k6.md
```

Position, outcome, slug and key are all in the *filename* — that scheme exists so
`pick` can walk a tree without opening a file — so the whole shape comes from one
`scandir` per directory. The only file ever read is the `**Kind:**` line of a live
leaf. Ordering is per-level numeric, and the live leaf is the first one in depth-first
pre-order, which is exactly what `grove-llm pick` returns.

Nothing here calls `grove` or `grove-llm`, opens a socket, or writes any state: the
tree on disk already *is* the status. So the plugin and the binary version
independently, deleting the plugin changes nothing about grove, and deleting grove
leaves the plugin with nothing to render but breaks nothing.

Refresh is a 1 s poll, redrawing only when the rendered frame changes. A grove tree
changes at most once per session, and a poll costs a handful of `scandir` calls, so a
filesystem watcher would be a dependency bought for nothing.

Foreign files under `.grove/` are ignored rather than reported, matching `pick`. An
unrecognised or missing `**Kind:**` reads as `impl`, and the retired `work` spelling
still reads as `impl` — the same degrade-on-read rule grove itself applies, so a
hand-edited task file can never make the viewer complain.

## Maintaining the fork

Grove's pane-state integration is shipped from a maintained Herdr fork. Its
branch model, build constraints, release sequence, restart procedure, and
acceptance checks are in [MAINTENANCE.md](MAINTENANCE.md).

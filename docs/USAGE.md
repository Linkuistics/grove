<!-- tc: pending because the conversion has not reached this part of the document -->

# Using Grove

Grove drives a long-running workstream as a sequence of fresh agent sessions.
The workstream lives in a `.grove/` task tree inside a **Jujutsu** working tree
that you create and own. The bare lifecycle drives jj: run it in a tree with
no `.jj/` and it stops before touching anything, naming
`jj git init --colocate` — which makes an existing Git repository jj-enabled
while keeping its history and leaving every Git tool working.
`grove run` is a separate standalone invocation which needs no project or jj.

Before starting, install Grove as described in the [README](../README.md) and
install a [launch policy](#usage-launch-policy). Grove launches no session
without one.

This guide covers the whole installed command surface — the `grove` binary you
run, and the `grove-llm` verbs a session runs over the tree — plus every journey
from scaffolding a grove to tearing one down. It says what a Grove owner does
about launch policy and leaves the policy contract to
[Ravel](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md). It does **not** carry
installation ([README](../README.md)), grove's vocabulary
([CONTEXT.md](../CONTEXT.md)), or the methodology a session executes (the
`grove` plugin's own skills). Each is linked where it is reached. The set this
guide is obliged to cover is written down separately, in
[the coverage inventory](specs/user-guide-coverage.md), and the
[coverage map](#usage-coverage-map) at the end says which section answers each
row of it.

Transcripts below illustrate the current command boundary, with the working tree rewritten to
`/home/you/app`, the home directory to `/home/you` and the directories holding
programs to system paths. The launches in them were selected by the sample
policy.

<!-- tc: on -->

<a id="usage-launch-policy"></a>
## Launch policy

Grove resolves the first executable `ravel` in an absolute PATH directory. It canonicalises
the path once and checks the CLI version of that Ravel. Grove admits stable Ravel 1.x from
1.1.0 and stable 2.x from 2.0.0. The version decides admission only. The PATH of the Grove
invocation selects the Ravel, and its configuration file and settings select the route.

Grove supports three routes. On the two native routes, each harness supplies its skills
through its own native installation. On the [directory route](#usage-directory-route),
Ravel 2.0 supplies them from local source directories that you configure.

| Route | Ravel | Configuration | Settings | Grove caller assets |
| --- | --- | --- | --- | --- |
| Native on 1.1 | 1.x from 1.1.0, Homebrew's `ravel` | `~/.config/ravel/policy.ts`, schema 2 | `~/.config/ravel/settings.json` with only `timeoutMs`, `contextBytes` and `policyEnv` | The TypeScript set |
| Native on 2.0 | 2.x from 2.0.0, Homebrew's keg-only `ravel@2` | `~/.config/ravel/config.rhai`, `SCHEMA_VERSION` 4 | `~/.config/ravel/settings.json` with the keys of 2.0.0 and no `skills` | The Rhai set |
| Directory on 2.0 | As native on 2.0 | As native on 2.0 | `~/.config/ravel/settings.json` with a `skills` object that names local sources | The Rhai set |

Install Ravel independently with `brew install linkuistics/taps/ravel`. Grove and Ravel can
have different prefixes and different versions. A missing `ravel` names that install remedy.
Homebrew installs Ravel 2.x as the keg-only `ravel@2`, so its `ravel` is never the default
command. To use it, put `$(brew --prefix ravel@2)/bin` first on the PATH of the Grove
invocation. The ending commands in the prompt then name that installation.

An unsupported or malformed version names both admitted majors with their configuration
files, and the `ravel@2` directory. Grove refuses Ravel 1.0.x, prereleases and unknown majors
such as 3.x. Empty and relative PATH entries, decoys in the working directory and Grove
siblings cannot supply a fallback. Do not upgrade a shared installation that a running
session or another configuration still uses.

Both configuration files can exist under one HOME. The settings file cannot serve both
majors with every key. Ravel 1.1.0 refuses the keys of 2.0, such as `configEnv`, and
Ravel 2.0.0 refuses `policyEnv`. A settings file that both accept holds only `timeoutMs` and
`contextBytes`. While a 1.1 launch still reads the settings under a HOME, run a 2.0 route
that needs another key under a separate HOME.

On 1.x, your `policy.ts` exports a policy. On 2.x, your `config.rhai` exports
`SCHEMA_VERSION` 4 and defines `select(request, context)`. The selection uses the kind, the
prompt, an optional task file and handle, and the working directory of the caller. Write or
copy the configuration yourself. Upgrades do not edit it, and Grove converts no
configuration from one major to the other.

Ravel has exactly two commands, `start-session` and `end-session RESULT`, with the ordinary
help and version flags. It has no init, inspect or record command. Ravel 1.x supplies the
public SDK `ravel/sdk`. Ravel 2.x has no SDK, and a Rhai script imports another by its path.
Owner settings live in `~/.config/ravel/settings.json`.

For example, an owner's deterministic fixture policy for 1.x is:

```ts
export const policy = {
  schemaVersion: 2,
  version: 'owner-1',
  select: request => ({
    status: 'selected', program: '/bin/sh',
    args: ['/absolute/path/to/your-harness.sh', request.prompt],
    provider: 'fixture', model: 'none', effort: 'none', reason: 'owner route'
  })
};
```

The same route in `config.rhai` for 2.x is:

```rhai
export const SCHEMA_VERSION = 4;
export const VERSION = "owner-1";

fn select(request, context) {
    #{ status: "selected", program: "/bin/sh",
       args: ["/absolute/path/to/your-harness.sh", request.prompt],
       provider: "fixture", model: "none", effort: "none", reason: "owner route" }
}
```

Your real routes choose your harness, model, effort and approvals. The generic
[Ravel 1.x CLI contract](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md)
describes the SDK, the bounds, the environment grants and the `.ravel-choice` helper. The
adapter, the examples and the provenance conventions of Grove are caller assets, with one
set for each major. [The assets guide](../assets/ravel-policy/README.md) says how to copy
each sample. Configuration imports load them. Ravel does not embed them as modules.

The lifecycle command is:

```sh
ravel start-session --kind=KIND --prompt=MANDATE --exit-dir=LAUNCH_DIR --task-file=TASK_FILE --task-id=HANDLE
```

The values come from the one selected leaf. The exit directory is launch mechanics. Grove
captures the result separately from the managed output. Grove supplies no policy path, no
config path, no record path and no named parameter.

Scaffolding verbs and tree verbs consult no policy. An unrouted kind refuses only at its
launch. The leaf stays live, and Grove resumes it after you correct the owner's route.

<a id="usage-directory-route"></a>
### The directory route

On the directory route, Ravel 2.0.0 adds the locations of your skill sources to each
launch prompt. The agent finds and reads its skills there, so no harness needs a native
copy. Grove sends the same mandate on every route, and it reads none of these settings.
The bare `grove` lifecycle and the confined `grove run` both support the route.

Add a `skills` object to the settings of the 2.0 HOME. Each source is a local directory,
named by an absolute path:

```json
{
  "skills": {
    "schemaVersion": 1,
    "sources": [
      { "id": "grove", "type": "local", "path": "/absolute/path/to/grove-checkout/plugins/grove", "roots": ["skills"] }
    ],
    "mandatory": []
  }
}
```

A `mandatory` entry names a file that the agent reads before your task. No Grove skill
must be mandatory, because the mandate names the kind skill. Your configuration must pass
`request.prompt` to the harness, as the example above does. `request.prompt` holds Ravel's
preamble and then your mandate in a delimited block. `request.originalPrompt` holds the
mandate alone.

**Ravel reads local directories only.** It runs no `git` command and makes no network
request for a source. It does not fetch, install or update a source. Make each checkout of
a skill repository with a tool outside Ravel. A source `type` other than `local` refuses as
`settings_invalid`.

Ravel captures the files of the checkout, with uncommitted changes, and it leaves out `.git`
and `.jj`. A dedicated checkout at a recorded commit therefore suits delivery better than a
development working copy.

**Use a separate HOME while a 1.1 launch remains.** Ravel 1.1.0 refuses a settings file
that holds `skills`, and a separate installation prefix cannot prevent that. Run the
directory route under its own HOME while a 1.1 launch still reads the settings of your
usual HOME. A separate HOME also moves Ravel's skill cache, the run logs of `grove run` and
the HOME-based configuration of your harnesses and VCS. A native launch on 1.1 and a
directory launch on 2.0 can then run at the same time.

If a configured source or root is missing, Ravel refuses as `skill_source_missing` before
any configuration code runs. Grove reports Ravel's statement and remedy and stops. The leaf
stays live. Restore the checkout, or change the settings deliberately, then rerun `grove`.

Each launch captures the current contents of its sources. A running session keeps the
snapshot it received, also when you change a source or the settings. The next launch
receives the new contents.

<a id="usage-standalone"></a>
## Running a standalone task

```sh
grove run --help
grove run release-notes --prompt-file prompt.md \
  --input changes.txt --output release-notes.md --ui auto
```

Route the kind to a noninteractive command in your [launch policy](#usage-launch-policy).
The `release-notes` kind must run the headless helper that the release task stages
([Releasing](RELEASING.md#release-notes)). Grove runs `ravel start-session --confine` in the
staged directory. Ravel selects outside confinement and supervises inside it.

Owner policy and settings stay outside the managed reads. The invocation has no task
identity and no persistent Ravel record. It receives only its own ephemeral ending context.
A refusal launches nothing and publishes nothing. The diagnostics stay in the transcript of
Grove.

This invocation can run from anywhere, including from a release task that a running grove
launched. It creates a private temporary directory and copies the inputs into it by
basename. It passes your prompt, the output instructions and the completion instructions to
the harness that your policy selects.

You can supply the prompt as a positional argument instead of `--prompt-file`. Repeat
`--input`, `--output` or `--runtime-read` for multiple files. The basenames of the inputs and
outputs must all be distinct. Grove refuses symlink inputs and existing output destinations.

The harness can write only inside its invocation directory. Supply private runtime files,
such as credentials, with explicit read-only `--runtime-read` grants. Writable harness state
must stay in the temporary directory. macOS requires its native sandbox. Linux requires
bubblewrap. An unavailable sandbox fails the invocation before any unconfined command can
run.

The sandbox does not grant your native skill installations under HOME. On the
[directory route](#usage-directory-route), Ravel 2.0.0 grants read access to the captured
skills, so the harness can read them. It cannot write them or read their source
directories. On a native route, the harness cannot read native skills under HOME. It
reports the missing guidance and runs the force-stop command, and Grove publishes nothing.
Grove does not retry without the sandbox.

Every invocation that launches announces its task kind and the location of its transcript
under `~/.local/state/grove/runs/`. It streams the sanitised output through its caller. It
reports completion or failure.

`--ui auto` also opens a view in an active tmux or Zellij session when one is available. If
the pane fails, Grove falls back to inline output. `--ui inline` requests just the
transcript of the caller. `--ui pane` requires a working supported mux and reports an error
otherwise.

The pane shows the transcript. Interaction and approvals belong to the noninteractive
command that your policy selects. The child receives no terminal handle and no mux handle
from its parent. The logs stay available after the pane or the temporary directory closes.

After the harness checks the outputs, it executes the exact canonical
`ravel end-session grove-continue` command in its prompt. The command takes no session ID.
It ends only its enclosing invocation. Ravel reaps the managed group before it returns the
result.

Grove checks its cancellation and for a successful Ravel status before it accepts the result.
It accepts exactly the bytes of `grove-continue` from a held unlinked stdout file. Managed output
cannot contaminate the result.

Grove publishes the outputs through the original held work-directory descriptor. It does not
overwrite a destination. A missing or unsupported result publishes nothing.

The prompt also supplies the exact `ravel end-session grove-force-stop` command. That command
fails this invocation and publishes no outputs, even when every declared output is valid. A
continue result succeeds after the cleanup, even when the managed process later exits with a
nonzero status.

<!-- tc: pending because the conversion has not reached this part of the document -->

<a id="usage-viewing-tree"></a>
## Viewing a tree

```console
$ grove view
$ grove view /path/to/another/worktree
```

The argument is the directory containing `.grove`. It defaults to the current
working directory, with **no upward search**: from a subdirectory, view observes
that subdirectory's `.grove`. The path is made absolute once. Temporary non-jj
directories and concurrent viewers work; viewing uses no driver lease or launch
policy. It never creates, repairs, renames or writes tree files, and all
selection, expansion and scrolling state disappears on exit.

The viewer starts in full-width **Tree** view. **Tab** switches to the selected
item’s full-width **File** view and back; the title and footer name the active
view and Tab destination. The root opens `_BRIEF.md`, a branch opens its named
node brief, and a task opens its task file. Branches start expanded.

Every row begins with a fixed status area before its indentation and handle:
**LIVE**, **✓ DONE**, **✗ ABANDONED**, or **EMPTY**. DONE markers, words and item
text are green; ABANDONED is red; LIVE and EMPTY use normal text. The separate
`>` cursor marks selection without changing those colors. The `+`/`-` beside a
branch's name marks folding, so lifecycle, selection and expansion remain
distinct without color. A witnessed **RUNNING** item uses bold yellow for its
activity word and item text; its lifecycle marker and word retain their colors.
A task can therefore show green **✓ DONE** beside **RUNNING** until its session
ends. The next eligible leaf has a bold **NEXT**
word in normal text color; selecting it does not highlight the rest of its row.

Both views keep **RUNNING** and **NEXT** summary lines above the body, even when
either item is folded or offscreen. Idle shows `RUNNING: none (idle)` and the
ordinary Grove selection as NEXT: the first live non-finish leaf in depth-first
position order, or the sole live finish leaf when no ordinary task remains.
For a witnessed running item in the current tree, NEXT applies that rule after
excluding its permanent key. Finish can thus be NEXT while the last ordinary
session is still running. Renaming, moving, retiring or decomposing the running
item preserves its marker, using its current handle; a running branch's children
remain eligible. Ancestors do not inherit RUNNING.
This is a forecast, not a promised launch. No eligible leaf means `NEXT: none`;
viewing never creates a finish sentinel. Long summary slugs shorten before their
permanent keys. The location/view, tree observation, RUNNING, NEXT and footer
lines leave three bordered content rows at the minimum 60 × 10 size.
A missing, unreadable or changing tree makes NEXT unavailable while the RUNNING
summary continues to report independently observed runtime activity. For example,
verified idle still reads `RUNNING: none (idle)` beside
`NEXT: unavailable — tree unavailable`. An active legacy epoch record without a
supported witness makes activity unavailable; that record alone does not prove
that a session is alive. A released, validated launch witness establishes idle
even if an active epoch and Started bytes remain after the session ends.
If the running item disappears, its launch handle remains in the summary with
`item absent`; NEXT still selects from the current tree. Removing the root shows
`tree absent`, and failed or stale tree reads show `tree unavailable`, with no
row attachment or NEXT. Replacing the root shows `previous tree`: the old mandate
attaches to no new row and excludes no reused key from NEXT. Replacement clears
old selection, folds and file positions; edits to the same root's brief preserve
them. Summary qualifiers and permanent keys survive shortened slugs at 60 × 10.
The exit signal alone does not clear RUNNING; it remains until the driver
reaps the session. Driver death releases the evidence even if a child survives.
An incomplete Started publication shows activity waiting.

The root and branches count all descendant leaves, including folded ones.
Their status is LIVE if any are live, otherwise DONE if any are done, otherwise
ABANDONED, or EMPTY with no leaves. Separate LIVE/DONE/ABANDONED totals expose
mixed terminal branches. Handles come before kinds and counts. At narrow widths,
deep indentation compresses to an ellipsis and long slugs are shortened before
their `-k<key>` suffix; trailing kinds and counts use the remaining space.
The lifecycle stays visible even at the supported minimum of 60 × 10.

| Key | Action |
|---|---|
| Tab | Switch full-width Tree/File views; the visible view is labeled `[active]` |
| Up/Down or k/j | In the tree, select the previous/next visible row; in the file, scroll one line |
| Right or l | In the tree, expand a branch or enter its first child; in the file, scroll right |
| Left or h | In the tree, collapse a branch or select its parent; in the file, scroll left |
| Enter or Space | In the tree, fold or expand the selected root/branch |
| Home/End | First/last visible tree row, or beginning/end of the file, according to the active view |
| PageUp/PageDown or Ctrl-u/Ctrl-d | In File view, scroll by one page |
| ? | Show key help; navigation pauses until help is dismissed |
| Escape | Dismiss help |
| r | Refresh immediately, preserving the selected item and reading position |
| q or Ctrl-c | Quit from either view or help |

Files render as **formatted Markdown**: headings, emphasis, nested lists,
quotes, rules, inline code, fenced/indented code blocks and aligned tables.
Prose wraps to the full view width. Code keeps its indentation and tables keep
their columns; Left/Right scrolls those wide blocks without shifting prose. Links show
labels and destinations, images show alt text, and HTML is inert text. Nothing
opens a link, fetches an image, runs code or executes terminal control characters.
Syntax highlighting is not provided. The tree and selected bytes refresh every
500 ms, including same-size edits; `r` refreshes immediately. Captured names and
file bytes remain readable after the shared tree lock is released, so reading
a file does not hold up tree mutations.

Resize preserves the reading location through prose reflow, clamping near the
end of a file. Returning to a previously selected file restores its reading
position, including horizontal scroll. These positions live only in memory.
Switching preserves tree selection, expansion and viewport, as well as each
file’s reading position. Hidden views keep their saved dimensions until shown;
returning to Tree adjusts its viewport only as needed to keep selection visible.
Switching does not force a reload or restart the refresh deadline. Automatic
observation continues in both views, in help and during the resize notice.
The footer points to key help.

Selection and expansion follow permanent keys across renumbering, renaming,
moves and retirement. Decomposing a selected task opens its new branch brief.
A moved selection reveals its new ancestors. If an item disappears, the viewer
selects its nearest surviving old ancestor and shows a notice. Replacing or
removing `.grove` clears saved item state, even if a new tree reuses its keys;
replacing only `_BRIEF.md` is an ordinary content edit. Duplicate permanent keys
anywhere in the tree, including terminal leaves and branches, or multiple live
`finish` leaves show an error. The driver and `grove-llm pick` refuse the same
ambiguities. Follow the [selection repair guidance](#reading-the-tree);
viewing never allocates a finish leaf. Reading positions follow unchanged source
lines through insertions and
deletions, including when you return to a file edited while reading another item.
Repeated lines use surrounding unchanged text, then proximity and source order
to choose consistently. A deleted line falls back to the nearest surviving old
line (the following line wins a tie), or a clamped position if none survives.
Saved reading positions survive file errors until readable content returns.

A missing tree shows its observed path and a Missing message. A malformed or
unreadable reload shows an error; a retained previous tree is explicitly STALE.
A selected-file failure is shown in File view. Fix the external problem;
the viewer retries automatically and never repairs it. An empty document has an
empty File view. Resize keeps the selected item. Below 60
columns or 10 rows, a resize message replaces the view; navigation pauses and
state is retained until the view fits again. Quit and refresh still work, including
while help is open. Refresh retains the selected item while the views are hidden.

Startup, selection and reload use a quiet nonblocking reader. While a writer
holds the tree, the viewer shows WAITING and retains its previous display;
navigation and quit remain responsive. It retries the pending read every 500 ms
until contention ends. Other external changes and repairs are observed on the
same deadline, with at most one pending observation.
Tree capture releases its shared guard before a separate, nonblocking runtime
read. Runtime failure does not prevent browsing. Epoch contention or changing
activity shows WAITING; unsupported, missing or malformed evidence beside a
lease shows activity unavailable. Both suppress the current pair and clear row
activity until observation recovers. Failed tree capture also withholds NEXT
and clears old row activity. Tree diagnostics remain separate from activity
freshness, so consistent tree updates still appear while activity is changing.
Legacy active drivers have no supported activity witness and therefore show
activity unavailable. Current-tree RUNNING and exclusion-aware NEXT require the
observer's verified same-tree relation; numeric identity or leftover bytes alone
cannot attach a mandate to a row.
All advisory guards release before rendering or waiting for input.

A driver handoff overlapping a runtime read can briefly print
`waiting for exclusive session epoch lock` with its operation name. Ordinary
reads finish promptly. If an observer is suspended while holding that runtime
guard, the driver fails the handoff after 30 seconds; resume or stop the suspended
holder so its guard releases, then restart the driver. Pausing after tree capture
or while displaying a captured result holds no epoch guard. Do not delete the
control files to bypass a waiting reader.

Run directly with interactive stdin and stdout; piping either is refused before
terminal setup. Quit, Ctrl-c, SIGINT, SIGTERM and SIGHUP restore raw mode,
alternate screen and cursor. Signal handlers only request an exit; cleanup runs
on the viewer thread. Partial setup failures and input/draw errors restore the
terminal before an error is printed, as do unwinding panics before their panic
diagnostic. A disconnected terminal causes the viewer to exit; cleanup output
cannot be delivered to a closed endpoint.

<a id="usage-running-grove"></a>
## Running Grove: start, resume, and finish

Run Grove from anywhere inside the working tree:

```console
$ grove
grove: launching requirements through Ravel — plan-k1
ravel: starting provider anthropic, model claude-opus-5-5, effort max for kind "requirements"
```

The first line is Grove's and names the kind and the leaf's stable handle. The
second is Ravel's: the labels your policy gave the command it
selected. Ravel keeps no run record.

Bare `grove` is the lifecycle command, with no launch-policy flags.
`grove view [WORKTREE]` opens the read-only browser described below.
`grove --help` and `grove --version` stop before touching a repository:

```console
$ grove --help
Grove: hierarchical workstream tool for AI agents

Usage: grove [COMMAND]

Commands:
  run   Run one task kind in a confined temporary directory, without a grove
  view  Browse a .grove task tree read-only with automatic refresh

Options:
  -h, --help     Print help
  -V, --version  Print version

$ grove --version
grove 21.13.0
```

Bare `grove` inspects the filesystem and does the appropriate next thing:

| What it finds | What it does |
|---|---|
| No `.grove/` | Creates the root brief and a first `requirements` leaf, then launches it. |
| Live leaves | Launches the first one in tree order. |
| No live leaves | Materializes a `finish` leaf and launches the teardown session. |

It then keeps going: when a session signals that its task is complete, Grove
relaunches with fresh context for the next leaf. Any other ending — you exit the
session, press Ctrl-C, or the process dies — stops the loop, and Grove says which
ending it was:

```console
grove: launching impl through Ravel — api-k7
ravel: starting provider anthropic, model claude-opus-5-5, effort high for kind "impl"
grove: session ended without a supported Grove result — status exit status: 0, elapsed 0.508s; loop stopped.
```

A session that recorded teardown in its launch directory prints
`grove: grove finished — loop complete.` instead, unless driver interruption or
a successful explicit force-stop takes precedence.

For bare `grove`, **the working directory is the only thing
that selects a workstream**. There is no tree to name and no confirmation step:
Grove scaffolds and commits against whichever working tree encloses the
directory you ran it from. That is what makes the command short, and it is worth
knowing before you run it in a repository that holds several Jujutsu workspaces —
running it in the wrong one starts a grove there, not in the one you meant.
`jj op restore` recovers it if that happens; see
[Undoing a mistake](#undoing-a-mistake) below.

To resume, run `grove` again. Grove has no progress database; it re-derives its
position from the task tree every iteration, which is what makes restart and
continuation the same thing.

None of those tree mutations consults your policy. Grove scaffolds a grove and
materializes a `finish` leaf with no policy installed, and the policy is
evaluated when the leaf launches
([launch policy](#usage-launch-policy)). The one thing Grove checks first is
that compatible Ravel is available on absolute PATH. Without it Grove stops before
touching the tree:

```console
$ grove
Error: Ravel is missing from absolute PATH directories. Install it with `brew install linkuistics/taps/ravel` and add its bin directory to PATH.
```

Grove makes one commit of its own — the teardown commit at the end. It touches
only `.grove/`, leaving unrelated working-copy changes in the successor commit.
Repository failures surface normally, and `jj op restore` undoes the commit like
any other jj operation.

### If the tree is not jj-enabled

Grove refuses before creating anything and names both remedies:

```console
$ grove
Error: not a Jujutsu working tree
  looked for a `.jj` directory at and above: /home/you/app

Make the tree jj-enabled and rerun:
      jj git init --colocate     # an existing Git repository, history kept
      jj git init                # no repository here yet

Nothing was created or changed.
```

`jj git init --colocate` in an existing Git repository keeps every commit and
leaves `git` itself working; `jj git init` is for a directory with no repository
at all. Run one and rerun `grove`. Jujutsu's own binary must be on `PATH` —
Grove asks jj for facts it will not guess, and says so if it cannot run it.

<a id="usage-refused-launch"></a>
### If a launch refuses

Your policy is evaluated when a leaf launches, and nowhere earlier
([launch policy](#usage-launch-policy)). When it cannot select a command for
the kind, Ravel refuses and launches nothing. Its diagnostic
appears on the terminal the session would have had, with the remedy. Grove
then reports the exit status, stops the loop and leaves the leaf live. That
status is Ravel's: Grove itself exits `0`, as it does for any
session that ends without signalling ([stopping the loop](#stopping-the-loop)).
Here the tree holds a leaf of a kind, `spike`, that the policy does not route:

```console
$ grove
grove: launching spike through Ravel — api-k1
ravel: refused (policy_refused, stage selection): the policy /home/you/.config/ravel/policy.ts refused the selection: ROUTES names no route for kind "spike"
  input: --kind spike
  source: /home/you/.config/ravel/policy.ts
  policy code: incomplete_mapping
  remedy: add a route for this kind to ROUTES in your policy
grove: session ended without a supported Grove result — status exit status: 3, elapsed 0.510s; loop stopped.
       session kind `spike` for `api-k1` failed; if Ravel refused the launch, its diagnostic and remedy are above and the leaf is still live. Either way, rerun `grove` to continue.
```

Read the diagnostic's policy path and remedy. Edit that personal policy, here
adding `spike` to its `ROUTES` table, then rerun `grove`. Grove selects the same
live leaf and Ravel evaluates the repaired policy with its actual task and
prompt. If selection still refuses, follow the new diagnostic and rerun after
repairing it. A successful selection launches the command:

```console
$ grove
grove: launching spike through Ravel — api-k1
ravel: starting provider anthropic, model claude-opus-5-5, effort high for kind "spike"
```

The same recovery serves every refusal. Three an owner meets:

- **No policy.** The refusal is `policy_missing`, and its remedy is
  writing the named policy entry, or explicitly reading and copying the
  [Grove policy assets](../assets/ravel-policy/README.md) and importing them
  from that entry. A fresh grove is scaffolded first, so its
  `requirements` leaf is waiting when you rerun.
- **A choice file that names something the policy does not offer**, or violates
  that policy's required number of arrangements. Correct `.ravel-choice` using
  the diagnostic's offered names and the owner's policy requirements.
- **A review whose task file names no creator**, under a policy that imports
  the Grove review example. A producer finished without recorded execution
  evidence leaves no caller provenance. Add your explicit
  `**Creator:** declared <provider>` beneath `**Reviews:**` in the review task,
  following the [caller provenance contract](grove-provider-evidence.md), and
  rerun `grove`.

<!-- tc: on -->

### Stopping the loop

**Ctrl-C belongs to the session, not to the loop.** While a session runs, it owns the
terminal in its own right. An interrupt that you type reaches the session. The driver stays
alive to decide whether to relaunch or stop. To end the loop from outside, send `kill` to
the `grove` process:

```console
# the shell running the loop
$ grove
grove: launching impl through Ravel — api-k7
ravel: starting provider anthropic, model claude-opus-5-5, effort high for kind "impl"
grove: interrupted by signal 15 — stopping the loop.
$ echo $?
143
```

```console
# a second shell
$ kill -TERM "$(pgrep -x grove)"
$ echo $?
0
```

**Read the status where Grove exits, not where you signal it.** `$?` after the `kill` is the
status of the *signal delivery*. It is `0` whenever the signal was sent. Only the wait status
of the `grove` process itself carries `128 + N`. A wrapper script that backgrounds Grove
reads that status with `wait "$grove_pid"`.

Grove forwards the signal to the `ravel` process. Ravel cancels and reaps the managed
group. Grove waits for Ravel and restores your terminal. Then Grove **dies of the same
signal**. A wrapper script, a `timeout` or a systemd unit reads `128 + N`, so it can tell an
interrupted grove from a finished one.

A grove that ran to a clean finish exits `0`. A grove that stopped because a session ended
without a signal also exits `0`.

<!-- tc: pending because the conversion has not reached this part of the document -->

<a id="undoing-a-mistake"></a>
### Undoing a mistake

Grove writes through Jujutsu and hand-builds no transaction of its own, so
jj's operation log is the undo:

```console
$ jj op log --limit 3
$ jj undo                    # reverse the most recent operation
$ jj op restore <operation>  # go back to a named point
```

That covers the two mistakes worth naming: a grove started in the workspace you
did not mean, and a teardown commit you want back. `jj undo` reverses the last
operation; `jj op restore` returns the whole repository to a chosen one. Grove
runs no recovery of its own and never rewrites history on your behalf.

<a id="usage-task-tree"></a>
## The task tree and its filename grammar

A small workstream might look like this:

```text
.grove/
├── _BRIEF.md
├── 01-DONE-requirements--plan-k1.md
├── 02-DONE-design--auth-k2.md
├── 03-review-design--auth-k3.md
└── 04-impl--ship-k4.md
```

A leaf is one agent-sized task. A directory is a node holding smaller tasks,
named `NN-k<key>/`, with exactly one node file `_<slug>.md`. That file names
the work in its filename and carries the brief in its body. The root requires
`_BRIEF.md`. Titles never come from file contents or directory names.
A leaf filename carries its routing and identity:

```text
NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md
```

- `NN` is the mutable position among siblings.
- `DONE` and `ABANDONED` are the two terminal outcomes, kept in place until the
  grove finishes.
- `<session-kind>` is any well-formed token — lowercase ASCII letters, digits and
  single dashes, no `--` — and is what your launch policy selects a command
  for and what names the skill a session loads. Grove holds no list of
  kinds; the ones that exist are the `grove-<kind>` skills you have installed.
  The `--` between the kind and the slug is what makes the name unambiguous.
- `<slug>` is the human-readable name.
- `k<key>` is the permanent identity. `<slug>-k<key>` is the **stable handle**,
  the way a work item is named in commit messages. A node combines the slug
  from its node file with the directory key. Handles survive renumbering; a
  title edit changes the handle, while the permanent key still resolves.

There is no witness file and no format stamp: the filenames *are* the format. A
tree whose names this grammar does not spell is refused by name, with the
offending path and the shape it should have had — Grove does not migrate an
older layout.

Every node file is a regular file. `_BRIEF.md` belongs only at the root;
`_<slug>.md` belongs only in a positioned node, and `BRIEF` is reserved.
A missing node file, two competing `_` files, or a malformed digit- or
underscore-prefixed entry refuses reads and mutations, naming the offending
level or files and the canonical form. Restore the intended file or move the
unintended one outside the tree before retrying; reads never repair the tree.
For a missing node file, first check for an interrupted decomposition: an empty
node directory and a sibling leaf with the same position and key are two halves
of one operation. Remove the empty directory to keep the leaf, or move the leaf
into it as `_<slug>.md`; do not allocate a new key or invent a second brief.

Grove picks the first live leaf in depth-first pre-order. There is no hidden
scheduler and no dependency inference from prose: ordering in a grove is
contiguity, at every level. The one exception is the driver-owned `finish` leaf,
which is skipped while any other work is live.

**Reading the tree by eye.** Because the state is entirely in the names, you can
answer "what runs next?" without running anything — the first leaf filename in
depth-first order with no outcome infix, excluding node files:

```console
$ find .grove -name '*.md' | sort
.grove/01-DONE-requirements--plan-k1.md
.grove/02-DONE-design--auth-k2.md
.grove/03-review-design--auth-k3.md
.grove/04-k4/01-impl--api-k7.md
.grove/04-k4/_ship.md
.grove/_BRIEF.md
```

`03-review-design--auth-k3.md` is next: it is the first entry carrying neither
`DONE` nor `ABANDONED`. `grove-llm pick` computes the same answer, and
[the verbs section](#usage-tree-verbs) shows it; the point of the grammar is that
you do not need it.

Review chains and research pairs are flat siblings named off a shared stem — the
`auth` producer and its `review-design` step above share one slug and differ only
by kind and key — so a listing shows the shape without any nesting. The kind
states each step's role, so the slug does not restate it. Nothing groups them: the
stem is a reading convention, not grammar. See
[Architecture: task kinds and composition](ARCHITECTURE.md#task-kind-taxonomy).

<a id="usage-session-lifecycle"></a>
## What happens in a session

Grove launches the command your policy selects for the selected leaf's kind and
hands it that leaf's stable handle as an explicit mandate. The session:

1. Resolves the mandated handle and reads the glossary, ancestor briefs, cited
   decision records, and the task file.
2. Applies the discipline named by the leaf's session kind.
3. Does and verifies the work.
4. Marks the leaf `DONE`, closes any completed parent nodes, and commits all of
   that as one focused commit naming the stable handle.
5. Publishes `grove-continue`; Ravel ends and reaps the managed group,
   and Grove relaunches for the next leaf.

Watching one leaf through, from the outside, is a launch line for it and a
launch line for the next, each followed by Ravel's own:

```console
grove: launching impl through Ravel — api-k7
ravel: starting provider anthropic, model claude-opus-5-5, effort high for kind "impl"
grove: launching review-impl through Ravel — api-k8
ravel: starting provider openai, model gpt-6.1-sol, effort high for kind "review-impl"
```

The second `grove:` line is the whole of step 5 as you see it: the session ran
the exact `ravel end-session grove-continue` command in its launch prompt, which published its opaque result; Ravel ended and reaped
that managed group and returned the exact `grove-continue` bytes, and Grove launched the
next leaf with fresh context. Between the two lines the session made
its own commit, so `jj log` shows `api-k7: …` before the relaunch.

The session does not pick its own leaf. If a new leaf is inserted ahead of the
running session's mandate, it becomes the next iteration's work rather than
preempting the session already launched.

A task that proves too large is decomposed in place into a node with its own
brief. Work that should no longer be done can be marked `ABANDONED`, but pruning
requires explicit human confirmation — Grove guides that decision and never makes
it autonomously. Both are verbs, and
[the verbs section](#usage-tree-verbs) works them.

The runtime methodology is the **`grove` plugin**, whose spine is
[`plugins/grove/skills/grove/SKILL.md`](../plugins/grove/skills/grove/SKILL.md)
and which ships one `grove-<kind>` skill per session kind. A launch prompt names
the one skill this session's kind needs, and the session loads it through its
harness's own skill-loading affordance. **A kind exists iff a skill of that name
exists** — adding one is authoring a skill and routing the kind in your policy,
never editing or rebuilding a binary.

<a id="usage-skill-delivery"></a>
**You supply the skills; Grove installs none.** Grove names the skill and
leaves its availability to you
([decision](adr/skill-delivery-is-the-users-choice.md)). Choose a route for
every harness your policy can select:

| Route | What you do |
|---|---|
| Native harness installation | Claude Code: the marketplace with auto-update enabled. Codex, Gemini CLI and Pi: the checkout's manual installer. See [Install the skill plugins](../README.md#install-the-skill-plugins). Repeat for each harness. |
| Ravel skill-directory delivery | Configure local skill sources for Ravel 2.0.0 under a HOME of their own. Ravel adds their locations to each launch prompt, so native copies are unnecessary. See [the directory route](#usage-directory-route). |

Native-only installation is a complete setup, not a fallback. Grove checks no
skill before launch. A session that cannot find its named skill reports that;
repair the route you chose and rerun. A checkout edit reaches your
installations through their own update route, not through a Grove rebuild.
Independently authored session-kind skills remain their author's
responsibility.

<a id="usage-earlier-codex-installations"></a>
**Codex skills installed by earlier Grove releases.** Grove 21.11 to 22.8
installed Codex skills automatically at bare startup. Links in `~/.agents/skills`
and in the Codex skill directory pointed through `~/.agents/.grove/current` into
snapshots under `~/.agents/.grove/snapshots`. Later releases never adopt,
repair, repoint or remove those entries, and an upgrade leaves them too. The
links keep working while their snapshots remain, but they no longer follow Grove
upgrades.

Keep them, or replace them with links that you choose. To remove the links,
follow the [optional cleanup procedure](#usage-codex-cleanup). Its path or name
alone does not show that Grove wrote an entry.

<a id="usage-codex-cleanup"></a>
### Optional cleanup of earlier Codex installations

You can keep the old entries indefinitely. A harness can see an old copy of a
skill beside a new one, and the old links cause that. This procedure removes
only the links in `~/.agents/skills` that you can show the old provisioner
wrote and that nothing still needs. Grove supplies no command for it, and no
Grove release runs it.

The procedure keeps the Grove root `~/.agents/.grove`, with `current`, every
snapshot and `install.lock`. No search can show that nothing needs an entry
there. A link or a setting that you made can lead through it, also from a
location that you do not search. The Grove root is small. In one installation
of Grove 22.8, 15 snapshots used 13 MB. This procedure gives no evidence for
the removal of any entry in it.

Each startup of Grove 21.11 to 22.8 wrote this layout when `CODEX_HOME` had a value
or `~/.codex` existed. The Codex skill directory is `$CODEX_HOME/skills`, and
the default of `CODEX_HOME` is `~/.codex`. If `CODEX_HOME` was a relative path,
Grove resolved it against the directory in which Grove started. Repeat the
inventory in each such directory.

Grove resolved a relative `HOME` in the same way. Each `HOME` under which Grove
ran, such as that of a service or of `sudo`, can hold its own layout. Repeat
this procedure for each.

| Entry | What the provisioner wrote |
|---|---|
| `~/.agents/.grove/install.lock` | An empty lock file. |
| `~/.agents/.grove/snapshots/bundle-XXXXXX` | A copy of every regular file in the `plugins` tree of that Grove build. A refresh added a snapshot and kept the earlier ones. |
| `~/.agents/.grove/current` | A link to one snapshot, by absolute path. |
| `~/.agents/skills/NAME` | A link to `~/.agents/.grove/current/plugins/PLUGIN/skills/NAME`, by absolute path. The provisioner replaced an earlier entry at NAME only when it was a link to a path below `current/plugins` or `snapshots`. It also replaced a link to a skill of a Grove plugin. For each other entry, startup stopped with an error. Thus you or the manual installer can have made the original link. |
| Codex skill directory, `NAME` | A link of the same form, only where a link already was. The provisioner replaced only a link that pointed below `current/plugins` or `snapshots`, or to a skill of a Grove plugin. It made no new links there. Thus you, the manual installer or another tool made each original link. |
| A `link-XXXXXX` directory | A directory with one link named `link`. A startup that was killed before it published that link left it. It can be in each skill directory and in the Grove root. |

A skill of a Grove plugin is a path of the form `.../plugins/PLUGIN/skills/SKILL`
for a plugin of the Grove repository. The provisioner also removed links of
these forms at the names that it did not ship.

The provisioner kept no record of what it wrote. Thus the layout alone is not
evidence of ownership. The manual installer, another tool or you can make an
entry of the same form.

#### 1. Stop every writer and reader

1. Stop every Grove driver and every session that it started. Include
   `grove run` tasks and sessions in detached terminals.
2. Stop every Codex session and every other harness that reads these skill
   directories.
3. Make sure that no Grove 22.8 or earlier executable can start again. Include
   Homebrew kegs, builds in old checkouts and copies elsewhere on `PATH`. Each
   startup of one writes the layout again.
4. List the remaining processes with `pgrep -fl 'grove|codex|ravel|claude|gemini'`.
   On Linux, use `pgrep -af` with the same pattern.
5. Add the name of each other harness that you use to the pattern.
6. Stop each listed process that can read a skill.

A list of open files cannot find every reader. `lsof +D ~/.agents/skills` shows
only the files that are open now. A session that read a skill can open the
resources of that skill later.

#### 2. Make an inventory

These commands only read. Run `sh` first, and run them in that shell. In zsh, a
pattern that matches nothing stops the whole loop. The commands need `realpath`,
which macOS 13 and later and Linux supply. If `~/.agents` does not exist, there
is nothing to remove. If `command -v realpath` prints nothing,
install GNU coreutils and put its `realpath` on `PATH` before you continue.

```sh
codex_skills="${CODEX_HOME:-$HOME/.codex}/skills"
for dir in ~/.agents/skills "$codex_skills"; do
  if [ -d "$dir" ]; then realpath "$dir"; else echo "$dir: no directory"; fi
done
for dir in ~/.agents/skills "$codex_skills"; do
  for entry in "$dir"/* "$dir"/.[!.]* "$dir"/..?*; do
    [ -L "$entry" ] || [ -e "$entry" ] || continue
    if [ -L "$entry" ]; then
      printf '%s -> %s\n' "$entry" "$(readlink "$entry")"
    else
      printf '%s (not a link)\n' "$entry"
    fi
  done
done
```

<!-- tc: depart synonym-rotation because launch policy is the name of the owner's policy file -->
If the two `realpath` lines are equal, the two skill directories are one
directory. Inspect and change that directory once. Repeat the inventory for each
other value of `CODEX_HOME` that you use. Look for such values in your shell
profile, your launch policy and the environments of your services. For each
`link-XXXXXX` directory, run `ls -la` on it and `readlink` on its `link`.

If the Codex skill directory does not exist, the loop says so. Then the
provisioner wrote no links there.

Then find the retained links and texts. A retained link or text is outside the
skill directories and leads through an entry of a skill directory. A reader
that follows it fails when that entry goes.

The search below visits each link in your home directory, in `~/.agents`, with
the Grove root, and in the Codex home. It also enters other file systems. It
follows the text of each link one component at a time, as the system does. It
also follows each link that it meets on the way, also outside these locations.
Thus it finds a link that reaches a skill through another link or through `..`.

Run the two blocks below in one shell, in order.

```sh
codex_skills="${CODEX_HOME:-$HOME/.codex}/skills"
agents="$(realpath ~/.agents)"
codex_home="$(realpath "${CODEX_HOME:-$HOME/.codex}" 2>/dev/null)"
codex_real="$(realpath "$codex_skills" 2>/dev/null)"
skills_real="$(realpath ~/.agents/skills 2>/dev/null)"
set -- "$(realpath ~)" "$agents"
[ -z "$codex_home" ] || set -- "$@" "$codex_home"
roots="$(printf '%s\n' "$@")"
find "$@" -type l -exec sh -c '
  a="$1" b="$2" roots="$3"
  shift 3
  for link do
    rest="$(readlink "$link"; echo .)"
    rest="${rest%?}"
    rest="${rest%?}"
    case "$rest" in /*) cur="" ;; *) cur="${link%/*}" ;; esac
    n=0
    stop() { [ ! -e "$link" ] || printf "%s unknown at %s\n" "$link" "$1"; }
    while [ -n "$rest" ]; do
      c="${rest%%/*}"
      case "$rest" in */*) rest="${rest#*/}" ;; *) rest="" ;; esac
      [ -n "$c" ] && [ "$c" != . ] || continue
      [ -d "${cur:-/}" ] || { stop "${cur:-/}"; continue 2; }
      [ -x "${cur:-/}" ] || { printf "%s unknown at %s\n" "$link" "${cur:-/}"; continue 2; }
      [ "$c" != .. ] || { cur="${cur%/*}"; continue; }
      for d in "$a" "$b"; do
        [ -n "$d" ] && [ "${cur:-/}" -ef "$d" ] && printf "%s uses %s\n" "$link" "$cur/$c"
      done
      if [ -L "$cur/$c" ]; then
        n=$((n + 1))
        [ "$n" -le 40 ] || { stop "$cur/$c"; continue 2; }
        t="$(readlink "$cur/$c"; echo .)"
        t="${t%?}"
        t="${t%?}"
        case "$t" in /*) cur="" ;; esac
        rest="$t${rest:+/$rest}"
      elif [ -e "$cur/$c" ]; then
        cur="$cur/$c"
      else
        stop "$cur/$c"
        continue 2
      fi
    done
    [ -d "${cur:-/}" ] || continue
    r="${cur:-/}"
    for d in "$a" "$b"; do
      p="${d:+$d/}"
      while [ -n "$p" ]; do
        p="${p%/*}"
        [ "$r" -ef "${p:-/}" ] && { printf "%s reaches %s\n" "$link" "$r"; break; }
      done
    done
    q="$r/"
    out=yes
    while [ -n "$out" ] && [ -n "$q" ]; do
      q="${q%/*}"
      while IFS= read -r p; do
        [ "${q:-/}" -ef "$p" ] && out=
      done <<ROOTS
$roots
ROOTS
    done
    [ -z "$out" ] || printf "%s leads out to %s\n" "$link" "$r"
  done
' sh "$skills_real" "$codex_real" "$roots" {} + | sort -u
```

Each line names a link and tells why the search shows it:

| Line | Meaning |
|---|---|
| `LINK uses ENTRY` | The path of LINK goes through ENTRY, an entry of a skill directory. The link needs ENTRY. |
| `LINK unknown at DIR` | The search cannot enter DIR, for example a directory that you cannot read. What the link needs is unknown. |
| `LINK reaches R` | LINK is an alias: R is a skill directory or a directory that contains one, such as your home directory. A reader can reach each entry of that skill directory through LINK. |
| `LINK leads out to R` | LINK leads to a directory outside the searched locations. The search does not visit the links in R. |

The search compares directories by device and inode, not by name. Thus it finds
a path in another case on a file system that ignores case. It also finds a path
through a firmlink or a bind mount. ENTRY and R keep the spelling of the link
text.

For each line that ends with `leads out to R`, search R too. After the line that
starts with `[ -z "$codex_home" ]`, add the line `set -- "$@" R`, with the path
R in quotes. Then run the block again. Repeat until the R of each such line is
in a location that you search. If you cannot search an R, keep every entry.

Some links fail for every reader now. The search shows no line for such a link
unless it goes through a skill directory before it fails. Such a link goes to a missing
path or through a file, or it is a loop. A removal cannot break it further for
a reader that sees the same paths.

A container, a virtual machine or another host can mount a skill directory or a
directory that contains one. A reader there sees other paths. That mount is an
alias, and its readers use the entries of that skill directory.

A volume that is not mounted makes its paths missing. Thus mount each volume
that can hold links or settings before you search. The search can take a long
time, and longer on a network file system.

Then find the texts that name such a path:

```sh
set -- -e .agents -e .codex -e CODEX_HOME
for p in "$agents" "$codex_home" "$codex_real"; do [ -z "$p" ] || set -- "$@" -e "$p"; done
find -L ~/.config "$agents" ${codex_home:+"$codex_home"} -path "$agents/.grove" -prune \
  -o -type f -exec grep -liF "$@" {} +
```

This search follows links, also links below its start directories. It shows each
file that names `.agents`, `.codex`, `CODEX_HOME` or one of the real paths, in
any mix of upper and lower case. A
settings file, a script or a skill that names a skill directory or a path below
it is a retained text. A file below a link of the inventory is a file of that
skill. It is a retained text only while that link stays.

A transcript or a log is a retained text while a tool can resume or replay
it. Codex keeps resumable transcripts below the Codex home. If you will not
resume a session, its transcript is not a reader.

For each line of the link search that ends with `reaches R`, add the path of its
link to the patterns with `-e`. For a link in your home directory, add the part
of its path after the home directory instead. For example, add `-e /myskills`
for `~/myskills`. That pattern also finds `$HOME/myskills`. Then run the search
for texts again. This finds a text that reaches an entry through such a link,
such as a setting of `HOME` to an alias.

Extend both searches to each other location that can hold links or settings.
Examples are a Codex home outside your home directory and project skill
directories. Give each search the real path of each such location. Add your
shell profiles to the search for texts. Also add each directory that holds your
harness settings, launch policies or scripts.

Both searches report each directory and each file that they cannot read. A link
or a text there is unknown. On macOS, a terminal without full disk access cannot
read some directories in `~/Library`. Make each such location readable and
search again. If a location stays unreadable, you cannot show that nothing needs
an entry, so keep the entries. A location that does not exist holds nothing.

The search for texts also enters the target of each link that it finds. A link
to a large tree, such as `/`, makes it search that tree too.

#### 3. Classify each entry

Give each entry of `~/.agents/skills` one of three classes. An owned and
unneeded entry is removable. An owned entry that something still needs stays.
An entry that is foreign or uncertain stays.

A link has the provisioner form when `readlink` shows exactly
`$HOME/.agents/.grove/current/plugins/PLUGIN/skills/NAME`, with your actual home
directory. NAME is the name of the link itself. The `link` in a `link-XXXXXX`
directory also has the provisioner form when NAME is any skill name.

A line of the link search keeps entries by these rules:

1. A line that ends with `uses ENTRY` keeps ENTRY. When ENTRY is a
   `link-XXXXXX` directory, it keeps that directory.
2. A line that ends with `unknown at DIR` keeps every entry. Make DIR readable
   and run the search again.
3. A line that ends with `leads out to R` keeps every entry until you search R.
4. A line that ends with `reaches R` keeps no entry by itself. Its readers can
   need entries. The last condition below covers them.

A link is removable only when all these conditions are true:

- Your own setup records, such as dotfiles and bootstrap scripts, show that
  neither you nor another tool made a link at this path. The table above shows
  why: the provisioner replaced links that you made.
- The link has the provisioner form.
- No line of the link search keeps the link.
- No retained text that the search for texts shows names the link or a path
  below it.
- You need that skill neither in that directory nor through an alias. If you do
  not know whether a reader uses an alias, you cannot show this.

A `link-XXXXXX` directory in `~/.agents/skills` is removable only when all these
conditions are true:

- `ls -A` shows only `link` in it.
- Its `link` has the provisioner form.
- The conditions for a link above are true for the directory and for its `link`.
  Some harnesses also search the directories inside a skill directory. Such a
  harness can load the skill of a staged `link`.

Keep each link in a separate Codex skill directory, or replace it yourself.
You cannot show that the provisioner made such a link. In a Codex skill
directory, the provisioner replaced only links that already existed. A directory
that was the agents skill directory under an earlier `HOME` looks the same.

If you need a skill, keep its link. Alternatively, replace the link yourself
with a link to a copy that you choose.

The manual installer writes only `~/.codex/skills`, `~/.gemini/skills`,
`~/.pi/agent/skills` and `~/.pi/agent/extensions`. It ignores `CODEX_HOME`. In each directory, it replaces every link at the name of a skill
that it ships. It also removes links to skills of Grove plugins that it does not
ship there, also links that you made. Run it only if you accept these changes.

These entries are always foreign or uncertain. Keep them:

- A real directory or file in a skill directory, except a `link-XXXXXX`
  directory. An example is Codex's own `.system`. Some earlier development
  builds of Grove also wrote real directories, and their provenance is
  uncertain.
- A link to a checkout or to another location outside the Grove root.
- A link directly to a snapshot. The provisioner did not write such links in a
  skill directory.
- A link with a different text form, also when its target is missing.
- Each entry of the Grove root.

#### 4. Move the removable entries

Move each removable entry into a new holding directory outside every skill
directory.

```sh
hold="$(mktemp -d "$HOME/grove-codex-cleanup.XXXXXX")" && echo "$hold"
mv ~/.agents/skills/NAME "${hold:?}/"
mv ~/.agents/skills/link-XXXXXX "${hold:?}/"
```

`mktemp` makes a new directory with a unique name, so no move can replace an
entry that was already there. If `mktemp` prints an error, stop. If `hold` is
empty, `${hold:?}` stops each command before it runs. Write down the
path that `echo` prints. In a new shell, set `hold` to that path before steps 5
and 6.

Name a link without a slash at its end. With the slash, `mv` and `rm -r`
operate on the target directory and not on the link.

#### 5. Check the result

1. Start a new session of each harness that reads these skill directories.
2. Make sure that each session loads each skill that you need.
3. If a skill is missing, move every held entry back with
   `mv "${hold:?}"/* ~/.agents/skills/`.

#### 6. Remove the holding directory

When the new sessions work, remove the holding directory with `rm -r "${hold:?}"`.
It holds only the links and `link-XXXXXX` directories that you moved. `rm -r`
removes a link and does not follow it, so the Grove root stays unchanged.

<a id="usage-tree-verbs"></a>
## The `grove-llm` verbs over the tree

`grove-llm` is the agent-facing tree interface a session drives during those
steps. Its own help text says the verbs "are [not] meant for direct human use",
and that is a statement about who *drives* them, not about who has to understand
them: reading a task tree means reading the result of these verbs, four of them
are diagnostics you may want by hand, and one is gated on your confirmation and
happens nowhere else.

What follows is what each verb does to the tree, what it prints, and whether it
commits. It is not the calling contract a session works from — that is the
`--help` text and the methodology skills, and **each verb's own `--help` is the
authority on what exists**:

```console
$ grove-llm --help          # the twelve verbs, one line each
$ grove-llm leaf-add --help # one verb's arguments and flags
$ grove-llm help leaf-add   # the same, spelled as a verb
$ grove-llm --version
grove-llm 20.1.0
```

Two facts nearly hold across the verbs below, and the three verbs that break them
are named rather than glossed over. **Only `finish-commit` commits** — every
other verb makes a working-tree change and leaves it there, for the session that
ran it to commit with its own work. And a verb's *answer* is absolute paths on
stdout, with diagnostics on stderr, so a caller can read it without parsing
prose. The exceptions:

| Verb | Commits? | On stdout |
|---|---|---|
| `kind` | No | A kind **token**, not a path — the one answer that is not a location. |
| `finish-commit` | **Yes** — deletes `.grove/` and commits that deletion | Nothing; the change id goes to stderr. |
| `record-teardown` | No | Nothing; it reports whether it recorded the teardown on stderr. |

One thing to expect if you run these inside a running session's own terminal:
`grove-llm` binds itself to the working tree that session was launched for, and
refuses another (`Error: wrong working tree for grove-llm <verb>: …`). Run them
in your own shell instead.

### Reading the tree

`pick` prints the next live leaf — the same depth-first pre-order answer the
driver computes:

```console
$ grove-llm pick
/home/you/app/.grove/01-requirements--auth-k1.md
```

Before selecting, both `pick` and the driver validate every item's permanent
key and refuse duplicates, even in DONE or ABANDONED leaves and branches.
Multiple live `finish` leaves also refuse selection. Ordinary live work comes
before `finish`, even when the finish leaf appears earlier in the tree.

To repair a duplicate key, stop the driver and inspect version history to
restore each item's original identity. Undo an accidental copy or interrupted
rename; do not renumber existing permanent keys or reuse a retired key. If the
copy represents genuinely new work, preserve its text outside `.grove`, restore
the valid tree, then use `leaf-add` to allocate a fresh key and restore that text.
For multiple live finishes, restore the single driver-owned sentinel from
history rather than inventing another identity. `grove view` reports these same
errors, retries after repair, and never changes the tree itself.

`kind` prints just that leaf's kind token, and `brief-chain` prints the
node-file chain root→leaf, one absolute path per line. Both default to `pick`'s
leaf and both take an optional explicit leaf path:

```console
$ grove-llm kind
requirements

$ grove-llm brief-chain
/home/you/app/.grove/_BRIEF.md

$ grove-llm brief-chain 04-k3/01-impl--api-k7.md
/home/you/app/.grove/_BRIEF.md
/home/you/app/.grove/04-k3/_ship.md
```

Every level must hold exactly one correctly placed node file. A missing,
competing or misplaced file refuses the whole read with exit `1`; no partial
chain is printed. On a valid tree, when nothing is live all
three print the same diagnostic on stderr, nothing on stdout, and exit `0`:

```console
$ grove-llm pick
grove app: no live leaves; this grove is done
```

`resolve` turns a reference into a path. It searches live, `DONE` and
`ABANDONED` entries alike, and accepts a permanent key (`[n]`, bare `n`, or
`[n]-slug`), a full `<slug>-k<key>` handle, or a bare slug:

```console
$ grove-llm resolve 1
/home/you/app/.grove/01-requirements--auth-k1.md

$ grove-llm resolve auth-k1
/home/you/app/.grove/01-DONE-requirements--auth-k1.md
note: referenced task is retired (DONE): /home/you/app/.grove/01-DONE-requirements--auth-k1.md
```

A node resolves to its **directory**. Read its single `_<slug>.md` node file
for the brief; `resolve ship` finds the node by that filename too:

```console
$ grove-llm resolve ship-k3
/home/you/app/.grove/04-k3
```

A bare slug that several entries share is ambiguous, and the diagnostic lists
each match's key so you can re-query by key. Not-found says so. Both still exit
`0`, because "which entry is this?" has an answer either way:

```console
$ grove-llm resolve auth
resolve: reference "auth" is ambiguous; re-query by key:
  [1] /home/you/app/.grove/01-requirements--auth-k1.md
  [2] /home/you/app/.grove/02-design--auth-k2.md
  [4] /home/you/app/.grove/05-review-design--auth-k4.md

$ grove-llm resolve nosuch
resolve: no entry matches reference "nosuch"
```

### Growing the tree

`root-init` scaffolds a brand-new grove: `.grove/`, the root `_BRIEF.md` node file,
and a first `requirements` leaf whose slug defaults to `plan`. It prints the
charter's path, then the leaf's. **Bare `grove` does this for you**, so you run
it by hand only to lay a tree down without launching a session:

```console
$ grove-llm root-init auth
/home/you/app/.grove/_BRIEF.md
/home/you/app/.grove/01-requirements--auth-k1.md
```

It refuses rather than merging into a grove that already exists:

```console
$ grove-llm root-init
Error: grove root already exists: /home/you/app/.grove
```

`leaf-add` appends work under a parent — `.` for the grove root, or a node by
key or path. `--kind` is required and **repeatable**, and one call adds one leaf
per `--kind` in the order given, as a single unit, all carrying the same slug:

```console
$ grove-llm leaf-add . ship --kind impl
/home/you/app/.grove/03-impl--ship-k3.md

$ grove-llm leaf-add . auth --kind review-design --kind integrate-review-design
/home/you/app/.grove/04-review-design--auth-k4.md
/home/you/app/.grove/05-integrate-review-design--auth-k5.md
```

The kind is what tells a shape's steps apart, which is why they share the slug.
`finish` is the driver's own and is refused here:

```console
$ grove-llm leaf-add . x --kind finish
Error: `finish` is driver-reserved and cannot be created by `leaf-add`
```

`leaf-insert` puts a new leaf at the slot an existing entry holds, shifting that
entry and every later sibling up one. It takes exactly one `--kind`. The new leaf
gets a fresh key; the shifted subtrees keep their own names and keys, and **no
file contents are rewritten** — which is why the renumber summary on stderr also
warns that position-prefixed cross-references in your prose are yours to fix:

```console
$ grove-llm leaf-insert 3 spike --kind prototype
/home/you/app/.grove/03-prototype--spike-k6.md
leaf-insert spike: renumbered 3 siblings:
  03 -> 04  (04-impl--ship-k3.md)
  04 -> 05  (05-review-design--auth-k4.md)
  05 -> 06  (06-integrate-review-design--auth-k5.md)
cross-references to review (verb does not auto-rewrite):
```

**Decomposing an oversized leaf** is `leaf-decompose`: the leaf file becomes a
node directory `NN-k<key>/` *with its key preserved*, its body moves in as
`_<slug>.md`, and a first child is grown atomically so the node is never childless.
The child inherits the decomposed leaf's kind unless `--kind` overrides it:

```console
$ grove-llm leaf-decompose 04-impl--ship-k3.md api
/home/you/app/.grove/04-k3/_ship.md
/home/you/app/.grove/04-k3/01-impl--api-k7.md
```

`04-impl--ship-k3.md` is now `04-k3/`, still `k3`, so every reference to
the handle `ship-k3` still resolves — to the directory.

### Ending work

`leaf-retire` marks a live leaf done in place by adding the `DONE` infix. The
leaf keeps its position and key, and the file's contents are untouched:

```console
$ grove-llm leaf-retire .grove/01-requirements--auth-k1.md
/home/you/app/.grove/01-DONE-requirements--auth-k1.md
leaf-retire: two steps remain:
  1. commit this session's work, including this rename
  2. run the exact ending command from your launch prompt as your last action (`ravel end-session grove-continue` for Ravel; legacy v22 sessions keep `harness-dispatch exit`)
```

It refuses a brief, an already-`DONE` leaf, and an `ABANDONED` one — there is no
second retirement of anything.

`leaf-prune` marks abandoned work `ABANDONED` in place, and **this is the one
verb gated on you**. Grove never abandons planned work on its own; a session that
reaches the question stops and asks. Given a node it marks every *live* leaf in
the subtree, leaving `DONE` ones alone, because that work really was done:

```console
$ grove-llm leaf-prune 04-k3
/home/you/app/.grove/04-k3/01-ABANDONED-impl--api-k7.md
leaf-prune: two steps remain:
  1. commit this session's work, including this rename
  2. run the exact ending command from your launch prompt as your last action (`ravel end-session grove-continue` for Ravel; legacy v22 sessions keep `harness-dispatch exit`)
```

It refuses the grove root, because abandoning a whole workstream is a
branch-delete rather than a tree mark:

```console
$ grove-llm leaf-prune .grove
Error: cannot prune the grove root (abandoning a whole grove is a branch-delete, not a tree mark)
```

Pruning only a producer leaves its review leaf live and next, deliberately
uncheckable; see [Review composition](#usage-review-composition).

**Closing a node has no verb, and that is deliberate.** A node is never marked:
its done-ness *is* the absence of a live leaf anywhere in its subtree, so `pick`
walks past a fully terminal directory without being told to. What a close costs
is judgement rather than a rename — the session that retires the last child
checks the node-file brief's `Done when` against what the subtree delivered,
`leaf-add`s any nameable gap, promotes what is still relevant up to the parent
brief or a decision record, and names the node's handle in its commit message
alongside the leaf's. You review a close after the fact, in that diff.

### Cutting a research pair

Research is the one shape cut eagerly, in a single all-or-nothing call, because
independence is the property being bought:

```console
$ grove-llm leaf-add . protocols --kind research-a --kind research-b --kind combine-research
/home/you/app/.grove/06-research-a--protocols-k8.md
/home/you/app/.grove/07-research-b--protocols-k9.md
/home/you/app/.grove/08-combine-research--protocols-k10.md
```

If `research-a` cut `research-b` at the end of its own session, `b` would inherit
`a`'s framing and corpus — destroying the independence the pair is run for; and
three separate calls would leave a live prefix indistinguishable from a
deliberately hand-cut partial pair. There is no pair verb: `leaf-add` takes an
ordered list of kinds, so the three tokens are spelled by the methodology that
owns them rather than held in the binary.

<!-- tc: on -->

### Ending the session, and ending the grove

The exact canonical `ravel end-session grove-continue` command in the launch prompt is the
last action of a session. The session runs it after retirement and commit. Ravel discovers
the enclosing session from the inherited process context. The command takes no session ID.
Ravel ends and reaps the managed group, and it returns the opaque result of Grove.

A finish session first records teardown with `grove-llm record-teardown`. Then it executes
that same ending command. `record-teardown` checks the current epoch, and it requires the
tree to be gone. Without `GROVE_LAUNCH_DIR` it records nothing and exits 0. Outside a Ravel
session, `end-session` refuses and signals no other run.

After the reap, Grove restores the terminal and invalidates the epoch. Driver interruption
wins, including signals that arrive after the reap.

A successful Ravel status with exactly `grove-force-stop` then stops the loop without
completion. Grove exits 1 and leaves the remaining live work resumable. The force-stop
overrides teardown. It does not restore a removed tree, and it does not retire a leaf.

A teardown record otherwise finishes the loop. Without a teardown record, an
unsuccessful Ravel status stops the loop. Only an exact
`grove-continue` continues. A missing, unsupported or unreadable result stops the loop with
the leaf live. If a session returns to an interactive prompt, the supervisor can wait until
the harness ends.

Use the exact canonical `end-session grove-force-stop` command of the launch prompt to stop
the sequence explicitly. Use its continue command after ordinary retirement and commit. Use
it also after you record a completed finish teardown. Ravel returns either value unchanged.
Grove owns these instructions and meanings.

**Legacy live-driver compatibility:** Sessions that Grove 22.1.0 launched through dispatch
keep their original `harness-dispatch exit` command. They also keep their inherited exit,
admission and teardown controls. New source and skills never rewrite a running launch
prompt. Grove substitutes no new Ravel command into such a session.

`finish-commit` is the teardown, covered in [Finish](#usage-finish) below. It takes the stable
handle of the launched finish leaf. Under the tree lock, it revalidates two facts. The leaf
is really the live finish leaf, and no ordinary work appeared. Only then does it remove and
commit `.grove/`. On success it names the change that it made, on stderr:

```console
$ grove-llm finish-commit finish-k42
finish-commit finish-k42: committed as qrsuvwxy
$ ls .grove
ls: cannot access '.grove': No such file or directory
```

That commit covers the path `.grove/` alone. Anything else in your working copy stays
uncommitted and untouched. It is the **one verb here that commits**. The deletion that it
records is the only change in it.

It **does not** stand in for the human confirmation that the finish session owes you. It
enforces tree facts and VCS facts. It infers nothing about your consent. Both of its
refusals leave the tree exactly as it was. The next example is the stale-handle refusal.
[Finish](#usage-finish) below shows the live-work refusal:

```console
$ grove-llm finish-commit finish-k42
Error: `grove-llm finish-commit finish-k42`

Caused by:
    the requested finish leaf is no longer live
```

<!-- tc: pending because the conversion has not reached this part of the document -->

<a id="usage-review-composition"></a>
## Review composition and escalation

Review in Grove happens two ways, and only one of them is yours to see in the
tree. A session may spend at most **one in-session** fresh-context reviewer over
its whole leaf — *which* kinds may spend it, and on what, is methodology the
`grove` plugin's skills own (`references/execute.md` in the `grove` skill) and
this guide does not restate. That reviewer leaves no trace in `.grove/`. What
does, and what this section covers, is the other way: a **review leaf**, a whole
session of its own, standing beside the producer as a file you can read.

**A review chain is built one step at a time, by the session that needs the next
one.** There is no chain verb and no chain node — each step is an ordinary
`leaf-add`, performed as the *last act* of the session before it:

```sh
# the producer's last act, if its artifact needs an adversarial read
grove-llm leaf-add <parent> <stem> --kind review-<producer>

# the review's last act, if it found something worth acting on — but
# `leaf-insert <first blocking sibling entry>` instead, if there is one
grove-llm leaf-add <parent> <stem> --kind integrate-review-<producer>
```

**The integration is placed next to its review on purpose.** An
`integrate-review-*` step consumes what the review wrote down — findings anchored
to files and line numbers — and resolves them against the working tree as it
*then* stands, so anything that edits a cited file in between moves those lines
and the drift is **silent**, leaving the integrating session guessing what the
reviewer meant. A `review-*` step re-derives by contrast: its body names the
producer's stable handle, task commits name their work item by that handle, so it
finds the producer's commit and reads that diff against the current source. It
can land anywhere, which is why only one of the two hops needs care.

`leaf-add` appends at the *end* of the directory, so use `leaf-insert` for the
integration whenever there is a blocking sibling, and target **the first sibling
entry after the review whose subtree still holds live work**. *Entry*, not leaf:
`pick` descends a node directory in place, so a later sibling node with a live
leaf anywhere beneath it blocks too, and the **node directory** is the target,
never the live leaf inside it (that inserts at the wrong level). A later `DONE`
or `ABANDONED` leaf, **a node whose subtree is wholly terminal**, and the
driver's `finish` sentinel are all stepped over, so none of them blocks — and
when nothing blocks, `leaf-add` is exactly right, because the walk finishes the
review's own directory, including the leaf just appended to it, before any later
sibling of an ancestor. There is no exception: at the moment the leaf is cut, the
blocking work has not run and no leaf's eventual file set is part of its
contract, so nothing could establish one.

A review that finds nothing creates nothing and simply retires — that empty
triage session is what the lazy shape removes. But the bigger payoff is that the
**creating session writes the new leaf's body**: it can name the exact case the
producer could not cover, which is strictly more than a constructor rendering a
goal sentence from a handle could ever supply. An **integration's** body is the
exception, and it carries the review's *handle* rather than its findings: a body
that is the finding list makes that list the integration's charter, leaving it no
structural place to reject one. The findings are read from the review's own
commit instead.

The producer finishes only to a coherent **reviewable boundary**, **retires
itself, then** commits the artifact, the new review leaf and that retirement
together under its own handle, and hands control back to Grove. Retirement comes
before the commit because the `DONE` rename belongs to this task: commit first
and the rename is either left uncommitted or swept into the next task's change.
Nothing about the producer's leaf moves, so its stable handle and bytes are
preserved by construction.

Write the relationship into the new leaf's body by hand — `**Reviews:**
<producer-handle>`, or `**Integrates:** <review-handle>`. Grove's binaries
neither write nor read those lines; they are a convention for you and for the
session that picks the step up. A caller-owned review adapter may read
`**Reviews:**` and the `**Creator:**` line directly below it.

Ravel stores no run history and exports no run identity. A review needs explicit
caller-owned execution provenance or the owner's `**Creator:** declared
<provider>` assertion. A finishing session without provenance removes a stale
reference rather than inferring the creator from today's mapping or an earlier
attempt. Reviews lacking sufficient provenance may refuse until the owner
supplies it; diversity remains the owner's policy.

Legacy `**Creator:** run <run-id>` references belong to the original dispatch
store. A legacy live session may name its own `HARNESS_DISPATCH_RUN_ID` when it
finishes a producer. A caller adapter must handle those references explicitly;
Ravel has no record/lookup/observation command. Findings and durable provenance
belong to the caller and may survive `.grove/` in its own artifacts.

Grove then launches the command your policy selects for the review kind.
Whether that command differs in harness or model from the producer's is
**your** policy's to decide: Grove passes the kind and never sees the command
that comes back, so it compares no two commands, keeps no record of how a producer ran and emits no
diversity warnings. The tree guarantees a fresh session. The sample policy
sends each review to the other provider by its arrangement, and the Grove
caller-owned review adapter checks available creator provenance at launch.

Pruning only the producer leaves its review live and next, deliberately
uncheckable. To abandon the whole reviewed path, prune each of its live steps —
usually just the one, since a review leaf exists only because a producer decided
review was required.

## Two habits for the human in the loop

Both are yours rather than the session's — a session *is* the LLM and cannot
perform either on itself. The session-facing counterparts are already in the
methodology: naming the trade-off you want input on
([`plugins/grove/skills/grove/references/execute.md`](../plugins/grove/skills/grove/references/execute.md))
and giving a recommended answer per question
([`plugins/grove/skills/grove-requirements/grilling.md`](../plugins/grove/skills/grove-requirements/grilling.md)).

**Ask the LLM "WDYT" before committing.** When a question feels close to
settled, the easy default is to nod and move on. Don't — ask what it thinks,
explicitly, including when you already hold a strong view. Two things happen.
It produces a recommendation it would otherwise suppress out of deference,
informed by the bootstrap context (glossary, briefs, decision records,
research), which is exactly the evidence base you want surfaced before you
commit. And where its recommendation diverges from yours, that divergence is
the cheapest signal available that the question still has something to teach —
interrogate it rather than dismissing it.

**Ask for pushback when the LLM agrees too easily.** Models default to
agreement under social pressure. When one agrees with your proposal without
surfacing trade-offs, push back yourself: "what would push you toward the other
option?", "what breaks if we do it this way?", or simply "pushback please". The
grilling format is built around recommended-answers-with-evidence precisely to
make pushback structural rather than personal — when the recommendation cites
primary evidence, the pushback is a debate about that evidence rather than
about whose preference wins.

<a id="usage-driver-lease"></a>
## One driver per working tree

A working tree can have only one live Grove driver. A second `grove` in the same
tree exits immediately, names the canonical working tree, and leaves the existing
driver as owner:

```console
$ grove
Error: another Grove driver already owns /home/you/app; the existing Grove driver must stop before this one can start
```

It does not queue, because two drivers would issue two mandates for the same
task. Stop the first — `kill` on its `grove` process, or let it finish — and
rerun. Different jj workspaces are independent even when they share a repository;
path aliases and symlinks to the same tree are not, which is why the refusal
names the canonical path rather than the one you typed.

Ownership is held by a kernel lock, so normal exit, a panic, and process death
all release it. Restarting after a crash is ordinary continuation.

<a id="usage-workspace-layouts"></a>
## Supported workspace layouts

Grove resolves the working tree by walking up from your current directory to the
first `.jj/` directory, and keeps its own coordination files — the driver lease,
the session epoch and per-session launch directories — in that workspace's `.jj/grove/`. Three
consequences are worth knowing.

**Every jj workspace layout works.** Native (`jj git init`), colocated
(`jj git init --colocate`, where `.git` and `.jj` sit side by side), and
secondary workspaces created with `jj workspace add` are all ordinary to Grove.
A secondary workspace borrows the default workspace's repository, and Grove asks
jj which one that is rather than guessing — one more reason its binary has to be
on `PATH`.

**Each workspace is its own grove.** The lease, the control directory and the
`.grove/` tree all belong to the workspace you ran `grove` in, so two workspaces
sharing one repository run two independent drivers over two independent
workstreams. That is the behaviour to keep in mind before running `grove` in a
repository with several workspaces checked out — see
[Running Grove](#usage-running-grove).

**The control directory must be usable.** Grove creates `.jj/grove/` on start
and refuses if it cannot, naming the directory it could not use and then, under
`Caused by:`, what the filesystem said about it:

```console
$ grove
Error: the control directory /home/you/app/.jj/grove is not usable

It must exist and be writable before anything can coordinate through it. Check the permissions on the workspace's `.jj` directory.

Caused by:
    Permission denied (os error 13)
```

Nothing is created or changed when that refusal lands. Fix the permissions on
`.jj/` and rerun. Because the control directory lives under `.jj/` rather than in
the tracked working copy, jj never snapshots it, and Grove's coordination files
never turn up in a commit.

<a id="usage-finish"></a>
## Finish

When the last live leaf is retired, Grove materializes a `finish` leaf and
launches a session that proposes one complete finish cycle:

1. Promote durable knowledge from the briefs into the repository's normal docs,
   decision records, specs, or context files where it still belongs.
2. Tear `.grove/` down with `grove-llm finish-commit <finish-handle>`, which
   deletes the tree and records the deletion in one focused commit.
3. Record the teardown with `grove-llm record-teardown`, then end the run
   with the exact `ravel end-session grove-continue` command in its launch prompt as the last action. The driver reads the
   teardown record and stops the loop cleanly.

```console
grove: launching finish through Ravel — finish-k42
ravel: starting provider anthropic, model claude-opus-5-5, effort medium for kind "finish"
grove: grove finished — loop complete.
```

This is Grove's one routine human confirmation point, because it deletes the
workstream tree. An unanswered or declined proposal leaves the finish leaf live
and the current interactive session available. The agent yields its turn while
waiting for your response. If you explicitly stop the run before teardown,
the next `grove` returns to that same leaf. If new work appears after
the finish session launched, teardown refuses and names that work, leaving the
tree untouched for the next iteration:

```console
$ grove-llm finish-commit finish-k42
Error: cannot finish while live work remains: api-k7 (/home/you/app/.grove/04-k3/01-impl--api-k7.md)
```

**Integration is yours, and it comes after step 2.** Branch or bookmark
integration and working-tree teardown remain yours; Grove never creates, merges,
or removes them. Whoever integrates should do so after teardown has committed, so
the integrated history never carries `.grove/`.

### What teardown guarantees

Step 2 is a plain deletion followed by a path-scoped `jj commit`, and **Grove
implements no transaction around it**. It does not need one: Jujutsu snapshots
the working copy before every command and its operation log is the transaction
record, so the guarantees a hand-built transaction would offer are already
yours, from the tool that owns them.

What this means for you:

- The deletion commit touches only `.grove/`. Unrelated working-copy changes are
  preserved, staying uncommitted rather than being swept into the teardown.
- Before deleting anything, Grove revalidates that the live leaf really is the
  finish leaf you named and that no ordinary work has appeared, and refuses a
  task tree Jujutsu does not track — because the operation log can only restore
  what it tracks. A `.grove` that is a symlink to a directory elsewhere is
  refused too: Grove will not delete a tree that is not its own.
- If teardown fails part way, Grove stops and names the command that puts the
  tree back: `jj restore .grove` if the deletion is what failed, `jj undo` if the
  commit is. **No Grove-authored recovery runs.** Once the tree is back, fix what
  failed and rerun the same command with the same handle. Grove will not reset,
  rebase, or rewrite history on your behalf, so nothing you did outside Grove is
  discarded to unblock it.
- `jj op log` is where you look if you are unsure what happened. It is the record
  of every operation, including the snapshot that captured the deletion.
- Once `.grove/` is gone, that workstream is over. A later `grove` in the same
  tree starts a **new** grove rather than recovering the finished one — Grove
  reads no VCS history to tell "recover" from "start again".

For why these boundaries exist, see [ARCHITECTURE.md](ARCHITECTURE.md).

<a id="usage-coverage-map"></a>
## Coverage map

Every row of [the coverage inventory](specs/user-guide-coverage.md), and the
section of this guide that answers it. It is here so completeness is checkable
row by row rather than asserted in aggregate, and
`crates/grove/tests/user_guide_coverage.rs` compares the two documents, so a row
added to the inventory and forgotten here is a test failure rather than a silence.

| Row | Covered in |
|---|---|
| G1 | [Running Grove](#usage-running-grove) |
| G2 | [Running Grove](#usage-running-grove) |
| G3 | [Running Grove](#usage-running-grove) |
| G4 | [Running Grove](#usage-running-grove) |
| G9 | [Running a standalone task](#usage-standalone) |
| G6 | [Viewing a tree](#usage-viewing-tree) |
| G5 | [Stopping the loop](#stopping-the-loop) |
| L1 | [Growing the tree](#growing-the-tree) |
| L2 | [Reading the tree](#reading-the-tree) |
| L3 | [Reading the tree](#reading-the-tree) |
| L4 | [Reading the tree](#reading-the-tree) |
| L5 | [Reading the tree](#reading-the-tree) |
| L6 | [Growing the tree](#growing-the-tree) |
| L7 | [Growing the tree](#growing-the-tree) |
| L8 | [Growing the tree](#growing-the-tree) |
| L9 | [Ending work](#ending-work) |
| L10 | [Ending work](#ending-work) |
| L11 | [Ending the session, and ending the grove](#ending-the-session-and-ending-the-grove) |
| L12 | [Ending the session, and ending the grove](#ending-the-session-and-ending-the-grove) |
| L13 | [The `grove-llm` verbs over the tree](#usage-tree-verbs) |
| J1 | [Running Grove](#usage-running-grove) |
| J2 | [If the tree is not jj-enabled](#if-the-tree-is-not-jj-enabled) |
| J3 | [Running Grove](#usage-running-grove) |
| J4 | [What happens in a session](#usage-session-lifecycle) |
| J5 | [Stopping the loop](#stopping-the-loop) |
| J6 | [The task tree and its filename grammar](#usage-task-tree) |
| J7 | [Growing the tree](#growing-the-tree) |
| J8 | [Review composition and escalation](#usage-review-composition) |
| J9 | [Cutting a research pair](#cutting-a-research-pair) |
| J10 | [Ending work](#ending-work) |
| J11 | [Ending work](#ending-work) |
| J12 | [Undoing a mistake](#undoing-a-mistake) |
| J13 | [One driver per working tree](#usage-driver-lease) |
| J14 | [Supported workspace layouts](#usage-workspace-layouts) |
| J15 | [Finish](#usage-finish) |
| J16 | [Finish](#usage-finish) |

# tmux-owning frontends — post-mortem survey

A focused review of how existing tools **own and drive a tmux session as a
backend** — the "dashboard + one window/session per long-running child process"
pattern grove has committed to (root BRIEF + 010-plan D2/D3). Organised, like its
sibling `in-repo-issue-tracker-postmortems.md`, around **what went wrong** after
contact with real multi-session use.

This survey does **not** evaluate in-process pty or zellij as candidates: that
fork is already decided (grove owns a tmux session — 010-plan D2). The job here
is *how*, not *whether*. Where a surveyed tool answers a different question than
grove is asking, that is recorded as a contrast, not a candidate.

## Audience

Two downstream leaves, whose questions structure the Synthesis:

- **030 — decide tmux integration mechanics.** Socket choice, control mode vs
  plain scripting, launch/attach sequence, owned config. Produces the binding
  ADR(s).
- **040 — harness pane.** Window lifecycle (create / name / tear down / learn a
  child exited) and the crash-isolation + persistence guarantee that D2 rests
  on.

The decision grove is de-risking, from the root BRIEF: grove **owns** a
dedicated tmux session; the TUI dashboard is window 0; each harness is a window
grove creates. Chosen over in-process pty for session persistence + crash
isolation.

## The spectrum

The surveyed systems fall on one axis — **how tightly the frontend couples to a
multiplexer** — and grove's position on that axis is what makes most of the
design choices fall out:

```
no multiplexer            own a session        own sessions on        frontend IS
(suspend-and-run)         (not a server)       the SHARED server      the tmux client
─────────────────────────────────────────────────────────────────────────────────────
lazygit, gh-dash    →     sesh, tmuxinator,  → claude-squad, uzi  →   iTerm2  tmux -CC
                          smug
zero persistence          trivial persistence   accidental             full persistence,
single tty, blocking      (it's a normal        persistence (leans     native rendering,
                          tmux session)         on the user's daemon)   heavy protocol
```

grove sits between the declarative scripters and the orchestrators: it wants to
**own the server** (for isolation and persistence) but let **tmux do the
rendering** (the dashboard is a tmux window, not a native-UI tmux client). That
position — server-owner, not protocol-frontend — is what makes "dedicated
socket + plain scripting + own config" the evidence-backed default, and makes
control mode (`-CC`) unnecessary. Each finding below is tagged with the
downstream question (Q1–Q6, defined in the Synthesis) it informs.

---

## 1. AI-agent orchestrators — claude-squad, uzi

The closest prior art: tools that spawn each agent into its own tmux
session/window and own it. Both are Go. The headline finding: both take the
*simplest possible* tmux integration — the **default tmux server**, **plain CLI
scripting** (not control mode), **one session per agent** — and almost every
reported failure traces back to the shared-server choice.

### claude-squad (`smtg-ai/claude-squad`)

**Summary.** A Bubble Tea TUI that manages multiple terminal agents (Claude
Code, Codex, Aider, Gemini), each in its own git worktree + tmux session. ~7.7k
stars, Go. Repo: <https://github.com/smtg-ai/claude-squad>. The entire tmux
integration lives in one file:
<https://github.com/smtg-ai/claude-squad/blob/main/session/tmux/tmux.go>
(verified: no `-L`/`-S`, no `-CC`, no `$TMUX` reference anywhere in the file).

**Ownership model.** **Default tmux server / default socket** — no `-L`/`-S`
flag. One **session per agent**, created detached and named with a
`claudesquad_` prefix:

```go
cmd := exec.Command("tmux", "new-session", "-d", "-s", t.sanitizedName, "-c", workDir, t.program)
```

The launched session runs *under a PTY* (`creack/pty`); after creation it polls
`tmux has-session -t=<name>` with exponential backoff, then sets
`history-limit 10000` and `mouse on` per-session via `set-option -t`. Existence
checks deliberately use `-t=` (exact) because `-t name` is a prefix match
(in-source comment in `DoesSessionExist()`).

The user-facing "attach" is a PTY running `tmux attach-session -t <name>`
(`Restore()`), with raw bytes copied between `os.Stdin/Stdout` and the pty;
detach is intercepted in-process on **Ctrl-Q** (ASCII 17), not tmux's own detach
key. Keystrokes to the agent are written as raw bytes straight to the pty
(`SendKeys` → `t.ptmx.Write`), *not* via `tmux send-keys`. Status/preview is read
with `tmux capture-pane -p -e -J -t <name>` (`CapturePaneContent`). **Plain
scripting, not control mode (Q2).**

**Walk-away check (Q6).** Because the agent sessions are children of the
**default tmux server** (a long-lived daemon independent of the `cs` process),
the sessions and their agent processes **survive the TUI dying** — `cs` only
holds a pty attached via `attach-session`, and closing it just detaches. Session
metadata is also persisted to disk (`session/storage.go`, `InstanceStorage`) and
re-bound on next launch via `Restore()`/`DoesSessionExist()`. A user can
re-attach manually with plain `tmux attach -t claudesquad_<name>`. **Caveat:**
`Restore()` after a *reboot* fails because the default tmux server itself does
not survive a reboot — issue
[#212](https://github.com/smtg-ai/claude-squad/issues/212). So "persistence"
means *survives the TUI dying*, not *survives a reboot*.

**Failure modes (real use).**

- **Shared-server env-var leakage (Q1).** Sessions created against an
  already-running default server don't inherit the invoking shell's custom env
  vars (tmux only propagates its `update-environment` allowlist), so e.g. Codex
  fails with `Missing environment variable: CPA_API_KEY`. Primary source:
  [#277](https://github.com/smtg-ai/claude-squad/issues/277) — **verified**; the
  issue's *own* proposed fix is "use a dedicated tmux socket for claude-squad."
  A direct consequence of reusing the default server.
- **Default-server collateral damage (Q1).** The maintainers' teardown script
  runs `tmux kill-server` — which kills the user's *entire* default tmux server,
  not just `cs`'s sessions:
  <https://github.com/smtg-ai/claude-squad/blob/main/clean_hard.sh>. (The
  in-binary `cs reset` is gentler — `CleanupSessions` only kills
  `claudesquad_`-prefixed sessions.)
- **send-keys / prompt race (Q5-adjacent).** Prompt text is written to the pty
  immediately after `Start()` returns, before the CLI finishes init and enters
  raw mode, so the prompt is silently dropped. Root-cause with file:line in
  [#266](https://github.com/smtg-ai/claude-squad/issues/266).
- **"timed out waiting for tmux session …"** on first launch — the `has-session`
  poll never succeeds, often under WSL or when env is off:
  [#115](https://github.com/smtg-ai/claude-squad/issues/115),
  [#132](https://github.com/smtg-ai/claude-squad/issues/132),
  [#96](https://github.com/smtg-ai/claude-squad/issues/96),
  [#51](https://github.com/smtg-ai/claude-squad/issues/51).
- **`capture-pane: exit status 1`** against a session that died / never
  attached — a recurring class:
  [#216](https://github.com/smtg-ai/claude-squad/issues/216),
  [#218](https://github.com/smtg-ai/claude-squad/issues/218),
  [#189](https://github.com/smtg-ai/claude-squad/issues/189),
  [#51](https://github.com/smtg-ai/claude-squad/issues/51). (#189: adding Claude
  `.claude` hooks broke tmux loading — an interaction between owned-tmux scraping
  and the agent's own startup output.)
- **Status detection is screen-scraping.** "Running vs ready" is inferred by
  string-matching pane content (e.g. `"No, and tell Claude what to do
  differently"`) in `HasUpdated()` — brittle against CLI UI changes. No filed
  break found, but the mechanism is in `tmux.go`.
- **`$TMUX` / nesting (Q3): no primary source found.** No `$TMUX` handling
  anywhere in the source, no README warning about launching `cs` from inside
  tmux. A silence — neither a documented guard nor a reported bug.
- **Custom config / clobbering (Q4).** Ships **no** tmux config; sets only two
  per-session options scoped to its own sessions. Because it reuses the default
  server, the user's `~/.tmux.conf` *is* loaded into cs's sessions (relevant to
  the env-var bug #277).

### uzi (`devflowinc/uzi`)

**Summary.** A CLI (not a persistent TUI) that fans out *large numbers* of agents
in parallel, each in its own worktree + tmux session, with optional per-agent
dev-server windows. ~580 stars, Go. Repo:
<https://github.com/devflowinc/uzi>. Core logic:
<https://github.com/devflowinc/uzi/blob/main/cmd/prompt/prompt.go>.

**Ownership model.** **Default tmux server** (no `-L`/`-S`). One **session per
agent**, named `agent-<project>-<githash>-<randomname>`. Within a session,
named windows: the first window is renamed `agent`, an optional `uzi-dev` window
runs the dev server. All driving is plain shell-string scripting (Q2), built
with `fmt.Sprintf` and run via `sh -c` (a quoting/injection hazard):

```go
cmd        = fmt.Sprintf("tmux new-session -d -s %s -c %s", sessionName, worktreePath)
renameCmd  = fmt.Sprintf("tmux rename-window -t %s:0 agent", sessionName)
newWindow  = fmt.Sprintf("tmux new-window -t %s -n uzi-dev -c %s", ...)
tmuxCmd    = fmt.Sprintf("tmux send-keys -t %s:agent '%s \"%%s\"' C-m", sessionName, ...)
```

**Window lifecycle (Q5).** Window created via `new-window -t <session> -n
uzi-dev`; agent window via `rename-window -t <session>:0 agent`. **No
window-closed hook and no exit notification.** Status is *polled* by other
commands scraping the pane: `uzi ls` runs `tmux capture-pane -t <session>:agent
-p` and string-matches `"esc to interrupt"`/`"Thinking"`
([cmd/ls/ls.go](https://github.com/devflowinc/uzi/blob/main/cmd/ls/ls.go)).
Active-session enumeration is `tmux has-session` over names in its JSON state
file. Teardown is `tmux kill-session -t <name>` then remove the worktree
([cmd/kill/kill.go](https://github.com/devflowinc/uzi/blob/main/cmd/kill/kill.go)).

**Walk-away check (Q6).** uzi is *inherently* walk-away-safe because it is **not
a long-running owner at all** — `uzi prompt` creates detached sessions on the
default server and exits; the agents keep running in the daemon. `uzi ls`/`uzi
auto` are separate short-lived pollers. With uzi uninstalled, the `agent-*`
sessions persist and the user can `tmux attach` manually. (uzi ships no attach
command.)

**Failure modes (real use).**

- **Window-index assumption breaks teardown/launch (Q4/Q5).** uzi hard-codes
  `rename-window -t <session>:0` and `send-keys -t <session>:agent`. If the
  user's `~/.tmux.conf` sets `base-index 1` / `pane-base-index 1`, the `:0`
  target doesn't exist and uzi "died when it tried to rename the session."
  Primary source:
  [#11 "Tmux Renaming fails if pane idx isn't 0"](https://github.com/devflowinc/uzi/issues/11)
  — **verified**. The reporter raises exactly grove's design question: *"might be
  worth considering how uzi inherits the user tmux config and if it would make
  sense to have its own configuration to keep things clean."* A direct argument
  for an owned socket + config (Q1, Q4).
- **Owned config / clobbering (Q4).** Ships no config; inherits the user's
  `~/.tmux.conf` on the shared server, which is what *causes* #11.
- **Concurrent-map crash under fan-out.** `uzi auto` panics with `fatal error:
  concurrent map writes` in `AgentWatcher.hasUpdated` when watching multiple
  sessions — polling many sessions in goroutines without synchronization.
  [#9](https://github.com/devflowinc/uzi/issues/9).
- **Brittle prompt feeding.** Prompt sent via `send-keys ... C-m` interpolated
  through `sh -c`; timing- and quoting-sensitive (same class as cs #266). No
  dedicated uzi lost-prompt issue found — **no primary source found**.
- **`$TMUX` / nesting (Q3): no primary source found.** No `$TMUX` handling, no
  README warning.

### Close siblings (brief)

- **Tmux-Orchestrator** (`Jedward23/Tmux-Orchestrator`) — not a binary owner; a
  *prompt/script pattern* where Claude itself runs `tmux new-session`/`send-keys`
  against the default server via a `send-claude-message.sh` helper. Same
  shared-server model; no socket isolation.
- **claude_code_agent_farm** (`Dicklesworthstone/claude_code_agent_farm`) —
  Python orchestrator for 20+ agents using "real-time tmux monitoring" and
  **file/lock-based coordination** rather than tmux IPC. Evidence that at scale,
  people add an *out-of-band* coordination channel instead of trusting tmux
  scraping. (Claim from repo description, not a source read.)

---

## 2. iTerm2 tmux control mode (`tmux -CC`)

The deep pattern: the **frontend becomes the tmux client**, speaking a
line-oriented control protocol on stdin/stdout, and renders each tmux window as
native macOS chrome. The relevant question for grove is **Q2 — is this worth the
complexity?**

**Summary.** `tmux -CC` (control mode, doubled to disable echo) makes iTerm2 the
tmux client; instead of tmux drawing a UI over a PTY, client and server exchange
a text protocol, and iTerm2 maps tmux windows/panes onto native tabs/splits. The
tmux man page: control mode "allows applications to communicate with tmux using a
simple text-only protocol" with command output wrapped in `%begin`/`%end`/`%error`
guard lines
([tmux.1 CONTROL MODE](https://raw.githubusercontent.com/tmux/tmux/master/tmux.1)).
The tmux changelog frames its intent: "Control mode … Currently more useful to
users of iterm2"
([CHANGES](https://raw.githubusercontent.com/tmux/tmux/master/CHANGES)). The
integration's *design goal* was ergonomic, not architectural: "not have to
sacrifice ^B … everything you want to do can be done with native iTerm2
interactions" (iTerm2 Best Practices wiki).

**Ownership model.** The tmux **server is owned by tmux, not iTerm2** — `tmux
-CC` is an ordinary client that happens to speak the control protocol. iTerm2's
`TmuxGateway` "interprets the control mode stream and converts it into actions
within iTerm2" ([DeepWiki](https://deepwiki.com/gnachman/iTerm2/5.2-tmux-integration)).
Importantly, `-C`/`-CC` are documented purely as client startup modes, orthogonal
to `-L`/`-S` socket selection ([tmux.1](https://raw.githubusercontent.com/tmux/tmux/master/tmux.1))
— **control mode does not imply a dedicated socket (Q1).**

**Walk-away check (Q6).** Sessions survive iTerm2 quitting or the SSH link
dropping, because the server is a separate daemon: "when iTerm2 quits or the ssh
session is lost, tmux keeps running," and re-attach with `tmux -CC attach`
restores the windows
([iterm2.com/documentation-tmux-integration.html](https://iterm2.com/documentation-tmux-integration.html)).
Crucially the left-behind session is a **normal, fully attachable tmux session** —
control mode is a property of the *client connection*, not the session — so it
re-attaches with plain `tmux attach` from any ordinary terminal too. The client
exit is signalled to the frontend via `%exit [reason]`
([tmux.1](https://raw.githubusercontent.com/tmux/tmux/master/tmux.1)).

**Window lifecycle (Q5).** The frontend never polls; tmux *pushes*
notifications (guaranteed never to occur inside an output block). From
[tmux.1 CONTROL MODE](https://raw.githubusercontent.com/tmux/tmux/master/tmux.1):
`%window-add window-id`, `%window-close window-id`, `%window-renamed`,
`%window-pane-changed`, `%layout-change`, `%output pane-id value`
(octal-escaped), plus `%session-changed`/`%sessions-changed`. This pushed stream
is control mode's headline advantage over scripting.

**What it buys vs. costs (Q2 — central).**

*Buys:* a **live, pushed event stream** (window open/close, layout change, pane
output) that plain scripting cannot deliver without polling; full frontend
ownership of rendering; format **subscriptions** in modern tmux ("subscribe to a
format and be notified of changes rather than having to poll" —
[CHANGES](https://raw.githubusercontent.com/tmux/tmux/master/CHANGES)).

*Costs:*

- **A stateful stream parser.** You must demux interleaved
  `%begin`/`%end`/`%error` command replies against asynchronous
  `%output`/`%layout-change`/`%window-*` notifications, decode octal-escaped
  bytes, and track window/pane identity — iTerm2 needed dedicated machinery
  (`TmuxGateway`, `TmuxStateParser`,
  [DeepWiki](https://deepwiki.com/gnachman/iTerm2/5.2-tmux-integration)).
- **Version coupling.** The protocol evolved under the frontend's feet:
  flow-control retrofit (`pause-after`/`%pause`/`%continue`/`%extended-output`),
  client-sizing change ("Control mode clients now do not affect session sizes
  until they issue `refresh-client -C`"), and the tmux 3.3 terminfo change to
  `tmux-256color` that broke shell-integration passthrough until users added
  `set-option -g allow-passthrough on`
  ([CHANGES](https://raw.githubusercontent.com/tmux/tmux/master/CHANGES); iTerm2
  Best Practices wiki). The man page documents **no version-negotiation
  handshake** — **no primary source found** for protocol version negotiation,
  which is itself the finding: the client must empirically know what its tmux
  supports.
- **In-band signaling fragility.** Control mode is entered via a DCS escape
  multiplexed in the same byte stream, so any layer that mangles bytes breaks it:
  [#8953](https://gitlab.com/gnachman/iterm2/-/issues/8953) (over
  SSH→Windows→WSL the control traffic prints as plain text because the leading
  `\033P1000p` DCS got trimmed). The wiki warns *against* running `tmux -CC`
  directly from a command line, and against mosh.

*Concrete regressions from the coupling/stream model:*
[#11174](https://gitlab.com/gnachman/iterm2/-/issues/11174) — control mode
"often fails to launch window when creating or attaching," ~50% success on some
hosts (no root cause stated — a silence about startup ordering);
[#5021](https://gitlab.com/gnachman/iterm2/-/issues/5021) — cannot detach when
network unstable; [#7089](https://gitlab.com/gnachman/iterm2/-/work_items/7089) —
improperly restores multi-tabbed window into separate windows;
[#10530](https://gitlab.com/gnachman/iterm2/-/work_items/10530) — OSC 52
clipboard broken under control mode;
[#1999](https://gitlab.com/gnachman/iterm2/-/issues/1999) — early `tmux -C`
crashed iTerm2 outright.

**Nesting / `$TMUX` (Q3): no authoritative source found.** Neither the iTerm2
docs nor the man-page CONTROL MODE section address `-CC` launched inside an
existing tmux. The wiki advises *not* to invoke `tmux -CC` manually; a tracker
snippet calls nested `-CC` flags "confusing to iTerm2." Treat nested `-CC` as
undefined.

**Evidence gaps (explicit).** Protocol version negotiation: none found. Nested
`-CC`/`$TMUX`: none found. The Best Practices wiki body is JS-rendered;
quotes attributed to it are from the search index, not a verbatim page read.

---

## 3. Declarative session managers — sesh, tmuxinator, smug

The lightweight, "fire-and-forget scripter" end. They do not run a persistent
supervisor, do not use control mode, and do not own a private server. They shell
out a sequence of `new-session -d` / `new-window` / `send-keys`, then
`attach-session` or `switch-client` and exit. Everything they build is an
ordinary session on the user's default server. This is the simple baseline:
the manager is needed only at *creation* time; afterwards there is nothing left
to crash or reconcile.

### sesh (`joshmedeski/sesh`)

**Summary.** "Smart tmux session manager," Go, the de-facto successor to the
author's `t-smart-tmux-session-manager`, widely used with fzf. Repo:
<https://github.com/joshmedeski/sesh>.

**Ownership model — default server, plain scripting.** No `-L`/`-S` anywhere in
[`tmux/tmux.go`](https://github.com/joshmedeski/sesh/blob/main/tmux/tmux.go);
every method calls the bare `tmux` binary. Creation:
`tmux new-session -d -s <name> -c <dir>`. The connect path
([`connector/tmux.go`](https://github.com/joshmedeski/sesh/blob/main/connector/tmux.go))
creates, optionally fires startup commands, then hands to `SwitchOrAttach`.

**`$TMUX` / nesting (Q3) — done right by construction.**

```go
func (t *RealTmux) IsAttached() bool { return len(t.os.Getenv("TMUX")) > 0 }
```

and in
[`tmux/switch_or_attach.go`](https://github.com/joshmedeski/sesh/blob/main/tmux/switch_or_attach.go):
inside tmux (`$TMUX` set) → `switch-client -t`; outside → `attach-session -t`.
The canonical guard against the `sessions should be nested with care, unset
$TMUX to force` error
([tmux/tmux#3124](https://github.com/tmux/tmux/issues/3124)).

**Walk-away check (Q6).** Trivially survives — a one-shot CLI
(`seshcli/connect.go`), not a daemon; the created session is a normal
default-server session with no sesh process attached after exit.

**Window lifecycle (Q5).** Interface exposes `NewWindow`/`SelectWindow`/
`NextWindow` — creation and naming only. **No window-exit tracking** anywhere.

**Config (Q4).** Passes no `-f`; pure consumer of the user's ambient
`~/.tmux.conf`. No clobber, no layering.

**Failure mode.** `SendKeys` dispatched immediately after `NewSession` with **no
readiness wait** — the same swallow-the-keys race as its peers, but inferred from
the code path; **no sesh-numbered issue found** (explicit silence).

### tmuxinator (`tmuxinator/tmuxinator`)

**Summary.** "Manage complex tmux sessions easily," Ruby, the oldest and
most-starred (2010-era YAML project manager). Repo:
<https://github.com/tmuxinator/tmuxinator>.

**Ownership model — default server by default; socket opt-in (Q1).** From
[`lib/tmuxinator/project.rb`](https://github.com/tmuxinator/tmuxinator/blob/master/lib/tmuxinator/project.rb):

```ruby
def tmux; "#{tmux_command}#{tmux_options}#{socket}"; end
def tmux_command; yaml["tmux_command"] || "tmux"; end
def socket
  if socket_path then " -S #{socket_path}"
  elsif socket_name then " -L #{socket_name}" end
end
```

Default is bare `tmux`; a dedicated socket is opt-in via `socket_name`/
`socket_path` YAML keys. Creation detached; panes via `send-keys ... C-m`.

**`$TMUX` / nesting (Q3).** In the rendered template
[`template.erb`](https://github.com/tmuxinator/tmuxinator/blob/master/lib/tmuxinator/assets/template.erb):
`if [ -z "$TMUX" ]; then … attach-session … else … switch-client … fi`.
Identical correct guard to sesh.

**Walk-away check (Q6).** Survives trivially — renders a shell script, runs it,
exits. No daemon.

**Failure modes (the richest record of the three).**

- **send-keys racing shell init (Q2).**
  [#371 "A slow-starting shell causes startup commands to never happen"](https://github.com/tmuxinator/tmuxinator/issues/371):
  "This delay is enough to swallow the send-keys input that tmuxinator sends, so
  nothing ends up happening." The canonical post-mortem for the whole class —
  plain `send-keys` has no shell-ready handshake. Related:
  [#481](https://github.com/tmuxinator/tmuxinator/issues/481),
  [#329](https://github.com/tmuxinator/tmuxinator/issues/329).
- **Layout fragility on first run.**
  [#568](https://github.com/tmuxinator/tmuxinator/issues/568) — `main-vertical`
  comes out reversed on the *first* project of a session: layout commands applied
  while the detached session still has default 80×24 dims before a real client
  size is known. Corroborating: #557, #296, #552, #320, #651 (all linked from
  the tracker).
- **Config (Q4).** No `-f` sourcing; depends on whatever the default server
  loaded. `kill-session` only on explicit `stop`, gated by `tmux_has_session?` on
  start — no surprise clobber.
- **Window lifecycle (Q5).** No window-exit tracking in the template — builds
  windows + runs hooks, no exit/watch logic.

### smug (`ivaaaan/smug`)

**Summary.** "Session manager and task runner for tmux," Go, "inspired by
tmuxinator and tmuxp." Repo: <https://github.com/ivaaaan/smug>.

**Ownership model — default server; socket opt-in (Q1).** From
[`tmux.go`](https://github.com/ivaaaan/smug/blob/master/tmux.go), a `-S`/`-L`
flag is added *only if configured*; default is the bare server. Creation
`tmux new -Pd`, windows `neww -Pd`, commands `send-keys ... Enter`.

**`$TMUX` / nesting (Q3).** In
[`smug.go`](https://github.com/ivaaaan/smug/blob/master/smug.go): inside tmux →
`switch-client`; outside → `attach`; inside-but-attach-disabled → no-op.
**Caveat:** its inside-tmux detection
([`context.go`](https://github.com/ivaaaan/smug/blob/master/context.go)) is
`os.Getenv("TERM") == "screen" || tmux` — the `TERM == "screen"` clause can
false-positive outside any multiplexer, mis-choosing `switch-client`. No filed
issue (explicit silence); visible in source.

**Walk-away check (Q6).** Survives trivially — Go one-shot CLI, no daemon.

**Failure modes.**

- **send-keys racing shell init (Q2).**
  [#132 "Commands not being executed, keys sent too fast"](https://github.com/ivaaaan/smug/issues/132):
  "The commands … are being ran so fast that my bash prompt hasn't even started."
  Same root cause as tmuxinator #371.
- **No state reconciliation.**
  [#131 "Adding -i flag duplicates all windows in session"](https://github.com/ivaaaan/smug/issues/131)
  — injecting into an existing session re-emits `neww` with no diff against
  current server state (5 configured → 10 created). Where plain scripting gets
  fragile.
- **Window lifecycle (Q5).** A `Stop` path kills on demand, but no monitoring of
  window/pane death. Create + exit.
- **Config (Q4).** Ambient default-server config; no `-f` in the default path.

---

## 4. Suspend-and-run TUIs — lazygit, gh-dash (the multiplexer-avoiders)

**Headline finding:** neither lazygit nor gh-dash owns or spawns a multiplexer.
Both run children by **suspending their own TUI and giving the child the same
terminal**. The only tmux involvement is *user-authored*. This is strong
evidence that a successful TUI can avoid owning a multiplexer entirely — and the
price it pays (zero child persistence) is exactly the gap that motivates grove's
decision to own one. Informs Q2, Q5, Q6.

**The two tools.** lazygit — `github.com/jesseduffield/lazygit`, Go, ~78.6k
stars, renders with its own fork of **gocui**. gh-dash —
`github.com/dlvhdr/gh-dash`, Go, ~11.7k stars, built on **Bubbletea**.

**Subprocess / handoff model.** lazygit suspends the TUI, runs the child in the
same terminal, resumes — `runSubprocessWithSuspense` in
[`pkg/gui/gui.go`](https://github.com/jesseduffield/lazygit/blob/master/pkg/gui/gui.go):
`gui.suspend()` (gocui releases the terminal) → child wired to the real
`os.Stdin/Stdout/Stderr` → `gui.resume()`. **No PTY, no new window/pane** — the
child reuses lazygit's tty directly. The `promptToReturnFromSubprocess`
("press enter to return") pause exists *because* the child shares the screen
([docs/Config.md](https://github.com/jesseduffield/lazygit/blob/master/docs/Config.md)).
A repo-wide search finds tmux only in docs/config examples, never in execution
logic.

gh-dash uses the identical model via Bubbletea's `tea.ExecProcess`
(`internal/tui/modelUtils.go`): runs the command under `$SHELL -c`, which
`tea.ExecProcess` "runs … in a blocking fashion, effectively pausing the Program
… useful for spawning other interactive applications such as editors and shells"
([pkg.go.dev/charmbracelet/bubbletea](https://pkg.go.dev/github.com/charmbracelet/bubbletea)).

**gh-dash does NOT integrate with tmux — but ships tmux *examples* for users.**
Its docs include keybindings like
`command: tmux new-window -c {{.RepoPath}} 'nvim …'`
([gh-dash.dev/configuration/examples](https://www.gh-dash.dev/configuration/examples/)).
This is **the user delegating to tmux through gh-dash's generic shell-out**, not
gh-dash managing panes. The mechanics matter: `tmux new-window` *returns
immediately*, so `tea.ExecProcess` unblocks instantly and the dashboard resumes
while the real work lives in a sibling tmux window the dashboard never tracks.
Contrast `cd {{.RepoPath}} && lazygit`, which blocks the dashboard for the whole
session. **Users reach for `tmux new-window` precisely to get the
persistence/parallelism the suspend model lacks** — concrete evidence that
multiplexer ownership addresses a real gap.

**The tradeoff that motivates owning tmux (Q5/Q6).** Suspend-and-run gives **no
crash isolation and no persistence**: the child is a foreground process sharing
the parent's tty, so if the terminal dies the child dies with it, only one child
is visible at a time, and the parent is blocked for the child's whole lifetime.
This is the direct answer to Q5 — a TUI *can* run children without any window
abstraction — and to Q6 — the cost is the persistence grove wants.

**Failure modes (suspend/restore boundary).** lazygit's tracker is full of
post-mortems, all from the same root cause (a suspend-and-run TUI must perfectly
save/restore raw-mode terminal state and the controlling-tty foreground process
group): permanent suspension when nested under another TUI
([#4320](https://github.com/jesseduffield/lazygit/issues/4320),
[#3937](https://github.com/jesseduffield/lazygit/issues/3937),
[#3903](https://github.com/jesseduffield/lazygit/issues/3903)); terminal
corruption opening an editor
([#437](https://github.com/jesseduffield/lazygit/issues/437));
`promptToReturnFromSubprocess` not firing so the child's output is clobbered
([#1915](https://github.com/jesseduffield/lazygit/issues/1915)); intermittent
"terminal is not fully functional"
([#3704](https://github.com/jesseduffield/lazygit/issues/3704)). **No primary
source found** for equivalent gh-dash corruption, but it inherits the same risk
class through `tea.ExecProcess` (cf. Bubbletea
[#431](https://github.com/charmbracelet/bubbletea/issues/431)).

**Running *inside* tmux.** lazygit: the standard truecolor-inside-tmux problem —
colours render wrong unless tmux sets `default-terminal "tmux-256color"` +
`terminal-features ",${TERM}:RGB"`
([#3668](https://github.com/jesseduffield/lazygit/issues/3668)); clipboard via an
OSC-52 `copyToClipboardCmd` with explicit `screen|tmux` passthrough escaping
(docs/Config.md). gh-dash: **no tmux-specific issue found**.

---

## Synthesis — answers for 030 and 040

Each answer states what the evidence shows, then the recommendation it implies
for grove. grove's position (own the server, let tmux render — §The spectrum)
is what makes these fall out.

### For 030 — decide integration mechanics

**Q1. Dedicated socket (`tmux -L grove`) vs default server.**
*Evidence:* the orchestrators that used the **default server** produced the
survey's worst bugs — env-var non-inheritance (cs
[#277](https://github.com/smtg-ai/claude-squad/issues/277), whose own fix is
"use a dedicated socket"), `tmux kill-server` collateral damage
([clean_hard.sh](https://github.com/smtg-ai/claude-squad/blob/main/clean_hard.sh)),
user-config bleed-through breaking window targeting (uzi
[#11](https://github.com/devflowinc/uzi/issues/11)), and no-survive-reboot (cs
[#212](https://github.com/smtg-ai/claude-squad/issues/212)). The declarative
managers make a socket opt-in and most users never set it. Control mode is
orthogonal to socket choice
([tmux.1](https://raw.githubusercontent.com/tmux/tmux/master/tmux.1)).
*Recommendation:* **grove owns `tmux -L grove`.** A private socket isolates
kills (grove can tear down its own server without touching the user's tmux),
guarantees a known base-index/config (kills uzi #11 by construction), and avoids
inheriting the user's `~/.tmux.conf`. **Residual concern for 030:** a fresh
server captures the *launching process's* environment at start; sessions created
later still face tmux's `update-environment` allowlist (cs #277's root cause). If
harness windows need specific env, pass it explicitly at `new-window`/
`new-session` time (`-e`) rather than relying on inheritance.

**Q2. Control mode (`-CC`) vs plain scripting.**
*Evidence:* **none** of the orchestrators or declarative managers use control
mode — all use plain scripting (`new-session`/`new-window`/`send-keys`/
`capture-pane`/`has-session`). Only iTerm2 uses `-CC`, and it pays heavily: a
stateful stream parser (`TmuxGateway`), protocol version coupling (no negotiation
handshake — **no source found** because there is none), in-band signaling
fragility ([#8953](https://gitlab.com/gnachman/iterm2/-/issues/8953)), and
nondeterministic startup ([#11174](https://gitlab.com/gnachman/iterm2/-/issues/11174)).
What `-CC` *buys* is native rendering + a pushed event stream
([tmux.1 CONTROL MODE](https://raw.githubusercontent.com/tmux/tmux/master/tmux.1)).
*Recommendation:* **plain scripting.** grove's dashboard is itself a tmux window
(D3), so grove does not need the one thing control mode is for — rendering tmux
windows as native UI; tmux renders them. The pushed-event benefit (learning a
window closed) is obtainable far more cheaply (Q5). Control mode would import
iTerm2's entire complexity budget for a benefit grove's architecture makes moot.

**Q3. Launch / attach sequence and the `$TMUX` pitfall.**
*Evidence:* the orchestrators **ignore `$TMUX` entirely** (cs and uzi — no
handling, no warning: a real gap, not a solved problem). The declarative managers
**all implement the same correct guard**: `$TMUX` empty → `attach-session`;
`$TMUX` set → `switch-client` (sesh `switch_or_attach.go`, tmuxinator
`template.erb`, smug `smug.go`), dodging tmux's `sessions should be nested with
care` error ([tmux/tmux#3124](https://github.com/tmux/tmux/issues/3124)).
*Recommendation, with a grove-specific subtlety:* the managers' `switch-client`
trick works because they live on the **same server** as the caller. grove's
server is a **different** server (its own socket), so `switch-client` cannot move
a client from the user's tmux into grove's. That leaves 030 a genuine decision
with three evidence-backed options:
  1. **Refuse-and-instruct** when `$TMUX` is set — tell the user to run `grove
     tui` from outside tmux. Safest; matches "don't nest." (No surveyed tool does
     this, but none owns a separate server either.)
  2. **Nest deliberately** — unset `$TMUX` and `tmux -L grove attach`, accepting
     a tmux-inside-tmux. Works, but the prefix-key collision and visual nesting
     are exactly what iTerm2's wiki and tmux#3124 warn against.
  3. **Detect and adapt** — outside tmux, attach normally; inside the user's
     tmux, open grove's dashboard as a *window in the user's own session* instead
     of attaching to grove's server (a hybrid). More code; defers the isolation
     benefit when nested.
  030 should pick among these explicitly; the survey's contribution is that
  option (1) or (2) is required because (the managers') `switch-client` path is
  unavailable across sockets.

**Q4. Owned config / keybindings / status line.**
*Evidence:* the orchestrators ship **no** config and inherit the user's,
which *causes* uzi [#11](https://github.com/devflowinc/uzi/issues/11) (base-index)
and contributes to cs [#277](https://github.com/smtg-ai/claude-squad/issues/277)
(env). The managers don't clobber `~/.tmux.conf` but depend on whatever it
loaded.
*Recommendation:* **grove ships a minimal `grove.conf` and starts its server
with `tmux -L grove -f <grove.conf>`.** A private socket only pays off with a
private config: it pins `base-index`/`pane-base-index` (eliminating uzi #11),
sets grove's own status line and keybindings, and never reads or clobbers the
user's `~/.tmux.conf`. This makes Q1 and Q4 a single decision — socket + config
travel together.

### For 040 — harness pane

**Q5. Window lifecycle — create, name, tear down, learn a child exited.**
*Evidence:* every surveyed scripting tool **creates** windows trivially
(`new-window -n <name>`, `rename-window`) and **learns about exit only by
polling** — cs/uzi scrape `capture-pane` and check `has-session`/`list-windows`;
no surveyed tool uses a tmux hook. The declarative managers don't track exit at
all. Only control mode gets *pushed* `%window-close`
([tmux.1 CONTROL MODE](https://raw.githubusercontent.com/tmux/tmux/master/tmux.1)),
which Q2 rules out. uzi #11 also warns against hard-coding window/pane indices.
*Recommendation:* **create** harness windows with `new-window -n <grove-name>` on
grove's server; **never hard-code `:0`** (grove's own config pins base-index, but
target by window name/id regardless). **Detect exit** via one of:
  - *Baseline (proven):* poll `tmux -L grove list-windows` on the existing
    fs-watch / refresh tick — exactly what cs and uzi do, and sufficient.
  - *Enhancement (available because grove owns the server/config, but
    unproven in this survey — no surveyed tool used it):* a server `set-hook`
    such as `pane-died` / `pane-exited` with `remain-on-exit on`, or a
    `window-pane-died` hook, to turn exit into an event instead of a poll. Flag
    as a 040 spike, not a settled mechanism.

**Q6. Crash isolation + persistence — the D2 load-bearing claim.**
*Evidence:* **confirmed.** Because cs and uzi lean on a long-lived tmux daemon,
agent sessions survive the orchestrator dying and re-attach with plain `tmux
attach` (cs `Restore()` + on-disk storage). iTerm2 `-CC`: the server is a
separate daemon, sessions survive iTerm2 quitting and re-attach with plain `tmux
attach` from any terminal
([iterm2 docs](https://iterm2.com/documentation-tmux-integration.html)). The
sharp contrast: suspend-and-run TUIs (lazygit/gh-dash) give **zero** persistence
— the child dies with the terminal — which is the exact gap D2 exists to close.
*Recommendation:* **D2 holds — own the tmux server.** With grove's dashboard
(window 0) killed, the harness windows and their processes survive on the `grove`
server, and the user can re-attach with `tmux -L grove attach`. **One honest
limit to record for 040** (so it doesn't over-promise): a tmux server — default
*or* dedicated — does **not** survive a reboot (cs
[#212](https://github.com/smtg-ai/claude-squad/issues/212)). "Persistence" means
*survives the dashboard dying*, not *survives a reboot*; durable
resume-after-reboot would need grove to persist session metadata and recreate
windows (as cs does on disk), which is out of scope for the harness-pane MVP.

### One-line decisions this survey backs

- **Socket + config travel together:** `tmux -L grove -f grove.conf`, pinning
  `base-index`/`pane-base-index`, never touching `~/.tmux.conf`. (Q1, Q4)
- **Plain scripting, not `-CC`:** grove's dashboard is a tmux window, so it needs
  none of what control mode is for. (Q2)
- **`$TMUX`-when-launching is a real decision, not a no-op** — and the managers'
  `switch-client` answer doesn't transfer across sockets; 030 must choose
  refuse / nest / hybrid. (Q3)
- **Create by name, detect exit by polling `list-windows`** (proven), with server
  hooks as an optional enhancement spike. (Q5)
- **D2 confirmed:** owning the server gives dashboard-crash survival + manual
  re-attach; it does *not* give reboot survival. (Q6)

## Method & limits

Four independent searches (one per cluster), each required to cite a primary
source (repo file, numbered issue, or official docs) per failure-mode claim and
to record silences explicitly as "no primary source found." Three load-bearing
citations were re-verified directly by the author after the searches: cs #277
(env / dedicated-socket fix), uzi #11 (base-index + own-config suggestion), and
cs `tmux.go` (default server, no `-CC`, no `$TMUX`). Known gaps are recorded
inline above; the largest are the absence of any `$TMUX`/nesting handling in the
agent orchestrators (a genuine gap in the prior art, not a solved problem) and
the absence of a tmux control-mode version-negotiation handshake (there is none).

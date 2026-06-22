# 17. Interactive `$EDITOR` runs on the dashboard proxy's tty via a framed down control verb

- Status: **superseded** (was accepted) — Superseded by
  [ADR-0031](0031-shed-machinery-keep-self-extension-core-and-methodology.md) (grove
  sheds its machinery to a self-extension core) and
  [ADR-0032](0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md) (the loop
  substrate is a self-driving shell loop). The rmux/ratatui TUI + Fleet tower this ADR
  belongs to is **deleted** in leaf `080-shed-tui`; its runtime lives only in git
  history. The decision is retained here as record. (Prior status: superseded by
  [ADR-0028](0028-rmux-substrate.md), rmux substrate, 2026-06-10, 070-teardown D4 —
  there is no proxy tty and no `RunEditor` control frame: open-in-editor is a direct
  suspend-loop + `$EDITOR` over `rmux capture-pane`, ADR-0029. Mechanism gone.)
- Date: 2026-05-31
- Deciders: Antony Blakey (with grove 060 build, leaf 040/020-controller-loop)
- Refines: ADR-0016 (dashboard surfaces are dumb-terminal proxies)

## Context

ADR-0016 made the dashboard a **dumb-terminal proxy** to one controlling
process: the controller renders and ships output bytes *down* a unix-domain
socket; the proxy blits them to its stdout and forwards stdin *up*. Leaf 010
built that seam with the down direction *unframed* (raw ANSI, "one message kind,
no framing") — but tagged the whole wire shape **"proposed; refine in build,"**
and ADR-0016 itself called the precise wire format "a 040 build detail."

Leaf 020 moves the v1 `grove tui` dashboard into the controller. Most of its
shell-out writes (`grove-llm inbox-add` / `inbox-drain` / `inbox-edit`) are
**non-interactive** subprocesses — in the controller they simply run and return
a status; they need no terminal. The v1 `suspended()` dance (suspend the
alt-screen, run, re-init) existed only because the dashboard process *itself*
owned the tty; the controller owns none, so those writes just run directly.

The exception is the interactive **`$EDITOR` drop** (`Ctrl-E` to edit a capture
body, or to edit a committed inbox observation). An editor needs a real tty —
termios raw mode, the alternate screen, SIGWINCH sizing — which **only the proxy
has**. The controller has no usable terminal of its own, and a tty cannot travel
over the socket. So "where does the editor run?" is the one place the dumb-proxy
model meets an interactive child process.

## Decision

**The interactive `$EDITOR` runs on the proxy's own tty, driven by a control
verb the controller sends down the seam.** To carry it, the down direction gains
the same tiny tag+length framing the up direction already had:

- **`DownFrame::Output(bytes)`** — the hot path: one frame per rendered draw,
  carrying ratatui's ANSI; the proxy blits the payload to stdout. A
  `FrameWriter` wraps the controller's socket and emits exactly one `Output`
  frame per draw flush, so the proxy stays a pure blit.
- **`DownFrame::RunEditor { path }`** — the control verb: run the user's
  `$EDITOR` against `path` on the proxy's tty.
- **`UpFrame::EditorDone { ok }`** — the reply, carrying the child's success so
  the controller can mirror v1's editor-exit-status check.

The flow: the controller creates a tempfile, seeds it with the text to edit,
sends `RunEditor { path }`, and **blocks rendering** until `EditorDone`. The
proxy, on `RunEditor`, **suspends** its tty ownership (leaves the alternate
screen, drops raw mode), spawns `$EDITOR <path>` with the tty inherited, waits,
**resumes** its tty ownership, and replies `EditorDone`. The controller then
reads the tempfile back and forces a full redraw (the proxy's tty was repainted).

Two consequences are deliberate and bounded:

- **The proxy resolves `$EDITOR`, not the controller.** `$EDITOR`/`$VISUAL` are
  the *user's terminal-session* environment, which is the proxy's, not the
  long-lived controller's. The controller only names the file to edit.
- **The tempfile is shared-filesystem** between controller and proxy. True for a
  local proxy (the v1 case: same machine, unix socket). A future *remote* proxy
  (e.g. a web client, ADR-0016) shares no filesystem and has no local `$EDITOR`,
  so it will get a different edit affordance (inline / upload), not `RunEditor`.
  This is acceptable: interactive `$EDITOR`-over-proxy is a local-proxy feature.

## Why not the alternatives

- **A separate control socket** (leave the tested raw-ANSI down path untouched,
  add a second framed socket for control) was rejected: it doubles socket setup
  and the 030 launch wiring for one rare verb, where 010 had already flagged the
  down format as build-time-refinable.
- **Running the editor controller-side** is impossible — the controller has no
  tty, and the user is looking at the proxy pane, not the controller's stdout.
- **Deferring `$EDITOR`-over-proxy** would leave leaf 020's acceptance unmet
  (`$EDITOR` must work).

## Consequences

- **Down direction is now framed** (`Output` / `RunEditor`), symmetric with the
  up direction. The proxy gains a `DownDecoder`; it is still pure transport — no
  grove state, logic, or ratatui. "Run a command on your tty" is legitimate
  terminal transport, symmetric with "blit these bytes."
- **The proxy is a single-threaded `poll(2)` loop** over (socket, stdin) instead
  of 010's detached stdin thread. While the editor child runs, the loop is
  blocked in `wait()` and is *not* polling stdin, so the child keeps the tty's
  keystrokes — no thread races a blocked `read` for the user's input. The
  SIGWINCH handler stays a detached thread (it only *writes* `Resize` up).
- **Non-interactive writes need no tty.** `inbox-add`/`drain`/`inbox-edit` run
  directly in the controller; the v1 suspend/restore is gone for them. The
  fs-watch picks up their effect and rescans, exactly as before.
- **A web front-end stays cheap** (ADR-0016): it speaks the same framed
  protocol; only its edit affordance differs from `RunEditor`.

## Notes

- Wire tags: down `O`=Output, `E`=RunEditor; up adds `D`=EditorDone (after
  010's `S`=Resize, `I`=Input). All are `tag + u32 len + payload` except the
  fixed-size `Resize`/`EditorDone`.
- The harness panes are unaffected — they remain native zellij panes running
  `grove do <name>` (zellij emulates them). This ADR is about the *dashboard*
  proxy's editor drop, not the harnesses.

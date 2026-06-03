# trellis host API — the library-you-link hosting seam

This is the contract by which a **host app** (grove, today; any TUI app, later)
embeds the trellis framework, renders its own surface as a real pane, and drives
layout — **in-process, no socket, no WASM**. It is the first cut of what an
extracted `trellis` will publish, and the contract grove's later UX leaves
(120 native-nav, 130 native-detail, 140 whichkey, 150 working-set) build on.

It records the seam as realised by leaf
`060-harness-pane/040-grove-integration/110-native-host-api/030-port-dashboard-drive`
(grove's v1 dashboard ported to render natively). It is deliberately **minimal**
— exactly what the v1 dashboard exercises — and widens only as a real consumer
demands (grove constraint 4). Decisions: **ADR-0020** (deep fork), **ADR-0021**
(library-you-link, the three seam points).

## The shape: grove owns `main`

trellis is **library-structured** — the work lives in `zellij_client::start_client`
and `zellij_server::start_server`, each taking an injectable
`Box<dyn ClientOsApi>` / `Box<dyn ServerOsApi>`. The host links the three crates
(`zellij-client`, `zellij-server`, `zellij-utils`) and **dispatches the roles
itself**, replacing trellis's thin `main.rs`:

- **no `--server` arg** → run the **client** UI path (`start_client`);
- **`--server <socket>` arg** → run the **trellis server** (`start_server`).

The server is a **re-exec'd separate process**: the client's `spawn_server` runs
`current_exe() --server <socket>`, which (because `current_exe()` is the host
binary) the host must recognise *before its own arg parser* and route to the
server path. **The host's surface renders inside that server daemon**, not the
thin foreground client (the client just blits the composited ANSI). Anything the
surface needs must therefore be reachable on the `--server` path — see *Passing
host state across the re-exec* below.

grove's realisation: `src/trellis_host.rs` (`run_client` / `run_server` /
`parse_server_invocation`) and the pre-clap `--server` intercept in `src/cli.rs`.

## Seam point 1 — role dispatch

The host owns `main` and branches on `--server`. Nothing trellis-specific to
implement beyond mirroring `start_client`'s setup (config / layout via
`Setup::from_cli_args`, `ClientInfo::New`) and `start_server`'s
(`get_server_os_input` → `start_server(os_input, socket)`).

## Seam point 2 — the OS/terminal injection (`ClientOsApi` / `ServerOsApi`)

Both entry points take a boxed trait object the host may substitute or wrap to
inject grove-specific OS/terminal behaviour. For 030 the stock impls
(`get_client_os_input` / `get_server_os_input`) suffice; the seam is here for when
a later leaf needs to intercept terminal I/O.

## Seam point 3 — the host pane (`HostSurface` + `HostDriver` + the tick)

The headline. grove's native surfaces become a **third pane kind**, `PaneId::Host`,
beside `Terminal` (pty) and `Plugin` (wasm). The pane (`HostPane`) is a `Pane`
impl that renders a host-supplied **surface** into an off-screen ratatui `Buffer`,
converts it to `CharacterChunk`s server-side, and delivers input/resize/focus to
the surface in-process. Defined in
`crates/trellis/zellij-server/src/panes/host_pane.rs`.

### `HostSurface` — what the host implements

```rust
pub trait HostSurface: Send {
    fn draw(&mut self, area: Rect, buf: &mut Buffer);     // ratatui draw; area origin (0,0)
    fn handle_key(&mut self, key: KeyEvent) -> bool;      // crossterm key; true = re-render
    fn resize(&mut self, cols: u16, rows: u16) {}         // content area changed
    fn set_focused(&mut self, focused: bool) {}           // gained/lost focus
    fn cursor(&self) -> Option<(u16, u16)> { None }        // content-relative cursor, or hidden
    fn set_driver(&mut self, driver: HostDriver) {}        // lifecycle: receive the drive handle
    fn tick(&mut self) -> bool { false }                   // out-of-band refresh; true = re-render
}
```

The host draws with **real ratatui widgets** (into the `Buffer`) and consumes
**crossterm key events** — so an existing crossterm/ratatui TUI ports across the
seam essentially unchanged. grove proves this by reusing its v1 `render(f, app)`
and `handle_key(app, …)` verbatim, rendering through an off-screen
`Terminal<TestBackend>` and blitting the cells (`src/tui.rs`, the `native`
submodule).

### `HostDriver` — what the host drives layout through

`set_driver` hands the surface a clonable, `Send` handle. Every method is
**fire-and-forget** — it posts a `ScreenInstruction` (or `ServerInstruction`) onto
the server's channel and returns, because the surface runs *on the screen thread*
(via input routing) and so cannot block waiting for that same thread to reply.
Layout driving is therefore by **message, not return value** — there is no
synchronous "create tab and return its id".

```rust
impl HostDriver {
    // open a NEW tab named `name` whose initial pane runs `command args…` in `cwd`
    fn new_command_tab(&self, name: &str, cwd: PathBuf, command: PathBuf, args: Vec<String>);
    fn focus_tab(&self, name: &str);   // GoToTabName(create=false) — never spawns an empty tab
    fn close_tab(&self, name: &str);   // focus the named tab, then close the now-active one
    fn request_tick(&self);            // post HostSurfaceTick(pane_id) — see below
    fn quit(&self);                    // ServerInstruction::ClientExit — tear the session down
}
```

Tabs are addressed **by name**, not numeric id — the async, no-reply model has no
clean way to round-trip a server-assigned id back to the surface, and names are
what grove already keys harnesses on. The host tracks which names are open (grove:
a `BTreeSet<String>`); trellis does not dedupe names. This subsumes the retired
`zellij action new-tab / go-to-tab-by-id / close-tab-by-id` shell-outs.

### The tick — waking a redraw with no input

A host pane re-renders when input or resize touches it. But a host often has
**out-of-band** state that changes with no keypress — grove watches the filesystem
(`notify`) for grove/inbox changes. The surface itself lives on the screen thread
and must never be touched from another thread. The bridge:

1. the host's background thread (grove's fs-watch debounce thread) holds a
   **clone of the `HostDriver`** and calls `request_tick()` when state settles;
2. that posts `ScreenInstruction::HostSurfaceTick(pane_id)` onto the screen
   thread's channel;
3. the screen thread finds the host pane by id, calls the surface's `tick()`
   (which refreshes and returns whether to redraw), and re-renders if so.

So the surface is mutated **only** on the screen thread — in `draw`, the input
path, or `tick` — never concurrently.

### Lifecycle / injection

Because the server is re-exec'd, the host cannot pass a surface *value* across the
spawn. Instead:

1. on the `--server` path, **before** `start_server`, the host calls
   `register_host_surface(factory)` with a one-shot `FnOnce() -> Box<dyn HostSurface>`;
2. the server takes it **once**, when the first tab's layout is applied, builds a
   single `HostPane`, and — crucially — hands the surface a `HostDriver` wired to
   that tab's screen/server senders via `set_driver` (those senders only exist on
   the screen thread, which is why the driver arrives here and not in the factory).

A general multi-host registry is intentionally not built (one host pane per
session is all grove needs today).

## Passing host state across the re-exec

The `--server` process is a fresh `current_exe()` exec; it does **not** inherit the
client's in-memory state, but it **does** inherit the environment (`spawn_server`
does not clear it). grove uses this to tell the server which repo to scan:
`run_client` sets `GROVE_REPO`; `run_server` reads it (falling back to the cwd's
git root). Env is the simplest reliable channel for the small, stable inputs a host
needs server-side.

## The one-way crate seam (ADR-0020 §4)

`zellij-server` defines `HostSurface` / `HostDriver`; the host implements them.
**trellis never names the host.** grove depends on the `zellij-*` crates; they
never depend on grove. The whole grove→trellis edge sits behind the `trellis-seam`
Cargo feature, so a default `grove` / `grove-llm` build never links the
~100k-LOC server.

## What is deliberately *not* here yet (constraint 4)

- **Synchronous results / a query API** — driving is fire-and-forget; no GraphQL,
  no network surface (ADR-0020 §5, explicitly out of scope).
- **Embedded-tool observability** (a wrapped tool's exit status, screen, cursor,
  scrollback, input injection — ADR-0020 §6). 130-native-detail's in-process
  `$EDITOR` drop needs it; until then grove surfaces a pointer rather than
  half-doing it.
- **Out-of-band tab reconciliation** — if the user closes a harness tab trellis
  opened, the host's name set goes stale (a `focus_tab` then no-ops). Robust
  reconciliation needs the observability above.
- **Non-blocking host writes** — grove's `grove-llm` capture/drain writes run
  synchronously on the screen thread (sub-second git commits). A background-thread
  + tick variant is a candidate follow-up if the brief freeze ever bites.
- **Multiple host panes / a host registry** — one per session today.

Each lands when a real consumer (120–150, or the eventual extraction) demands it.

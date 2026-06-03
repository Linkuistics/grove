//! grove as a **trellis host**: grove owns `main` and dispatches the trellis
//! client/server roles itself (ADR-0021, *library-you-link*). This is leaf
//! `110-native-host-api/010-boot-role-dispatch` — the foundation the native pane
//! (020) and dashboard port (030) stack on.
//!
//! # Why grove owns `main`
//!
//! The [[trellis framework]] (grove's hard fork of zellij, vendored at
//! `crates/trellis/`) is **library-structured**: the real work lives in
//! `zellij_client::start_client` and `zellij_server::start_server`, each taking an
//! injectable `Box<dyn ClientOsApi>` / `Box<dyn ServerOsApi>`. zellij's own
//! `main.rs` is a thin dispatcher over those. ADR-0021 (forced by the structure,
//! not chosen) is that grove **replaces that dispatcher with its own**:
//!
//! - no `--server` arg → grove runs the **client** UI path ([`run_client`]);
//! - `--server <socket>` arg → grove runs the **trellis server** ([`run_server`]).
//!
//! # The re-exec is load-bearing
//!
//! `zellij_client`'s `spawn_server` starts the server by running
//! `Command::new(current_exe()).arg("--server").arg(<socket>)` (plus `--debug`).
//! Because `current_exe()` is the **grove** binary, grove must recognise that
//! re-exec and route it to the server start path — which is what
//! [`server_invocation`] detects (before grove's clap parser, which knows nothing
//! of `--server`). grove's render/pane code therefore runs **inside the server
//! daemon** (ADR-0021 Evidence #3); 020/030 build grove's native surfaces there.
//!
//! # The seam stays one-way (ADR-0020 §4)
//!
//! grove depends on the `zellij-{client,server,utils}` crates (aliased
//! `trellis*`); they never depend on grove. Everything here is gated behind the
//! `trellis-seam` feature so a default `grove`/`grove-llm` build never links the
//! ~100k-LOC server.
//!
//! # Scope of this leaf (010)
//!
//! Bring up a *working* trellis session from the grove binary — role-dispatch +
//! the re-exec proven by a **stock** session (zellij's default config/layout =
//! a terminal pane the daemon renders and the client blits). grove's **native**
//! surfaces (the dashboard pane, tab/pane/focus driving) are 020/030; grove's
//! *tamed* config (bars-free, locked mode — already drafted in
//! [`crate::zellij`]'s `CONFIG_TEMPLATE`) is a fast-follow, deferred so this
//! foundation step maximises the chance the session simply boots.

#![cfg(feature = "trellis-seam")]

use std::path::PathBuf;

use anyhow::Result;

use trellis::cli::CliArgs;
use trellis::setup::Setup;
use trellis_client::os_input_output::get_client_os_input;
use trellis_client::{start_client, ClientInfo};
use trellis_server::os_input_output::get_server_os_input;
use trellis_server::start_server;

use crate::cli::RepoArgs;

/// The trellis server re-exec, parsed from grove's own argv.
///
/// `zellij_client::spawn_server` re-execs `current_exe() --server <socket>
/// [--debug]`; since `current_exe()` is grove, grove sees exactly those args.
#[derive(Debug, PartialEq, Eq)]
pub struct ServerInvocation {
    pub socket: PathBuf,
    pub debug: bool,
}

/// Detect a trellis server re-exec in an argument list (everything after argv\[0\]).
///
/// Returns `Some` iff a `--server <socket>` pair is present. Pure over its input
/// so it is unit-testable without touching the real process args; [`server_invocation`]
/// is the thin wrapper that feeds it `std::env::args_os()`.
///
/// This must run **before** grove's clap parser: grove's `Cli` has no `--server`
/// flag and would reject the re-exec'd argv outright. No grove verb uses
/// `--server`, so a normal invocation never false-positives here.
pub fn parse_server_invocation<I, S>(args: I) -> Option<ServerInvocation>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut socket: Option<PathBuf> = None;
    let mut debug = false;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        let arg = arg.as_ref();
        if arg == "--server" {
            socket = iter.next().map(|s| PathBuf::from(s.as_ref()));
        } else if arg == "--debug" {
            debug = true;
        }
    }
    socket.map(|socket| ServerInvocation { socket, debug })
}

/// [`parse_server_invocation`] over the real process arguments (skipping argv\[0\]).
pub fn server_invocation() -> Option<ServerInvocation> {
    parse_server_invocation(std::env::args_os().skip(1))
}

/// Run the **trellis server** for a re-exec invocation (ADR-0021): the role grove
/// plays when `spawn_server` re-execs it. Mirrors zellij `commands::start_server`
/// — set the instance-wide debug flag, open the server OS seam, hand off to
/// `zellij_server::start_server`, which daemonizes and runs the render/IPC loop
/// until the session ends.
pub fn run_server(inv: ServerInvocation) -> Result<()> {
    boot_framework();
    // zellij sets this once per process before starting the server; a second set
    // (it is a `OnceCell`) would be a bug, so ignore the already-set Err.
    let _ = trellis::consts::DEBUG_MODE.set(inv.debug);

    // Register grove's host surface (ADR-0021 native-pane seam; leaf 020). The
    // trellis server takes this factory once, when the first tab's layout is
    // applied, and injects it as a `HostPane` — grove's ratatui drawn in-process
    // and composited as a real pane, no proxy socket, no WASM. For 020 the
    // surface is a trivial render+input prover (a keypress counter); leaf 030
    // swaps in the real v1 dashboard surface. This must happen on the `--server`
    // path because the host pane renders inside the server daemon (Evidence #3).
    trellis_server::panes::host_pane::register_host_surface(Box::new(|| {
        Box::new(host_surface::CounterSurface::default())
    }));

    let os_input =
        get_server_os_input().map_err(|e| anyhow::anyhow!("opening the trellis server: {e}"))?;
    // Returns `()`; it owns the process until the session exits.
    start_server(Box::new(os_input), inv.socket);
    Ok(())
}

/// Run the **trellis client** UI for `args.repo` (default: the cwd's git root):
/// grove's foreground role when launched as the TUI. Mirrors the essential setup
/// of zellij `commands::start_client` — resolve config/options/layout via
/// [`Setup::from_cli_args`], open the client OS seam, and start a **new** named
/// session via `zellij_client::start_client`. The client connects to the server
/// (spawning it by re-exec — see [`run_server`]) and blits its rendered frames.
///
/// 010 uses a stock session (default config/layout → a terminal pane), proving
/// the in-binary client/server split and the re-exec. The reconnect handshake
/// (`start_client` can return a `ConnectToSession`) is intentionally not looped
/// here — a single session is the foundation proof; 030's dashboard port revisits
/// it if detach/reattach is needed.
pub fn run_client(args: &RepoArgs) -> Result<()> {
    let repo = crate::repo::resolve(args.repo.as_deref())?;
    boot_framework();

    let session = crate::zellij::session_name(&repo);
    let opts = client_cli_args(&session);

    let (config, layout_info, config_options, _config_no_layout, _options_no_layout) =
        Setup::from_cli_args(&opts)
            .map_err(|e| anyhow::anyhow!("resolving the trellis config/layout: {e}"))?;

    let os_input =
        get_client_os_input().map_err(|e| anyhow::anyhow!("opening the terminal: {e}"))?;

    let info = ClientInfo::New(session, layout_info, Some(repo.clone()));

    // Foreground client loop: owns the tty until the session detaches/exits.
    let _reconnect = start_client(
        Box::new(os_input),
        opts,
        config,
        config_options,
        info,
        None,  // tab_position_to_focus
        None,  // pane_id_to_focus
        false, // is_a_reconnect
        false, // start_detached_and_exit
    );
    Ok(())
}

/// The `CliArgs` grove hands the trellis client. Built via `Default` (CliArgs
/// derives it) rather than zellij's clap parser — trellis pins **clap 3** while
/// grove uses **clap 4**, so the two `Parser` impls are different crates and
/// grove must never route argv through zellij's. Only the session identity is set;
/// everything else (config/layout/data dirs) takes trellis's defaults for 010.
fn client_cli_args(session: &str) -> CliArgs {
    CliArgs {
        session: Some(session.to_string()),
        ..CliArgs::default()
    }
}

/// The preamble zellij's `main.rs` runs before any role dispatch: install the
/// file logger and ensure the config/cache folders (and bundled plugin assets the
/// default layout's bars load) exist. Idempotent; called on both the client and
/// server paths, and only on the trellis paths (a plain `grove status` must not
/// spin up zellij's logger).
fn boot_framework() {
    trellis::logging::configure_logger();
    trellis::consts::create_config_and_cache_folders();
}

/// grove's host surface for leaf 020 — the trivial render+input prover that runs
/// through the trellis native-pane seam ([`trellis_server::panes::host_pane`]).
/// It draws a keypress counter with ratatui and increments on every key, proving
/// a grove-drawn ratatui surface renders as a real trellis pane and reacts to
/// input in-process. Leaf 030 replaces it with the ported v1 dashboard.
mod host_surface {
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::event::KeyEvent;
    use ratatui::layout::Rect;
    use ratatui::style::Stylize;
    use ratatui::text::{Line, Text};
    use ratatui::widgets::{Paragraph, Widget, Wrap};

    use trellis_server::panes::host_pane::HostSurface;

    /// A self-drawing keypress counter. State lives here, in the server daemon.
    #[derive(Default)]
    pub struct CounterSurface {
        keypresses: u64,
        last_key: Option<String>,
        focused: bool,
        cols: u16,
        rows: u16,
    }

    impl HostSurface for CounterSurface {
        fn draw(&mut self, area: Rect, buf: &mut Buffer) {
            let lines = vec![
                Line::from("grove · native trellis pane".bold()),
                Line::from(""),
                Line::from(format!("keypresses: {}", self.keypresses).cyan()),
                Line::from(format!(
                    "last key:   {}",
                    self.last_key.as_deref().unwrap_or("—")
                )),
                Line::from(format!("focused:    {}", self.focused)),
                Line::from(format!("content:    {}×{}", self.cols, self.rows)),
                Line::from(""),
                Line::from("press any key to count".dim()),
            ];
            Paragraph::new(Text::from(lines))
                .wrap(Wrap { trim: false })
                .render(area, buf);
        }

        fn handle_key(&mut self, key: KeyEvent) -> bool {
            self.keypresses += 1;
            self.last_key = Some(format!("{:?}", key.code));
            true
        }

        fn resize(&mut self, cols: u16, rows: u16) {
            self.cols = cols;
            self.rows = rows;
        }

        fn set_focused(&mut self, focused: bool) {
            self.focused = focused;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_server_reexec_with_socket() {
        let inv = parse_server_invocation(["--server", "/tmp/trellis.sock"]);
        assert_eq!(
            inv,
            Some(ServerInvocation {
                socket: PathBuf::from("/tmp/trellis.sock"),
                debug: false,
            })
        );
    }

    #[test]
    fn detects_server_reexec_with_debug() {
        // zellij's spawn_server appends `--debug` after the socket when debug is on.
        let inv = parse_server_invocation(["--server", "/tmp/t.sock", "--debug"]);
        assert_eq!(
            inv,
            Some(ServerInvocation {
                socket: PathBuf::from("/tmp/t.sock"),
                debug: true,
            })
        );
    }

    #[test]
    fn normal_invocations_are_not_server_reexecs() {
        // The client path and grove's own verbs must never be mistaken for a
        // server re-exec — only an explicit `--server <socket>` pair triggers it.
        assert_eq!(parse_server_invocation(["tui"]), None);
        assert_eq!(parse_server_invocation(["status", "--repo", "/x"]), None);
        assert_eq!(parse_server_invocation(Vec::<String>::new()), None);
        // A lone `--debug` (no socket) is not a server invocation.
        assert_eq!(parse_server_invocation(["--debug"]), None);
    }

    #[test]
    fn client_cli_args_set_only_the_session() {
        let opts = client_cli_args("grove-acme");
        assert_eq!(opts.session.as_deref(), Some("grove-acme"));
        // 010 leaves the rest at trellis defaults (no forced config/layout).
        assert!(opts.server.is_none());
        assert!(opts.layout.is_none());
        assert!(opts.config.is_none());
    }
}

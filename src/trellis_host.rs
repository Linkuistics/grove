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
use trellis::input::config::Config;
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

    // Resolve the repo the dashboard surfaces. The client passes it via the
    // `GROVE_REPO` env var, which the re-exec inherits (`spawn_server` runs
    // `current_exe() --server …` without clearing the environment); fall back to
    // the cwd's git root for a bare `grove --server` invocation.
    let repo = match std::env::var_os("GROVE_REPO") {
        Some(p) => PathBuf::from(p),
        None => crate::repo::resolve(None)?,
    };

    // Register grove's host surface (ADR-0021 native-pane seam). The trellis
    // server takes this factory once, when the first tab's layout is applied, and
    // injects it as a `HostPane` — grove's v1 dashboard ratatui drawn in-process
    // and composited as a real pane, no proxy socket, no WASM. The surface is
    // built **now** (so a scan failure surfaces before the session boots) and
    // moved into the one-shot factory. This must happen on the `--server` path
    // because the host pane renders inside the server daemon (Evidence #3).
    let surface = crate::tui::dashboard_surface(repo)?;
    trellis_server::panes::host_pane::register_host_surface(Box::new(move || Box::new(surface)));

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

    // The re-exec'd server inherits our environment; pass it the repo so its
    // dashboard surface (built on the `--server` path) scans the right tree.
    std::env::set_var("GROVE_REPO", &repo);

    let session = crate::zellij::session_name(&repo);
    let opts = client_cli_args(&session);

    let (config, layout_info, _config_options, _config_no_layout, _options_no_layout) =
        Setup::from_cli_args(&opts)
            .map_err(|e| anyhow::anyhow!("resolving the trellis config/layout: {e}"))?;

    // Merge grove's tamed config (locked mode + the `Ctrl-o` leader that focuses
    // the constant nav) onto the trellis base. `config_options` is taken from the
    // merged config so the session boots in `locked` mode — the modal design that
    // lets every other key pass through to the focused harness / nav surface.
    let config = grove_tui_config(config)?;
    let config_options = config.options.clone();

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
/// grove must never route argv through zellij's. Sets the session identity and the
/// stringified two-pane [`GROVE_TUI_LAYOUT`] (constant nav + content slot,
/// ADR-0022/0023); everything else (config/data dirs) takes trellis's defaults.
fn client_cli_args(session: &str) -> CliArgs {
    CliArgs {
        session: Some(session.to_string()),
        layout_string: Some(GROVE_TUI_LAYOUT.to_string()),
        ..CliArgs::default()
    }
}

/// grove's tamed trellis config (leaf `120-native-nav`), built **in-process** and
/// merged onto the trellis base in [`grove_tui_config`] — never written to disk
/// (the bundled-config-to-cache-dir machinery retired with ADR-0020). The
/// decisions here survive from the superseded `zellij action` era (the retired
/// `030-zellij-launch` `CONFIG_TEMPLATE`); only the *realisation* of the leader
/// changed — `LaunchOrFocusPlugin` → a native `GoToTab` (ADR-0020/0018).
///
/// - `default_mode "locked"`: trellis passes every *unbound* key straight through
///   to the focused pane — the harness (claude/codex chords, vim Esc) or grove's
///   own nav surface (`j`/`k`/`⏎`/`c`). Only the binds below are intercepted.
/// - The **leader, `Ctrl-o`**, **cycles focus** across the session's panes with
///   `FocusNextPane`: nav → harness → detail → nav. 010 bound it to `MoveFocus
///   "Left"` (good for two panes — the nav was the only other pane), but 130/020's
///   content region is a *pair* (harness + detail), so a single-direction move can
///   no longer reach every surface; a cycle does, with one repeated key, and on a
///   two-pane session it degrades to the old nav↔harness toggle. A zellij keybind
///   *must* carry the leader: in `locked` only zellij sees keys while a harness is
///   focused. This supersedes the ADR-0018 era's `GoToTab 1` (there is no longer a
///   home *tab*; ADR-0022's constant nav + swapped content has one tab, so
///   `GoToTab`/`Alt-1..9` tab switching is retired, ADR-0023).
/// - `pane_frames false` / `simplified_ui true` / startup floats off: the
///   composite reads as grove's own UI, not "a zellij session".
/// - `session_serialization false`: the dashboard is rebuilt from the live
///   `RepoView` each launch and the host pane is not serialisable; persistence is
///   a convenience the artifacts-over-state model does not need (root brief).
const GROVE_TUI_CONFIG: &str = r##"// grove's in-process trellis config (leaf 130-native-detail/020). Not hand-edited.
keybinds clear-defaults=true {
    locked {
        // The leader: cycle focus across the session's panes
        // (nav → harness → detail → nav), so every surface is reachable with one
        // repeated key now the content region is a harness+detail pair (130/020).
        bind "Ctrl o" { FocusNextPane; }
    }
}
default_mode "locked"
pane_frames false
simplified_ui true
session_serialization false
show_release_notes false
show_startup_tips false
"##;

/// grove's session layout (ADR-0022/0023, pair-aware per 130/020): one tab of a
/// **fixed-width constant nav** beside a **content region** that is itself a split
/// of two slots — the **harness** (primary) and the per-grove **detail**
/// (secondary), arranged side by side. The host nav surface adopts the leftmost
/// (nav) pane at first-layout (`Tab::inject_host_pane`); the two content-slot
/// placeholders are the addressable targets the content-swap verb
/// (`HostDriver::swap_content`) mounts each selected grove's harness + detail into
/// (closed on the first selection, then swapped as a parked-alive pair). Passed as
/// a stringified layout (`CliArgs::layout_string` → `LayoutInfo::Stringified`), so
/// nothing is written to disk.
const GROVE_TUI_LAYOUT: &str = r##"layout {
    pane split_direction="vertical" {
        pane size=34 name="grove-nav"
        pane split_direction="vertical" {
            pane name="grove-harness"
            pane name="grove-detail"
        }
    }
}
"##;

/// Merge [`GROVE_TUI_CONFIG`] onto `base` (the trellis default resolved by
/// [`Setup::from_cli_args`]). `clear-defaults=true` in the KDL discards trellis's
/// stock keybinds so grove's locked-mode map is the whole keymap; the options
/// (locked default mode, no chrome) merge over the base.
fn grove_tui_config(base: Config) -> Result<Config> {
    Config::from_kdl(GROVE_TUI_CONFIG, Some(base))
        .map_err(|e| anyhow::anyhow!("building grove's trellis config: {e}"))
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
    fn grove_config_parses_and_locks_the_session() {
        use trellis::data::{BareKey, InputMode, KeyWithModifier};

        // A bad action name or KDL typo in GROVE_TUI_CONFIG fails here, before any
        // session ever boots.
        let config = grove_tui_config(Config::default()).expect("grove config builds");

        // The session must start in locked mode (keys pass through to the harness;
        // only grove's binds are intercepted).
        assert_eq!(config.options.default_mode, Some(InputMode::Locked));

        // The leader is bound *in locked mode* — the only mode that sees keys while
        // a harness is focused. An unbound leader would make the nav unreachable.
        let locked = config
            .keybinds
            .0
            .get(&InputMode::Locked)
            .expect("a locked keybind block");
        let leader = KeyWithModifier::new(BareKey::Char('o')).with_ctrl_modifier();
        assert!(
            locked.contains_key(&leader),
            "Ctrl-o (the leader) must be bound in locked mode"
        );
    }

    #[test]
    fn grove_layout_parses_into_a_fixed_nav_beside_a_content_pair() {
        use trellis::input::layout::{Layout, SplitDirection, TiledPaneLayout};

        // A KDL typo or bad attribute in GROVE_TUI_LAYOUT fails here, before any
        // session boots (mirrors `grove_config_parses_and_locks_the_session`).
        let layout =
            Layout::from_kdl(GROVE_TUI_LAYOUT, None, None, None).expect("grove layout parses");
        let (root, _floating) = layout.template.expect("a template layout");

        // The tree's leaf panes are the nav + the content pair (harness + detail).
        fn leaves<'a>(node: &'a TiledPaneLayout, out: &mut Vec<&'a TiledPaneLayout>) {
            if node.children.is_empty() {
                out.push(node);
            } else {
                for c in &node.children {
                    leaves(c, out);
                }
            }
        }
        let mut found = vec![];
        leaves(&root, &mut found);
        let names: Vec<_> = found.iter().filter_map(|p| p.name.as_deref()).collect();
        assert!(names.contains(&"grove-nav"), "a nav pane: {names:?}");
        assert!(names.contains(&"grove-harness"), "a harness slot: {names:?}");
        assert!(names.contains(&"grove-detail"), "a detail slot: {names:?}");
        assert_eq!(found.len(), 3, "exactly nav + harness + detail: {names:?}");

        // The nav is fixed-width, so it stays a constant sidebar on tab resize.
        let nav = found
            .iter()
            .find(|p| p.name.as_deref() == Some("grove-nav"))
            .unwrap();
        assert!(nav.split_size.is_some(), "nav pane is fixed-size");

        // The nav sits beside the content region (a vertical split = a vertical
        // divider); `inject_host_pane` relies on the nav being leftmost. The first
        // node with two children is the nav | content-region split.
        fn split_dir_of_pair(node: &TiledPaneLayout) -> Option<SplitDirection> {
            if node.children.len() == 2 {
                return Some(node.children_split_direction);
            }
            node.children.iter().find_map(split_dir_of_pair)
        }
        assert_eq!(
            split_dir_of_pair(&root),
            Some(SplitDirection::Vertical),
            "nav and content region are split side by side"
        );
    }

    #[test]
    fn client_cli_args_set_session_and_content_pair_layout() {
        let opts = client_cli_args("grove-acme");
        assert_eq!(opts.session.as_deref(), Some("grove-acme"));
        // The constant-nav + content-pair layout rides in as a stringified layout
        // (no file on disk); a named/path layout and config stay at defaults.
        let layout = opts.layout_string.as_deref().expect("a stringified layout");
        assert!(layout.contains("grove-nav"), "nav pane in the layout");
        assert!(layout.contains("grove-harness"), "harness slot in the layout");
        assert!(layout.contains("grove-detail"), "detail slot in the layout");
        assert!(opts.server.is_none());
        assert!(opts.layout.is_none());
        assert!(opts.config.is_none());
    }
}

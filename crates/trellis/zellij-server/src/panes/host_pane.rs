//! The **host-rendered pane** — trellis's third pane kind (ADR-0021).
//!
//! A [`HostPane`] is a [`Pane`] whose content comes from a host-supplied
//! [`HostSurface`] drawing into an off-screen `ratatui` [`Buffer`], rather than
//! from a pty ([`TerminalPane`](crate::panes::terminal_pane::TerminalPane)) or a
//! wasm plugin ([`PluginPane`](crate::panes::plugin_pane::PluginPane)). The pane
//! converts the buffer's cells into [`CharacterChunk`]s server-side, so the
//! compositor blits it like any other pane, and it delivers input/resize/focus to
//! the surface in-process — no pty, no socket, no `zellij action`.
//!
//! # The seam (kept deliberately small — discovered by a real consumer)
//!
//! [`HostSurface`] is the entire host-facing API for this leaf: draw, key, resize,
//! focus, cursor. It hands the host **`crossterm` key events** (grove's surfaces
//! are crossterm-modelled) and a **`ratatui` `Buffer`** (grove draws with real
//! widgets). The one-way framework→host seam (ADR-0020 §4) holds: trellis defines
//! `HostSurface`; the host implements it; trellis never names the host. The
//! surface is widened only as grove's real dashboard (leaf 030/120–150) demands.
//!
//! # Injection
//!
//! Because zellij's server is a re-exec'd daemon, the host (grove) cannot pass a
//! surface *value* across the spawn. Instead the host registers a one-shot
//! [`register_host_surface`] factory before `start_server`; the server takes it
//! once when the first tab's layout is applied and injects a single `HostPane`.
//! A general multi-host registry is intentionally not built here (constraint 4).

use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use anyhow::Result;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier};

use zellij_utils::channels::SenderWithContext;
use zellij_utils::data::{
    BareKey, InputMode, KeyModifier, KeyWithModifier, PaletteColor, PaneContents, Style,
};
use zellij_utils::errors::prelude::*;
use zellij_utils::input::command::RunCommand;
use zellij_utils::input::layout::{Run, TiledPaneLayout};
use zellij_utils::pane_size::{Offset, PaneGeom};
use zellij_utils::position::Position;

use crate::output::{CharacterChunk, SixelImageChunk};
use crate::panes::terminal_character::{
    AnsiCode, CharacterStyles, NamedColor, RcCharacterStyles, TerminalCharacter, DEFAULT_STYLES,
};
use crate::panes::terminal_pane::PaneId;
use crate::screen::ScreenInstruction;
use crate::tab::{AdjustedInput, Pane};
use crate::ui::pane_boundaries_frame::{FrameParams, PaneFrame};
use crate::{ClientId, ServerInstruction};

/// The host-facing surface a [`HostPane`] renders and drives. Implemented by the
/// host (grove); trellis only ever calls these methods. See the module docs for
/// why the surface is this small.
pub trait HostSurface: Send {
    /// Draw the surface into `buf` (whose `area` is the pane's content rect,
    /// origin `0,0`). Called only when the pane needs to re-render.
    fn draw(&mut self, area: Rect, buf: &mut Buffer);

    /// Deliver a key event (the crossterm model). Return `true` if it changed
    /// state and the pane should re-render.
    fn handle_key(&mut self, _key: KeyEvent) -> bool {
        false
    }

    /// Notify the surface that its content area resized to `cols`×`rows`.
    fn resize(&mut self, _cols: u16, _rows: u16) {}

    /// Notify the surface it gained (`true`) or lost (`false`) focus.
    fn set_focused(&mut self, _focused: bool) {}

    /// The cursor position **relative to the content area** (`col`, `row`), or
    /// `None` to hide it.
    fn cursor(&self) -> Option<(u16, u16)> {
        None
    }

    /// Hand the surface its [`HostDriver`] — the in-process handle through which
    /// it drives layout (open/focus/close tabs) and requests redraws. Called
    /// once, when the `HostPane` is constructed inside the server's screen
    /// thread (which is where the senders the driver wraps become available).
    /// The surface clones the driver into any background threads (e.g. an
    /// fs-watch thread) that need to wake a redraw — the driver is `Send` +
    /// `Clone`. Defaulted no-op: a surface that drives nothing ignores it.
    fn set_driver(&mut self, _driver: HostDriver) {}

    /// Called when a [`HostDriver::request_tick`] lands on the screen thread:
    /// the surface may refresh out-of-band state (re-scan the filesystem,
    /// drain a queue) and returns `true` if it now needs a redraw. This is the
    /// only safe way for a background thread to drive the surface — the surface
    /// itself is touched **only** on the screen thread, here and in the
    /// input/draw methods. Defaulted to "nothing changed".
    fn tick(&mut self) -> bool {
        false
    }

    /// A `$EDITOR` pane this surface opened via [`HostDriver::open_editor`] has
    /// exited and this surface's host pane is restored into its slot. `exit_status`
    /// is the editor child's process exit code (`None` if it was killed by a
    /// signal). The surface reads back the tempfile it seeded — applying the edit,
    /// or treating an abnormal exit as "abort" — and returns `true` to re-render.
    /// The first slice of ADR-0020 §6 embedded-tool observability: observe an
    /// embedded tool's exit, then read its side effect. Called only on the screen
    /// thread (in `Tab::close_pane`). Defaulted no-op: a surface that opens no
    /// editor ignores it.
    fn editor_exited(&mut self, _exit_status: Option<i32>) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// The host driver: a clonable, thread-safe handle the surface drives layout
// through. It wraps the screen-thread sender (and the server sender for quit),
// so a host surface — or any thread it clones the driver into — can post
// layout instructions and redraw requests into the running session without
// touching server state directly. This is the consumer-facing half of the
// ADR-0021 host-API seam: the smallest verb set grove's v1 dashboard exercises
// (open/focus/close a named command-tab, request a redraw, quit), widened only
// as later host surfaces (120–150) demand.
// ---------------------------------------------------------------------------

/// The in-process handle a [`HostSurface`] drives the session through. Cheap to
/// clone (the senders are channel handles); `Send` so a surface can hand a clone
/// to a background thread. Every method is fire-and-forget: it posts a
/// [`ScreenInstruction`]/[`ServerInstruction`] and returns, because the surface
/// runs *on* the screen thread (via input routing) and so cannot block waiting
/// for that same thread to process a reply.
#[derive(Clone)]
pub struct HostDriver {
    to_screen: SenderWithContext<ScreenInstruction>,
    to_server: SenderWithContext<ServerInstruction>,
    /// The host pane's own id — the target of [`request_tick`](Self::request_tick).
    pane_id: u32,
    /// The client the host pane is focused by; layout instructions are issued as
    /// if from this client (zellij's instructions are client-scoped).
    client_id: ClientId,
}

impl HostDriver {
    /// Open a **new tab** named `name` whose initial pane runs `command args…`
    /// in `cwd`, focusing it. This is the native replacement for the v1
    /// `zellij action new-tab --cwd … --name … -- <cmd>` shell-out: the command
    /// rides in as the tab layout's `Run::Command`, so the harness process is
    /// spawned by trellis's own pty thread, in-process, no external CLI.
    ///
    /// The caller (grove) tracks which names are open and chooses between this
    /// and [`focus_tab`](Self::focus_tab); trellis does not dedupe names.
    pub fn new_command_tab(&self, name: &str, cwd: PathBuf, command: PathBuf, args: Vec<String>) {
        let run = RunCommand {
            command,
            args,
            cwd: Some(cwd.clone()),
            ..RunCommand::default()
        };
        let layout = TiledPaneLayout {
            run: Some(Run::Command(run)),
            ..TiledPaneLayout::default()
        };
        let _ = self.to_screen.send(ScreenInstruction::NewTab(
            Some(cwd),
            None, // default_shell — unused, the layout carries the command
            Some(layout),
            vec![], // no floating panes
            Some(name.to_string()),
            (None, None), // fall back to the session's swap layouts
            None,         // initial_panes
            false,        // block_on_first_terminal
            true,         // should_change_focus_to_new_tab
            (self.client_id, false),
            None, // no completion signal — fire-and-forget
        ));
    }

    /// Swap the **content region** beside the constant nav to show the working set
    /// keyed by `key` (ADR-0022/0023) — the native switcher that supersedes the
    /// tab model. The content region is an **ordered working set** (020-aux-tool-panes):
    /// a primary command pane (grove's harness, `command args…` in `cwd`), an
    /// optional secondary **host surface** (grove's per-grove detail), and the `aux`
    /// command members (terminal/yazi/vcs), each a `(role, RunCommand)` carrying its
    /// own worktree cwd. First selection of `key` spawns every command member and —
    /// if `secondary_surface_key` names a surface previously stashed via
    /// [`register_keyed_host_surface`] — mounts it as the secondary; re-selection
    /// restores `key`'s parked-alive **set** (every member's scrollback + the detail
    /// state intact). The opaque `key` is the host's own identifier (grove passes a
    /// grove name); the caller need not track open/parked state — the screen thread
    /// dedupes (already-shown → no-op, parked → restore, new → spawn). Fire-and-
    /// forget like the other verbs.
    ///
    /// `secondary_surface_key` is the **id-only** half of the registry seam
    /// (ADR-0023): a `Box<dyn HostSurface>` cannot ride a `Clone + Debug`
    /// `ScreenInstruction`, so the host stashes the built surface under a key and
    /// passes only the key here; the screen thread takes it from the registry when
    /// it mounts the set. `None` mounts a working set without a secondary surface.
    /// Each `aux` member's `role` is the host's opaque tag (also the
    /// [`toggle_member`](Self::toggle_member) handle); trellis never interprets it.
    pub fn swap_content(
        &self,
        key: &str,
        cwd: PathBuf,
        command: PathBuf,
        args: Vec<String>,
        secondary_surface_key: Option<String>,
        aux: Vec<(String, RunCommand)>,
    ) {
        let run = RunCommand {
            command,
            args,
            cwd: Some(cwd),
            ..RunCommand::default()
        };
        let _ = self.to_screen.send(ScreenInstruction::SwapContent {
            key: key.to_string(),
            run,
            secondary_surface_key,
            aux,
            client_id: self.client_id,
        });
    }

    /// Toggle the visibility of one member (named by its opaque `role`) of the
    /// **currently-mounted** working set `key` (150-working-set/010). Hiding a member
    /// suppresses it alive and re-tiles its content-region siblings to fill the gap;
    /// showing it restores it beside a still-visible sibling and re-tiles — the
    /// embedded child keeps running and its scrollback survives either way. `role` is
    /// the host's own member tag (grove: harness/detail/terminal/yazi/vcs); trellis
    /// never interprets it. A no-op when `key` is not the mounted set, the role is
    /// unknown, or a show finds no visible sibling. Fire-and-forget like the other
    /// verbs; only ids/keys ride the instruction (no surface — ADR-0023 Evidence #4).
    pub fn toggle_member(&self, key: &str, role: &str) {
        let _ = self.to_screen.send(ScreenInstruction::ToggleContentMember {
            key: key.to_string(),
            role: role.to_string(),
            client_id: self.client_id,
        });
    }

    /// Focus the existing tab named `name` (the native replacement for `zellij
    /// action go-to-tab-by-id`). `create = false`, so a missing tab is a no-op
    /// rather than spawning an empty tab — the command tab is only ever created
    /// by [`new_command_tab`](Self::new_command_tab).
    pub fn focus_tab(&self, name: &str) {
        let _ = self.to_screen.send(ScreenInstruction::GoToTabName(
            name.to_string(),
            None,
            false, // do not create
            Some(self.client_id),
            None,
        ));
    }

    /// Close the tab named `name` (native replacement for `close-tab-by-id`).
    /// Focus it by name first, then close the now-active tab — the two
    /// instructions are processed in order on the screen thread, so this closes
    /// the intended tab without needing its numeric id.
    pub fn close_tab(&self, name: &str) {
        self.focus_tab(name);
        let _ = self
            .to_screen
            .send(ScreenInstruction::CloseTab(self.client_id, None));
    }

    /// Ask the screen thread to **tick** this host pane: post a
    /// [`ScreenInstruction::HostSurfaceTick`] naming this pane. The handler calls
    /// the surface's [`tick`](HostSurface::tick) and re-renders if it asks.
    /// Safe to call from any thread — this is how grove's fs-watch thread wakes
    /// a redraw when the filesystem changes with no keypress.
    pub fn request_tick(&self) {
        let _ = self
            .to_screen
            .send(ScreenInstruction::HostSurfaceTick(self.pane_id));
    }

    /// Open `$EDITOR <tempfile>` as a real trellis terminal pane in place of this
    /// host pane, observing the editor child's exit (ADR-0020 §6, first slice of
    /// embedded-tool observability). While the editor runs the host pane is
    /// suppressed (parked alive); on exit the editor pane closes itself, the host
    /// pane is restored into its slot, and this surface's
    /// [`HostSurface::editor_exited`] fires so it can read `tempfile` back. The
    /// editor binary (`$EDITOR`/`$VISUAL`/`vi`) is resolved by trellis's existing
    /// in-place-editor machinery — the surface only seeds the file and owns its
    /// lifetime until `editor_exited`. Fire-and-forget like the other verbs; the
    /// caller must be the focused pane (the host pane the editor replaces is the
    /// focused one).
    pub fn open_editor(&self, tempfile: PathBuf) {
        let _ = self.to_screen.send(ScreenInstruction::OpenHostEditor {
            host_pane_id: self.pane_id,
            tempfile,
            client_id: self.client_id,
        });
    }

    /// Quit the session (native replacement for the v1 dashboard's `q`). Posts a
    /// client-exit to the server thread, which tears the session down.
    pub fn quit(&self) {
        let _ = self
            .to_server
            .send(ServerInstruction::ClientExit(self.client_id, None));
    }
}

// ---------------------------------------------------------------------------
// Injection: a one-shot host-surface factory the host sets before `start_server`.
// ---------------------------------------------------------------------------

type HostSurfaceFactory = Box<dyn FnOnce() -> Box<dyn HostSurface> + Send>;

static HOST_SURFACE_FACTORY: Mutex<Option<HostSurfaceFactory>> = Mutex::new(None);
static NEXT_HOST_PANE_ID: AtomicU32 = AtomicU32::new(0);

/// The keyed host-surface registry (ADR-0023): surfaces the host builds **at
/// runtime, server-side** (grove's per-grove detail) and stashes under an opaque
/// key, to be taken when a [`ScreenInstruction::SwapContent`] mounts them as the
/// secondary pane of a content pair. A `Vec` (not a `HashMap`) so the static needs
/// no lazy init — `Vec::new()` is const, and a session never holds more than a
/// handful of pending surfaces. Distinct from [`HOST_SURFACE_FACTORY`]: that is the
/// one-shot **nav** injected before `start_server` (it cannot be built server-side
/// because the surface value cannot cross the re-exec); these are built in-process
/// once the session is up.
static KEYED_HOST_SURFACES: Mutex<Vec<(String, Box<dyn HostSurface>)>> = Mutex::new(Vec::new());

/// The one-shot factory for the grove-owned **whichkey** bar (ADR-0019, leaf
/// 140): a second statically-injected host pane, distinct from the [`nav`
/// factory`](HOST_SURFACE_FACTORY) above. It cannot ride the nav factory (each is
/// taken once and injected into a different layout pane), and unlike the per-grove
/// detail surfaces it is **not** built server-side — like the nav, the whichkey
/// value cannot cross the re-exec, so the host registers a factory before
/// `start_server`. A dedicated slot rather than a general multi-host registry
/// keeps the seam minimal (constraint 4): the session has exactly two
/// statically-placed host panes (nav + whichkey).
static WHICHKEY_SURFACE_FACTORY: Mutex<Option<HostSurfaceFactory>> = Mutex::new(None);

/// Register the host-surface factory the server injects once, at first-layout.
/// Call this on the `--server` path (grove's `run_server`) **before**
/// `start_server`. Overwrites any previously-registered factory.
pub fn register_host_surface(factory: HostSurfaceFactory) {
    if let Ok(mut slot) = HOST_SURFACE_FACTORY.lock() {
        *slot = Some(factory);
    }
}

/// Take the registered factory and build the surface, if one was registered.
/// Returns `None` after the first successful take (a single host pane per
/// session) or when the host registered nothing (stock trellis behaviour).
pub(crate) fn take_host_surface() -> Option<Box<dyn HostSurface>> {
    let factory = HOST_SURFACE_FACTORY.lock().ok()?.take()?;
    Some(factory())
}

/// Register the **whichkey** host-surface factory (the grove-owned bottom hint
/// bar). Same contract as [`register_host_surface`]: call on the `--server` path
/// before `start_server`; overwrites any prior factory. The session injects it at
/// first-layout into the layout's `grove-whichkey` pane.
pub fn register_whichkey_surface(factory: HostSurfaceFactory) {
    if let Ok(mut slot) = WHICHKEY_SURFACE_FACTORY.lock() {
        *slot = Some(factory);
    }
}

/// Take the registered whichkey factory and build the surface, if one was
/// registered. `None` after the first take, or when the host registered no
/// whichkey (a layout without the bar — then the `grove-whichkey` pane, if any,
/// stays a plain placeholder, which is why grove only emits the pane when it also
/// registers the surface).
pub(crate) fn take_whichkey_surface() -> Option<Box<dyn HostSurface>> {
    let factory = WHICHKEY_SURFACE_FACTORY.lock().ok()?.take()?;
    Some(factory())
}

/// Stash a built host surface under `key` for a later [`SwapContent`](crate::screen::ScreenInstruction::SwapContent)
/// to mount as the secondary pane of a content pair. Replaces any surface already
/// registered under the same key (idempotent re-selection before the mount lands).
/// Called by the host on the screen thread (grove's nav surface, when a grove is
/// first selected).
pub fn register_keyed_host_surface(key: String, surface: Box<dyn HostSurface>) {
    if let Ok(mut reg) = KEYED_HOST_SURFACES.lock() {
        reg.retain(|(k, _)| k != &key);
        reg.push((key, surface));
    }
}

/// Take the surface stashed under `key`, if any. One-shot per key (the surface is
/// removed): the screen thread takes it exactly when it mounts the pair, after
/// which the live `HostPane` owns it.
pub(crate) fn take_keyed_host_surface(key: &str) -> Option<Box<dyn HostSurface>> {
    let mut reg = KEYED_HOST_SURFACES.lock().ok()?;
    let idx = reg.iter().position(|(k, _)| k == key)?;
    Some(reg.swap_remove(idx).1)
}

/// A fresh id for a host pane (the `u32` inside `PaneId::Host`). Independent of
/// the pty/plugin id spaces — host panes are never known to those threads.
pub(crate) fn next_host_pane_id() -> u32 {
    NEXT_HOST_PANE_ID.fetch_add(1, Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// The ratatui-Buffer → CharacterChunk converter (pure; unit-tested below).
// ---------------------------------------------------------------------------

/// Convert an off-screen `ratatui` [`Buffer`] into trellis [`CharacterChunk`]s,
/// one chunk per row, positioned at the pane's content origin.
///
/// `content_x` / `content_y` are the pane's absolute content coordinates on
/// screen (from `get_content_x`/`get_content_y`); the converter offsets each row
/// by them, matching what `TerminalPane::render` does with its grid. The function
/// is pure over `(buffer, content_x, content_y)`, so it is testable with a
/// synthetic `Buffer` and no server.
pub fn buffer_to_character_chunks(
    buffer: &Buffer,
    content_x: usize,
    content_y: usize,
) -> Vec<CharacterChunk> {
    let area = buffer.area;
    let mut chunks = Vec::with_capacity(area.height as usize);
    for row in 0..area.height {
        let mut characters = Vec::with_capacity(area.width as usize);
        for col in 0..area.width {
            let cell = &buffer[(area.x + col, area.y + row)];
            let character = cell.symbol().chars().next().unwrap_or(' ');
            let styles = cell_styles(cell.fg, cell.bg, cell.modifier);
            characters.push(TerminalCharacter::new_styled(character, styles));
        }
        chunks.push(CharacterChunk::new(
            characters,
            content_x,
            content_y + row as usize,
        ));
    }
    chunks
}

/// Build trellis cell styles from a ratatui cell's fg/bg/modifier.
fn cell_styles(fg: Color, bg: Color, modifier: Modifier) -> RcCharacterStyles {
    let mut styles = CharacterStyles {
        foreground: color_to_ansi(fg),
        background: color_to_ansi(bg),
        ..DEFAULT_STYLES
    };
    let on = |present: bool| present.then_some(AnsiCode::On);
    styles.bold = on(modifier.contains(Modifier::BOLD));
    styles.dim = on(modifier.contains(Modifier::DIM));
    styles.italic = on(modifier.contains(Modifier::ITALIC));
    styles.underline = on(modifier.contains(Modifier::UNDERLINED));
    styles.slow_blink = on(modifier.contains(Modifier::SLOW_BLINK));
    styles.fast_blink = on(modifier.contains(Modifier::RAPID_BLINK));
    styles.reverse = on(modifier.contains(Modifier::REVERSED));
    styles.hidden = on(modifier.contains(Modifier::HIDDEN));
    styles.strike = on(modifier.contains(Modifier::CROSSED_OUT));
    styles.into()
}

/// Map a ratatui [`Color`] onto trellis's [`AnsiCode`]. `None` is never returned
/// for a real color (`Reset` maps to `AnsiCode::Reset`), so every cell carries an
/// explicit fg/bg and renders deterministically.
fn color_to_ansi(color: Color) -> Option<AnsiCode> {
    Some(match color {
        Color::Reset => AnsiCode::Reset,
        Color::Black => AnsiCode::NamedColor(NamedColor::Black),
        Color::Red => AnsiCode::NamedColor(NamedColor::Red),
        Color::Green => AnsiCode::NamedColor(NamedColor::Green),
        Color::Yellow => AnsiCode::NamedColor(NamedColor::Yellow),
        Color::Blue => AnsiCode::NamedColor(NamedColor::Blue),
        Color::Magenta => AnsiCode::NamedColor(NamedColor::Magenta),
        Color::Cyan => AnsiCode::NamedColor(NamedColor::Cyan),
        // ratatui's `Gray` is ANSI 7 (white); `White` is ANSI 15 (bright white).
        Color::Gray => AnsiCode::NamedColor(NamedColor::White),
        Color::DarkGray => AnsiCode::NamedColor(NamedColor::BrightBlack),
        Color::LightRed => AnsiCode::NamedColor(NamedColor::BrightRed),
        Color::LightGreen => AnsiCode::NamedColor(NamedColor::BrightGreen),
        Color::LightYellow => AnsiCode::NamedColor(NamedColor::BrightYellow),
        Color::LightBlue => AnsiCode::NamedColor(NamedColor::BrightBlue),
        Color::LightMagenta => AnsiCode::NamedColor(NamedColor::BrightMagenta),
        Color::LightCyan => AnsiCode::NamedColor(NamedColor::BrightCyan),
        Color::White => AnsiCode::NamedColor(NamedColor::BrightWhite),
        Color::Rgb(r, g, b) => AnsiCode::RgbCode((r, g, b)),
        Color::Indexed(i) => AnsiCode::ColorIndex(i),
    })
}

/// Convert a trellis [`KeyWithModifier`] into a `crossterm` [`KeyEvent`] for the
/// host surface. Returns `None` for keys the surface model does not cover (the
/// pane then ignores them). This is the inverse of the client-side
/// `cast_termwiz_key`, restricted to the common keys grove's surfaces consume.
pub fn key_with_modifier_to_crossterm(key: &KeyWithModifier) -> Option<KeyEvent> {
    let code = match key.bare_key {
        BareKey::Char(c) => KeyCode::Char(c),
        BareKey::Backspace => KeyCode::Backspace,
        BareKey::Left => KeyCode::Left,
        BareKey::Right => KeyCode::Right,
        BareKey::Up => KeyCode::Up,
        BareKey::Down => KeyCode::Down,
        BareKey::Home => KeyCode::Home,
        BareKey::End => KeyCode::End,
        BareKey::PageUp => KeyCode::PageUp,
        BareKey::PageDown => KeyCode::PageDown,
        BareKey::Tab => KeyCode::Tab,
        BareKey::Delete => KeyCode::Delete,
        BareKey::Insert => KeyCode::Insert,
        BareKey::Enter => KeyCode::Enter,
        BareKey::Esc => KeyCode::Esc,
        BareKey::F(n) => KeyCode::F(n),
        _ => return None,
    };
    let mut modifiers = KeyModifiers::empty();
    for m in &key.key_modifiers {
        match m {
            KeyModifier::Ctrl => modifiers.insert(KeyModifiers::CONTROL),
            KeyModifier::Alt => modifiers.insert(KeyModifiers::ALT),
            KeyModifier::Shift => modifiers.insert(KeyModifiers::SHIFT),
            KeyModifier::Super => modifiers.insert(KeyModifiers::SUPER),
        }
    }
    Some(KeyEvent::new(code, modifiers))
}

// ---------------------------------------------------------------------------
// The pane itself.
// ---------------------------------------------------------------------------

/// trellis's third pane kind: a [`Pane`] backed by a host-supplied
/// [`HostSurface`]. See the module docs.
pub struct HostPane {
    pid: u32,
    geom: PaneGeom,
    geom_override: Option<PaneGeom>,
    content_offset: Offset,
    surface: Box<dyn HostSurface>,
    should_render: bool,
    selectable: bool,
    borderless: bool,
    exclude_from_sync: bool,
    style: Style,
    title: String,
    active_at: Instant,
    pane_frame_color_override: Option<(PaletteColor, Option<String>)>,
    frame: std::collections::HashMap<ClientId, PaneFrame>,
    // Host panes are not spawned from a `Run`; this is always `None` and exists
    // only so `invoked_with` can return a reference of the right type.
    invoked_with: Option<Run>,
}

impl HostPane {
    /// Build a host pane and hand its surface a [`HostDriver`] wired to the
    /// screen/server senders. Called from `Tab::inject_host_pane`, which passes
    /// its own bus senders (`to_screen` / `to_server`) and the focusing client —
    /// the screen thread is the only place those senders exist, which is why the
    /// driver cannot be set up earlier (in the pre-`start_server` factory).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pid: u32,
        geom: PaneGeom,
        style: Style,
        mut surface: Box<dyn HostSurface>,
        title: String,
        to_screen: SenderWithContext<ScreenInstruction>,
        to_server: SenderWithContext<ServerInstruction>,
        client_id: ClientId,
    ) -> Self {
        surface.set_driver(HostDriver {
            to_screen,
            to_server,
            pane_id: pid,
            client_id,
        });
        HostPane {
            pid,
            geom,
            geom_override: None,
            content_offset: Offset::default(),
            surface,
            should_render: true,
            selectable: true,
            borderless: false,
            exclude_from_sync: false,
            style,
            title,
            active_at: Instant::now(),
            pane_frame_color_override: None,
            frame: std::collections::HashMap::new(),
            invoked_with: None,
        }
    }

    fn get_x(&self) -> usize {
        self.geom_override.unwrap_or(self.geom).x
    }
    fn get_y(&self) -> usize {
        self.geom_override.unwrap_or(self.geom).y
    }
    fn get_columns(&self) -> usize {
        self.geom_override.unwrap_or(self.geom).cols.as_usize()
    }
    fn get_rows(&self) -> usize {
        self.geom_override.unwrap_or(self.geom).rows.as_usize()
    }

    /// Tell the surface its current content size (called on any geometry change).
    fn notify_surface_resize(&mut self) {
        let cols = self.get_content_columns() as u16;
        let rows = self.get_content_rows() as u16;
        self.surface.resize(cols, rows);
        self.should_render = true;
    }
}

impl Pane for HostPane {
    fn x(&self) -> usize {
        self.get_x()
    }
    fn y(&self) -> usize {
        self.get_y()
    }
    fn rows(&self) -> usize {
        self.get_rows()
    }
    fn cols(&self) -> usize {
        self.get_columns()
    }
    fn get_content_x(&self) -> usize {
        self.get_x() + self.content_offset.left
    }
    fn get_content_y(&self) -> usize {
        self.get_y() + self.content_offset.top
    }
    fn get_content_columns(&self) -> usize {
        self.get_columns()
            .saturating_sub(self.content_offset.left + self.content_offset.right)
    }
    fn get_content_rows(&self) -> usize {
        self.get_rows()
            .saturating_sub(self.content_offset.top + self.content_offset.bottom)
    }
    fn reset_size_and_position_override(&mut self) {
        self.geom_override = None;
        self.notify_surface_resize();
    }
    fn set_geom(&mut self, position_and_size: PaneGeom) {
        let is_pinned = self.geom.is_pinned;
        self.geom = position_and_size;
        self.geom.is_pinned = is_pinned;
        self.notify_surface_resize();
    }
    fn set_geom_override(&mut self, pane_geom: PaneGeom) {
        self.geom_override = Some(pane_geom);
        self.notify_surface_resize();
    }
    fn cursor_coordinates(&self, _client_id: Option<ClientId>) -> Option<(usize, usize, bool)> {
        if self.get_content_rows() < 1 || self.get_content_columns() < 1 {
            return None;
        }
        let Offset { top, left, .. } = self.content_offset;
        self.surface
            .cursor()
            .map(|(x, y)| (x as usize + left, y as usize + top, true))
    }
    fn adjust_input_to_terminal(
        &mut self,
        key_with_modifier: &Option<KeyWithModifier>,
        _raw_input_bytes: Vec<u8>,
        _raw_input_bytes_are_kitty: bool,
        _client_id: Option<ClientId>,
    ) -> Option<AdjustedInput> {
        // Host panes consume input in-process: deliver the key to the surface and
        // return `None` (nothing is routed to a pty — there isn't one). The Tab's
        // `PaneId::Host` arm calls this and re-renders regardless.
        if let Some(key) = key_with_modifier.as_ref() {
            if let Some(event) = key_with_modifier_to_crossterm(key) {
                if self.surface.handle_key(event) {
                    self.should_render = true;
                }
            }
        }
        None
    }
    fn position_and_size(&self) -> PaneGeom {
        self.geom
    }
    fn current_geom(&self) -> PaneGeom {
        self.geom_override.unwrap_or(self.geom)
    }
    fn geom_override(&self) -> Option<PaneGeom> {
        self.geom_override
    }
    fn should_render(&self) -> bool {
        self.should_render
    }
    fn set_should_render(&mut self, should_render: bool) {
        self.should_render = should_render;
    }
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn set_selectable(&mut self, selectable: bool) {
        self.selectable = selectable;
    }
    fn render(
        &mut self,
        _client_id: Option<ClientId>,
    ) -> Result<Option<(Vec<CharacterChunk>, Option<String>, Vec<SixelImageChunk>)>> {
        if !self.should_render {
            return Ok(None);
        }
        let rows = self.get_content_rows();
        let cols = self.get_content_columns();
        if rows < 1 || cols < 1 {
            return Ok(None);
        }
        let area = Rect::new(0, 0, cols as u16, rows as u16);
        let mut buffer = Buffer::empty(area);
        self.surface.draw(area, &mut buffer);
        let chunks = buffer_to_character_chunks(&buffer, self.get_content_x(), self.get_content_y());
        self.should_render = false;
        Ok(Some((chunks, None, vec![])))
    }
    fn render_frame(
        &mut self,
        client_id: ClientId,
        frame_params: FrameParams,
        _input_mode: InputMode,
    ) -> Result<Option<(Vec<CharacterChunk>, Option<String>)>> {
        if self.borderless {
            return Ok(None);
        }
        let err_context = || format!("failed to render host pane frame for client {client_id}");
        let frame_geom = self.current_geom();
        let is_pinned = frame_geom.is_pinned;
        let mut frame =
            PaneFrame::new(frame_geom.into(), (0, 0), self.title.clone(), frame_params)
                .is_pinned(is_pinned);
        if let Some((frame_color_override, _text)) = self.pane_frame_color_override.as_ref() {
            frame.override_color(*frame_color_override);
        }
        let res = match self.frame.get(&client_id) {
            Some(last_frame) if last_frame == &frame => None,
            _ => {
                let frame_output = frame.render().with_context(err_context)?;
                self.frame.insert(client_id, frame);
                Some(frame_output)
            },
        };
        Ok(res)
    }
    fn render_fake_cursor(
        &mut self,
        _cursor_color: PaletteColor,
        _text_color: PaletteColor,
    ) -> Option<String> {
        None
    }
    fn render_terminal_title(&mut self, _input_mode: InputMode) -> String {
        self.title.clone()
    }
    fn update_name(&mut self, name: &str) {
        self.title = name.to_owned();
        self.should_render = true;
    }
    fn pid(&self) -> PaneId {
        PaneId::Host(self.pid)
    }
    fn reduce_height(&mut self, percent: f64) {
        if let Some(p) = self.geom.rows.as_percent() {
            self.geom.rows.set_percent(p - percent);
            self.notify_surface_resize();
        }
    }
    fn increase_height(&mut self, percent: f64) {
        if let Some(p) = self.geom.rows.as_percent() {
            self.geom.rows.set_percent(p + percent);
            self.notify_surface_resize();
        }
    }
    fn reduce_width(&mut self, percent: f64) {
        if let Some(p) = self.geom.cols.as_percent() {
            self.geom.cols.set_percent(p - percent);
            self.notify_surface_resize();
        }
    }
    fn increase_width(&mut self, percent: f64) {
        if let Some(p) = self.geom.cols.as_percent() {
            self.geom.cols.set_percent(p + percent);
            self.notify_surface_resize();
        }
    }
    fn push_down(&mut self, count: usize) {
        self.geom.y += count;
        self.should_render = true;
    }
    fn push_right(&mut self, count: usize) {
        self.geom.x += count;
        self.should_render = true;
    }
    fn pull_left(&mut self, count: usize) {
        self.geom.x = self.geom.x.saturating_sub(count);
        self.should_render = true;
    }
    fn pull_up(&mut self, count: usize) {
        self.geom.y = self.geom.y.saturating_sub(count);
        self.should_render = true;
    }
    fn clear_screen(&mut self) {
        self.should_render = true;
    }
    fn scroll_up(&mut self, _count: usize, _client_id: ClientId) {}
    fn scroll_down(&mut self, _count: usize, _client_id: ClientId) {}
    fn clear_scroll(&mut self) {}
    fn is_scrolled(&self) -> bool {
        false
    }
    fn active_at(&self) -> Instant {
        self.active_at
    }
    fn set_active_at(&mut self, instant: Instant) {
        self.active_at = instant;
    }
    fn set_frame(&mut self, _frame: bool) {}
    fn set_content_offset(&mut self, offset: Offset) {
        self.content_offset = offset;
        self.notify_surface_resize();
    }
    fn get_content_offset(&self) -> Offset {
        self.content_offset
    }
    fn store_pane_name(&mut self) {}
    fn load_pane_name(&mut self) {}
    fn set_borderless(&mut self, borderless: bool) {
        self.borderless = borderless;
    }
    fn borderless(&self) -> bool {
        self.borderless
    }
    fn set_exclude_from_sync(&mut self, exclude_from_sync: bool) {
        self.exclude_from_sync = exclude_from_sync;
    }
    fn exclude_from_sync(&self) -> bool {
        self.exclude_from_sync
    }
    fn focus_event(&self) -> Option<String> {
        None
    }
    fn set_focused(&mut self, focused: bool) {
        self.surface.set_focused(focused);
        self.should_render = true;
    }
    fn host_tick(&mut self) -> bool {
        if self.surface.tick() {
            self.should_render = true;
            true
        } else {
            false
        }
    }
    fn host_editor_exited(&mut self, exit_status: Option<i32>) -> bool {
        if self.surface.editor_exited(exit_status) {
            self.should_render = true;
            true
        } else {
            false
        }
    }
    fn add_red_pane_frame_color_override(&mut self, error_text: Option<String>) {
        self.pane_frame_color_override =
            Some((self.style.colors.exit_code_error.base, error_text));
    }
    fn clear_pane_frame_color_override(&mut self, _client_id: Option<ClientId>) {
        self.pane_frame_color_override = None;
    }
    fn frame_color_override(&self) -> Option<PaletteColor> {
        self.pane_frame_color_override
            .as_ref()
            .map(|(color, _text)| *color)
    }
    fn invoked_with(&self) -> &Option<Run> {
        &self.invoked_with
    }
    fn set_title(&mut self, title: String) {
        self.title = title;
    }
    fn current_title(&self) -> String {
        self.title.clone()
    }
    fn custom_title(&self) -> Option<String> {
        None
    }
    fn set_should_be_suppressed(&mut self, _should_be_suppressed: bool) {}
    fn pane_contents(
        &self,
        _client_id: Option<ClientId>,
        _get_full_scrollback: bool,
        _max_scrollback_lines: Option<usize>,
    ) -> PaneContents {
        PaneContents::default()
    }
    fn relative_position(&self, position_on_screen: &Position) -> Position {
        position_on_screen.relative_to(self.get_content_y(), self.get_content_x())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Stylize;
    use ratatui::text::Line;
    use ratatui::widgets::Widget;

    /// The whichkey factory is a one-shot, distinct from the nav factory: the host
    /// registers it before `start_server`, the session takes it exactly once at
    /// first-layout, and a second take is empty (one whichkey pane per session).
    #[test]
    fn whichkey_factory_is_a_one_shot() {
        struct Dummy;
        impl HostSurface for Dummy {
            fn draw(&mut self, _area: Rect, _buf: &mut Buffer) {}
        }
        register_whichkey_surface(Box::new(|| Box::new(Dummy)));
        assert!(
            take_whichkey_surface().is_some(),
            "first take yields the registered whichkey surface"
        );
        assert!(
            take_whichkey_surface().is_none(),
            "the whichkey factory is one-shot"
        );
    }

    #[test]
    fn converter_emits_one_chunk_per_row_at_content_origin() {
        let area = Rect::new(0, 0, 4, 3);
        let buffer = Buffer::empty(area);
        let chunks = buffer_to_character_chunks(&buffer, 10, 5);
        assert_eq!(chunks.len(), 3, "one chunk per row");
        for (row, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.x, 10, "chunk x is the content x");
            assert_eq!(chunk.y, 5 + row, "chunk y is content y + row index");
            assert_eq!(chunk.terminal_characters.len(), 4, "one cell per column");
        }
    }

    #[test]
    fn converter_carries_text_and_default_styling() {
        let area = Rect::new(0, 0, 5, 1);
        let mut buffer = Buffer::empty(area);
        Line::from("hi").render(area, &mut buffer);
        let chunks = buffer_to_character_chunks(&buffer, 0, 0);
        let row: String = chunks[0]
            .terminal_characters
            .iter()
            .map(|c| c.character)
            .collect();
        assert_eq!(row, "hi   ", "text then padding to the buffer width");
    }

    #[test]
    fn converter_maps_colors_and_modifiers() {
        let area = Rect::new(0, 0, 1, 1);
        let mut buffer = Buffer::empty(area);
        // A bold red-on-blue 'X'.
        Line::from("X".red().on_blue().bold()).render(area, &mut buffer);
        let cell = &chunks_first_char(&buffer);
        assert_eq!(cell.character, 'X');
        assert_eq!(cell.styles.foreground, Some(AnsiCode::NamedColor(NamedColor::Red)));
        assert_eq!(cell.styles.background, Some(AnsiCode::NamedColor(NamedColor::Blue)));
        assert_eq!(cell.styles.bold, Some(AnsiCode::On));
        assert_eq!(cell.styles.italic, None);
    }

    fn chunks_first_char(buffer: &Buffer) -> TerminalCharacter {
        let chunks = buffer_to_character_chunks(buffer, 0, 0);
        chunks[0].terminal_characters[0].clone()
    }

    #[test]
    fn rgb_and_indexed_colors_round_trip() {
        assert_eq!(color_to_ansi(Color::Rgb(1, 2, 3)), Some(AnsiCode::RgbCode((1, 2, 3))));
        assert_eq!(color_to_ansi(Color::Indexed(200)), Some(AnsiCode::ColorIndex(200)));
        assert_eq!(color_to_ansi(Color::Reset), Some(AnsiCode::Reset));
        // ratatui Gray => ANSI white(7); White => bright white(15).
        assert_eq!(color_to_ansi(Color::Gray), Some(AnsiCode::NamedColor(NamedColor::White)));
        assert_eq!(
            color_to_ansi(Color::White),
            Some(AnsiCode::NamedColor(NamedColor::BrightWhite))
        );
    }

    #[test]
    fn key_conversion_covers_chars_arrows_and_modifiers() {
        let ctrl_c = KeyWithModifier::new(BareKey::Char('c')).with_ctrl_modifier();
        let event = key_with_modifier_to_crossterm(&ctrl_c).expect("ctrl-c maps");
        assert_eq!(event.code, KeyCode::Char('c'));
        assert!(event.modifiers.contains(KeyModifiers::CONTROL));

        let up = KeyWithModifier::new(BareKey::Up);
        assert_eq!(
            key_with_modifier_to_crossterm(&up).map(|e| e.code),
            Some(KeyCode::Up)
        );
    }

    /// The consumer-facing host-API contract (ADR-0021): each [`HostDriver`] verb
    /// posts exactly the expected [`ScreenInstruction`] onto the screen channel.
    /// This pins the seam grove's dashboard (and 120–150) drives layout through.
    #[test]
    fn host_driver_posts_the_expected_screen_instructions() {
        use zellij_utils::channels::unbounded;
        use zellij_utils::errors::ErrorContext;

        let (screen_tx, screen_rx) = unbounded::<(ScreenInstruction, ErrorContext)>();
        let (server_tx, _server_rx) = unbounded::<(ServerInstruction, ErrorContext)>();
        let driver = HostDriver {
            to_screen: SenderWithContext::new(screen_tx),
            to_server: SenderWithContext::new(server_tx),
            pane_id: 7,
            client_id: 1,
        };
        let next = || screen_rx.try_recv().map(|(i, _)| i).expect("an instruction");

        // request_tick names this pane so the handler finds it.
        driver.request_tick();
        assert!(matches!(next(), ScreenInstruction::HostSurfaceTick(7)));

        // open_editor → an OpenHostEditor naming this host pane + the seeded
        // tempfile, issued as this client (ADR-0020 §6). The screen handler forwards
        // it to the pty in-place-editor spawn; here we pin the host-facing contract.
        driver.open_editor(PathBuf::from("/tmp/grove-edit-xyz.md"));
        match next() {
            ScreenInstruction::OpenHostEditor {
                host_pane_id,
                tempfile,
                client_id,
            } => {
                assert_eq!(host_pane_id, 7, "names this host pane");
                assert_eq!(tempfile, PathBuf::from("/tmp/grove-edit-xyz.md"));
                assert_eq!(client_id, 1);
            }
            other => panic!("expected OpenHostEditor, got {other:?}"),
        }

        // new_command_tab → a focused, named NewTab whose layout runs the command.
        driver.new_command_tab(
            "auth",
            PathBuf::from("/work/acme"),
            PathBuf::from("/opt/grove"),
            vec!["do".to_string(), "auth".to_string()],
        );
        match next() {
            ScreenInstruction::NewTab(cwd, _, layout, _, name, _, _, _, focus, (cid, _), _) => {
                assert_eq!(name.as_deref(), Some("auth"));
                assert_eq!(cwd, Some(PathBuf::from("/work/acme")));
                assert!(focus, "a freshly opened harness tab is focused");
                assert_eq!(cid, 1);
                match layout.expect("a tab layout").run.expect("a command run") {
                    Run::Command(rc) => {
                        assert_eq!(rc.command, PathBuf::from("/opt/grove"));
                        assert_eq!(rc.args, vec!["do".to_string(), "auth".to_string()]);
                        assert_eq!(rc.cwd, Some(PathBuf::from("/work/acme")));
                    }
                    other => panic!("expected Run::Command, got {other:?}"),
                }
            }
            other => panic!("expected NewTab, got {other:?}"),
        }

        // swap_content → a SwapContent carrying the primary (harness) run, the
        // secondary detail-surface key, and the **ordered aux command members**
        // (020-aux-tool-panes: terminal/yazi/vcs), each with its own worktree cwd.
        driver.swap_content(
            "auth",
            PathBuf::from("/work/acme/.grove-worktrees/auth"),
            PathBuf::from("/opt/grove"),
            vec!["do".to_string(), "auth".to_string()],
            Some("grove-detail:auth".to_string()),
            vec![
                (
                    "terminal".to_string(),
                    RunCommand {
                        command: PathBuf::from("/bin/zsh"),
                        cwd: Some(PathBuf::from("/work/acme/.grove-worktrees/auth")),
                        ..RunCommand::default()
                    },
                ),
                (
                    "vcs".to_string(),
                    RunCommand {
                        command: PathBuf::from("/usr/bin/lazygit"),
                        cwd: Some(PathBuf::from("/work/acme/.grove-worktrees/auth")),
                        ..RunCommand::default()
                    },
                ),
            ],
        );
        match next() {
            ScreenInstruction::SwapContent {
                key,
                run,
                secondary_surface_key,
                aux,
                client_id,
            } => {
                assert_eq!(key, "auth");
                assert_eq!(run.command, PathBuf::from("/opt/grove"));
                assert_eq!(run.args, vec!["do".to_string(), "auth".to_string()]);
                assert_eq!(secondary_surface_key.as_deref(), Some("grove-detail:auth"));
                assert_eq!(aux.len(), 2, "both aux members ride the instruction");
                assert_eq!(aux[0].0, "terminal", "aux roles preserved in order");
                assert_eq!(aux[0].1.command, PathBuf::from("/bin/zsh"));
                assert_eq!(
                    aux[0].1.cwd,
                    Some(PathBuf::from("/work/acme/.grove-worktrees/auth")),
                    "aux runs in the grove's worktree cwd"
                );
                assert_eq!(aux[1].0, "vcs");
                assert_eq!(aux[1].1.command, PathBuf::from("/usr/bin/lazygit"));
                assert_eq!(client_id, 1);
            }
            other => panic!("expected SwapContent, got {other:?}"),
        }

        // toggle_member → a ToggleContentMember naming the mounted set's key and the
        // opaque member role, as this client (150-working-set/030 — the toggle UX drives
        // this verb for the currently-mounted working set).
        driver.toggle_member("auth", "yazi");
        match next() {
            ScreenInstruction::ToggleContentMember {
                key,
                role,
                client_id,
            } => {
                assert_eq!(key, "auth", "names the mounted set");
                assert_eq!(role, "yazi", "carries the opaque member role verbatim");
                assert_eq!(client_id, 1);
            }
            other => panic!("expected ToggleContentMember, got {other:?}"),
        }

        // focus_tab → GoToTabName by name, create=false (never spawns an empty tab).
        driver.focus_tab("auth");
        match next() {
            ScreenInstruction::GoToTabName(name, _, create, cid, _) => {
                assert_eq!(name, "auth");
                assert!(!create, "focus must not create");
                assert_eq!(cid, Some(1));
            }
            other => panic!("expected GoToTabName, got {other:?}"),
        }

        // close_tab → focus the named tab, then close the now-active one.
        driver.close_tab("auth");
        assert!(
            matches!(next(), ScreenInstruction::GoToTabName(ref n, _, false, _, _) if n == "auth")
        );
        assert!(matches!(next(), ScreenInstruction::CloseTab(1, _)));

        // quit → a client-exit on the server channel.
        driver.quit();
        assert!(matches!(
            _server_rx.try_recv().map(|(i, _)| i),
            Ok(ServerInstruction::ClientExit(1, _))
        ));
    }
}

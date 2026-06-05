//! grove-nav — grove's [[leader]]-focused command surface, realised as a zellij
//! WASM plugin (ADR-0018, leaves 070-nav-plugin + 080-controller-plugin-pipe).
//!
//! ## Why a plugin (the one API fact this rests on)
//!
//! grove runs zellij in `default_mode "locked"`, so every key passes through to
//! the focused app — only a zellij keybind can intercept a key while a harness is
//! focused. And a keybind **cannot** focus a tiled pane by id: the *only* action
//! that lands focus on a named surface is `LaunchOrFocusPlugin "<name>"`, and a
//! focused plugin then receives every keypress (`Event::Key`, no permission). So
//! the leader-reachable control surface *must* be a plugin. `Ctrl-o` is bound to
//! `LaunchOrFocusPlugin "file:…"` (see `src/zellij.rs`); this crate is what it
//! focuses.
//!
//! ## What it does (with the 080 pipe)
//!
//! The nav lists **every grove in the repo** — not just the ones already open as
//! tabs — and switches to the selected one. It learns that full list from the
//! **controller**, which owns grove state (`RepoView` + fs-watch) and pushes it
//! over a `grove-state` [[pipe]] on startup and on every fs-watch settle (leaf
//! 080). The plugin holds **no grove state of its own** (ADR-0018): it renders
//! the piped list and cross-references zellij's own `TabUpdate` to know, per
//! grove, whether it is already open and where.
//!
//! That cross-reference drives the **split** (ADR-0018): selecting a grove that
//! already has a tab switches to it with pure-zellij `switch_tab_to` (snappy, no
//! round-trip); selecting one that is *not* yet open streams an `open <name>`
//! intent back up the **`grove-intent` back-channel** to the controller, which
//! first-opens the tab + working set via the 040 `zellij action` driver (so
//! `grove do` command/cwd composition stays in Rust). The back-channel is a CLI
//! pipe the controller holds open (by keeping its stdin open); the plugin
//! captures that invocation's `pipe_id` from the `__init` message and
//! `cli_pipe_output`s intents up it at any time.
//!
//! ## Persistent home sidebar
//!
//! The nav is **not** a vanish-on-use palette: the layout launches it as a
//! permanent left sidebar on the "home" tab and it stays open. The leader
//! (`Ctrl-o`, `LaunchOrFocusPlugin "file:…"`) focuses this running instance —
//! which lives on home, so `Ctrl-o` from anywhere brings you to home and onto the
//! nav. Acting (`Enter`/`l`) switches to / opens the selected grove; the nav stays
//! put on home as focus moves. Cancelling (`q`/`Esc`) hands focus to the sibling
//! dashboard pane via `focus_next_pane` rather than closing.

use std::collections::BTreeMap;

use zellij_tile::prelude::*;

/// Pipe message names — must match the controller's (`src/nav_pipe.rs`).
const STATE_PIPE: &str = "grove-state";
const INTENT_PIPE: &str = "grove-intent";

/// One grove as piped from the controller (the `grove-state` pipe, leaf 080).
/// The nav keeps nothing else — open-ness/position come from `TabUpdate`.
#[derive(Clone, Debug, PartialEq, Eq)]
struct GroveRow {
    name: String,
    /// Pending inbox observations, surfaced as a badge (the inbox is the point
    /// of grove, so a count belongs in the nav).
    pending: usize,
}

/// One zellij tab as the nav sees it (from `TabUpdate`): used only to resolve
/// whether a grove is already open (and where), to mark the active one, and to
/// find the "home" tab. Not a source of grove identity.
#[derive(Clone, Debug, PartialEq, Eq)]
struct TabEntry {
    name: String,
    /// 0-based tab position. `switch_tab_to` is 1-based, so switching uses
    /// `position + 1` (see [`activate`]).
    position: usize,
    active: bool,
}

/// What selecting a grove does — the ADR-0018 split driving, as a value so it is
/// unit-testable apart from the zellij-tile calls that carry it out.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Activation {
    /// The grove already has a tab → switch via pure zellij (0-based position).
    Switch(usize),
    /// The grove is not yet open → ask the controller to first-open it.
    Open(String),
}

#[derive(Default)]
struct State {
    /// The full grove list, piped from the controller. Ordered as the controller
    /// sends it (the dashboard's own grove order).
    groves: Vec<GroveRow>,
    /// zellij's live tab list (from `TabUpdate`), for open/active/position lookup.
    tabs: Vec<TabEntry>,
    /// The highlighted row in [`State::groves`].
    selected: usize,
    /// The current zellij input mode, shown in the hint footer (discoverability).
    mode: String,
    /// The `grove-intent` back-channel's CLI pipe id, captured from its `__init`
    /// message. `Some` once the controller has opened the channel; intents are
    /// streamed up it via `cli_pipe_output`.
    intent_pipe: Option<String>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // `switch_tab_to` needs ChangeApplicationState; reading tabs/mode needs
        // ReadApplicationState. Requested once on first load; zellij remembers the
        // grant across launches. (Pipe handling needs no permission/subscription.)
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::Key,
            EventType::TabUpdate,
            EventType::ModeUpdate,
            EventType::Visible,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::TabUpdate(tabs) => {
                self.tabs = tabs
                    .iter()
                    .map(|t| TabEntry {
                        name: t.name.clone(),
                        position: t.position,
                        active: t.active,
                    })
                    .collect();
                // A tab closing/opening can change which grove is active; keep the
                // selection in range but don't yank the cursor around.
                self.selected = clamp_index(self.selected, self.groves.len());
                true
            }
            Event::ModeUpdate(mode_info) => {
                self.mode = format!("{:?}", mode_info.mode).to_lowercase();
                true
            }
            // Re-focused (the leader fired again): re-anchor the cursor on the
            // active workspace so the nav opens "where you are."
            Event::Visible(true) => {
                if let Some(i) = active_grove_index(&self.groves, &self.tabs) {
                    self.selected = i;
                }
                true
            }
            Event::Key(key) => self.handle_key(key),
            _ => false,
        }
    }

    fn pipe(&mut self, message: PipeMessage) -> bool {
        match message.name.as_str() {
            STATE_PIPE => {
                let was_empty = self.groves.is_empty();
                if let Some(payload) = &message.payload {
                    self.groves = parse_groves(payload);
                }
                if was_empty {
                    // First list: start the cursor on the active workspace.
                    if let Some(i) = active_grove_index(&self.groves, &self.tabs) {
                        self.selected = i;
                    }
                }
                self.selected = clamp_index(self.selected, self.groves.len());
                // A one-shot state push: release the CLI invocation so it exits
                // promptly rather than lingering blocked.
                if let PipeSource::Cli(id) = &message.source {
                    unblock_cli_pipe_input(id);
                }
                true
            }
            INTENT_PIPE => {
                // The long-lived back-channel: capture this CLI invocation's id so
                // we can `cli_pipe_output` `open <name>` up it later. The
                // controller keeps the invocation alive by holding its stdin open;
                // we just record the id (and don't unblock — there is no next
                // stdin message we want).
                if let PipeSource::Cli(id) = message.source {
                    self.intent_pipe = Some(id);
                }
                false
            }
            _ => false,
        }
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        // The nav lives in a narrow sidebar; keep every line within `cols` and the
        // hints terse and stacked so nothing wraps.
        println!("\u{1b}[1m grove\u{1b}[0m");
        println!();
        if self.groves.is_empty() {
            println!("\u{1b}[2m (loading…)\u{1b}[0m");
        }
        for (i, g) in self.groves.iter().enumerate() {
            let arrow = if i == self.selected { "▸" } else { " " };
            // ● active workspace, ○ open but not focused, blank not yet open.
            let dot = match grove_open_state(g, &self.tabs) {
                OpenState::Active => "●",
                OpenState::Open => "○",
                OpenState::Closed => " ",
            };
            let label = truncate(&grove_label(g), cols.saturating_sub(5));
            if i == self.selected {
                // Reverse-video the selected row.
                println!("\u{1b}[7m {arrow} {dot} {label}\u{1b}[0m");
            } else {
                println!(" {arrow} {dot} {label}");
            }
        }
        println!();
        for hint in [" j/k move", " ent  go", " h   home", " esc back"] {
            println!("\u{1b}[2m{hint}\u{1b}[0m");
        }
        println!();
        println!("\u{1b}[2m mode: {}\u{1b}[0m", truncate(&self.mode, cols.saturating_sub(7)));
    }
}

impl State {
    /// Map a keypress to an action. Returns whether to re-render. Modified keys
    /// are ignored so the nav stays a plain, single-key palette.
    fn handle_key(&mut self, key: KeyWithModifier) -> bool {
        if !key.has_no_modifiers() {
            return false;
        }
        match key.bare_key {
            BareKey::Char('j') | BareKey::Down => {
                self.selected = next_index(self.selected, self.groves.len());
                true
            }
            BareKey::Char('k') | BareKey::Up => {
                self.selected = prev_index(self.selected, self.groves.len());
                true
            }
            BareKey::Char('h') => {
                self.jump_home();
                false
            }
            BareKey::Char('l') | BareKey::Enter => {
                self.activate_selected();
                false
            }
            BareKey::Char('q') | BareKey::Esc => {
                // Hand focus back to the sibling dashboard pane; the nav stays
                // open (it is the permanent home sidebar, not a palette).
                focus_next_pane();
                false
            }
            _ => false,
        }
    }

    /// Act on the selected grove (ADR-0018 split): switch to its tab if open,
    /// else stream an open-intent up the back-channel for the controller to
    /// first-open. The nav stays put on home; zellij moves focus.
    fn activate_selected(&mut self) {
        let Some(grove) = self.groves.get(self.selected) else {
            return;
        };
        match resolve_activation(grove, &self.tabs) {
            Activation::Switch(position) => activate(position),
            Activation::Open(name) => {
                if let Some(id) = &self.intent_pipe {
                    cli_pipe_output(id, &format!("open {name}\n"));
                }
                // No back-channel yet → drop silently; the controller pushes
                // state (and opens the channel) at startup, so this is a brief
                // pre-init window only.
            }
        }
    }

    /// Jump to the "home" (dashboard) workspace.
    fn jump_home(&mut self) {
        if let Some(pos) = home_position(&self.tabs) {
            activate(pos);
        }
    }
}

/// Truncate `s` to at most `max` characters (char-aware), for the narrow sidebar.
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        s.chars().take(max).collect()
    }
}

/// Switch zellij to the tab at 0-based `position`. `switch_tab_to` is 1-based
/// (index 0 means "tab 1"), so a position maps to `position + 1`.
fn activate(position: usize) {
    switch_tab_to(position as u32 + 1);
}

// ---------------------------------------------------------------------------
// Pure list logic (unit-tested; no zellij-tile calls).

/// Whether a grove is the active workspace, merely open, or not yet open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpenState {
    Active,
    Open,
    Closed,
}

/// Parse the `grove-state` pipe payload (leaf 080): one grove per line, `name \t
/// pending`. Blank lines are skipped; a missing or non-numeric pending column is
/// 0. Names cannot contain a tab (they are sanitised), so the split is safe.
fn parse_groves(payload: &str) -> Vec<GroveRow> {
    payload
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut cols = line.splitn(2, '\t');
            let name = cols.next().unwrap_or("").trim().to_string();
            let pending = cols.next().and_then(|p| p.trim().parse().ok()).unwrap_or(0);
            GroveRow { name, pending }
        })
        .filter(|g| !g.name.is_empty())
        .collect()
}

/// Resolve how to act on `grove`: switch to its tab if one is open (matched by
/// name — the controller names a grove's tab for the grove), else open it.
fn resolve_activation(grove: &GroveRow, tabs: &[TabEntry]) -> Activation {
    match tabs.iter().find(|t| t.name == grove.name) {
        Some(tab) => Activation::Switch(tab.position),
        None => Activation::Open(grove.name.clone()),
    }
}

/// The open/active state of `grove` per the live tab list.
fn grove_open_state(grove: &GroveRow, tabs: &[TabEntry]) -> OpenState {
    match tabs.iter().find(|t| t.name == grove.name) {
        Some(tab) if tab.active => OpenState::Active,
        Some(_) => OpenState::Open,
        None => OpenState::Closed,
    }
}

/// The render label for a grove: its name plus a `(N)` pending-inbox badge when
/// there are pending observations.
fn grove_label(grove: &GroveRow) -> String {
    if grove.pending > 0 {
        format!("{} ({})", grove.name, grove.pending)
    } else {
        grove.name.clone()
    }
}

/// The index into `groves` of the grove whose tab is currently active, if any.
/// `None` when the active tab is not a grove (e.g. "home") or there is none.
fn active_grove_index(groves: &[GroveRow], tabs: &[TabEntry]) -> Option<usize> {
    let active = tabs.iter().find(|t| t.active)?;
    groves.iter().position(|g| g.name == active.name)
}

/// The 0-based position of the tab named "home" (the dashboard), if present.
/// Falls back to the first tab's position so `h` always lands somewhere sane.
fn home_position(tabs: &[TabEntry]) -> Option<usize> {
    tabs.iter()
        .find(|t| t.name == "home")
        .or_else(|| tabs.first())
        .map(|t| t.position)
}

/// Clamp `idx` into `0..len` (saturating to the last row), or 0 when empty.
fn clamp_index(idx: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        idx.min(len - 1)
    }
}

/// Next row, wrapping to the top. A no-op when the list is empty.
fn next_index(idx: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        (idx + 1) % len
    }
}

/// Previous row, wrapping to the bottom. A no-op when the list is empty.
fn prev_index(idx: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        (idx + len - 1) % len
    }
}

// zellij provides `host_run_plugin_command` as a wasm import at runtime; every
// `zellij_tile` shim call funnels through it. A host-target `cargo test` (which
// runs the pure list logic below) never calls a shim, but the linker still needs
// the symbol to resolve. Provide a no-op off-wasm so `cargo test` links; the wasm
// build excludes this and binds the real zellij import instead.
#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
extern "C" fn host_run_plugin_command() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn groves() -> Vec<GroveRow> {
        vec![
            GroveRow { name: "auth".into(), pending: 0 },
            GroveRow { name: "billing".into(), pending: 3 },
            GroveRow { name: "search".into(), pending: 0 },
        ]
    }

    fn tabs() -> Vec<TabEntry> {
        vec![
            TabEntry { name: "home".into(), position: 0, active: false },
            TabEntry { name: "billing".into(), position: 1, active: true },
        ]
    }

    #[test]
    fn parse_groves_reads_name_and_pending_columns() {
        let parsed = parse_groves("auth\t0\nbilling\t3");
        assert_eq!(
            parsed,
            vec![
                GroveRow { name: "auth".into(), pending: 0 },
                GroveRow { name: "billing".into(), pending: 3 },
            ]
        );
    }

    #[test]
    fn parse_groves_tolerates_blank_lines_and_missing_pending() {
        // Empty payload → no groves; a name with no pending column → 0; a blank
        // line is skipped; a non-numeric pending falls back to 0.
        assert_eq!(parse_groves(""), vec![]);
        assert_eq!(
            parse_groves("auth\n\nbilling\tx\n"),
            vec![
                GroveRow { name: "auth".into(), pending: 0 },
                GroveRow { name: "billing".into(), pending: 0 },
            ]
        );
    }

    #[test]
    fn resolve_activation_switches_open_groves_and_opens_closed_ones() {
        // "billing" has a tab → switch to its 0-based position.
        assert_eq!(
            resolve_activation(&groves()[1], &tabs()),
            Activation::Switch(1)
        );
        // "auth" has no tab → open it (intent to the controller).
        assert_eq!(
            resolve_activation(&groves()[0], &tabs()),
            Activation::Open("auth".into())
        );
    }

    #[test]
    fn grove_open_state_marks_active_open_and_closed() {
        assert_eq!(grove_open_state(&groves()[1], &tabs()), OpenState::Active); // billing tab is active
        // A grove that is open but whose tab is not active reads as Open.
        let open_not_active = vec![TabEntry { name: "auth".into(), position: 2, active: false }];
        assert_eq!(grove_open_state(&groves()[0], &open_not_active), OpenState::Open);
        // No tab at all → Closed.
        assert_eq!(grove_open_state(&groves()[0], &tabs()), OpenState::Closed);
    }

    #[test]
    fn grove_label_shows_a_pending_badge_only_when_nonzero() {
        assert_eq!(grove_label(&GroveRow { name: "auth".into(), pending: 0 }), "auth");
        assert_eq!(grove_label(&GroveRow { name: "billing".into(), pending: 3 }), "billing (3)");
    }

    #[test]
    fn active_grove_index_finds_the_grove_whose_tab_is_focused() {
        // billing's tab is active → its index in the grove list is 1.
        assert_eq!(active_grove_index(&groves(), &tabs()), Some(1));
        // Active tab is "home" (not a grove) → None.
        let home_active = vec![TabEntry { name: "home".into(), position: 0, active: true }];
        assert_eq!(active_grove_index(&groves(), &home_active), None);
        // No active tab → None.
        assert_eq!(active_grove_index(&groves(), &[]), None);
    }

    #[test]
    fn home_position_prefers_the_named_home_tab() {
        assert_eq!(home_position(&tabs()), Some(0));
        // Falls back to the first tab when there is no "home".
        let no_home = vec![
            TabEntry { name: "auth".into(), position: 3, active: true },
            TabEntry { name: "billing".into(), position: 4, active: false },
        ];
        assert_eq!(home_position(&no_home), Some(3));
        assert_eq!(home_position(&[]), None);
    }

    #[test]
    fn selection_wraps_in_both_directions() {
        // down from the last row wraps to the top...
        assert_eq!(next_index(2, 3), 0);
        assert_eq!(next_index(0, 3), 1);
        // ...up from the top wraps to the bottom.
        assert_eq!(prev_index(0, 3), 2);
        assert_eq!(prev_index(2, 3), 1);
        // empty list: no movement, no panic.
        assert_eq!(next_index(0, 0), 0);
        assert_eq!(prev_index(0, 0), 0);
    }

    #[test]
    fn truncate_respects_the_narrow_sidebar_width() {
        assert_eq!(truncate("auth", 10), "auth");
        assert_eq!(truncate("a-very-long-grove-name", 8), "a-very-l");
        assert_eq!(truncate("auth", 0), "");
    }

    #[test]
    fn clamp_keeps_selection_in_range_when_the_list_shrinks() {
        // A grove leaving the list shrinks it; a stale selection clamps to the
        // last row rather than pointing past the end.
        assert_eq!(clamp_index(5, 3), 2);
        assert_eq!(clamp_index(1, 3), 1);
        assert_eq!(clamp_index(0, 0), 0);
    }
}

//! grove-nav — grove's [[leader]]-focused command surface, realised as a zellij
//! WASM plugin (ADR-0018, leaf 070-nav-plugin).
//!
//! ## Why a plugin (the one API fact this rests on)
//!
//! grove runs zellij in `default_mode "locked"`, so every key passes through to
//! the focused app — only a zellij keybind can intercept a key while a harness is
//! focused. And a keybind **cannot** focus a tiled pane by id: the *only* action
//! that lands focus on a named surface is `LaunchOrFocusPlugin "<name>"`, and a
//! focused plugin then receives every keypress (`Event::Key`, no permission). So
//! the leader-reachable control surface *must* be a plugin. `Ctrl-o` is bound to
//! `LaunchOrFocusPlugin "grove-nav" { move_to_focused_tab true }` (see
//! `src/zellij.rs`); this crate is what it focuses.
//!
//! ## What it does (this leaf)
//!
//! Per ADR-0018 the plugin holds **no grove state** — it carries only
//! zellij-layout logic. For workspace switching it needs only zellij's own tab
//! list, which it gets from the `TabUpdate` event: each grove is a tab and the
//! dashboard is the "home" tab (leaf 060). So the nav renders the live tab list,
//! moves a selection over it, and switches via `switch_tab_to` — entirely from
//! zellij state, no controller round-trip. The controller→plugin **pipe** that
//! carries the *full* grove list (including groves not yet opened as a tab, to
//! first-open them) is leaf 080; until then the nav switches among already-open
//! workspaces and jumps home, and is the live mode/key discoverability surface
//! (subsuming the former 050-mode-discoverability concern).
//!
//! ## Returning focus
//!
//! Acting (`Enter`/`h`) switches the tab and then `close_self()`s, so the nav
//! behaves like a command palette: it vanishes and focus lands on the chosen
//! workspace. Cancelling (`q`/`Esc`) just `close_self()`s, returning focus to the
//! pane that was focused when the leader fired. locked mode is otherwise
//! untouched — keys reach the nav only because zellij focused it.

use std::collections::BTreeMap;

use zellij_tile::prelude::*;

/// One zellij tab as the nav sees it: its display name and 0-based position.
/// Built from `TabInfo`; the nav keeps nothing else (no grove state).
#[derive(Clone, Debug, PartialEq, Eq)]
struct TabEntry {
    name: String,
    /// 0-based tab position. `switch_tab_to` is 1-based, so switching uses
    /// `position + 1` (see [`activate`]).
    position: usize,
    active: bool,
}

#[derive(Default)]
struct State {
    /// The live workspace list, mirrored from `TabUpdate`. Ordered by position.
    tabs: Vec<TabEntry>,
    /// The highlighted row in [`State::tabs`].
    selected: usize,
    /// The current zellij input mode, shown in the hint footer (discoverability).
    mode: String,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // `switch_tab_to` needs ChangeApplicationState; reading tabs/mode needs
        // ReadApplicationState. Requested once on first load; zellij remembers the
        // grant across launches.
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
                let was_empty = self.tabs.is_empty();
                self.tabs = tabs
                    .iter()
                    .map(|t| TabEntry {
                        name: t.name.clone(),
                        position: t.position,
                        active: t.active,
                    })
                    .collect();
                // On first population, start the cursor on the active workspace so
                // the nav opens "where you are."
                if was_empty {
                    if let Some(i) = active_index(&self.tabs) {
                        self.selected = i;
                    }
                }
                self.selected = clamp_index(self.selected, self.tabs.len());
                true
            }
            Event::ModeUpdate(mode_info) => {
                self.mode = format!("{:?}", mode_info.mode).to_lowercase();
                true
            }
            // Re-focused (the leader fired again): re-anchor the cursor on the
            // active workspace.
            Event::Visible(true) => {
                if let Some(i) = active_index(&self.tabs) {
                    self.selected = i;
                }
                true
            }
            Event::Key(key) => self.handle_key(key),
            _ => false,
        }
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        println!("\u{1b}[1m grove · workspaces\u{1b}[0m");
        println!();
        for (i, tab) in self.tabs.iter().enumerate() {
            let dot = if tab.active { "●" } else { " " };
            let label = if tab.name.is_empty() {
                format!("tab {}", tab.position + 1)
            } else {
                tab.name.clone()
            };
            if i == self.selected {
                // Reverse-video the selected row.
                println!("\u{1b}[7m > {dot} {label}\u{1b}[0m");
            } else {
                println!("   {dot} {label}");
            }
        }
        println!();
        println!(
            "\u{1b}[2m j/k move · enter switch · h home · q close\u{1b}[0m"
        );
        println!("\u{1b}[2m mode: {}\u{1b}[0m", self.mode);
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
                self.selected = next_index(self.selected, self.tabs.len());
                true
            }
            BareKey::Char('k') | BareKey::Up => {
                self.selected = prev_index(self.selected, self.tabs.len());
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
                close_self();
                false
            }
            _ => false,
        }
    }

    /// Switch to the selected workspace, then dismiss the nav (focus lands on it).
    fn activate_selected(&mut self) {
        if let Some(tab) = self.tabs.get(self.selected) {
            activate(tab.position);
        }
        close_self();
    }

    /// Jump to the "home" (dashboard) workspace, then dismiss the nav.
    fn jump_home(&mut self) {
        if let Some(pos) = home_position(&self.tabs) {
            activate(pos);
        }
        close_self();
    }
}

/// Switch zellij to the tab at 0-based `position`. `switch_tab_to` is 1-based
/// (index 0 means "tab 1"), so a position maps to `position + 1`.
fn activate(position: usize) {
    switch_tab_to(position as u32 + 1);
}

// ---------------------------------------------------------------------------
// Pure list logic (unit-tested; no zellij-tile calls).

/// The index of the active tab, if any.
fn active_index(tabs: &[TabEntry]) -> Option<usize> {
    tabs.iter().position(|t| t.active)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn tabs() -> Vec<TabEntry> {
        vec![
            TabEntry { name: "home".into(), position: 0, active: false },
            TabEntry { name: "auth".into(), position: 1, active: true },
            TabEntry { name: "billing".into(), position: 2, active: false },
        ]
    }

    #[test]
    fn active_index_finds_the_focused_tab() {
        assert_eq!(active_index(&tabs()), Some(1));
        assert_eq!(active_index(&[]), None);
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
    fn clamp_keeps_selection_in_range_when_tabs_shrink() {
        // A workspace closing shrinks the list; a stale selection clamps to the
        // last row rather than pointing past the end.
        assert_eq!(clamp_index(5, 3), 2);
        assert_eq!(clamp_index(1, 3), 1);
        assert_eq!(clamp_index(0, 0), 0);
    }
}

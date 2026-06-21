//! TUI configuration — for now just the **leader key** (E4), configurable from
//! day one. Mirrors the fleet manifest pattern ([`crate::fleet`]): an optional
//! TOML file under XDG config, absent → defaults.
//!
//! The leader is grove's single control-seam key. While the harness pane is
//! focused, grove forwards *every* key to the pane except the leader; the leader
//! flips focus to grove's own surfaces (E4). The default is **`Alt-g`** (mnemonic
//! g = grove): `M-g` is largely unbound across readline / vim / claude, so it has
//! the lowest collision cost, and crossterm reports it cleanly as `Char('g')` +
//! `ALT`.
//!
//! ## Parsing
//!
//! [`parse_leader`] accepts a tmux/human-ish spec: modifiers joined to a key by
//! `-`, e.g. `Alt-g`, `M-g`, `Ctrl-o`, `C-o`, `Alt-Shift-x`. Modifier names are
//! case-insensitive (`Ctrl`/`Control`/`C`, `Alt`/`Meta`/`Option`/`M`,
//! `Shift`/`S`). The final segment is the key: a single printable char, or a
//! named key (`Up`, `Enter`, `Esc`, `Tab`, `Space`, `F3`, …).

use std::path::{Path, PathBuf};

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;

/// grove's control-seam key (E4). A code + the modifier set that must be held.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Leader {
    code: KeyCode,
    mods: KeyModifiers,
}

/// Only these modifier bits participate in leader matching; Super/Hyper/Meta and
/// other host bits are ignored (a known gap, see [`crate::tui::input`]).
const MATCHED_MODS: KeyModifiers = KeyModifiers::from_bits_truncate(
    KeyModifiers::CONTROL.bits() | KeyModifiers::ALT.bits() | KeyModifiers::SHIFT.bits(),
);

impl Leader {
    /// The default leader, `Alt-g`.
    pub fn alt_g() -> Self {
        Self {
            code: KeyCode::Char('g'),
            mods: KeyModifiers::ALT,
        }
    }

    /// Does this key event press the leader? Compares the code and the
    /// Ctrl/Alt/Shift bits, ignoring other host modifier bits.
    pub fn matches(&self, key: &KeyEvent) -> bool {
        key.code == self.code && (key.modifiers & MATCHED_MODS) == self.mods
    }

    /// A compact display label for the whichkey footer, e.g. `⌥g`, `⌃o`,
    /// `⌥⇧x`, `⌃Up`. Modifiers render as macOS glyphs (⌃ Ctrl, ⌥ Alt, ⇧ Shift)
    /// in the conventional Ctrl→Alt→Shift order; the key follows.
    pub fn label(&self) -> String {
        let mut s = String::new();
        if self.mods.contains(KeyModifiers::CONTROL) {
            s.push('⌃');
        }
        if self.mods.contains(KeyModifiers::ALT) {
            s.push('⌥');
        }
        if self.mods.contains(KeyModifiers::SHIFT) {
            s.push('⇧');
        }
        s.push_str(&key_label(self.code));
        s
    }
}

/// Display a key code for the leader label: a printable char as itself, `Space`
/// spelled out, function keys as `F1`…, named keys by name.
fn key_label(code: KeyCode) -> String {
    match code {
        KeyCode::Char(' ') => "Space".to_string(),
        KeyCode::Char(c) => c.to_string(),
        KeyCode::F(n) => format!("F{n}"),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        other => format!("{other:?}"),
    }
}

impl Default for Leader {
    fn default() -> Self {
        Self::alt_g()
    }
}

/// Parse a leader spec (`Alt-g`, `Ctrl-o`, `M-S-x`, …) into a [`Leader`].
/// Returns `Err` with a human message on an unknown modifier or empty key.
pub fn parse_leader(spec: &str) -> Result<Leader, String> {
    let spec = spec.trim();
    if spec.is_empty() {
        return Err("leader is empty".to_string());
    }
    let parts: Vec<&str> = spec.split('-').collect();
    let (key_name, modifier_names) = parts.split_last().expect("non-empty after the empty check");
    if key_name.is_empty() {
        return Err(format!("leader `{spec}` has no key after its modifiers"));
    }

    let mut mods = KeyModifiers::NONE;
    for name in modifier_names {
        match name.to_ascii_lowercase().as_str() {
            "c" | "ctrl" | "control" => mods |= KeyModifiers::CONTROL,
            "m" | "alt" | "meta" | "option" => mods |= KeyModifiers::ALT,
            "s" | "shift" => mods |= KeyModifiers::SHIFT,
            other => return Err(format!("leader `{spec}`: unknown modifier `{other}`")),
        }
    }

    let code = parse_key_name(key_name)
        .ok_or_else(|| format!("leader `{spec}`: unknown key `{key_name}`"))?;
    Ok(Leader { code, mods })
}

/// Map a key-name segment to a [`KeyCode`]: a single char, or a named key.
fn parse_key_name(name: &str) -> Option<KeyCode> {
    let mut chars = name.chars();
    if let (Some(c), None) = (chars.next(), chars.clone().next()) {
        // Exactly one char.
        return Some(KeyCode::Char(c));
    }
    // Function keys `F1`..`F12`.
    if let Some(n) = name
        .strip_prefix(['F', 'f'])
        .and_then(|d| d.parse::<u8>().ok())
    {
        return Some(KeyCode::F(n));
    }
    match name.to_ascii_lowercase().as_str() {
        "up" => Some(KeyCode::Up),
        "down" => Some(KeyCode::Down),
        "left" => Some(KeyCode::Left),
        "right" => Some(KeyCode::Right),
        "enter" | "return" => Some(KeyCode::Enter),
        "esc" | "escape" => Some(KeyCode::Esc),
        "tab" => Some(KeyCode::Tab),
        "space" => Some(KeyCode::Char(' ')),
        "home" => Some(KeyCode::Home),
        "end" => Some(KeyCode::End),
        _ => None,
    }
}

/// The TUI config file (`tui.toml`). All keys optional so an empty or absent file
/// yields defaults.
#[derive(Debug, Default, Deserialize)]
pub struct TuiConfig {
    /// Leader spec; `None` → the default [`Leader::alt_g`].
    #[serde(default)]
    pub leader: Option<String>,
}

impl TuiConfig {
    /// Load and parse the config at `path`. `Ok(None)` when the file is absent
    /// (the zero-config case); `Err` only on a read or parse failure.
    pub fn load(path: &Path) -> anyhow::Result<Option<TuiConfig>> {
        use anyhow::Context;
        if !path.exists() {
            return Ok(None);
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading tui config {}", path.display()))?;
        let config: TuiConfig = toml::from_str(&text)
            .with_context(|| format!("parsing tui config {}", path.display()))?;
        Ok(Some(config))
    }

    /// Resolve the configured leader, falling back to the default. A malformed
    /// `leader` string is surfaced as an error rather than silently defaulted.
    pub fn leader(&self) -> anyhow::Result<Leader> {
        match &self.leader {
            Some(spec) => parse_leader(spec).map_err(|e| anyhow::anyhow!(e)),
            None => Ok(Leader::default()),
        }
    }
}

/// XDG config path: `$XDG_CONFIG_HOME/grove/tui.toml`, falling back to
/// `~/.config/grove/tui.toml`. `None` when neither var is set.
pub fn default_config_path() -> Option<PathBuf> {
    config_path_from(
        std::env::var("XDG_CONFIG_HOME").ok(),
        std::env::var("HOME").ok(),
    )
}

/// Pure XDG-path policy, factored out for testing (mirrors
/// [`crate::fleet`]'s manifest path).
fn config_path_from(xdg: Option<String>, home: Option<String>) -> Option<PathBuf> {
    if let Some(xdg) = xdg.filter(|s| !s.is_empty()) {
        return Some(PathBuf::from(xdg).join("grove").join("tui.toml"));
    }
    home.filter(|s| !s.is_empty()).map(|home| {
        PathBuf::from(home)
            .join(".config")
            .join("grove")
            .join("tui.toml")
    })
}

/// Resolve the active leader from the environment: load the XDG config if
/// present, else default. Side-effecting wrapper around [`TuiConfig`].
pub fn resolve_leader() -> anyhow::Result<Leader> {
    match default_config_path() {
        Some(path) => match TuiConfig::load(&path)? {
            Some(config) => config.leader(),
            None => Ok(Leader::default()),
        },
        None => Ok(Leader::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_leader_is_alt_g() {
        let leader = Leader::default();
        assert!(leader.matches(&KeyEvent::new(KeyCode::Char('g'), KeyModifiers::ALT)));
    }

    #[test]
    fn label_renders_alt_g_with_the_option_glyph() {
        assert_eq!(Leader::alt_g().label(), "⌥g");
    }

    #[test]
    fn label_renders_ctrl_and_named_keys() {
        assert_eq!(parse_leader("Ctrl-o").unwrap().label(), "⌃o");
        assert_eq!(parse_leader("Ctrl-Up").unwrap().label(), "⌃Up");
    }

    #[test]
    fn label_orders_modifiers_ctrl_alt_shift() {
        assert_eq!(parse_leader("Alt-Shift-x").unwrap().label(), "⌥⇧x");
        assert_eq!(parse_leader("Shift-Ctrl-Alt-x").unwrap().label(), "⌃⌥⇧x");
    }

    #[test]
    fn leader_does_not_match_plain_g() {
        let leader = Leader::alt_g();
        assert!(!leader.matches(&KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)));
    }

    #[test]
    fn leader_match_ignores_unrelated_host_modifier_bits() {
        // A stray SUPER bit must not defeat an Alt-g match.
        let leader = Leader::alt_g();
        let key = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::ALT | KeyModifiers::SUPER);
        assert!(leader.matches(&key));
    }

    #[test]
    fn parses_alt_g_long_and_short_forms() {
        assert_eq!(parse_leader("Alt-g").unwrap(), Leader::alt_g());
        assert_eq!(parse_leader("M-g").unwrap(), Leader::alt_g());
    }

    #[test]
    fn parses_ctrl_o() {
        let leader = parse_leader("Ctrl-o").unwrap();
        assert!(leader.matches(&KeyEvent::new(KeyCode::Char('o'), KeyModifiers::CONTROL)));
    }

    #[test]
    fn parses_named_and_function_keys() {
        let up = parse_leader("Ctrl-Up").unwrap();
        assert!(up.matches(&KeyEvent::new(KeyCode::Up, KeyModifiers::CONTROL)));
        let f3 = parse_leader("F3").unwrap();
        assert!(f3.matches(&KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE)));
    }

    #[test]
    fn parses_multiple_modifiers() {
        let leader = parse_leader("Alt-Shift-x").unwrap();
        assert!(leader.matches(&KeyEvent::new(
            KeyCode::Char('x'),
            KeyModifiers::ALT | KeyModifiers::SHIFT
        )));
    }

    #[test]
    fn rejects_unknown_modifier() {
        assert!(parse_leader("Hyper-g").is_err());
    }

    #[test]
    fn rejects_empty_and_modifier_only_specs() {
        assert!(parse_leader("").is_err());
        assert!(parse_leader("Alt-").is_err());
    }

    #[test]
    fn config_leader_defaults_when_absent() {
        let config = TuiConfig::default();
        assert_eq!(config.leader().unwrap(), Leader::default());
    }

    #[test]
    fn config_leader_parses_configured_value() {
        let config = TuiConfig {
            leader: Some("Ctrl-o".to_string()),
        };
        let leader = config.leader().unwrap();
        assert!(leader.matches(&KeyEvent::new(KeyCode::Char('o'), KeyModifiers::CONTROL)));
    }

    #[test]
    fn config_path_prefers_xdg_over_home() {
        let path = config_path_from(Some("/xdg".to_string()), Some("/home/u".to_string()));
        assert_eq!(path, Some(PathBuf::from("/xdg/grove/tui.toml")));
        let path = config_path_from(None, Some("/home/u".to_string()));
        assert_eq!(path, Some(PathBuf::from("/home/u/.config/grove/tui.toml")));
        assert_eq!(config_path_from(None, None), None);
    }
}

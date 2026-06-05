//!
//! # This module contain everything you'll need to access local system paths
//! containing configuration and layouts

use crate::consts::{SYSTEM_DEFAULT_CONFIG_DIR, ZELLIJ_PROJ_DIR};

use std::{path::Path, path::PathBuf};

#[cfg(not(windows))]
use crate::home_unix as platform;
#[cfg(windows)]
use crate::home_windows as platform;

/// grove fork: the trellis TUI is **not** zellij and must never inherit the
/// user's zellij configuration, so this returns `None` unconditionally.
///
/// This is the single chokepoint every on-disk *user* config source funnels
/// through — the config file (`$ZELLIJ_CONFIG_DIR` / `$XDG_CONFIG_HOME/zellij` /
/// `~/.config/zellij/config.kdl`, via `Config::try_from`), the user theme dir and
/// layout dir (via `Setup::from_cli_args`), and the `~/.config/zellij` mkdir in
/// `create_config_and_cache_folders`. Returning `None` severs all of them at once:
/// grove's only config is trellis's built-in defaults plus its in-process
/// `GROVE_TUI_CONFIG` merge (see `src/trellis_host.rs`). A user with a populated
/// `~/.config/zellij` therefore sees identical grove behaviour to one with none,
/// and grove never creates that directory.
///
/// Upstream walked `default_config_dirs()` here and returned the first existing
/// match; the fork owes upstream no compatibility (grove project principle /
/// ADR-0020). The previous implementation already returned `None` under
/// `#[cfg(test)]` — this just makes the test behaviour universal.
pub fn find_default_config_dir() -> Option<PathBuf> {
    None
}

/// Order in which config directories are checked
pub(crate) fn default_config_dirs() -> Vec<Option<PathBuf>> {
    vec![
        home_config_dir(),
        Some(xdg_config_dir()),
        Some(Path::new(SYSTEM_DEFAULT_CONFIG_DIR).to_path_buf()),
    ]
}

/// Looks for an existing dir, uses that, else returns a
/// dir matching the config spec.
pub fn get_default_data_dir() -> PathBuf {
    [xdg_data_dir(), platform::system_data_dir()]
        .into_iter()
        .find(|p| p.exists())
        .unwrap_or_else(xdg_data_dir)
}

pub fn xdg_config_dir() -> PathBuf {
    ZELLIJ_PROJ_DIR.config_dir().to_owned()
}

pub fn xdg_data_dir() -> PathBuf {
    ZELLIJ_PROJ_DIR.data_dir().to_owned()
}

pub fn home_config_dir() -> Option<PathBuf> {
    platform::home_config_dir()
}

pub fn try_create_home_config_dir() {
    platform::try_create_home_config_dir()
}

pub fn system_data_dir() -> PathBuf {
    platform::system_data_dir()
}

pub fn get_layout_dir(config_dir: Option<PathBuf>) -> Option<PathBuf> {
    config_dir.map(|dir| dir.join("layouts"))
}

pub fn default_layout_dir() -> Option<PathBuf> {
    find_default_config_dir().map(|dir| dir.join("layouts"))
}

pub fn get_theme_dir(config_dir: Option<PathBuf>) -> Option<PathBuf> {
    config_dir.map(|dir| dir.join("themes"))
}

pub fn default_theme_dir() -> Option<PathBuf> {
    find_default_config_dir().map(|dir| dir.join("themes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// grove fork contract: the user's zellij config is never sourced. The config
    /// file, user theme dir, user layout dir, and the `~/.config/zellij` mkdir all
    /// funnel through `find_default_config_dir`; pinning it to `None` is what makes
    /// a populated `~/.config/zellij` invisible to grove. If a future edit
    /// re-introduces the upstream `default_config_dirs()` search, this fails.
    #[test]
    fn user_zellij_config_dir_is_never_sourced() {
        assert_eq!(find_default_config_dir(), None);
        // The derived user theme/layout dirs collapse to None with it.
        assert_eq!(default_theme_dir(), None);
        assert_eq!(default_layout_dir(), None);
    }
}

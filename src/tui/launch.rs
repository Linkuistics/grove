//! **Launch wiring** (060/010): point both the SDK daemon-spawn and the ADR-0029
//! `capture-pane` shell-out at the rmux binary grove ships beside its own
//! executable, with zero user config. Implements ADR-0030 §3.
//!
//! ## The seam is one binary, one env var (ADR-0030)
//!
//! Both `Rmux::connect_or_start` (daemon-spawn) and [`crate::tui::editor`]
//! (`capture-pane`) resolve one binary the same way: `$RMUX_SDK_DAEMON_BINARY`
//! when set, else bare `rmux` on `PATH`. Setting that one var once at startup
//! redirects both. The bundled `rmux` (060/020) is installed into the same `bin/`
//! as `grove`, so it is `current_exe().parent()/rmux`.
//!
//! ## Precedence (ADR-0030 §3)
//!
//! **user override (`RMUX_SDK_DAEMON_BINARY` already set) → sibling-of-exe (only
//! if the file exists) → leave unset so the SDK falls back to bare `rmux` on
//! `PATH`** (which still catches dev builds + `cargo install grove`).
//!
//! ## Testability split
//!
//! The decisions are pure functions ([`daemon_binary_to_set`],
//! [`binary_resolves`]) — given an "already set?" flag / env path, a candidate
//! directory set, and an existence predicate, they pick the path to set (or
//! none) and report whether a binary resolves at all. The effectful wrappers
//! ([`redirect_daemon_binary_to_sibling`], [`daemon_binary_resolves`]) read the
//! real env, call `current_exe()`, and `set_var` — thin shells over the pure
//! core. The `set_var` MUST run in the synchronous `grove tui` entry, before the
//! tokio runtime + reader thread spawn (Rust 2024 makes it unsound once threads
//! exist) — see [`crate::tui::run`].

use std::path::{Path, PathBuf};

use rmux_sdk::bootstrap::discovery::SDK_DAEMON_BINARY_ENV;

/// The bundled binary's file name — a sibling of grove's own executable.
const DAEMON_BINARY_NAME: &str = "rmux";

/// Friendly first-run hint when no `rmux` resolves anywhere (env var → bundled
/// sibling → `PATH`). Surfaced in place of the raw transport error.
pub const DAEMON_NOT_FOUND_HINT: &str =
    "rmux not found — it ships with grove; reinstall, or set RMUX_SDK_DAEMON_BINARY";

/// Pure: decide the path to set `RMUX_SDK_DAEMON_BINARY` to, or `None` to leave
/// it untouched. Precedence (ADR-0030 §3): a user override (`already_set`) wins —
/// leave it (`None`); else the sibling-of-exe `rmux` when it exists; else `None`
/// so the SDK falls back to bare `rmux` on `PATH`.
fn daemon_binary_to_set(
    already_set: bool,
    exe_dir: Option<&Path>,
    exists: impl Fn(&Path) -> bool,
) -> Option<PathBuf> {
    if already_set {
        return None;
    }
    let candidate = exe_dir?.join(DAEMON_BINARY_NAME);
    exists(&candidate).then_some(candidate)
}

/// Pure: does an `rmux` binary resolve at all? If the env var is set, resolution
/// is whether *that* path exists; otherwise whether any `PATH` directory holds an
/// `rmux`. Used only to choose between the friendly "rmux not found" hint and the
/// generic connect error.
fn binary_resolves(
    env_path: Option<&Path>,
    path_dirs: &[PathBuf],
    exists: impl Fn(&Path) -> bool,
) -> bool {
    if let Some(p) = env_path {
        return exists(p);
    }
    path_dirs.iter().any(|d| exists(&d.join(DAEMON_BINARY_NAME)))
}

/// Resolve the bundled daemon as a sibling of `current_exe()` and set
/// `RMUX_SDK_DAEMON_BINARY` to it when the file exists and the var is unset; a
/// user override is respected verbatim. One set covers both the SDK daemon-spawn
/// and the ADR-0029 `capture-pane` shell-out.
///
/// MUST be called from the synchronous `grove tui` entry, before the tokio
/// runtime + input-reader thread spawn — `std::env::set_var` is unsound under
/// Rust 2024 once other threads exist.
pub fn redirect_daemon_binary_to_sibling() {
    let already_set = std::env::var_os(SDK_DAEMON_BINARY_ENV).is_some();
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf));
    if let Some(path) = daemon_binary_to_set(already_set, exe_dir.as_deref(), |p| p.exists()) {
        // SAFETY: ADR-0030 §3 — called from the synchronous `grove tui` entry,
        // before the tokio runtime / input-reader thread spawn, so the process is
        // still single-threaded and `set_var` is sound under Rust 2024.
        unsafe { std::env::set_var(SDK_DAEMON_BINARY_ENV, path) };
    }
}

/// Whether an `rmux` binary resolves anywhere: the `RMUX_SDK_DAEMON_BINARY` path
/// (if set) exists, or some `PATH` directory holds an `rmux`. Drives the
/// first-run error message, not control flow (an already-running daemon may be
/// reachable over its socket without a local binary).
pub fn daemon_binary_resolves() -> bool {
    let env_path = std::env::var_os(SDK_DAEMON_BINARY_ENV).map(PathBuf::from);
    let path_dirs: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    binary_resolves(env_path.as_deref(), &path_dirs, |p| p.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- daemon_binary_to_set: precedence (ADR-0030 §3) ----------------------

    #[test]
    fn user_override_wins_and_leaves_the_var_untouched() {
        // already_set → None even if a sibling rmux exists.
        let chosen = daemon_binary_to_set(true, Some(Path::new("/opt/grove/bin")), |_| true);
        assert_eq!(chosen, None);
    }

    #[test]
    fn picks_the_sibling_rmux_when_unset_and_present() {
        let chosen = daemon_binary_to_set(false, Some(Path::new("/opt/grove/bin")), |p| {
            p == Path::new("/opt/grove/bin/rmux")
        });
        assert_eq!(chosen, Some(PathBuf::from("/opt/grove/bin/rmux")));
    }

    #[test]
    fn leaves_unset_when_no_sibling_rmux_exists() {
        // No sibling → None → SDK falls back to bare `rmux` on PATH.
        let chosen = daemon_binary_to_set(false, Some(Path::new("/opt/grove/bin")), |_| false);
        assert_eq!(chosen, None);
    }

    #[test]
    fn leaves_unset_when_exe_dir_is_unknown() {
        let chosen = daemon_binary_to_set(false, None, |_| true);
        assert_eq!(chosen, None);
    }

    // --- binary_resolves: friendly-error gate --------------------------------

    #[test]
    fn resolves_via_existing_env_path() {
        assert!(binary_resolves(Some(Path::new("/custom/rmux")), &[], |p| p
            == Path::new("/custom/rmux")));
    }

    #[test]
    fn does_not_resolve_when_env_path_is_missing() {
        // An override pointing at a nonexistent file does not resolve — PATH is
        // NOT consulted as a fallback once the var is set (matches the SDK).
        let path_dirs = vec![PathBuf::from("/usr/bin")];
        assert!(!binary_resolves(
            Some(Path::new("/custom/rmux")),
            &path_dirs,
            |p| p == Path::new("/usr/bin/rmux"),
        ));
    }

    #[test]
    fn resolves_via_path_when_env_unset() {
        let path_dirs = vec![PathBuf::from("/sbin"), PathBuf::from("/usr/local/bin")];
        assert!(binary_resolves(None, &path_dirs, |p| p
            == Path::new("/usr/local/bin/rmux")));
    }

    #[test]
    fn does_not_resolve_when_unset_and_absent_from_path() {
        let path_dirs = vec![PathBuf::from("/sbin"), PathBuf::from("/usr/bin")];
        assert!(!binary_resolves(None, &path_dirs, |_| false));
    }
}

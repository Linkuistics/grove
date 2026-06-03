//! The grove zellij **session-name** helper.
//!
//! Historically (`060-harness-pane/040-grove-integration/030-zellij-launch`)
//! this module was grove's *head binary* — it wrote a tamed zellij config +
//! bars-free layout to a cache dir, spawned an installed zellij as a child, and
//! drove a dumb dashboard proxy over a unix socket (ADR-0015/0016). The deep
//! fork (ADR-0020) and the library-you-link hosting model (ADR-0021) made all of
//! that machinery obsolete: `grove tui` now owns `main`, links the forked
//! `zellij-*` crates, and renders the dashboard natively in-process (see
//! [`crate::trellis_host`] and [`crate::tui`]'s native surface). The launch /
//! config-template / proxy / nav-embedding code retired with leaf
//! `110-native-host-api/030-port-dashboard-drive`.
//!
//! What survives is the one pure helper still needed: deriving a stable zellij
//! **session name** from a repo path, used by the trellis client when it starts
//! the session.

use std::path::Path;

/// The trellis/zellij session name for `repo`: `grove-<sanitised-basename>`, so
/// two repos' dashboards don't collide on one session. Non-`[A-Za-z0-9_-]` chars
/// become `-`; a basename that sanitises to nothing falls back to plain `grove`.
///
/// Consumed by the trellis client ([`crate::trellis_host`]), which only exists
/// under the `trellis-seam` feature — so a default build sees it as unused; the
/// test below keeps it exercised in every build.
#[cfg_attr(not(feature = "trellis-seam"), allow(dead_code))]
pub(crate) fn session_name(repo: &Path) -> String {
    let base = repo
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let cleaned: String = base
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect();
    let cleaned = cleaned.trim_matches('-');
    if cleaned.is_empty() {
        "grove".to_string()
    } else {
        format!("grove-{}", cleaned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn session_name_sanitises_and_namespaces() {
        assert_eq!(session_name(Path::new("/work/grove")), "grove-grove");
        assert_eq!(session_name(Path::new("/work/my repo.git")), "grove-my-repo-git");
        // A basename that sanitises to nothing falls back to plain `grove`.
        assert_eq!(session_name(Path::new("/work/...")), "grove");
    }
}

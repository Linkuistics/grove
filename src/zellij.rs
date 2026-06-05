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
//! What survives is the one pure helper still needed: the grove fleet TUI's
//! **session name**, used by the trellis client when it starts the session.

/// The trellis/zellij session name for grove's fleet TUI: the constant
/// `grove-fleet` (ADR-0027 §4). With no cwd repo anchor the TUI is a **singleton
/// session** — a second `grove tui` re-attaches `grove-fleet` rather than
/// spawning a per-launch-directory session, which matches a multi-repo tool's
/// mental model. (Superseding the old `grove-<repo-basename>` derivation, which
/// fragmented the fleet into one session per launch directory.)
///
/// Consumed by the trellis client ([`crate::trellis_host`]), the one, always-on
/// TUI (ADR-0026).
pub(crate) fn session_name() -> String {
    "grove-fleet".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_name_is_the_fleet_singleton() {
        // One constant session, so every `grove tui` re-attaches the same fleet.
        assert_eq!(session_name(), "grove-fleet");
    }
}

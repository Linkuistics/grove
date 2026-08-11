//! The known-harness registry — **provisioning targets only**.
//!
//! Grove launches whatever `~/.config/grove/config.kdl` names, as an opaque
//! command, so nothing here selects, detects, or parameterises a launch. What
//! survives is the delivery question the configured command cannot answer:
//! *which personal skill directories should the embedded methodology be swept
//! into before the driver owns the working tree?* Each row is one such target,
//! and the registry is deliberately independent of launch policy — a row is a
//! place to write files, never a thing to run.

/// One provisioning target: a harness Grove knows how to deliver its
/// methodology to.
///
/// The registry is a slice rather than a lookup table because provisioning
/// *iterates* it — every installed harness is swept, none is selected. A
/// `by_name` lookup lived here until `dead-non-launch-exports-k166`; it was the
/// last shape of the removed name→launch-target inference, and after that
/// removal nothing but its own tests called it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Harness {
    pub name: &'static str,
    /// Home-relative marker directory whose existence means "this harness is
    /// installed for this user". Presence gates provisioning; an absent root is
    /// skipped, never created.
    pub project_dir: &'static str,
    /// Home-relative path of this harness's **global skills dir** — the
    /// provisioning destination for the embedded methodology. A field (not
    /// derived from `project_dir`) because pi nests its skills under `agent/`.
    pub skills_dir: &'static str,
}

pub const HARNESSES: &[Harness] = &[
    Harness {
        name: "claude",
        project_dir: ".claude",
        skills_dir: ".claude/skills",
    },
    Harness {
        name: "codex",
        project_dir: ".codex",
        skills_dir: ".codex/skills",
    },
    Harness {
        name: "pi",
        project_dir: ".pi",
        skills_dir: ".pi/agent/skills",
    },
];

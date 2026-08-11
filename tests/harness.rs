//! The known-harness registry, which is now a **provisioning target directory
//! registry and nothing else**.
//!
//! Every launch-policy row this file used to assert — `exec_bin`, `name_args`,
//! `model_args` — is gone with harness inference: the configured command
//! template is the whole of launch policy, so there is no binary to resolve and
//! no flag to template. What is left is a delivery question the opaque
//! configured command cannot answer, and the two properties below are what keep
//! the two concerns apart.

use grove::harness::{Harness, HARNESSES};

/// The lookup the tests do for themselves. `harness::by_name` used to export
/// this one line, and nothing but these tests ever called it: provisioning
/// *iterates* `HARNESSES` rather than selecting a row, so a name lookup was
/// residue of the removed name→launch-target inference. Asserting against the
/// slice is asserting against what production actually reads.
fn row(name: &str) -> Option<&'static Harness> {
    HARNESSES.iter().find(|harness| harness.name == name)
}

#[test]
fn registry_contains_claude_codex_and_pi() {
    assert!(row("claude").is_some());
    assert!(row("codex").is_some());
    assert!(row("pi").is_some());
    assert!(row("nonsense").is_none());
}

// The rows carry a *destination*, not a *command*. pi is the row that proves
// `skills_dir` has to be its own field rather than a transform of
// `project_dir`: it nests skills under `agent/`, so any derivation would send
// pi's methodology to a directory pi does not read.
#[test]
fn every_row_names_an_install_marker_and_a_skills_destination() {
    let claude = row("claude").unwrap();
    assert_eq!(claude.project_dir, ".claude");
    assert_eq!(claude.skills_dir, ".claude/skills");

    let codex = row("codex").unwrap();
    assert_eq!(codex.project_dir, ".codex");
    assert_eq!(codex.skills_dir, ".codex/skills");

    let pi = row("pi").unwrap();
    assert_eq!(pi.project_dir, ".pi");
    assert_eq!(
        pi.skills_dir, ".pi/agent/skills",
        "pi nests skills under agent/, which is why skills_dir is a field"
    );
}

// The structural half of the same claim, and the one that survives a row being
// added. A `Harness` has exactly three fields, all of them paths-or-names with
// no runtime behaviour attached — so a future row cannot smuggle launch policy
// back in without changing the type, which is a visible, reviewable edit.
#[test]
fn a_registry_row_is_a_destination_record_with_no_launch_policy() {
    assert!(!HARNESSES.is_empty());
    for harness in HARNESSES {
        let Harness {
            name,
            project_dir,
            skills_dir,
        } = *harness;
        assert!(!name.is_empty());
        assert!(
            project_dir.starts_with('.') && !project_dir.contains('/'),
            "{name}: the install marker is one home-relative dotdir, got {project_dir:?}"
        );
        assert!(
            skills_dir.starts_with(project_dir),
            "{name}: the skills destination must live under the install marker, got {skills_dir:?}"
        );
        assert_eq!(
            row(name),
            Some(harness),
            "{name}: every row must be findable by its own name"
        );
    }
}

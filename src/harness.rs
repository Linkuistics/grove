use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Harness {
    pub name: &'static str,
    pub project_dir: &'static str,
    /// Binary name to exec (resolved via PATH at runtime).
    pub exec_bin: &'static str,
    /// CLI arg used by the harness to set the session name.
    /// For claude: ["-n", "<name>"]. For codex: see codex docs; default
    /// matches claude's pattern until verified.
    pub name_args: &'static [&'static str],
}

pub const HARNESSES: &[Harness] = &[
    Harness {
        name: "claude",
        project_dir: ".claude",
        exec_bin: "claude",
        name_args: &["-n"],
    },
    Harness {
        name: "codex",
        project_dir: ".codex",
        exec_bin: "codex",
        // Verified during implementation against `codex --help`; if codex
        // doesn't support a session-name flag, leave empty and skip pre-naming.
        name_args: &["--name"],
    },
];

pub fn by_name(name: &str) -> Option<&'static Harness> {
    HARNESSES.iter().find(|h| h.name == name)
}

impl Harness {
    pub fn install_path(&self, repo: &Path) -> PathBuf {
        repo.join(self.project_dir).join("skills").join("grove")
    }

    pub fn project_dir_path(&self, repo: &Path) -> PathBuf {
        repo.join(self.project_dir)
    }
}

/// Return the harnesses that have a project directory in `repo`.
pub fn detect_in_repo(repo: &Path) -> Vec<&'static Harness> {
    HARNESSES
        .iter()
        .filter(|h| h.project_dir_path(repo).is_dir())
        .collect()
}

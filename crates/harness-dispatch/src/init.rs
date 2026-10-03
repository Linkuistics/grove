//! `init`: install the sample policy as the owner's own
//! (`docs/specs/harness-selection-and-execution.md`, *The sample policy and the
//! choice file*).
//!
//! The front carries the sample's text, so installing it needs no worker. It
//! is written to the personal default path and nowhere else, only when nothing
//! is there, and nothing but this command ever writes a policy.

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write as _};
use std::path::PathBuf;

use crate::authority;
use crate::refusal::{Refusal, Stage, EXIT_REFUSED};

/// The sample policy, as it also ships readable beside the worker.
const SAMPLE: &str = include_str!("../worker/sample/policy.ts");

/// Where the sample was installed.
pub struct Installed(PathBuf);

pub fn init() -> Result<Installed, Refusal> {
    let home = std::env::var_os("HOME");
    let path = authority::personal_default(home.as_deref())?;
    let shown = path.to_string_lossy().into_owned();
    let unwritable = |error: std::io::Error| {
        Refusal::new(
            "policy_unwritable",
            Stage::Authority,
            EXIT_REFUSED,
            format!("the sample policy cannot be written to {shown}: {error}"),
            "make its directory writable and run `harness-dispatch init` again",
        )
        .source(shown.clone())
    };
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory).map_err(unwritable)?;
    }
    // Exclusive creation refuses whatever is at the path, a directory or a
    // dangling link included, in the one step that creates the file.
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|error| {
            if error.kind() == ErrorKind::AlreadyExists {
                Refusal::new(
                    "policy_exists",
                    Stage::Authority,
                    EXIT_REFUSED,
                    format!("{shown} already exists, and init replaces nothing"),
                    "keep it, or move it away and run `harness-dispatch init` again",
                )
                .source(shown.clone())
            } else {
                unwritable(error)
            }
        })?;
    if let Err(error) = file.write_all(SAMPLE.as_bytes()) {
        // A partial policy would refuse every later init.
        let _ = fs::remove_file(&path);
        return Err(unwritable(error));
    }
    Ok(Installed(path))
}

impl Installed {
    pub fn to_text(&self) -> String {
        format!(
            "Installed the sample policy as {}.\n\
             It launches codex with approvals off and full access (--ask-for-approval never, \
             default_permissions=:danger-full-access). Read it, and edit it, before the first \
             launch: it is yours.\n\
             See what it selects, from the directory Grove runs in, with\n  \
             harness-dispatch inspect --kind impl\n",
            self.0.display()
        )
    }
}

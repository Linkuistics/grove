//! `exit`: the exit signal (`docs/specs/harness-selection-and-execution.md`,
//! *Supervision*, *The exit signal*).
//!
//! A supervised run publishes a fresh exit channel to its harness as
//! [`EXIT_FILE_VARIABLE`]. Creating that file is the whole signal: it carries
//! nothing, and the run that published it reads no content. So this verb takes
//! no input and reads no owner setting, policy or record, which is what lets it
//! run inside a sandbox that can reach none of them. A nested run publishes its
//! own channel to its own harness, so the variable a process holds names the
//! one run it is under.

use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::process::ExitCode;

use crate::run::EXIT_FILE_VARIABLE;

/// Create the channel, or say there is none. One that already exists is
/// success, since its appearance is already the signal.
///
/// The file is created exclusively, which never follows a link at its name: a
/// name already taken, by anything, has already appeared.
pub fn exit() -> ExitCode {
    let Some(path) = std::env::var_os(EXIT_FILE_VARIABLE).filter(|path| !path.is_empty()) else {
        eprintln!(
            "harness-dispatch exit: not running under a supervised run ({EXIT_FILE_VARIABLE} is \
             unset or empty), so nothing was signalled; a harness that harness-dispatch run \
             started has it set"
        );
        return ExitCode::SUCCESS;
    };
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) if error.kind() == ErrorKind::AlreadyExists => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!(
                "harness-dispatch exit: cannot create the exit channel {}: {error}; the run that \
                 published it allocated it in a directory that should exist and be writable \
                 here — check that the directory was not removed, and that a sandbox lets this \
                 process write there",
                std::path::Path::new(&path).display()
            );
            ExitCode::FAILURE
        }
    }
}

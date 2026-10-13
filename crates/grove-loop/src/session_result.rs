//! Caller-owned stdout transport for Ravel's Grove continue result.

use std::fs::File;
use std::io::{Read, Seek};
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use anyhow::Context;

use crate::Error;

/// The opaque result Grove accepts for ordinary continuation.
pub const CONTINUE_RESULT: &str = "grove-continue";

/// A held, unlinked regular file for Ravel's public stdout result.
///
/// Pass [`Self::stdout`] to keyed-launch's stdout-capturing entry point. Call
/// [`Self::read_continue`] only after the supervisor has been reaped and all
/// writers have closed. Lifecycle callers must also invalidate the session
/// epoch first. Teardown precedence, cancellation, exit status and survivor
/// checks belong to the caller; accepted bytes alone attest none of them.
///
/// The result has no pathname for a managed process to substitute. Policy and
/// Ravel still have the stdout authority deliberately given to the supervisor.
pub struct SessionResult(File);

impl SessionResult {
    /// Allocate a fresh result capture.
    ///
    /// # Errors
    /// Failure to allocate or verify an unlinked regular file. No launch should
    /// start if capture allocation fails.
    pub fn allocate() -> Result<Self, Error> {
        // tempfile 3.27's Unix fallback ignores unlink failures; verify the held
        // file before granting stdout rather than assuming it has no pathname.
        // https://github.com/Stebalien/tempfile/blob/v3.27.0/src/file/imp/unix.rs
        let file = tempfile::tempfile().context("allocating session result capture")?;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))
            .context("restricting session result capture permissions")?;
        let metadata = file.metadata().context("checking session result capture")?;
        if !metadata.is_file() || metadata.nlink() != 0 {
            return Err(Error::msg(
                "session result capture must be an unlinked regular file; no session was launched",
            ));
        }
        Ok(Self(file))
    }

    /// An owned descriptor to pass to the stdout-capturing runner.
    ///
    /// It shares this capture's file offset. The file starts empty; the reader
    /// rewinds after supervision. This descriptor must carry only Ravel stdout,
    /// never managed output, stderr or a transcript.
    ///
    /// # Errors
    /// Failure to duplicate the file descriptor.
    pub fn stdout(&self) -> Result<File, Error> {
        Ok(self
            .0
            .try_clone()
            .context("duplicating session result stdout")?)
    }

    /// Consume the capture and accept exactly `grove-continue`, without trimming.
    ///
    /// Reads at most the literal's length plus one byte; a matching prefix with
    /// any tail refuses. Never reopens a path or parses a private ending report.
    ///
    /// # Errors
    /// Missing, unsupported or unreadable results.
    pub fn read_continue(mut self) -> Result<(), Error> {
        self.0
            .rewind()
            .context("rewinding session result capture")?;
        let mut bytes = Vec::with_capacity(CONTINUE_RESULT.len() + 1);
        self.0
            .take((CONTINUE_RESULT.len() + 1) as u64)
            .read_to_end(&mut bytes)
            .context("reading session result capture")?;
        // Ravel's public CLI returns the exact opaque argument without a newline.
        // https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/session-cli.md
        if bytes.is_empty() {
            return Err(Error::msg(format!(
                "Ravel returned no session result; complete with `ravel end-session {CONTINUE_RESULT}`"
            )));
        }
        if bytes != CONTINUE_RESULT.as_bytes() {
            return Err(Error::msg(format!(
                "Ravel returned an unsupported session result; expected exactly {CONTINUE_RESULT}"
            )));
        }
        Ok(())
    }
}

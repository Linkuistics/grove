//! Caller-owned stdout transport for Ravel's opaque Grove results.

use std::fs::File;
use std::io::{Read, Seek};
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use anyhow::Context;

use crate::Error;

/// The opaque result Grove accepts for ordinary continuation.
pub const CONTINUE_RESULT: &str = "grove-continue";

/// The opaque result Grove accepts for an explicit stop without completion.
pub const FORCE_STOP_RESULT: &str = "grove-force-stop";

/// Grove's interpretation of a supported opaque result; Ravel assigns none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionDisposition {
    Continue,
    ForceStop,
}

/// A held, unlinked regular file for Ravel's public stdout result.
///
/// Pass [`Self::stdout`] to keyed-launch's stdout-capturing entry point. Call
/// [`Self::read`] only after the supervisor has been reaped and all
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

    /// Consume the capture and accept either exact Grove result, without trimming.
    ///
    /// Reads at most the longest literal's length plus one byte; a prefix with
    /// any tail refuses. Never reopens a path or parses a private ending report.
    ///
    /// # Errors
    /// Missing, unsupported or unreadable results.
    pub fn read(mut self) -> Result<SessionDisposition, Error> {
        self.0
            .rewind()
            .context("rewinding session result capture")?;
        let bound = CONTINUE_RESULT.len().max(FORCE_STOP_RESULT.len()) + 1;
        let mut bytes = Vec::with_capacity(bound);
        self.0
            .take(bound as u64)
            .read_to_end(&mut bytes)
            .context("reading session result capture")?;
        // Ravel's public CLI returns the exact opaque argument without a newline.
        // https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md
        if bytes.is_empty() {
            return Err(Error::msg(format!(
                "Ravel returned no session result; complete with `ravel end-session {CONTINUE_RESULT}`"
            )));
        }
        match bytes.as_slice() {
            bytes if bytes == CONTINUE_RESULT.as_bytes() => Ok(SessionDisposition::Continue),
            bytes if bytes == FORCE_STOP_RESULT.as_bytes() => Ok(SessionDisposition::ForceStop),
            _ => Err(Error::msg(format!(
                "Ravel returned an unsupported session result; expected exactly {CONTINUE_RESULT} or {FORCE_STOP_RESULT}"
            ))),
        }
    }
}

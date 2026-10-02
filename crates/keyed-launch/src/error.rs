use std::fmt;

/// Everything that can go wrong allocating a channel, spawning a child, or
/// supervising one.
///
/// **Opaque**: the message is the interface, and the type implements
/// `std::error::Error` without imposing an error-handling dependency on its
/// consumers.
///
/// Its obligation is to name what is wrong, name where, and name what would
/// fix it.
pub struct LaunchError {
    message: String,
}

impl LaunchError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump: a `{:?}` of this error is read by a human in
/// a panic or an `anyhow` chain, and a record dump would obscure the human report.
impl fmt::Debug for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LaunchError {}

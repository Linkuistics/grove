use std::fmt;

/// Everything that can go wrong reading, validating or expanding a template
/// configuration.
///
/// **Opaque, and that is the design rather than an omission.** A variant list
/// would be a second interface — every consumer matching on it, every new
/// diagnostic a breaking change — for a value whose only use is being shown to
/// the person who has to fix the file. The obligation this type carries instead
/// is a property of every message it holds: name what is wrong, name *where*
/// (path, line and column, wherever a document has one), and name what fixes it.
///
/// It implements [`std::error::Error`], so a consumer using `anyhow`,
/// `thiserror` or nothing at all takes on no dependency of this crate's here.
pub struct ConfigError {
    message: String,
}

impl ConfigError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump: a `{:?}` of this error is read by a human in
/// a panic or an `anyhow` chain, and the braces would be noise around the one
/// field.
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ConfigError {}

/// Everything that can go wrong allocating a channel, spawning a child, or
/// supervising one.
///
/// **Opaque for the same reason [`ConfigError`] is**, and deliberately a
/// *separate* type rather than a shared one: a caller that only loads and
/// expands a configuration never handles a spawn failure, and one that only
/// launches never handles a KDL parse error. Two types keep the two halves of
/// this crate usable apart — which is the whole claim `Templates` and `run`
/// make by not referring to each other.
///
/// Its obligation is the same: name what is wrong, name where, and name what
/// would fix it.
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

/// The message, not a struct dump — see [`ConfigError`]'s `Debug` for why.
impl fmt::Debug for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LaunchError {}

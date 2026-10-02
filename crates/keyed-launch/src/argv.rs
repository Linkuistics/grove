use std::ffi::{OsStr, OsString};

/// A value for one declared slot, at expansion.
///
/// Substitution is whole-word: the crate never learns what a name means, and
/// never rewrites part of a word.
pub struct Slot<'a> {
    pub name: &'a str,
    pub value: &'a OsStr,
}

/// A program and its arguments, in order, ready to spawn.
///
/// There is no shell and no second reading of any word: each is spawned as it
/// is given. [`Templates::expand`](crate::Templates::expand) authors one from a
/// template, with each whole-word slot replaced by the value offered for it. A
/// caller that already holds a command builds one with [`Argv::new`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Argv {
    program: OsString,
    args: Vec<OsString>,
}

impl Argv {
    /// A program and arguments the caller built, each string one whole word.
    #[must_use]
    pub fn new(program: OsString, args: Vec<OsString>) -> Self {
        Self { program, args }
    }

    #[must_use]
    pub fn program(&self) -> &OsStr {
        &self.program
    }

    #[must_use]
    pub fn args(&self) -> &[OsString] {
        &self.args
    }

    /// The whole launch as one word list, program first — the shape a
    /// `Command`-building consumer and a diagnostic both want.
    #[must_use]
    pub fn words(&self) -> Vec<OsString> {
        let mut words = Vec::with_capacity(self.args.len() + 1);
        words.push(self.program.clone());
        words.extend(self.args.iter().cloned());
        words
    }
}

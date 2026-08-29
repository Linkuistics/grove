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
/// Built only by [`Templates::expand`](crate::Templates::expand), so nothing
/// reaches a spawn that a template did not author. There is no constructor and
/// no shell: the words are the words the file holds, with each whole-word slot
/// replaced by the value offered for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Argv {
    program: OsString,
    args: Vec<OsString>,
}

impl Argv {
    pub(crate) fn new(program: OsString, args: Vec<OsString>) -> Self {
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

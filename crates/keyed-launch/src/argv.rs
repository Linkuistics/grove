use std::ffi::{OsStr, OsString};

/// A program and its arguments, in order, ready to spawn.
///
/// There is no shell and no second reading of any word: each is spawned as it
/// is given.
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

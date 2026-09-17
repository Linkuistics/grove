/// The slot vocabulary a consumer's templates are written against.
///
/// **Supplied at load, because every template rule is a rule about slot
/// *names*.** That a substitution occupies a whole word, that it names a
/// declared slot, that a required slot appears exactly once and an optional one
/// at most once — none of them is checkable by a loader that will not learn the
/// names until expansion. Capture retains the vocabulary; resolution checks
/// active commands before anything is spawned. Expansion then checks that the
/// values offered fill the slots declared here, without reparsing their bytes.
/// Names beginning with `param.` are reserved for configuration parameters and
/// are refused by both Catalog and Templates loading, before source I/O.
pub struct Vocabulary<'a> {
    pub slots: &'a [SlotRule<'a>],
}

/// One slot, named bare. A slot named `prompt` is written `${prompt}` in a
/// template; the crate never learns what the name means.
pub struct SlotRule<'a> {
    pub name: &'a str,
    pub requirement: Requirement,
}

/// How often a slot may appear in one template.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Requirement {
    ExactlyOnce,
    AtMostOnce,
}

impl Requirement {
    pub(crate) fn admits(self, occurrences: usize) -> bool {
        match self {
            Self::ExactlyOnce => occurrences == 1,
            Self::AtMostOnce => occurrences <= 1,
        }
    }

    pub(crate) fn violation(self, name: &str) -> String {
        match self {
            Self::ExactlyOnce => {
                format!("command template must contain `${{{name}}}` exactly once")
            }
            Self::AtMostOnce => format!("`${{{name}}}` may appear at most once"),
        }
    }
}

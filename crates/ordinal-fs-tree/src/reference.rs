//! The reference domain: the course syllabus the architecture document uses for
//! all of its examples.
//!
//! Modules and lessons, each lesson carrying a `draft`/`published` attribute,
//! with a node's own content in an `OVERVIEW.md`:
//!
//! ```text
//! OVERVIEW.md                        the root's own content
//! 01-published-orientation-i1.md     a lesson  (leaf)
//! 02-linear-algebra-i2/              a module  (node)
//!   OVERVIEW.md                      the module's own content
//!   01-published-vectors-i5.md
//!   02-draft-matrices-i6.md
//! 03-draft-assessment-i9.md
//! PUBLISHING                         a witness left by an interrupted run
//! ```
//!
//! It is part of the library rather than of its tests for one reason: it is
//! shared by every test in this crate *and* by the CLI, so the document's own
//! examples and the fixtures the code is checked against cannot drift apart. It
//! is not a base class, not a default, and not something a real domain builds
//! on — a domain implements [`EntryName`] and this module is only what one
//! looks like.
//!
//! **The grammar is strict, and so is grove's** — this paragraph used to say
//! otherwise, citing a `src/tree_id.rs` that accepted a hand-typed `5-…` and
//! rendered `05-…`, so that two filenames named one entry there. That model was
//! withdrawn: grove's `task_name` refuses a lenient position by rendering what
//! it parsed and comparing, and `docs/adr/task-names-are-canonical.md` carries
//! why. Here too, `5-…` is [`Verdict::Malformed`] with the canonical spelling in
//! the message — no longer a contrast with its first consumer, just the
//! obligation both meet.

use core::fmt;

use crate::{
    EntryName, Found, Key, NameView, Ordinal, PositionedSpecies, Species, Triple, Verdict,
};

/// The name of a node's distinguished child in this domain.
pub const OVERVIEW: &str = "OVERVIEW.md";

/// A reserved name: the witness an interrupted publishing run leaves behind.
///
/// It is this domain's name and it is not an entry, so meeting one halts the
/// operation rather than being skipped — the library cannot know what it means,
/// and proceeding past it is a guess.
pub const PUBLISHING: &str = "PUBLISHING";

/// Why a string is not a well-formed [`Label`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelError {
    /// What is wrong with it, phrased for whoever has to fix the filename.
    pub reason: &'static str,
}

impl fmt::Display for LabelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.reason)
    }
}

impl std::error::Error for LabelError {}

/// The human-facing part of a name. Not unique, and not identity.
///
/// Validated on construction, so a `Label` that exists is one that renders and
/// re-parses: lowercase ASCII letters, digits and single interior hyphens,
/// starting with a letter.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label(String);

impl Label {
    /// Validate a string as a label.
    ///
    /// # Errors
    ///
    /// Returns [`LabelError`] for anything the grammar cannot render and read
    /// back — an empty string, an uppercase letter, a leading digit, a leading
    /// or trailing hyphen, or any other character.
    pub fn new(s: &str) -> Result<Self, LabelError> {
        let reason = |reason| Err(LabelError { reason });
        let mut chars = s.chars();
        match chars.next() {
            None => return reason("a label may not be empty"),
            Some(c) if !c.is_ascii_lowercase() => {
                return reason("a label starts with a lowercase ASCII letter")
            }
            Some(_) => {}
        }
        if s.ends_with('-') {
            return reason("a label may not end with a hyphen");
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return reason("a label holds lowercase ASCII letters, digits and hyphens only");
        }
        Ok(Self(s.to_string()))
    }

    /// The label as it appears in a filename.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A lesson's publication state — this domain's one attribute.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    /// Not ready to be seen.
    Draft,
    /// Live.
    Published,
}

impl Status {
    /// The token this status takes in a filename.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
        }
    }

    /// The status a filename's token names, or `None` for a token this domain
    /// does not recognise.
    ///
    /// The inverse of [`Status::token`], and public for the reason the pair
    /// exists at all: a consumer building [`Parts`] from a string — the CLI's
    /// `--status` flag is the one in this workspace — would otherwise write the
    /// mapping a second time, and a domain that can render a token it cannot
    /// read back is a domain whose own output is not valid input to it.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "draft" => Some(Self::Draft),
            "published" => Some(Self::Published),
            _ => None,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}

/// Everything in a name that this domain understands and the library does not.
///
/// The two variants are how *the species follows from the parts*: a lesson is a
/// leaf and a module is a node, and the library never has to be told which it is
/// looking at. A domain whose leaves and nodes carried the same metadata would
/// still want two variants, because that is where the species lives.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Parts {
    /// A lesson: a leaf, carrying its publication status.
    Lesson {
        /// Whether the lesson is live.
        status: Status,
        /// Its human-facing name.
        label: Label,
    },
    /// A module: a node holding lessons and further modules. Modules carry no
    /// status — publication is a property of a lesson here.
    Module {
        /// Its human-facing name.
        label: Label,
    },
}

impl Parts {
    /// A lesson's parts.
    #[must_use]
    pub const fn lesson(status: Status, label: Label) -> Self {
        Self::Lesson { status, label }
    }

    /// A module's parts.
    #[must_use]
    pub const fn module(label: Label) -> Self {
        Self::Module { label }
    }

    /// The label, whichever variant this is.
    #[must_use]
    pub const fn label(&self) -> &Label {
        match self {
            Self::Lesson { label, .. } | Self::Module { label } => label,
        }
    }

    /// The species these parts imply.
    ///
    /// [`PositionedSpecies`] and not [`Species`]: parts belong to a positioned
    /// name, and the distinguished child has none.
    #[must_use]
    pub const fn species(&self) -> PositionedSpecies {
        match self {
            Self::Lesson { .. } => PositionedSpecies::Leaf,
            Self::Module { .. } => PositionedSpecies::Node,
        }
    }
}

/// A syllabus entry's name.
///
/// The two variants are the whole of the type, and that is deliberate: a name
/// carries an ordinal, a key and parts **together**, or it is the distinguished
/// child and carries none of them. The obligation *a name is positioned or
/// distinguished, never neither* is therefore not something this domain can
/// break — see [`EntryName::view`], and `docs/formalism-findings.md` entry
/// 002, whose counterfactual asked for exactly this encoding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyllabusName {
    /// An ordinary entry: a lesson or a module.
    Positioned {
        /// Its position among its siblings.
        ordinal: Ordinal,
        /// Its identity.
        key: Key,
        /// Everything else.
        parts: Parts,
    },
    /// `OVERVIEW.md` — a node's own content.
    Overview,
}

impl fmt::Display for SyllabusName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overview => f.write_str(OVERVIEW),
            Self::Positioned {
                ordinal,
                key,
                parts,
            } => {
                let ordinal = ordinal.get();
                let key = key.get();
                match parts {
                    // A lesson is a regular file, so it takes the `.md` suffix;
                    // a module is a directory and takes none. The suffix is what
                    // the *name* declares its species to be, which `parse` then
                    // reconciles against what the listing actually found.
                    Parts::Lesson { status, label } => {
                        write!(f, "{ordinal:02}-{status}-{label}-i{key}.md")
                    }
                    Parts::Module { label } => write!(f, "{ordinal:02}-{label}-i{key}"),
                }
            }
        }
    }
}

/// What this domain says when it refuses a name.
///
/// Every variant carries recovery advice in its [`fmt::Display`], not just
/// detection: the library halts on a [`Verdict::Malformed`] or a
/// [`Verdict::Reserved`] wherever in the tree it sits, and an error that only
/// says *something is wrong* leaves whoever hit it with a frozen tree and no
/// next step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyllabusError {
    /// A name that means what a different spelling of it means.
    NotCanonical {
        /// What is on disk.
        name: String,
        /// What it should be.
        canonical: String,
    },
    /// A lesson whose first token is neither `draft` nor `published`.
    UnknownStatus {
        /// What is on disk.
        name: String,
        /// The token found in the status position.
        token: String,
    },
    /// A label the grammar cannot render and read back.
    BadLabel {
        /// What is on disk.
        name: String,
        /// The offending label.
        label: String,
        /// Why it is not a label.
        error: LabelError,
    },
    /// A name that is this domain's shape but whose species contradicts what the
    /// listing found under it.
    SpeciesMismatch {
        /// What is on disk.
        name: String,
        /// What the name says it is.
        declares: Species,
        /// What the listing reported.
        found: Found,
    },
    /// The reserved witness of an interrupted publishing run.
    PublishInterrupted {
        /// What is on disk.
        name: String,
    },
}

impl fmt::Display for SyllabusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotCanonical { name, canonical } => write!(
                f,
                "`{name}` is a syllabus name spelled a way this grammar does not write. \
                 Rename it to `{canonical}`. Two spellings of one name mean two files on \
                 disk are one entry, sharing a key and an ordinal."
            ),
            Self::UnknownStatus { name, token } => write!(
                f,
                "`{name}` is a lesson whose status is `{token}`, which is neither `draft` \
                 nor `published`. Rename it with one of those two in that position, or \
                 out of the `NN-…-i<key>.md` shape entirely if it is not a lesson."
            ),
            Self::BadLabel { name, label, error } => write!(
                f,
                "`{name}` carries the label `{label}`, which is not one: {error}. Rename \
                 it with a label that is."
            ),
            Self::SpeciesMismatch {
                name,
                declares,
                found,
            } => write!(
                f,
                "`{name}` names a {declares}, which must be {}, but the listing found \
                 {found}. Nothing here can be right: either the name or the object is \
                 wrong, and a walk that skipped this would lose everything under it.",
                declares.requires()
            ),
            Self::PublishInterrupted { name } => write!(
                f,
                "`{name}` is the witness a publishing run leaves while it works, so a run \
                 was interrupted or one is in progress. Wait for it, or — once you have \
                 confirmed no run is live — delete the file to release the tree."
            ),
        }
    }
}

impl std::error::Error for SyllabusError {}

impl EntryName for SyllabusName {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        // The distinguished child and the reserved witness are exact names, and
        // both are checked before the positioned grammar because neither is
        // positioned: nothing below would recognise them.
        if name == OVERVIEW {
            return match agree(Species::Distinguished, found, name) {
                Some(err) => Verdict::Malformed(err),
                None => Verdict::Entry(Self::Overview),
            };
        }
        if name == PUBLISHING {
            return Verdict::Reserved(SyllabusError::PublishInterrupted {
                name: name.to_string(),
            });
        }

        // The `.md` suffix is what the name *declares* its species to be.
        let (stem, declares_lesson) = match name.strip_suffix(".md") {
            Some(stem) => (stem, true),
            None => (name, false),
        };

        // Is this name ours at all? A name is this domain's when it is
        // positioned *and* keyed — a leading digit run and a terminal
        // `-i<digits>`. Everything else is Foreign and skipped, which is safe
        // precisely because we are disclaiming it. Everything that *is* this
        // shape and does not parse is Malformed, because skipping it would be
        // data loss — and a whole subtree of it when the name is a directory.
        let Some((digits, middle, key_digits)) = split_shape(stem) else {
            return Verdict::Foreign;
        };

        // An empty middle is this domain's shape with its label missing —
        // `01--i3`, which is exactly what an empty label renders as — so it is
        // Malformed and not Foreign. Disclaiming it would skip the file, and
        // skip the whole subtree beneath it when it is a directory, while the
        // walk reported a healthy tree. Checked here rather than left to the
        // branches below so that the lesson form, whose middle is
        // `status-label`, refuses for the reason it is actually missing a label
        // rather than for a missing status.
        if middle.is_empty() {
            return Verdict::Malformed(SyllabusError::BadLabel {
                name: name.to_string(),
                label: String::new(),
                error: Label::new(middle).expect_err("the empty string is not a label"),
            });
        }

        // Canonicity, in the cheapest form there is: parse the number, render it
        // the one way this grammar renders it, and refuse anything else. `5-…`
        // and `005-…` are this domain's names spelled wrong, not foreign ones.
        let Ok(ordinal) = digits.parse::<u32>() else {
            return Verdict::Malformed(not_canonical(name));
        };
        let Ok(key) = key_digits.parse::<u32>() else {
            return Verdict::Malformed(not_canonical(name));
        };

        let parts = if declares_lesson {
            let Some((token, label)) = middle.split_once('-') else {
                return Verdict::Malformed(SyllabusError::UnknownStatus {
                    name: name.to_string(),
                    token: middle.to_string(),
                });
            };
            let Some(status) = Status::from_token(token) else {
                return Verdict::Malformed(SyllabusError::UnknownStatus {
                    name: name.to_string(),
                    token: token.to_string(),
                });
            };
            match Label::new(label) {
                Ok(label) => Parts::lesson(status, label),
                Err(error) => {
                    return Verdict::Malformed(SyllabusError::BadLabel {
                        name: name.to_string(),
                        label: label.to_string(),
                        error,
                    })
                }
            }
        } else {
            match Label::new(middle) {
                Ok(label) => Parts::module(label),
                Err(error) => {
                    return Verdict::Malformed(SyllabusError::BadLabel {
                        name: name.to_string(),
                        label: middle.to_string(),
                        error,
                    })
                }
            }
        };

        let parts_species = parts.species();
        let parsed = Self::Positioned {
            ordinal: Ordinal::new(ordinal),
            key: Key::new(key),
            parts,
        };

        // The canonicity obligation, discharged in one line over the whole
        // grammar rather than one check per field: whatever was parsed, render
        // it, and refuse the input if it is not what this grammar writes. A
        // padding rule added later cannot escape this, which is why it is here
        // as well as in the field-level parses above.
        if parsed.to_string() != name {
            return Verdict::Malformed(not_canonical_as(name, &parsed));
        }

        // The species of a positioned name is `positioned_species(parts)` and
        // nothing else — the same function the seam derives `species()` from.
        match agree(parts_species.species(), found, name) {
            Some(err) => Verdict::Malformed(err),
            None => Verdict::Entry(parsed),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self::Positioned {
            ordinal,
            key,
            parts,
        }
    }

    fn distinguished() -> Option<Self> {
        Some(Self::Overview)
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        match self {
            Self::Overview => NameView::Distinguished,
            Self::Positioned {
                ordinal,
                key,
                parts,
            } => NameView::Positioned(Triple {
                ordinal: *ordinal,
                key: *key,
                parts,
            }),
        }
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        parts.species()
    }
}

/// `Some(error)` when what the listing found contradicts what the name declares.
fn agree(declares: Species, found: Found, name: &str) -> Option<SyllabusError> {
    (!declares.agrees_with(found)).then(|| SyllabusError::SpeciesMismatch {
        name: name.to_string(),
        declares,
        found,
    })
}

/// A canonicity refusal whose advice we cannot compute — the numbers did not
/// even parse, so there is no canonical spelling to offer.
fn not_canonical(name: &str) -> SyllabusError {
    SyllabusError::NotCanonical {
        name: name.to_string(),
        canonical: "a name with an ordinal and a key that fit in 32 bits".to_string(),
    }
}

/// A canonicity refusal that can name the spelling this grammar writes.
fn not_canonical_as(name: &str, parsed: &SyllabusName) -> SyllabusError {
    SyllabusError::NotCanonical {
        name: name.to_string(),
        canonical: parsed.to_string(),
    }
}

/// Split a stem into `(ordinal digits, middle, key digits)`, or `None` when the
/// stem is not this domain's shape at all.
///
/// The key is the *terminal* `-i<digits>`, which is what keeps a label
/// containing `-i7` unambiguous: `01-draft-notes-i7-i3.md` is the label
/// `notes-i7` at key 3.
fn split_shape(stem: &str) -> Option<(&str, &str, &str)> {
    let dash = stem.find('-')?;
    let (digits, rest) = (&stem[..dash], &stem[dash + 1..]);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let key_start = rest.len() - rest.bytes().rev().take_while(u8::is_ascii_digit).count();
    if key_start == rest.len() {
        return None; // no trailing key digits
    }
    let middle = rest[..key_start].strip_suffix("-i")?;
    // An empty middle is *not* a reason to disclaim the name: `01--i3` has both
    // markers this domain recognises its own names by, with the label between
    // them missing. `parse` refuses it as Malformed; skipping it would be data
    // loss, and a whole subtree of it when the name is a directory.
    Some((digits, middle, &rest[key_start..]))
}

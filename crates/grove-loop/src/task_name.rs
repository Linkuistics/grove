// Grove's canonical task names, handles and per-level node-file rule.
//
//     leaf       NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md
//     node dir   NN-k<key>
//     node file  _<slug>.md
//     root file  _BRIEF.md
//
// Directories carry position and key; their node files carry their titles.
// Names starting with a digit or underscore belong to this grammar, so a
// malformed spelling refuses the whole read rather than hiding a subtree.
// The contract is docs/adr/task-names-are-canonical.md.

use core::fmt;

use ordinal_fs_tree::{
    EntryName, Found, Key, NameView, Ordinal, PositionedSpecies, Species, Triple, Verdict,
};

/// The name of a node's distinguished child: the charter every node directory is
/// headed by.
pub const BRIEF: &str = "_BRIEF.md";

/// The permanent key's delimiter — the terminal `-k<digits>` of every positioned
/// name (task-tree-scheme, amending the original `[<key>]`: brackets are
/// shell-glob metacharacters and `-k` is glob-safe).
const KEY_MARK: &str = "-k";

/// The separator between a leaf's session kind and its slug
/// (`grammar-separator-k15`, `docs/specs/module-decomposition.md` decision 3).
///
/// A single `-` cannot delimit them: both tokens are hyphenated words, so
/// `design-decomposition` reads as kind `design` + slug `decomposition` **and**
/// as kind `design-decomposition` + empty slug. Matching the middle against a
/// closed kind set was the only thing that resolved that, and `open-kind-k20`
/// took the set away — so the separator is now the *whole* of what says where
/// the kind ends, and a name without it has no reading at all rather than two.
/// The middle splits at the **first** `--`; neither token may contain one, which
/// is why [`refuse_token`] refuses it for both.
const SEPARATOR: &str = "--";

/// A leaf's outcome: live, retired (`DONE`), or abandoned (`ABANDONED`) —
/// mutually exclusive by construction, so the impossible fourth state cannot be
/// written. A node directory never carries one; its done-ness is the absence of
/// a live leaf in its subtree, which is why [`Parts::Node`] has no such field
/// rather than a field constrained to one value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Not yet retired or abandoned — what `pick` returns.
    Live,
    /// Work completed — the `DONE-` infix.
    Done,
    /// Work rejected, closed, not going to happen — the `ABANDONED-` infix. The
    /// *why* lives in the ADR set, not the filename.
    Abandoned,
}

impl Outcome {
    /// The infix this outcome takes, immediately after the position. Empty for
    /// [`Outcome::Live`], which is the absence of a mark rather than a mark.
    const fn infix(self) -> &'static str {
        match self {
            Self::Live => "",
            Self::Done => "DONE-",
            Self::Abandoned => "ABANDONED-",
        }
    }

    /// The outcome an infix names, and how much of the name it consumed.
    ///
    /// The inverse of [`Outcome::infix`], and paired with it here for the reason
    /// `cli-k16` found the hard way (`docs/formalism-findings.md` entry 019): a
    /// domain whose token mapping runs one way only gets the other direction
    /// written a second time by its first consumer, and two spellings of one
    /// mapping drift.
    fn strip(rest: &str) -> (Self, &str) {
        for outcome in [Self::Done, Self::Abandoned] {
            if let Some(after) = rest.strip_prefix(outcome.infix()) {
                return (outcome, after);
            }
        }
        (Self::Live, rest)
    }
}

/// Why a string is not a well-formed [`Slug`] or [`Kind`].
///
/// **One refusal because there is one rule.** A leaf name is built from two
/// words — the session kind and the slug — and whatever else they mean, each has
/// to survive being written into a filename beside the `--` that separates them
/// and read back as the same two words. A second error type would be a second
/// statement of that shape, and the two would drift the first time either word's
/// character set moved. `open-kind-k20` is what made the sharing possible: until
/// then a kind was checked against a closed set rather than against a shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenError {
    /// What is wrong with it, phrased for whoever has to fix the filename, and
    /// **naming the offending character** where there is one — "not one of
    /// those" about a forty-character name is a hunt.
    pub reason: String,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.reason)
    }
}

impl std::error::Error for TokenError {}

/// The shape a leaf name's two words share — or the reason this string has not
/// got it.
///
/// `noun` is the word being refused, so one rule produces a message that reads
/// as though it had been written for the word in hand. Every clause here is
/// load-bearing on the grammar rather than on taste:
///
/// * **empty** — a missing word would move the `--` and change where the name
///   splits;
/// * **the grammar's own markers** — reserved so a name cannot spell one;
/// * **a leading or trailing dash** — a kind ending in one renders `impl---slug`,
///   which splits at the *first* `--` and reads back as kind `impl`, slug
///   `-slug`. Canonicity is what this clause protects, not tidiness;
/// * **the separator** — either word containing `--` gives the name two readings;
/// * **the character set** — everything outside it either blurs a name boundary
///   (`.`, `/`) or collides with the uppercase outcome infixes.
fn refuse_token(noun: &str, token: &str) -> Option<String> {
    if token.is_empty() {
        return Some(format!("a {noun} may not be empty"));
    }
    if matches!(token, "BRIEF" | "DONE" | "ABANDONED") {
        return Some(format!(
            "`BRIEF`, `DONE` and `ABANDONED` are reserved: the grammar's own markers, and a \
             {noun} spelling one would name a marker instead"
        ));
    }
    if token.starts_with('-') || token.ends_with('-') {
        return Some(format!("a {noun} may not start or end with a dash"));
    }
    if token.contains(SEPARATOR) {
        return Some(format!(
            "a {noun} may not contain `{SEPARATOR}`: that is the separator between the session \
             kind and the slug"
        ));
    }
    if let Some(refused) = token
        .chars()
        .find(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-'))
    {
        return Some(format!(
            "a {noun} holds lowercase ASCII letters, digits and dashes only, and {refused:?} is \
             none of those"
        ));
    }
    None
}

/// A leaf's **session kind**: the word before the `--`, the skill a session is
/// told to load, and the key its command template is configured under.
///
/// **It is an open token, and that is the whole of the type**
/// (`docs/adr/a-kind-is-an-open-token.md`, `docs/specs/module-decomposition.md`
/// decision 5). It was a compiled enum of nineteen variants until
/// `open-kind-k20`, with a parse arm, a label arm and an `ALL` roster per kind —
/// so every kind the methodology has ever had was also a fact about the binary,
/// and adding one meant editing and shipping Rust. Now grove validates the
/// *shape* and holds no opinion about the meaning: a kind for which no skill is
/// installed parses, launches, and fails in the session that could not load the
/// skill, where a human is present to read the message.
///
/// **Grove spells exactly two of them**, and only because it writes those two
/// leaves itself with no session to delegate to: [`Kind::requirements`] for
/// root scaffolding and [`Kind::finish`] for the teardown sentinel. No third
/// *kind* literal exists in the machinery, and no enumeration at all. (Other
/// kind-**shaped** literals do — `plan` and `finish` are the two slugs grove
/// names, and `leaf-add` and friends are verb names in refusals — which is why
/// the claim is checked by enumerating every string literal and classifying it,
/// never by grepping a list of kind names.)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kind(String);

/// Root scaffolding's kind. One of the two tokens grove may name.
const REQUIREMENTS: &str = "requirements";

/// The driver-owned teardown sentinel's kind. The other one.
const FINISH: &str = "finish";

impl Kind {
    /// Validate a string as a session kind.
    ///
    /// # Errors
    ///
    /// Returns [`TokenError`] for anything the grammar cannot render and read
    /// back — the same shape a [`Slug`] obeys, for the same reasons.
    pub fn new(kind: &str) -> Result<Self, TokenError> {
        match refuse_token("session kind", kind) {
            Some(reason) => Err(TokenError { reason }),
            None => Ok(Self(kind.to_string())),
        }
    }

    /// The kind of the leaf `root-init` lays down.
    ///
    /// A constructor rather than a constant so the literal has one home; grove
    /// authors that leaf before any session exists, which is the licence for
    /// naming a kind at all.
    #[must_use]
    pub fn requirements() -> Self {
        Self(REQUIREMENTS.to_string())
    }

    /// The kind of the teardown sentinel the loop writes between the last
    /// ordinary session and the finish session — the other leaf grove authors.
    #[must_use]
    pub fn finish() -> Self {
        Self(FINISH.to_string())
    }

    /// Is this the driver's own sentinel?
    ///
    /// Grove recognising the leaf it wrote itself — which is what licenses the
    /// seven calls in six functions that ask: selection, creation, `finish_commit`,
    /// and the three refusals to decompose, retire or prune an existing one. Not
    /// an interpretation of the methodology, which grove performs nowhere.
    #[must_use]
    pub fn is_finish(&self) -> bool {
        self.0 == FINISH
    }

    /// The token as it appears in a filename, in a skill name, and as a
    /// configuration key.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The human-facing part of a name. Not unique, and not identity — the key is.
///
/// Validated on construction, so a `Slug` that exists is one that renders and
/// re-parses. The character set already excludes everything that could blur a
/// name boundary (`.`, `/`, `[`, `]`), and being lowercase keeps it clear of the
/// uppercase outcome infixes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slug(String);

impl Slug {
    /// Validate a string as a slug.
    ///
    /// # Errors
    ///
    /// Returns [`TokenError`] for anything the grammar cannot render and read
    /// back: an empty string, a leading or trailing hyphen, a character outside
    /// lowercase ASCII, digits and hyphens, or one of the reserved words the
    /// grammar's own markers use.
    pub fn new(slug: &str) -> Result<Self, TokenError> {
        match refuse_token("slug", slug) {
            Some(reason) => Err(TokenError { reason }),
            None => Ok(Self(slug.to_string())),
        }
    }

    /// The slug as it appears in a filename.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Why a string is not a well-formed [`Handle`].
///
/// The same model as [`TaskNameError`]: every variant carries what it was
/// handed **and** what it should have been, because a handle reaches this type
/// from a human's command line as often as from a name, and a refusal that only
/// says *no* leaves the operator guessing at the grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandleError {
    /// No terminal `-k<digits>` at all — not a handle, whatever else it is.
    NotHandleShaped {
        /// What was handed in.
        text: String,
    },
    /// A key that is noncanonical, zero or outside the representable range.
    BadKey {
        /// What was handed in.
        text: String,
        /// The refused digit run.
        digits: String,
    },
    /// A terminal key preceded by something that is not a slug.
    BadSlug {
        /// What was handed in.
        text: String,
        /// The offending slug.
        slug: String,
        /// Why it is not a slug.
        error: TokenError,
    },
}

impl fmt::Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotHandleShaped { text } => write!(
                f,
                "{text:?} is not a Grove handle: expected <slug>-k<key>, the position-free \
                 identity a task keeps for its whole life — `name-ownership-k14`. The key is \
                 the terminal `-k<digits>`, so a slug may contain `-k9` and still be read \
                 unambiguously."
            ),
            Self::BadKey { text, digits } => write!(
                f,
                "{text:?} is not a Grove handle: the key {digits:?} must be positive decimal without leading zero and fit in 32 bits. \
                 A handle's key is the one the tree allocated, and no tree has allocated \
                 that."
            ),
            Self::BadSlug { text, slug, error } => write!(
                f,
                "{text:?} is not a Grove handle: the slug {slug:?} is not one — {error}. A \
                 handle is <slug>-k<key> and its slug obeys the same rule a filename's does."
            ),
        }
    }
}

impl std::error::Error for HandleError {}

/// The position-free identity of a work item: `<slug>-k<key>`.
/// Leaf names supply both fields; node handles pair the directory key with
/// the slug of its node file, supplied by the tree's guarded snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handle {
    slug: Slug,
    key: Key,
}

impl Handle {
    /// The handle of a slug and the key the tree allocated for it.
    ///
    /// The caller supplies a positive key. The generic library permits zero,
    /// but zero is outside Grove’s domain and does not round-trip through parse.
    #[must_use]
    pub const fn new(slug: Slug, key: Key) -> Self {
        Self { slug, key }
    }

    /// Construct a leaf handle; every other name species has none.
    #[must_use]
    pub fn of_leaf(name: &TaskName) -> Option<Self> {
        match name {
            TaskName::Positioned {
                key,
                parts: Parts::Leaf { slug, .. },
                ..
            } => Some(Self::new(slug.clone(), *key)),
            _ => None,
        }
    }

    /// Pair a node directory with its titled node file. The caller establishes
    /// parentage; this constructor rejects every other species pairing.
    #[must_use]
    pub fn of_node(node: &TaskName, file: &TaskName) -> Option<Self> {
        match (node, file) {
            (
                TaskName::Positioned {
                    key,
                    parts: Parts::Node,
                    ..
                },
                TaskName::NodeFile(slug),
            ) => Some(Self::new(slug.clone(), *key)),
            _ => None,
        }
    }

    /// Parse a canonical handle using the same key grammar as directory names.
    ///
    /// # Errors
    /// Returns [`HandleError`] for a missing, noncanonical or out-of-range key,
    /// or a head that is not a valid slug.
    pub fn parse(text: &str) -> Result<Self, HandleError> {
        let Some((before, digits)) = peel_key(text) else {
            return Err(HandleError::NotHandleShaped {
                text: text.to_string(),
            });
        };
        let Some(key) = parse_key(digits) else {
            return Err(HandleError::BadKey {
                text: text.to_string(),
                digits: digits.to_string(),
            });
        };
        match Slug::new(before) {
            Ok(slug) => Ok(Self::new(slug, key)),
            Err(error) => Err(HandleError::BadSlug {
                text: text.to_string(),
                slug: before.to_string(),
                error,
            }),
        }
    }

    /// Its human-facing part.
    #[must_use]
    pub const fn slug(&self) -> &Slug {
        &self.slug
    }

    /// Its permanent identity.
    #[must_use]
    pub const fn key(&self) -> Key {
        self.key
    }

    /// **The one place the `<slug>-k<key>` grammar is spelled.**
    ///
    /// Taken by parts rather than by `&self` so [`TaskName`]'s renderings can
    /// end in it without cloning a slug they already hold — the point being that
    /// there is one `write!`, not that a `Handle` value has to exist to reach
    /// it.
    fn render(f: &mut fmt::Formatter<'_>, slug: &Slug, key: Key) -> fmt::Result {
        write!(f, "{slug}")?;
        render_key(f, key)
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::render(f, &self.slug, self.key)
    }
}

/// Everything in a name that grove understands and the library does not.
///
/// The two variants are how *the species follows from the parts*: a task is a
/// leaf and a node directory is a node, and the library never has to be told
/// which it is looking at. They also carry the asymmetry the grammar already
/// has — a leaf has a session kind and an outcome, a node has neither — as an
/// absence of fields rather than as fields nothing may fill.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Parts {
    /// A task file: a leaf, carrying its session kind and its outcome.
    Leaf {
        /// Live, `DONE` or `ABANDONED`.
        outcome: Outcome,
        /// The session kind the file is driven as.
        kind: Kind,
        /// Its human-facing name.
        slug: Slug,
    },
    /// A positioned directory, whose title belongs to its node file.
    Node,
}

impl Parts {
    /// A leaf's parts.
    #[must_use]
    pub const fn leaf(outcome: Outcome, kind: Kind, slug: Slug) -> Self {
        Self::Leaf {
            outcome,
            kind,
            slug,
        }
    }

    /// A node's parts.
    #[must_use]
    pub const fn node() -> Self {
        Self::Node
    }

    /// A leaf's title. Nodes carry no title in their parts.
    #[must_use]
    pub const fn slug(&self) -> Option<&Slug> {
        match self {
            Self::Leaf { slug, .. } => Some(slug),
            Self::Node => None,
        }
    }

    /// The species these parts imply.
    ///
    /// [`PositionedSpecies`] and not [`Species`]: parts belong to a positioned
    /// name, and the distinguished child has none.
    #[must_use]
    pub const fn species(&self) -> PositionedSpecies {
        match self {
            Self::Leaf { .. } => PositionedSpecies::Leaf,
            Self::Node => PositionedSpecies::Node,
        }
    }
}

/// A task tree entry's name.
///
/// Direct construction, including [`EntryName::compose`], requires a positive
/// key for a domain-valid Grove name. The generic library’s total constructor
/// preserves even zero, which Grove refuses when parsing and never allocates.
/// The parse/render round-trip promise applies only to domain-valid inputs.
///
/// A positioned name carries ordinal, key and parts together. Root and titled
/// node files are distinguished names with no position or key.
/// The obligation *a name is positioned or distinguished, never neither* is
/// therefore not something this domain can break — see
/// [`EntryName::view`](ordinal_fs_tree::EntryName::view).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskName {
    /// An ordinary entry: a task leaf or a node directory.
    Positioned {
        /// Its position among its siblings.
        ordinal: Ordinal,
        /// Its permanent identity.
        key: Key,
        /// Everything else.
        parts: Parts,
    },
    /// `_BRIEF.md` — the root node file.
    Brief,
    /// A positioned node's titled file; never a leaf or a separate handle.
    NodeFile(Slug),
}

impl fmt::Display for TaskName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Brief => f.write_str(BRIEF),
            Self::NodeFile(slug) => write!(f, "_{slug}.md"),
            Self::Positioned {
                ordinal,
                key,
                parts,
            } => {
                // `{:02}` is a *minimum* width, not an exact one: position 100
                // renders `100`. The canonical rule is therefore "zero-padded to
                // at least two digits, and no other leading zero" — `05` and
                // `100` canonical, `5` and `005` not.
                let ordinal = ordinal.get();
                // **Both arms below end in `Handle::render`**, and neither
                // spells `<slug>-k<key>` itself. That is decision 4's structural
                // form: the filename and the handle are one rendering, so drift
                // between them is not something this type can express — and the
                // handle is a contiguous terminal substring of the name (a leaf
                // then takes its suffix), which is the property
                // `grammar-separator-k15` builds on.
                match parts {
                    // A leaf is a regular file and takes the `.md` suffix; a node
                    // is a directory and takes none. The suffix is what the
                    // *name* declares its species to be, which `parse` then
                    // reconciles against what the listing actually found.
                    Parts::Leaf {
                        outcome,
                        kind,
                        slug,
                    } => {
                        write!(
                            f,
                            "{ordinal:02}-{}{}{SEPARATOR}",
                            outcome.infix(),
                            kind.label()
                        )?;
                        Handle::render(f, slug, *key)?;
                        f.write_str(".md")
                    }
                    Parts::Node => {
                        write!(f, "{ordinal:02}")?;
                        render_key(f, *key)
                    }
                }
            }
        }
    }
}

/// What grove says when it refuses a name.
///
/// Every variant carries recovery advice in its [`fmt::Display`], not merely
/// detection: the library halts on a [`Verdict::Malformed`] or a
/// [`Verdict::Reserved`] wherever in the tree it sits, and an error that only
/// says *something is wrong* leaves whoever hit it with a frozen tree and no
/// next step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskNameError {
    /// An owned name does not have a complete canonical spelling.
    InvalidName { name: String },
    /// The complete node-file set has wrong cardinality or placement.
    NodeFiles {
        node: Option<String>,
        names: Vec<String>,
    },
    /// A task-shaped name spelled a way grove does not write — a hand-typed
    /// `5-…` where grove renders `05-…`, or a number too large to hold.
    NotCanonical {
        /// What is on disk.
        name: String,
        /// What it should be.
        canonical: String,
    },
    /// A task-shaped leaf with no `--` between its session kind and its slug —
    /// a name written under the grammar that predates `grammar-separator-k15`,
    /// or one hand-typed without it.
    MissingSeparator {
        /// What is on disk.
        name: String,
    },
    /// A task-shaped leaf whose session-kind token is not a well-formed one.
    ///
    /// **A shape refusal, not an unknown-kind one** (`open-kind-k20`). Grove
    /// holds no set of kinds to fail membership in; what it can still say is
    /// that a token cannot be written into a name and read back, and the
    /// refusal names the character that made it so.
    BadKind {
        /// What is on disk.
        name: String,
        /// The token that sat before the separator.
        kind: String,
        /// Why it is not a session kind.
        error: TokenError,
    },
    BadSlug {
        /// What is on disk.
        name: String,
        /// The offending slug.
        slug: String,
        /// Why it is not a slug.
        error: TokenError,
    },
    /// A task-shaped name whose species contradicts what the listing found under
    /// it — a directory named `01-impl--a-k1.md`, a file named `01-a-k1`.
    SpeciesMismatch {
        /// What is on disk.
        name: String,
        /// What the name says it is.
        declares: Species,
        /// What the listing reported.
        found: Found,
    },
}

impl fmt::Display for TaskNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName { name } => write!(f,
                "malformed Grove name {name:?}: expected NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md, NN-k<key>/, _<slug>.md in a node, or _BRIEF.md at the root; keys are positive decimal without leading zero and fit in 32 bits"),
            Self::NodeFiles { node, names } => {
                let required = if node.is_some() { "_<slug>.md" } else { "_BRIEF.md" };
                write!(f, "malformed Grove level: expected exactly one regular node file {required}; found {names:?}")?;
                if node.is_some() && names.is_empty() {
                    f.write_str(". Check for an interrupted `leaf-decompose`: if this directory is empty and a sibling leaf shares its position and key, delete the empty directory to retain the leaf, or move that leaf into it as its _<slug>.md node file. Retain the key and leaf body; do not allocate a fresh key or manufacture a brief")?;
                }
                Ok(())
            }

            Self::NotCanonical { name, canonical } => write!(
                f,
                "{name:?} is a Grove task name spelled a way Grove does not write. Rename it \
                 to {canonical:?}. A position is zero-padded to at least two digits and \
                 carries no other leading zero, so `05` and `100` are names and `5` and \
                 `005` are not. Two spellings of one name mean two files on disk are one \
                 entry, sharing a key and a position."
            ),
            Self::MissingSeparator { name } => write!(
                f,
                "malformed Grove leaf {name:?}: no `--` between the session kind and the \
                 slug. The canonical form is \
                 NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md — rename it with \
                 `--` where the kind ends, and single dashes everywhere else. Without the \
                 separator a hyphenated kind beside a hyphenated slug has more than one \
                 reading, and the readings differ in the handle."
            ),
            Self::BadKind { name, kind, error } => write!(
                f,
                "malformed Grove leaf {name:?}: the session kind {kind:?} is not one — \
                 {error}. Expected NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md. \
                 Any well-formed token is a kind; whether a skill exists for it is the \
                 methodology's business and not this grammar's."
            ),
            Self::BadSlug { name, slug, error } => write!(
                f,
                "malformed Grove task name {name:?}: the slug {slug:?} is not one — \
                 {error}. Expected _<slug>.md or NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md."
            ),
            Self::SpeciesMismatch {
                name,
                declares,
                found,
            } => write!(
                f,
                "malformed Grove tree: {name:?} names a {declares}, which must be {}, but \
                 the listing found {found}. Nothing here can be right — either the name or \
                 the object is wrong — and a walk that skipped it would lose everything \
                 under it. Required forms: NN-k<key>/, NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md, _<slug>.md or _BRIEF.md.",
                declares.requires()
            ),
        }
    }
}

impl std::error::Error for TaskNameError {}

impl EntryName for TaskName {
    type Parts = Parts;
    type Err = TaskNameError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        // The charter is matched before the positioned grammar, because it is
        // not positioned and nothing below would recognise it.
        if let Some(token) = name.strip_prefix('_') {
            let parsed = if name == BRIEF {
                Self::Brief
            } else {
                let Some(token) = token.strip_suffix(".md") else {
                    return Verdict::Malformed(TaskNameError::InvalidName {
                        name: name.to_string(),
                    });
                };
                match Slug::new(token) {
                    Ok(slug) => Self::NodeFile(slug),
                    Err(error) => return Verdict::Malformed(bad_slug(name, token, error)),
                }
            };
            return match disagreement(Species::Distinguished, found, name) {
                Some(error) => Verdict::Malformed(error),
                None => Verdict::Entry(parsed),
            };
        }
        if !name.as_bytes().first().is_some_and(u8::is_ascii_digit) {
            return Verdict::Foreign;
        }
        let (stem, declares_leaf) = match name.strip_suffix(".md") {
            Some(stem) => (stem, true),
            None => (name, false),
        };

        // Digit-prefixed entries are owned even when their position, key or
        // species is malformed. Refusing them keeps hidden work out of a walk.
        let shape = if declares_leaf {
            split_shape(stem)
        } else {
            peel_key(stem).map(|(digits, key)| (digits, "", key))
        };
        let Some((digits, middle, key_digits)) = shape else {
            return Verdict::Malformed(TaskNameError::InvalidName {
                name: name.to_string(),
            });
        };
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Verdict::Malformed(TaskNameError::InvalidName {
                name: name.to_string(),
            });
        }
        let Ok(ordinal) = digits.parse::<u32>() else {
            return Verdict::Malformed(uncomputable_canonical(name));
        };
        let Some(key) = parse_key(key_digits) else {
            return Verdict::Malformed(TaskNameError::InvalidName {
                name: name.to_string(),
            });
        };

        let (outcome, after_outcome) = Outcome::strip(middle);

        let parts = if declares_leaf {
            // The middle splits at the **first** `--`, and that is the whole of
            // the kind/slug boundary: no longest-match against a label set, no
            // second reading to choose between. `split_filename_prefix` — which
            // resolved the ambiguity by consulting the closed set — went with
            // this line, because `open-kind-k20` takes the set away and the
            // separator is what makes that safe.
            let Some((kind_token, slug)) = after_outcome.split_once(SEPARATOR) else {
                return Verdict::Malformed(TaskNameError::MissingSeparator {
                    name: name.to_string(),
                });
            };
            // Two distinct failures now, where the old grammar could only report
            // one: a name with no separator is *shaped* wrong, and a name with a
            // separator has a single token to quote back at whoever wrote it.
            let kind = match Kind::new(kind_token) {
                Ok(kind) => kind,
                Err(error) => {
                    return Verdict::Malformed(TaskNameError::BadKind {
                        name: name.to_string(),
                        kind: kind_token.to_string(),
                        error,
                    })
                }
            };
            match Slug::new(slug) {
                Ok(slug) => Parts::leaf(outcome, kind, slug),
                Err(error) => return Verdict::Malformed(bad_slug(name, slug, error)),
            }
        } else {
            Parts::node()
        };

        let parts_species = parts.species();
        let parsed = Self::Positioned {
            ordinal: Ordinal::new(ordinal),
            key,
            parts,
        };

        // Canonicity, in the cheapest form there is, and over the whole grammar
        // rather than one rule per field: whatever was parsed, render it, and
        // refuse the input when it is not what Grove writes. This is the line
        // the withdrawn grammar did not have — it accepted `5` where its own
        // renderer wrote `05` — and a padding or ordering rule added later
        // cannot escape it.
        let canonical = parsed.to_string();
        if canonical != name {
            return Verdict::Malformed(TaskNameError::NotCanonical {
                name: name.to_string(),
                canonical,
            });
        }

        match disagreement(parts_species.species(), found, name) {
            Some(error) => Verdict::Malformed(error),
            None => Verdict::Entry(parsed),
        }
    }

    fn validate_distinguished(node: Option<&Self>, children: &[Self]) -> Result<(), Self::Err> {
        let valid = matches!(
            (node, children),
            (None, [Self::Brief])
                | (
                    Some(Self::Positioned {
                        parts: Parts::Node,
                        ..
                    }),
                    [Self::NodeFile(_)]
                )
        );
        if valid {
            Ok(())
        } else {
            let mut names: Vec<_> = children.iter().map(ToString::to_string).collect();
            names.sort();
            Err(TaskNameError::NodeFiles {
                node: node.map(ToString::to_string),
                names,
            })
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self::Positioned {
            ordinal,
            key,
            parts,
        }
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        match self {
            Self::Brief | Self::NodeFile(_) => NameView::Distinguished,
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
fn disagreement(declares: Species, found: Found, name: &str) -> Option<TaskNameError> {
    (!declares.agrees_with(found)).then(|| TaskNameError::SpeciesMismatch {
        name: name.to_string(),
        declares,
        found,
    })
}

fn bad_slug(name: &str, slug: &str, error: TokenError) -> TaskNameError {
    TaskNameError::BadSlug {
        name: name.to_string(),
        slug: slug.to_string(),
        error,
    }
}

/// A canonicity refusal whose advice cannot be computed: the numbers did not fit
/// in 32 bits, so there is no spelling to offer back.
fn uncomputable_canonical(name: &str) -> TaskNameError {
    TaskNameError::InvalidName {
        name: name.to_string(),
    }
}

/// Split a leaf stem into its position, kind/slug middle and terminal key.
/// A failed split is still an owned refusal because the caller classified
/// digit-prefixed names before reaching this helper.
fn split_shape(stem: &str) -> Option<(&str, &str, &str)> {
    let dash = stem.find('-')?;
    let (digits, rest) = (&stem[..dash], &stem[dash + 1..]);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let (middle, key_digits) = peel_key(rest)?;
    Some((digits, middle, key_digits))
}

/// Read a terminal canonical key token. Callers still validate the title when
/// using this as a full handle rather than a bare key reference.
#[must_use]
pub fn terminal_key(reference: &str) -> Option<Key> {
    let (_, digits) = peel_key(reference)?;
    parse_key(digits)
}

/// Parse the shared canonical key digits, independently of the preceding title.
fn parse_key(digits: &str) -> Option<Key> {
    if digits.starts_with('0') || digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok().map(Key::new)
}

fn render_key(f: &mut fmt::Formatter<'_>, key: Key) -> fmt::Result {
    write!(f, "{KEY_MARK}{}", key.get())
}

/// Peel the final `-k<digits>` token; `parse_key` validates its digits.
fn peel_key(text: &str) -> Option<(&str, &str)> {
    let digits_start = text.len() - text.bytes().rev().take_while(u8::is_ascii_digit).count();
    if digits_start == text.len() {
        return None; // no trailing digits → no key
    }
    let before = text[..digits_start].strip_suffix(KEY_MARK)?;
    Some((before, &text[digits_start..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_files_and_slugless_directories_are_canonical() {
        for (name, found) in [
            ("_BRIEF.md", Found::File),
            ("_topic-k9.md", Found::File),
            ("02-k7", Found::Dir),
        ] {
            assert_eq!(entry(name, found).to_string(), name);
        }
        for (name, found) in [
            ("_", Found::File),
            ("_Topic.md", Found::File),
            ("_topic.txt", Found::File),
            ("01-topic-k7", Found::Dir),
            ("01-k0", Found::Dir),
            ("01-k07", Found::Dir),
            ("01-broken", Found::Dir),
            ("01-impl--topic-k0.md", Found::File),
        ] {
            malformed(name, found);
        }
    }

    #[test]
    fn every_level_requires_a_correctly_placed_node_file() {
        assert!(TaskName::validate_distinguished(None, &[]).is_err());
        let root = entry("_BRIEF.md", Found::File);
        let title = entry("_topic.md", Found::File);
        let node = entry("02-k7", Found::Dir);
        assert!(TaskName::validate_distinguished(None, std::slice::from_ref(&root)).is_ok());
        assert!(
            TaskName::validate_distinguished(Some(&node), std::slice::from_ref(&title)).is_ok()
        );
        assert!(TaskName::validate_distinguished(None, std::slice::from_ref(&title)).is_err());
        assert!(TaskName::validate_distinguished(Some(&node), &[root, title]).is_err());
    }

    /// A [`Kind`] for a test that needs one, by its label.
    ///
    /// A kind is an **open token** since `open-kind-k20`, so a test names the token
    /// it means rather than a variant, and an invalid one is a test bug that panics
    /// here rather than a compile error somewhere else.
    fn a_kind(label: &str) -> Kind {
        Kind::new(label).expect("a test kind must be well-formed")
    }
    use ordinal_fs_tree::conformance;

    fn slug(s: &str) -> Slug {
        Slug::new(s).expect("a well-formed slug")
    }

    fn verdict(name: &str, found: Found) -> Verdict<TaskName, TaskNameError> {
        TaskName::parse(name, found)
    }

    #[track_caller]
    fn entry(name: &str, found: Found) -> TaskName {
        match verdict(name, found) {
            Verdict::Entry(parsed) => parsed,
            other => panic!("{name:?} was not an entry: {other:?}"),
        }
    }

    #[track_caller]
    fn malformed(name: &str, found: Found) -> TaskNameError {
        match verdict(name, found) {
            Verdict::Malformed(error) => error,
            other => panic!("{name:?} was not malformed: {other:?}"),
        }
    }

    // ---- the conformance kit ------------------------------------------------

    /// Every shape a real `.grove/` holds, in the proportions one holds them:
    /// the charter, a live leaf, both terminal marks, a node directory and a
    /// foreign `README.md` — then the two near-misses the grammar refuses.
    ///
    /// **The last two lines are not shapes a healthy tree holds, and the first
    /// of them is what poses canonicity at all.** The check is
    /// `format(parse(f)) == f` over the filenames handed in, so a grammar that
    /// accepts `5-…` and renders `05-…` is caught only when handed a `5-…`
    /// *that parses*; the canonical listings render back as themselves, so the
    /// kit would report conforming rather than unexercised. Measured, not
    /// reasoned: removing the seven canonicity lines from `parse` leaves the kit
    /// green without `5-impl--domain-k29.md` and red with it
    /// (`docs/formalism-findings.md` entry 020). `07-DONE-grove-flip-k28` is
    /// malformed directory syntax: a near-miss for the crate, never for the kit.
    fn listings() -> Vec<(&'static str, Found)> {
        vec![
            ("_BRIEF.md", Found::File),
            ("01-DONE-requirements--plan-k1.md", Found::File),
            ("02-impl--domain-k29.md", Found::File),
            ("03-ABANDONED-design--refusals-k30.md", Found::File),
            ("07-k28", Found::Dir),
            ("README.md", Found::File),
            ("5-impl--domain-k29.md", Found::File),
            ("07-DONE-grove-flip-k28", Found::Dir),
        ]
    }

    fn triples() -> Vec<(Ordinal, Key, Parts)> {
        vec![
            (
                Ordinal::new(1),
                Key::new(1),
                Parts::leaf(Outcome::Live, a_kind("impl"), slug("domain")),
            ),
            (Ordinal::new(2), Key::new(28), Parts::node()),
        ]
    }

    fn level_samples() -> Vec<conformance::LevelSample<TaskName>> {
        let root = TaskName::Brief;
        let title = TaskName::NodeFile(slug("topic"));
        let other = TaskName::NodeFile(slug("other"));
        let node = TaskName::compose(Ordinal::FIRST, Key::new(1), Parts::node());
        vec![
            (None, vec![], false),
            (None, vec![root.clone()], true),
            (None, vec![title.clone()], false),
            (None, vec![root.clone(), title.clone()], false),
            (Some(node.clone()), vec![], false),
            (Some(node.clone()), vec![title.clone()], true),
            (Some(node.clone()), vec![root], false),
            (Some(node), vec![title, other], false),
        ]
        .into_iter()
        .map(|(node, distinguished, accepted)| conformance::LevelSample {
            node,
            distinguished,
            accepted,
        })
        .collect()
    }

    #[test]
    fn the_task_tree_domain_conforms() {
        conformance::check::<TaskName>(
            &listings(),
            &triples(),
            &[TaskName::Brief],
            &level_samples(),
        )
        .assert_conforming();
    }

    /// The kit's canonicity check reparses what the domain composes, so a kind
    /// whose token does not survive the round trip is a defect it would catch —
    /// but only for the kinds the fixture happens to compose.
    ///
    /// **There is no set to sweep any more** (`open-kind-k20`), so the fixture is
    /// over the *shapes* a token can have rather than over nineteen labels: one
    /// word, two words, three words, a pair sharing a two-word prefix, digits,
    /// and a single character. Those are the shapes that could break the round
    /// trip — the `--` has to be found in the right place and the token read
    /// back whole — and a token nobody has ever configured is in the list
    /// deliberately, because the grammar must not be able to tell.
    #[test]
    fn every_shape_of_session_kind_survives_the_round_trip() {
        let kinds = [
            "impl",
            "research-a",
            "review-impl",
            "integrate-review-impl",
            "integrate-review-prototype",
            "postmortem-2",
            "x",
            "9",
        ];
        let triples: Vec<_> = kinds
            .into_iter()
            .enumerate()
            .flat_map(|(index, label)| {
                let ordinal = Ordinal::new(u32::try_from(index).unwrap() + 1);
                let key = Key::new(u32::try_from(index).unwrap() + 1);
                // The outcome infix is part of the name a kind renders into, and
                // `DONE-`/`ABANDONED-` are stripped before the kind is read.
                [Outcome::Live, Outcome::Done, Outcome::Abandoned]
                    .into_iter()
                    .map(move |outcome| {
                        (
                            ordinal,
                            key,
                            Parts::leaf(outcome, a_kind(label), slug("a-slug-9")),
                        )
                    })
            })
            .collect();
        conformance::check::<TaskName>(&listings(), &triples, &[TaskName::Brief], &level_samples())
            .assert_conforming();
    }

    /// The other half of the same claim, and the one that would have been a
    /// *compile* error before: a token no methodology has ever declared parses,
    /// because the grammar checks shape and holds no set to fail membership in.
    #[test]
    fn a_kind_no_methodology_declares_still_parses() {
        for label in ["wrok", "work", "postmortem", "spike", "impl2"] {
            let name = format!("01-{label}--a-k1.md");
            assert_eq!(
                entry(&name, Found::File),
                TaskName::Positioned {
                    ordinal: Ordinal::new(1),
                    key: Key::new(1),
                    parts: Parts::leaf(Outcome::Live, a_kind(label), slug("a")),
                },
                "{name:?}"
            );
        }
    }

    // ---- classification: the four verdicts ---------------------------------

    #[test]
    fn the_charter_is_the_distinguished_child() {
        assert_eq!(entry("_BRIEF.md", Found::File), TaskName::Brief);
        assert!(matches!(TaskName::Brief.view(), NameView::Distinguished));
    }

    #[test]
    fn a_name_that_is_not_task_shaped_is_foreign() {
        for name in [
            "README.md",
            "notes",
            "impl-a-k1.md", // unpositioned
            ".gitignore",
        ] {
            assert_eq!(verdict(name, Found::File), Verdict::Foreign, "{name:?}");
        }
    }

    // ---- the grammar --------------------------------------------------------

    #[test]
    fn a_live_leaf_parses_and_renders() {
        let name = entry("02-impl--domain-k29.md", Found::File);
        assert_eq!(
            name,
            TaskName::Positioned {
                ordinal: Ordinal::new(2),
                key: Key::new(29),
                parts: Parts::leaf(Outcome::Live, a_kind("impl"), slug("domain")),
            }
        );
        assert_eq!(name.to_string(), "02-impl--domain-k29.md");
    }

    #[test]
    fn both_terminal_marks_parse() {
        for (name, outcome) in [
            ("01-DONE-requirements--plan-k1.md", Outcome::Done),
            ("03-ABANDONED-design--refusals-k30.md", Outcome::Abandoned),
        ] {
            match entry(name, Found::File) {
                TaskName::Positioned {
                    parts: Parts::Leaf { outcome: got, .. },
                    ..
                } => assert_eq!(got, outcome, "{name:?}"),
                other => panic!("{other:?}"),
            }
        }
    }

    #[test]
    fn a_node_directory_parses() {
        assert_eq!(
            entry("07-k28", Found::Dir),
            TaskName::Positioned {
                ordinal: Ordinal::new(7),
                key: Key::new(28),
                parts: Parts::node(),
            }
        );
    }

    /// The key is the *terminal* `-k<digits>`, so a slug that itself ends in one
    /// stays unambiguous.
    #[test]
    fn the_key_is_the_terminal_marker() {
        match entry("05-impl--task-k9-k3.md", Found::File) {
            TaskName::Positioned { key, parts, .. } => {
                assert_eq!(key, Key::new(3));
                assert_eq!(parts.slug().unwrap().as_str(), "task-k9");
            }
            other => panic!("{other:?}"),
        }
    }

    // ---- question 2: the grammar is canonical -------------------------------

    /// The leaf's headline decision, and the one place this domain deliberately
    /// refuses what the withdrawn grammar accepted. A lenient spelling is a
    /// refusal **naming the canonical form** — without that, a hand-edited tree
    /// is unreadable with no stated way back.
    #[test]
    fn a_lenient_position_is_refused_and_the_refusal_names_the_canonical_spelling() {
        for (written, canonical) in [
            ("5-impl--a-k1.md", "05-impl--a-k1.md"),
            ("005-impl--a-k1.md", "05-impl--a-k1.md"),
            ("0100-impl--a-k1.md", "100-impl--a-k1.md"),
            ("7-k2", "07-k2"),
        ] {
            let found = if written.ends_with(".md") {
                Found::File
            } else {
                Found::Dir
            };
            assert_eq!(
                malformed(written, found),
                TaskNameError::NotCanonical {
                    name: written.to_string(),
                    canonical: canonical.to_string(),
                },
                "{written:?}"
            );
            let advice = malformed(written, found).to_string();
            assert!(advice.contains(canonical), "{advice}");
            assert!(advice.contains("Rename it"), "{advice}");
        }
    }

    /// `{:02}` is a minimum width, not an exact one, so the canonical rule is
    /// *zero-padded to at least two digits and no other leading zero*. Three
    /// digits past 99 is a name, not a violation.
    #[test]
    fn a_position_past_ninety_nine_is_canonical_unpadded() {
        let name = entry("100-impl--a-k1.md", Found::File);
        assert_eq!(name.to_string(), "100-impl--a-k1.md");
    }

    /// A number too large to hold is still this domain's name and still
    /// Malformed; what changes is that there is no canonical spelling to offer.
    #[test]
    fn an_unrepresentable_number_is_refused_without_a_suggestion() {
        for name in ["99999999999-impl--a-k1.md", "01-impl--a-k99999999999.md"] {
            let advice = malformed(name, Found::File).to_string();
            assert!(advice.contains("32 bits"), "{advice}");
            assert!(advice.contains(name), "{advice}");
        }
    }

    // ---- refusals inside the shape -----------------------------------------

    /// A task-shaped leaf whose kind is not a well-formed **token** is
    /// Malformed, never Foreign: skipping it is lost work.
    ///
    /// **A shape refusal, and it names the character it refused**
    /// (`open-kind-k20`). It used to be a membership refusal that listed all
    /// nineteen labels — which is why `01-wrok--a-k1.md` was the fixture here
    /// and is now in the test above, parsing. What is left to refuse is what
    /// cannot be written and read back: an empty token, and one carrying a
    /// character the grammar does not spell.
    #[test]
    fn a_session_kind_that_is_not_a_token_is_malformed() {
        for (name, token, expected) in [
            // The degenerate kind: a separator with nothing before it.
            ("01---a-k1.md", "", "may not be empty"),
            ("01-Impl--a-k1.md", "Impl", "'I'"),
            ("01-DONE-my_kind--a-k1.md", "my_kind", "'_'"),
            ("01-impl.rs--a-k1.md", "impl.rs", "'.'"),
        ] {
            let error = malformed(name, Found::File);
            let TaskNameError::BadKind {
                name: reported,
                kind,
                error: token_error,
            } = &error
            else {
                panic!("{name:?} should be a bad-kind refusal, got {error:?}")
            };
            assert_eq!(reported, name, "{name:?}");
            assert_eq!(kind, token, "{name:?}");
            let advice = error.to_string();
            assert!(advice.contains("malformed Grove leaf"), "{advice}");
            assert!(advice.contains(&format!("{token:?}")), "{advice}");
            assert!(
                token_error.reason.contains(expected),
                "the refusal must name what it refused: {advice}"
            );
            // The advice states the grammar, and states no set — there is none.
            assert!(
                advice.contains("<session-kind>--<slug>-k<key>.md"),
                "{advice}"
            );
        }
    }

    /// **The scenario `grammar-separator-k15` exists to refuse.** A task-shaped
    /// leaf with no `--` is every name the old grammar wrote, so the refusal has
    /// to carry the canonical form and not merely the fact of failure — a tree
    /// written yesterday would otherwise be unreadable with no stated way back
    /// (principle 2: the advice is part of the error).
    ///
    /// The last two are the degenerate ones: a middle that is empty entirely
    /// still carries both marks grove recognises its own names by, so it is
    /// Malformed rather than Foreign — skipping it is lost work, and a whole
    /// subtree of it when the name is a directory.
    #[test]
    fn a_leaf_without_the_separator_is_refused_and_the_refusal_names_the_grammar() {
        for name in [
            "01-impl-a-k1.md",
            "01-DONE-design-decomposition-k2.md",
            "02-integrate-review-design-module-decomposition-k4.md",
            "01-a-k1.md",
            "01--k1.md",
        ] {
            let error = malformed(name, Found::File);
            assert_eq!(
                error,
                TaskNameError::MissingSeparator {
                    name: name.to_string()
                },
                "{name:?}"
            );
            let advice = error.to_string();
            assert!(advice.contains(name), "{advice}");
            assert!(
                advice.contains("NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md"),
                "{advice}"
            );
            assert!(advice.contains("rename it"), "{advice}");
        }
    }

    /// The spec's own round-trip scenario, by name: *a multi-word kind beside a
    /// multi-word slug* (`docs/specs/module-decomposition.md`, requirement *a
    /// leaf filename has exactly one reading*). This is the name that had four
    /// readings under the old grammar and has one under this one, and the rival
    /// splits are spelled out so the assertion is about *which* reading, not
    /// merely that some reading happened.
    #[test]
    fn a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading() {
        let filename = "04-integrate-review-design--module-decomposition-k5.md";
        let name = entry(filename, Found::File);
        assert_eq!(
            name,
            TaskName::Positioned {
                ordinal: Ordinal::new(4),
                key: Key::new(5),
                parts: Parts::leaf(
                    Outcome::Live,
                    a_kind("integrate-review-design"),
                    slug("module-decomposition"),
                ),
            }
        );
        assert_eq!(name.to_string(), filename);
        // The old spelling — no separator at all — is the one that had four
        // readings, and it now has none: there is nothing to match the middle
        // against since `open-kind-k20`, so it is refused rather than guessed at.
        assert!(
            !matches!(
                verdict(
                    "04-integrate-review-design-module-decomposition-k5.md",
                    Found::File
                ),
                Verdict::Entry(_)
            ),
            "a name with no separator must not parse"
        );
        // Moving the separator does **not** give a rival reading of this name;
        // it gives a *different name*, which parses to different parts and
        // renders back to itself. That is the requirement — *a leaf filename has
        // exactly one reading* — and not *only one placement of `--` is legal*.
        // The distinction was invisible while the kind set was closed, because
        // `integrate-review` was not a kind and the name simply failed; with an
        // open token it is one, and the property still holds because the two
        // spellings are two files.
        let moved = "04-integrate-review--design-module-decomposition-k5.md";
        let other = entry(moved, Found::File);
        assert_eq!(
            other,
            TaskName::Positioned {
                ordinal: Ordinal::new(4),
                key: Key::new(5),
                parts: Parts::leaf(
                    Outcome::Live,
                    a_kind("integrate-review"),
                    slug("design-module-decomposition"),
                ),
            }
        );
        assert_ne!(other, name, "two filenames must never name one entry");
        assert_eq!(other.to_string(), moved);
    }

    /// A directory wearing an outcome infix keeps the diagnostic it has today,
    /// wording included: it is one of the better error messages in the codebase
    /// and it names the real damage.
    #[test]
    fn a_node_wearing_an_outcome_infix_is_malformed() {
        for name in ["07-DONE-k28", "07-ABANDONED-k28"] {
            let advice = malformed(name, Found::Dir).to_string();
            assert!(advice.contains(name), "{advice}");
            assert!(advice.contains("NN-k<key>"), "{advice}");
        }
    }

    #[test]
    fn a_slug_the_grammar_cannot_read_back_is_malformed() {
        for (name, found, bad) in [
            ("01-impl--Domain-k1.md", Found::File, "Domain"),
            ("01-impl--a_b-k1.md", Found::File, "a_b"),
            ("01-impl--a--b-k1.md", Found::File, "a--b"),
            ("_.md", Found::File, ""),
            ("_DONE.md", Found::File, "DONE"),
        ] {
            match malformed(name, found) {
                TaskNameError::BadSlug { slug, .. } => assert_eq!(slug, bad, "{name:?}"),
                other => panic!("{name:?}: {other:?}"),
            }
        }
    }

    /// The species half of the obligation, both ways round. A directory wearing
    /// a leaf's name and a file wearing a node's are each a malformed *tree*,
    /// not a foreign entry — the library can see the contradiction and has no
    /// domain error to report it with, so the judgement lives here.
    #[test]
    fn a_species_mismatch_is_malformed_in_both_directions() {
        for (name, found, declares) in [
            ("02-impl--domain-k29.md", Found::Dir, Species::Leaf),
            ("07-k28", Found::File, Species::Node),
            ("_BRIEF.md", Found::Dir, Species::Distinguished),
            ("02-impl--domain-k29.md", Found::Other, Species::Leaf),
        ] {
            assert_eq!(
                malformed(name, found),
                TaskNameError::SpeciesMismatch {
                    name: name.to_string(),
                    declares,
                    found,
                },
                "{name:?} under {found}"
            );
        }
    }

    // ---- the slug rule ------------------------------------------------------

    #[test]
    fn the_slug_rule_is_the_one_grove_already_had() {
        for good in ["a", "domain", "grove-flip", "h3-probe", "k29", "9"] {
            assert!(Slug::new(good).is_ok(), "{good:?}");
        }
        for bad in [
            "",
            "-a",
            "a-",
            // The separator, which the kind/slug boundary owns. The split is
            // still unambiguous with one inside a slug — it takes the *first* —
            // but the spec's rule is that neither token carries one, and a slug
            // that did would leave `BadKind` quoting a token nobody wrote.
            "a--b",
            "--",
            "A",
            "a_b",
            "a.b",
            "a/b",
            "BRIEF",
            "DONE",
            "ABANDONED",
        ] {
            assert!(Slug::new(bad).is_err(), "{bad:?}");
        }
    }

    // ---- the handle owns the grammar ----------------------------------------

    /// Leaf renderings end in their handle, followed by the file suffix.
    #[test]
    fn every_leaf_name_ends_in_its_own_handle() {
        let names = [
            TaskName::compose(
                Ordinal::new(5),
                Key::new(14),
                Parts::leaf(Outcome::Live, a_kind("impl"), slug("name-ownership")),
            ),
            TaskName::compose(
                Ordinal::new(1),
                Key::new(3),
                Parts::leaf(Outcome::Done, a_kind("design"), slug("decomposition")),
            ),
            TaskName::compose(
                Ordinal::new(100),
                Key::new(1),
                Parts::leaf(Outcome::Abandoned, a_kind("finish"), slug("a")),
            ),
        ];
        for name in names {
            let handle = Handle::of_leaf(&name).expect("a positioned name has a handle");
            let rendered = name.to_string();
            let tail = rendered.strip_suffix(".md").unwrap_or(&rendered);
            assert!(
                tail.ends_with(&handle.to_string()),
                "{rendered:?} does not end in its handle {handle}"
            );
            // And the handle read back out of that tail is the same handle, so
            // the terminal substring is not merely a suffix by coincidence.
            assert_eq!(
                Handle::parse(&tail[tail.len() - handle.to_string().len()..]),
                Ok(handle)
            );
        }
    }

    /// The charter is the one name with no key, and therefore no identity of its
    /// own — `of` says so rather than inventing one.
    #[test]
    fn the_brief_has_no_handle() {
        assert_eq!(Handle::of_leaf(&TaskName::Brief), None);
    }

    /// `parse` is the inverse of the rendering, including across the slug that
    /// contains the key marker.
    #[test]
    fn a_handle_round_trips_through_its_own_rendering() {
        for (text, expect_slug, expect_key) in [
            ("name-ownership-k14", "name-ownership", 14u32),
            ("a-k1", "a", 1),
            ("migrate-v1-to-v2-k27", "migrate-v1-to-v2", 27),
            // The terminal rule: the *last* `-k<digits>` is the key, so a slug
            // may carry one. This is the fact `split_shape` and `Handle::parse`
            // now share a single peel to guarantee.
            ("task-k9-k3", "task-k9", 3),
        ] {
            let handle = Handle::parse(text).expect("a well-formed handle");
            assert_eq!(handle.slug().as_str(), expect_slug);
            assert_eq!(handle.key().get(), expect_key);
            assert_eq!(handle.to_string(), text);
        }
    }

    /// A handle and a filename find the key by one rule. Asserted over the pair
    /// rather than over either alone, because the failure this replaces was two
    /// implementations agreeing on the easy cases.
    #[test]
    fn a_handle_and_a_filename_peel_the_same_key() {
        for (filename, handle_text) in [
            ("05-impl--task-k9-k3.md", "task-k9-k3"),
            ("01-DONE-design--decomposition-k2.md", "decomposition-k2"),
        ] {
            let found = if filename.ends_with(".md") {
                Found::File
            } else {
                Found::Dir
            };
            let name = entry(filename, found);
            let from_name = Handle::of_leaf(&name).expect("a positioned name has a handle");
            let from_text = Handle::parse(handle_text).expect("a well-formed handle");
            assert_eq!(from_name, from_text, "{filename:?} vs {handle_text:?}");
        }
    }

    /// Every refusal names what it was handed and what a handle is, which is the
    /// error model the rest of this design follows.
    #[test]
    fn a_refused_handle_says_what_it_should_have_been() {
        let not_shaped = Handle::parse("build").expect_err("not handle-shaped");
        assert_eq!(
            not_shaped,
            HandleError::NotHandleShaped {
                text: "build".to_string()
            }
        );
        assert!(not_shaped.to_string().contains("<slug>-k<key>"));

        // Trailing digits without the marker are not a handle either.
        assert!(matches!(
            Handle::parse("build-14"),
            Err(HandleError::NotHandleShaped { .. })
        ));

        let wide = Handle::parse("a-k99999999999").expect_err("key too wide");
        assert!(matches!(wide, HandleError::BadKey { .. }));
        assert!(wide.to_string().contains("99999999999"));

        let bad = Handle::parse("Bad-Slug-k2").expect_err("not a slug");
        assert!(matches!(bad, HandleError::BadSlug { .. }));
        assert!(bad.to_string().contains("lowercase"));

        // The empty slug: both of the grammar's markers, nothing between them.
        assert!(matches!(
            Handle::parse("-k3"),
            Err(HandleError::BadSlug { .. })
        ));
    }

    #[test]
    fn handles_require_canonical_keys_and_slugs() {
        for text in ["a-k007", "a-k0", "-k3", "A-k3", "DONE-k3", "a_b-k3"] {
            assert!(Handle::parse(text).is_err(), "{text}");
        }
    }

    #[test]
    fn node_handles_require_a_directory_and_its_titled_file() {
        let node = entry("07-k2", Found::Dir);
        let file = entry("_migrate-k9-to-k10.md", Found::File);
        let leaf = entry("01-impl--work-k3.md", Found::File);
        let root = TaskName::Brief;
        assert_eq!(
            Handle::of_node(&node, &file).unwrap().to_string(),
            "migrate-k9-to-k10-k2"
        );
        for name in [&node, &file, &root] {
            assert!(Handle::of_leaf(name).is_none());
        }
        for first in [&node, &file, &leaf, &root] {
            for second in [&node, &file, &leaf, &root] {
                assert_eq!(
                    Handle::of_node(first, second).is_some(),
                    first == &node && second == &file
                );
            }
        }
    }
}

// Grove's implementation of `ordinal_fs_tree::EntryName` — the whole seam
// between the task tree and the library that drives it (gh issue #13,
// increment 2).
//
// **This is the only grammar grove has.** It was written in the *expand* stage
// against the library's conformance kit while grove's own path-walking name
// model was still live, each verb group moved onto it in its own leaf through
// the *migrate* stage, and `sweep-k37` deleted the other side. So there is no
// longer a call site whose `use` line has to be read to know which model it
// means, and the two-grammar hazards this header used to enumerate are history
// (`docs/ARCHITECTURE.md`, *The withdrawn tree algebra*).
//
// The three on-disk shapes, as `grammar-separator-k15` left them — that leaf
// put the `--` between a leaf's session kind and its slug and renamed every
// entry in this repo's own tree onto it, in the same session as the release
// that can read it:
//
//     leaf       NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md
//     node dir   NN-<slug>-k<key>
//     brief      BRIEF.md                     (the containing node's charter)
//
// **The grammar is canonical, and that was the departure from the model it
// replaced.** The withdrawn one was deliberately lenient on padding — it
// accepted a hand-typed `5` and rendered `05` — so `format(parse(f)) == f`
// failed there and one entry could occupy two files, sharing a key and a
// position. That is the library's *canonicity* obligation broken, and
// `docs/ordinal-fs-tree/models/structure.als` draws the picture under
// `witness_two_filenames_name_one_entry`. Here a lenient spelling is a refusal
// that names the spelling grove writes. The decision, its cost and the
// alternative are `docs/adr/task-names-are-canonical.md`.
//
// A second, smaller departure, and the reason no caller may hand this a path:
// the withdrawn parser tolerated a trailing `/` on a node name for callers
// passing one. A `parse` fed by a directory listing never sees one, and
// tolerating it would be a second spelling of one name — exactly what
// canonicity forbids. Trimming a caller's argument is the caller's job.
//
// The classification is where a name grammar loses data, so it is where the care
// goes. `Verdict` has four outcomes and the load-bearing split is between two of
// them: `Foreign` is skipped **recursively**, taking a whole subtree with it when
// the name is a directory, while `Malformed` and `Reserved` halt. So:
//
//   - `BRIEF.md`                              -> the distinguished child
//   - `NN-…-k<key>[.md]`                      -> an entry, or `Malformed` if it
//                                                does not parse completely
//   - `README.md`, anything else              -> `Foreign`
//
// **The handle is part of this grammar, not a second one** (`name-ownership-k14`,
// `docs/specs/module-decomposition.md` decision 4). `<slug>-k<key>` — the
// position-free identity that crosses every module boundary, from the store that
// produces it, through the prompt, to the verbs a session hands it back to — was
// spelled by four `format!`s outside this file and by both arms of the renderer
// inside it, and peeled by `split_shape` here and by `task_tree::handle_key`
// there, whose own comment conceded it *"mirrors the filename grammar"*. None of
// them was behind a type.
// It is now [`Handle`], and the ownership is structural rather than
// disciplinary: [`Handle::render`] is the only `write!` the grammar appears in,
// [`peel_key`] the only place it is taken apart, and **both of [`TaskName`]'s
// renderings end in a call to the former**. So a filename and a handle saying
// different things is not a bug this module can have — it is not expressible.
//
// The same fact read the other way: the handle is a **contiguous terminal
// substring** of every name that has one, a leaf's followed only by the `.md`
// its species takes. That is the property `grammar-separator-k15` bought with
// its rename, and with one renderer it cost that leaf one `write!` and one
// `split_once`.

use core::fmt;

use ordinal_fs_tree::{
    EntryName, Found, Key, NameView, Ordinal, PositionedSpecies, Species, Triple, Verdict,
};

use crate::leaf::Kind;

/// The name of a node's distinguished child: the charter every node directory is
/// headed by.
pub const BRIEF: &str = "BRIEF.md";

/// The permanent key's delimiter — the terminal `-k<digits>` of every positioned
/// name (task-tree-scheme, amending the original `[<key>]`: brackets are
/// shell-glob metacharacters and `-k` is glob-safe).
const KEY_MARK: &str = "-k";

/// The separator between a leaf's session kind and its slug
/// (`grammar-separator-k15`, `docs/specs/module-decomposition.md` decision 3).
///
/// A single `-` cannot delimit them: both tokens are hyphenated words, so
/// `design-decomposition` reads as kind `design` + slug `decomposition` **and**
/// as kind `design-decomposition` + empty slug. Only matching the middle against
/// the closed kind set resolves that today — the very thing `open-kind-k20`
/// removes — so one filename would name two entries, differing in the *handle*.
/// The middle splits at the **first** `--`; neither token may contain one, which
/// is why [`Slug::new`] refuses it and why no kind label carries it.
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

/// Why a string is not a well-formed [`Slug`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlugError {
    /// What is wrong with it, phrased for whoever has to fix the filename.
    pub reason: &'static str,
}

impl fmt::Display for SlugError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.reason)
    }
}

impl std::error::Error for SlugError {}

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
    /// Returns [`SlugError`] for anything the grammar cannot render and read
    /// back: an empty string, a leading or trailing hyphen, a character outside
    /// lowercase ASCII, digits and hyphens, or one of the reserved words the
    /// grammar's own markers use.
    pub fn new(slug: &str) -> Result<Self, SlugError> {
        let reason = |reason| Err(SlugError { reason });
        if slug.is_empty() {
            return reason("a slug may not be empty");
        }
        if matches!(slug, "BRIEF" | "DONE" | "ABANDONED") {
            return reason(
                "`BRIEF`, `DONE` and `ABANDONED` are reserved: the grammar's own markers",
            );
        }
        if slug.starts_with('-') || slug.ends_with('-') {
            return reason("a slug may not start or end with a dash");
        }
        if slug.contains(SEPARATOR) {
            return reason(
                "a slug may not contain `--`: that is the separator between the session kind \
                 and the slug",
            );
        }
        if !slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return reason("a slug holds lowercase ASCII letters, digits and dashes only");
        }
        Ok(Self(slug.to_string()))
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
    /// A terminal key that does not fit in 32 bits, so there is no key to name.
    KeyOutOfRange {
        /// What was handed in.
        text: String,
        /// The digit run that overflowed.
        digits: String,
    },
    /// A terminal key preceded by something that is not a slug.
    BadSlug {
        /// What was handed in.
        text: String,
        /// The offending slug.
        slug: String,
        /// Why it is not a slug.
        error: SlugError,
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
            Self::KeyOutOfRange { text, digits } => write!(
                f,
                "{text:?} is not a Grove handle: the key {digits:?} does not fit in 32 bits. \
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

/// The permanent, position-free identity of a work item: `<slug>-k<key>`.
///
/// **This type owns the `<slug>-k<key>` grammar, and it is the only thing that
/// spells it.** [`Handle::render`] is the single `write!` the grammar appears
/// in, and both of [`TaskName`]'s renderings end in a call to it — so the
/// filename and the handle cannot drift, because saying two different things is
/// not expressible. That is the *structural* form of `one type owns a name`
/// (`docs/specs/module-decomposition.md`, decision 4); the disciplinary form —
/// a rule a review has to hold — is what the six hand-rolled sites this type
/// replaced showed does not hold.
///
/// It is also why the handle is a **contiguous terminal substring** of every
/// name that has one. That property is what `grammar-separator-k15` bought, and
/// with the grammar in one function that leaf was an edit to [`render`]'s
/// caller rather than a rewrite — the separator sits *before* the handle, never
/// inside it, so [`render`] itself did not change at all.
///
/// [`render`]: Handle::render
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handle {
    slug: Slug,
    key: Key,
}

impl Handle {
    /// The handle of a slug and the key the tree allocated for it.
    #[must_use]
    pub const fn new(slug: Slug, key: Key) -> Self {
        Self { slug, key }
    }

    /// The handle of a positioned name.
    ///
    /// `None` for the charter brief, which is the one name in the grammar with
    /// no key — and therefore no identity of its own, its subject being the node
    /// that contains it.
    #[must_use]
    pub fn of(name: &TaskName) -> Option<Self> {
        match name {
            TaskName::Brief => None,
            TaskName::Positioned { key, parts, .. } => Some(Self::new(parts.slug().clone(), *key)),
        }
    }

    /// Read a handle back out of its rendering.
    ///
    /// The inverse of [`Handle::render`], and the *only* peel of the terminal
    /// `-k<digits>` outside [`split_shape`], which shares [`peel_key`] with it —
    /// so a handle and a filename find the key by one rule and cannot disagree.
    ///
    /// **Deliberately lenient on the key's spelling where [`TaskName::parse`] is
    /// canonical, and the asymmetry is the point.** Canonicity exists because
    /// two spellings of one *filename* are two files on disk sharing one key and
    /// one position (`docs/adr/task-names-are-canonical.md`); a handle is never
    /// on disk, so that argument does not reach it. It is a **reference**
    /// namespace — typed by a human at `resolve` and at `finish-commit` — and
    /// `parse_ref` is already lenient beside it, taking a bare `007` for key 7.
    /// So `a-k007` is key 7 here, exactly as the `task_tree::handle_key` this
    /// replaced had it, and `Handle::parse(x).to_string() == x` holds only for
    /// what [`Handle::render`] writes.
    ///
    /// **It is stricter than the deleted `task_tree::handle_key` on the slug**,
    /// which that function did not look at — and that is why `resolve`'s
    /// fallback asks [`terminal_key`] instead. This is the *handle* question,
    /// asked where a handle is genuinely meant: `finish-commit`'s argument. A
    /// caller who only wants the key a reference ends in must not ask it here,
    /// or an operator pasting `01-DONE-impl--build-k5` gets a refusal for a head
    /// that was never going to be a slug.
    ///
    /// # Errors
    ///
    /// Returns [`HandleError`] when there is no terminal `-k<digits>`, when the
    /// key does not fit in 32 bits, or when what precedes the key is not a
    /// [`Slug`].
    pub fn parse(text: &str) -> Result<Self, HandleError> {
        let Some((before, digits)) = peel_key(text) else {
            return Err(HandleError::NotHandleShaped {
                text: text.to_string(),
            });
        };
        let Ok(key) = digits.parse::<u32>() else {
            return Err(HandleError::KeyOutOfRange {
                text: text.to_string(),
                digits: digits.to_string(),
            });
        };
        match Slug::new(before) {
            Ok(slug) => Ok(Self::new(slug, Key::new(key))),
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
        write!(f, "{slug}{KEY_MARK}{}", key.get())
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
    /// A node directory: children, headed by a `BRIEF.md` charter.
    Node {
        /// Its human-facing name.
        slug: Slug,
    },
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
    pub const fn node(slug: Slug) -> Self {
        Self::Node { slug }
    }

    /// The slug, whichever variant this is.
    #[must_use]
    pub const fn slug(&self) -> &Slug {
        match self {
            Self::Leaf { slug, .. } | Self::Node { slug } => slug,
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
            Self::Node { .. } => PositionedSpecies::Node,
        }
    }
}

/// A task tree entry's name.
///
/// The two variants are the whole of the type: a name carries an ordinal, a key
/// and parts **together**, or it is the charter brief and carries none of them.
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
    /// `BRIEF.md` — the containing node's charter.
    Brief,
}

impl fmt::Display for TaskName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Brief => f.write_str(BRIEF),
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
                    Parts::Node { slug } => {
                        write!(f, "{ordinal:02}-")?;
                        Handle::render(f, slug, *key)
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
    /// A task-shaped leaf whose session-kind token is not one of the closed set.
    UnknownKind {
        /// What is on disk.
        name: String,
        /// The token that sat before the separator.
        kind: String,
    },
    /// A node directory wearing an outcome infix.
    NodeWearsOutcome {
        /// What is on disk.
        name: String,
    },
    /// A slug the grammar cannot render and read back.
    BadSlug {
        /// What is on disk.
        name: String,
        /// The offending slug.
        slug: String,
        /// Why it is not a slug.
        error: SlugError,
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
            Self::UnknownKind { name, kind } => write!(
                f,
                "malformed Grove leaf {name:?}: {kind:?} is not a session kind. Expected \
                 NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md with session kind \
                 one of {}",
                Kind::label_list()
            ),
            Self::NodeWearsOutcome { name } => write!(
                f,
                "malformed Grove node directory {name:?}: expected NN-<slug>-k<key>. A node \
                 is never marked DONE or ABANDONED — its done-ness is the absence of a live \
                 leaf in its subtree — so an outcome infix on a directory hides every leaf \
                 under it. Drop the infix to restore the subtree, or rename the directory \
                 out of the task-shaped grammar if it is not Grove's."
            ),
            Self::BadSlug { name, slug, error } => write!(
                f,
                "malformed Grove task name {name:?}: the slug {slug:?} is not one — \
                 {error}. Rename it with a slug that is."
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
                 under it.",
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
        if name == BRIEF {
            return match disagreement(Species::Distinguished, found, name) {
                Some(error) => Verdict::Malformed(error),
                None => Verdict::Entry(Self::Brief),
            };
        }
        // The `.md` suffix is what the name *declares* its species to be.
        let (stem, declares_leaf) = match name.strip_suffix(".md") {
            Some(stem) => (stem, true),
            None => (name, false),
        };

        // Is this name Grove's at all? A name is Grove's when it is **positioned
        // and keyed** — a leading digit run, and a terminal `-k<digits>`. That
        // shape is the one only Grove's grow verbs write, so everything else is
        // Foreign and skipped, which is safe precisely because we are
        // disclaiming it. A stray `README.md` lands here.
        // Everything that *is* this shape and does not parse is Malformed,
        // whichever species it declares: a task-shaped name Grove skips is lost
        // work, and a whole subtree when the name is a directory.
        let Some((digits, middle, key_digits)) = split_shape(stem) else {
            return Verdict::Foreign;
        };

        let Ok(ordinal) = digits.parse::<u32>() else {
            return Verdict::Malformed(uncomputable_canonical(name));
        };
        let Ok(key) = key_digits.parse::<u32>() else {
            return Verdict::Malformed(uncomputable_canonical(name));
        };

        // The outcome infix sits immediately after the position, and is admitted
        // here for *both* species precisely so that a directory wearing one is
        // reported rather than skipped.
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
            let Some(kind) = Kind::from_label(kind_token) else {
                return Verdict::Malformed(TaskNameError::UnknownKind {
                    name: name.to_string(),
                    kind: kind_token.to_string(),
                });
            };
            match Slug::new(slug) {
                Ok(slug) => Parts::leaf(outcome, kind, slug),
                Err(error) => return Verdict::Malformed(bad_slug(name, slug, error)),
            }
        } else {
            if outcome != Outcome::Live {
                return Verdict::Malformed(TaskNameError::NodeWearsOutcome {
                    name: name.to_string(),
                });
            }
            match Slug::new(after_outcome) {
                Ok(slug) => Parts::node(slug),
                Err(error) => return Verdict::Malformed(bad_slug(name, after_outcome, error)),
            }
        };

        let parts_species = parts.species();
        let parsed = Self::Positioned {
            ordinal: Ordinal::new(ordinal),
            key: Key::new(key),
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

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self::Positioned {
            ordinal,
            key,
            parts,
        }
    }

    fn distinguished() -> Option<Self> {
        Some(Self::Brief)
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        match self {
            Self::Brief => NameView::Distinguished,
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

fn bad_slug(name: &str, slug: &str, error: SlugError) -> TaskNameError {
    TaskNameError::BadSlug {
        name: name.to_string(),
        slug: slug.to_string(),
        error,
    }
}

/// A canonicity refusal whose advice cannot be computed: the numbers did not fit
/// in 32 bits, so there is no spelling to offer back.
fn uncomputable_canonical(name: &str) -> TaskNameError {
    TaskNameError::NotCanonical {
        name: name.to_string(),
        canonical: "a name whose position and key both fit in 32 bits".to_string(),
    }
}

/// Split a stem into `(position digits, middle, key digits)`, or `None` when the
/// stem is not task-shaped at all.
///
/// The position is the leading digit run, ended by the first `-` — the position
/// is pure digits, so the first dash is its unambiguous boundary. The key is the
/// *terminal* `-k<digits>`, which is what keeps a slug containing `-k9`
/// unambiguous: `05-impl--task-k9-k3.md` is the slug `task-k9` at key 3.
///
/// The middle is returned unexamined, including when it is empty: `01--k3` has
/// both markers Grove recognises its own names by, with everything between them
/// missing, so it is Malformed rather than Foreign. Disclaiming it would skip the
/// file — and the whole subtree beneath it when it is a directory — while the
/// walk reported a healthy tree.
fn split_shape(stem: &str) -> Option<(&str, &str, &str)> {
    let dash = stem.find('-')?;
    let (digits, rest) = (&stem[..dash], &stem[dash + 1..]);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let (middle, key_digits) = peel_key(rest)?;
    Some((digits, middle, key_digits))
}

/// Peel a terminal `-k<digits>` into what precedes it and the digit run, or
/// `None` when there is none.
///
/// **The only peel of the key in grove**, shared by [`split_shape`] and
/// [`Handle::parse`] — which is what makes *a handle and a filename find the key
/// identically* a fact rather than a claim. It was two functions
/// (`task_tree::handle_key` was the second, and its own comment conceded it
/// "mirrors the filename grammar"), and the terminality rule is subtle enough
/// that two of it is one too many: the key is the **last** `-k<digits>`, so
/// `migrate-v1-to-v2-k27` is key 27 and a slug may contain `-k9` and still read
/// unambiguously.
///
/// The digits are returned unparsed because the two callers disagree about what
/// an over-wide key means — a name says [`TaskNameError::NotCanonical`], a
/// handle says [`HandleError::KeyOutOfRange`] — and that is their judgement, not
/// this function's.
/// The [`Key`] a reference ends in, or `None` when it does not end in one.
///
/// **A narrower question than [`Handle::parse`], asked by the reference
/// namespace and answered by the same peel.** `resolve`'s bare-slug fallback
/// wants *does this end in a key*, not *is this a handle*: an operator pastes a
/// retired leaf's whole stem — `01-DONE-impl--build-k5` — and means key 5, and
/// nothing before the key is a slug there or needs to be. Routing that through
/// `Handle::parse` narrows `resolve` to references whose head happens to be a
/// well-formed slug, which is a change to the verb rather than to the grammar's
/// ownership, and this leaf owns the second and not the first.
///
/// One peel still: this and [`Handle::parse`] both go through [`peel_key`], and
/// the difference between them is what they *require of what precedes it*.
#[must_use]
pub fn terminal_key(reference: &str) -> Option<Key> {
    let (_, digits) = peel_key(reference)?;
    digits.parse().ok().map(Key::new)
}

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
    /// the charter, a live leaf, both terminal marks, a node directory, a
    /// foreign `README.md`, and both transaction sentinels.
    ///
    /// **The last two lines are the load-bearing ones, and they are not shapes a
    /// healthy tree holds.** They are the near-misses the grammar is meant to
    /// refuse, and without them the kit passes a *lenient* domain: its canonicity
    /// check is `format(parse(f)) == f` over the filenames it is handed, so a
    /// grammar that accepts `5-…` and renders `05-…` is only caught when it is
    /// handed a `5-…`. Every other listing parsed, so the kit does not report the
    /// obligation unexercised either — it reports conforming. Measured, not
    /// reasoned: disabling this domain's canonicity check leaves the kit green
    /// without these two entries and red with them
    /// (`docs/formalism-findings.md` entry 020).
    fn listings() -> Vec<(&'static str, Found)> {
        vec![
            ("BRIEF.md", Found::File),
            ("01-DONE-requirements--plan-k1.md", Found::File),
            ("02-impl--domain-k29.md", Found::File),
            ("03-ABANDONED-design--refusals-k30.md", Found::File),
            ("07-grove-flip-k28", Found::Dir),
            ("README.md", Found::File),
            ("5-impl-domain-k29.md", Found::File),
            ("07-DONE-grove-flip-k28", Found::Dir),
        ]
    }

    fn triples() -> Vec<(Ordinal, Key, Parts)> {
        vec![
            (
                Ordinal::new(1),
                Key::new(1),
                Parts::leaf(Outcome::Live, Kind::Impl, slug("domain")),
            ),
            (
                Ordinal::new(2),
                Key::new(28),
                Parts::node(slug("grove-flip")),
            ),
        ]
    }

    /// The leaf's own *Done when*: the kit runs green over a fixture covering
    /// every shape a real `.grove/` holds. It discharges the five obligations
    /// the library assumes and cannot check from inside an operation —
    /// `compose` places what it is given, the grammar is canonical,
    /// `distinguished()` names the only entry of its species, `parse` refuses
    /// what `found` contradicts, and a name renders as one path component.
    #[test]
    fn the_task_tree_domain_conforms() {
        conformance::check::<TaskName>(&listings(), &triples()).assert_conforming();
    }

    /// The kit's canonicity check reparses what the domain composes, so a kind
    /// whose label does not survive the round trip is a defect it would catch —
    /// but only for the kinds the fixture happens to compose. The label set is
    /// closed and nineteen strong, several of them multi-word, and
    /// `Kind::from_label` reads them back by exact equality against the token the
    /// `--` delimits. So every one of them goes through the kit rather than the
    /// two the fixture names.
    #[test]
    fn every_session_kind_survives_the_round_trip() {
        let triples: Vec<_> = Kind::ALL
            .into_iter()
            .enumerate()
            .flat_map(|(index, kind)| {
                let ordinal = Ordinal::new(u32::try_from(index).unwrap() + 1);
                let key = Key::new(u32::try_from(index).unwrap() + 1);
                // The outcome infix is part of the name a kind renders into, and
                // `DONE-`/`ABANDONED-` are stripped before the kind is read.
                [Outcome::Live, Outcome::Done, Outcome::Abandoned]
                    .into_iter()
                    .map(move |outcome| {
                        (ordinal, key, Parts::leaf(outcome, kind, slug("a-slug-9")))
                    })
            })
            .collect();
        conformance::check::<TaskName>(&listings(), &triples).assert_conforming();
    }

    // ---- classification: the four verdicts ---------------------------------

    #[test]
    fn the_charter_is_the_distinguished_child() {
        assert_eq!(entry("BRIEF.md", Found::File), TaskName::Brief);
        assert_eq!(TaskName::distinguished(), Some(TaskName::Brief));
    }

    #[test]
    fn a_name_that_is_not_task_shaped_is_foreign() {
        for name in [
            "README.md",
            "notes",
            "01-k3.md",     // no `-k` key delimiter
            "impl-a-k1.md", // unpositioned
            "01-verbs-k2/", // a path argument's trailing slash is the caller's to trim
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
                parts: Parts::leaf(Outcome::Live, Kind::Impl, slug("domain")),
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
            entry("07-grove-flip-k28", Found::Dir),
            TaskName::Positioned {
                ordinal: Ordinal::new(7),
                key: Key::new(28),
                parts: Parts::node(slug("grove-flip")),
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
                assert_eq!(parts.slug().as_str(), "task-k9");
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
            ("7-verbs-k2", "07-verbs-k2"),
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
            match malformed(name, Found::File) {
                TaskNameError::NotCanonical { canonical, .. } => {
                    assert!(canonical.contains("32 bits"), "{canonical}");
                }
                other => panic!("{other:?}"),
            }
        }
    }

    // ---- refusals inside the shape -----------------------------------------

    /// A task-shaped leaf whose kind is not one of the closed set is Malformed,
    /// never Foreign: skipping it is lost work. The advice is the grammar and
    /// the whole set — and now also the offending token, which is what the
    /// separator makes quotable: before it, an unknown kind and a missing one
    /// were the same failure with no single token to name.
    #[test]
    fn an_unknown_session_kind_is_malformed() {
        for (name, token) in [
            ("01-wrok--a-k1.md", "wrok"),
            ("01-DONE-x-y--a-k1.md", "x-y"),
            // The degenerate kind: a separator with nothing before it.
            ("01---a-k1.md", ""),
        ] {
            let error = malformed(name, Found::File);
            assert_eq!(
                error,
                TaskNameError::UnknownKind {
                    name: name.to_string(),
                    kind: token.to_string(),
                },
                "{name:?}"
            );
            let advice = error.to_string();
            assert!(advice.contains("malformed Grove leaf"), "{advice}");
            assert!(advice.contains(&format!("{token:?}")), "{advice}");
            assert!(advice.contains("`impl`"), "{advice}");
            assert!(advice.contains("`integrate-review-prototype`"), "{advice}");
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
                    Kind::IntegrateReviewDesign,
                    slug("module-decomposition"),
                ),
            }
        );
        assert_eq!(name.to_string(), filename);
        // The old spelling, and the one a mis-placed separator would give: each
        // is now a name that does not parse at all, rather than a rival reading
        // of this one.
        for rival in [
            "04-integrate-review-design-module-decomposition-k5.md",
            "04-integrate-review--design-module-decomposition-k5.md",
        ] {
            assert!(
                !matches!(verdict(rival, Found::File), Verdict::Entry(_)),
                "{rival:?} still parses"
            );
        }
    }

    /// A directory wearing an outcome infix keeps the diagnostic it has today,
    /// wording included: it is one of the better error messages in the codebase
    /// and it names the real damage.
    #[test]
    fn a_node_wearing_an_outcome_infix_is_malformed() {
        for name in ["07-DONE-grove-flip-k28", "07-ABANDONED-grove-flip-k28"] {
            let error = malformed(name, Found::Dir);
            assert_eq!(
                error,
                TaskNameError::NodeWearsOutcome {
                    name: name.to_string()
                },
                "{name:?}"
            );
            let advice = error.to_string();
            assert!(
                advice.contains("malformed Grove node directory"),
                "{advice}"
            );
            assert!(advice.contains("hides every leaf under it"), "{advice}");
            assert!(advice.contains("Drop the infix"), "{advice}");
        }
    }

    #[test]
    fn a_slug_the_grammar_cannot_read_back_is_malformed() {
        for (name, found, bad) in [
            ("01-impl--Domain-k1.md", Found::File, "Domain"),
            ("01-impl--a_b-k1.md", Found::File, "a_b"),
            ("01-impl--a--b-k1.md", Found::File, "a--b"),
            ("01--k1", Found::Dir, ""),
            ("01-BRIEF-k1", Found::Dir, "BRIEF"),
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
            ("07-grove-flip-k28", Found::File, Species::Node),
            ("BRIEF.md", Found::Dir, Species::Distinguished),
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
            // that did would leave `UnknownKind` quoting a token nobody wrote.
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

    /// **The structural claim decision 4 asks for, asserted rather than
    /// reviewed.** Every positioned name's rendering ends in its own handle's
    /// rendering — a node's exactly, a leaf's followed only by the `.md` suffix
    /// its species takes. A second spelling of `<slug>-k<key>` anywhere in
    /// `TaskName`'s `Display` fails this the moment the two disagree, which is
    /// what *drift is not expressible* has to mean if it is not to be a promise.
    #[test]
    fn every_positioned_name_ends_in_its_own_handle() {
        let names = [
            TaskName::compose(
                Ordinal::new(5),
                Key::new(14),
                Parts::leaf(Outcome::Live, Kind::Impl, slug("name-ownership")),
            ),
            TaskName::compose(
                Ordinal::new(1),
                Key::new(3),
                Parts::leaf(Outcome::Done, Kind::Design, slug("decomposition")),
            ),
            TaskName::compose(
                Ordinal::new(100),
                Key::new(1),
                Parts::leaf(Outcome::Abandoned, Kind::Finish, slug("a")),
            ),
            // The slug that contains the key marker: the case terminality
            // exists for.
            TaskName::compose(
                Ordinal::new(7),
                Key::new(2),
                Parts::node(slug("migrate-k9-to-k10")),
            ),
        ];
        for name in names {
            let handle = Handle::of(&name).expect("a positioned name has a handle");
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
        assert_eq!(Handle::of(&TaskName::Brief), None);
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
            ("07-migrate-k9-to-k10-k2", "migrate-k9-to-k10-k2"),
        ] {
            let found = if filename.ends_with(".md") {
                Found::File
            } else {
                Found::Dir
            };
            let name = entry(filename, found);
            let from_name = Handle::of(&name).expect("a positioned name has a handle");
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
        assert!(matches!(wide, HandleError::KeyOutOfRange { .. }));
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

    /// The two ways `parse` departs from the `task_tree::handle_key` it
    /// replaced, pinned because they are the only behaviour this leaf moved.
    ///
    /// Lenient where `handle_key` was, on the key's spelling — a handle is a
    /// reference a human types and never a name on disk, so canonicity has no
    /// argument here. Stricter where `handle_key` looked at nothing, on the
    /// slug — `handle_key` answered *key 3* for three references no entry could
    /// ever wear, since every slug on disk went through `Slug::new`.
    #[test]
    fn parse_is_lenient_on_the_key_and_strict_on_the_slug() {
        for (text, key) in [("a-k007", 7u32), ("a-k0", 0)] {
            assert_eq!(
                Handle::parse(text)
                    .expect("a lenient key spelling")
                    .key()
                    .get(),
                key
            );
        }
        // Not canonical, and deliberately so: the rendering normalises.
        assert_eq!(Handle::parse("a-k007").expect("parses").to_string(), "a-k7");
        // What `handle_key` used to resolve by key and this refuses.
        for text in ["-k3", "A-k3", "DONE-k3", "a_b-k3"] {
            assert!(
                matches!(Handle::parse(text), Err(HandleError::BadSlug { .. })),
                "{text:?} should be refused for its slug"
            );
        }
    }
}

# Reference domain
<!-- book-page id="reference-domain" slice="reference-domain-k13" order="3" -->
[Previous: Name seam](02-name-seam.md) | [Contents](README.md) | [Next: Read path](04-read-path.md)

The reference domain is a complete consumer of the `EntryName` seam. It models a
course syllabus so the library, its tests, and the demonstration CLI use one
concrete vocabulary. It is not a default or a superclass. Another consumer
implements `EntryName` directly and may choose an unrelated grammar.

The two source roots on this page expand into `src/reference.rs` and
`src/conformance.rs`. The first defines the syllabus grammar. The second turns
the seam's semantic assumptions into a reusable test kit.

<!-- fragment «reference-domain-source» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/reference.rs" lines="1-555" parent="source-reference" -->
<!-- insert «reference-vocabulary» -->
<!-- insert «reference-name-and-errors» -->
<!-- insert «reference-parser» -->
<!-- insert «reference-seam-methods» -->
<!-- insert «reference-parser-helpers» -->
<!-- /fragment -->

<!-- fragment «reference-conformance-source» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/conformance.rs" lines="1-667" parent="source-conformance" -->
<!-- insert «conformance-obligations» -->
<!-- insert «conformance-report» -->
<!-- insert «conformance-compose-and-canonical» -->
<!-- insert «conformance-component-and-distinguished» -->
<!-- insert «conformance-found-agreement» -->
<!-- /fragment -->

<a id="syllabus-vocabulary"></a>
## One consumer's vocabulary

A syllabus has two positioned species and one distinguished name:

| Domain value | Filename role | Generic algebra |
|---|---|---|
| `Parts::Lesson { status, label }` | Regular `.md` file | positioned `Leaf` |
| `Parts::Module { label }` | Directory without a suffix | positioned `Node` |
| `SyllabusName::Overview` | `OVERVIEW.md` inside a level | `Distinguished` |
| `Status::Draft` / `Published` | Lesson attribute | opaque part |
| `Label` | Human-facing text | opaque part |

`Label::new` admits lowercase ASCII letters, digits, and hyphens, requires the
first character to be a lowercase letter, and rejects a trailing hyphen. Labels
need not be unique. `Status` is the domain's only attribute and applies only to
lessons. The `Parts` variants carry the species distinction, so the library
learns leaf versus node without learning what a lesson or module means.

The read path uses a variant of the orientation tree. It adds a second
root-level lesson so the walk crosses from a module subtree to a later root
sibling, and omits `foundations` to keep the module small:

```text
OVERVIEW.md
01-published-orientation-i1.md
02-linear-algebra-i2/
  OVERVIEW.md
  01-published-vectors-i5.md
  02-draft-matrices-i6.md
03-draft-assessment-i9.md
```

The leading number is the mutable ordinal. The terminal `i` number is the stable
key. A lesson inserts its status before the label and ends in `.md`; a module
omits both. `OVERVIEW.md` has neither ordinal nor key because it is a level's own
content rather than an ordered child.

<!-- fragment «reference-vocabulary» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/reference.rs" lines="1-210" parent="reference-domain-source" -->
````rust
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
//! **The grammar is strict where grove's own is lenient**, and the difference is
//! the canonicity obligation. `src/tree_id.rs` accepts a hand-typed `5-…` and
//! renders `05-…`, so two filenames name one entry there. Here `5-…` is
//! [`Verdict::Malformed`] with the canonical spelling in the message.

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
````
<!-- /fragment -->

<a id="worked-reference-name"></a>
## One reference name through the seam

The lesson `02-draft-matrices-i6.md` starts as the consumer-owned parts
`Parts::lesson(Status::Draft, Label::new("matrices")?)`, paired with ordinal 2
and key 6. `SyllabusName::compose` forms the positioned name, and its `Display`
arm renders those values as `{ordinal:02}-{status}-{label}-i{key}.md`, producing
the exact filename above.

When the filesystem reports that filename as `Found::File`, `parse` removes the
`.md` suffix and `split_shape` returns the three strings `("02",
"draft-matrices", "6")`. The parser reads the middle token as draft lesson
parts, rebuilds the positioned name, and compares its rendering byte for byte
with the input. That whole-grammar comparison establishes the canonical
spelling rather than accepting another representation of ordinal 2 or key 6.

The parts determine `PositionedSpecies::Leaf`; the positioned name therefore
has `Species::Leaf`, whose required filesystem observation is `Found::File`.
The agreeing observation yields `Verdict::Entry`. Presenting the same owned
filename as a directory or other object yields `Verdict::Malformed` instead.
The conformance kit exercises this boundary as
`ParseRefusesWhatFoundContradicts`: each supplied name is parsed under all three
`Found` values, and every contradiction must halt as `Malformed` rather than be
silently skipped as `Foreign` or treated as a deliberately reserved name.

<a id="rendered-names"></a>
## Rendering names and carrying recovery advice

`SyllabusName` makes the seam's structural choice explicit. `Positioned` holds
ordinal, key, and parts together. `Overview` holds none. Display renders a
positioned lesson as
`{ordinal:02}-{status}-{label}-i{key}.md` and a module as
`{ordinal:02}-{label}-i{key}`. It renders the distinguished variant as the exact
constant `OVERVIEW.md`.

Every halting verdict carries a `SyllabusError` with corrective action.
Noncanonical numbers name the canonical spelling when one can be computed.
Unknown status and bad-label errors identify the token to repair.
`SpeciesMismatch` explains whether the name or filesystem object must change.
`PublishInterrupted` tells the operator to wait, or to remove the witness only
after confirming that no publishing run remains live.

<!-- fragment «reference-name-and-errors» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/reference.rs" lines="211-351" parent="reference-domain-source" -->
````rust

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
````
<!-- /fragment -->

<a id="classification-walkthrough"></a>
## Classification walkthrough

Parsing checks the two exact unpositioned names first. `OVERVIEW.md` is accepted
only over a regular file. `PUBLISHING` is always `Reserved` because it is a
domain-owned transaction witness, not an entry.

All other owned names have a leading digit run and a terminal `-i<digits>`,
after removing the optional `.md` suffix. The suffix declares a lesson. Its
absence declares a module. A name missing either ownership marker is
`Foreign`. Once both markers are present, every failure is `Malformed`; treating
one as foreign could silently omit a file or an entire directory subtree.

Representative outcomes make the boundary precise:

| Listing | Observed as | Verdict | Reason |
|---|---|---|---|
| `01-published-vectors-i5.md` | file | `Entry` leaf | canonical lesson |
| `02-linear-algebra-i2` | directory | `Entry` node | canonical module |
| `OVERVIEW.md` | file | `Entry` distinguished | exact distinguished name |
| `README.md` | file | `Foreign` | lacks both ownership markers |
| `01--i3` | directory | `Malformed` | owned shape with an empty label |
| `5-draft-matrices-i6.md` | file | `Malformed` | canonical ordinal is `05` |
| `01-review-vectors-i5.md` | file | `Malformed` | unknown lesson status |
| `02-draft-matrices-i6.md` | directory | `Malformed` | leaf spelling over a directory |
| `PUBLISHING` | file | `Reserved` | interrupted-run witness |

The key separator is the terminal `-i` followed by digits. Therefore
`01-draft-notes-i7-i3.md` has label `notes-i7` and key 3. Parsing constructs the
name, renders it, and requires byte equality with the input. This single final
comparison rejects alternate padding and any future noncanonical spelling.

<!-- fragment «reference-parser» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/reference.rs" lines="352-473" parent="reference-domain-source" -->
````rust

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
````
<!-- /fragment -->

<a id="seam-mapping"></a>
## Mapping the domain back to the generic seam

The remaining trait methods contain no extra policy. `compose` stores exactly
the supplied triple. `distinguished` returns `Overview`. `view` exposes either
that distinguished case or the complete positioned triple.
`positioned_species` delegates to `Parts::species`, whose signature has no
ordinal or key available.

These methods make the worked shift deterministic. Composing ordinal 3 with the
key and parts read from `02-draft-matrices-i6.md` produces
`03-draft-matrices-i6.md`: only the ordinal changes. Later mutation chapters use
this same lesson and module vocabulary without redefining it.

<!-- fragment «reference-seam-methods» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/reference.rs" lines="474-505" parent="reference-domain-source" -->
````rust

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
````
<!-- /fragment -->

The helpers centralize species agreement, canonical advice, and ownership-shape
splitting. `split_shape` deliberately returns an empty middle as an owned shape;
`parse` then reports the missing label instead of silently disclaiming it.

<!-- fragment «reference-parser-helpers» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/reference.rs" lines="506-555" parent="reference-domain-source" -->
````rust

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
````
<!-- /fragment -->

<a id="conformance-obligations"></a>
## Seven obligations, with two enforcement mechanisms

The library can call a consumer's methods but cannot generally prove that they
agree across calls. Seven obligations define the usable seam:

1. `compose` places the ordinal, key, and parts it receives.
2. Parsing and rendering form a canonical grammar in both directions.
3. A name is positioned or distinguished, never neither.
4. A positioned name's species follows only from its parts.
5. `distinguished()` names the only entry of that species.
6. `parse` returns `Malformed` when `Found` contradicts the declared species.
7. Every rendering is one filesystem path component.

Rust constrains obligations 3 and 4 structurally. Each `NameView` return makes a
partial triple and a positioned distinguished name unrepresentable, while
`positioned_species(&Parts)` receives no name, ordinal, or key. Neither shape
prevents hidden mutable state from changing an answer across identical calls.
The `TYPE_SHAPE_CONSTRAINTS` table states both the enforced shape and the
remaining deterministic-call assumption so five sampled checks cannot be
mistaken for an incomplete proof.

The conformance kit samples the other five. The library also enforces obligation
7 at both filesystem boundaries because violating it would address outside the
locked tree. The other four remain semantic assumptions in production; a
consumer is expected to test them before using real data.

<!-- fragment «conformance-obligations» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/conformance.rs" lines="1-208" parent="reference-conformance-source" -->
````rust
//! The conformance kit: hand it sample names and sample triples, and learn
//! which of the trait's obligations your implementation violates.
//!
//! The seven obligations under [`EntryName`] are the **consumer's**, and the
//! library can check only the last of them from inside an operation — a design
//! missing any one of the other six admits a tree the library will quietly
//! corrupt, and it corrupts it silently, in a tree someone is using. This module
//! is where a domain finds that out instead, from a test, before there is a
//! tree. The seventh is here too, because meeting it as an
//! [`Error::NameIsNotOneComponent`] in an operation is worse than meeting it in
//! a test, even though it is not silent.
//!
//! [`Error::NameIsNotOneComponent`]: crate::Error::NameIsNotOneComponent
//!
//! ```no_run
//! # use ordinal_fs_tree::{conformance, reference::{SyllabusName, Parts, Status, Label}, Found, Ordinal, Key};
//! let report = conformance::check::<SyllabusName>(
//!     &[("01-draft-vectors-i1.md", Found::File), ("README.md", Found::File)],
//!     &[(Ordinal::new(1), Key::new(1), Parts::lesson(Status::Draft, Label::new("vectors").unwrap()))],
//! );
//! report.assert_conforming();
//! ```
//!
//! # Two kinds of finding, and why the second one exists
//!
//! A kit that only reports violations reads exactly the same when it is handed
//! nothing to check: no samples, no violations, conforming. That is the failure
//! this workstream has already met three times — a model, a tool and a runner
//! each reporting *found nothing* and *succeeded* with the same bytes
//! (`docs/formalism-findings.md`, entry 003). So the kit also reports which
//! obligations were never **exercised**, and [`Report::is_conforming`] is false
//! while any of them is. A suite of must-hold claims cannot detect that it did
//! not run; one that also says what it reached can.

use core::fmt;

use crate::{EntryName, EntryNameExt, Found, NameView, Species, Verdict};

/// One of the obligations this kit checks.
///
/// Five, not seven. Rust constrains the visible shape of the other two — *a
/// name is positioned or distinguished, never neither* and *the species
/// follows from the parts* — and those constraints are listed in
/// [`TYPE_SHAPE_CONSTRAINTS`]. Their stability across calls remains a semantic
/// law: neither Rust nor this sample-based kit proves the absence of hidden
/// mutable state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Obligation {
    /// `compose(o, k, p)` yields a name whose triple is `Some` and equal to
    /// `(o, k, p)`.
    ComposePlacesWhatItIsGiven,
    /// Distinct filenames never parse to the same name: `format(parse(f)) == f`,
    /// and every name the consumer can produce parses back to itself.
    TheGrammarIsCanonical,
    /// `parse` yields species `Distinguished` for `distinguished()` and for
    /// nothing else, and that name carries no triple.
    DistinguishedNamesTheOnlyEntryOfItsSpecies,
    /// A name declaring a species the listing contradicts is `Malformed`, never
    /// `Entry`.
    ParseRefusesWhatFoundContradicts,
    /// Every name the domain renders is exactly one filename: not empty, not
    /// `.` or `..`, and holding no path separator.
    ///
    /// The one obligation the library also enforces, so a domain that skips
    /// this check meets it as an `Error` rather than as a corrupted tree. It is
    /// checked here anyway, because a test is a cheaper place to meet it than
    /// an operation.
    ANameRendersAsOnePathComponent,
}

impl Obligation {
    /// Every obligation this kit checks.
    pub const ALL: [Self; 5] = [
        Self::ComposePlacesWhatItIsGiven,
        Self::TheGrammarIsCanonical,
        Self::DistinguishedNamesTheOnlyEntryOfItsSpecies,
        Self::ParseRefusesWhatFoundContradicts,
        Self::ANameRendersAsOnePathComponent,
    ];

    /// The obligation as the architecture document states it.
    #[must_use]
    pub const fn statement(self) -> &'static str {
        match self {
            Self::ComposePlacesWhatItIsGiven => "compose places what it is given",
            Self::TheGrammarIsCanonical => "the grammar is canonical",
            Self::DistinguishedNamesTheOnlyEntryOfItsSpecies => {
                "distinguished() names the only entry of its species"
            }
            Self::ParseRefusesWhatFoundContradicts => "parse refuses what found contradicts",
            Self::ANameRendersAsOnePathComponent => "a name renders as one path component",
        }
    }

    /// What a tree looks like when this obligation does not hold.
    ///
    /// Every one of these but the last is a structure
    /// `docs/ordinal-fs-tree/models/structure.als` produces on demand, under the
    /// named `witness_…` command. The last has no witness and can have none:
    /// both models hold no strings by design, so a rendering that is not a
    /// filename is not a thing either can say — which is why it is the one
    /// obligation the library enforces instead of assuming.
    #[must_use]
    pub const fn what_it_admits(self) -> &'static str {
        match self {
            Self::ComposePlacesWhatItIsGiven => {
                "a sibling shift that moves one entry's key onto another's position, \
                 while every stated invariant still holds (witness_shift_corrupts_identity)"
            }
            Self::TheGrammarIsCanonical => {
                "two files on disk that are one entry, sharing a key and an ordinal \
                 (witness_two_filenames_name_one_entry)"
            }
            Self::DistinguishedNamesTheOnlyEntryOfItsSpecies => {
                "a node holding two distinguished children (witness_two_distinguished_children)"
            }
            Self::ParseRefusesWhatFoundContradicts => {
                "a directory wearing a leaf's name, or a distinguished child that is a \
                 directory — either way an entire subtree invisible to every traversal \
                 (witness_species_mismatch_is_unclassifiable, \
                 witness_distinguished_directory_hides_a_subtree)"
            }
            Self::ANameRendersAsOnePathComponent => {
                "a create, a rename, a rollback removal and a reported path addressing \
                 outside the tree whose containing directory is the only thing locked \
                 — the library joins this rendering to a level's directory, while the \
                 algebra compares views and sees a perfectly canonical name (no model \
                 witness: both models hold no strings)"
            }
        }
    }
}

impl fmt::Display for Obligation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.statement())
    }
}

/// The part of an obligation constrained by Rust's type shape.
pub struct TypeShapeConstraint {
    /// The obligation, as the architecture document states it.
    pub statement: &'static str,
    /// What the type shape enforces for each call.
    pub enforced: &'static str,
    /// The semantic law that the type shape does not enforce.
    pub assumed: &'static str,
}

/// The structural constraints this kit does **not** sample because each return
/// value already has the required Rust shape.
///
/// Reporting them is the point: a consumer reading five checks where the
/// document states seven needs to know that the other two were not forgotten.
/// This table deliberately does not call either obligation discharged: trait
/// methods may consult interior or global mutable state, so identical explicit
/// inputs can produce different well-shaped answers across calls. A finite
/// conformance sample could expose an implementation that changes during that
/// sample, but could not prove deterministic behavior in general.
pub const TYPE_SHAPE_CONSTRAINTS: &[TypeShapeConstraint] = &[
    TypeShapeConstraint {
        statement: "a name is positioned or distinguished, never neither",
        enforced: "Each `EntryName::view` call returns one `NameView`: either a `Triple` — \
                   the ordinal, key and parts together — or the distinguished child, which \
                   has none of them. A single returned value cannot carry only some triple \
                   fields or carry a triple while claiming the distinguished species.",
        assumed: "Repeated `EntryName::view` calls with the same receiver and no \
                  caller-visible mutation are deterministic; hidden mutable state does not \
                  influence the variant or triple.",
    },
    TypeShapeConstraint {
        statement: "the species follows from the parts",
        enforced: "`EntryName::positioned_species` is an associated function whose only \
                   explicit input is `&Parts`: it receives no `self`, ordinal or key.",
        assumed: "`EntryName::positioned_species` is deterministic from the parts value \
                  across calls and does not derive its answer from hidden mutable state. A \
                  `Parts` equality coarser than the species remains lawful; occupancy uses \
                  `EntryNameExt::same_name`, which compares both view and species.",
    },
];

/// Legacy description of a type-shape constraint.
#[deprecated(note = "use TypeShapeConstraint and TYPE_SHAPE_CONSTRAINTS")]
pub struct Discharged {
    /// The obligation, as the architecture document states it.
    pub statement: &'static str,
    /// What Rust constrains and which deterministic behavior remains assumed.
    pub how: &'static str,
}

/// Compatibility view of [`TYPE_SHAPE_CONSTRAINTS`] under its original name.
///
/// The name is retained for source compatibility, not as a claim that Rust
/// proves call stability. New code should use [`TYPE_SHAPE_CONSTRAINTS`].
#[allow(deprecated)]
#[deprecated(note = "use TYPE_SHAPE_CONSTRAINTS; call stability is a semantic law")]
pub const DISCHARGED_BY_THE_TYPE_SYSTEM: &[Discharged] = &[
    Discharged {
        statement: "a name is positioned or distinguished, never neither",
        how: "Each call returns one complete `NameView`; repeated calls with the same \
              receiver are assumed deterministic because hidden state is not excluded.",
    },
    Discharged {
        statement: "the species follows from the parts",
        how: "The only explicit input is `&Parts`; the answer is assumed deterministic from \
              that input because global mutable state is not excluded.",
    },
];
````
<!-- /fragment -->

<a id="findings-and-coverage"></a>
## Findings distinguish failure from absence

A `Finding::Violated` records a concrete counterexample. A
`Finding::NotExercised` records that the supplied samples never posed an
obligation. Both make `Report::is_conforming` false.

This distinction prevents an empty or one-sided sample set from reading as a
pass. `assert_conforming` is the consumer test's one-line assertion, but its
success means only that every checked obligation was exercised by these samples
and no counterexample was found. The kit is a test over examples, not a proof
over the consumer's whole grammar.

<!-- fragment «conformance-report» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/conformance.rs" lines="209-313" parent="reference-conformance-source" -->
````rust

/// What the kit learned about one obligation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Finding {
    /// The implementation breaks this obligation. `detail` says how.
    Violated {
        /// The broken obligation.
        obligation: Obligation,
        /// The concrete case that broke it.
        detail: String,
    },
    /// The samples never put this obligation to the test, so a pass would mean
    /// nothing. `detail` says what sample was missing.
    NotExercised {
        /// The untested obligation.
        obligation: Obligation,
        /// What the samples would need to contain.
        detail: String,
    },
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Violated { obligation, detail } => {
                write!(f, "VIOLATED   {obligation}\n           {detail}")
            }
            Self::NotExercised { obligation, detail } => {
                write!(f, "UNTESTED   {obligation}\n           {detail}")
            }
        }
    }
}

/// What [`check`] found.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    findings: Vec<Finding>,
}

impl Report {
    /// Every finding, violations and untested obligations alike.
    #[must_use]
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    /// Only the obligations the implementation actually breaks.
    pub fn violations(&self) -> impl Iterator<Item = &Finding> {
        self.findings
            .iter()
            .filter(|f| matches!(f, Finding::Violated { .. }))
    }

    /// Only the obligations the samples never reached.
    pub fn unexercised(&self) -> impl Iterator<Item = &Finding> {
        self.findings
            .iter()
            .filter(|f| matches!(f, Finding::NotExercised { .. }))
    }

    /// True when every obligation was exercised and none was violated.
    ///
    /// Untested counts against it deliberately: see this module's header.
    #[must_use]
    pub fn is_conforming(&self) -> bool {
        self.findings.is_empty()
    }

    /// Panic with the whole report unless [`is_conforming`](Report::is_conforming).
    ///
    /// The one line a consumer's own test needs.
    #[track_caller]
    pub fn assert_conforming(&self) {
        assert!(self.is_conforming(), "{self}");
    }

    fn violate(&mut self, obligation: Obligation, detail: String) {
        self.findings.push(Finding::Violated { obligation, detail });
    }

    fn untested(&mut self, obligation: Obligation, detail: &str) {
        self.findings.push(Finding::NotExercised {
            obligation,
            detail: detail.to_string(),
        });
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.findings.is_empty() {
            return write!(
                f,
                "conforming: all {} checked obligations exercised and held",
                Obligation::ALL.len()
            );
        }
        writeln!(f, "{} finding(s):", self.findings.len())?;
        for finding in &self.findings {
            writeln!(f, "{finding}")?;
        }
        Ok(())
    }
}
````
<!-- /fragment -->

<a id="checking-the-seam"></a>
## How the reusable check probes the seam

`check::<N>` accepts directory-listing samples `(&str, Found)` and positioned
sample triples `(Ordinal, Key, N::Parts)`. A useful set includes accepted names
of every species, the distinguished child, a foreign name, domain-specific near
misses, and one triple for each positioned species.

The check first composes every triple and compares the returned view field by
field. It then checks canonicality in both directions: every accepted listing
renders to the original filename, and every composed or distinguished name
parses back to the same view and species. Comparing only strings would miss a
parser that preserves its display while changing the key behind it.

<!-- fragment «conformance-compose-and-canonical» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/conformance.rs" lines="314-473" parent="reference-conformance-source" -->
````rust

/// Check an [`EntryName`] implementation against the obligations the library
/// assumes and cannot enforce.
///
/// `listings` are sample directory entries — a filename and what the listing
/// reports is under it — and should include the domain's own well-formed names,
/// its distinguished child, at least one foreign name, and any near-miss the
/// grammar is meant to refuse. `triples` are sample `(ordinal, key, parts)`
/// values, one per species the domain composes.
///
/// The samples do not have to be exhaustive and cannot be: this is a test kit,
/// not a proof. What it does guarantee is that it says so when they are too
/// thin to test something — see [`Report::is_conforming`].
#[must_use]
pub fn check<N: EntryName>(
    listings: &[(&str, Found)],
    triples: &[(crate::Ordinal, crate::Key, N::Parts)],
) -> Report {
    let mut report = Report::default();

    // --- compose places what it is given ----------------------------------
    //
    // Alloy: `ComposeLawful`, checked as `ShiftPreservesIdentity`. Without it a
    // shift is a corruption, because a shift is nothing but a compose.
    let composed: Vec<N> = triples
        .iter()
        .map(|(o, k, p)| N::compose(*o, *k, p.clone()))
        .collect();
    for ((ordinal, key, parts), name) in triples.iter().zip(&composed) {
        match name.view() {
            NameView::Distinguished => report.violate(
                Obligation::ComposePlacesWhatItIsGiven,
                format!(
                    "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, which is \
                     the distinguished child and carries no triple. A composed name is \
                     positioned by construction."
                ),
            ),
            NameView::Positioned(t) => {
                if t.ordinal != *ordinal {
                    report.violate(
                        Obligation::ComposePlacesWhatItIsGiven,
                        format!(
                            "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, \
                             which reports ordinal {}.",
                            t.ordinal
                        ),
                    );
                }
                if t.key != *key {
                    report.violate(
                        Obligation::ComposePlacesWhatItIsGiven,
                        format!(
                            "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, \
                             which reports key {}.",
                            t.key
                        ),
                    );
                }
                if t.parts != parts {
                    report.violate(
                        Obligation::ComposePlacesWhatItIsGiven,
                        format!(
                            "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, \
                             whose parts are not the ones it was given."
                        ),
                    );
                }
            }
        }
    }
    if triples.is_empty() {
        report.untested(
            Obligation::ComposePlacesWhatItIsGiven,
            "no sample triples were supplied, so `compose` was never called.",
        );
    }

    let distinguished = N::distinguished();
    let distinguished_name = distinguished.as_ref().map(ToString::to_string);

    // --- the grammar is canonical -----------------------------------------
    //
    // Both directions, because *isomorphic* means both: a filename that parses
    // must render back to itself, and a name the consumer can produce must
    // parse back to *that name*. Alloy: `ParseIsCanonical` and
    // `RoundTripDisplay`.
    let mut parsed_any_listing = false;
    for (filename, found) in listings {
        if let Verdict::Entry(name) = N::parse(filename, *found) {
            parsed_any_listing = true;
            let rendered = name.to_string();
            if rendered != *filename {
                report.violate(
                    Obligation::TheGrammarIsCanonical,
                    format!(
                        "`{filename}` parsed to a name that renders as `{rendered}`. Two \
                         spellings of one name means two files on disk are one entry."
                    ),
                );
            }
        }
    }
    if !parsed_any_listing {
        report.untested(
            Obligation::TheGrammarIsCanonical,
            "no sample listing parsed to an entry, so no filename was rendered back. \
             Supply at least one well-formed name of each species.",
        );
    }
    // `RoundTripDisplay` is `v.seen = n`: the name that comes back is *the same
    // name*, not merely one that renders the same way. Comparing renderings
    // alone lets a domain parse its own output into a different triple while
    // `Display` keeps saying what it said — the strings agree, and every
    // snapshot then reads an ordinal and a key that were never composed. The
    // distinguished spelling goes through the same check, where the thing to
    // come back is the absence of a triple.
    for name in composed.iter().chain(distinguished.as_ref()) {
        let rendered = name.to_string();
        match N::parse(&rendered, name.species().requires()) {
            Verdict::Entry(reparsed) => {
                let again = reparsed.to_string();
                if again != rendered {
                    report.violate(
                        Obligation::TheGrammarIsCanonical,
                        format!("`{rendered}` parses to a name rendering as `{again}`."),
                    );
                } else if reparsed.view() != name.view() {
                    report.violate(
                        Obligation::TheGrammarIsCanonical,
                        format!(
                            "`{rendered}` renders back to itself but parses to a different \
                             name — its ordinal, key or parts are not the ones that were \
                             composed. Every operation reads the triple, not the string."
                        ),
                    );
                } else if reparsed.species() != name.species() {
                    // Implied by the views agreeing, since the species is read
                    // off the view; stated because the obligation is stated of
                    // both and a later change to either derivation lands here.
                    report.violate(
                        Obligation::TheGrammarIsCanonical,
                        format!(
                            "`{rendered}` parses back to a {} where it was composed as a {}.",
                            reparsed.species(),
                            name.species()
                        ),
                    );
                }
            }
            _ => report.violate(
                Obligation::TheGrammarIsCanonical,
                format!(
                    "`{rendered}` does not parse back to an entry. Every name the consumer \
                     can produce must be one the consumer can read."
                ),
            ),
        }
    }

````
<!-- /fragment -->

The component check renders every name the domain can produce: composed names,
accepted listings, and the distinguished name. The distinguished uniqueness
check verifies that `distinguished()` parses as that species and that no other
sample claims it. The distinguished name is inspected but does not by itself
count as sample coverage; otherwise an empty caller-supplied set could appear to
exercise the obligation.

<!-- fragment «conformance-component-and-distinguished» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/conformance.rs" lines="474-572" parent="reference-conformance-source" -->
````rust
    // --- a name renders as one path component ------------------------------
    //
    // No Alloy claim, and there cannot be one: `structure.als` holds no strings,
    // so it cannot pose a rendering at all. This is the obligation the library
    // enforces at both boundaries where a name becomes a path, and the kit
    // checks it so that a domain meets it in a test rather than in an operation.
    //
    // Every name the domain can *produce* is a candidate: what it composes, what
    // it parses out of a listing, and its distinguished child.
    // `distinguished()` is checked like any other name but does not *count* as
    // coverage, for the reason the found-contradicts check gives: a domain that
    // supplies its own name would otherwise let a kit handed no samples at all
    // report this obligation as exercised.
    let mut rendered_any = !composed.is_empty();
    let render_check = |name: &N, report: &mut Report| {
        let rendered = name.to_string();
        if let Some(reason) = crate::name::not_one_component(&rendered) {
            report.violate(
                Obligation::ANameRendersAsOnePathComponent,
                format!(
                    "`{rendered}` {reason}, so it is not one filename. The library joins \
                     a rendering to a level's directory to reach the entry, so this one \
                     addresses outside the tree."
                ),
            );
        }
    };
    for name in composed.iter().chain(distinguished.as_ref()) {
        render_check(name, &mut report);
    }
    for (filename, found) in listings {
        if let Verdict::Entry(name) = N::parse(filename, *found) {
            rendered_any = true;
            render_check(&name, &mut report);
        }
    }
    if !rendered_any {
        report.untested(
            Obligation::ANameRendersAsOnePathComponent,
            "no sample yielded a name, so nothing was rendered. Supply a triple to \
             compose, or a listing this domain recognises.",
        );
    }

    // --- distinguished() names the only entry of its species ---------------
    //
    // Alloy: `OneDistinguishedName` and `DistLawful`, checked as
    // `DistinguishedIsUniquePerNode`. That `distinguished()` itself carries no
    // triple is no longer checkable — `NameView::Distinguished` holds none —
    // so what is left is the half about every *other* name.
    if let Some(d) = &distinguished {
        let rendered = d.to_string();
        match N::parse(&rendered, Found::File) {
            Verdict::Entry(n) if n.species() == Species::Distinguished => {}
            Verdict::Entry(n) => report.violate(
                Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                format!("`{rendered}` parses as {} rather than as the distinguished child.", n.species()),
            ),
            _ => report.violate(
                Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                format!("`{rendered}` is the name distinguished() returns and does not parse as an entry."),
            ),
        }
    }
    for (filename, found) in listings {
        if let Verdict::Entry(name) = N::parse(filename, *found) {
            if name.species() == Species::Distinguished {
                match &distinguished_name {
                    Some(d) if d == filename => {}
                    Some(d) => report.violate(
                        Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                        format!(
                            "`{filename}` parses as a distinguished child, but distinguished() \
                             returns `{d}`. A node could then hold both."
                        ),
                    ),
                    None => report.violate(
                        Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                        format!(
                            "`{filename}` parses as a distinguished child in a domain whose \
                             distinguished() is None."
                        ),
                    ),
                }
            }
        }
    }
    // Checking `distinguished()` against itself is half the obligation. The other
    // half — that *no other* name claims that species — needs names to look at,
    // and a domain whose own distinguished child is the only thing the kit saw
    // has not been asked the question at all.
    if !parsed_any_listing && composed.is_empty() {
        report.untested(
            Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
            "no supplied sample yielded a name, so nothing showed that no name other \
             than distinguished() claims that species.",
        );
    }

````
<!-- /fragment -->

For species agreement, each candidate is offered under all three `Found` values.
At least one agreeing case and one `Malformed` contradiction must be observed.
`Foreign` and `Reserved` are not acceptable contradiction outcomes for a name
the domain accepts under another observation: foreign would skip it silently,
and reserved would deny that it is an entry at all.

<!-- fragment «conformance-found-agreement» owner="reference-domain-k13" source="crates/ordinal-fs-tree/src/conformance.rs" lines="573-667" parent="reference-conformance-source" -->
````rust
    // --- parse refuses what found contradicts ------------------------------
    //
    // Every sample name is offered under all three `Found` values, not only the
    // one it was paired with: the obligation is about what `parse` does when the
    // listing contradicts the name, and a sample that is only ever shown its own
    // truth never asks the question. Alloy: `SpeciesAgreementIsParsed`.
    //
    // The refusal has to be `Malformed`, and that is the whole of the
    // obligation rather than a detail of it: `Foreign` means *not my name*, and
    // a walk skips a foreign name silently — together with everything beneath
    // it when it is a directory. A domain that answers `Foreign` where its own
    // name contradicts the listing has hidden exactly the subtree this
    // obligation exists to expose, so the kit only accepts a halt that carries
    // the domain's own error.
    let mut agreed = false;
    let mut refused = false;
    let every_found = [Found::File, Found::Dir, Found::Other];
    let mut candidates: Vec<String> = listings.iter().map(|(f, _)| (*f).to_string()).collect();
    candidates.extend(composed.iter().map(ToString::to_string));
    // `distinguished()` is checked like any other name but does not *count* as
    // coverage: a domain that supplies its own name would otherwise let a kit
    // handed no samples at all report this obligation as exercised, which is
    // the failure mode the two kinds of finding exist to prevent.
    let supplied = candidates.len();
    if let Some(d) = &distinguished_name {
        candidates.push(d.clone());
    }
    for (index, filename) in candidates.iter().enumerate() {
        let mut recognised = false;
        let mut not_entry: Vec<(Found, &'static str)> = Vec::new();
        for found in every_found {
            match N::parse(filename, found) {
                Verdict::Entry(name) => {
                    recognised = true;
                    if name.species().agrees_with(found) {
                        agreed |= index < supplied;
                    } else {
                        report.violate(
                            Obligation::ParseRefusesWhatFoundContradicts,
                            format!(
                                "`{filename}` over {found} parsed as an entry of species {}, \
                                 which requires {}. That is how a subtree disappears from every \
                                 traversal while the tree reports itself healthy.",
                                name.species(),
                                name.species().requires()
                            ),
                        );
                    }
                }
                Verdict::Foreign => not_entry.push((found, "Foreign")),
                Verdict::Malformed(_) => not_entry.push((found, "Malformed")),
                Verdict::Reserved(_) => not_entry.push((found, "Reserved")),
            }
        }
        // A name this domain never accepts is its own affair: whether it is
        // foreign or reserved says nothing about this obligation. Only a name
        // the domain *does* recognise can contradict a listing.
        if !recognised {
            continue;
        }
        for (found, verdict) in not_entry {
            if verdict == "Malformed" {
                refused |= index < supplied;
            } else {
                report.violate(
                    Obligation::ParseRefusesWhatFoundContradicts,
                    format!(
                        "`{filename}` parses as an entry under another listing, so this \
                         domain owns the name; over {found} it answers {verdict}. A \
                         contradiction must be Malformed, carrying this domain's own error: \
                         Foreign is skipped silently, taking the whole subtree with it when \
                         the name is a directory, and Reserved says the name is deliberately \
                         not an entry, which this one is not."
                    ),
                );
            }
        }
    }
    if !agreed {
        report.untested(
            Obligation::ParseRefusesWhatFoundContradicts,
            "no sample name parsed as an entry under any `Found`, so the agreeing case \
             was never seen.",
        );
    } else if !refused {
        report.untested(
            Obligation::ParseRefusesWhatFoundContradicts,
            "no sample name that parsed was refused as Malformed under a contradicting \
             `Found`, so no contradiction was ever put to the domain. Supply a name of a \
             species that requires a file, or one that requires a directory.",
        );
    }

    report
}
````
<!-- /fragment -->

<a id="reusable-samples"></a>
## Reusable reference samples

The crate's conformance test uses this compact corpus:

- `OVERVIEW.md` as a regular file;
- one published lesson and one draft lesson as regular files;
- `02-linear-algebra-i2` as a directory;
- `README.md` as a foreign regular file;
- `PUBLISHING` as the reserved witness;
- one lesson triple and one module triple.

That set makes the reference implementation conform under all five sampled
obligations. Separate adversarial domains prove that the kit catches ignored
compose arguments, lenient spelling, ignored filesystem species, a second
distinguished spelling, a contradiction disguised as foreign, hidden key drift,
and a rendering containing `../`. Domain-specific parser tests additionally
cover malformed labels, terminal-key ambiguity, noncanonical padding, and
recovery advice. These samples establish the vocabulary later pages reuse; they
do not replace the crate tests or the formal model.

[Previous: Name seam](02-name-seam.md) | [Contents](README.md) | [Next: Read path](04-read-path.md)

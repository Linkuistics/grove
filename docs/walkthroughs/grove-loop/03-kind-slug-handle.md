# Kind, slug, handle
<!-- book-page id="kind-slug-handle" slice="the-handle-not-the-position" order="3" -->
[Previous: The tokens, and the four verdicts](02-the-tokens.md) | [Contents](README.md) | [Next: The name, and canonicity](04-the-name.md)

<a id="the-handle-not-the-position"></a>
## The rule: the handle is the identity, and the position is not in it

Chapter 2 read the grammar's punctuation and the judgement grove reaches about a
name in a directory listing. It named `Kind`, `Slug`, `Handle` and `Parts` on
every page of the module header and defined none of them. This chapter defines
them, and the rule it exists for is what separates two of them:

> A work item's identity is its **handle**, `<slug>-k<key>`. Its **position**
> among its siblings is not part of that identity, and no operation on the
> position is an operation on the identity.

That separation is the second thing this crate kept. The store gives every entry
an ordinal and a key and has no word for either a *slug* or a *handle*; it can
order entries and it can tell one from another, but it cannot say which of the
two facts an operator means when they write a name down. Grove says so by
spelling the identity in one place and leaving the position out of it, and it
pays for that by owning a second grammar — the handle's — beside the filename's.

Take the first leaf a new grove holds, `01-requirements--plan-k1.md`. Its bytes
divide six ways, and only the last two are its identity.

| Byte run | What it is | Type | Read in |
|---|---|---|---|
| `01` | position among siblings | `Ordinal`, the store's | chapter 4 |
| `requirements` | session kind | `Kind` | this chapter |
| `--` | kind/slug separator | `SEPARATOR` | chapter 2 |
| `plan` | human-facing name | `Slug` | this chapter |
| `-k1` | key marker and permanent key | `KEY_MARK`, and `Key`, the store's | chapter 2, and chapter 4 |
| `.md` | declared species | — | chapter 4 |

The table is the chapter's rule in the form a reader can check against a
directory listing: the handle is `plan-k1`, which is rows four and five run
together, and row one is not in it. Strip `.md` and the name ends in exactly
those bytes.

Two routes reach that handle and neither carries the position. `Handle::of`
takes the parsed name and reads the slug and the key out of it. `Handle::parse`
takes the text `plan-k1` — typed by an operator who never saw the file — and
reads the same slug and the same key out of that. Both produce a `Handle` whose
two fields are `plan` and `1`; neither has a field the `01` could go in. The
observable end is that `Handle::of` applied to the parsed `01-requirements--plan-k1.md`
and `Handle::parse` applied to the string `plan-k1` are equal, which is asserted
over three filenames in `a_handle_and_a_filename_peel_the_same_key` below.

The separation preserves references where the position changes and the identity
must not. `leaf-insert` shifts every later sibling up one position, and
`leaf-decompose` turns a leaf file into a node directory; in both the entry keeps
its key, so it keeps its handle, so every reference already written to it still
resolves. Chapters 10 and 12 read those two verbs. This chapter is where the
property they rely on is established, and the test that holds it is
`every_positioned_name_ends_in_its_own_handle`, whose own comment says why it is
asserted rather than reviewed for: *drift is not expressible* has to be held by
something, or it is a promise.

The chapter owns 563 of the file's 1,712 lines in three blocks — the named parts
themselves at lines 221 to 590, and the two labelled sections of the inline test
module that hold this chapter's claims, at 1,522 to 1,550 and 1,551 to 1,712. The
name that carries these parts is chapter 4's, and so is every route into them
from a string on disk.

<a id="the-open-token"></a>
## The kind: an open token, and the whole of the type

The block opens on the first of the two words a leaf name carries. Chapter 2
established that both words obey one rule, `refuse_token`, and that the rule is a
free function rather than a method because the canonicity of a leaf name depends
on both words rather than on either type. `Kind` is the first type built on it.

The composite below is this chapter's production ownership block. It expands, in
order, to lines 221 through 590 of the file, and the sixteen fragments it names
run from here to the end of the seventh section.

<!-- fragment «kind-slug-and-handle» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="221-590" parent="source-task-name" -->
<!-- insert «name-kind» -->
<!-- insert «name-reserved-labels» -->
<!-- insert «name-kind-methods» -->
<!-- insert «name-kind-display» -->
<!-- insert «name-slug» -->
<!-- insert «name-slug-methods» -->
<!-- insert «name-slug-display» -->
<!-- insert «name-handle-error» -->
<!-- insert «name-handle-error-display» -->
<!-- insert «name-handle» -->
<!-- insert «name-handle-new-and-of» -->
<!-- insert «name-handle-parse» -->
<!-- insert «name-handle-accessors-and-render» -->
<!-- insert «name-handle-display» -->
<!-- insert «name-parts» -->
<!-- insert «name-parts-methods» -->
<!-- /fragment -->

The type is a newtype over `String` with a private field, so the only way from a
string to a `Kind` is the constructor two fragments below, and the only way back
out is `label` or `Display`. The derives are worth reading against that: `Clone`,
`Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash` — everything a
`String` offers, because a kind is a token and grove holds no opinion about it
that a string comparison would not already answer.

<!-- fragment «name-kind» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="221-244" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

The first line's second clause compresses one step. A kind is not the skill a
session loads; it is the stem the skill's name is built from. `crates/grove-loop/src/prompt.rs`
line 63 renders `format!("{PLUGIN}-{}", kind.label())` with `PLUGIN` as `"grove"`,
so the kind `impl` names the skill `grove-impl`, and chapter 19 reads that
composition. The third clause is exact as written: the configuration is keyed by
the label itself, at `crates/grove-loop/src/loop_driver.rs` lines 254 and 266,
which chapter 20 reads.

The claim in the second bold sentence is checkable inside this corpus, and it
holds. Extracting every string literal from the production source of
`crates/grove-loop/src` and classifying each finds exactly two kind literals —
`REQUIREMENTS` and `FINISH`, in the fragment below — and no third. The
parenthesis is right about which near-misses to expect and is not a complete
partition of what the extraction returns: `"plan"` at
`crates/grove-loop/src/tree_lifecycle.rs` line 56 and `"finish"` at line 166 of
the same file are the two slug literals it names. The second is the clearest
case in the crate: a literal `"finish"` that is a slug rather than a kind and
reaches `Slug::new` rather than `Kind::new`. Beyond those the extraction also
returns tokens that are neither — `"done"` and `"relaunch"` in
`crates/grove-loop/src/complete.rs`, `"shared"` and `"exclusive"` in
`crates/grove-loop/src/driver_lease.rs` — which is why the method the comment
prescribes is classification rather than a two-way sort. Chapter 14 reads the two
slug literals, chapter 15 reads `complete.rs`, and chapter 16 reads the lease.
The clause *and no enumeration at all* holds in the strongest form available:
the type is one `String`, and there is no `ALL`, no roster and no match on a kind
label anywhere in the crate.

<a id="the-two-grove-spells"></a>
## The two labels grove spells, and the one question it asks about them

The two constants are private and the two constructors are public, which is the
shape the doc comment's parenthetical reason produces: a constant that is the
literal's one home, and a function that is how anything else reaches it.

<!-- fragment «name-reserved-labels» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="245-250" parent="kind-slug-and-handle" -->
````rust
/// Root scaffolding's kind. One of the two tokens grove may name.
const REQUIREMENTS: &str = "requirements";

/// The driver-owned teardown sentinel's kind. The other one.
const FINISH: &str = "finish";

````
<!-- /fragment -->

`Kind::requirements` is spent once in the crate's production source, at
`crates/grove-loop/src/tree_lifecycle.rs` line 83, where the lifecycle transition
scaffolds a grove for a driver that has no operator to ask; `Kind::finish` is
spent at lines 136 and 147 of the same file for the teardown sentinel, and at
`crates/grove-loop/src/loop_driver.rs` line 254 to name the template that leaf
will need. Chapters 14 and 20 read those. The asymmetry in the implementation
below follows from that: there is an `is_finish` and no `is_requirements`,
because grove never has to ask whether a leaf is the one `root-init` laid down —
that leaf is ordinary work from the moment it exists — and does have to ask
whether a leaf is the one the driver reserved for itself.

<!-- fragment «name-kind-methods» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="251-300" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

`is_finish` compares against the constant rather than against `Self::finish()`,
so the question costs no allocation. Everything else here is the newtype's
minimum: one validating constructor that returns the same `TokenError` a `Slug`
returns, two constructors that cannot fail, and one accessor.

**Seven calls, six functions, and the comment's four categories cover them.**
Two calls are in selection, at `crates/grove-loop/src/task_tree.rs` lines 615 and
629, both inside `selected` — and only the second is the ordering, since the
first refuses a tree holding more than one live `finish` leaf, which is a
malformed-tree report rather than a sort. One is the creation refusal, at
`crates/grove-loop/src/task_grow.rs` line 491, reached by `leaf-add`,
`leaf-insert`, `root-init` and `leaf-decompose`. One is at
`crates/grove-loop/src/tree_lifecycle.rs` line 230, inside `finish_commit`, which
is the verb the finish session runs — `crates/grove-loop/src/loop_driver.rs`
never calls `is_finish` at all, so nothing in the loop driver itself asks. The
remaining three, at lines 621, 757 and 921 of `tree_lifecycle.rs`, refuse to
decompose, retire or prune a `finish` leaf that already exists. Line 621 is
provably a separate question from the creation refusal, because `leaf-decompose`
asks both: line 621 about the leaf it is converting, and line 560's call to
`refuse_finish_kind` about the kind of the child it is growing. Chapters 7, 10,
12, 13 and 14 read the seven sites. The licence reaches all of them: every one of
the seven is grove recognising a leaf it wrote itself, and none of them
interprets what a `finish` session is for.

<!-- fragment «name-kind-display» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="301-306" parent="kind-slug-and-handle" -->
````rust
impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

````
<!-- /fragment -->

`Display` and `label` reach the same bytes by two routes because their callers
want two things. `label` returns a `&str` for a caller that needs to index or
concatenate — the skill name and the configuration key above. `Display` is what a
format string reaches, and the one that matters is chapter 4's rendering of a
whole name, where `kind.label()` is written into the middle of a `write!` rather
than the kind being formatted directly.

<a id="the-slug"></a>
## The slug: the half of a name that is not identity

The second word is the human-facing one, and its doc comment states the rule this
chapter is named for from the other side — the slug is not unique and not
identity, and the key is.

<!-- fragment «name-slug» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="307-315" parent="kind-slug-and-handle" -->
````rust
/// The human-facing part of a name. Not unique, and not identity — the key is.
///
/// Validated on construction, so a `Slug` that exists is one that renders and
/// re-parses. The character set already excludes everything that could blur a
/// name boundary (`.`, `/`, `[`, `]`), and being lowercase keeps it clear of the
/// uppercase outcome infixes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slug(String);

````
<!-- /fragment -->

The character set the comment describes is `refuse_token`'s, read backwards: the
guard exists so that the exclusions hold, and the comment names the two the slug
depends on. Excluding `.`, `/`, `[` and `]` keeps a slug from blurring a name
boundary, and lowercase keeps it clear of `DONE` and `ABANDONED`, which is why
those two words are also refused explicitly rather than left to the character
set. The consequence a reader can hold on to is the one the second paragraph
states: a `Slug` value that exists is one that renders and re-parses, so no code
downstream of construction has to re-check it.

<!-- fragment «name-slug-methods» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="316-338" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

That guarantee is spent where a constant would have been easier.
`crates/grove-loop/src/tree_lifecycle.rs` line 166 builds the `finish` sentinel's
slug by putting the literal `"finish"` through this constructor and turning a
failure into an error, rather than holding a validated `Slug` constant — the
comment there says a constant that goes through the validating constructor is
still one constant, and going through it is what keeps the guarantee true of
every `Slug` in the process rather than of most of them. Chapter 14 reads it.

<!-- fragment «name-slug-display» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="339-344" parent="kind-slug-and-handle" -->
````rust
impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

````
<!-- /fragment -->

`Slug` names its accessor `as_str` where `Kind` names its `label`, and the
difference is not cosmetic: a kind's string is a key in two places outside the
filename, and a slug's is not a key anywhere.

<a id="what-a-refusal-owes"></a>
## What a refused handle owes an operator

A handle reaches grove from two directions, and the second is what shapes this
type: a filename that fails to parse is a report about a tree, but a handle that
fails to parse is usually something a person typed.

<!-- fragment «name-handle-error» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="345-375" parent="kind-slug-and-handle" -->
````rust
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
        error: TokenError,
    },
}

````
<!-- /fragment -->

Three variants, in the order `Handle::parse` reaches them, and each carries the
text it was handed. `NotHandleShaped` carries nothing else because there is
nothing else to name — the string had no terminal `-k<digits>` at all.
`KeyOutOfRange` carries the digit run, which is the part the operator has to
look at. `BadSlug` carries both the offending slug and the `TokenError`
explaining it, which is the one place in this file where an error of chapter 2's
is nested inside an error of this chapter's.

<!-- fragment «name-handle-error-display» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="376-402" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

Every message states the grammar it wanted, which is the model chapter 2 read on
`TokenError` applied one level up. Two details are this type's own. The nesting
in `BadSlug` is rendered inline, `— {error}`, rather than chained: `HandleError`
implements `std::error::Error` without a `source`, exactly as `TokenError` does,
so the `TokenError` it holds is visible in the message and not through the error
chain. And `NotHandleShaped`'s message spends its length on the terminality rule
— that the key is the *terminal* `-k<digits>`, so a slug may contain `-k9` — which
is the one property of this grammar an operator cannot guess and the one the
tests below spend three fixtures on.

<a id="one-place-the-grammar-is-spelled"></a>
## One place the grammar is spelled

Chapter 2 reproduced the module header's claim that this type owns the
`<slug>-k<key>` grammar and that a filename and a handle saying different things
is not expressible, and said this chapter would prove it rather than restate it.
The proof has two halves: this section, where the single renderer is defined and
every rendering path is shown to end in it, and the test section below, where the
consequence is asserted.

<!-- fragment «name-handle» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="403-426" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

The struct is two private fields and the derives stop at `Clone`, `Debug`,
`PartialEq` and `Eq` — no `Ord` and no `Hash`, where both `Slug` and `Kind` have
them. That is consistent with the rule the chapter opened on rather than an
oversight to note: ordering entries is the position's job, the position is not in
the handle, and the two places that compare handles compare them for equality.

<!-- fragment «name-handle-new-and-of» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="427-446" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

`of` is where the charter leaves the identity namespace. `TaskName` is chapter
4's type and has exactly two variants: `Brief`, which is the `BRIEF.md` charter
and carries no ordinal, no key and no parts, and `Positioned`, which carries all
three. A handle needs a key, `Brief` has none, and `of` returns `None` rather
than inventing one — the charter's subject is the node that contains it, so its
identity is that node's. `the_brief_has_no_handle` below is the assertion.

<!-- fragment «name-handle-parse» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="447-498" parent="kind-slug-and-handle" -->
````rust
    /// Read a handle back out of its rendering.
    ///
    /// The inverse of [`Handle::render`], and one of [`peel_key`]'s three callers
    /// beside [`split_shape`] and [`terminal_key`] — so a handle, a filename and a
    /// bare reference find the key by one rule and cannot disagree.
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

````
<!-- /fragment -->

The asymmetry the doc comment argues for is the chapter's rule seen from the
reference side. `TaskName::parse` — chapter 4's, and the only route from a
filename to a parsed name — is canonical: it refuses a name whose spelling is not
the one the renderer would have written, because two spellings of one filename
are two files on disk sharing one key and one position. A handle is never on
disk, so the argument does not reach it, and `Handle::parse` accepts `a-k007` for
key 7 and normalises it away on the way back out. `parse_ref`, at
`crates/grove-loop/src/task_tree.rs` line 986, is the reference grammar's own
front door and is lenient in the same way for a bare key, which chapter 9 reads.

The strictness in the other direction is what sends one caller elsewhere.
`Handle::parse` refuses a head that is not a `Slug`, so an operator pasting a
retired leaf's whole stem — `01-DONE-impl--build-k5` — would be refused for a head
that was never going to be a slug. `resolve`'s bare-slug fallback therefore asks
`terminal_key` instead, a public function at
`crates/grove-loop/src/task_name.rs` line 991 that answers *does this end in a
key* and requires nothing of what precedes it. Chapter 4 defines it and chapter 9
reads the fallback that calls it.

**One peel, and the three callers that ask it.** A terminal `-k<digits>` is
taken apart in exactly one function, `peel_key`, whose `strip_suffix` is one of
the crate's two reads of `KEY_MARK`; the other is `Handle::render` below, and the
third mention of the constant is its own definition. Its callers are
`split_shape` for filenames at line 973, `Handle::parse` at line 478, and
`terminal_key` at line 992, which chapter 4 reads. That is what the clause the
comment ends on rests on: a handle, a filename and a bare reference cannot
disagree about where the key is, because none of the three finds it and all three
ask `peel_key`.

<!-- fragment «name-handle-accessors-and-render» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="499-521" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

`render` is the single `write!` the header claimed, and its signature is why the
claim holds rather than merely being policy. It takes a slug and a key rather
than `&self`, so chapter 4's `Display for TaskName` can end both of its positioned
arms in a call to it — at lines 650 and 655 — without building a `Handle` it does
not need. A `Handle` value is not the point; one `write!` is. Both accessors are
`const fn`, which costs nothing here and is the same treatment `Parts` gives its
three below.

<!-- fragment «name-handle-display» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="522-527" parent="kind-slug-and-handle" -->
````rust
impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::render(f, &self.slug, self.key)
    }
}

````
<!-- /fragment -->

That is the third caller of `render` and the shortest. Together with the two arms
in chapter 4 it is the whole set: three call sites, one `write!`, and nothing
else in the crate's production code that spells `<slug>-k<key>`.

<a id="the-parts"></a>
## The parts, and the species that follows from them

The last type in the block is the one the store asks grove for by name. Where
`Kind`, `Slug` and `Handle` are grove's own vocabulary, `Parts` is the associated
type the store's `EntryName` trait carries, and it is how grove tells the store
what kind of thing it is looking at without the store having to know.

<!-- fragment «name-parts» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="528-552" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

The asymmetry the second paragraph describes is worth reading against chapter 2's
`Outcome`. A node directory has no outcome because a node is done when no live
leaf remains anywhere beneath it, which is a fact about a subtree rather than a
mark on a directory — and the `Node` variant expresses that by having no
`outcome` field at all rather than a field constrained to one value. What the
type cannot hold, the bytes can still spell, which is why chapter 2's
`a_node_wearing_an_outcome_infix_is_malformed` exists: the refusal has to be in
the parse, because a directory named `01-DONE-thing-k2` is a name a filesystem
will accept and this enum has nowhere to put it.

<!-- fragment «name-parts-methods» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="553-590" parent="kind-slug-and-handle" -->
````rust
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

````
<!-- /fragment -->

`slug` is the accessor that makes the variants interchangeable where a caller
needs only the human-facing half, and `Handle::of` above is its first customer.
`species` returns `PositionedSpecies` rather than the store's wider `Species`,
and the doc comment gives the reason in one line: parts belong to a positioned
name, and the store's `Species` has a third case for the distinguished child,
which by construction has no parts to ask. That is the same move `Handle::of`
makes with its `Option`, in the type system rather than in a return value.

<a id="the-slug-rule"></a>
## The slug rule, unchanged

The first of this chapter's two test blocks is twenty-nine lines and holds one
test. It is the file's own labelled section for the slug, and what it pins is
that the rule did not change when `open-kind-k20` opened the kind: a slug is
still the shape it always was, and the kind is now the same shape.

<!-- fragment «slug-rule-tests» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="1551-1579" parent="source-task-name" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** Six strings are slugs and twelve are not, and the six
cover the cases a reader would doubt: a single character, a plain word, a word
with an internal dash, a token mixing letters and digits, a token that is all
digits, and `k29`, which begins with the key marker's letter and is a perfectly
ordinary slug because the marker is `-k` and terminal. The twelve refusals reach
all five clauses of `refuse_token`: the empty string, a leading and a trailing
dash, the separator both embedded and alone, an uppercase letter, an underscore,
a dot, a slash, and the three reserved words. The inline comment on
`a--b` states the one thing the mechanism alone would not tell a reader: the split
would still be unambiguous with a `--` inside a slug, since it takes the first
one, and the rule refuses it anyway because the specification's rule is that
neither token carries one.

**What it would still pass under.** Both loops assert only `is_ok` and `is_err`,
never which clause fired, so the test cannot distinguish a rule with five guards
from a rule with one that refused everything the same way. Two of the eleven
fixtures are refused for a different reason than their placement suggests. `--`
is refused by the leading-dash guard, which runs before the separator guard, so
it never reaches the clause it appears to be there for. And `BRIEF`, `DONE` and
`ABANDONED` are refused by the reserved-word guard only because that guard runs
first: all three are uppercase, so the character-set guard would refuse them
anyway, and deleting the reserved-word clause entirely would change the message
an operator reads and not the verdict this test asserts. Nothing here shows that
clause is load-bearing on acceptance, because it is not. The test is also silent
about `Kind`: it exercises `Slug::new` alone, and *one rule for both words* is a
fact about `refuse_token` having one body, which chapter 2 read and no test in
this file asserts over the pair.

**`BadKind` is the variant a refused kind arrives in, and it carries a rename.**
Under the closed kind set it was `TaskNameError::UnknownKind`, and
`open-kind-k20` replaced membership refusal with shape refusal; `BadKind`'s own
doc comment at line 689 records that change, calling it *a shape refusal, not an
unknown-kind one*. The point the inline comment makes does not depend on which
name carries it: a slug holding `--` would move where the name splits, and the
refusal would quote a token nobody wrote.

<a id="the-handle-is-the-identity"></a>
## The handle owns the grammar, asserted

The second test block is 164 lines and holds six tests. It is where the module
header's structural claim stops being a claim, and it is the only place in this
chapter that reaches for a name rather than a part.

<!-- fragment «handle-grammar-tests» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="1580-1743" parent="source-task-name" -->
<!-- insert «name-tests-ends-in-handle» -->
<!-- insert «name-tests-brief-no-handle» -->
<!-- insert «name-tests-handle-round-trip» -->
<!-- insert «name-tests-same-peel» -->
<!-- insert «name-tests-refused-handle» -->
<!-- insert «name-tests-lenient-strict» -->
<!-- /fragment -->

The first test is the chapter's structural claim, and it is the one place
`TaskName::compose` appears. `compose` is chapter 4's implementation of the
store's `EntryName::compose`: it takes an ordinal, a key and a `Parts` and returns
a `TaskName::Positioned` holding all three, with no validation, because every part
handed to it was validated when it was built. That is what lets this test assert
over names it *builds* rather than names it parses.

<!-- fragment «name-tests-ends-in-handle» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="1580-1630" parent="handle-grammar-tests" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** For four composed names — three leaves and a node,
across all three outcomes and across a slug that itself contains `-k9` — the
rendering ends in the rendering of the handle `Handle::of` derives from the same
name, with only `.md` allowed after it. The second assertion takes the byte range
that suffix occupies and parses it back, so the tail is the handle by the
grammar's own reading rather than by coincidence of bytes. Together they are the
consequence the header claimed: a second spelling of `<slug>-k<key>` anywhere in
`TaskName`'s `Display` fails here the moment the two spellings disagree.

**What it would still pass under.** The assertion is `ends_with`, so everything
before the handle is unconstrained: `Display` could write the wrong ordinal,
omit the outcome infix or misspell the kind and this test would not notice. It
detects drift between two spellings, not the existence of a second spelling — a
duplicate `write!` producing identical bytes passes, and what rules that out is
reading `render`'s three call sites rather than running this. The `.md` suffix is
stripped with `strip_suffix(".md").unwrap_or(&rendered)`, so a leaf whose
rendering lost its suffix entirely would still pass. Every name here is composed
rather than parsed, so nothing about canonicity, about padding or about any name
that has been on disk follows from it. And the one fixture whose slug contains
the key marker is the node, so a leaf-only failure on that case is outside what
these four cover.

<!-- fragment «name-tests-brief-no-handle» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="1631-1637" parent="handle-grammar-tests" -->
````rust
    /// The charter is the one name with no key, and therefore no identity of its
    /// own — `of` says so rather than inventing one.
    #[test]
    fn the_brief_has_no_handle() {
        assert_eq!(Handle::of(&TaskName::Brief), None);
    }

````
<!-- /fragment -->

**What it establishes.** The charter has no handle, and `of` says so with `None`
rather than fabricating one from the node around it.

**What it would still pass under.** Read alone it would pass while `of` returned
`None` for every name; only the test above rules that out, and the two are halves
of one statement about `of` rather than independent claims. It also says nothing
about `Handle::new`, which is public and will build a handle for any slug and key
a caller supplies — the guarantee is that nothing *derives* an identity for the
charter, not that no such value can exist.

<!-- fragment «name-tests-handle-round-trip» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="1638-1657" parent="handle-grammar-tests" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** `Handle::parse` inverts `Handle`'s `Display` for four
texts, and the fourth is the one the terminality rule exists for: `task-k9-k3` is
the slug `task-k9` at key 3, not the slug `task` at anything. A rule that took the
first `-k` instead of the last would fail on it, which makes this fixture the
discriminating one rather than a fourth example.

**What it would still pass under.** All four keys are already canonical decimal
with no leading zero, so the leniency the doc comment argues for is invisible
here — the test below is what pins it. Every case starts from text and ends at
text, so a `Handle::new` that normalised or rejected a slug on the way in would
not be seen. And the third assertion compares against the input, which means it
holds only for strings `render` would have written; the test carries no case where
`parse` accepts something and renders it differently, again leaving that to the
last test in this block.

<!-- fragment «name-tests-same-peel» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="1658-1679" parent="handle-grammar-tests" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** The filename route and the handle route reach the same
`Handle` for three names — one with a slug containing `-k9`, one carrying a `DONE`
infix, and one node directory — which is the carried example's step for this
chapter, run three times. It is asserted over the pair rather than over either
side because the failure it replaces was two implementations of the peel agreeing
on the easy cases and diverging on the terminal one.

**What it would still pass under.** `Handle` compares only a slug and a key, so
the assertion is blind by construction to everything the filename carries and the
handle does not — the position, the outcome and the kind could all be read wrong
and this would still hold. That is the chapter's rule, so it is a property rather
than a gap, but it does mean this test says nothing about the parse beyond the two
fields. All three filenames are canonical, so no refusal path is exercised. And
the two routes could share one wrong rule and agree: what this pins is that they
agree, and what makes the shared rule right is the round-trip fixtures above.

<!-- fragment «name-tests-refused-handle» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="1680-1713" parent="handle-grammar-tests" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** Each of the three refusal variants is reached by a
string that reaches it for the stated reason, and each message names what a
handle should have been. The five fixtures cover more ground than the three
variants suggest: `build` has no terminal digits at all, `build-14` has trailing
digits with no marker before them, `a-k99999999999` overflows `u32`, `Bad-Slug-k2`
fails the character set, and `-k3` peels to an empty slug, which is the empty-token
clause of `refuse_token` reached through this type.

**What it would still pass under.** Only the first fixture asserts an exact error
value; the rest use `matches!` on the variant, so the payload fields could carry
anything. The message assertions are `contains`, so a message that said the right
thing and then also said a wrong thing would pass, and the `lowercase` check on
`Bad-Slug-k2` would pass on any message that used the word — including one that
had refused it for the wrong clause. Nothing here asserts *which* clause of
`refuse_token` produced the nested `TokenError` for either bad-slug fixture, so
the two are indistinguishable to the test even though the code refuses them for
different reasons.

<!-- fragment «name-tests-lenient-strict» owner="the-handle-not-the-position" source="crates/grove-loop/src/task_name.rs" lines="1714-1743" parent="handle-grammar-tests" -->
````rust
    /// The two ways `parse` departs from the `task_tree::handle_key` it
    /// replaced, pinned because they are the only behaviour this leaf moved.
    ///
    /// Lenient where `handle_key` was, on the key's spelling — a handle is a
    /// reference a human types and never a name on disk, so canonicity has no
    /// argument here. Stricter where `handle_key` looked at nothing, on the
    /// slug — `handle_key` answered *key 3* for four references no entry could
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
````
<!-- /fragment -->

**What it establishes.** The two ways `Handle::parse` differs from the function it
replaced, pinned in both directions: `a-k007` and `a-k0` are accepted for keys 7
and 0, and `a-k007` renders back as `a-k7`, which is the one assertion in this
file that shows the parse is deliberately not canonical. Four references that end
in `-k3` and could never be an entry's name are refused for their slug.

**What it would still pass under.** The leniency is exercised only through
leading zeros, so a parse that also accepted whitespace or a sign would pass. The
four refused references are asserted only to be `BadSlug`, and they trip three
different clauses of `refuse_token` — `-k3` the empty-token clause, `A-k3` and
`a_b-k3` the character set, `DONE-k3` the reserved words — so the test cannot show
that any one of those clauses is reached. And the whole comparison is with a
function that no longer exists: `task_tree::handle_key` was deleted, so what this
test protects is a behaviour, not a difference a reader can run both sides of.

**Four references, and `handle_key` would have answered every one of them.**
`-k3`, `A-k3`, `DONE-k3` and `a_b-k3` are all refused here, and all four would
have answered *key 3* under a function that did not look at the slug, since each
ends in a terminal `-k3`. That is what the fixtures show, and it is the claim the
doc comment makes: the strictness is on the slug, and every slug on disk went
through `Slug::new`.

The named parts are defined, and the handle's structural claim is asserted rather
than promised. What is still missing is the name that carries them: nothing so far
has read a string off a disk. `TaskName`, the `EntryName` implementation that is
the whole seam, the conformance kit the store runs against grove's grammar and the
canonicity argument the module header opened on are chapter 4's, and every route
from a filename into the types this chapter defined runs through them.

[Previous: The tokens, and the four verdicts](02-the-tokens.md) | [Contents](README.md) | [Next: The name, and canonicity](04-the-name.md)

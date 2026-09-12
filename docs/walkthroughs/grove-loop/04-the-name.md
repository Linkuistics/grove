# The name, and canonicity
<!-- book-page id="the-name" slice="canonical-or-nothing" order="4" -->
[Previous: Kind, slug, handle](03-kind-slug-handle.md) | [Contents](README.md) | [Next: Opening, contention and refusal](05-opening.md)

<a id="canonical-or-nothing"></a>
## The rule: `format(parse(f)) == f`, or one entity occupies two files

Chapters 2 and 3 read the grammar's punctuation and its named parts. Neither has
read a string off a disk: every name so far was either handed to a token
constructor or built by a test. This chapter is where a filename becomes a
`TaskName`, and the rule it exists for is the one the module header stated in
chapter 2 and explicitly deferred to here:

> Whatever grove parses out of a filename must render back to the bytes it was
> parsed from. A grammar that accepts a spelling its own renderer would not have
> written admits two files on disk that are **one entry, sharing a key and a
> position**.

That is the store's canonicity obligation, and it is grove's to discharge because
the store cannot. `ordinal-fs-tree` owns an ordered tree and a key; it has no
grammar, so it cannot look at `05-impl--a-k1.md` and `5-impl--a-k1.md` and say
whether they are two entries or one entry written twice. Grove owns the grammar,
so grove owns the answer, and the price of owning it is the seven lines at the end
of `parse` and the conformance kit this chapter carries.

The carried example is the first leaf a new grove holds, taken the whole way in
and the whole way back out. The input is a filename and what the listing found
under it; the observable end is a `Verdict::Entry` whose rendering is the input,
byte for byte.

```text
"01-requirements--plan-k1.md", Found::File
  │
  ├─ not _BRIEF.md, so not the charter
  ├─ strip ".md"          →  stem "01-requirements--plan-k1", declares a leaf
  ├─ split_shape          →  digits "01" · middle "requirements--plan" · key "1"
  ├─ parse both numbers   →  ordinal 1, key 1        (either failing ends here)
  ├─ Outcome::strip       →  Live, "requirements--plan"
  ├─ split at the first --→  kind "requirements", slug "plan"
  ├─ compose              →  Positioned { ordinal: 1, key: 1, parts: Leaf { … } }
  ├─ render it back       →  "01-requirements--plan-k1.md"
  ├─ compare with input   →  equal, so not NotCanonical
  └─ species vs found     →  a leaf requires a file, and a file was found
                             ⇒ Verdict::Entry
```

The figure is the chapter in one column: read it downwards for what `parse` does,
and note that the two steps a reader would not predict are the last two. Nothing
in the first six steps is canonicity — they are the grammar chapters 2 and 3
already read, applied in order. Canonicity is the ninth step, and it is a
comparison rather than a rule about any one field.


Two refusals illustrate the boundary. An unpadded position can be parsed
and rendered, so `NotCanonical` supplies the padded filename. An unrepresentable
position or invalid key cannot produce a valid Grove name, so `InvalidName`
states the required grammar and range without inventing a replacement.


The withdrawn model is what makes this concrete rather than theoretical. Before
this domain existed grove's grammar was lenient on padding: it accepted a
hand-typed `5-…` and its renderer wrote `05-…`. Under that grammar an operator
who created `5-impl--a-k1.md` beside the `05-impl--a-k1.md` grove had written had
two files on disk that the tree read as one entry, at one position and under one
key. Every operation would then plan against a snapshot in which one of the two
was invisible.

| Byte run of `01-requirements--plan-k1.md` | Written by | Read back by |
|---|---|---|
| `01` | the leaf arm's `write!` at line 567 | `split_shape`'s leading digit run |
| `requirements` | `kind.label()` | the head of the first `--` split |
| `--` | `SEPARATOR` | `split_once(SEPARATOR)` |
| `plan-k1` | `Handle::render` | `peel_key`, then `Slug::new` |
| `.md` | the leaf arm's `f.write_str(".md")` | `strip_suffix(".md")` |

The table is what `format(parse(f)) == f` means concretely for one name: each row
is a byte run, the one place that writes it, and the one place that reads it
back. Canonicity is the claim that the right-hand column composed with the middle
one is the identity, and the check at the end of `parse` establishes it by
performing both rather than by reasoning about the pairs.

This chapter owns 773 lines of `task_name.rs` in 3 blocks.
The source index records their current ranges; the fragments below reconstruct
every owned byte.

<a id="the-parsed-name"></a>
## The name, and the rendering that is `format`


`TaskName` distinguishes positioned entries, the root file and titled node
files. Leaf handles come from one positioned name; node handles require the
directory name together with its titled file.


The composite below reassembles this chapter’s production block. Its
fragment directive records the current source range, and each inserted
fragment is explained where the corresponding behavior is introduced.


<!-- fragment «the-task-name» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="506-953" parent="source-task-name" -->
<!-- insert «name-task-name» -->
<!-- insert «name-task-name-display» -->
<!-- insert «name-task-name-error» -->
<!-- insert «name-task-name-error-display» -->
<!-- insert «name-parse-charter» -->
<!-- insert «name-parse-shape» -->
<!-- insert «name-parse-parts» -->
<!-- insert «name-parse-canonicity» -->
<!-- insert «name-entry-name-rest» -->
<!-- insert «name-refusal-helpers» -->
<!-- insert «name-uncomputable-canonical» -->
<!-- insert «name-split-shape» -->
<!-- insert «name-terminal-key» -->
<!-- insert «name-peel-key» -->
<!-- /fragment -->

The enum is three variants and the derives stop at `Clone`, `Debug`, `PartialEq`
and `Eq`. A positioned name carries its ordinal, its key and its parts in one
variant rather than in three independent fields, and the charter carries none of
them; the doc comment names the obligation that shape discharges and points at
the trait method that states it.

<!-- fragment «name-task-name» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="506-534" parent="the-task-name" -->
````rust
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

````
<!-- /fragment -->

The obligation the comment names — *a name is positioned or distinguished, never
neither* — is one of the seven `EntryName` obligations, and it is the first of
the two Rust constraints rather than the kit samples. The store's `NameView` carries the
triple and the positioned-or-distinguished choice in **one** returned value, so a
domain cannot return a leaf with no ordinal or a distinguished name carrying a
triple. This enum has the same shape one level up: the three fields live inside
`Positioned` and `Brief` has no room for them, so the malformed combination is
not a value this type can hold. The store's own comment adds the limit — the type
shape does not make the choice stable across calls, and no finite sample can
prove the absence of hidden state — which is why the obligation is listed as
constrained rather than discharged.

`Display` is `format`, and it is the half of the round trip the whole chapter is
named for. Both of its arms reach `Handle::render` — the node arm ends there, the
leaf arm writes `.md` after it — and chapter 3 established that call is the only
`write!` in the crate spelling `<slug>-k<key>`.

<!-- fragment «name-task-name-display» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="535-585" parent="the-task-name" -->
````rust
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

````
<!-- /fragment -->

The first inline comment is the canonical rule stated exactly, and it is worth
reading as a rule about `{:02}` rather than about names. `{:02}` is a **minimum**
width, so position 100 renders `100` and not `0100`; the canonical spelling is
therefore *zero-padded to at least two digits, and carrying no other leading
zero*. That is not a rule anyone wrote down separately and then implemented — it
is whatever this format string does, read backwards, and the comparison at the
end of `parse` is what makes it binding. A padding rule added here later cannot
escape that comparison, because the comparison is against this function's output
and not against a specification of it.


The leaf renderer writes the position, outcome and kind, then calls
`Handle::render` and appends `.md`. The node renderer writes only position and
the shared key token. A node handle is assembled from two filenames and need
not be a substring of either. `render_key` is shared by handle and directory
rendering, keeping their terminal key spelling consistent.


The third comment states the fact `parse` has to reconcile: the `.md` suffix is
what the **name** declares its species to be, and a listing reports what is
actually there. Nothing forces those to agree, which is why the last thing
`parse` does is ask.

<a id="what-a-refusal-carries"></a>
## What a refusal carries back

Six variants, and the doc comment states why each carries advice rather than a
diagnosis: the store halts on a `Malformed` verdict wherever in the tree it sits,
so whoever hit it has a frozen tree and needs a next step, not a category.

<!-- fragment «name-task-name-error» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="586-650" parent="the-task-name" -->
````rust
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

````
<!-- /fragment -->


The errors distinguish malformed owned syntax, noncanonical positions, token
failures, species contradictions and invalid distinguished-file sets. Each
refusal carries the offending spelling or the level’s actual file names.


| Variant | Boundary |
|---|---|
| `InvalidName` | owned syntax or key cannot form a canonical name |
| `NotCanonical` | parsed name renders with a different position spelling |
| `MissingSeparator` | leaf has no kind/slug separator |
| `BadKind`, `BadSlug` | token construction |
| `SpeciesMismatch` | listing species contradicts the name |
| `NodeFiles` | required file cardinality or root/node placement |


`NotCanonical` can offer the spelling obtained by rendering a parsed name.
`InvalidName` instead states the required grammar when no valid name can be
constructed. `NodeFiles` belongs to the complete-level callback rather than the
individual parser.


Two variants nest a `TokenError` — the refusal chapter 2 read — and neither
chains it. `BadKind` and `BadSlug` each carry the offending token beside the
error explaining it, so the message can name both.

<!-- fragment «name-task-name-error-display» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="651-711" parent="the-task-name" -->
````rust
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

````
<!-- /fragment -->


The messages state the grammar they require or delegate to the token error
that names the broken clause. `NotCanonical` explains why alternate spellings
can conceal duplicate identity; `SpeciesMismatch` explains why skipping the
entry could hide everything beneath it. A missing node file also supplies
conditional recovery advice without claiming to have inspected a sibling.


`SpeciesMismatch`'s message is the only one that **computes** a value rather
than quoting one it was handed: `declares.requires()` is the store's, and it
renders what the declared species demands of the listing. That is also the only
refusal `parse` can raise for the charter, at line 733 where a `_BRIEF.md` the
listing found as a directory is refused before the positioned grammar is
reached.

`impl std::error::Error for TaskNameError {}` follows with no `source`, exactly
as `TokenError` and `HandleError` do. The nested `TokenError` is rendered inline
with `— {error}` and is not reachable through the error chain, which is the
one-error rule chapter 1 read applied to a type that is not `grove_loop::Error`
at all.

<a id="the-seam"></a>
## The seam: `parse`, and the seven lines canonicity costs

`impl EntryName for TaskName` is the whole seam between the task tree and the
library that drives it, and `parse` is the half of it that reads. The first
fragment carries the impl header, the two associated types, and the charter
branch that has to run before anything positioned is attempted.

<!-- fragment «name-parse-charter» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="712-737" parent="the-task-name" -->
````rust
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
````
<!-- /fragment -->

`type Parts = Parts` and `type Err = TaskNameError` are the two the trait leaves
to the domain: everything the library does not understand, and whatever the
domain says when it refuses. The library's bounds on `Parts` are `Clone + Eq`,
which is the whole of what it may do with one — copy a value it already holds,
and compare two of them — so chapter 3's `Parts` needs no constructor the store
could reach.


Underscore-prefixed names are handled before positioned names. `_BRIEF.md`
is the root-file variant; another underscore name must end in `.md` and contain
a valid slug. The level callback then enforces placement: the root marker only
at the root, and one titled file in each positioned node.


Even the charter is asked the species question.
`disagreement(Species::Distinguished, found, name)` is the same call the
positioned path ends on, and it refuses a `_BRIEF.md` the listing found as a
directory — the one shape in which a node's charter could hide a subtree.

The second fragment decides whether the name belongs to Grove at all and whether
both number fields are numeric.

<!-- fragment «name-parse-shape» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="738-770" parent="the-task-name" -->
````rust
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
````
<!-- /fragment -->


A leading digit establishes Grove ownership before shape parsing. The `.md`
suffix selects the leaf grammar; a directory name contains only the position
and key. Failure to split an owned spelling is malformed syntax, never a
foreign entry that the reader could skip.



The position parser checks representability, while the shared `parse_key`
checks positivity, canonical decimal spelling and representability. A position
that can be represented but needs padding reaches the render comparison; an
invalid key or unrepresentable position receives `InvalidName`.


The third fragment builds the parts, and it is the grammar chapters 2 and 3
defined, applied in order.

<!-- fragment «name-parse-parts» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="771-805" parent="the-task-name" -->
````rust

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
````
<!-- /fragment -->


Only leaf names use the stripped outcome and parse a kind and slug. A node
has no such fields: its shape parser accepts exactly position and key, and its
parts are `Parts::Node`. An outcome or title in a directory spelling therefore
fails shape parsing.



The leaf middle splits at the first `--`. Both kind and slug may contain
single hyphens, so this separator makes the boundary independent of a registry
of known kinds. The two token constructors validate their own fields.


The second comment states the distinction made by the two-stage failure: a name
with no separator is *shaped* wrong and can only be told to rename, while a name
with a separator has a single token to quote back. Those are the
`MissingSeparator` and
`BadKind` variants, and chapter 2 read both being asserted.

The fourth fragment is canonicity, and the check itself is seven lines of it.

<!-- fragment «name-parse-canonicity» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="806-857" parent="the-task-name" -->
````rust

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

````
<!-- /fragment -->

`parts_species` is read off `parts` and bound **before** `parts` is moved into
the `Positioned` variant, which is why the species question at the end can be
asked without taking the name apart again. `PositionedSpecies::species()` widens
it to the store's `Species`, whose third case is the distinguished child that by
construction has no parts.

The comment above the comparison is the chapter's rule in the form the code takes
it: whatever was parsed, render it, and refuse the input when it is not what
grove writes. Three properties follow from its being a comparison rather than a
rule, and together they are why it sits here rather than distributed across the
field parsers.

- **It is over the whole grammar.** There is no padding rule, no ordering rule
  and no suffix rule to keep in step with the renderer, because the renderer is
  what it compares against.
- **A later rule cannot escape it.** A field-level rule added to `Display`
  changes what `canonical` is, so the comparison changes with it. The comment
  states this as *a padding or ordering rule added later cannot escape it*, and
  the mechanism is that `canonical` is `parsed.to_string()` and nothing else.
- **It costs one rendering per name read.** Every entry in every listing grove
  parses is rendered once in order to be compared, which is the visible price of
  owning a grammar the library cannot check.

One consequence of the comparison reaches every test in this file and is easy to
miss. Because `parse` returns `Entry` only after `parsed.to_string() == name`,
**any test that reaches an entry at all has already asserted that name's whole
rendering.** The `entry` helper panics on anything but `Verdict::Entry`, so a
test that binds only a key and a slug still fails if `Display` writes the wrong
ordinal, drops an outcome infix or misspells a kind — not through its own
`assert_eq!`, but because the name would have come back
`Malformed(NotCanonical)`. Adding one to the ordinal inside `Display` turns
twelve of this file's twenty-five tests red. So what the tests reproduced below
are blind to is never a single misreading; it is a **coordinated** change to
`Display` and `parse` together, or a case no fixture carries.

The species question is last, and it is last because it is the only one needing a
fact from outside the string. Everything above it is about bytes; `disagreement`
compares what the name declares against what the listing found, and a name that
is perfectly canonical and contradicted by its own directory entry is still
`Malformed`.

<a id="the-rest-of-the-seam"></a>
## The other four methods, and what each declines to do

The rest of the trait is four methods in thirty-two lines, and contains no
validation at all. That is the point of it: every part these methods place was validated when it was
built, and re-checking here would be a second opinion about a question already
settled.

<!-- fragment «name-entry-name-rest» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="858-885" parent="the-task-name" -->
````rust
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

````
<!-- /fragment -->

`compose` is the store's constructor for a name, and it is the one chapter 3's
`every_leaf_name_ends_in_its_own_handle` builds its four fixtures with. It
takes an ordinal, a key and a `Parts` and returns a `Positioned` holding all
three, unexamined — which is what makes that test's assertion about `Display`
rather than about parsing. There is no arm for the charter, because the charter
has no triple to compose from.

`distinguished` returns `Some(Self::Brief)`, and the `Option` is the trait's: a
domain may have none, and the store's doc says `None` means promotion is refused
rather than guessed at. Grove has one, and the store's wording for why this
method exists is exact and narrower than it first looks — a distinguished child
carries neither an ordinal nor a key, so it can never come out of `compose`, and
this is *the only way the library can name one*. It is not the only way anything
can: `pub const BRIEF` is public, chapter 2 read it, and `parse`'s own first
comparison is against that constant rather than against this method.

`view` is the method the **first** of the two Rust-constrained obligations lives
on; the second, *the species follows from the parts*, is written on
`positioned_species` below, and the trait's own header names both methods as
carrying them. It returns one value rather than the four accessors an earlier
draft of the trait had — `ordinal()`, `key()`, `parts()` and `species()`, three
of them `Option`s beside an independent species —
and the store's own comment gives the witness: independent accessors admit a leaf
with no ordinal, and a name carrying a triple while claiming to be the
distinguished child. `NameView` carries the triple and the
positioned-or-distinguished choice together, so neither malformed value can be
returned. Grove's implementation is a two-arm match with nothing in it, which is
what a domain whose type has the same shape looks like.

`positioned_species` delegates to `Parts::species`, the `const fn` chapter 3
read. Between it and `view`, the store derives the `species()` and `triple()`
readings every operation uses, and a domain cannot override the derived readings
independently — which is what makes *the species follows from the parts* a
constraint rather than an agreement.

<a id="under-the-seam"></a>
## Under the seam: the helpers, and the one peel


Below the implementation, small helpers centralize species errors, slug
errors, unrepresentable-number refusals, leaf shape splitting and key parsing
and rendering. `terminal_key` is the public lookup helper. The token helpers
keep names and handles on one key grammar.


<!-- fragment «name-refusal-helpers» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="886-902" parent="the-task-name" -->
````rust
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

````
<!-- /fragment -->


`disagreement` compares the name's declared species with the listing's
actual species. It rejects a directory wearing a leaf or node-file name, and a
file wearing a node-directory name. `bad_slug` carries the original filename,
the offending token and its token error for both leaf slugs and node-file slugs.


The third is the refusal with nothing to advise.

<!-- fragment «name-uncomputable-canonical» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="903-910" parent="the-task-name" -->
````rust
/// A canonicity refusal whose advice cannot be computed: the numbers did not fit
/// in 32 bits, so there is no spelling to offer back.
fn uncomputable_canonical(name: &str) -> TaskNameError {
    TaskNameError::InvalidName {
        name: name.to_string(),
    }
}

````
<!-- /fragment -->


`uncomputable_canonical` returns `InvalidName`: the spelling cannot be
represented, so there is no replacement filename to suggest. `NotCanonical`
remains reserved for a parsed name whose renderer provides an actual target.


The fourth is where a stem becomes three pieces.

<!-- fragment «name-split-shape» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="911-923" parent="the-task-name" -->
````rust
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

````
<!-- /fragment -->

Both boundaries are unambiguous for different reasons, and the doc comment states
each. The position ends at the **first** `-` because a position is pure digits,
so nothing before that dash could be anything else; the guard on the next line is
what makes that true rather than assumed, since `find('-')` succeeding says
nothing about what precedes it. The key is the **terminal** `-k<digits>`, which
is what keeps a slug containing `-k9` readable, and it is found by `peel_key`
rather than here.


The leading digit already establishes ownership before `split_shape`
runs. A missing key, malformed directory position or incomplete kind/slug
separator therefore produces a refusal, never a foreign-entry skip. The leaf
middle is split at the first `--`; directory names contain no middle or slug.


The fifth and sixth fragments are the one peel, and they read in the opposite
order to the call graph: the public narrowing first, then the private primitive
it goes through. `terminal_key` delegates suffix extraction to `peel_key`
and digit validation to `parse_key`. Each helper's comment belongs to the
function immediately below it.

<!-- fragment «name-terminal-key» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="924-931" parent="the-task-name" -->
````rust
/// Read a terminal canonical key token. Callers still validate the title when
/// using this as a full handle rather than a bare key reference.
#[must_use]
pub fn terminal_key(reference: &str) -> Option<Key> {
    let (_, digits) = peel_key(reference)?;
    parse_key(digits)
}

````
<!-- /fragment -->

`terminal_key` is the public narrowing, and what it declines to require is the
whole of it: nothing before the key has to be a slug. That is what lets
`resolve`'s bare-slug fallback take an operator's pasted stem —
`01-DONE-impl--build-k5` — and answer key 5, where `Handle::parse` would refuse a
head that was never going to be a slug. The body is two statements and neither
validates anything: peel, and then `digits.parse().ok()`, which is one of the
three judgements the next fragment's comment enumerates.

<!-- fragment «name-peel-key» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="932-953" parent="the-task-name" -->
````rust
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

````
<!-- /fragment -->


`peel_key` finds the final digit run and the preceding `-k`. The shared
`parse_key` then checks canonical positive decimal spelling and `u32` width.
Names, handles and terminal-key lookup use this boundary; their callers
validate the fields before the key according to their own role.



Invalid key digits become `InvalidName` in filename parsing, `BadKey` in
handle parsing, or `None` in terminal-key lookup. These are different reports
of the same canonical-key rule, not different accepted key spellings.


Eight lines of body, and the terminality rule is line 950. The digit run is
found from the **end** — `bytes().rev().take_while(u8::is_ascii_digit)` — so
`task-k9-k3` yields the digits `3` and the text `task-k9-k`, and the
`strip_suffix(KEY_MARK)` then leaves the slug `task-k9`. A rule that searched
forwards for `-k` would answer the slug `task` and the key `9`, which is the
divergence chapter 3's `a_handle_and_a_filename_peel_the_same_key` exists to
rule out for the pair of routes and this function rules out for all three.


The no-trailing-digits guard returns `None`. Filename parsing has already
classified digit-prefixed input as owned, so that result produces a malformed
name refusal. `README.md` is disclaimed before these helpers run. A missing
key cannot make a positioned-looking entry disappear from the snapshot.


That completes the production block. All 1,020 production lines of
`task_name.rs` are now read across three chapters, and what remains of the file
is its 694-line inline test module.

<a id="the-conformance-kit"></a>
## The conformance kit, and the fixture that has to pose the question

The test module opens with five helpers the whole file uses and then with the
kit: the library's own checker, pointed at grove's grammar. This is the second
block, 157 lines, and it is where canonicity stops being grove's private
comparison and becomes an obligation something outside grove can grade.

<!-- fragment «name-test-support-and-kit» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="954-1168" parent="source-task-name" -->
<!-- insert «name-tests-support» -->
<!-- insert «name-tests-kit-fixture» -->
<!-- insert «name-tests-conforms» -->
<!-- insert «name-tests-kind-shapes» -->
<!-- insert «name-tests-undeclared-kind» -->
<!-- /fragment -->

The five helpers are the vocabulary every test in the file is written in, and all
five were named in chapter 2 before their definitions were read: `verdict`,
`entry` and `malformed` at its four-verdicts section, `a_kind` and `slug` at its
refusals section. Each has a row in the early-use ledger for that reason.

<!-- fragment «name-tests-support» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="954-1028" parent="name-test-support-and-kit" -->
````rust
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

````
<!-- /fragment -->

`a_kind` and `slug` each take a label, put it through the validating constructor,
and panic on failure — so an invalid fixture is a test bug that fails where it
was written, and not, as the comment on `a_kind` says, a compile error somewhere
else. That was a compile error before `open-kind-k20`: a kind was an enum, so a
test naming one grove did not have could not be built. Now it is a token, and the
panic is what replaces the compiler.

`verdict` is a one-line alias for `TaskName::parse` and the only place in the
test module that names it. `entry` and `malformed` narrow a `Verdict` to the arm a
test wants and panic on any other, and both carry `#[track_caller]`, so a
`01-…` fixture that stops being an entry reports the assertion's line rather than
this helper's. There is no helper for `Verdict::Foreign` or `Verdict::Reserved`:
`a_name_that_is_not_task_shaped_is_foreign` compares against `Verdict::Foreign`
directly, and chapter 2 established that this domain never returns `Reserved`
at all.

The fixture is next, and it is the part of the kit a reader should spend time on.
The kit's own header states why: a checker that reports violations reads exactly
the same when it is handed nothing to check, so it also reports which obligations
were never **exercised** — and that second finding distinguishes *no samples*
from *samples*, not *samples* from *samples that pose the question*.

<!-- fragment «name-tests-kit-fixture» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1029-1068" parent="name-test-support-and-kit" -->
````rust
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

````
<!-- /fragment -->

Eight listings and two triples. The first six listings are the shapes a healthy
`.grove/` holds — the charter, a live leaf, both terminal marks, a node directory
and a foreign `README.md` — and the last two are not shapes a healthy tree holds
at all. They are the near-misses, and the comment above them says two things
about them: what the fixture holds, and what a mutation does to it.

**The whole fixture, and what the canonicity check is doing to each entry.** The
kit's listings check renders back only the names that reach `Verdict::Entry`, so
the entry that poses canonicity is the one canonicity is itself refusing: take
the check away and it becomes an entry that renders as something else. A name
refused for any other reason cannot pose it, because removing the check leaves
that refusal exactly where it was.

| Fixture | Verdict | With the check removed |
|---|---|---|
| `_BRIEF.md` | `Entry(Brief)` | unchanged |
| `01-DONE-requirements--plan-k1.md` | `Entry` | unchanged |
| `02-impl--domain-k29.md` | `Entry` | unchanged |
| `03-ABANDONED-design--refusals-k30.md` | `Entry` | unchanged |
| `07-k28` | `Entry` | unchanged |
| `README.md` | `Foreign` | unchanged |
| `5-impl--domain-k29.md` | `Malformed(NotCanonical)` | **`Entry`, renders `05-impl--domain-k29.md`** |
| `07-DONE-grove-flip-k28` | `Malformed(InvalidName)` | unchanged |

`5-impl--domain-k29.md` is the entry the obligation rests on, and it is the only
one that could fail it. Its kind, slug and key are all well-formed — `impl`,
`domain`, 29 — so nothing refuses it earlier, and the refusal it does carry is
canonicity's own: its position is spelled `5` where grove's renderer writes `05`,
and `NotCanonical` hands that spelling back. Every other listing reaching the
comparison is already spelled the way grove writes it and would pass either way,
which is why the fixture's coverage of this obligation is exactly one entry
wide.

The double dash is not incidental. A leaf's kind and slug have been divided by
`--` since `grammar-separator-k15`, and a name carrying a single one is
`Malformed(MissingSeparator)`: refused for its separator before canonicity is
ever asked, and so present in the fixture without posing anything. A near-miss
has to miss by exactly the rule it is there to exercise, and by nothing else.

**Measured, not reasoned.** The comment's claim is a claim about what a mutation
does, so it was checked by performing that mutation in a copy of the workspace —
the seven canonicity lines removed from `parse` — rather than reasoned about:

| Arm | `listings()` | `the_task_tree_domain_conforms` |
|---|---|---|
| no mutation | all eight | green |
| canonicity removed | all eight | **red**, `TheGrammarIsCanonical` |
| canonicity removed | the first six | green |

The second arm is the claim and the third is what makes it evidence: the check
goes red only because of an entry the first six do not contain, so the fixture is
what carries the property rather than the checker being loud in general. The
message the second arm prints is *`5-impl--domain-k29.md` parsed to a name that
renders as `05-impl--domain-k29.md`. Two spellings of one name means two files on
disk are one entry.* The first arm is the one the suite runs.


`07-DONE-grove-flip-k28` is malformed directory syntax. It never reaches
the canonical rendering comparison, so it cannot alone demonstrate that the
comparison protects accepted names. The short-position fixture does.


The clause about coverage is the one worth carrying away. A kit's *this
obligation was never exercised* finding cannot see a gap of this shape: the
canonical listings parse and render back correctly, which is what the kit counts
as exercising canonicity, so a fixture holding nothing that could fail the
obligation still reports as covering it. A kit reports coverage of an obligation,
never coverage of the case that could fail it, and the distinction is invisible
from inside the report.

`triples()` is the other half of the sample. Two triples — a live leaf and a node
— feed the kit's `compose` obligation and the second direction of its canonicity
check, which composes a name, renders it, parses the rendering back, and requires
that the name that comes back be the **same name** rather than one that merely
renders the same way. That direction is why the store's check compares `view`s
and not strings.

The kit is then one line.

<!-- fragment «name-tests-conforms» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1069-1103" parent="name-test-support-and-kit" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** The name samples exercise the five semantic name
checks. Independent root/node fixtures also exercise Grove's current permissive
level method with missing, single and repeated brief values. Repeating the one
valid spelling tests a method input, not a realizable directory. The kit detects
missing contexts only; authors supply shapes and expected verdicts. Their expected
acceptance describes the domain method; the library separately refuses multiple
distinguished children. The kit's repetitions and permutations remain finite
samples and do not prove universal conformance or runtime enforcement.

**What it would still pass under.** Whatever the fixture does not pose — and
which obligations those are turns on whether the kit can generate the case for
itself. *Parse refuses what `found` contradicts* is generated: the kit offers
every sample name under all three `Found` values rather than only the one it was
paired with. Distinguished-name checks use the explicitly supplied
`TaskName::Brief` value and verify its canonical round trip.

Canonicity is the one it cannot generate: the kit can only render back names it
was handed or composed, so its reach here is exactly the fixture's, which is why
one listing carries the whole obligation and the mutation above is what says so.
Remove `5-impl--domain-k29.md` and the same run is green with the check gone.
*A name renders as one path component* cannot fail here at
all, though not for a reason about tokens: the store's check rejects an empty
rendering, exactly `.` or `..`, a path separator and a NUL, and every name this
domain renders opens with a digit run and a dash. The `.md` a leaf ends in is a
dot the check has no objection to. And the whole run is a finite sample:
the store's own header says a kit cannot prove the absence of hidden mutable
state behind `view` or `positioned_species`, and this call does not try to.

The second kit call replaces the triples and keeps the listings.

<!-- fragment «name-tests-kind-shapes» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1104-1149" parent="name-test-support-and-kit" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** Twenty-four composed names — eight kind spellings across
all three outcomes — survive the compose-render-parse-compare round trip. The
claim behind the fixture is the comment's, and it is about shape rather than
membership: there is no kind set to sweep since `open-kind-k20`, so the sample is
over the shapes a token can have. Those are the shapes that could break the round
trip, because the `--` has to be found in the right place and the token read back
whole. The inline comment records why all three outcomes are swept rather than
one: the infix is part of the name a kind renders into and is stripped before the
kind is read, so an off-by-one in `Outcome::strip` would show up here as a
mangled kind.

**Each of the six shapes the comment names is in the eight.**

| Shape the comment names | What the eight hold |
|---|---|
| one word | `impl`, `x`, `9` |
| two words | `research-a`, `review-impl`, `postmortem-2` |
| three words | `integrate-review-impl` and `integrate-review-prototype`, which are also the longest |
| a pair sharing a two-word prefix | those same two, which agree on `integrate-review` and diverge after it |
| digits | `9`, which is all digits, and `postmortem-2`, which ends in one |
| a single character | `x`, and `9` again |

Three words is the maximum and it is enough: a three-word token exercises
multi-word splitting exactly as a longer one would, and two of the eight carry
it. The shared prefix is the more interesting row, and it is worth being precise
about what it is not. A kind that is a *proper prefix* of another kind — the case
a longest match against a closed label set would have had to disambiguate — is a
hazard this grammar cannot have, because the first-`--` split has no set to match
against and a prefix relation between two kinds is not a fact the parser can
consult. What a shared prefix does pose is the thing the split can still get
wrong: two tokens that agree for their first two words and diverge after must be
read back whole, or the round trip fails on the one that was truncated.

**What it would still pass under.** The slug is the constant `a-slug-9` in every
one of the twenty-four, so nothing here varies the half of the name the kind is
distinguished from; a rule that split at the *last* `--` instead of the first
would pass, because no slug in this fixture contains one. The ordinal and key run
1 to 8 in step, so a `Display` that rendered the key where the ordinal belongs
would produce the same string for every fixture and pass. All eight kinds are
already canonical tokens, so `Kind::new`'s refusals are not reached — `a_kind`
would panic rather than the kit reporting, which is a different failure with a
different message. And the listings argument is `listings()` unchanged, so this
call re-runs the previous test's entire listings half; the twenty-four triples are
the only thing that is new, and the two calls are not independent evidence about
the fixture.

The last test in the block is the negative the open kind bought.

<!-- fragment «name-tests-undeclared-kind» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1150-1168" parent="name-test-support-and-kit" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** Five kind tokens no methodology declares — including
`wrok`, which is `work` misspelled, and `impl2`, which is a real kind with a
digit appended — parse into exactly the `Positioned` name their bytes describe.
The assertion is on the whole value rather than on success, so the kind that
comes back is the label that went in. As the comment says, this would have been a
*compile* error before `open-kind-k20`, when a kind was an enum variant; that it
is now a passing test is the whole content of the open token.

**What it would still pass under.** Every fixture is built by the same
`format!("01-{label}--a-k1.md")`, so position, slug and key are fixed at `01`,
`a` and `1` and nothing about their interaction with an unusual kind is tested.
`wrok`, `work`, `postmortem` and `spike` are all single words and pose the shape
`impl` already posed, so the five fixtures carry two distinct shapes rather than
five; `impl2` — a word with a digit fused to it, where `postmortem-2` puts a
hyphen before the digit — is the only one the previous test's eight did not
already hold.
And the test says nothing about the other direction — that a kind grove *does*
declare is not treated specially — which is `Kind::is_finish`'s question and
chapter 3's.

<a id="the-grammar-asserted"></a>
## The grammar, asserted

The third block is 113 lines and two labelled sections. The first is four tests
that walk the carried example's own shapes through `parse` and back out; the
second is three that hold canonicity itself.

<!-- fragment «grammar-and-canonicity-tests» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1189-1298" parent="source-task-name" -->
<!-- insert «name-tests-live-leaf» -->
<!-- insert «name-tests-terminal-marks» -->
<!-- insert «name-tests-node-directory» -->
<!-- insert «name-tests-terminal-key-marker» -->
<!-- insert «name-tests-lenient-position» -->
<!-- insert «name-tests-unpadded-past-99» -->
<!-- insert «name-tests-unrepresentable» -->
<!-- /fragment -->

The first is the round trip in the smallest form there is: one name, its whole
parsed value, and its rendering.

<!-- fragment «name-tests-live-leaf» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1189-1204" parent="grammar-and-canonicity-tests" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** `02-impl--domain-k29.md` parses to exactly the
`Positioned` name its bytes describe — ordinal 2, key 29, a live leaf of kind
`impl` and slug `domain` — and renders back to the same bytes. The second
assertion is `format(parse(f)) == f` for one name, written out by hand rather
than reached through the kit. Chapter 2's
`a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading` is the only
other test in the file that pins both a whole parsed value and its rendering, and
it does so twice.

**What it would still pass under.** The name is canonical, so the comparison
inside `parse` never fires and nothing here shows it exists; removing the seven
canonicity lines leaves this test green. Both the ordinal and the key are
two-digit-or-more numbers whose canonical rendering is their own digits, so the
padding rule is not exercised either. And the equality is against a whole
`TaskName` value, which is the strongest form available for a single fixture and
still says nothing about any name but this one.

The second widens it to the two terminal marks.

<!-- fragment «name-tests-terminal-marks» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1205-1220" parent="grammar-and-canonicity-tests" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** A `DONE-` infix parses to `Outcome::Done` and an
`ABANDONED-` infix to `Outcome::Abandoned`, so `Outcome::strip` reads both marks
off the position and hands the rest on with the kind intact. Both fixtures are
listings the kit also holds, which is what ties the two blocks to one tree.

**What it would still pass under.** Less than the destructuring suggests. The
match arm binds `outcome` and ignores the ordinal, the key, the kind and the
slug — but `entry` has already required each name to render back to itself, so
those four are held by the canonicity comparison rather than left free. The same
comparison holds the infix: replacing `outcome.infix()` in the leaf arm with the
empty string turns this test red, because `01-DONE-requirements--plan-k1.md`
would render as `01-requirements--plan-k1.md` and never reach the assertion. What
is genuinely absent is what no fixture carries — no `Live` name, so the absence of
a mark is not asserted to yield `Outcome::Live`; no directory, so the reason the
infix is stripped for both species *before* the branch is chapter 2's to pin; and
no non-canonical spelling, so no refusal path is reached.

The third is the other species.

<!-- fragment «name-tests-node-directory» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1221-1232" parent="grammar-and-canonicity-tests" -->
````rust
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

````
<!-- /fragment -->


**What it establishes.** `07-k28` parses as a directory at position 7 and
key 28 with `Parts::Node`. Its parts carry no title. A separate `_topic.md`
parses as a distinguished name, and the level rule decides whether it is
correctly placed beside that directory's children.


**What it would still pass under.** The slug `grove-flip` contains a hyphen and
no `-k`, so the terminality rule is not posed here. The node arm of `Display` is
not among the gaps, although nothing in the test says otherwise: widening its
position format to three digits turns this test red, because `07-k28`
would render as `007-k28` and `entry` would meet a `NotCanonical`. The
hazard the fixture does not carry is its own, and chapter 2 carries it instead:
`a_node_wearing_an_outcome_infix_is_malformed` refuses this very name wearing
each of the two infixes, and that refusal is what makes accepting the bare form
meaningful rather than permissive.

The fourth is the terminality rule from the parsing side, and it is the same
fixture chapter 3 used from the handle side.

<!-- fragment «name-tests-terminal-key-marker» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1233-1245" parent="grammar-and-canonicity-tests" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** In `05-impl--task-k9-k3.md` the key is 3 and the slug is
`task-k9`, so `peel_key`'s search from the end is what `split_shape` inherits and
a slug may contain the key marker. Chapter 3's
`a_handle_and_a_filename_peel_the_same_key` asserted that the filename route and
the handle route agree about this name; this asserts what the filename route
answers, which is the half that makes the agreement worth having.

**What it would still pass under.** The match binds `key` and `parts` and ignores
the ordinal, the outcome and the kind — and again `entry` has pinned the whole
rendering, so corrupting `Display`'s ordinal turns this test red without the
assertion looking. The real limits are the fixture's: one name, a single-digit
key, and one embedded marker `-k9`, so nothing here poses a multi-digit key, a
slug ending in the marker with no digits after it, or two markers of the same
width. What the test genuinely leaves to chapter 3 is the agreement between the
two routes — `a_handle_and_a_filename_peel_the_same_key` runs this same fixture
from the handle side, and that agreement is what makes one shared peel worth
having.

<a id="canonicity-asserted"></a>
## Question 2: the grammar is canonical

Three tests, and between them they are the whole of what this crate says about
canonicity in its own voice. The kit grades the obligation from outside; these
state what grove refuses, what it accepts, and what it cannot advise about.

<!-- fragment «name-tests-lenient-position» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1246-1278" parent="grammar-and-canonicity-tests" -->
````rust
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

````
<!-- /fragment -->

**What it establishes.** Four lenient spellings are refused, and each refusal
carries the exact spelling grove would have written. `5-…` has too few leading
zeros; `005-…` and `0100-…` have too many, on either side of the point where
padding stops; and `7-k2` is a directory rather than a file, so the fourth
refusal is reached through the node arm rather than the leaf arm. The assertion is on the whole `TaskNameError`
value, so the `canonical` field is the name and not merely a string containing
it, and the two `contains` checks then hold that the rendered message carries
both that name and the word *Rename*. The doc comment states why the advice is
part of the claim: a lenient spelling refused with no canonical form leaves a
hand-edited tree unreadable with no stated way back.

**What it would still pass under.** Every fixture varies the position and nothing
else — the kind, the slug and the key are `impl`, `a` and `1` throughout, and
`7-k2` differs only in being a node. The gap that leaves is the **key**,
which has a canonicity of its own and is not tested here: the key renders as
plain decimal, so `01-impl--a-k007.md` is refused as `NotCanonical` with the
canonical form `01-impl--a-k7.md`, and no fixture in this file poses it. That
absence is the same asymmetry chapter 3 read from the other side —
`handles_require_canonical_keys_and_slugs` asserts that `Handle::parse`
*accepts* `a-k007` for key 7, because a handle is a reference a human types and
never a file on disk. The two directions are both deliberate and only one of them
has a fixture. `malformed` is also called twice per fixture, at lines 1,237 and
1,244 so the test assumes `parse` is deterministic rather than showing it, and
the message assertions are `contains`, so a message that named the canonical form
and then said something wrong beside it would pass.

The second is the boundary the format string produces rather than the one anyone
chose.

<!-- fragment «name-tests-unpadded-past-99» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1279-1287" parent="grammar-and-canonicity-tests" -->
````rust
    /// `{:02}` is a minimum width, not an exact one, so the canonical rule is
    /// *zero-padded to at least two digits and no other leading zero*. Three
    /// digits past 99 is a name, not a violation.
    #[test]
    fn a_position_past_ninety_nine_is_canonical_unpadded() {
        let name = entry("100-impl--a-k1.md", Found::File);
        assert_eq!(name.to_string(), "100-impl--a-k1.md");
    }

````
<!-- /fragment -->

**What it establishes.** `100-impl--a-k1.md` is a name, not a violation, and it
renders back to itself. That is the consequence of `{:02}` being a minimum width,
and it is the reason the canonical rule has to be stated as *padded to at least
two digits and carrying no other leading zero* rather than as *two digits*. Taken
with `0100-impl--a-k1.md`'s refusal in the previous test, the pair fixes the
boundary from both sides.

**What it would still pass under.** One fixture at one value. `100` is the
smallest three-digit position, so nothing here distinguishes a rule that stops
padding at three digits from one that stops at any width, and `99` — the largest
position the format string pads to exactly two digits — is absent from both
tests. The
assertion is that the rendering equals the input, which `entry` already implies
by construction: `entry` panics unless `parse` returned `Entry`, and `parse`
returns `Entry` only after comparing that same rendering against that same input.
The line is a restatement of the canonicity check rather than an independent
observation of it, and it would go on passing if `Display` and `parse` drifted
together.

The third is the refusal with no advice to give.

<!-- fragment «name-tests-unrepresentable» owner="canonical-or-nothing" source="crates/grove-loop/src/task_name.rs" lines="1288-1298" parent="grammar-and-canonicity-tests" -->
````rust
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

````
<!-- /fragment -->


The overflow fixtures require `InvalidName`, preserving the offending
spelling and stating the representable key range. The canonical-name matrix
also covers `u32::MAX` and one beyond it. No canonical replacement is offered
when the number cannot be represented.



The assertions pin the offending spelling and the range advice, while the
accepted maximum-key case protects against refusing too early. These cases
are sampled evidence; the shared integer parser supplies the complete width
check. Zero is explicitly refused by `parse_key`.


The grammar is complete. Every byte an entry's name can carry has one place that
writes it and one place that reads it back, the comparison at the end of `parse`
holds those two to each other, and the library's own kit grades the obligation
that comparison discharges. All nine of `task_name.rs`'s ownership blocks are now
resolved, and the file is reconstructed in full.

What the grammar cannot do is find anything. It judges a name it is handed, one
at a time, with no notion of a directory, a parent or an order — `parse` takes a
string and a `Found` and nothing else, and the module refuses a path outright.
Chapter 5 opens the tree those names live in: what a `Tree` is, what its guard
proves, and why a caller who has one has been told only that the tree was there
when it was opened.

[Previous: Kind, slug, handle](03-kind-slug-handle.md) | [Contents](README.md) | [Next: Opening, contention and refusal](05-opening.md)

# Two documents, neither one assembled
<!-- book-page id="two-documents" slice="never-assembled" order="3" -->
[Previous: The names a template is written against](02-the-names.md) | [Contents](README.md) | [Next: What a template must be](04-template-law.md)

<a id="never-assembled"></a>
## Never assembled

Chapter 2 put the names on the page and the shapes a loaded configuration
compiles into. This chapter is the function that fills them: `Templates::load`,
the crate's one entry point for a configuration, together with the reading and
the whole-document validation it drives.

What this stage must not add and must not interpret is a launch that came from
more than one place. A key resolves to one complete command template, read whole
out of one file. Two documents are read, and a key resolves from the primary or
from the overlay, never from both, and only if the **primary** declares it. One
whole template replaces one whole template, so nothing in the crate has to decide
which *words* of a launch come from where. There is no precedence order, no base
and override, no inheritance and no way to write *the same command with a
different model*: the two documents are searched, and they are never merged.

That is the first arm of the test this book closes on. A layer learns what a
value means **on the way in** by assembling one value out of more than one
source, and the cost is that nobody can see the whole of it in one place and no
single author owns it. This chapter is where the crate declines to pay that cost,
and two tests in `crates/keyed-launch/tests/templates.rs` pin the decline:
`an_overlay_replaces_a_whole_template_and_reports_its_own_path`, which requires a
key the overlay declares to come from the overlay *entire* and to report the
overlay's own path, and `a_key_only_the_overlay_declares_does_not_resolve`, which
is this chapter's second ending.

The chapter owns two blocks of `src/templates.rs` that are not adjacent, and the
gap between them is deliberate. `src/templates.rs` is ordered by Rust convention —
the types, then the public `impl`, then the free functions, then the diagnostic
helpers — so `load` is declared at line 104 and the first function it calls
begins at line 283. The book follows the concept rather than the file: `load` is
read here with the four functions that do its work, and the 130 lines lying
between them, which are the rest of the public `impl`, are chapter 5's. The table
below is the map to read the rest of this chapter against; take from it which of
`src/templates.rs`'s three middle blocks a given line belongs to, and which
chapter will explain it.

| Lines | Block | Chapter | What it holds |
|---|---|---:|---|
| `92-145` | `templates-load` | 3 | `impl Templates {`, and `load` whole |
| `146-275` | `resolution-and-expansion` | 5 | `source`, `require`, `expand`, `match_values`, `declared_slots`, `unresolved`, and the `}` that closes the `impl` |
| `276-414` | `reading-and-whole-document-validation` | 3 | `compile_vocabulary`, `read_primary`, `read_overlay`, `parse_and_validate`, `validate_document` |

One consequence of that split is visible in the fragments themselves and is
easier met here than discovered later. `impl Templates {` opens this chapter's
first block at line 92, and the brace that closes that `impl` is the last line of
chapter 5's block at line 275; this page never prints it. A reader following the
file rather than the book meets `source`, `require` and `expand` between the two
halves of this chapter, and meets the first of `load`'s callees at line 283,
139 lines below the brace that ends `load`.

<a id="one-entry-point"></a>
## One entry point, and three promises

`load` is the only constructor this type has. `Templates`'s five fields are all
private, no other associated function returns `Self`, and no `Default`, `From` or
builder exists, so every `Templates` value anywhere in the workspace came through
this function and was therefore validated whole. That is the same shape as
`Argv`, whose absent public constructor chapter 5 reads, and it is what lets
every later chapter assume a template it holds is one that passed.

The block is read in four fragments, cut at the boundaries of what `load` does:
the promises it states, the primary document, the overlay, and the value it
returns.

<!-- fragment «templates-load» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="92-145" parent="source-templates" -->
<!-- insert «templates-load-three-promises» -->
<!-- insert «templates-load-primary» -->
<!-- insert «templates-load-overlay» -->
<!-- insert «templates-load-value» -->
<!-- /fragment -->

The first fragment is the doc comment and the signature. Its three paragraphs are
the whole of what a consumer is promised by a successful load, and they are the
only place in `src/templates.rs` where those promises are written down rather
than enforced — the file is 13% comment, and the rules the promises name are
scattered across five functions and two hundred lines below. The signature is
what makes the second and third promises checkable at all: a primary path, an
optional overlay path, and the vocabulary chapter 2 argued belongs here.

<!-- fragment «templates-load-three-promises» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="92-108" parent="templates-load" -->
````rust
impl Templates {
    /// Read and fully validate both documents, then resolve one template per key.
    ///
    /// A key resolves from the primary file or the overlay, never from both, and
    /// only if the **primary** declares it: the overlay overrides and never
    /// supplies. Both documents are validated whole against `vocabulary` — a
    /// malformed template for a key this run will never reach still fails here,
    /// before anything is spawned.
    ///
    /// All-or-nothing in both halves: an unreadable, unparseable or invalid
    /// overlay fails the load rather than falling back to the very policy its
    /// owner was moving work away from.
    pub fn load(
        primary: &Path,
        overlay: Option<&Path>,
        vocabulary: Vocabulary<'_>,
    ) -> Result<Self, ConfigError> {
````
<!-- /fragment -->

Read the three paragraphs as three separate promises, because they fail
separately and are pinned by different tests. The table states each with the line
that enforces it, what an operator sees when it is broken, and the record the
line answers to; the crate's source states almost none of that, and the rules are
the reason the records exist.

| Promise | Enforced at | What a breach produces | Pinned by |
|---|---|---|---|
| a key resolves only if the primary declares it | the `Entry::Vacant` arm, line 131 | a refusal at the moment the key is used, naming the overlay the key *is* in and the primary that must declare it | `a_key_only_the_overlay_declares_does_not_resolve` |
| both documents are validated whole against the vocabulary | the two `parse_and_validate` calls, lines 113 and 119 | one aggregate refusal against the failing document's own path | `an_invalid_overlay_fails_the_load_against_its_own_path` |
| the load is all-or-nothing in both halves | the `?` on lines 109, 111, 113, 117 and 119 | no `Templates` value at all, and so nothing spawned | `an_unreadable_overlay_fails_closed` |

Two decision records are what those promises keep, and this chapter is where
their lines are. *Complete session configuration* settles that every kind
resolves to one complete command-template string read whole out of a single file,
that nothing is merged within a kind, and that presence is per-kind and
just-in-time while everything else about a document is eager — a malformed entry
for a kind this iteration will not reach still fails before anything is spawned.
The second promise is that eagerness, and the sentence in `load`'s comment
restates it in the crate's own vocabulary. *The untracked configuration delta*
states the first promise as its own property: a key resolves only if the primary
declares it, and where only the second document declares one the refusal names
the key and the primary file that must declare it. Neither record is linked from
this page; a book's local link targets are its own pages, its own roots, the
guide and the glossary.

The third promise is the one a reader is most likely to assume away, so it is
worth stating what the rejected alternative was. A load that fell back to the
primary when the overlay could not be read or did not validate would look
forgiving. What it would actually do is run the launch policy the operator was
moving work *away* from, silently, at the moment their replacement policy was
broken — and the two policies differ precisely in which program they name.
`an_unreadable_overlay_fails_closed` passes a path to a file that does not exist
and requires the whole load to fail; that is a deliberate asymmetry with
`read_primary`, which this chapter reads two sections below, and it is the cost
side of *optional*.

<a id="overrides-never-supplies"></a>
## The overlay overrides and never supplies

`load`'s body is three steps and a value, and their order is the argument. The
next fragment is the first two steps: the vocabulary is compiled into the owned
slot table before any file is opened, and the primary document is read and
validated with that table in hand. Every one of these lines carries a `?`, so a
duplicate slot name, an absent primary, or a single bad template anywhere in the
primary ends the call with no value returned and no second file read.

<!-- fragment «templates-load-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="109-114" parent="templates-load" -->
````rust
        let slots = compile_vocabulary(&vocabulary)?;

        let primary_source = read_primary(primary)?;
        let mut templates =
            parse_and_validate(primary, &primary_source, DocumentRole::Primary, &slots)?;

````
<!-- /fragment -->

The ordering is not incidental. `compile_vocabulary` refuses a duplicate slot
name, and it runs before the filesystem is touched, so a consumer that declared
`prompt` twice learns it whether or not its configuration file exists;
`a_duplicated_slot_name_is_refused_at_load` supplies a real file and still gets
the vocabulary's refusal. `read_primary` and `parse_and_validate` then take the
`&slots` binding by reference, which is why the table is compiled once per load
rather than once per document.

The overlay is the whole of the second fragment, and it is the only branch in the
function. Its five-line comment is the densest argument in the block: it says
what an occupied entry means, what a vacant one means, and which older rule the
vacant arm is the per-key restatement of.

<!-- fragment «templates-load-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="115-135" parent="templates-load" -->
````rust
        let mut overlay_only = BTreeSet::new();
        if let Some(overlay_path) = overlay {
            let overlay_source = read_overlay(overlay_path)?;
            let declared =
                parse_and_validate(overlay_path, &overlay_source, DocumentRole::Overlay, &slots)?;
            // Each key the primary already declares wins outright: one whole
            // template replaces one whole template, so no rule has to decide
            // which *words* of a launch come from where. A key the primary does
            // not declare is set aside rather than admitted, which is the
            // per-key form of what the old all-keys completeness rule bought.
            for (key, template) in declared {
                match templates.entry(key) {
                    Entry::Occupied(mut occupied) => {
                        occupied.insert(template);
                    }
                    Entry::Vacant(vacant) => {
                        overlay_only.insert(vacant.into_key());
                    }
                }
            }
        }
````
<!-- /fragment -->

`Entry::Occupied` and `Entry::Vacant` are the two cases, and there is no third.
The occupied arm calls `insert`, which replaces one whole `Template` — its
compiled `Vec<Word>` and the `source` path it was read from — with another. No
field of the primary's template survives that, which is what *one whole template
replaces one whole template* means in code and why no rule has to decide which
words win. `an_overlay_replaces_a_whole_template_and_reports_its_own_path` is
the adjudication: it loads a two-key primary and a one-key overlay and requires
both that the replaced key expands to the overlay's words and that
`templates.source` names the overlay's path for that key and the primary's for
its neighbour.

The vacant arm is the one that does not do the obvious thing. It could insert;
the map is right there and the template it holds has already been validated
against the vocabulary. Instead it calls `vacant.into_key()` and puts the key in
`overlay_only`, a set, and drops the template. The comment states the cost of the
alternative: a second source that could introduce a key would name a program the
operator never chose, and for grove that second source is a file a project ships.
`a_key_only_the_overlay_declares_does_not_resolve` names itself *the per-key
restatement of what a completeness quantifier used to buy*, which is the
through-line: the primary file used to be required to declare every key a
methodology could name, so a second document could only ever override something
already written down. That quantifier was retired because nothing could enumerate the
set it quantified over. The vacant arm buys the same property one key at a time,
at the moment the key is used, without either file having to know the whole set.

What is kept and what is dropped is the last thing to notice here. The template
the overlay wrote for an unresolvable key is discarded — it was validated, and
then it was thrown away — while the *key* is kept. Chapter 2 read the field's own
comment on why: `overlay_only` exists so a refusal can distinguish a typo from a
misunderstanding of what an overlay may do. Keeping the template would have
bought nothing, because no call in the crate can reach it; keeping the key buys
one sentence in one error message.

The last fragment is the value, and it is a plain struct literal with no further
work in it.

<!-- fragment «templates-load-value» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="136-145" parent="templates-load" -->
````rust

        Ok(Templates {
            primary: primary.to_path_buf(),
            overlay: overlay.map(Path::to_path_buf),
            slots,
            templates,
            overlay_only,
        })
    }

````
<!-- /fragment -->

Both paths are stored as owned `PathBuf`s and the overlay as an `Option`, so a
loaded configuration can name its own files long after the borrowed arguments are
gone. That is what `unresolved` — chapter 5's — reads when it builds the refusal
that names the primary, and it is why `Templates` can answer *which file did this
come from* per key without the caller keeping the paths.

<a id="both-documents"></a>
## Both documents

The example this book carries takes its second step here. Chapter 1 put two lines
in the operator's personal `~/.config/grove/config.kdl` and chapter 2 fixed the
four-slot vocabulary they are written against. Now there is a second file. Its
grove-side name is `.grove.kdl` and it sits at the worktree root, `/work/atlas`;
the crate knows it only as `Some(overlay_path)` and applies to it every rule it
applied to the first. The figure is the whole input to this section: two
documents, one of which declares a key the other also declares.

```text
~/.config/grove/config.kdl
  impl "claude --model opus ${prompt}"
  review-impl "codex exec --model gpt-5 ${prompt}"

/work/atlas/.grove.kdl
  impl "claude --model opus-fast ${prompt}"
```

The call is `Templates::load(primary, Some(overlay), vocabulary)`, and it runs
the three steps above in order. `compile_vocabulary` turns the four `SlotRule`s
into four `SlotSpec`s. `read_primary` reads the first file and
`parse_and_validate` compiles and checks it whole, producing two templates. Then
`read_overlay` reads the second file and `parse_and_validate` checks *it* whole,
against the same slot table and the same rules — its single template must contain
`${prompt}` exactly once just as the primary's must, and
`an_invalid_overlay_fails_the_load_against_its_own_path` requires that failing it
fails the load and names the overlay's path.

That second validation is the first place in the book where `validate_node` and
`validate_template` are named. They are the per-node and per-template rule checks
`validate_document` drives over both documents; each returns diagnostics with
locations rather than stopping at the first, and chapter 4 owns them both. The
minimum a reader needs here is that they are what *validated whole* is made of,
and that nothing in either of them consults the document's role — the rules are
identical for both files, and the one asymmetry between the documents is a
resolution rule applied after both have passed.

The overlay declares `impl`, which the primary also declares, so the
`Entry::Occupied` arm replaces it. The observable end of the step is that the two
keys now resolve from different files, and each names its own.

```text
templates.source("impl")        -> Some("/work/atlas/.grove.kdl")
templates.source("review-impl") -> Some("~/.config/grove/config.kdl")
```

Neither line is a merge. `impl`'s template is the overlay's four words entire,
including the `claude` the two files happen to agree on; `review-impl`'s is the
primary's, untouched. If the overlay had been absent the `if let` would not have
run and both would name the primary. The reason `Template` carries a `source`
path per key rather than one path per configuration — chapter 2 read that
comment — is exactly this state: after an overlay resolves there is no single
answer to *which file did this configuration come from*.

<a id="the-second-ending"></a>
## A key only the overlay declares

The second of the book's two endings is the case the vacant arm produces, and it
is worth walking because the load **succeeds**. The figure below is the input,
and the thing to take from it is that nothing in either document is wrong:
suppose the overlay declares a key the personal file does not.

```text
/work/atlas/.grove.kdl
  impl "claude --model opus-fast ${prompt}"
  research-a "gemini --model pro ${prompt}"
```

Both documents are well-formed and both pass every rule: `research-a`'s template
has one `${prompt}`, a literal word zero, no properties, no children and no
duplicate. It is validated *before* it is set aside, which is what makes an
invalid overlay fail the load even for a key that was never going to resolve.
`load` returns `Ok`. Nothing is refused yet, and `templates.source("research-a")`
is `None` rather than an error.

The refusal arrives at the moment the key is committed to — `require`, or
`expand`, both chapter 5's — and it is `overlay_only` that gives it its wording.

```console
key `research-a` does not resolve: it is declared only in the configuration overlay at /work/atlas/.grove.kdl, and an overlay overrides a key the primary declares but never supplies one of its own.
  Declare `research-a` in ~/.config/grove/config.kdl.
```

The set built on line 131 decides which of two messages this is, and that choice
is what puts the other two facts in front of the reader. This is the
*overlay-only* branch rather than the plain `no template for it` one, so the
reader is not sent looking for a typo. It names the overlay path, so a reader who
wrote the key down is told which file they wrote it in. And its last line names
the primary, which is the file that must declare it — the same sentence
`a_key_nobody_declares_names_the_primary_file` requires for a key nobody wrote at
all. `a_key_only_the_overlay_declares_does_not_resolve` asserts all three, and
then asserts that `expand` produces the same refusal, because a key is committed
to at two moments and one wording owns both.

The property that buys is one sentence long: an untracked file a project ships
can change which program an already-chosen key runs, and it cannot introduce a
key. Whether such a file is admissible at all — where it is searched for, and why
it must be untracked — is the consumer's question and not this crate's. `load`
takes a path and reads it.

<a id="the-slot-table-first"></a>
## The slot table first, and the duplicate it refuses

The rest of the chapter is the five functions `load` reaches, in the order it
reaches them. They are free functions rather than methods because none of them
needs a `Templates` — they run before one exists — and all five are private:
`load` is the only way into any of them, and the only way to obtain a
`Templates`. The block is read in seven fragments.

<!-- fragment «reading-and-whole-document-validation» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="276-414" parent="source-templates" -->
<!-- insert «compile-vocabulary» -->
<!-- insert «read-primary» -->
<!-- insert «read-overlay» -->
<!-- insert «parse-and-validate» -->
<!-- insert «validate-document-nodes» -->
<!-- insert «validate-document-duplicates» -->
<!-- insert «validate-document-report» -->
<!-- /fragment -->

`compile_vocabulary` is the first, and it does two things that look like one: it
copies the consumer's borrowed `SlotRule`s into the owned `SlotSpec` table that
`Templates` will keep, and it refuses a duplicate name. Its comment is the only
statement anywhere in the crate of why the duplicate is a refusal rather than a
tolerated redundancy, and the reason is that the failure it would otherwise cause
is silent and lands on the wrong file.

<!-- fragment «compile-vocabulary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="276-298" parent="reading-and-whole-document-validation" -->
````rust

/// Turn the borrowed vocabulary into the owned slot table `Templates` keeps, and
/// refuse a duplicate name.
///
/// A duplicated slot would be counted twice against its own cardinality rule and
/// would take whichever value arrived first at expansion — a consumer bug that
/// looks like a template bug for as long as it goes unnamed.
fn compile_vocabulary(vocabulary: &Vocabulary<'_>) -> Result<Vec<SlotSpec>, ConfigError> {
    let mut slots: Vec<SlotSpec> = Vec::with_capacity(vocabulary.slots.len());
    for rule in vocabulary.slots {
        if slots.iter().any(|slot| slot.name == rule.name) {
            return Err(ConfigError::new(format!(
                "the slot vocabulary declares `{}` more than once",
                rule.name
            )));
        }
        slots.push(SlotSpec {
            name: rule.name.to_owned(),
            requirement: rule.requirement,
        });
    }
    Ok(slots)
}
````
<!-- /fragment -->

The check is a linear scan over the slots accumulated so far, which is quadratic
in the vocabulary's size and is the right shape for a table with four entries in
grove's case and no plausible consumer with hundreds. It fires before the
filesystem is touched, and its message names the offending slot:
``the slot vocabulary declares `prompt` more than once``. Note what it is *not* —
there is no location, because a vocabulary is a `const` array in the consumer's
own source rather than a document this crate read, and the crate has no offset to
report.

The comment's second paragraph is the argument this section owes, and it is the
kind the source states only here. A duplicated slot would be counted twice
against its own cardinality rule: `validate_template` walks the slot table and
asks each entry's `Requirement` whether the occurrence count admits it, so a
`prompt` declared twice would be asked twice about the same one occurrence, and a
template containing `${prompt}` once would be refused by the second copy under
`ExactlyOnce` or accepted twice under `AtMostOnce`. At expansion the same
duplication makes `match_values` — chapter 5's — resolve the name to the *first*
matching index, so a value offered for `prompt` fills one of the two positions
and the other reports itself unfilled. Both symptoms name a template, or a value,
and neither names the vocabulary. That is the phrase the comment ends on: *a
consumer bug that looks like a template bug for as long as it goes unnamed*.
`a_duplicated_slot_name_is_refused_at_load` is what holds it, and it is the only
test in the file that builds its own vocabulary rather than using the shared one.

<a id="optional-at-the-filesystem"></a>
## What *optional* means at the filesystem

The two reading functions are thirteen lines and eight, and neither carries a
comment. They are worth reading side by side: the difference between them is the
whole of what *the overlay is optional* means once a path has been handed in, and
the source nowhere says so.

<!-- fragment «read-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="299-312" parent="reading-and-whole-document-validation" -->
````rust

fn read_primary(path: &Path) -> Result<String, ConfigError> {
    match fs::read_to_string(path) {
        Ok(source) => Ok(source),
        Err(error) if error.kind() == ErrorKind::NotFound => Err(ConfigError::new(format!(
            "configuration is missing at {}",
            path.display()
        ))),
        Err(error) => Err(ConfigError::new(format!(
            "failed to read the configuration at {}: {error}",
            path.display()
        ))),
    }
}
````
<!-- /fragment -->

`read_primary` distinguishes one `io::ErrorKind` and folds every other into a
second message. `NotFound` gets ``configuration is missing at <path>``, which
tells an operator that the file they have not written yet is the thing to write;
`a_missing_primary_names_its_path` requires both the sentence and the path. Every
other kind — a permission denial, a directory where a file was expected, an I/O
error — gets ``failed to read the configuration at <path>: <error>``, with the
underlying error interpolated rather than classified. That is the whole taxonomy:
one case named, everything else reported verbatim.

The overlay's reader has no such case.

<!-- fragment «read-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="313-321" parent="reading-and-whole-document-validation" -->
````rust

fn read_overlay(path: &Path) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|error| {
        ConfigError::new(format!(
            "failed to read the configuration overlay at {}: {error}",
            path.display()
        ))
    })
}
````
<!-- /fragment -->

Every failure is the same failure, including `NotFound`. The asymmetry is the
answer to a question the signature raises and does not settle: `load` takes
`Option<&Path>`, so *optional* has already been decided one level up. A consumer
that has no overlay passes `None` and `read_overlay` is never called. A consumer
that passes `Some(path)` has asserted that a file is there, and a missing file at
an asserted path is a broken assertion rather than an absence.
`an_unreadable_overlay_fails_closed` pins exactly that: it passes a path inside a
temporary directory that was never written and requires
``failed to read the configuration overlay at`` in the error. The optionality
lives in the `Option`, and nowhere else — which is what lets the fallback
question be answered once, in `load`'s comment, rather than per error kind here.

<a id="parsed-then-validated"></a>
## Parsed, then validated

`parse_and_validate` is the seam between the two document formats the crate
depends on and the rules it adds on top of them. It is nineteen lines, and eight
of them are the message it builds when the parse fails.

<!-- fragment «parse-and-validate» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="322-341" parent="reading-and-whole-document-validation" -->
````rust

fn parse_and_validate(
    path: &Path,
    source: &str,
    role: DocumentRole,
    slots: &[SlotSpec],
) -> Result<BTreeMap<String, Template>, ConfigError> {
    let document: KdlDocument = source.parse().map_err(|error: kdl::KdlError| {
        let location = source_location(source, error.span.offset());
        ConfigError::new(format!(
            "{}:{}:{}: KDL syntax error: {}",
            path.display(),
            location.line,
            location.column,
            error
        ))
    })?;

    validate_document(path, source, &document, role, slots)
}
````
<!-- /fragment -->

The parse is `source.parse::<KdlDocument>()`, and the crate adds one thing to
`kdl`'s own error: a position a human can act on. `kdl` reports a byte offset in
`error.span`; `source_location` converts it into a one-based line and column, and
the message is assembled as `path:line:column: KDL syntax error: <error>`, with
the crate's location first and the parser's own text last.
`a_kdl_syntax_error_names_its_source_location` requires both the phrase and a
`:1:` in the result.

Three functions chapter 4 owns are used here and in the next section, and the
minimum to carry until then is small. `source_location` turns a byte offset into
a one-based line and column by counting newlines before the offset and characters
since the last one; `format_location` renders one as `path:line:column`; and
`render_diagnostics` assembles a whole refusal from a document's path, its role
and its list of diagnostics. Chapter 4 explains all three together, as the
machinery behind *name what is wrong, name where, name what fixes it*.

Two things are worth naming about the shape. A syntax error returns immediately
and is *not* aggregated with anything: there is no document to walk, so the
first-error rule that governs this function is a consequence of parsing, not a
choice about reporting. And `role` is carried through untouched — it is not
consulted here at all, and it reaches `validate_document` only to be handed to
`render_diagnostics` at the end. `DocumentRole`, which chapter 2 read, changes a
noun and no rule.

<a id="one-refusal"></a>
## Every diagnostic in one refusal

`validate_document` is the last function in the block and the one that decides
what a refusal looks like. It runs in three passes over one document, and the
three fragments below are those passes. The first walks the nodes, recording each
node's validation and building an index from key to every location that key was
declared at.

<!-- fragment «validate-document-nodes» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="342-361" parent="reading-and-whole-document-validation" -->
````rust

fn validate_document(
    path: &Path,
    source: &str,
    document: &KdlDocument,
    role: DocumentRole,
    slots: &[SlotSpec],
) -> Result<BTreeMap<String, Template>, ConfigError> {
    let mut validations = Vec::new();
    let mut occurrences: HashMap<String, Vec<SourceLocation>> = HashMap::new();

    for node in document.nodes() {
        let validation = validate_node(source, node, slots);
        occurrences
            .entry(validation.key.clone())
            .or_default()
            .push(validation.location);
        validations.push(validation);
    }

````
<!-- /fragment -->

Nothing is rejected in this pass. `validate_node` is called for every node in the
document, and the `NodeValidation` it returns is pushed whether or not the node
was valid — which is the design chapter 2 read from the type's side, where its
`key` and `location` fields are filled unconditionally and its `template` field
is `Some` only when the node compiled. The `occurrences` map is a
`HashMap<String, Vec<SourceLocation>>` rather than a count, because the finding
this pass makes possible has to name every declaration, not say how many there
were.

The second pass is the duplicate check, and it is the one finding
`validate_document` produces on its own rather than collecting from a node.

<!-- fragment «validate-document-duplicates» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="362-389" parent="reading-and-whole-document-validation" -->
````rust
    let mut diagnostics = Vec::new();

    // Duplicates, in declaration order of their first appearance, each naming
    // every one of its own locations.
    let mut duplicates: Vec<String> = Vec::new();
    for validation in &validations {
        if occurrences
            .get(&validation.key)
            .is_some_and(|items| items.len() > 1)
            && !duplicates.contains(&validation.key)
        {
            duplicates.push(validation.key.clone());
        }
    }
    for key in duplicates {
        let Some(locations) = occurrences.get(&key) else {
            continue;
        };
        let declarations = locations
            .iter()
            .map(|location| format_location(path, *location))
            .collect::<Vec<_>>()
            .join(", ");
        diagnostics.push(ValidationDiagnostic {
            location: locations.first().copied(),
            message: format!("duplicate key `{key}`; declarations at {declarations}"),
        });
    }
````
<!-- /fragment -->

The two loops are two different jobs. The first builds `duplicates` by walking
`validations` in document order and taking each key whose occurrence list has
more than one entry, guarded by `!duplicates.contains` so a key declared three
times is reported once; the comment above it fixes the resulting order as
*declaration order of their first appearance*, which is a stable, file-shaped
order rather than the `HashMap`'s. The second loop turns each into a diagnostic
whose message joins every location for that key with `format_location`.
`a_duplicate_key_reports_every_declaration_location` is the adjudication: it
loads a document declaring `one` twice and requires both `:1:1,` and `:2:1` in
the single message — the trailing comma in the first assertion is what pins that
the locations are joined rather than only the first being reported.

The `location` field of that diagnostic is `locations.first().copied()`, and it
is the sole reason `ValidationDiagnostic::location` is an `Option` at all.
Chapter 2 named this from the type's side: the surrounding code has already
established that the vector is non-empty, the compiler cannot see it, and the
`None` is unreachable. This is the line that makes it so.

The third pass drains everything into one result. It is where a document either
becomes a map of templates or becomes a single refusal.

<!-- fragment «validate-document-report» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="390-414" parent="reading-and-whole-document-validation" -->
````rust

    let mut templates = BTreeMap::new();
    for validation in validations {
        diagnostics.extend(validation.diagnostics);
        if let Some(words) = validation.template {
            templates.insert(
                validation.key,
                Template {
                    words,
                    source: path.to_path_buf(),
                },
            );
        }
    }

    if !diagnostics.is_empty() {
        return Err(ConfigError::new(render_diagnostics(
            path,
            role,
            diagnostics,
        )));
    }

    Ok(templates)
}
````
<!-- /fragment -->

The order of the two loops matters and is easy to read past. Duplicate findings
were pushed into `diagnostics` first, so they lead the report; per-node
diagnostics follow in document order, extended one node at a time. Templates are
collected in the same walk that drains the diagnostics, into a `BTreeMap` keyed
by name — so `Templates::keys`, chapter 5's, returns them in name order for free,
and a duplicate key silently keeps the last-inserted template, which is
unreachable because a duplicate is always also a diagnostic.

Then the decision, and it is one line of condition. A document with any
diagnostic at all produces `Err` and no templates; a document with none produces
every template it declared. There is no partial success, no "valid keys plus
warnings", and no way for a caller to receive a `Templates` that dropped a key it
could not compile. The aggregation is the point of the shape:
`schema_and_template_failures_are_aggregated_with_source_locations` loads a
five-node document in which every node is faulty in a different way and requires
the *single* refusal to carry a finding for each — a node-shape problem, a
missing required substitution, a doubled optional one, an unknown slot, and a
partial substitution — each with a location. One edit fixes the file rather than
uncovering the next problem, which is what a validator that returned on the first
error could not offer.

Read against the alternative, that is the same decision as `NodeValidation`'s
four fields, one level up. A `validate_document` that returned
`Result<Template, ConfigError>` per node could report the first problem in each
node and could not report the duplicate at all, because a node that failed for
some other reason would have left no key behind to compare. Aggregating costs a
`Vec` and a second walk; it buys every problem in both documents, with locations,
before anything is spawned.

That is the whole of what a successful load has established. Both documents
parsed, every node in both satisfied every rule the vocabulary makes checkable,
no key was declared twice in either, and each key that resolves resolves to one
complete template together with the file it was read from. What none of it
established is what any of those words mean. Chapter 4 reads the rules
themselves — the node shape, the words, the `#` that would silently truncate a
line, and the diagnostics all three of this chapter's aggregating passes carry.

[Previous: The names a template is written against](02-the-names.md) | [Contents](README.md) | [Next: What a template must be](04-template-law.md)

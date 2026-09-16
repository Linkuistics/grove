# Two documents, neither one assembled
<!-- book-page id="two-documents" slice="never-assembled" order="3" -->
[Previous: The names a template is written against](02-the-names.md) | [Contents](README.md) | [Next: What a template must be](04-template-law.md)

<a id="never-assembled"></a>
## Capture once, resolve without reading

`Catalog::load` reads the explicit primary and optional overlay, validates each
flat document and retains its original bytes, parsed KDL and compiled commands.
`Catalog::resolve` applies primary authority and whole-template replacement to
those captured declarations. `Templates::load` is the convenience composition
of those operations with an empty selection. None of these operations decides
which files a consumer should supply.

The running example resolves each key to one whole template. Capture preserves
both original declarations when an overlay replaces a command, and Templates
owns its snapshot independently. Changing or removing a file cannot change
expansion, inspection or diagnostic locations. Flat inspection and structured
diagnostics are available; wrapper composition remains pending.

<a id="one-entry-point"></a>
## One validation path

Both public loading routes reach Catalog's validator. The constructor checks
the vocabulary, then captures both explicit document results even when the
primary fails. It reports read, syntax, shape and duplicate failures before
template-semantic failures. With valid structure, unused flat templates still
undergo eager checking. Errors are ordered by primary/overlay, byte position
and key; a bad document never causes fallback to another configuration.

<!-- fragment «templates-load» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="166-392" parent="source-templates" -->
<!-- insert «templates-load-three-promises» -->
<!-- insert «templates-load-primary» -->
<!-- insert «templates-load-overlay» -->
<!-- insert «templates-load-value» -->
<!-- /fragment -->

The opening fragment states the eager flat validation and fail-closed contract.
Its output is a Catalog rather than an already merged map.

<!-- fragment «templates-load-three-promises» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="166-182" parent="templates-load" -->
````rust
impl Catalog {
    /// Read and fully validate both documents, retaining their original declarations.
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

The next fragment captures the documents and implements selection accessors and
the start of resolution. Flat documents have no selection declaration, so both
accessors return `None`. Any nonempty explicit selection names an unknown
profile and fails. Every selected entry has a diagnostic occurrence with its
zero-based selection index; external selections have no invented span. An empty
selection clones the captured primary command map.
The consumer chooses that list, even when the consumer is the convenience loader.

<!-- fragment «templates-load-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="183-249" parent="templates-load" -->
````rust
        let slots = compile_vocabulary(&vocabulary)?;

        let primary_result = read_primary(primary)
            .and_then(|text| parse_and_validate(primary, text, DocumentRole::Primary, &slots));
        let overlay_result = overlay
            .map(|path| {
                read_overlay(path)
                    .and_then(|text| parse_and_validate(path, text, DocumentRole::Overlay, &slots))
            })
            .transpose();
        let mut diagnostics = Vec::new();
        let primary = primary_result
            .map_err(|error| diagnostics.extend(error.into_diagnostics()))
            .ok();
        let overlay = overlay_result
            .map_err(|error| diagnostics.extend(error.into_diagnostics()))
            .ok();
        if !diagnostics.is_empty() {
            // Structure must pass before semantic reports are meaningful.
            if diagnostics.iter().any(|d| d.category != "invalid_template") {
                diagnostics.retain(|d| d.category != "invalid_template");
            }
            return Err(ConfigError::from_diagnostics(diagnostics));
        }
        let primary = primary.expect("successful primary capture");
        let overlay = overlay.expect("successful overlay capture");
        Ok(Self {
            captured: Arc::new(Captured {
                primary,
                overlay,
                slots,
            }),
        })
    }

    /// Flat documents contain no profile selection declaration.
    #[must_use]
    pub fn primary_selection(&self) -> Option<&Selection> {
        None
    }

    /// Flat overlays contain no profile selection declaration.
    #[must_use]
    pub fn overlay_selection(&self) -> Option<&Selection> {
        None
    }

    /// Resolve captured declarations with primary authority and whole-template
    /// overlay replacement. The returned snapshot owns its inputs independently.
    pub fn resolve(&self, selection: &Selection) -> Result<Templates, ConfigError> {
        if !selection.profiles.is_empty() {
            let diagnostics = selection.profiles.iter().enumerate().map(|(index, profile)| {
                let mut diagnostic = Diagnostic::new("unknown_profile", format!(
                    "unknown profile `{profile}` at selection index {index}; flat configuration declares no profiles."
                ), "Resolve with an empty selection; flat files declare no profiles.");
                diagnostic.primary.clone_from(&selection.origin);
                diagnostic.source = selection.origin.as_ref().map(|span| span.source.clone());
                diagnostic.occurrence_chain.push(Occurrence {
                    id: index, profile: profile.clone(), parent: None,
                    selection_index: index, via: selection.origin.clone(),
                });
                diagnostic
            }).collect();
            return Err(ConfigError::from_diagnostics(diagnostics));
        }
        let mut templates = self.captured.primary.templates.clone();

````
<!-- /fragment -->

<a id="overrides-never-supplies"></a>
## An overlay overrides and never supplies

Resolution visits the captured overlay declarations. `Entry::Occupied` replaces
one complete template. `Entry::Vacant` records an overlay-only key for a later
refusal, without admitting its command. The original overlay declaration remains
in Captured even when it cannot enter the resolved map. This is the authority
boundary; choosing an executable for an admitted key remains the caller's policy.

<!-- fragment «templates-load-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="250-267" parent="templates-load" -->
````rust
        let mut overlay_only = BTreeMap::new();
        if let Some(overlay) = &self.captured.overlay {
            // Each key the primary already declares wins outright: one whole
            // template replaces one whole template, so no rule has to decide
            // which *words* of a launch come from where. A key the primary does
            // not declare is set aside rather than admitted, which is the
            // per-key form of what the old all-keys completeness rule bought.
            for (key, template) in &overlay.templates {
                match templates.entry(key.clone()) {
                    Entry::Occupied(mut occupied) => {
                        occupied.insert(template.clone());
                    }
                    Entry::Vacant(vacant) => {
                        overlay_only.insert(vacant.into_key(), template.span.clone());
                    }
                }
            }
        }
````
<!-- /fragment -->

The resolved value owns the winner map and slot table and shares captured inputs
through `Arc`. `slot_names` lends the captured vocabulary to conformance.
Templates' convenience constructor delegates directly to the same Catalog path,
so it cannot drift into a second reader or different vocabulary rules.

`Catalog::inspect_flat` records both documents before projecting admitted
commands. It sorts declarations by their captured node offsets, assigns origins
in primary-then-overlay order, and accumulates every target assignment. It then
walks histories by key: admitted commands receive the winner's words and origin,
while overlay-only keys receive a reason without a command. Histories preserve
the overwritten primary assignment. Empty documents still appear in `sources`.
No path is reopened, and `selection` is the caller's explicit empty list.

<!-- fragment «templates-load-value» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="268-392" parent="templates-load" -->
````rust

        Ok(Templates {
            inspection: self.inspect_flat(selection, &templates),
            _captured: Arc::clone(&self.captured),
            primary: self.captured.primary.path.clone(),
            overlay: self
                .captured
                .overlay
                .as_ref()
                .map(|source| source.path.clone()),
            slots: self.captured.slots.clone(),
            templates,
            overlay_only,
        })
    }

    /// Record declarations before projecting the winners. Sorting by span keeps
    /// application order independent of the maps' alphabetical lookup order.
    fn inspect_flat(
        &self,
        selection: &Selection,
        templates: &BTreeMap<String, Template>,
    ) -> Inspection {
        let mut view = Inspection {
            sources: Vec::new(),
            selection: selection.clone(),
            profile_occurrences: Vec::new(),
            commands: Vec::new(),
            non_admitted_keys: Vec::new(),
            origins: Vec::new(),
            histories: Vec::new(),
        };
        let mut histories: BTreeMap<String, Vec<Assignment>> = BTreeMap::new();
        for (document, role) in std::iter::once((&self.captured.primary, SourceRole::Primary))
            .chain(
                self.captured
                    .overlay
                    .as_ref()
                    .map(|document| (document, SourceRole::Overlay)),
            )
        {
            view.sources.push(Source {
                role,
                path: document.path.clone(),
            });
            let mut declarations: Vec<_> = document.templates.iter().collect();
            declarations.sort_by_key(|(_, template)| template.span.start);
            for (key, template) in declarations {
                let id = view.origins.len();
                view.origins.push(Origin {
                    id,
                    span: template.span.clone(),
                    occurrence: None,
                });
                histories.entry(key.clone()).or_default().push(Assignment {
                    order: id,
                    value: AssignmentValue::LiteralTemplate(template.text.clone()),
                    origin: id,
                });
            }
        }
        for (key, assignments) in histories {
            let id = view.histories.len();
            // Every history was created by a declaration. Its final assignment
            // supplies the whole flat template, with no synthetic binding.
            let origin = assignments
                .last()
                .expect("a declared target has an assignment")
                .origin;
            if let Some(template) = templates.get(&key) {
                view.commands.push(CommandView {
                    key: key.clone(),
                    binding: None,
                    command: None,
                    parameters: Vec::new(),
                    words: template
                        .words
                        .iter()
                        .map(|word| WordView {
                            word: word.clone(),
                            origins: vec![origin],
                        })
                        .collect(),
                    origins: vec![origin],
                    histories: vec![id],
                });
            } else {
                view.non_admitted_keys.push(NonAdmittedKey {
                    key: key.clone(),
                    origins: assignments.iter().map(|a| a.origin).collect(),
                    reason: "Only the overlay declares this key; primary policy must authorize it."
                        .to_owned(),
                });
            }
            view.histories.push(AssignmentHistory {
                id,
                setting: Setting::RouteTarget { key },
                assignments,
            });
        }
        view
    }

    pub(crate) fn slot_names(&self) -> impl Iterator<Item = &str> {
        self.captured.slots.iter().map(|slot| slot.name.as_str())
    }
}

impl Templates {
    /// Explain this captured resolution without reading sources or launching.
    #[must_use]
    pub fn inspect(&self) -> &Inspection {
        &self.inspection
    }

    /// Load a Catalog and resolve an empty explicit selection through the same
    /// validation path. Source discovery and selection policy belong to the caller.
    pub fn load(
        primary: &Path,
        overlay: Option<&Path>,
        vocabulary: Vocabulary<'_>,
    ) -> Result<Self, ConfigError> {
        Catalog::load(primary, overlay, vocabulary)?.resolve(&Selection::default())
    }

````
<!-- /fragment -->

<a id="both-documents"></a>
## Two files, one captured result

For a primary declaring `impl` and `review-impl`, and an overlay replacing only
`impl`, resolution keeps the primary review command and the overlay impl command.
`source("impl")` names the overlay; `source("review-impl")` names the primary.
`validate_node` and `validate_template` are the per-node and per-template rule
checks `validate_document` drives over both documents; each returns diagnostics
with locations rather than stopping at the first. Chapter 4 explains those rules.

`crates/keyed-launch/tests/catalog.rs` uses the unrelated key `opaque` and runtime
slot `payload` to test the same boundary. It edits the primary and removes the
overlay before resolution and conformance, then removes the primary and drops
Catalog before expansion. Exact expected words prove both snapshot independence
and equality with the convenience loader; native non-UTF-8 slot bytes survive.

<a id="the-second-ending"></a>
## A local declaration cannot admit a key

A valid overlay-only command is retained but does not appear in `keys()` and has
no `source()`. `require` and `expand` refuse it, naming the primary file where the
key must be declared. Validation and admission are distinct: an invalid
overlay-only command still prevents loading, because flat template checks are
eager in both documents. A valid one lets other admitted keys resolve normally.

<a id="the-slot-table-first"></a>
## The slot table first, and the duplicate it refuses

The rest of the chapter is the five functions `load` reaches, in the order it
reaches them. They are free functions rather than methods because none of them
needs a `Templates` — they run before one exists — and all five are private:
`Catalog::load` is their entry point; Catalog resolution then produces
Templates without invoking them again. The block is read in seven fragments.

<!-- fragment «reading-and-whole-document-validation» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="575-778" parent="source-templates" -->
<!-- insert «compile-vocabulary» -->
<!-- insert «read-primary» -->
<!-- insert «read-overlay» -->
<!-- insert «parse-and-validate» -->
<!-- insert «validate-document-nodes» -->
<!-- insert «validate-document-duplicates» -->
<!-- insert «validate-document-report» -->
<!-- /fragment -->

`compile_vocabulary` is the first, and it validates names before copying them: it
copies the consumer's borrowed `SlotRule`s into the owned `SlotSpec` table that
`Templates` will keep, and it refuses duplicate names and names in the reserved `param.` namespace. Its comment is the only
statement anywhere in the crate of why the duplicate is a refusal rather than a
tolerated redundancy, and the reason is that the failure it would otherwise cause
is silent and lands on the wrong file.

<!-- fragment «compile-vocabulary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="575-612" parent="reading-and-whole-document-validation" -->
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
        if rule.name.starts_with("param.") {
            return Err(ConfigError::new(
                "invalid_value",
                format!(
                    "slot `{}` uses the reserved `param.` prefix; choose a runtime slot name \
                 outside the configuration parameter namespace",
                    rule.name
                ),
                "Use unique runtime slot names outside the reserved `param.` namespace.",
            ));
        }
        if slots.iter().any(|slot| slot.name == rule.name) {
            return Err(ConfigError::new(
                "invalid_value",
                format!(
                    "the slot vocabulary declares `{}` more than once",
                    rule.name
                ),
                "Use unique runtime slot names outside the reserved `param.` namespace.",
            ));
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

<!-- fragment «read-primary» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="613-637" parent="reading-and-whole-document-validation" -->
````rust

fn read_primary(path: &Path) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|error| {
        let message = if error.kind() == ErrorKind::NotFound {
            format!("configuration is missing at {}", path.display())
        } else {
            format!(
                "failed to read the configuration at {}: {error}",
                path.display()
            )
        };
        ConfigError::new(
            "source_read",
            message,
            "Create a readable UTF-8 configuration at this path.",
        )
        .contextualize(
            Some(Source {
                role: SourceRole::Primary,
                path: path.to_owned(),
            }),
            None,
        )
    })
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

<!-- fragment «read-overlay» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="638-657" parent="reading-and-whole-document-validation" -->
````rust

fn read_overlay(path: &Path) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|error| {
        ConfigError::new(
            "source_read",
            format!(
                "failed to read the configuration overlay at {}: {error}",
                path.display()
            ),
            "Make the selected overlay readable as UTF-8, or stop selecting it.",
        )
        .contextualize(
            Some(Source {
                role: SourceRole::Overlay,
                path: path.to_owned(),
            }),
            None,
        )
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

<!-- fragment «parse-and-validate» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="658-695" parent="reading-and-whole-document-validation" -->
````rust

fn parse_and_validate(
    path: &Path,
    source: String,
    role: DocumentRole,
    slots: &[SlotSpec],
) -> Result<CapturedDocument, ConfigError> {
    let document: KdlDocument = source.parse().map_err(|error: kdl::KdlError| {
        let location = source_location(&source, error.span.offset());
        let mut diagnostic = Diagnostic::new(
            "kdl_syntax",
            format!(
                "{}:{}:{}: KDL syntax error: {}",
                path.display(),
                location.line,
                location.column,
                error
            ),
            "Correct the KDL syntax at the reported location.",
        );
        let source = role.source(path);
        diagnostic.primary = Some(SourceSpan {
            source: source.clone(),
            start: error.span.offset(),
            end: error.span.offset() + error.span.len(),
        });
        diagnostic.source = Some(source);
        ConfigError::from_diagnostics(vec![diagnostic])
    })?;

    let templates = validate_document(path, &source, &document, role, slots)?;
    Ok(CapturedDocument {
        path: path.to_path_buf(),
        _source: source,
        _document: document,
        templates,
    })
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

On success, `CapturedDocument` retains the source String, parsed KDL document,
and validated map together. The parsed spans therefore still refer to the exact
bytes that were read, even after the path changes.

A syntax error stops validation of that document because there is no parsed
node list to walk. Its parser byte range becomes a `kdl_syntax` diagnostic.
Catalog still collects errors from the other explicit document. DocumentRole
attaches source identity to syntax and node reports; it changes no validity rule.

<a id="one-refusal"></a>
## Every diagnostic in one refusal

`validate_document` is the last function in the block and the one that decides
what a refusal looks like. It runs in three passes over one document, and the
three fragments below are those passes. The first walks the nodes, recording each
node's validation and building an index from key to every location that key was
declared at.

<!-- fragment «validate-document-nodes» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="696-715" parent="reading-and-whole-document-validation" -->
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

<!-- fragment «validate-document-duplicates» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="716-747" parent="reading-and-whole-document-validation" -->
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
            category: "duplicate",
            key: Some(key.clone()),
            related: locations.iter().skip(1).copied().collect(),
            remedy: "Keep one declaration per key in each document.",
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

<!-- fragment «validate-document-report» owner="never-assembled" source="crates/keyed-launch/src/templates.rs" lines="748-778" parent="reading-and-whole-document-validation" -->
````rust

    let mut templates = BTreeMap::new();
    for validation in validations {
        diagnostics.extend(validation.diagnostics);
        if let Some(words) = validation.template {
            templates.insert(
                validation.key,
                Template {
                    text: validation.text,
                    span: SourceSpan {
                        source: role.source(path),
                        start: validation.location.start,
                        end: validation.location.end,
                    },
                    words,
                    source: path.to_path_buf(),
                },
            );
        }
    }

    if !diagnostics.is_empty() {
        return Err(ConfigError::from_diagnostics(render_diagnostics(
            path,
            role,
            diagnostics,
        )));
    }

    Ok(templates)
}
````
<!-- /fragment -->

Duplicate and per-node findings enter one vector. `render_diagnostics` sorts
them by source position and key, so duplicates do not jump ahead of an earlier
shape failure. Templates are collected into a BTreeMap for deterministic keys.
A duplicate may replace a temporary map entry, but that map cannot escape:
any diagnostic makes the whole document fail.

A document with diagnostics returns an error rather than a partial command
map. Catalog combines that error with the other document result, reporting
structure first and semantics only after structure passes. The template
aggregation test supplies structurally valid declarations with missing, doubled,
unknown and embedded substitutions, then checks that all those independent
failures appear together with locations. The structured diagnostics suite covers
the separate shape phase and cross-file ordering.

NodeValidation retains the key and location even when its node is malformed,
so duplicate checking can still identify every declaration. The resulting load
is all-or-nothing: no invalid template or partial source set reaches expansion.

That is the whole of what a successful load has established. Both documents
parsed, every node in both satisfied every rule the vocabulary makes checkable,
no key was declared twice in either, and each key that resolves resolves to one
complete template together with the file it was read from. What none of it
established is what any of those words mean. Chapter 4 reads the rules
themselves — the node shape, the words, the `#` that would silently truncate a
line, and the diagnostics all three of this chapter's aggregating passes carry.

[Previous: The names a template is written against](02-the-names.md) | [Contents](README.md) | [Next: What a template must be](04-template-law.md)

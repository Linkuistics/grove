# From a template to an argv
<!-- book-page id="to-an-argv" slice="whole-word-or-nothing" order="5" -->
[Previous: What a template must be](04-template-law.md) | [Contents](README.md) | [Next: Appearance is the event](06-the-channel.md)

<a id="whole-word-or-nothing"></a>
## Whole word, or nothing

Chapter 4 read every rule a template must satisfy and the five functions that
turn a broken one into a sentence an operator can act on. All of those rules bind
at load, and none of them binds again. This chapter is what happens afterwards:
the rest of the public `impl` that chapter 3 stepped over on its way
from `load` to the reading and validation `load` drives — ten more lines at the
very end of `src/templates.rs`, and the whole of `src/argv.rs`.

What this stage must not add and must not interpret is **the value**. A caller
hands the crate a name and an `OsStr`; the crate finds the position that name
occupies in the slot table, and puts the `OsStr` at the corresponding position of
the argv. It rejects NUL, which cannot be passed as part of a process argument.
It does not split, quote, unquote or expand the value, or compare it with another
value. Other native bytes pass through unchanged.
Substitution is whole-word or nothing, and that phrase binds in two places rather
than one: chapter 4 refused a template in which a substitution was less than a
whole word, and this chapter never re-reads the word that substitution produced.

That completes the book's second arm. A layer learns what a value means **on the
way through** by re-reading a value it has already read, and the cost is that a
value with a space becomes two arguments and a `$(…)` becomes a command. Chapter
4 owned the third of that arm where declining to interpret was not enough on its
own — the `#`, which a dependency would otherwise have interpreted on the crate's
behalf. The other two-thirds are here, and they are cheaper than the `#` was,
because they are structural rather than defensive: the template was split into
words once, at load, and there is no line in this chapter that can put two words
together or take one apart.

Two further things are settled here. Expansion's one remaining obligation is
stated over the **vocabulary** rather than over the template being expanded,
which is what stops a consumer having a call that works for one key and fails for
its neighbour. And the seam chapter 1 claimed — that the crate's two halves meet
only at `Argv`, so nothing reaches a spawn that a template did not author — stops
being a claim about the design and becomes a fact about the types. This chapter
reads the line that makes `Argv::new` `pub(crate)` and counts the callers it has.

<a id="four-words"></a>
## Four words

This section takes step 2 of the five-call trace chapter 1 wrote, and expands it
to full resolution. Chapter 3 added an overlay to the picture; this section sets
it aside and loads the primary alone, because what is being traced is expansion,
and which file a template was read from changes nothing about it. The input is
the operator's personal file and the vocabulary its lines are written against.

```text
~/.config/grove/config.kdl
  impl "claude --model opus ${prompt}"
  review-impl "codex exec --model gpt-5 ${prompt}"

vocabulary
  prompt        ExactlyOnce
  session_name  AtMostOnce
  worktree      AtMostOnce
  repo          AtMostOnce
```

That figure is the whole input to the call below: two keys, and the four names any
template of theirs is allowed to mention. `Templates::load(primary, None,
vocabulary)` returns, and chapter 4 fixed what `impl` compiled to — three literals
and one slot reference, where the reference is a *position* in the four-name table
rather than the name itself.

```text
[Word::Literal("claude"), Word::Literal("--model"), Word::Literal("opus"), Word::Slot("prompt".into())]
```

Now grove offers values. It offers **four** of them, one for every slot the
vocabulary declares, even though this template mentions exactly one. The mandate
is spelled out here for the first time — chapter 1 wrote it as `<the mandate>`
because nothing before this point depended on what was inside it, and here
everything does. It holds five spaces and a `$(…)`, and both of those are the
point.

```text
templates.expand(
    "impl",
    &[
        Slot { name: "prompt",       value: "Fix the $(date) helper in scripts/check.sh" },
        Slot { name: "session_name", value: "atlas-impl-k41" },
        Slot { name: "worktree",     value: "/work/atlas" },
        Slot { name: "repo",         value: "/src/atlas" },
    ],
)
```

The last two differ because this is a secondary workspace; in a single-worktree
repository they would be the same path, and the crate would not notice either
way. The call returns `Ok`, and what it returns is the observable end of the
step:

```text
Argv {
    program: "claude",
    args: ["--model", "opus", "Fix the $(date) helper in scripts/check.sh"],
}

argv.words()
  ["claude", "--model", "opus", "Fix the $(date) helper in scripts/check.sh"]
```

The table below is the whole of what expansion did, position by position; it is
what the reader should take from this section, because every remaining section of
the chapter is one column of it read closely.

| Position | Compiled word | Filled from | The word the child receives |
|---:|---|---|---|
| 0 | `Word::Literal("claude")` | the file | `claude` |
| 1 | `Word::Literal("--model")` | the file | `--model` |
| 2 | `Word::Literal("opus")` | the file | `opus` |
| 3 | `Word::Slot("prompt".into())` | `offered["prompt"]`, the value offered for `prompt` | `Fix the $(date) helper in scripts/check.sh` |

Four words in, four words out. The mandate is one argument by construction: the
only splitting this crate ever does happened at load, to the template, and a
value offered at expansion never passes through it. In
`crates/keyed-launch/tests/templates.rs`,
`a_slot_value_is_one_argument_whatever_it_contains` pins exactly that shape: it
expands a template with the value `two words $(not a command)` and asserts four
words out. Its companion `shell_metacharacters_stay_literal` pins the other
direction — a `|` and a `>` written in the *template* are quoted words in the
file and stay single-character arguments — so neither side of the substitution
can reach a shell, because no shell is involved at any point.

Three of the four offered values were never mentioned by this template.
`session_name`, `worktree` and `repo` were checked, placed in a vector, and then
not read. That is not waste; it is the obligation, and the section on
`match_values` below is where the reason lives.

<a id="four-questions"></a>
## What the block answers

The following table collects the inspection, lookup and expansion operations.
Each answers one question about a captured snapshot. The tests named here use
the public API in `tests/templates.rs` and `tests/inspection.rs`.

| Function | Answers | Refuses with | Pinned by |
|---|---|---|---|
| `inspect` | captured sources, histories and compiled words | — | `captured_flat_inspection_retains_history_and_matches_expansion` |
| `source` | which file this key's template was read from | `None` is an answer, not a refusal | `an_overlay_replaces_a_whole_template_and_reports_its_own_path`, `a_key_only_the_overlay_declares_does_not_resolve` |
| `require` | does this key resolve to exactly one complete template | `` key `k` does not resolve: … `` | `a_key_nobody_declares_names_the_primary_file`, `a_key_only_the_overlay_declares_does_not_resolve` |
| `expand` | this key's template plus these values, as an argv | both of `require`'s and all three of `match_values`'s | `a_slot_value_is_one_argument_whatever_it_contains`, `program_and_arguments_split_at_word_zero`, `shell_metacharacters_stay_literal` |
| `match_values` | which offered value fills which declared slot | `` no slot named `n` is declared ``, `` slot `n` was offered more than one value ``, `` no value offered for declared slot: n `` | `expansion_refuses_values_that_do_not_fill_the_vocabulary` |
| `declared_slots` | what the declared names are, for one message | — | — |
| `unresolved` | why this key did not resolve, in the reader's terms | it owns both wordings `require` uses | `a_key_only_the_overlay_declares_does_not_resolve`, `a_key_nobody_declares_names_the_primary_file` |
| `keys` | which keys resolve at all | — | `an_unused_key_is_not_an_error` |

The composite below is the block as a whole, and it sits in `src/templates.rs`
between the two halves of chapter 3's reading. Those are ownership-block
boundaries rather than function boundaries. Chapter 3 captures and resolves the
inputs; this block reads the snapshot; the later validation block defines the
rules capture applies. The
interleaving is the cost of ordering the book by concept, and the ownership
ledger in the source index is where it is visible: eight blocks of one root,
divided across four chapters.

<!-- fragment «resolution-and-expansion» owner="whole-word-or-nothing" source="crates/keyed-launch/src/templates.rs" lines="282-463" parent="source-templates" -->
<!-- insert «templates-source» -->
<!-- insert «templates-require» -->
<!-- insert «templates-expand» -->
<!-- insert «match-values» -->
<!-- insert «declared-slots» -->
<!-- insert «templates-unresolved» -->
<!-- /fragment -->

<a id="which-file"></a>
## Which file this key came from

`source` is the narrow provenance lookup beside the full inspection view. It takes a key and returns the path of the
file that supplied that key's template, or `None`. The actor is the `Templates`
value itself, reading the per-key `source` field chapter 2 read the comment on;
the input is a borrowed key and the output is a borrowed path, so the caller
learns which file to name in its own diagnostics without the configuration having
to be re-read.

<!-- fragment «templates-source» owner="whole-word-or-nothing" source="crates/keyed-launch/src/templates.rs" lines="282-290" parent="resolution-and-expansion" -->
````rust
    /// The file this key's template was actually read from — the primary file,
    /// or the overlay that overrode it. `None` when the primary does not declare
    /// it, whatever the overlay says.
    #[must_use]
    pub fn source(&self, key: &str) -> Option<&Path> {
        self.templates
            .get(key)
            .map(|template| template.source.as_path())
    }
````
<!-- /fragment -->

Three lines of comment carry the whole rule, and the last clause of them is the
one worth slowing down for: `None` **when the primary does not declare it,
whatever the overlay says**. The map is `templates`, and chapter 3 read the loop
that fills it — a key the overlay declares and the primary does not takes the
`Entry::Vacant` arm, where the template is dropped and only the key is kept, in
`overlay_only`. So there is no entry to `get`, and this function's answer for
such a key is the same `None` it gives for a key nobody has ever written down.

That is one consequence of *the untracked configuration delta*; the section on
`unresolved` below is where the record is actually kept, and this is where it
first shows. The record settles that a kind resolves only if the personal
file declares it, and that where only the delta declares one, the kind does not
resolve. An earlier form of the rule got that for free from a quantifier over all
kinds — a delta could only override something already written down — and the
quantifier is gone, because nothing in the system can enumerate the kinds a
methodology declares. What replaced it is per-key and just-in-time, and this is
what *just-in-time* costs: `load` returned `Ok`, `source` returns `None` rather
than an error, and nothing is refused until the key is used.
`a_key_only_the_overlay_declares_does_not_resolve` asserts precisely this pair —
`source("two")` is `None`, and only the next call is a refusal.

The two-file case is the other half of what the function is for.
`an_overlay_replaces_a_whole_template_and_reports_its_own_path` loads a primary
declaring `one` and `two` and an overlay declaring `two`, then asserts that
`source("one")` is the primary and `source("two")` is the overlay. After an
overlay resolves there is no single answer to *which file did this configuration
come from*, and a diagnostic that named one file for every key would point half
its readers at a file that never held the failing template.

<a id="before-committing"></a>
## The question asked before a key is committed to

`require` is the whole of what a consumer must do before it acts on a key. Its
input is a key and its output is `Ok(())` or a refusal; it reads the same map
`source` reads and adds nothing to what that map already knows. What it
establishes is a precondition rather than a value, which is why it exists at all
as a separate call from `expand`.

The refusal is `unconfigured_key`. Its source names the primary that must
admit the key, and its remedy names that key. An overlay-only key contributes
its captured declaration as a related span. `expand` returns this same refusal
before matching runtime values. Runtime mismatches use `invalid_value`, carry
the requested key and winning source, and have no fabricated file span.

<!-- fragment «templates-require» owner="whole-word-or-nothing" source="crates/keyed-launch/src/templates.rs" lines="291-316" parent="resolution-and-expansion" -->
````rust

    /// Does this key resolve to exactly one complete template?
    ///
    /// The obligation a consumer discharges *before* it commits to a key —
    /// before it writes down work of that kind, or launches it — stated once,
    /// here, so the refusal's wording has one owner. [`Self::expand`] asks the
    /// same question on its own way in.
    pub fn require(&self, key: &str) -> Result<(), ConfigError> {
        if self.templates.contains_key(key) {
            return Ok(());
        }
        let mut diagnostic = Diagnostic::new(
            "unconfigured_key",
            self.unresolved(key),
            &format!("Declare `{key}` in {}.", self.primary.display()),
        );
        diagnostic.source = Some(Source {
            role: SourceRole::Primary,
            path: self.primary.clone(),
        });
        diagnostic.key = Some(key.to_owned());
        if let Some(span) = self.overlay_only.get(key) {
            diagnostic.related.push(span.clone());
        }
        Err(ConfigError::from_diagnostics(vec![diagnostic]))
    }
````
<!-- /fragment -->

The doc comment names the moment: *before it writes down work of that kind, or
launches it*. Both halves of that are real calls in grove.
`crates/grove-loop/src/session_config.rs` wraps this call as its own `require`,
and `crates/grove-llm/src/cli.rs` calls that wrapper before it writes a leaf of a
kind — a task tree that records a kind nothing can launch is a tree whose work
cannot be done, and the earliest moment to report that is before the entry
exists. Reporting it only at launch would arrive after the operator had already
committed the entry.

The rest of the comment is the reason the function is not merely a `contains_key`
a caller could write itself: *stated once, here, so the refusal's wording has one
owner*. Two call sites in the crate reach a key that does not resolve — this one
and `expand` — and if each had built its own message the two would drift, and an
operator would get one sentence when a leaf was written and a different one when
it was launched, for the same misplaced key. The body decides nothing about
wording at all: the early return covers the resolving case, and the last line
delegates to `unresolved`, which has exactly one caller and it is that line.

<a id="one-obligation"></a>
## Expansion's one remaining obligation

`expand` is the crate's configuration half in one function: a key and a set of
offered values in, an `Argv` out. Everything it could have checked was checked at
load, so what remains is a single question about the caller's values — and the
comment on it is the longest in the block precisely because that question is
stated over something other than what a reader would first expect.

<!-- fragment «templates-expand» owner="whole-word-or-nothing" source="crates/keyed-launch/src/templates.rs" lines="317-368" parent="resolution-and-expansion" -->
````rust

    /// Expand this key's template into an argv.
    ///
    /// The values must fill the slots the vocabulary declared: one value per
    /// declared slot, no duplicates, no name the vocabulary does not hold, and
    /// no NUL in any offered value, even for an unused optional slot. Every
    /// other template rule was checked
    /// at load — and it is stated over the vocabulary rather than over this
    /// template's own words so a consumer cannot have a call that works for one
    /// key and fails for its neighbour purely because the two templates mention
    /// different optional slots.
    pub fn expand(&self, key: &str, values: &[Slot<'_>]) -> Result<Argv, ConfigError> {
        self.require(key)?;
        let template = &self.templates[key];
        let offered = self.match_values(values).map_err(|error| {
            let role = if self.overlay.as_ref() == Some(&template.source) {
                SourceRole::Overlay
            } else {
                SourceRole::Primary
            };
            error.contextualize(
                Some(Source {
                    role,
                    path: template.source.clone(),
                }),
                Some(key),
            )
        })?;

        let offered: HashMap<_, _> = self
            .slots
            .iter()
            .map(|slot| slot.name.as_str())
            .zip(offered)
            .collect();
        let mut words = Vec::with_capacity(template.words.len());
        for word in &template.words {
            words.push(match word {
                Word::Literal(value) => OsString::from(value),
                Word::Slot(name) => offered[name.as_str()].to_owned(),
            });
        }

        // Word zero is a literal non-empty executable, checked at load for every
        // template in both documents, so the split below cannot fail on a
        // template this type holds.
        let mut words = words.into_iter();
        let program = words
            .next()
            .expect("a validated template has at least one word");
        Ok(Argv::new(program, words.collect()))
    }
````
<!-- /fragment -->

Read the obligation exactly as it is written: *the values must fill the slots the
**vocabulary** declared* — one value per declared slot, no duplicates, no name
the vocabulary does not hold. Not the slots this template mentions. The template
being expanded is not consulted about which values are acceptable at all.

The consequence is the one the comment states, and it is worth having in front of
the reader as a comparison rather than a sentence, because it is a difference
between two rules over the same four-slot vocabulary and the same two templates:

| | Obligation over the vocabulary | Obligation over this template |
|---|---|---|
| `expand("impl", …)`, template mentions `${prompt}` | four values required | one value required |
| `expand("review-impl", …)`, template mentions `${prompt}` | four values required | one value required |
| if the operator adds `${session_name}` to `impl` alone | four values, unchanged | `impl` now needs two, `review-impl` still one |
| what the consumer must know to make a valid call | the vocabulary it declared | the contents of each template it did not write |

The right-hand column is the rejected design, and the last row is why it was
rejected. A consumer's call site would work for one key and fail for its
neighbour purely because the two templates mention different optional slots — and
which optional slots a template mentions is the *operator's* choice, made in a
file the consumer never reads. The consumer would have to inspect a template to
know how to expand it, which is the crate handing back exactly the knowledge it
exists to keep. Under the rule as written, one call shape is valid for every key
in the configuration, and it is derivable from the vocabulary the consumer itself
declared. `expansion_refuses_values_that_do_not_fill_the_vocabulary` pins it from
both sides: a template mentioning only `${prompt}`, expanded with only a `prompt`
value, is refused with `no value offered for declared slot: label`.

Decision 7 of `docs/specs/module-decomposition.md` argues that the vocabulary is
an input to `load` and not to `expand`, because every template rule is a rule
about slot names and none of them is checkable by a loader that will not learn
the names until expansion. Chapter 2 read that argument at the signature and
chapter 4 read the rules it makes checkable. The decision closes with a
consequence rather than a rule — supplied at load, the vocabulary leaves
expansion with one obligation, that the values offered fill the slots it declared
— and this comment is that consequence written down. `match_values`, below, is
the line that keeps it.

Expansion first calls `require`, so an unknown key fails before runtime values
are examined. `match_values` checks the complete vocabulary and returns borrowed
values in its order; zipping those values with the slot names creates the lookup.
The compiled-word loop then copies a literal into an `OsString`, or copies the
native value associated with a `Word::Slot(name)`. This is the same named word
representation inspection exposes. Neither branch splits or interprets the
value, so spaces and non-Unicode bytes remain inside one argument. The matching
step also refuses NUL in every offered value, including optional slots absent
from this template. Its `invalid_value` diagnostic names the slot; expansion
adds the key and template source without inventing a source range for caller
data. `nul_runtime_values_fail_even_for_unused_optional_slots` checks that
boundary, and the native-string regression checks that non-Unicode bytes survive.

The final split separates word zero from the arguments. The split cannot fail, and the reason it cannot is
in another chapter: word zero is checked at load to be a literal and non-empty,
for every template in both documents, so a `Templates` value cannot hold a
template with no words. The `expect` message says as much. `Argv::new` receives
the program and the remaining words, and it is the last line of the crate's
configuration half.

<a id="by-name"></a>
## Lining the offered values up

`match_values` is the check the comment on `expand` promised, and it is a private
function because the vector it produces has no meaning outside the one call that
consumes it. Its input is the caller's slice of `Slot` values; its output is a
vector of borrowed `OsStr`s in vocabulary order. Expansion pairs them with
validated names before it walks the compiled words.

<!-- fragment «match-values» owner="whole-word-or-nothing" source="crates/keyed-launch/src/templates.rs" lines="369-431" parent="resolution-and-expansion" -->
````rust

    /// Line up the offered values with the declared slots, by name.
    fn match_values<'v>(
        &self,
        values: &[Slot<'v>],
    ) -> Result<Vec<&'v std::ffi::OsStr>, ConfigError> {
        let mut offered: Vec<Option<&std::ffi::OsStr>> = vec![None; self.slots.len()];
        for value in values {
            let Some(index) = self.slots.iter().position(|slot| slot.name == value.name) else {
                return Err(ConfigError::new(
                    "invalid_value",
                    format!(
                        "no slot named `{}` is declared; declared slots: {}",
                        value.name,
                        self.declared_slots()
                    ),
                    "Supply exactly one value for each declared runtime slot and no other names.",
                ));
            };
            if offered[index].is_some() {
                return Err(ConfigError::new(
                    "invalid_value",
                    format!("slot `{}` was offered more than one value", value.name),
                    "Supply exactly one value for each declared runtime slot and no other names.",
                ));
            }
            // OsStr's encoding preserves ASCII, so NUL can be checked without
            // converting native strings to Unicode (stable since Rust 1.74):
            // https://doc.rust-lang.org/1.85.0/std/ffi/struct.OsStr.html#method.as_encoded_bytes
            if value.value.as_encoded_bytes().contains(&0) {
                return Err(ConfigError::new(
                    "invalid_value",
                    format!("runtime slot `{}` contains NUL", value.name),
                    "Remove NUL from the runtime slot value before expansion.",
                ));
            }
            offered[index] = Some(value.value);
        }

        let missing = self
            .slots
            .iter()
            .zip(&offered)
            .filter(|(_, value)| value.is_none())
            .map(|(slot, _)| slot.name.as_str())
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            return Err(ConfigError::new(
                "invalid_value",
                format!(
                    "no value offered for declared slot{}: {}",
                    if missing.len() == 1 { "" } else { "s" },
                    missing.join(", ")
                ),
                "Supply exactly one value for each declared runtime slot and no other names.",
            ));
        }

        Ok(offered
            .into_iter()
            .map(|value| value.expect("checked"))
            .collect())
    }
````
<!-- /fragment -->

The vector starts as one `None` per declared slot, in vocabulary order, and the
function fills it. Names are compared with `==` inside `position`, which is the
third and last of the three sites chapter 2 counted: `compile_vocabulary`
compares names to reject a duplicate, `parse_template_word` compares them to
resolve a substitution, and this compares them to pair a value with a slot. No
site anywhere in the crate branches on a *particular* name, and this one is the
easiest place to see why that is a property rather than an accident — the
function has no way to say `prompt` even if it wanted to.

Four refusals come out of it, and they are found at two different times, which
is a distinction the messages themselves make visible.

| Refusal | Found | Message |
|---|---|---|
| a name the vocabulary does not declare | during the loop, at the first offending value | `` no slot named `mandate` is declared; declared slots: prompt, session_name, worktree, repo `` |
| two values for one slot | during the loop, at the second value for that slot | `` slot `prompt` was offered more than one value `` |
| a runtime value containing NUL | during the loop, after its name and uniqueness checks | `` runtime slot `prompt` contains NUL `` |
| a declared slot with no value | after the loop, over all of them at once | `` no value offered for declared slots: worktree, repo `` |

The first three return on the first offending value in caller order. Within one
value, an unknown name takes precedence over a duplicate, which takes precedence
over NUL. Missing values are collected only after every offered value has passed
those checks. That final message names all missing slots, joined with `, `, and
pluralises its noun: `declared slot: label` or `declared slots: worktree, repo`.

`declared_slots` exists for the unknown-name message and for nothing else.
It has exactly one call site, in `match_values` above it.

<!-- fragment «declared-slots» owner="whole-word-or-nothing" source="crates/keyed-launch/src/templates.rs" lines="432-439" parent="resolution-and-expansion" -->
````rust

    fn declared_slots(&self) -> String {
        self.slots
            .iter()
            .map(|slot| slot.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
````
<!-- /fragment -->

It joins the declared names in vocabulary order — the order the consumer wrote
them in, not sorted — so an operator who reads `declared slots: prompt,
session_name, worktree, repo` sees the list in the shape their own code declares
it. It is a `String` rather than a `Vec<&str>` because its one caller
interpolates it directly. It is a method rather than a free function taking
`&[SlotSpec]` — which the file's own `parse_template_word` shows would compile —
because it is a question about a loaded configuration rather than a step in
building one, and every free function in this file is the second sort.

The message it feeds is the only one of the three that tells the caller what the
alternatives were. The other two name a slot the caller already knows about, so
listing the vocabulary would be noise; an unknown name is the one case where the
caller's model of the vocabulary is wrong, and the correction has to be printed
or the caller has nowhere to look. This is `ConfigError`'s obligation from
chapter 1 — name what is wrong, name where, name what fixes it — discharged for a
failure that has no file and no line: *where* is the call, and *what fixes it* is
the list.

<a id="two-wordings"></a>
## The refusal, in two wordings

`unresolved` is the last function of the block and the one carrying the record
this chapter keeps. It takes a key that failed `require` and returns the sentence
the operator will read. It has one caller, and it is the reason `require` exists
as a named obligation rather than as a `contains_key` at each call site.

<!-- fragment «templates-unresolved» owner="whole-word-or-nothing" source="crates/keyed-launch/src/templates.rs" lines="440-463" parent="resolution-and-expansion" -->
````rust

    /// The refusal for a key that does not resolve — naming the key and the
    /// primary file that must declare it, and saying so differently when the
    /// overlay does declare it, because that reader has written the key down and
    /// needs to know it is in the wrong file rather than misspelled.
    fn unresolved(&self, key: &str) -> String {
        if self.overlay_only.contains_key(key) {
            let overlay = self.overlay.as_deref().map_or_else(
                || "the overlay".to_owned(),
                |path| path.display().to_string(),
            );
            return format!(
                "key `{key}` does not resolve: it is declared only in the configuration overlay \
                 at {overlay}, and an overlay overrides a key the primary declares but never \
                 supplies one of its own.\n  Declare `{key}` in {primary}.",
                primary = self.primary.display()
            );
        }
        format!(
            "key `{key}` does not resolve: no template for it.\n  Declare `{key}` in {primary}.",
            primary = self.primary.display()
        )
    }
}
````
<!-- /fragment -->

Two wordings, chosen by membership in the non-admitted-key map. `overlay_only` is the field chapter 2
found had an argument attached to it — the keys an overlay declares and the
primary does not, kept rather than dropped — and this is the only thing in the
crate that reads it. The choice it makes is not about correctness; both messages
would be true. It is about which mistake the reader has actually made.

| The key is | The reader has | The wording tells them |
|---|---|---|
| in `overlay_only` | written the key down, in the overlay | it is in the wrong file, names that file, and says what an overlay may do |
| in neither document | not written the key down anywhere | there is no template for it |
| in both branches | — | `` Declare `k` in <primary>. `` |

The first row is *the untracked configuration delta* at the point where the
record is actually kept. The record's own property is that a delta overrides and
never supplies, and its stated reason is that this is what stands between an
untracked file a project ships and a program its operator never chose. That
property is enforced in chapter 3, in the `Entry::Vacant` arm — but a reader who
hits it does not see an arm of a match. They see this sentence, and it is the
sentence that has to carry the reason, because the operator looking at it is
holding a file with the key plainly written in it and would otherwise conclude
the crate cannot read. So the message names the overlay's path, states the rule
in the same breath, and then tells them which file to move the declaration to.
`a_key_only_the_overlay_declares_does_not_resolve` asserts the overlay's path and
the fix on the `require` path, and then asserts the rule clause on the `expand`
path, because that is the other moment a key is committed to and the sentence has
to be the same one.

The second row's message is shorter for a reason that is visible only by
comparison: there is nothing to explain. A key with no template anywhere is
almost always a typo, and a paragraph about overlay semantics in front of a typo
is an obstacle. Both messages end with the same second line — `Declare` the key
`in` the primary — which is *name what fixes it*, and both name the primary by
the `PathBuf` `load` stored, so the sentence works long after the borrowed
argument that named it is gone.

One line in the overlay branch is unreachable, and it is worth saying so rather
than passing over it. Line 373 defaults the overlay's name to the string `the
overlay` when `self.overlay` is `None`. That state cannot occur: `overlay_only`
is only ever inserted into inside `Catalog::resolve`'s overlay block, and
the `overlay` field is `Some` exactly when that block ran, so a key in
`overlay_only` implies a path to print. The code keeps the branch because the two
fields are independent as far as the type system is concerned, and the cost of
keeping it is one line that produces a sentence still true if it ever ran. It is
the same instinct as chapter 4's clamp on a source offset: the crate declines to
prove a thing the types do not say, and arranges for the unproved case to be
harmless.

Line 388 closes the `impl` block. Chapter 3's convenience-loader fragment opens it, and
between the two are the two halves this chapter and that one own.

<a id="one-window"></a>
## One window, and nothing more

Ten lines at the very end of `src/templates.rs`, after every free function in the
file, are a second `impl Templates`. The function in it exists for one caller in
another module, and its placement says so: it is not part of the block a reader
of the type's public surface walks, and it was added where it could be read
against its purpose rather than against its neighbours.

<!-- fragment «templates-keys» owner="whole-word-or-nothing" source="crates/keyed-launch/src/templates.rs" lines="976-985" parent="source-templates" -->
````rust

/// The keys the primary document declares, in name order. The conformance kit's
/// one window into a loaded configuration — enough to say *this checked
/// nothing*, and nothing more.
impl Templates {
    #[must_use]
    pub fn keys(&self) -> Vec<&str> {
        self.templates.keys().map(String::as_str).collect()
    }
}
````
<!-- /fragment -->

The comment sits on the `impl` block rather than on the method, which is where a
reader will notice it is describing exactly one function; there is nothing else
in the block for it to be about. It is also the whole design. `keys` returns the
keys that resolve, borrowed from the map, and the map is a `BTreeMap`, so *in
name order* is free rather than
sorted — chapter 2 read the field and named this as what its ordering buys. The
qualifier *the primary document declares* is exact: an overlay-only key is not in
the map, so it is not in this list, which keeps `keys` consistent with `source`
and `require` rather than offering a third answer to *which keys are there*.

*Enough to say `this checked nothing`, and nothing more* is the sentence that
bounds it. The conformance kit — chapter 9's — is handed a consumer's
configuration file and must report whether it conforms. To do that it needs to
know a document declared at least one key, because a kit that only reports
violations reads identically when it is given nothing to check, and an empty
configuration would otherwise pass every check by having nothing to fail.
`crates/keyed-launch/src/conformance.rs` uses the result twice: it fails a
document whose key list is empty, and it iterates the list to expand each key
with placeholder values. That is the entire window, and `an_unused_key_is_not_an_error`
fixes its other edge — a key the crate has never heard of is not a
diagnostic, because there is no key set to be absent from. What a key means is
the consumer's; one nobody asks for costs nothing.

What the window deliberately does not offer is a template. There is no accessor
for the words of a key's compiled template, no way to ask how many slots it
mentions, and no way to render it back to text. A kit that could read a template
could assert things about *which program* a key runs, and that is a judgement
about meaning — the one thing neither this crate nor its kit is allowed to make.

<a id="no-constructor"></a>
## The type no caller can construct

`src/argv.rs` is forty-eight lines and two types, and it is where the crate's two
halves meet. Chapter 1 stated that as a claim about the design and pointed here;
this section is the two lines that make it a fact about the types instead — the
`pub(crate)` on the constructor, and the single call to it. The whole file is one
ownership block, and it is the last of the configuration half.

<!-- fragment «argv» owner="whole-word-or-nothing" source="crates/keyed-launch/src/argv.rs" lines="1-48" parent="source-argv" -->
<!-- insert «argv-slot» -->
<!-- insert «argv-authored-only-by-expand» -->
<!-- insert «argv-no-public-constructor» -->
<!-- insert «argv-program-and-args» -->
<!-- insert «argv-words» -->
<!-- /fragment -->

`Slot` is the caller's half of an expansion call: one name, one value, both
borrowed for the duration of the call. It is the type the four-element array in
this chapter's example is made of, and its two fields are public because a caller
constructs them directly — there is nothing to validate at construction, since
whether the name is declared is `match_values`'s question and it cannot be
answered without the configuration.

<!-- fragment «argv-slot» owner="whole-word-or-nothing" source="crates/keyed-launch/src/argv.rs" lines="1-10" parent="argv" -->
````rust
use std::ffi::{OsStr, OsString};

/// A value for one declared slot, at expansion.
///
/// Substitution is whole-word: the crate never learns what a name means, and
/// never rewrites part of a word.
pub struct Slot<'a> {
    pub name: &'a str,
    pub value: &'a OsStr,
}
````
<!-- /fragment -->

The value is an `&OsStr` and not a `&str`, which is the one choice in this file
that needs a reason. An argument to a process is a byte string on Unix and need
not be UTF-8; a path very often is not, and `worktree` and `repo` are paths. A
crate that took `&str` here would refuse to launch on a filename it had no
business having an opinion about — and having an opinion about the contents of a
value is exactly what this stage must not do. The lifetime `'a` is shared by both
fields and outlives only the call, so the caller keeps ownership of everything it
offers and `match_values` returns borrows of the same bytes.

The comment above the struct is the crate's spine stated for the value rather
than for the template: substitution is whole-word, the crate never learns what a
name means, and it never rewrites part of a word. Chapter 4 enforced the first
clause on the template. This chapter's expansion loop is the second and third.

`Argv` is what expansion produces and what the launch half consumes. Its two
fields are private, and its doc comment states the property that makes the whole
book's third arm checkable before any of it is spawned.

<!-- fragment «argv-authored-only-by-expand» owner="whole-word-or-nothing" source="crates/keyed-launch/src/argv.rs" lines="11-22" parent="argv" -->
````rust

/// A program and its arguments, in order, ready to spawn.
///
/// Built only by [`Templates::expand`](crate::Templates::expand), so nothing
/// reaches a spawn that a template did not author. There is no constructor and
/// no shell: the words are the words the file holds, with each whole-word slot
/// replaced by the value offered for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Argv {
    program: OsString,
    args: Vec<OsString>,
}
````
<!-- /fragment -->

*Built only by `Templates::expand`, so nothing reaches a spawn that a template did
not author.* That is the sentence, and the next fragment is what enforces it.

<!-- fragment «argv-no-public-constructor» owner="whole-word-or-nothing" source="crates/keyed-launch/src/argv.rs" lines="23-27" parent="argv" -->
````rust

impl Argv {
    pub(crate) fn new(program: OsString, args: Vec<OsString>) -> Self {
        Self { program, args }
    }
````
<!-- /fragment -->

Five lines, and the load-bearing token in them is `pub(crate)`. `argv` is a
private module — chapter 1 read the module list — so the only things visible
outside the crate are what `pub use argv::{Argv, Slot}` re-exports: the two
types, and none of the constructor. Inside the crate, `Argv::new` has exactly one
caller. It is line 375 of `src/templates.rs`, the last line of `expand`, and a
search of the whole workspace finds no other. The figure below is that count
stated as what it proves.

```text
Argv::new                       pub(crate), crates/keyed-launch/src/argv.rs:25
  called from                   crates/keyed-launch/src/templates.rs:198  (Templates::expand)
  called from                   — nothing else, in this crate or any other
```

So the claim *nothing reaches a spawn that a template did not author* is not a
convention anyone has to maintain. A consumer cannot build an `Argv` from words
it chose, cannot build one from a string it read somewhere, and cannot build one
by any route that does not pass through a template that was validated at load —
because there is no function that would let it, and the compiler is what says so.
`run` in chapter 7 takes an `Argv` and nothing else that could name a program,
which is the other end of the same seam: the launch half has no way to be handed
words the configuration half did not produce.

It is one boundary, proved once. Chapter 1 stated it and this section proves it,
so no chapter in between has to carry it; the closing chapter revisits it only as
one of the nine answers to the book's own test.

The two accessors are the launch half's whole view of the value.

<!-- fragment «argv-program-and-args» owner="whole-word-or-nothing" source="crates/keyed-launch/src/argv.rs" lines="28-37" parent="argv" -->
````rust

    #[must_use]
    pub fn program(&self) -> &OsStr {
        &self.program
    }

    #[must_use]
    pub fn args(&self) -> &[OsString] {
        &self.args
    }
````
<!-- /fragment -->

`program` and `args` are separated because that is the shape a spawn wants:
`Command::new` takes the program and `args` takes the rest, and word zero is
special to the operating system rather than to this crate. The separation happens
in `expand`, at the four lines above — and what load contributes is not the split
but the *guarantee* that there is a word zero to split at, which is chapter 4's
word-zero rule. So nothing on this side of the seam has to decide which word is
the program. `program_and_arguments_split_at_word_zero` pins the pair: expanding
`wrapper --flag ${prompt}` gives `program()` of `wrapper` and `args()` of two
elements.

Both return borrows and both are `#[must_use]`, which is the same attribute
`source` and `keys` carry and is absent from `expand` and `require` — the two
that return a `Result`, where the compiler already refuses a discarded value.
Twelve functions in the crate carry the attribute. Eleven are queries whose
returned value is the only reason to call them; the twelfth, `take_interrupt` in
`src/run.rs`, is not — it *clears* a latch as it reads it, and chapter 8 is where
that difference matters.

The last method is the same value in the other shape.

<!-- fragment «argv-words» owner="whole-word-or-nothing" source="crates/keyed-launch/src/argv.rs" lines="38-48" parent="argv" -->
````rust

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
````
<!-- /fragment -->

`words` is a program and its arguments as one list, program first, and it
allocates: it clones every `OsString` rather than borrowing, because there is no
contiguous slice in the struct to borrow — the program is a field and the
arguments are a `Vec`, and joining them requires somewhere to put them. The
comment names its callers by shape rather than by name: a consumer building a
`Command` wants the whole launch as one list, and so does a diagnostic that has
to print what was about to run.

No production code in this workspace calls it. Every current caller is a test:
`crates/grove-loop/tests/session_config.rs` asserts an expanded launch word by
word in seven places, and this crate's own `crates/keyed-launch/tests/templates.rs`
routes four of its expansion tests through a `words()` helper — including the two
this chapter cited above. The code that actually spawns takes the opposite route
on purpose. `crates/grove-loop/src/loop_driver.rs` passes the `Argv` itself into
`keyed_launch::run`, and the comment on the call that produced it explains why
the `Argv` remains unflattened. `Argv` has no constructor, so handing it through
makes *nothing reaches a spawn that a template did not author* a compiler-checked
property. `words` is the shape for inspecting a launch, and the `Argv` itself is
the shape for performing one. Once a caller flattens an `Argv` to a
`Vec<OsString>`, it has a word list it could have built by any other means, and
nothing downstream can distinguish its provenance.

Every path from a human's file to a process has now been read. A key was declared
in one file, a template was split into words once and checked whole, four values
were offered against a vocabulary the consumer declared, and four words came out
— none of which the crate has an opinion about. What it does not yet have is
anywhere to send them. The next three chapters are the launch half: chapter 6
allocates the path whose *appearance* will be the only thing that ends the
launch, and chapters 7 and 8 spawn the child and watch for that appearance. The
`Argv` this chapter built is the only thing that crosses between them.

[Previous: What a template must be](04-template-law.md) | [Contents](README.md) | [Next: Appearance is the event](06-the-channel.md)

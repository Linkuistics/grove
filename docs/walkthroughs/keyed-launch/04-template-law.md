# What a template must be
<!-- book-page id="template-law" slice="words-not-shell" order="4" -->
[Previous: Two documents, neither one assembled](03-two-documents.md) | [Contents](README.md) | [Next: From a template to an argv](05-to-an-argv.md)

<a id="words-not-shell"></a>
## Words, not shell

Chapter 3 read `load` and the three passes that check a document whole, and it
named two functions it did not read. `validate_node` and `validate_template` are
those two, and they are where every rule a template must satisfy actually binds.
This chapter follows those functions, the scanner one rule needs, and the five
helpers that turn a finding into a sentence an operator can act on.

What this stage must not add and must not interpret is a **shell**. A template is
one string a human wrote, and a word of it is a word: the line is split by POSIX
quoting rules and the resulting words are handed to the operating system in that
order. No variable is expanded, no `$(…)` is run, no glob is matched, no
redirection or pipeline is honoured, no `~` is a home directory, and no alias or
shell function is reachable. The crate's own `${name}` is not an exception, for
two reasons this chapter reads the lines of: it is resolved against a slot table
the consumer declared rather than against anything ambient, and it may only ever
stand for one whole word, so it can never change where one argument ends and the
next begins.

That is where the book's second arm begins. A layer learns what a value means
**on the way through** by re-reading a value it has already read, and the cost is
that a value with a space becomes two arguments, a `#` truncates the line, and a
`$(…)` becomes a command. Two-thirds of that arm is chapter 5's, where a
substituted value is protected after the fact; the third is this chapter's, and
it is the `#`. This is the one place where declining to interpret is not enough
on its own. The splitter this crate
depends on *does* interpret a `#`, silently, and dropping the rest of the line is
a legal thing for it to do. So the crate scans for the character itself, before
the split, and refuses the line rather than launching an argv that means less
than the operator wrote.

<a id="the-rules"></a>
## Every rule, and the line that binds it

The rules are not written down in `src/templates.rs`. The file is 13% comment,
and the three comments standing over this chapter's 246 lines explain two
mechanisms and one output policy rather than the law they serve. The law itself
is spread across four functions and two hundred lines, one `push` at a time. The
table is that law collected: each rule with the line that enforces it, the exact
text an operator sees when it is broken, and the test that pins it. Read the rest
of the chapter against it.

| Rule | Enforced at | The diagnostic | Pinned by |
|---|---|---|---|
| a node has no properties and no child block | 548 | `` properties and child blocks are not allowed `` | `schema_and_template_failures_are_aggregated_with_source_locations`, `a_child_block_is_refused_like_a_property` |
| neither the node nor an entry carries a type annotation | 554 | `` type annotations are not allowed `` | `a_type_annotation_is_refused_on_the_node_and_on_its_argument` |
| a node has exactly one positional argument | 566 | `` a key must have exactly one positional argument `` | `a_key_needs_exactly_one_positional_argument` |
| that argument is a string | 574 | `` a key's sole argument must be a string `` | `a_keys_sole_argument_must_be_a_string` |
| no key is declared twice in one document | 502–515, chapter 3's | `` duplicate key `one`; declarations at … `` naming every declaration | `a_duplicate_key_reports_every_declaration_location` |
| no unquoted `#` begins a word | 603 | `` `#` starts a comment in a command template; quote it to pass it literally `` | `an_unquoted_hash_is_refused_rather_than_truncating_the_argv`, `quoted_and_midword_hashes_stay_literal` |
| every quote closes | 612 | `` command template has unmatched quotes `` | `unmatched_quotes_are_refused` |
| word zero is a literal, and not empty | 624 and 636 | `` word zero must be a literal executable ``, or `` word zero must be a literal non-empty executable `` | `word_zero_must_be_a_literal_executable` |
| a substitution occupies a complete word | 729 | `` substitutions must occupy a complete shell word, got `pre${prompt}` `` | `schema_and_template_failures_are_aggregated_with_source_locations` |
| a substitution names a declared slot | 719 | `` unknown substitution `${unknown}` `` | `schema_and_template_failures_are_aggregated_with_source_locations` |
| a required slot appears exactly once | 650 | `` command template must contain `${prompt}` exactly once `` | `schema_and_template_failures_are_aggregated_with_source_locations` |
| an optional slot appears at most once | 650 | `` `${label}` may appear at most once `` | `schema_and_template_failures_are_aggregated_with_source_locations` |

Every row carries a test, and three of them did not when this chapter was first
written. The four node-shape rules at the top of the table were the gap: the
aggregate test loads a five-node document in which every node is faulty in a
different way, and the property arm of the first rule was the only one of the
four it reached. The type-annotation, argument-count and argument-type rules were
exercised nowhere in `crates/keyed-launch/tests/`, and two of the three nowhere
in the workspace at all. Nothing about them was ever wrong — each is four lines
of `if` and each produces a message in the same shape as its neighbours — but
*enforced* and *proved* are different words, and this book uses the second one
only where it is earned. `template-rule-tests-k118` earned it: four tests in the
same `load_error` + `assert_contains` idiom as the rest of the file, one per
node-shape rule, each asserting the exact text of the row it stands under. They
touch no root, so closing the gap was not a corpus change and waited on no book.

Two of the four are worth reading for what they say about the rules rather than
about the tests. The count rule is `positional.len() != 1`, not a
missing-argument check, so a second template on the line is refused as firmly as
none, and the test asserts both directions. And the argument-type rule is
reachable only through KDL values this crate never mentions — `one 42`,
`one true`, `one null` all parse, and all take the `as_string() == None` arm —
so the test asserts all three rather than assume they travel together.

Two records are what these rules keep, and this chapter is where their lines are.
*Complete session configuration* settles that the whole of a document is read and
validated **for syntax, duplicates, node shape and every template rule** before
anything is spawned, and — in the option it rejects — that templates are not
executed through a shell, because shell evaluation would turn quoting,
environment expansion, pipelines and redirection into a second configuration
language and obscure the direct foreground child the runner must supervise. Every
row above from the `#` down is the price of that rejection: a crate that ran a
shell would need none of them, and would have handed the operator a language it
could not validate. `docs/specs/module-decomposition.md`'s decision 7 states four
of the rows in one sentence — *that a substitution is a whole word and not
embedded in one, that it names a declared slot, that a required slot appears
exactly once, that an optional one appears at most once* — and it states them to
argue something else: that none of the four is checkable by a loader that will
not learn the slot names until expansion. Chapter 2 read that argument at the
signature. The closing loop of `validate_template` is the second of the two lines
that keep it, and this chapter reads it.

One obligation reaches this chapter from chapter 1 and is discharged nowhere
else. `ConfigError`'s doc comment states what every message it carries must
satisfy — name what is wrong, name **where**, and name what fixes it — and
chapter 1 could only quote it, because the type is opaque and holds a `String`.
The last section of this chapter is the five functions that build that string.

<a id="the-rules-on-one-line"></a>
## The rules on one line

The example this book carries reaches the rules here. Chapter 1 put two lines in
the operator's personal `~/.config/grove/config.kdl`, chapter 2 fixed the
four-slot vocabulary they are written against, and chapter 3 read them both out
of their files. This section is one of those lines meeting all twelve rows of the
table at once, and then the same line with one character added.

```text
impl "claude --model opus ${prompt}"
```

The node is one KDL node named `impl` with one positional entry, no properties,
no children and no type annotation, so the four node-shape rules pass and the
entry's value is a string. The string holds no `#` at a word boundary and its
quotes close, so it splits into four words. Word zero is `claude`, a non-empty
literal. Three words are literals and one is nothing but `${prompt}`, whose name
is at position 0 of the slot table, so it compiles to a slot reference rather
than to text. The compiled value is what a caller never sees and every later
chapter assumes:

```text
[Word::Literal("claude"), Word::Literal("--model"), Word::Literal("opus"), Word::Slot(0)]
```

The counts that ride alongside it are `[1, 0, 0, 0]` — one for `prompt` and none
for `session_name`, `worktree` or `repo`. `prompt` is `ExactlyOnce` and admits 1;
the other three are `AtMostOnce` and admit 0. The template is valid, and the
`Vec<Word>` above is what `validate_node` hands back for `load` to store.

Now add one character. The operator wants a comment on the line, or wants to pass
a literal `#`, and writes:

```text
impl "claude --model opus # ${prompt}"
```

A shell would drop everything from the `#` onward. So would the splitter, and
what it would return is `["claude", "--model", "opus"]` — three words that parse,
that name a real executable, and that are missing the one thing the whole launch
exists to deliver. The template would then fail the `${prompt}` cardinality rule
and the operator would be told their prompt was missing, which is true and
useless: the prompt is right there on the line they are looking at. The crate
refuses first, and names the actual cause and its position:

```console
invalid configuration at ~/.config/grove/config.kdl:
  - ~/.config/grove/config.kdl:1:1: key `impl`: `#` starts a comment in a command template; quote it to pass it literally
```

That is `an_unquoted_hash_is_refused_rather_than_truncating_the_argv`, and its
companion `quoted_and_midword_hashes_stay_literal` fixes the other half of the
rule: `'#tag'` and `mid#word` are not comment starts and reach the child
unchanged. The rest of this chapter is the lines that produce both answers.

<a id="the-node-shape"></a>
## What a node must be

`validate_node` is the first of the chapter's three blocks and the only one that
sees a `KdlNode`. Everything below it works on a `&str`. The split matters
because it is where the crate stops depending on its parser: one function asks
KDL what shape the node has, and from there on the template is text.

<!-- fragment «node-and-template-rules» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="541-660" parent="source-templates" -->
<!-- insert «validate-node-shape» -->
<!-- insert «validate-node-one-argument» -->
<!-- insert «validate-node-result» -->
<!-- insert «validate-template-signature» -->
<!-- insert «validate-template-comment-start» -->
<!-- insert «validate-template-split» -->
<!-- insert «validate-template-word-zero» -->
<!-- insert «validate-template-cardinality» -->
<!-- /fragment -->

The first fragment takes the node's identity and its two shape rules.

<!-- fragment «validate-node-shape» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="541-559" parent="node-and-template-rules" -->
````rust

fn validate_node(source: &str, node: &KdlNode, slots: &[SlotSpec]) -> NodeValidation {
    let key = node.name().value().to_owned();
    let location = source_location(source, node.span().offset());
    let mut diagnostics = Vec::new();
    let has_property = node.entries().iter().any(|entry| entry.name().is_some());

    if has_property || node.children().is_some() {
        diagnostics.push(at_node(
            location,
            "properties and child blocks are not allowed".to_owned(),
        ));
    }
    if node.ty().is_some() || node.entries().iter().any(|entry| entry.ty().is_some()) {
        diagnostics.push(at_node(
            location,
            "type annotations are not allowed".to_owned(),
        ));
    }
````
<!-- /fragment -->

Three facts are collected before any rule runs, and each is needed by something
other than this function. The key is cloned out of the node name because
`validate_document` will index duplicates by it, and because a node that fails
every rule below must still surrender a key — chapter 2 read that in
`NodeValidation`'s four fields, and this is the line that makes it true. The
location is computed once, from the node's byte offset, by a function at the very
bottom of this chapter; every diagnostic this node produces will carry it, and so
will the duplicate report chapter 3 read. The `diagnostics` vector is local, and
is the reason a node with four problems produces four findings.

Then the two shape rules, and they are worth reading as one decision rather than
two. KDL is a general document language: a node may carry named properties, a
child block in braces, and a type annotation in parentheses on itself or on any
entry. This crate wants none of them, and says so rather than ignoring them. The
cost of ignoring would fall entirely on the operator, who would write something
with an obvious meaning — a `model=opus` property, a child block grouping two
kinds — and get a launch that silently did not have it. The `has_property`
binding on line 546 is the entry-level half of the first rule and line 548 is the
node-level half; the two are combined into one condition, and so into one
finding, because an operator who wrote both has made one mistake about what a
node may carry.

Both push through `at_node`, which is the constructor that does *not* prefix the
key. A node-shape problem is reported at its location bare, because the key is
what the offending line *is*: the line and column already point at it, and
`` key `impl`: properties and child blocks are not allowed `` would name the key
twice and locate it once.

<a id="one-argument"></a>
## Exactly one argument, and it is a string

A node that passes both shape rules still has to carry exactly one thing, and
that thing has to be text. Those are the last two rules `validate_node` owns, and
the second of them is the point at which the node stops being a `KdlNode` and
becomes a `&str` for the seven template rules to work on.

<!-- fragment «validate-node-one-argument» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="560-586" parent="node-and-template-rules" -->
````rust

    let positional = node
        .entries()
        .iter()
        .filter(|entry| entry.name().is_none())
        .collect::<Vec<_>>();
    if positional.len() != 1 {
        diagnostics.push(at_node(
            location,
            "a key must have exactly one positional argument".to_owned(),
        ));
    }

    let template = if positional.len() == 1 {
        match positional[0].value().as_string() {
            Some(template) => validate_template(&key, location, template, slots, &mut diagnostics),
            None => {
                diagnostics.push(at_node(
                    location,
                    "a key's sole argument must be a string".to_owned(),
                ));
                None
            }
        }
    } else {
        None
    };
````
<!-- /fragment -->

The positional entries are the entries with no name, which is the same predicate
line 546 used with the sense reversed — a KDL entry is a property when it has a
name and an argument when it does not. Exactly one is required, and both failure
directions are one message: none, and the key names no program at all; two or
more, and there is no rule for deciding which is the template, so inventing one
would be the crate assembling a launch out of more than one thing on the line.
That is the same refusal chapter 3 made across two files, one scale down.

The delegation on line 575 is the only call site of `validate_template`, and the
shape around it is deliberate. `as_string` is `None` when the sole argument is a
number, a boolean or a null — `impl 42` parses perfectly well as KDL — and that
gets its own message rather than being folded into the count rule, because the
two are different mistakes with different fixes. When the count rule fails the
template is `None` and `validate_template` is never called, so a node with no
argument, or with two, produces its own diagnostic and no cascade of template
findings about a string that was never there. The two rules in the previous
section do **not** gate it: a node carrying a stray property alongside a perfectly
good template produces exactly one finding, and the template is compiled anyway.
That is the aggregate test's first node.

Note what the `else` arm does *not* do: it does not return early. The vector
built above it survives, and so does the key.

<!-- fragment «validate-node-result» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="587-594" parent="node-and-template-rules" -->
````rust

    NodeValidation {
        key,
        location,
        template,
        diagnostics,
    }
}
````
<!-- /fragment -->

`NodeValidation` is the whole return, and it is four fields rather than a
`Result` because every one of them is still wanted when the node failed. Chapter
2 argued the shape at the struct; this is where it is filled. Chapter 3 read what
it buys one level up — a `validate_document` returning `Result<Template, _>` per
node could report the first problem in each node and could not report a duplicate
at all, because a node that failed for some other reason would have left no key
behind to compare.

<a id="the-comment-start"></a>
## The `#` that would truncate the line

`validate_template` enforces five of the twelve rules itself and delegates two
more, and it is the rest of this block. It takes the string `validate_node`
extracted and returns the compiled words, and it opens with a signature that
fixes what a rule check is allowed to do with what it finds.

<!-- fragment «validate-template-signature» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="595-602" parent="node-and-template-rules" -->
````rust

fn validate_template(
    key: &str,
    location: SourceLocation,
    template: &str,
    slots: &[SlotSpec],
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> Option<Vec<Word>> {
````
<!-- /fragment -->

The signature is the one place the two halves of the validation meet. The key is
borrowed for message prefixes only; the location is the node's, already computed,
so every template diagnostic points at the node rather than at some offset within
the string; the slot table is the vocabulary chapter 2 argued belongs at `load`;
and `diagnostics` is `&mut`, which is what lets this function contribute findings
to the node's list and still return an `Option<Vec<Word>>` describing whether a
template came out. `None` here does not mean *no diagnostic* — it means *no
compiled template*, and the diagnostics have already been pushed.

<!-- fragment «validate-template-comment-start» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="603-610" parent="node-and-template-rules" -->
````rust
    if contains_shell_comment_start(template) {
        diagnostics.push(at_template(
            location,
            key,
            "`#` starts a comment in a command template; quote it to pass it literally".to_owned(),
        ));
        return None;
    }
````
<!-- /fragment -->

This is the first rule and the only one that had to be invented. Every other rule
in the chapter refuses something the crate could see; this one refuses something
the crate's own dependency would quietly do on its behalf.

The `return None` is the point. Every other rule in the table aggregates — a
template with four faults yields four findings — and this one stops. So does the
next, and they are the only two that do. The reason is that both of them make
every later rule meaningless rather than merely unmet: after a comment start, the
words the splitter would return are not
the words the operator wrote, so counting `${prompt}` in them, or checking word
zero, would be checking a line nobody authored. Reporting *the prompt is missing*
alongside *there is a comment here* would be worse than reporting nothing,
because the first sentence is a consequence of the second and reads as a second,
independent problem. Chapter 3's aggregation is a property of the document and of
the node; inside one template, two of the twelve rules are gates and the rest are
a list.

The message names the fix, which is the third of `ConfigError`'s three
obligations and the hardest to satisfy in the abstract. Here it is concrete: the
operator meant one of two things and both are quoting.

<a id="unmatched-quotes"></a>
## The split, and the quotes that do not close

With a comment start ruled out, the line can be split into words. This is the
crate's only call into `shell-words`, and the only way that call can fail.

<!-- fragment «validate-template-split» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="611-622" parent="node-and-template-rules" -->
````rust

    let words = match shell_words::split(template) {
        Ok(words) => words,
        Err(_) => {
            diagnostics.push(at_template(
                location,
                key,
                "command template has unmatched quotes".to_owned(),
            ));
            return None;
        }
    };
````
<!-- /fragment -->

`shell_words::split` is the whole of the crate's use of that dependency, and
`Err` from it has exactly one cause: a quote that never closes. The error value
is discarded — `Err(_)` — because the crate's message is better than the
library's for this reader. A library error would describe a parse state; the
operator needs to be told which of their two files, which line, which key, and
what to look for, and `at_template` and the location the caller passed in supply
the first three.

This is the second gate, and it stops for the same reason as the first: there are
no words to check.

What the `Ok` arm produces is a `Vec<String>` in which quoting has already been
resolved. `'#tag'` has become `#tag`, `"a b"` has become one word `a b`, and
`a\ b` has become `a b`. That is the whole of the familiar shell surface the
operator gets, and it is also all of it: the words that come back are inert text,
and nothing below re-reads them for meaning.

<a id="word-zero"></a>
## Word zero, and the loop that compiles the rest

Both rules above could refuse the template outright. Every rule below
accumulates, and the first of them is about the first word.

<!-- fragment «validate-template-word-zero» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="623-647" parent="node-and-template-rules" -->
````rust

    if words.is_empty() {
        diagnostics.push(at_template(
            location,
            key,
            "word zero must be a literal non-empty executable".to_owned(),
        ));
    }

    let mut compiled = Vec::with_capacity(words.len());
    let mut counts = vec![0usize; slots.len()];
    for (index, word) in words.into_iter().enumerate() {
        let parsed = parse_template_word(key, &word, location, slots, diagnostics);
        if index == 0 && !matches!(parsed, Word::Literal(ref value) if !value.is_empty()) {
            diagnostics.push(at_template(
                location,
                key,
                "word zero must be a literal executable".to_owned(),
            ));
        }
        if let Word::Slot(slot) = parsed {
            counts[slot] += 1;
        }
        compiled.push(parsed);
    }
````
<!-- /fragment -->

Word zero is one rule with two lines, because the empty case cannot be expressed
by the loop that checks the other. An empty template splits to no words at all,
so the loop never runs and line 636 never fires; line 624 catches it and says
*non-empty*, which is the more useful of the two words for an operator who typed
`impl ""`. That template also fails its `${prompt}` requirement, and both
findings come out together, because neither line returns.

The loop is where a template becomes a `Vec<Word>`. Three things happen per word
and they are worth separating. `parse_template_word` — two sections below —
decides what the word *is*. Line 636 applies the word-zero rule to the first
result: word zero must be a `Word::Literal` and must not be empty, so a
`${prompt}` in first position fails even though it is a perfectly good
substitution anywhere else. That is the rule that makes the executable name a
fixed, readable fact about the configuration rather than something a caller
supplies at launch, and it is why *what program does this key run* is answerable
by reading the file.

Then the counting, and this is the through-line chapter 2 set up. `counts` is a
`Vec<usize>` as long as the slot table and indexed **by position**, not by name,
because `Word::Slot` is an index rather than a name — chapter 2 argued that at
the type, on the ground that the table is compiled once per load and a compiled
word can then refer to a slot by where it sits. The consequence lands here: the
cardinality check is a `zip` of two vectors rather than a map lookup per word,
and no slot name is compared during the count at all. The names were compared
once, at `parse_template_word`, and never again — in this crate or in chapter 5's
expansion.

Note the ordering of the last two statements. `Word` is not `Copy`, because
`Word::Literal` owns a `String`, so `compiled.push(parsed)` moves it. The
`if let` above reads `parsed` first and moves nothing, because the only field it
binds is a `usize`.
Swap the two lines and the function stops compiling. That is a small consequence
of the index representation, and it is why the count is taken before the word is
stored rather than after.

<a id="the-counts"></a>
## What the counts are for

The counts the loop collected are spent in the last nine lines of the function,
against the vocabulary that produced their positions.

<!-- fragment «validate-template-cardinality» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="648-660" parent="node-and-template-rules" -->
````rust

    for (slot, count) in slots.iter().zip(counts) {
        if !slot.requirement.admits(count) {
            diagnostics.push(at_template(
                location,
                key,
                slot.requirement.violation(&slot.name),
            ));
        }
    }

    Some(compiled)
}
````
<!-- /fragment -->

This is the loop chapter 2 named as the second of the two lines that keep
decision 7 of `docs/specs/module-decomposition.md`. It is nine lines and it is
the entire reason the vocabulary is a parameter of `load`: a loader that did not
hold the slot table could not run it, and *a required slot appears exactly once*
would become a fact discovered at the moment a launch was attempted rather than
at the moment the file was read.

Two things about it are worth stating. Neither the test nor the message is
written here — `admits` and `violation` are `Requirement`'s, and chapter 2 read
both — so adding a third cardinality would change one enum and nothing in this
file. And the loop runs over the **vocabulary**, not over the template's words,
which is what makes a *missing* required slot detectable at all: a rule about
something that is not present cannot be checked by walking what is.

The `Some(compiled)` on line 659 is returned whether or not diagnostics were
pushed in this function, and that is not an oversight. The caller does not consult
it in the failing case — `validate_document` refuses the whole document if any
diagnostic exists — so the compiled words of an invalid template are built and
then dropped. Returning `None` on any diagnostic would express the same outcome
and add a third place where this function decides not to produce a template;
returning the words costs nothing and keeps the two gates above as the only two.

<a id="the-scan"></a>
## Why the scan cannot be a `contains`

The chapter's second block is one enum, the scan that walks it, and the two
functions that decide what a single word is.

<!-- fragment «word-scanning» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="661-743" parent="source-templates" -->
<!-- insert «shell-word-scan-state» -->
<!-- insert «contains-shell-comment-start» -->
<!-- insert «parse-template-word» -->
<!-- insert «whole-substitution» -->
<!-- /fragment -->

The enum comes first, because it is the argument. Nothing else in the crate
mentions it; it exists so that one function's positions can be named, and the
names are the reason that function is readable as a rule rather than as a table.

<!-- fragment «shell-word-scan-state» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="661-671" parent="word-scanning" -->
````rust

#[derive(Clone, Copy)]
enum ShellWordScanState {
    Delimiter,
    DelimiterBackslash,
    Unquoted,
    UnquotedBackslash,
    SingleQuoted,
    DoubleQuoted,
    DoubleQuotedBackslash,
}
````
<!-- /fragment -->

Seven states, and every one of them is a position in which a `#` means something
different. That is the whole justification for the type, and it is why the rule
above cannot be `template.contains('#')`. The table below is that justification
as a partition: exactly one row of the seven is the rule, and the other six are
the reasons that finding it needs a state machine.

| State | What a `#` read here is | Written |
|---|---|---|
| `Delimiter` | a comment start, and the only one — the scan returns `true` | `runner # x` |
| `DelimiterBackslash` | a literal, and the first character of a word | `runner \# x` |
| `Unquoted` | a literal, inside the word being read | `mid#word`, `tag#1` |
| `UnquotedBackslash` | a literal, inside the word being read | `a\#b` |
| `SingleQuoted` | a literal, inside the quotes | `'#tag'` |
| `DoubleQuoted` | a literal, inside the quotes | `"#tag"` |
| `DoubleQuotedBackslash` | a literal, inside the quotes | `"a\#b"` |

A `contains` would refuse all six literal rows. It would also refuse the one
thing the diagnostic tells an operator to do: quote the `#`. Being refused a
second time, with the same message, for having followed it is the failure mode
the enum exists to avoid.

The scan is that enum walked once over the template's characters, and the comment
above it is one of only three in the chapter's 246 lines.

<!-- fragment «contains-shell-comment-start» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="672-709" parent="word-scanning" -->
````rust

/// Does a `#` start a comment anywhere in this template?
///
/// `shell_words::split` treats `#` at a word boundary as a comment and silently
/// drops the rest of the line, so a template that meant to pass a `#` literally
/// would lose every word after it with nothing said. This scan is the same state
/// machine the splitter walks, run only to answer that one question.
fn contains_shell_comment_start(template: &str) -> bool {
    use ShellWordScanState::{
        Delimiter, DelimiterBackslash, DoubleQuoted, DoubleQuotedBackslash, SingleQuoted, Unquoted,
        UnquotedBackslash,
    };

    let mut state = Delimiter;
    for character in template.chars() {
        state = match (state, character) {
            (Delimiter, '#') => return true,
            (Delimiter, '\'') => SingleQuoted,
            (Delimiter, '"') => DoubleQuoted,
            (Delimiter, '\\') => DelimiterBackslash,
            (Delimiter, '\t' | ' ' | '\n') => Delimiter,
            (Delimiter, _) => Unquoted,
            (DelimiterBackslash, '\n') => Delimiter,
            (DelimiterBackslash, _) => Unquoted,
            (Unquoted, '\'') => SingleQuoted,
            (Unquoted, '"') => DoubleQuoted,
            (Unquoted, '\\') => UnquotedBackslash,
            (Unquoted, '\t' | ' ' | '\n') => Delimiter,
            (Unquoted, _) | (UnquotedBackslash, _) => Unquoted,
            (SingleQuoted, '\'') => Unquoted,
            (SingleQuoted, _) => SingleQuoted,
            (DoubleQuoted, '"') => Unquoted,
            (DoubleQuoted, '\\') => DoubleQuotedBackslash,
            (DoubleQuoted, _) | (DoubleQuotedBackslash, _) => DoubleQuoted,
        };
    }
    false
}
````
<!-- /fragment -->

The comment states the *why*; the state machine states the *what*, and reading
the arms in order is reading the rule. The function begins in `Delimiter`, which
is the only state with a `return` in it: a `#` seen there is a comment start and
nothing further need be examined. Every other arm is a transition.

The arms fall into four groups. From `Delimiter`, a quote opens a quoted region,
a backslash arms an escape, whitespace stays at the boundary, and anything else
begins a word. From `Unquoted` — inside a word — the same characters do the same
things except that whitespace *returns* to the boundary, and `#` is not listed,
so it falls to the catch-all and remains part of the word. The two quoted states
are closed by their own quote and by nothing else, which is why a `#` between
quotes is invisible to this scan; a double quote additionally honours a backslash
so that `\"` does not end the region. And the three backslash states each consume
exactly one character: `DelimiterBackslash` on a newline is a line continuation
and returns to the boundary, and on anything else begins a word.

One arm's placement carries meaning that is easy to read past. `(Unquoted, '\t' |
' ' | '\n') => Delimiter` is listed *before* `(Unquoted, _) | (UnquotedBackslash,
_) => Unquoted`, and the whitespace arm names only `Unquoted`. So an escaped
space — state `UnquotedBackslash` — falls through to the catch-all and stays
inside the word, which is correct, and would be wrong if the two arms were
swapped or if the earlier arm had named both states. `a\ b` is one word to the
splitter, and it is one word to this scan.

Note what the closing quote does: `(SingleQuoted, '\'') => Unquoted`, not
`Delimiter`. A quoted region ends inside the word it was part of, so `'a'b` is one
word and a `#` immediately after a closing quote is a literal. The comment says
this scan is *the same state machine the splitter walks*, and that is the claim it
has to keep: the two must agree on every boundary, or the crate would refuse
lines the splitter would have handled or accept lines it would have truncated.
The claim is checkable rather than rhetorical. `shell-words` 1.1.1 carries a
private seven-variant `State` enum, and the seven above are it, in its order,
with one renamed — its `Backslash` is this crate's `DelimiterBackslash`, which
says which backslash it is. The copy exists because the original is private:
there is no way to ask the splitter *would you treat this as a comment* short of
splitting, and splitting is the thing whose result cannot be trusted here.
`quoted_and_midword_hashes_stay_literal` is the test that holds the agreement,
and it holds it end to end — it loads a template carrying both a quoted and a
mid-word `#`, expands it, and compares the four resulting arguments.

<a id="a-whole-word"></a>
## A whole word, or not a substitution at all

`parse_template_word` is what the compile loop calls once per word. It takes one
word of resolved text and returns the `Word` it compiles to, pushing a diagnostic
on the way when the word is a substitution that breaks one of two rules.

<!-- fragment «parse-template-word» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="710-737" parent="word-scanning" -->
````rust

fn parse_template_word(
    key: &str,
    word: &str,
    location: SourceLocation,
    slots: &[SlotSpec],
    diagnostics: &mut Vec<ValidationDiagnostic>,
) -> Word {
    if let Some(name) = whole_substitution(word) {
        if let Some(index) = slots.iter().position(|slot| slot.name == name) {
            return Word::Slot(index);
        }
        diagnostics.push(at_template(
            location,
            key,
            format!("unknown substitution `{word}`"),
        ));
        return Word::Literal(word.to_owned());
    }
    if word.contains("${") {
        diagnostics.push(at_template(
            location,
            key,
            format!("substitutions must occupy a complete shell word, got `{word}`"),
        ));
    }
    Word::Literal(word.to_owned())
}
````
<!-- /fragment -->

The two rules `validate_template` delegates are in these twenty-eight lines and
run in order. The function first checks whether the word is *entirely* a
substitution. If it is and the name is declared, the word is a slot reference
and the function is done. If it is and the name is not declared, the operator is
told which substitution is unknown and the word is kept as a literal, so the
rest of the template is still checked and the reader gets every other finding in
the same report.

The second rule handles a word that is not wholly a substitution but still
contains `${`: it is partially substituted, and that is refused.
`--prompt=${prompt}` is the shape an operator reaches for, and it is exactly what
the crate will not do, because doing it would mean re-reading a word after it had
already been split — building a new word out of a value the crate does not
understand. Once that is allowed, a value with a space in it can change how many
arguments the child receives, and the property chapter 5 proves is gone. The
message quotes the offending word back, which matters here more than elsewhere:
the fault is a fragment of a word, and quoting the whole word is how the operator
finds it on a long line.

Both branches fall through to `Word::Literal`. A diagnostic and a compiled word
are not alternatives in this function, and the reason is the one from the last
section: a template that fails is still compiled, and compiling it is what lets
the remaining rules — word zero, and both cardinalities — report against it.

<!-- fragment «whole-substitution» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="738-743" parent="word-scanning" -->
````rust

/// The slot name in `${name}`, when the word is *nothing but* that substitution.
fn whole_substitution(word: &str) -> Option<&str> {
    let inner = word.strip_prefix("${")?.strip_suffix('}')?;
    (!inner.contains('}')).then_some(inner)
}
````
<!-- /fragment -->

Two statements, and the second is the rule. The prefix and suffix strips are the
obvious part; the guard under them is not, and it is what refuses `${a}${b}`.

Without it, `${a}${b}` would strip to `a}${b`, that string would be looked up in
the slot table, no slot would match, and the operator would be told
`` unknown substitution `${a}${b}` `` — an accusation about a name they did not
write. With it, the function returns `None`, the caller's `contains("${")` test
fires instead, and the message is `substitutions must occupy a complete shell
word`, which is the actual rule they broke. The same line means a slot name may
not contain `}`, which no vocabulary would want and which nothing else enforces.

That is the whole of the substitution surface: `${` at the start, `}` at the end,
no `}` inside, and nothing else in the word. Chapter 5 reads what the index it
produces is worth at expansion.

<a id="what-a-refusal-owes"></a>
## Name what is wrong, name where, name what fixes it

The third block is forty-three lines and no rule at all. It is the machinery every
refusal in both documents is rendered by, and it is where chapter 1's statement
about `ConfigError` — that its obligation is a property of every message it holds
rather than a variant list — stops being a promise.

<!-- fragment «diagnostics» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="744-786" parent="source-templates" -->
<!-- insert «diagnostic-constructors» -->
<!-- insert «render-diagnostics» -->
<!-- insert «location-rendering» -->
<!-- /fragment -->

The two constructors come first, because every diagnostic in the chapter above
was built by one of them.

<!-- fragment «diagnostic-constructors» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="744-754" parent="diagnostics" -->
````rust

fn at_node(location: SourceLocation, message: String) -> ValidationDiagnostic {
    ValidationDiagnostic {
        location: Some(location),
        message,
    }
}

fn at_template(location: SourceLocation, key: &str, message: String) -> ValidationDiagnostic {
    at_node(location, format!("key `{key}`: {message}"))
}
````
<!-- /fragment -->

Two constructors and one difference between them, and the difference is the key
prefix. `at_template` is `at_node` with `` key `<name>`: `` in front, which is why
every message in the table above reads as it does: a node-shape finding is bare
and a template finding is attributed. That is not decoration. A document has many
keys and one of them is wrong; a reader scanning an aggregate report needs the key
before the sentence, because the location tells them which line and the key tells
them which of their launches stops working.

Both constructors fill the location unconditionally, and `at_template` does it by
delegating, so there is one struct literal here rather than two. Chapter 2 read
the only other one in the file: `validate_document`'s duplicate finding, which
takes `locations.first().copied()` from a vector the surrounding code has already
made non-empty, and so absorbs an `Option` the compiler cannot see through. The
`None` is unreachable — and `render_diagnostics`, two fragments below, branches on
it anyway.

<!-- fragment «render-diagnostics» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="755-773" parent="diagnostics" -->
````rust

/// Aggregate, not first-error: one report lists every duplicate with all of its
/// locations, every malformed node, and every invalid template with its key and
/// location, so one edit fixes the file rather than uncovering the next problem.
fn render_diagnostics(
    path: &Path,
    role: DocumentRole,
    diagnostics: Vec<ValidationDiagnostic>,
) -> String {
    let mut rendered = format!("invalid {} at {}:", role.noun(), path.display());
    for diagnostic in diagnostics {
        rendered.push_str("\n  - ");
        if let Some(location) = diagnostic.location {
            let _ = write!(rendered, "{}: ", format_location(path, location));
        }
        rendered.push_str(&diagnostic.message);
    }
    rendered
}
````
<!-- /fragment -->

This is the one function in the chapter whose comment argues rather than
describes, so the prose owes something other than a second copy of it. What the
signature adds is that the policy is **structural**. `diagnostics` arrives by
value as a finished `Vec`, and the function has exactly one call site — line 532,
inside chapter 3's `if !diagnostics.is_empty()` — so there is no incremental
form of it and no way to render a partial report by mistake. Aggregate, not
first-error, is not a convention this function follows; it is the only thing its
shape permits.

The header names the file and the *role* — `configuration` or `configuration
overlay`, which is the only thing `DocumentRole` decides. That is *name where* at
document scale, and it is owed because the reader cannot infer it: chapter 3
established that the rules do not differ by role, so a finding's text is the same
whichever file produced it, and only the header says which one did.

Then each finding on its own line, indented and bulleted, with its location
rendered ahead of the message when it has one — the branch chapter 2 showed is
always taken, kept because the type says it might not be. The path is repeated on
every line rather than printed once. That is deliberate, and it is the cheapest
editor integration there is: `path:line:column` at the start of a line is the
format every terminal, editor and CI annotator already knows how to turn into a
jump, and a reader who copies one line out of a report of nine gets a location
that still works.

<!-- fragment «location-rendering» owner="words-not-shell" source="crates/keyed-launch/src/templates.rs" lines="774-786" parent="diagnostics" -->
````rust

fn format_location(path: &Path, location: SourceLocation) -> String {
    format!("{}:{}:{}", path.display(), location.line, location.column)
}

fn source_location(source: &str, offset: usize) -> SourceLocation {
    let offset = offset.min(source.len());
    let before = &source[..offset];
    let line = before.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let line_start = before.rfind('\n').map_or(0, |index| index + 1);
    let column = source[line_start..offset].chars().count() + 1;
    SourceLocation { line, column }
}
````
<!-- /fragment -->

`format_location` is the format, and it is also chapter 3's: the duplicate report
builds its `declarations at …` list by mapping this function over every location
it collected, which is how one finding names three lines at once.

`source_location` is the last function in the chapter and the one that makes
*name where* true at all. KDL hands back a byte offset; an operator needs a line
and a column. The line is the count of newlines before the offset, plus one. The
column is the distance from the last newline, plus one — and it is counted in
**characters**, not bytes, while the line is counted in bytes. That asymmetry is
correct, and it is why the two counts are taken differently: a newline is one
byte and can be counted as one, but a column is a position a human reads off a
screen, and a line holding an em dash or an accented path would otherwise be
reported further right than the editor puts the cursor, by one column per extra
byte.

The clamp on line 780 is the defensive line. An offset past the end of the source
would panic on the slice below it rather than produce a wrong answer, and
`min(source.len())` costs one comparison to make that unreachable regardless of
what the parser hands over. It is the same instinct as `Err(_)` at the split: the
crate declines to model its dependency's internals, and arranges not to need
to.

That is every rule a template must satisfy, the line that binds each one, and the
five functions that turn a broken one into a sentence naming the file, the line,
the column, the key and the fix. What none of it has done is give any of those
words a meaning. `Word::Slot(0)` is an index into a table of names the consumer
declared, and the crate still does not know what `prompt` is, what `claude` is, or
what any of it is for. Chapter 5 takes the compiled template and a caller's
values and produces the argv, which is where the whole-word rule this chapter
enforced determines the output — and where the two halves of the crate meet, at
the one type no caller can construct.

[Previous: Two documents, neither one assembled](03-two-documents.md) | [Contents](README.md) | [Next: From a template to an argv](05-to-an-argv.md)

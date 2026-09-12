# Reading the tree
<!-- book-page id="reading-the-tree" slice="information-not-error" order="3" -->
[Previous: The grammar and the openings](02-the-grammar.md) | [Contents](README.md) | [Next: Growing the tree](04-growing-the-tree.md)

<a id="information-not-error"></a>
## Information, not error

The four verbs a session runs before it changes anything are `pick`,
`brief-chain`, `kind` and `resolve`, and every one of them can come back with
nothing: a grove whose every leaf is retired has no next leaf, a reference can
name no entry or several. A malformed node-file shape instead refuses the read. The rule
this chapter opens on is what the binary does with that answer — **an absent
answer is information, not an error.** `pick` on a finished grove prints its
diagnostic on stderr and exits zero; `resolve` reports a reference that matches
nothing, or matches more than one entry, the same way; and a reference that
matches a retired or abandoned entry gets its path on stdout *and* a note on
stderr, so a resolved dead end never looks live. The distinction the
chapter rests on is between an answer and a refusal. A refusal is what *The
grammar and the openings* read — a working tree with no grove, a stale session
— and this page adds two more, a malformed reference and a named path that is
not a leaf; each is an error from `main` with exit `1`. An absent answer is a
successful search that matched nothing, and the loop's type for it is read
first.

`Sought` is the type every absent answer arrives in, and it is the store's word
rather than the loop's: `ordinal-fs-tree` declares it, `grove-loop` re-exports
it at its root, and three of the four reading verbs return one —
`Sought<Selection>` from `pick`, `Sought<Kind>` from `kind`,
`Sought<Resolution>` from `resolve`. It has two variants, `Match(T)` and
`Nothing`, and it is `#[must_use]`, so a handler that called a verb and ignored
the answer would draw a compiler warning. What separates it from `Option` is
what its documentation says it is not: `Nothing` is a search that completed and
matched nothing, not a refusal. A refusal is the `Err` the same call can return
— a tree carrying a name grove refuses, a named path that is not a leaf, a
malformed key reference — and every handler on this page propagates that with
`?`, so a refusal exits `1` through `main` while an absent answer is rendered
and exits `0`. That is the chapter's rule as the types state it; the rest of
the page is where each handler renders `Nothing`.

The chapter's premise is the tree these verbs read, stated once in the form the
[task-tree scheme](../../../CONTEXT.md#task-tree-scheme) fixes. A node is a
directory `NN-k<key>/` of numbered children with exactly one `_<slug>.md`
node file; the root holds `_BRIEF.md`. A leaf is a file
`NN-<kind>--<slug>-k<key>.md`, and a retired or abandoned leaf carries `DONE-` or `ABANDONED-` immediately after its position.
The position is per directory and moves under a renumber; the key is permanent
and unique across the whole tree. That is why every spelling of an entry this
chapter reads — `[n]`, a bare `n`, `[n]-slug`, the handle `<slug>-k<key>` — is
a key or ends in one, and why the bare slug is the one spelling that can match
several entries. What each verb does with that tree — the walk, the ascent, the
search — is `grove_loop::verbs`'s, and is named here and explained nowhere in
this book.

The chapter owns three blocks of `cli.rs` and reads them in the order the
argument takes. The four handlers with `leaf_in` and `render_resolution`, lines
511 to 634 are read first, inside and after the worked example, because the
example is `resolve` at full resolution and every other handler is a shorter
form of the same shape; the three helpers at lines 902 to 942 follow, one of
them the path rule that the through-line from *The grammar and the openings*
explains; and the four variants whose doc comments are the verbs' `--help`,
lines 75 to 124 are read last, as the catalogue of promises the handlers have
just been seen to keep — after the example, where a catalogue belongs.

<a id="worked-resolve"></a>
## Worked example: one reference, three renderings, and the chain

The session is the one *Orientation* carries, and its first two verbs are this
chapter's example: `resolve rate-limit-k3`, admitted and dispatched as *The
grammar and the openings* traced it, and `brief-chain` on the path it printed.
The example then runs `resolve` twice more on the tree as the same session
leaves it — after *Growing the tree*'s `leaf-add` has landed
`02-review-impl--rate-limit-k4.md` and *Ending work*'s `leaf-retire` has
renamed the session's own leaf — because the two renderings the rule is named
for need a retired entry and a shared slug, and the carried session produces
both. Every line below is what the built binary printed at the frozen corpus on
a scratch tree of the same shape, without a driver, with the scratch paths
replaced by the carried tree's.

```console
$ cd /work/atlas && grove-llm resolve rate-limit-k3
/work/atlas/.grove/01-impl--rate-limit-k3.md

$ grove-llm brief-chain /work/atlas/.grove/01-impl--rate-limit-k3.md
/work/atlas/.grove/_BRIEF.md

# … the session's leaf-add and leaf-retire have landed …

$ grove-llm resolve rate-limit-k3
/work/atlas/.grove/01-DONE-impl--rate-limit-k3.md
note: referenced task is retired (DONE): /work/atlas/.grove/01-DONE-impl--rate-limit-k3.md

$ grove-llm resolve rate-limit
resolve: reference "rate-limit" is ambiguous; re-query by key:
  [3] rate-limit-k3 /work/atlas/.grove/01-DONE-impl--rate-limit-k3.md (retired)
  [4] rate-limit-k4 /work/atlas/.grove/02-review-impl--rate-limit-k4.md
```

The transcript merges the two streams as a terminal does; the table separates
them, and it is the example as a relation — which stream each line went to and
what the process exited — so the three renderings can be compared line by line.

| Invocation | The tree | stdout | stderr | Exit |
|---|---|---|---|---|
| `resolve rate-limit-k3` | the leaf live | its path | nothing | `0` |
| `brief-chain` on that path | the same | `/work/atlas/.grove/_BRIEF.md` | nothing | `0` |
| `resolve rate-limit-k3` | the leaf retired | its renamed path | the retired note, naming the same path | `0` |
| `resolve rate-limit` | two entries carry the slug | nothing | the ambiguity line, then one keyed line per match, in walk order | `0` |

The handler is `cmd_resolve`, and its order is the first fact on this page: the working tree is resolved, then the reference is read by
`Reference::parse`, and only then is the shared opening taken and the call
made. `Reference` is the loop's type for what a session typed, and this is
where the ledger's row for it is read in full. It carries the text unchanged
and settles one thing only, that there is something to look for: an empty or
blank reference is refused — *an empty reference names nothing*, then the
spellings to give instead — and the fragment's `?` propagates that refusal
before any lock is taken, which is the text-before-lock order *Growing the
tree* states for every writing verb, met here in a reading one. Which form the
text is — a key, a handle, a slug — is the call's to decide, not this
module's. `resolve_refuses_an_empty_reference` in
`crates/grove-llm/tests/resolve.rs` pins the refusal and its wording; the order
is held by the source alone. One clause of that wording is wider than this
verb: the loop's list of spellings ends with *a path under `.grove/`*, which is
a form the grow verbs' `<parent>` and `<target>` arguments accept and `resolve`
does not — measured, `resolve .grove/01-impl--rate-limit-k3.md` answers *no
entry matches* — and this verb's own help, read at the end of the page, names
no path form.

The first thing the handler does with the answer is check for one shape before
rendering. **The root is the one answer with no entry behind it**: `.` is a
reference `Reference::parse` accepts and the call answers with
`Resolution::Root`, but the root has no key, no slug, no kind and no outcome,
so nothing in the resolution carries a path to print. The handler prints the
working-tree root joined with `.grove` — the second of the three places this
module spells that name for a reader, and, as *The grammar and the openings*
said of the first, a spelling for display and not a second source of the root
for any call. The path is therefore the caller's own: `Workspace::resolve`'s
canonicalised root, which is why `resolve_dot_prints_the_grove_root` compares
the answer through `canonicalize` rather than against the fixture's spelling.
The `matches!` is what keeps this branch ahead of rendering, and the rendering
function's own root arm, read below, is what it keeps out.

Everything else is rendered by one pure function and printed as it came back.
`render_resolution` takes the reference as typed — the `&str`, not the parsed
value, because its only use is to be quoted back — and the resolution, and
returns the two strings the verb emits; `print!` and `eprint!` add nothing, not
even a newline, so an empty string is a stream with nothing on it. The `Ok(())`
after them is the whole mechanism of the rule: not-found and ambiguity reach
this line exactly as a match does, so the process exits `0` for all three, and
the comment names the verb this is modelled on — `pick`, whose own absent
answer is read in the next section.

<!-- fragment «handler-resolve» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="559-577" parent="handlers-reading-and-rendering" -->
````rust
fn cmd_resolve(reference: &str) -> Result<()> {
    let worktree = worktree()?;
    let parsed = Reference::parse(reference)?;
    let resolution = verbs::resolve(&readable(&worktree)?, &parsed)?;
    // **The root is the one answer with no entry behind it**, so its path is the
    // one thing rendering cannot supply: it is the caller's own spelling of the
    // tree, not something read out of it.
    if matches!(resolution, Sought::Match(Resolution::Root)) {
        println!("{}", worktree.join(".grove").display());
        return Ok(());
    }
    let (stdout, stderr) = render_resolution(reference, &resolution);
    // `resolve` is pick-style: a not-found / ambiguous reference is reported on
    // stderr and still exits zero (it is information, not an error).
    print!("{stdout}");
    eprint!("{stderr}");
    Ok(())
}

````
<!-- /fragment -->

`render_resolution` is the crate's one pure function, and its doc comment gives
both reasons for its shape. It is `pub` and `#[must_use]` because it is
unit-tested through the library target: `crates/grove-llm/tests/resolve_rendering.rs`
imports `grove_llm::cli::render_resolution` and calls it six times with
resolutions built by hand — a live entry, a retired one, an abandoned one,
nothing, an ambiguity of two, an ambiguity of one abandoned match — asserting
stdout exactly and stderr by the words that carry its meaning, and spawns no
process. That is the through-line from
*Orientation*: the manifest's separate-crate argument is why `lib.rs` exists at
all, `lib.rs` is why a test can name a function in this module, and a function
a test names must be `pub`. `#[must_use]` is the attribute a pure function
whose result is the whole output takes — a caller that dropped the pair would
print nothing and be warned. It lives here rather than in `grove-loop` because
it is presentation: the loop answers what a reference names, and what a
terminal should see about that is this binary's alone. The signature takes a
`Sought<Resolution>`, and `Resolution` is read in full here: `Root`;
`Entry(Located)` for exactly one match; and `Ambiguous(Vec<Located>)` for a
bare slug that matched several, where a `Located` is an entry's current path,
its handle, its kind — `None` for a node — and its outcome. The first arm is
the root, and it is unreachable through `resolve` for the reason the comment
gives — the handler answered the root before calling — so it returns two empty
strings, which would be the not-found shape with no diagnostic; the arm exists
because the `match` is exhaustive over the type, and no test constructs it.

<!-- fragment «render-resolution-head» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="578-591" parent="handlers-reading-and-rendering" -->
````rust
/// Render a resolution to the `(stdout, stderr)` the `resolve` verb emits.
///
/// Kept pure and separate from the I/O so the exact contract is unit-testable
/// without going through the CLI dispatch. It lives here rather than in
/// `grove-loop` because it is presentation and nothing else — the loop crate
/// answers what a reference names, and what a terminal should see about it is
/// this binary's alone.
#[must_use]
pub fn render_resolution(reference: &str, resolution: &Sought<Resolution>) -> (String, String) {
    match resolution {
        // Unreachable through `resolve`, which answers the root before it gets
        // here: the root's path is the caller's own spelling of the tree, and
        // nothing in a resolution carries it.
        Sought::Match(Resolution::Root) => (String::new(), String::new()),
````
<!-- /fragment -->

The entry arm is where a dead end is kept from looking live, and it is the row
the ledger calls `Outcome`. `Outcome` is live, retired or abandoned — the infix
a leaf's filename carries after its position, `DONE-` or `ABANDONED-`, or the
absence of one — and the `Located` the call returns carries it; a node
directory carries none, and a node resolves to its directory with no note,
measured. The path
goes to stdout in every case, because the reference did resolve; the outcome
decides what goes to stderr — nothing for a live entry, and for the other two a
note that names the state and repeats the path, so a session reading only
stderr still knows which entry the note is about. Three process-level tests pin
the arm from outside: `resolve_finds_retired_leaf_with_note` requires the
`DONE` path on stdout and *retired* on stderr,
`resolve_key_resolves_a_node_to_its_directory` requires the directory, and
`terminal_infixes_preserve_filename_kind_and_stable_resolution` in
`crates/grove-llm/tests/session_kind_tree.rs` requires the `DONE` and the
`ABANDONED` paths on stdout and never reads stderr; so the abandoned note is
pinned only through the library, by
`render_found_abandoned_notes_on_stderr_but_still_prints_path`, which also
requires that it does not say *retired*.

<!-- fragment «render-resolution-entry» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="592-608" parent="handlers-reading-and-rendering" -->
````rust
        Sought::Match(Resolution::Entry(entry)) => {
            let stdout = format!("{}\n", entry.path.display());
            let stderr = match entry.outcome {
                Outcome::Live => String::new(),
                Outcome::Done => format!(
                    "note: referenced task is retired (DONE): {}\n",
                    entry.path.display()
                ),
                // The abandoned counterpart of the DONE note above: `resolve`
                // must not let a pruned dead end look live.
                Outcome::Abandoned => format!(
                    "note: referenced task is abandoned (ABANDONED): {}\n",
                    entry.path.display()
                ),
            };
            (stdout, stderr)
        }
````
<!-- /fragment -->

The last two arms are the absent answers, and neither writes to stdout.
`Nothing` becomes one line quoting the reference in Rust's debug form — the
form both absent arms use, and what puts the quotation marks around
`rate-limit` in the transcript's fourth invocation — which would escape a
reference that contained one. `Ambiguous`
becomes a header and one line per match, in the order the loop's walk found
them: the key in brackets, the path, and a tag for a retired or abandoned
match. The key is printed in the `[n]` form because that form is the first the
verb's help lists and the one a bare slug cannot collide with, so each line is
a re-query the session can run unchanged;
`resolve_ambiguous_slug_lists_keys_on_stderr` requires *ambiguous*, `[1]` and
`[2]` on stderr and nothing on stdout, and the two `render_ambiguous_` unit
tests pin *retired* and *(abandoned)* on those lines. The type-level fact behind
the arm is that ambiguity is a `Match`, not a `Nothing`: the search found
entries, several of them, and the loop's doc comment says why that is an answer
rather than a refusal — the caller is a session that can re-ask with a narrower
reference, and listing the matches is what lets it.

<!-- fragment «render-resolution-absent» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="609-634" parent="handlers-reading-and-rendering" -->
````rust
        Sought::Nothing => (
            String::new(),
            format!("resolve: no entry matches reference {reference:?}\n"),
        ),
        Sought::Match(Resolution::Ambiguous(matches)) => {
            let mut stderr =
                format!("resolve: reference {reference:?} is ambiguous; re-query by key:\n");
            for matched in matches {
                let tag = match matched.outcome {
                    Outcome::Live => "",
                    Outcome::Done => " (retired)",
                    Outcome::Abandoned => " (abandoned)",
                };
                stderr.push_str(&format!(
                    "  [{}] {} {}{}\n",
                    matched.handle.key(),
                    matched.handle,
                    matched.path.display(),
                    tag
                ));
            }
            (String::new(), stderr)
        }
    }
}

````
<!-- /fragment -->


`brief-chain` opens the guarded tree, selects the requested leaf or the
next live leaf, and prints the chain root-first. The first path is `_BRIEF.md`;
subsequent paths are the titled files of positioned ancestors.
`leaf_two_levels_deep_returns_root_and_ancestor_node_briefs` pins this order.
`missing_intermediate_node_file_refuses` and `missing_root_node_file_refuses`
require failure at open, before a partial chain can be printed. A valid tree
with no live leaf still uses the shared no-live-leaves diagnostic.


<!-- fragment «handler-brief-chain» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="521-533" parent="handlers-reading-and-rendering" -->
````rust
fn cmd_brief_chain(leaf_path: Option<&Path>) -> Result<()> {
    let worktree = worktree()?;
    let tree = readable(&worktree)?;
    let Some(leaf) = leaf_in(&tree, leaf_path)? else {
        no_live_leaves(&worktree);
        return Ok(());
    };
    for path in verbs::brief_chain(&tree, &leaf)? {
        println!("{}", path.display());
    }
    Ok(())
}

````
<!-- /fragment -->

`leaf_in` has one caller. `kind` needs the same default and does not use this
function: `verbs::kind` takes an `Option<&Path>` and picks for itself when
given `None`, where `verbs::brief_chain` takes a `&Path` and no option, so
`brief-chain` picks in this module and `kind` picks in the loop. That is the
second call *Orientation* counted against the header's slogan for this verb.
The named branch goes through `normalize_leaf_path`, read under its own heading
below; the unnamed branch calls `verbs::pick`, keeps the selection's path,
discards the rest of the selection, and turns `Nothing` into `None`.

<!-- fragment «handler-leaf-in» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="548-558" parent="handlers-reading-and-rendering" -->
````rust
/// The leaf a read verb acts on: the one named, or `pick`'s next.
fn leaf_in(tree: &Tree, leaf_path: Option<&Path>) -> Result<Option<PathBuf>> {
    Ok(match leaf_path {
        Some(path) => Some(normalize_leaf_path(path)),
        None => match verbs::pick(tree)? {
            Sought::Match(selection) => Some(selection.path),
            Sought::Nothing => None,
        },
    })
}

````
<!-- /fragment -->

<a id="the-absent-answer"></a>
## The absent answer: `pick`, `kind`, and one diagnostic

The finished grove is the case the chapter is named for, and the three verbs
that meet it print one line. The carried tree reaches that state once the
review leaf's own session has run and retired it: `01-DONE-impl--rate-limit-k3.md`
beside `02-DONE-review-impl--rate-limit-k4.md`, and no live leaf. Measured on a
tree of that shape, each line below is on stderr, stdout is empty, and each
process exits `0`; the transcript shows the three verbs agreeing, and the table
after the fragments says which test holds each.

```console
$ grove-llm pick
grove atlas: no live leaves; this grove is done

$ grove-llm kind
grove atlas: no live leaves; this grove is done

$ grove-llm brief-chain
grove atlas: no live leaves; this grove is done
```

`cmd_pick` is the shortest handler on this page and the model the others
follow, including `resolve`, whose comment names it. The call returns
`Sought<Selection>`, and a `Selection` is everything a launch needs about one
leaf — its path, its handle and its kind, copied under one shared guard — of
which the handler prints the path and nothing else. The other two fields are
the driver's: it selects in-process and never runs this verb, which is what the
`kind` help means by *nothing here routes a launch*, so the verb is the
session's and a human's view of the same walk. `Nothing` is the finish trigger,
and here it is rendered rather than acted on. The walk — pre-order, per-level
numeric order, briefs and terminal leaves skipped — is the loop's; what this
crate's tests pin is the rendering of it. `picks_first_live_leaf_in_numeric_order`,
`descends_a_node_directory_in_preorder`, `skips_retired_done_leaves`,
`root_brief_is_not_a_leaf` and `foreign_files_are_not_leaves` in
`crates/grove-llm/tests/pick.rs` require the path on stdout, and
`fully_retired_grove_prints_diagnostic_and_exits_zero` requires empty stdout,
*no live leaves* on stderr and exit `0`. No test in that file skips an
`ABANDONED` leaf; the help promises it, and it holds, measured — an abandoned
leaf at position one is passed over for the live leaf at position two.

<!-- fragment «handler-pick» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="511-520" parent="handlers-reading-and-rendering" -->
````rust
fn cmd_pick() -> Result<()> {
    let worktree = worktree()?;
    let tree = readable(&worktree)?;
    match verbs::pick(&tree)? {
        Sought::Match(selection) => println!("{}", selection.path.display()),
        Sought::Nothing => no_live_leaves(&worktree),
    }
    Ok(())
}

````
<!-- /fragment -->

`cmd_kind` differs from `cmd_pick` in two lines. The optional path is
normalised — the comment says to what, and the function that does it is read
next — and passed to the call as an `Option<&Path>`, so the loop, not this
module, picks when the option is `None`. What comes back is `Sought<Kind>`, and
`Kind::label` is the token as the filename carries it, printed with `println!`
so the output is one lowercase token and a newline —
`every_shipped_kind_round_trips_through_the_verb` requires exactly that for
every kind the shipped plugin declares, with nothing on stderr, and the verb
knows of no set: `frobnicate` is a kind if a filename carries it, measured.
`Nothing` is only the unnamed form over a finished grove. A named path that is
not a leaf is a refusal from the call, propagated by the `?` — *Grove leaf not
found* for a path that is no file, a missing path or a node directory alike,
and *path is not a current-format Grove leaf* for a brief — with exit `1`; and a malformed name anywhere in the
tree refuses earlier still, at `readable`, before the named leaf is looked at,
with the loop's wording naming the token and the grammar. Those refusals are
the loop's, and `crates/grove-llm/tests/kind.rs` pins none of them. What it
pins is that the body of the leaf is never read — a `**Kind:**` or
`**Harness:**` line, however garbled, changes nothing — and that the two
routing flags the verb once carried are rejected as unknown arguments.

<!-- fragment «handler-kind» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="534-547" parent="handlers-reading-and-rendering" -->
````rust
fn cmd_kind(leaf_path: Option<&Path>) -> Result<()> {
    let worktree = worktree()?;
    let tree = readable(&worktree)?;
    // Normalize a cwd-relative path to what the verb accepts (absolute or
    // grove-root-relative); a `None` stays `None` so the verb defaults to
    // `pick`'s next live leaf.
    let leaf = leaf_path.map(normalize_leaf_path);
    match verbs::kind(&tree, leaf.as_deref())? {
        Sought::Match(kind) => println!("{}", kind.label()),
        Sought::Nothing => no_live_leaves(&worktree),
    }
    Ok(())
}

````
<!-- /fragment -->

The diagnostic is one function and one wording, and its label is the working
tree's basename: `atlas` for the carried session, because `worktree` resolved
`/work/atlas`. The comment's parenthesis — that the basename equals the grove
name — is a convention the binary checks nothing of; it holds when a working
tree is named for the grove it drives, which is how the loop derives a
session's name, and a grove driven from `/work/app` is labelled `app`.
`unwrap_or_else` covers a path with no final component, which a resolved
working tree has only at the filesystem root, so the fallback is the type's
rather than a case the verb meets.

<!-- fragment «helper-label» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="926-934" parent="path-and-label-helpers" -->
````rust
// The grove's display label for the pick/brief-chain "no live leaves" diagnostic
// — the worktree directory's basename (it equals the grove name / branch).
fn label(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| worktree.display().to_string())
}

````
<!-- /fragment -->

`no_live_leaves` has three callers — `cmd_pick`, `cmd_brief_chain` and
`cmd_kind` — and its doc comment says *four*. `resolve` is the fourth reading
verb and does not call it, because its absent answer carries a reference to
quote and is `render_resolution`'s. The comment's count is a claim a reader can
check with one search and it does not hold; the function's own claim — one
wording, printed here rather than spelled at each caller — does. The wording
ends in *this grove is done*, which is a statement for the session reading it
and not a signal: no verb on this page acts on it, and the driver, which never
runs this verb, reaches the same answer by its own walk and acts on `Nothing`
rather than on any line of text.

<!-- fragment «helper-no-live-leaves» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="935-942" parent="path-and-label-helpers" -->
````rust
/// The one diagnostic every read verb shares, printed once here rather than
/// spelled four times.
fn no_live_leaves(worktree: &Path) {
    eprintln!(
        "grove {}: no live leaves; this grove is done",
        label(worktree)
    );
}
````
<!-- /fragment -->


The table distinguishes absent selections from malformed trees. An absent
selection is a successful answer with a diagnostic; an invalid required-file
set is a refusal and exits 1 before the requested read runs.


| Verb | What was absent | stdout | stderr | Exit | Held by |
|---|---|---|---|---|---|
| `pick` | a live leaf | nothing | `grove atlas: no live leaves; this grove is done` | `0` | `fully_retired_grove_prints_diagnostic_and_exits_zero` |
| `kind`, no argument | a live leaf | nothing | the same line | `0` | `empty_grove_prints_no_live_leaves_on_stderr_and_exits_zero` |
| `brief-chain`, no argument | a live leaf | nothing | the same line | `0` | the handler alone |
| `brief-chain` | required node file missing | nothing | level-validation error | `1` | `missing_root_node_file_refuses` |
| `resolve` | any entry | nothing | `resolve: no entry matches reference "…"` | `0` | `resolve_not_found_exits_zero_with_diagnostic` |
| `resolve` | one entry, several found | nothing | the ambiguity listing | `0` | `resolve_ambiguous_slug_lists_keys_on_stderr` |
| `resolve` | a live entry behind the match | the path | the retired or abandoned note | `0` | `resolve_finds_retired_leaf_with_note` |
| any of the four | the grove itself | nothing | `grove root not found`, and the remedy | `1` | `errors_when_grove_root_absent`, for `pick` alone and the first line alone — a refusal, not an answer |
| `resolve` | a well-formed reference | nothing | the loop's wording | `1` | `resolve_malformed_bracket_ref_errors`, which asserts the exit alone, and `resolve_refuses_an_empty_reference` — refusals |

The composite that reassembles the handlers is stated here, in source order,
and the source index names it as one of the root's twenty-two children.

<!-- fragment «handlers-reading-and-rendering» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="511-634" parent="source-command-surface" -->
<!-- insert «handler-pick» -->
<!-- insert «handler-brief-chain» -->
<!-- insert «handler-kind» -->
<!-- insert «handler-leaf-in» -->
<!-- insert «handler-resolve» -->
<!-- insert «render-resolution-head» -->
<!-- insert «render-resolution-entry» -->
<!-- insert «render-resolution-absent» -->
<!-- /fragment -->

<a id="the-leaf-path"></a>
## A leaf path, three ways

`normalize_leaf_path` is the rule behind the optional path of `brief-chain`
and `kind`, and its three cases are the comment's. An absolute path is returned
as it is — the driving case, where a session passes back what `pick` or
`brief-chain` printed. A relative path that exists relative to the current
directory is made absolute by joining it onto that directory, which is what
lets `.grove/01-impl--rate-limit-k3.md` typed at the working-tree root name the
leaf. Anything else is passed through unchanged, and the through-line from *The
grammar and the openings* is why: `worktree` returns the working tree's root
and the loop joins `.grove` itself, so this module holds no spelling of the
grove root that it could join a bare `01-impl--rate-limit-k3.md` onto, and the
join is left to the verb, which makes it against the tree's own root. A current
directory that cannot be read takes the same branch. The table is the three
cases measured, with a fourth row for the case the comment's *typed at the
worktree root* qualifier exists for; it is what a reader needs to predict which
spelling a verb will accept from where.

| Argument | Current directory | What the function returns | What the verb does with it |
|---|---|---|---|
| `/work/atlas/.grove/01-impl--rate-limit-k3.md` | anywhere | the same path | the leaf |
| `.grove/01-impl--rate-limit-k3.md` | `/work/atlas` | `/work/atlas/.grove/01-impl--rate-limit-k3.md` | the leaf |
| `01-impl--rate-limit-k3.md` | `/work/atlas` | `01-impl--rate-limit-k3.md`, unchanged | joins it onto the grove root: the leaf |
| `../../.grove/01-impl--rate-limit-k3.md` | `/work/atlas/crates/gateway` | the absolute path | the leaf |
| `.grove/01-impl--rate-limit-k3.md` | `/work/atlas/crates/gateway` | `.grove/01-impl--rate-limit-k3.md`, unchanged | joins it onto the grove root, and refuses: *Grove leaf not found: /work/atlas/.grove/.grove/01-impl--rate-limit-k3.md*, exit `1` |

The last row is the limit the qualifier names. The function cannot tell a
grove-relative name from a mistyped working-tree-relative one, because the only
test it can make is against the current directory; a `.grove/` path typed from
a subdirectory fails that test, passes through, and is joined by the verb onto
a root that already ends in `.grove`. The refusal quotes the doubled path,
which is the diagnosis. The first two branches are exercised by this crate's
fixture-driven tests — `terminal_infixes_preserve_filename_kind_and_stable_resolution`
in `crates/grove-llm/tests/session_kind_tree.rs` passes `kind` an absolute
path, and every test that passes a `.grove/`-relative path from the
working-tree root is the second row — and no test exercises the pass-through
branch or the last row.

<!-- fragment «helper-normalize-leaf-path» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="902-925" parent="path-and-label-helpers" -->
````rust
// Normalize a user-supplied leaf path to what the verbs accept (absolute, or
// relative to the grove root). The real driving flow passes back the **absolute**
// path `pick`/`brief-chain` printed — handled by the absolute branch. A path given
// relative to the cwd that exists (e.g. `.grove/01-foo-k1.md` typed at the
// worktree root) is resolved to absolute here; a bare grove-root-relative name
// (`01-foo-k1.md`) does not exist relative to cwd, so it passes through for the
// verb to join onto the grove root.
fn normalize_leaf_path(p: &Path) -> PathBuf {
    if p.is_absolute() {
        return p.to_path_buf();
    }
    match std::env::current_dir() {
        Ok(cwd) => {
            let cwd_rel = cwd.join(p);
            if cwd_rel.exists() {
                cwd_rel
            } else {
                p.to_path_buf()
            }
        }
        Err(_) => p.to_path_buf(),
    }
}

````
<!-- /fragment -->

The composite that reassembles the three helpers, the last block of the module,
is stated here.

<!-- fragment «path-and-label-helpers» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="902-942" parent="source-command-surface" -->
<!-- insert «helper-normalize-leaf-path» -->
<!-- insert «helper-label» -->
<!-- insert «helper-no-live-leaves» -->
<!-- /fragment -->

<a id="the-four-contracts"></a>
## The four contracts, as `--help` states them

The four variants are lines 75 to 124 of the `Command` enum, between
`RootInit`, which *Growing the tree* owns, and `LeafAdd`. Their doc comments
are the `--help` text a session reads, and the guide paraphrases them for the
human; this page reproduces them because they are corpus, and reads them for
one thing only — which line of the handler keeps each promise, and which test
would catch its breach. `Pick` is a unit variant: no argument, and `run` calls
its handler with none.

<!-- fragment «verbs-pick-help» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="75-82" parent="verbs-reading" -->
````rust
    /// Print the absolute path of the next live leaf in this grove's tree — a
    /// recursive depth-first **pre-order** walk over the directory tree (a node
    /// is a directory of numbered children and exactly one `_<slug>.md` node file),
    /// returning the
    /// first live leaf and skipping briefs and terminal leaves — retired
    /// (`DONE`) and abandoned (`ABANDONED`) alike. Empty stdout
    /// (and a diagnostic on stderr) when the grove has no live leaves.
    Pick,
````
<!-- /fragment -->

`BriefChain` and `Kind` carry the same optional path with the same two-line
description, and *A leaf path, three ways* is the account of what *absolute,
or relative to the grove root* costs a path that is neither. `BriefChain`'s
comment makes three promises and the table below places them; the one it does
not make — what the no-argument form does on a finished grove — is the
handler's, stated under *The absent answer*.

<!-- fragment «verbs-brief-chain-help» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="83-92" parent="verbs-reading" -->
````rust
    /// Print the node-file chain for a leaf, root→leaf, one absolute path per
    /// line — `_BRIEF.md` at the root, then each ancestor's `_<slug>.md`,
    /// from the grove root down to the leaf's containing directory. With no
    /// argument the chain is computed for `pick`'s next leaf. Missing or misplaced
    /// node files refuse the whole tree read.
    BriefChain {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
    },
````
<!-- /fragment -->

Two clauses of `Kind`'s comment are read against the source rather than
repeated. *A task-shaped current filename with
a missing or unknown kind is malformed* is a sentence from the closed-set era
of the grammar: since any well-formed token is a kind, which the same comment
says in its second line, there is no unknown kind to refuse, and what refuses
is a token that is not one — measured under `cmd_kind` above — so *malformed*
is the whole of it and *unknown* names nothing. *Mirroring `brief-chain`* is
true of the handler and not of the tests: the diagnostic is one function with
three callers, and the test that pins it for `kind` is the only one that pins
it for the no-argument form of either.

<!-- fragment «verbs-kind-help» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="93-107" parent="verbs-reading" -->
````rust
    /// Print a leaf's task **kind** — the token before the `--` — read from its
    /// current-format filename. Any well-formed token is a kind: grove holds no
    /// list of them (`docs/adr/a-kind-is-an-open-token.md`).
    /// With no argument the kind is read for `pick`'s next live leaf; on an
    /// empty grove it prints the standard "no live leaves" diagnostic on stderr
    /// (mirroring `brief-chain`) and exits 0. A task-shaped current filename
    /// with a missing or unknown kind is malformed and errors visibly; foreign
    /// files remain ignored. The output is a single lowercase token + newline.
    /// It is a diagnostic and tree-interface verb: the loop driver selects its
    /// own leaf in-process, so nothing here routes a launch.
    Kind {
        /// Optional leaf path. Absolute, or relative to the grove root
        /// (`.grove/`). If absent, uses `pick`'s next live leaf.
        leaf_path: Option<PathBuf>,
    },
````
<!-- /fragment -->


The help describes keys, slugs and current full handles. Numeric keys
ignore the title; handles check it. A node resolves to its directory and its
brief lives in the uniquely validated titled file. Ambiguous slug output names
keys, handles and paths. `.` resolves the root, while paths belong to the
mutating verbs’ reference interface rather than `resolve`.


<!-- fragment «verbs-resolve-help» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="108-124" parent="verbs-reading" -->
````rust
    /// Resolve a reference to its current file path, searching live, retired
    /// (`DONE`), **and** abandoned (`ABANDONED`) entries alike
    /// across the whole directory tree. A permanent key (`[n]` or bare `n`,
    /// optionally `[n]-slug`) resolves the unique keyed entry; a bare slug
    /// resolves by slug (0 ⇒ not found, 1 ⇒ that entry, >1 ⇒ ambiguous, listing
    /// each match's key so you re-query by key); the full `<slug>-k<key>` handle
    /// resolves by key and checks the current title. A node resolves to
    /// its **directory** path; its `_<slug>.md` carries the brief. Prints
    /// the path on stdout; a `DONE` or `ABANDONED` match also prints its own
    /// note on stderr (so the two are distinguishable — a resolved dead end
    /// never looks live); a not-found or ambiguous reference prints a
    /// diagnostic on stderr instead. Either way it still exits zero.
    Resolve {
        /// The reference: `[n]` / `n` / `[n]-slug` / `<slug>-k<key>` (by
        /// permanent key) or a bare slug.
        reference: String,
    },
````
<!-- /fragment -->

The table is the chapter's reading of the four comments as promises, and it is
what the closing chapter's row for each of these verbs is built from: for each
promise, the line in the handler that keeps it and the test that would catch
its breach, with the promises no test holds named as such rather than left
implied.

| Verb | The help promises | Kept at | Held by |
|---|---|---|---|
| `pick` | the first live leaf, pre-order, briefs and `DONE`/`ABANDONED` leaves skipped | the call, line 514; the walk is the loop's | `descends_a_node_directory_in_preorder`, `skips_retired_done_leaves`; the `ABANDONED` skip by no test in this crate |
| `pick` | empty stdout and a diagnostic on stderr when no live leaves | lines 516 and 518 | `fully_retired_grove_prints_diagnostic_and_exits_zero` |
| `brief-chain` | root→leaf, one absolute path per line | lines 528 to 530 | `leaf_two_levels_deep_returns_root_and_ancestor_node_briefs` |
| `brief-chain` | no argument: `pick`'s next leaf | `leaf_in`, lines 552 to 554 | `no_arg_form_uses_picks_next_leaf` |
| `brief-chain` | a missing required node file refuses the read | the guarded opening, before the call | `missing_intermediate_node_file_refuses`, `missing_root_node_file_refuses` |
| `kind` | one lowercase token and a newline | line 542 | `every_shipped_kind_round_trips_through_the_verb` |
| `kind` | no argument: `pick`'s leaf; a finished grove: the diagnostic, exit `0` | lines 540 to 543 | `no_arg_form_reads_picks_next_leaf`, `empty_grove_prints_no_live_leaves_on_stderr_and_exits_zero` |
| `kind` | a malformed name errors visibly; foreign files are ignored | the `?` on line 536 at the opening, for a malformed name anywhere in the tree; the `?` on line 541 for a named path that is not a leaf; the walk | by no test in `kind.rs`; `foreign_files_are_not_leaves` in `pick.rs` |
| `kind` | nothing here routes a launch | no flag and no field to route by | `the_removed_routing_flags_are_rejected` |
| `resolve` | a key in three spellings; the handle by key and current title; a node to its directory | the call, line 562 | `resolve_by_key_bracketed_and_bare`, `resolve_by_full_slug_handle_finds_by_terminal_key`, `resolve_key_resolves_a_node_to_its_directory`; `[n]-slug` by no test |
| `resolve` | a bare slug: not found, that entry, or ambiguous listing each key and handle | lines 609 to 631 | `resolve_not_found_exits_zero_with_diagnostic`, `resolve_by_unique_slug`, `resolve_ambiguous_slug_lists_keys_on_stderr` |
| `resolve` | a `DONE` or `ABANDONED` match prints its path and its own note | lines 593 to 606 | `resolve_finds_retired_leaf_with_note`; `render_found_abandoned_notes_on_stderr_but_still_prints_path` |
| `resolve` | either way it still exits zero | line 575 | the three slug tests above, each asserting success |

The composite that reassembles the four variants is stated here.

<!-- fragment «verbs-reading» owner="information-not-error" source="crates/grove-llm/src/cli.rs" lines="75-124" parent="source-command-surface" -->
<!-- insert «verbs-pick-help» -->
<!-- insert «verbs-brief-chain-help» -->
<!-- insert «verbs-kind-help» -->
<!-- insert «verbs-resolve-help» -->
<!-- /fragment -->

The four verbs that only read have now been read, and each renders the answer
it was given and refuses nothing the loop did not refuse first. The next four
write, and what a thin binary has to get right there is not what it prints but
what it reads, and asks, before it takes the lock.

[Previous: The grammar and the openings](02-the-grammar.md) | [Contents](README.md) | [Next: Growing the tree](04-growing-the-tree.md)

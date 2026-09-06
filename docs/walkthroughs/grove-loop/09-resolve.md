# Resolve
<!-- book-page id="resolve" slice="wider-than-a-key" order="9" -->
[Previous: Kind, and the brief chain](08-kind-and-briefs.md) | [Contents](README.md) | [Next: Growing: leaf-add and leaf-insert](10-growing.md)

<a id="wider-than-a-key"></a>
## The rule: the reference grammar is wider than a key

Chapters 7 and 8 both took a leaf that the tree had already picked out — the
walk chose it, and the argument named it by a path. This chapter is the one
where **the operator does the naming**, and that changes what the layer has to
own. A session, a commit message, an ADR or a brief refers to an entry by
whatever spelling was to hand: a bare or bracketed permanent key, the
`<slug>-k<key>` handle the tree prints, a bare slug, `.` for the root, or a path
pasted back out of an earlier answer. All of them have to arrive at the same
entry, and this chapter owns every one.

> The library beneath grove can find an entry by key and can walk with a
> predicate. It cannot be told what a *slug* is. So the grammar that turns any of
> five spellings into one entry is grove's, and so is the only outcome the
> library has no word for: **a reference that matched more than one thing**.

That is the third and last of `task_tree.rs`'s answers to *what did not go, and
why could it not?* — and it is the narrowest place in the whole seam. The
module's own header said *what changes is who owns the walk*; here the walk comes
back to grove, not because the library refused it but because the library has no
type in which the question can be asked. `by_key` takes a `Key`. `seek` takes the
consumer's own predicate and stops at the first hit. There is no `by_label`,
because the trait names no label type for one to take — and a `seek` would answer
*the first `add`* when the honest answer is *two things are called `add`, here are
their keys*.

**Ambiguity is the chapter's whole argument.** It is not an error, and the
distinction is load-bearing: the caller is a session that can re-ask with a
narrower reference, and listing the matches is what lets it. An error would end
the session; an answer lets it name the entry properly on the second try. The
cost grove pays for that is a whole walk instead of a short-circuiting search,
every time a bare slug is resolved — which is the price of the outcome, stated in
the doc comment and again below.

The carried example reaches its ninth step here, and it is the step where one
entity has four names.

```text
.grove/                                the tree as root-init left it
├── BRIEF.md                           carries no key and no slug — unreferenceable
└── 01-requirements--plan-k1.md        position 01, kind requirements, slug plan, key 1

resolve(tree, "plan-k1")   the handle the tree prints, read by its terminal key
resolve(tree, "[1]")       the bracketed key
resolve(tree, "1")         the bare key
resolve(tree, "plan")      the bare slug — a walk, because it might match twice
  └─ all four ⇒ Resolution::Entry(Located {
                  path:    <root>/01-requirements--plan-k1.md   built by chapter 6's entry_path
                  handle:  plan-k1                              the way to re-ask unambiguously
                  kind:    Some(requirements)                   None for a node directory
                  outcome: Live                                 so a dead end cannot look live
                })

resolve(tree, ".")     ⇒ Resolution::Root      the root is not an entry
resolve(tree, "BRIEF") ⇒ Sought::Nothing       and neither is the root brief
```

The bare slug is the spelling that can fail to be unique. Once a second entry in
the same tree carries the slug `plan` — a different subtree, a different key,
possibly retired — it answers `Ambiguous`, and it does so with both `Located`s in
hand so the caller can re-ask by either key. The other spellings are unaffected,
because a key is unique tree-wide. **The grammar is wider than a key, and exactly
one of its forms pays for the width.**

**One form is missing from that list, and its absence is worth stating before the
code is read: `resolve` does not take a path.** `verbs::resolve` is one line over
`resolve_in`, and `resolve_in` checks `.` and then goes straight to the key/slug
grammar; there is no branch that tries the argument as a file. The path form
belongs to the *other* caller of the same grammar — `reference`, the door the
mutating verbs of chapter 10 come through — and that function tries a path first
and a reference only afterwards. Both live in this chapter's block, which is why
the chapter owns the whole grammar even though no single entry point accepts all
of it.

That also settles a count chapter 1 reproduced without adjudicating. `Reference`'s
own doc comment opens *Four forms, and the whole grammar is here* and then names
five things — `.`, a permanent key, a handle, a bare slug, and a path. The
sentence is not wrong so much as compressed: `Reference` is shared by both
callers, so its list is the union, and `.` is a path as well as a root spelling —
`reference`'s comment says exactly that, *`.` is a path — the grove root — and so
needs no case of its own*. Count the union and there are five; count what any one
entry point accepts and there are four. The page reads it as the union, and says
which four `resolve` itself takes.

<a id="an-answer-the-library-cannot-return"></a>
## An answer the library cannot return

The chapter's first ownership block is the last of `task_tree.rs`'s five
production concerns, at lines 747 to 1015 — 269 lines, 40% of them comment
prose. It opens on the two types the grammar answers in, and they are declared
before anything that produces them, which is the order the concern is best read
in: the outcome first, then the walk that reaches it.

<!-- fragment «resolution» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="747-1015" parent="source-task-tree" -->
<!-- insert «resolution-outcome» -->
<!-- insert «resolution-located» -->
<!-- insert «resolution-located-fn» -->
<!-- insert «resolution-resolve-in» -->
<!-- insert «resolution-lookup-type» -->
<!-- insert «resolution-slug-match-key» -->
<!-- insert «resolution-grammar» -->
<!-- insert «resolution-reference» -->
<!-- insert «resolution-existing-path» -->
<!-- insert «resolution-ref-type» -->
<!-- insert «resolution-parse-ref» -->
<!-- insert «resolution-read-count» -->
<!-- /fragment -->

`Resolution` is a three-way answer, and the interesting thing about it is what
is **not** in it.

<!-- fragment «resolution-outcome» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="747-765" parent="resolution" -->
````rust
/// What a reference resolved to.
///
/// **Ambiguity is an answer, not an error**: the caller is a session that can
/// re-ask with a narrower reference, and the whole point of listing the matches
/// is that it can. *Nothing matched* is not here at all — that is the store's
/// [`Sought::Nothing`](ordinal_fs_tree::Sought), which is what
/// [`crate::verbs::resolve`] answers with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// The reference was `.`, the grove root. It is not an entry: it carries no
    /// key, no slug and no kind.
    Root,
    /// Exactly one entry matched.
    Entry(Located),
    /// A bare-slug reference matched more than one entry. Each carries its own
    /// handle, so the caller re-queries by the unambiguous key in it.
    Ambiguous(Vec<Located>),
}

````
<!-- /fragment -->

*Nothing matched* is absent from the enum on purpose. It is not grove's word:
the store already has one, `Sought::Nothing`, and the verb answers with it
rather than adding a fourth variant that would mean the same thing. So the
public signature of the whole operation is `Sought<Resolution>` — the library's
word for *found or not*, wrapped around grove's word for *what, exactly*. The
two vocabularies compose rather than duplicate, and the seam is visible in the
return type. This is the same discipline chapter 5 read in `Vacancy`, from the
other direction: there grove took the library's shape and re-worded it, here
grove takes the library's shape and keeps it.

`Root` is the variant with no entry behind it. `.` names the grove root, which
carries no key, no slug and no kind, and so cannot be a `Located` — the caller
gets a variant that says *the root* and nothing more. `Ambiguous` carries a
vector rather than a count, because a count would not let the caller act.

The entry that a match resolves to is described once, and every field of it
answers a question a caller was otherwise going to answer wrongly.

<!-- fragment «resolution-located» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="766-787" parent="resolution" -->
````rust
/// One entry a reference matched.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Located {
    /// Where it is now.
    pub path: PathBuf,
    /// Its position-free identity, which is also how to re-ask for it
    /// unambiguously.
    pub handle: Handle,
    /// Its session kind — `None` for a node directory, which is not driven as a
    /// session.
    pub kind: Option<Kind>,
    /// Live, `DONE` or `ABANDONED`.
    ///
    /// **Not in `docs/specs/module-decomposition.md`'s listing of this struct,
    /// and deliberately added back.** `resolve` must not let a retired or
    /// abandoned dead end look live, and the note that says so is the caller's
    /// to print — the alternative was for the caller to read the outcome off the
    /// filename, which is the one thing principle 3 forbids anything but
    /// [`crate::TaskName`] to do.
    pub outcome: Outcome,
}

````
<!-- /fragment -->

**Three of the four fields exist because the caller must not have to read the
filename.** `path` is where the entry is *now* — the position may have changed
since anyone wrote the reference down. `handle` is the position-free identity,
and the doc comment gives the reason it is here rather than a bare key: it is
*also how to re-ask for it unambiguously*, which is precisely what the caller of
an `Ambiguous` needs. `kind` is `None` for a node directory, which distinguishes
a node from a leaf without parsing anything.

`outcome` carries its own argument, and it is the one field the design record
did not have. The doc comment states the case in full: `resolve` must not let a
retired or abandoned dead end look live, and the alternative — letting the caller
read the outcome off the filename — is the one thing principle 3 forbids anything
but `TaskName` to do. So the field is added back against the listing in
`docs/specs/module-decomposition.md`, and the comment says so rather than
quietly diverging. The chapter's fourth test demonstrates why that field is
required.

The construction is a single function, and it is the only place in the block
that can fail for a reason that is not the caller's fault.

<!-- fragment «resolution-located-fn» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="788-804" parent="resolution" -->
````rust
/// Everything a resolved reference says about one entry.
fn located(root: &Path, entry: Entry<'_, TaskName>) -> Result<Located> {
    let triple = entry
        .triple()
        .context("a resolved reference matched the root brief, which carries no identity")?;
    let (kind, slug) = match &triple.parts {
        Parts::Leaf { kind, slug, .. } => (Some(kind.clone()), slug.clone()),
        Parts::Node { slug } => (None, slug.clone()),
    };
    Ok(Located {
        path: entry_path(root, entry),
        handle: Handle::new(slug, triple.key),
        kind,
        outcome: entry_outcome(&entry),
    })
}

````
<!-- /fragment -->

`located` is where the two chapters before this one are drawn on: `entry_path`
is chapter 6's one place paths are built, and `entry_outcome` is chapter 6's
reading of the terminal marks. What this chapter adds is the `Parts` match —
a leaf yields a kind and a slug, a node yields a slug and `None` — and
`Handle::new`, which pairs that slug with the key the triple already carries.
Chapter 3 established that the handle *is* the identity; here it is built rather
than parsed, which is why the key comes from the triple and not from the text of
the reference.

The `.context(…)` on the first line is a refusal, and it is the block's most
interesting one because it describes a state the rest of the file works to make
impossible: an entry that matched but carries no identity is the root brief, and
the root brief is unreachable through either branch of the grammar. Whether any
test observes that arm is a question this chapter answers by measurement rather
than by reading, further down.

<a id="one-walk-two-callers"></a>
## One walk, and the two verbs that share it

The verb the chapter is named for is next, and it is short because the work is
below it. Its doc comment is the longest in the block, and what it spends its
length on is the cost rather than the code.

<!-- fragment «resolution-resolve-in» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="805-845" parent="resolution" -->
````rust
/// `resolve <ref>` against a tree already read.
///
/// Turn a reference into the current path of the entity it names, searching the
/// whole tree — live leaves, `DONE` and `ABANDONED` leaves, and node directories
/// alike.
///
///   * `.` → [`Resolution::Root`].
///   * `[n]` / `n` → the entity whose permanent key is `n`. Keys are unique
///     tree-wide, so this is the library's `by_key`.
///   * `[n]-slug` → same; the slug part is decorative (ignored).
///   * bare slug → 0 ⇒ `Sought::Nothing`; 1 ⇒ `Entry`; >1 ⇒ `Ambiguous`.
///   * `<slug>-k<key>` → the full canonical handle, read as its terminal
///     `-k<key>`. Tried only after the bare slug fails, so a literal slug ending
///     in `-k<digits>` still wins.
///
/// **The one lookup grove has that is not by key**, and the one place the seam's
/// narrowness is felt from grove's side. The library offers `by_key` and a
/// `seek` taking the consumer's own predicate, and deliberately *no* lookup by
/// label — the trait names no label type, so a `by_label` would have nothing to
/// take. Slug lookup is therefore a walk with grove's predicate over grove's own
/// `Parts`, and it is a whole walk rather than a `seek` because ambiguity is a
/// property of the match *set*: `seek` short-circuits at the first hit, which is
/// precisely the answer `resolve` must not give.
///
/// The root brief is unreferenceable: it carries no key and no slug.
pub(crate) fn resolve_in(tree: &Tree, reference: &Reference) -> Result<Sought<Resolution>> {
    if reference.is_root() {
        return Ok(Sought::Match(Resolution::Root));
    }
    Ok(match lookup(tree.snapshot(), reference.as_str())? {
        Lookup::Found(entry) => Sought::Match(Resolution::Entry(located(tree.root(), entry)?)),
        Lookup::NotFound => Sought::Nothing,
        Lookup::Ambiguous(matches) => Sought::Match(Resolution::Ambiguous(
            matches
                .into_iter()
                .map(|entry| located(tree.root(), entry))
                .collect::<Result<Vec<_>>>()?,
        )),
    })
}

````
<!-- /fragment -->

The doc comment is the chapter's rule in the crate's own words, and the reason
this chapter's prose does not restate the grammar line by line: the five bullets
already say what each spelling does, and the fragment above quotes them
verbatim. What the comment argues, and what is worth connecting, is the sentence
beginning **the one lookup grove has that is not by key**. Two claims sit inside
it, and they are different claims.

The first is about the *seam*: the library offers `by_key` and a `seek` taking
the consumer's predicate, and deliberately no lookup by label, because the trait
names no label type. That is a fact about `ordinal-fs-tree`'s algebra, and it is
why the walk is here at all. Chapter 6 read the same shape from the other side —
the library returns no paths, so grove builds them; here the library knows no
labels, so grove matches them.

The second is about the *outcome*, and it is the sharper one: it is a whole walk
rather than a `seek` **because ambiguity is a property of the match set**. A
`seek` would short-circuit at the first hit, which is exactly the answer
`resolve` must not give. A reader who knows the library will notice that `seek`
is the cheaper call and would have been the obvious one; the comment exists to
say that the cheap call answers a different question. This is the clearest
instance in Part II of a cost paid for meaning rather than for mechanism.

`resolve_in` itself is short because the work is elsewhere. It takes a tree
already read — the guarded observation chapters 5 and 7 established — handles
`.` before anything else, and otherwise maps the three `Lookup` cases onto the
three answers. The `Ambiguous` arm is the only place a `Result` is collected out
of a vector, because each match must be turned into a `Located` and any of them
could in principle fail.

The intermediate type is what makes one grammar serve two verbs.

<!-- fragment «resolution-lookup-type» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="846-858" parent="resolution" -->
````rust
/// What a reference matched in the snapshot, before anything is said about it.
///
/// **Entries and not paths**, because a mutating verb needs the entry itself: it
/// has to read a key off it and call the library *by key* against this same
/// snapshot (`docs/ARCHITECTURE.md#library-refusals`, clause 1). [`resolve_in`]
/// renders the same three outcomes as paths, so grove has one lookup and not
/// two — the read verb's answer and the write verb's are the same walk.
enum Lookup<'a> {
    Found(Entry<'a, TaskName>),
    NotFound,
    Ambiguous(Vec<Entry<'a, TaskName>>),
}

````
<!-- /fragment -->

**`Lookup` holds entries, not paths, and the doc comment gives the reason in one
sentence**: a mutating verb needs the entry itself, because it has to read a key
off it and call the library *by key* against this same snapshot. That is clause 1
of the library-refusals rule, and chapter 10 is where the mutating side of it is
read. `resolve_in` renders the same three outcomes as paths, so grove has one
lookup and not two — *the read verb's answer and the write verb's are the same
walk*.

That sentence is worth pausing on, because it is a claim about what could have
gone wrong rather than about what the code does. Two lookups would have been the
natural shape: a read verb that wants paths and a write verb that wants entries
have different outputs, and writing each its own search is the path of least
resistance. The cost would not have shown up as a bug for a long time — both
would have worked — and then one of them would have gained a spelling the other
lacked. Sharing the walk makes that divergence impossible to introduce
accidentally, and it is why `Lookup` is a private type with no other purpose than
to be the thing both verbs read.

One small helper serves only the ambiguous arm, and it carries an
`unreachable!`.

<!-- fragment «resolution-slug-match-key» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="859-869" parent="resolution" -->
````rust
/// The key of an entry a bare slug matched.
///
/// [`Lookup::Ambiguous`] arises only from the slug branch of [`lookup`], whose
/// filter reads the slug off a `triple` — so every entry in it is positioned and
/// carries a key.
fn slug_match_key(entry: &Entry<'_, TaskName>) -> u32 {
    entry
        .key()
        .map_or_else(|| unreachable!("a slug match is positioned"), Key::get)
}

````
<!-- /fragment -->

The doc comment supplies the argument that makes the `unreachable!` sound:
`Lookup::Ambiguous` arises **only** from the slug branch of `lookup`, whose
filter reads the slug off a `triple`, so every entry in the vector is positioned
and carries a key. The invariant is not local to this function — it is a property
of the one place that constructs the variant — which is why the comment states
where it comes from rather than asserting it. Chapter 8 read an arm of the same
species in `kind_in`, a `bail!` the file itself calls unreachable; this one is a
panic rather than a refusal, and the difference is that `kind_in`'s arm sits on a
public path where a future caller could reach it, while this one is reachable
only from ten lines away.

Then the grammar itself, which is the function the whole chapter is named for.

<!-- fragment «resolution-grammar» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="870-918" parent="resolution" -->
````rust
/// The reference grammar itself: `[n]` / `n` / `[n]-slug` by key, a bare slug by
/// slug, and a full `<slug>-k<key>` handle by its terminal key once the bare
/// slug has failed.
fn lookup<'a>(snapshot: &'a Snapshot<TaskName>, reference: &str) -> Result<Lookup<'a>> {
    // The library answers a search with `Sought`, its own word for *matched
    // nothing* — not a refusal, and not an error. Grove already has a word for
    // the same thing in its own vocabulary, so this maps one onto the other and
    // stops there: `Lookup` is what the rest of grove reads.
    let by_key = |key: u32| -> Lookup<'a> {
        match snapshot.by_key(Key::new(key)) {
            Sought::Match(entry) => Lookup::Found(entry),
            Sought::Nothing => Lookup::NotFound,
        }
    };
    match parse_ref(reference)? {
        Ref::Key(key) => Ok(by_key(key)),
        Ref::Slug(slug) => {
            // A whole walk and never `seek`: ambiguity is a property of the
            // match *set*, and `seek` short-circuits at the first hit — which is
            // precisely the answer this must not give.
            let matches: Vec<Entry<'a, TaskName>> = snapshot
                .walk()
                .filter(|entry| {
                    entry
                        .triple()
                        .is_some_and(|triple| triple.parts.slug().as_str() == slug.as_str())
                })
                .collect();
            Ok(match matches.len() {
                // A bare slug that matched nothing is retried as a reference
                // ending in a key, and the peel is the name owner's —
                // `task_name::terminal_key`, which shares `peel_key` with the
                // filename grammar. **Not `Handle::parse`**, which would also
                // require the head to be a slug: an operator pastes a retired
                // leaf's whole stem (`01-DONE-impl--build-k5`) and means key 5,
                // and the deleted `task_tree::handle_key` served that by
                // ignoring everything before the key. A reference that ends in
                // no key is simply unmatched.
                0 => match task_name::terminal_key(&slug) {
                    Some(key) => by_key(key.get()),
                    None => Lookup::NotFound,
                },
                1 => Lookup::Found(matches[0]),
                _ => Lookup::Ambiguous(matches),
            })
        }
    }
}

````
<!-- /fragment -->

**Two vocabularies meet in the first four lines and the comment says so
plainly.** The library answers a search with `Sought`, its own word for *matched
nothing* — not a refusal and not an error — and grove already has a word for the
same thing, so `by_key` maps one onto the other and stops there. The closure is
the whole of the translation, and everything downstream reads `Lookup`.

The key branch is a single call. The slug branch is the walk, and it carries the
`seek` argument a second time, in the body this time rather than in the doc
comment — a deliberate repetition, because the temptation to replace `walk` with
`seek` is at its strongest exactly here, three lines above the `matches.len()`
that needs the whole set.

**The zero case is the chapter's most consequential design decision, and it is
where chapter 3's forward reference is finally paid.** A bare slug that matched
nothing is retried as a reference ending in a key, and the peel is
`task_name::terminal_key` — which shares `peel_key` with the filename grammar —
and explicitly **not** `Handle::parse`. The comment gives the operational reason:
`Handle::parse` would also require the head to be a slug, and an operator pastes
a retired leaf's whole stem, `01-DONE-impl--build-k5`, meaning key 5. The deleted
`task_tree::handle_key` served that by ignoring everything before the key, and
this fallback preserves the behaviour rather than the function.

Chapter 3 read `Handle::parse`'s leniency on a zero-padded key — `a-k007` parses
as key 7 and normalises it away on the way back out — and argued it from a
precedent it named but could not yet show: *`parse_ref` is already lenient beside
it, taking a bare `007` for key 7.* This is that grammar, read at last. The
leniency is in the last branch of `parse_ref` below, and the two are consistent
because they are the same idea applied at two layers: a key written with leading
zeros is the same key, wherever it is written.

**The order of the two attempts is the part that has to be argued rather than
read.** The slug is tried first and the terminal key only when the slug matched
nothing, which means a real slug that happens to end in `-k<digits>` beats a
different entity that happens to hold that key. That is a deliberate precedence
and not an accident of control flow — it is pinned by a test built out of exactly
that collision, and the test is one of six in the block's second labelled
section. The alternative ordering would make an entry unnameable by its own slug
as soon as some unrelated entry took the matching key, which is a defect that
would appear only in trees large enough to collide.

<a id="a-path-first-then-a-reference"></a>
## A path first, then a reference

`resolve_in` is the read side. The same grammar has a second caller, and it is
the one the mutating verbs of chapter 10 go through.

<!-- fragment «resolution-reference» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="919-956" parent="resolution" -->
````rust
/// What a `<parent>` / `<target>` argument names in the tree: a path, or a
/// reference in the key/slug namespace.
///
/// **Clause 1, for the verbs whose argument is a reference rather than a path.**
/// The resolution runs against the snapshot the mutation will plan from, which
/// one guard already guarantees — where the pre-flip verbs resolved a reference
/// to a *path* under grove's own guard and then re-read the directory to act on
/// it.
///
/// Tried as a path first so an explicit, existing path always wins; only a
/// non-existent path is re-tried as a reference, so the two namespaces never
/// collide in practice. `.` is a path — the grove root — and so needs no case of
/// its own.
pub(crate) fn reference<'a>(
    root: &Path,
    snapshot: &'a Snapshot<TaskName>,
    argument: &str,
) -> Result<Target<'a>> {
    if let Some(path) = existing_path(root, argument) {
        return target(root, snapshot, &path);
    }
    match lookup(snapshot, argument)? {
        Lookup::Found(entry) => Ok(Target::Entry(entry)),
        Lookup::NotFound => bail!(
            "no entry matches {argument:?} (tried as a path under the grove root \
             and as a key/slug)"
        ),
        Lookup::Ambiguous(matches) => {
            let keys = matches
                .iter()
                .map(|entry| format!("[{}]", slug_match_key(entry)))
                .collect::<Vec<_>>()
                .join(", ");
            bail!("reference {argument:?} is ambiguous; re-query by key: {keys}")
        }
    }
}

````
<!-- /fragment -->

**This is clause 1 for the verbs whose argument is a reference rather than a
path**, and the doc comment states the property that makes it correct: the
resolution runs against the snapshot the mutation will then plan from. The
contrast it draws is with the shape grove used to have — the pre-flip verbs
resolved a reference to a *path* under grove's own guard and then re-read the
directory to act on it, which is a race with a name. Chapter 6 read `target`,
which this function delegates to; what `reference` adds is the second namespace
and the order the two are tried in.

**Tried as a path first, so an explicit existing path always wins.** Only a
non-existent path is re-tried as a reference, which is why the two namespaces
never collide in practice: a string that names a file that is really there is
never reinterpreted as a slug. The comment notes the consequence for `.` — it is
a path, the grove root, and so needs no case of its own here, unlike in
`resolve_in` where it is the first thing checked. The two callers of one grammar
differ in exactly that one place, and for a reason: `resolve_in` is answering a
question about the root, while `reference` is being handed something to mutate
under.

The two refusals are the operator-facing half of this function, and the second
of them is the ambiguity again, rendered for a human rather than for a caller.
`reference` cannot return `Ambiguous` — its result is a single `Target` — so
where `resolve_in` answers, `reference` refuses, and the refusal carries the
keys: *re-query by key: [2], [4]*. That is the same information in the same
order, turned from a value into a sentence, and it is why `slug_match_key` exists.

The path branch is a small function with three cases and no error.

<!-- fragment «resolution-existing-path» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="957-975" parent="resolution" -->
````rust
/// Interpret an argument as a path that actually exists: absolute, or relative
/// to the grove root, or relative to the cwd. `None` if no such path exists —
/// then it is a key/slug reference.
///
/// This preserves the *pass back what `pick`/`resolve` printed* ergonomics (the
/// absolute branch) and the worktree-relative convenience.
fn existing_path(grove_root: &Path, argument: &str) -> Option<PathBuf> {
    let candidate = Path::new(argument);
    if candidate.is_absolute() {
        return candidate.exists().then(|| candidate.to_path_buf());
    }
    let grove_relative = grove_root.join(candidate);
    if grove_relative.exists() {
        return Some(grove_relative);
    }
    let cwd_relative = std::env::current_dir().ok()?.join(candidate);
    cwd_relative.exists().then_some(cwd_relative)
}

````
<!-- /fragment -->

Absolute, then grove-root-relative, then cwd-relative — and `None` if none of
the three exists, which is the signal to try the key/slug namespace. The doc
comment names what each branch preserves: the absolute branch retains the *pass
back what `pick`/`resolve` printed* workflow, which is the loop's own habit,
and the grove-relative branch is the worktree convenience. The function tests
existence rather than shape, which is what makes the two namespaces disjoint by
construction instead of by grammar.

The reference grammar's own front door is two items, and they are the last of
the block's production.

<!-- fragment «resolution-ref-type» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="976-981" parent="resolution" -->
````rust
/// A parsed reference: a permanent key, or a bare slug.
pub(crate) enum Ref {
    Key(u32),
    Slug(String),
}

````
<!-- /fragment -->
`Ref` has two cases because the grammar has two branches, and the function that
produces it is where the bracket syntax is actually read.

<!-- fragment «resolution-parse-ref» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="982-1004" parent="resolution" -->
````rust
/// Classify a `resolve` reference. `[n]` / `[n]-slug` and a bare integer `n`
/// resolve by key; anything else is a bare slug. A bracketed-but-malformed key
/// (`[abc]`, `[4`) is a reference error, distinct from a valid-but-unmatched
/// reference, which is `NotFound`.
pub(crate) fn parse_ref(reference: &str) -> Result<Ref> {
    if let Some(rest) = reference.strip_prefix('[') {
        let close = rest
            .find(']')
            .with_context(|| format!("reference {reference:?}: unclosed '['"))?;
        let key: u32 = rest[..close]
            .parse()
            .with_context(|| format!("reference {reference:?}: '[…]' is not an integer key"))?;
        // Anything after `]` (e.g. `-slug`) is decorative.
        Ok(Ref::Key(key))
    } else if !reference.is_empty() && reference.bytes().all(|b| b.is_ascii_digit()) {
        let key: u32 = reference
            .parse()
            .with_context(|| format!("reference {reference:?}: not an integer key"))?;
        Ok(Ref::Key(key))
    } else {
        Ok(Ref::Slug(reference.to_string()))
    }
}
````
<!-- /fragment -->

**The three-way distinction in the doc comment is the one worth holding on to**,
because two of the three look alike from outside: a bracketed-but-malformed key
(`[abc]`, `[4`) is a *reference error*, distinct from a valid-but-unmatched
reference, which is `NotFound`. Grove refuses the first and answers the second.
The rule is that once a caller has committed to the bracket syntax, a thing that
cannot be a key is a mistake rather than a miss; without brackets there is
nothing to be wrong about, because anything that is not all-digits is simply a
slug.

That is why the middle branch tests `is_ascii_digit` over every byte rather than
attempting a parse and falling through on failure: `"12x"` must become the slug
`12x` and not a failed key. The `.parse()` that follows can then only fail on
overflow, and its context sentence says so in the same words as the bracketed
one.

**The leniency chapter 3 argued from is the plain `u32::parse` in both key
branches.** `007` parses as 7 because that is what parsing an integer does, and
nothing normalises it back — the key is the number, and the spelling was never
retained. Chapter 3 showed the same leniency in `Handle::parse` and had to argue
that it was deliberate rather than accidental, using this function as its
precedent; the precedent turns out to be one line of ordinary integer parsing in
two places, which is a weaker foundation than a rule written down, and it is
worth saying so. What makes it deliberate here is the test that pins `build-k005`
alongside `build-k5`.

The block closes on two functions that are not part of the grammar at all.

<!-- fragment «resolution-read-count» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1005-1015" parent="resolution" -->
````rust

#[cfg(test)]
pub(crate) fn reset_read_count() {
    READ_COUNT.with(|count| count.set(0));
}

#[cfg(test)]
pub(crate) fn read_count() -> usize {
    READ_COUNT.with(std::cell::Cell::get)
}

````
<!-- /fragment -->

`reset_read_count` and `read_count` are the accessors for the `READ_COUNT`
thread-local chapter 5 declared, and they are `#[cfg(test)]` — compiled only for
tests, and used by tests in three other chapters' blocks rather than in this one.
Chapter 7's `select_returns_path_handle_and_kind_from_one_guarded_observation`
is the assertion that most depends on them: it resets, calls, and requires the
count to be `1`. They sit at the end of chapter 9's block for no reason except
that they sit at the end of the file's production half, which is the clearest
small illustration in Part II of why the book's ownership is by concept and the
file's order is by convention.

<a id="twenty-one-tests"></a>
## Twenty-one tests, in two labelled sections

The chapter's second ownership block is lines 1653 to 1996 — 344 lines, 19%
comment prose, and twenty-one `#[test]` functions. It is the largest test block
in `task_tree.rs`, ahead of chapter 8's twenty-two tests over 292 lines and
chapter 7's nineteen over 255, though it is the least test-heavy of the three by
proportion: 344 of this chapter's 613 owned lines are tests, against chapter 8's
319 of 428 and chapter 7's 255 of 322.

**The source separates it into two labelled sections, and this chapter keeps the
separation.** Lines 1653 to 1874 are `// ---- resolve`, fifteen tests over the
grammar as a whole; lines 1875 to 1996 are
`` // ---- resolve: the full `<slug>-k<key>` handle (task-tree-scheme §5) ``, six
tests over the terminal-key fallback alone. The second label exists because that
fallback was added after the rest and is argued from a design record rather than
from the grammar's own shape; keeping the two apart is what lets the six be read
as one argument.

As in chapters 7 and 8, each test below is given twice over: **the property it
establishes, and what would have to be true for it to pass while that property
was broken.** For this block the second half has a recurring shape worth naming
in advance. Almost every test here asserts on a *path* — `name_of(&path)` — and a
path is the one thing that several different wrong implementations would agree
on. Where a test's force actually comes from somewhere else, that is said.

<!-- fragment «resolve-tests» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1653-1996" parent="source-task-tree" -->
<!-- insert «resolve-tests-fixture» -->
<!-- insert «resolve-tests-bracket-key» -->
<!-- insert «resolve-tests-bare-number» -->
<!-- insert «resolve-tests-pruned» -->
<!-- insert «resolve-tests-decorative-slug» -->
<!-- insert «resolve-tests-node-by-key» -->
<!-- insert «resolve-tests-key-not-found» -->
<!-- insert «resolve-tests-slug-unique» -->
<!-- insert «resolve-tests-slug-nested» -->
<!-- insert «resolve-tests-slug-not-found» -->
<!-- insert «resolve-tests-ambiguous» -->
<!-- insert «resolve-tests-root-brief» -->
<!-- insert «resolve-tests-dot» -->
<!-- insert «resolve-tests-empty-reference» -->
<!-- insert «resolve-tests-malformed-bracket» -->
<!-- insert «resolve-tests-absent-root» -->
<!-- insert «resolve-handle-tests-full-handle» -->
<!-- insert «resolve-handle-tests-terminal-key» -->
<!-- insert «resolve-handle-tests-disambiguates» -->
<!-- insert «resolve-handle-tests-node» -->
<!-- insert «resolve-handle-tests-precedence» -->
<!-- insert «resolve-handle-tests-unmatched» -->
<!-- /fragment -->

<a id="the-fixture-and-its-two-adds"></a>
### One fixture, and the collision it is built around

The section opens on a helper and a fixture rather than on a test, and the doc
comment on the helper is where the block's design is stated.

<!-- fragment «resolve-tests-fixture» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1653-1691" parent="resolve-tests" -->
````rust
    // ---- resolve ------------------------------------------------------------

    /// A nested tree with two `add` slugs (one live leaf, one retired leaf in a
    /// different subtree) so bare-slug `add` is ambiguous, plus a node directory
    /// and a unique `build` leaf — to exercise key, slug, node, and `DONE`.
    ///
    /// ```text
    /// .grove/
    ///   BRIEF.md
    ///   01-design-k1/         node
    ///     BRIEF.md
    ///     01-impl--add-k2.md        live leaf, slug "add"
    ///     02-impl--remove-k3.md
    ///   02-add-k4.DONE? -> 02-DONE-impl--add-k4.md   retired leaf, slug "add"
    ///   03-impl--build-k5.md
    /// ```
    /// `resolve` against a grove root, for the tests: the verb takes a tree
    /// already read, and every case below is about the answer rather than about
    /// the opening.
    fn resolve(grove_root: &Path, reference: &str) -> Result<Sought<Resolution>> {
        let tree = read(grove_root)?;
        resolve_in(
            &tree,
            &Reference::parse(reference).map_err(|error| anyhow!("{error}"))?,
        )
    }

    fn resolve_fixture() -> (TempDir, PathBuf) {
        let (tmp, g) = grove();
        touch(&g, "BRIEF.md");
        let design = mknode(&g, "01-design-k1");
        touch(&design, "BRIEF.md");
        touch(&design, "01-impl--add-k2.md");
        touch(&design, "02-impl--remove-k3.md");
        touch(&g, "02-DONE-impl--add-k4.md");
        touch(&g, "03-impl--build-k5.md");
        (tmp, g)
    }

````
<!-- /fragment -->

**The fixture is built around a deliberate collision**, and the doc comment's
ASCII tree says exactly what it is for: two `add` slugs — one live leaf nested
inside a node, one retired leaf at the root level in a different subtree — so
that bare-slug `add` is ambiguous; plus a node directory and a unique `build`
leaf, *to exercise key, slug, node, and `DONE`*. Four of the five grammar forms
and all three `Resolution` variants are reachable from this one tree, which is
why twelve of the fifteen tests in this section use it. The three that do not are
the pruned-leaf test, which needs an `ABANDONED` entry the fixture has no reason
to carry; the absent-root test, which needs no tree at all; and the
blank-reference test, which never opens one.

The `resolve` helper reads the tree and calls `resolve_in`, and its own comment
says why: the verb takes a tree already read, and every case below is about the
answer rather than about the opening. That is a scoping statement, and it is very
nearly true. Fourteen of the fifteen tests call the helper — the blank-reference
test is the exception, and never reaches a tree — and of those fourteen, thirteen
are about the answer. The fourteenth reaches past the helper's stated scope
entirely, and the chapter says so when it arrives.

Note what the helper does with a `Reference`: it parses one, and maps the
library-shaped error through `anyhow!`. `Reference::parse` is chapter 1's, in
`lib.rs`, and this is the only place in the block where a refusal comes from
outside `task_tree.rs` entirely.

<a id="four-spellings-of-a-key"></a>
### Four tests on the key branch

The first four tests take the key branch through its spellings. The opening one
sends a bracketed key to the deepest entry the fixture has.

<!-- fragment «resolve-tests-bracket-key» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1692-1704" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_by_bracket_key_finds_a_nested_leaf() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[2]").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "01-impl--add-k2.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**The property is that a bracketed key finds an entry wherever it sits**,
including inside a node directory, and that the answer carries the outcome. The
two path assertions pin both the filename and the parent directory, so a match
that found some other `add` would fail.

**What it would pass under with the property broken:** an implementation that
ignored the brackets entirely and treated `[2]` as a bare slug would fail here —
nothing has the slug `[2]` — so the bracket parse is genuinely exercised. But an
implementation that searched only the root level would also fail, and one that
searched only node directories would pass; the test does not distinguish
*recursive* from *node-only*. `resolve_bare_slug_unique_across_dirs` below, which
finds a root-level entry, is what closes that reading, and the pair is what
establishes recursion rather than either alone.

<!-- fragment «resolve-tests-bare-number» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1705-1716" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_by_bare_number_finds_a_done_leaf() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "4").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "02-DONE-impl--add-k4.md");
                assert_eq!(outcome, Outcome::Done, "the key-4 task is DONE");
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**The property is that a bare integer is the same reference as a bracketed one,
and that a retired leaf is still reachable.** The `outcome` assertion carries its
own message — *the key-4 task is `DONE`* — and it is the first of three places in
the block where `Located::outcome` is the thing under test rather than
incidental.

**What it would pass under with the property broken:** the path assertion alone
holds for an implementation that returned `Outcome::Live` unconditionally, since
the file it names is the right one either way. The second assertion is the whole
of the test's force, and it is worth noticing that the two assertions test
different things — the first that the *number* branch works, the second that the
`DONE` infix survives into the answer. A single test carrying two independent
claims is fine, but it means a failure here does not localise.

<!-- fragment «resolve-tests-pruned» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1717-1745" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_finds_a_pruned_leaf_by_key() {
        // pruning: an abandoned leaf's key must stay resolvable — durable
        // cross-references to it (commit messages, ADRs, briefs) are precisely
        // what the ADR protects. And `resolve` must not let the match *look*
        // live: `outcome` must come back `Abandoned`, not just the right path —
        // this is the exact failure mode pruning exists to prevent ("a
        // tree that hides its dead ends lies"), here in `resolve` rather than
        // the tree itself.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "01-ABANDONED-impl--spike-k1.md");
        match resolve(&g, "[1]").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "01-ABANDONED-impl--spike-k1.md");
                assert_eq!(outcome, Outcome::Abandoned);
            }
            other => panic!("expected one entry, got {other:?}"),
        }
        // The full `<slug>-k<key>` handle resolves it too.
        match resolve(&g, "spike-k1").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "01-ABANDONED-impl--spike-k1.md");
                assert_eq!(outcome, Outcome::Abandoned);
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**This is the test the `outcome` field was added for**, and its comment says so
in the field's own terms: an abandoned leaf's key must stay resolvable, because
durable cross-references to it — commit messages, ADRs, briefs — are what the
entries-are-never-removed decision protects; and `resolve` must not let the match
*look* live. The comment names the failure mode exactly: *a tree that hides its
dead ends lies*, here in `resolve` rather than in the tree itself.

**What it would pass under with the property broken:** nothing much, and this is
the strongest test in the section. The two assertions are the path and the
outcome, and the outcome assertion cannot be satisfied by any implementation that
drops the `ABANDONED` infix on the way through. It is also the only test in the
block that exercises a *pruned* leaf at all.

The test's second half is doing something the section label does not advertise:
it resolves `spike-k1`, the full handle, twenty-five lines before the block's
second labelled section begins. So the handle fallback is exercised inside the
first section too, on a tree the second section never builds. That is not a
defect — the comment introduces it as *the full `<slug>-k<key>` handle resolves it
too* — but a reader counting handle tests from the section labels alone will
count six and the true number is seven.

<!-- fragment «resolve-tests-decorative-slug» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1746-1757" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_bracket_key_ignores_decorative_slug() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[5]-whatever").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "03-impl--build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**The property is that everything after `]` is decorative**, which is the third
bullet of `resolve_in`'s grammar. `[5]-whatever` finds key 5 and the suffix is
discarded without being examined.

**What it would pass under with the property broken:** an implementation that
parsed `[5]-whatever` by taking everything before the first `-` would also pass,
as would one that split on `]` and ignored the tail. Both are in fact what the
code does; what the test does not reach is any suffix that could be mistaken for
a second reference — `[5]-build`, where the decorative slug names a *different*
entity's slug, is not tested, and would be the case that distinguishes *ignored*
from *ignored unless it matches something*. Reading `parse_ref` settles that it is
truly ignored, but the block does not.

<a id="a-node-is-an-entry"></a>
### A node, an absence, and the slug branch

A node is an entry too, and resolving one is where `Located::kind` earns its
`Option`.

<!-- fragment «resolve-tests-node-by-key» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1758-1772" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_key_resolves_a_node_to_its_directory() {
        // A node's identity rides in its directory name, so a key reference to a
        // node resolves to the directory path (append /BRIEF.md to read it).
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[1]").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "01-design-k1");
                assert!(path.is_dir());
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**The property is that a node resolves to its directory**, which is what makes
`Located::kind` optional. The comment gives the operator consequence — append
`/BRIEF.md` to read it — and `assert!(path.is_dir())` is what separates this from
a leaf.

**What it would pass under with the property broken:** the test never inspects
`kind`, so an implementation that reported `Some(design)` for the node — reading
the slug as a kind, say — would pass all three assertions. The claim that a node's
`kind` is `None` is made by the `Located` doc comment and by `located`'s match on
`Parts`, and it is **not** pinned anywhere in this block. That is the clearest
coverage gap in the chapter, and `resolve_handle_of_a_node_resolves_to_its_directory`
in the second section repeats the same omission.

<!-- fragment «resolve-tests-key-not-found» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1773-1778" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_key_not_found() {
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "[99]").unwrap(), Sought::Nothing);
    }

````
<!-- /fragment -->

**The property is that an unmatched key is `Sought::Nothing` rather than an
error** — a miss is an answer. One line, and it is the whole of the claim.

**What it would pass under with the property broken:** an implementation that
returned `Nothing` for *every* bracketed key would pass this test; the four tests
above are what rule that out. This is the clearest case in the block of a test
whose force is entirely borrowed from its siblings.

<!-- fragment «resolve-tests-slug-unique» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1779-1790" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_bare_slug_unique_across_dirs() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "build").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "03-impl--build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->
Its twin moves the unique match inside the node directory, so that between them
the two cover both levels of the fixture.

<!-- fragment «resolve-tests-slug-nested» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1791-1804" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_bare_slug_resolves_a_nested_unique_leaf() {
        // `remove` lives only inside the node directory — slug search recurses.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "remove").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "02-impl--remove-k3.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**Together these two establish that slug search is recursive and finds exactly
one thing when only one thing matches.** The first finds `build` at the root
level; the second finds `remove`, which — as its comment says — lives only inside
the node directory. Neither alone would do: the first is satisfied by a
root-only search, the second by a node-only one, and it is the pair that forces
a walk of the whole tree. The second also pins the parent directory, so a match
that resolved to the right filename in the wrong place would fail.

**What they would pass under with the property broken:** an implementation that
walked the tree but compared the *whole filename* rather than the slug would fail
both, since neither reference carries a position or a kind. One that compared a
suffix of the filename would pass both — `01-impl--build-k5.md` does not end in
`build`, so in fact it would not; but one comparing the slug *prefix* would.
Neither test carries a near-miss slug, so *equality* rather than *prefix* is
established only by `resolve_prefers_a_real_slug_over_the_handle_fallback` in the
second section, which is the one test in the block built out of two slugs that
overlap.

<!-- fragment «resolve-tests-slug-not-found» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1805-1810" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_bare_slug_not_found() {
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "nope").unwrap(), Sought::Nothing);
    }

````
<!-- /fragment -->

**The property is that an unmatched slug is `Nothing` and not an error**, the
slug-branch twin of `resolve_key_not_found`. It is also, quietly, the only test
that exercises the zero-match arm of the slug branch on a reference that ends in
no key at all — the arm that calls `terminal_key` and gets `None`. The second
section's `resolve_reads_a_terminal_key_whatever_precedes_it` pins that arm
deliberately with three references; this one reaches it by accident.

<a id="the-ambiguous-arm"></a>
### The answer the library has no word for

One test in the block observes the variant the library has no counterpart for,
and it is the case the fixture's two `add` leaves were built to produce.

<!-- fragment «resolve-tests-ambiguous» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1811-1832" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_bare_slug_ambiguous_lists_every_match_by_key() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "add").unwrap() {
            Sought::Match(Resolution::Ambiguous(matches)) => {
                // Pre-order: the nested `01-design/01-add-k2` precedes the
                // root-level `02-DONE-add-k4`.
                assert_eq!(matches.len(), 2);
                // The key comes back inside the handle now: it is the handle
                // that a caller re-queries with, and the key alone was never
                // enough to name the entry back.
                assert_eq!(matches[0].handle.to_string(), "add-k2");
                assert_eq!(name_of(&matches[0].path), "01-impl--add-k2.md");
                assert_eq!(matches[0].outcome, Outcome::Live);
                assert_eq!(matches[1].handle.to_string(), "add-k4");
                assert_eq!(name_of(&matches[1].path), "02-DONE-impl--add-k4.md");
                assert_eq!(matches[1].outcome, Outcome::Done);
            }
            other => panic!("expected an ambiguous match, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**This is the chapter's central test**, and it is the only one that observes an
`Ambiguous`. The property is threefold: that a bare slug matching twice answers
rather than refuses, that both matches come back, and that they come back **in
pre-order** — the nested `01-design/01-add-k2` before the root-level
`02-DONE-add-k4`, as the comment says. The assertions pin each match's handle,
path and outcome in turn.

The comment on the handle assertions records a change and its reason: *the key
comes back inside the handle now — it is the handle that a caller re-queries
with, and the key alone was never enough to name the entry back.* That is the
`Located::handle` field's justification, observed rather than asserted in the
abstract, and it is the one place in the block where the shape of the answer is
tested rather than its content.

**What it would pass under with the property broken:** an implementation that
collected matches in *file-system* order rather than pre-order would pass on many
platforms and fail on some, because `01-design-k1` sorts before `02-DONE-…`
either way; the ordering claim is therefore weaker than it looks, and the test
that would distinguish walk order from name order is chapter 7's
`pick_orders_numerically_not_lexically`, not this one. More sharply: an
implementation that returned `Ambiguous` for *every* slug match, including the
unique ones, would fail `resolve_bare_slug_unique_across_dirs` but pass here — so
the boundary between one match and two is held by that test rather than by this
one. The `matches.len()` assertion establishes *two*, not *exactly the two that
match*; that is established by the four assertions that follow it, which are
specific enough to leave no room.

<a id="the-root-and-its-brief"></a>
### Two things that are not entries

Two references name things the grammar deliberately cannot reach, and each takes
a test of its own. The first is the root brief.

<!-- fragment «resolve-tests-root-brief» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1833-1841" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_root_brief_is_unreferenceable() {
        let (_t, g) = resolve_fixture();
        // It carries no key and no slug, so nothing spells it. Its own filename
        // does not either.
        assert_eq!(resolve(&g, "BRIEF").unwrap(), Sought::Nothing);
        assert_eq!(resolve(&g, "BRIEF.md").unwrap(), Sought::Nothing);
    }

````
<!-- /fragment -->
The second is `.`, which does reach an answer — just not an entry.

<!-- fragment «resolve-tests-dot» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1842-1851" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_dot_is_the_grove_root_itself() {
        let (_t, g) = resolve_fixture();
        assert_eq!(
            resolve(&g, ".").unwrap(),
            Sought::Match(Resolution::Root),
            "`.` is the root, and the root is not an entry"
        );
    }

````
<!-- /fragment -->

**The pair establishes the two ends of *not an entry*.** The root brief carries
no key and no slug, so nothing spells it — and, as the comment adds, *its own
filename does not either*, which is why both `BRIEF` and `BRIEF.md` are tried.
`.` is the root itself, and answers `Resolution::Root`, with the assertion
message carrying the claim: *`.` is the root, and the root is not an entry*.

**What they would pass under with the property broken:** `resolve_root_brief_is_unreferenceable`
would pass for an implementation that refused *every* reference containing an
uppercase letter, or one that never matched anything ending in `.md`. Neither is
excluded here — but `resolve_reads_a_terminal_key_whatever_precedes_it` in the
second section resolves `Build-k5` and `02-DONE-impl--add-k4`, which rules out
both. The two tests are two sections apart and neither mentions the other.

The `.` test is the stronger of the pair: `Resolution::Root` is a variant nothing
else in the block produces, so no wrong implementation reaches it by accident.
What it does not establish is that `.` is checked *before* the tree is consulted
— `resolve_in` returns `Root` from `reference.is_root()` without touching the
snapshot — and a version that walked the tree first and special-cased a miss on
`.` would be indistinguishable here.

<a id="refusals-from-elsewhere"></a>
### Three refusals, two of which are not this chapter's

The section closes on three refusals. The first of them never reaches this
chapter's code at all.

<!-- fragment «resolve-tests-empty-reference» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1852-1856" parent="resolve-tests" -->
````rust
    #[test]
    fn an_empty_reference_names_nothing_and_is_refused_before_the_tree() {
        assert!(Reference::parse("   ").is_err());
    }

````
<!-- /fragment -->

**This test never reaches chapter 9's code at all.** `Reference::parse("   ")`
refuses a blank reference in `lib.rs` — chapter 1's block — and the test's own
name says where: *refused before the tree*. It sits in the resolve block because
that is where the grammar is documented, but the clause it holds is chapter 1's,
and the honest description is that it is chapter 1's refusal observed through
chapter 9's section label. The parent brief's rule applies exactly: a refusal a
verb surfaces was usually not produced by that verb.

**What it would pass under with the property broken:** anything that made
`Reference::parse` reject whitespace, including a change with no relation to
resolution.

<!-- fragment «resolve-tests-malformed-bracket» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1857-1863" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_malformed_bracket_ref_errors() {
        let (_t, g) = resolve_fixture();
        assert!(resolve(&g, "[abc]").is_err());
        assert!(resolve(&g, "[4").is_err());
    }

````
<!-- /fragment -->

**This one is chapter 9's, and it is the only test in the block that holds a
`parse_ref` refusal.** Two assertions, and they land on two *different* clauses:
`[4` has no closing bracket and fails at `unclosed '['`; `[abc]` closes but does
not parse, and fails at *`'[…]'` is not an integer key*. Neither assertion
inspects the message, so the test establishes only that both are errors rather
than misses — which is, to be fair, the distinction the doc comment draws.

**What it would pass under with the property broken:** an implementation that
returned an error for every reference beginning with `[` would pass, and
`resolve_by_bracket_key_finds_a_nested_leaf` is what rules it out. More to the
point, an implementation that swapped the two messages would pass unchanged,
since neither is read. The measurement below is what settles which clauses this
test actually holds.

<!-- fragment «resolve-tests-absent-root» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1864-1874" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = resolve(&missing, "[1]").unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

````
<!-- /fragment -->

**The third refusal is chapter 5's**, and this is the one test in the section
that reaches past the helper's stated scope. `resolve` calls `read(grove_root)`
before `resolve_in`, so a missing root fails during the *opening* and the
resolution never runs. The message it asserts on, *grove root not found*, is
produced in chapter 5's block — at `absent_tree` for the vacancy shape and again
inside `restate` for a failed read — and nothing in chapter 9's 613 lines can
produce it.

That makes three of the section's fifteen tests refusal tests, and only one of
the three refuses inside this chapter's code. It also makes the helper's comment
— *every case below is about the answer rather than about the opening* — true of
thirteen of the fourteen that call it, and false of this one. `task_tree.rs`
asserts that same substring four times over: once in chapter 7's `pick` tests,
**twice** in chapter 8's — the brief-chain block and the kind block each carry
their own — and once here. Pinning it per verb is reasonable; treating any one of
the four as evidence about the verb it sits under is not.

<a id="the-full-handle"></a>
### Six tests on the fallback, under their own label

The source gives the terminal-key fallback its own section, and cites a design
record for it: `task-tree-scheme` §5, which is where `<slug>-k<key>` is the
canonical commit and prose handle. The six tests below are the argument that
`resolve` accepts that spelling, and — more carefully — that it accepts it
*without* accepting a handle-shaped grammar it never promised.

<!-- fragment «resolve-handle-tests-full-handle» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1875-1891" parent="resolve-tests" -->
````rust
    // ---- resolve: the full `<slug>-k<key>` handle (task-tree-scheme §5) --------------

    #[test]
    fn resolve_by_full_slug_handle_finds_by_terminal_key() {
        // §5's canonical commit/prose handle is `<slug>-k<key>`; resolve accepts it
        // directly — the terminal `-k<key>` is read as the key, the slug decorative
        // — so the handle round-trips back to a path.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "build-k5").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, outcome, .. })) => {
                assert_eq!(name_of(&path), "03-impl--build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**The property is the round trip**: the handle the tree prints is a reference the
tree accepts, and it comes back to the path it names. The comment restates the
mechanism — the terminal `-k<key>` is read as the key, the slug decorative — which
is where a careless reader would stop.

**What it would pass under with the property broken:** everything. `build` is
unique in the fixture, so an implementation that ignored the `-k5` entirely and
matched on the slug alone would pass this test unchanged. The bare slug `build`
is separately tested nine tests earlier and resolves to the same file. Taken by
itself this test establishes nothing about the fallback at all; its force comes
entirely from `resolve_full_handle_disambiguates_what_a_bare_slug_could_not`
below, where the slug is *not* unique and the key is the only thing that could
have chosen. That is the clearest instance in this chapter of a test whose name
promises more than its fixture can deliver.

The next test is the section's argument, and it carries the longest doc comment
in the block.

<!-- fragment «resolve-handle-tests-terminal-key» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1892-1940" parent="resolve-tests" -->
````rust
    /// The fallback reads a **terminal key**, not a handle, and nothing before
    /// that key has to be a slug.
    ///
    /// The realistic case is the first: an operator pastes a retired leaf's
    /// whole filename stem, which carries a position and a `DONE` infix and is
    /// therefore not a slug at all. `name-ownership-k14` briefly routed this
    /// through `Handle::parse` and lost every row below — a change to what
    /// `resolve` accepts, smuggled in by a refactor whose subject was who owns
    /// the grammar. Pinned so the next such routing has to be deliberate.
    #[test]
    fn resolve_reads_a_terminal_key_whatever_precedes_it() {
        let (_t, g) = resolve_fixture();
        for reference in [
            // A retired leaf's stem, as it literally appears on disk.
            "02-DONE-impl--add-k4",
            // A live leaf's stem.
            "03-impl--build-k5",
            // Shapes no slug may take: uppercase, an underscore, a reserved
            // word, an empty head. All of them still end in a key.
            "Build-k5",
            "a_b-k5",
            "DONE-k5",
            "-k5",
            // A lenient key spelling, as `parse_ref` already accepts for a bare
            // integer.
            "build-k005",
        ] {
            match resolve(&g, reference).unwrap() {
                Sought::Match(Resolution::Entry(Located { path, .. })) => assert_eq!(
                    name_of(&path),
                    if reference.contains("-k4") {
                        "02-DONE-impl--add-k4.md"
                    } else {
                        "03-impl--build-k5.md"
                    },
                    "{reference:?}"
                ),
                other => panic!("{reference:?}: expected one entry, got {other:?}"),
            }
        }
        // A reference ending in no key at all is still simply unmatched.
        for reference in ["nothing", "nothing-k", "nothing-kx"] {
            assert!(
                matches!(resolve(&g, reference).unwrap(), Sought::Nothing),
                "{reference:?} should not resolve"
            );
        }
    }

````
<!-- /fragment -->

**The property is that the fallback reads a terminal key and not a handle**, and
that nothing before that key has to be a slug. The doc comment does two things
this book's third obligation would otherwise have to do from outside. It names
the realistic case — an operator pastes a retired leaf's whole filename stem,
which carries a position and a `DONE` infix and is therefore not a slug at all —
and it records the history: `name-ownership-k14` briefly routed this through
`Handle::parse` and lost every row below, *a change to what `resolve` accepts,
smuggled in by a refactor whose subject was who owns the grammar*. The test is
pinned so the next such routing has to be deliberate.

The table of references is the coverage, and it is unusually well chosen: two
real filename stems, four shapes no slug may take — uppercase `Build-k5`, an
underscore `a_b-k5`, the reserved word `DONE-k5`, an empty head `-k5` — and the
zero-padded `build-k005`. **Six of the seven are references `Handle::parse`
refuses** — the two filename stems for the `--` separator between kind and slug,
and the four shape rows for an uppercase letter, an underscore, a reserved word
and an empty head — so the comment's *lost every row below* is very nearly
literal: `build-k005` is the only one of the seven that is also a well-formed
handle. That is what makes the table a regression pin rather than a
demonstration. It is also why the last row does double duty, since it is the row
where chapter 3's leniency precedent is pinned by an assertion rather than argued
from parsing behaviour.

**What it would pass under with the property broken:** the seven positive rows
would all pass under an implementation that read a terminal key with *no* slug
attempt first — the wrong precedence, in other words — because none of the seven
is also a live slug. That reading is closed by
`resolve_prefers_a_real_slug_over_the_handle_fallback` two tests below, and by
nothing else in the crate. The three negative rows — `nothing`, `nothing-k`,
`nothing-kx` — establish that a reference ending in no key is unmatched rather
than an error, which is the arm of the slug branch that calls `terminal_key` and
gets `None`.

They look like three different shapes and they are not. `peel_key`, the one peel
grove has, first takes the trailing run of ASCII digits and returns `None` when
there is none; only then does it require `-k` in front of that run. All three
rows end in a non-digit, so all three stop at the first clause, and the second —
digits present but no `-k` marker, which is what `nothing5` would be — is
exercised by nothing in this block. The trio is one case written three ways.

<!-- fragment «resolve-handle-tests-disambiguates» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1941-1954" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_full_handle_disambiguates_what_a_bare_slug_could_not() {
        // The bare slug `add` is ambiguous (two matches); the full handle `add-k2`
        // names exactly the nested live leaf via its key.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "add-k2").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, .. })) => {
                assert_eq!(name_of(&path), "01-impl--add-k2.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**This is the test that gives the section its point.** The bare slug `add` is
ambiguous — the fixture is built for it — and the full handle `add-k2` names
exactly the nested live leaf. The two path assertions pin the filename and the
parent, so the *other* `add` would fail it.

**What it would pass under with the property broken:** an implementation that
resolved `add-k2` by taking the text before the last `-k` and then, on finding
two matches, returning the first in pre-order would pass — the nested `add-k2` is
first. That is not a fanciful alternative: it is precisely what a `seek` over the
slug branch would do, and it is the implementation the whole chapter argues
against. Nothing in this test distinguishes *resolved by key 2* from *returned
the first of two `add`s*. What does distinguish them is
`resolve_bare_slug_ambiguous_lists_every_match_by_key`, which shows that the bare
slug does **not** short-circuit — so the handle's answer cannot be the
short-circuit either. The pair is the argument; neither half is.

<!-- fragment «resolve-handle-tests-node» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1955-1967" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_handle_of_a_node_resolves_to_its_directory() {
        // A node's handle (`design-k1`) resolves to the node directory, like its key.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "design-k1").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, .. })) => {
                assert_eq!(name_of(&path), "01-design-k1");
                assert!(path.is_dir());
            }
            other => panic!("expected one entry, got {other:?}"),
        }
    }

````
<!-- /fragment -->

**The property is that a node's handle behaves like its key**, resolving to the
directory. It repeats `resolve_key_resolves_a_node_to_its_directory` through the
other spelling, and it repeats its omission too: `kind` is never inspected, so
nothing here or there pins the claim that a node's kind is `None`.

**What it would pass under with the property broken:** the same alternatives as
its key-branch twin, plus one more — since `design` is a unique slug in the
fixture, an implementation ignoring the `-k1` would pass.

<!-- fragment «resolve-handle-tests-precedence» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1968-1988" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_prefers_a_real_slug_over_the_handle_fallback() {
        // A bare slug that itself ends in `-k<digits>` resolves as a slug first; the
        // key fallback fires only when the slug match is empty. So a slug `foo-k5`
        // (key 7) wins over a *different* entity that happens to hold key 5.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "01-impl--foo-k5-k7.md"); // slug "foo-k5", key 7
        touch(&g, "02-impl--other-k5.md"); // slug "other", key 5
        match resolve(&g, "foo-k5").unwrap() {
            Sought::Match(Resolution::Entry(Located { path, .. })) => {
                assert_eq!(
                    name_of(&path),
                    "01-impl--foo-k5-k7.md",
                    "the real slug match must win over the key-5 handle fallback"
                );
            }
            other => panic!("expected Found (slug), got {other:?}"),
        }
    }

````
<!-- /fragment -->

**This is the only test in the block that establishes precedence**, and it is
built out of a collision that has to be constructed on purpose: a leaf whose slug
is literally `foo-k5` holding key 7, beside a different leaf holding key 5. The
comment states the rule and the consequence — a bare slug that itself ends in
`-k<digits>` resolves as a slug first, and the key fallback fires only when the
slug match is empty — and the assertion carries its own message.

This is the test that makes the ordering inside `lookup`'s slug branch a claim
rather than an implementation detail, and it is the one the production half
pointed forward to. It is also the only test in this second section that builds
its own tree, because the collision it needs cannot exist in a tree built for the
ambiguity case; three tests in the first section build their own too, each for a
reason just as specific.

**What it would pass under with the property broken:** very little. An
implementation that tried the terminal key first would return
`02-impl--other-k5.md` and fail. One that tried the slug first but fell back on
*any* result rather than on an empty one would also fail. The narrow gap it
leaves is that the fixture's two entries differ in both slug and key, so an
implementation that preferred the *longer* match, or the lower position, would
also pass — but neither is a plausible reading of the code, and this is as close
to airtight as the block gets.

<!-- fragment «resolve-handle-tests-unmatched» owner="wider-than-a-key" source="crates/grove-loop/src/task_tree.rs" lines="1989-1996" parent="resolve-tests" -->
````rust
    #[test]
    fn resolve_handle_shaped_but_unmatched_is_not_found() {
        // A handle whose key matches nothing is NotFound (not an error), like any
        // unmatched reference.
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "ghost-k99").unwrap(), Sought::Nothing);
    }

````
<!-- /fragment -->

**The property closes the section symmetrically with how the first one opened**:
a handle whose key matches nothing is `NotFound` and not an error, *like any
unmatched reference*. `ghost` is not a slug in the fixture and `99` is not a key,
so both halves of the grammar miss and the answer is `Nothing`.

**What it would pass under with the property broken:** an implementation that
answered `Nothing` for every reference containing a `-k` would pass, and the five
tests above rule it out. Like `resolve_key_not_found` and
`resolve_bare_slug_not_found` before it, this test's job is to hold the
*miss-is-an-answer* line at one more spelling, and its force is the block's rather
than its own.

<a id="what-the-refusals-are-worth"></a>
## What the refusals are worth, measured

`resolve_in`, `lookup`, `reference`, `located` and `parse_ref` are the shape
chapter 8 met in `leaf_entry`: private resolvers with several refusal arms, whose
tests assert on outcomes rather than on which clause produced them. Chapter 8
settled that question by mutation and found five of seven refusals unobserved,
and the parent brief's instruction to this chapter is that a coverage sentence
about a function of the same shape is worth exactly the same re-run. Reading the
tests will not answer it: `resolve_malformed_bracket_ref_errors` asserts
`is_err()` twice and never looks at a message, so from the source alone it is
impossible to say which of `parse_ref`'s three clauses it holds.

**The block carries seven refusal arms.** One is settled without a run: the
`unreachable!` in `slug_match_key` is already a panic, so a green suite is itself
proof that no test reaches it. The other six were replaced one at a time, in a
copy of the workspace, with a panic carrying a sentinel, and the whole of
`grove-loop` and `grove-llm` was run against each — 245 inline tests and thirty
test targets. The result is the failure set attributable to each arm. **Arms 3
and 4 were measured a second time**, with `grove` added to the run and every
failure set diffed against an unmutated control run of the same copy; their rows
below are that second run's, and the paragraph after the table says why it was
needed.

| # | Line | The refusal | Tests that fail when it is replaced |
|---:|---:|---|---|
| 1 | 792 | `located`: *a resolved reference matched the root brief, which carries no identity* | **none** |
| 2 | 867 | `slug_match_key`: `unreachable!("a slug match is positioned")` | **none** |
| 3 | 942 | `reference`: *no entry matches … (tried as a path … and as a key/slug)* | `add_refuses_a_parent_that_names_nothing_in_the_tree`, `insert_errors_when_target_missing`, `add_under_nonexistent_parent_errors`, `insert_requires_an_existing_target` |
| 4 | 952 | `reference`: *is ambiguous; re-query by key: …* | `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys` |
| 5 | 990 | `parse_ref`: *unclosed `'['`* | `resolve_malformed_bracket_ref_errors` |
| 6 | 993 | `parse_ref`: *`'[…]'` is not an integer key* | `resolve_malformed_bracket_ref_errors` |
| 7 | 999 | `parse_ref`: *not an integer key* | **none** |

Each covered arm was mutated separately, and each produced a small, distinct
failure set — which is what attributes the failures rather than merely counting
them, and what rules out a second silent observer standing behind the first.

**Three of the seven are unobserved, and the three are unobserved for three
different reasons.** Arm 1 is unreachable rather than untested: the root brief
cannot come back from either branch of `lookup`, because `by_key` cannot return
an entry with no key and the slug filter requires a `triple` the root brief does
not have. `resolve_root_brief_is_unreferenceable` establishes the *behaviour*
without going anywhere near the clause that would state it, which is a fair
description of a well-guarded invariant rather than a gap. Arm 2 is the same
story with a panic instead of a refusal, and its doc comment already argues why.
Arm 7 is different: a bare all-digit reference that overflows `u32` —
`99999999999` — reaches it, and nothing in the crate ever writes one. That is a
genuine hole, and a small one.

**The more interesting result is where the covered arms are covered from**, and
the two `reference` arms are not covered from the same place. Arm 4 is held by
exactly one test, `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys`, in
`crates/grove-loop/src/task_grow/tests.rs` — chapter 10's block, and the book's
one declared corpus exclusion at that, which this book cites by name and never
reproduces. Arm 3 is held by four, and only two of them are in that file:
`add_refuses_a_parent_that_names_nothing_in_the_tree` (line 413) and
`insert_errors_when_target_missing` (line 1,159). The other two,
`add_under_nonexistent_parent_errors` and `insert_requires_an_existing_target`,
are in `crates/grove-llm/tests/leaf.rs` at lines 432 and 526 — an integration
target of a **different crate**, outside this book's corpus altogether. So
chapter 10 owns the whole of arm 4's evidence and half of arm 3's, and the rest
of arm 3's is in a crate this book does not document. Arms 5 and 6 are held by
one test between them, and it is chapter 9's own; but because that test reads
no messages, an implementation that swapped the two clauses' wording would pass
it unchanged.

**This table's arm 3 row was wrong the first time, and the way it was wrong is
worth more than the correction.** The row named only the two `grove-llm` tests,
and the sentence that followed it placed *both* `reference` arms in `task_grow`'s
tests on the strength of the arm 4 row beside it. The row was rebuilt by
re-running the mutation rather than by re-reading the page — each arm replaced
with a panicking sentinel, `grove-loop`, `grove-llm` and `grove` run against
each, and every failure set diffed against an unmutated control run of the same
copy — and that is what surfaced the two observers in the excluded file. What
made the first reading look complete is that it started from the names the row
already carried: locating those names says where *those* tests live and nothing
about whether they are the whole set, and no amount of care about the two will
produce the third. Only the mutation enumerates. Note also that the mutation has
to be a **panic**, not a reworded `bail!`: three of arm 3's four observers assert
on the substring *no entry matches* and a rewording would catch them, but
`insert_errors_when_target_missing` asserts a bare `is_err()` and stays green
under any message at all. **A list of the tests that hold a clause is a
measurement, and it is worth exactly the re-run.**

So the accurate sentence about this block is not *four of seven refusals are
covered*. It is that **one test in this chapter holds two clauses without
distinguishing them, two clauses are held entirely from outside this chapter —
one of them partly from outside the book's corpus as well — and three are held
by nothing** — and that the reason to know this is not to add tests to a
frozen corpus, but so that chapter 21 can answer *what did not go* with a measured
answer rather than a plausible one.

<a id="what-resolution-kept"></a>
## What resolution kept, and the file it completes

The chapter's answer to *what did not go, and why could it not?* is the
narrowest of Part II's six, and the easiest to state. The library can find an
entry by key, and it can walk with a predicate the consumer supplies. It cannot
be told what a slug is, because its trait names no label type — so there is
nothing for a `by_label` to take. Everything in this chapter follows from that
one absence: the grammar, the walk, the three-way outcome, and the cost.

**The cost is the walk, and it is paid for the outcome rather than for the
match.** A `seek` would find *an* entry with the slug more cheaply, and grove
declines it, because the question `resolve` answers is not *is there one?* but
*how many, and which?* That is the clearest example in the book of grove paying
for meaning: the mechanism was available and was rejected because it answers a
different question. The `Ambiguous` variant is the shape of the answer the
library has no counterpart for, and `Located::handle` is what makes it
actionable — a caller that gets two matches gets two handles, and re-asks with
one of them.

**Chapter 3's forward reference is closed here.** That chapter argued
`Handle::parse`'s leniency on `a-k007` from a precedent it named without reading:
`parse_ref` takes a bare `007` for key 7. The precedent is real, and it turns out
to be one line of ordinary `u32` parsing in each of two branches rather than a
rule written down anywhere — which is a thinner foundation than the argument
implied, and worth knowing. What makes the leniency deliberate rather than
incidental is the `build-k005` row in
`resolve_reads_a_terminal_key_whatever_precedes_it`, and that row is in this
chapter's block.

**The block also settles what the fallback is not.** It reads a terminal key,
not a handle: nothing before the `-k<key>` has to be a slug, and the deleted
`task_tree::handle_key` is named in the comment as the behaviour being preserved.
The four shapes no slug may take — `Build-k5`, `a_b-k5`, `DONE-k5`, `-k5` — are
exactly what `Handle::parse` would refuse, and they are in the test because a
refactor once routed the fallback through `Handle::parse` and silently narrowed
what `resolve` accepts. The grammar chapter 4 made canonical is the *filename*
grammar; the reference grammar is deliberately wider and deliberately lenient,
and the two meet only at `peel_key`, which `terminal_key` shares with the
filename side.

**One thing this block does not pin, and it is worth naming.** `Located::kind` is
`None` for a node directory, and that is stated in the field's doc comment and
implemented in `located`'s match on `Parts`. Two tests resolve a node — once by
key, once by handle — and neither inspects `kind`. The claim is true, and it is
held by the type rather than by the suite.

**With this chapter, `task_tree.rs` is whole.** The file's ten ownership blocks
are complete across five chapters: opening at 1–290 and paths at 291–570 in
chapters 5 and 6, the walk at 571–637 and kind and briefs at 638–746 in chapters
7 and 8, resolution at 747–1015 here; then the test module's six labelled
sections and its closing composition — path compositions at 1016–1105, `pick` at
1106–1360, brief chain and kind at 1361–1652, `resolve` at 1653–1996, and
`pick + brief-chain together` at 1997–2023. All 2,023 lines of it are now
reproduced and explained. Four of the crate's thirteen roots are split across
chapters, and this is the one split the furthest — five ways, against
`tree_lifecycle.rs`'s four, `task_name.rs`'s three and `driver_lease.rs`'s two —
which is why it took five chapters to close and why each of its five production
concerns has now been read against the tests that establish it.

Chapter 10 leaves the reading surface entirely. `task_grow.rs` is 518 lines of
growing the tree — `leaf-add` and `leaf-insert` — and it is the one chapter in
this book whose proof lies wholly outside its own pages: `src/task_grow/tests.rs`
is the book's single declared corpus exclusion, 1,680 lines of evidence cited by
name and never reproduced. Two of this chapter's own refusals are pinned there,
which is a preview of the problem chapter 10 has to state rather than solve.

[Previous: Kind, and the brief chain](08-kind-and-briefs.md) | [Contents](README.md) | [Next: Growing: leaf-add and leaf-insert](10-growing.md)

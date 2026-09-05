# Kind, and the brief chain
<!-- book-page id="kind-and-briefs" slice="root-to-leaf" order="8" -->
[Previous: The walk: pick and select](07-the-walk.md) | [Contents](README.md)

<a id="root-to-leaf"></a>
## The rule: ancestor briefs, root to leaf, and a brief is not a leaf

Chapter 7 answered *what should be worked next* and stopped there. A session
that has been handed a leaf still needs two things before it can begin, and both
are questions about the same snapshot the selection came from: **what kind of
session does this leaf want, and what has already been decided above it?**

> The kind is the token in the filename, never anything in the body. The context
> is the `BRIEF.md` of every ancestor level, root to leaf — and a brief is not a
> leaf, so it is never the thing the argument may name.

That is grove's answer to *what did not go, and why could it not?* asked of the
two reading verbs the module header named beside `pick`. It is also the one
chapter in Part II where the honest answer is that **almost nothing stayed**.
`brief_chain`'s body is four lines and its own doc comment says why: the
operation already existed in the library beneath it, and it existed there because
the library's author had this consumer in mind. What grove kept is the two
things the library could not have supplied — a grammar to decide which entry a
caller's path argument names, and a path to hand back afterwards.

The carried example reaches its eighth step here. It starts from the tree
chapter 7 walked and then from the same grove one level deeper, because the
chain is only interesting once there is something above the leaf to collect.

```text
.grove/                                the tree as root-init left it, walked in chapter 7
├── BRIEF.md
└── 01-requirements--plan-k1.md

brief_chain(tree, "01-requirements--plan-k1.md")
  └─ [ <root>/BRIEF.md ]               one ancestor level: the grove root's own charter

.grove/                                the same grove, once that leaf has become a node
├── BRIEF.md                           the root charter
└── 01-plan-k1/                        a node — NN-<slug>-k<key>, no kind and no outcome
    ├── BRIEF.md                       the node's charter
    └── 01-requirements--scheme-k2.md  ← the leaf the argument names

leaf_entry(tree, "01-plan-k1/01-requirements--scheme-k2.md")
  │  relative ⇒ joined onto the caller's own spelling of the root
  │  is_file ✓   parses as Positioned { parts: Leaf { .. } } ✓
  │  canonicalised only to compare: not the root, and under it
  └─ the snapshot entry whose rendered name is that filename

entry.distinguished_chain()            the library's operation, root-first
  ├─ level .grove/       → BRIEF.md
  └─ level 01-plan-k1/   → BRIEF.md    a level with none is skipped, not reported

brief_chain ⇒ [ <root>/BRIEF.md,
                <root>/01-plan-k1/BRIEF.md ]      paths built by chapter 6's entry_path
kind_in     ⇒ Some(requirements)                  read from the filename, never the body
```

Two of those steps are not grove's. `distinguished_chain` is the library's, and
the ordering it returns — root-first, one entry per ancestor level that has a
distinguished child — is the library's too. `entry_path` is chapter 6's. The
figure is drawn with them in it because the shape of this chapter's answer is
exactly how little is left once they are taken out.

The chapter owns 428 of `task_tree.rs`'s 2,023 lines, in **three** blocks — more
than any other chapter of this file. A hundred and nine lines are the production
run: two verbs against an already-read tree, and the private resolver they share.
The other 319 are tests, in two blocks the file separates: 292 carrying the
`brief-chain` and `kind` sections, and the file's closing 27, which exercise
`pick` and `brief_chain` against one observation and are this chapter's because
of the second of those. That last block is the file's last 27 lines, and it is
the only place in `task_tree.rs` where two verbs are put in front of one tree
together.

<a id="two-questions-one-entry"></a>
## Two questions, one entry

The chapter's first ownership block is the production run. It expands to lines
638 through 746 of the file, and the six fragments it names run to the end of the
section after next.

<!-- fragment «kind-and-brief-chain» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="638-746" parent="source-task-tree" -->
<!-- insert «kind-in» -->
<!-- insert «brief-chain-fn» -->
<!-- insert «leaf-entry-signature» -->
<!-- insert «leaf-entry-grammar» -->
<!-- insert «leaf-entry-compare» -->
<!-- insert «leaf-entry-walk» -->
<!-- /fragment -->

The two verbs come first and they are both thin. `kind_in` is nineteen lines of
which six are a refusal the file itself calls unreachable.

<!-- fragment «kind-in» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="638-656" parent="kind-and-brief-chain" -->
````rust
/// [`kind`] against a tree already read.
pub(crate) fn kind_in(tree: &Tree, leaf_path: Option<&Path>) -> Result<Option<Kind>> {
    let target = match leaf_path {
        Some(path) => Some(path.to_path_buf()),
        None => pick_in(tree)?,
    };
    let Some(target) = target else {
        return Ok(None);
    };
    match leaf_entry(tree, &target)?.triple().map(|t| t.parts) {
        Some(Parts::Leaf { kind, .. }) => Ok(Some(kind.clone())),
        // Unreachable: `leaf_entry` refuses anything that is not a leaf.
        _ => bail!(
            "path is not a current-format Grove leaf: {}",
            target.display()
        ),
    }
}

````
<!-- /fragment -->

**The actor is grove, the input is an optional leaf path, and the output is a
session kind or nothing.** The optionality is the verb's whole interface
decision: with `Some`, that leaf; with `None`, whatever `pick_in` would have
chosen. Chapter 7 read `pick_in` and noted that its three call sites are outside
its own chapter; this is one of them, and it is the reason the doc comment there
says *used by every verb that needs a leaf and its brief chain from the same
observation*. `kind_in` takes a tree already read, so the leaf it defaults to and
the kind it reports come from one snapshot. A version that called `pick` and then
reopened the tree could report the kind of a leaf that was no longer the
selection.

**`Ok(None)` is an answer and not a failure**, and it is the same answer chapter
7's `pick_in` gives: the `let ... else` at the top returns it when there is no
leaf to ask about. The guide states the operator-visible form of this — when
nothing is live, `pick`, `kind` and `brief-chain` all print a diagnostic on
stderr, nothing on stdout, and exit `0`
([the guide's account of the tree verbs](../../USAGE.md#usage-tree-verbs)). The
crate's contribution is that the *absence* of live work travels as a value rather
than as an error, so the loop above can tell *done* from *broken*.

The `[`kind`]` the doc comment points at is `verbs::kind`, the public verb of that
name in `verbs.rs`: it opens the tree, calls this function, and renders the answer
as a `Sought`. Chapter 15 reads it with the other eleven. The minimum here is that
`kind_in` is the half of that verb which runs against a tree somebody else opened,
which is why its own name carries the crate's `_in` suffix — and that the link is
one the compiler does not check, a point the closing section returns to.

**The last arm is the file admitting to a shape its types cannot rule out.** The
match asks for `Parts::Leaf` and the comment beside the fallback says
*unreachable: `leaf_entry` refuses anything that is not a leaf*. That is true, and
it is true for a reason the reader can check rather than take: `leaf_entry`
returns only entries whose name parsed as `Positioned { parts: Parts::Leaf { .. } }`,
so `triple()` is `Some` and its `parts` is a `Leaf`. The arm exists because the
compiler cannot see the argument the comment makes, and the honest reading is
that grove chose a runtime `bail!` over an `unwrap` — the same refusal text a
non-leaf argument would have got, so a reachable-after-all path degrades into a
message rather than a panic.

**The kind is read from the filename and nothing else.** That claim is not
visible in these nineteen lines at all — the function reads `parts`, and `parts`
came from a name, and the name came from a directory entry. It is the twelve
tests below that make it a fact about grove rather than an implementation detail,
because the only way to demonstrate *never from the body* is to write bodies that
say otherwise and show they are not read.


<a id="the-chain-the-library-already-had"></a>
## The chain the library already had

The second verb is the one this chapter is named for, and it is the shortest
answer to *what could not move* in the book so far.

<!-- fragment «brief-chain-fn» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="657-673" parent="kind-and-brief-chain" -->
````rust
/// `brief-chain`: the `BRIEF.md` of each of the leaf's ancestor levels, from the
/// grove root down to its containing node, root→leaf.
///
/// This is the library's `distinguished_chain` and nothing else: a node's
/// distinguished child *is* its charter, and the library already skips levels
/// that have none — which is exactly `brief-chain`'s documented *a directory
/// level with no `BRIEF.md` is skipped silently*. A leaf has no brief of its
/// own, so its containing node's is the deepest one collected.
pub(crate) fn brief_chain(tree: &Tree, leaf_path: &Path) -> Result<Vec<PathBuf>> {
    let entry = leaf_entry(tree, leaf_path)?;
    Ok(entry
        .distinguished_chain()
        .into_iter()
        .map(|brief| entry_path(tree.root(), brief))
        .collect())
}

````
<!-- /fragment -->

**Nine lines of doc comment over seven lines of body, and the doc comment spends
its first sentence disclaiming authorship.** *This is the library's
`distinguished_chain` and nothing else.* The verb resolves the argument to an
entry, asks the library for the chain, turns each entry into a path, and returns
them.

The reason it can be that short is worth stating precisely, because it is the
opposite of the pattern every other chapter of Part II has shown. Everywhere
else, `grove-loop` is the layer that stayed because the library had no word for
something — no word for *live*, none for *finish*, none for a kind or an outcome.
Here the library has the word. `ordinal-fs-tree` offers `distinguished_chain` as
an operation in its own right, and its documentation says why it is one rather
than something a caller assembles: a node's distinguished child is that node's
own content, so the chain is every piece of content on the path down to an entry,
**which is what a consumer assembling context from a tree wants**. That is a
description of grove written into the crate underneath grove, and it is the
clearest evidence in this book that the extraction was designed rather than
merely performed.

**Three of grove's own concepts are nonetheless in this function, and each is a
seam.** The first is that a `BRIEF.md` *is* the distinguished child — the
identification is grove's, made in chapter 2 where `TaskName::distinguished`
advertises the name, and the library never learns what the file means. The
second is the skip. The doc comment claims the library's behaviour already
matches the verb's documented contract — *a directory level with no `BRIEF.md`
is skipped silently* — and that is a claim about two documents agreeing, which
this chapter can check: the library's `distinguished_chain` filters ancestors
through `distinguished()` and collects what survives, so a level without one
contributes nothing and reports nothing. Two tests below hold the grove side of
that agreement.

The third is the last line of the body. `entry_path(tree.root(), brief)` is
chapter 6's builder, and it is here because **the library returns entries and
never paths.** A consumer that wanted paths would have to build them, and
chapter 5's module header said grove builds them in one place and from the
caller's own spelling of the root. So the verb's output is absolute paths, which
is what the guide shows an operator seeing, and every component after the root is
a name the snapshot admitted.

The doc comment spells the verb `brief-chain`, hyphenated, because it is naming
the operator-facing verb rather than this function: `verbs::brief_chain` opens the
tree, calls this, and hands back the paths, and chapter 15 reads it. The minimum
here is that the *documented* contract the comment appeals to — a level with no
charter is skipped silently — belongs to that verb and to the guide, and this
function is where the library's behaviour is claimed to already satisfy it.

**A leaf has no brief of its own.** The doc comment's last sentence is the rule
in the chapter's title, stated from the collecting end: the deepest brief in the
chain belongs to the leaf's *containing node*, because the leaf is a file and a
file has no distinguished child. The other end of the same rule — that a brief
may not be the argument — is enforced in the resolver both verbs share, and that
is the next section.

<a id="what-a-leaf-argument-names"></a>
## What a leaf argument names, and the five things that can be wrong with it

Both verbs reach the snapshot through one private function, and it is the
chapter's real production content: 72 of the block's 109 lines. It is also where
this chapter's share of the crate's grammar work sits, because deciding which
entry a caller meant is a question about names before it is a question about
paths.

<!-- fragment «leaf-entry-signature» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="674-693" parent="kind-and-brief-chain" -->
````rust
/// The snapshot entry a caller's leaf argument names: absolute, or relative to
/// the grove root.
///
/// The clauses are in the order the path-walking reader had, because each says
/// something different to whoever hit it — *there is no such file*, *that is not
/// a task file*, *that is not in this tree*.
fn leaf_entry<'a>(tree: &'a Tree, leaf_path: &Path) -> Result<Entry<'a, TaskName>> {
    let root = tree.root();
    let candidate = if leaf_path.is_absolute() {
        leaf_path.to_path_buf()
    } else {
        root.join(leaf_path)
    };
    if !candidate.is_file() {
        bail!("Grove leaf not found: {}", candidate.display());
    }
    let name = candidate
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("Grove leaf has no UTF-8 filename: {}", candidate.display()))?;
````
<!-- /fragment -->

**The doc comment states an ordering decision rather than a behaviour.** *The
clauses are in the order the path-walking reader had, because each says something
different to whoever hit it — there is no such file, that is not a task file,
that is not in this tree.* This is a function whose refusals are its interface:
it returns one `Entry` and otherwise it explains. The order is preserved from an
earlier implementation on the ground that the three messages are not
interchangeable, and that is a claim about diagnostics rather than about
correctness — a reordering would refuse the same arguments and tell a different
story about why.

The first two clauses are in this fragment. A relative argument is joined onto
the root, which is the same *caller's own spelling* discipline chapter 6 read in
`entry_path`; the tests below exercise the relative form for both verbs. Then
`is_file` — and this is the clause that catches more than its name suggests,
because a directory argument, including the grove root itself, is not a file.

The next clause is the one that makes this grove's rather than the library's.

<!-- fragment «leaf-entry-grammar» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="694-707" parent="kind-and-brief-chain" -->
````rust
    // The grammar itself, rather than a second reading of the filename: a
    // non-canonical or unknown-kind argument gets the domain's own recovery
    // advice instead of a bare *not a leaf*.
    match TaskName::parse(name, Found::File) {
        Verdict::Entry(TaskName::Positioned {
            parts: Parts::Leaf { .. },
            ..
        }) => {}
        Verdict::Malformed(error) | Verdict::Reserved(error) => bail!("{error}"),
        _ => bail!(
            "path is not a current-format Grove leaf: {}",
            candidate.display()
        ),
    }
````
<!-- /fragment -->

**This is chapter 4's grammar, observed through chapter 8's verb.** The comment
says why the grammar is asked rather than the filename re-read: *a non-canonical
or unknown-kind argument gets the domain's own recovery advice instead of a bare
not a leaf*. `TaskName::parse` is the one route from a filename to a parsed name,
and the two refusing verdicts are forwarded verbatim — `bail!("{error}")` passes
the domain's rendered message through untouched, which is the mechanism by which
chapter 4's advice reaches an operator who typed a path at this verb.

**And this is where *a brief is not a leaf* is enforced.** The accepting arm asks
for `Positioned { parts: Parts::Leaf { .. } }` and nothing else. `BRIEF.md` parses
as `TaskName::Brief`, a variant that carries no position, no key and no parts at
all, so it cannot match; it falls to the final arm and is refused as *not a
current-format Grove leaf*. That is the rule the chapter is named for, and it is
enforced by the shape of a pattern rather than by a check. The same arm refuses a
node directory, for the same structural reason — chapter 3's `Parts::Node` has a
slug and nothing else.

**The second ending passes through this arm.** The book's second ending is a
task-*shaped* name that grove refuses although the store would have accepted the
entry, and the middle arm is where such a name dies in this chapter: a filename
whose session kind is not a well-formed token is `Verdict::Malformed`, and the
verb re-raises the grammar's own words. Chapter 2 holds the grammar half of that
in `a_session_kind_that_is_not_a_token_is_malformed`. Put beside the twelve kind
tests below, the pair is the crate's precise position on what a kind is: **open
as to vocabulary and closed as to shape.** grove will read `postmortem` and
`spike-2` out of a filename without knowing what either means, and will refuse a
kind that is not a token at all. The decision record `a-kind-is-an-open-token` is
where that was settled; the code's contribution is that the openness is not a
list anywhere.

The clause that follows is the one chapter 5 pointed forward to.

<!-- fragment «leaf-entry-compare» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="708-729" parent="kind-and-brief-chain" -->
````rust
    // Canonicalised to *compare* and never to report: two spellings of one path
    // name one entry, and the paths this module returns are still built from the
    // caller's own spelling of the root.
    let target = candidate
        .canonicalize()
        .with_context(|| format!("resolving leaf path {}", candidate.display()))?;
    let root_real = root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", root.display()))?;
    if target == root_real {
        bail!(
            "leaf path {} is the grove root, not a leaf",
            target.display()
        );
    }
    if !target.starts_with(&root_real) {
        bail!(
            "leaf path {} is not under grove root {}",
            target.display(),
            root_real.display()
        );
    }
````
<!-- /fragment -->

**What is compared is the caller's spelling against the tree's, and the result is
a boolean.** The candidate path is canonicalised, the root is canonicalised, and
the two derived facts — *this is not the root* and *this is under the root* — are
answered against those resolved forms. Nothing canonicalised leaves the function.
The `Entry` it returns is a snapshot entry, and every path the module hands back
is built afterwards by `entry_path` from the root the caller supplied.

That is the difference the header called out and it is worth stating as an
operation rather than a rule: **comparing collapses two spellings that name one
inode, and reporting would replace the operator's spelling with a different
one.** On macOS `/var` and `/private/var` are that pair. A comparison's answer is
`true` or `false` and the operator never sees either side; a reported path is the
answer itself. So a function may canonicalise all it likes as long as it throws
the result away.

**The header's claim that this is the only place it happens is false, and chapter
6 owns the counterexample.** `target`, in chapter 6's block, canonicalises three
times for the same reason — its own doc comment says *exactly as `leaf_entry`
does* — and the adjudication, with the enumeration of all eight call sites behind
it, is at [chapter 6's account](06-paths.md#canonicalise-to-compare). This
chapter reads the second of the two functions and does not restate the count.

**Both of the refusals in this fragment are unobserved by the entire workspace**,
and that is measured rather than inferred: deleting the two `if` blocks — lines
717 to 729, leaving the canonicalisation and the walk untouched — leaves all 245
of `grove-loop`'s inline tests green and all twenty-five of `grove-llm`'s test
targets green. Nothing in the repository asserts *is the grove root, not a leaf*
or *is not under grove root*. The section on the tests returns to this, because
two tests are named as though they reach these lines and neither does.

The last fragment is the walk, and the refusal that closes the function.

<!-- fragment «leaf-entry-walk» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="730-746" parent="kind-and-brief-chain" -->
````rust
    for entry in tree.walk() {
        if entry.name().to_string() != name {
            continue;
        }
        if entry_path(root, entry).canonicalize().ok() == Some(target.clone()) {
            return Ok(entry);
        }
    }
    // Task-shaped, under the root, and still not in the tree: it sits under a
    // name the grammar disclaimed, so no walk reaches it.
    bail!(
        "Grove leaf {} is not in the task tree: every level above it must be a \
         node directory named NN-<slug>-k<key>",
        candidate.display()
    )
}

````
<!-- /fragment -->

**The match is on the rendered name and the path is the tie-breaker.** The loop
compares `entry.name().to_string()` against the caller's filename, which is a
comparison between a name the snapshot admitted and a byte string from the
argument — sound because chapter 4's canonicity check makes *admitted* and
*would have been rendered so* the same thing. A tree may hold two entries with
the same filename at different levels, so the name match alone is not an answer;
the canonicalised path comparison picks which one, and it is the third and last
use of canonicalisation in the function.

`entry_path(root, entry).canonicalize().ok()` discards the error rather than
propagating it, and the `ok()` is doing real work: an entry the snapshot lists
may have been removed since, and a failed canonicalisation of *some other*
candidate is not a reason to refuse the caller's. It simply does not match.

**The closing refusal names a cause the earlier clauses have already excluded.**
By the time control reaches it the argument is task-shaped, is a file, and is
under the root — and is still not in the tree. The comment gives the only
remaining explanation: *it sits under a name the grammar disclaimed, so no walk
reaches it*, and the message tells the operator the repair — every level above a
leaf must be a node directory named `NN-<slug>-k<key>`. This is the mirror of
chapter 7's disclaimed-name result. There, a name the grammar does not claim is
passed over and the walk continues; here, work parked under such a name is
invisible to every verb, and this message is the only place the crate says so to
whoever went looking for it.

**Five of this function's seven refusals are unobserved by the workspace.** The
same instrument gives the whole picture: replacing the UTF-8 refusal, the
grammar arm's forwarded `{error}`, the two root-containment clauses and this
closing `bail!` with unreachable markers, one mutation at a time, leaves both
suites green. Only two of the seven are held by a test — the `is_file` clause and
the *not a current-format Grove leaf* arm — and the next section names the two
tests that hold them, which are not the two their names suggest.


<a id="twenty-two-tests"></a>
## The twenty-two tests, and what each would still pass under

The chapter's second ownership block is the file's `brief-chain` and `kind` test
sections: 292 lines under two section labels, carrying twenty-two of the file's
sixty-three `#[test]` functions — ten and twelve. It is the largest of the four
blocks in this file that hold tests, ahead of chapter 9's twenty-one over 344
lines and chapter 7's nineteen over 255. The block introduces one fixture of its
own, `touch_body`, and otherwise builds every tree with `grove`, `touch` and
`mknode` and drives both verbs through the `brief_chain_at` and `kind`
compositions — all five of them chapter 6's, and the first of them the one whose
comment says production never wants it.

<!-- fragment «brief-chain-and-kind-tests» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1361-1652" parent="source-task-tree" -->
<!-- insert «chain-tests-shape» -->
<!-- insert «chain-tests-siblings» -->
<!-- insert «chain-tests-skipping» -->
<!-- insert «chain-tests-done-and-relative» -->
<!-- insert «chain-tests-refusals» -->
<!-- insert «kind-tests-label-and-fixture» -->
<!-- insert «kind-tests-two-leaves» -->
<!-- insert «kind-tests-open-token» -->
<!-- insert «kind-tests-legacy-label» -->
<!-- insert «kind-tests-default-and-empty» -->
<!-- insert «kind-tests-relative-path» -->
<!-- insert «kind-tests-body-ignored» -->
<!-- insert «kind-tests-absent-root» -->
<!-- /fragment -->

Each test below is given twice over: the property it establishes, and **what
would have to be true for it to pass while that property was broken**. This
block's answer to the second half is unusually blunt, and it is the chapter's
main result. **Three of the block's four refusal tests refuse somewhere other
than the clause their names describe**, and that is measured rather than read off
the names.

| Group | Tests | The property the group holds |
|---|---:|---|
| the shape of a chain | 2 | one entry per ancestor level, root first, each a `BRIEF.md` |
| not a sibling's | 1 | the ascent cannot reach a sibling subtree |
| a level without one | 2 | a missing brief is skipped, at a node and at the root alike |
| outcome and spelling | 2 | a `DONE` leaf has ancestors; a relative argument is joined onto the root |
| three refusals | 3 | each is refused, and none by the clause its name suggests |
| the filename, not the body | 6 | the kind is the token in the name whatever the body says |
| the open token | 1 | grove reads a kind it has never heard of |
| defaulting and absence | 3 | no argument means `pick`'s leaf; no live work means `Ok(None)` |
| a relative argument | 1 | the same joining rule as the chain verb |
| an absent root | 1 | refused by the opening, not by this verb |

The grouping is this chapter's; the file has two labels and two flat runs. The
thirteen fragments below follow source order.

The section label and the first two chain tests come together, because the second
is what the first is worth.

<!-- fragment «chain-tests-shape» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1361-1395" parent="brief-chain-and-kind-tests" -->
````rust
    // ---- brief-chain --------------------------------------------------------

    #[test]
    fn brief_chain_root_level_leaf_returns_only_root_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let leaf = touch(&g, "01-impl--a-k1.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_two_levels_deep_root_then_each_ancestor_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "02-mid-k1");
        touch(&n1, "BRIEF.md");
        let n2 = mknode(&n1, "01-node-k2");
        touch(&n2, "BRIEF.md");
        let leaf = touch(&n2, "01-impl--leaf-k3.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        // Each brief's parent dir distinguishes them; assert on the parent.
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "02-mid-k1", "01-node-k2"]
        );
        assert!(chain.iter().all(|p| name_of(p) == "BRIEF.md"));
    }

````
<!-- /fragment -->

**The first test's property is that a root-level leaf collects exactly the grove
root's own charter** — one level of ancestry, one brief. It is the base case, and
alone it establishes very little: the fixture contains exactly one `BRIEF.md`, so
an implementation that globbed every `BRIEF.md` in the tree, or returned the
leaf's own directory's brief, or returned the root's brief unconditionally, would
all answer identically.

**The second test is where the chain becomes a chain.** Three levels, three
briefs, and the assertion is on each brief's *parent directory* — `.grove`,
`02-mid-k1`, `01-node-k2` — with a second assertion that every returned path is
in fact named `BRIEF.md`. The comment above it says why the assertion is shaped
that way: the three files are indistinguishable by their own names, so the parent
is the only thing that identifies them.

**What the pair would pass under with the property broken:** the order is
genuinely pinned — `Vec` equality is ordered, so a leaf-to-root implementation
fails — and the two-assertion structure closes the reading that the function
returns *directories* rather than briefs. What neither closes is breadth: this
fixture is a single spine with no branch, so an implementation collecting every
`BRIEF.md` anywhere in the tree still passes both. The next test exists for that,
and it is the only test in the block that could close it.

<!-- fragment «chain-tests-siblings» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1396-1416" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn brief_chain_only_includes_ancestors_not_sibling_subtrees() {
        // The directory ascent inherently excludes a sibling node's brief: a leaf
        // under `01-design` never sees `02-other`'s brief.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let design = mknode(&g, "01-design-k1");
        touch(&design, "BRIEF.md");
        let other = mknode(&g, "02-other-k3");
        touch(&other, "BRIEF.md");
        let leaf = touch(&design, "01-impl--leaf-k2.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

````
<!-- /fragment -->

**The property is that the chain is an ancestry and not a search**: a leaf under
`01-design-k1` never sees `02-other-k3`'s brief, though both nodes sit at the
same level and both have one.

**What it would pass under with the property broken:** almost anything, and the
test's own comment says so before the fixture is built — *the directory ascent
inherently excludes a sibling node's brief*. This is a test of a property the
implementation cannot fail to have, because the implementation is
`distinguished_chain` over `ancestors()` and an ancestor list has no branch in
it. It discriminates against exactly one implementation, the tree-wide glob that
test 2 left open, and nothing else. That is a real thing to close and a thin
thing to have a test for; the honest description is that this test pins the
*contract* against a future rewrite rather than the code that exists.

The pair that follows is the block's sharpest, and it is sharp because of what
differs between its two halves.

<!-- fragment «chain-tests-skipping» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1417-1452" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn brief_chain_skips_missing_intermediate_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        // No BRIEF.md in `02-mid` — a mid-decomposition transient.
        let n1 = mknode(&g, "02-mid-k1");
        let n2 = mknode(&n1, "01-node-k2");
        touch(&n2, "BRIEF.md");
        let leaf = touch(&n2, "01-impl--leaf-k3.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-node-k2"]
        );
    }

    #[test]
    fn brief_chain_skips_missing_root_brief() {
        let (_t, g) = grove();
        // No root BRIEF.md.
        let n1 = mknode(&g, "02-mid-k1");
        touch(&n1, "BRIEF.md");
        let leaf = touch(&n1, "01-impl--leaf-k2.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec!["02-mid-k1"]
        );
    }

````
<!-- /fragment -->

**The property is the verb's documented behaviour: a directory level with no
`BRIEF.md` is skipped silently.** Silently is the operative word — the chain is
shorter, and nothing reports that a level was passed over. The first test removes
the brief from an intermediate node; the second removes it from the grove root
itself.

**What each would pass under with the property broken:** an implementation that
returned an error on a missing brief fails both; one that inserted a placeholder
path fails both. So each test alone closes *refuse* and *fabricate*. What only
the **pair** closes is the reading that the root is special. A perfectly
plausible implementation treats the grove's own charter as mandatory and every
node's as optional — that is how a great many hierarchical-config readers behave
— and it passes the first test and fails the second. Having both is what makes
*a level* mean every level, and it is the clearest case in this block of two
tests being worth more than twice one.

**Neither test observes the silence.** Both assert on the returned vector, and a
silent skip and a skip that logged a warning to stderr are indistinguishable to
them. The word *silently* in the verb's contract is held by no test in this
crate, and the operator-facing half of it — that a short chain means a level had
no charter and never that the walk stopped — is stated in the guide rather than
asserted anywhere.

The next pair tests two independent properties and is grouped only by source
order.

<!-- fragment «chain-tests-done-and-relative» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1453-1487" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn brief_chain_resolves_chain_for_a_done_leaf() {
        // Normally called on a live leaf, but a `DONE` leaf still has ancestors.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-design-k1");
        touch(&n1, "BRIEF.md");
        let leaf = touch(&n1, "01-DONE-impl--leaf-k2.md");
        let chain = brief_chain_at(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

    #[test]
    fn brief_chain_accepts_grove_root_relative_leaf_path() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-design-k1");
        touch(&n1, "BRIEF.md");
        touch(&n1, "01-impl--leaf-k2.md");
        let chain = brief_chain_at(&g, Path::new("01-design-k1/01-impl--leaf-k2.md")).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

````
<!-- /fragment -->

**The first property is that the chain does not depend on the leaf's outcome.**
Its comment states the case honestly — *normally called on a live leaf, but a
`DONE` leaf still has ancestors* — and this is the only test in the block whose
leaf is not live. It matters because `brief-chain` is what a session reads to
bootstrap, and a session may be re-reading a task it has already retired.

**What it would pass under with the property broken:** any implementation that
does not consult the outcome, which is most of them, because the resolver's
accepting pattern is `Parts::Leaf { .. }` and the wildcard discards the outcome
without a decision being visible. The test would catch a deliberate live-only
filter and nothing subtler. Note what it does *not* establish: it says nothing
about `ABANDONED`, the other terminal mark chapter 2 read, and no test in this
block uses one.

**The second property is that a grove-root-relative argument is joined onto the
root** rather than resolved against the process's working directory. This one
discriminates properly: the fixture creates the file under the temporary grove
and then passes `01-design-k1/01-impl--leaf-k2.md` with no leading directory, so
an implementation that skipped the join would `is_file`-test a path relative to
wherever the test binary happened to run and refuse. It is a real test of a real
clause, and its twin for the other verb appears eight tests later.

Then the three refusals, and this is where the block's names stop matching its
behaviour.

<!-- fragment «chain-tests-refusals» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1488-1522" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn brief_chain_errors_when_leaf_outside_grove_root() {
        let (tmp, g) = grove();
        touch(&g, "BRIEF.md");
        let stray = tmp.path().join("stray.md");
        fs::write(&stray, b"# stub\n").unwrap();
        let err = brief_chain_at(&g, &stray).unwrap_err();
        assert!(
            err.to_string().contains("not a current-format Grove leaf"),
            "got {err}"
        );
    }

    #[test]
    fn brief_chain_errors_when_given_the_grove_root_itself() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let err = brief_chain_at(&g, &g).unwrap_err();
        assert!(
            err.to_string().contains("Grove leaf not found"),
            "got {err}"
        );
    }

    #[test]
    fn brief_chain_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = brief_chain_at(&missing, Path::new("01-impl--a-k1.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

````
<!-- /fragment -->

These three are the most instructive tests in the chapter, and not for the
reasons their names give.

**`brief_chain_errors_when_leaf_outside_grove_root` does not reach the
outside-the-root clause.** The fixture writes `stray.md` in the temporary
directory beside the grove. `stray.md` is not a task-shaped name, so
`TaskName::parse` does not yield `Positioned { parts: Leaf }`, and the resolver
refuses at the grammar arm — which is precisely the message the test asserts on,
*not a current-format Grove leaf*. Control never reaches the `starts_with` check
twenty lines below. Measured: changing that arm's message breaks this test;
deleting the containment clause entirely does not. **To exercise containment you
would need a task-shaped file outside the root, and nothing in the workspace
builds one.**

**`brief_chain_errors_when_given_the_grove_root_itself` does not reach the
grove-root clause either.** It passes `g`, the root directory, and a directory is
not a file, so the resolver refuses at the very first clause — *Grove leaf not
found*, which is again exactly the text asserted. And the clause the test is
named for is not merely untested: **it cannot fire.** Reaching it requires an
argument that `is_file` accepted and that canonicalises to the same path as the
grove root, and chapter 5's opening refuses a root that is not a directory
(`task_tree.rs` line 276, *grove root not found*). A regular file and a directory
cannot canonicalise to one path, so `target == root_real` is unreachable while
the tree is open. It is defensive code with a test named after it and no way in.

**`brief_chain_errors_when_grove_root_absent` does not reach the resolver at
all.** The composition opens the tree first, and an absent root is refused there,
in chapter 5's words. Like chapter 7's last test, what this is really evidence
for is which of chapter 5's four openings `brief_chain_at` composed: `read`,
which refuses, rather than `read_or_vacant`, which treats absence as an answer.

**What all three would pass under:** any implementation that refuses these three
arguments for any reason whatever, since each asserts on a substring of a rendered
message and none asserts where the refusal came from. Taken with the mutation
result in the previous section — five of the resolver's seven refusals unobserved
across both suites — the block's coverage of refusal is one clause and one
grammar arm, wearing the names of four.

The `kind` section opens with its label and the fixture that makes its central
claim testable.

<!-- fragment «kind-tests-label-and-fixture» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1523-1532" parent="brief-chain-and-kind-tests" -->
````rust
    // ---- kind ---------------------------------------------------------------

    /// Write a leaf whose body carries arbitrary legacy routing metadata. The
    /// current reader must derive kind solely from the filename.
    fn touch_body(dir: &Path, name: &str, body_after_header: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, format!("# stub\n\n{body_after_header}").as_bytes()).unwrap();
        p
    }

````
<!-- /fragment -->

**The fixture states the property the section exists to hold**: *write a leaf
whose body carries arbitrary legacy routing metadata; the current reader must
derive kind solely from the filename*. Every test that uses it writes a body, and
the body is always a lie or an irrelevance. `touch_body` is the only fixture this
chapter's blocks define; the rest are chapter 6's.

The first two tests are a smoke test and the claim.

<!-- fragment «kind-tests-two-leaves» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1533-1546" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn kind_reads_an_impl_leaf() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-impl--a-k1.md", "**Kind:** impl\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(a_kind("impl")));
    }

    #[test]
    fn kind_reads_a_planning_leaf() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-planning--a-k1.md", "**Kind:** impl\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(a_kind("planning")));
    }

````
<!-- /fragment -->

**The first establishes almost nothing**, and it is worth saying so. Its filename
says `impl` and its body says `impl`, so the two agree; an implementation that
read the body, an implementation that read the filename, and an implementation
that read whichever it found first all return `impl`. It is a smoke test wearing
the section's name.

**The second is the section's actual claim.** The filename says `planning` and
the body says `impl`, and the expected answer is `planning`. Body and name
disagree, and the test says which wins. Every later test in the section is a
variation on this one; this is the only structural difference between the pair,
and it is the whole difference between a test and an example.

The third is the section's most interesting test, and its doc comment records why
it was rewritten.

<!-- fragment «kind-tests-open-token» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1547-1574" parent="brief-chain-and-kind-tests" -->
````rust
    /// The verb reads whatever token the filename carries, including tokens no
    /// methodology declares and none this repo has ever configured.
    ///
    /// It used to sweep `Kind::ALL`, which made the assertion *the reader agrees
    /// with the enum* — true by construction and blind to the property that
    /// matters now (`open-kind-k20`). The list below is shapes plus two tokens
    /// grove has never heard of, and the point is that it cannot tell.
    #[test]
    fn kind_reads_whatever_token_the_filename_carries() {
        let (_t, g) = grove();
        for (i, label) in [
            "requirements",
            "impl",
            "research-a",
            "integrate-review-prototype",
            "finish",
            "postmortem",
            "spike-2",
        ]
        .into_iter()
        .enumerate()
        {
            let name = format!("{:02}-{}--a-k{}.md", i + 1, label, i + 1);
            let leaf = touch_body(&g, &name, "**Kind:** bogus\n**Harness:** bogus\n");
            assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(a_kind(label)));
        }
    }

````
<!-- /fragment -->

**The property is that the kind is an open token: grove reads whatever the
filename carries, including tokens no methodology declares.** The list runs
through four shapes grove does use — a reserved kind, the ordinary one, a
research arm, a long hyphenated integration kind, the other reserved one — and
then `postmortem` and `spike-2`, which nothing in this repository has ever
configured. The body of every one says `**Kind:** bogus`.

**The doc comment is a record of a test that used to prove nothing**, and it is
the clearest statement in the crate of what a tautological test looks like: *it
used to sweep `Kind::ALL`, which made the assertion the reader agrees with the
enum — true by construction and blind to the property that matters now.* When
`Kind` was a closed enum with a roster, a test that read every variant back was
checking that two derived tables agreed with each other. Once `open-kind-k20`
made a kind any well-formed token, the roster went, and the property worth
holding became *it cannot tell* — which only a token grove has never heard of can
demonstrate. `Kind::ALL` no longer exists, and the comment's account of it is
history rather than a claim about the code.

**What it would pass under with the property broken:** an implementation carrying
a hard-coded list that happened to contain all seven labels. That is not a
realistic implementation, but the way the test closes it is worth noticing — it
is the *unfamiliar* tokens that do the work, and they do it precisely because
nobody would have thought to list them. Put beside chapter 2's
`a_session_kind_that_is_not_a_token_is_malformed`, which refuses a kind that is
not a well-formed token at all, the two give the crate's full position: **open as
to vocabulary, closed as to shape.** grove will carry `spike-2` and will refuse
`Spike 2`, and neither fact is a list anywhere.

The remaining nine tests split into one more disagreement, a defaulting pair, a
relative path, four more bodies, and a refusal.

<!-- fragment «kind-tests-legacy-label» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1575-1581" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn kind_ignores_a_legacy_work_label_in_the_body() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-impl--a-k1.md", "**Kind:** work\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(a_kind("impl")));
    }

````
<!-- /fragment -->

**The property is the second test's, with `work` in the body** — a label the
methodology retired. It discriminates against a body-reading implementation
exactly as far as its predecessor does and no further; what it adds is a record
of which legacy strings were actually met in real task files.

<!-- fragment «kind-tests-default-and-empty» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1582-1599" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn kind_no_arg_defaults_to_picks_next_leaf() {
        let (_t, g) = grove();
        touch(&g, "01-DONE-impl--old-k1.md"); // skipped by pick
        touch_body(&g, "02-planning--live-k2.md", "**Kind:** impl\n");
        // No leaf arg ⇒ pick's next live leaf (02-live), whose kind is planning.
        assert_eq!(kind(&g, None).unwrap(), Some(a_kind("planning")));
    }

    #[test]
    fn kind_none_on_empty_grove() {
        // No live leaves ⇒ Ok(None), the same signal pick gives (the CLI renders
        // the "no live leaves" diagnostic).
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        assert_eq!(kind(&g, None).unwrap(), None);
    }

````
<!-- /fragment -->

**The first property is the `None` argument: the verb asks `pick`.** The fixture
is careful — a `DONE` leaf at position 01 and a live one at 02 — so the answer
`planning` is only reachable if the default skipped the retired leaf, which makes
this a test of chapter 7's walk observed through this verb. The body of the live
leaf says `impl`, so the test carries the filename claim as well.

**What it would pass under:** an implementation defaulting to *the last leaf*, or
to *the highest-numbered live leaf*, answers `planning` here too. The fixture has
two entries and the correct answer is the second of them, so ordering is
unconstrained. Chapter 7's nineteen tests are what pin the walk; this one pins
only that the default is the walk's answer rather than a fixed choice.

**The second property is that no live work is `Ok(None)` and not an error.** The
comment names the consequence — the CLI renders the *no live leaves* diagnostic —
and this is the same signal chapter 7 read out of `pick`. It is the shape the
loop needs in order to distinguish a finished grove from a broken one.

<!-- fragment «kind-tests-relative-path» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1600-1609" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn kind_accepts_a_grove_root_relative_path() {
        let (_t, g) = grove();
        let node = mknode(&g, "01-design-k1");
        touch(&node, "BRIEF.md");
        touch_body(&node, "01-impl--leaf-k2.md", "**Kind:** impl\n");
        let got = kind(&g, Some(Path::new("01-design-k1/01-impl--leaf-k2.md"))).unwrap();
        assert_eq!(got, Some(a_kind("impl")));
    }

````
<!-- /fragment -->

**The property is the joining rule, for the second verb.** It is the twin of the
chain verb's relative-path test, and it discriminates for the same reason: the
argument names a nested leaf with no leading directory, and only a join onto the
grove root finds it. Both verbs share one resolver, so the pair tests one clause
twice — which is defensible, since the two verbs are separately callable and the
sharing is an implementation fact rather than a contract.

<!-- fragment «kind-tests-body-ignored» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1610-1641" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn kind_ignores_trailing_commentary_on_a_legacy_kind_line() {
        let (_t, g) = grove();
        let leaf = touch_body(
            &g,
            "01-impl--a-k1.md",
            "**Kind:** impl          (or: planning)\n",
        );
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(a_kind("impl")));
    }

    #[test]
    fn kind_reads_an_impl_filename_with_no_kind_line() {
        let (_t, g) = grove();
        let leaf = touch(&g, "01-impl--a-k1.md");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(a_kind("impl")));
    }

    #[test]
    fn kind_ignores_a_garbled_kind_token_in_the_body() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-impl--a-k1.md", "**Kind:** bogus\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(a_kind("impl")));
    }

    #[test]
    fn kind_ignores_a_family_name_written_in_the_body() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-impl--a-k1.md", "**Kind:** review\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(a_kind("impl")));
    }

````
<!-- /fragment -->

**These four are one property and four bodies**, and reading them as four claims
would overestimate the section by three.

- **trailing commentary** — `impl          (or: planning)`, a kind line with a
  human's aside after it;
- **no kind line at all** — a body of `# stub`, which is what `touch` writes;
- **a garbled token** — `bogus`, which is not a kind;
- **a family name** — `review`, which names a family of kinds rather than one.

The implementation never opens the file, so none of the four can fail while the
others pass. **What they are actually worth is an inventory of what the migration
met**: the shapes that existed in real task files when the body stopped being
read, kept as fixtures so that a future implementation which started parsing
bodies again would break on all four at once rather than on whichever the author
had thought of.

**Two of them discriminate nothing even against a body-reading implementation.**
The no-kind-line test has no body claim to prefer, so a reader that consulted the
body and fell back to the filename passes it; and the same is true of the very
first test in the section, where body and filename agree. Ten of the twelve `kind`
tests genuinely separate the two sources; those two do not.

**And no test in this block asserts that the file is never read.** The strongest
property the twelve support is *the body's content does not change the answer*,
which an implementation that read the body and then discarded it would also
satisfy. Chapter 7's block holds the instrument that would close this — the
`READ_COUNT` accessors and an assertion on the count — and this block does not use
it. That is the residue of the `kind` section, and it is the same shape as the
residue chapter 7 recorded for its own refusals: a property stated in a doc
comment and held by nothing.

<!-- fragment «kind-tests-absent-root» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1642-1652" parent="brief-chain-and-kind-tests" -->
````rust
    #[test]
    fn kind_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = kind(&missing, None).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

````
<!-- /fragment -->

**The last test is the chain section's last test again, for the other verb**: an
absent root is refused by the opening rather than by anything this chapter owns,
and the assertion is on chapter 5's wording. Both verbs compose `read`, so both
refuse; a verb that had composed `read_or_vacant` would have answered instead.


<a id="one-observation"></a>
## The file's last block: `pick` and `brief_chain` together

The chapter's third ownership block is the last 27 lines of `task_tree.rs`: one
section label, one test, and the closing brace of the test module. It is this
chapter's rather than chapter 7's because the thing it exercises second is
`brief_chain`, and it is the only place in the file where two verbs are put in
front of one tree.

<!-- fragment «pick-with-brief-chain-tests» owner="root-to-leaf" source="crates/grove-loop/src/task_tree.rs" lines="1997-2023" parent="source-task-tree" -->
````rust
    // ---- pick + brief-chain together ----------------------------------------

    #[test]
    fn pick_then_brief_chain_on_a_realistic_nested_tree() {
        // End-to-end: pick the first live leaf in a nested tree, then resolve its
        // ancestor brief chain — the loop's bootstrap path.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-scheme-k1");
        touch(&n1, "BRIEF.md");
        touch(&n1, "01-DONE-impl--id-model-k2.md");
        let leaf = touch(&n1, "02-impl--read-verbs-k3.md");
        touch(&g, "02-impl--shed-tui-k4.md");

        let picked = pick(&g).unwrap().unwrap();
        assert_eq!(picked, leaf);

        let chain = brief_chain_at(&g, &picked).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-scheme-k1"]
        );
    }
}
````
<!-- /fragment -->

**The property is the loop's bootstrap path, end to end**, and the comment says
so: pick the first live leaf in a nested tree, then resolve its ancestor brief
chain. The fixture is the block's own word for what makes it worth having —
*realistic* — and it is the only tree in the chapter with all four features a
real grove has at once: a root charter, a node with a charter of its own, a
retired leaf, and a live leaf after the node at root level.

That shape does discriminate. `pick` must descend into `01-scheme-k1` rather than
taking the root-level `02-impl--shed-tui-k4.md`, and it must skip the `DONE` leaf
at position 01 inside the node. Both are chapter 7's properties, re-established
here incidentally; what is new is that the leaf `pick` chose is then a valid
argument to `brief_chain`, and that the chain it produces is the one belonging to
where that leaf actually sits.

**And the test composes the two verbs in exactly the way production must not.**
`pick(&g)` opens the tree and closes it; `brief_chain_at(&g, &picked)` opens it
again. Two observations, with the lock released in between. Chapter 6 read
`brief_chain_at`'s comment — *production never wants it: `llm_cli` holds one tree
across `pick` and the ancestor walk, because selecting a leaf and reading its
brief chain under two observations would be reading a tree that could move in
between* — and chapter 7 read `pick_in`'s, which exists so that a caller can have
both *from the same observation*. This block is labelled *pick + brief-chain
together*, and it is the one place in the file where they are together the wrong
way.

**What it would pass under with the property broken:** an implementation with the
race the doc comments warn about, which is to say the implementation this test
actually exercises. Nothing here could catch a tree that moved between the two
opens, because nothing makes it move; the two reads happen microseconds apart on
a tree only this test can see. The property *one observation* is stated in two doc
comments, held by chapter 7's `read_count` assertion for `select` alone, and not
held at all for the pairing this block is named after. The composition that does
it correctly lives outside this crate, in `crates/grove-llm/src/cli.rs`, whose
`cmd_brief_chain` opens one tree and hands the same guard to both — and no test
in `task_tree.rs` can reach it.

That is the honest close of the block, and it is not a criticism of the fixture.
A test that could observe the race would have to interleave a mutation between
two reads under a lock the first read released, which is a test of the store's
locking rather than of these two verbs. The residue is that the file's most
integrated-looking test is integrated at the wrong seam.

<a id="what-the-chain-kept"></a>
## What the chain kept

Every other chapter of Part II has answered *what did not go, and why could it
not?* by naming something the library has no word for. This chapter answers it by
naming how little that turned out to be.

**The chain itself moved.** `distinguished_chain` is the library's, and it is an
operation there rather than something a caller composes because the library's
author wrote it for a consumer assembling context from a tree — which is this
consumer. The ordering, the skipping and the root-first shape are all beneath the
seam. `brief_chain`'s body is four lines and its doc comment opens by saying the
function is nothing else.

**Three things stayed, and each is one of the book's three questions.** *On the
way in, the names:* the library hands back entries and has no opinion about which
of them a caller's path argument meant, so `leaf_entry` exists — and it decides by
asking chapter 4's grammar rather than by re-reading the filename, which is what
lets a wrong argument get the domain's own recovery advice. *On the way through,
the preconditions:* both verbs take an already-read tree so that the leaf and its
context come from one snapshot, and `kind_in` defaults through `pick_in` rather
than through `pick` for exactly that reason. *On the way out, the policy:* the
kind is the token in the filename and never anything in the body, and it is open
as to vocabulary and closed as to shape — a choice nothing beneath grove could
have defaulted, because nothing beneath grove knows what a kind is.

**A brief is not a leaf, and that is a fact about a pattern.** The resolver
accepts `Positioned { parts: Parts::Leaf { .. } }` and refuses everything else, so
a `BRIEF.md` argument and a node-directory argument are both turned away by the
shape of a match arm rather than by a check anyone had to remember to write. The
collecting end of the same rule is that a leaf contributes no brief of its own,
so the deepest brief in a chain always belongs to the leaf's containing node.

**What the twenty-three tests do not hold** is this chapter's largest residue,
and all of it is measured. Five of the resolver's seven refusals are unobserved by
both suites; one of those five — the grove-root clause — cannot fire at all while
the tree is open, and has a test named after it that refuses two clauses earlier.
Two more refusal tests assert on messages produced somewhere other than where
their names point. No test asserts that a leaf's body is never read, though that
is the `kind` section's whole subject. And the block named for the two verbs
together composes them under two observations, which is the one thing both doc
comments say production must not do.

**One smaller defect sits in the block's first line and is not a test's fault.**
`kind_in`'s doc comment opens `[`kind`]`, an intra-doc link, and it does not
resolve: `verbs` is not imported into `task_tree`, so rustdoc renders the text
and emits *unresolved link to `kind`*. The sentence it appears in is true — the
link is the only broken thing about it — and `task_tree.rs` carries two more of
the same shape in chapter 7's block, at `[`pick`]` and `[`select`]`. Checked with
`cargo doc --no-deps --document-private-items`, which is the only instrument that
sees it, because nothing about a doc link fails a build or a test. It is recorded
here rather than repaired: the corpus is frozen, and a fix changes bytes two
chapters reproduce.

None of that makes a claim in this chapter false. It makes the tests a weaker
witness than their names suggest, and the difference between those two things is
what this book's third obligation exists to record.

Chapter 9 takes the same snapshot and the last of `task_tree.rs`'s production
concerns: the reference grammar, which is wider than a key and has an outcome —
ambiguity — that the library has no counterpart for. Two of this file's ten
ownership blocks remain, and both are chapter 9's.

[Previous: The walk: pick and select](07-the-walk.md) | [Contents](README.md)


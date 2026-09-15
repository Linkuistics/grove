# grove-loop
<!-- book-page id="contents" role="contents" -->

This book explains the `grove-loop` crate to a reader who knows Rust and jj and
has driven a grove. grove's own vocabulary is linked to the glossary rather than
re-taught, and the entry point to the system is the
[user guide's account of the task tree and its filename grammar](../../USAGE.md#usage-task-tree),
which states this crate's subject in the terms an operator already uses: a
directory of leaf files whose names carry a kind, a slug and a permanent key,
and whose shape is the only state grove keeps. This reader has run a grove, has
watched `pick` choose a leaf, and has seen a session's commit name a work item by
its handle.

`crates/grove-loop` is sixteen source roots and 12,158 lines, and it is the layer that
stayed. Three domain-free crates sit underneath it — an ordered filesystem tree,
a keyed launcher, a version-control workspace — and none of them has a word for a
*kind*, a *brief chain*, an *outcome*, a *handle* or *finishing*. **This is the
library that owns the domain alongside the read-only viewer**, and each
chapter opens on what this module kept when the domain-free crates took the rest,
and on why that part could not move.

**The intended outcome is the what-could-not-move test.** At the end you should
be able to take any system that has extracted a domain-free library from
underneath itself — a store, a runner, a version-control seam — and ask of the
layer that stayed: *what did not go, and why could it not?* The answer comes in
three parts, each with a cost the layer visibly pays.

- **On the way in — the names.** Does the layer own a grammar the library beneath
  it cannot check? A grammar you own must be canonical — `format(parse(f)) == f`
  — or one entity occupies two files, sharing a key and a position. Canonicity
  costs a conformance kit.
- **On the way through — the preconditions.** Does the layer check what the
  library cannot see, and against *which* snapshot? The check must run against
  the same snapshot the operation then plans from, or it is a race with a name,
  and a refused run must consume nothing.
- **On the way out — the policy.** What does the layer choose that nothing
  beneath it could have defaulted? A chosen value must be stated where a reader
  can find it, and the layer must not restate what the layer above owns.

All three are examined inside this corpus, and the closing chapter applies
them to each of the twenty source-owning chapters in turn.

**The book's boundary is this crate's side of every seam it sits on.** Where
`grove-loop` calls `ordinal-fs-tree`, `keyed-launch` or `jj-workspace`, the book
says what grove asked for and what came back, names the crate in prose, and stops;
each of those crates has a book of its own. The methodology is outside it too:
what a `requirements` session is for, when a leaf should decompose and which
kinds involve a human are the plugin's and the guide's, and the crate knows only
that a kind is a token and that two of them are reserved.

The corpus is the crate's own manifest and every Rust file under `src/`.
**It includes the inline `#[cfg(test)] mod tests`,
and none of that is excluded**: those tests are owned, reconstructed and explained
by the chapter whose concept each proves, because a test is where several of this
crate's claims are actually held. One file is excluded, `src/task_grow/tests.rs`,
and chapter 10 says what it costs that chapter to have its proof outside its own
pages. The crate's `tests/` directory is evidence throughout and is never reproduced.

The production source is authoritative. Literal fragments in the numbered pages
are copied from it exactly, and the source index records how those fragments
reconstruct each in-scope file. During authoring a scoped check proves the
completed prefix and reports later-owned ranges as deferred; only the final check
proves complete reconstruction of the current corpus listed in the source index.

<a id="reading-fragments"></a>
## Reading fragments

A declaration such as `«library-root»` names one globally unique fragment. A
**literal fragment** carries exact source bytes inside a four-backtick fence;
nothing in it is trimmed, reindented or normalised. A **composite fragment**
contains only whole-line `insert` references and expands them in order. Each
**source root** in the source index expands to one complete production file. A
**`defer` line** reserves an exact source range for a later chapter: it is
planned work, not an unresolved reference, and not reconstructed source.

<a id="contents"></a>
## Contents

1. [Orientation](01-orientation.md)
2. [The tokens, and the four verdicts](02-the-tokens.md)
3. [Kind, slug, handle](03-kind-slug-handle.md)
4. [The name, and canonicity](04-the-name.md)
5. [Opening, contention and refusal](05-opening.md)
6. [Paths, and addressing](06-paths.md)
7. [The walk: pick and select](07-the-walk.md)
8. [Kind, and the brief chain](08-kind-and-briefs.md)
9. [Resolve](09-resolve.md)
10. [Growing: leaf-add and leaf-insert](10-growing.md)
11. [A grove begins](11-a-grove-begins.md)
12. [A leaf becomes a node](12-leaf-to-node.md)
13. [Outcomes are marked in place](13-outcomes.md)
14. [Finishing](14-finishing.md)
15. [The twelve verbs, and the two that are not](15-the-verbs.md)
16. [One live driver per working tree](16-the-lease.md)
17. [Which calls the lease admits](17-the-epoch.md)
18. [Which files take part](18-which-files.md)
19. [The guaranteed core](19-the-core.md)
20. [The loop](20-the-loop.md)
21. [What could not move](21-what-could-not-move.md)

Optional lookup:

- [Concept index](concept-index.md)
- [Source index](source-index.md)

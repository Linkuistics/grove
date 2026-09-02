# grove-llm
<!-- book-page id="contents" role="contents" -->

This book explains the `grove-llm` binary to a reader who knows Rust and
Jujutsu and has driven a grove. grove's own vocabulary is linked to the glossary
rather than re-taught, and the entry point to the system is the
[user guide's account of the `grove-llm` verbs over the tree](../../USAGE.md#usage-tree-verbs),
which says what each of the twelve verbs does to the tree, what it prints and
whether it commits. This reader has run most of those verbs; what they have not
read is the code.

`crates/grove-llm` is four files and 1,017 lines: a manifest, a sixteen-line
library root, a three-line entry point, and one module of 944 lines that is
the whole command surface. Every verb in that module is one call into
`grove_loop::verbs` plus rendering, and the file's own header says what is left
that is not rendering: **order**. Three orders, each stated where it happens —
the operator's text read by the type that owns it before a lock is taken, the
just-in-time presence rule asked before the mutation, and the session admitted
against the completion channel before it is written to. Each chapter opens on
the one thing a thin binary still has to get right at that point in a session,
and the description of the verbs a session runs at that point is that chapter's
body.

**The intended outcome is the what-is-left test, not a reference card.** At the
end you should be able to take a thin command surface over a library in your
own code, ask what is left there that is not rendering, and answer in three
parts: which text is parsed before which lock, which check runs before which
mutation, which admission precedes which signal — and for each, say what
reversing the order costs and name the test that would catch it. All three are
provable inside these 1,017 lines. The stream contract — data on stdout,
advice on stderr, exit zero for information, every refusal carrying its
remedy — is the rendering half of the same thesis, and the last chapter
tabulates it.

**The book's boundary is the call.** It explains one module, `grove_llm::cli`,
and stops at `grove_loop::verbs`. What a verb does to the tree — the walk
`pick` performs, the renumber `leaf-insert` performs, the atomic run `leaf-add`
lands or rolls back, the epoch record admission reads — is another crate's and
is named where a page cannot be followed without it and explained nowhere here;
the last chapter says so in one place rather than apologising per row.

It does not teach Rust or `clap` from first principles. `clap`'s derive macros
and its command model are named where the source names them and explained only
where an argument rests on their behaviour. `flock(2)` semantics are stated
once, in *Growing the tree*, as that chapter's premise. The crate's own `tests/`
directory — twenty-three files, nine times the corpus — is cited as evidence
throughout and is not reproduced: it is outside the corpus this book
reconstructs, and this book cites it more heavily than its predecessors,
because for every promise a verb's help text makes the page names the line in
the handler that keeps it and the test that would catch its breach.

The production source is authoritative. Literal fragments in the numbered pages
are copied from it exactly, and the source index records how those fragments
reconstruct each in-scope file. During authoring a scoped check proves the
completed prefix and reports later-owned ranges as deferred; only the final
check proves complete reconstruction of all four files and 1,017 lines.

<a id="reading-fragments"></a>
## Reading fragments

A declaration such as `«manifest-thin-by-crate»` names one globally unique
fragment. A **literal fragment** carries exact source bytes inside a
four-backtick fence; nothing in it is trimmed, reindented or normalised. A
**composite fragment** contains only whole-line `insert` references and expands
them in order. Each **source root** in the source index expands to one complete
production file. A **`defer` line** reserves an exact source range for a later
chapter: it is planned work, not an unresolved reference, and not reconstructed
source.

<a id="contents"></a>
## Contents

1. [Orientation](01-orientation.md)
2. The grammar and the openings
3. Reading the tree
4. Growing the tree
5. Ending work
6. Leaving the loop
7. What order holds

Optional lookup:

- [Concept index](concept-index.md)
- [Source index](source-index.md)

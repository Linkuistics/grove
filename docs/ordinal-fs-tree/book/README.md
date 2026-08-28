# Ordinal filesystem tree
<!-- book-page id="contents" role="contents" -->

This book explains `ordinal-fs-tree` to readers who already know Rust, common
crates, and operating-system filesystem APIs. It covers the library and its
`syllabus` demonstration consumer: their vocabulary, public seam, read and
mutation flows, filesystem boundary, concurrency, recovery limits, errors, and
trade-offs. It does not teach Rust or filesystem basics, reproduce standalone
tests or formal models, or present the syllabus vocabulary as a library default.

The production source is authoritative. Literal fragments in the numbered pages
are copied from it exactly, and `source-index.md` records how those fragments
reconstruct each in-scope file. During authoring, a scoped check
proves the completed prefix and identifies later-owned ranges as deferred. Only
the final check proves complete reconstruction of all fifteen files.

<a id="reading-fragments"></a>
## Reading fragments

A declaration such as `«library-crate-surface»` names one globally unique
fragment. A literal fragment contains exact source bytes in a four-backtick
fence. A composition expands its whole-line `insert` references in order. Each
source root in the source index expands to one complete production file. A
`defer` line reserves an exact range for a later page; it is planned work, not
an unresolved reference and not reconstructed source.

<a id="contents"></a>
## Contents

1. [Orientation](01-orientation.md)
2. [Name seam](02-name-seam.md)
3. Reference domain
4. Read path
5. Mutation algebra
6. Filesystem interpreter
7. Syllabus CLI
8. Invariants and trade-offs

Optional lookup:

- [Concept index](concept-index.md)
- [Source index](source-index.md)

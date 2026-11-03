# System overview
<!-- book-page id="contents" role="contents" -->

This book explains the `grove` binary to a reader who knows Rust and Jujutsu
and has driven a grove. The book links Grove vocabulary to the glossary. The
[user guide's account of running Grove](../../USAGE.md#usage-running-grove)
provides the product entry point. This book explains the code that implements
its human command surface.

Bare invocation resolves a working tree, takes a driver lease, resolves Ravel
and calls the loop. It installs no skills.
Viewing returns before
that lifecycle.

`grove run` adds a separate lifetime. It has one kind and the command that the
owner's Ravel policy selects for that kind. It runs in private temporary
storage, with staged inputs, checked output publication and a parent-owned
transcript. It needs no workspace or task tree.

The corpus is every production source file and the manifest of `crates/grove`.
The book explains the parser, dispatch, standalone orchestration and display. Library internals remain behind
their public seams: the book supplies the local contract needed to understand
each call, while their own books explain their implementation.

The organising question is which boundary holds each behaviour: a public library
interface, a closed parser surface, an operating-system policy or a checked file
transfer. The lifecycle entry is small. Standalone invocation deliberately owns
artifact and transcript orchestration in this binary. A package boundary limits
access to private library items, but does not by itself prove that a binary
contains no application logic.

The book assumes Rust, clap and ordinary operating-system APIs. It explains the
codebase-specific order of setup, supervision and publication. Tests under
`crates/grove/tests/` are evidence, outside the reconstructed corpus. Literal
fragments quote authoritative production bytes. Final validation checks their
coverage, ownership, reconstruction and links.

<a id="reading-fragments"></a>
## Reading fragments

A declaration such as `«manifest-thin-by-construction»` names one globally
unique fragment. A **literal fragment** carries exact source bytes inside a
four-backtick fence. The book does not trim, reindent or normalise anything in it. A
**composite fragment** contains only whole-line `insert` references and expands
them in order. Each **source root** in the source index expands to one complete
production file. A **`defer` line** reserves an exact source range for a later
chapter: it is planned work, not an unresolved reference, and not reconstructed
source.

<a id="contents"></a>
## Contents

1. [Orientation](01-orientation.md)
2. [The surface](02-the-surface.md)
3. [Lifecycle startup](03-three-steps.md)
4. [Proving a negative](04-proving-a-negative.md)
5. [What the call reaches](05-what-the-call-reaches.md)

6. [One isolated invocation](06-standalone-invocations.md)

Optional lookup:

- [Concept index](concept-index.md)
- [Source index](source-index.md)

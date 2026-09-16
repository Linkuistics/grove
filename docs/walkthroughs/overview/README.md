# System overview
<!-- book-page id="contents" role="contents" -->

This book explains the `grove` binary to a reader who knows Rust and Jujutsu
and has driven a grove. grove's own vocabulary is linked to the glossary rather
than re-taught, and the entry point to the system is the
[user guide's account of running Grove](../../USAGE.md#usage-running-grove) —
start, resume and finish, alongside the [read-only browser](../../USAGE.md#usage-viewing-tree) and
[configuration inspector](../../USAGE.md#usage-inspecting-configuration).

`crates/grove` contains a manifest, an entry point, the CLI dispatcher and a
configuration report formatter. Bare invocation resolves the working tree,
takes the driver lease and calls the loop; observation returns before the lease. That is the whole human entry
point onto grove, and this book is the system's overview rather than a fourth
crate's account because that crate is where the system is entered and nothing
else is decided. Each chapter opens at one of the binary's own steps — the
package it is, the grammar it accepts, the paths it dispatches, the tests that
hold the grammar closed, and the modules the call reaches — and the
description of what the system does at that step is that chapter's body.

**The intended outcome is the thin-entry-point test, not a reference card.** At
the end you should be able to take an entry point in your own code, ask what is
left for an argument to select, and tell a binary that is thin from one that
merely looks thin — naming which of three mechanisms holds it: a package
boundary the compiler enforces, a closure property a test asserts, or a
convention enforced only by a test. The book identifies the evidence for each mechanism.

**The book's boundary is the call.** It explains `grove::cli` and `grove::config`, and
names the libraries they call. Everything behind `grove_tui::run` and `grove_loop::run` — the loop, the task tree,
the kinds, the lifecycle and the session epoch — is named where a page
cannot be followed without it and explained nowhere here; the last chapter
says so in one
place rather than apologising per row.

It does not teach Rust, `clap`, or operating-system signal semantics from first
principles. `clap`'s derive macro and its command model are named where the
source names them and explained only where an argument rests on their
behaviour. Signal semantics are stated once, in *Three steps*, as that chapter's
premise. The crate's own `tests/` directory is cited as evidence throughout and
is not reproduced: it is outside the corpus this book reconstructs.

The production source is authoritative. Literal fragments in the numbered pages
are copied from it exactly, and the source index records how those fragments
reconstruct each in-scope file. During authoring a scoped check proves the
completed prefix and reports later-owned ranges as deferred; only the final
check proves complete reconstruction of every declared source root.

<a id="reading-fragments"></a>
## Reading fragments

A declaration such as `«manifest-thin-by-construction»` names one globally
unique fragment. A **literal fragment** carries exact source bytes inside a
four-backtick fence; nothing in it is trimmed, reindented or normalised. A
**composite fragment** contains only whole-line `insert` references and expands
them in order. Each **source root** in the source index expands to one complete
production file. A **`defer` line** reserves an exact source range for a later
chapter: it is planned work, not an unresolved reference, and not reconstructed
source.

<a id="contents"></a>
## Contents

1. [Orientation](01-orientation.md)
2. [The surface](02-the-surface.md)
3. [Three steps](03-three-steps.md)
4. [Proving a negative](04-proving-a-negative.md)
5. [What the call reaches](05-what-the-call-reaches.md)

Optional lookup:

- [Concept index](concept-index.md)
- [Source index](source-index.md)

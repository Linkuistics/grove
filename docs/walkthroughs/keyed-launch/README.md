# keyed-launch
<!-- book-page id="contents" role="contents" -->

This book explains the `keyed-launch` crate to a reader who knows Rust and
Jujutsu and has driven a grove. grove's own vocabulary is linked to the glossary
rather than re-taught, and the entry point to the system is the
[user guide's account of running a grove](../../USAGE.md#usage-running-grove).
This reader has watched a session end without ever seeing what ended it.

`crates/keyed-launch` is 7 files and 1,688 lines, and it is the layer between a
command its caller built and a running process. The crate keeps the name it took
when it also resolved a key to a command template; that half is deleted, and it
reads no configuration. **Nothing in the crate understands what it launches.**
Each chapter opens on the one thing this stage must not add and must not
interpret — one dependency and no domain; a file whose *appearance* is the
event; a child handed nothing the caller did not write; an ending the launcher
must perform because the child cannot; and a name held to a grammar without
anyone asking what it refers to.

**The intended outcome is the pass-through test, not a reference card.** At the
end you should be able to take any layer in your own code that carries a value
to an effect in the world — a command to a process, a query to an engine, a
route to a handler — and ask *where does this layer learn what the value means?*
The right answer is nowhere. This crate's part of that test is **the way out**:
inferring what came back, or adding to the launch what the caller did not write.
The assembly chapter applies it to the preceding stages. The final chapter adds
the explicit noninteractive and native confinement boundaries.

**The book's boundary is the argv it was handed and the process it spawned.** It
explains one crate and stops there. What a session *is*, what a kind means, the
driver lease, the task tree and the methodology are grove's, not this crate's,
and the account of what a launch is for lives in the
[user guide's session lifecycle](../../USAGE.md#usage-session-lifecycle), where
this book points once so that no chapter has to. A book that explained grove's
sessions would have documented the wrong crate.

It does not teach Rust or libc: `signal(2)`, `setpgid(2)`, `tcsetpgrp(2)`,
`execve(2)` and `getpgrp(2)` are used and their *consequences* are argued at
length, because the consequences are the design, while their signatures and
portability are the operating system's documentation. The crate is Unix-only by
construction and no chapter treats portability as an open question. The crate's
own `tests/` directory is cited as evidence throughout and is not reproduced: it
is outside the corpus this book reconstructs. The eleven tests the book *does*
reproduce are the inline module inside `src/channel.rs`, which are corpus because
a root is `src/**/*.rs`.

The production source is authoritative. Literal fragments in the numbered pages
are copied from it exactly, and the source index records how those fragments
reconstruct each in-scope file. During authoring a scoped check proves the
completed prefix and reports later-owned ranges as deferred; only the final check
proves complete reconstruction of all 7 files and 1,688 lines.

<a id="reading-fragments"></a>
## Reading fragments

A declaration such as `«manifest-one-dependency»` names one globally unique
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
2. [Appearance is the event](02-the-channel.md)
3. [The child is a job](03-the-job.md)
4. [The watch and the escalation](04-the-escalation.md)
5. [How this is checked](05-how-checked.md)
6. [What passes through](06-what-passes-through.md)
7. [Confined noninteractive jobs](07-confined-jobs.md)

Optional lookup:

- [Concept index](concept-index.md)
- [Source index](source-index.md)

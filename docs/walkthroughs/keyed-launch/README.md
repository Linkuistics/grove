# keyed-launch
<!-- book-page id="contents" role="contents" -->

This book explains the `keyed-launch` crate to a reader who knows Rust and
Jujutsu and has driven a grove. grove's own vocabulary is linked to the glossary
rather than re-taught, and the entry point to the system is the
[user guide's account of running a grove](../../USAGE.md#usage-running-grove),
whose statement that full configuration validation precedes every tree mutation —
so a missing or malformed `config.kdl` leaves your working tree byte-identical —
is this crate's central property seen from the operator's side. This reader has
written a `config.kdl`, has had grove refuse a malformed one, and has watched a
session end without ever seeing what ended it.

`crates/keyed-launch` is 10 files and 2,753 lines, and it is the layer between
a human's configuration file and a running process. A consumer names a key; a
template names a program. **Nothing in the crate understands either.** A key is
an opaque string, a slot is a name the consumer declares, and the words of a
template are the words the file holds. Each chapter opens on the one thing this
stage must not add and must not interpret — three dependencies and no domain; a
rule about a slot's *name*; a launch read whole out of one file; a word that is a
word; substitution that is whole-word or nothing; a file whose *appearance* is
the event; a child handed nothing the operator did not write; an ending the
launcher must perform because the child cannot; and a contract checked without
either side knowing what a key is for.

**The intended outcome is the pass-through test, not a reference card.** At the
end you should be able to take any layer in your own code that carries a value
from a human's file to an effect in the world — a configuration to a process, a
query to an engine, a route to a handler — and ask *where does this layer learn
what the value means?* The right answer is nowhere, and you should be able to
name the three places such a layer usually learns it anyway, what each costs, and
the test that catches it: **on the way in**, by assembling one value out of more
than one source; **on the way through**, by re-reading a value it has already
read; and **on the way out**, by inferring what came back or adding to the launch
what the operator did not write. All three are provable inside these 2,753 lines,
and the closing chapter states the test and applies it to all nine source-owning
chapters.

**The book's boundary is the words in the file and the process it spawned.** It
explains one crate and stops there. What a session *is*, what a kind means, the
driver lease, the task tree and the methodology are grove's, not this crate's:
the mapping between them is one sentence in chapter 1 — a session kind is a
key — and the account of what a launch is for lives in the
[user guide's session lifecycle](../../USAGE.md#usage-session-lifecycle), where
this book points once so that no chapter has to. A book that explained grove's
sessions would have documented the wrong crate.

It does not teach KDL or shell-word splitting as formats. `kdl` parses the
document and `shell-words` splits the line; the book explains what the crate asks
of each and what it adds on top — the node shape it requires, and the
comment-start scan it performs because `shell-words` would otherwise silently
truncate an argv — and never the crates themselves. It does not teach Rust or
libc: `signal(2)`, `setpgid(2)`, `tcsetpgrp(2)`, `execve(2)` and `getpgrp(2)` are
used and their *consequences* are argued at length, because the consequences are
the design, while their signatures and portability are the operating system's
documentation. The crate is Unix-only by construction and no chapter treats
portability as an open question. The crate's own `tests/` directory
is cited as evidence throughout and is not reproduced: it is
outside the corpus this book reconstructs. The nine tests the book *does*
reproduce are the inline module inside `src/channel.rs`, which are corpus because
a root is `src/**/*.rs`.

The production source is authoritative. Literal fragments in the numbered pages
are copied from it exactly, and the source index records how those fragments
reconstruct each in-scope file. During authoring a scoped check proves the
completed prefix and reports later-owned ranges as deferred; only the final check
proves complete reconstruction of all 10 files and 2,753 lines.

<a id="reading-fragments"></a>
## Reading fragments

A declaration such as `«manifest-three-dependencies»` names one globally unique
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
2. [The names a template is written against](02-the-names.md)
3. [Two documents, neither one assembled](03-two-documents.md)
4. [What a template must be](04-template-law.md)
5. [From a template to an argv](05-to-an-argv.md)
6. [Appearance is the event](06-the-channel.md)
7. [The child is a job](07-the-job.md)
8. [The watch and the escalation](08-the-escalation.md)
9. [How this is checked](09-how-checked.md)
10. [What passes through](10-what-passes-through.md)

Optional lookup:

- [Concept index](concept-index.md)
- [Source index](source-index.md)

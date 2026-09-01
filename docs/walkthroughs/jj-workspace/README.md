# jj-workspace
<!-- book-page id="contents" role="contents" -->

This book explains the `jj-workspace` crate to a reader who knows Rust and
Jujutsu and has driven a grove. Grove's own vocabulary is linked to the glossary
rather than re-taught, and the entry point to the system this crate serves is the
[user guide's account of workspace layouts](../../USAGE.md#usage-workspace-layouts),
whose three cases — native, colocated and secondary — are exactly the ones this
crate's resolution has to answer for.

The crate resolves a Jujutsu workspace, refuses a working tree that is not one,
reserves a control namespace for its consumer, and takes a path-scoped commit.
The book covers all of it: the manifest, the gate, the child-process seam, the
namespace, the commit algebra, and the ten refusals. Every chapter opens on
something the crate declines to own and names who owns it instead.

**The intended outcome is a transferable test, not a reference card.** At the end
you should be able to take a boundary in your own code, ask what the tool
underneath already owns, and tell a justified subtraction from an abdication.

It does not teach Rust, Jujutsu, or operating-system behaviour from first
principles. Where a chapter's argument rests on a specific jj behaviour, that
behaviour is stated once on that page as the chapter's premise, with a link to
jj's own documentation. The crate's tests are cited as evidence throughout and are
not reproduced: they are outside the corpus this book reconstructs. The
consumers' own source — grove's loop driver and its CLI — is named by path and
never reproduced, because those bytes belong to other books.

The production source is authoritative. Literal fragments in the numbered pages
are copied from it exactly, and the source index records how
those fragments reconstruct each in-scope file. During authoring a scoped check
proves the completed prefix and reports later-owned ranges as deferred; only the
final check proves complete reconstruction of all four files and 698 lines.

<a id="reading-fragments"></a>
## Reading fragments

A declaration such as `«library-crate-thesis»` names one globally unique
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
2. [The gate](02-the-gate.md)
3. [The subprocess seam](03-subprocess-seam.md)
4. [The namespace it will not name](04-namespace.md)
5. [Scope and commit](05-scope-and-commit.md)
6. [Refusal](06-refusal.md)
7. [What jj owns](07-what-jj-owns.md)

Optional lookup:

- [Concept index](concept-index.md)
- [Source index](source-index.md)

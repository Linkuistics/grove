# A witnessless root refuses what it cannot account for

Grove classifies a present root from names alone. A format witness and file
contents supply no ownership evidence. The reader validates every reachable
level before classification; malformed names, missing node files and competing
node files are refused there. Among successfully opened roots, classification
is ordered:

1. **Taskless**: nothing but `_BRIEF.md`. Refuse,
   naming the missing work and the required root form.
2. **ATree**: at least one validated positioned name. Proceed with the snapshot.
3. **Unrecognised**: `_BRIEF.md` plus foreign names and no positioned work.
   Refuse without mutation,
   naming the disclaimed entries and the grammar Grove accepts.

A node file is not evidence of a task. Its body can be a copied charter and its
name carries no key. An invalid `_` file is nevertheless owned by the grammar
and must reach a malformed-name or malformed-level error; it must not turn a
root into an apparently empty grove. A taskless root with foreign entries is
unrecognised, while any owned malformed name reaches the reader's refusal.

Initialization writes the root, `_BRIEF.md` and the first leaf under the one
exclusive guard that observed the vacancy. Reported failures unwind that work;
process death can leave an incomplete shape, which is refused on the next open.
A root holding only `_BRIEF.md` has a valid node-file shape and no work; a root
without `_BRIEF.md` has an invalid node-file shape, whether or not it contains
work. An empty or foreign-only root therefore fails before classification.
None is a finish signal.

## The trade-off

Automatically completing a root from the presence of a charter proves only
that the bytes to write are predictable. It does not prove that this directory
is Grove's to modify. The library therefore performs no repair, and the
operator restores the missing entries or moves the directory aside before
starting a fresh grove.

Treating foreign-only contents as an empty tree would silently finish work
Grove cannot interpret. Treating every unknown entry as owned would instead
prevent the domain from disclaiming ordinary foreign files. The ordered test
keeps both distinctions.

Reopen automatic completion only if a separate ownership guarantee can establish
that this particular incomplete root belongs to this operation. A charter's
contents and an advisory lock do not establish that guarantee.

[`entries-are-never-removed`](entries-are-never-removed.md) explains why a
completed task remains visible. The lifecycle tests exercise these three
outcomes and verify that refusals leave the tree unchanged.

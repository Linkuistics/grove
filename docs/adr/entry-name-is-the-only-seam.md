# The entry name is the only seam

Every point at which `ordinal-fs-tree` is parameterised by its consumer is one
trait over the **entry name**. There are no callbacks, no hooks, no registration
and no configuration objects, and there is no `Domain` type: the name type owns
its own parsing, validation and formatting, reserved names are variants of it,
and everything the library does not understand travels as one opaque associated
type. Locking and version control are not in the trait because they are not
domain questions — the library locks the directory containing the tree root
whatever the domain is, and a rename is `rename(2)`.

`EntryNameExt` is a second trait and not a second seam: it is
blanket-implemented for every `EntryName` and sealed, so nothing outside the
library can implement it and no consumer chooses anything by it. It exists
because two of the obligations are discharged by being *derived* — a name's
triple and its species are read off `view` and `positioned_species` — and a
provided method on `EntryName` could be overridden, which would hand back the
two defects the shape was chosen to remove.

[`docs/ordinal-fs-tree/ARCHITECTURE.md`](../ordinal-fs-tree/ARCHITECTURE.md)
carries the trait itself, the seven obligations an implementation must meet, and
the name/`(ordinal, key, parts)` isomorphism the seam rests on. This record
carries what that document does not: the shapes that were rejected on the way to
it, and what it would cost to change course.

## The trade-off

A seam this narrow buys three things at once, and each of them is a property
something can check rather than a claim.

**The library holds no strings.** The algebra works on triples, so both formal
models under `docs/ordinal-fs-tree/models/` contain no strings at all and the
entire grammar reduces to one round-trip law. A seam that admitted a second
domain concern would put something in the state that neither model could
abstract away.

That claim has exactly one qualification, and it is worth stating here because
it is where the claim is made. The *filesystem* layer does look at one string:
the name's rendering, which is what it joins to a level's directory. So the
seventh obligation — a name renders as one path component — is the one the
library enforces rather than assumes, at the two boundaries where a name becomes
a path. It is not a second seam and not a widening of this one: a domain supplies
no more than it did, and what changed is that the library stopped trusting the
one value it was already using. `interpreter-k21` found the hole and
`docs/formalism-findings.md` entry 011 records why it was invisible to both
models — a rendering that leaves the tree is not a thing either can say.

**The sibling shift is derived rather than implemented.** Shifting is
`compose(new_ordinal, key, parts)` and nothing else, so it is structurally
incapable of disturbing a key, a label or an attribute. That is only true while
`compose` is the sole way to build a positioned name.

**No `D` propagates.** With one trait on the name rather than a domain type
threaded through the tree, the spurious `D: Clone` derive bounds a
domain-parameterised design accumulates never arise.

What it costs is that a domain needing genuinely per-domain *behaviour* — a
different lock scope, a version-control-aware move — cannot express it. That is
paid deliberately: those two were examined and neither generalises the way a
domain hook implies. The containing directory outlives both the root's creation
and its deletion in every domain, and a rename that a version control system
must be told about commits a byte-identical tree either way.

## Considered options

- **A `Domain` trait with associated functions for lock scope and moving.**
  Rejected: these are the callbacks the design rules out, respelled as a trait.
  Each of the two behaviours it would carry was examined on its own and found to
  have one right answer for every domain, so the trait would exist to make a
  settled question configurable. It also puts `D` in every type. Reopen if a
  consumer appears whose lock scope genuinely cannot be the containing
  directory — a tree whose root is a mount point, say — which would make the
  scope a real domain input rather than a general rule.
- **A two-trait split by layer** — one trait for the grammar, one for
  everything above it. Rejected because the layers are not independent: the
  species follows from the parts, so a grammar trait that did not also answer
  *what species is this* would leave the upper trait re-deriving it, and the two
  would drift. Reopen if a consumer needs one grammar under two different
  higher-level behaviours, which is the case the split would actually serve.
- **A name that is a plain string, parsed by the library against a supplied
  grammar description.** Rejected: it makes the library hold strings, puts a
  grammar language in the interface, and moves reserved-name handling — which
  must carry the domain's own recovery advice — into a layer that has no domain
  errors to raise. Reopen never; it is the design being replaced.

## Why this is hard to reverse

The trait *is* the library's public surface, so changing its shape changes every
consumer, not merely the library. Two checked models and the architecture
document are stated in its terms, and the isomorphism that makes the shift
derivable is a property of *this* trait rather than of the design in general. A
later split of the crate into separately modellable units is mechanical only
while the algebra stays free of `std::fs`, which the single-trait shape is what
makes possible.

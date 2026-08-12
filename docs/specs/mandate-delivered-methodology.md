# mandate-delivered-methodology

## Problem

A Grove session is told almost nothing and expected to derive the rest. Today's
mandate is roughly 1.1 kB — one embedded launcher, the selected stable handle,
and the resolved version control — and everything else the session needs it must
find for itself in a 51 kB `SKILL.md` sitting in a global skill directory some
other build may have written.

Three costs follow, in the order they matter.

**Reasoning not performed.** A session that is told a fact succinctly never runs
the derivation that would have established it. That derivation is the largest of
the three costs and the only one that scales with the session's difficulty, not
with the document's size.

**Methodology prose not written.** Every fact the driver states is a paragraph
the methodology no longer has to teach a session how to work out, paid back in
every session forever.

**Bytes not read.** The smallest of the three, and never on its own a reason to
move anything: turn count alone would justify inlining 46 kB of glossary into
argv.

The failure this creates when it is done badly is **silent and behavioural**.
Withhold a procedure and the session knows it is missing something and asks.
Withhold a *condition* and the session never learns there was a question — it
absorbs work that should have become its own leaf, and nothing errors, nothing
exits non-zero, and no reviewer sees a diff. That is Grove's stated primary
failure mode, and any design that delivers methodology selectively has to
convert it into a structural failure or it is not safe to ship.

The current delivery path also cannot be made specific. `content/` reaches a
session as whole documents in a shared directory, so the only granularity
available is "the file" — while the granularity the session needs is "this one
bullet, out of nineteen". Pointing at a location leaves the session reading the
whole section and performing the selection itself, which is precisely the
reasoning cost being removed.

## Solution

The driver composes each session's `${prompt}` from **byte-exact slices of its
own embedded `content/`**, selected by the launched session kind. The mandate
becomes the sole delivery path for the methodology; global skill provisioning
retires.

Three properties make that safe.

**Slice, never paraphrase.** A slice is the source bytes of a marked unit of
`content/`, copied verbatim. Driver-composed prose would make `content/`
non-canonical and create a second source of truth that drifts across the build
boundary; a byte-exact projection cannot contradict what it copies.

**Keep the `if`, defer the `then`.** Every rule in the methodology is a
conditional. Its **condition** — that a situation exists calling for something
other than what this session is doing — is *triggering* and ships in every
mandate the unit is scoped to. Its **body** — how to act once that is decided —
is *procedural*, ships in no mandate, and is served on demand by a new
`grove-llm methodology` verb reading the binary's own embed. The asymmetry is the
whole argument: withholding a procedural body costs a lookup the session knows to
make, while withholding a triggering condition yields an unasked question.

**The classification is data, marked in `content/` beside the prose it
classifies.** Adjacency is what keeps it true when someone edits the prose, and
data is what makes the split mechanically checkable. That converts the silent
behavioural failure into a structural one: what can go wrong narrows from "did we
build the mandate right?" to "did we classify this one unit right?", and the
residue gets an adversarial review pass.

`content/` stays a set of whole, readable markdown documents. The markers are
HTML comments and per-file frontmatter, so nothing about reading `content/` as
prose changes.

## Decisions

### Units partition a file; they are not islands in it

Every byte of an embedded markdown file's body belongs to **exactly one** unit.
A unit begins at its marker and runs to the byte before the next marker, or to
end of file. There are no gaps, no nesting, and no close markers.

This is the design's load-bearing structural choice, and it is not tidiness.
If units were islands, prose outside every unit would be unclassified and
invisible to both the parser and the completeness invariant — which is exactly
the hole the whole design exists to close. Under total partition, unclassified
prose cannot exist.

It also converts the parser's own worst failure into a visible one. A parser that
goes blind to some marker shape does not silently drop a unit; it makes the
**preceding** unit absorb the text, which changes that unit's bytes and moves the
golden snapshots. A blind island-parser reads exactly like a clean file.

Nesting is rejected because it makes "the unit's bytes" ambiguous — an outer unit
either contains its children's bytes or does not, and both answers break a
byte-exact slice. Close markers are rejected because they permit gaps, double the
edit surface at every insertion point, and add an unclosed-unit failure mode that
total partition does not have.

The marker line is **part of the unit's source**, so a slice arriving in a mandate
carries its own id. Addressing is therefore self-documenting at zero
driver-authored cost: a session reading a triggering slice can see the id to pass
to `grove-llm methodology` for the deferred half.

### The marker grammar

```
<!-- unit: <id> kinds=<scope> class=triggering -->
<!-- unit: <id> class=procedural -->
```

- **`<id>`** is kebab-case and **unique across the whole embed**, not merely
  within its file. One namespace, because `grove-llm methodology` addresses units
  by id alone.
- **`<scope>`** is `*` or a space-separated list of session-kind labels, quoted
  when it has more than one member. Every member is validated against the closed
  set of nineteen.
- **`class`** is `triggering` or `procedural`.
- **`kinds` is required on a triggering unit and forbidden on a procedural one.**
  A procedural unit ships to no mandate, so a scope there would be a lie; making
  it an error rather than an ignored field keeps every marker's meaning readable
  off the marker.
- **Attribute order is fixed.** Free order costs nothing to parse but this is
  content a human must audit line by line across the whole embed, and a uniform
  column layout is what makes that audit fast.

Markers inside fenced code blocks are **not** markers. The methodology documents
itself, so example markers inside fences are certain to appear; the rule is
pinned by test on the exact shapes that decide it.

### `kinds=` admits `*` and explicit lists, and nothing else

No family shorthand (`producers`, `reviews`) and no negation.

`*` is the overwhelming default: a triggering condition is triggering for every
kind unless it is genuinely about one kind's discipline. The lists actually
written are therefore small — `kinds=impl`, `kinds=finish` — and the
nineteen-member list nobody wants to read is spelled `*`.

A shorthand is a second grammar to learn, but the decisive objection is that it
**silently absorbs a kind added later**. That is the failure
[complete session configuration](../adr/complete-session-configuration.md)
designed against when it made the configuration repeat all nineteen targets so
that adding a kind fails visibly in every old config. A `kinds=producers` unit
would change what it ships the day a twentieth producer is added, with no diff for
anyone to review. A complement (`kinds=!finish`) has the same defect and no case
that needs it: guidance that applies to every kind including `finish` is `*`, and
guidance that does not is a short list.

### A malformed embed fails the build

`build.rs` parses `content/` and fails `cargo build`. The parser is one
implementation shared with the crate rather than a second traversal, because a
duplicated reader is the drift the equality tests in this repository already
exist to prevent.

Constraint 5 — Grove guides and does not gate — governs **the human's task
tree**. It does not govern Grove's own compile-time artifact, and the costs here
are wildly asymmetric. A hard failure inconveniences a Grove contributor at build
time, in the repository where the mistake was made, with the file and offset in
hand. A soft failure ships a binary that silently drops a triggering unit from
every session it ever launches. There is no proxy and no opaque target in this
question: the embed is fully observable by the very build that produced it, which
is what separates it from the launch-time checks that deliberately report rather
than refuse.

Failing the *driver* instead would be the worst of both — the fact is visible at
compile time, so deferring it converts a contributor's build error into a
stranger's stalled loop.

Three classes of malformation fail there: syntax (unparseable marker, unknown
attribute, missing `class`, `kinds` on a procedural unit), semantics (duplicate
unit id anywhere in the embed, body text before the first marker, duplicate
file-ordering key), and reference (a `kinds=` member that is not one of the
nineteen).

"Unknown unit id" is deliberately **not** in that list. Ids are declared by
markers and referenced by nobody inside `content/`, so an unknown id can only
arise as an argument to `grove-llm methodology` — an ordinary runtime user error,
answered with a message naming the id and the available set.

### Per-file frontmatter carries the file's mandate order

Each embedded markdown file opens with `---`-delimited **KDL** frontmatter. KDL
because it keeps the repository to one metadata language — the session
configuration parser already owns one. `---` delimiters because it is the shape
the files already use and the diff is minimal.

It carries one field: the file's position in mandate composition. Ordering has to
live somewhere, and putting it in `content/` rather than in a file list in Rust
keeps the driver from owning a fact about content's presentation. Duplicate
positions are a build error, because the composition order must be total.

Frontmatter is **required on every embedded markdown file**. That is what makes
"an unmarked file contributes no units" unreachable — the silent hole in file
form.

`content/SKILL.md`'s current YAML frontmatter exists solely for harness skill
discovery. Retiring provisioning frees that slot: once nothing provisions the
file, no parser depends on it.

### `content/prompts/continue.md` becomes `content/MANDATE.md`

Its "use the grove skill" instruction is false the moment provisioning retires,
so the file cannot survive unchanged. What survives is its **job**: framing. A
mandate is a wall of sliced methodology, and the session needs to know what that
text is, that it is complete with respect to triggering conditions, and that
`grove-llm methodology` serves the rest.

That framing is methodology, so it belongs in `content/` as an ordinary marked
unit — `class=triggering kinds=*`, with the file ordered first — and not in a
driver-side format string, where it would be the one piece of methodology the
completeness invariant cannot see. It stops being a special case: the composer
needs no rule to place it, because the file ordering already does.

`content/prompts/` is removed. It held three launchers; two went with their
lifecycle verbs, and this is the third.

### The driver authors mandate prose only for facts it resolves at runtime

Everything static is a slice. Today exactly two facts are not static: the
selected leaf's stable handle, and the resolved version control. Both are
resolved before the session exists, neither is expressible in `content/`, and
both stay driver-authored.

That rule is the whole answer to what may introduce, separate, or frame a slice:
nothing may. Slices are joined by a blank line and no more. Every framing
sentence a driver writes is methodology living in Rust and therefore a drift
candidate, and this rule reduces the set of them to the facts that have no other
home.

Composition order is: the mandate preamble, then every triggering unit whose
scope admits this kind — ordered by file position, then by position within the
file — then the runtime facts. The session-specific instructions land last, where
they are not buried under the generic bulk.

### `grove-llm` links the embed, and the methodology identity simplifies

`grove-llm methodology` serves unit bytes from **its own** embed, so the
agent-facing binary starts linking `content/`. Two consequences follow.

The release path's binary scan inverts. It currently fails a release if
`grove-llm` contains the content marker; it must assert that **both** binaries
carry it. This is a release-path check, not a `cargo test` one, so it fails at
the first release cut rather than during development — it is named here so that
it is scheduled rather than discovered.

The compile-time methodology-identity constant loses its reason to exist. It is a
constant precisely so that naming the identity does not link the embed; once
`grove-llm` links the embed anyway, both binaries can hash it directly. That
removes a build-script traversal, its hash dependency, and the equality test that
existed only to keep two traversals in step. `build.rs` still walks `content/` —
for the parse gate, and to emit change tracking for the embed macro — but it no
longer hashes.

### Build pairing survives, sharpened

Under provisioning the skew was between two copies of a *whole* methodology: a
skill directory some other build had written, and the CLI a session resolved.
Mandate delivery removes that vector entirely — there is no shared mutable
directory, and the methodology arrives in argv from the driver that composed it.

What replaces it is narrower and sharper. The triggering half of a rule reaches
the session from the **driver's** embed; the deferred procedural half comes from
whichever `grove-llm` the session's `PATH` resolves. A mismatched pair is a
split-brain *inside one rule*, not two copies of one document.

That failure is also **loud**, which is a genuine improvement: a mandate that
supplies a unit id the session's `grove-llm` does not know produces an error
naming the id, at the moment it matters, rather than a silent divergence. The
pre-launch report survives on its existing argument — an opaque configured
command's environment is not the driver's to observe, so the probe reports and
never refuses — and the in-session stamp warning disappears with the stamps it
read. See [one build owns a session](../adr/one-build-owns-a-session.md).

### Mandate size is a fact, not a design constraint

`ARG_MAX` is 1 MiB on macOS, shared with the environment, and `${prompt}` is
expanded into argv and executed directly with no shell. The triggering share is
expected to be low tens of kilobytes against that ceiling — two orders of
magnitude of headroom — so the ceiling constrains nothing here.

A per-kind size bound is nonetheless asserted, framed honestly as what it is: a
**classification alarm**. A composed mandate that approaches the whole of
`SKILL.md` means someone has classified procedural bodies as triggering, and that
is worth failing on long before argv is at risk.

## Requirements

### Requirement: Every embedded markdown file is fully classified

The build SHALL reject any embedded markdown file that lacks frontmatter, that
has body text before its first unit marker, or that carries a malformed marker.

#### Scenario: unmarked file
- **WHEN** an embedded markdown file carries no unit marker
- **THEN** the build fails, naming the file

#### Scenario: text before the first marker
- **WHEN** body text appears between the frontmatter and the first marker
- **THEN** the build fails, naming the file and the offset

#### Scenario: duplicate id across files
- **WHEN** two units in different files declare the same id
- **THEN** the build fails, naming both files

### Requirement: Triggering units reach every kind they are scoped to

The composer SHALL include, in the mandate for a given session kind, exactly
those triggering units whose scope admits that kind, and no procedural unit.

#### Scenario: universal scope
- **WHEN** a unit is `class=triggering kinds=*`
- **THEN** it appears in the composed mandate of all nineteen kinds

#### Scenario: narrowed scope
- **WHEN** a unit is `class=triggering kinds=impl`
- **THEN** it appears in the `impl` mandate and in no other kind's mandate

#### Scenario: procedural exclusion
- **WHEN** a unit is `class=procedural`
- **THEN** it appears in no kind's composed mandate

### Requirement: Every unit is reachable by id

`grove-llm methodology` SHALL serve any unit's source bytes by id and SHALL list
the available units when given no argument.

#### Scenario: fetch by id
- **WHEN** the verb is given one or more known unit ids
- **THEN** it writes those units' source bytes, in the order given

#### Scenario: unknown id
- **WHEN** the verb is given an id no unit declares
- **THEN** it exits non-zero with a message naming the unknown id and directing
  the caller to the listing

#### Scenario: listing
- **WHEN** the verb is given no argument
- **THEN** it lists every unit's id, class, scope, and source file

### Requirement: Slices are byte-exact

A composed mandate SHALL contain each selected unit's source bytes unmodified,
including its marker line, joined only by blank lines and followed only by facts
the driver resolved at runtime.

#### Scenario: verbatim projection
- **WHEN** a unit is selected for a mandate
- **THEN** the mandate contains that unit's bytes with no paraphrase,
  truncation, re-wrapping, or driver-authored introduction

## Test seams

One new module seam, `methodology`, exposing the parse function over
`(path, text)`, the embed's unit set, and the composition function over
`(units, kind)`. Every interesting test runs through it against fixture strings;
the `grove-llm methodology` verb and the driver's mandate construction are thin
wrappers covered by the existing command-level and loop-driver seams. This adds
one row to the architecture's module-seam table and no new public surface beyond
it.

A separate composer seam was considered and rejected: the composer's fixtures
come from the parser's output, so the boundary would not buy isolation. Testing
only through the CLI and the driver was also rejected — every parser edge case
would then need a spawned process to provoke.

The checks that seam carries:

- **Parse shapes, pinned on every form that decides the reading rule** — accepted
  and rejected alike, including markers inside fenced blocks. The repository's
  existing instructed-verb scanner is the precedent: its reading rule is pinned
  on the shapes it must see *and* the shapes it must ignore, because two of those
  were live holes rather than hypotheticals.
- **The malformed cases**, each asserted to fail: every syntax, semantic, and
  reference error listed above.
- **The completeness invariant** — every triggering unit appears in the composed
  mandate of every kind its scope admits, every procedural unit appears in none,
  and every unit is reachable by id.
- **A positive control, pinned complete rather than floored.** The full set of
  unit ids is a test constant. Losing any unit fails; gaining one fails until
  someone names it, which is exactly when to confirm the classification. A floor
  bounds only how much the scan may quietly lose, and that slack is enough to
  defeat the universal claim — the same argument the instructed-verb set already
  makes at length.
- **The classifier must be able to fail**, demonstrated on a synthetic malformed
  marker and a synthetic well-formed one. A well-formed pattern matching nothing
  reads exactly like a clean repository.
- **Golden per-kind mandate snapshots**, for drift. They say nothing about
  correctness; they say loudly that something moved.
- **The per-kind size alarm**, as specified above.

Two existing checks **relocate rather than die** when provisioning retires: the
scan asserting the embedded methodology instructs no `grove-llm` verb the
embedded CLI lacks, and the flat-verb-surface pin that makes that comparison
mean what it claims. Both are claims about the embed, not about provisioning,
and the architecture already names the first as the enforceable half of the
build boundary. Their corpus improves in the move: they will scan the embed
itself rather than a provisioned extraction of it.

## Out of scope

- **Behavioural evaluation of the composed mandate.** Rejected during grilling:
  expensive, non-deterministic, it measures a model rather than Grove's artifact,
  and it localizes nothing when red. The honest behavioural check is the next
  real Grove run after the change lands, with a human watching.
- **A `--kind` filter or JSON output on `grove-llm methodology`.** The consumer
  is an agent reading text, the golden snapshots exercise composition through the
  module seam without a process, and every flag is surface to keep in step.
  Reopen if a human needs to audit a composed mandate without reading a snapshot
  file.
- **A templating engine.** One was accepted in principle to keep `content/`
  readable from the code, but byte-exact slicing reaches that without one. No
  engine is adopted to satisfy a sentence.
- **Making the `linkuistics` plugin sliceable.** ADR philosophy, seam judgement
  and the Jujutsu lane live in a separately installed plugin, are not embedded,
  and are therefore not sliceable. Extending the packaging goal to them is a
  different and larger shape.
- **Anything that lets a session consume freshly committed methodology ahead of
  its binary.** The build boundary is deliberate and unchanged: a session reads
  the methodology its own build carries. This design narrows the boundary's
  exposure by removing the shared directory; it does not move the boundary.

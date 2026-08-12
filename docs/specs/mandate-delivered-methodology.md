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
make, while withholding a triggering condition yields an unasked question. That
lookup is **addressed, not guessed** — a marker names the procedural unit it
defers to, and the name arrives with the slice — which is what turns "the session
knows to make it" from an assumption into a property.

**The classification is data, marked in `content/` beside the prose it
classifies.** Adjacency is what keeps it true when someone edits the prose, and
data is what makes the split mechanically checkable. That converts the silent
behavioural failure into a structural one: what can go wrong narrows from "did we
build the mandate right?" to "did we classify this one unit right?", and the
residue gets an adversarial review pass.

`content/` stays a set of whole, readable markdown documents. The markers are
HTML comments, so nothing about reading `content/` as prose changes — and a file
that already opens with a `---`-delimited block keeps it, untouched and unread.

## Decisions

### Units partition a file; they are not islands in it

Every byte of an embedded markdown file's body belongs to **exactly one** unit.
A unit begins at its marker and runs to the byte before the next marker, or to
end of file. There are no gaps, no nesting, and no close markers. *Body* here
means everything after the optional leading `---`-delimited preamble, which is
the file's only region no unit covers and the parser's only unread bytes — see
*A leading `---` block is opaque preamble* below.

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
carries its own marker — its id, and the id of the procedural unit it defers to
where it has one. Addressing is therefore self-documenting at zero
driver-authored cost: a session reading a triggering slice reads the id to pass to
`grove-llm methodology` straight off the text it is already holding.

### The marker grammar

```
<!-- unit: <id> kinds=<scope> class=triggering -->
<!-- unit: <id> kinds=<scope> class=triggering defers=<target> -->
<!-- unit: <id> class=procedural -->
<!-- unit: <id> class=procedural defers=<target> -->
```

- **`<id>`** is kebab-case and **unique across the whole embed**, not merely
  within its file. One namespace, because `grove-llm methodology` addresses units
  by id alone.
- **`<scope>`** is `*` or a space-separated list of session-kind labels, quoted
  when it has more than one member. Every member is validated against the closed
  set of nineteen.
- **`class`** is `triggering` or `procedural`.
- **`<target>`** is one unit id, or a quoted space-separated list of them — the
  same shape as `<scope>`, because a second list grammar would be a second thing
  to learn for no gain.
- **`kinds` is required on a triggering unit and forbidden on a procedural one.**
  A procedural unit ships to no mandate, so a scope there would be a lie; making
  it an error rather than an ignored field keeps every marker's meaning readable
  off the marker.
- **`defers` is optional on either class.** Its absence is meaningful: a unit with
  no `defers=` is complete as delivered, which is how a session knows *not* to go
  looking for more.
- **Attribute order is fixed** — `id`, then `kinds`, then `class`, then `defers`.
  Free order costs nothing to parse but this is content a human must audit line by
  line across the whole embed, and a uniform layout is what makes that audit fast.

### A unit names the procedure it defers to

`defers=` is what makes "keep the `if`, defer the `then`" operable rather than
merely stated. Without it a session holding a triggering slice has no way to reach
its own deferred body: ids are unique across both classes, so the slice's own id
addresses the slice, and asking `grove-llm methodology` for it returns the
condition the session has already read. The relationship has to be **declared**,
and the marker is where it goes — adjacent to the prose, for the same reason the
classification is.

Four rules make the declaration mechanical rather than a comment:

- **Every `defers=` member names a declared unit**, and that unit's class is
  `procedural`. Both are build errors, checked across the whole embed.
- **Deferral may chain.** A procedural body that itself states a condition may
  defer onward, and the fetched bytes carry that marker exactly as a mandate
  carries a triggering one, so the session sees the next id the same way it saw
  the first. Forbidding chains would buy nothing — the graph is checked at every
  level — and would force procedures to merge for the grammar's convenience.
- **Every procedural unit is reachable from some kind's mandate** by following
  `defers=` from a triggering unit that mandate carries. This is the mirror of
  total partition, and it closes the mirror hole. Partition makes unclassified
  prose impossible; reachability makes an *undiscoverable procedure* impossible —
  a body no session can be told about is deleted from the methodology as surely as
  prose no parser can see, and just as silently.
- **No chain of deferrals returns to a unit it has already passed through.**
  Reachability does **not** subsume this, and reading it as though it did was the
  design's own error. A ring of procedural units that defer only to each other is
  entered by no triggering unit and fails as unreachable — but a ring a triggering
  unit *does* enter is reached like any other chain, and a session following
  `defers=` out of its mandate is directed round it forever with no terminal body
  and no visited-id rule to save it. Reachability asks whether a mandate can get
  *to* a unit; termination asks whether a session that follows the deferrals ever
  arrives *anywhere*. Both are build errors, and termination is checked behind
  reachability, so a ring nothing enters is still reported as the orphan it also
  is — that is the repair its author has to make first, and the ring is dead prose
  until they make it.

Together with partition, that yields the structural claim the design is actually
for: **every byte of the methodology is either in a mandate or reachable from
one.** Reachability is per kind, not universal — a procedure reached only from a
`kinds=impl` condition is reachable from the `impl` mandate and from no other,
which is exactly right.

### Fence state, and what it guarantees

Markers inside fenced code blocks are **not** markers. The methodology documents
itself, so example markers inside fences are certain to appear.

The parser therefore tracks fence state across the whole file body, and a marker
is recognised only as a complete line, unindented, while that state is **neutral**.
An indented or mid-fence marker-shaped line is prose — which is the intended
reading for an example, and a *visible* misreading for a real marker, because the
preceding unit absorbs it and the pinned id set moves.

**The fence rule is [CommonMark 0.31.2 §4.5](https://spec.commonmark.org/0.31.2/#fenced-code-blocks)
exactly, and the exactness is load-bearing in both directions.** An opener is at
most three columns of indentation, then three or more backticks or tildes, then
an info string — which may contain no backtick on a backtick fence. A close is at
most three columns of indentation, then at least as many of the same character,
then nothing but spaces and tabs. Loosening either end has a cost, and they are
not the same cost. Accept an opener CommonMark rejects — an over-indented one, or
a paragraph opening with an inline code span — and the parser swallows later real
markers into the preceding unit: wrong, but *visible*, because the pinned id set
moves. Accept a **close** CommonMark rejects and the parser returns to neutral
inside a block a reader still sees as code, promoting an example marker to a unit
boundary: wrong and **silent**, because the id set gains a unit nobody notices is
imaginary. A rule stated as "three or more of the same character" and implemented
by trimming the line accepts both.

The rule is applied to the document's lines, without container-block context: a
fence nested in a list item or a block quote is read at its own indentation. The
residue is one-directional — such a fence keeps swallowing past the point the
container would have closed it — so it lands in the visible direction above, and
modelling containers would mean carrying a Markdown block parser to decide a
question the corpus does not pose.

The **asymmetry with markers is deliberate and is the safe direction**. A fence
line may be indented up to three columns because CommonMark says so; a marker may
not be indented at all, though CommonMark would still read an indented HTML
comment as one. The strictness only ever *withholds* unit-hood, so a marker
written one column in is absorbed by the preceding unit and moves the pinned id
set; the converse rule would let an indented example inside a list become a
boundary.

**A file whose fence state is not neutral at end of file is a build error.**
Without that rule the gate has a hole precisely where it is least visible: a fence
opened after the first real marker and never closed absorbs every later
marker-shaped line into one giant final unit, violating no syntax rule, no
semantic rule, and no reference rule. The file parses, the build passes, and most
of a document silently stops being classified.

Closing that hole also earns a guarantee rather than only removing a failure. A
unit boundary is a recognised marker, and a marker is recognised only at neutral
fence state, so **no unit can begin or end inside a fence** and every unit's
fenced blocks are balanced within it. That is the fence half of "a unit must read
correctly standing alone", and it is now mechanical. The rest of that
property — that the prose reads sensibly out of its neighbours' company — stays an
authoring rule for the classification review pass, because no further mechanical
rule is specified for it and claiming one would be the design's one unverified
claim.

All of it is pinned by test on the exact shapes that decide it: a balanced fenced
example marker ignored, an unterminated fence rejected, a fence three columns in
still opening, a fence four columns in opening nothing, an over-indented run
failing to close, and a backtick info string carrying a backtick opening nothing.

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

Three classes of malformation fail there:

- **Syntax** — an unparseable marker, an unknown attribute, attributes out of the
  fixed order, a missing `class`, `kinds` on a procedural unit, a file whose fence
  state is not neutral at end of file, a leading `---` block that never closes, or
  a unit marker inside one that does. Two more are syntax about the *file* rather
  than about a line in it — a file that does not end in a newline, and an embedded
  path carrying a byte the listing's grammar cannot — and both are argued where
  they are decided, under the fetch and listing contracts below.
- **Semantics** — a duplicate unit id anywhere in the embed, a file declaring no
  unit at all, or body text before the first marker. A duplicate file-ordering
  key joins this class when the ordering directive arrives with the composer, and
  not before.
- **Reference** — a `kinds=` member that is not one of the nineteen, a `defers=`
  member that names no declared unit, a `defers=` member whose unit is not
  `class=procedural`, a procedural unit no mandate can reach, or a chain of
  `defers=` that returns to a unit it has already passed through.

**What the gate requires per file, and what it requires across the embed**, are
separable and are built separately. A single `(path, text)` decides every syntax
error, plus the two semantic errors about one file — no unit declared, and body
text before the first marker — and the `kinds=` membership check. Everything else
needs the assembled set: id uniqueness, `defers=` resolution and its class check,
procedural reachability, and chain termination behind it. **Neither half requires
anything of a file-ordering key** for as long as no composer exists — nothing per
file, and no uniqueness across the embed.

The reference class is the one `defers=` changes, and it changes it in the
direction that matters: **`content/` now references ids, so an unknown id inside
the embed is a build error.** An unknown id supplied as a *CLI argument* to
`grove-llm methodology` remains what it always was — an ordinary runtime user
error, answered with a message naming the id and directing the caller to the
listing. The two cases are distinct in who can fix them: a bad `defers=` target is
a contributor's mistake, visible to the build that produced it, while a bad
argument is a caller's, visible only when the call is made.

### A leading `---` block is opaque preamble

The parser skips a `---`-delimited block occupying an embedded markdown file's
first bytes, without interpreting it. The block is **optional**, and the build
neither requires it, forbids it, nor reads a field out of it; everything after it
is body, and body must begin with a unit marker.

Today exactly one embedded file has one. `content/SKILL.md`'s YAML frontmatter is
what every harness reads to discover the provisioned skill — its `name:` and
`description:` are the entry a session sees in its skill list — and it must keep
working unmodified for as long as anything is provisioned, which is the whole of
this design's delivery. Without this rule the per-file gate rejects
`content/SKILL.md` on the day it lands: that block is text before the first
marker.

Making the rule about *a leading delimited block* rather than about *that file*
is the point. A gate whose value is that it has no exceptions cannot afford a
`content/SKILL.md` case, and an exemption keyed to a filename carries a removal
obligation into a later increment, which is how exemptions outlive their reason.
This rule applies uniformly; most files simply have no such block, and the one
that does is not special-cased anywhere.

The block is unread rather than merely unrequired, and that is what decouples it
from provisioning's retirement: nothing in the embed depends on it, so it may be
deleted the day nothing provisions the file, or left in place, and no parser
changes either way.

**An opened block that never closes is a build error**, named on the line it
opened. That is the same hole the unterminated-fence rule closes and it earns the
same answer: a file whose first line is `---` with no matching close would
otherwise swallow the entire document as unread preamble, declaring no unit,
violating no marker rule, and leaving nothing for the gate to catch. `---` is not
a fence opener, so the two rules do not interact; this is simply the second place
a delimiter can run away with a file.

**A first line of `---` is *reserved* for this block**, because nothing in the
bytes distinguishes a frontmatter opener from an ordinary Markdown thematic
break, and a rule that is generic, opaque and indistinguishable cannot tell which
region the author meant. Reserving costs an author nothing, and the reason is
already in this design rather than in a convention: a body must begin with a unit
marker, so a **leading thematic break is a build error in every other spelling** —
`***`, `___`, `----`, `- - -` — under *body text before the first marker*. The one
spelling that would have been swallowed silently instead of rejected loudly is
`---`, and it is the one now spoken for. What the reservation buys is therefore
uniformity rather than a restriction: a leading thematic break is refused in
*every* spelling, instead of refused in four and silently honoured in the fifth.

**A unit marker inside the block is a build error**, named on the line it appears.
That is what keeps the reservation from costing anything the design claims. The
residue of reserving is an over-long unread region — an author who opens with
`---` loses whatever the block then swallows — and the load-bearing half of that
loss is *classified* bytes: a marker and its prose hidden behind an ordinary
Markdown spelling, with the unit set quietly one shorter. Making a swallowed
marker an error removes exactly that half. What remains is a swallowed region of
**prose**, which is unreachable without writing, on line one, a construct that is
already an error in every other spelling — and closing it too would mean
interpreting the block, which is the one thing this rule exists not to do.

### The file's mandate order is a comment directive, and it arrives with the composer

Ordering has to live somewhere, and putting it in `content/` rather than in a
file list in Rust keeps the driver from owning a fact about content's
presentation. Two questions follow — what carries it, and when it lands — and
they are answered separately.

**What carries it is an HTML-comment file directive**, the same device and the
same recogniser as a unit marker: an unindented whole line at neutral fence
state. `content/` therefore gains **no** metadata language for this. Frontmatter
carrying KDL was the earlier answer, argued from the repository already owning
one KDL parser; a comment directive wins on that argument's own terms, because it
adds nothing to own and reuses a reader the parser must have anyway. Duplicate
positions are a build error once the directive exists, because the composition
order must be total.

**When it lands is with the composer, and not before.** Composition is the
ordering key's only consumer — the parser does not need it, and `grove-llm
methodology` neither serves nor lists it. Marked and gated ahead of a composer it
would be parsed, checked for uniqueness, and read by nobody, which is scaffolding
a build gate should not be grown for; worse, the order's *values* would be chosen
by a session that has not written the thing that consumes them. So until the
composer exists, **the build requires no ordering key of any file and checks no
ordering-key uniqueness across the embed.**

Two alternatives were weighed and rejected, and one was tested and found false.

*Keep KDL frontmatter and exempt `content/SKILL.md` until provisioning retires.*
Rejected twice over. It puts a filename-keyed hole in the invariant that exists
precisely to have no holes, and it breaks the **ordering** rule as well as the
frontmatter one: an exempt file carries no position, so the composition order
stops being total — the single property the ordering key exists to supply, given
up by the file that most needs a position.

*Defer the whole question to the composer without settling the carrier.* Rejected
because deferring the *decision* does not resolve the collision, it re-poses it:
the composer needs an order while provisioning is still live, so a later
increment would meet the same conflict with less context. Deferring the
directive's **arrival** while settling its **shape** is what actually resolves
it, and is what this section does.

*One `---` block that parses as KDL and as the two-field YAML subset a harness
reads.* Tested against this repository's own `kdl` dependency and found false:
`content/SKILL.md`'s frontmatter fails to parse — KDL v1 admits no unquoted
*string* values, and both fields carry one. The only variant that does parse
quotes every value and yields a KDL document whose every node name ends in `:` —
YAML wearing a KDL parser, in a block two consumers must keep mutually valid by
hand.

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

Composition order is: `content/MANDATE.md`'s framing unit, then every triggering
unit whose scope admits this kind — ordered by file position, then by position
within the file — then the runtime facts. The framing unit needs no rule of its
own; it leads because its file is ordered first. The session-specific
instructions land last, where they are not buried under the generic bulk.
(*Preamble* in this spec means only the unread leading `---` block; the framing
unit is an ordinary marked unit and is never called one.)

### `grove-llm methodology` fetches bytes, or lists rows

The verb has two modes and they answer to different contracts.

**Given ids, it fetches**: the named units' source bytes, in the order given,
verbatim and framed by nothing. That output is the methodology itself, so any
decoration would be driver-authored prose arriving through a second door.

That contract and a *multi*-id fetch need one invariant to coexist, so the build
supplies it: **an embedded markdown file ends in a newline**. A unit runs to the
byte before the next marker or to end of file, so the last unit of a file without
one ends mid-line, and concatenating anything after it lands that unit's marker on
the tail of someone else's prose — the output stops being self-addressing in
exactly the mode the marker line was carried for. The alternative is a separator
the fetch inserts, which is framing, and framing is the thing the contract
refuses. Making it a file invariant keeps the fetch a concatenation and puts the
error where a contributor can act on it.

**Given no argument, it lists** — and a listing is *data*, so it needs a grammar
rather than a layout. The consumer is an agent, which is the reason it needs one
and not a reason to skip one: a row that has to be recovered from prose is a row
that is recovered wrongly. One unit per line, tab-separated, five fields in a
fixed order:

```
<id>	<class>	<scope>	<defers>	<file>
```

`<scope>` is `*` or the space-separated kind list, and `-` for a procedural unit,
which has none — the listing may not promise a field it cannot supply for every
row. `<defers>` is the space-separated target list, and `-` where the unit defers
to nothing. `<file>` is the unit's `content/`-relative path.

Tabs need no escaping rule here because no field can contain one, and neither can
a newline end a row early. Four of the five fields prove it from their own
grammar: ids are kebab-case, and class and scope are drawn from closed sets. The
fifth does not. A filename is mutable data rather than a grammar — a markdown file
whose name holds a tab is a legal file, passes every other rule, and writes a row
with six fields — so the premise is made structural instead of assumed: **the
build rejects an embedded path containing a control character**, which is the
class the row's two delimiters belong to.

Enforcing it is what lets the no-escaping design stand as a property of the data
rather than a claim about today's tree. The check lives with the parser, which is
the one implementation both the build gate and the runtime reader share, so it
cannot hold on only one of the two traversals.

This is the shape the rest of `grove-llm` already speaks — every existing verb
writes plain lines of whitespace-free tokens, and none takes an output-format
flag. A `--json` mode would be the first, on a surface deliberately pinned flat,
to serialize five closed-set tokens that are already unambiguous.

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

Exactly two source citations go stale here, and they are named for the same
reason the release scan is. `build.rs`'s header cites
[one build owns a session](../adr/one-build-owns-a-session.md) for "only `grove`
should carry it", which that record now contradicts outright — both binaries link
the embed. `src/provision.rs`'s stamp re-verification cites it for the restore,
which the record no longer carries at all. Both are comments on mechanism that is
decided to go, in code the retirement increment deletes outright, so they are
scheduled rather than repaired.

The other source citations survive intact and should not be touched:
`src/loop_driver.rs` and `src/llm_cli.rs` cite the pre-launch pairing report, and
`src/provision.rs`'s identity documentation cites the empty-directory exclusion
from the payload — all three claims the reworked record still makes.

### Build pairing survives, sharpened

Under provisioning the skew was between two copies of a *whole* methodology: a
skill directory some other build had written, and the CLI a session resolved.
Mandate delivery removes that vector entirely — there is no shared mutable
directory, and the methodology arrives in argv from the driver that composed it.

What replaces it is narrower and sharper. The triggering half of a rule reaches
the session from the **driver's** embed; the deferred procedural half comes from
whichever `grove-llm` the session's `PATH` resolves. A mismatched pair is a
split-brain *inside one rule*, not two copies of one document.

That failure is also **loud**, and `defers=` is what makes it so: the mandate
carries the deferred procedure's id in the slice's own marker, so a session
reaching for that procedure names an id its `grove-llm` either knows or errors on,
at the moment it matters, rather than diverging silently. Loudness is a
consequence of the deferral being *declared*; a design in which the session had to
guess which id held the deferred half would have no error to raise. The
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

**The bound is 64 KiB — 65 536 bytes — per kind**, and the assertion is that the
composer's output for each of the nineteen kinds is not greater than it.

*What is counted* is exactly what the composer returns: the selected units' source
bytes and the blank line joining each adjacent pair. The driver's runtime facts
are excluded. They are appended after composition, they vary per session with the
selected handle and the resolved version control, and they are a few hundred bytes
against a threshold in the tens of thousands — including them would make a
deterministic assertion depend on a session that does not exist yet, and would
measure the two things in the mandate that cannot be misclassified.

*Why 64 KiB*, given that any threshold here is a choice rather than a derivation.
The embedded markdown corpus is about 139 kB, and its largest single document,
`SKILL.md`, is about 50 KiB. The expected triggering share of one kind's mandate
is low tens of kilobytes. So 64 KiB sits above the largest whole document and just
under half of everything: a mandate can only reach it if roughly half the corpus
was classified triggering and admitted to one kind, or if one large document's
procedural bodies were misclassified wholesale. Neither is reachable by honest
classification, and both are the drift the alarm exists to see. It remains far
below the ceiling it is deliberately not derived from — about 6% of `ARG_MAX`, so
it fires with more than an order of magnitude of argv headroom still in hand,
which is what keeps it an alarm about classification rather than a safety limit
about argv.

*Where it lives* is the test suite, not the build and not the driver. A malformed
embed is a syntactic fact the build can establish; an oversized mandate is a
judgement about classification measured against an admittedly arbitrary number,
and failing a contributor's build on that would be the gate this design is
otherwise careful not to erect.

### Provisioning retires after the mandate is proven, and the sweep is judgement rather than a file list

Composition lands first and provisioning retires after it, so the mandate is
proven working before the fallback is removed. Sessions therefore receive both a
mandate and a provisioned skill for one increment. That state is a **transient
and never a resting state**: two delivery paths that can disagree is the shape
[the mandate delivers the methodology](../adr/mandate-delivers-the-methodology.md)
rejects outright, and the only reason it is admitted at all is that the reverse
order removes the fallback before anything has replaced it.

The retirement itself is mostly subtraction, and `grep -rn provision` recovers
the file set — which is why what is recorded here is the part it cannot: the
sites where deleting the match is the wrong edit, and the sites that match
nothing and still have to move.

**A record is reworked when the new decision supersedes its own, not when it
merely mentions a mechanism that is going.** A record describing mechanism that
has not yet changed stays accurate until it does, and editing it early makes it
describe a build nobody can run. That is what separates
[one build owns a session](../adr/one-build-owns-a-session.md) — already reworked,
because mandate delivery changed what pairing *means* — from the architecture,
usage and configuration documents, which stay accurate until the sweep and are
edited with it.

**Three checks in `tests/provision.rs` are claims about the embed or the release
rather than about provisioning**, so deleting the file drops them silently:
`both_binaries_carry_the_embedded_methodology`, the `CONTENT_MARKER` constant,
and `the_release_path_scans_for_the_same_marker`, which is what keeps that
constant in step with `scripts/release-common.sh` — whose own diagnostic cites
this filename and needs repointing with the move. They belong beside the other
embed claims in `tests/methodology.rs`, which is where the instructed-verb scan
and the flat-verb-surface pin already went. Two more in that file set are not
straight deletions either:
`exactly_one_launcher_is_embedded_and_provisioned` is about a launcher the
composer renames, and `tests/methodology.rs`'s
`the_skill_frontmatter_survives_marking_and_provisioning` calls `provision_into`,
so it stops compiling the moment that module goes — it is the check that decides
whether `content/SKILL.md`'s YAML is still load-bearing.
`tests/support/mod.rs`'s `HARNESS_NAMES` environment scrub is neither: it is
hygiene for removed variables and may well survive on its own terms.

**Two prose sites lose a claim rather than a sentence.**
[config-driven sessions](config-driven-sessions.md) states that *provisioning
precedes ownership so a refused second driver still receives the independently
delivered methodology* — a fact about what a refused driver is still left
holding, which the retirement changes rather than deletes. `CONTEXT-MAP.md`'s
*Shared target: the personal skill directory* stops being a relationship at all,
because the two contexts no longer share the namespace; it goes rather than gets
rewritten.

**Two matches are outside the sweep.** Everything under `plugins/` belongs to the
plugin installer's own skill directories, a different bounded context; and
`src/tree_migrate.rs`'s v1 leaf-name fixtures are historical strings that happen
to contain the word.

### Deferred `content/` decisions

Three prose calls this design surfaced and deliberately did not make, recorded
because nothing else tracks them.

**`skill-adrs-and-specs` fuses four rules into one unit** — raise ADRs sparingly,
write a spec at a genuine agreement point, the ADR set is current-state, and the
same rule governs `docs/specs/`. Markers are whole lines and two of those
sentence boundaries fall mid-line, so the paragraph cannot be split without
editing prose, and the classification pass edited none. The unit is triggering
because two of the four ship nowhere else at `kinds=*`. De-fusing it is a prose
edit, not a marking decision.

**Eight of the nine embedded files still open with an H1 document title inside
their first unit**, which ships into every mandate that unit's scope admits — a
document heading delivered to a session that is not reading a document.
`content/driving.md` lost its H1 when its file-reader narrative was stripped and
carries a `<!-- grove reference file — … -->` note instead; whether the other
eight follow is a decision, not an oversight.

**A claim written into a new leaf's `Goal` is the one assertion nothing
re-checks** before a whole session is spent on it. A bad citation inside a
review's *findings* is caught by the session that integrates them, because it
reads the code; the same citation in a leaf's `Goal` launches a session against a
phantom, and `duplicated-prose-k36` was spent exactly that way. `driving.md`'s
`driving-turning-a-sweep-into-evidence` covers the neighbouring class — control
the instrument before asserting a repo-wide count — but not that destination, and
`driving-externalizing-surfaced-work` says only that externalizing is cheap.
Whether one instance earns a sentence in either is a `content/` call.

## Requirements

### Requirement: Every embedded markdown file is fully classified

The build SHALL reject any embedded markdown file that declares no unit, that has
body text before its first unit marker, that carries a malformed marker, whose
fence state is unbalanced at end of file, whose leading `---` block is never
closed or carries a unit marker, that does not end in a newline, or whose path
carries a control character. Marker-shaped lines that are indented or inside a
balanced fence SHALL declare no unit. Fences SHALL be recognised by CommonMark's
rule. A leading `---`-delimited block SHALL be skipped uninterpreted, and SHALL be
neither required nor rejected.

#### Scenario: unmarked file
- **WHEN** an embedded markdown file carries no unit marker
- **THEN** the build fails, naming the file

#### Scenario: text before the first marker
- **WHEN** body text appears before the first marker
- **THEN** the build fails, naming the file and the offset

#### Scenario: leading delimited block
- **WHEN** a file opens with a `---`-delimited block and its body then begins with
  a unit marker
- **THEN** the build accepts it, the block belongs to no unit, and no field is
  read from it

#### Scenario: file with no leading block
- **WHEN** a file's first bytes are a unit marker, with no `---`-delimited block
- **THEN** the build accepts it

#### Scenario: unterminated leading block
- **WHEN** a file opens with `---` and no matching close appears
- **THEN** the build fails, naming the file and the line the block opened on

#### Scenario: marker inside a leading block
- **WHEN** a file opens with `---` and a unit marker appears before the block's
  close
- **THEN** the build fails, naming the marker's line, rather than skipping that
  unit as preamble

#### Scenario: file that does not end in a newline
- **WHEN** an embedded markdown file's last byte is not a newline
- **THEN** the build fails, naming the file

#### Scenario: path a listing row could not carry
- **WHEN** an embedded markdown file's path contains a tab, a newline, or any
  other control character
- **THEN** the build fails, naming the file

#### Scenario: duplicate id across files
- **WHEN** two units in different files declare the same id
- **THEN** the build fails, naming both files

#### Scenario: example marker inside a balanced fence
- **WHEN** a marker-shaped line appears inside a fenced block that opens and
  closes within the file
- **THEN** it declares no unit, and its bytes belong to the unit containing the
  fence

#### Scenario: unterminated fence
- **WHEN** a file's fence state is not neutral at end of file
- **THEN** the build fails, naming the file and the line the unclosed fence opened
  on

#### Scenario: fence indented past CommonMark's bound
- **WHEN** a fence-shaped line is indented four or more columns
- **THEN** it opens nothing and closes nothing, so a marker below it is an
  ordinary unit boundary and a fence it appears to close stays open

### Requirement: Every procedural unit is reachable from a mandate, by a chain that ends

The build SHALL reject any `defers=` target that is not a declared procedural
unit, any procedural unit not reachable by following `defers=` from a triggering
unit, and any chain of `defers=` that returns to a unit it has already passed
through.

#### Scenario: unknown target
- **WHEN** a `defers=` member names no declared unit
- **THEN** the build fails, naming the referring unit and the unknown id

#### Scenario: target of the wrong class
- **WHEN** a `defers=` member names a `class=triggering` unit
- **THEN** the build fails, naming both units

#### Scenario: unreachable procedure
- **WHEN** a procedural unit is named by no unit's `defers=`, or only by units
  themselves unreachable from any triggering unit
- **THEN** the build fails, naming the unreachable ids

#### Scenario: unrooted ring
- **WHEN** procedural units defer only to each other and no triggering unit
  names any of them
- **THEN** the build fails as unreachable, not as a cycle — nothing entering the
  ring is the fault its author repairs first

#### Scenario: rooted ring
- **WHEN** a triggering unit's `defers=` reaches a chain of procedural units that
  returns to one it has already passed through, whether after one hop or several
- **THEN** the build fails, located at the deferral that closes the ring and
  naming the ring's ids in the order a session would walk them

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

### Requirement: `grove-llm methodology` serves any unit and lists them all

`grove-llm methodology` SHALL serve any unit's source bytes by id and SHALL list
the available units, in a parseable grammar, when given no argument.

#### Scenario: fetch by id
- **WHEN** the verb is given one or more known unit ids
- **THEN** it writes those units' source bytes, in the order given

#### Scenario: several ids at once
- **WHEN** the verb is given several ids
- **THEN** every fetched unit's marker still begins a line of the output, so each
  slice remains self-addressing

#### Scenario: unknown id
- **WHEN** the verb is given an id no unit declares
- **THEN** it exits non-zero with a message naming the unknown id and directing
  the caller to the listing

#### Scenario: listing
- **WHEN** the verb is given no argument
- **THEN** it writes one tab-separated line per unit — id, class, scope, defers,
  source file — with `-` in the scope field of a procedural unit and in the defers
  field of a unit that defers to nothing

#### Scenario: listed ids round-trip
- **WHEN** an id is taken from a listing row's first field
- **THEN** the verb accepts it as a fetch argument unchanged

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
  and rejected alike, including markers inside fenced blocks, a file with a
  leading `---` block beside one without, and the boundary shapes on both sides of
  each bound: a fence at three columns and one at four, an over-indented run that
  does not close, a backtick info string that opens nothing, and a marker inside a
  leading block. Each of those is a shape a looser rule reads differently, so a
  suite that does not carry it goes green on the looser rule. The repository's
  existing instructed-verb scanner is the precedent: its reading rule is pinned
  on the shapes it must see *and* the shapes it must ignore, because two of those
  were live holes rather than hypotheticals. The leading-block pair is the shape
  that keeps `content/SKILL.md` parseable while it is still provisioned, so a
  test that only ever sees files without one would go green on a parser that
  rejects the real embed.
- **The malformed cases**, each asserted to fail: every syntax, semantic, and
  reference error listed above.
- **Termination checked against an independent method**, not only against
  hand-written shapes. The walk's verdict is agreed with a transitive closure
  computed by saturation rather than by descent, over every digraph on three
  procedural units, self-loops included. Every unit is rooted, so reachability is
  held satisfied and the cycle question is the only one live — this isolates
  termination rather than covering both rules. It earns the machinery because
  termination is the one rule in the set that shipped as a claim plausible
  fixtures did not cover, and a suite proves the shapes its author thought of,
  which is exactly what was in question.
- **The completeness invariant**, now three claims rather than two — every
  triggering unit appears in the composed mandate of every kind its scope admits,
  every procedural unit appears in none, and every unit is reachable: triggering
  units through a mandate, procedural units by following `defers=` from one. The
  third claim is what makes the invariant cover the whole embed rather than only
  its delivered half.
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
- **The per-kind size alarm** — each of the nineteen composed mandates at or under
  64 KiB, counting the composer's returned bytes and not the driver's runtime
  facts, for the reasons given above.
- **The listing's grammar**, asserted as a grammar rather than a golden string:
  five tab-separated fields per line, the `-` placeholder in both optional fields,
  and every id in the listing accepted as a fetch argument. The round trip is the
  point — an inventory an agent cannot feed back into the verb is prose.

Two existing checks **relocated rather than died** ahead of provisioning's
retirement: the scan asserting the embedded methodology instructs no `grove-llm`
verb the embedded CLI lacks, and the flat-verb-surface pin that makes that
comparison mean what it claims. Both are claims about the embed, not about
provisioning, and the architecture already names the first as the enforceable
half of the build boundary. They live in `tests/methodology.rs` and their corpus
improved in the move: they scan the embed itself, through the module seam, rather
than a provisioned extraction of it — which is what dissolved the recursive
filesystem walk that used to gather it.

## Out of scope

- **Behavioural evaluation of the composed mandate.** Rejected during grilling:
  expensive, non-deterministic, it measures a model rather than Grove's artifact,
  and it localizes nothing when red. The honest behavioural check is the next
  real Grove run after the change lands, with a human watching.
- **A `--kind` filter on `grove-llm methodology`.** Filtering the inventory by
  session kind is a query concern rather than marker semantics, the golden
  snapshots exercise composition through the module seam without a process, and
  every flag is surface to keep in step. Reopen if a human needs to audit a
  composed mandate without reading a snapshot file.
- **JSON output on `grove-llm methodology`.** Rejected on the merits rather than
  on "the consumer is an agent", which is the argument *for* a stable format and
  is answered by giving the listing a specified tab-separated grammar. What is
  rejected is a *second* serialization: the listing's five fields are closed-set
  tokens that cannot contain a tab, so JSON would add a flag, a code path and a
  schema to keep in step in exchange for nothing the rows do not already say —
  on a verb surface a test deliberately pins flat. Reopen if a listing field ever
  has to carry free text, such as a title or a summary, since that is the point
  where escaping stops being a property of the data and becomes a rule someone
  must remember.
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

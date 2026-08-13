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
anyone to review. A complement (`kinds=!finish`) has the same defect: it re-reads
itself the day the kind set changes, in the direction nobody inspects.

**A complement case exists, and it is long rather than short.** The design first
argued that none would arise — guidance for every kind including `finish` is `*`,
and guidance that is not is a short list — and the session-ending instruction
falsifies that: the relaunch ending is for exactly the eighteen non-`finish`
kinds, and the only spelling the grammar admits is all eighteen labels.

The ugly marker is still the right one, and the reason is **adjacency and single
ownership of scope**. A marker answers *which kinds does this unit reach* beside
the prose it scopes, and it is the only place that question is answered. A
complement moves the answer: `kinds=!finish` states what the unit is not for, so
the set it does reach is derived from a kind set declared elsewhere and re-reads
itself whenever that set changes. Recovering a per-kind guarantee then means
writing each kind's ending down a second time, in a test, away from the prose —
which is the *classify in a manifest beside `content/`* shape
[the mandate delivers the methodology](../adr/mandate-delivers-the-methodology.md)
already rejects, for the same reason: two statements of one classification, with
nothing holding them in step.

The two hazards remain asymmetric, and the asymmetry is what each costs to guard
rather than whether it can be guarded at all. A complement's hazard is that a
twentieth kind is silently **absorbed** into guidance nobody chose to give it. The
list's is the mirror — a twentieth kind is silently **omitted** from guidance it
needs, and its sessions are then launched with no ending at all, never signal, and
stop the loop one by one. Grammar closes neither. The **omission** is closed by a
check that needs nothing beyond the closed kind set and the invariant already
stated — for every kind, exactly one ending unit — so it costs no second source of
truth (*Requirement: Every kind's mandate states exactly one session ending*). The
**absorption** is invisible to exactly those checks: a mandate that gained an
ending nobody chose still holds exactly one, and a scope that widened silently
satisfies every claim derived from `Kind::ALL` alone. It is not, however,
untestable — an exhaustive expected-ending mapping over the kind enum, with no
wildcard arm, stops compiling until a kind added later is given an ending
explicitly. What that costs is precisely the duplicated scope the explicit list
makes unnecessary. So the long list stays: it keeps the classification adjacent to
its prose *and* checkable against nothing but the kind set.

This does not reach the ADR's reopen condition, which is the kind set growing
large enough that explicit scopes stop being auditable, and then only for a
replacement preserving fail-on-kind-addition. One eighteen-member list against
nineteen kinds is still a list a human reads; negation preserves nothing.

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
needs no rule to place it, because the file ordering already does. The unit
survives the rename; only its file moves.

**It carries framing and no instructions**, and that reduction is a subtraction
with nothing to replace. Every instruction the launcher states today is already a
`kinds=*` unit the composer delivers into the same mandate: bootstrapping from
the handle and assembling the brief chain (`skill-bootstrap`), externalizing
surfaced work with the grow verbs rather than absorbing it (`skill-decompose`),
retiring the finished leaf (`skill-retire`), naming the work item by its stable
handle at the commit (`skill-commit`), the relaunch signal (`skill-signal`), and
the `finish` exception with its human gate (`skill-finish`). Exactly one clause
states something composition does not — "use the grove skill" — and it is the one
that goes false when provisioning retires.

A duplicate *inside* `content/` is **not** build-boundary drift: both copies ship
from one embed and cannot disagree across builds. What it costs is a reader's
decision. Two statements of one rule, in one prompt, with nothing holding them in
step, pose the question *do these agree?* — and a session answering it is
performing the derivation the mandate exists to remove. It is also the one shape
the completeness invariant is blind to, because a launcher sentence restating a
unit is not a missing unit; the invariant checks that every unit reaches its
kinds, not that no unit says what another already said.

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

### The ending is specialised where the conditional lives

Reducing the launcher to framing removes one copy of the session-ending branch.
It does not remove the branch. `skill-signal` is `kinds=*` and states both
endings itself — plain `complete` relaunches, the finish cycle ends with
`--done` — so a mandate composed from today's markers still hands every session
an `if` on its own kind, one slice later than the launcher did. Deleting the
launcher's copy alone would relocate the derivation rather than retire it.

The specialisation therefore lands on the units carrying the conditional.
`skill-signal` and `skill-finish` are each **split**, because each spans a
universal statement and a kind-specific ending and a unit has one scope. Three
scopes result:

- **The relaunch ending is scoped to the eighteen non-`finish` labels.** Such a
  session is told to run `grove-llm complete` as its last action and is told
  nothing about `--done`. There is no exception in its mandate, because the
  exception is not about it.
- **The finish endings are scoped to `finish`.** They are stated as outcomes of
  what the session did, never as a rule with another kind's rule beside it.
- **One sentence stays `kinds=*`**: a session never discovers that a grove is
  finished — the driver does, and tells it by launching a `finish` session. This
  is a **negative trigger**, and it is the case "keep the `if`" exists for. The
  eighteen are its load-bearing readers: withhold it and a session that retires
  the last live leaf holds no statement about what it is looking at, with the
  unasked question attached to a destructive action, and what the sentence tells
  it is *this is not yours*. `finish` reads the same sentence as a true statement
  of how it came to be launched — it did not discover the grove was finished
  either — so nothing has to be withheld from it, and the scope is `*` rather
  than a second spelling of the eighteen. Narrowing it would buy one sentence of
  tokens and pay for it with another explicit list to keep in step with the kind
  set.

Two claims those scopes carry are **authoring rules rather than mechanical
ones** — that the finish endings read as outcomes, and that no unit restates an
ending it was not scoped to state. The composer returns opaque bytes and carries
no role metadata, so both are carried by the classification review and pinned for
drift by the golden snapshots. The requirement below says which of its limbs the
guard establishes, rather than implying a check the seam cannot support.

**The finish endings are triggering, not procedural**, and the reopening ending
is why. A `finish` session that externalises surfaced work never reaches the
teardown steps, so an ending deferred into the procedural cycle body is an ending
it never fetches — it would end holding a mandate that named only the endings it
did not take. That is the deferral asymmetry read at its own boundary: a
procedural body is safe to withhold *because the session knows to ask*, and a
session that has decided not to tear down has no reason to ask for the teardown
procedure.

**Scope is bounded to the session-ending instruction, and the cut is whatever
that instruction forces.** How many units each split yields, and where the
boundaries fall, is prose the writing leaves decide against the three scopes
above; neither split is assumed to be a single cut. `skill-finish` in particular
holds more than one universal fragment — the negative trigger, and the clause
telling *every* session that its own escalations are discretionary rather than
routine — so a two-way cut may not reach it. **Prose that rides along because it
sits on the `finish` side of a forced boundary is not a scope audit**; the
cycle's teardown steps, sentinel mechanics and human gate are read by one kind
anyway, and narrowing them is a consequence of the cut rather than a finding
about them.

What is *not* in scope is re-deciding the scope of a unit that carries no ending.
Several `kinds=*` units are plausibly narrower than their scope, and a systematic
audit of them is its own concern with its own risk profile — one that trades the
completeness invariant's protection for token savings across the board, which is
a materially different bet from this increment's. It is externalised as its own
leaf rather than absorbed here.

### A `finish` session that reopens the grove signals a relaunch

The methodology names a `finish` session two endings, `--done` and no signal, and
separately tells it — as it tells every session — to externalise surfaced work
rather than absorb it. A session that does so cannot tear down: ordinary work is
live. It needs an ending it was never given, and both endings it was given are
wrong for what it did.

The third ending is one the mechanics already implement. `pick` selects the first
live non-`finish` leaf wherever it sits and falls back to the sentinel only when
nothing ordinary is live, so a leaf a `finish` session adds at the root is
selected next even though it lands *after* the sentinel; and the completion verb
is not gated by kind, so a `finish` session may signal a plain relaunch today.
Nothing in Grove has to change for this ending to work. What is missing is prose
saying it exists — a session-visible gap rather than a defect, and one the
session meets holding a mandate whose endings do not cover what it did.

A `finish` mandate therefore names three endings, as **outcomes of what the
session did** rather than as rules:

| what the session did | ending |
|---|---|
| teardown completed | `grove-llm complete --done` — the loop stops |
| externalised work instead | `grove-llm complete` — the loop relaunches and picks the new leaf; the sentinel waits |
| declined, or no human present | no signal — the loop stops, the leaf stays live and resumable |

**This is not a relapse into the branch the specialisation removes**, and what
separates them is what the driver can resolve. A branch on the **session kind**
is resolved before the session exists, so shipping it makes the session re-derive
a fact already in hand — that is the branch specialisation deletes. A branch on
**what happened during the session** is resolvable by nobody but the session, so
withholding it saves no derivation and yields an unasked question instead. The
`finish` mandate carries exactly one branch, over the one variable no composer
can bind.

**No confirmation is carried across the reopening.** The sentinel is never
retired, so once the reopened work is terminal the driver launches a fresh
`finish` session, which proposes the cycle and waits for a confirmation of its
own. A reopening banks no earlier answer — a session that externalised work never
reached teardown, so there was none — and it could not spend one later anyway,
because the finish transaction's proof is bound to its own launch.

**This is methodology, not an ADR**, and it fails the when-to-write test on
reversibility alone. Reversing it costs a prose edit and nothing else: no state
is stored under either answer, no tree is restructured, and the next session
composed the other way behaves the other way. Miss any one of the three limbs and
there is no ADR to write, so that limb settles it.

The other two are cleared, and the trade-off limb is cleared honestly rather than
argued away. The alternative — ending a reopening `finish` with no signal, so the
loop stops and a human reruns `grove` — is a real control choice. The driver makes
relaunch **opt-in** and treats no signal as a resumable stop, so an operator may
reasonably want the loop to pause exactly when a `finish` session has just widened
the grove's scope, and to read the new leaf before further sessions run against
it. What decides against it is that the sentinel-passing rule settled the same
question one level down — ordinary work preempts the sentinel without human
intervention — so stopping here would make a `finish` session's `leaf-add`
uniquely require a human where no other session's does. That is a preference
between two workable behaviours, which is what a trade-off is; it is simply the
losing one. It is also **surprising**, which is the third limb and the reason it
has to be written down at all — and a decision that is surprising and genuinely
traded off, but free to reverse, is prose. `CONTEXT.md`'s *Complete finish cycle*
entry already carries the three-ending reading; this section is where the design
says so, and `content/` is where the session is told.

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

### Requirement: Every kind's mandate states exactly one session ending

A composed mandate SHALL carry exactly one unit from the **declared ending
set** — the set naming, per kind, the unit that states that kind's session
ending. The mandate of every kind other than `finish` SHALL carry the relaunch
ending unit and SHALL NOT carry the `--done` token. The `finish` mandate SHALL
carry the finish ending unit, which states three endings as outcomes of what the
session did, and SHALL carry no other kind's ending unit. Every mandate, `finish`
included, SHALL carry the unit stating that a session never discovers a grove is
finished. Within any mandate, the completion verb SHALL be named only by that
mandate's ending unit.

**Two limbs of this requirement are prose rather than structure**, and the
distinction is recorded rather than blurred: that the `finish` unit states its
endings *as outcomes of what the session did* rather than as a rule qualified by
another kind's, and that no unit restates an ending in words naming neither the
completion verb nor `--done`. The composer returns opaque bytes and carries no
role metadata, so a mechanical claim about either would be a substring heuristic
wearing a SHALL. They are carried by the classification review, and pinned for
drift — not for correctness — by the golden snapshots, which hold the ending
unit's bytes verbatim.

#### Scenario: every kind is covered
- **WHEN** a mandate is composed for each member of the closed kind set
- **THEN** each one carries exactly one unit from the declared ending set

#### Scenario: a kind added later
- **WHEN** a session kind is added to the closed set and no ending unit's scope
  is widened to admit it
- **THEN** the assertion fails, naming that kind — rather than the loop stopping
  silently on the first session ever launched for it

#### Scenario: an ending unit the declared set does not name
- **WHEN** a composed mandate carries a unit naming the completion verb, and the
  declared ending set does not name that unit
- **THEN** the assertion fails, naming the unit — which the membership count
  alone cannot see, because the declared unit is still present and still counts
  one

#### Scenario: no exception for another kind's rule
- **WHEN** the mandate of any kind other than `finish` is composed
- **THEN** it contains no `--done` token, and names the completion verb only in
  its own relaunch ending unit

#### Scenario: the finish mandate is self-contained
- **WHEN** the `finish` mandate is composed
- **THEN** it carries the finish ending unit and the `--done` token, and carries
  no other kind's ending unit

#### Scenario: the negative trigger is universal
- **WHEN** any kind's mandate is composed, `finish` included
- **THEN** it carries the unit stating that a session never discovers a grove is
  finished; the driver does, and tells it by launching a `finish` session

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
- **The session-ending guard, generated from the closed kind set.** Four claims,
  every one of them about unit membership or a token, because membership and
  bytes are what the seam returns: for every kind, exactly one unit from the
  **declared ending set** appears in that kind's composed mandate; no mandate but
  `finish`'s carries the `--done` token, and `finish`'s does; within every
  composed mandate the completion verb is named only by units the declared set
  names; and the negative trigger appears in all of them.

  This is a **new check, not a strengthening of the golden snapshots** — and the
  distinction is the point of writing it down. A golden asserts that a mandate has
  not moved, which is the wrong shape twice over: it says nothing about what a
  mandate must contain, and a kind added to the enum produces a *new* golden
  rather than failing an existing one, so the exact hazard the guard exists for
  passes straight through the snapshots.

  **The two ways of breaking the guard do not fail alike, and only one of them
  leaves a kind at zero.** A kind added without a scope widened to admit it does:
  the count for that kind is zero and the first claim fails, naming it. An ending
  unit introduced without being named in the declared set does **not** — the
  declared unit is still there and still counts one, while the newcomer is
  invisible to a membership check. That is the duplicate-prose blind spot this
  design already admits when it reduces the launcher: the invariant checks that
  every unit reaches its kinds, not that no unit says what another already said.
  The third claim is what converts it, and it converts it only so far — a second
  ending statement operable enough to matter names the completion verb, so it
  fails the sweep; a restatement that names neither the verb nor `--done` escapes
  to the classification review, which is where the requirement above already puts
  it.

  **Every classifier carries both controls**, on the precedent's own rule that a
  sweep which cannot fail is worth nothing: the membership count shown failing on
  a kind whose ending unit's scope is withdrawn and passing on the real set, and
  the complement sweep shown failing on a synthetic mandate naming the verb
  outside the declared set and passing on one that does not.
  `tests/session_kind_guidance.rs` is the precedent and the place — it already
  generates its claims from the kind enum so that a twentieth kind fails until the
  guidance names it, states the limits of each sweep rather than papering over
  them, and this is the same claim about a different surface.
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
- **A systematic audit of every unit's scope.** The ending specialisation
  narrows the units that carry a session ending and no others, even though the
  `kinds=*` default is certainly wider than some units need. An audit is a
  different bet: each narrowing trades a slice of the completeness invariant's
  protection — a unit scoped to a list is a unit some kind can be wrongly
  omitted from — for token savings, and it makes that trade across the whole
  embed at once, where the ending specialisation makes it on one instruction with
  a test that covers exactly the hazard it introduces. Reopen it as its own
  increment, where the general question of what guards a narrowed scope can be
  answered before anything is narrowed.
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

# A book carries no asset

A walkthrough book's figures are **bytes in its own Markdown pages** — tables,
box-drawing diagrams, and other non-fragment fenced blocks. There is no figure
format, no asset group in `walkthrough.toml`, no image or diagram file in a book
directory, and no validator concept of an asset. The book directory keeps the
closed-world rule it already has: `walkthrough.toml`, the manifest's declared
pages, and nothing else at any depth.

What *is* written down is the **convention**, which costs no machinery: every
figure carries an adjacent statement of its role in the page's argument, stated
in `docs/specs/walkthrough-books.md`'s prose contract and applied by author and
reviewer like every other rule there.

## The trade-off

The publishing pipeline the editorial pilot measured has an art stage, and that
stage was kept. The obvious reading of "keep the art stage" is that art needs a
medium — a figure format, an asset convention, a validator that knows what an
asset is. The evidence points the other way, and specifically enough that the
omission is worth recording as a decision rather than left as a gap.

**Art paid for itself with nothing but Markdown.** The stage produced seven
claims over six pages of the `jj-workspace` book: six relations the pages had been
carrying in running prose, each redrawn as a table, and one existing table given
the statement of its role it had never had. Six of the seven were marginal — the
simulated two-stage arm reached only the seventh — which cleared the
preregistered threshold. No asset was created, no manifest group was added, and
the validator's ignorance of figures was unchanged. Whatever the art stage is
worth, it is worth that with no machinery behind it.

**The absence of assets is weak evidence, and the argument is built so that it
does not have to be strong.** The pilot's stage table chartered art as *whatever
figures, diagrams or tables plain Markdown can carry, and nothing else*, so a
stage that produced no asset produced none because it was forbidden to. The
narrower question — did the stage ever record a relation it declined to draw
**because Markdown could not carry it**? — is not answered by a positive control
either, and it is worth being exact about why.

Both of the art stage's survey classes are themselves defined in Markdown's
terms. `A1` fires only where *a Markdown table, list figure or diagram would
carry* the relation, and `A2` reaches figures that already exist. A relation no
arrangement of text could carry is outside both, and outside the stage task's own
production boundary. The stage record's `## Findings not fixed` is used four
times, and none of the four is that case: three are editorial declinations whose
reasons no medium would change — a catalogue left undrawn because compressing it
risked a defect in another stage's class, a route table that would have restated
code the page already walks in order, and a whole-book relation handed forward
because whole-book coherence is proof's class — and the fourth is an in-taxonomy
`A2` the book contract **forbade** the stage to fix. That fourth shows the
surface records a blocked case and distinguishes *declined on judgement* from
*forbidden*. It does not show that a ceiling-bound relation would have surfaced,
because a blocked fix inside the taxonomy is not the same event as a defect
outside it.

The only obligation that reaches the outside case at all is the
preregistration's own admission that its taxonomy is incomplete: a stage that
*meets* a reader-facing defect neither frozen standard names records it in
`## Findings not fixed` with the class it would need. That is a duty to record
what is met, not a duty to go looking. So the result the pilot actually supports
is the weaker one — **no ceiling-bound demand was observed by a
Markdown-bounded, single-book pass** — and the weight is carried instead by the
asymmetry below and by a reopening trigger written as a recordable event.

**The Markdown ceiling is higher than "tables".** Ten pages across the two
existing books already carry box-drawing tree diagrams inside `text` fences, and
they read identically as raw text and as rendered Markdown. The books have
figures; what they had no name for was the convention governing them.

**The cost is interface, and it is not incremental.** An asset would be the first
thing in a book directory that is not the manifest and not a page, and the
specification's final mode requires that directory to contain *exactly*
`walkthrough.toml` and the declared pages as regular files, with no other file,
directory, symlink or special entry at any depth.

**Inventory completeness is not what admitting assets would cost, and this record
does not claim it is.** `M101` already builds its expected set from the
manifest's own authored `[[page]]` list plus `walkthrough.toml`, then compares
that set against every entry found on disk in **both** directions
(`crates/book-validation/src/markdown.rs`). Pages are an author declaration
today. An `[[asset]]` group added to that expected set would keep the closed
world exactly as it is: an undeclared file would still be reported as outside the
inventory, a declared-but-missing one still as missing, and the recursive rule
would still admit nothing else. The obvious argument against assets — that they
turn a complete inventory into a check against a list — is simply false of this
implementation, and a decision resting on it would not survive the first reader
who opened the validator.

What assets would really cost is surface, paid by every book and every check for
a demand never observed. The manifest gains a group and a schema bump; the
snapshot gains an input class that is not UTF-8 text; `M201`'s permitted-target
set gains a member; and the diagnostic contract gains at least three codes — a
missing asset, an asset no page references, and a reference to an asset no
manifest declares. Each is a rule an author has to learn and a reviewer has to
apply. And it introduces a second kind of byte into a book: a page is text a
reader can read where the file lies, and an asset is not — which is the property
the *Rendering* exclusion exists to protect.

**The asymmetry runs toward waiting.** Building later is additive and needs no
migration: a book written under the closed-directory rule simply has no asset,
and `schema` is an integer in the manifest precisely so a format change is a
refusal rather than a misreading. Building now imposes the cost on every book and
every check for a demand no session has yet recorded. This asymmetry, not the
strength of the pilot's silence, is what decides the question.

**What this closes off, plainly.** A contributor who drops an image into a book
directory gets a hard final-mode failure and no escape hatch — not a warning, not
an ignored file. That is deliberate: the only way to add one is to reopen this
record, which is where the argument belongs.

## The alternatives that were rejected

### Asset machinery now, on the strength of the books still to come

The pilot measured one book, and the smallest of the five: `jj-workspace` is four
roots and 698 lines, a thin seam onto a subprocess. `grove-loop` is thirteen
roots and 10,533 lines, with a lifecycle state machine and outcome partitions —
structure of exactly the kind a drawn diagram is supposed to beat a table at.
Generalising *tables sufficed* from the first to the second is a real
extrapolation, and it is the strongest case for building the machinery in
advance.

It is rejected on two grounds. The demand it forecasts has not been observed —
once, weakly, under the limits stated above — and the whole reason this decision comes after the pilot rather than before it is
that machinery ordered ahead of the measurement meant to justify it cannot answer
whether it was needed. And the extrapolation is narrower than it looks: a
transition table and a box-drawing state diagram are both inside the ceiling, and
the ten pages that already carry tree diagrams are the proof. What would have to
bind is not "a diagram would be clearer here" but "no arrangement of text carries
this" — genuine two-dimensional layout, or a graph too dense to draw in
characters. That case may arrive; it has not, and the reopening condition below
is written so that its arrival is recorded rather than argued.

### A text-native diagram notation, such as a Mermaid fence

This is the cheap middle option and it deserves a real answer, because it costs
almost nothing mechanically: a fenced block is already a page's own bytes, so
there is no asset file, no directory-inventory change, no new permitted link
target, and no new diagnostic. Only the forbidden-fence rule and the convention
would have anything to say about it.

It is rejected for three reasons that compound. The evidence that says no asset
is needed says no notation is needed either — the six drawn relations were
drawn as tables by choice, and none of the three declined ones was declined for
want of a notation. A fence in a diagram language renders as a picture only under
a renderer, and rendering is out of scope: `docs/walkthroughs/` is read as
Markdown, and the walk-away property depends on that staying true — so a reader
of the raw file would get a diagram DSL where a box-drawing figure gives them the
diagram. And it is a second notation for authors to learn and reviewers to check,
covering a class of figure the `text` fence demonstrably already carries.

The second reason is the load-bearing one, and it is conditional on a fact that
could change. If `docs/walkthroughs/` ever gains a rendering surface, that
objection evaporates and this alternative becomes the first one to revisit —
ahead of assets, because it keeps every byte in the page.

### Leaving the convention unwritten too

"Build nothing" would have been the tidier outcome, and it is wrong. The classes
that made the art stage measurable — an undrawn relation, and a figure with no
statement of its role — live in the pilot's preregistration, which is a frozen
evaluation document and stops being the live instrument the moment the pipeline
is extracted as session kinds. Left there, the rule that produced the seventh
claim would bind nothing, and the art kind's discipline would have no durable
contract to point at.

So the convention goes where every other author-and-reviewer rule about a book's
prose already goes, in the specification's prose contract. It is the half of this
question that was answered *yes*, and it is free.

## What would reopen this

- **An art session records a declined figure whose stated reason is the Markdown
  ceiling** — not an editorial judgement, and not a rule in the book contract.
  That is the trigger, and it is deliberately a recordable event in the same
  surface that produced this decision's evidence rather than a judgement someone
  has to make in the abstract. One such record is a reason to revisit; a second
  in a different book is a reason to build.
- **A rendering surface for `docs/walkthroughs/`.** It removes the objection that
  decides against a text-native diagram notation, and that notation is then the
  first thing to reconsider — not assets.
- **A book whose subject is not a Rust crate.** Every argument here is calibrated
  on source code, whose structure text carries well. A book over a wire protocol,
  a UI, or measured data is a different medium question and this record does not
  cover it.
- **The fragment validator moving to the `writing-code-walkthroughs` skill**,
  where a book need not sit in this repository at all and the closed-directory
  rule may not be the right one to carry along.

The evidence behind this record is
`docs/evaluations/editorial-pipeline-pilot/README.md` and its art stage record,
and the specification it settles a hole in is
`docs/specs/walkthrough-books.md`.

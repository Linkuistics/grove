# Teaching a codebase through concept-first prose and exact source fragments — survey B

Second of a paired survey (`walkthrough-method-k4`). Written without reading
`walkthrough-method-a.md`, which was already on disk. Downstream consumers are
`book-system-k6` and `walkthrough-skill-delivery-k19`.

## How this corpus was chosen

The brief asked for a corpus biased away from the obvious introductory sources,
so the pair buys different evidence and different blind spots. Three deliberate
choices follow from that:

- **Program-comprehension psychology over documentation-style advice.** The
  question "what order should concepts go in" has a forty-year empirical
  literature under a different name, and it is more specific than any style
  guide.
- **Instructional-design sequencing theory over "write good docs" sources.**
  Elaboration Theory turns out to prescribe, in 1983, precisely the
  epitome-then-expand shape the root brief specifies, and to name the failure
  mode of the alternative.
- **Literate-programming primary sources and their critics, not Knuth's
  original essay.** The interesting evidence is in the retrospectives and the
  successor tools, where failure modes are stated by people who used them.

Sources are listed at the end with a note on whether the full text was read or
only an abstract. Statements that are my own reasoning rather than a source's
claim are marked **Inference.**

---

## 1. Sequencing concepts

### The elaborative sequence, and the failure mode of file order

Reigeluth and Stein's Elaboration Theory prescribes a "simple-to-complex"
macro-sequence built on an **epitome**: not a summary and not an abstract
overview, but a small, *application-level* treatment of the whole subject.
Their definition is that an elaborative sequence is one in which "the
[first] ideas epitomize rather than summarize the ideas that follow"
(Reigeluth & Stein 1983, p. 343). The content of an epitome is arrived at "by
(1) epitomizing the organizing content to a small number of the most
fundamental, representative, general, and/or simple ideas (i.e., the ideas that
best subsume the rest of the organizing content); and (2) including whatever of
the [supporting] types of content that are highly relevant (including learning
prerequisites)" (p. 342).

The controlling image is a zoom lens:

> A person starts with a wide-angle view, which allows him or her to see the
> major parts of the picture and the major relationships among those [parts]
> (e.g., the composition or balance of the picture), but without any detail.
> The person then zooms in on a part of the picture. ... After having studied
> those subparts and their interrelationships, [the] person could then zoom
> back out to the wide-angle view to review the ... parts of the whole picture
> and to review the context of this part within the whole picture.
> — Reigeluth & Stein 1983, p. 341

The value of this source for us is not the recommendation, which is
unsurprising, but its explicit naming of the two alternatives *and their
consequences*. Reigeluth and Stein describe conventional textbooks as beginning
"with the 'lens' zoomed in to the level of complexity deemed appropriate [for
the] intended student population; and they proceed — with the 'lens' locked
[at that] level of complexity — to pan across the entire subject matter. This
has [unfortunate] consequences for synthesis, retention, and motivation."
They then describe a second failure: "beginning with the lens zoomed all the way
in and proceeding in a fragmented manner to pan across a small part and zoom out
a bit on [it], pan across another small part and zoom out a bit, and so on,
until the [whole] scene has been covered and, to some limited degree, has been
[integrated] at the very end of the instruction. This has also had unfortunate
consequences [for] synthesis, retention, and motivation" (p. 342).

**Inference.** The second failure is an exact description of a
walkthrough organised in source-file order: each file explained in full detail,
integration deferred to a closing chapter. The first failure describes a book
that fixes one level of abstraction — say, "the public API" — and sweeps the
whole crate at it. Both are the natural shapes a code walkthrough falls into
without a deliberate sequencing rule, and this source is a citable argument
against both.

Elaboration Theory also supplies two devices for the integration problem the
zoom-out step implies. A **synthesizer** is "a strategy component for relating
and integrating ideas of a single type" using an explicit knowledge structure;
a **summarizer** is a component "to help prevent forgetting," of which there are
two kinds, an *internal* summarizer at the end of a lesson and a *within-set*
summarizer covering all lessons in a set (pp. 693–702). These are distinct
roles: the synthesizer builds relationships, the summarizer defends retention.

### What comprehension research says about the reader's own strategy

Three classical models disagree productively about the direction of
understanding. Brooks describes assimilation as top-down and hypothesis-driven,
"an opportunistic process driven by beacons in the code," with the reader
generating, refining, or repudiating hypotheses at several levels of
abstraction. Pennington's evidence supports a bottom-up account in which
readers assemble microstructures into macrostructures to form a *program model*,
and only then a *situation model* representing goals and functional means.
Letovsky's knowledge-based account treats programmers as opportunistic
processors "capable of exploiting either bottom-up or top-down cues," with a
knowledge base, a mental model, and an assimilation process (O'Brien 2003,
review). The commonly reported boundary condition is that **bottom-up strategies
dominate when the reader is unfamiliar with the code or application domain**.

**Inference, and it matters for this book.** Our reader is expert in Rust and in
operating-system APIs but has no knowledge of this crate's domain — ordinals,
keys, species, verdicts, plans, refusals. That is precisely the condition under
which the literature predicts bottom-up reading. A book that opens with a
purely top-down architecture chapter is betting against the reader's actual
strategy. The resolution consistent with both bodies of evidence is the
epitome: a *concrete, complete, application-level* trace of one real operation,
which supplies bottom-up material (real names, real values, real call order)
while establishing the top-down frame. This is stronger support for
`orientation-k11`'s design than "an overview is nice to have."

### Labelling the parts

A 2025 controlled experiment gives direct, recent evidence for naming the intent
of a code region. Fifty-six participants were split into an experimental group
receiving code annotated with **algorithm labels** — "the algorithm's name and
additional information" — and a control group receiving unannotated code.
"Annotating source code with algorithm labels significantly improves program
comprehension (p=0.040), with a median improvement of 6 points (~23%), but does
not affect completion times (p=0.991)" (arXiv:2504.19225, abstract). The authors
note the benefit was clearest "for developers with medium programming
experience," and that participants found labels most helpful "for recognizing
the code's intent."

This converges with the subgoal-labelling line in computing education, where
subgoal labels "emphasize the worked example structure, help learners organize
new information effectively, and improve self-explanation" (Margulieux &
Catrambone; Margulieux, Guzdial & Catrambone — abstracts only).

**Inference.** A fragment identifier is an opportunity for an intent label and
is usually wasted on a structural one. `«snapshot-read-dir»` names a location;
`«refuse-when-a-child-name-is-reserved»` names an intent. The experimental
evidence above is about labels that state *what the code is for*, and the
median 23% figure is worth quoting to a skill user who thinks fragment ids are
bookkeeping.

---

## 2. Explaining complete operations

The strongest evidence here is a 1986 IEEE Software paper that is, as far as I
can tell, the single most relevant primary source to this entire workstream.

Letovsky and Soloway studied maintainers reading a Fortran database program and
identified **delocalized plans**: plans "which are delocalized — that is, spread
far and wide in the text of the program." Their headline finding is that
"delocalized plans are more liable to misinterpretation than plans whose code is
closely grouped," and their subtitle states the mechanism: "A maintainer's
understanding can go awry when it is based on purely local clues."

The concrete failures are instructive because the code was *well documented*.
"An interesting feature of examples 1 and 2 is that the subjects had
documentation describing the problematic variables. This documentation was
straightforward and reasonable; some of the subjects remarked that the program
had better than average documentation. Yet the [documentation was insufficient]
in the cases we have described."

Their diagnosis is a distinction between the **role** of a variable — "the
identity of the datum for which that variable is home" — and its **goal**, the
reason the program maintains it. Documentation gave the role; the reader needed
the goal:

> NCHNGE: contains the number of changes to the database
>   Role: contains the number of changes to the database so far
>   Goal: used to test whether database file needs to be updated at the end of
>         a session
>
> Here we explicitly document the goal of NCHNGE so that wherever a reader
> encounters a fragment of the delocalized plan, he has access to the
> explanation for that plan.
> — Letovsky & Soloway 1986, p. 44

Their second strategy handles updates that do not serve the stated role:
"The strategy we prescribe for documenting more complex variables like IREC is
to find their non-normal updates and document them with in-line comments. Normal
updates need no comments, since we rely on the documentation of the variable's
role and goal and on the conclusions the reader will naturally draw from them.
For non-normal updates, we override the implications of the documentation with
explicit in-line comments."

A third example concerns a caching plan whose delocalization *concealed dead
code*: a redundant assignment that "virtually all readers of the program"
failed to notice. Their remedy was documentation that enumerated the plan's
sites — "By explicitly enumerating the places where this can happen, the
documentation counters the delocalization of the plan and makes it more likely
that the redundancy will be detected."

The paper's closing formulation is the design rule:

> Information needed to form correct interpretations of delocalized plans should
> be easily accessible when components of those plans are encountered.
> — Letovsky & Soloway 1986, p. 48

### Why this source is unusually well aimed at this crate

**Inference.** `ordinal-fs-tree` is built out of delocalized plans, in
Letovsky and Soloway's precise sense. Key allocation is `max key over the whole
tree + 1` — a plan whose correctness depends on sites in name parsing, snapshot
construction, and every insert. Ordinal shift is derived, not implemented:
each shifted name is recomposed from its unchanged key and parts, so the plan
lives across `name`, `plan`, and `ops`. Rollback spans `plan` and `fs/apply`.
Refusal totality spans `ops` and `report`. Each of these is exactly the
configuration the paper predicts will be misread from local clues.

Two consequences for the book. First, every fragment participating in such a
plan should carry the plan's **goal**, not merely its role — the crate's own
glossary already draws this line (`ordinal` is "a locator, not an identity"),
and the book should restate the goal at each site rather than once. Second, the
paper's enumeration remedy argues for something the ledger can carry: for each
cross-cutting invariant, an explicit list of the fragments that participate in
it.

### The paper's verdict on literate programming

Letovsky and Soloway discuss WEB directly, and their assessment is more
qualified than literate-programming advocacy usually admits. They credit it:
the system "allows the code to be taken apart into fragments, which can be
presented in whatever order is convenient for exposition, while retaining the
ability to reassemble a working program from the fragments," and it "speaks to
the need for richly indexed program documentation and to the possibility that
the presentation of the program for optimal human comprehensibility may not be
the same as the presentation demanded by a compiler."

Then the criticism:

> Neither the system nor its author takes a position on how programs should be
> presented to optimize comprehensibility, however. Moreover, the reliance on
> paper documents as a medium of presentation imposes costs such as the need to
> look things up in the index, to find and search the pages listed under an
> index entry for the desired piece of information, and so on.
> — Letovsky & Soloway 1986, pp. 47–48

**This is the central warning for `book-system-k6`.** A fragment mechanism gives
permission to reorder; it supplies no ordering, and the navigation it creates
has a lookup cost the reader pays. A book that reorders without a stated
sequencing rule has bought the cost and not the benefit.

---

## 3. Choosing worked examples for an expert audience

### Worked examples stop working, and then hurt

The worked-example effect is one of the better-replicated findings in
instructional psychology, and it has a boundary this brief must respect. Kalyuga,
Ayres, Chandler and Sweller's **expertise reversal effect** holds that
"instruction consisting of studying worked examples was effective for learners
with little — if any — prior knowledge, but lost its effectiveness and even
hampered learning for more advanced students who had some prior knowledge"
(Kalyuga et al. 2003 — abstract and secondary summaries). The effect is
characterised as a form of the redundancy effect: information beneficial to
novices becomes redundant, and then harmful, as knowledge grows. The prescribed
response is **guidance fading**: "tailor the fading of worked examples to
individual students' growing expertise levels."

**Inference, and it is a live risk for this book.** The reader is a strong Rust
programmer meeting an unfamiliar domain. Expertise is therefore *split*: high on
the language and OS substrate, near-zero on the crate's algebra. The expertise
reversal effect predicts that fully worked treatment of anything on the Rust
side — trait objects, `Result` plumbing, iterator chains, `io::Error` handling —
will be redundant and will cost comprehension, while fully worked treatment of
the algebra will not. This gives an operational rule sharper than "assume Rust
knowledge": worked detail should be spent on *domain* element interactivity and
withdrawn from *language* mechanics, and the fading should run across the book
as the reader's domain expertise grows — heavy in `orientation` and
`name-seam`, sparse by `filesystem-interpreter`.

### Examples that are not explained are a top-ten documentation defect

Uddin and Robillard's survey of 323 IBM professionals catalogues **unexplained
examples** as a distinct content problem — "A code example was insufficiently
explained" — reported in 10 examples by 8 developers, and one of six problems
that at least one respondent ranked a "Blocker" (Uddin & Robillard 2015,
Table 2). Developers "generally appreciate code [examples]," but an unexplained
one is a defect, not a neutral addition.

### Minimalism, as the countervailing pressure

Carroll's minimalist instruction supplies the opposite constraint, and it is the
less obvious half. Its four principles are: choose an action-oriented approach;
anchor the tool in the task domain; support error recognition and recovery; and
support reading to do, to study, and to locate (Carroll 1990; Carroll & van der
Meij 1998 — secondary summaries only). Carroll's argument is that training
materials should present short task-oriented chunks rather than "lengthy,
monolithic documentation that tries to explain everything in a long narrative."

**Inference.** Minimalism was formulated for task-oriented user documentation
and cannot be adopted wholesale for a conceptual walkthrough, whose purpose is
explicitly study rather than doing. The transferable parts are the third and
fourth principles. "Support error recognition and recovery" maps onto this
crate's refusals and error taxonomy — a chapter that explains what the library
*refuses* and why is doing minimalist work. "Support reading to do, study, and
locate" is a reminder that the same book will be entered by a reader locating a
single fact, which is the concern §5 and §7 take up.

---

## 4. Progressive disclosure to a proficient audience

### Disclosure as a notation property, not a formatting choice

The Cognitive Dimensions of Notations framework is the most useful analytic tool
I found for evaluating a fragment scheme, because it names the cost that
reordering imposes rather than only the benefit.

Relevant definitions, verbatim (Blackwell & Green, CDs chapter, v4.3, pp. 8–9):

- **Visibility**: "ability to view components easily. Systems that bury
  information in encapsulations reduce visibility."
- **Hidden dependencies**: "important links between entities are not visible. If
  one entity cites another entity, which in turn cites a third, changing the
  value of the third entity may have unexpected repercussions."
- **Role-expressiveness**: "the purpose of an entity is readily inferred.
  Role-expressive notations make it easy to discover why the author has built
  the structure in a particular way; in other notations each entity looks much
  the same and discovering their relationships is difficult."
- **Premature commitment**: "constraints on the order of doing things."
- **Viscosity**: "resistance to change... We distinguish repetition viscosity,
  many actions of the same type, from knock-on viscosity, where further actions
  are required to restore consistency."
- **Abstraction**: "types and availability of abstraction mechanisms... Systems
  that allow many abstractions are potentially difficult to learn."

The framework's trade-off discussion is the finding: "One way to reduce
viscosity is to introduce abstractions, but that will always require an
abstraction manager in which to define the abstractions and some early
commitment to choose which abstractions to define. The abstractions themselves
may then become viscous, introduce hidden dependencies, etc." Their trade-off
figure records that abstractions "can increase" hidden dependencies and stand in
a "complex relationship" to visibility.

**Inference, and this is the load-bearing one for `book-system-k6`.** A named
fragment is an abstraction in exactly the CDs sense. The framework therefore
predicts, without needing to see the design, that a fragment graph will
(a) require an abstraction manager — which is what the *ownership ledger* is,
and the brief is right to demand one; (b) introduce hidden dependencies, since
`«a»` inserting `«b»` inserting `«c»` is the definition given above; and
(c) impose premature commitment, since fragment boundaries must be chosen before
the prose that motivates them is written. Each has a mitigation:
- Hidden dependencies are mitigated by **visibility**: render, next to each
  fragment, where it is inserted and what it inserts. noweb does this with
  `noweave -x` cross-referencing (Ramsey 1994).
- Premature commitment is mitigated by making the ledger revisable and by
  `book-system-k6` explicitly reserving the right to refine slice ownership,
  which its brief already does.
- Role-expressiveness is bought by intent-named fragments (§1).

### The mechanism the Rust ecosystem actually uses for disclosure

mdBook's `{{#rustdoc_include}}` implements progressive disclosure at the
snippet level in a way worth borrowing conceptually. Given a file and an anchor
or line range, "the lines not in the line number range or between the anchors
will still be included, but they will be prefaced with `#`. This way, a reader
can expand the snippet to see the complete example, and Rustdoc will use the
complete example when you run `mdbook test`" (mdBook documentation). Anchors are
comments in the real source — `// ANCHOR: tag` / `// ANCHOR_END: tag`, matched
by the regex `ANCHOR:\s*[\w_-]+` — and "lines containing anchor patterns inside
the included anchor are ignored."

The design property is that **the elided material still exists in the
artifact**; disclosure is a display state, not an omission. Rust's own book moved
to this model: PR rust-lang/book#1949 extracted inline snippets into a
`listings/` tree of real Cargo projects, motivated by inline code that could not
be compiled or tested, documentation drifting out of sync with working code, and
error messages whose line numbers did not match what readers saw.

---

## 5. Local repetition versus internal links

This is the question the root brief flags as needing evidence, and the evidence
is unusually crisp: cognitive load theory does not merely permit a
case-by-case judgement, it supplies **two tests** that decide the case.

### The split-attention effect

Ayres and Sweller define the effect and its cause:

> [Split attention occurs with] several sources of physically or temporally
> disparate information, where each source of information is essential for
> understanding the material. Cognitive load is increased by the need to
> mentally integrate the multiple sources of information. This increase in
> extraneous cognitive load is likely to have a negative impact on learning
> compared to conditions where the information has been restructured to
> eliminate the need to split attention.
> — Ayres & Sweller, *Cambridge Handbook of Multimedia Learning*, ch. 8,
> chapter opening

They stress that separation need not be spatial: "Physical separation is not the
only form of separation generating unnecessary search. Multiple sources of
information that must be integrated before they can be understood can also be
separated in time, resulting in temporal separation."

### The two boundary conditions — the actual finding

The chapter's instructional-implications section states the limits explicitly,
and this passage is the one to cite in an ADR:

> 1. The principle only applies when multiple sources of information are
>    unintelligible in isolation. For example, physically integrating a diagram
>    with statements that merely redescribe the diagram has negative, not
>    positive effects on learning due to the redundancy effect. If all sources
>    of information are intelligible in isolation and redundant, elimination of
>    redundancy rather than physical integration should be pursued. Thus,
>    analysing the relation between multiple sources of information prior to
>    physical integration is critical.
> 2. The split-attention principle only applies to high element interactivity
>    material.
> — Ayres & Sweller, ch. 8, "Instructional implications", pp. 145–146

**Element interactivity** is defined as "the number of elements that must be
simultaneously processed in working memory in order to understand the
information. Materials low in element interactivity are easy to learn because
they keep working memory demands to a minimum."

The authors' own summary of the practical upshot: "A simple recommendation such
as 'eliminate split-attention between diagrams and text' is not sufficient. To
adequately understand the split-attention effect, instructional designers may
require considerably more training in cognitive theory and its instructional
implications than is currently the norm."

### The empirical calibration

Chandler and Sweller's spreadsheet study, reported in the same chapter, gives
the effect its boundary in data. Students learned spreadsheet skills either
with instructions integrated into the software, or with a conventional
manual-plus-computer package. "On test questions following the instructions, the
integrated (computer-only) group significantly outperformed the split-attention
group (computer plus manual). However, this difference was only found on test
questions tapping knowledge that was high in element interactivity, such as
creating a formula. On low element interactivity tasks such as selecting a row,
no [difference was found]."

**This is the decision procedure the root brief needs**, and it is checkable by a
reviewer:

| Test | If yes | If no |
|---|---|---|
| Is the passage's material high in element interactivity — must several facts be held together at once to understand it? | continue to test 2 | link; do not repeat |
| Is the referenced content unintelligible in isolation from this passage? | **repeat locally** | do not repeat; either link, or delete the reference as redundant |

Note the asymmetry the second test creates. When both sources *are* intelligible
in isolation and say the same thing, the prescription is not "link instead of
repeating" — it is "eliminate the redundancy." That is a stronger instruction
than the root brief's current phrasing and matches its clause about removing
repetition when each occurrence is independently intelligible and adds no
current explanatory value.

### Converging evidence from the other two sources

Letovsky and Soloway arrive at the same place from program comprehension:
information for a delocalized plan "should be easily accessible when components
of those plans are encountered" — repetition at the site, for material that is
by construction unintelligible from local clues.

Uddin and Robillard supply the cost of getting it wrong in the link direction.
**Fragmentation**: "The information related to an element or topic was fragmented
or scattered over too many pages or sections" (5 examples, 5 developers). A
respondent: "Fragmented documentation I find really difficult to use, where you
have to have 10s of clicks through links to find the information you need, and
page after page to read" (R2:69). Another described a two-page split between an
"API document" and a "usage document": "This information (functionality
description and usage configuration) should be part of the API, instead of
separated into two documents."

And the cost of getting it wrong in the repetition direction. **Bloat**: "The
description of an API element or topic was verbose or excessively extensive"
(12 examples, 11 developers — the most-reported presentation problem).
**Excessive structural information**: "The description of an element contained
redundant information about the element's syntax or structure, which could be
easily obtained through modern IDEs" — a respondent: "Again too much information
on how the class is related to other classes; I don't need docs for this."
**Tangled information**: "The description of an API element or topic was tangled
with information the respondent didn't need."

**Inference.** Bloat outnumbers fragmentation better than two to one in this
data (12 vs 5 examples; 11 vs 5 developers). A book with an explicit
local-repetition rule is walking toward the more frequently reported defect, and
the rule therefore needs the redundancy-elimination half enforced as firmly as
the repetition half. This is a real argument for making the editorial reviewer
score *both* directions, not just check that context is present.

### The tension the brief does not name

Mark Baker's "Every Page is Page One" argues that in web-delivered
documentation each topic must stand alone, because any page may be the first one
a reader sees, and readers forage — "if the information they find doesn't have
the right information scent, they move on" (Baker 2013 — secondary summaries
only).

**Inference.** This is in direct tension with an elaborative sequence, which
assumes the reader arrives at chapter 1 and proceeds. The root brief resolves
the tension in the sequence's favour — the book is "self-contained" as a *whole*,
with cross-references carrying navigation rather than required context — and
that is the right call for a book meant to be read. But the tension is real, it
will surface as editorial-review disagreement about how much context each page
restates, and naming it in the design saves that argument. The reconciling
observation is that Baker's condition and Sweller's first test agree: a page
whose material is intelligible in isolation should be left that way.

---

## 6. Literate technique: reordering while retaining byte-exact coverage

### What noweb actually settled

Ramsey's noweb is the reference design for a minimal fragment mechanism, and its
motivation is a documented failure of its predecessor. On WEB: "With experience,
many WEB users became dissatisfied. Some found WEB not worth the trouble... The
literate-programming forum was dropped, on the grounds that literate programming
had become the province of those who could build their own tools" (Ramsey 1994,
p. 1). WEB's specific defects were complexity and language-dependence: it
required significant work "to make WEB usable with a new programming language."

noweb's design, verbatim from the paper:

- "A noweb file is a sequence of chunks, which may appear in any order."
- Code chunks begin with `<<chunk name>>=` on a line by itself; "The double left
  angle bracket (`<<`) must be in the first column."
- "Several code chunks may have the same name; notangle concatenates their
  definitions to produce a single chunk."
- "notangle extracts a program by expanding one chunk (by default the chunk
  named `<<*>>`). The definition of that chunk contains references to other code
  chunks, which are themselves expanded, and so on."
- "notangle's output is readable; it preserves the indentation of expanded
  chunks with respect to the chunks in which they appear."
- Line provenance: "On a large project, it is essential that compilers and other
  tools be able to refer to locations in the noweb source, even though they work
  with notangle's output. Giving notangle the `-L` option makes it emit pragmas
  that inform compilers of the placement of lines in the noweb source."
- Cross-referencing: "If given the `-x` option, noweave uses LaTeX to show on
  what pages each chunk is defined and used."
- Mapping: "WEB files map one to one [to] both programs and documents. The
  mapping of noweb files to programs is many to many."

Reported use, from the author: markup and nt (400 doc lines / 1,200 total), an
ML code generator (900 / 2,600), a multi-architecture debugger (1,400 / 11,000),
and a colleague's experimental file system (4,400 / 27,000).

**Inference on the concatenation rule.** noweb's "several chunks may have the
same name" is convenient and is a hidden dependency in the CDs sense: whether
`«x»` is complete cannot be determined by looking at it. For a book that must
prove exhaustive, non-duplicated coverage against a frozen corpus, implicit
concatenation makes the duplicate-source and unreachable-fragment diagnostics
harder to state. A design that either forbids re-definition or requires an
explicit continuation marker buys a clearer diagnostic at a small notational
cost.

### The team-project retrospective

Ramsey and Marceau report using WEB on Penelope, a 33,000-line program for the
Synthesizer Generator, written by a team rather than an individual and never
intended for publication. They report that "the WEB source served as good
internal documentation throughout development and maintenance," but that "their
experience also uncovered a number of problems with WEB" (Ramsey & Marceau 1991
— **abstract and secondary summaries only; full text not obtained**). I flag
this as an evidence gap in §8: this is the one paper in the corpus that directly
addresses literate programming at team scale over time, and I could not read it.

### Reverse literate programming: the other direction

Knasmüller inverts the relationship. His framing of the problem is the sharpest
statement of it I found:

> Knuth's Literate Programming system allows an author to design and describe a
> program hierarchically according to the method of stepwise refinement. The
> result is source code, which can be read sequentially like a book, section
> after section. This helps when reading printed source code, but on screen
> source code is read rather selectively like an encyclopedia. There the
> programmer wants a system which allows, possibly even encourages, selective
> browsing; zoom in at interesting points; jump to other locations according to
> control flow or other semantic relationships.
> — Knasmüller, *Reverse Literate Programming*, abstract

The key architectural move: "**Starting with traditional source code**, these
features can be used to write Literate Programs." Documentation is attached to
folded regions of real source; "A special print command prints the source code
(the hypertext screen document) as a Literate Program, i.e. an essay, including
documentation, pictures, and program code."

His section-design rules are directly reusable as fragment-design rules:
"Each section should be short and simple to understand" and "There should only
be few and simple relations between a set of sections."

### Bidirectional: Entangled

Entangled keeps Markdown and generated source in sync in both directions —
*tangling* Markdown to code and *stitching* code changes back. Its fragment
syntax is CSS-attribute-shaped rather than noweb-shaped, though it uses noweb
references inside blocks:

````markdown
``` {.rust #hello file="src/world.rs"}
...
```
````

"`#hello` gives the block the `hello` identifier, `.rust` adds the `rust`
class"; the `file=` attribute names the tangle target; `<<hello>>` inside another
block inserts it. The documented workflow shows the round trip:

```
entangled tangle --force           # overwrites some changes you made
git restore src/brilliant_code.c   # retrieve from latest commit
entangled stitch                   # apply changes back to markdown
```

The reference publication is Hidding, "Entangled, a Bidirectional System for
Sustainable Literate Programming" (doi:10.1109/e-Science58273.2023.10254816 —
**not read**).

### Org-mode Babel

Org-mode Babel implements noweb references (`<<name>>`, enabled per-block with
`:noweb yes`) and tangling from a plain-text outline that "may contain code in
arbitrary programming languages, raw data, links to external resources, project
management data, working notes, and text for publication" (Schulte, Davison, Dye
& Dominik 2012, JSS 46(3) — **abstract only**). Its relevance here is the
walk-away property below, not its feature set.

### The walk-away check

For each system: with the tool uninstalled, what is still legible?

| System | Book still legible? | Compilable source still present? | Verdict |
|---|---|---|---|
| **WEB / CWEB** | The `.w` file is TeX-plus-Pascal with `@` control codes — readable with effort; the *woven* document requires TeX. | **No.** Only tangled output exists, and Knuth's TANGLE output is famously unreadable. | Worst case. Deleting the tool orphans the program. |
| **noweb** | The `.nw` file is plain text; documentation chunks are verbatim prose, code chunks carry `<<name>>=`. Fully legible. | **No.** The repo holds `.nw`, not `.c`. `notangle -L` output is generated. | Better: the *source of truth* stays human-readable, but the build dies with the tool. |
| **Knasmüller RLP** | Documentation lives in Oberon active-text elements — a structured binary editor format, not plain text. | **Yes** — source code is primary. | Inverted trade: code survives, prose does not. The prose is the tool-dependent half. |
| **Entangled** | Markdown is plain text and legible. | **Yes** — real source files exist in the repo and are the thing you compile and debug. | Best of the bidirectional designs; both halves survive, though sync is lost and the two can then drift silently. |
| **org-babel** | The `.org` file is plain text and legible without Emacs. | **No**, unless tangled output is committed. | Same shape as noweb. |
| **mdBook `{{#include}}` / anchors** | The `.md` retains prose but the code becomes `{{#include ...}}` placeholders — **the book's code content is lost**. | **Yes** — files are ordinary compiled source, and anchors are ordinary comments. | Inverted: code fully survives, book does not. |
| **Projection + tangle-and-diff** (see §9) | Book carries the full text of every fragment; it is complete Markdown with no placeholders. | **Yes** — the crate is untouched and is the source of truth. | Both halves survive independently. Only the *equality guarantee* is lost with the tool. |

**Inference, and this is the survey's main recommendation.** grove's walk-away
condition is not a stylistic preference here; it discriminates decisively among
these designs. Every tool in the table except the last sacrifices one of the two
artifacts. The last row is not a tool anyone in this corpus has built — it is
what falls out of a constraint this project has and the tool authors did not:
**the source corpus is frozen** for the duration of `ordinal-fs-tree-book-k10`.
When source cannot change under you, you do not need tangling (to generate code)
or stitching (to sync it back). You need only a checker that tangles the book's
fragments in memory and byte-compares the result against the real files. That
buys noweb's arbitrary reordering *and* mdBook's exactness-by-construction, and
degrades to two independently legible artifacts.

The residual risk is the one the frozen-corpus assumption carries: if a source
file does change, the equality check fails loudly rather than drifting silently
— which is the correct failure direction, and is what `book-assembly-k18`'s
"rerun exhaustive fragment validation" clause already anticipates.

### Notational details worth borrowing or rejecting

- **Borrow** noweb's root-chunk model — one root per output file, expanded
  recursively — because it makes "reproduces every in-scope file" a
  well-defined predicate rather than a review judgement.
- **Borrow** noweave `-x` cross-referencing as a *rendered* property: each
  fragment displays where it is inserted and what it inserts. This is the direct
  mitigation for the hidden dependencies the abstraction creates (§4).
- **Reject** noweb's relative-indentation rule for this project. noweb applies
  the reference's indentation to every line of the expansion. For generating new
  code that is a convenience; for byte-matching an existing file it adds a
  whitespace-transformation step between the book and the target, and a whole
  class of near-miss diagnostics. **Inference:** requiring each fragment to
  carry its own absolute indentation, with the reference contributing none,
  makes expansion pure concatenation and makes any mismatch a content bug rather
  than a possible indentation bug.
- **Reject** line-range includes (mdBook's `file.rs:2:10` form) as a fragment
  identity mechanism. **Inference:** line numbers are the most brittle possible
  anchor into a corpus, and the corpus is frozen only for this node's duration.
  Named fragments carrying their own text have no such coupling.
- **Note** that mdBook's anchors are *comments in the source*. That is a real
  cost this project should not pay: it would modify the fifteen frozen files to
  serve the book, which the brief's freeze forbids in spirit.

---

## 7. Editorial and navigational defects, and how to detect them

### The empirical taxonomy

Uddin and Robillard's ten problems, from two surveys of 323 IBM professionals
(E = examples mentioning it, D = developers reporting it):

**Content** — Incompleteness, "The description of an API element or topic wasn't
where it was expected to be" (E 20, D 20); Ambiguity, "mostly complete but
unclear" (16, 15); Unexplained examples (10, 8); Obsoleteness, "referred to a
previous version" (6, 6); Inconsistency, "The documentation of elements meant to
be combined didn't agree" (5, 4); Incorrectness (4, 4).

**Presentation** — Bloat (12, 11); Fragmentation (5, 5); Excess structural
information (4, 3); Tangled information (4, 3).

Severity, from the validation survey: "The three severest problems were
ambiguity, incompleteness, and incorrectness of content." Respondents "ranked
six problems as 'Blocker' at least once: incompleteness, ambiguity,
obsoleteness, incorrectness, inconsistency, and unexplained examples."
Ambiguity was ranked top priority 51.9% of the time. Notably, "'Tangled
information' and 'Excessive structural information' are absent [from the top
rankings] because no one selected the two problems as the top problems."
Incorrectness was infrequently observed but judged severe when present.

Aghajani et al. complement this with a taxonomy derived from artifacts rather
than opinion: they "mined, analyzed, and categorized 878 documentation-related
artifacts stemming from four different sources, namely mailing lists, Stack
Overflow discussions, issue repositories, and pull requests," explicitly to
avoid the bias of survey-based studies (Aghajani et al., ICSE 2019, pp.
1199–1210 — **abstract and secondary summary only; the full taxonomy figure was
not obtained**).

### Mapping defects to detection mechanism

**Inference throughout this table.** The mapping is mine; the defect names are
Uddin and Robillard's and the tool behaviours are from the tools' own docs.

| Defect | Detectable how | Deterministic? |
|---|---|---|
| Incompleteness (of source coverage) | Tangle-and-compare against the corpus; ledger row per file with owned line counts | **Yes** — this is exactly what `book-validation-k7` builds |
| Incompleteness (of exposition) | Reviewer against a named checklist of required topics | No — editorial review |
| Incorrectness | Fresh-context technical review against source, tests, models; the crate's own verification staying green | No — but bounded by evidence |
| Obsoleteness | Byte-equality check fails the moment source moves | **Yes** |
| Inconsistency | Glossary-term linting: terms defined in `CONTEXT.md` used with their `_Avoid_` synonyms | **Partly** — a Vale vocabulary can enforce the avoid-lists mechanically |
| Ambiguity | Editorial review; no mechanical proxy found | No |
| Unexplained examples | Structural check: every fragment has adjacent prose above a length floor | **Partly** |
| Bloat | Per-section prose-to-code ratio, flagged as an outlier not a threshold | **Partly** |
| Fragmentation | Count of inbound cross-references a page requires to be understood; fragments-per-source-file | **Partly** |
| Broken navigation | Link checking | **Yes** |
| Orphan pages | Reachability from the table of contents, same algorithm as unreachable-fragment detection | **Yes** |
| Heading-structure defects | markdownlint MD001 (heading increment) and siblings | **Yes** |

### Tooling that exists

- **mdbook-linkcheck** — an mdBook backend that validates links; its process is
  "Find all the links in a body of markdown text, Validate all the links we've
  found, taking into account cached results and configuration options, and Cache
  the results in the output directory."
- **lychee** / **lychee-action** — fast link checker for Markdown, HTML and text.
- **markdownlint** — structural rules including heading-increment.
- **Vale** — "an open source rule-based prose linter which supports a range of
  document types, including Markdown and MDX," which can carry a project
  vocabulary.
- GitLab's documentation pipeline is a worked example of combining these: the
  docs "are linted with markdownlint and Vale," with link checks in CI, and
  their guidance is that local runs must use the same configuration as CI.

**Inference on prose linting.** Vale is the only tool in this list that can
enforce the root brief's *style* contract — no rhetorical questions, no
metaphors, no persuasive framing — and it does so by pattern, which will produce
false positives on a technical text. It is worth adopting for the crate's
`_Avoid_` vocabulary (a closed, high-precision list drawn straight from
`CONTEXT.md`) and worth treating with suspicion for the rhetorical rules, which
belong to the editorial reviewer.

### A defect this corpus predicts but does not name

**Inference.** The fragment mechanism creates a navigational defect class that
none of the documentation-defect literature covers, because none of it studied
literate books: a fragment whose *insertion point* is far from its *definition*
reproduces Letovsky and Soloway's delocalization inside the exposition itself.
The book can therefore fail in precisely the way it exists to prevent. Two
cheap metrics detect it: for each fragment, the distance in pages between its
definition and the reference that inserts it, and the fan-out (number of
distinct fragments a single fragment inserts). Knasmüller's rule — "there should
only be few and simple relations between a set of sections" — is the design
constraint these metrics measure.

---

## 8. Where the search found silence

Recording these so a later reader does not repeat the search.

- **No controlled experiment measuring literate programming's effect on
  comprehension.** I searched specifically for one. The nearest result is Shum
  and Cook (SIGCSE 1994), which measured *documentation production* in an
  educational setting — literate programming "produces more comments in general
  and, more convincingly, it produced 'how documentation' and examples where,
  without literate programming, no examples were produced at all." That is
  evidence about authors, not readers. The forty-year literate-programming
  literature appears to rest on argument and experience reports, not measured
  reader outcomes. **Any ADR claiming the book form aids comprehension should
  cite the cognitive-load and program-comprehension evidence in §1 and §5, not
  literate-programming advocacy.**
- **Ramsey & Marceau (1991) full text not obtained** — paywalled at Wiley and
  DeepDyve. This is the corpus's most relevant experience report and the gap is
  material; the abstract confirms problems were found but does not enumerate
  them.
- **Aghajani et al.'s full taxonomy not obtained.** The summary I could reach
  gives the method and artifact count but not the category breakdown.
- **Entangled's tangled-file marker format is undocumented in its own
  documentation.** I checked the project homepage, the `entangled.py` README,
  and spot-checked the package source listing on 2026-08-29 without finding the
  comment-marker format that makes stitching possible. Its round-trip guarantee
  is asserted rather than specified. **This weakens my walk-away assessment of
  Entangled**, which assumes the generated files contain provenance markers; if
  they do, those markers are a modification to the source files, which would
  move Entangled toward mdBook's position in the table.
- **No primary source found on validating that a reordered exposition
  reconstructs an existing corpus.** The literature covers generating code from
  a document (noweb, WEB, Entangled, org-babel) and extracting snippets from
  code (mdBook, Sphinx `literalinclude`, Antora tags), but not checking a
  document against a frozen corpus. The projection design in §9 is therefore an
  inference from the two halves, not a borrowed pattern, and it should be
  treated as unproven.
- **No empirical evidence found for or against progressive disclosure in
  long-form technical books specifically.** The expertise-reversal work is about
  instructional sequences, not documents; the transfer is an inference.

---

## 9. Synthesis

### For `book-system-k6`

**Which artifact is the source of truth.** The crate. The book is a *projection*
of a frozen corpus, and the fragment validator's job is an equality proof, not a
build step: expand the root fragments and byte-compare against the real files.
This is the only arrangement in §6's table where both artifacts survive the tool
being deleted, and the freeze in `ordinal-fs-tree-book-k10`'s brief is what makes
it available. Do not tangle to disk; do not add anchors to the fifteen files.

**Fragment grammar.** Take noweb's shape — named fragments, references,
recursive expansion from one root per output file — with three deviations, each
justified above:
1. Absolute indentation carried by the fragment, reference contributes none.
   Expansion is then pure concatenation and every mismatch is a content bug.
2. Re-definition of a fragment id either forbidden, or requiring an explicit
   continuation marker, so completeness is locally decidable (§6).
3. A deferred reference is syntactically distinct from an unresolved one — the
   brief already requires this — and the ledger names the slice that fills it.
   **Inference:** this makes "unresolved" mean *defect* at every point in the
   book's life, which is what lets scoped validation run before assembly.

**Fragment naming.** Intent labels, not location labels. The 23% median
comprehension improvement from algorithm labels (§1) is the citable warrant, and
role-expressiveness (§4) is the design property being bought.

**Rendered cross-referencing is not optional.** Each fragment displays where it
is inserted and what it inserts, as `noweave -x` does. This is the mitigation
for the hidden dependencies the abstraction necessarily introduces (§4), and
without it the book reproduces the delocalization it exists to explain (§7).

**Chapter order.** `orientation-k11`'s complete-operation tour is an *epitome* in
Reigeluth and Stein's technical sense, and the design should hold it to that
standard: application-level, using real names and a real operation, subsuming
the layers that follow — not an abstract architecture overview. The layer
sequence that follows is the zoom. `book-assembly-k18`'s cross-cutting chapter
is the *synthesizer*: its job is relating layers, distinct from summarizing them.
The `syllabus-cli-k17` resolution of the opening operation is the zoom-back-out
step the analogy prescribes.

**Prose contract, made reviewable.** The repetition rule gets the two tests from
§5 verbatim as its criterion, with both directions enforced:
- Repeat locally only when the passage is high in element interactivity *and* the
  referenced content is unintelligible in isolation.
- When both passages are intelligible in isolation and say the same thing,
  *eliminate* the redundancy — do not convert it to a link and do not keep it.
- Every fragment participating in a cross-cutting plan (key allocation, ordinal
  shift, rollback, refusal totality) restates that plan's **goal**, not just its
  role (§2).

**Ledger contents,** beyond what the brief already specifies: per-fragment
fan-out and definition-to-insertion distance, so the delocalization defect in §7
is measurable; and, per cross-cutting invariant, the enumerated list of
participating fragments, which is Letovsky and Soloway's enumeration remedy.

**Validator diagnostics** — the brief's list, plus **drift**: tangled text not
equal to source. It should report the first differing byte offset and the owning
fragment, because a diff at book scale is otherwise unreadable.

### For `walkthrough-skill-delivery-k19`

**The elicitation questions map onto the variables this survey found to be
load-bearing**, which is a better justification for the question set than
completeness:
- *Audience expertise, split by axis.* Not "what level" but "expert in what, and
  novice in what." The expertise reversal effect (§3) makes this the variable
  that decides how much worked detail to spend and where to fade it. A skill that
  asks only "beginner/intermediate/advanced" cannot express the case this book
  is in.
- *Source scope and its stability.* Whether the corpus is frozen decides the
  whole tangling architecture (§6). This is the question a generic skill is most
  likely to omit and most needs.
- *Output form and its walk-away requirement.* "What must still be usable if this
  tooling disappears" is a question with three defensible answers, and it selects
  among noweb-style, mdBook-style, and projection-style designs.
- *Verification requirements.* Which of §7's defects must be caught
  mechanically versus by review.
- *Depth and style* as the brief already specifies.

**The skill's method body should carry, generically:** epitome-first ordering
with the two named failure modes to avoid (file-order panning; one-level
panning); intent-named fragments; the two-test repetition rule; goal-not-just-role
documentation for delocalized plans; rendered fragment cross-referencing; and
the defect checklist with its detection column.

**What the skill should not claim.** That literate presentation improves
comprehension. Per §8 there is no reader-outcome evidence for it, and the
defensible claim is narrower and still strong: *exact* fragment coverage makes a
walkthrough verifiable, and the sequencing and repetition rules are supported by
evidence that is about readers rather than about literate programming.

**A caution worth encoding.** Naur's Theory Building View sets the ceiling on
what any walkthrough can deliver. The programmer holding the theory "can explain
why each part of the program is what it is, in other words is able to support
the actual program text with a justification of some sort," and Naur argues this
knowledge "necessarily, and in an essential manner, transcends that which is
recorded in the documented products." A code-walkthrough book is an attempt to
write down ability (2) — the justification of each part — which is the most
transferable of the three abilities Naur lists and still, on his argument, not
fully transferable. **Inference:** the honest framing for the skill is that the
book maximises what can be written down, and a reviewer's completeness criterion
should be "every fact the reader needs for the claim in front of them," which is
what the root brief already says, rather than "everything the authors knew."

---

## Sources

Read in full (text obtained and quoted from the primary document):

1. Letovsky, S. & Soloway, E. "Delocalized Plans and Program Comprehension."
   *IEEE Software* 3(3), 1986, pp. 41–49.
   https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf
2. Ayres, P. & Sweller, J. "The Split-Attention Principle in Multimedia
   Learning," ch. 8 in Mayer (ed.), *The Cambridge Handbook of Multimedia
   Learning*.
   https://www.davidlewisphd.com/courses/EDD8121/readings/2006-AyersSweller.pdf
   (Scanned two-column copy; quotations are exact, page numbers are inferred
   from running footers and should be confirmed against a clean copy.)
3. Uddin, G. & Robillard, M. P. "How API Documentation Fails." *IEEE Software*
   32(4), 2015. DOI 10.1109/MS.2014.80.
   https://www.cs.mcgill.ca/~martin/papers/ieeesw2015.pdf
4. Ramsey, N. "Literate-Programming Can Be Simple and Extensible" (noweb).
   *IEEE Software* 11(5), 1994.
   https://mirror.gutenberg-asso.fr/tex.loria.fr/litte/ieee.pdf
5. Blackwell, A. & Green, T. R. G. "Notational Systems — the Cognitive
   Dimensions of Notations Framework," v4.3.
   https://www.cl.cam.ac.uk/~afb21/publications/BlackwellGreen-CDsChapter.pdf
6. Reigeluth, C. M. & Stein, F. S. "The Elaboration Theory of Instruction," in
   Reigeluth (ed.), *Instructional-Design Theories and Models*, 1983.
   https://ocw.metu.edu.tr/pluginfile.php/9337/mod_resource/content/1/Reigeluth%201983%20Article.pdf
   (Scanned two-column OCR; quotations reconstructed carefully and page numbers
   given, but wording should be re-verified against a clean copy before an ADR
   quotes it directly.)
7. Knasmüller, M. "Reverse Literate Programming." Johannes Kepler University
   Linz. http://www.literateprogramming.com/rlp.pdf
8. Naur, P. "Programming as Theory Building," 1985 (as reprinted).
   https://gwern.net/doc/cs/algorithm/1985-naur.pdf
9. mdBook documentation, "mdBook-specific features."
   https://rust-lang.github.io/mdBook/format/mdbook.html
10. Entangled documentation and `entangled.py` README.
    https://entangled.github.io/ ·
    https://github.com/entangled/entangled.py

Read as abstract, summary, or secondary source only — verify before quoting:

11. Kalyuga, S., Ayres, P., Chandler, P. & Sweller, J. "The Expertise Reversal
    Effect." *Educational Psychologist* 38(1), 2003.
    https://link.springer.com/article/10.1007/s11251-009-9102-0 (special issue
    introduction)
12. Ramsey, N. & Marceau, C. "Literate programming on a team project."
    *Software: Practice and Experience* 21(7), 1991, pp. 677–683.
13. Aghajani, E. et al. "Software Documentation Issues Unveiled." *ICSE 2019*,
    pp. 1199–1210. https://dl.acm.org/doi/10.1109/ICSE.2019.00122
14. Margulieux, L. & Catrambone, R., and Margulieux, Guzdial & Catrambone, on
    subgoal-labelled instruction. https://www.cs1subgoals.org/publications/
15. Carroll, J. M. *The Nurnberg Funnel*, 1990; Carroll (ed.), *Minimalism
    Beyond the Nurnberg Funnel*, MIT Press, 1998.
    https://mitpress.mit.edu/9780262512954/minimalism-beyond-the-nurnberg-funnel/
16. Baker, M. *Every Page Is Page One: Topic-Based Writing for Technical
    Communication and the Web*, XML Press, 2013.
    https://everypageispageone.com/the-book/
17. Schulte, E., Davison, D., Dye, T. & Dominik, C. "A Multi-Language Computing
    Environment for Literate Programming and Reproducible Research." *JSS*
    46(3), 2012. DOI 10.18637/jss.v046.i03
18. Shum, S. & Cook, C. "Using literate programming to teach good programming
    practices." *SIGCSE* 1994.
19. O'Brien, M. P. "Software Comprehension — A Review & Research Direction,"
    2003 (review of Brooks, Pennington, Letovsky models).
    https://www.st.cs.uni-saarland.de/edu/empirical-se/2006/PDFs/brien03.pdf
20. Hidding, J. "Entangled, a Bidirectional System for Sustainable Literate
    Programming." DOI 10.1109/e-Science58273.2023.10254816
21. "Providing Information About Implemented Algorithms Improves Program
    Comprehension: A Controlled Experiment." arXiv:2504.19225 (abstract read in
    full). https://arxiv.org/abs/2504.19225
22. rust-lang/book PR #1949, "Extract code into external files."
    https://github.com/rust-lang/book/pull/1949
23. Tooling: mdbook-linkcheck (https://docs.rs/mdbook-linkcheck), lychee-action
    (https://github.com/lycheeverse/lychee-action), Vale, markdownlint; GitLab
    documentation testing guide
    (https://docs.gitlab.com/development/documentation/testing/).

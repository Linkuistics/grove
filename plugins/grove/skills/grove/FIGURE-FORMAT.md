<!-- grove reference file — the figure contract -->
<!-- Provenance: the C4 levels in "Choosing the view" (system context, container,
     component and code, read from context to detail) adapt the C4 model by
     Simon Brown, https://c4model.com — CC BY 4.0; see LICENSES/c4-model.LICENSE. -->

# FIGURE-FORMAT — what a figure promises its reader

A **figure** is a diagram, chart or sketch that answers one reader question.
This file states what every figure must do. `art` reads it for every figure
that it draws or keeps. `proof` reads it when it checks figures. A design
session reads it for its views. A requirements or design session reads it for
a view that it draws in an exchange with the human.

The file separates three things. The **notation** defines what the symbols
and relationships mean. The **model** is the subject that the figure shows.
The **renderer** is the tool that arranges and draws the notation.

## Choosing the view

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**Choose the view by the reader's question and level (model, structure,
sequence, state, timing, grammar, or a concrete snippet), and keep notation,
model and renderer distinct. Before a renderer choice turns on a dated
capability fact, recheck the fact, and cite it with its date.** The current
facts and their check date are in `references/documentation-toolchain.md`.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**Run from context to detail through only the levels that add value: C4 where
the subject is a software system or ecosystem, other views below it. Link back
to an overview where that orients the reader, rather than repeating it on
every page.**

## Drawing the figure

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**A figure's title states its type and scope, it says what it omits, it has an
identifiable reading entry, labels sit on the elements they name, and each
one-way arrow carries a specific verb, on the arrow or, where many arrows
share one verb, through a line style the key names.**

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**Every convention the figure chooses (symbol, colour, line style or
abbreviation) is identifiable on or beside it. Teaching the notation itself
depends on the declared reader, and colour never carries meaning alone.**

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**A document set keeps one symbol and colour per concept, the prose's names
and one legend convention. It adds a new diagram kind only when the question
changes, and no decoration that carries no meaning.**

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**The figure's information is reachable in text (prose, an equivalent textual
form such as the grammar, and a text alternative) without narrating notation a
fluent reader reads.**

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**Place a figure where its question is asked: a semantic relationship with its
explanation, and a lookup view at its lookup destination with links from
elsewhere.**

## Source and rendering

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**A figure drawn from a formal source (a grammar, schema or model) is checked
against that source, and what the notation cannot show is stated beside it
with rule links.**

<!-- tc: depart long-sentence,semicolon because the conformance inventory pins this rule statement verbatim -->
**A figure in a tool-rendered notation commits its editable source and
tool-generated SVG together, with the tool version, the layout settings and
the regeneration command recorded; a repeatable recipe checks regeneration,
and drift after a tool upgrade is regenerated and reviewed. A figure in the
page's own bytes is its own source.** A document whose contract admits only
the page's own bytes never fires this clause.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**Source and SVG agreement, inspection in the declared environment, and reader
understanding are separate results, and none stands in for another.**

## Sketches in an exchange

<!-- tc: depart long-sentence,semicolon because the conformance inventory pins this rule statement verbatim -->
**In a live requirements or design exchange, sketch in the simplest dialect
that answers the question, which may be simpler than a document's figures
use. The content clauses above bind the sketch; the delivery clause binds once
it is committed.** The delivery clause is the rule on tool-rendered source and
SVG. A session that preserves an agreed exchange view commits its SVG export
under that clause. A live rendering stays available for a sketch that nobody
commits.

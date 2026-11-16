<!-- grove reference file — the document contract and its parts -->

# DOCUMENT-CONTRACT-FORMAT — what a document promises its reader

A **document contract** records the decisions that a document's text must
serve. It names the reader, the structure, the prose rules and the format.
An author writes the contract before the editorial stages start. The stages
read it while they work, and no stage invents a fact that the contract omits.

## Declaring a contract

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
<!-- tc: depart semicolon because the conformance inventory pins this rule statement verbatim -->
**A document declares its contract by path from its leaf body or node brief;
a part may be its own file shared across a document set.** A document set is
documents that share a format and prose rules, such as the pages of one book.
The structure brief belongs to one document. The prose contract and the
format and media part usually belong to the set. A reader checkpoint can
belong to either.

**The human makes the substantive structure decisions. The authoring session
records the human's own words and the locations they identify rather than a
paraphrase.** The session can propose a plan, ask questions and show the
consequences of each option. It does not choose the reader, the section plan,
the emphasis or the exclusions for the human.

## The parts

A contract has four parts. A part that the contract omits takes its default
(see [Absent parts](#absent-parts)).

### Structure brief

The structure brief is per document. **The reader outcome states what the
reader can find, understand and do. A readability score, grade level or
length limit is not an outcome and gates nothing.**

**The structure brief states the reader outcome and these fields:**

- **Expertise.** State the reader's expertise per domain and per
  representation, such as the subject, the notation and the tools. State
  also what needs no teaching for this reader.
- **Intended domain.** State the domain that the document serves. Keep it
  apart from the repository, ecosystem or examples that motivated the
  document. Tell the subject, its producers and its consumers apart.
- **Section plan.** Give the sections in conceptual order. The subject model
  and the reader's task come before the notation that expresses them. State
  the purpose of each page.
- **Entry routes.** Name the learning route through the document. Name also
  the direct entries that readers use, such as rules, reference anchors,
  examples and figures.
- **Emphasis and exclusions.** State what deserves emphasis and what the
  document leaves out.
- **Term risks.** List the terms whose everyday or neighbouring meaning
  intrudes. For each term, state the difference that the text must make.
- **Coverage inventory.** When the document claims coverage, enumerate the
  inventory from the subject itself, not from an earlier document.
- **Examples policy and figure needs.** State them as the reader's questions
  and levels, not as a count of examples or figures.

### Prose contract

The prose contract is per document set.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**The prose contract names its base
and exceptions, its keyword convention, term list, code typography and
example labels, and how normative text and aids are told apart.**

- **Base and exceptions.** The base is a named style guide, another prose
  contract by path, or the house style. The contract lists the exceptions
  that the set takes from it.
- **Keyword convention.** When normative keywords such as "must" and "should"
  appear, state what each keyword means and how the text marks it.
- **Term list.** Give each defined term once, with its meaning.
- **Code typography.** State how the text sets identifiers, code, commands
  and file names.
- **Example labels.** State how an example shows that it is complete or a
  fragment.
- **Normative text and aids.** State how a reader tells a normative statement
  from a non-normative aid, such as a note, an example or a figure.
- **Layout.** A layout convention, when the contract states one, governs the
  heading levels, lists and tables that a stage adds.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
<!-- tc: depart semicolon,long-paragraph because the conformance inventory pins this rule statement verbatim -->
**A set whose document contract declares no prose contract takes the house
style. A prose contract may name one base: a style guide, another prose
contract by path, or the house style. One that names no base builds on the
house style. Each point resolves by a walk that starts at the set's prose
contract, follows each base in turn, and ends at the house style: the first
contract on the walk that states the point governs it. A style guide on the
walk has the house style as its base. Where the guide settles a point
otherwise than the house style, the guide governs the whole point; every
other point comes from the house style. A contract the walk cannot follow
ends the walk at the house style for every point still unresolved, and the
session records the break once in its running log. The walk cannot follow a
path that names no file, resolved from the file that names it, or a file it
has already met, the starting contract included.**

A project-wide prose contract is one file that each set's contract names, as
its prose contract or as its base. A style guide is read for what it states
itself. The walk does not follow the guides that it defers to. When the
session cannot consult a named guide, the guide is still the base. The
session applies what it can establish of the guide and records what it
could not check. A recorded break blocks no stage.

When a set takes the house style, or a walk reaches it, read
`references/house-style.md`. That file states each point of the house style,
and it records which points Google's guide settles otherwise.

### Format and media

The format and media part is per document set. It states the format, such as
Markdown or a book source.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**Choose the format per document set, with the human. Weigh reader tasks,
publishing targets, source readability, accessibility, cross-reference
checking, reuse, generation and tool cost. Before any choice in the format
part (format, media, renderers or viewing environments) turns on a dated
capability fact, recheck the fact, and cite it with its date.**

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**The format part names the admitted figure media and renderers with pinned
versions, any generation step, and the viewing environments readers use.**

When you choose or check a format, renderer or viewing route, read
`references/documentation-toolchain.md`. That file holds dated capability
facts about formats, hosts, renderers and viewers. It states no rule.

### Reader checkpoints

The reader checkpoints part states when the intended reader, or a labelled
stand-in, reads the document. For each checkpoint, it names the tasks, the
entry surfaces and the viewing environment.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**A declared reader checkpoint is cut as a `requirements` leaf after the
document's `proof` by the session that cuts the document's leaf.** The
checkpoint runs outside the four editorial stages and mints no kind. The skill
of the `requirements` kind holds the checkpoint's procedure and reaches it
only in such a leaf. No stage result is read as reader understanding.

## Absent parts

A stage that meets an absent part applies its default and continues.

| Absent | Default |
|---|---|
| coverage inventory | the document makes no coverage claim |
| term risks, examples policy, figure needs | the stage charters' own rules apply unchanged |
| prose contract | the house style, in `references/house-style.md` |
| format and media | the set's existing format, figures in the page's own bytes only, and the raw file as the viewing environment |
| reader checkpoints | none is scheduled, and no stage result is read as reader understanding |

**An absent part takes the default in the table above and blocks no stage.**

A stage's own skill can require some fields of the structure brief. Such a
requirement is the stage's precondition, not a default of this file.

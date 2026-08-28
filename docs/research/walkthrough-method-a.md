# Evidence for a concept-ordered code walkthrough

## Scope and confidence

This survey asks how to explain a substantial codebase in a self-contained,
multi-page Markdown book for an expert technical reader. It draws technical
claims about the codebase from no external source; the sources below concern
program comprehension, instructional design, literate programming,
documentation structure, and navigation.

The strongest evidence is narrower than the desired artifact. Controlled
studies cover worked examples, split attention, redundancy, segmentation, and
program comprehension. Literate-programming manuals establish mechanisms for
reordering and reconstructing source. Maintained documentation guides establish
conventions that have survived sustained use. No primary study found in this
survey compares complete, source-reconstructing Markdown walkthroughs for
expert Rust developers. Recommendations that combine these bodies of evidence
are therefore marked as **inference** rather than reported as experimental
results.

## Findings at a glance

The book should have two simultaneous orders:

1. A **reader order** that introduces purpose, vocabulary, the public seam, and
   one complete operation before increasing resolution along causal paths.
2. A **source order** represented by a checked fragment graph whose file roots
   recursively expand to every in-scope file byte for byte, including the
   manifest.

The reader order should not be a directory listing and the source proof should
not depend on the reader following file order. Named fragments provide the
seam between the two.

For local context, use a semantic-dependency test rather than a global rule
against repetition. Repeat the shortest statement needed to understand the
current causal step. Link when the destination is optional depth, reference
material, or a substantial procedure that the current passage does not require.
Do not duplicate production code merely to restore local context; repeat the
meaning in prose and point to the fragment's single definition.

## 1. Concept ordering

### Established approaches

**Advance organizer.** Ausubel's experiment introduced higher-level concepts
before unfamiliar technical material and found better learning and retention
than an equal-length historical introduction. The experiment concerned novice
undergraduates and metallurgy, not expert programmers, so it supports an
initial conceptual frame but not a particular software chapter sequence
([Ausubel 1960](https://doi.org/10.1037/h0046669)).

**Program textbase followed by a functional model.** Pennington studied
professional programmers reading and modifying procedural programs. Control
flow and procedural relations dominated early representations; a functional,
problem-domain representation emerged later and required substantial
engagement. Pennington explicitly suggested that documentation connecting
procedures to the real-world domain might promote construction of both models.
The languages and programs were COBOL and FORTRAN, so the exact time course
must not be assumed for Rust, but the distinction between execution structure
and purpose remains useful
([Pennington 1987](https://www.sciencedirect.com/science/article/pii/0010028587900077)).

**Expert plans and discourse conventions.** In two empirical studies, Soloway
and Ehrlich found support for the claim that expert programmers recognize
stereotyped programming plans and rely on discourse conventions; plan-like
programs were easier for experts to comprehend than convention-violating ones
([Soloway and Ehrlich 1984](https://doi.org/10.1109/TSE.1984.5010283)). This
argues for naming recognizable operations and invariants before presenting
their details, and for explaining departures from normal Rust or systems
programming conventions where they occur. Later experimental work questioned
the universality of programming plans and found their development tied to a
programmer's learning experience and language notation
([Davies 1990](https://www.sciencedirect.com/science/article/pii/S0020737305801439)).
The book should therefore identify patterns evidenced by this code and its
audience, not impose a universal plan taxonomy.

**Human order in literate programming.** Knuth defined literate programming
around introducing concepts in the order best suited to human understanding,
with formal and informal accounts reinforcing each other
([Knuth 1984](https://www.cs.tufts.edu/~nr/cs257/archive/literate-programming/01-knuth-lp.pdf)).
CWEB operationalizes the idea with named sections that can be defined in one
place and spliced into another by `CTANGLE`
([CWEB manual](https://tug.ctan.org/web/cweb/cwebman.pdf)). This establishes
that explanation order and compiler order can be separated; it does not
establish which explanatory order is best for a given codebase.

### Recommended sequence

**Inference from the evidence:** use a whole-to-path-to-parts sequence.

1. **Purpose and boundary.** State what the system owns, what it deliberately
   leaves to consumers, and the observable result of using it.
2. **Vocabulary and invariant map.** Define only the terms needed for the first
   complete path. Show the stable identities, mutable positions, layers, and
   boundary objects in one compact organizer.
3. **Public seam.** Present the types and operations a consumer can see before
   internal machinery. This gives the expert reader handles for later details.
4. **One complete operation at low resolution.** Follow a representative call
   from input through classification, pure decision, filesystem interpretation,
   and returned result. At this stage, name each transition and suppress local
   implementation detail.
5. **Read path.** Expand discovery, parsing/classification, ordering, lookup,
   and snapshot formation.
6. **Mutation path.** Expand decisions, refusals, plans, primitive effects,
   locking, rollback, and reports, reusing the complete operation as an anchor.
7. **Boundary adapters.** Cover the reference domain and demonstration CLI
   after the library contract is stable in the reader's model.
8. **Trade-offs and alternatives.** Explain rejected or unavailable operations
   only after the implemented design is concrete.

This order gives the reader an advance organizer, respects evidence that
procedural relations are available early, and supplies the domain-to-procedure
connections that Pennington identified as a possible aid to functional
understanding. It also avoids turning the book into an API reference: the
public seam orients the walkthrough but the complete path supplies its spine.

## 2. Worked examples

Sweller and Cooper's five algebra experiments found that studying worked
examples could outperform conventional problem-solving practice during schema
acquisition. The authors also required transfer to be assessed with new
problems and cautioned that broader, longer-term evidence was still needed
([Sweller and Cooper 1985](https://doi.org/10.1207/s1532690xci0201_3)). Chi and
colleagues' protocol study found that stronger learners did more than inspect
worked solutions: they generated explanations that connected individual
actions to governing principles and monitored gaps in their understanding
([Chi et al. 1989](https://doi.org/10.1207/s15516709cog1302_1)).

For an expert walkthrough, the useful unit is therefore not a toy snippet but a
**worked execution**:

- Begin with a realistic request and its pre-state.
- Trace the exact production types and functions that carry it.
- At every layer boundary, state the input, output, preserved invariant, and
  reason the transition exists.
- Show refusal and failure forks adjacent to the successful path when they
  change the meaning of that transition.
- End with the filesystem state or other observable result, then summarize the
  causal chain in the system's vocabulary.

The prose should perform the self-explanation that strong learners generated in
Chi et al.'s study: not merely “this function runs next,” but “this layer must
produce this fact because the following layer is deliberately unable to infer
it.” A second example should be added only when it exercises a materially
different invariant or boundary. Variants that differ only in surface syntax
belong in concise comparisons or links.

Worked examples must be calibrated to expertise. Kalyuga, Chandler, and
Sweller found across three experiments that integrated explanatory text helped
less-experienced learners, while the same text became redundant and removing
it helped more-experienced learners
([Kalyuga, Chandler, and Sweller 1998](https://doi.org/10.1518/001872098779480587)).
For a Rust-proficient audience, explain the codebase-specific contract,
filesystem semantics, and non-obvious Rust choices; omit tutorials on ownership,
traits, iterators, or ordinary error propagation unless this code uses them in
an unusual way.

## 3. Progressive disclosure for an expert reader

Mayer's controlled slideshow experiment found better transfer and lower rated
difficulty for learner-paced small segments than for larger segments
([Mayer 2019](https://doi.org/10.1002/acp.3560)). Progressive disclosure in
interface design similarly keeps primary choices visible and defers secondary
ones, but its practitioner guidance warns that more than two disclosure levels
often causes users to become lost
([Nielsen 2006](https://www.nngroup.com/articles/progressive-disclosure/)).

**Inference:** a Markdown book should implement progressive disclosure through
resolution, not hidden facts.

- The opening path is complete but low-resolution: it names every major stage
  and omits local mechanics.
- Later chapters revisit those stages at high resolution.
- Within a page, the load-bearing explanation and code remain visible.
- Cross-references offer one step of optional depth: API detail, an alternative
  path, a proof, a test, or historical rationale.
- A reader never needs to follow a chain of links to assemble one current
  claim.

The approach differs from simplifying for novices. The primary layer still
states exact types, invariants, and effects. It defers breadth and local
mechanism, not precision.

Diátaxis supplies a useful role distinction: tutorials serve learning by
action, reference serves factual lookup, and explanation serves understanding.
Its author identifies blurred documentation modes as a common source of
documentation problems
([Diátaxis overview](https://diataxis.fr/start-here/)). This is maintained
practitioner guidance rather than controlled evidence. Applied here, the book
is primarily explanation organized around worked executions. API tables and
CLI syntax should retain a compact reference form, but they should not take over
the chapter order; procedural setup should remain subordinate to the concept
being explained.

## 4. Local repetition versus cross-reference

### Evidence

Chandler and Sweller's two experiments found that learners who received
physically integrated, mutually referring material outperformed learners who
had to integrate separated material; in one experiment they also spent less
time processing it
([Chandler and Sweller 1992](https://doi.org/10.1111/j.2044-8279.1992.tb01017.x)).
The expertise experiments above show the countervailing redundancy effect:
material required by a less-experienced reader can impose avoidable load once
the reader already has the relevant schema
([Kalyuga, Chandler, and Sweller 1998](https://doi.org/10.1518/001872098779480587)).

Google's maintained developer-documentation guide translates the same tension
into an operational rule. It says cross-references should generally point to
nonessential information, recommends putting short definitions, brief concept
explanations, and a couple of necessary steps in context, and warns that each
link adds a decision and a chance to lose one's place
([Google cross-reference guidance](https://developers.google.com/style/cross-references)).
This is first-party guidance, not experimental validation.

### Decision procedure

For every contemplated link, apply these tests in order:

1. **Dependency:** Can the target reader explain the current code transition
   and continue reading without opening the destination? If no, put the needed
   context here.
2. **Size:** Can the missing context be stated accurately in one to three
   sentences? If yes, state it locally and optionally link for depth. If no,
   provide a precise local summary and link to the substantial treatment.
3. **Expertise:** Is the context ordinary Rust or operating-system knowledge
   already promised by the audience contract? If yes, omit it. Is it a
   codebase-specific meaning or invariant? If yes, state it.
4. **Coupling:** Must two pieces be mentally combined to understand a single
   diagram, code fragment, or causal claim? If yes, integrate them physically
   on the same page and as close as Markdown permits.
5. **Redundancy:** Does the proposed repetition add a relation, condition, or
   consequence not already evident at this point? If no, remove it.
6. **Drift:** Would repeating the material create a second authoritative copy
   of code, syntax, a list of variants, or a detailed procedure? If yes, keep
   one authority and link to it; repeat only the current implication.

The distinction is between **semantic repetition** and **artifact duplication**.
Repeating “the key survives every rename” at the mutation where that fact bears
the reasoning can reduce split attention. Copying the same source fragment into
two chapters creates two artifacts that can drift and adds redundancy for the
expert. The book should permit the first and mechanically prohibit or detect
the second.

Links should name both destination and purpose. W3C requires link purpose to be
determinable from the text or its immediate context and requires descriptive
headings that communicate topic or purpose
([WCAG link purpose](https://www.w3.org/WAI/WCAG22/Understanding/link-purpose-in-context.html),
[WCAG headings](https://www.w3.org/WAI/WCAG22/Understanding/headings-and-labels)).
Use “Rollback after a failed effect explains the unwind order,” not “see
below.” Prefer a backward link to already-established optional detail. If a
forward link carries required context, the chapter order is wrong or the
current page needs a local summary.

## 5. Explanatory order with exact source reconstruction

### What prior systems establish

CWEB sections contain documentation, definitions, and program fragments.
`CTANGLE` recursively replaces named section references with their definitions;
`CWEAVE` generates cross-referenced human documentation. The manual recommends
small, comprehensible sections and meaningful section names
([CWEB manual](https://tug.ctan.org/web/cweb/cwebman.pdf)). Noweb generalizes
named chunks across target languages and extracts a program by recursively
expanding a root chunk. Ramsey's paper also records that WEB's complexity was a
barrier and that noweb deliberately used a smaller syntax
([Ramsey 1994](https://www.cs.tufts.edu/~nr/pubs/lpsimp.pdf)). Org Babel retains
the named-reference model and can select whether expansion occurs during
evaluation, tangling, or export
([Org noweb reference syntax](https://orgmode.org/manual/Noweb-Reference-Syntax.html)).

These systems prove the feasibility of separating presentation order from
program order. They do not by themselves prove that every intended production
file is present, that no source bytes are duplicated or omitted, or that the
tangled result still equals an independently maintained working tree. Those are
validation obligations for this book.

### Recommended fragment contract

**Inference:** use Markdown-native, inert markers and a deliberately small
tangler rather than adopting a full literate-programming environment.

- Every displayed in-scope fragment has one globally unique
  `«fragment-id»` definition.
- Every in-scope file, including a manifest, has one root fragment identified
  with its repository path.
- A fragment contains literal source text and explicit insertion references.
  Each reference names the child and states its exact insertion point.
- The reader may encounter definitions in any chapter order. Expansion order
  follows references, not page position.
- The reference graph must be acyclic; every reference must resolve; every
  non-root production fragment must be reachable from exactly the intended
  file root.
- Recursive expansion must yield one byte stream per in-scope file. A validator
  compares that stream byte for byte with the repository source.
- A coverage validator maps every in-scope file byte or line to exactly one
  fragment definition and rejects gaps and unintended overlaps.
- Fragment identifiers and references remain readable in raw Markdown. The
  generated site improves navigation but is not required to understand the
  graph.

The root fragment can appear early as a compact skeleton containing references
to fragments defined later. This gives the reader an overview without copying
implementation. Later pages define the referenced fragments beside their
explanations. Tangling recursively replaces the references, reconstructing
file order independently of chapter order.

### Comparison with source inclusion

mdBook can include whole files, line ranges, or anchored regions, and its
documentation recommends anchors over line numbers to avoid breakage after
edits
([mdBook includes](https://rust-lang.github.io/mdBook/format/mdbook.html)).
It can also compile-test Rust code blocks
([mdBook test](https://rust-lang.github.io/mdBook/cli/test.html)). This is strong
for keeping examples synchronized with source, but an include graph reads from
source order; it does not demonstrate that explanatory-order fragments
reconstruct every file. The proposed validator should therefore use tangling
and byte comparison for the completeness claim. mdBook may still render the
pages and test examples.

## 6. Failure modes and mitigations

### Directory-order exposition

**Failure.** Walking files in repository order makes source layout determine
the learning sequence. Pennington's results distinguish early procedural
understanding from later functional understanding, while literate programming
exists specifically to free exposition from compiler order
([Pennington 1987](https://www.sciencedirect.com/science/article/pii/0010028587900077),
[Knuth 1984](https://www.cs.tufts.edu/~nr/cs257/archive/literate-programming/01-knuth-lp.pdf)).

**Mitigation.** Establish purpose and the public seam, trace one complete
operation, and introduce files only where they enter that causal path. Provide
a source index separately for lookup and coverage.

### Fragmented code with hidden state or unverifiable order

**Failure.** Jupyter permits cells to execute in an order different from their
display order and stores code, prose, output, and metadata in JSON
([Jupyter notebook format](https://nbformat.readthedocs.io/en/latest/format_description.html)).
In a study of 1,159,166 notebooks, only 24.11% of attempted valid notebook
executions completed without errors and 4.03% produced the same results; the
reported causes included missing dependencies, hidden state, out-of-order
execution, and inaccessible data
([Pimentel et al. 2019](https://leomurta.github.io/papers/pimentel2019a.pdf)).
This study concerns computational notebooks, not static code books, but it is
direct evidence that apparent literate order is not an execution or
reconstruction guarantee.

**Mitigation.** Keep the book inert. Give fragment references one deterministic
recursive expansion, compare generated files with source, and run the crate's
ordinary build and tests independently. Do not permit hidden execution state in
the documentation pipeline.

### Unstable line-range includes and stale copied snippets

**Failure.** Line-based includes change meaning when preceding source changes;
mdBook's own manual recommends anchors to avoid this breakage
([mdBook includes](https://rust-lang.github.io/mdBook/format/mdbook.html)).
Copied snippets have no equivalent built-in connection and can silently drift.

**Mitigation.** Address production code by stable fragment identity, not line
number. Reject duplicate fragment definitions, tangle all roots, and compare
exact bytes. Treat illustrative pseudocode as explicitly non-production and
outside source coverage.

### Too many links and non-linear navigation

**Failure.** An experiment comparing linear, hierarchical, and non-linear
hypertexts found better navigation performance with linear text than with the
non-linear form; hierarchy fell between them, and decision-making and visual
processing demands impaired performance
([McDonald and Stevenson 1996](https://www.sciencedirect.com/science/article/pii/0003687095000739)).
A separate map study found that greater map use produced less relevant search
and less effective search effort, so adding a graphical overview is not a
universal remedy
([Stanton, Correia, and Dias 2000](https://bura.brunel.ac.uk/handle/2438/2065)).

**Mitigation.** Preserve one canonical linear reading order, a shallow chapter
hierarchy, descriptive headings, previous/next navigation, and a source index.
Use links for optional depth and lookup rather than making the reader choose the
next required step. Test navigation with real comprehension tasks, not merely
with link validity.

### Split explanations

**Failure.** Separating mutually dependent code and explanation forces mental
integration and can reduce performance
([Chandler and Sweller 1992](https://doi.org/10.1111/j.2044-8279.1992.tb01017.x)).

**Mitigation.** Put a fragment's purpose, inputs, outputs, invariants, and
immediate consequence beside it. Repeat a short governing fact when necessary;
link only for optional depth.

### Expert-facing redundancy

**Failure.** Explanations that assist less-experienced readers can become
redundant and harmful as expertise increases
([Kalyuga, Chandler, and Sweller 1998](https://doi.org/10.1518/001872098779480587)).

**Mitigation.** Write an explicit audience contract. Explain domain-specific
semantics and surprising mechanisms, not ordinary language constructs. During
editorial review, delete sentences that neither introduce a codebase concept
nor establish a relation needed at that point.

### Mixed documentation roles

**Failure.** Diátaxis identifies conflation of tutorial, how-to, reference, and
explanation as a recurring documentation problem because each serves a
different reader need
([Diátaxis overview](https://diataxis.fr/start-here/)). This is a methodology
claim from its maintainer; no primary comparative evaluation was found.

**Mitigation.** Make explanation the book's dominant mode. Keep CLI invocation
and API facts in clearly bounded reference sections, and use a worked operation
as evidence within the explanation rather than turning the book into a setup
tutorial.

### Tool-dependent unreadability

**Failure.** A documentation artifact can satisfy its build tool while ceasing
to be legible as ordinary text. Jupyter's authoritative source is JSON, and raw
mdBook files can contain unresolved preprocessor directives
([Jupyter notebook format](https://nbformat.readthedocs.io/en/latest/format_description.html),
[mdBook preprocessors](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html)).

**Mitigation.** Keep prose, headings, links, fragment labels, and references
meaningful in raw Markdown. Rendered output is an enhancement. A checkout with
no documentation tool installed must still expose the chapter order, prose,
code, and reconstruction relationships.

## 7. Precise expert-facing prose

The RFC Style Guide requires a concise, comprehensive overview for technically
knowledgeable readers and a document introduction that explains motivation and
applicability
([RFC 7322](https://www.rfc-editor.org/rfc/rfc7322.html)). Google's maintained
developer style guide recommends active voice, descriptive headings, explicit
actors, meaningful link text, and code formatting for program entities
([Google style highlights](https://developers.google.com/style/highlights),
[Google headings](https://developers.google.com/style/headings),
[Google code in text](https://developers.google.com/style/code-in-text)). W3C
likewise ties descriptive headings and consistent navigation to orientation
([WCAG headings](https://www.w3.org/WAI/WCAG22/Understanding/headings-and-labels),
[WCAG consistent navigation](https://www.w3.org/WAI/WCAG22/Understanding/consistent-navigation.html)).

Applied to this book:

- Open each page with its claim and scope, not a preview of the writing process.
- Give every sentence an explicit actor when actor identity affects behavior:
  “the algebra returns,” “the interpreter renames,” “the consumer supplies.”
- Put conditions before consequences and keep one causal relation per sentence
  when the relation is dense.
- Use one stable term for each concept. Define a term before using it in a
  load-bearing claim; do not rotate synonyms for variety.
- Make headings describe concepts or operations, not rhetorical stages such as
  “The surprising part.”
- Distinguish refusal, environmental failure, programmer defect, and invariant
  violation rather than grouping them under “error.”
- State why a mechanism exists at the first place where its absence would make
  the code confusing.
- Label inference as inference and place source-backed facts before it.
- Delete narrative suspense, rhetorical questions, praise, metaphor, and claims
  that code is “simple,” “obvious,” or “elegant.”

## 8. Walk-away checks

| System or method | What remains legible with it uninstalled? | Consequence for this book |
|---|---|---|
| CWEB/WEB | The `.w` source remains text, but TeX/CWEB control syntax and shuffled sections require learned conventions; committed tangled C remains ordinary source. Generated cross-indexes are unavailable. | Borrow named fragments and recursive expansion, not the authoring syntax or language coupling. |
| noweb | The `.nw` file remains mostly plain prose and code with visible `<<name>>=` markers. Tangling, weaving, and generated indexes are unavailable. Ramsey's repository also describes its old source build as “baroque and brittle” ([noweb repository](https://github.com/nrnrnr/noweb)). | Prefer a smaller Markdown-native marker set and a validator that is easy to replace. |
| Org Babel | The `.org` file remains structured text, but its execution/tangle/export behavior depends on Org header arguments and Emacs tooling ([Org manual](https://orgmode.org/manual/Noweb-Reference-Syntax.html)). | Avoid executable-document state and tool-specific authoring format. |
| Jupyter | The authoritative `.ipynb` file is JSON containing cells, outputs, and metadata; a person can recover text manually, but normal reading is poor without Jupyter or an export ([nbformat](https://nbformat.readthedocs.io/en/latest/format_description.html)). | Reject notebook format as the durable book source. |
| mdBook | Chapter files and `SUMMARY.md` remain readable Markdown, but includes and custom preprocessors remain unresolved; repository source is separately legible. | Suitable as an optional renderer and Rust example tester, provided raw pages still make sense and exactness uses an independent validator. |
| Diátaxis | It installs nothing and imposes no source format; the resulting documents remain whatever ordinary format their authors chose. | Borrow role distinctions without encoding them as required metadata or navigation machinery. |

The desired artifact should pass a stronger walk-away check than any
tool-dependent renderer: after deleting the walkthrough skill, tangler, and
renderer, a reader can still open Markdown pages, follow the linear order,
understand fragment identities and insertion relationships, and read every
production fragment. What disappears is automated proof and presentation
enhancement, not meaning.

## 9. Explicit answers for the book-system design

### Chapter and page contract

Use a dependency-ordered chapter sequence: purpose and vocabulary; public seam;
complete-operation tour; reads; mutations; filesystem interpretation and
rollback; reference domain; CLI; trade-offs. Each page should state its purpose
and prerequisite concepts, develop one principal causal claim, keep the needed
code and explanation together, and end only with optional depth or navigation.

Use a canonical linear order plus three lookup paths: a table of contents, a
concept index/glossary, and a source-file/fragment index. W3C recommends more
than one way to locate pages and specifically identifies tables of contents and
indexes as useful for digital publications
([WCAG multiple ways](https://www.w3.org/WAI/WCAG22/Understanding/multiple-ways)).

### Fragment contract

Store every in-scope fragment once under a unique `«fragment-id»`; give each
file, including the manifest, one root; use explicit, human-readable child
references at insertion points; and let recursive expansion determine source
order. Require complete reachability, acyclicity, resolved references, unique
definitions, exact source
coverage, and byte equality with the working tree. Keep code repetition out of
the prose layer; repeat only short semantic context.

### Authoring policy

Write the low-resolution complete operation before detailed chapters. Use it to
discover missing vocabulary and chapter dependencies. Define the fragment roots
and graph before dispersing fragments across the book, then author pages in
reader order. Calibrate explanations against the audience contract and apply the
local repetition decision procedure during every page review.

### Verification policy

Mechanical checks must prove fragment uniqueness, reference resolution,
acyclicity, root reachability, exact source coverage, byte-for-byte tangling,
Markdown heading structure, and local-link validity. Run the crate's ordinary
formatting, build, lint, and test commands separately. Commission technical
review for claims and causal traces and editorial review for concept order,
local completeness, terminology, and redundancy. Code-block compilation alone
is insufficient: mdBook itself describes its test command as testing available
Rust examples, not source reconstruction
([mdBook test](https://rust-lang.github.io/mdBook/cli/test.html)).

## 10. Explicit answers for the generic walkthrough skill

### Elicitation

Ask one question at a time and record the answer before the next:

1. What repository, package, executable, or bounded subsystem is the target?
2. Which manifests and production files are in scope, and which tests, models,
   generated files, examples, and dependencies are evidence only?
3. What language, systems, and domain knowledge may the reader already use
   without explanation?
4. What depth is required: orientation, subsystem, complete production source,
   or another explicit level?
5. What output form is required: one document, multi-page Markdown, a rendered
   book, or a tool-specific format?
6. What prose, vocabulary, citation, and navigation constraints govern the
   artifact?
7. What must verification prove: compilation, tests, source coverage, exact
   tangling, links, technical review, editorial review, or a stated subset?

Do not ask for a preferred chapter list before inspecting the system. Chapter
order is a research result derived from purpose, seams, complete operations,
and dependencies, not a formatting preference.

### Authoring procedure

1. Freeze the source inventory and evidence-only exclusions.
2. Extract the system vocabulary, public seam, invariants, boundary effects,
   and one representative end-to-end operation.
3. Build a concept dependency graph and select a whole-to-path-to-parts reading
   order.
4. Define one file root per in-scope source file and partition production source
   into uniquely identified fragments.
5. Validate the fragment graph before prose depends on it.
6. Write the purpose, organizer, public seam, and low-resolution worked
   operation.
7. Expand later chapters along causal paths, placing fragment definitions where
   they best support understanding.
8. Apply the repetition-versus-link tests at each dependency boundary.
9. Edit for stable terms, explicit actors, direct claims, and expert-calibrated
   detail.
10. Build the source index and concept index after the content stabilizes.

### Verification procedure

Run checks in this order so failures identify one contract at a time:

1. Inventory: every in-scope file has one root and no excluded file appears.
2. Graph: fragment IDs are unique, references resolve, expansion is acyclic,
   and all production fragments are reachable.
3. Exactness: tangled roots equal all in-scope files byte for byte and coverage
   has neither gaps nor unintended overlaps.
4. Markdown: headings form a valid hierarchy; internal anchors, chapter links,
   and fragment links resolve; navigation is consistent.
5. Codebase: existing format, build, lint, and test suites pass.
6. Technical review: a fresh reader checks each public contract, invariant,
   control/data-flow trace, concurrency claim, rollback claim, and error/refusal
   distinction against authoritative repository evidence.
7. Editorial review: a fresh reader checks concept dependency order, local
   self-containedness, unnecessary repetition, terminology, and the promised
   audience level.
8. Walk-away review: raw Markdown remains intelligible with all custom book and
   skill tooling absent.

## Search silence and limits

- No controlled comparison was found for alternative chapter orders in a
  complete, source-reconstructing walkthrough of a modern Rust codebase.
- No primary study was found that tests the local-repeat-versus-link decision in
  long expert code books specifically. The rule above is an inference from
  split-attention and expertise-reversal experiments plus maintained developer
  documentation guidance.
- No empirical evaluation was found for Diátaxis applied to code walkthroughs.
  Its taxonomy is used as maintained practitioner guidance only.
- No primary evidence was found for a universal ideal fragment size. CWEB's
  “about a dozen lines” is its manual's heuristic, not a threshold for Rust.
- No prior system found in this survey combines raw Markdown walk-away
  readability, arbitrary explanatory order, unique fragment identity, complete
  production-source coverage, and byte-for-byte reconstruction as one stated
  contract. The proposed fragment validator therefore needs project-specific
  tests rather than an appeal to prior art.

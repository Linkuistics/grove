# Evidence base for a concept-ordered, exact-source code walkthrough

## Scope and evidence policy

This synthesis reconciles the independent `walkthrough-method-a` and
`walkthrough-method-b` surveys. It concerns communication and learning design
for a self-contained Markdown book aimed at a Rust- and operating-system-
proficient reader. Repository source, tests, models, and authoritative dependency
documentation remain the evidence for claims about `ordinal-fs-tree` itself.

The evidence does not directly evaluate the proposed artifact. No source in
either survey reports a controlled comparison of complete, byte-reconstructing
code walkthroughs for expert Rust developers. Literate-programming sources
establish mechanisms and report experience; they do not establish reader
outcomes. Recommendations below that combine program-comprehension,
instructional-design, documentation, and literate-programming evidence are
therefore labelled **Project inference**.

Confidence labels mean:

- **High**: several relevant primary or authoritative sources support the
  underlying principle, with at least one materially independent line of
  evidence.
- **Moderate**: the evidence is credible but transferred from another medium,
  population, or task, or the recommendation combines evidence with substantial
  project reasoning.
- **Low**: the claim rests on practitioner guidance, an abstract or secondary
  account, or a project-specific design for which no direct precedent was found.

## Agreements and independence audit

Agreement is counted only when the surveys reached it through materially
different primary evidence. Shared citations and citations from the same
cognitive-load research lineage are correlated evidence, not independent
confirmation.

| Agreed claim | Survey A evidence | Survey B evidence | Independence and confidence |
|---|---|---|---|
| Explanatory order should differ from file order and should establish a whole before expanding its parts. | Ausubel's advance-organizer experiment, Pennington's program/situation models, and Knuth/CWEB's human ordering ([Ausubel 1960](https://doi.org/10.1037/h0046669), [Pennington 1987](https://www.sciencedirect.com/science/article/pii/0010028587900077), [Knuth 1984](https://www.cs.tufts.edu/~nr/cs257/archive/literate-programming/01-knuth-lp.pdf)). | Reigeluth and Stein's epitome-and-elaboration sequence, plus program-comprehension models reviewed by O'Brien ([Reigeluth and Stein 1983](https://ocw.metu.edu.tr/pluginfile.php/9337/mod_resource/content/1/Reigeluth%201983%20Article.pdf), [O'Brien 2003](https://www.st.cs.uni-saarland.de/edu/empirical-se/2006/PDFs/brien03.pdf)). | The instructional sources are independent; the program-comprehension line partially overlaps. **High** for rejecting file order; **moderate** for the exact chapter sequence. |
| A low-resolution complete operation should organize the walkthrough. | Worked-example and self-explanation studies support a complete, principle-linked execution rather than an unexplained snippet ([Sweller and Cooper 1985](https://doi.org/10.1207/s1532690xci0201_3), [Chi et al. 1989](https://doi.org/10.1207/s15516709cog1302_1)). | Elaboration Theory's application-level epitome and Letovsky and Soloway's findings about delocalized plans support a real operation that exposes goals across sites ([Reigeluth and Stein 1983](https://ocw.metu.edu.tr/pluginfile.php/9337/mod_resource/content/1/Reigeluth%201983%20Article.pdf), [Letovsky and Soloway 1986](https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf)). | Different research traditions converge. The transfer from algebra instruction and legacy-language maintenance remains material. **Moderate-high**. |
| Explanatory detail must be calibrated by expertise, and this audience's Rust expertise does not imply expertise in the crate's domain. | Integrated explanation became redundant for experienced learners in the expertise-reversal experiments ([Kalyuga, Chandler, and Sweller 1998](https://doi.org/10.1518/001872098779480587)). | The expertise-reversal review and the API-documentation defect survey support fading familiar mechanics while still explaining the unfamiliar domain ([Kalyuga et al. 2003](https://link.springer.com/article/10.1007/s11251-009-9102-0), [Uddin and Robillard 2015](https://www.cs.mcgill.ca/~martin/papers/ieeesw2015.pdf)). | The Kalyuga line is shared and B did not obtain its primary text. Uddin and Robillard independently establish that unexplained examples are a defect, not the expertise boundary itself. **Moderate**. |
| Required context should be local when separated passages must be mentally integrated; redundant restatement should be removed when each passage is independently intelligible. | Split-attention experiments, expertise reversal, and maintained cross-reference guidance ([Chandler and Sweller 1992](https://doi.org/10.1111/j.2044-8279.1992.tb01017.x), [Kalyuga, Chandler, and Sweller 1998](https://doi.org/10.1518/001872098779480587), [Google cross-references](https://developers.google.com/style/cross-references)). | Ayres and Sweller's explicit boundary conditions, Letovsky and Soloway's delocalized-plan findings, and Uddin and Robillard's fragmentation/bloat defects ([Ayres and Sweller](https://www.davidlewisphd.com/courses/EDD8121/readings/2006-AyersSweller.pdf), [Letovsky and Soloway 1986](https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf), [Uddin and Robillard 2015](https://www.cs.mcgill.ca/~martin/papers/ieeesw2015.pdf)). | The cognitive-load citations are correlated, but the program-comprehension and documentation-defect evidence are independent. **High** for the semantic-dependency rule; **moderate** for applying it page by page in this book. |
| Named fragments can separate reader order from source order, but the mechanism itself supplies no good reader order. | CWEB, noweb, and Org Babel establish recursive named expansion; the survey separately derives the learning sequence ([CWEB manual](https://tug.ctan.org/web/cweb/cwebman.pdf), [Ramsey 1994](https://www.cs.tufts.edu/~nr/pubs/lpsimp.pdf), [Org noweb syntax](https://orgmode.org/manual/Noweb-Reference-Syntax.html)). | noweb and Letovsky and Soloway establish arbitrary ordering and its lookup/delocalization costs ([Ramsey 1994](https://mirror.gutenberg-asso.fr/tex.loria.fr/litte/ieee.pdf), [Letovsky and Soloway 1986](https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf)). | Ramsey is shared, so agreement about the mechanism is one-source agreement. Letovsky and Soloway independently support the navigation warning. **High** for mechanism feasibility; **moderate** for the proposed controls. |
| A canonical linear reading order should coexist with lookup paths, while links should not carry required context. | Hypertext navigation experiments, W3C navigation guidance, and Google link guidance ([McDonald and Stevenson 1996](https://www.sciencedirect.com/science/article/pii/0003687095000739), [Stanton, Correia, and Dias 2000](https://bura.brunel.ac.uk/handle/2438/2065), [WCAG multiple ways](https://www.w3.org/WAI/WCAG22/Understanding/multiple-ways), [Google cross-references](https://developers.google.com/style/cross-references)). | Letovsky and Soloway's lookup-cost analysis and Uddin and Robillard's fragmentation findings ([Letovsky and Soloway 1986](https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf), [Uddin and Robillard 2015](https://www.cs.mcgill.ca/~martin/papers/ieeesw2015.pdf)). | Independent experimental, program-comprehension, and practitioner evidence converge. **High** for preserving a linear path; **moderate** for the exact index set. |
| The book and source should remain independently legible, with tooling proving equality rather than becoming the only readable representation. | Tool-by-tool walk-away checks and evidence about hidden notebook state ([Pimentel et al. 2019](https://leomurta.github.io/papers/pimentel2019a.pdf), [Jupyter format](https://nbformat.readthedocs.io/en/latest/format_description.html)). | A separate walk-away comparison of WEB, noweb, reverse literate programming, Entangled, Org, and mdBook. | The conclusion is project reasoning; the tool assessments contain disagreements recorded below. **Moderate** for the project requirement, **low** as a general comparative claim about the tools. |

## Disagreements and unresolved tensions

### 1. What comes before the complete operation

Survey A recommends purpose and boundary, vocabulary, and the public seam before
the first complete operation. Survey B warns that a purely top-down architecture
opening conflicts with bottom-up comprehension under domain unfamiliarity and
defines the first operation as Reigeluth and Stein's application-level epitome.

These positions are compatible only if the preliminary material is kept inside
the same opening unit and limited to what the operation needs. A detached,
high-resolution architecture or API chapter before the operation would satisfy
A's list while violating B's evidence-based warning.

**Resolution:** open with purpose and observable boundary, introduce the minimum
vocabulary and public types required to name the operation, then trace the real
operation at low resolution. Expand the public seam and architecture after the
trace. This preserves the root brief's purpose/public-behaviour requirement
without delaying the application-level whole. **Confidence: moderate.**

### 2. Which test controls local repetition

Survey A proposes six ordered tests: dependency, size, expertise, coupling,
redundancy, and drift. Its size test permits a short local restatement whenever
missing context can be stated in one to three sentences. Survey B gives the
narrower boundary from Ayres and Sweller: integration is warranted only for
high-element-interactivity material whose sources are not intelligible in
isolation. A short statement can fail that boundary and still pass A's size
test.

**Resolution:** semantic dependency controls; size only controls the form of an
already-required local statement. The decision rule below therefore adopts B's
two boundary tests and retains A's expertise and drift checks as safeguards.
**Confidence: high** for the boundary, **moderate** for the operational wording.

### 3. Whether fragment identifiers may have continuations

Survey A requires every fragment identifier to have one globally unique
definition. Survey B rejects noweb's implicit same-name concatenation but leaves
two acceptable choices: forbid redefinition or require an explicit continuation
marker.

**Resolution:** forbid redefinition. The agreed exactness criterion needs no
continuation feature, and a one-definition rule makes duplicate detection and
source ownership local and deterministic. If later authoring demonstrates a
real need, an explicit continuation syntax must be designed and validated as a
separate change. **Confidence: moderate.**

### 4. Whether generated fragment cross-references are optional

Survey A says generated presentation may improve navigation but raw Markdown
must remain sufficient. Survey B says a rendered statement of where every
fragment is inserted and what it inserts is mandatory because named fragments
create hidden dependencies.

**Resolution:** the relationship is mandatory; a particular renderer is not.
Every definition and insertion reference must be understandable in raw
Markdown, and mechanical validation must generate or check both inbound and
outbound relationships. A rendered index may expose the same data more
conveniently, but the book cannot depend on that rendering for meaning.
**Confidence: moderate.**

### 5. How literate tools survive removal

The surveys disagree about WEB/CWEB and noweb under the walk-away test. Survey A
counts committed tangled source as ordinary, surviving code and describes the
literate inputs as readable with learned conventions. Survey B treats tangled
output as absent or unsuitable and concludes that deleting the tool orphans the
build. The difference partly depends on repository policy—whether generated
source is committed—and partly on a qualitative judgment about readability.

They also assess Entangled differently. A treats its round-trip as a useful
comparison; B could not verify the marker format and explicitly weakens its own
assessment.

**Resolution:** do not use either disputed tool comparison as a design premise.
Apply a project-local acceptance test instead: ordinary crate sources remain the
compiled source of truth; Markdown contains the full source text rather than
include placeholders; removing the validator leaves both artifacts readable;
and the lost capability is equality proof only. **Confidence: high** for the
acceptance test, **low** for generalizing the comparative tool table.

### 6. Whether every page should stand alone

Survey A emphasizes locally complete pages and a canonical linear order. Survey
B identifies a real tension between an elaborative book sequence and the
Every-Page-Is-Page-One position, for which B obtained only secondary material.

**Resolution:** the book is self-contained as a whole, and each page is locally
complete for its principal claim given explicitly named prerequisites. A page
need not restate the whole book for arbitrary entry. Navigation and a concise
local summary support non-linear entry without turning each page into an
independent manual. **Confidence: moderate.**

### 7. How many disclosure levels are safe

Survey A cites practitioner guidance warning that more than two progressive-
disclosure levels can disorient users. Survey B treats disclosure as a property
of the fragment notation and reports no empirical evidence specific to
long-form technical books.

**Resolution:** use two explanatory resolutions—complete low-resolution path,
then detailed expansions—as a design constraint for this book, not as a
universal measured threshold. Optional reference material may sit beside those
resolutions but must not form a required third link-following layer.
**Confidence: low-moderate.**

## Reconciled method for this book

### Concept ordering

**Project inference:** build a concept dependency graph from the system's
observable purpose, vocabulary, public seam, representative operations,
invariants, and boundary effects. Use that graph to order concepts rather than
using directories, modules, or file positions.

The canonical reading order is:

1. State the system's purpose, ownership boundary, and observable result.
2. Introduce only the domain terms and public types needed for one real
   operation.
3. Trace that operation end to end at low resolution. Name each stage, the
   value passed across it, and the reason the boundary exists; defer local
   mechanics.
4. Expand the public seam and read path.
5. Expand mutation decisions, refusals, plans, effects, concurrency, rollback,
   and reports along causal dependencies.
6. Explain the filesystem interpreter, reference domain, and demonstration CLI
   after the library model is stable.
7. Reassemble cross-cutting invariants and trade-offs, returning to the opening
   operation at full resolution.

The opening operation is an epitome, not a summary: it uses real production
types, real values, and every major stage, while suppressing only breadth and
local mechanism. Reigeluth and Stein support the whole-then-elaborate shape;
Pennington and Letovsky/Soloway support connecting execution sites to functional
goals ([Reigeluth and Stein 1983](https://ocw.metu.edu.tr/pluginfile.php/9337/mod_resource/content/1/Reigeluth%201983%20Article.pdf), [Pennington 1987](https://www.sciencedirect.com/science/article/pii/0010028587900077), [Letovsky and Soloway 1986](https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf)).

### Worked examples

Use one representative successful operation as the book's spine. For every
stage, state:

- the production type or function that acts;
- its concrete input and output;
- the codebase-specific invariant it preserves or establishes;
- why the following layer cannot or should not infer that fact itself; and
- adjacent refusal or environmental-failure forks when they change the stage's
  meaning.

End the operation with the observable filesystem and returned result, then
summarize the causal chain in the system's vocabulary. This applies the
self-explanation finding to a code walkthrough without claiming that the
worked-example effect transfers unchanged from algebra
([Sweller and Cooper 1985](https://doi.org/10.1207/s1532690xci0201_3), [Chi et al. 1989](https://doi.org/10.1207/s15516709cog1302_1)).

Add another fully worked operation only when it introduces a materially
different invariant, boundary, rollback shape, or refusal class. Express
surface variants as compact comparisons. Fade guidance as codebase-domain
knowledge accumulates, while never spending worked detail on ordinary Rust,
trait, iterator, `Result`, or OS concepts already granted by the audience
contract unless this code uses them unexpectedly. **Confidence: moderate.**

### Progressive disclosure

Progressive disclosure changes resolution, not truth:

- The opening trace names all major stages and their contracts.
- Detailed chapters revisit those same stages and expose their mechanisms.
- Load-bearing prose and source fragments remain visible in Markdown; no
  required fact is hidden behind a link, collapsed section, renderer, or tool.
- Optional depth is one hop away: a proof, test, alternate path, exhaustive API
  table, or historical rationale.
- A reader never follows a chain of links to assemble the current causal claim.

The primary layer remains precise about types, invariants, and effects. It
defers breadth and mechanism rather than simplifying the system into an
inaccurate overview. Evidence for learner-paced segmentation supports smaller
units, but not this exact book structure ([Mayer 2019](https://doi.org/10.1002/acp.3560)).

### Navigation

Maintain one canonical linear reading order with previous/next movement and a
shallow heading hierarchy. Add three independent lookup paths:

1. a table of contents for reader order;
2. a concept index or glossary for vocabulary and invariants; and
3. a source-file/fragment index for coverage and insertion relationships.

Links name both destination and purpose. Forward links carry optional depth
only. If a forward destination is required to understand the current passage,
the author either moves the dependency earlier or states the required context
locally. This follows experimental evidence that non-linear hypertext adds
navigation cost and accessibility guidance that link purpose and headings must
be descriptive ([McDonald and Stevenson 1996](https://www.sciencedirect.com/science/article/pii/0003687095000739), [WCAG link purpose](https://www.w3.org/WAI/WCAG22/Understanding/link-purpose-in-context.html), [WCAG headings](https://www.w3.org/WAI/WCAG22/Understanding/headings-and-labels)).

### Local repetition versus cross-reference

Apply this decision rule at every proposed repetition or internal link:

1. **Audience test.** If the fact is ordinary Rust or operating-system
   knowledge granted by the audience contract, omit it. If it is a codebase-
   specific meaning, invariant, condition, or consequence, continue.
2. **Interaction test.** Must the reader hold this fact together with the
   current code or prose to understand one causal claim? If not, do not repeat
   it; link only when the destination offers useful optional depth.
3. **Independence test.** Can the current passage and the destination each be
   understood correctly in isolation? If yes, remove a restatement that adds no
   new local relation, condition, or consequence. A link is not required merely
   to preserve deleted redundancy.
4. **Local form test.** If integration is required, repeat the shortest exact
   semantic statement that makes the current claim intelligible. If the needed
   material is substantial, state its current implication locally and link to
   the authoritative treatment.
5. **Artifact test.** Never duplicate a production source fragment, exhaustive
   variant list, syntax contract, or detailed procedure to restore context.
   Keep one authoritative artifact and repeat only its current meaning.
6. **Goal test.** At every site participating in a delocalized plan, state the
   plan's goal when local clues reveal only the site's role. Enumerate all
   participating fragments in the source/fragment index.

The interaction and independence tests come from the split-attention
boundary conditions; the goal test comes from observed misinterpretation of
delocalized plans ([Ayres and Sweller](https://www.davidlewisphd.com/courses/EDD8121/readings/2006-AyersSweller.pdf), [Letovsky and Soloway 1986](https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf)). The precise six-step procedure is a project
inference. It implements the settled root-brief contract rather than reopening
it.

## Fragment-system requirements

### Source and grammar

The production crate and manifest remain the source of truth. The book is a
checked projection. The validator expands fragments in memory and compares the
result against the real files; it does not generate or overwrite production
source.

The fragment grammar must satisfy these requirements:

- Every in-scope file has exactly one root fragment associated with its
  repository-relative path.
- Every fragment definition has one globally unique, intent-expressive
  `«fragment-id»`. File roots may additionally encode their file role because
  they are structural entry points.
- A fragment contains literal source bytes and explicit child references at
  exact insertion points.
- A child reference contributes no implicit indentation or other text
  transformation. Each fragment carries the bytes needed at its source
  position, so expansion is pure recursive concatenation.
- Fragment identifiers cannot be redefined or implicitly continued.
- A planned-but-not-yet-authored reference is syntactically distinct from a
  resolved reference and names its owning book slice. Final validation permits
  no deferred reference.
- Definition order in Markdown has no expansion meaning. References alone
  determine source order.

Named recursive expansion is established by CWEB and noweb. Intent-expressive
naming is consistent with a recent controlled experiment on algorithm labels,
but that study's abstract reports a different task and a mixed-experience
population, so it supplies only low-moderate support here
([CWEB manual](https://tug.ctan.org/web/cweb/cwebman.pdf), [Ramsey 1994](https://www.cs.tufts.edu/~nr/pubs/lpsimp.pdf), [algorithm-label experiment](https://arxiv.org/abs/2504.19225)). Uniqueness, absolute indentation,
deferred-reference state, and in-memory comparison are project-specific
constraints.

### Validation

Mechanical validation must reject:

- a missing or duplicate file root;
- a duplicate fragment definition;
- an unresolved or final deferred reference;
- a reference cycle;
- a production fragment unreachable from its intended file root;
- a fragment reachable from an unintended root;
- missing, duplicated, or overlapping source coverage;
- any byte difference between an expanded root and its source file;
- an in-scope file with no fragment coverage;
- an excluded file presented as covered production source; and
- a fragment relationship that the source/fragment index does not expose.

A byte mismatch diagnostic identifies the file, first differing byte offset,
and owning fragment. Graph diagnostics print the reference path that establishes
the cycle, wrong-root reachability, or unresolved insertion.

The fragment ledger records, at minimum, each identifier, defining page,
owning source file and range, parent insertion sites, child references, and
book-slice owner. It also records each cross-cutting invariant's participating
fragments. Definition-to-insertion distance and fragment fan-out are review
signals, not pass/fail thresholds: they reveal hidden dependencies and
delocalization but have no evidence-backed universal limit. The Cognitive
Dimensions framework supports inspecting visibility, hidden dependencies, and
role-expressiveness, not a numeric threshold ([Blackwell and Green](https://www.cl.cam.ac.uk/~afb21/publications/BlackwellGreen-CDsChapter.pdf)).

### Reader explanation

The book explains the fragment notation before its first nontrivial use with
one small example showing:

1. a file root;
2. a child reference at an insertion point;
3. the child's definition elsewhere in reader order;
4. recursive expansion into source order; and
5. byte comparison with the repository file.

Every real fragment visibly states where it is inserted and what it inserts,
either beside the definition or through a raw-Markdown-readable index entry.
Fragment labels and references remain meaningful without a renderer. Removing
the validator removes automated proof, not the source, prose, code fragments,
reading order, or insertion relationships.

## Candidate generic instructions for `writing-code-walkthroughs`

### Elicitation

Ask one question at a time and record each answer before continuing:

1. What repository, package, executable, or bounded subsystem is the target?
2. Which manifests and production files are in scope, and which tests, models,
   fixtures, generated files, examples, and dependencies are evidence only?
3. Can the source corpus change while the walkthrough is authored, and which
   artifact remains authoritative if it does?
4. In which language, systems, tooling, and domain axes is the reader already
   proficient, and which domain is new?
5. What depth is required: orientation, subsystem, complete production source,
   or another explicit level?
6. What output form is required, and what must remain usable if custom tooling
   disappears?
7. Which prose, terminology, citation, and navigation constraints govern the
   artifact?
8. What must verification prove mechanically, and which claims require
   technical or editorial review?

Do not ask the user to prescribe a chapter list before inspecting the system.
Derive reader order from purpose, vocabulary, public seams, complete operations,
invariants, boundary effects, and concept dependencies.

### Authoring

The prose rules below adapt maintained developer-documentation guidance and the
RFC style requirement for concise, comprehensive introductions to technically
knowledgeable readers; they are editorial instructions, not measured reader-
outcome claims ([RFC 7322](https://www.rfc-editor.org/rfc/rfc7322.html), [Google style highlights](https://developers.google.com/style/highlights), [Google headings](https://developers.google.com/style/headings)).

1. Freeze and record the source inventory and evidence-only exclusions.
2. Extract the codebase vocabulary, audience-specific unknowns, public seam,
   invariants, effects, refusals, and representative end-to-end operations.
3. Build a concept dependency graph and choose a real operation that can serve
   as the low-resolution whole.
4. Select a whole-to-path-to-parts reading order; do not use the directory tree
   as the syllabus.
5. Define one root per source file and partition production bytes into uniquely
   identified fragments. Validate the graph before prose depends on it.
6. Write the purpose, minimum vocabulary, public boundary, and complete
   low-resolution operation.
7. Expand later chapters along causal paths, placing each fragment definition
   where it best supports comprehension while preserving source reconstruction.
8. State goals at every site of a delocalized plan and expose insertion
   relationships for every fragment.
9. Apply the repetition decision rule at every dependency boundary.
10. Edit for explicit actors, stable terms, direct claims, descriptive headings,
    audience-calibrated detail, and distinct refusal/failure/defect categories.
11. Build the concept and source/fragment indexes after content stabilizes.

### Verification

Run checks in an order that isolates contracts:

1. source inventory and exclusions;
2. fragment identity, references, cycles, and reachability;
3. coverage and byte-for-byte equality;
4. Markdown headings, anchors, links, page reachability, and navigation;
5. the target codebase's existing format, build, lint, and test commands;
6. technical review of contracts, invariants, control/data flow, effects,
   concurrency, rollback, and error/refusal distinctions;
7. editorial review of dependency order, local completeness, unexplained
   examples, terminology, ambiguity, fragmentation, and bloat; and
8. a walk-away review of raw Markdown and ordinary source with custom tooling
   absent.

## Observable behaviours for baseline and skill-enabled scenarios

| Scenario | Observable successful behaviour |
|---|---|
| Scope elicitation | The author asks separately for target, included production source, evidence-only material, corpus stability, and verification; it does not infer scope from a directory listing alone. |
| Audience elicitation | The author records expertise by axis, distinguishing language/OS proficiency from codebase-domain unfamiliarity. |
| Ordering | The outline begins with purpose and a real low-resolution complete operation, then expands causal layers; it is not a sequence of source files or modules. |
| Worked execution | The example names production inputs, outputs, stage boundaries, governing invariants, and observable results; it explains why each transition exists. |
| Guidance calibration | Ordinary language mechanics are omitted unless codebase usage is surprising; domain algebra and non-obvious effects receive the detail. |
| Repetition | Required codebase-specific context is restated locally only when the current claim depends on mental integration; independently intelligible duplication is removed. |
| Linking | Links have descriptive purpose text, carry optional depth or lookup, and never form a required chain for one current claim. |
| Fragment design | Each file has one root, identifiers are unique and intent-expressive, insertions are explicit, and reader order does not control expansion. |
| Fragment visibility | A raw-Markdown reader can determine where a fragment is inserted and what it inserts without a generated site. |
| Exactness | Mechanical output proves complete reachability, exact coverage, and byte equality against the authoritative source; copied snippets or compilation alone are not accepted as proof. |
| Failure handling | A source change produces a localized equality failure; it does not silently regenerate or overwrite production files. |
| Technical prose | Pages lead with claims, use stable vocabulary and explicit actors, distinguish refusals from environmental failures and defects, and avoid rhetorical staging. |
| Review | Technical review checks claims against authoritative codebase evidence; editorial review checks both missing local context and redundant repetition. |
| Walk-away property | With walkthrough-specific tooling removed, ordinary source and the full Markdown exposition remain independently readable; only automated equality proof is lost. |

These behaviours are suitable for paired baseline and skill-enabled evaluation
because an evaluator can inspect the questions asked, outline produced,
fragment graph, validation output, prose, and tool-removal result. They do not
require the evaluator to accept a subjective claim that the book is engaging or
that literate programming is inherently superior.

## Corpus gaps and residual uncertainty

- Neither survey found a controlled comparison of chapter orders in a complete
  modern codebase walkthrough. The proposed sequence combines evidence from
  other instructional and comprehension tasks. **Confidence: moderate.**
- Neither found a primary study of local repetition versus internal links in a
  long expert code book. The rule transfers split-attention boundaries and
  program-documentation findings. **Confidence: moderate.**
- The surveys share Ramsey's noweb paper and mdBook documentation. Agreement
  about named recursive expansion and includes is therefore not independent
  corroboration. **Confidence: high** about documented mechanics, not outcomes.
- No controlled reader-outcome evidence for literate programming was found.
  The project may claim verifiable coverage and flexible ordering, not improved
  comprehension caused by literate form. **Confidence: high** about the search
  result within these two surveys, not proof that no study exists.
- Survey B could not obtain the full Ramsey and Marceau team-project paper, the
  full Aghajani taxonomy, or the Entangled reference publication. Claims derived
  from those sources remain excluded or qualified. **Confidence: low.**
- No evidence-backed universal fragment size, fan-out, definition distance, or
  prose-to-code ratio was found. Use measurements as review prompts, never hard
  quality thresholds. **Confidence: high.**
- The projection-and-compare system appears to be a project-specific synthesis;
  neither survey found a primary source describing the same contract against a
  frozen corpus. Its correctness must come from explicit invariants and tests,
  not analogy to prior tools. **Confidence: moderate.**
- Progressive disclosure evidence comes from other media and instructional
  sequences. The two-resolution scheme is a conservative project design, not a
  demonstrated optimum for Markdown books. **Confidence: low-moderate.**
- Naur's theory-building argument places a limit on any documentation artifact:
  a walkthrough can state justifications but cannot promise to transfer every
  tacit part of a maintainer's theory ([Naur 1985](https://gwern.net/doc/cs/algorithm/1985-naur.pdf)). The appropriate completeness claim is local and
  checkable: every fact required to understand each claim appears in the book.

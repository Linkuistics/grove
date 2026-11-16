<!--
  Imported survey. Everything below the horizontal rule is the APIAnyware
  survey as committed there, with only the seven link targets listed in the
  provenance table changed. Do not edit the survey text here; correct or
  refine it in docs/research/documentation-evidence.md, which cites it.
-->

> **Imported research — provenance**
>
> - **Repository:** [Linkuistics/APIAnyware](https://github.com/Linkuistics/APIAnyware)
> - **Path:** `semantic/docs/research/documentation-basis-a.md`
> - **Revision:** `2a874cf35bcb6da5bf0f1846cf3bb8adcc6f5f67` (the
>   `improve-api-interface-language-docs` bookmark and `main`, both pushed
>   to `origin`). Recover it with
>   `jj file show -r 2a874cf35bcb6da5bf0f1846cf3bb8adcc6f5f67 semantic/docs/research/documentation-basis-a.md`
>   in that repository. The file's text there is identical to its state at
>   `8103dd6b10eb7c243abd5b4344212936690b3001` (2026-09-23 16:38 +10:00),
>   which added §11 to the survey first committed at
>   `1c4b8e7ec3da7151f12ff648fad538454143e272` (2026-09-23 15:53 +10:00).
> - **Research date:** 2026-09-23, as the survey states. **Imported:**
>   2026-09-30, by Grove leaf `documentation-evidence-k5` for
>   [#14](https://github.com/Linkuistics/grove/issues/14).
> - **SHA-256 of the file at that revision:**
>   `e4db4b6cc32c98d77c2c90212dcd8a4c87f990ff5f1f7a0a4a0e5583a8c5cced`.
>
> **What changed in this copy.** Only the seven link targets below. The survey
> used paths relative to its APIAnyware location, which do not resolve in this
> repository. Each now points to the same file on GitHub at `8103dd6b…`, the
> first revision at which the survey reached its final text and every target
> exists. The link text and all prose are unchanged, so every inspection-depth
> statement, evidence qualifier and the C4 licence note survive as written.
>
> | Where | Original target | Now |
> |---|---|---|
> | `[ail]` definition | `../../specification/interface-language/README.md` | APIAnyware `semantic/specification/interface-language/README.md` at `8103dd6b…` |
> | `[ail-language]` definition | `../../specification/interface-language/language.md` | same repository and revision, `…/language.md` |
> | `[ail-contracts]` definition | `../../specification/interface-language/contracts.md` | same repository and revision, `…/contracts.md` |
> | `[ail-conformance]` definition | `../../specification/interface-language/conformance.md` | same repository and revision, `…/conformance.md` |
> | `[ail-design]` definition | `../design/2026-09-22-interface-language.md` | same repository and revision, `semantic/docs/design/2026-09-22-interface-language.md` |
> | §11, "documentation audit" | `interface-language-documentation-audit.md` | same repository and revision, `semantic/docs/research/interface-language-documentation-audit.md` |
> | §11, "documentation plan" | `../design/2026-09-23-interface-language-documentation-plan.md` | same repository and revision, `semantic/docs/design/2026-09-23-interface-language-documentation-plan.md` |
>
> The survey's §7 read the AIL pages "at the working copy of 2026-09-23"; those
> pages kept changing afterwards, so a pinned target shows a nearby state rather
> than necessarily the exact text read.
>
> **Limits to carry with it.** The survey had one author and delegated
> readers; it states its own inspection depth per source in §2–§3 and its gaps
> in §10. It paraphrases ISO 24495-1 and quotes none of it; later guidance must
> keep citing, not quoting, that standard. The C4 site and example diagrams are
> CC BY 4.0, so adapting them requires attribution. Refinements from later
> reader evidence, and the inventory of proposals drawn from this survey and
> from issues #14–#19, are in
> [`documentation-evidence.md`](documentation-evidence.md); this copy presents
> no guidance as adopted by Grove.

---

# Documentation basis for reviewing the API Interface Language: survey A

Research date: 2026-09-23. This survey gathers the evidence for an editorial
approach to documentation that lets language designers who are new to API
binding evaluate the proposed [API Interface Language][ail] (AIL), draft 0.1.
It feeds the documentation audit and information architecture that follow. It
does not rewrite AIL documents and proposes no language changes. It is a single
breadth-seeking survey; no second, independent survey was commissioned.

## Summary

- **The reader is expert and novice at once.** Reviewers are experts in
  language design and novices in binding contracts. Techniques that help
  novices can hinder experts, so the set should not teach language design, and
  should teach the binding domain concretely, early and with worked examples
  ([§4.1](#41-reviewers-new-to-the-domain-judging-a-proposal)).
- **Borrow the proposal genre.** Language communities put proposals before
  expert reviewers in one recurring shape: the problem before the design, an
  example-led explanation beside the precise one, and explicit drawbacks,
  alternatives and open questions. Product-documentation guidance does not
  cover this shape ([§4.1](#41-reviewers-new-to-the-domain-judging-a-proposal)).
- **Separate purposes, not contexts.** Diátaxis separates tutorial, how-to,
  reference and explanation by the reader's need, but calls itself a guide
  rather than a plan. Field studies found readers could not predict which
  information-type section held what they needed. Keep each page pure in
  purpose, organise navigation by the reviewer's journey and the language's
  structure, and let every page stand as page one
  ([§4.2](#42-how-the-four-forms-cooperate)).
- **Examples carry the review.** Use at least two examples per concept, small
  multi-part patterns, labelled status, expected observations a reader can
  check by inspection, failure pairs drawn from the conformance mutations and a
  modification task. No validator exists, so no example may show invented tool
  output ([§4.3](#43-examples)).
- **Test against inventories and readers, not scores.** Check accuracy and
  completeness against an enumerated inventory of productions, clauses and
  rules. Check outcomes with a few real readers. Readability formulas and voice
  counts are not gates, and the style guides the editing prompt cites call
  their own rules defaults with exceptions
  ([§4.4](#44-tests-of-reader-outcomes-and-document-quality),
  [§6](#6-the-editing-prompt-against-its-sources)).
- **Fixed-field reference entries.** Every clause, declaration kind and type
  constructor needs an entry with the same fields, stable rule identifiers, and
  "used by" and "see also" links against hidden dependencies
  ([§4.5](#45-navigation-reference-entries-glossary-tables-and-diagrams)).
- **Figures by level, under one vocabulary.** Start from one overview
  diagram and zoom level by level. C4 fits AIL's ecosystem and toolchain, but
  by its own account not a library's contracts, which need sequence, state and
  timing diagrams. Every added notation costs readers effort, so PlantUML, D2
  and Mermaid render under one shared visual vocabulary with a key on every
  diagram, and prose carries every fact
  ([§4.5](#45-navigation-reference-entries-glossary-tables-and-diagrams)).
- **Format.** Reviewers read this set in the repository, where GitHub renders
  Markdown in full but AsciiDoc only as a preview (includes become links,
  diagram blocks show their source) and DocBook or Typst not at all.
  Single-sourced examples therefore need a small committed generation step in
  either Markdown or AsciiDoc. The evidence favours Markdown for this set, and
  AsciiDoc if a published edition is wanted; the choice is the human's, at
  planning ([§8](#8-formats-diagram-notations-and-the-walk-away-check)).
- **Stage-scoped criteria.** [§9](#9-stage-scoped-acceptance-criteria)
  partitions the resulting checks by the editorial stage that applies them, so
  each stage loads only its own slice.

## 1. Reader, purpose and questions

The primary readers know language design, language aesthetics and what
developers expect, but not API binding: FFI, ABI, ownership transfer, borrow
validity, quiescence, projections or profiles. They read to judge the language —
syntax and readability, conceptual consistency, expressive scope, authoring
burden, discoverability, and fitness for AIL's producers and consumers — and to
give feedback. API authors and tool implementers are secondary readers who still
need a precise reference. No parser, validator or native implementation of AIL
exists ([README][ail]).

The questions this survey answers:

1. Which guidance helps a language-design reviewer understand an unfamiliar
   domain and judge a language proposal, and how does that differ from
   documenting an implemented SDK for experienced users?
2. How should tutorial, review guide, reference and explanation cooperate, and
   where would rigid separation remove necessary context?
3. What makes examples useful for evaluating syntax, semantic consistency,
   expressiveness and authoring effort?
4. Which concrete tests of reader outcomes and document quality should
   planning, drafting, copy-editing, figures and proof use?
5. Which navigation, reference structure, glossary, tables and diagrams reduce
   review effort without removing technical detail, and how does a reader who
   lands on any page recover its context?

## 2. Method

- **Primary sources first:** publisher and author pages, official guidance,
  and papers from author-hosted or open repositories. No unofficial copies of
  books were opened.
- **Depth is stated for every source** in [§3](#3-inventory): full text,
  sections, excerpt (publisher sample or snippet search), contents (table of
  contents, outline or description), abstract, or not inspected. A book is
  never described as read when only its contents or an excerpt was seen.
- **Who read what.** The survey's author read the Rabil article, every current
  Diátaxis page and one removed page (from the project's history), Baker's
  outlines, Redish (2000), CSS 2.1 §1.4.2, man-pages(7), GitHub's documentation
  on diagrams and anchors, and Mermaid's C4 and accessibility pages. Research
  agents read the other clusters under the same rules and returned URLs, page
  types and verbatim quotes.
- **Full text through the human's own access.** Through the human's O'Reilly
  Learning subscription, the survey's author read selected sections of DQTI
  chapters 7, 8, 9 and 13, *Docs for Developers* chapters 5, 6 and 9, and
  Clark and Mayer's *e-Learning and the Science of Instruction* (5th ed.)
  chapters 6, 9 and 13 — the sections cited, not whole chapters. The human's
  Zotero library supplied PDFs of Clements et al.,
  *Documenting Software Architectures* (2nd ed.), and Rozanski and Woods,
  *Software Systems Architecture* (2005), also read in the sections cited.
  Nothing from these copies is reproduced beyond short quotations.
- **Spot checks.** Load-bearing quotes from the delegated reading were checked
  against freshly fetched sources: Google's "guidelines, not rules", "Put
  conditions before instructions, not after" and its passive-voice exceptions;
  DQTI's characteristic definitions and "Quality is ultimately determined by
  users"; Robillard and DeLine's five factors; Meng et al.'s structuring
  recommendation; Kalyuga et al.'s definition of the expertise reversal
  effect; Ambrose et al.'s prior-knowledge principle; Gopen and Swan's "not
  rules" caveat; four C4 statements; Asciidoctor's remarks on GitHub; Typst's
  HTML warning; and Structurizr's end-of-life notice. All matched. Every DOI
  cited resolves in Crossref to the work named.
- **Dates and quotation.** Pages were accessed on 2026-09-23. Quotations are
  short; everything else is paraphrase.
- **The editing prompt.** The human's verbatim editing prompt is not in the
  repository. [§6](#6-the-editing-prompt-against-its-sources) checks the
  criteria and attributions that the requirements record gives for it.

## 3. Inventory

### 3.1 The supplied reading list

[Rabil (2024)][rabil] reviews a main list, adds "Supplemental
Recommendations", lists "Tech Comm Books I Want to Read" (books its author has
not read) and "Non-Tech Comm Books All Tech Writers Should Read". Its review of
*Document Design* names three adjacent books, and a reader comment suggests two
more. The article excludes generic writing books such as Strunk and White. It
never mentions Diátaxis or Google's style guide.

Relevance marks what a source contributes here: **H** shapes a recommendation;
**M** supplies a specific transferable principle; **L** is a pointer at most;
**NA** has nothing to cite for this purpose.

| Title (edition used here) | Place in the article | Inspected for this survey | Rel. | Used for |
|---|---|---|---|---|
| [*Developing Quality Technical Information*][dqti-sample], 3rd ed., IBM Press 2014 | Reviewed | Publisher sample (full contents, chapter 1, index); cited sections of chapters 7, 8, 9 and 13 ([O'Reilly][dqti-oreilly]); snippet search for other pages | H | Quality model, examples, voice, organization, review |
| [*Every Page Is Page One*][eppo-pub], Baker, XML Press 2013 | Reviewed | Author's [expanded outline][eppo-outline] and publisher outline | H | Page-one context, qualified reader, linking |
| [*Docs for Developers*][dfd], Apress 2021 (2nd ed. [2026][dfd2]) | Want to read (unread by its author) | Overview, abstracts, free front and back matter; cited sections of chapters 5, 6 and 9 ([O'Reilly][dfd-oreilly]) | H | Code samples, visual content, quality types |
| [*Technical Editing*][rude-eaton], Rude and Eaton, 5th ed., Pearson 2011 | Reviewed | Publisher overview and contents; section-level contents from a retail page | M | Levels of edit, comprehensive editing |
| [*Technical Communication*][markel13], Markel and Selber, 13th ed., Bedford 2021 (14th ed. 2025) | Reviewed | Archived publisher overview and contents | M | Audience and purpose; testing documents (titles only) |
| [*Technical Communication*][lannon15], Lannon and Gurak, 15th ed., Pearson 2019/2021 (16th ed. 2024) | Reviewed (a different book) | Archived publisher overview and contents | L | Definitions and instructions (titles only) |
| [*Practical Strategies for Technical Communication*][markel-ps], Markel and Selber, 5th ed., 2025 | Passing mention; its author has not read it | Publisher description and contents | L | — |
| *Microsoft Manual of Style*, 4th ed., 2012 → [Microsoft Writing Style Guide][ms-welcome] | Passing mention (print book) | The online guide that states it replaces the book, in full | M | Reference sections, code examples, terms |
| [*Document Design*][document-design], Kimball and Hawkins, 2008 (2nd ed., Kimball and Ross, 2025) | Reviewed | 2nd-edition contents and chapter 1 excerpt | L | Whole-document consistency, tables |
| [*Technical Writing 101*][tw101], Pringle and O'Keefe, 3rd ed., 2009 | Reviewed | Publisher description and chapter list (free official e-book not obtained) | L | — |
| [*Information Development*][hackos], Hackos, Wiley 2006 | Reviewed | Wiley contents and chapter 1 excerpt | L | Review types (quality-assurance chapter) |
| [*e-Learning by Design*][horton], Horton, 2nd ed., 2011 | Supplemental | Wiley detailed contents | L | Prerequisites, sequencing |
| *Central Works in Technical Communication*, OUP 2004 | Supplemental | Library contents only | NA | — |
| Katz, "The Ethic of Expediency"; Slack et al., "The Technical Communicator as Author" | Essays the supplemental entry names | Not inspected | NA | — |
| *Reporting Technical Information*, Houp et al., 11th ed., OUP 2005 | Supplemental | Retailer contents only | L | — |
| [*DITA Best Practices*][dita-bp], IBM Press 2011 | Want to read | Contents and publisher sample chapter | M | One purpose per topic; DITA mechanics fail the walk-away check |
| [*Managing Writers*][managing-writers], Hamilton, XML Press 2009 | Want to read | Publisher sample chapter | L | Caution on metrics |
| [*Docs Like Code*][docs-like-code], Gentle, 3rd ed., 2022 | Want to read | Book page and contents | M | Review, linting and versioning in git |
| *Docs-as-Ecosystem*, Quetzalli, Apress 2023 | Want to read | Author contents | L | — |
| *The Product is Docs*, Gales and Splunk, 2nd ed., 2020 | Want to read | Announcement; partial chapter list (secondary) | L | — |
| [*Don't Make Me Think, Revisited*][krug], Krug, 2014 | Adjacent; non-tech-comm list | Official contents and sample chapter | M | Scanning, link labels, small usability tests |
| [*Web Style Guide*][web-style-guide], Lynch and Horton, 4th ed., 2016 | Adjacent; non-tech-comm list | Full text free online (CC BY-NC-SA); editorial chapter sampled | M | Chunks, headings, descriptive links |
| [*The Design of Everyday Things*][norman], Norman, rev. ed. 2013 | Adjacent; non-tech-comm list | Author contents | M | Conceptual models as a lens |
| *Information Architecture*, Rosenfeld, Morville and Arango, 4th ed., O'Reilly 2015 | Non-tech-comm list (links the 3rd ed.) | Contents via Google Books | M | Labelling, navigation, controlled vocabularies |
| *The Global English Style Guide*, Kohl, SAS 2008 | Reader comment | Library contents; third-party review | L | Sentence-level ambiguity rules |
| *Writing from the Core*, Young, 2009/2010 | Reader comment | Bibliographic record only | NA | — |

### 3.2 Sources named by the editing prompt

| Source | In the article? | Inspected |
|---|---|---|
| [Diátaxis][dx] | No | Every current page; one [removed page][dx-hier] from the project's history |
| [*Docs for Developers*][dfd] | Listed among books its author has not read | Overview, abstracts, front and back matter; sections of chapters 5, 6 and 9 |
| [DQTI][dqti-sample] | Reviewed | Publisher sample; sections of chapters 7, 8, 9 and 13; snippets |
| [Google developer documentation style guide][g-style] and [Technical Writing courses][tw1-audience] | No | Official pages, in full |
| [Microsoft Writing Style Guide][ms-welcome] | Only its 2012 print predecessor | Official pages, in full |

### 3.3 Sources this survey added

The article serves technical communicators who document products; this reader
evaluates a notation. The survey therefore added:

- **Proposal genres for expert reviewers:** the [Rust RFC template][rust-rfc],
  [PEP 1][pep-1], the [Swift Evolution template][swift-template] and
  [process][swift-process], the W3C TAG's
  [Writing Effective Explainers][w3c-explainer], the
  [TC39 process][tc39-process], and the [Go 2 language-change template][go2].
- **How languages split their documentation:** the
  [Python reference][py-ref-index] and [developer guide][py-devguide], the
  [Rust Reference][rust-ref], the [Go specification][go-spec], the
  [C99 Rationale][c99-rationale], the [Annotated Ada Reference Manual][aarm],
  and [RFC 2119][rfc2119] with [RFC 8174][rfc8174].
- **Evaluating notations:** cognitive dimensions
  ([Green and Petre 1996][cd-1996]; [tutorial][cd-tutorial];
  [Blackwell and Green 2003][cd-2003]; [questionnaire][cd-q];
  [pilot][cd-2000]), [Clarke 2004][clarke-2004] on API usability, and
  [PLIERS][pliers] (Coblenz et al. 2021).
- **API documentation research:** [Robillard 2009][robillard-2009],
  [Robillard and DeLine 2011][rd-2011], [Uddin and Robillard 2015][ur-2015],
  Meng et al. [2018][meng-2018] (abstract only), [2019][meng-2019] and
  [2020][meng-2020], and [Nassif and Robillard 2023][nr-2023].
- **Instruction and learning:** minimalism
  ([Carroll and van der Meij 1996][minimalism-1996]; heuristics through
  [Virtaluoto et al. 2021][minimalism-2021]), worked examples
  ([Atkinson et al. 2000][atkinson-2000]), the expertise reversal effect
  ([Kalyuga et al. 2003][kalyuga-2003]), progressive disclosure
  ([Nielsen 2006][nielsen-2006]), and the pre-training, segmenting,
  coherence and signaling principles
  ([Clark and Mayer 2023][clark-mayer], chapters 6, 9 and 13).
- **Learning and reader psychology:** Mayer's reviews
  ([Mayer and Moreno 2003][mayer-moreno-2003]; [Mayer 2008][mayer-2008];
  [Mayer 2014][mayer-2014]) and the pre-training study
  ([Mayer, Mathias and Wetzell 2002][mayer-2002], abstract only);
  prior knowledge ([Ambrose et al. 2010][ambrose-2010], chapter 1;
  [Kaplan et al. 2009][kaplan-2009]); analogies and comparison
  ([Spiro et al. 1988][spiro-1988]; [Gentner et al. 2003][gentner-2003];
  [Alfieri et al. 2013][alfieri-2013]); the curse of knowledge
  ([Camerer et al. 1989][camerer-1989]; [Hinds 1999][hinds-1999];
  [Pinker 2014][pinker-2014]); reader-protocol training
  ([Schriver 1992][schriver-1992], abstract only); and scanning and
  information scent ([Pirolli and Card 1999][pirolli-1999];
  [Budiu 2020][budiu-2020]; [Pernice 2017][pernice-2017]).
- **Prose and plain language:** [Gopen and Swan 1990][gopen-swan];
  Williams's *Style* ([excerpt][williams-1992]); the
  [Federal Plain Language Guidelines][fed-pl] (2011); the
  [International Plain Language Federation][iplf] definition, which ISO
  24495-1:2023 adopts; and [Jarrett and Redish 2020][jarrett-redish] on
  testing documents.
- **Architecture documentation:** Clements et al.,
  [*Documenting Software Architectures*][dsa] (2nd ed.), and Rozanski and
  Woods, [*Software Systems Architecture*][rozanski-woods] (2005), for rules
  of sound documentation, refinement from overview to detail, context diagrams
  and views chosen by stakeholder concern.
- **Readability and review:** [Redish 2000][redish-2000] on readability
  formulas.
- **Reference-entry precedents:** [CSS 2.1 property definitions][css21] and
  [man-pages(7)][man-pages].
- **Figures, formats and tooling:** GitHub's documentation on
  [diagrams][gh-diagrams] and [anchors][gh-basic], Mermaid's
  [C4][mermaid-c4] and [accessibility][mermaid-a11y] pages, and
  [Moir 2024][moir-2024] on docs-as-code.
- **Document formats:** the [GFM specification][gfm-spec] and GitHub's
  rendering behaviour ([markup][gh-markup]); the
  [AsciiDoc language documentation][asciidoc-docs] and Asciidoctor's
  [safe modes][asciidoctor-safe]; [DocBook: The Definitive Guide][docbook-tdg];
  and the [Typst documentation][typst-docs] and [package index][typst-packages].
- **Diagrams and hierarchy:** the [C4 model][c4-diagrams] site with its
  notation rules, checklist and FAQ; C4 tooling (Structurizr, C4-PlantUML,
  Mermaid C4, D2, LikeC4; see
  [§8](#8-formats-diagram-notations-and-the-walk-away-check)); Moody's
  principles for visual notations ([2009][moody-2009], through
  [Moody et al. 2010][moody-2010]); [Larkin and Simon 1987][larkin-simon];
  [Ainsworth 2006][ainsworth-2006]; [Petre 1995][petre-1995]; and
  [Shneiderman 1996][shneiderman-1996].

## 4. Findings

### 4.1 Reviewers new to the domain, judging a proposal

**Evidence**

- **Expertise belongs to a domain.** Techniques that work for inexperienced
  learners "can lose their effectiveness and even have negative consequences"
  for experienced ones ([Kalyuga et al. 2003][kalyuga-2003]). Worked examples
  matter most early in acquiring a skill ([Atkinson et al. 2000][atkinson-2000]).
- **Audience is distance from the knowledge, not role.** Google's course
  defines good documentation as the knowledge a task needs minus what the
  audience already has, and says roles alone are insufficient
  ([Audience][tw1-audience]). For a design document, list what readers should
  learn. Its worked example addresses readers deciding whether an algorithm is
  "worth implementing", and its outline includes edge cases and "known
  unknowns" ([Documents][tw1-docs]).
- **Qualified, but not in everything.** Baker: address a qualified reader, and
  give unqualified readers enough context to "recognize that they are
  unqualified", with links to what would qualify them ([outline][eppo-outline],
  chapter 11).
- **Prior knowledge helps and interferes.** One developer said that mapping
  known technologies onto a new API "interfered with my learning"
  ([Robillard and DeLine 2011][rd-2011]). Minimalism's task is "helping the
  learners to identify what prior knowledge is relevant, and how"
  ([Carroll and van der Meij 1996][minimalism-1996]). DQTI warns that familiar
  terms may have unexpected definitions (p. 210, [snippet search][dqti-gbooks]),
  and Microsoft tells writers to assume readers know a word's common meaning
  ([Microsoft][ms-common]). The learning-science synthesis is that "Students'
  prior knowledge can help or hinder learning": inaccurate or inappropriate
  prior knowledge "may actively distort or impede" new learning, notably when
  everyday meanings intrude on technical terms. Its remedies are to state the
  conditions under which an idea applies, "point out differences, as well as
  similarities, when employing analogies", show where an analogy breaks down,
  and activate the right prior knowledge ([Ambrose et al. 2010][ambrose-2010],
  chapter 1; [Kaplan et al. 2009][kaplan-2009] on statistics terms). In an
  agency case, every reader called a letter clear while several acted on a
  different meaning of its term of art ([Federal guidelines][fed-pl],
  pp. 110–111). No study of false friends *between programming languages*
  was found; the risk for `borrow` or `owned` is an inference from these
  cases.
- **Experts cannot debias themselves.** Better-informed people are "unable to
  ignore private information" when predicting others' judgments, and
  incentives and feedback did not remove the bias
  ([Camerer et al. 1989][camerer-1989]). Experts were worse than
  intermediates at predicting novice difficulty, and debiasing instructions
  made them *less* accurate ([Hinds 1999][hinds-1999]). What did help writers
  in a controlled classroom study was reading think-aloud transcripts from real
  readers ([Schriver 1992][schriver-1992], abstract). Pinker's remedy is
  practical: show a draft to people like the intended readers
  ([Pinker 2014][pinker-2014]).
- **Experts misjudge what novices need.** "Subject matter experts often
  overload the instruction with too much content that at their knowledge level
  most likely does not seem overwhelming" ([Clark and Mayer 2023][clark-mayer],
  chapter 13). "Reviewers tend to assume that because they understand
  something, a user will also" ([DQTI][dqti-oreilly], chapter 13). Leaving
  something out because "everybody knows that" makes an architectural
  description incomplete ([Rozanski and Woods 2005][rozanski-woods],
  chapter 13).
- **Teach names before procedures.** By the pre-training principle, "people
  learn better when they are introduced to the names and characteristics of
  the key elements before the main lesson is presented"; it matters when the
  essential material would otherwise overwhelm the learner
  ([Clark and Mayer 2023][clark-mayer], chapter 13).
- **Know the readers; drop insider jargon.** Clements et al.'s first rule of
  sound documentation is to write from the reader's point of view: find out
  what readers know and expect, avoid unnecessary insider jargon, and add a
  glossary ([*Documenting Software Architectures*][dsa], prologue).
- **Ownership is hard even for programmers.** In the PLIERS studies an
  ownership task proved "very difficult"; the authors conclude that language
  decisions with structural implications, such as ownership, "require
  substantial high-level training" ([Coblenz et al. 2021][pliers]).
- **Intent must be reachable from the entry page.** "If you don't know what
  the intent is, you fight the system"; design decisions that affect use
  should be "traceable from the point-of-entry documentation pages"
  ([Robillard 2009][robillard-2009]). Developers struggle to relate an API
  "upwards towards its problem domain and downwards towards its
  implementation" ([Robillard and DeLine 2011][rd-2011]).
- **Proposal genres put the problem first and the costs in view.**
  - Rust RFCs: motivation; a guide-level explanation that teaches the feature
    "as if it was already included in the language"; a reference-level
    explanation that returns to the same examples; drawbacks ("Why should we
    not do this?"); rationale and alternatives; prior art; unresolved
    questions ([template][rust-rfc]). The guide-level section was added to make
    RFCs "more accessible to people who are Rust users but not compiler or
    language design experts" ([2017 change][rust-rfc-2017]).
  - PEPs: motivation, specification, rationale with alternatives, rejected
    ideas and open issues, with "How to Teach This" recommended
    ([PEP 1][pep-1]).
  - Swift: an ABI section for constraints on the design; alternatives, because
    "Reviewers are often familiar with other approaches"; and, if the proposal
    is not implemented, "just say that" ([template][swift-template]).
  - W3C TAG: "A reader who doesn't already know the details should be able to
    distinguish the context from the proposed changes"
    ([explainers][w3c-explainer]).
  - Go: describe the change precisely and "informally, as in a class teaching
    Go"; "Every language change has a cost" ([Go 2 template][go2]).
- **A precedent in AIL's own domain.** Swift's C++ interoperability vision
  explains memory management and lifetimes before any design, calls itself "a
  sketch, rather than a final design", and ends with examples and definitions
  ([vision][swift-cxx]).
- **Notation reviewers need words and an object.** Cognitive dimensions give
  non-specialists names for judgements they "may only have half-formulated"
  ([tutorial][cd-tutorial]). The questionnaire asks respondents to describe
  their activities before any dimension question, and its pilot found people
  confusing a notation with its editor ([pilot][cd-2000]).
- **The SDK contrast.** Microsoft assumes developers understand programming
  and tells writers to "skip the basic knowledge" ([developer
  content][ms-dev]). Product documentation serves people doing work with a
  shipped product; a proposal is read to decide. Swift separates the two
  explicitly: user documentation does not need evolution review
  ([process][swift-process]).

**Application to AIL**

- The orientation page states what is under evaluation (the notation and its
  semantics; no tooling exists), who produces and consumes AIL packages, the
  draft status, and the kind of feedback wanted.
- A domain explanation comes before the tutorial. It takes one small signature,
  shows what the signature leaves unstated (who frees the result, how long a
  borrowed pointer stays valid, which thread may call back), and names the
  failure each obligation prevents.
- Each explanation separates what the API requires, whatever the notation,
  from how AIL chooses to say it. The design proposal's rejected alternatives
  (extending WIT, extending Swift syntax) and its open question about ownership
  in types or in clauses belong in view.
- A "familiar term, different meaning" device covers AIL words that collide
  with Rust, Swift or C++ usage, such as `borrow`, `owned`, `consume`, `shared`
  and `profile`.
- Language-design basics (EBNF, type systems, closed vocabularies) are assumed,
  not taught.

### 4.2 How the four forms cooperate

**Evidence**

- **Diátaxis's split.** Four needs — learning, goals, information and
  understanding — map to tutorial, how-to guide, reference and explanation
  ([map][dx-map], [compass][dx-compass]).
  - Tutorials "ruthlessly minimise explanation" but link to it
    ([tutorials][dx-tut]).
  - How-to guides include "thinking and judgement" and use conditional
    imperatives ([how-to guides][dx-how]).
  - Reference is "austere" and "wholly authoritative", mirrors the structure of
    the thing described, may describe "how something works", and includes
    examples ([reference][dx-ref]).
  - Explanation covers "design decisions, historical reasons, technical
    constraints" and "must consider alternatives" ([explanation][dx-exp]).
- **Diátaxis on its own limits.** It is a guide, "not a plan"; creating empty
  four-part structures is "horrible"; structure should emerge from improvement
  ([workflow][dx-work]). A user "may enter the documentation anywhere"
  ([map][dx-map]). Diátaxis "cannot address functional quality"
  ([quality][dx-quality]). A page removed from the site in August 2026 said it
  "is not a scheme into which documentation must be placed - four boxes", and
  that a landing page "should read like an overview"
  ([removed page][dx-hier]). No peer-reviewed evaluation of Diátaxis was found
  ([§10](#10-access-gaps-silences-and-acquisition-priorities)).
- **DQTI holds both sides.** Separating concept, task and reference topics
  helps readers find them, and each topic must "make sense if read by itself".
  Yet "users do not read most information linearly", so they need a map at the
  start of a journey and along it, and "Providing the topics and a navigation
  alone is often not enough" ([DQTI][dqti-oreilly], chapter 9). A set "must
  tell a cohesive story" ([sample][dqti-sample]).
- **Signals and refinement.** Summaries and headers that preview structure
  help learners, with a modest average effect across 103 experiments
  ([Clark and Mayer 2023][clark-mayer], chapter 6). Architecture
  documentation discloses detail by refinement, "the process of gradually
  disclosing information across a series of descriptions", and less detail is
  "excellent for introductions, overviews" ([Clements et al.][dsa],
  chapter 6).
- **Readers could not predict the right section.** Observed developers "had
  no specific hypothesis" whether information was under Concepts, API
  reference or Samples. The authors recommend structuring by the API's
  function or content domain and integrating conceptual information with tasks
  ([Meng et al. 2019][meng-2019]). Conceptual obstacles were rated more severe
  than reference obstacles, and some developers want "a coherent, linear
  presentation" rather than fragmented hyperlinked articles
  ([Robillard and DeLine 2011][rd-2011]). Fragmentation and tangled
  information are both documented failures
  ([Uddin and Robillard 2015][ur-2015]).
- **Google mixes where it helps.** Alternating concepts and steps is "a
  particularly engaging way to learn"; introduce terms near the instructions
  that use them; give every heading a brief introduction; explain why before
  asking for a task ([organizing large documents][tw2-large]). Prefer help in
  context to a link ([cross-references][g-xref]).
- **Every page is page one.** Baker's topics are self-contained, establish
  their context, stay on one level and link richly; the big picture lives in
  an explicit big-picture topic with pathfinder topics
  ([outline][eppo-outline]).
- **Language communities split, then reconnect.**
  - Python's tutorial is for programmers new to Python, and "many of whom will
    be using the tutorial to evaluate the language as a whole"; its reference
    is "terse, but attempts to be exact and complete"
    ([developer guide][py-devguide]; [reference][py-ref-index]).
  - Rust Reference chapters "can be read standalone, but will cross-link"
    ([Rust Reference][rust-ref]).
  - The C99 Rationale mirrors the Standard's numbering but "is not part of the
    Standard" ([C99 Rationale][c99-rationale]).
  - The Annotated Ada Reference Manual interleaves non-normative annotations
    with the rules they explain ([AARM][aarm]).
- **Readers scan, and follow scent.** "Assume that most users will be
  scanning"; if readers see only the first two words of a heading, "they
  should still get the gist" ([Pernice 2017][pernice-2017]). Link labels in
  words the audience does not yet understand carry little information scent,
  and a novice can face two layers of it: not knowing which label leads to a
  topic, and not knowing the topic's own name ([Budiu 2020][budiu-2020];
  [Pirolli and Card 1999][pirolli-1999]). Organise material "in the order
  they'd ask" questions, and prefer question headings to vague ones such as
  "General" ([Federal guidelines][fed-pl], pp. 6, 11).
- **Progressive disclosure has limits.** More than two disclosure levels
  "typically have low usability", and staged disclosure fails when steps are
  interdependent ([Nielsen 2006][nielsen-2006]). In a field study, 42–61% of
  on-demand annotations were never opened ([Nassif and Robillard 2023][nr-2023]).

**Application to AIL**

- Use the Diátaxis compass as a purpose test for each page and section, not as
  the top-level navigation.
- Organise navigation by the reviewer's journey (orientation, domain,
  tutorial, review tasks) and the reference by the language's own structure
  (declarations, clauses, types, rules).
- Open every page with a short block: purpose, reader, linked prerequisites,
  status, and where its canonical rules live.
- Teach domain concepts before the tutorial; inside it, give a one-line reason
  and a link.
- Keep normative rules in one place. Other pages restate a rule only as a
  labelled, non-normative summary linked to its identifier. The AARM model,
  with annotations beside the rules, is an alternative the plan should weigh
  against separate explanation pages.
- Use at most two levels of disclosure, and never hide what a reviewer needs to
  judge a rule behind a link alone.

### 4.3 Examples

**Evidence**

- **Tutorial examples must work and say what happens.** A tutorial's exercise
  must be "meaningful", "successful", "logical" and "usefully complete"; keep
  "a narrative of the expected", show expected output and flag signs of going
  wrong; reliability comes from observing users ([tutorials][dx-tut]).
- **Errors belong beside the action.** Minimalism puts error information "in
  the immediate vicinity of error-prone actions", with checkpoints, and
  includes conceptual information "only when necessary"
  ([Carroll and van der Meij 1996][minimalism-1996]). The 1995 heuristics were
  read only through [Virtaluoto et al. 2021][minimalism-2021].
- **Two examples, with labelled structure.** "'at least add a second example'
  appears to be a basic rule for worked-examples instructional design"; vary
  surface features to reveal
  deep structure, label subgoals, integrate text with diagrams, and use
  incomplete examples to prompt self-explanation
  ([Atkinson et al. 2000][atkinson-2000]).
- **Small patterns, marked authority.** Small examples showing a usage pattern
  beat both single calls and large tangled programs, and developers read
  examples as recommendations, so their authority should be marked
  ([Robillard and DeLine 2011][rd-2011]). State the range of usage an
  example set covers ([Robillard 2009][robillard-2009]). Unexplained examples
  reached blocker severity ([Uddin and Robillard 2015][ur-2015]).
- **Complete, tested, with expected results.** Google: test samples, because
  snippets degrade; describe expected output when code is hard to run; show
  "what not to do"; demonstrate a range of complexity; choose descriptive names
  ([sample code][tw2-code]). Microsoft: start simple, show expected output,
  and "Always compile and test" ([code examples][ms-code]). DQTI: show the
  results of users' actions, set the context for each example, and treat
  reference samples as essential ([snippets][dqti-gbooks]). *Docs for
  Developers* asks for samples that are explained, concise, clear, usable and
  trustworthy, across a range of complexity (headings, [front
  matter][dfd-front]).
- **Examples of a language are written in the language, and explained.**
  "When you describe the C++ language, use examples of C++ code"; prioritise
  "the most difficult, frequent, important, and new information"; put
  instruction in comment lines; and "Be explicit about what's important in an
  example" ([DQTI][dqti-oreilly], chapter 7). An analogy must "make the basis
  of the comparison explicit", and metaphors, which leave it implied, are to be
  avoided (same chapter).
- **Explanatory code need not run, but must be explained.** *Docs for
  Developers* separates executable samples from explanatory ones, which are
  not "expected to be runnable". "Explanations that accompany your samples are
  as important as the samples themselves"; they should say why the code does
  what it does, state limitations, and "Clearly mark any alpha or beta
  features" ([chapter 5][dfd-oreilly]).
- **Leave out what does not serve the goal.** Interesting but irrelevant
  material harms learning, so avoid adding anything that does not support the
  instructional goal ([Clark and Mayer 2023][clark-mayer], chapter 9).
- **Placeholders block reuse.** They "made it impossible to simply reuse the
  code via copy and paste" ([Meng et al. 2019][meng-2019]).
- **Language communities show failures.** Rust RFCs explain "largely in terms
  of examples" with sample error messages ([template][rust-rfc]); the Rust
  Reference tests its examples and puts each failure case in its own block
  ([Rust Reference][rust-ref]). Go asks for code "before and after the change"
  ([Go 2 template][go2]).
- **Examples can evaluate a notation before implementation.** Scenario code
  can be reviewed against cognitive-dimension questions "before the API has
  even been implemented" ([Clarke 2004][clarke-2004]). A timed modification of
  one program in different notations exposed large differences in the effort
  of change ([Blackwell and Green 2003][cd-2003]). PLIERS ran
  documentation-only "Wizard of Oz" studies, with a person standing in for the
  compiler, and split its guide into short parts with comprehension checks
  after readers who skimmed failed ([Coblenz et al. 2021][pliers]).
- **Status is stated plainly.** Swift proposals say when nothing is
  implemented ([template][swift-template]); TC39 accepts "loosely-correct (not
  for production use)" prototypes at Stage 2 ([process][tc39-process]).
- **Pre-training works, mainly for novices.** Learners who first meet "the
  names and characteristics of the key concepts" learn more from a complex
  lesson (median d = 0.88 over ten experiments), and the effect "may be
  strongest for low knowledge learners" ([Mayer 2014][mayer-2014]). The
  original studies taught each component's possible states before the causal
  explanation ([Mayer et al. 2002][mayer-2002], abstract). Segmenting,
  signalling, coherence and spatial contiguity have similar support, each
  with effects that may shrink for high-knowledge learners
  ([Mayer 2008][mayer-2008]; [Mayer 2014][mayer-2014]).
- **Compare two cases, then state the principle.** Comparing two cases beats
  studying them separately ([Gentner et al. 2003][gentner-2003]); across a
  meta-analysis, case comparison outperformed other forms of case study
  (d = .50), most when learners looked for similarities and the principle
  came after the comparison ([Alfieri et al. 2013][alfieri-2013]). A single
  analogy can entrench a misconception, so say "the ways that analogies fail
  or mislead" and use several ([Spiro et al. 1988][spiro-1988]).

**Application to AIL**

- Label every example either **complete package** (admitted by
  `grammar.ebnf` and complete under draft 0.1 by inspection) or **fragment**,
  saying what it omits. The design proposal already labels its syntax sketch
  this way.
- Grow the tutorial in checkpoints. Rule L3 lets a package describe a named
  subset together with its whole dependency closure, which appears to allow
  each checkpoint to be a complete package for a smaller surface. The plan
  must confirm this against L3 before relying on it.
- Replace tool output with expected observations a reader can check by
  inspection: which clause changed, which rule applies, and what a conforming
  validator must reject and under which rule. Never show invented diagnostic
  text.
- Pair positive examples with failures drawn from conformance mutations
  N01–N33, one failure per block, each citing its rule. Keep N11's lesson: a
  valid document can still be false about an implementation.
- Include modification tasks, such as turning a borrowed result into an owned
  one, or adding a parameter and tracing every clause it then requires. The
  effort of change and hidden dependencies are review targets.
- Show the full clause burden in complete examples. Authoring burden is a
  review question, and hiding it would bias the review.
- Use the design history's before-and-after pair: the design proposal's
  syntax sketch set against draft 0.1.

### 4.4 Tests of reader outcomes and document quality

**Evidence**

- **Two kinds of quality.** Diátaxis separates functional quality (accuracy,
  completeness, consistency, usefulness, precision; measurable against the
  world) from deep quality (flow, fit to needs; judged), and addresses only the
  second ([quality][dx-quality]). *Docs for Developers* separates functional
  quality (whether a document achieves its goal) from structural quality (how
  well it is written), and holds that functional quality "is more important".
  It also recommends writing technical documentation "to a tenth grade level",
  checked with readability formulas ([chapter 9][dfd-oreilly]) — a position
  the evidence below contradicts
  ([§5](#5-tensions-between-sources)).
- **Rules that double as review criteria.** Clements et al. give seven rules
  for sound documentation — write for the reader, avoid unnecessary
  repetition, avoid ambiguity (and explain your notation), use a standard
  organization, record rationale, keep documentation current but not too
  current, and review for fitness of purpose — and say the same rules let a
  reader "say something constructive in a critical review". On the last rule:
  "Only the intended users of a document will be able to tell you whether it
  contains the right information presented in the right way"
  ([*Documenting Software Architectures*][dsa], prologue).
- **The reviewer's question.** "If I were the user, could I find, understand,
  and use this information?" — asked while reading the material in the same
  context users will see it ([DQTI][dqti-oreilly], chapter 13).
- **DQTI's model.** Nine characteristics in three groups: easy to use (task
  orientation, accuracy, completeness), easy to understand (clarity,
  concreteness, style) and easy to find (organization, retrievability, visual
  effectiveness). Each has guidelines and a checklist, and "Quality is
  ultimately determined by users" ([sample][dqti-sample]). Checklists and
  ratings track progress; usability tests find problems "especially in task
  orientation and concreteness" ([snippets][dqti-gbooks]). IBM's
  content-quality standard is an abridgement of the book ([IBM][ibm-quality]).
- **Quality lies partly outside the document.** For Rude and Eaton, "the
  measure of a 'good' document is in part outside that document", in its match
  to readers' needs ([publisher][rude-eaton]).
- **Readability formulas mislead.** They count sentence and word features,
  ignore content, organisation and audience, and "a readability score is
  seldom useful and can, indeed, be misleading"; "Even testing with a few
  people is much more informative" ([Redish 2000][redish-2000]).
- **Outcomes beat satisfaction.** A guideline-based revision raised task
  accuracy while ratings did not change ([Meng et al. 2020][meng-2020]). The
  cognitive-dimensions pilot proposes counting specific criticisms rather than
  satisfaction ([pilot][cd-2000]). Comprehension checks caught skimming
  readers ([PLIERS][pliers]). Measuring one aspect of writing tends to improve
  it at the expense of what is not measured (Hamilton's
  [sample chapter][managing-writers], paraphrased).
- **Paper tests that work.** Have someone removed from the work act as the
  reader; read only headings and first sentences and ask whether they tell a
  story ([Microsoft final review][ms-review]). Test with a screen reader and
  without colour or images ([Google accessibility][g-a11y]). Check the whole
  document against the expectations its introduction sets
  ([organizing large documents][tw2-large]).
- **Review questions from language communities.** TC39 asks whether a feature
  is "intuitive and learnable" and whether anything is "unstated"
  ([review guidance][tc39-review]). Swift asks reviewers how much effort they
  spent, and treats their questions as signals to improve the document
  ([process][swift-process]). Precision is repeatedly tested by whether
  someone other than the authors could implement the design (PEP, Swift, Rust
  and TC39).
- **Style rules are defaults.** Google: "This guide contains guidelines, not
  rules" ([About][g-style]); passive voice is acceptable to emphasise an
  object or when readers need not know the actor ([voice][g-voice]).
  Microsoft lists its own exceptions ([verbs][ms-verbs]). DQTI: "In general,
  use active voice unless the content and audience are better served by
  passive voice", and passive voice is "often more appropriate in reference
  information" ([DQTI][dqti-oreilly], chapter 8).
- **Plain language is defined by outcomes.** A communication is in plain
  language when "the intended readers can easily find what they need,
  understand what they find, and use that information"
  ([International Plain Language Federation][iplf]); ISO 24495-1:2023 adopts
  the same framing and sets readability formulas aside. ISO's terms exclude
  using its text to prompt AI systems, so later skills should cite the
  standard, not quote it.
- **Test with a few readers, repeatedly.** The US federal guidelines judge
  content clear only if users can find, understand and use it, and prescribe
  paraphrase testing ("6 to 9 interviews" per document), usability rounds of
  about three people, and "Plan to test at least twice"
  ([Federal guidelines][fed-pl], pp. 94–108). Paraphrase testing checks
  meaning, plus–minus marking captures reactions, and task tests suit
  documents people consult ([Jarrett and Redish 2020][jarrett-redish]). After
  a revision, readers report different kinds of problems — credibility rather
  than clarity ([de Jong and Rijnks 2006][dejong-2006], abstract).
- **Prose principles are heuristics.** Put the grammatical subject next to its
  verb, the action in the verb, old information in the topic position and new
  information in the stress position — but "None of these reader-expectation
  principles should be considered 'rules'" ([Gopen and Swan 1990][gopen-swan]).
  Clear prose names the characters as subjects and their actions as verbs;
  what feels passive usually comes "from abstract nouns and missing
  characters", not from passive verbs ([Williams][williams-1992]).

**Application to AIL.** [§9](#9-stage-scoped-acceptance-criteria) turns
these into stage checks under two principles. Functional quality is tested
against an enumerated inventory (grammar productions, clause names, rule
identifiers, conformance mutations), never a remembered list. Reader outcomes
are tested with a handful of real reviewers, and any model-based reader is
labelled as a proxy.

### 4.5 Navigation, reference entries, glossary, tables and diagrams

**Evidence**

- **Every page may be the first.** Navigation should show "where in the
  documentation they currently are, and what the context of the current topic
  is" ([Meng et al. 2019][meng-2019]). Opening a page is "an implicit query"
  that boilerplate fails to answer ([Robillard and DeLine 2011][rd-2011]).
  DQTI treats entry points as "signposts" and warns against assuming linear
  reading ([snippets][dqti-gbooks]).
- **Reference entries follow fixed patterns.**
  - Diátaxis: "Adopt standard patterns" ([reference][dx-ref]).
  - Microsoft: "Consistency is essential in reference documentation", with
    sections such as syntax, parameters, remarks, example, exceptions and see
    also ([reference documentation][ms-ref]).
  - DQTI's editing check asks whether reference information follows a pattern
    "such as purpose, syntax, restrictions, and usage notes", and whether
    syntax diagrams explain how to read them (p. 530).
  - CSS opens every property with value, initial value, "applies to",
    inheritance and computed value ([CSS 2.1 §1.4.2][css21]).
  - Man pages keep a conventional order that includes ERRORS and SEE ALSO
    ([man-pages(7)][man-pages]).
  - The AARM gives each clause an introduction, then fixed headings: syntax,
    name resolution, legality rules, static and dynamic semantics, bounded
    errors, examples ([AARM §1.1.2][aarm-structure]).
- **Rule identifiers need stable anchors.** Rust labels each rule, but says
  the identifiers are "not stable between releases" ([Rust Reference][rust-ref]).
  Ada makes a paragraph's number part of the paragraph and marks changes by
  version ([AARM §1.1.2][aarm-structure]). GitHub derives heading anchors from
  heading text, so rewording a heading breaks inbound links unless an explicit
  anchor is used ([GitHub][gh-basic];
  [headings as link targets][g-headings-targets]).
- **Grammar notation is defined before use.** Python, Go and Rust each define
  their EBNF dialect first; Go writes its EBNF in EBNF ([Go spec][go-spec]).
- **One-way links hide dependencies.** The classic hidden dependency is "the
  one-way pointer, where A points to B but B does not contain a back-pointer
  to A" ([tutorial][cd-tutorial]).
- **Terms and glossary.** Collect many new terms into a glossary
  ([Words][tw1-words]); define technical terms in context and keep "one term,
  one concept" ([Microsoft][ms-terms]); link "the first use of a term in a
  unit" ([Python developer guide][py-devguide]). A "Terms you need to know"
  section was part of Meng et al.'s successful revision, whose separate effects
  cannot be isolated ([2020][meng-2020]).
- **Tables.** Use a table when each item has "three or more pieces of related
  data"; do not merge cells; introduce the table in text
  ([Google tables][g-tables]; [accessibility][g-a11y]). DQTI: keep tables
  simple and avoid spanning cells (p. 482). No study of tables'
  effect on comprehension in this kind of material was found
  ([§10](#10-access-gaps-silences-and-acquisition-priorities)).
- **Diagrams.** Convey no new information only in an image, and distinguish
  alt text, caption and figure description ([Google images][g-images]). The
  reader must get "the whole story" from either picture or text
  ([Microsoft graphics][ms-graphics]). Write the caption first, keep about a
  paragraph of information per diagram, and show the big picture before each
  subsystem ([illustrating][tw2-illus]). Mermaid embeds an accessible title
  and description in the diagram source ([Mermaid][mermaid-a11y]).
- **Diagrams by level, with keys.** *Docs for Developers*: several diagrams
  are fine where splitting keeps each simple; overview diagrams help "readers
  new to your product or domain", lower-level diagrams suit reference; give
  each diagram a clear starting point; publish SVG; describe the image in the
  body text, not only in alt text; and it names C4 as "a standardized way of
  visualizing levels of abstraction" ([chapter 6][dfd-oreilly]). Clements et
  al.: "Every diagram in the architecture documentation should include a key
  that explains the meaning of every symbol used" (rule 3a); a top-level
  context diagram "makes a good first introduction"; and a reader cannot tell
  whether a relation a diagram does not show is absent or merely omitted
  unless the documentation states its completeness, which they call
  descriptive completeness ([*Documenting Software Architectures*][dsa],
  prologue and chapter 6). Rozanski and Woods define a view by the stakeholder
  concerns it addresses ([2005][rozanski-woods], chapter 3). Simple line
  drawings serve understanding better than decorative graphics
  ([Clark and Mayer 2023][clark-mayer], chapter 9).
- **C4: hierarchical, but for systems.** C4 is "A set of hierarchical
  diagrams" — system context, containers, components and code — and teams
  need only the levels "that add value"; context and container diagrams "are
  sufficient for most" ([diagrams][c4-diagrams]). Its notation rules require a
  title stating type and scope, a key "explaining the notation being used",
  each element's type and description, and labelled one-way arrows; a
  yes/no review checklist applies them ([notation][c4-notation];
  [checklist][c4-checklist]). C4 is "designed to model a software system":
  "To document a library, framework or SDK, you might be better off using
  something like UML", and it invites supplementing C4 "with UML state
  diagrams, timing diagrams" ([FAQ][c4-faq]). The site and its example
  diagrams are licensed CC BY 4.0.
- **Hierarchy and integration across diagrams.** Moody's principles for
  notations include complexity management by "recursive decomposition", with
  levels that "should vary depending on the complexity of the underlying
  domain"; cognitive integration across diagrams through an overview diagram
  that "acts as an overall cognitive map" of about seven elements; dual coding,
  in which "text and graphics together" beat either alone; graphic economy; and
  cognitive fit, with "different visual dialects for different tasks and/or
  audiences" ([Moody et al. 2010][moody-2010], restating
  [Moody 2009][moody-2009], whose full text was not accessible).
  Shneiderman's mantra is "Overview first, zoom and filter, then
  details-on-demand" ([1996][shneiderman-1996]).
- **Every representation has a price.** A diagram helps only readers who know
  how to use it; it is useful "only to those who know the appropriate
  computational processes" ([Larkin and Simon 1987][larkin-simon]). Each added
  representation must be learned and translated, and "an excessive number of
  representations rarely helps learning" ([Ainsworth 2006][ainsworth-2006],
  author's manuscript). Reading graphics is "an acquired skill", and novices
  "tend to confuse visibility with relevance" ([Petre 1995][petre-1995]).

**Application to AIL**

- **Reference entry fields:** name; form (production and clause syntax);
  operands and their types; applies to; required or optional, and
  repeatability; default ("none; required" where the draft has no default);
  meaning, normative, with rule identifier; constraints; rejected when (rule
  identifiers and mutation IDs); a labelled example; used by and see also; and
  a link to non-normative rationale.
- **Indexes:** alphabetical clauses, clauses by declaration kind, rule
  identifiers, and mutations to rules.
- **Anchors:** give each rule identifier an explicit anchor that does not
  depend on heading wording.
- **Notation page:** explain how to read `grammar.ebnf`, including why its
  generic `clause` production leaves the clause vocabulary to prose tables.
- **Glossary:** include "familiar term, different meaning" notes, linked from
  each term's first use on a page.
- **Figures by level:** a C4 system-context diagram of the AIL ecosystem
  (producers, packages, validators, consumers) as the one overview reused on
  every page; a C4 container view of the toolchain; then, because contracts
  are not software systems, UML-style sequence, state and timing diagrams for
  ownership transfer, handle lifecycles, borrow validity and callback
  schedules; plus a document map and a clause-applicability matrix drawn as a
  table.
- **One visual vocabulary, several renderers:** PlantUML, D2 and Mermaid
  render under one shared vocabulary — the same shape and colour per concept,
  the same names as the prose, one legend convention — and the diagram kind
  changes only when the question changes.

## 5. Tensions between sources

| Tension | Sources | Resolution for this reader |
|---|---|---|
| Explanation inside tutorials | Diátaxis minimises it; Google explains why before a task; Meng et al. integrate concepts into tasks | Teach domain concepts before the tutorial; give a one-line reason and a link inside it |
| Separate by type or integrate | DQTI (p. 340) and Diátaxis's warning against blur, against Meng et al. 2019 and Robillard and DeLine | Pages pure in purpose; navigation by journey and subject; local context on every page |
| Linear or page one | Robillard and DeLine prefer linear overviews; Baker and Nassif and Robillard favour self-contained fragments | A linear tutorial and domain explanation; page-one reference entries |
| Complete or incomplete examples | Meng et al.: complete, no placeholders; Atkinson et al. and minimalism: deliberate gaps prompt reasoning | Complete canonical examples; gaps only as labelled exercises in the review guide |
| Redundancy | Meng et al. repeat critical concepts; Kalyuga et al. find redundancy harms experts | Repeat binding obligations where needed; never repeat language-design basics |
| Generic or subject types | Diátaxis's four modes; Baker's types specific to the subject | Modes set a page's purpose; subject templates (clause entry, domain concept, review task) set its shape |
| Guide or plan | Diátaxis: no top-down plan and no empty structures; the brief: audit first, then structure briefs | Plan small increments that each land with working navigation and no empty sections |
| Authoritative reference or honest draft | Diátaxis: "wholly authoritative"; proposal genres: open questions explicit | State draft 0.1 rules authoritatively; keep an open-question register; never soften a MUST |
| "should" | Google: "Generally avoid"; RFC 2119 and 8174 give capitalised SHOULD a defined meaning | Keep capitalised keywords in normative text under a stated BCP 14 convention; plain-language rules govern explanatory prose |
| Brevity | Microsoft: "Shorter is always better"; Google prefers clarity; minimalism: brevity is no end in itself; DQTI: "only as much detail as users need" | Judge length against the reader's need, and keep the context |
| Conditions before actions | Google: condition, goal and location first, results after; Microsoft: location first where confusion is likely; DQTI accepts either order and starts conditional steps with "if" | Apply to procedures and review tasks, not to the wording of normative rules |
| Self-contained pages or one rule source | Baker's self-containment; the brief's single canonical rule source | Local summaries labelled non-normative and linked to the rule identifier |
| Reading-level targets | *Docs for Developers*: write "to a tenth grade level", checked by formulas; Redish: formulas are invalid for adult technical material and "seldom useful" | No grade-level target. For expert readers and normative text, test comprehension with readers instead |
| Detail in overview diagrams | Overview diagrams for newcomers (*Docs for Developers*; Google); descriptive completeness (Clements et al.) | Each level states what it omits, so an absent line never reads as an absent relation |
| Variety of notations | The human's decision for PlantUML, D2 and Mermaid as equals; Ainsworth: "an excessive number of representations rarely helps"; Moody: graphic economy and the cost of integrating diagrams; Petre: reading graphics is learned | Tools become renderers under one visual vocabulary and one legend convention; the diagram kind changes only when the question does |
| Similarity or difference first | Comparison research: seek similarities, then state the principle (Alfieri et al.); misconception research: stress differences and where analogies fail (Ambrose et al.; Spiro et al.) | Compare cases, state the principle, then name where the comparison breaks |

## 6. The editing prompt against its sources

The requirements record gives the prompt's criteria: an audit before rewriting;
Diátaxis separation; technical accuracy; conditions before actions; clear actors
and active voice; explicit noun references; imperative procedural steps;
meaningful identifiers; code typography and tagged fences. It cites Diátaxis,
*Docs for Developers*, DQTI, and Google and Microsoft guidance.

| Criterion or attribution | What the source says | Consequence |
|---|---|---|
| Diátaxis separation | Four needs, but a guide rather than a plan; no empty structures; readers enter anywhere; no claim on functional quality | A purpose test for pages, not four mandatory top-level sections |
| *Docs for Developers* | Content types, functional and structural quality, code-sample principles; sections of three chapters read, the rest only as headings and abstracts; Rabil lists it unread. Its tenth-grade reading-level target conflicts with the evidence | Supports the approach, except the reading-level target; nothing is attributed to chapters not read |
| DQTI "quality" | Nine characteristics, of which style is one; active voice "most of the time"; passive "often more appropriate in reference information" | Quality is not voice counting |
| Active voice | Google, Microsoft and DQTI each name exceptions | Active where the actor matters; passive or stative where a rule concerns a document and the actor is irrelevant |
| Conditions before actions | Google's rule, for circumstances, goals and locations, with results and reasons after; Microsoft only for location; DQTI accepts either order | Applies to procedures and review tasks |
| Explicit noun references | Google: "In many cases" follow *this* or *these* with a noun; a five-word rule of thumb | A copy-edit check, not a ban |
| Imperative steps | Google and Microsoft, for procedures; Microsoft keeps indicative mood elsewhere | Procedures and review tasks only |
| Code typography | Google and Microsoft agree | Adopt |
| Tagged fences | Only Microsoft's contributor guide prefers them; Google's own pages disagree on fences versus indentation | A project convention; GitHub has no `ail` highlighter |
| "Production-ready documentation" | — | Documentation fit for review. AIL remains a proposed draft with no parser |
| "Get an API call working" | A product-documentation outcome | No runtime exists. The equivalent outcome is tracing or modifying a contract and predicting a validator's verdict |
| Timeless and future-feature rules | Google aims them at shipped products | Applied here, they would suppress required "not implemented" statements |

The prompt's own summaries could not be compared with the books verbatim
because the prompt text is unavailable; the table checks the attributions the
requirements record.

## 7. The current AIL material, read as this reader

These observations ground the survey in the actual pages. They are inputs to
the audit, not the audit: they come from reading the [README][ail],
[language rules][ail-language], the start of [contracts][ail-contracts],
[conformance][ail-conformance], `grammar.ebnf`, `examples/buffers.ail` and the
[design proposal][ail-design]'s alternatives and syntax sections at the working
copy of 2026-09-23.

- **The entry path starts with normative rules.** The README states status and
  scope precisely, then routes readers through language rules, contracts,
  profiles, baseline and conformance in that order. It uses *access path*,
  *ABI* and *profile* before any page defines them, and no page introduces the
  binding domain. The design rationale is linked last, under "Design basis".
- **The examples are complete but unannotated.** The three example packages
  contain no comments and one `doc(...)` clause each. A reader meets
  `buffers.ail`'s `view` operation as a run of effect and slot clauses, an
  outcome block and an access block, with its prose meaning in a separate
  table in `conformance.md`.
- **One passage already shows the target explanation style.** "Two views of one
  contract" in `conformance.md` renders `view` as five plain sentences and as an
  illustrative machine projection, and marks the projection as non-normative.
- **Failure cases already exist.** The rejection mutations N01–N33 each pair a
  one-line change to a positive example with the rule that must reject it. N11
  separately shows that a valid document can still be false about an
  implementation.
- **Reference lookup crosses documents.** The grammar's `clause` production
  admits any name, so the closed clause vocabulary exists only in prose tables
  spread across `language.md`, `contracts.md` and `profiles.md`. Rule
  identifiers are section-level (L1–L6, C1–C8, A1–A5, F1). A reader who meets
  `retain($store, none)` in an example must know to look in `contracts.md` C3.
- **Rationale and open design questions sit outside the reading path.** The
  design proposal's alternatives table and its note that "whether ownership
  appears principally in types or contract clauses needs a corpus-backed
  decision" are exactly what a language reviewer wants to judge. Draft 0.1
  chose clauses; the sketch in the same proposal used `borrow` in types.
- **Process narration shares reader pages.** `conformance.md` ends with a
  verification record of a disposable script run. It is honest evidence, but it
  is record material inside a reader page.

## 8. Formats, diagram notations and the walk-away check

The human asked whether AsciiDoc or DocBook would suit this richer
documentation better than Markdown, and named Typst as another production
target. They also decided that PlantUML, D2 and Mermaid are equal diagram
notations, each diagram committing its source and a tool-generated SVG. The
brief requires that core content be readable without a new publishing site or
graphical tool. For every approach, the walk-away question is: with the tool
absent, what can a reviewer still read?

### 8.1 Document formats

| Criterion | GitHub-flavoured Markdown | AsciiDoc (Asciidoctor 2.0.26) | DocBook 5.2 | Typst 0.15.1 |
|---|---|---|---|---|
| On GitHub | Rendered in full, including alerts, footnotes, native Mermaid and SVG images | A "preview" in secure mode: includes become links, diagram blocks show their source; TOC, cross-references, callouts and attributes work | Shown as XML source | Shown as source; a committed PDF is viewable but its links are ignored |
| Single-sourcing `.ail` examples | None; needs a generator and committed output | Tagged-region includes, with tags behind `//` comments, which AIL already treats as comments; disabled on GitHub | XInclude or external text; no tagged regions | `read()` into `raw()`; no tagged regions |
| Line annotations | None | Numbered callouts behind line comments | Callouts by coordinates or inline | Package annotations keyed to line numbers |
| Admonitions | Five alert types with fixed titles | Five types with configurable captions | A fixed set, plus editorial remarks | Any, through packages or functions |
| Stable rule references | Heading anchors change with the text; explicit `<a name>` anchors work but are unchecked | Explicit IDs; invalid references warned with `-v` | `xml:id` and `xref`; checkable with Schematron | Labels on referenceable elements |
| Glossary and index | None | Glossary description lists; an index only in PDF or DocBook output | Glossary and index elements | Packages (glossarium, in-dexter) |
| Diagrams | Native Mermaid; others as committed SVG | Build-time rendering (Asciidoctor Diagram, Kroki); source only on GitHub | Pre-rendered images | Native packages, or SVG or PDF images |
| Outputs | HTML through the host | HTML5, DocBook, man pages; PDF and EPUB add-ons | HTML5 (xslTNG), print through CSS or XSL-FO | PDF, PNG, SVG; HTML experimental |
| Raw legibility | Highest | High | Lowest | Medium: markup mixed with code |

Evidence behind the table:

- **Markdown.** GFM "is a strict superset of CommonMark" that adds tables,
  task lists, strikethrough, autolinks and disallowed raw HTML
  ([GFM spec][gfm-spec]); alerts have five fixed types ([GitHub][gh-basic]).
  Code snippets from other files can be embedded only in comments, not in
  Markdown files. GitHub will not add a language to Linguist without "at least
  2000 files" per extension ([Linguist][linguist-contrib]), so `ail` code stays
  unhighlighted.
- **AsciiDoc.** Tagged includes select lines between markers placed "after a
  line comment", and tag lines are discarded ([include][asciidoc-include]);
  callouts can sit behind line comments. But GitHub "processes AsciiDoc files
  using the `SECURE` mode", in which "the include directive is converted to a
  link" ([safe modes][asciidoctor-safe]). Asciidoctor's lead: "there is no
  such thing as GitHub-flavored AsciiDoc" and GitHub's output "will always just
  be a preview" ([issue 939][asciidoctor-939]); the official workaround
  flattens includes in CI ([Reducer][asciidoctor-reducer]). "Until the first
  version of the AsciiDoc Language Specification is ratified, AsciiDoc is
  defined by the Asciidoctor implementation" ([docs][asciidoc-docs]).
- **DocBook.** Semantically the richest, but OASIS development "ended with the
  closure of the DocBook Technical Committee in September, 2024"
  ([docbook.org][docbook-org]); the XSLT 1.0 stylesheets were last released
  in 2016, and the current XSLT 3.0 stylesheets need Java and Saxon
  ([DocBook: The Definitive Guide][docbook-tdg]).
- **Typst.** HTML export is "very incomplete" — "Do not use this feature for
  production use cases" ([HTML][typst-html]). SVG and PNG exports turn text
  into shapes, so screen readers cannot read it ([SVG][typst-svg]). Packages
  are pinned to exact versions and immutable once published
  ([packages][typst-packages]). Mermaid SVGs that use HTML labels lose their
  text when embedded ([issue 1421][typst-1421]). GitHub renders neither Typst
  nor DocBook.

**Assessment.** Reviewers read this set in the repository. Markdown is the
only format that GitHub renders in full, so it passes the walk-away check
wherever reviewers read. AsciiDoc is the stronger authoring format — tagged
includes, callouts, validated identifiers, glossaries and spanning tables —
but its main advantage, single-sourced examples, is exactly what GitHub
disables. Both formats therefore need a small generation step with committed
output if examples are single-sourced for GitHub readers. DocBook fails the
walk-away check and has an uncertain toolchain future. Typst suits a PDF
edition, not reading in the repository.

The evidence favours Markdown plus a deterministic, committed generation step
for this review set, keeping one format across the repository. AsciiDoc
becomes the better choice if the human wants a published HTML or PDF edition,
and Asciidoctor and Asciidoctor PDF are already installed locally. The brief
leaves this decision to planning, with the human.

### 8.2 Diagram toolchains

| Notation | Produces SVG with | Needs | C4 support | Legend | Local |
|---|---|---|---|---|---|
| PlantUML, with C4-PlantUML | `plantuml -tsvg`; its `--extract-source` option recovers the source from the SVG | Java; Graphviz for several diagram types, or the experimental Smetana layout | C4-PlantUML in the standard library, offline | `SHOW_LEGEND()` | Installed (1.2026.8, Graphviz 16.1.0) |
| D2 | `d2 in.d2 out.svg` (SVG is the default) | One binary; dagre, ELK and TALA layouts (TALA open source since 2026-09-07) | `c4-person` shape, C4 theme, model and views since v0.7 | `d2-legend` | Installed (0.9.0) |
| Mermaid | `mmdc -i x.mmd -o x.svg` | Node and a headless Chromium | Experimental C4 diagrams | None for C4 | Not installed; GitHub renders Mermaid natively |
| Structurizr (consolidated, Apache-2.0) | Export to PlantUML or Mermaid, or SVG through headless Chromium | Java 21 or Docker | Reference implementation of C4: one model, many views | Generated key | Not usable as installed: local Java is 16 |
| LikeC4 | Generate D2, PlantUML, Mermaid or Dot, then render | Node | C4-inspired, any number of levels | Experimental | Not checked |
| Kroki | HTTP API returning SVG | The public service, or self-hosted containers | Through C4-PlantUML, Structurizr and D2 | As the underlying tool | — |
| Typst packages | Inside Typst (fletcher, CeTZ, diagraph for Graphviz, pintorita, chronos for sequences, digidraw for timing, Mermaid re-implementations) | Typst; WASM plugins | None found; import the tool's SVG instead | Per package | Typst 0.15.1 installed |

Sources: [PlantUML and Graphviz][plantuml-dot], [C4-PlantUML][c4-plantuml],
[D2 exports][d2-exports] and [D2's C4 support][d2-c4],
[mermaid-cli][mermaid-cli], [Structurizr end of life][structurizr-eol] and
[export][structurizr-export], [LikeC4][likec4], [Kroki][kroki],
[Asciidoctor Diagram][asciidoctor-diagram], and the
[Typst package index][typst-packages]. Local versions come from version checks
only.

Three risks follow. **Drift:** tool upgrades change committed SVGs even when
the picture does not — Mermaid 12 re-lays out existing diagrams, D2 0.9.0
writes smaller files, and TALA layouts vary with small edits — so pin tool
versions and regenerate and compare in a repeatable check. **Accessibility:**
only Mermaid documents accessible titles and descriptions; no accessibility
documentation was found for PlantUML or D2, and Typst's SVG text is not
readable by screen readers, so the adjacent prose must carry every fact.
**Privacy:** Kroki's public service receives the diagram source, and no
privacy statement was found.

### 8.3 The walk-away check for this repository

| Approach | Depends on | Without it | Recommendation |
|---|---|---|---|
| Markdown in the repository | GitHub, or any GFM renderer, for tables, anchors and the outline | Raw text stays legible; links show their targets | Baseline. Keep tables narrow enough to read raw, since reviews happen on raw markup ([Moir 2024][moir-2024]) |
| Heading-derived anchors | GitHub's slug rules | Links still show `file.md#fragment` | Give rule identifiers explicit, lower-case anchors that survive heading edits |
| PlantUML, D2 or Mermaid diagrams | Their renderers, to regenerate | Source text and the committed SVG both remain; GitHub also renders Mermaid source | The human's decision. Pin versions; prose carries every fact |
| Fenced text diagrams | Nothing | Fully legible | For small sketches, with an adjacent sentence stating the figure's role |
| AsciiDoc files | Asciidoctor for full rendering | GitHub shows a preview: includes as links, diagram blocks as source | Only with flattened, committed output or a published edition |
| Typst | The Typst compiler | Source only on GitHub; PDF links ignored | A possible PDF edition, not the reading format |
| DocBook or DITA | An XML toolchain | Verbose XML source | Not recommended |
| Static-site generators or Antora | A build and a site | Source legible; navigation and search lost | Not requested by the brief |
| Example extraction and link checks | A small script or checker | Documents unaffected; the check only verifies | Candidate `Taskfile.yml` tasks if planning adopts them |
| `ail` code fences | GitHub Linguist, which has no AIL entry | Unhighlighted but legible | Tag fences `ail` anyway |

## 9. Stage-scoped acceptance criteria

These checks are the survey's acceptance criteria for the audit, the plan and
the editorial pipeline (`draft`, `copy-edit`, `art`, `proof`). Each belongs to
the stage whose charter owns it, so a stage needs only its own list. Bracketed
tags name the evidence.

**Planning: the audit, the architecture and the structure briefs**

- P1. Each page has one stated purpose (a compass answer), a reader, an
  outcome, prerequisites and non-goals. [Diátaxis; Technical Writing One]
- P2. The reviewer journey is explicit, and every page also works as a first
  page. [Baker; Meng et al. 2019]
- P3. Every domain concept the brief lists (FFI, ABI, ownership transfer,
  borrow validity, quiescence, projections, profiles), and any the audit adds,
  has one defining page, the failure it prevents, and a rule for linking its
  first use. [brief; Microsoft terms; Python developer guide]
- P4. A reference coverage map is built from an enumerated inventory: grammar
  productions, clause names taken from the prose tables, rule identifiers and
  conformance mutations. [brief; Diátaxis reference; TC39 "unstated"]
- P5. A diagram inventory lists each figure's level, the question it answers,
  its diagram kind and its renderer, starting from one overview diagram; C4
  levels cover the ecosystem and toolchain, and contract diagrams use other
  kinds. [C4; Moody; Shneiderman; Technical Writing Two illustrating]
- P6. One prose contract serves the whole set: the BCP 14 keyword convention,
  the voice rule with its exceptions, the scope of conditions before actions,
  the term list, code typography, fence tags, example labels and figure rules.
  [RFC 8174; Google; Microsoft; DQTI]
- P7. The document format is settled with the human before any draft. [brief]
- P8. Each increment lands with working navigation and no empty sections.
  [Diátaxis workflow]
- P9. A term-risk register lists every AIL term that collides with Rust,
  Swift, C++ or everyday usage, with the difference to state and a rule of
  thumb. [Ambrose et al.; Kaplan et al.; Federal guidelines]
- P10. Binding-domain components and their possible states are introduced
  before the tutorial combines them; language-design basics are never taught.
  [Mayer; Kalyuga et al.]
- P11. Each document is aimed at one primary reader, and its sections follow
  the order in which that reader would ask their questions. [Federal
  guidelines]

**Draft: structure and technical truth**

- D1. Every normative statement traces to a rule identifier; no rule is
  invented; each ambiguity found becomes review feedback, not a quiet fix.
  [brief; TC39 on authoritative text]
- D2. Domain obligations and AIL's choices are distinguished wherever both
  appear. [W3C TAG; Swift ABI section]
- D3. Where a choice is explained, its alternatives, drawbacks and open
  questions are visible. [Rust RFC; PEP; Swift; W3C TAG]
- D4. Every example is labelled complete or fragment; each concept has at
  least two examples; failures cite their rule; expected observations can be
  checked by inspection; no tool output is invented. [Atkinson et al.;
  Diátaxis; Rust Reference]
- D5. Each page opens with its page-one block and stays on one level.
  [Baker; Meng et al. 2019]
- D6. The tutorial is split into short parts, each ending with a check.
  [PLIERS; minimalism; Mayer segmenting]
- D7. A concept is taught by comparing two cases before stating its principle;
  every analogy to another language states where it fails.
  [Gentner et al.; Alfieri et al.; Spiro et al.; DQTI chapter 7]
- D8. Anything that does not serve a page's purpose moves to the page whose
  purpose it serves. [Clark and Mayer coherence; Diátaxis]

**Copy-edit: sentences, terms and the prose contract**

- C1. One term, one concept. Glossary terms are linked at their first use on
  each page, and familiar-term collisions are flagged. [Microsoft; Python
  developer guide; DQTI p. 210]
- C2. In procedures and review tasks, conditions, goals and locations come
  before the action, and results after it. [Google; Microsoft; DQTI]
- C3. Steps use the imperative; running text uses the indicative. [Microsoft]
- C4. *This* and *these* are followed by a noun wherever the referent could be
  unclear. [Google]
- C5. Active voice where the actor matters; passive or stative where a rule
  concerns a document and the actor is irrelevant. [Google; Microsoft; DQTI]
- C6. Capitalised BCP 14 keywords keep their defined meaning, and lowercase
  *should* stays out of normative sentences. [RFC 8174; Google prescriptive
  documentation]
- C7. Identifiers, clause names and file names are in code font; code fences
  are tagged. [Google; Microsoft contributor guide]
- C8. A figure of speech gives way to the mechanism it stands for, and a claim
  the reader cannot check gives way to one they can. [grove `copy-edit`]
- C9. Each obligation sentence names the responsible party as its subject and
  the action as its verb, with old information first and the new condition in
  the stress position ("The callee borrows `buf` until the callback
  returns"). These are heuristics, not rules. [Gopen and Swan; Williams]
- C10. Headings, list items and paragraph openings start with their
  information-carrying words, and link text pairs an AIL term with a gloss a
  newcomer understands. [Pernice; Nielsen; Budiu]

**Art: figures**

- A1. A relation the reader must otherwise hold in mind — a sequence, a
  lifetime, a partition, a mapping — is drawn, not left to prose alone.
  [grove `art`; Technical Writing Two]
- A2. Each figure states its role in an adjacent sentence, the prose carries
  every fact the figure shows, and alt text or a description is present.
  [Google images; Microsoft graphics; Mermaid accessibility]
- A3. Diagrams run from context to detail; the overview diagram is reused as
  the map on every page; each level says what it zooms into and what it omits.
  [C4; Moody cognitive integration; Clements et al.; Technical Writing Two]
- A4. Each diagram's source and its generated SVG are committed together, and
  the SVG regenerates from the source. [human decision, 2026-09-23]
- A5. Meaning never rests on colour alone, and tables have no merged cells.
  [Microsoft; Google]
- A6. Labels sit on the diagram elements they name, and code annotations sit
  beside the lines they explain rather than in a caption. [Mayer spatial
  contiguity]
- A7. Every diagram passes the C4 checklist, whatever its notation: a title
  stating type and scope; a key for every shape, colour and line style; each
  element's type and description; one-way arrows labelled with specific verbs.
  [C4 notation and checklist; Clements et al. rule 3a]
- A8. All renderers share one visual vocabulary — one symbol and colour per
  concept, the prose's names, one legend convention — and a new diagram kind
  appears only when the question changes. [Moody; Ainsworth; Petre]

**Proof: the whole document**

- R1. The reviewer journey is walked end to end from each entry page. [brief]
- R2. Reference coverage matches the enumerated inventory from P4. [brief]
- R3. Links, anchors, code fences and tables are checked. [brief]
- R4. Status statements agree everywhere: draft 0.1, no parser, synthetic
  fixtures. [brief]
- R5. Terms are used consistently across pages, and the handed-forward list is
  cleared. [grove `proof`]

**Reader test: outside the pipeline, scheduled by the plan**

- T1. Two or three language designers new to the domain attempt the brief's
  completion tasks cold. Record where they stall and the specific criticisms
  they make. [Redish; Blackwell and Green 2000; Microsoft final review]
- T2. Tutorial parts carry comprehension checks. [PLIERS]
- T3. Criticisms are counted and compared across readers; agreement between
  independent readers is the validity signal. [Blackwell and Green 2000]
- T4. A model-based reader is reported as a labelled proxy, never as evidence
  of human comprehension.
- T5. Paraphrase-test the contract terms; use plus–minus marking on the
  orientation page and task tests on the reference and review guide; run at
  least two rounds, expecting different problem types in the second.
  [Federal guidelines; Jarrett and Redish; de Jong and Rijnks]
- T6. Give authors and editors the readers' think-aloud transcripts before
  they revise; do not rely on the authors' own estimates of difficulty.
  [Schriver 1992; Hinds 1999]

The same partition suits requirements and design conversations with a human,
which share these readers' needs: a model of what the listener already knows,
a concrete case before the abstraction, the big picture before the detail, and
trade-offs presented with a recommendation and room to disagree. Codifying the
partition as stage-scoped skills is separate work for the grove repository.

## 10. Access gaps, silences and acquisition priorities

**Searched for, and found no primary source:**

- An empirical study of how programmers read language references or
  specifications. The delegated searches recorded their queries.
- A peer-reviewed evaluation of Diátaxis; only practitioner reports exist.
- Guidance written for documenting an unimplemented language for reviewers.
  The proposal genres in [§4.1](#41-reviewers-new-to-the-domain-judging-a-proposal)
  are the nearest evidence.
- A study of how tables affect comprehension in material of this kind.
- Any treatment of normative keywords (RFC 2119) or text-based diagram
  languages in Google's or Microsoft's style guidance.
- A glossary section in any proposal template inspected (Rust, PEP, Swift,
  W3C TAG, TC39, Go, Kotlin KEEP).

**Read only in part, or not at all:**

- DQTI beyond chapter 1 and the cited sections of chapters 7, 8, 9 and 13;
  *Docs for Developers* beyond the cited sections of chapters 5, 6 and 9;
  Clark and Mayer beyond the cited sections of chapters 6, 9 and 13;
  *Every Page Is Page One* beyond its outlines; *Technical Editing* and Markel
  and Selber beyond their contents.
- *The Nurnberg Funnel*; van der Meij and Carroll (1995), whose heuristics were
  read through a 2021 secondary source; Meng et al. (2018), abstract only;
  Carroll and Carrithers (1984); Nasehi et al. (2012).
- IBM's content-quality page now shows an end-of-support notice; its text came
  through IBM's documentation interface.
- Diátaxis's complex-hierarchies page is cited from the project's history; it
  left the live site on 2026-08-03.
- The human's verbatim editing prompt.

**Acquisition priorities.** Full text would add most where the survey now
rests on outlines or contents and the book bears on a stage the pipeline runs.
"Usable" means a DRM-free PDF or EPUB that can be read locally; purchased
copies stay outside this repository, which is public, because each vendor's
licence is personal.

| Priority | Title | What full text would add | Usable legitimate copy | Already reachable |
|---|---|---|---|---|
| 1 | *Structured Writing* (Baker, 2018) | Structured-writing domains, structures, and markup languages from Markdown to DITA, bearing on entry templates and the open format question; known only from its contents | [XML Press][structured-writing] EPUB and PDF bundle | No |
| 2 | *Every Page Is Page One* (Baker, 2013) | Methods behind principles already taken from its outline: defining topic types from the subject, subject-affinity linking under a link policy, big-picture and pathfinder topics | [XML Press][eppo-pub] EPUB and PDF bundle | No |
| 3 | DQTI (3rd ed.) | Chapters not yet read; purchase is a convenience only | [InformIT][dqti-informit] EPUB and PDF, watermarked | Yes: O'Reilly, read in a browser |
| — | *Technical Editing* (Rude and Eaton, 5th ed.) | Levels of edit and the comprehensive-editing process | None: print only | No. Substitute: Eaton's open-access chapter ["Editing"][eaton-editing] (2023), which blocks automated download |
| 4 | *Docs for Developers* (2nd ed., 2026) | The new edition's additions on AI and automation; chapters not yet read | Springer EPUB and PDF | 1st ed. on O'Reilly |
| 5 | *Style: Lessons in Clarity and Grace* (13th ed.) | Sentence-level principles for copy-editing | None: DRM editions only | No |
| 6 | *Multimedia Learning* (Mayer, 3rd ed.) | Primary evidence behind pre-training, segmenting and signaling | None: DRM editions only | Partly, through Clark and Mayer on O'Reilly |
| 7 | *Information Architecture* (4th ed.) | Labelling, navigation and controlled vocabularies | ebooks.com EPUB and PDF | Yes: O'Reilly |

*Technical Writing 101* (from its publisher), the *Web Style Guide* (4th
edition, online) and the Microsoft Writing Style Guide are free. So is the
C4 model: its site and example diagrams are licensed CC BY 4.0, which also
allows adapting them with attribution.

## 11. Where these findings were applied

The [documentation audit](https://github.com/Linkuistics/APIAnyware/blob/8103dd6b10eb7c243abd5b4344212936690b3001/semantic/docs/research/interface-language-documentation-audit.md) applies
§7 and the planning criteria in §9. The
[documentation plan](https://github.com/Linkuistics/APIAnyware/blob/8103dd6b10eb7c243abd5b4344212936690b3001/semantic/docs/design/2026-09-23-interface-language-documentation-plan.md)
applies §4 (page architecture, reference entries and figures), §5 and §6 (its
prose contract), §8 (the human chose Markdown on 2026-09-23) and §9 (its stage
checks and reader checkpoints). The plan records one departure from §9: no
cold readers new to the domain are available, so T1, T3 and T5 become a
labelled proxy reader plus the human's own reading.

[ail]: https://github.com/Linkuistics/APIAnyware/blob/8103dd6b10eb7c243abd5b4344212936690b3001/semantic/specification/interface-language/README.md
[ail-language]: https://github.com/Linkuistics/APIAnyware/blob/8103dd6b10eb7c243abd5b4344212936690b3001/semantic/specification/interface-language/language.md
[ail-contracts]: https://github.com/Linkuistics/APIAnyware/blob/8103dd6b10eb7c243abd5b4344212936690b3001/semantic/specification/interface-language/contracts.md
[ail-conformance]: https://github.com/Linkuistics/APIAnyware/blob/8103dd6b10eb7c243abd5b4344212936690b3001/semantic/specification/interface-language/conformance.md
[ail-design]: https://github.com/Linkuistics/APIAnyware/blob/8103dd6b10eb7c243abd5b4344212936690b3001/semantic/docs/design/2026-09-22-interface-language.md
[rabil]: https://richardrabil.com/2024/08/18/the-best-books-on-technical-writing-an-in-depth-analysis/
[dx]: https://diataxis.fr/
[dx-tut]: https://diataxis.fr/tutorials/
[dx-how]: https://diataxis.fr/how-to-guides/
[dx-ref]: https://diataxis.fr/reference/
[dx-exp]: https://diataxis.fr/explanation/
[dx-compass]: https://diataxis.fr/compass/
[dx-work]: https://diataxis.fr/how-to-use-diataxis/
[dx-map]: https://diataxis.fr/map/
[dx-quality]: https://diataxis.fr/quality/
[dx-hier]: https://github.com/evildmp/diataxis-documentation-framework/blob/de7a2d813e2fda339b959e3075afb4e2f09bf54c/complex-hierarchies.rst
[eppo-outline]: https://everypageispageone.com/the-book/
[eppo-pub]: https://xmlpress.net/publications/eppo/
[dqti-sample]: https://www.informit.com/content/images/9780133118971/samplepages/9780133118971.pdf
[dqti-gbooks]: https://books.google.com/books?id=RlLiAwAAQBAJ
[dqti-oreilly]: https://www.oreilly.com/library/view/developing-quality-technical/9780133119046/
[dqti-informit]: https://www.informit.com/store/developing-quality-technical-information-a-handbook-9780133119008
[structured-writing]: https://xmlpress.net/publications/structured-writing/
[eaton-editing]: https://doi.org/10.37514/TPC-B.2023.1923.2.11
[dfd-oreilly]: https://www.oreilly.com/library/view/docs-for-developers/9781484272176/
[clark-mayer]: https://www.wiley.com/en-us/e-learning-and-the-science-of-instruction-proven-guidelines-for-consumers-and-designers-of-multimedia-learning-5th-edition-p-9781394177370
[dsa]: https://www.informit.com/store/documenting-software-architectures-views-and-beyond-9780321552686
[rozanski-woods]: https://www.viewpoints-and-perspectives.info/
[ibm-quality]: https://www.ibm.com/docs/en/technical-content?topic=standards-content-quality
[dfd]: https://link.springer.com/book/10.1007/978-1-4842-7217-6
[dfd-front]: https://link.springer.com/content/pdf/bfm:978-1-4842-7217-6/1
[dfd2]: https://link.springer.com/book/10.1007/979-8-8688-2509-5
[rude-eaton]: https://www.pearson.com/en-us/subject-catalog/p/technical-editing/P200000002181
[markel13]: https://web.archive.org/web/20230209085408/https://www.macmillanlearning.com/college/us/product/Technical-Communication/p/1319245005
[lannon15]: https://web.archive.org/web/20250328183440/https://www.pearson.com/en-us/subject-catalog/p/technical-communication/P200000002249/9780137477548
[markel-ps]: https://www.macmillanlearning.com/college/us/product/Practical-Strategies-for-Technical-Communication/p/1319476775
[document-design]: https://sunypress.edu/Books/D/Document-Design-Second-Edition
[tw101]: https://store.scriptorium.com/product/classic-technical-writing-101-a-real-world-guide-to-planning-and-writing-technical-content-third-edition/
[hackos]: https://www.wiley.com/en-us/Information+Development:+Managing+Your+Documentation+Projects,+Portfolio,+and+People-p-x000306263
[horton]: https://www.wiley.com/en-us/e+Learning+by+Design,+2nd+Edition-p-9780470900024
[dita-bp]: https://www.informit.com/store/dita-best-practices-a-roadmap-for-writing-editing-and-9780132480529
[managing-writers]: https://xmlpress.net/publications/managing-writers/
[docs-like-code]: https://www.docslikecode.com/book/
[krug]: https://sensible.com/dont-make-me-think/
[web-style-guide]: https://webstyleguide.com/
[norman]: https://jnd.org/books/the-design-of-everyday-things-revised-and-expanded-edition/
[g-style]: https://developers.google.com/style
[g-voice]: https://developers.google.com/style/voice
[g-xref]: https://developers.google.com/style/cross-references
[g-headings-targets]: https://developers.google.com/style/headings-targets
[g-a11y]: https://developers.google.com/style/accessibility
[g-images]: https://developers.google.com/style/images
[g-tables]: https://developers.google.com/style/tables
[tw1-audience]: https://developers.google.com/tech-writing/one/audience
[tw1-docs]: https://developers.google.com/tech-writing/one/documents
[tw1-words]: https://developers.google.com/tech-writing/one/words
[tw2-large]: https://developers.google.com/tech-writing/two/large-docs
[tw2-illus]: https://developers.google.com/tech-writing/two/illustrations
[tw2-code]: https://developers.google.com/tech-writing/two/sample-code
[ms-welcome]: https://learn.microsoft.com/en-us/style-guide/welcome/
[ms-verbs]: https://learn.microsoft.com/en-us/style-guide/grammar/verbs
[ms-dev]: https://learn.microsoft.com/en-us/style-guide/developer-content/
[ms-code]: https://learn.microsoft.com/en-us/style-guide/developer-content/code-examples
[ms-ref]: https://learn.microsoft.com/en-us/style-guide/developer-content/reference-documentation
[ms-terms]: https://learn.microsoft.com/en-us/style-guide/word-choice/use-technical-terms-carefully
[ms-common]: https://learn.microsoft.com/en-us/style-guide/word-choice/dont-use-common-words-in-new-ways
[ms-review]: https://learn.microsoft.com/en-us/style-guide/final-publishing-review
[ms-graphics]: https://learn.microsoft.com/en-us/style-guide/accessibility/graphics-design-media
[rust-rfc]: https://github.com/rust-lang/rfcs/blob/master/0000-template.md
[rust-rfc-2017]: https://github.com/rust-lang/rfcs/commit/f026f04f77
[pep-1]: https://peps.python.org/pep-0001/
[swift-template]: https://github.com/swiftlang/swift-evolution/blob/main/proposal-templates/0000-swift-template.md
[swift-process]: https://github.com/swiftlang/swift-evolution/blob/main/process.md
[swift-cxx]: https://github.com/swiftlang/swift-evolution/blob/main/visions/using-c%2B%2B-from-swift.md
[w3c-explainer]: https://w3ctag.github.io/explainer-explainer/
[tc39-process]: https://tc39.es/process-document/
[tc39-review]: https://github.com/tc39/how-we-work/blob/main/stage-3-review.md
[go2]: https://github.com/golang/proposal/blob/master/go2-language-changes.md
[py-ref-index]: https://docs.python.org/3/reference/index.html
[py-devguide]: https://devguide.python.org/documentation/style-guide/
[rust-ref]: https://doc.rust-lang.org/reference/introduction.html
[go-spec]: https://go.dev/ref/spec
[c99-rationale]: https://www.open-std.org/jtc1/sc22/wg14/www/C99RationaleV5.10.pdf
[aarm]: http://www.ada-auth.org/standards/22aarm/html/AA-0-2.html
[aarm-structure]: http://www.ada-auth.org/standards/22aarm/html/AA-1-1-2.html
[rfc2119]: https://www.rfc-editor.org/rfc/rfc2119
[rfc8174]: https://www.rfc-editor.org/rfc/rfc8174
[cd-1996]: http://web.archive.org/web/20160418025934/http://homepage.ntlworld.com/greenery/workStuff/Papers/UsabilityVPs.PDF
[cd-tutorial]: https://www.cl.cam.ac.uk/~afb21/CognitiveDimensions/CDtutorial.pdf
[cd-2003]: https://www.cl.cam.ac.uk/~afb21/publications/BlackwellGreen-CDsChapter.pdf
[cd-q]: https://www.cl.cam.ac.uk/~afb21/CognitiveDimensions/CDquestionnaire.pdf
[cd-2000]: https://www.ppig.org/files/2000-PPIG-12th-blackwell.pdf
[clarke-2004]: http://web.archive.org/web/20241111015143/https://drdobbs.com/windows/measuring-api-usability/184405654
[pliers]: https://arxiv.org/abs/1912.04719
[robillard-2009]: https://www.cs.mcgill.ca/~martin/papers/software2009a.pdf
[rd-2011]: https://www.cs.mcgill.ca/~martin/papers/ese2011.pdf
[ur-2015]: https://www.cs.mcgill.ca/~martin/papers/ieeesw2015.pdf
[meng-2018]: https://doi.org/10.1177/0047281617721853
[meng-2019]: http://www.mangold-international.com/resources/meng-et-al-how-developers-use-api-documentation-2019.pdf
[meng-2020]: https://dl.acm.org/doi/fullHtml/10.1145/3380851.3416759
[nr-2023]: https://www.cs.mcgill.ca/~martin/papers/2023-chi-lbw.pdf
[minimalism-1996]: https://ris.utwente.nl/ws/files/249663536/Caroll1996ten.pdf
[minimalism-2021]: https://osuva.uwasa.fi/server/api/core/bitstreams/e96bd3f9-c878-49ae-9cc0-e5774f840bdf/content
[atkinson-2000]: https://doi.org/10.3102/00346543070002181
[kalyuga-2003]: https://doi.org/10.1207/S15326985EP3801_4
[nielsen-2006]: https://www.nngroup.com/articles/progressive-disclosure/
[redish-2000]: https://redish.net/wp-content/uploads/Redish_on_Readability_Formulas.pdf
[css21]: https://www.w3.org/TR/CSS2/about.html#property-defs
[man-pages]: https://man7.org/linux/man-pages/man7/man-pages.7.html
[gh-diagrams]: https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-diagrams
[gh-basic]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax
[mermaid-c4]: https://mermaid.js.org/syntax/c4.html
[c4-diagrams]: https://c4model.com/diagrams
[c4-notation]: https://c4model.com/diagrams/notation
[c4-checklist]: https://c4model.com/diagrams/checklist
[c4-faq]: https://c4model.com/faq
[moody-2010]: https://doi.org/10.1007/s00766-010-0100-1
[moody-2009]: https://doi.org/10.1109/TSE.2009.67
[shneiderman-1996]: http://hdl.handle.net/1903/466
[larkin-simon]: https://doi.org/10.1111/j.1551-6708.1987.tb00863.x
[ainsworth-2006]: https://doi.org/10.1016/j.learninstruc.2006.03.001
[petre-1995]: https://doi.org/10.1145/203241.203251
[mayer-moreno-2003]: https://doi.org/10.1207/S15326985EP3801_6
[mayer-2008]: https://doi.org/10.1037/0003-066X.63.8.760
[mayer-2014]: https://doi.org/10.1007/978-1-4614-3185-5_31
[mayer-2002]: https://doi.org/10.1037/1076-898X.8.3.147
[ambrose-2010]: https://media.wiley.com/product_data/excerpt/01/04704841/0470484101.pdf
[kaplan-2009]: https://doi.org/10.1080/10691898.2009.11889535
[spiro-1988]: https://files.eric.ed.gov/fulltext/ED301873.pdf
[gentner-2003]: https://doi.org/10.1037/0022-0663.95.2.393
[alfieri-2013]: https://doi.org/10.1080/00461520.2013.775712
[camerer-1989]: https://doi.org/10.1086/261651
[hinds-1999]: https://doi.org/10.1037/1076-898X.5.2.205
[pinker-2014]: https://stevenpinker.com/files/pinker/files/the_source_of_bad_writing_-_wsj_0.pdf
[schriver-1992]: https://doi.org/10.1177/0741088392009002001
[pirolli-1999]: https://doi.org/10.1037/0033-295X.106.4.643
[budiu-2020]: https://www.nngroup.com/articles/information-scent/
[pernice-2017]: https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content/
[gopen-swan]: https://www.americanscientist.org/blog/the-long-view/the-science-of-scientific-writing
[williams-1992]: https://www.michbar.org/file/generalinfo/plainenglish/pdfs/92_jan.pdf
[fed-pl]: https://web.archive.org/web/20241231152827/https://www.plainlanguage.gov/media/FederalPLGuidelines.pdf
[iplf]: https://www.iplfederation.org/plain-language/
[jarrett-redish]: https://www.uxmatters.com/mt/archives/2020/05/how-to-test-the-usability-of-documents.php
[dejong-2006]: https://research.utwente.nl/en/publications/dynamics-of-iterative-reader-feedback-an-analysis-of-two-successi/
[mermaid-a11y]: https://mermaid.js.org/config/accessibility.html
[moir-2024]: https://thisisimportant.net/posts/docs-as-code-broken-promise/
[gfm-spec]: https://github.github.com/gfm/
[gh-markup]: https://github.com/github/markup
[linguist-contrib]: https://github.com/github-linguist/linguist/blob/main/CONTRIBUTING.md
[asciidoc-docs]: https://docs.asciidoctor.org/asciidoc/latest/
[asciidoc-include]: https://docs.asciidoctor.org/asciidoc/latest/directives/include-tagged-regions/
[asciidoctor-safe]: https://docs.asciidoctor.org/asciidoctor/latest/safe-modes/
[asciidoctor-939]: https://github.com/asciidoctor/asciidoctor.org/issues/939
[asciidoctor-reducer]: https://github.com/asciidoctor/asciidoctor-reducer
[asciidoctor-diagram]: https://docs.asciidoctor.org/diagram-extension/latest/
[docbook-org]: https://docbook.org/
[docbook-tdg]: https://tdg.docbook.org/tdg/5.2.1/
[typst-docs]: https://typst.app/docs/
[typst-html]: https://typst.app/docs/reference/html/
[typst-svg]: https://typst.app/docs/reference/svg/
[typst-packages]: https://github.com/typst/packages
[typst-1421]: https://github.com/typst/typst/issues/1421
[plantuml-dot]: https://plantuml.com/graphviz-dot
[c4-plantuml]: https://github.com/plantuml-stdlib/C4-PlantUML
[d2-exports]: https://d2lang.com/tour/exports/
[d2-c4]: https://d2lang.com/blog/c4/
[mermaid-cli]: https://github.com/mermaid-js/mermaid-cli
[structurizr-eol]: https://docs.structurizr.com/eol
[structurizr-export]: https://docs.structurizr.com/export
[likec4]: https://likec4.dev/
[kroki]: https://docs.kroki.io/kroki/

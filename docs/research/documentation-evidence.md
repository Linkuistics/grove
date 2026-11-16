# Documentation evidence: reader checkpoints, proposal inventory and dated facts

This document preserves the evidence behind Grove's documentation guidance work
([#14]–[#19]) so that later design, evaluation and skill changes can cite it by
ID. It holds three records:

1. the **reader-evidence record** of five human reading checkpoints in
   APIAnyware's documentation workstream;
2. an **enumerated proposal inventory**, one stable ID per proposal and
   material refinement in #14–#19 and in the bounded part of the [#9 comment];
3. the **dated capability facts** about tools, formats and hosts that were
   checked on 2026-09-23 and must be rechecked before guidance relies on them.

**Nothing here is adopted guidance.** Every item is a proposal awaiting a
disposition — adopted with an owner and acceptance case, narrowed with a
reason, or declined with a reason — which the design leaf
`documentation-guidance-k7` records against these IDs. A source recommending
an item is not evidence that adding it to a Grove stage improves that stage.

The underlying survey is imported verbatim, with its provenance, as
[`documentation-basis-a.md`](documentation-basis-a.md) (cited below as
"survey §n"). The issue bodies and comments were read on 2026-09-30 from
GitHub; an item's wording here paraphrases its source, and the link beside it
is the authority.

## Contents

- [Evidence classes and inspection depth](#evidence-classes-and-inspection-depth)
- [Reader-evidence record](#reader-evidence-record)
- [How the inventory was enumerated](#how-the-inventory-was-enumerated)
- [Proposal inventory](#proposal-inventory)
- [Where the issues go beyond the survey](#where-the-issues-go-beyond-the-survey)
- [Dated capability facts](#dated-capability-facts)
- [Enumeration ledger](#enumeration-ledger)
- [Where the proposals landed](#where-the-proposals-landed)

## Evidence classes and inspection depth

Each inventory item names the class of evidence it rests on, strongest first
where it has several. The classes are the root brief's eight; they are not
interchangeable, and one never stands in for another.

| Code | Class | What it covers here |
|---|---|---|
| CR | Controlled research | Experiments, meta-analyses and syntheses of them. **CR-obs** marks an observational or field study (readers observed or interviewed, nothing manipulated); it stays in this class rather than a ninth one, but is weaker evidence of cause. |
| RH | Rhetorical heuristic | Analysis or argument about prose, diagrams or design that was not tested: Gopen and Swan, Williams, Pinker, Moody's notation principles, Shneiderman's mantra. |
| HS | House-style default | A style guide, framework, template or community convention stating its default: Google, Microsoft, DQTI, Diátaxis, C4, Baker, the Federal Plain Language Guidelines, Clements et al.'s rules, proposal templates, RFC 8174. |
| RP | Explicit reader preference | The human's stated request, direction or audience clarification. In this record it is the human directing the APIAnyware workstream, who is also the reader at every checkpoint. |
| MP | Model proxy | A model reader's answers or findings. Never evidence of human comprehension. |
| MC | Mechanical check | A link check, render, regeneration, reproduction of viewer behaviour or version check. It establishes the property checked and nothing about understanding. |
| HA | Human acceptance | The human's positive verdict on material. It does not imply a full read or a completed task. |
| TP | Demonstrated task performance | The intended reader completing a task with the material. **No item in this record rests on TP by this workstream's readers**; Meng et al. 2020's task accuracy is CR about other readers. |

**Inspection depth** says how much of the source was actually read, as the
survey reports it (survey [§2](documentation-basis-a.md#2-method) and
[§3](documentation-basis-a.md#3-inventory)):

| Depth | Meaning | Sources at this depth |
|---|---|---|
| primary | The official page or paper itself | Google and Microsoft guides, Diátaxis, C4 pages, proposal templates, RFCs, tool documentation, and the papers §3.3 lists without a narrower depth |
| sections | Named sections of a book, in full text | DQTI ch. 7, 8, 9, 13; *Docs for Developers* ch. 5, 6, 9; Clark and Mayer (5th ed.) ch. 6, 9, 13; Clements et al. (2nd ed.) prologue and ch. 6; Rozanski and Woods ch. 3, 13 |
| excerpt | Publisher sample or snippet search | DQTI's other pages (sample, snippets); Ambrose et al. ch. 1; Williams |
| contents | Outline or table of contents only | Baker, *Every Page Is Page One*; *Technical Editing* |
| abstract | Abstract only | Schriver 1992; Meng et al. 2018; Mayer, Mathias and Wetzell 2002; de Jong and Rijnks 2006 |
| secondary | Read through another source | Moody 2009 (through Moody et al. 2010); van der Meij and Carroll 1995 (through Virtaluoto et al. 2021) |
| checkpoint | An APIAnyware reader-checkpoint report | See the [reader-evidence record](#reader-evidence-record) for its extent |
| unverified | The issue's quotation or figure is not in the survey text, so no depth was recorded | See [where the issues go beyond the survey](#where-the-issues-go-beyond-the-survey) |

The survey had one author, with research agents reading some clusters under
the same rules, and it spot-checked the load-bearing quotations it lists in §2.
It did not revalidate every source, and this document fetched none.

## Reader-evidence record

All five checkpoints come from one APIAnyware documentation set: reviewer
documentation for a proposed API Interface Language (AIL), whose declared
reader knows language design and railroad notation but is new to API binding.
**One human read at every checkpoint** — the human directing that
workstream, who already knows the binding domain (#14's first checkpoint
comment says so), reading on behalf of that declared reader. None of
the five reports the human's full reading extent. The handles name leaves in
APIAnyware's task tree, which was removed when that workstream finished; their
running logs survive only in that repository's history. The issue comments
linked in each row are the durable record.

### RC-1 — `first-path-reading-k8`, 2026-09-23

- **Reported in:** [#14][i14-k8], [#15][i15-k8], [#16][i16-k8],
  [#17][i17-k8], [#18][i18-k8].
- **State of the material:** orientation, concept pages and tutorial had
  completed draft → copy-edit → art → proof, with the documentation checks
  recorded as passing (MC).
- **Model proxy (MP):** one fresh-context model read the twelve first-path
  pages. It explained the language's purpose, interpreted the examples,
  answered the three requested outcomes, made a borrow-to-owned change on paper
  and predicted an additional enum/outcome error. It reported repeated
  introductions and growing complete examples as friction.
- **Human (RP):** stopped the read.
  > I don't understand the phrase "every producer crosses the same publication
  > boundary". And many other phrases are not understandable. The language is
  > too compressed, and needs to be cast in terms the reader can understand.

  They also requested positive framing, citing the title "What a signature
  leaves unsaid", and asked for these problems to be fixed before further
  feedback.
- **Reading extent and route:** an interrupted read of the first path; how far
  the human got is not stated beyond the quoted phrase and "many other
  phrases".
- **Probe independence:** the proxy answered every outcome in one model
  context; the human read separately.
- **Established:** the passing checks and the proxy's successful answers did
  not establish human understanding. The human identified a compressed
  relationship and negative framing as defects.
- **Not established:** prevalence, causal attribution to any skill or stage,
  or that the proposed remedies work (the comments say so explicitly).
- **Refines:** nothing earlier; it is the first checkpoint. It gives rise to
  DR-33–DR-37, CE-17–CE-20, AR-12–AR-14 and PR-16–PR-20.

### RC-2 — `first-path-reading-k48`, 2026-09-28

- **Reported in:** [#14][i14-k48], [#15][i15-k48], [#16][i16-k48],
  [#17][i17-k48], [#18][i18-k48].
- **Human (RP):** quoted the opening two paragraphs of the canonical
  `language.md` L2 rule on identity and dependencies. Asked for an example
  distinguishing a local declaration from a non-local one, a member example,
  and snippets as constructs are introduced. Of the following paragraph's
  reference forms and namespace restrictions:
  > Introduces syntactic items that make no sense because I don't know the
  > context.

  They requested language-model diagrams, railroad diagrams and examples,
  applied generally before another review.
- **Reading extent and route:** the assigned starting route was the revised
  overview and domain explanation; the response identifies canonical L2
  passages instead. The route taken and the extent are unreported.
- **Probe independence:** no model proxy is reported for this checkpoint.
- **Established:** a missing teaching context at a direct rule entry.
- **Not established:** that the earlier corrections to orientation, concepts
  or tutorial failed (the quoted wording was preserved canonical text outside
  those corrections), or that the new remedy works.
- **Refines:** RC-1's remedies, by adding direct rule entries to what a
  teaching route must serve (X-23, DR-38–DR-43, CE-21–CE-23, AR-15, AR-16,
  PR-21–PR-24).

### RC-3 — `first-path-reading-k64`, 2026-09-28 and 2026-09-29

- **Reported in:** [#14][i14-k64], [#15][i15-k64], [#17][i17-k64],
  [#12][i12-k64]. The #14, #17 and #12 comments date the checkpoint
  2026-09-28; the #15 comment dates it 2026-09-29. The railroad and
  "improvement" statements are from 2026-09-28; the model-first direction and
  the presentation requests are from 2026-09-29.
- **Human (HA, RP):**
  > I mean your improvement is good.

  > You can assume the reader is familiar with railroad diagrams.

  On 2026-09-29:
  > I think we should prioritize explaining the model, and then describing the
  > language by reference to the model.

  with the reason "Because the language is a tool and is not of interest by
  itself." They asked to remove the boxes teaching railroad reading and to
  delimit non-normative aids with a background colour or left margin line.
- **Reading extent and route:** neither reported. The positive verdict is on
  the examples-and-notation correction and precedes the model-first revision.
- **Probe independence:** no model proxy is reported for this checkpoint.
- **Established:** a positive human assessment of that correction; an audience
  fact (railroad fluency); a structural requirement (model first).
- **Not established:** full-path coverage, the contribution of any stage,
  diagram type or skill, or any effect of the later structural revision.
- **Refines:** the generic "key and how-to-read line" of AR-06 and the railroad
  keys of AR-16 (now AR-17); the snippet-beside-syntax remedy of DR-39 (now
  DR-44, DR-45); representation familiarity versus subject familiarity (WU-01).

### RC-4 — `model-reading-k73`, 2026-09-29

- **Reported in:** [#14][i14-k73], [#15][i15-k73], [#17][i17-k73],
  [#18][i18-k73].
- **Human (HA, RP):** said the material was "looking good", praised the
  example fragments, and quoted the punctuation walkthrough in
  `explanation/contracts.md`, *From signature to contract*, calling the
  explanations of parentheses, commas and semicolons obvious to a
  language-aware reader.
  > In fact, given your excellent use of example fragments, we should keep the
  > railroad diagrams with the grammar description, and just link to the
  > grammer/railroad diagrams.

  > Remember that we want to convert everything we've learnt here about
  > produce good documentation into skills/agent guidance, so we want to keep
  > everything we've learnt in some way.
- **Delivery environment (MC):** the human reported "The model page diagrams
  show as broken image placeholders", viewing "the MD in VSCode Preview".
  Reproduction showed `No Folder Opened` and HTTP 401 for both SVGs; the links
  resolved to existing, valid SVGs, which also rendered in an independent
  renderer. Opening the containing documentation folder restored both diagrams
  with Restricted Mode still enabled. The fact is recorded as CF-40.
- **Reading extent and route:** the extent is not reported; the reported
  surfaces are the quoted passage of `explanation/contracts.md` and the model
  page in a standalone VS Code preview.
- **Probe independence:** no model proxy is reported for this checkpoint.
- **Established:** a scoped reader preference (no punctuation narration for
  this audience; railroads at the grammar destination); a verified image-loading
  repair.
- **Not established:** comprehension, a general advantage of centralised
  diagrams, or any stage's effectiveness. The environment repair is not a
  content or comprehension result.
- **Refines:** the local instruction to explain "each new name and punctuation
  mark", and with it DR-39 and CE-22 (now DR-46, X-26, X-27); the
  diagrams-at-introductions request of RC-2 (now AR-19, X-28).

### RC-5 — `full-set-reading-k12`, 2026-09-29

- **Reported in:** [#14][i14-k12], [#15][i15-k12], [#18][i18-k12].
- **Human (HA, RP):**
  > Apart from that point, I am very impressed with your work

  and "for now this is good." The exception is source/platform framing: the
  model and AIL are intended for .NET, Linux/Wayland/GTK/KDE, Rust and other
  typed libraries as well as Apple-related APIs, and a rationale comparing
  dedicated syntax with extended Swift syntax made Swift seem privileged. They
  clarified that interpreted and dynamically typed languages are intended
  consumers; their source-side preference was tentative. More conversational
  prose and clause-specific syntax exploration are explicitly later work.
- **Model proxy (MP):** one fresh-context model read all 29 published pages,
  the grammar, nine complete example packages and 26 diagram source/SVG pairs,
  and attempted all six reading outcomes. **Its five direct-entry probes shared
  one accumulating context**, so they are not five independent cold reads. One
  finding survived inspection: entering `buffers.ail` directly shows the
  complete package but no onward reading link.
- **Mechanical (MC):** local-link checks passed with 29 pages reachable, which
  establishes reachability from the checked indexes, not usability from a
  raw-source entry.
- **Reading extent and route:** the human cited the Swift rationale passage;
  their route, full extent and task outcomes were not reported. The proxy read
  the whole published set.
- **Probe independence:** the proxy's direct-entry probes shared one
  accumulating context, as above; the human read separately.
- **Established:** acceptance for now with one bounded framing correction; a
  navigation gap at a raw-source entry, found by proxy.
- **Not established:** human comprehension, task performance, or which stage
  caused the positive verdict.
- **Refines:** adds domain-breadth and role-separation proposals (X-31–X-33,
  DR-47–DR-49) and entry-surface recording (PR-27–PR-29).

## How the inventory was enumerated

The inventory is complete by construction, not by recall. Every structural
unit of each in-scope source was listed and then classified:

- **Units:** every numbered item, bullet and table row in each issue body and
  comment, and every block of a comment (comments carry most of their proposals
  in paragraphs rather than lists). A comment's **block n** is its nth
  blank-line-separated block, counting a quotation, a list or a table as one
  block; bullets and rows are numbered within their block.
- **In scope:** the bodies and all comments of [#14]–[#19]; section 2 of the
  [#9 comment], whole; section 1 of that comment only where it restates #17 or
  bears on `visual-design-discussion.md`'s Structurizr recommendation; of #12,
  only the [one comment][i12-k64] that #14–#18 link. Bare mentions of #12 or #9
  are pointers, not content.
- **Classification:** each unit is exactly one of — a **proposal** (gets an
  ID); a **material refinement** that narrows or redirects an earlier proposal
  (gets an ID and names what it narrows); a **tension** or **conflict** between
  sources that a disposition must settle (gets an ID); a **fact** checked at a
  date (gets a CF ID); **evidence** for an item, a **pointer**, **provenance**,
  or a **local disposition** in APIAnyware (no ID). A unit that restates an
  existing ID is recorded against that ID rather than given a new one.
- **Survey crosswalk:** the survey's §9 criteria, §5 tensions, §8.3 walk-away
  recommendations and each §4 "Application to AIL" bullet were mapped to IDs.
  Those with no counterpart in the issues get `SV` IDs. The survey's §4
  "Evidence" bullets are evidence for items, not proposals, and are not
  enumerated.

The [enumeration ledger](#enumeration-ledger) at the end lists every unit and
its classification, so a missing proposal shows up as a unit with no entry.

## Proposal inventory

Columns: **Source** links the unit; **Class** uses the codes above; **Depth**
is the inspection depth of the cited source; **Relations** names what the item
narrows (←) and what narrows it (→).

### X — cross-stage principles ([#14])

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| X-01 | Distribute documentation knowledge across `draft` → `copy-edit` → `art` → `proof` as stage-specific tasks, not one large skill every task loads. | [#14] Intent, bullet 1 | RP | human direction | Root brief requirement |
| X-02 | Weight prose, expression style, pedagogic structure and reader psychology, not only technical accuracy. | [#14] Intent, bullet 2 | RP | human direction | |
| X-03 | Use a variety of diagrams by hierarchical decomposition, C4 at the top then PlantUML, D2 and Mermaid, each committing source and tool-generated SVG. | [#14] Intent, bullet 3 | RP | human direction | → AR-01–AR-11; the root brief exempts figures in the page's own bytes and makes notation teaching conditional (AR-17) |
| X-04 | Apply the same practice to requirements gathering and design conversations with the human. | [#14] Intent, bullet 4 | RP | human direction | → CV-01–CV-06 |
| X-05 | Model the reader per domain: expertise is domain-specific; documentation is what the task needs minus what the reader has; address a qualified reader but let the unqualified recognise it. | [#14] Principle 1 | CR (Kalyuga et al.); HS (Google; Baker) | primary; Baker contents | → X-25, WU-01 |
| X-06 | Writers cannot judge difficulty alone; reader evidence beats self-review. | [#14] Principle 2 | CR (Camerer et al.; Hinds); CR, abstract (Schriver) | primary; Schriver abstract | Evidence for PR-08, SV-03 |
| X-07 | Style rules are defaults with stated exceptions. | [#14] Principle 3 | HS (Google, Microsoft, DQTI) | primary; DQTI sections | → CE-05, CE-16 |
| X-08 | Quality is outcome-based (readers find, understand, use); readability scores are not gates, including *Docs for Developers*' tenth-grade target. | [#14] Principle 4 | HS (IPLF definition; Diátaxis; DfD ch. 9); CR review (Redish 2000) | primary; DfD sections | Overlaps CE-11 |
| X-09 | Separate purposes, not contexts: each page pure in purpose, every page with local context. | [#14] Principle 5 | HS (Diátaxis); CR-obs (Meng et al. 2019) | primary | Overlaps DR-01, DR-13; survey §10: no peer-reviewed evaluation of Diátaxis |
| X-10 | Teach names before mechanisms, and state where analogies fail. | [#14] Principle 6 | CR (Mayer 2014; Alfieri et al.); CR synthesis (Ambrose et al.) | primary; Ambrose excerpt | Overlaps DR-03, DR-19 |
| X-11 | Partition: the structure brief (written at planning, read by `draft`) owns reader model, journey, page purpose, concept inventory, term risks, coverage and diagram inventories, prose contract, format. | [#14] Partition, row 1 | HS | issue synthesis of survey §9 | Detailed by DR-01–DR-09 |
| X-12 | Partition: `draft` owns structure and technical truth, proposal-genre shape, examples policy, reference entries. | [#14] Partition, row 2 | HS | as above | Detailed by DR-10–DR-26 |
| X-13 | Partition: `copy-edit` owns prose-contract tests with their sources' exceptions. | [#14] Partition, row 3 | HS | as above | Detailed by CE items |
| X-14 | Partition: `art` owns figures by level, one visual vocabulary, keys, dual coding. | [#14] Partition, row 4 | HS | as above | Detailed by AR items |
| X-15 | Partition: `proof` owns whole-journey checks, the diagram checklist and a cold re-read. | [#14] Partition, row 5 | HS | as above | Detailed by PR-01–PR-07 |
| X-16 | Partition: reader testing sits outside the pipeline — paraphrase, plus–minus and task tests, counted criticisms. | [#14] Partition, row 6 | HS; CR | as above | Detailed by PR-08–PR-15 |
| X-17 | Knowledge added to a stage meets the pilot's bar: preregister the defect class it should close and measure whether it does, rather than adding checks because a source recommends them. | [#14] How to evaluate | — (process rule; precedent: the editorial-pipeline pilot) | this repository | Root brief acceptance seam 2 |
| X-18 | Label evidence strength: Mayer's principles are controlled experiments with novices; Gopen and Swan and Williams are rhetorical analysis; style guides are house defaults. | [#14] Cautions, bullet 1 | — (meta-rule) | — | Root brief requirement; this document's class column |
| X-19 | Do not use ISO 24495-1 as prompt material; cite the IPLF definition instead. | [#14] Cautions, bullet 2; [#16] Evidence notes | — (licence constraint recorded in #12) | — | Root brief note |
| X-20 | The C4 site and example diagrams are CC BY 4.0; adapt with attribution. | [#14] Cautions, bullet 3; [#9 comment] §1 | MC (licence) | primary | Root brief note |
| X-21 | Acquire, in priority order, DRM-free editions of Baker's *Structured Writing* and *Every Page Is Page One* and optionally DQTI; use Eaton's open-access "Editing" chapter in place of *Technical Editing*. | [#14] Books | — (acquisition) | survey §10 | Would raise depth for DR-01, DR-13, CE items |
| X-22 | An abstract-relationship defect crosses stages: teach it in `draft`, then sentence work, figure-language checks and whole-path proof. | [#14 k8 comment][i14-k8], block 6 | RP; MP | checkpoint RC-1 | → CE-20, PR-19 |
| X-23 | An accessible explanation route and a precise rule source need a usable transition between them; the figure and example inventory accounts for direct rule entries, not only the tutorial route. | [#14 k48 comment][i14-k48], block 7 | RP | checkpoint RC-2 | ← DR-07; overlaps DR-43, PR-22 |
| X-24 | Delimit non-normative aids visually (background colour or left margin line) instead of headings and end-headings. | [#14 k64 comment][i14-k64], block 9; [#17 k64 comment][i17-k64], block 6 | RP | checkpoint RC-3 | ← SV-05 |
| X-25 | Requirements and structure brief: state expertise separately for each domain and representation, including what the reader does not need taught; preserve the human's words and the locations they actually report. | [#14 k73 comment][i14-k73], block 8, row 1 | RP | checkpoint RC-4 | ← X-05, DR-02; with WU-01 |
| X-26 | Draft: "explain the example" means establish unfamiliar referents, relationships, conditions and consequences for this audience, not a prose translation of every token; located fragments and complete examples serve different jobs. | [#14 k73 comment][i14-k73], block 8, row 2 | RP | checkpoint RC-4 | ← DR-39, CE-22; same as DR-46 |
| X-27 | Copy-edit: check each explanation adds meaning the reader needs; remove elementary syntax narration without compressing unfamiliar concepts back into jargon. | [#14 k73 comment][i14-k73], block 8, row 3 | RP | checkpoint RC-4 | ← CE-22 |
| X-28 | Art: place semantic relationship figures with their explanation, and railroad views with the grammar description linked from elsewhere; usefulness does not imply embedding at every introduction. | [#14 k73 comment][i14-k73], block 8, row 4 | RP | checkpoint RC-4 | ← AR-15; same as AR-19 |
| X-29 | Proof and reader testing: verify both routes and the actual viewing environment; keep rendering, structural consistency and human understanding as separate results; a quoted passage does not establish a complete route. | [#14 k73 comment][i14-k73], block 8, row 5 | RP; MC | checkpoint RC-4 | Overlaps PR-24, PR-26 |
| X-30 | Cross-session agent guidance: preserve observation, requirement, proposed change and limits together, and retain refinements so later work does not recover an over-broad rule; the human asked to "keep everything we've learnt". | [#14 k73 comment][i14-k73], block 8, row 6 and quote | RP | checkpoint RC-4 | Root brief requirement; this document |
| X-31 | State a general model's intended domain separately from the repository or examples that motivated it. | [#14 k12 comment][i14-k12], block 4 | RP | checkpoint RC-5 | Same as DR-47 |
| X-32 | Keep the source API, the description producer and the consumer roles distinct. | [#14 k12 comment][i14-k12], block 4 | RP | checkpoint RC-5 | Same as DR-48 |
| X-33 | Preserve the timing of feedback: acceptance with a bounded correction can coexist with later ambitions, which are not instructions to rewrite now. | [#14 k12 comment][i14-k12], block 4; [#15 k12 comment][i15-k12], block 4 | RP | checkpoint RC-5 | |

### DR — structure brief and `draft` ([#15])

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| DR-01 | Brief: one purpose per page, one primary reader per document; the Diátaxis compass is a purpose test, not mandatory top-level sections. | [#15] brief item 1 | HS (Diátaxis; Federal guidelines p. 3) | primary; p. 3 unverified | Survey P1, P11; §10: no evaluation of Diátaxis |
| DR-02 | Brief: a reader model stated as distance from the knowledge, including what not to teach. | [#15] brief item 2 | HS (Google); CR (Kalyuga et al.) | primary | → X-25, WU-01 |
| DR-03 | Brief: a concept inventory in pre-training order — each domain concept, the failure it prevents, and the section introducing it before use. | [#15] brief item 3 | CR (Mayer 2014, median d = 0.88); HS/CR (Clark and Mayer ch. 13) | primary; sections | Survey P3, P10; → DR-44 |
| DR-04 | Brief: a term-risk register for terms whose everyday or neighbouring meanings intrude, with the difference to state. | [#15] brief item 4 | CR synthesis (Ambrose et al.; Kaplan et al.); HS case (Federal guidelines pp. 110–111) | excerpt; primary | Survey P9; risk between programming languages is *inferred* (no study found) |
| DR-05 | Brief: sections in the order the reader would ask their questions; question headings over vague ones. | [#15] brief item 5 | HS (Federal guidelines pp. 6, 11) | primary | Survey P11 |
| DR-06 | Brief: an enumerated reference coverage inventory taken from the subject itself, never from memory. | [#15] brief item 6 | HS (survey brief; Diátaxis reference; TC39) | primary | Survey P4; → PR-02 |
| DR-07 | Brief: a diagram inventory by level for `art` — level, question, kind, renderer — starting from one overview. | [#15] brief item 7 | HS (C4); RH (Moody; Shneiderman) | primary; Moody 2009 secondary | Survey P5; → X-23 |
| DR-08 | Brief: a named prose contract for `copy-edit` — keyword convention, voice rule with exceptions, term list, code typography, example labels. | [#15] brief item 8 | HS | primary | Survey P6; with CE-16 |
| DR-09 | Brief: the medium and format stated in the document contract. | [#15] brief item 9 | HS | — | Survey P7; → FM-01, FM-03 |
| DR-10 | For proposals and design documents: problem before design, costs in view — motivation, example-led then reference-level explanation reusing the examples, drawbacks, alternatives, prior art, unresolved questions. | [#15] draft check 1 | HS (Rust RFC, PEP 1, Swift, W3C TAG, Go 2 templates) | primary | Survey D3; with CV-04 |
| DR-11 | Separate what the problem requires, whatever the design, from what the design chose. | [#15] draft check 2 | HS (W3C TAG; Swift ABI section) | primary | Survey D2 |
| DR-12 | Make intent reachable from entry pages. | [#15] draft check 3 | CR-obs (Robillard 2009; Robillard and DeLine 2011) | primary | |
| DR-13 | Local context on every page: pages make sense read alone, and navigation alone is not enough. | [#15] draft check 4 | CR-obs (Meng et al. 2019); HS (DQTI ch. 9; Baker) | primary; sections; Baker contents | Survey P2, D5 |
| DR-14 | Short parts with comprehension checks. | [#15] draft check 5 | CR-obs (PLIERS); CR (Mayer segmenting) | primary | Survey D6; with PR-13 |
| DR-15 | Coherence: move anything that does not serve a page's purpose to the page whose purpose it serves. | [#15] draft check 6 | CR (Clark and Mayer ch. 9); HS (Diátaxis) | sections | Survey D8 |
| DR-16 | Technical truth against the subject: every normative statement traces to its rule; ambiguities become recorded feedback, not quiet fixes. | [#15] draft check 7 | HS | — | Survey D1 |
| DR-17 | Label every example complete or fragment, and say what a fragment omits. | [#15] examples bullet 1 | HS | — | Survey D4 |
| DR-18 | At least two examples per concept; vary surface features; label subgoals; integrate text and diagrams. | [#15] examples bullet 2 | CR (Atkinson et al. 2000) | primary | Survey D4 |
| DR-19 | Compare two cases, then state the principle, then name where the comparison breaks. | [#15] examples bullet 3 | CR (Alfieri et al., d = .50; Gentner et al.; Spiro et al.) | primary | Survey D7, §5; with CV-03 |
| DR-20 | Small pattern-showing examples beat single calls and tangled programs; mark examples' authority, since readers take them as recommendations. | [#15] examples bullet 4 | CR-obs (Robillard and DeLine; Uddin and Robillard) | primary | |
| DR-21 | Explanatory samples need not run but must be explained: say why, state limits, mark alpha or beta features. | [#15] examples bullet 5 | HS (*Docs for Developers* ch. 5) | sections | |
| DR-22 | Pair positive examples with failures, one per block, each naming the rule that rejects it. | [#15] examples bullet 6 | HS (Rust Reference) | primary | Survey D4 |
| DR-23 | Where no tool exists, give expected observations checkable by inspection; never show invented tool output. | [#15] examples bullet 7 | HS | — | Survey D4 |
| DR-24 | Show the full authoring burden in complete examples when the reader's task includes judging it. | [#15] examples bullet 8 | HS | — | |
| DR-25 | Reference entries have fixed fields organised by the structure of the thing described. | [#15] Reference entries | HS (Microsoft; DQTI; CSS 2.1; man-pages(7); AARM) | primary; DQTI excerpt | Survey §4.5 application |
| DR-26 | Give rule identifiers explicit anchors that survive heading edits, and add "used by" and "see also" links against one-way hidden dependencies. | [#15] Reference entries | HS (Google headings; GitHub); RH (cognitive dimensions) | primary | Overlaps PR-03 |
| DR-27 | Tension: Diátaxis minimises explanation in tutorials; Google and Meng integrate it — resolution offered: pre-train concepts, then a one-line reason and link inside the tutorial. | [#15] tension row 1 | HS vs CR-obs | primary | Survey §5 row 1 |
| DR-28 | Tension: separate by type vs integrate — resolution offered: pure purpose per page, navigation by journey and subject, local context everywhere. | [#15] tension row 2 | HS vs CR-obs | primary | Survey §5 row 2 |
| DR-29 | Tension: linear overviews vs page-one fragments — resolution offered: linear tutorial and explanation, page-one reference entries. | [#15] tension row 3 | CR-obs vs HS | primary; Baker contents | Survey §5 row 3 |
| DR-30 | Tension: complete examples vs deliberate gaps — resolution offered: complete canonical examples, gaps only as labelled exercises. | [#15] tension row 4 | CR-obs vs CR | primary | Survey §5 row 4 |
| DR-31 | Tension: redundancy helps novices, hurts experts — resolution offered: repeat what is new to the reader, never re-explain what they know. | [#15] tension row 5 | CR | primary | Survey §5 row 5; → DR-46 |
| DR-32 | Tension: Diátaxis's generic modes vs Baker's subject-specific types — resolution offered: modes set purpose, subject templates set shape. | [#15] tension row 6 | HS vs HS | primary; Baker contents | Survey §5 row 6 |
| DR-33 | Establish the actor, action, object and consequence in the local explanation before using an abstract label. | [#15 k8 comment][i15-k8], block 4, bullet 1 | RP | checkpoint RC-1 | ← DR-03; with CE-17 |
| DR-34 | Give concepts the space they need; reconsider brief limits such as "two sentences" or "one-line rationale" when they prevent the required explanation. | [#15 k8 comment][i15-k8], block 4, bullet 2 | RP | checkpoint RC-1 | ← DR-27's "one-line reason" |
| DR-35 | Name the subject positively in headings and describe each case's actual behaviour, preserving conditions, limits and exact obligations. | [#15 k8 comment][i15-k8], block 4, bullet 3 | RP | checkpoint RC-1 | With CE-18 |
| DR-36 | Check page openings, summaries and tables as well as main sections; glossary links help revisit concepts prose has taught. | [#15 k8 comment][i15-k8], block 4, bullet 4 | RP | checkpoint RC-1 | With CE-19 |
| DR-37 | Preserve complete examples for inspection, but make each tutorial part's new concept or change directly reachable rather than hidden behind repeated introductions. | [#15 k8 comment][i15-k8], block 5 | MP | checkpoint RC-1 (proxy) | With AR-12 |
| DR-38 | Establish the containing model before its notation (for AIL: package, declaration, imported package or alias, callable and slot, member association). | [#15 k48 comment][i15-k48], block 4, bullet 1 | RP | checkpoint RC-2 | → DR-44 |
| DR-39 | At each substantive introduction, put a small explained excerpt in its enclosing context, using the same concrete names in prose, code and diagrams. | [#15 k48 comment][i15-k48], block 4, bullet 2 | RP | checkpoint RC-2 | → DR-46 (narrowed: no token narration) |
| DR-40 | For a viewpoint-relative term such as "local", identify whose perspective applies; show related entities and the clause connecting them. | [#15 k48 comment][i15-k48], block 4, bullet 3 | RP | checkpoint RC-2 | With CE-21 |
| DR-41 | Distinguish reference kinds (declaration or slot references versus local places) by showing their uses, not a token list. | [#15 k48 comment][i15-k48], block 4, bullet 4 | RP | checkpoint RC-2 | |
| DR-42 | Preserve complete examples while providing local excerpts where the reader meets each new construct. | [#15 k48 comment][i15-k48], block 4, bullet 5 | RP | checkpoint RC-2 | ← DR-30; with DR-37 |
| DR-43 | Give direct entries into normative pages teaching support, clearly separated from the preserved requirements. | [#15 k48 comment][i15-k48], block 4, bullet 6 | RP | checkpoint RC-2 | With X-23, SV-05, X-24 |
| DR-44 | Explain the subject model first and describe the notation by reference to it; the model and the reader's task determine the learning sequence. | [#15 k64 comment][i15-k64], blocks 2–4 | RP | checkpoint RC-3 | ← DR-38, DR-39 |
| DR-45 | Brief check: each learning section has a model concept and practical task as its purpose, and its notation points back to that explanation; inventorying syntax examples or figures alone does not answer this. | [#15 k64 comment][i15-k64], block 5 | RP | checkpoint RC-3 | ← DR-07 |
| DR-46 | An explained example answers what this reader cannot already infer: check each explanatory sentence against the audience's knowledge and task, and remove token-by-token narration while keeping context and meaning. | [#15 k73 comment][i15-k73], blocks 3–4 | RP | checkpoint RC-4 | ← DR-39, CE-22; same as X-26 |
| DR-47 | Establish the intended domain breadth in the orientation or model before comparing alternatives, and evaluate adopting any one source's assumptions against it. | [#15 k12 comment][i15-k12], block 3 | RP | checkpoint RC-5 | Same as X-31 |
| DR-48 | Keep described library, description producer and consumer roles separate; a tentative preference must not silently exclude consumers or equate "interpreted" with "untyped". | [#15 k12 comment][i15-k12], block 3 | RP | checkpoint RC-5 | Same as X-32 |
| DR-49 | An ecosystem list establishes neither a common ABI nor working provider support; do not let it imply either. | [#15 k12 comment][i15-k12], block 3 | RP | checkpoint RC-5 | |

### CE — `copy-edit` ([#16])

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| CE-01 | One term, one concept; define terms where used; link a term's first use in a unit; flag collisions against the term-risk register. | [#16] test 1 | HS (Microsoft; Federal guidelines pp. 41–46; Python devguide) | primary; Microsoft and Federal quotations unverified | Survey C1; ← DR-04 |
| CE-02 | Conditions, goals and location before the action — in procedures and task steps only, never rewriting normative rule sentences to fit. | [#16] test 2 | HS (Google; Microsoft; DQTI) | primary; sections | Survey C2, §5 |
| CE-03 | Imperative in steps; indicative elsewhere. | [#16] test 3 | HS (Microsoft) | primary | Survey C3 |
| CE-04 | Follow a demonstrative with a noun where the referent could be unclear — a check, not a ban. | [#16] test 4 | HS (Google) | primary | Survey C4 |
| CE-05 | Active voice where the actor matters; passive or stative where it does not; what feels passive usually comes from abstract nouns and missing characters. | [#16] test 5 | HS (Google, Microsoft, DQTI, Federal guidelines); RH (Williams) | primary; sections; Williams excerpt | Survey C5; ← X-07 |
| CE-06 | Characters as subjects, actions as verbs, subject near verb, old before new — heuristics, not rules. | [#16] test 6 | RH (Gopen and Swan; Williams) | primary; Williams excerpt | Survey C9; with CE-17 |
| CE-07 | Front-load information-carrying words in headings, list items and paragraph openings; link text pairs a specialist term with a newcomer's gloss. | [#16] test 7 | CR-obs (Pernice; Nielsen); RH (Budiu, information scent) | primary | Survey C10 |
| CE-08 | Where MUST/SHOULD/MAY are used, state the BCP 14 convention; keep lowercase *should* out of normative sentences; plain-language rules govern the surrounding prose. | [#16] test 8 | HS (RFC 8174; Google) | primary | Survey C6, §5 |
| CE-09 | Code font for identifiers and file names; tagged fences as a project convention (guides disagree). | [#16] test 9 | HS (Google; Microsoft) | primary | Survey C7 |
| CE-10 | Figures of speech give way to mechanisms; an analogy states its basis and where it fails. | [#16] test 10 | HS (DQTI ch. 7); CR (Spiro et al.; Ambrose et al.) | sections; primary; excerpt | Survey C8 (also Grove's existing `copy-edit` charter) |
| CE-11 | No readability-score or voice-percentage gates. | [#16] test 11 | CR review (Redish 2000); HS (DfD ch. 9, contradicted) | primary; sections | Overlaps X-08 |
| CE-12 | Conflict: task headings as bare infinitive (Google) versus "To create…" (Microsoft). | [#16] conflicts bullet 1 | HS vs HS | unverified | Settled by CE-16 |
| CE-13 | Conflict: introducing a procedure with a fragment and colon — not recommended (Google) versus used (Microsoft). | [#16] conflicts bullet 2 | HS vs HS | unverified | Settled by CE-16 |
| CE-14 | Conflict: "Write like you speak" (Microsoft) versus "Don't try to write exactly the way you speak" (Google). | [#16] conflicts bullet 3 | HS vs HS | unverified | Settled by CE-16 |
| CE-15 | Conflict: brevity — "Shorter is always better" (Microsoft) versus redundancy that improves comprehension (Google) and brevity "not as a self-sufficient end" (Carroll and van der Meij). | [#16] conflicts bullet 4 | HS vs HS; CR (minimalism) | primary; two quotations unverified | Survey §5 brevity row; with DR-34 |
| CE-16 | Copy-edit applies a *named* prose contract: the document declares its default guide and the exceptions it takes. | [#16] conflicts, conclusion | HS | primary | ← X-07; with DR-08 |
| CE-17 | Practical test: after reading a passage, can the intended reader identify who acts, what they do and what follows? A synonym for an abstract noun leaves the gap. | [#16 k8 comment][i16-k8], block 2 | RP | checkpoint RC-1 | ← CE-06, CE-10; with DR-33 |
| CE-18 | Headings and openings state the subject, its operation and applicable conditions (positive framing), while exact prohibitions and rejection conditions stay recoverable from their rules. | [#16 k8 comment][i16-k8], block 3 | RP | checkpoint RC-1 | With DR-35 |
| CE-19 | Apply the test to headings, first sentences, table summaries, example labels and captions; sentence length, a glossary link or a model's paraphrase is not evidence of human understanding. | [#16 k8 comment][i16-k8], block 4 | RP; MP limit | checkpoint RC-1 | With DR-36, AR-13 |
| CE-20 | Where the missing material is an untaught relationship, route it to `draft` through the correction procedure rather than repair it in sentences. | [#16 k8 comment][i16-k8], block 5; [#16 k48 comment][i16-k48], block 4 | RP | checkpoints RC-1, RC-2 | ← X-22 |
| CE-21 | Check the prose–snippet relationship: can the reader identify the viewpoint that makes a name "local", what an alias addresses, which owner a member belongs to? A consistent label can still withhold that viewpoint. | [#16 k48 comment][i16-k48], block 2 | RP | checkpoint RC-2 | With DR-40 |
| CE-22 | Read introductions, code labels, captions and tables together; the same example names refer to the same entities; each newly introduced token is explained in its context. | [#16 k48 comment][i16-k48], block 3 | RP | checkpoint RC-2 | → X-27, DR-46 (narrowed: explain meaning, not familiar tokens) |
| CE-23 | In canonical pages, edit only the labelled teaching additions and preserve the normative wording. | [#16 k48 comment][i16-k48], block 4 | RP | checkpoint RC-2 | With SV-05 |

### AR — `art` ([#17])

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| AR-01 | Figures run from one overview down, level by level, with only the levels that add value; reuse the overview as the map on every page and give each diagram "zooms into" pointers. | [#17] check 1; [#9 comment] §1 bullet 3 | RH (Shneiderman; Moody); HS (C4) | primary; Moody 2009 secondary | Survey A3; → AR-12 (overview on every page becomes a link back) |
| AR-02 | Each level says what it omits, so an absent line never reads as an absent relation. | [#17] check 2; [#9 comment] §1 bullet 4 | HS (Clements et al., descriptive completeness) | sections | Survey A3, §5 |
| AR-03 | C4 above for systems and ecosystems; sequence, state, timing and other kinds below for contracts and behaviour (C4 itself points libraries to UML). | [#17] check 3; [#9 comment] §1 bullet 2 | HS (C4 FAQ) | primary | |
| AR-04 | Every diagram passes the C4 checklist whatever its notation: title with type and scope, a key for the notation, element types and descriptions, one-way arrows with specific verbs, consistent colour. | [#17] check 4; [#9 comment] §1 bullet 1 | HS (C4; Clements et al. rule 3a) | primary; sections; "colour-blind-safe" and "rather than 'Uses'" unverified | Survey A7; → AR-17 (the key identifies conventions; notation teaching depends on the reader) |
| AR-05 | One visual vocabulary across renderers — one symbol and colour per concept, the prose's names, one legend convention — and a new diagram kind only when the question changes. | [#17] check 5; [#9 comment] §1 bullet 5 | CR review (Ainsworth); RH (Moody; Petre) | primary | Survey A8, §5 |
| AR-06 | Prose carries every fact (dual coding); describe the image in body text, not only alt text; "the key and a line on how to read it are part of the figure". | [#17] check 6 | HS (Google; Microsoft; DfD ch. 6); RH (Moody); CR (Larkin and Simon) | primary; sections | Survey A2; → AR-17, AR-19 (how-to-read conditional; no punctuation narration) |
| AR-07 | Labels on the element, annotations beside the line (spatial contiguity). | [#17] check 7 | CR (Mayer 2014; Mayer 2008, "d = 1.12"); RH (Moody) | primary; the d = 1.12 figure unverified | Survey A6 |
| AR-08 | A clear, identifiable starting point for reading each diagram. | [#17] check 8 | HS (DfD ch. 6; Grove's `visual-design-discussion.md` "Start here") | sections | |
| AR-09 | Simple line drawings, not decorative detail. | [#17] check 9 | CR (Clark and Mayer ch. 9) | sections | |
| AR-10 | Commit source and SVG together, regenerated from source, recording tool version and command. | [#17] check 10 | RP (the human's 2026-09-23 decision) | — | Survey A4; root brief exempts figures in the page's own bytes |
| AR-11 | Pin tool versions and regenerate and compare in a repeatable check, treating tool-upgrade drift as expected. | [#17] "Drift and accessibility" | MC (observed drift, CF-17–CF-19) | primary | With FM-06 |
| AR-12 | Treat overview-to-detail ordering across the route separately from repeating a full overview on every page; a link back may orient, and full reuse must earn its space. | [#17 k8 comment][i17-k8], block 3 | MP; RP | checkpoint RC-1 | ← AR-01 |
| AR-13 | Apply the prose contract to diagram titles, element labels, arrows, keys, alt text and captions; inspect them with the adjacent prose. | [#17 k8 comment][i17-k8], block 4 | RP | checkpoint RC-1 | With CE-19 |
| AR-14 | Keep source/SVG agreement, visual review and reader understanding as separate evidence. | [#17 k8 comment][i17-k8], block 5 | MC; RP | checkpoint RC-1 | With PR-24, PR-28 |
| AR-15 | Choose among a model view (what the entities are and how they relate), a grammar or railroad view (how a form is assembled) and a concrete snippet, with names aligned across all three. | [#17 k48 comment][i17-k48], block 4, bullets 1–3 | RP | checkpoint RC-2 | → AR-19 (placement) |
| AR-16 | Railroad keys identify literals, nonterminals, alternatives, optional parts and repetition; each path is checked against its named grammar production; semantic restrictions are explained separately with rule links. | [#17 k48 comment][i17-k48], block 5 | RP; MC | checkpoint RC-2 | → AR-17 (no reading instruction for a fluent reader) |
| AR-17 | Make generic "how to read this notation" instruction conditional on the declared reader; a concise legend still identifies the chosen drawing conventions. | [#17 k64 comment][i17-k64], blocks 4–5 | RP | checkpoint RC-3 | ← AR-04, AR-06, AR-16; root brief requirement |
| AR-18 | A figure's role is to explain a model relationship or its expression in the notation, not the notation itself when the reader knows it. | [#17 k64 comment][i17-k64], block 8 | RP | checkpoint RC-3 | With DR-44 |
| AR-19 | Choose placement by the question a figure answers and the reader's fluency; a useful figure may belong on a lookup page; "text carries the figure's information" does not require narrating a railroad's punctuation when the textual grammar is provided. | [#17 k73 comment][i17-k73], block 4 | RP | checkpoint RC-4 | ← AR-06, AR-15; same as X-28 |

### PR — `proof` and reader testing ([#18])

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| PR-01 | Walk the reader journey end to end from every entry page, not only in reading order. | [#18] proof check 1 | HS (Diátaxis; Baker); CR-obs (Meng et al. 2019) | primary; Baker contents | Survey R1; → PR-22, PR-27 |
| PR-02 | Check reference coverage against the enumerated inventory from the subject. | [#18] proof check 2 | HS | — | Survey R2; ← DR-06 |
| PR-03 | Check links, anchors, code fences and tables mechanically, and that heading-derived anchors survived rewording. | [#18] proof check 3 | MC; HS (Google headings) | primary | Survey R3; with DR-26 |
| PR-04 | Run the C4 review checklist on every diagram and audit cross-diagram consistency of symbols, names and reading order. | [#18] proof check 4; [#9 comment] §1 bullet 1 | HS (C4); RH (Moody) | primary | ← AR-04; → AR-17 |
| PR-05 | Status statements agree everywhere, and terms are used consistently across pages. | [#18] proof check 5 | HS | — | Survey R4, R5 |
| PR-06 | A headings-and-first-sentences read: do they tell a story? | [#18] proof check 6 | HS (Microsoft final review) | primary; exact quotation unverified | |
| PR-07 | A cold re-read after a time gap. | [#18] proof check 7 | RH (Pinker; "argument, not tested") | primary | |
| PR-08 | Reader testing is a documented activity outside the four stages, scheduled by planning, because writers cannot judge difficulty. | [#18] reader testing, "Why it is needed" | CR (Camerer; Hinds; Schriver abstract); HS (Clements et al. rule 7) | primary; abstract; sections | Survey T1; ← X-06; root brief seam 3 |
| PR-09 | Measure outcomes, not liking: task accuracy moved while satisfaction did not. | [#18] reader testing, bullet 4 | CR (Meng et al. 2020) | primary | |
| PR-10 | Paraphrase-test key terms and messages; task tests in rounds of about three; plan to test at least twice — guideline numbers, not validated thresholds. | [#18] methods bullet 1 | HS (Federal guidelines) | primary; page range differs from survey | Survey T5 |
| PR-11 | Plus–minus marking for reactions and task-based tests for consulted documents; expect a second round to surface different problems. | [#18] methods bullet 2 | HS (Jarrett and Redish); CR (de Jong and Rijnks, abstract) | primary; abstract | Survey T5 |
| PR-12 | Count specific criticisms, using agreement across independent readers as the validity signal. | [#18] methods bullet 3 | CR-obs (Blackwell and Green 2000 pilot) | primary | Survey T3 |
| PR-13 | Comprehension checks at the end of tutorial parts. | [#18] methods bullet 4 | CR-obs (PLIERS) | primary | Survey T2; with DR-14 |
| PR-14 | Ask the reviewer's question — could I find, understand and use this? — in the context users will see. | [#18] methods bullet 5 | HS (DQTI ch. 13) | sections | With FM-05 |
| PR-15 | A model-based reader is a labelled proxy, never evidence of human comprehension. | [#18] final paragraph | — (validity rule) | — | Survey T4; root brief requirement |
| PR-16 | Record an interrupted human read and a successful model read as separate outcomes; the model's success does not turn the human's into a passed checkpoint. | [#18 k8 comment][i18-k8], block 4 | HA/RP; MP | checkpoint RC-1 | With PR-28 |
| PR-17 | Let an early comprehension failure stop the questionnaire, and record the reader's words and how far they got. | [#18 k8 comment][i18-k8], block 6, bullet 1 | RP | checkpoint RC-1 | |
| PR-18 | Route the correction through the stages that own it, then schedule another human read before dependent document work. | [#18 k8 comment][i18-k8], block 6, bullet 2 | RP | checkpoint RC-1 | With CE-20 |
| PR-19 | Begin the follow-up read at the repaired entry explanation and extend the path as the human can proceed. | [#18 k8 comment][i18-k8], block 6, bullet 3 | RP | checkpoint RC-1 | → PR-23 |
| PR-20 | During whole-document proof, check compressed relationships and negative framing in headings, openings, tables and captions, keeping the human test as separate evidence. | [#18 k8 comment][i18-k8], block 6, bullet 4 | RP | checkpoint RC-1 | With CE-19 |
| PR-21 | Record the exact locations the reader identifies, and leave unreported coverage unknown. | [#18 k48 comment][i18-k48], block 5, bullet 1 | RP | checkpoint RC-2 | → PR-27 |
| PR-22 | Follow rule links as real entry points, and inspect whether model context and examples are available where new notation appears. | [#18 k48 comment][i18-k48], block 5, bullet 2 | RP | checkpoint RC-2 | ← PR-01; with X-23 |
| PR-23 | Start the next read at the reported difficulty and its new aids, then extend the route from the human's response. | [#18 k48 comment][i18-k48], block 5, bullet 3 | RP | checkpoint RC-2 | ← PR-19 |
| PR-24 | Keep source/example consistency, grammar-to-railroad checks and rendered inspection separate from the human's understanding. | [#18 k48 comment][i18-k48], block 5, bullet 4 | MC; RP | checkpoint RC-2 | With AR-14 |
| PR-25 | When requesting a local VS Code read, specify opening the containing documentation folder or repository root before the preview. | [#18 k73 comment][i18-k73], block 5 | MC | checkpoint RC-4 | CF-40 |
| PR-26 | Verify embedded images in the actual reading context; link checks, SVG regeneration and standalone rendering establish different properties. | [#18 k73 comment][i18-k73], block 5 | MC | checkpoint RC-4 | With FM-05, X-29 |
| PR-27 | Record the exact entry surface — raw source, anchor, rendered page or index — and how context was recovered from it. | [#18 k12 comment][i18-k12], block 5 | MP; MC | checkpoint RC-5 | ← PR-01, PR-21 |
| PR-28 | Keep proxy success, mechanical reachability, human acceptance and demonstrated task performance as distinct evidence; do not infer that any stage caused a positive verdict. | [#18 k12 comment][i18-k12], block 5 | — (validity rule) | checkpoint RC-5 | Root brief requirement |
| PR-29 | Probes run in one accumulating model context are not independent cold reads; report them as such. | [#18 k12 comment][i18-k12], block 2 | MP limit | checkpoint RC-5 | Root brief requirement |

### FM — document format ([#19])

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| FM-01 | Format is a per-document-set decision made at planning, with the human, from evidence the skills supply. | [#19] Question; "Walk-away assessment", paragraph 2 | RP; HS | — | Survey P7; ← DR-09 |
| FM-02 | For a set reviewers read in a repository, the evidence favoured Markdown plus a deterministic committed generation step, and AsciiDoc when a published HTML or PDF edition is wanted. Recorded as the APIAnyware case's assessment. | [#19] "Walk-away assessment", paragraph 2 | MC (CF-21–CF-35); HS (Moir; DQTI ch. 13) | primary; sections | Context-specific; rests on facts k6 rechecks |
| FM-03 | Structure brief / document contract names the format, the admitted figure media and renderers, and any generation step with pinned tool versions. | [#19] implications bullet 1 | HS | — | ← DR-09 |
| FM-04 | `art`: the admitted medium bounds figures; record a declined figure whose medium could not carry it, including format limits such as GitHub not rendering PlantUML or D2 source. | [#19] implications bullet 2 | HS; MC | primary | Grove's existing `art` charter already records declined figures |
| FM-05 | `proof`: check the rendering readers will actually see (for AsciiDoc on GitHub, the secure-mode preview, not a local build). | [#19] implications bullet 3 | MC; HS (DQTI ch. 13) | primary; sections | With PR-14, PR-26 |
| FM-06 | Generated artifacts: commit outputs, regenerate and compare in a repeatable check, and treat drift after tool upgrades as expected rather than content change. | [#19] implications bullet 4 | MC | primary | With AR-11 |

### CV — requirements and design conversations ([#9 comment] §2)

The root brief paraphrases this section as five practices; the comment has six
bullets (the first two are merged there), and each gets its own ID.

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| CV-01 | Model what the listener already knows: audience is distance from the knowledge, and guidance that helps novices can hinder experts. | [#9 comment] §2 bullet 1 | HS (Google); CR (Kalyuga et al.) | primary | ← X-05; with X-25 |
| CV-02 | Introduce names before mechanisms: fix the vocabulary before the trade-off. | [#9 comment] §2 bullet 2 | CR (Mayer 2014) | primary | ← X-10 |
| CV-03 | Compare two options, then state the principle, then say where the comparison breaks — extending the spine's "name the trade-off, recommend, give evidence". | [#9 comment] §2 bullet 3 | CR (Alfieri et al.; Spiro et al.) | primary | ← DR-19 |
| CV-04 | Problem before solution, costs in view: proposal genres serve design conversations as well as documents. | [#9 comment] §2 bullet 4 | HS (Rust RFC; W3C TAG) | primary | ← DR-10 |
| CV-05 | Don't trust your own sense of clarity: when stakes are high, ask the human to restate the decision in their own words. | [#9 comment] §2 bullet 5 | CR (Hinds); HS (Federal guidelines, paraphrase testing) | primary | ← X-06, PR-10 |
| CV-06 | Sketch in a simpler diagram dialect for conversation than for final diagrams; a whiteboard for design sessions; disciplined secondary notation in stakeholder diagrams. | [#9 comment] §2 bullet 6 | RH (Moody; Petre); HS (C4 tooling page) | primary; Moody 2009 secondary | ← AR-05 |

### DS — design reference ([#9 comment] §1)

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| DS-01 | Update `visual-design-discussion.md`'s Structurizr recommendation for the 2025–26 product changes (Lite, CLI and on-premises end-of-life; cloud end on 30 September 2026; export without D2). | [#9 comment] §1 bullet 6 | MC (dated facts CF-06–CF-09) | primary | Depends on k6's recheck; see also CF-10, CF-11 |

### WU — writing-for-understanding evidence cited from #12 ([#12 comment][i12-k64])

| ID | Proposal | Source | Class | Depth | Relations |
|---|---|---|---|---|---|
| WU-01 | Familiarity with a representation and familiarity with the subject are separate assumptions; a reader may follow a railroad yet still need the subject model and explained examples. | [#12 comment][i12-k64], block 3 | RP | checkpoint RC-3 | ← X-05, DR-02; with X-25 |
| WU-02 | Evaluation case: supply both kinds of prior knowledge explicitly, and check whether the answer develops the missing subject knowledge while preserving the example's technical detail. | [#12 comment][i12-k64], block 4 | — (evaluation design) | checkpoint RC-3 | For the preregistration |
| WU-03 | Evaluation distinction beyond sentence clarity: can the reader understand the subject model and use the notation to express it? | [#12 comment][i12-k64], block 7 | — (evaluation design) | checkpoint RC-3 | With DR-44; for the preregistration |

### SV — survey items no issue carries

| ID | Proposal | Survey location | Class | Depth | Relations |
|---|---|---|---|---|---|
| SV-01 | Plan small increments that each land with working navigation and no empty sections. | [§9](documentation-basis-a.md#9-stage-scoped-acceptance-criteria) P8; [§5](documentation-basis-a.md#5-tensions-between-sources) "Guide or plan" | HS (Diátaxis workflow) | primary | |
| SV-02 | Meaning never rests on colour alone, and tables have no merged cells. | §9 A5 | HS (Microsoft; Google) | primary | Partly in AR-04's "colour-blind-safe" |
| SV-03 | Give authors and editors the readers' think-aloud transcripts before they revise; do not rely on authors' own estimates of difficulty. | §9 T6 | CR (Schriver, abstract; Hinds) | abstract; primary | ← X-06 |
| SV-04 | Use at most two levels of disclosure, and never hide what a reader needs to judge a rule behind a link alone. | [§4.2](documentation-basis-a.md#42-how-the-four-forms-cooperate) application | CR-obs (Nielsen 2006; Nassif and Robillard 2023) | primary | |
| SV-05 | Keep normative rules in one place; other pages restate a rule only as a labelled, non-normative summary linked to its identifier (annotations beside the rules, as in the AARM, being the alternative to weigh). | §4.2 application; §5 "Self-contained pages or one rule source" | HS (Baker; AARM) | contents; primary | → X-24, DR-43, CE-23; root brief: normative text and aids distinguishable |
| SV-06 | State draft rules authoritatively, keep an open-question register, and never soften a MUST. | §5 "Authoritative reference or honest draft" | HS (Diátaxis; proposal genres) | primary | With DR-10 |
| SV-07 | Include modification tasks (for example, changing one property and tracing every clause it then requires): the effort of change and hidden dependencies are review targets. | [§4.3](documentation-basis-a.md#43-examples) application | CR (Blackwell and Green 2003) | primary | |
| SV-08 | Where a design history exists, show a before-and-after pair. | §4.3 application | HS (Go 2 template) | primary | |
| SV-09 | Provide lookup indexes: alphabetical, by kind, by rule identifier, and from failure cases to rules. | [§4.5](documentation-basis-a.md#45-navigation-reference-entries-glossary-tables-and-diagrams) application | HS | — | With DR-25 |
| SV-10 | A notation page explains how to read the grammar notation before use. | §4.5 application | HS (Python, Go and Rust define their EBNF first; DQTI p. 530) | primary; excerpt | → AR-17 applies the same condition: only for readers who do not know the notation |
| SV-11 | Keep tables narrow enough to read as raw markup, since reviews happen on raw source. | [§8.3](documentation-basis-a.md#83-the-walk-away-check-for-this-repository) | HS (Moir 2024) | primary | Survey §10: no study of tables' effect on comprehension |

## Where the issues go beyond the survey

The issues restate the survey but are not a copy of it. Checking every
quotation in the issue bodies and the #9 comment against the survey text found
these differences. Each one's depth is recorded here as **unverified**. The
source is not thereby wrong, but no recorded inspection stands behind this
exact wording or number, and a later leaf that relies on it should fetch it.

| Item | In the issue | In the survey |
|---|---|---|
| AR-07 | Spatial contiguity "Mayer 2008, d = 1.12" | Mayer 2008 cited; no effect size given |
| AR-04 | C4 checklist includes "consistent, colour-blind-safe colour" and arrows labelled "rather than 'Uses'" | Neither phrase |
| PR-10 | Federal guidelines pp. 100–108 | pp. 94–108 |
| DR-01 | Federal guidelines p. 3: documents that mix audiences confuse readers | Not cited |
| CE-01 | Microsoft "Aim for one term, one concept"; Federal guidelines "Never define a word to mean something other than its commonly accepted meaning", pp. 41–46 | Microsoft paraphrased as "one term, one concept"; the Federal quotation is absent |
| CE-02 | Microsoft "If there's a chance of confusion" | Paraphrased as "location first where confusion is likely" |
| CE-12–CE-15 | Google/Microsoft quotations on task headings, procedure introductions, "Write like you speak", redundancy, and Carroll and van der Meij's "not as a self-sufficient end" | Only the brevity row, with "Shorter is always better" |
| PR-06 | Microsoft's full "Read only the headings, and then only the first sentence of every paragraph. Do they tell a story?" | Paraphrased |
| PR-08 (its evidence bullet 5) | Redish "Even testing with a few people is much more informative than a readability score" | Quoted without "than a readability score" |

The survey's §2 reports Google's and Microsoft's pages read in full, so those
quotations are probably from inspected pages; this document does not assume so.

**CE-12–CE-15, rechecked 2026-09-30.** The house-style design fetched the
guides' current pages because it takes a side on each conflict. Google's
headings, procedures, voice-and-tone and global-audience pages, and Microsoft's
headings, step-by-step and top-10 pages, confirm all four conflicts. Microsoft
is itself mixed on task headings: its headings page suggests "To create a
heading", while its step-by-step page heads an example "Create a profile".
Carroll and van der Meij's "not as a self-sufficient end" was not fetched and
stays unverified. The quotations, their pages and their dates are in
[`documentation-guidance`](../specs/documentation-guidance.md) section 9.

## Dated capability facts

These are the tool, format and host facts that guidance could depend on.
Unless marked otherwise they were **checked on 2026-09-23** and are stated as
of that date. The leaf `documentation-toolchain-k6` rechecks them before any
guidance relies on them; nothing here is current advice. That recheck, as of
2026-09-30, is [`documentation-toolchain-a.md`](documentation-toolchain-a.md#1-status-of-the-2026-09-23-facts),
which marks each fact unchanged, changed or unverifiable. Note that one fact,
Structurizr cloud's end of life (CF-08), falls on the day after this document
was written.

### Diagram tools

| ID | Fact as of 2026-09-23 | Source |
|---|---|---|
| CF-01 | Structurizr (consolidated, Apache-2.0): one model, many views, C4 levels; a generated key; SVG through headless Chromium export or export to PlantUML/Mermaid then render; DSL text readable, manual layout stored only in JSON. | [#17] toolchain table; survey [§8.2](documentation-basis-a.md#82-diagram-toolchains) |
| CF-02 | C4-PlantUML (MIT, v2.14.0): context, container, component, dynamic and deployment diagrams; `SHOW_LEGEND()`; SVG via PlantUML (`-tsvg`/`--svg`), needing Java and Graphviz for several diagram types (or the experimental Smetana layout); in PlantUML's standard library, offline. | [#17] table; survey §8.2 |
| CF-03 | Mermaid C4 (Mermaid v12): five C4 diagram types, experimental; no legend support; SVG via `mmdc` (Node and headless Chromium); GitHub renders Mermaid natively; syntax may change. | [#17] table; [#9 comment] §1; survey §8.2 |
| CF-04 | D2 (MPL-2.0, v0.9.0): C4 theme and `c4-person` shape since v0.7; `layers` and `link` for zoom; `d2-legend`; native SVG output by default; layouts dagre, ELK and TALA. | [#17] table; [#9 comment] §1; survey §8.2 |
| CF-05 | LikeC4 (MIT): C4-inspired, any number of levels; generates D2, PlantUML, Mermaid, Dot and DrawIO; no documented SVG export; experimental notations. | [#17] table; [#9 comment] §1; survey §8.2 |
| CF-06 | Structurizr Lite, CLI and on-premises are end-of-life, with archived repositories; one Apache-2.0 application (`structurizr/structurizr`) replaces them. | [#17]; [#9 comment] §1 |
| CF-07 | Structurizr's `export` no longer offers D2 (nor DOT or Ilograph); it produces PlantUML, C4-PlantUML, Mermaid, WebSequenceDiagrams, a static site, and PNG/SVG via headless Chromium. | [#17]; [#9 comment] §1 |
| CF-08 | The Structurizr cloud service reaches end of life on 30 September 2026. | [#17]; [#9 comment] §1 |
| CF-09 | Structurizr needs Java 21 or Docker; the survey machine's Java was 16, so it was not usable as installed. | survey §8.2 |
| CF-10 | Asciidoctor Diagram renders PlantUML, Mermaid, D2, Structurizr and other notations at build time with local tools. | [#17] "Other routes" |
| CF-11 | Kroki offers a unified HTTP rendering service that can be self-hosted; its public instance receives the diagram source, and no privacy statement was found. | [#17] "Other routes"; survey §8.2 |
| CF-12 | GitHub runs neither Asciidoctor Diagram nor Kroki, so such blocks appear there as source. | [#17] "Other routes" |
| CF-13 | Typst has native diagram packages (fletcher, CeTZ, diagraph, pintorita, chronos, digidraw, Mermaid re-implementations such as merman and mmdr) but no PlantUML, D2 or C4 package. | [#17] "Other routes"; survey §8.2 |
| CF-14 | Mermaid SVGs that use HTML labels lose their text when embedded in Typst (typst#1421). | [#17] "Other routes"; survey §8.1 |
| CF-15 | PlantUML SVGs embed their source by default; `--extract-source` recovers it. | [#17] check 10; survey §8.2 |
| CF-16 | D2 SVGs carry injected CSS that viewers may render differently. | [#17] check 10 |
| CF-17 | Mermaid 12 re-lays out existing diagrams. | [#17] "Drift and accessibility"; survey §8.2 |
| CF-18 | D2 0.9.0 writes smaller SVG files than before. | as CF-17 |
| CF-19 | TALA layouts vary with small edits; TALA has been open source since 2026-09-07. | as CF-17 |
| CF-20 | Only Mermaid documents accessible titles and descriptions (`accTitle`, `accDescr`); no accessibility documentation was found for PlantUML or D2. | [#17]; survey §8.2 |

### Document formats and hosts

| ID | Fact as of 2026-09-23 | Source |
|---|---|---|
| CF-21 | On GitHub: GFM renders in full (alerts, footnotes, native Mermaid, SVG images); AsciiDoc is a secure-mode preview in which includes become links and diagram blocks show source, while TOC, cross-references, callouts and attributes work; DocBook shows as XML source; Typst as source, with committed PDFs viewable but their links ignored. | [#19] table row 1; survey [§8.1](documentation-basis-a.md#81-document-formats) |
| CF-22 | Single-sourcing code examples: Markdown has none (needs a generator and committed output); AsciiDoc has tagged-region includes, disabled on GitHub; DocBook has XInclude or external text; Typst has `read()` into `raw()`, without tagged regions. | [#19] row 2; survey §8.1 |
| CF-23 | Line annotations: none in Markdown; numbered callouts in AsciiDoc; coordinates or inline callouts in DocBook; packages keyed to line numbers in Typst. | [#19] row 3 |
| CF-24 | Admonitions: five fixed-title alert types in GFM; five types with configurable captions in AsciiDoc; a fixed set plus remarks in DocBook; any, via packages or functions, in Typst. | [#19] row 4 |
| CF-25 | Stable cross-references: GFM heading anchors change with the text and explicit anchors are unchecked; AsciiDoc has explicit IDs with invalid references warned under `-v`; DocBook has `xml:id`/`xref`, Schematron-checkable; Typst has labels. | [#19] row 5 |
| CF-26 | Glossary and index: none in GFM; AsciiDoc glossary lists, with an index only in PDF or DocBook output; full support in DocBook; packages in Typst. | [#19] row 6 |
| CF-27 | Outputs: GFM, HTML through the host; AsciiDoc (Asciidoctor 2.0.26), HTML5, DocBook and man with PDF and EPUB add-ons; DocBook 5.2, HTML5 (xslTNG) and print; Typst 0.15.1, PDF, PNG and SVG with experimental HTML. | [#19] row 7 |
| CF-28 | Raw legibility, as judged by the survey: GFM highest, AsciiDoc high, Typst medium, DocBook lowest. (A judgement, not a measurement.) | [#19] row 8 |
| CF-29 | GFM is a strict superset of CommonMark adding tables, task lists, strikethrough, autolinks and disallowed raw HTML. | [#19] evidence bullet 1 |
| CF-30 | GitHub Linguist will not add a language without at least 2000 files per extension, so a new language's code stays unhighlighted wherever GitHub renders it. | [#19] evidence bullet 1 |
| CF-31 | GitHub processes AsciiDoc in `SECURE` mode (includes become links); Asciidoctor's lead says there is no GitHub-flavored AsciiDoc; Asciidoctor Reducer is the official workaround; until the AsciiDoc Language Specification is ratified, the Asciidoctor implementation defines AsciiDoc. | [#19] evidence bullet 2 |
| CF-32 | OASIS DocBook development ended with the Technical Committee's closure in September 2024; the XSLT 1.0 stylesheets were last released in 2016; xslTNG needs Java and Saxon. | [#19] evidence bullet 3 |
| CF-33 | Typst's HTML export is "very incomplete" and not for production; SVG and PNG export turn text into shapes that screen readers cannot read; packages are pinned and immutable once published. | [#19] evidence bullet 4 |
| CF-34 | Reviewing documentation in a repository means reading raw markup in pull requests. | [#19] evidence bullet 5 (Moir 2024) |
| CF-35 | GitHub renders neither Typst nor DocBook. | survey §8.1 |

### Local installation on the survey machine

Version checks only, on 2026-09-23, on the machine the survey ran on; they say
nothing about another machine.

| ID | Fact as of 2026-09-23 | Source |
|---|---|---|
| CF-36 | PlantUML 1.2026.8 with Graphviz 16.1.0 installed. | survey §8.2 |
| CF-37 | D2 0.9.0 installed. | survey §8.2 |
| CF-38 | Mermaid CLI not installed; Typst 0.15.1 installed. | survey §8.2 |
| CF-39 | Asciidoctor and Asciidoctor PDF installed. | survey §8.1 |

### Viewing environment, checked 2026-09-29

| ID | Fact as of 2026-09-29 | Source |
|---|---|---|
| CF-40 | VS Code's Markdown preview of a standalone file (no folder opened) limits local resources to the file's directory, so sibling-directory SVGs fail (HTTP 401 in the reproduction); opening the containing folder restores them, with Restricted Mode still enabled. The upstream resource-root implementation matched the installed extension. | [#18 k73 comment][i18-k73] (RC-4) |

## Enumeration ledger

Every unit of every in-scope source, in source order, with its classification.
"→" names the ID a unit became or is recorded against.

**[#14] body.** Intent: framing paragraph (provenance); bullets 1–4 → X-01–X-04;
stage-issue bullets 1–5 (pointers); "Related" line (pointer). "Where the
evidence comes from" (provenance). Principles 1–6 → X-05–X-10. Partition
rows 1–6 → X-11–X-16. "How to evaluate" → X-17. Cautions bullets 1–3 →
X-18–X-20. Books paragraph → X-21.

**[#14 k8 comment][i14-k8].** Blocks 1–5 (RC-1 evidence); block 6 → X-22, and
evidence for PR-16; block 7 (local disposition; its three requirements are
carried as DR-33, DR-35, CE-18); block 8 (pointers and RC-1 limits).

**[#14 k48 comment][i14-k48].** Blocks 1–4 (RC-2 evidence); block 5 (RC-2
limits); block 6 (local disposition, carried as DR-38, DR-39); block 7 → X-23;
block 8 (limits).

**[#14 k64 comment][i14-k64].** Blocks 1–4 (RC-3 evidence); block 5 (local
disposition, → AR-17); block 6 (RC-3 limits); blocks 7–8 (RC-3 evidence,
→ DR-44); block 9 (RC-3 evidence, → X-24); block 10 (local disposition, → AR-17,
X-24, and limits).

**[#14 k73 comment][i14-k73].** Blocks 1–4 (RC-4 evidence); block 5 (the
reminder, → X-30); block 6 (refinement narrative, → DR-46); block 7 (table
introduction); block 8 rows 1–6 → X-25–X-30; block 9 (environment, → PR-25,
CF-40); block 10 (local disposition, no ID; evidence for X-26–X-28); block 11
(RC-4 limits).

**[#14 k12 comment][i14-k12].** Blocks 1–2 (RC-5 evidence); block 3 (RC-5
evidence; its "later work" sentence → X-33); block 4 → X-31, X-32, X-33;
block 5 (RC-5 limits).

**[#15] body.** Summary paragraphs 1–2 (framing). Brief items 1–9 →
DR-01–DR-09. Draft checks 1–7 → DR-10–DR-16. Examples bullets 1–8 →
DR-17–DR-24. Reference entries paragraph → DR-25, DR-26. Tension rows 1–6 →
DR-27–DR-32. Source (provenance).

**[#15 k8 comment][i15-k8].** Block 1 (RC-1 evidence); block 2 (worked
replacement, evidence for DR-33); block 3 (list introduction); block 4
bullets 1–4 → DR-33–DR-36; block 5 → DR-37; block 6 (limits).

**[#15 k48 comment][i15-k48].** Blocks 1–2 (RC-2 evidence); block 3 (list
introduction); block 4 bullets 1–6 → DR-38–DR-43; block 5 (local disposition
and limits).

**[#15 k64 comment][i15-k64].** Block 1 (RC-3 evidence); blocks 2–3 → DR-44;
block 4 (local disposition, → DR-44); block 5 → DR-45; block 6 (limits).

**[#15 k73 comment][i15-k73].** Blocks 1–2 (RC-4 evidence); blocks 3–4 →
DR-46; block 5 (local disposition and limits).

**[#15 k12 comment][i15-k12].** Blocks 1–2 (RC-5 evidence); block 3 → DR-47,
DR-48, DR-49; block 4 (local disposition, → X-33); block 5 (limits).

**[#16] body.** Summary (framing). Tests 1–11 → CE-01–CE-11. Conflicts
bullets 1–4 → CE-12–CE-15; conclusion → CE-16. Evidence notes bullet 1 (→
X-08, X-19); bullet 2 (→ X-18). Source (provenance).

**[#16 k8 comment][i16-k8].** Block 1 (RC-1 evidence); block 2 → CE-17;
block 3 → CE-18; block 4 → CE-19; block 5 → CE-20 (and RC-1 limits).

**[#16 k48 comment][i16-k48].** Block 1 (RC-2 evidence); block 2 → CE-21;
block 3 → CE-22; block 4 → CE-20, CE-23; block 5 (limits).

**[#17] body.** Summary (framing). Checks 1–10 → AR-01–AR-10 (check 10 also → CF-15, CF-16). Toolchain
rows 1–5 → CF-01–CF-05. Structurizr paragraph → CF-06–CF-08. "Other routes" →
CF-10–CF-14. "Drift and accessibility" → AR-11, CF-17–CF-20. Source
(provenance; licence → X-20).

**[#17 k8 comment][i17-k8].** Block 1 (pointer); block 2 (RC-1 evidence);
block 3 → AR-12; block 4 → AR-13; block 5 → AR-14.

**[#17 k48 comment][i17-k48].** Blocks 1–2 (RC-2 evidence); block 3 (list
introduction); block 4 bullets 1–3 → AR-15; block 5 → AR-16 (and local
disposition); block 6 (limits).

**[#17 k64 comment][i17-k64].** Blocks 1–3 (RC-3 evidence); blocks 4–5 →
AR-17; block 6 (RC-3 evidence, → X-24); block 7 (local disposition); block 8 →
AR-18.

**[#17 k73 comment][i17-k73].** Blocks 1–2 (RC-4 evidence); block 3 (local
disposition); block 4 → AR-19; block 5 (limits).

**[#18] body.** Summary (framing). Proof checks 1–7 → PR-01–PR-07. "Why it is
needed" bullets 1–6: bullets 1–3, 5, 6 are evidence for PR-08; bullet 4 →
PR-09. Methods bullets 1–5 → PR-10–PR-14. Model-proxy paragraph → PR-15.
Source (provenance).

**[#18 k8 comment][i18-k8].** Blocks 1–3 (RC-1 evidence); block 4 → PR-16;
block 5 (list introduction); block 6 bullets 1–4 → PR-17–PR-20; block 7 (local
disposition and limits).

**[#18 k48 comment][i18-k48].** Blocks 1–2 (RC-2 evidence); block 3 (RC-2
limits); block 4 (list introduction); block 5 bullets 1–4 → PR-21–PR-24;
block 6 (local disposition; the second-rerun escalation boundary is Grove's
existing bounded-correction rule operating, not a proposal); block 7 (limits).

**[#18 k73 comment][i18-k73].** Blocks 1–4 (RC-4 environment evidence, →
CF-40); block 5 → PR-25, PR-26; block 6 (limits).

**[#18 k12 comment][i18-k12].** Block 1 (heading line); block 2 (RC-5 proxy
evidence, → PR-29); block 3 (RC-5 navigation finding, evidence for PR-27);
block 4 (RC-5 human evidence); block 5 → PR-27, PR-28.

**[#19] body.** Question → FM-01. Comparison rows 1–8 → CF-21–CF-28.
Evidence bullets 1–5 → CF-29–CF-34. Walk-away paragraph 1 (assessment, evidence
for FM-02); paragraph 2 → FM-01, FM-02. Implications bullets 1–4 →
FM-03–FM-06. Source (provenance).

**[#9 comment] §1** (bounded). Intro paragraph (restates X-03 for the
requirements and design skills, → X-03, X-04). Bullets: 1 → AR-04, PR-04 and
CF-02–CF-04; 2 → AR-03; 3 → AR-01; 4 → AR-02; 5 → AR-05; 6 → DS-01 and
CF-05–CF-08; 7 → X-20.

**[#9 comment] §2.** Intro (framing, → X-04). Bullets 1–6 → CV-01–CV-06.

**[#12 comment][i12-k64].** Blocks 1–2 (RC-3 evidence); block 3 → WU-01;
block 4 → WU-02 (and local disposition); block 5 (limits); block 6 (RC-3
evidence, → DR-44); block 7 → WU-03 (and limits).

**Survey crosswalk.** §9: P1 → DR-01; P2 → DR-13, PR-01; P3 → DR-03, CE-01;
P4 → DR-06; P5 → DR-07; P6 → DR-08; P7 → DR-09, FM-01; P8 → SV-01; P9 → DR-04;
P10 → DR-03, DR-02; P11 → DR-01, DR-05. D1 → DR-16; D2 → DR-11; D3 → DR-10;
D4 → DR-17, DR-18, DR-22, DR-23; D5 → DR-13; D6 → DR-14; D7 → DR-19; D8 →
DR-15. C1 → CE-01; C2 → CE-02; C3 → CE-03; C4 → CE-04; C5 → CE-05; C6 →
CE-08; C7 → CE-09; C8 → CE-10; C9 → CE-06; C10 → CE-07. A1 (Grove's existing
`art` charter, no ID); A2 → AR-06; A3 → AR-01, AR-02; A4 → AR-10; A5 →
SV-02; A6 → AR-07; A7 → AR-04; A8 → AR-05. R1 → PR-01; R2 → PR-02; R3 →
PR-03; R4 → PR-05; R5 → PR-05 (and the existing `proof` charter). T1 → PR-08;
T2 → PR-13; T3 → PR-12; T4 → PR-15; T5 → PR-10, PR-11; T6 → SV-03. The
closing paragraph → X-04, CV-01–CV-06.
§5 rows: explanation inside tutorials → DR-27; separate or integrate → DR-28;
linear or page one → DR-29; complete or incomplete examples → DR-30;
redundancy → DR-31; generic or subject types → DR-32; guide or plan → SV-01;
authoritative reference or honest draft → SV-06; "should" → CE-08; brevity →
CE-15; conditions before actions → CE-02; self-contained pages or one rule
source → SV-05; reading-level targets → X-08, CE-11; detail in overview
diagrams → AR-02; variety of notations → AR-05; similarity or difference
first → DR-19.
§4 "Application to AIL": §4.1 — orientation states what is evaluated, status
and wanted feedback → DR-10, PR-05; domain explanation before tutorial →
DR-03, DR-27; requirement versus choice → DR-11; familiar-term device →
DR-04; basics assumed → DR-02. §4.2 — compass as purpose test → DR-01;
navigation by journey and structure → DR-28; page-opening block → DR-13;
concepts before tutorial → DR-27; normative rules in one place → SV-05;
disclosure levels → SV-04. §4.3 — complete or fragment → DR-17; tutorial
checkpoints → DR-14; expected observations → DR-23; failure pairs → DR-22;
modification tasks → SV-07; full clause burden → DR-24; before-and-after pair
→ SV-08. §4.4 — inventory and reader tests → DR-06, PR-02, PR-15. §4.5 —
entry fields → DR-25; indexes → SV-09; anchors → DR-26; notation page →
SV-10; glossary → DR-04, CE-01; figures by level → AR-01, AR-03; one
vocabulary → AR-05.
§6 table rows map to CE-02–CE-09 and X-07 (Diátaxis separation → DR-01;
reading level → X-08); its product-documentation rows ("production-ready",
"get an API call working", timeless-feature rules) are AIL-specific
reinterpretations with no general proposal.
§8.3 rows: Markdown baseline and narrow tables → SV-11; heading anchors →
DR-26; diagram renderers → AR-10, AR-11; fenced text diagrams (the root
brief's own-bytes exception); AsciiDoc, Typst, DocBook, static sites →
FM-02; example extraction and link checks → FM-06; untagged-language fences →
CE-09.


## Where the proposals landed

Each proposal ID above has a disposition in
[`docs/specs/documentation-guidance.md`](../specs/documentation-guidance.md#dispositions):
adopted as a named rule with its owner and acceptance seam, narrowed with its
reason, or declined with its reason. The decisions that meet the ADR bar are:

- [`a-document-declares-its-contract`](../adr/a-document-declares-its-contract.md),
  from the structure-brief proposals (X-11, DR-01–DR-09, DR-44–DR-47, WU-01) and
  RC-1–RC-5, and for its house style from the style-guide proposals (X-07, X-24, X-27,
  CE-01–CE-05, CE-07–CE-09, CE-12–CE-16, SV-05) and the human's direction of
  2026-09-30;
- [`a-reader-checkpoint-is-a-requirements-leaf`](../adr/a-reader-checkpoint-is-a-requirements-leaf.md),
  from the reader-testing proposals (X-16, PR-08–PR-29);
- [`a-rendered-figure-ships-with-its-source`](../adr/a-rendered-figure-ships-with-its-source.md),
  from AR-10, AR-11 and FM-06, on the facts rechecked in
  [`documentation-toolchain-a.md`](documentation-toolchain-a.md).

[`the-editorial-pipeline-is-four-kinds`](../adr/the-editorial-pipeline-is-four-kinds.md)
records how the draft precondition grew.

[#14]: https://github.com/Linkuistics/grove/issues/14
[#15]: https://github.com/Linkuistics/grove/issues/15
[#16]: https://github.com/Linkuistics/grove/issues/16
[#17]: https://github.com/Linkuistics/grove/issues/17
[#18]: https://github.com/Linkuistics/grove/issues/18
[#19]: https://github.com/Linkuistics/grove/issues/19
[#9 comment]: https://github.com/Linkuistics/grove/issues/9#issuecomment-5789802715
[i12-k64]: https://github.com/Linkuistics/grove/issues/12#issuecomment-5869469009
[i14-k8]: https://github.com/Linkuistics/grove/issues/14#issuecomment-5794534445
[i14-k48]: https://github.com/Linkuistics/grove/issues/14#issuecomment-5864305374
[i14-k64]: https://github.com/Linkuistics/grove/issues/14#issuecomment-5869477737
[i14-k73]: https://github.com/Linkuistics/grove/issues/14#issuecomment-5885384609
[i14-k12]: https://github.com/Linkuistics/grove/issues/14#issuecomment-5890462651
[i15-k8]: https://github.com/Linkuistics/grove/issues/15#issuecomment-5794534758
[i15-k48]: https://github.com/Linkuistics/grove/issues/15#issuecomment-5864305698
[i15-k64]: https://github.com/Linkuistics/grove/issues/15#issuecomment-5883240495
[i15-k73]: https://github.com/Linkuistics/grove/issues/15#issuecomment-5885388298
[i15-k12]: https://github.com/Linkuistics/grove/issues/15#issuecomment-5890463091
[i16-k8]: https://github.com/Linkuistics/grove/issues/16#issuecomment-5794535032
[i16-k48]: https://github.com/Linkuistics/grove/issues/16#issuecomment-5864305985
[i17-k8]: https://github.com/Linkuistics/grove/issues/17#issuecomment-5794535422
[i17-k48]: https://github.com/Linkuistics/grove/issues/17#issuecomment-5864306291
[i17-k64]: https://github.com/Linkuistics/grove/issues/17#issuecomment-5869468605
[i17-k73]: https://github.com/Linkuistics/grove/issues/17#issuecomment-5885388607
[i18-k8]: https://github.com/Linkuistics/grove/issues/18#issuecomment-5794535784
[i18-k48]: https://github.com/Linkuistics/grove/issues/18#issuecomment-5864306604
[i18-k73]: https://github.com/Linkuistics/grove/issues/18#issuecomment-5884943848
[i18-k12]: https://github.com/Linkuistics/grove/issues/18#issuecomment-5890463477

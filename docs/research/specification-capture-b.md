# Specification capture: independent survey B

## Scope and method

This survey asks how a long-running human/LLM development loop should capture a
stakeholder's intent, elaborate it into material precise enough to build and
check, and keep both views useful over time. It is deliberately biased toward
failure after adoption rather than toward introductory success stories.

The evidence base prioritizes original authors, maintainers, official project
documentation, longitudinal retrospectives, and empirical studies. A dated
practice or tool page establishes that a notation still exists; it does not by
itself establish broad adoption or effectiveness. Vendor and method-creator
reports are treated as practitioner evidence, not controlled evidence.

The central result is qualified support for a two-audience architecture, but
not for two independently authored sources of truth:

- Stakeholders and implementers usually need different projections.
- A bridge is maintainable when one projection is generated from the other,
  or when explicit traces and executable checks expose divergence.
- Two editable documents connected only by convention incur the same
  synchronization cost that repeatedly appears in model-driven engineering
  and requirements-traceability studies.
- A single constrained notation can sometimes serve both audiences. EARS and
  Attempto Controlled English are important counterexamples to a categorical
  claim that one notation must always fail.
- No primary longitudinal evidence found here validates an unconstrained
  pseudo-DSL whose formal meaning is supplied only by an LLM.

“Survived” below means there is dated evidence of continued use or reflection
over several years. It is not a prevalence estimate.

## 1. Which notations work for mixed audiences, and which survived?

### The durable seam is conversation plus a small durable artifact

The oldest successful approaches do not ask every participant to become an
editor of a formal model. They give a group a low-friction surface for reaching
shared understanding, then preserve a smaller artifact for later work.

| Approach | Mixed-audience mechanism | Evidence of survival | What survives if the method or tool disappears? |
| --- | --- | --- | --- |
| Domain-driven design's ubiquitous language | Domain experts and developers use the same model-backed terms in speech, prose, diagrams, and code. | Eric Evans's [2015 DDD Reference](https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf) restates the practice more than a decade after the book; Martin Fowler's [2006 account](https://martinfowler.com/bliki/UbiquitousLanguage.html) records the same core discipline. | Ordinary prose, identifiers, diagrams, and code remain readable. The social agreement and active correction practice do not survive automatically. |
| EventStorming | A facilitated room uses events and a visible legend to let specialists contribute before a software notation is imposed. | The creator dates the technique to about 2013 and documents multiple current formats on the [official EventStorming page](https://www.avanscoperta.it/en/eventstorming/). This is creator evidence, not an adoption study. | Photos, exported boards, and a narrative can survive. The conversation, discarded alternatives, and tacit interpretation of spatial layout are lossy unless summarized. |
| User story mapping | Participants arrange activities, tasks, and slices spatially so scope and sequence can be negotiated together. | Jeff Patton's [2009 article](https://www.jpattonassociates.com/wp-content/uploads/2015/01/patton_story_mapping_bettersw_1109.pdf) and his later history, [“The New Backlog”](https://jpattonassociates.com/the-new-backlog/), document continuity from work preceding the original 2005 article. This is author history, not independent longitudinal evaluation. | A photographed or exported map and release slices remain. Sequence, grouping, and rationale are easily lost if flattened into a backlog. |
| Example Mapping | A story is decomposed into rules, examples, and open questions on four card types. Product specialists can participate without writing automation syntax. | Cucumber's [2015 introduction](https://cucumber.io/blog/bdd/example-mapping-introduction/) and [current documentation](https://cucumber.io/docs/bdd/example-mapping/) show continued support. No primary multi-year organizational outcome study was found. | The four concepts fit plain text, tables, or cards. The meeting format and color semantics are replaceable. |
| Specification by Example / Gherkin | Concrete examples become shared executable specifications when the same examples are understood by business, development, and testing roles. | Gojko Adzic's [ten-year retrospective](https://gojko.net/2020/03/17/sbe-10-years.html) reports a 514-response survey and compares practice with the expectations in his 2011 book. | Human-readable examples remain, but step definitions, test runners, fixtures, and UI vocabulary can turn them into tool-bound scripts. |
| EARS | A small set of controlled-natural-language sentence patterns makes requirements more precise without exposing temporal logic. | Original authors' [ten-year retrospective](https://www.researchgate.net/publication/335535918_Ten_Years_of_EARS) reports use across domains; their [six-year lessons report](https://www.researchgate.net/publication/308970788_Listens_Learned_8_Lessons_Learned_Applying_EARS) says the method can be used on paper or in ordinary office tools, though coaching remains useful. | The sentences remain ordinary text. Pattern linting and coaching disappear, but the capture is not locked to a specialist file format. |
| C4 diagrams | A small hierarchy of context, container, component, and code views gives technical and nontechnical readers an agreed zoom level. | The creator's [FAQ](https://c4model.com/faq) traces the approach to 2006–2011 and documents later teaching and use. The [C4 site](https://c4model.com/) keeps the model notation- and tooling-independent. | Exported diagrams and their labels remain. Interactive navigation, layout, and validation may not. A diagram without supporting decisions is only a view, not the architecture. |
| Structurizr DSL | Text defines a C4 model and views, from which interactive or static diagrams can be generated. | The current [DSL documentation](https://docs.structurizr.com/dsl) supports static-site, PlantUML, Mermaid, PNG, and SVG exports. This proves current capability, not organizational longevity. | The DSL source and exported views are text/images. Rendering, automatic layout, and validation need compatible tooling. |
| Architecture decision records | A short current decision, context, status, and consequences are kept beside the code; later decisions supersede rather than erase it. | Michael Nygard's [2011 proposal](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) remains recognizable in the current [GDS guidance](https://gds-way.digital.cabinet-office.gov.uk/standards/architecture-decisions.html). | Small Markdown records and VCS history are highly portable. Search, cross-links, status, and consequences decay unless maintained. |

Two distinctions matter.

First, a ubiquitous language is not a glossary file. Evans requires the team to
exercise the language in speech, writing, diagrams, and code, and to change it
when awkwardness exposes a weak model. A glossary can publish the current
vocabulary, but cannot create that feedback loop.

Second, workshop artifacts and durable specifications optimize for different
moments. EventStorming's own account says the conversation can be more valuable
than the resulting model. Example Mapping explicitly lets a product specialist
leave detailed Gherkin drafting to developers and testers, then review it. That
is already a two-stage editorial pipeline, even though it is not described as
one.

### The most useful counterexample: EARS

EARS shows that “human-readable” and “machine-checkable enough to elaborate” do
not always require separate syntaxes. Its patterns constrain prose while
remaining usable in Word, Excel, or on paper. The ten-year report says
simplicity is its main strength and also reports the limiting condition: users
can apply the patterns badly, and training and coaching still matter. EARS is
therefore evidence for a *small, teachable constraint*, not for a rich new DSL.

## 2. What failed after adoption?

### BDD decays when examples become test scripts

Cucumber's own anti-pattern reports describe a consistent failure mechanism.
When scenarios are written after implementation, they become tests rather than
a medium for shared understanding. Incidental UI and production-data detail
then couples the prose to implementation. Multi-rule scenarios become harder
to discuss and stop serving as collaborative documentation. See Cucumber's
[anti-pattern series](https://cucumber.io/blog/bdd/cucumber-antipatterns-part-one/)
and its experience report, [“Keep your scenarios brief”](https://cucumber.io/blog/bdd/keep-your-scenarios-brief/).
The latter notes that long scenarios are often not read by product owners and
can fail even when the business rule they illustrate has not changed.

Adzic's [ten-year Specification by Example survey](https://gojko.net/2020/03/17/sbe-10-years.html)
is an unusually useful longitudinal correction to early expectations:

- 29% of users did not automate examples, often because automation was not
  worth the trouble.
- Only 12% overall used version-controlled text files as the source of truth;
  even among respondents who automated, the figure was 20%.
- 57% kept examples in a task tracker.
- The author concluded that one text artifact serving simultaneously as
  specification, test, and version-controlled living documentation had not
  worked out as expected.

The survey is self-selected and reports correlation rather than causation. It
nonetheless contradicts a strong claim that executable text naturally becomes
the lasting shared record.

**Implication:** examples should remain first-class evidence, but their durable
human representation should not inherit every detail needed by an automation
harness. Generate or link tests; do not require stakeholders to maintain them.

### Model-driven engineering pays synchronization and tool taxes

An industrial empirical assessment based on hundreds of survey responses and
20 interviews across 17 companies found both benefits and persistent costs.
In [“Empirical assessment of MDE in industry”](https://www.researchgate.net/publication/221554010_Empirical_assessment_of_MDE_in_industry),
74% reported significant additional training; just under 40% spent substantial
time synchronizing models and code; 43% found UML too complex; 43% found tools
too expensive; and 56% reported inappropriate or inadequate tools. Roughly two
thirds reported improved stakeholder understanding, while almost a quarter
reported unexpected confusion.

This is not evidence that model-driven engineering categorically failed. It is
evidence that benefits depend on process and that manual edits on both sides of
a generated boundary require an explicit synchronization policy.

**Implication:** a model/formal tier earns its cost only when it provides a
specific check or projection that prose cannot, and when authorship direction
is explicit.

### DSL evolution is ecosystem evolution

A language change is rarely confined to its grammar. ASML's industrial study
of [DSL evolution in a large-scale industrial repository](https://www.scitepress.org/papers/2018/66058/66058.pdf)
describes co-evolution pressure on models, transformations, editors, and other
dependent artifacts. A corpus-based study of the DERRIC language similarly
found that many changes were interdependent rather than isolated; see
[“A Case Study in Evidence-Based DSL Evolution”](https://homepages.cwi.nl/~storm/publications/ecmfa13.pdf).

Long-running success is possible, but it changes the staffing model. The
[RT Sequencer experience report](https://www.mdpi.com/2076-3417/12/23/12286)
describes a DSL used since 2010 by more than 1,000 domain experts daily. Its
evolution required a real language/tooling program and eventually a
general-purpose-language extension for advanced users. The paper also notes
that empirical reports about DSL evolution remain sparse.

**Implication:** a project-specific pseudo-DSL is not “just syntax.” If it has
parsing, diagnostics, migrations, projections, and escape hatches, it is a
language product. If it lacks those things, it is prose with an unreliable
appearance of formality.

### Decision records can become a write-only archive

Nygard proposed ADRs because large documents were not updated or read. The
current GDS guidance adds the operational conditions that keep small records
from suffering the same fate: VCS storage, review by the right people,
discoverable links, updated consequences, explicit supersession, and companion
architecture documentation. It also says an ADR is not a complete architecture
description.

Thoughtworks' [Technology Radar volume 29](https://www.thoughtworks.com/content/dam/thoughtworks/documents/radar/2023/09/tr_technology_radar_vol_29_en.pdf)
makes the analogous warning for lightweight RFCs: if they are not narrow and
brief, they grow into traditional documents that are filed and forgotten.

**Implication:** retain decision provenance, but project only current decisions
into the everyday capture surface. VCS and linked records can carry history;
every reader should not have to re-read it.

## 3. Can a human tier feed a formal tier without prohibitive drift?

The evidence favors *one authored representation plus generated projections*,
or *separate artifacts joined by checked traces*. It does not support two
freely edited documents synchronized by memory.

| System | Human-facing tier | Formal/elaboration tier | Drift control and cost | Walk-away result |
| --- | --- | --- | --- | --- |
| FRET | FRETISH, a restricted natural language, plus generated English and diagrams | Future/past-time temporal logic and verification artifacts | FRET translates rather than asking people to maintain both meanings. Mapping requirements to model/code variables is still explicit work. | Source requirements and generated explanations can be exported; translation and simulation depend on FRET. |
| Attempto Controlled English | A constrained subset of English | Discourse representation structures and Prolog | One notation has a defined translation, avoiding independent document drift at the price of a narrower language. | Text remains legible; machine semantics require the parser and controlled-language rules. |
| KAOS | Goal models and selectively informal descriptions | Temporal-logic assertions for critical properties, with solver/model-checker translations | Formalization is applied selectively, not to every sentence. Trace and tool discipline remain necessary. | Goal diagrams/prose can survive as views; proof and round-trip features do not. |
| ProR + Event-B | ReqIF requirements, including informal material | Event-B models and proofs | Explicit traces connect artifacts; the published case demonstrates feasibility but only on a traffic-light example, not sustained production economics. | ReqIF and Event-B are open representations, but integrated navigation and consistency checks need the toolchain. |
| LLM translation through a structured intermediate form | Natural language plus reviewer-facing structured natural language | Temporal logic | Current work reduces manual translation burden but still needs semantic validation and distinguishing traces. | The source and intermediate text survive. Reproducibility requires pinned prompts/models or a conventional compiler. |

Sources for the table include the NASA [FRET paper](https://ntrs.nasa.gov/api/citations/20200001989/downloads/20200001989.pdf),
the original [Attempto paper](https://arxiv.org/abs/cmp-lg/9603004), a
[KAOS overview](https://www.sciencedirect.com/science/article/pii/S1571066105050334),
and the [ProR/Event-B integration report](https://docserv.uni-duesseldorf.de/servlets/DocumentServlet?id=21882).

Traceability is not free. A controlled experiment on requirements-model trace
maintenance found that delayed updates are harder because the maintainer must
reconstruct the earlier reasoning; see
[Mäder and Egyed's study](https://pmc.ncbi.nlm.nih.gov/articles/PMC3587459/).
A systematic account of traceability challenges identifies maintenance and
reconstruction cost, inadequate tools, weak commitment, and conflicting
viewpoints as recurring barriers; see
[“Requirements traceability: a systematic review”](https://pmc.ncbi.nlm.nih.gov/articles/PMC8802773/).

LLMs change the price of elaboration, not the semantic obligation. The
[NL2Postcond study](https://arxiv.org/abs/2310.01831) found natural-language to
formal-postcondition generation useful enough to expose historical bugs, but
also identifies alignment between prose, code, and formalization as a remaining
problem. More recent work on
[ARTEMIS](https://cs.stanford.edu/~trippel/pubs/mendoza_ICSE26.pdf) inserts a
FRETISH-like structured intermediate representation and presents
distinguishing traces to reviewers because direct manual or model translation
is error-prone. That architecture treats the model output as a reviewable
proposal, not as authority.

No primary longitudinal production study was found for the full pipeline
“unconstrained stakeholder prose → LLM-created formal model → generated
implementation → sustained maintenance.” A recent paper proposing an empirical
study of whether formal specifications improve LLM code generation is still a
[study design](https://arxiv.org/abs/2601.03878), not outcome evidence.

### Verdict on the two-tier hypothesis

The hypothesis survives in this narrower form:

1. Keep a human-authoritative current-state capture.
2. Elaborate it into a formal or executable projection only where a named check
   justifies the cost.
3. Make elaboration one-way and reproducible when possible.
4. Where two authored artifacts are unavoidable, give requirements stable
   identities, explicit traces, coverage checks, and visible stale status.
5. Return counterexamples, examples, and unresolved ambiguities to the human
   tier; do not silently “repair” intent in the formal tier.

EARS and Attempto demonstrate that one constrained notation can sometimes
cover both audiences. They refute “two syntaxes are always required,” but not
the need for two views or levels of rigor.

## 4. What should a shareable glossary carry?

A shareable glossary is useful as a projection of current language, not as the
whole language practice. Kubernetes provides a mature example: its
[glossary](https://kubernetes.io/docs/reference/glossary/) is searchable and
tagged, gives a short definition before longer explanation, and links outward.
Its [style guide](https://kubernetes.io/docs/contribute/style/style-guide/)
separately governs wording, time-sensitive terms, contribution, and a broader
word list. Governance and presentation are related but not collapsed into each
entry.

A useful current glossary entry should carry:

- a stable term identifier and preferred display term;
- a short present-tense definition;
- scope or bounded context when the term is not globally uniform;
- one discriminating example or counterexample when ambiguity is likely;
- links to the decision or source that established the meaning;
- an owner or review surface, and machine-visible stale/unresolved status.

Rationale, full edit history, and reopen conditions should be *linkable and
projectable*, but not mandatory inline content in the default stakeholder view.
They answer a provenance question, while the definition answers a current-use
question. A maintainer view can join both. VCS already retains edit history;
ADRs or equivalent records can retain decision-specific rationale and
supersession.

No primary comparative study was found that tests an enriched glossary
(definition plus rationale, history, and reopen conditions) against a split or
projected design. The recommendation above is therefore an inference from
ubiquitous-language practice, ADR lifecycle guidance, and the usability goal of
a short current surface, not an empirically settled rule.

For this repository, a stakeholder glossary should be generated or curated
from the same current-state records the loop uses. It should not duplicate a
second manually maintained vocabulary beside `CONTEXT.md`, specifications, and
decision records.

## 5. What persists human guidance across LLM sessions, and how does it decay?

### Repository instructions are durable context, not enforcement

Claude Code's official [memory documentation](https://code.claude.com/docs/en/memory)
makes the persistence boundary explicit: each session begins with a fresh
context; checked-in `CLAUDE.md` files provide human-authored shared
instructions; auto-memory contains agent-authored notes; and both are loaded as
context rather than enforced rules. The documentation recommends keeping
project instructions under about 200 lines because longer files consume
context and reduce adherence, warns that contradictory instructions may be
chosen arbitrarily, and recommends removing stale or conflicting material.
Auto-memory is machine-local rather than team-shared and only an initial bounded
portion is loaded automatically.

GitHub Copilot likewise supports repository-wide, path-specific, and agent
instruction files, but support and precedence vary by surface; its
[custom-instruction documentation](https://docs.github.com/en/copilot/concepts/prompting/response-customization?tool=webui)
warns against conflicts. The portable core is therefore checked-in text with a
clear scope, not a particular product's discovery order.

An empirical study of 2,303 agent instruction files across 1,925 repositories,
[“Agent READMEs”](https://arxiv.org/abs/2511.12884), found that these files often
grow through frequent small additions and can become complex and difficult to
read. Functional repository context was common, while security and performance
guidance appeared in only 14.5% of files. This is evidence of additive accretion
and uneven coverage, not proof that any particular instruction causes model
behavior.

### Persistence mechanisms and their failure modes

| Mechanism | What persists | Characteristic decay | Walk-away result |
| --- | --- | --- | --- |
| Checked-in project instructions | Human-authoritative current rules and repository facts | Additive patches, contradictory scopes, stale commands, context dilution, and differences in client support | Plain text and VCS history survive. Automatic discovery and precedence may not. |
| Path-scoped rules or skills | Focused procedures loaded for relevant work | Discoverability gaps, overlapping scopes, orphaned references, and product-specific routing | The files survive; automatic selection may not. A small explicit index helps migration. |
| Agent auto-memory | Agent-authored observations and preferences | Stale inference, duplication, local-machine divergence, and uncertain authority | Usually poor: content may be local or stored in a proprietary location. Exportable Markdown improves auditability. |
| Retrieved long-term memory | Large histories or summaries selected on demand | Retrieval misses, stale traces, conflicting memories, opaque consolidation, and provenance loss | Raw records may survive, but selection and ranking depend on the memory system. |
| Task, requirement, and decision artifacts | Explicit human-approved state, unresolved questions, rationale, and outcomes | Write-only archives and history mixed into current state | Strong when stored as portable text with stable links and an explicit lifecycle. |

Memory research independently identifies naive accumulation as a problem. The
Microsoft Research paper
[“Human-inspired memory architecture for LLM agents”](https://www.microsoft.com/en-us/research/publication/human-inspired-memory-architecture-for-llm-agents/)
argues for consolidation, forgetting, and reconsolidation rather than retaining
all observations at equal weight. That reinforces the editorial lesson from
instruction files: persistence needs retirement and promotion, not just append.

For a development loop, the durable unit should therefore be an approved
current-state artifact, not the chat transcript or an agent's recollection.
Promote feedback into one of a small number of typed destinations:

- a changed requirement or example;
- an unresolved question with owner and reopening condition;
- a current decision plus a linked rationale record;
- a scoped operating instruction;
- or no durable record once the feedback has served its purpose.

Resolved guidance should be rewritten or retired from the active surface.
History remains in VCS and linked records. This makes “what applies now?” cheap
without erasing “why did it change?”

No primary evidence was found for a cross-vendor standard that preserves
agent-owned memory, its provenance, and its authority semantics across coding
agents. Checked-in text is the only broadly portable substrate in the surveyed
systems.

## 6. What tooling is worth building, and does presentation change participation?

### Tool capabilities that earn their maintenance cost

The useful tool is an editorial compiler, not a proprietary authoring silo. It
should:

- accept portable text with a deliberately small schema;
- give requirements, terms, examples, questions, and decisions stable IDs;
- lint missing fields, ambiguous references, forbidden cycles, stale links,
  and conflicting definitions;
- generate stakeholder, implementer, reviewer, glossary, and formal-check
  projections from the same current state;
- preserve source links and provenance in every projection;
- report trace coverage and distinguish “not formalized” from “proved”;
- return counterexamples or distinguishing traces in human-readable terms;
- export static Markdown/HTML and common image or model formats;
- make generated artifacts visibly generated and prevent accidental
  round-trip editing;
- work locally and in ordinary review workflows, with actionable diagnostics;
- keep the source useful when the compiler is absent.

The tool should not require a stakeholder to install a specialist IDE, buy a
seat, or understand proof syntax merely to correct wording. EventStorming's
creator explicitly notes that paper and sticky notes lower the contribution
barrier while digital workshops introduce familiarity and licensing friction.
That is practitioner observation, not causal evidence, but it matches the
training and tool barriers in the MDE survey.

### Medium changes feedback; visual polish alone has weak evidence

The strongest evidence found distinguishes representation and interaction from
mere polish:

- In a controlled prototype study, tangible prototypes produced a higher share
  of useful, design-relevant responses than virtual prototypes (77% versus 65%
  in the combined comparison), while high versus low visual fidelity was not a
  significant factor. Stakeholder group and the question asked also changed the
  result. See [“How prototypes influence stakeholder feedback”](https://link.springer.com/article/10.1007/s00163-019-00317-5).
- A 48-participant study found that showing multiple design alternatives led to
  less inflated appraisals and more critical comments. It also cautioned that a
  paper prototype alone did not guarantee engagement or redesign proposals.
  See [Tohidi et al., “Getting the right design and the design right”](https://groups.cs.umass.edu/nmahyar/wp-content/uploads/sites/8/2019/01/reading22.pdf).
- A seven-pair comparison of physical sticky notes and a no-frills digital
  equivalent found no significant difference in ideation or collaboration
  outcomes. The digital condition created more interactions with notes but also
  more attention shifts and breakdowns around devices. See
  [“Sticky notes and digital notes in collaborative design”](https://link.springer.com/article/10.1007/s10606-018-9325-1).
- An analytic study of sticky-note practices in EventStorming and user story
  mapping found generally positive support for organizing discussion but mixed
  results for illustration and persuasion. It was not a causal participation
  experiment. See [“A pragmatic analysis of sticky-note practices in collaborative modelling”](https://www.sciencedirect.com/science/article/abs/pii/S2590118420300575).

The defensible conclusion is that the representation medium, alternatives, and
questions change the feedback obtained. There is not strong evidence here that
greater aesthetic fidelity by itself increases stakeholder participation.

For the Grove loop, generate a readable stakeholder projection with diagrams,
examples, and open questions, but retain the portable text source. Offer
alternatives when feedback is genuinely open, and label fixed versus negotiable
areas. The source format should never be made harder to leave merely because a
rendered view is attractive.

## Cross-cutting walk-away test

The systems above expose a useful portability ladder:

1. **Strong:** ordinary text is authoritative; tools lint or project it. EARS,
   Markdown ADRs, and well-scoped repository instructions fit here.
2. **Acceptable:** a documented text DSL is authoritative and static exports
   are routine. Structurizr DSL and controlled English fit here, provided a
   compatible parser can be replaced.
3. **Conditional:** linked open formats preserve artifacts, but integrated
   meaning depends on traces and tooling. ReqIF plus Event-B fits here.
4. **Weak:** layout, navigation, memory ranking, or semantics live primarily in
   a vendor service or opaque model. Exported snapshots preserve evidence but
   not the working system.

Every proposed capture mechanism should answer four questions before adoption:

- Can a human read and correct the authoritative source without the tool?
- Can another implementation reproduce the projections from documented rules?
- Is history separable from current state without destroying provenance?
- If the elaborator is unavailable, which checks stop running and which
  meanings become inaccessible?

## Implications for the specification-capture design

### Recommended capture shape

Use portable Markdown with a small typed structure rather than a new general
DSL. The human-authoritative layer should support:

- a concise outcome and explicit non-goals;
- actors or audiences and the language they use;
- requirements written in direct prose, with optional EARS-like patterns where
  they improve precision;
- rules paired with examples, counterexamples, and open questions;
- stable term, requirement, and decision references;
- acceptance evidence and named uncertainties;
- linked rationale records rather than inline historical narration;
- explicit status only where it drives a transition or check.

An elaborator may compile that material into schemas, tests, diagrams, Quint or
another formal notation, trace matrices, and implementation briefs. These are
projections unless the workflow explicitly promotes one to authority. Generated
artifacts must record their source revision and elaborator version.

### Recommended feedback loop

1. Capture the stakeholder's current language and examples with unresolved
   ambiguity visible.
2. Generate an implementer/formal projection and trace each derived statement
   to stable source IDs.
3. Validate mechanically, then present counterexamples and distinguishing
   traces in stakeholder language.
4. Resolve meaning in the human-authoritative source; regenerate downstream
   artifacts.
5. Promote lasting rationale to a linked decision record, promote active rules
   to scoped instructions, and retire transient feedback.
6. Refuse completion when projections are stale, required traces are missing,
   or a generated artifact has been edited independently.

This is two-tier in audience and rigor, but one-way in ordinary authorship.

### What a downstream brief should contain

The research supports a brief that is more explicit than a prose request but
smaller than a complete specification:

- the audience and the decision or outcome the artifact must enable;
- current vocabulary and links to the relevant glossary projection;
- ordered sections and the purpose of each;
- rules plus representative examples and known counterexamples;
- emphasis, exclusions, and non-goals;
- acceptance evidence and checks to generate;
- open questions, with a condition for reopening resolved ones;
- source and decision links sufficient to recover provenance;
- the required static/exported form for the walk-away case.

### Claims the design should not make

- Do not claim that executable examples naturally remain shared
  documentation; the ten-year Specification by Example evidence contradicts
  that generalization.
- Do not claim that two editable tiers stay aligned cheaply; MDE and
  traceability evidence contradict it.
- Do not claim that a pseudo-DSL is lightweight merely because its syntax is
  short; evolution reaches parsers, transformations, editors, migrations, and
  users.
- Do not claim that an LLM's elaboration is formalization unless the output is
  checked and its semantic choices are reviewable.
- Do not claim that a polished view guarantees participation; controlled
  evidence supports effects from medium and alternatives, not polish alone.
- Do not claim that persistent agent memory is authoritative. Durable guidance
  needs a human-owned current-state artifact and an explicit retirement path.

## Evidence gaps to preserve for later design and review

- No direct primary comparison was found between a compact glossary and a
  glossary that embeds rationale, history, and reopen conditions.
- No longitudinal production evaluation was found for an LLM-mediated,
  two-tier stakeholder-to-formal capture system with measured drift cost.
- No independent multi-year organizational outcome study was found for Example
  Mapping or EventStorming; current evidence is largely creator/practitioner
  experience.
- Empirical reports of long-term DSL evolution are sparse, and the successful
  examples involve substantial language/tooling ownership.
- No cross-vendor standard was found for portable, authoritative coding-agent
  memory.
- No strong causal evidence was found that visual *polish* alone increases
  participation. Evidence instead concerns tangibility, alternatives,
  facilitation, and interaction cost.

These are not invitations to fill silence with confidence. They are constraints
on the claims the design and later user guide should make.

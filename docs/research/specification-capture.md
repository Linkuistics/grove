# Capturing human intent: the combined evidence

## Scope and evidence policy

This document reconciles the two independent surveys commissioned by
`plan-k1`, decision 16 —
[`specification-capture-a.md`](specification-capture-a.md) and
[`specification-capture-b.md`](specification-capture-b.md) — into one account
of how human intent is captured in a form stakeholders can share and elaborated
into something a machine can build against. It unions their coverage, names
every disagreement rather than silently resolving one, and carries the
adversarial move neither survey could perform on itself.

**No new primary source was fetched for this document.** Every locator below is
inherited from the survey that fetched it, and each claim is attributed to the
survey that read it. Where the two surveys report the same underlying source
differently, the discrepancy is recorded as a discrepancy. This is a
reconciliation, not a third survey; the corpus is exactly the union of two
corpora and is bounded by their two search strategies.

**Confidence labels** grade *independence* first, because that is the axis a
combiner can measure and a survey cannot:

- **High** — both surveys reached the claim through materially different
  primary sources, and the claim is stated in compatible terms by both.
- **Moderate** — one survey supplies the evidence and the other's corpus
  contains nothing that contradicts it, or both reached it but through sources
  from one research programme.
- **Low** — practitioner testimony, a single reachable source, or a claim
  assembled here from evidence neither survey combined.
- **Commissioned** — the claim was named in the shared brief before either
  survey ran. Its appearance in both is not convergence.

## Corpus independence audit

The task file requires this before any concurrence is credited, and it is
the load-bearing measurement in this document.

Extracting every distinct URL from each survey and intersecting them:

| | Survey A | Survey B |
| --- | ---: | ---: |
| Distinct cited URLs | 25 | 41 |
| Shared exactly with the other survey | 1 | 1 |
| Distinct sources in the union | | **65** |

**The single shared source is
[Agent READMEs 2025](https://arxiv.org/abs/2511.12884)**, the study of 2,303
agent context files across 1,925 repositories. It is cited by both for the same
finding — additive accretion — and even there the two reports differ: A gives
security specifications at 14.8% and performance at 14.5% as two figures, while
B reports that "security and performance guidance appeared in only 14.5% of
files", collapsing them. A's reading is the more precise one and is adopted.

**One further overlap is at the level of a research programme rather than a
document, and it is not visible from either survey alone.** A cites
[Hutchinson, Whittle and Rouncefield 2014](https://doi.org/10.1016/j.scico.2013.03.017);
B cites
[the same team's earlier industrial assessment](https://www.researchgate.net/publication/221554010_Empirical_assessment_of_MDE_in_industry).
That these draw on one survey instrument is established from the documents
themselves rather than asserted: both report **74%** for significant additional
training, a figure neither would have reproduced from an independent sample.
Section 3 of *Disagreements* records what the two papers do not share.

**Everything else is disjoint.** No other citation appears in both, and the
subject-level asymmetries are severe:

| Present only in A | Present only in B |
| --- | --- |
| ShardStore / Bornholt et al.; TLA+ and Newcombe et al.; SKOS; Moody's *Physics of Notations*; the expertise-reversal literature; METR's perception-gap trial; catastrophic remembering; AGENTbench; the 1,650-session instruction-adherence factorial; MemDelta; Spec Kit / Eberhardt; the 4,300-ADR text-mining study; the 2,502-repository DDD characterisation; the Gherkin repository-mining study; the 75-practitioner BDD survey; Hellesøy's post-mortem | **EARS and Attempto Controlled English**; FRET; KAOS; ProR + Event-B; ARTEMIS; NL2Postcond; Adzic's ten-year Specification by Example survey; the ASML, DERRIC and RT Sequencer DSL-evolution studies; Mäder and Egyed on trace maintenance; the traceability systematic review; the Kubernetes glossary and style guide; Nygard and the GDS ADR guidance; the Thoughtworks RFC warning; the prototype-fidelity, design-alternatives and sticky-note participation experiments; Claude Code and Copilot instruction documentation |

**Conclusion of the audit: agreement between these two surveys is worth
crediting.** The skill's standing suspicion — that two surveys agreeing is one
corpus reported twice — is measurably false here at 1 shared source in 65. That
is a result, not an assumption, and it raises the value of the agreements in
the next section by exactly as much as it would have lowered them had the
number come back the other way.

**But one correlation the audit cannot remove is upstream of both corpora.**
Both surveys received the same brief, and that brief named the hypothesis, named
seven candidate notations, instructed both to *bias the search toward failure*,
and prescribed the walk-away check. So:

- Both finding failure is **Commissioned**, not convergent. A failure-biased
  search returns failures. A names two structural biases that inflate positive
  results — survivor-biased adoption surveys, repository mining that excludes
  dead projects — and neither survey names the opposite bias introduced by its
  own instructions.
- Both producing a walk-away table is **Commissioned**. The instrument was
  mandated; only the verdicts were reached independently, and section 8 of
  *Disagreements* records where those verdicts differ.
- The successes that did surface — EARS, Attempto, RT Sequencer — are **all
  B's**, and B is the survey that treated a counterexample as a first-class
  target. That distribution is itself a signal about the brief.

## Findings at a glance

1. **The corpora are genuinely independent** — 1 shared source in 65 — so the
   agreements below are corroboration rather than echo.
2. **The strongest agreement is that executable examples did not become the
   lasting shared record.** Two disjoint measurement instruments and one
   creator post-mortem converge on it.
3. **The two surveys propose incompatible architectures**, and both call the
   result "two-tier". A draws the boundary at the *audience*; B refuses two
   authored artifacts at all and draws it at *authorship direction*. This is
   the disagreement the downstream design must settle.
4. **A's reformulation of the hypothesis has a counterexample it never saw.**
   A concludes the tier boundary belongs at the audience boundary; EARS is one
   notation, two audiences, ten years, and it is absent from A's corpus
   entirely.
5. **The mechanism that explains every case in both corpora is arbitration, not
   audience and not readability** — including the four cases that defeat A's
   own rule. Section *The adversarial move* states it and the prediction it
   makes.
6. **The hypothesis's second clause has no support in either corpus.** No
   source in 65 measures whether a formal tier improves an LLM's output. The
   premise the whole arm rests half of its case on is unevidenced.
7. **The proposal under examination sits on the horn the evidence says fails.**
   A checked pseudo-DSL acquires a machine obligation and is arbitrated;
   an unchecked one is "prose with an unreliable appearance of formality".
8. **Verdict: the hypothesis survives in a restated, narrower form and is
   refuted in its categorical form.** Section *Verdict* states it.

## Agreements, graded by independence

Agreement is counted only where the surveys reached the claim through different
primary evidence, and the grade says which.

| Agreed claim | Survey A's evidence | Survey B's evidence | Independence |
| --- | --- | --- | --- |
| Executable examples did not naturally become the durable shared record. | Repository mining of 1,572 Gherkin files across 23 projects: ~83% of repositories committed in 2021, only ~39% committed to spec files; 6–15 spec contributors in repositories of up to 736 ([Chandorkar et al. 2022](https://scg.unibe.ch/archive/papers/Patk22b.pdf)). Plus the creator's post-mortem ([Hellesøy 2013](https://cucumber.io/blog/collaboration/the-worlds-most-misunderstood-collaboration-tool/)) and a 75-practitioner survey ([Binamungu et al. 2018](https://pure.manchester.ac.uk/ws/files/181992545/SANER2018BinamunguKonstantinouEmbury.pdf)). | Adzic's 514-response ten-year retrospective: 29% never automated; only 12% used version-controlled text as the source of truth (20% among automators); 57% kept examples in a task tracker ([Adzic 2020](https://gojko.net/2020/03/17/sbe-10-years.html)). Plus Cucumber's own anti-pattern series. | **High.** Three instruments — repository mining, two independent practitioner surveys — and two creator post-mortems, sharing no source. The strongest agreement in the union. |
| A formal tier stays current only when something structural keeps it there; editorial intent never sufficed. | Every case where a specification stayed alive put it on the critical path: ShardStore's reference models double as unit-test mocks ([Bornholt et al. 2021](https://jamesbornholt.com/papers/shardstore-sosp21.pdf)); AWS keeps two tiers and makes no synchronisation claim ([Newcombe et al. 2014](https://lamport.azurewebsites.net/tla/formal-methods-amazon.pdf)); the Telecom Company's manual synchronisation procedure lost to the code. | "Elaborate it into a formal or executable projection only where a named check justifies the cost." Delayed trace updates cost more because the maintainer must reconstruct the reasoning ([Mäder and Egyed](https://pmc.ncbi.nlm.nih.gov/articles/PMC3587459/)); maintenance and reconstruction cost recur as the principal traceability barriers ([systematic review](https://pmc.ncbi.nlm.nih.gov/articles/PMC8802773/)). | **High.** Disjoint corpora — industrial case studies versus traceability experiments — reaching the same structural requirement. |
| Two freely edited artifacts joined only by convention do not stay aligned. | ~35% of a success-biased MDE sample spend significant time synchronising; engineers overrode the model with hand-inserted logic under a written manual procedure ([Hutchinson et al. 2014](https://doi.org/10.1016/j.scico.2013.03.017)). | "It does not support two freely edited documents synchronized by memory" — from the traceability literature above and the MDE assessment. | **Moderate.** The MDE evidence is one research programme reported through two papers; the traceability evidence is independent. |
| Plain text under version control is the only substrate that survived; tool-bound representations did not. | An eleven-system walk-away table. Every artifact still maintained years later scored Full; every Nil was abandoned or became a liability. | A four-rung portability ladder — Strong / Acceptable / Conditional / Weak — plus four questions every capture mechanism must answer before adoption. | **Commissioned** as an instrument (the brief mandated a walk-away check); **High** as a verdict, since the two independently graded corpora sort the same way. |
| A shareable glossary carries the current definition; provenance is linked or projected, never inline by default. | SKOS splits its note set by audience, not by definition-versus-rationale ([SKOS Primer §4.2](https://www.w3.org/TR/skos-primer/)). C4's level slicing is one model rendered as audience-specific views. | The Kubernetes [glossary](https://kubernetes.io/docs/reference/glossary/) gives a short definition before longer explanation and keeps governance in a separate [style guide](https://kubernetes.io/docs/contribute/style/style-guide/); rationale and history are "linkable and projectable". | **High** on *one maintained source, several projections*. The two disagree on what belongs in the shared projection — see *Disagreements* 4. |
| Agent context files accrete additively, are loaded as context rather than enforced, and need a retirement path. | Instruction files more than triple over their lifetime (+226%), and the older an instruction is the less likely it is to be deleted ([catastrophic remembering 2026](https://arxiv.org/abs/2608.11095)); no structural variable changed adherence across 1,650 sessions ([McMillan 2026](https://arxiv.org/pdf/2605.10039)). | Claude Code's own [memory documentation](https://code.claude.com/docs/en/memory) states the boundary — context, not enforcement; Microsoft Research argues for [consolidation, forgetting and reconsolidation](https://www.microsoft.com/en-us/research/publication/human-inspired-memory-architecture-for-llm-agents/) over equal-weight retention. | **Moderate.** The shared source (Agent READMEs) supplies the accretion observation to both; each brings independent corroboration on top of it. |
| An agent's self-curated memory is not an authoritative artifact; a human-owned current-state text is. | Agent self-memory measured *worse* than plain retrieval (42% vs 47%), and a headline +11pp memory gain reversed on changing only the embedding model ([MemDelta 2026](https://arxiv.org/pdf/2606.29914)). | Auto-memory is machine-local, of uncertain authority, and walks away poorly; no cross-vendor standard exists for portable, authoritative agent memory. | **High.** A controlled ablation and a portability analysis, sharing nothing. |
| Aesthetic polish alone does not buy stakeholder participation. | Reported as a **silence**: no study found relating presentation quality to participation. METR's trial, where developers were 19% slower while believing they were 20% faster, means any in-house claim must be instrumented behaviourally ([METR 2025](https://arxiv.org/abs/2507.09089)). | Reported as an **answer**: tangible prototypes produced more design-relevant responses than virtual ones (77% vs 65%) while visual fidelity was not significant ([prototype study](https://link.springer.com/article/10.1007/s00163-019-00317-5)); showing multiple alternatives produced more critical comments; physical and digital sticky notes showed no significant difference. | **Moderate** for the shared negative conclusion, but the two surveys disagree on whether the question is answered — see *Disagreements* 5. |
| The value of capture is realised largely at capture time, not at consumption time. | MDE practitioners were unanimous that the benefit was explicit architecture rather than code generation; AWS report insight "without ever going as far as a full specification"; 43% of LLM revisions surfaced aspects the stakeholder had not mentioned; Spec Kit's trialist: user stories "lack much value once the feature is complete". | EventStorming's own account says the conversation can be more valuable than the resulting model; Example Mapping's value is the refinement conversation, with detailed drafting deferred. | **High**, and **neither survey listed this as one of its findings.** It is assembled here, and section *The adversarial move* records what it costs the proposal. |

## Disagreements and unresolved tensions

Every one is named. None is silently resolved; where the union does settle one,
the resolution is stated with its confidence.

### 1. Can a single notation ever serve both audiences?

This is the disagreement that decides the arm.

**A says no, structurally.** Its organising claim is that "the notations that
survived are the ones with no execution semantics"; it invokes Moody's
*cognitive fit* principle, which names single-notation practice *visual
monolinguism* and treats it as a design flaw; and it reads the 44%-too-complex
/ 31%-not-powerful-enough MDE split as the "fails at both" pattern measured in
one sample.

**B says yes, with two named counterexamples**, and calls one of them "the most
useful counterexample". **EARS** constrains prose into a small set of sentence
patterns, exposes no temporal logic, remains usable in Word, Excel or on paper,
and has a ten-year retrospective from its original authors reporting use across
domains. **Attempto Controlled English** is a constrained English subset with a
*defined translation* to discourse representation structures and Prolog — one
notation, legible text, machine semantics. B's conclusion: they "refute 'two
syntaxes are always required', but not the need for two views or levels of
rigor."

**The asymmetry is total: survey A contains zero occurrences of EARS, Attempto,
or controlled natural language.** The survey that pronounced the hypothesis
survived never had its principal counterexample in view.

**Resolution.** The categorical clause of the hypothesis — *a format attempting
both fails at both* — is **refuted**. But B's counterexamples are weaker than
they first look, and the weakness is instructive rather than convenient:

- EARS's formality is pattern conformance, not machine semantics. You can lint
  an EARS requirement; you cannot model-check one. It refutes "two syntaxes",
  not "two tiers of rigor". Its evidence is also its authors' own retrospective,
  which B labels practitioner evidence.
- Attempto has the machine semantics EARS lacks, but B offers no adoption or
  survival evidence for it at all — the citation is the original 1996 paper.
  It demonstrates that the thing is *possible*, not that it is viable, and B
  states the price: a narrower language, and semantics that need the parser.

So: one notation for both audiences is demonstrably achievable, and the two
achieved instances bought it by making the notation *small*, not by making it
*rich*. **Confidence: high** for the refutation of the categorical clause;
**moderate** for the reading that smallness is the price.

### 2. Where does the tier boundary belong — at the audience, or at authorship?

**A concludes the boundary is the audience boundary**: "the tier boundary
belongs where the *audience* boundary is, not where the formality boundary is."
Where one population owns both purposes, fuse (ShardStore); where two
populations do, split (Gherkin). Under this rule two separately authored
artifacts are legitimate whenever the audiences differ.

**B refuses two authored artifacts outright.** Its architecture is "one
authored representation plus generated projections", elaboration "one-way and
reproducible", generated artifacts "visibly generated" with round-trip editing
prevented, and completion refused when "a generated artifact has been edited
independently". Its summary sentence: "This is two-tier in audience and rigor,
but one-way in ordinary authorship."

**These are different systems, and both are called two-tier.** A's permits two
maintained sources; B's permits exactly one. Sections 4 and 5 below inherit the
disagreement.

**Resolution.** B's architecture is the better-evidenced one and A's own
evidence supports it over A's own rule. A's anti-drift enumeration found that
every surviving specification was on the critical path of ordinary work; a
second independently authored artifact is by construction not on that path
unless something checks it, and A's Telecom Company case is what a manual
synchronisation procedure degenerates into. A's audience rule is also the one
EARS breaks (disagreement 1) while B's authorship rule is untouched by it —
EARS is a single authored source, projections optional. **Adopt B's
one-authored-source-plus-projections architecture, and keep A's requirement that
the projection be on the critical path.** The two compose; that composition is
this document's principal recommendation. **Confidence: moderate-high.**

### 3. The same MDE research programme yields opposite-signed conclusions

Both surveys cite the Hutchinson–Whittle–Rouncefield industrial programme
(established as one programme in the audit above, by the shared 74% figure).
They report different slices of it, and the slices point in different
directions.

| | Survey A | Survey B |
| --- | --- | --- |
| Significant additional training | 74% | 74% |
| UML too complex | 44% | 43% |
| UML **not powerful enough** | 31% | *not reported* |
| Significant time synchronising models and code | ~35% | just under 40% |
| Tools too expensive / inadequate | *not reported* | 43% / 56% |
| **Improved stakeholder understanding** | *not reported* | roughly two thirds |
| Unexpected confusion | *not reported* | almost a quarter |

**A's "fails at both" evidence and B's majority-positive stakeholder finding
come from the same programme, and neither survey reports the other's figure.**
A's 44%/31% split is the only quantitative support for the hypothesis's "fails
at both" clause anywhere in either corpus, and A presents it as exactly that. B's two-thirds
reporting *improved stakeholder understanding* is direct evidence against
reading MDE as an audience failure.

This is a selection effect visible only from the combined view, and it is the
clearest instance in this reconciliation of a survey's frame shaping which
figure it carried out of a shared source. Neither survey is wrong; the pair,
read together, will not support the confident reading either gives.

**Resolution.** The MDE corpus establishes that a two-artifact modelling
practice is *expensive* — training, tooling, synchronisation — and that its
principal reported benefit is at capture time (explicit architecture, improved
stakeholder understanding). It does **not** establish that UML failed at both
purposes. A's "fails at both, measured in one sample" reading is downgraded to
**low confidence**: the two percentages are population-level and, as A itself
notes, not necessarily the same respondents, and the same programme reports a
two-thirds positive on the very audience question at issue.

### 4. What may the shared glossary projection contain?

Both surveys reach *one source, several projections* (agreement above). They
cut the projection in different places, and the difference is exactly the open
question the root brief parks under *On the horizon* for `CONTEXT.md`.

**A, from SKOS, puts significant meaning change inside the stakeholder-facing
set.** `skos:historyNote` — "describes significant changes to the meaning or
the form of a concept" — is one of the four user-facing note types, alongside
definition, scope note and example. Only `skos:editorialNote` (housekeeping)
and `skos:changeNote` (fine-grained administrative churn) are editor-facing.
A's line is therefore *significant meaning change versus the churn of
maintaining it*, which it explicitly contrasts with the definition-versus-
rationale line the question anticipated.

**B puts history outside the default view.** "Rationale, full edit history, and
reopen conditions should be *linkable and projectable*, but not mandatory
inline content in the default stakeholder view." VCS retains edit history; ADRs
retain decision rationale.

**These are opposed on the one field that matters.** A's `historyNote` is B's
excluded material.

**Resolution.** The two lines are reconcilable because they partition different
things, and the union is sharper than either. Three tiers, not two:

1. **Current meaning** — definition, scope, discriminating example. In every
   projection. Both agree.
2. **Significant meaning change** — that a term's meaning shifted, and to what.
   A's evidence puts this in the stakeholder view and gives a standardised name
   for it; B's objection is to *full edit history*, which is a different thing.
   **In the stakeholder projection, per A.**
3. **Churn and provenance** — the edit record, editorial to-do, the deciding
   ADR, reopen conditions. Linked, not inline. Both agree.

B's *reopen conditions* are the residual disagreement and are not settled here:
they are neither current meaning nor past change, and no evidence in either
corpus bears on them. **Confidence: moderate** for tiers 1 and 3, **low** for
tier 2, since it rests on a standards document rather than on a study, and
**unsettled** for reopen conditions.

**A separate warning applies to all three tiers, and it points the opposite way
from the question's worry.** The measured failure in the wild is not a glossary
made unreadable by too much rationale — it is a glossary with none: rationale
and considered options were "absent or misplaced in most" of ~4,300 ADRs, and
25.3% of 2,502 DDD projects record no explicit business context at all. B's
counter-warning — that decision records become write-only archives, and that
insufficiently narrow RFCs grow into documents that are filed and forgotten —
is about a different failure, and both are real. The combined instruction is
*attach rationale and retire it*, never *attach or omit*.

### 5. Is the participation question answered or unanswered?

**A reports a silence** and reports it deliberately: "No study was found
relating presentation or tooling *quality* to stakeholder *participation*", and
records that it searched for this directly rather than answering from the
adjacent comprehension literature.

**B reports four studies**: prototype tangibility (77% vs 65% design-relevant
responses, fidelity not significant), design alternatives producing more
critical comments, a seven-pair sticky-note comparison finding no significant
ideation difference, and an analytic study of sticky-note practices.

**A's silence is too strong for the union.** B's corpus contains controlled
evidence bearing on the question. The two are reconcilable on what each
measured: A asked about *participation* (who contributes, how much), and B's
studies measure *feedback quality and character* obtained from a given set of
participants — except the sticky-note comparison, which does address
collaboration outcomes and finds no difference.

**Resolution.** Restate the finding rather than pick a survey. What the union
supports: **medium, tangibility and the presence of alternatives change the
feedback obtained; aesthetic fidelity alone does not.** What remains a genuine
silence: whether presentation changes *whether a stakeholder participates at
all*. A's METR warning stands over any in-house attempt on either —
self-reported participation is exactly the measure that trial inverted by 39
percentage points. **Confidence: moderate** for the medium finding; the
participation question stays in *What neither survey found*.

### 6. Is the DSL-lifecycle suspicion substantiated?

**A reports it unsubstantiated from primary sources** — the one named suspicion
it could not close — and names two paywalled targets (Hermans, Pinzger and van
Deursen; Kosar et al.) as next steps rather than citing secondary summaries.

**B found three reachable studies A did not**: ASML's industrial DSL-evolution
study, the DERRIC corpus study, and the RT Sequencer experience report — a DSL
used since 2010 by more than 1,000 domain experts daily, whose evolution
required a real language/tooling programme and eventually a general-purpose
language extension for advanced users.

**Resolution — and it is not simply "B closed A's gap".** B's three establish
that DSL change is *ecosystem* change, reaching models, transformations,
editors, migrations and users, and that sustained success requires owning a
language product. They do **not** establish that maintenance outlived benefit;
RT Sequencer is a success. So A's suspicion is neither confirmed nor refuted —
it is **replaced by a better-specified one**: the question is not whether a DSL
decays but whether the project is prepared to staff a language product. B states
the consequence for this repository's proposal exactly: a pseudo-DSL with
parsing, diagnostics, migrations and projections "is a language product"; one
without them "is prose with an unreliable appearance of formality".
**Confidence: high** for the ecosystem-cost mechanism; A's two paywalled targets
remain worth fetching and are recorded below.

### 7. Do ADRs decay?

**A** reports this as an explicit unmeasured silence: the 4,300-ADR study
"contains no analysis of ADR updates, supersessions, staleness, or abandonment
rates", which "were explicitly outside the research scope". So ADRs are known
to be adopted and known to under-record rationale, with decay unmeasured.

**B** treats decay as an established risk to be designed against, citing
Nygard's original motivation (large documents were not updated or read), GDS's
operational conditions, and Thoughtworks' RFC warning — all practitioner
guidance, no measurement.

**Resolution.** No measurement exists; B's material is prescriptive rather than
empirical, and A's silence is accurate. B's operational conditions are worth
adopting as design guidance without being cited as evidence of decay.
**Confidence: low** — this remains an unmeasured question, recorded below.

### 8. Walk-away verdicts that differ on the same system

The instrument was commissioned, so its two applications are comparable, and
they do not always agree.

| System | A's verdict | B's verdict | Note |
| --- | --- | --- | --- |
| Architecture decision records | **Full.** "Nothing to uninstall." | Portable, but "search, cross-links, status, and consequences decay unless maintained." | B is less sanguine, and is describing decay of the *set* rather than legibility of a *record*. Both are right about different objects. |
| Structurizr DSL | **Partial.** Source legible, rendered views — "the artifact non-technical readers actually engaged with" — lost. | **Acceptable** (rung 2): text DSL authoritative, static exports routine. | B counts static exports as surviving; A counts interactive views as lost. B's is the more accurate reading of current tooling; A's is the more accurate reading of what the non-technical audience used. |
| Agent memory stores | **Nil.** | "Usually poor… exportable Markdown improves auditability." | Compatible. |
| Gherkin / Specification by Example | **Partial.** The half the business reads survives; the half that made it true does not. | Human-readable examples remain, but tooling "can turn them into tool-bound scripts". | Compatible. |

**Resolution.** Where they differ, the difference is about *what the check is
asked about* — a single record versus a maintained set, a source file versus
the rendered artifact an audience actually consumed. That is a defect in the
instrument, not in either survey, and it is worth carrying forward: **a
walk-away check must name the artifact and the audience it is asked about.**
**Confidence: high**, as a correction to the instrument.

## The adversarial move

The task file requires it, and the root brief states why: both surveys were
commissioned against a hypothesis the human supplied, and a survey commissioned
against a hypothesis tends to find it. The question is what the evidence would
look like if the hypothesis were wrong, and whether that pattern appears.

### The hypothesis has three clauses, and they are not equally tested

> *Formal specifications are poor human artifacts* (H1) *and good LLM
> artifacts* (H2)*, so the capture layer and the formal tier are two layers
> rather than one notation; a format attempting both fails at both* (H3).

Both surveys tested H3. Neither tested H2. That is the first finding of this
section.

### What falsification would look like, and where it appears

**Pattern 1 — a machine-consumable notation that non-specialists authored and
maintained for years, with no decay of the human half.** *It appears.* EARS,
ten years, both audiences, usable on paper. Attempto, one notation with a
defined formal translation. Both are B's, and both are absent from A. The
categorical clause is refuted; see *Disagreements* 1 for what survives of it.

**Pattern 2 — deliberate fusion of the tiers outperforming separation.** *It
appears, in A's own corpus.* ShardStore chose to write its reference models in
the implementation language, **against explicitly better alternatives**, and
gave drift avoidance as the reason: separate specifications end up "languishing
as separate expert-written artifacts". Fusion was the right answer there.

A rescues the hypothesis by reassigning this case to a different seam
(formal ↔ code) and restating the rule as an *audience* boundary. That rescue
must be audited rather than adopted, because it was constructed after seeing
the counterexample. The audit is to ask whether it makes a prediction the
corpus can check. It does: one population sharing an artifact holds, two
populations sharing one decays. Against the union — these four are the cases both surveys treat as decisive,
and the rest of the corpus (ADRs, TLA+, Spec Kit: one population each) is
consistent with the rule without testing it:

| Case | Populations sharing one artifact | Outcome | A's audience rule |
| --- | --- | --- | --- |
| Gherkin | two | decayed | ✓ predicted |
| MDE / UML | two | model lost to code | ✓ predicted |
| ShardStore | one | held | ✓ predicted |
| **EARS** | **two** | **held, ten years** | **✗ contradicted** |

**A's reformulation is falsified by the one case A never had.** This is the
correlated blind spot the pair was bought to expose, and it is the single most
important product of this reconciliation.

EARS is not the only two-population artifact in the union that held —
ubiquitous language and architecture decision records are two more, and both
scored Full on the walk-away check. What makes EARS the decisive counterexample
is that it is the only two-population survivor with any claim to being
machine-checkable, which is the property A's rule is really about. The other
two corroborate the mechanism named below rather than testing the rule: neither
has an automated enforcer either.

**Pattern 3 — evidence that a formal tier does not improve machine output
(against H2).** *Neither survey reports a source that measures it.* The nearest approaches
are all A's or B's adjacent material: LLM-generated context files *reduced*
agent success by 0.5–2% while raising cost 20–23%, and human-written ones
helped by ~4% at up to 19% more cost (A); a Spec Kit trial that ran ten times
slower than the trialist's incremental approach and did not prevent the bug
(A); a four-month uncontrolled before/after pilot reporting improvements, which
its own authors disclaim thoroughly (A); NL2Postcond finding LLM-generated
formal postconditions useful for exposing historical bugs but leaving
prose/code/formalisation alignment unsolved (B); and a 2026 paper *proposing*
an empirical study of whether formal specifications improve LLM code generation
which is, as B notes, "a study design, not outcome evidence" (B).

**H2 is unevidenced.** It is the clause that motivates building a formal tier at
all, and it is the one clause neither survey was asked to test.

### What the hypothesis predicts that does not appear

If formal specifications were poor human artifacts *because they are formal*,
decay should track formality. **It does not.** The cleanest decay signature in
either corpus belongs to Gherkin — the *least* formal machine-readable notation
surveyed, deliberately written as near-prose. The most formal notation
surveyed, TLA+, shows no decay signature in the AWS record at all; what it
shows is a restricted audience, and A's own walk-away table scores it
"Expert-only" rather than decayed.

**So formality predicts audience width, not survival.** H1's stated mechanism —
readability — is wrong even where H1's conclusion holds. A came close to
saying this (it names arbitration as the real mechanism) and then substituted
the audience rule that EARS breaks.

### The mechanism that explains every case in both corpora

Not readability, and not audience. **Arbitration.** Where one artifact carries
both a machine obligation and a human one, only the machine obligation has an
automated enforcer, so it wins every conflict — silently, because nothing goes
red when the human obligation degrades. The practitioner quotation A found
states it without any reference to syntax: refactoring scenarios to remove
duplication "causes the requirements and their understanding to change from
what the Product Owner wants."

Checked against all four cases, including the one that defeats A's rule:

- **Gherkin** — two obligations, one enforcer (the test runner). The enforced
  one rewrote the other. Decayed.
- **MDE / UML** — the code obligation had the enforcer (the compiler and the
  shipping product). Engineers overrode the model. Model lost.
- **ShardStore** — one population, so the two obligations never conflict; and
  the enforcer runs *on the specification*, making it the artifact that cannot
  be skipped. Held.
- **EARS** — two populations, but **no automated enforcer at all**. Nothing can
  fail a build over an EARS sentence, so nothing arbitrates it, so the human
  obligation is never overridden. Held.

Arbitration explains all four. Audience explains three. Readability explains
none of them.

**The prediction it makes for this repository is sharp, and it is unwelcome.**
The proposal under examination is *a DSL, or a pseudo-DSL that does not
syntactically restrict, paired with LLM skills*. Both horns fail:

- **Give the capture notation a checker**, and it acquires a machine obligation.
  It is then arbitrated, and Gherkin is the measured outcome — the artifact
  stays alive as the machine's and dies as the human's.
- **Leave it unchecked**, and it is, in B's words, "prose with an unreliable
  appearance of formality" — and A's evidence says an unchecked tier is the one
  that goes stale, because nothing puts it on the critical path.

**The escape the union supports is to move the check off the capture artifact
and onto its projection.** The capture layer carries no machine obligation, so
nothing arbitrates it. The generated formal projection carries all of them and
sits on the critical path, satisfying A's anti-drift requirement. Failures
return to the human tier as counterexamples in the human tier's own language —
B's step 3, and ARTEMIS's architecture, which "treats the model output as a
reviewable proposal, not as authority" — rather than as edits to the capture
artifact. This is B's one-way authorship composed with A's critical-path
requirement, and it is the only shape in the union that neither horn defeats.
**Confidence: moderate-high** — it is assembled from both corpora rather than
observed running anywhere, and no source in either survey reports a
production instance of it.

### Two more things the hypothesis does not anticipate

**The reviewer least able to detect drift is the stakeholder.** In the only
study measuring it, high-experience participants caught six content errors to
the low-experience group's one, in a setting where LLM revisions were rated
better on every dimension and 5% introduced factual or logical inaccuracies. A
two-layer design that has an LLM elaborate the capture layer and asks the
non-technical author to confirm it has placed its weakest instrument at its most
dangerous seam. (A's finding; nothing in B contradicts it.)

**The value is upstream of the artifact.** The agreement table records this,
and neither survey listed it as a finding. If the returns are at capture time —
explicit architecture, surfaced omissions, improved stakeholder understanding,
"insight without ever going as far as a full specification" — then an elaborate
notation-plus-tooling story spends where the returns are not, and Adzic's
figures say what practice actually does with the durable artifact: 57% keep
examples in a task tracker and 12% keep them as version-controlled text. The
proposal assumes a maintained document; the measured default is a ticket.

## Verdict on the two-tier hypothesis

Stated as the root brief requires, and a result contradicting the hypothesis is
a successful outcome here.

**The hypothesis is refuted in its categorical form.** "A format attempting
both fails at both" is false: EARS and Attempto Controlled English are
single notations serving both audiences, one of them with ten years of reported
use. Survey A concluded otherwise and had neither in its corpus.

**It survives in a restated, narrower form, for a mechanism neither the human's
statement nor survey A's reformulation gives.** The restatement:

> Where one artifact carries both a machine obligation and a human one, the
> machine's obligation has the only automated enforcer and silently rewrites the
> other. Separate the tiers when — and only when — a machine obligation would
> otherwise attach to the artifact people share.

This is an *arbitration* boundary. It subsumes A's audience boundary (which
holds wherever the corpus tests it except EARS), it explains ShardStore's
fusion without reassigning it to another seam, and it explains why the small
constrained notations survived: they bought their survival by having nothing
that could fail a build.

**Its second clause is unevidenced.** Neither survey reports a source measuring
whether a formal tier improves an LLM's output. Nothing downstream may treat H2 as established,
and the brief's rule — that nothing downstream of P4 may cite the two-layer
form as settled before k6 reports — should be read as now permitting the
restatement above and *still forbidding* any claim that a formal tier pays for
itself against an LLM consumer.

**The architecture the union supports** is one human-authoritative capture
source; formal projections generated one-way, visibly generated, and refused
when stale or independently edited; every check attached to the projection and
none to the capture artifact; failures returned to the human tier as
counterexamples in its own language; and the notation kept small, because both
instances of a notation that served two audiences bought it with smallness
rather than richness.

## What neither survey found

A question returned unanswered by both is a finding, and the downstream design
needs it stated rather than left as silence. Union of the two silence sections,
with the ones **both** surveys reached independently marked — those are the
strongest, because two disjoint search strategies failed the same way.

- **Whether a formal tier improves an LLM's output.** Neither survey reports a
  source that measures it. The strongest silence in this document, because it is the
  hypothesis's own second clause. *B names the study design that would answer
  it; A does not raise the question.*
- **Whether non-technical stakeholders actually read specification
  artifacts.** *(A explicit; B's mixed-audience evidence is entirely creator and
  practitioner testimony, so implicit.)* This is the load-bearing premise of
  Gherkin, C4, example mapping and every "living documentation" claim. The only
  proxy is A's Gherkin contributor measurement — 6 to 15 spec-file contributors
  in repositories of up to 736 — which suggests an unflattering answer without
  measuring it.
- **Whether an enriched glossary outperforms a split or projected one.**
  **Both, independently.** A: "No empirical work was found on glossary
  shareability." B: "No direct primary comparison was found between a compact
  glossary and a glossary that embeds rationale, history, and reopen
  conditions." Section *Disagreements* 4 is therefore inference from a standards
  document and a documentation exemplar, and is the least empirically grounded
  recommendation here.
- **Whether projection outperforms duplication.** *(A explicit.)* C4 and SKOS
  establish that one-source-many-views is buildable; neither establishes it is
  better than maintaining two artifacts.
- **Any longitudinal production evaluation of an LLM-mediated two-tier capture
  system with measured drift cost.** **Both, independently.** This is precisely
  the system this grove proposes to build, so it is being built without
  precedent — which is a reason to instrument it, not a reason not to build it.
- **Any independent multi-year outcome study of EventStorming, user story
  mapping, or Example Mapping.** **Both, independently.** A: no rigorous
  longitudinal or controlled evaluation for event storming or user story
  mapping, with example mapping the partial exception (two case studies, whose
  authors describe the field's condition: these techniques are "frequently
  presented by consultants and agile trainers" while "the effectiveness of these
  techniques is not evaluated in a rigorous manner"). B: current evidence is
  largely creator and practitioner experience.
- **Whether ADRs are updated, superseded, or go stale.** *(A explicit; B
  prescribes against decay without measuring it.)* Explicitly outside the scope
  of the only large ADR study.
- **Whether presentation changes whether a stakeholder participates at all.**
  Reduced from A's broader silence by B's four studies — see *Disagreements* 5 —
  but the participation question itself survives. Any in-house answer must be
  instrumented behaviourally, per METR's 39-point perception gap.
- **Any cross-vendor standard for portable, authoritative agent memory.** *(B
  explicit.)* Checked-in text is the only broadly portable substrate in either
  corpus.
- **Primary post-mortems for DSL maintenance outliving DSL benefit.** A's
  silence is *partially* closed by B's three studies, but they establish
  ecosystem cost rather than benefit exhaustion — see *Disagreements* 6. A's two
  named targets (Hermans, Pinzger and van Deursen on DSL success factors; Kosar
  et al. on DSL usability in maintenance) were behind a 403 and a paywall, were
  not read, and remain the next targets.

**Two limits on the union itself**, which no reader should have to reconstruct:

- **Both surveys were instructed to bias toward failure**, so the corpus
  systematically under-samples success. A separately records the two biases that
  push the other way — survivor-biased adoption surveys, repository mining that
  excludes dead projects — but neither survey names the bias its own brief
  introduced. Every success in this document (EARS, RT Sequencer, ShardStore,
  Attempto) surfaced despite the search strategy, not because of it.
- **The LLM-era evidence is months old and version-bound.** The agent-context,
  catastrophic-remembering, instruction-adherence, memory and spec-driven-
  development results are 2025–2026 preprints, several without visible peer
  review, each tied to specific model and harness versions. They are the best
  available answers to question 5 and will need re-reading rather than
  re-citing. The agent-memory subfield is in open, largely vendor-published
  disagreement; A's methodological position — fix embedding models across
  comparisons, stratify by model family, report write-path cost before
  attributing gains to architecture — is adopted here.

## Adopted findings and the ADR bridge

None yet. Under `ADR-FORMAT.md`, an **adopted** research finding gets a bridge
pointing both ways — a section here naming the ADRs its findings landed in, and
a citation of this survey in each of those ADRs' rationale. This session
reports; it adopts nothing and records no decision, so it writes no ADR and
opens no bridge. Neither precedent survey carries one either.

**The obligation passes to whichever design session adopts a finding above.**
The findings most likely to earn a record, each already carrying its rejected
alternative:

- The arbitration boundary, rejecting the audience boundary and the readability
  boundary.
- One authored source plus one-way generated projections, rejecting two
  authored artifacts joined by convention.
- Checks attached to the projection and never to the capture artifact,
  rejecting a checked capture notation.
- The three-tier glossary split, rejecting both an enriched single artifact and
  a bare current-state one.

That session adds the bridge section here when it lands the record.

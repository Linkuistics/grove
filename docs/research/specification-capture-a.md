# Capturing human intent for stakeholders and for machines

## Scope and confidence

This survey asks how requirements, specification, guidance and feedback are
captured in a form people without formal training can read, share and
contribute to, **and** elaborated into something formal enough for a machine to
build against. It is commissioned against a hypothesis — that the capture layer
and the formal tier are two layers rather than one notation, and that a format
attempting both fails at both — with instructions to test that expectation
rather than assume it. It draws no technical claim from this codebase.

**The corpus is deliberately biased toward failure.** Introductions to these
techniques are abundant and were skipped; what is cited here is post-mortems,
controlled experiments, repository-mining measurements, and practitioner
surveys. Where only an advocate's account exists, that is said.

**"Two-tier" names two different seams, and conflating them is the easiest
error in this literature.** The commissioned hypothesis is about the *human ↔
formal* seam. Most published industrial two-tier experience is about a
different seam, *formal ↔ code*, and the two have opposite verdicts: Amazon Web
Services keeps prose and TLA+ deliberately apart and accepts an unverified gap
down to code, while the S3 ShardStore team deliberately collapses the formal
tier *into* the implementation language. Section 3 answers for both seams and
labels which is which. Evidence that a formal model belongs in the
implementation language is not evidence that a stakeholder can read it.

**Confidence is uneven across the six questions.** Questions 1, 2 and 5 have
real quantitative evidence — repository mining over thousands of projects,
controlled trials, practitioner surveys with stated sample sizes. Question 3
rests on a small number of unusually well-documented industrial cases.
Question 4 has almost no direct empirical evidence, but has a precise
*standardised* answer from vocabulary publishing that nobody in the software
literature seems to have noticed. Question 6's second half — whether
presentation quality changes stakeholder *participation* — is largely a
silence, and the last section says so.

Recommendations that combine bodies of evidence are marked **inference** rather
than reported as results.

## Findings at a glance

1. **The notations that survived are the ones with no execution semantics.**
   Architecture decision records, ubiquitous language and plain-Markdown agent
   context files are alive at scale across thousands of repositories. The
   notations that promised to be simultaneously human-readable and executable —
   Gherkin above all — show a measurable abandonment signature: living
   repositories with dead specification files.

2. **The recurring failure is not notation quality, it is that one artifact
   acquires two masters.** In Gherkin the executable obligation rewrites the
   requirement; practitioners reported that refactoring scenarios to remove
   duplication "causes the requirements and their understanding to change from
   what the Product Owner wants". In model-driven engineering the code
   obligation defeats the model; engineers inserted logic into generated code
   "to the point where these overrode the model".

3. **Elaboration cost is real, measured, and paid at the seam.** A third of a
   success-biased MDE sample spends significant time synchronising models and
   code. AWS, asked how they know the code matches the verified design, answer
   "we don't."

4. **The anti-drift devices that worked are structural, not editorial.** Every
   case where a formal artifact stayed current put it on the critical path of
   ordinary work — ShardStore's reference models double as unit-test mocks, so
   adding a feature *requires* updating the specification.

5. **A glossary is shareable when it carries why a meaning changed and not the
   churn of changing it.** The line is not definition-versus-rationale; SKOS
   puts significant meaning change in the user-facing note set and
   administrative housekeeping in the editor-facing one.

6. **Persistent LLM guidance decays by unbounded accretion, and rationale is
   the cure rather than the disease.** Instruction files more than triple over
   their lifetime and old instructions get *less* likely to be deleted; the
   measured mechanism is that a rule whose reason is lost cannot be safely
   removed. Attaching the reason removed 99.3% of the excess.

7. **File structure does not measurably change adherence; having something that
   fails does.** A 1,650-session factorial study found no detectable effect of
   file size, instruction position, file splitting or contradictions — with
   affirmative null support for two of them.

8. **The hypothesis survives at the seam it was aimed at, and is contradicted at
   the seam it was not.** Section 8 states the verdict.

## 1. Notations for a mixed audience, and which survived

The question named seven candidates. They divide cleanly by whether the
notation carries execution semantics, and that division predicts survival
better than anything else in this corpus.

### The notations alive at scale

**Architecture decision records.** A 2026 text-mining study assembled roughly
4,300 ADRs from 547 open-source GitHub repositories, filtered from an initial
6,061 across 867 projects, and classified them with an LLM pipeline. That
population size is itself the survival evidence: a format introduced as a blog
post is now a routine repository artifact. Its content, however, is thinner
than the format promises — "Considered Options … was absent or misplaced in
most ADRs", and "Rationale and alternatives remain under-documented". Existence
decisions dominate at 64%, with executive at 18% and property at 14%
([Text mining ADRs 2026](https://arxiv.org/html/2609.07375)). The format
survived; the fields that distinguish a decision record from a changelog
frequently did not get filled in.

**Ubiquitous language, as carried by domain-driven design.** A large-scale
characterisation identified 2,502 verified DDD repositories from 11,742
candidates, with substantial agreement against human raters (κ = 0.77). DDD
projects are unusually persistent: median longevity 340.37 days and mean 660.73
days, against a 9.9-day median for a typical GitHub project, with the longest
reaching about 14.9 years. Tactically, Entities and Value Objects are the most
adopted patterns while Event-Driven Architecture (9.67%) and microservices
(2.56%) remain minorities. Crucially for question 4, "25.3% of DDD projects
record no explicit business context in their documentation, revealing a
persistent gap between how domain intent is designed and how it is preserved in
version-controlled artifacts"
([DDD in practice 2026](https://arxiv.org/html/2607.06471)). The longevity
figure must not be read as caused by DDD — projects that adopt an explicit
design methodology are plausibly more serious to begin with, and the study is a
characterisation rather than a controlled comparison.

**Plain-Markdown agent context files.** An empirical study of 2,303 agent
context files across 1,925 repositories found them "complex,
difficult-to-read artifacts that evolve like configuration code through
frequent, small additions". What developers put in them is strongly functional:
test procedures 75.9%, implementation details 70.8%, architecture 68.1%,
against security specifications at 14.8% and performance at 14.5%
([Agent READMEs 2025](https://arxiv.org/abs/2511.12884)). A separate report
puts AGENTS.md uptake at more than 40,000 open-source projects, though that
figure reaches this survey through a secondary source rather than a
measurement
([McMillan 2026](https://arxiv.org/pdf/2605.10039), citing Layer5).

### The notation with a measured abandonment signature

**Gherkin.** An exploratory study analysed 1,572 specification files from 23
open-source projects and produced the cleanest decay measurement in this
corpus. About 83% of the repositories had commits in 2021, but only about 39%
had commits *to their specification files* in 2021 — and a similar number last
touched specification files before 2019. The authors' conclusion is stated
plainly: "Although repositories are maintained actively, teams might stop using
BDD in their project." Participation is narrow too: repositories with between 3
and 736 contributors had only 6 to 15 contributors to the specification files,
"meaning only a small fraction of all contributors are involved in modifying
them". Of 1,572 files only 590 used data tables, and those averaged two rows
([Chandorkar et al. 2022](https://scg.unibe.ch/archive/papers/Patk22b.pdf)).
The corpus excluded unmaintained projects, so this is a *survival-biased*
sample showing abandonment anyway.

### The notations where practice outran the evidence

**Model-driven engineering, UML and custom DSLs.** The largest empirical
account is a 449-response survey plus four company case studies. DSLs were "far
more prevalent than expected", with 62% of custom-DSL users also using UML; 95%
use models for problem understanding and about 70% use model-to-model
transformations
([Hutchinson, Whittle and Rouncefield 2014](https://doi.org/10.1016/j.scico.2013.03.017)).
The authors state their own sampling limit: respondents "generally … have had
at least some success with MDE — those who have tried MDE but failed were
probably not accessible". Every figure below is therefore a floor on the
difficulty, not an estimate of it.

**C4 and its DSL.** The C4 model's load-bearing choice is that one text model
projects several audience-specific views rather than one diagram serving
everyone, and its author reports that as a participation mechanism: "Slicing
the picture into levels invites more people to the party: business and UX
people engage with the context diagram because they're thinking about how users
reach their goals, and infrastructure and DevOps people gravitate to the
container diagram." The DSL was chosen over a drag-and-drop editor because
canvases "become unmanageable at scale", and "one text file can drive multiple
views with consistent styling"
([Brown, interviewed 2025](https://newsletter.nerdnoir.com/p/mapping-your-architecture-with-simon)).
This is the designer's account of his own tool and contains no measurement; it
is reported as a design argument with a strong structural precedent, not as
evidence of effect.

**Example mapping.** The only rigorous evaluation found is two longitudinal
case studies — eight sessions over seven weeks at one company, five over four
weeks at a large Dutch pension manager — using a purpose-built Refinement
Evaluation Tool. The paper opens by naming the field's problem: "New techniques
for managing, specifying, and analyzing requirements … are frequently presented
by consultants and agile trainers. However, the effectiveness of these
techniques is not evaluated in a rigorous manner." Example mapping did
contribute to shared understanding, and two sessions were generally needed
before teams used it effectively. Two findings deserve the downstream design's
attention: "intention to use is rated lower than the other dimensions", and a
contextual factor "the grey literature behind EM did not mention" — story size.
"conducting a 30 minutes refinement for a user story with effort estimated to 1
or 2 hours is clearly too expensive"
([Berends and Dalpiaz 2021](https://webspace.science.uu.nl/~dalpi001/papers/bere-dalp-2021-re.pdf)).
Results are qualitative and from two cases; the authors say generalisation is
difficult.

**Event storming, specification by example and user story mapping.** No
rigorous longitudinal or controlled evaluation was found for event storming or
user story mapping as *notations that survived years of use*; see *Search
silence and limits*. Specification by example is represented here through its
executable descendant, Gherkin, because that is where the measurements are.

## 2. What went wrong after adoption

The post-mortems converge on one mechanism, and it is not that the notations
were badly designed. It is that a single artifact acquired two masters with
different obligations, and the stronger obligation rewrote the weaker one.

### Gherkin: the executable obligation rewrites the requirement

**The creator's own post-mortem is the strongest single citation in this
survey.** Writing in 2013, after Cucumber had reached millions of downloads,
Aslak Hellesøy opened with "If you think Cucumber is a testing tool, please
read on, because you are wrong", and identified the misuse precisely: "Many of
them saw Cucumber as a **better** automated testing tool, but most of them
completely missed out on the underlying practices." He names three failure
modes. Scenarios written backwards — "They write Cucumber scenarios **after**
the software instead of the other way around." Loss of the documentation role —
"When Cucumber is adopted solely as a tool to write automated tests without any
input from business analysts they tend to become imperative and lose their
documentation value." And, decisively for the commissioned hypothesis, an
audience failure stated by the tool's own author: "Cucumber is a development
tool, and therefore many business analysts won't touch it with a ten feet
pole."
([Hellesøy 2013](https://cucumber.io/blog/collaboration/the-worlds-most-misunderstood-collaboration-tool/)).
The notation designed to be readable by both audiences was, in its creator's
assessment, not adopted by one of them.

**A practitioner survey supplies the frequencies.** Binamungu, Konstantinou and
Embury surveyed 75 BDD practitioners from 26 countries. Adoption is real but
shallow: 20% of organisations had made BDD mandatory while 61% used it as an
optional tool. On maintenance, 61% held that duplication in specifications can
make them "difficult to extend and change (leading potentially to frozen
functionality)", 49% that it makes suites take longer than necessary, and 43%
that it makes specifications difficult to comprehend. Duplication management is
mostly manual (40%), and "there is a significant proportion (17%) of
respondents who have given up the duplication detection and management process,
because of its complexity"
([Binamungu, Konstantinou and Embury 2018](https://pure.manchester.ac.uk/ws/files/181992545/SANER2018BinamunguKonstantinouEmbury.pdf)).

**The mechanism, in a practitioner's words.** One respondent stated the
two-masters collision exactly: "Over refactoring features and scenarios to
avoid duplication causes the requirements and their understanding to change
from what the Product Owner wants." An engineering-hygiene operation on the
artifact silently rewrote the requirement it was also serving as. The same
paper records which half gets dropped under pressure: "there are vertical use
concerns whereby some practitioners do not observe BDD best practice, notably
by **avoiding or downplaying the collaboration aspects**, resulting in future
costs."

**Inference from the evidence:** when one artifact must satisfy a machine and
inform a human, the machine's obligation is the one with an automated
enforcer, so it wins every conflict. The human obligation degrades without
anything going red.

### Model-driven engineering: the code obligation defeats the model

**The cost is distributed away from the beneficiary.** Against a
success-biased sample: 74% said MDE requires significant extra training (fewer
than 9% disagreed), while only 46% thought it let them employ less experienced
engineers. More than 75% called code generation important to their productivity
gains, but 40% said integrating generated code is a significant problem against
36% who said it is not. On synchronisation, about 70% mainly update models
rather than code, yet approximately 35% "definitely or probably spend a lot of
time synchronizing their models and code"
([Hutchinson, Whittle and Rouncefield 2014](https://doi.org/10.1016/j.scico.2013.03.017)).

**One notation, both complaints, from the same population.** 44% of
respondents thought UML "definitely or probably too complex" while 31% thought
it "definitely or probably … not powerful enough". The authors note: "After
fourteen years of development of the UML language, almost half of our
respondents still believe that UML is too complex and almost a third believe
that, despite this complexity, it is not yet powerful enough." This is the
predicted "fails at both" pattern measured in a single sample — though the two
figures are not necessarily the same respondents, so it is a population-level
result rather than a demonstration that individuals held both views.

**The purest recorded drift failure.** In the Telecom Company case study, a
top-down mandate put fifty engineers on a CASE tool. When the approach proved
inflexible, engineers "developed 'work-arounds' – primarily, inserting program
logic into code fragments that would augment the generated code, **to the point
where these overrode the model**." Consistency was maintained by a written
manual procedure: "after generating the code, if you want to do this then you
have to delete this line and that line and also you have to change this
parameter like this." Over millions of lines, the team lost the ability to
reason about the artifact they shipped — "they totally… attach to the tool and
they do not know what is going on with the generated code". The product
shipped; "the company did not then adopt this approach for any other
projects."

**What the same study found MDE was actually good for is the more useful
finding.** "our data suggests that productivity gains from code generation
alone are not considered significant enough to drive an MDE adoption effort …
Rather, it turns out that the main advantages are in the effect that MDE has on
helping to create a well-defined software architecture. Unanimously, our
interviewees argue that MDE makes it easier to define explicit architectures."
The value was in the act of capture and structuring, not in the elaboration to
code.

### DSL maintenance outliving DSL benefit

This is the one named suspicion the survey could not substantiate from primary
sources. The empirical DSL literature is dominated by usability and
productivity comparisons rather than lifecycle post-mortems, and the honest
summary of what was reachable is that "most papers primarily provide anecdotal
evidence for the benefits claimed". The nearest recorded mechanism is that
"more substantial changes may become more difficult, as such changes may
involve altering the domain-specific language, which will require compiler
technology knowledge that not every commercial enterprise has easily
available." The MDE survey's adjacent finding is that small DSLs for "narrow,
well-understood domains" are typical — which bounds the exposure rather than
removing it. See *Search silence and limits*; two candidate primary studies
were paywalled.

### Spec-driven development with LLMs: the newest instance of the oldest failure

**A practitioner trial with figures.** Colin Eberhardt removed a feature from
a working application and rebuilt it with GitHub Spec Kit, following
Constitution → Specify → Plan → Tasks → Implement. The specification comprised
5 user stories and 18 functional specs, and the run produced 2,577 lines of
Markdown across 33.5 minutes of agent execution, followed by 3.5 hours of
review. His own incremental approach took 8 minutes of agent time and 15
minutes of review to produce roughly 1,000 lines of code; he concludes "I am a
lot more productive without SDD, around ten times faster". The specifications
did not prevent an implementation bug, and he found no guidance for the
backward edge: "how to express this bug from a specification perspective?"
Finally, on persistence: "once complete, and fully tested, how often do you
re-visit a user story? Very rarely, they lack much value once the feature is
complete"
([Eberhardt 2025](https://blog.scottlogic.com/2025/11/26/putting-spec-kit-through-its-paces-radical-idea-or-reinvented-waterfall.html)).

**A contrary indicative result, and why they are compatible.** A four-month
before/after pilot across three full-stack teams and fourteen engineers
reported median lead time per feature falling from 8–12 to 6–9 working days,
late-stage hotfixes from 3–5 to 1–2 per sprint, and short-horizon churn from
12–18% to 6–10% of changed lines, at a specification-authoring overhead of
45–90 minutes per medium feature. The authors disclaim it thoroughly: "not
statistically controlled", no parallel control group, self-reports alongside
system metrics, junior developers deliberately excluded, and "temporal
confounds … cannot be excluded"
([Farrag 2026](https://arxiv.org/abs/2605.01160)).
**Inference from the evidence:** the two results differ by what the
specification is protecting. Eberhardt's subject was a small well-understood
feature in a personal app, where a specification carries no information the
implementer lacked; the pilot's subjects were mature multi-team codebases where
the specification pinned cross-cutting frontend/backend contracts. This
predicts that capture pays where the intent is not recoverable from the code,
and is pure overhead where it is — the same shape as question 5's finding about
non-inferable content.

## 3. Two-tier architectures and what elaboration costs

### Seam A: human-facing capture elaborated into a formal tier

**The clearest working instance is AWS's use of TLA+, and it is close to the
architecture under examination.** "We have adopted the practice of first
writing a conventional prose design document, then incrementally refining parts
of it into PlusCal or TLA+. Often this gives important insights without ever
going as far as a full specification or model checking." Two layers; the
elaboration deliberately partial; value claimed from partial elaboration
([Newcombe et al. 2014](https://lamport.azurewebsites.net/tla/formal-methods-amazon.pdf)).
The reported scale: TLA+ applied to 10 large complex real-world systems, 7
teams using it, engineers from entry level to Principal learning it from scratch
"in 2 to 3 weeks", with specifications between 102 and 939 lines. Five of the
six tabulated specifications found one to three bugs each; the sixth found none
and the table says why — "Failed to find a liveness bug as we did not check
liveness", which is the corpus's one recorded instance of a formal tier
reporting clean because nobody asked it the question.

**The same paper contains the sharpest admission that the formal tier is a poor
human artifact.** In a paper written to advocate the method: "We have not
included any snippets of specifications because their unfamiliar syntax can be
off-putting to potential new users." The authors judged that showing their own
notation would cost them readers. Their documentation claim is also carefully
bounded — "a precise, testable, **well commented** description of a design is
an excellent form of documentation" — and its stated audience is engineers
being brought up to speed, not non-technical stakeholders. Note too that
PlusCal exists precisely because "the syntax and idioms of TLA+ are somewhat
unfamiliar to programmers": the formal tier needed its *own* more approachable
dialect for an audience of programmers.

**Gherkin is the same seam attempted as one notation, and section 2 records the
result.** Scenario text plus step definitions is a two-tier system wearing one
name: the tiers are the `.feature` file and the glue code. The measured outcome
is that the human tier stopped being maintained while the repositories stayed
alive.

### Seam B: formal tier elaborated into code

**Where the elaboration is not mechanised, the gap is simply accepted.** Asked
the question engineers always ask — "How do we know that the executable code
correctly implements the verified design?" — Newcombe et al. answer: "The
answer is that we don't." And: "While we would like to verify that the
executable code correctly implements the high-level specification, or even
generate the code from the specification, we are not aware of any such tools
that can handle distributed systems as large and complex as those we are
building." The two tiers are joined by human discipline, and the paper argues
the value anyway — a broken design guarantees broken code, and formal methods
help find the strong invariants that make good assertions.

**Where drift was unacceptable, the seam was removed rather than managed.** The
ShardStore team validated a 40,000-line Rust storage node with executable
reference models — and names the failure mode it was avoiding: reference models
"are written in the same language as the implementation and embedded in its
code base, allowing them to be written and maintained by the engineering team
**rather than languishing as separate expert-written artifacts**." The choice
was made against better alternatives and the trade-off is stated: "We could
instead have used a language intended for modeling (Alloy, Promela, P, etc.),
which would have given us better expressiveness and built-in checking tools.
However, we found that by writing reference models in the same language as the
implementation, we make them easier for engineers to keep up to date. We also
minimize the cognitive burden of learning a new language and mapping concepts
between model and implementation"
([Bornholt et al. 2021](https://jamesbornholt.com/papers/shardstore-sosp21.pdf)).

### The anti-drift devices that actually worked

This is the most transferable result in the survey, and it is uniform: in every
case where a specification stayed current, something *structural* kept it
there. Editorial intent appears nowhere in the successes.

- **Put the specification on the critical path of ordinary work.** ShardStore
  reuses each reference model as the mock in unit tests, "effectively requiring
  engineers to update the reference model specifications themselves when
  developing new code". A developer adding a feature cannot skip the
  specification without breaking their own tests.
- **Keep it in a language its maintainers already read.** The same paper's
  stated reason, and it is why 18% of the reference-model and harness code had
  been written by non-formal-methods engineers by publication.
- **Make it cheap to run and let it be run more.** The checks are
  "pay-as-you-go", runnable locally during development and at scale before
  deployment. Reference models are 1% of implementation code; properties and
  harnesses together 12%; 16 issues were prevented from reaching production.
- **Give the notation a machine that rejects invalid input.** Structurizr's
  author added MCP tooling "so an LLM can run validation and parsing, read the
  error messages, and fix its own invalid output" — the checker is what makes
  an LLM-authored artifact converge.
- **Where none of these is possible, say so and do not pretend otherwise.** The
  AWS answer — "we don't" — is a better engineering position than a
  round-trip mechanism nobody maintains, and the MDE case study shows what the
  alternative degenerates into.

**Inference from the evidence:** drift cost is not a property of having two
tiers. It is a property of having two tiers with only one of them checked. The
Telecom Company had two tiers and a manual synchronisation procedure, and the
model lost; AWS has two tiers and no synchronisation claim, and nothing is
silently false; ShardStore has two tiers with the lower one checking the upper
one on every test run, and the specification stayed alive.

## 4. What makes a glossary shareable

The question asks whether a ubiquitous language that also carries rationale,
history and reopen conditions is still usable by a stakeholder, or whether
those are two artifacts that must be separated or projected. The software
literature has almost nothing direct on this. The vocabulary-publishing world
standardised an answer twenty years ago, and it cuts the problem in a third
place that neither option in the question anticipated.

**SKOS splits the note set by audience, and the line is not
definition-versus-rationale.** The SKOS Primer states it explicitly: "In
addition to these notes that are intended for users of a concept scheme, SKOS
includes two specializations of `skos:note` that are useful for KOS managers or
editors." The user-facing set is `skos:definition` ("supplies a complete
explanation of the intended meaning of a concept"), `skos:scopeNote`
("supplies some, possibly partial, information about the intended meaning"),
`skos:example`, and — the load-bearing one — `skos:historyNote`, which
"describes significant changes to the meaning or the form of a concept". The
editor-facing set is `skos:editorialNote`, which "supplies information that is
an aid to administrative housekeeping, such as reminders of editorial work
still to be done", and `skos:changeNote`, which "documents fine-grained changes
to a concept, for the purposes of administration and maintenance"
([SKOS Primer, §4.2](https://www.w3.org/TR/skos-primer/); the
[SKOS Reference](https://www.w3.org/TR/skos-reference/) defines the properties
but deliberately delegates recommended usage to the Primer).

So the answer to "is a glossary carrying history still shareable?" is: **yes
for significant meaning change, no for the churn of maintaining it.** A
stakeholder needs to know that a term's meaning shifted and how; they do not
need the fine-grained edit record or the editor's to-do list. That is a
sharper line than "separate them", and it is a projection rather than a
duplication — all six are properties of one concept, distinguished by role, so
one source can yield two views.

**The empirical evidence such as it is says rationale rarely gets written at
all, which reframes the risk.** Across roughly 4,300 ADRs, "Decision Drivers
were frequently misplaced … and often lacked substantive content" and
"Considered Options … was absent or misplaced in most ADRs"
([Text mining ADRs 2026](https://arxiv.org/html/2609.07375)). Across 2,502 DDD
repositories, 25.3% "record no explicit business context in their
documentation"
([DDD in practice 2026](https://arxiv.org/html/2607.06471)). The practical
failure mode observed in the wild is not a glossary made unreadable by too much
rationale; it is a glossary with none.

**And an unrelated literature supplies a positive reason to keep rationale in
the shared artifact.** The agent-instruction study in section 5 measures
directly what a missing rationale costs: "once an instruction's rationale is
gone, deleting it without risking a correctness regression costs O(2^|D|)".
Rationale is what makes an entry *removable*; without it the artifact can only
grow. That result is about instruction files rather than glossaries, so
transferring it is an **inference** — but it is the only measurement in this
corpus that puts a cost on rationale's absence, and it points the opposite way
from the question's worry.

**Projection has a working precedent in this corpus.** C4's level slicing is
one model rendered as several audience-specific views, and its author reports
different roles engaging with different levels
([Brown 2025](https://newsletter.nerdnoir.com/p/mapping-your-architecture-with-simon)).
Combined with SKOS's role-tagged notes, the pattern is consistent: one
maintained source, several projections, never two maintained artifacts. No
empirical comparison of projection against duplication was found.

## 5. Making human guidance persist across many LLM sessions

This question has the newest and, unexpectedly, some of the best evidence in
the survey.

### What decays, and the measured mechanism

**Instruction files grow without bound, and old instructions become harder to
remove.** An analysis of 247,694 instruction lifetimes across 1,867
repositories found agentic prompt files "grow without bound, more than tripling
over their lifetime (+226%), gaining +4.9 net instructions every commit", and
that "the older an instruction gets, the less likely it is to be deleted
(log-hazard −0.032/commit)". The authors name the phenomenon *catastrophic
remembering*, the inverse of catastrophic forgetting, and identify its
mechanism as an asymmetry of information rather than of effort: "appending an
instruction is always cheap, but once an instruction's rationale is gone,
deleting it without risking a correctness regression costs O(2^|D|)"
([Catastrophic remembering 2026](https://arxiv.org/abs/2608.11095)).

**The remedy they measure is attaching the reason to the rule.** "comments
encoding latent reasoning remove 99.3% of excess instructions (+211.3% to
+1.4%)", and the same intervention "can improve real-world agentic
instruction-following by up to 23.1%". Their closing question is the design
statement: "If English is the new code, why don't we have comments yet?"

### What actually helps, and what merely costs

**Only non-inferable content pays.** AGENTbench comprises 138 instances from 12
repositories sourced from 5,694 pull requests, all with developer-written
context files, evaluated with four agents in three settings, alongside
SWE-bench Lite. LLM-generated context files *reduced* success by 0.5% to 2% on
average while increasing cost by 20–23% (2.45–3.92 additional steps).
Human-written files improved success by about 4% on average — described as
marginal — at up to 19% additional cost. Instructions are followed when
present: `uv` usage rose 1.6× when mentioned, and repository-specific tools
2.5×. The authors' recommendation is narrow and specific: omit LLM-generated
context files, and include "only minimal requirements (e.g. specific tooling to
use with this repository)", because "unnecessary requirements from context
files make tasks harder"
([Gloaguen et al. 2026](https://arxiv.org/html/2602.11988v1)).

**File structure is not the lever practitioners assume.** A factorial study
measured compliance across 1,650 Claude Code CLI sessions and 16,050
function-level observations, on two TypeScript codebases, three frontier
models and five tasks, manipulating file size (25 to 500 lines), instruction
position, file architecture, and the presence of a contradicting instruction in
a secondary file. "None of the four structural variables or three two-way
interactions in the design produces a detectable contrast after
multiple-testing correction", with affirmative-null Bayes factors between 0.05
and 0.10 for the size and conflict nulls. The largest effect was within-session
attenuation: "approximately 5.6% lower odds of compliance per generation step
(OR = 0.944)". The paper also names the structural gap: the file "carries no
schema and no built-in confirmation that its rules have been followed, so
without supporting tooling (agent hooks, guardrails, linters, CI checks) a
missed instruction may go unnoticed"
([McMillan 2026](https://arxiv.org/pdf/2605.10039)). Its own stated limit
matters: "our target instruction is deliberately trivial", so the null result
should not be extended to multi-step procedural or conditional rules.

**Inference from the evidence:** across the three studies the levers that
moved anything were content selection (non-inferable only), rationale
attachment (removability), and external checking. Formatting was not one of
them.

### Retrieval and agent memory, and why the reported gains need care

**Letting the agent curate its own memory measured worse than plain
retrieval.** A controlled protocol varying one component at a time on
LongMemEval-S (500 questions, 50+ sessions, three model families) found "agent
self-memory (42%) underperforms basic retrieval (47%)". More importantly it
showed that published gains are frequently mis-attributed: Mem0 beat a
MiniLM-embedding RAG baseline by +11pp, but changing *only* the embedding model
reversed the conclusion — "The apparent '+11pp memory gain' was not a property
of the memory architecture. It was an embedding confound." Swapping the
embedding model alone shifts accuracy by 6.2pp (p = 0.004). On 2 of 6 question
types (n = 88), Mem0 matched cloud RAG (72.7% vs 73.9%, p = 1.0) "at 50× the
cost". Baselines are not single points either: Sonnet gained +31pp from RAG
"partly because it refuses 63% of full-context queries"
([MemDelta 2026](https://arxiv.org/pdf/2606.29914)).

**This corpus therefore contains a direct disagreement, and it splits along
vendor lines.** Mem0's own research pages report 92.5 on LoCoMo and 94.4 on
LongMemEval; independent benchmark work reports commercial memory agents
performing poorly across most benchmarks, and MemDelta shows one uncontrolled
variable flipping such a comparison. The disagreement is recorded rather than
resolved; MemDelta's recommendation — fix embedding models across comparisons,
stratify by model family, and report write-path cost before attributing gains
to architecture — is the methodological position this survey adopts.

**Inference from the evidence:** durable artifacts plus retrieval currently
measures at least as well as an agent-curated memory, and is far easier to
audit. A design that keeps the record as legible files and retrieves over them
is not conceding anything to the state of the art.

## 6. Tooling quality, and whether presentation changes participation

The question has two halves. The first — what good tooling looks like — has
usable evidence. The second — whether presentation quality changes stakeholder
*participation* — is close to a silence, and the distinction matters because
the available literature measures comprehension and satisfaction instead.

### The theory is explicitly against one notation for all audiences

**Moody's cognitive-fit principle names the anti-pattern.** The Physics of
Notations argues that software engineering has neither a theory nor a
systematic body of empirical evidence guiding visual notation design, and
proposes nine principles for cognitive effectiveness. The relevant one is
Cognitive Fit: "use different visual dialects for different tasks and
audiences", against the prevailing practice it labels *visual monolinguism* —
"use a single visual representation for all purposes (usually the case in SE)".
The principle is grounded partly in expert–novice differences and the
expertise reversal effect
(Moody, "The 'Physics' of Notations", *IEEE Transactions on Software
Engineering* 35(6):756–779, 2009. The article is paywalled and was **not**
read for this survey; the principles above come from a
[lecture presentation of the paper](http://www.cs.toronto.edu/~chechik/courses18/csc2125/week3/Moody-presentation.pdf),
so principle names and framing are reported and nothing is quoted from the
article itself.)

**The expertise reversal effect is the strongest mechanism available for "fails
at both".** It is a well-replicated instructional-design result: support that
helps novices can *harm* higher-knowledge learners, because "what is essential
for novices could be redundant for more knowledgeable learners, so integrated
formats that are effective for novices could be ineffective for more expert
learners". It has been "replicated in many studies with a large range of
instructional materials and participants" as either a full or a partial
reversal
([Kalyuga 2007](https://link.springer.com/article/10.1007/s10648-007-9054-3);
[Kalyuga and Renkl 2009](https://link.springer.com/article/10.1007/s11251-009-9102-0);
both read as abstracts and secondary summaries, not in full).
**This is an inference when applied here**, and the gap should be stated
plainly: the effect concerns the same learner at different expertise levels
studying the same material, not two different audiences reading one artifact at
the same time. It supplies a mechanism, not a result.

### Participation: what the evidence actually measures

**Involvement helps on average, unreliably, and the review that says so is
worth reading for its caution.** A systematic review selected 87 empirical
studies published 1980–2012. Its abstract reports 52 positive, 12 negative and
23 uncertain; its discussion reports 59 positive (68%), 7 negative and 21
uncertain — the two tallies disagree and both sum to 87, so the paper is
internally inconsistent on this point and the discrepancy is recorded here
rather than silently resolved. The conclusion is unambiguous: user involvement
"is a double edged sword and if not managed carefully it may cause more
problems than benefits". And on the question of *where* to invest: "we did not
find any convincing or compelling evidence in the empirical studies that would
illustrate with absolute certainty which phase(s) of SDLC user involvement is
most effective"
([Bano and Zowghi 2015](https://opus.lib.uts.edu.au/rest/bitstreams/ca6ce2c6-eb32-4191-a616-5732b7ff2981/retrieve),
*Information and Software Technology*).

**Participation mostly buys satisfaction; buying quality requires designing for
knowledge transfer.** A meta-analysis of 82 studies from 1974–2007 found
"stronger positive results for attitudinal/behavioral outcomes than the
productivity" outcomes, and its recommendation is a direct design instruction:
"if the productivity and quality of the system is the focus then user
participation should be designed to facilitate and increase the domain
knowledge of the development team"
(He and King's meta-analysis, cited and summarised as reference [3] of
[Bano and Zowghi 2015](https://opus.lib.uts.edu.au/rest/bitstreams/ca6ce2c6-eb32-4191-a616-5732b7ff2981/retrieve);
the meta-analysis itself was not retrieved, so it is reported here at one
remove).

**A measurement warning that applies to the whole question, and to any pilot
that tries to answer it.** A randomised controlled trial had 16 experienced
developers complete 246 tasks in mature repositories on which they averaged
five years of prior experience. With AI tools allowed they took 19% *longer* —
having expected a 24% speed-up, and still believing afterwards that they had
been sped up by 20%
([METR 2025](https://arxiv.org/abs/2507.09089)). The self-report inverted the
measurement by roughly 39 percentage points. Since satisfaction and perceived
usefulness are exactly the measures the participation literature relies on,
**any claim that better presentation increased participation must be
instrumented behaviourally rather than asked about.**

### LLM assistance in the capture layer: promising, with a specific hazard

**Machine revision of stakeholder statements improved them on every measured
dimension.** A guided survey with 26 participants (16 low-experience, 10
high-experience) produced 130 paired comparisons of original versus
LLM-revised user stories. Revisions rated better on alignment with intent,
readability, reasoning and unambiguity (all p < .001, r ≈ .88), and "no
participant rated any dimension below the neutral midpoint". Beyond wording:
"In 43% of all evaluations, participants indicated that the LLM revisions
correctly surfaced additional aspects they had not originally mentioned", and
35% said the revisions improved their own understanding of the requirement.
Only 5% (7 of 130) were seen to introduce factual or logical inaccuracies, and
20 of 26 reported no loss of information
([Supporting Stakeholder Requirements Expression with LLM Revisions
2026](https://arxiv.org/html/2601.16699)).

**The hazard is in the subgroup analysis and it is structural.**
"high-experience participants … also identified a greater number of
content-related errors (six cases compared to one among low-experience
participants)". The reviewer least able to detect semantic drift is the
non-expert stakeholder — who is precisely the person a capture layer exists to
serve, and precisely the person who would be asked to sign off an
LLM-elaborated statement of their own intent. The authors' own limit is that
"LLMs should not replace stakeholders or practitioners in RE processes but
rather complement them". Sample size is small, participants were German
students and professionals, and the domain was IDEs, which the authors flag as
established rather than novel.

### What good tooling looks like, from the cases that worked

Assembled from sections 2, 3 and 5 rather than from a study of tooling as such:

- **A checker the author can run, whose failures are readable.** Structurizr's
  MCP integration lets an LLM "run validation and parsing, read the error
  messages, and fix its own invalid output".
- **Plain text under version control.** Every artifact in this corpus that was
  still alive years later was plain text in a repository; the tool-format
  artifacts (MDE models, vector-store memories) were not legible without their
  tool. See section 7.
- **One source, several audience-specific projections** — C4's levels, SKOS's
  role-tagged notes — rather than several maintained artifacts.
- **Enough tooling to fail, since nothing else made adherence hold.**
  McMillan's null results plus the ShardStore anti-drift devices point the same
  way: the mechanism that keeps an artifact true is an automated reader that
  can reject it.

## 7. Walk-away checks

For each prior system: **with the tool uninstalled, what is still legible?** It
is the cheapest invariant to demand and the most discriminating one in this
corpus — it sorts the architectures that can be borrowed from the ones that
cannot, and it correlates with survival better than any quality judgement.

| System | With the tool gone | Verdict |
| --- | --- | --- |
| Architecture decision records | Everything. Plain Markdown prose; the format is a heading convention and no tool ever read it. | **Full.** Nothing to uninstall. |
| Ubiquitous language (DDD) | Everything, and it also survives inside the code's own identifiers. Its weakness is that 25.3% of projects never wrote it down at all. | **Full**, when it exists. |
| Agent context files (`AGENTS.md`, `CLAUDE.md`) | The whole instruction text, readable as a contributor guide. What is lost is not legibility but *effect*: nothing applies the rules, and nothing ever confirmed they were applied. | **Full for reading, nil for enforcement.** |
| Gherkin `.feature` files | The scenario prose, which is genuinely readable. The step definitions are ordinary code and lose their referent; the executable claim dies entirely. | **Partial.** The half the business reads survives; the half that made it true does not. |
| Structurizr DSL | The `.dsl` source is plain text and a human can follow the model. The rendered views — the artifact non-technical readers actually engaged with — are gone. | **Partial.** Source legible, projections lost. |
| TLA+ / PlusCal specifications | The `.tla` file is text, but only to a reader who knows the notation — and the paper's own authors declined to print any of it for fear of putting readers off. The model checker's verdicts live nowhere in the file. | **Expert-only, and the proofs are lost.** |
| ShardStore reference models | Everything, and it is not even a separate artifact: the models are Rust in the repository, readable by anyone who reads the implementation. Property harnesses degrade to ordinary unreadable-but-present test code. | **Full.** The strongest in the corpus, and by construction. |
| MDE / UML tool models | Typically nothing a human can use. Proprietary model files need the tool; the generated code survives and becomes the only truth, which is exactly the Telecom Company's end state. | **Nil.** |
| Event storming / example mapping | Nothing, unless someone transcribed the wall. The artifact is a canvas of stickies, physical or hosted, and the value the studies measure is the shared understanding in people's heads rather than the residue. | **Nil as an artifact.** |
| Agent memory stores (Mem0 and kin) | Nothing. A vector index or database is not legible without the tool that wrote it, and MemDelta shows the *stored* content is a weaker basis than retrieval over the original record anyway. | **Nil.** |
| Spec Kit specification sets | All 2,577 lines of Markdown, per feature. Legible, and per its trialist of rapidly diminishing value once the feature ships. | **Full, and possibly not worth it.** |

**Inference from the evidence:** the walk-away check and the survival evidence
agree, and no other property in this survey does that. Every artifact that was
still being maintained years later scored Full; every artifact scoring Nil was
either abandoned or became a liability. The formats that scored Partial are
precisely the ones whose two halves came apart.

## 8. Does the two-tier hypothesis survive?

The commissioned hypothesis: *formal specifications are poor human artifacts
and good LLM artifacts, so the capture layer and the formal tier are two layers
rather than one notation; a format attempting both fails at both.*

**On its own seam — human ↔ formal — the evidence supports it, and from
unusually credible witnesses.** Two of the three sharpest citations come from
people arguing *for* their own formal method: Cucumber's creator reporting that
"Cucumber is a development tool, and therefore many business analysts won't
touch it with a ten feet pole", and AWS's advocates omitting every specification
snippet from their paper because "their unfamiliar syntax can be off-putting to
potential new users". The "fails at both" clause has a population-level
measurement behind it: 44% of MDE practitioners found UML too complex while 31%
found it not powerful enough. And Moody's cognitive-fit principle names the
single-notation practice — *visual monolinguism* — as a design flaw rather than
a neutral choice.

**But the failure mechanism the evidence actually shows is not readability. It
is arbitration.** The Gherkin survey's practitioner put it best: refactoring
scenarios to remove duplication "causes the requirements and their
understanding to change from what the Product Owner wants". Nothing there is
about syntax. One artifact had two obligations, only one of them had an
automated enforcer, and the enforced obligation silently rewrote the other. The
Telecom Company case is the same shape with the tiers one level lower. This is
a stronger reason for two layers than readability is, because it survives the
objection that stakeholders could simply learn the notation — they could, and
the artifact would still be arbitrated by whichever obligation can fail a build.

**On the adjacent seam the hypothesis is contradicted, and this is the result
the arm should not miss.** ShardStore deliberately fused its formal tier into
the implementation language, against explicitly better alternatives, and gives
its reason as drift avoidance: separate specifications end up "languishing as
separate expert-written artifacts". So *one notation for two purposes* was the
right answer there. The reconciliation is that the two purposes were held by
the same population — engineers who read Rust — whereas the capture seam's two
purposes are held by different populations. **Inference from the evidence:** the
tier boundary belongs where the *audience* boundary is, not where the
formality boundary is. Where one population owns both purposes, fusing is
better; where two populations do, splitting is better. That reading explains
every case in this survey, including the ones that disagree.

**Three findings qualify the hypothesis rather than confirming or refuting it,
and all three are actionable.**

1. **Two layers are not the cost; two *unchecked* layers are.** Section 3's
   anti-drift enumeration is uniform: every specification that stayed current
   was on the critical path of ordinary work. A two-layer design whose formal
   tier nothing checks has bought the cost of the seam and none of its value.

2. **The capture layer's value is realised at capture time, not at
   consumption time.** MDE's practitioners were unanimous that the benefit was
   explicit architecture rather than code generation. AWS report insight "without
   ever going as far as a full specification or model checking". The LLM-revision
   study found 43% of revisions surfaced aspects the stakeholder had not
   mentioned and 35% improved their own understanding. If the elaboration's
   value is mostly upstream of the elaborated artifact, then a partial, lazy
   elaboration is not a compromise — it is where the returns are.

3. **The reviewer least able to detect drift is the stakeholder.** In the only
   study measuring it, high-experience participants caught six content errors to
   the low-experience group's one. A two-layer design that has an LLM elaborate
   the capture layer and asks the non-technical author to confirm it has put its
   weakest instrument at its most dangerous seam. This is a specific hazard the
   hypothesis does not anticipate and it needs a specific answer.

**Verdict.** The hypothesis survives the evidence at the seam it was aimed at,
for a better reason than the one it gives. It is contradicted at the
formal ↔ code seam, which it does not claim. Nothing found here recommends
abandoning the two-layer position; several things recommend restating it as an
*audience* boundary and pairing it with a checker, because a two-layer design
without one has every failure mode in section 2 available to it. Per the root
brief, `specification-capture-k6` owes the adversarial move and the stated
verdict; this section is one survey's reading, not that adjudication.

## Search silence and limits

Recording the absences, because each one is a confidence signal and each one
stops a later reader re-running a fruitless search.

- **No study was found that measures whether non-technical stakeholders
  actually read specification artifacts.** This is the survey's most important
  silence: it is the load-bearing premise of Gherkin, C4, example mapping and
  every "living documentation" claim, and the only evidence found for it is the
  tool authors' own assessments and one proxy — the Gherkin contributor
  measurement (6 to 15 spec-file contributors in repositories with up to 736
  contributors), which suggests the answer is unflattering without measuring it.
- **No study was found relating presentation or tooling *quality* to
  stakeholder *participation*.** The adjacent literature measures comprehension
  and decision quality (colour and graphical presentation experiments; a
  103-participant comparison of ERD, natural language and class diagrams for
  documenting stakeholder statements) or satisfaction (the involvement reviews).
  Question 6's second half should be treated as unanswered. Given METR's
  39-point perception gap, an in-house answer would have to be instrumented
  behaviourally rather than surveyed.
- **No primary post-mortem was found for DSL maintenance outliving DSL
  benefit.** Two candidate empirical studies — Hermans, Pinzger and van Deursen
  on DSL success factors, and Kosar et al. on DSL usability in maintenance —
  were behind ResearchGate 403s and paywalls and were **not** read; they are
  named here as the obvious next targets rather than cited. What was reachable
  was secondary summaries only, and section 2 says so in place of a finding.
- **No rigorous longitudinal or controlled evaluation was found for event
  storming or user story mapping as durable notations.** Example mapping is the
  exception, and its own authors describe the field's condition: these
  techniques are "frequently presented by consultants and agile trainers" while
  "the effectiveness of these techniques is not evaluated in a rigorous
  manner".
- **No study was found on whether ADRs are updated, superseded, or go stale.**
  The 4,300-ADR content study states this explicitly: it "contains no analysis
  of ADR updates, supersessions, staleness, or abandonment rates", which "were
  explicitly outside the research scope". So ADRs are known to be widely
  adopted and known to under-record rationale, and their decay over time is
  unmeasured.
- **No empirical work was found on glossary shareability.** The requirements
  engineering glossary literature is overwhelmingly about *automated term
  extraction* rather than about whether a maintained glossary stays usable by
  the people it is for. The DDD characterisation does not measure whether a
  glossary artifact exists or is shared with non-developers. Section 4's answer
  is therefore drawn from a standards document plus one transferred measurement,
  and is the least empirically grounded section here.
- **No empirical comparison was found of projection against duplication** —
  maintaining one source with audience-specific views, versus maintaining two
  artifacts. C4 and SKOS establish that the pattern exists and is buildable;
  neither establishes that it is better.
- **The agent-memory literature is in open disagreement and largely
  vendor-published.** Section 5 records rather than resolves it. MemDelta's
  demonstration that a single uncontrolled variable reverses a headline
  comparison means no figure in that subfield should be cited without its
  baseline configuration.
- **Two structural biases run through the whole corpus and inflate every
  positive result.** Adoption surveys reach practitioners who persisted — the
  MDE authors state that those "who have tried MDE but failed were probably not
  accessible" — and repository mining excludes dead projects, as the Gherkin
  study's own filter did. The failure rates in section 2 are floors.
- **The LLM-era evidence is weeks-to-months old and dependent on model
  versions.** The AGENTS.md, catastrophic-remembering, instruction-adherence
  and memory results are 2026 preprints, several without visible peer review,
  and every one is tied to specific model and harness versions. They are the
  best available answer to question 5 and they will need re-reading rather than
  re-citing.
- **This survey read no sibling output and made no attempt to align with one.**
  Per the brief, `specification-capture-k5` runs the same questions over an
  independently chosen corpus and `specification-capture-k6` performs the
  reconciliation and owes the adversarial move. Disagreement between the two
  surveys is the product being bought here, not a defect.

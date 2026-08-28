# The routing table

One row per **kind of question**, naming the instrument that answered it and the
evidence that says so. Keys like `[003]` name entries in the log this skill was
distilled from (`docs/formalism-findings.md` in `Linkuistics/grove`);
`[synthesis]` names its closing sections and `[c1]`–`[c5]` the adjudication in
`docs/candidate-lessons.md`. The skill's provenance block defines all three.

Read it as a lookup, not as a reading order. **Empty and *untested* rows are
honest**; a row with no evidence behind it would be a guess, and the whole point
of accumulating before distilling was to have none.

## Contents

- [Routing itself](#routing-itself)
- [Questions that go to a formalism](#questions-that-go-to-a-formalism)
- [Questions that are free](#questions-that-are-free)
- [Reading a model on the way into an implementation](#reading-a-model-on-the-way-into-an-implementation)
- [Judging an instrument that reports on itself](#judging-an-instrument-that-reports-on-itself)
- [Consumer-side questions](#consumer-side-questions)
- [Named but never run](#named-but-never-run)

## Routing itself

| question | instrument | evidence |
|---|---|---|
| **which of the two am I holding?** | count the states the property mentions | one → structural; two or more → behavioural. Mechanical, applicable before choosing. Supersedes shape-versus-operation, which mis-sorts "no key is ever reissued" `[003]`. **Zero → neither**: a question about what a verb prints mentions no state at all `[018]` |
| **how much will a model be worth to this work?** | the fraction of it inside the model's stated scope | 5 of 23 tests named a claim where the model barely reached; 28 of 42 where the subject *was* the model's. Legible from the handoff block before the work starts `[009]` |
| **what will I have to get right unaided?** | the model's own recorded misses | one sentence — *walk order is unmodelled* — located every line of undefended prose in a leaf before any code existed `[006]` |
| **what refusal will I need that no model can name?** | the model's stated idealisations, read as a list | no strings, no bytes, unbounded integers — three refusal classes, one per idealisation, and the model lists them itself `[009]` |
| **what will the findings be worth *to*, on code that already ships?** | ask whether a finding here can be a failing test at all, or only a catalogue row | *no derived test* is the modal outcome — about four material findings in five — because the model says the shipped behaviour is fine and the catalogue never described it. The consuming phase is documentation, not implementation `[synthesis]` |

## Questions that go to a formalism

| question | formalism | evidence |
|---|---|---|
| **structural** — is this shape coherent, and can it even represent what I need? | Alloy | two defects found unprompted, two more sharpened, and a tree satisfying *every stated invariant* with an entire subtree invisible to every traversal `[002]` |
| **reachability** — is every thing I must name reachable from the constructors I have? | Alloy | both blocking defects across the prose baseline and the structural model were this question; it is the one Alloy answers best `[001, 002]` |
| **behavioural** — does this operation preserve the invariant, from any reachable state? | Quint | confirmed: all three invariants Alloy could not *state* were checkable, and six further defects came out of the same file `[003]` |
| **interruption** — what does a crash halfway through leave behind? | Quint | the ordering rule's real payoff, and `promote`'s unavoidable transient duplicate, are invisible to every other method in this log `[003]` |
| **ordering** — does the order of these steps matter, and why? | Quint | the stated reason was wrong and the real one was unwritten; a model in which an operation can stop halfway is the only thing that separates them `[003]` |
| **dead branch** — is this refusal reachable at all? | either, via a reachability witness | one refusal fires in 0% of traces on any tree the library builds. Sampled, so evidence and not proof `[003]` |
| **universal** — does this hold for all inputs, not just those a checker reached? | Lean *(untested)* | — |
| **my claim is about more than one operation** | state it over the **trace**, not over one action's before/after pair | a claim accumulated from one operation's pair was silently correct; restated over the trace — same tool, same model — it produced a counterexample nothing else had. Pair-shaped is right whenever the subject *is* one operation `[synthesis]` |
| **what would have to be observed for this to fail?** | if the answer names a constant you set, the claim is a restatement | one property was discharged by the model's own dial spelled backwards, and no grain check detects that `[synthesis]` |
| **comparing two behavioural formalisms** | control for statement shape and for what discharges the property | otherwise the comparison measures the modeller. The one arm that depended on eventualities had nothing to run on: the shared catalogue deliberately contains none `[synthesis]` |

## Questions that are free

No formalism, no run. These retired more work per minute than anything else here.

| question | move | evidence |
|---|---|---|
| **already guaranteed** — can the target language forbid this outright? | ask before modelling | two of eight structural claims were free from the type system `[002]`; folding three `Option` accessors into one view removed an obligation from every consumer for one trait method `[004]` |
| **partial discharge** — the language forbids *some* of this | split it into two named obligations | the qualification survived in the file that made the claim and in none of the three artifacts that repeated it; the unqualified half was a real defect `[005]` |
| **an obligation with nowhere left to go** — can the *interface* forbid this? | read the obligation against the argument surface, not just the type system | *the species follows from the parts* became a verb-grammar rule and one refusal became unreachable by construction `[018]` |
| **stated isomorphism** — are both directions written down? | ask | the canonicity gap was half an isomorphism, and asking costs nothing `[002]` |
| **enforceable obligation** — the type system cannot forbid this, but can the library check it? | ask, before writing it down as an assumption | six of seven obligations are genuinely uncheckable; the seventh costs one string test at two boundaries and turns an escape from the locked tree into a refusal with recovery advice `[011]` |
| **an opaque sort** — what does the model assume about comparing this? | read each atom against the target language's bound | `Eq`/`Ord`/`Hash` are all coarser than a model's identity; a conforming domain lost every promotion to a spurious refusal `[015]` |
| **already arranged** — did the model check this, or make it true? | ask | subtree preservation is true by construction of the state shape, and such a model is indistinguishable from one that verified it `[003]` |
| **out of scope** — the model says it excludes this; what does that oblige? | the exclusion list is the worklist | three of five defects sat in the three domains the handoff names as excluded, written two leaves earlier `[011]` |
| **two questions, one piece of code?** | read the two model functions side by side | two plan functions differ in one operand — a constant against a value read off the target — so the shared part is one trait call and wrapping it would be shallower than the expression. The model turned a judgement call into a reading `[016]` |

## Reading a model on the way into an implementation

| question | move | evidence |
|---|---|---|
| **deriving tests** — what do I actually run against the implementation? | the model's must-be-reached witnesses | four Alloy witnesses became executable tests; not one `check` translated into anything runnable `[004]` |
| **reading order** | the witnesses before the invariants | the invariants confirmed what the document already said; the witness *pair* discriminating one outcome by a predicate turned one refusal into two messages `[012]` |
| **an unenforced invariant hiding in plain sight** | the model's explanatory comments | *`Remove` never appears in a forward plan* was a comment, and became two types that make it unrepresentable `[009]`. A second instance carried arithmetic stated nowhere else `[014]` |
| **model-claim-to-test** — does this test still say what the claim said? | quote the claim's predicate beside the assertion | two tests named a claim and checked a weaker property; both drifted toward the property that was easier to observe `[005]` |
| **a message derived from a model** — may this refusal say what it says? | name the carried value behind each clause | a predicate that *discriminates* a case does not *characterise* it; the comment beside it did, and the message transcribed the comment `[013]` |
| **an enumerated refusal** — can an argument actually produce each case it names? | name the argument that reaches each case | half of one documented refusal was unreachable. A model declines such a case by having **no witness**, which is invisible in a way a failing claim is not `[014]` |
| **one property, how many mechanisms?** | a mutation control per mechanism, not per property | *claims its destination* is an exclusive create for one effect and a look-then-rename for another; eight controls passed while the second was undefended `[009]`. A later reader counted three, not two `[010]` |
| **a rule whose payoff is a state nothing observes** | the witness live in only one instance — it names the control | a lone "every state is distinct" assertion passes under any shift order; the one-instance witness says to replay the same landings the other way `[012]` |
| **an order that looks like a rule** — could it have been the other way? | look for the model's counterfactual instance; its absence is the answer | one operation's order has an alternative instance and another's has none, because the second effect lands in the level the first creates. A forced order is a consequence — assert the forcing, not the order `[014]` |
| **a state the model reports as rare** — will this be expensive to test? | no — ask what steers into it, not what stumbles into it | 0.07% of traces under random simulation; one line of a fault seam in the implementation `[014]` |
| **an edge case the model names for an operation that does not exist yet** | build it into the layer beneath, with a test naming the witness | the operation later landed with **no code**. Written the other way round it is a layer-crossing defect, where the algebra proves a plan applicable and the applier refuses it `[016]` |
| **what may a refusal carry?** — which field is derivable from the others? | count the model's variants for the sort | two positioned species, so *the entry is a leaf* already says *the parts make a node*; two fields that restate each other are two fields that can disagree `[013, 016]` |
| **when the discriminating case belongs to a later leaf** | build it by hand now | otherwise the decision sits untested until that leaf arrives and a regression in it is silent `[009]` |

## Judging an instrument that reports on itself

| question | move | evidence |
|---|---|---|
| **did it run at all?** | must-be-reached claims beside the must-hold ones | a JVM too old made Alloy print nothing, which the runner read as thirteen unfired witnesses and seven holding checks. Every witness failing at once is a signature no real defect produces `[003]` |
| **does this test test anything?** — the suite is green, but was it ever red? | deliberate mutation of the implementation | six breakages, each firing exactly the expected tests; the only evidence for two claims made in prose. Minutes, no tooling, any suite `[006]` |
| **which independently wrong implementation does this control still let through?** | mutate the assumption, not the happy-path structure the test author chose | a stubbed lock proves presence, not identity; one inverted sort key proves ordering matters, not that all later keys are in the stated precedence `[007]` |
| **a comparator with `k` keys** | `k` controls, each arriving out of order | a tie-break fixture supplying entries in the order the rule prescribes cannot distinguish the rule from a stable sort `[008]` |
| **a suite that reports its own coverage** — does *exercised* mean the case that could fail it ran? | one mutation per reported obligation | a conformance kit read green over ten real listings while the grammar's canonicity check was disabled, because nine other listings parsed `[020]` |
| **filling a fixture for someone else's checker** | read its parameter documentation as a checklist of what to supply | the kit asked for "any near-miss the grammar is meant to refuse"; the fixture was filled from what a healthy tree holds, and the near-misses fell out `[020]` |
| **a claim about a guard** — will this change trip the detector? | plant the violation and watch it fail | settled in ninety seconds, against a reading of a token scan and its carve-out `[018]` |
| **a boundary guard that reads source text** | use the language's own lexer | a hand-written comment stripper missed four of ten literal forms; a partial lexer is a second implementation of the hardest part of the language `[008]` |
| **is this per-test convention countable?** | count, then read whatever the count could not classify | a regex left 22 of 204 tests unclassified and every one was labelled. A count over-reports one way and under-reports the other, so an account is counted **and** read `[011, 019]` |
| **a finding from a review** | run its counterexample before believing it | one finding's stated mutation cost two minutes and was wrong, while the real defect it pointed at was worse `[008]` |
| **a case the platform makes untestable** | assert which fact is true on the host, never skip | APFS refuses non-UTF-8 filenames, so a halting branch is unreachable on macOS; a skipped test reports what a passing one reports `[006]` |
| **the bytes did not change** | the inode, not the content | bytes read and written back identically compare equal; an unchanged inode says nothing read them `[016]` |
| **a written law nobody holds the code against** | an adversarial reader briefed to attack one named judgement | five of six findings, including both trait-shape defects, which neither model could reach: the models supplied the law and a reader supplied the comparison `[005]` |
| **it went red — but for the reason I claim?** | the isolating run: each part of the claim as its own check over the same world, the others shown to stay green | three controls killed their obligation through the wrong mechanism and read identically from outside — a defect in the coverage sweep, a second half the world could not reach, and an operand that was never exercised `[c1]` |
| **is this narrowed module still evidence?** | check both halves by hand — a strict subset, *and* still reaching the subject | five modules were narrowed past reachability and reported green over nothing. A mutant module's environment is part of its control `[c3]` |
| **this family cannot meet this catalogue row** | run the deeper attempt; never cost it in prose | a state-count argument said the check could not be run. Run, the state was reached in fourteen states and 8.9 s — contradicted twice inside the file that made the claim `[c3]` |
| **does this module falsify a claim it does not declare?** | sweep the whole claim library against it | a module carries only its own commands *by design*, so it hides the rest; two sweeps of one candidate returned four unlooked-for findings and no clean result. Run the sweep and record it — repairing what it finds is a separate and possibly larger cost `[c4]` |
| **a claim stated over facts my own steps record** | ask for the control that makes a step **omit or falsify** the fact | the flags are written by exactly the steps the claim is about, so it is one edit from satisfied-by-construction — and *record it at the establishing step* recreates the hazard one level down `[c2]` |
| **a witness that calls the definition it is about** | write its conjuncts out inline and freeze them, then rename | one witness went from reached in 3,410 traces to unreached in 8,000 samples across its subject's repair, with no error and no red `[c5]` |
| **a witness that never lands** | a search dial that removes no behaviour, plus focused instances built on it | over a ~20-step transaction with every environment action enabled, an unfocused search reaches the end with probability `(1/k)^20`; every claim past the commit would have read green on a witness that never landed `[045]` |
| **budgeting a bounded model check** | price the **depth**; treat the property count as free, and quote the depth beside the result | 3 invariants at depth 3 took 373 s, 61 at depth 4 took 377 s — and a depth-4 result on an eleven-step path verifies the beginning of the transaction and nothing after it `[045]` |
| **a pre-registered measure** | an instrument that reads the **ledger**, not the obligations | two of eight measures survived as specified; the decay landed exactly at a session handoff and nothing detected it, because the runner checks obligations and each reviewer was checking the model `[synthesis]` |
| **two models, one shared catalogue** | freeze the document both descend from, before the first model | four sessions of one column edited the shared catalogue before the other column existed, so its uniqueness tags there are true and uninformative `[synthesis]` |

## Consumer-side questions

For a consumer building on a library that already has models. These are where a
refactor's evidence comes from `[025]`.

| question | move | evidence |
|---|---|---|
| **a design document's reachability table** | write it at design time, then **transcribe it into the suite** | six refusals marked reachable and seven not, before any argument parsing existed; every reachable one got a test naming its witness, and not one was reachable after all `[019]` |
| **an inherited reachability row** — is this row I did not write actually right? | build the row's fixture and write its filenames out in full | the occupying name and the live one must share a key, so the row was unreachable — and it could not have been checked by re-reading `[022]` |
| **a row whose justification says *we keep our own check in front of it*** | it is not a reachable row | a table over a layered surface must say *reachable by an operator* or *reachable by the algebra*, and is worth nothing if it silently means both `[023]` |
| **a refusal whose reachability depends on the order effects are folded** | check it against the interpreter, not the operation signature | the only refusal that is a property of the *plan* rather than of the arguments, and the only one whose row was wrong twice `[022, 023]` |
| **how often will an inherited table be wrong?** | at a rate that does not decay | four leaves, four corrected rows, four *different* mechanisms. Mark such a table *checked by X against revision Y*; it cannot be settled once `[024]` |
| **an operation named by key** | add the row for *two entries share the key* | resolution answers with whichever entry the walk reaches first, and walk order was one of the model's recorded misses. A verb aimed by path marked the other twin and reported success `[022]` |
| **a consumer whose preconditions are stronger than the library's** | count the checks it needs for its own reasons before counting the library's refusals | four preconditions the library cannot see retired four refusals, cutting a ten-variant question to one. Make the pre-check the library's own predicate read off the snapshot, or it is a second wording at the level of code `[021]` |
| **whose words does this error speak?** | compose one example message in the *consumer's* own vocabulary at design time | a library refusal says *a leaf, which holds nothing* to a tool that has lessons and modules. A design that promises a rendering should render one `[019]` |
| **an inherited claim that a message is wrong in my words** | put each clause beside the glossary entry it lands on and mark it true or false | four of six clauses were true; the defect was the *operation the message names*, not the nouns — and three briefs had repeated a summary that named the wrong half `[021]` |
| **an error variant describing a persistent state** | ask which of your own commands can meet that state later | the recovery advice for the one state the library can damage was addressed to a reader nobody had: never the process that made it, always a later command `[024]` |
| **is it gone?** — this module or surface has been removed | enumerate then classify, with a positive and a cross-tree control | a clean build discharged the *linkage* claim and said nothing about prose; enumeration found thirty-three stale references in twelve files, not one a `use` line `[025]` |
| **a predicate was just tightened** — what else was built against the loose one? | grep the *test* side for filters encoding the old rule | a duplicated leniency is free while the parser is lenient and silent in both directions the moment it is not `[025]` |
| **a claim about the repo itself** | read the summary and overview layers as a separate pass | a token sweep rooted at `src/` can never reach a root manifest, and a stale claim need not contain a name `[025]` |
| **two live implementations of one contract** | an equivalence test, while both sides exist | cheaper than the review that would otherwise stand in for it, and it fails rather than reassures. Such tests die with the old side, so the evidence has to be spent before the contract stage `[025]` |

## Named but never run

| question | instrument | status |
|---|---|---|
| **grammar** — can two spellings name one thing? | property-based testing: `format(parse(f)) == f` over generated names | *untested.* Two domains met the question; neither reached for it. What was used instead — one adversarial sample plus a mutation — establishes one spelling where the law would establish all of them `[004, 020]` |
| **order stability** — does this answer depend on the order the input arrived in? | property-based testing over permutations: `walk(shuffle(l)) == walk(l)` | *untested.* Found by composing two model premises instead; the direct instrument was named and not run `[006]` |

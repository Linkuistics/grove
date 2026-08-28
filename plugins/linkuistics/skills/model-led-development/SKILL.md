---
name: model-led-development
description: Route a specification question to the instrument that answers it — Alloy for counterexample structures, Quint for counterexample traces, a type system or an argument surface for free, and often no model at all — then let the checked model lead the implementation. Use when writing a design document or specification, deciding whether a property is worth modelling before building it, implementing against a model that already exists, or distrusting a green suite, a coverage report, or a reachability table.
harnesses: [any]
---

# Model-led development

A **checked model guarantees consistency with itself, never that the right
properties were stated.** So a model does not remove review — it relocates it,
from hundreds of lines of prose to roughly fifteen invariant statements. That
sentence is the whole reason this skill exists and it is also its own honesty
clause: what follows is prose about how to write specifications, and nothing
checks it.

> **Provenance.** Every rule here is traceable to a numbered entry in a log kept
> across two modelling campaigns — `docs/formalism-findings.md` in
> `Linkuistics/grove`. **Entries 001–025** extract a tree-on-disk library under
> two formalisms — Alloy on its structure, Quint on its operations — with the
> model written *before* each operation was implemented. **Entries 026–048 and
> the synthesis closing the log** put Alloy 6 *and* Quint on the same
> behavioural questions about a workstream tool's own protocols, against a
> catalogue of 129 obligations matrixed across both families — 258
> `(family, obligation)` cells, of which **256 are complete and two are declared
> Alloy gaps** (`TT-24.c`, `TT-24.d`) — with the implementation **already
> shipped and green**, which changes what a finding is: usually a correction to the
> specification rather than a defect in the code.
>
> **Three key forms, each naming where to check a rule.** `[003]` is a log
> entry; `[synthesis]` is the log's closing sections, from *What is being
> compared* onward; `[c1]`–`[c5]` are the five candidate lessons adjudicated in
> `docs/candidate-lessons.md`, which re-opened every model file it cites rather
> than trusting the sessions' own accounts of them, and which weakened or
> falsified some of what those sessions believed. The six-field entry format is
> in [`references/keeping-a-log.md`](references/keeping-a-log.md), and the full
> evidence table is in
> [`references/routing-table.md`](references/routing-table.md). **Rules with no
> entry behind them were not written**, and the two instruments the log names
> without ever running are marked *untested* wherever they appear.

## Route before you reach

**Count the states the property mentions.** That is mechanical, it can be applied
*before* choosing a tool, and it is what a whole workstream's routing reduced to
`[003]`:

| states mentioned | kind of question | instrument |
|---|---|---|
| **zero** | what does this print, in whose words, on which stream | **neither** — see *Questions no model can hold* |
| **one** | is this shape coherent, can it represent what I need | **structural** → Alloy: counterexample *structures* |
| **two or more** | does this operation preserve it, from any reachable state | **behavioural** → Quint: counterexample *traces* |

Count states, not verbs. The earlier version of this rule sorted by *shape versus
operation* and mis-sorted "no key is ever reissued" — which reads structural and
quantifies over every past state `[002, 003]`.

**This table rests on the first campaign and was not retested by the second**,
which deliberately put *both* tools on behavioural questions to stress the split
rather than confirm it `[synthesis]`. What that campaign supports is narrower and
sits below: the two families' blind spots do differ, on two of three scopes — but
what separated them was **the shape a property was stated in**, not temporal
operators against guarded actions. So route with this table, and do not read a
two-family comparison as evidence for it.

### Six free questions, before any model

Each costs no tooling and no run, and each has retired real work.

1. **Does the target language already forbid it?** Two of eight structural
   claims were free from the type system, and modelling them taught nothing
   `[002]`. One leaf later, applying it at the seam folded three `Option`
   accessors into one view and removed an obligation from every consumer, for one
   trait method `[004]`. Look for candidates in **the model's explanatory
   comments** — that is where its unenforced invariants hide `[009, 014]`.
2. **Does the argument surface already forbid it?** An argument surface discharges
   an obligation the way a type does. *The species follows from the parts* became
   a verb-grammar rule and one refusal became unreachable by construction `[018]`;
   a refusal reachable in all three of its messages from a CLI taking an ordinal
   was unreachable from a verb taking the entry whose slot is taken `[021]`.
3. **Of every claimed isomorphism, are both directions stated?** The document
   said "isomorphic" and stated `parse(format(n)) == n` only. The missing half
   meant two filenames could name one entry — one key, one position, two files
   `[002]`.
4. **Can the *library* check what the language cannot forbid?** Six of seven
   obligations were genuinely uncheckable; the seventh cost one string test at
   two boundaries and turned an escape from a locked tree into a refusal with
   recovery advice `[011]`.
5. **Is the discharge partial?** *The language forbids this* is a proof you did
   not write down. `triple()` closed *neither* and left *both* — and the
   qualification survived in the file that made the claim and in **none** of the
   three artifacts that repeated it. Split a partial discharge into two named
   obligations; never annotate it as one `[005]`.
6. **Is the language's comparison as fine as the model's?** A model's opaque sort
   is compared by *identity*; `Eq` is any lawful equivalence, `Ord` a total
   preorder, `Hash` neither. A conforming domain that compared two parts equal
   lost every promotion to a spurious refusal `[015]`. List the model's opaque
   atoms beside the trait bounds on the types they stand for.

### How much will a model be worth to this work?

**The fraction of it inside the model's stated scope — legible from the model's
own handoff block before a line is written** `[009]`. Measured as tests naming a
model claim, across six pieces of one library:

| the subject is… | naming a claim |
|---|---|
| the model's own subject (a plan interpreter, `insert`, `promote`, `rewrite`) | 30/48, 19/26, 17/24, 10/14 |
| beside it (a reading layer the model barely reaches) | 5/23 |
| outside it (a CLI) | 8/31, and none of the eight about the CLI |

Plan the test budget from that ratio, not from the model's quality `[016]`.

Two cautions on the counting. Each figure is the leaf's **final** account, after
its review — three of the four were revised upward by the leaf that reviewed them
`[011, 013, 015]`. And **an account is counted and then read**: a regex over one
crate's claim labels left 22 of 204 tests unclassified and every one of them was
labelled, in a form the pattern did not hold `[019]`.

**Then ask what the findings will be worth *to*, because on existing code it is
not the tests.** The second campaign modelled an implementation that was already
shipped and green, and **no derived executable test is the modal outcome across
every entry that records the measure**: about four material findings in five
yielded nothing to run, because the finding is that the catalogue never described
behaviour the model says is fine — there is no defect, and the correction is a
catalogue row `[synthesis]`. So a model run against existing code buys
**specification correction**, and **the phase that consumes it is documentation,
not implementation**. Budget that phase, or the model's output has no consumer.

**Two cost figures and one anti-figure, quoted rather than estimated.** Authoring
ran **0.289 h per obligation** component-scoped against **0.373 h**
system-scoped — the same direction comparing first slice to first slice, 0.19
against 0.42 — though both arms mix the cost of *building* a model with the cost
of *adding claims to one*, so neither is an estimate for a new scope
`[synthesis]`. Roughly **4.2 h** went to *wrangling* — the tool rather than the
problem — its largest single item three hours of performance cliffs,
output-format discovery and a dead end in one session `[synthesis]`. The
anti-figure is machine time: one family's column cost 6888 s of CPU against the
other's 1209 s for *more* commands, and the difference is that only one of the
two files carries integers, whose bitwidth arithmetic dominates it. **Run cost
tracks the data structure in the model, not the tool family** `[synthesis]`.

### Questions no model can hold

Recording these is as load-bearing as recording the wins: a log without them lets
a reader conclude that reaching for a formalism is at worst neutral. It is not.

- **Anything about strings, bytes, message text, argv or exit codes**, where the
  models hold no strings by design `[006, 018, 019, 020, 021]`.
- **Anything about the interval between two operations, across a process
  boundary** — a model of operations has no notion of a run *ending*, or of a
  tree being *met* rather than held `[024]`.
- **A refactor onto an already-checked library**, which inherits its models
  rather than needing new ones. Four of seven such leaves reached for no
  formalism and none was the worse for it; what they needed instead were
  consumer-side instruments `[025]`.

## Writing a model that can be read

**Write the runner before you trust the tool.** Both Alloy and Quint report
*found nothing* with **exit code 0**, so a model whose result cannot be read is
not a checked model. That cost was paid twice, once per tool, and then a third
time *on the runner built to prevent it*: a JVM too old to start Alloy printed
nothing, which the runner recorded as seven holding checks and thirteen unfired
witnesses `[002, 003]`.

**State two kinds of claim, not one.** Claims that must **hold** (`check`,
`inv_…`) and claims that must be **reached** (`run witness_…`, `wit_…`). The
second kind is the load-bearing half, four times over:

- It is the **positive control that detects a dead tool.** Every witness failing
  at once is a signature no real defect produces; a suite of must-hold claims
  alone cannot tell *it ran and found nothing* from *it never ran* `[003]`.
- It is the **test suite.** Four Alloy witnesses became executable tests; not one
  `check` translated into anything runnable — a check says the design is sound and
  hands you nothing to run `[004]`.
- It **explains why a law is there**, which the invariant list does not `[002]`.
- A witness live in **only one instance names the control** for a rule whose
  payoff no passing test observes `[012]`.

**Keep claims out of the transition relation.** Nothing the document merely
claims is a global `fact`; claims are named predicates and every command declares
which it assumes. That is what lets one file hold both the corrected design and a
reproducible witness for each defect the correction closes `[002]`.

**Model a refusal as a transition, not as a disabled action.** The tools' own
idiom is a guard, which makes a refusal simply *not happen* — and a refusal
modelled as an impossibility can never be shown reachable *or* dead. Every
operation enabled for every argument, returning an outcome, is what gives
totality a pass/fail `[003]`.

**State a property at the grain of its subject, not at the grain of an action.**
A property stated over **one action's own before/after pair** cannot discover
that its claim was quantified too widely; a property stated over **the trace**
can. One claim accumulated its evidence from a single operation's pair and was
*silently correct*; restated over the trace — same tool, same model, only the
statement shape changed — it immediately produced a counterexample nothing else
had found `[synthesis]`. Pair-shaped is not a defect in itself: twelve of that
scope's twenty-five properties are pair-shaped and each is the right grain. It is
a defect exactly when **the claim's subject is wider than one operation**.

**And a property discharged by a configuration rather than by an observation is
unfalsifiable at any grain.** One was satisfied by `not(hist.silentPark)` — the
model's own dial spelled backwards — which no grain check detects `[synthesis]`.
Ask what would have to be *observed* for the claim to fail; if the answer names a
constant you set, the claim is a restatement.

**Record what the model does not establish, in the model file.** This is the
worklist the implementation reads first `[006]`, and two of its entries were still
paying out five leaves later `[022]`.

### Six traps

| trap | what it looks like | evidence |
|---|---|---|
| **Satisfied by construction** | An invariant the model's own construction makes unreachable is indistinguishable, from outside, from one that verified it. Subtree preservation was *arranged*, not checked | `[003, 012]` |
| **A scope constant in the specification** | A bound meant to limit *exploration*, used inside a pure function of the design, does not limit the search — it changes the spec | `[003]` |
| **Checking your own plumbing** | An executable state machine must *maintain* facts a structural model can assert; checking those tests your transition relation, not the design | `[003]` |
| **Prose inside a checked file is not checked** | A witness's doc comment said "a gap in the middle"; its predicate said only `at < max`. The refusal message transcribed the comment and was wrong for every leading hole | `[013]` |
| **A self-certifying claim** | A claim stated over the model's own history, flags or classifier cannot fail — the facts it reads are written by exactly the steps it is about. **And the repair carries the trap one level down**: *record the fact at the step that establishes it* puts the flag in the hands of the same steps, so **every recorded fact owes a control that can make a step omit or falsify it**. One node hit this four times — three places in one claim's expression, and a fourth time in the repair of the first three | `[c2]` |
| **A self-calling witness** | A witness written by calling the definition it is about stops measuring the moment that definition is repaired. One went from reached in 3,410 traces to unreached in 8,000 samples the instant its subject was fixed — *"not because the defect went away but because the sentence had come to mean something else"*. No error, no red, no signal at all; the repair is to write the pre-repair conjuncts out **inline** and rename, which makes it an A/B on one world | `[c5]` |

Two sampling notes: unreachability under `quint run` is **sampled, not proved**
`[003]`; and a failure mode enabled at every step starves every interesting trace,
so put it behind a constant and give the rare instance its own sample budget
`[003]`. Cost is a cliff rather than a slope `[002]`.

**A long transaction starves its own witnesses, and the arithmetic is the whole
warning.** Over a transaction of roughly twenty steps with every environment
action enabled at every one, an unfocused search reaches the end with probability
`(1/k)^20`; at 8,000 samples **every claim from the commit onward would have been
reported green on a witness that never landed** `[045]`. The remedy is a *search
dial* — a budget, a phase set, a kind set — plus focused instances built on it,
and the dial must **remove no behaviour**: the unfocused world grants a budget no
trace can spend, at every phase, of every kind, and every property is still
checked there. A third of that model's authoring cost went to making witnesses
reachable, and two rounds of *this witness does not land, why* were findings in
their own right `[045]`.

**Quote a model-checked result with its depth, or it is the scope trap restated
as a result.** A bounded check reported no violation over all 61 properties at
depth 4 — while the protocol's shortest path from entry to a settled refusal is
**eleven steps**, so it verified the beginning of the transaction and said
nothing about the commit, either handoff, disposal or recovery `[045]`. And when
budgeting such a run: **adding properties was nearly free and depth was not** —
3 invariants at depth 3 took 373 s against 61 invariants at depth 4 at 377 s, so
58 extra properties cost almost nothing there. Read it as a bound on the
*property count*, not as an isolation of depth: that pair moves both operands at
once, and the same model took 445 s when three added constants widened its state
at no extra depth. Price the property count as cheap, expect depth and state
width to be where the time goes, and quote the depth beside the result `[045]`.

### Narrowing the world — the conditional, not the caution

> **A narrowed world is evidence about the wide one only when the narrowing is a
> strict subset that still reaches the subject** `[c3]`.

Both halves are load-bearing and this corpus violated each of them. The **subset**
property is what makes a violation found narrow a violation wide; it is
established by hand, and twice it had to be established after the fact `[c3]`.
**Reachability** is what makes a *green* mean anything, and **five modules failed
it** — each narrowed until the thing it was mutating could no longer happen: an
environment that never reaches the foreign marker its control names, a budget
under which the mutated write can never fire, a focus constant whose action menu
excludes the operation being controlled, a crash-only environment that cannot
reach the in-transaction hand edit its overlap needs, and a scenario at one entry
that hid an un-evacuated-entry defect from the session that wrote it `[c3]`. So narrowing is
a sound way to **hunt a counterexample the sampler never drew**, and an unsound
way to **read a green**. The corpus's own compression, reached independently
three times, is the sentence to carry: **a mutant module's environment is part of
its control.**

**The earlier form — *false greens are found by narrowing, not widening* — is
retired, not softened.** The first half runs five to one against, and the second
half is falsified twice: a dead control was repaired by *widening* its bound and
fired in 1.5 s; and a declared impossibility — an argument from a state-count
estimate that a check could not be run — died when someone simply ran the deeper
attempt, reaching the state in **fourteen states and 8.9 seconds**, against two
commands already in the same file that had reached both boundaries `[c3]`. The
rule the catalogue gained from it generalises past that one cell: **a modelling
language's failure to meet a catalogue row is established by running the deeper
attempt, never by costing it in prose** `[c3]`.

## Implementing against a checked model

**Read the model in this order.**

1. **The misses.** They are the specification of what you must get right
   unaided, and one sentence of them located every line of undefended prose in a
   leaf before any code existed `[006]`.
2. **The idealisation list.** Each idealisation is a **refusal class** the
   implementation will need and the model cannot name — no strings, no bytes,
   unbounded integers. Reading it in advance produces all of them at once instead
   of one at a time as the types demand them `[009]`.
3. **The witnesses.** They are the test suite and they say which cases are live
   `[004, 012]`.
4. **The comments.** `promote`'s hardest arithmetic — that a promotion consumes
   no key, so a first child takes `freshKey` and not `freshKey + 1` — existed
   nowhere but one paragraph of prose inside the model `[014]`.
5. **The invariants**, last. Three leaves running, they confirmed what the
   document already said `[012, 016]`.

**Quote the claim's predicate beside the assertion, not its name.** Two tests
named a claim and checked something weaker — `v.seen = n` became string equality,
`Malformed` became *not an entry* — and both drifted toward the property that was
easier to observe. The citation made the drift invisible; the repair costs one
comment `[005]`.

**A witness predicate is a discriminator, not a characterisation.** It says this
case is not that one; it does not say what this case looks like, and the picture
in the reader's head fills the gap silently. **A message may claim only what the
predicate that selected it proves** — so take each clause of an error message and
name the carried value behind it. The clause with no value behind it is the defect
`[013]`.

**A missing witness is invisible in a way a failing one is not.** A model declines
to support a case by having no witness for it, and nobody asks a runner about a
case they think is live. So for each case a refusal enumerates, **name the
argument that reaches it** — half of one documented refusal was unreachable
because targets are named by key and the thing it named carries none `[014]`.

**When a model excludes a domain by design, that exclusion is a worklist, not a
disclaimer.** Three of five defects in one leaf sat in the three domains the
model's handoff names as excluded, and the exclusions had been written two leaves
before the defects `[011]`. Prose covering an idealisation must be **quantified
over the property**, never written as a list of the operations that exist today —
there is no typechecker for prose, and the next operation falls off the list
`[012]`.

**Re-run the unchanged model as a control after every code change.** It is what
separates *the code was wrong* from *the specification moved*, and only the first
is a fix `[011]`. It stays cheap — twenty seconds and four minutes for the two
suites — and it is bought as a control, not as a step.

**Where a model and a test disagree, the model wins and the test changes.** That
is the premise a model-led workstream runs on; here is the qualification six
leaves of evidence actually support. Across `[011]`–`[016]` **no claim in either
model ever moved.** What moved was code, and twice a *comment* — once tightening
a witness comment that overstated its own predicate `[013]`, once recording a
domain neither model could pose `[015]`. So treat a claim that has to move as a
finding about the specification, not as a licence to edit one.

**Rarity in a model measures the simulator, not your test.** A state reached in
0.07% of traces, which needed its own sample budget to find at all, took one line
of a fault seam to reach deterministically in the implementation. Ask what
*steers* into a state, not what stumbles into it `[014]`.

**Nothing checks that the code matches the model.** Unchanged across the whole
corpus `[004, 006, 009]`, and it grows more expensive as more tests cite claims.
The only discipline that substitutes for a checker is the citation above — which
reads clean when it is broken.

## The instruments that are not formalisms

Most of what this corpus found was found by these. Detail, and when each fails,
in [`references/instruments.md`](references/instruments.md).

- **Mutation controls, in their isolating form.** Breaking the implementation and
  watching the tests go red is minutes, no tooling, any suite, and still the only
  thing that distinguishes a suite that holds from one written to pass `[006]`.
  **But going red is not enough.** A control earns a claim only when it is shown
  to kill that claim **for the reason claimed** `[c1]`, and three named controls
  were found doing the other thing while reading identically from outside: one
  turning its obligation red through a defect in the coverage sweep rather than
  through the behaviour it named; one always firing through *one half* of a
  two-part claim because its world could never reach the other half; one going
  red under both the old wording and the repaired one, *"which is evidence that
  the new operand was never exercised rather than that it was controlled"*
  `[c1]`. What establishes the link is the **isolating** measurement — **run each
  part of the claim as its own check over the same world, and show the others
  stay green.** Mutate the **assumption**, not the happy path `[007]`, and use
  **one control per mechanism**, not per property `[009]`.
- **An adversarial reader briefed to attack one named judgement** — five of six
  findings in one pass, including two the models could not reach `[005]`. See
  the sibling skill `linkuistics:doubt-driven-development`.
- **A design-time reachability table, transcribed into a suite** — worth writing
  and worth distrusting in the same breath: one table, four leaves, four
  corrected rows, by four *different* mechanisms `[019, 022, 023, 024]`.
- **Enumerate, then classify** — for any *this is gone* claim, in preference to a
  grep. Complete by construction where a pattern list is complete only as far as
  the list `[025]`.
- **An error message read as a specification** — for every error variant whose
  message describes a *persistent* state, ask which of your own commands can meet
  that state later `[024]`.
- **A coverage report needs one mutation per reported obligation** — and even
  then it is not sufficient. A conformance kit can distinguish *no samples* from
  *samples*; it cannot distinguish *samples* from *samples that pose the
  question*, and the distinction is invisible from inside `[020]`. A
  **two-direction** coverage assertion — built precisely to stop a cell being
  credited without evidence, and asserting the manifest in both directions —
  passed while one cell's evidence came from a world in which the obligation was
  **false** `[c4]`.
- **Run every module against every claim the model has, not the ones it
  declares.** A `mutant_`, `relax_` or `scenario_` instance carries only the
  commands written inside it, and that rule is correct — each exists precisely
  because some obligation behaves differently there, so inheriting the library's
  would assert the opposite of what the instance is for. The hazard is that
  rule's shadow: **a module that falsifies a claim it does not declare hides it
  by design** `[c4]`. Two sweeps of the full claim library against one candidate
  module returned **four unlooked-for findings and no clean result** `[c4]`. And
  the obligation is bounded, which the sweep's advocates usually leave out: **run
  the sweep and record what it says** — repairing what it finds may cost far
  more, and one such repair, needing a new constant in *every* module
  instantiation (2 in one file, 37 in another), was cut as its own task and then
  abandoned `[c4]`.

## What this evidence does not support

- **"A model beats prose of equivalent length at the implementation site" is not
  supported.** A pre-registered probe gave a fresh context the prose alone; it
  violated **zero** model claims on a suite with a demonstrated live control, and
  failed only on literal wordings of its scorer's own messages `[017]`.
- **What replaces it is narrower and better supported:** a model earns its keep
  **upstream**, in the prose it corrects, and a session implementing against
  already-corrected prose gets most of the benefit without reading the model. So
  the question at the implementation site is not *model or prose?* but **has this
  prose been through a model yet?** `[017]`
- **If you run such a comparison, have the measure written by a party that
  implements neither arm, from the claims alone.** Fixing the *claims* in advance
  was not enough: a test is a claim plus an assertion, and all four failures
  landed on assertion wording the scoring arm had authored `[017]`.
- **Property-based testing is the instrument this corpus names and never runs.**
  `format(parse(f)) == f` for a grammar `[004, 020]`, and
  `walk(shuffle(l)) == walk(l)` for an order-independence claim `[006]`. Three
  leaves reached for it and none ran it; that pattern is the finding, and the row
  stays *untested*.
- **The counterfactuals of a falsification do not predict the next one.** A
  counterfactual written at the moment of a falsification describes *that*
  mechanism, and across four leaves the mechanisms did not repeat once — a
  predictive record of 0 for 2, stated plainly `[024]`.
- **How many greens were sound is unmeasured, and the hit rate is not it.** No
  arm ever ran without controls, so what the second campaign measured is the hit
  rate of **looking**: ten sessions went looking for an empty green and **ten
  found one**, across fifteen named obligations plus three defects in the runner
  itself `[c1]`. That is a strong argument for looking. It is *not* a measurement
  of how much rot there was, and reading it as one is the same error the rest of
  this skill is about.
- **Nothing about either tool in general.** One subject, one pair of pinned
  versions, one team, one methodology `[synthesis]` — and the second campaign
  could not retest *the model leads profitably* at all, because its
  implementation already existed and was green, so the counterfactual was
  unavailable `[synthesis]`.
- **The two families' finding counts are not a fair race.** One column ran nine
  finish slices and four lifecycle slices to the other's one apiece, and the
  counting rule scores *what was written down as a finding*: an unrecorded
  modelling choice and a recorded catalogue finding are the same event, and only
  one of them counts `[synthesis]`. Uncorrected, the census read the two columns
  as sharing **nothing at all** — an artifact of each column tagging its own
  findings in good faith before replay. **A pre-replay census is not a
  measurement; it is a pair of self-reports** `[synthesis]`.
- **An independence barrier over two models is not a barrier while their common
  ancestor is writable.** Four sessions of one column edited the shared claim
  catalogue before the other column existed, so the second column read corrected
  text and could not rediscover what had been removed from its search space — its
  competitor's uniqueness tags there are true and uninformative `[synthesis]`.
  **Freeze the document both models descend from, not just the models.**

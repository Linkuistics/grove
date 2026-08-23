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

> **Provenance.** Every rule here is traceable to a numbered entry in a 25-entry
> log kept while extracting a tree-on-disk library under two formalisms — Alloy
> on its structure, Quint on its operations — with the model written *before*
> each operation was implemented. Keys like `[003]` name that entry. The log is
> `docs/formalism-findings.md` in `Linkuistics/grove`; its six-field entry format
> is in [`references/keeping-a-log.md`](references/keeping-a-log.md), and the
> full evidence table is in
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

**Record what the model does not establish, in the model file.** This is the
worklist the implementation reads first `[006]`, and two of its entries were still
paying out five leaves later `[022]`.

### Four traps

| trap | what it looks like | evidence |
|---|---|---|
| **Satisfied by construction** | An invariant the model's own construction makes unreachable is indistinguishable, from outside, from one that verified it. Subtree preservation was *arranged*, not checked | `[003, 012]` |
| **A scope constant in the specification** | A bound meant to limit *exploration*, used inside a pure function of the design, does not limit the search — it changes the spec | `[003]` |
| **Checking your own plumbing** | An executable state machine must *maintain* facts a structural model can assert; checking those tests your transition relation, not the design | `[003]` |
| **Prose inside a checked file is not checked** | A witness's doc comment said "a gap in the middle"; its predicate said only `at < max`. The refusal message transcribed the comment and was wrong for every leading hole | `[013]` |

Two sampling notes: unreachability under `quint run` is **sampled, not proved**
`[003]`; and a failure mode enabled at every step starves every interesting trace,
so put it behind a constant and give the rare instance its own sample budget
`[003]`. Small scopes lie quietly, and cost is a cliff rather than a slope
`[002]`.

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

- **Mutation controls** — break the implementation and watch the tests go red.
  Minutes, no tooling, any suite, and the only thing that distinguishes a suite
  that holds from one written to pass `[006]`. Mutate the **assumption**, not the
  happy path `[007]`, and use **one control per mechanism**, not per property
  `[009]`.
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
- **A coverage report needs one mutation per reported obligation.** A conformance
  kit can distinguish *no samples* from *samples*; it cannot distinguish *samples*
  from *samples that pose the question*, and the distinction is invisible from
  inside `[020]`.

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

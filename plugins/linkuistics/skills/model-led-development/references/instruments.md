# The instruments that are not formalisms

Most of what a formal-modelling workstream actually finds is found by these.
They cost minutes, need no tooling, and each answers a question a checked model
structurally cannot. Keys like `[003]` name entries in the log this skill was
distilled from (`docs/formalism-findings.md` in `Linkuistics/grove`).

## Contents

- [Mutation controls](#mutation-controls)
- [An adversarial reader briefed to attack one judgement](#an-adversarial-reader-briefed-to-attack-one-judgement)
- [The reachability table, and its transcription protocol](#the-reachability-table-and-its-transcription-protocol)
- [Enumerate, then classify](#enumerate-then-classify)
- [An error message read as a specification](#an-error-message-read-as-a-specification)
- [Equivalence tests, while both implementations are live](#equivalence-tests-while-both-implementations-are-live)
- [The instrument-reports-less-than-it-has failure](#the-instrument-reports-less-than-it-has-failure)

## Mutation controls

**Break the implementation on purpose and watch the tests go red.** Every test in
a leaf passing the first time it runs is the state in which a suite is least
trustworthy: a test never seen failing is an assertion about the author's
confidence `[006]`.

Six deliberate breakages in one leaf, each rebuilt and run, each firing exactly
the tests it should — and two of them were the only proof that claims made in
*prose* were true of the code at all `[006]`. Later leaves ran four `[012]`, six
`[014]`, nine `[009]`, five `[016]`, scripted as a loop over one file, which is
what made six affordable where four had been the budget.

Three sharpenings, in the order they were learned:

1. **Mutate the assumption, not the happy-path structure the test author
   chose.** A stubbed lock proves the lock is *present*, not that it names the
   right inode; one inverted sort key proves ordering matters, not that all
   later keys are in the stated precedence. Ask which *independently wrong*
   implementation each control still lets through `[007]`.
2. **One control per mechanism, not per property.** *Claims its destination* is
   one sentence covering an exclusive create and a look-then-rename; the control
   for the first says nothing about the second `[009]` — and a reader counting
   from outside the module found a third `[010]`. Count mechanisms **across**
   boundaries, not within the module where the property was first stated.
3. **Mutate the repair, not only the original.** A fix and the test that proves
   it are written by one session, which is exactly the condition that produces
   controls varying only the dimension their author was already thinking about
   `[008]`. A correct helper that nothing calls passes any test written against
   the helper `[015]`.

**A comparator with `k` keys wants `k` controls, each arriving out of order.** A
tie-break fixture that supplies entries in the order the rule prescribes is
satisfied by construction and cannot distinguish the rule from a stable sort
`[008]`.

## An adversarial reader briefed to attack one judgement

**Name the judgement to attack, do not ask for a review.** Five of six findings
came from a brief that said *the trait deviates from the document, deliberately —
that judgement is the main thing to attack* `[005]`. The two structural findings
were unreachable by either model in the repository: the models supplied the law,
and a reader supplied the comparison. A model with no reader is worth what an
unread test is.

It stays valuable even where model overlap and test density are high, because
**models and mutation controls both partition by their own abstraction
boundaries**. The cheapest adversarial move is to follow one promised predicate
across the next boundary and count the mechanisms again there `[010]`.

Run it **before dependants**, not after: the fix for two trait-shape defects
changed the public surface, and doing it after the eight dependent leaves would
have been a rework of the subtree instead of a change to one file `[005]`.

**A finding is a hypothesis.** Run its counterexample before believing it — one
review's stated mutation cost two minutes to run and was wrong, while the real
defect it pointed at was worse. An integration session that agrees with its
review by *reading* it inherits the review's mistakes, including the flattering
ones `[008]`.

For the full discipline, see the sibling skill
`linkuistics:doubt-driven-development`.

## The reachability table, and its transcription protocol

**Write, at design time, which of a library's refusals your own verb set can
actually reach** — before any argument parsing exists. Then **transcribe the
table into a suite**: every row marked reachable gets a test naming its model
witness, every row marked unreachable gets none `[019]`. That turns the
implementation's test suite into a transcription rather than an invention.

**And distrust it in the same breath.** One table, four consuming leaves, four
corrected rows — by four *different* mechanisms:

| leaf | the row was wrong because | `[entry]` |
|---|---|---|
| 1 | the occupying name and the live one must share a key — a fact about how two names compose, visible only by writing the filenames out in full | `[022]` |
| 2 | the row's own next clause said *we keep our own check in front of it* — legible by reading, and not read | `[023]` |
| 3 | the plan is folded through the snapshot in effect order, so the only possible occupant has already vacated — visible only in the interpreter | `[023]` |
| 4 | a consumer-side check added two leaves earlier for an unrelated reason refuses before anything is planned | `[024]` |

The transcription is what repeats; **what it takes to falsify a row is not**, and
a leaf that budgets only for a fixture will re-affirm a row it should have broken
`[023]`. Two standing rules follow:

- **A consumer-side reachability row is a claim about the *composition* of the
  consumer's preconditions with the algebra**, so it goes stale whenever either
  side changes and cannot be settled once. Mark such a table *checked by X
  against revision Y*, and expect a leaf that adds a precondition to retire rows
  it was not looking at `[024]`.
- **A table over a by-key surface owes a row for *two entries share the key*.**
  Resolution answers with whichever entry the walk reaches first — and walk order
  was one of the model's own recorded misses, recorded twice by two layers before
  the first consumer that could feel it arrived `[022]`.

## Enumerate, then classify

For any **this is gone** claim, in preference to a grep, every time.

A clean build discharges the *linkage* claim and says nothing about prose:
enumerating every module-shaped token across the trees and classifying each
against a live set read **off disk** found thirty-three stale references in
twelve files, and not one of them was a `use` line `[025]`. **A pattern list is
complete only as far as the list; an enumeration is complete by construction.**

Two limits, both real:

- **A path scope can never reach the files that are in no tree at all** — a root
  manifest, a dotfile. Read the summary and overview layers as a separate pass.
- **A token sweep finds names, and a stale claim need not contain one.** Two
  further stale claims sat in files the enumerator *did* cover, under wordings it
  did not match `[025]`.

**Pair it with a positive control and a cross-tree control.** A broken instrument
reads clean everywhere, so clean-here alone proves nothing; clean-here plus
dirty-there cannot be produced by a broken instrument.

**And when a predicate is tightened, grep the *test* side for filters built
against the loose one.** Re-aiming one oracle at a canonical grammar left the same
file's candidate scanner and shape explainer still matching the lenient rule — so
the sweep reported a clean corpus about a class it never handed to the parser. A
duplicated leniency is free while the parser is lenient, and silent in *both*
directions the moment it is not `[025]`.

## An error message read as a specification

**A library's error message is a specification of a persistent state, and the
consumer is the only party that can act on it after the failing run has gone.**

Read every error variant whose message describes a persistent state of the
artifact, and ask **which of your own commands can meet that state later**. Of
nine variants in one library, three described a persistent state; one of them had
no owner at all, because the process that produced the message reported and
exited, and every later command met the damaged tree in silence `[024]`. The
control is the variant that says in as many words that the artifact is as it was
found — a retry is safe and nothing needs to recognise anything.

The same reading applies to any store whose failures persist. Where a design
forbids removal, this is structural rather than incidental: a damaged artifact is
never tidied away.

**And check the words, not a summary of them.** One offending message was carried
verbatim through a design document and three briefs, each with a one-line summary
of what was wrong with it — and the summary named the wrong half. Four of its six
clauses were true in the consumer's vocabulary; the defect was the *operation the
message names*. A summary of a message cannot be checked clause by clause and the
message can, so put each clause beside the glossary entry it lands on and mark it
true or false `[021]`.

## Equivalence tests, while both implementations are live

While a verb group is mid-migration there are **two live implementations of one
contract**. An equivalence test between them is one of the three consumer-side
instruments a refactor onto an already-checked library needs in place of a model,
alongside transcription suites and enumerate-then-classify sweeps `[025]`.

**The scheduling constraint is the whole trick: such tests die with the old
side**, so the evidence has to be spent *before* the contract stage rather than
after — which is what turns *this is a pure refactor* into a measured claim
rather than an argued one. (Recorded in the log's distillation note; the
increment's own close is where it was first stated.)

## The instrument-reports-less-than-it-has failure

The single most recurrent failure in this corpus, in five dresses. Each time, the
instrument **succeeded** and reported nothing:

| dress | what happened | `[entry]` |
|---|---|---|
| the tool | Alloy and Quint both report *found nothing* with exit code 0 | `[002, 003]` |
| the runner | a JVM too old to start Alloy printed nothing, recorded as seven holding checks and thirteen unfired witnesses | `[003]` |
| the test harness | `cargo test` stops after the first failing binary, so a mutation round silently omitted every integration test — one flag, `--no-fail-fast`, in a tool nobody would think to distrust | `[012]` |
| the harness wrapper | a caught mutation's failure line contains the word `error`, so a naive did-it-compile check reads every success as a build failure | `[014]` |
| the platform | APFS refuses to create a non-UTF-8 filename, so a halting branch is unreachable on the host; **a skipped test reports what a passing one reports** | `[006]` |

**The transferable defence is one shape:** a suite containing only *must-hold*
claims cannot detect that it did not run; one that also contains *must-be-reached*
claims detects it immediately, because every witness failing at once is a
signature no real defect produces `[003]`. Assert which fact is true on the host
rather than skipping `[006]`, and make every runner find its own toolchain and
abort on a tool-did-not-run signature rather than recording it as a result.

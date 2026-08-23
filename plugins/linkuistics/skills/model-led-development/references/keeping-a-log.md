# Accumulate, then distil

The routing rules in this skill could not have been written by the sessions that
learned them. Keeping the log that made them possible is a discipline in its own
right, and it is cheap. Keys like `[003]` name entries in the log this skill was
distilled from (`docs/formalism-findings.md` in `Linkuistics/grove`).

## Why the log and the skill cannot be one artifact

**They have opposite production schedules.**

A **design** converges: decisions get settled, stop moving, and are written once
and edited in place. A **method** accumulates: you cannot know which formalism
suits which situation until you have used several and been burned by at least
one, so its artifact can only be written at the end, from evidence gathered
throughout.

Collapsing the two fails in both directions. Write the skill only at the end and
the evidence is gone — a session reconstructing an earlier session's modelling
lesson from memory is exactly the failure the log prevents. Write it continuously
and you generalise from a single data point, encoding the accidents of one design
as principles.

So: **every session that reaches for a formalism appends an entry before it
finishes, and one later session turns the whole log into the skill.**

## Six fields, and the last two are the ones that matter

An entry saying "Quint was helpful here" is worth nothing.

1. **Situation** — what was being decided or specified, in one or two sentences.
2. **Formalism** — which one, and *why that one was reached for*. `None` is an
   answer, and the sessions that record *why nothing was reached for* are what
   stop the finished skill implying that a formalism is always at worst neutral
   `[018]`.
3. **Caught / missed** — concretely what it found, and what it demonstrably
   failed to find. The misses are as informative as the hits and are the first
   thing a later reader wants.
4. **Cost** — wall-clock, tooling friction, what had to be learned first.
5. **Counterfactual** — *what would have caught this earlier or more cheaply.*
   **This is the routing evidence.** "Alloy would have found this in one line" is
   what later becomes "reach for Alloy when…".
6. **Verdict** — one line: reach for this again, for this kind of question?

**The counterfactual is the spine.** An entry that says what went wrong is a war
story; an entry that says what would have caught it earlier or more cheaply is a
rule. Where the distilled skill states a rule, some counterfactual should
actually have asked for it.

**Record misses and dead ends with the same care as successes.** A log that only
contains wins produces a skill that only knows how to succeed.

## State the hypotheses up front, so they can be judged rather than quietly confirmed

The three this corpus carried, and what happened to each:

| hypothesis | outcome |
|---|---|
| **the split is real** — structural and behavioural questions want different tools, and a practitioner can tell which they hold *before* choosing | **Supported, with the line in a different place than predicted.** Not shape-versus-operation but *how many states the property mentions* `[003]`, and a later leaf routed correctly to *neither* before it started `[018, 021]` |
| **the model leads profitably** — writing the specification before the implementation beats writing it after | **Supported, and the strongest evidence is about coverage rather than quality**: the model's value tracked, almost exactly, the fraction of the work inside its scope `[009]` |
| **a checked model beats prose of equivalent length at the implementation site** | **Not supported**, by a deliberate pre-registered probe `[017]`. What replaced it is narrower and better supported: a model pays *upstream*, in the prose it corrects |

**Pre-register the probe that tests the load-bearing one** — its arms, its
measure, its prediction and its falsification condition, written before any
implementation runs `[017]`. Two conditions that probe failed to meet, and that
any repeat should:

- **The measure must be written by a party that implements neither arm, from the
  claims alone.** Fixing the *claims* in advance was not enough: a test is a
  claim plus an assertion, and every failure landed on assertion wording the
  scoring arm had authored.
- **Record whether the prose an arm reads has already been through a model.** The
  probe's prose arm read model-*corrected* prose, so it compared *model + prose*
  against *prose*, which is not the hypothesis.

## The log becomes an instrument

The payoff is not only the distilled skill. Late in the corpus a leaf routed
itself, sized its own model coverage, chose two instruments and ran a negative
control — **all four read out of the accumulated routing table rather than
invented**, and no model was involved `[021]`. That is evidence for the practice
that no single modelling episode could produce, and it is the point at which the
log starts paying for itself before it is ever distilled.

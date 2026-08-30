# The skill delivers the methodology, and `${prompt}` carries a guaranteed core

Grove's methodology reaches a session as a **provisioned skill** — the whole of
the driver's embedded `content/`, swept into every installed harness's personal
skill directory — and `${prompt}` carries a **short guaranteed core** that points
at it. Delivery is two channels with one source, and neither is a fallback for
the other: a session that reads only the core has the three things it could not
learn any other way, and everything else is on disk in one skill, named in the
prompt, with this session's own reference file named too.

> **Superseded in part by `prompt-names-the-kind-k18`, and retired whole at
> `delete-provisioning-k19`.** What this record describes below is the delivery
> design as it stood while the binary provisioned `content/` and composed a core
> out of it. Three of its claims are no longer true of the code: the core no
> longer names a [kind reference file](../../content/references/), it no longer
> inlines a corpus signal file byte-exact, and it no longer lists the provisioned
> directories — all three went with `src/prompt.rs`'s content dependency, so the
> core is now three driver-authored parts that read nothing, and the four
> structural couplings below are one (the `grove-<kind>` skill the prompt names
> must exist). What replaces the byte-frozen ending is Grove's own signalling
> contract, stated once for every kind, with the ending a kind takes inline in
> that kind's `grove-<kind>` skill. The rest of the record — the too-late test,
> the two observed failures it sits between, and the residue it accepts — is what
> `docs/specs/module-decomposition.md`'s decisions 9 and 10 were argued from, and
> it is left standing until provisioning itself goes, because rewriting it ahead
> of that code would describe a delivery path that still runs.

**Two failures are observed, not theorised, and they pull opposite ways.**
Delivering the methodology through `${prompt}` degrades behaviour — sessions
finish their work correctly and then fail to signal, stalling the loop. Delivering
it as a skill alone fails differently: sessions demonstrably did not read it. A
design answering one is a swap for the other, and both limbs are required of any
check that claims to clear this.

## The core rule: what earns a place in `${prompt}`

**The too-late test.** A sentence earns `${prompt}` only if its failure mode is
one the skill cannot repair, because by the time the skill could speak the moment
has passed. Three shapes pass, and the test is what makes them three rather than
a list someone may extend: **a fact only the driver holds**, which the skill
cannot state at any strength because it is not knowable at build time; **the
instruction to open the skill**, which the skill cannot deliver because it is the
bootstrap edge; and **the session's last action**, which falls due after
everything else the session does.

What the test refuses matters more than what it admits, because every argument
for widening the core takes one of four forms. *This is important* — refused;
every rule in a methodology is important, and importance is unbounded, so the
wall is what importance-as-a-criterion builds. *This is needed in every session* —
refused; frequency is not timing, and a rule needed in every session is needed
while the skill is open. *A session got this wrong once* — refused unless the
failure was too-late-shaped, since otherwise the fix is a stronger trigger or
better skill prose, both cheaper and neither growing the wall. *It is only one
more sentence* — refused; size is a consequence of the rule and never a criterion
for admission, because a core defended by its byte count is defended by nothing.

**The test is closed on the word "fact":**

> A driver fact is a **launch-varying value** that `content/` cannot know at build
> time. Its static meaning, and every normative consequence of it, stay in the
> skill.

Without that clause "fact" is unbounded, because almost any rule can be restated
declaratively — *the pick is authoritative*, *the stated VCS is definitive* — and
smuggled in as though it were a value. A value has no counterpart in `content/`
to drift from; a restated rule has exactly the counterpart the drift claim denies.
The closure has one real cost, paid deliberately: *do not probe for the version
control* used to ride the prompt and now rides the skill, where it depends on the
skill being read like every other rule.

Applied candidate by candidate the test admits **five** things, and its output is
one methodology condition rather than the open plural the requirements handed it:
the instruction to load the skill and read this kind's reference file, where the
skill was provisioned, the selected leaf's handle, the working tree's version
control, and the completion verb as the session's last action.

## Why two channels do not mean two sources

The record this replaces objected to two delivery paths on the ground that they
**can disagree**. Disagreement needs two sources, and this design has one. The
load instruction and the provisioned locations are driver prose with no
counterpart in `content/` to drift from — a skill cannot tell you to read it, and
cannot know which directories a particular driver wrote. The runtime facts are
values, not expressible in `content/` at all. The session-ending instruction is
the one genuine duplicate and is not duplicated: the driver inlines the embedded
corpus's own signal file **verbatim**, from the same embed that is provisioned, so
there is one source, two deliveries, and no build boundary between them.

So the **prose** drift surface is zero bytes, and the property is structural
rather than a claim about size: `${prompt}` is a fixed template in which exactly
one part is embedded content. What the two channels do share is **four structural
couplings**, enumerated rather than covered by a summary claim. Two are closed by
construction — the signal file is embedded at compile time, and the locations are
computed by the same registry that writes them. The other two — the skill's
**name** and each kind's **reference path** — are closed by assertion, and both
fail by name. A coupling held by a check is not drift; a coupling held by nothing
is.

This is what remains of *slice, never paraphrase*, and it is the right residue.
The composer, the marker grammar and the completeness invariant existed to make
*selective* delivery safe; delivery stops being selective, so they have no job.
But the reason a slice was byte-exact rather than summarised never depended on
selection, and it applies with full force to the one file that still travels both
ways.

**One kind's ending is a choice, so it gets a second file rather than no file.**
A `finish` session has three endings, chosen by what it did, and a fixed
`grove-llm complete` last in the prompt of a session that may have just torn the
tree down would relaunch the loop onto a deleted grove. The too-late test admits
*the session's last action*; it does not license stating the wrong one. But
dropping the part reads that as licence to say nothing, and `finish` is the
session where a forgotten signal costs most — teardown lands, `--done` is
forgotten, and the loop waits on a session that will not end. Reading the branches
in a reference file at bootstrap cannot repair a last action forgotten an hour
later, which is exactly the shape the test admits a sentence for. So the choice
itself is the ending: `content/SIGNAL-FINISH.md` states all three outcomes,
`content/references/finish.md` routes to it rather than restating it, and the core
inlines those bytes. Two signal files, one source each, and every kind's prompt is
three parts with exactly one of them embedded content.

## What is retained, and what is overturned

**Retained** from the record this replaces: that a session told a fact succinctly
never runs the derivation that would have established it — which is exactly why
the driver's four values ride the prompt; that driver-authored prose *about the
methodology* would make `content/` non-canonical and create a second source that
drifts across the build boundary; and that the `if`/`then` asymmetry is real — a
withheld procedure costs a lookup the session knows to make, while a withheld
condition yields an unasked question.

**Overturned** is that record's own clause, in its rejection of pointing at
locations: *"only as a supplement for units too large to inline, never as a
replacement for triggering conditions."* Four strands, the first three of them the
record's own material.

**1. The clause's stated premise has been falsified, by the record's own reopen
condition.** The reason a pointer could not replace a condition was granularity —
*a kind's discipline is one bullet inside a section of nineteen, so a pointer
leaves the session reading the whole section and performing the selection itself.*
The record named the reopen: *reopen if `content/` is ever restructured so that
every rule is separately addressable.* The unit markers made that true, and this
design goes further by restructuring the corpus so a kind's discipline is a
**whole file the driver names by path**. The session performs no selection at all.

**2. The risk model counted one failure and the evidence names two.**
*Withholding a condition yields an unasked question* remains true. What the model
never priced is that a condition **delivered inside a wall and not acted on** is,
behaviourally, a condition not delivered — and the superseded design was explicit
that its invariant never claimed otherwise, warning against reading the invariant
as a promise about detection. Overturning the clause is therefore not accepting a
risk the old design avoided; it is choosing between two ways of failing to
deliver, with measurement on both, where the measured cost of the wall is a
stalled loop and the measured cost of the skill was a session that ran without it.

**3. What is withheld is nothing, and the failure changes kind.** Under the
mandate a withheld thing was a procedure the session knew to ask for. Here nothing
is withheld. The failure is no longer *the session was never told there was a
question* — silent, and unattributable — but *the session did not open the
document it was told to open*, which leaves a trace a human watching can see.

**4. The residue is real and is named rather than argued away.** A session that
ignores the pointer gets nothing, where a session that ignored a slice at least
had the bytes in front of it. That is a worse failure per occurrence, and the whole
of what pays for it is **trigger strength** — which is why the wording was
micro-tested against a control before anything shipped, five fresh-context
repetitions per arm on both configured targets, and why the acceptance check is
the one the superseded record itself nominated: the next real Grove run, with a
human watching, both limbs required.

That micro-test settled the wording and narrowed it. Three elements ship — one
imperative naming both targets, an ordering clause enumerating the tempting
alternatives, and the provisioned directories by absolute path — and the control's
failure turns out to be **ordering rather than ignorance**: it opened `SKILL.md`
in 9 of 10 sessions and reached its kind's *procedure* only after starting work,
in every session of both arms.
[`wording-micro-test`](../research/wording-micro-test.md) carries the arms, the
counts and the limitations.

Provisioning returns unchanged, and with it the shared mutable directory and the
build-pairing story [`one-build-owns-a-session`](one-build-owns-a-session.md)
prices. The corpus's condition/procedure layout, the three size alarms over it,
and the seam this is tested through are in
[`ARCHITECTURE.md`](../ARCHITECTURE.md#corpus-shape); what this record's
requirements asked for is now discharged as tests.

## Considered options

The first four are this decision's own; the rest are the superseded record's,
carried forward where their reasoning survives its mechanism.

- **Keep the mandate: deliver the methodology as composed `${prompt}` slices.**
  Rejected on the measurement above — the wall degrades behaviour, and a stalled
  loop is the failure that costs most because it is the one nobody is present for.
  Reopen if the human-watched run shows sessions ending correctly *and* not
  reading the skill, which would mean trigger strength did not pay for the residue
  after all.
- **Widen the core beyond the too-late test.** Rejected, and this is the entry the
  section above exists to be argued against, because the predictable failure mode
  of this design is **erosion by addition** rather than a single wrong decision.
  Each candidate is individually small and individually defensible on importance,
  frequency, or an anecdote; the four refusals are what make the test a rule
  instead of a preference. Reopen **only** by adding a *shape* to the three the
  test admits, with the argument made against the four refusals by name — never by
  admitting a sentence and leaving the rule as it was. A size alarm in the suite,
  set well above anything legitimate, exists to make the erosion visible rather
  than to bound it.
- **Give `finish` the same fixed ending as the other eighteen.** Rejected because
  two of its three outcomes make a fixed ending wrong, and the one it would state
  is the destructive one: bare `grove-llm complete` after a completed teardown
  relaunches the loop onto a torn-down grove, which the driver then re-scaffolds.
  Reopen if the finish cycle ever collapses to a single ending.
- **Give `finish` no ending part at all, and let `references/finish.md` carry its
  outcomes.** Rejected because it answers the objection above by deleting the very
  too-late-shaped instruction the core exists to carry, in the session where a
  forgotten signal costs most: a completed teardown followed by a forgotten
  `--done` leaves the loop waiting on a session that will not end, which is the
  measured failure this record sits between. A reference file read at bootstrap
  cannot repair a last action forgotten an hour later. Reopen only if the ending
  stops being a last action — never on the grounds that `finish`'s outcomes are a
  choice, which is what `content/SIGNAL-FINISH.md` exists to state.
- **Split this into two records — the delivery reversal and the core rule.**
  Rejected because they are inseparable: answering either alone is a swap, so two
  records for one decision is one ADR pretending to be two. Reopen if a future
  design keeps the core rule while changing the delivery path, which would make
  them separable in fact rather than only on paper.
- **Inline the whole methodology into every `${prompt}`.** Rejected, and the
  reasoning survives the reversal in a changed form: the objection is no longer
  that it abandons specificity but that it *is* the wall, ~49 KiB on the one
  channel a session cannot skip. Reopen never.
- **Have the driver compose or summarize the methodology in its own prose.**
  Rejected because it makes `content/` non-canonical: a summary can contradict its
  source, and nothing would detect it. This is why the one duplicated file is
  inlined byte-exact rather than restated, and why the core carries no rule that
  has a counterpart in `content/`. Reopen never.
- **Ship the methodology through the marketplace beside `linkuistics`.** Rejected
  in requirements: it makes installing Grove two steps and lets skill and binary
  versions drift with no check left. Single-command `brew install` keeps working
  and the content-hash stamp keeps meaning something. Reopen if the marketplace
  ever offers a version-pinning mechanism a binary can assert against.
- **Harness-specific loading — a hook, an MCP server, an injected system
  prompt.** Rejected because Grove executes an opaque configured command directly
  and adds no hidden harness-specific argv; a per-harness delivery path would be
  launch policy Grove does not own. Reopen only if a harness offers a *standard*
  skill preload the Agent Skills spec defines.
- **A machine-checked read receipt.** Rejected because Grove cannot observe what a
  session reads, and the one available proxy has a false negative the methodology
  itself invites. Reopen if a launch target exposes which skills a session loaded.
- **Verify the change behaviourally as a gate.** Rejected, unchanged from the
  record this replaces: a standing eval is expensive, non-deterministic, measures a
  model rather than Grove's artifact, and localizes nothing when red. Two things
  are deliberately *not* gates but are still required — the wording micro-test,
  run once before shipping, and the human-watched acceptance run with both limbs.
  A gate is what a contributor's build has to satisfy; these are experiments a
  human reads once.

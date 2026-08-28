---
name: doubt-driven-development
description: In-flight adversarial verify for non-trivial decisions — materialise a fresh-context reviewer biased to *disprove* a decision before it stands, instead of a post-hoc gate on finished work. Use when about to commit non-trivial code, make an architectural call under uncertainty, assert a property the compiler can't check (thread-safety, idempotence, ordering, an invariant), or work in unfamiliar code — any time a confident output would be cheaper to verify now than to debug later.
harnesses: [any]
assumes-personal-setup: true
---

# Doubt-Driven Development

A confident answer is not a correct one. Long sessions accumulate context that
quietly turns assumptions into "facts" without anyone noticing, and the moment
you feel most certain is exactly when a blind spot is hiding. Doubt-driven
development is the discipline of **materialising a fresh-context reviewer —
biased to disprove, not approve — before any non-trivial decision stands**.

This is **not** `/code-review`, and not superpowers' `requesting-code-review` /
`receiving-code-review`. Those are *post-hoc*: a verdict on finished work, at PR
time. This is *in-flight*: a non-trivial decision gets cross-examined **while
course-correction is still cheap** — before you build on top of it. By the time
a post-hoc gate runs, the wrong direction is already load-bearing.

> The discipline below is adapted from `addyosmani/agent-skills`'
> `doubt-driven-development` (MIT; source and snapshot in
> [`../../PROVENANCE.md`](../../PROVENANCE.md)). It is **convergent** — `gstack-G2`,
> `superpowers-G4`, and `wshobson-G2` independently reached the
> don't-bias-the-reviewer rule. The core that transfers is the *discipline*, not
> the upstream's ecosystem scaffolding.

> **Measurements.** Every figure cited below comes from one workstream's own
> review record, `docs/review-yield.md` in `Linkuistics/grove` — nine tree-level
> review chains and three in-session reviewers. **The two channels were counted
> by different instruments**, and the document says so: the tree-level counts are
> derived from markdown block structure under a rule fixed before any count was
> taken, while the in-session findings are written as bullets that enumeration
> deliberately skips, so **that channel is a hand count** from the sessions' own
> four-way classification. **One grove, one subject matter, one operator**, in a
> repository whose whole purpose was rigour about evidence. The rates are rates
> *there*; what transfers is the structure they expose, not the numbers.

Harness-specific spawn commands (codex ↔ pi ↔ claude, cross-model):
see `references/harness-spawns.md`.

## When to use

A decision is **non-trivial** when at least one is true:

- It introduces or modifies branching logic.
- It crosses a module or service boundary.
- It asserts a property the type system or compiler cannot verify — thread
  safety, idempotence, ordering, an invariant.
- Its correctness depends on context a future reader cannot see.
- Its blast radius is irreversible — production deploy, data migration, public
  API change.

Apply it when about to make an architectural call under uncertainty, commit
non-trivial code, assert a non-obvious fact ("this is safe", "this scales",
"this matches the spec"), or work in code you don't fully understand.

**When NOT to use** — mechanical edits (rename, format, file move); following a
clear unambiguous instruction; reading or summarising code; one-line changes
with obvious correctness; pure tooling (running tests, listing files); or when
the user has explicitly chosen speed over verification. **If you doubt every
keystroke, you ship nothing** — this is for non-trivial decisions only.

### Spend a reviewer when it is a *different instrument*

A reader who reads what you read, the way you read it, returns a **second
opinion**. One that reads something you did not — another corpus, another
representation, your artifact swept against a library of claims you never
checked it against — returns a **different instrument**. That is the question to
put to a candidate reviewer before spawning it.

Three sessions in the measured workstream spent their allowance, and **all three
changed the session that spent them**: one broke a disposition its session had
already landed, forcing a reversal across nine artifacts; one swept all 63 of a
library's invariants against the candidate's module and found two violated, one
nobody was looking for; one made its session withdraw a sentence it had already
written. Three for three, and not one of them a second opinion.

**Declining is a real answer, and the reason is what makes it one.** Five
sessions in the same workstream considered the allowance and did not spend it,
for four distinct reasons — two a judgement about the better instrument, one
about where the same one belonged, and one a constraint rather than a
judgement:

- **Executable evidence beats a fresh read of the same prose.** One session
  spent four tests that would break if its central claim were wrong instead.
- **A later step is already chartered to contest the verdict**, so a reviewer
  here buys the same read twice.
- **The doubt is one a fresh read cannot discharge** — the control you would
  hand over is itself part of what needs challenging.
- **The harness forbids the spawn** — a constraint, and the only one of the four
  that is not a judgement. Then the answer is to schedule the fresh-context read
  where it *can* happen, not to skip it: two of the five sessions cut a scheduled
  review step instead, and one called that the stronger instrument.

A decline recorded as *not needed* is the skip this skill exists to catch.

## This is a main-session orchestrator skill

Step 3 (DOUBT) **spawns a fresh-context subagent**, so this skill runs from the
**main session**, which owns the ability to dispatch one.

- **Don't list it in a subagent's own skills.** A subagent that reached Step 3
  would try to spawn another subagent — nested dispatch the harness forbids.
- **If you find yourself applying it from inside a subagent**, prefer to surface
  to the user that doubt-driven can't run nested and let the main session handle
  it. Last resort only: a *degraded* self-questioning fallback — rewrite ARTIFACT
  + CONTRACT as a fresh self-prompt with a hard separator from your prior
  reasoning, then walk the steps. This is **not** fresh-context review (you carry
  your own context), so flag the result as degraded.

## Composition with Grove

Grove owns review escalation only when the Grove driver launched this exact
session with a selected-leaf mandate in its prompt and the session adopted that
mandate by running Grove's **Bootstrap**. A `.grove/` directory or an inherited
Grove control variable is not enough. Outside that predicate, use the standalone
process below unchanged.

Inside it, the rules here replace per-artifact verification, re-looping,
diverse-lens fan-out, and optional cross-model follow-ups as needed to enforce
one budget across the **whole picked leaf**:

- A plain `requirements`, `design`, `planning`, `prototype`, or `impl` producer
  may materialise at most one fresh-context reviewer. Every independently
  spawned reviewer counts. A cross-model reviewer may be that one only after the
  normal explicit authorisation; it is never an additional cycle.
- A second review need runs
  `grove-llm leaf-add <parent> <stem> --kind review-<producer>`, with the
  specific doubt written into the new leaf's body — that body is the reason to
  cut the leaf now rather than have had a constructor cut it up front.
  This normally follows a substantive actionable finding whose non-mechanical
  fix needs re-review. Trivial findings, noise, visible accepted trade-offs, and
  fixes conclusively covered by an executable test seam do not force escalation.
- A producer that already has a `review-*` leaf beside it invokes none: that
  scheduled leaf supplies the fresh context. A `review-*` leaf invokes none
  because it is already the adversarial read and must produce findings, not
  fixes.
- An `integrate-review-*` leaf may spend one narrow reviewer. Substantial
  redesign becomes a new producer review chain beside the leaf being integrated.
- `research-a`, `research-b` and `combine-research` invoke none. The pair supplies independent
  corpora and the combiner supplies the adversarial move; put a load-bearing
  derived decision in its own reviewed producer chain.

**Grove owns the tree-level form of RECONCILE's contract rule, and it binds
without this skill.** Grove's own `references/decompose.md` states that an
integration's body carries the review's **handle, never its findings**, and its
`references/integrate-review.md` sends the integrating session to read those
findings from the review's own commit. Those are Grove's rules rather than a
précis of this one: with this plugin absent the rule still binds, because it
changes what a session *writes* into a leaf body. Read them there and follow
them; RECONCILE adds only the same rule for a doubt cycle's own handoffs, which
Grove does not cover — and outside a Grove mandate there is no leaf body to
write, so nothing is lost in the other direction either.

Writing the specific doubt into a new review leaf's body, above, is the
compatible case and not an exception: that is the producer's own doubt
**aiming** a reader, not a reviewer's finding list becoming somebody's charter.
Aiming is what the escalated leaf is *for* — so aim it deliberately, and then
count what comes back as **one aimed reader**, not as an independent second
sample ([*Independent means unaimed*](#independent-means-unaimed)).

After cutting that leaf, finish only to a coherent reviewable boundary, **retire
the producer, then** commit the artifact, the new leaf and that retirement
together under the producer's own handle, and hand back with `grove-llm
complete`. The order matters: the `DONE` rename is part of this task, so a
commit taken before it either leaves the rename uncommitted or seals it into the
next task's change. Do not spawn another doubt reviewer. Nothing about the
producer's leaf moves, so its handle and bytes are preserved by construction;
retirement is the filename `DONE` transition alone, recording nothing about the
session that just ran and leaving the review task untouched, because the review
reads the committed artifact.

Grove owns the escalated route from there. It launches the scheduled `review-*`
from that kind's own configuration entry, and whether that reaches a different
harness or model from the producer's is the configuration owner's policy — Grove
compares no targets and raises no notice, so there is none to wait for or
consume. Do not reconstruct that history, and do not start another review cycle
to compensate for its absence.

## The process — CLAIM → EXTRACT → DOUBT → RECONCILE → STOP

Copy this checklist when applying the skill:

```
Doubt cycle:
- [ ] CLAIM     — wrote the claim + why it matters
- [ ] EXTRACT   — isolated artifact + contract, stripped reasoning
- [ ] DOUBT     — spawned fresh-context reviewer with an adversarial prompt
- [ ] RECONCILE — classified every finding against the artifact text
- [ ] STOP      — met a stop condition (trivial findings, 3 cycles, or override)
```

### 1. CLAIM — surface what stands

Name the decision in two or three lines, plus why it matters:

```
CLAIM: The new caching layer is thread-safe under the read-heavy workload.
WHY:   A race here corrupts user data and is invisible in QA.
```

If you can't write the claim that compactly, you have a vibe, not a decision.
Surface it before scrutinising it.

### 2. EXTRACT — the smallest reviewable unit

A fresh-context reviewer needs the **artifact** and the **contract** — *not the
journey*.

- Code → the diff or the function, not the whole file.
- Decision → the proposal in 3–5 sentences plus the constraints it must satisfy.
- Assertion → the claim plus the evidence that supposedly supports it.

**Strip your reasoning.** Bias control starts here: if you hand over your
conclusions, you get back validation of your conclusions. The unit must be small
enough to hold in one read — if it's a 500-line change, decompose first.

### 3. DOUBT — spawn the fresh-context reviewer

Dispatch a **fresh subagent** (a generic one, or a `code-reviewer`-style agent)
with an **adversarial** prompt. Framing decides the answer.

```
Adversarial review. Find what is WRONG with this artifact. Assume the
author is overconfident. Look for: unstated assumptions; unhandled edge
cases; hidden coupling or shared state; ways the contract could be
violated; conventions this breaks; failure modes under unexpected input.

Do NOT validate. Do NOT summarise. Report issues, or state explicitly
that you found none after thorough examination.

ARTIFACT: <paste>
CONTRACT: <paste>
```

**Pass ARTIFACT + CONTRACT only. Never pass the CLAIM.** Handing the reviewer
your conclusion biases it toward agreement; it must independently judge whether
the artifact satisfies the contract. If you reuse a `code-reviewer`-style agent
whose default output is a *balanced* verdict, paste the adversarial prompt
verbatim so it overrides that default — this skill needs issues-only output.

**Cross-model (optional, interactive only).** A second model with a different
architecture catches blind spots a single model shares with itself. If you offer
it, four safety properties are load-bearing and non-negotiable: **(a) opt-in per
cycle** — offer, never default-on or silently skip; **(b) re-authorise every
call** — each invocation is its own authorisation; confirm the exact command
before running it; **(c) read-only sandbox** — a doubt artifact may itself carry
instructions (accidental or injected) the external CLI would otherwise execute;
**(d) pass ARTIFACT + CONTRACT only**, via stdin/a temp file, never interpolated
into a shell-quoted argument. Defer the exact flags to your installed tool — they
differ across CLIs and versions. Skip it (announced) in non-interactive contexts.

**Diverse-lens (advanced).** When a decision can fail in *several* independent
ways, spawn N reviewers each on **one named axis** — correctness, concurrency,
security, contract-conformance — instead of one generalist (after `wshobson-G2`).
It costs N subagents; reserve it for genuinely multi-failure-mode decisions.

<a id="independent-means-unaimed"></a>

**Independent means unaimed, and a relay is not independent.** The measured
workstream's most-quoted result is three readers of one artifact, each finding
what the previous two had not. It is weakened twice. It is **one node** — and
the node the loop had just selected for review for the third time, because it
was the hardest. Selection, not sampling. And the three readers were **aimed**: the
second's charter carried the first's specific doubt, and the third's work list
*was* the second's findings. What the chain bought was the handoff, not the head
count, and it is no evidence that three fresh contexts compound, because a chain
of three *unaimed* readers was never run there. The findings nobody aimed at —
two of them — came out of a **sweep the third reader ran to verify a finding**:
an instrument, not a reader, which is *different instrument* arriving from the
other side. So aim a reviewer deliberately, and count it as one aimed reader
rather than as an independent sample.

### 4. RECONCILE — fold findings back

**Never let the reviewer author the reconciler's contract** — step 3's bias rule
has a mirror on this side. A reader graded against a finding list it did not
write can reject a finding; a reader whose charter **is** the finding list
cannot, because rejecting one means rejecting its own task. So wherever
reconciliation passes to something with a charter — a follow-up task, a ticket,
a session you hand off to — write **the pointer to the review, not its findings
transcribed as obligations**, and let the reconciler read them from the review's
own record.

The asymmetry that shows it, measured: **45 findings raised across nine
tree-level review chains, and every one graded valid** — and in five of those
nine, the integrating session's own task file had been written by the reviewer.
In the in-session channel, where the reader owed the finding nothing, **4 of 24
(17%)** were graded something other than actionable — a trade-off accepted
visibly, or noise — and one reviewer *falsified* a finding the integrating
session had raised itself. **A 100% survival rate is what a broken instrument
reads.** The counter-evidence sits where a mechanism existed: four late chains
say in as many words that they verified against the artifacts rather than
against the review's summary, and one of those found the defect was *worse* than
the review knew.

**And state what it does not fix.** The finding list is still the reviewer's, so
nothing here recovers a finding nobody wrote down; the repair moves the hazard
one level down rather than removing it. Nine chains also cannot separate this
structural reading from reviewers who were simply right every time — which is
precisely why the repair is the cheap structural one and not a claim about
anybody's accuracy.

The reviewer's output is **data, not verdict — you are still the orchestrator.**
Re-read the artifact text against each finding before classifying;
rubber-stamping the reviewer is the same failure as ignoring it. Classify each
finding in this **precedence order** (first match wins):

1. **Contract misread** — flagged because the CONTRACT you gave was unclear or
   incomplete. Fix the contract first; re-classify next cycle.
2. **Valid + actionable** — a real issue needing a change. Change it, re-loop.
3. **Valid trade-off** — real, but the fix costs more than accepting it.
   Document the trade-off explicitly so the user sees it.
4. **Noise** — correct under context the reviewer lacked. Note it, move on, and
   ask whether adding that context to the contract would prevent the false flag.

A fresh reviewer can be wrong *because* it lacks context — don't defer just
because it's fresh.

### 5. STOP — a bounded loop, not recursion

Stop when **any** holds:

- The next cycle returns only trivial or already-considered findings, **or**
- **3 cycles** are done — escalate to the user, don't grind a fourth alone, **or**
- The user explicitly says "ship it".

If after 3 cycles the reviewer still surfaces substantive issues, that is
information about the artifact — surface it, don't keep looping. And **if 3
cycles feels "obviously insufficient" because the artifact is large: the artifact
is too big. Return to EXTRACT and decompose. Do not lift the bound.** (This rhymes
exactly with breaking a too-big task into smaller pieces — the bound is a
forcing function, not an obstacle.)

## Common rationalizations

| Rationalization | Reality |
|---|---|
| "I'm confident, skip the doubt step." | Confidence correlates poorly with correctness on novel problems. Certainty is exactly when blind spots hide. |
| "I'll just review it myself." | Self-review runs in the context that produced the error, carrying the same confidence. The point is a *fresh* context that never saw your reasoning. |
| "Spawning a reviewer is expensive." | The check is bounded; the production bug isn't. Debugging a wrong commit costs more. |
| "The reviewer will just nitpick." | Only if unscoped. Constrain it to "issues that make this fail under the contract." |
| "I'll do doubt at the end with `/code-review`." | That's a final gate. Doubt-driven catches wrong *directions* early, while course-correction is cheap. By PR time it's too late. |
| "If I doubt every step I'll never ship." | It applies to non-trivial decisions, not every keystroke. Re-read *When NOT to use*. |
| "The reviewer disagreed, so I was wrong." | It lacks your context — disagreement is data, not verdict. Re-read the artifact, classify, then decide. |

## Red flags — stop and check yourself

- Spawning a reviewer for a one-line rename or a formatting change.
- Treating reviewer output as authoritative without re-reading the artifact.
- Looping past 3 cycles without escalating to the user.
- Prompting the reviewer with "is this good?" instead of "find issues".
- Re-spawning a reviewer on an **unchanged** artifact (same findings — you're stalling).
- Skipping doubt under time pressure on a high-stakes decision.
- Passing the CLAIM, or your reasoning, to the reviewer (biases it toward agreement).
- Stripping the contract from the reviewer's input.
- Handing a reconciler the finding list **as its charter** — a task, ticket or
  session graded against findings it did not write has nowhere to stand to
  reject one. Pass the pointer; let it read them from the review.
- **Doubt theater (checkable signal):** across 2+ cycles where the reviewer
  surfaced substantive findings, **zero** were classified actionable. You are
  validating, not doubting. Stop and escalate.
  **Read it on zero-actionable, which is scale-free — never on the count going
  down.** A falling count is not decay, and one workstream measured why it
  cannot be read as decay: findings per review ran **6.2 then 3.5** across nine
  chains, a gap **inside the noise at about 1.8σ** (roughly 25 matched chains
  would be needed to see through it), over subjects that were **not matched** —
  first-of-their-kind artifacts first, incremental decisions on already-reviewed
  ones later — while the **review rate itself rose**, reaching further down its
  own value curve. And the unit is elastic: without a severity scale fixed
  **before** the reviewer starts, counting findings measures enumeration style,
  since sub-corrections per finding ran **1.0 to 3.3** and consequence was not
  captured at all. So more cycles is the weaker fix; a severity scale you
  pre-register is the prior one.

## Interaction with other skills

- **`/code-review`, `requesting-code-review`, `receiving-code-review`** —
  complementary, *post-hoc*. Those are the final-gate verdict on finished work;
  doubt-driven is the in-flight per-decision check. Use both.
- **`test-driven-development`** — TDD's RED step *is* doubt made concrete: a
  failing test is a disproof attempt. When TDD applies, that failing test
  satisfies the doubt step for behavioural claims.
- **`systematic-debugging`** — when the reviewer surfaces a real failure mode,
  drop into systematic debugging to localise and fix it.

## Verification

- [ ] Every non-trivial decision was named as a CLAIM before it stood.
- [ ] Outside a Grove-owned picked leaf, at least one fresh-context review per
      non-trivial artifact (a TDD RED test satisfies this for behavioural
      claims); inside one, the leaf-wide allowance and scheduled review own it.
- [ ] The reviewer received ARTIFACT + CONTRACT — **not** the CLAIM, not your reasoning.
- [ ] The reviewer's prompt was adversarial ("find issues"), not validating.
- [ ] Findings were classified against the artifact text using the precedence:
      contract-misread / actionable / trade-off / noise.
- [ ] Nothing downstream was graded against a finding list it would have to
      reject: reconcilers got the pointer to the review, not its findings
      restated as their obligations.
- [ ] A reviewer that was declined was declined **with its reason** — the better
      instrument, the better place for the same one, or the constraint that
      moved it somewhere it can happen.
- [ ] A stop condition was met (trivial findings, 3 cycles, or user override).
- [ ] Any cross-model invocation was opt-in, re-authorised, read-only-sandboxed,
      and given ARTIFACT + CONTRACT only.

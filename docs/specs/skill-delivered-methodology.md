# skill-delivered-methodology

## Problem

Two ways of delivering Grove's methodology to a session have now been tried, and
**both have been measured failing** — in opposite directions. Neither failure is
theorised; each was observed on a real Grove run with a human watching, which is
the check [the mandate delivers the
methodology](../adr/skill-delivers-the-methodology.md) itself nominated.

| delivered by | observed failure |
|---|---|
| a provisioned skill (before the mandate) | sessions did not read it |
| `${prompt}` (today) | the wall degrades behaviour — sessions finish their work and fail to signal, which under an interactive harness **stalls** the loop |

The second failure is the sharper one and it is the reason a change is being
made. It is also *not* the failure the two increments before this grove were
chasing: `retire-next-steps-k2` added a commit-then-complete reminder at
retirement and `signal-unit-placement-k3` moved the signal unit to compose last.
Both treated the symptom. The diagnosis is that the wall itself is the cause, and
a ~49 KiB `${prompt}` carries two further costs beyond it — the text is no longer
on demand, arriving whole and up front whether or not the session's situation
ever calls for any of it, and the mechanism is alien enough to put a user off.

**The trap this creates is that a design answering either failure alone is a
swap.** Move everything back to a skill and the first failure returns intact.
Keep the wall and nothing improves. Any design here owes an answer to both, and
owes it in a form where the two answers do not simply reintroduce each other.

One measurement narrows the field before any design starts. The procedural half
of the methodology is **already** out of every mandate — 93,102 bytes reachable
only through `grove-llm methodology`. The wall is made entirely of *triggering*
units: 51 universal ones totalling 45,032 bytes, plus 13,869 bytes across the
narrowed 20. So relocating procedures changes mandate size by exactly zero, and
the wall is only reachable by moving triggering conditions — which is the one
thing the superseded record forbids.

A second measurement decides the shape of the answer. 51 universal units
averaging ~880 bytes each are not terse `if`s; they are **prose that contains a
condition**. So a straight re-plumb of `content/` into a skill reinstates the
~50 KiB monolithic `SKILL.md` whose size was the superseded design's own opening
complaint — moving the wall from launch time to first-invocation time rather than
dissolving it, because a harness skill is not lazily read *within* itself.

## Solution

The methodology is delivered as a **provisioned progressive-disclosure skill**
again, and `content/` is **rewritten** to be one — a short `SKILL.md` of
conditions over a flat `references/` directory carrying the procedures. That
answers the wall.

`${prompt}` shrinks to a **guaranteed core** of a few kilobytes, and what it
carries is decided by one rule rather than by a list. That is what keeps the
answer from being a swap: `${prompt}` is the one channel a session cannot skip,
so the core rides it, and its whole job is to make the skill un-skippable in
practice.

Four properties make that safe, and they are the four the design is judged on.

**The core is bounded by a rule about *timing*, not about importance.** A
sentence earns `${prompt}` only when its failure mode is one the skill cannot
repair, because by the time the skill could speak the moment has passed. Applied
to every candidate, that yields three shapes and nothing else — a fact only the
driver holds, the instruction to open the skill, and the session's last action.
Importance is the argument the rule exists to refuse.

**There are two delivery channels and one source.** The superseded record
rejected two paths because they *can disagree*, and disagreement needs two
sources. Two of the core's three shapes cannot be in the skill at all; the third
is inlined byte-exact from the embedded corpus. So the slicing machinery is
retired and **byte-exactness survives, at exactly one file**.

**"Keep the `if`, defer the `then`" survives; only its channel changes.** The
split that made selective delivery safe becomes the split between `SKILL.md` and
`references/`. This design does not find that principle wrong. It finds
`${prompt}` the wrong carrier for the `if`.

**Trigger strength is the load-bearing half, not a finishing touch.** Nothing
else in this design answers the *first* observed failure. It is treated here as a
designed surface with its own decisions and its own review, because a pointer
nobody follows delivers nothing at all — which is a worse residue than the wall's.

## Decisions

### The core rule: what earns a place in `${prompt}`

**The too-late test.** A sentence earns `${prompt}` only if its failure mode is
one the skill cannot repair — because by the time the skill could speak, the
moment has passed.

Three shapes pass, and the test is what makes them three rather than a list
someone may extend:

- **A fact only the driver holds.** The skill cannot state it, at any strength,
  because it is not knowable at build time.
- **The instruction to open the skill.** The skill cannot deliver it; it is the
  bootstrap edge.
- **The session's last action.** The skill spoke at the start; the obligation
  falls due after everything else the session does.

What the test **refuses** matters more than what it admits, because every
argument for widening the core will take one of these forms:

- *"This is important."* Refused — every rule in a methodology is important, and
  importance is unbounded. The wall is what importance-as-a-criterion builds.
- *"This is needed in every session."* Refused — frequency is not timing. A rule
  needed in every session is needed *while the skill is open*.
- *"A session got this wrong once."* Refused unless the failure was
  too-late-shaped. Otherwise the fix is a stronger trigger or better skill prose,
  both of which are cheaper and neither of which grows the wall.
- *"It is only one more sentence."* Refused — size is a consequence of the rule,
  never a criterion for admission. A core defended by its byte count is defended
  by nothing, since each addition is individually small.

**The test's first output is a list of one.** `plan-k1` handed this design "the
conditions that must never be missed" as an open plural; applied candidate by
candidate, the test admits exactly one methodology condition. The working is
recorded rather than the conclusion, so that a disagreement lands on the rule and
not on the list:

| candidate | passes? | why |
|---|---|---|
| load the skill; read this kind's reference file | **yes** | the skill cannot deliver it |
| where the skill was provisioned | **yes** | the skill cannot know where it landed |
| the selected leaf's handle | **yes** | a value resolved before the session existed |
| the working tree's version control | **yes** | a value resolved before the session existed |
| run the completion verb as the last action | **yes** | the skill spoke first; this falls due last |
| externalize surfaced work rather than absorbing it | no | the situation arises mid-session, with the skill open |
| retire before commit | no | same |
| name the work item by its handle in the commit message | no | same |
| pruning is HITL | no | the skill is read before any prune is reachable |
| a session never discovers a grove is finished | no | see below — mechanism guards this one, not prose |
| the seven constraints | no | read at the start, applied throughout |
| the in-session doubt budget | no | arises mid-session |
| do not pick again | no | a static rule; only the handle it applies to varies per launch |
| the selected pick is authoritative | no | a static rule *about* a fact; the skill already states it |
| do not probe for the version control | no | same shape — the skill states it, and must |

The last three rows are the shape the rule most often produces at its boundary,
and getting them wrong is how the core grows. **The test is closed on the word
"fact":**

> A driver fact is a **launch-varying value** that `content/` cannot know at
> build time. Its static meaning, and every normative consequence of it, stay in
> the skill.

Without that clause "fact" is unbounded, because almost any rule can be restated
declaratively — *the pick is authoritative*, *the stated VCS is definitive* — and
smuggled in as though it were a value. A value has no counterpart in `content/`
to drift from; a restated rule has exactly the counterpart the drift claim
denies. `content/` states today that the driver makes an authoritative pick and
that a second walk must not override the mandate
(`content/SKILL.md`, `skill-pick` and `skill-do-not-pick-again`), so a core
saying *and it is authoritative* would have been a second source for prose that
already exists.

**One obligation falls out of the closure and belongs to the rewrite.** Today's
runtime-facts prose carries *do not probe for it* on the version-control line.
Under the closed test that clause leaves the core, so `content/` must state it —
the driver's stated version control is definitive and is not re-derived, and a
harness banner disagreeing with it does not win. The requirements below carry
that as a scenario. This is the closure's one real
cost: a rule the core used to carry moves to the skill and depends on the skill
being read, like every other rule.

### Several conditions are guarded by mechanism, not by prose

The superseded risk model treated every triggering condition as though prose were
its only guard. Several are not, and that is part of why this design's residue is
smaller than that model implies.

*A session never discovers a grove is finished* is the clearest case, and it was
the case the old design singled out as "the one 'keep the `if`' exists for",
precisely because a destructive action hangs off it. But teardown is reachable
only through the finish transaction, which requires a driver-created `finish`
sentinel leaf and explicit human confirmation. A session that missed the
condition cannot delete a task tree by accident; it would have to reach for a
verb it was never told about. The prose is worth keeping — it is in the skill —
but the guard is the sentinel and the gate.

The same holds for stale-session action (the session epoch), for a malformed or
foreign task tree (the format witness and the fail-closed readers), and for
launching an unknown kind (configuration validation before any tree mutation).

**Where prose really is the only guard, that is a finding rather than a reason to
widen the core.** `leaf-prune`'s HITL rule is the live example: nothing mechanical
stops an agent pruning on its own. That is a candidate for a guard *in the verb* —
where the confirmation boundary's own tests would place it — and it is recorded
here as a pointer, not settled. Widening the core to cover it would be
importance-as-a-criterion, which the rule refuses.

### The core is `content/` wherever it can be, and driver prose only where it cannot

The superseded record's objection to two delivery paths was that they **can
disagree**. Disagreement needs two sources. This design has one.

- The **load instruction** and the **provisioned locations** are driver prose,
  and have no counterpart in `content/` to drift from: a skill cannot tell you to
  read it, and cannot know which directories a particular driver wrote.
- The **runtime facts** are driver prose for the reason they always were — the
  handle and the [[Stated VCS]] are *values* resolved before the session exists
  and are not expressible in `content/`. This rule is carried forward unchanged
  from the superseded design; it was never contingent on slicing. What is new is
  the closure above: the values ride the core and their normative consequences do
  not, which is what makes "no counterpart in `content/`" true rather than
  merely asserted.
- The **session-ending instruction** is the one genuine duplicate, and it is not
  duplicated: the driver inlines the embedded corpus's own signal file
  **verbatim**, from the same embed that is provisioned. One source, two
  deliveries, and no build boundary between them, since both come from the
  running `grove`'s embed.

So the **prose** drift surface of the two-channel design is zero bytes, and the
property is structural rather than a claim about size: `${prompt}` is assembled
from a fixed three-part template in which exactly one part is embedded content.

**What the two channels do share is four structural couplings**, and stating
"zero drift" without naming them would overclaim. The core names the skill by
**name**, names this kind's reference file by **path**, inlines the signal file by
**path**, and names the **provisioned locations**. Two are closed by construction —
the signal file is embedded at compile time, and the locations are computed by the
same registry that writes them — and the other two are closed by assertion, below.
A coupling held by a check is not drift; a coupling held by nothing is, which is
why they are enumerated rather than covered by a summary claim.

**This is what remains of "slice, never paraphrase", and it is the right
residue.** The composer, the marker grammar and the completeness invariant existed
to make *selective* delivery safe. Delivery stops being selective, so they have
no job — but the reason a slice was byte-exact rather than summarised does not
depend on selection at all, and it applies with full force to the one file that
still travels both ways.

### The prompt's order is the session's own timeline

Three parts, and the order is the only ordering rule this design has:

1. **The load instruction** — first, because it is the first action.
2. **The runtime facts** — the handle and the stated VCS.
3. **The session ending** — last, because it is the last action.

Recency is the whole reason for the third position, and it is inherited rather
than re-derived: the ending instruction was moved to compose last for exactly
this reason, after sessions were seen finishing correctly and then not
signalling. The file-ordering machinery that held it there is retired, and the
property survives as a fixed template a human verifies by eye — which is a
better home for a three-part order than a total ordering key over ten files.

Two consequences are stated rather than hidden. The ending's recency advantage is
much weaker here than it was: it now trails ~1.5 KiB rather than seven files, so
the position is nearly free and buys correspondingly less. And the ending is
delivered **twice** to every session — once in the core, once inside the skill —
which is deliberate, costs no drift, and is the one place this design accepts a
reader's *do these agree?* because the answer is mechanically yes.

### The size alarm is small, and it fires early

**`${prompt}` is asserted at or under 4 KiB for every session kind.** Framed
honestly as what it is: an alarm on the too-late test, not a budget the design was
fitted to.

**Measured on the built composition**, against one provisioned location: an
`impl` prompt is **2,307 bytes** and the largest of the nineteen is 2,318, so 4
KiB leaves about 43% in hand. The design's own estimate was ~1,969, and the
difference is bookkeeping rather than prose: `content/SIGNAL.md` is 1,499 bytes
of which ~390 is the unit marker line listing eighteen kind labels, and that line
dies with the mandate machinery. A real machine with three installed harnesses
adds roughly 80 bytes of location list. The alarm is unchanged, because it was
never a budget the prose was fitted to — and the honest reading of these numbers
is that headroom is *comfortable*, not that it is *large*.

The number is a choice rather than a derivation, and what makes it the right kind
of choice is that **nothing legitimate approaches it**. A core that reaches 4 KiB
has gained roughly a page of prose, which the too-late test admits only if the
driver has acquired a new runtime fact — a rare and visible event. Everything
else that could push it there is importance-as-a-criterion, which is exactly what
the alarm exists to see. It lives in the test suite rather than the build, for
the reason the previous alarm did: it measures a judgement against an admittedly
arbitrary number, and failing a contributor's build on that is a gate this design
is otherwise careful not to erect.

### Trigger strength is the design's load-bearing half

Nothing else here answers the *first* observed failure, so this is a designed
surface with three parts. The failure being designed against is a **discipline**
failure — the session knows it should read the skill and skips it — which is a
classification with a prescribed form: prohibition, a rationalization table, and
red flags. That prescription matters because the same body of guidance warns that
bare prohibitions **backfire** on *shaping* problems, and a later session reading
only that warning would strip the apparatus. It is kept for a reason, and the
reason is that this is not a shaping problem.

**What actually failed before was weaker than it is usually remembered.** The
pre-mandate `${prompt}` was ~1.1 kB: a launcher whose single relevant clause was
*"use the grove skill"*, among other launcher prose. That is not a session
ignoring a forceful instruction; it is a session given a mild one. The reversal is
not a return to that state, and the difference is the whole of this section.

**(1) The frontmatter `description:`.** Its job changes and narrows. It is no
longer the trigger that matters — `${prompt}` names the skill outright — but it
is still the fallback for a session a human started inside a grove working tree,
and it is what a session matches the prompt's instruction against.

The skill stays **model-invoked**. `disable-model-invocation: true` would remove
the fallback and, on a harness that lists only model-invoked skills, would remove
the skill from the list the prompt tells the session to reach into.

The current description is written for a session deciding *whether* to start a
grove ("Use when driving a long, multi-session workstream that cannot be planned
exhaustively upfront"). A session already inside one can read that and conclude
the skill is about the choice rather than about how to run this session — a real
undertriggering path. It is rewritten to the house shape, a **capability clause
plus an explicit "Use when"**, whose first trigger is the situation every
driver-launched session is actually in: *a Grove mandate names this skill*.

**(2) The launcher's wording**, which is the largest part of the core and the
part most likely to be trimmed by a later reader. **Three elements ship, and two
that this section previously carried do not** — the split is the micro-test's
result rather than a judgement, and
[`wording-micro-test`](../research/wording-micro-test.md) is the evidence.

- **One imperative naming both targets.** The skill by name, and this kind's
  reference file by path — the driver resolved the kind before the session
  existed, so the session performs no selection. **This is the element the test
  measured as load-bearing**: the control's failure was not opening the skill —
  it did that in 9 of 10 sessions — but reaching the kind's *procedure* only
  after it had started work, in every session of both arms.
- **An ordering clause that enumerates the tempting alternatives** — before
  reading a task file, before running a `grove-llm` verb, before looking at
  `.grove/`, before answering a question. The enumeration is what makes an
  ordering clause bite; "read this first" without it is advice.
- **The provisioned directories, by absolute path.** This is what makes the
  instruction actionable by plain file read, which is the one capability every
  harness has. A session whose harness offers no skill-loading affordance is
  otherwise handed an instruction it cannot follow.

**A rationalization table and an explicit *this prompt is not a summary* clause
were designed here and are cut.** An ablation arm carrying only the three
elements above scored identically to the full wording — 10/10 against the
control's 0/10 — so nothing measured is attributable to them, and the house
no-op test cuts prose that does not change behaviour. The classification
argument above is *not* what this overturns: the failure is still a discipline
failure and prohibition remains its prescribed form. What the result establishes
is narrower and is the honest ground for the cut — the two elements are
**unmeasured**, the ablation could only have detected a large negative effect
against a ceiling, and unmeasured prose does not ride the one channel a session
cannot skip. Reinstate either if the human-watched acceptance run shows sessions
working from the core as though it were an abridged methodology, or reaching the
procedure late under the real corpus.

**(3) How `SKILL.md` opens.** The first screen routes rather than introduces: it
names the reference file for each kind, so a session that arrived by description
match rather than by mandate still lands in the right place, and it states the
loop within a page — which is constraint 7 made literally checkable for the first
time. It states conditions and no procedure, because a description or an opening
that summarises the *workflow* becomes a shortcut the session takes instead of
reading the body.

**The wording is micro-tested before the rewrite ships.** Everything above is a
*design* for wording, and wording is the one thing in this spec that no amount of
reading settles. The house authoring rule is explicit that behaviour-shaping
wording is micro-tested against a control with at least five fresh-context
repetitions (`plugins/linkuistics/skills/authoring-conventions/SKILL.md`, *Test
the wording, cheaply*), and this section is the design's only answer to the first
measured failure. Leaving it to the human-watched run would make that run the
first experiment capable of falsifying the central claim — after the whole
corpus rewrite and the machinery deletion have landed, in a setting that
confounds wording with everything else that changed.

Two arms, five fresh-context repetitions each, run once as design validation:

- **Control** — the pre-mandate launcher shape: ~1.1 kB whose single relevant
  clause is *use the grove skill*. This is the arm the field failure was measured
  on, so it is the control the house rule asks for rather than a synthetic one.
- **Variant** — the designed core: the imperative naming both targets, the
  ordering clause with its enumeration, the absolute provisioned paths, the
  rationalization table, and *this prompt is not a summary*.

Both arms run against a short stand-in `SKILL.md` and reference file — the real
corpus is not needed to test whether a session opens one — and against the
configured model and harness targets this workstream's own
`~/.config/grove/config.kdl` launches, named in the result, because a wording
result does not transfer across targets. The observable is the same one the
design already relies on: did the session open the skill and name its reference
file before acting.

**It has run.** [`wording-micro-test`](../research/wording-micro-test.md) carries
the arms, targets, repetitions, counts, limitations and the winning wording. The
variant beat the control 10/10 against 0/10 on both targets, so this design's
answer to the first observed failure stands; a third, ablation arm cut two of the
five designed elements, as recorded above.

**Two outcomes are useful, and one of them cuts this section down.** If the
variant does not beat the control, the wording is not the fix and the design's
answer to the first failure is missing — that is a stop, not a tweak. If the
*control* does not exhibit the failure on these targets, the house rule's own
stop clause applies: there is nothing to shape, and the prohibition and
rationalization apparatus is unwarranted apparatus that should be cut rather than
defended. The experiment is cheap enough that either finding is worth more than
the argument it replaces.

This is design validation, not a gate: it runs once, its result is recorded with
the record this design reworks, and nothing in the suite or the build re-runs it.

**The observable, and its honest limit.** The core instructs the session to state,
in its first message, that it has read the skill and to name the reference file it
read. That line is the design's only observable, and it exists because **Grove
structurally cannot see what a session reads**: the configured command is opaque
and owns the real TTY, so the driver never sees a byte of the session's output.
The audience for the line is the human watching the loop — the same human who
reads the pairing report between sessions, and the instrument the grove's own
`Done when` names.

A machine-checked read receipt was designed and rejected on its merits, recorded
here so it is not re-proposed. The available proxy is *did this session run the
bootstrap verbs under its epoch* — and it has a false negative the methodology
itself invites, since constraint 2 says the one command Grove asks for "is a
lookup you could do by eye". A proxy the methodology tells sessions they may skip
reports on the methodology, not on the session. Reopen if a launch target ever
exposes which skills a session loaded.

### The skill's layout is recovered, not invented

**The per-kind reference files are recovered from the existing narrowed marker
scopes, one file per distinct scope — after the ending unit is removed into the
guaranteed core.** The order of those two steps is the derivation, and stating it
loosely would misdirect a mechanical rewrite: the corpus carries **eleven**
distinct narrowed scopes, not ten, because `skill-signal` is narrowed to the
eighteen non-`finish` kinds (`content/SIGNAL.md:2`). That scope is not a family;
it is the session-ending text, which the too-late test moves to `${prompt}` and
which is inlined byte-exact from its own file. Remove it first, and the remaining
ten distinct scopes are exactly the per-kind reference files:

| reference file | the kinds it serves |
|---|---|
| `requirements` | `requirements` |
| `design` | `design` |
| `planning` | `planning` |
| `prototype` | `prototype` |
| `impl` | `impl` |
| `review` | the five `review-*` kinds |
| `integrate-review` | the five `integrate-review-*` kinds |
| `research` | `research-a`, `research-b` |
| `combine-research` | `combine-research` |
| `finish` | `finish` |

Ten files for nineteen kinds, because `content/` already treats each family as
one unit — the five `review-*` kinds share one marker today, as do the five
`integrate-review-*` and the two research producers. The thin `design` file is
kept rather than folded: the driver selects a kind's file directly, so a
one-kind file costs a path and buys the mapping its exhaustiveness.

**The kind→file map is an exhaustive `match` over the kind enum, in the driver.**
This is routing, and routing by kind is what the driver already owns: it reads the
kind from the filename and resolves that kind's configured target. Naming that
kind's reference file is the same act. The map's own hazard — a twentieth kind
silently absorbed into a family's file — is closed the way this repository already
closes it, by an exhaustive match that fails to compile until someone classifies
the new variant, plus a test that every path the match yields exists in the embed.

*One file per kind label, by naming convention, needing no map* was considered and
rejected: it makes fifteen of nineteen files near-duplicates of four, and
duplicated prose is the drift risk one level down from the one being removed.
Reopen if the families ever stop sharing discipline.

**Reference files stay one level deep and flat.** `references/design.md`, not
`references/kinds/design.md` — the house progressive-disclosure rule, and it keeps
the path the core names short enough to read at a glance.

### `SKILL.md` states conditions; `references/` states procedures

This is "keep the `if`, defer the `then`", relocated intact. The 140 unit markers
already record which prose is which — 71 triggering, 69 procedural — so the
classification drives the rewrite and is then deleted with the machinery that read
it.

**The universal triggering units do not transfer verbatim, and that is the
rewrite.** 51 units averaging ~880 bytes are prose containing a condition, not
conditions. Each yields a **condition sentence** for `SKILL.md` and a
**remainder** that joins the procedures in `references/`. Worked, on
`skill-decompose`: ~1.1 kB of prose about externalizing versus absorbing becomes
one line — *work surfaced that does not serve this leaf's stated goal, or the leaf
proved bigger than its brief* → `references/decompose.md` — and the two triggers,
two verbs, the inline-continuation bar and the laziness argument all move to that
file.

Arithmetic, so the target is a bound rather than a hope: ~51 condition lines, a
loop narrative of about eighty, and a ten-row routing table land near 200 lines
and well inside the ~500-line house ceiling, at roughly 8 KiB against today's
50 KiB.

**Grouping the universal procedures is left to the rewrite**, bounded rather than
enumerated: the loop steps and the existing format documents are the natural
seams, the set stays under about eight files beside the ten per-kind ones, and any
file over ~300 lines gets a table of contents. Fixing filenames a session has not
yet written would be over-specification.

### What the reworked records say

**`skill-delivers-the-methodology` is reworked in place and renamed
`skill-delivers-the-methodology`.** The slug is the identity, so a slug that says
the mandate delivers the methodology cannot survive the mandate not delivering
it; and `mandate-` → `skill-` keeps the citation reconciliation mechanical. It
becomes one record, not two: the delivery reversal and the core rule are
**inseparable** — answering either alone is a swap — and two records for one
inseparable decision is one ADR pretending to be two. The core rule is a named
section inside it, with its own reopen conditions under `## Considered options`,
because the predictable failure mode is erosion by addition and erosion needs
somewhere to be argued against.

What it retains from the record it replaces: that a session told a fact succinctly
never runs the derivation that would have established it; that driver-authored
prose about the methodology would make `content/` non-canonical; and that the
`if`/`then` asymmetry is real. What it overturns is argued below.

**`one-build-owns-a-session` gets a targeted rework, not a rewrite.** Its
substance is untouched — the identity is a content hash rather than a version, the
probe resolves through the session's `PATH` from the worktree root, and it reports
rather than refuses because an opaque configured command's environment is not the
driver's to observe. What changes is the paragraph asserting that *since the
mandate delivers the methodology there is no shared directory left to clobber*.
The shared directory returns, and with it the skew the record was originally
written for: two copies of a whole methodology, the provisioned skill and the
resolved CLI. The "split-brain inside one rule" framing goes with the deferral
that produced it, as does the claim that the failure is loud because the deferral
is declared — the returning skew is quiet, which is precisely what the pre-launch
report and the per-verb stamp warning exist for.

One consequence that looks like it should reopen and does not: **the compile-time
methodology-identity constant stays deleted.** It existed so that naming the
identity did not link the embed, and `grove-llm` links the embed for a second,
independent reason that survives — its per-verb foreign-skill-directory warning
needs the identity. So both binaries still carry the payload, and the release
path's assertion that both carry the content marker stays as it is.

**`mandate-delivered-methodology` is replaced by this spec, and deleted by the
increment that removes the last machinery it describes** — not before. Roughly ten
live source sites cite its sections as the rationale for code that still runs
(the parse gate, the fence rule, the file ordering, the composer's runtime-facts
rule), and deleting it now would point live code at a record that no longer
describes it, which is the dangling-citation defect the rework discipline
forbids. That timing is the old spec's own rule — *a record describing mechanism
that has not yet changed stays accurate until it does* — outliving the design that
wrote it. The arguments of its that survive their mechanism are carried into this
document rather than left in VCS history.

**`CONTEXT-MAP.md`'s shared-target relationship stays, minus one clause.** The
`grove` entry in the personal skill directory was recorded as going away; it does
not, so the relationship is a relationship again. Nothing collides today and
nothing new collides: the overlap with the `linkuistics` symlink install is
`~/.codex/skills/` and `~/.pi/agent/skills/`, where the names are disjoint. What
reopens the precedence question is one context provisioning the other's content,
which no part of this design does.

### The argument for overturning "never as a replacement for triggering conditions"

The clause being overturned is the superseded record's own, in its rejection of
pointing at locations: worth re-examining *"only as a supplement for units too
large to inline, never as a replacement for triggering conditions."* Four strands,
and the first three are the record's own material.

**1. The clause's stated premise has been falsified, by the record's own reopen
condition.** The reason a pointer could not replace a condition was granularity —
*"a kind's discipline is one bullet inside a section of nineteen, so a pointer
leaves the session reading the whole section and performing the selection
itself."* The record named the reopen: *"Reopen if `content/` is ever restructured
so that every rule is separately addressable."* The 140 markers made that true,
and this design goes further by restructuring the corpus so a kind's discipline is
a **whole file the driver names by path**. The session performs no selection at
all — the driver resolved the kind before the session existed. The reasoning cost
the clause was protecting against is not present in the thing it forbade.

**2. The risk model counted one failure and the evidence names two.** *Withholding
a condition yields an unasked question* is true and remains true. What the model
never priced is that a condition **delivered inside a wall and not acted on** is,
behaviourally, a condition not delivered — and the superseded design is explicit
that its invariant never claimed otherwise, warning against "reading the invariant
as a promise about detection". So overturning the clause is not accepting a risk
the old design avoided. It is choosing between two ways of failing to deliver,
with measurement on both, where the measured cost of the wall is a **stalled
loop** and the measured cost of the skill was a session that ran without it.

**3. What is withheld is nothing, and the failure changes kind.** Under the
mandate a withheld thing was a procedure the session knew to ask for. Under this
design nothing is withheld: the whole methodology is on disk, in one skill, named
in the prompt, with this session's own file named too. The failure is no longer
*the session was never told there was a question* — silent, and unattributable —
but *the session did not open the document it was told to open*, which leaves a
trace a human watching can see, and which the acknowledgement line is designed to
surface.

**4. The residue is real and is named rather than argued away.** A session that
ignores the pointer gets nothing, where a session that ignored a slice at least
had the bytes in front of it. That is a worse failure per occurrence, and the
whole of what pays for it is trigger strength — which is why that is a
load-bearing section of this spec with its own review, its own micro-test before
anything ships, and why the design accepts the same check the superseded record
nominated: **the next real Grove run after the change lands, with a human
watching**, with both limbs required. Sessions that
end and do not read the skill are a swap; sessions that read it and do not end are
a swap. Neither alone clears this.

### What the mandate machinery was for, and what its deletion costs

The composer, the marker grammar, the fence-state parser, the build gate, the
completeness invariant, the file-ordering directive and `grove-llm methodology`
all go. Recorded here for the increments that delete them: what each bought, and
what pays for it afterwards.

- **The marker grammar and total partition** bought *unclassified prose cannot
  exist*, and made a parser going blind produce a visibly larger unit rather than
  a silent hole. Afterwards there is no parser and no classification, so there is
  nothing to be blind to; the corpus is read by a harness as markdown.
- **The completeness invariant** bought a mechanical answer to *did we build the
  mandate right?*, narrowing the question to *did we classify this one unit
  right?*. It is worth being clear about what it did not buy, in its own words:
  it settled which units a mandate carries, "never that a session notices the
  situation a carried condition describes". Afterwards that question is answered
  by the corpus being delivered whole.
- **The reachability and termination checks** bought *an undiscoverable procedure
  is impossible*. Afterwards discoverability is the skill's routing table and the
  file system, both of which a human reads directly.
- **The build gate** bought a contributor's build error in place of a stranger's
  stalled loop. Afterwards the analogous claims are corpus tests — that every
  kind's reference file exists, that `SKILL.md` stays within its budget — and they
  live in the suite rather than the build for the reason the size alarm does.
- **`grove-llm methodology`** bought addressed, not guessed, lookup of a deferred
  body. Afterwards nothing is deferred.
- **The file-ordering directive** bought a total composition order over ten files.
  Afterwards the only order is the prompt's fixed three parts.

**Two checks in that neighbourhood are claims about the embed rather than about
the machinery, and must survive the deletion.** The scan asserting that the
embedded methodology instructs no `grove-llm` verb the embedded CLI lacks is the
enforceable half of the build boundary, and it becomes *more* load-bearing here,
not less: the skill is once again the only thing teaching a session which verbs
exist. The flat-verb-surface pin is what makes that comparison mean what it
claims. Both are named so that a deletion increment does not take them along with
the parser they currently sit beside.

### Provisioning and build pairing return unchanged

`provision` stays as built: the per-invocation sweep of the embed into every
*installed* harness's personal skill directory before the driver owns a working
tree, guarded by a content-hash stamp, with per-launch re-verification restoring a
directory another build has taken. Single-command `brew install` keeps working and
the stamp keeps meaning something. Shipping via the marketplace beside
`linkuistics` was rejected in requirements: it makes installing Grove two steps
and lets skill and binary versions drift with no check left.

**Extraction already handles nested directories** — the embedded corpus carries a
`LICENSES/` subdirectory today — so `references/` needs no change to the sweep.

**One new report.** If no known harness root exists, nothing is provisioned, and a
session then receives a core pointing at a skill that is not there — a total
failure that is currently silent. The driver says so before every launch. It
reports and never refuses, on the line Grove's surface already draws: it stops on
what governs its own operation and reports what it can only predict about a
session's environment, and which harness an opaque configured command reaches is
firmly the latter.

**The unsupported shape is named rather than defended.** A launch target that can
receive a large `${prompt}` but cannot read a provisioned skill now gets a session
with no methodology at all. The driver cannot detect it, for the same reason it
cannot detect the harness. The harness registry is what "supported" means, and
this is the mirror of the reopen condition the superseded record carried in the
other direction.

### The cutover has two ordering constraints

Increment ordering is a planning concern with two exceptions that are design
facts.

**The wording micro-test runs before the core's wording ships.** It is cheap, it
needs no part of the rewrite, and its whole value is being able to change the
design's answer to the first observed failure while changing the design is still
free. Run after the rewrite lands, it is a post-mortem.

The second is the one the increments must be ordered around:
**`${prompt}` must not shrink before `SKILL.md` is short.** A core delivered
against an unrewritten corpus hands the session a ~50 KiB `SKILL.md` in one gulp,
which reproduces the wall at first invocation; and a rewritten corpus delivered
alongside a full mandate reproduces it at launch. Either half alone is one of the
two measured failures.

## Requirements

### Requirement: `${prompt}` carries the guaranteed core and nothing else

The driver SHALL compose `${prompt}` from three parts in order — the load
instruction with the provisioned locations and this kind's reference file, the
runtime facts, and the session-ending text — and SHALL include no other
methodology prose. The third part SHALL be present for every kind whose ending is
one fixed instruction, and SHALL be absent for `finish`.

#### Scenario: the three parts, in order
- **WHEN** a session of any kind but `finish` is launched
- **THEN** `${prompt}` contains the load instruction, then the selected handle and
  the stated version control, then the session-ending text, and nothing else

#### Scenario: a `finish` session takes no fixed ending
- **WHEN** a `finish` session is launched
- **THEN** `${prompt}` ends at the runtime facts, and names no completion verb
- **AND** the reference file it names states all three of that session's endings

**This is the one exception to the three-part shape, and it is a correctness
exception rather than a tidiness one.** Eighteen kinds end exactly one way and
`content/SIGNAL.md` says so. A `finish` session has three endings chosen by what
it did — `complete --done` after teardown, bare `complete` if it externalised work
instead, and no signal at all if the human declined or was absent. Inlining
`SIGNAL.md` for it would put *run `grove-llm complete`* last in the prompt of the
one session that may have just deleted the task tree, relaunching the loop onto a
torn-down grove which the driver then re-scaffolds. The too-late test admits *the
session's last action*; it does not license stating the wrong one. A `finish`
session's ending rides its reference file, which the load instruction names first
and by path.

#### Scenario: no prompt states an ending outside its ending
- **WHEN** `${prompt}` is composed for each member of the closed kind set
- **THEN** no part before the session-ending text names the completion verb, and
  no prompt of any kind names `--done`

#### Scenario: the ending is embedded content, not a copy
- **WHEN** the session-ending text is compared with the embedded corpus's signal
  reference file
- **THEN** it is that file's bytes, unmodified

#### Scenario: the skill is named as the embed names itself
- **WHEN** the load instruction's skill name is compared with the embedded
  `SKILL.md`'s own `name:`
- **THEN** they are equal — the core points at a skill by the name a harness will
  have registered it under

#### Scenario: size alarm
- **WHEN** `${prompt}` is composed for each member of the closed kind set
- **THEN** each is at or under 4 KiB

### Requirement: every session kind names an existing reference file

The driver SHALL resolve every session kind to one reference file in the embedded
corpus, by a total mapping over the closed kind set.

#### Scenario: a kind added later
- **WHEN** a session kind is added to the closed set and the mapping is not
  extended
- **THEN** the build fails, rather than a session being launched with a dangling
  pointer

#### Scenario: a reference file that does not exist
- **WHEN** the mapping names a path the embedded corpus does not carry
- **THEN** the assertion fails, naming the kind and the path

#### Scenario: a family's kinds share one file
- **WHEN** `${prompt}` is composed for each of the five `review-*` kinds
- **THEN** each names the same reference file

### Requirement: the skill is progressively disclosed

`SKILL.md` SHALL state the loop, the conditions, and the routing to reference
files, and SHALL state no procedure. Its body SHALL stay within the house
progressive-disclosure ceiling.

**Two of those limbs are mechanical and one is not, and the split is stated so
the suite is not read as establishing more than it does.** Line budgets are
mechanical. *Procedure* has no classifier once the unit markers are deleted —
that classification was the markers' whole job — so the no-procedure limb is a
**review obligation**, discharged by a human against named evidence, not by a
test. A corpus-budget test that passes says nothing about it.

#### Scenario: body budget
- **WHEN** the embedded `SKILL.md` is measured
- **THEN** its body is at or under 500 lines

#### Scenario: the loop section fits a page
- **WHEN** the loop section is measured — the lines from its heading to the next
  heading of the same level, blank lines included
- **THEN** it is at or under 100 lines

This is constraint 7 made checkable, and the measure is deliberate rather than
implied: "a page" is otherwise unmeasurable, and a number that no reader can
recompute is a SHALL with no verification boundary. 100 lines is the alarm on a
narrative the rewrite estimates at ~80, chosen the way the 4 KiB alarm was — as
the point where growth has become visible, not as a budget the prose is fitted
to.

#### Scenario: no procedure in `SKILL.md`
- **WHEN** the rewritten `SKILL.md` is reviewed
- **THEN** a reviewer confirms, per section, that each states a condition and
  routes to a reference file, and that no section states steps a session
  performs — the evidence being the section itself against the reference file it
  routes to, which is where the corresponding procedure must be found
- **AND** this is recorded as a review finding, not a test result

#### Scenario: the skill carries the rules the core sheds
- **WHEN** the rewritten corpus is reviewed against the closed fact test
- **THEN** it states that the driver's pick is authoritative and must not be
  re-walked, and that the driver's stated version control is definitive and is
  not re-derived from the working tree or from a harness banner

#### Scenario: a session that arrives without a mandate
- **WHEN** the skill is opened by a session Grove did not launch
- **THEN** its opening routes that session to a reference file by kind, without a
  mandate having named one

### Requirement: an absent skill destination is reported

The driver SHALL report, before every launch, when no known harness root exists,
and SHALL launch anyway.

#### Scenario: no harness installed
- **WHEN** no known harness home marker is present
- **THEN** the driver prints one diagnostic naming the roots it looked for, and
  the launch proceeds

#### Scenario: at least one harness installed
- **WHEN** any known harness root exists
- **THEN** nothing is printed — absence of a destination is the only claim on
  offer, and it cannot be made about a machine that has one

## Test seams

**`prompt` is a new seam and `methodology` narrows; it is not a one-for-one
replacement.** The architecture's module-seam table gains a row and rewrites
one — the shape is stated here because leaving it undecided would hand planning
an ownership question dressed as bookkeeping.

- **`prompt`** (new) exposes composition over `(kind, handle, stated VCS,
  provisioned locations)` and the kind→reference-file mapping.
- **`methodology`** (narrowed) keeps the embed handle and the methodology
  identity, and loses the two readers, the unit model and `compose`. Both
  survivors are live: `provision` consumes the whole embed (`src/methodology.rs`,
  `embed()`), and the identity feeds the per-launch pairing report and the
  per-verb stamp check (`src/methodology.rs`, `identity()`; `src/provision.rs`,
  `reverify_installed`). Its table row is rewritten to *the embed itself and the
  build's methodology identity* — nothing about units, composition or readers.

`prompt` **depends on** `methodology` rather than absorbing it: composition reads
the embed to inline the ending file and to assert its mapped paths exist. Moving
embed ownership into `prompt` would put provisioning's supplier behind a
prompt-composition seam, which is the inversion the narrowing avoids.

Every interesting check runs through the `prompt` seam against the real embed.
The driver's launch path is a thin wrapper covered by the existing loop-driver
seam.

The checks it carries:

- **The three-part shape**, asserted structurally rather than by substring: the
  composed prompt is the template with its four substitutions, and exactly one
  substitution is embedded content. What the wording *says* is not mechanically
  checkable and is not claimed to be — it is carried by the review of this
  design's trigger-strength section. Recording where the automated boundary stops
  is the same discipline the ending guard it replaces followed.
- **The ending is the embedded file's bytes**, so a driver-side copy cannot
  reappear as a Rust literal without failing — and its complement, that no part
  before it names the completion verb and no prompt names `--done`. The
  mandate-era version of that complement asked what a *composed mandate* carried
  and went with the mandate; asked of the prompt it is narrower and sharper,
  because the prompt is the channel a session cannot skip.
- **Both ends of the closed fact test.** The prompt carries none of the four
  phrases the retired normative tails used, *and* `content/SKILL.md` still
  declares the two conditions that replaced them. An absence asserted alone is
  indistinguishable from a rule deleted rather than moved, which is the closure's
  one real cost going unpaid.
- **The two couplings not closed by construction**: the skill name the core
  states against the embedded `SKILL.md`'s own `name:`, and every mapped reference
  path against the embed. Both fail by name, which is what keeps "the prose drift
  surface is zero" from being read as "there is nothing to hold in step".
- **The kind mapping, generated from the closed kind set**: every kind resolves,
  every resolved path exists in the embed, and family members agree. Generated
  rather than enumerated, so a twentieth kind fails until it is classified —
  the precedent is the existing kind-guidance suite, which already generates its
  claims this way.
- **The size alarm**, per kind, at 4 KiB, counting the composed prompt.
- **Corpus budgets**: `SKILL.md`'s body length, the loop section's length under
  the heading-to-heading measure above, and a reference file over ~300 lines
  carrying a table of contents. These are line counts and establish nothing
  semantic — the no-procedure limb is a review obligation with its own scenario,
  and no budget test may be cited as evidence for it.
- **The instructed-verb scan and the flat-verb-surface pin**, kept — they are
  claims about the embed, they survive the machinery they currently sit beside,
  and they matter more once the skill is the only thing naming verbs to a session.
- **A control on every generated claim.** A sweep that cannot fail is worth
  nothing: the mapping check is shown failing on a kind whose path is removed, and
  the size alarm on a synthetic oversized prompt.

**Golden per-kind prompt snapshots are dropped**, and the reasoning is the
composition golden's own: the ids-not-bytes golden existed because nineteen ~48 kB
mandates could not be held as bytes. Nineteen ~2.3 KiB prompts differ only in one
path and one handle, so the mapping check above says everything a golden would and
says it by name.

## Out of scope

- **Cutting the increments.** The decomposition follows from this spec and belongs
  to a `planning` leaf.
- **A machine-checked read receipt.** Rejected on the merits above: Grove cannot
  observe what a session reads, and the one available proxy has a false negative
  the methodology itself invites. Reopen if a launch target exposes which skills a
  session loaded.
- **A guard in `leaf-prune` for its HITL rule.** Named as a real gap the core
  deliberately does not cover; settling it is its own decision about the
  confirmation boundary, not part of a delivery design.
- **Behavioural evaluation as a gate.** Unchanged from the design this replaces:
  a standing eval in the suite or the build is expensive, non-deterministic, it
  measures a model rather than Grove's artifact, and it localizes nothing when
  red. Two things are *not* out of scope, and the distinction is between a gate
  and an experiment. The **wording micro-test** is required before the rewrite
  ships — five reps an arm, run once, recorded, re-run by nothing. The
  **human-watched run with both limbs** is required as the end-to-end acceptance
  check. Neither becomes a gate; a gate is what a contributor's build or a `cargo
  test` run has to satisfy, and these are experiments whose results are read by a
  human once.
- **Harness-specific loading mechanisms** — a hook, an MCP server, an injected
  system prompt. Grove executes the configured command directly and adds no
  hidden harness-specific argv, and a per-harness delivery path would be launch
  policy Grove does not own. Reopen only if a harness offers a *standard* skill
  preload the Agent Skills spec defines.
- **Shipping the methodology through the marketplace beside `linkuistics`.**
  Settled in requirements: two installs, and no check left on skill/binary drift.
- **Making the `linkuistics` plugin part of this delivery.** Unchanged: ADR
  philosophy, seam judgement and the Jujutsu lane live in a separately installed
  plugin and are not embedded. The dependency stays documentation-level.
- **Moving the build boundary.** A session reads the methodology its own build
  carries. This design widens the boundary's exposure — the shared mutable
  directory returns — and that cost is priced by `one-build-owns-a-session`
  rather than re-decided here.

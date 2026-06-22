# Driving a grove well — field guide

`docs/grove.md` is the methodology: *what* grove is and *why*.
[Workflows](workflows/) are the verb-by-verb walkthroughs: what `grove
do`, `grove retire`, etc. actually do. This file is the third
companion — the field guide to *driving* a grove well as the human
collaborator. The moves and habits below turn the loop into productive
design work rather than ceremonial bookkeeping.

The worked example throughout this doc is grove's own
**`refactor-to-archon`** workstream — the refactor that shed grove's
machinery to a self-extension core and put it on a self-driving loop.
Its founding premise was literally "refactor grove to be an
[Archon](https://archon.diy) workflow"; the workstream's own research
then *reversed* that premise. Paths below point at durable artifacts
from that workstream — the research doc, the ADRs — so the chain from
concern → research → grilling → ADR is traceable end to end.

## When to commission prior-art research

A planning leaf is the right unit for a grilling session when the
design tree fits in one session. When the leaf's design depends on
lessons that prior tools have already learned the hard way — and
those lessons are not obvious from the current codebase — *insert a
research leaf ahead of the planning leaf*. The research leaf's job is
to surface the failure modes the planning leaf would otherwise have
to learn from scratch.

**Signs you want a research leaf:**

- The planning leaf sits in an architectural neighbourhood with
  well-known prior art (workflow engines, agentic loops, issue
  trackers, sync protocols, build systems, etc.).
- Earlier leaves in the subtree have already touched architectural
  decisions the new leaf depends on — i.e. several downstream
  planning leaves share a common evidence base.
- The planning conversation surfaces a question like "has anyone
  tried this before, and what happened to them?" — that question is
  the signal.

The refactor's leaf `loop-substrate-spike` (now retired) is the
worked example. The one open foundational fork was *which substrate
drives grove's fresh-context loop* — and the founding premise (adopt
the Archon workflow engine) was exactly the kind of load-bearing
assumption that deserved evidence before commitment. The spike was
inserted *before* the `substrate-decision` planning leaf that would
choose, and its findings reversed the premise: Archon was rejected,
and the decision (ADR-0031/0032) landed on a thin, grove-owned loop
instead.

## How to write a research-leaf brief

The single most leveraged move is to **name the downstream decision
the research must feed, and the questions it must answer**. The
researcher doesn't have to guess what's load-bearing.

The substrate spike's brief did exactly that. It named the deciding
leaf (`substrate-decision`) as the audience, and pinned the verdict to
**three pass/fail gates**, marking the make-or-break one explicitly:

> **(C) fresh-context-per-task** — can it automate "run one task,
> clear/exit the context, start the next fresh"? **(D) interactive
> grilling *inside* the loop** — can a multi-turn human interview
> happen mid-iteration, then the loop resume? *This is the
> make-or-break; a candidate that fails D is disqualified for
> planning tasks.* **(B) restart-safety** — after a crash mid-loop,
> does re-invoking resume from tree state, not a marker file?

It also handed the researcher a **named hypothesis to confirm or
refute against a source**: that Archon's `loop`/`until` + `depends_on`
could express *both* restart and loop-until-empty with one declarative
`grove-llm pick` evaluation. (It was refuted — Archon's restart is a
separate DB-keyed `resume`, not the same `pick`.) A named hypothesis
cuts the researcher's degrees of freedom far more than "go find out
about Archon" would.

**Bias the search.** The brief framed the recommendation as *which
complexity to own* — an external dependency (Archon) vs. brittle
terminal automation vs. a small but real supervisor vs. autonomy-only
— and told the researcher to weigh grove's own documented history of
shedding process machinery (ADR-0028) against the cost of a new
dependency. It also flagged that training-data recall of Archon was
known-stale (the project had been rewritten ground-up), so the
evidence had to come from primary sources, not memory. The resulting
doc fetched every Archon claim from the live branch and dated them.
Both moves cut the researcher's degrees of freedom in productive
directions.

**Demand a walk-away check per candidate.** For each substrate the
brief required one answer: with it uninstalled mid-grove, is the
committed `.grove/` tree + git still fully legible, or does the
substrate hide load-bearing state in a DB? This is the cheapest
invariant to require and the most revealing — it separates the
architectures that can be borrowed from the ones that cannot. It was
decisive here: Archon holds live run state in SQLite/Postgres, so a
half-finished run is not legible from git alone.

**Demand a citation per capability claim.** "Archon's `interactive:`
can't do open-ended grilling" without a source is mood, not evidence.
The brief required every capability claim to cite a primary source by
URL and quote the load-bearing bits; the resulting options doc cited
Archon's own docs and workflow schema directly. When you later sit
down to write the ADR, those citations *are* the ADR's rationale
section — ADR-0032's "Rationale (from the spike)" is built from them.

**Acknowledge missing sources.** When the researcher searches and
finds silence, that's a finding too. The brief said to record "no
primary source found" as an explicit finding rather than guessing —
the absence is itself a confidence signal, and recording it stops
future readers from re-doing the same fruitless search.

## When to invoke a design discussion (grilling)

The trigger is: a planning leaf's brief lists three or more questions
whose answers interdepend. Grilling is the procedure that walks the
dependency tree without the LLM making decisions on the human's
behalf.

The grilling skill (bundled in the global skill at
`~/.claude/skills/grove/grilling.md`) states it briefly: interview one
question at a time, propose a recommended answer for each, walk down
the design tree until shared understanding is reached. The moves below
make that interview productive rather than ceremonial.

### Ask the LLM "WDYT" before committing

When a question feels close to settled, the easy default is to nod
and move on. **Don't.** Ask the LLM what it thinks — explicitly,
including when you already have a strong view yourself.

Two things happen when you ask:

1. The LLM produces a recommendation it would otherwise suppress out
   of deference. The recommendation is informed by the bootstrap
   context (glossary, briefs, ADRs, research) which is exactly the
   evidence base you want surfaced before you commit.
2. If the LLM's recommendation diverges from yours, that divergence
   is the cheapest signal you have that the question still has
   something to teach. Don't dismiss the divergence — interrogate
   it.

The substrate grilling is the worked case for point 2. The user
opened with a concrete mechanism — a transparent **PTY wrapper** that
watches the agent's output for a completion sentinel (the spike's
candidate 3). Rather than build straight to that opening proposal, the
grilling kept interrogating it, and the design moved twice: first the
completion signal became an out-of-band `grove-llm` verb instead of a
fragile stream-scan, and *then*, because the signal no longer sat in
the human↔agent I/O path, the PTY wrapper was dropped **entirely** in
favour of a native foreground `claude` plus an out-of-band kill (the
running log's D3 → D4). The shipped mechanism is materially simpler
than the one the session opened with — the gain came from not stopping
at the first settled-looking answer.

### Ask for pushback when the LLM agrees too easily

Models default to agreement under social pressure. When the LLM
agrees with your proposal without surfacing trade-offs, push back
yourself: "What would push you toward the other option?" or "What
breaks if we do it this way?" or simply "Pushback please."

The grilling format is built around recommended-answers-with-evidence
specifically to make pushback structural rather than personal. When
the recommendation cites primary evidence, the pushback is a debate
about that evidence, not about whose preference wins.

### Don't merge questions

The grilling skill's "ask the questions one at a time" rule is
load-bearing. Two questions in one prompt — even closely related
ones — produce answers that conflate. Resist the urge to batch.

Where two questions truly interdepend, sequence them: ask the
*foundational* one first, propose the recommended answer, wait, then
ask the *derived* one with the foundational answer already in hand.
The substrate grilling settled the **completion signal** (a task
fires an explicit `grove-llm` verb when it's done) before it could
settle the **interrupt semantics** (the loop relaunches *only* when
that signal fired, so a human Ctrl-C stops the loop instead of being
trapped in a respawn) — because "relaunch only on the signal" is
meaningless until you have decided there *is* a signal to gate on.

### Record decisions inline while they're fresh

The running-log pattern: each settled question gets a paragraph
appended to a `## Decisions (running log)` section in the task file
*at the moment it settles*, not at the end of the session. This
serves two purposes:

- **Survives interruption.** A grilling session can run an hour or
  more; if the conversation drops, the log is the source of truth
  for what has and hasn't been settled.
- **Produces audit trail without a separate phase file.** The grove
  spine's first constraint — *artifacts, not state* — forbids phase
  files; the inline log gives the same legibility for free.

The `substrate-decision` leaf is the worked example: its running log
carries dated, numbered decisions (D1–D6) plus an explicitly-flagged
**open issue** (the loop's interrupt/stop semantics) recorded
mid-session and deferred to the wiring leaf rather than lost. The
decisions log is *not* the ADR. ADRs come at the end of grilling
(or sometimes during, if a decision is genuinely durable enough to
deserve one). The log is for the conversation; the ADR is for the
durable record.

## When research findings retire into ADRs

Research outlives the grove that commissioned it (grove constraint
6, *walk-away-able*). ADRs are the place where research findings
become *binding* on future work.

The rule of thumb: a research finding that *changed a decision* gets
cited in the relevant ADR's rationale section. A research finding
that *confirmed an existing decision* gets a "validated here
against…" note in the relevant ADR, or stays in the research doc
with a forward pointer.

The substrate spike is the bridge in action: its findings reversed
the refactor's founding premise, so they are cited directly in
**ADR-0032**'s rationale — "Archon fails the make-or-break gate D,"
"the named hypothesis is refuted," and "walk-away regression" each
trace back to the options doc, which ADR-0032 names as its evidence.
A future reader of either artifact can trace the evidence chain
without re-doing the research.

See `docs/research/loop-substrate-options.md` (the spike) and
`docs/adr/0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md`'s
rationale section for the worked end-to-end.

## Anti-patterns

- **The rigid wizard.** Turning an open design question into a fixed
  prompt sequence ("pick A or B; now C or D") breaks the grilling's
  whole point — it can't follow a concern the script didn't
  anticipate. Grilling is a walk down a *discovered* tree, not a
  form. If you find yourself answering pre-set questions in order
  regardless of where the answers point, stop and follow the live
  thread instead.
- **The decision summary at session end.** Don't reconstruct
  decisions in the commit message or a session-summary file. The
  inline running log and the ADRs together are the durable record;
  anything else duplicates and rots.
- **The "ask if you have questions" non-prompt.** Vague invitations
  produce vague responses. If you want input on a specific
  trade-off, ask about that trade-off specifically. The grilling
  format's recommended-answer-with-evidence structure makes the
  trade-off visible by construction.
- **The pre-baked answer.** If you already know the answer and just
  want it executed, you don't need a grilling session — that's a
  work task. The grilling discipline exists for genuinely open
  decisions; using it for pre-decided ones is theatre.
- **The runaway tree.** Decomposition is meant to be lazy (grove
  constraint 4, *lazy and optional*). If a planning session grows
  new child leaves faster than you can settle the current question,
  stop and ask whether the parent question was the wrong unit. When a
  concern genuinely must sequence ahead of existing work,
  `grove-llm leaf-insert` absorbs it with one renumber — but if the
  inserts start stacking, that is the signal to pause and
  consolidate, not to keep extending the brief.

## The shortest version

If you remember one paragraph of this doc, remember this:

> Commission research with a brief that names the downstream
> decision, the pass/fail gates, and any named hypothesis; demand
> primary-source citations and per-candidate walk-away checks. When
> the research is in, grill one question at a time with
> recommended-answer-and-evidence; ask the LLM "WDYT" before you
> commit and ask for pushback when it agrees too easily; record each
> settled decision inline in the task file as it lands. At the end,
> ADRs cite the research by primary source; the research doc stays as
> the legible evidence trail. That's the loop.

## See also

- `docs/grove.md` — what grove is and why, and the shorter "Steering
  a planning session" subsection covering the interrupt and
  foundational-ask moves.
- `docs/workflows/` — verb-by-verb walkthroughs.
- The bundled `grilling.md` in the global skill
  (`~/.claude/skills/grove/grilling.md`) — the LLM-side grilling
  procedure.
- The bundled `driving.md` in the global skill
  (`~/.claude/skills/grove/driving.md`) — the LLM-side companion to
  this doc.

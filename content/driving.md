# Driving a grove session well

The grove `SKILL.md` and `grilling.md` files state *what* the loop is. This
file is about *how* to drive it well — the moves a human collaborator makes
that turn the loop into productive design work. It is a field guide, not a
specification; treat it as a starting set of habits, not a checklist. Most of
it concerns *planning* sessions — research and grilling — but the later
sections are habits for *work* sessions too: grounding framework decisions in
the source, doubting a decision before it stands, and externalizing surfaced
work into new leaves rather than absorbing it.

The dogfood reference throughout this doc is the
`capture-issues-for-later-groves` workstream in the grove project's own
repo, which is the worked example for every pattern below. Paths point at
artifacts in that workstream so a reader can trace a real chain
end-to-end.

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/wayfinder/SKILL.md, "Chart the map" step 2's no-fog
     early exit) — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

## When not to start a grove

Before reaching for `root-init`, check that the journey actually needs a map.
If a first planning session surfaces no real fog — the path to done is
already clear and the whole thing fits in one sitting — do the work directly
instead: a grove would only wrap it in a tree, a brief chain, and a relaunch
cycle it doesn't need. Reach for `root-init` once genuine session-to-session
fog shows up — work whose shape you can't see past the next session or two.
`root-init` will always mint a first planning leaf on request; that
mechanical fact is not itself the signal to run it.

## When to commission prior-art research

A planning leaf is the right unit for a grilling session when the design
tree fits in one session. When the leaf's design depends on lessons that
prior tools have learned the hard way — and those lessons are not
obvious from the current codebase — *insert a research leaf ahead of
the planning leaf*. The research leaf's job is to surface the failure
modes the planning leaf will otherwise have to learn from scratch.

**Signs you want a research leaf:**

- The planning leaf sits in an architectural neighbourhood with well-known
  prior art (issue trackers, sync protocols, schema registries, etc.).
- Earlier leaves in the subtree have already touched architectural
  decisions the new leaf depends on — i.e. several downstream planning
  leaves share a common evidence base.
- The planning conversation surfaces a question like "has anyone tried
  this before, and what happened to them?" — that question itself is the
  signal.

The grove workstream's leaf
`050-research-in-repo-issue-trackers.md` (retired) is the worked
example. It was inserted *after* the design's first ADRs had been
written but *before* four downstream planning leaves (060, 070, 080,
090). Its findings de-risked the sync-semantics grilling specifically
(the post-mortem survey's "Synthesis for 060" section pre-judged most
of the design tree the 060 grilling had to walk).

## How to write a research leaf brief

The single most leveraged move is to **name the downstream questions the
research must answer**, leaf by leaf. The researcher doesn't have to
guess what's load-bearing.

The 050 leaf's brief (since retired) put it this way:

> The audience is four open planning leaves: 060, 070, 080, 090. Each
> system section ends with a *takeaway for grove* pointing at the
> leaf(es) its lesson informs. The final **Synthesis** section answers
> the leaf-specific questions in one place.

Followed by a list of concrete questions per leaf. The output
(`docs/research/in-repo-issue-tracker-postmortems.md`) is structured
around those questions.

**Bias the search.** "What's already been tried" produces broad,
shallow surveys. "What went wrong after years of real multi-user,
multi-machine use" produces post-mortems. The 050 brief explicitly
demanded post-mortem framing per system, and explicitly biased toward
*non-obvious paradigms* (the previous prior-art survey had already
established that Linear/GitHub-Issues integration was the obvious
fallback to beat, not the starting candidate). Both moves cut the
researcher's degrees of freedom in productive directions.

**Demand a walk-away check per system.** For each prior tool, the
researcher must answer: with the tool uninstalled, what is still
legible? This is the cheapest invariant to require and the most
revealing — it separates the architectures that can be borrowed from
the ones that cannot.

**Demand a citation per failure-mode claim.** "git-bug had this
problem" without an issue link, blog post, or thread quote is mood,
not evidence. The 050 brief required primary sources; the resulting
postmortem doc cites primary issues by URL and quotes from them
directly. When you later sit down to write an ADR, those citations
*are* the ADR's rationale section.

**Acknowledge missing sources.** When the researcher searches and
finds silence, that's a finding too. The 050 doc flags multiple "no
primary source found" notes (e.g. Google's internal use or non-use of
git-appraise, CRDT-merge surprises in Radicle COBs) — the absence is
itself a confidence signal, and recording it stops future readers from
re-doing the same fruitless search.

## When to invoke a design discussion (grilling)

The trigger is: a planning leaf's brief lists three or more questions
whose answers interdepend. Grilling is the procedure that walks the
dependency tree without the LLM making decisions on the human's
behalf.

The grilling skill (`grilling.md`) says it briefly: interview one
question at a time, propose a recommended answer for each, walk down
the design tree until shared understanding is reached. The moves
below make that interview productive rather than ceremonial.

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

In the 060 grilling, the LLM's recommendation for inbox-shape was
"directory of files" (heavily pre-judged by the research). The
recommendation for entry-naming was a UUID short-suffix. The user's
"WDYT, but the slug should be descriptive" steer shifted the
sub-decision toward content-hash suffixes for idempotency — a
materially better outcome than either party's initial proposal. That
particular step exists in the conversation record because the user
asked for the LLM's view before committing.

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
load-bearing. Two questions in one prompt — even closely related ones
— produce answers that conflate. Resist the urge to batch.

Where two questions truly interdepend, sequence them: ask the
*foundational* one first, propose the recommended answer, wait, then
ask the *derived* one with the foundational answer already in hand.
The 060 grilling sequenced Q1 (shape) before Q5 (entry naming)
specifically because the entry-naming decision is only meaningful
once shape is settled.

### Record decisions inline, while they're fresh

The running-log pattern: each settled question gets a paragraph
appended to a `## Decisions (running log)` section in the task file
*at the moment it settles*, not at the end of the session. This
serves two purposes:

- Survives interruption. A grilling session can run an hour or more;
  if the conversation drops, the log is the source of truth for what
  has and hasn't been settled.
- Produces audit trail without a separate phase file. The grove
  spine's first constraint (`SKILL.md` constraint 1) forbids phase
  files; the inline log gives the same legibility for free.

The decisions log is *not* the ADR. ADRs come at the end of grilling
(or sometimes during, if a decision is genuinely durable enough to
deserve one). The log is for the conversation; the ADR is for the
durable record.

## When to retire research into ADRs versus leave it in `docs/research/`

Research outlives the grove that commissioned it (constraint 6).
ADRs are the place where research findings become *binding* on
future work.

The rule of thumb: a research finding that *changed a decision* gets
cited in the relevant ADR's rationale section. A research finding
that *confirmed an existing decision* gets a "validated here against
…" note in the relevant ADR, or stays in the research doc with a
forward pointer. Either way you are editing the ADR **in place** — the
set is current-state, so a finding that overturns a recorded decision
rewrites (or merges / deletes) that ADR rather than spawning a
superseding one; see *Reworking ADRs and briefs* below.

The 060 grilling's "Findings adopted" pattern is the bridge in both
directions: the research doc gets a section pointing forward at the
ADRs its findings landed in, and each ADR has a rationale section
citing the survey by primary source. A future reader of either
artifact can trace the evidence chain without re-doing the research.

## Reworking ADRs and briefs as understanding shifts

An ADR set is a **minimum coherent set describing the current design**, not a
changelog (`linkuistics:decision-records`). So when a later leaf's work *changes*
a decision an earlier ADR recorded, the move is **not** to add a new ADR that
supersedes the old one — that grows exactly the chronology grove rejects (it is
constraint 1 again: the artifacts hold the present, git holds the past, now
applied to `docs/adr/`). Instead:

- **Edit in place.** Rewrite the affected ADR to state what *now* binds and why.
  Merge two ADRs whose decisions have converged; split one that turned out to
  cover two independent calls; delete one whose decision no longer holds (git
  keeps it). No `superseded by`, no status line.
- **Keep the set minimal.** After the edit, the set should be the fewest ADRs
  that coherently explain the current design. A rework that leaves both the old
  ADR *and* a new one live has failed the test.
- **Reconcile every citation.** An ADR you merged or deleted leaves dangling
  references behind. Grep its slug across the briefs, the other ADRs, and
  `docs/`; fix or drop each pointer. A dangling citation is a defect, not
  acceptable collateral — and when the rework is big enough that you might miss a
  caller, run the doubt pass (below) over the reconciled set.

The natural checkpoints are the two the loop already has: a **grilling session**
that changes a recorded decision, and **retiring** a leaf or node (`SKILL.md`'s
Plan and Retire steps), where you promote brief material upward and reconcile the
ADR set in the same pass.

<!-- The two sections below are adapted (paraphrased into grove's voice, not
     bundled verbatim) from addyosmani/agent-skills@13e43f23 —
     skills/source-driven-development and skills/doubt-driven-development.
     MIT licensed; see LICENSES/addyosmani-agent-skills.LICENSE. -->

## Verifying framework decisions against the source

The research-leaf discipline — a citation per claim, primary sources, and a
note for what you *couldn't* verify — is not only for research. It applies to
**work tasks** too, whenever you write framework- or library-specific code
whose correctness depends on the version. Training data goes stale; an API you
"remember" may have been deprecated two releases ago.

For that kind of code — and only that kind, not version-invariant logic,
renames, or plumbing:

- **Read the manifest first.** `Cargo.toml`, `package.json`, `pyproject.toml`,
  `go.mod` — whatever pins the version. The version decides which pattern is
  correct, so guessing it defeats the exercise.
- **Fetch the official source, not your memory.** Context7
  (`resolve-library-id` → `query-docs`) or your harness's web-fetch tool
  against the project's own docs or changelog — docs.rs for a crate, the
  framework's reference for a framework. Official docs and changelogs outrank
  Stack Overflow, blog posts, and training data. This is reading, not running:
  constraint 2 is satisfied.
- **Cite at the decision site.** A one-line code comment with the source URL
  next to the non-obvious call, so the next reader can check it without you. A
  citation that lives only in the chat evaporates; one in the code is
  walk-away-able.
- **Flag what you couldn't verify.** "No official source found; based on
  training data, verify before relying on it" beats false confidence — and, as
  with the research-leaf "no primary source found" note, the absence is itself
  a finding.

When the decision is also hard to reverse, this pairs with the doubt pass
below: cite the source *and* have a fresh context try to break it.

## Doubting a decision before it stands

"Ask the LLM WDYT" and "ask for pushback" are the cheap version of doubt: the
same context second-guessing itself. But a long session turns its own
assumptions into "facts," and the context that produced a decision shares its
blind spots. The stronger move is to materialise a reviewer that **never saw
your reasoning** and ask it to *disprove*, not approve.

Reach for it when a decision clears the bar that already governs ADRs — hard to
reverse, surprising, or a real trade-off — *or* when it asserts a correctness
property the compiler can't check for you: thread-safety, ordering,
idempotence, an invariant. One trigger, two consequences: such a decision may
deserve an ADR, and it deserves a doubt pass before it stands. Don't doubt
every keystroke — that is theatre.

The pass:

1. **State the claim** in two or three lines, plus why it matters. If you can't
   write it that compactly, you have a vibe, not a decision.
2. **Extract the smallest reviewable unit** — the diff, or the proposal plus
   the constraints it must satisfy. Strip your reasoning out of it.
3. **Spawn a fresh-context reviewer** — a harness subagent (`code-reviewer`, or
   a plain `Explore`) — and hand it the artifact and the contract *but not your
   conclusion*; passing the conclusion biases it toward agreeing. Frame it
   adversarially: "find what's wrong; assume the author is overconfident." A
   mid-session subagent is fine under constraint 2 — read-don't-run governs
   *bootstrap*, not the tools you reach for while working.
4. **Reconcile.** Each finding is one of: a contract you stated unclearly (fix
   the contract), a real issue (fix the artifact), a real trade-off (accept it,
   visibly), or noise raised for want of context (note it, move on). You are
   still the one deciding — a fresh reviewer can be wrong precisely because it
   is fresh.
5. **Stop** when a pass turns up only trivia, or after three rounds. Three
   unresolved rounds is information about the artifact, not a reason to grind a
   fourth.

This is in-flight doubt, before the commit — not the post-hoc review of a
finished branch, by which point course-correction is expensive.

## Externalizing surfaced work

grove's value is many small, low-context sessions — each leaf picked up fresh,
driven, and retired before the next begins. That value leaks away the moment a
session quietly absorbs work that should have been its own leaf: context fills
with half-related threads, the session swells past one focused unit, and the
clean fresh-relaunch boundary the loop is built on is gone. So when work
surfaces mid-session, the **default is to externalize it as a new leaf**, not to
fold it in. `SKILL.md`'s Decompose step states the rule; this is the habit that
honours it.

Two triggers, two verbs:

- **A new concern surfaces** — the human raises something, or a tangent appears
  that does not serve *this leaf's stated goal*. Append it to the tree with
  `leaf-add` (or `leaf-insert` when it must sequence ahead of live leaves) and
  keep driving the current leaf. The pull to handle it "while you're here" is
  exactly the pull to resist.
- **The current leaf proves bigger** than its brief assumed. Decompose it with
  `leaf-decompose` and do only the first child this session; the remaining
  children are leaves a later session picks up with fresh context.

How to tell inline from externalize: the test is **"does this still fit *this*
session," not "can I finish it."** You almost always *can* finish it — that's
the trap that grows a runaway session. Ask instead whether the work still serves
this leaf's stated goal *and* stays inside one focused, low-context session. If
either answer is no, externalize.

Externalizing is cheap, so spend it freely. A permanent key never moves, a
renumber is a single `git mv` that rewrites zero file contents, and `leaf-insert`
exists precisely so a late-surfacing concern can slot ahead of queued work
without disturbing it. A tree that keeps sprouting small, concrete leaves is the
system working as intended — lazy means *just-in-time, not few* (`SKILL.md`
constraint 4), so don't ration leaves to keep the tree looking tidy.

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/to-tickets/SKILL.md, vertical-slice-rules and the
     wide-refactor expand-contract exception) — MIT licensed; see
     LICENSES/mattpocock-skills.LICENSE. -->

## What a good child leaf looks like

Externalizing tells you *when* to split; this is what the split should
produce. A good child leaf is a **vertical slice**: it cuts a narrow but
complete path through everything its goal touches, not a horizontal layer
that sits dead until its siblings land. The test is independence — can this
leaf's work be demoed or verified on its own, without waiting on a sibling? A
leaf that fails this test needs its lines redrawn, even if it still fits one
session. This is a second axis alongside "fits this session" (`SKILL.md`'s
Decompose step), not a replacement for it.

**Wide refactors are the exception.** A mechanical change whose blast radius
fans across the whole codebase — rename a shared column, retype a symbol
every caller touches — breaks too many call sites at once for any vertical
slice to land green. Sequence it **expand → contract** instead, each stage
its own leaf, added in order with `leaf-add`: first a leaf that adds the new
form beside the old so nothing breaks; then a leaf per migration batch (sized
by blast radius — per package, per directory), each keeping CI green because
the old form still exists alongside the new; finally a leaf that deletes the
old form once no caller remains.

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/wayfinder/SKILL.md, "Fog of war" / "Not yet
     specified") — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

## Recording fog without pre-slicing it

grove's laziness (`SKILL.md` constraint 4) means a leaf either exists or it
doesn't — there's nowhere to keep the dim view of work you can see coming but
can't yet leaf-shape. A brief's **On the horizon** note (`BRIEF-FORMAT.md`)
is that place: a line or two recording foreseen work without pre-slicing it
into a leaf.

The line between a horizon note and a leaf is the **fog-or-ticket test**: can
you state the question precisely right now — not whether you can answer it.
A question you can already phrase precisely earns a leaf immediately with
`leaf-add`, even if it's blocked and unanswerable today. A question still too
dim to phrase that precisely stays a horizon note until a later session
sharpens it enough to graduate.

## Prune, reorder, or file an issue — the triage a status word would hide

A leaf whose place in the tree is in doubt — surfacing new, or already sitting
there under a doubted premise — tempts a status word: `blocked`, `deferred`,
`superseded`. Resist it; that is exactly the taxonomy ADR *pruning* rejects,
and reaching for a fourth state is how a tree starts lying about what is still
live. The doubt always resolves to one of three existing mechanisms:

- **Not now, but still ours** → a **reorder**. The work is good and belongs in
  this tree, just not next: leave the leaf live and `leaf-insert` something
  ahead of it (or reorder by hand). Nothing is decided against — `pick` will
  still return it in its turn.
- **Not ours at all** → a **GitHub issue**, not a leaf. Work that belongs to a
  different repo, owner, or workstream gets filed and dropped rather than grown
  into this tree — this grove's own charter (issue #2) is the worked example.
  If the concern already has a leaf here, prune it once it's filed: the work
  still needs doing, just not in this grove.
- **Decided against** → a **prune** (`grove-llm leaf-prune`, HITL, `SKILL.md`
  "Retire"). The path was considered on its merits and rejected; the leaf
  stays in the tree `ABANDONED`, and the *why* goes to the ADR set, not the
  filename (ADR *pruning*).

Misfiling any of the three corrupts the tree in a different way: reordering a
decided-against leaf keeps a dead end reading as "still coming"; pruning a
not-yet-due leaf erases work nobody rejected; leafing a not-ours concern grows
the tree with work this grove will never do. When a leaf's status is in doubt,
name which of the three sentences above is actually true before reaching for
the CLI — the sentence picks the verb.

## Anti-patterns

- **The wizard.** A capture verb that opens an interactive prompt
  sequence ("title? description? type? component?") breaks
  mid-flow concentration. The prior-art evidence on this is
  emphatic — see ditz's failure mode in the postmortem survey.
  Capture must be one non-interactive gesture (flags or stdin).
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

## The shortest version

If you remember one paragraph of this doc, remember this:

> Commission research with a brief that names the downstream
> questions; demand primary-source citations and per-system
> walk-away checks. When the research is in, grill one question at
> a time with recommended-answer-and-evidence; ask the LLM "WDYT"
> before you commit and ask for pushback when it agrees too
> easily; record each settled decision inline in the task file as
> it lands. At the end, ADRs cite the research by primary source;
> the research doc gets a "Findings adopted" pointer back. That's
> the loop.
